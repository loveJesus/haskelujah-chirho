// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

pub mod eval_chirho;
pub mod ffi_chirho;
pub mod gc_chirho;
pub mod heap_chirho;
pub mod prim_chirho;
pub mod stack_chirho;
pub mod value_chirho;

pub use eval_chirho::{
    ArgSourceChirho, CodeChirho, EvalErrorChirho, MachineChirho, RecBindingSpecChirho,
};
pub use ffi_chirho::{
    FfiCallConvChirho, FfiErrorChirho, FfiSafetyChirho, FfiTypeChirho, FfiValueChirho,
    ForeignImportChirho, ForeignTableChirho,
};
pub use gc_chirho::{
    GcConfigChirho, GcStateChirho, GcStatsChirho, extract_roots_from_stack_chirho,
    extract_roots_from_values_chirho,
};
pub use heap_chirho::HeapChirho;
pub use prim_chirho::{PrimErrorChirho, apply_prim_binop_chirho};
pub use stack_chirho::{FrameChirho, PrimOpKindChirho, StackChirho};
pub use value_chirho::{
    ClosureChirho, CodePtrChirho, DataConTagChirho, HeapAddrChirho, InfoTableChirho, InfoTagChirho,
    ValueChirho,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionModeChirho {
    BatchChirho,
    ScriptChirho,
    ReplChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePlanChirho {
    pub execution_mode_chirho: ExecutionModeChirho,
    pub entry_module_chirho: String,
    pub incremental_session_chirho: bool,
}

impl RuntimePlanChirho {
    pub fn for_module_chirho(
        execution_mode_chirho: ExecutionModeChirho,
        entry_module_chirho: impl Into<String>,
    ) -> Self {
        let incremental_session_chirho = matches!(
            execution_mode_chirho,
            ExecutionModeChirho::ScriptChirho | ExecutionModeChirho::ReplChirho
        );

        Self {
            execution_mode_chirho,
            entry_module_chirho: entry_module_chirho.into(),
            incremental_session_chirho,
        }
    }
}
