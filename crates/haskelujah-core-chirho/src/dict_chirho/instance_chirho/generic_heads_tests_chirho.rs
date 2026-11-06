// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The guards of a generic instance head: the class environment (G1-G4), the
//! typed spine (G3), the written spelling (G5) and the superclass slots (G6,
//! row 699). Each refusal here is a shape where finding the dictionary under
//! its constructor could answer for the wrong instance.

use std::cell::Cell;
use std::collections::HashMap;

use haskelujah_typing_chirho::class_chirho::{
    ClassDeclChirho, ClassEnvChirho, InstDeclChirho, PredChirho,
};
use haskelujah_typing_chirho::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

use super::{
    DictLayoutChirho, DictPassCtxChirho, DictionaryUnavailableChirho, GenericHeadChirho,
    SUPERCLASS_STEPS_CHIRHO, distinct_variable_spine_chirho, generic_spine_chirho, head_key_chirho,
    matching_instance_chirho, superclass_at_head_chirho, written_spine_chirho,
};
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho};

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
    // A variable head is outside this rule. A list over one variable is the
    // list constructor `[]` over it (U2, gpt_chirho #28111).
    assert_eq!(
        distinct_variable_spine_chirho(&app_chirho(var_chirho(1), var_chirho(0))),
        None
    );
    assert_eq!(
        distinct_variable_spine_chirho(&TyChirho::ListChirho(Box::new(var_chirho(0)))),
        Some(("[]", 1))
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
            super_templates_chirho: Vec::new(),
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
    // G6 is not the spine's: a class with superclasses is judged by `generic_head_chirho`,
    // which requires every superclass slot to be served here (row 699).
    assert_eq!(
        first_spine_chirho(&class_env_chirho(
            &["EqChirho"],
            Vec::new(),
            vec![(box_chirho(), vec![])],
        )),
        generic_chirho
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

fn list_chirho(element_chirho: TyChirho) -> TyChirho {
    TyChirho::ListChirho(Box::new(element_chirho))
}

#[test]
fn a_list_over_one_variable_is_the_list_constructor_chirho() {
    // gpt_chirho #28111: the typed head `[v]` and its one written spelling.
    assert_eq!(
        distinct_variable_spine_chirho(&list_chirho(var_chirho(0))),
        Some(("[]", 1))
    );
    assert_eq!(
        distinct_variable_spine_chirho(&list_chirho(con_chirho("Int"))),
        None
    );
    assert!(written_spine_chirho("[a]", "[]", 1));
    assert!(written_spine_chirho("[a']", "[]", 1));
    // A specialized element, the unknown `_`, two tokens, the bare list, a
    // nested list, an unbracketed variable.
    for spelling_chirho in ["[Int]", "[_]", "[a b]", "[]", "[[a]]", "a"] {
        assert!(
            !written_spine_chirho(spelling_chirho, "[]", 1),
            "{spelling_chirho}"
        );
    }
    assert!(!written_spine_chirho("[a]", "[]", 2));
    assert!(!written_spine_chirho("[a]", "T", 1));
}

#[test]
fn a_list_head_is_generic_only_without_a_competing_head_chirho() {
    let alone_chirho = first_spine_chirho(&class_env_chirho(
        &[],
        Vec::new(),
        vec![(list_chirho(var_chirho(0)), vec![])],
    ));
    assert_eq!(alone_chirho, Some(("[]".to_string(), 1)));
    // Another constructor does not compete.
    assert_eq!(
        first_spine_chirho(&class_env_chirho(
            &[],
            Vec::new(),
            vec![
                (list_chirho(var_chirho(0)), vec![]),
                (con_chirho("Int"), vec![])
            ],
        )),
        Some(("[]".to_string(), 1))
    );
    // G4: a specialized list head or a variable head competes.
    for competitor_chirho in [list_chirho(con_chirho("Int")), var_chirho(1)] {
        assert_eq!(
            first_spine_chirho(&class_env_chirho(
                &[],
                Vec::new(),
                vec![
                    (list_chirho(var_chirho(0)), vec![]),
                    (competitor_chirho, vec![])
                ],
            )),
            None
        );
    }
}

#[test]
fn only_the_list_head_may_omit_a_method_chirho() {
    // The list head's rows have one written spelling, so a slot without one is
    // a method the instance omits; every other head keeps G5 whole.
    let layout_chirho = DictLayoutChirho {
        class_name_chirho: "TagChirho".to_string(),
        super_slots_chirho: Vec::new(),
        method_slots_chirho: vec![
            ("firstChirho".to_string(), 0),
            ("secondChirho".to_string(), 1),
        ],
        field_count_chirho: 2,
    };
    let list_env_chirho =
        class_env_chirho(&[], Vec::new(), vec![(list_chirho(var_chirho(0)), vec![])]);
    let list_inst_chirho = &list_env_chirho.instances_chirho["TagChirho"][0];
    let list_key_chirho = format!("{}", list_inst_chirho.head_ty_chirho);
    let head_chirho = pass_with_rows_chirho(&[("$prim_TagChirho_firstChirho_[a]", 1)])
        .generic_head_chirho(
            &list_env_chirho,
            "TagChirho",
            list_inst_chirho,
            &layout_chirho,
            &list_key_chirho,
        )
        .expect("a list head whose omitted method takes the stub");
    assert_eq!(head_chirho.constructor_chirho, "[]");
    assert_eq!(head_chirho.written_key_chirho, "[a]");
    let box_env_chirho = class_env_chirho(
        &[],
        Vec::new(),
        vec![(app_chirho(con_chirho("BoxChirho"), var_chirho(0)), vec![])],
    );
    let box_inst_chirho = &box_env_chirho.instances_chirho["TagChirho"][0];
    let box_key_chirho = format!("{}", box_inst_chirho.head_ty_chirho);
    assert!(
        pass_with_rows_chirho(&[("$prim_TagChirho_firstChirho_BoxChirho a", 1)])
            .generic_head_chirho(
                &box_env_chirho,
                "TagChirho",
                box_inst_chirho,
                &layout_chirho,
                &box_key_chirho,
            )
            .is_none()
    );
}

/// A one-method class over the variable `var_chirho`, whose superclasses are the templates
/// `supers_chirho` over that variable: each a class and the type it constrains.
fn superclass_decl_chirho(
    name_chirho: &str,
    method_chirho: &str,
    var_chirho: u32,
    supers_chirho: Vec<(&str, TyChirho)>,
) -> ClassDeclChirho {
    let class_var_chirho = TyVarChirho(var_chirho);
    ClassDeclChirho {
        name_chirho: name_chirho.to_string(),
        supers_chirho: supers_chirho
            .iter()
            .map(|(super_chirho, _)| super_chirho.to_string())
            .collect(),
        super_templates_chirho: supers_chirho
            .into_iter()
            .map(|(super_chirho, ty_chirho)| SchemePredChirho {
                class_name_chirho: super_chirho.to_string(),
                ty_chirho,
                extra_tys_chirho: Vec::new(),
            })
            .collect(),
        var_chirho: class_var_chirho,
        methods_chirho: HashMap::from([(
            method_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![class_var_chirho],
                preds_chirho: Vec::new(),
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(class_var_chirho),
                    TyChirho::string_chirho(),
                ),
            },
        )]),
        extra_vars_chirho: Vec::new(),
        fundeps_chirho: Vec::new(),
        defaults_chirho: HashMap::new(),
    }
}

/// A one-parameter instance of `class_chirho` at `head_chirho` under `context_chirho`.
fn instance_of_chirho(
    class_chirho: &str,
    head_chirho: TyChirho,
    context_chirho: Vec<PredChirho>,
) -> InstDeclChirho {
    InstDeclChirho {
        class_name_chirho: class_chirho.to_string(),
        head_ty_chirho: head_chirho,
        extra_head_tys_chirho: Vec::new(),
        context_chirho,
    }
}

fn box_head_chirho(variable_chirho: u32) -> TyChirho {
    app_chirho(con_chirho("BoxChirho"), var_chirho(variable_chirho))
}

/// `ListyChirho`, without superclasses, and `TagChirho`, whose one superclass is
/// `ListyChirho <template_chirho>` over Tag's variable 100, with these instances.
fn listy_tag_env_chirho(
    template_chirho: TyChirho,
    instances_chirho: Vec<InstDeclChirho>,
) -> ClassEnvChirho {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.add_class_chirho(superclass_decl_chirho(
        "ListyChirho",
        "describeChirho",
        200,
        Vec::new(),
    ));
    env_chirho.add_class_chirho(superclass_decl_chirho(
        "TagChirho",
        "tagChirho",
        100,
        vec![("ListyChirho", template_chirho)],
    ));
    for instance_chirho in instances_chirho {
        env_chirho.add_instance_chirho(instance_chirho);
    }
    env_chirho
}

/// A pass over `env_chirho`'s layouts whose source holds exactly these rows.
fn pass_over_chirho(env_chirho: &ClassEnvChirho, rows_chirho: &[(&str, u32)]) -> DictPassCtxChirho {
    let mut pass_chirho = pass_with_rows_chirho(rows_chirho);
    pass_chirho.build_layouts_chirho(env_chirho);
    pass_chirho
}

/// The instance of `class_chirho` whose registry key is `key_chirho`.
fn instance_at_chirho<'a>(
    env_chirho: &'a ClassEnvChirho,
    class_chirho: &str,
    key_chirho: &str,
) -> &'a InstDeclChirho {
    env_chirho.instances_chirho[class_chirho]
        .iter()
        .find(|instance_chirho| {
            head_key_chirho(
                &instance_chirho.head_ty_chirho,
                &instance_chirho.extra_head_tys_chirho,
            ) == key_chirho
        })
        .unwrap_or_else(|| panic!("no instance {class_chirho} {key_chirho}"))
}

/// `generic_head_chirho` for the instance of `class_chirho` at `key_chirho`.
fn head_at_chirho(
    pass_chirho: &DictPassCtxChirho,
    env_chirho: &ClassEnvChirho,
    class_chirho: &str,
    key_chirho: &str,
) -> Option<GenericHeadChirho> {
    pass_chirho.generic_head_chirho(
        env_chirho,
        class_chirho,
        instance_at_chirho(env_chirho, class_chirho, key_chirho),
        &pass_chirho.layouts_chirho[class_chirho],
        key_chirho,
    )
}

/// The binder and fields of the dictionary the pass generated as `name_chirho`.
fn dictionary_chirho(
    pass_chirho: &DictPassCtxChirho,
    name_chirho: &str,
) -> (CoreIdChirho, Vec<CoreExprChirho>) {
    let binding_chirho = pass_chirho
        .generated_bindings_chirho
        .iter()
        .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == name_chirho)
        .unwrap_or_else(|| panic!("no dictionary {name_chirho}"));
    let CoreExprChirho::ConAppChirho { args_chirho, .. } = &binding_chirho.rhs_chirho else {
        panic!("{name_chirho} is not a dictionary");
    };
    (binding_chirho.binder_chirho.id_chirho, args_chirho.clone())
}

/// The dictionary a use site finds for `class_chirho` under the constructor `constructor_chirho`.
fn alias_chirho(
    pass_chirho: &DictPassCtxChirho,
    class_chirho: &str,
    constructor_chirho: &str,
) -> Option<CoreIdChirho> {
    pass_chirho
        .instance_dicts_chirho
        .get(&(class_chirho.to_string(), constructor_chirho.to_string()))
        .copied()
}

#[test]
fn a_transformed_superclass_slot_holds_the_dictionary_its_template_matches_chirho() {
    // `class ListyChirho [a] => TagChirho a` (row 699, #32248): the slot of
    // `TagChirho (BoxChirho x)` is `ListyChirho [BoxChirho x]`, which `ListyChirho [b]` serves,
    // never `ListyChirho (BoxChirho x)`, the dictionary its own key names.
    let env_chirho = listy_tag_env_chirho(
        list_chirho(var_chirho(100)),
        vec![
            instance_of_chirho("ListyChirho", list_chirho(var_chirho(5)), Vec::new()),
            instance_of_chirho("ListyChirho", box_head_chirho(0), Vec::new()),
            instance_of_chirho("TagChirho", box_head_chirho(0), Vec::new()),
        ],
    );
    let mut pass_chirho = pass_over_chirho(
        &env_chirho,
        &[
            ("$prim_ListyChirho_describeChirho_[b]", 1),
            ("$prim_ListyChirho_describeChirho_BoxChirho x", 2),
            ("$prim_TagChirho_tagChirho_BoxChirho y", 3),
        ],
    );
    let head_chirho = head_at_chirho(&pass_chirho, &env_chirho, "TagChirho", "(BoxChirho t0)")
        .expect("a head whose superclass template is served here");
    assert_eq!(
        head_chirho.superclass_dictionaries_chirho,
        vec![("ListyChirho".to_string(), "[t5]".to_string())]
    );
    pass_chirho.generate_instance_dicts_chirho(&env_chirho);
    let (list_dictionary_chirho, _) = dictionary_chirho(&pass_chirho, "$fListyChirho[t5]");
    let (own_key_dictionary_chirho, _) =
        dictionary_chirho(&pass_chirho, "$fListyChirho(BoxChirho t0)");
    let (tag_dictionary_chirho, tag_fields_chirho) =
        dictionary_chirho(&pass_chirho, "$fTagChirho(BoxChirho t0)");
    assert_ne!(list_dictionary_chirho, own_key_dictionary_chirho);
    assert_eq!(
        tag_fields_chirho,
        vec![
            CoreExprChirho::VarChirho(list_dictionary_chirho),
            CoreExprChirho::VarChirho(CoreIdChirho(3)),
        ]
    );
    assert_eq!(
        alias_chirho(&pass_chirho, "TagChirho", "BoxChirho"),
        Some(tag_dictionary_chirho)
    );
    assert!(pass_chirho.unsupported_dictionaries_chirho.is_empty());
}

/// How the one superclass template of `TagChirho (BoxChirho t0)` matches: the number of context
/// goals its one match leaves, or why there is no one match.
fn tag_template_match_chirho(
    pass_chirho: &DictPassCtxChirho,
    env_chirho: &ClassEnvChirho,
) -> Result<usize, DictionaryUnavailableChirho> {
    let template_chirho = superclass_at_head_chirho(
        env_chirho,
        instance_at_chirho(env_chirho, "TagChirho", "(BoxChirho t0)"),
        &pass_chirho.layouts_chirho["TagChirho"],
        0,
    )
    .expect("Tag's template instantiates at its head");
    matching_instance_chirho(env_chirho, &template_chirho)
        .map(|(_, _, goals_chirho)| goals_chirho.len())
}

#[test]
fn an_unserved_superclass_slot_is_a_gap_without_alias_or_refusal_chirho() {
    // Row 699, #32248: each head keeps its own-key slot as before, takes no alias, and is
    // refused nothing here. This pass cannot tell any of these from GHC's GHC-39999, which
    // belongs to the checker's instance obligations.
    let tag_row_chirho = ("$prim_TagChirho_tagChirho_BoxChirho y", 3);
    let box_row_chirho = ("$prim_ListyChirho_describeChirho_BoxChirho x", 2);
    let cases_chirho = [
        (
            // `instance ListyChirho a => ListyChirho [a]` leaves `ListyChirho (BoxChirho x)`,
            // which GHC discharges by the Box instance.
            "contextual",
            listy_tag_env_chirho(
                list_chirho(var_chirho(100)),
                vec![
                    instance_of_chirho(
                        "ListyChirho",
                        list_chirho(var_chirho(5)),
                        vec![PredChirho {
                            class_name_chirho: "ListyChirho".to_string(),
                            ty_chirho: var_chirho(5),
                            extra_tys_chirho: Vec::new(),
                        }],
                    ),
                    instance_of_chirho("ListyChirho", box_head_chirho(1), Vec::new()),
                    instance_of_chirho("TagChirho", box_head_chirho(0), Vec::new()),
                ],
            ),
            vec![
                ("$prim_ListyChirho_describeChirho_[a]", 1),
                box_row_chirho,
                tag_row_chirho,
            ],
            Ok(1),
        ),
        (
            // The one match is context-free, but its rows are another module's.
            "built elsewhere",
            listy_tag_env_chirho(
                var_chirho(100),
                vec![
                    instance_of_chirho("ListyChirho", box_head_chirho(1), Vec::new()),
                    instance_of_chirho("TagChirho", box_head_chirho(0), Vec::new()),
                ],
            ),
            vec![tag_row_chirho],
            Ok(0),
        ),
        (
            // No instance in scope: GHC's error, or one no interface carries here.
            "unmatched",
            listy_tag_env_chirho(
                var_chirho(100),
                vec![instance_of_chirho(
                    "TagChirho",
                    box_head_chirho(0),
                    Vec::new(),
                )],
            ),
            vec![tag_row_chirho],
            Err(DictionaryUnavailableChirho::NoMatchChirho(
                "ListyChirho (BoxChirho t0)".to_string(),
            )),
        ),
        (
            // A catch-all beside the Box instance, as the compiler's `Typeable a` beside a
            // derived one. Overlap pragmas are not kept, so two matches decide nothing.
            "several",
            listy_tag_env_chirho(
                var_chirho(100),
                vec![
                    instance_of_chirho("ListyChirho", var_chirho(7), Vec::new()),
                    instance_of_chirho("ListyChirho", box_head_chirho(1), Vec::new()),
                    instance_of_chirho("TagChirho", box_head_chirho(0), Vec::new()),
                ],
            ),
            vec![box_row_chirho, tag_row_chirho],
            Err(DictionaryUnavailableChirho::SeveralChirho(
                "ListyChirho (BoxChirho t0)".to_string(),
            )),
        ),
    ];
    for (case_chirho, env_chirho, rows_chirho, matched_chirho) in cases_chirho {
        let mut pass_chirho = pass_over_chirho(&env_chirho, &rows_chirho);
        assert_eq!(
            tag_template_match_chirho(&pass_chirho, &env_chirho),
            matched_chirho,
            "{case_chirho}"
        );
        assert!(
            head_at_chirho(&pass_chirho, &env_chirho, "TagChirho", "(BoxChirho t0)").is_none(),
            "{case_chirho}"
        );
        pass_chirho.generate_instance_dicts_chirho(&env_chirho);
        assert_eq!(
            alias_chirho(&pass_chirho, "TagChirho", "BoxChirho"),
            None,
            "{case_chirho}"
        );
        assert!(
            pass_chirho.unsupported_dictionaries_chirho.is_empty(),
            "{case_chirho}"
        );
    }
}

#[test]
fn a_superclass_cycle_ends_at_the_step_budget_as_a_gap_chirho() {
    // UndecidableSuperClasses: `class CycleChirho [a] => CycleChirho a`. The slot of
    // `CycleChirho [b]` is `CycleChirho [[b]]`, which `CycleChirho [b]` serves again, so only
    // the step budget ends the query, and a spent query is a gap, never a proof (row 699).
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.add_class_chirho(superclass_decl_chirho(
        "CycleChirho",
        "cycleChirho",
        300,
        vec![("CycleChirho", list_chirho(var_chirho(300)))],
    ));
    env_chirho.add_instance_chirho(instance_of_chirho(
        "CycleChirho",
        list_chirho(var_chirho(5)),
        Vec::new(),
    ));
    env_chirho.add_instance_chirho(instance_of_chirho(
        "CycleChirho",
        box_head_chirho(0),
        Vec::new(),
    ));
    let pass_chirho = pass_over_chirho(
        &env_chirho,
        &[
            ("$prim_CycleChirho_cycleChirho_[b]", 1),
            ("$prim_CycleChirho_cycleChirho_BoxChirho x", 2),
        ],
    );
    let budget_chirho = Cell::new(SUPERCLASS_STEPS_CHIRHO);
    assert!(
        pass_chirho
            .generic_head_within_chirho(
                &env_chirho,
                "CycleChirho",
                instance_at_chirho(&env_chirho, "CycleChirho", "(BoxChirho t0)"),
                &pass_chirho.layouts_chirho["CycleChirho"],
                "(BoxChirho t0)",
                &budget_chirho,
            )
            .is_none()
    );
    assert_eq!(budget_chirho.get(), 0);
    assert!(head_at_chirho(&pass_chirho, &env_chirho, "CycleChirho", "[t5]").is_none());
}
