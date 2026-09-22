#!/bin/bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# The one verdict rule for `haskelujah check`, sourced by the worker and the replay.
# Order matters (gpt_chirho review, room #24609): a timeout or an abnormal exit is
# classified first, then a REAL Rust panic header, and only then is a diagnostic
# credited: REJECT needs `error[E` AND exit 1, ACCEPT needs exit 0 and no `error[E`.
# The bare substring `panic` is never a verdict: quoted source can contain it.
# PURE SHELL matching, never grep: the interactive grep here is a ugrep wrapper.
# usage (sourced): verdict_chirho "<combined output>" <exit-code>  -> prints the verdict word
verdict_chirho() {
  local out_chirho="$1" rc_chirho="$2"
  if [ "$rc_chirho" -eq 124 ]; then echo TIMEOUT; return; fi
  if [ "$rc_chirho" -ne 0 ] && [ "$rc_chirho" -ne 1 ]; then echo ABNORMAL; return; fi
  case "$out_chirho" in
    *"thread '"*"' panicked at"*) echo PANIC; return;;
  esac
  case "$out_chirho" in
    *"error[E"*) if [ "$rc_chirho" -eq 1 ]; then echo REJECT; else echo UNCLASSIFIED; fi; return;;
  esac
  if [ "$rc_chirho" -eq 0 ]; then echo ACCEPT; else echo UNCLASSIFIED; fi
}
