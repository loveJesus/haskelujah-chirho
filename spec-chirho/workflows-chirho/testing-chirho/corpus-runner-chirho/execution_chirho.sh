#!/bin/bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# Execution evidence on one frozen CLI. Every command's stdout, stderr and exit code are
# kept (gpt_chirho review, room #24611): a PASS needs the reference (runghc) to exit 0,
# the tested command to exit 0, and stdout byte-identical to the reference. A native
# compile failure is kept with its output and is a FAIL, never a skip.
# usage: execution_chirho.sh <frozen-cli> <sources-dir> <out-dir> [<reject-corpus-dir>]
cli_chirho="$1"; sources_chirho="$2"; out_chirho="$3"; rejects_chirho="${4:-}"
here_chirho="$(cd "$(dirname "$0")" && pwd)"
# The rejection checks below judge with the SAME rule as every corpus pass, never a
# second copy of the pattern (gpt_chirho review, room #24614).
. "$here_chirho/classify_chirho.sh"
mkdir -p "$out_chirho"
pass_chirho=0; fail_chirho=0
report_chirho() { if [ "$1" = PASS ]; then pass_chirho=$((pass_chirho+1)); else fail_chirho=$((fail_chirho+1)); fi; printf '%s  %s\n' "$1" "$2"; }
# run_chirho <tag> <command...>: keeps <tag>.stdout, <tag>.stderr and <tag>.rc
run_chirho() { local tag_chirho="$1"; shift; "$@" > "$out_chirho/$tag_chirho.stdout" 2> "$out_chirho/$tag_chirho.stderr"; echo $? > "$out_chirho/$tag_chirho.rc"; }
rc_of_chirho() { cat "$out_chirho/$1.rc"; }
echo "frozen CLI $(shasum -a 256 "$cli_chirho" | cut -d' ' -f1)"
if [ -n "$rejects_chirho" ]; then
  for pair in "SCLoop:SC ()" "T5684:A Bool" "T5684b:A Bool" "T5684c:A Bool" "T5684d:A Bool" "T5684e:A Bool" "T5684f:A Bool"; do
    file_chirho="${pair%%:*}"; pred_chirho="${pair#*:}"
    run_chirho "reject-$file_chirho" "$cli_chirho" check "$rejects_chirho/$file_chirho.hs"
    out_text_chirho="$(cat "$out_chirho/reject-$file_chirho.stdout" "$out_chirho/reject-$file_chirho.stderr")"
    rc_chirho="$(rc_of_chirho "reject-$file_chirho")"
    verdict_word_chirho="$(verdict_chirho "$out_text_chirho" "$rc_chirho")"
    if [ "$verdict_word_chirho" != REJECT ]; then
      report_chirho FAIL "reject $file_chirho: verdict $verdict_word_chirho, rc=$rc_chirho"
    else
      case "$out_text_chirho" in
        *"error[E0204]"*"no instance for \`$pred_chirho\`"*) report_chirho PASS "reject $file_chirho: E0204 no instance for \`$pred_chirho\`, rc=1";;
        *) report_chirho FAIL "reject $file_chirho: expected E0204 naming \`$pred_chirho\`, rc=$rc_chirho";;
      esac
    fi
  done
fi
for source_chirho in "$sources_chirho"/*.hs; do
  name_chirho="$(basename "$source_chirho" .hs)"
  (cd "$sources_chirho" && run_chirho "ghc-$name_chirho" runghc "$name_chirho.hs")
  ghc_rc_chirho="$(rc_of_chirho "ghc-$name_chirho")"
  if [ "$ghc_rc_chirho" -ne 0 ]; then report_chirho FAIL "reference $name_chirho: runghc rc=$ghc_rc_chirho"; continue; fi
  run_chirho "run-$name_chirho" "$cli_chirho" run "$source_chirho"
  rc_chirho="$(rc_of_chirho "run-$name_chirho")"
  if [ "$rc_chirho" -eq 0 ] && cmp -s "$out_chirho/run-$name_chirho.stdout" "$out_chirho/ghc-$name_chirho.stdout"; then
    report_chirho PASS "run    $name_chirho: $(tr '\n' '/' < "$out_chirho/run-$name_chirho.stdout")"
  else
    report_chirho FAIL "run    $name_chirho: rc=$rc_chirho stdout=$(tr '\n' '/' < "$out_chirho/run-$name_chirho.stdout") stderr=$(head -c 120 "$out_chirho/run-$name_chirho.stderr" | tr '\n' ' ')"
  fi
  run_chirho "compile-$name_chirho" "$cli_chirho" compile "$source_chirho" -o "$out_chirho/native-$name_chirho"
  compile_rc_chirho="$(rc_of_chirho "compile-$name_chirho")"
  if [ "$compile_rc_chirho" -ne 0 ]; then
    report_chirho FAIL "native $name_chirho: compile rc=$compile_rc_chirho: $(head -c 120 "$out_chirho/compile-$name_chirho.stderr" | tr '\n' ' ')"
    continue
  fi
  run_chirho "native-run-$name_chirho" "$out_chirho/native-$name_chirho"
  rc_chirho="$(rc_of_chirho "native-run-$name_chirho")"
  if [ "$rc_chirho" -eq 0 ] && cmp -s "$out_chirho/native-run-$name_chirho.stdout" "$out_chirho/ghc-$name_chirho.stdout"; then
    report_chirho PASS "native $name_chirho: $(tr '\n' '/' < "$out_chirho/native-run-$name_chirho.stdout")"
  else
    report_chirho FAIL "native $name_chirho: rc=$rc_chirho stdout=$(tr '\n' '/' < "$out_chirho/native-run-$name_chirho.stdout")"
  fi
done
echo "source SHA-256:"; (cd "$sources_chirho" && shasum -a 256 *.hs)
echo "$pass_chirho/$((pass_chirho+fail_chirho)) predicates pass"
echo "frozen CLI after $(shasum -a 256 "$cli_chirho" | cut -d' ' -f1)"
[ $fail_chirho -eq 0 ]
