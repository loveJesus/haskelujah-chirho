// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 628: native symbol identity. Each program's expected output is GHC 9.14.1's, measured on
//! the program text embedded here. LLVM and Cranelift each run alone, with no fallback.
//! - Namesakes in different modules stay apart (N2, N4, N8; N3 is the ordinary control).
//! - Only `Main.main` is the C entry, though a dependency defines or exports a `main` (E1-E3).
//! - A program's own namesake of a builtin keeps its own body (B1-B8, U1), even a libc name the
//!   backend imports (L5), and spellings the old mangler merged stay apart (M1, M2).
//! - A dependency's builtin reference never binds to another module's own namesake (K4).
//!
//! The interpreter row is checked wherever it matches GHC; it has no multi-module path.
//! workflow: compiler-pipeline-chirho/native-symbol-identity-chirho

use std::time::Duration;

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_project_round_trip_chirho, native_round_trip_chirho,
};

/// A run's budget for a multi-module program. One run of the parallel suite exceeded 15 s while
/// another agent's tests loaded the machine, though the same binaries finish in under 0.1 s.
const PROJECT_DEADLINE_CHIRHO: Duration = Duration::from_secs(30);

const BACKENDS_CHIRHO: [NativeBackendChirho; 2] = [
    NativeBackendChirho::LlvmChirho,
    NativeBackendChirho::CraneliftChirho,
];

/// `n2_both_namesakes_reachable`: GHC prints '708'.
const N2_CHIRHO: (&[(&str, &str)], &str) = (
    &[
        (
            "A",
            r#"module A (useBoth) where
import qualified B
myFun :: Int -> Int
myFun = (* 100)
useBoth :: Int -> Int
useBoth x = myFun x + B.myFun x
"#,
        ),
        (
            "B",
            r#"module B (myFun) where
myFun :: Int -> Int
myFun = (+ 1)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (useBoth 7)
"#,
        ),
    ],
    r#"708
"#,
);

/// `n3_ordinary_cross_module_calls`: GHC prints '(7,15)'.
const N3_CHIRHO: (&[(&str, &str)], &str) = (
    &[
        (
            "A",
            r#"module A (f) where
import B (g)
f :: Int -> Int
f x = g x + 1
"#,
        ),
        (
            "B",
            r#"module B (g) where
g :: Int -> Int
g = (* 3)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A (f)
import B (g)
main :: IO ()
main = print (f 2, g 5)
"#,
        ),
    ],
    r#"(7,15)
"#,
);

/// `n4_private_helpers_same_name`: GHC prints '(10,1005)'.
const N4_CHIRHO: (&[(&str, &str)], &str) = (
    &[
        (
            "A",
            r#"module A (useA) where
helper :: Int -> Int
helper = (* 2)
useA :: Int -> Int
useA = helper
"#,
        ),
        (
            "B",
            r#"module B (useB) where
helper :: Int -> Int
helper = (+ 1000)
useB :: Int -> Int
useB = helper
"#,
        ),
        (
            "Main",
            r#"module Main where
import A (useA)
import B (useB)
main :: IO ()
main = print (useA 5, useB 5)
"#,
        ),
    ],
    r#"(10,1005)
"#,
);

/// `n8_main_local_shadows_import`: GHC prints '(6,51)'.
const N8_CHIRHO: (&[(&str, &str)], &str) = (
    &[
        (
            "A",
            r#"module A (twice, applyTwice) where
import qualified B
twice :: Int -> Int
twice = (* 3)
applyTwice :: Int -> Int
applyTwice x = B.twice (twice x)
"#,
        ),
        (
            "B",
            r#"module B (twice) where
twice :: Int -> Int
twice = (* 2)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A (applyTwice)
twice :: Int -> Int
twice = (+ 50)
main :: IO ()
main = print (applyTwice 1, twice 1)
"#,
        ),
    ],
    r#"(6,51)
"#,
);

/// `e1_dependency_exports_main`: GHC prints '42'.
const E1_CHIRHO: (&[(&str, &str)], &str) = (
    &[
        (
            "A",
            r#"module A (main, helper) where
main :: IO ()
main = putStrLn "A.main"
helper :: Int
helper = 41
"#,
        ),
        (
            "Main",
            r#"module Main where
import qualified A
main :: IO ()
main = print (A.helper + 1)
"#,
        ),
    ],
    r#"42
"#,
);

/// `e2_dependency_main_called`: GHC prints 'A.main\nMain.main'.
const E2_CHIRHO: (&[(&str, &str)], &str) = (
    &[
        (
            "A",
            r#"module A (main) where
main :: IO ()
main = putStrLn "A.main"
"#,
        ),
        (
            "Main",
            r#"module Main where
import qualified A
main :: IO ()
main = A.main >> putStrLn "Main.main"
"#,
        ),
    ],
    r#"A.main
Main.main
"#,
);

/// `e3_dependency_private_main`: GHC prints '7'.
const E3_CHIRHO: (&[(&str, &str)], &str) = (
    &[
        (
            "A",
            r#"module A (helper) where
main :: IO ()
main = putStrLn "A private main"
helper :: Int
helper = 7
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print helper
"#,
        ),
    ],
    r#"7
"#,
);

/// `k4_own_comparing_beside_builtin_comparing`: GHC prints '([(2,1),(1,3)],2000)'. Native `comparing` is not implemented yet, so
/// the builtin reference fails named; on main it bound to B's own `comparing` and exited 1 silently.
const K4_CHIRHO: (&[(&str, &str)], &str) = (
    &[
        (
            "A",
            r#"module A (sortedBy) where
import Data.List (sortBy)
import Data.Ord (comparing)
sortedBy :: [(Int, Int)] -> [(Int, Int)]
sortedBy = sortBy (comparing snd)
"#,
        ),
        (
            "B",
            r#"module B (comparing) where
comparing :: Int -> Int
comparing = (* 1000)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
import qualified B
main :: IO ()
main = print (sortedBy [(1,3),(2,1)], B.comparing 2)
"#,
        ),
    ],
    r#"([(2,1),(1,3)],2000)
"#,
);

/// `b1_own_print`: GHC prints 'custom 5'.
const B1_CHIRHO: (&str, &str) = (
    r#"module Main where
import Prelude hiding (print)
import qualified Prelude as P
print :: Int -> IO ()
print n = putStrLn ("custom " ++ P.show n)
main :: IO ()
main = print 5
"#,
    r#"custom 5
"#,
);

/// `b2_own_show`: GHC prints '<7>'.
const B2_CHIRHO: (&str, &str) = (
    r#"module Main where
import Prelude hiding (show)
import qualified Prelude as P
show :: Int -> String
show n = "<" ++ P.show n ++ ">"
main :: IO ()
main = putStrLn (show 7)
"#,
    r#"<7>
"#,
);

/// `b4_own_compare`: GHC prints '34'.
const B4_CHIRHO: (&str, &str) = (
    r#"module Main where
import Prelude hiding (compare)
compare :: Int -> Int -> Int
compare a b = a * 10 + b
main :: IO ()
main = print (compare 3 4)
"#,
    r#"34
"#,
);

/// `b7_own_undefined`: GHC prints '43'.
const B7_CHIRHO: (&str, &str) = (
    r#"module Main where
import Prelude hiding (undefined)
undefined :: Int
undefined = 42
main :: IO ()
main = print (undefined + 1)
"#,
    r#"43
"#,
);

/// `b8_own_put_str_ln`: GHC prints 'hi!'.
const B8_CHIRHO: (&str, &str) = (
    r#"module Main where
import Prelude hiding (putStrLn)
import qualified Prelude as P
putStrLn :: String -> IO ()
putStrLn s = P.putStrLn (s ++ "!")
main :: IO ()
main = putStrLn "hi"
"#,
    r#"hi!
"#,
);

/// `u1_own_get_line`: GHC prints 'mine'.
const U1_CHIRHO: (&str, &str) = (
    r#"module Main where
import Prelude hiding (getLine)
getLine :: IO String
getLine = return "mine"
main :: IO ()
main = getLine >>= putStrLn
"#,
    r#"mine
"#,
);

/// `l5_own_strcmp_with_string_compare`: GHC prints '304\n(LT,GT)'.
const L5_CHIRHO: (&str, &str) = (
    r#"module Main where
strcmp :: Int -> Int -> Int
strcmp a b = a * 100 + b
main :: IO ()
main = print (strcmp 3 4) >> print (compare "ab" "ac", compare "b" "a")
"#,
    r#"304
(LT,GT)
"#,
);

/// `m1_prime_and_underscore_prime`: GHC prints '(2,20)'.
const M1_CHIRHO: (&str, &str) = (
    r#"module Main where
f' :: Int -> Int
f' = (+ 1)
f_prime :: Int -> Int
f_prime = (* 10)
main :: IO ()
main = print (f' 1, f_prime 2)
"#,
    r#"(2,20)
"#,
);

/// `m2_operator_and_escape_spelling`: GHC prints '(6,5)'.
const M2_CHIRHO: (&str, &str) = (
    r#"module Main where
(+++) :: Int -> Int -> Int
a +++ b = a * b
_u002b_u002b_u002b :: Int
_u002b_u002b_u002b = 5
main :: IO ()
main = print (2 +++ 3, _u002b_u002b_u002b)
"#,
    r#"(6,5)
"#,
);

fn assert_project_chirho((modules_chirho, expected_chirho): (&[(&str, &str)], &str)) {
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

/// GHC's output, or the named failure of a builtin with no native body yet: never a binding
/// that only shares the builtin's spelling.
fn assert_project_exact_or_named_chirho(
    (modules_chirho, expected_chirho): (&[(&str, &str)], &str),
    missing_chirho: &str,
) {
    for backend_chirho in BACKENDS_CHIRHO {
        match native_project_round_trip_chirho(
            modules_chirho,
            backend_chirho,
            "",
            PROJECT_DEADLINE_CHIRHO,
        ) {
            Ok(outcome_chirho) => assert_eq!(
                outcome_chirho,
                (0, expected_chirho.to_string()),
                "{backend_chirho:?}"
            ),
            Err(error_chirho) => assert!(
                error_chirho.contains(&format!("missing native binding `{missing_chirho}`")),
                "{backend_chirho:?}: {error_chirho}"
            ),
        }
    }
}

fn assert_native_chirho((source_chirho, expected_chirho): (&str, &str)) {
    for backend_chirho in BACKENDS_CHIRHO {
        assert_eq!(
            native_round_trip_chirho(source_chirho, backend_chirho, "")
                .unwrap_or_else(|error_chirho| panic!("{backend_chirho:?}: {error_chirho}")),
            (0, expected_chirho.to_string()),
            "{backend_chirho:?}"
        );
    }
}

fn assert_all_engines_chirho(program_chirho: (&str, &str)) {
    let (source_chirho, expected_chirho) = program_chirho;
    let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "Main.hs",
        None,
    )
    .expect("the interpreter runs the program");
    assert_eq!(
        machine_chirho.io_output_chirho, expected_chirho,
        "interpreter"
    );
    assert_native_chirho(program_chirho);
}

#[test]
fn namesakes_in_different_modules_stay_apart_chirho() {
    assert_project_chirho(N2_CHIRHO);
    assert_project_chirho(N4_CHIRHO);
    assert_project_chirho(N8_CHIRHO);
    assert_project_chirho(N3_CHIRHO);
}

#[test]
fn only_main_main_is_the_native_entry_chirho() {
    assert_project_chirho(E1_CHIRHO);
    assert_project_chirho(E2_CHIRHO);
    assert_project_chirho(E3_CHIRHO);
}

#[test]
fn a_builtin_reference_never_binds_to_a_programs_namesake_chirho() {
    assert_project_exact_or_named_chirho(K4_CHIRHO, "comparing");
}

#[test]
fn a_programs_own_print_keeps_its_body_chirho() {
    assert_all_engines_chirho(B1_CHIRHO);
}

#[test]
fn a_programs_own_show_keeps_its_body_chirho() {
    assert_all_engines_chirho(B2_CHIRHO);
}

#[test]
fn a_programs_own_compare_keeps_its_body_chirho() {
    assert_all_engines_chirho(B4_CHIRHO);
}

#[test]
fn a_programs_own_undefined_keeps_its_body_chirho() {
    // The interpreter still fails here ("Prelude.undefined"): not this row's engine.
    assert_native_chirho(B7_CHIRHO);
}

#[test]
fn a_programs_own_put_str_ln_keeps_its_body_chirho() {
    assert_all_engines_chirho(B8_CHIRHO);
}

#[test]
fn a_programs_own_get_line_keeps_its_body_chirho() {
    assert_all_engines_chirho(U1_CHIRHO);
}

#[test]
fn a_programs_own_strcmp_stays_apart_from_libc_chirho() {
    assert_all_engines_chirho(L5_CHIRHO);
}

#[test]
fn a_prime_and_its_old_mangled_spelling_stay_apart_chirho() {
    assert_all_engines_chirho(M1_CHIRHO);
}

#[test]
fn an_operator_and_its_escape_spelling_stay_apart_chirho() {
    assert_all_engines_chirho(M2_CHIRHO);
}
