// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Surface agreement reads the selected declaration and its actual checked telescope.
//! Workflow: compiler-pipeline-chirho/builtin-type-environment-chirho.

use haskelujah_ast_chirho::name_chirho::{
    TypeConstructorDeclarationChirho, TypeConstructorRoleChirho,
};
use haskelujah_ast_chirho::promotion_chirho::PromotionScopeChirho;
use haskelujah_naming_chirho::{ModuleIfaceChirho, build_iface_with_imports_chirho};
use haskelujah_parser_chirho::{
    cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
};
use haskelujah_span_chirho::FileIdChirho;
use haskelujah_typing_chirho::infer_chirho::{TypeFamilyEnvChirho, TypeFamilyScopeChirho};
use haskelujah_typing_chirho::kind_chirho::elaboration_chirho::{
    ElaborationImportsChirho, elaborate_module_chirho,
};
use haskelujah_typing_chirho::ty_chirho::declarations_chirho::{
    DeclarationSchemeChirho, TelescopeVisibilityChirho,
};
use haskelujah_typing_chirho::{SchemeChirho, TyChirho};

pub(super) struct CheckedSurfaceChirho {
    registry_chirho: TypeFamilyEnvChirho,
}

impl CheckedSurfaceChirho {
    /// The public producer supplies this catalogue; no scheme or arity is invented here.
    pub(super) fn from_source_chirho(
        source_chirho: &str,
        ifaces_chirho: &[ModuleIfaceChirho],
    ) -> (Self, ModuleIfaceChirho) {
        let mut module_chirho = lower_module_chirho(
            &ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO).parse_chirho(),
            FileIdChirho::SYNTHETIC_CHIRHO,
        );
        let type_scope_chirho = haskelujah_naming_chirho::build_type_constructor_scope_chirho(
            &module_chirho,
            ifaces_chirho,
        );
        let classes_chirho = haskelujah_naming_chirho::build_builtin_class_scope_chirho(
            &module_chirho,
            ifaces_chirho,
        );
        let empty_registry_chirho = TypeFamilyEnvChirho::new_chirho();
        let elaborated_chirho = elaborate_module_chirho(
            &mut module_chirho,
            ElaborationImportsChirho {
                registry_chirho: &empty_registry_chirho,
                type_scope_chirho: &type_scope_chirho,
                family_scope_chirho: &TypeFamilyScopeChirho::default(),
                promotion_scope_chirho: &PromotionScopeChirho::default(),
                builtin_classes_chirho: &classes_chirho,
            },
        )
        .expect("surface fixture must have an actual checked declaration catalogue Chirho");
        let iface_chirho = build_iface_with_imports_chirho(&module_chirho, ifaces_chirho)
            .expect("surface fixture must publish its actual type declarations Chirho");
        (
            Self {
                registry_chirho: elaborated_chirho.registry_chirho,
            },
            iface_chirho,
        )
    }

    pub(super) fn builtin_chirho(ifaces_chirho: &[ModuleIfaceChirho]) -> Self {
        Self::from_source_chirho("module AgreementSurfaceChirho where\n", ifaces_chirho).0
    }

    fn arguments_chirho<'argument_chirho>(
        &self,
        head_chirho: &str,
        arguments_chirho: &[&'argument_chirho TyChirho],
        iface_chirho: &ModuleIfaceChirho,
    ) -> Result<Vec<&'argument_chirho TyChirho>, String> {
        let checked_chirho = self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(head_chirho);
        let selected_chirho =
            iface_chirho
                .exports_chirho
                .types_chirho
                .values()
                .find_map(|ty_chirho| {
                    ty_chirho
                        .declaration_chirho
                        .as_ref()
                        .filter(|declaration_chirho| {
                            declaration_chirho.type_key_chirho() == head_chirho
                        })
                });
        match (selected_chirho, checked_chirho) {
            (Some(declaration_chirho), Some(checked_chirho)) => {
                project_arguments_chirho(declaration_chirho, checked_chirho, arguments_chirho)
            }
            (_, Some(checked_chirho)) if checked_chirho.hidden_arity_chirho() != 0 => {
                Err("hidden applications have no selected declaration proof Chirho".to_string())
            }
            (Some(_), None) => {
                Err("selected declaration has no checked telescope Chirho".to_string())
            }
            _ => Ok(arguments_chirho.to_vec()),
        }
    }

    fn ty_text_chirho(
        &self,
        ty_chirho: &TyChirho,
        top_chirho: bool,
        iface_chirho: &ModuleIfaceChirho,
    ) -> Result<String, String> {
        let wrap_chirho = |text_chirho: String| {
            if top_chirho {
                text_chirho
            } else {
                format!("({text_chirho})")
            }
        };
        Ok(match ty_chirho {
            TyChirho::VarChirho(variable_chirho) => format!("t{}", variable_chirho.0),
            TyChirho::ConChirho(name_chirho) => {
                if self
                    .registry_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(name_chirho)
                    .is_some_and(|checked_chirho| checked_chirho.hidden_arity_chirho() != 0)
                {
                    self.arguments_chirho(name_chirho, &[], iface_chirho)?;
                }
                name_chirho.clone()
            }
            TyChirho::ForallVarChirho(name_chirho) => name_chirho.clone(),
            TyChirho::ListChirho(inner_chirho) => format!(
                "[{}]",
                self.ty_text_chirho(inner_chirho, true, iface_chirho)?
            ),
            TyChirho::TupleChirho(elements_chirho) => format!(
                "({})",
                elements_chirho
                    .iter()
                    .map(|element_chirho| self.ty_text_chirho(element_chirho, true, iface_chirho))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ")
            ),
            TyChirho::FunChirho(argument_chirho, result_chirho, _) => {
                let argument_text_chirho =
                    self.ty_text_chirho(argument_chirho, true, iface_chirho)?;
                let argument_text_chirho = if matches!(**argument_chirho, TyChirho::FunChirho(..)) {
                    format!("({argument_text_chirho})")
                } else {
                    argument_text_chirho
                };
                wrap_chirho(format!(
                    "{argument_text_chirho} -> {}",
                    self.ty_text_chirho(result_chirho, true, iface_chirho)?
                ))
            }
            TyChirho::AppChirho(..) => {
                let mut arguments_chirho = Vec::new();
                let mut head_chirho = ty_chirho;
                while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
                    arguments_chirho.push(argument_chirho.as_ref());
                    head_chirho = function_chirho;
                }
                arguments_chirho.reverse();
                if let TyChirho::ConChirho(key_chirho) = head_chirho {
                    arguments_chirho =
                        self.arguments_chirho(key_chirho, &arguments_chirho, iface_chirho)?;
                }
                // The head is already checked as part of the complete application;
                // rendering it alone must not mistake it for a partial application.
                let head_text_chirho = if let TyChirho::ConChirho(key_chirho) = head_chirho {
                    iface_chirho
                        .exports_chirho
                        .types_chirho
                        .values()
                        .find(|ty_chirho| {
                            ty_chirho.declaration_chirho.as_ref().is_some_and(
                                |declaration_chirho| {
                                    declaration_chirho.type_key_chirho() == *key_chirho
                                },
                            )
                        })
                        .map_or_else(
                            || key_chirho.clone(),
                            |ty_chirho| ty_chirho.name_chirho.clone(),
                        )
                } else {
                    self.ty_text_chirho(head_chirho, false, iface_chirho)?
                };
                wrap_chirho(format!(
                    "{} {}",
                    head_text_chirho,
                    arguments_chirho
                        .into_iter()
                        .map(|argument_chirho| self.ty_text_chirho(
                            argument_chirho,
                            false,
                            iface_chirho
                        ))
                        .collect::<Result<Vec<_>, _>>()?
                        .join(" ")
                ))
            }
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => wrap_chirho(format!(
                "({}) => {}",
                context_chirho
                    .iter()
                    .map(|term_chirho| self.ty_text_chirho(term_chirho, true, iface_chirho))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", "),
                self.ty_text_chirho(body_chirho, true, iface_chirho)?
            )),
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => format!(
                "(forall {}. {})",
                vars_chirho
                    .iter()
                    .map(|variable_chirho| format!("t{}", variable_chirho.0))
                    .collect::<Vec<_>>()
                    .join(" "),
                self.ty_text_chirho(body_chirho, true, iface_chirho)?
            ),
        })
    }

    pub(super) fn scheme_text_chirho(
        &self,
        scheme_chirho: &SchemeChirho,
        iface_chirho: &ModuleIfaceChirho,
    ) -> Result<String, String> {
        let body_chirho = self.ty_text_chirho(&scheme_chirho.ty_chirho, true, iface_chirho)?;
        if scheme_chirho.preds_chirho.is_empty() {
            return Ok(body_chirho);
        }
        let predicates_chirho = scheme_chirho
            .preds_chirho
            .iter()
            .map(|predicate_chirho| {
                let mut parts_chirho = vec![
                    predicate_chirho.class_name_chirho.clone(),
                    self.ty_text_chirho(&predicate_chirho.ty_chirho, false, iface_chirho)?,
                ];
                for extra_chirho in &predicate_chirho.extra_tys_chirho {
                    parts_chirho.push(self.ty_text_chirho(extra_chirho, false, iface_chirho)?);
                }
                Ok(parts_chirho.join(" "))
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(format!(
            "({}) => {body_chirho}",
            predicates_chirho.join(", ")
        ))
    }
}

fn project_arguments_chirho<'argument_chirho>(
    declaration_chirho: &TypeConstructorDeclarationChirho,
    checked_chirho: &DeclarationSchemeChirho,
    arguments_chirho: &[&'argument_chirho TyChirho],
) -> Result<Vec<&'argument_chirho TyChirho>, String> {
    let required_chirho = checked_chirho
        .binders_chirho
        .iter()
        .filter(|binder_chirho| {
            binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho
        })
        .count();
    let unique_chirho = checked_chirho
        .binders_chirho
        .iter()
        .map(|binder_chirho| binder_chirho.variable_chirho)
        .collect::<std::collections::HashSet<_>>();
    if checked_chirho.held_chirho
        || checked_chirho.owner_chirho.is_some()
        || checked_chirho.role_chirho == TypeConstructorRoleChirho::UnknownChirho
        || checked_chirho.declaration_chirho.as_ref() != Some(declaration_chirho)
        || checked_chirho.identity_chirho.as_ref() != declaration_chirho.type_key_chirho()
        || checked_chirho.role_chirho != declaration_chirho.role_chirho
        || checked_chirho.visible_arity_chirho != required_chirho
        || checked_chirho.reduction_arity_chirho != checked_chirho.binders_chirho.len()
        || unique_chirho.len() != checked_chirho.binders_chirho.len()
        || arguments_chirho.len() != checked_chirho.binders_chirho.len()
    {
        return Err(
            "surface application lacks a complete selected checked telescope Chirho".to_string(),
        );
    }
    Ok(checked_chirho
        .binders_chirho
        .iter()
        .zip(arguments_chirho)
        .filter_map(|(binder_chirho, argument_chirho)| {
            (binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho)
                .then_some(*argument_chirho)
        })
        .collect())
}

fn application_scheme_chirho(
    declaration_chirho: &TypeConstructorDeclarationChirho,
    arguments_chirho: &[TyChirho],
) -> SchemeChirho {
    SchemeChirho {
        vars_chirho: Vec::new().into(),
        preds_chirho: Vec::new(),
        ty_chirho: arguments_chirho.iter().fold(
            TyChirho::ConChirho(declaration_chirho.type_key_chirho()),
            |function_chirho, argument_chirho| {
                TyChirho::AppChirho(Box::new(function_chirho), Box::new(argument_chirho.clone()))
            },
        ),
    }
}

#[test]
fn checked_compiler_hidden_and_source_required_arguments_keep_their_surface_meaning_chirho() {
    let ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let (surface_chirho, source_iface_chirho) = CheckedSurfaceChirho::from_source_chirho(
        "module SourceSurfaceChirho where\ndata Proxy leftChirho rightChirho = SourceProxyChirho\n",
        &ifaces_chirho,
    );
    let builtin_iface_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Proxy")
        .unwrap();
    let schemes_chirho = haskelujah_typing_chirho::infer_chirho::builtin_value_schemes_chirho();
    assert_eq!(
        super::normalized_type_chirho(
            &surface_chirho
                .scheme_text_chirho(&schemes_chirho["Proxy"], builtin_iface_chirho)
                .unwrap()
        ),
        "Proxy v0",
    );
    let source_declaration_chirho = source_iface_chirho.exports_chirho.types_chirho["Proxy"]
        .declaration_chirho
        .as_ref()
        .unwrap();
    let source_scheme_chirho = application_scheme_chirho(
        source_declaration_chirho,
        &[TyChirho::int_chirho(), TyChirho::bool_chirho()],
    );
    assert_eq!(
        surface_chirho
            .scheme_text_chirho(&source_scheme_chirho, &source_iface_chirho)
            .unwrap(),
        "Proxy Int Bool",
    );
    assert!(
        surface_chirho
            .scheme_text_chirho(&schemes_chirho["Proxy"], &source_iface_chirho)
            .is_err(),
        "a source namesake cannot publish the compiler constructor's hidden-argument proof",
    );
}

#[test]
fn specified_source_kind_slot_is_hidden_but_its_required_argument_is_kept_chirho() {
    let ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let (surface_chirho, iface_chirho) = CheckedSurfaceChirho::from_source_chirho(
        "{-# LANGUAGE StandaloneKindSignatures, ExplicitForAll, PolyKinds, KindSignatures #-}\nmodule SpecifiedSurfaceChirho where\nimport Data.Kind (Type)\ntype SpecifiedChirho :: forall kindChirho. kindChirho -> Type\ndata SpecifiedChirho argumentChirho = MkSpecifiedChirho\n",
        &ifaces_chirho,
    );
    let declaration_chirho = iface_chirho.exports_chirho.types_chirho["SpecifiedChirho"]
        .declaration_chirho
        .as_ref()
        .unwrap();
    let scheme_chirho = application_scheme_chirho(
        declaration_chirho,
        &[
            haskelujah_typing_chirho::kind_chirho::elaboration_chirho::ElaborationStateChirho::type_universe_chirho(),
            TyChirho::int_chirho(),
        ],
    );
    assert_eq!(
        surface_chirho
            .scheme_text_chirho(&scheme_chirho, &iface_chirho)
            .unwrap(),
        "SpecifiedChirho Int",
    );
}

#[test]
fn missing_or_stub_identity_and_partial_applications_cannot_compare_as_agreement_chirho() {
    let ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let surface_chirho = CheckedSurfaceChirho::builtin_chirho(&ifaces_chirho);
    let builtin_iface_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Proxy")
        .unwrap();
    let declaration_chirho = builtin_iface_chirho.exports_chirho.types_chirho["Proxy"]
        .declaration_chirho
        .as_ref()
        .unwrap();
    let schemes_chirho = haskelujah_typing_chirho::infer_chirho::builtin_value_schemes_chirho();
    let mut stub_chirho = builtin_iface_chirho.clone();
    stub_chirho
        .exports_chirho
        .types_chirho
        .get_mut("Proxy")
        .unwrap()
        .declaration_chirho = None;
    let mut unknown_chirho = stub_chirho.clone();
    unknown_chirho.exports_chirho.types_chirho.clear();
    for weak_chirho in [&stub_chirho, &unknown_chirho] {
        assert!(
            surface_chirho
                .scheme_text_chirho(&schemes_chirho["Proxy"], weak_chirho)
                .is_err()
        );
    }
    for arguments_chirho in [
        Vec::new(),
        vec![TyChirho::int_chirho()],
        vec![
            TyChirho::int_chirho(),
            TyChirho::bool_chirho(),
            TyChirho::char_chirho(),
        ],
    ] {
        assert!(
            surface_chirho
                .scheme_text_chirho(
                    &application_scheme_chirho(declaration_chirho, &arguments_chirho),
                    builtin_iface_chirho,
                )
                .is_err(),
            "a partial or malformed internal application must not accidentally print as Proxy v0"
        );
    }
}

#[test]
fn held_conflicting_or_incomplete_telescope_handoffs_refuse_projection_chirho() {
    let ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let surface_chirho = CheckedSurfaceChirho::builtin_chirho(&ifaces_chirho);
    let declaration_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Proxy")
        .unwrap()
        .exports_chirho
        .types_chirho["Proxy"]
        .declaration_chirho
        .as_ref()
        .unwrap();
    let checked_chirho = surface_chirho
        .registry_chirho
        .declaration_schemes_chirho()
        .get_chirho(&declaration_chirho.type_key_chirho())
        .unwrap();
    let argument_chirho = TyChirho::int_chirho();
    let arguments_chirho = [&argument_chirho, &argument_chirho];
    let mut held_chirho = checked_chirho.as_ref().clone();
    held_chirho.held_chirho = true;
    let mut conflicting_chirho = checked_chirho.as_ref().clone();
    conflicting_chirho.declaration_chirho = None;
    let mut incomplete_chirho = checked_chirho.as_ref().clone();
    incomplete_chirho.reduction_arity_chirho += 1;
    let mut incorrect_visible_chirho = checked_chirho.as_ref().clone();
    incorrect_visible_chirho.visible_arity_chirho += 1;
    let mut duplicate_chirho = checked_chirho.as_ref().clone();
    duplicate_chirho.binders_chirho[1].variable_chirho =
        duplicate_chirho.binders_chirho[0].variable_chirho;
    for invalid_chirho in [
        held_chirho,
        conflicting_chirho,
        incomplete_chirho,
        incorrect_visible_chirho,
        duplicate_chirho,
    ] {
        assert!(
            project_arguments_chirho(declaration_chirho, &invalid_chirho, &arguments_chirho)
                .is_err(),
            "an invalid telescope handoff must not erase arguments to create agreement"
        );
    }
}
