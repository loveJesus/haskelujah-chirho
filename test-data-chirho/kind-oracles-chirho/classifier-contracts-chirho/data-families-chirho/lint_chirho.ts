// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Audit the retained clippy output against this unit's actual changed lines.
const directory_chirho = `${import.meta.dir}/data-family-gates-chirho`;
const output_chirho = `${directory_chirho}/lint-chirho.json`;
if (await Bun.file(output_chirho).exists()) throw new Error("Lint receipts are immutable");
const bytes_chirho = Bun.gunzipSync(await Bun.file(`${directory_chirho}/gate-1-stdout-chirho.gz`).bytes());
const messages_chirho = new TextDecoder().decode(bytes_chirho).split("\n").filter(Boolean).map(line_chirho => JSON.parse(line_chirho));
const warnings_chirho = messages_chirho.filter(row_chirho => row_chirho.reason === "compiler-message" && row_chirho.message.level === "warning").map(row_chirho => row_chirho.message);
const diff_chirho = Bun.spawnSync(["git", "diff", "--unified=0", "--", "crates/"]);
if (diff_chirho.exitCode !== 0) throw new Error(diff_chirho.stderr.toString());
const lines_chirho = new Map<string, [number, number][]>();
let path_chirho = "";
for (const line_chirho of diff_chirho.stdout.toString().split("\n")) {
  if (line_chirho.startsWith("+++ b/")) {
    path_chirho = line_chirho.slice(6);
    lines_chirho.set(path_chirho, []);
  }
  const match_chirho = line_chirho.match(/^@@ .* \+(\d+)(?:,(\d+))? @@/);
  if (match_chirho && Number(match_chirho[2] ?? 1) > 0) {
    const first_chirho = Number(match_chirho[1]);
    lines_chirho.get(path_chirho)?.push([first_chirho, first_chirho + Number(match_chirho[2] ?? 1) - 1]);
  }
}
const untracked_chirho = Bun.spawnSync(["git", "ls-files", "--others", "--exclude-standard", "--", "crates/"]);
if (untracked_chirho.exitCode !== 0) throw new Error(untracked_chirho.stderr.toString());
for (const file_chirho of untracked_chirho.stdout.toString().split("\n").filter(Boolean)) lines_chirho.set(file_chirho, [[1, Number.MAX_SAFE_INTEGER]]);
const distinct_chirho = [...new Map(warnings_chirho.map(warning_chirho => [JSON.stringify({ code_chirho: warning_chirho.code, message_chirho: warning_chirho.message, spans_chirho: warning_chirho.spans }), warning_chirho])).values()];
const changed_chirho = distinct_chirho.filter(warning_chirho => warning_chirho.spans.some((span_chirho: { is_primary: boolean; file_name: string; line_start: number; line_end: number }) =>
  span_chirho.is_primary && (lines_chirho.get(span_chirho.file_name) ?? []).some(([first_chirho, last_chirho]) => span_chirho.line_start <= last_chirho && span_chirho.line_end >= first_chirho)));
const receipt_chirho = {
  log_sha256_chirho: new Bun.CryptoHasher("sha256").update(bytes_chirho).digest("hex"),
  distinct_warnings_chirho: distinct_chirho.length,
  changed_line_warnings_chirho: changed_chirho,
  clean_lint_chirho: distinct_chirho.length === 0,
};
await Bun.write(output_chirho, JSON.stringify(receipt_chirho, null, 2) + "\n");
console.log(JSON.stringify(receipt_chirho));
if (changed_chirho.length) process.exitCode = 1;
