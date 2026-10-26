// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Required runtime imports. A missing runtime declaration is a compile error,
//! never a null function pointer or a fabricated result. Each failure leaves
//! through the runtime's one route: one stderr line and exit 1.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

use cranelift_codegen::ir::{AbiParam, FuncRef, Function, InstBuilder, Value, types};
use cranelift_frontend::FunctionBuilder;
use cranelift_module::{Linkage, Module};
use cranelift_object::ObjectModule;

pub(crate) struct RuntimeCallsChirho {
    error_chirho: FuncRef,
    strlen_chirho: FuncRef,
    no_match_chirho: FuncRef,
    undefined_chirho: FuncRef,
    uncaught_exception_chirho: FuncRef,
    unbound_thunk_chirho: FuncRef,
    pow_int_chirho: FuncRef,
}

fn import_chirho(
    module_chirho: &mut ObjectModule,
    function_chirho: &mut Function,
    name_chirho: &str,
    param_count_chirho: usize,
    returns_value_chirho: bool,
) -> Result<FuncRef, String> {
    let mut signature_chirho = module_chirho.make_signature();
    signature_chirho
        .params
        .extend((0..param_count_chirho).map(|_| AbiParam::new(types::I64)));
    if returns_value_chirho {
        signature_chirho.returns.push(AbiParam::new(types::I64));
    }
    let id_chirho = module_chirho
        .declare_function(name_chirho, Linkage::Import, &signature_chirho)
        .map_err(|error_chirho| format!("declaring {name_chirho}: {error_chirho}"))?;
    Ok(module_chirho.declare_func_in_func(id_chirho, function_chirho))
}

impl RuntimeCallsChirho {
    pub(crate) fn declare_chirho(
        module_chirho: &mut ObjectModule,
        function_chirho: &mut Function,
    ) -> Result<Self, String> {
        Ok(Self {
            error_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_error_chirho",
                2,
                false,
            )?,
            strlen_chirho: import_chirho(module_chirho, function_chirho, "strlen", 1, true)?,
            no_match_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_no_match_chirho",
                0,
                false,
            )?,
            undefined_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_undefined_chirho",
                0,
                false,
            )?,
            uncaught_exception_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_uncaught_exception_chirho",
                0,
                false,
            )?,
            unbound_thunk_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_unbound_thunk_chirho",
                2,
                true,
            )?,
            pow_int_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_pow_int_chirho",
                2,
                true,
            )?,
        })
    }

    /// `^#`, sharing the interpreter's wrapping integer-power convention.
    pub(crate) fn pow_int_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        base_chirho: Value,
        exponent_chirho: Value,
    ) -> Value {
        let call_chirho = builder_chirho
            .ins()
            .call(self.pow_int_chirho, &[base_chirho, exponent_chirho]);
        builder_chirho.inst_results(call_chirho)[0]
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
        // The runtime exits; this syntactic value lets expression lowering
        // finish its block. It is unreachable in the generated program.
        builder_chirho.ins().iconst(types::I64, 0)
    }

    /// A case none of whose alternatives matched. Emits only the call, so the
    /// caller still ends the block with its own terminator.
    pub(crate) fn no_match_chirho(&self, builder_chirho: &mut FunctionBuilder<'_>) {
        builder_chirho.ins().call(self.no_match_chirho, &[]);
    }

    /// The same failure where expression lowering needs a value.
    pub(crate) fn no_match_value_chirho(&self, builder_chirho: &mut FunctionBuilder<'_>) -> Value {
        self.no_match_chirho(builder_chirho);
        builder_chirho.ins().iconst(types::I64, 0)
    }

    /// `undefined`, failing where it is referenced, as on LLVM.
    pub(crate) fn undefined_chirho(&self, builder_chirho: &mut FunctionBuilder<'_>) -> Value {
        builder_chirho.ins().call(self.undefined_chirho, &[]);
        builder_chirho.ins().iconst(types::I64, 0)
    }

    /// `throw#`: a native executable has no handler to catch it.
    pub(crate) fn uncaught_exception_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
    ) -> Value {
        builder_chirho
            .ins()
            .call(self.uncaught_exception_chirho, &[]);
        builder_chirho.ins().iconst(types::I64, 0)
    }

    /// The value of an unbound reference: the runtime's failing thunk for the
    /// static name at `name_chirho`, which fails named only when demanded.
    pub(crate) fn unbound_thunk_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        name_chirho: Value,
        name_len_chirho: usize,
    ) -> Value {
        let length_chirho = builder_chirho
            .ins()
            .iconst(types::I64, name_len_chirho as i64);
        let call_chirho = builder_chirho
            .ins()
            .call(self.unbound_thunk_chirho, &[name_chirho, length_chirho]);
        builder_chirho.inst_results(call_chirho)[0]
    }
}
