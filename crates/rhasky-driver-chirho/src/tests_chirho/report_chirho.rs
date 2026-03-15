// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Haskell 2010 Report conformance integration tests.
//!
//! Validates that the conformance tracker reflects the actual
//! capabilities of the Rhasky compiler by spot-checking sections
//! marked DONE against real compilation.

use rhasky_span_chirho::SourceMapChirho;
use rhasky_test_harness_chirho::report_chirho::{
    haskell_2010_conformance_chirho, StatusChirho,
};

use crate::eval_source_with_machine_chirho;

/// Helper: compile and eval, return Ok(i64) or Err(String).
fn eval_chirho(source_chirho: &str) -> Result<i64, String> {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (val_chirho, _machine_chirho) =
        eval_source_with_machine_chirho(source_chirho, &mut sm_chirho, "Report.hs", None)
            .map_err(|e_chirho| format!("{}", e_chirho))?;
    match val_chirho {
        rhasky_runtime_chirho::ValueChirho::IntChirho(n_chirho) => Ok(n_chirho),
        other_chirho => Err(format!("expected Int, got {:?}", other_chirho)),
    }
}

// ── Spot-check sections marked DONE ────────────────────────────────────

#[test]
fn report_sec_2_5_numeric_literals_chirho() {
    // §2.5 Numeric Literals — hex and octal
    let src_chirho = "module Report where\nmain = 0xFF + 0o17\n";
    assert_eq!(eval_chirho(src_chirho).unwrap(), 270);
}

#[test]
fn report_sec_3_6_conditionals_chirho() {
    // §3.6 Conditionals — if-then-else
    let src_chirho = "module Report where\nmain = if 1 > 0 then 42 else 0\n";
    assert_eq!(eval_chirho(src_chirho).unwrap(), 42);
}

#[test]
fn report_sec_3_10_arithmetic_sequences_chirho() {
    // §3.10 Arithmetic Sequences — [1..5]
    let src_chirho = concat!(
        "module Report where\n",
        "sumList [] = 0\n",
        "sumList (x:xs) = x + sumList xs\n",
        "main = sumList [1..5]\n",
    );
    assert_eq!(eval_chirho(src_chirho).unwrap(), 15);
}

#[test]
fn report_sec_3_12_let_expressions_chirho() {
    // §3.12 Let Expressions
    let src_chirho = "module Report where\nmain = let x = 10 in x + 32\n";
    assert_eq!(eval_chirho(src_chirho).unwrap(), 42);
}

#[test]
fn report_sec_3_14_do_expressions_chirho() {
    // §3.14 Do Expressions — bind, return
    let src_chirho = concat!(
        "module Report where\n",
        "main = do\n",
        "  let x = 21\n",
        "  return (x * 2)\n",
    );
    assert_eq!(eval_chirho(src_chirho).unwrap(), 42);
}

#[test]
fn report_sec_4_2_user_defined_datatypes_chirho() {
    // §4.2 User-Defined Datatypes — data, case
    let src_chirho = concat!(
        "module Report where\n",
        "data Color = Red | Green | Blue\n",
        "colorCode Red = 1\n",
        "colorCode Green = 2\n",
        "colorCode Blue = 3\n",
        "main = colorCode Green\n",
    );
    assert_eq!(eval_chirho(src_chirho).unwrap(), 2);
}

#[test]
fn report_sec_6_2_strict_evaluation_chirho() {
    // §6.2 Strict Evaluation — seq
    let src_chirho = "module Report where\nmain = seq 99 42\n";
    assert_eq!(eval_chirho(src_chirho).unwrap(), 42);
}

// ── Tracker metadata tests ─────────────────────────────────────────────

#[test]
fn report_tracker_reflects_reality_chirho() {
    let tracker_chirho = haskell_2010_conformance_chirho();
    let done_chirho = tracker_chirho.count_by_status_chirho(StatusChirho::DoneChirho);
    let total_chirho = tracker_chirho.sections_chirho.len();
    // At least 70% done
    assert!(
        (done_chirho as f64 / total_chirho as f64) > 0.70,
        "expected >70% done, got {}/{}", done_chirho, total_chirho
    );
}

#[test]
fn report_pass_rate_above_80_chirho() {
    let tracker_chirho = haskell_2010_conformance_chirho();
    let rate_chirho = tracker_chirho.pass_rate_chirho();
    assert!(
        rate_chirho > 80.0,
        "pass rate should be above 80%, got {:.1}%", rate_chirho
    );
}

#[test]
fn report_summary_table_renders_chirho() {
    let tracker_chirho = haskell_2010_conformance_chirho();
    let table_chirho = tracker_chirho.summary_table_chirho();
    assert!(table_chirho.contains("Section"));
    assert!(table_chirho.contains("DONE"));
    assert!(table_chirho.contains("Pass rate"));
}
