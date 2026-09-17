// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Class source validity is independent of boot agreement.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::*;
pub(super) fn check_class_chirho(
    declaration_chirho: &DeclChirho,
    errors_chirho: &mut Vec<ValidityErrorChirho>,
) {
    let DeclChirho::ClassDeclChirho {
        associated_tfs_chirho,
        ..
    } = declaration_chirho
    else {
        return;
    };
    let mut names_chirho = std::collections::HashSet::new();
    for family_chirho in associated_tfs_chirho {
        let reason_chirho = if !family_chirho.head_declared_chirho {
            Some("default names no declared associated family")
        } else if family_chirho.result_chirho.binder_chirho.is_some()
            && family_chirho.result_chirho.injectivity_chirho.is_none()
        {
            Some("associated family result binder requires an injectivity annotation")
        } else if !names_chirho.insert(family_chirho.name_chirho.text_chirho()) {
            Some("duplicate associated family declaration")
        } else if family_chirho.defaults_chirho.len() > 1 {
            Some("multiple defaults for an associated family")
        } else if family_chirho.data_chirho && !family_chirho.defaults_chirho.is_empty() {
            Some("associated data family cannot have a type default")
        } else {
            None
        };
        if let Some(reason_chirho) = reason_chirho {
            errors_chirho.push(ValidityErrorChirho {
                message_chirho: reason_chirho.to_owned(),
                suggested_extension_chirho: None,
                span_chirho: family_chirho.span_chirho,
            });
        }
        if !family_chirho.head_declared_chirho {
            // No declaration exists from which to establish a default's arity.
            continue;
        }
        for equation_chirho in &family_chirho.defaults_chirho {
            let mut binders_chirho = std::collections::HashSet::new();
            let reason_chirho = if equation_chirho.lhs_types_chirho.len()
                != family_chirho.type_vars_chirho.len()
            {
                Some(format!(
                    "associated default arity must match its family: expected {} parameters, found {}",
                    family_chirho.type_vars_chirho.len(),
                    equation_chirho.lhs_types_chirho.len(),
                ))
            } else if equation_chirho
                .lhs_types_chirho
                .iter()
                .any(
                    |argument_chirho| match argument_chirho.unannotated_chirho() {
                        TypeChirho::VarChirho(name_chirho) => {
                            !binders_chirho.insert(name_chirho.text_chirho())
                        }
                        _ => true,
                    },
                )
            {
                Some("associated default requires distinct variable arguments".to_owned())
            } else {
                None
            };
            if let Some(message_chirho) = reason_chirho {
                errors_chirho.push(ValidityErrorChirho {
                    message_chirho,
                    suggested_extension_chirho: None,
                    span_chirho: equation_chirho.span_chirho,
                });
            }
        }
    }
}
