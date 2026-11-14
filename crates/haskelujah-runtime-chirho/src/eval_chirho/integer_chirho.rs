// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Opaque arithmetic consumes WHNF operands and a machine-owned cumulative allowance.
//! Workflow: next-pipeline-chirho/ghc-implementation-map-chirho.

use super::{EvalErrorChirho, MachineChirho, PrimErrorChirho, PrimOpKindChirho, ValueChirho};
use crate::integer_arithmetic_chirho::{
    IntegerArithmeticErrorChirho, IntegerOperationChirho, IntegerResultChirho,
};
use crate::integer_literal_chirho::IntegerValueChirho;
use crate::value_chirho::InfoTagChirho;

impl MachineChirho {
    pub(super) fn eval_integer_chirho(
        &mut self,
        operation_chirho: IntegerOperationChirho,
        arguments_chirho: &[ValueChirho],
    ) -> Result<ValueChirho, EvalErrorChirho> {
        if arguments_chirho.len() != operation_chirho.arity_chirho() {
            return Err(EvalErrorChirho::PrimFailChirho(
                PrimErrorChirho::IntegerArithmeticChirho(IntegerArithmeticErrorChirho::ArityChirho),
            ));
        }
        let left_chirho = self.integer_operand_chirho(operation_chirho, &arguments_chirho[0])?;
        let right_chirho = arguments_chirho
            .get(1)
            .map(|argument_chirho| self.integer_operand_chirho(operation_chirho, argument_chirho))
            .transpose()?;
        if operation_chirho.result_chirho() != IntegerResultChirho::IntegerChirho {
            let right_chirho = right_chirho
                .as_ref()
                .ok_or(EvalErrorChirho::PrimFailChirho(
                    PrimErrorChirho::IntegerArithmeticChirho(
                        IntegerArithmeticErrorChirho::ArityChirho,
                    ),
                ))?;
            let ordering_chirho = left_chirho
                .compare_bounded_chirho(right_chirho, &mut self.integer_budget_chirho)
                .map_err(|error_chirho| {
                    EvalErrorChirho::PrimFailChirho(PrimErrorChirho::IntegerArithmeticChirho(
                        error_chirho,
                    ))
                })?;
            if operation_chirho == IntegerOperationChirho::CompareChirho {
                return Ok(self.make_ordering_chirho(ordering_chirho));
            }
            let boolean_chirho = match operation_chirho {
                IntegerOperationChirho::EqualChirho => ordering_chirho.is_eq(),
                IntegerOperationChirho::NotEqualChirho => !ordering_chirho.is_eq(),
                IntegerOperationChirho::LessChirho => ordering_chirho.is_lt(),
                IntegerOperationChirho::LessEqualChirho => !ordering_chirho.is_gt(),
                IntegerOperationChirho::GreaterChirho => ordering_chirho.is_gt(),
                IntegerOperationChirho::GreaterEqualChirho => !ordering_chirho.is_lt(),
                _ => {
                    return Err(EvalErrorChirho::TypeErrorChirho {
                        message_chirho: "Integer comparison result contract mismatch".into(),
                    });
                }
            };
            return Ok(ValueChirho::BoolChirho(boolean_chirho));
        }
        left_chirho
            .apply_chirho(
                operation_chirho,
                right_chirho.as_ref(),
                &mut self.integer_budget_chirho,
            )
            .map(ValueChirho::IntegerChirho)
            .map_err(|error_chirho| {
                EvalErrorChirho::PrimFailChirho(PrimErrorChirho::IntegerArithmeticChirho(
                    error_chirho,
                ))
            })
    }

    fn integer_operand_chirho(
        &self,
        operation_chirho: IntegerOperationChirho,
        argument_chirho: &ValueChirho,
    ) -> Result<IntegerValueChirho, EvalErrorChirho> {
        let value_chirho = match argument_chirho {
            ValueChirho::HeapPtrChirho(address_chirho) => {
                let address_chirho = self.heap_chirho.follow_ind_chirho(*address_chirho);
                if self
                    .heap_chirho
                    .read_chirho(address_chirho)
                    .info_chirho
                    .tag_chirho
                    != InfoTagChirho::ConChirho
                {
                    return Err(EvalErrorChirho::TypeErrorChirho {
                        message_chirho: "opaque Integer primitive received an unevaluated operand"
                            .into(),
                    });
                }
                self.resolve_heap_value_chirho(&ValueChirho::HeapPtrChirho(address_chirho))
            }
            value_chirho => value_chirho.clone(),
        };
        match value_chirho {
            ValueChirho::IntegerChirho(value_chirho) => Ok(value_chirho),
            _ => Err(EvalErrorChirho::PrimFailChirho(
                PrimErrorChirho::TypeMismatchChirho {
                    op_chirho: PrimOpKindChirho::IntegerArithmeticChirho(operation_chirho),
                    expected_chirho: "an opaque Integer",
                    got_chirho: "a non-Integer runtime value".into(),
                },
            )),
        }
    }
}
