// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! seq, deepseq, force, evaluate, $!! and halting on error or undefined (row 700).

use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn eval_error_halts_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = error \"kaboom\"\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    assert!(result_chirho.is_err(), "error should halt evaluation");
    let msg_chirho = result_chirho.unwrap_err();
    assert!(
        msg_chirho.contains("kaboom"),
        "error message should propagate: {}",
        msg_chirho
    );
}

#[test]
fn eval_undefined_halts_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = undefined\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    assert!(result_chirho.is_err(), "undefined should halt evaluation");
    let msg_chirho = result_chirho.unwrap_err();
    assert!(
        msg_chirho.contains("undefined") || msg_chirho.contains("Prelude.undefined"),
        "undefined error message: {}",
        msg_chirho
    );
}

#[test]
fn eval_seq_forces_first_returns_second_chirho() {
    use crate::eval_source_with_machine_chirho;
    // seq forces its first argument and returns the second
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
        "module Test where\nmain = seq 1 42\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("seq should evaluate");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
    );
}

#[test]
fn eval_seq_strict_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // seq forces strict evaluation of first arg, returns second
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = seq 1 42\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => panic!("seq should work: {}", e_chirho),
    }
}

// ── deepseq / force / evaluate tests ─────────────────────────────────

#[test]
fn eval_deepseq_returns_second_chirho() {
    // deepseq x y = x `seq` y → deepseq (1 + 2) 42 should evaluate to 42
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
        "module Test where\nmain = deepseq (1 + 2) 42\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("deepseq should evaluate");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
    );
}

#[test]
fn eval_force_returns_value_chirho() {
    // force x = x `seq` x → force (2 + 3) should evaluate to 5
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
        "module Test where\nmain = force (2 + 3)\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("force should evaluate");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
    );
}

#[test]
fn eval_evaluate_returns_value_chirho() {
    // evaluate x = return x → evaluate 42 in an IO context
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
        "module Test where\nimport Control.Exception (evaluate)\nmain = evaluate 42\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("evaluate should evaluate");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
    );
}

// ── $!! (deep strict application) tests ──────────────────────────────

#[test]
fn eval_double_bang_dollar_basic_chirho() {
    // f $!! x = deepseq x (f x) → (+1) $!! 41 should evaluate to 42
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
        "module Test where\nmain = (+1) $!! 41\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("$!! should evaluate");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
    );
}

#[test]
fn eval_double_bang_dollar_with_expression_chirho() {
    // f $!! (2 + 3) should deeply evaluate 2+3 then apply f
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
        "module Test where\nf x = x * 2\nmain = f $!! (2 + 3)\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("$!! with expression should evaluate");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
    );
}
