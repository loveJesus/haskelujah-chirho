// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Core IR → Cranelift IR lowering utilities.
//!
//! Provides helpers for translating Core expressions into Cranelift IR
//! instructions, managing variable bindings, and handling the runtime
//! object layout.
//!
//! ## Expression lowering coverage
//!
//! | Core variant         | Cranelift output                              |
//! |----------------------|-----------------------------------------------|
//! | `LitChirho::Int`     | `iconst i64 N`                                |
//! | `LitChirho::Float`   | `f64const F`                                  |
//! | `LitChirho::Char`    | `iconst i64 codepoint`                        |
//! | `LitChirho::String`  | `iconst i64 len` (placeholder)                |
//! | `VarChirho`          | SSA value from `VarEnvChirho`                 |
//! | `PrimOpChirho` arith | `iadd / isub / imul / sdiv / srem / ineg`     |
//! | `PrimOpChirho` cmp   | `icmp + bint` → 0/1 in i64                    |
//! | `PrimOpChirho` float | `fadd / fsub / fmul / fdiv` (f64)             |
//! | `LetChirho`          | Cranelift `Variable`s via `use_var/def_var`   |
//! | `CaseChirho` literal | Conditional branches with `brif/jump`         |
//! | `TyLamChirho`        | Skip wrapper, lower body directly             |
//! | `TyAppChirho`        | Lower inner expression                        |
//! | `AppChirho`          | Indirect function call via `call_indirect`    |
//! | `ConAppChirho`       | Constructor tag as `iconst` (simplified)      |
//! | `LamChirho`          | Error — must be peeled by `codegen_chirho`    |

use cranelift_codegen::ir::condcodes::IntCC as IntCcChirho;
use cranelift_codegen::ir::types as cl_types_chirho;
use cranelift_codegen::ir::{InstBuilder as _, Value as ClValueChirho};
use cranelift_frontend::FunctionBuilder as FuncBuilderChirho;
use cranelift_frontend::Variable as ClVariableChirho;

use rhasky_core_chirho::expr_chirho::{
    AltConChirho, CoreAltChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho,
};

use std::collections::HashMap;

/// Variable environment mapping Core IDs to Cranelift SSA values.
pub struct VarEnvChirho {
    vars_chirho: HashMap<CoreIdChirho, ClValueChirho>,
}

impl VarEnvChirho {
    pub fn new_chirho() -> Self {
        Self {
            vars_chirho: HashMap::new(),
        }
    }

    pub fn bind_chirho(&mut self, id_chirho: CoreIdChirho, val_chirho: ClValueChirho) {
        self.vars_chirho.insert(id_chirho, val_chirho);
    }

    pub fn lookup_chirho(&self, id_chirho: CoreIdChirho) -> Option<ClValueChirho> {
        self.vars_chirho.get(&id_chirho).copied()
    }
}

/// Lowering context threading mutable state through a single function body.
pub struct LowerCtxChirho<'a> {
    /// SSA value environment for variables.
    pub env_chirho: &'a mut VarEnvChirho,
    /// Counter for allocating fresh `cranelift_frontend::Variable` indices.
    pub next_var_idx_chirho: &'a mut u32,
    /// Cranelift Variable environment for `LetChirho` bindings.
    pub cl_vars_chirho: &'a mut HashMap<CoreIdChirho, ClVariableChirho>,
    /// Map of CoreId → (FuncRef, arity) for direct top-level function calls.
    /// FuncRef is pre-imported into the current function via declare_func_in_func.
    pub func_ref_map_chirho: &'a HashMap<CoreIdChirho, (cranelift_codegen::ir::FuncRef, usize)>,
}

impl<'a> LowerCtxChirho<'a> {
    /// Allocate a fresh Cranelift `Variable` index and associate it with a Core ID.
    pub fn fresh_var_chirho(
        &mut self,
        id_chirho: CoreIdChirho,
        builder_chirho: &mut FuncBuilderChirho,
    ) -> ClVariableChirho {
        let idx_chirho = *self.next_var_idx_chirho;
        *self.next_var_idx_chirho += 1;
        let var_chirho = ClVariableChirho::from_u32(idx_chirho);
        builder_chirho.declare_var(var_chirho, cl_types_chirho::I64);
        self.cl_vars_chirho.insert(id_chirho, var_chirho);
        var_chirho
    }
}

/// Lower a Core literal to a Cranelift constant value.
pub fn lower_lit_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    lit_chirho: &CoreLitChirho,
) -> ClValueChirho {
    match lit_chirho {
        CoreLitChirho::IntChirho(n_chirho) => {
            builder_chirho.ins().iconst(cl_types_chirho::I64, *n_chirho)
        }
        CoreLitChirho::FloatChirho(f_chirho) => builder_chirho.ins().f64const(*f_chirho),
        CoreLitChirho::CharChirho(c_chirho) => {
            builder_chirho
                .ins()
                .iconst(cl_types_chirho::I64, *c_chirho as i64)
        }
        CoreLitChirho::StringChirho(s_chirho) => {
            // String literals: return length as placeholder until heap allocation
            // is wired in. This keeps the backend compilable end-to-end while
            // the runtime representation is designed.
            builder_chirho
                .ins()
                .iconst(cl_types_chirho::I64, s_chirho.len() as i64)
        }
    }
}

/// Lower a Core expression to a Cranelift SSA value.
///
/// This is the central dispatch function for expression lowering. It handles
/// all `CoreExprChirho` variants that can produce a value inside a function body.
/// Lambda abstractions (`LamChirho`) must be peeled off before calling this
/// function; reaching `LamChirho` here returns a zero placeholder.
pub fn lower_expr_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    expr_chirho: &CoreExprChirho,
) -> ClValueChirho {
    match expr_chirho {
        // ── Literals ───────────────────────────────────────────────────────
        CoreExprChirho::LitChirho(lit_chirho) => lower_lit_chirho(builder_chirho, lit_chirho),

        // ── Variables ──────────────────────────────────────────────────────
        CoreExprChirho::VarChirho(id_chirho) => {
            // First check if there is a Cranelift Variable (from a let-binding).
            if let Some(cl_var_chirho) = ctx_chirho.cl_vars_chirho.get(id_chirho) {
                builder_chirho.use_var(*cl_var_chirho)
            } else if let Some(val_chirho) = ctx_chirho.env_chirho.lookup_chirho(*id_chirho) {
                // Function parameters are stored directly as SSA values.
                val_chirho
            } else {
                // Unresolved variable — emit 0 as a safe placeholder.
                builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
            }
        }

        // ── Primitive operations ───────────────────────────────────────────
        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => lower_primop_chirho(builder_chirho, ctx_chirho, name_chirho, args_chirho),

        // ── Let bindings ───────────────────────────────────────────────────
        //
        // Non-recursive lets: evaluate each RHS, store in a Cranelift Variable,
        // then lower the body expression in the extended environment.
        //
        // Recursive lets: allocate all variables first, then define each RHS
        // (allowing forward references), then lower the body.
        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho,
            body_chirho,
        } => lower_let_chirho(builder_chirho, ctx_chirho, *rec_chirho, binds_chirho, body_chirho),

        // ── Case expressions ───────────────────────────────────────────────
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => lower_case_chirho(
            builder_chirho,
            ctx_chirho,
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
        ),

        // ── Type-level wrappers ────────────────────────────────────────────
        // TyLam/TyApp are erased at runtime; lower the inner expression.
        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            lower_expr_chirho(builder_chirho, ctx_chirho, body_chirho)
        }
        CoreExprChirho::TyAppChirho { expr_chirho: inner_chirho, .. } => {
            lower_expr_chirho(builder_chirho, ctx_chirho, inner_chirho)
        }

        // ── Function application ───────────────────────────────────────────
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => lower_app_chirho(builder_chirho, ctx_chirho, fun_chirho, arg_chirho),

        // ── Data constructor application ───────────────────────────────────
        // Simplified: return the constructor tag index (0-based) as an i64.
        // Full heap allocation would be added once the runtime is integrated.
        CoreExprChirho::ConAppChirho {
            con_name_chirho, ..
        } => {
            let tag_chirho = con_tag_for_chirho(con_name_chirho);
            builder_chirho
                .ins()
                .iconst(cl_types_chirho::I64, tag_chirho as i64)
        }

        // ── Lambda — should have been peeled by the caller ─────────────────
        CoreExprChirho::LamChirho { .. } => {
            // Nested lambdas beyond the arity computed in codegen are placeholders.
            builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
        }
    }
}

// ─── Let lowering ─────────────────────────────────────────────────────────────

fn lower_let_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    rec_chirho: bool,
    binds_chirho: &[(rhasky_core_chirho::expr_chirho::BinderChirho, CoreExprChirho)],
    body_chirho: &CoreExprChirho,
) -> ClValueChirho {
    if rec_chirho {
        // For recursive lets: allocate all variables first with a temporary 0
        // value, so that references between bindings can be resolved. This
        // matches the approach taken by most Cranelift users for letrec.
        let mut cl_vars_chirho: Vec<ClVariableChirho> = Vec::with_capacity(binds_chirho.len());
        for (binder_chirho, _) in binds_chirho {
            let var_chirho = ctx_chirho.fresh_var_chirho(binder_chirho.id_chirho, builder_chirho);
            let zero_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
            builder_chirho.def_var(var_chirho, zero_chirho);
            cl_vars_chirho.push(var_chirho);
        }
        // Now compute each RHS (may reference other bindings via use_var).
        for (idx_chirho, (_binder_chirho, rhs_chirho)) in binds_chirho.iter().enumerate() {
            let val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, rhs_chirho);
            // Normalise to i64 — detect float by inspecting the Cranelift type.
            let val_i64_chirho = ensure_i64_chirho(builder_chirho, val_chirho, false);
            builder_chirho.def_var(cl_vars_chirho[idx_chirho], val_i64_chirho);
        }
    } else {
        // Non-recursive: evaluate each RHS, store in a fresh Variable.
        for (binder_chirho, rhs_chirho) in binds_chirho {
            let val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, rhs_chirho);
            // Normalise to i64 (ensure_i64_chirho inspects the actual Cranelift type).
            let val_i64_chirho = ensure_i64_chirho(builder_chirho, val_chirho, false);
            let var_chirho = ctx_chirho.fresh_var_chirho(binder_chirho.id_chirho, builder_chirho);
            builder_chirho.def_var(var_chirho, val_i64_chirho);
        }
    }

    lower_expr_chirho(builder_chirho, ctx_chirho, body_chirho)
}

// ─── Case lowering ─────────────────────────────────────────────────────────────

/// Lower a `CaseChirho` to conditional branches.
///
/// Currently handles:
/// - Literal integer alts → compare + conditional jump
/// - Default (wildcard) alt → fall-through or unconditional jump
/// - Data constructor alts → compare tag + branch (simplified)
///
/// The result of each alternative is collected into a merge block so the
/// overall expression produces a single SSA value.
///
/// ## Block structure
///
/// Each concrete alt gets two blocks:
/// - A **check block** where the comparison is emitted and `brif` terminates it.
/// - An **alt body block** where the RHS is evaluated and jumps to merge.
///
/// The blocks are chained: check_0 → (alt_body_0 or check_1) → ... → default.
///
/// ```text
/// entry_block:
///   scrut = <evaluate scrutinee>
///   jump check_0
///
/// check_0:
///   eq = icmp eq scrut, expected_0
///   brif eq  alt_body_0  check_1
///
/// alt_body_0:
///   result_0 = <rhs_0>
///   jump merge(result_0)
///
/// check_1:
///   eq = icmp eq scrut, expected_1
///   brif eq  alt_body_1  default
///
/// alt_body_1:
///   result_1 = <rhs_1>
///   jump merge(result_1)
///
/// default:
///   result_d = <default rhs>  (or trap)
///   jump merge(result_d)
///
/// merge(result):
///   result
/// ```
fn lower_case_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    scrutinee_chirho: &CoreExprChirho,
    bind_chirho: &rhasky_core_chirho::expr_chirho::BinderChirho,
    alts_chirho: &[CoreAltChirho],
) -> ClValueChirho {
    // ── Evaluate the scrutinee in the current block ────────────────────────
    let scrut_val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, scrutinee_chirho);
    let scrut_i64_chirho = ensure_i64_chirho(builder_chirho, scrut_val_chirho, false);

    // Bind the scrutinee to the case binder so alts can reference it.
    ctx_chirho
        .env_chirho
        .bind_chirho(bind_chirho.id_chirho, scrut_i64_chirho);

    if alts_chirho.is_empty() {
        return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
    }

    // ── Merge block collects results from all alts ─────────────────────────
    let merge_block_chirho = builder_chirho.create_block();
    builder_chirho.append_block_param(merge_block_chirho, cl_types_chirho::I64);

    // ── Separate default alt from concrete alts ────────────────────────────
    let (default_alts_chirho, concrete_alts_chirho): (Vec<_>, Vec<_>) = alts_chirho
        .iter()
        .partition(|alt_chirho| matches!(alt_chirho.con_chirho, AltConChirho::DefaultChirho));

    // ── Build check blocks and alt body blocks ─────────────────────────────
    // check_blocks_chirho[i] is where we emit the icmp+brif for concrete alt i.
    // alt_body_blocks_chirho[i] is where we emit the RHS of concrete alt i.
    let check_blocks_chirho: Vec<cranelift_codegen::ir::Block> = concrete_alts_chirho
        .iter()
        .map(|_| builder_chirho.create_block())
        .collect();
    let alt_body_blocks_chirho: Vec<cranelift_codegen::ir::Block> = concrete_alts_chirho
        .iter()
        .map(|_| builder_chirho.create_block())
        .collect();
    let default_block_chirho = builder_chirho.create_block();

    // ── Jump from current block to the first check block (or default) ──────
    if check_blocks_chirho.is_empty() {
        builder_chirho.ins().jump(default_block_chirho, &[]);
    } else {
        builder_chirho.ins().jump(check_blocks_chirho[0], &[]);
    }

    // ── Emit each check block ──────────────────────────────────────────────
    for (idx_chirho, (alt_chirho, &check_block_chirho)) in
        concrete_alts_chirho.iter().zip(check_blocks_chirho.iter()).enumerate()
    {
        builder_chirho.switch_to_block(check_block_chirho);
        builder_chirho.seal_block(check_block_chirho);

        // The "no-match" destination is the next check block, or the default.
        let no_match_block_chirho = if idx_chirho + 1 < check_blocks_chirho.len() {
            check_blocks_chirho[idx_chirho + 1]
        } else {
            default_block_chirho
        };
        let alt_body_block_chirho = alt_body_blocks_chirho[idx_chirho];

        let expected_val_chirho = alt_expected_value_chirho(builder_chirho, &alt_chirho.con_chirho);
        let eq_val_chirho = builder_chirho.ins().icmp(
            IntCcChirho::Equal,
            scrut_i64_chirho,
            expected_val_chirho,
        );
        builder_chirho
            .ins()
            .brif(eq_val_chirho, alt_body_block_chirho, &[], no_match_block_chirho, &[]);
    }

    // ── Emit each alt body block ───────────────────────────────────────────
    for (alt_chirho, &alt_body_block_chirho) in
        concrete_alts_chirho.iter().zip(alt_body_blocks_chirho.iter())
    {
        builder_chirho.switch_to_block(alt_body_block_chirho);
        builder_chirho.seal_block(alt_body_block_chirho);

        // Bind alt binders to the scrutinee value (simplified for non-product alts).
        for binder_chirho in &alt_chirho.binders_chirho {
            ctx_chirho
                .env_chirho
                .bind_chirho(binder_chirho.id_chirho, scrut_i64_chirho);
        }

        let result_val_chirho =
            lower_expr_chirho(builder_chirho, ctx_chirho, &alt_chirho.rhs_chirho);
        let result_i64_chirho = ensure_i64_chirho(builder_chirho, result_val_chirho, false);
        builder_chirho.ins().jump(merge_block_chirho, &[result_i64_chirho]);
    }

    // ── Default block ──────────────────────────────────────────────────────
    builder_chirho.switch_to_block(default_block_chirho);
    builder_chirho.seal_block(default_block_chirho);

    let default_result_chirho = if let Some(default_alt_chirho) = default_alts_chirho.first() {
        for binder_chirho in &default_alt_chirho.binders_chirho {
            ctx_chirho
                .env_chirho
                .bind_chirho(binder_chirho.id_chirho, scrut_i64_chirho);
        }
        let val_chirho =
            lower_expr_chirho(builder_chirho, ctx_chirho, &default_alt_chirho.rhs_chirho);
        ensure_i64_chirho(builder_chirho, val_chirho, false)
    } else {
        // No default alt — pattern match is exhaustive; emit a trap.
        // `trap` is itself a block terminator so we must NOT emit another
        // terminator (jump) after it; just return a dummy value and skip
        // the jump.
        builder_chirho
            .ins()
            .trap(cranelift_codegen::ir::TrapCode::unwrap_user(1));
        // Skip the jump to merge — trap already terminates the block.
        builder_chirho.switch_to_block(merge_block_chirho);
        builder_chirho.seal_block(merge_block_chirho);
        return builder_chirho.block_params(merge_block_chirho)[0];
    };
    builder_chirho
        .ins()
        .jump(merge_block_chirho, &[default_result_chirho]);

    // ── Merge block — result is the block parameter ────────────────────────
    builder_chirho.switch_to_block(merge_block_chirho);
    builder_chirho.seal_block(merge_block_chirho);

    builder_chirho.block_params(merge_block_chirho)[0]
}

/// Compute the expected i64 value for a case alternative constructor/literal.
fn alt_expected_value_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    con_chirho: &AltConChirho,
) -> ClValueChirho {
    match con_chirho {
        AltConChirho::LitConChirho(lit_chirho) => match lit_chirho {
            CoreLitChirho::IntChirho(n_chirho) => {
                builder_chirho.ins().iconst(cl_types_chirho::I64, *n_chirho)
            }
            CoreLitChirho::CharChirho(c_chirho) => {
                builder_chirho
                    .ins()
                    .iconst(cl_types_chirho::I64, *c_chirho as i64)
            }
            CoreLitChirho::FloatChirho(f_chirho) => {
                // Float case: bit-cast the float bits to i64 for comparison.
                let bits_chirho = f_chirho.to_bits() as i64;
                builder_chirho.ins().iconst(cl_types_chirho::I64, bits_chirho)
            }
            CoreLitChirho::StringChirho(_) => {
                builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
            }
        },
        AltConChirho::DataConChirho(name_chirho) => {
            let tag_chirho = con_tag_for_chirho(name_chirho);
            builder_chirho
                .ins()
                .iconst(cl_types_chirho::I64, tag_chirho as i64)
        }
        AltConChirho::DefaultChirho => {
            builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
        }
    }
}

// ─── Application lowering ──────────────────────────────────────────────────────

/// Flatten App chains: `(((f a) b) c)` → `(f, [a, b, c])`.
fn flatten_apps_chirho(expr_chirho: &CoreExprChirho) -> (&CoreExprChirho, Vec<&CoreExprChirho>) {
    let mut args_chirho = Vec::new();
    let mut current_chirho = expr_chirho;
    while let CoreExprChirho::AppChirho { fun_chirho, arg_chirho } = current_chirho {
        args_chirho.push(arg_chirho.as_ref());
        current_chirho = fun_chirho;
    }
    args_chirho.reverse();
    (current_chirho, args_chirho)
}

/// Lower a function application `(fun arg)` to a Cranelift call.
///
/// For known top-level functions, emits a direct `call` instruction using
/// pre-imported `FuncRef`s from `func_ref_map_chirho`. For unknown callees,
/// falls back to `call_indirect`.
fn lower_app_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    fun_chirho: &CoreExprChirho,
    arg_chirho: &CoreExprChirho,
) -> ClValueChirho {
    // Flatten the full App chain to detect multi-arg direct calls.
    let full_expr_chirho = CoreExprChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho.clone()),
        arg_chirho: Box::new(arg_chirho.clone()),
    };
    let (callee_chirho, all_args_chirho) = flatten_apps_chirho(&full_expr_chirho);

    // Try direct call for known functions via pre-imported FuncRefs.
    if let CoreExprChirho::VarChirho(func_id_chirho) = callee_chirho {
        if let Some((func_ref_chirho, _arity_chirho)) =
            ctx_chirho.func_ref_map_chirho.get(func_id_chirho)
        {
            // Lower all arguments.
            let mut arg_vals_chirho = Vec::new();
            for a_chirho in &all_args_chirho {
                let val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, a_chirho);
                arg_vals_chirho.push(ensure_i64_chirho(builder_chirho, val_chirho, false));
            }

            let call_inst_chirho = builder_chirho
                .ins()
                .call(*func_ref_chirho, &arg_vals_chirho);
            return builder_chirho.inst_results(call_inst_chirho)[0];
        }
    }

    // Fallback: lower both sides and use call_indirect.
    let arg_val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, arg_chirho);
    let arg_i64_chirho = ensure_i64_chirho(builder_chirho, arg_val_chirho, false);

    let fun_ptr_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, fun_chirho);
    let fun_ptr_i64_chirho = ensure_i64_chirho(builder_chirho, fun_ptr_chirho, false);

    let mut sig_chirho = builder_chirho.func.stencil.signature.clone();
    sig_chirho.params.clear();
    sig_chirho.returns.clear();
    sig_chirho
        .params
        .push(cranelift_codegen::ir::AbiParam::new(cl_types_chirho::I64));
    sig_chirho
        .returns
        .push(cranelift_codegen::ir::AbiParam::new(cl_types_chirho::I64));

    let sig_ref_chirho = builder_chirho.import_signature(sig_chirho);
    let call_inst_chirho = builder_chirho
        .ins()
        .call_indirect(sig_ref_chirho, fun_ptr_i64_chirho, &[arg_i64_chirho]);
    builder_chirho.inst_results(call_inst_chirho)[0]
}

// ─── Primitive operation lowering ─────────────────────────────────────────────

/// Lower a primitive operation to Cranelift instructions.
///
/// Handles integer arithmetic, integer comparisons, float arithmetic,
/// and float comparisons. Unknown primops return 0.
pub fn lower_primop_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    name_chirho: &str,
    args_chirho: &[CoreExprChirho],
) -> ClValueChirho {
    // Evaluate operands.
    let lhs_raw_chirho = if !args_chirho.is_empty() {
        lower_expr_chirho(builder_chirho, ctx_chirho, &args_chirho[0])
    } else {
        builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
    };
    let rhs_raw_chirho = if args_chirho.len() > 1 {
        lower_expr_chirho(builder_chirho, ctx_chirho, &args_chirho[1])
    } else {
        builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
    };

    match name_chirho {
        // ── Integer arithmetic ─────────────────────────────────────────────
        "+#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            let rhs_chirho = ensure_i64_chirho(builder_chirho, rhs_raw_chirho, false);
            builder_chirho.ins().iadd(lhs_chirho, rhs_chirho)
        }
        "-#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            let rhs_chirho = ensure_i64_chirho(builder_chirho, rhs_raw_chirho, false);
            builder_chirho.ins().isub(lhs_chirho, rhs_chirho)
        }
        "*#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            let rhs_chirho = ensure_i64_chirho(builder_chirho, rhs_raw_chirho, false);
            builder_chirho.ins().imul(lhs_chirho, rhs_chirho)
        }
        "div#" | "quot#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            let rhs_chirho = ensure_i64_chirho(builder_chirho, rhs_raw_chirho, false);
            builder_chirho.ins().sdiv(lhs_chirho, rhs_chirho)
        }
        "mod#" | "rem#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            let rhs_chirho = ensure_i64_chirho(builder_chirho, rhs_raw_chirho, false);
            builder_chirho.ins().srem(lhs_chirho, rhs_chirho)
        }
        "negate#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            builder_chirho.ins().ineg(lhs_chirho)
        }
        "^#" => {
            // Integer power — simplified: return lhs^rhs via repeated multiply
            // (not emitted here; return placeholder i64 0 for now).
            ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false)
        }

        // ── Integer comparisons → 0 or 1 in i64 ───────────────────────────
        "==#" => icmp_to_i64_chirho(builder_chirho, IntCcChirho::Equal, lhs_raw_chirho, rhs_raw_chirho),
        "/=#" => icmp_to_i64_chirho(builder_chirho, IntCcChirho::NotEqual, lhs_raw_chirho, rhs_raw_chirho),
        "<#" => icmp_to_i64_chirho(
            builder_chirho,
            IntCcChirho::SignedLessThan,
            lhs_raw_chirho,
            rhs_raw_chirho,
        ),
        "<=#" => icmp_to_i64_chirho(
            builder_chirho,
            IntCcChirho::SignedLessThanOrEqual,
            lhs_raw_chirho,
            rhs_raw_chirho,
        ),
        ">#" => icmp_to_i64_chirho(
            builder_chirho,
            IntCcChirho::SignedGreaterThan,
            lhs_raw_chirho,
            rhs_raw_chirho,
        ),
        ">=#" => icmp_to_i64_chirho(
            builder_chirho,
            IntCcChirho::SignedGreaterThanOrEqual,
            lhs_raw_chirho,
            rhs_raw_chirho,
        ),

        // ── Float arithmetic ───────────────────────────────────────────────
        "+.#" => {
            let lhs_chirho = ensure_f64_chirho(builder_chirho, lhs_raw_chirho);
            let rhs_chirho = ensure_f64_chirho(builder_chirho, rhs_raw_chirho);
            let result_chirho = builder_chirho.ins().fadd(lhs_chirho, rhs_chirho);
            // Bitcast result back to i64 for uniform representation.
            builder_chirho.ins().bitcast(
                cl_types_chirho::I64,
                cranelift_codegen::ir::MemFlags::new(),
                result_chirho,
            )
        }
        "-.#" => {
            let lhs_chirho = ensure_f64_chirho(builder_chirho, lhs_raw_chirho);
            let rhs_chirho = ensure_f64_chirho(builder_chirho, rhs_raw_chirho);
            let result_chirho = builder_chirho.ins().fsub(lhs_chirho, rhs_chirho);
            builder_chirho.ins().bitcast(
                cl_types_chirho::I64,
                cranelift_codegen::ir::MemFlags::new(),
                result_chirho,
            )
        }
        "*.#" => {
            let lhs_chirho = ensure_f64_chirho(builder_chirho, lhs_raw_chirho);
            let rhs_chirho = ensure_f64_chirho(builder_chirho, rhs_raw_chirho);
            let result_chirho = builder_chirho.ins().fmul(lhs_chirho, rhs_chirho);
            builder_chirho.ins().bitcast(
                cl_types_chirho::I64,
                cranelift_codegen::ir::MemFlags::new(),
                result_chirho,
            )
        }
        "/.#" => {
            let lhs_chirho = ensure_f64_chirho(builder_chirho, lhs_raw_chirho);
            let rhs_chirho = ensure_f64_chirho(builder_chirho, rhs_raw_chirho);
            let result_chirho = builder_chirho.ins().fdiv(lhs_chirho, rhs_chirho);
            builder_chirho.ins().bitcast(
                cl_types_chirho::I64,
                cranelift_codegen::ir::MemFlags::new(),
                result_chirho,
            )
        }

        // ── Float comparisons → 0 or 1 in i64 ─────────────────────────────
        "eqFloat#" => fcmp_to_i64_chirho(
            builder_chirho,
            cranelift_codegen::ir::condcodes::FloatCC::Equal,
            lhs_raw_chirho,
            rhs_raw_chirho,
        ),
        "<.#" => fcmp_to_i64_chirho(
            builder_chirho,
            cranelift_codegen::ir::condcodes::FloatCC::LessThan,
            lhs_raw_chirho,
            rhs_raw_chirho,
        ),
        ">.#" => fcmp_to_i64_chirho(
            builder_chirho,
            cranelift_codegen::ir::condcodes::FloatCC::GreaterThan,
            lhs_raw_chirho,
            rhs_raw_chirho,
        ),

        // ── Boolean operations ─────────────────────────────────────────────
        "not#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            // not x = x XOR 1  (for 0/1 boolean representation)
            let one_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 1);
            builder_chirho.ins().bxor(lhs_chirho, one_chirho)
        }

        // ── Unknown primop ─────────────────────────────────────────────────
        _ => builder_chirho.ins().iconst(cl_types_chirho::I64, 0),
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Emit an integer comparison, returning the boolean result as 0 or 1 in i64.
fn icmp_to_i64_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    cc_chirho: IntCcChirho,
    lhs_chirho: ClValueChirho,
    rhs_chirho: ClValueChirho,
) -> ClValueChirho {
    let lhs_i64_chirho = ensure_i64_chirho(builder_chirho, lhs_chirho, false);
    let rhs_i64_chirho = ensure_i64_chirho(builder_chirho, rhs_chirho, false);
    let bool_val_chirho = builder_chirho
        .ins()
        .icmp(cc_chirho, lhs_i64_chirho, rhs_i64_chirho);
    // Extend the i1 result to i64 (0 or 1).
    builder_chirho
        .ins()
        .uextend(cl_types_chirho::I64, bool_val_chirho)
}

/// Emit a float comparison, returning the boolean result as 0 or 1 in i64.
fn fcmp_to_i64_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    cc_chirho: cranelift_codegen::ir::condcodes::FloatCC,
    lhs_chirho: ClValueChirho,
    rhs_chirho: ClValueChirho,
) -> ClValueChirho {
    let lhs_f64_chirho = ensure_f64_chirho(builder_chirho, lhs_chirho);
    let rhs_f64_chirho = ensure_f64_chirho(builder_chirho, rhs_chirho);
    let bool_val_chirho = builder_chirho.ins().fcmp(cc_chirho, lhs_f64_chirho, rhs_f64_chirho);
    builder_chirho
        .ins()
        .uextend(cl_types_chirho::I64, bool_val_chirho)
}

/// Ensure a value is of type i64.
/// If `is_float_chirho` is true, bitcast the f64 bits to i64 instead of reinterpreting.
/// If the value is already i64, it is returned unchanged.
pub fn ensure_i64_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    val_chirho: ClValueChirho,
    is_float_chirho: bool,
) -> ClValueChirho {
    let ty_chirho = builder_chirho.func.dfg.value_type(val_chirho);
    if ty_chirho == cl_types_chirho::I64 {
        val_chirho
    } else if ty_chirho == cl_types_chirho::F64 || is_float_chirho {
        builder_chirho.ins().bitcast(
            cl_types_chirho::I64,
            cranelift_codegen::ir::MemFlags::new(),
            val_chirho,
        )
    } else if ty_chirho == cl_types_chirho::I8
        || ty_chirho == cl_types_chirho::I16
        || ty_chirho == cl_types_chirho::I32
    {
        builder_chirho.ins().uextend(cl_types_chirho::I64, val_chirho)
    } else {
        // Unknown type — return as-is and let Cranelift validation catch it.
        val_chirho
    }
}

/// Ensure a value is of type f64.
/// If the value is already f64, it is returned unchanged.
/// If it is an i64 (bitcast representation), bitcast back to f64.
fn ensure_f64_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    val_chirho: ClValueChirho,
) -> ClValueChirho {
    let ty_chirho = builder_chirho.func.dfg.value_type(val_chirho);
    if ty_chirho == cl_types_chirho::F64 {
        val_chirho
    } else {
        builder_chirho.ins().bitcast(
            cl_types_chirho::F64,
            cranelift_codegen::ir::MemFlags::new(),
            val_chirho,
        )
    }
}

/// Map a constructor name to an integer tag.
///
/// Well-known constructors from the runtime are mapped to their tags.
/// Unknown constructors are mapped to a hash-derived value for stability.
pub fn con_tag_for_chirho(name_chirho: &str) -> u32 {
    match name_chirho {
        // Bool
        "False" => 0,
        "True" => 1,
        // Maybe
        "Nothing" => 0,
        "Just" => 1,
        // Either
        "Left" => 0,
        "Right" => 1,
        // Ordering
        "LT" => 0,
        "EQ" => 1,
        "GT" => 2,
        // List
        "[]" => 0,
        ":" => 1,
        // Unit
        "()" => 0,
        // Tuples
        "(,)" => 0,
        "(,,)" => 0,
        _ => {
            // Stable hash for unknown constructors (fnv-1a style, 32-bit).
            let mut hash_chirho: u32 = 2166136261;
            for byte_chirho in name_chirho.bytes() {
                hash_chirho ^= byte_chirho as u32;
                hash_chirho = hash_chirho.wrapping_mul(16777619);
            }
            hash_chirho
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn var_env_bind_lookup_chirho() {
        let env_chirho = VarEnvChirho::new_chirho();
        // We cannot create real ClValueChirho without a builder, so just test the None case.
        assert!(env_chirho.lookup_chirho(CoreIdChirho(0)).is_none());
    }

    #[test]
    fn con_tag_known_constructors_chirho() {
        assert_eq!(con_tag_for_chirho("False"), 0);
        assert_eq!(con_tag_for_chirho("True"), 1);
        assert_eq!(con_tag_for_chirho("Nothing"), 0);
        assert_eq!(con_tag_for_chirho("Just"), 1);
        assert_eq!(con_tag_for_chirho("LT"), 0);
        assert_eq!(con_tag_for_chirho("EQ"), 1);
        assert_eq!(con_tag_for_chirho("GT"), 2);
        assert_eq!(con_tag_for_chirho("[]"), 0);
        assert_eq!(con_tag_for_chirho(":"), 1);
    }

    #[test]
    fn con_tag_unknown_is_stable_chirho() {
        // Unknown constructors must produce a stable hash (deterministic).
        let tag_a_chirho = con_tag_for_chirho("MyConstructorChirho");
        let tag_b_chirho = con_tag_for_chirho("MyConstructorChirho");
        assert_eq!(tag_a_chirho, tag_b_chirho);
        // Different names must differ.
        let tag_c_chirho = con_tag_for_chirho("OtherConstructorChirho");
        assert_ne!(tag_a_chirho, tag_c_chirho);
    }
}
