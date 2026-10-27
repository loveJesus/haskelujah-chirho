// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC-compatible `Double -> Int` conversion after an explicit rounding step.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

const FRACTION_BITS_CHIRHO: i32 = 52;
const EXPONENT_BIAS_CHIRHO: i32 = 1023;
const FRACTION_MASK_CHIRHO: u64 = (1_u64 << FRACTION_BITS_CHIRHO) - 1;
const IMPLICIT_BIT_CHIRHO: u64 = 1_u64 << FRACTION_BITS_CHIRHO;

/// Return the low 64 bits of an integral finite `f64`, with sign, as GHC's
/// `Int` conversion does. Non-finite inputs convert to zero. Working from the
/// IEEE significand avoids backend-specific poison or saturating casts.
fn integral_float_to_wrapping_int_chirho(value_chirho: f64) -> i64 {
    if !value_chirho.is_finite() || value_chirho == 0.0 {
        return 0;
    }

    let bits_chirho = value_chirho.to_bits();
    let exponent_bits_chirho = ((bits_chirho >> FRACTION_BITS_CHIRHO) & 0x7ff) as i32;
    if exponent_bits_chirho == 0 {
        return 0;
    }

    let exponent_chirho = exponent_bits_chirho - EXPONENT_BIAS_CHIRHO;
    if exponent_chirho < 0 {
        return 0;
    }

    let significand_chirho = IMPLICIT_BIT_CHIRHO | (bits_chirho & FRACTION_MASK_CHIRHO);
    let magnitude_chirho = if exponent_chirho < FRACTION_BITS_CHIRHO {
        significand_chirho >> (FRACTION_BITS_CHIRHO - exponent_chirho)
    } else {
        let shift_chirho = exponent_chirho - FRACTION_BITS_CHIRHO;
        if shift_chirho >= 64 {
            0
        } else {
            significand_chirho << shift_chirho
        }
    };

    if bits_chirho >> 63 == 0 {
        magnitude_chirho as i64
    } else {
        magnitude_chirho.wrapping_neg() as i64
    }
}

pub(crate) fn round_float_to_int_chirho(value_chirho: f64) -> i64 {
    integral_float_to_wrapping_int_chirho(value_chirho.round_ties_even())
}

pub fn truncate_float_to_int_chirho(value_chirho: f64) -> i64 {
    integral_float_to_wrapping_int_chirho(value_chirho.trunc())
}

pub(crate) fn floor_float_to_int_chirho(value_chirho: f64) -> i64 {
    integral_float_to_wrapping_int_chirho(value_chirho.floor())
}

pub(crate) fn ceiling_float_to_int_chirho(value_chirho: f64) -> i64 {
    integral_float_to_wrapping_int_chirho(value_chirho.ceil())
}

#[cfg(test)]
mod tests_chirho {
    use super::{
        ceiling_float_to_int_chirho, floor_float_to_int_chirho, round_float_to_int_chirho,
        truncate_float_to_int_chirho,
    };

    #[test]
    fn each_direction_matches_ghc_on_positive_and_negative_values_chirho() {
        let inputs_chirho = [-2.7, -2.5, -2.0, -0.5, 0.0, 0.5, 2.0, 2.5, 2.7];
        assert_eq!(
            inputs_chirho.map(round_float_to_int_chirho),
            [-3, -2, -2, 0, 0, 0, 2, 2, 3]
        );
        assert_eq!(
            inputs_chirho.map(truncate_float_to_int_chirho),
            [-2, -2, -2, 0, 0, 0, 2, 2, 2]
        );
        assert_eq!(
            inputs_chirho.map(floor_float_to_int_chirho),
            [-3, -3, -2, -1, 0, 0, 2, 2, 2]
        );
        assert_eq!(
            inputs_chirho.map(ceiling_float_to_int_chirho),
            [-2, -2, -2, 0, 0, 1, 2, 3, 3]
        );
    }

    #[test]
    fn non_finite_values_zero_and_large_finite_values_wrap_chirho() {
        for conversion_chirho in [
            round_float_to_int_chirho,
            truncate_float_to_int_chirho,
            floor_float_to_int_chirho,
            ceiling_float_to_int_chirho,
        ] {
            assert_eq!(conversion_chirho(f64::NAN), 0);
            assert_eq!(conversion_chirho(f64::INFINITY), 0);
            assert_eq!(conversion_chirho(f64::NEG_INFINITY), 0);
            assert_eq!(conversion_chirho(1.0e20), 7_766_279_631_452_241_920);
            assert_eq!(conversion_chirho(-1.0e20), -7_766_279_631_452_241_920);
            assert_eq!(
                conversion_chirho(9_223_372_036_854_774_784.0),
                i64::MAX - 1023
            );
            assert_eq!(conversion_chirho(9_223_372_036_854_775_808.0), i64::MIN);
            assert_eq!(conversion_chirho(18_446_744_073_709_549_568.0), -2048);
            assert_eq!(conversion_chirho(18_446_744_073_709_551_616.0), 0);
            assert_eq!(conversion_chirho(-9_223_372_036_854_775_808.0), i64::MIN);
            assert_eq!(
                conversion_chirho(-9_223_372_036_854_777_856.0),
                i64::MAX - 2047
            );
        }
    }
}
