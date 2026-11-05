// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The roles of the module's own data and newtype parameters (row 694, #31784).
//!
//! A role says how far a coercion may change a type constructor's argument: a phantom
//! argument freely, a representational one to any type with the same representation, a
//! nominal one not at all. GND's method coercion proof reads these roles, so they must never
//! be more permissive than GHC's. A parameter is phantom until a constructor field uses it:
//!
//! - as the field itself, or under `->`, a list or a tuple, it is representational;
//! - as an argument of another local data or newtype, it takes that parameter's role;
//! - under a type family, any other named constructor (whose roles this module cannot prove),
//!   a type variable's application, a constraint or a promoted list, it is nominal.
//!
//! A `type role` annotation can only make a parameter more restrictive here: an annotated role
//! is a floor on the inferred one. A type with a GADT constructor, whose result indices GHC
//! makes nominal, reads every parameter as nominal. The roles are a least fixed point over the
//! module's declarations, monotone in the order phantom < representational < nominal.
//! workflow: language-features-chirho/deriving-strategies-chirho

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, TyVarChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};

/// How far a coercion may change one argument of a type constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParameterRoleChirho(u8);

impl ParameterRoleChirho {
    /// The argument is never used: any coercion may change it.
    pub const PHANTOM_CHIRHO: Self = Self(0);
    /// The argument may change to a type of the same representation.
    pub const REPRESENTATIONAL_CHIRHO: Self = Self(1);
    /// The argument may not change at all.
    pub const NOMINAL_CHIRHO: Self = Self(2);

    /// The role a `type role` annotation writes; `None` for `_` (infer it) or a word that
    /// names no role.
    pub fn from_annotation_chirho(word_chirho: &str) -> Option<Self> {
        match word_chirho {
            "phantom" => Some(Self::PHANTOM_CHIRHO),
            "representational" => Some(Self::REPRESENTATIONAL_CHIRHO),
            "nominal" => Some(Self::NOMINAL_CHIRHO),
            _ => None,
        }
    }

    /// Whether this is the nominal role.
    pub fn is_nominal_chirho(self) -> bool {
        self == Self::NOMINAL_CHIRHO
    }

    /// Whether this is the phantom role.
    pub fn is_phantom_chirho(self) -> bool {
        self == Self::PHANTOM_CHIRHO
    }
}

/// The roles of each local data and newtype declaration's visible parameters, by type name.
pub type ParameterRolesChirho = HashMap<String, Vec<ParameterRoleChirho>>;

/// One local data or newtype declaration, as role inference reads it.
struct RoledTypeChirho<'a> {
    name_chirho: String,
    parameters_chirho: Vec<String>,
    /// Each annotated role, a floor on the inferred one.
    floors_chirho: Vec<ParameterRoleChirho>,
    fields_chirho: Vec<&'a TypeChirho>,
    /// A GADT constructor makes every parameter nominal.
    has_gadt_constructor_chirho: bool,
}

/// The roles of every local data and newtype declaration's visible parameters.
/// workflow: language-features-chirho/deriving-strategies-chirho
pub fn local_parameter_roles_chirho(module_chirho: &ModuleChirho) -> ParameterRolesChirho {
    let types_chirho: Vec<RoledTypeChirho<'_>> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(roled_type_chirho)
        .collect();
    let mut roles_chirho: ParameterRolesChirho = types_chirho
        .iter()
        .map(|roled_chirho| {
            let start_chirho = if roled_chirho.has_gadt_constructor_chirho {
                vec![ParameterRoleChirho::NOMINAL_CHIRHO; roled_chirho.parameters_chirho.len()]
            } else {
                roled_chirho.floors_chirho.clone()
            };
            (roled_chirho.name_chirho.clone(), start_chirho)
        })
        .collect();
    // Each round can only raise a role, and a role has three values, so the rounds are
    // bounded by twice the number of parameters plus one.
    let bound_chirho = 2 * roles_chirho.values().map(Vec::len).sum::<usize>() + 1;
    for _ in 0..bound_chirho {
        let mut changed_chirho = false;
        for roled_chirho in &types_chirho {
            let mut found_chirho = roles_chirho[&roled_chirho.name_chirho].clone();
            let mut walker_chirho = RoleWalkChirho {
                parameters_chirho: &roled_chirho.parameters_chirho,
                roles_chirho: &roles_chirho,
                found_chirho: &mut found_chirho,
            };
            for field_chirho in &roled_chirho.fields_chirho {
                walker_chirho.walk_chirho(
                    field_chirho,
                    ParameterRoleChirho::REPRESENTATIONAL_CHIRHO,
                    &HashSet::new(),
                );
            }
            if found_chirho != roles_chirho[&roled_chirho.name_chirho] {
                roles_chirho.insert(roled_chirho.name_chirho.clone(), found_chirho);
                changed_chirho = true;
            }
        }
        if !changed_chirho {
            break;
        }
    }
    roles_chirho
}

/// A local data or newtype declaration's parameters, annotation floors and field types.
fn roled_type_chirho(decl_chirho: &DeclChirho) -> Option<RoledTypeChirho<'_>> {
    let (name_chirho, type_vars_chirho, constructors_chirho): (_, _, Vec<&ConDeclChirho>) =
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                ..
            } => (
                name_chirho,
                type_vars_chirho,
                constructors_chirho.iter().collect(),
            ),
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructor_chirho,
                ..
            } => (name_chirho, type_vars_chirho, vec![constructor_chirho]),
            _ => return None,
        };
    let visible_chirho = TyVarChirho::visible_binders_chirho(type_vars_chirho);
    let mut fields_chirho = Vec::new();
    let mut has_gadt_constructor_chirho = false;
    for constructor_chirho in constructors_chirho {
        match constructor_chirho {
            ConDeclChirho::OrdinaryChirho {
                fields_chirho: ordinary_chirho,
                ..
            } => fields_chirho.extend(ordinary_chirho.iter().map(|(_, field_chirho)| field_chirho)),
            ConDeclChirho::RecordChirho {
                fields_chirho: record_chirho,
                ..
            } => fields_chirho.extend(
                record_chirho
                    .iter()
                    .map(|field_chirho| &field_chirho.ty_chirho),
            ),
            ConDeclChirho::GadtChirho { .. } => has_gadt_constructor_chirho = true,
        }
    }
    Some(RoledTypeChirho {
        name_chirho: name_chirho.text_chirho().to_string(),
        parameters_chirho: visible_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.text_chirho().to_string())
            .collect(),
        floors_chirho: visible_chirho
            .iter()
            .map(|binder_chirho| {
                binder_chirho
                    .role_annotation_chirho
                    .as_ref()
                    .and_then(|word_chirho| {
                        ParameterRoleChirho::from_annotation_chirho(word_chirho.text_chirho())
                    })
                    .unwrap_or(ParameterRoleChirho::PHANTOM_CHIRHO)
            })
            .collect(),
        fields_chirho,
        has_gadt_constructor_chirho,
    })
}

/// One round of role inference over one declaration's fields.
struct RoleWalkChirho<'a> {
    parameters_chirho: &'a [String],
    roles_chirho: &'a ParameterRolesChirho,
    found_chirho: &'a mut Vec<ParameterRoleChirho>,
}

impl RoleWalkChirho<'_> {
    /// Raise the role of every parameter `type_chirho` uses, where it stands under
    /// `context_chirho`. `bound_chirho` holds the variables a forall in the field binds.
    fn walk_chirho(
        &mut self,
        type_chirho: &TypeChirho,
        context_chirho: ParameterRoleChirho,
        bound_chirho: &HashSet<String>,
    ) {
        match type_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                self.raise_chirho(name_chirho.text_chirho(), context_chirho, bound_chirho)
            }
            TypeChirho::ConChirho(_)
            | TypeChirho::PromotedConChirho { .. }
            | TypeChirho::WildcardChirho { .. }
            | TypeChirho::LitChirho { .. } => {}
            TypeChirho::AppChirho { .. } => {
                let (head_chirho, arguments_chirho) = application_spine_chirho(type_chirho);
                match head_chirho {
                    TypeChirho::ConChirho(constructor_chirho) => {
                        let roles_table_chirho = self.roles_chirho;
                        let argument_roles_chirho =
                            roles_table_chirho.get(constructor_chirho.text_chirho());
                        for (index_chirho, argument_chirho) in arguments_chirho.iter().enumerate() {
                            let role_chirho = argument_roles_chirho
                                .and_then(|roles_chirho| roles_chirho.get(index_chirho))
                                .copied()
                                .unwrap_or(ParameterRoleChirho::NOMINAL_CHIRHO);
                            if !role_chirho.is_phantom_chirho() {
                                self.walk_chirho(
                                    argument_chirho,
                                    role_chirho.max(context_chirho),
                                    bound_chirho,
                                );
                            }
                        }
                    }
                    other_chirho => {
                        self.walk_chirho(other_chirho, context_chirho, bound_chirho);
                        for argument_chirho in arguments_chirho {
                            self.walk_chirho(
                                argument_chirho,
                                ParameterRoleChirho::NOMINAL_CHIRHO,
                                bound_chirho,
                            );
                        }
                    }
                }
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                self.walk_chirho(arg_chirho, context_chirho, bound_chirho);
                self.walk_chirho(result_chirho, context_chirho, bound_chirho);
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    self.walk_chirho(element_chirho, context_chirho, bound_chirho);
                }
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                self.walk_chirho(element_chirho, context_chirho, bound_chirho)
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.walk_chirho(inner_chirho, context_chirho, bound_chirho)
            }
            TypeChirho::QualChirho {
                context_chirho: constraints_chirho,
                body_chirho,
                ..
            } => {
                for constraint_chirho in constraints_chirho {
                    self.walk_constraint_chirho(constraint_chirho, bound_chirho);
                }
                self.walk_chirho(body_chirho, context_chirho, bound_chirho);
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            }
            | TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                let mut inner_chirho = bound_chirho.clone();
                inner_chirho.extend(
                    vars_chirho
                        .iter()
                        .map(|binder_chirho| binder_chirho.text_chirho().to_string()),
                );
                self.walk_chirho(body_chirho, context_chirho, &inner_chirho);
            }
            TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    self.walk_chirho(
                        element_chirho,
                        ParameterRoleChirho::NOMINAL_CHIRHO,
                        bound_chirho,
                    );
                }
            }
        }
    }

    /// Every type a constraint mentions is nominal.
    fn walk_constraint_chirho(
        &mut self,
        constraint_chirho: &ConstraintChirho,
        bound_chirho: &HashSet<String>,
    ) {
        match constraint_chirho {
            ConstraintChirho::ClassChirho { args_chirho, .. } => {
                for argument_chirho in args_chirho {
                    self.walk_chirho(
                        argument_chirho,
                        ParameterRoleChirho::NOMINAL_CHIRHO,
                        bound_chirho,
                    );
                }
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => {
                let mut inner_chirho = bound_chirho.clone();
                inner_chirho.extend(
                    vars_chirho
                        .iter()
                        .map(|binder_chirho| binder_chirho.text_chirho().to_string()),
                );
                for premise_chirho in context_chirho {
                    self.walk_constraint_chirho(premise_chirho, &inner_chirho);
                }
                self.walk_constraint_chirho(body_chirho, &inner_chirho);
            }
        }
    }

    /// Raise parameter `name_chirho`'s role to at least `role_chirho`, unless a forall in
    /// the field binds that name.
    fn raise_chirho(
        &mut self,
        name_chirho: &str,
        role_chirho: ParameterRoleChirho,
        bound_chirho: &HashSet<String>,
    ) {
        if bound_chirho.contains(name_chirho) {
            return;
        }
        if let Some(index_chirho) = self
            .parameters_chirho
            .iter()
            .position(|parameter_chirho| parameter_chirho == name_chirho)
        {
            let current_chirho = &mut self.found_chirho[index_chirho];
            *current_chirho = (*current_chirho).max(role_chirho);
        }
    }
}

/// An application's head and its arguments in order, through parentheses.
fn application_spine_chirho(type_chirho: &TypeChirho) -> (&TypeChirho, Vec<&TypeChirho>) {
    let mut arguments_chirho = Vec::new();
    let mut head_chirho = type_chirho;
    loop {
        match head_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                arguments_chirho.push(arg_chirho.as_ref());
                head_chirho = fun_chirho.as_ref();
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => head_chirho = inner_chirho.as_ref(),
            _ => break,
        }
    }
    arguments_chirho.reverse();
    (head_chirho, arguments_chirho)
}
