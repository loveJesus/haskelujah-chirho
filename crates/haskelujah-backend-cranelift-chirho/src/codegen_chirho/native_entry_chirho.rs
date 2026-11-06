// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The native launcher owns process status, independently of a lazy IO payload.
//! Workflow: testing-chirho/native-entry-role-chirho.

use cranelift_codegen::ir::{
    AbiParam as AbiParamChirho, Function as ClFunctionChirho, InstBuilder as _,
    types as cl_types_chirho,
};
use cranelift_frontend::{
    FunctionBuilder as FuncBuilderChirho, FunctionBuilderContext as FuncBuilderCtxChirho,
};
use cranelift_module::{Linkage as LinkageChirho, Module as _};
use cranelift_object::ObjectModule as ObjModuleChirho;
use haskelujah_core_chirho::io_actions_chirho::NativeExecutionRoleChirho;

pub(super) fn define_native_main_wrapper_chirho(
    module_chirho: &mut ObjModuleChirho,
    fb_ctx_chirho: &mut FuncBuilderCtxChirho,
    haskelujah_main_func_id_chirho: cranelift_module::FuncId,
    main_with_large_stack_func_id_chirho: cranelift_module::FuncId,
    execution_role_chirho: NativeExecutionRoleChirho,
) -> Result<(), String> {
    let mut native_main_sig_chirho = module_chirho.make_signature();
    native_main_sig_chirho
        .returns
        .push(AbiParamChirho::new(cl_types_chirho::I32));
    let native_main_func_id_chirho = module_chirho
        .declare_function("main", LinkageChirho::Export, &native_main_sig_chirho)
        .map_err(|error_chirho| format!("failed to declare native main wrapper: {error_chirho}"))?;

    let mut func_chirho = ClFunctionChirho::with_name_signature(
        cranelift_codegen::ir::UserFuncName::user(0, native_main_func_id_chirho.as_u32()),
        native_main_sig_chirho,
    );

    {
        let mut builder_chirho = FuncBuilderChirho::new(&mut func_chirho, fb_ctx_chirho);
        let entry_block_chirho = builder_chirho.create_block();
        builder_chirho.switch_to_block(entry_block_chirho);
        builder_chirho.seal_block(entry_block_chirho);

        let haskelujah_main_ref_chirho =
            module_chirho.declare_func_in_func(haskelujah_main_func_id_chirho, builder_chirho.func);
        let main_with_large_stack_ref_chirho = module_chirho
            .declare_func_in_func(main_with_large_stack_func_id_chirho, builder_chirho.func);
        let haskelujah_main_addr_chirho = builder_chirho
            .ins()
            .func_addr(cl_types_chirho::I64, haskelujah_main_ref_chirho);
        let call_inst_chirho = builder_chirho.ins().call(
            main_with_large_stack_ref_chirho,
            &[haskelujah_main_addr_chirho],
        );
        let exit_code_chirho = match execution_role_chirho {
            NativeExecutionRoleChirho::ProgramChirho => {
                // Executing the action is required; its lazy result is deliberately unused.
                builder_chirho.ins().iconst(cl_types_chirho::I32, 0)
            }
            NativeExecutionRoleChirho::ValueChirho => {
                let result_chirho = builder_chirho.inst_results(call_inst_chirho)[0];
                builder_chirho
                    .ins()
                    .ireduce(cl_types_chirho::I32, result_chirho)
            }
        };
        builder_chirho.ins().return_(&[exit_code_chirho]);
        builder_chirho.finalize();
    }

    let mut ctx_chirho = cranelift_codegen::Context::for_function(func_chirho);
    if let Err(verifier_error_chirho) = ctx_chirho.verify(module_chirho.isa()) {
        return Err(format!(
            "failed to verify native main wrapper: {verifier_error_chirho}\n{}",
            ctx_chirho.func.display()
        ));
    }
    module_chirho
        .define_function(native_main_func_id_chirho, &mut ctx_chirho)
        .map_err(|error_chirho| format!("failed to define native main wrapper: {error_chirho}"))?;

    Ok(())
}
