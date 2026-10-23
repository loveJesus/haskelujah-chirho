// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Unit 524: a value whose forcing fails while `print` renders it fails the
//! run with its message, or reaches the enclosing `catch`, instead of ending
//! the program silently with exit 0.
//!
//! Each program is a measured probe, embedded byte for byte by generation,
//! beside GHC 9.14.1's stdout for it. Our renderer builds the whole string
//! before writing it, so where GHC first writes a partial prefix (`[1.5,`) we
//! write nothing: a failing run's stdout must be a prefix of GHC's, and a
//! handled run's stdout a suffix of it.
//! workflow: stg-forcing-core-chirho

use haskelujah_span_chirho::SourceMapChirho;

/// The message every failing probe raises.
const MESSAGE_CHIRHO: &str = "boomChirho";

/// Probes GHC ends with the uncaught failure (exit 1): the probe, its
/// program, GHC's stdout, and the output already written when it fails.
const FAILS_CLOSED_CHIRHO: &[(&str, &str, &str, &str)] = &[
    (
        "C4AfterFloatListErrorChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = do
  putStrLn "before"
  print [1.5 :: Float, error "boomChirho"]
  putStrLn "after"
"#,
        "before\n[1.5,",
        "before\n",
    ),
    (
        "G10FloatTailErrorChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = print (1.5 : error "boomChirho" :: [Float])
"#,
        "[1.5",
        "",
    ),
    (
        "G1FloatListErrorChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = print [1.5 :: Float, error "boomChirho"]
"#,
        "[1.5,",
        "",
    ),
    (
        "G3FloatListFirstErrorChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = print [error "boomChirho" :: Float]
"#,
        "[",
        "",
    ),
    (
        "G5NestedFloatErrorChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = print [[1.5 :: Float], [error "boomChirho"]]
"#,
        "[[1.5],[",
        "",
    ),
    (
        "G6MaybeFloatErrorChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = print (Just (error "boomChirho" :: Float))
"#,
        "Just ",
        "",
    ),
    (
        "G7TupleFloatErrorChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = print (1.5 :: Float, error "boomChirho" :: Float)
"#,
        "(1.5,",
        "",
    ),
    (
        "L1ListMiddleErrorChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = print [1, error "boomChirho", 3 :: Int]
"#,
        "[1,",
        "",
    ),
    (
        "S1ScalarErrorChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = print (error "boomChirho" :: Int)
"#,
        "",
        "",
    ),
];

/// Probes whose handler catches the failure (exit 0): the probe, its
/// program, GHC's stdout, and the handler's own output.
const HANDLER_RUNS_CHIRHO: &[(&str, &str, &str, &str)] = &[
    (
        "C8CatchIntListPlainChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Control.Exception

main :: IO ()
main = catch (print [1 :: Int, error "boomChirho"]) handlerChirho

handlerChirho :: ErrorCall -> IO ()
handlerChirho _ = putStrLn "caught"
"#,
        "[1,caught\n",
        "caught\n",
    ),
    (
        "C9CatchFloatListPlainChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Control.Exception

main :: IO ()
main = catch (print [1.5 :: Float, error "boomChirho"]) handlerChirho

handlerChirho :: ErrorCall -> IO ()
handlerChirho _ = putStrLn "caught"
"#,
        "[1.5,caught\n",
        "caught\n",
    ),
];

/// Probes that force nothing failing, or catch a failure directly (exit 0):
/// the probe, its program, and GHC's stdout.
const UNCHANGED_CHIRHO: &[(&str, &str, &str)] = &[
    (
        "C5CatchDirectErrorChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Control.Exception

main :: IO ()
main = catch (error "boomChirho") handlerChirho

handlerChirho :: ErrorCall -> IO ()
handlerChirho _ = putStrLn "caught"
"#,
        "caught\n",
    ),
    (
        "C6CatchScalarPrintChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Control.Exception

main :: IO ()
main = catch (print (error "boomChirho" :: Int)) handlerChirho

handlerChirho :: ErrorCall -> IO ()
handlerChirho _ = putStrLn "caught"
"#,
        "caught\n",
    ),
    (
        "C7CatchNoErrorChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Control.Exception

main :: IO ()
main = catch (print (1 :: Int)) handlerChirho

handlerChirho :: ErrorCall -> IO ()
handlerChirho _ = putStrLn "caught"
"#,
        "1\n",
    ),
    (
        "G2FloatListChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = print [2.5 :: Float]
"#,
        "[2.5]\n",
    ),
    (
        "Z1LazyLengthChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = print (length [1, error "boomChirho", 3 :: Int])
"#,
        "3\n",
    ),
    (
        "Z2LazyTakeChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = print (take 1 [1, error "boomChirho" :: Int])
"#,
        "[1]\n",
    ),
    (
        "Z3LazyHeadChirho",
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

main :: IO ()
main = print (head [1, error "boomChirho" :: Int])
"#,
        "1\n",
    ),
];

/// The run's stdout, and its runtime failure if it failed.
fn run_chirho(probe_chirho: &str, program_chirho: &str) -> (String, Result<(), String>) {
    let outcome_chirho = haskelujah_driver::eval_source_outcome_chirho(
        program_chirho,
        &mut SourceMapChirho::new_chirho(),
        &format!("{probe_chirho}.hs"),
        None,
    )
    .unwrap_or_else(|error_chirho| panic!("{probe_chirho} must compile: {error_chirho}"));
    (
        outcome_chirho.machine_chirho.io_output_chirho.clone(),
        outcome_chirho.result_chirho.map(|_| ()),
    )
}

#[test]
fn a_failed_element_or_tail_fails_the_run_with_its_message_chirho() {
    for &(probe_chirho, program_chirho, ghc_stdout_chirho, written_chirho) in FAILS_CLOSED_CHIRHO {
        let (stdout_chirho, result_chirho) = run_chirho(probe_chirho, program_chirho);
        let Err(error_chirho) = result_chirho else {
            panic!("{probe_chirho}: GHC fails, we returned after writing {stdout_chirho:?}");
        };
        assert!(
            error_chirho.contains(MESSAGE_CHIRHO),
            "{probe_chirho}: {error_chirho}"
        );
        assert!(
            ghc_stdout_chirho.starts_with(stdout_chirho.as_str())
                && stdout_chirho.starts_with(written_chirho),
            "{probe_chirho}: we wrote {stdout_chirho:?}; it must keep {written_chirho:?} \
             and be a prefix of GHC's {ghc_stdout_chirho:?}"
        );
    }
}

#[test]
fn a_catch_handler_receives_a_failed_element_chirho() {
    for &(probe_chirho, program_chirho, ghc_stdout_chirho, handler_chirho) in HANDLER_RUNS_CHIRHO {
        let (stdout_chirho, result_chirho) = run_chirho(probe_chirho, program_chirho);
        assert_eq!(result_chirho, Ok(()), "{probe_chirho}: the handler returns");
        assert!(
            ghc_stdout_chirho.ends_with(stdout_chirho.as_str())
                && stdout_chirho.ends_with(handler_chirho),
            "{probe_chirho}: we wrote {stdout_chirho:?}; it must hold the handler's \
             {handler_chirho:?} and be a suffix of GHC's {ghc_stdout_chirho:?}"
        );
    }
}

#[test]
fn lazy_successful_and_directly_caught_runs_match_ghc_chirho() {
    for &(probe_chirho, program_chirho, ghc_stdout_chirho) in UNCHANGED_CHIRHO {
        let (stdout_chirho, result_chirho) = run_chirho(probe_chirho, program_chirho);
        assert_eq!(result_chirho, Ok(()), "{probe_chirho}");
        assert_eq!(stdout_chirho, ghc_stdout_chirho, "{probe_chirho}");
    }
}
