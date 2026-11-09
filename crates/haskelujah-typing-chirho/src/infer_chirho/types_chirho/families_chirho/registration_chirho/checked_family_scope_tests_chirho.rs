// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Fused typed default substitution retains simultaneous ownership semantics
//! and refuses growth before materializing repeated borrowed images.

use super::*;
use std::borrow::Cow as CowChirho;

#[test]
fn default_images_are_simultaneous_even_when_numeric_ids_overlap_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let actual_chirho = TyChirho::VarChirho(TyVarChirho(1));
    let other_chirho = TyChirho::ConChirho("Bool".to_owned());
    let images_chirho = HashMap::from([
        (TyVarChirho(0), CowChirho::Borrowed(&actual_chirho)),
        (TyVarChirho(1), CowChirho::Borrowed(&other_chirho)),
    ]);
    let variable_chirho = TyChirho::VarChirho(TyVarChirho(0));
    let (lhs_chirho, rhs_chirho, names_chirho) = context_chirho
        .specialize_checked_family_terms_chirho(
            std::slice::from_ref(&variable_chirho),
            &variable_chirho,
            &images_chirho,
            &mut 100_000,
        )
        .expect("actual meta does not reenter template substitution");
    assert_eq!(
        lhs_chirho,
        vec![TyChirho::ForallVarChirho(names_chirho[0].clone())]
    );
    assert_eq!(lhs_chirho[0], rhs_chirho);
}

#[test]
fn repeated_borrowed_default_images_share_one_growth_allowance_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let actual_chirho = TyChirho::ConChirho("x".repeat(60_000));
    let images_chirho = HashMap::from([(TyVarChirho(0), CowChirho::Borrowed(&actual_chirho))]);
    let variable_chirho = TyChirho::VarChirho(TyVarChirho(0));
    assert!(
        context_chirho
            .specialize_checked_family_terms_chirho(
                &[variable_chirho.clone(), variable_chirho.clone()],
                &variable_chirho,
                &images_chirho,
                &mut 100_000
            )
            .is_none()
    );
    assert!(context_chirho.has_non_deferrable_failure_chirho());
}

#[test]
fn a_deep_default_image_is_refused_before_unbounded_recursion_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let mut actual_chirho = TyChirho::ConChirho("Bool".to_owned());
    for _depth_chirho in 0..129 {
        actual_chirho = TyChirho::ListChirho(Box::new(actual_chirho));
    }
    let images_chirho = HashMap::from([(TyVarChirho(0), CowChirho::Borrowed(&actual_chirho))]);
    let variable_chirho = TyChirho::VarChirho(TyVarChirho(0));
    assert!(
        context_chirho
            .specialize_checked_family_terms_chirho(
                std::slice::from_ref(&variable_chirho),
                &variable_chirho,
                &images_chirho,
                &mut 100_000
            )
            .is_none()
    );
    assert!(context_chirho.has_non_deferrable_failure_chirho());
}

#[test]
fn default_rhs_variables_require_a_checked_lhs_binder_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    assert!(
        context_chirho
            .abstract_checked_family_terms_chirho(
                &[TyChirho::ConChirho("Bool".to_owned())],
                &TyChirho::VarChirho(TyVarChirho(9))
            )
            .is_none()
    );
    assert!(context_chirho.has_non_deferrable_failure_chirho());
}

fn published_context_chirho() -> (
    InferCtxChirho,
    haskelujah_ast_chirho::name_chirho::NameChirho,
) {
    use crate::kind_chirho::elaboration_chirho::{
        ElaborationImportsChirho, elaborate_declaration_module_chirho,
    };
    use haskelujah_ast_chirho::decl_chirho::{DeclChirho, TypeFamilyEquationChirho};
    use haskelujah_ast_chirho::name_chirho::{
        NameChirho, RawNameChirho, TypeConstructorDeclarationChirho, TypeConstructorRoleChirho,
        TypeConstructorScopeChirho,
    };
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let name_chirho = |text_chirho: &str| {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(text_chirho, span_chirho))
    };
    let mut family_chirho = name_chirho("FamilyChirho");
    let proof_chirho = TypeConstructorDeclarationChirho::source_chirho(
        "Nested.PublicationChirho",
        "FamilyChirho",
        TypeConstructorRoleChirho::TypeFamilyChirho,
    );
    family_chirho.set_type_constructor_chirho(Some(proof_chirho.clone()));
    let mut scope_chirho = TypeConstructorScopeChirho::default();
    scope_chirho.bind_chirho("FamilyChirho".into(), Some(proof_chirho));
    let mut module_chirho = ModuleChirho {
        name_chirho: NameChirho::RawChirho(RawNameChirho::qualified_chirho(
            "Nested",
            "PublicationChirho",
            span_chirho,
        )),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::TypeFamilyDeclChirho {
            injectivity_chirho: None,
            name_chirho: family_chirho.clone(),
            type_vars_chirho: vec![name_chirho("aChirho").into()],
            result_kind_chirho: None,
            data_family_chirho: false,
            standalone_kind_chirho: None,
            closed_chirho: true,
            equations_chirho: vec![TypeFamilyEquationChirho {
                kind_args_chirho: vec![],
                head_chirho: Some(family_chirho.clone()),
                explicit_binders_chirho: None,
                unwritten_binders_chirho: vec![],
                lhs_types_chirho: vec![TypeChirho::VarChirho(name_chirho("aChirho"))],
                rhs_chirho: TypeChirho::VarChirho(name_chirho("aChirho")),
                malformed_chirho: false,
                span_chirho,
            }],
            span_chirho,
        }],
        extensions_chirho: vec!["TypeFamilies".into()],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho,
        origin_supply_chirho: Default::default(),
    };
    let checked_chirho = elaborate_declaration_module_chirho(
        &mut module_chirho,
        ElaborationImportsChirho {
            registry_chirho: &TypeFamilyEnvChirho::default(),
            type_scope_chirho: &scope_chirho,
            family_scope_chirho: &Default::default(),
            promotion_scope_chirho: &Default::default(),
            builtin_classes_chirho: &Default::default(),
            source_unit_chirho: None,
        },
        true,
    )
    .unwrap();
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.elaboration_state_chirho = checked_chirho.state_chirho;
    context_chirho.type_families_chirho = checked_chirho.registry_chirho;
    context_chirho.prepare_family_scope_chirho(&module_chirho, None);
    (context_chirho, family_chirho)
}

#[test]
fn checked_publication_reuse_requires_actual_owned_full_boundary_chirho() {
    let (mut context_chirho, name_chirho) = published_context_chirho();
    assert!(context_chirho.has_published_checked_closed_family_chirho(&name_chirho, 1, 1));
    assert!(!context_chirho.has_published_checked_closed_family_chirho(&name_chirho, 0, 1));
    assert!(!context_chirho.has_published_checked_closed_family_chirho(&name_chirho, 1, 0));
    let mut raw_chirho = name_chirho.clone();
    raw_chirho.set_type_constructor_chirho(None);
    assert!(!context_chirho.has_published_checked_closed_family_chirho(&raw_chirho, 1, 1));
    context_chirho.family_module_name_chirho = "OtherChirho".into();
    assert!(!context_chirho.has_published_checked_closed_family_chirho(&name_chirho, 1, 1));
}

#[test]
fn raw_legacy_context_cannot_reuse_checked_equations_by_spelling_chirho() {
    let (mut context_chirho, name_chirho) = published_context_chirho();
    context_chirho.elaboration_state_chirho =
        crate::kind_chirho::elaboration_chirho::ElaborationStateChirho::unowned_chirho();
    assert!(!context_chirho.has_published_checked_closed_family_chirho(&name_chirho, 1, 1));
}
