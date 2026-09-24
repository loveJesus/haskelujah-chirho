#!/bin/bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# Controls for the one verdict rule, run before any measurement that uses it. The first
# eight are gpt_chirho's audit cases verbatim (room #24614), including the two that the
# first version of this rule failed: the current Rust header with a numeric thread id,
# and a header QUOTED inside a diagnostic excerpt.
# usage: classify_tests_chirho.sh
here_chirho="$(cd "$(dirname "$0")" && pwd)"
. "$here_chirho/classify_chirho.sh"
pass_chirho=0; fail_chirho=0
case_chirho() {
  local name_chirho="$1" out_chirho="$2" rc_chirho="$3" want_chirho="$4"
  local got_chirho; got_chirho="$(verdict_chirho "$out_chirho" "$rc_chirho")"
  if [ "$got_chirho" = "$want_chirho" ]; then pass_chirho=$((pass_chirho+1)); printf 'ok    %-28s %s\n' "$name_chirho" "$want_chirho"
  else fail_chirho=$((fail_chirho+1)); printf 'FAIL  %-28s want %s got %s\n' "$name_chirho" "$want_chirho" "$got_chirho"; fi
}
case_chirho diagnostic_chirho "error[E0300]: mismatch
" 1 REJECT
case_chirho legacy_panic_header_chirho "thread 'main' panicked at file.rs:1:1:
boom
error[E0001]: caught
" 1 PANIC
case_chirho current_panic_header_chirho "thread 'main' (46403379) panicked at file.rs:1:1:
boom
error[E0001]: caught
" 1 PANIC
case_chirho quoted_header_chirho "error[E0300]: mismatch
  3 | labelChirho = \"thread 'main' panicked at fake\"
" 1 REJECT
case_chirho abnormal_panic_chirho "thread 'main' (4) panicked at file.rs:1:1:
" 101 ABNORMAL
case_chirho timeout_first_chirho "error[E0300]: partial output
" 124 TIMEOUT
case_chirho diagnostic_zero_chirho "error[E0300]: mismatch
" 0 UNCLASSIFIED
case_chirho success_chirho "" 0 ACCEPT
case_chirho bare_panic_word_chirho "mainChirho = panic \"x\"
" 0 ACCEPT
case_chirho panic_word_in_diagnostic_chirho "error[E0001]: bad: panic in source
" 1 REJECT
case_chirho non_numeric_id_header_chirho "thread 'main' (worker) panicked at file.rs:1:1:
" 1 UNCLASSIFIED
case_chirho signal_kill_chirho "" 137 ABNORMAL
case_chirho no_markers_exit_one_chirho "some text
" 1 UNCLASSIFIED
echo "$pass_chirho/$((pass_chirho+fail_chirho)) verdict controls pass"
[ $fail_chirho -eq 0 ]
