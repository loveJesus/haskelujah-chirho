// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Portable contracts must close identities before module boundaries.
use super::*;

fn dependent_contract_chirho() -> KindContractChirho {
    KindContractChirho {
        binding_chirho: KindBindingChirho::PolyChirho(KindSchemeChirho {
            quantified_chirho: vec![KindVarChirho(0), KindVarChirho(1)],
            specified_chirho: [KindVarChirho(1)].into_iter().collect(),
            classifiers_chirho: vec![
                KindChirho::StarChirho,
                KindChirho::VarChirho(KindVarChirho(0)),
            ],
            source_names_chirho: vec![None, Some("aChirho".to_owned())],
            body_chirho: KindChirho::arrow_chirho(
                KindChirho::VarChirho(KindVarChirho(1)),
                KindChirho::StarChirho,
            ),
        }),
        shape_chirho: imports_chirho::KindHeadShapeChirho::NominalChirho,
    }
}

#[test]
fn portable_kind_templates_reject_free_and_forward_classifier_identities_chirho() {
    let contract_chirho = dependent_contract_chirho();
    assert!(KindContractChirho::binding_is_closed_chirho(
        &contract_chirho.binding_chirho
    ));
    assert!(!KindContractChirho::binding_is_closed_chirho(
        &KindBindingChirho::MonoChirho(KindChirho::VarChirho(KindVarChirho(0)))
    ));
    for identity_chirho in [KindVarChirho(1), KindVarChirho(99)] {
        let mut malformed_chirho = contract_chirho.binding_chirho.clone();
        let KindBindingChirho::PolyChirho(scheme_chirho) = &mut malformed_chirho else {
            unreachable!()
        };
        scheme_chirho.classifiers_chirho[0] = KindChirho::VarChirho(identity_chirho);
        assert!(!KindContractChirho::binding_is_closed_chirho(
            &malformed_chirho
        ));
    }
}

#[test]
fn imported_kind_templates_rebase_dependencies_and_keep_specificity_chirho() {
    let mut context_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
    context_chirho.seed_imported_kind_contracts_chirho(
        &[
            ("AChirho".to_owned(), dependent_contract_chirho()),
            ("BChirho".to_owned(), dependent_contract_chirho()),
        ]
        .into_iter()
        .collect(),
    );
    let binding_chirho = |name_chirho| match context_chirho
        .env_chirho
        .lookup_binding_chirho(name_chirho)
        .unwrap()
    {
        KindBindingChirho::PolyChirho(scheme_chirho) => scheme_chirho,
        KindBindingChirho::MonoChirho(_) => panic!("a checked scheme lost its quantifiers"),
    };
    let left_chirho = binding_chirho("AChirho");
    let right_chirho = binding_chirho("BChirho");
    assert!(
        left_chirho
            .quantified_chirho
            .iter()
            .all(|identity_chirho| !right_chirho.quantified_chirho.contains(identity_chirho))
    );
    for scheme_chirho in [left_chirho, right_chirho] {
        assert_eq!(
            scheme_chirho.classifiers_chirho[1],
            KindChirho::VarChirho(scheme_chirho.quantified_chirho[0])
        );
        assert!(
            scheme_chirho
                .specified_chirho
                .contains(&scheme_chirho.quantified_chirho[1])
        );
        assert!(
            !scheme_chirho
                .specified_chirho
                .contains(&scheme_chirho.quantified_chirho[0])
        );
    }
}
