// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::haskelujah_pow_int_chirho;

#[test]
fn integer_power_matches_the_interpreter_convention_chirho() {
    assert_eq!(haskelujah_pow_int_chirho(2, 10), 1024);
    assert_eq!(haskelujah_pow_int_chirho(0, 0), 1);
    assert_eq!(haskelujah_pow_int_chirho(-2, 3), -8);
    assert_eq!(haskelujah_pow_int_chirho(2, -1), 0);
    assert_eq!(
        haskelujah_pow_int_chirho(i64::MAX, 2),
        i64::MAX.wrapping_pow(2)
    );
}
