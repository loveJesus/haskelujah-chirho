// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The same source licence applies to types exposed by a checked synonym.
//! Canonical telescope visibility keeps hidden kind arguments in kind position.

use super::{
    DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho, ILLEGAL_TYPE_FORM_CODE_CHIRHO,
    LicenseChirho, TyPositionChirho,
};
use crate::ty_chirho::TyChirho;
use crate::ty_chirho::declarations_chirho::{DeclarationSchemesChirho, TelescopeVisibilityChirho};
use haskelujah_span_chirho::SpanChirho;

pub(crate) fn check_expanded_type_chirho(
    ty_chirho: &TyChirho,
    license_chirho: LicenseChirho,
    catalogue_chirho: &DeclarationSchemesChirho,
    span_chirho: SpanChirho,
) -> DiagnosticBundleChirho {
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();
    let mut pending_chirho = vec![(ty_chirho, TyPositionChirho::TOP_CHIRHO, 0usize)];
    let mut work_chirho = 1_000_000usize;
    while let Some((ty_chirho, position_chirho, depth_chirho)) = pending_chirho.pop() {
        if work_chirho == 0 || depth_chirho > 128 {
            diagnostics_chirho.push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(300),
                "expanded type validity resource bound exhausted",
                span_chirho,
            ));
            break;
        }
        work_chirho -= 1;
        let width_chirho = match ty_chirho {
            TyChirho::TupleChirho(elements_chirho) => elements_chirho.len(),
            TyChirho::QualChirho { context_chirho, .. } => context_chirho.len(),
            _ => 0,
        };
        if width_chirho > work_chirho.saturating_sub(pending_chirho.len()) {
            diagnostics_chirho.push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(300),
                "expanded type validity resource bound exhausted",
                span_chirho,
            ));
            break;
        }
        match ty_chirho {
            TyChirho::ForallChirho { body_chirho, .. }
            | TyChirho::RequiredForallChirho { body_chirho, .. }
            | TyChirho::QualChirho { body_chirho, .. } => {
                let extension_chirho = match position_chirho {
                    TyPositionChirho::RANK_N_CHIRHO if !license_chirho.rank_n_chirho => {
                        Some("RankNTypes")
                    }
                    TyPositionChirho::IMPREDICATIVE_CHIRHO
                        if !license_chirho.impredicative_chirho =>
                    {
                        Some("ImpredicativeTypes")
                    }
                    _ => None,
                };
                if let Some(extension_chirho) = extension_chirho {
                    diagnostics_chirho.push_chirho(
                        DiagnosticChirho::error_with_code_chirho(
                            ErrorCodeChirho::error_chirho(ILLEGAL_TYPE_FORM_CODE_CHIRHO),
                            format!("Illegal expanded polymorphic or qualified type: {ty_chirho}"),
                            span_chirho,
                        )
                        .with_note_chirho(format!(
                            "perhaps you intended to use the `{extension_chirho}` extension"
                        )),
                    );
                    continue;
                }
                pending_chirho.push((body_chirho, position_chirho, depth_chirho + 1));
                if let TyChirho::QualChirho { context_chirho, .. } = ty_chirho {
                    for constraint_chirho in context_chirho {
                        pending_chirho.push((
                            constraint_chirho,
                            TyPositionChirho::TOP_CHIRHO,
                            depth_chirho + 1,
                        ));
                    }
                }
            }
            TyChirho::FunChirho(argument_chirho, result_chirho, _) => {
                let nested_chirho = if position_chirho == TyPositionChirho::KIND_CHIRHO {
                    position_chirho
                } else {
                    TyPositionChirho::RANK_N_CHIRHO
                };
                pending_chirho.extend([
                    (argument_chirho.as_ref(), nested_chirho, depth_chirho + 1),
                    (result_chirho.as_ref(), nested_chirho, depth_chirho + 1),
                ]);
            }
            TyChirho::AppChirho(_, _) => {
                let mut head_chirho = ty_chirho;
                let mut arguments_chirho = Vec::new();
                while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
                    if work_chirho == 0 {
                        break;
                    }
                    work_chirho -= 1;
                    arguments_chirho.push(argument_chirho.as_ref());
                    head_chirho = function_chirho;
                }
                arguments_chirho.reverse();
                let schema_chirho = if let TyChirho::ConChirho(key_chirho) = head_chirho {
                    catalogue_chirho.get_chirho(key_chirho)
                } else {
                    None
                };
                pending_chirho.push((head_chirho, position_chirho, depth_chirho + 1));
                for (index_chirho, argument_chirho) in arguments_chirho.into_iter().enumerate() {
                    let hidden_chirho = schema_chirho.is_some_and(|schema_chirho| {
                        schema_chirho.binders_chirho.get(index_chirho).is_some_and(
                            |binder_chirho| {
                                binder_chirho.visibility_chirho
                                    != TelescopeVisibilityChirho::RequiredChirho
                            },
                        )
                    });
                    let abstraction_chirho = schema_chirho.is_some_and(|schema_chirho| {
                        schema_chirho.binders_chirho.get(index_chirho).is_some_and(
                            |binder_chirho| {
                                matches!(
                                    binder_chirho.kind_chirho,
                                    TyChirho::ForallChirho { .. }
                                        | TyChirho::RequiredForallChirho { .. }
                                )
                            },
                        )
                    });
                    pending_chirho.push((
                        argument_chirho,
                        if abstraction_chirho && position_chirho != TyPositionChirho::KIND_CHIRHO {
                            TyPositionChirho::KIND_ABSTRACTION_CHIRHO
                        } else if hidden_chirho || position_chirho == TyPositionChirho::KIND_CHIRHO
                        {
                            TyPositionChirho::KIND_CHIRHO
                        } else {
                            TyPositionChirho::IMPREDICATIVE_CHIRHO
                        },
                        depth_chirho + 1,
                    ));
                }
            }
            TyChirho::ListChirho(element_chirho) => pending_chirho.push((
                element_chirho,
                if position_chirho == TyPositionChirho::KIND_CHIRHO {
                    position_chirho
                } else {
                    TyPositionChirho::IMPREDICATIVE_CHIRHO
                },
                depth_chirho + 1,
            )),
            TyChirho::TupleChirho(elements_chirho) => {
                let nested_chirho = if position_chirho == TyPositionChirho::KIND_CHIRHO {
                    position_chirho
                } else {
                    TyPositionChirho::IMPREDICATIVE_CHIRHO
                };
                pending_chirho.extend(
                    elements_chirho
                        .iter()
                        .map(|element_chirho| (element_chirho, nested_chirho, depth_chirho + 1)),
                );
            }
            TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {}
        }
    }
    diagnostics_chirho
}
