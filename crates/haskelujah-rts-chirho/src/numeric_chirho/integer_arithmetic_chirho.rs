// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Checked operations on the opaque Integer representation, with an owner-held allowance.
//! Workflow: next-pipeline-chirho/ghc-implementation-map-chirho.

use crate::integer_literal_chirho::IntegerValueChirho;
use num_bigint::{BigInt as BigIntChirho, Sign as SignChirho};
use std::{fmt, sync::Arc};

/// Bounds the largest single result independently of the aggregate operation allowance.
pub const MAX_INTEGER_BITS_CHIRHO: u64 = 1 << 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerOperationChirho {
    AddChirho,
    SubtractChirho,
    MultiplyChirho,
    NegateChirho,
    AbsoluteChirho,
    SignumChirho,
    EqualChirho,
    NotEqualChirho,
    LessChirho,
    LessEqualChirho,
    GreaterChirho,
    GreaterEqualChirho,
    CompareChirho,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerResultChirho {
    IntegerChirho,
    BoolChirho,
    OrderingChirho,
}

impl IntegerOperationChirho {
    pub fn result_chirho(self) -> IntegerResultChirho {
        match self {
            Self::AddChirho
            | Self::SubtractChirho
            | Self::MultiplyChirho
            | Self::NegateChirho
            | Self::AbsoluteChirho
            | Self::SignumChirho => IntegerResultChirho::IntegerChirho,
            Self::CompareChirho => IntegerResultChirho::OrderingChirho,
            _ => IntegerResultChirho::BoolChirho,
        }
    }

    pub fn arity_chirho(self) -> usize {
        match self {
            Self::AddChirho
            | Self::SubtractChirho
            | Self::MultiplyChirho
            | Self::EqualChirho
            | Self::NotEqualChirho
            | Self::LessChirho
            | Self::LessEqualChirho
            | Self::GreaterChirho
            | Self::GreaterEqualChirho
            | Self::CompareChirho => 2,
            Self::NegateChirho | Self::AbsoluteChirho | Self::SignumChirho => 1,
        }
    }

    /// These are compiler-internal operation keys, never source-library spellings.
    pub fn primitive_chirho(self) -> &'static str {
        match self {
            Self::AddChirho => "%next:integer-add-chirho",
            Self::SubtractChirho => "%next:integer-subtract-chirho",
            Self::MultiplyChirho => "%next:integer-multiply-chirho",
            Self::NegateChirho => "%next:integer-negate-chirho",
            Self::AbsoluteChirho => "%next:integer-absolute-chirho",
            Self::SignumChirho => "%next:integer-signum-chirho",
            Self::EqualChirho => "%next:integer-equal-chirho",
            Self::NotEqualChirho => "%next:integer-not-equal-chirho",
            Self::LessChirho => "%next:integer-less-chirho",
            Self::LessEqualChirho => "%next:integer-less-equal-chirho",
            Self::GreaterChirho => "%next:integer-greater-chirho",
            Self::GreaterEqualChirho => "%next:integer-greater-equal-chirho",
            Self::CompareChirho => "%next:integer-compare-chirho",
        }
    }

    pub fn from_primitive_chirho(name_chirho: &str) -> Option<Self> {
        Some(match name_chirho {
            "%next:integer-add-chirho" => Self::AddChirho,
            "%next:integer-subtract-chirho" => Self::SubtractChirho,
            "%next:integer-multiply-chirho" => Self::MultiplyChirho,
            "%next:integer-negate-chirho" => Self::NegateChirho,
            "%next:integer-absolute-chirho" => Self::AbsoluteChirho,
            "%next:integer-signum-chirho" => Self::SignumChirho,
            "%next:integer-equal-chirho" => Self::EqualChirho,
            "%next:integer-not-equal-chirho" => Self::NotEqualChirho,
            "%next:integer-less-chirho" => Self::LessChirho,
            "%next:integer-less-equal-chirho" => Self::LessEqualChirho,
            "%next:integer-greater-chirho" => Self::GreaterChirho,
            "%next:integer-greater-equal-chirho" => Self::GreaterEqualChirho,
            "%next:integer-compare-chirho" => Self::CompareChirho,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerArithmeticErrorChirho {
    ArityChirho,
    ResourceChirho,
}

impl fmt::Display for IntegerArithmeticErrorChirho {
    fn fmt(&self, formatter_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter_chirho.write_str(match self {
            Self::ArityChirho => "opaque Integer operation has the wrong operand count",
            Self::ResourceChirho => "opaque Integer arithmetic allowance exhausted",
        })
    }
}

/// Work is a conservative limb-operation estimate, not a wall-clock measurement.
/// The output charge is cumulative, including results later reclaimed by the GC.
/// Keep this on the evaluation owner; a new operation must not replenish it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerBudgetChirho {
    work_left_chirho: u64,
    output_left_chirho: u64,
    exhausted_chirho: bool,
}

impl Default for IntegerBudgetChirho {
    fn default() -> Self {
        Self::limited_chirho(1 << 24, 1 << 20)
    }
}

impl IntegerBudgetChirho {
    pub fn limited_chirho(work_chirho: u64, output_limbs_chirho: u64) -> Self {
        Self {
            work_left_chirho: work_chirho,
            output_left_chirho: output_limbs_chirho,
            exhausted_chirho: false,
        }
    }

    pub fn exhausted_chirho(&self) -> bool {
        self.exhausted_chirho
    }

    pub(crate) fn reserve_chirho(
        &mut self,
        work_chirho: u64,
        output_bits_chirho: u64,
    ) -> Result<(), IntegerArithmeticErrorChirho> {
        let output_limbs_chirho = output_bits_chirho.div_ceil(64).max(1);
        if self.exhausted_chirho
            || output_bits_chirho > MAX_INTEGER_BITS_CHIRHO
            || work_chirho > self.work_left_chirho
            || output_limbs_chirho > self.output_left_chirho
        {
            self.exhausted_chirho = true;
            return Err(IntegerArithmeticErrorChirho::ResourceChirho);
        }
        self.work_left_chirho -= work_chirho;
        self.output_left_chirho -= output_limbs_chirho;
        Ok(())
    }
}

impl IntegerValueChirho {
    /// Reserve worst-case input-limb work and one bounded constructor result before comparing.
    /// No operand is cloned or normalized; failure latches the same evaluation-owner budget.
    pub fn compare_bounded_chirho(
        &self,
        right_chirho: &Self,
        budget_chirho: &mut IntegerBudgetChirho,
    ) -> Result<std::cmp::Ordering, IntegerArithmeticErrorChirho> {
        let left_bits_chirho = self.0.bits();
        let right_bits_chirho = right_chirho.0.bits();
        if left_bits_chirho > MAX_INTEGER_BITS_CHIRHO || right_bits_chirho > MAX_INTEGER_BITS_CHIRHO
        {
            budget_chirho.exhausted_chirho = true;
            return Err(IntegerArithmeticErrorChirho::ResourceChirho);
        }
        budget_chirho.reserve_chirho(
            left_bits_chirho.div_ceil(64).max(1) + right_bits_chirho.div_ceil(64).max(1) + 1,
            1,
        )?;
        Ok(self.0.cmp(&right_chirho.0))
    }

    /// Reserve the whole operation before materializing a result. Inputs remain immutable on failure.
    pub fn apply_chirho(
        &self,
        operation_chirho: IntegerOperationChirho,
        right_chirho: Option<&Self>,
        budget_chirho: &mut IntegerBudgetChirho,
    ) -> Result<Self, IntegerArithmeticErrorChirho> {
        if operation_chirho.result_chirho() != IntegerResultChirho::IntegerChirho
            || (operation_chirho.arity_chirho() == 2) != right_chirho.is_some()
        {
            return Err(IntegerArithmeticErrorChirho::ArityChirho);
        }
        let left_bits_chirho = self.0.bits();
        let right_bits_chirho = right_chirho.map_or(0, |value_chirho| value_chirho.0.bits());
        if left_bits_chirho > MAX_INTEGER_BITS_CHIRHO || right_bits_chirho > MAX_INTEGER_BITS_CHIRHO
        {
            budget_chirho.exhausted_chirho = true;
            return Err(IntegerArithmeticErrorChirho::ResourceChirho);
        }
        let left_limbs_chirho = left_bits_chirho.div_ceil(64).max(1);
        let right_limbs_chirho = right_bits_chirho.div_ceil(64).max(1);
        let (work_chirho, output_bits_chirho) = match operation_chirho {
            IntegerOperationChirho::AddChirho | IntegerOperationChirho::SubtractChirho => (
                left_limbs_chirho + right_limbs_chirho + 1,
                left_bits_chirho.max(right_bits_chirho) + 1,
            ),
            IntegerOperationChirho::MultiplyChirho => (
                4 * left_limbs_chirho * right_limbs_chirho + left_limbs_chirho + right_limbs_chirho,
                if left_bits_chirho == 0 || right_bits_chirho == 0 {
                    0
                } else {
                    left_bits_chirho + right_bits_chirho
                },
            ),
            IntegerOperationChirho::NegateChirho | IntegerOperationChirho::AbsoluteChirho => {
                (left_limbs_chirho + 1, left_bits_chirho)
            }
            IntegerOperationChirho::SignumChirho => (1, 1),
            _ => return Err(IntegerArithmeticErrorChirho::ArityChirho),
        };
        budget_chirho.reserve_chirho(work_chirho, output_bits_chirho)?;
        let result_chirho = match operation_chirho {
            IntegerOperationChirho::AddChirho => {
                self.0.as_ref()
                    + right_chirho
                        .ok_or(IntegerArithmeticErrorChirho::ArityChirho)?
                        .0
                        .as_ref()
            }
            IntegerOperationChirho::SubtractChirho => {
                self.0.as_ref()
                    - right_chirho
                        .ok_or(IntegerArithmeticErrorChirho::ArityChirho)?
                        .0
                        .as_ref()
            }
            IntegerOperationChirho::MultiplyChirho => {
                self.0.as_ref()
                    * right_chirho
                        .ok_or(IntegerArithmeticErrorChirho::ArityChirho)?
                        .0
                        .as_ref()
            }
            IntegerOperationChirho::NegateChirho => -self.0.as_ref(),
            IntegerOperationChirho::AbsoluteChirho => {
                if self.0.sign() != SignChirho::Minus {
                    return Ok(self.clone());
                }
                -self.0.as_ref()
            }
            IntegerOperationChirho::SignumChirho => BigIntChirho::from(match self.0.sign() {
                SignChirho::Minus => -1,
                SignChirho::NoSign => 0,
                SignChirho::Plus => 1,
            }),
            _ => return Err(IntegerArithmeticErrorChirho::ArityChirho),
        };
        Ok(Self(Arc::new(result_chirho)))
    }
}

#[cfg(test)]
#[path = "integer_arithmetic_tests_chirho.rs"]
mod tests_chirho;
