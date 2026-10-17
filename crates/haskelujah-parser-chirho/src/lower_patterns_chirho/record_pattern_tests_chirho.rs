// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Assigned record-field atoms must survive lowering as patterns, rather than
//! becoming punned field variables that accept every value.

use super::super::lower_module_chirho;
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::FileIdChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};

fn has_error_node_chirho(node_chirho: &GreenNodeChirho) -> bool {
    node_chirho.kind_chirho() == SyntaxKindChirho::ErrorNodeChirho
        || node_chirho.children_chirho().iter().any(|child_chirho| {
            matches!(child_chirho, GreenElementChirho::NodeChirho(inner_chirho) if has_error_node_chirho(inner_chirho))
        })
}

fn unparenthesized_chirho(pattern_chirho: &PatChirho) -> &PatChirho {
    match pattern_chirho {
        PatChirho::ParenChirho { inner_chirho, .. } => unparenthesized_chirho(inner_chirho),
        _ => pattern_chirho,
    }
}

fn field_pattern_chirho(field_source_chirho: &str) -> PatChirho {
    let source_chirho = format!(
        "{{-# LANGUAGE NamedFieldPuns #-}}\nmodule MChirho where\nfChirho RChirho {{ {field_source_chirho} }} = ()\n"
    );
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(&source_chirho, file_chirho);
    assert!(!has_error_node_chirho(&cst_chirho), "{source_chirho}");
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    let pattern_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { matches_chirho, .. } => {
                matches_chirho.first()?.pats_chirho.first().cloned()
            }
            _ => None,
        })
        .expect("a function parameter");
    let PatChirho::RecordChirho { fields_chirho, .. } = pattern_chirho else {
        panic!("expected a record pattern: {pattern_chirho:?}");
    };
    assert_eq!(fields_chirho.len(), 1);
    fields_chirho[0].pattern_chirho.clone()
}

#[test]
fn assigned_literals_and_wildcards_are_not_field_puns_chirho() {
    assert!(matches!(
        field_pattern_chirho("fieldChirho = 'x'"),
        PatChirho::LitChirho(LitChirho::CharChirho('x', ..))
    ));
    assert!(matches!(
        field_pattern_chirho("fieldChirho = \"hello\""),
        PatChirho::LitChirho(LitChirho::StringChirho(value_chirho, ..)) if value_chirho == "hello"
    ));
    assert!(matches!(
        field_pattern_chirho("fieldChirho = 7"),
        PatChirho::LitChirho(LitChirho::IntChirho(7, ..))
    ));
    assert!(matches!(
        field_pattern_chirho("fieldChirho = 0.5"),
        PatChirho::LitChirho(LitChirho::FloatChirho(value_chirho, ..)) if value_chirho == 0.5
    ));
    assert!(matches!(
        field_pattern_chirho("fieldChirho = _"),
        PatChirho::WildcardChirho(_)
    ));
}

#[test]
fn constructor_arguments_and_real_puns_keep_their_ownership_chirho() {
    for source_chirho in ["fieldChirho = Just 'x'", "fieldChirho = Just ('x')"] {
        let pattern_chirho = field_pattern_chirho(source_chirho);
        let PatChirho::ConChirho {
            con_chirho,
            args_chirho,
            ..
        } = pattern_chirho
        else {
            panic!("expected Just to own its field argument: {pattern_chirho:?}");
        };
        assert_eq!(con_chirho.text_chirho(), "Just");
        assert_eq!(args_chirho.len(), 1);
        assert!(matches!(
            unparenthesized_chirho(&args_chirho[0]),
            PatChirho::LitChirho(LitChirho::CharChirho('x', ..))
        ));
    }
    let PatChirho::ConChirho { args_chirho, .. } = field_pattern_chirho("fieldChirho = Just True")
    else {
        panic!("expected a constructor pattern");
    };
    assert!(
        matches!(args_chirho.as_slice(), [PatChirho::ConChirho { con_chirho, args_chirho, .. }] if con_chirho.text_chirho() == "True" && args_chirho.is_empty())
    );
    assert!(
        matches!(field_pattern_chirho("fieldChirho"), PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "fieldChirho")
    );
    assert!(
        matches!(field_pattern_chirho("fieldChirho = valueChirho"), PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "valueChirho")
    );
}

#[test]
fn nested_pattern_delimiters_belong_to_the_field_pattern_chirho() {
    let PatChirho::ConChirho { args_chirho, .. } =
        field_pattern_chirho("fieldChirho = Just (True, 'x')")
    else {
        panic!("expected Just to own its tuple argument");
    };
    assert_eq!(args_chirho.len(), 1);
    assert!(
        matches!(unparenthesized_chirho(&args_chirho[0]), PatChirho::TupleChirho { elements_chirho, .. } if elements_chirho.len() == 2)
    );
    let PatChirho::RecordChirho { fields_chirho, .. } =
        field_pattern_chirho("fieldChirho = RChirho { fieldChirho = 'x' }")
    else {
        panic!("expected the nested record to own its braces");
    };
    assert!(
        matches!(fields_chirho.as_slice(), [field_chirho] if matches!(field_chirho.pattern_chirho, PatChirho::LitChirho(LitChirho::CharChirho('x', ..))))
    );
}

#[test]
fn expression_tokens_cannot_silently_become_a_record_pattern_chirho() {
    let source_chirho =
        "module MChirho where\nfChirho RChirho { fieldChirho = (toEnum 120) } = ()\n";
    let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(
        source_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
    );
    assert!(has_error_node_chirho(&cst_chirho));
}
