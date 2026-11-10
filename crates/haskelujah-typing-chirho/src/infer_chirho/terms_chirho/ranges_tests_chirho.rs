// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A range's endpoints solve its own Enum evidence independently of a shadow.

use super::InferCtxChirho;
use crate::class_chirho::{PredChirho, PredPartsChirho};
use crate::infer_chirho::ReferenceEvidenceChirho;
use crate::ty_chirho::{SchemeChirho, TyChirho};
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::name_chirho::{
    CompilerClassChirho, PredicateClassIdentityChirho, PredicateClassProofChirho,
};
use haskelujah_span_chirho::{ByteOffsetChirho, SourceMapChirho, SpanChirho};

#[test]
fn a_range_solves_enum_for_its_endpoints_past_a_lexical_shadow_chirho() {
    let mut map_chirho = SourceMapChirho::new_chirho();
    let file_chirho = map_chirho.add_file_chirho("RangeChirho.hs", "['a' .. 'c']");
    let span_chirho = SpanChirho::new_chirho(
        file_chirho,
        ByteOffsetChirho::new_chirho(0),
        ByteOffsetChirho::new_chirho(12),
    );
    let range_chirho = ExprChirho::ArithSeqChirho {
        from_chirho: Box::new(ExprChirho::LitChirho(LitChirho::CharChirho(
            'a',
            span_chirho,
            None,
        ))),
        then_chirho: None,
        to_chirho: Some(Box::new(ExprChirho::LitChirho(LitChirho::CharChirho(
            'c',
            span_chirho,
            None,
        )))),
        span_chirho,
    };
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho.env_chirho.bind_chirho(
        "enumFromTo".to_string(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );
    let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&range_chirho);
    assert_eq!(ty_chirho, TyChirho::string_chirho());
    assert!(!ctx_chirho.diagnostics_chirho.has_errors_chirho());
    let (by_span_chirho, _by_origin_chirho) =
        ctx_chirho.finalize_reference_evidence_chirho(&subst_chirho);
    // The range selects the standard Enum scheme, so its evidence is the compiler's Enum and
    // never the lexical shadow's.
    let compiler_enum_chirho = PredicateClassProofChirho::ProvenChirho(
        PredicateClassIdentityChirho::CompilerChirho(CompilerClassChirho::EnumChirho),
    );
    assert_eq!(
        by_span_chirho.get(&span_chirho),
        Some(&vec![ReferenceEvidenceChirho {
            dictionary_position_chirho: Some(0),
            applied_head_chirho: None,
            class_name_chirho: "Enum".to_string(),
            class_identity_chirho: compiler_enum_chirho.clone(),
            ty_key_chirho: Some("Char".to_string()),
            ground_pred_chirho: Some(PredChirho::from_compiler_chirho(
                CompilerClassChirho::EnumChirho,
                PredPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Enum".to_string(),
                    ty_chirho: TyChirho::char_chirho(),
                    extra_tys_chirho: Vec::new(),
                },
            )),
            applied_ty_chirho: None,
        }]),
    );
    // Predicate equality ignores the class proof, so the ground goal's own proof is pinned here.
    let ground_chirho = by_span_chirho[&span_chirho][0]
        .ground_pred_chirho
        .as_ref()
        .expect("the range's ground goal");
    assert_eq!(ground_chirho.class_identity_chirho(), &compiler_enum_chirho);
}
