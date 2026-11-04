// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Capture and growth controls for the actual checked synonym converter.

use super::*;
use crate::ty_chirho::declarations_chirho::{
    DeclarationBinderChirho, DeclarationSchemeChirho, TelescopeVisibilityChirho,
};
use haskelujah_ast_chirho::name_chirho::{
    TypeConstructorDeclarationChirho, TypeConstructorRoleChirho,
};

fn alias_chirho(
    context_chirho: &mut InferCtxChirho,
    name_chirho: &str,
    body_chirho: TyChirho,
) -> String {
    let declaration_chirho = TypeConstructorDeclarationChirho::source_chirho(
        "ProducerChirho",
        name_chirho,
        TypeConstructorRoleChirho::TypeSynonymChirho,
    );
    let key_chirho = declaration_chirho.type_key_chirho();
    context_chirho
        .type_families_chirho
        .declaration_schemes_mut_chirho()
        .insert_chirho(DeclarationSchemeChirho {
            association_chirho:
                crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::NoneChirho,
            identity_chirho: key_chirho.clone().into(),
            owner_chirho: None,
            role_chirho: TypeConstructorRoleChirho::TypeSynonymChirho,
            declaration_chirho: Some(declaration_chirho),
            binders_chirho: vec![DeclarationBinderChirho {
                variable_chirho: TyVarChirho(0),
                kind_chirho: TyChirho::ConChirho("%universe:type".to_owned()),
                visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
            }],
            source_variables_chirho: Vec::new(),
            result_kind_chirho: TyChirho::ConChirho("%universe:type".to_owned()),
            synonym_rhs_chirho: Some(body_chirho),
            visible_arity_chirho: 1,
            reduction_arity_chirho: 1,
            promoted_chirho: false,
            held_chirho: false,
        })
        .expect("fixture uses a closed, checked telescope");
    key_chirho
}

fn apply_chirho(key_chirho: String, argument_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho(key_chirho)),
        Box::new(argument_chirho),
    )
}

#[test]
fn checked_alias_substitution_does_not_capture_the_callers_free_variable_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let key_chirho = alias_chirho(
        &mut context_chirho,
        "RankChirho",
        TyChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho(1)],
            body_chirho: Box::new(TyChirho::FunChirho(
                Box::new(TyChirho::VarChirho(TyVarChirho(1))),
                Box::new(TyChirho::VarChirho(TyVarChirho(0))),
                crate::ty_chirho::MultChirho::ManyChirho,
            )),
        },
    );
    let expanded_chirho = context_chirho.expand_type_synonyms_chirho(&apply_chirho(
        key_chirho,
        TyChirho::VarChirho(TyVarChirho(1)),
    ));
    assert_eq!(expanded_chirho.free_vars_chirho(), vec![TyVarChirho(1)]);
    let TyChirho::ForallChirho {
        vars_chirho,
        body_chirho,
    } = expanded_chirho
    else {
        panic!("rank-n synonym remains universal");
    };
    let TyChirho::FunChirho(argument_chirho, result_chirho, _) = *body_chirho else {
        panic!("rank-n body remains a function");
    };
    assert_eq!(*argument_chirho, TyChirho::VarChirho(vars_chirho[0]));
    assert_eq!(*result_chirho, TyChirho::VarChirho(TyVarChirho(1)));
    assert_ne!(vars_chirho[0], TyVarChirho(1));
}

#[test]
fn checked_alias_growth_sets_the_non_deferrable_resource_channel_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let mut application_chirho = TyChirho::VarChirho(TyVarChirho(0));
    for index_chirho in 0..16 {
        let key_chirho = alias_chirho(
            &mut context_chirho,
            &format!("Double{index_chirho}Chirho"),
            TyChirho::TupleChirho(vec![application_chirho.clone(), application_chirho]),
        );
        application_chirho = apply_chirho(key_chirho, TyChirho::VarChirho(TyVarChirho(0)));
    }
    context_chirho.expand_type_synonyms_chirho(&application_chirho);
    assert!(context_chirho.has_non_deferrable_failure_chirho());
    assert!(
        context_chirho
            .finish_chirho()
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
}

#[test]
fn checked_aliases_share_the_equality_normalizers_allowance_and_substitution_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let key_chirho = alias_chirho(
        &mut context_chirho,
        "IdentityChirho",
        TyChirho::VarChirho(TyVarChirho(0)),
    );
    let application_chirho = apply_chirho(key_chirho, TyChirho::VarChirho(TyVarChirho(17)));
    context_chirho.record_equality_rewrites_chirho(vec![(
        TyChirho::VarChirho(TyVarChirho(17)),
        TyChirho::int_chirho(),
    )]);
    assert_eq!(
        context_chirho
            .normalize_equality_type_chirho(&application_chirho, 0)
            .unwrap(),
        TyChirho::int_chirho()
    );
}
