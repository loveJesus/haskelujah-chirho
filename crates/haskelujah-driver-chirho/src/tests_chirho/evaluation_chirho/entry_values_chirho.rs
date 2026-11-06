// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! What the evaluator API returns for an entry (row 692, gpt_chirho #32492).
//!
//! The API runs an IO entry's action and returns its result. Any other entry is returned in
//! weak head normal form, exactly as it is. The value is never deep-forced, never applied to
//! anything, and a heap object is never unwrapped.
//!
//! So a function entry stays a function closure, a data entry stays a constructor with its
//! fields unevaluated, and an action entry yields what it returns. A monomorphic numeric
//! entry is a number only because typing made it monomorphic (the monomorphism restriction),
//! never because the API guessed a dictionary.
//! workflow: language-features-chirho/fundep-generalization-chirho

use haskelujah_runtime_chirho::ValueChirho;
use haskelujah_runtime_chirho::eval_chirho::MachineChirho;
use haskelujah_runtime_chirho::value_chirho::InfoTagChirho;
use haskelujah_span_chirho::SourceMapChirho;

use crate::eval_source_with_machine_chirho;

/// Evaluate `source_chirho`'s `main` through the public API.
fn entry_value_chirho(file_name_chirho: &str, source_chirho: &str) -> (ValueChirho, MachineChirho) {
    eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
        None,
    )
    .unwrap_or_else(|error_chirho| panic!("{file_name_chirho}: {error_chirho}"))
}

/// The tag of the heap object an entry evaluated to; any other value fails the caller's claim.
fn entry_tag_chirho(file_name_chirho: &str, source_chirho: &str) -> InfoTagChirho {
    let (value_chirho, machine_chirho) = entry_value_chirho(file_name_chirho, source_chirho);
    let ValueChirho::HeapPtrChirho(address_chirho) = value_chirho else {
        panic!("{file_name_chirho}: expected a heap object, got {value_chirho:?}");
    };
    machine_chirho
        .heap_chirho
        .read_chirho(address_chirho)
        .info_chirho
        .tag_chirho
}

#[test]
fn a_function_entry_stays_a_function_closure_chirho() {
    assert_eq!(
        entry_tag_chirho("FunctionEntryChirho.hs", "module Test where\nmain = id\n"),
        InfoTagChirho::FunChirho
    );
}

#[test]
fn a_data_entry_is_a_constructor_with_unforced_fields_chirho() {
    // Restricted: `2 + 3` defaults to Integer, `undefined` stays unforced in its field.
    assert_eq!(
        entry_tag_chirho(
            "DataEntryChirho.hs",
            "module Test where\nmain = (2 + 3, undefined)\n"
        ),
        InfoTagChirho::ConChirho
    );
}

#[test]
fn an_action_entry_runs_and_yields_its_result_chirho() {
    let (value_chirho, machine_chirho) = entry_value_chirho(
        "ActionEntryChirho.hs",
        "module Test where\nmain = print 1 >> return 5\n",
    );
    assert_eq!(machine_chirho.io_output_chirho, "1\n");
    assert_eq!(value_chirho, ValueChirho::IntChirho(5));
}
