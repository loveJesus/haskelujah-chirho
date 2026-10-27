// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::{
    haskelujah_pow_double_int_chirho, haskelujah_pow_float_chirho, haskelujah_pow_int_chirho,
};
use crate::float_math_chirho::{
    apply_float_math_chirho, atan2_double_chirho, float_math_symbol_chirho,
    haskelujah_atan2_double_chirho, haskelujah_exponent_double_chirho,
    haskelujah_float_acos_chirho, haskelujah_float_acosh_chirho, haskelujah_float_asin_chirho,
    haskelujah_float_asinh_chirho, haskelujah_float_atan_chirho, haskelujah_float_atanh_chirho,
    haskelujah_float_cos_chirho, haskelujah_float_cosh_chirho, haskelujah_float_exp_chirho,
    haskelujah_float_log_chirho, haskelujah_float_sin_chirho, haskelujah_float_sinh_chirho,
    haskelujah_float_sqrt_chirho, haskelujah_float_tan_chirho, haskelujah_float_tanh_chirho,
    haskelujah_significand_double_chirho, significand_double_chirho,
};
use crate::float_power_chirho::{pow_double_int_chirho, pow_float_chirho};
use crate::integer_power_chirho::pow_int_chirho;
use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

fn native_float_chirho(native_chirho: extern "C" fn(i64) -> i64, argument_chirho: f64) -> f64 {
    f64::from_bits(native_chirho(argument_chirho.to_bits() as i64) as u64)
}

/// Row 575: the unary Floating primops natively, against GHC 9.14.1's printed answers. A
/// finite answer is compared as its printed digits, which Rust's shortest form shares with
/// GHC's in this range; several are constants such as π/6, which Clippy would otherwise ask
/// to be named.
#[test]
fn unary_floating_primops_answer_as_ghc_chirho() {
    let cases_chirho: [(extern "C" fn(i64) -> i64, f64, &str); 12] = [
        (haskelujah_float_sqrt_chirho, 2.0, "1.4142135623730951"),
        (haskelujah_float_exp_chirho, 1.0, "2.718281828459045"),
        (haskelujah_float_log_chirho, 10.0, "2.302585092994046"),
        (haskelujah_float_sin_chirho, 1.0, "0.8414709848078965"),
        (haskelujah_float_cos_chirho, 1.0, "0.5403023058681398"),
        (haskelujah_float_tan_chirho, 1.0, "1.557407724654902"),
        (haskelujah_float_tan_chirho, 0.5, "0.5463024898437905"),
        (haskelujah_float_asin_chirho, 0.5, "0.5235987755982988"),
        (haskelujah_float_acos_chirho, 0.5, "1.0471975511965976"),
        (haskelujah_float_acos_chirho, -1.0, "3.141592653589793"),
        (haskelujah_float_atan_chirho, 1.0, "0.7853981633974483"),
        (haskelujah_float_atan_chirho, 1e300, "1.5707963267948966"),
    ];
    for (native_chirho, argument_chirho, ghc_chirho) in cases_chirho {
        assert_eq!(
            native_float_chirho(native_chirho, argument_chirho).to_string(),
            ghc_chirho,
            "{ghc_chirho}"
        );
    }
    assert!(native_float_chirho(haskelujah_float_sqrt_chirho, -1.0).is_nan());
    assert!(native_float_chirho(haskelujah_float_asin_chirho, 2.0).is_nan());
    assert_eq!(
        native_float_chirho(haskelujah_float_exp_chirho, 710.0),
        f64::INFINITY
    );
    assert_eq!(
        native_float_chirho(haskelujah_float_log_chirho, 0.0),
        f64::NEG_INFINITY
    );
    assert_eq!(
        float_math_symbol_chirho("sqrt#"),
        Some("haskelujah_float_sqrt_chirho")
    );
    assert_eq!(float_math_symbol_chirho("+.#"), None);
}

/// Row 578: the hyperbolic Floating primops, against GHC 9.14.1's printed answers. Rust's
/// std computes `asinh`, `acosh` and `atanh` itself, which gives 1.3504407402749723,
/// 0.6223625037147785 and -1.4722194895832208 at 1.8, 1.2 and -0.9. libm and GHC give
/// the answers below, natively and through the interpreter's call.
#[test]
fn hyperbolic_floating_primops_answer_as_ghc_chirho() {
    let cases_chirho: [(extern "C" fn(i64) -> i64, f64, &str); 9] = [
        (haskelujah_float_sinh_chirho, 1.0, "1.1752011936438014"),
        (haskelujah_float_cosh_chirho, 1.0, "1.5430806348152437"),
        (haskelujah_float_tanh_chirho, 0.5, "0.46211715726000974"),
        (haskelujah_float_asinh_chirho, 1.0, "0.881373587019543"),
        (haskelujah_float_asinh_chirho, 1.8, "1.3504407402749725"),
        (haskelujah_float_acosh_chirho, 2.0, "1.3169578969248166"),
        (haskelujah_float_acosh_chirho, 1.2, "0.6223625037147786"),
        (haskelujah_float_atanh_chirho, 0.5, "0.5493061443340549"),
        (haskelujah_float_atanh_chirho, -0.9, "-1.4722194895832204"),
    ];
    for (native_chirho, argument_chirho, ghc_chirho) in cases_chirho {
        assert_eq!(
            native_float_chirho(native_chirho, argument_chirho).to_string(),
            ghc_chirho,
            "{ghc_chirho}"
        );
    }
    for (primop_chirho, argument_chirho, ghc_chirho) in [
        ("asinh#", 1.8, "1.3504407402749725"),
        ("acosh#", 1.2, "0.6223625037147786"),
        ("atanh#", -0.9, "-1.4722194895832204"),
    ] {
        assert_eq!(
            apply_float_math_chirho(primop_chirho, argument_chirho)
                .map(|v_chirho| v_chirho.to_string()),
            Some(ghc_chirho.to_string()),
            "{primop_chirho} interpreted"
        );
    }
    assert_eq!(native_float_chirho(haskelujah_float_tanh_chirho, 30.0), 1.0);
    assert_eq!(
        native_float_chirho(haskelujah_float_sinh_chirho, 710.0),
        1.1169973830808555e308
    );
    assert_eq!(
        native_float_chirho(haskelujah_float_sinh_chirho, 711.0),
        f64::INFINITY
    );
    assert_eq!(
        native_float_chirho(haskelujah_float_atanh_chirho, 1.0),
        f64::INFINITY
    );
    assert!(native_float_chirho(haskelujah_float_acosh_chirho, 0.5).is_nan());
    assert_eq!(
        float_math_symbol_chirho("atanh#"),
        Some("haskelujah_float_atanh_chirho")
    );
}

/// Row 580: GHC's RealFloat default `atan2` at Double, against GHC 9.14.1's printed
/// answers. The table covers every quadrant, both axes, each signed-zero pair, NaN and the
/// infinities. libm's `atan2` gives 0.4636476090008061 and 2.9441970937399127 for the
/// first two, and π/4 for two positive infinities. Signed zeros and ±π are compared as raw
/// bits, because `==` cannot tell -0.0 from 0.0.
#[test]
fn atan2_is_ghcs_real_float_default_chirho() {
    for (y_chirho, x_chirho, ghc_chirho) in [
        (1.0, 2.0, "0.46364760900080615"),
        (1.0, -5.0, "2.9441970937399122"),
        (-1.0, -5.0, "-2.9441970937399122"),
        (-1.0, 5.0, "-0.19739555984988078"),
        (5.0, -1.0, "1.7681918866447772"),
        (1.0, 1.0, "0.7853981633974483"),
        (1.0, 0.0, "1.5707963267948966"),
        (-1.0, 0.0, "-1.5707963267948966"),
        (f64::INFINITY, 1.0, "1.5707963267948966"),
        (f64::NEG_INFINITY, 0.0, "-1.5707963267948966"),
    ] {
        assert_eq!(
            atan2_double_chirho(y_chirho, x_chirho).to_string(),
            ghc_chirho,
            "atan2 {y_chirho} {x_chirho}"
        );
    }
    for (y_chirho, x_chirho, ghc_chirho) in [
        (0.0, 1.0, 0.0),
        (0.0, -1.0, PI),
        (-0.0, -1.0, -PI),
        (-0.0, 1.0, -0.0),
        (0.0, 0.0, 0.0),
        (-0.0, 0.0, -0.0),
        (0.0, -0.0, PI),
        (-0.0, -0.0, -PI),
        (1.0, f64::NEG_INFINITY, PI),
        (-1.0, f64::NEG_INFINITY, -PI),
    ] {
        assert_eq!(
            atan2_double_chirho(y_chirho, x_chirho).to_bits(),
            ghc_chirho.to_bits(),
            "atan2 {y_chirho} {x_chirho}"
        );
    }
    for (y_chirho, x_chirho) in [
        (f64::NAN, 1.0),
        (1.0, f64::NAN),
        (f64::INFINITY, f64::INFINITY),
        (f64::NEG_INFINITY, f64::INFINITY),
        (f64::INFINITY, f64::NEG_INFINITY),
        (f64::NEG_INFINITY, f64::NEG_INFINITY),
    ] {
        assert!(
            atan2_double_chirho(y_chirho, x_chirho).is_nan(),
            "atan2 {y_chirho} {x_chirho}"
        );
    }
    let native_chirho = |y_chirho: f64, x_chirho: f64| {
        f64::from_bits(haskelujah_atan2_double_chirho(
            y_chirho.to_bits() as i64,
            x_chirho.to_bits() as i64,
        ) as u64)
    };
    assert_eq!(native_chirho(1.0, 2.0).to_string(), "0.46364760900080615");
    assert_eq!(native_chirho(-0.0, -0.0).to_bits(), (-PI).to_bits());
}

/// Row 585: representation-level RealFloat operations, including GHC's deliberate
/// representation answers for infinities and NaNs.
#[test]
fn real_float_components_answer_as_ghc_chirho() {
    let values_chirho = [0.0, -0.0, 1.5, -2.5, 1.0e-300, f64::INFINITY, f64::NAN];
    assert_eq!(
        values_chirho.map(crate::float_math_chirho::exponent_double_chirho),
        [0, 0, 1, 2, -996, 1025, 1025]
    );
    for (value_chirho, ghc_chirho) in
        values_chirho
            .into_iter()
            .zip([0.0f64, 0.0, 0.75, -0.625, 0.6696928794914171, 0.5, 0.75])
    {
        assert_eq!(
            significand_double_chirho(value_chirho).to_bits(),
            ghc_chirho.to_bits(),
            "significand {value_chirho}"
        );
    }
    assert_eq!(
        crate::float_math_chirho::exponent_double_chirho(f64::from_bits(1)),
        -1073
    );
    assert_eq!(significand_double_chirho(f64::from_bits(1)), 0.5);
    assert_eq!(
        haskelujah_exponent_double_chirho(1.5f64.to_bits() as i64),
        1
    );
    assert_eq!(
        haskelujah_significand_double_chirho((-2.5f64).to_bits() as i64),
        (-0.625f64).to_bits() as i64
    );
}

/// Row 573: GHC's Double `**`, the C library's `pow`, against GHC 9.14.1's printed answers,
/// libm's edge cases included. GHC prints `2 ** 0.5` as 1.4142135623730951 and
/// `0.5 ** 0.5` as 0.7071067811865476, which are `SQRT_2` and `FRAC_1_SQRT_2` exactly.
#[test]
fn float_power_is_the_c_librarys_pow_chirho() {
    for (base_chirho, exponent_chirho, ghc_chirho) in [
        (2.0, 0.5, SQRT_2),
        (2.0, 3.0, 8.0),
        (10.0, -2.0, 1.0e-2),
        (0.5, 0.5, FRAC_1_SQRT_2),
        (2.5, 1.5, 3.952847075210474),
        (0.0, 0.0, 1.0),
        (f64::NAN, 0.0, 1.0),
        (1.0, f64::NAN, 1.0),
        (1e308, 2.0, f64::INFINITY),
        (0.0, -1.0, f64::INFINITY),
        (2.0, 1024.0, f64::INFINITY),
        (2.0, -1075.0, 0.0),
        (-2.0, 3.0, -8.0),
    ] {
        assert_eq!(
            pow_float_chirho(base_chirho, exponent_chirho),
            ghc_chirho,
            "{base_chirho} ** {exponent_chirho}"
        );
    }
    assert!(pow_float_chirho(-8.0, 1.0 / 3.0).is_nan());
    assert!(pow_float_chirho(-2.0, 0.5).is_nan());
    assert_eq!(
        haskelujah_pow_float_chirho(2.0_f64.to_bits() as i64, 0.5_f64.to_bits() as i64),
        SQRT_2.to_bits() as i64
    );
}
/// Row 570: GHC.Real's `(^)` at Double, against GHC 9.14.1's printed answers. `1.1 ^ 50`
/// and `1.0000001 ^ 1000003` tell repeated squaring apart from `powf`, which gives
/// 117.39085287969579 and 1.1051712441656278.
#[test]
fn double_power_multiplies_in_ghcs_order_chirho() {
    for (base_chirho, exponent_chirho, ghc_chirho) in [
        (1.5, 2, 2.25),
        (1.5, 3, 3.375),
        (1.5, 10, 57.6650390625),
        (2.0, 60, 1.152921504606847e18),
        (1.1, 50, 117.39085287969571),
        (0.9, 100, 2.6561398887587605e-5),
        (1.0000001, 1_000_003, 1.1051712441594597),
        (-2.0, 3, -8.0),
        (-0.5, 4, 6.25e-2),
        (2.5, 0, 1.0),
        (2.5, 1, 2.5),
        (f64::INFINITY, 3, f64::INFINITY),
        (f64::NEG_INFINITY, 3, f64::NEG_INFINITY),
        (1e200, 2, f64::INFINITY),
    ] {
        assert_eq!(
            pow_double_int_chirho(base_chirho, exponent_chirho),
            Some(ghc_chirho),
            "{base_chirho} ^ {exponent_chirho}"
        );
    }
    assert!(pow_double_int_chirho(f64::NAN, 2).is_some_and(f64::is_nan));
    assert_eq!(pow_double_int_chirho(f64::NAN, 0), Some(1.0));
    assert!(
        pow_double_int_chirho(-0.0, 3)
            .is_some_and(|zero_chirho| { zero_chirho == 0.0 && zero_chirho.is_sign_negative() })
    );
    assert_eq!(pow_double_int_chirho(2.5, -1), None);
    assert_eq!(
        haskelujah_pow_double_int_chirho(1.1_f64.to_bits() as i64, 50),
        117.39085287969571_f64.to_bits() as i64
    );
}

#[test]
fn integer_power_matches_ghc_chirho() {
    assert_eq!(haskelujah_pow_int_chirho(2, 10), 1024);
    assert_eq!(haskelujah_pow_int_chirho(0, 0), 1);
    assert_eq!(haskelujah_pow_int_chirho(-2, 3), -8);
    assert_eq!(pow_int_chirho(2, -1), None);
    assert_eq!(
        haskelujah_pow_int_chirho(i64::MAX, 2),
        i64::MAX.wrapping_pow(2)
    );
}
