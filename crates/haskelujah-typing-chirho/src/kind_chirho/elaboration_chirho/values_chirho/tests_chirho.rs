// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Failure-recovery controls for the original quote-stage/state continuation.
use super::*;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

#[test]
fn quotation_depth_failure_is_sticky_and_restores_its_original_stage_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: name_chirho("QuoteBudgetChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![],
        extensions_chirho: vec!["TemplateHaskellQuotes".into()],
        inline_pragmas_chirho: Default::default(),
        specialize_pragmas_chirho: Default::default(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };
    let registry_chirho = TypeFamilyEnvChirho::default();
    let type_scope_chirho = TypeConstructorScopeChirho::default();
    let family_scope_chirho = TypeFamilyScopeChirho::default();
    let promotion_scope_chirho = PromotionScopeChirho::default();
    let builtin_classes_chirho = BuiltinClassScopeChirho::default();
    let mut elaborator_chirho = TypeElaboratorChirho::new_chirho(
        &module_chirho,
        ElaborationImportsChirho {
            registry_chirho: &registry_chirho,
            type_scope_chirho: &type_scope_chirho,
            family_scope_chirho: &family_scope_chirho,
            promotion_scope_chirho: &promotion_scope_chirho,
            builtin_classes_chirho: &builtin_classes_chirho,
        },
    )
    .unwrap();
    let mut syntax_chirho = TypeChirho::ConChirho(name_chirho("Int"));
    for _ in 0..130 {
        syntax_chirho = TypeChirho::ParenChirho {
            inner_chirho: Box::new(syntax_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
    }
    let mut quotation_chirho = ExprChirho::QuoteExprChirho {
        typed_chirho: false,
        expr_chirho: Box::new(ExprChirho::AnnChirho {
            expr_chirho: Box::new(ExprChirho::VarChirho(name_chirho("undefined"))),
            ty_chirho: syntax_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let error_chirho = elaborator_chirho
        .expression_chirho(&mut quotation_chirho)
        .expect_err("borrowed quoted syntax uses the original remaining depth");
    assert_eq!(elaborator_chirho.quote_stage_chirho, 0);
    assert_eq!(elaborator_chirho.type_depth_chirho, 0);
    assert!(elaborator_chirho.scope_undo_chirho.is_empty());
    assert_eq!(
        elaborator_chirho.state_chirho.failure_chirho(),
        Some(&error_chirho)
    );
    let mut tiny_chirho = ExprChirho::VarChirho(name_chirho("undefined"));
    assert_eq!(
        elaborator_chirho.expression_chirho(&mut tiny_chirho),
        Err(error_chirho)
    );
}
