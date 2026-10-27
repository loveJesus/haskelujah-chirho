// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Dictionary lambdas for constrained bindings.

use std::collections::{HashMap, HashSet};

use haskelujah_typing_chirho::ty_chirho::{SchemeChirho, TyChirho};

use super::DictPassCtxChirho;
use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, InlineAnnotationChirho,
};

impl DictPassCtxChirho {
    /// Predicate indices whose dictionaries this binding must receive.
    ///
    /// Result-only `Num` predicates ordinarily keep the Report's defaulting
    /// behavior. A constrained reference in the binding body can instead prove
    /// that the exact predicate parameter is used: the checker names that proof
    /// `$own:<index>`. In that case definition and caller must retain the same
    /// dictionary parameter rather than silently replacing it with `Int`.
    ///
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn binding_dict_param_indices_chirho(
        &self,
        binding_chirho: &CoreBindingChirho,
        scheme_chirho: &SchemeChirho,
    ) -> Vec<usize> {
        let free_ids_chirho = crate::simplify_chirho::free_vars_chirho(&binding_chirho.rhs_chirho);
        scheme_chirho
            .preds_chirho
            .iter()
            .enumerate()
            .filter_map(|(predicate_index_chirho, predicate_chirho)| {
                let own_key_chirho =
                    haskelujah_typing_chirho::infer_chirho::own_dictionary_key_chirho(Some(
                        predicate_index_chirho,
                    ));
                let body_proves_own_chirho = predicate_chirho.class_name_chirho == "Num"
                    && free_ids_chirho.iter().any(|id_chirho| {
                        self.reference_evidence_chirho.get(id_chirho).is_some_and(
                            |evidence_chirho| {
                                evidence_chirho.iter().any(|(class_chirho, key_chirho)| {
                                    class_chirho == &predicate_chirho.class_name_chirho
                                        && key_chirho.as_deref() == Some(own_key_chirho.as_str())
                                })
                            },
                        )
                    });
                (body_proves_own_chirho
                    || self
                        .resolved_pred_dictionary_chirho(predicate_chirho, scheme_chirho)
                        .is_none())
                .then_some(predicate_index_chirho)
            })
            .collect()
    }

    pub(super) fn dict_param_classes_for_binding_chirho(
        &self,
        binding_chirho: &CoreBindingChirho,
        scheme_chirho: &SchemeChirho,
    ) -> Vec<String> {
        self.binding_dict_param_indices_chirho(binding_chirho, scheme_chirho)
            .into_iter()
            .map(|predicate_index_chirho| {
                scheme_chirho.preds_chirho[predicate_index_chirho]
                    .class_name_chirho
                    .clone()
            })
            .collect()
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
        let dict_param_indices_chirho =
            self.binding_dict_param_indices_chirho(binding_chirho, scheme_chirho);

        for (pred_index_chirho, pred_chirho) in scheme_chirho.preds_chirho.iter().enumerate() {
            evidence_classes_chirho.insert(pred_chirho.class_name_chirho.clone());
            let resolved_chirho = (!dict_param_indices_chirho.contains(&pred_index_chirho))
                .then(|| self.resolved_pred_dictionary_chirho(pred_chirho, scheme_chirho))
                .flatten();

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
            let classes_chirho =
                self.dict_param_classes_for_binding_chirho(binding_chirho, scheme_chirho);
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
        let signature_rhs_chirho = signature_monad_key_chirho
            .as_deref()
            .and_then(|key_chirho| {
                self.rewrite_proven_return_pure_head_chirho(&binding_chirho.rhs_chirho, key_chirho)
            });
        let mut rhs_chirho = self.rewrite_method_refs_with_locals_chirho(
            signature_rhs_chirho
                .as_ref()
                .unwrap_or(&binding_chirho.rhs_chirho),
            &dict_vars_chirho,
            &evidence_classes_chirho,
            &local_type_keys_chirho,
            &local_instance_dicts_chirho,
        );

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
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_span_chirho::SpanChirho;
    use haskelujah_typing_chirho::ty_chirho::{SchemePredChirho, TyVarChirho};

    fn result_only_binding_chirho(rhs_id_chirho: u32) -> CoreBindingChirho {
        CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(1),
                name_chirho: "convertChirho".to_string(),
                ty_chirho: TyChirho::int_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(rhs_id_chirho)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        }
    }

    fn result_only_scheme_chirho(result_class_chirho: &str) -> SchemeChirho {
        let source_var_chirho = TyChirho::VarChirho(TyVarChirho(0));
        let result_var_chirho = TyChirho::VarChirho(TyVarChirho(1));
        SchemeChirho {
            vars_chirho: vec![TyVarChirho(0), TyVarChirho(1)],
            preds_chirho: vec![
                SchemePredChirho {
                    class_name_chirho: "Integral".to_string(),
                    ty_chirho: source_var_chirho.clone(),
                    extra_tys_chirho: Vec::new(),
                },
                SchemePredChirho {
                    class_name_chirho: result_class_chirho.to_string(),
                    ty_chirho: result_var_chirho.clone(),
                    extra_tys_chirho: Vec::new(),
                },
            ],
            ty_chirho: TyChirho::fun_chirho(source_var_chirho, result_var_chirho),
        }
    }

    #[test]
    fn only_exact_body_proven_result_num_evidence_defeats_defaulting_chirho() {
        let rhs_id_chirho = CoreIdChirho(20);
        let binding_chirho = result_only_binding_chirho(rhs_id_chirho.0);
        let scheme_chirho = result_only_scheme_chirho("Num");
        let mut pass_chirho = DictPassCtxChirho::new_chirho(100, HashMap::new(), HashMap::new());
        pass_chirho
            .instance_dicts_chirho
            .insert(("Num".to_string(), "Int".to_string()), CoreIdChirho(90));
        pass_chirho.reference_evidence_chirho.insert(
            rhs_id_chirho,
            vec![
                ("Integral".to_string(), Some("$own:0".to_string())),
                ("Num".to_string(), Some("$own:1".to_string())),
            ],
        );
        assert_eq!(
            pass_chirho.binding_dict_param_indices_chirho(&binding_chirho, &scheme_chirho),
            vec![0, 1]
        );

        pass_chirho.reference_evidence_chirho.insert(
            rhs_id_chirho,
            vec![("Integral".to_string(), Some("$own:0".to_string()))],
        );
        assert_eq!(
            pass_chirho.binding_dict_param_indices_chirho(&binding_chirho, &scheme_chirho),
            vec![0]
        );

        pass_chirho
            .instance_dicts_chirho
            .insert(("Enum".to_string(), "Int".to_string()), CoreIdChirho(91));
        pass_chirho.reference_evidence_chirho.insert(
            rhs_id_chirho,
            vec![
                ("Integral".to_string(), Some("$own:0".to_string())),
                ("Enum".to_string(), Some("$own:1".to_string())),
            ],
        );
        assert_eq!(
            pass_chirho.binding_dict_param_indices_chirho(
                &binding_chirho,
                &result_only_scheme_chirho("Enum")
            ),
            vec![0]
        );
    }
}
