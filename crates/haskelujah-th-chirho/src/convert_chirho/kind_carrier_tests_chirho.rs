// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Full kind syntax survives TH conversion and reification.

use super::*;
use crate::reify_chirho::{
    ast_con_to_th_chirho, ast_type_to_th_chirho, ast_tyvar_to_th_chirho, reify_decl_chirho,
};

fn constructor_chirho(text_chirho: &str) -> ThTypeChirho {
    ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(text_chirho))
}

fn variable_chirho(text_chirho: &str) -> ThTypeChirho {
    ThTypeChirho::VarTChirho(ThNameChirho::mk_name_chirho(text_chirho))
}

#[test]
fn kind_application_and_annotation_round_trip_without_ordinary_argument_growth_chirho() {
    let source_chirho = ThTypeChirho::SigTChirho(
        Box::new(ThTypeChirho::AppTChirho(
            Box::new(ThTypeChirho::AppKindTChirho(
                Box::new(constructor_chirho("ProxyChirho")),
                Box::new(ThTypeChirho::StarTChirho),
            )),
            Box::new(variable_chirho("aChirho")),
        )),
        Box::new(ThTypeChirho::StarTChirho),
    );
    let ast_chirho = th_type_to_ast_chirho(&source_chirho);
    assert_eq!(ast_type_to_th_chirho(&ast_chirho), source_chirho);
    let TypeChirho::KindAnnotationChirho { ty_chirho, .. } = ast_chirho else {
        panic!("kind annotation lost");
    };
    let TypeChirho::AppChirho { fun_chirho, .. } = ty_chirho.as_ref() else {
        panic!("ordinary argument lost");
    };
    assert!(matches!(
        fun_chirho.as_ref(),
        TypeChirho::KindApplicationChirho {
            written_chirho: true,
            ..
        }
    ));
}

#[test]
fn a_full_kinded_binder_is_shared_by_forall_and_declaration_conversion_chirho() {
    let binder_chirho = ThTyVarBndrChirho::KindedTVChirho(
        ThNameChirho::mk_name_chirho("aChirho"),
        Box::new(ThTypeChirho::AppTChirho(
            Box::new(constructor_chirho("Maybe")),
            Box::new(variable_chirho("kChirho")),
        )),
    );
    let forall_chirho = ThTypeChirho::ForallTChirho(
        vec![binder_chirho.clone()],
        vec![],
        Box::new(variable_chirho("aChirho")),
    );
    assert_eq!(
        ast_type_to_th_chirho(&th_type_to_ast_chirho(&forall_chirho)),
        forall_chirho
    );
    let alias_chirho = ThDecChirho::TySynDChirho(
        ThNameChirho::mk_name_chirho("AliasChirho"),
        vec![binder_chirho.clone()],
        Box::new(variable_chirho("aChirho")),
    );
    let DeclChirho::TypeAliasDeclChirho {
        type_vars_chirho, ..
    } = th_dec_to_ast_chirho(&alias_chirho).expect("alias")
    else {
        panic!("alias lost");
    };
    assert_eq!(ast_tyvar_to_th_chirho(&type_vars_chirho[0]), binder_chirho);
}

#[test]
fn constraint_kind_prefix_and_whole_annotation_round_trip_chirho() {
    let predicate_chirho = ThTypeChirho::SigTChirho(
        Box::new(ThTypeChirho::AppTChirho(
            Box::new(ThTypeChirho::AppKindTChirho(
                Box::new(constructor_chirho("CChirho")),
                Box::new(ThTypeChirho::StarTChirho),
            )),
            Box::new(variable_chirho("aChirho")),
        )),
        Box::new(ThTypeChirho::ConstraintTChirho),
    );
    let source_chirho = ThTypeChirho::ForallTChirho(
        vec![],
        vec![predicate_chirho.clone()],
        Box::new(constructor_chirho("Int")),
    );
    let ast_chirho = th_type_to_ast_chirho(&source_chirho);
    assert_eq!(ast_type_to_th_chirho(&ast_chirho), source_chirho);
    let TypeChirho::QualChirho { context_chirho, .. } = ast_chirho else {
        panic!("context lost");
    };
    let ConstraintChirho::KindAnnotationChirho {
        constraint_chirho, ..
    } = &context_chirho[0]
    else {
        panic!("outer annotation lost");
    };
    let ConstraintChirho::ClassChirho {
        kind_args_chirho,
        args_chirho,
        ..
    } = constraint_chirho.as_ref()
    else {
        panic!("class lost");
    };
    assert_eq!(kind_args_chirho.len(), 1);
    assert_eq!(args_chirho.len(), 1);
}

#[test]
fn quantified_constraint_binders_retain_their_kinds_chirho() {
    let binder_chirho = ThTyVarBndrChirho::KindedTVChirho(
        ThNameChirho::mk_name_chirho("aChirho"),
        Box::new(ThTypeChirho::StarTChirho),
    );
    let predicate_chirho = ThTypeChirho::ForallTChirho(
        vec![binder_chirho],
        vec![],
        Box::new(ThTypeChirho::AppTChirho(
            Box::new(constructor_chirho("CChirho")),
            Box::new(variable_chirho("aChirho")),
        )),
    );
    let source_chirho = ThTypeChirho::ForallTChirho(
        vec![],
        vec![predicate_chirho],
        Box::new(constructor_chirho("Int")),
    );
    assert_eq!(
        ast_type_to_th_chirho(&th_type_to_ast_chirho(&source_chirho)),
        source_chirho
    );
}

#[test]
fn typed_th_universes_have_authority_and_source_homonyms_do_not_chirho() {
    for (source_chirho, identity_chirho) in [
        (
            ThTypeChirho::StarTChirho,
            CompilerTypeConstructorChirho::TypeUniverseChirho,
        ),
        (
            ThTypeChirho::ConstraintTChirho,
            CompilerTypeConstructorChirho::ConstraintUniverseChirho,
        ),
    ] {
        let ast_chirho = th_type_to_ast_chirho(&source_chirho);
        let TypeChirho::ConChirho(name_chirho) = &ast_chirho else {
            panic!("universe head");
        };
        assert_eq!(
            name_chirho.type_constructor_chirho(),
            Some(&TypeConstructorDeclarationChirho::compiler_data_chirho(
                identity_chirho
            ))
        );
        assert_eq!(ast_type_to_th_chirho(&ast_chirho), source_chirho);
    }
    let source_chirho = constructor_chirho("Type");
    let ast_chirho = th_type_to_ast_chirho(&source_chirho);
    let TypeChirho::ConChirho(name_chirho) = &ast_chirho else {
        panic!("source head");
    };
    assert!(name_chirho.type_constructor_chirho().is_none());
    assert_eq!(ast_type_to_th_chirho(&ast_chirho), source_chirho);
}

#[test]
fn promoted_kind_structure_and_wildcards_survive_the_surface_boundary_chirho() {
    for kind_chirho in [
        ThTypeChirho::PromotedListTChirho(vec![
            ThTypeChirho::StarTChirho,
            ThTypeChirho::ConstraintTChirho,
        ]),
        ThTypeChirho::PromotedTupleTChirho(vec![
            ThTypeChirho::StarTChirho,
            ThTypeChirho::ConstraintTChirho,
        ]),
        ThTypeChirho::WildCardTChirho,
    ] {
        let source_chirho =
            ThTypeChirho::SigTChirho(Box::new(variable_chirho("aChirho")), Box::new(kind_chirho));
        let first_chirho = th_type_to_ast_chirho(&source_chirho);
        let second_chirho = th_type_to_ast_chirho(&ast_type_to_th_chirho(&first_chirho));
        assert_eq!(second_chirho, first_chirho);
    }
}

#[test]
fn data_result_kinds_are_retained_by_conversion_and_reification_chirho() {
    let source_chirho = ThDecChirho::DataDChirho(
        vec![],
        ThNameChirho::mk_name_chirho("DataChirho"),
        vec![],
        Some(Box::new(ThTypeChirho::StarTChirho)),
        vec![],
        vec![],
    );
    let ast_chirho = th_dec_to_ast_chirho(&source_chirho).expect("data declaration");
    let mut budget_chirho =
        haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho();
    assert_eq!(
        reify_decl_chirho(&ast_chirho, &mut budget_chirho).expect("within projection budget"),
        Some(ThInfoChirho::TyConIChirho(source_chirho))
    );
}

#[test]
fn gadt_forall_kinded_binders_use_the_same_full_carrier_chirho() {
    let source_chirho = ThConChirho::ForallCChirho(
        vec![ThTyVarBndrChirho::KindedTVChirho(
            ThNameChirho::mk_name_chirho("aChirho"),
            Box::new(ThTypeChirho::AppTChirho(
                Box::new(constructor_chirho("Maybe")),
                Box::new(variable_chirho("kChirho")),
            )),
        )],
        vec![],
        Box::new(ThConChirho::GadtCChirho(
            vec![ThNameChirho::mk_name_chirho("MkDataChirho")],
            vec![],
            Box::new(constructor_chirho("DataChirho")),
        )),
    );
    let ast_chirho = th_con_to_ast_chirho(&source_chirho).expect("constructor");
    let mut budget_chirho =
        haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho();
    assert_eq!(
        ast_con_to_th_chirho(&ast_chirho, &mut budget_chirho).expect("within projection budget"),
        source_chirho
    );
}

#[test]
fn instance_declarations_keep_hidden_kinds_separate_from_visible_head_types_chirho() {
    let source_chirho = ThDecChirho::InstanceDChirho(
        None,
        vec![],
        Box::new(ThTypeChirho::AppTChirho(
            Box::new(ThTypeChirho::AppKindTChirho(
                Box::new(constructor_chirho("CChirho")),
                Box::new(ThTypeChirho::StarTChirho),
            )),
            Box::new(constructor_chirho("Int")),
        )),
        vec![],
    );
    let DeclChirho::InstanceDeclChirho {
        kind_args_chirho,
        types_chirho,
        ..
    } = th_dec_to_ast_chirho(&source_chirho).expect("instance")
    else {
        panic!("instance lost");
    };
    assert_eq!(kind_args_chirho.len(), 1);
    assert_eq!(types_chirho.len(), 1);
    assert_eq!(
        ast_type_to_th_chirho(&kind_args_chirho[0]),
        ThTypeChirho::StarTChirho
    );
    assert_eq!(
        ast_type_to_th_chirho(&types_chirho[0]),
        constructor_chirho("Int")
    );
}
