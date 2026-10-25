// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Derived Show's Core helpers, structurally (row 533): each constructor is parenthesized
//! above its own precedence (10 in prefix and record form, p in infix form), and its fields
//! render at 11, 0 and p + 1. Infix values cannot be built end to end on this main, so these
//! tests are the proof until the replay gate runs G2, K06f and N1.

use super::*;
use haskelujah_typing_chirho::deriving_chirho::DerivedShowChirho;

fn context_chirho(types_chirho: Vec<(&str, DerivedShowChirho)>) -> DictPassCtxChirho {
    let mut context_chirho = DictPassCtxChirho::new_chirho(1, HashMap::new(), HashMap::new());
    context_chirho.derived_shows_chirho = types_chirho
        .into_iter()
        .map(|(name_chirho, metadata_chirho)| (name_chirho.to_string(), metadata_chirho))
        .collect();
    context_chirho
}

fn monomorphic_chirho(constructors_chirho: Vec<DerivedShowConChirho>) -> DerivedShowChirho {
    DerivedShowChirho {
        type_vars_chirho: vec![],
        constructors_chirho,
    }
}

fn constructor_chirho(
    name_chirho: &str,
    form_chirho: ShowConFormChirho,
    fields_chirho: Vec<ShowFieldTypeChirho>,
) -> DerivedShowConChirho {
    DerivedShowConChirho {
        name_chirho: name_chirho.to_string(),
        form_chirho,
        fields_chirho,
    }
}

fn type_chirho(name_chirho: &str) -> ShowFieldTypeChirho {
    ShowFieldTypeChirho::ConChirho(name_chirho.to_string())
}

/// Render `key` once, so its helper (and every helper it needs) is generated.
fn render_chirho(context_chirho: &mut DictPassCtxChirho, key_chirho: &str) {
    let value_chirho = CoreExprChirho::VarChirho(CoreIdChirho(0));
    let _ = context_chirho.show_derived_expr_chirho(key_chirho, value_chirho, 0);
}

/// The helper of `key`: its id, its precedence parameter and its alternatives.
fn helper_chirho<'c>(
    context_chirho: &'c DictPassCtxChirho,
    key_chirho: &str,
) -> (CoreIdChirho, CoreIdChirho, &'c [CoreAltChirho]) {
    let name_chirho = format!("$prim_Show_showsPrec_{key_chirho}");
    let binding_chirho = context_chirho
        .generated_bindings_chirho
        .iter()
        .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == name_chirho)
        .expect("the helper is generated");
    let CoreExprChirho::LamChirho {
        binder_chirho: precedence_chirho,
        body_chirho,
    } = &binding_chirho.rhs_chirho
    else {
        panic!("a helper takes the precedence first");
    };
    let CoreExprChirho::LamChirho { body_chirho, .. } = body_chirho.as_ref() else {
        panic!("a helper takes the value second");
    };
    let CoreExprChirho::CaseChirho { alts_chirho, .. } = body_chirho.as_ref() else {
        panic!("a helper cases on the value");
    };
    (
        binding_chirho.binder_chirho.id_chirho,
        precedence_chirho.id_chirho,
        alts_chirho.as_slice(),
    )
}

fn alternative_chirho<'a>(
    alts_chirho: &'a [CoreAltChirho],
    name_chirho: &str,
) -> &'a CoreAltChirho {
    alts_chirho
        .iter()
        .find(|alt_chirho| {
            matches!(&alt_chirho.con_chirho, AltConChirho::DataConChirho(constructor_chirho)
                if constructor_chirho == name_chirho)
        })
        .expect("one alternative per constructor")
}

fn nodes_chirho<'e>(expr_chirho: &'e CoreExprChirho, out_chirho: &mut Vec<&'e CoreExprChirho>) {
    out_chirho.push(expr_chirho);
    match expr_chirho {
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            nodes_chirho(fun_chirho, out_chirho);
            nodes_chirho(arg_chirho, out_chirho);
        }
        CoreExprChirho::LamChirho { body_chirho, .. }
        | CoreExprChirho::TyLamChirho { body_chirho, .. } => nodes_chirho(body_chirho, out_chirho),
        CoreExprChirho::TyAppChirho { expr_chirho, .. } => nodes_chirho(expr_chirho, out_chirho),
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (_binder_chirho, rhs_chirho) in binds_chirho {
                nodes_chirho(rhs_chirho, out_chirho);
            }
            nodes_chirho(body_chirho, out_chirho);
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            nodes_chirho(scrutinee_chirho, out_chirho);
            for alt_chirho in alts_chirho {
                nodes_chirho(&alt_chirho.rhs_chirho, out_chirho);
            }
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. }
        | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                nodes_chirho(arg_chirho, out_chirho);
            }
        }
        _ => {}
    }
}

fn all_nodes_chirho(expr_chirho: &CoreExprChirho) -> Vec<&CoreExprChirho> {
    let mut out_chirho = Vec::new();
    nodes_chirho(expr_chirho, &mut out_chirho);
    out_chirho
}

/// Every `n <# d` that compares a literal with the helper's precedence parameter.
fn thresholds_chirho(expr_chirho: &CoreExprChirho, precedence_chirho: CoreIdChirho) -> Vec<i64> {
    all_nodes_chirho(expr_chirho)
        .into_iter()
        .filter_map(|node_chirho| match node_chirho {
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } if name_chirho == "<#" => match args_chirho.as_slice() {
                [
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(threshold_chirho)),
                    CoreExprChirho::VarChirho(id_chirho),
                ] if *id_chirho == precedence_chirho => Some(*threshold_chirho),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

/// The precedence of every call `helper p _` inside `expr`.
fn call_precedences_chirho(expr_chirho: &CoreExprChirho, helper_chirho: CoreIdChirho) -> Vec<i64> {
    all_nodes_chirho(expr_chirho)
        .into_iter()
        .filter_map(|node_chirho| match node_chirho {
            CoreExprChirho::AppChirho { fun_chirho, .. } => match fun_chirho.as_ref() {
                CoreExprChirho::AppChirho {
                    fun_chirho: callee_chirho,
                    arg_chirho,
                } => match (callee_chirho.as_ref(), arg_chirho.as_ref()) {
                    (
                        CoreExprChirho::VarChirho(id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(precedence_chirho)),
                    ) if *id_chirho == helper_chirho => Some(*precedence_chirho),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
        .collect()
}

fn strings_chirho(expr_chirho: &CoreExprChirho) -> Vec<String> {
    all_nodes_chirho(expr_chirho)
        .into_iter()
        .filter_map(|node_chirho| match node_chirho {
            CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(text_chirho)) => {
                Some(text_chirho.clone())
            }
            _ => None,
        })
        .collect()
}

#[test]
fn an_infix_constructor_parenthesizes_above_its_precedence_and_renders_operands_at_one_more_chirho()
{
    // data EChirho = LitChirho Int | EChirho :+ EChirho, with infixl 6 :+
    let mut context_chirho = context_chirho(vec![(
        "EChirho",
        monomorphic_chirho(vec![
            constructor_chirho(
                "LitChirho",
                ShowConFormChirho::PrefixChirho,
                vec![type_chirho("Int")],
            ),
            constructor_chirho(
                ":+",
                ShowConFormChirho::InfixChirho(6),
                vec![type_chirho("EChirho"), type_chirho("EChirho")],
            ),
        ]),
    )]);
    assert_eq!(
        context_chirho.derived_show_key_chirho("EChirho"),
        Some("EChirho".to_string())
    );
    render_chirho(&mut context_chirho, "EChirho");
    let (helper_chirho, precedence_chirho, alts_chirho) = helper_chirho(&context_chirho, "EChirho");
    let infix_chirho = &alternative_chirho(alts_chirho, ":+").rhs_chirho;
    assert_eq!(thresholds_chirho(infix_chirho, precedence_chirho), vec![6]);
    let operands_chirho = call_precedences_chirho(infix_chirho, helper_chirho);
    assert!(
        !operands_chirho.is_empty()
            && operands_chirho
                .iter()
                .all(|precedence_chirho| *precedence_chirho == 7)
    );
    assert!(strings_chirho(infix_chirho).contains(&" :+ ".to_string()));
    let prefix_chirho = &alternative_chirho(alts_chirho, "LitChirho").rhs_chirho;
    assert_eq!(
        thresholds_chirho(prefix_chirho, precedence_chirho),
        vec![10]
    );
    assert!(strings_chirho(prefix_chirho).contains(&"LitChirho".to_string()));
}

#[test]
fn prefix_fields_render_at_eleven_and_record_fields_at_zero_chirho() {
    // data QChirho = QChirho Int; data PChirho = PChirho QChirho QChirho;
    // data RChirho = RChirho { innerChirho :: QChirho }
    let mut context_chirho = context_chirho(vec![
        (
            "QChirho",
            monomorphic_chirho(vec![constructor_chirho(
                "QChirho",
                ShowConFormChirho::PrefixChirho,
                vec![type_chirho("Int")],
            )]),
        ),
        (
            "PChirho",
            monomorphic_chirho(vec![constructor_chirho(
                "PChirho",
                ShowConFormChirho::PrefixChirho,
                vec![type_chirho("QChirho"), type_chirho("QChirho")],
            )]),
        ),
        (
            "RChirho",
            monomorphic_chirho(vec![constructor_chirho(
                "RChirho",
                ShowConFormChirho::RecordChirho(vec!["innerChirho".to_string()]),
                vec![type_chirho("QChirho")],
            )]),
        ),
    ]);
    render_chirho(&mut context_chirho, "PChirho");
    render_chirho(&mut context_chirho, "RChirho");
    let (q_helper_chirho, _, _) = helper_chirho(&context_chirho, "QChirho");
    let (_, p_precedence_chirho, p_alts_chirho) = helper_chirho(&context_chirho, "PChirho");
    let prefix_chirho = &alternative_chirho(p_alts_chirho, "PChirho").rhs_chirho;
    assert_eq!(
        thresholds_chirho(prefix_chirho, p_precedence_chirho),
        vec![10]
    );
    let fields_chirho = call_precedences_chirho(prefix_chirho, q_helper_chirho);
    assert!(
        !fields_chirho.is_empty()
            && fields_chirho
                .iter()
                .all(|precedence_chirho| *precedence_chirho == 11)
    );
    let (_, r_precedence_chirho, r_alts_chirho) = helper_chirho(&context_chirho, "RChirho");
    let record_chirho = &alternative_chirho(r_alts_chirho, "RChirho").rhs_chirho;
    assert_eq!(
        thresholds_chirho(record_chirho, r_precedence_chirho),
        vec![10]
    );
    let fields_chirho = call_precedences_chirho(record_chirho, q_helper_chirho);
    assert!(
        !fields_chirho.is_empty()
            && fields_chirho
                .iter()
                .all(|precedence_chirho| *precedence_chirho == 0)
    );
    let strings_chirho = strings_chirho(record_chirho);
    for text_chirho in ["RChirho {", "innerChirho = ", "}"] {
        assert!(
            strings_chirho.contains(&text_chirho.to_string()),
            "{text_chirho}"
        );
    }
}

#[test]
fn a_field_no_shape_renders_keeps_the_type_on_its_existing_route_chirho() {
    // data UChirho = UChirho FooChirho, where FooChirho has no metadata and no Show row.
    let context_chirho = context_chirho(vec![(
        "UChirho",
        monomorphic_chirho(vec![constructor_chirho(
            "UChirho",
            ShowConFormChirho::PrefixChirho,
            vec![type_chirho("FooChirho")],
        )]),
    )]);
    assert_eq!(context_chirho.derived_show_key_chirho("UChirho"), None);
    assert!(
        context_chirho
            .parse_show_shape_at_chirho("UChirho", 11)
            .is_none()
    );
    assert!(context_chirho.generated_bindings_chirho.is_empty());
}

#[test]
fn a_parameter_takes_the_concrete_keys_argument_chirho() {
    // data TChirho a = TChirho a, shown at TChirho (TChirho Char).
    let mut context_chirho = context_chirho(vec![(
        "TChirho",
        DerivedShowChirho {
            type_vars_chirho: vec!["aChirho".to_string()],
            constructors_chirho: vec![constructor_chirho(
                "TChirho",
                ShowConFormChirho::PrefixChirho,
                vec![ShowFieldTypeChirho::VarChirho("aChirho".to_string())],
            )],
        },
    )]);
    render_chirho(&mut context_chirho, "TChirho (TChirho Char)");
    let (inner_chirho, _, _) = helper_chirho(&context_chirho, "TChirho Char");
    let (_, _, outer_alts_chirho) = helper_chirho(&context_chirho, "TChirho (TChirho Char)");
    let outer_chirho = &alternative_chirho(outer_alts_chirho, "TChirho").rhs_chirho;
    let fields_chirho = call_precedences_chirho(outer_chirho, inner_chirho);
    assert!(
        !fields_chirho.is_empty()
            && fields_chirho
                .iter()
                .all(|precedence_chirho| *precedence_chirho == 11)
    );
}

#[test]
fn metadata_supplied_for_nonempty_renders_infixr_five_without_reopening_the_renderer_chirho() {
    // The seam unit 517 fills: NonEmpty a = a :| [a], infixr 5. Its head renders at 6, where
    // a negative number needs no parentheses: GHC prints `-1 :| [2]`.
    let mut context_chirho = context_chirho(vec![(
        "NonEmpty",
        DerivedShowChirho {
            type_vars_chirho: vec!["a".to_string()],
            constructors_chirho: vec![constructor_chirho(
                ":|",
                ShowConFormChirho::InfixChirho(5),
                vec![
                    ShowFieldTypeChirho::VarChirho("a".to_string()),
                    ShowFieldTypeChirho::ListChirho(Box::new(ShowFieldTypeChirho::VarChirho(
                        "a".to_string(),
                    ))),
                ],
            )],
        },
    )]);
    render_chirho(&mut context_chirho, "NonEmpty Int");
    let (_, precedence_chirho, alts_chirho) = helper_chirho(&context_chirho, "NonEmpty Int");
    let cons_chirho = alternative_chirho(alts_chirho, ":|");
    assert_eq!(
        thresholds_chirho(&cons_chirho.rhs_chirho, precedence_chirho),
        vec![5]
    );
    assert!(strings_chirho(&cons_chirho.rhs_chirho).contains(&" :| ".to_string()));
    let head_chirho = cons_chirho.binders_chirho[0].id_chirho;
    let negative_checks_chirho = all_nodes_chirho(&cons_chirho.rhs_chirho)
        .into_iter()
        .filter(|node_chirho| {
            matches!(node_chirho, CoreExprChirho::PrimOpChirho { name_chirho, args_chirho }
                if name_chirho == "<#"
                    && matches!(args_chirho.first(), Some(CoreExprChirho::VarChirho(id_chirho)) if *id_chirho == head_chirho))
        })
        .count();
    assert_eq!(negative_checks_chirho, 0);
}

#[test]
fn a_monomorphic_public_row_becomes_its_helper_at_zero_chirho() {
    let mut context_chirho = context_chirho(vec![(
        "QChirho",
        monomorphic_chirho(vec![constructor_chirho(
            "QChirho",
            ShowConFormChirho::PrefixChirho,
            vec![type_chirho("Int")],
        )]),
    )]);
    let row_chirho =
        context_chirho.fresh_binder_chirho("$prim_Show_show_QChirho", TyChirho::int_chirho());
    let mut bindings_chirho = vec![CoreBindingChirho {
        binder_chirho: row_chirho,
        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho("naive".to_string())),
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    }];
    context_chirho.rewrite_derived_show_rows_chirho(&mut bindings_chirho);
    let (helper_chirho, _, _) = helper_chirho(&context_chirho, "QChirho");
    let CoreExprChirho::LamChirho { body_chirho, .. } = &bindings_chirho[0].rhs_chirho else {
        panic!("the row takes the value");
    };
    assert_eq!(call_precedences_chirho(body_chirho, helper_chirho), vec![0]);
}

#[test]
fn a_wide_type_graph_is_checked_once_per_key_chirho() {
    // T0Chirho has two fields of T1Chirho, T1Chirho two of T2Chirho, and so on: checked
    // afresh at every field, the work would double at each of the 40 levels.
    let levels_chirho = 40;
    let mut types_chirho = Vec::new();
    for level_chirho in 0..levels_chirho {
        let name_chirho = format!("T{level_chirho}Chirho");
        let next_chirho = type_chirho(&format!("T{}Chirho", level_chirho + 1));
        types_chirho.push((
            name_chirho.clone(),
            monomorphic_chirho(vec![constructor_chirho(
                &name_chirho,
                ShowConFormChirho::PrefixChirho,
                vec![next_chirho.clone(), next_chirho],
            )]),
        ));
    }
    let last_chirho = format!("T{levels_chirho}Chirho");
    types_chirho.push((
        last_chirho.clone(),
        monomorphic_chirho(vec![constructor_chirho(
            &last_chirho,
            ShowConFormChirho::PrefixChirho,
            vec![],
        )]),
    ));
    let context_chirho = context_chirho(
        types_chirho
            .iter()
            .map(|(name_chirho, metadata_chirho)| (name_chirho.as_str(), metadata_chirho.clone()))
            .collect(),
    );
    assert_eq!(
        context_chirho.derived_show_key_chirho("T0Chirho"),
        Some("T0Chirho".to_string())
    );
    assert_eq!(
        context_chirho.derived_show_known_chirho.borrow().len(),
        levels_chirho + 1
    );
}
