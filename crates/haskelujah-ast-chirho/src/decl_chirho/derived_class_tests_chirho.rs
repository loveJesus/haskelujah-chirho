// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The deriving-strategy carrier: which words are strategies, which entries allow the class's
//! own stock code, and the arguments an applied class keeps (row 694).

use haskelujah_span_chirho::SpanChirho;

use super::{DerivedClassChirho, DerivingStrategyChirho};
use crate::name_chirho::{NameChirho, RawNameChirho};
use crate::ty_chirho::TypeChirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

#[test]
fn only_the_three_strategy_keywords_construct_a_strategy_chirho() {
    for keyword_chirho in ["stock", "newtype", "anyclass"] {
        let strategy_chirho =
            DerivingStrategyChirho::from_keyword_chirho(name_chirho(keyword_chirho))
                .unwrap_or_else(|| panic!("`{keyword_chirho}` is a deriving strategy"));
        assert_eq!(
            strategy_chirho.keyword_chirho().text_chirho(),
            keyword_chirho
        );
    }
    // `via` carries a type and keeps its own record; a class name is no strategy.
    for word_chirho in ["via", "Show", "Newtype", ""] {
        assert!(
            DerivingStrategyChirho::from_keyword_chirho(name_chirho(word_chirho)).is_none(),
            "`{word_chirho}` must not be read as a deriving strategy"
        );
    }
}

#[test]
fn each_strategy_answers_exactly_one_predicate_chirho() {
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let answers_chirho = |strategy_chirho: &DerivingStrategyChirho| {
        (
            strategy_chirho.is_stock_chirho(),
            strategy_chirho.is_newtype_chirho(),
            strategy_chirho.is_anyclass_chirho(),
        )
    };
    assert_eq!(
        answers_chirho(&DerivingStrategyChirho::stock_at_chirho(span_chirho)),
        (true, false, false)
    );
    assert_eq!(
        answers_chirho(&DerivingStrategyChirho::newtype_at_chirho(span_chirho)),
        (false, true, false)
    );
    assert_eq!(
        answers_chirho(&DerivingStrategyChirho::anyclass_at_chirho(span_chirho)),
        (false, false, true)
    );
}

#[test]
fn only_unspecified_and_stock_entries_allow_stock_code_chirho() {
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let show_with_chirho =
        |strategy_chirho| DerivedClassChirho::new_chirho(name_chirho("Show"), strategy_chirho);
    assert!(DerivedClassChirho::unspecified_chirho(name_chirho("Show")).allows_stock_chirho());
    assert!(
        show_with_chirho(Some(DerivingStrategyChirho::stock_at_chirho(span_chirho)))
            .allows_stock_chirho()
    );
    // `deriving newtype Show` prints the representation, never the constructor.
    assert!(
        !show_with_chirho(Some(DerivingStrategyChirho::newtype_at_chirho(span_chirho)))
            .allows_stock_chirho()
    );
    assert!(
        !show_with_chirho(Some(DerivingStrategyChirho::anyclass_at_chirho(
            span_chirho
        )))
        .allows_stock_chirho()
    );
}

#[test]
fn an_applied_class_keeps_its_arguments_in_written_order_chirho() {
    let arguments_chirho = vec![
        TypeChirho::ConChirho(name_chirho("Int")),
        TypeChirho::ConChirho(name_chirho("Bool")),
    ];
    let applied_chirho = DerivedClassChirho::applied_chirho(
        name_chirho("TripleChirho"),
        arguments_chirho.clone(),
        None,
    );
    assert!(applied_chirho.has_class_arguments_chirho());
    assert_eq!(applied_chirho.class_arguments_chirho, arguments_chirho);
    assert!(
        !DerivedClassChirho::unspecified_chirho(name_chirho("Show")).has_class_arguments_chirho()
    );
}
