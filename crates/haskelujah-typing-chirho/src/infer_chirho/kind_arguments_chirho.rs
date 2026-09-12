// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Materialize solved invisible arguments, including implicit source applications.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::*;
use crate::kind_chirho::KindChirho;

impl InferCtxChirho {
    pub(super) fn elaborated_nominal_application_chirho(
        &mut self,
        source_chirho: &TypeChirho,
        variables_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> Option<TyChirho> {
        let mut head_chirho = source_chirho;
        let mut arguments_chirho = Vec::new();
        loop {
            match head_chirho {
                TypeChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    arguments_chirho.push(arg_chirho.as_ref());
                    head_chirho = fun_chirho;
                }
                TypeChirho::KindAppChirho { fun_chirho, .. } => head_chirho = fun_chirho,
                TypeChirho::ParenChirho { inner_chirho, .. } => head_chirho = inner_chirho,
                _ => break,
            }
        }
        let TypeChirho::ConChirho(name_chirho) = head_chirho else {
            return None;
        };
        let elaboration_chirho = self.kind_elaboration_chirho.as_ref()?;
        let binders_chirho = elaboration_chirho
            .nominal_heads_chirho
            .get(&name_chirho.full_name_chirho())?;
        let indices_chirho = elaboration_chirho
            .applications_chirho
            .get(&source_chirho.span_chirho())?;
        if indices_chirho.len() != binders_chirho.len() {
            return None;
        }
        let indices_chirho = indices_chirho.clone();
        let mut result_chirho = TyChirho::ConChirho(
            self.normalize_imported_type_name_chirho(&name_chirho.full_name_chirho()),
        );
        for index_chirho in &indices_chirho {
            let argument_chirho =
                self.kind_term_type_chirho(index_chirho, variables_chirho, &mut Vec::new());
            result_chirho =
                TyChirho::KindAppChirho(Box::new(result_chirho), Box::new(argument_chirho));
        }
        for argument_chirho in arguments_chirho.into_iter().rev() {
            result_chirho = TyChirho::AppChirho(
                Box::new(result_chirho),
                Box::new(self.ast_type_to_ty_chirho(argument_chirho, variables_chirho)),
            );
        }
        Some(result_chirho)
    }

    /// Kind metavariables share only inside the caller's lexical conversion map.
    /// They never become a module-global type variable reused across signatures.
    fn kind_term_type_chirho(
        &mut self,
        term_chirho: &KindChirho,
        variables_chirho: &mut HashMap<String, TyVarChirho>,
        bound_chirho: &mut Vec<TyVarChirho>,
    ) -> TyChirho {
        match term_chirho {
            KindChirho::StarChirho => TyChirho::ConChirho("Type".to_owned()),
            KindChirho::ConstraintChirho => TyChirho::ConChirho("Constraint".to_owned()),
            KindChirho::ConChirho(name_chirho) => TyChirho::ConChirho(name_chirho.clone()),
            KindChirho::VarChirho(identity_chirho) | KindChirho::RigidChirho(identity_chirho) => {
                let name_chirho = self
                    .kind_elaboration_chirho
                    .as_ref()
                    .and_then(|elaboration_chirho| {
                        elaboration_chirho.source_names_chirho.get(identity_chirho)
                    })
                    .cloned()
                    .unwrap_or_else(|| format!("$kind_chirho_{}", identity_chirho.0));
                let variable_chirho = self.kind_type_variable_chirho(name_chirho, variables_chirho);
                TyChirho::VarChirho(variable_chirho)
            }
            KindChirho::ArrowChirho(argument_chirho, result_chirho) => TyChirho::fun_chirho(
                self.kind_term_type_chirho(argument_chirho, variables_chirho, bound_chirho),
                self.kind_term_type_chirho(result_chirho, variables_chirho, bound_chirho),
            ),
            KindChirho::AppChirho(fun_chirho, argument_chirho) => TyChirho::AppChirho(
                Box::new(self.kind_term_type_chirho(fun_chirho, variables_chirho, bound_chirho)),
                Box::new(self.kind_term_type_chirho(
                    argument_chirho,
                    variables_chirho,
                    bound_chirho,
                )),
            ),
            KindChirho::KindAppChirho(fun_chirho, argument_chirho) => TyChirho::KindAppChirho(
                Box::new(self.kind_term_type_chirho(fun_chirho, variables_chirho, bound_chirho)),
                Box::new(self.kind_term_type_chirho(
                    argument_chirho,
                    variables_chirho,
                    bound_chirho,
                )),
            ),
            KindChirho::BoundChirho(index_chirho) => {
                TyChirho::VarChirho(bound_chirho[bound_chirho.len() - 1 - *index_chirho as usize])
            }
            KindChirho::DependentChirho { result_chirho, .. } => {
                let variable_chirho = TyVarChirho(self.next_var_chirho);
                self.next_var_chirho += 1;
                bound_chirho.push(variable_chirho);
                let result_chirho =
                    self.kind_term_type_chirho(result_chirho, variables_chirho, bound_chirho);
                bound_chirho.pop();
                TyChirho::RequiredForallChirho {
                    vars_chirho: vec![variable_chirho],
                    body_chirho: Box::new(result_chirho),
                }
            }
        }
    }

    fn kind_type_variable_chirho(
        &mut self,
        name_chirho: String,
        variables_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> TyVarChirho {
        *variables_chirho.entry(name_chirho).or_insert_with(|| {
            let variable_chirho = TyVarChirho(self.next_var_chirho);
            self.next_var_chirho += 1;
            variable_chirho
        })
    }

    pub(super) fn data_head_kind_arguments_chirho(
        &mut self,
        name_chirho: &str,
        variables_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> TyChirho {
        let binders_chirho = self
            .kind_elaboration_chirho
            .as_ref()
            .and_then(|elaboration_chirho| elaboration_chirho.nominal_heads_chirho.get(name_chirho))
            .cloned()
            .unwrap_or_default();
        let mut result_chirho = TyChirho::ConChirho(name_chirho.to_owned());
        for (index_chirho, binder_chirho) in binders_chirho.into_iter().enumerate() {
            let name_chirho = binder_chirho
                .name_chirho
                .unwrap_or_else(|| format!("$head_kind_chirho_{index_chirho}"));
            let variable_chirho = self.kind_type_variable_chirho(name_chirho, variables_chirho);
            result_chirho = TyChirho::KindAppChirho(
                Box::new(result_chirho),
                Box::new(TyChirho::VarChirho(variable_chirho)),
            );
        }
        result_chirho
    }
}
