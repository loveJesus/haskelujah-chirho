// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
use super::*;
fn method_chirho(name_chirho: &str) -> MinimalFormulaChirho {
    MinimalFormulaChirho::MethodChirho(name_chirho.to_owned())
}
#[test]
fn boot_minimal_implication_is_semantic_and_directional_chirho() {
    let first_chirho = method_chirho("firstChirho");
    let second_chirho = method_chirho("secondChirho");
    let all_chirho =
        MinimalFormulaChirho::AllChirho(vec![first_chirho.clone(), second_chirho.clone()]);
    let any_chirho =
        MinimalFormulaChirho::AnyChirho(vec![first_chirho.clone(), second_chirho.clone()]);
    assert_eq!(implies_chirho(&all_chirho, &first_chirho), Ok(true));
    assert_eq!(implies_chirho(&any_chirho, &first_chirho), Ok(false));
    assert_eq!(implies_chirho(&first_chirho, &any_chirho), Ok(true));
    assert_eq!(
        implies_chirho(
            &any_chirho,
            &MinimalFormulaChirho::AnyChirho(vec![second_chirho, first_chirho])
        ),
        Ok(true)
    );
    assert_eq!(
        implies_chirho(&MinimalFormulaChirho::AllChirho(vec![]), &all_chirho),
        Ok(false)
    );
}
#[test]
fn boot_minimal_invalid_and_exhausted_are_not_proofs_chirho() {
    let valid_chirho = method_chirho("validChirho");
    assert!(implies_chirho(&MinimalFormulaChirho::InvalidChirho, &valid_chirho).is_err());
    assert!(validate_chirho(&valid_chirho, &BTreeSet::new()).is_err());
    let excessive_chirho =
        MinimalFormulaChirho::AllChirho(vec![valid_chirho.clone(); LIMIT_CHIRHO + 1]);
    assert!(implies_chirho(&excessive_chirho, &excessive_chirho).is_err());
    assert!(
        search_chirho(
            &valid_chirho,
            &valid_chirho,
            &["validChirho".to_owned()],
            &mut HashMap::new(),
            0,
            &mut 0
        )
        .is_err()
    );
}

#[test]
fn boot_minimal_empty_requirement_short_circuits_large_method_sets_chirho() {
    let methods_chirho: BTreeSet<_> = (0..16)
        .map(|index_chirho| format!("method{index_chirho}Chirho"))
        .collect();
    let promised_chirho = MinimalFormulaChirho::AnyChirho(
        methods_chirho
            .iter()
            .map(|name_chirho| method_chirho(name_chirho))
            .collect(),
    );
    assert_eq!(validate_chirho(&promised_chirho, &methods_chirho), Ok(()));
    let required_chirho = MinimalFormulaChirho::AllChirho(vec![]);
    assert_eq!(implies_chirho(&promised_chirho, &required_chirho), Ok(true));
    // A false implication must still be found when one implementation method
    // has no default. This cannot be satisfied by the true short-circuit.
    assert_eq!(
        implies_chirho(&promised_chirho, &method_chirho("method0Chirho")),
        Ok(false)
    );
}
