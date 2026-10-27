// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The unary Floating primops at Double that native code reaches through libm. GHC's
//! `sqrtDouble#`, `expDouble#` and the rest are the C library's functions, and Rust's
//! f64 methods call the same ones, so each RTS symbol answers as GHC does. Both native
//! backends refused these primops; they now call the symbol this one table names, and the
//! interpreter's own primops already make the same calls.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

macro_rules! float_math_chirho {
    ($(($primop_chirho:literal, $symbol_chirho:ident, $method_chirho:ident)),* $(,)?) => {
        /// Each unary Floating primop and the RTS symbol native code calls for it.
        pub const FLOAT_MATH_PRIMOPS_CHIRHO: &[(&str, &str)] =
            &[$(($primop_chirho, stringify!($symbol_chirho))),*];

        $(
            #[doc = concat!(
                "Native `", $primop_chirho, "`: libm's `", stringify!($method_chirho),
                "` over a Double's bits."
            )]
            #[unsafe(no_mangle)]
            pub extern "C" fn $symbol_chirho(bits_chirho: i64) -> i64 {
                f64::from_bits(bits_chirho as u64)
                    .$method_chirho()
                    .to_bits() as i64
            }
        )*
    };
}

float_math_chirho! {
    ("sqrt#", haskelujah_float_sqrt_chirho, sqrt),
    ("exp#", haskelujah_float_exp_chirho, exp),
    ("log#", haskelujah_float_log_chirho, ln),
    ("sin#", haskelujah_float_sin_chirho, sin),
    ("cos#", haskelujah_float_cos_chirho, cos),
    ("tan#", haskelujah_float_tan_chirho, tan),
    ("asin#", haskelujah_float_asin_chirho, asin),
    ("acos#", haskelujah_float_acos_chirho, acos),
    ("atan#", haskelujah_float_atan_chirho, atan),
}

/// The RTS symbol native code calls for `primop_chirho`, if it is a unary Floating primop.
pub fn float_math_symbol_chirho(primop_chirho: &str) -> Option<&'static str> {
    FLOAT_MATH_PRIMOPS_CHIRHO
        .iter()
        .find(|(name_chirho, _)| *name_chirho == primop_chirho)
        .map(|(_, symbol_chirho)| *symbol_chirho)
}
