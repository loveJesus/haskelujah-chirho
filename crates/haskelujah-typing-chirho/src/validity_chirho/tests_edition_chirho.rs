// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
use super::*;
use haskelujah_ast_chirho::decl_chirho::TyVarChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

fn mk_name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

/// `f :: (forall a. a -> a) -> Int` under the given extensions.
fn rank_two_is_accepted_with_chirho(extensions_chirho: Vec<String>) -> bool {
    let poly_chirho = TypeChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
        body_chirho: Box::new(TypeChirho::FunChirho {
            arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
            mult_chirho: None,
            result_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let sig_chirho = DeclChirho::TypeSigChirho {
        name_chirho: mk_name_chirho("f"),
        ty_chirho: TypeChirho::FunChirho {
            arg_chirho: Box::new(poly_chirho),
            mult_chirho: None,
            result_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Int"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = ModuleChirho {
        name_chirho: mk_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![sig_chirho],
        extensions_chirho,
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    check_module_type_validity_chirho(&module_chirho).is_empty()
}

// Regression guard: the first version of this check assumed rank-N types
// were off whenever `RankNTypes` was absent. That wrongly rejected 13
// previously-passing GHC should_compile files, because modern GHC compiles
// an undeclared module under GHC2021, which enables RankNTypes — and
// because `Rank2Types` is a legacy spelling of the same extension.
#[test]
fn modern_default_edition_allows_rank_n_chirho() {
    assert!(rank_two_is_accepted_with_chirho(vec![]));
}

#[test]
fn legacy_rank2types_spelling_is_honoured_chirho() {
    assert!(rank_two_is_accepted_with_chirho(vec![
        "Rank2Types".to_string()
    ]));
}

#[test]
fn explicit_haskell2010_still_rejects_rank_n_chirho() {
    assert!(!rank_two_is_accepted_with_chirho(vec![
        "Haskell2010".to_string()
    ]));
}
