// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Decimal text for opaque source-owned numeric representations, without heap references.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use crate::{
    int_arithmetic_chirho::IntTypeChirho,
    integer_arithmetic_chirho::{
        IntegerArithmeticErrorChirho, IntegerBudgetChirho, MAX_INTEGER_BITS_CHIRHO,
    },
    integer_literal_chirho::IntegerValueChirho,
};
use num_bigint::Sign as SignChirho;

pub const MAX_DECIMAL_TEXT_BYTES_CHIRHO: u64 = 1 << 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShowNumberOperationChirho {
    IntChirho,
    IntegerChirho,
}

impl ShowNumberOperationChirho {
    pub const ALL_CHIRHO: [Self; 2] = [Self::IntChirho, Self::IntegerChirho];
    pub fn argument_chirho(self) -> IntTypeChirho {
        match self {
            Self::IntChirho => IntTypeChirho::IntChirho,
            Self::IntegerChirho => IntTypeChirho::IntegerChirho,
        }
    }
    pub fn primitive_chirho(self) -> &'static str {
        match self {
            Self::IntChirho => "%next:show-int-chirho",
            Self::IntegerChirho => "%next:show-integer-chirho",
        }
    }
    pub fn from_primitive_chirho(name_chirho: &str) -> Option<Self> {
        Self::ALL_CHIRHO
            .into_iter()
            .find(|operation_chirho| operation_chirho.primitive_chirho() == name_chirho)
    }
}

impl IntegerBudgetChirho {
    /// Text expands through the existing list carrier: charge three cells per ASCII byte plus its terminator.
    /// Conversion work is a conservative quadratic limb bound, not a timing claim.
    fn reserve_decimal_chirho(
        &mut self,
        bits_chirho: u64,
        negative_chirho: bool,
        parentheses_chirho: bool,
    ) -> Result<(), IntegerArithmeticErrorChirho> {
        let digits_chirho = bits_chirho.saturating_mul(30_103).div_ceil(100_000).max(1);
        let bytes_chirho =
            digits_chirho + u64::from(negative_chirho) + 2 * u64::from(parentheses_chirho);
        let limbs_chirho = bits_chirho.div_ceil(64).max(1);
        let work_chirho = limbs_chirho
            .saturating_mul(limbs_chirho)
            .saturating_mul(64)
            .saturating_add(bytes_chirho.saturating_mul(3));
        if bits_chirho > MAX_INTEGER_BITS_CHIRHO || bytes_chirho > MAX_DECIMAL_TEXT_BYTES_CHIRHO {
            return self.fail_resource_chirho();
        }
        self.reserve_units_chirho(
            work_chirho,
            bytes_chirho.saturating_mul(3).saturating_add(1),
        )
    }
}

pub fn show_int_chirho(
    precedence_chirho: i64,
    number_chirho: i64,
    budget_chirho: &mut IntegerBudgetChirho,
) -> Result<String, IntegerArithmeticErrorChirho> {
    let negative_chirho = number_chirho < 0;
    let parentheses_chirho = precedence_chirho > 6 && negative_chirho;
    budget_chirho.reserve_decimal_chirho(64, negative_chirho, parentheses_chirho)?;
    // Rust's signed formatting handles i64::MIN without a narrowing or overflowing abs.
    let text_chirho = number_chirho.to_string();
    Ok(parenthesize_chirho(text_chirho, parentheses_chirho))
}

impl IntegerValueChirho {
    pub fn show_decimal_chirho(
        &self,
        precedence_chirho: i64,
        budget_chirho: &mut IntegerBudgetChirho,
    ) -> Result<String, IntegerArithmeticErrorChirho> {
        let negative_chirho = self.0.sign() == SignChirho::Minus;
        let parentheses_chirho = precedence_chirho > 6 && negative_chirho;
        budget_chirho.reserve_decimal_chirho(self.0.bits(), negative_chirho, parentheses_chirho)?;
        let text_chirho = self.0.to_str_radix(10);
        Ok(parenthesize_chirho(text_chirho, parentheses_chirho))
    }
}

fn parenthesize_chirho(text_chirho: String, parentheses_chirho: bool) -> String {
    if parentheses_chirho {
        format!("({text_chirho})")
    } else {
        text_chirho
    }
}

#[cfg(test)]
#[path = "show_number_tests_chirho.rs"]
mod tests_chirho;
