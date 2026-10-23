// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Keys carried into lambdas and local or module calls, and out of payloads.

use std::collections::{HashMap, HashSet};

use super::DictPassCtxChirho;
use crate::expr_chirho::{AltConChirho, CoreExprChirho, CoreIdChirho, CoreModuleChirho};

impl DictPassCtxChirho {
    pub(super) fn monad_payload_type_key_chirho(
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

    pub(super) fn payload_type_key_from_value_key_chirho(value_key_chirho: &str) -> Option<String> {
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

    pub(super) fn extend_first_lambda_type_key_chirho(
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

    pub(super) fn first_value_lambda_binder_id_chirho(
        expr_chirho: &CoreExprChirho,
    ) -> Option<CoreIdChirho> {
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

    pub(super) fn first_value_lambda_param_expects_list_chirho(
        expr_chirho: &CoreExprChirho,
    ) -> bool {
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

    pub(super) fn collect_local_call_arg_type_keys_chirho(
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

    pub(super) fn collect_module_call_arg_type_keys_chirho(
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
}
