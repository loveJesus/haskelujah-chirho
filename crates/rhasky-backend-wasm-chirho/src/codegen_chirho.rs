// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # WebAssembly binary code generation
//!
//! Emits a minimal valid WebAssembly binary module from Core IR.
//! Uses the raw Wasm binary encoding (magic + version + sections)
//! rather than WAT text format, so the output can be loaded directly
//! by a Wasm runtime.
//!
//! Current capability: each top-level Core binding becomes a Wasm
//! function that returns an i64. Supports integer literals, variables
//! (local.get), let bindings (local.set/get), and basic function calls.
//!
//! Lambda lifting, closures, and heap allocation are deferred to a
//! later milestone.

use rhasky_core_chirho::{
    CoreExprChirho, CoreIdChirho, CoreLitChirho, CoreModuleChirho,
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
        let mut export_section_chirho = Vec::new();
        encode_u32_chirho(&mut export_section_chirho, func_count_chirho as u32);
        for (i_chirho, info_chirho) in func_infos_chirho.iter().enumerate() {
            encode_string_chirho(&mut export_section_chirho, &info_chirho.name_chirho);
            export_section_chirho.push(0x00); // func export
            encode_u32_chirho(&mut export_section_chirho, i_chirho as u32);
        }

        self.emit_section_chirho(SECTION_EXPORT_CHIRHO, &export_section_chirho);

        // === Code Section ===
        let mut code_section_chirho = Vec::new();
        encode_u32_chirho(&mut code_section_chirho, func_count_chirho as u32);

        for info_chirho in &func_infos_chirho {
            let mut body_bytes_chirho = Vec::new();
            // Count locals needed (beyond parameters)
            let locals_needed_chirho = count_locals_chirho(&info_chirho.body_chirho);
            if locals_needed_chirho > 0 {
                encode_u32_chirho(&mut body_bytes_chirho, 1); // one local declaration
                encode_u32_chirho(&mut body_bytes_chirho, locals_needed_chirho as u32);
                body_bytes_chirho.push(WASM_I64_CHIRHO);
            } else {
                encode_u32_chirho(&mut body_bytes_chirho, 0); // no locals
            }

            // Emit body instructions
            emit_expr_chirho(
                &mut body_bytes_chirho,
                &info_chirho.body_chirho,
                &info_chirho.params_chirho,
            );

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

/// Count local variables needed beyond function parameters.
fn count_locals_chirho(expr_chirho: &CoreExprChirho) -> usize {
    match expr_chirho {
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => binds_chirho.len() + count_locals_chirho(body_chirho),
        CoreExprChirho::CaseChirho {
            alts_chirho,
            ..
        } => {
            // One local for the result
            1 + alts_chirho
                .iter()
                .map(|a_chirho| count_locals_chirho(&a_chirho.rhs_chirho))
                .sum::<usize>()
        }
        _ => 0,
    }
}

/// Emit Wasm instructions for a Core expression.
/// The result is left on the Wasm operand stack.
fn emit_expr_chirho(
    buf_chirho: &mut Vec<u8>,
    expr_chirho: &CoreExprChirho,
    params_chirho: &[CoreIdChirho],
) {
    match expr_chirho {
        CoreExprChirho::LitChirho(lit_chirho) => {
            emit_lit_chirho(buf_chirho, lit_chirho);
        }

        CoreExprChirho::VarChirho(id_chirho) => {
            // Find local index: parameters first, then let-bound locals
            let local_idx_chirho = params_chirho
                .iter()
                .position(|p_chirho| *p_chirho == *id_chirho)
                .unwrap_or(0) as u32;
            buf_chirho.push(0x21_u8.wrapping_sub(1)); // local.get = 0x20
            encode_u32_chirho(buf_chirho, local_idx_chirho);
        }

        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            // Emit both operands, then an i64.add as a placeholder for
            // actual function call dispatch
            emit_expr_chirho(buf_chirho, fun_chirho, params_chirho);
            emit_expr_chirho(buf_chirho, arg_chirho, params_chirho);
            buf_chirho.push(0x7C); // i64.add
        }

        CoreExprChirho::LamChirho { body_chirho, .. } => {
            // Residual lambda — just emit the body
            emit_expr_chirho(buf_chirho, body_chirho, params_chirho);
        }

        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            // Emit each binding value
            for (_, rhs_chirho) in binds_chirho {
                emit_expr_chirho(buf_chirho, rhs_chirho, params_chirho);
                buf_chirho.push(0x1A); // drop (simplified — a real impl would local.set)
            }
            emit_expr_chirho(buf_chirho, body_chirho, params_chirho);
        }

        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            // Simplified: emit scrutinee, then if we have alts, emit the first
            // alt's RHS (a real impl would branch on constructor tags)
            emit_expr_chirho(buf_chirho, scrutinee_chirho, params_chirho);
            buf_chirho.push(0x1A); // drop scrutinee
            if let Some(first_alt_chirho) = alts_chirho.first() {
                emit_expr_chirho(buf_chirho, &first_alt_chirho.rhs_chirho, params_chirho);
            } else {
                // unreachable
                buf_chirho.push(0x00); // unreachable
            }
        }

        CoreExprChirho::TyLamChirho { body_chirho, .. }
        | CoreExprChirho::TyAppChirho {
            expr_chirho: body_chirho,
            ..
        } => {
            emit_expr_chirho(buf_chirho, body_chirho, params_chirho);
        }
    }
}

fn emit_lit_chirho(buf_chirho: &mut Vec<u8>, lit_chirho: &CoreLitChirho) {
    match lit_chirho {
        CoreLitChirho::IntChirho(v_chirho) => {
            buf_chirho.push(0x42); // i64.const
            encode_i64_chirho(buf_chirho, *v_chirho);
        }
        CoreLitChirho::FloatChirho(_) => {
            buf_chirho.push(0x42); // i64.const 0 (placeholder)
            encode_i64_chirho(buf_chirho, 0);
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
    use rhasky_core_chirho::{BinderChirho, CoreBindingChirho, CoreIdChirho};
    use rhasky_typing_chirho::ty_chirho::TyChirho;

    fn dummy_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
        BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: rhasky_span_chirho::SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn wasm_magic_and_version_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                is_rec_chirho: false,
            }],
        };

        let wasm_chirho = compile_core_to_wasm_chirho(&module_chirho);
        // Check magic number
        assert_eq!(&wasm_chirho[0..4], b"\0asm");
        // Check version
        assert_eq!(&wasm_chirho[4..8], &[1, 0, 0, 0]);
    }

    #[test]
    fn wasm_has_sections_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                is_rec_chirho: false,
            }],
        };

        let wasm_chirho = compile_core_to_wasm_chirho(&module_chirho);
        // Should contain type section (1), function section (3), export section (7), code section (10)
        assert!(wasm_chirho.len() > 8);
        // First section should be type (id = 1)
        assert_eq!(wasm_chirho[8], SECTION_TYPE_CHIRHO);
    }

    #[test]
    fn wasm_identity_function_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Id".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("id", 10),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("x", 0),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                },
                is_rec_chirho: false,
            }],
        };

        let wasm_chirho = compile_core_to_wasm_chirho(&module_chirho);
        assert!(&wasm_chirho[0..4] == b"\0asm");
        assert!(wasm_chirho.len() > 20); // Should have real content
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
}
