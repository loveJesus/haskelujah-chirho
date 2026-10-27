// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Demand adapters for strict primitives, not for lazy producers or IO actions.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use super::{EvalErrorChirho, MachineChirho, PrimOpKindChirho, ValueChirho};

impl MachineChirho {
    /// Interpret the integer-shaped primitives used by the shared finite-range
    /// helpers when their current value is an `Ordering` constructor. Native
    /// code represents LT/EQ/GT as 0/1/2 already; the interpreter keeps them
    /// boxed, so this adapter preserves the constructor representation.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn try_apply_ordering_enum_prim_chirho(
        &mut self,
        op_chirho: PrimOpKindChirho,
        args_chirho: &[ValueChirho],
    ) -> Result<Option<ValueChirho>, EvalErrorChirho> {
        if !matches!(
            op_chirho,
            PrimOpKindChirho::LtIntChirho
                | PrimOpKindChirho::GtIntChirho
                | PrimOpKindChirho::SubIntChirho
                | PrimOpKindChirho::EnumStepChirho
        ) || args_chirho.len() != 2
        {
            return Ok(None);
        }

        let left_chirho = self.force_to_prim_chirho(args_chirho[0].clone());
        if op_chirho == PrimOpKindChirho::EnumStepChirho {
            let Some(left_tag_chirho) = self.ordering_tag_chirho(&left_chirho) else {
                return Ok(None);
            };
            let ValueChirho::IntChirho(step_chirho) =
                self.force_to_prim_chirho(args_chirho[1].clone())
            else {
                return Ok(None);
            };
            let next_tag_chirho = left_tag_chirho.wrapping_add(step_chirho);
            let next_chirho = match next_tag_chirho {
                0 => self.make_ordering_chirho(std::cmp::Ordering::Less),
                1 => self.make_ordering_chirho(std::cmp::Ordering::Equal),
                2 => self.make_ordering_chirho(std::cmp::Ordering::Greater),
                // The shared range loop compares this one-step sentinel with
                // its boxed bound before it could become a list element.
                _ => ValueChirho::IntChirho(next_tag_chirho),
            };
            return Ok(Some(next_chirho));
        }

        let right_chirho = self.force_to_prim_chirho(args_chirho[1].clone());
        let Some((left_tag_chirho, left_is_ordering_chirho)) =
            self.ordering_enum_scalar_chirho(&left_chirho)
        else {
            return Ok(None);
        };
        let Some((right_tag_chirho, right_is_ordering_chirho)) =
            self.ordering_enum_scalar_chirho(&right_chirho)
        else {
            return Ok(None);
        };
        if !left_is_ordering_chirho && !right_is_ordering_chirho {
            return Ok(None);
        }
        let result_chirho = match op_chirho {
            PrimOpKindChirho::LtIntChirho => {
                ValueChirho::BoolChirho(left_tag_chirho < right_tag_chirho)
            }
            PrimOpKindChirho::GtIntChirho => {
                ValueChirho::BoolChirho(left_tag_chirho > right_tag_chirho)
            }
            PrimOpKindChirho::SubIntChirho => {
                ValueChirho::IntChirho(left_tag_chirho.wrapping_sub(right_tag_chirho))
            }
            _ => unreachable!("the Ordering Enum adapter filters its primitive set"),
        };
        Ok(Some(result_chirho))
    }

    fn ordering_enum_scalar_chirho(&self, value_chirho: &ValueChirho) -> Option<(i64, bool)> {
        match value_chirho {
            ValueChirho::IntChirho(value_chirho) => Some((*value_chirho, false)),
            _ => self
                .ordering_tag_chirho(value_chirho)
                .map(|tag_chirho| (tag_chirho, true)),
        }
    }

    fn ordering_tag_chirho(&self, value_chirho: &ValueChirho) -> Option<i64> {
        let ValueChirho::HeapPtrChirho(address_chirho) = value_chirho else {
            return None;
        };
        let address_chirho = self.heap_chirho.follow_ind_chirho(*address_chirho);
        match self
            .heap_chirho
            .read_chirho(address_chirho)
            .info_chirho
            .name_chirho
            .as_str()
        {
            "LT" => Some(0),
            "EQ" => Some(1),
            "GT" => Some(2),
            _ => None,
        }
    }

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
