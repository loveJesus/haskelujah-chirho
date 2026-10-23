// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The monad-transformer type builders the seed's parts share. Each was a non-capturing
//! closure in `seed_builtins_chirho` (original lines 12349-12392), its parameters and body
//! unchanged (rule B3).

use crate::ty_chirho::TyChirho;

pub(super) fn mk_state_t_ty_chirho(
    state_ty_chirho: TyChirho,
    monad_ty_chirho: TyChirho,
    inner_ty_chirho: TyChirho,
) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(state_ty_chirho),
            )),
            Box::new(monad_ty_chirho),
        )),
        Box::new(inner_ty_chirho),
    )
}

pub(super) fn mk_state_ty_chirho(state_ty_chirho: TyChirho, inner_ty_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("State".to_string())),
            Box::new(state_ty_chirho),
        )),
        Box::new(inner_ty_chirho),
    )
}

pub(super) fn mk_reader_t_ty_chirho(
    reader_env_ty_chirho: TyChirho,
    monad_ty_chirho: TyChirho,
    inner_ty_chirho: TyChirho,
) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                Box::new(reader_env_ty_chirho),
            )),
            Box::new(monad_ty_chirho),
        )),
        Box::new(inner_ty_chirho),
    )
}

pub(super) fn mk_maybe_t_ty_chirho(
    monad_ty_chirho: TyChirho,
    inner_ty_chirho: TyChirho,
) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("MaybeT".to_string())),
            Box::new(monad_ty_chirho),
        )),
        Box::new(inner_ty_chirho),
    )
}
