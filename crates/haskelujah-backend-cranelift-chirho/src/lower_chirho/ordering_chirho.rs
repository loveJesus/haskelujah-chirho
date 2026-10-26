// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `compare` on Int, Char, Double and String (`compare#`, `compareChar#`,
//! `compareFloat#`, `compareStr#`), with LLVM's semantics
//! (`codegen_chirho/primitives_chirho.rs`), which are GHC's `Ord` instances:
//! less gives `LT`, equal gives `EQ`, and anything else gives `GT`, as the
//! Ordering constructor tags. A NaN is neither less nor equal, so it gives
//! `GT`; strings order by `strcmp`'s unsigned bytes, which for UTF-8 is
//! code-point order. A Prelude `Ord` dictionary that a program reaches
//! compiles these bodies even when the program never calls `compare`, so
//! leaving them out would refuse such programs.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

use cranelift_codegen::ir::condcodes::{FloatCC as FloatCcChirho, IntCC as IntCcChirho};
use cranelift_codegen::ir::types as cl_types_chirho;
use cranelift_codegen::ir::{InstBuilder as _, Value as ClValueChirho};
use cranelift_frontend::FunctionBuilder as FuncBuilderChirho;

use super::{
    LowerCtxChirho, con_tag_for_chirho, ensure_f64_chirho, ensure_i64_chirho, failures_chirho,
};

/// The Ordering of two forced Int or Char operands.
pub(super) fn lower_compare_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    lhs_raw_chirho: ClValueChirho,
    rhs_raw_chirho: ClValueChirho,
) -> ClValueChirho {
    let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
    let rhs_chirho = ensure_i64_chirho(builder_chirho, rhs_raw_chirho, false);
    let less_chirho =
        builder_chirho
            .ins()
            .icmp(IntCcChirho::SignedLessThan, lhs_chirho, rhs_chirho);
    let equal_chirho = builder_chirho
        .ins()
        .icmp(IntCcChirho::Equal, lhs_chirho, rhs_chirho);
    ordering_chirho(builder_chirho, less_chirho, equal_chirho)
}

/// The Ordering of two forced Double operands. Both tests are ordered, so a
/// NaN on either side is neither less nor equal.
pub(super) fn lower_compare_float_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    lhs_raw_chirho: ClValueChirho,
    rhs_raw_chirho: ClValueChirho,
) -> ClValueChirho {
    let lhs_chirho = ensure_f64_chirho(builder_chirho, lhs_raw_chirho);
    let rhs_chirho = ensure_f64_chirho(builder_chirho, rhs_raw_chirho);
    let less_chirho = builder_chirho
        .ins()
        .fcmp(FloatCcChirho::LessThan, lhs_chirho, rhs_chirho);
    let equal_chirho = builder_chirho
        .ins()
        .fcmp(FloatCcChirho::Equal, lhs_chirho, rhs_chirho);
    ordering_chirho(builder_chirho, less_chirho, equal_chirho)
}

/// The Ordering of two forced native strings, by C `strcmp`. Without the
/// `strcmp` import no answer is faithful, so the primitive is refused.
pub(super) fn lower_compare_str_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    lhs_raw_chirho: ClValueChirho,
    rhs_raw_chirho: ClValueChirho,
) -> ClValueChirho {
    let Some(strcmp_ref_chirho) = ctx_chirho.strcmp_ref_chirho else {
        return failures_chirho::refuse_primitive_chirho(
            builder_chirho,
            ctx_chirho,
            "compareStr#",
            2,
        );
    };
    let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
    let rhs_chirho = ensure_i64_chirho(builder_chirho, rhs_raw_chirho, false);
    let call_chirho = builder_chirho
        .ins()
        .call(strcmp_ref_chirho, &[lhs_chirho, rhs_chirho]);
    let order_chirho = builder_chirho.inst_results(call_chirho)[0];
    let zero_chirho = builder_chirho.ins().iconst(cl_types_chirho::I32, 0);
    let less_chirho =
        builder_chirho
            .ins()
            .icmp(IntCcChirho::SignedLessThan, order_chirho, zero_chirho);
    let equal_chirho = builder_chirho
        .ins()
        .icmp(IntCcChirho::Equal, order_chirho, zero_chirho);
    ordering_chirho(builder_chirho, less_chirho, equal_chirho)
}

/// `LT` when `less_chirho`, else `EQ` when `equal_chirho`, else `GT`, as the
/// Ordering constructor tags.
fn ordering_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    less_chirho: ClValueChirho,
    equal_chirho: ClValueChirho,
) -> ClValueChirho {
    let [lt_chirho, eq_chirho, gt_chirho] = ["LT", "EQ", "GT"].map(|name_chirho| {
        builder_chirho.ins().iconst(
            cl_types_chirho::I64,
            i64::from(con_tag_for_chirho(name_chirho)),
        )
    });
    let not_less_chirho = builder_chirho
        .ins()
        .select(equal_chirho, eq_chirho, gt_chirho);
    builder_chirho
        .ins()
        .select(less_chirho, lt_chirho, not_less_chirho)
}
