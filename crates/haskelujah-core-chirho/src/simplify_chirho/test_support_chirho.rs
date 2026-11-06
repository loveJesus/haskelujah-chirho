// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared builders for the simplifier's unit tests (row 700).

use crate::expr_chirho::{BinderChirho, CoreIdChirho};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

pub(super) fn dummy_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
    BinderChirho {
        id_chirho: CoreIdChirho(id_chirho),
        name_chirho: name_chirho.to_string(),
        ty_chirho: TyChirho::int_chirho(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}
