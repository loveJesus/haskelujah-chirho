// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Core IR → Cranelift IR lowering utilities.
//!
//! Provides helpers for translating Core expressions into Cranelift IR
//! instructions, managing variable bindings, and handling the runtime
//! object layout.

use cranelift_codegen::ir::types as cl_types_chirho;
use cranelift_codegen::ir::{InstBuilder as _, Value as ClValueChirho};
use cranelift_frontend::FunctionBuilder as FuncBuilderChirho;

use rhasky_core_chirho::expr_chirho::{CoreExprChirho, CoreIdChirho, CoreLitChirho};

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

/// Lower a Core literal to a Cranelift constant.
pub fn lower_lit_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    lit_chirho: &CoreLitChirho,
) -> ClValueChirho {
    match lit_chirho {
        CoreLitChirho::IntChirho(n_chirho) => {
            builder_chirho.ins().iconst(cl_types_chirho::I64, *n_chirho)
        }
        CoreLitChirho::FloatChirho(f_chirho) => {
            builder_chirho.ins().f64const(*f_chirho)
        }
        CoreLitChirho::CharChirho(c_chirho) => {
            builder_chirho
                .ins()
                .iconst(cl_types_chirho::I64, *c_chirho as i64)
        }
        CoreLitChirho::StringChirho(s_chirho) => {
            // TODO: allocate string on heap and return pointer
            let len_chirho = s_chirho.len() as i64;
            builder_chirho.ins().iconst(cl_types_chirho::I64, len_chirho)
        }
    }
}

/// Lower a Core expression to Cranelift IR, returning the SSA value.
pub fn lower_expr_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    env_chirho: &mut VarEnvChirho,
    expr_chirho: &CoreExprChirho,
) -> ClValueChirho {
    match expr_chirho {
        CoreExprChirho::LitChirho(lit_chirho) => lower_lit_chirho(builder_chirho, lit_chirho),

        CoreExprChirho::VarChirho(id_chirho) => {
            env_chirho
                .lookup_chirho(*id_chirho)
                .unwrap_or_else(|| builder_chirho.ins().iconst(cl_types_chirho::I64, 0))
        }

        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => {
            lower_primop_chirho(builder_chirho, env_chirho, name_chirho, args_chirho)
        }

        // TODO: handle App, Lam, Let, Case, ConApp, etc.
        _ => builder_chirho.ins().iconst(cl_types_chirho::I64, 0),
    }
}

/// Lower a primitive operation to Cranelift instructions.
fn lower_primop_chirho(
    builder_chirho: &mut FuncBuilderChirho,
    env_chirho: &mut VarEnvChirho,
    name_chirho: &str,
    args_chirho: &[CoreExprChirho],
) -> ClValueChirho {
    let lhs_chirho = if !args_chirho.is_empty() {
        lower_expr_chirho(builder_chirho, env_chirho, &args_chirho[0])
    } else {
        builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
    };
    let rhs_chirho = if args_chirho.len() > 1 {
        lower_expr_chirho(builder_chirho, env_chirho, &args_chirho[1])
    } else {
        builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
    };

    match name_chirho {
        "+#" => builder_chirho.ins().iadd(lhs_chirho, rhs_chirho),
        "-#" => builder_chirho.ins().isub(lhs_chirho, rhs_chirho),
        "*#" => builder_chirho.ins().imul(lhs_chirho, rhs_chirho),
        "div#" => builder_chirho.ins().sdiv(lhs_chirho, rhs_chirho),
        "mod#" => builder_chirho.ins().srem(lhs_chirho, rhs_chirho),
        "negate#" => builder_chirho.ins().ineg(lhs_chirho),
        _ => {
            // Unknown primop — return 0 as placeholder.
            builder_chirho.ins().iconst(cl_types_chirho::I64, 0)
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn var_env_bind_lookup_chirho() {
        // VarEnvChirho is just a HashMap wrapper — test bind + lookup.
        let mut env_chirho = VarEnvChirho::new_chirho();
        // We can't create real ClValueChirho without a builder, so just test the None case.
        assert!(env_chirho.lookup_chirho(CoreIdChirho(0)).is_none());
    }
}
