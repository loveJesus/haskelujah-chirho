// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 629: a reference to another module's export names that module's binding; row 642 sees
//! through re-exports to the module that defines it. Each program and its expected output are
//! GHC 9.14.1's, measured on the text embedded here. The interpreter, LLVM and true Cranelift each
//! run it, with no fallback. An ambiguous reference is refused by name instead, on every engine.
//! workflow: compiler-pipeline-chirho/native-symbol-identity-chirho

use std::time::Duration;

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_project_round_trip_chirho,
};

/// A run's budget for a multi-module program; row 628's controls name the same.
const PROJECT_DEADLINE_CHIRHO: Duration = Duration::from_secs(30);

const BACKENDS_CHIRHO: [NativeBackendChirho; 2] = [
    NativeBackendChirho::LlvmChirho,
    NativeBackendChirho::CraneliftChirho,
];

pub(super) type ProgramChirho = &'static [(&'static str, &'static str)];

/// n1 (row 621's p11, byte-identical): A's own `myFun` and B's re-exported `other`. GHC prints '700'.
const N1_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (myFun, other) where
myFun :: Int -> Int
myFun = (+ 1)
other :: Int
other = 7
"#,
        ),
        (
            "A",
            r#"module A (myFun, other) where
import B hiding (myFun)
myFun :: Int -> Int
myFun = (* 100)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (myFun other)
"#,
        ),
    ],
    r#"700
"#,
);

/// n5: `A.limit` beside `B.limit`, qualified. GHC prints '(5,7,57)'.
const N5_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (limit) where
limit :: Int
limit = 5
"#,
        ),
        (
            "B",
            r#"module B (limit) where
limit :: Int
limit = 7
"#,
        ),
        (
            "Main",
            r#"module Main where
import qualified A
import qualified B
main :: IO ()
main = print (A.limit, B.limit, A.limit * 10 + B.limit)
"#,
        ),
    ],
    r#"(5,7,57)
"#,
);

/// n6: `A.step` beside `B.step`, qualified and first-class. GHC prints '(11,9,[2,3],[0,1])'.
const N6_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (step) where
step :: Int -> Int
step = (+ 1)
"#,
        ),
        (
            "B",
            r#"module B (step) where
step :: Int -> Int
step = subtract 1
"#,
        ),
        (
            "Main",
            r#"module Main where
import qualified A
import qualified B
main :: IO ()
main = print (A.step 10, B.step 10, map A.step [1, 2], map B.step [1, 2])
"#,
        ),
    ],
    r#"(11,9,[2,3],[0,1])
"#,
);

/// a1: the same through aliases `P` and `Q`. GHC prints '(5,7,20,30)'.
const A1_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (limit, scale) where
limit :: Int
limit = 5
scale :: Int -> Int
scale = (* 2)
"#,
        ),
        (
            "B",
            r#"module B (limit, scale) where
limit :: Int
limit = 7
scale :: Int -> Int
scale = (* 3)
"#,
        ),
        (
            "Main",
            r#"module Main where
import qualified A as P
import qualified B as Q
main :: IO ()
main = print (P.limit, Q.limit, P.scale 10, Q.scale 10)
"#,
        ),
    ],
    r#"(5,7,20,30)
"#,
);

/// x1: a dependency's own `show`, imported unqualified. GHC prints '<7>'.
const X1_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (show) where
import Prelude hiding (show)
import qualified Prelude as P
show :: Int -> String
show n = "<" ++ P.show n ++ ">"
"#,
        ),
        (
            "Main",
            r#"module Main where
import Prelude hiding (show)
import A (show)
main :: IO ()
main = putStrLn (show 7)
"#,
        ),
    ],
    r#"<7>
"#,
);

/// x2: a dependency's own `print`. GHC prints 'custom 5'.
const X2_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (print) where
import Prelude hiding (print)
import qualified Prelude as P
print :: Int -> IO ()
print n = P.putStrLn ("custom " ++ P.show n)
"#,
        ),
        (
            "Main",
            r#"module Main where
import Prelude hiding (print)
import A (print)
main :: IO ()
main = print 5
"#,
        ),
    ],
    r#"custom 5
"#,
);

/// x4: a dependency's own `compare`. GHC prints '34'.
const X4_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (compare) where
import Prelude hiding (compare)
compare :: Int -> Int -> Int
compare a b = a * 10 + b
"#,
        ),
        (
            "Main",
            r#"module Main where
import Prelude hiding (compare)
import A (compare)
main :: IO ()
main = print (compare 3 4)
"#,
        ),
    ],
    r#"34
"#,
);

/// r2: a re-export with one definition in the program. GHC prints '21'.
const R2_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (g) where
g :: Int -> Int
g = (* 7)
"#,
        ),
        (
            "A",
            r#"module A (g, h) where
import B (g)
h :: Int
h = 3
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (g h)
"#,
        ),
    ],
    r#"21
"#,
);

/// d1: identical-text bodies over each module's own `helper`. GHC prints '(11,20)'.
const D1_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (run) where
helper :: Int -> Int
helper x = x + 1
run :: Int
run = helper 10
"#,
        ),
        (
            "B",
            r#"module B (run) where
helper :: Int -> Int
helper x = x * 2
run :: Int
run = helper 10
"#,
        ),
        (
            "Main",
            r#"module Main where
import qualified A
import qualified B
main :: IO ()
main = print (A.run, B.run)
"#,
        ),
    ],
    r#"(11,20)
"#,
);

/// r1: A re-exports B's `f` beside C's own; row 642 makes it exact.
const R1_CHIRHO: (ProgramChirho, &str) = (
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
import qualified C
main :: IO ()
main = print (f 5, C.f 5)
"#,
        ),
    ],
    r#"(6,50)
"#,
);

/// z1: two unqualified imports each define `f` (GHC-87543 rejects it).
const Z1_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "A",
            r#"module A (f) where
f :: Int -> Int
f = (+ 1)
"#,
        ),
        (
            "B",
            r#"module B (f) where
f :: Int -> Int
f = (* 10)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
import B
main :: IO ()
main = print (f 5)
"#,
        ),
    ],
    r#"cannot link `f`: it is ambiguous, defined by A, B (row 642)"#,
);

/// The interpreter's run of a program given as modules in dependency order.
fn interpret_chirho(modules_chirho: ProgramChirho) -> Result<String, String> {
    let sources_chirho: Vec<(String, &str)> = modules_chirho
        .iter()
        .map(|(module_chirho, source_chirho)| (format!("{module_chirho}.hs"), *source_chirho))
        .collect();
    let borrowed_chirho: Vec<(&str, &str)> = sources_chirho
        .iter()
        .map(|(file_chirho, source_chirho)| (file_chirho.as_str(), *source_chirho))
        .collect();
    haskelujah_driver::eval_modules_with_machine_chirho(
        &borrowed_chirho,
        &mut SourceMapChirho::new_chirho(),
        None,
    )
    .map(|(_, machine_chirho)| machine_chirho.io_output_chirho)
}

pub(super) fn assert_exact_chirho((modules_chirho, expected_chirho): (ProgramChirho, &str)) {
    assert_eq!(
        interpret_chirho(modules_chirho).as_deref(),
        Ok(expected_chirho),
        "interpreter"
    );
    for backend_chirho in BACKENDS_CHIRHO {
        assert_eq!(
            native_project_round_trip_chirho(
                modules_chirho,
                backend_chirho,
                "",
                PROJECT_DEADLINE_CHIRHO
            )
            .unwrap_or_else(|error_chirho| panic!("{backend_chirho:?}: {error_chirho}")),
            (0, expected_chirho.to_string()),
            "{backend_chirho:?}"
        );
    }
}

pub(super) fn assert_refused_chirho(program_chirho: (ProgramChirho, &str)) {
    let (modules_chirho, refusal_chirho) = program_chirho;
    let interpreted_chirho = interpret_chirho(modules_chirho);
    assert!(
        interpreted_chirho
            .as_ref()
            .is_err_and(|error_chirho| error_chirho.contains(refusal_chirho)),
        "interpreter: {interpreted_chirho:?}"
    );
    assert_refused_natively_chirho(program_chirho);
}

/// LLVM and Cranelift each refuse the program, naming `refusal_chirho`.
fn assert_refused_natively_chirho((modules_chirho, refusal_chirho): (ProgramChirho, &str)) {
    for backend_chirho in BACKENDS_CHIRHO {
        let outcome_chirho = native_project_round_trip_chirho(
            modules_chirho,
            backend_chirho,
            "",
            PROJECT_DEADLINE_CHIRHO,
        );
        assert!(
            outcome_chirho
                .as_ref()
                .is_err_and(|error_chirho| error_chirho.contains(refusal_chirho)),
            "{backend_chirho:?}: {outcome_chirho:?}"
        );
    }
}

/// The identities desugaring records in `module_chirho`'s Core for its references to other
/// modules' exports, as (name, defining modules), sorted.
pub(super) fn imported_identities_chirho(
    modules_chirho: ProgramChirho,
    module_chirho: &str,
) -> Vec<(String, Vec<String>)> {
    let mut identities_chirho: Vec<_> = compiled_core_chirho(modules_chirho, module_chirho)
        .provenance_chirho
        .imported_references_chirho
        .into_values()
        .map(|reference_chirho| {
            (
                reference_chirho.name_chirho,
                reference_chirho.defining_modules_chirho,
            )
        })
        .collect();
    identities_chirho.sort();
    identities_chirho
}

/// `module_chirho`'s Core, compiled with the rest of the program.
pub(super) fn compiled_core_chirho(
    modules_chirho: ProgramChirho,
    module_chirho: &str,
) -> haskelujah_core_chirho::CoreModuleChirho {
    let sources_chirho: Vec<(String, &str)> = modules_chirho
        .iter()
        .map(|(module_chirho, source_chirho)| (format!("{module_chirho}.hs"), *source_chirho))
        .collect();
    let borrowed_chirho: Vec<(&str, &str)> = sources_chirho
        .iter()
        .map(|(file_chirho, source_chirho)| (file_chirho.as_str(), *source_chirho))
        .collect();
    haskelujah_driver::compile_modules_chirho(&borrowed_chirho, &mut SourceMapChirho::new_chirho())
        .expect("the program compiles")
        .into_iter()
        .find(|result_chirho| {
            result_chirho.module_chirho.name_chirho.full_name_chirho() == module_chirho
        })
        .expect("the module is compiled")
        .core_chirho
}

pub(super) fn identities_chirho(entries_chirho: &[(&str, &[&str])]) -> Vec<(String, Vec<String>)> {
    entries_chirho
        .iter()
        .map(|(name_chirho, definers_chirho)| {
            (
                name_chirho.to_string(),
                definers_chirho
                    .iter()
                    .map(|definer_chirho| definer_chirho.to_string())
                    .collect(),
            )
        })
        .collect()
}

#[test]
fn desugaring_gives_each_supplied_name_its_own_identity_chirho() {
    // A qualified reference names the modules behind its qualifier, an alias's included.
    assert_eq!(
        imported_identities_chirho(N5_CHIRHO.0, "Main"),
        identities_chirho(&[("limit", &["A"]), ("limit", &["B"])])
    );
    assert_eq!(
        imported_identities_chirho(A1_CHIRHO.0, "Main"),
        identities_chirho(&[
            ("limit", &["A"]),
            ("limit", &["B"]),
            ("scale", &["A"]),
            ("scale", &["B"]),
        ])
    );
    // One unqualified name two imports supply is one reference that names both.
    assert_eq!(
        imported_identities_chirho(Z1_CHIRHO.0, "Main"),
        identities_chirho(&[("f", &["A", "B"])])
    );
}

#[test]
fn no_prelude_body_or_method_takes_an_imported_reference_chirho() {
    for (program_chirho, name_chirho) in [(X1_CHIRHO.0, "show"), (X4_CHIRHO.0, "compare")] {
        let main_chirho = compiled_core_chirho(program_chirho, "Main");
        let (&id_chirho, _) = main_chirho
            .provenance_chirho
            .imported_references_chirho
            .iter()
            .find(|(_, reference_chirho)| reference_chirho.name_chirho == name_chirho)
            .unwrap_or_else(|| panic!("Main records its imported `{name_chirho}`"));
        assert!(
            !main_chirho
                .bindings_chirho
                .iter()
                .any(|binding_chirho| binding_chirho.binder_chirho.id_chirho == id_chirho),
            "a generated body took the imported `{name_chirho}`'s id"
        );
        assert!(
            haskelujah_core_chirho::module_free_reference_ids_chirho(&main_chirho)
                .contains(&id_chirho),
            "Main no longer refers to A's `{name_chirho}`"
        );
    }
}

#[test]
fn direct_qualified_and_aliased_imports_name_their_modules_chirho() {
    assert_exact_chirho(N5_CHIRHO);
    assert_exact_chirho(N6_CHIRHO);
    assert_exact_chirho(A1_CHIRHO);
}

#[test]
fn an_unqualified_import_names_its_supplier_not_a_namesake_chirho() {
    assert_exact_chirho(N1_CHIRHO);
}

#[test]
fn an_imported_user_function_never_takes_a_prelude_body_chirho() {
    assert_exact_chirho(X1_CHIRHO);
    assert_exact_chirho(X2_CHIRHO);
    assert_exact_chirho(X4_CHIRHO);
}

#[test]
fn a_re_export_with_one_definition_links_to_it_chirho() {
    assert_exact_chirho(R2_CHIRHO);
}

#[test]
fn same_text_bodies_over_different_helpers_stay_apart_chirho() {
    assert_exact_chirho(D1_CHIRHO);
}

#[test]
fn a_re_export_names_its_definer_beside_a_namesake_chirho() {
    assert_exact_chirho(R1_CHIRHO);
}

#[test]
fn an_ambiguous_unqualified_import_gets_no_link_chirho() {
    assert_refused_chirho(Z1_CHIRHO);
}
