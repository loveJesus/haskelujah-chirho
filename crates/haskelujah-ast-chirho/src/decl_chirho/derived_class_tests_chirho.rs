// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The deriving-strategy carrier: which words are strategies, and which entries take the
//! class's own stock code (row 694).

use haskelujah_span_chirho::SpanChirho;

use super::{DerivedClassChirho, DerivingStrategyChirho};
use crate::name_chirho::{NameChirho, RawNameChirho};

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
fn only_unspecified_and_stock_entries_name_a_stock_class_chirho() {
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let show_with_chirho =
        |strategy_chirho| DerivedClassChirho::new_chirho(name_chirho("Show"), strategy_chirho);
    assert!(
        DerivedClassChirho::unspecified_chirho(name_chirho("Show"))
            .names_stock_class_chirho("Show")
    );
    assert!(
        show_with_chirho(Some(DerivingStrategyChirho::stock_at_chirho(span_chirho)))
            .names_stock_class_chirho("Show")
    );
    // `deriving newtype Show` prints the representation, never the constructor.
    assert!(
        !show_with_chirho(Some(DerivingStrategyChirho::newtype_at_chirho(span_chirho)))
            .names_stock_class_chirho("Show")
    );
    assert!(
        !show_with_chirho(Some(DerivingStrategyChirho::anyclass_at_chirho(
            span_chirho
        )))
        .names_stock_class_chirho("Show")
    );
    assert!(!show_with_chirho(None).names_stock_class_chirho("Eq"));
}
