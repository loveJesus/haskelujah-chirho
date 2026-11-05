// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho;
use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::{
    ElaborationOwnerChirho, ScopedTypeVariableIdChirho,
};

fn header_chirho(identity_chirho: &str) -> DeclarationSchemeChirho {
    DeclarationSchemeChirho {
        injectivity_chirho: None,
        association_chirho:
            crate::ty_chirho::declarations_chirho::DeclarationAssociationChirho::NoneChirho,
        identity_chirho: Arc::from(identity_chirho),
        owner_chirho: None,
        role_chirho: TypeConstructorRoleChirho::TypeFamilyChirho,
        declaration_chirho: None,
        binders_chirho: Vec::new(),
        source_variables_chirho: Vec::new(),
        result_kind_chirho: TyChirho::ConChirho("TypeChirho".to_owned()),
        synonym_rhs_chirho: None,
        synonym_syntax_chirho: None,
        local_binders_chirho: std::sync::Arc::from([]),
        visible_arity_chirho: 0,
        reduction_arity_chirho: 0,
        promoted_chirho: None,
        held_chirho: false,
    }
}

#[test]
fn promotion_witness_must_name_the_same_published_declaration_chirho() {
    use haskelujah_ast_chirho::promotion_chirho::PromotedKeyChirho;
    let mut target_chirho = DeclarationSchemesChirho::default();
    target_chirho
        .insert_chirho(header_chirho("existingChirho"))
        .unwrap();
    for key_chirho in [
        PromotedKeyChirho::builtin_nil_chirho(),
        PromotedKeyChirho::unknown_chirho("", "UnknownChirho"),
    ] {
        let mut invalid_chirho = header_chirho("differentChirho");
        invalid_chirho.promoted_chirho = Some(key_chirho);
        assert!(
            target_chirho
                .insert_chirho(invalid_chirho)
                .unwrap_err()
                .contains("promotion witness")
        );
        assert!(target_chirho.get_chirho("differentChirho").is_none());
        assert!(target_chirho.get_chirho("existingChirho").is_some());
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
        header_index_chirho: 0,
        source_chirho: ScopedTypeVariableIdChirho::QuantifiedChirho {
            declaration_chirho: header_chirho.identity_chirho.clone(),
            index_chirho: 1,
        },
        variable_chirho: TyVarChirho(1),
        specificity_chirho: TyVarSpecificityChirho::SpecifiedChirho,
    }];
    header_chirho.visible_arity_chirho = 1;
    header_chirho.reduction_arity_chirho = 2;
    assert!(
        DeclarationSchemesChirho::default()
            .insert_chirho(header_chirho.clone())
            .is_ok()
    );
    // Written parameter zero is canonical slot one after its inferred kind.
    header_chirho.source_variables_chirho[0].source_chirho =
        ScopedTypeVariableIdChirho::QuantifiedChirho {
            declaration_chirho: header_chirho.identity_chirho.clone(),
            index_chirho: 0,
        };
    assert!(
        DeclarationSchemesChirho::default()
            .insert_chirho(header_chirho)
            .unwrap_err()
            .contains("source-to-published")
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

#[test]
fn associated_owner_and_default_slots_are_validated_before_publication_chirho() {
    let mut class_chirho = header_chirho("classChirho");
    class_chirho.role_chirho = TypeConstructorRoleChirho::ClassChirho;
    class_chirho.binders_chirho.push(DeclarationBinderChirho {
        variable_chirho: TyVarChirho(0),
        kind_chirho: TyChirho::ConChirho("TypeChirho".to_owned()),
        visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
    });
    class_chirho.visible_arity_chirho = 1;
    class_chirho.reduction_arity_chirho = 1;
    let mut family_chirho = class_chirho.clone();
    family_chirho.identity_chirho = Arc::from("familyChirho");
    family_chirho.role_chirho = TypeConstructorRoleChirho::TypeFamilyChirho;
    let mut declarations_chirho = DeclarationSchemesChirho::default();
    declarations_chirho.insert_chirho(class_chirho).unwrap();
    declarations_chirho.insert_chirho(family_chirho).unwrap();
    let invalid_chirho = DeclarationAssociationChirho::FamilyChirho {
        class_chirho: Arc::from("classChirho"),
        correspondence_chirho: vec![(TyVarChirho(0), TyVarChirho(9))],
        default_chirho: None,
    };
    assert!(
        declarations_chirho
            .complete_association_chirho("familyChirho", invalid_chirho)
            .is_err()
    );
    assert_eq!(
        declarations_chirho
            .get_chirho("familyChirho")
            .unwrap()
            .association_chirho,
        DeclarationAssociationChirho::NoneChirho
    );
    let valid_chirho = DeclarationAssociationChirho::FamilyChirho {
        class_chirho: Arc::from("classChirho"),
        correspondence_chirho: vec![(TyVarChirho(0), TyVarChirho(0))],
        default_chirho: Some(AssociatedDefaultEquationChirho {
            lhs_chirho: vec![TyChirho::VarChirho(TyVarChirho(0))],
            rhs_chirho: TyChirho::VarChirho(TyVarChirho(0)),
        }),
    };
    declarations_chirho
        .complete_association_chirho("familyChirho", valid_chirho.clone())
        .unwrap();
    let mut invalid_chirho = valid_chirho.clone();
    if let DeclarationAssociationChirho::FamilyChirho {
        default_chirho: Some(default_chirho),
        ..
    } = &mut invalid_chirho
    {
        default_chirho.rhs_chirho = TyChirho::VarChirho(TyVarChirho(77));
    }
    assert!(
        declarations_chirho
            .complete_association_chirho("familyChirho", invalid_chirho)
            .is_err()
    );
    assert_eq!(
        declarations_chirho
            .get_chirho("familyChirho")
            .unwrap()
            .association_chirho,
        valid_chirho
    );
}

#[test]
fn local_binder_kinds_require_preceding_lexical_owners_chirho() {
    use super::DeclarationLocalBinderChirho;
    for invalid_chirho in [None, Some(TyVarChirho(1)), Some(TyVarChirho(2))] {
        let mut schema_chirho = header_chirho("localScopeChirho");
        schema_chirho.result_kind_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho(1), TyVarChirho(2)].into(),
            body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(2))),
        };
        schema_chirho.local_binders_chirho = std::sync::Arc::from([
            DeclarationLocalBinderChirho {
                binder_chirho: DeclarationBinderChirho {
                    variable_chirho: TyVarChirho(1),
                    kind_chirho: invalid_chirho
                        .map_or_else(|| TyChirho::ConChirho("Type".into()), TyChirho::VarChirho),
                    visibility_chirho: TelescopeVisibilityChirho::SpecifiedChirho,
                },
                specificity_chirho:
                    haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho,
            },
            DeclarationLocalBinderChirho {
                binder_chirho: DeclarationBinderChirho {
                    variable_chirho: TyVarChirho(2),
                    kind_chirho: TyChirho::VarChirho(TyVarChirho(1)),
                    visibility_chirho: TelescopeVisibilityChirho::SpecifiedChirho,
                },
                specificity_chirho:
                    haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho,
            },
        ]);
        if let TyChirho::ForallChirho { vars_chirho, .. } = &mut schema_chirho.result_kind_chirho {
            *vars_chirho = checked_group_chirho(
                &schema_chirho.identity_chirho,
                &schema_chirho.local_binders_chirho,
            );
        }
        let result_chirho = DeclarationSchemesChirho::default().insert_chirho(schema_chirho);
        assert_eq!(
            result_chirho.is_ok(),
            invalid_chirho.is_none(),
            "self/forward kinds must refuse; preceding dependent kinds must publish: {result_chirho:?}"
        );
    }
}

fn checked_group_chirho(
    identity_chirho: &std::sync::Arc<str>,
    binders_chirho: &[super::DeclarationLocalBinderChirho],
) -> crate::ty_chirho::QuantifiedVariablesChirho {
    crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(binders_chirho.iter().map(|record_chirho|
        crate::ty_chirho::QuantifiedVariableChirho {
            variable_chirho: record_chirho.binder_chirho.variable_chirho,
            kind_chirho: Some(std::sync::Arc::new(record_chirho.binder_chirho.kind_chirho.clone())),
            source_chirho: Some(haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho::QuantifiedChirho {
                declaration_chirho: identity_chirho.clone(), index_chirho: record_chirho.binder_chirho.variable_chirho.0,
            }),
            specificity_chirho: Some(record_chirho.specificity_chirho),
        }).collect())
}

#[test]
fn inline_quantifier_proof_drift_is_refused_before_first_publication_chirho() {
    use haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho;
    use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;
    let mut schema_chirho = header_chirho("inlineProofChirho");
    schema_chirho.local_binders_chirho =
        std::sync::Arc::from([super::DeclarationLocalBinderChirho {
            binder_chirho: DeclarationBinderChirho {
                variable_chirho: TyVarChirho(1),
                kind_chirho: TyChirho::ConChirho("Type".into()),
                visibility_chirho: TelescopeVisibilityChirho::SpecifiedChirho,
            },
            specificity_chirho: TyVarSpecificityChirho::SpecifiedChirho,
        }]);
    let records_chirho = checked_group_chirho(
        &schema_chirho.identity_chirho,
        &schema_chirho.local_binders_chirho,
    );
    schema_chirho.result_kind_chirho = TyChirho::ForallChirho {
        vars_chirho: records_chirho.clone(),
        body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(1))),
    };
    DeclarationSchemesChirho::default()
        .insert_chirho(schema_chirho.clone())
        .expect("actual producer agrees");
    for mutation_chirho in 0..4 {
        let mut invalid_chirho = schema_chirho.clone();
        let mut record_chirho = records_chirho.records_chirho()[0].clone();
        match mutation_chirho {
            0 => record_chirho.kind_chirho = None,
            1 => record_chirho.source_chirho = None,
            2 => {
                record_chirho.source_chirho = Some(ScopedTypeVariableIdChirho::QuantifiedChirho {
                    declaration_chirho: "foreignChirho".into(),
                    index_chirho: 1,
                })
            }
            _ => record_chirho.specificity_chirho = Some(TyVarSpecificityChirho::InferredChirho),
        }
        let invalid_group_chirho =
            crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(vec![record_chirho]);
        if mutation_chirho != 0 {
            assert_eq!(
                records_chirho, invalid_group_chirho,
                "ordinary equality intentionally ignores proof flags"
            );
        }
        if let TyChirho::ForallChirho { vars_chirho, .. } = &mut invalid_chirho.result_kind_chirho {
            *vars_chirho = invalid_group_chirho;
        }
        let mut registry_chirho = DeclarationSchemesChirho::default();
        assert!(registry_chirho.insert_chirho(invalid_chirho).is_err());
        assert!(registry_chirho.is_empty_chirho());
    }
}
