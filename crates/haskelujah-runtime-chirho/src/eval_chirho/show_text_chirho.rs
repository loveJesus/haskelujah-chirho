// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `showStr#` on a heap `[Char]` (row 557). The generated `[Char]` Show row hands its
//! argument to `showStr#`. A heap list is read here cell by cell, forcing each tail and
//! character, including a literal tail (`'a' : "bc"` is `"abc"`), and shown by the
//! renderer native code shares. A cell that holds no character fails the program, named,
//! where the universal show would print some other wrong text. A value that is no list
//! keeps the universal show.
//! workflow: print-show-evidence-chirho

use haskelujah_rts_chirho::show_string_chirho;

use super::{EvalErrorChirho, MachineChirho, ValueChirho};
use crate::prim_chirho::PrimErrorChirho;
use crate::stack_chirho::PrimOpKindChirho;

/// The named failure for a `[Char]` cell that holds no character.
fn not_a_character_chirho(value_chirho: &ValueChirho) -> EvalErrorChirho {
    EvalErrorChirho::PrimFailChirho(PrimErrorChirho::TypeMismatchChirho {
        op_chirho: PrimOpKindChirho::ShowStrChirho,
        expected_chirho: "Char",
        got_chirho: format!("{value_chirho}"),
    })
}

impl MachineChirho {
    /// GHC's `show` for a heap `[Char]`, or None when the value is not a list.
    /// workflow: print-show-evidence-chirho
    pub(super) fn show_heap_string_chirho(
        &mut self,
        value_chirho: &ValueChirho,
    ) -> Result<Option<String>, EvalErrorChirho> {
        let ValueChirho::HeapPtrChirho(addr_chirho) = value_chirho else {
            return Ok(None);
        };
        let mut current_chirho = self.force_addr_to_whnf_chirho(*addr_chirho)?;
        let mut code_points_chirho = Vec::new();
        loop {
            let cell_chirho = self.heap_chirho.read_chirho(current_chirho).clone();
            match (
                cell_chirho.info_chirho.name_chirho.as_str(),
                cell_chirho.payload_chirho.as_slice(),
            ) {
                ("[]", _) => break,
                (":", [head_chirho, tail_chirho, ..]) => {
                    code_points_chirho.push(self.list_character_chirho(head_chirho)?);
                    match tail_chirho {
                        ValueChirho::HeapPtrChirho(tail_addr_chirho) => {
                            current_chirho = self.force_addr_to_whnf_chirho(*tail_addr_chirho)?;
                        }
                        ValueChirho::StringChirho(literal_chirho) => {
                            code_points_chirho.extend(literal_chirho.chars().map(u32::from));
                            break;
                        }
                        other_chirho => return Err(not_a_character_chirho(other_chirho)),
                    }
                }
                // Only the outermost value may be something other than a list.
                _ if code_points_chirho.is_empty() => return Ok(None),
                _ => {
                    return Err(not_a_character_chirho(&ValueChirho::HeapPtrChirho(
                        current_chirho,
                    )));
                }
            }
        }
        Ok(Some(show_string_chirho(code_points_chirho)))
    }

    /// The code point a `[Char]` cell holds, forcing it when it is a thunk.
    fn list_character_chirho(&mut self, head_chirho: &ValueChirho) -> Result<u32, EvalErrorChirho> {
        let ValueChirho::HeapPtrChirho(head_addr_chirho) = head_chirho else {
            return match head_chirho {
                ValueChirho::CharChirho(char_chirho) => Ok(u32::from(*char_chirho)),
                other_chirho => Err(not_a_character_chirho(other_chirho)),
            };
        };
        let forced_chirho = self.force_addr_to_whnf_chirho(*head_addr_chirho)?;
        match self
            .heap_chirho
            .read_chirho(forced_chirho)
            .payload_chirho
            .first()
        {
            Some(ValueChirho::CharChirho(char_chirho)) => Ok(u32::from(*char_chirho)),
            Some(other_chirho) => Err(not_a_character_chirho(other_chirho)),
            None => Err(not_a_character_chirho(head_chirho)),
        }
    }
}
