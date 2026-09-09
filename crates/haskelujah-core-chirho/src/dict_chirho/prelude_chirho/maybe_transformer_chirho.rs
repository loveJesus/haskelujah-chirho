// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! MaybeT stores m (Maybe a). Its generated helpers receive the underlying
//! Monad dictionary proved at the call site; neither the action nor its result
//! may be erased into a plain Maybe. The same code serves IO, Maybe and lists.
//! Workflow: language-features-chirho/dictionary-evidence-chirho.md.

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho,
    DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho, TyVarChirho,
};

fn apply_chirho(
    function_chirho: CoreExprChirho,
    argument_chirho: CoreExprChirho,
) -> CoreExprChirho {
    CoreExprChirho::AppChirho {
        fun_chirho: Box::new(function_chirho),
        arg_chirho: Box::new(argument_chirho),
    }
}

fn variable_chirho(binder_chirho: &BinderChirho) -> CoreExprChirho {
    CoreExprChirho::VarChirho(binder_chirho.id_chirho)
}

fn lambdas_chirho(
    parameters_chirho: Vec<BinderChirho>,
    body_chirho: CoreExprChirho,
) -> CoreExprChirho {
    parameters_chirho
        .into_iter()
        .rev()
        .fold(body_chirho, |body_chirho, binder_chirho| {
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho: Box::new(body_chirho),
            }
        })
}

impl DictPassCtxChirho {
    pub(super) fn generate_maybe_transformer_actions_chirho(&mut self) {
        let Some((_, return_selector_chirho)) = self.method_selectors_chirho.get("pure").cloned()
        else {
            return;
        };
        let Some((_, bind_selector_chirho)) = self.method_selectors_chirho.get(">>=").cloned()
        else {
            return;
        };
        let Some(applicative_selector_chirho) = self
            .super_selectors_chirho
            .get(&("Monad".to_string(), "Applicative".to_string()))
            .copied()
        else {
            return;
        };
        let a_chirho = TyChirho::VarChirho(TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(TyVarChirho(9991));
        let m_chirho = TyChirho::VarChirho(TyVarChirho(9992));
        let transformer_ty_chirho = |value_chirho: TyChirho| {
            TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("MaybeT".to_string())),
                    Box::new(m_chirho.clone()),
                )),
                Box::new(value_chirho),
            )
        };
        let dictionary_ty_chirho = TyChirho::ConChirho("$Dict_Monad".to_string());

        let return_id_chirho = self.resolve_or_fresh_id_chirho("returnMaybeT");
        let dictionary_chirho =
            self.fresh_binder_chirho("$dMonad_chirho", dictionary_ty_chirho.clone());
        let value_chirho = self.fresh_binder_chirho("maybe_value_chirho", a_chirho.clone());
        let applicative_chirho = apply_chirho(
            CoreExprChirho::VarChirho(applicative_selector_chirho),
            variable_chirho(&dictionary_chirho),
        );
        let return_chirho = apply_chirho(
            CoreExprChirho::VarChirho(return_selector_chirho),
            applicative_chirho,
        );
        let just_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "Just".to_string(),
            args_chirho: vec![variable_chirho(&value_chirho)],
        };
        let returned_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "MaybeT".to_string(),
            args_chirho: vec![apply_chirho(return_chirho, just_chirho)],
        };
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: return_id_chirho,
                name_chirho: "returnMaybeT".to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![dictionary_ty_chirho.clone(), a_chirho.clone()],
                    transformer_ty_chirho(a_chirho.clone()),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: lambdas_chirho(vec![dictionary_chirho, value_chirho], returned_chirho),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        self.dict_param_bindings_chirho
            .insert(return_id_chirho, vec!["Monad".to_string()]);

        let bind_id_chirho = self.resolve_or_fresh_id_chirho("bindMaybeT");
        let dictionary_chirho =
            self.fresh_binder_chirho("$dMonad_chirho", dictionary_ty_chirho.clone());
        let action_chirho = self.fresh_binder_chirho(
            "maybe_action_chirho",
            transformer_ty_chirho(a_chirho.clone()),
        );
        let continuation_ty_chirho =
            TyChirho::fun_chirho(a_chirho.clone(), transformer_ty_chirho(b_chirho.clone()));
        let continuation_chirho =
            self.fresh_binder_chirho("maybe_continuation_chirho", continuation_ty_chirho.clone());
        let maybe_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(a_chirho.clone()),
        );
        let maybe_chirho = self.fresh_binder_chirho("maybe_result_chirho", maybe_ty_chirho.clone());
        let case_chirho = self.fresh_binder_chirho("maybe_case_chirho", maybe_ty_chirho);
        let value_chirho = self.fresh_binder_chirho("maybe_value_chirho", a_chirho.clone());
        let unwrapped_action_chirho = self.unwrap_maybe_transformer_chirho(
            variable_chirho(&action_chirho),
            m_chirho.clone(),
            a_chirho.clone(),
        );
        let continued_chirho = self.unwrap_maybe_transformer_chirho(
            apply_chirho(
                variable_chirho(&continuation_chirho),
                variable_chirho(&value_chirho),
            ),
            m_chirho.clone(),
            b_chirho.clone(),
        );
        let applicative_chirho = apply_chirho(
            CoreExprChirho::VarChirho(applicative_selector_chirho),
            variable_chirho(&dictionary_chirho),
        );
        let return_chirho = apply_chirho(
            CoreExprChirho::VarChirho(return_selector_chirho),
            applicative_chirho,
        );
        let dispatch_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(variable_chirho(&maybe_chirho)),
            bind_chirho: case_chirho,
            result_ty_chirho: TyChirho::AppChirho(
                Box::new(m_chirho.clone()),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(b_chirho.clone()),
                )),
            ),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: apply_chirho(
                        return_chirho,
                        CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Nothing".to_string(),
                            args_chirho: vec![],
                        },
                    ),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                    binders_chirho: vec![value_chirho],
                    rhs_chirho: continued_chirho,
                },
            ],
        };
        let bind_chirho = apply_chirho(
            CoreExprChirho::VarChirho(bind_selector_chirho),
            variable_chirho(&dictionary_chirho),
        );
        let sequenced_chirho = apply_chirho(
            apply_chirho(bind_chirho, unwrapped_action_chirho),
            lambdas_chirho(vec![maybe_chirho], dispatch_chirho),
        );
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: bind_id_chirho,
                name_chirho: "bindMaybeT".to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        dictionary_ty_chirho,
                        transformer_ty_chirho(a_chirho),
                        continuation_ty_chirho,
                    ],
                    transformer_ty_chirho(b_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: lambdas_chirho(
                vec![dictionary_chirho, action_chirho, continuation_chirho],
                CoreExprChirho::ConAppChirho {
                    con_name_chirho: "MaybeT".to_string(),
                    args_chirho: vec![sequenced_chirho],
                },
            ),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        self.dict_param_bindings_chirho
            .insert(bind_id_chirho, vec!["Monad".to_string()]);
    }

    fn unwrap_maybe_transformer_chirho(
        &mut self,
        expression_chirho: CoreExprChirho,
        monad_chirho: TyChirho,
        element_chirho: TyChirho,
    ) -> CoreExprChirho {
        let transformer_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("MaybeT".to_string())),
                Box::new(monad_chirho.clone()),
            )),
            Box::new(element_chirho.clone()),
        );
        let result_chirho = TyChirho::AppChirho(
            Box::new(monad_chirho),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(element_chirho),
            )),
        );
        let case_chirho = self.fresh_binder_chirho("transformer_case_chirho", transformer_chirho);
        let inner_chirho =
            self.fresh_binder_chirho("transformer_inner_chirho", result_chirho.clone());
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(expression_chirho),
            bind_chirho: case_chirho,
            result_ty_chirho: result_chirho,
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho("MaybeT".to_string()),
                binders_chirho: vec![inner_chirho.clone()],
                rhs_chirho: variable_chirho(&inner_chirho),
            }],
        }
    }
}
