// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared helpers of the inference tests.
//! Moved from `infer_chirho.rs` in row 661.

use super::*;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

pub(super) fn dummy_name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

/// A capture as generated code made it before provenance: no span, no origin.
pub(super) fn unspanned_capture_chirho(
    name_chirho: &str,
    ordinal_chirho: u32,
    class_name_chirho: &str,
    ty_chirho: TyChirho,
) -> evidence_chirho::OccurrenceCaptureChirho {
    evidence_chirho::OccurrenceCaptureChirho {
        name_chirho: name_chirho.to_string(),
        ordinal_chirho,
        class_name_chirho: class_name_chirho.to_string(),
        ty_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_chirho: None,
    }
}

pub(super) fn rank_n_identity_consumer_ty_chirho() -> TyChirho {
    let identity_var_chirho = TyVarChirho(9006);
    TyChirho::fun_chirho(
        TyChirho::ForallChirho {
            vars_chirho: vec![identity_var_chirho],
            body_chirho: Box::new(TyChirho::fun_chirho(
                TyChirho::VarChirho(identity_var_chirho),
                TyChirho::VarChirho(identity_var_chirho),
            )),
        },
        TyChirho::TupleChirho(vec![TyChirho::bool_chirho(), TyChirho::int_chirho()]),
    )
}

pub(super) fn rank_n_identity_consumer_lambda_chirho() -> ExprChirho {
    let function_name_chirho = dummy_name_chirho("functionChirho");
    let apply_function_chirho = |arg_chirho: ExprChirho| ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::VarChirho(function_name_chirho.clone())),
        arg_chirho: Box::new(arg_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    ExprChirho::LamChirho {
        pats_chirho: vec![PatChirho::VarChirho(function_name_chirho.clone())],
        body_chirho: Box::new(ExprChirho::TupleChirho {
            elements_chirho: vec![
                apply_function_chirho(ExprChirho::ConChirho(dummy_name_chirho("True"))),
                apply_function_chirho(ExprChirho::LitChirho(LitChirho::IntChirho(
                    5,
                    SpanChirho::DUMMY_CHIRHO,
                    None,
                ))),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

pub(super) fn type_family_app2_chirho(
    family_chirho: &str,
    left_chirho: TyChirho,
    right_chirho: TyChirho,
) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(family_chirho.to_string())),
            Box::new(left_chirho),
        )),
        Box::new(right_chirho),
    )
}

pub(super) fn type_family_app1_chirho(family_chirho: &str, arg_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho(family_chirho.to_string())),
        Box::new(arg_chirho),
    )
}

pub(super) fn type_lit_chirho(value_chirho: &str) -> TyChirho {
    TyChirho::ConChirho(value_chirho.to_string())
}
