// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The guards of a generic instance head: the class environment (G1-G4, G6),
//! the typed spine (G3) and the written spelling (G5). Each refusal here is a
//! shape where finding the dictionary under its constructor could answer for
//! the wrong instance.

use std::collections::HashMap;

use haskelujah_typing_chirho::class_chirho::{ClassDeclChirho, ClassEnvChirho, InstDeclChirho};
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::{
    DictLayoutChirho, DictPassCtxChirho, distinct_variable_spine_chirho, generic_spine_chirho,
    written_spine_chirho,
};
use crate::expr_chirho::CoreIdChirho;

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

/// A class `TagChirho` with the given superclasses and extra parameters, and
/// one instance per (head, supplied extra heads).
fn class_env_chirho(
    supers_chirho: &[&str],
    extra_vars_chirho: Vec<TyVarChirho>,
    heads_chirho: Vec<(TyChirho, Vec<TyChirho>)>,
) -> ClassEnvChirho {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.classes_chirho.insert(
        "TagChirho".to_string(),
        ClassDeclChirho {
            name_chirho: "TagChirho".to_string(),
            supers_chirho: supers_chirho
                .iter()
                .map(|name_chirho| name_chirho.to_string())
                .collect(),
            var_chirho: TyVarChirho(100),
            methods_chirho: HashMap::new(),
            extra_vars_chirho,
            fundeps_chirho: Vec::new(),
            defaults_chirho: HashMap::new(),
        },
    );
    env_chirho.instances_chirho.insert(
        "TagChirho".to_string(),
        heads_chirho
            .into_iter()
            .map(|(head_chirho, extra_chirho)| InstDeclChirho {
                class_name_chirho: "TagChirho".to_string(),
                head_ty_chirho: head_chirho,
                extra_head_tys_chirho: extra_chirho,
                context_chirho: Vec::new(),
            })
            .collect(),
    );
    env_chirho
}

/// `generic_spine_chirho` for the class's first instance.
fn first_spine_chirho(env_chirho: &ClassEnvChirho) -> Option<(String, usize)> {
    let first_chirho = &env_chirho.instances_chirho["TagChirho"][0];
    generic_spine_chirho(env_chirho, "TagChirho", first_chirho)
        .map(|(constructor_chirho, arity_chirho)| (constructor_chirho.to_string(), arity_chirho))
}

#[test]
fn the_class_environment_admits_only_an_uncontested_one_parameter_head_chirho() {
    let box_chirho = || app_chirho(con_chirho("BoxChirho"), var_chirho(0));
    let generic_chirho = Some(("BoxChirho".to_string(), 1));
    assert_eq!(
        first_spine_chirho(&class_env_chirho(
            &[],
            Vec::new(),
            vec![(box_chirho(), vec![])]
        )),
        generic_chirho
    );
    // Another constructor does not compete.
    assert_eq!(
        first_spine_chirho(&class_env_chirho(
            &[],
            Vec::new(),
            vec![
                (box_chirho(), vec![]),
                (app_chirho(con_chirho("OtherChirho"), var_chirho(1)), vec![]),
            ],
        )),
        generic_chirho
    );
    // G1: a class declared with two parameters, even where the instance
    // supplies no second head, and an instance that supplies one.
    assert_eq!(
        first_spine_chirho(&class_env_chirho(
            &[],
            vec![TyVarChirho(101)],
            vec![(box_chirho(), vec![])],
        )),
        None
    );
    assert_eq!(
        first_spine_chirho(&class_env_chirho(
            &[],
            Vec::new(),
            vec![(box_chirho(), vec![con_chirho("Int")])],
        )),
        None
    );
    // G6: a superclass slot would be named by the instance's own key.
    assert_eq!(
        first_spine_chirho(&class_env_chirho(
            &["EqChirho"],
            Vec::new(),
            vec![(box_chirho(), vec![])],
        )),
        None
    );
    // G4: a specialized head for the same constructor, or a variable head.
    assert_eq!(
        first_spine_chirho(&class_env_chirho(
            &[],
            Vec::new(),
            vec![
                (box_chirho(), vec![]),
                (
                    app_chirho(con_chirho("BoxChirho"), con_chirho("Int")),
                    vec![]
                ),
            ],
        )),
        None
    );
    assert_eq!(
        first_spine_chirho(&class_env_chirho(
            &[],
            Vec::new(),
            vec![(box_chirho(), vec![]), (var_chirho(1), vec![])],
        )),
        None
    );
}

/// A pass whose source holds exactly these rows.
fn pass_with_rows_chirho(rows_chirho: &[(&str, u32)]) -> DictPassCtxChirho {
    let mut pass_chirho = DictPassCtxChirho::new_chirho(10, HashMap::new(), HashMap::new());
    for (row_name_chirho, id_chirho) in rows_chirho {
        pass_chirho
            .body_backed_names_chirho
            .insert(row_name_chirho.to_string(), CoreIdChirho(*id_chirho));
    }
    pass_chirho
}

#[test]
fn slot_coverage_reads_the_rows_the_slot_lookup_takes_chirho() {
    // agy_chirho #27369, gpt_chirho #27370: G5 counts a method as covered only
    // by a row `resolve_or_missing_method_id_chirho` takes, the exact row for
    // the typed key first, then the one written spelling.
    let env_chirho = class_env_chirho(
        &[],
        Vec::new(),
        vec![(app_chirho(con_chirho("BoxChirho"), var_chirho(0)), vec![])],
    );
    let inst_chirho = &env_chirho.instances_chirho["TagChirho"][0];
    let typed_key_chirho = format!("{}", inst_chirho.head_ty_chirho);
    let layout_chirho = DictLayoutChirho {
        class_name_chirho: "TagChirho".to_string(),
        super_slots_chirho: Vec::new(),
        method_slots_chirho: vec![
            ("firstChirho".to_string(), 0),
            ("secondChirho".to_string(), 1),
        ],
        field_count_chirho: 2,
    };
    let first_typed_chirho = format!("$prim_TagChirho_firstChirho_{typed_key_chirho}");
    // A bare `T` row covers nothing, so the first method has no row the slot
    // would take.
    let bare_chirho = pass_with_rows_chirho(&[
        ("$prim_TagChirho_firstChirho_BoxChirho", 1),
        ("$prim_TagChirho_secondChirho_BoxChirho a", 2),
    ]);
    assert!(
        bare_chirho
            .generic_head_chirho(
                &env_chirho,
                "TagChirho",
                inst_chirho,
                &layout_chirho,
                &typed_key_chirho
            )
            .is_none()
    );
    // The exact typed row for one method and the written row for the other:
    // both slots resolve, and where both spellings exist the exact row wins.
    for rows_chirho in [
        vec![
            (first_typed_chirho.as_str(), 3),
            ("$prim_TagChirho_secondChirho_BoxChirho a", 2),
        ],
        vec![
            (first_typed_chirho.as_str(), 3),
            ("$prim_TagChirho_firstChirho_BoxChirho a", 4),
            ("$prim_TagChirho_secondChirho_BoxChirho a", 2),
        ],
    ] {
        let mut pass_chirho = pass_with_rows_chirho(&rows_chirho);
        let head_chirho = pass_chirho
            .generic_head_chirho(
                &env_chirho,
                "TagChirho",
                inst_chirho,
                &layout_chirho,
                &typed_key_chirho,
            )
            .expect("a generic head whose every slot has a row the lookup takes");
        assert_eq!(head_chirho.written_key_chirho, "BoxChirho a");
        let resolved_chirho: Vec<CoreIdChirho> = ["firstChirho", "secondChirho"]
            .iter()
            .map(|method_chirho| {
                pass_chirho.resolve_or_missing_method_id_chirho(
                    &env_chirho,
                    "TagChirho",
                    method_chirho,
                    &typed_key_chirho,
                    &format!("$prim_TagChirho_{method_chirho}_{typed_key_chirho}"),
                    Some(head_chirho.written_key_chirho.as_str()),
                )
            })
            .collect();
        assert_eq!(resolved_chirho, vec![CoreIdChirho(3), CoreIdChirho(2)]);
    }
}
