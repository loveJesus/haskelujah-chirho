// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Exhausting agreement work is an error, never partial publication.
use super::*;

#[test]
fn boot_first_mismatch_uses_stable_source_names_chirho() {
    for _round_chirho in 0..32 {
        let mut boot_chirho = DeclarationContractsChirho::default();
        for name_chirho in ["ZChirho", "AChirho"] {
            boot_chirho.kinds_chirho.insert(
                name_chirho.to_owned(),
                KindContractChirho::monomorphic_nominal_chirho(
                    crate::kind_chirho::KindChirho::StarChirho,
                ),
            );
        }
        let error_chirho = boot_chirho
            .check_boot_promises_chirho(
                &DeclarationContractsChirho::default(),
                &boot_chirho.kinds_chirho.keys().cloned().collect(),
            )
            .expect_err("neither type has a local implementation");
        assert!(
            error_chirho.contains("AChirho") && !error_chirho.contains("ZChirho"),
            "{error_chirho}"
        );
    }
}

#[test]
fn boot_instance_search_exhaustion_never_counts_as_agreement_chirho() {
    let promise_chirho =
        SchemeChirho::mono_chirho(TyChirho::TupleChirho(vec![TyChirho::ConChirho(
            "ExpectedChirho".to_owned(),
        )]));
    let wrong_chirho = SchemeChirho::mono_chirho(TyChirho::TupleChirho(vec![TyChirho::ConChirho(
        "OtherChirho".to_owned(),
    )]));
    let mut boot_chirho = DeclarationContractsChirho::default();
    boot_chirho
        .instances_chirho
        .insert("ClassChirho".to_owned(), vec![promise_chirho.clone()]);
    let mut implementation_chirho = DeclarationContractsChirho::default();
    let mut candidates_chirho = vec![wrong_chirho.clone(); INSTANCE_COMPARISON_LIMIT_CHIRHO - 1];
    candidates_chirho.push(promise_chirho);
    implementation_chirho
        .instances_chirho
        .insert("ClassChirho".to_owned(), candidates_chirho);
    boot_chirho
        .check_boot_promises_chirho(&implementation_chirho, &HashSet::new())
        .unwrap();
    implementation_chirho
        .instances_chirho
        .get_mut("ClassChirho")
        .unwrap()
        .insert(0, wrong_chirho);
    let error_chirho = boot_chirho
        .check_boot_promises_chirho(&implementation_chirho, &HashSet::new())
        .expect_err("an unseen final match cannot satisfy the promise");
    assert!(
        error_chirho.contains("comparison budget exhausted"),
        "{error_chirho}"
    );
}
