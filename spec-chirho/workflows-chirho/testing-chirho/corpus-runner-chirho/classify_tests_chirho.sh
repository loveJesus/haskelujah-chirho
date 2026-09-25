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
# gpt_chirho room #24703: the earlier `[0-9]*` glob meant one digit then anything, so
# a mixed id passed for numeric. It is not what the runtime prints, so it is no header.
case_chirho mixed_id_header_chirho "thread 'main' (4worker) panicked at file.rs:1:1:
" 1 UNCLASSIFIED
case_chirho all_digit_id_header_chirho "thread 'main' (4) panicked at file.rs:1:1:
" 1 PANIC
# claude2_chirho's finding: a crash whose header is GLUED to output that ended without
# a newline used to be missed, and with a diagnostic also present it was credited as a
# REJECT - a crash counted as a diagnostic, the one direction that must not happen.
case_chirho glued_header_chirho "partial outputthread 'main' panicked at file.rs:1:1:
error[E0001]: caught
" 1 PANIC
case_chirho cr_separated_header_chirho "progress$(printf '\r')thread 'main' panicked at file.rs:1:1:
error[E0001]: caught
" 1 PANIC
# The other direction still holds: a quotation is not a crash. Behind a gutter, inside
# a diagnostic line, or merely indented, a header stays a quotation.
case_chirho glued_into_gutter_chirho "error[E0300]: mismatch
  3 | labelChirho = thread 'main' panicked at fake
" 1 REJECT
case_chirho header_in_error_line_chirho "error[E0001]: bad: thread 'main' panicked at fake
" 1 REJECT
case_chirho indented_header_chirho "error[E0300]: mismatch
     thread 'main' panicked at fake
" 1 REJECT
# gpt_chirho review F1 (#24820): a line can carry MORE THAN ONE `thread '...'`
# fragment, and a scan that keeps only the text since the previous candidate loses
# the gutter, the `error[` body or the indent. Each of these is quoted text and must
# stay a REJECT, in the numeric-id form as well.
case_chirho two_fragments_behind_gutter_chirho "error[E0001]: source rejected
  4 | \"thread 'not a header' blah thread 'main' panicked at fake\"
" 1 REJECT
case_chirho two_fragments_behind_gutter_numeric_chirho "error[E0001]: source rejected
  4 | \"thread 'not a header' blah thread 'main' (46403379) panicked at fake\"
" 1 REJECT
case_chirho two_fragments_in_error_line_chirho "error[E0001]: bad: thread 'x' blah thread 'main' panicked at fake
" 1 REJECT
case_chirho two_fragments_in_error_line_numeric_chirho "error[E0001]: bad: thread 'x' blah thread 'main' (46403379) panicked at fake
" 1 REJECT
case_chirho two_fragments_indented_chirho "error[E0300]: mismatch
     thread 'x' blah thread 'main' panicked at fake
" 1 REJECT
case_chirho two_fragments_indented_numeric_chirho "error[E0300]: mismatch
     thread 'x' blah thread 'main' (46403379) panicked at fake
" 1 REJECT
# And the genuine glued header still counts when a second fragment precedes it in
# real OUTPUT rather than in a quotation.
case_chirho two_fragments_in_output_chirho "thread it said blahthread 'main' panicked at file.rs:1:1:
error[E0001]: caught
" 1 PANIC

# The rule must give the same verdict in whatever shell sources it. Every runner script
# here is bash, but the interactive shell in this environment is zsh, and a rule that
# aborts on a pattern returns an EMPTY string rather than a verdict - fail-unsafe, and
# exactly the regression claude2_chirho caught (#24819). So the shapes most likely to
# break on a glob-operator difference are run through the OTHER shell too.
other_shell_chirho() {
  local shell_chirho="$1"
  command -v "$shell_chirho" > /dev/null 2>&1 || { printf 'skip  %s not present\n' "$shell_chirho"; return 0; }
  local probe_chirho
  for probe_chirho in \
    "thread 'main' (46403379) panicked at f.rs:1:1:|PANIC" \
    "thread 'main' panicked at f.rs:1:1:|PANIC" \
    "thread 'main' (4worker) panicked at f.rs:1:1:|UNCLASSIFIED" \
    "thread 'main' (worker) panicked at f.rs:1:1:|UNCLASSIFIED" \
    "error[E0300]: mismatch|REJECT"
  do
    local text_chirho="${probe_chirho%|*}" want_chirho="${probe_chirho##*|}" got_chirho
    got_chirho="$("$shell_chirho" -c ". '$here_chirho/classify_chirho.sh'; verdict_chirho \"\$1\" 1" _ "$text_chirho" 2>&1)"
    case "$got_chirho" in
      "$want_chirho") pass_chirho=$((pass_chirho+1)); printf 'ok    %-6s %-46s %s\n' "$shell_chirho" "${text_chirho%%:*}" "$want_chirho";;
      *) fail_chirho=$((fail_chirho+1)); printf 'FAIL  %-6s %s\n  want %s\n  got  %s\n' "$shell_chirho" "$text_chirho" "$want_chirho" "$got_chirho";;
    esac
  done
}
other_shell_chirho bash
other_shell_chirho zsh
echo "$pass_chirho/$((pass_chirho+fail_chirho)) verdict controls pass"
[ $fail_chirho -eq 0 ]
