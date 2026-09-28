// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One-pass, outermost-first rewriting shared by signature and pattern givens.
//! Callers own scope, orientation and the bounded normalization fixpoint.

use crate::ty_chirho::TyChirho;

pub(crate) fn rewrite_type_chirho(
    ty_chirho: &TyChirho,
    lookup_chirho: &impl Fn(&TyChirho) -> Option<TyChirho>,
) -> TyChirho {
    if let Some(replacement_chirho) = lookup_chirho(ty_chirho) {
        return replacement_chirho;
    }
    let child_chirho = |ty_chirho: &TyChirho| rewrite_type_chirho(ty_chirho, lookup_chirho);
    match ty_chirho {
        TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {
            ty_chirho.clone()
        }
        TyChirho::AppChirho(fun_chirho, arg_chirho) => TyChirho::AppChirho(
            Box::new(child_chirho(fun_chirho)),
            Box::new(child_chirho(arg_chirho)),
        ),
        TyChirho::KindAppChirho(fun_chirho, arg_chirho) => TyChirho::KindAppChirho(
            Box::new(child_chirho(fun_chirho)),
            Box::new(child_chirho(arg_chirho)),
        ),
        TyChirho::FunChirho(arg_chirho, result_chirho, mult_chirho) => TyChirho::FunChirho(
            Box::new(child_chirho(arg_chirho)),
            Box::new(child_chirho(result_chirho)),
            *mult_chirho,
        ),
        TyChirho::TupleChirho(elements_chirho) => {
            TyChirho::TupleChirho(elements_chirho.iter().map(child_chirho).collect())
        }
        TyChirho::ListChirho(inner_chirho) => {
            TyChirho::ListChirho(Box::new(child_chirho(inner_chirho)))
        }
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } => TyChirho::ForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(child_chirho(body_chirho)),
        },
        TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => TyChirho::RequiredForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(child_chirho(body_chirho)),
        },
    }
}
