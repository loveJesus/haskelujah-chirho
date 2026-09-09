// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Workflow: spec-chirho/workflows-chirho/testing-chirho/execution-oracles-chirho.md.
import { createHash as createHashChirho } from "node:crypto";
import { mkdtempSync as mkdtempSyncChirho, readdirSync as readdirSyncChirho } from "node:fs";
import { tmpdir as tmpdirChirho } from "node:os";
import { join as joinChirho, resolve as resolveChirho } from "node:path";
import { spawnSync as spawnSyncChirho } from "node:child_process";

interface ExecutionRecordChirho {
  record_chirho: string;
  source_chirho: string;
  source_sha256_chirho: string;
  expected_chirho: string;
}

const rootChirho = resolveChirho(import.meta.dir, "../..");
const manifestChirho = (await Bun.file(joinChirho(import.meta.dir, "ghc-9.14.1-chirho.jsonl")).text())
  .trim().split("\n").map(lineChirho => JSON.parse(lineChirho));
const metadataChirho = manifestChirho.shift();
const recordsChirho: ExecutionRecordChirho[] = manifestChirho;
const ghcChirho = Bun.which("ghc");
const runghcChirho = Bun.which("runghc");
const timeoutChirho = Bun.which("timeout");
if (!ghcChirho || !runghcChirho || !timeoutChirho) throw new Error("GHC, runghc and GNU timeout are required");
const versionChirho = spawnSyncChirho(ghcChirho, ["--numeric-version"], { encoding: "utf8" });
if (versionChirho.status !== 0 || versionChirho.stdout.trim() !== metadataChirho.ghc_version_chirho) {
  throw new Error("reference GHC version differs from the manifest");
}
const declaredChirho: string[] = [];
for (const nameChirho of readdirSyncChirho(joinChirho(rootChirho, "ghc-tests-chirho"))) {
  if (!nameChirho.endsWith(".hs")) continue;
  const relativeChirho = "ghc-tests-chirho/" + nameChirho;
  if (/^\s*-- TEST:\s*compile_and_run\s*$/m.test(await Bun.file(joinChirho(rootChirho, relativeChirho)).text())) {
    declaredChirho.push(relativeChirho);
  }
}
if (JSON.stringify(declaredChirho.sort()) !== JSON.stringify(recordsChirho.map(recordChirho => recordChirho.source_chirho).sort())
  || recordsChirho.length !== metadataChirho.execution_oracles_chirho) {
  throw new Error("manifest differs from the declared execution inventory");
}
const scratchChirho = mkdtempSyncChirho(joinChirho(tmpdirChirho(), "haskelujah-reference-chirho."));
let failuresChirho = 0;
for (const recordChirho of recordsChirho) {
  const sourceChirho = joinChirho(rootChirho, recordChirho.source_chirho);
  const bytesChirho = await Bun.file(sourceChirho).arrayBuffer();
  if (createHashChirho("sha256").update(new Uint8Array(bytesChirho)).digest("hex") !== recordChirho.source_sha256_chirho) {
    throw new Error("source differs from reference measurement: " + recordChirho.source_chirho);
  }
  console.log(JSON.stringify({ event_chirho: "BEGIN", source_chirho: recordChirho.source_chirho }));
  const resultChirho = spawnSyncChirho(timeoutChirho, ["--kill-after=2s", "15s", runghcChirho, "-v0", sourceChirho], {
    cwd: scratchChirho, encoding: "utf8", input: "", timeout: 20000, maxBuffer: 1024 * 1024,
  });
  const matchesChirho = resultChirho.status === 0 && !resultChirho.error
    && resultChirho.stdout.replace(/\n+$/, "") === recordChirho.expected_chirho.replace(/\n+$/, "");
  if (!matchesChirho) failuresChirho += 1;
  console.log(JSON.stringify({ event_chirho: "END", source_chirho: recordChirho.source_chirho,
    matches_chirho: matchesChirho, status_chirho: resultChirho.status,
    signal_chirho: resultChirho.signal, stderr_chirho: resultChirho.stderr,
    error_chirho: resultChirho.error?.message }));
}
console.log(JSON.stringify({ event_chirho: "COMPLETE", total_chirho: recordsChirho.length, failures_chirho: failuresChirho }));
if (failuresChirho !== 0) process.exitCode = 1;
