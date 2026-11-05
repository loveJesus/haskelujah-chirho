// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rules 17 to 24: role annotations (`type role T r1 .. rn`, row 694, #31844, #31863).
//!
//! Lowering keeps every annotation as written. Declaration checking judges it here, in the
//! three stages GHC 9.14.1 judges it in (root's `role-annotation-r1-chirho` and
//! `role-surfaces-and-direct-gnd-r1-chirho` measurements), each stopping the later ones:
//!
//! - As GHC's parser: 17. every word is `nominal`, `representational`, `phantom` or `_`
//!   (GHC-09009).
//! - As GHC's renamer, over the written names: 20. a type takes one annotation
//!   (GHC-97170, at the repeat); 21. the target is a type or class this module declares
//!   (GHC-44432 for an undeclared or an imported one, RX7 and RX8). A qualified target is a
//!   named Unsupported gap.
//! - As GHC's `checkValidRoleAnnots`, for each annotation of a data type, newtype or class:
//!   22. the module has RoleAnnotations (GHC-17779, RX1; GHC2024 implies it, and only the
//!   last edition counts, RX2 and RX3), and checking goes on; 18. one role per visible
//!   parameter (GHC-54298), else nothing more is judged; 19. each written role is at least the
//!   role inference proves (GHC-29178), since the final role starts from the written one and
//!   only rises; 24. a class parameter's final role is nominal unless IncoherentInstances is
//!   on (GHC-18273, RX6; RX5's nominal holds). An annotation of a synonym or a type family,
//!   23, takes no roles at all (GHC-77192, RX4).
//!
//! A role the module cannot prove (Unknown) never stands for GHC's verdict: against it rules
//! 19 and 24 give a named Unsupported gap, and so does an annotation of an associated family,
//! whose check this module has no measurement for. Every annotation is judged, whether or not
//! a GND request reads its roles.
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, TyVarChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_span_chirho::SpanChirho;

use super::super::extension_enabled_chirho;
use super::{DeclarationErrorChirho, UNSUPPORTED_SHAPE_CHIRHO};
use crate::infer_chirho::gnd_coercion_chirho::{
    ParameterRoleChirho, ParameterRolesChirho, local_class_roles_chirho,
    local_parameter_roles_chirho,
};

/// The words a role annotation may write; `_` asks for the inferred role.
const ROLE_WORDS_CHIRHO: [&str; 4] = ["nominal", "representational", "phantom", "_"];

/// One `type role` declaration as lowering kept it.
struct AnnotationChirho<'a> {
    target_chirho: &'a NameChirho,
    words_chirho: &'a [NameChirho],
    span_chirho: SpanChirho,
}

/// Rules 17 to 24 over every role annotation of the module, in declaration order.
/// workflow: compiler-pipeline-chirho/declaration-validity-chirho
pub(super) fn check_role_annotations_chirho(
    module_chirho: &ModuleChirho,
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) {
    let annotations_chirho: Vec<AnnotationChirho<'_>> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::RoleAnnotationDeclChirho {
                type_name_chirho,
                roles_chirho,
                span_chirho,
            } => Some(AnnotationChirho {
                target_chirho: type_name_chirho,
                words_chirho: roles_chirho,
                span_chirho: *span_chirho,
            }),
            _ => None,
        })
        .collect();
    if annotations_chirho.is_empty() {
        return;
    }
    // Rule 17, a parse error in GHC: nothing else is judged.
    let unreadable_chirho: Vec<DeclarationErrorChirho> = annotations_chirho
        .iter()
        .flat_map(|annotation_chirho| annotation_chirho.words_chirho)
        .filter(|word_chirho| !ROLE_WORDS_CHIRHO.contains(&word_chirho.text_chirho()))
        .map(|word_chirho| DeclarationErrorChirho {
            ghc_code_chirho: "GHC-09009",
            message_chirho: format!("Illegal role name ‘{}’", word_chirho.text_chirho()),
            span_chirho: word_chirho.span_chirho(),
            related_span_chirho: None,
        })
        .collect();
    if !unreadable_chirho.is_empty() {
        errors_chirho.extend(unreadable_chirho);
        return;
    }
    // Rules 20 and 21, renamer errors in GHC: no role is judged after one.
    let mut renamer_errors_chirho = Vec::new();
    let resolved_chirho = renamed_annotations_chirho(
        module_chirho,
        &annotations_chirho,
        &mut renamer_errors_chirho,
    );
    if !renamer_errors_chirho.is_empty() {
        errors_chirho.extend(renamer_errors_chirho);
        return;
    }
    let data_roles_chirho = local_parameter_roles_chirho(module_chirho);
    let judge_chirho = RoleJudgeChirho {
        class_roles_chirho: local_class_roles_chirho(module_chirho, &data_roles_chirho),
        data_roles_chirho,
        role_annotations_chirho: extension_enabled_chirho(module_chirho, "RoleAnnotations"),
        incoherent_instances_chirho: extension_enabled_chirho(module_chirho, "IncoherentInstances"),
    };
    for annotation_chirho in resolved_chirho {
        judge_chirho.judge_chirho(module_chirho, annotation_chirho, errors_chirho);
    }
}

/// Rules 20 and 21: the first annotation of each written name, when its target is a type or
/// class this module declares; every other annotation is an error.
fn renamed_annotations_chirho<'a>(
    module_chirho: &ModuleChirho,
    annotations_chirho: &'a [AnnotationChirho<'a>],
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) -> Vec<&'a AnnotationChirho<'a>> {
    let mut first_chirho: HashMap<String, &AnnotationChirho<'_>> = HashMap::new();
    let mut unique_chirho = Vec::new();
    for annotation_chirho in annotations_chirho {
        let written_chirho = annotation_chirho.target_chirho.full_name_chirho();
        // Rule 20: GHC reports a repeat at its own declaration.
        if let Some(earlier_chirho) = first_chirho.get(&written_chirho) {
            errors_chirho.push(DeclarationErrorChirho {
                ghc_code_chirho: "GHC-97170",
                message_chirho: format!("Duplicate role annotations for ‘{written_chirho}’"),
                span_chirho: annotation_chirho.span_chirho,
                related_span_chirho: Some(earlier_chirho.span_chirho),
            });
            continue;
        }
        first_chirho.insert(written_chirho, annotation_chirho);
        unique_chirho.push(annotation_chirho);
    }
    unique_chirho
        .into_iter()
        .filter(|annotation_chirho| {
            let target_chirho = annotation_chirho.target_chirho;
            // Rule 21.
            let error_chirho = if target_chirho.qualifier_chirho().is_some() {
                DeclarationErrorChirho {
                    ghc_code_chirho: UNSUPPORTED_SHAPE_CHIRHO,
                    message_chirho: format!(
                        "Haskelujah does not resolve the qualified role annotation target `{}`",
                        target_chirho.full_name_chirho()
                    ),
                    span_chirho: target_chirho.span_chirho(),
                    related_span_chirho: None,
                }
            } else if declares_type_chirho(module_chirho, target_chirho.text_chirho()) {
                return true;
            } else {
                DeclarationErrorChirho {
                    ghc_code_chirho: "GHC-44432",
                    message_chirho: format!(
                        "The role annotation for ‘{}’ lacks an accompanying binding",
                        target_chirho.text_chirho()
                    ),
                    span_chirho: target_chirho.span_chirho(),
                    related_span_chirho: None,
                }
            };
            errors_chirho.push(error_chirho);
            false
        })
        .collect()
}

/// Whether the module declares a type-level name `name_chirho`: a data type, newtype, class,
/// synonym or type family, associated ones included. GHC's renamer looks a role annotation's
/// target up among exactly these, never through imports.
fn declares_type_chirho(module_chirho: &ModuleChirho, name_chirho: &str) -> bool {
    module_chirho
        .decls_chirho
        .iter()
        .any(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho: declared_chirho,
                ..
            }
            | DeclChirho::NewtypeDeclChirho {
                name_chirho: declared_chirho,
                ..
            }
            | DeclChirho::TypeAliasDeclChirho {
                name_chirho: declared_chirho,
                ..
            }
            | DeclChirho::TypeFamilyDeclChirho {
                name_chirho: declared_chirho,
                ..
            } => declared_chirho.text_chirho() == name_chirho,
            DeclChirho::ClassDeclChirho {
                name_chirho: declared_chirho,
                associated_tfs_chirho,
                ..
            } => {
                declared_chirho.text_chirho() == name_chirho
                    || associated_tfs_chirho
                        .iter()
                        .any(|family_chirho| family_chirho.name_chirho.text_chirho() == name_chirho)
            }
            _ => false,
        })
}

/// The visible parameters of the data type, newtype or class `name_chirho` the module
/// declares, and whether it is a class.
fn role_bearing_parameters_chirho(
    module_chirho: &ModuleChirho,
    name_chirho: &str,
) -> Option<(Vec<String>, bool)> {
    module_chirho.decls_chirho.iter().find_map(|decl_chirho| {
        let (declared_chirho, type_vars_chirho, class_chirho) = match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                ..
            }
            | DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                ..
            } => (name_chirho, type_vars_chirho, false),
            DeclChirho::ClassDeclChirho {
                name_chirho,
                type_vars_chirho,
                ..
            } => (name_chirho, type_vars_chirho, true),
            _ => return None,
        };
        (declared_chirho.text_chirho() == name_chirho).then(|| {
            (
                TyVarChirho::visible_binders_chirho(type_vars_chirho)
                    .iter()
                    .map(|binder_chirho| binder_chirho.text_chirho().to_string())
                    .collect(),
                class_chirho,
            )
        })
    })
}

/// What `checkValidRoleAnnots` reads for every annotation of one module.
struct RoleJudgeChirho {
    data_roles_chirho: ParameterRolesChirho,
    class_roles_chirho: ParameterRolesChirho,
    role_annotations_chirho: bool,
    incoherent_instances_chirho: bool,
}

impl RoleJudgeChirho {
    /// Rules 22, 18, 19, 24 and 23 for one annotation whose target the module declares.
    fn judge_chirho(
        &self,
        module_chirho: &ModuleChirho,
        annotation_chirho: &AnnotationChirho<'_>,
        errors_chirho: &mut Vec<DeclarationErrorChirho>,
    ) {
        let type_name_chirho = annotation_chirho.target_chirho.text_chirho();
        let Some((parameters_chirho, class_chirho)) =
            role_bearing_parameters_chirho(module_chirho, type_name_chirho)
        else {
            // Rule 23: a synonym or a family; an associated family's check is unmeasured.
            let associated_chirho = !module_chirho.decls_chirho.iter().any(|decl_chirho| {
                matches!(
                    decl_chirho,
                    DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
                        | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. }
                        if name_chirho.text_chirho() == type_name_chirho
                )
            });
            errors_chirho.push(if associated_chirho {
                DeclarationErrorChirho {
                    ghc_code_chirho: UNSUPPORTED_SHAPE_CHIRHO,
                    message_chirho: format!(
                        "Haskelujah does not check a role annotation for the associated family `{type_name_chirho}`"
                    ),
                    span_chirho: annotation_chirho.span_chirho,
                    related_span_chirho: None,
                }
            } else {
                DeclarationErrorChirho {
                    ghc_code_chirho: "GHC-77192",
                    message_chirho: format!(
                        "Illegal role annotation for {type_name_chirho}; they are allowed only for datatypes and classes."
                    ),
                    span_chirho: annotation_chirho.span_chirho,
                    related_span_chirho: None,
                }
            });
            return;
        };
        // Rule 22: checking goes on, as GHC's does.
        if !self.role_annotations_chirho {
            errors_chirho.push(DeclarationErrorChirho {
                ghc_code_chirho: "GHC-17779",
                message_chirho: format!("Illegal role annotation for {type_name_chirho}"),
                span_chirho: annotation_chirho.span_chirho,
                related_span_chirho: None,
            });
        }
        // Rule 18.
        let words_chirho = annotation_chirho.words_chirho;
        if words_chirho.len() != parameters_chirho.len() {
            errors_chirho.push(DeclarationErrorChirho {
                ghc_code_chirho: "GHC-54298",
                message_chirho: format!(
                    "Wrong number of roles listed in role annotation; Expected {}, got {}",
                    parameters_chirho.len(),
                    words_chirho.len()
                ),
                span_chirho: annotation_chirho.span_chirho,
                related_span_chirho: None,
            });
            return;
        }
        let roles_by_type_chirho = if class_chirho {
            &self.class_roles_chirho
        } else {
            &self.data_roles_chirho
        };
        let Some(final_roles_chirho) = roles_by_type_chirho.get(type_name_chirho) else {
            return;
        };
        // Rule 19.
        let mut unsupported_chirho = false;
        for ((parameter_chirho, word_chirho), final_chirho) in parameters_chirho
            .iter()
            .zip(words_chirho)
            .zip(final_roles_chirho)
        {
            let Some(written_chirho) =
                ParameterRoleChirho::from_annotation_chirho(word_chirho.text_chirho())
            else {
                continue;
            };
            if written_chirho >= *final_chirho {
                continue;
            }
            errors_chirho.push(match role_word_chirho(*final_chirho) {
                Some(required_chirho) => DeclarationErrorChirho {
                    ghc_code_chirho: "GHC-29178",
                    message_chirho: format!(
                        "Role mismatch on variable {parameter_chirho}: Annotation says {} but role {required_chirho} is required",
                        word_chirho.text_chirho()
                    ),
                    span_chirho: word_chirho.span_chirho(),
                    related_span_chirho: None,
                },
                None => {
                    unsupported_chirho = true;
                    DeclarationErrorChirho {
                        ghc_code_chirho: UNSUPPORTED_SHAPE_CHIRHO,
                        message_chirho: format!(
                            "Haskelujah cannot check the role annotation `{}` on variable {parameter_chirho} of `{type_name_chirho}`: it does not prove that parameter's role",
                            word_chirho.text_chirho()
                        ),
                        span_chirho: word_chirho.span_chirho(),
                        related_span_chirho: None,
                    }
                }
            });
        }
        // Rule 24.
        if !class_chirho
            || self.incoherent_instances_chirho
            || final_roles_chirho
                .iter()
                .all(|role_chirho| role_chirho.is_nominal_chirho())
        {
            return;
        }
        if final_roles_chirho.iter().any(|role_chirho| {
            role_word_chirho(*role_chirho).is_some_and(|word_chirho| word_chirho != "nominal")
        }) {
            errors_chirho.push(DeclarationErrorChirho {
                ghc_code_chirho: "GHC-18273",
                message_chirho:
                    "Roles other than ‘nominal’ for class parameters can lead to incoherence."
                        .to_string(),
                span_chirho: annotation_chirho.span_chirho,
                related_span_chirho: None,
            });
        } else if !unsupported_chirho {
            errors_chirho.push(DeclarationErrorChirho {
                ghc_code_chirho: UNSUPPORTED_SHAPE_CHIRHO,
                message_chirho: format!(
                    "Haskelujah cannot check the role annotation of the class `{type_name_chirho}`: it does not prove every parameter's role"
                ),
                span_chirho: annotation_chirho.span_chirho,
                related_span_chirho: None,
            });
        }
    }
}

/// How GHC's message names a proven role; `None` for one the module does not prove.
fn role_word_chirho(role_chirho: ParameterRoleChirho) -> Option<&'static str> {
    if role_chirho == ParameterRoleChirho::PHANTOM_CHIRHO {
        Some("phantom")
    } else if role_chirho == ParameterRoleChirho::REPRESENTATIONAL_CHIRHO {
        Some("representational")
    } else if role_chirho == ParameterRoleChirho::NOMINAL_CHIRHO {
        Some("nominal")
    } else {
        None
    }
}

#[cfg(test)]
#[path = "role_annotations_tests_chirho.rs"]
mod tests_chirho;
