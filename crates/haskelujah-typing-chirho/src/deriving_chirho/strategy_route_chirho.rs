// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Deriving strategies: which mechanism produces each requested class (row 694).
//!
//! A written strategy is honored as written. `stock` runs the class's own generator,
//! `anyclass` writes an empty instance that takes the class defaults, and `newtype` takes the
//! representation type's instance through the GND dictionary path; an explicit `newtype`
//! never falls back to a stock instance. A class with no strategy keeps the legacy policy:
//! the stock classes run their generators, a data type's well-known builtin classes get an
//! empty instance, any other class on a newtype goes through GND, and on a data type it
//! needs DeriveAnyClass. Declaration validity rejects a `newtype` strategy on a data type
//! (GHC-10333) or without GeneralizedNewtypeDeriving (GHC-26557).
//! workflow: language-features-chirho/deriving-strategies-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DeclChirho, DerivedClassChirho, TyVarChirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{BuiltinClassScopeChirho, NameChirho};
use haskelujah_span_chirho::SpanChirho;

use super::{declaration_heads_chirho, derive_anyclass_chirho, newtype_route_chirho};

/// The classes the legacy policy derives with their own generator when no strategy is
/// written; the generators live beside the root.
const LEGACY_STOCK_CLASSES_CHIRHO: &[&str] = &[
    "Eq",
    "Ord",
    "Show",
    "Enum",
    "Bounded",
    "Read",
    "Functor",
    "Foldable",
    "Traversable",
    "Generic",
];

/// Builtin classes a data type's deriving clause gets as an empty instance.
const EMPTY_INSTANCE_CLASSES_CHIRHO: &[&str] = &["Typeable", "Data", "Lift", "NFData"];

/// How one requested class's instance is produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DerivationRouteChirho {
    /// Written here: by the class's stock generator, or, when `defaults_only_chirho`, as an
    /// empty instance that takes the class defaults.
    WrittenChirho { defaults_only_chirho: bool },
    /// The representation type's instance, through the GND dictionary path.
    RepresentationChirho,
}

/// What a module enables for deriving, and the class arities a GND head needs.
pub(super) struct DerivingPolicyChirho<'a> {
    derive_any_class_chirho: bool,
    builtin_classes_chirho: &'a BuiltinClassScopeChirho,
}

impl<'a> DerivingPolicyChirho<'a> {
    /// The policy of `module_chirho`, with the builtin classes naming proved for it.
    pub(super) fn of_module_chirho(
        module_chirho: &ModuleChirho,
        builtin_classes_chirho: &'a BuiltinClassScopeChirho,
    ) -> Self {
        Self {
            derive_any_class_chirho: module_chirho
                .extensions_chirho
                .iter()
                .any(|extension_chirho| extension_chirho == "DeriveAnyClass"),
            builtin_classes_chirho,
        }
    }

    /// How `derived_chirho` is produced for a newtype (`is_newtype_chirho`) or a data
    /// type. `None` when nothing is derived: a `newtype` strategy on a data type, which
    /// declaration validity rejects, or a class the legacy policy cannot derive.
    pub(super) fn route_chirho(
        &self,
        is_newtype_chirho: bool,
        derived_chirho: &DerivedClassChirho,
    ) -> Option<DerivationRouteChirho> {
        let class_text_chirho = derived_chirho.class_text_chirho();
        let empty_instance_chirho = EMPTY_INSTANCE_CLASSES_CHIRHO.contains(&class_text_chirho);
        match &derived_chirho.strategy_chirho {
            Some(strategy_chirho) if strategy_chirho.is_newtype_chirho() => {
                is_newtype_chirho.then_some(DerivationRouteChirho::RepresentationChirho)
            }
            Some(strategy_chirho) if strategy_chirho.is_anyclass_chirho() => {
                Some(DerivationRouteChirho::WrittenChirho {
                    defaults_only_chirho: true,
                })
            }
            // `stock`: the generator, or the empty instance a builtin class already uses.
            Some(_) => Some(DerivationRouteChirho::WrittenChirho {
                defaults_only_chirho: empty_instance_chirho,
            }),
            None if empty_instance_chirho && !is_newtype_chirho => {
                Some(DerivationRouteChirho::WrittenChirho {
                    defaults_only_chirho: true,
                })
            }
            None if LEGACY_STOCK_CLASSES_CHIRHO.contains(&class_text_chirho) => {
                Some(DerivationRouteChirho::WrittenChirho {
                    defaults_only_chirho: false,
                })
            }
            None if is_newtype_chirho => Some(DerivationRouteChirho::RepresentationChirho),
            None => self
                .derive_any_class_chirho
                .then_some(DerivationRouteChirho::WrittenChirho {
                    defaults_only_chirho: true,
                }),
        }
    }

    /// How many trailing type variables a GND head drops for `class_chirho`: the kind
    /// arity of the builtin class's last parameter, as naming proved the class. Any other
    /// class keeps the full head.
    pub(super) fn eta_arity_chirho(&self, class_chirho: &NameChirho) -> usize {
        self.builtin_classes_chirho
            .class_chirho(&class_chirho.full_name_chirho())
            .and_then(|builtin_chirho| builtin_chirho.parameter_arities_chirho.last())
            .map_or(0, |arity_chirho| usize::from(*arity_chirho))
    }
}

/// A local `data` or `newtype` declaration whose classes are being derived.
pub(super) struct DeclaredTypeChirho<'a> {
    pub(super) name_chirho: &'a NameChirho,
    /// The binders an instance head applies.
    pub(super) type_vars_chirho: &'a [TyVarChirho],
    pub(super) constructors_chirho: &'a [ConDeclChirho],
    /// The newtype's constructor; `None` for a data type.
    pub(super) newtype_constructor_chirho: Option<&'a ConDeclChirho>,
    pub(super) span_chirho: SpanChirho,
}

/// Derive every class a declaration's deriving clauses request.
/// workflow: language-features-chirho/deriving-strategies-chirho
pub(super) fn derive_declared_classes_chirho(
    declared_chirho: &DeclaredTypeChirho<'_>,
    deriving_chirho: &[DerivedClassChirho],
    policy_chirho: &DerivingPolicyChirho<'_>,
    instances_chirho: &mut Vec<DeclChirho>,
    warnings_chirho: &mut Vec<String>,
) {
    for derived_chirho in deriving_chirho {
        derive_class_chirho(
            declared_chirho,
            derived_chirho,
            policy_chirho,
            instances_chirho,
            warnings_chirho,
        );
    }
}

/// Derive one class for a declaration, by its route.
/// workflow: language-features-chirho/deriving-strategies-chirho
pub(super) fn derive_class_chirho(
    declared_chirho: &DeclaredTypeChirho<'_>,
    derived_chirho: &DerivedClassChirho,
    policy_chirho: &DerivingPolicyChirho<'_>,
    instances_chirho: &mut Vec<DeclChirho>,
    warnings_chirho: &mut Vec<String>,
) {
    let class_text_chirho = derived_chirho.class_text_chirho();
    let type_text_chirho = declared_chirho.name_chirho.text_chirho();
    let route_chirho = policy_chirho.route_chirho(
        declared_chirho.newtype_constructor_chirho.is_some(),
        derived_chirho,
    );
    match route_chirho {
        Some(DerivationRouteChirho::WrittenChirho {
            defaults_only_chirho: false,
        }) => {
            let derived_by_stock_chirho = declaration_heads_chirho::derive_stock_class_chirho(
                class_text_chirho,
                declared_chirho.name_chirho,
                declared_chirho.type_vars_chirho,
                declared_chirho.constructors_chirho,
                declared_chirho.span_chirho,
                instances_chirho,
                warnings_chirho,
            );
            if !derived_by_stock_chirho {
                warnings_chirho.push(format!(
                    "deriving {class_text_chirho} not yet supported for {type_text_chirho}"
                ));
            }
        }
        Some(DerivationRouteChirho::WrittenChirho {
            defaults_only_chirho: true,
        }) => instances_chirho.push(derive_anyclass_chirho(
            declared_chirho.name_chirho,
            declared_chirho.type_vars_chirho,
            &derived_chirho.class_chirho,
            declared_chirho.span_chirho,
        )),
        Some(DerivationRouteChirho::RepresentationChirho) => {
            let Some(constructor_chirho) = declared_chirho.newtype_constructor_chirho else {
                return;
            };
            match newtype_route_chirho::derive_newtype_instance_chirho(
                declared_chirho.name_chirho,
                declared_chirho.type_vars_chirho,
                constructor_chirho,
                &derived_chirho.class_chirho,
                policy_chirho.eta_arity_chirho(&derived_chirho.class_chirho),
            ) {
                Some(instance_chirho) => instances_chirho.push(instance_chirho),
                None => warnings_chirho.push(format!(
                    "deriving newtype {class_text_chirho} for {type_text_chirho}: the representation type does not end in the variables the class drops"
                )),
            }
        }
        // A written `newtype` strategy on a data type: declaration validity reports it.
        None if derived_chirho.strategy_chirho.is_some() => {}
        None => warnings_chirho.push(format!(
            "deriving {class_text_chirho} not yet supported for {type_text_chirho}"
        )),
    }
}

/// Every local newtype that takes a class through the newtype strategy, with those
/// classes: its deriving clauses as routed, and its `deriving newtype instance`
/// declarations. The dictionary pass gives only these classes the representation's
/// dictionary, and Core erases only these newtypes' constructors.
/// workflow: language-features-chirho/deriving-strategies-chirho
pub fn newtype_strategy_classes_chirho(
    module_chirho: &ModuleChirho,
) -> HashMap<String, Vec<String>> {
    // Routing does not depend on class arities.
    let builtin_classes_chirho = BuiltinClassScopeChirho::default();
    let policy_chirho =
        DerivingPolicyChirho::of_module_chirho(module_chirho, &builtin_classes_chirho);
    let mut classes_chirho: HashMap<String, Vec<String>> = HashMap::new();
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                deriving_chirho,
                ..
            } => {
                for derived_chirho in deriving_chirho {
                    if policy_chirho.route_chirho(true, derived_chirho)
                        == Some(DerivationRouteChirho::RepresentationChirho)
                    {
                        classes_chirho
                            .entry(name_chirho.text_chirho().to_string())
                            .or_default()
                            .push(derived_chirho.class_text_chirho().to_string());
                    }
                }
            }
            DeclChirho::StandaloneDerivingDeclChirho {
                strategy_chirho: Some(strategy_chirho),
                class_chirho,
                types_chirho,
                ..
            } if strategy_chirho.is_newtype_chirho() => {
                let Some(target_chirho) = types_chirho
                    .first()
                    .and_then(declaration_heads_chirho::target_name_chirho)
                else {
                    continue;
                };
                if declares_newtype_chirho(module_chirho, target_chirho) {
                    classes_chirho
                        .entry(target_chirho.to_string())
                        .or_default()
                        .push(class_chirho.text_chirho().to_string());
                }
            }
            _ => {}
        }
    }
    classes_chirho
}

/// Whether `module_chirho` declares a newtype named `name_chirho`.
fn declares_newtype_chirho(module_chirho: &ModuleChirho, name_chirho: &str) -> bool {
    module_chirho.decls_chirho.iter().any(|decl_chirho| {
        matches!(
            decl_chirho,
            DeclChirho::NewtypeDeclChirho { name_chirho: declared_chirho, .. }
                if declared_chirho.text_chirho() == name_chirho
        )
    })
}
