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
//! - under a local type's nominal parameter, which GHC refuses (GHC-18872, R1 and R2);
//! - under any other named constructor, a family or a synonym, whose roles this module cannot
//!   prove, as an argument of a type variable's application, or in a constraint. Haskelujah
//!   refuses these until a proven role table exists; GHC accepts some of them.
//!
//! A method that never mentions the class parameter needs no coercion of it (R4).
//! workflow: language-features-chirho/deriving-strategies-chirho

use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

use super::parameter_roles_chirho::{ParameterRoleChirho, ParameterRolesChirho};

/// Where the class parameter is blocked in one method's type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlockedPositionChirho {
    /// What the class parameter stands under, as a diagnostic names it.
    pub(crate) under_chirho: String,
    /// Whether the module proves the position nominal: a local type's nominal parameter.
    pub(crate) proven_nominal_chirho: bool,
}

/// The first position in `scheme_chirho` where class `class_name_chirho`'s parameter
/// `class_var_chirho` cannot change representation; `None` when every occurrence can.
/// workflow: language-features-chirho/deriving-strategies-chirho
pub(crate) fn blocked_method_position_chirho(
    scheme_chirho: &SchemeChirho,
    class_name_chirho: &str,
    class_var_chirho: TyVarChirho,
    roles_chirho: &ParameterRolesChirho,
) -> Option<BlockedPositionChirho> {
    let class_var_ty_chirho = TyChirho::VarChirho(class_var_chirho);
    for predicate_chirho in &scheme_chirho.preds_chirho {
        // The class's own predicate on its parameter is the dictionary GND replaces.
        let own_predicate_chirho = predicate_chirho.class_name_chirho == class_name_chirho
            && predicate_chirho.ty_chirho == class_var_ty_chirho
            && predicate_chirho.extra_tys_chirho.is_empty();
        if own_predicate_chirho {
            continue;
        }
        let mentions_chirho = std::iter::once(&predicate_chirho.ty_chirho)
            .chain(&predicate_chirho.extra_tys_chirho)
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
    };
    walker_chirho.walk_chirho(&scheme_chirho.ty_chirho, None)
}

/// One method type's walk.
struct MethodWalkChirho<'a> {
    class_var_chirho: TyVarChirho,
    roles_chirho: &'a ParameterRolesChirho,
}

impl MethodWalkChirho<'_> {
    /// The first blocked occurrence in `ty_chirho`, where `blocked_chirho` is the position
    /// that already blocks the whole of it, if any.
    fn walk_chirho(
        &self,
        ty_chirho: &TyChirho,
        blocked_chirho: Option<&BlockedPositionChirho>,
    ) -> Option<BlockedPositionChirho> {
        match ty_chirho {
            TyChirho::VarChirho(var_chirho) if *var_chirho == self.class_var_chirho => {
                blocked_chirho.cloned()
            }
            TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => None,
            TyChirho::FunChirho(argument_chirho, result_chirho, _) => self
                .walk_chirho(argument_chirho, blocked_chirho)
                .or_else(|| self.walk_chirho(result_chirho, blocked_chirho)),
            TyChirho::TupleChirho(elements_chirho) => elements_chirho
                .iter()
                .find_map(|element_chirho| self.walk_chirho(element_chirho, blocked_chirho)),
            TyChirho::ListChirho(element_chirho) => {
                self.walk_chirho(element_chirho, blocked_chirho)
            }
            TyChirho::ForallChirho { body_chirho, .. }
            | TyChirho::RequiredForallChirho { body_chirho, .. } => {
                self.walk_chirho(body_chirho, blocked_chirho)
            }
            TyChirho::AppChirho(..) => self.walk_application_chirho(ty_chirho, blocked_chirho),
        }
    }

    /// An application: its arguments take the roles its head gives them.
    fn walk_application_chirho(
        &self,
        ty_chirho: &TyChirho,
        blocked_chirho: Option<&BlockedPositionChirho>,
    ) -> Option<BlockedPositionChirho> {
        let mut arguments_chirho = Vec::new();
        let mut head_chirho = ty_chirho;
        while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
            arguments_chirho.push(argument_chirho.as_ref());
            head_chirho = function_chirho.as_ref();
        }
        arguments_chirho.reverse();
        match head_chirho {
            TyChirho::ConChirho(constructor_chirho) => {
                let local_roles_chirho = self.roles_chirho.get(constructor_chirho);
                arguments_chirho
                    .iter()
                    .enumerate()
                    .find_map(|(index_chirho, argument_chirho)| {
                        let role_chirho = local_roles_chirho
                            .and_then(|roles_chirho| roles_chirho.get(index_chirho))
                            .copied();
                        match role_chirho {
                            Some(role_chirho) if role_chirho.is_phantom_chirho() => None,
                            Some(ParameterRoleChirho::REPRESENTATIONAL_CHIRHO) => {
                                self.walk_chirho(argument_chirho, blocked_chirho)
                            }
                            known_chirho => {
                                let here_chirho = BlockedPositionChirho {
                                    under_chirho: format!("`{constructor_chirho}`"),
                                    proven_nominal_chirho: known_chirho
                                        .is_some_and(ParameterRoleChirho::is_nominal_chirho),
                                };
                                self.walk_chirho(
                                    argument_chirho,
                                    Some(blocked_chirho.unwrap_or(&here_chirho)),
                                )
                            }
                        }
                    })
            }
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
                self.walk_chirho(head_chirho, blocked_chirho).or_else(|| {
                    arguments_chirho.iter().find_map(|argument_chirho| {
                        self.walk_chirho(
                            argument_chirho,
                            Some(blocked_chirho.unwrap_or(&here_chirho)),
                        )
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
        TyChirho::ForallChirho { body_chirho, .. }
        | TyChirho::RequiredForallChirho { body_chirho, .. } => {
            mentions_var_chirho(body_chirho, var_chirho)
        }
    }
}
