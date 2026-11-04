// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho;
use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::{
    ElaborationOwnerChirho, ScopedTypeVariableIdChirho,
};

fn header_chirho(identity_chirho: &str) -> DeclarationSchemeChirho {
    DeclarationSchemeChirho {
        identity_chirho: Arc::from(identity_chirho),
        owner_chirho: None,
        role_chirho: TypeConstructorRoleChirho::TypeFamilyChirho,
        declaration_chirho: None,
        binders_chirho: Vec::new(),
        source_variables_chirho: Vec::new(),
        result_kind_chirho: TyChirho::ConChirho("TypeChirho".to_owned()),
        synonym_rhs_chirho: None,
        visible_arity_chirho: 0,
        reduction_arity_chirho: 0,
        promoted_chirho: false,
        held_chirho: false,
    }
}

#[test]
fn a_foreign_unpublished_telescope_never_partially_merges_chirho() {
    let mut target_chirho = DeclarationSchemesChirho::default();
    target_chirho
        .insert_chirho(header_chirho("existingChirho"))
        .unwrap();
    let mut incoming_chirho = DeclarationSchemesChirho::default();
    incoming_chirho.stage_chirho(header_chirho("otherwiseReadyChirho"));
    let mut unfinished_chirho = header_chirho("unfinishedChirho");
    unfinished_chirho.owner_chirho = Some(ElaborationOwnerChirho(17));
    incoming_chirho.stage_chirho(unfinished_chirho);
    assert!(
        target_chirho
            .merge_chirho(&incoming_chirho)
            .unwrap_err()
            .contains("unpublished")
    );
    assert!(target_chirho.get_chirho("existingChirho").is_some());
    assert!(target_chirho.get_chirho("otherwiseReadyChirho").is_none());
    assert!(target_chirho.get_chirho("unfinishedChirho").is_none());
}

#[test]
fn an_ownerless_free_variable_cannot_be_published_chirho() {
    let mut header_chirho = header_chirho("freeChirho");
    header_chirho.result_kind_chirho = TyChirho::VarChirho(TyVarChirho(23));
    assert!(
        DeclarationSchemesChirho::default()
            .insert_chirho(header_chirho)
            .unwrap_err()
            .contains("unquantified")
    );
}

#[test]
fn a_source_binder_maps_through_an_inferred_prefix_chirho() {
    let mut header_chirho = header_chirho("mappedChirho");
    header_chirho.binders_chirho = vec![
        DeclarationBinderChirho {
            variable_chirho: TyVarChirho(0),
            kind_chirho: TyChirho::ConChirho("TypeChirho".to_owned()),
            visibility_chirho: TelescopeVisibilityChirho::InferredChirho,
        },
        DeclarationBinderChirho {
            variable_chirho: TyVarChirho(1),
            kind_chirho: TyChirho::VarChirho(TyVarChirho(0)),
            visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
        },
    ];
    header_chirho.source_variables_chirho = vec![SourceDeclarationVariableChirho {
        source_chirho: ScopedTypeVariableIdChirho::QuantifiedChirho {
            declaration_chirho: Arc::from("sourceChirho"),
            index_chirho: 0,
        },
        variable_chirho: TyVarChirho(1),
        specificity_chirho: TyVarSpecificityChirho::SpecifiedChirho,
    }];
    header_chirho.visible_arity_chirho = 1;
    header_chirho.reduction_arity_chirho = 2;
    assert!(
        DeclarationSchemesChirho::default()
            .insert_chirho(header_chirho)
            .is_ok()
    );
}

#[test]
fn result_arrows_never_add_reduction_parameters_chirho() {
    let mut header_chirho = header_chirho("arrowChirho");
    header_chirho.result_kind_chirho = TyChirho::FunChirho(
        Box::new(TyChirho::ConChirho("TypeChirho".to_owned())),
        Box::new(TyChirho::ConChirho("TypeChirho".to_owned())),
        super::super::MultChirho::ManyChirho,
    );
    let mut headers_chirho = DeclarationSchemesChirho::default();
    headers_chirho.insert_chirho(header_chirho.clone()).unwrap();
    header_chirho.reduction_arity_chirho = 1;
    assert!(
        DeclarationSchemesChirho::default()
            .insert_chirho(header_chirho)
            .unwrap_err()
            .contains("boundary")
    );
}
