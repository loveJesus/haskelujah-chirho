// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin constructor seeds: original lines 7835-7867 of `seed_builtins_chirho`
//! (True, False, Nothing, Just), in order.

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
