// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Int defaulting of numeric dictionary arguments.

use std::collections::{HashMap, HashSet};

use super::DictPassCtxChirho;
use crate::expr_chirho::{CoreAltChirho, CoreExprChirho, CoreIdChirho};

impl DictPassCtxChirho {
    pub(super) fn print_arg_needs_int_default_chirho(&self, expr_chirho: &CoreExprChirho) -> bool {
        if matches!(expr_chirho, CoreExprChirho::TyAppChirho { .. }) {
            return false;
        }
        if self.is_constructor_headed_app_chirho(expr_chirho) {
            return false;
        }
        self.expr_contains_numeric_default_marker_chirho(expr_chirho)
            || self.is_defaultable_numeric_dict_param_expr_chirho(expr_chirho)
    }

    fn is_numeric_dict_param_expr_chirho(&self, expr_chirho: &CoreExprChirho) -> bool {
        let Some((_fn_id_chirho, classes_chirho, _args_chirho)) =
            self.collect_dict_param_app_chirho(expr_chirho)
        else {
            return false;
        };
        classes_chirho.iter().any(|class_name_chirho| {
            matches!(
                class_name_chirho.as_str(),
                "Num" | "Integral" | "Real" | "Eq" | "Ord" | "Show"
            )
        })
    }

    fn is_defaultable_numeric_dict_param_expr_chirho(&self, expr_chirho: &CoreExprChirho) -> bool {
        let Some((_fn_id_chirho, classes_chirho, _args_chirho)) =
            self.collect_dict_param_app_chirho(expr_chirho)
        else {
            return false;
        };
        classes_chirho.iter().any(|class_name_chirho| {
            matches!(
                class_name_chirho.as_str(),
                "Num" | "Integral" | "Real" | "Fractional" | "Floating" | "RealFrac" | "RealFloat"
            )
        })
    }

    fn expr_needs_int_numeric_default_chirho(&self, expr_chirho: &CoreExprChirho) -> bool {
        self.expr_contains_numeric_default_marker_chirho(expr_chirho)
            || self.is_numeric_dict_param_expr_chirho(expr_chirho)
    }

    fn short_name_for_id_chirho(&self, id_chirho: CoreIdChirho) -> Option<&str> {
        self.names_chirho
            .get(&id_chirho)
            .map(|name_chirho| name_chirho.rsplit('.').next().unwrap_or(name_chirho))
    }

    fn rewrite_expr_with_int_numeric_default_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> CoreExprChirho {
        self.try_rewrite_typed_dict_param_arg_chirho(
            expr_chirho,
            dict_vars_chirho,
            evidence_classes_chirho,
            local_type_keys_chirho,
            local_instance_dicts_chirho,
            "Int",
        )
        .or_else(|| {
            self.try_rewrite_typed_method_arg_chirho(
                expr_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
                "Int",
            )
        })
        .unwrap_or_else(|| {
            let defaulted_chirho = self.rewrite_numeric_methods_with_type_key_chirho(
                expr_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
                "Int",
            );
            self.rewrite_method_refs_with_locals_chirho(
                &defaulted_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_type_keys_chirho,
                local_instance_dicts_chirho,
            )
        })
    }

    pub(super) fn try_rewrite_show_numeric_default_chirho(
        &self,
        head_id_chirho: CoreIdChirho,
        args_chirho: &[&CoreExprChirho],
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        if self.short_name_for_id_chirho(head_id_chirho)? != "show" || args_chirho.len() != 1 {
            return None;
        }
        let arg_chirho = args_chirho[0];
        if !self.print_arg_needs_int_default_chirho(arg_chirho) {
            return None;
        }
        let rewritten_fun_chirho = self.rewrite_method_refs_with_locals_chirho(
            &CoreExprChirho::VarChirho(head_id_chirho),
            dict_vars_chirho,
            evidence_classes_chirho,
            local_type_keys_chirho,
            local_instance_dicts_chirho,
        );
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(rewritten_fun_chirho),
            arg_chirho: Box::new(self.rewrite_expr_with_int_numeric_default_chirho(
                arg_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_type_keys_chirho,
                local_instance_dicts_chirho,
            )),
        })
    }

    pub(super) fn try_rewrite_prim_show_int_arg_chirho(
        &self,
        head_id_chirho: CoreIdChirho,
        args_chirho: &[&CoreExprChirho],
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let head_name_chirho = self.names_chirho.get(&head_id_chirho)?;
        if head_name_chirho != "$prim_Show_show_Int" || args_chirho.len() != 1 {
            return None;
        }
        let rewritten_arg_chirho = self.rewrite_expr_with_type_key_chirho(
            args_chirho[0],
            dict_vars_chirho,
            evidence_classes_chirho,
            local_type_keys_chirho,
            local_instance_dicts_chirho,
            "Int",
        )?;
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(head_id_chirho)),
            arg_chirho: Box::new(rewritten_arg_chirho),
        })
    }

    pub(super) fn try_rewrite_modify_ioref_numeric_default_chirho(
        &self,
        head_id_chirho: CoreIdChirho,
        args_chirho: &[&CoreExprChirho],
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        if self.short_name_for_id_chirho(head_id_chirho)? != "modifyIORef" || args_chirho.len() != 2
        {
            return None;
        }
        let update_fn_chirho = args_chirho[1];
        if !self.expr_needs_int_numeric_default_chirho(update_fn_chirho) {
            return None;
        }

        let mut result_chirho = self.rewrite_method_refs_with_locals_chirho(
            &CoreExprChirho::VarChirho(head_id_chirho),
            dict_vars_chirho,
            evidence_classes_chirho,
            local_type_keys_chirho,
            local_instance_dicts_chirho,
        );
        result_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(result_chirho),
            arg_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                args_chirho[0],
                dict_vars_chirho,
                evidence_classes_chirho,
                local_type_keys_chirho,
                local_instance_dicts_chirho,
            )),
        };
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(result_chirho),
            arg_chirho: Box::new(self.rewrite_expr_with_int_numeric_default_chirho(
                update_fn_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_type_keys_chirho,
                local_instance_dicts_chirho,
            )),
        })
    }

    fn strict_numeric_default_arg_indices_chirho(
        &self,
        head_id_chirho: CoreIdChirho,
    ) -> Option<&'static [usize]> {
        let is_generated_head_chirho = self
            .generated_bindings_chirho
            .iter()
            .any(|binding_chirho| binding_chirho.binder_chirho.id_chirho == head_id_chirho);
        if !is_generated_head_chirho {
            return None;
        }
        let head_name_chirho = self.short_name_for_id_chirho(head_id_chirho)?;
        if self
            .body_backed_names_chirho
            .get(head_name_chirho)
            .is_some_and(|source_id_chirho| *source_id_chirho == head_id_chirho)
        {
            return None;
        }
        match head_name_chirho {
            "newIORef" | "newTVar" | "newTVarIO" => Some(&[0]),
            "writeIORef" | "writeTVar" => Some(&[1]),
            "mapSingleton" => Some(&[0, 1]),
            "mapInsert" => Some(&[0, 1]),
            "mapInsertWith" => Some(&[1, 2]),
            "mapLookup" | "mapDelete" | "mapMember" | "mapNotMember" => Some(&[0]),
            "mapFindWithDefault" => Some(&[0, 1]),
            "mapAdjust" => Some(&[1]),
            _ => None,
        }
    }

    pub(super) fn try_rewrite_strict_numeric_args_chirho(
        &self,
        head_id_chirho: CoreIdChirho,
        args_chirho: &[&CoreExprChirho],
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let strict_arg_indices_chirho =
            self.strict_numeric_default_arg_indices_chirho(head_id_chirho)?;
        let mut defaulted_any_arg_chirho = false;
        let mut rewritten_args_chirho = Vec::with_capacity(args_chirho.len());

        for (idx_chirho, arg_chirho) in args_chirho.iter().enumerate() {
            if strict_arg_indices_chirho.contains(&idx_chirho)
                && self.print_arg_needs_int_default_chirho(arg_chirho)
            {
                defaulted_any_arg_chirho = true;
                rewritten_args_chirho.push(self.rewrite_expr_with_int_numeric_default_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            } else {
                rewritten_args_chirho.push(self.rewrite_method_refs_with_locals_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            }
        }

        if !defaulted_any_arg_chirho {
            return None;
        }

        let rewritten_head_chirho = self.rewrite_method_refs_with_locals_chirho(
            &CoreExprChirho::VarChirho(head_id_chirho),
            dict_vars_chirho,
            evidence_classes_chirho,
            local_type_keys_chirho,
            local_instance_dicts_chirho,
        );
        Some(rewritten_args_chirho.into_iter().fold(
            rewritten_head_chirho,
            |fun_acc_chirho, arg_chirho| CoreExprChirho::AppChirho {
                fun_chirho: Box::new(fun_acc_chirho),
                arg_chirho: Box::new(arg_chirho),
            },
        ))
    }

    fn is_constructor_headed_app_chirho(&self, expr_chirho: &CoreExprChirho) -> bool {
        let mut cur_chirho = expr_chirho;
        while let CoreExprChirho::AppChirho { fun_chirho, .. } = cur_chirho {
            cur_chirho = fun_chirho;
        }
        while let CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ..
        } = cur_chirho
        {
            cur_chirho = inner_chirho;
        }
        let CoreExprChirho::VarChirho(head_id_chirho) = cur_chirho else {
            return matches!(expr_chirho, CoreExprChirho::ConAppChirho { .. });
        };
        let Some(name_chirho) = self.names_chirho.get(head_id_chirho) else {
            return false;
        };
        self.con_types_chirho.contains_key(name_chirho)
            || matches!(
                name_chirho.as_str(),
                "Just"
                    | "Nothing"
                    | "Left"
                    | "Right"
                    | "Identity"
                    | "Proxy"
                    | "Const"
                    | "Sum"
                    | "Product"
                    | "All"
                    | "Any"
                    | "Min"
                    | "Max"
                    | "First"
                    | "Last"
                    | "Down"
                    | "Endo"
                    | ":"
                    | "[]"
                    | "(,)"
                    | "(,,)"
                    | "(,,,)"
            )
            || name_chirho.starts_with("$tuple")
    }

    pub(super) fn rewrite_numeric_methods_with_type_key_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        type_key_chirho: &str,
    ) -> CoreExprChirho {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                if let Some(name_chirho) = self.names_chirho.get(id_chirho) {
                    if let Some((class_name_chirho, _)) =
                        self.class_method_selector_for_name_chirho(name_chirho)
                    {
                        if matches!(class_name_chirho.as_str(), "Num" | "Integral" | "Real") {
                            return self.try_rewrite_method_var_chirho(
                                *id_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_instance_dicts_chirho,
                                Some(type_key_chirho),
                            );
                        }
                    }
                }
                expr_chirho.clone()
            }
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => CoreExprChirho::AppChirho {
                fun_chirho: Box::new(self.rewrite_numeric_methods_with_type_key_chirho(
                    fun_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )),
                arg_chirho: Box::new(self.rewrite_numeric_methods_with_type_key_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )),
            },
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => CoreExprChirho::LamChirho {
                binder_chirho: binder_chirho.clone(),
                body_chirho: Box::new(self.rewrite_numeric_methods_with_type_key_chirho(
                    body_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )),
            },
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => CoreExprChirho::LetChirho {
                rec_chirho: *rec_chirho,
                binds_chirho: binds_chirho
                    .iter()
                    .map(|(binder_chirho, rhs_chirho)| {
                        (
                            binder_chirho.clone(),
                            self.rewrite_numeric_methods_with_type_key_chirho(
                                rhs_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_instance_dicts_chirho,
                                type_key_chirho,
                            ),
                        )
                    })
                    .collect(),
                body_chirho: Box::new(self.rewrite_numeric_methods_with_type_key_chirho(
                    body_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )),
            },
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                result_ty_chirho,
                alts_chirho,
            } => CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(self.rewrite_numeric_methods_with_type_key_chirho(
                    scrutinee_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )),
                bind_chirho: bind_chirho.clone(),
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: alts_chirho
                    .iter()
                    .map(|alt_chirho| CoreAltChirho {
                        con_chirho: alt_chirho.con_chirho.clone(),
                        binders_chirho: alt_chirho.binders_chirho.clone(),
                        rhs_chirho: self.rewrite_numeric_methods_with_type_key_chirho(
                            &alt_chirho.rhs_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_instance_dicts_chirho,
                            type_key_chirho,
                        ),
                    })
                    .collect(),
            },
            CoreExprChirho::TyLamChirho {
                ty_var_chirho,
                body_chirho,
            } => CoreExprChirho::TyLamChirho {
                ty_var_chirho: ty_var_chirho.clone(),
                body_chirho: Box::new(self.rewrite_numeric_methods_with_type_key_chirho(
                    body_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )),
            },
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
            } => CoreExprChirho::TyAppChirho {
                expr_chirho: Box::new(self.rewrite_numeric_methods_with_type_key_chirho(
                    inner_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )),
                ty_chirho: ty_chirho.clone(),
            },
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => CoreExprChirho::PrimOpChirho {
                name_chirho: name_chirho.clone(),
                args_chirho: args_chirho
                    .iter()
                    .map(|arg_chirho| {
                        self.rewrite_numeric_methods_with_type_key_chirho(
                            arg_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_instance_dicts_chirho,
                            type_key_chirho,
                        )
                    })
                    .collect(),
            },
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } => CoreExprChirho::ConAppChirho {
                con_name_chirho: con_name_chirho.clone(),
                args_chirho: args_chirho
                    .iter()
                    .map(|arg_chirho| {
                        self.rewrite_numeric_methods_with_type_key_chirho(
                            arg_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_instance_dicts_chirho,
                            type_key_chirho,
                        )
                    })
                    .collect(),
            },
            CoreExprChirho::LitChirho(_) => expr_chirho.clone(),
        }
    }
}
