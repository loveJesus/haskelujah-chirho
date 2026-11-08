// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pattern synonyms, moved whole from the desugar root (row 691, D3): collecting a module's
//! synonyms, expanding a synonym's use in a pattern by substituting its arguments, turning a
//! bidirectional synonym's pattern into its builder expression, and expanding the `..`
//! wildcard of a record pattern.

use super::*;

impl DesugarCtxChirho {
    /// Collect pattern synonym declarations into `pat_syns_chirho`.
    pub(super) fn collect_pat_syns_chirho(&mut self, module_chirho: &ModuleChirho) {
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::PatSynDeclChirho {
                name_chirho,
                args_chirho,
                dir_chirho,
                pat_chirho,
                ..
            } = decl_chirho
            {
                let def_chirho = PatSynDefChirho {
                    args_chirho: args_chirho
                        .iter()
                        .map(|a_chirho| a_chirho.text_chirho().to_string())
                        .collect(),
                    pat_chirho: pat_chirho.clone(),
                    dir_chirho: dir_chirho.clone(),
                };
                self.pat_syns_chirho
                    .insert(name_chirho.text_chirho().to_string(), def_chirho);
            }
        }
    }

    /// If `pat_chirho` is a `ConChirho` whose name is a pattern synonym, expand
    /// it by substituting the actual arguments into the synonym's expansion
    /// pattern. Returns `Some(expanded)` if expanded, `None` otherwise.
    pub(super) fn expand_pat_syn_chirho(&self, pat_chirho: &PatChirho) -> Option<PatChirho> {
        if let PatChirho::ConChirho {
            con_chirho,
            args_chirho,
            span_chirho,
            ..
        } = pat_chirho
        {
            let name_chirho = con_chirho.text_chirho();
            if let Some(def_chirho) = self.pat_syns_chirho.get(name_chirho) {
                // Build substitution: formal param name → actual sub-pattern
                let mut subst_chirho: HashMap<String, PatChirho> = HashMap::new();
                for (formal_chirho, actual_chirho) in
                    def_chirho.args_chirho.iter().zip(args_chirho.iter())
                {
                    subst_chirho.insert(formal_chirho.clone(), actual_chirho.clone());
                }
                // For any remaining formals without actuals, bind to wildcard
                for formal_chirho in def_chirho.args_chirho.iter().skip(args_chirho.len()) {
                    subst_chirho.insert(
                        formal_chirho.clone(),
                        PatChirho::WildcardChirho(*span_chirho),
                    );
                }
                return Some(Self::substitute_pat_chirho(
                    &def_chirho.pat_chirho,
                    &subst_chirho,
                ));
            }
        }
        None
    }

    /// Recursively substitute variables in a pattern according to `subst_chirho`.
    /// A `VarChirho(name)` that appears in subst is replaced by the mapped pattern.
    fn substitute_pat_chirho(
        pat_chirho: &PatChirho,
        subst_chirho: &HashMap<String, PatChirho>,
    ) -> PatChirho {
        match pat_chirho {
            PatChirho::VarChirho(name_chirho) => {
                if let Some(replacement_chirho) = subst_chirho.get(name_chirho.text_chirho()) {
                    replacement_chirho.clone()
                } else {
                    pat_chirho.clone()
                }
            }
            PatChirho::ConChirho {
                con_chirho,
                type_args_chirho,
                args_chirho,
                span_chirho,
            } => PatChirho::ConChirho {
                type_args_chirho: type_args_chirho.clone(),
                con_chirho: con_chirho.clone(),
                args_chirho: args_chirho
                    .iter()
                    .map(|a_chirho| Self::substitute_pat_chirho(a_chirho, subst_chirho))
                    .collect(),
                span_chirho: *span_chirho,
            },
            PatChirho::InfixConChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                span_chirho,
            } => PatChirho::InfixConChirho {
                left_chirho: Box::new(Self::substitute_pat_chirho(left_chirho, subst_chirho)),
                op_chirho: op_chirho.clone(),
                right_chirho: Box::new(Self::substitute_pat_chirho(right_chirho, subst_chirho)),
                span_chirho: *span_chirho,
            },
            PatChirho::TupleChirho {
                elements_chirho,
                span_chirho,
            } => PatChirho::TupleChirho {
                elements_chirho: elements_chirho
                    .iter()
                    .map(|e_chirho| Self::substitute_pat_chirho(e_chirho, subst_chirho))
                    .collect(),
                span_chirho: *span_chirho,
            },
            PatChirho::ListChirho {
                elements_chirho,
                span_chirho,
            } => PatChirho::ListChirho {
                elements_chirho: elements_chirho
                    .iter()
                    .map(|e_chirho| Self::substitute_pat_chirho(e_chirho, subst_chirho))
                    .collect(),
                span_chirho: *span_chirho,
            },
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho,
                span_chirho,
            } => PatChirho::AsChirho {
                name_chirho: name_chirho.clone(),
                pattern_chirho: Box::new(Self::substitute_pat_chirho(pattern_chirho, subst_chirho)),
                span_chirho: *span_chirho,
            },
            PatChirho::ParenChirho {
                inner_chirho,
                span_chirho,
            } => PatChirho::ParenChirho {
                inner_chirho: Box::new(Self::substitute_pat_chirho(inner_chirho, subst_chirho)),
                span_chirho: *span_chirho,
            },
            PatChirho::BangChirho {
                inner_chirho,
                span_chirho,
            } => PatChirho::BangChirho {
                inner_chirho: Box::new(Self::substitute_pat_chirho(inner_chirho, subst_chirho)),
                span_chirho: *span_chirho,
            },
            // Wildcard, Lit, Neg — no variables to substitute
            _ => pat_chirho.clone(),
        }
    }

    /// Convert a pattern synonym's expansion pattern into a Core expression
    /// for use in expression position (bidirectional synonym builder).
    /// `subst_chirho` maps formal parameter names to their CoreIdChirho binders.
    pub(super) fn pat_to_builder_expr_chirho(
        &mut self,
        pat_chirho: &PatChirho,
        subst_chirho: &HashMap<String, CoreIdChirho>,
    ) -> CoreExprChirho {
        match pat_chirho {
            PatChirho::SpliceChirho { .. } => {
                self.case_failure_chirho("unsupported unexpanded executable pattern splice")
            }
            PatChirho::VarChirho(name_chirho) => {
                if let Some(id_chirho) = subst_chirho.get(name_chirho.text_chirho()) {
                    CoreExprChirho::VarChirho(*id_chirho)
                } else {
                    let id_chirho = self.resolve_var_chirho(name_chirho.text_chirho());
                    CoreExprChirho::VarChirho(id_chirho)
                }
            }
            PatChirho::ConChirho {
                con_chirho,
                args_chirho,
                ..
            } => {
                let core_args_chirho: Vec<CoreExprChirho> = args_chirho
                    .iter()
                    .map(|a_chirho| self.pat_to_builder_expr_chirho(a_chirho, subst_chirho))
                    .collect();
                CoreExprChirho::ConAppChirho {
                    con_name_chirho: con_chirho.text_chirho().to_string(),
                    args_chirho: core_args_chirho,
                }
            }
            PatChirho::InfixConChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                ..
            } => {
                let left_expr_chirho = self.pat_to_builder_expr_chirho(left_chirho, subst_chirho);
                let right_expr_chirho = self.pat_to_builder_expr_chirho(right_chirho, subst_chirho);
                CoreExprChirho::ConAppChirho {
                    con_name_chirho: op_chirho.text_chirho().to_string(),
                    args_chirho: vec![left_expr_chirho, right_expr_chirho],
                }
            }
            PatChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let tuple_name_chirho = format!("$tuple{}", elements_chirho.len());
                let core_args_chirho: Vec<CoreExprChirho> = elements_chirho
                    .iter()
                    .map(|e_chirho| self.pat_to_builder_expr_chirho(e_chirho, subst_chirho))
                    .collect();
                CoreExprChirho::ConAppChirho {
                    con_name_chirho: tuple_name_chirho,
                    args_chirho: core_args_chirho,
                }
            }
            PatChirho::LitChirho(lit_chirho) => {
                CoreExprChirho::LitChirho(self.desugar_lit_chirho(lit_chirho))
            }
            PatChirho::WildcardChirho(_) => {
                // Wildcard in builder position — produce a unit-like placeholder.
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
            }
            PatChirho::ParenChirho { inner_chirho, .. } => {
                self.pat_to_builder_expr_chirho(inner_chirho, subst_chirho)
            }
            _ => CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
        }
    }

    /// Expand a RecordWildCards pattern: fill in missing fields as variable patterns.
    /// Returns `Some(expanded_pat)` if expansion was needed, `None` if no expansion.
    pub(super) fn expand_record_wildcard_pat_chirho(
        &self,
        pat_chirho: &PatChirho,
    ) -> Option<PatChirho> {
        if let PatChirho::RecordChirho {
            con_chirho,
            fields_chirho,
            has_wildcard_chirho: true,
            span_chirho,
        } = pat_chirho
        {
            let con_name_chirho = con_chirho.text_chirho();
            if let Some(all_fields_chirho) = self.con_field_names_chirho.get(con_name_chirho) {
                let explicit_names_chirho: HashSet<String> = fields_chirho
                    .iter()
                    .map(|f_chirho| f_chirho.name_chirho.text_chirho().to_string())
                    .collect();
                let mut expanded_fields_chirho = fields_chirho.clone();
                for field_name_chirho in all_fields_chirho {
                    if !explicit_names_chirho.contains(field_name_chirho) {
                        // Create a Var pattern binding the field name as a variable
                        let name_chirho = haskelujah_ast_chirho::name_chirho::NameChirho::RawChirho(
                            haskelujah_ast_chirho::name_chirho::RawNameChirho::unqualified_chirho(
                                field_name_chirho.to_string(),
                                *span_chirho,
                            ),
                        );
                        expanded_fields_chirho.push(
                            haskelujah_ast_chirho::pat_chirho::PatFieldChirho {
                                name_chirho: name_chirho.clone(),
                                pattern_chirho: PatChirho::VarChirho(name_chirho),
                                span_chirho: *span_chirho,
                            },
                        );
                    }
                }
                // Reorder to match constructor field order
                let mut ordered_fields_chirho = Vec::with_capacity(all_fields_chirho.len());
                for field_name_chirho in all_fields_chirho {
                    if let Some(f_chirho) = expanded_fields_chirho
                        .iter()
                        .find(|f_chirho| f_chirho.name_chirho.text_chirho() == field_name_chirho)
                    {
                        ordered_fields_chirho.push(f_chirho.clone());
                    }
                }
                return Some(PatChirho::RecordChirho {
                    con_chirho: con_chirho.clone(),
                    fields_chirho: ordered_fields_chirho,
                    has_wildcard_chirho: false,
                    span_chirho: *span_chirho,
                });
            }
        }
        None
    }
}
