// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-ordered pattern rows; scopes are resolved once before checking.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;
use haskelujah_ast_chirho::expr_chirho::AltChirho;

impl LowerChirho<'_> {
    fn pattern_binder_chirho(
        &mut self,
        name_chirho: &NameChirho,
        seen_chirho: &mut HashSet<String>,
    ) -> ResultChirho<NameIdChirho> {
        if !seen_chirho.insert(name_chirho.full_name_chirho()) {
            return Err(HirErrorChirho {
                kind_chirho: HirErrorKindChirho::InvalidChirho,
                message_chirho: "a parameter is bound twice in one pattern group".into(),
                span_chirho: name_chirho.span_chirho(),
            });
        }
        self.bind_chirho(
            name_chirho.text_chirho(),
            NamespaceChirho::ValueChirho,
            name_chirho.span_chirho(),
            true,
            None,
        )
    }

    fn pattern_chirho(
        &mut self,
        pattern_chirho: &PatChirho,
        seen_chirho: &mut HashSet<String>,
        depth_chirho: usize,
    ) -> ResultChirho<PatternChirho> {
        let span_chirho = pattern_chirho.span_chirho();
        self.room_chirho(depth_chirho, span_chirho)?;
        let kind_chirho = match pattern_chirho {
            PatChirho::VarChirho(name_chirho) => PatternKindChirho::VariableChirho(
                self.pattern_binder_chirho(name_chirho, seen_chirho)?,
            ),
            PatChirho::WildcardChirho(_) => PatternKindChirho::WildcardChirho,
            PatChirho::ConChirho {
                con_chirho,
                args_chirho,
                ..
            } => PatternKindChirho::ConstructorChirho(
                self.lookup_chirho(con_chirho, NamespaceChirho::ValueChirho)?,
                args_chirho
                    .iter()
                    .map(|arg_chirho| {
                        self.pattern_chirho(arg_chirho, seen_chirho, depth_chirho + 1)
                    })
                    .collect::<ResultChirho<_>>()?,
            ),
            PatChirho::InfixConChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                ..
            } => {
                let left_chirho =
                    self.pattern_chirho(left_chirho, seen_chirho, depth_chirho + 1)?;
                let right_chirho =
                    self.pattern_chirho(right_chirho, seen_chirho, depth_chirho + 1)?;
                if op_chirho.text_chirho() == ":" && op_chirho.full_name_chirho() == ":" {
                    PatternKindChirho::ConsChirho(Box::new(left_chirho), Box::new(right_chirho))
                } else {
                    PatternKindChirho::ConstructorChirho(
                        self.lookup_chirho(op_chirho, NamespaceChirho::ValueChirho)?,
                        vec![left_chirho, right_chirho],
                    )
                }
            }
            PatChirho::TupleChirho {
                elements_chirho, ..
            } if elements_chirho.len() != 1 => PatternKindChirho::TupleChirho(
                elements_chirho
                    .iter()
                    .map(|item_chirho| {
                        self.pattern_chirho(item_chirho, seen_chirho, depth_chirho + 1)
                    })
                    .collect::<ResultChirho<_>>()?,
            ),
            PatChirho::ListChirho {
                elements_chirho, ..
            } => PatternKindChirho::ListChirho(
                elements_chirho
                    .iter()
                    .map(|item_chirho| {
                        self.pattern_chirho(item_chirho, seen_chirho, depth_chirho + 1)
                    })
                    .collect::<ResultChirho<_>>()?,
            ),
            PatChirho::LitChirho(LitChirho::CharChirho(value_chirho, ..)) => {
                PatternKindChirho::CharChirho(*value_chirho)
            }
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho,
                ..
            } => {
                let name_chirho = self.pattern_binder_chirho(name_chirho, seen_chirho)?;
                PatternKindChirho::AsChirho(
                    name_chirho,
                    Box::new(self.pattern_chirho(pattern_chirho, seen_chirho, depth_chirho + 1)?),
                )
            }
            PatChirho::ParenChirho { inner_chirho, .. } => {
                return self.pattern_chirho(inner_chirho, seen_chirho, depth_chirho + 1);
            }
            _ => {
                return Err(
                    self.error_chirho("next does not yet check this pattern form", span_chirho)
                );
            }
        };
        self.room_chirho(depth_chirho, span_chirho)?;
        let id_chirho = ExprIdChirho(self.next_expr_chirho);
        self.next_expr_chirho += 1;
        Ok(PatternChirho {
            id_chirho,
            kind_chirho,
            span_chirho,
        })
    }

    fn row_body_chirho(
        &mut self,
        rhs_chirho: &RhsChirho,
        locals_chirho: &[LocalBindChirho],
        span_chirho: SpanChirho,
        depth_chirho: usize,
    ) -> ResultChirho<ExprChirho> {
        let bindings_chirho = self.local_bindings_chirho(locals_chirho, depth_chirho + 1)?;
        let body_chirho = self.rhs_chirho(rhs_chirho, depth_chirho + 1)?;
        if bindings_chirho.is_empty() {
            Ok(body_chirho)
        } else {
            self.node_chirho(
                ExprKindChirho::LetChirho(bindings_chirho, Box::new(body_chirho)),
                span_chirho,
            )
        }
    }

    pub(super) fn equation_chirho(
        &mut self,
        matches_chirho: &[haskelujah_ast_chirho::expr_chirho::MatchArmChirho],
        span_chirho: SpanChirho,
        depth_chirho: usize,
    ) -> ResultChirho<ExprChirho> {
        self.room_chirho(depth_chirho, span_chirho)?;
        let Some(first_chirho) = matches_chirho.first() else {
            return Err(self.error_chirho("empty function equation group", span_chirho));
        };
        let arity_chirho = first_chirho.pats_chirho.len();
        if matches_chirho
            .iter()
            .any(|arm_chirho| arm_chirho.pats_chirho.len() != arity_chirho)
        {
            return Err(HirErrorChirho {
                kind_chirho: HirErrorKindChirho::InvalidChirho,
                message_chirho: "function equations have different numbers of arguments".into(),
                span_chirho,
            });
        }
        let mut parameters_chirho = Vec::new();
        let mut inputs_chirho = Vec::new();
        for _argument_chirho in 0..arity_chirho {
            let name_chirho = self.bind_chirho(
                &format!("%next:argument:{}Chirho", self.names_chirho.len()),
                NamespaceChirho::ValueChirho,
                span_chirho,
                true,
                None,
            )?;
            parameters_chirho.push(name_chirho);
            inputs_chirho
                .push(self.node_chirho(ExprKindChirho::VarChirho(name_chirho), span_chirho)?);
        }
        let mut arms_chirho = Vec::new();
        for arm_chirho in matches_chirho {
            self.env_chirho.push_scope_chirho();
            let result_chirho = (|| {
                let mut seen_chirho = HashSet::new();
                let patterns_chirho = arm_chirho
                    .pats_chirho
                    .iter()
                    .map(|pat_chirho| {
                        self.pattern_chirho(pat_chirho, &mut seen_chirho, depth_chirho + 1)
                    })
                    .collect::<ResultChirho<_>>()?;
                let body_chirho = self.row_body_chirho(
                    &arm_chirho.rhs_chirho,
                    &arm_chirho.where_binds_chirho,
                    span_chirho,
                    depth_chirho + 1,
                )?;
                Ok(crate::MatchArmChirho {
                    patterns_chirho,
                    body_chirho,
                })
            })();
            self.env_chirho.pop_scope_chirho();
            arms_chirho.push(result_chirho?);
        }
        if arity_chirho == 0 {
            if arms_chirho.len() != 1 {
                return Err(
                    self.error_chirho("multiple equations without value parameters", span_chirho)
                );
            }
            return Ok(arms_chirho.remove(0).body_chirho);
        }
        let body_chirho = self.node_chirho(
            ExprKindChirho::MatchChirho(inputs_chirho, arms_chirho),
            span_chirho,
        )?;
        if parameters_chirho.is_empty() {
            Ok(body_chirho)
        } else {
            self.node_chirho(
                ExprKindChirho::LamChirho(parameters_chirho, Box::new(body_chirho)),
                span_chirho,
            )
        }
    }

    pub(super) fn case_chirho(
        &mut self,
        scrutinee_chirho: &AstExprChirho,
        alternatives_chirho: &[AltChirho],
        depth_chirho: usize,
    ) -> ResultChirho<ExprKindChirho> {
        if alternatives_chirho.is_empty() {
            return Err(self.error_chirho(
                "next does not yet check empty cases",
                scrutinee_chirho.span_chirho(),
            ));
        }
        let scrutinee_chirho = self.expr_chirho(scrutinee_chirho, depth_chirho + 1)?;
        let mut arms_chirho = Vec::new();
        for alternative_chirho in alternatives_chirho {
            self.env_chirho.push_scope_chirho();
            let result_chirho = (|| {
                let pattern_chirho = self.pattern_chirho(
                    &alternative_chirho.pat_chirho,
                    &mut HashSet::new(),
                    depth_chirho + 1,
                )?;
                let body_chirho = self.row_body_chirho(
                    &alternative_chirho.rhs_chirho,
                    &alternative_chirho.where_binds_chirho,
                    alternative_chirho.span_chirho,
                    depth_chirho + 1,
                )?;
                Ok(crate::MatchArmChirho {
                    patterns_chirho: vec![pattern_chirho],
                    body_chirho,
                })
            })();
            self.env_chirho.pop_scope_chirho();
            arms_chirho.push(result_chirho?);
        }
        Ok(ExprKindChirho::MatchChirho(
            vec![scrutinee_chirho],
            arms_chirho,
        ))
    }
}
