// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Evaluated scalar leaves only. Lists/Chars are demanded by ordinary Core cases.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::{EvalErrorChirho, InfoTagChirho, MachineChirho, PrimErrorChirho, ValueChirho};
use crate::io_chirho::{IoErrorChirho, IoOperationChirho, STDOUT_KEY_CHIRHO, WORLD_KEY_CHIRHO};
impl MachineChirho {
    fn io_token_chirho(&self, value_chirho: &ValueChirho, expected_chirho: &str) -> bool {
        let ValueChirho::HeapPtrChirho(address_chirho) = value_chirho else {
            return false;
        };
        let closure_chirho = self
            .heap_chirho
            .read_chirho(self.heap_chirho.follow_ind_chirho(*address_chirho));
        closure_chirho.info_chirho.tag_chirho == InfoTagChirho::ConChirho
            && closure_chirho.info_chirho.name_chirho == expected_chirho
            && closure_chirho.payload_chirho.is_empty()
    }
    pub(super) fn eval_next_io_chirho(
        &mut self,
        operation_chirho: IoOperationChirho,
        arguments_chirho: &[ValueChirho],
    ) -> Result<ValueChirho, EvalErrorChirho> {
        let failure_chirho = || {
            EvalErrorChirho::PrimFailChirho(PrimErrorChirho::NextIoChirho(
                IoErrorChirho::ContractChirho,
            ))
        };
        let (Some(handle_chirho), Some(world_chirho)) =
            (arguments_chirho.first(), arguments_chirho.last())
        else {
            return Err(failure_chirho());
        };
        if !self.io_token_chirho(handle_chirho, STDOUT_KEY_CHIRHO)
            || !self.io_token_chirho(world_chirho, WORLD_KEY_CHIRHO)
        {
            return Err(failure_chirho());
        }
        let result_chirho = match operation_chirho {
            IoOperationChirho::PREPARE_CHIRHO if arguments_chirho.len() == 2 => self
                .next_io_chirho
                .prepare_chirho(&mut self.io_output_chirho),
            IoOperationChirho::APPEND_CHIRHO if arguments_chirho.len() == 3 => {
                let ValueChirho::CharChirho(character_chirho) =
                    self.evaluated_primitive_operand_chirho(&arguments_chirho[1])?
                else {
                    return Err(failure_chirho());
                };
                self.next_io_chirho.append_chirho(character_chirho)
            }
            IoOperationChirho::FinishChirho if arguments_chirho.len() == 3 => {
                let ValueChirho::BoolChirho(newline_chirho) =
                    self.evaluated_primitive_operand_chirho(&arguments_chirho[1])?
                else {
                    return Err(failure_chirho());
                };
                self.next_io_chirho
                    .finish_chirho(&mut self.io_output_chirho, newline_chirho)
            }
            _ => return Err(failure_chirho()),
        };
        result_chirho.map_err(|error_chirho| {
            EvalErrorChirho::PrimFailChirho(PrimErrorChirho::NextIoChirho(error_chirho))
        })?;
        Ok(world_chirho.clone())
    }
}

#[cfg(test)]
mod tests_chirho;
