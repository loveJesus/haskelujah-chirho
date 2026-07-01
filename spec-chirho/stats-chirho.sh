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

# Test counts (from cargo test, cached if available)
STATS_CACHE_CHIRHO="$REPO_ROOT_CHIRHO/spec-chirho/.stats-cache-chirho.json"
TEST_PASSED_COUNT_CHIRHO="unknown"
TEST_FAILED_COUNT_CHIRHO="unknown"
TEST_IGNORED_COUNT_CHIRHO="unknown"
TEST_MEASURED_COUNT_CHIRHO="unknown"
TEST_FILTERED_COUNT_CHIRHO="unknown"
TEST_RESULT_LINES_CHIRHO="unknown"
TEST_CARGO_STATUS_CHIRHO="unknown"
TEST_TIMESTAMP_CHIRHO="unknown"
TEST_COMMAND_CHIRHO="cargo test --workspace --quiet"
TEST_SOURCE_CHIRHO="none"

load_test_stats_cache_chirho() {
    if [ ! -f "$STATS_CACHE_CHIRHO" ]; then
        return 0
    fi

    python3 - "$STATS_CACHE_CHIRHO" <<'PYCACHE'
import json
import shlex
import sys

cache_path_chirho = sys.argv[1]
try:
    with open(cache_path_chirho, "r", encoding="utf-8") as cache_file_chirho:
        data_chirho = json.load(cache_file_chirho)
except Exception:
    data_chirho = {}

values_chirho = {
    "TEST_PASSED_COUNT_CHIRHO": data_chirho.get(
        "test_passed_count_chirho",
        data_chirho.get("test_count_chirho", "unknown"),
    ),
    "TEST_FAILED_COUNT_CHIRHO": data_chirho.get("test_failed_count_chirho", "unknown"),
    "TEST_IGNORED_COUNT_CHIRHO": data_chirho.get("test_ignored_count_chirho", "unknown"),
    "TEST_MEASURED_COUNT_CHIRHO": data_chirho.get("test_measured_count_chirho", "unknown"),
    "TEST_FILTERED_COUNT_CHIRHO": data_chirho.get("test_filtered_count_chirho", "unknown"),
    "TEST_RESULT_LINES_CHIRHO": data_chirho.get("test_result_lines_chirho", "unknown"),
    "TEST_CARGO_STATUS_CHIRHO": data_chirho.get("test_cargo_status_chirho", "unknown"),
    "TEST_TIMESTAMP_CHIRHO": data_chirho.get("test_timestamp_chirho", "unknown"),
    "TEST_COMMAND_CHIRHO": data_chirho.get(
        "test_command_chirho",
        "cargo test --workspace --quiet",
    ),
}

for key_chirho, value_chirho in values_chirho.items():
    print(f"{key_chirho}={shlex.quote(str(value_chirho))}")
PYCACHE
}

write_test_stats_cache_chirho() {
    local output_path_chirho="$1"
    local cargo_status_chirho="$2"

    python3 - "$output_path_chirho" "$STATS_CACHE_CHIRHO" "$cargo_status_chirho" <<'PYRUN'
import datetime
import json
import re
import sys

output_path_chirho = sys.argv[1]
cache_path_chirho = sys.argv[2]
cargo_status_chirho = int(sys.argv[3])

with open(output_path_chirho, "r", encoding="utf-8", errors="replace") as output_file_chirho:
    output_chirho = output_file_chirho.read()

totals_chirho = {
    "passed": 0,
    "failed": 0,
    "ignored": 0,
    "measured": 0,
    "filtered_out": 0,
}
labels_chirho = {
    "passed": "passed",
    "failed": "failed",
    "ignored": "ignored",
    "measured": "measured",
    "filtered_out": "filtered out",
}
result_lines_chirho = 0

for line_chirho in output_chirho.splitlines():
    if not line_chirho.startswith("test result:"):
        continue
    result_lines_chirho += 1
    for key_chirho, label_chirho in labels_chirho.items():
        match_chirho = re.search(rf"(\d+)\s+{re.escape(label_chirho)}", line_chirho)
        if match_chirho:
            totals_chirho[key_chirho] += int(match_chirho.group(1))

try:
    with open(cache_path_chirho, "r", encoding="utf-8") as cache_file_chirho:
        data_chirho = json.load(cache_file_chirho)
except Exception:
    data_chirho = {}

data_chirho.update(
    {
        "test_count_chirho": totals_chirho["passed"],
        "test_passed_count_chirho": totals_chirho["passed"],
        "test_failed_count_chirho": totals_chirho["failed"],
        "test_ignored_count_chirho": totals_chirho["ignored"],
        "test_measured_count_chirho": totals_chirho["measured"],
        "test_filtered_count_chirho": totals_chirho["filtered_out"],
        "test_result_lines_chirho": result_lines_chirho,
        "test_cargo_status_chirho": cargo_status_chirho,
        "test_command_chirho": "cargo test --workspace --quiet",
        "test_timestamp_chirho": datetime.datetime.now(datetime.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z"),
    }
)

with open(cache_path_chirho, "w", encoding="utf-8") as cache_file_chirho:
    json.dump(data_chirho, cache_file_chirho, sort_keys=True)
    cache_file_chirho.write("\n")

if cargo_status_chirho != 0 or totals_chirho["failed"] != 0:
    sys.exit(1)
PYRUN
}

if [ "${1:-}" = "--run-tests" ]; then
    TEST_OUTPUT_CHIRHO="$(mktemp)"
    set +e
    cargo test --workspace --quiet > "$TEST_OUTPUT_CHIRHO" 2>&1
    CARGO_STATUS_CHIRHO=$?
    set -e

    if ! write_test_stats_cache_chirho "$TEST_OUTPUT_CHIRHO" "$CARGO_STATUS_CHIRHO"; then
        cat "$TEST_OUTPUT_CHIRHO" >&2
        rm -f "$TEST_OUTPUT_CHIRHO"
        eval "$(load_test_stats_cache_chirho)"
        echo "stats-chirho: cargo test reported failures; cache updated with failure counts." >&2
        exit 1
    fi
    rm -f "$TEST_OUTPUT_CHIRHO"
    eval "$(load_test_stats_cache_chirho)"
    TEST_SOURCE_CHIRHO="live"
elif [ -f "$STATS_CACHE_CHIRHO" ]; then
    eval "$(load_test_stats_cache_chirho)"
    TEST_SOURCE_CHIRHO="cached"
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
  "test_count_chirho": "$TEST_PASSED_COUNT_CHIRHO",
  "test_passed_count_chirho": "$TEST_PASSED_COUNT_CHIRHO",
  "test_failed_count_chirho": "$TEST_FAILED_COUNT_CHIRHO",
  "test_ignored_count_chirho": "$TEST_IGNORED_COUNT_CHIRHO",
  "test_measured_count_chirho": "$TEST_MEASURED_COUNT_CHIRHO",
  "test_filtered_count_chirho": "$TEST_FILTERED_COUNT_CHIRHO",
  "test_result_lines_chirho": "$TEST_RESULT_LINES_CHIRHO",
  "test_cargo_status_chirho": "$TEST_CARGO_STATUS_CHIRHO",
  "test_timestamp_chirho": "$TEST_TIMESTAMP_CHIRHO",
  "test_source_chirho": "$TEST_SOURCE_CHIRHO",
  "test_command_chirho": "$TEST_COMMAND_CHIRHO",
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
echo "Tests passed:      $TEST_PASSED_COUNT_CHIRHO"
echo "Tests failed:      $TEST_FAILED_COUNT_CHIRHO"
echo "Tests ignored:     $TEST_IGNORED_COUNT_CHIRHO"
echo "Test stats source: $TEST_SOURCE_CHIRHO ($TEST_TIMESTAMP_CHIRHO)"
echo "Test command:      $TEST_COMMAND_CHIRHO"
echo "Module interfaces: $IFACE_COUNT_CHIRHO"
echo ""
echo "Backend Maturity:"
echo "  LLVM:      beta"
echo "  Wasm:      beta"
echo "  Cranelift: experimental"
echo "  JVM:       research"
echo "  BEAM:      research"
