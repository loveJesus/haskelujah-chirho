// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
import { resolve } from "node:path";
import { git_chirho, hash_chirho, json_lines_chirho } from "./support_chirho.ts";

const checkpoint_chirho = resolve(import.meta.dir, "../checkpoint-chirho");
const gate_directory_chirho = process.argv[2] ?? "gates-chirho";
const bytes_chirho = Bun.gunzipSync(await Bun.file(`${checkpoint_chirho}/${gate_directory_chirho}/gate-1-stdout-chirho.gz`).bytes());
const warnings_chirho = json_lines_chirho(new TextDecoder().decode(bytes_chirho))
  .filter(row_chirho => row_chirho.reason === "compiler-message" && row_chirho.message.level === "warning").map(row_chirho => row_chirho.message);
const lines_chirho = new Map<string, [number, number][]>();
let path_chirho = "";
for (const line_chirho of git_chirho("diff", "--unified=0", "--", "crates/").split("\n")) {
  if (line_chirho.startsWith("+++ b/")) { path_chirho = line_chirho.slice(6); lines_chirho.set(path_chirho, []); }
  const match_chirho = line_chirho.match(/^@@ .* \+(\d+)(?:,(\d+))? @@/);
  if (match_chirho && Number(match_chirho[2] ?? 1) > 0) lines_chirho.get(path_chirho)?.push([Number(match_chirho[1]), Number(match_chirho[1]) + Number(match_chirho[2] ?? 1) - 1]);
}
for (const path_chirho of git_chirho("ls-files", "--others", "--exclude-standard", "--", "crates/").split("\n").filter(Boolean)) lines_chirho.set(path_chirho, [[1, Number.MAX_SAFE_INTEGER]]);
const unique_chirho = [...new Map(warnings_chirho.map(warning_chirho => [JSON.stringify({ code_chirho: warning_chirho.code, message_chirho: warning_chirho.message, spans_chirho: warning_chirho.spans }), warning_chirho])).values()];
const changed_chirho = unique_chirho.filter(warning_chirho => warning_chirho.spans.some((span_chirho: { is_primary: boolean; file_name: string; line_start: number; line_end: number }) =>
  span_chirho.is_primary && (lines_chirho.get(span_chirho.file_name) ?? []).some(([start_chirho, end_chirho]) => span_chirho.line_start <= end_chirho && span_chirho.line_end >= start_chirho)));
const receipt_chirho = { log_sha256_chirho: hash_chirho(bytes_chirho), distinct_warnings_chirho: unique_chirho.length, changed_line_warnings_chirho: changed_chirho, clean_lint_chirho: unique_chirho.length === 0 };
await Bun.write(`${checkpoint_chirho}/${gate_directory_chirho === "gates-chirho" ? "lint" : "final-lint"}-chirho.json`, JSON.stringify(receipt_chirho, null, 2) + "\n");
console.log(JSON.stringify(receipt_chirho));
if (changed_chirho.length) process.exitCode = 1;
