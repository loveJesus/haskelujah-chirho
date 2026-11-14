// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Check dependencies before expansion; argument kinds are checked even when erased.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_hir_chirho::{ClassDeclChirho, DataDeclChirho, SynonymDeclChirho};
use std::collections::HashSet;

impl CheckedModuleChirho {
    pub(super) fn first_order_headers_chirho(
        &mut self,
        data_chirho: &[DataDeclChirho],
        classes_chirho: &[ClassDeclChirho],
        synonyms_chirho: &[SynonymDeclChirho],
    ) -> Result<(), TcErrorChirho> {
        // Certify the admitted declaration subset before a consumer uses its headers.
        // Otherwise an earlier field can refute a valid later higher-kinded producer.
        let mut work_chirho = Vec::new();
        for declaration_chirho in data_chirho {
            for constructor_chirho in &declaration_chirho.constructors_chirho {
                for field_chirho in &constructor_chirho.fields_chirho {
                    self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                    work_chirho.push(field_chirho);
                }
            }
        }
        for declaration_chirho in classes_chirho {
            for method_chirho in &declaration_chirho.methods_chirho {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                work_chirho.push(&method_chirho.ty_chirho);
            }
        }
        for declaration_chirho in synonyms_chirho {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            work_chirho.push(&declaration_chirho.body_chirho);
        }
        while let Some(ty_chirho) = work_chirho.pop() {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            match ty_chirho {
                SurfaceTypeChirho::AppChirho(function_chirho, argument_chirho) => {
                    if matches!(
                        function_chirho.as_ref(),
                        SurfaceTypeChirho::VariableChirho(_)
                    ) {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::UnsupportedChirho,
                            "next requires declaration kind inference for an applied type variable",
                        ));
                    }
                    work_chirho.extend([function_chirho.as_ref(), argument_chirho.as_ref()]);
                }
                SurfaceTypeChirho::FunChirho(left_chirho, right_chirho) => {
                    work_chirho.extend([left_chirho.as_ref(), right_chirho.as_ref()])
                }
                SurfaceTypeChirho::ListChirho(item_chirho) => work_chirho.push(item_chirho),
                SurfaceTypeChirho::TupleChirho(items_chirho) => work_chirho.extend(items_chirho),
                _ => {}
            }
        }
        Ok(())
    }

    pub(super) fn synonym_declarations_chirho(
        &mut self,
        declarations_chirho: &[SynonymDeclChirho],
    ) -> Result<(), TcErrorChirho> {
        let mut indices_chirho = HashMap::new();
        let mut remaining_chirho = Vec::new();
        let mut dependents_chirho = Vec::new();
        for (index_chirho, declaration_chirho) in declarations_chirho.iter().enumerate() {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            indices_chirho.insert(declaration_chirho.name_chirho, index_chirho);
            remaining_chirho.push(0usize);
            dependents_chirho.push(Vec::new());
        }
        for (index_chirho, declaration_chirho) in declarations_chirho.iter().enumerate() {
            let mut dependencies_chirho = HashSet::new();
            let mut work_chirho = vec![&declaration_chirho.body_chirho];
            while let Some(ty_chirho) = work_chirho.pop() {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                match ty_chirho {
                    SurfaceTypeChirho::SynonymChirho(name_chirho) => {
                        if let Some(dependency_chirho) = indices_chirho.get(name_chirho) {
                            dependencies_chirho.insert(*dependency_chirho);
                        }
                    }
                    SurfaceTypeChirho::AppChirho(left_chirho, right_chirho)
                    | SurfaceTypeChirho::FunChirho(left_chirho, right_chirho) => {
                        work_chirho.extend([left_chirho.as_ref(), right_chirho.as_ref()]);
                    }
                    SurfaceTypeChirho::ListChirho(item_chirho) => work_chirho.push(item_chirho),
                    SurfaceTypeChirho::TupleChirho(items_chirho) => {
                        work_chirho.extend(items_chirho)
                    }
                    _ => {}
                }
            }
            remaining_chirho[index_chirho] = dependencies_chirho.len();
            for dependency_chirho in dependencies_chirho {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                dependents_chirho[dependency_chirho].push(index_chirho);
            }
        }
        let mut ready_chirho: Vec<_> = remaining_chirho
            .iter()
            .enumerate()
            .filter_map(|(index_chirho, count_chirho)| (*count_chirho == 0).then_some(index_chirho))
            .collect();
        let mut order_chirho = Vec::new();
        while let Some(index_chirho) = ready_chirho.pop() {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            order_chirho.push(index_chirho);
            for &dependent_chirho in &dependents_chirho[index_chirho] {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                remaining_chirho[dependent_chirho] -= 1;
                if remaining_chirho[dependent_chirho] == 0 {
                    ready_chirho.push(dependent_chirho);
                }
            }
        }
        if order_chirho.len() != declarations_chirho.len() {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::TypeMismatchChirho,
                "recursive type synonym dependency",
            ));
        }
        for index_chirho in order_chirho {
            let declaration_chirho = &declarations_chirho[index_chirho];
            let type_chirho = self
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::TypeChirho)?;
            let mut parameters_chirho = HashMap::new();
            let mut quantified_chirho = Vec::new();
            for name_chirho in &declaration_chirho.parameters_chirho {
                let (_, parameter_chirho) = self
                    .solver_chirho
                    .arena_chirho
                    .fresh_rigid_chirho(0, type_chirho)?;
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                parameters_chirho.insert(*name_chirho, parameter_chirho);
                quantified_chirho.push(parameter_chirho);
            }
            // Nominal headers precede their fields because fields may mention aliases.
            // Their first-order kinds are provisional until data checking succeeds:
            // a mismatch here may mean a valid, unsupported higher-kinded declaration.
            // Never refute source from that provisional premise (e.g. GHC tc238).
            let body_chirho = self.surface_with_chirho(&declaration_chirho.body_chirho, &parameters_chirho)
                .map_err(|error_chirho| {
                    if error_chirho.kind_chirho == TcErrorKindChirho::KindMismatchChirho {
                        TcErrorChirho::new_chirho(TcErrorKindChirho::UnsupportedChirho,
                            format!("next cannot certify synonym RHS kinds before full declaration kind inference: {}", error_chirho.message_chirho))
                    } else { error_chirho }
                })?;
            let constraint_chirho = self
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::ConstraintChirho)?;
            if self.solver_chirho.arena_chirho.kind_chirho(body_chirho)? == constraint_chirho {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::UnsupportedChirho,
                    "next does not yet check Constraint-kinded synonyms",
                ));
            }
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            self.synonyms_chirho.insert(
                declaration_chirho.name_chirho,
                CheckedSynonymChirho {
                    scheme_chirho: SchemeChirho {
                        quantified_chirho,
                        context_chirho: Vec::new(),
                        body_chirho,
                    },
                    higher_kinds_partial_chirho: declaration_chirho.higher_kinds_partial_chirho,
                },
            );
        }
        Ok(())
    }

    pub(super) fn synonym_application_chirho(
        &mut self,
        surface_chirho: &SurfaceTypeChirho,
        parameters_chirho: &HashMap<NameIdChirho, TyChirho>,
    ) -> Result<Option<TyChirho>, TcErrorChirho> {
        let mut head_chirho = surface_chirho;
        let mut arguments_chirho = Vec::new();
        while let SurfaceTypeChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            arguments_chirho.push(argument_chirho.as_ref());
            head_chirho = function_chirho;
        }
        let SurfaceTypeChirho::SynonymChirho(name_chirho) = head_chirho else {
            return Ok(None);
        };
        let synonym_chirho = self.synonyms_chirho.get(name_chirho).ok_or_else(|| {
            TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "synonym lacks its checked dependency",
            )
        })?;
        let arity_chirho = synonym_chirho.scheme_chirho.quantified_chirho.len();
        if arguments_chirho.len() < arity_chirho {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::TypeMismatchChirho,
                "type synonym must be saturated",
            ));
        }
        // Copy handles only, against the owner's allowance. Interface trees never enter here.
        for _binder_chirho in 0..=arity_chirho {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
        }
        let synonym_chirho = synonym_chirho.clone();
        let mut replacements_chirho = HashMap::new();
        let mut extra_chirho = Vec::new();
        for (index_chirho, argument_chirho) in arguments_chirho.into_iter().rev().enumerate() {
            let argument_chirho = self.surface_with_chirho(argument_chirho, parameters_chirho)?;
            let actual_chirho = self
                .solver_chirho
                .arena_chirho
                .kind_chirho(argument_chirho)?;
            if let Some(binder_chirho) = synonym_chirho
                .scheme_chirho
                .quantified_chirho
                .get(index_chirho)
            {
                let expected_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .kind_chirho(*binder_chirho)?;
                if actual_chirho != expected_chirho {
                    return Err(TcErrorChirho::new_chirho(
                        if synonym_chirho.higher_kinds_partial_chirho {
                            TcErrorKindChirho::UnsupportedChirho
                        } else {
                            TcErrorKindChirho::KindMismatchChirho
                        },
                        "next first-order synonym parameters require kind Type; PolyKinds generalization is not implemented",
                    ));
                }
                replacements_chirho.insert(*binder_chirho, argument_chirho);
            } else {
                extra_chirho.push(argument_chirho);
            }
        }
        let mut result_chirho = self.solver_chirho.arena_chirho.replace_chirho(
            synonym_chirho.scheme_chirho.body_chirho,
            replacements_chirho,
        )?;
        for argument_chirho in extra_chirho {
            result_chirho = self
                .solver_chirho
                .arena_chirho
                .intern_chirho(TyNodeChirho::AppChirho(result_chirho, argument_chirho))?;
            self.solver_chirho
                .arena_chirho
                .kind_chirho(result_chirho)
                .map_err(|error_chirho| {
                    if error_chirho.kind_chirho == TcErrorKindChirho::KindMismatchChirho {
                        TcErrorChirho::new_chirho(
                            TcErrorKindChirho::UnsupportedChirho,
                            "next does not yet generalize a synonym result's higher kinds",
                        )
                    } else {
                        error_chirho
                    }
                })?;
        }
        Ok(Some(result_chirho))
    }
}
