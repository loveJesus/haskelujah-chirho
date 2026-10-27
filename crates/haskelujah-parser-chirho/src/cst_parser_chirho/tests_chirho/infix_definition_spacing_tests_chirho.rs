// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC's whitespace rule for `!` and `-` after a pattern (row 617). Only a prefix occurrence,
//! loose on the left and tight on the right, starts an argument pattern: a bang pattern, or a
//! negative literal. Every other `!` or `-` is the operator an infix definition defines. The
//! comment cases are GHC's own `proposal-229e` test. Before this, a `!` or `-` after a name
//! head was always an argument pattern, so `x ! y = ...` defined `x`.
//! workflow: language-features-chirho/pattern-matching-chirho

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_span_chirho::FileIdChirho;

use super::ParserChirho;
use crate::lower_chirho::lower_module_chirho;

/// Each declaration, and the name GHC 9.14.1 reads it as defining.
const DEFINITIONS_CHIRHO: [(&str, &str); 13] = [
    ("x ! y = x\n", "!"),
    ("a!b = a\n", "!"),
    ("a! b = a\n", "!"),
    ("f !x = x\n", "f"),
    // A `}` that is not a comment's closes a token, so the `!` is tight infix.
    ("Nothing{}!x = x\n", "!"),
    // A `{-` opens no token, so the `!` is loose.
    ("Just a !{-comment-}x = a\n", "!"),
    // A comment's `-}` closes no token, so the `!` is prefix.
    ("f{-comment-}!x = x\n", "f"),
    ("a - b = a\n", "-"),
    ("a-b = a\n", "-"),
    ("a -b = a\n", "-"),
    ("g -1 y = y\n", "g"),
    ("V a - V b = a\n", "-"),
    ("Box a ! i = a\n", "!"),
];

fn defined_name_chirho(source_chirho: &str) -> String {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let green_chirho = ParserChirho::new_chirho(source_chirho, file_chirho).parse_chirho();
    lower_module_chirho(&green_chirho, file_chirho)
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("no function binding in {source_chirho:?}"))
}

#[test]
fn spacing_decides_between_an_operator_and_an_argument_pattern_chirho() {
    for (source_chirho, name_chirho) in DEFINITIONS_CHIRHO {
        assert_eq!(
            defined_name_chirho(source_chirho),
            name_chirho,
            "{source_chirho:?}"
        );
    }
}

#[test]
fn prefix_bangs_in_lambdas_and_constructor_arguments_stay_patterns_chirho() {
    for source_chirho in ["f = \\ !x !y -> x\n", "f m = case m of Just !x -> x\n"] {
        let tree_chirho = format!(
            "{:?}",
            ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO).parse_chirho()
        );
        assert!(tree_chirho.contains("BangPatChirho"), "{source_chirho:?}");
        assert!(
            !tree_chirho.contains("ErrorNodeChirho"),
            "{source_chirho:?}"
        );
    }
}
