// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Promoted constructors' identity through the typing gateways (row 686), on the direct
//! typing API: the legacy spellings a caller without naming still gets (not corrected),
//! and the one identity a naming scope gives both converters, the builtin Bool and list
//! producers, suffix equality, the imported-name normalizer and rigidity.

use super::*;
use crate::unify_chirho::unify_chirho;
use haskelujah_ast_chirho::name_chirho::RawNameChirho;
use haskelujah_ast_chirho::promotion_chirho::{
    PromotedKeyChirho, PromotionResolutionChirho, PromotionScopeChirho,
};

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn quoted_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::PromotedConChirho {
        name_chirho: name_chirho(text_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn unquoted_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::ConChirho(name_chirho(text_chirho))
}

fn con_chirho(key_chirho: &PromotedKeyChirho) -> TyChirho {
    TyChirho::ConChirho(key_chirho.as_str_chirho().to_string())
}

/// Main declares `data TChirho = TChirho`; the standard `True` is in scope too.
fn scope_chirho() -> PromotionScopeChirho {
    let constructors_chirho = HashMap::from([
        (
            "True".to_string(),
            PromotionResolutionChirho::ResolvedChirho(PromotedKeyChirho::builtin_bool_chirho(true)),
        ),
        (
            "TChirho".to_string(),
            PromotionResolutionChirho::ResolvedChirho(PromotedKeyChirho::source_chirho(
                "Main", "TChirho",
            )),
        ),
    ]);
    let types_chirho = HashSet::from(["TChirho".to_string(), "Bool".to_string()]);
    PromotionScopeChirho::new_chirho(constructors_chirho, types_chirho)
}

fn scoped_ctx_chirho() -> InferCtxChirho {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho.promotion_scope_chirho = Some(Arc::new(scope_chirho()));
    ctx_chirho
}

/// Both converters' results for one written type.
fn both_chirho(ctx_chirho: &mut InferCtxChirho, ty_chirho: &TypeChirho) -> (TyChirho, TyChirho) {
    let use_site_chirho = ctx_chirho.ast_type_to_ty_chirho(ty_chirho, &mut HashMap::new());
    let synonym_chirho =
        ast_type_to_syn_rhs_chirho(ty_chirho, &[], ctx_chirho.promotion_scope_chirho.as_deref());
    (use_site_chirho, synonym_chirho)
}

#[test]
fn without_naming_the_legacy_spellings_still_disagree_chirho() {
    // A direct caller that never ran naming keeps the pre-686 spellings. They are not the
    // corrected identity: the two converters disagree on `'True`.
    let (use_site_chirho, synonym_chirho) =
        both_chirho(&mut InferCtxChirho::new_chirho(), &quoted_chirho("True"));
    assert_eq!(use_site_chirho, TyChirho::ConChirho("True".to_string()));
    assert_eq!(synonym_chirho, TyChirho::ConChirho("'True".to_string()));
}

#[test]
fn a_scope_gives_both_converters_one_identity_chirho() {
    let true_chirho = con_chirho(&PromotedKeyChirho::builtin_bool_chirho(true));
    let mut ctx_chirho = scoped_ctx_chirho();
    assert_eq!(
        both_chirho(&mut ctx_chirho, &quoted_chirho("True")),
        (true_chirho.clone(), true_chirho.clone()),
        "C17's 'True"
    );
    assert_eq!(
        both_chirho(&mut ctx_chirho, &unquoted_chirho("True")),
        (true_chirho.clone(), true_chirho),
        "C17's unquoted True names the same constructor"
    );
}

#[test]
fn a_quoted_homonym_differs_from_its_type_chirho() {
    let mut ctx_chirho = scoped_ctx_chirho();
    let promoted_chirho = con_chirho(&PromotedKeyChirho::source_chirho("Main", "TChirho"));
    assert_eq!(
        both_chirho(&mut ctx_chirho, &quoted_chirho("TChirho")),
        (promoted_chirho.clone(), promoted_chirho),
        "C15b's 'TChirho"
    );
    let (use_site_chirho, synonym_chirho) =
        both_chirho(&mut ctx_chirho, &unquoted_chirho("TChirho"));
    assert_eq!(
        use_site_chirho,
        TyChirho::ConChirho("TChirho".to_string()),
        "the type"
    );
    assert_eq!(synonym_chirho, TyChirho::ConChirho("TChirho".to_string()));
}

#[test]
fn an_unproven_quote_is_one_unknown_identity_chirho() {
    let mut ctx_chirho = scoped_ctx_chirho();
    let (use_site_chirho, synonym_chirho) =
        both_chirho(&mut ctx_chirho, &quoted_chirho("MissingChirho"));
    assert_eq!(
        use_site_chirho, synonym_chirho,
        "reflexive: the same spelling meets itself"
    );
    let unknown_chirho = con_chirho(&PromotedKeyChirho::unknown_chirho("", "MissingChirho"));
    assert_eq!(use_site_chirho, unknown_chirho);
    assert!(
        !ctx_chirho.ty_is_simple_rigid_arg_chirho(&unknown_chirho),
        "an Unknown constructor is never trusted as rigid"
    );
    assert!(
        ctx_chirho.ty_is_simple_rigid_arg_chirho(&con_chirho(
            &PromotedKeyChirho::builtin_bool_chirho(true)
        ))
    );
}

#[test]
fn a_stamp_wins_over_the_scope_chirho() {
    let mut stamped_chirho = name_chirho("TChirho");
    let exporter_key_chirho = PromotedKeyChirho::source_chirho("DefChirho", "TChirho");
    stamped_chirho.set_promotion_chirho(PromotionResolutionChirho::ResolvedChirho(
        exporter_key_chirho.clone(),
    ));
    let written_chirho = TypeChirho::PromotedConChirho {
        name_chirho: stamped_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let empty_scope_chirho = PromotionScopeChirho::default();
    assert_eq!(
        ast_type_to_syn_rhs_chirho(&written_chirho, &[], Some(&empty_scope_chirho)),
        con_chirho(&exporter_key_chirho),
        "an imported right side keeps its exporter's identity"
    );
    assert_eq!(
        ast_type_to_syn_rhs_chirho(&written_chirho, &[], Some(&scope_chirho())),
        con_chirho(&exporter_key_chirho),
        "the importer's own scope never re-resolves it"
    );
}

#[test]
fn the_compilers_bool_and_lists_are_keyed_under_a_scope_chirho() {
    let true_chirho = con_chirho(&PromotedKeyChirho::builtin_bool_chirho(true));
    let false_chirho = con_chirho(&PromotedKeyChirho::builtin_bool_chirho(false));
    assert_eq!(
        scoped_ctx_chirho().reduce_type_family_chirho("&&", &[true_chirho, false_chirho.clone()]),
        Some(false_chirho)
    );
    assert_eq!(
        InferCtxChirho::new_chirho().reduce_type_family_chirho(
            "&&",
            &[
                TyChirho::ConChirho("True".to_string()),
                TyChirho::ConChirho("False".to_string()),
            ],
        ),
        Some(TyChirho::ConChirho("False".to_string())),
        "legacy spellings without a scope"
    );
    let nil_chirho = TypeChirho::PromotedListChirho {
        elements_chirho: Vec::new(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert_eq!(
        ast_type_to_syn_rhs_chirho(&nil_chirho, &[], Some(&scope_chirho())),
        con_chirho(&PromotedKeyChirho::builtin_nil_chirho())
    );
    assert_eq!(
        scoped_ctx_chirho().ast_type_to_ty_chirho(&quoted_chirho(":"), &mut HashMap::new()),
        con_chirho(&PromotedKeyChirho::builtin_cons_chirho()),
        "':` is always the compiler's cons"
    );
}

#[test]
fn keys_unify_only_whole_and_survive_the_name_normalizer_chirho() {
    // Both keys end in `TypesTrue` after their last `.`: a qualified-suffix match would
    // have equated the compiler's True with a source module `My.Types`' True.
    let builtin_chirho = con_chirho(&PromotedKeyChirho::builtin_bool_chirho(true));
    let source_chirho = con_chirho(&PromotedKeyChirho::source_chirho("My.Types", "True"));
    assert!(unify_chirho(&builtin_chirho, &source_chirho, SpanChirho::DUMMY_CHIRHO).is_err());
    assert!(unify_chirho(&builtin_chirho, &builtin_chirho, SpanChirho::DUMMY_CHIRHO).is_ok());
    let key_chirho = PromotedKeyChirho::builtin_bool_chirho(true);
    assert_eq!(
        scoped_ctx_chirho().normalize_imported_type_name_chirho(key_chirho.as_str_chirho()),
        key_chirho.as_str_chirho()
    );
}

#[test]
fn the_compilers_unit_is_keyed_without_a_constructor_table_chirho() {
    let unit_chirho = con_chirho(&PromotedKeyChirho::builtin_unit_chirho());
    assert_eq!(
        both_chirho(&mut scoped_ctx_chirho(), &quoted_chirho("()")),
        (unit_chirho.clone(), unit_chirho),
        "TypeOf's '() is the compiler's unit, not an unproven spelling"
    );
}
