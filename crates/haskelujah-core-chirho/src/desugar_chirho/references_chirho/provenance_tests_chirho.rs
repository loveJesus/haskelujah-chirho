// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The desugarer copies each use's origin onto the occurrence id it mints, with
//! the role of that use, and never mints an origin of its own: a variable, an
//! infix operator and an integer literal each come out carrying the provenance
//! the producer put on the AST node.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::provenance_chirho::{
    OccurrenceRoleChirho, OriginSupplyChirho, ProvenanceChirho,
};
use haskelujah_span_chirho::SpanChirho;

use super::DesugarCtxChirho;
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho};

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

#[test]
fn each_minted_occurrence_carries_its_uses_origin_and_role_chirho() {
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let show_chirho = supply_chirho.fresh_chirho();
    let plus_chirho = supply_chirho.fresh_chirho();
    let five_chirho = supply_chirho.fresh_chirho();
    let minted_before_chirho = supply_chirho.minted_chirho();
    // show (x + 5), every occurrence under the placeholder span.
    let expr_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::VarChirho(
            name_chirho("show").with_origin_chirho(show_chirho),
        )),
        arg_chirho: Box::new(ExprChirho::InfixChirho {
            left_chirho: Box::new(ExprChirho::VarChirho(name_chirho("x"))),
            op_chirho: name_chirho("+").with_origin_chirho(plus_chirho),
            right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                5,
                SpanChirho::DUMMY_CHIRHO,
                Some(five_chirho),
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    ctx_chirho.set_method_occurrence_names_chirho(HashSet::from([
        "show".to_string(),
        "+".to_string(),
        "fromInteger".to_string(),
    ]));
    let _core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

    let by_name_chirho: HashMap<&str, ProvenanceChirho> = ctx_chirho
        .occurrence_provenance_chirho
        .iter()
        .map(|(id_chirho, provenance_chirho)| {
            let (name_chirho, _canonical_chirho) = &ctx_chirho.method_occurrences_chirho[id_chirho];
            (name_chirho.as_str(), *provenance_chirho)
        })
        .collect();
    assert_eq!(
        by_name_chirho.get("show"),
        Some(&ProvenanceChirho::new_chirho(
            show_chirho,
            OccurrenceRoleChirho::Reference
        ))
    );
    assert_eq!(
        by_name_chirho.get("+"),
        Some(&ProvenanceChirho::new_chirho(
            plus_chirho,
            OccurrenceRoleChirho::Reference
        ))
    );
    assert_eq!(
        by_name_chirho.get("fromInteger"),
        Some(&ProvenanceChirho::new_chirho(
            five_chirho,
            OccurrenceRoleChirho::IntegerLiteral
        ))
    );
    // The desugarer read origins; it minted none.
    assert_eq!(supply_chirho.minted_chirho(), minted_before_chirho);
}

/// The three literal forms row 660 covers, each with the extension that makes it a method call:
/// the generated method's name, the literal, and the provenance it records.
fn literal_forms_chirho(
    supply_chirho: &mut OriginSupplyChirho,
) -> Vec<(
    Option<&'static str>,
    &'static str,
    ExprChirho,
    Option<ProvenanceChirho>,
)> {
    let integer_chirho = supply_chirho.fresh_chirho();
    let string_chirho = supply_chirho.fresh_chirho();
    vec![
        (
            None,
            "fromInteger",
            ExprChirho::LitChirho(LitChirho::IntChirho(
                5,
                SpanChirho::DUMMY_CHIRHO,
                Some(integer_chirho),
            )),
            Some(ProvenanceChirho::new_chirho(
                integer_chirho,
                OccurrenceRoleChirho::IntegerLiteral,
            )),
        ),
        (
            Some("OverloadedStrings"),
            "fromString",
            ExprChirho::LitChirho(LitChirho::StringChirho(
                "literal".to_string(),
                SpanChirho::DUMMY_CHIRHO,
                Some(string_chirho),
            )),
            Some(ProvenanceChirho::new_chirho(
                string_chirho,
                OccurrenceRoleChirho::StringLiteral,
            )),
        ),
        (
            Some("OverloadedLists"),
            "fromList",
            ExprChirho::ListChirho {
                elements_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            None,
        ),
    ]
}

/// The id the literal's desugaring applies to its value.
fn applied_method_id_chirho(core_chirho: &CoreExprChirho) -> CoreIdChirho {
    match core_chirho {
        CoreExprChirho::AppChirho { fun_chirho, .. } => match fun_chirho.as_ref() {
            CoreExprChirho::VarChirho(id_chirho) => *id_chirho,
            other_chirho => panic!("expected the method applied by id, got {other_chirho:?}"),
        },
        other_chirho => panic!("expected the literal applied to its method, got {other_chirho:?}"),
    }
}

/// A literal's generated method takes an occurrence of its own past the program's binding of
/// that name, with the literal's provenance recorded on it, while the program's written
/// reference keeps the program's binding (row 660).
#[test]
fn a_literals_generated_method_passes_the_programs_own_binding_chirho() {
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    for (extension_chirho, method_chirho, literal_chirho, provenance_chirho) in
        literal_forms_chirho(&mut supply_chirho)
    {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        ctx_chirho.extensions_chirho = extension_chirho.into_iter().map(String::from).collect();
        let own_chirho = ctx_chirho.fresh_id_chirho(method_chirho);
        ctx_chirho.bind_in_scope_chirho(method_chirho, own_chirho);

        let core_chirho = ctx_chirho.desugar_expr_chirho(&literal_chirho);
        let generated_chirho = applied_method_id_chirho(&core_chirho);
        assert_ne!(
            generated_chirho, own_chirho,
            "{method_chirho}: the literal took the program's binding"
        );
        let (occurrence_name_chirho, canonical_chirho) =
            &ctx_chirho.method_occurrences_chirho[&generated_chirho];
        assert_eq!(occurrence_name_chirho, method_chirho);
        assert_ne!(
            *canonical_chirho, own_chirho,
            "{method_chirho}: the fallback is the program's binding"
        );
        assert_eq!(
            ctx_chirho
                .occurrence_provenance_chirho
                .get(&generated_chirho)
                .copied(),
            provenance_chirho,
            "{method_chirho}: the literal's provenance"
        );

        let written_chirho =
            ctx_chirho.desugar_expr_chirho(&ExprChirho::VarChirho(name_chirho(method_chirho)));
        assert!(
            matches!(written_chirho, CoreExprChirho::VarChirho(id_chirho) if id_chirho == own_chirho),
            "{method_chirho}: a written reference must keep the program's binding, got {written_chirho:?}"
        );
    }
}

/// Under RebindableSyntax a literal's method is whatever binding is in scope, as GHC defines
/// (row 660).
#[test]
fn under_rebindable_syntax_a_literal_takes_the_binding_in_scope_chirho() {
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    for (extension_chirho, method_chirho, literal_chirho, _provenance_chirho) in
        literal_forms_chirho(&mut supply_chirho)
    {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        ctx_chirho.extensions_chirho = std::iter::once("RebindableSyntax")
            .chain(extension_chirho)
            .map(String::from)
            .collect();
        let own_chirho = ctx_chirho.fresh_id_chirho(method_chirho);
        ctx_chirho.bind_in_scope_chirho(method_chirho, own_chirho);

        let core_chirho = ctx_chirho.desugar_expr_chirho(&literal_chirho);
        assert_eq!(
            applied_method_id_chirho(&core_chirho),
            own_chirho,
            "{method_chirho}"
        );
    }
}

#[test]
fn ranges_have_their_own_method_identity_even_under_rebindable_syntax_chirho() {
    for extensions_chirho in [Vec::new(), vec!["RebindableSyntax".to_string()]] {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        ctx_chirho.extensions_chirho = extensions_chirho;
        let own_chirho = ctx_chirho.fresh_id_chirho("enumFromTo");
        ctx_chirho.bind_in_scope_chirho("enumFromTo", own_chirho);
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        let range_chirho = ExprChirho::ArithSeqChirho {
            from_chirho: Box::new(ExprChirho::VarChirho(name_chirho("firstChirho"))),
            then_chirho: None,
            to_chirho: Some(Box::new(ExprChirho::VarChirho(name_chirho("lastChirho")))),
            span_chirho,
        };
        let range_core_chirho = ctx_chirho.desugar_expr_chirho(&range_chirho);
        let mut head_chirho = &range_core_chirho;
        while let CoreExprChirho::AppChirho { fun_chirho, .. } = head_chirho {
            head_chirho = fun_chirho;
        }
        let CoreExprChirho::VarChirho(method_chirho) = head_chirho else {
            panic!("expected a range method application: {range_core_chirho:?}");
        };
        let (name_chirho, canonical_chirho) = &ctx_chirho.method_occurrences_chirho[method_chirho];
        assert_eq!(name_chirho, "enumFromTo");
        assert_ne!(*canonical_chirho, own_chirho);
        assert_eq!(
            ctx_chirho.range_occurrence_spans_chirho.get(method_chirho),
            Some(&span_chirho),
        );
        assert_eq!(
            ctx_chirho.resolve_var_chirho("enumFromTo"),
            own_chirho,
            "a written call still names the source binding",
        );
        assert!(
            !ctx_chirho
                .range_occurrence_spans_chirho
                .contains_key(&own_chirho)
        );
    }
}
