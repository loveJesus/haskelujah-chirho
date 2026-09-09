// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Calls and constructor fields preserve lazy arguments. Native preparation
//! closes nontrivial expressions; strict consumers explicitly enter the result.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use super::{
    ClValueChirho, CoreExprChirho, FuncBuilderChirho, LowerCtxChirho, lower_expr_chirho,
    try_create_thunk_for_app_chirho,
};

pub(super) fn lower_lazy_value_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    context_chirho: &mut LowerCtxChirho<'_>,
    expression_chirho: &CoreExprChirho,
) -> ClValueChirho {
    try_create_thunk_for_app_chirho(builder_chirho, context_chirho, expression_chirho)
        .unwrap_or_else(|| lower_expr_chirho(builder_chirho, context_chirho, expression_chirho))
}
