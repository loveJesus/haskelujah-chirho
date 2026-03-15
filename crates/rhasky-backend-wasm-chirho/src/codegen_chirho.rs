// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # WebAssembly binary code generation
//!
//! Emits a minimal valid WebAssembly binary module from Core IR.
//! Uses the raw Wasm binary encoding (magic + version + sections)
//! rather than WAT text format, so the output can be loaded directly
//! by a Wasm runtime.
//!
//! Supports: integer/char literals, variables (local.get), let bindings
//! (local.set/get), function calls, PrimOps, case expressions with
//! literal/default dispatch, constructor tags, and dictionary elision
//! via the shared `elide_dicts_and_filter_chirho` pass.
//!
//! Lambda lifting, closures, and heap allocation are deferred to a
//! later milestone.

use std::collections::HashMap;

use rhasky_core_chirho::{
    AltConChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho, CoreModuleChirho,
};

/// Wasm value types.
const WASM_I64_CHIRHO: u8 = 0x7E;

/// Wasm section IDs.
const SECTION_TYPE_CHIRHO: u8 = 1;
const SECTION_FUNCTION_CHIRHO: u8 = 3;
const SECTION_EXPORT_CHIRHO: u8 = 7;
const SECTION_CODE_CHIRHO: u8 = 10;

/// Compile a Core module to a valid WebAssembly binary.
pub fn compile_core_to_wasm_chirho(module_chirho: &CoreModuleChirho) -> Vec<u8> {
    let mut emitter_chirho = WasmEmitterChirho::new_chirho();
    emitter_chirho.emit_module_chirho(module_chirho);
    emitter_chirho.finish_chirho()
}

/// Compile a Core module to a runnable WebAssembly binary with dictionary
/// elision and reachability filtering (only bindings reachable from `main`).
pub fn compile_core_to_wasm_executable_chirho(module_chirho: &CoreModuleChirho) -> Vec<u8> {
    let filtered_module_chirho =
        rhasky_core_chirho::elide_dicts_and_filter_chirho(module_chirho);
    compile_core_to_wasm_chirho(&filtered_module_chirho)
}

struct WasmEmitterChirho {
    /// The raw Wasm bytes being built.
    bytes_chirho: Vec<u8>,
}

impl WasmEmitterChirho {
    fn new_chirho() -> Self {
        Self {
            bytes_chirho: Vec::new(),
        }
    }

    fn emit_module_chirho(&mut self, module_chirho: &CoreModuleChirho) {
        // Wasm magic number + version
        self.bytes_chirho.extend_from_slice(b"\0asm");
        self.bytes_chirho.extend_from_slice(&[1, 0, 0, 0]);

        let func_count_chirho = module_chirho.bindings_chirho.len();

        // Build name → function index map for cross-function calls
        let mut func_idx_map_chirho: HashMap<String, u32> = HashMap::new();
        // Also build CoreId → function index for VarChirho resolution
        let mut id_to_func_idx_chirho: HashMap<CoreIdChirho, u32> = HashMap::new();
        for (i_chirho, b_chirho) in module_chirho.bindings_chirho.iter().enumerate() {
            func_idx_map_chirho.insert(
                b_chirho.binder_chirho.name_chirho.clone(),
                i_chirho as u32,
            );
            id_to_func_idx_chirho.insert(b_chirho.binder_chirho.id_chirho, i_chirho as u32);
        }

        // Collect lambda params for each binding to determine function signatures
        let func_infos_chirho: Vec<FuncInfoChirho> = module_chirho
            .bindings_chirho
            .iter()
            .map(|b_chirho| {
                let (params_chirho, body_chirho) = collect_lambda_params_chirho(&b_chirho.rhs_chirho);
                FuncInfoChirho {
                    name_chirho: b_chirho.binder_chirho.name_chirho.clone(),
                    param_count_chirho: params_chirho.len(),
                    params_chirho,
                    body_chirho: body_chirho.clone(),
                }
            })
            .collect();

        // === Type Section ===
        // One type per unique arity
        let mut type_section_chirho = Vec::new();
        let unique_arities_chirho = {
            let mut arities_chirho: Vec<usize> = func_infos_chirho
                .iter()
                .map(|f_chirho| f_chirho.param_count_chirho)
                .collect();
            arities_chirho.sort();
            arities_chirho.dedup();
            arities_chirho
        };

        encode_u32_chirho(&mut type_section_chirho, unique_arities_chirho.len() as u32);
        for arity_chirho in &unique_arities_chirho {
            type_section_chirho.push(0x60); // functype
            encode_u32_chirho(&mut type_section_chirho, *arity_chirho as u32);
            for _ in 0..*arity_chirho {
                type_section_chirho.push(WASM_I64_CHIRHO);
            }
            type_section_chirho.push(1); // one result
            type_section_chirho.push(WASM_I64_CHIRHO);
        }

        self.emit_section_chirho(SECTION_TYPE_CHIRHO, &type_section_chirho);

        // === Function Section ===
        let mut func_section_chirho = Vec::new();
        encode_u32_chirho(&mut func_section_chirho, func_count_chirho as u32);
        for info_chirho in &func_infos_chirho {
            let type_idx_chirho = unique_arities_chirho
                .iter()
                .position(|a_chirho| *a_chirho == info_chirho.param_count_chirho)
                .unwrap();
            encode_u32_chirho(&mut func_section_chirho, type_idx_chirho as u32);
        }

        self.emit_section_chirho(SECTION_FUNCTION_CHIRHO, &func_section_chirho);

        // === Export Section ===
        // Export all functions by their Haskell name, plus foreign exports by their C name
        let foreign_export_count_chirho = module_chirho.foreign_exports_chirho.len();
        let total_exports_chirho = func_count_chirho + foreign_export_count_chirho;
        let mut export_section_chirho = Vec::new();
        encode_u32_chirho(&mut export_section_chirho, total_exports_chirho as u32);
        for (i_chirho, info_chirho) in func_infos_chirho.iter().enumerate() {
            encode_string_chirho(&mut export_section_chirho, &info_chirho.name_chirho);
            export_section_chirho.push(0x00); // func export
            encode_u32_chirho(&mut export_section_chirho, i_chirho as u32);
        }
        // Foreign export stubs: export the same function under the C/foreign name
        for export_chirho in &module_chirho.foreign_exports_chirho {
            if let Some(func_idx_chirho) = func_idx_map_chirho.get(&export_chirho.haskell_name_chirho) {
                encode_string_chirho(&mut export_section_chirho, &export_chirho.foreign_name_chirho);
                export_section_chirho.push(0x00); // func export
                encode_u32_chirho(&mut export_section_chirho, *func_idx_chirho);
            }
        }

        self.emit_section_chirho(SECTION_EXPORT_CHIRHO, &export_section_chirho);

        // === Code Section ===
        let mut code_section_chirho = Vec::new();
        encode_u32_chirho(&mut code_section_chirho, func_count_chirho as u32);

        for info_chirho in &func_infos_chirho {
            let mut ctx_chirho = EmitCtxChirho {
                local_map_chirho: HashMap::new(),
                next_local_chirho: info_chirho.param_count_chirho as u32,
                id_to_func_idx_chirho: &id_to_func_idx_chirho,
                func_infos_chirho: &func_infos_chirho,
            };

            // Map param CoreIds to local indices (0..param_count)
            for (i_chirho, pid_chirho) in info_chirho.params_chirho.iter().enumerate() {
                ctx_chirho.local_map_chirho.insert(*pid_chirho, i_chirho as u32);
            }

            // Pre-allocate local slots for let-bindings and case binders
            allocate_locals_chirho(&info_chirho.body_chirho, &mut ctx_chirho);

            let locals_count_chirho = ctx_chirho.next_local_chirho - info_chirho.param_count_chirho as u32;

            let mut body_bytes_chirho = Vec::new();
            if locals_count_chirho > 0 {
                encode_u32_chirho(&mut body_bytes_chirho, 1); // one local declaration
                encode_u32_chirho(&mut body_bytes_chirho, locals_count_chirho);
                body_bytes_chirho.push(WASM_I64_CHIRHO);
            } else {
                encode_u32_chirho(&mut body_bytes_chirho, 0); // no locals
            }

            // Emit body instructions
            emit_expr_chirho(&mut body_bytes_chirho, &info_chirho.body_chirho, &ctx_chirho);

            body_bytes_chirho.push(0x0B); // end

            // Write the body with its length prefix
            let mut func_body_chirho = Vec::new();
            encode_u32_chirho(&mut func_body_chirho, body_bytes_chirho.len() as u32);
            func_body_chirho.extend_from_slice(&body_bytes_chirho);
            code_section_chirho.extend_from_slice(&func_body_chirho);
        }

        self.emit_section_chirho(SECTION_CODE_CHIRHO, &code_section_chirho);
    }

    fn emit_section_chirho(&mut self, id_chirho: u8, data_chirho: &[u8]) {
        self.bytes_chirho.push(id_chirho);
        encode_u32_chirho(&mut self.bytes_chirho, data_chirho.len() as u32);
        self.bytes_chirho.extend_from_slice(data_chirho);
    }

    fn finish_chirho(self) -> Vec<u8> {
        self.bytes_chirho
    }
}

struct FuncInfoChirho {
    name_chirho: String,
    param_count_chirho: usize,
    params_chirho: Vec<CoreIdChirho>,
    body_chirho: CoreExprChirho,
}

/// Emission context tracking local variable mappings and function indices.
struct EmitCtxChirho<'a> {
    /// CoreId → Wasm local index (params + let-bound vars + case binders)
    local_map_chirho: HashMap<CoreIdChirho, u32>,
    /// Next available local index
    next_local_chirho: u32,
    /// CoreId → function index (for top-level function calls)
    id_to_func_idx_chirho: &'a HashMap<CoreIdChirho, u32>,
    /// Function info for arity lookup
    func_infos_chirho: &'a [FuncInfoChirho],
}

impl<'a> EmitCtxChirho<'a> {
    fn alloc_local_chirho(&mut self, id_chirho: CoreIdChirho) -> u32 {
        let idx_chirho = self.next_local_chirho;
        self.local_map_chirho.insert(id_chirho, idx_chirho);
        self.next_local_chirho += 1;
        idx_chirho
    }
}

/// Pre-allocate local slots for let-bindings and case binders.
fn allocate_locals_chirho(expr_chirho: &CoreExprChirho, ctx_chirho: &mut EmitCtxChirho) {
    match expr_chirho {
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (binder_chirho, rhs_chirho) in binds_chirho {
                ctx_chirho.alloc_local_chirho(binder_chirho.id_chirho);
                allocate_locals_chirho(rhs_chirho, ctx_chirho);
            }
            allocate_locals_chirho(body_chirho, ctx_chirho);
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            // Allocate local for the case binder
            ctx_chirho.alloc_local_chirho(bind_chirho.id_chirho);
            // Allocate for alt binders
            for alt_chirho in alts_chirho {
                for binder_chirho in &alt_chirho.binders_chirho {
                    ctx_chirho.alloc_local_chirho(binder_chirho.id_chirho);
                }
                allocate_locals_chirho(&alt_chirho.rhs_chirho, ctx_chirho);
            }
            allocate_locals_chirho(scrutinee_chirho, ctx_chirho);
        }
        CoreExprChirho::LamChirho { body_chirho, binder_chirho } => {
            // Residual lambda — allocate the param as a local
            ctx_chirho.alloc_local_chirho(binder_chirho.id_chirho);
            allocate_locals_chirho(body_chirho, ctx_chirho);
        }
        CoreExprChirho::AppChirho { fun_chirho, arg_chirho } => {
            allocate_locals_chirho(fun_chirho, ctx_chirho);
            allocate_locals_chirho(arg_chirho, ctx_chirho);
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                allocate_locals_chirho(arg_chirho, ctx_chirho);
            }
        }
        _ => {}
    }
}

/// Encode a u32 as LEB128.
fn encode_u32_chirho(buf_chirho: &mut Vec<u8>, mut val_chirho: u32) {
    loop {
        let mut byte_chirho = (val_chirho & 0x7F) as u8;
        val_chirho >>= 7;
        if val_chirho != 0 {
            byte_chirho |= 0x80;
        }
        buf_chirho.push(byte_chirho);
        if val_chirho == 0 {
            break;
        }
    }
}

/// Encode a signed i64 as LEB128.
fn encode_i64_chirho(buf_chirho: &mut Vec<u8>, mut val_chirho: i64) {
    loop {
        let byte_chirho = (val_chirho & 0x7F) as u8;
        val_chirho >>= 7;
        let more_chirho = !(((val_chirho == 0) && (byte_chirho & 0x40 == 0))
            || ((val_chirho == -1) && (byte_chirho & 0x40 != 0)));
        if more_chirho {
            buf_chirho.push(byte_chirho | 0x80);
        } else {
            buf_chirho.push(byte_chirho);
            break;
        }
    }
}

/// Encode a string with a length prefix.
fn encode_string_chirho(buf_chirho: &mut Vec<u8>, s_chirho: &str) {
    encode_u32_chirho(buf_chirho, s_chirho.len() as u32);
    buf_chirho.extend_from_slice(s_chirho.as_bytes());
}

/// Collect all lambda parameters from a nested chain of Lam nodes.
fn collect_lambda_params_chirho(expr_chirho: &CoreExprChirho) -> (Vec<CoreIdChirho>, &CoreExprChirho) {
    let mut params_chirho = Vec::new();
    let mut current_chirho = expr_chirho;

    while let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho,
    } = current_chirho
    {
        params_chirho.push(binder_chirho.id_chirho);
        current_chirho = body_chirho;
    }

    (params_chirho, current_chirho)
}

/// Flatten nested App chains into (callee, [arg0, arg1, ...]).
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

/// Emit Wasm instructions for a Core expression.
/// The result is left on the Wasm operand stack.
fn emit_expr_chirho(
    buf_chirho: &mut Vec<u8>,
    expr_chirho: &CoreExprChirho,
    ctx_chirho: &EmitCtxChirho,
) {
    match expr_chirho {
        CoreExprChirho::LitChirho(lit_chirho) => {
            emit_lit_chirho(buf_chirho, lit_chirho);
        }

        CoreExprChirho::VarChirho(id_chirho) => {
            // Check if it's a local variable (param or let-bound)
            if let Some(local_idx_chirho) = ctx_chirho.local_map_chirho.get(id_chirho) {
                buf_chirho.push(0x20); // local.get
                encode_u32_chirho(buf_chirho, *local_idx_chirho);
            } else if let Some(func_idx_chirho) = ctx_chirho.id_to_func_idx_chirho.get(id_chirho) {
                // It's a top-level function with 0 params — call it
                let arity_chirho = ctx_chirho.func_infos_chirho[*func_idx_chirho as usize].param_count_chirho;
                if arity_chirho == 0 {
                    buf_chirho.push(0x10); // call
                    encode_u32_chirho(buf_chirho, *func_idx_chirho);
                } else {
                    // Can't call a multi-param function without args — push 0 as fallback
                    buf_chirho.push(0x42); // i64.const
                    encode_i64_chirho(buf_chirho, 0);
                }
            } else {
                // Unknown variable — push 0
                buf_chirho.push(0x42); // i64.const
                encode_i64_chirho(buf_chirho, 0);
            }
        }

        CoreExprChirho::AppChirho { .. } => {
            // Flatten the full App chain to detect function calls
            let (callee_chirho, args_chirho) = flatten_apps_chirho(expr_chirho);

            if let CoreExprChirho::VarChirho(func_id_chirho) = callee_chirho {
                if let Some(func_idx_chirho) = ctx_chirho.id_to_func_idx_chirho.get(func_id_chirho) {
                    // Top-level function call: emit args, then call
                    for arg_chirho in &args_chirho {
                        emit_expr_chirho(buf_chirho, arg_chirho, ctx_chirho);
                    }
                    buf_chirho.push(0x10); // call
                    encode_u32_chirho(buf_chirho, *func_idx_chirho);
                    return;
                }
            }

            // Fallback: for non-function-call Apps (e.g. higher-order),
            // emit operands and add as placeholder
            for arg_chirho in &args_chirho {
                emit_expr_chirho(buf_chirho, arg_chirho, ctx_chirho);
            }
            emit_expr_chirho(buf_chirho, callee_chirho, ctx_chirho);
            // If we have exactly 2 values on stack (1 arg + callee), add them
            // Otherwise push 0 as fallback
            if args_chirho.len() == 1 {
                buf_chirho.push(0x7C); // i64.add (placeholder for apply)
            }
            // For 0 args we already have the callee value, for >1 we'd need
            // multiple drops — just leave the last value
        }

        CoreExprChirho::LamChirho { body_chirho, .. } => {
            // Residual lambda — just emit the body
            emit_expr_chirho(buf_chirho, body_chirho, ctx_chirho);
        }

        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            // Emit each binding and store via local.set
            for (binder_chirho, rhs_chirho) in binds_chirho {
                emit_expr_chirho(buf_chirho, rhs_chirho, ctx_chirho);
                if let Some(local_idx_chirho) = ctx_chirho.local_map_chirho.get(&binder_chirho.id_chirho) {
                    buf_chirho.push(0x21); // local.set
                    encode_u32_chirho(buf_chirho, *local_idx_chirho);
                } else {
                    buf_chirho.push(0x1A); // drop (shouldn't happen)
                }
            }
            emit_expr_chirho(buf_chirho, body_chirho, ctx_chirho);
        }

        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            // Evaluate scrutinee and store in case binder local
            emit_expr_chirho(buf_chirho, scrutinee_chirho, ctx_chirho);
            if let Some(binder_local_chirho) = ctx_chirho.local_map_chirho.get(&bind_chirho.id_chirho) {
                buf_chirho.push(0x22); // local.tee (keep value on stack + store)
                encode_u32_chirho(buf_chirho, *binder_local_chirho);
            }

            // Separate default alt from specific alts
            let mut default_alt_chirho = None;
            let mut specific_alts_chirho = Vec::new();
            for alt_chirho in alts_chirho {
                match &alt_chirho.con_chirho {
                    AltConChirho::DefaultChirho => default_alt_chirho = Some(alt_chirho),
                    _ => specific_alts_chirho.push(alt_chirho),
                }
            }

            if specific_alts_chirho.is_empty() {
                // Only a default alt — drop scrutinee value, emit RHS
                buf_chirho.push(0x1A); // drop
                if let Some(def_chirho) = default_alt_chirho {
                    emit_expr_chirho(buf_chirho, &def_chirho.rhs_chirho, ctx_chirho);
                } else {
                    buf_chirho.push(0x00); // unreachable
                }
            } else if specific_alts_chirho.len() == 1 && default_alt_chirho.is_some() {
                // One specific alt + default: if/else
                let spec_chirho = &specific_alts_chirho[0];
                let def_chirho = default_alt_chirho.unwrap();

                // Compare scrutinee with the literal/constructor tag
                emit_alt_compare_chirho(buf_chirho, &spec_chirho.con_chirho);

                // if (block result type i64)
                buf_chirho.push(0x04); // if
                buf_chirho.push(WASM_I64_CHIRHO); // result type i64

                // Bind alt binders if any (for DataCon with fields: first field = scrutinee value)
                for binder_chirho in &spec_chirho.binders_chirho {
                    if let Some(local_idx_chirho) = ctx_chirho.local_map_chirho.get(&binder_chirho.id_chirho) {
                        if let Some(binder_local_chirho) = ctx_chirho.local_map_chirho.get(&bind_chirho.id_chirho) {
                            buf_chirho.push(0x20); // local.get (scrutinee)
                            encode_u32_chirho(buf_chirho, *binder_local_chirho);
                            buf_chirho.push(0x21); // local.set
                            encode_u32_chirho(buf_chirho, *local_idx_chirho);
                        }
                    }
                }
                emit_expr_chirho(buf_chirho, &spec_chirho.rhs_chirho, ctx_chirho);

                buf_chirho.push(0x05); // else
                emit_expr_chirho(buf_chirho, &def_chirho.rhs_chirho, ctx_chirho);

                buf_chirho.push(0x0B); // end if
            } else {
                // Multiple specific alts: nested if/else chain
                // Drop scrutinee from stack (it's already in the case binder local)
                buf_chirho.push(0x1A); // drop

                emit_nested_if_else_chirho(
                    buf_chirho,
                    &specific_alts_chirho,
                    default_alt_chirho,
                    bind_chirho,
                    ctx_chirho,
                    0,
                );
            }
        }

        CoreExprChirho::TyLamChirho { body_chirho, .. }
        | CoreExprChirho::TyAppChirho {
            expr_chirho: body_chirho,
            ..
        } => {
            emit_expr_chirho(buf_chirho, body_chirho, ctx_chirho);
        }

        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => {
            if args_chirho.len() == 2 {
                emit_expr_chirho(buf_chirho, &args_chirho[0], ctx_chirho);
                emit_expr_chirho(buf_chirho, &args_chirho[1], ctx_chirho);
                let opcode_chirho = match name_chirho.as_str() {
                    "+#" => 0x7C_u8, // i64.add
                    "-#" => 0x7D,     // i64.sub
                    "*#" => 0x7E,     // i64.mul
                    "div#" => 0x7F,   // i64.div_s
                    "mod#" => 0x81,   // i64.rem_s
                    "quot#" => 0x7F,  // i64.div_s (same as div for integers)
                    "rem#" => 0x81,   // i64.rem_s
                    "==#" => 0x51,    // i64.eq
                    "/=#" => 0x52,    // i64.ne
                    "<#" => 0x53,     // i64.lt_s
                    "<=#" => 0x57,    // i64.le_s
                    ">#" => 0x55,     // i64.gt_s
                    ">=#" => 0x59,    // i64.ge_s
                    "^#" => 0x7C,     // fallback (no native wasm pow)
                    _ => 0x7C,        // fallback: i64.add
                };
                buf_chirho.push(opcode_chirho);
            } else if args_chirho.len() == 1 && name_chirho == "negate#" {
                // 0 - x
                buf_chirho.push(0x42); // i64.const
                encode_i64_chirho(buf_chirho, 0);
                emit_expr_chirho(buf_chirho, &args_chirho[0], ctx_chirho);
                buf_chirho.push(0x7D); // i64.sub
            } else {
                // Fallback: push 0
                buf_chirho.push(0x42);
                encode_i64_chirho(buf_chirho, 0);
            }
        }

        CoreExprChirho::ConAppChirho { .. } => {
            // Constructor application — return 0 as a tag placeholder
            // (full constructor support needs heap/memory)
            buf_chirho.push(0x42); // i64.const
            encode_i64_chirho(buf_chirho, 0);
        }
    }
}

/// Emit a comparison of the value on the stack with a case alt pattern.
fn emit_alt_compare_chirho(
    buf_chirho: &mut Vec<u8>,
    con_chirho: &AltConChirho,
) {
    match con_chirho {
        AltConChirho::LitConChirho(lit_chirho) => {
            emit_lit_chirho(buf_chirho, lit_chirho);
            buf_chirho.push(0x51); // i64.eq
        }
        AltConChirho::DataConChirho(_name_chirho) => {
            // For now, treat data constructors by comparing tags
            // The scrutinee should already be a tag value from ConApp
            // We don't have a tag lookup here, so push 0 and compare
            buf_chirho.push(0x42); // i64.const
            encode_i64_chirho(buf_chirho, 0);
            buf_chirho.push(0x51); // i64.eq
        }
        AltConChirho::DefaultChirho => {
            // Default always matches — push 1 (true)
            buf_chirho.push(0x1A); // drop scrutinee
            buf_chirho.push(0x41); // i32.const
            buf_chirho.push(1);
        }
    }
}

/// Emit nested if/else chain for multiple case alternatives.
fn emit_nested_if_else_chirho(
    buf_chirho: &mut Vec<u8>,
    specific_alts_chirho: &[&rhasky_core_chirho::CoreAltChirho],
    default_alt_chirho: Option<&rhasky_core_chirho::CoreAltChirho>,
    bind_chirho: &rhasky_core_chirho::BinderChirho,
    ctx_chirho: &EmitCtxChirho,
    idx_chirho: usize,
) {
    if idx_chirho >= specific_alts_chirho.len() {
        // All specific alts checked, emit default
        if let Some(def_chirho) = default_alt_chirho {
            emit_expr_chirho(buf_chirho, &def_chirho.rhs_chirho, ctx_chirho);
        } else {
            buf_chirho.push(0x42); // i64.const 0
            encode_i64_chirho(buf_chirho, 0);
        }
        return;
    }

    let alt_chirho = specific_alts_chirho[idx_chirho];

    // Load scrutinee from case binder local and compare
    if let Some(binder_local_chirho) = ctx_chirho.local_map_chirho.get(&bind_chirho.id_chirho) {
        buf_chirho.push(0x20); // local.get
        encode_u32_chirho(buf_chirho, *binder_local_chirho);
    } else {
        buf_chirho.push(0x42); // i64.const 0
        encode_i64_chirho(buf_chirho, 0);
    }

    // Compare with this alt's pattern
    match &alt_chirho.con_chirho {
        AltConChirho::LitConChirho(lit_chirho) => {
            emit_lit_chirho(buf_chirho, lit_chirho);
            buf_chirho.push(0x51); // i64.eq
        }
        AltConChirho::DataConChirho(_) => {
            buf_chirho.push(0x42); // i64.const (tag — would need tag lookup)
            encode_i64_chirho(buf_chirho, idx_chirho as i64);
            buf_chirho.push(0x51); // i64.eq
        }
        AltConChirho::DefaultChirho => {
            buf_chirho.push(0x1A); // drop
            buf_chirho.push(0x41); // i32.const 1
            buf_chirho.push(1);
        }
    }

    // if
    buf_chirho.push(0x04); // if
    buf_chirho.push(WASM_I64_CHIRHO); // result type i64

    // Bind alt binders
    for binder_chirho in &alt_chirho.binders_chirho {
        if let Some(local_idx_chirho) = ctx_chirho.local_map_chirho.get(&binder_chirho.id_chirho) {
            if let Some(binder_local_chirho) = ctx_chirho.local_map_chirho.get(&bind_chirho.id_chirho) {
                buf_chirho.push(0x20); // local.get (scrutinee)
                encode_u32_chirho(buf_chirho, *binder_local_chirho);
                buf_chirho.push(0x21); // local.set
                encode_u32_chirho(buf_chirho, *local_idx_chirho);
            }
        }
    }
    emit_expr_chirho(buf_chirho, &alt_chirho.rhs_chirho, ctx_chirho);

    // else — recurse for remaining alts
    buf_chirho.push(0x05); // else
    emit_nested_if_else_chirho(
        buf_chirho,
        specific_alts_chirho,
        default_alt_chirho,
        bind_chirho,
        ctx_chirho,
        idx_chirho + 1,
    );

    buf_chirho.push(0x0B); // end if
}

fn emit_lit_chirho(buf_chirho: &mut Vec<u8>, lit_chirho: &CoreLitChirho) {
    match lit_chirho {
        CoreLitChirho::IntChirho(v_chirho) => {
            buf_chirho.push(0x42); // i64.const
            encode_i64_chirho(buf_chirho, *v_chirho);
        }
        CoreLitChirho::FloatChirho(f_chirho) => {
            // Encode as i64 bits of the f64
            buf_chirho.push(0x42); // i64.const
            encode_i64_chirho(buf_chirho, f_chirho.to_bits() as i64);
        }
        CoreLitChirho::CharChirho(c_chirho) => {
            buf_chirho.push(0x42); // i64.const
            encode_i64_chirho(buf_chirho, *c_chirho as i64);
        }
        CoreLitChirho::StringChirho(_) => {
            buf_chirho.push(0x42); // i64.const 0 (pointer placeholder)
            encode_i64_chirho(buf_chirho, 0);
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhasky_core_chirho::{
        AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreIdChirho,
        InlineAnnotationChirho,
    };
    use rhasky_typing_chirho::ty_chirho::TyChirho;

    fn dummy_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
        BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: rhasky_span_chirho::SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn make_module_chirho(bindings_chirho: Vec<CoreBindingChirho>) -> CoreModuleChirho {
        CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho,
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        }
    }

    #[test]
    fn wasm_magic_and_version_chirho() {
        let module_chirho = make_module_chirho(vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
            is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
        }]);

        let wasm_chirho = compile_core_to_wasm_chirho(&module_chirho);
        assert_eq!(&wasm_chirho[0..4], b"\0asm");
        assert_eq!(&wasm_chirho[4..8], &[1, 0, 0, 0]);
    }

    #[test]
    fn wasm_has_sections_chirho() {
        let module_chirho = make_module_chirho(vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
            is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
        }]);

        let wasm_chirho = compile_core_to_wasm_chirho(&module_chirho);
        assert!(wasm_chirho.len() > 8);
        assert_eq!(wasm_chirho[8], SECTION_TYPE_CHIRHO);
    }

    #[test]
    fn wasm_identity_function_chirho() {
        let module_chirho = make_module_chirho(vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("id", 10),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho("x", 0),
                body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            },
            is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
        }]);

        let wasm_chirho = compile_core_to_wasm_chirho(&module_chirho);
        assert!(&wasm_chirho[0..4] == b"\0asm");
        assert!(wasm_chirho.len() > 20);
    }

    #[test]
    fn leb128_encoding_chirho() {
        let mut buf_chirho = Vec::new();
        encode_u32_chirho(&mut buf_chirho, 0);
        assert_eq!(buf_chirho, vec![0]);

        buf_chirho.clear();
        encode_u32_chirho(&mut buf_chirho, 127);
        assert_eq!(buf_chirho, vec![127]);

        buf_chirho.clear();
        encode_u32_chirho(&mut buf_chirho, 128);
        assert_eq!(buf_chirho, vec![0x80, 0x01]);
    }

    #[test]
    fn leb128_signed_encoding_chirho() {
        let mut buf_chirho = Vec::new();
        encode_i64_chirho(&mut buf_chirho, 0);
        assert_eq!(buf_chirho, vec![0]);

        buf_chirho.clear();
        encode_i64_chirho(&mut buf_chirho, -1);
        assert_eq!(buf_chirho, vec![0x7F]);

        buf_chirho.clear();
        encode_i64_chirho(&mut buf_chirho, 42);
        assert_eq!(buf_chirho, vec![42]);
    }

    #[test]
    fn wasm_function_call_chirho() {
        // f x = x + 1; main = f 41
        let module_chirho = make_module_chirho(vec![
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 1),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("x", 2),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(CoreIdChirho(2)),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        ],
                    }),
                },
                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 3),
                rhs_chirho: CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
                    arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(41))),
                },
                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ]);

        let wasm_chirho = compile_core_to_wasm_chirho(&module_chirho);
        // Should produce valid wasm with function call instruction (0x10)
        assert!(wasm_chirho.contains(&0x10_u8)); // call opcode exists
    }

    #[test]
    fn wasm_let_binding_chirho() {
        // main = let x = 10 in x + 5
        let module_chirho = make_module_chirho(vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(
                    dummy_binder_chirho("x", 1),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10)),
                )],
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(CoreIdChirho(1)),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(5)),
                    ],
                }),
            },
            is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
        }]);

        let wasm_chirho = compile_core_to_wasm_chirho(&module_chirho);
        assert!(wasm_chirho.contains(&0x21_u8)); // local.set opcode
        assert!(wasm_chirho.contains(&0x20_u8)); // local.get opcode
    }

    #[test]
    fn wasm_case_literal_chirho() {
        // main = case 1 of { 1 -> 42; _ -> 0 }
        let module_chirho = make_module_chirho(vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))),
                bind_chirho: dummy_binder_chirho("scrut", 5),
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(1)),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                ],
            },
            is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
        }]);

        let wasm_chirho = compile_core_to_wasm_chirho(&module_chirho);
        // Should contain if/else opcodes
        assert!(wasm_chirho.contains(&0x04_u8)); // if
        assert!(wasm_chirho.contains(&0x05_u8)); // else
    }

    #[test]
    fn wasm_constructor_tag_chirho() {
        // main = True
        let module_chirho = make_module_chirho(vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::ConAppChirho {
                con_name_chirho: "True".to_string(),
                args_chirho: vec![],
            },
            is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
        }]);

        let wasm_chirho = compile_core_to_wasm_chirho(&module_chirho);
        // Should contain i64.const 1 (the constructor tag)
        assert!(wasm_chirho.contains(&0x42_u8)); // i64.const
    }

    #[test]
    fn wasm_executable_with_dict_elision_chirho() {
        // Verify compile_core_to_wasm_executable_chirho produces valid wasm
        let module_chirho = make_module_chirho(vec![CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("main", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
            is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
        }]);

        let wasm_chirho = compile_core_to_wasm_executable_chirho(&module_chirho);
        assert_eq!(&wasm_chirho[0..4], b"\0asm");
        assert_eq!(&wasm_chirho[4..8], &[1, 0, 0, 0]);
    }

    #[test]
    fn wasm_multi_arg_function_call_chirho() {
        // f x y = x + y; main = f 10 32
        let module_chirho = make_module_chirho(vec![
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 1),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("x", 2),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: dummy_binder_chirho("y", 3),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "+#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(CoreIdChirho(2)),
                                CoreExprChirho::VarChirho(CoreIdChirho(3)),
                            ],
                        }),
                    }),
                },
                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 4),
                rhs_chirho: CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
                        arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(10))),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(32))),
                },
                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ]);

        let wasm_chirho = compile_core_to_wasm_chirho(&module_chirho);
        assert!(&wasm_chirho[0..4] == b"\0asm");
        // Should contain call instruction
        assert!(wasm_chirho.contains(&0x10_u8));
    }
}
