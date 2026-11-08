// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Deriving strategies: which mechanism produces each requested class (row 694).
//!
//! A written strategy is honored as written. `stock` runs the class's own generator,
//! `anyclass` writes an empty instance that takes the class defaults, and `newtype` takes the
//! representation type's instance through the GND dictionary path; an explicit `newtype`
//! never falls back to a stock instance. A class with no strategy keeps the legacy policy:
//! the stock classes run their generators, a data type's builtin classes the compiler does not
//! generate get an empty instance, DeriveAnyClass takes any other class, then GND on a
//! newtype. Which stock class a written class names is `stock_identity_chirho.rs`'s judgment
//! from the builtin classes naming proved (K01), never its spelling, and a generated instance
//! names its class as written. A class applied to arguments derives as an anyclass instance
//! over the full head `C args (T vs)`, or through GND for the module's own methodless class
//! whose last parameter's kind arity the module proves (`class_shapes_chirho.rs`, T3955):
//! `C args rep' => C args N'`, every argument kept and both heads eta-reduced by that arity.
//! The stock generators write one head type, so declaration validity refuses the rest.
//! A request K01 proves names the builtin `Typeable` takes no route at all: GHC ignores it
//! (`ignores_derived_class_chirho`), so whatever its strategy it mints no instance and asks
//! for no representation dictionary (#32106).
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DeclChirho, DerivedClassChirho, TyVarChirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{BuiltinClassScopeChirho, NameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

use super::class_shapes_chirho::LocalClassShapesChirho;

use super::extension_policy_chirho::DerivingExtensionsChirho;
use super::stock_identity_chirho::{StockIdentityChirho, spell_generated_class_chirho};
use super::{declaration_heads_chirho, derive_anyclass_chirho, newtype_route_chirho};

/// The stock classes the legacy policy derives with their own generator when no strategy is
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

/// Stock derivable builtin classes the compiler writes as an empty instance.
const EMPTY_INSTANCE_CLASSES_CHIRHO: &[&str] = &["Typeable", "Data", "Lift"];

/// How one requested class's instance is produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DerivationRouteChirho {
    /// Written here: by the class's stock generator, or, when `defaults_only_chirho`, as an
    /// empty instance that takes the class defaults.
    WrittenChirho { defaults_only_chirho: bool },
    /// The representation type's instance, through the GND dictionary path.
    RepresentationChirho,
}

/// What a module enables for deriving, and which builtin classes its spellings name.
pub(super) struct DerivingPolicyChirho<'a> {
    extensions_chirho: DerivingExtensionsChirho,
    identity_chirho: StockIdentityChirho<'a>,
    /// The module's own classes, as GND with written class arguments reads them.
    class_shapes_chirho: LocalClassShapesChirho,
}

impl<'a> DerivingPolicyChirho<'a> {
    /// The policy of `module_chirho`, with the builtin classes naming proved for it.
    pub(super) fn of_module_chirho(
        module_chirho: &ModuleChirho,
        builtin_classes_chirho: &'a BuiltinClassScopeChirho,
    ) -> Self {
        Self::with_type_scope_chirho(module_chirho, builtin_classes_chirho, None)
    }

    pub(super) fn with_type_scope_chirho(
        module_chirho: &ModuleChirho,
        builtin_classes_chirho: &'a BuiltinClassScopeChirho,
        type_scope_chirho: Option<
            &'a haskelujah_ast_chirho::name_chirho::TypeConstructorScopeChirho,
        >,
    ) -> Self {
        let identity_chirho = StockIdentityChirho::with_type_scope_chirho(
            module_chirho,
            builtin_classes_chirho,
            type_scope_chirho,
        );
        Self {
            extensions_chirho: DerivingExtensionsChirho::of_module_chirho(module_chirho),
            class_shapes_chirho: LocalClassShapesChirho::of_module_chirho(
                module_chirho,
                &identity_chirho,
            ),
            identity_chirho,
        }
    }

    /// The stock class whose generator may derive `derived_chirho`, by its strategy.
    pub(super) fn stock_class_for_chirho(
        &self,
        derived_chirho: &DerivedClassChirho,
    ) -> Option<&'static str> {
        self.identity_chirho.stock_class_for_chirho(derived_chirho)
    }

    /// The stock class the legacy route takes for `class_chirho` under an unwritten strategy.
    pub(super) fn legacy_stock_class_chirho(
        &self,
        class_chirho: &NameChirho,
    ) -> Option<&'static str> {
        self.identity_chirho.legacy_stock_class_chirho(class_chirho)
    }

    /// Whether an unwritten strategy on a newtype takes anyclass over GND for this class:
    /// GHC 9.14.1 defaults to DeriveAnyClass when both are on, with warning GHC-20042
    /// (root's D1).
    fn anyclass_over_newtype_chirho(&self, derived_chirho: &DerivedClassChirho) -> bool {
        derived_chirho.strategy_chirho.is_none()
            && self.extensions_chirho.any_class_chirho
            && self.extensions_chirho.newtype_chirho
            && self
                .legacy_stock_class_chirho(&derived_chirho.class_chirho)
                .is_none()
    }

    /// How `derived_chirho` is produced for a newtype (`is_newtype_chirho`) or a data
    /// type. `None` when nothing is derived: a request GHC ignores (the builtin `Typeable`),
    /// a strategy declaration validity refuses, a class applied to arguments outside the
    /// anyclass route, or a class the legacy policy cannot derive.
    pub(super) fn route_chirho(
        &self,
        is_newtype_chirho: bool,
        derived_chirho: &DerivedClassChirho,
    ) -> Option<DerivationRouteChirho> {
        if self
            .identity_chirho
            .ignores_derived_class_chirho(&derived_chirho.class_chirho)
        {
            return None;
        }
        let stock_class_chirho = self.stock_class_for_chirho(derived_chirho);
        let empty_instance_chirho = stock_class_chirho
            .is_some_and(|class_chirho| EMPTY_INSTANCE_CLASSES_CHIRHO.contains(&class_chirho));
        let route_chirho = match &derived_chirho.strategy_chirho {
            Some(strategy_chirho) if strategy_chirho.is_newtype_chirho() => {
                is_newtype_chirho.then_some(DerivationRouteChirho::RepresentationChirho)
            }
            Some(strategy_chirho) if strategy_chirho.is_anyclass_chirho() => {
                Some(DerivationRouteChirho::WrittenChirho {
                    defaults_only_chirho: true,
                })
            }
            // `stock`: the proven class's generator, or the empty instance a builtin class
            // already uses; nothing for a class K01 does not prove stock derivable.
            Some(_) => stock_class_chirho.map(|_| DerivationRouteChirho::WrittenChirho {
                defaults_only_chirho: empty_instance_chirho,
            }),
            None if empty_instance_chirho && !is_newtype_chirho => {
                Some(DerivationRouteChirho::WrittenChirho {
                    defaults_only_chirho: true,
                })
            }
            None if stock_class_chirho.is_some_and(|class_chirho| {
                LEGACY_STOCK_CLASSES_CHIRHO.contains(&class_chirho)
            }) =>
            {
                Some(DerivationRouteChirho::WrittenChirho {
                    defaults_only_chirho: false,
                })
            }
            // DeriveAnyClass takes precedence over GND (root's D1; D2 without it is GND).
            None if self.extensions_chirho.any_class_chirho => {
                Some(DerivationRouteChirho::WrittenChirho {
                    defaults_only_chirho: true,
                })
            }
            // Without GND declaration validity refuses it (GHC-82023, root's C1).
            None if is_newtype_chirho => Some(DerivationRouteChirho::RepresentationChirho),
            None => None,
        };
        route_chirho.filter(|route_chirho| {
            !derived_chirho.has_class_arguments_chirho()
                || *route_chirho
                    == DerivationRouteChirho::WrittenChirho {
                        defaults_only_chirho: true,
                    }
                || (*route_chirho == DerivationRouteChirho::RepresentationChirho
                    && self
                        .class_shapes_chirho
                        .class_argument_arity_chirho(derived_chirho, &self.identity_chirho)
                        .is_some())
        })
    }

    /// How many trailing type variables the GND head for `derived_chirho` drops: the proven
    /// arity of the module's own class's last parameter when the class takes written
    /// arguments, else `eta_arity_chirho`.
    pub(super) fn representation_eta_arity_chirho(
        &self,
        derived_chirho: &DerivedClassChirho,
    ) -> usize {
        if derived_chirho.has_class_arguments_chirho() {
            self.class_shapes_chirho
                .class_argument_arity_chirho(derived_chirho, &self.identity_chirho)
                .unwrap_or(0)
        } else {
            self.eta_arity_chirho(&derived_chirho.class_chirho)
        }
    }

    /// How many trailing type variables a GND head drops for `class_chirho`: the outer arrows
    /// of the builtin class's one parameter kind, as naming proved the class (`Functor` drops
    /// one, `MonadTrans` two). Any other class keeps the full head, as does a builtin schema
    /// with a hidden binder or a kind outside `Type` and its arrows.
    pub(super) fn eta_arity_chirho(&self, class_chirho: &NameChirho) -> usize {
        self.identity_chirho
            .builtin_classes_chirho()
            .class_chirho(&class_chirho.full_name_chirho())
            .and_then(|builtin_chirho| {
                builtin_chirho
                    .schema_chirho()
                    .single_parameter_eta_arity_chirho()
            })
            .unwrap_or(0)
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
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
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
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
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
            // The stock route exists only for a class the judgment names.
            let Some(stock_class_chirho) = policy_chirho.stock_class_for_chirho(derived_chirho)
            else {
                return;
            };
            let first_generated_chirho = instances_chirho.len();
            let derived_by_stock_chirho = declaration_heads_chirho::derive_stock_class_chirho(
                stock_class_chirho,
                declared_chirho.name_chirho,
                declared_chirho.type_vars_chirho,
                declared_chirho.constructors_chirho,
                declared_chirho.span_chirho,
                instances_chirho,
                warnings_chirho,
            );
            spell_generated_class_chirho(
                &mut instances_chirho[first_generated_chirho..],
                stock_class_chirho,
                &derived_chirho.class_chirho,
            );
            if !derived_by_stock_chirho {
                warnings_chirho.push(format!(
                    "deriving {class_text_chirho} not yet supported for {type_text_chirho}"
                ));
            }
        }
        Some(DerivationRouteChirho::WrittenChirho {
            defaults_only_chirho: true,
        }) => {
            if declared_chirho.newtype_constructor_chirho.is_some()
                && policy_chirho.anyclass_over_newtype_chirho(derived_chirho)
            {
                warnings_chirho.push(format!(
                    "Both DeriveAnyClass and GeneralizedNewtypeDeriving are enabled: defaulting to the DeriveAnyClass strategy for {class_text_chirho} on {type_text_chirho} (GHC-20042)"
                ));
            }
            instances_chirho.push(derive_anyclass_chirho(
                declared_chirho.name_chirho,
                declared_chirho.type_vars_chirho,
                &derived_chirho.class_chirho,
                &derived_chirho.class_arguments_chirho,
                declared_chirho.span_chirho,
            ));
        }
        Some(DerivationRouteChirho::RepresentationChirho) => {
            let Some(constructor_chirho) = declared_chirho.newtype_constructor_chirho else {
                return;
            };
            match newtype_route_chirho::derive_newtype_instance_chirho(
                declared_chirho.name_chirho,
                declared_chirho.type_vars_chirho,
                constructor_chirho,
                &derived_chirho.class_chirho,
                &derived_chirho.class_arguments_chirho,
                policy_chirho.representation_eta_arity_chirho(derived_chirho),
            ) {
                Some(instance_chirho) => instances_chirho.push(instance_chirho),
                None => warnings_chirho.push(format!(
                    "deriving newtype {class_text_chirho} for {type_text_chirho}: the representation type does not end in the variables the class drops"
                )),
            }
        }
        // A written strategy no route takes, or a class applied to arguments outside the
        // anyclass route: declaration validity reports it. A request GHC ignores derives
        // nothing and warns nothing.
        None if derived_chirho.strategy_chirho.is_some()
            || derived_chirho.has_class_arguments_chirho()
            || policy_chirho
                .identity_chirho
                .ignores_derived_class_chirho(&derived_chirho.class_chirho) => {}
        None => warnings_chirho.push(format!(
            "deriving {class_text_chirho} not yet supported for {type_text_chirho}"
        )),
    }
}

/// One instance the module's declarations derive through the newtype strategy: the newtype,
/// and the class as written, whose span a refusal points at (row 694).
#[derive(Debug, Clone, PartialEq)]
pub struct GndRequestChirho {
    /// The local newtype's name.
    pub newtype_chirho: String,
    /// The class, as the deriving clause or standalone declaration writes it.
    pub class_chirho: NameChirho,
    /// The class's written arguments before the newtype (`a` in `MonadReader a`).
    pub class_arguments_chirho: Vec<TypeChirho>,
}

/// Every instance the module's declarations derive through the newtype strategy, in
/// declaration order: its deriving clauses as routed with the builtin classes naming proved
/// for the module, the same routing the deriving pass used, and its `deriving newtype
/// instance` declarations, except a request GHC ignores. GND's coercion proof checks exactly
/// these, and the dictionary pass gives only these the representation's dictionary.
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
pub fn newtype_strategy_requests_chirho(
    module_chirho: &ModuleChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
) -> Vec<GndRequestChirho> {
    let policy_chirho =
        DerivingPolicyChirho::of_module_chirho(module_chirho, builtin_classes_chirho);
    let mut requests_chirho = Vec::new();
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
                        requests_chirho.push(GndRequestChirho {
                            newtype_chirho: name_chirho.text_chirho().to_string(),
                            class_chirho: derived_chirho.class_chirho.clone(),
                            class_arguments_chirho: derived_chirho.class_arguments_chirho.clone(),
                        });
                    }
                }
            }
            DeclChirho::StandaloneDerivingDeclChirho {
                strategy_chirho: Some(strategy_chirho),
                class_chirho,
                types_chirho,
                ..
            } if strategy_chirho.is_newtype_chirho() => {
                // The derived type is the head's last type. A class applied to arguments
                // (`deriving newtype instance C Int N`) is refused by declaration validity
                // until the GND dictionary keys every head type, and a request GHC ignores
                // (the builtin `Typeable`) asks for no dictionary.
                if types_chirho.len() > 1
                    || policy_chirho
                        .identity_chirho
                        .ignores_derived_class_chirho(class_chirho)
                {
                    continue;
                }
                let Some(target_chirho) = types_chirho
                    .last()
                    .and_then(declaration_heads_chirho::target_name_chirho)
                else {
                    continue;
                };
                if declares_newtype_chirho(module_chirho, target_chirho) {
                    requests_chirho.push(GndRequestChirho {
                        newtype_chirho: target_chirho.to_string(),
                        class_chirho: class_chirho.clone(),
                        class_arguments_chirho: Vec::new(),
                    });
                }
            }
            _ => {}
        }
    }
    requests_chirho
}

/// Each local newtype's classes from `newtype_strategy_requests_chirho`, by newtype name.
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
pub fn newtype_strategy_classes_chirho(
    module_chirho: &ModuleChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
) -> HashMap<String, Vec<String>> {
    let mut classes_chirho: HashMap<String, Vec<String>> = HashMap::new();
    for request_chirho in newtype_strategy_requests_chirho(module_chirho, builtin_classes_chirho) {
        classes_chirho
            .entry(request_chirho.newtype_chirho)
            .or_default()
            .push(request_chirho.class_chirho.text_chirho().to_string());
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
