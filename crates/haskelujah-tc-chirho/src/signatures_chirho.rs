// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Complete rank-1 signatures use fresh owned skolems and written dictionary Givens.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;
use crate::ScopeKindChirho;
use haskelujah_hir_chirho::SurfaceSignatureChirho;

impl CheckedModuleChirho {
    pub(super) fn signature_chirho(
        &mut self,
        name_chirho: NameIdChirho,
        signature_chirho: &SurfaceSignatureChirho,
    ) -> Result<(), TcErrorChirho> {
        // A complete signature is checked once at its actual lexical parent. Its
        // scheme is available to every dependency group, which instantiates it freshly.
        let scope_chirho = self.solver_chirho.implication_kind_chirho(
            self.scope_chirho,
            false,
            ScopeKindChirho::SignatureChirho,
        )?;
        let kind_chirho = self
            .solver_chirho
            .arena_chirho
            .con_chirho(TyConChirho::TypeChirho)?;
        let mut variables_chirho = HashMap::new();
        let mut quantified_chirho = Vec::new();
        for binder_chirho in &signature_chirho.binders_chirho {
            let variable_chirho = self.solver_chirho.signature_variable_chirho(
                scope_chirho,
                *binder_chirho,
                kind_chirho,
            )?;
            variables_chirho.insert(*binder_chirho, variable_chirho);
            quantified_chirho.push(variable_chirho);
        }
        let body_chirho =
            self.surface_with_chirho(&signature_chirho.body_chirho, &variables_chirho)?;
        if self.solver_chirho.arena_chirho.kind_chirho(body_chirho)? != kind_chirho {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::KindMismatchChirho,
                "a value signature must have kind Type",
            ));
        }
        let body_variables_chirho = self
            .solver_chirho
            .arena_chirho
            .free_variables_chirho(&[body_chirho])?;
        let mut context_chirho = Vec::new();
        let mut dictionaries_chirho = Vec::new();
        for predicate_chirho in &signature_chirho.context_chirho {
            if !self
                .solver_chirho
                .classes_chirho
                .contains_key(&predicate_chirho.class_chirho)
            {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::KindMismatchChirho,
                    "a signature context requires a declared class",
                ));
            }
            let argument_chirho = self
                .surface_variable_chirho(predicate_chirho.argument_chirho, &variables_chirho)?
                .ok_or_else(|| {
                    TcErrorChirho::new_chirho(
                        TcErrorKindChirho::InvalidReferenceChirho,
                        "signature predicate lost its lexical binder",
                    )
                })?;
            if quantified_chirho.contains(&argument_chirho)
                && !body_variables_chirho.contains(&argument_chirho)
            {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::TypeMismatchChirho,
                    "ambiguous written class context: its variable is absent from the value type",
                ));
            }
            let predicate_chirho = self
                .solver_chirho
                .predicate_chirho(predicate_chirho.class_chirho, argument_chirho)?;
            let dictionary_chirho = self
                .solver_chirho
                .given_dictionary_chirho(scope_chirho, predicate_chirho)?;
            context_chirho.push(predicate_chirho);
            dictionaries_chirho.push(dictionary_chirho);
        }
        for _entry_chirho in 0..3 + 2 * quantified_chirho.len() + context_chirho.len() {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
        }
        self.names_chirho.insert(name_chirho, body_chirho);
        self.schemes_chirho.insert(
            name_chirho,
            SchemeChirho {
                body_chirho,
                quantified_chirho: quantified_chirho.clone(),
                context_chirho,
            },
        );
        self.abstractions_chirho.insert(
            name_chirho,
            AbstractionChirho {
                type_binders_chirho: quantified_chirho,
                dictionary_binders_chirho: dictionaries_chirho,
                scope_chirho,
            },
        );
        Ok(())
    }

    pub(super) fn check_signature_chirho(
        &mut self,
        binding_chirho: &BindingChirho,
    ) -> Result<(), TcErrorChirho> {
        let scope_chirho = self
            .abstractions_chirho
            .get(&binding_chirho.name_chirho)
            .ok_or_else(|| {
                TcErrorChirho::new_chirho(
                    TcErrorKindChirho::InvalidReferenceChirho,
                    "signed binding has no checked signature scope",
                )
            })?
            .scope_chirho;
        let parent_chirho = self.scope_chirho;
        self.scope_chirho = scope_chirho;
        let result_chirho = (|| {
            let actual_chirho = self.infer_chirho(&binding_chirho.body_chirho)?;
            self.want_chirho(
                actual_chirho,
                self.names_chirho[&binding_chirho.name_chirho],
                binding_chirho.span_chirho,
            );
            self.solver_chirho.solve_chirho()?;
            self.solver_chirho.solve_dictionaries_chirho()
        })();
        self.scope_chirho = parent_chirho;
        result_chirho
    }
}
