// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Transparent aliases retain their hidden kind indices in both binder and body.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::*;
use haskelujah_ast_chirho::decl_chirho::TyVarChirho as AstTyVarChirho;

#[derive(Clone, Debug)]
pub(super) struct TypeSynonymChirho {
    pub(super) parameters_chirho: Vec<String>,
    kind_parameters_chirho: Vec<String>,
    pub(super) body_chirho: TyChirho,
}

impl TypeSynonymChirho {
    pub(super) fn ordinary_chirho(parameters_chirho: Vec<String>, body_chirho: TyChirho) -> Self {
        Self {
            parameters_chirho,
            kind_parameters_chirho: Vec::new(),
            body_chirho,
        }
    }
}

impl InferCtxChirho {
    /// Import/builtin contracts have no local kind elaboration. Their existing
    /// ordinary parameter contract remains explicit instead of guessing indices.
    pub fn register_type_synonym_chirho(
        &mut self,
        name_chirho: String,
        parameters_chirho: Vec<String>,
        body_chirho: TyChirho,
    ) {
        let body_chirho = self.normalize_imported_ty_chirho(&body_chirho);
        self.type_synonyms_chirho.insert(
            name_chirho,
            TypeSynonymChirho::ordinary_chirho(parameters_chirho, body_chirho),
        );
    }

    pub(super) fn register_local_type_synonym_chirho(
        &mut self,
        name_chirho: &str,
        binders_chirho: &[AstTyVarChirho],
        source_chirho: &TypeChirho,
    ) {
        let parameters_chirho: Vec<_> = binders_chirho
            .iter()
            .filter(|binder_chirho| binder_chirho.is_visible_chirho())
            .map(|binder_chirho| binder_chirho.text_chirho().to_owned())
            .collect();
        let kind_parameters_chirho: Vec<_> = self
            .kind_elaboration_chirho
            .as_ref()
            .and_then(|elaboration_chirho| elaboration_chirho.synonym_heads_chirho.get(name_chirho))
            .map(|binders_chirho| {
                binders_chirho
                    .iter()
                    .map(|binder_chirho| binder_chirho.parameter_name_chirho())
                    .collect()
            })
            .unwrap_or_default();
        let mut variables_chirho = HashMap::new();
        let mut abstraction_chirho = SubstChirho::empty_chirho();
        for parameter_chirho in parameters_chirho.iter().chain(&kind_parameters_chirho) {
            let variable_chirho = *variables_chirho
                .entry(parameter_chirho.clone())
                .or_insert_with(|| {
                    let variable_chirho = TyVarChirho(self.next_var_chirho);
                    self.next_var_chirho += 1;
                    variable_chirho
                });
            abstraction_chirho.insert_chirho(
                variable_chirho,
                TyChirho::ForallVarChirho(parameter_chirho.clone()),
            );
        }
        // Use exactly the conversion used at a signature occurrence, then close
        // its declaration-local identities. A static AST converter cannot know
        // which invisible nominal arguments the kind checker solved here.
        let body_chirho = self.ast_type_to_ty_chirho(source_chirho, &mut variables_chirho);
        let body_chirho = abstraction_chirho.apply_ty_chirho(&body_chirho);
        if !body_chirho.free_vars_chirho().is_empty() {
            self.diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(300),
                    "type synonym body contains kind arguments outside its binder contract",
                    source_chirho.span_chirho(),
                ));
            return;
        }
        self.type_synonyms_chirho.insert(
            name_chirho.to_owned(),
            TypeSynonymChirho {
                parameters_chirho,
                kind_parameters_chirho,
                body_chirho: self.normalize_imported_ty_chirho(&body_chirho),
            },
        );
    }

    pub(super) fn lookup_type_synonym_chirho(
        &self,
        name_chirho: &str,
    ) -> Option<&TypeSynonymChirho> {
        self.type_synonyms_chirho.get(name_chirho).or_else(|| {
            name_chirho
                .rsplit_once('.')
                .and_then(|(_, bare_chirho)| self.type_synonyms_chirho.get(bare_chirho))
        })
    }

    pub fn expand_type_synonyms_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        self.expand_syn_chirho(ty_chirho, 0)
    }

    fn expand_syn_chirho(&self, ty_chirho: &TyChirho, depth_chirho: usize) -> TyChirho {
        if depth_chirho > 100 {
            return ty_chirho.clone();
        }
        let mut head_chirho = ty_chirho;
        let mut arguments_chirho = Vec::new();
        loop {
            match head_chirho {
                TyChirho::AppChirho(fun_chirho, argument_chirho) => {
                    arguments_chirho.push((argument_chirho.as_ref(), false));
                    head_chirho = fun_chirho;
                }
                TyChirho::KindAppChirho(fun_chirho, argument_chirho) => {
                    arguments_chirho.push((argument_chirho.as_ref(), true));
                    head_chirho = fun_chirho;
                }
                _ => break,
            }
        }
        arguments_chirho.reverse();
        if let TyChirho::ConChirho(name_chirho) = head_chirho
            && let Some(synonym_chirho) = self.lookup_type_synonym_chirho(name_chirho)
            && arguments_chirho
                .iter()
                .filter(|(_, invisible_chirho)| !invisible_chirho)
                .count()
                >= synonym_chirho.parameters_chirho.len()
            && arguments_chirho
                .iter()
                .filter(|(_, invisible_chirho)| *invisible_chirho)
                .count()
                >= synonym_chirho.kind_parameters_chirho.len()
        {
            // Decide saturation before expanding any arguments. Doing it
            // afterward would traverse an undersaturated argument once here
            // and again below, multiplying work at every nested alias.
            let mut ordinary_chirho = synonym_chirho.parameters_chirho.iter();
            let mut invisible_chirho = synonym_chirho.kind_parameters_chirho.iter();
            let mut substitutions_chirho = HashMap::new();
            let mut remaining_chirho = Vec::new();
            for (argument_chirho, is_kind_chirho) in &arguments_chirho {
                let parameter_chirho = if *is_kind_chirho {
                    invisible_chirho.next()
                } else {
                    ordinary_chirho.next()
                };
                let argument_chirho = self.expand_syn_chirho(argument_chirho, depth_chirho + 1);
                if let Some(parameter_chirho) = parameter_chirho {
                    substitutions_chirho.insert(parameter_chirho.as_str(), argument_chirho);
                } else {
                    remaining_chirho.push((argument_chirho, *is_kind_chirho));
                }
            }
            if ordinary_chirho.next().is_none() && invisible_chirho.next().is_none() {
                let mut expanded_chirho = substitute_named_parameters_chirho(
                    &synonym_chirho.body_chirho,
                    &substitutions_chirho,
                );
                for (argument_chirho, is_kind_chirho) in remaining_chirho {
                    expanded_chirho = if is_kind_chirho {
                        TyChirho::KindAppChirho(
                            Box::new(expanded_chirho),
                            Box::new(argument_chirho),
                        )
                    } else {
                        TyChirho::AppChirho(Box::new(expanded_chirho), Box::new(argument_chirho))
                    };
                }
                return self.expand_syn_chirho(&expanded_chirho, depth_chirho + 1);
            }
        }
        // Rebuild an unapplied/undersaturated spine in one walk. Recursing on
        // each prefix both repeats the scan and can expand the head too early.
        if !arguments_chirho.is_empty() {
            let mut result_chirho = self.expand_syn_chirho(head_chirho, depth_chirho + 1);
            for (argument_chirho, is_kind_chirho) in arguments_chirho {
                let argument_chirho = self.expand_syn_chirho(argument_chirho, depth_chirho + 1);
                result_chirho = if is_kind_chirho {
                    TyChirho::KindAppChirho(Box::new(result_chirho), Box::new(argument_chirho))
                } else {
                    TyChirho::AppChirho(Box::new(result_chirho), Box::new(argument_chirho))
                };
            }
            return result_chirho;
        }
        match ty_chirho {
            TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
                TyChirho::FunChirho(
                    Box::new(self.expand_syn_chirho(argument_chirho, depth_chirho + 1)),
                    Box::new(self.expand_syn_chirho(result_chirho, depth_chirho + 1)),
                    *multiplicity_chirho,
                )
            }
            TyChirho::ListChirho(element_chirho) => TyChirho::ListChirho(Box::new(
                self.expand_syn_chirho(element_chirho, depth_chirho + 1),
            )),
            TyChirho::TupleChirho(elements_chirho) => TyChirho::TupleChirho(
                elements_chirho
                    .iter()
                    .map(|element_chirho| self.expand_syn_chirho(element_chirho, depth_chirho + 1))
                    .collect(),
            ),
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::ForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.expand_syn_chirho(body_chirho, depth_chirho + 1)),
            },
            TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.expand_syn_chirho(body_chirho, depth_chirho + 1)),
            },
            _ => ty_chirho.clone(),
        }
    }
}

/// Substitute simultaneously: a replacement mentioning another parameter is
/// caller-owned and must not be rewritten by a later substitution. Numeric
/// forall binders are distinct from these declaration-local named parameters.
pub(super) fn substitute_named_parameters_chirho(
    ty_chirho: &TyChirho,
    substitutions_chirho: &HashMap<&str, TyChirho>,
) -> TyChirho {
    match ty_chirho {
        TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
            substitutions_chirho
                .get(name_chirho.as_str())
                .cloned()
                .unwrap_or_else(|| ty_chirho.clone())
        }
        TyChirho::AppChirho(fun_chirho, argument_chirho) => TyChirho::AppChirho(
            Box::new(substitute_named_parameters_chirho(
                fun_chirho,
                substitutions_chirho,
            )),
            Box::new(substitute_named_parameters_chirho(
                argument_chirho,
                substitutions_chirho,
            )),
        ),
        TyChirho::KindAppChirho(fun_chirho, argument_chirho) => TyChirho::KindAppChirho(
            Box::new(substitute_named_parameters_chirho(
                fun_chirho,
                substitutions_chirho,
            )),
            Box::new(substitute_named_parameters_chirho(
                argument_chirho,
                substitutions_chirho,
            )),
        ),
        TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
            TyChirho::FunChirho(
                Box::new(substitute_named_parameters_chirho(
                    argument_chirho,
                    substitutions_chirho,
                )),
                Box::new(substitute_named_parameters_chirho(
                    result_chirho,
                    substitutions_chirho,
                )),
                *multiplicity_chirho,
            )
        }
        TyChirho::ListChirho(element_chirho) => TyChirho::ListChirho(Box::new(
            substitute_named_parameters_chirho(element_chirho, substitutions_chirho),
        )),
        TyChirho::TupleChirho(elements_chirho) => TyChirho::TupleChirho(
            elements_chirho
                .iter()
                .map(|element_chirho| {
                    substitute_named_parameters_chirho(element_chirho, substitutions_chirho)
                })
                .collect(),
        ),
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } => TyChirho::ForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(substitute_named_parameters_chirho(
                body_chirho,
                substitutions_chirho,
            )),
        },
        TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => TyChirho::RequiredForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(substitute_named_parameters_chirho(
                body_chirho,
                substitutions_chirho,
            )),
        },
        TyChirho::VarChirho(_) => ty_chirho.clone(),
    }
}
