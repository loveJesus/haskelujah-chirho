// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 671's structure, read from the generated Core of one program that owns an `enumFromTo` of
//! its own and uses Enum through a polymorphic function at a context instance, a derived type and
//! the builtin keys. The compiler binds no body under an enumeration's source spelling, so nothing
//! unresolved can run Int's bodies, while the program's own `enumFromTo` stays its binding. Every
//! Enum dictionary carries all eight slots, each a body or headed by a bound row, never a dangling
//! id, and the unsupported Float and `()` slots are stubs that name their method.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashSet;

use haskelujah_core_chirho::expr_chirho::{CoreExprChirho, CoreIdChirho};
use haskelujah_driver::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

const STRUCTURE_SOURCE_CHIRHO: &str = r#"module Main where

import Prelude hiding (enumFromTo)

newtype WrapChirho a = WrapChirho a deriving Show

instance Enum a => Enum (WrapChirho a) where
  toEnum n = WrapChirho (toEnum n)
  fromEnum (WrapChirho x) = fromEnum x

data DayChirho = MonChirho | TueChirho deriving (Show, Enum)

enumFromTo :: Int -> Int -> [Int]
enumFromTo lowChirho highChirho = [highChirho, lowChirho]

startChirho :: Enum a => a -> [a]
startChirho = enumFrom

main :: IO ()
main = do
  print (enumFromTo 1 2)
  print (fromEnum (WrapChirho 'c'))
  print (startChirho TueChirho)
  print (startChirho False)
  print (take 1 (startChirho 'a'))
  print (take 1 (startChirho (1.0 :: Float)))
  print (take 1 (startChirho ()))
"#;

/// The head variable of an application spine, if it has one.
fn head_id_chirho(expr_chirho: &CoreExprChirho) -> Option<CoreIdChirho> {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => Some(*id_chirho),
        CoreExprChirho::AppChirho { fun_chirho, .. } => head_id_chirho(fun_chirho),
        CoreExprChirho::TyAppChirho { expr_chirho, .. } => head_id_chirho(expr_chirho),
        _ => None,
    }
}

/// Every id a lambda, let or case binds anywhere in `expr_chirho`.
fn binder_ids_chirho(expr_chirho: &CoreExprChirho, bound_chirho: &mut HashSet<CoreIdChirho>) {
    match expr_chirho {
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            bound_chirho.insert(binder_chirho.id_chirho);
            binder_ids_chirho(body_chirho, bound_chirho);
        }
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (binder_chirho, rhs_chirho) in binds_chirho {
                bound_chirho.insert(binder_chirho.id_chirho);
                binder_ids_chirho(rhs_chirho, bound_chirho);
            }
            binder_ids_chirho(body_chirho, bound_chirho);
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            bound_chirho.insert(bind_chirho.id_chirho);
            binder_ids_chirho(scrutinee_chirho, bound_chirho);
            for alt_chirho in alts_chirho {
                bound_chirho.extend(
                    alt_chirho
                        .binders_chirho
                        .iter()
                        .map(|b_chirho| b_chirho.id_chirho),
                );
                binder_ids_chirho(&alt_chirho.rhs_chirho, bound_chirho);
            }
        }
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            binder_ids_chirho(fun_chirho, bound_chirho);
            binder_ids_chirho(arg_chirho, bound_chirho);
        }
        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            binder_ids_chirho(body_chirho, bound_chirho)
        }
        CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
            binder_ids_chirho(expr_chirho, bound_chirho)
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. }
        | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                binder_ids_chirho(arg_chirho, bound_chirho);
            }
        }
        CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => {}
    }
}

/// Every `$Dict_Enum` construction in `expr_chirho`, with its field expressions.
fn enum_dictionaries_chirho<'a>(
    expr_chirho: &'a CoreExprChirho,
    found_chirho: &mut Vec<&'a [CoreExprChirho]>,
) {
    match expr_chirho {
        CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho,
        } => {
            if con_name_chirho == "$Dict_Enum" {
                found_chirho.push(args_chirho);
            }
            for arg_chirho in args_chirho {
                enum_dictionaries_chirho(arg_chirho, found_chirho);
            }
        }
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            enum_dictionaries_chirho(fun_chirho, found_chirho);
            enum_dictionaries_chirho(arg_chirho, found_chirho);
        }
        CoreExprChirho::LamChirho { body_chirho, .. }
        | CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            enum_dictionaries_chirho(body_chirho, found_chirho)
        }
        CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
            enum_dictionaries_chirho(expr_chirho, found_chirho)
        }
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (_binder_chirho, rhs_chirho) in binds_chirho {
                enum_dictionaries_chirho(rhs_chirho, found_chirho);
            }
            enum_dictionaries_chirho(body_chirho, found_chirho);
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            enum_dictionaries_chirho(scrutinee_chirho, found_chirho);
            for alt_chirho in alts_chirho {
                enum_dictionaries_chirho(&alt_chirho.rhs_chirho, found_chirho);
            }
        }
        _ => {}
    }
}

#[test]
fn every_enum_slot_is_a_bound_row_or_a_named_stub_chirho() {
    let compiled_chirho = compile_source_chirho(
        STRUCTURE_SOURCE_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "EnumSlotStructureChirho.hs",
    )
    .expect("the program compiles");
    let core_chirho = &compiled_chirho.core_chirho;
    let mut bound_chirho: HashSet<CoreIdChirho> = HashSet::new();
    for binding_chirho in &core_chirho.bindings_chirho {
        bound_chirho.insert(binding_chirho.binder_chirho.id_chirho);
        binder_ids_chirho(&binding_chirho.rhs_chirho, &mut bound_chirho);
    }
    let row_chirho = |name_chirho: &str| {
        core_chirho
            .bindings_chirho
            .iter()
            .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == name_chirho)
            .unwrap_or_else(|| panic!("{name_chirho} is generated"))
    };
    let named_chirho = |name_chirho: &str| {
        core_chirho
            .bindings_chirho
            .iter()
            .filter(|binding_chirho| binding_chirho.binder_chirho.name_chirho == name_chirho)
            .count()
    };

    // The program's own `enumFromTo` is its one binding of that spelling. A compiler body
    // sharing it would have been spelled `$prelude_enumFromTo` apart from it (row 597).
    assert_eq!(
        named_chirho("enumFromTo"),
        1,
        "the program's own enumFromTo"
    );
    for spelling_chirho in ["enumFrom", "enumFromThen", "enumFromThenTo"] {
        assert_eq!(
            named_chirho(spelling_chirho),
            0,
            "a body bound as {spelling_chirho}"
        );
    }
    for spelling_chirho in ["enumFrom", "enumFromThen", "enumFromTo", "enumFromThenTo"] {
        let apart_chirho = format!("$prelude_{spelling_chirho}");
        assert_eq!(
            named_chirho(&apart_chirho),
            0,
            "a body bound as {apart_chirho}"
        );
    }

    let mut dictionaries_chirho = Vec::new();
    for binding_chirho in &core_chirho.bindings_chirho {
        enum_dictionaries_chirho(&binding_chirho.rhs_chirho, &mut dictionaries_chirho);
    }
    assert!(
        !dictionaries_chirho.is_empty(),
        "no Enum dictionary was built"
    );
    for fields_chirho in dictionaries_chirho {
        assert_eq!(
            fields_chirho.len(),
            8,
            "an Enum dictionary without eight slots"
        );
        for head_chirho in fields_chirho.iter().filter_map(head_id_chirho) {
            assert!(
                bound_chirho.contains(&head_chirho),
                "a slot names the unbound id {head_chirho:?}"
            );
        }
    }

    for (key_chirho, method_chirho) in [("Float", "enumFrom"), ("()", "enumFrom")] {
        let stub_chirho = format!(
            "{:?}",
            row_chirho(&format!("$prim_Enum_{method_chirho}_{key_chirho}")).rhs_chirho
        );
        assert!(
            stub_chirho.contains(&format!(
                "missing method Enum.{method_chirho} for {key_chirho}"
            )),
            "{stub_chirho}"
        );
    }
    let bool_row_chirho = format!("{:?}", row_chirho("$prim_Enum_enumFrom_Bool").rhs_chirho);
    assert!(
        !bool_row_chirho.contains("missing method"),
        "{bool_row_chirho}"
    );
}
