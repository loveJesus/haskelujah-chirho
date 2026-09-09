// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Required failure-path imports. A missing runtime declaration is a compile
//! error, never a null function pointer or a fabricated result.

use cranelift_codegen::ir::{AbiParam, FuncRef, Function, InstBuilder, Value, types};
use cranelift_frontend::FunctionBuilder;
use cranelift_module::{Linkage, Module};
use cranelift_object::ObjectModule;

pub(crate) struct RuntimeCallsChirho {
    error_chirho: FuncRef,
    strlen_chirho: FuncRef,
}

impl RuntimeCallsChirho {
    pub(crate) fn declare_chirho(
        module_chirho: &mut ObjectModule,
        function_chirho: &mut Function,
    ) -> Result<Self, String> {
        let mut error_signature_chirho = module_chirho.make_signature();
        error_signature_chirho
            .params
            .extend([AbiParam::new(types::I64); 2]);
        let error_id_chirho = module_chirho
            .declare_function(
                "haskelujah_error_chirho",
                Linkage::Import,
                &error_signature_chirho,
            )
            .map_err(|error_chirho| format!("declaring runtime error: {error_chirho}"))?;
        let mut strlen_signature_chirho = module_chirho.make_signature();
        strlen_signature_chirho
            .params
            .push(AbiParam::new(types::I64));
        strlen_signature_chirho
            .returns
            .push(AbiParam::new(types::I64));
        let strlen_id_chirho = module_chirho
            .declare_function("strlen", Linkage::Import, &strlen_signature_chirho)
            .map_err(|error_chirho| format!("declaring string length: {error_chirho}"))?;
        Ok(Self {
            error_chirho: module_chirho.declare_func_in_func(error_id_chirho, function_chirho),
            strlen_chirho: module_chirho.declare_func_in_func(strlen_id_chirho, function_chirho),
        })
    }

    /// The caller demands the message before crossing the C-string boundary.
    pub(crate) fn error_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        message_chirho: Value,
    ) -> Value {
        let length_call_chirho = builder_chirho
            .ins()
            .call(self.strlen_chirho, &[message_chirho]);
        let length_chirho = builder_chirho.inst_results(length_call_chirho)[0];
        builder_chirho
            .ins()
            .call(self.error_chirho, &[message_chirho, length_chirho]);
        // The runtime aborts; this syntactic value lets expression lowering
        // finish its block. It is unreachable in the generated program.
        builder_chirho.ins().iconst(types::I64, 0)
    }
}
