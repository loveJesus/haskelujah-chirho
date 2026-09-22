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
