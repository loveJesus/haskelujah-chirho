// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::{
    haskelujah_pow_double_int_chirho, haskelujah_pow_float_chirho, haskelujah_pow_int_chirho,
};
use crate::float_power_chirho::{pow_double_int_chirho, pow_float_chirho};
use crate::integer_power_chirho::pow_int_chirho;
use std::f64::consts::{FRAC_1_SQRT_2, SQRT_2};

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
