// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The native launcher retains the caller's execution role through the worker join.
//! An IO result is lazy data, never a process status or an implicit print request.
//! Workflow: testing-chirho/native-entry-role-chirho.

use super::{CoreModuleChirho, NativeExecutionRoleChirho};
use std::fmt::Write;

pub(super) fn write_native_entry_chirho(
    output_chirho: &mut String,
    module_chirho: &CoreModuleChirho,
    filtered_module_chirho: &CoreModuleChirho,
    execution_role_chirho: NativeExecutionRoleChirho,
) {
    // Preserve the legacy value API's name-based output policy only in that role.
    let has_builtin_io_chirho = execution_role_chirho == NativeExecutionRoleChirho::ValueChirho
        && filtered_module_chirho
            .bindings_chirho
            .iter()
            .any(|binding_chirho| {
                matches!(
                    binding_chirho.binder_chirho.name_chirho.as_str(),
                    "putStrLn"
                        | "putStrLn#"
                        | "putStr"
                        | "putStr#"
                        | "putChar"
                        | "putChar#"
                        | "print"
                        | "interact"
                        | "getLine"
                        | "getLine#"
                        | "getChar"
                        | "getContents"
                        | "getContents#"
                        | "readFile"
                        | "writeFile"
                        | "appendFile"
                        | "return"
                        | "pure"
                        | ">>="
                        | "bindIO#"
                        | ">>"
                        | "thenIO#"
                )
            });

    // The recorded program entry, never a spelling, gets the C `main()` (row 628).
    let has_main_chirho =
        module_chirho
            .provenance_chirho
            .entry_chirho
            .is_some_and(|entry_chirho| {
                module_chirho
                    .bindings_chirho
                    .iter()
                    .any(|binding_chirho| binding_chirho.binder_chirho.id_chirho == entry_chirho)
            });

    if has_main_chirho {
        // Add a C main() entry point.
        writeln!(output_chirho).unwrap();
        writeln!(output_chirho, "; ── RTS entry point ──").unwrap();
        writeln!(output_chirho).unwrap();
        writeln!(output_chirho, "define i32 @main() {{").unwrap();
        writeln!(output_chirho, "entry:").unwrap();
        writeln!(
            output_chirho,
            "  %main_fn = ptrtoint ptr @haskelujah_main to i64"
        )
        .unwrap();
        if execution_role_chirho == NativeExecutionRoleChirho::ProgramChirho
            || has_builtin_io_chirho
        {
            writeln!(
                output_chirho,
                "  call i64 @haskelujah_main_with_large_stack_chirho(i64 %main_fn)"
            )
            .unwrap();
            writeln!(output_chirho, "  ret i32 0").unwrap();
        } else {
            writeln!(
                output_chirho,
                "  %result = call i64 @haskelujah_main_with_large_stack_chirho(i64 %main_fn)"
            )
            .unwrap();
            // The runtime refuses a live heap value instead of printing its address.
            writeln!(
                output_chirho,
                "  call i64 @haskelujah_print_int_chirho(i64 %result)"
            )
            .unwrap();
            writeln!(output_chirho, "  %exitcode = trunc i64 %result to i32").unwrap();
            writeln!(output_chirho, "  ret i32 %exitcode").unwrap();
        }
        writeln!(output_chirho, "}}").unwrap();
    }
}
