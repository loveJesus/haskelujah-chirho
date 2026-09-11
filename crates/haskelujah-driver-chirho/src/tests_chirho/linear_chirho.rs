// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! End-to-end tests for the LinearTypes extension.

#[allow(unused_imports)]
use crate::{compile_source_chirho, eval_source_chirho, frontend_warnings_chirho};
#[allow(unused_imports)]
use haskelujah_runtime_chirho::ValueChirho;
#[allow(unused_imports)]
use haskelujah_span_chirho::SourceMapChirho;

// ── Parsing and evaluation ────────────────────────────────────────────────

#[test]
fn linear_arrow_percent1_parses_chirho() {
    // A function with %1 -> annotation should parse and type-check
    let src_chirho = "\
{-# LANGUAGE LinearTypes #-}
module Test where
f :: Int %1 -> Int
f x = x + 1
main = f 41
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "LinearTest.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn linear_arrow_unicode_parses_chirho() {
    // The ⊸ Unicode linear arrow should work identically to %1 ->
    let src_chirho = "\
{-# LANGUAGE LinearTypes #-}
module Test where
g :: Int \u{22B8} Int
g x = x * 2
main = g 21
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "LinearUnicode.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn percent_many_arrow_parses_chirho() {
    // %Many -> is the unrestricted (normal) arrow
    let src_chirho = "\
{-# LANGUAGE DataKinds, LinearTypes #-}
module Test where
import GHC.Types (Multiplicity(Many))
h :: Int %Many -> Int
h x = x + x
main = h 21
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "LinearMany.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── Linearity violation detection ─────────────────────────────────────────

#[test]
fn linear_used_once_no_warning_chirho() {
    // Using a linear parameter exactly once should produce no linearity warning
    let src_chirho = "\
{-# LANGUAGE LinearTypes #-}
module Test where
f :: Int %1 -> Int
f x = x + 1
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let warnings_chirho =
        frontend_warnings_chirho(src_chirho, &mut sm_chirho, "LinearOk.hs").unwrap();
    let linearity_warns_chirho: Vec<_> = warnings_chirho
        .iter()
        .filter(|w_chirho| w_chirho.contains("Linearity violation"))
        .collect();
    assert!(
        linearity_warns_chirho.is_empty(),
        "Expected no linearity warnings, got: {:?}",
        linearity_warns_chirho
    );
}

#[test]
fn linear_used_twice_warns_chirho() {
    // Using a linear parameter more than once should trigger a warning
    let src_chirho = "\
{-# LANGUAGE LinearTypes #-}
module Test where
dup :: Int %1 -> Int
dup x = x + x
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let warnings_chirho =
        frontend_warnings_chirho(src_chirho, &mut sm_chirho, "LinearDup.hs").unwrap();
    let linearity_warns_chirho: Vec<_> = warnings_chirho
        .iter()
        .filter(|w_chirho| w_chirho.contains("Linearity violation"))
        .collect();
    assert!(
        !linearity_warns_chirho.is_empty(),
        "Expected a linearity violation warning for duplicated use"
    );
    assert!(
        linearity_warns_chirho[0].contains("used 2 times"),
        "Warning should mention 2 uses: {:?}",
        linearity_warns_chirho[0]
    );
}

#[test]
fn linear_unused_warns_chirho() {
    // Not using a linear parameter at all should trigger a warning
    let src_chirho = "\
{-# LANGUAGE LinearTypes #-}
module Test where
discard :: Int %1 -> Int
discard x = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let warnings_chirho =
        frontend_warnings_chirho(src_chirho, &mut sm_chirho, "LinearDiscard.hs").unwrap();
    let linearity_warns_chirho: Vec<_> = warnings_chirho
        .iter()
        .filter(|w_chirho| w_chirho.contains("Linearity violation"))
        .collect();
    assert!(
        !linearity_warns_chirho.is_empty(),
        "Expected a linearity violation warning for unused linear variable"
    );
    assert!(
        linearity_warns_chirho[0].contains("unused"),
        "Warning should mention unused: {:?}",
        linearity_warns_chirho[0]
    );
}

#[test]
fn no_extension_no_linearity_check_chirho() {
    // Without the LinearTypes extension, duplicate use should not warn
    let src_chirho = "\
module Test where
dup :: Int -> Int
dup x = x + x
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let warnings_chirho =
        frontend_warnings_chirho(src_chirho, &mut sm_chirho, "NoLinear.hs").unwrap();
    let linearity_warns_chirho: Vec<_> = warnings_chirho
        .iter()
        .filter(|w_chirho| w_chirho.contains("Linearity violation"))
        .collect();
    assert!(
        linearity_warns_chirho.is_empty(),
        "Should not check linearity without LinearTypes extension"
    );
}

#[test]
fn unrestricted_param_no_linearity_check_chirho() {
    // Even with LinearTypes, %Many -> params should not be checked
    let src_chirho = "\
{-# LANGUAGE DataKinds, LinearTypes #-}
module Test where
import GHC.Types (Multiplicity(Many))
h :: Int %Many -> Int
h x = x + x
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let warnings_chirho =
        frontend_warnings_chirho(src_chirho, &mut sm_chirho, "LinearManyOk.hs").unwrap();
    let linearity_warns_chirho: Vec<_> = warnings_chirho
        .iter()
        .filter(|w_chirho| w_chirho.contains("Linearity violation"))
        .collect();
    assert!(
        linearity_warns_chirho.is_empty(),
        "Unrestricted (%Many) params should not produce linearity warnings"
    );
}
