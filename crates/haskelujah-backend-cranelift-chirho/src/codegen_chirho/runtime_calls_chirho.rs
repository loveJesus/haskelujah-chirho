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
    pow_double_int_chirho: FuncRef,
    pow_float_chirho: FuncRef,
    atan2_double_chirho: FuncRef,
    exponent_double_chirho: FuncRef,
    significand_double_chirho: FuncRef,
    new_io_ref_chirho: FuncRef,
    read_io_ref_chirho: FuncRef,
    write_io_ref_chirho: FuncRef,
    map_from_list_chirho: FuncRef,
    map_size_chirho: FuncRef,
    char_to_upper_chirho: FuncRef,
    round_float_to_int_chirho: FuncRef,
    truncate_float_to_int_chirho: FuncRef,
    floor_float_to_int_chirho: FuncRef,
    ceiling_float_to_int_chirho: FuncRef,
    nul_string_literal_chirho: FuncRef,
    show_string_chirho: FuncRef,
    /// Each unary Floating primop with its RTS libm import (row 575).
    float_math_imports_chirho: Vec<(&'static str, FuncRef)>,
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
            pow_double_int_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_pow_double_int_chirho",
                2,
                true,
            )?,
            pow_float_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_pow_float_chirho",
                2,
                true,
            )?,
            atan2_double_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_atan2_double_chirho",
                2,
                true,
            )?,
            exponent_double_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_exponent_double_chirho",
                1,
                true,
            )?,
            significand_double_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_significand_double_chirho",
                1,
                true,
            )?,
            new_io_ref_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_new_io_ref_chirho",
                1,
                true,
            )?,
            read_io_ref_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_read_io_ref_chirho",
                1,
                true,
            )?,
            write_io_ref_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_write_io_ref_chirho",
                2,
                true,
            )?,
            map_from_list_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_map_from_list_chirho",
                2,
                true,
            )?,
            map_size_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_map_size_chirho",
                1,
                true,
            )?,
            char_to_upper_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_char_to_upper_chirho",
                1,
                true,
            )?,
            round_float_to_int_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_round_float_to_int_chirho",
                1,
                true,
            )?,
            truncate_float_to_int_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_truncate_float_to_int_chirho",
                1,
                true,
            )?,
            floor_float_to_int_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_floor_float_to_int_chirho",
                1,
                true,
            )?,
            ceiling_float_to_int_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_ceiling_float_to_int_chirho",
                1,
                true,
            )?,
            nul_string_literal_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_nul_string_literal_chirho",
                0,
                true,
            )?,
            show_string_chirho: import_chirho(
                module_chirho,
                function_chirho,
                "haskelujah_show_string_chirho",
                1,
                true,
            )?,
            float_math_imports_chirho:
                haskelujah_rts_chirho::float_math_chirho::FLOAT_MATH_PRIMOPS_CHIRHO
                    .iter()
                    .map(|(primop_chirho, symbol_chirho)| {
                        import_chirho(module_chirho, function_chirho, symbol_chirho, 1, true)
                            .map(|import_chirho| (*primop_chirho, import_chirho))
                    })
                    .collect::<Result<_, _>>()?,
        })
    }

    /// A unary Floating primop's call into the RTS's libm symbol over the operand's bits
    /// (row 575), or None for any other primop.
    pub(crate) fn float_math_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        primop_chirho: &str,
        bits_chirho: Value,
    ) -> Option<Value> {
        let (_, import_chirho) = self
            .float_math_imports_chirho
            .iter()
            .find(|(name_chirho, _)| *name_chirho == primop_chirho)?;
        let call_chirho = builder_chirho.ins().call(*import_chirho, &[bits_chirho]);
        Some(builder_chirho.inst_results(call_chirho)[0])
    }

    /// `exponentFloatChirho#`: GHC's representation exponent for a Double (row 585).
    pub(crate) fn exponent_double_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        bits_chirho: Value,
    ) -> Value {
        let call_chirho = builder_chirho
            .ins()
            .call(self.exponent_double_chirho, &[bits_chirho]);
        builder_chirho.inst_results(call_chirho)[0]
    }

    /// `significandFloatChirho#`: GHC's normalized Double significand (row 585).
    pub(crate) fn significand_double_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        bits_chirho: Value,
    ) -> Value {
        let call_chirho = builder_chirho
            .ins()
            .call(self.significand_double_chirho, &[bits_chirho]);
        builder_chirho.inst_results(call_chirho)[0]
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

    /// `^.#`, GHC.Real's `(^)` at Double over the base's bits (row 570).
    pub(crate) fn pow_double_int_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        base_bits_chirho: Value,
        exponent_chirho: Value,
    ) -> Value {
        let call_chirho = builder_chirho.ins().call(
            self.pow_double_int_chirho,
            &[base_bits_chirho, exponent_chirho],
        );
        builder_chirho.inst_results(call_chirho)[0]
    }

    /// `**#`, GHC's Double `**` (row 573), or `atan2#`, GHC's RealFloat default `atan2`
    /// (row 580): the RTS's one routine over both operands' bits.
    pub(crate) fn float_binary_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        primop_chirho: &str,
        lhs_bits_chirho: Value,
        rhs_bits_chirho: Value,
    ) -> Value {
        let routine_chirho = if primop_chirho == "atan2#" {
            self.atan2_double_chirho
        } else {
            self.pow_float_chirho
        };
        let call_chirho = builder_chirho
            .ins()
            .call(routine_chirho, &[lhs_bits_chirho, rhs_bits_chirho]);
        builder_chirho.inst_results(call_chirho)[0]
    }

    pub(crate) fn new_io_ref_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        initial_value_chirho: Value,
    ) -> Value {
        let call_chirho = builder_chirho
            .ins()
            .call(self.new_io_ref_chirho, &[initial_value_chirho]);
        builder_chirho.inst_results(call_chirho)[0]
    }

    pub(crate) fn read_io_ref_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        io_ref_chirho: Value,
    ) -> Value {
        let call_chirho = builder_chirho
            .ins()
            .call(self.read_io_ref_chirho, &[io_ref_chirho]);
        builder_chirho.inst_results(call_chirho)[0]
    }

    pub(crate) fn write_io_ref_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        io_ref_chirho: Value,
        value_chirho: Value,
    ) -> Value {
        let call_chirho = builder_chirho
            .ins()
            .call(self.write_io_ref_chirho, &[io_ref_chirho, value_chirho]);
        builder_chirho.inst_results(call_chirho)[0]
    }

    pub(crate) fn map_from_list_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        list_chirho: Value,
        pair_tag_chirho: Value,
    ) -> Value {
        let call_chirho = builder_chirho
            .ins()
            .call(self.map_from_list_chirho, &[list_chirho, pair_tag_chirho]);
        builder_chirho.inst_results(call_chirho)[0]
    }

    pub(crate) fn map_size_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        map_chirho: Value,
    ) -> Value {
        let call_chirho = builder_chirho
            .ins()
            .call(self.map_size_chirho, &[map_chirho]);
        builder_chirho.inst_results(call_chirho)[0]
    }

    /// `toUpper#`, sharing the interpreter's Unicode scalar convention.
    pub(crate) fn char_to_upper_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        value_chirho: Value,
    ) -> Value {
        let call_chirho = builder_chirho
            .ins()
            .call(self.char_to_upper_chirho, &[value_chirho]);
        builder_chirho.inst_results(call_chirho)[0]
    }

    pub(crate) fn float_to_int_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        primitive_chirho: &str,
        value_chirho: Value,
    ) -> Value {
        let function_chirho = match primitive_chirho {
            "round#" => self.round_float_to_int_chirho,
            "truncate#" => self.truncate_float_to_int_chirho,
            "floor#" => self.floor_float_to_int_chirho,
            "ceiling#" => self.ceiling_float_to_int_chirho,
            _ => unreachable!("the caller supplies one of the four float-to-Int primitives"),
        };
        let call_chirho = builder_chirho.ins().call(function_chirho, &[value_chirho]);
        builder_chirho.inst_results(call_chirho)[0]
    }

    /// An evaluated string literal holding NUL (row 560): its NUL-terminated global would read
    /// shorter, so the runtime fails by its named line.
    pub(crate) fn nul_string_literal_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
    ) -> Value {
        let call_chirho = builder_chirho
            .ins()
            .call(self.nul_string_literal_chirho, &[]);
        builder_chirho.inst_results(call_chirho)[0]
    }

    /// `showStr#`: GHC's `show` for a `String`, by the renderer the interpreter
    /// shares. The runtime forces the value and fails named on a representation
    /// it cannot read exactly.
    /// workflow: print-show-evidence-chirho
    pub(crate) fn show_string_chirho(
        &self,
        builder_chirho: &mut FunctionBuilder<'_>,
        value_chirho: Value,
    ) -> Value {
        let call_chirho = builder_chirho
            .ins()
            .call(self.show_string_chirho, &[value_chirho]);
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
