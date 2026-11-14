// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Int primitives consume only values already forced by the ordinary continuation stack.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use crate::int_arithmetic_chirho::{IntOperationChirho, IntOutcomeChirho};

impl MachineChirho {
    pub(super) fn eval_int_chirho(
        &mut self,
        operation_chirho: IntOperationChirho,
        arguments_chirho: &[ValueChirho],
    ) -> Result<ValueChirho, EvalErrorChirho> {
        let error_chirho = || EvalErrorChirho::TypeErrorChirho {
            message_chirho: "opaque Int primitive operand or arity invariant failed".into(),
        };
        if arguments_chirho.len() != operation_chirho.arity_chirho() {
            return Err(error_chirho());
        }
        let left_chirho = self.evaluated_primitive_operand_chirho(&arguments_chirho[0])?;
        let outcome_chirho = if operation_chirho == IntOperationChirho::FromIntegerChirho {
            let ValueChirho::IntegerChirho(left_chirho) = left_chirho else {
                return Err(error_chirho());
            };
            left_chirho
                .wrapping_int_chirho(&mut self.integer_budget_chirho)
                .map(IntOutcomeChirho::IntChirho)
        } else {
            let ValueChirho::IntChirho(left_chirho) = left_chirho else {
                return Err(error_chirho());
            };
            let right_chirho = arguments_chirho
                .get(1)
                .map(|argument_chirho| {
                    match self.evaluated_primitive_operand_chirho(argument_chirho)? {
                        ValueChirho::IntChirho(value_chirho) => Ok(value_chirho),
                        _ => Err(error_chirho()),
                    }
                })
                .transpose()?;
            operation_chirho.apply_chirho(
                left_chirho,
                right_chirho,
                &mut self.integer_budget_chirho,
            )
        }
        .map_err(|error_chirho| {
            EvalErrorChirho::PrimFailChirho(PrimErrorChirho::IntegerArithmeticChirho(error_chirho))
        })?;
        Ok(match outcome_chirho {
            IntOutcomeChirho::IntChirho(value_chirho) => ValueChirho::IntChirho(value_chirho),
            IntOutcomeChirho::BoolChirho(value_chirho) => ValueChirho::BoolChirho(value_chirho),
            IntOutcomeChirho::OrderingChirho(value_chirho) => {
                self.make_ordering_chirho(value_chirho)
            }
        })
    }
}
