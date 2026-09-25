#!/bin/bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# The one verdict rule for `haskelujah check`, sourced by every script here.
# Order matters (gpt_chirho review, room #24609): a timeout or an abnormal exit is
# classified first, then a REAL Rust panic header, and only then is a diagnostic
# credited: REJECT needs `error[E` AND exit 1, ACCEPT needs exit 0 and no `error[E`.
#
# A panic header is the shape the current Rust runtime prints, with or without the
# numeric thread id (gpt_chirho review, room #24614):
#   thread 'main' panicked at file.rs:1:1:
#   thread 'main' (46403379) panicked at file.rs:1:1:
# The id must be ALL digits: `(worker)` and `(4worker)` are not what the runtime
# prints, so neither is a header (gpt_chirho review, room #24703 — the earlier
# `[0-9]*` glob accepted `(4worker)`, because that glob means one digit then
# anything).
#
# WHERE a header may appear. It begins a line, and a line ends at either \n or a
# bare \r, so a header after a carriage return still begins one. It may also be
# GLUED to preceding output, when whatever wrote before it emitted no newline;
# that is a crash too, and missing it would credit a crash as a diagnostic, which
# is the one direction that must not happen (claude2_chirho's finding). What must
# still NOT count is a header QUOTED inside a diagnostic: a source excerpt behind
# a `|` gutter, the body of an `error[`/`warning:` line, or an indented
# continuation. So a glued header counts only when the text before it on its line
# is non-blank and carries none of those markers.
# The bare substring `panic` is never a verdict, because source can contain it.
# PURE SHELL matching, never grep: the interactive grep here is a ugrep wrapper.
# usage (sourced): verdict_chirho "<combined output>" <exit-code>  -> prints the verdict word

# True when `$1` begins with a real panic header.
#
# Every pattern here is built from QUOTED literals and tested one piece at a time,
# never as one glob containing bare parentheses. zsh treats an unquoted `(` in a
# pattern as a grouping operator, so a single combined pattern aborts the function
# with "bad pattern" and `verdict_chirho` then returns an EMPTY string instead of a
# verdict. That is fail-unsafe in a measuring instrument, and it was a regression:
# the rule this replaced worked under zsh. Every runner script here is bash, but the
# interactive shell in this environment is zsh and the rule is meant to be sourceable
# by hand. Controls run it under both shells.
header_shape_chirho() {
  case "$1" in
    "thread '"*) ;;
    *) return 1;;
  esac
  # Without a thread id.
  case "$1" in
    *"' panicked at"*) return 0;;
  esac
  # With one, which must be ALL digits.
  local after_chirho="${1#*\' }"
  case "$after_chirho" in
    "("*) ;;
    *) return 1;;
  esac
  local id_chirho="${after_chirho#"("}"
  case "$id_chirho" in
    *") panicked at"*) ;;
    *) return 1;;
  esac
  id_chirho="${id_chirho%%") panicked at"*}"
  case "$id_chirho" in
    '' | *[!0-9]*) return 1;;
  esac
  return 0
}

# True when the text before a glued header is ordinary output rather than a
# diagnostic quoting one.
glue_prefix_is_output_chirho() {
  case "$1" in
    # Nothing before it: that case was already decided as a line start.
    '') return 1;;
    # At least one non-blank character: something really was written before it.
    *[!" 	"]*) ;;
    # All blanks: an indented excerpt or continuation, not a crash.
    *) return 1;;
  esac
  case "$1" in
    *'|'* | *'error['* | *'warning:'*) return 1;;
  esac
  return 0
}

# True when some line of the output carries a real Rust panic header.
has_panic_header_chirho() {
  local line_chirho rest_chirho prefix_chirho
  while IFS= read -r line_chirho || [ -n "$line_chirho" ]; do
    if header_shape_chirho "$line_chirho"; then return 0; fi
    # Then each glued occurrence further along the same line.
    rest_chirho="$line_chirho"
    while case "$rest_chirho" in *"thread '"*) true;; *) false;; esac; do
      prefix_chirho="${rest_chirho%%thread \'*}"
      rest_chirho="thread '${rest_chirho#*thread \'}"
      if header_shape_chirho "$rest_chirho" &&
        glue_prefix_is_output_chirho "$prefix_chirho"; then
        return 0
      fi
      rest_chirho="${rest_chirho#thread \'}"
    done
  done <<CLASSIFY_INPUT_CHIRHO
${1//$'\r'/$'\n'}
CLASSIFY_INPUT_CHIRHO
  return 1
}

verdict_chirho() {
  local out_chirho="$1" rc_chirho="$2"
  if [ "$rc_chirho" -eq 124 ]; then echo TIMEOUT; return; fi
  if [ "$rc_chirho" -ne 0 ] && [ "$rc_chirho" -ne 1 ]; then echo ABNORMAL; return; fi
  if has_panic_header_chirho "$out_chirho"; then echo PANIC; return; fi
  case "$out_chirho" in
    *"error[E"*) if [ "$rc_chirho" -eq 1 ]; then echo REJECT; else echo UNCLASSIFIED; fi; return;;
  esac
  if [ "$rc_chirho" -eq 0 ]; then echo ACCEPT; else echo UNCLASSIFIED; fi
}
