// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Unit 523's core controls, run on the dictionary pass alone:
//! - every integer-represented key's Enum dictionary takes real `toEnum` and
//!   `fromEnum` rows (32 slots), never a missing-method stub;
//! - `fromEnum` is the identity;
//! - `toEnum` guards exactly the table's bounds, lower first, then yields the
//!   tag;
//! - each guard renders GHC 9.14.1's measured message, and every measured
//!   in-range tag passes all guards.
//!
//! The measured tables come from the receipts in
//! `tmp-chirho/integral-enum-1018-chirho/scalar-out-chirho/` of claude2_chirho's
//! worktree.
//! workflow: language-features-chirho/integral-enum-rows-chirho

use std::collections::{BTreeSet, HashMap};

use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, InstDeclChirho};
use haskelujah_typing_chirho::env_chirho::TyEnvChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::{INTEGRAL_KEYS_CHIRHO, OutOfRangeChirho};
use crate::dict_chirho::{DictPassCtxChirho, DictPassResultChirho, dict_pass_module_chirho};
use crate::expr_chirho::{
    AltConChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho, CoreModuleChirho,
};

/// GHC 9.14.1's measured failures, `print (fromEnum (toEnum tag :: Key))`.
const MEASURED_FAILURES_CHIRHO: &[(&str, i64, &str)] = &[
    (
        "Int8",
        -300,
        "Enum.toEnum{Int8}: tag (-300) is outside of enumeration's range (-128,127)",
    ),
    (
        "Int8",
        300,
        "Enum.toEnum{Int8}: tag (300) is outside of enumeration's range (-128,127)",
    ),
    (
        "Int16",
        -70000,
        "Enum.toEnum{Int16}: tag (-70000) is outside of enumeration's range (-32768,32767)",
    ),
    (
        "Int16",
        70000,
        "Enum.toEnum{Int16}: tag (70000) is outside of enumeration's range (-32768,32767)",
    ),
    (
        "Int32",
        -5000000000,
        "Enum.toEnum{Int32}: tag (-5000000000) is outside of enumeration's range (-2147483648,2147483647)",
    ),
    (
        "Int32",
        5000000000,
        "Enum.toEnum{Int32}: tag (5000000000) is outside of enumeration's range (-2147483648,2147483647)",
    ),
    (
        "Word",
        -1,
        "Enum.toEnum{Word}: tag (-1) is outside of enumeration's range (0,18446744073709551615)",
    ),
    (
        "Word8",
        -1,
        "Enum.toEnum{Word8}: tag (-1) is outside of enumeration's range (0,255)",
    ),
    (
        "Word8",
        300,
        "Enum.toEnum{Word8}: tag (300) is outside of enumeration's range (0,255)",
    ),
    (
        "Word16",
        -1,
        "Enum.toEnum{Word16}: tag (-1) is outside of enumeration's range (0,65535)",
    ),
    (
        "Word16",
        70000,
        "Enum.toEnum{Word16}: tag (70000) is outside of enumeration's range (0,65535)",
    ),
    (
        "Word32",
        -1,
        "Enum.toEnum{Word32}: tag (-1) is outside of enumeration's range (0,4294967295)",
    ),
    (
        "Word32",
        5000000000,
        "Enum.toEnum{Word32}: tag (5000000000) is outside of enumeration's range (0,4294967295)",
    ),
    (
        "Word64",
        -1,
        "Enum.toEnum{Word64}: tag (-1) is outside of enumeration's range (0,18446744073709551615)",
    ),
    ("Natural", -1, "toEnum: unexpected negative Int"),
    (
        "CChar",
        -300,
        "Enum.toEnum{Int8}: tag (-300) is outside of enumeration's range (-128,127)",
    ),
    (
        "CChar",
        300,
        "Enum.toEnum{Int8}: tag (300) is outside of enumeration's range (-128,127)",
    ),
    (
        "CInt",
        -5000000000,
        "Enum.toEnum{Int32}: tag (-5000000000) is outside of enumeration's range (-2147483648,2147483647)",
    ),
    (
        "CInt",
        5000000000,
        "Enum.toEnum{Int32}: tag (5000000000) is outside of enumeration's range (-2147483648,2147483647)",
    ),
    (
        "CUInt",
        -1,
        "Enum.toEnum{Word32}: tag (-1) is outside of enumeration's range (0,4294967295)",
    ),
    (
        "CUInt",
        5000000000,
        "Enum.toEnum{Word32}: tag (5000000000) is outside of enumeration's range (0,4294967295)",
    ),
    (
        "CULong",
        -1,
        "Enum.toEnum{Word64}: tag (-1) is outside of enumeration's range (0,18446744073709551615)",
    ),
    (
        "CSize",
        -1,
        "Enum.toEnum{Word64}: tag (-1) is outside of enumeration's range (0,18446744073709551615)",
    ),
];

/// Tags GHC 9.14.1 round-trips unchanged at each key.
const MEASURED_IN_RANGE_CHIRHO: &[(&str, i64)] = &[
    ("Int8", -1),
    ("Int8", 7),
    ("Int16", -1),
    ("Int16", 7),
    ("Int32", -1),
    ("Int32", 7),
    ("Int64", -1),
    ("Int64", 7),
    ("Word", 7),
    ("Word8", 7),
    ("Word16", 7),
    ("Word32", 7),
    ("Word64", 7),
    ("Natural", 7),
    ("CChar", -1),
    ("CChar", 7),
    ("CInt", -1),
    ("CInt", 7),
    ("CLong", -1),
    ("CLong", 7),
    ("CUInt", 7),
    ("CULong", 7),
    ("CSize", 7),
];

/// The dictionary pass over an empty module, with an Enum instance at every key.
fn dict_pass_chirho() -> DictPassResultChirho {
    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    class_env_chirho.seed_standard_chirho();
    for integral_chirho in INTEGRAL_KEYS_CHIRHO {
        let head_chirho = TyChirho::ConChirho(integral_chirho.key_chirho.to_string());
        let declared_chirho =
            class_env_chirho
                .instances_chirho
                .get("Enum")
                .is_some_and(|instances_chirho| {
                    instances_chirho
                        .iter()
                        .any(|instance_chirho| instance_chirho.head_ty_chirho == head_chirho)
                });
        if !declared_chirho {
            class_env_chirho.add_instance_chirho(InstDeclChirho {
                class_name_chirho: "Enum".to_string(),
                head_ty_chirho: head_chirho,
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            });
        }
    }
    let module_chirho = CoreModuleChirho {
        name_chirho: "IntegralEnumChirho".to_string(),
        bindings_chirho: Vec::new(),
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: Vec::new(),
    };
    dict_pass_module_chirho(
        &module_chirho,
        HashMap::new(),
        &TyEnvChirho::new_chirho(),
        &class_env_chirho,
    )
}

/// The binding the pass generated under `name_chirho`.
fn binding_chirho<'a>(
    result_chirho: &'a DictPassResultChirho,
    name_chirho: &str,
) -> &'a CoreBindingChirho {
    result_chirho
        .module_chirho
        .bindings_chirho
        .iter()
        .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == name_chirho)
        .unwrap_or_else(|| panic!("{name_chirho} is generated"))
}

/// One guard of a checked `toEnum` row.
struct GuardChirho<'a> {
    op_chirho: &'a str,
    bound_chirho: i64,
    failure_chirho: &'a CoreExprChirho,
}

/// A `toEnum` row's tag and its guards, outermost first. Past the last guard
/// the row must yield the tag itself.
fn guards_chirho(row_chirho: &CoreBindingChirho) -> (CoreIdChirho, Vec<GuardChirho<'_>>) {
    let name_chirho = &row_chirho.binder_chirho.name_chirho;
    let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho,
    } = &row_chirho.rhs_chirho
    else {
        panic!("{name_chirho} is a lambda");
    };
    let tag_chirho = binder_chirho.id_chirho;
    let mut guards_chirho = Vec::new();
    let mut current_chirho = body_chirho.as_ref();
    loop {
        match current_chirho {
            CoreExprChirho::VarChirho(id_chirho) if *id_chirho == tag_chirho => {
                return (tag_chirho, guards_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                let CoreExprChirho::PrimOpChirho {
                    name_chirho: op_chirho,
                    args_chirho,
                } = scrutinee_chirho.as_ref()
                else {
                    panic!("{name_chirho}: a guard compares the tag");
                };
                let [
                    CoreExprChirho::VarChirho(compared_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(bound_chirho)),
                ] = args_chirho.as_slice()
                else {
                    panic!("{name_chirho}: a guard compares the tag with a bound");
                };
                assert_eq!(
                    *compared_chirho, tag_chirho,
                    "{name_chirho} compares its tag"
                );
                let [then_chirho, otherwise_chirho] = alts_chirho.as_slice() else {
                    panic!("{name_chirho}: a guard has two alternatives");
                };
                assert!(
                    matches!(&then_chirho.con_chirho, AltConChirho::DataConChirho(con_chirho) if con_chirho == "True")
                        && matches!(otherwise_chirho.con_chirho, AltConChirho::DefaultChirho),
                    "{name_chirho}: a guard fails on True and continues otherwise"
                );
                guards_chirho.push(GuardChirho {
                    op_chirho: op_chirho.as_str(),
                    bound_chirho: *bound_chirho,
                    failure_chirho: &then_chirho.rhs_chirho,
                });
                current_chirho = &otherwise_chirho.rhs_chirho;
            }
            other_chirho => panic!("{name_chirho}: unexpected shape {other_chirho:?}"),
        }
    }
}

/// The text a message expression renders when the tag is `tag_value_chirho`.
fn message_text_chirho(
    message_chirho: &CoreExprChirho,
    tag_chirho: CoreIdChirho,
    tag_value_chirho: i64,
) -> String {
    match message_chirho {
        CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(literal_chirho)) => {
            literal_chirho.clone()
        }
        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => match (name_chirho.as_str(), args_chirho.as_slice()) {
            ("++#", [left_chirho, right_chirho]) => {
                message_text_chirho(left_chirho, tag_chirho, tag_value_chirho)
                    + &message_text_chirho(right_chirho, tag_chirho, tag_value_chirho)
            }
            ("showInt#", [CoreExprChirho::VarChirho(id_chirho)]) if *id_chirho == tag_chirho => {
                tag_value_chirho.to_string()
            }
            _ => panic!("unexpected message part {message_chirho:?}"),
        },
        other_chirho => panic!("unexpected message part {other_chirho:?}"),
    }
}

/// What the `toEnum` row at `key_chirho` raises for a tag, or `None` when the
/// tag passes every guard.
fn raised_chirho(
    result_chirho: &DictPassResultChirho,
    key_chirho: &str,
    tag_value_chirho: i64,
) -> Option<String> {
    let row_name_chirho = format!("$prim_Enum_toEnum_{key_chirho}");
    let (tag_chirho, guards_chirho) =
        guards_chirho(binding_chirho(result_chirho, &row_name_chirho));
    let guard_chirho = guards_chirho
        .iter()
        .find(|guard_chirho| match guard_chirho.op_chirho {
            "<#" => tag_value_chirho < guard_chirho.bound_chirho,
            ">#" => tag_value_chirho > guard_chirho.bound_chirho,
            other_chirho => panic!("{row_name_chirho}: unexpected comparison {other_chirho}"),
        })?;
    let CoreExprChirho::PrimOpChirho {
        name_chirho,
        args_chirho,
    } = guard_chirho.failure_chirho
    else {
        panic!("{row_name_chirho}: a failure is a primitive");
    };
    assert_eq!(
        name_chirho, "error",
        "{row_name_chirho} fails through `error`"
    );
    let [message_chirho] = args_chirho.as_slice() else {
        panic!("{row_name_chirho}: `error` takes one message");
    };
    Some(message_text_chirho(
        message_chirho,
        tag_chirho,
        tag_value_chirho,
    ))
}

#[test]
fn every_integral_dictionary_takes_real_rows_chirho() {
    let result_chirho = dict_pass_chirho();
    let layout_chirho = &result_chirho.layouts_chirho["Enum"];
    let mut slots_chirho = 0;
    for integral_chirho in INTEGRAL_KEYS_CHIRHO {
        let dictionary_name_chirho = format!("$fEnum{}", integral_chirho.key_chirho);
        let CoreExprChirho::ConAppChirho {
            args_chirho: fields_chirho,
            ..
        } = &binding_chirho(&result_chirho, &dictionary_name_chirho).rhs_chirho
        else {
            panic!("{dictionary_name_chirho} is a dictionary constructor");
        };
        for method_chirho in ["toEnum", "fromEnum"] {
            let (_, slot_chirho) = layout_chirho
                .method_slots_chirho
                .iter()
                .find(|(name_chirho, _)| name_chirho == method_chirho)
                .unwrap_or_else(|| panic!("Enum has a {method_chirho} slot"));
            let CoreExprChirho::VarChirho(row_id_chirho) = &fields_chirho[*slot_chirho] else {
                panic!("{dictionary_name_chirho}'s {method_chirho} slot is a reference");
            };
            let row_chirho = result_chirho
                .module_chirho
                .bindings_chirho
                .iter()
                .find(|binding_chirho| binding_chirho.binder_chirho.id_chirho == *row_id_chirho)
                .unwrap_or_else(|| {
                    panic!("{dictionary_name_chirho}'s {method_chirho} row is bound")
                });
            assert_eq!(
                row_chirho.binder_chirho.name_chirho,
                format!("$prim_Enum_{method_chirho}_{}", integral_chirho.key_chirho),
                "{dictionary_name_chirho}'s {method_chirho} slot"
            );
            assert!(
                !DictPassCtxChirho::is_generated_missing_method_body_chirho(&row_chirho.rhs_chirho),
                "{} is a missing-method stub",
                row_chirho.binder_chirho.name_chirho
            );
            slots_chirho += 1;
        }
    }
    assert_eq!(slots_chirho, 32);
}

#[test]
fn from_enum_rows_are_the_identity_chirho() {
    let result_chirho = dict_pass_chirho();
    for integral_chirho in INTEGRAL_KEYS_CHIRHO {
        let row_name_chirho = format!("$prim_Enum_fromEnum_{}", integral_chirho.key_chirho);
        let CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } = &binding_chirho(&result_chirho, &row_name_chirho).rhs_chirho
        else {
            panic!("{row_name_chirho} is a lambda");
        };
        assert!(
            matches!(body_chirho.as_ref(), CoreExprChirho::VarChirho(id_chirho) if *id_chirho == binder_chirho.id_chirho),
            "{row_name_chirho} returns its argument"
        );
    }
}

#[test]
fn to_enum_rows_guard_exactly_the_table_bounds_chirho() {
    let result_chirho = dict_pass_chirho();
    for integral_chirho in INTEGRAL_KEYS_CHIRHO {
        let row_name_chirho = format!("$prim_Enum_toEnum_{}", integral_chirho.key_chirho);
        let (_tag_chirho, guards_chirho) =
            guards_chirho(binding_chirho(&result_chirho, &row_name_chirho));
        let found_chirho: Vec<(&str, i64)> = guards_chirho
            .iter()
            .map(|guard_chirho| (guard_chirho.op_chirho, guard_chirho.bound_chirho))
            .collect();
        let expected_chirho: Vec<(&str, i64)> = match integral_chirho.out_of_range_chirho {
            OutOfRangeChirho::UncheckedChirho => Vec::new(),
            OutOfRangeChirho::NegativeChirho => vec![("<#", 0)],
            OutOfRangeChirho::BoundsChirho {
                lower_chirho,
                upper_chirho,
                ..
            } => lower_chirho
                .map(|lower_chirho| ("<#", lower_chirho))
                .into_iter()
                .chain(upper_chirho.map(|upper_chirho| (">#", upper_chirho)))
                .collect(),
        };
        assert_eq!(found_chirho, expected_chirho, "{row_name_chirho}");
    }
}

#[test]
fn each_row_raises_ghcs_measured_message_chirho() {
    let table_keys_chirho: BTreeSet<&str> = INTEGRAL_KEYS_CHIRHO
        .iter()
        .map(|integral_chirho| integral_chirho.key_chirho)
        .collect();
    let measured_keys_chirho: BTreeSet<&str> = MEASURED_IN_RANGE_CHIRHO
        .iter()
        .map(|(key_chirho, _)| *key_chirho)
        .collect();
    assert_eq!(
        table_keys_chirho, measured_keys_chirho,
        "the table holds exactly the measured keys"
    );

    let result_chirho = dict_pass_chirho();
    for (key_chirho, tag_chirho, message_chirho) in MEASURED_FAILURES_CHIRHO {
        assert_eq!(
            raised_chirho(&result_chirho, key_chirho, *tag_chirho).as_deref(),
            Some(*message_chirho),
            "{key_chirho} at {tag_chirho}"
        );
    }
    for (key_chirho, tag_chirho) in MEASURED_IN_RANGE_CHIRHO {
        assert_eq!(
            raised_chirho(&result_chirho, key_chirho, *tag_chirho),
            None,
            "{key_chirho} at {tag_chirho}"
        );
    }
}
