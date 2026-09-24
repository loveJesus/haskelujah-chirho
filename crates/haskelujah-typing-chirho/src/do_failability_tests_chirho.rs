// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The failability rule against GHC 9.14.1, case for case.
//!
//! Each case below was RUN against GHC first, as `f m = do { p <- m; return 0 }`
//! in a Monad deliberately given no MonadFail instance: GHC accepting the
//! program means the pattern is irrefutable, and GHC demanding MonadFail means
//! it can fail. The seventeen results are reproduced here so the rule cannot
//! drift away from the compiler it is copying.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, StrictnessChirho};
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::{PatChirho, PatFieldChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

use crate::do_failability_chirho::pattern_can_fail_chirho;
use crate::exhaust_chirho::TypeConEnvChirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn data_decl_chirho(type_chirho: &str, cons_chirho: &[(&str, usize)]) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: name_chirho(type_chirho),
        type_vars_chirho: vec![],
        constructors_chirho: cons_chirho
            .iter()
            .map(|(con_chirho, arity_chirho)| ConDeclChirho::OrdinaryChirho {
                name_chirho: name_chirho(con_chirho),
                fields_chirho: (0..*arity_chirho)
                    .map(|_| {
                        (
                            StrictnessChirho::LazyChirho,
                            TypeChirho::ConChirho(name_chirho("Int")),
                        )
                    })
                    .collect(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            })
            .collect(),
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `WrapChirho` has one constructor, `OneChirho` one with two fields, and
/// `TwoChirho` two — the three shapes the GHC harness declared.
fn env_chirho() -> TypeConEnvChirho {
    let module_chirho = ModuleChirho {
        name_chirho: name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            data_decl_chirho("WrapChirho", &[("WrapChirho", 1)]),
            data_decl_chirho("OneChirho", &[("OneChirho", 2)]),
            data_decl_chirho("TwoChirho", &[("TwoAChirho", 1), ("TwoBChirho", 0)]),
        ],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };
    TypeConEnvChirho::from_module_chirho(&module_chirho)
}

fn var_chirho() -> PatChirho {
    PatChirho::VarChirho(name_chirho("x"))
}

fn wildcard_chirho() -> PatChirho {
    PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)
}

fn literal_chirho() -> PatChirho {
    PatChirho::LitChirho(LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO))
}

fn tuple_chirho(elements_chirho: Vec<PatChirho>) -> PatChirho {
    PatChirho::TupleChirho {
        elements_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn con_chirho(con_text_chirho: &str, args_chirho: Vec<PatChirho>) -> PatChirho {
    PatChirho::ConChirho {
        con_chirho: name_chirho(con_text_chirho),
        args_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn lazy_chirho(inner_chirho: PatChirho) -> PatChirho {
    PatChirho::LazyChirho {
        inner_chirho: Box::new(inner_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn bang_chirho(inner_chirho: PatChirho) -> PatChirho {
    PatChirho::BangChirho {
        inner_chirho: Box::new(inner_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn as_chirho(pattern_chirho: PatChirho) -> PatChirho {
    PatChirho::AsChirho {
        name_chirho: name_chirho("p"),
        pattern_chirho: Box::new(pattern_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn view_chirho(pat_chirho: PatChirho) -> PatChirho {
    PatChirho::ViewChirho {
        expr_chirho: Box::new(ExprChirho::VarChirho(name_chirho("id"))),
        pat_chirho: Box::new(pat_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn list_chirho(elements_chirho: Vec<PatChirho>) -> PatChirho {
    PatChirho::ListChirho {
        elements_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `pattern_can_fail_chirho` must answer exactly what GHC answered.
fn assert_matches_ghc_chirho(case_chirho: &str, pat_chirho: PatChirho, ghc_can_fail_chirho: bool) {
    assert_eq!(
        pattern_can_fail_chirho(&pat_chirho, &env_chirho()),
        ghc_can_fail_chirho,
        "{case_chirho}"
    );
}

#[test]
fn irrefutable_patterns_select_no_fail_chirho() {
    // GHC accepted every one of these in a Monad with no MonadFail instance.
    assert_matches_ghc_chirho("variable", var_chirho(), false);
    assert_matches_ghc_chirho("wildcard", wildcard_chirho(), false);
    assert_matches_ghc_chirho("tuple", tuple_chirho(vec![var_chirho(), var_chirho()]), false);
    assert_matches_ghc_chirho(
        "nested tuple",
        tuple_chirho(vec![
            var_chirho(),
            tuple_chirho(vec![var_chirho(), var_chirho()]),
        ]),
        false,
    );
    assert_matches_ghc_chirho("lazy", lazy_chirho(var_chirho()), false);
    assert_matches_ghc_chirho("bang variable", bang_chirho(var_chirho()), false);
    assert_matches_ghc_chirho(
        "newtype-shaped sole constructor",
        con_chirho("WrapChirho", vec![var_chirho()]),
        false,
    );
    assert_matches_ghc_chirho(
        "single-constructor data",
        con_chirho("OneChirho", vec![var_chirho(), var_chirho()]),
        false,
    );
    assert_matches_ghc_chirho(
        "as-pattern over a tuple",
        as_chirho(tuple_chirho(vec![var_chirho(), var_chirho()])),
        false,
    );
    assert_matches_ghc_chirho("view onto a variable", view_chirho(var_chirho()), false);
}

#[test]
fn a_lazy_pattern_is_irrefutable_even_when_its_inner_pattern_is_not_chirho() {
    // The case most likely to be got wrong. GHC accepts `~(Just x) <- m` in a
    // Monad with no MonadFail: the lazy pattern defers the match, so the bind
    // cannot fail however refutable the pattern inside it is.
    assert_matches_ghc_chirho(
        "lazy over a refutable pattern",
        lazy_chirho(con_chirho("TwoAChirho", vec![var_chirho()])),
        false,
    );
    // Without the `~`, the same pattern does select `fail`.
    assert_matches_ghc_chirho(
        "the same pattern, not lazy",
        con_chirho("TwoAChirho", vec![var_chirho()]),
        true,
    );
}

#[test]
fn failable_patterns_select_fail_chirho() {
    // GHC demanded MonadFail for every one of these.
    assert_matches_ghc_chirho(
        "multi-constructor data",
        con_chirho("TwoBChirho", vec![]),
        true,
    );
    assert_matches_ghc_chirho("literal", literal_chirho(), true);
    assert_matches_ghc_chirho("empty list", list_chirho(vec![]), true);
    assert_matches_ghc_chirho("view onto a literal", view_chirho(literal_chirho()), true);
    assert_matches_ghc_chirho(
        "tuple with a literal in it",
        tuple_chirho(vec![literal_chirho(), var_chirho()]),
        true,
    );
}

#[test]
fn an_unknown_constructor_is_treated_as_failable_chirho() {
    // A constructor from a module this environment does not have is not proved
    // sole, so it keeps the `fail` the desugarer already emitted for it. The
    // safe direction: a missing `fail` leaves a failing match nowhere to go.
    assert_matches_ghc_chirho(
        "constructor the environment never saw",
        con_chirho("FromElsewhereChirho", vec![var_chirho()]),
        true,
    );
}

#[test]
fn a_sole_constructor_still_fails_through_a_refutable_field_chirho() {
    // `OneChirho 0 b <- m` cannot be reached by constructor mismatch, but the
    // literal inside it can still fail, so the statement selects `fail`.
    assert_matches_ghc_chirho(
        "sole constructor with a literal field",
        con_chirho("OneChirho", vec![literal_chirho(), var_chirho()]),
        true,
    );
    assert_matches_ghc_chirho(
        "sole constructor, record form, literal field",
        PatChirho::RecordChirho {
            con_chirho: name_chirho("OneChirho"),
            fields_chirho: vec![PatFieldChirho {
                name_chirho: name_chirho("fieldChirho"),
                pattern_chirho: literal_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            has_wildcard_chirho: false,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        true,
    );
}
