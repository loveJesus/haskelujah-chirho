#!/bin/bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# Replay every REJECT row of saved passes on the same frozen CLI, one file at a time,
# keeping stdout, stderr, exit code and source SHA-256 per file, and classify each with
# the one verdict rule. A pass list establishes status and membership; this replay is
# what establishes that each rejection is a diagnostic and not a crash.
# usage: replay_rejects_chirho.sh <bin> <out-dir> <corpus-dir> <pass-file> [<corpus-dir> <pass-file> ...]
bin_chirho="$1"; out_chirho="$2"; shift 2
here_chirho="$(cd "$(dirname "$0")" && pwd)"
mkdir -p "$out_chirho"
: > "$out_chirho/replay-verdicts-chirho.txt"
while [ $# -ge 2 ]; do
  dir_chirho="$1"; pass_chirho="$2"; shift 2
  axis_chirho="$(basename "$dir_chirho")"
  awk '$1=="REJECT" {print $2}' "$pass_chirho" | while read -r rel_chirho; do
    line_chirho="$("$here_chirho/worker_chirho.sh" "$bin_chirho" "$dir_chirho" 60 "$out_chirho/raw-$axis_chirho" "$dir_chirho/$rel_chirho")"
    echo "$axis_chirho $line_chirho" >> "$out_chirho/replay-verdicts-chirho.txt"
  done
done
echo "replayed $(wc -l < "$out_chirho/replay-verdicts-chirho.txt" | tr -d ' ') rejections:"
awk '{n[$2]++} END {for (v in n) printf "  %s=%d\n", v, n[v]}' "$out_chirho/replay-verdicts-chirho.txt"
echo "CLI $(shasum -a 256 "$bin_chirho" | cut -d' ' -f1)"
