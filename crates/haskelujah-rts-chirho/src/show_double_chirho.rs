// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC-compatible scalar `Double` rendering shared by the interpreter and native RTS.
//! workflow: print-show-evidence-chirho

/// Render one Haskell `Double` with GHC's special-value spellings and notation boundary.
/// workflow: print-show-evidence-chirho
pub fn show_double_chirho(value_chirho: f64) -> String {
    if value_chirho.is_nan() {
        "NaN".to_string()
    } else if value_chirho.is_infinite() {
        if value_chirho.is_sign_negative() {
            "-Infinity".to_string()
        } else {
            "Infinity".to_string()
        }
    } else {
        let scientific_chirho = format!("{value_chirho:e}");
        let Some((mantissa_chirho, exponent_chirho)) = scientific_chirho.rsplit_once('e') else {
            return format!("{value_chirho:?}");
        };
        let exponent_value_chirho = exponent_chirho.parse::<i32>().unwrap_or(0);
        if !(-1..=6).contains(&exponent_value_chirho) {
            if mantissa_chirho.contains('.') {
                scientific_chirho
            } else {
                format!("{mantissa_chirho}.0e{exponent_chirho}")
            }
        } else {
            format!("{value_chirho:?}")
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::show_double_chirho;

    #[test]
    fn specials_and_finite_thresholds_match_ghc_chirho() {
        for (value_chirho, expected_chirho) in [
            (f64::NAN, "NaN"),
            (f64::INFINITY, "Infinity"),
            (f64::NEG_INFINITY, "-Infinity"),
            (0.0, "0.0"),
            (-0.0, "-0.0"),
            (1.5, "1.5"),
            (9_999_999.0, "9999999.0"),
            (10_000_000.0, "1.0e7"),
            (-10_000_000.0, "-1.0e7"),
            (0.1, "0.1"),
            (0.01, "1.0e-2"),
            (-0.01, "-1.0e-2"),
            (1.0e20, "1.0e20"),
            (1.0e-20, "1.0e-20"),
            (f64::from_bits(1), "5.0e-324"),
            (f64::MIN_POSITIVE, "2.2250738585072014e-308"),
            (f64::MAX, "1.7976931348623157e308"),
        ] {
            assert_eq!(show_double_chirho(value_chirho), expected_chirho);
        }
    }
}
