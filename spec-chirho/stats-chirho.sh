#!/usr/bin/env bash
# For God so loved the world that he gave his only begotten Son, that whoever
# believes in him should not perish but have eternal life. — John 3:16

# stats-chirho.sh — Single source of truth for project metrics.
# Run from repo root: bash spec-chirho/stats-chirho.sh
# Outputs JSON and human-readable stats that can feed README, docs, AGENTS.md.

set -euo pipefail

REPO_ROOT_CHIRHO="$(cd "$(dirname "$0")/.." && pwd)"

# Crate count
CRATE_COUNT_CHIRHO=$(grep '"crates/' "$REPO_ROOT_CHIRHO/Cargo.toml" | wc -l | tr -d ' ')

# Rust file count
RUST_FILES_CHIRHO=$(find "$REPO_ROOT_CHIRHO/crates" -name '*.rs' | wc -l | tr -d ' ')

# Rust LOC
RUST_LOC_CHIRHO=$(find "$REPO_ROOT_CHIRHO/crates" -name '*.rs' -exec cat {} + | wc -l | tr -d ' ')

# Test count (from cargo test, cached if available)
STATS_CACHE_CHIRHO="$REPO_ROOT_CHIRHO/spec-chirho/.stats-cache-chirho.json"
if [ "${1:-}" = "--run-tests" ]; then
    TEST_COUNT_CHIRHO=$(cargo test --workspace --quiet 2>&1 | grep "^test result:" | awk -F'[;,]' '{for(i=1;i<=NF;i++) if($i ~ /passed/) {gsub(/[^0-9]/,"",$i); sum+=$i}} END{print sum}')
    echo "{\"test_count_chirho\": $TEST_COUNT_CHIRHO}" > "$STATS_CACHE_CHIRHO"
elif [ -f "$STATS_CACHE_CHIRHO" ]; then
    TEST_COUNT_CHIRHO=$(python3 -c "import json; print(json.load(open('$STATS_CACHE_CHIRHO'))['test_count_chirho'])" 2>/dev/null || echo "unknown")
else
    TEST_COUNT_CHIRHO="unknown"
fi

# Backend maturity
BACKEND_MATURITY_CHIRHO='{
  "llvm_chirho": "beta",
  "wasm_chirho": "beta",
  "cranelift_chirho": "beta",
  "jvm_chirho": "research",
  "beam_chirho": "research"
}'

# Module interface count
IFACE_COUNT_CHIRHO=$(grep -c 'name_chirho:.*"[A-Z]' "$REPO_ROOT_CHIRHO/crates/haskelujah-naming-chirho/src/iface_chirho.rs" 2>/dev/null | tr -d ' ' || echo "0")

# Output
cat << EOFSTATS
{
  "date_chirho": "$(date +%Y-%m-%d)",
  "crate_count_chirho": $CRATE_COUNT_CHIRHO,
  "rust_files_chirho": $RUST_FILES_CHIRHO,
  "rust_loc_chirho": $RUST_LOC_CHIRHO,
  "test_count_chirho": "$TEST_COUNT_CHIRHO",
  "module_ifaces_chirho": $IFACE_COUNT_CHIRHO,
  "backend_maturity_chirho": $BACKEND_MATURITY_CHIRHO
}
EOFSTATS

# Human-readable
echo ""
echo "=== Haskelujah Chirho Stats ($(date +%Y-%m-%d)) ==="
echo "Crates:           $CRATE_COUNT_CHIRHO"
echo "Rust source files: $RUST_FILES_CHIRHO"
echo "Rust LOC:          $RUST_LOC_CHIRHO"
echo "Tests passing:     $TEST_COUNT_CHIRHO"
echo "Module interfaces: $IFACE_COUNT_CHIRHO"
echo ""
echo "Backend Maturity:"
echo "  LLVM:      beta"
echo "  Wasm:      beta"
echo "  Cranelift: experimental"
echo "  JVM:       research"
echo "  BEAM:      research"
