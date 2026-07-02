// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Method rewriting: dict param insertion, method reference rewriting,
//! module transformation, and the final pass finish.

#![allow(unused_imports)]
use std::collections::HashMap;

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::{DictLayoutChirho, DictPassCtxChirho};
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
};

use super::DictPassResultChirho;
use haskelujah_typing_chirho::env_chirho::TyEnvChirho;
use haskelujah_typing_chirho::ty_chirho::{SchemeChirho, SchemePredChirho};
use std::cell::RefCell;
use std::collections::HashSet;

impl DictPassCtxChirho {
    fn parse_prim_binding_info_chirho(
        &self,
        binding_name_chirho: &str,
    ) -> Option<(String, String, String)> {
        for (class_name_chirho, layout_chirho) in &self.layouts_chirho {
            for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                let prefix_chirho = format!("$prim_{}_{}_", class_name_chirho, method_name_chirho);
                if let Some(type_key_chirho) = binding_name_chirho.strip_prefix(&prefix_chirho) {
                    return Some((
                        class_name_chirho.clone(),
                        method_name_chirho.clone(),
                        type_key_chirho.to_string(),
                    ));
                }
            }
        }
        None
    }

    fn binder_type_key_chirho(&self, binder_chirho: &BinderChirho) -> Option<String> {
        match &binder_chirho.ty_chirho {
            TyChirho::FunChirho(_, _, _) => None,
            TyChirho::ForallChirho { body_chirho, .. } => Some(format!("{}", body_chirho)),
            _ => Some(format!("{}", binder_chirho.ty_chirho)),
        }
    }

    fn seed_value_binder_type_keys_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        ty_chirho: &TyChirho,
        local_type_keys_chirho: &mut HashMap<CoreIdChirho, String>,
    ) {
        match (expr_chirho, ty_chirho) {
            (
                CoreExprChirho::LamChirho {
                    binder_chirho,
                    body_chirho,
                },
                TyChirho::FunChirho(arg_ty_chirho, result_ty_chirho, _),
            ) => {
                local_type_keys_chirho
                    .insert(binder_chirho.id_chirho, format!("{}", arg_ty_chirho));
                self.seed_value_binder_type_keys_chirho(
                    body_chirho,
                    result_ty_chirho,
                    local_type_keys_chirho,
                );
            }
            (
                CoreExprChirho::TyLamChirho { body_chirho, .. },
                TyChirho::ForallChirho {
                    body_chirho: inner_ty_chirho,
                    ..
                },
            ) => {
                self.seed_value_binder_type_keys_chirho(
                    body_chirho,
                    inner_ty_chirho,
                    local_type_keys_chirho,
                );
            }
            (CoreExprChirho::TyLamChirho { body_chirho, .. }, _) => {
                self.seed_value_binder_type_keys_chirho(
                    body_chirho,
                    ty_chirho,
                    local_type_keys_chirho,
                );
            }
            _ => {}
        }
    }

    fn strip_foralls_chirho<'a>(&self, mut ty_chirho: &'a TyChirho) -> &'a TyChirho {
        while let TyChirho::ForallChirho { body_chirho, .. } = ty_chirho {
            ty_chirho = body_chirho;
        }
        ty_chirho
    }

    fn binding_instance_type_key_chirho(
        &self,
        class_name_chirho: &str,
        scheme_chirho: &SchemeChirho,
    ) -> Option<String> {
        let param_count_chirho = self
            .class_param_count_chirho
            .get(class_name_chirho)
            .copied()
            .unwrap_or(1);
        let mut arg_keys_chirho = Vec::new();
        let mut current_ty_chirho = self.strip_foralls_chirho(&scheme_chirho.ty_chirho);
        while let TyChirho::FunChirho(arg_ty_chirho, result_ty_chirho, _) = current_ty_chirho {
            arg_keys_chirho.push(format!("{}", arg_ty_chirho));
            if arg_keys_chirho.len() == param_count_chirho {
                return Some(arg_keys_chirho.join("_"));
            }
            current_ty_chirho = result_ty_chirho;
        }
        None
    }

    fn conditional_context_classes_for_prim_binding_chirho(
        &self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        parsed_type_key_chirho: &str,
    ) -> Option<Vec<String>> {
        let instances_chirho = class_env_chirho.instances_chirho.get(class_name_chirho)?;
        for inst_chirho in instances_chirho {
            if inst_chirho.context_chirho.is_empty() {
                continue;
            }
            let head_type_key_chirho = format!("{}", inst_chirho.head_ty_chirho);
            let matches_chirho = if head_type_key_chirho == parsed_type_key_chirho {
                true
            } else {
                matches!(
                    &inst_chirho.head_ty_chirho,
                    TyChirho::ListChirho(elem_ty_chirho)
                        if matches!(elem_ty_chirho.as_ref(), TyChirho::VarChirho(_))
                            && parsed_type_key_chirho.starts_with('[')
                            && parsed_type_key_chirho.ends_with(']')
                )
            };
            if matches_chirho {
                return Some(
                    inst_chirho
                        .context_chirho
                        .iter()
                        .map(|pred_chirho| pred_chirho.class_name_chirho.clone())
                        .collect(),
                );
            }
        }
        None
    }

    fn dict_param_classes_for_scheme_chirho(scheme_chirho: &SchemeChirho) -> Vec<String> {
        scheme_chirho
            .preds_chirho
            .iter()
            .filter(|p_chirho| matches!(p_chirho.ty_chirho, TyChirho::VarChirho(_)))
            .filter(|p_chirho| !Self::is_defaultable_pred_chirho(p_chirho, scheme_chirho))
            .map(|p_chirho| p_chirho.class_name_chirho.clone())
            .collect()
    }

    fn extend_alt_type_keys_chirho(
        &self,
        scrutinee_type_key_chirho: Option<String>,
        bind_chirho: &BinderChirho,
        alt_chirho: &CoreAltChirho,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> HashMap<CoreIdChirho, String> {
        let mut alt_type_keys_chirho = local_type_keys_chirho.clone();
        if let Some(type_key_chirho) = scrutinee_type_key_chirho.clone() {
            alt_type_keys_chirho.insert(bind_chirho.id_chirho, type_key_chirho);
        }
        match (&alt_chirho.con_chirho, alt_chirho.binders_chirho.as_slice()) {
            (
                AltConChirho::DataConChirho(con_name_chirho),
                [head_binder_chirho, tail_binder_chirho],
            ) if con_name_chirho == ":" => {
                if let Some(scrutinee_list_key_chirho) = scrutinee_type_key_chirho {
                    if scrutinee_list_key_chirho.starts_with('[')
                        && scrutinee_list_key_chirho.ends_with(']')
                    {
                        let elem_type_key_chirho = scrutinee_list_key_chirho
                            [1..scrutinee_list_key_chirho.len() - 1]
                            .to_string();
                        alt_type_keys_chirho
                            .insert(head_binder_chirho.id_chirho, elem_type_key_chirho);
                        alt_type_keys_chirho
                            .insert(tail_binder_chirho.id_chirho, scrutinee_list_key_chirho);
                    }
                }
            }
            (AltConChirho::DataConChirho(con_name_chirho), binders_chirho)
                if Self::is_tuple_constructor_name_chirho(con_name_chirho) =>
            {
                if let Some(scrutinee_tuple_key_chirho) = scrutinee_type_key_chirho {
                    if let Some(elem_type_keys_chirho) =
                        Self::tuple_payload_type_keys_chirho(&scrutinee_tuple_key_chirho)
                    {
                        if elem_type_keys_chirho.len() == binders_chirho.len() {
                            for (binder_chirho, elem_type_key_chirho) in
                                binders_chirho.iter().zip(elem_type_keys_chirho.into_iter())
                            {
                                alt_type_keys_chirho
                                    .insert(binder_chirho.id_chirho, elem_type_key_chirho);
                            }
                        }
                    }
                }
            }
            (AltConChirho::DataConChirho(con_name_chirho), [inner_binder_chirho])
                if con_name_chirho == "Just" =>
            {
                if let Some(scrutinee_maybe_key_chirho) = scrutinee_type_key_chirho {
                    if let Some(inner_type_key_chirho) =
                        scrutinee_maybe_key_chirho.strip_prefix("Maybe ")
                    {
                        alt_type_keys_chirho.insert(
                            inner_binder_chirho.id_chirho,
                            inner_type_key_chirho.to_string(),
                        );
                    }
                }
            }
            _ => {
                for binder_chirho in &alt_chirho.binders_chirho {
                    if let Some(type_key_chirho) = self.binder_type_key_chirho(binder_chirho) {
                        alt_type_keys_chirho.insert(binder_chirho.id_chirho, type_key_chirho);
                    }
                }
            }
        }
        alt_type_keys_chirho
    }

    fn infer_type_key_for_rewrite_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<String> {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => local_type_keys_chirho
                .get(id_chirho)
                .cloned()
                .or_else(|| self.infer_type_key_chirho(expr_chirho)),
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.infer_type_key_for_rewrite_chirho(inner_chirho, local_type_keys_chirho),
            _ => self.infer_type_key_chirho(expr_chirho),
        }
    }

    fn infer_strict_dispatch_key_for_rewrite_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<String> {
        match expr_chirho {
            CoreExprChirho::TyAppChirho { ty_chirho, .. } => Some(format!("{}", ty_chirho)),
            CoreExprChirho::VarChirho(id_chirho) => local_type_keys_chirho
                .get(id_chirho)
                .cloned()
                .or_else(|| self.infer_type_key_chirho(expr_chirho)),
            CoreExprChirho::ConAppChirho { .. }
            | CoreExprChirho::LitChirho(_)
            | CoreExprChirho::PrimOpChirho { .. } => self.infer_type_key_chirho(expr_chirho),
            CoreExprChirho::AppChirho { .. } => {
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
                    return None;
                };
                let name_chirho = self.names_chirho.get(head_id_chirho)?;
                let is_con_head_chirho = self.con_types_chirho.contains_key(name_chirho)
                    || matches!(
                        name_chirho.as_str(),
                        "Just"
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
                            | "(,)"
                            | "(,,)"
                            | "(,,,)"
                    )
                    || name_chirho.starts_with("$tuple");
                if is_con_head_chirho {
                    return self.infer_type_key_chirho(expr_chirho);
                }
                if self.dict_param_bindings_chirho.contains_key(head_id_chirho) {
                    let mut args_chirho = Vec::new();
                    let mut app_chirho = expr_chirho;
                    while let CoreExprChirho::AppChirho {
                        fun_chirho,
                        arg_chirho,
                    } = app_chirho
                    {
                        args_chirho.push(arg_chirho.as_ref());
                        app_chirho = fun_chirho.as_ref();
                    }
                    args_chirho.reverse();
                    return args_chirho.iter().find_map(|arg_chirho| {
                        self.infer_strict_dispatch_key_for_rewrite_chirho(
                            arg_chirho,
                            local_type_keys_chirho,
                        )
                    });
                }
                None
            }
            _ => None,
        }
    }

    fn print_arg_needs_int_default_chirho(&self, expr_chirho: &CoreExprChirho) -> bool {
        if matches!(expr_chirho, CoreExprChirho::TyAppChirho { .. }) {
            return false;
        }
        if self.is_constructor_headed_app_chirho(expr_chirho) {
            return false;
        }
        self.expr_contains_numeric_default_marker_chirho(expr_chirho)
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

    fn expr_contains_numeric_default_marker_chirho(&self, expr_chirho: &CoreExprChirho) -> bool {
        match expr_chirho {
            CoreExprChirho::LitChirho(crate::expr_chirho::CoreLitChirho::IntChirho(_)) => true,
            CoreExprChirho::VarChirho(id_chirho) => {
                self.names_chirho.get(id_chirho).is_some_and(|name_chirho| {
                    let short_name_chirho = name_chirho.rsplit('.').next().unwrap_or(name_chirho);
                    let marker_chirho = short_name_chirho
                        .strip_prefix('(')
                        .and_then(|name_chirho| name_chirho.strip_suffix(')'))
                        .unwrap_or(short_name_chirho);
                    matches!(
                        marker_chirho,
                        "+" | "-" | "*" | "negate" | "abs" | "signum" | "fromInteger" | "toInteger"
                    )
                })
            }
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => {
                matches!(
                    name_chirho.as_str(),
                    "+#" | "-#" | "*#" | "negate#" | "fromInteger#" | "toInteger#"
                ) || args_chirho
                    .iter()
                    .any(|arg_chirho| self.expr_contains_numeric_default_marker_chirho(arg_chirho))
            }
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                self.expr_contains_numeric_default_marker_chirho(fun_chirho)
                    || self.expr_contains_numeric_default_marker_chirho(arg_chirho)
            }
            CoreExprChirho::LamChirho { body_chirho, .. }
            | CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                self.expr_contains_numeric_default_marker_chirho(body_chirho)
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.expr_contains_numeric_default_marker_chirho(inner_chirho),
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                binds_chirho.iter().any(|(_, rhs_chirho)| {
                    self.expr_contains_numeric_default_marker_chirho(rhs_chirho)
                }) || self.expr_contains_numeric_default_marker_chirho(body_chirho)
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                self.expr_contains_numeric_default_marker_chirho(scrutinee_chirho)
                    || alts_chirho.iter().any(|alt_chirho| {
                        self.expr_contains_numeric_default_marker_chirho(&alt_chirho.rhs_chirho)
                    })
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => args_chirho
                .iter()
                .any(|arg_chirho| self.expr_contains_numeric_default_marker_chirho(arg_chirho)),
            _ => false,
        }
    }

    fn rewrite_numeric_methods_with_type_key_chirho(
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

    fn collect_method_app_chirho<'a>(
        &self,
        expr_chirho: &'a CoreExprChirho,
    ) -> Option<(CoreIdChirho, Vec<&'a CoreExprChirho>, Option<String>)> {
        let mut args_chirho = Vec::new();
        let mut current_chirho = expr_chirho;
        let mut type_key_hint_chirho = None;

        // Peel off value applications and interleaved type applications.
        // Interface-imported overloaded methods can arrive as
        // `v @Sum arg1 @Sum arg2`; the type app is the reliable instance key
        // when interface newtype erasure makes the value args look like Ints.
        loop {
            match current_chirho {
                CoreExprChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                } => {
                    args_chirho.push(arg_chirho.as_ref());
                    current_chirho = fun_chirho.as_ref();
                }
                CoreExprChirho::TyAppChirho {
                    expr_chirho: inner_chirho,
                    ty_chirho,
                } => {
                    type_key_hint_chirho = Some(format!("{}", ty_chirho));
                    current_chirho = inner_chirho.as_ref();
                }
                _ => break,
            }
        }

        // Check if the innermost function is a class method Var
        if let CoreExprChirho::VarChirho(id_chirho) = current_chirho {
            if let Some(name_chirho) = self.names_chirho.get(id_chirho) {
                if self
                    .class_method_selector_for_name_chirho(name_chirho)
                    .is_some()
                {
                    args_chirho.reverse(); // Now [arg1, arg2, ...]
                    return Some((*id_chirho, args_chirho, type_key_hint_chirho));
                }
            }
        }

        None
    }

    fn class_method_selector_for_name_chirho(
        &self,
        name_chirho: &str,
    ) -> Option<(String, CoreIdChirho)> {
        if let Some((class_name_chirho, sel_id_chirho)) =
            self.method_selectors_chirho.get(name_chirho)
        {
            return Some((class_name_chirho.clone(), *sel_id_chirho));
        }

        let short_name_chirho = name_chirho.rsplit('.').next()?;
        if let Some(stripped_chirho) = short_name_chirho
            .strip_prefix('(')
            .and_then(|n_chirho| n_chirho.strip_suffix(')'))
        {
            if let Some((class_name_chirho, sel_id_chirho)) =
                self.method_selectors_chirho.get(stripped_chirho)
            {
                return Some((class_name_chirho.clone(), *sel_id_chirho));
            }
        }
        if short_name_chirho == name_chirho {
            return None;
        }
        self.method_selectors_chirho
            .get(short_name_chirho)
            .map(|(class_name_chirho, sel_id_chirho)| (class_name_chirho.clone(), *sel_id_chirho))
    }

    /// Collect a dict-parameterized function application chain: if `expr_chirho`
    /// is `App^n(Var(f), arg1, ..., argN)` where `f` has dict parameters,
    /// return `(fn_id, classes, [arg1, ..., argN])`.
    fn collect_dict_param_app_chirho<'a>(
        &self,
        expr_chirho: &'a CoreExprChirho,
    ) -> Option<(CoreIdChirho, &[String], Vec<&'a CoreExprChirho>)> {
        let mut args_chirho = Vec::new();
        let mut current_chirho = expr_chirho;

        while let CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } = current_chirho
        {
            args_chirho.push(arg_chirho.as_ref());
            current_chirho = fun_chirho.as_ref();
        }

        while let CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ..
        } = current_chirho
        {
            current_chirho = inner_chirho.as_ref();
        }

        if let CoreExprChirho::VarChirho(id_chirho) = current_chirho {
            if let Some(classes_chirho) = self.dict_param_bindings_chirho.get(id_chirho) {
                args_chirho.reverse();
                return Some((*id_chirho, classes_chirho, args_chirho));
            }
        }

        None
    }

    /// Try to rewrite a method Var reference into a dictionary projection.
    /// If `type_key_override_chirho` is provided, use it to select a
    /// type-specific instance dictionary instead of the default.
    fn try_rewrite_method_var_chirho(
        &self,
        id_chirho: CoreIdChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        type_key_override_chirho: Option<&str>,
    ) -> CoreExprChirho {
        // Skip rewriting for locally-bound variables that shadow class methods
        if self.local_shadow_ids_chirho.borrow().contains(&id_chirho) {
            return CoreExprChirho::VarChirho(id_chirho);
        }
        if let Some(name_chirho) = self.names_chirho.get(&id_chirho) {
            if let Some((class_name_chirho, sel_id_chirho)) =
                self.class_method_selector_for_name_chirho(name_chirho)
            {
                // Try type-specific instance dict first. For higher-kinded
                // classes, value inference sees keys like `Either Int Int`
                // while generated dictionaries are keyed by the type head
                // (`Either`, `Maybe`, `[]`).
                if let Some(type_key_chirho) = type_key_override_chirho {
                    let mut candidate_keys_chirho = vec![type_key_chirho.to_string()];
                    if Self::should_normalize_instance_head_for_class_chirho(&class_name_chirho) {
                        let normalized_chirho =
                            Self::normalize_instance_head_key_chirho(type_key_chirho);
                        if normalized_chirho != type_key_chirho {
                            candidate_keys_chirho.push(normalized_chirho);
                        }
                    }
                    for candidate_key_chirho in candidate_keys_chirho {
                        if let Some(dict_id_chirho) = local_instance_dicts_chirho
                            .get(&(class_name_chirho.clone(), candidate_key_chirho.clone()))
                        {
                            return CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(*dict_id_chirho)),
                            };
                        }
                        if let Some(dict_id_chirho) = self
                            .instance_dicts_chirho
                            .get(&(class_name_chirho.clone(), candidate_key_chirho))
                        {
                            return CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(*dict_id_chirho)),
                            };
                        }
                    }
                }

                // Fall back only to dictionaries backed by real local evidence.
                // Ambient seeded defaults are not proof for an unresolved method
                // use and used to pick arbitrary semantics.
                if evidence_classes_chirho.contains(&class_name_chirho) {
                    if let Some(dict_id_chirho) = dict_vars_chirho.get(&class_name_chirho) {
                        return CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(*dict_id_chirho)),
                        };
                    }
                }
            }
        }
        CoreExprChirho::VarChirho(id_chirho)
    }

    // Rewrites typed method arguments such as `(empty :: [Int])` after an
    // outer method application has selected a concrete instance key.
    // workflow: monadic-dispatch-chirho
    fn try_rewrite_typed_method_arg_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        type_key_chirho: &str,
    ) -> Option<CoreExprChirho> {
        match expr_chirho {
            CoreExprChirho::VarChirho(arg_id_chirho) => Some(self.try_rewrite_method_var_chirho(
                *arg_id_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
                Some(type_key_chirho),
            )),
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.try_rewrite_typed_method_arg_chirho(
                inner_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
                type_key_chirho,
            ),
            _ => None,
        }
    }

    fn try_rewrite_typed_dict_param_arg_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        type_key_chirho: &str,
    ) -> Option<CoreExprChirho> {
        let (fn_id_chirho, classes_chirho, args_chirho) =
            self.collect_dict_param_app_chirho(expr_chirho)?;

        let mut result_chirho = CoreExprChirho::VarChirho(fn_id_chirho);
        let mut inserted_dict_chirho = false;
        let classes_chirho = classes_chirho.to_vec();
        for class_name_chirho in &classes_chirho {
            let dict_id_chirho = self
                .instance_dicts_chirho
                .get(&(class_name_chirho.clone(), type_key_chirho.to_string()))
                .copied()
                .or_else(|| {
                    Self::fallback_dict_for_class_chirho(
                        class_name_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                    )
                });
            if let Some(dict_id_chirho) = dict_id_chirho {
                inserted_dict_chirho = true;
                result_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(result_chirho),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
                };
            }
        }
        if !inserted_dict_chirho {
            return None;
        }

        for a_chirho in &args_chirho {
            let rewritten_arg_chirho = self
                .try_rewrite_typed_dict_param_arg_chirho(
                    a_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )
                .or_else(|| {
                    self.try_rewrite_typed_method_arg_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_instance_dicts_chirho,
                        type_key_chirho,
                    )
                })
                .unwrap_or_else(|| {
                    self.rewrite_method_refs_with_locals_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )
                });
            result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(rewritten_arg_chirho),
            };
        }
        Some(result_chirho)
    }

    fn try_rewrite_typed_higher_order_args_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let mut args_chirho = Vec::new();
        let mut head_chirho = expr_chirho;
        while let CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } = head_chirho
        {
            args_chirho.push(arg_chirho.as_ref());
            head_chirho = fun_chirho.as_ref();
        }
        if args_chirho.len() < 2 {
            return None;
        }
        args_chirho.reverse();

        let type_key_chirho = args_chirho.iter().rev().find_map(|arg_chirho| {
            self.infer_strict_dispatch_key_for_rewrite_chirho(arg_chirho, local_type_keys_chirho)
        })?;

        let mut changed_chirho = false;
        let mut rewritten_args_chirho = Vec::with_capacity(args_chirho.len());
        for arg_chirho in &args_chirho {
            if let Some(rewritten_chirho) = self
                .try_rewrite_typed_dict_param_arg_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    &type_key_chirho,
                )
                .or_else(|| {
                    self.try_rewrite_typed_method_arg_chirho(
                        arg_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_instance_dicts_chirho,
                        &type_key_chirho,
                    )
                })
            {
                changed_chirho = true;
                rewritten_args_chirho.push(rewritten_chirho);
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
        if !changed_chirho {
            return None;
        }

        let mut result_chirho = self.rewrite_method_refs_with_locals_chirho(
            head_chirho,
            dict_vars_chirho,
            evidence_classes_chirho,
            local_type_keys_chirho,
            local_instance_dicts_chirho,
        );
        for rewritten_arg_chirho in rewritten_args_chirho {
            result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(rewritten_arg_chirho),
            };
        }
        Some(result_chirho)
    }

    fn fallback_dict_for_class_chirho(
        class_name_chirho: &str,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
    ) -> Option<CoreIdChirho> {
        if evidence_classes_chirho.contains(class_name_chirho) {
            dict_vars_chirho.get(class_name_chirho).copied()
        } else {
            None
        }
    }

    /// Normalize a value-shaped type key (`Maybe Int`, `Either e a`, `[Char]`)
    /// to the instance head key (`Maybe`, `Either`, `[]`) used to name the
    /// generated per-type instance bodies. Non-higher-kinded keys pass through.
    fn normalize_instance_head_key_chirho(value_key_chirho: &str) -> String {
        let trimmed_chirho = value_key_chirho.trim();
        if trimmed_chirho.starts_with('[') {
            return "[]".to_string();
        }
        match trimmed_chirho.split_whitespace().next() {
            Some(head_chirho) => head_chirho.to_string(),
            None => trimmed_chirho.to_string(),
        }
    }

    fn annotated_monad_head_key_chirho(&self, ty_chirho: &TyChirho) -> Option<String> {
        self.body_backed_monad_head_key_for_ty_chirho(ty_chirho, false)
    }

    fn body_backed_monad_head_key_for_ty_chirho(
        &self,
        ty_chirho: &TyChirho,
        follow_fun_result_chirho: bool,
    ) -> Option<String> {
        let head_key_chirho = Self::raw_type_head_key_chirho(ty_chirho, follow_fun_result_chirho)?;
        let prim_name_chirho = format!("$prim_Applicative_pure_{head_key_chirho}");
        self.lookup_dispatch_body_name_id_chirho(&prim_name_chirho)
            .map(|_| head_key_chirho)
    }

    fn raw_type_head_key_chirho(
        ty_chirho: &TyChirho,
        follow_fun_result_chirho: bool,
    ) -> Option<String> {
        match ty_chirho {
            TyChirho::ForallChirho { body_chirho, .. } => {
                Self::raw_type_head_key_chirho(body_chirho, follow_fun_result_chirho)
            }
            TyChirho::FunChirho(_, result_ty_chirho, _) if follow_fun_result_chirho => {
                Self::raw_type_head_key_chirho(result_ty_chirho, true)
            }
            TyChirho::FunChirho(_, _, _) => None,
            TyChirho::ListChirho(_) => Some("[]".to_string()),
            TyChirho::AppChirho(fun_chirho, _) => Self::raw_type_head_key_chirho(fun_chirho, false),
            TyChirho::ConChirho(name_chirho) => {
                Some(Self::normalize_instance_head_key_chirho(name_chirho))
            }
            _ => None,
        }
    }

    fn strip_value_lams_chirho<'a>(mut expr_chirho: &'a CoreExprChirho) -> &'a CoreExprChirho {
        loop {
            match expr_chirho {
                CoreExprChirho::LamChirho { body_chirho, .. }
                | CoreExprChirho::TyLamChirho { body_chirho, .. }
                | CoreExprChirho::TyAppChirho {
                    expr_chirho: body_chirho,
                    ..
                } => expr_chirho = body_chirho,
                _ => return expr_chirho,
            }
        }
    }

    fn has_direct_return_or_pure_head_chirho(&self, expr_chirho: &CoreExprChirho) -> bool {
        let mut cur_chirho = expr_chirho;
        loop {
            match cur_chirho {
                CoreExprChirho::AppChirho { fun_chirho, .. } => {
                    cur_chirho = fun_chirho;
                }
                CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
                    cur_chirho = expr_chirho;
                }
                CoreExprChirho::VarChirho(id_chirho) => {
                    return self
                        .names_chirho
                        .get(id_chirho)
                        .map(|name_chirho| name_chirho == "return" || name_chirho == "pure")
                        .unwrap_or(false);
                }
                _ => return false,
            }
        }
    }

    fn try_dispatch_contextual_return_pure_var_chirho(
        &self,
        id_chirho: CoreIdChirho,
    ) -> Option<CoreExprChirho> {
        if self.local_shadow_ids_chirho.borrow().contains(&id_chirho) {
            return None;
        }
        let name_chirho = self.names_chirho.get(&id_chirho)?;
        if name_chirho != "return" && name_chirho != "pure" {
            return None;
        }
        let context_key_chirho = self.monad_context_stack_chirho.borrow().last().cloned()?;
        let inst_id_chirho = self.lookup_dispatch_body_name_id_chirho(&format!(
            "$prim_Applicative_pure_{context_key_chirho}"
        ))?;
        Some(CoreExprChirho::VarChirho(inst_id_chirho))
    }

    fn should_normalize_instance_head_for_class_chirho(class_name_chirho: &str) -> bool {
        matches!(
            class_name_chirho,
            "Functor"
                | "Applicative"
                | "Monad"
                | "Foldable"
                | "Traversable"
                | "Alternative"
                | "MonadPlus"
        )
    }

    fn rebuild_preserved_method_app_chirho(
        &self,
        head_id_chirho: CoreIdChirho,
        args_chirho: &[&CoreExprChirho],
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> CoreExprChirho {
        let mut result_chirho = CoreExprChirho::VarChirho(head_id_chirho);
        for a_chirho in args_chirho {
            result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                    a_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                )),
            };
        }
        result_chirho
    }

    /// Conservative type key for monad-chain dispatch (`>>=` / `>>`): only
    /// shapes that PROVE the monad head are trusted — constructor
    /// applications (Just x, Right y, x : xs), constructor vars, locally
    /// keyed vars, literals, primops with exact result types, and type
    /// annotations. Non-constructor function applications are NOT trusted:
    /// the general inference blindly propagates argument types through Apps,
    /// which routed IO chains into the list monad (`putStrLn "a" >> ...`
    /// inferred as `[Char]` -> `[]`). workflow: monadic-dispatch-chirho
    fn infer_monad_dispatch_key_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<String> {
        match expr_chirho {
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
            } => self
                .annotated_monad_head_key_chirho(ty_chirho)
                .filter(|_| self.has_direct_return_or_pure_head_chirho(inner_chirho))
                .or_else(|| {
                    self.infer_monad_dispatch_key_chirho(inner_chirho, local_type_keys_chirho)
                }),
            CoreExprChirho::VarChirho(id_chirho) => local_type_keys_chirho
                .get(id_chirho)
                .cloned()
                .or_else(|| self.infer_type_key_chirho(expr_chirho)),
            CoreExprChirho::ConAppChirho { .. }
            | CoreExprChirho::LitChirho(_)
            | CoreExprChirho::PrimOpChirho { .. } => self.infer_type_key_chirho(expr_chirho),
            CoreExprChirho::AppChirho { .. } => {
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
                    return None;
                };
                let name_chirho = self.names_chirho.get(head_id_chirho)?;
                let is_con_head_chirho = self.con_types_chirho.contains_key(name_chirho)
                    || matches!(
                        name_chirho.as_str(),
                        "Just"
                            | "Left"
                            | "Right"
                            | "Down"
                            | "Endo"
                            | ":"
                            | "(,)"
                            | "(,,)"
                            | "(,,,)"
                    )
                    || name_chirho.starts_with("$tuple");
                if is_con_head_chirho {
                    return self.infer_type_key_chirho(expr_chirho);
                }
                None
            }
            _ => None,
        }
    }

    fn is_tuple_constructor_name_chirho(name_chirho: &str) -> bool {
        name_chirho == "(,)"
            || name_chirho == "(,,)"
            || name_chirho == "(,,,)"
            || name_chirho.starts_with("$tuple")
    }

    fn tuple_payload_type_keys_chirho(value_key_chirho: &str) -> Option<Vec<String>> {
        let trimmed_chirho = value_key_chirho.trim();
        if !trimmed_chirho.starts_with('(')
            || !trimmed_chirho.ends_with(')')
            || trimmed_chirho.len() <= 2
        {
            return None;
        }

        let inner_chirho = &trimmed_chirho[1..trimmed_chirho.len() - 1];
        let mut parts_chirho = Vec::new();
        let mut depth_chirho = 0i32;
        let mut start_chirho = 0usize;
        let mut saw_comma_chirho = false;
        for (idx_chirho, ch_chirho) in inner_chirho.char_indices() {
            match ch_chirho {
                '(' | '[' => depth_chirho += 1,
                ')' | ']' => depth_chirho -= 1,
                ',' if depth_chirho == 0 => {
                    saw_comma_chirho = true;
                    parts_chirho.push(inner_chirho[start_chirho..idx_chirho].trim().to_string());
                    start_chirho = idx_chirho + ch_chirho.len_utf8();
                }
                _ => {}
            }
        }
        if !saw_comma_chirho {
            return None;
        }

        parts_chirho.push(inner_chirho[start_chirho..].trim().to_string());
        if parts_chirho
            .iter()
            .any(|part_chirho| part_chirho.is_empty())
        {
            return None;
        }
        Some(parts_chirho)
    }

    fn monad_payload_type_key_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        value_key_chirho: &str,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<String> {
        match expr_chirho {
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.monad_payload_type_key_chirho(
                inner_chirho,
                value_key_chirho,
                local_type_keys_chirho,
            ),
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } => match con_name_chirho.as_str() {
                "Just" | "Right" | "Identity" | "Down" => {
                    args_chirho.first().and_then(|arg_chirho| {
                        self.infer_type_key_for_rewrite_chirho(arg_chirho, local_type_keys_chirho)
                    })
                }
                ":" => Self::list_payload_type_key_chirho(value_key_chirho),
                _ => Self::payload_type_key_from_value_key_chirho(value_key_chirho),
            },
            CoreExprChirho::AppChirho { .. } => {
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
                    return Self::payload_type_key_from_value_key_chirho(value_key_chirho);
                };
                match self.names_chirho.get(head_id_chirho).map(String::as_str) {
                    Some("Just" | "Right" | "Identity" | "Down") => {
                        Self::payload_type_key_from_value_key_chirho(value_key_chirho)
                    }
                    Some(":") => Self::list_payload_type_key_chirho(value_key_chirho),
                    _ => Self::payload_type_key_from_value_key_chirho(value_key_chirho),
                }
            }
            _ => Self::payload_type_key_from_value_key_chirho(value_key_chirho),
        }
    }

    fn payload_type_key_from_value_key_chirho(value_key_chirho: &str) -> Option<String> {
        let trimmed_chirho = value_key_chirho.trim();
        if let Some(inner_chirho) = Self::list_payload_type_key_chirho(trimmed_chirho) {
            return Some(inner_chirho);
        }
        if let Some(inner_chirho) = trimmed_chirho.strip_prefix("Maybe ") {
            return Some(inner_chirho.to_string());
        }
        if let Some(rest_chirho) = trimmed_chirho.strip_prefix("Either ") {
            return rest_chirho
                .split_whitespace()
                .last()
                .map(|payload_chirho| payload_chirho.to_string());
        }
        None
    }

    fn list_payload_type_key_chirho(value_key_chirho: &str) -> Option<String> {
        let trimmed_chirho = value_key_chirho.trim();
        if trimmed_chirho.starts_with('[')
            && trimmed_chirho.ends_with(']')
            && trimmed_chirho.len() > 2
        {
            return Some(trimmed_chirho[1..trimmed_chirho.len() - 1].to_string());
        }
        None
    }

    fn extend_first_lambda_type_key_chirho(
        expr_chirho: &CoreExprChirho,
        payload_key_chirho: &str,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> HashMap<CoreIdChirho, String> {
        let mut extended_chirho = local_type_keys_chirho.clone();
        let mut cur_chirho = expr_chirho;
        loop {
            match cur_chirho {
                CoreExprChirho::LamChirho { binder_chirho, .. } => {
                    extended_chirho.insert(binder_chirho.id_chirho, payload_key_chirho.to_string());
                    break;
                }
                CoreExprChirho::TyLamChirho { body_chirho, .. }
                | CoreExprChirho::TyAppChirho {
                    expr_chirho: body_chirho,
                    ..
                } => {
                    cur_chirho = body_chirho;
                }
                _ => break,
            }
        }
        extended_chirho
    }

    /// Dispatch a higher-kinded class method to the correct per-type instance
    /// body based on the type key of its dispatch argument.
    /// The method otherwise resolves to a monomorphic default binding, which is
    /// left in place (returns `None`) when the key cannot be determined — so IO
    /// / unknown cases keep their existing behaviour.
    fn try_dispatch_hk_method_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        // Collect the application spine: head + ordered args.
        let mut args_rev_chirho: Vec<&CoreExprChirho> = Vec::new();
        let mut cur_chirho = expr_chirho;
        while let CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } = cur_chirho
        {
            args_rev_chirho.push(arg_chirho);
            cur_chirho = fun_chirho;
        }
        let head_id_chirho = match cur_chirho {
            CoreExprChirho::VarChirho(id_chirho) => *id_chirho,
            _ => return None,
        };
        // Locally-shadowed names (`let (>>=) = ...`) are never intercepted.
        if self
            .local_shadow_ids_chirho
            .borrow()
            .contains(&head_id_chirho)
        {
            return None;
        }
        let head_name_chirho = self.names_chirho.get(&head_id_chirho)?.clone();
        let args_chirho: Vec<&CoreExprChirho> = args_rev_chirho.iter().rev().copied().collect();

        // return/pure are return-position polymorphic: no argument carries the
        // monad, so dispatch from the innermost positively-dispatched chain
        // context; with no context they keep their name (ReturnIOChirho
        // fallback, INV-001). workflow: monadic-dispatch-chirho
        if head_name_chirho == "return" || head_name_chirho == "pure" {
            let Some(context_key_chirho) = self.monad_context_stack_chirho.borrow().last().cloned()
            else {
                return Some(self.rebuild_preserved_method_app_chirho(
                    head_id_chirho,
                    &args_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            };
            let Some(inst_id_chirho) = self.lookup_dispatch_body_name_id_chirho(&format!(
                "$prim_Applicative_pure_{context_key_chirho}"
            )) else {
                return Some(self.rebuild_preserved_method_app_chirho(
                    head_id_chirho,
                    &args_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            };
            let mut result_chirho = CoreExprChirho::VarChirho(inst_id_chirho);
            for a_chirho in &args_chirho {
                result_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(result_chirho),
                    arg_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )),
                };
            }
            return Some(result_chirho);
        }

        // (instance-body name prefix, candidate dispatch-argument indexes)
        let (prefix_chirho, dispatch_indices_chirho): (&str, &[usize]) =
            match head_name_chirho.as_str() {
                "fmap" => ("$prim_Functor_fmap_", &[1usize]),
                ">>=" => ("$prim_Monad_>>=_", &[0usize]),
                ">>" => ("$prim_Monad_>>_", &[0usize]),
                "<|>" => ("$prim_Alternative_<|>_", &[1usize, 0usize]),
                _ => return None,
            };
        let dispatch_arg_chirho = dispatch_indices_chirho
            .iter()
            .find_map(|idx_chirho| args_chirho.get(*idx_chirho))?;
        // >>= / >> use the conservative key: a wrong positive key sends an IO
        // chain into another monad's body (worse than no dispatch). fmap and
        // <|> keep permissive value-shape inference because their dispatch
        // operands are plain values, not effectful IO chains.
        let value_key_opt_chirho = if matches!(head_name_chirho.as_str(), ">>=" | ">>") {
            self.infer_monad_dispatch_key_chirho(dispatch_arg_chirho, local_type_keys_chirho)
        } else {
            dispatch_indices_chirho.iter().find_map(|idx_chirho| {
                args_chirho.get(*idx_chirho).and_then(|arg_chirho| {
                    self.infer_type_key_for_rewrite_chirho(arg_chirho, local_type_keys_chirho)
                })
            })
        };
        let Some(value_key_chirho) = value_key_opt_chirho else {
            if matches!(head_name_chirho.as_str(), ">>=" | ">>") {
                return Some(self.rebuild_preserved_method_app_chirho(
                    head_id_chirho,
                    &args_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            }
            return None;
        };
        let head_key_chirho = Self::normalize_instance_head_key_chirho(&value_key_chirho);
        let Some(inst_id_chirho) =
            self.lookup_dispatch_body_name_id_chirho(&format!("{prefix_chirho}{head_key_chirho}"))
        else {
            if matches!(head_name_chirho.as_str(), ">>=" | ">>") {
                return Some(self.rebuild_preserved_method_app_chirho(
                    head_id_chirho,
                    &args_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            }
            return None;
        };
        // Monad chains carry their head key while their arguments are
        // rewritten so return/pure in the continuation dispatch to the same
        // monad. workflow: monadic-dispatch-chirho
        let is_monad_chain_chirho = matches!(head_name_chirho.as_str(), ">>=" | ">>");
        if is_monad_chain_chirho {
            self.monad_context_stack_chirho
                .borrow_mut()
                .push(head_key_chirho.clone());
        }
        let bind_payload_key_chirho = if head_name_chirho == ">>=" {
            self.monad_payload_type_key_chirho(
                dispatch_arg_chirho,
                &value_key_chirho,
                local_type_keys_chirho,
            )
        } else {
            None
        };
        // Rebuild the application with the instance body as head; rewrite args.
        let mut result_chirho = CoreExprChirho::VarChirho(inst_id_chirho);
        for (idx_chirho, a_chirho) in args_chirho.iter().enumerate() {
            let rewritten_arg_chirho = if head_name_chirho == "<|>" {
                if let Some(rewritten_chirho) = self.try_rewrite_typed_method_arg_chirho(
                    a_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    &head_key_chirho,
                ) {
                    rewritten_chirho
                } else {
                    self.rewrite_method_refs_with_locals_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )
                }
            } else if idx_chirho == 1 && head_name_chirho == ">>=" {
                if let Some(payload_key_chirho) = bind_payload_key_chirho.as_deref() {
                    let continuation_type_keys_chirho = Self::extend_first_lambda_type_key_chirho(
                        a_chirho,
                        payload_key_chirho,
                        local_type_keys_chirho,
                    );
                    self.rewrite_method_refs_with_locals_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        &continuation_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )
                } else {
                    self.rewrite_method_refs_with_locals_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )
                }
            } else {
                self.rewrite_method_refs_with_locals_chirho(
                    a_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                )
            };
            result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(rewritten_arg_chirho),
            };
        }
        if is_monad_chain_chirho {
            self.monad_context_stack_chirho.borrow_mut().pop();
        }
        Some(result_chirho)
    }

    /// Rewrite method references in an expression body.
    ///
    /// Given a mapping of in-scope dictionary variables
    /// `(class_name -> dict_id)`, replaces `VarChirho` references to
    /// overloaded methods with `($sel_Class_method $dClass)`.
    ///
    /// When a class method is applied to arguments whose type can be
    /// inferred from the expression (e.g. a constructor application like
    /// `Red`), the type-specific instance dictionary is selected instead
    /// of the default one in `dict_vars_chirho`.
    pub(super) fn rewrite_method_refs_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
    ) -> CoreExprChirho {
        self.rewrite_method_refs_with_locals_chirho(
            expr_chirho,
            dict_vars_chirho,
            &HashSet::new(),
            &HashMap::new(),
            &HashMap::new(),
        )
    }

    fn rewrite_method_refs_with_locals_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> CoreExprChirho {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                if let Some(dispatched_chirho) =
                    self.try_dispatch_contextual_return_pure_var_chirho(*id_chirho)
                {
                    return dispatched_chirho;
                }
                // Check if this var references a constrained user binding
                // that needs dict arguments inserted at the call site.
                if let Some(classes_chirho) = self.dict_param_bindings_chirho.get(id_chirho) {
                    let mut result_chirho = CoreExprChirho::VarChirho(*id_chirho);
                    for class_name_chirho in classes_chirho {
                        if let Some(dict_id_chirho) = Self::fallback_dict_for_class_chirho(
                            class_name_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                        ) {
                            result_chirho = CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(result_chirho),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
                            };
                        }
                    }
                    result_chirho
                } else {
                    // Rewrite a standalone method reference (not applied to args).
                    // This uses the default dict from dict_vars_chirho.
                    self.try_rewrite_method_var_chirho(
                        *id_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_instance_dicts_chirho,
                        None,
                    )
                }
            }
            CoreExprChirho::LitChirho(_) => expr_chirho.clone(),
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                if let CoreExprChirho::LamChirho { binder_chirho, .. } = fun_chirho.as_ref() {
                    let arg_key_chirho = self
                        .infer_strict_dispatch_key_for_rewrite_chirho(
                            arg_chirho,
                            local_type_keys_chirho,
                        )
                        .or_else(|| {
                            if self.expr_contains_numeric_default_marker_chirho(arg_chirho) {
                                Some("Int".to_string())
                            } else {
                                None
                            }
                        });
                    if let Some(arg_key_chirho) = arg_key_chirho {
                        let mut app_type_keys_chirho = local_type_keys_chirho.clone();
                        app_type_keys_chirho.insert(binder_chirho.id_chirho, arg_key_chirho);
                        return CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                                fun_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                &app_type_keys_chirho,
                                local_instance_dicts_chirho,
                            )),
                            arg_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                                arg_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_type_keys_chirho,
                                local_instance_dicts_chirho,
                            )),
                        };
                    }
                }

                if let CoreExprChirho::VarChirho(fun_id_chirho) = fun_chirho.as_ref() {
                    if self
                        .names_chirho
                        .get(fun_id_chirho)
                        .is_some_and(|name_chirho| {
                            name_chirho
                                .rsplit('.')
                                .next()
                                .is_some_and(|short_chirho| short_chirho == "print")
                        })
                        && self.print_arg_needs_int_default_chirho(arg_chirho)
                    {
                        let rewritten_fun_chirho = self.rewrite_method_refs_with_locals_chirho(
                            fun_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        );
                        let defaulted_arg_chirho = self
                            .rewrite_numeric_methods_with_type_key_chirho(
                                arg_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_instance_dicts_chirho,
                                "Int",
                            );
                        return CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(rewritten_fun_chirho),
                            arg_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                                &defaulted_arg_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_type_keys_chirho,
                                local_instance_dicts_chirho,
                            )),
                        };
                    }
                }

                // Higher-kinded class-method dispatch (fmap, ...): the method
                // resolves to a monomorphic default binding, so select the right
                // per-type instance body from the dispatch argument's type key
                // (normalized to the instance head, e.g. `Maybe Int` -> `Maybe`).
                if let Some(dispatched_chirho) = self.try_dispatch_hk_method_chirho(
                    expr_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ) {
                    return dispatched_chirho;
                }

                // Detect the pattern App(Var(method), arg) or
                // App(App(Var(method), arg1), arg2) to determine the
                // argument type for type-aware dictionary selection.
                if let Some((method_id_chirho, args_chirho, type_key_hint_chirho)) =
                    self.collect_method_app_chirho(expr_chirho)
                {
                    let method_name_chirho = self.names_chirho.get(&method_id_chirho).cloned();
                    // Infer the type key, combining multiple argument types
                    // for multi-parameter type classes.
                    let type_key_chirho = {
                        let class_name_chirho =
                            method_name_chirho.as_deref().and_then(|n_chirho| {
                                self.class_method_selector_for_name_chirho(n_chirho)
                                    .map(|(c_chirho, _)| c_chirho)
                            });
                        let param_count_chirho = class_name_chirho
                            .as_deref()
                            .and_then(|cn_chirho| self.class_param_count_chirho.get(cn_chirho))
                            .copied()
                            .unwrap_or(1);

                        if param_count_chirho > 1 {
                            // MPTC: infer type keys from first N arguments
                            let keys_chirho: Vec<String> = args_chirho
                                .iter()
                                .take(param_count_chirho)
                                .filter_map(|a_chirho| {
                                    self.infer_type_key_for_rewrite_chirho(
                                        a_chirho,
                                        local_type_keys_chirho,
                                    )
                                })
                                .collect();
                            if keys_chirho.len() == param_count_chirho {
                                Some(keys_chirho.join("_"))
                            } else {
                                keys_chirho.first().cloned()
                            }
                        } else {
                            type_key_hint_chirho.or_else(|| {
                                args_chirho.iter().find_map(|a_chirho| {
                                    self.infer_type_key_for_rewrite_chirho(
                                        a_chirho,
                                        local_type_keys_chirho,
                                    )
                                })
                            })
                        }
                    };

                    let type_key_chirho = match (method_name_chirho.as_deref(), type_key_chirho) {
                        (Some(method_name_chirho), Some(key_chirho)) => {
                            let short_name_chirho = method_name_chirho
                                .rsplit('.')
                                .next()
                                .unwrap_or(method_name_chirho);
                            if short_name_chirho == "mconcat"
                                && key_chirho.starts_with('[')
                                && key_chirho.ends_with(']')
                                && key_chirho.len() > 2
                            {
                                let elem_key_chirho = &key_chirho[1..key_chirho.len() - 1];
                                if elem_key_chirho.starts_with('[')
                                    || matches!(
                                        elem_key_chirho,
                                        "()" | "Ordering"
                                            | "Sum"
                                            | "Product"
                                            | "All"
                                            | "Any"
                                            | "First"
                                            | "Last"
                                            | "Endo"
                                    )
                                {
                                    Some(elem_key_chirho.to_string())
                                } else {
                                    Some(key_chirho)
                                }
                            } else {
                                Some(key_chirho)
                            }
                        }
                        (_, key_chirho) => key_chirho,
                    };

                    let type_key_override_chirho = type_key_chirho.as_deref();
                    let rewritten_method_chirho = self.try_rewrite_method_var_chirho(
                        method_id_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_instance_dicts_chirho,
                        type_key_override_chirho,
                    );

                    // Rebuild the application chain with rewritten args
                    let mut result_chirho = rewritten_method_chirho;
                    for a_chirho in &args_chirho {
                        let rewritten_arg_chirho =
                            if let Some(type_key_chirho) = type_key_override_chirho {
                                if let Some(rewritten_chirho) = self
                                    .try_rewrite_typed_method_arg_chirho(
                                        a_chirho,
                                        dict_vars_chirho,
                                        evidence_classes_chirho,
                                        local_instance_dicts_chirho,
                                        type_key_chirho,
                                    )
                                {
                                    rewritten_chirho
                                } else {
                                    self.rewrite_method_refs_with_locals_chirho(
                                        a_chirho,
                                        dict_vars_chirho,
                                        evidence_classes_chirho,
                                        local_type_keys_chirho,
                                        local_instance_dicts_chirho,
                                    )
                                }
                            } else {
                                self.rewrite_method_refs_with_locals_chirho(
                                    a_chirho,
                                    dict_vars_chirho,
                                    evidence_classes_chirho,
                                    local_type_keys_chirho,
                                    local_instance_dicts_chirho,
                                )
                            };
                        result_chirho = CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(result_chirho),
                            arg_chirho: Box::new(rewritten_arg_chirho),
                        };
                    }
                    return result_chirho;
                }

                // Detect calls to dict-parameterized user functions:
                // App(App(Var(f), arg1), arg2) where f has dict params.
                // Infer argument types to select the right dicts.
                if let Some((fn_id_chirho, classes_chirho, args_chirho)) =
                    self.collect_dict_param_app_chirho(expr_chirho)
                {
                    // Infer the type key from the actual arguments
                    let type_key_chirho = args_chirho.iter().find_map(|a_chirho| {
                        self.infer_strict_dispatch_key_for_rewrite_chirho(
                            a_chirho,
                            local_type_keys_chirho,
                        )
                    });

                    // Build dict args: for each required class, select the
                    // type-appropriate dict if we can infer the type
                    let mut result_chirho = CoreExprChirho::VarChirho(fn_id_chirho);
                    let classes_chirho = classes_chirho.to_vec();
                    for class_name_chirho in &classes_chirho {
                        let dict_id_chirho = if let Some(ref tk_chirho) = type_key_chirho {
                            // Try type-specific dict
                            self.instance_dicts_chirho
                                .get(&(class_name_chirho.clone(), tk_chirho.clone()))
                                .copied()
                                .or_else(|| {
                                    Self::fallback_dict_for_class_chirho(
                                        class_name_chirho,
                                        dict_vars_chirho,
                                        evidence_classes_chirho,
                                    )
                                })
                        } else {
                            Self::fallback_dict_for_class_chirho(
                                class_name_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                            )
                        };
                        if let Some(dict_id_chirho) = dict_id_chirho {
                            result_chirho = CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(result_chirho),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
                            };
                        }
                    }

                    // Rebuild the application chain with rewritten args
                    for a_chirho in &args_chirho {
                        let rewritten_arg_chirho =
                            if let Some(type_key_chirho) = type_key_chirho.as_deref() {
                                self.try_rewrite_typed_dict_param_arg_chirho(
                                    a_chirho,
                                    dict_vars_chirho,
                                    evidence_classes_chirho,
                                    local_type_keys_chirho,
                                    local_instance_dicts_chirho,
                                    type_key_chirho,
                                )
                                .or_else(|| {
                                    self.try_rewrite_typed_method_arg_chirho(
                                        a_chirho,
                                        dict_vars_chirho,
                                        evidence_classes_chirho,
                                        local_instance_dicts_chirho,
                                        type_key_chirho,
                                    )
                                })
                                .unwrap_or_else(|| {
                                    self.rewrite_method_refs_with_locals_chirho(
                                        a_chirho,
                                        dict_vars_chirho,
                                        evidence_classes_chirho,
                                        local_type_keys_chirho,
                                        local_instance_dicts_chirho,
                                    )
                                })
                            } else {
                                self.rewrite_method_refs_with_locals_chirho(
                                    a_chirho,
                                    dict_vars_chirho,
                                    evidence_classes_chirho,
                                    local_type_keys_chirho,
                                    local_instance_dicts_chirho,
                                )
                            };
                        result_chirho = CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(result_chirho),
                            arg_chirho: Box::new(rewritten_arg_chirho),
                        };
                    }
                    return result_chirho;
                }

                if let Some(rewritten_chirho) = self.try_rewrite_typed_higher_order_args_chirho(
                    expr_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ) {
                    return rewritten_chirho;
                }

                // Default: recursively rewrite fun and arg
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                        fun_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )),
                    arg_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                        arg_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )),
                }
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                let was_new_chirho = self
                    .local_shadow_ids_chirho
                    .borrow_mut()
                    .insert(binder_chirho.id_chirho);
                let mut lam_type_keys_chirho = local_type_keys_chirho.clone();
                if let Some(type_key_chirho) = self.binder_type_key_chirho(binder_chirho) {
                    lam_type_keys_chirho
                        .entry(binder_chirho.id_chirho)
                        .or_insert(type_key_chirho);
                }
                let result_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: binder_chirho.clone(),
                    body_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                        body_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        &lam_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )),
                };
                if was_new_chirho {
                    self.local_shadow_ids_chirho
                        .borrow_mut()
                        .remove(&binder_chirho.id_chirho);
                }
                result_chirho
            }
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => {
                // Shadow let/where-bound IDs so method rewriting skips them
                let mut added_chirho = Vec::new();
                let mut let_type_keys_chirho = local_type_keys_chirho.clone();
                for (b_chirho, _) in binds_chirho {
                    if !self
                        .local_shadow_ids_chirho
                        .borrow()
                        .contains(&b_chirho.id_chirho)
                    {
                        self.local_shadow_ids_chirho
                            .borrow_mut()
                            .insert(b_chirho.id_chirho);
                        added_chirho.push(b_chirho.id_chirho);
                    }
                    if let Some(type_key_chirho) = self.binder_type_key_chirho(b_chirho) {
                        let_type_keys_chirho
                            .entry(b_chirho.id_chirho)
                            .or_insert(type_key_chirho);
                    }
                }
                let result_chirho = CoreExprChirho::LetChirho {
                    rec_chirho: *rec_chirho,
                    binds_chirho: binds_chirho
                        .iter()
                        .map(|(b_chirho, r_chirho)| {
                            (
                                b_chirho.clone(),
                                self.rewrite_method_refs_with_locals_chirho(
                                    r_chirho,
                                    dict_vars_chirho,
                                    evidence_classes_chirho,
                                    &let_type_keys_chirho,
                                    local_instance_dicts_chirho,
                                ),
                            )
                        })
                        .collect(),
                    body_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                        body_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        &let_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )),
                };
                // Restore shadow set
                for id_chirho in added_chirho {
                    self.local_shadow_ids_chirho.borrow_mut().remove(&id_chirho);
                }
                result_chirho
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                result_ty_chirho,
                alts_chirho,
            } => CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                    scrutinee_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                )),
                bind_chirho: bind_chirho.clone(),
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: alts_chirho
                    .iter()
                    .map(|alt_chirho| {
                        let scrutinee_type_key_chirho = self
                            .infer_type_key_for_rewrite_chirho(
                                scrutinee_chirho,
                                local_type_keys_chirho,
                            )
                            .or_else(|| self.binder_type_key_chirho(bind_chirho))
                            .or_else(|| {
                                local_instance_dicts_chirho
                                    .keys()
                                    .map(|(_, type_key_chirho)| type_key_chirho)
                                    .find(|type_key_chirho| {
                                        type_key_chirho.starts_with('[')
                                            && type_key_chirho.ends_with(']')
                                    })
                                    .cloned()
                            });
                        let scrutinee_type_key_chirho = match scrutinee_type_key_chirho {
                            Some(type_key_chirho)
                                if !type_key_chirho.starts_with('[')
                                    && !type_key_chirho.contains(' ')
                                    && local_instance_dicts_chirho.keys().any(
                                        |(_, local_type_key_chirho)| {
                                            local_type_key_chirho.starts_with('[')
                                                && local_type_key_chirho.ends_with(']')
                                        },
                                    ) =>
                            {
                                local_instance_dicts_chirho
                                    .keys()
                                    .map(|(_, local_type_key_chirho)| local_type_key_chirho)
                                    .find(|local_type_key_chirho| {
                                        local_type_key_chirho.starts_with('[')
                                            && local_type_key_chirho.ends_with(']')
                                    })
                                    .cloned()
                            }
                            other_type_key_chirho => other_type_key_chirho,
                        };
                        let alt_type_keys_chirho = self.extend_alt_type_keys_chirho(
                            scrutinee_type_key_chirho,
                            bind_chirho,
                            alt_chirho,
                            local_type_keys_chirho,
                        );
                        CoreAltChirho {
                            con_chirho: alt_chirho.con_chirho.clone(),
                            binders_chirho: alt_chirho.binders_chirho.clone(),
                            rhs_chirho: self.rewrite_method_refs_with_locals_chirho(
                                &alt_chirho.rhs_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                &alt_type_keys_chirho,
                                local_instance_dicts_chirho,
                            ),
                        }
                    })
                    .collect(),
            },
            CoreExprChirho::TyLamChirho {
                ty_var_chirho,
                body_chirho,
            } => CoreExprChirho::TyLamChirho {
                ty_var_chirho: ty_var_chirho.clone(),
                body_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                    body_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                )),
            },
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
            } => {
                let annotated_monad_key_chirho = self
                    .annotated_monad_head_key_chirho(ty_chirho)
                    .filter(|_| self.has_direct_return_or_pure_head_chirho(inner_chirho));
                if let Some(monad_key_chirho) = annotated_monad_key_chirho.clone() {
                    self.monad_context_stack_chirho
                        .borrow_mut()
                        .push(monad_key_chirho);
                }
                let rewritten_inner_chirho = self.rewrite_method_refs_with_locals_chirho(
                    inner_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                );
                if annotated_monad_key_chirho.is_some() {
                    self.monad_context_stack_chirho.borrow_mut().pop();
                }
                CoreExprChirho::TyAppChirho {
                    expr_chirho: Box::new(rewritten_inner_chirho),
                    ty_chirho: ty_chirho.clone(),
                }
            }
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => CoreExprChirho::PrimOpChirho {
                name_chirho: name_chirho.clone(),
                args_chirho: args_chirho
                    .iter()
                    .map(|a_chirho| {
                        self.rewrite_method_refs_with_locals_chirho(
                            a_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
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
                    .map(|a_chirho| {
                        self.rewrite_method_refs_with_locals_chirho(
                            a_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        )
                    })
                    .collect(),
            },
        }
    }

    /// Transform a constrained top-level binding by adding dictionary lambda
    /// parameters and rewriting method references in the body.
    ///
    /// Given a binding `f = rhs` where `f :: forall a. (C1 a, C2 a) => T`,
    /// produces `f = \$dC1 -> \$dC2 -> rhs'` where `rhs'` has overloaded
    /// method references replaced with dictionary projections.
    pub fn add_dict_params_chirho(
        &mut self,
        binding_chirho: &CoreBindingChirho,
        scheme_chirho: &SchemeChirho,
    ) -> CoreBindingChirho {
        // Create dictionary binders and build the class→dict_id mapping.
        // For ground predicates (concrete types like Int, Char, Bool) with
        // known instance dictionaries, resolve directly instead of
        // abstracting over a dictionary lambda parameter.
        //
        // Even unconstrained bindings may reference class methods at
        // ground types (e.g. `main = myShow 42`), so we always build a
        // dict_vars map seeded with all known ground instance dicts and
        // then rewrite method references in the body.
        let mut dict_vars_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
        let mut evidence_classes_chirho: HashSet<String> = HashSet::new();

        // Seed with ground instance dictionaries. Prefer Int instances as
        // the default dict for numeric/comparison classes, since integer
        // literals are the most common and type defaulting resolves
        // ambiguous Num/Eq/Ord/Show to Int.
        for ((class_name_chirho, type_key_chirho), dict_id_chirho) in &self.instance_dicts_chirho {
            let is_int_chirho = type_key_chirho == "Int";
            if is_int_chirho {
                // Int always wins as default
                dict_vars_chirho.insert(class_name_chirho.clone(), *dict_id_chirho);
            } else {
                dict_vars_chirho
                    .entry(class_name_chirho.clone())
                    .or_insert(*dict_id_chirho);
            }
        }

        let mut dict_binders_chirho = Vec::new();

        for pred_chirho in &scheme_chirho.preds_chirho {
            evidence_classes_chirho.insert(pred_chirho.class_name_chirho.clone());
            let is_ground_chirho = !matches!(pred_chirho.ty_chirho, TyChirho::VarChirho(_));
            let resolved_chirho = if is_ground_chirho {
                let type_key_chirho = format!("{}", pred_chirho.ty_chirho);
                self.instance_dicts_chirho
                    .get(&(pred_chirho.class_name_chirho.clone(), type_key_chirho))
                    .copied()
            } else if Self::is_defaultable_pred_chirho(pred_chirho, scheme_chirho) {
                // Type defaulting (Haskell 2010 §4.3.4): when a predicate
                // has an ambiguous type variable (does not appear in any
                // function argument position) and the class is one of the
                // standard numeric / Prelude classes, default to Int.
                let default_type_chirho = match pred_chirho.class_name_chirho.as_str() {
                    "Num" | "Eq" | "Ord" | "Show" | "Read" | "Enum" | "Bounded" | "Integral"
                    | "Real" | "RealFrac" | "Floating" | "RealFloat" => Some("Int"),
                    "IsString" => Some("[Char]"),
                    "IsList" => Some("[t9037]"),
                    _ => None,
                };
                default_type_chirho.and_then(|dt_chirho| {
                    self.instance_dicts_chirho
                        .get(&(pred_chirho.class_name_chirho.clone(), dt_chirho.to_string()))
                        .copied()
                })
            } else {
                None
            };

            if let Some(inst_id_chirho) = resolved_chirho {
                // Ground predicate with known instance — use concrete dict
                dict_vars_chirho.insert(pred_chirho.class_name_chirho.clone(), inst_id_chirho);
            } else {
                // Unresolved — abstract over a dictionary lambda parameter
                let dict_name_chirho = format!("$d{}", pred_chirho.class_name_chirho);
                let dict_ty_chirho =
                    TyChirho::ConChirho(format!("$Dict_{}", pred_chirho.class_name_chirho));
                let dict_binder_chirho =
                    self.fresh_binder_chirho(&dict_name_chirho, dict_ty_chirho);
                dict_vars_chirho.insert(
                    pred_chirho.class_name_chirho.clone(),
                    dict_binder_chirho.id_chirho,
                );
                dict_binders_chirho.push(dict_binder_chirho);
            }
        }

        // Record which classes this binding abstracts over so that call
        // sites can insert the corresponding dict arguments.
        if !dict_binders_chirho.is_empty() {
            let classes_chirho = Self::dict_param_classes_for_scheme_chirho(scheme_chirho);
            if !classes_chirho.is_empty() {
                self.dict_param_bindings_chirho
                    .insert(binding_chirho.binder_chirho.id_chirho, classes_chirho);
            }
        }

        let mut local_type_keys_chirho: HashMap<CoreIdChirho, String> = HashMap::new();
        self.seed_value_binder_type_keys_chirho(
            &binding_chirho.rhs_chirho,
            &scheme_chirho.ty_chirho,
            &mut local_type_keys_chirho,
        );

        let mut local_instance_dicts_chirho: HashMap<(String, String), CoreIdChirho> =
            HashMap::new();
        let mut self_dict_let_binds_chirho: Vec<(BinderChirho, CoreExprChirho)> = Vec::new();
        let mut binding_is_rec_chirho = binding_chirho.is_rec_chirho;

        if let Some((class_name_chirho, _method_name_chirho, parsed_type_key_chirho)) =
            self.parse_prim_binding_info_chirho(&binding_chirho.binder_chirho.name_chirho)
        {
            let mut current_expr_chirho = &binding_chirho.rhs_chirho;
            while let CoreExprChirho::TyLamChirho { body_chirho, .. } = current_expr_chirho {
                current_expr_chirho = body_chirho;
            }
            if let CoreExprChirho::LamChirho { binder_chirho, .. } = current_expr_chirho {
                local_type_keys_chirho
                    .insert(binder_chirho.id_chirho, parsed_type_key_chirho.clone());
            }

            let self_type_key_chirho = self
                .binding_instance_type_key_chirho(&class_name_chirho, scheme_chirho)
                .unwrap_or_else(|| parsed_type_key_chirho.clone());
            if !self
                .instance_dicts_chirho
                .contains_key(&(class_name_chirho.clone(), self_type_key_chirho.clone()))
            {
                if let Some((has_no_supers_chirho, method_slot_names_chirho)) = self
                    .layouts_chirho
                    .get(&class_name_chirho)
                    .map(|layout_chirho| {
                        (
                            layout_chirho.super_slots_chirho.is_empty(),
                            layout_chirho
                                .method_slots_chirho
                                .iter()
                                .map(|(slot_method_name_chirho, _)| slot_method_name_chirho.clone())
                                .collect::<Vec<_>>(),
                        )
                    })
                {
                    if has_no_supers_chirho {
                        let self_dict_binder_chirho = self.fresh_binder_chirho(
                            &format!("$d{}Self", class_name_chirho),
                            TyChirho::ConChirho(format!("$Dict_{}", class_name_chirho)),
                        );
                        let mut field_args_chirho = Vec::new();
                        for slot_method_name_chirho in &method_slot_names_chirho {
                            let prim_name_chirho = format!(
                                "$prim_{}_{}_{}",
                                class_name_chirho, slot_method_name_chirho, parsed_type_key_chirho
                            );
                            let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
                            let mut method_expr_chirho = CoreExprChirho::VarChirho(prim_id_chirho);
                            for pred_chirho in &scheme_chirho.preds_chirho {
                                if let Some(dict_id_chirho) =
                                    dict_vars_chirho.get(&pred_chirho.class_name_chirho)
                                {
                                    method_expr_chirho = CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(method_expr_chirho),
                                        arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                            *dict_id_chirho,
                                        )),
                                    };
                                }
                            }
                            field_args_chirho.push(method_expr_chirho);
                        }
                        self_dict_let_binds_chirho.push((
                            self_dict_binder_chirho.clone(),
                            CoreExprChirho::ConAppChirho {
                                con_name_chirho: format!("$Dict_{}", class_name_chirho),
                                args_chirho: field_args_chirho,
                            },
                        ));
                        local_instance_dicts_chirho.insert(
                            (class_name_chirho.clone(), self_type_key_chirho),
                            self_dict_binder_chirho.id_chirho,
                        );
                        local_instance_dicts_chirho.insert(
                            (class_name_chirho, parsed_type_key_chirho),
                            self_dict_binder_chirho.id_chirho,
                        );
                        binding_is_rec_chirho = true;
                    }
                }
            }
        }

        // Superclass extraction: for each dict binder whose class has
        // superclasses, generate let-bindings that extract the superclass
        // dicts from the subclass dict.  This is needed after context
        // reduction removes redundant predicates (e.g. Eq a removed
        // when Num a is present, since Num has Eq as a superclass).
        let mut super_let_binds_chirho: Vec<(BinderChirho, CoreExprChirho)> = Vec::new();
        {
            // Collect classes that have dict binders (unresolved predicates)
            let classes_with_binders_chirho: Vec<String> = dict_binders_chirho
                .iter()
                .filter_map(|b_chirho| {
                    // The dict binder name is "$dClassName"
                    let name_chirho = &b_chirho.name_chirho;
                    name_chirho
                        .strip_prefix("$d")
                        .map(|s_chirho| s_chirho.to_string())
                })
                .collect();

            // Collect (class, super, sel_id, sub_dict_id) tuples before
            // mutating self via fresh_binder_chirho.
            let mut extractions_chirho: Vec<(String, String, CoreIdChirho, CoreIdChirho)> =
                Vec::new();
            for class_name_chirho in &classes_with_binders_chirho {
                if let Some(layout_chirho) = self.layouts_chirho.get(class_name_chirho) {
                    for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
                        let already_has_binder_chirho =
                            classes_with_binders_chirho.contains(super_name_chirho);
                        if !already_has_binder_chirho {
                            if let Some(sel_id_chirho) = self
                                .super_selectors_chirho
                                .get(&(class_name_chirho.clone(), super_name_chirho.clone()))
                            {
                                if let Some(sub_dict_id_chirho) =
                                    dict_vars_chirho.get(class_name_chirho)
                                {
                                    extractions_chirho.push((
                                        class_name_chirho.clone(),
                                        super_name_chirho.clone(),
                                        *sel_id_chirho,
                                        *sub_dict_id_chirho,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            // Now create binders and let-bindings
            for (_class_chirho, super_name_chirho, sel_id_chirho, sub_dict_id_chirho) in
                extractions_chirho
            {
                let super_dict_binder_chirho = self.fresh_binder_chirho(
                    &format!("$d{}", super_name_chirho),
                    TyChirho::ConChirho(format!("$Dict_{}", super_name_chirho)),
                );
                let extraction_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(sub_dict_id_chirho)),
                };
                evidence_classes_chirho.insert(super_name_chirho.clone());
                dict_vars_chirho.insert(super_name_chirho, super_dict_binder_chirho.id_chirho);
                super_let_binds_chirho.push((super_dict_binder_chirho, extraction_chirho));
            }
        }

        // Rewrite method references in the original body. For a binding whose
        // signature proves a concrete result monad, only push that context when
        // the RHS is directly return/pure after value lambdas.
        let signature_monad_key_chirho = self
            .body_backed_monad_head_key_for_ty_chirho(&scheme_chirho.ty_chirho, true)
            .filter(|_| {
                self.has_direct_return_or_pure_head_chirho(Self::strip_value_lams_chirho(
                    &binding_chirho.rhs_chirho,
                ))
            });
        if let Some(monad_key_chirho) = signature_monad_key_chirho.clone() {
            self.monad_context_stack_chirho
                .borrow_mut()
                .push(monad_key_chirho);
        }
        let mut rhs_chirho = self.rewrite_method_refs_with_locals_chirho(
            &binding_chirho.rhs_chirho,
            &dict_vars_chirho,
            &evidence_classes_chirho,
            &local_type_keys_chirho,
            &local_instance_dicts_chirho,
        );
        if signature_monad_key_chirho.is_some() {
            self.monad_context_stack_chirho.borrow_mut().pop();
        }

        // Wrap in only the superclass extraction let-bindings that the
        // rewritten body actually references. Unused extraction lets can keep
        // an otherwise-resolved entry point abstracted over a dead subclass
        // dictionary.
        let used_super_ids_chirho = crate::simplify_chirho::free_vars_chirho(&rhs_chirho);
        for (binder_chirho, extraction_chirho) in super_let_binds_chirho.iter().rev() {
            if !used_super_ids_chirho.contains(&binder_chirho.id_chirho) {
                continue;
            }
            rhs_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(binder_chirho.clone(), extraction_chirho.clone())],
                body_chirho: Box::new(rhs_chirho),
            };
        }

        for (binder_chirho, self_dict_expr_chirho) in self_dict_let_binds_chirho.iter().rev() {
            rhs_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(binder_chirho.clone(), self_dict_expr_chirho.clone())],
                body_chirho: Box::new(rhs_chirho),
            };
        }

        // Wrap in dictionary lambdas only for unresolved predicates.
        // For monomorphic entry points (e.g. `main`) where all method
        // calls resolved to concrete instance dicts, strip unused dict
        // lambdas so the runtime can evaluate them directly.
        let is_entry_chirho = binding_chirho.binder_chirho.name_chirho == "main";
        let free_ids_chirho = if is_entry_chirho {
            crate::simplify_chirho::free_vars_chirho(&rhs_chirho)
        } else {
            // For non-main bindings keep all dict lambdas unconditionally
            HashSet::new()
        };
        let mut used_dict_binders_chirho: Vec<&BinderChirho> = Vec::new();
        for dict_binder_chirho in dict_binders_chirho.iter().rev() {
            if !is_entry_chirho || free_ids_chirho.contains(&dict_binder_chirho.id_chirho) {
                rhs_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: dict_binder_chirho.clone(),
                    body_chirho: Box::new(rhs_chirho),
                };
                used_dict_binders_chirho.push(dict_binder_chirho);
            }
        }

        // Update the binder's type to include dictionary parameters
        // (only for dict binders that were actually kept as lambdas)
        let mut result_ty_chirho = binding_chirho.binder_chirho.ty_chirho.clone();
        for dict_binder_chirho in used_dict_binders_chirho.iter() {
            result_ty_chirho =
                TyChirho::fun_chirho(dict_binder_chirho.ty_chirho.clone(), result_ty_chirho);
        }

        CoreBindingChirho {
            binder_chirho: BinderChirho {
                ty_chirho: result_ty_chirho,
                ..binding_chirho.binder_chirho.clone()
            },
            rhs_chirho,
            is_rec_chirho: binding_is_rec_chirho,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }
    }

    /// Run the dictionary-passing transform on a Core module.
    pub fn transform_module_chirho(
        &mut self,
        module_chirho: &CoreModuleChirho,
        type_env_chirho: &TyEnvChirho,
        class_env_chirho: &ClassEnvChirho,
    ) -> CoreModuleChirho {
        self.seed_body_backed_bindings_chirho(module_chirho);

        // Build layouts from the class environment
        self.build_layouts_chirho(class_env_chirho);

        // Generate method selectors
        self.generate_selectors_chirho();

        // Generate built-in $prim_ bindings for standard class methods
        self.generate_builtin_prim_bindings_chirho();

        // Generate Prelude function bindings (not, id, const)
        self.generate_prelude_bindings_chirho();

        // Generate instance dictionary bindings
        self.generate_instance_dicts_chirho(class_env_chirho);

        // Generate GND (GeneralizedNewtypeDeriving) instance dicts
        self.generate_gnd_dicts_chirho(class_env_chirho);

        // Generate ground specializations of conditional instances
        // (e.g. Eq [Double] from Eq a => Eq [a] + Eq Double)
        self.generate_conditional_ground_dicts_chirho(class_env_chirho);

        // Seed local_shadow_ids with top-level user bindings whose name
        // collides with a class method (e.g. user defines `toList` which
        // shadows the IsList class method).  Without this, the dict pass
        // would incorrectly rewrite user calls to `toList` as if they were
        // the IsList method.
        for binding_chirho in &module_chirho.bindings_chirho {
            let name_chirho = &binding_chirho.binder_chirho.name_chirho;
            if self.method_selectors_chirho.contains_key(name_chirho) {
                // Only shadow if the binding is NOT itself a class method
                // (i.e. it has no matching class predicate in its type scheme)
                let is_class_method_chirho = type_env_chirho
                    .lookup_chirho(name_chirho)
                    .map(|s_chirho| {
                        s_chirho.preds_chirho.iter().any(|p_chirho| {
                            let (class_chirho, _) = &self.method_selectors_chirho[name_chirho];
                            &p_chirho.class_name_chirho == class_chirho
                        })
                    })
                    .unwrap_or(false);
                if !is_class_method_chirho {
                    self.local_shadow_ids_chirho
                        .borrow_mut()
                        .insert(binding_chirho.binder_chirho.id_chirho);
                }
            }
            if let Some(scheme_chirho) = type_env_chirho.lookup_chirho(name_chirho) {
                let classes_chirho = Self::dict_param_classes_for_scheme_chirho(scheme_chirho);
                if !classes_chirho.is_empty() {
                    self.dict_param_bindings_chirho
                        .insert(binding_chirho.binder_chirho.id_chirho, classes_chirho);
                }
            }
        }

        // Transform each binding
        let mut bindings_chirho = Vec::new();

        for binding_chirho in &module_chirho.bindings_chirho {
            let name_chirho = &binding_chirho.binder_chirho.name_chirho;

            // Look up the type scheme for this binding
            if let Some(scheme_chirho) = type_env_chirho.lookup_chirho(name_chirho) {
                let transformed_chirho = self.add_dict_params_chirho(binding_chirho, scheme_chirho);
                bindings_chirho.push(transformed_chirho);
            } else {
                // Binding not in type env (e.g. $prim_ instance method
                // bodies).  Still rewrite class-method references in the
                // body so that Var(+) etc. are resolved to selectors.
                let mut dict_vars_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
                let mut evidence_classes_chirho: HashSet<String> = HashSet::new();
                for ((class_name_chirho, type_key_chirho), dict_id_chirho) in
                    &self.instance_dicts_chirho
                {
                    let is_int_chirho = type_key_chirho == "Int";
                    if is_int_chirho {
                        dict_vars_chirho.insert(class_name_chirho.clone(), *dict_id_chirho);
                    } else {
                        dict_vars_chirho
                            .entry(class_name_chirho.clone())
                            .or_insert(*dict_id_chirho);
                    }
                }
                if let Some((class_name_chirho, _method_name_chirho, parsed_type_key_chirho)) =
                    self.parse_prim_binding_info_chirho(name_chirho)
                {
                    if let Some(context_classes_chirho) = self
                        .conditional_context_classes_for_prim_binding_chirho(
                            class_env_chirho,
                            &class_name_chirho,
                            &parsed_type_key_chirho,
                        )
                    {
                        let mut dict_binders_chirho = Vec::new();
                        for context_class_chirho in context_classes_chirho {
                            let dict_binder_chirho = self.fresh_binder_chirho(
                                &format!("$d{}", context_class_chirho),
                                TyChirho::ConChirho(format!("$Dict_{}", context_class_chirho)),
                            );
                            evidence_classes_chirho.insert(context_class_chirho.clone());
                            dict_vars_chirho
                                .insert(context_class_chirho, dict_binder_chirho.id_chirho);
                            dict_binders_chirho.push(dict_binder_chirho);
                        }

                        let mut local_type_keys_chirho: HashMap<CoreIdChirho, String> =
                            HashMap::new();
                        let mut current_expr_chirho = &binding_chirho.rhs_chirho;
                        while let CoreExprChirho::TyLamChirho { body_chirho, .. } =
                            current_expr_chirho
                        {
                            current_expr_chirho = body_chirho;
                        }
                        if let CoreExprChirho::LamChirho { binder_chirho, .. } = current_expr_chirho
                        {
                            local_type_keys_chirho
                                .insert(binder_chirho.id_chirho, parsed_type_key_chirho.clone());
                        }

                        let mut local_instance_dicts_chirho: HashMap<
                            (String, String),
                            CoreIdChirho,
                        > = HashMap::new();
                        let mut self_dict_let_binds_chirho = Vec::new();
                        let mut binding_is_rec_chirho = binding_chirho.is_rec_chirho;

                        if !self.instance_dicts_chirho.contains_key(&(
                            class_name_chirho.clone(),
                            parsed_type_key_chirho.clone(),
                        )) {
                            if let Some((has_no_supers_chirho, method_slot_names_chirho)) = self
                                .layouts_chirho
                                .get(&class_name_chirho)
                                .map(|layout_chirho| {
                                    (
                                        layout_chirho.super_slots_chirho.is_empty(),
                                        layout_chirho
                                            .method_slots_chirho
                                            .iter()
                                            .map(|(slot_method_name_chirho, _)| {
                                                slot_method_name_chirho.clone()
                                            })
                                            .collect::<Vec<_>>(),
                                    )
                                })
                            {
                                if has_no_supers_chirho {
                                    let self_dict_binder_chirho = self.fresh_binder_chirho(
                                        &format!("$d{}Self", class_name_chirho),
                                        TyChirho::ConChirho(format!("$Dict_{}", class_name_chirho)),
                                    );
                                    let mut field_args_chirho = Vec::new();
                                    for slot_method_name_chirho in &method_slot_names_chirho {
                                        let prim_name_chirho = format!(
                                            "$prim_{}_{}_{}",
                                            class_name_chirho,
                                            slot_method_name_chirho,
                                            parsed_type_key_chirho
                                        );
                                        let prim_id_chirho =
                                            self.resolve_or_fresh_id_chirho(&prim_name_chirho);
                                        let mut method_expr_chirho =
                                            CoreExprChirho::VarChirho(prim_id_chirho);
                                        for dict_binder_chirho in &dict_binders_chirho {
                                            method_expr_chirho = CoreExprChirho::AppChirho {
                                                fun_chirho: Box::new(method_expr_chirho),
                                                arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                                    dict_binder_chirho.id_chirho,
                                                )),
                                            };
                                        }
                                        field_args_chirho.push(method_expr_chirho);
                                    }
                                    self_dict_let_binds_chirho.push((
                                        self_dict_binder_chirho.clone(),
                                        CoreExprChirho::ConAppChirho {
                                            con_name_chirho: format!("$Dict_{}", class_name_chirho),
                                            args_chirho: field_args_chirho,
                                        },
                                    ));
                                    local_instance_dicts_chirho.insert(
                                        (class_name_chirho.clone(), parsed_type_key_chirho.clone()),
                                        self_dict_binder_chirho.id_chirho,
                                    );
                                    binding_is_rec_chirho = true;
                                }
                            }
                        }

                        let mut rewritten_rhs_chirho = self.rewrite_method_refs_with_locals_chirho(
                            &binding_chirho.rhs_chirho,
                            &dict_vars_chirho,
                            &evidence_classes_chirho,
                            &local_type_keys_chirho,
                            &local_instance_dicts_chirho,
                        );
                        for (binder_chirho, self_dict_expr_chirho) in
                            self_dict_let_binds_chirho.iter().rev()
                        {
                            rewritten_rhs_chirho = CoreExprChirho::LetChirho {
                                rec_chirho: false,
                                binds_chirho: vec![(
                                    binder_chirho.clone(),
                                    self_dict_expr_chirho.clone(),
                                )],
                                body_chirho: Box::new(rewritten_rhs_chirho),
                            };
                        }
                        let mut result_ty_chirho = binding_chirho.binder_chirho.ty_chirho.clone();
                        for dict_binder_chirho in dict_binders_chirho.iter().rev() {
                            rewritten_rhs_chirho = CoreExprChirho::LamChirho {
                                binder_chirho: dict_binder_chirho.clone(),
                                body_chirho: Box::new(rewritten_rhs_chirho),
                            };
                            result_ty_chirho = TyChirho::fun_chirho(
                                dict_binder_chirho.ty_chirho.clone(),
                                result_ty_chirho,
                            );
                        }

                        bindings_chirho.push(CoreBindingChirho {
                            binder_chirho: BinderChirho {
                                ty_chirho: result_ty_chirho,
                                ..binding_chirho.binder_chirho.clone()
                            },
                            rhs_chirho: rewritten_rhs_chirho,
                            is_rec_chirho: binding_is_rec_chirho,
                            inline_chirho: InlineAnnotationChirho::NoneChirho,
                        });
                        continue;
                    }
                }

                let rewritten_rhs_chirho = if let Some((
                    _class_name_chirho,
                    _method_name_chirho,
                    parsed_type_key_chirho,
                )) = self.parse_prim_binding_info_chirho(name_chirho)
                {
                    let mut local_type_keys_chirho: HashMap<CoreIdChirho, String> = HashMap::new();
                    let mut current_expr_chirho = &binding_chirho.rhs_chirho;
                    while let CoreExprChirho::TyLamChirho { body_chirho, .. } = current_expr_chirho
                    {
                        current_expr_chirho = body_chirho;
                    }
                    if let CoreExprChirho::LamChirho { binder_chirho, .. } = current_expr_chirho {
                        local_type_keys_chirho
                            .insert(binder_chirho.id_chirho, parsed_type_key_chirho);
                    }
                    self.rewrite_method_refs_with_locals_chirho(
                        &binding_chirho.rhs_chirho,
                        &dict_vars_chirho,
                        &evidence_classes_chirho,
                        &local_type_keys_chirho,
                        &HashMap::new(),
                    )
                } else {
                    self.rewrite_method_refs_chirho(&binding_chirho.rhs_chirho, &dict_vars_chirho)
                };
                bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: binding_chirho.binder_chirho.clone(),
                    rhs_chirho: rewritten_rhs_chirho,
                    is_rec_chirho: binding_chirho.is_rec_chirho,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                });
            }
        }

        // Prepend generated dictionary bindings
        let mut all_bindings_chirho = self.generated_bindings_chirho.clone();
        all_bindings_chirho.extend(bindings_chirho);

        CoreModuleChirho {
            name_chirho: module_chirho.name_chirho.clone(),
            bindings_chirho: all_bindings_chirho,
            names_chirho: self.names_chirho.clone(),
            specialize_pragmas_chirho: module_chirho.specialize_pragmas_chirho.clone(),
            foreign_exports_chirho: module_chirho.foreign_exports_chirho.clone(),
        }
    }

    /// Finish the transform and return the result.
    pub fn finish_chirho(self, module_chirho: CoreModuleChirho) -> DictPassResultChirho {
        DictPassResultChirho {
            module_chirho,
            names_chirho: self.names_chirho,
            layouts_chirho: self.layouts_chirho,
        }
    }
}
