// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin constructor seeds: True and False, LT, EQ and GT, then Nothing and Just. A
//! module's own constructors of the same names replace them before its declarations are
//! inferred (`collect_module_shadowed_names_chirho`).

use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

pub(super) fn seed_constructors_chirho(env_chirho: &mut TyEnvChirho) {
    // True :: Bool, False :: Bool
    env_chirho.bind_chirho(
        "True".to_string(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );
    env_chirho.bind_chirho(
        "False".to_string(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );

    // LT, EQ, GT :: Ordering. Unbound, each was a fresh type variable whose Eq and Ord
    // occurrences defaulted to Int evidence, so `compare LT GT` dispatched at Int.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    for name_chirho in ["LT", "EQ", "GT"] {
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho::mono_chirho(TyChirho::ConChirho("Ordering".to_string())),
        );
    }

    // Nothing :: forall a. Maybe a
    // Just :: forall a. a -> Maybe a
    let maybe_arg_chirho = TyVarChirho(1042);
    let maybe_ty_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("Maybe".to_string())),
        Box::new(TyChirho::VarChirho(maybe_arg_chirho)),
    );
    env_chirho.bind_chirho(
        "Nothing".to_string(),
        SchemeChirho {
            vars_chirho: vec![maybe_arg_chirho],
            preds_chirho: vec![],
            ty_chirho: maybe_ty_chirho.clone(),
        },
    );
    env_chirho.bind_chirho(
        "Just".to_string(),
        SchemeChirho {
            vars_chirho: vec![maybe_arg_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(TyChirho::VarChirho(maybe_arg_chirho), maybe_ty_chirho),
        },
    );
}
