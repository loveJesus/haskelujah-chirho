// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
import { resolve } from "node:path";
import { hash_chirho, json_lines_chirho } from "./support_chirho.ts";

const evidence_chirho = resolve(import.meta.dir, "..");
const output_chirho = `${evidence_chirho}/checkpoint-chirho/corpus-chirho`;
const receipt_chirho = await Bun.file(`${output_chirho}/receipt-chirho.json`).json();
if (!receipt_chirho.complete_chirho) throw new Error("Corpus incomplete or invalid");
const archive_chirho = resolve(evidence_chirho, "../../first-corpus-chirho.tar.gz");
const comparisons_chirho = [];
const moved_chirho = [];
// Manual reason audit against the retained candidate output and each own GHC
// stderr. These are evidence annotations, not detector or compiler exceptions.
const reviewed_reasons_chirho: Record<string, string> = {
  "T6001.hs": "ADJACENT: real Int/Integer InstanceSigs defect, but candidate reverses expected/actual, anchors the binding and omits the generality rule. Already observed before GND; not a GND reject gain.",
  "T15712.hs": "PRIOR REASON WRONG: prior term-level method-skolem mismatch did not implement GHC's DerivingVia kind-arity error on GEndo. Current acceptance removes that false rejection; the reference rule remains unimplemented.",
  "T26137.hs": "PRIOR REASON WRONG: prior missing checked-kind-argument metadata was an internal bail-out, not GHC's DerivingVia coercion/role error. Current acceptance removes that false rejection; the reference rule remains unimplemented.",
};
for (const [axis_chirho, total_chirho] of [["compile", 938], ["fail", 767]] as const) {
  const current_bytes_chirho = Bun.gunzipSync(await Bun.file(`${output_chirho}/${axis_chirho}-observations-chirho.jsonl.gz`).bytes());
  const current_chirho = json_lines_chirho(new TextDecoder().decode(current_bytes_chirho));
  if (current_chirho.length !== total_chirho || current_chirho.some(row_chirho => !row_chirho.valid_chirho)) throw new Error("Invalid current inventory");
  const current_map_chirho = new Map(current_chirho.map(row_chirho => [row_chirho.name_chirho, row_chirho]));
  if (current_map_chirho.size !== total_chirho) throw new Error("Duplicate current names");
  for (const baseline_chirho of ["parent", "candidate"]) {
    const extracted_chirho = Bun.spawnSync(["tar", "-xOf", archive_chirho, `${baseline_chirho}-chirho/${axis_chirho}-observations-chirho.jsonl`]);
    if (extracted_chirho.exitCode !== 0) throw new Error("Historical receipt extraction failed");
    const rows_chirho = json_lines_chirho(extracted_chirho.stdout.toString()).filter(row_chirho => row_chirho.name_chirho);
    if (rows_chirho.length !== total_chirho || new Set(rows_chirho.map(row_chirho => row_chirho.name_chirho)).size !== total_chirho) throw new Error("Invalid historical inventory");
    const gains_chirho: string[] = [];
    const losses_chirho: string[] = [];
    for (const prior_chirho of rows_chirho) {
      const current_row_chirho = current_map_chirho.get(prior_chirho.name_chirho);
      if (!current_row_chirho || current_row_chirho.source_sha256_chirho !== prior_chirho.source_sha256_chirho) throw new Error(`Source identity changed: ${prior_chirho.name_chirho}`);
      if (current_row_chirho.rejected_chirho === prior_chirho.rejected_chirho) continue;
      const gained_chirho = axis_chirho === "compile" ? !current_row_chirho.rejected_chirho : current_row_chirho.rejected_chirho;
      (gained_chirho ? gains_chirho : losses_chirho).push(prior_chirho.name_chirho);
      const oracle_chirho = Bun.file(current_row_chirho.path_chirho.replace(/\.hs$/, ".stderr"));
      const oracle_text_chirho = await oracle_chirho.exists() ? await oracle_chirho.text() : null;
      moved_chirho.push({ axis_chirho, baseline_chirho, gained_chirho, name_chirho: prior_chirho.name_chirho,
        prior_stdout_chirho: prior_chirho.stdout_chirho, prior_stderr_chirho: prior_chirho.stderr_chirho,
        current_stdout_chirho: current_row_chirho.stdout_chirho, current_stderr_chirho: current_row_chirho.stderr_chirho,
        ghc_stderr_chirho: oracle_text_chirho,
        reason_audit_chirho: axis_chirho === "fail" && reviewed_reasons_chirho[prior_chirho.name_chirho]
          ? reviewed_reasons_chirho[prior_chirho.name_chirho]
          : "Unreviewed: verdict movement alone gives no reason credit." });
    }
    comparisons_chirho.push({ axis_chirho, baseline_chirho, total_chirho,
      historical_observations_sha256_chirho: hash_chirho(extracted_chirho.stdout), current_observations_sha256_chirho: hash_chirho(current_bytes_chirho),
      gains_chirho: gains_chirho.sort(), losses_chirho: losses_chirho.sort() });
  }
}
await Bun.write(`${output_chirho}/comparison-chirho.json`, JSON.stringify({ comparisons_chirho, main_chirho: receipt_chirho.main_chirho, main_comparison_chirho: receipt_chirho.summaries_chirho }, null, 2) + "\n");
await Bun.write(`${output_chirho}/moved-diagnostics-chirho.jsonl.gz`, Bun.gzipSync(Buffer.from(moved_chirho.map(row_chirho => JSON.stringify(row_chirho)).join("\n") + "\n")));
console.log(JSON.stringify(comparisons_chirho, null, 2));
