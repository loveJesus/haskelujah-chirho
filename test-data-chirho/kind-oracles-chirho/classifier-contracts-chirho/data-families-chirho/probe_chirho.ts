// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Retain exact reference/candidate observations for nominal family patterns.
import { spawnSync as spawnSyncChirho } from "node:child_process";
import { createHash as createHashChirho } from "node:crypto";
import { existsSync as existsSyncChirho, readFileSync as readFileSyncChirho, writeFileSync as writeFileSyncChirho } from "node:fs";
import { resolve as resolveChirho } from "node:path";

const directory_chirho = import.meta.dir;
const cases_chirho = [
  ["LocalChirho.hs", true],
  ["ImportedChirho.hs", true],
  ["ShadowChirho.hs", true],
  ["AssociatedChirho.hs", true],
  ["TypePatternChirho.hs", false],
  ["ImportedTypePatternChirho.hs", false],
  ["AssociatedTypePatternChirho.hs", false],
] as const;
const binary_chirho = process.argv[2] ? resolveChirho(process.argv[2]) : "/opt/homebrew/bin/ghc-9.14.1";
const candidate_chirho = process.argv[2] !== undefined;
const output_chirho = process.argv[3] ?? resolveChirho(directory_chirho, candidate_chirho ? "candidate-chirho.json" : "reference-chirho.json");
if (existsSyncChirho(output_chirho)) throw new Error("Receipts are immutable; supply a new output path");
const digest_chirho = (path_chirho: string) => createHashChirho("sha256").update(readFileSyncChirho(path_chirho)).digest("hex");
const before_chirho = digest_chirho(binary_chirho);
const rows_chirho = [];
for (const [file_chirho, expected_chirho] of cases_chirho) {
  const source_chirho = resolveChirho(directory_chirho, file_chirho);
  const commands_chirho = candidate_chirho
    ? [[binary_chirho, "check", source_chirho], ...(expected_chirho ? [[binary_chirho, "run", source_chirho]] : [])]
    : [[binary_chirho, "-v0", "-fforce-recomp", "-fno-code", "-fno-write-interface", `-i${directory_chirho}`, source_chirho],
        ...(expected_chirho ? [[binary_chirho, "-v0", "-i" + directory_chirho, source_chirho, "-e", "main"]] : [])];
  for (const [index_chirho, command_chirho] of commands_chirho.entries()) {
    const result_chirho = spawnSyncChirho(command_chirho[0]!, command_chirho.slice(1), { cwd: directory_chirho, encoding: "utf8", timeout: 60000, maxBuffer: 4 * 1024 * 1024 });
    const expected_exit_chirho = expected_chirho ? 0 : 1;
    rows_chirho.push({ file_chirho, command_chirho, expected_exit_chirho,
      expected_stdout_chirho: index_chirho === 1 ? "7\n" : null,
      exit_chirho: result_chirho.status, signal_chirho: result_chirho.signal,
      error_chirho: result_chirho.error?.message ?? null,
      stdout_chirho: result_chirho.stdout, stderr_chirho: result_chirho.stderr,
      source_sha256_chirho: digest_chirho(source_chirho),
      provider_sha256_chirho: digest_chirho(resolveChirho(directory_chirho, "ProviderChirho.hs")),
      matches_chirho: result_chirho.status === expected_exit_chirho && !result_chirho.error && (index_chirho !== 1 || result_chirho.stdout === "7\n") });
  }
}
const after_chirho = digest_chirho(binary_chirho);
const receipt_chirho = { measured_at_chirho: new Date().toISOString(), probe_sha256_chirho: digest_chirho(import.meta.path), binary_chirho, before_chirho, after_chirho, stable_chirho: before_chirho === after_chirho, rows_chirho };
writeFileSyncChirho(output_chirho, JSON.stringify(receipt_chirho, null, 2) + "\n");
console.log(JSON.stringify({ output_chirho, stable_chirho: receipt_chirho.stable_chirho, cases_chirho: rows_chirho.map(row_chirho => ({file_chirho: row_chirho.file_chirho, exit_chirho: row_chirho.exit_chirho, matches_chirho: row_chirho.matches_chirho, error_chirho: row_chirho.stderr_chirho})) }));
if (!receipt_chirho.stable_chirho || rows_chirho.some(row_chirho => !row_chirho.matches_chirho)) process.exitCode = 1;
