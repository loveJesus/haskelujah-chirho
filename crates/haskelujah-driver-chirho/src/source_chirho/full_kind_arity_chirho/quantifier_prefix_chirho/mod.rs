// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written result quantifiers keep complete kinds outside the declaration cursor.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{DeclChirho, check_chirho};

#[test]
fn required_result_binder_kinds_must_agree_with_the_standalone_contract_chirho() {
    let error_chirho = check_chirho(include_str!("WrongGroupedResultPrefixChirho.hs"), &[])
        .err()
        .expect("fresh GHC rejection: RuntimeRep cannot replace the result binder's Type kind");
    assert!(error_chirho.to_string().contains("error[E0300]"));
}

#[test]
fn an_inferred_binder_retains_its_complete_written_kind_chirho() {
    if let Err(error_chirho) = check_chirho(include_str!("InferredBinderKindChirho.hs"), &[]) {
        panic!("fresh GHC-positive inferred binder kind: {error_chirho}");
    }
}

#[test]
fn a_required_quantifier_group_retains_its_result_suffix_chirho() {
    let frontend_chirho = check_chirho(include_str!("GroupedResultPrefixChirho.hs"), &[])
        .unwrap_or_else(|error_chirho| panic!("fresh GHC-positive grouped result: {error_chirho}"));
    let declaration_chirho = frontend_chirho
        .module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| match declaration_chirho {
            DeclChirho::NewtypeDeclChirho { name_chirho, .. } => {
                name_chirho.type_constructor_chirho()
            }
            _ => None,
        })
        .expect("checked source newtype producer");
    let catalogue_chirho = frontend_chirho
        .infer_result_chirho
        .type_families_chirho
        .declaration_schemes_chirho();
    let header_chirho = catalogue_chirho
        .get_chirho(&declaration_chirho.type_key_chirho())
        .expect("the actual producer publishes its complete kind");
    assert_eq!(header_chirho.binders_chirho.len(), 1, "{header_chirho:?}");
    assert_eq!(header_chirho.visible_arity_chirho, 1);
    assert_eq!(header_chirho.reduction_arity_chirho, 1);
    let haskelujah_typing_chirho::TyChirho::RequiredForallChirho { vars_chirho, .. } =
        &header_chirho.result_kind_chirho
    else {
        panic!("the result suffix must retain its required quantifier");
    };
    assert_eq!(vars_chirho.len(), 1);
    assert_eq!(
        vars_chirho.record_chirho(0).unwrap().kind_chirho.as_deref(),
        Some(&haskelujah_typing_chirho::kind_chirho::elaboration_chirho::ElaborationStateChirho::type_universe_chirho())
    );
    assert_eq!(
        header_chirho.result_kind_chirho.free_vars_chirho(),
        vec![header_chirho.binders_chirho[0].variable_chirho]
    );
}
