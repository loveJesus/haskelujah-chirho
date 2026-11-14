// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC's 64-bit Int boundary; all arithmetic wraps and shares the evaluation allowance.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use crate::{
    integer_arithmetic_chirho::{IntegerArithmeticErrorChirho, IntegerBudgetChirho},
    integer_literal_chirho::IntegerValueChirho,
};
use num_bigint::Sign as SignChirho;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntOperationChirho {
    AddChirho,
    SubtractChirho,
    MultiplyChirho,
    NegateChirho,
    FromIntegerChirho,
    EqualChirho,
    NotEqualChirho,
    LessChirho,
    LessEqualChirho,
    GreaterChirho,
    GreaterEqualChirho,
    CompareChirho,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntOutcomeChirho {
    IntChirho(i64),
    BoolChirho(bool),
    OrderingChirho(std::cmp::Ordering),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntTypeChirho {
    IntChirho,
    IntegerChirho,
    BoolChirho,
    OrderingChirho,
}

impl IntOperationChirho {
    pub fn argument_chirho(self) -> IntTypeChirho {
        if self == Self::FromIntegerChirho {
            IntTypeChirho::IntegerChirho
        } else {
            IntTypeChirho::IntChirho
        }
    }
    pub fn result_chirho(self) -> IntTypeChirho {
        match self {
            Self::AddChirho
            | Self::SubtractChirho
            | Self::MultiplyChirho
            | Self::NegateChirho
            | Self::FromIntegerChirho => IntTypeChirho::IntChirho,
            Self::CompareChirho => IntTypeChirho::OrderingChirho,
            Self::EqualChirho
            | Self::NotEqualChirho
            | Self::LessChirho
            | Self::LessEqualChirho
            | Self::GreaterChirho
            | Self::GreaterEqualChirho => IntTypeChirho::BoolChirho,
        }
    }
    pub const ALL_CHIRHO: [Self; 12] = [
        Self::AddChirho,
        Self::SubtractChirho,
        Self::MultiplyChirho,
        Self::NegateChirho,
        Self::FromIntegerChirho,
        Self::EqualChirho,
        Self::NotEqualChirho,
        Self::LessChirho,
        Self::LessEqualChirho,
        Self::GreaterChirho,
        Self::GreaterEqualChirho,
        Self::CompareChirho,
    ];
    pub fn arity_chirho(self) -> usize {
        if matches!(self, Self::NegateChirho | Self::FromIntegerChirho) {
            1
        } else {
            2
        }
    }
    pub fn primitive_chirho(self) -> &'static str {
        match self {
            Self::AddChirho => "%next:int-add-chirho",
            Self::SubtractChirho => "%next:int-subtract-chirho",
            Self::MultiplyChirho => "%next:int-multiply-chirho",
            Self::NegateChirho => "%next:int-negate-chirho",
            Self::FromIntegerChirho => "%next:int-from-integer-chirho",
            Self::EqualChirho => "%next:int-equal-chirho",
            Self::NotEqualChirho => "%next:int-not-equal-chirho",
            Self::LessChirho => "%next:int-less-chirho",
            Self::LessEqualChirho => "%next:int-less-equal-chirho",
            Self::GreaterChirho => "%next:int-greater-chirho",
            Self::GreaterEqualChirho => "%next:int-greater-equal-chirho",
            Self::CompareChirho => "%next:int-compare-chirho",
        }
    }
    pub fn from_primitive_chirho(name_chirho: &str) -> Option<Self> {
        Self::ALL_CHIRHO
            .into_iter()
            .find(|operation_chirho| operation_chirho.primitive_chirho() == name_chirho)
    }
    pub fn apply_chirho(
        self,
        left_chirho: i64,
        right_chirho: Option<i64>,
        budget_chirho: &mut IntegerBudgetChirho,
    ) -> Result<IntOutcomeChirho, IntegerArithmeticErrorChirho> {
        if self == Self::FromIntegerChirho || (self.arity_chirho() == 2) != right_chirho.is_some() {
            return Err(IntegerArithmeticErrorChirho::ArityChirho);
        }
        budget_chirho.reserve_chirho(3, 64)?;
        let right_chirho = right_chirho.unwrap_or(0);
        Ok(match self {
            Self::AddChirho => IntOutcomeChirho::IntChirho(left_chirho.wrapping_add(right_chirho)),
            Self::SubtractChirho => {
                IntOutcomeChirho::IntChirho(left_chirho.wrapping_sub(right_chirho))
            }
            Self::MultiplyChirho => {
                IntOutcomeChirho::IntChirho(left_chirho.wrapping_mul(right_chirho))
            }
            Self::NegateChirho => IntOutcomeChirho::IntChirho(left_chirho.wrapping_neg()),
            Self::EqualChirho => IntOutcomeChirho::BoolChirho(left_chirho == right_chirho),
            Self::NotEqualChirho => IntOutcomeChirho::BoolChirho(left_chirho != right_chirho),
            Self::LessChirho => IntOutcomeChirho::BoolChirho(left_chirho < right_chirho),
            Self::LessEqualChirho => IntOutcomeChirho::BoolChirho(left_chirho <= right_chirho),
            Self::GreaterChirho => IntOutcomeChirho::BoolChirho(left_chirho > right_chirho),
            Self::GreaterEqualChirho => IntOutcomeChirho::BoolChirho(left_chirho >= right_chirho),
            Self::CompareChirho => IntOutcomeChirho::OrderingChirho(left_chirho.cmp(&right_chirho)),
            Self::FromIntegerChirho => return Err(IntegerArithmeticErrorChirho::ArityChirho),
        })
    }
}

impl IntegerValueChirho {
    /// Inspect at most one magnitude word, then convert two's complement modulo 2^64.
    pub fn wrapping_int_chirho(
        &self,
        budget_chirho: &mut IntegerBudgetChirho,
    ) -> Result<i64, IntegerArithmeticErrorChirho> {
        budget_chirho.reserve_chirho(3, 64)?;
        let magnitude_chirho = self.0.magnitude().iter_u64_digits().next().unwrap_or(0);
        Ok(if self.0.sign() == SignChirho::Minus {
            magnitude_chirho.wrapping_neg() as i64
        } else {
            magnitude_chirho as i64
        })
    }
}

#[cfg(test)]
#[path = "int_arithmetic_tests_chirho.rs"]
mod tests_chirho;
