// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `toEnum` and `fromEnum` at the integer-represented builtin Enum keys: the
//! sized `Int` and `Word` types, `Word`, `Natural` and the C integer wrappers.
//! Before these rows, every such instance dictionary was filled with
//! missing-method stubs.
//!
//! `toEnum` checks its tag against the key's range and, outside it, fails with
//! GHC 9.14.1's message. A value of each key is an i64 at run time, so
//! `fromEnum` is the identity for the eleven keys whose every value fits Int.
//!
//! The five wide keys keep their missing-method `fromEnum`, which fails loudly:
//! `Word`, `Word64`, `CULong`, `CSize` and `Natural`. GHC fails their values
//! above Int's bounds, and an i64 cannot hold such a value: integer literals
//! parse to i64, and an out-of-range one becomes 0. An identity would therefore
//! answer wrongly as soon as any producer builds such a value.
//!
//! The C wrappers follow the measured aarch64-darwin model, the default target:
//! - `CChar` is `Int8`, `CInt` is `Int32` and `CLong` is `Int64`;
//! - `CUInt` is `Word32`, and `CULong` and `CSize` are `Word64`.
//!
//! GHC names that representation in its message. The compiler has no target ABI
//! map yet, so another target's sizes would be a parameter of this table.
//! workflow: language-features-chirho/integral-enum-rows-chirho

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho,
};

#[cfg(test)]
#[path = "integral_enum_tests_chirho.rs"]
mod integral_enum_tests_chirho;

/// What GHC 9.14.1's `toEnum` does with a tag outside a key's range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OutOfRangeChirho {
    /// Every Int tag is in range (`Int64`, `CLong`).
    UncheckedChirho,
    /// `Enum.toEnum{<shown>}: tag (<n>) is outside of enumeration's range
    /// <range>`. Only the bounds an Int tag can pass are checked.
    BoundsChirho {
        shown_chirho: &'static str,
        lower_chirho: Option<i64>,
        upper_chirho: Option<i64>,
        range_chirho: &'static str,
    },
    /// `Natural`: `toEnum: unexpected negative Int`.
    NegativeChirho,
}

/// One integer-represented key and its `toEnum` range.
struct IntegralKeyChirho {
    key_chirho: &'static str,
    out_of_range_chirho: OutOfRangeChirho,
    /// Every value of the key fits Int, so `fromEnum` is the identity. A wide
    /// key keeps the missing-method `fromEnum` until a value above i64 can be
    /// represented and checked against Int's bounds.
    identity_from_enum_chirho: bool,
}

/// A key with both bounds inside Int's range, so every value fits Int.
const fn bounded_chirho(
    key_chirho: &'static str,
    shown_chirho: &'static str,
    lower_chirho: i64,
    upper_chirho: i64,
    range_chirho: &'static str,
) -> IntegralKeyChirho {
    IntegralKeyChirho {
        key_chirho,
        out_of_range_chirho: OutOfRangeChirho::BoundsChirho {
            shown_chirho,
            lower_chirho: Some(lower_chirho),
            upper_chirho: Some(upper_chirho),
            range_chirho,
        },
        identity_from_enum_chirho: true,
    }
}

/// A key that is Int itself (`Int64`, `CLong`): every tag and value fits.
const fn int_sized_chirho(key_chirho: &'static str) -> IntegralKeyChirho {
    IntegralKeyChirho {
        key_chirho,
        out_of_range_chirho: OutOfRangeChirho::UncheckedChirho,
        identity_from_enum_chirho: true,
    }
}

/// An unsigned 64-bit key: only a negative tag is out of range, and its values
/// can pass Int's upper bound, so it is a wide key.
const fn non_negative_chirho(
    key_chirho: &'static str,
    shown_chirho: &'static str,
) -> IntegralKeyChirho {
    IntegralKeyChirho {
        key_chirho,
        out_of_range_chirho: OutOfRangeChirho::BoundsChirho {
            shown_chirho,
            lower_chirho: Some(0),
            upper_chirho: None,
            range_chirho: "(0,18446744073709551615)",
        },
        identity_from_enum_chirho: false,
    }
}

/// The sixteen keys, measured on GHC 9.14.1 (aarch64-darwin).
const INTEGRAL_KEYS_CHIRHO: &[IntegralKeyChirho] = &[
    bounded_chirho("Int8", "Int8", -128, 127, "(-128,127)"),
    bounded_chirho("Int16", "Int16", -32_768, 32_767, "(-32768,32767)"),
    bounded_chirho(
        "Int32",
        "Int32",
        -2_147_483_648,
        2_147_483_647,
        "(-2147483648,2147483647)",
    ),
    int_sized_chirho("Int64"),
    non_negative_chirho("Word", "Word"),
    bounded_chirho("Word8", "Word8", 0, 255, "(0,255)"),
    bounded_chirho("Word16", "Word16", 0, 65_535, "(0,65535)"),
    bounded_chirho("Word32", "Word32", 0, 4_294_967_295, "(0,4294967295)"),
    non_negative_chirho("Word64", "Word64"),
    IntegralKeyChirho {
        key_chirho: "Natural",
        out_of_range_chirho: OutOfRangeChirho::NegativeChirho,
        identity_from_enum_chirho: false,
    },
    bounded_chirho("CChar", "Int8", -128, 127, "(-128,127)"),
    bounded_chirho(
        "CInt",
        "Int32",
        -2_147_483_648,
        2_147_483_647,
        "(-2147483648,2147483647)",
    ),
    int_sized_chirho("CLong"),
    bounded_chirho("CUInt", "Word32", 0, 4_294_967_295, "(0,4294967295)"),
    non_negative_chirho("CULong", "Word64"),
    non_negative_chirho("CSize", "Word64"),
];

/// `left ++# right`.
fn append_chirho(left_chirho: CoreExprChirho, right_chirho: CoreExprChirho) -> CoreExprChirho {
    CoreExprChirho::PrimOpChirho {
        name_chirho: "++#".to_string(),
        args_chirho: vec![left_chirho, right_chirho],
    }
}

/// A failure carrying `message_chirho`, which may be computed.
fn fail_with_chirho(message_chirho: CoreExprChirho) -> CoreExprChirho {
    CoreExprChirho::PrimOpChirho {
        name_chirho: "error".to_string(),
        args_chirho: vec![message_chirho],
    }
}

/// GHC's out-of-range message for one tag:
/// `Enum.toEnum{Word8}: tag (300) is outside of enumeration's range (0,255)`.
fn bad_tag_chirho(
    tag_chirho: CoreIdChirho,
    shown_chirho: &str,
    range_chirho: &str,
) -> CoreExprChirho {
    let literal_chirho =
        |text_chirho: String| CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(text_chirho));
    let shown_tag_chirho = CoreExprChirho::PrimOpChirho {
        name_chirho: "showInt#".to_string(),
        args_chirho: vec![CoreExprChirho::VarChirho(tag_chirho)],
    };
    fail_with_chirho(append_chirho(
        literal_chirho(format!("Enum.toEnum{{{shown_chirho}}}: tag (")),
        append_chirho(
            shown_tag_chirho,
            literal_chirho(format!(
                ") is outside of enumeration's range {range_chirho}"
            )),
        ),
    ))
}

impl DictPassCtxChirho {
    /// `$prim_Enum_toEnum_<Key>` for every integer-represented key, and
    /// `$prim_Enum_fromEnum_<Key>` for each key whose values fit Int. They are
    /// generated before the instance dictionaries, which then take them instead
    /// of missing-method stubs.
    pub(super) fn generate_integral_enum_rows_chirho(&mut self, int_ty_chirho: &TyChirho) {
        for integral_chirho in INTEGRAL_KEYS_CHIRHO {
            let key_ty_chirho = TyChirho::ConChirho(integral_chirho.key_chirho.to_string());

            let to_name_chirho = format!("$prim_Enum_toEnum_{}", integral_chirho.key_chirho);
            let to_id_chirho = self.resolve_or_fresh_id_chirho(&to_name_chirho);
            let tag_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let checked_chirho = self.checked_integral_tag_chirho(
                tag_chirho.id_chirho,
                integral_chirho.out_of_range_chirho,
                &key_ty_chirho,
            );
            self.push_integral_enum_row_chirho(
                (to_id_chirho, to_name_chirho),
                TyChirho::fun_chirho(int_ty_chirho.clone(), key_ty_chirho.clone()),
                tag_chirho,
                checked_chirho,
            );

            // A wide key gets no row: its dictionary keeps the missing-method
            // `fromEnum`, loud rather than wrong for a value past Int.
            if !integral_chirho.identity_from_enum_chirho {
                continue;
            }
            let from_name_chirho = format!("$prim_Enum_fromEnum_{}", integral_chirho.key_chirho);
            let from_id_chirho = self.resolve_or_fresh_id_chirho(&from_name_chirho);
            let value_chirho = self.fresh_binder_chirho("x", key_ty_chirho.clone());
            let identity_chirho = CoreExprChirho::VarChirho(value_chirho.id_chirho);
            self.push_integral_enum_row_chirho(
                (from_id_chirho, from_name_chirho),
                TyChirho::fun_chirho(key_ty_chirho, int_ty_chirho.clone()),
                value_chirho,
                identity_chirho,
            );
        }
    }

    /// `\param -> body`, bound at top level under the row's name.
    fn push_integral_enum_row_chirho(
        &mut self,
        (id_chirho, name_chirho): (CoreIdChirho, String),
        ty_chirho: TyChirho,
        param_chirho: BinderChirho,
        body_chirho: CoreExprChirho,
    ) {
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho,
                name_chirho,
                ty_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: param_chirho,
                body_chirho: Box::new(body_chirho),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }

    /// The tag itself when it is in range, else GHC's failure. The lower bound
    /// is tested first.
    fn checked_integral_tag_chirho(
        &mut self,
        tag_chirho: CoreIdChirho,
        out_of_range_chirho: OutOfRangeChirho,
        key_ty_chirho: &TyChirho,
    ) -> CoreExprChirho {
        let in_range_chirho = CoreExprChirho::VarChirho(tag_chirho);
        match out_of_range_chirho {
            OutOfRangeChirho::UncheckedChirho => in_range_chirho,
            OutOfRangeChirho::NegativeChirho => {
                let negative_chirho = fail_with_chirho(CoreExprChirho::LitChirho(
                    CoreLitChirho::StringChirho("toEnum: unexpected negative Int".to_string()),
                ));
                self.integral_tag_case_chirho(
                    ("<#", tag_chirho, 0),
                    negative_chirho,
                    in_range_chirho,
                    key_ty_chirho,
                )
            }
            OutOfRangeChirho::BoundsChirho {
                shown_chirho,
                lower_chirho,
                upper_chirho,
                range_chirho,
            } => {
                let mut checked_chirho = in_range_chirho;
                if let Some(upper_chirho) = upper_chirho {
                    checked_chirho = self.integral_tag_case_chirho(
                        (">#", tag_chirho, upper_chirho),
                        bad_tag_chirho(tag_chirho, shown_chirho, range_chirho),
                        checked_chirho,
                        key_ty_chirho,
                    );
                }
                if let Some(lower_chirho) = lower_chirho {
                    checked_chirho = self.integral_tag_case_chirho(
                        ("<#", tag_chirho, lower_chirho),
                        bad_tag_chirho(tag_chirho, shown_chirho, range_chirho),
                        checked_chirho,
                        key_ty_chirho,
                    );
                }
                checked_chirho
            }
        }
    }

    /// `case tag `op` bound of True -> then; _ -> otherwise`.
    fn integral_tag_case_chirho(
        &mut self,
        (op_chirho, tag_chirho, bound_chirho): (&str, CoreIdChirho, i64),
        then_chirho: CoreExprChirho,
        otherwise_chirho: CoreExprChirho,
        result_ty_chirho: &TyChirho,
    ) -> CoreExprChirho {
        let compared_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: op_chirho.to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(tag_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(bound_chirho)),
                ],
            }),
            bind_chirho: compared_chirho,
            result_ty_chirho: result_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: then_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: otherwise_chirho,
                },
            ],
        }
    }
}
