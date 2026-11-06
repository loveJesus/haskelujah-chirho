// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Unary boxed entries, staged partial applications and rooted result handoff.
//! Known saturated workers and the existing bare-pointer ABI stay separate.
//! Workflow: testing-chirho/native-root-lifetimes-chirho.md.

use super::root_frames_chirho::{write_root_frame_entry_chirho, write_root_frame_return_chirho};
use super::{LlvmCodegenChirho, NATIVE_OBJECT_HEADER_PTR_MASK_CHIRHO};
use std::fmt::Write;

fn application_entry_chirho(name_chirho: &str) -> LlvmCodegenChirho {
    let mut entry_chirho = LlvmCodegenChirho::new_chirho();
    writeln!(
        entry_chirho.output_chirho,
        "define i64 @{name_chirho}(i64 %env_chirho, i64 %partial_arg_chirho) {{\nentry:"
    )
    .unwrap();
    write_root_frame_entry_chirho(
        &mut entry_chirho.output_chirho,
        &["%env_chirho".to_string(), "%partial_arg_chirho".to_string()],
    );
    entry_chirho
}

impl LlvmCodegenChirho {
    fn finish_application_entry_chirho(&mut self, mut entry_chirho: Self, result_chirho: &str) {
        write_root_frame_return_chirho(&mut entry_chirho.output_chirho, result_chirho);
        writeln!(entry_chirho.output_chirho, "}}").unwrap();
        self.lifted_functions_chirho
            .push(entry_chirho.output_chirho);
    }

    // Every stage consumes exactly one argument. Its environment contains the
    // original worker's closure (if any), followed by already supplied values.
    fn emit_partial_application_entry_chirho(
        &mut self,
        fn_ref_chirho: &str,
        closure_environment_chirho: bool,
        applied_arg_count_chirho: usize,
        remaining_arity_chirho: usize,
    ) -> String {
        let mut next_entry_chirho: Option<String> = None;
        // Build from the saturated stage backwards, without compiler recursion.
        for stage_remaining_chirho in 1..=remaining_arity_chirho {
            let stored_count_chirho =
                applied_arg_count_chirho + remaining_arity_chirho - stage_remaining_chirho;
            let name_chirho = self.fresh_lifted_name_chirho();
            let mut entry_chirho = application_entry_chirho(&name_chirho);
            let mut captured_chirho = Vec::new();
            let mut field_chirho = 1;
            if closure_environment_chirho {
                captured_chirho.push(
                    entry_chirho.load_boxed_constructor_field_chirho("%env_chirho", field_chirho),
                );
                field_chirho += 1;
            }
            for _index_chirho in 0..stored_count_chirho {
                captured_chirho.push(
                    entry_chirho.load_boxed_constructor_field_chirho("%env_chirho", field_chirho),
                );
                field_chirho += 1;
            }
            captured_chirho.push("%partial_arg_chirho".to_string());
            let result_chirho = if let Some(next_chirho) = next_entry_chirho.as_deref() {
                // Entry roots retain the old environment and new argument
                // while this next PAP allocates and publishes its captures.
                entry_chirho.emit_captured_value_closure_chirho(next_chirho, &captured_chirho)
            } else {
                let arguments_chirho = captured_chirho
                    .iter()
                    .map(|value_chirho| format!("i64 {value_chirho}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                let result_chirho = entry_chirho.fresh_tmp_chirho();
                writeln!(
                    entry_chirho.output_chirho,
                    "  {result_chirho} = tail call i64 {fn_ref_chirho}({arguments_chirho})"
                )
                .unwrap();
                entry_chirho.emit_gc_root_push_i64_chirho(&result_chirho);
                result_chirho
            };
            self.finish_application_entry_chirho(entry_chirho, &result_chirho);
            next_entry_chirho = Some(name_chirho);
        }
        next_entry_chirho.expect("partial application needs a remaining argument Chirho")
    }

    pub(super) fn emit_partial_application_closure_chirho(
        &mut self,
        fn_ref_chirho: &str,
        closure_env_val_chirho: Option<&str>,
        applied_arg_vals_chirho: &[String],
        remaining_arity_chirho: usize,
    ) -> String {
        let entry_chirho = self.emit_partial_application_entry_chirho(
            fn_ref_chirho,
            closure_env_val_chirho.is_some(),
            applied_arg_vals_chirho.len(),
            remaining_arity_chirho,
        );
        let mut captured_chirho = Vec::new();
        if let Some(environment_chirho) = closure_env_val_chirho {
            captured_chirho.push(environment_chirho.to_string());
        }
        captured_chirho.extend(applied_arg_vals_chirho.iter().cloned());
        self.emit_captured_value_closure_chirho(&entry_chirho, &captured_chirho)
    }

    pub(super) fn emit_boxed_closure_entry_chirho(
        &mut self,
        worker_name_chirho: &str,
        arity_chirho: usize,
    ) -> String {
        if arity_chirho <= 1 {
            return worker_name_chirho.to_string();
        }
        let next_chirho = self.emit_partial_application_entry_chirho(
            &format!("@{worker_name_chirho}"),
            true,
            1,
            arity_chirho - 1,
        );
        let name_chirho = self.fresh_lifted_name_chirho();
        let mut entry_chirho = application_entry_chirho(&name_chirho);
        // Keep the original environment and its field layout for direct worker
        // calls. Only the boxed entry header points at this unary adapter.
        let result_chirho = entry_chirho.emit_captured_value_closure_chirho(
            &next_chirho,
            &["%env_chirho".to_string(), "%partial_arg_chirho".to_string()],
        );
        self.finish_application_entry_chirho(entry_chirho, &result_chirho);
        name_chirho
    }

    pub(super) fn compile_curried_indirect_apps_chirho(
        &mut self,
        fun_val_chirho: &str,
        arg_vals_chirho: &[String],
    ) -> String {
        if arg_vals_chirho.is_empty() {
            return fun_val_chirho.to_string();
        }
        self.compile_indirect_call_chirho(fun_val_chirho, arg_vals_chirho)
    }

    fn compile_indirect_call_chirho(
        &mut self,
        fun_val_chirho: &str,
        arg_vals_chirho: &[String],
    ) -> String {
        let forced_fun_chirho = self.emit_force_thunk_chirho(fun_val_chirho);
        let fun_val_chirho = forced_fun_chirho.as_str();
        let boxed_label_chirho = self.fresh_label_chirho("call.boxed");
        let direct_label_chirho = self.fresh_label_chirho("call.direct");
        let boxed_return_label_chirho = self.fresh_label_chirho("call.boxed.return.chirho");
        let join_label_chirho = self.fresh_label_chirho("call.join");
        let is_boxed_tmp_chirho = self.fresh_tmp_chirho();
        let boxed_probe_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {boxed_probe_tmp_chirho} = call i64 @haskelujah_is_heap_ptr_chirho(i64 {fun_val_chirho})"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {is_boxed_tmp_chirho} = icmp ne i64 {boxed_probe_tmp_chirho}, 0"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  br i1 {is_boxed_tmp_chirho}, label %{boxed_label_chirho}, label %{direct_label_chirho}"
        )
        .unwrap();

        writeln!(self.output_chirho, "{direct_label_chirho}:").unwrap();
        let direct_fun_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {direct_fun_ptr_tmp_chirho} = inttoptr i64 {fun_val_chirho} to ptr"
        )
        .unwrap();
        let direct_result_tmp_chirho = self.fresh_tmp_chirho();
        let arguments_chirho = arg_vals_chirho
            .iter()
            .map(|value_chirho| format!("i64 {value_chirho}"))
            .collect::<Vec<_>>()
            .join(", ");
        // Preserve the existing bare-pointer ABI. Its missing runtime arity
        // authority is an explicit remaining gap, not solved by boxed entries.
        writeln!(
            self.output_chirho,
            "  {direct_result_tmp_chirho} = call i64 {direct_fun_ptr_tmp_chirho}({arguments_chirho})"
        )
        .unwrap();
        writeln!(self.output_chirho, "  br label %{join_label_chirho}").unwrap();

        writeln!(self.output_chirho, "{boxed_label_chirho}:").unwrap();
        let closure_fun_bits_tmp_chirho =
            self.load_boxed_constructor_field_chirho(fun_val_chirho, 0);
        let closure_entry_bits_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {closure_entry_bits_tmp_chirho} = and i64 {closure_fun_bits_tmp_chirho}, {NATIVE_OBJECT_HEADER_PTR_MASK_CHIRHO}"
        )
        .unwrap();
        let closure_fun_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {closure_fun_ptr_tmp_chirho} = inttoptr i64 {closure_entry_bits_tmp_chirho} to ptr"
        )
        .unwrap();
        let closure_result_tmp_chirho = self.fresh_tmp_chirho();
        let first_arg_chirho = &arg_vals_chirho[0];
        writeln!(
            self.output_chirho,
            "  {closure_result_tmp_chirho} = call i64 {closure_fun_ptr_tmp_chirho}(i64 {fun_val_chirho}, i64 {first_arg_chirho})"
        )
        .unwrap();
        self.emit_gc_root_push_i64_chirho(&closure_result_tmp_chirho);
        let boxed_result_chirho = self.compile_curried_indirect_apps_chirho(
            &closure_result_tmp_chirho,
            &arg_vals_chirho[1..],
        );
        // Nested applications introduce their own blocks. A dedicated landing
        // keeps this phi predecessor exact without guessing the current block.
        writeln!(
            self.output_chirho,
            "  br label %{boxed_return_label_chirho}\n{boxed_return_label_chirho}:\n  br label %{join_label_chirho}\n{join_label_chirho}:"
        )
        .unwrap();
        let result_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {result_tmp_chirho} = phi i64 [{direct_result_tmp_chirho}, %{direct_label_chirho}], [{boxed_result_chirho}, %{boxed_return_label_chirho}]"
        )
        .unwrap();
        self.emit_gc_root_push_i64_chirho(&result_tmp_chirho);
        result_tmp_chirho
    }
}
