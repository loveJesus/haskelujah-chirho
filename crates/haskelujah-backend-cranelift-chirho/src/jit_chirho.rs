// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Cranelift JIT execution
//!
//! Compiles Core IR to native code in-memory via Cranelift JIT and executes
//! the `main` function directly — no linker, no object file, no external tools.
//! This is the fastest path from Haskell source to execution.

use std::collections::HashMap;

use cranelift_codegen::ir::types as cl_types_chirho;
use cranelift_codegen::ir::{AbiParam as AbiParamChirho, Function as ClFunctionChirho, InstBuilder as _};
use cranelift_codegen::settings as cl_settings_chirho;
use cranelift_frontend::{FunctionBuilder as FuncBuilderChirho, FunctionBuilderContext as FuncBuilderCtxChirho};
use cranelift_jit::{JITBuilder as JitBuilderChirho, JITModule as JitModuleChirho};
use cranelift_module::{Linkage as LinkageChirho, Module as ModuleTraitChirho};

use haskelujah_core_chirho::expr_chirho::{CoreModuleChirho, CoreLitChirho};

/// JIT-compile a Core module and execute its `main` function.
/// Returns the i64 result of main (0 for IO programs).
/// No linker or object file needed — runs entirely in-memory.
pub fn jit_execute_chirho(module_chirho: &CoreModuleChirho) -> Result<i64, String> {
    let filtered_chirho =
        haskelujah_core_chirho::elide_dicts_and_filter_chirho(module_chirho);

    // Build JIT module
    let mut flag_builder_chirho = cl_settings_chirho::builder();
    cranelift_codegen::settings::Configurable::set(
        &mut flag_builder_chirho,
        "is_pic",
        "false",
    )
    .map_err(|e_chirho| format!("JIT settings: {e_chirho}"))?;

    let isa_builder_chirho = cranelift_codegen::isa::lookup_by_name(
        &target_lexicon::Triple::host().to_string(),
    )
    .map_err(|e_chirho| format!("JIT ISA: {e_chirho}"))?;

    let flags_chirho = cl_settings_chirho::Flags::new(flag_builder_chirho);
    let isa_chirho = isa_builder_chirho
        .finish(flags_chirho)
        .map_err(|e_chirho| format!("JIT ISA finish: {e_chirho}"))?;

    let mut jit_builder_chirho = JitBuilderChirho::with_isa(
        isa_chirho,
        cranelift_module::default_libcall_names(),
    );

    // Link libc symbols for IO
    jit_builder_chirho.symbol("puts", libc_puts as *const u8);
    jit_builder_chirho.symbol(
        "haskelujah_print_int_chirho",
        jit_print_int_chirho as *const u8,
    );
    jit_builder_chirho.symbol(
        "haskelujah_alloc_chirho",
        jit_alloc_chirho as *const u8,
    );

    let mut jit_module_chirho = JitModuleChirho::new(jit_builder_chirho);

    // Use the same codegen as object mode but with JIT module
    // For now, return an error indicating JIT is WIP
    Err("JIT execution is work-in-progress — use object mode for now".to_string())
}

// JIT helper: puts wrapper
extern "C" fn libc_puts(s: *const i8) -> i32 {
    unsafe { libc::puts(s) }
}

// JIT helper: print int
extern "C" fn jit_print_int_chirho(n: i64) -> i64 {
    println!("{n}");
    0
}

// JIT helper: alloc
extern "C" fn jit_alloc_chirho(size: u64) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(size.max(8) as usize, 8).unwrap();
    unsafe { std::alloc::alloc_zeroed(layout) }
}
