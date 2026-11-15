// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
use super::*;
#[test]
fn preparation_and_completion_preserve_the_spare_buffer_boundary_chirho() {
    let mut state_chirho = OutputStateChirho::with_limit_chirho(10_000);
    let mut output_chirho = String::new();
    for _index_chirho in 0..2047 {
        state_chirho.prepare_chirho(&mut output_chirho).unwrap();
        state_chirho.append_chirho('a').unwrap();
    }
    assert!(output_chirho.is_empty());
    state_chirho.discard_pending_chirho();
    assert!(output_chirho.is_empty());
    for _index_chirho in 0..2047 {
        state_chirho.prepare_chirho(&mut output_chirho).unwrap();
        state_chirho.append_chirho('a').unwrap();
    }
    state_chirho.prepare_chirho(&mut output_chirho).unwrap();
    assert_eq!(output_chirho, "a".repeat(2047));
    state_chirho.discard_pending_chirho();
    for _index_chirho in 0..2047 {
        state_chirho.prepare_chirho(&mut output_chirho).unwrap();
        state_chirho.append_chirho('b').unwrap();
    }
    state_chirho
        .finish_chirho(&mut output_chirho, true)
        .unwrap();
    assert!(output_chirho.ends_with(&("b".repeat(2047) + "\n")));
}
#[test]
fn utf8_is_charged_before_mutation_and_exhaustion_stays_latched_chirho() {
    let mut state_chirho = OutputStateChirho::with_limit_chirho(6);
    state_chirho.append_chirho('λ').unwrap();
    state_chirho.append_chirho('😀').unwrap();
    assert_eq!(
        state_chirho.append_chirho('x'),
        Err(IoErrorChirho::BudgetChirho)
    );
    let mut output_chirho = String::new();
    assert_eq!(
        state_chirho.finish_chirho(&mut output_chirho, false),
        Err(IoErrorChirho::BudgetChirho)
    );
    state_chirho.discard_pending_chirho();
    assert!(state_chirho.exhausted_chirho());
    assert_eq!(
        state_chirho.prepare_chirho(&mut output_chirho),
        Err(IoErrorChirho::BudgetChirho)
    );
    assert!(output_chirho.is_empty());
}
