// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rules 17 to 19: role annotations (`type role T r1 .. rn`, row 694, #31844).
//!
//! Lowering keeps every annotation as written. Declaration checking judges it here, against
//! root's GHC 9.14.1 measurements (`role-annotation-r1-chirho`):
//!
//! 17. every word is `nominal`, `representational`, `phantom` or `_` (GHC-09009, which GHC
//!     reports while parsing), and nothing more is judged when one is not;
//! 18. the annotation lists one role per visible parameter (GHC-54298), and nothing more is
//!     judged when it does not;
//! 19. each written role is at least the role inference proves for its parameter
//!     (GHC-29178). The final role starts from the written one and only rises, so a written
//!     role below it is one the declarations contradict. Against a role the module cannot
//!     prove (Unknown) the annotation is a named Unsupported gap, never GHC's mismatch.
//!
//! Every annotation is judged, whether or not a GND request reads its roles. An annotation
//! naming no data or newtype the module declares, a repeated one and a module without
//! RoleAnnotations are not judged yet: GHC rejects them, with codes still to be measured.
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use std::collections::HashSet;

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, TyVarChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;

use super::{DeclarationErrorChirho, UNSUPPORTED_SHAPE_CHIRHO};
use crate::infer_chirho::gnd_coercion_chirho::{ParameterRoleChirho, local_parameter_roles_chirho};

/// The words a role annotation may write; `_` asks for the inferred role.
const ROLE_WORDS_CHIRHO: [&str; 4] = ["nominal", "representational", "phantom", "_"];

/// Rules 17 to 19 over every role annotation of the module, in declaration order.
/// workflow: compiler-pipeline-chirho/declaration-validity-chirho
pub(super) fn check_role_annotations_chirho(
    module_chirho: &ModuleChirho,
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) {
    let annotations_chirho: Vec<(
        &NameChirho,
        &[NameChirho],
        haskelujah_span_chirho::SpanChirho,
    )> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::RoleAnnotationDeclChirho {
                type_name_chirho,
                roles_chirho,
                span_chirho,
            } => Some((type_name_chirho, roles_chirho.as_slice(), *span_chirho)),
            _ => None,
        })
        .collect();
    if annotations_chirho.is_empty() {
        return;
    }
    let final_roles_chirho = local_parameter_roles_chirho(module_chirho);
    let mut judged_chirho = HashSet::new();
    for (type_name_chirho, words_chirho, span_chirho) in annotations_chirho {
        if type_name_chirho.qualifier_chirho().is_some()
            || !judged_chirho.insert(type_name_chirho.text_chirho())
        {
            continue;
        }
        let Some(parameters_chirho) =
            local_parameters_chirho(module_chirho, type_name_chirho.text_chirho())
        else {
            continue;
        };
        // Rule 17.
        let mut readable_chirho = true;
        for word_chirho in words_chirho {
            if !ROLE_WORDS_CHIRHO.contains(&word_chirho.text_chirho()) {
                readable_chirho = false;
                errors_chirho.push(DeclarationErrorChirho {
                    ghc_code_chirho: "GHC-09009",
                    message_chirho: format!("Illegal role name ‘{}’", word_chirho.text_chirho()),
                    span_chirho: word_chirho.span_chirho(),
                    related_span_chirho: None,
                });
            }
        }
        if !readable_chirho {
            continue;
        }
        // Rule 18.
        if words_chirho.len() != parameters_chirho.len() {
            errors_chirho.push(DeclarationErrorChirho {
                ghc_code_chirho: "GHC-54298",
                message_chirho: format!(
                    "Wrong number of roles listed in role annotation; Expected {}, got {}",
                    parameters_chirho.len(),
                    words_chirho.len()
                ),
                span_chirho,
                related_span_chirho: None,
            });
            continue;
        }
        // Rule 19.
        let Some(roles_chirho) = final_roles_chirho.get(type_name_chirho.text_chirho()) else {
            continue;
        };
        for ((parameter_chirho, word_chirho), final_chirho) in
            parameters_chirho.iter().zip(words_chirho).zip(roles_chirho)
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
                None => DeclarationErrorChirho {
                    ghc_code_chirho: UNSUPPORTED_SHAPE_CHIRHO,
                    message_chirho: format!(
                        "Haskelujah cannot check the role annotation `{}` on variable {parameter_chirho} of `{}`: it does not prove that parameter's role",
                        word_chirho.text_chirho(),
                        type_name_chirho.text_chirho()
                    ),
                    span_chirho: word_chirho.span_chirho(),
                    related_span_chirho: None,
                },
            });
        }
    }
}

/// The visible parameters of the data or newtype `type_name_chirho` the module declares.
fn local_parameters_chirho(
    module_chirho: &ModuleChirho,
    type_name_chirho: &str,
) -> Option<Vec<String>> {
    module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                ..
            }
            | DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                ..
            } if name_chirho.text_chirho() == type_name_chirho => Some(
                TyVarChirho::visible_binders_chirho(type_vars_chirho)
                    .iter()
                    .map(|binder_chirho| binder_chirho.text_chirho().to_string())
                    .collect(),
            ),
            _ => None,
        })
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
