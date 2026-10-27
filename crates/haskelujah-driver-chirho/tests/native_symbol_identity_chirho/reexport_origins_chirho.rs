// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 642: an imported value links to the program module that defines it, whatever module
//! forwarded it, and a value no program module defines keeps the builtin route. Each program and
//! its expected output are GHC 9.14.1's, measured on the text embedded here, and each runs on the
//! interpreter, LLVM and true Cranelift. Before row 629 q4 to q4d and m1 were exact; row 629
//! refused them, and row 642 restores them.
//! workflow: compiler-pipeline-chirho/native-symbol-identity-chirho

use haskelujah_core_chirho::expr_chirho::CoreIdChirho;
use haskelujah_naming_chirho::{ModuleIfaceChirho, build_iface_with_imports_chirho};
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::FileIdChirho;

use super::import_identity_chirho::{
    ProgramChirho, assert_exact_chirho, assert_refused_chirho, compiled_core_chirho,
    identities_chirho, imported_identities_chirho,
};

/// q4: A forwards Prelude's `map`. GHC prints '[2,3,4]'.
const Q4_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (map, helper) where
helper :: Int
helper = 1
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (map (+ helper) [1, 2, 3 :: Int])
"#,
        ),
    ],
    r#"[2,3,4]
"#,
);

/// q4b: A forwards the class method `show`. GHC prints '5'.
const Q4B_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (show) where
"#,
        ),
        (
            "Main",
            r#"module Main where
import Prelude hiding (show)
import A
main :: IO ()
main = putStrLn (show (5 :: Int))
"#,
        ),
    ],
    r#"5
"#,
);

/// q4c: A forwards the IO action `putStrLn`. GHC prints 'hi'.
const Q4C_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (putStrLn) where
"#,
        ),
        (
            "Main",
            r#"module Main where
import Prelude hiding (putStrLn)
import A
main :: IO ()
main = putStrLn "hi"
"#,
        ),
    ],
    r#"hi
"#,
);

/// q4d: a project Prelude forwards `module Prelude`. GHC prints '[2,4,6]'.
const Q4D_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "MyPrelude",
            r#"module MyPrelude (module Prelude, twice) where
twice :: Int -> Int
twice = (* 2)
"#,
        ),
        (
            "Main",
            r#"module Main where
import Prelude ()
import MyPrelude
main :: IO ()
main = print (map twice [1, 2, 3])
"#,
        ),
    ],
    r#"[2,4,6]
"#,
);

/// m1: A and B both forward `map`, used through each qualifier. GHC prints '([2,3],[6])'.
const M1_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (map) where
"#,
        ),
        (
            "B",
            r#"module B (map) where
"#,
        ),
        (
            "Main",
            r#"module Main where
import qualified A
import qualified B
main :: IO ()
main = print (A.map (+ 1) [1, 2 :: Int], B.map (* 2) [3 :: Int])
"#,
        ),
    ],
    r#"([2,3],[6])
"#,
);

/// a1: A forwards Prelude's `map`; B defines its own `map`. GHC prints '([2,3],1005)'.
const A1_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (map) where
"#,
        ),
        (
            "B",
            r#"module B (map) where
import Prelude hiding (map)
map :: Int -> Int
map = (+ 1000)
"#,
        ),
        (
            "Main",
            r#"module Main where
import qualified A
import qualified B
main :: IO ()
main = print (A.map (+ 1) [1, 2 :: Int], B.map 5)
"#,
        ),
    ],
    r#"([2,3],1005)
"#,
);

/// q1: C defines `f`, B and then A forward it; D has its own. GHC prints '(6,50)'.
const Q1_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "C",
            r#"module C (f) where
f :: Int -> Int
f = (+ 1)
"#,
        ),
        (
            "B",
            r#"module B (f) where
import C (f)
"#,
        ),
        (
            "A",
            r#"module A (f) where
import B (f)
"#,
        ),
        (
            "D",
            r#"module D (f) where
f :: Int -> Int
f = (* 10)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
import qualified D
main :: IO ()
main = print (f 5, D.f 5)
"#,
        ),
    ],
    r#"(6,50)
"#,
);

/// q2: B's `f` reached through A and through B; C has its own. GHC prints '(6,50)'.
const Q2_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (f) where
f :: Int -> Int
f = (+ 1)
"#,
        ),
        (
            "A",
            r#"module A (f) where
import B (f)
"#,
        ),
        (
            "C",
            r#"module C (f) where
f :: Int -> Int
f = (* 10)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
import B
import qualified C
main :: IO ()
main = print (f 5, C.f 5)
"#,
        ),
    ],
    r#"(6,50)
"#,
);

/// q5: A forwards B's `f` with `module B`; C has its own. GHC prints '(6,50)'.
const Q5_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (f) where
f :: Int -> Int
f = (+ 1)
"#,
        ),
        (
            "A",
            r#"module A (module B) where
import B
"#,
        ),
        (
            "C",
            r#"module C (f) where
f :: Int -> Int
f = (* 10)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
import qualified C
main :: IO ()
main = print (f 5, C.f 5)
"#,
        ),
    ],
    r#"(6,50)
"#,
);

/// q6: B's `f` through `import qualified A as Q`; C has its own. GHC prints '(6,50)'.
const Q6_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (f) where
f :: Int -> Int
f = (+ 1)
"#,
        ),
        (
            "A",
            r#"module A (f) where
import B (f)
"#,
        ),
        (
            "C",
            r#"module C (f) where
f :: Int -> Int
f = (* 10)
"#,
        ),
        (
            "Main",
            r#"module Main where
import qualified A as Q
import qualified C
main :: IO ()
main = print (Q.f 5, C.f 5)
"#,
        ),
    ],
    r#"(6,50)
"#,
);

/// q7: A forwards B's `f`, and nothing else defines one. GHC prints '6'.
const Q7_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (f) where
f :: Int -> Int
f = (+ 1)
"#,
        ),
        (
            "A",
            r#"module A (f) where
import B (f)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (f 5)
"#,
        ),
    ],
    r#"6
"#,
);

/// q3: A's forwarded `f` (B's) and C's own `f`, both unqualified; GHC rejects it (GHC-87543).
const Q3_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (f) where
f :: Int -> Int
f = (+ 1)
"#,
        ),
        (
            "A",
            r#"module A (f) where
import B (f)
"#,
        ),
        (
            "C",
            r#"module C (f) where
f :: Int -> Int
f = (* 10)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
import C
main :: IO ()
main = print (f 5)
"#,
        ),
    ],
    r#"cannot link `f`: it is ambiguous, defined by B, C (row 642)"#,
);

/// Every module-level id `core_chirho` names `name_chirho`: a top-level binder or a free
/// reference. A stale name, which no expression uses, and a local binder inside generated code
/// are no reference to the name.
fn ids_named_chirho(
    core_chirho: &haskelujah_core_chirho::CoreModuleChirho,
    name_chirho: &str,
) -> Vec<CoreIdChirho> {
    let free_chirho = haskelujah_core_chirho::module_free_reference_ids_chirho(core_chirho);
    core_chirho
        .names_chirho
        .iter()
        .filter(|(id_chirho, text_chirho)| {
            *text_chirho == name_chirho
                && (free_chirho.contains(id_chirho)
                    || core_chirho.bindings_chirho.iter().any(|binding_chirho| {
                        binding_chirho.binder_chirho.id_chirho == **id_chirho
                    }))
        })
        .map(|(id_chirho, _)| *id_chirho)
        .collect()
}

/// Whether `id_chirho` takes the compiler's route in `core_chirho`: a compiler-owned binding of
/// the module's own Core, or a free reference its provenance records as builtin.
fn takes_compiler_route_chirho(
    core_chirho: &haskelujah_core_chirho::CoreModuleChirho,
    id_chirho: &CoreIdChirho,
) -> bool {
    let provenance_chirho = &core_chirho.provenance_chirho;
    provenance_chirho
        .compiler_bindings_chirho
        .contains(id_chirho)
        || provenance_chirho
            .builtin_references_chirho
            .contains(id_chirho)
}

/// Every id Main names each of `names_chirho` takes the compiler's route, none is an imported
/// user reference, and no user identity carries the name. So the exact output cannot come from
/// linking a user binding by its spelling.
fn assert_builtin_route_chirho(program_chirho: ProgramChirho, names_chirho: &[&str]) {
    let core_chirho = compiled_core_chirho(program_chirho, "Main");
    let imported_chirho = &core_chirho.provenance_chirho.imported_references_chirho;
    for name_chirho in names_chirho {
        let ids_chirho = ids_named_chirho(&core_chirho, name_chirho);
        assert!(!ids_chirho.is_empty(), "Main names `{name_chirho}`");
        for id_chirho in &ids_chirho {
            assert!(
                takes_compiler_route_chirho(&core_chirho, id_chirho),
                "`{name_chirho}` ({id_chirho:?}) takes the compiler's route"
            );
            assert!(
                !imported_chirho.contains_key(id_chirho),
                "`{name_chirho}` is no imported user reference"
            );
        }
        assert!(
            !imported_chirho
                .values()
                .any(|reference_chirho| reference_chirho.name_chirho == *name_chirho),
            "no user identity names `{name_chirho}`"
        );
    }
}

#[test]
fn a_forwarded_builtin_value_takes_the_builtin_route_chirho() {
    assert_exact_chirho(Q4_CHIRHO);
    assert_builtin_route_chirho(Q4_CHIRHO.0, &["map"]);
}

#[test]
fn a_forwarded_builtin_method_takes_the_builtin_route_chirho() {
    assert_exact_chirho(Q4B_CHIRHO);
    assert_builtin_route_chirho(Q4B_CHIRHO.0, &["show"]);
}

#[test]
fn a_forwarded_builtin_action_takes_the_builtin_route_chirho() {
    assert_exact_chirho(Q4C_CHIRHO);
    assert_builtin_route_chirho(Q4C_CHIRHO.0, &["putStrLn"]);
}

#[test]
fn a_project_prelude_forwards_the_builtins_chirho() {
    assert_exact_chirho(Q4D_CHIRHO);
    assert_builtin_route_chirho(Q4D_CHIRHO.0, &["map", "print"]);
    assert_eq!(
        imported_identities_chirho(Q4D_CHIRHO.0, "Main"),
        identities_chirho(&[("twice", &["MyPrelude"])])
    );
}

#[test]
fn one_builtin_through_two_forwarders_stays_builtin_chirho() {
    assert_exact_chirho(M1_CHIRHO);
    assert_builtin_route_chirho(M1_CHIRHO.0, &["map"]);
}

#[test]
fn a_forwarded_builtin_and_a_source_namesake_stay_apart_chirho() {
    assert_exact_chirho(A1_CHIRHO);
    let core_chirho = compiled_core_chirho(A1_CHIRHO.0, "Main");
    let provenance_chirho = &core_chirho.provenance_chirho;
    // B's own `map` is one user identity, defined by B: never compiler-owned, never builtin.
    let user_chirho: Vec<CoreIdChirho> = provenance_chirho
        .imported_references_chirho
        .iter()
        .filter(|(_, reference_chirho)| reference_chirho.name_chirho == "map")
        .map(|(id_chirho, reference_chirho)| {
            assert_eq!(reference_chirho.defining_modules_chirho, ["B"]);
            *id_chirho
        })
        .collect();
    assert_eq!(user_chirho.len(), 1, "one user identity for B's `map`");
    assert!(!takes_compiler_route_chirho(&core_chirho, &user_chirho[0]));
    // A's forwarded `map` is every other id of that name, and each takes the compiler's route.
    let forwarded_chirho: Vec<CoreIdChirho> = ids_named_chirho(&core_chirho, "map")
        .into_iter()
        .filter(|id_chirho| *id_chirho != user_chirho[0])
        .collect();
    assert!(!forwarded_chirho.is_empty(), "Main names A's `map`");
    for id_chirho in &forwarded_chirho {
        assert!(takes_compiler_route_chirho(&core_chirho, id_chirho));
    }
}

#[test]
fn a_chain_of_forwarders_names_the_first_definer_chirho() {
    assert_exact_chirho(Q1_CHIRHO);
    assert_eq!(
        imported_identities_chirho(Q1_CHIRHO.0, "Main"),
        identities_chirho(&[("f", &["C"]), ("f", &["D"])])
    );
}

#[test]
fn two_paths_to_one_definition_are_one_identity_chirho() {
    assert_exact_chirho(Q2_CHIRHO);
    // `f` through A and through B is B's one `f`: never two identities, never ambiguous.
    assert_eq!(
        imported_identities_chirho(Q2_CHIRHO.0, "Main"),
        identities_chirho(&[("f", &["B"]), ("f", &["C"])])
    );
}

#[test]
fn a_module_re_export_names_its_definer_chirho() {
    assert_exact_chirho(Q5_CHIRHO);
}

#[test]
fn a_qualified_alias_of_a_forwarder_names_the_definer_chirho() {
    assert_exact_chirho(Q6_CHIRHO);
}

#[test]
fn a_unique_re_export_still_links_chirho() {
    assert_exact_chirho(Q7_CHIRHO);
}

#[test]
fn two_definitions_in_unqualified_scope_are_refused_as_ambiguous_chirho() {
    assert_refused_chirho(Q3_CHIRHO);
}

/// Each module's interface, built from its source in order over the builtin interfaces, as the
/// compiler builds them.
fn interfaces_chirho(modules_chirho: &[(&str, &str)]) -> Vec<ModuleIfaceChirho> {
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    for (_, source_chirho) in modules_chirho {
        let green_chirho =
            ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO).parse_chirho();
        let module_chirho = lower_module_chirho(&green_chirho, FileIdChirho::SYNTHETIC_CHIRHO);
        let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);
        ifaces_chirho.push(iface_chirho);
    }
    ifaces_chirho
}

/// The origin `module_chirho`'s interface records for its export `value_chirho`.
fn origin_chirho(
    ifaces_chirho: &[ModuleIfaceChirho],
    module_chirho: &str,
    value_chirho: &str,
) -> Option<String> {
    ifaces_chirho
        .iter()
        .rev()
        .find(|iface_chirho| iface_chirho.name_chirho == module_chirho)
        .unwrap_or_else(|| panic!("{module_chirho} has an interface"))
        .exports_chirho
        .values_chirho
        .get(value_chirho)
        .unwrap_or_else(|| panic!("{module_chirho} exports `{value_chirho}`"))
        .defining_module_chirho
        .clone()
}

/// B defines a function, a record constructor and its field, a constructor and a class method.
const B_DEFINES_CHIRHO: &str = "module B (f, T(..), Shape(..), area, Pretty(..)) where
f :: Int -> Int
f = (+ 1)
data T = MkT { field :: Int }
data Shape = Circle Int
area :: Shape -> Int
area (Circle r) = r
class Pretty a where
  pretty :: a -> String
";

#[test]
fn a_modules_own_definitions_carry_their_definer_chirho() {
    let ifaces_chirho = interfaces_chirho(&[("B", B_DEFINES_CHIRHO)]);
    for value_chirho in ["f", "MkT", "field", "Circle", "area", "pretty"] {
        assert_eq!(
            origin_chirho(&ifaces_chirho, "B", value_chirho).as_deref(),
            Some("B"),
            "{value_chirho}"
        );
    }
}

#[test]
fn every_re_export_keeps_the_definers_origin_chirho() {
    for (form_chirho, forwarder_chirho, values_chirho) in [
        (
            "explicit",
            "module A (f) where
import B (f)
",
            &["f"][..],
        ),
        (
            "module",
            "module A (module B) where
import B
",
            &["f", "MkT", "field", "Circle", "area", "pretty"][..],
        ),
        (
            "member",
            "module A (T(..), Shape(..)) where
import B (T(..), Shape(..))
",
            &["MkT", "field", "Circle"][..],
        ),
    ] {
        let ifaces_chirho = interfaces_chirho(&[("B", B_DEFINES_CHIRHO), ("A", forwarder_chirho)]);
        for value_chirho in values_chirho {
            assert_eq!(
                origin_chirho(&ifaces_chirho, "A", value_chirho).as_deref(),
                Some("B"),
                "{form_chirho}: {value_chirho}"
            );
        }
    }
}

#[test]
fn a_chain_keeps_the_first_definer_chirho() {
    let ifaces_chirho = interfaces_chirho(&[
        (
            "C",
            "module C (g) where
g :: Int
g = 1
",
        ),
        (
            "B",
            "module B (g) where
import C (g)
",
        ),
        (
            "A",
            "module A (g) where
import B (g)
",
        ),
    ]);
    assert_eq!(
        origin_chirho(&ifaces_chirho, "B", "g").as_deref(),
        Some("C")
    );
    assert_eq!(
        origin_chirho(&ifaces_chirho, "A", "g").as_deref(),
        Some("C")
    );
}

#[test]
fn builtin_values_and_forwarded_builtins_have_no_origin_chirho() {
    let with_origin_chirho: Vec<String> = haskelujah_naming_chirho::builtin_module_ifaces_chirho()
        .iter()
        .flat_map(|iface_chirho| {
            iface_chirho
                .exports_chirho
                .values_chirho
                .values()
                .filter(|value_chirho| value_chirho.defining_module_chirho.is_some())
                .map(|value_chirho| {
                    format!("{}.{}", iface_chirho.name_chirho, value_chirho.name_chirho)
                })
        })
        .collect();
    assert!(with_origin_chirho.is_empty(), "{with_origin_chirho:?}");
    // A forwarded builtin keeps no origin; a source module's namesake keeps its own.
    let ifaces_chirho = interfaces_chirho(&[
        (
            "A",
            "module A (map) where
import Prelude (map)
",
        ),
        (
            "B",
            "module B (map) where
import Prelude hiding (map)
map :: Int -> Int
map = (+ 1000)
",
        ),
    ]);
    assert_eq!(origin_chirho(&ifaces_chirho, "A", "map"), None);
    assert_eq!(
        origin_chirho(&ifaces_chirho, "B", "map").as_deref(),
        Some("B")
    );
}
