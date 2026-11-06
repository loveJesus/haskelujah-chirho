// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared helpers of the inference tests.
//! Moved from `infer_chirho.rs` in row 661.

use super::*;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

/// Synthetic data declarations used by low-level family/normalization fixtures.
/// Production learns these facts from source declarations, not arbitrary Con strings.
pub(super) fn family_fixture_context_chirho() -> InferCtxChirho {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let declarations_chirho = [
        "Product", "U1", "GRep", "Rep1", "STGen", "STGenM", "StdGen", "S0",
    ]
    .into_iter()
    .map(|name_chirho| (name_chirho.to_owned(), true))
    .collect::<Vec<_>>();
    context_chirho
        .type_families_chirho
        .constructor_origins_mut_chirho()
        .replace_source_module_chirho("FamilyFixtureChirho", &declarations_chirho)
        .unwrap();
    context_chirho
}

pub(super) fn bind_family_fixture_alias_chirho(
    context_chirho: &mut InferCtxChirho,
    bare_chirho: &str,
    alias_chirho: &str,
) {
    let key_chirho = context_chirho.family_name_key_chirho(bare_chirho).unwrap();
    let identity_chirho = context_chirho
        .type_families_chirho
        .definition_chirho(&key_chirho)
        .unwrap()
        .id_chirho()
        .clone();
    context_chirho
        .type_families_chirho
        .export_alias_chirho(alias_chirho.to_owned(), &identity_chirho)
        .unwrap();
    let mut scope_chirho = context_chirho
        .family_scope_chirho
        .as_deref()
        .cloned()
        .unwrap();
    scope_chirho.import_export_chirho(
        alias_chirho.to_owned(),
        alias_chirho,
        &context_chirho.type_families_chirho,
    );
    context_chirho.family_scope_chirho = Some(Arc::new(scope_chirho));
}

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
        instance_context_chirho: None,
        resolved_entry_chirho: None,
    }
}

pub(super) fn rank_n_identity_consumer_ty_chirho() -> TyChirho {
    let identity_var_chirho = TyVarChirho(9006);
    TyChirho::fun_chirho(
        TyChirho::ForallChirho {
            vars_chirho: vec![identity_var_chirho].into(),
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
