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
                    .map(|name_chirho| name_chirho.text_chirho().to_string()),
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
