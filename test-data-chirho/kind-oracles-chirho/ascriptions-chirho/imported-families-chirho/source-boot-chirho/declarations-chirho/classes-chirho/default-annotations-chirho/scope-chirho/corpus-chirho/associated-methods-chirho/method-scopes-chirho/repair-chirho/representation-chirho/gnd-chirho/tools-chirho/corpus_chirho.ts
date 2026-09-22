// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
// One diagnostic pair only. Coordinate START/DONE separately; do not publish totals as a landing.
import { mkdir, readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { git_chirho, hash_chirho, observe_chirho } from "./support_chirho.ts";

const evidence_chirho = resolve(import.meta.dir, "..");
const gate_directory_chirho = process.argv[2] ?? "gates-chirho";
const gate_path_chirho = gate_directory_chirho.endsWith(".json")
  ? resolve(gate_directory_chirho)
  : `${evidence_chirho}/checkpoint-chirho/${gate_directory_chirho}/receipt-chirho.json`;
const gate_chirho = await Bun.file(gate_path_chirho).json();
if (!gate_chirho.complete_chirho) throw new Error("Gates incomplete");
const binary_chirho: string = gate_chirho.binary_path_chirho;
const output_chirho = process.argv[3] ? resolve(process.argv[3]) : `${evidence_chirho}/checkpoint-chirho/corpus-chirho`;
if (await Bun.file(`${output_chirho}/receipt-chirho.json`).exists()) throw new Error("Corpus output exists");
await mkdir(output_chirho, { recursive: true });
const before_chirho = hash_chirho(await readFile(binary_chirho));
if (before_chirho !== gate_chirho.binary_sha256_chirho) throw new Error("CLI provenance mismatch");
const main_chirho = git_chirho("rev-parse", "main_chirho");
const summaries_chirho: Record<string, unknown>[] = [];
const receipt_chirho = { started_chirho: new Date().toISOString(), binary_chirho, before_chirho, after_chirho: "", main_chirho,
  gate_head_chirho: gate_chirho.head_chirho, gate_directory_chirho, subject_head_chirho: git_chirho("rev-parse", "HEAD"), summaries_chirho, complete_chirho: false,
  scope_chirho: "One diagnostic pass per axis; not a landing measurement. No reason credit from verdicts alone." };
const save_chirho = () => Bun.write(`${output_chirho}/receipt-chirho.json`, JSON.stringify(receipt_chirho, null, 2) + "\n");
await save_chirho();
let all_valid_chirho = true;
for (const [axis_chirho, total_chirho] of [["compile", 938], ["fail", 767]] as const) {
  const root_chirho = `ghc-tests-chirho/typecheck-chirho/should_${axis_chirho}`;
  const listed_chirho = Bun.spawnSync(["rg", "--files", root_chirho]);
  if (listed_chirho.exitCode !== 0) throw new Error("Corpus enumeration failed");
  const files_chirho = listed_chirho.stdout.toString().trim().split("\n").filter(path_chirho => path_chirho.endsWith(".hs")).sort();
  if (files_chirho.length !== total_chirho) throw new Error(`Wrong denominator: ${axis_chirho} ${files_chirho.length}`);
  const rows_chirho: (Awaited<ReturnType<typeof observe_chirho>> & { name_chirho: string })[] = [];
  let next_chirho = 0;
  await Promise.all(Array.from({ length: 4 }, async () => {
    while (next_chirho < files_chirho.length) {
      const index_chirho = next_chirho++;
      const path_chirho = files_chirho[index_chirho];
      rows_chirho[index_chirho] = { name_chirho: path_chirho.slice(root_chirho.length + 1), ...await observe_chirho(binary_chirho, path_chirho, 15000) };
      if ((index_chirho + 1) % 100 === 0) console.log(`${axis_chirho}: dispatched ${index_chirho + 1}/${total_chirho}`);
    }
  }));
  const initial_timeouts_chirho = rows_chirho.filter(row_chirho => row_chirho.timed_out_chirho);
  for (const [index_chirho, row_chirho] of rows_chirho.entries()) {
    if (row_chirho.timed_out_chirho) rows_chirho[index_chirho] = { name_chirho: row_chirho.name_chirho, ...await observe_chirho(binary_chirho, row_chirho.path_chirho, 60000) };
  }
  for (const row_chirho of rows_chirho) {
    if (hash_chirho(await readFile(row_chirho.path_chirho)) !== row_chirho.source_sha256_chirho) throw new Error("Corpus source changed");
  }
  const baseline_text_chirho = git_chirho("show", `${main_chirho}:spec-chirho/ghc-should-${axis_chirho}-measurement-chirho.txt`);
  const baseline_failures_chirho = new Set(baseline_text_chirho.split("\n").map(line_chirho => line_chirho.trim()).filter(line_chirho => line_chirho && !line_chirho.startsWith("#")));
  const failures_chirho = rows_chirho.filter(row_chirho => axis_chirho === "compile" ? row_chirho.rejected_chirho : !row_chirho.rejected_chirho).map(row_chirho => row_chirho.name_chirho);
  const failure_set_chirho = new Set(failures_chirho);
  const invalid_chirho = rows_chirho.filter(row_chirho => !row_chirho.valid_chirho).map(row_chirho => row_chirho.name_chirho);
  all_valid_chirho &&= invalid_chirho.length === 0;
  const summary_chirho = { axis_chirho, total_chirho, matching_verdicts_chirho: total_chirho - failures_chirho.length,
    failed_verdicts_chirho: failures_chirho, initial_timeouts_chirho: initial_timeouts_chirho.map(row_chirho => row_chirho.name_chirho), invalid_chirho,
    main_artifact_sha256_chirho: hash_chirho(baseline_text_chirho),
    gains_against_main_chirho: [...baseline_failures_chirho].filter(name_chirho => !failure_set_chirho.has(name_chirho)).sort(),
    losses_against_main_chirho: failures_chirho.filter(name_chirho => !baseline_failures_chirho.has(name_chirho)).sort() };
  summaries_chirho.push(summary_chirho);
  for (const [name_chirho, data_chirho] of [["observations", rows_chirho], ["initial-timeouts", initial_timeouts_chirho]] as const) {
    await Bun.write(`${output_chirho}/${axis_chirho}-${name_chirho}-chirho.jsonl.gz`, Bun.gzipSync(Buffer.from(data_chirho.map(row_chirho => JSON.stringify(row_chirho)).join("\n") + "\n")));
  }
  await save_chirho();
  console.log(JSON.stringify(summary_chirho));
}
receipt_chirho.after_chirho = hash_chirho(await readFile(binary_chirho));
if (receipt_chirho.after_chirho !== before_chirho) throw new Error("CLI changed during diagnostic");
for (const [path_chirho, sha256_chirho] of Object.entries(gate_chirho.source_hashes_chirho)) {
  if (hash_chirho(await readFile(path_chirho)) !== sha256_chirho) throw new Error(`Source no longer matches gated revision: ${path_chirho}`);
}
receipt_chirho.complete_chirho = all_valid_chirho;
await save_chirho();
if (!all_valid_chirho) process.exitCode = 1;
