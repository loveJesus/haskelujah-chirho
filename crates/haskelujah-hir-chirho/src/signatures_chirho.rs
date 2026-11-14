// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rank-1 signature binders and source constraints retain lexical declaration identities.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;
use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;

impl LowerChirho<'_> {
    fn invalid_signature_chirho(
        &self,
        message_chirho: &str,
        span_chirho: SpanChirho,
    ) -> HirErrorChirho {
        HirErrorChirho {
            kind_chirho: HirErrorKindChirho::InvalidChirho,
            message_chirho: message_chirho.into(),
            span_chirho,
        }
    }

    pub(super) fn binding_parts_chirho(
        &mut self,
        signature_chirho: Option<&TypeChirho>,
        matches_chirho: &[haskelujah_ast_chirho::expr_chirho::MatchArmChirho],
        span_chirho: SpanChirho,
        depth_chirho: usize,
    ) -> ResultChirho<(Option<SurfaceSignatureChirho>, ExprChirho)> {
        let Some(signature_chirho) = signature_chirho else {
            return Ok((
                None,
                self.equation_chirho(matches_chirho, span_chirho, depth_chirho)?,
            ));
        };
        self.env_chirho.push_scope_chirho();
        let (signature_chirho, scoped_chirho) = match self.signature_body_chirho(signature_chirho) {
            Ok(result_chirho) => result_chirho,
            Err(error_chirho) => {
                self.env_chirho.pop_scope_chirho();
                return Err(error_chirho);
            }
        };
        if !scoped_chirho {
            self.env_chirho.pop_scope_chirho();
        }
        let body_chirho = self.equation_chirho(matches_chirho, span_chirho, depth_chirho);
        if scoped_chirho {
            self.env_chirho.pop_scope_chirho();
        }
        Ok((Some(signature_chirho), body_chirho?))
    }

    pub(super) fn signature_body_chirho(
        &mut self,
        signature_chirho: &TypeChirho,
    ) -> ResultChirho<(SurfaceSignatureChirho, bool)> {
        let mut body_chirho = signature_chirho;
        let mut explicit_chirho = false;
        let mut bound_names_chirho = HashSet::new();
        let mut binders_chirho = Vec::new();
        let mut context_chirho = Vec::new();
        while let TypeChirho::ParenChirho { inner_chirho, .. } = body_chirho {
            body_chirho = inner_chirho;
        }
        if let TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho: inner_chirho,
            ..
        } = body_chirho
        {
            if !self.explicit_forall_chirho {
                return Err(self.error_chirho(
                    "next explicit forall checking currently requires the GHC2021 profile",
                    signature_chirho.span_chirho(),
                ));
            }
            explicit_chirho = true;
            for variable_chirho in vars_chirho {
                if !bound_names_chirho.insert(variable_chirho.full_name_chirho()) {
                    return Err(self.invalid_signature_chirho(
                        "duplicate signature type binder",
                        variable_chirho.span_chirho(),
                    ));
                }
                binders_chirho.push(self.bind_chirho(
                    variable_chirho.text_chirho(),
                    NamespaceChirho::TypeChirho,
                    variable_chirho.span_chirho(),
                    true,
                    None,
                )?);
            }
            body_chirho = inner_chirho;
        }
        let mut free_names_chirho = Vec::new();
        let mut seen_chirho = HashSet::new();
        loop {
            match body_chirho {
                TypeChirho::ParenChirho { inner_chirho, .. } => body_chirho = inner_chirho,
                TypeChirho::QualChirho {
                    context_chirho: layer_chirho,
                    body_chirho: inner_chirho,
                    ..
                } => {
                    for constraint_chirho in layer_chirho {
                        let ConstraintChirho::ClassChirho {
                            class_chirho,
                            args_chirho,
                            span_chirho,
                        } = constraint_chirho
                        else {
                            return Err(self.error_chirho(
                                "next does not yet check quantified signature constraints",
                                constraint_span_chirho(constraint_chirho),
                            ));
                        };
                        let [argument_chirho] = args_chirho.as_slice() else {
                            return Err(self.error_chirho(
                                "next signatures require a single class argument",
                                *span_chirho,
                            ));
                        };
                        let mut argument_chirho = argument_chirho;
                        while let TypeChirho::ParenChirho { inner_chirho, .. } = argument_chirho {
                            argument_chirho = inner_chirho;
                        }
                        let TypeChirho::VarChirho(argument_chirho) = argument_chirho else {
                            return Err(self.error_chirho("next written signature contexts currently require bare type variables", *span_chirho));
                        };
                        if seen_chirho.insert(argument_chirho.full_name_chirho()) {
                            free_names_chirho.push(argument_chirho.clone());
                        }
                        self.room_chirho(0, *span_chirho)?;
                        self.next_expr_chirho += 1;
                        context_chirho.push((
                            self.lookup_chirho(class_chirho, NamespaceChirho::TypeChirho)?,
                            argument_chirho.clone(),
                        ));
                    }
                    body_chirho = inner_chirho;
                }
                _ => break,
            }
        }
        self.signature_variables_chirho(body_chirho, &mut free_names_chirho, &mut seen_chirho, 0)?;
        for variable_chirho in free_names_chirho {
            if bound_names_chirho.contains(&variable_chirho.full_name_chirho()) {
                continue;
            }
            // A free name already scoped by an enclosing explicit forall is a capture,
            // not a new implicit binder. An inner explicit binder has already shadowed it.
            if self
                .env_chirho
                .lookup_chirho(
                    &variable_chirho.full_name_chirho(),
                    NamespaceChirho::TypeChirho,
                )
                .is_some()
            {
                continue;
            }
            if explicit_chirho {
                return Err(self.invalid_signature_chirho(
                    "an explicitly quantified signature has an unbound type variable",
                    variable_chirho.span_chirho(),
                ));
            }
            bound_names_chirho.insert(variable_chirho.full_name_chirho());
            binders_chirho.push(self.bind_chirho(
                variable_chirho.text_chirho(),
                NamespaceChirho::TypeChirho,
                variable_chirho.span_chirho(),
                true,
                None,
            )?);
        }
        let mut predicates_chirho = Vec::new();
        for (class_chirho, argument_chirho) in context_chirho {
            predicates_chirho.push(SurfacePredicateChirho {
                class_chirho,
                argument_chirho: self
                    .lookup_chirho(&argument_chirho, NamespaceChirho::TypeChirho)?,
            });
        }
        Ok((
            SurfaceSignatureChirho {
                binders_chirho,
                context_chirho: predicates_chirho,
                body_chirho: self.type_chirho(body_chirho, 0)?,
            },
            explicit_chirho,
        ))
    }

    pub(super) fn signature_variables_chirho(
        &mut self,
        ty_chirho: &TypeChirho,
        variables_chirho: &mut Vec<NameChirho>,
        seen_chirho: &mut HashSet<String>,
        depth_chirho: usize,
    ) -> ResultChirho<()> {
        self.room_chirho(depth_chirho, ty_chirho.span_chirho())?;
        self.next_expr_chirho += 1;
        match ty_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                if seen_chirho.insert(name_chirho.full_name_chirho()) {
                    variables_chirho.push(name_chirho.clone());
                }
            }
            TypeChirho::ConChirho(_) => {}
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            }
            | TypeChirho::FunChirho {
                arg_chirho: fun_chirho,
                result_chirho: arg_chirho,
                mult_chirho: None,
                ..
            } => {
                self.signature_variables_chirho(
                    fun_chirho,
                    variables_chirho,
                    seen_chirho,
                    depth_chirho + 1,
                )?;
                self.signature_variables_chirho(
                    arg_chirho,
                    variables_chirho,
                    seen_chirho,
                    depth_chirho + 1,
                )?;
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                for item_chirho in elements_chirho {
                    self.signature_variables_chirho(
                        item_chirho,
                        variables_chirho,
                        seen_chirho,
                        depth_chirho + 1,
                    )?;
                }
            }
            TypeChirho::ParenChirho { inner_chirho, .. }
            | TypeChirho::ListChirho {
                element_chirho: inner_chirho,
                ..
            } => self.signature_variables_chirho(
                inner_chirho,
                variables_chirho,
                seen_chirho,
                depth_chirho + 1,
            )?,
            _ => {
                return Err(self.error_chirho(
                    "next binding signatures are rank-1 with Type-kinded binders",
                    ty_chirho.span_chirho(),
                ));
            }
        }
        Ok(())
    }
}

fn constraint_span_chirho(constraint_chirho: &ConstraintChirho) -> SpanChirho {
    match constraint_chirho {
        ConstraintChirho::ClassChirho { span_chirho, .. }
        | ConstraintChirho::QuantifiedChirho { span_chirho, .. } => *span_chirho,
    }
}
