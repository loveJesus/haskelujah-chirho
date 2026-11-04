// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: associated equations keep their written arguments (row 686).
//! An instance body's `type` member, in either spelling, is lowered from its own
//! header and never rebuilt from the instance head; a class body's associated
//! declaration counts only its written parameters.
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, TyVarChirho};

use super::test_support_chirho::{
    find_class_decl_chirho, parse_and_lower_chirho, type_shape_chirho,
};

/// Every associated instance in `source_chirho`, rendered as its family name
/// followed by the shapes of its written arguments in order.
fn associated_equations_chirho(source_chirho: &str) -> Vec<String> {
    let module_chirho = parse_and_lower_chirho(source_chirho);
    let mut rendered_chirho = Vec::new();
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::InstanceDeclChirho {
            assoc_tf_instances_chirho,
            ..
        } = decl_chirho
        {
            for instance_chirho in assoc_tf_instances_chirho {
                let mut words_chirho =
                    vec![instance_chirho.family_name_chirho.text_chirho().to_string()];
                words_chirho.extend(
                    instance_chirho
                        .lhs_types_chirho
                        .iter()
                        .map(type_shape_chirho),
                );
                rendered_chirho.push(words_chirho.join(" "));
            }
        }
    }
    rendered_chirho
}

/// The parameters a class body declares for each of its associated families.
fn associated_parameters_chirho(source_chirho: &str, class_chirho: &str) -> Vec<String> {
    let module_chirho = parse_and_lower_chirho(source_chirho);
    let DeclChirho::ClassDeclChirho {
        associated_tfs_chirho,
        ..
    } = find_class_decl_chirho(&module_chirho, class_chirho)
    else {
        panic!("expected a class declaration");
    };
    associated_tfs_chirho
        .iter()
        .map(|family_chirho| {
            let mut words_chirho = vec![family_chirho.name_chirho.text_chirho().to_string()];
            words_chirho.extend(
                family_chirho
                    .type_vars_chirho
                    .iter()
                    .filter(|binder_chirho| binder_chirho.is_visible_chirho())
                    .map(|binder_chirho| binder_chirho.name_chirho.text_chirho().to_string()),
            );
            words_chirho.join(" ")
        })
        .collect()
}

#[test]
fn an_associated_wildcard_keeps_its_place_chirho() {
    // V03: GHC 9.14.1 prints True; before row 686 the `_` was erased here.
    let source_chirho = "module M where
class ShapeChirho a where
  type ResultChirho a b
instance ShapeChirho Int where
  type ResultChirho Int _ = Bool
";
    assert_eq!(
        associated_equations_chirho(source_chirho),
        ["ResultChirho Int _"]
    );
    assert_eq!(
        associated_parameters_chirho(source_chirho, "ShapeChirho"),
        ["ResultChirho a b"]
    );
}

#[test]
fn a_named_binder_twin_keeps_its_place_chirho() {
    // V03n, the wildcard's named twin, measured right on main and the candidate.
    assert_eq!(
        associated_equations_chirho(
            "module M where
class ShapeChirho a where
  type ResultChirho a b
instance ShapeChirho Int where
  type ResultChirho Int b = Bool
"
        ),
        ["ResultChirho Int b"]
    );
}

#[test]
fn the_instance_keyword_spelling_keeps_its_wildcard_chirho() {
    // V03i: `type instance` inside an instance body is the same equation.
    assert_eq!(
        associated_equations_chirho(
            "module M where
class ShapeChirho a where
  type ResultChirho a b
instance ShapeChirho Int where
  type instance ResultChirho Int _ = Bool
"
        ),
        ["ResultChirho Int _"]
    );
}

#[test]
fn a_top_level_open_instance_keeps_its_wildcard_chirho() {
    // V06: GHC 9.14.1 prints True for `GChirho Int Char`.
    let module_chirho = parse_and_lower_chirho(
        "module M where
type family GChirho a b
type instance GChirho Int _ = Bool
",
    );
    let arguments_chirho: Vec<String> = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeFamilyInstanceDeclChirho {
                lhs_types_chirho, ..
            } => Some(lhs_types_chirho.iter().map(type_shape_chirho).collect()),
            _ => None,
        })
        .expect("expected a type family instance");
    assert_eq!(arguments_chirho, ["Int", "_"]);
}

#[test]
fn a_parenthesized_operator_names_its_open_family_chirho() {
    // T14010: `(~>)` before any argument is the family's name, not an argument.
    // A parenthesized argument after a named family stays an argument.
    let module_chirho = parse_and_lower_chirho(
        "module M where
type family (~>) :: c -> c -> Type
type instance (~>) = (->)
type instance (++) a b = a
type instance GChirho (Maybe a) = a
",
    );
    let instances_chirho: Vec<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeFamilyInstanceDeclChirho {
                family_name_chirho,
                lhs_types_chirho,
                ..
            } => {
                let mut words_chirho = vec![family_name_chirho.text_chirho().to_string()];
                words_chirho.extend(lhs_types_chirho.iter().map(type_shape_chirho));
                Some(words_chirho.join(" "))
            }
            _ => None,
        })
        .collect();
    assert_eq!(instances_chirho, ["~>", "++ a b", "GChirho (Maybe a)"]);
}

#[test]
fn a_family_over_a_subset_of_class_parameters_keeps_its_written_arguments_chirho() {
    // T11401's shape: each family names only some class parameters, so the old
    // rebuild from the whole instance head gave every equation three arguments.
    let source_chirho = "module M where
class CallArgs f g r where
  type CalledFunction g :: Type
  type CallerFunction f r :: Type
  call :: f -> g
instance CallArgs (IO a) (CodeGen r (Value a)) r where
  type CalledFunction (CodeGen r (Value a)) = IO a
  type CallerFunction (IO a) r = CodeGen r (Value a)
  call = undefined
";
    assert_eq!(
        associated_equations_chirho(source_chirho),
        [
            "CalledFunction ((CodeGen r) (Value a))",
            "CallerFunction (IO a) r"
        ]
    );
    assert_eq!(
        associated_parameters_chirho(source_chirho, "CallArgs"),
        ["CalledFunction g", "CallerFunction f r"]
    );
}

#[test]
fn a_result_kind_binds_no_associated_parameter_chirho() {
    // T17566's shape: the family's one parameter is not a class parameter, and
    // the variables of its result kind `f a` are not parameters either.
    let source_chirho = "module M where
class C1 (f :: k -> Type) z where
  type T1 (x :: f a) :: f a
instance C1 Proxy z where
  type T1 x = F1 x
";
    assert_eq!(associated_parameters_chirho(source_chirho, "C1"), ["T1 x"]);
    assert_eq!(associated_equations_chirho(source_chirho), ["T1 x"]);
}

#[test]
fn a_nullary_associated_instance_has_no_arguments_chirho() {
    // T16008's shape: `S` is declared by a kind signature and takes no argument.
    assert_eq!(
        associated_equations_chirho(
            "module M where
class C k where
  type S :: k -> Type
instance C (D a) where
  type S = SD
"
        ),
        ["S"]
    );
}

#[test]
fn a_visible_kind_argument_is_not_a_family_argument_chirho() {
    // T22560c's shape: `@Bool` instantiates the invisible `@j`.
    let source_chirho = "module M where
class C @j (a :: j) where
  type F @j a
instance C True where
  type F @Bool True = ()
";
    assert_eq!(associated_equations_chirho(source_chirho), ["F True"]);
    assert_eq!(associated_parameters_chirho(source_chirho, "C"), ["F a"]);
}

#[test]
fn a_promoted_tuple_argument_keeps_its_shape_chirho() {
    // T16394's shape (GHC rejects the instance for its context, not its header).
    assert_eq!(
        associated_equations_chirho(
            "module M where
class C a where
  type T (n :: a)
instance C (a, b) where
  type T '(n, m) = (T n, T m)
"
        ),
        ["T (('(,) n) m)"]
    );
}

#[test]
fn a_class_default_keeps_its_parameters_and_right_side_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where
class C a where
  type F a = [a]
",
    );
    let DeclChirho::ClassDeclChirho {
        associated_tfs_chirho,
        ..
    } = find_class_decl_chirho(&module_chirho, "C")
    else {
        panic!("expected a class declaration");
    };
    assert_eq!(associated_tfs_chirho.len(), 1);
    let names_chirho: Vec<&str> = associated_tfs_chirho[0]
        .type_vars_chirho
        .iter()
        .map(|name_chirho| name_chirho.text_chirho())
        .collect();
    assert_eq!(names_chirho, ["a"]);
    let default_chirho = associated_tfs_chirho[0]
        .default_rhs_chirho
        .as_ref()
        .expect("the default right side is kept");
    assert_eq!(type_shape_chirho(default_chirho), "[a]");
}

#[test]
fn crossed_class_defaults_are_kept_chirho() {
    // T5481: each default names the other class parameter, which is not one of
    // its family's own; it is still the written default, and each bodiless
    // declaration stays without one.
    let module_chirho = parse_and_lower_chirho(
        "module M where
class Foo a b where
    type X a
    type X a = b
    type Y b
    type Y b = a
",
    );
    let DeclChirho::ClassDeclChirho {
        associated_tfs_chirho,
        ..
    } = find_class_decl_chirho(&module_chirho, "Foo")
    else {
        panic!("expected a class declaration");
    };
    let defaults_chirho: Vec<(&str, Option<String>)> = associated_tfs_chirho
        .iter()
        .map(|family_chirho| {
            (
                family_chirho.name_chirho.text_chirho(),
                family_chirho
                    .default_rhs_chirho
                    .as_ref()
                    .map(type_shape_chirho),
            )
        })
        .collect();
    assert_eq!(
        defaults_chirho,
        [
            ("X", None),
            ("X", Some("b".to_string())),
            ("Y", None),
            ("Y", Some("a".to_string()))
        ]
    );
}

/// Each associated family a class declares: name, parameters, the names its
/// written default may use (`scope`), and the default's shape.
fn class_families_chirho(source_chirho: &str, class_chirho: &str) -> Vec<String> {
    let module_chirho = parse_and_lower_chirho(source_chirho);
    let DeclChirho::ClassDeclChirho {
        associated_tfs_chirho,
        ..
    } = find_class_decl_chirho(&module_chirho, class_chirho)
    else {
        panic!("expected a class declaration");
    };
    associated_tfs_chirho
        .iter()
        .map(|family_chirho| {
            let params_chirho: Vec<&str> = family_chirho
                .type_vars_chirho
                .iter()
                .map(|name_chirho| name_chirho.text_chirho())
                .collect();
            let mut text_chirho = format!(
                "{} [{}]",
                family_chirho.name_chirho.text_chirho(),
                params_chirho.join(" ")
            );
            if let Some(binders_chirho) = &family_chirho.default_binders_chirho {
                let scope_chirho: Vec<&str> = binders_chirho
                    .iter()
                    .map(|binder_chirho| binder_chirho.name_chirho.text_chirho())
                    .collect();
                text_chirho.push_str(&format!(" scope [{}]", scope_chirho.join(" ")));
            }
            if let Some(default_chirho) = &family_chirho.default_rhs_chirho {
                text_chirho.push_str(&format!(" = {}", type_shape_chirho(default_chirho)));
            }
            text_chirho
        })
        .collect()
}

#[test]
fn a_default_scope_binds_the_kind_variables_its_binders_write_chirho() {
    // T18585's arrow kind, an applied kind and a list kind: each default's kind
    // variables are bound by its own binder, never counted as parameters.
    let source_chirho = "module M where
class FunctorChirho (f :: i -> j) where
  type DomChirho f :: Cat i
  type DomChirho (f :: i -> j) = SelfDom f (DefaultCat i)
class BoxChirho (f :: Maybe k) where
  type ElemChirho f :: Type
  type ElemChirho (f :: Maybe k) = Proxy k
class ListChirho (f :: [k]) where
  type ItemChirho f :: Type
  type ItemChirho (f :: [k]) = Proxy k
";
    assert_eq!(
        class_families_chirho(source_chirho, "FunctorChirho"),
        [
            "DomChirho [f]",
            "DomChirho [f] scope [f i j] = ((SelfDom f) (DefaultCat i))"
        ]
    );
    assert_eq!(
        class_families_chirho(source_chirho, "BoxChirho"),
        ["ElemChirho [f]", "ElemChirho [f] scope [f k] = (Proxy k)"]
    );
    assert_eq!(
        class_families_chirho(source_chirho, "ListChirho"),
        ["ItemChirho [f]", "ItemChirho [f] scope [f k] = (Proxy k)"]
    );
}

#[test]
fn a_nested_forall_binds_only_inside_its_group_chirho() {
    // `k` is bound by the inner `forall` alone, so the default may not use it;
    // `j` stays free in the outer kind and is the default's to use.
    let source_chirho = "module M where
class ApplyChirho (f :: forall k. k -> Type) where
  type ResChirho f :: Type
  type ResChirho (f :: forall k. k -> Type) = Proxy k
class LiftChirho (g :: (forall k. k -> Type) -> j) where
  type OutChirho g :: Type
  type OutChirho (g :: (forall k. k -> Type) -> j) = Proxy j
";
    assert_eq!(
        class_families_chirho(source_chirho, "ApplyChirho"),
        ["ResChirho [f]", "ResChirho [f] scope [f] = (Proxy k)"]
    );
    assert_eq!(
        class_families_chirho(source_chirho, "LiftChirho"),
        ["OutChirho [g]", "OutChirho [g] scope [g j] = (Proxy j)"]
    );
}

#[test]
fn an_injectivity_annotation_stays_with_its_declaration_chirho() {
    // T26255a's shape: `= (r :: Symbol) | r -> era` names the result. It is no
    // default, and the members after it stay in the class.
    let source_chirho = "module M where
class EraChirho era where
  type EraNameChirho era = (r :: Symbol) | r -> era
  type ProtVerLowChirho era :: Nat
  eraNameChirho :: Proxy era -> String
";
    assert_eq!(
        class_families_chirho(source_chirho, "EraChirho"),
        ["EraNameChirho [era]", "ProtVerLowChirho [era]"]
    );
    let module_chirho = parse_and_lower_chirho(source_chirho);
    let DeclChirho::ClassDeclChirho { methods_chirho, .. } =
        find_class_decl_chirho(&module_chirho, "EraChirho")
    else {
        panic!("expected a class declaration");
    };
    let methods_chirho: Vec<&str> = methods_chirho
        .iter()
        .map(|method_chirho| method_chirho.name_chirho.text_chirho())
        .collect();
    assert_eq!(methods_chirho, ["eraNameChirho"]);
}

#[test]
fn a_synonym_after_an_instance_stays_a_synonym_chirho() {
    // The instance-body reading ends with the instance's own members.
    let module_chirho = parse_and_lower_chirho(
        "module M where
instance ShapeChirho Int where
  type ResultChirho Int b = Bool
type PairChirho a = (a, a)
",
    );
    let binders_chirho: Vec<&str> = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                ..
            } if name_chirho.text_chirho() == "PairChirho" => Some(
                type_vars_chirho
                    .iter()
                    .map(|binder_chirho| binder_chirho.name_chirho.text_chirho())
                    .collect(),
            ),
            _ => None,
        })
        .expect("the top-level synonym stays a type synonym");
    assert_eq!(binders_chirho, ["a"]);
}

#[test]
fn an_invisible_family_binder_is_not_a_visible_parameter_chirho() {
    // T22560d: `@(k :: Type)` binds an inferred kind, like `@k`, so the family's
    // visible arity is one and its equation `F a = a` matches it.
    let module_chirho = parse_and_lower_chirho(
        "module M where
type family F @(k :: Type) (a :: k) :: k where
  F a = a
",
    );
    let DeclChirho::TypeFamilyDeclChirho {
        type_vars_chirho,
        equations_chirho,
        ..
    } = &module_chirho.decls_chirho[0]
    else {
        panic!("expected a type family declaration");
    };
    let visible_binders_chirho = TyVarChirho::visible_binders_chirho(type_vars_chirho);
    let visible_chirho: Vec<&str> = visible_binders_chirho
        .iter()
        .map(|binder_chirho| binder_chirho.name_chirho.text_chirho())
        .collect();
    assert_eq!(visible_chirho, ["a"]);
    assert_eq!(equations_chirho.len(), 1);
    assert_eq!(
        equations_chirho[0].lhs_types_chirho.len(),
        visible_chirho.len()
    );
}

#[test]
fn associated_headers_keep_their_full_telescope_and_result_kind_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE PolyKinds, TypeAbstractions, TypeFamilies #-}\nmodule AssociatedKindsChirho where\nclass CChirho @kChirho (aChirho :: kChirho) where\n  type FChirho @kChirho (xChirho :: Maybe kChirho) :: Either kChirho Bool\n",
    );
    let DeclChirho::ClassDeclChirho {
        associated_tfs_chirho,
        ..
    } = find_class_decl_chirho(&module_chirho, "CChirho")
    else {
        panic!("class");
    };
    let family_chirho = &associated_tfs_chirho[0];
    assert_eq!(family_chirho.type_vars_chirho.len(), 2);
    assert!(!family_chirho.type_vars_chirho[0].is_visible_chirho());
    assert!(family_chirho.type_vars_chirho[1].is_visible_chirho());
    assert_eq!(
        family_chirho.type_vars_chirho[1]
            .kind_annotation_chirho
            .as_deref()
            .map(type_shape_chirho)
            .as_deref(),
        Some("(Maybe kChirho)")
    );
    assert_eq!(
        family_chirho
            .result_kind_chirho
            .as_ref()
            .map(type_shape_chirho)
            .as_deref(),
        Some("((Either kChirho) Bool)")
    );
    assert_eq!(
        associated_parameters_chirho(
            "module M where\nclass CChirho aChirho where\n  type FChirho @kChirho (xChirho :: Maybe kChirho) :: Either kChirho Bool\n",
            "CChirho"
        ),
        ["FChirho xChirho"]
    );
}

#[test]
fn a_plain_member_binder_is_not_hidden_by_the_class_binders_spelling_chirho() {
    assert_eq!(
        associated_parameters_chirho(
            "{-# LANGUAGE TypeAbstractions, TypeFamilies #-}\nmodule M where\nclass CChirho @kChirho aChirho where\n  type FChirho kChirho aChirho\n",
            "CChirho"
        ),
        ["FChirho kChirho aChirho"]
    );
}

#[test]
fn associated_result_injectivity_keeps_its_kind_and_following_method_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies #-}\nmodule AssociatedResultChirho where\nclass CChirho aChirho where\n  type FChirho aChirho = (rChirho :: Type) | rChirho -> aChirho\n  methodChirho :: aChirho -> aChirho\n",
    );
    let DeclChirho::ClassDeclChirho {
        associated_tfs_chirho,
        methods_chirho,
        ..
    } = find_class_decl_chirho(&module_chirho, "CChirho")
    else {
        panic!("class");
    };
    let family_chirho = &associated_tfs_chirho[0];
    assert_eq!(family_chirho.type_vars_chirho.len(), 1);
    assert_eq!(
        family_chirho
            .result_kind_chirho
            .as_ref()
            .map(type_shape_chirho)
            .as_deref(),
        Some("Type")
    );
    assert!(family_chirho.default_rhs_chirho.is_none());
    assert!(family_chirho.default_binders_chirho.is_none());
    assert!(
        methods_chirho
            .iter()
            .any(|method_chirho| method_chirho.name_chirho.text_chirho() == "methodChirho")
    );
}

#[test]
fn written_defaults_keep_scoped_kind_names_after_the_canonical_family_route_chirho() {
    for (kind_chirho, expected_chirho) in [
        ("iChirho -> jChirho", vec!["fChirho", "iChirho", "jChirho"]),
        ("Maybe kChirho", vec!["fChirho", "kChirho"]),
        ("[kChirho]", vec!["fChirho", "kChirho"]),
        ("forall kChirho. kChirho -> Type", vec!["fChirho"]),
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE PolyKinds, RankNTypes, TypeFamilies #-}}\nmodule ScopedDefaultChirho where\nclass CChirho aChirho where\n  type FChirho (fChirho :: {kind_chirho}) = fChirho\n"
        );
        let module_chirho = parse_and_lower_chirho(&source_chirho);
        let DeclChirho::ClassDeclChirho {
            associated_tfs_chirho,
            ..
        } = find_class_decl_chirho(&module_chirho, "CChirho")
        else {
            panic!("class");
        };
        let family_chirho = &associated_tfs_chirho[0];
        let binders_chirho = family_chirho
            .default_binders_chirho
            .as_ref()
            .expect("written default scope retained");
        assert_eq!(
            binders_chirho
                .iter()
                .map(|binder_chirho| binder_chirho.name_chirho.text_chirho())
                .collect::<Vec<_>>(),
            expected_chirho
        );
        assert_eq!(
            family_chirho
                .default_params_chirho
                .iter()
                .map(|name_chirho| name_chirho.text_chirho())
                .collect::<Vec<_>>(),
            ["fChirho"]
        );
        assert_eq!(
            family_chirho
                .type_vars_chirho
                .iter()
                .filter(|binder_chirho| binder_chirho.is_visible_chirho())
                .count(),
            1
        );
    }
}
