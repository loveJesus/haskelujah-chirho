// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC-compatible `Ord Double` ordering for the STG interpreter.
//!
//! Workflow: language-features-chirho/instance-obligations-chirho.md.

use std::cmp::Ordering;

/// Match GHC's `Ord Double` definition: less gives `LT`, equality gives `EQ`,
/// and every other case gives `GT`. IEEE NaN is neither less nor equal, so a
/// NaN on either side gives `GT`; negative and positive zero compare equal.
pub(super) fn compare_float_as_ghc_chirho(left_chirho: f64, right_chirho: f64) -> Ordering {
    if left_chirho < right_chirho {
        Ordering::Less
    } else if left_chirho == right_chirho {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn nan_on_either_side_is_greater_chirho() {
        let nan_chirho = f64::NAN;
        assert_eq!(
            compare_float_as_ghc_chirho(nan_chirho, 1.0),
            Ordering::Greater
        );
        assert_eq!(
            compare_float_as_ghc_chirho(1.0, nan_chirho),
            Ordering::Greater
        );
        assert_eq!(
            compare_float_as_ghc_chirho(nan_chirho, nan_chirho),
            Ordering::Greater
        );
    }

    #[test]
    fn ordinary_and_signed_zero_ordering_stays_total_chirho() {
        assert_eq!(compare_float_as_ghc_chirho(1.0, 2.0), Ordering::Less);
        assert_eq!(compare_float_as_ghc_chirho(2.0, 2.0), Ordering::Equal);
        assert_eq!(compare_float_as_ghc_chirho(3.0, 2.0), Ordering::Greater);
        assert_eq!(compare_float_as_ghc_chirho(-0.0, 0.0), Ordering::Equal);
    }
}
