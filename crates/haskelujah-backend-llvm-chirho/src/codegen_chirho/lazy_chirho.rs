// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lazy producers pass a value or a suspended computation. Only an explicit
//! demand (case or strict primitive) enters it. Native thunk preparation closes
//! nontrivial expressions into known applications before this boundary.
//!
//! A reference to a nullary global, a top-level binding or a builtin of arity 0, is
//! suspended here too (row 588). `compile_expr_chirho` calls it for its value, which in a
//! lazy position ran a CAF nothing had demanded: `length [undefined, undefined]` failed,
//! and a self-recursive CAF overflowed the stack. Each suspension allocates one thunk; a
//! CAF is still not shared between references.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use std::fmt::Write;

use super::{CoreExprChirho, LlvmCodegenChirho, mangle_name_chirho};

impl LlvmCodegenChirho {
    pub(super) fn compile_lazy_value_chirho(
        &mut self,
        expression_chirho: &CoreExprChirho,
    ) -> String {
        if let Some(fn_ref_chirho) = self.nullary_global_ref_chirho(expression_chirho) {
            let fn_addr_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {fn_addr_chirho} = ptrtoint ptr {fn_ref_chirho} to i64"
            )
            .unwrap();
            return self.emit_thunk_chirho(0, &[fn_addr_chirho]);
        }
        self.try_create_thunk_for_app_chirho(expression_chirho)
            .unwrap_or_else(|| self.compile_expr_chirho(expression_chirho))
    }

    /// The function `compile_expr_chirho` calls for a reference to a nullary top-level binding
    /// or a nullary builtin, found in that function's own order of scopes.
    fn nullary_global_ref_chirho(&mut self, expression_chirho: &CoreExprChirho) -> Option<String> {
        let mut head_chirho = expression_chirho;
        while let CoreExprChirho::TyAppChirho { expr_chirho, .. } = head_chirho {
            head_chirho = expr_chirho;
        }
        let CoreExprChirho::VarChirho(id_chirho) = head_chirho else {
            return None;
        };
        if self.local_scope_chirho.contains(id_chirho)
            || self.lifted_local_names_chirho.contains_key(id_chirho)
        {
            return None;
        }
        if let Some(name_chirho) = self.toplevel_names_chirho.get(id_chirho) {
            let arity_chirho = self
                .toplevel_arities_chirho
                .get(id_chirho)
                .copied()
                .unwrap_or(0);
            return (arity_chirho == 0).then(|| format!("@{}", mangle_name_chirho(name_chirho)));
        }
        let name_chirho = self.resolved_names_chirho.get(id_chirho).cloned()?;
        let (fn_ref_chirho, arity_chirho) =
            self.resolve_builtin_runtime_function_chirho(&name_chirho)?;
        (arity_chirho == 0).then_some(fn_ref_chirho)
    }
}
