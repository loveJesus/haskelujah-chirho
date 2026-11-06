// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Root ownership belongs to generated activations and their actual value handoffs.
//! Workflow: testing-chirho/native-root-lifetimes-chirho.md.

use cranelift_codegen::ir::{
    AbiParam as AbiParamChirho, FuncRef as FuncRefChirho, InstBuilder as _, Value as ValueChirho,
    types as types_chirho,
};
use cranelift_frontend::FunctionBuilder as FunctionBuilderChirho;
use cranelift_module::{Linkage as LinkageChirho, Module as _};
use cranelift_object::ObjectModule as ObjectModuleChirho;

#[derive(Clone, Copy)]
pub(crate) struct NativeRootFrameEmitterChirho {
    token_chirho: ValueChirho,
    reset_chirho: FuncRefChirho,
    restore_chirho: FuncRefChirho,
    push_chirho: FuncRefChirho,
}

fn declare_chirho(
    module_chirho: &mut ObjectModuleChirho,
    builder_chirho: &mut FunctionBuilderChirho,
    name_chirho: &str,
    argument_count_chirho: usize,
    returns_value_chirho: bool,
) -> Result<FuncRefChirho, String> {
    let mut signature_chirho = module_chirho.make_signature();
    for _index_chirho in 0..argument_count_chirho {
        signature_chirho
            .params
            .push(AbiParamChirho::new(types_chirho::I64));
    }
    if returns_value_chirho {
        signature_chirho
            .returns
            .push(AbiParamChirho::new(types_chirho::I64));
    }
    let id_chirho = module_chirho
        .declare_function(name_chirho, LinkageChirho::Import, &signature_chirho)
        .map_err(|error_chirho| {
            format!("native root frame import {name_chirho}: {error_chirho}")
        })?;
    Ok(module_chirho.declare_func_in_func(id_chirho, builder_chirho.func))
}

impl NativeRootFrameEmitterChirho {
    pub(crate) fn begin_chirho(
        module_chirho: &mut ObjectModuleChirho,
        builder_chirho: &mut FunctionBuilderChirho,
        parameters_chirho: &[ValueChirho],
    ) -> Result<Self, String> {
        let mark_chirho = declare_chirho(
            module_chirho,
            builder_chirho,
            "haskelujah_gc_root_frame_mark_chirho",
            0,
            true,
        )?;
        let reset_chirho = declare_chirho(
            module_chirho,
            builder_chirho,
            "haskelujah_gc_root_frame_reset_chirho",
            1,
            false,
        )?;
        let restore_chirho = declare_chirho(
            module_chirho,
            builder_chirho,
            "haskelujah_gc_root_frame_restore_chirho",
            1,
            false,
        )?;
        let push_chirho = declare_chirho(
            module_chirho,
            builder_chirho,
            "haskelujah_gc_root_push_chirho",
            1,
            false,
        )?;
        let call_chirho = builder_chirho.ins().call(mark_chirho, &[]);
        let token_chirho = builder_chirho.inst_results(call_chirho)[0];
        let frame_chirho = Self {
            token_chirho,
            reset_chirho,
            restore_chirho,
            push_chirho,
        };
        for parameter_chirho in parameters_chirho {
            frame_chirho.handoff_chirho(builder_chirho, *parameter_chirho);
        }
        Ok(frame_chirho)
    }

    /// Values only: no force, conversion, token or raw interior-pointer admission.
    pub(crate) fn handoff_chirho(
        &self,
        builder_chirho: &mut FunctionBuilderChirho,
        value_chirho: ValueChirho,
    ) {
        if builder_chirho.func.dfg.value_type(value_chirho) == types_chirho::I64 {
            builder_chirho.ins().call(self.push_chirho, &[value_chirho]);
        }
    }

    pub(crate) fn reset_chirho(
        &self,
        builder_chirho: &mut FunctionBuilderChirho,
        arguments_chirho: &[ValueChirho],
    ) {
        builder_chirho
            .ins()
            .call(self.reset_chirho, &[self.token_chirho]);
        // Neither reset nor these root pushes invokes native allocation/collection.
        for argument_chirho in arguments_chirho {
            self.handoff_chirho(builder_chirho, *argument_chirho);
        }
    }

    pub(crate) fn finish_chirho(&self, builder_chirho: &mut FunctionBuilderChirho) {
        builder_chirho
            .ins()
            .call(self.restore_chirho, &[self.token_chirho]);
    }
}
