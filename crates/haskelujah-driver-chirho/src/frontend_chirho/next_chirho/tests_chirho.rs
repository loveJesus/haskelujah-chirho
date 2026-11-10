// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Execute the complete next path and reject invalid source before Core publication.

use super::{NextErrorKindChirho, check_source_chirho, compile_source_chirho, eval_binding_chirho};
use haskelujah_runtime_chirho::ValueChirho;

const CHOICE_CHIRHO: &str = "module NextChoiceChirho where
chooseChirho :: Bool -> Char -> Char -> Char
chooseChirho flagChirho leftChirho rightChirho = if flagChirho then leftChirho else rightChirho
resultChirho = chooseChirho True 'x' 'y'
";

#[test]
fn checked_hir_executes_through_core_and_the_existing_machine_chirho() {
    let (value_chirho, machine_chirho) =
        eval_binding_chirho(CHOICE_CHIRHO, "resultChirho", 10_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('x'));
    assert!(machine_chirho.io_output_chirho.is_empty());
}

#[test]
fn lexical_shadowing_uses_ids_and_unused_recursion_remains_lazy_chirho() {
    let source_chirho = "module NextShadowChirho where
loopChirho = loopChirho
pickChirho xChirho = (\\xChirho -> xChirho) 'b'
resultChirho = pickChirho 'a'
";
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 10_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('b'));
}

#[test]
fn recursive_local_bindings_and_annotations_are_elaborated_chirho() {
    let source_chirho = "module NextLocalChirho where
resultChirho = let { firstChirho = secondChirho; secondChirho = 'q' } in (firstChirho :: Char)
";
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 10_000).unwrap();
    assert_eq!(value_chirho, ValueChirho::CharChirho('q'));
}

#[test]
fn inconsistent_branches_and_infinite_types_never_publish_core_chirho() {
    for source_chirho in [
        "module NextBadChirho where\nbadChirho = if True then 'x' else False\n",
        "module NextBadChirho where\nbadChirho xChirho = xChirho xChirho\n",
        "module NextBadChirho where\nbadChirho :: Bool\nbadChirho = 'x'\n",
        "module NextBadChirho where\nbadChirho = ['x', True]\n",
    ] {
        assert_eq!(
            compile_source_chirho(source_chirho)
                .unwrap_err()
                .kind_chirho,
            NextErrorKindChirho::TypeChirho,
            "{source_chirho}"
        );
    }
}

#[test]
fn duplicate_parameters_are_rejected_but_nested_shadowing_is_legal_chirho() {
    let source_chirho = "module NextDuplicateChirho where\nbadChirho xChirho xChirho = xChirho\n";
    assert_eq!(
        check_source_chirho(source_chirho).unwrap_err().kind_chirho,
        NextErrorKindChirho::InvalidChirho
    );
}

#[test]
fn parser_recovery_is_never_an_acceptance_chirho() {
    for source_chirho in [
        "module NextSyntaxChirho where\nresultChirho = if True then 'x'\n",
        "module NextSyntaxChirho where\nresultChirho = ('x'\n",
        "module NextSyntaxChirho where\nresultChirho = \"unterminated\n",
    ] {
        assert!(
            check_source_chirho(source_chirho).is_err(),
            "{source_chirho}"
        );
    }
}

#[test]
fn missing_interfaces_and_numeric_classes_do_not_get_placeholder_types_chirho() {
    for source_chirho in [
        "module NextUnknownChirho where\nimport MissingChirho\nresultChirho = unknownChirho\n",
        "module NextUnknownChirho where\nresultChirho = 42\n",
        "module NextUnknownChirho where\nresultChirho = print True\n",
    ] {
        assert_eq!(
            check_source_chirho(source_chirho).unwrap_err().kind_chirho,
            NextErrorKindChirho::UnsupportedChirho
        );
    }
}

#[test]
fn main_requires_the_real_io_contract_and_library_entry_is_explicit_chirho() {
    assert_eq!(
        check_source_chirho("main = True\n")
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    assert_eq!(
        eval_binding_chirho(CHOICE_CHIRHO, "absentChirho", 1_000)
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::InvalidChirho
    );
}

#[test]
fn unchecked_pragma_bodies_and_export_lists_are_never_silently_accepted_chirho() {
    for source_chirho in [
        "module RulesChirho where\n{-# RULES \"badChirho\" forall (fChirho :: aChirho -> _). fChirho = fChirho #-}\n",
        "module ExportChirho (missingChirho) where\nvalueChirho = 'x'\n",
        "module ImportChirho where\nimport \"absent-unit-chirho\" Prelude\nvalueChirho = True\n",
    ] {
        assert_eq!(
            check_source_chirho(source_chirho).unwrap_err().kind_chirho,
            NextErrorKindChirho::UnsupportedChirho
        );
    }
}
