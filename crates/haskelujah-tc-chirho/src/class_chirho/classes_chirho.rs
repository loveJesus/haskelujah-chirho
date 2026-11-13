// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source class registration and checked instance methods. No self Given is manufactured.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;
use crate::{
    ClassInfoChirho, InstanceInfoChirho, InstanceMethodInfoChirho, MethodInfoChirho,
    ScopeKindChirho,
};
use haskelujah_hir_chirho::{ClassDeclChirho, InstanceDeclChirho};

impl CheckedModuleChirho {
    pub(super) fn class_headers_chirho(
        &mut self,
        declarations_chirho: &[ClassDeclChirho],
    ) -> Result<(), TcErrorChirho> {
        for declaration_chirho in declarations_chirho {
            self.solver_chirho
                .arena_chirho
                .declare_class_chirho(declaration_chirho.name_chirho)?;
        }
        Ok(())
    }

    pub(super) fn classes_chirho(
        &mut self,
        declarations_chirho: &[ClassDeclChirho],
    ) -> Result<(), TcErrorChirho> {
        for declaration_chirho in declarations_chirho {
            let kind_chirho = self
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::TypeChirho)?;
            let (_, parameter_chirho) = self
                .solver_chirho
                .arena_chirho
                .fresh_rigid_chirho(0, kind_chirho)?;
            let parameters_chirho =
                HashMap::from([(declaration_chirho.parameter_chirho, parameter_chirho)]);
            let predicate_chirho = self
                .solver_chirho
                .predicate_chirho(declaration_chirho.name_chirho, parameter_chirho)?;
            let mut methods_chirho = Vec::new();
            for method_chirho in &declaration_chirho.methods_chirho {
                let body_chirho =
                    self.surface_with_chirho(&method_chirho.ty_chirho, &parameters_chirho)?;
                if self.solver_chirho.arena_chirho.kind_chirho(body_chirho)? != kind_chirho {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::KindMismatchChirho,
                        "a class method must have kind Type",
                    ));
                }
                if !self
                    .solver_chirho
                    .arena_chirho
                    .free_variables_chirho(&[body_chirho])?
                    .contains(&parameter_chirho)
                {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::TypeMismatchChirho,
                        "ambiguous class method: its type does not mention the class parameter",
                    ));
                }
                let scheme_chirho = SchemeChirho {
                    body_chirho,
                    quantified_chirho: vec![parameter_chirho],
                    context_chirho: vec![predicate_chirho],
                };
                for _entry_chirho in 0..5 {
                    self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                }
                self.names_chirho
                    .insert(method_chirho.name_chirho, body_chirho);
                self.schemes_chirho
                    .insert(method_chirho.name_chirho, scheme_chirho.clone());
                methods_chirho.push(MethodInfoChirho {
                    name_chirho: method_chirho.name_chirho,
                    scheme_chirho,
                });
            }
            let mut superclasses_chirho = Vec::new();
            if declaration_chirho.superclasses_chirho.len()
                != declaration_chirho.superclass_selectors_chirho.len()
            {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::InvalidReferenceChirho,
                    "superclass selector has no declaration",
                ));
            }
            for (superclass_chirho, selector_chirho) in declaration_chirho
                .superclasses_chirho
                .iter()
                .zip(&declaration_chirho.superclass_selectors_chirho)
            {
                if superclass_chirho.argument_chirho != declaration_chirho.parameter_chirho {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::UnsupportedChirho,
                        "next superclass must mention the class parameter",
                    ));
                }
                let superclass_chirho = self
                    .solver_chirho
                    .predicate_chirho(superclass_chirho.class_chirho, parameter_chirho)?;
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                self.names_chirho
                    .insert(*selector_chirho, superclass_chirho);
                self.schemes_chirho.insert(
                    *selector_chirho,
                    SchemeChirho {
                        body_chirho: superclass_chirho,
                        quantified_chirho: vec![parameter_chirho],
                        context_chirho: vec![predicate_chirho],
                    },
                );
                superclasses_chirho.push(superclass_chirho);
            }
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            self.solver_chirho.classes_chirho.insert(
                declaration_chirho.name_chirho,
                ClassInfoChirho {
                    name_chirho: declaration_chirho.name_chirho,
                    parameter_chirho,
                    superclasses_chirho,
                    superclass_selectors_chirho: declaration_chirho
                        .superclass_selectors_chirho
                        .clone(),
                    methods_chirho,
                },
            );
        }
        self.solver_chirho.check_superclass_cycles_chirho()
    }

    pub(super) fn instances_chirho(
        &mut self,
        declarations_chirho: &[InstanceDeclChirho],
    ) -> Result<(), TcErrorChirho> {
        for declaration_chirho in declarations_chirho {
            let scope_chirho = self.solver_chirho.implication_kind_chirho(
                self.solver_chirho.root_chirho(),
                false,
                ScopeKindChirho::InstanceChirho,
            )?;
            let type_chirho = self
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::TypeChirho)?;
            let mut parameters_chirho = HashMap::new();
            let mut binders_chirho = Vec::new();
            for name_chirho in &declaration_chirho.binders_chirho {
                let binder_chirho = self.solver_chirho.signature_variable_chirho(
                    scope_chirho,
                    *name_chirho,
                    type_chirho,
                )?;
                parameters_chirho.insert(*name_chirho, binder_chirho);
                binders_chirho.push(binder_chirho);
            }
            let head_chirho =
                self.surface_with_chirho(&declaration_chirho.head_chirho, &parameters_chirho)?;
            self.check_partial_instance_chirho(declaration_chirho.class_chirho, head_chirho)?;
            let predicate_chirho = self
                .solver_chirho
                .predicate_chirho(declaration_chirho.class_chirho, head_chirho)?;
            self.solver_chirho.register_instance_head_chirho(
                declaration_chirho.class_chirho,
                head_chirho,
                &binders_chirho,
                declaration_chirho.name_chirho,
            )?;
            let mut context_chirho = Vec::new();
            for predicate_chirho in &declaration_chirho.context_chirho {
                let argument_chirho = *parameters_chirho
                    .get(&predicate_chirho.argument_chirho)
                    .ok_or_else(|| {
                        TcErrorChirho::new_chirho(
                            TcErrorKindChirho::InvalidReferenceChirho,
                            "instance context has no head binder",
                        )
                    })?;
                let predicate_chirho = self
                    .solver_chirho
                    .predicate_chirho(predicate_chirho.class_chirho, argument_chirho)?;
                self.solver_chirho
                    .given_dictionary_chirho(scope_chirho, predicate_chirho)?;
                context_chirho.push(predicate_chirho);
            }
            let class_chirho = self
                .solver_chirho
                .class_info_chirho(declaration_chirho.class_chirho)?
                .clone();
            let mut methods_chirho = Vec::new();
            for method_chirho in &class_chirho.methods_chirho {
                let implementation_chirho = declaration_chirho
                    .methods_chirho
                    .iter()
                    .find(|implementation_chirho| {
                        implementation_chirho.method_chirho == method_chirho.name_chirho
                    })
                    .ok_or_else(|| {
                        TcErrorChirho::new_chirho(
                            TcErrorKindChirho::UnsupportedChirho,
                            "next has no missing-method failure producer",
                        )
                    })?;
                let expected_chirho = self.solver_chirho.arena_chirho.replace_chirho(
                    method_chirho.scheme_chirho.body_chirho,
                    HashMap::from([(class_chirho.parameter_chirho, head_chirho)]),
                )?;
                let method_scope_chirho = self.solver_chirho.implication_kind_chirho(
                    scope_chirho,
                    false,
                    ScopeKindChirho::MethodChirho,
                )?;
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                self.names_chirho.insert(
                    implementation_chirho.binding_chirho.name_chirho,
                    expected_chirho,
                );
                self.abstractions_chirho.insert(
                    implementation_chirho.binding_chirho.name_chirho,
                    AbstractionChirho {
                        type_binders_chirho: Vec::new(),
                        dictionary_binders_chirho: Vec::new(),
                        scope_chirho: method_scope_chirho,
                    },
                );
                methods_chirho.push(InstanceMethodInfoChirho {
                    method_chirho: method_chirho.name_chirho,
                    binding_chirho: implementation_chirho.binding_chirho.name_chirho,
                    scope_chirho: method_scope_chirho,
                });
            }
            self.names_chirho
                .insert(declaration_chirho.name_chirho, predicate_chirho);
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            self.schemes_chirho.insert(
                declaration_chirho.name_chirho,
                SchemeChirho {
                    quantified_chirho: binders_chirho.clone(),
                    context_chirho: context_chirho.clone(),
                    body_chirho: predicate_chirho,
                },
            );
            self.solver_chirho.instances_chirho.insert(
                declaration_chirho.name_chirho,
                InstanceInfoChirho {
                    name_chirho: declaration_chirho.name_chirho,
                    class_chirho: declaration_chirho.class_chirho,
                    binders_chirho,
                    context_chirho,
                    head_chirho,
                    scope_chirho,
                    superclasses_chirho: Vec::new(),
                    methods_chirho,
                },
            );
        }
        Ok(())
    }

    pub(super) fn instance_methods_chirho(
        &mut self,
        declarations_chirho: &[InstanceDeclChirho],
    ) -> Result<(), TcErrorChirho> {
        for declaration_chirho in declarations_chirho {
            let info_chirho = self
                .solver_chirho
                .instance_info_chirho(declaration_chirho.name_chirho)?
                .clone();
            let class_chirho = self
                .solver_chirho
                .class_info_chirho(info_chirho.class_chirho)?
                .clone();
            let mut superclasses_chirho = Vec::new();
            for predicate_chirho in class_chirho.superclasses_chirho {
                let predicate_chirho = self.solver_chirho.arena_chirho.replace_chirho(
                    predicate_chirho,
                    HashMap::from([(class_chirho.parameter_chirho, info_chirho.head_chirho)]),
                )?;
                superclasses_chirho.push(self.solver_chirho.want_dictionary_origin_chirho(
                    info_chirho.scope_chirho,
                    predicate_chirho,
                    declaration_chirho.span_chirho,
                    crate::WantedOriginChirho::SuperclassChirho { naked_chirho: true },
                )?);
            }
            self.solver_chirho
                .instances_chirho
                .get_mut(&declaration_chirho.name_chirho)
                .ok_or_else(|| {
                    TcErrorChirho::new_chirho(
                        TcErrorKindChirho::InvalidReferenceChirho,
                        "instance lost its declaration",
                    )
                })?
                .superclasses_chirho = superclasses_chirho;
            for method_chirho in &info_chirho.methods_chirho {
                let implementation_chirho = declaration_chirho
                    .methods_chirho
                    .iter()
                    .find(|implementation_chirho| {
                        implementation_chirho.binding_chirho.name_chirho
                            == method_chirho.binding_chirho
                    })
                    .ok_or_else(|| {
                        TcErrorChirho::new_chirho(
                            TcErrorKindChirho::InvalidReferenceChirho,
                            "checked instance method has no source body",
                        )
                    })?;
                let previous_chirho = self.scope_chirho;
                self.scope_chirho = method_chirho.scope_chirho;
                let result_chirho = (|| {
                    let actual_chirho =
                        self.infer_chirho(&implementation_chirho.binding_chirho.body_chirho)?;
                    self.want_chirho(
                        actual_chirho,
                        self.names_chirho[&method_chirho.binding_chirho],
                        implementation_chirho.binding_chirho.span_chirho,
                    );
                    self.solver_chirho.solve_chirho()?;
                    self.solver_chirho.solve_dictionaries_chirho()
                })();
                self.scope_chirho = previous_chirho;
                result_chirho?;
            }
        }
        Ok(())
    }

    pub(super) fn finish_dictionaries_chirho(&mut self) -> Result<(), TcErrorChirho> {
        self.solver_chirho.solve_dictionaries_chirho()?;
        let Some(wanted_chirho) = self
            .solver_chirho
            .unresolved_dictionaries_chirho()?
            .into_iter()
            .next()
        else {
            return Ok(());
        };
        let predicate_chirho = self
            .solver_chirho
            .arena_chirho
            .replace_chirho(wanted_chirho.predicate_chirho, HashMap::new())?;
        let (class_chirho, argument_chirho) =
            self.solver_chirho.predicate_view_chirho(predicate_chirho)?;
        let mut ambiguous_chirho = false;
        for variable_chirho in self
            .solver_chirho
            .arena_chirho
            .free_variables_chirho(&[argument_chirho])?
        {
            ambiguous_chirho |= self
                .solver_chirho
                .arena_chirho
                .is_flexible_chirho(variable_chirho)?;
        }
        if self.instance_absence_is_partial_chirho(class_chirho, argument_chirho)? {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::UnsupportedChirho,
                "instance absence or defaulting depends on an incomplete source unit",
            ));
        }
        let name_chirho =
            &self.module_chirho.names_chirho[class_chirho.0 as usize].occurrence_chirho;
        let argument_chirho = self.describe_type_chirho(argument_chirho, &mut 128, 0)?;
        let mut error_chirho = TcErrorChirho::new_chirho(
            TcErrorKindChirho::TypeMismatchChirho,
            format!(
                "{} `{name_chirho} {argument_chirho}`",
                if ambiguous_chirho {
                    "ambiguous class constraint"
                } else {
                    "no instance for"
                }
            ),
        );
        error_chirho.span_chirho = wanted_chirho.span_chirho;
        Err(error_chirho)
    }

    fn describe_type_chirho(
        &self,
        ty_chirho: TyChirho,
        left_chirho: &mut usize,
        depth_chirho: usize,
    ) -> Result<String, TcErrorChirho> {
        if *left_chirho == 0 || depth_chirho > 32 {
            return Ok("...".into());
        }
        *left_chirho -= 1;
        let arena_chirho = &self.solver_chirho.arena_chirho;
        let ty_chirho = arena_chirho.resolve_chirho(ty_chirho)?;
        Ok(match arena_chirho.node_chirho(ty_chirho)? {
            TyNodeChirho::ConChirho(TyConChirho::BoolChirho) => "Bool".into(),
            TyNodeChirho::ConChirho(TyConChirho::CharChirho) => "Char".into(),
            TyNodeChirho::ConChirho(
                TyConChirho::DeclaredChirho(name_chirho) | TyConChirho::ClassChirho(name_chirho),
            ) => self.module_chirho.names_chirho[name_chirho.0 as usize]
                .occurrence_chirho
                .chars()
                .take(96)
                .collect(),
            TyNodeChirho::ConChirho(TyConChirho::ListChirho) => "[]".into(),
            TyNodeChirho::ConChirho(_) => "Type".into(),
            TyNodeChirho::MetaChirho(_) | TyNodeChirho::RigidChirho(_) => {
                format!("t{}Chirho", arena_chirho.variable_id_chirho(ty_chirho)?)
            }
            TyNodeChirho::AppChirho(function_chirho, argument_chirho) => {
                if matches!(
                    arena_chirho.node_chirho(*function_chirho)?,
                    TyNodeChirho::ConChirho(TyConChirho::ListChirho)
                ) {
                    format!(
                        "[{}]",
                        self.describe_type_chirho(*argument_chirho, left_chirho, depth_chirho + 1)?
                    )
                } else {
                    format!(
                        "({} {})",
                        self.describe_type_chirho(*function_chirho, left_chirho, depth_chirho + 1)?,
                        self.describe_type_chirho(*argument_chirho, left_chirho, depth_chirho + 1)?
                    )
                }
            }
            TyNodeChirho::FunChirho(argument_chirho, result_chirho) => format!(
                "({} -> {})",
                self.describe_type_chirho(*argument_chirho, left_chirho, depth_chirho + 1)?,
                self.describe_type_chirho(*result_chirho, left_chirho, depth_chirho + 1)?
            ),
            TyNodeChirho::TupleChirho(items_chirho) => format!(
                "({})",
                items_chirho
                    .iter()
                    .map(|item_chirho| self.describe_type_chirho(
                        *item_chirho,
                        left_chirho,
                        depth_chirho + 1
                    ))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ")
            ),
            TyNodeChirho::ForallChirho { .. } => "forall ...".into(),
        })
    }
}
