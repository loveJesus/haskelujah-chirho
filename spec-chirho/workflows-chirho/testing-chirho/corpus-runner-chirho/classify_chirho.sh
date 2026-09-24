#!/bin/bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# The one verdict rule for `haskelujah check`, sourced by every script here.
# Order matters (gpt_chirho review, room #24609): a timeout or an abnormal exit is
# classified first, then a REAL Rust panic header, and only then is a diagnostic
# credited: REJECT needs `error[E` AND exit 1, ACCEPT needs exit 0 and no `error[E`.
#
# A panic header is ANCHORED TO THE START OF A LINE and allows the thread id the
# current Rust runtime prints (gpt_chirho review, room #24614):
#   thread 'main' panicked at file.rs:1:1:
#   thread 'main' (46403379) panicked at file.rs:1:1:
# Anchoring is what keeps a header QUOTED INSIDE A DIAGNOSTIC from counting: an
# excerpt line is indented behind its line number and gutter. The bare substring
# `panic` is never a verdict, because source can contain it.
# PURE SHELL matching, never grep: the interactive grep here is a ugrep wrapper.
# usage (sourced): verdict_chirho "<combined output>" <exit-code>  -> prints the verdict word

# True when some line of the output BEGINS a real Rust panic header.
has_panic_header_chirho() {
  local line_chirho
  while IFS= read -r line_chirho || [ -n "$line_chirho" ]; do
    case "$line_chirho" in
      "thread '"*"' panicked at"*) return 0;;
      "thread '"*"' ("[0-9]*") panicked at"*) return 0;;
    esac
  done <<CLASSIFY_INPUT_CHIRHO
$1
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
