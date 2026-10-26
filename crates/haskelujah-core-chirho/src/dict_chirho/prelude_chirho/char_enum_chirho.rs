// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Body-backed `Enum Char` rows. `toEnum` and `fromEnum` convert through the
//! scalar `chr#` and `ord#` representation. `succ` and `pred` change that
//! scalar by one, then restore the Char value. GHC's endpoint errors stay
//! exact. A result in the surrogate range fails by name because the current
//! Rust `char` interpreter representation cannot carry GHC's surrogate Char
//! values. Complete Char range generation remains a separate boundary.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreLitChirho,
    DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho,
};

fn var_chirho(binder_chirho: &BinderChirho) -> CoreExprChirho {
    CoreExprChirho::VarChirho(binder_chirho.id_chirho)
}

fn primop_chirho(name_chirho: &str, args_chirho: Vec<CoreExprChirho>) -> CoreExprChirho {
    CoreExprChirho::PrimOpChirho {
        name_chirho: name_chirho.to_string(),
        args_chirho,
    }
}

fn shifted_scalar_chirho(value_chirho: &BinderChirho, op_chirho: &str) -> CoreExprChirho {
    let scalar_chirho = primop_chirho("ord#", vec![var_chirho(value_chirho)]);
    primop_chirho(
        op_chirho,
        vec![
            scalar_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
        ],
    )
}

fn error_chirho(message_chirho: &str) -> CoreExprChirho {
    primop_chirho(
        "error",
        vec![CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
            message_chirho.to_string(),
        ))],
    )
}

impl DictPassCtxChirho {
    /// Generate the four body-backed `Enum Char` methods. Row 520 supplied
    /// `toEnum` and `fromEnum`; row 567 adds `succ` and `pred`, with GHC's
    /// endpoint errors and a named surrogate-representation boundary.
    pub(super) fn generate_char_enum_rows_chirho(&mut self, int_ty_chirho: &TyChirho) {
        let char_ty_chirho = TyChirho::ConChirho("Char".to_string());

        let tag_chirho = self.fresh_binder_chirho("tag", int_ty_chirho.clone());
        self.push_char_enum_row_chirho(
            "$prim_Enum_toEnum_Char",
            TyChirho::fun_chirho(int_ty_chirho.clone(), char_ty_chirho.clone()),
            tag_chirho.clone(),
            primop_chirho("chr#", vec![var_chirho(&tag_chirho)]),
        );

        let char_chirho = self.fresh_binder_chirho("char", char_ty_chirho.clone());
        self.push_char_enum_row_chirho(
            "$prim_Enum_fromEnum_Char",
            TyChirho::fun_chirho(char_ty_chirho.clone(), int_ty_chirho.clone()),
            char_chirho.clone(),
            primop_chirho("ord#", vec![var_chirho(&char_chirho)]),
        );

        let char_chirho = self.fresh_binder_chirho("char", char_ty_chirho.clone());
        let successor_chirho = self.checked_shifted_char_chirho(
            &char_chirho,
            "+#",
            0x10_FFFF,
            "Prelude.Enum.Char.succ: bad argument",
            &char_ty_chirho,
        );
        self.push_char_enum_row_chirho(
            "$prim_Enum_succ_Char",
            TyChirho::fun_chirho(char_ty_chirho.clone(), char_ty_chirho.clone()),
            char_chirho.clone(),
            successor_chirho,
        );

        let char_chirho = self.fresh_binder_chirho("char", char_ty_chirho.clone());
        let predecessor_chirho = self.checked_shifted_char_chirho(
            &char_chirho,
            "-#",
            0,
            "Prelude.Enum.Char.pred: bad argument",
            &char_ty_chirho,
        );
        self.push_char_enum_row_chirho(
            "$prim_Enum_pred_Char",
            TyChirho::fun_chirho(char_ty_chirho.clone(), char_ty_chirho),
            char_chirho.clone(),
            predecessor_chirho,
        );
    }

    fn checked_shifted_char_chirho(
        &mut self,
        value_chirho: &BinderChirho,
        op_chirho: &str,
        endpoint_chirho: i64,
        endpoint_message_chirho: &str,
        char_ty_chirho: &TyChirho,
    ) -> CoreExprChirho {
        let scalar_chirho = primop_chirho("ord#", vec![var_chirho(value_chirho)]);
        let at_endpoint_chirho = primop_chirho(
            "==#",
            vec![
                scalar_chirho,
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(endpoint_chirho)),
            ],
        );
        let shifted_chirho = shifted_scalar_chirho(value_chirho, op_chirho);
        let converted_chirho = primop_chirho("chr#", vec![shifted_chirho.clone()]);
        let at_or_after_surrogate_chirho = primop_chirho(
            ">=#",
            vec![
                shifted_chirho.clone(),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0xD800)),
            ],
        );
        let at_or_before_surrogate_end_chirho = primop_chirho(
            "<=#",
            vec![
                shifted_chirho,
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0xDFFF)),
            ],
        );
        let surrogate_failure_chirho = error_chirho(
            "Enum Char result is not representable by the current Unicode scalar runtime",
        );
        let inside_surrogate_chirho = self.bool_case_chirho(
            at_or_before_surrogate_end_chirho,
            surrogate_failure_chirho,
            converted_chirho.clone(),
            char_ty_chirho,
        );
        let scalar_result_chirho = self.bool_case_chirho(
            at_or_after_surrogate_chirho,
            inside_surrogate_chirho,
            converted_chirho,
            char_ty_chirho,
        );
        self.bool_case_chirho(
            at_endpoint_chirho,
            error_chirho(endpoint_message_chirho),
            scalar_result_chirho,
            char_ty_chirho,
        )
    }

    fn bool_case_chirho(
        &mut self,
        scrutinee_chirho: CoreExprChirho,
        true_chirho: CoreExprChirho,
        false_chirho: CoreExprChirho,
        result_ty_chirho: &TyChirho,
    ) -> CoreExprChirho {
        let case_chirho = self.fresh_binder_chirho("$char_enum_case", TyChirho::bool_chirho());
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(scrutinee_chirho),
            bind_chirho: case_chirho,
            result_ty_chirho: result_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: true_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: false_chirho,
                },
            ],
        }
    }

    fn push_char_enum_row_chirho(
        &mut self,
        name_chirho: &str,
        ty_chirho: TyChirho,
        parameter_chirho: BinderChirho,
        body_chirho: CoreExprChirho,
    ) {
        let id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho,
                name_chirho: name_chirho.to_string(),
                ty_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: parameter_chirho,
                body_chirho: Box::new(body_chirho),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
