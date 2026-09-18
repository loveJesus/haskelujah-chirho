// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
// Preserve the independent GHC observations and adapt only their field names for replay.
import { join } from "node:path";
const input_chirho = process.argv[2];
if (!input_chirho) throw new Error("supply the independent evidence directory");
const output_chirho = import.meta.dir;
const records_chirho = [];
for (const name_chirho of ["observations-chirho.jsonl", "observations-followup-chirho.jsonl"]) {
  const bytes_chirho = await Bun.file(join(input_chirho, name_chirho)).text();
  await Bun.write(join(output_chirho, "reference-" + name_chirho), bytes_chirho);
  for (const row_chirho of bytes_chirho.trim().split("\n").map(line_chirho => JSON.parse(line_chirho))) {
    for (const [file_chirho, source_chirho] of Object.entries(row_chirho.sources)) {
      if (new Bun.CryptoHasher("sha256").update(source_chirho as string).digest("hex") !== row_chirho.sha256[file_chirho])
        throw new Error("independent source hash mismatch: " + row_chirho.case);
    }
    records_chirho.push({
      name_chirho: row_chirho.case,
      files_chirho: row_chirho.sources,
      root_chirho: row_chirho.command.trim().split(/\s+/).at(-1),
      exit_chirho: row_chirho.exit,
      reference_chirho: "reference-" + name_chirho,
      reference_version_chirho: "GHC 9.14.1, independently measured by HASKELUJAH/claude_chirho",
    });
  }
}
for (const name_chirho of ["probe_chirho.py", "probe_followup_chirho.py"]) {
  await Bun.write(join(output_chirho, name_chirho), await Bun.file(join(input_chirho, name_chirho)).arrayBuffer());
}
await Bun.write(join(output_chirho, "replay-reference-chirho.jsonl"), records_chirho.map(row_chirho => JSON.stringify(row_chirho)).join("\n") + "\n");
function rust_string_chirho(value_chirho: string): string {
  if (value_chirho.includes('"########')) throw new Error("Rust fixture delimiter collision");
  return 'r########"' + value_chirho + '"########';
}
const fixture_rows_chirho = records_chirho.map(row_chirho => {
  const sources_chirho = Object.entries(row_chirho.files_chirho).map(([name_chirho, source_chirho]) =>
    `        (${rust_string_chirho(name_chirho)}, ${rust_string_chirho(source_chirho as string)}),`).join("\n");
  return `    (${row_chirho.exit_chirho === 0}, ${rust_string_chirho(row_chirho.name_chirho)}, &[\n${sources_chirho}\n    ]),`;
});
await Bun.write(join(output_chirho, "fixtures_chirho.rs"),
  "// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n" +
  "// Generated from independent GHC reference bytes; do not edit the source or verdicts here.\n&[\n" + fixture_rows_chirho.join("\n") + "\n]\n");
console.log("Preserved and source-hash-checked " + records_chirho.length + " independent observations");
