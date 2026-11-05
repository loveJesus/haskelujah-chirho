// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The roles of the module's own data and newtype parameters (row 694, #31784, #31810, #31811).
//!
//! A role says how far a coercion may change a type constructor's argument: a phantom
//! argument freely, a representational one to any type with the same representation, a
//! nominal one not at all. GND's method coercion proof reads these roles, so they must never
//! be more permissive than GHC's, and a role this module cannot prove is Unknown, never
//! nominal: an unknown position blocks a coercion without claiming GHC refuses it.
//!
//! A parameter is phantom until a constructor uses it. As GHC's role inference does:
//!
//! - as a field, or under `->`, a list or a tuple, it is representational;
//! - as the argument of another local data or newtype, it takes that parameter's role;
//! - as the argument of a local type family, of a type variable's application, or in a
//!   constraint (the constructor's own context included), it is nominal;
//! - below a nominal argument every variable is nominal, phantom positions included, as GHC's
//!   `markNominal` marks every free variable there; below an Unknown one every variable is
//!   Unknown.
//!
//! Only the local declaration a constructor names has authority over its roles: a qualified
//! name is another module's unless its qualifier is this module's name, and a constructor the
//! module does not declare (an import, a synonym, a builtin) is Unknown. A constructor's
//! existential binders are not parameters. A GADT constructor, whose signature this inference
//! does not read, and a constructor whose prefix the lowering could not read make every
//! parameter at least Unknown.
//!
//! A `type role` annotation is a floor on the inferred role. The roles are a least fixed point
//! over the module's declarations, monotone in phantom < representational < Unknown < nominal.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, ConPrefixChirho, DeclChirho, TyVarChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};

/// How far a coercion may change one argument of a type constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParameterRoleChirho(u8);

impl ParameterRoleChirho {
    /// The argument is never used: any coercion may change it.
    pub const PHANTOM_CHIRHO: Self = Self(0);
    /// The argument may change to a type of the same representation.
    pub const REPRESENTATIONAL_CHIRHO: Self = Self(1);
    /// The module cannot prove the argument's role: no coercion may change it, and no
    /// refusal may claim GHC's.
    pub const UNKNOWN_CHIRHO: Self = Self(2);
    /// The argument may not change at all.
    pub const NOMINAL_CHIRHO: Self = Self(3);

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
    /// Each parameter's starting role: its annotation, raised to Unknown for a GADT.
    start_chirho: Vec<ParameterRoleChirho>,
    constructors_chirho: Vec<&'a ConDeclChirho>,
}

/// The roles of every local data and newtype declaration's visible parameters.
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
pub fn local_parameter_roles_chirho(module_chirho: &ModuleChirho) -> ParameterRolesChirho {
    let types_chirho: Vec<RoledTypeChirho<'_>> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(roled_type_chirho)
        .collect();
    let authority_chirho = LocalAuthorityChirho {
        module_name_chirho: module_chirho.name_chirho.full_name_chirho(),
        families_chirho: local_type_families_chirho(module_chirho),
    };
    let mut roles_chirho: ParameterRolesChirho = types_chirho
        .iter()
        .map(|roled_chirho| {
            (
                roled_chirho.name_chirho.clone(),
                roled_chirho.start_chirho.clone(),
            )
        })
        .collect();
    // Each round can only raise a role, and a role has four values, so the rounds are
    // bounded by three times the number of parameters plus one.
    let bound_chirho = 3 * roles_chirho.values().map(Vec::len).sum::<usize>() + 1;
    for _ in 0..bound_chirho {
        let mut changed_chirho = false;
        for roled_chirho in &types_chirho {
            let mut found_chirho = roles_chirho[&roled_chirho.name_chirho].clone();
            let mut walker_chirho = RoleWalkChirho {
                parameters_chirho: &roled_chirho.parameters_chirho,
                roles_chirho: &roles_chirho,
                authority_chirho: &authority_chirho,
                found_chirho: &mut found_chirho,
            };
            for constructor_chirho in &roled_chirho.constructors_chirho {
                walker_chirho.walk_constructor_chirho(constructor_chirho);
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

/// Every local type family's name, top-level or associated: their arguments are nominal.
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
pub fn local_type_families_chirho(module_chirho: &ModuleChirho) -> HashSet<String> {
    let mut families_chirho = HashSet::new();
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::TypeFamilyDeclChirho { name_chirho, .. } => {
                families_chirho.insert(name_chirho.text_chirho().to_string());
            }
            DeclChirho::ClassDeclChirho {
                associated_tfs_chirho,
                ..
            } => families_chirho.extend(
                associated_tfs_chirho
                    .iter()
                    .map(|family_chirho| family_chirho.name_chirho.text_chirho().to_string()),
            ),
            _ => {}
        }
    }
    families_chirho
}

/// A local data or newtype declaration's parameters, starting roles and constructors.
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
    let has_gadt_constructor_chirho = constructors_chirho
        .iter()
        .any(|constructor_chirho| matches!(constructor_chirho, ConDeclChirho::GadtChirho { .. }));
    Some(RoledTypeChirho {
        name_chirho: name_chirho.text_chirho().to_string(),
        parameters_chirho: visible_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.text_chirho().to_string())
            .collect(),
        start_chirho: visible_chirho
            .iter()
            .map(|binder_chirho| {
                let floor_chirho = binder_chirho
                    .role_annotation_chirho
                    .as_ref()
                    .and_then(|word_chirho| {
                        ParameterRoleChirho::from_annotation_chirho(word_chirho.text_chirho())
                    })
                    .unwrap_or(ParameterRoleChirho::PHANTOM_CHIRHO);
                if has_gadt_constructor_chirho {
                    floor_chirho.max(ParameterRoleChirho::UNKNOWN_CHIRHO)
                } else {
                    floor_chirho
                }
            })
            .collect(),
        constructors_chirho,
    })
}

/// What the module's own declarations prove about a constructor a type names.
struct LocalAuthorityChirho {
    /// The module's name, the only qualifier under which a name is still the module's own.
    module_name_chirho: String,
    /// The local type families, whose arguments are nominal.
    families_chirho: HashSet<String>,
}

/// One round of role inference over one declaration's constructors.
struct RoleWalkChirho<'a> {
    parameters_chirho: &'a [String],
    roles_chirho: &'a ParameterRolesChirho,
    authority_chirho: &'a LocalAuthorityChirho,
    found_chirho: &'a mut Vec<ParameterRoleChirho>,
}

impl RoleWalkChirho<'_> {
    /// Raise every parameter one constructor uses: its context's variables are nominal, its
    /// fields' are as they stand, its existential binders are not parameters, and an unread
    /// prefix makes every parameter at least Unknown.
    fn walk_constructor_chirho(&mut self, constructor_chirho: &ConDeclChirho) {
        let (fields_chirho, prefix_chirho): (Vec<&TypeChirho>, &ConPrefixChirho) =
            match constructor_chirho {
                ConDeclChirho::OrdinaryChirho {
                    fields_chirho,
                    prefix_chirho,
                    ..
                } => (
                    fields_chirho
                        .iter()
                        .map(|(_, field_chirho)| field_chirho)
                        .collect(),
                    prefix_chirho,
                ),
                ConDeclChirho::RecordChirho {
                    fields_chirho,
                    prefix_chirho,
                    ..
                } => (
                    fields_chirho
                        .iter()
                        .map(|field_chirho| &field_chirho.ty_chirho)
                        .collect(),
                    prefix_chirho,
                ),
                // Its parameters started at Unknown.
                ConDeclChirho::GadtChirho { .. } => return,
            };
        if prefix_chirho.unread_chirho {
            for found_chirho in self.found_chirho.iter_mut() {
                *found_chirho = (*found_chirho).max(ParameterRoleChirho::UNKNOWN_CHIRHO);
            }
            return;
        }
        let bound_chirho: HashSet<String> = prefix_chirho
            .existential_vars_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.text_chirho().to_string())
            .collect();
        for constraint_chirho in &prefix_chirho.context_chirho {
            self.walk_constraint_chirho(
                constraint_chirho,
                ParameterRoleChirho::NOMINAL_CHIRHO,
                &bound_chirho,
            );
        }
        for field_chirho in fields_chirho {
            self.walk_chirho(
                field_chirho,
                ParameterRoleChirho::REPRESENTATIONAL_CHIRHO,
                &bound_chirho,
            );
        }
    }

    /// The role argument `index_chirho` of `constructor_chirho` has: its local declaration's,
    /// nominal for a local type family, else Unknown.
    fn argument_role_chirho(
        &self,
        constructor_chirho: &NameChirho,
        index_chirho: usize,
    ) -> ParameterRoleChirho {
        let local_chirho = match constructor_chirho.qualifier_chirho() {
            None => true,
            Some(qualifier_chirho) => qualifier_chirho == self.authority_chirho.module_name_chirho,
        };
        let text_chirho = constructor_chirho.text_chirho();
        if !local_chirho {
            return ParameterRoleChirho::UNKNOWN_CHIRHO;
        }
        if let Some(roles_chirho) = self.roles_chirho.get(text_chirho) {
            return roles_chirho
                .get(index_chirho)
                .copied()
                .unwrap_or(ParameterRoleChirho::UNKNOWN_CHIRHO);
        }
        if self.authority_chirho.families_chirho.contains(text_chirho) {
            ParameterRoleChirho::NOMINAL_CHIRHO
        } else {
            ParameterRoleChirho::UNKNOWN_CHIRHO
        }
    }

    /// Raise the role of every parameter `type_chirho` uses where it stands at `role_chirho`.
    /// `bound_chirho` holds the variables a forall or the constructor's prefix binds.
    fn walk_chirho(
        &mut self,
        type_chirho: &TypeChirho,
        role_chirho: ParameterRoleChirho,
        bound_chirho: &HashSet<String>,
    ) {
        if role_chirho != ParameterRoleChirho::REPRESENTATIONAL_CHIRHO {
            self.mark_all_chirho(type_chirho, role_chirho, bound_chirho);
            return;
        }
        match type_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                self.raise_chirho(name_chirho.text_chirho(), role_chirho, bound_chirho)
            }
            TypeChirho::ConChirho(_)
            | TypeChirho::PromotedConChirho { .. }
            | TypeChirho::WildcardChirho { .. }
            | TypeChirho::LitChirho { .. } => {}
            TypeChirho::AppChirho { .. } => {
                let (head_chirho, arguments_chirho) = application_spine_chirho(type_chirho);
                match head_chirho {
                    TypeChirho::ConChirho(constructor_chirho) => {
                        for (index_chirho, argument_chirho) in arguments_chirho.iter().enumerate() {
                            let argument_role_chirho =
                                self.argument_role_chirho(constructor_chirho, index_chirho);
                            if !argument_role_chirho.is_phantom_chirho() {
                                self.walk_chirho(
                                    argument_chirho,
                                    argument_role_chirho,
                                    bound_chirho,
                                );
                            }
                        }
                    }
                    // A type variable's application: GHC walks the head and makes every
                    // argument nominal.
                    other_chirho => {
                        self.walk_chirho(other_chirho, role_chirho, bound_chirho);
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
                self.walk_chirho(arg_chirho, role_chirho, bound_chirho);
                self.walk_chirho(result_chirho, role_chirho, bound_chirho);
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    self.walk_chirho(element_chirho, role_chirho, bound_chirho);
                }
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                self.walk_chirho(element_chirho, role_chirho, bound_chirho)
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.walk_chirho(inner_chirho, role_chirho, bound_chirho)
            }
            TypeChirho::QualChirho {
                context_chirho: constraints_chirho,
                body_chirho,
                ..
            } => {
                for constraint_chirho in constraints_chirho {
                    self.walk_constraint_chirho(
                        constraint_chirho,
                        ParameterRoleChirho::NOMINAL_CHIRHO,
                        bound_chirho,
                    );
                }
                self.walk_chirho(body_chirho, role_chirho, bound_chirho);
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
                self.walk_chirho(body_chirho, role_chirho, &inner_chirho);
            }
            // A promoted constructor's roles are not the module's to prove.
            TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    self.walk_chirho(
                        element_chirho,
                        ParameterRoleChirho::UNKNOWN_CHIRHO,
                        bound_chirho,
                    );
                }
            }
        }
    }

    /// Every type a constraint mentions takes `role_chirho`: nominal where a coercion could
    /// reach the constraint, as a class's parameters are nominal, and Unknown below an Unknown
    /// position, which might be phantom.
    fn walk_constraint_chirho(
        &mut self,
        constraint_chirho: &ConstraintChirho,
        role_chirho: ParameterRoleChirho,
        bound_chirho: &HashSet<String>,
    ) {
        match constraint_chirho {
            ConstraintChirho::ClassChirho { args_chirho, .. } => {
                for argument_chirho in args_chirho {
                    self.mark_all_chirho(argument_chirho, role_chirho, bound_chirho);
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
                    self.walk_constraint_chirho(premise_chirho, role_chirho, &inner_chirho);
                }
                self.walk_constraint_chirho(body_chirho, role_chirho, &inner_chirho);
            }
        }
    }

    /// Raise every variable `type_chirho` mentions to `role_chirho`, below a nominal or
    /// Unknown position, whatever roles the constructors in between have.
    fn mark_all_chirho(
        &mut self,
        type_chirho: &TypeChirho,
        role_chirho: ParameterRoleChirho,
        bound_chirho: &HashSet<String>,
    ) {
        match type_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                self.raise_chirho(name_chirho.text_chirho(), role_chirho, bound_chirho)
            }
            TypeChirho::ConChirho(_)
            | TypeChirho::PromotedConChirho { .. }
            | TypeChirho::WildcardChirho { .. }
            | TypeChirho::LitChirho { .. } => {}
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                self.mark_all_chirho(fun_chirho, role_chirho, bound_chirho);
                self.mark_all_chirho(arg_chirho, role_chirho, bound_chirho);
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                self.mark_all_chirho(arg_chirho, role_chirho, bound_chirho);
                self.mark_all_chirho(result_chirho, role_chirho, bound_chirho);
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    self.mark_all_chirho(element_chirho, role_chirho, bound_chirho);
                }
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                self.mark_all_chirho(element_chirho, role_chirho, bound_chirho)
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.mark_all_chirho(inner_chirho, role_chirho, bound_chirho)
            }
            TypeChirho::QualChirho {
                context_chirho: constraints_chirho,
                body_chirho,
                ..
            } => {
                for constraint_chirho in constraints_chirho {
                    self.walk_constraint_chirho(constraint_chirho, role_chirho, bound_chirho);
                }
                self.mark_all_chirho(body_chirho, role_chirho, bound_chirho);
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
                self.mark_all_chirho(body_chirho, role_chirho, &inner_chirho);
            }
        }
    }

    /// Raise parameter `name_chirho`'s role to at least `role_chirho`, unless a forall or the
    /// constructor's prefix binds that name.
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
