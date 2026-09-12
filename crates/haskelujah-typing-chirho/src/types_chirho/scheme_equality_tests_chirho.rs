// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Closed promises require equality, not a successful specialization.
use super::*;
use crate::ty_chirho::MultChirho;

fn identity_chirho(id_chirho: u32) -> SchemeChirho {
    let variable_chirho = TyChirho::VarChirho(TyVarChirho(id_chirho));
    SchemeChirho {
        vars_chirho: vec![TyVarChirho(id_chirho)],
        preds_chirho: vec![],
        ty_chirho: TyChirho::fun_chirho(variable_chirho.clone(), variable_chirho),
    }
}

#[test]
fn boot_scheme_equality_retains_variable_correlations_and_predicates_chirho() {
    let expected_chirho = identity_chirho(0);
    let mut actual_chirho = identity_chirho(91);
    assert!(expected_chirho.alpha_equivalent_chirho(&actual_chirho));
    actual_chirho.vars_chirho.push(TyVarChirho(92));
    actual_chirho.ty_chirho = TyChirho::fun_chirho(
        TyChirho::VarChirho(TyVarChirho(91)),
        TyChirho::VarChirho(TyVarChirho(92)),
    );
    assert!(!expected_chirho.alpha_equivalent_chirho(&actual_chirho));
    let mut constrained_chirho = expected_chirho.clone();
    constrained_chirho.preds_chirho.push(SchemePredChirho {
        class_name_chirho: "ClassChirho".to_owned(),
        ty_chirho: TyChirho::VarChirho(TyVarChirho(0)),
        extra_tys_chirho: vec![],
    });
    assert!(!expected_chirho.alpha_equivalent_chirho(&constrained_chirho));
}

#[test]
fn boot_scheme_equality_does_not_cancel_qualified_names_or_multiplicity_chirho() {
    let nominal_chirho =
        |name_chirho: &str| SchemeChirho::mono_chirho(TyChirho::ConChirho(name_chirho.to_owned()));
    assert!(
        !nominal_chirho("AChirho.TChirho")
            .alpha_equivalent_chirho(&nominal_chirho("BChirho.TChirho"))
    );
    assert!(!nominal_chirho("Int").alpha_equivalent_chirho(&nominal_chirho("Double")));
    let expected_chirho = identity_chirho(4);
    let mut linear_chirho = expected_chirho.clone();
    let TyChirho::FunChirho(_, _, multiplicity_chirho) = &mut linear_chirho.ty_chirho else {
        unreachable!()
    };
    *multiplicity_chirho = MultChirho::OneChirho;
    assert!(!expected_chirho.alpha_equivalent_chirho(&linear_chirho));
}

#[test]
fn boot_scheme_equality_closes_nested_scopes_without_capture_chirho() {
    let nested_chirho = |outer_chirho, inner_chirho| SchemeChirho {
        vars_chirho: vec![TyVarChirho(outer_chirho)],
        preds_chirho: vec![],
        ty_chirho: TyChirho::TupleChirho(vec![
            TyChirho::ForallChirho {
                vars_chirho: vec![TyVarChirho(inner_chirho)],
                body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(inner_chirho))),
            },
            TyChirho::VarChirho(TyVarChirho(outer_chirho)),
        ]),
    };
    assert!(nested_chirho(0, 0).alpha_equivalent_chirho(&nested_chirho(50, 51)));
    let mut visible_chirho = nested_chirho(50, 51);
    let TyChirho::TupleChirho(elements_chirho) = &mut visible_chirho.ty_chirho else {
        unreachable!()
    };
    elements_chirho[0] = TyChirho::RequiredForallChirho {
        vars_chirho: vec![TyVarChirho(51)],
        body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(51))),
    };
    assert!(!nested_chirho(0, 0).alpha_equivalent_chirho(&visible_chirho));
    for free_chirho in [
        TyChirho::VarChirho(TyVarChirho(0)),
        TyChirho::ForallVarChirho("tv0".to_owned()),
    ] {
        let free_chirho = SchemeChirho::mono_chirho(free_chirho);
        assert!(!free_chirho.alpha_equivalent_chirho(&free_chirho));
    }
}

#[test]
fn boot_scheme_equality_bounds_large_and_deep_contracts_chirho() {
    let large_chirho = SchemeChirho::mono_chirho(TyChirho::TupleChirho(vec![
        TyChirho::ConChirho(
            "Int".to_owned()
        );
        16_384
    ]));
    assert!(!large_chirho.alpha_equivalent_chirho(&large_chirho));
    let mut deep_chirho = TyChirho::ConChirho("Int".to_owned());
    for _index_chirho in 0..300 {
        deep_chirho = TyChirho::ListChirho(Box::new(deep_chirho));
    }
    let deep_chirho = SchemeChirho::mono_chirho(deep_chirho);
    assert!(!deep_chirho.alpha_equivalent_chirho(&deep_chirho));
}

#[test]
fn boot_alias_equality_renames_only_declared_parameters_chirho() {
    let body_chirho = |name_chirho: &str| {
        TyChirho::ListChirho(Box::new(TyChirho::ConChirho(name_chirho.to_owned())))
    };
    assert_eq!(
        canonical_named_body_chirho(&["aChirho"], &body_chirho("aChirho")),
        canonical_named_body_chirho(&["bChirho"], &body_chirho("bChirho"))
    );
    assert_ne!(
        canonical_named_body_chirho(&["aChirho"], &body_chirho("AChirho.TChirho")),
        canonical_named_body_chirho(&["bChirho"], &body_chirho("BChirho.TChirho"))
    );
    assert!(
        canonical_named_body_chirho(&["aChirho", "aChirho"], &body_chirho("aChirho")).is_none()
    );
}
