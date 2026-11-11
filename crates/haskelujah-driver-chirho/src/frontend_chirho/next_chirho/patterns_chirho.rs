// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ordered matches with shared lazy fallthrough. Constructor keys retain their checked identity.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;
use haskelujah_hir_chirho::{MatchArmChirho, PatternChirho, PatternKindChirho};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho as CoreTypeChirho;

impl ElaboratorChirho<'_> {
    pub(super) fn expansion_depth_chirho(
        &self,
        expression_chirho: &ExprChirho,
    ) -> Result<usize, NextErrorChirho> {
        let mut cost_chirho = 1;
        match &expression_chirho.kind_chirho {
            ExprKindChirho::VarChirho(_) => {
                cost_chirho += self
                    .types_chirho
                    .checked_chirho
                    .wrappers_chirho
                    .get(&expression_chirho.id_chirho)
                    .map_or(0, Vec::len)
            }
            ExprKindChirho::LamChirho(parameters_chirho, _) => {
                cost_chirho += parameters_chirho.len()
            }
            ExprKindChirho::MatchChirho(inputs_chirho, arms_chirho) => {
                cost_chirho += 2 * (inputs_chirho.len() + arms_chirho.len()) + 2;
                let mut work_chirho: Vec<_> = arms_chirho
                    .iter()
                    .flat_map(|arm_chirho| &arm_chirho.patterns_chirho)
                    .collect();
                while let Some(pattern_chirho) = work_chirho.pop() {
                    cost_chirho += 2;
                    if cost_chirho > self.depth_left_chirho {
                        return Err(NextErrorChirho::new_chirho(
                            NextErrorKindChirho::ResourceChirho,
                            "next Core pattern expansion depth ceiling exhausted",
                        ));
                    }
                    match &pattern_chirho.kind_chirho {
                        PatternKindChirho::ConstructorChirho(_, items_chirho)
                        | PatternKindChirho::TupleChirho(items_chirho)
                        | PatternKindChirho::ListChirho(items_chirho) => {
                            work_chirho.extend(items_chirho)
                        }
                        PatternKindChirho::ConsChirho(left_chirho, right_chirho) => {
                            work_chirho.extend([left_chirho.as_ref(), right_chirho.as_ref()])
                        }
                        PatternKindChirho::AsChirho(_, inner_chirho) => {
                            work_chirho.push(inner_chirho)
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(cost_chirho)
    }

    pub(super) fn fresh_binder_chirho(
        &mut self,
        ty_chirho: CoreTypeChirho,
        span_chirho: SpanChirho,
    ) -> Result<BinderChirho, NextErrorChirho> {
        self.types_chirho.reserve_term_chirho()?;
        let id_chirho = CoreIdChirho(self.next_id_chirho);
        self.next_id_chirho = self.next_id_chirho.checked_add(1).ok_or_else(|| {
            NextErrorChirho::new_chirho(NextErrorKindChirho::ResourceChirho, "Core ID overflow")
        })?;
        Ok(BinderChirho {
            id_chirho,
            name_chirho: format!("next_match_{}_chirho", id_chirho.0),
            ty_chirho,
            span_chirho,
        })
    }

    pub(super) fn constructor_chirho(
        &mut self,
        name_chirho: NameIdChirho,
    ) -> Result<CoreBindingChirho, NextErrorChirho> {
        let fields_chirho = &self.types_chirho.checked_chirho.constructors_chirho[&name_chirho];
        if fields_chirho.len() + self.types_chirho.quantified_chirho(name_chirho)?.len() > 256 {
            return Err(NextErrorChirho::new_chirho(
                NextErrorKindChirho::ResourceChirho,
                "next Core constructor depth ceiling exhausted",
            ));
        }
        let mut parameters_chirho = Vec::new();
        for field_chirho in fields_chirho {
            let ty_chirho = self.types_chirho.convert_chirho(*field_chirho)?;
            parameters_chirho.push(self.fresh_binder_chirho(ty_chirho, SpanChirho::DUMMY_CHIRHO)?);
        }
        let mut body_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: self.types_chirho.constructor_key_chirho(name_chirho),
            args_chirho: parameters_chirho
                .iter()
                .map(|parameter_chirho| CoreExprChirho::VarChirho(parameter_chirho.id_chirho))
                .collect(),
        };
        for parameter_chirho in parameters_chirho.into_iter().rev() {
            body_chirho = CoreExprChirho::LamChirho {
                binder_chirho: parameter_chirho,
                body_chirho: Box::new(body_chirho),
            };
        }
        for variable_chirho in self
            .types_chirho
            .quantified_chirho(name_chirho)?
            .into_iter()
            .rev()
        {
            body_chirho = CoreExprChirho::TyLamChirho {
                ty_var_chirho: format!("next_type_{}_chirho", variable_chirho.0),
                body_chirho: Box::new(body_chirho),
            };
        }
        Ok(CoreBindingChirho {
            binder_chirho: self.binder_chirho(name_chirho)?,
            rhs_chirho: body_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        })
    }

    fn pattern_type_chirho(
        &mut self,
        pattern_chirho: &PatternChirho,
    ) -> Result<CoreTypeChirho, NextErrorChirho> {
        self.types_chirho.convert_chirho(
            self.types_chirho.checked_chirho.patterns_chirho[&pattern_chirho.id_chirho],
        )
    }

    fn case_node_chirho(
        &mut self,
        input_chirho: &BinderChirho,
        con_chirho: AltConChirho,
        fields_chirho: Vec<BinderChirho>,
        success_chirho: CoreExprChirho,
        failure_chirho: CoreIdChirho,
        result_chirho: &CoreTypeChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        self.types_chirho.reserve_term_chirho()?;
        let input_type_chirho = self.types_chirho.copy_chirho(&input_chirho.ty_chirho)?;
        let case_binder_chirho =
            self.fresh_binder_chirho(input_type_chirho, input_chirho.span_chirho)?;
        Ok(CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(input_chirho.id_chirho)),
            bind_chirho: case_binder_chirho,
            result_ty_chirho: self.types_chirho.copy_chirho(result_chirho)?,
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho,
                    binders_chirho: fields_chirho,
                    rhs_chirho: success_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: Vec::new(),
                    rhs_chirho: CoreExprChirho::VarChirho(failure_chirho),
                },
            ],
        })
    }

    fn list_pattern_chirho(
        &mut self,
        items_chirho: &[PatternChirho],
        input_chirho: &BinderChirho,
        success_chirho: CoreExprChirho,
        failure_chirho: CoreIdChirho,
        result_chirho: &CoreTypeChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        let Some((head_chirho, rest_chirho)) = items_chirho.split_first() else {
            return self.case_node_chirho(
                input_chirho,
                AltConChirho::DataConChirho("[]".into()),
                Vec::new(),
                success_chirho,
                failure_chirho,
                result_chirho,
            );
        };
        let head_type_chirho = self.pattern_type_chirho(head_chirho)?;
        let head_binder_chirho =
            self.fresh_binder_chirho(head_type_chirho, head_chirho.span_chirho)?;
        let tail_type_chirho = self.types_chirho.copy_chirho(&input_chirho.ty_chirho)?;
        let tail_binder_chirho =
            self.fresh_binder_chirho(tail_type_chirho, input_chirho.span_chirho)?;
        let tail_chirho = self.list_pattern_chirho(
            rest_chirho,
            &tail_binder_chirho,
            success_chirho,
            failure_chirho,
            result_chirho,
        )?;
        let body_chirho = self.pattern_chirho(
            head_chirho,
            &head_binder_chirho,
            tail_chirho,
            failure_chirho,
            result_chirho,
        )?;
        self.case_node_chirho(
            input_chirho,
            AltConChirho::DataConChirho(":".into()),
            vec![head_binder_chirho, tail_binder_chirho],
            body_chirho,
            failure_chirho,
            result_chirho,
        )
    }

    fn alias_chirho(
        &mut self,
        name_chirho: NameIdChirho,
        input_chirho: &BinderChirho,
        body_chirho: CoreExprChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        self.types_chirho.reserve_term_chirho()?;
        Ok(CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(
                self.binder_chirho(name_chirho)?,
                CoreExprChirho::VarChirho(input_chirho.id_chirho),
            )],
            body_chirho: Box::new(body_chirho),
        })
    }

    fn pattern_chirho(
        &mut self,
        pattern_chirho: &PatternChirho,
        input_chirho: &BinderChirho,
        success_chirho: CoreExprChirho,
        failure_chirho: CoreIdChirho,
        result_chirho: &CoreTypeChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        self.types_chirho.reserve_term_chirho()?;
        match &pattern_chirho.kind_chirho {
            PatternKindChirho::WildcardChirho => Ok(success_chirho),
            PatternKindChirho::VariableChirho(name_chirho) => {
                self.alias_chirho(*name_chirho, input_chirho, success_chirho)
            }
            PatternKindChirho::AsChirho(name_chirho, inner_chirho) => {
                let body_chirho = self.pattern_chirho(
                    inner_chirho,
                    input_chirho,
                    success_chirho,
                    failure_chirho,
                    result_chirho,
                )?;
                self.alias_chirho(*name_chirho, input_chirho, body_chirho)
            }
            PatternKindChirho::CharChirho(value_chirho) => self.case_node_chirho(
                input_chirho,
                AltConChirho::LitConChirho(CoreLitChirho::CharChirho(*value_chirho)),
                Vec::new(),
                success_chirho,
                failure_chirho,
                result_chirho,
            ),
            PatternKindChirho::ListChirho(items_chirho) => {
                if items_chirho.len() > 256 {
                    return Err(NextErrorChirho::new_chirho(
                        NextErrorKindChirho::ResourceChirho,
                        "next Core match depth ceiling exhausted",
                    ));
                }
                self.list_pattern_chirho(
                    items_chirho,
                    input_chirho,
                    success_chirho,
                    failure_chirho,
                    result_chirho,
                )
            }
            PatternKindChirho::ConstructorChirho(_, _)
            | PatternKindChirho::TupleChirho(_)
            | PatternKindChirho::ConsChirho(_, _) => {
                let (con_chirho, items_chirho) = match &pattern_chirho.kind_chirho {
                    PatternKindChirho::ConstructorChirho(name_chirho, arguments_chirho) => (
                        self.types_chirho.constructor_key_chirho(*name_chirho),
                        arguments_chirho.iter().collect::<Vec<_>>(),
                    ),
                    PatternKindChirho::TupleChirho(items_chirho) => (
                        format!("({})", ",".repeat(items_chirho.len().saturating_sub(1))),
                        items_chirho.iter().collect(),
                    ),
                    PatternKindChirho::ConsChirho(head_chirho, tail_chirho) => {
                        (":".into(), vec![head_chirho.as_ref(), tail_chirho.as_ref()])
                    }
                    _ => unreachable!(),
                };
                let mut fields_chirho = Vec::new();
                for item_chirho in &items_chirho {
                    let ty_chirho = self.pattern_type_chirho(item_chirho)?;
                    fields_chirho
                        .push(self.fresh_binder_chirho(ty_chirho, item_chirho.span_chirho)?);
                }
                let mut body_chirho = success_chirho;
                for (item_chirho, field_chirho) in
                    items_chirho.into_iter().zip(&fields_chirho).rev()
                {
                    body_chirho = self.pattern_chirho(
                        item_chirho,
                        field_chirho,
                        body_chirho,
                        failure_chirho,
                        result_chirho,
                    )?;
                }
                self.case_node_chirho(
                    input_chirho,
                    AltConChirho::DataConChirho(con_chirho),
                    fields_chirho,
                    body_chirho,
                    failure_chirho,
                    result_chirho,
                )
            }
        }
    }

    pub(super) fn match_chirho(
        &mut self,
        expression_chirho: &ExprChirho,
        inputs_chirho: &[ExprChirho],
        arms_chirho: &[MatchArmChirho],
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        if arms_chirho.len() + inputs_chirho.len() > 256 {
            return Err(NextErrorChirho::new_chirho(
                NextErrorKindChirho::ResourceChirho,
                "next Core match depth ceiling exhausted",
            ));
        }
        let checked_chirho = self.types_chirho.checked_chirho;
        let result_chirho = self
            .types_chirho
            .convert_chirho(checked_chirho.expressions_chirho[&expression_chirho.id_chirho])?;
        let mut bindings_chirho = Vec::new();
        let mut parameters_chirho = Vec::new();
        for input_chirho in inputs_chirho {
            let ty_chirho = self
                .types_chirho
                .convert_chirho(checked_chirho.expressions_chirho[&input_chirho.id_chirho])?;
            let binder_chirho = self.fresh_binder_chirho(ty_chirho, input_chirho.span_chirho)?;
            let retained_chirho = BinderChirho {
                id_chirho: binder_chirho.id_chirho,
                name_chirho: binder_chirho.name_chirho.clone(),
                ty_chirho: self.types_chirho.copy_chirho(&binder_chirho.ty_chirho)?,
                span_chirho: binder_chirho.span_chirho,
            };
            bindings_chirho.push((retained_chirho, self.expression_chirho(input_chirho)?));
            parameters_chirho.push(binder_chirho);
        }
        // The existing interpreter reports a non-exhaustive case if every source row fails.
        // This is reached only on demand; wildcard/variable rows never force the scrutinee.
        let mut body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::ConAppChirho {
                con_name_chirho: "()".into(),
                args_chirho: Vec::new(),
            }),
            bind_chirho: self.fresh_binder_chirho(
                CoreTypeChirho::TupleChirho(Vec::new()),
                expression_chirho.span_chirho,
            )?,
            result_ty_chirho: self.types_chirho.copy_chirho(&result_chirho)?,
            alts_chirho: Vec::new(),
        };
        for arm_chirho in arms_chirho.iter().rev() {
            let fallback_type_chirho = self.types_chirho.copy_chirho(&result_chirho)?;
            let fallback_chirho =
                self.fresh_binder_chirho(fallback_type_chirho, expression_chirho.span_chirho)?;
            let mut row_chirho = self.expression_chirho(&arm_chirho.body_chirho)?;
            for (pattern_chirho, input_chirho) in arm_chirho
                .patterns_chirho
                .iter()
                .zip(&parameters_chirho)
                .rev()
            {
                row_chirho = self.pattern_chirho(
                    pattern_chirho,
                    input_chirho,
                    row_chirho,
                    fallback_chirho.id_chirho,
                    &result_chirho,
                )?;
            }
            body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(fallback_chirho, body_chirho)],
                body_chirho: Box::new(row_chirho),
            };
        }
        Ok(CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: bindings_chirho,
            body_chirho: Box::new(body_chirho),
        })
    }
}
