// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Native failures leave through the runtime's one route
//! (`haskelujah-rts-chirho/src/ffi_chirho/failure_chirho.rs`): one named
//! stderr line and exit 1. No unbound name reads an enclosing parameter, and no
//! case default traps.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

use super::{BinderChirho, CoreExprChirho, CoreIdChirho, LlvmCodegenChirho};
use std::collections::HashSet;
use std::fmt::Write;

/// The runtime entries the failure routes call.
pub(super) const FAILURE_DECLARATIONS_CHIRHO: &[&str] = &[
    "declare i64 @haskelujah_unbound_thunk_chirho(ptr, i64)",
    "declare void @haskelujah_no_match_chirho() noreturn",
    "declare void @haskelujah_undefined_chirho() noreturn",
    "declare i64 @haskelujah_print_int_chirho(i64)",
];

/// The body of a built-in `error` or `errorWithoutStackTrace`: force the
/// message, which a lazy argument passes as a thunk, and raise it exactly.
pub(super) fn write_error_body_chirho(out_chirho: &mut String, message_value_chirho: &str) {
    writeln!(
        out_chirho,
        "  %message_bits_chirho = call i64 @haskelujah_enter_thunk_chirho(i64 {message_value_chirho})"
    )
    .unwrap();
    writeln!(
        out_chirho,
        "  %message_chirho = inttoptr i64 %message_bits_chirho to ptr"
    )
    .unwrap();
    writeln!(
        out_chirho,
        "  call i64 @raise_error_chirho(ptr %message_chirho)"
    )
    .unwrap();
    writeln!(out_chirho, "  unreachable").unwrap();
}

/// The body of a built-in `undefined`.
pub(super) fn write_undefined_body_chirho(out_chirho: &mut String) {
    writeln!(out_chirho, "  call void @haskelujah_undefined_chirho()").unwrap();
    writeln!(out_chirho, "  unreachable").unwrap();
}

impl LlvmCodegenChirho {
    /// Records the free ids of a function body that nothing binds: no
    /// parameter, capture, top-level, lifted local or built-in.
    pub(super) fn note_unbound_ids_chirho(
        &mut self,
        param_binders_chirho: &[&BinderChirho],
        body_chirho: &CoreExprChirho,
        capture_ids_chirho: &[CoreIdChirho],
    ) {
        let param_ids_chirho: HashSet<CoreIdChirho> = param_binders_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.id_chirho)
            .collect();
        self.unbound_ids_chirho = self
            .collect_free_runtime_var_ids_chirho(body_chirho)
            .into_iter()
            .filter(|id_chirho| {
                !param_ids_chirho.contains(id_chirho) && !capture_ids_chirho.contains(id_chirho)
            })
            .collect();
    }

    /// A variable no scope resolved: either a local this function defines on
    /// another path, or an unbound reference. The latter's value is the
    /// runtime's failing thunk for its name, so it fails named only if demanded.
    pub(super) fn compile_unresolved_reference_chirho(
        &mut self,
        id_chirho: CoreIdChirho,
    ) -> String {
        if !self.unbound_ids_chirho.contains(&id_chirho) {
            return format!("%v{}", id_chirho.0);
        }
        let name_chirho = self
            .resolved_names_chirho
            .get(&id_chirho)
            .cloned()
            .unwrap_or_else(|| format!("CoreId {}", id_chirho.0));
        let global_chirho = self.intern_string_global_name_chirho(&name_chirho);
        let thunk_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {thunk_chirho} = call i64 @haskelujah_unbound_thunk_chirho(ptr @{global_chirho}, i64 {})",
            name_chirho.len()
        )
        .unwrap();
        thunk_chirho
    }

    /// The default of a case whose compiled alternatives can all miss.
    pub(super) fn emit_no_match_default_chirho(&mut self, default_label_chirho: &str) {
        writeln!(self.output_chirho, "{default_label_chirho}:").unwrap();
        writeln!(
            self.output_chirho,
            "  call void @haskelujah_no_match_chirho()"
        )
        .unwrap();
        writeln!(self.output_chirho, "  unreachable").unwrap();
    }
}
