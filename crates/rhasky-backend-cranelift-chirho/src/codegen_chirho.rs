// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Core IR → Cranelift IR code generation.
//!
//! Translates `CoreModuleChirho` bindings into Cranelift IR functions,
//! builds an object module, and emits a native object file.

use cranelift_codegen::ir::types as cl_types_chirho;
use cranelift_codegen::ir::{AbiParam as AbiParamChirho, Function as ClFunctionChirho, InstBuilder as _};
use cranelift_codegen::isa as cl_isa_chirho;
use cranelift_codegen::settings as cl_settings_chirho;
use cranelift_frontend::{FunctionBuilder as FuncBuilderChirho, FunctionBuilderContext as FuncBuilderCtxChirho};
use cranelift_module::{Linkage as LinkageChirho, Module as ModuleTraitChirho};
use cranelift_object::{ObjectBuilder as ObjBuilderChirho, ObjectModule as ObjModuleChirho};

use rhasky_core_chirho::expr_chirho::{CoreBindingChirho, CoreExprChirho, CoreModuleChirho};

use crate::{NativeObjectChirho, TargetConfigChirho};

/// Compile a `CoreModuleChirho` to a native object file via Cranelift.
pub fn compile_core_to_object_chirho(
    module_chirho: &CoreModuleChirho,
    config_chirho: &TargetConfigChirho,
) -> Result<NativeObjectChirho, String> {
    // Build Cranelift ISA from target triple.
    let flag_builder_chirho = cl_settings_chirho::builder();
    let isa_builder_chirho = cl_isa_chirho::lookup_by_name(&config_chirho.triple_chirho)
        .map_err(|e_chirho| format!("unsupported target triple '{}': {}", config_chirho.triple_chirho, e_chirho))?;
    let flags_chirho = cl_settings_chirho::Flags::new(flag_builder_chirho);
    let isa_chirho = isa_builder_chirho
        .finish(flags_chirho)
        .map_err(|e_chirho| format!("failed to build ISA: {e_chirho}"))?;

    // Create object module.
    let obj_builder_chirho = ObjBuilderChirho::new(
        isa_chirho,
        "rhasky_module_chirho",
        cranelift_module::default_libcall_names(),
    )
    .map_err(|e_chirho| format!("failed to create object builder: {e_chirho}"))?;
    let mut obj_module_chirho = ObjModuleChirho::new(obj_builder_chirho);

    let mut fb_ctx_chirho = FuncBuilderCtxChirho::new();

    // Lower each top-level binding to a Cranelift function.
    for binding_chirho in &module_chirho.bindings_chirho {
        lower_binding_chirho(
            &mut obj_module_chirho,
            &mut fb_ctx_chirho,
            binding_chirho,
        )?;
    }

    // Finalize and emit object bytes.
    let product_chirho = obj_module_chirho.finish();
    let object_bytes_chirho = product_chirho.emit()
        .map_err(|e_chirho| format!("failed to emit object: {e_chirho}"))?;

    Ok(NativeObjectChirho {
        object_bytes_chirho,
        target_triple_chirho: config_chirho.triple_chirho.clone(),
    })
}

/// Lower a single Core binding to a Cranelift function.
fn lower_binding_chirho(
    module_chirho: &mut ObjModuleChirho,
    fb_ctx_chirho: &mut FuncBuilderCtxChirho,
    binding_chirho: &CoreBindingChirho,
) -> Result<(), String> {
    let name_chirho = &binding_chirho.binder_chirho.name_chirho;

    // Create function signature: for now, all functions take and return i64
    // (boxed/tagged pointer representation).
    let mut sig_chirho = module_chirho.make_signature();
    sig_chirho.returns.push(AbiParamChirho::new(cl_types_chirho::I64));

    // Count parameters from nested lambdas.
    let (param_count_chirho, _body_chirho) = count_params_chirho(&binding_chirho.rhs_chirho);
    for _ in 0..param_count_chirho {
        sig_chirho.params.push(AbiParamChirho::new(cl_types_chirho::I64));
    }

    // Declare the function in the object module.
    let linkage_chirho = if name_chirho == "main" {
        LinkageChirho::Export
    } else {
        LinkageChirho::Local
    };
    let func_id_chirho = module_chirho
        .declare_function(name_chirho, linkage_chirho, &sig_chirho)
        .map_err(|e_chirho| format!("failed to declare function '{name_chirho}': {e_chirho}"))?;

    // Build function body.
    let mut func_chirho = ClFunctionChirho::with_name_signature(
        cranelift_codegen::ir::UserFuncName::user(0, func_id_chirho.as_u32()),
        sig_chirho,
    );

    {
        let mut builder_chirho = FuncBuilderChirho::new(&mut func_chirho, fb_ctx_chirho);
        let entry_block_chirho = builder_chirho.create_block();
        builder_chirho.append_block_params_for_function_params(entry_block_chirho);
        builder_chirho.switch_to_block(entry_block_chirho);
        builder_chirho.seal_block(entry_block_chirho);

        // TODO: lower CoreExprChirho → Cranelift IR instructions
        // For now, return 0 as placeholder.
        let zero_chirho = builder_chirho.ins().iconst(cl_types_chirho::I64, 0);
        builder_chirho.ins().return_(&[zero_chirho]);

        builder_chirho.finalize();
    }

    // Define the function in the module.
    let mut ctx_chirho = cranelift_codegen::Context::for_function(func_chirho);
    module_chirho
        .define_function(func_id_chirho, &mut ctx_chirho)
        .map_err(|e_chirho| format!("failed to define function '{name_chirho}': {e_chirho}"))?;

    Ok(())
}

/// Count the number of lambda parameters wrapping a Core expression.
fn count_params_chirho(expr_chirho: &CoreExprChirho) -> (usize, &CoreExprChirho) {
    match expr_chirho {
        CoreExprChirho::LamChirho { body_chirho, .. } => {
            let (inner_count_chirho, inner_body_chirho) = count_params_chirho(body_chirho);
            (1 + inner_count_chirho, inner_body_chirho)
        }
        other_chirho => (0, other_chirho),
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::TargetConfigChirho;
    use rhasky_core_chirho::expr_chirho::{
        BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho,
        CoreModuleChirho,
    };
    use rhasky_span_chirho::SpanChirho;
    use rhasky_typing_chirho::ty_chirho::TyChirho;

    #[test]
    fn compile_empty_module_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![],
            names_chirho: Default::default(),
        };
        let config_chirho = TargetConfigChirho::default();
        let result_chirho = compile_core_to_object_chirho(&module_chirho, &config_chirho);
        assert!(result_chirho.is_ok());
        let obj_chirho = result_chirho.unwrap();
        assert!(!obj_chirho.object_bytes_chirho.is_empty());
    }

    #[test]
    fn compile_simple_binding_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: CoreIdChirho(0),
                    name_chirho: "main".to_string(),
                    ty_chirho: TyChirho::int_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                is_rec_chirho: false,
            }],
            names_chirho: Default::default(),
        };
        let config_chirho = TargetConfigChirho::default();
        let result_chirho = compile_core_to_object_chirho(&module_chirho, &config_chirho);
        assert!(result_chirho.is_ok());
    }

    #[test]
    fn compile_lambda_binding_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: CoreIdChirho(0),
                    name_chirho: "id".to_string(),
                    ty_chirho: TyChirho::int_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: BinderChirho {
                        id_chirho: CoreIdChirho(1),
                        name_chirho: "x".to_string(),
                        ty_chirho: TyChirho::int_chirho(),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
                },
                is_rec_chirho: false,
            }],
            names_chirho: Default::default(),
        };
        let config_chirho = TargetConfigChirho::default();
        let result_chirho = compile_core_to_object_chirho(&module_chirho, &config_chirho);
        assert!(result_chirho.is_ok());
    }
}
