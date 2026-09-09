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

    fn key_token_is_type_var_chirho(token_chirho: &str) -> bool {
        token_chirho.strip_prefix('t').is_some_and(|rest_chirho| {
            !rest_chirho.is_empty()
                && rest_chirho
                    .chars()
                    .all(|c_chirho| c_chirho.is_ascii_digit())
        })
    }

    fn type_key_has_type_var_chirho(type_key_chirho: &str) -> bool {
        type_key_chirho
            .split(|c_chirho: char| !(c_chirho.is_ascii_alphanumeric() || c_chirho == '_'))
            .any(Self::key_token_is_type_var_chirho)
    }

    fn concrete_local_type_key_chirho(type_key_chirho: String) -> Option<String> {
        if type_key_chirho.is_empty() || Self::type_key_has_type_var_chirho(&type_key_chirho) {
            None
        } else {
            Some(type_key_chirho)
        }
    }

    fn concrete_binder_type_key_chirho(&self, binder_chirho: &BinderChirho) -> Option<String> {
        self.binder_type_key_chirho(binder_chirho)
            .and_then(Self::concrete_local_type_key_chirho)
    }

    fn infer_local_value_binding_type_key_chirho(
        &self,
        rhs_chirho: &CoreExprChirho,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<String> {
        self.infer_type_key_for_rewrite_chirho(rhs_chirho, local_type_keys_chirho)
            .or_else(|| {
                self.expr_contains_numeric_default_marker_chirho(rhs_chirho)
                    .then(|| "Int".to_string())
            })
            .and_then(Self::concrete_local_type_key_chirho)
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
            // A type application carries the type we are trying to recover, so USE it when
            // it is concrete. Previously this arm discarded `ty_chirho` and recursed into
            // the inner expression — which meant an explicit annotation was ignored:
            //   show (id [1,2,3] :: [Int])   rendered as a heap address
            // because the inner expression is an application, has no binder to read a type
            // off, and the `Show` fallback is `showInt#`. The sibling resolver
            // `infer_strict_dispatch_key_for_rewrite_chirho` already reads `ty_chirho`;
            // these two had simply diverged.
            // See spec-chirho/bug-native-print-list-pointer-chirho.md
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
                ..
            } => Self::concrete_local_type_key_chirho(format!("{ty_chirho}")).or_else(|| {
                self.infer_type_key_for_rewrite_chirho(inner_chirho, local_type_keys_chirho)
            }),
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
                if self
                    .dict_param_bindings_chirho
                    .contains_key(&self.canonical_id_chirho(*head_id_chirho))
                {
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
                    return self.infer_dict_param_call_type_key_chirho(
                        &args_chirho,
                        local_type_keys_chirho,
                    );
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

    fn collect_app_head_var_chirho<'a>(
        expr_chirho: &'a CoreExprChirho,
    ) -> Option<(CoreIdChirho, Vec<&'a CoreExprChirho>)> {
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
        let CoreExprChirho::VarChirho(head_id_chirho) = current_chirho else {
            return None;
        };
        args_chirho.reverse();
        Some((*head_id_chirho, args_chirho))
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

    fn try_rewrite_show_numeric_default_chirho(
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

    fn try_rewrite_prim_show_int_arg_chirho(
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

    fn try_rewrite_modify_ioref_numeric_default_chirho(
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

    fn try_rewrite_strict_numeric_args_chirho(
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
            if self
                .dict_param_bindings_chirho
                .contains_key(&self.canonical_id_chirho(*id_chirho))
            {
                return None;
            }
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
    /// The canonical id behind a reference occurrence id (itself otherwise).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    fn canonical_id_chirho(&self, id_chirho: CoreIdChirho) -> CoreIdChirho {
        self.method_occurrence_canon_chirho
            .get(&id_chirho)
            .map(|(_name_chirho, canonical_chirho)| *canonical_chirho)
            .unwrap_or(id_chirho)
    }

    /// The dictionary a constrained reference's predicate is served by, from
    /// the checker's evidence for that position: an instance dictionary for a
    /// proven key, the enclosing binding's own parameter for
    /// `OWN_DICTIONARY_KEY_CHIRHO`, and nothing when nothing was proved.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    fn evidence_dict_for_class_chirho(
        &self,
        class_name_chirho: &str,
        ty_key_chirho: Option<&str>,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreIdChirho> {
        let key_chirho = ty_key_chirho?;
        if haskelujah_typing_chirho::infer_chirho::is_own_dictionary_key_chirho(key_chirho) {
            return Self::own_dict_chirho(
                class_name_chirho,
                key_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
            );
        }
        let mut candidate_keys_chirho = vec![key_chirho.to_string()];
        if Self::should_normalize_instance_head_for_class_chirho(class_name_chirho) {
            let normalized_chirho = Self::normalize_instance_head_key_chirho(key_chirho);
            if normalized_chirho != key_chirho {
                candidate_keys_chirho.push(normalized_chirho);
            }
        }
        candidate_keys_chirho
            .into_iter()
            .find_map(|candidate_chirho| {
                let lookup_chirho = (class_name_chirho.to_string(), candidate_chirho);
                local_instance_dicts_chirho
                    .get(&lookup_chirho)
                    .or_else(|| self.instance_dicts_chirho.get(&lookup_chirho))
                    .copied()
            })
    }

    /// The binding's own dictionary parameter an own-parameter key names: the
    /// parameter of that predicate index when the key carries one, else the
    /// binding's parameter for the class (superclass extractions included).
    fn own_dict_chirho(
        class_name_chirho: &str,
        key_chirho: &str,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
    ) -> Option<CoreIdChirho> {
        dict_vars_chirho.get(key_chirho).copied().or_else(|| {
            Self::fallback_dict_for_class_chirho(
                class_name_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
            )
        })
    }

    /// Take the evidence for the next predicate of `class_name` from a
    /// reference's evidence list (scheme order), if the checker recorded one.
    fn take_reference_evidence_chirho(
        evidence_chirho: &mut Vec<(String, Option<String>)>,
        class_name_chirho: &str,
    ) -> Option<Option<String>> {
        let position_chirho = evidence_chirho
            .iter()
            .position(|(class_chirho, _key_chirho)| class_chirho == class_name_chirho)?;
        Some(evidence_chirho.remove(position_chirho).1)
    }

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
            if let Some(classes_chirho) = self
                .dict_param_bindings_chirho
                .get(&self.canonical_id_chirho(*id_chirho))
            {
                args_chirho.reverse();
                return Some((*id_chirho, classes_chirho, args_chirho));
            }
        }

        None
    }

    fn infer_dict_param_call_type_key_chirho(
        &self,
        args_chirho: &[&CoreExprChirho],
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<String> {
        args_chirho.iter().find_map(|a_chirho| {
            self.infer_strict_dispatch_key_for_rewrite_chirho(a_chirho, local_type_keys_chirho)
                .or_else(|| {
                    self.expr_contains_numeric_default_marker_chirho(a_chirho)
                        .then(|| "Int".to_string())
                })
        })
    }

    fn is_class_method_name_chirho(&self, name_chirho: &str) -> bool {
        let short_name_chirho = name_chirho.rsplit('.').next().unwrap_or(name_chirho);
        let stripped_name_chirho = short_name_chirho
            .strip_prefix('(')
            .and_then(|n_chirho| n_chirho.strip_suffix(')'))
            .unwrap_or(short_name_chirho);

        self.method_selectors_chirho.contains_key(name_chirho)
            || self.method_selectors_chirho.contains_key(short_name_chirho)
            || self
                .method_selectors_chirho
                .contains_key(stripped_name_chirho)
            || self
                .class_method_selector_for_name_chirho(name_chirho)
                .is_some()
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
        // Dictionary evidence: an occurrence the checker proved dispatches by
        // that proof, before any syntactic guess — the engine's prim row when
        // it has one, else the keyed selector path below with the proven key.
        // workflow: language-features-chirho/dictionary-evidence-chirho
        let evidence_chirho = self.occurrence_evidence_chirho.get(&id_chirho);
        if let Some((evidence_class_chirho, evidence_key_chirho)) = evidence_chirho {
            if let Some((method_name_chirho, _canonical_chirho)) =
                self.method_occurrence_canon_chirho.get(&id_chirho)
            {
                // Literal wrappers take the selector form (see
                // `occurrence_head_replacement_chirho`).
                if !matches!(method_name_chirho.as_str(), "fromInteger" | "fromString") {
                    let prim_name_chirho = format!(
                        "$prim_{evidence_class_chirho}_{method_name_chirho}_{evidence_key_chirho}"
                    );
                    if let Some(prim_id_chirho) =
                        self.find_global_id_by_name_chirho(&prim_name_chirho)
                    {
                        return CoreExprChirho::VarChirho(prim_id_chirho);
                    }
                }
            }
        }
        let type_key_override_chirho = evidence_chirho
            .map(|(_class_chirho, key_chirho)| key_chirho.as_str())
            .or(type_key_override_chirho);
        if let Some(name_chirho) = self.names_chirho.get(&id_chirho) {
            if let Some((class_name_chirho, sel_id_chirho)) =
                self.class_method_selector_for_name_chirho(name_chirho)
            {
                // The checker proved this occurrence is at a rigid variable of
                // the enclosing signature: the binding's own dictionary
                // parameter for the class is the evidence, no instance is.
                // workflow: language-features-chirho/dictionary-evidence-chirho
                if let Some(own_key_chirho) = type_key_override_chirho.filter(|key_chirho| {
                    haskelujah_typing_chirho::infer_chirho::is_own_dictionary_key_chirho(key_chirho)
                }) {
                    if let Some(dict_id_chirho) = Self::own_dict_chirho(
                        &class_name_chirho,
                        own_key_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                    ) {
                        return CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
                        };
                    }
                }
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
                    if self
                        .class_param_count_chirho
                        .get(&class_name_chirho)
                        .copied()
                        .unwrap_or(1)
                        > 1
                    {
                        let prefix_chirho = format!("{type_key_chirho}_");
                        let mut dict_ids_chirho = Vec::new();
                        for ((dict_class_chirho, dict_key_chirho), dict_id_chirho) in
                            local_instance_dicts_chirho
                        {
                            if dict_class_chirho == &class_name_chirho
                                && dict_key_chirho.starts_with(&prefix_chirho)
                                && !dict_ids_chirho.contains(dict_id_chirho)
                            {
                                dict_ids_chirho.push(*dict_id_chirho);
                            }
                        }
                        for ((dict_class_chirho, dict_key_chirho), dict_id_chirho) in
                            &self.instance_dicts_chirho
                        {
                            if dict_class_chirho == &class_name_chirho
                                && dict_key_chirho.starts_with(&prefix_chirho)
                                && !dict_ids_chirho.contains(dict_id_chirho)
                            {
                                dict_ids_chirho.push(*dict_id_chirho);
                            }
                        }
                        if let [dict_id_chirho] = dict_ids_chirho.as_slice() {
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
        let (written_id_chirho, classes_chirho, args_chirho) =
            self.collect_dict_param_app_chirho(expr_chirho)?;
        // The surrounding expression's result type is not the type of this
        // callee's predicate (e.g. Eq a for dedup :: [a] -> [a]). A captured
        // reference must take the same evidence-first path in every context.
        if self
            .reference_evidence_chirho
            .contains_key(&written_id_chirho)
        {
            return Some(self.rewrite_method_refs_with_locals_chirho(
                expr_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_type_keys_chirho,
                local_instance_dicts_chirho,
            ));
        }
        let fn_id_chirho = self.canonical_id_chirho(written_id_chirho);

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

    // See spec-chirho/workflows-chirho/stg-forcing-core-chirho.md: prove nested
    // PAP-looking residues are missing evidence before changing runtime forcing.
    fn rewrite_expr_with_type_key_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        type_key_chirho: &str,
    ) -> Option<CoreExprChirho> {
        if let Some(rewritten_chirho) = self
            .try_rewrite_typed_dict_param_arg_chirho(
                expr_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_type_keys_chirho,
                local_instance_dicts_chirho,
                type_key_chirho,
            )
            .or_else(|| {
                self.try_rewrite_typed_method_arg_chirho(
                    expr_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )
            })
        {
            return Some(rewritten_chirho);
        }

        match expr_chirho {
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                let rewritten_fun_chirho = self.rewrite_expr_with_type_key_chirho(
                    fun_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                );
                let rewritten_arg_chirho = self.rewrite_expr_with_type_key_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                );
                if rewritten_fun_chirho.is_none() && rewritten_arg_chirho.is_none() {
                    return None;
                }
                let rebuilt_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(rewritten_fun_chirho.unwrap_or_else(|| {
                        self.rewrite_method_refs_with_locals_chirho(
                            fun_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        )
                    })),
                    arg_chirho: Box::new(rewritten_arg_chirho.unwrap_or_else(|| {
                        self.rewrite_method_refs_with_locals_chirho(
                            arg_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        )
                    })),
                };
                Some(
                    self.try_dispatch_semigroup_monoid_body_chirho(
                        &rebuilt_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )
                    .unwrap_or(rebuilt_chirho),
                )
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
            } => self
                .rewrite_expr_with_type_key_chirho(
                    inner_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )
                .map(|rewritten_inner_chirho| CoreExprChirho::TyAppChirho {
                    expr_chirho: Box::new(rewritten_inner_chirho),
                    ty_chirho: ty_chirho.clone(),
                }),
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                let lambda_type_keys_chirho = Self::extend_first_lambda_type_key_chirho(
                    expr_chirho,
                    type_key_chirho,
                    local_type_keys_chirho,
                );
                self.rewrite_expr_with_type_key_chirho(
                    body_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    &lambda_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )
                .map(|rewritten_body_chirho| CoreExprChirho::LamChirho {
                    binder_chirho: binder_chirho.clone(),
                    body_chirho: Box::new(rewritten_body_chirho),
                })
            }
            CoreExprChirho::TyLamChirho {
                ty_var_chirho,
                body_chirho,
            } => self
                .rewrite_expr_with_type_key_chirho(
                    body_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )
                .map(|rewritten_body_chirho| CoreExprChirho::TyLamChirho {
                    ty_var_chirho: ty_var_chirho.clone(),
                    body_chirho: Box::new(rewritten_body_chirho),
                }),
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => {
                let mut changed_chirho = false;
                let rewritten_binds_chirho = binds_chirho
                    .iter()
                    .map(|(binder_chirho, rhs_chirho)| {
                        let rewritten_rhs_chirho = self.rewrite_expr_with_type_key_chirho(
                            rhs_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                            type_key_chirho,
                        );
                        if rewritten_rhs_chirho.is_some() {
                            changed_chirho = true;
                        }
                        (
                            binder_chirho.clone(),
                            rewritten_rhs_chirho.unwrap_or_else(|| {
                                self.rewrite_method_refs_with_locals_chirho(
                                    rhs_chirho,
                                    dict_vars_chirho,
                                    evidence_classes_chirho,
                                    local_type_keys_chirho,
                                    local_instance_dicts_chirho,
                                )
                            }),
                        )
                    })
                    .collect();
                let rewritten_body_chirho = self.rewrite_expr_with_type_key_chirho(
                    body_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                );
                if rewritten_body_chirho.is_some() {
                    changed_chirho = true;
                }
                if !changed_chirho {
                    return None;
                }
                Some(CoreExprChirho::LetChirho {
                    rec_chirho: *rec_chirho,
                    binds_chirho: rewritten_binds_chirho,
                    body_chirho: Box::new(rewritten_body_chirho.unwrap_or_else(|| {
                        self.rewrite_method_refs_with_locals_chirho(
                            body_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        )
                    })),
                })
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                result_ty_chirho,
                alts_chirho,
            } => {
                let rewritten_scrutinee_chirho = self.rewrite_expr_with_type_key_chirho(
                    scrutinee_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                );
                let mut changed_chirho = rewritten_scrutinee_chirho.is_some();
                let rewritten_alts_chirho = alts_chirho
                    .iter()
                    .map(|alt_chirho| {
                        let rewritten_rhs_chirho = self.rewrite_expr_with_type_key_chirho(
                            &alt_chirho.rhs_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                            type_key_chirho,
                        );
                        if rewritten_rhs_chirho.is_some() {
                            changed_chirho = true;
                        }
                        CoreAltChirho {
                            con_chirho: alt_chirho.con_chirho.clone(),
                            binders_chirho: alt_chirho.binders_chirho.clone(),
                            rhs_chirho: rewritten_rhs_chirho.unwrap_or_else(|| {
                                self.rewrite_method_refs_with_locals_chirho(
                                    &alt_chirho.rhs_chirho,
                                    dict_vars_chirho,
                                    evidence_classes_chirho,
                                    local_type_keys_chirho,
                                    local_instance_dicts_chirho,
                                )
                            }),
                        }
                    })
                    .collect();
                if !changed_chirho {
                    return None;
                }
                Some(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(rewritten_scrutinee_chirho.unwrap_or_else(|| {
                        self.rewrite_method_refs_with_locals_chirho(
                            scrutinee_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        )
                    })),
                    bind_chirho: bind_chirho.clone(),
                    result_ty_chirho: result_ty_chirho.clone(),
                    alts_chirho: rewritten_alts_chirho,
                })
            }
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => {
                let mut changed_chirho = false;
                let rewritten_args_chirho = args_chirho
                    .iter()
                    .map(|arg_chirho| {
                        let rewritten_arg_chirho = self.rewrite_expr_with_type_key_chirho(
                            arg_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                            type_key_chirho,
                        );
                        if rewritten_arg_chirho.is_some() {
                            changed_chirho = true;
                        }
                        rewritten_arg_chirho.unwrap_or_else(|| {
                            self.rewrite_method_refs_with_locals_chirho(
                                arg_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_type_keys_chirho,
                                local_instance_dicts_chirho,
                            )
                        })
                    })
                    .collect();
                changed_chirho.then(|| CoreExprChirho::PrimOpChirho {
                    name_chirho: name_chirho.clone(),
                    args_chirho: rewritten_args_chirho,
                })
            }
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } => {
                let mut changed_chirho = false;
                let rewritten_args_chirho = args_chirho
                    .iter()
                    .map(|arg_chirho| {
                        let rewritten_arg_chirho = self.rewrite_expr_with_type_key_chirho(
                            arg_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                            type_key_chirho,
                        );
                        if rewritten_arg_chirho.is_some() {
                            changed_chirho = true;
                        }
                        rewritten_arg_chirho.unwrap_or_else(|| {
                            self.rewrite_method_refs_with_locals_chirho(
                                arg_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_type_keys_chirho,
                                local_instance_dicts_chirho,
                            )
                        })
                    })
                    .collect();
                changed_chirho.then(|| CoreExprChirho::ConAppChirho {
                    con_name_chirho: con_name_chirho.clone(),
                    args_chirho: rewritten_args_chirho,
                })
            }
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => None,
        }
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
        let CoreExprChirho::VarChirho(head_id_chirho) = head_chirho else {
            return None;
        };
        let head_name_chirho = self.names_chirho.get(head_id_chirho);
        if head_name_chirho.is_some_and(|name_chirho| {
            name_chirho.starts_with("$prim_") || name_chirho.starts_with("$sel_")
        }) || head_name_chirho
            .and_then(|name_chirho| self.class_method_selector_for_name_chirho(name_chirho))
            .is_some()
            || self
                .dict_param_bindings_chirho
                .contains_key(&self.canonical_id_chirho(*head_id_chirho))
        {
            return None;
        }

        let type_key_chirho = args_chirho
            .iter()
            .rev()
            .find_map(|arg_chirho| {
                self.infer_strict_dispatch_key_for_rewrite_chirho(
                    arg_chirho,
                    local_type_keys_chirho,
                )
            })
            .or_else(|| {
                args_chirho
                    .iter()
                    .any(|arg_chirho| self.expr_contains_numeric_default_marker_chirho(arg_chirho))
                    .then(|| "Int".to_string())
            })?;
        let payload_type_key_chirho =
            Self::payload_type_key_from_value_key_chirho(&type_key_chirho);

        let mut changed_chirho = false;
        let mut rewritten_args_chirho = Vec::with_capacity(args_chirho.len());
        for arg_chirho in &args_chirho {
            // A reference the checker proved keeps its own evidence: the
            // variable arm dispatches it; a key guessed from its siblings
            // must not.
            // workflow: language-features-chirho/dictionary-evidence-chirho
            if let CoreExprChirho::VarChirho(arg_id_chirho) = arg_chirho {
                if self.reference_evidence_chirho.contains_key(arg_id_chirho) {
                    rewritten_args_chirho.push(self.rewrite_method_refs_with_locals_chirho(
                        arg_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    ));
                    changed_chirho = true;
                    continue;
                }
            }
            let rewritten_typed_arg_chirho = payload_type_key_chirho
                .as_deref()
                .and_then(|payload_key_chirho| {
                    self.rewrite_expr_with_type_key_chirho(
                        arg_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                        payload_key_chirho,
                    )
                })
                .or_else(|| {
                    self.rewrite_expr_with_type_key_chirho(
                        arg_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                        &type_key_chirho,
                    )
                });
            if let Some(rewritten_chirho) = rewritten_typed_arg_chirho {
                changed_chirho = true;
                rewritten_args_chirho.push(rewritten_chirho);
            } else if Self::first_value_lambda_binder_id_chirho(arg_chirho).is_some() {
                let payload_type_key_chirho = payload_type_key_chirho
                    .clone()
                    .unwrap_or_else(|| type_key_chirho.clone());
                let lambda_type_keys_chirho = Self::extend_first_lambda_type_key_chirho(
                    arg_chirho,
                    &payload_type_key_chirho,
                    local_type_keys_chirho,
                );
                changed_chirho = true;
                rewritten_args_chirho.push(self.rewrite_method_refs_with_locals_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    &lambda_type_keys_chirho,
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
                | "MonadFix"
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

    fn first_value_lambda_binder_id_chirho(expr_chirho: &CoreExprChirho) -> Option<CoreIdChirho> {
        let mut current_chirho = expr_chirho;
        while let CoreExprChirho::TyLamChirho { body_chirho, .. } = current_chirho {
            current_chirho = body_chirho;
        }
        match current_chirho {
            CoreExprChirho::LamChirho { binder_chirho, .. } => Some(binder_chirho.id_chirho),
            _ => None,
        }
    }

    fn expr_has_list_case_on_var_chirho(
        expr_chirho: &CoreExprChirho,
        var_id_chirho: CoreIdChirho,
    ) -> bool {
        match expr_chirho {
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                matches!(scrutinee_chirho.as_ref(), CoreExprChirho::VarChirho(id_chirho) if *id_chirho == var_id_chirho)
                    && alts_chirho.iter().any(|alt_chirho| {
                        matches!(
                            &alt_chirho.con_chirho,
                            AltConChirho::DataConChirho(con_name_chirho) if con_name_chirho == ":"
                        )
                    })
                    || Self::expr_has_list_case_on_var_chirho(scrutinee_chirho, var_id_chirho)
                    || alts_chirho.iter().any(|alt_chirho| {
                        Self::expr_has_list_case_on_var_chirho(
                            &alt_chirho.rhs_chirho,
                            var_id_chirho,
                        )
                    })
            }
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                Self::expr_has_list_case_on_var_chirho(fun_chirho, var_id_chirho)
                    || Self::expr_has_list_case_on_var_chirho(arg_chirho, var_id_chirho)
            }
            CoreExprChirho::LamChirho { body_chirho, .. }
            | CoreExprChirho::TyLamChirho { body_chirho, .. }
            | CoreExprChirho::TyAppChirho {
                expr_chirho: body_chirho,
                ..
            } => Self::expr_has_list_case_on_var_chirho(body_chirho, var_id_chirho),
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                binds_chirho.iter().any(|(_, rhs_chirho)| {
                    Self::expr_has_list_case_on_var_chirho(rhs_chirho, var_id_chirho)
                }) || Self::expr_has_list_case_on_var_chirho(body_chirho, var_id_chirho)
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. }
            | CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
                args_chirho.iter().any(|arg_chirho| {
                    Self::expr_has_list_case_on_var_chirho(arg_chirho, var_id_chirho)
                })
            }
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => false,
        }
    }

    fn first_value_lambda_param_expects_list_chirho(expr_chirho: &CoreExprChirho) -> bool {
        let mut current_chirho = expr_chirho;
        while let CoreExprChirho::TyLamChirho { body_chirho, .. } = current_chirho {
            current_chirho = body_chirho;
        }
        if let CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } = current_chirho
        {
            return Self::expr_has_list_case_on_var_chirho(body_chirho, binder_chirho.id_chirho);
        }
        false
    }

    fn record_local_call_arg_type_key_chirho(
        inferred_chirho: &mut HashMap<CoreIdChirho, String>,
        conflicts_chirho: &mut HashSet<CoreIdChirho>,
        param_id_chirho: CoreIdChirho,
        type_key_chirho: String,
    ) {
        if conflicts_chirho.contains(&param_id_chirho) {
            return;
        }
        if let Some(existing_chirho) = inferred_chirho.get(&param_id_chirho) {
            if existing_chirho != &type_key_chirho {
                inferred_chirho.remove(&param_id_chirho);
                conflicts_chirho.insert(param_id_chirho);
            }
        } else {
            inferred_chirho.insert(param_id_chirho, type_key_chirho);
        }
    }

    fn collect_local_call_arg_type_keys_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        local_function_params_chirho: &HashMap<CoreIdChirho, (CoreIdChirho, bool)>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        inferred_chirho: &mut HashMap<CoreIdChirho, String>,
        conflicts_chirho: &mut HashSet<CoreIdChirho>,
    ) {
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
        if let CoreExprChirho::VarChirho(head_id_chirho) = head_chirho {
            // A call through a reference occurrence is a call to the binding.
            // workflow: language-features-chirho/dictionary-evidence-chirho
            if let Some((param_id_chirho, param_expects_list_chirho)) =
                local_function_params_chirho.get(&self.canonical_id_chirho(*head_id_chirho))
            {
                if let Some(first_arg_chirho) = args_chirho.last() {
                    if let Some(type_key_chirho) = self
                        .infer_strict_dispatch_key_for_rewrite_chirho(
                            first_arg_chirho,
                            local_type_keys_chirho,
                        )
                        .or_else(|| {
                            if self.expr_contains_numeric_default_marker_chirho(first_arg_chirho) {
                                Some("Int".to_string())
                            } else {
                                None
                            }
                        })
                    {
                        let type_key_chirho =
                            if *param_expects_list_chirho && !type_key_chirho.starts_with('[') {
                                format!("[{type_key_chirho}]")
                            } else {
                                type_key_chirho
                            };
                        Self::record_local_call_arg_type_key_chirho(
                            inferred_chirho,
                            conflicts_chirho,
                            *param_id_chirho,
                            type_key_chirho,
                        );
                    }
                }
            }
        }

        match expr_chirho {
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                self.collect_local_call_arg_type_keys_chirho(
                    fun_chirho,
                    local_function_params_chirho,
                    local_type_keys_chirho,
                    inferred_chirho,
                    conflicts_chirho,
                );
                self.collect_local_call_arg_type_keys_chirho(
                    arg_chirho,
                    local_function_params_chirho,
                    local_type_keys_chirho,
                    inferred_chirho,
                    conflicts_chirho,
                );
            }
            CoreExprChirho::LamChirho { body_chirho, .. }
            | CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                self.collect_local_call_arg_type_keys_chirho(
                    body_chirho,
                    local_function_params_chirho,
                    local_type_keys_chirho,
                    inferred_chirho,
                    conflicts_chirho,
                );
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (_, rhs_chirho) in binds_chirho {
                    self.collect_local_call_arg_type_keys_chirho(
                        rhs_chirho,
                        local_function_params_chirho,
                        local_type_keys_chirho,
                        inferred_chirho,
                        conflicts_chirho,
                    );
                }
                self.collect_local_call_arg_type_keys_chirho(
                    body_chirho,
                    local_function_params_chirho,
                    local_type_keys_chirho,
                    inferred_chirho,
                    conflicts_chirho,
                );
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                self.collect_local_call_arg_type_keys_chirho(
                    scrutinee_chirho,
                    local_function_params_chirho,
                    local_type_keys_chirho,
                    inferred_chirho,
                    conflicts_chirho,
                );
                for alt_chirho in alts_chirho {
                    self.collect_local_call_arg_type_keys_chirho(
                        &alt_chirho.rhs_chirho,
                        local_function_params_chirho,
                        local_type_keys_chirho,
                        inferred_chirho,
                        conflicts_chirho,
                    );
                }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. }
            | CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    self.collect_local_call_arg_type_keys_chirho(
                        arg_chirho,
                        local_function_params_chirho,
                        local_type_keys_chirho,
                        inferred_chirho,
                        conflicts_chirho,
                    );
                }
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.collect_local_call_arg_type_keys_chirho(
                inner_chirho,
                local_function_params_chirho,
                local_type_keys_chirho,
                inferred_chirho,
                conflicts_chirho,
            ),
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => {}
        }
    }

    fn collect_module_call_arg_type_keys_chirho(
        &self,
        module_chirho: &CoreModuleChirho,
    ) -> HashMap<CoreIdChirho, String> {
        let mut local_function_params_chirho: HashMap<CoreIdChirho, (CoreIdChirho, bool)> =
            HashMap::new();
        for binding_chirho in &module_chirho.bindings_chirho {
            if let Some(param_id_chirho) =
                Self::first_value_lambda_binder_id_chirho(&binding_chirho.rhs_chirho)
            {
                local_function_params_chirho.insert(
                    binding_chirho.binder_chirho.id_chirho,
                    (
                        param_id_chirho,
                        Self::first_value_lambda_param_expects_list_chirho(
                            &binding_chirho.rhs_chirho,
                        ),
                    ),
                );
            }
        }

        if local_function_params_chirho.is_empty() {
            return HashMap::new();
        }

        let mut inferred_param_keys_chirho: HashMap<CoreIdChirho, String> = HashMap::new();
        let mut conflicting_param_keys_chirho: HashSet<CoreIdChirho> = HashSet::new();
        let empty_type_keys_chirho: HashMap<CoreIdChirho, String> = HashMap::new();
        for binding_chirho in &module_chirho.bindings_chirho {
            self.collect_local_call_arg_type_keys_chirho(
                &binding_chirho.rhs_chirho,
                &local_function_params_chirho,
                &empty_type_keys_chirho,
                &mut inferred_param_keys_chirho,
                &mut conflicting_param_keys_chirho,
            );
        }

        inferred_param_keys_chirho
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
            // Both names are served by Applicative.pure; Monad supplies its
            // Applicative superclass. An own dictionary is stronger evidence
            // than an enclosing expression's monad guess, including for return,
            // which need not itself be a class method.
            if let Some((class_chirho, selector_chirho)) =
                self.class_method_selector_for_name_chirho("pure")
                && let Some(dictionary_chirho) = Self::fallback_dict_for_class_chirho(
                    &class_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                )
            {
                let mut result_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(selector_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(dictionary_chirho)),
                };
                for argument_chirho in &args_chirho {
                    result_chirho = CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(result_chirho),
                        arg_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                            argument_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        )),
                    };
                }
                return Some(result_chirho);
            }
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

        if head_name_chirho == "fail" {
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
                "$prim_MonadFail_fail_{context_key_chirho}"
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
                "mplus" => ("$prim_MonadPlus_mplus_", &[1usize, 0usize]),
                _ => return None,
            };
        let dispatch_arg_chirho = dispatch_indices_chirho
            .iter()
            .find_map(|idx_chirho| args_chirho.get(*idx_chirho))?;
        // >>= / >> / mplus use the conservative key: a wrong positive key sends an IO
        // chain into another monad's body (worse than no dispatch). fmap and
        // <|> keep permissive value-shape inference because their dispatch
        // operands are plain values, not effectful IO chains.
        let value_key_opt_chirho = if matches!(head_name_chirho.as_str(), ">>=" | ">>" | "mplus") {
            self.infer_monad_dispatch_key_chirho(dispatch_arg_chirho, local_type_keys_chirho)
        } else {
            dispatch_indices_chirho.iter().find_map(|idx_chirho| {
                args_chirho.get(*idx_chirho).and_then(|arg_chirho| {
                    self.infer_type_key_for_rewrite_chirho(arg_chirho, local_type_keys_chirho)
                })
            })
        };
        let Some(value_key_chirho) = value_key_opt_chirho else {
            if matches!(head_name_chirho.as_str(), ">>=" | ">>" | "mplus") {
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
            if matches!(head_name_chirho.as_str(), ">>=" | ">>" | "mplus") {
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
            let rewritten_arg_chirho = if matches!(head_name_chirho.as_str(), "<|>" | "mplus") {
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

    fn try_dispatch_semigroup_monoid_body_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let (method_id_chirho, args_chirho, type_key_hint_chirho) =
            self.collect_method_app_chirho(expr_chirho)?;
        let method_name_chirho = self.names_chirho.get(&method_id_chirho)?;
        if self
            .local_shadow_ids_chirho
            .borrow()
            .contains(&method_id_chirho)
        {
            return None;
        }
        let short_name_chirho = method_name_chirho
            .rsplit('.')
            .next()
            .unwrap_or(method_name_chirho);
        let method_key_chirho = short_name_chirho
            .strip_prefix('(')
            .and_then(|name_chirho| name_chirho.strip_suffix(')'))
            .unwrap_or(short_name_chirho);

        let (prefix_chirho, dispatch_indices_chirho): (&str, &[usize]) = match method_key_chirho {
            "<>" => ("$prim_Semigroup_<>_", &[0usize, 1usize]),
            // Monoid.mappend has the same operational body as Semigroup.(<>)
            // for every generated ground row, including newtype-erased wrappers.
            "mappend" => ("$prim_Semigroup_<>_", &[0usize, 1usize]),
            "mconcat" => ("$prim_Monoid_mconcat_", &[0usize]),
            _ => return None,
        };

        let mut key_chirho = type_key_hint_chirho.or_else(|| {
            dispatch_indices_chirho.iter().find_map(|idx_chirho| {
                args_chirho.get(*idx_chirho).and_then(|arg_chirho| {
                    self.infer_type_key_for_rewrite_chirho(arg_chirho, local_type_keys_chirho)
                })
            })
        })?;

        if method_key_chirho == "mconcat"
            && key_chirho.starts_with('[')
            && key_chirho.ends_with(']')
            && key_chirho.len() > 2
        {
            key_chirho = key_chirho[1..key_chirho.len() - 1].to_string();
        }

        let inst_id_chirho =
            self.lookup_dispatch_body_name_id_chirho(&format!("{prefix_chirho}{key_chirho}"))?;
        let mut result_chirho = CoreExprChirho::VarChirho(inst_id_chirho);
        for arg_chirho in &args_chirho {
            let rewritten_arg_chirho = self
                .try_rewrite_typed_method_arg_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    &key_chirho,
                )
                .unwrap_or_else(|| {
                    self.rewrite_method_refs_with_locals_chirho(
                        arg_chirho,
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

    fn try_rewrite_annotated_method_app_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        ty_chirho: &TyChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let type_key_chirho = Self::raw_type_head_key_chirho(ty_chirho, false)?;
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
        args_chirho.reverse();
        let CoreExprChirho::VarChirho(head_id_chirho) = head_chirho else {
            return None;
        };
        let method_name_chirho = self.names_chirho.get(head_id_chirho)?;
        let short_method_name_chirho = method_name_chirho
            .rsplit('.')
            .next()
            .unwrap_or(method_name_chirho);
        if short_method_name_chirho != "fail" {
            return None;
        }
        self.class_method_selector_for_name_chirho(method_name_chirho)?;

        let mut result_chirho = self.try_rewrite_method_var_chirho(
            *head_id_chirho,
            dict_vars_chirho,
            evidence_classes_chirho,
            local_instance_dicts_chirho,
            Some(&type_key_chirho),
        );
        if matches!(result_chirho, CoreExprChirho::VarChirho(id_chirho) if id_chirho == *head_id_chirho)
        {
            return None;
        }
        for arg_chirho in args_chirho {
            let rewritten_arg_chirho = self
                .try_rewrite_typed_method_arg_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    &type_key_chirho,
                )
                .unwrap_or_else(|| {
                    self.rewrite_method_refs_with_locals_chirho(
                        arg_chirho,
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
    /// Evidence-threading P1: immutable name → id scan over the ctx name map
    /// (used to resolve `$prim_{class}_{method}_{key}` targets for occurrence
    /// evidence; returns None when no such generated row exists).
    fn find_global_id_by_name_chirho(&self, name_chirho: &str) -> Option<CoreIdChirho> {
        self.names_chirho
            .iter()
            .find(|(_, existing_chirho)| existing_chirho.as_str() == name_chirho)
            .map(|(id_chirho, _)| *id_chirho)
    }

    /// Evidence-threading P1/P2b: if `head_id` is a method OCCURRENCE id, return
    /// the replacement head expression — the evidence-dispatched `$prim` row when
    /// one exists, else the existing keyed selector/instance dispatch driven by
    /// the PROVEN type key, else the canonical shared id (today's path).
    fn occurrence_head_replacement_chirho(
        &self,
        head_id_chirho: CoreIdChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let (method_name_chirho, canonical_id_chirho) =
            self.method_occurrence_canon_chirho.get(&head_id_chirho)?;
        // A constrained reference with evidence keeps its occurrence id: the
        // dictionary-parameter call-site path reads the evidence off it and
        // canonicalizes the head itself.
        // workflow: language-features-chirho/dictionary-evidence-chirho
        if self.reference_evidence_chirho.contains_key(&head_id_chirho) {
            return None;
        }
        if let Some((class_name_chirho, ty_key_chirho)) =
            self.occurrence_evidence_chirho.get(&head_id_chirho)
        {
            // A literal's wrapper takes the selector form: the simplifier
            // reduces `$sel_Num_fromInteger dict lit` to the literal, which
            // every backend already handles; the prim row is an interpreter
            // shape (Cranelift returned garbage for `Box 42` through it).
            // workflow: language-features-chirho/dictionary-evidence-chirho
            let literal_wrapper_chirho =
                matches!(method_name_chirho.as_str(), "fromInteger" | "fromString");
            let prim_name_chirho = format!(
                "$prim_{}_{}_{}",
                class_name_chirho, method_name_chirho, ty_key_chirho
            );
            if !literal_wrapper_chirho {
                if let Some(prim_id_chirho) = self.find_global_id_by_name_chirho(&prim_name_chirho)
                {
                    return Some(CoreExprChirho::VarChirho(prim_id_chirho));
                }
            }
            // Keyed fallback: reuse the existing selector/instance dispatch with
            // the proven key so selector-backed instances (Eq/Num Int, ...) work.
            return Some(self.try_rewrite_method_var_chirho(
                *canonical_id_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
                Some(ty_key_chirho),
            ));
        }
        Some(CoreExprChirho::VarChirho(*canonical_id_chirho))
    }

    /// Evidence-threading extension for constrained Prelude functions:
    /// `print x` carries a solved `Show` predicate even though `print` is not
    /// itself a class method. Consume that proof before generic occurrence
    /// replacement so native code never has to guess a runtime value's type.
    /// Workflow: `spec-chirho/workflows-chirho/print-show-evidence-chirho.md`.
    fn try_rewrite_evidenced_print_chirho(
        &self,
        head_id_chirho: CoreIdChirho,
        arg_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let (reference_name_chirho, _canonical_id_chirho) =
            self.method_occurrence_canon_chirho.get(&head_id_chirho)?;
        if reference_name_chirho
            .rsplit('.')
            .next()
            .unwrap_or(reference_name_chirho)
            != "print"
        {
            return None;
        }
        let (class_name_chirho, ty_key_chirho) =
            self.occurrence_evidence_chirho.get(&head_id_chirho)?;
        if class_name_chirho != "Show" {
            return None;
        }
        let show_prim_name_chirho = format!("$prim_Show_show_{ty_key_chirho}");
        let show_prim_id_chirho = self.find_global_id_by_name_chirho(&show_prim_name_chirho)?;
        let put_str_ln_id_chirho = self.find_global_id_by_name_chirho("putStrLn")?;
        let rewritten_arg_chirho = self.rewrite_method_refs_with_locals_chirho(
            arg_chirho,
            dict_vars_chirho,
            evidence_classes_chirho,
            local_type_keys_chirho,
            local_instance_dicts_chirho,
        );
        let shown_arg_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(show_prim_id_chirho)),
            arg_chirho: Box::new(rewritten_arg_chirho),
        };
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(put_str_ln_id_chirho)),
            arg_chirho: Box::new(shown_arg_chirho),
        })
    }

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
                // Evidence-threading P1 (design-evidence-threading-chirho.md):
                // per-occurrence method ids. Evidence present → dispatch to the
                // body-backed `$prim_{class}_{method}_{key}` row; no evidence →
                // restore the canonical shared id. Either way recurse once so
                // today's logic applies to the replacement (never an occ id).
                if let Some(replacement_chirho) = self.occurrence_head_replacement_chirho(
                    *id_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                ) {
                    return self.rewrite_method_refs_with_locals_chirho(
                        &replacement_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    );
                }
                if let Some(dispatched_chirho) =
                    self.try_dispatch_contextual_return_pure_var_chirho(*id_chirho)
                {
                    return dispatched_chirho;
                }
                // Check if this var references a constrained user binding
                // that needs dict arguments inserted at the call site.
                if let Some(classes_chirho) = self
                    .dict_param_bindings_chirho
                    .get(&self.canonical_id_chirho(*id_chirho))
                {
                    let mut reference_evidence_chirho = self
                        .reference_evidence_chirho
                        .get(id_chirho)
                        .cloned()
                        .unwrap_or_default();
                    let mut result_chirho =
                        CoreExprChirho::VarChirho(self.canonical_id_chirho(*id_chirho));
                    for class_name_chirho in classes_chirho {
                        let evidence_dict_chirho = Self::take_reference_evidence_chirho(
                            &mut reference_evidence_chirho,
                            class_name_chirho,
                        )
                        .and_then(|key_chirho| {
                            self.evidence_dict_for_class_chirho(
                                class_name_chirho,
                                key_chirho.as_deref(),
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_instance_dicts_chirho,
                            )
                        });
                        if let Some(dict_id_chirho) = evidence_dict_chirho.or_else(|| {
                            Self::fallback_dict_for_class_chirho(
                                class_name_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                            )
                        }) {
                            result_chirho = CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(result_chirho),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
                            };
                        }
                    }
                    return result_chirho;
                }
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
            CoreExprChirho::LitChirho(_) => expr_chirho.clone(),
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                // Evidence-threading P1: canonicalize/dispatch method-occurrence
                // heads on PURE App spines before the specialized App handlers
                // (which key on ids/names of the shared canonical form). TyApp-
                // headed spines fall through — their head Var is handled by the
                // Var arm during normal recursion.
                {
                    let mut spine_args_chirho: Vec<&CoreExprChirho> = Vec::new();
                    let mut spine_head_chirho: &CoreExprChirho = expr_chirho;
                    while let CoreExprChirho::AppChirho {
                        fun_chirho: spine_fun_chirho,
                        arg_chirho: spine_arg_chirho,
                    } = spine_head_chirho
                    {
                        spine_args_chirho.push(spine_arg_chirho.as_ref());
                        spine_head_chirho = spine_fun_chirho.as_ref();
                    }
                    if let CoreExprChirho::VarChirho(head_id_chirho) = spine_head_chirho {
                        if spine_args_chirho.len() == 1 {
                            if let Some(rewritten_print_chirho) = self
                                .try_rewrite_evidenced_print_chirho(
                                    *head_id_chirho,
                                    spine_args_chirho[0],
                                    dict_vars_chirho,
                                    evidence_classes_chirho,
                                    local_type_keys_chirho,
                                    local_instance_dicts_chirho,
                                )
                            {
                                return rewritten_print_chirho;
                            }
                        }
                        if let Some(new_head_chirho) = self.occurrence_head_replacement_chirho(
                            *head_id_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_instance_dicts_chirho,
                        ) {
                            let rebuilt_chirho = spine_args_chirho.iter().rev().fold(
                                new_head_chirho,
                                |fun_acc_chirho, spine_arg_chirho| CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(fun_acc_chirho),
                                    arg_chirho: Box::new((*spine_arg_chirho).clone()),
                                },
                            );
                            return self.rewrite_method_refs_with_locals_chirho(
                                &rebuilt_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_type_keys_chirho,
                                local_instance_dicts_chirho,
                            );
                        }
                    }
                }
                if let Some((head_id_chirho, args_chirho)) =
                    Self::collect_app_head_var_chirho(expr_chirho)
                {
                    if let Some(rewritten_chirho) = self
                        .try_rewrite_show_numeric_default_chirho(
                            head_id_chirho,
                            &args_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        )
                        .or_else(|| {
                            self.try_rewrite_modify_ioref_numeric_default_chirho(
                                head_id_chirho,
                                &args_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_type_keys_chirho,
                                local_instance_dicts_chirho,
                            )
                        })
                        .or_else(|| {
                            self.try_rewrite_strict_numeric_args_chirho(
                                head_id_chirho,
                                &args_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_type_keys_chirho,
                                local_instance_dicts_chirho,
                            )
                        })
                        .or_else(|| {
                            self.try_rewrite_prim_show_int_arg_chirho(
                                head_id_chirho,
                                &args_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_type_keys_chirho,
                                local_instance_dicts_chirho,
                            )
                        })
                    {
                        return rewritten_chirho;
                    }
                }

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
                if let Some(dispatched_chirho) = self.try_dispatch_semigroup_monoid_body_chirho(
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
                                            | "Min"
                                            | "Max"
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
                if let Some((written_id_chirho, classes_chirho, args_chirho)) =
                    self.collect_dict_param_app_chirho(expr_chirho)
                {
                    let fn_id_chirho = self.canonical_id_chirho(written_id_chirho);
                    // The checker's evidence for this reference, one record per
                    // predicate, consulted before the argument-shape guess.
                    // workflow: language-features-chirho/dictionary-evidence-chirho
                    let mut reference_evidence_chirho = self
                        .reference_evidence_chirho
                        .get(&written_id_chirho)
                        .cloned()
                        .unwrap_or_default();
                    // Infer the type key from the actual arguments
                    let type_key_chirho = self.infer_dict_param_call_type_key_chirho(
                        &args_chirho,
                        local_type_keys_chirho,
                    );

                    // Build dict args: for each required class, select the
                    // type-appropriate dict if we can infer the type
                    let mut result_chirho = CoreExprChirho::VarChirho(fn_id_chirho);
                    let classes_chirho = classes_chirho.to_vec();
                    for class_name_chirho in &classes_chirho {
                        let evidence_dict_chirho = Self::take_reference_evidence_chirho(
                            &mut reference_evidence_chirho,
                            class_name_chirho,
                        )
                        .and_then(|key_chirho| {
                            self.evidence_dict_for_class_chirho(
                                class_name_chirho,
                                key_chirho.as_deref(),
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_instance_dicts_chirho,
                            )
                        });
                        let dict_id_chirho = evidence_dict_chirho.or_else(|| {
                            if let Some(ref tk_chirho) = type_key_chirho {
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
                            }
                        });
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
                let mut local_function_params_chirho: HashMap<CoreIdChirho, (CoreIdChirho, bool)> =
                    HashMap::new();
                for (b_chirho, r_chirho) in binds_chirho {
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
                    if let Some(type_key_chirho) =
                        self.concrete_binder_type_key_chirho(b_chirho).or_else(|| {
                            self.infer_local_value_binding_type_key_chirho(
                                r_chirho,
                                &let_type_keys_chirho,
                            )
                        })
                    {
                        let_type_keys_chirho.insert(b_chirho.id_chirho, type_key_chirho);
                    }
                }
                for (b_chirho, r_chirho) in binds_chirho {
                    if let Some(param_id_chirho) =
                        Self::first_value_lambda_binder_id_chirho(r_chirho)
                    {
                        local_function_params_chirho.insert(
                            b_chirho.id_chirho,
                            (
                                param_id_chirho,
                                Self::first_value_lambda_param_expects_list_chirho(r_chirho),
                            ),
                        );
                    }
                }
                if !local_function_params_chirho.is_empty() {
                    let mut inferred_param_keys_chirho: HashMap<CoreIdChirho, String> =
                        HashMap::new();
                    let mut conflicting_param_keys_chirho: HashSet<CoreIdChirho> = HashSet::new();
                    self.collect_local_call_arg_type_keys_chirho(
                        body_chirho,
                        &local_function_params_chirho,
                        &let_type_keys_chirho,
                        &mut inferred_param_keys_chirho,
                        &mut conflicting_param_keys_chirho,
                    );
                    for (_, r_chirho) in binds_chirho {
                        self.collect_local_call_arg_type_keys_chirho(
                            r_chirho,
                            &local_function_params_chirho,
                            &let_type_keys_chirho,
                            &mut inferred_param_keys_chirho,
                            &mut conflicting_param_keys_chirho,
                        );
                    }
                    for (param_id_chirho, type_key_chirho) in inferred_param_keys_chirho {
                        let_type_keys_chirho
                            .entry(param_id_chirho)
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
                if let Some(rewritten_chirho) = self.try_rewrite_annotated_method_app_chirho(
                    inner_chirho,
                    ty_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ) {
                    return CoreExprChirho::TyAppChirho {
                        expr_chirho: Box::new(rewritten_chirho),
                        ty_chirho: ty_chirho.clone(),
                    };
                }
                if let CoreExprChirho::VarChirho(inner_id_chirho) = inner_chirho.as_ref() {
                    if let Some(type_key_chirho) = Self::raw_type_head_key_chirho(ty_chirho, false)
                    {
                        if self
                            .names_chirho
                            .get(inner_id_chirho)
                            .and_then(|name_chirho| {
                                self.class_method_selector_for_name_chirho(name_chirho)
                            })
                            .is_some()
                        {
                            return CoreExprChirho::TyAppChirho {
                                expr_chirho: Box::new(self.try_rewrite_method_var_chirho(
                                    *inner_id_chirho,
                                    dict_vars_chirho,
                                    evidence_classes_chirho,
                                    local_instance_dicts_chirho,
                                    Some(&type_key_chirho),
                                )),
                                ty_chirho: ty_chirho.clone(),
                            };
                        }
                    }
                }
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
            } => {
                let show_type_key_chirho = match name_chirho.as_str() {
                    "showInt#" => Some("Int"),
                    _ => None,
                };
                CoreExprChirho::PrimOpChirho {
                    name_chirho: name_chirho.clone(),
                    args_chirho: args_chirho
                        .iter()
                        .map(|a_chirho| {
                            show_type_key_chirho
                                .and_then(|type_key_chirho| {
                                    self.rewrite_expr_with_type_key_chirho(
                                        a_chirho,
                                        dict_vars_chirho,
                                        evidence_classes_chirho,
                                        local_type_keys_chirho,
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
                        })
                        .collect(),
                }
            }
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
        self.add_dict_params_with_local_type_keys_chirho(
            binding_chirho,
            scheme_chirho,
            &HashMap::new(),
        )
    }

    pub fn add_dict_params_with_local_type_keys_chirho(
        &mut self,
        binding_chirho: &CoreBindingChirho,
        scheme_chirho: &SchemeChirho,
        extra_local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
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

        for (pred_index_chirho, pred_chirho) in scheme_chirho.preds_chirho.iter().enumerate() {
            evidence_classes_chirho.insert(pred_chirho.class_name_chirho.clone());
            let resolved_chirho = self.resolved_pred_dictionary_chirho(pred_chirho, scheme_chirho);

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
                // Also by predicate index, which is how the checker's
                // own-parameter evidence names this parameter (two predicates
                // of one class are two parameters).
                // workflow: language-features-chirho/dictionary-evidence-chirho
                dict_vars_chirho.insert(
                    haskelujah_typing_chirho::infer_chirho::own_dictionary_key_chirho(Some(
                        pred_index_chirho,
                    )),
                    dict_binder_chirho.id_chirho,
                );
                dict_binders_chirho.push(dict_binder_chirho);
            }
        }

        // Record which classes this binding abstracts over so that call
        // sites can insert the corresponding dict arguments.
        if !dict_binders_chirho.is_empty() {
            let classes_chirho = self.dict_param_classes_for_scheme_chirho(scheme_chirho);
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
        for (id_chirho, type_key_chirho) in extra_local_type_keys_chirho {
            let should_insert_chirho =
                local_type_keys_chirho
                    .get(id_chirho)
                    .is_none_or(|existing_chirho| {
                        Self::concrete_local_type_key_chirho(existing_chirho.clone()).is_none()
                    });
            if should_insert_chirho {
                local_type_keys_chirho.insert(*id_chirho, type_key_chirho.clone());
            }
        }

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

        // Superclass extraction: compute the transitive superclass closure for
        // each dict binder. Context reduction can leave a binding with only a
        // subclass predicate (for example Integral) while the body still uses a
        // superclass method such as (==).
        let mut super_let_binds_chirho: Vec<(BinderChirho, CoreExprChirho)> = Vec::new();
        {
            let mut available_dicts_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
            let mut pending_classes_chirho = Vec::new();
            for dict_binder_chirho in &dict_binders_chirho {
                if let Some(class_name_chirho) = dict_binder_chirho.name_chirho.strip_prefix("$d") {
                    let class_name_chirho = class_name_chirho.to_string();
                    available_dicts_chirho
                        .insert(class_name_chirho.clone(), dict_binder_chirho.id_chirho);
                    pending_classes_chirho.push(class_name_chirho);
                }
            }

            while let Some(class_name_chirho) = pending_classes_chirho.pop() {
                let Some(sub_dict_id_chirho) =
                    available_dicts_chirho.get(&class_name_chirho).copied()
                else {
                    continue;
                };
                let super_names_chirho = self
                    .layouts_chirho
                    .get(&class_name_chirho)
                    .map(|layout_chirho| {
                        layout_chirho
                            .super_slots_chirho
                            .iter()
                            .map(|(super_name_chirho, _)| super_name_chirho.clone())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                for super_name_chirho in super_names_chirho {
                    if available_dicts_chirho.contains_key(&super_name_chirho) {
                        continue;
                    }
                    let Some(sel_id_chirho) = self
                        .super_selectors_chirho
                        .get(&(class_name_chirho.clone(), super_name_chirho.clone()))
                        .copied()
                    else {
                        continue;
                    };

                    let super_dict_binder_chirho = self.fresh_binder_chirho(
                        &format!("$d{}", super_name_chirho),
                        TyChirho::ConChirho(format!("$Dict_{}", super_name_chirho)),
                    );
                    let extraction_chirho = CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(sub_dict_id_chirho)),
                    };
                    evidence_classes_chirho.insert(super_name_chirho.clone());
                    dict_vars_chirho.insert(
                        super_name_chirho.clone(),
                        super_dict_binder_chirho.id_chirho,
                    );
                    available_dicts_chirho.insert(
                        super_name_chirho.clone(),
                        super_dict_binder_chirho.id_chirho,
                    );
                    pending_classes_chirho.push(super_name_chirho);
                    super_let_binds_chirho.push((super_dict_binder_chirho, extraction_chirho));
                }
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
        let mut used_super_ids_chirho = crate::simplify_chirho::free_vars_chirho(&rhs_chirho);
        for (binder_chirho, extraction_chirho) in super_let_binds_chirho.iter().rev() {
            if !used_super_ids_chirho.contains(&binder_chirho.id_chirho) {
                continue;
            }
            used_super_ids_chirho
                .extend(crate::simplify_chirho::free_vars_chirho(extraction_chirho));
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

        // Proven occurrence evidence can demand a concrete structured row that
        // is not part of the finite bootstrap table (for example
        // `Show (Either Int Bool)`). Generate only portable, body-backed rows.
        self.generate_evidenced_show_bindings_chirho();

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
                let classes_chirho = self.dict_param_classes_for_scheme_chirho(scheme_chirho);
                if !classes_chirho.is_empty() {
                    self.dict_param_bindings_chirho
                        .insert(binding_chirho.binder_chirho.id_chirho, classes_chirho);
                }
            }
        }
        for (id_chirho, name_chirho) in self.names_chirho.clone() {
            let short_name_chirho = name_chirho
                .rsplit('.')
                .next()
                .unwrap_or(name_chirho.as_str());
            let stripped_name_chirho = short_name_chirho
                .strip_prefix('(')
                .and_then(|n_chirho| n_chirho.strip_suffix(')'))
                .unwrap_or(short_name_chirho);
            let is_extra_imported_chirho =
                self.extra_dict_param_names_chirho.contains(&name_chirho)
                    || self
                        .extra_dict_param_names_chirho
                        .contains(short_name_chirho)
                    || self
                        .extra_dict_param_names_chirho
                        .contains(stripped_name_chirho);
            if !is_extra_imported_chirho {
                continue;
            }
            if self.is_class_method_name_chirho(&name_chirho) {
                continue;
            }
            if let Some(scheme_chirho) = type_env_chirho.lookup_chirho(&name_chirho) {
                let classes_chirho = self.dict_param_classes_for_scheme_chirho(scheme_chirho);
                if !classes_chirho.is_empty() {
                    self.dict_param_bindings_chirho
                        .entry(id_chirho)
                        .or_insert(classes_chirho);
                }
            }
        }

        // Transform each binding
        let mut bindings_chirho = Vec::new();
        let module_call_arg_type_keys_chirho =
            self.collect_module_call_arg_type_keys_chirho(module_chirho);

        for binding_chirho in &module_chirho.bindings_chirho {
            let name_chirho = &binding_chirho.binder_chirho.name_chirho;

            // Look up the type scheme for this binding
            if let Some(scheme_chirho) = type_env_chirho.lookup_chirho(name_chirho) {
                let transformed_chirho = self.add_dict_params_with_local_type_keys_chirho(
                    binding_chirho,
                    scheme_chirho,
                    &module_call_arg_type_keys_chirho,
                );
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
    /// Evidence-threading P2b: total safety-net sweep enforcing the guaranteed-
    /// elimination invariant — any occurrence id that survived rewriting (some
    /// specialized paths clone subtrees without the occurrence intercepts) is
    /// restored to its canonical shared id so no occ id ever reaches STG.
    fn canonicalize_surviving_occurrences_chirho(&self, expr_chirho: &mut CoreExprChirho) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                if let Some((_name_chirho, canonical_id_chirho)) =
                    self.method_occurrence_canon_chirho.get(id_chirho)
                {
                    *id_chirho = *canonical_id_chirho;
                }
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                self.canonicalize_surviving_occurrences_chirho(fun_chirho);
                self.canonicalize_surviving_occurrences_chirho(arg_chirho);
            }
            CoreExprChirho::LamChirho { body_chirho, .. }
            | CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                self.canonicalize_surviving_occurrences_chirho(body_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                self.canonicalize_surviving_occurrences_chirho(inner_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (_binder_chirho, rhs_chirho) in binds_chirho.iter_mut() {
                    self.canonicalize_surviving_occurrences_chirho(rhs_chirho);
                }
                self.canonicalize_surviving_occurrences_chirho(body_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                self.canonicalize_surviving_occurrences_chirho(scrutinee_chirho);
                for alt_chirho in alts_chirho.iter_mut() {
                    self.canonicalize_surviving_occurrences_chirho(&mut alt_chirho.rhs_chirho);
                }
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho.iter_mut() {
                    self.canonicalize_surviving_occurrences_chirho(arg_chirho);
                }
            }
        }
    }

    pub fn finish_chirho(self, mut module_chirho: CoreModuleChirho) -> DictPassResultChirho {
        if !self.method_occurrence_canon_chirho.is_empty() {
            for binding_chirho in module_chirho.bindings_chirho.iter_mut() {
                self.canonicalize_surviving_occurrences_chirho(&mut binding_chirho.rhs_chirho);
            }
        }
        DictPassResultChirho {
            module_chirho,
            names_chirho: self.names_chirho,
            layouts_chirho: self.layouts_chirho,
        }
    }
}
