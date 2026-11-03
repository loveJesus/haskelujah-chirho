// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The closed-equation rules on hand-built AST: each GHC 9.14.1 negative of row
//! 686's oracle set and the neighbouring shapes the rules must leave alone.

use super::*;
use haskelujah_ast_chirho::decl_chirho::TyVarVisibilityChirho;
use haskelujah_ast_chirho::name_chirho::RawNameChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn qualified_name_chirho(qualifier_chirho: &str, text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::qualified_chirho(
        qualifier_chirho,
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn binders_chirho(count_chirho: usize) -> Vec<TyVarChirho> {
    (0..count_chirho)
        .map(|index_chirho| TyVarChirho::plain_chirho(name_chirho(&format!("a{index_chirho}"))))
        .collect()
}

/// `head Int … = Bool`, with `argument_count_chirho` arguments.
fn equation_chirho(
    head_chirho: Option<NameChirho>,
    argument_count_chirho: usize,
) -> TypeFamilyEquationChirho {
    TypeFamilyEquationChirho {
        head_chirho,
        explicit_binders_chirho: None,
        unwritten_binders_chirho: Vec::new(),
        lhs_types_chirho: (0..argument_count_chirho)
            .map(|_| TypeChirho::ConChirho(name_chirho("Int")))
            .collect(),
        rhs_chirho: TypeChirho::ConChirho(name_chirho("Bool")),
        malformed_chirho: false,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// GHC codes for family `F`, declared in module `M` with these binders.
fn codes_chirho(
    type_vars_chirho: &[TyVarChirho],
    equations_chirho: &[TypeFamilyEquationChirho],
) -> Vec<&'static str> {
    let mut errors_chirho = Vec::new();
    check_family_equations_chirho(
        &name_chirho("M"),
        &name_chirho("F"),
        type_vars_chirho,
        equations_chirho,
        &mut errors_chirho,
    );
    errors_chirho
        .iter()
        .map(|error_chirho| error_chirho.ghc_code_chirho)
        .collect()
}

#[test]
fn an_equation_naming_its_family_with_the_declared_arguments_passes_chirho() {
    assert!(
        codes_chirho(
            &binders_chirho(1),
            &[equation_chirho(Some(name_chirho("F")), 1)]
        )
        .is_empty()
    );
    assert!(
        codes_chirho(&[], &[equation_chirho(Some(name_chirho("F")), 0)]).is_empty(),
        "a nullary family"
    );
}

#[test]
fn a_head_naming_another_type_is_mismatched_chirho() {
    assert_eq!(
        codes_chirho(&[], &[equation_chirho(Some(name_chirho("Maybe")), 1)]),
        ["GHC-88221"],
        "T11623: `type family T where {{ Maybe T = Int }}`; the name is judged before the count"
    );
    assert_eq!(
        codes_chirho(&[], &[equation_chirho(Some(name_chirho("Bar")), 0)]),
        ["GHC-88221"],
        "T20260"
    );
}

#[test]
fn a_head_qualified_by_this_module_names_the_family_chirho() {
    assert!(
        codes_chirho(
            &binders_chirho(1),
            &[equation_chirho(Some(qualified_name_chirho("M", "F")), 1)]
        )
        .is_empty(),
        "oracle R08: GHC 9.14.1 accepts its own module's qualifier"
    );
    assert_eq!(
        codes_chirho(
            &binders_chirho(1),
            &[equation_chirho(Some(qualified_name_chirho("N", "F")), 1)]
        ),
        ["GHC-88221"]
    );
}

#[test]
fn the_argument_count_follows_the_visible_binders_chirho() {
    assert_eq!(
        codes_chirho(
            &binders_chirho(1),
            &[equation_chirho(Some(name_chirho("F")), 2)]
        ),
        ["GHC-12985"],
        "oracle R02"
    );
    assert_eq!(
        codes_chirho(
            &binders_chirho(2),
            &[equation_chirho(Some(name_chirho("F")), 1)]
        ),
        ["GHC-12985"],
        "oracle R03"
    );
    let mut invisible_chirho = TyVarChirho::plain_chirho(name_chirho("k"));
    invisible_chirho.visibility_chirho = TyVarVisibilityChirho::InvisibleChirho;
    let with_invisible_chirho = [
        invisible_chirho,
        TyVarChirho::plain_chirho(name_chirho("a")),
    ];
    assert!(
        codes_chirho(
            &with_invisible_chirho,
            &[equation_chirho(Some(name_chirho("F")), 1)]
        )
        .is_empty(),
        "an invisible `@k` binder is not an argument position"
    );
}

#[test]
fn a_headless_equation_is_a_malformed_head_chirho() {
    assert_eq!(
        codes_chirho(&binders_chirho(1), &[equation_chirho(None, 1)]),
        ["GHC-47568"],
        "oracles R04/R05"
    );
}

#[test]
fn an_incomplete_equation_is_a_parse_error_first_chirho() {
    let mut incomplete_chirho = equation_chirho(Some(name_chirho("Maybe")), 3);
    incomplete_chirho.malformed_chirho = true;
    assert_eq!(
        codes_chirho(&binders_chirho(1), &[incomplete_chirho]),
        ["GHC-58481"],
        "oracles R06/R07: the parse error is reported, not the head or count"
    );
}

#[test]
fn every_equation_is_judged_in_written_order_chirho() {
    assert_eq!(
        codes_chirho(
            &binders_chirho(1),
            &[
                equation_chirho(Some(name_chirho("F")), 1),
                equation_chirho(Some(name_chirho("G")), 1),
                equation_chirho(Some(name_chirho("F")), 0),
            ]
        ),
        ["GHC-88221", "GHC-12985"]
    );
}

#[test]
fn an_unused_binder_is_held_not_reported_chirho() {
    let mut unused_chirho = equation_chirho(Some(name_chirho("F")), 0);
    unused_chirho.explicit_binders_chirho = Some(binders_chirho(1));
    unused_chirho.unwritten_binders_chirho = vec![name_chirho("a0")];
    assert!(
        codes_chirho(&[], std::slice::from_ref(&unused_chirho)).is_empty(),
        "no standalone GHC-30337 rule in row 686"
    );
    assert!(
        unused_chirho.is_held_chirho(),
        "registration never reduces through it"
    );
}
