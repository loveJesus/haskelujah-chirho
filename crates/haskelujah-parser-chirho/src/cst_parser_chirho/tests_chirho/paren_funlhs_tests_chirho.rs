// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A parenthesized function left side, Haskell 2010's `( funlhs ) apat { apat }` (row 618). The
//! parser lays it out as one flat run in the Match: `(`, the left operand, the operator, the
//! right operand and `)`, or a nested one and its arguments, then the arguments. Lowering names
//! the binding by the operator after the first operand and takes every pattern in order. Before
//! this, `(f . g) x = g (f x)` broke into error nodes and a binding of `x`, and `.` stayed
//! unbound. A constructor operator or a tuple at the head, or no argument after the `)`, still
//! makes a pattern binding, as in GHC 9.14.1.
//! workflow: language-features-chirho/pattern-matching-chirho

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_span_chirho::FileIdChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};

use super::ParserChirho;
use crate::lower_chirho::lower_module_chirho;

/// Each definition, the kinds of its Match before `=` with trivia left out, the operator it
/// defines, and how many patterns its equation takes.
const DEFINITIONS_CHIRHO: [(&str, &[&str], &str, usize); 4] = [
    (
        "(f . g) x = g (f x)\n",
        &[
            "LeftParenChirho",
            "VarPatChirho",
            "VarSymChirho",
            "VarPatChirho",
            "RightParenChirho",
            "VarPatChirho",
        ],
        ".",
        3,
    ),
    (
        "((f <%> g) x) y = f x + g y\n",
        &[
            "LeftParenChirho",
            "LeftParenChirho",
            "VarPatChirho",
            "VarSymChirho",
            "VarPatChirho",
            "RightParenChirho",
            "VarPatChirho",
            "RightParenChirho",
            "VarPatChirho",
        ],
        "<%>",
        4,
    ),
    (
        "(f `andThen` g) x = g (f x)\n",
        &[
            "LeftParenChirho",
            "VarPatChirho",
            "BacktickChirho",
            "VarIdChirho",
            "BacktickChirho",
            "VarPatChirho",
            "RightParenChirho",
            "VarPatChirho",
        ],
        "andThen",
        3,
    ),
    (
        "(Just a <?> b) x = a + b + x\n",
        &[
            "LeftParenChirho",
            "ConPatChirho",
            "VarSymChirho",
            "VarPatChirho",
            "RightParenChirho",
            "VarPatChirho",
        ],
        "<?>",
        3,
    ),
];

fn first_node_chirho(
    node_chirho: &GreenNodeChirho,
    kind_chirho: SyntaxKindChirho,
) -> Option<&GreenNodeChirho> {
    if node_chirho.kind_chirho() == kind_chirho {
        return Some(node_chirho);
    }
    node_chirho
        .children_chirho()
        .iter()
        .find_map(|child_chirho| match child_chirho {
            GreenElementChirho::NodeChirho(inner_chirho) => {
                first_node_chirho(inner_chirho, kind_chirho)
            }
            GreenElementChirho::TokenChirho(_) => None,
        })
}

/// The kinds of the first Match's children before `=`, trivia left out.
fn match_shape_chirho(source_chirho: &str) -> Vec<String> {
    let green_chirho =
        ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO).parse_chirho();
    let match_chirho = first_node_chirho(&green_chirho, SyntaxKindChirho::MatchChirho)
        .unwrap_or_else(|| panic!("no Match in {source_chirho:?}"));
    match_chirho
        .children_chirho()
        .iter()
        .filter_map(|child_chirho| match child_chirho {
            GreenElementChirho::TokenChirho(token_chirho) => {
                let kind_chirho = format!("{:?}", token_chirho.kind_chirho());
                (!kind_chirho.ends_with("TriviaChirho")).then_some(kind_chirho)
            }
            GreenElementChirho::NodeChirho(node_chirho) => {
                Some(format!("{:?}", node_chirho.kind_chirho()))
            }
        })
        .take_while(|kind_chirho| kind_chirho != "EqualsChirho")
        .collect()
}

/// Every function binding the source lowers to: its name and its first equation's pattern count.
fn function_bindings_chirho(source_chirho: &str) -> Vec<(String, usize)> {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let green_chirho = ParserChirho::new_chirho(source_chirho, file_chirho).parse_chirho();
    lower_module_chirho(&green_chirho, file_chirho)
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } => Some((
                name_chirho.text_chirho().to_string(),
                matches_chirho
                    .first()
                    .map_or(0, |arm_chirho| arm_chirho.pats_chirho.len()),
            )),
            _ => None,
        })
        .collect()
}

#[test]
fn a_parenthesized_funlhs_is_one_flat_run_that_lowers_to_the_operator_chirho() {
    for (source_chirho, shape_chirho, name_chirho, arity_chirho) in DEFINITIONS_CHIRHO {
        assert_eq!(
            match_shape_chirho(source_chirho),
            shape_chirho,
            "{source_chirho:?}"
        );
        assert_eq!(
            function_bindings_chirho(source_chirho),
            [(name_chirho.to_string(), arity_chirho)],
            "{source_chirho:?}"
        );
    }
}

#[test]
fn no_argument_a_constructor_operator_or_a_tuple_never_defines_the_operator_chirho() {
    // GHC rejects the first ("Parse error in pattern"); it stays on the pattern-binding path,
    // whose error recovery binds no `<+>`. The other two are pattern bindings.
    for (source_chirho, operator_chirho) in [
        ("(a <+> b) = a * 10 + b\n", "<+>"),
        ("(x:_) = [5, 6]\n", ":"),
        ("(a, b) = (1, 2)\n", ","),
    ] {
        assert!(
            function_bindings_chirho(source_chirho)
                .iter()
                .all(|(name_chirho, _)| name_chirho != operator_chirho),
            "{source_chirho:?}"
        );
    }
}
