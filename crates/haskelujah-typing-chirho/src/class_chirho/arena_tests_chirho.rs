// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Internal substrate controls, not GHC-output oracle claims.
use super::*;
use haskelujah_ast_chirho::name_chirho::{
    CompilerClassChirho, PredicateClassIdentityChirho, PredicateClassProofChirho,
};
use std::collections::HashMap;
use std::sync::Arc;

use crate::ty_chirho::{TyChirho, TyVarChirho};

fn class_chirho(name_chirho: &str) -> ClassDeclChirho {
    ClassDeclChirho::legacy_chirho(ClassDeclPartsChirho {
        kind_vars_chirho: vec![],
        name_chirho: name_chirho.to_owned(),
        supers_chirho: vec![],
        var_chirho: TyVarChirho(0),
        methods_chirho: HashMap::new(),
        extra_vars_chirho: vec![],
        fundeps_chirho: vec![],
        defaults_chirho: HashMap::new(),
    })
}
fn instance_chirho(name_chirho: &str, ty_chirho: TyChirho) -> InstDeclChirho {
    InstDeclChirho::legacy_chirho(InstDeclPartsChirho {
        kind_tys_chirho: vec![],
        class_name_chirho: name_chirho.to_owned(),
        head_ty_chirho: ty_chirho,
        extra_head_tys_chirho: vec![],
        context_chirho: vec![],
    })
}

#[test]
fn spelling_overwrite_conserves_instances_and_present_empty_keys_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.add_class_chirho(class_chirho("EmptyChirho"));
    env_chirho.add_instance_chirho(instance_chirho("CChirho", TyChirho::int_chirho()));
    env_chirho.add_class_chirho(class_chirho("CChirho"));
    env_chirho.add_class_chirho(class_chirho("CChirho"));
    assert!(
        env_chirho
            .instances_chirho()
            .get_chirho("AbsentChirho")
            .is_none()
    );
    assert!(
        env_chirho
            .instances_chirho()
            .get_chirho("EmptyChirho")
            .expect("class exists")
            .is_empty_chirho()
    );
    assert_eq!(
        env_chirho
            .instances_chirho()
            .get_chirho("CChirho")
            .expect("instance exists")
            .first_chirho()
            .expect("first")
            .head_ty_chirho,
        TyChirho::int_chirho()
    );
}

#[test]
fn shared_records_are_interned_once_without_dropping_instance_duplicates_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    let declaration_chirho = Arc::new(class_chirho("CChirho"));
    let first_chirho = env_chirho
        .add_class_record_chirho(Arc::clone(&declaration_chirho))
        .expect("first registration");
    let second_chirho = env_chirho
        .add_class_record_chirho(Arc::clone(&declaration_chirho))
        .expect("same record");
    assert_eq!(first_chirho, second_chirho);
    assert!(Arc::ptr_eq(
        env_chirho
            .class_record_chirho(&first_chirho)
            .expect("record"),
        &declaration_chirho
    ));
    let instance_chirho = Arc::new(instance_chirho("CChirho", TyChirho::int_chirho()));
    env_chirho.add_shared_instance_chirho(Arc::clone(&instance_chirho));
    env_chirho.add_shared_instance_chirho(Arc::clone(&instance_chirho));
    let values_chirho: Vec<_> = env_chirho
        .instances_chirho()
        .get_chirho("CChirho")
        .expect("class")
        .into_iter()
        .collect();
    assert_eq!(values_chirho.len(), 2);
    assert!(
        values_chirho
            .into_iter()
            .all(|value_chirho| Arc::ptr_eq(value_chirho, &instance_chirho))
    );
    assert_eq!(env_chirho.instance_records_chirho.len(), 1);
}

#[test]
fn clone_remaps_local_ids_and_conserves_table_order_and_payloads_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    for name_chirho in ["CChirho", "BChirho", "AChirho"] {
        env_chirho.add_class_chirho(class_chirho(name_chirho));
    }
    let old_chirho = env_chirho.class_names_chirho["CChirho"].clone();
    let mut copy_chirho = env_chirho.try_snapshot_chirho().expect("small snapshot");
    assert!(copy_chirho.class_record_chirho(&old_chirho).is_none());
    assert_eq!(
        env_chirho
            .classes_chirho()
            .keys_chirho()
            .collect::<Vec<_>>(),
        copy_chirho
            .classes_chirho()
            .keys_chirho()
            .collect::<Vec<_>>()
    );
    let copied_chirho = &copy_chirho.class_names_chirho["CChirho"];
    assert_ne!(&old_chirho, copied_chirho);
    assert!(Arc::ptr_eq(
        env_chirho
            .class_record_chirho(&old_chirho)
            .expect("old record"),
        copy_chirho
            .class_record_chirho(copied_chirho)
            .expect("copied record")
    ));
    env_chirho.add_class_chirho(class_chirho("LeftChirho"));
    copy_chirho.add_class_chirho(class_chirho("RightChirho"));
    let foreign_chirho = &copy_chirho.class_names_chirho["RightChirho"];
    assert!(env_chirho.class_record_chirho(foreign_chirho).is_none());
}

#[test]
fn actual_seed_records_own_compiler_proof_even_when_spelling_is_shadowed_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();
    for (name_chirho, declaration_chirho) in env_chirho.classes_chirho().iter_chirho() {
        let PredicateClassProofChirho::ProvenChirho(PredicateClassIdentityChirho::CompilerChirho(
            identity_chirho,
        )) = declaration_chirho.class_identity_chirho()
        else {
            panic!("a real class seed lacked its typed compiler identity: {name_chirho}");
        };
        assert_eq!(identity_chirho.legacy_name_chirho(), name_chirho);
    }
    env_chirho.add_class_chirho(class_chirho("Num"));
    assert!(
        env_chirho
            .classes_chirho()
            .get_chirho("Num")
            .expect("shadow")
            .class_identity_chirho()
            .identity_chirho()
            .is_none()
    );
    assert!(
        env_chirho
            .class_record_by_identity_chirho(&PredicateClassIdentityChirho::CompilerChirho(
                CompilerClassChirho::NumChirho
            ))
            .is_some()
    );
}

#[test]
fn distinct_records_with_one_claimed_identity_do_not_choose_by_order_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();
    let identity_chirho =
        PredicateClassIdentityChirho::CompilerChirho(CompilerClassChirho::NumChirho);
    let old_chirho = env_chirho
        .class_record_by_identity_chirho(&identity_chirho)
        .expect("seed")
        .as_ref()
        .clone();
    env_chirho.add_class_chirho(old_chirho);
    assert!(
        env_chirho
            .class_record_by_identity_chirho(&identity_chirho)
            .is_none()
    );
}

#[test]
fn bounded_registration_and_clone_retain_named_failure_without_partial_views_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.remaining_work_chirho = 0;
    env_chirho.add_class_chirho(class_chirho("CChirho"));
    assert_eq!(
        env_chirho.registry_error_chirho(),
        Some(ClassRegistryErrorChirho::WorkBoundChirho)
    );
    assert!(env_chirho.classes_chirho().is_empty_chirho());
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.add_class_chirho(class_chirho("CChirho"));
    env_chirho.remaining_work_chirho = 0;
    assert!(matches!(
        env_chirho.try_snapshot_chirho(),
        Err(ClassRegistryErrorChirho::WorkBoundChirho)
    ));
    assert!(env_chirho.classes_chirho().contains_key_chirho("CChirho"));
}
