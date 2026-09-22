// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

import { resolve as resolveChirho } from "node:path";
import { hash_chirho, json_lines_chirho } from "../../ascriptions-chirho/imported-families-chirho/source-boot-chirho/declarations-chirho/classes-chirho/default-annotations-chirho/scope-chirho/corpus-chirho/associated-methods-chirho/method-scopes-chirho/repair-chirho/representation-chirho/gnd-chirho/tools-chirho/support_chirho.ts";

if (!process.argv[2]) throw new Error("Supply the retained parent corpus directory");
const parent_chirho = resolveChirho(process.argv[2]);
const current_chirho = import.meta.dir;
const output_chirho = `${current_chirho}/comparison-chirho.json`;
if (await Bun.file(output_chirho).exists()) throw new Error("Comparisons are immutable");
const parent_receipt_chirho = await Bun.file(`${parent_chirho}/receipt-chirho.json`).json();
const current_receipt_chirho = await Bun.file(`${current_chirho}/receipt-chirho.json`).json();
for (const receipt_chirho of [parent_receipt_chirho, current_receipt_chirho]) {
  if (!receipt_chirho.complete_chirho || receipt_chirho.before_chirho !== receipt_chirho.after_chirho) throw new Error("Incomplete or unstable corpus receipt");
}
const comparisons_chirho = [];
const moved_chirho = [];
for (const [axis_chirho, total_chirho] of [["compile", 938], ["fail", 767]] as const) {
  const parent_bytes_chirho = Bun.gunzipSync(await Bun.file(`${parent_chirho}/${axis_chirho}-observations-chirho.jsonl.gz`).bytes());
  const current_bytes_chirho = Bun.gunzipSync(await Bun.file(`${current_chirho}/${axis_chirho}-observations-chirho.jsonl.gz`).bytes());
  const parent_rows_chirho = json_lines_chirho(new TextDecoder().decode(parent_bytes_chirho));
  const current_rows_chirho = json_lines_chirho(new TextDecoder().decode(current_bytes_chirho));
  for (const rows_chirho of [parent_rows_chirho, current_rows_chirho]) {
    if (rows_chirho.length !== total_chirho || new Set(rows_chirho.map(row_chirho => row_chirho.name_chirho)).size !== total_chirho || rows_chirho.some(row_chirho => !row_chirho.valid_chirho)) throw new Error("Invalid inventory or uncredited exit");
  }
  const parent_map_chirho = new Map(parent_rows_chirho.map(row_chirho => [row_chirho.name_chirho, row_chirho]));
  const gains_chirho = [];
  const losses_chirho = [];
  for (const row_chirho of current_rows_chirho) {
    const prior_chirho = parent_map_chirho.get(row_chirho.name_chirho);
    if (!prior_chirho || prior_chirho.source_sha256_chirho !== row_chirho.source_sha256_chirho) throw new Error(`Source identity changed: ${row_chirho.name_chirho}`);
    if (prior_chirho.rejected_chirho === row_chirho.rejected_chirho) continue;
    const gained_chirho = axis_chirho === "compile" ? !row_chirho.rejected_chirho : row_chirho.rejected_chirho;
    (gained_chirho ? gains_chirho : losses_chirho).push(row_chirho.name_chirho);
    const oracle_chirho = Bun.file(row_chirho.path_chirho.replace(/\.hs$/, ".stderr"));
    moved_chirho.push({ axis_chirho, gained_chirho, name_chirho: row_chirho.name_chirho,
      source_sha256_chirho: row_chirho.source_sha256_chirho,
      parent_stdout_chirho: prior_chirho.stdout_chirho, parent_stderr_chirho: prior_chirho.stderr_chirho,
      current_stdout_chirho: row_chirho.stdout_chirho, current_stderr_chirho: row_chirho.stderr_chirho,
      ghc_stderr_chirho: await oracle_chirho.exists() ? await oracle_chirho.text() : null,
      reason_audit_chirho: "Unreviewed: verdict movement alone gives no reason credit." });
  }
  comparisons_chirho.push({ axis_chirho, total_chirho, parent_observations_sha256_chirho: hash_chirho(parent_bytes_chirho), current_observations_sha256_chirho: hash_chirho(current_bytes_chirho), gains_chirho: gains_chirho.sort(), losses_chirho: losses_chirho.sort() });
}
await Bun.write(output_chirho, JSON.stringify({ parent_binary_sha256_chirho: parent_receipt_chirho.before_chirho, current_binary_sha256_chirho: current_receipt_chirho.before_chirho, comparisons_chirho }, null, 2) + "\n");
await Bun.write(`${current_chirho}/moved-diagnostics-chirho.jsonl.gz`, Bun.gzipSync(Buffer.from(moved_chirho.map(row_chirho => JSON.stringify(row_chirho)).join("\n") + "\n")));
console.log(JSON.stringify(comparisons_chirho, null, 2));
