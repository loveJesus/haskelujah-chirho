// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! BEAM VM opcode definitions.
//!
//! BEAM opcodes are variable-length instructions that operate on
//! registers (x-registers for arguments, y-registers for stack).

/// BEAM opcode numbers (from OTP 26 / BEAM instruction set).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeamOpChirho {
    LabelChirho = 1,
    FuncInfoChirho = 2,
    IntCodeEndChirho = 3,

    // Function calls
    CallChirho = 4,
    CallLastChirho = 5,
    CallOnlyChirho = 6,
    CallExtChirho = 7,
    CallExtLastChirho = 8,
    CallExtOnlyChirho = 78,

    // Returns
    ReturnChirho = 19,

    // Stack operations
    AllocateChirho = 12,
    AllocateZeroChirho = 14,
    DeallocateChirho = 18,

    // Moves
    MoveChirho = 64,

    // Tests
    IsLtChirho = 39,
    IsGeChirho = 40,
    IsEqChirho = 41,
    IsNeChirho = 42,
    IsEqExactChirho = 43,
    IsNeExactChirho = 44,
    IsIntegerChirho = 45,
    IsAtomChirho = 48,
    IsTupleChirho = 57,
    IsNilChirho = 55,
    IsNonemptyListChirho = 56,

    // Pattern matching
    SelectValChirho = 59,
    SelectTupleArityChirho = 60,

    // Tuple operations
    TestArityChirho = 58,
    GetTupleElementChirho = 66,
    SetTupleElementChirho = 67,
    PutTupleChirho = 70,

    // List operations
    GetListChirho = 65,
    PutListChirho = 69,

    // Arithmetic (GC BIFs)
    GcBif2Chirho = 125,
    GcBif1Chirho = 124,

    // Misc
    BadmatchChirho = 22,
    CaseEndChirho = 23,
    IfEndChirho = 24,

    // Closures / funs
    MakeFunChirho = 103,
    MakeFun2Chirho = 131,
    CallFunChirho = 75,
    ApplyChirho = 112,

    // Try/catch
    TryChirho = 104,
    TryEndChirho = 105,
    TryCaseChirho = 106,
    RaiseChirho = 107,

    // Maps (OTP 17+)
    HasMapFieldsChirho = 158,
    GetMapElementsChirho = 159,
}

/// BEAM register types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeamRegChirho {
    /// X-register (argument/temporary register).
    XChirho(u32),
    /// Y-register (stack frame slot).
    YChirho(u32),
}

/// A single BEAM instruction with its operands.
#[derive(Debug, Clone)]
pub struct BeamInsnChirho {
    pub op_chirho: BeamOpChirho,
    pub operands_chirho: Vec<BeamOperandChirho>,
}

/// BEAM instruction operand types.
#[derive(Debug, Clone)]
pub enum BeamOperandChirho {
    RegChirho(BeamRegChirho),
    LiteralChirho(i64),
    AtomChirho(u32),    // atom table index
    LabelChirho(u32),   // label number
    ArityChirho(u32),
}

/// Builder for a sequence of BEAM instructions.
pub struct BeamCodeBuilderChirho {
    instructions_chirho: Vec<BeamInsnChirho>,
    next_label_chirho: u32,
}

impl BeamCodeBuilderChirho {
    pub fn new_chirho() -> Self {
        Self {
            instructions_chirho: Vec::new(),
            next_label_chirho: 1,
        }
    }

    /// Allocate a fresh label number.
    pub fn fresh_label_chirho(&mut self) -> u32 {
        let label_chirho = self.next_label_chirho;
        self.next_label_chirho += 1;
        label_chirho
    }

    /// Emit a label.
    pub fn emit_label_chirho(&mut self, label_chirho: u32) {
        self.instructions_chirho.push(BeamInsnChirho {
            op_chirho: BeamOpChirho::LabelChirho,
            operands_chirho: vec![BeamOperandChirho::LabelChirho(label_chirho)],
        });
    }

    /// Emit a move instruction.
    pub fn emit_move_chirho(&mut self, src_chirho: BeamOperandChirho, dst_chirho: BeamRegChirho) {
        self.instructions_chirho.push(BeamInsnChirho {
            op_chirho: BeamOpChirho::MoveChirho,
            operands_chirho: vec![src_chirho, BeamOperandChirho::RegChirho(dst_chirho)],
        });
    }

    /// Emit a return instruction.
    pub fn emit_return_chirho(&mut self) {
        self.instructions_chirho.push(BeamInsnChirho {
            op_chirho: BeamOpChirho::ReturnChirho,
            operands_chirho: vec![],
        });
    }

    /// Emit a call instruction.
    pub fn emit_call_chirho(&mut self, arity_chirho: u32, label_chirho: u32) {
        self.instructions_chirho.push(BeamInsnChirho {
            op_chirho: BeamOpChirho::CallChirho,
            operands_chirho: vec![
                BeamOperandChirho::ArityChirho(arity_chirho),
                BeamOperandChirho::LabelChirho(label_chirho),
            ],
        });
    }

    /// Get the generated instructions.
    pub fn finish_chirho(self) -> Vec<BeamInsnChirho> {
        self.instructions_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn beam_code_builder_basic_chirho() {
        let mut builder_chirho = BeamCodeBuilderChirho::new_chirho();
        let label_chirho = builder_chirho.fresh_label_chirho();
        builder_chirho.emit_label_chirho(label_chirho);
        builder_chirho.emit_move_chirho(
            BeamOperandChirho::LiteralChirho(42),
            BeamRegChirho::XChirho(0),
        );
        builder_chirho.emit_return_chirho();
        let insns_chirho = builder_chirho.finish_chirho();
        assert_eq!(insns_chirho.len(), 3);
        assert_eq!(insns_chirho[0].op_chirho, BeamOpChirho::LabelChirho);
        assert_eq!(insns_chirho[2].op_chirho, BeamOpChirho::ReturnChirho);
    }

    #[test]
    fn beam_register_types_chirho() {
        assert_eq!(BeamRegChirho::XChirho(0), BeamRegChirho::XChirho(0));
        assert_ne!(BeamRegChirho::XChirho(0), BeamRegChirho::YChirho(0));
    }

    #[test]
    fn beam_fresh_labels_chirho() {
        let mut builder_chirho = BeamCodeBuilderChirho::new_chirho();
        let l1_chirho = builder_chirho.fresh_label_chirho();
        let l2_chirho = builder_chirho.fresh_label_chirho();
        assert_eq!(l1_chirho, 1);
        assert_eq!(l2_chirho, 2);
    }
}
