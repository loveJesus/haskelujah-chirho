// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ordinary algebraic constructors and patterns, following GHC.Tc.Gen.Pat's no-givens arm.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;
use haskelujah_hir_chirho::{DataDeclChirho, MatchArmChirho, PatternChirho, PatternKindChirho};

impl CheckedModuleChirho {
    pub(super) fn surface_with_chirho(
        &mut self,
        surface_chirho: &SurfaceTypeChirho,
        parameters_chirho: &HashMap<NameIdChirho, TyChirho>,
    ) -> Result<TyChirho, TcErrorChirho> {
        Ok(match surface_chirho {
            SurfaceTypeChirho::BoolChirho => self
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::BoolChirho)?,
            SurfaceTypeChirho::CharChirho => self
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::CharChirho)?,
            SurfaceTypeChirho::DeclaredChirho(name_chirho) => {
                let ty_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .con_chirho(TyConChirho::DeclaredChirho(*name_chirho))?;
                self.solver_chirho.arena_chirho.kind_chirho(ty_chirho)?;
                ty_chirho
            }
            SurfaceTypeChirho::VariableChirho(name_chirho) => {
                *parameters_chirho.get(name_chirho).ok_or_else(|| {
                    TcErrorChirho::new_chirho(
                        TcErrorKindChirho::UnsupportedChirho,
                        "next does not yet check explicit polymorphic signatures",
                    )
                })?
            }
            SurfaceTypeChirho::AppChirho(function_chirho, argument_chirho) => {
                let function_chirho =
                    self.surface_with_chirho(function_chirho, parameters_chirho)?;
                let argument_chirho =
                    self.surface_with_chirho(argument_chirho, parameters_chirho)?;
                if matches!(
                    self.solver_chirho
                        .arena_chirho
                        .node_chirho(function_chirho)?,
                    TyNodeChirho::MetaChirho(_) | TyNodeChirho::RigidChirho(_)
                ) {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::UnsupportedChirho,
                        "next does not yet infer higher-kinded data parameters",
                    ));
                }
                let ty_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .intern_chirho(TyNodeChirho::AppChirho(function_chirho, argument_chirho))?;
                self.solver_chirho.arena_chirho.kind_chirho(ty_chirho)?;
                ty_chirho
            }
            SurfaceTypeChirho::FunChirho(argument_chirho, result_chirho) => {
                let argument_chirho =
                    self.surface_with_chirho(argument_chirho, parameters_chirho)?;
                let result_chirho = self.surface_with_chirho(result_chirho, parameters_chirho)?;
                self.solver_chirho
                    .arena_chirho
                    .function_chirho(argument_chirho, result_chirho)?
            }
            SurfaceTypeChirho::ListChirho(item_chirho) => {
                let item_chirho = self.surface_with_chirho(item_chirho, parameters_chirho)?;
                self.solver_chirho.arena_chirho.list_chirho(item_chirho)?
            }
            SurfaceTypeChirho::TupleChirho(items_chirho) => {
                let type_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .con_chirho(TyConChirho::TypeChirho)?;
                let mut fields_chirho = Vec::new();
                for item_chirho in items_chirho {
                    let ty_chirho = self.surface_with_chirho(item_chirho, parameters_chirho)?;
                    if self.solver_chirho.arena_chirho.kind_chirho(ty_chirho)? != type_chirho {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::KindMismatchChirho,
                            "tuple fields require kind Type",
                        ));
                    }
                    fields_chirho.push(ty_chirho);
                }
                self.solver_chirho
                    .arena_chirho
                    .intern_chirho(TyNodeChirho::TupleChirho(fields_chirho))?
            }
        })
    }

    pub(super) fn data_chirho(
        &mut self,
        declarations_chirho: &[DataDeclChirho],
    ) -> Result<(), TcErrorChirho> {
        for declaration_chirho in declarations_chirho {
            self.solver_chirho.arena_chirho.declare_chirho(
                declaration_chirho.name_chirho,
                declaration_chirho.parameters_chirho.len(),
            )?;
        }
        for declaration_chirho in declarations_chirho {
            let mut parameters_chirho = HashMap::new();
            let type_chirho = self
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::TypeChirho)?;
            let mut result_chirho = self
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::DeclaredChirho(declaration_chirho.name_chirho))?;
            for name_chirho in &declaration_chirho.parameters_chirho {
                let parameter_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .fresh_meta_chirho(1, type_chirho)?;
                parameters_chirho.insert(*name_chirho, parameter_chirho);
                result_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .intern_chirho(TyNodeChirho::AppChirho(result_chirho, parameter_chirho))?;
            }
            let mut roots_chirho = Vec::new();
            for constructor_chirho in &declaration_chirho.constructors_chirho {
                let mut fields_chirho = Vec::new();
                let mut body_chirho = result_chirho;
                for field_chirho in &constructor_chirho.fields_chirho {
                    let field_chirho =
                        self.surface_with_chirho(field_chirho, &parameters_chirho)?;
                    if self.solver_chirho.arena_chirho.kind_chirho(field_chirho)? != type_chirho {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::KindMismatchChirho,
                            "constructor fields require kind Type",
                        ));
                    }
                    self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                    fields_chirho.push(field_chirho);
                }
                for field_chirho in fields_chirho.iter().rev() {
                    body_chirho = self
                        .solver_chirho
                        .arena_chirho
                        .function_chirho(*field_chirho, body_chirho)?;
                }
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                self.names_chirho
                    .insert(constructor_chirho.name_chirho, body_chirho);
                self.constructors_chirho
                    .insert(constructor_chirho.name_chirho, fields_chirho);
                roots_chirho.push(body_chirho);
            }
            let quantified_chirho = self
                .solver_chirho
                .arena_chirho
                .quantify_group_chirho(&roots_chirho, 0)?;
            for constructor_chirho in &declaration_chirho.constructors_chirho {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                for _parameter_chirho in &quantified_chirho {
                    self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                }
                self.schemes_chirho.insert(
                    constructor_chirho.name_chirho,
                    SchemeChirho {
                        context_chirho: Vec::new(),
                        body_chirho: self.names_chirho[&constructor_chirho.name_chirho],
                        quantified_chirho: quantified_chirho.clone(),
                    },
                );
            }
        }
        Ok(())
    }

    fn pattern_chirho(
        &mut self,
        pattern_chirho: &PatternChirho,
        expected_chirho: TyChirho,
    ) -> Result<(), TcErrorChirho> {
        self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
        self.patterns_chirho
            .insert(pattern_chirho.id_chirho, expected_chirho);
        match &pattern_chirho.kind_chirho {
            PatternKindChirho::WildcardChirho => {}
            PatternKindChirho::VariableChirho(name_chirho) => {
                self.names_chirho.insert(*name_chirho, expected_chirho);
            }
            PatternKindChirho::AsChirho(name_chirho, inner_chirho) => {
                self.names_chirho.insert(*name_chirho, expected_chirho);
                self.pattern_chirho(inner_chirho, expected_chirho)?;
            }
            PatternKindChirho::CharChirho(_) => {
                let char_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .con_chirho(TyConChirho::CharChirho)?;
                self.want_chirho(expected_chirho, char_chirho, pattern_chirho.span_chirho);
            }
            PatternKindChirho::TupleChirho(items_chirho) => {
                let mut fields_chirho = Vec::new();
                for item_chirho in items_chirho {
                    let ty_chirho = self.fresh_chirho()?;
                    self.pattern_chirho(item_chirho, ty_chirho)?;
                    fields_chirho.push(ty_chirho);
                }
                let tuple_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .intern_chirho(TyNodeChirho::TupleChirho(fields_chirho))?;
                self.want_chirho(expected_chirho, tuple_chirho, pattern_chirho.span_chirho);
            }
            PatternKindChirho::ListChirho(items_chirho) => {
                let item_chirho = self.fresh_chirho()?;
                let list_chirho = self.solver_chirho.arena_chirho.list_chirho(item_chirho)?;
                self.want_chirho(expected_chirho, list_chirho, pattern_chirho.span_chirho);
                for pattern_chirho in items_chirho {
                    self.pattern_chirho(pattern_chirho, item_chirho)?;
                }
            }
            PatternKindChirho::ConsChirho(head_chirho, tail_chirho) => {
                let item_chirho = self.fresh_chirho()?;
                let list_chirho = self.solver_chirho.arena_chirho.list_chirho(item_chirho)?;
                self.want_chirho(expected_chirho, list_chirho, pattern_chirho.span_chirho);
                self.pattern_chirho(head_chirho, item_chirho)?;
                self.pattern_chirho(tail_chirho, list_chirho)?;
            }
            PatternKindChirho::ConstructorChirho(name_chirho, arguments_chirho) => {
                let arity_chirho = self
                    .constructors_chirho
                    .get(name_chirho)
                    .map(Vec::len)
                    .ok_or_else(|| {
                        TcErrorChirho::new_chirho(
                            TcErrorKindChirho::InvalidReferenceChirho,
                            "pattern has no checked constructor declaration",
                        )
                    })?;
                if arity_chirho != arguments_chirho.len() {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::TypeMismatchChirho,
                        "constructor pattern has the wrong number of fields",
                    ));
                }
                let scheme_chirho = self.schemes_chirho[name_chirho].clone();
                let level_chirho = self.solver_chirho.level_chirho(self.scope_chirho)?;
                let (mut body_chirho, _, _) = self
                    .solver_chirho
                    .arena_chirho
                    .instantiate_chirho(&scheme_chirho, level_chirho)?;
                for argument_chirho in arguments_chirho {
                    let TyNodeChirho::FunChirho(field_chirho, result_chirho) = self
                        .solver_chirho
                        .arena_chirho
                        .node_chirho(body_chirho)?
                        .clone()
                    else {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::InvalidReferenceChirho,
                            "constructor scheme disagrees with its field catalogue",
                        ));
                    };
                    self.pattern_chirho(argument_chirho, field_chirho)?;
                    body_chirho = result_chirho;
                }
                self.want_chirho(body_chirho, expected_chirho, pattern_chirho.span_chirho);
            }
        }
        Ok(())
    }

    pub(super) fn match_chirho(
        &mut self,
        inputs_chirho: &[ExprChirho],
        arms_chirho: &[MatchArmChirho],
    ) -> Result<TyChirho, TcErrorChirho> {
        let inputs_chirho = inputs_chirho
            .iter()
            .map(|input_chirho| self.infer_chirho(input_chirho))
            .collect::<Result<Vec<_>, _>>()?;
        let result_chirho = self.fresh_chirho()?;
        for arm_chirho in arms_chirho {
            if arm_chirho.patterns_chirho.len() != inputs_chirho.len() {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::InvalidReferenceChirho,
                    "pattern matrix has the wrong arity",
                ));
            }
            for (pattern_chirho, ty_chirho) in arm_chirho.patterns_chirho.iter().zip(&inputs_chirho)
            {
                self.pattern_chirho(pattern_chirho, *ty_chirho)?;
            }
            let body_chirho = self.infer_chirho(&arm_chirho.body_chirho)?;
            self.want_chirho(
                body_chirho,
                result_chirho,
                arm_chirho.body_chirho.span_chirho,
            );
        }
        Ok(result_chirho)
    }
}
