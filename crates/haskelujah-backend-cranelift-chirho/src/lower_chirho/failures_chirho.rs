// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The expression-level half of Cranelift's failure routes: an unbound
//! reference becomes the runtime's failing thunk, and a primitive this backend
//! cannot lower is recorded for refusal instead of becoming 0.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

use cranelift_codegen::ir::types as cl_types_chirho;
use cranelift_codegen::ir::{InstBuilder as _, Value as ClValueChirho};
use cranelift_frontend::FunctionBuilder as FuncBuilderChirho;

use haskelujah_core_chirho::expr_chirho::CoreIdChirho;

use super::LowerCtxChirho;

/// Names whose calls `lower_app_chirho` implements at the call site. A body
/// primitive with one of these names belongs to the adapter binding behind
/// such a call (the IO-action pass names its raw effects this way), so it is
/// not refused. Keep in step with those call-site matches.
const CALL_SITE_PRIMITIVES_CHIRHO: &[&str] = &[
    "error",
    "error#",
    "errorWithoutStackTrace",
    "return",
    "pure",
    "returnIO#",
    ">>",
    "thenIO#",
    ">>=",
    "bindIO#",
    "putStr",
    "putStr#",
    "putStrLn",
    "putStrLn#",
    "print",
    "print#",
    "getLine",
    "getLine#",
    "readFile",
    "readFile#",
    "writeFile",
    "writeFile#",
    "toUpper",
    "toUpper#",
    "toLower",
    "toLower#",
    "isDigit",
    "isDigit#",
    "isAlpha",
    "isAlpha#",
    "unpack",
    "unpack#",
    "ord",
    "ord#",
    "chr",
    "chr#",
    "show",
    "show#",
    "$sel_Show_show",
];

/// A reference no scope resolved. An unbound free id becomes the runtime's
/// failing thunk for its name, so it fails named only if demanded; `undefined`
/// fails where it is referenced in a strict position, as on LLVM, and a lazy
/// position suspends it first (row 588). Any other miss, a bound local that
/// lowering did not bind, keeps the 0 placeholder (outside row 535).
pub(super) fn lower_unresolved_reference_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    id_chirho: CoreIdChirho,
) -> ClValueChirho {
    let unbound_names_chirho = ctx_chirho.unbound_names_chirho;
    let Some(name_chirho) = unbound_names_chirho.get(&id_chirho) else {
        return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
    };
    if matches!(name_chirho.as_str(), "undefined" | "undefined#") {
        return ctx_chirho
            .runtime_calls_chirho
            .undefined_chirho(builder_chirho);
    }
    let name_global_chirho = *ctx_chirho
        .string_globals_chirho
        .get(name_chirho.as_str())
        .expect("the pre-scan embeds every unbound name");
    let name_ptr_chirho = builder_chirho
        .ins()
        .global_value(cl_types_chirho::I64, name_global_chirho);
    ctx_chirho.runtime_calls_chirho.unbound_thunk_chirho(
        builder_chirho,
        name_ptr_chirho,
        name_chirho.len(),
    )
}

/// Records a primitive this backend cannot lower, unless a call site already
/// implements it. The placeholder only lets expression lowering finish: an
/// executable that compiles a recorded primitive is refused, as on LLVM.
pub(super) fn refuse_primitive_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    name_chirho: &str,
    arity_chirho: usize,
) -> ClValueChirho {
    if !CALL_SITE_PRIMITIVES_CHIRHO.contains(&name_chirho) {
        ctx_chirho
            .unsupported_primitives_chirho
            .insert(format!("{name_chirho}/{arity_chirho}"));
    }
    builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
}
