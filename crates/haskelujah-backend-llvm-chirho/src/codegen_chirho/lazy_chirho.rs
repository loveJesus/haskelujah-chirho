// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lazy producers pass a value or a suspended computation. Only an explicit
//! demand (case or strict primitive) enters it. Native thunk preparation closes
//! nontrivial expressions into known applications before this boundary.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use super::{CoreExprChirho, LlvmCodegenChirho};

impl LlvmCodegenChirho {
    pub(super) fn compile_lazy_value_chirho(
        &mut self,
        expression_chirho: &CoreExprChirho,
    ) -> String {
        self.try_create_thunk_for_app_chirho(expression_chirho)
            .unwrap_or_else(|| self.compile_expr_chirho(expression_chirho))
    }
}
