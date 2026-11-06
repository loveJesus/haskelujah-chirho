// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Where a class parameter stands in a method's type (row 694, #31784).
//!
//! GND gives a newtype `N` its representation `R`'s method at the type the class gives it at
//! `N`. That is sound only when the method's type at `R` and at `N` have one representation:
//! every occurrence of the class parameter must sit where a coercion may change it. It may
//! stand directly, under `->`, a list or a tuple, as the head of an application (an
//! eta-reduced class such as Functor), or as an argument a local type's representational
//! parameter takes. Anywhere else it is blocked:
//!
//! - under a local type's nominal parameter or a local type family's argument, which GHC
//!   refuses (GHC-18872: R1, R2, and the constructor context of `ConstructorContextRoleChirho`);
//! - under a local parameter whose role is Unknown, any other named constructor (an import, a
//!   qualified name, a synonym), as an argument of a type variable's application, or in a
//!   constraint. Haskelujah refuses these as named gaps until their roles are proven; GHC
//!   accepts some of them.
//!
//! Below a blocked position every occurrence is blocked, phantom positions included, as GHC
//! makes every variable below a nominal argument nominal; the outermost position names the
//! refusal. A method that never mentions the class parameter needs no coercion of it (R4).
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use std::collections::HashSet;

use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

use super::parameter_roles_chirho::{ParameterRoleChirho, ParameterRolesChirho};

/// Where the class parameter is blocked in one method's type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlockedPositionChirho {
    /// What the class parameter stands under, as a diagnostic names it.
    pub(crate) under_chirho: String,
    /// Whether the module proves the position nominal: a local type's nominal parameter or a
    /// local type family's argument.
    pub(crate) proven_nominal_chirho: bool,
}

/// The first position in `scheme_chirho` where class `class_name_chirho`'s parameter
/// `class_var_chirho` cannot change representation; `None` when every occurrence can.
/// `families_chirho` holds the local type families, whose arguments are nominal.
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
pub(crate) fn blocked_method_position_chirho(
    scheme_chirho: &SchemeChirho,
    class_name_chirho: &str,
    class_var_chirho: TyVarChirho,
    roles_chirho: &ParameterRolesChirho,
    families_chirho: &HashSet<String>,
) -> Option<BlockedPositionChirho> {
    let class_var_ty_chirho = TyChirho::VarChirho(class_var_chirho);
    if scheme_chirho
        .vars_chirho
        .records_chirho()
        .iter()
        .filter_map(|record_chirho| record_chirho.kind_chirho.as_deref())
        .any(|kind_chirho| mentions_var_chirho(kind_chirho, class_var_chirho))
    {
        return Some(BlockedPositionChirho {
            under_chirho: "a quantified binder kind".to_owned(),
            proven_nominal_chirho: false,
        });
    }
    for predicate_chirho in &scheme_chirho.preds_chirho {
        // The class's own predicate on its parameter is the dictionary GND replaces.
        let own_predicate_chirho = predicate_chirho.class_name_chirho == class_name_chirho
            && predicate_chirho.ty_chirho == class_var_ty_chirho
            && predicate_chirho.extra_tys_chirho.is_empty();
        if own_predicate_chirho {
            continue;
        }
        let mentions_chirho = predicate_chirho
            .semantic_tys_chirho()
            .any(|ty_chirho| mentions_var_chirho(ty_chirho, class_var_chirho));
        if mentions_chirho {
            return Some(BlockedPositionChirho {
                under_chirho: format!("the constraint `{}`", predicate_chirho.class_name_chirho),
                proven_nominal_chirho: false,
            });
        }
    }
    let walker_chirho = MethodWalkChirho {
        class_var_chirho,
        roles_chirho,
        families_chirho,
    };
    walker_chirho.walk_chirho(&scheme_chirho.ty_chirho, None)
}

/// One method type's walk.
struct MethodWalkChirho<'a> {
    class_var_chirho: TyVarChirho,
    roles_chirho: &'a ParameterRolesChirho,
    families_chirho: &'a HashSet<String>,
}

impl MethodWalkChirho<'_> {
    /// The first blocked occurrence in `ty_chirho`, where `blocked_chirho` is the position
    /// that already blocks the whole of it, if any.
    fn walk_chirho(
        &self,
        ty_chirho: &TyChirho,
        blocked_chirho: Option<&BlockedPositionChirho>,
    ) -> Option<BlockedPositionChirho> {
        if let Some(blocked_chirho) = blocked_chirho {
            return mentions_var_chirho(ty_chirho, self.class_var_chirho)
                .then(|| blocked_chirho.clone());
        }
        match ty_chirho {
            TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => None,
            TyChirho::FunChirho(argument_chirho, result_chirho, _) => self
                .walk_chirho(argument_chirho, None)
                .or_else(|| self.walk_chirho(result_chirho, None)),
            TyChirho::TupleChirho(elements_chirho) => elements_chirho
                .iter()
                .find_map(|element_chirho| self.walk_chirho(element_chirho, None)),
            TyChirho::ListChirho(element_chirho) => self.walk_chirho(element_chirho, None),
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                if vars_chirho
                    .records_chirho()
                    .iter()
                    .filter_map(|record_chirho| record_chirho.kind_chirho.as_deref())
                    .any(|kind_chirho| mentions_var_chirho(kind_chirho, self.class_var_chirho))
                {
                    return Some(BlockedPositionChirho {
                        under_chirho: "a quantified binder kind".to_owned(),
                        proven_nominal_chirho: false,
                    });
                }
                self.walk_chirho(body_chirho, None)
            }
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                if context_chirho.iter().any(|constraint_chirho| {
                    mentions_var_chirho(constraint_chirho, self.class_var_chirho)
                }) {
                    return Some(BlockedPositionChirho {
                        under_chirho: "a qualified constraint".to_owned(),
                        proven_nominal_chirho: false,
                    });
                }
                self.walk_chirho(body_chirho, None)
            }
            TyChirho::AppChirho(..) => self.walk_application_chirho(ty_chirho),
        }
    }

    /// The role argument `index_chirho` of `constructor_chirho` has: its local declaration's,
    /// nominal for a local type family's, `None` when the module proves none.
    fn argument_role_chirho(
        &self,
        constructor_chirho: &str,
        index_chirho: usize,
    ) -> Option<ParameterRoleChirho> {
        match self.roles_chirho.get(constructor_chirho) {
            Some(roles_chirho) => roles_chirho.get(index_chirho).copied(),
            None => self
                .families_chirho
                .contains(constructor_chirho)
                .then_some(ParameterRoleChirho::NOMINAL_CHIRHO),
        }
    }

    /// An application at a position a coercion reaches: its arguments take the roles its head
    /// gives them.
    fn walk_application_chirho(&self, ty_chirho: &TyChirho) -> Option<BlockedPositionChirho> {
        let mut arguments_chirho = Vec::new();
        let mut head_chirho = ty_chirho;
        while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
            arguments_chirho.push(argument_chirho.as_ref());
            head_chirho = function_chirho.as_ref();
        }
        arguments_chirho.reverse();
        match head_chirho {
            TyChirho::ConChirho(constructor_chirho) => arguments_chirho
                .iter()
                .enumerate()
                .find_map(|(index_chirho, argument_chirho)| {
                    match self.argument_role_chirho(constructor_chirho, index_chirho) {
                        Some(role_chirho) if role_chirho.is_phantom_chirho() => None,
                        Some(ParameterRoleChirho::REPRESENTATIONAL_CHIRHO) => {
                            self.walk_chirho(argument_chirho, None)
                        }
                        known_chirho => {
                            let here_chirho = BlockedPositionChirho {
                                under_chirho: format!("`{constructor_chirho}`"),
                                proven_nominal_chirho: known_chirho
                                    .is_some_and(ParameterRoleChirho::is_nominal_chirho),
                            };
                            self.walk_chirho(argument_chirho, Some(&here_chirho))
                        }
                    }
                }),
            head_chirho => {
                let applies_class_var_chirho = matches!(
                    head_chirho,
                    TyChirho::VarChirho(var_chirho) if *var_chirho == self.class_var_chirho
                );
                let here_chirho = BlockedPositionChirho {
                    under_chirho: if applies_class_var_chirho {
                        "an application of the class parameter".to_string()
                    } else {
                        "a type variable's application".to_string()
                    },
                    proven_nominal_chirho: false,
                };
                self.walk_chirho(head_chirho, None).or_else(|| {
                    arguments_chirho.iter().find_map(|argument_chirho| {
                        self.walk_chirho(argument_chirho, Some(&here_chirho))
                    })
                })
            }
        }
    }
}

/// Whether `ty_chirho` mentions `var_chirho`.
fn mentions_var_chirho(ty_chirho: &TyChirho, var_chirho: TyVarChirho) -> bool {
    match ty_chirho {
        TyChirho::VarChirho(found_chirho) => *found_chirho == var_chirho,
        TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => false,
        TyChirho::AppChirho(left_chirho, right_chirho)
        | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
            mentions_var_chirho(left_chirho, var_chirho)
                || mentions_var_chirho(right_chirho, var_chirho)
        }
        TyChirho::TupleChirho(elements_chirho) => elements_chirho
            .iter()
            .any(|element_chirho| mentions_var_chirho(element_chirho, var_chirho)),
        TyChirho::ListChirho(element_chirho) => mentions_var_chirho(element_chirho, var_chirho),
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => {
            vars_chirho
                .records_chirho()
                .iter()
                .filter_map(|record_chirho| record_chirho.kind_chirho.as_deref())
                .any(|kind_chirho| mentions_var_chirho(kind_chirho, var_chirho))
                || mentions_var_chirho(body_chirho, var_chirho)
        }
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => {
            context_chirho
                .iter()
                .any(|constraint_chirho| mentions_var_chirho(constraint_chirho, var_chirho))
                || mentions_var_chirho(body_chirho, var_chirho)
        }
    }
}
