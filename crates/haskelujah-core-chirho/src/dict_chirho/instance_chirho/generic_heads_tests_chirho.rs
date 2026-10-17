// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The guards of a generic instance head: the typed spine (G3) and the
//! written spelling (G5). Each refusal here is a shape where keying the
//! dictionary by its constructor could answer for the wrong instance.

use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::{distinct_variable_spine_chirho, written_spine_chirho};

fn con_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ConChirho(name_chirho.to_string())
}

fn var_chirho(number_chirho: u32) -> TyChirho {
    TyChirho::VarChirho(TyVarChirho(number_chirho))
}

fn app_chirho(function_chirho: TyChirho, argument_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(Box::new(function_chirho), Box::new(argument_chirho))
}

#[test]
fn a_typed_spine_is_a_constructor_over_distinct_variables_chirho() {
    let pair_chirho = |left_chirho, right_chirho| {
        app_chirho(
            app_chirho(con_chirho("PairBoxChirho"), left_chirho),
            right_chirho,
        )
    };
    assert_eq!(
        distinct_variable_spine_chirho(&app_chirho(con_chirho("SBoolChirho"), var_chirho(0))),
        Some(("SBoolChirho", 1))
    );
    assert_eq!(
        distinct_variable_spine_chirho(&pair_chirho(var_chirho(0), var_chirho(1))),
        Some(("PairBoxChirho", 2))
    );
    // A repeated variable is not a bijection.
    assert_eq!(
        distinct_variable_spine_chirho(&pair_chirho(var_chirho(0), var_chirho(0))),
        None
    );
    // A specialized argument.
    assert_eq!(
        distinct_variable_spine_chirho(&app_chirho(con_chirho("BoxChirho"), con_chirho("Int"))),
        None
    );
    // No argument: the typed key is already the constructor.
    assert_eq!(
        distinct_variable_spine_chirho(&con_chirho("MonoChirho")),
        None
    );
    // A variable head, and a list, are outside this rule.
    assert_eq!(
        distinct_variable_spine_chirho(&app_chirho(var_chirho(1), var_chirho(0))),
        None
    );
    assert_eq!(
        distinct_variable_spine_chirho(&TyChirho::ListChirho(Box::new(var_chirho(0)))),
        None
    );
}

#[test]
fn a_written_spine_is_the_constructor_then_distinct_variables_chirho() {
    assert!(written_spine_chirho("SBoolChirho b", "SBoolChirho", 1));
    assert!(written_spine_chirho(
        "PairBoxChirho a' b_2",
        "PairBoxChirho",
        2
    ));
    // A repeated variable is not a bijection.
    assert!(!written_spine_chirho(
        "PairBoxChirho a a",
        "PairBoxChirho",
        2
    ));
    // `T (U a)` is written `T U a`: a constructor token is never a variable.
    assert!(!written_spine_chirho("T U a", "T", 2));
    // The arity must be the typed head's.
    assert!(!written_spine_chirho("T a", "T", 2));
    assert!(!written_spine_chirho("T a b", "T", 1));
    // Another constructor, including one the name is a prefix of.
    assert!(!written_spine_chirho("S a", "T", 1));
    assert!(!written_spine_chirho("TT a", "T", 1));
    // The desugarer's unknown `_`, a promoted constructor, a literal.
    assert!(!written_spine_chirho("T _", "T", 1));
    assert!(!written_spine_chirho("T 'True", "T", 1));
    assert!(!written_spine_chirho("T 3", "T", 1));
    // Stray spacing never counts as a variable.
    assert!(!written_spine_chirho("T  a", "T", 1));
    assert!(!written_spine_chirho("T  a", "T", 2));
}
