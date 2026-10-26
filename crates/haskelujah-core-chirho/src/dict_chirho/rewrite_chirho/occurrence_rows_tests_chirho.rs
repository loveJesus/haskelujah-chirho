// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Where a proven occurrence may land: the self dictionary only by agreement
//! with the caller's key, and a `$prim` row only with the dictionaries its
//! instance's context makes it take first.

use std::collections::{HashMap, HashSet};

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, InstDeclChirho, PredChirho};
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use crate::dict_chirho::DictLayoutChirho;
use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho, CoreModuleChirho,
    InlineAnnotationChirho,
};

use super::DictPassCtxChirho;

fn con_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ConChirho(name_chirho.to_string())
}

fn var_expr_chirho(id_chirho: u32) -> CoreExprChirho {
    CoreExprChirho::VarChirho(CoreIdChirho(id_chirho))
}

fn app_chirho(fun_chirho: CoreExprChirho, arg_chirho: CoreExprChirho) -> CoreExprChirho {
    CoreExprChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
    }
}

/// A pass holding one occurrence (id 2) of `describeChirho` (canonical id 1),
/// proved at `proven_key_chirho`, whose class selector is id 3.
fn recursion_pass_chirho(proven_key_chirho: &str) -> DictPassCtxChirho {
    let names_chirho = HashMap::from([
        (CoreIdChirho(1), "describeChirho".to_string()),
        (CoreIdChirho(2), "describeChirho".to_string()),
        (
            CoreIdChirho(3),
            "$sel_DescribableChirho_describeChirho".to_string(),
        ),
    ]);
    let mut pass_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
    pass_chirho.method_occurrence_canon_chirho.insert(
        CoreIdChirho(2),
        ("describeChirho".to_string(), CoreIdChirho(1)),
    );
    pass_chirho.occurrence_evidence_chirho.insert(
        CoreIdChirho(2),
        (
            "DescribableChirho".to_string(),
            proven_key_chirho.to_string(),
        ),
    );
    pass_chirho.method_selectors_chirho.insert(
        "describeChirho".to_string(),
        ("DescribableChirho".to_string(), CoreIdChirho(3)),
    );
    pass_chirho
}

#[test]
fn a_head_only_proof_takes_the_self_dictionary_its_caller_key_names_chirho() {
    let self_dicts_chirho = HashMap::from([
        (
            ("DescribableChirho".to_string(), "[a]".to_string()),
            CoreIdChirho(4),
        ),
        (
            ("DescribableChirho".to_string(), "[Int]".to_string()),
            CoreIdChirho(5),
        ),
    ]);
    let pass_chirho = recursion_pass_chirho("[]");
    assert_eq!(
        pass_chirho.agreeing_self_dictionary_chirho(
            CoreIdChirho(2),
            &|| Some("[a]".to_string()),
            &self_dicts_chirho
        ),
        Some(app_chirho(var_expr_chirho(3), var_expr_chirho(4)))
    );
    // A key with another head, the bare head itself, or no key at all.
    for candidate_chirho in [Some("TreeChirho a"), Some("[]"), None] {
        assert_eq!(
            pass_chirho.agreeing_self_dictionary_chirho(
                CoreIdChirho(2),
                &|| candidate_chirho.map(str::to_string),
                &self_dicts_chirho
            ),
            None,
            "{candidate_chirho:?}"
        );
    }
    // A key this binding holds no dictionary for.
    assert_eq!(
        pass_chirho.agreeing_self_dictionary_chirho(
            CoreIdChirho(2),
            &|| Some("[b]".to_string()),
            &self_dicts_chirho
        ),
        None
    );
    // A whole spelling is no head-only proof, even when a local dictionary
    // carries it: its own row or dictionary serves it.
    assert_eq!(
        recursion_pass_chirho("[Int]").agreeing_self_dictionary_chirho(
            CoreIdChirho(2),
            &|| Some("[Int]".to_string()),
            &self_dicts_chirho
        ),
        None
    );
}

#[test]
fn only_the_bindings_own_key_admits_the_context_dictionary_chirho() {
    assert!(DictPassCtxChirho::proof_admits_context_chirho(
        &haskelujah_typing_chirho::infer_chirho::own_dictionary_key_chirho(None)
    ));
    for key_chirho in ["[]", "[Int]", "TreeChirho", "Int"] {
        assert!(
            !DictPassCtxChirho::proof_admits_context_chirho(key_chirho),
            "{key_chirho}"
        );
    }
}

/// A pass with a context-leading row (id 10, `instance Show Int => MarkChirho
/// FooChirho`), a ground row the pass generates (id 12) and Show Int's
/// dictionary (id 11).
fn rows_pass_chirho(context_chirho: Vec<PredChirho>) -> DictPassCtxChirho {
    let names_chirho = HashMap::from([
        (
            CoreIdChirho(10),
            "$prim_MarkChirho_markChirho_FooChirho".to_string(),
        ),
        (CoreIdChirho(11), "$fShowInt".to_string()),
        (
            CoreIdChirho(12),
            "$prim_NeedsChirho_neededChirho_[Int]".to_string(),
        ),
    ]);
    let mut pass_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
    pass_chirho
        .context_leading_rows_chirho
        .insert(CoreIdChirho(10), context_chirho);
    pass_chirho
        .instance_dicts_chirho
        .insert(("Show".to_string(), "Int".to_string()), CoreIdChirho(11));
    pass_chirho
}

#[test]
fn a_context_leading_row_is_served_only_with_its_context_chirho() {
    let show_int_chirho = PredChirho::new_chirho("Show", con_chirho("Int"));
    let pass_chirho = rows_pass_chirho(vec![show_int_chirho.clone()]);
    assert_eq!(
        pass_chirho.proof_row_chirho("$prim_MarkChirho_markChirho_FooChirho"),
        Some(app_chirho(var_expr_chirho(10), var_expr_chirho(11)))
    );
    // A generated ground row takes no dictionary.
    assert_eq!(
        pass_chirho.proof_row_chirho("$prim_NeedsChirho_neededChirho_[Int]"),
        Some(var_expr_chirho(12))
    );
    assert_eq!(
        pass_chirho.proof_row_chirho("$prim_MarkChirho_markChirho_Int"),
        None
    );
    // A context dictionary that is not registered, over a variable, or for a
    // multi-parameter class: the row is not served.
    let mut unregistered_chirho = rows_pass_chirho(vec![show_int_chirho]);
    unregistered_chirho.instance_dicts_chirho.clear();
    let open_chirho = PredChirho::new_chirho("Show", TyChirho::VarChirho(TyVarChirho(0)));
    let mut multi_chirho = PredChirho::new_chirho("Show", con_chirho("Int"));
    multi_chirho.extra_tys_chirho.push(con_chirho("Bool"));
    for pass_chirho in [
        unregistered_chirho,
        rows_pass_chirho(vec![open_chirho]),
        rows_pass_chirho(vec![multi_chirho]),
    ] {
        assert_eq!(
            pass_chirho.proof_row_chirho("$prim_MarkChirho_markChirho_FooChirho"),
            None
        );
    }
}

fn binding_chirho(id_chirho: u32, name_chirho: &str) -> CoreBindingChirho {
    CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: con_chirho("Int"),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    }
}

fn instance_chirho(
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

fn layout_chirho(class_chirho: &str, method_chirho: &str) -> DictLayoutChirho {
    DictLayoutChirho {
        class_name_chirho: class_chirho.to_string(),
        super_slots_chirho: Vec::new(),
        method_slots_chirho: vec![(method_chirho.to_string(), 0)],
        field_count_chirho: 1,
    }
}

#[test]
fn context_leading_rows_are_read_from_the_class_environment_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    let element_chirho = TyChirho::VarChirho(TyVarChirho(0));
    env_chirho.instances_chirho.insert(
        "MarkChirho".to_string(),
        vec![instance_chirho(
            "MarkChirho",
            con_chirho("FooChirho"),
            vec![PredChirho::new_chirho("Show", con_chirho("Int"))],
        )],
    );
    env_chirho.instances_chirho.insert(
        "NeedsChirho".to_string(),
        vec![
            instance_chirho("NeedsChirho", con_chirho("Int"), Vec::new()),
            instance_chirho(
                "NeedsChirho",
                TyChirho::ListChirho(Box::new(element_chirho.clone())),
                vec![PredChirho::new_chirho("NeedsChirho", element_chirho)],
            ),
        ],
    );
    let module_chirho = CoreModuleChirho {
        name_chirho: "Main".to_string(),
        bindings_chirho: vec![
            binding_chirho(20, "$prim_MarkChirho_markChirho_FooChirho"),
            binding_chirho(21, "$prim_NeedsChirho_neededChirho_[a]"),
            binding_chirho(22, "$prim_NeedsChirho_neededChirho_Int"),
            binding_chirho(23, "mainChirho"),
        ],
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: Vec::new(),
    };
    let mut pass_chirho = DictPassCtxChirho::new_chirho(100, HashMap::new(), HashMap::new());
    for (class_chirho, method_chirho) in [
        ("MarkChirho", "markChirho"),
        ("NeedsChirho", "neededChirho"),
    ] {
        pass_chirho.layouts_chirho.insert(
            class_chirho.to_string(),
            layout_chirho(class_chirho, method_chirho),
        );
    }
    pass_chirho.record_context_leading_rows_chirho(&module_chirho, &env_chirho);
    let mut recorded_chirho: Vec<(u32, Vec<String>)> = pass_chirho
        .context_leading_rows_chirho
        .iter()
        .map(|(id_chirho, context_chirho)| {
            (
                id_chirho.0,
                context_chirho
                    .iter()
                    .map(|pred_chirho| pred_chirho.class_name_chirho.clone())
                    .collect(),
            )
        })
        .collect();
    recorded_chirho.sort();
    // The ground-headed row with a context and the conditional list row; the
    // context-free row and an ordinary binding take no dictionary.
    assert_eq!(
        recorded_chirho,
        vec![
            (20, vec!["Show".to_string()]),
            (21, vec!["NeedsChirho".to_string()]),
        ]
    );
}

#[test]
fn a_builtin_alias_without_dict_params_canonicalizes_despite_reference_evidence_chirho() {
    let names_chirho = HashMap::from([
        (CoreIdChirho(30), "M.fromList".to_string()),
        (CoreIdChirho(31), "mapFromList".to_string()),
    ]);
    let mut pass_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
    pass_chirho
        .method_occurrence_canon_chirho
        .insert(CoreIdChirho(30), ("fromList".to_string(), CoreIdChirho(31)));
    pass_chirho.reference_evidence_chirho.insert(
        CoreIdChirho(30),
        vec![("Ord".to_string(), Some("Int".to_string()))],
    );
    let empty_dicts_chirho = HashMap::new();
    let empty_classes_chirho = HashSet::new();
    let empty_local_dicts_chirho = HashMap::new();
    // Without dictionary parameters on `mapFromList`, the occurrence canonicalizes
    // immediately so `M.fromList` is not left on the spine for `IsList.fromList`.
    assert_eq!(
        pass_chirho.occurrence_head_replacement_chirho(
            CoreIdChirho(30),
            &empty_dicts_chirho,
            &empty_classes_chirho,
            &empty_local_dicts_chirho,
            &|| None,
        ),
        Some(var_expr_chirho(31))
    );
    // When the canonical binding takes dictionary parameters, the occurrence id is
    // kept for the dictionary-parameter call-site path.
    pass_chirho
        .dict_param_bindings_chirho
        .insert(CoreIdChirho(31), vec!["Ord".to_string()]);
    assert_eq!(
        pass_chirho.occurrence_head_replacement_chirho(
            CoreIdChirho(30),
            &empty_dicts_chirho,
            &empty_classes_chirho,
            &empty_local_dicts_chirho,
            &|| None,
        ),
        None
    );
}

/// A pass holding one `sum` occurrence (id 2, canonical id 1) proved `Num Double`, whose
/// rows and `foldl` (ids 3, 4, 5) exist only as name-table entries, applied to `xs` (id 6).
fn aggregate_pass_chirho() -> DictPassCtxChirho {
    let names_chirho = HashMap::from([
        (CoreIdChirho(1), "sum".to_string()),
        (CoreIdChirho(2), "sum".to_string()),
        (CoreIdChirho(3), "$prim_Num_+_Double".to_string()),
        (CoreIdChirho(4), "$prim_Num_fromInteger_Double".to_string()),
        (CoreIdChirho(5), "foldl".to_string()),
        (CoreIdChirho(6), "xs".to_string()),
    ]);
    let mut pass_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
    pass_chirho
        .method_occurrence_canon_chirho
        .insert(CoreIdChirho(2), ("sum".to_string(), CoreIdChirho(1)));
    pass_chirho
        .occurrence_evidence_chirho
        .insert(CoreIdChirho(2), ("Num".to_string(), "Double".to_string()));
    pass_chirho
}

/// Row 552: a proven `sum` lands on `foldl $prim_Num_+_K ($prim_Num_fromInteger_K 0)` only
/// when those rows have actual bodies. Dangling name-table entries keep the canonical route.
#[test]
fn a_proven_sum_folds_only_over_body_backed_rows_chirho() {
    let applied_chirho = app_chirho(var_expr_chirho(2), var_expr_chirho(6));
    let mut pass_chirho = aggregate_pass_chirho();
    let unchanged_chirho = pass_chirho.rewrite_method_refs_chirho(&applied_chirho, &HashMap::new());
    assert!(
        matches!(&unchanged_chirho, CoreExprChirho::AppChirho { fun_chirho, arg_chirho }
            if matches!(fun_chirho.as_ref(), CoreExprChirho::VarChirho(CoreIdChirho(1 | 2)))
                && matches!(arg_chirho.as_ref(), CoreExprChirho::VarChirho(CoreIdChirho(6)))),
        "dangling names must keep the canonical sum route, got {unchanged_chirho:?}"
    );

    for (id_chirho, name_chirho) in [
        (3, "$prim_Num_+_Double"),
        (4, "$prim_Num_fromInteger_Double"),
        (5, "foldl"),
    ] {
        pass_chirho
            .generated_bindings_chirho
            .push(binding_chirho(id_chirho, name_chirho));
    }
    let folded_chirho = pass_chirho.rewrite_method_refs_chirho(&applied_chirho, &HashMap::new());
    let expected_chirho = app_chirho(
        app_chirho(
            app_chirho(var_expr_chirho(5), var_expr_chirho(3)),
            app_chirho(
                var_expr_chirho(4),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
            ),
        ),
        var_expr_chirho(6),
    );
    assert_eq!(
        folded_chirho, expected_chirho,
        "body-backed rows give the Double fold"
    );
}
