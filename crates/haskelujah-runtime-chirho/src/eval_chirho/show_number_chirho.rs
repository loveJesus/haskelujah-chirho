// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Only evaluated scalar operands cross the pure formatting boundary; tails stay in source Core.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::{EvalErrorChirho, MachineChirho, PrimErrorChirho, PrimOpKindChirho, ValueChirho};
use crate::{
    integer_arithmetic_chirho::IntegerArithmeticErrorChirho,
    show_number_chirho::ShowNumberOperationChirho,
};

impl MachineChirho {
    pub(super) fn eval_show_number_chirho(
        &mut self,
        operation_chirho: ShowNumberOperationChirho,
        arguments_chirho: &[ValueChirho],
    ) -> Result<ValueChirho, EvalErrorChirho> {
        let [precedence_chirho, number_chirho] = arguments_chirho else {
            return Err(EvalErrorChirho::PrimFailChirho(
                PrimErrorChirho::IntegerArithmeticChirho(IntegerArithmeticErrorChirho::ArityChirho),
            ));
        };
        let mismatch_chirho = || {
            EvalErrorChirho::PrimFailChirho(PrimErrorChirho::TypeMismatchChirho {
                op_chirho: PrimOpKindChirho::ShowNumberChirho(operation_chirho),
                expected_chirho: "evaluated numeric formatting operands",
                got_chirho: "a value outside the checked formatting contract".into(),
            })
        };
        let ValueChirho::IntChirho(precedence_chirho) =
            self.evaluated_primitive_operand_chirho(precedence_chirho)?
        else {
            return Err(mismatch_chirho());
        };
        let number_chirho = self.evaluated_primitive_operand_chirho(number_chirho)?;
        let text_chirho = match (operation_chirho, number_chirho) {
            (ShowNumberOperationChirho::IntChirho, ValueChirho::IntChirho(number_chirho)) => {
                crate::show_number_chirho::show_int_chirho(
                    precedence_chirho,
                    number_chirho,
                    &mut self.integer_budget_chirho,
                )
            }
            (
                ShowNumberOperationChirho::IntegerChirho,
                ValueChirho::IntegerChirho(number_chirho),
            ) => number_chirho
                .show_decimal_chirho(precedence_chirho, &mut self.integer_budget_chirho),
            _ => return Err(mismatch_chirho()),
        }
        .map_err(|error_chirho| {
            EvalErrorChirho::PrimFailChirho(PrimErrorChirho::IntegerArithmeticChirho(error_chirho))
        })?;
        Ok(ValueChirho::StringChirho(text_chirho))
    }
}

#[cfg(test)]
#[path = "show_number_tests_chirho.rs"]
mod tests_chirho;
