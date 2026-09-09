// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared, linear Core traversal support for execution lowering.

use crate::expr_chirho::CoreExprChirho;

pub(crate) fn children_mut_chirho(
    expression_chirho: &mut CoreExprChirho,
    visit_chirho: &mut impl FnMut(&mut CoreExprChirho),
) {
    match expression_chirho {
        CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => {}
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            visit_chirho(fun_chirho);
            visit_chirho(arg_chirho);
        }
        CoreExprChirho::LamChirho { body_chirho, .. }
        | CoreExprChirho::TyLamChirho { body_chirho, .. }
        | CoreExprChirho::TyAppChirho {
            expr_chirho: body_chirho,
            ..
        } => visit_chirho(body_chirho),
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (_, rhs_chirho) in binds_chirho {
                visit_chirho(rhs_chirho);
            }
            visit_chirho(body_chirho);
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            visit_chirho(scrutinee_chirho);
            for alt_chirho in alts_chirho {
                visit_chirho(&mut alt_chirho.rhs_chirho);
            }
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. }
        | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                visit_chirho(arg_chirho);
            }
        }
    }
}

pub(crate) fn max_expr_id_chirho(expression_chirho: &CoreExprChirho, maximum_chirho: &mut u32) {
    match expression_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            *maximum_chirho = (*maximum_chirho).max(id_chirho.0)
        }
        CoreExprChirho::LitChirho(_) => {}
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            max_expr_id_chirho(fun_chirho, maximum_chirho);
            max_expr_id_chirho(arg_chirho, maximum_chirho);
        }
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            *maximum_chirho = (*maximum_chirho).max(binder_chirho.id_chirho.0);
            max_expr_id_chirho(body_chirho, maximum_chirho);
        }
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (binder_chirho, rhs_chirho) in binds_chirho {
                *maximum_chirho = (*maximum_chirho).max(binder_chirho.id_chirho.0);
                max_expr_id_chirho(rhs_chirho, maximum_chirho);
            }
            max_expr_id_chirho(body_chirho, maximum_chirho);
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            *maximum_chirho = (*maximum_chirho).max(bind_chirho.id_chirho.0);
            max_expr_id_chirho(scrutinee_chirho, maximum_chirho);
            for alt_chirho in alts_chirho {
                for binder_chirho in &alt_chirho.binders_chirho {
                    *maximum_chirho = (*maximum_chirho).max(binder_chirho.id_chirho.0);
                }
                max_expr_id_chirho(&alt_chirho.rhs_chirho, maximum_chirho);
            }
        }
        CoreExprChirho::TyLamChirho { body_chirho, .. }
        | CoreExprChirho::TyAppChirho {
            expr_chirho: body_chirho,
            ..
        } => max_expr_id_chirho(body_chirho, maximum_chirho),
        CoreExprChirho::PrimOpChirho { args_chirho, .. }
        | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                max_expr_id_chirho(arg_chirho, maximum_chirho);
            }
        }
    }
}
