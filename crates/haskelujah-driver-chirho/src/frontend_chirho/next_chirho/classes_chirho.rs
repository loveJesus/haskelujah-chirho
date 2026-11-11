// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Elaborate the checker's scope-owned evidence directly, without legacy dictionary lookup.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;
use haskelujah_hir_chirho::InstanceDeclChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_tc_chirho::{EvTermChirho, ScopeIdChirho};

struct ScopeBindingsChirho {
    parameters_chirho: Vec<BinderChirho>,
    definitions_chirho: Vec<(EvVarChirho, BinderChirho)>,
}

fn invalid_evidence_chirho(message_chirho: &str) -> NextErrorChirho {
    NextErrorChirho::new_chirho(NextErrorKindChirho::InvalidChirho, message_chirho)
}

impl ElaboratorChirho<'_> {
    fn dictionary_key_chirho(&self, class_chirho: NameIdChirho) -> String {
        let info_chirho =
            &self.types_chirho.checked_chirho.module_chirho.names_chirho[class_chirho.0 as usize];
        format!(
            "%next:home:{}:dictionary-constructor:{}:{}",
            info_chirho.module_chirho, class_chirho.0, info_chirho.occurrence_chirho
        )
    }

    pub(super) fn evidence_variable_chirho(
        &mut self,
        evidence_chirho: EvVarChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        self.types_chirho.reserve_term_chirho()?;
        self.evidence_scopes_chirho
            .iter()
            .rev()
            .find_map(|scope_chirho| scope_chirho.get(&evidence_chirho))
            .copied()
            .map(CoreExprChirho::VarChirho)
            .ok_or_else(|| invalid_evidence_chirho("checked dictionary has no lexical Core binder"))
    }

    fn enter_evidence_scope_chirho(
        &mut self,
        scope_chirho: ScopeIdChirho,
    ) -> Result<ScopeBindingsChirho, NextErrorChirho> {
        let checked_chirho = self.types_chirho.checked_chirho;
        let solver_chirho = &checked_chirho.solver_chirho;
        let mut map_chirho = HashMap::new();
        let mut parameters_chirho = Vec::new();
        let mut definitions_chirho = Vec::new();
        for given_chirho in solver_chirho.scope_givens_chirho(scope_chirho)? {
            let ty_chirho = self
                .types_chirho
                .convert_chirho(given_chirho.predicate_chirho)?;
            let binder_chirho = self.fresh_binder_chirho(ty_chirho, SpanChirho::DUMMY_CHIRHO)?;
            map_chirho.insert(given_chirho.evidence_chirho, binder_chirho.id_chirho);
            parameters_chirho.push(binder_chirho);
        }
        // Preallocate every owner-local binding before elaborating its A-normal right-hand side.
        for evidence_chirho in solver_chirho.scope_bindings_chirho(scope_chirho)? {
            let ty_chirho = self
                .types_chirho
                .convert_chirho(solver_chirho.evidence_predicate_chirho(*evidence_chirho)?)?;
            let binder_chirho = self.fresh_binder_chirho(ty_chirho, SpanChirho::DUMMY_CHIRHO)?;
            if map_chirho
                .insert(*evidence_chirho, binder_chirho.id_chirho)
                .is_some()
            {
                return Err(invalid_evidence_chirho(
                    "duplicate dictionary binder in checked scope",
                ));
            }
            definitions_chirho.push((*evidence_chirho, binder_chirho));
        }
        self.evidence_scopes_chirho.push(map_chirho);
        Ok(ScopeBindingsChirho {
            parameters_chirho,
            definitions_chirho,
        })
    }

    fn evidence_term_chirho(
        &mut self,
        term_chirho: &EvTermChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        match term_chirho {
            EvTermChirho::VariableChirho(variable_chirho) => {
                self.evidence_variable_chirho(*variable_chirho)
            }
            EvTermChirho::InstanceChirho {
                instance_chirho,
                type_arguments_chirho,
                dictionary_arguments_chirho,
            } => {
                self.types_chirho.reserve_term_chirho()?;
                let mut result_chirho = CoreExprChirho::VarChirho(CoreIdChirho(instance_chirho.0));
                for argument_chirho in type_arguments_chirho {
                    self.types_chirho.reserve_term_chirho()?;
                    result_chirho = CoreExprChirho::TyAppChirho {
                        expr_chirho: Box::new(result_chirho),
                        ty_chirho: self.types_chirho.convert_chirho(*argument_chirho)?,
                    };
                }
                for argument_chirho in dictionary_arguments_chirho {
                    self.types_chirho.reserve_term_chirho()?;
                    result_chirho = CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(result_chirho),
                        arg_chirho: Box::new(self.evidence_variable_chirho(*argument_chirho)?),
                    };
                }
                Ok(result_chirho)
            }
            EvTermChirho::SuperclassChirho { .. } => Err(invalid_evidence_chirho(
                "superclass evidence has no producer in the current next slice",
            )),
        }
    }

    fn evidence_let_chirho(
        &mut self,
        definitions_chirho: Vec<(EvVarChirho, BinderChirho)>,
        body_chirho: CoreExprChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        if definitions_chirho.is_empty() {
            return Ok(body_chirho);
        }
        let checked_chirho = self.types_chirho.checked_chirho;
        let mut binds_chirho = Vec::new();
        for (evidence_chirho, binder_chirho) in definitions_chirho {
            let term_chirho = checked_chirho
                .solver_chirho
                .evidence_term_chirho(evidence_chirho)?
                .ok_or_else(|| invalid_evidence_chirho("unsolved dictionary reached Core"))?;
            binds_chirho.push((binder_chirho, self.evidence_term_chirho(term_chirho)?));
        }
        self.types_chirho.reserve_term_chirho()?;
        Ok(CoreExprChirho::LetChirho {
            rec_chirho: true,
            binds_chirho,
            body_chirho: Box::new(body_chirho),
        })
    }

    pub(super) fn abstract_binding_chirho(
        &mut self,
        binding_chirho: &BindingChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        let checked_chirho = self.types_chirho.checked_chirho;
        let abstraction_chirho = checked_chirho
            .abstractions_chirho
            .get(&binding_chirho.name_chirho)
            .ok_or_else(|| invalid_evidence_chirho("checked binding has no abstraction"))?;
        let cost_chirho = abstraction_chirho.type_binders_chirho.len()
            + abstraction_chirho.dictionary_binders_chirho.len()
            + 1;
        if cost_chirho > self.depth_left_chirho {
            return Err(NextErrorChirho::new_chirho(
                NextErrorKindChirho::ResourceChirho,
                "next dictionary abstraction depth exhausted",
            ));
        }
        let scope_chirho = abstraction_chirho.scope_chirho;
        let givens_chirho = checked_chirho
            .solver_chirho
            .scope_givens_chirho(scope_chirho)?;
        if !givens_chirho
            .iter()
            .map(|given_chirho| given_chirho.evidence_chirho)
            .eq(abstraction_chirho.dictionary_binders_chirho.iter().copied())
        {
            return Err(invalid_evidence_chirho(
                "dictionary abstraction order disagrees with its scope",
            ));
        }
        let frame_chirho = self.enter_evidence_scope_chirho(scope_chirho)?;
        self.depth_left_chirho -= cost_chirho;
        let result_chirho = (|| {
            let body_chirho = self.expression_chirho(&binding_chirho.body_chirho)?;
            let mut body_chirho =
                self.evidence_let_chirho(frame_chirho.definitions_chirho, body_chirho)?;
            for parameter_chirho in frame_chirho.parameters_chirho.into_iter().rev() {
                self.types_chirho.reserve_term_chirho()?;
                body_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: parameter_chirho,
                    body_chirho: Box::new(body_chirho),
                };
            }
            for variable_chirho in abstraction_chirho.type_binders_chirho.iter().rev() {
                self.types_chirho.reserve_term_chirho()?;
                let id_chirho = checked_chirho
                    .solver_chirho
                    .arena_chirho
                    .variable_id_chirho(*variable_chirho)?;
                body_chirho = CoreExprChirho::TyLamChirho {
                    ty_var_chirho: format!("next_type_{}_chirho", id_chirho),
                    body_chirho: Box::new(body_chirho),
                };
            }
            Ok(body_chirho)
        })();
        self.depth_left_chirho += cost_chirho;
        self.evidence_scopes_chirho.pop();
        result_chirho
    }

    pub(super) fn class_selectors_chirho(
        &mut self,
        class_chirho: NameIdChirho,
    ) -> Result<Vec<CoreBindingChirho>, NextErrorChirho> {
        let checked_chirho = self.types_chirho.checked_chirho;
        let class_info_chirho = checked_chirho
            .solver_chirho
            .class_info_chirho(class_chirho)?;
        let mut bindings_chirho = Vec::new();
        for (index_chirho, method_chirho) in class_info_chirho.methods_chirho.iter().enumerate() {
            let dictionary_type_chirho = self
                .types_chirho
                .convert_chirho(method_chirho.scheme_chirho.context_chirho[0])?;
            let case_type_chirho = self.types_chirho.copy_chirho(&dictionary_type_chirho)?;
            let dictionary_chirho =
                self.fresh_binder_chirho(dictionary_type_chirho, SpanChirho::DUMMY_CHIRHO)?;
            let case_binder_chirho =
                self.fresh_binder_chirho(case_type_chirho, SpanChirho::DUMMY_CHIRHO)?;
            let mut fields_chirho = Vec::new();
            for field_chirho in &class_info_chirho.methods_chirho {
                let field_type_chirho = self
                    .types_chirho
                    .convert_chirho(field_chirho.scheme_chirho.body_chirho)?;
                fields_chirho
                    .push(self.fresh_binder_chirho(field_type_chirho, SpanChirho::DUMMY_CHIRHO)?);
            }
            self.types_chirho.reserve_term_chirho()?;
            let mut body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(dictionary_chirho.id_chirho)),
                bind_chirho: case_binder_chirho,
                result_ty_chirho: self
                    .types_chirho
                    .convert_chirho(method_chirho.scheme_chirho.body_chirho)?,
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(
                        self.dictionary_key_chirho(class_chirho),
                    ),
                    rhs_chirho: CoreExprChirho::VarChirho(fields_chirho[index_chirho].id_chirho),
                    binders_chirho: fields_chirho,
                }],
            };
            self.types_chirho.reserve_term_chirho()?;
            body_chirho = CoreExprChirho::LamChirho {
                binder_chirho: dictionary_chirho,
                body_chirho: Box::new(body_chirho),
            };
            for variable_chirho in self
                .types_chirho
                .quantified_chirho(method_chirho.name_chirho)?
                .into_iter()
                .rev()
            {
                self.types_chirho.reserve_term_chirho()?;
                body_chirho = CoreExprChirho::TyLamChirho {
                    ty_var_chirho: format!("next_type_{}_chirho", variable_chirho.0),
                    body_chirho: Box::new(body_chirho),
                };
            }
            bindings_chirho.push(CoreBindingChirho {
                binder_chirho: self.binder_chirho(method_chirho.name_chirho)?,
                rhs_chirho: body_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
        Ok(bindings_chirho)
    }

    pub(super) fn instance_dictionary_chirho(
        &mut self,
        instance_chirho: &InstanceDeclChirho,
    ) -> Result<CoreBindingChirho, NextErrorChirho> {
        let checked_chirho = self.types_chirho.checked_chirho;
        let info_chirho = checked_chirho
            .solver_chirho
            .instance_info_chirho(instance_chirho.name_chirho)?;
        let frame_chirho = self.enter_evidence_scope_chirho(info_chirho.scope_chirho)?;
        let result_chirho = (|| {
            if !frame_chirho.parameters_chirho.is_empty() {
                return Err(invalid_evidence_chirho(
                    "context-free instance has dictionary parameters",
                ));
            }
            let mut methods_chirho = Vec::new();
            for method_chirho in &info_chirho.methods_chirho {
                let source_chirho = instance_chirho
                    .methods_chirho
                    .iter()
                    .find(|source_chirho| {
                        source_chirho.binding_chirho.name_chirho == method_chirho.binding_chirho
                    })
                    .ok_or_else(|| {
                        invalid_evidence_chirho("instance method lost its producer identity")
                    })?;
                methods_chirho.push(self.binding_body_chirho(&source_chirho.binding_chirho)?);
            }
            self.types_chirho.reserve_term_chirho()?;
            let body_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: self.dictionary_key_chirho(info_chirho.class_chirho),
                args_chirho: methods_chirho,
            };
            let body_chirho =
                self.evidence_let_chirho(frame_chirho.definitions_chirho, body_chirho)?;
            Ok(CoreBindingChirho {
                binder_chirho: self.binder_chirho(instance_chirho.name_chirho)?,
                rhs_chirho: body_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            })
        })();
        self.evidence_scopes_chirho.pop();
        result_chirho
    }
}
