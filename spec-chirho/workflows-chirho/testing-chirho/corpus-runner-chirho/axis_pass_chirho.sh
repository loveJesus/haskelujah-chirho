#!/bin/bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# One axis, one complete pass: find-based walk (subdirectories included), parallelism 4,
# 15s per file, every TIMEOUT re-run alone at 60s before it counts.
# usage: axis_pass_chirho.sh <bin> <corpus-dir> <out-file> [<raw-dir>]
bin_chirho="$1"; dir_chirho="$2"; out_chirho="$3"; raw_chirho="${4:-}"
here_chirho="$(cd "$(dirname "$0")" && pwd)"
# One C-collated ordering, so a verdict file and its list hash do not depend on the
# caller's locale (see sort_chirho.sh).
. "$here_chirho/sort_chirho.sh"
find "$dir_chirho" -name '*.hs' -print0 \
  | xargs -0 -P 4 -n 1 "$here_chirho/worker_chirho.sh" "$bin_chirho" "$dir_chirho" 15 "$raw_chirho" \
  | sort_rows_chirho > "$out_chirho"
awk '$1=="TIMEOUT" {print $2}' "$out_chirho" | while read -r rel_chirho; do
  line_chirho="$("$here_chirho/worker_chirho.sh" "$bin_chirho" "$dir_chirho" 60 "$raw_chirho" "$dir_chirho/$rel_chirho")"
  python3 - "$out_chirho" "$rel_chirho" "$line_chirho" <<'PY'
import sys
path, rel, verdict = sys.argv[1], sys.argv[2], sys.argv[3]
lines = open(path).read().splitlines()
lines = [verdict if l.split()[1:2] == [rel] else l for l in lines]
open(path, "w").write("\n".join(lines) + "\n")
PY
done
awk -v name="$(basename "$dir_chirho")" '{n[$1]++; t++} END {printf "%s: total=%d", name, t; for (v in n) printf " %s=%d", v, n[v]; printf "\n"}' "$out_chirho"
