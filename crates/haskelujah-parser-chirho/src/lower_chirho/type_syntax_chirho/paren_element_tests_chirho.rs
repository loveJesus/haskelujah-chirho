// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Parenthesized elements that carry `:: k` (T5120, tc218, T12507): every element may be
//! annotated and the annotation survives; a single annotation and the unannotated forms
//! keep their shapes. Shapes ignore parentheses (`type_shape_chirho`).

use haskelujah_ast_chirho::decl_chirho::DeclChirho;

use super::test_support_chirho::{parse_and_lower_chirho, type_shape_chirho};

/// The shape of `binding_chirho`'s signature in a module holding `signature_chirho`.
fn signature_shape_chirho(binding_chirho: &str, signature_chirho: &str) -> String {
    let source_chirho = format!(
        "module ParenElementsChirho where\n\n{binding_chirho} :: {signature_chirho}\n{binding_chirho} = undefined\n"
    );
    parse_and_lower_chirho(&source_chirho)
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } if name_chirho.text_chirho() == binding_chirho => Some(type_shape_chirho(ty_chirho)),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no signature for {binding_chirho}"))
}

#[test]
fn an_implicit_parameter_after_a_comma_keeps_the_whole_qualified_type_chirho() {
    // T5120 and tc218, as written: the signature is the qualified type, never the tuple
    // before the second `::`.
    assert_eq!(
        signature_shape_chirho("b", "(C t, ?x :: TF t) => t"),
        "(C t, ? (?x :: (TF t))) => t"
    );
    assert_eq!(
        signature_shape_chirho("bar", "(Show a, ?c::a) => String"),
        "(Show a, ? (?c :: a)) => String"
    );
}

#[test]
fn every_annotated_tuple_element_survives_chirho() {
    // T12507's argument, then an annotation on a later and on the first element only.
    assert_eq!(
        signature_shape_chirho("record", "Rec (?a :: Int, ?b :: String)"),
        "(Rec ((?a :: Int), (?b :: String)))"
    );
    assert_eq!(
        signature_shape_chirho("pair", "P (a, b :: Type)"),
        "(P (a, (b :: Type)))"
    );
    assert_eq!(
        signature_shape_chirho("pair", "P (a :: Type, b)"),
        "(P ((a :: Type), b))"
    );
}

#[test]
fn a_single_annotation_and_unannotated_groups_keep_their_shapes_chirho() {
    for (signature_chirho, shape_chirho) in [
        ("P (a :: Type)", "(P (a :: Type))"),
        ("(Int, Bool)", "(Int, Bool)"),
        ("(Maybe Int, [a])", "((Maybe Int), [a])"),
        ("(f a) -> (Eq a, Show a)", "((f a) -> ((Eq a), (Show a)))"),
        ("(Eq a, Show a) => a", "(Eq a, Show a) => a"),
        ("forall (a :: Type). a -> a", "forall a. (a -> a)"),
    ] {
        assert_eq!(
            signature_shape_chirho("held", signature_chirho),
            shape_chirho,
            "{signature_chirho}"
        );
    }
}
