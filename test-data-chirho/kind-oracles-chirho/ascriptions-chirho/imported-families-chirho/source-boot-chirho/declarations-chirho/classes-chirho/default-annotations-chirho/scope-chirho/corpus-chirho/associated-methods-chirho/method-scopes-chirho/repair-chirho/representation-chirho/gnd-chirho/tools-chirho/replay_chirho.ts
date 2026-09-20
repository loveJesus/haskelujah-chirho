// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
import { mkdir, readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { hash_chirho, json_lines_chirho, observe_chirho } from "./support_chirho.ts";

const evidence_chirho = resolve(import.meta.dir, "..");
const gate_directory_chirho = process.argv[2] ?? "gates-chirho";
const receipt_chirho = await Bun.file(`${evidence_chirho}/checkpoint-chirho/${gate_directory_chirho}/receipt-chirho.json`).json();
if (!receipt_chirho.complete_chirho) throw new Error("Gates incomplete");
const binary_chirho: string = receipt_chirho.binary_path_chirho;
const before_chirho = hash_chirho(await readFile(binary_chirho));
if (before_chirho !== receipt_chirho.binary_sha256_chirho) throw new Error("CLI provenance mismatch");
const cases_chirho: { name_chirho: string; entry_chirho: string; expected_chirho: boolean; sources_chirho: { path_chirho: string; source_chirho: string; sha256_chirho: string }[] }[] = [];
const original_chirho = json_lines_chirho(await Bun.file(`${evidence_chirho}/../gnd-next-brick-chirho.jsonl`).text());
for (const row_chirho of original_chirho.filter(row_chirho => row_chirho.source_chirho)) {
  cases_chirho.push({ name_chirho: row_chirho.name_chirho, entry_chirho: "ProbeChirho.hs", expected_chirho: row_chirho.reference_verdict_chirho === "ACCEPT",
    sources_chirho: [{ path_chirho: "ProbeChirho.hs", source_chirho: row_chirho.source_chirho, sha256_chirho: row_chirho.source_hash_chirho }] });
}
for (const row_chirho of json_lines_chirho(await Bun.file(`${evidence_chirho}/reference-probes-chirho.jsonl`).text())) {
  cases_chirho.push({ name_chirho: row_chirho.case_chirho, entry_chirho: row_chirho.case_chirho, expected_chirho: row_chirho.reference_exit_chirho === 0, sources_chirho: row_chirho.sources_chirho });
}
for (const file_chirho of ["stock-reference-probes-chirho.jsonl", "monad-reference-probes-chirho.jsonl"]) {
  for (const row_chirho of json_lines_chirho(await Bun.file(`${evidence_chirho}/${file_chirho}`).text()).filter(row_chirho => row_chirho.source_chirho)) {
    cases_chirho.push({ name_chirho: row_chirho.probe_chirho ?? row_chirho.name_chirho, entry_chirho: "ProbeChirho.hs", expected_chirho: (row_chirho.reference_exit_chirho ?? row_chirho.exit_code_chirho) === 0,
      sources_chirho: [{ path_chirho: "ProbeChirho.hs", source_chirho: row_chirho.source_chirho, sha256_chirho: row_chirho.source_sha256_chirho }] });
  }
}
if (cases_chirho.length !== 26) throw new Error(`Unexpected reference inventory: ${cases_chirho.length}`);
const output_chirho = `${evidence_chirho}/checkpoint-chirho/${gate_directory_chirho === "gates-chirho" ? "replay" : "final-replay"}-chirho.jsonl`;
if (await Bun.file(output_chirho).exists()) throw new Error("Replay output exists");
const records_chirho: Record<string, unknown>[] = [];
const save_chirho = () => Bun.write(output_chirho, records_chirho.map(record_chirho => JSON.stringify(record_chirho)).join("\n") + "\n");
records_chirho.push({ kind_chirho: "start_chirho", started_chirho: new Date().toISOString(), binary_chirho, before_chirho });
await save_chirho();
for (const [index_chirho, case_chirho] of cases_chirho.entries()) {
  const root_chirho = resolve(`tmp-chirho/gnd-resume-chirho/cases-chirho/${index_chirho}`);
  for (const source_chirho of case_chirho.sources_chirho) {
    if (hash_chirho(source_chirho.source_chirho) !== source_chirho.sha256_chirho) throw new Error(`Retained hash mismatch: ${case_chirho.name_chirho}`);
    const path_chirho = resolve(root_chirho, source_chirho.path_chirho);
    if (!path_chirho.startsWith(root_chirho + "/")) throw new Error("Source outside case root");
    await mkdir(dirname(path_chirho), { recursive: true });
    await Bun.write(path_chirho, source_chirho.source_chirho);
  }
  const observed_chirho = await observe_chirho(binary_chirho, resolve(root_chirho, case_chirho.entry_chirho), 60000);
  const agrees_chirho = observed_chirho.valid_chirho && !observed_chirho.rejected_chirho === case_chirho.expected_chirho;
  records_chirho.push({ name_chirho: case_chirho.name_chirho, expected_accept_chirho: case_chirho.expected_chirho, ...observed_chirho, agrees_chirho });
  await save_chirho();
  console.log(JSON.stringify({ name_chirho: case_chirho.name_chirho, agrees_chirho }));
  if (!agrees_chirho) throw new Error(`Replay disagreement: ${case_chirho.name_chirho}`);
}
for (const name_chirho of ["T11552", "T13142", "T21323", "T5676", "T14010", "T18129", "T3955", "T15839a", "T15839b", "T12734"]) {
  const observed_chirho = await observe_chirho(binary_chirho, resolve(`ghc-tests-chirho/typecheck-chirho/should_compile/${name_chirho}.hs`), 60000);
  const agrees_chirho = observed_chirho.valid_chirho && !observed_chirho.rejected_chirho;
  records_chirho.push({ name_chirho, expected_accept_chirho: true, ...observed_chirho, agrees_chirho });
  await save_chirho();
  console.log(JSON.stringify({ name_chirho, agrees_chirho }));
  if (!agrees_chirho) throw new Error(`Corpus recovery failed: ${name_chirho}`);
}
const after_chirho = hash_chirho(await readFile(binary_chirho));
if (before_chirho !== after_chirho) throw new Error("CLI changed during replay");
records_chirho.push({ kind_chirho: "complete_chirho", finished_chirho: new Date().toISOString(), after_chirho, matching_cases_chirho: 36 });
await save_chirho();
