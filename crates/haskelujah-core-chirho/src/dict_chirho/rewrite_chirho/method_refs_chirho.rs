// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The method-reference walker, `rewrite_method_refs_with_locals_chirho`.

use std::collections::{HashMap, HashSet};

use super::DictPassCtxChirho;
use crate::dict_chirho::prelude_chirho::PRINT_VIA_SHOW_CHIRHO;
use crate::expr_chirho::{CoreAltChirho, CoreExprChirho, CoreIdChirho};

impl DictPassCtxChirho {
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

    /// The `$prim_Show_show_<key>` a `print` occurrence's solved `Show` predicate
    /// names, or None for any other occurrence.
    /// Workflow: `spec-chirho/workflows-chirho/print-show-evidence-chirho.md`.
    fn evidenced_print_show_chirho(&self, head_id_chirho: CoreIdChirho) -> Option<CoreIdChirho> {
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
        // Only a row with an actual body counts (row 556): the name table can hold
        // dangling placeholder ids, and a generated missing-method body is not the row.
        self.lookup_dispatch_body_name_id_chirho(&format!("$prim_Show_show_{ty_key_chirho}"))
    }

    /// An unapplied, evidenced `print`, as in `mapM_ print xs`: the same proof the
    /// applied form consumes, as `$print_via_show_chirho $prim_Show_show_<key>`.
    /// Workflow: `spec-chirho/workflows-chirho/print-show-evidence-chirho.md`.
    pub(super) fn evidenced_print_function_chirho(
        &self,
        id_chirho: CoreIdChirho,
    ) -> Option<CoreExprChirho> {
        let show_prim_id_chirho = self.evidenced_print_show_chirho(id_chirho)?;
        let helper_id_chirho = self.lookup_body_backed_name_id_chirho(PRINT_VIA_SHOW_CHIRHO)?;
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(helper_id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(show_prim_id_chirho)),
        })
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
        let show_prim_id_chirho = self.evidenced_print_show_chirho(head_id_chirho)?;
        let put_str_ln_id_chirho = self.lookup_body_backed_name_id_chirho("putStrLn")?;
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

    pub(super) fn rewrite_method_refs_with_locals_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> CoreExprChirho {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                // A first-class `print` keeps its solved Show proof, never the
                // generic Int-only binding (row 550).
                if let Some(print_function_chirho) = self.evidenced_value_chirho(*id_chirho) {
                    return print_function_chirho;
                }
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
                    // A bare reference has no argument to read a key off.
                    &|| None,
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
                    return self.apply_reference_dicts_chirho(
                        *id_chirho,
                        classes_chirho,
                        None,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_instance_dicts_chirho,
                    );
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
                        // A first-class, evidenced `print` among the arguments, as
                        // in `mapM_ print xs`, takes its proof before any special
                        // path rebuilds the spine and restores its canonical id
                        // (row 550).
                        if spine_args_chirho.iter().any(|spine_arg_chirho| {
                            matches!(spine_arg_chirho, CoreExprChirho::VarChirho(arg_id_chirho)
                                if self.evidenced_value_chirho(*arg_id_chirho).is_some())
                        }) {
                            let rebuilt_chirho = spine_args_chirho.iter().rev().fold(
                                CoreExprChirho::VarChirho(*head_id_chirho),
                                |fun_acc_chirho, spine_arg_chirho| CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(fun_acc_chirho),
                                    arg_chirho: Box::new(match spine_arg_chirho {
                                        CoreExprChirho::VarChirho(arg_id_chirho) => self
                                            .evidenced_value_chirho(*arg_id_chirho)
                                            .unwrap_or_else(|| (*spine_arg_chirho).clone()),
                                        _ => (*spine_arg_chirho).clone(),
                                    }),
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
                        // `(^^)` has two independently proved type variables. Its first value
                        // argument is known Double from the Fractional proof, so rewrite that
                        // argument under this exact key before its blind `negate#` or numeric
                        // literal can default to Int. The exponent keeps its own Integral proof.
                        // workflow: language-features-chirho/dictionary-evidence-chirho
                        if let Some(power_chirho) =
                            self.evidenced_fractional_power_chirho(*head_id_chirho)
                        {
                            return spine_args_chirho.iter().rev().enumerate().fold(
                                power_chirho,
                                |fun_acc_chirho, (index_chirho, spine_arg_chirho)| {
                                    let rewritten_arg_chirho =
                                        if index_chirho == 0 {
                                            self.rewrite_expr_with_type_key_chirho(
                                                spine_arg_chirho,
                                                dict_vars_chirho,
                                                evidence_classes_chirho,
                                                local_type_keys_chirho,
                                                local_instance_dicts_chirho,
                                                "Double",
                                            )
                                            .unwrap_or_else(|| {
                                                self.rewrite_method_refs_with_locals_chirho(
                                                    spine_arg_chirho,
                                                    dict_vars_chirho,
                                                    evidence_classes_chirho,
                                                    local_type_keys_chirho,
                                                    local_instance_dicts_chirho,
                                                )
                                            })
                                        } else {
                                            self.rewrite_method_refs_with_locals_chirho(
                                                spine_arg_chirho,
                                                dict_vars_chirho,
                                                evidence_classes_chirho,
                                                local_type_keys_chirho,
                                                local_instance_dicts_chirho,
                                            )
                                        };
                                    CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(fun_acc_chirho),
                                        arg_chirho: Box::new(rewritten_arg_chirho),
                                    }
                                },
                            );
                        }
                        // An evidenced `sum` or `product` (row 552), or `max`, `min`,
                        // `maximum` or `minimum` (row 565), at the head of an application
                        // takes its proof as an argument does.
                        if let Some(aggregate_chirho) =
                            self.evidenced_applied_head_chirho(*head_id_chirho)
                        {
                            let rebuilt_chirho = spine_args_chirho.iter().rev().fold(
                                aggregate_chirho,
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
                        // The argument-shape key a proven list key may agree
                        // with, read as the method-application path reads it:
                        // the first argument with a key, in source order.
                        let syntactic_key_chirho = || {
                            spine_args_chirho.iter().rev().find_map(|spine_arg_chirho| {
                                self.infer_type_key_for_rewrite_chirho(
                                    spine_arg_chirho,
                                    local_type_keys_chirho,
                                )
                            })
                        };
                        if let Some(new_head_chirho) = self.occurrence_head_replacement_chirho(
                            *head_id_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_instance_dicts_chirho,
                            &syntactic_key_chirho,
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
                    // Infer the type key from the actual arguments
                    let type_key_chirho = self.infer_dict_param_call_type_key_chirho(
                        &args_chirho,
                        local_type_keys_chirho,
                    );
                    // The checker's evidence first, then the argument-shape key.
                    let mut result_chirho = self.apply_reference_dicts_chirho(
                        written_id_chirho,
                        classes_chirho,
                        type_key_chirho.as_deref(),
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_instance_dicts_chirho,
                    );

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
                let annotated_inner_chirho =
                    annotated_monad_key_chirho
                        .as_deref()
                        .and_then(|key_chirho| {
                            self.rewrite_proven_return_pure_head_chirho(inner_chirho, key_chirho)
                        });
                let rewritten_inner_chirho = self.rewrite_method_refs_with_locals_chirho(
                    annotated_inner_chirho.as_ref().unwrap_or(inner_chirho),
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                );
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
                let rewritten_args_chirho: Vec<CoreExprChirho> = args_chirho
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
                    .collect();
                self.compare_route_chirho(
                    name_chirho,
                    args_chirho,
                    &rewritten_args_chirho,
                    local_type_keys_chirho,
                )
                .unwrap_or_else(|| CoreExprChirho::PrimOpChirho {
                    name_chirho: self
                        .primop_at_double_chirho(name_chirho, args_chirho, local_type_keys_chirho)
                        .map_or_else(|| name_chirho.clone(), str::to_string),
                    args_chirho: rewritten_args_chirho,
                })
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
}
