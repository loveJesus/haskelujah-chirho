// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Generated activation ownership, without guessing identities or demanding values.
//! Workflow: testing-chirho/native-root-lifetimes-chirho.md.

use super::{CoreIdChirho, LlvmCodegenChirho};
use std::fmt::Write;

pub(super) fn write_parameter_frame_entry_chirho(
    output_chirho: &mut String,
    parameters_chirho: &[CoreIdChirho],
    entry_names_chirho: bool,
) {
    let suffix_chirho = if entry_names_chirho {
        ".entry_chirho"
    } else {
        ""
    };
    let values_chirho = parameters_chirho
        .iter()
        .map(|id_chirho| format!("%v{}{suffix_chirho}", id_chirho.0))
        .collect::<Vec<_>>();
    write_root_frame_entry_chirho(output_chirho, &values_chirho);
}

pub(super) fn write_arity_frame_entry_chirho(output_chirho: &mut String, arity_chirho: usize) {
    let values_chirho = (0..arity_chirho)
        .map(|index_chirho| format!("%v{index_chirho}"))
        .collect::<Vec<_>>();
    write_root_frame_entry_chirho(output_chirho, &values_chirho);
}

pub(super) fn write_root_frame_entry_chirho(output_chirho: &mut String, values_chirho: &[String]) {
    writeln!(
        output_chirho,
        "  %gc_frame_token_chirho = call i64 @haskelujah_gc_root_frame_mark_chirho()"
    )
    .unwrap();
    for (index_chirho, value_chirho) in values_chirho.iter().enumerate() {
        writeln!(
            output_chirho,
            "  %gc_frame_arg_{index_chirho}_chirho = inttoptr i64 {value_chirho} to ptr"
        )
        .unwrap();
        writeln!(
            output_chirho,
            "  call void @haskelujah_gc_root_push_chirho(ptr %gc_frame_arg_{index_chirho}_chirho)"
        )
        .unwrap();
    }
}

pub(super) fn write_root_frame_return_chirho(output_chirho: &mut String, value_chirho: &str) {
    writeln!(
        output_chirho,
        "  call void @haskelujah_gc_root_frame_restore_chirho(i64 %gc_frame_token_chirho)"
    )
    .unwrap();
    // The caller installs this actual value before its next native safepoint.
    writeln!(output_chirho, "  ret i64 {value_chirho}").unwrap();
}

impl LlvmCodegenChirho {
    pub(super) fn emit_gc_root_push_ptr_chirho(&mut self, pointer_chirho: &str) {
        let bits_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {bits_chirho} = ptrtoint ptr {pointer_chirho} to i64"
        )
        .unwrap();
        self.emit_gc_root_push_i64_chirho(&bits_chirho);
    }

    pub(super) fn emit_gc_root_push_i64_chirho(&mut self, value_chirho: &str) {
        let root_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {root_ptr_tmp_chirho} = inttoptr i64 {value_chirho} to ptr"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  call void @haskelujah_gc_root_push_chirho(ptr {root_ptr_tmp_chirho})"
        )
        .unwrap();
    }

    // Heap results belong to the current generated activation frame.
    pub(super) fn emit_gc_root_push_heap_i64_chirho(&mut self, value_chirho: &str) {
        let is_heap_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {is_heap_tmp_chirho} = call i64 @haskelujah_is_heap_ptr_chirho(i64 {value_chirho})"
        )
        .unwrap();
        let should_push_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {should_push_tmp_chirho} = icmp ne i64 {is_heap_tmp_chirho}, 0"
        )
        .unwrap();
        let push_label_chirho = self.fresh_label_chirho("gc.root.push");
        let done_label_chirho = self.fresh_label_chirho("gc.root.done");
        writeln!(
            self.output_chirho,
            "  br i1 {should_push_tmp_chirho}, label %{push_label_chirho}, label %{done_label_chirho}"
        )
        .unwrap();
        writeln!(self.output_chirho, "{push_label_chirho}:").unwrap();
        self.emit_gc_root_push_i64_chirho(value_chirho);
        writeln!(self.output_chirho, "  br label %{done_label_chirho}").unwrap();
        writeln!(self.output_chirho, "{done_label_chirho}:").unwrap();
    }

    pub(super) fn emit_gc_root_pop_count_chirho(&mut self, count_chirho: usize) {
        for _ in 0..count_chirho {
            writeln!(
                self.output_chirho,
                "  call void @haskelujah_gc_root_pop_chirho()"
            )
            .unwrap();
        }
    }

    pub(super) fn emit_root_frame_reset_chirho(&mut self, values_chirho: &[String]) {
        writeln!(
            self.output_chirho,
            "  call void @haskelujah_gc_root_frame_reset_chirho(i64 %gc_frame_token_chirho)"
        )
        .unwrap();
        for value_chirho in values_chirho
            .iter()
            .chain(self.current_root_captures_chirho.clone().iter())
        {
            self.emit_gc_root_push_i64_chirho(value_chirho);
        }
    }
}
