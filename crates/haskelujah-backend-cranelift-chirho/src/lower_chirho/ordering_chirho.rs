// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `compare` on Int and Char (`compare#`, `compareChar#`), with LLVM's
//! semantics (`codegen_chirho/primitives_chirho.rs`): signed less gives `LT`,
//! equal gives `EQ`, and anything else gives `GT`, as the Ordering constructor
//! tags. A Prelude `Ord` dictionary that a program reaches compiles these
//! bodies even when the program never calls `compare`, so leaving them out
//! would refuse such programs.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

use cranelift_codegen::ir::condcodes::IntCC as IntCcChirho;
use cranelift_codegen::ir::types as cl_types_chirho;
use cranelift_codegen::ir::{InstBuilder as _, Value as ClValueChirho};
use cranelift_frontend::FunctionBuilder as FuncBuilderChirho;

use super::{con_tag_for_chirho, ensure_i64_chirho};

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
