// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Linearity checking for the frontend, moved from the driver root (row 691, L5): in a module that
//! enables `LinearTypes`, each function whose signature has a linear arrow must use each linear
//! parameter exactly once; violations come back as warnings.

use super::*;

/// Check linearity constraints when the `LinearTypes` extension is enabled.
///
/// For each function with a type signature containing a linear arrow (`%1 ->` or `⊸`),
/// verifies that each linear parameter is used exactly once in the body.
/// Returns warning strings for any violations found.
pub(crate) fn check_module_linearity_chirho(
    module_chirho: &ModuleChirho,
    _infer_result_chirho: &InferResultChirho,
) -> Vec<String> {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_ast_chirho::ty_chirho::{MultiplicityChirho, TypeChirho as AstTypeChirho};
    use haskelujah_typing_chirho::linearity_chirho::{
        check_linearity_chirho, pat_bound_names_chirho,
    };

    let mut warnings_chirho = Vec::new();

    // Collect type signatures: name → AST type
    let mut type_sigs_chirho: std::collections::HashMap<String, &AstTypeChirho> =
        std::collections::HashMap::new();
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::TypeSigChirho {
            name_chirho,
            ty_chirho,
            ..
        } = decl_chirho
        {
            type_sigs_chirho.insert(name_chirho.text_chirho().to_string(), ty_chirho);
        }
    }

    // Check each function binding that has a type sig with linear arrows
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::FunBindChirho {
            name_chirho,
            matches_chirho,
            span_chirho,
        } = decl_chirho
        {
            let fn_name_chirho = name_chirho.text_chirho().to_string();
            let sig_ty_chirho = match type_sigs_chirho.get(&fn_name_chirho) {
                Some(ty_chirho) => *ty_chirho,
                None => continue,
            };

            // Extract linear parameter positions from the AST type signature
            // Walk FunChirho chain, checking mult_chirho field
            let mut linear_positions_chirho = Vec::new();
            let mut ty_cursor_chirho = sig_ty_chirho;
            loop {
                match ty_cursor_chirho {
                    AstTypeChirho::FunChirho {
                        mult_chirho,
                        result_chirho,
                        ..
                    } => {
                        let is_linear_chirho = match mult_chirho {
                            Some(MultiplicityChirho::OneChirho) => true,
                            _ => false,
                        };
                        linear_positions_chirho.push(is_linear_chirho);
                        ty_cursor_chirho = result_chirho;
                    }
                    // Skip forall, context, and parentheses to find inner FunChirho
                    AstTypeChirho::ForallChirho { body_chirho, .. } => {
                        ty_cursor_chirho = body_chirho;
                    }
                    AstTypeChirho::QualChirho { body_chirho, .. } => {
                        ty_cursor_chirho = body_chirho;
                    }
                    AstTypeChirho::ParenChirho { inner_chirho, .. } => {
                        ty_cursor_chirho = inner_chirho;
                    }
                    _ => break,
                }
            }

            // If no linear parameters, skip this function
            if !linear_positions_chirho.iter().any(|b_chirho| *b_chirho) {
                continue;
            }

            // Check each match arm
            for arm_chirho in matches_chirho {
                let mut param_names_chirho: Vec<(String, bool)> = Vec::new();
                for (i_chirho, pat_chirho) in arm_chirho.pats_chirho.iter().enumerate() {
                    let is_linear_chirho = linear_positions_chirho
                        .get(i_chirho)
                        .copied()
                        .unwrap_or(false);
                    for name_chirho in pat_bound_names_chirho(pat_chirho) {
                        param_names_chirho.push((name_chirho, is_linear_chirho));
                    }
                }

                let body_expr_chirho = match &arm_chirho.rhs_chirho {
                    haskelujah_ast_chirho::expr_chirho::RhsChirho::UnguardedChirho(e_chirho) => {
                        e_chirho
                    }
                    haskelujah_ast_chirho::expr_chirho::RhsChirho::GuardedChirho(_) => {
                        continue;
                    }
                };

                let violations_chirho =
                    check_linearity_chirho(&param_names_chirho, body_expr_chirho, *span_chirho);

                for v_chirho in &violations_chirho {
                    match v_chirho {
                        haskelujah_typing_chirho::linearity_chirho::LinearityViolationChirho::UsedMultipleChirho {
                            name_chirho: var_name_chirho,
                            count_chirho,
                            ..
                        } => {
                            warnings_chirho.push(format!(
                                "Linearity violation: linear variable `{var_name_chirho}` \
                                 is used {count_chirho} times in `{fn_name_chirho}`"
                            ));
                        }
                        haskelujah_typing_chirho::linearity_chirho::LinearityViolationChirho::UnusedLinearChirho {
                            name_chirho: var_name_chirho,
                            ..
                        } => {
                            warnings_chirho.push(format!(
                                "Linearity violation: linear variable `{var_name_chirho}` \
                                 is unused in `{fn_name_chirho}`"
                            ));
                        }
                    }
                }
            }
        }
    }

    warnings_chirho
}
