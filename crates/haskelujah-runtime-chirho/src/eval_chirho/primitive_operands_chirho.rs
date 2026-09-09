// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Demand adapters for strict primitives, not for lazy producers or IO actions.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use super::{EvalErrorChirho, MachineChirho, PrimOpKindChirho, ValueChirho};

impl MachineChirho {
    pub(super) fn resolve_primitive_operands_chirho(
        &mut self,
        op_chirho: PrimOpKindChirho,
        args_chirho: &[ValueChirho],
    ) -> Result<Vec<ValueChirho>, EvalErrorChirho> {
        // Read consumes a String, whether packed or represented as a lazy [Char].
        // Resolve before the legacy unbox helper, which cannot carry forcing errors.
        // The primitive still validates the text; conversion does not invent a value.
        if matches!(
            op_chirho,
            PrimOpKindChirho::ReadIntChirho
                | PrimOpKindChirho::ReadFloatChirho
                | PrimOpKindChirho::ReadBoolChirho
        ) && let [argument_chirho] = args_chirho
            && let Some(text_chirho) = self.try_resolve_value_to_string_chirho(argument_chirho)?
        {
            return Ok(vec![ValueChirho::StringChirho(text_chirho)]);
        }

        let mut resolved_args_chirho: Vec<ValueChirho> = args_chirho
            .iter()
            .cloned()
            .map(|value_chirho| self.force_to_prim_chirho(value_chirho))
            .collect();
        let is_string_compare_chirho = matches!(
            op_chirho,
            PrimOpKindChirho::EqStrChirho
                | PrimOpKindChirho::LtStrChirho
                | PrimOpKindChirho::EqIntChirho
                | PrimOpKindChirho::NeIntChirho
                | PrimOpKindChirho::LtIntChirho
                | PrimOpKindChirho::LeIntChirho
                | PrimOpKindChirho::GtIntChirho
                | PrimOpKindChirho::GeIntChirho
        );
        if is_string_compare_chirho && resolved_args_chirho.len() == 2 {
            let left_chirho = self.try_resolve_value_to_string_chirho(&resolved_args_chirho[0])?;
            let right_chirho = self.try_resolve_value_to_string_chirho(&resolved_args_chirho[1])?;
            if let (Some(left_chirho), Some(right_chirho)) = (left_chirho, right_chirho) {
                resolved_args_chirho[0] = ValueChirho::StringChirho(left_chirho);
                resolved_args_chirho[1] = ValueChirho::StringChirho(right_chirho);
            }
        }
        Ok(resolved_args_chirho)
    }
}
