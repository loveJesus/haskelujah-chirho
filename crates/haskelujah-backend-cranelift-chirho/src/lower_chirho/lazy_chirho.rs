// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Calls and constructor fields preserve lazy arguments. Native preparation
//! closes nontrivial expressions; strict consumers explicitly enter the result.
//!
//! A reference to a nullary global, a known function of arity 0, `getLine` or `undefined`,
//! is suspended here too (row 588). `lower_expr_chirho` calls it for its value, which in a
//! lazy position ran a CAF nothing had demanded: `length [undefined, undefined]` failed,
//! and a self-recursive CAF overflowed the stack. Each suspension allocates one thunk; a
//! CAF is still not shared between references.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use cranelift_codegen::ir::{FuncRef, InstBuilder as _};

use super::{
    ClValueChirho, CoreExprChirho, FuncBuilderChirho, LowerCtxChirho, alloc_thunk_from_fvs_chirho,
    cl_types_chirho, lower_expr_chirho, try_create_thunk_for_app_chirho,
};

pub(super) fn lower_lazy_value_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    context_chirho: &mut LowerCtxChirho<'_>,
    expression_chirho: &CoreExprChirho,
) -> ClValueChirho {
    if let Some(thunk_chirho) =
        suspend_nullary_global_chirho(builder_chirho, context_chirho, expression_chirho)
    {
        return thunk_chirho;
    }
    try_create_thunk_for_app_chirho(builder_chirho, context_chirho, expression_chirho)
        .unwrap_or_else(|| lower_expr_chirho(builder_chirho, context_chirho, expression_chirho))
}

/// A thunk over the function a nullary global names, entered by the arity-0 trampoline, or
/// None where thunks are unavailable and the reference stays a call.
fn suspend_nullary_global_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    context_chirho: &mut LowerCtxChirho<'_>,
    expression_chirho: &CoreExprChirho,
) -> Option<ClValueChirho> {
    let func_ref_chirho = nullary_global_ref_chirho(context_chirho, expression_chirho)?;
    let alloc_thunk_ref_chirho = context_chirho.alloc_thunk_ref_chirho?;
    if context_chirho.thunk_trampoline_refs_chirho.is_empty() {
        return None;
    }
    let func_addr_chirho = builder_chirho
        .ins()
        .func_addr(cl_types_chirho::I64, func_ref_chirho);
    Some(alloc_thunk_from_fvs_chirho(
        builder_chirho,
        context_chirho,
        alloc_thunk_ref_chirho,
        0,
        &[func_addr_chirho],
    ))
}

/// The function `lower_expr_chirho` calls for a reference to a nullary global, found in that
/// function's own order of scopes.
fn nullary_global_ref_chirho(
    context_chirho: &LowerCtxChirho<'_>,
    expression_chirho: &CoreExprChirho,
) -> Option<FuncRef> {
    let mut head_chirho = expression_chirho;
    while let CoreExprChirho::TyAppChirho { expr_chirho, .. } = head_chirho {
        head_chirho = expr_chirho;
    }
    let CoreExprChirho::VarChirho(id_chirho) = head_chirho else {
        return None;
    };
    if context_chirho.cl_vars_chirho.contains_key(id_chirho)
        || context_chirho
            .env_chirho
            .lookup_chirho(*id_chirho)
            .is_some()
    {
        return None;
    }
    if let Some(&(func_ref_chirho, arity_chirho)) =
        context_chirho.func_ref_map_chirho.get(id_chirho)
    {
        return (arity_chirho == 0).then_some(func_ref_chirho);
    }
    if let Some(name_chirho) = context_chirho.recognized_names_chirho.get(id_chirho)
        && matches!(name_chirho.as_str(), "getLine" | "getLine#")
    {
        return context_chirho.get_line_ref_chirho;
    }
    let name_chirho = context_chirho.recognized_names_chirho.get(id_chirho)?;
    (context_chirho.unbound_names_chirho.contains_key(id_chirho)
        && matches!(name_chirho.as_str(), "undefined" | "undefined#"))
    .then(|| context_chirho.runtime_calls_chirho.undefined_ref_chirho())
}
