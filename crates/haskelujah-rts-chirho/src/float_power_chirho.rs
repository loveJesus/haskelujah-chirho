// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC's `(^)` at Double (row 570). `x ^ n` is not `pow`. GHC.Real squares and multiplies
//! through the type's own `*`, in `powImpl` and `powImplAcc`'s order, so the result can
//! differ from a correctly rounded power in its last bits: `1.1 ^ 50` is
//! 117.39085287969571 by GHC's multiplications and 117.39085287969579 by `powf`. The
//! interpreter and both native backends share this one implementation.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

/// `base_chirho ^ exponent_chirho` by GHC.Real's repeated squaring, or `None` for a
/// negative exponent, which GHC rejects with `Negative exponent`.
pub fn pow_double_int_chirho(base_chirho: f64, exponent_chirho: i64) -> Option<f64> {
    if exponent_chirho < 0 {
        return None;
    }
    if exponent_chirho == 0 {
        return Some(1.0);
    }
    // powImpl x y, which is x ^ y.
    let (mut x_chirho, mut y_chirho) = (base_chirho, exponent_chirho);
    while y_chirho % 2 == 0 {
        x_chirho *= x_chirho;
        y_chirho /= 2;
    }
    if y_chirho == 1 {
        return Some(x_chirho);
    }
    // powImplAcc (x * x) (y `quot` 2) x, which is (x ^ y) * z.
    let mut z_chirho = x_chirho;
    x_chirho *= x_chirho;
    y_chirho /= 2;
    loop {
        if y_chirho % 2 == 0 {
            x_chirho *= x_chirho;
            y_chirho /= 2;
        } else if y_chirho == 1 {
            return Some(x_chirho * z_chirho);
        } else {
            // GHC's `x * z`; IEEE multiplication is commutative, bit for bit.
            z_chirho *= x_chirho;
            x_chirho *= x_chirho;
            y_chirho /= 2;
        }
    }
}
