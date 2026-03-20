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
//! | `ConAppChirho`       | Immediate tag or boxed tag+fields             |
//! | `LamChirho`          | Error — must be peeled by `codegen_chirho`    |

use cranelift_codegen::ir::condcodes::IntCC as IntCcChirho;
use cranelift_codegen::ir::types as cl_types_chirho;
use cranelift_codegen::ir::{InstBuilder as _, Value as ClValueChirho};
use cranelift_frontend::FunctionBuilder as FuncBuilderChirho;
use cranelift_frontend::Variable as ClVariableChirho;

use haskelujah_core_chirho::expr_chirho::{
    AltConChirho, CoreAltChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho,
};

use std::collections::HashMap;

pub type PapWrapperKeyChirho = (CoreIdChirho, usize);

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
    /// Map of CoreId → (FuncRef, arity) for direct calls to known functions.
    /// This includes top-level bindings and lifted local lambdas, each already
    /// imported into the current function via `declare_func_in_func`.
    pub func_ref_map_chirho: &'a HashMap<CoreIdChirho, (cranelift_codegen::ir::FuncRef, usize)>,
    /// Captured outer runtime ids for lifted local functions. Top-level
    /// functions are absent; lifted helpers list the extra values that must be
    /// appended to direct calls in declaration order.
    pub lifted_capture_ids_chirho: &'a HashMap<CoreIdChirho, Vec<CoreIdChirho>>,
    /// Map of (target CoreId, applied arg count) → wrapper FuncRef for boxed
    /// partial applications.
    pub pap_wrapper_ref_map_chirho:
        &'a HashMap<PapWrapperKeyChirho, cranelift_codegen::ir::FuncRef>,
    /// Map of CoreId → name for detecting Prelude functions (putStrLn, print, etc.)
    pub toplevel_names_chirho: &'a HashMap<CoreIdChirho, String>,
    /// Optional FuncRef for RTS `haskelujah_put_str_ln_chirho`
    pub put_str_ln_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Optional FuncRef for RTS `haskelujah_print_int_chirho` (non-variadic print)
    pub print_int_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Optional FuncRef for RTS `haskelujah_append_str_chirho`
    pub append_str_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Optional FuncRef for RTS `haskelujah_alloc_chirho` (boxed constructors)
    pub alloc_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Optional FuncRef for RTS `haskelujah_show_int_chirho` (int → string)
    pub show_int_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Optional FuncRef for RTS `haskelujah_show_bool_chirho` (bool → string)
    pub show_bool_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Optional FuncRef for RTS `haskelujah_show_char_chirho` (char → string)
    pub show_char_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Optional FuncRef for RTS `haskelujah_show_float_chirho` (f64 bits → string)
    pub show_float_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Optional FuncRef for RTS `haskelujah_put_str_chirho` (no newline)
    pub put_str_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Optional FuncRef for RTS `haskelujah_get_line_chirho` (read stdin)
    pub get_line_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Optional FuncRef for RTS `haskelujah_write_file_chirho`
    pub write_file_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Optional FuncRef for RTS `haskelujah_read_file_chirho`
    pub read_file_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Optional FuncRef for RTS `haskelujah_unpack_string_chirho`
    pub unpack_string_ref_chirho: Option<cranelift_codegen::ir::FuncRef>,
    /// Map of string content → GlobalValue for data section string literals
    pub string_globals_chirho: HashMap<String, cranelift_codegen::ir::GlobalValue>,
    /// Current function Core id, used for self-tail-call elimination.
    pub tco_self_id_chirho: Option<CoreIdChirho>,
    /// Optional loop-header block for self-tail-call elimination.
    pub tco_loop_block_chirho: Option<cranelift_codegen::ir::Block>,
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
        CoreLitChirho::CharChirho(c_chirho) => builder_chirho
            .ins()
            .iconst(cl_types_chirho::I64, *c_chirho as i64),
        CoreLitChirho::StringChirho(_s_chirho) => {
            // String literals are embedded in the data section.
            // The actual pointer is resolved via lower_string_lit_chirho
            // which needs the LowerCtxChirho. As a fallback, return 0.
            builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
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
        CoreExprChirho::LitChirho(lit_chirho) => {
            // Check for string literals with data section globals
            if let CoreLitChirho::StringChirho(s_chirho) = lit_chirho {
                if let Some(gv_chirho) = ctx_chirho.string_globals_chirho.get(s_chirho.as_str()) {
                    return builder_chirho
                        .ins()
                        .global_value(cl_types_chirho::I64, *gv_chirho);
                }
            }
            lower_lit_chirho(builder_chirho, lit_chirho)
        }

        // ── Variables ──────────────────────────────────────────────────────
        CoreExprChirho::VarChirho(id_chirho) => {
            // First check if there is a Cranelift Variable (from a let-binding).
            if let Some(cl_var_chirho) = ctx_chirho.cl_vars_chirho.get(id_chirho) {
                builder_chirho.use_var(*cl_var_chirho)
            } else if let Some(val_chirho) = ctx_chirho.env_chirho.lookup_chirho(*id_chirho) {
                // Function parameters are stored directly as SSA values.
                val_chirho
            } else if let Some((func_ref_chirho, arity_chirho)) =
                ctx_chirho.func_ref_map_chirho.get(id_chirho)
            {
                lower_known_function_value_chirho(
                    builder_chirho,
                    ctx_chirho,
                    *id_chirho,
                    *func_ref_chirho,
                    *arity_chirho,
                )
            } else if let Some(name_chirho) = ctx_chirho.toplevel_names_chirho.get(id_chirho) {
                // Check for 0-arg IO actions that should be called immediately
                if matches!(name_chirho.as_str(), "getLine" | "getLine#") {
                    if let Some(get_line_ref_chirho) = ctx_chirho.get_line_ref_chirho {
                        let call_chirho = builder_chirho.ins().call(get_line_ref_chirho, &[]);
                        return builder_chirho.inst_results(call_chirho)[0];
                    }
                }
                // Other unresolved named variables — emit 0 as placeholder
                builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
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
        } => lower_let_chirho(
            builder_chirho,
            ctx_chirho,
            *rec_chirho,
            binds_chirho,
            body_chirho,
        ),

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
        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ..
        } => lower_expr_chirho(builder_chirho, ctx_chirho, inner_chirho),

        // ── Function application ───────────────────────────────────────────
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => lower_app_chirho(builder_chirho, ctx_chirho, fun_chirho, arg_chirho),

        CoreExprChirho::ConAppChirho {
            con_name_chirho, ..
        } => lower_constructor_app_chirho(builder_chirho, ctx_chirho, con_name_chirho, expr_chirho),

        // ── Lambda — should have been peeled by the caller ─────────────────
        CoreExprChirho::LamChirho { .. } => {
            // Nested lambdas beyond the arity computed in codegen are placeholders.
            builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
        }
    }
}

pub enum TailLowerOutcomeChirho {
    ValueChirho(ClValueChirho),
    TerminatedChirho,
}

pub fn lower_tail_expr_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    expr_chirho: &CoreExprChirho,
) -> TailLowerOutcomeChirho {
    match expr_chirho {
        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho,
            body_chirho,
        } => lower_tail_let_chirho(
            builder_chirho,
            ctx_chirho,
            *rec_chirho,
            binds_chirho,
            body_chirho,
        ),
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => lower_tail_case_chirho(
            builder_chirho,
            ctx_chirho,
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
        ),
        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            lower_tail_expr_chirho(builder_chirho, ctx_chirho, body_chirho)
        }
        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ..
        } => lower_tail_expr_chirho(builder_chirho, ctx_chirho, inner_chirho),
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            if lower_self_tail_call_chirho(builder_chirho, ctx_chirho, fun_chirho, arg_chirho) {
                TailLowerOutcomeChirho::TerminatedChirho
            } else {
                TailLowerOutcomeChirho::ValueChirho(lower_app_chirho(
                    builder_chirho,
                    ctx_chirho,
                    fun_chirho,
                    arg_chirho,
                ))
            }
        }
        _ => TailLowerOutcomeChirho::ValueChirho(lower_expr_chirho(
            builder_chirho,
            ctx_chirho,
            expr_chirho,
        )),
    }
}

// ─── Let lowering ─────────────────────────────────────────────────────────────

fn lower_let_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    rec_chirho: bool,
    binds_chirho: &[(
        haskelujah_core_chirho::expr_chirho::BinderChirho,
        CoreExprChirho,
    )],
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
        for (idx_chirho, (binder_chirho, rhs_chirho)) in binds_chirho.iter().enumerate() {
            let val_chirho = lower_let_rhs_value_chirho(
                builder_chirho,
                ctx_chirho,
                binder_chirho.id_chirho,
                rhs_chirho,
            );
            // Normalise to i64 — detect float by inspecting the Cranelift type.
            let val_i64_chirho = ensure_i64_chirho(builder_chirho, val_chirho, false);
            builder_chirho.def_var(cl_vars_chirho[idx_chirho], val_i64_chirho);
        }
    } else {
        // Non-recursive: evaluate each RHS, store in a fresh Variable.
        for (binder_chirho, rhs_chirho) in binds_chirho {
            let val_chirho = lower_let_rhs_value_chirho(
                builder_chirho,
                ctx_chirho,
                binder_chirho.id_chirho,
                rhs_chirho,
            );
            // Normalise to i64 (ensure_i64_chirho inspects the actual Cranelift type).
            let val_i64_chirho = ensure_i64_chirho(builder_chirho, val_chirho, false);
            let var_chirho = ctx_chirho.fresh_var_chirho(binder_chirho.id_chirho, builder_chirho);
            builder_chirho.def_var(var_chirho, val_i64_chirho);
        }
    }

    lower_expr_chirho(builder_chirho, ctx_chirho, body_chirho)
}

fn lower_tail_let_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    rec_chirho: bool,
    binds_chirho: &[(
        haskelujah_core_chirho::expr_chirho::BinderChirho,
        CoreExprChirho,
    )],
    body_chirho: &CoreExprChirho,
) -> TailLowerOutcomeChirho {
    if rec_chirho {
        let mut cl_vars_chirho: Vec<ClVariableChirho> = Vec::with_capacity(binds_chirho.len());
        for (binder_chirho, _) in binds_chirho {
            let var_chirho = ctx_chirho.fresh_var_chirho(binder_chirho.id_chirho, builder_chirho);
            let zero_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
            builder_chirho.def_var(var_chirho, zero_chirho);
            cl_vars_chirho.push(var_chirho);
        }
        for (idx_chirho, (binder_chirho, rhs_chirho)) in binds_chirho.iter().enumerate() {
            let val_chirho = lower_let_rhs_value_chirho(
                builder_chirho,
                ctx_chirho,
                binder_chirho.id_chirho,
                rhs_chirho,
            );
            let val_i64_chirho = ensure_i64_chirho(builder_chirho, val_chirho, false);
            builder_chirho.def_var(cl_vars_chirho[idx_chirho], val_i64_chirho);
        }
    } else {
        for (binder_chirho, rhs_chirho) in binds_chirho {
            let val_chirho = lower_let_rhs_value_chirho(
                builder_chirho,
                ctx_chirho,
                binder_chirho.id_chirho,
                rhs_chirho,
            );
            let val_i64_chirho = ensure_i64_chirho(builder_chirho, val_chirho, false);
            let var_chirho = ctx_chirho.fresh_var_chirho(binder_chirho.id_chirho, builder_chirho);
            builder_chirho.def_var(var_chirho, val_i64_chirho);
        }
    }

    lower_tail_expr_chirho(builder_chirho, ctx_chirho, body_chirho)
}

fn lambda_arity_chirho(expr_chirho: &CoreExprChirho) -> usize {
    match expr_chirho {
        CoreExprChirho::LamChirho { body_chirho, .. } => 1 + lambda_arity_chirho(body_chirho),
        CoreExprChirho::TyLamChirho { body_chirho, .. } => lambda_arity_chirho(body_chirho),
        _ => 0,
    }
}

fn lower_let_rhs_value_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    binder_id_chirho: CoreIdChirho,
    rhs_chirho: &CoreExprChirho,
) -> ClValueChirho {
    let lambda_arity_chirho = lambda_arity_chirho(rhs_chirho);
    if lambda_arity_chirho > 0 {
        if let Some((func_ref_chirho, arity_chirho)) =
            ctx_chirho.func_ref_map_chirho.get(&binder_id_chirho)
        {
            return lower_known_function_value_chirho(
                builder_chirho,
                ctx_chirho,
                binder_id_chirho,
                *func_ref_chirho,
                *arity_chirho,
            );
        }
    }
    lower_expr_chirho(builder_chirho, ctx_chirho, rhs_chirho)
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
    bind_chirho: &haskelujah_core_chirho::expr_chirho::BinderChirho,
    alts_chirho: &[CoreAltChirho],
) -> ClValueChirho {
    // ── Evaluate the scrutinee in the current block ────────────────────────
    let scrut_val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, scrutinee_chirho);
    let scrut_i64_chirho = ensure_i64_chirho(builder_chirho, scrut_val_chirho, false);
    let needs_constructor_tags_chirho = concrete_alts_need_constructor_tag_chirho(alts_chirho);
    let scrut_cmp_i64_chirho = if needs_constructor_tags_chirho {
        load_constructor_tag_chirho(builder_chirho, scrut_i64_chirho)
    } else {
        scrut_i64_chirho
    };

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
    for (idx_chirho, (alt_chirho, &check_block_chirho)) in concrete_alts_chirho
        .iter()
        .zip(check_blocks_chirho.iter())
        .enumerate()
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
            scrut_cmp_i64_chirho,
            expected_val_chirho,
        );
        builder_chirho.ins().brif(
            eq_val_chirho,
            alt_body_block_chirho,
            &[],
            no_match_block_chirho,
            &[],
        );
    }

    // ── Emit each alt body block ───────────────────────────────────────────
    for (alt_chirho, &alt_body_block_chirho) in concrete_alts_chirho
        .iter()
        .zip(alt_body_blocks_chirho.iter())
    {
        builder_chirho.switch_to_block(alt_body_block_chirho);
        builder_chirho.seal_block(alt_body_block_chirho);

        if matches!(alt_chirho.con_chirho, AltConChirho::DataConChirho(_)) {
            for (field_idx_chirho, binder_chirho) in alt_chirho.binders_chirho.iter().enumerate() {
                let field_val_chirho = load_constructor_field_chirho(
                    builder_chirho,
                    scrut_i64_chirho,
                    field_idx_chirho,
                );
                ctx_chirho
                    .env_chirho
                    .bind_chirho(binder_chirho.id_chirho, field_val_chirho);
            }
        } else {
            for binder_chirho in &alt_chirho.binders_chirho {
                ctx_chirho
                    .env_chirho
                    .bind_chirho(binder_chirho.id_chirho, scrut_i64_chirho);
            }
        }

        let result_val_chirho =
            lower_expr_chirho(builder_chirho, ctx_chirho, &alt_chirho.rhs_chirho);
        let result_i64_chirho = ensure_i64_chirho(builder_chirho, result_val_chirho, false);
        builder_chirho
            .ins()
            .jump(merge_block_chirho, &[result_i64_chirho]);
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

fn lower_tail_case_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    scrutinee_chirho: &CoreExprChirho,
    bind_chirho: &haskelujah_core_chirho::expr_chirho::BinderChirho,
    alts_chirho: &[CoreAltChirho],
) -> TailLowerOutcomeChirho {
    let scrut_val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, scrutinee_chirho);
    let scrut_i64_chirho = ensure_i64_chirho(builder_chirho, scrut_val_chirho, false);
    let needs_constructor_tags_chirho = concrete_alts_need_constructor_tag_chirho(alts_chirho);
    let scrut_cmp_i64_chirho = if needs_constructor_tags_chirho {
        load_constructor_tag_chirho(builder_chirho, scrut_i64_chirho)
    } else {
        scrut_i64_chirho
    };

    ctx_chirho
        .env_chirho
        .bind_chirho(bind_chirho.id_chirho, scrut_i64_chirho);

    if alts_chirho.is_empty() {
        return TailLowerOutcomeChirho::ValueChirho(
            builder_chirho.ins().iconst(cl_types_chirho::I64, 0),
        );
    }

    let merge_block_chirho = builder_chirho.create_block();
    builder_chirho.append_block_param(merge_block_chirho, cl_types_chirho::I64);

    let (default_alts_chirho, concrete_alts_chirho): (Vec<_>, Vec<_>) = alts_chirho
        .iter()
        .partition(|alt_chirho| matches!(alt_chirho.con_chirho, AltConChirho::DefaultChirho));

    let check_blocks_chirho: Vec<cranelift_codegen::ir::Block> = concrete_alts_chirho
        .iter()
        .map(|_| builder_chirho.create_block())
        .collect();
    let alt_body_blocks_chirho: Vec<cranelift_codegen::ir::Block> = concrete_alts_chirho
        .iter()
        .map(|_| builder_chirho.create_block())
        .collect();
    let default_block_chirho = builder_chirho.create_block();

    if check_blocks_chirho.is_empty() {
        builder_chirho.ins().jump(default_block_chirho, &[]);
    } else {
        builder_chirho.ins().jump(check_blocks_chirho[0], &[]);
    }

    for (idx_chirho, (alt_chirho, &check_block_chirho)) in concrete_alts_chirho
        .iter()
        .zip(check_blocks_chirho.iter())
        .enumerate()
    {
        builder_chirho.switch_to_block(check_block_chirho);
        builder_chirho.seal_block(check_block_chirho);

        let no_match_block_chirho = if idx_chirho + 1 < check_blocks_chirho.len() {
            check_blocks_chirho[idx_chirho + 1]
        } else {
            default_block_chirho
        };
        let alt_body_block_chirho = alt_body_blocks_chirho[idx_chirho];

        let expected_val_chirho = alt_expected_value_chirho(builder_chirho, &alt_chirho.con_chirho);
        let eq_val_chirho = builder_chirho.ins().icmp(
            IntCcChirho::Equal,
            scrut_cmp_i64_chirho,
            expected_val_chirho,
        );
        builder_chirho.ins().brif(
            eq_val_chirho,
            alt_body_block_chirho,
            &[],
            no_match_block_chirho,
            &[],
        );
    }

    let mut merge_incoming_count_chirho = 0usize;
    for (alt_chirho, &alt_body_block_chirho) in concrete_alts_chirho
        .iter()
        .zip(alt_body_blocks_chirho.iter())
    {
        builder_chirho.switch_to_block(alt_body_block_chirho);
        builder_chirho.seal_block(alt_body_block_chirho);

        if matches!(alt_chirho.con_chirho, AltConChirho::DataConChirho(_)) {
            for (field_idx_chirho, binder_chirho) in alt_chirho.binders_chirho.iter().enumerate() {
                let field_val_chirho = load_constructor_field_chirho(
                    builder_chirho,
                    scrut_i64_chirho,
                    field_idx_chirho,
                );
                ctx_chirho
                    .env_chirho
                    .bind_chirho(binder_chirho.id_chirho, field_val_chirho);
            }
        } else {
            for binder_chirho in &alt_chirho.binders_chirho {
                ctx_chirho
                    .env_chirho
                    .bind_chirho(binder_chirho.id_chirho, scrut_i64_chirho);
            }
        }

        match lower_tail_expr_chirho(builder_chirho, ctx_chirho, &alt_chirho.rhs_chirho) {
            TailLowerOutcomeChirho::ValueChirho(result_val_chirho) => {
                let result_i64_chirho = ensure_i64_chirho(builder_chirho, result_val_chirho, false);
                builder_chirho
                    .ins()
                    .jump(merge_block_chirho, &[result_i64_chirho]);
                merge_incoming_count_chirho += 1;
            }
            TailLowerOutcomeChirho::TerminatedChirho => {}
        }
    }

    builder_chirho.switch_to_block(default_block_chirho);
    builder_chirho.seal_block(default_block_chirho);

    if let Some(default_alt_chirho) = default_alts_chirho.first() {
        for binder_chirho in &default_alt_chirho.binders_chirho {
            ctx_chirho
                .env_chirho
                .bind_chirho(binder_chirho.id_chirho, scrut_i64_chirho);
        }
        match lower_tail_expr_chirho(builder_chirho, ctx_chirho, &default_alt_chirho.rhs_chirho) {
            TailLowerOutcomeChirho::ValueChirho(default_val_chirho) => {
                let default_i64_chirho =
                    ensure_i64_chirho(builder_chirho, default_val_chirho, false);
                builder_chirho
                    .ins()
                    .jump(merge_block_chirho, &[default_i64_chirho]);
                merge_incoming_count_chirho += 1;
            }
            TailLowerOutcomeChirho::TerminatedChirho => {}
        }
    } else {
        builder_chirho
            .ins()
            .trap(cranelift_codegen::ir::TrapCode::unwrap_user(1));
    }

    if merge_incoming_count_chirho == 0 {
        return TailLowerOutcomeChirho::TerminatedChirho;
    }

    builder_chirho.switch_to_block(merge_block_chirho);
    builder_chirho.seal_block(merge_block_chirho);
    TailLowerOutcomeChirho::ValueChirho(builder_chirho.block_params(merge_block_chirho)[0])
}

fn lower_constructor_app_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    con_name_chirho: &str,
    expr_chirho: &CoreExprChirho,
) -> ClValueChirho {
    let tag_chirho = con_tag_for_chirho(con_name_chirho) as i64;
    let CoreExprChirho::ConAppChirho { args_chirho, .. } = expr_chirho else {
        return builder_chirho
            .ins()
            .iconst(cl_types_chirho::I64, tag_chirho);
    };
    if args_chirho.is_empty() {
        return builder_chirho
            .ins()
            .iconst(cl_types_chirho::I64, tag_chirho);
    }
    let Some(alloc_ref_chirho) = ctx_chirho.alloc_ref_chirho else {
        return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
    };

    let alloc_size_chirho = ((args_chirho.len() + 1) * 8) as i64;
    let alloc_size_val_chirho = builder_chirho
        .ins()
        .iconst(cl_types_chirho::I64, alloc_size_chirho);
    let alloc_call_chirho = builder_chirho
        .ins()
        .call(alloc_ref_chirho, &[alloc_size_val_chirho]);
    let alloc_ptr_chirho = builder_chirho.inst_results(alloc_call_chirho)[0];
    let mem_flags_chirho = cranelift_codegen::ir::MemFlags::new();

    let tag_val_chirho = builder_chirho
        .ins()
        .iconst(cl_types_chirho::I64, tag_chirho);
    builder_chirho
        .ins()
        .store(mem_flags_chirho, tag_val_chirho, alloc_ptr_chirho, 0);

    for (field_idx_chirho, arg_chirho) in args_chirho.iter().enumerate() {
        let field_val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, arg_chirho);
        let field_i64_chirho = ensure_i64_chirho(builder_chirho, field_val_chirho, false);
        let offset_chirho = ((field_idx_chirho + 1) * 8) as i32;
        builder_chirho.ins().store(
            mem_flags_chirho,
            field_i64_chirho,
            alloc_ptr_chirho,
            offset_chirho,
        );
    }

    let boxed_mask_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, i64::MIN);
    builder_chirho
        .ins()
        .bor(alloc_ptr_chirho, boxed_mask_chirho)
}

fn concrete_alts_need_constructor_tag_chirho(alts_chirho: &[CoreAltChirho]) -> bool {
    alts_chirho
        .iter()
        .any(|alt_chirho| matches!(alt_chirho.con_chirho, AltConChirho::DataConChirho(_)))
}

fn looks_like_data_constructor_name_chirho(name_chirho: &str) -> bool {
    if matches!(name_chirho, "[]" | ":") {
        return true;
    }
    if name_chirho.starts_with("$Dict_") || name_chirho.starts_with("$tuple") {
        return true;
    }
    if name_chirho.starts_with('(') && name_chirho.ends_with(')') {
        return true;
    }
    name_chirho
        .chars()
        .next()
        .is_some_and(|chirho| chirho.is_ascii_uppercase())
}

fn load_constructor_tag_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    scrut_i64_chirho: ClValueChirho,
) -> ClValueChirho {
    let boxed_block_chirho = builder_chirho.create_block();
    let immediate_block_chirho = builder_chirho.create_block();
    let join_block_chirho = builder_chirho.create_block();
    builder_chirho.append_block_param(join_block_chirho, cl_types_chirho::I64);

    let zero_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
    let is_boxed_chirho =
        builder_chirho
            .ins()
            .icmp(IntCcChirho::SignedLessThan, scrut_i64_chirho, zero_chirho);
    builder_chirho.ins().brif(
        is_boxed_chirho,
        boxed_block_chirho,
        &[],
        immediate_block_chirho,
        &[],
    );

    builder_chirho.switch_to_block(immediate_block_chirho);
    builder_chirho
        .ins()
        .jump(join_block_chirho, &[scrut_i64_chirho]);
    builder_chirho.seal_block(immediate_block_chirho);

    builder_chirho.switch_to_block(boxed_block_chirho);
    let boxed_ptr_chirho = decode_boxed_constructor_ptr_chirho(builder_chirho, scrut_i64_chirho);
    let boxed_tag_chirho = builder_chirho.ins().load(
        cl_types_chirho::I64,
        cranelift_codegen::ir::MemFlags::new(),
        boxed_ptr_chirho,
        0,
    );
    builder_chirho
        .ins()
        .jump(join_block_chirho, &[boxed_tag_chirho]);
    builder_chirho.seal_block(boxed_block_chirho);

    builder_chirho.switch_to_block(join_block_chirho);
    builder_chirho.seal_block(join_block_chirho);
    builder_chirho.block_params(join_block_chirho)[0]
}

fn decode_boxed_constructor_ptr_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    scrut_i64_chirho: ClValueChirho,
) -> ClValueChirho {
    let ptr_mask_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, i64::MAX);
    builder_chirho.ins().band(scrut_i64_chirho, ptr_mask_chirho)
}

fn load_constructor_field_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    scrut_i64_chirho: ClValueChirho,
    field_idx_chirho: usize,
) -> ClValueChirho {
    let boxed_ptr_chirho = decode_boxed_constructor_ptr_chirho(builder_chirho, scrut_i64_chirho);
    let offset_chirho = ((field_idx_chirho + 1) * 8) as i32;
    builder_chirho.ins().load(
        cl_types_chirho::I64,
        cranelift_codegen::ir::MemFlags::new(),
        boxed_ptr_chirho,
        offset_chirho,
    )
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
            CoreLitChirho::CharChirho(c_chirho) => builder_chirho
                .ins()
                .iconst(cl_types_chirho::I64, *c_chirho as i64),
            CoreLitChirho::FloatChirho(f_chirho) => {
                // Float case: bit-cast the float bits to i64 for comparison.
                let bits_chirho = f_chirho.to_bits() as i64;
                builder_chirho
                    .ins()
                    .iconst(cl_types_chirho::I64, bits_chirho)
            }
            CoreLitChirho::StringChirho(_) => builder_chirho.ins().iconst(cl_types_chirho::I64, 0),
        },
        AltConChirho::DataConChirho(name_chirho) => {
            let tag_chirho = con_tag_for_chirho(name_chirho);
            builder_chirho
                .ins()
                .iconst(cl_types_chirho::I64, tag_chirho as i64)
        }
        AltConChirho::DefaultChirho => builder_chirho.ins().iconst(cl_types_chirho::I64, 0),
    }
}

// ─── Application lowering ──────────────────────────────────────────────────────

/// Flatten App chains: `(((f a) b) c)` → `(f, [a, b, c])`.
fn flatten_apps_chirho(expr_chirho: &CoreExprChirho) -> (&CoreExprChirho, Vec<&CoreExprChirho>) {
    let mut args_chirho = Vec::new();
    let mut current_chirho = expr_chirho;
    while let CoreExprChirho::AppChirho {
        fun_chirho,
        arg_chirho,
    } = current_chirho
    {
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
    let callee_chirho = strip_runtime_wrappers_chirho(callee_chirho);

    // Try direct call for known functions via pre-imported FuncRefs.
    if let CoreExprChirho::VarChirho(func_id_chirho) = callee_chirho {
        // Check for Prelude IO functions (putStrLn, print)
        if let Some(name_chirho) = ctx_chirho.toplevel_names_chirho.get(func_id_chirho) {
            if looks_like_data_constructor_name_chirho(name_chirho) {
                let constructor_expr_chirho = CoreExprChirho::ConAppChirho {
                    con_name_chirho: name_chirho.clone(),
                    args_chirho: all_args_chirho
                        .iter()
                        .map(|arg_chirho| (*arg_chirho).clone())
                        .collect(),
                };
                return lower_constructor_app_chirho(
                    builder_chirho,
                    ctx_chirho,
                    name_chirho,
                    &constructor_expr_chirho,
                );
            }
            // putStr :: String -> IO () (no trailing newline)
            if matches!(name_chirho.as_str(), "putStr" | "putStr#") {
                if let Some(put_str_ref_chirho) = ctx_chirho.put_str_ref_chirho {
                    if let Some(arg_expr_chirho) = all_args_chirho.last() {
                        let arg_val_chirho =
                            lower_expr_chirho(builder_chirho, ctx_chirho, arg_expr_chirho);
                        let arg_i64_chirho =
                            ensure_i64_chirho(builder_chirho, arg_val_chirho, false);
                        builder_chirho
                            .ins()
                            .call(put_str_ref_chirho, &[arg_i64_chirho]);
                        return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
                    }
                }
            }
            // getLine :: IO String
            if matches!(name_chirho.as_str(), "getLine" | "getLine#") {
                if let Some(get_line_ref_chirho) = ctx_chirho.get_line_ref_chirho {
                    let call_chirho = builder_chirho.ins().call(get_line_ref_chirho, &[]);
                    return builder_chirho.inst_results(call_chirho)[0];
                }
            }
            // writeFile :: FilePath -> String -> IO ()
            if matches!(name_chirho.as_str(), "writeFile" | "writeFile#") {
                if let Some(write_file_ref_chirho) = ctx_chirho.write_file_ref_chirho {
                    if all_args_chirho.len() >= 2 {
                        let path_val_chirho =
                            lower_expr_chirho(builder_chirho, ctx_chirho, all_args_chirho[0]);
                        let content_val_chirho =
                            lower_expr_chirho(builder_chirho, ctx_chirho, all_args_chirho[1]);
                        let path_i64_chirho =
                            ensure_i64_chirho(builder_chirho, path_val_chirho, false);
                        let content_i64_chirho =
                            ensure_i64_chirho(builder_chirho, content_val_chirho, false);
                        builder_chirho.ins().call(
                            write_file_ref_chirho,
                            &[path_i64_chirho, content_i64_chirho],
                        );
                        return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
                    }
                }
            }
            // toUpper :: Char -> Char
            if matches!(name_chirho.as_str(), "toUpper" | "toUpper#") {
                if let Some(arg_expr_chirho) = all_args_chirho.last() {
                    let val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, arg_expr_chirho);
                    let v_chirho = ensure_i64_chirho(builder_chirho, val_chirho, false);
                    let c97_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 97);
                    let c122_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 122);
                    let c32_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 32);
                    let is_lower_chirho = builder_chirho.ins().icmp(IntCcChirho::SignedGreaterThanOrEqual, v_chirho, c97_chirho);
                    let is_lower_end_chirho = builder_chirho.ins().icmp(IntCcChirho::SignedLessThanOrEqual, v_chirho, c122_chirho);
                    let in_range_chirho = builder_chirho.ins().band(is_lower_chirho, is_lower_end_chirho);
                    let upper_chirho = builder_chirho.ins().isub(v_chirho, c32_chirho);
                    return builder_chirho.ins().select(in_range_chirho, upper_chirho, v_chirho);
                }
            }
            // toLower :: Char -> Char
            if matches!(name_chirho.as_str(), "toLower" | "toLower#") {
                if let Some(arg_expr_chirho) = all_args_chirho.last() {
                    let val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, arg_expr_chirho);
                    let v_chirho = ensure_i64_chirho(builder_chirho, val_chirho, false);
                    let c65_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 65);
                    let c90_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 90);
                    let c32_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 32);
                    let is_upper_chirho = builder_chirho.ins().icmp(IntCcChirho::SignedGreaterThanOrEqual, v_chirho, c65_chirho);
                    let is_upper_end_chirho = builder_chirho.ins().icmp(IntCcChirho::SignedLessThanOrEqual, v_chirho, c90_chirho);
                    let in_range_chirho = builder_chirho.ins().band(is_upper_chirho, is_upper_end_chirho);
                    let lower_chirho = builder_chirho.ins().iadd(v_chirho, c32_chirho);
                    return builder_chirho.ins().select(in_range_chirho, lower_chirho, v_chirho);
                }
            }
            // isDigit :: Char -> Bool
            if matches!(name_chirho.as_str(), "isDigit" | "isDigit#") {
                if let Some(arg_expr_chirho) = all_args_chirho.last() {
                    let val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, arg_expr_chirho);
                    let v_chirho = ensure_i64_chirho(builder_chirho, val_chirho, false);
                    let c48_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 48);
                    let c57_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 57);
                    let ge_chirho = builder_chirho.ins().icmp(IntCcChirho::SignedGreaterThanOrEqual, v_chirho, c48_chirho);
                    let le_chirho = builder_chirho.ins().icmp(IntCcChirho::SignedLessThanOrEqual, v_chirho, c57_chirho);
                    let result_chirho = builder_chirho.ins().band(ge_chirho, le_chirho);
                    return builder_chirho.ins().uextend(cl_types_chirho::I64, result_chirho);
                }
            }
            // isAlpha :: Char -> Bool
            if matches!(name_chirho.as_str(), "isAlpha" | "isAlpha#") {
                if let Some(arg_expr_chirho) = all_args_chirho.last() {
                    let val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, arg_expr_chirho);
                    let v_chirho = ensure_i64_chirho(builder_chirho, val_chirho, false);
                    let c65_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 65);
                    let c90_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 90);
                    let c97_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 97);
                    let c122_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 122);
                    let upper_chirho = builder_chirho.ins().icmp(IntCcChirho::SignedGreaterThanOrEqual, v_chirho, c65_chirho);
                    let upper_end_chirho = builder_chirho.ins().icmp(IntCcChirho::SignedLessThanOrEqual, v_chirho, c90_chirho);
                    let is_upper_chirho = builder_chirho.ins().band(upper_chirho, upper_end_chirho);
                    let lower_chirho = builder_chirho.ins().icmp(IntCcChirho::SignedGreaterThanOrEqual, v_chirho, c97_chirho);
                    let lower_end_chirho = builder_chirho.ins().icmp(IntCcChirho::SignedLessThanOrEqual, v_chirho, c122_chirho);
                    let is_lower_chirho = builder_chirho.ins().band(lower_chirho, lower_end_chirho);
                    let result_chirho = builder_chirho.ins().bor(is_upper_chirho, is_lower_chirho);
                    return builder_chirho.ins().uextend(cl_types_chirho::I64, result_chirho);
                }
            }
            // unpack :: String -> [Char] — convert C string to cons-list
            if matches!(name_chirho.as_str(), "unpack" | "unpack#") {
                if let Some(unpack_ref_chirho) = ctx_chirho.unpack_string_ref_chirho {
                    if let Some(arg_expr_chirho) = all_args_chirho.last() {
                        let arg_val_chirho =
                            lower_expr_chirho(builder_chirho, ctx_chirho, arg_expr_chirho);
                        let arg_i64_chirho =
                            ensure_i64_chirho(builder_chirho, arg_val_chirho, false);
                        let call_chirho = builder_chirho
                            .ins()
                            .call(unpack_ref_chirho, &[arg_i64_chirho]);
                        return builder_chirho.inst_results(call_chirho)[0];
                    }
                }
            }
            // ord :: Char -> Int, chr :: Int -> Char — identity at runtime
            if matches!(name_chirho.as_str(), "ord" | "chr" | "ord#" | "chr#") {
                if let Some(arg_expr_chirho) = all_args_chirho.last() {
                    return lower_expr_chirho(builder_chirho, ctx_chirho, arg_expr_chirho);
                }
            }
            // readFile :: FilePath -> IO String
            if matches!(name_chirho.as_str(), "readFile" | "readFile#") {
                if let Some(read_file_ref_chirho) = ctx_chirho.read_file_ref_chirho {
                    if let Some(path_expr_chirho) = all_args_chirho.last() {
                        let path_val_chirho =
                            lower_expr_chirho(builder_chirho, ctx_chirho, path_expr_chirho);
                        let path_i64_chirho =
                            ensure_i64_chirho(builder_chirho, path_val_chirho, false);
                        let call_chirho = builder_chirho
                            .ins()
                            .call(read_file_ref_chirho, &[path_i64_chirho]);
                        return builder_chirho.inst_results(call_chirho)[0];
                    }
                }
            }
            if matches!(name_chirho.as_str(), "putStrLn" | "putStrLn#") {
                if let Some(put_str_ln_ref_chirho) = ctx_chirho.put_str_ln_ref_chirho {
                    if let Some(arg_expr_chirho) = all_args_chirho.last() {
                        let arg_val_chirho =
                            lower_expr_chirho(builder_chirho, ctx_chirho, arg_expr_chirho);
                        let arg_i64_chirho =
                            ensure_i64_chirho(builder_chirho, arg_val_chirho, false);
                        builder_chirho
                            .ins()
                            .call(put_str_ln_ref_chirho, &[arg_i64_chirho]);
                        return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
                    }
                }
            }
            // print :: Show a => a -> IO () — use RTS haskelujah_print_int
            if matches!(name_chirho.as_str(), "print" | "print#") {
                if let Some(print_int_ref_chirho) = ctx_chirho.print_int_ref_chirho {
                    if let Some(arg_expr_chirho) = all_args_chirho.last() {
                        let arg_val_chirho =
                            lower_expr_chirho(builder_chirho, ctx_chirho, arg_expr_chirho);
                        let arg_i64_chirho =
                            ensure_i64_chirho(builder_chirho, arg_val_chirho, false);
                        builder_chirho
                            .ins()
                            .call(print_int_ref_chirho, &[arg_i64_chirho]);
                        return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
                    }
                }
            }
            // show :: Show a => a -> String — use RTS haskelujah_show_int
            if matches!(name_chirho.as_str(), "show" | "show#") {
                if let Some(show_int_ref_chirho) = ctx_chirho.show_int_ref_chirho {
                    if let Some(arg_expr_chirho) = all_args_chirho.last() {
                        let arg_val_chirho =
                            lower_expr_chirho(builder_chirho, ctx_chirho, arg_expr_chirho);
                        let arg_i64_chirho =
                            ensure_i64_chirho(builder_chirho, arg_val_chirho, false);
                        let call_chirho = builder_chirho
                            .ins()
                            .call(show_int_ref_chirho, &[arg_i64_chirho]);
                        return builder_chirho.inst_results(call_chirho)[0];
                    }
                }
            }
        }

        if let Some((func_ref_chirho, _arity_chirho)) =
            ctx_chirho.func_ref_map_chirho.get(func_id_chirho)
        {
            if *_arity_chirho < all_args_chirho.len() {
                let mut direct_arg_vals_chirho = Vec::with_capacity(*_arity_chirho);
                for arg_chirho in all_args_chirho.iter().take(*_arity_chirho) {
                    let val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, arg_chirho);
                    direct_arg_vals_chirho.push(ensure_i64_chirho(
                        builder_chirho,
                        val_chirho,
                        false,
                    ));
                }
                append_lifted_capture_arg_vals_chirho(
                    builder_chirho,
                    ctx_chirho,
                    *func_id_chirho,
                    &mut direct_arg_vals_chirho,
                );
                let direct_call_inst_chirho = builder_chirho
                    .ins()
                    .call(*func_ref_chirho, &direct_arg_vals_chirho);
                let direct_result_chirho = builder_chirho.inst_results(direct_call_inst_chirho)[0];
                return lower_indirect_app_chirho(
                    builder_chirho,
                    ctx_chirho,
                    direct_result_chirho,
                    &all_args_chirho[*_arity_chirho..],
                );
            }
            if *_arity_chirho != all_args_chirho.len() {
                if let Some(wrapper_ref_chirho) = ctx_chirho
                    .pap_wrapper_ref_map_chirho
                    .get(&(*func_id_chirho, all_args_chirho.len()))
                {
                    let mut stored_arg_vals_chirho = Vec::with_capacity(all_args_chirho.len());
                    for arg_chirho in &all_args_chirho {
                        let arg_val_chirho =
                            lower_expr_chirho(builder_chirho, ctx_chirho, arg_chirho);
                        stored_arg_vals_chirho.push(ensure_i64_chirho(
                            builder_chirho,
                            arg_val_chirho,
                            false,
                        ));
                    }
                    append_lifted_capture_arg_vals_chirho(
                        builder_chirho,
                        ctx_chirho,
                        *func_id_chirho,
                        &mut stored_arg_vals_chirho,
                    );
                    return emit_partial_application_closure_chirho(
                        builder_chirho,
                        ctx_chirho,
                        *wrapper_ref_chirho,
                        &stored_arg_vals_chirho,
                    );
                }
                let fun_val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, callee_chirho);
                return lower_indirect_app_chirho(
                    builder_chirho,
                    ctx_chirho,
                    fun_val_chirho,
                    &all_args_chirho,
                );
            }
            // Lower all arguments.
            let mut arg_vals_chirho = Vec::new();
            for a_chirho in &all_args_chirho {
                let val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, a_chirho);
                arg_vals_chirho.push(ensure_i64_chirho(builder_chirho, val_chirho, false));
            }
            append_lifted_capture_arg_vals_chirho(
                builder_chirho,
                ctx_chirho,
                *func_id_chirho,
                &mut arg_vals_chirho,
            );

            let call_inst_chirho = builder_chirho
                .ins()
                .call(*func_ref_chirho, &arg_vals_chirho);
            return builder_chirho.inst_results(call_inst_chirho)[0];
        }
    }

    let fun_ptr_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, fun_chirho);
    lower_indirect_app_chirho(builder_chirho, ctx_chirho, fun_ptr_chirho, &[arg_chirho])
}

fn lower_self_tail_call_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    fun_chirho: &CoreExprChirho,
    arg_chirho: &CoreExprChirho,
) -> bool {
    let (Some(tco_self_id_chirho), Some(tco_loop_block_chirho)) = (
        ctx_chirho.tco_self_id_chirho,
        ctx_chirho.tco_loop_block_chirho,
    ) else {
        return false;
    };

    let full_expr_chirho = CoreExprChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho.clone()),
        arg_chirho: Box::new(arg_chirho.clone()),
    };
    let (callee_chirho, all_args_chirho) = flatten_apps_chirho(&full_expr_chirho);
    let callee_chirho = strip_runtime_wrappers_chirho(callee_chirho);
    let CoreExprChirho::VarChirho(callee_id_chirho) = callee_chirho else {
        return false;
    };
    if *callee_id_chirho != tco_self_id_chirho {
        return false;
    }

    let Some((_, arity_chirho)) = ctx_chirho.func_ref_map_chirho.get(&tco_self_id_chirho) else {
        return false;
    };
    if *arity_chirho != all_args_chirho.len() {
        return false;
    }

    let mut jump_arg_vals_chirho = Vec::with_capacity(*arity_chirho);
    for arg_expr_chirho in &all_args_chirho {
        let arg_val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, arg_expr_chirho);
        jump_arg_vals_chirho.push(ensure_i64_chirho(builder_chirho, arg_val_chirho, false));
    }
    append_lifted_capture_arg_vals_chirho(
        builder_chirho,
        ctx_chirho,
        tco_self_id_chirho,
        &mut jump_arg_vals_chirho,
    );
    builder_chirho
        .ins()
        .jump(tco_loop_block_chirho, &jump_arg_vals_chirho);
    true
}

fn strip_runtime_wrappers_chirho<'a>(expr_chirho: &'a CoreExprChirho) -> &'a CoreExprChirho {
    match expr_chirho {
        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            strip_runtime_wrappers_chirho(body_chirho)
        }
        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ..
        } => strip_runtime_wrappers_chirho(inner_chirho),
        _ => expr_chirho,
    }
}

fn append_lifted_capture_arg_vals_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    func_id_chirho: CoreIdChirho,
    arg_vals_chirho: &mut Vec<ClValueChirho>,
) {
    arg_vals_chirho.extend(collect_lifted_capture_arg_vals_chirho(
        builder_chirho,
        ctx_chirho,
        func_id_chirho,
    ));
}

fn collect_lifted_capture_arg_vals_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    func_id_chirho: CoreIdChirho,
) -> Vec<ClValueChirho> {
    let Some(capture_ids_chirho) = ctx_chirho.lifted_capture_ids_chirho.get(&func_id_chirho) else {
        return Vec::new();
    };

    capture_ids_chirho
        .iter()
        .map(|capture_id_chirho| {
            let capture_val_chirho = lower_expr_chirho(
                builder_chirho,
                ctx_chirho,
                &CoreExprChirho::VarChirho(*capture_id_chirho),
            );
            ensure_i64_chirho(builder_chirho, capture_val_chirho, false)
        })
        .collect()
}

pub fn emit_partial_application_closure_with_alloc_ref_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    alloc_ref_chirho: cranelift_codegen::ir::FuncRef,
    wrapper_ref_chirho: cranelift_codegen::ir::FuncRef,
    stored_arg_vals_chirho: &[ClValueChirho],
) -> ClValueChirho {
    let alloc_size_chirho = ((stored_arg_vals_chirho.len() + 1) * 8) as i64;
    let alloc_size_val_chirho = builder_chirho
        .ins()
        .iconst(cl_types_chirho::I64, alloc_size_chirho);
    let alloc_call_chirho = builder_chirho
        .ins()
        .call(alloc_ref_chirho, &[alloc_size_val_chirho]);
    let alloc_ptr_chirho = builder_chirho.inst_results(alloc_call_chirho)[0];
    let mem_flags_chirho = cranelift_codegen::ir::MemFlags::new();

    let wrapper_bits_chirho = builder_chirho
        .ins()
        .func_addr(cl_types_chirho::I64, wrapper_ref_chirho);
    builder_chirho
        .ins()
        .store(mem_flags_chirho, wrapper_bits_chirho, alloc_ptr_chirho, 0);

    for (arg_idx_chirho, stored_arg_val_chirho) in stored_arg_vals_chirho.iter().enumerate() {
        let offset_chirho = ((arg_idx_chirho + 1) * 8) as i32;
        builder_chirho.ins().store(
            mem_flags_chirho,
            *stored_arg_val_chirho,
            alloc_ptr_chirho,
            offset_chirho,
        );
    }

    let boxed_mask_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, i64::MIN);
    builder_chirho
        .ins()
        .bor(alloc_ptr_chirho, boxed_mask_chirho)
}

fn emit_partial_application_closure_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    wrapper_ref_chirho: cranelift_codegen::ir::FuncRef,
    stored_arg_vals_chirho: &[ClValueChirho],
) -> ClValueChirho {
    let Some(alloc_ref_chirho) = ctx_chirho.alloc_ref_chirho else {
        return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
    };
    emit_partial_application_closure_with_alloc_ref_chirho(
        builder_chirho,
        alloc_ref_chirho,
        wrapper_ref_chirho,
        stored_arg_vals_chirho,
    )
}

fn lower_known_function_value_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    func_id_chirho: CoreIdChirho,
    func_ref_chirho: cranelift_codegen::ir::FuncRef,
    arity_chirho: usize,
) -> ClValueChirho {
    if arity_chirho == 0 {
        let call_inst_chirho = builder_chirho.ins().call(func_ref_chirho, &[]);
        return builder_chirho.inst_results(call_inst_chirho)[0];
    }

    let hidden_capture_arg_vals_chirho =
        collect_lifted_capture_arg_vals_chirho(builder_chirho, ctx_chirho, func_id_chirho);
    if let Some(wrapper_ref_chirho) = ctx_chirho
        .pap_wrapper_ref_map_chirho
        .get(&(func_id_chirho, 0))
    {
        return emit_partial_application_closure_chirho(
            builder_chirho,
            ctx_chirho,
            *wrapper_ref_chirho,
            &hidden_capture_arg_vals_chirho,
        );
    }

    builder_chirho
        .ins()
        .func_addr(cl_types_chirho::I64, func_ref_chirho)
}

fn lower_indirect_call_with_sig_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    fun_ptr_i64_chirho: ClValueChirho,
    arg_vals_chirho: &[ClValueChirho],
) -> ClValueChirho {
    let mut sig_chirho = builder_chirho.func.stencil.signature.clone();
    sig_chirho.params.clear();
    sig_chirho.returns.clear();
    for _ in arg_vals_chirho {
        sig_chirho
            .params
            .push(cranelift_codegen::ir::AbiParam::new(cl_types_chirho::I64));
    }
    sig_chirho
        .returns
        .push(cranelift_codegen::ir::AbiParam::new(cl_types_chirho::I64));

    let sig_ref_chirho = builder_chirho.import_signature(sig_chirho);
    let call_inst_chirho =
        builder_chirho
            .ins()
            .call_indirect(sig_ref_chirho, fun_ptr_i64_chirho, arg_vals_chirho);
    builder_chirho.inst_results(call_inst_chirho)[0]
}

fn lower_indirect_app_values_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    fun_ptr_i64_chirho: ClValueChirho,
    arg_vals_chirho: &[ClValueChirho],
) -> ClValueChirho {
    if arg_vals_chirho.is_empty() {
        return fun_ptr_i64_chirho;
    }

    let boxed_block_chirho = builder_chirho.create_block();
    let direct_block_chirho = builder_chirho.create_block();
    let join_block_chirho = builder_chirho.create_block();
    builder_chirho.append_block_param(join_block_chirho, cl_types_chirho::I64);

    let zero_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
    let is_boxed_chirho =
        builder_chirho
            .ins()
            .icmp(IntCcChirho::SignedLessThan, fun_ptr_i64_chirho, zero_chirho);
    builder_chirho.ins().brif(
        is_boxed_chirho,
        boxed_block_chirho,
        &[],
        direct_block_chirho,
        &[],
    );

    builder_chirho.switch_to_block(direct_block_chirho);
    let direct_result_chirho =
        lower_indirect_call_with_sig_chirho(builder_chirho, fun_ptr_i64_chirho, arg_vals_chirho);
    builder_chirho
        .ins()
        .jump(join_block_chirho, &[direct_result_chirho]);
    builder_chirho.seal_block(direct_block_chirho);

    builder_chirho.switch_to_block(boxed_block_chirho);
    let boxed_ptr_chirho = decode_boxed_constructor_ptr_chirho(builder_chirho, fun_ptr_i64_chirho);
    let closure_fun_bits_chirho = builder_chirho.ins().load(
        cl_types_chirho::I64,
        cranelift_codegen::ir::MemFlags::new(),
        boxed_ptr_chirho,
        0,
    );
    let first_arg_chirho = arg_vals_chirho[0];
    let closure_result_chirho = lower_indirect_call_with_sig_chirho(
        builder_chirho,
        closure_fun_bits_chirho,
        &[fun_ptr_i64_chirho, first_arg_chirho],
    );
    let boxed_result_chirho = if arg_vals_chirho.len() == 1 {
        closure_result_chirho
    } else {
        lower_indirect_app_values_chirho(
            builder_chirho,
            closure_result_chirho,
            &arg_vals_chirho[1..],
        )
    };
    builder_chirho
        .ins()
        .jump(join_block_chirho, &[boxed_result_chirho]);
    builder_chirho.seal_block(boxed_block_chirho);

    builder_chirho.switch_to_block(join_block_chirho);
    builder_chirho.seal_block(join_block_chirho);
    builder_chirho.block_params(join_block_chirho)[0]
}

fn lower_indirect_app_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    fun_ptr_chirho: ClValueChirho,
    args_chirho: &[&CoreExprChirho],
) -> ClValueChirho {
    let fun_ptr_i64_chirho = ensure_i64_chirho(builder_chirho, fun_ptr_chirho, false);
    let mut arg_vals_chirho = Vec::with_capacity(args_chirho.len());
    for arg_chirho in args_chirho {
        let arg_val_chirho = lower_expr_chirho(builder_chirho, ctx_chirho, arg_chirho);
        arg_vals_chirho.push(ensure_i64_chirho(builder_chirho, arg_val_chirho, false));
    }
    lower_indirect_app_values_chirho(builder_chirho, fun_ptr_i64_chirho, &arg_vals_chirho)
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
        "div#" | "quot#" | "divInt#" | "quotInt#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            let rhs_chirho = ensure_i64_chirho(builder_chirho, rhs_raw_chirho, false);
            builder_chirho.ins().sdiv(lhs_chirho, rhs_chirho)
        }
        "mod#" | "rem#" | "modInt#" | "remInt#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            let rhs_chirho = ensure_i64_chirho(builder_chirho, rhs_raw_chirho, false);
            builder_chirho.ins().srem(lhs_chirho, rhs_chirho)
        }
        "negate#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            builder_chirho.ins().ineg(lhs_chirho)
        }
        "absInt#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            let neg_chirho = builder_chirho.ins().ineg(lhs_chirho);
            let zero_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
            let is_neg_chirho =
                builder_chirho
                    .ins()
                    .icmp(IntCcChirho::SignedLessThan, lhs_chirho, zero_chirho);
            builder_chirho
                .ins()
                .select(is_neg_chirho, neg_chirho, lhs_chirho)
        }
        "signumInt#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            let zero_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
            let one_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 1);
            let neg_one_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, -1i64);
            let is_neg_chirho =
                builder_chirho
                    .ins()
                    .icmp(IntCcChirho::SignedLessThan, lhs_chirho, zero_chirho);
            let is_zero_chirho =
                builder_chirho
                    .ins()
                    .icmp(IntCcChirho::Equal, lhs_chirho, zero_chirho);
            let neg_or_pos_chirho =
                builder_chirho
                    .ins()
                    .select(is_neg_chirho, neg_one_chirho, one_chirho);
            builder_chirho
                .ins()
                .select(is_zero_chirho, zero_chirho, neg_or_pos_chirho)
        }
        "minInt#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            let rhs_chirho = ensure_i64_chirho(builder_chirho, rhs_raw_chirho, false);
            let cmp_chirho =
                builder_chirho
                    .ins()
                    .icmp(IntCcChirho::SignedLessThan, lhs_chirho, rhs_chirho);
            builder_chirho
                .ins()
                .select(cmp_chirho, lhs_chirho, rhs_chirho)
        }
        "maxInt#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            let rhs_chirho = ensure_i64_chirho(builder_chirho, rhs_raw_chirho, false);
            let cmp_chirho =
                builder_chirho
                    .ins()
                    .icmp(IntCcChirho::SignedGreaterThan, lhs_chirho, rhs_chirho);
            builder_chirho
                .ins()
                .select(cmp_chirho, lhs_chirho, rhs_chirho)
        }
        "showInt#" => {
            if let Some(arg_kind_chirho) =
                args_chirho.first().and_then(infer_show_int_arg_kind_chirho)
            {
                return match arg_kind_chirho {
                    ShowIntArgKindChirho::BoolChirho => {
                        lower_show_bool_primop_chirho(builder_chirho, ctx_chirho, lhs_raw_chirho)
                    }
                    ShowIntArgKindChirho::StringChirho => {
                        lower_show_str_primop_chirho(builder_chirho, ctx_chirho, lhs_raw_chirho)
                    }
                };
            }
            // Call RTS haskelujah_show_int_chirho(i64) -> ptr
            let val_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            if let Some(show_ref_chirho) = ctx_chirho.show_int_ref_chirho {
                let call_chirho = builder_chirho.ins().call(show_ref_chirho, &[val_chirho]);
                builder_chirho.inst_results(call_chirho)[0]
            } else {
                builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
            }
        }
        "showBool#" => lower_show_bool_primop_chirho(builder_chirho, ctx_chirho, lhs_raw_chirho),
        "showChar#" => {
            let val_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            if let Some(show_char_ref_chirho) = ctx_chirho.show_char_ref_chirho {
                let call_chirho = builder_chirho
                    .ins()
                    .call(show_char_ref_chirho, &[val_chirho]);
                builder_chirho.inst_results(call_chirho)[0]
            } else {
                builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
            }
        }
        "showFloat#" => {
            let val_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, true);
            if let Some(show_float_ref_chirho) = ctx_chirho.show_float_ref_chirho {
                let call_chirho = builder_chirho
                    .ins()
                    .call(show_float_ref_chirho, &[val_chirho]);
                builder_chirho.inst_results(call_chirho)[0]
            } else {
                builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
            }
        }
        "showStr#" => lower_show_str_primop_chirho(builder_chirho, ctx_chirho, lhs_raw_chirho),
        "^#" => {
            // Integer power — simplified: return lhs^rhs via repeated multiply
            // (not emitted here; return placeholder i64 0 for now).
            ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false)
        }

        // ── Integer comparisons → 0 or 1 in i64 ───────────────────────────
        "==#" => icmp_to_i64_chirho(
            builder_chirho,
            IntCcChirho::Equal,
            lhs_raw_chirho,
            rhs_raw_chirho,
        ),
        "/=#" => icmp_to_i64_chirho(
            builder_chirho,
            IntCcChirho::NotEqual,
            lhs_raw_chirho,
            rhs_raw_chirho,
        ),
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
        "++#" => {
            let lhs_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            let rhs_chirho = ensure_i64_chirho(builder_chirho, rhs_raw_chirho, false);
            let Some(append_str_ref_chirho) = ctx_chirho.append_str_ref_chirho else {
                return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
            };
            let append_call_chirho = builder_chirho
                .ins()
                .call(append_str_ref_chirho, &[lhs_chirho, rhs_chirho]);
            builder_chirho.inst_results(append_call_chirho)[0]
        }

        // ── unpack# — convert C string to [Char] cons-list ───────────────
        "unpack#" => {
            let val_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            if let Some(unpack_ref_chirho) = ctx_chirho.unpack_string_ref_chirho {
                let call_chirho = builder_chirho.ins().call(unpack_ref_chirho, &[val_chirho]);
                builder_chirho.inst_results(call_chirho)[0]
            } else {
                builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
            }
        }

        // ── pack# — convert [Char] cons-list to C string ────────────────
        "pack#" => {
            let val_chirho = ensure_i64_chirho(builder_chirho, lhs_raw_chirho, false);
            // Reuse unpack_string_ref since pack has the same signature (i64 → i64)
            // We need a separate FuncRef for pack. For now, use call_indirect.
            let mut sig_chirho = builder_chirho.func.stencil.signature.clone();
            sig_chirho.params.clear();
            sig_chirho.returns.clear();
            sig_chirho
                .params
                .push(cranelift_codegen::ir::AbiParam::new(cl_types_chirho::I64));
            sig_chirho
                .returns
                .push(cranelift_codegen::ir::AbiParam::new(cl_types_chirho::I64));
            // Try to find the pack function by searching for it
            // For now, just return the input as-is (strings and char lists are both i64)
            // TODO: wire haskelujah_pack_string_chirho FuncRef properly
            val_chirho
        }

        // ── Unknown primop ─────────────────────────────────────────────────
        _ => builder_chirho.ins().iconst(cl_types_chirho::I64, 0),
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
enum ShowIntArgKindChirho {
    BoolChirho,
    StringChirho,
}

fn infer_show_int_arg_kind_chirho(expr_chirho: &CoreExprChirho) -> Option<ShowIntArgKindChirho> {
    match expr_chirho {
        CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(_)) => {
            Some(ShowIntArgKindChirho::StringChirho)
        }
        CoreExprChirho::ConAppChirho {
            con_name_chirho, ..
        } if matches!(con_name_chirho.as_str(), "True" | "False") => {
            Some(ShowIntArgKindChirho::BoolChirho)
        }
        CoreExprChirho::PrimOpChirho { name_chirho, .. } => match name_chirho.as_str() {
            "==#" | "/=#" | "<#" | "<=#" | ">#" | ">=#" | "not#" | "eqFloat#" | "<.#" | ">.#"
            | "readBool#" | "showStr#" | "++#" | "eqStr#" => {
                Some(if matches!(name_chirho.as_str(), "showStr#" | "++#") {
                    ShowIntArgKindChirho::StringChirho
                } else {
                    ShowIntArgKindChirho::BoolChirho
                })
            }
            _ => None,
        },
        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            infer_show_int_arg_kind_chirho(body_chirho)
        }
        CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
            infer_show_int_arg_kind_chirho(expr_chirho)
        }
        _ => None,
    }
}

fn lower_show_bool_primop_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    raw_bool_chirho: ClValueChirho,
) -> ClValueChirho {
    let val_chirho = ensure_i64_chirho(builder_chirho, raw_bool_chirho, false);
    if let Some(show_bool_ref_chirho) = ctx_chirho.show_bool_ref_chirho {
        let call_inst_chirho = builder_chirho
            .ins()
            .call(show_bool_ref_chirho, &[val_chirho]);
        return builder_chirho.inst_results(call_inst_chirho)[0];
    }
    let zero_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
    let is_true_chirho = builder_chirho
        .ins()
        .icmp(IntCcChirho::NotEqual, val_chirho, zero_chirho);
    let Some(true_global_chirho) = ctx_chirho.string_globals_chirho.get("True").copied() else {
        return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
    };
    let Some(false_global_chirho) = ctx_chirho.string_globals_chirho.get("False").copied() else {
        return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
    };
    let true_ptr_chirho = builder_chirho
        .ins()
        .global_value(cl_types_chirho::I64, true_global_chirho);
    let false_ptr_chirho = builder_chirho
        .ins()
        .global_value(cl_types_chirho::I64, false_global_chirho);
    builder_chirho
        .ins()
        .select(is_true_chirho, true_ptr_chirho, false_ptr_chirho)
}

fn lower_show_str_primop_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    ctx_chirho: &mut LowerCtxChirho<'_>,
    raw_str_chirho: ClValueChirho,
) -> ClValueChirho {
    let val_chirho = ensure_i64_chirho(builder_chirho, raw_str_chirho, false);
    let Some(append_str_ref_chirho) = ctx_chirho.append_str_ref_chirho else {
        return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
    };
    let Some(quote_global_chirho) = ctx_chirho.string_globals_chirho.get("\"").copied() else {
        return builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
    };
    let quote_ptr_chirho = builder_chirho
        .ins()
        .global_value(cl_types_chirho::I64, quote_global_chirho);
    let quoted_prefix_call_chirho = builder_chirho
        .ins()
        .call(append_str_ref_chirho, &[quote_ptr_chirho, val_chirho]);
    let quoted_prefix_ptr_chirho = builder_chirho.inst_results(quoted_prefix_call_chirho)[0];
    let quoted_full_call_chirho = builder_chirho.ins().call(
        append_str_ref_chirho,
        &[quoted_prefix_ptr_chirho, quote_ptr_chirho],
    );
    builder_chirho.inst_results(quoted_full_call_chirho)[0]
}

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
    let bool_val_chirho = builder_chirho
        .ins()
        .fcmp(cc_chirho, lhs_f64_chirho, rhs_f64_chirho);
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
        builder_chirho
            .ins()
            .uextend(cl_types_chirho::I64, val_chirho)
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
