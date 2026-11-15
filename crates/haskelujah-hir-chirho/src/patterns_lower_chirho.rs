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
            PatChirho::InfixConChirho { .. } => {
                return self.infix_pattern_chirho(pattern_chirho, seen_chirho, depth_chirho);
            }
            PatChirho::TupleChirho {
                elements_chirho, ..
            } if elements_chirho.len() != 1 => {
                self.tuple_arity_chirho(elements_chirho.len(), pattern_chirho.span_chirho())?;
                PatternKindChirho::TupleChirho(
                    elements_chirho
                        .iter()
                        .map(|item_chirho| {
                            self.pattern_chirho(item_chirho, seen_chirho, depth_chirho + 1)
                        })
                        .collect::<ResultChirho<_>>()?,
                )
            }
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

    fn infix_pattern_chirho(
        &mut self,
        source_chirho: &PatChirho,
        seen_chirho: &mut HashSet<String>,
        depth_chirho: usize,
    ) -> ResultChirho<PatternChirho> {
        let span_chirho = source_chirho.span_chirho();
        let mut work_chirho = vec![source_chirho];
        let mut atoms_chirho = Vec::new();
        let mut operators_chirho = Vec::new();
        while let Some(pattern_chirho) = work_chirho.pop() {
            self.charge_names_chirho(1, span_chirho)?;
            if let PatChirho::InfixConChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                ..
            } = pattern_chirho
            {
                operators_chirho.push(op_chirho);
                work_chirho.extend([right_chirho.as_ref(), left_chirho.as_ref()]);
            } else {
                atoms_chirho.push(pattern_chirho);
            }
        }
        if operators_chirho
            .iter()
            .any(|operator_chirho| operator_chirho.full_name_chirho() != ":")
        {
            if operators_chirho.len() > 1 {
                return Err(self.error_chirho(
                    "constructor-operator pattern chain needs resolved fixity checking",
                    span_chirho,
                ));
            }
            let operator_chirho =
                self.lookup_chirho(operators_chirho[0], NamespaceChirho::ValueChirho)?;
            let left_chirho =
                self.pattern_chirho(atoms_chirho[0], seen_chirho, depth_chirho + 1)?;
            let right_chirho =
                self.pattern_chirho(atoms_chirho[1], seen_chirho, depth_chirho + 1)?;
            self.room_chirho(depth_chirho, span_chirho)?;
            let id_chirho = ExprIdChirho(self.next_expr_chirho);
            self.next_expr_chirho += 1;
            return Ok(PatternChirho {
                id_chirho,
                span_chirho,
                kind_chirho: PatternKindChirho::ConstructorChirho(
                    operator_chirho,
                    vec![left_chirho, right_chirho],
                ),
            });
        }
        // Wired cons cannot be shadowed; normalize a pure chain independently of
        // the reused parser. Parentheses remained atomic in the walk above.
        self.room_chirho(
            depth_chirho.saturating_add(operators_chirho.len()),
            span_chirho,
        )?;
        let mut lowered_chirho = atoms_chirho
            .into_iter()
            .map(|atom_chirho| self.pattern_chirho(atom_chirho, seen_chirho, depth_chirho + 1))
            .collect::<ResultChirho<Vec<_>>>()?;
        let mut tail_chirho = lowered_chirho
            .pop()
            .ok_or_else(|| self.error_chirho("empty cons pattern", span_chirho))?;
        for head_chirho in lowered_chirho.into_iter().rev() {
            self.room_chirho(depth_chirho, span_chirho)?;
            let id_chirho = ExprIdChirho(self.next_expr_chirho);
            self.next_expr_chirho += 1;
            tail_chirho = PatternChirho {
                id_chirho,
                span_chirho,
                kind_chirho: PatternKindChirho::ConsChirho(
                    Box::new(head_chirho),
                    Box::new(tail_chirho),
                ),
            };
        }
        Ok(tail_chirho)
    }

    fn row_body_chirho(
        &mut self,
        rhs_chirho: &RhsChirho,
        locals_chirho: &[LocalBindChirho],
        single_equation_chirho: bool,
        span_chirho: SpanChirho,
        depth_chirho: usize,
    ) -> ResultChirho<ExprChirho> {
        let bindings_chirho = self.local_bindings_chirho(locals_chirho, depth_chirho + 1)?;
        let body_chirho = if single_equation_chirho {
            self.single_equation_rhs_chirho(rhs_chirho, depth_chirho + 1)?
        } else {
            self.rhs_chirho(rhs_chirho, depth_chirho + 1)?
        };
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
            .flat_map(|arm_chirho| &arm_chirho.pats_chirho)
            .any(|pattern_chirho| matches!(pattern_chirho, PatChirho::InfixConChirho { .. }))
        {
            return Err(self.error_chirho("unparenthesized constructor operator in a function head needs checked head precedence", span_chirho));
        }
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
                    matches_chirho.len() == 1,
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
                    false,
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
