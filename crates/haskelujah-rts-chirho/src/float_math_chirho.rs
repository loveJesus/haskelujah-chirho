// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The unary Floating primops at Double, through libm. GHC's `sqrtDouble#`,
//! `sinhDouble#` and the rest are the C library's functions.
//! - **Rust's std:** its f64 methods call the same functions for all but `asinh`, `acosh`
//!   and `atanh`, which std computes itself. Those three are bound to libm's own symbols.
//! - **One table:** both native backends call the symbol it names (row 575).
//! - **The interpreter** applies the same functions through `apply_float_math_chirho`, so
//!   every engine makes one call per primop (row 578).
//! - **`atan2`** is GHC's own Haskell, not libm: `atan2_double_chirho` transcribes it, and
//!   every engine calls it (row 580).
//!
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

// libm's inverse hyperbolics, which Rust's std does not call.
unsafe extern "C" {
    safe fn asinh(value_chirho: f64) -> f64;
    safe fn acosh(value_chirho: f64) -> f64;
    safe fn atanh(value_chirho: f64) -> f64;
}

macro_rules! float_math_chirho {
    ($(($primop_chirho:literal, $symbol_chirho:ident, $function_chirho:path)),* $(,)?) => {
        /// Each unary Floating primop and the RTS symbol native code calls for it.
        pub const FLOAT_MATH_PRIMOPS_CHIRHO: &[(&str, &str)] =
            &[$(($primop_chirho, stringify!($symbol_chirho))),*];

        /// The libm function a unary Floating primop names, applied to `value_chirho`;
        /// None for any other primop.
        pub fn apply_float_math_chirho(primop_chirho: &str, value_chirho: f64) -> Option<f64> {
            match primop_chirho {
                $($primop_chirho => Some($function_chirho(value_chirho)),)*
                _ => None,
            }
        }

        $(
            #[doc = concat!(
                "Native `", $primop_chirho, "`: libm's `", stringify!($function_chirho),
                "` over a Double's bits."
            )]
            #[unsafe(no_mangle)]
            pub extern "C" fn $symbol_chirho(bits_chirho: i64) -> i64 {
                $function_chirho(f64::from_bits(bits_chirho as u64)).to_bits() as i64
            }
        )*
    };
}

float_math_chirho! {
    ("sqrt#", haskelujah_float_sqrt_chirho, f64::sqrt),
    ("exp#", haskelujah_float_exp_chirho, f64::exp),
    ("log#", haskelujah_float_log_chirho, f64::ln),
    ("sin#", haskelujah_float_sin_chirho, f64::sin),
    ("cos#", haskelujah_float_cos_chirho, f64::cos),
    ("tan#", haskelujah_float_tan_chirho, f64::tan),
    ("asin#", haskelujah_float_asin_chirho, f64::asin),
    ("acos#", haskelujah_float_acos_chirho, f64::acos),
    ("atan#", haskelujah_float_atan_chirho, f64::atan),
    ("sinh#", haskelujah_float_sinh_chirho, f64::sinh),
    ("cosh#", haskelujah_float_cosh_chirho, f64::cosh),
    ("tanh#", haskelujah_float_tanh_chirho, f64::tanh),
    ("asinh#", haskelujah_float_asinh_chirho, asinh),
    ("acosh#", haskelujah_float_acosh_chirho, acosh),
    ("atanh#", haskelujah_float_atanh_chirho, atanh),
}

/// The RTS symbol native code calls for `primop_chirho`, if it is a unary Floating primop.
pub fn float_math_symbol_chirho(primop_chirho: &str) -> Option<&'static str> {
    FLOAT_MATH_PRIMOPS_CHIRHO
        .iter()
        .find(|(name_chirho, _)| *name_chirho == primop_chirho)
        .map(|(_, symbol_chirho)| *symbol_chirho)
}

fn is_negative_zero_chirho(value_chirho: f64) -> bool {
    value_chirho == 0.0 && value_chirho.is_sign_negative()
}

/// GHC's `atan2 y x` at Double: GHC.Float's `RealFloat` class default, which the Double
/// instance keeps, with its guards in GHC's order (row 580). It is not libm's `atan2`.
/// `atan (y/x)` rounds the quotient first, so `atan2 1 2` is 0.46364760900080615 where
/// libm gives 0.4636476090008061, and two infinities make a NaN quotient.
pub fn atan2_double_chirho(y_chirho: f64, x_chirho: f64) -> f64 {
    if x_chirho > 0.0 {
        (y_chirho / x_chirho).atan()
    } else if x_chirho == 0.0 && y_chirho > 0.0 {
        std::f64::consts::PI / 2.0
    } else if x_chirho < 0.0 && y_chirho > 0.0 {
        std::f64::consts::PI + (y_chirho / x_chirho).atan()
    } else if (x_chirho <= 0.0 && y_chirho < 0.0)
        || (x_chirho < 0.0 && is_negative_zero_chirho(y_chirho))
        || (is_negative_zero_chirho(x_chirho) && is_negative_zero_chirho(y_chirho))
    {
        -atan2_double_chirho(-y_chirho, x_chirho)
    } else if y_chirho == 0.0 && (x_chirho < 0.0 || is_negative_zero_chirho(x_chirho)) {
        std::f64::consts::PI
    } else if x_chirho == 0.0 && y_chirho == 0.0 {
        y_chirho
    } else {
        // x or y is a NaN
        x_chirho + y_chirho
    }
}

/// Native `atan2#`: [`atan2_double_chirho`] over two Doubles' bits.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_atan2_double_chirho(y_bits_chirho: i64, x_bits_chirho: i64) -> i64 {
    atan2_double_chirho(
        f64::from_bits(y_bits_chirho as u64),
        f64::from_bits(x_bits_chirho as u64),
    )
    .to_bits() as i64
}
