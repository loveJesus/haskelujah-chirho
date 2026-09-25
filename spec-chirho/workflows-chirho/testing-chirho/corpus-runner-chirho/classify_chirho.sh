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
# GLUED to output that ended without a newline; that is a crash too, and missing it
# would credit a crash as a diagnostic, which is the one direction that must not
# happen (claude2_chirho's finding). What must still NOT count is a header quoted
# inside a diagnostic, so a line that is diagnostic RENDERING carries no crash
# anywhere along it.
# The bare substring `panic` is never a verdict, because source can contain it.
# PURE SHELL matching, never grep: the interactive grep here is a ugrep wrapper.
# usage (sourced): verdict_chirho "<combined output>" <exit-code>  -> prints the verdict word

# The literal every candidate starts with, kept in one place so the scan can strip
# it without another quoted-parenthesis pattern.
header_marker_chirho="thread '"

# True when `$1` begins with a real panic header.
#
# Every pattern here is built from QUOTED literals and tested one piece at a time,
# never as one glob containing bare parentheses. zsh reads an unquoted `(` in a
# pattern as a grouping operator, so a single combined pattern aborted the function
# with "bad pattern" and verdict_chirho then returned an EMPTY string instead of a
# verdict — fail-unsafe, and a regression, since the rule this replaced worked under
# zsh (claude2_chirho, room #24819). Every runner script here is bash, but the
# interactive shell in this environment is zsh and the rule is meant to be
# sourceable by hand. Controls run it under both shells.
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

# True when this LINE is diagnostic rendering rather than program output, in which
# case no header anywhere along it is a crash. Three shapes: a source excerpt behind
# a `|` gutter, the body of an `error[`/`warning:` line, and an indented
# continuation.
#
# The test is per LINE, not per candidate. A line can carry SEVERAL `thread '...'`
# fragments and only the first one sees the start of the line: scanning candidate by
# candidate while keeping only the text since the previous candidate loses the
# gutter, and a quoted string holding two fragments was then read as a crash
# (gpt_chirho review F1, room #24820).
line_is_quotation_chirho() {
  case "$1" in
    [" 	"]*) return 0;;
    *'|'* | *'error['* | *'warning:'*) return 0;;
  esac
  return 1
}

# True when some line of the output carries a real Rust panic header.
#
# BOUNDED LIMITATION, stated rather than papered over. Because a `|` anywhere on a
# line marks that line as rendering, a genuine glued header whose preceding output
# happens to contain a pipe — `progress|thread 'main' panicked at ...` — is MISSED.
# This rule does not distinguish all output from all quotation and does not claim
# to. It errs toward the side that matters for what is being protected: a quotation
# is never promoted to a crash. gpt_chirho's deliberately unanchored scan over all
# 7,396 retained stdout/stderr files from the four corpus passes and the replay
# found no header candidate at all, so no retained result depends on either side of
# this boundary.
has_panic_header_chirho() {
  local line_chirho rest_chirho prefix_chirho candidate_chirho before_chirho
  while IFS= read -r line_chirho || [ -n "$line_chirho" ]; do
    # A header that begins the line.
    if header_shape_chirho "$line_chirho"; then return 0; fi
    # Otherwise it may be glued to output — but never on a rendering line.
    if line_is_quotation_chirho "$line_chirho"; then continue; fi
    rest_chirho="$line_chirho"
    before_chirho=""
    while :; do
      case "$rest_chirho" in
        *"thread '"*) ;;
        *) break;;
      esac
      prefix_chirho="${rest_chirho%%thread \'*}"
      candidate_chirho="$header_marker_chirho${rest_chirho#*thread \'}"
      # Something must really have been written before it, judged against the WHOLE
      # line so far and not merely since the previous candidate.
      case "$before_chirho$prefix_chirho" in
        *[!" 	"]*) header_shape_chirho "$candidate_chirho" && return 0;;
      esac
      before_chirho="$before_chirho$prefix_chirho$header_marker_chirho"
      rest_chirho="${candidate_chirho#"$header_marker_chirho"}"
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
