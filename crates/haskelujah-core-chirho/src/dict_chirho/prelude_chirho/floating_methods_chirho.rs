// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Floating methods at Double beyond the unary primop list. The checker typed each one,
//! but no usable Core bound it.
//! - **`logBase` (row 578)** is GHC's `log y / log x`, so `logBase 10 1000` is
//!   2.9999999999999996, as GHC prints it.
//! - **`pi` (row 580)** was a function from `()` that called a `pi#` primop. The interpreter
//!   met a closure where it needed a Double, and both natives refused `pi#`. It is now a
//!   Double value with π's bits, in its dictionary row and under its Prelude name.
//! - **`atan2` (row 580)** calls `atan2#`, the RTS's one transcription of GHC's RealFloat
//!   default, which every engine makes. It is not libm's `atan2`.
//!
//! Each Prelude name yields to a program's own top-level binding of that name.
//! workflow: testing-chirho/native-fail-loud-chirho

use super::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho,
    DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho,
};

fn primop_chirho(name_chirho: &str, args_chirho: Vec<CoreExprChirho>) -> CoreExprChirho {
    CoreExprChirho::PrimOpChirho {
        name_chirho: name_chirho.to_string(),
        args_chirho,
    }
}

fn log_chirho(id_chirho: CoreIdChirho) -> CoreExprChirho {
    primop_chirho("log#", vec![CoreExprChirho::VarChirho(id_chirho)])
}

impl DictPassCtxChirho {
    /// Generate `$prim_Floating_pi_Double = 3.141592653589793` and its Prelude alias `pi`.
    /// workflow: testing-chirho/native-fail-loud-chirho
    pub(super) fn generate_floating_pi_prelude_chirho(&mut self) {
        let prim_name_chirho = "$prim_Floating_pi_Double";
        let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: prim_id_chirho,
                name_chirho: prim_name_chirho.to_string(),
                ty_chirho: TyChirho::double_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(std::f64::consts::PI)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        self.push_prelude_alias_chirho("pi", prim_name_chirho);
    }

    /// Generate `logBase` and `atan2` at Double, each with its Prelude alias.
    /// workflow: testing-chirho/native-fail-loud-chirho
    pub(super) fn generate_floating_binary_prelude_chirho(&mut self) {
        self.push_binary_double_row_chirho("logBase", ["x", "y"], |base_chirho, value_chirho| {
            primop_chirho(
                "/.#",
                vec![log_chirho(value_chirho), log_chirho(base_chirho)],
            )
        });
        self.push_binary_double_row_chirho("atan2", ["y", "x"], |y_chirho, x_chirho| {
            primop_chirho(
                "atan2#",
                vec![
                    CoreExprChirho::VarChirho(y_chirho),
                    CoreExprChirho::VarChirho(x_chirho),
                ],
            )
        });
    }

    /// Push `$prim_Floating_<method>_Double = \a b -> body`, typed `Double -> Double ->
    /// Double`, and its guarded Prelude alias `method`.
    fn push_binary_double_row_chirho(
        &mut self,
        method_chirho: &str,
        [first_name_chirho, second_name_chirho]: [&str; 2],
        body_chirho: impl FnOnce(CoreIdChirho, CoreIdChirho) -> CoreExprChirho,
    ) {
        let double_ty_chirho = TyChirho::double_chirho();
        let prim_name_chirho = format!("$prim_Floating_{method_chirho}_Double");
        let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
        let first_chirho = self.fresh_binder_chirho(first_name_chirho, double_ty_chirho.clone());
        let second_chirho = self.fresh_binder_chirho(second_name_chirho, double_ty_chirho.clone());
        let body_chirho = body_chirho(first_chirho.id_chirho, second_chirho.id_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: prim_id_chirho,
                name_chirho: prim_name_chirho.clone(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![double_ty_chirho.clone(), double_ty_chirho.clone()],
                    double_ty_chirho,
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: first_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: second_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        self.push_prelude_alias_chirho(method_chirho, &prim_name_chirho);
    }
}
