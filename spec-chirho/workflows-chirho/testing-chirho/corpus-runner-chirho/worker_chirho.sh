#!/bin/bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# One file: print "<verdict> <rel> <exit-code>". With a raw directory, also keep the
# file's stdout, stderr, exit code and source SHA-256 there.
# usage: worker_chirho.sh <bin> <root> <seconds> <raw-dir-or-empty> <file>
bin_chirho="$1"; root_chirho="$2"; seconds_chirho="$3"; raw_chirho="$4"; file_chirho="$5"
here_chirho="$(cd "$(dirname "$0")" && pwd)"
. "$here_chirho/classify_chirho.sh"
rel_chirho="${file_chirho#$root_chirho/}"
scratch_chirho="$(mktemp -d)"
/opt/homebrew/bin/timeout "$seconds_chirho" "$bin_chirho" check "$file_chirho" \
  > "$scratch_chirho/stdout" 2> "$scratch_chirho/stderr"
rc_chirho=$?
out_chirho="$(cat "$scratch_chirho/stdout" "$scratch_chirho/stderr")"
verdict_word_chirho="$(verdict_chirho "$out_chirho" "$rc_chirho")"
if [ -n "$raw_chirho" ]; then
  key_chirho="$(printf '%s' "$rel_chirho" | tr '/' '%')"
  mkdir -p "$raw_chirho"
  cp "$scratch_chirho/stdout" "$raw_chirho/$key_chirho.stdout"
  cp "$scratch_chirho/stderr" "$raw_chirho/$key_chirho.stderr"
  printf '%s %s %s %s\n' "$verdict_word_chirho" "$rel_chirho" "$rc_chirho" \
    "$(shasum -a 256 "$file_chirho" | cut -d' ' -f1)" > "$raw_chirho/$key_chirho.verdict"
fi
rm -rf "$scratch_chirho"
echo "$verdict_word_chirho $rel_chirho $rc_chirho"
