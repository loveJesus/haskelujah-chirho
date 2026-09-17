// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
// One diagnostic per axis, not a published gate. Record verdict sets and failures of the instrument separately.
import { readFile, mkdir } from "node:fs/promises";
import { join, resolve } from "node:path";

const [binary_argument_chirho, output_argument_chirho] = process.argv.slice(2);
if (!binary_argument_chirho || !output_argument_chirho) throw new Error("supply explicit CLI and output directory");
const binary_chirho = resolve(binary_argument_chirho);
const output_chirho = resolve(output_argument_chirho);
await mkdir(output_chirho, { recursive: true });
const hash_chirho = (bytes_chirho: string | Uint8Array) => new Bun.CryptoHasher("sha256").update(bytes_chirho).digest("hex");
const command_output_chirho = (command_chirho: string[]) => {
  const result_chirho = Bun.spawnSync(command_chirho);
  if (result_chirho.exitCode !== 0) throw new Error(`instrument command failed: ${command_chirho.join(" ")}`);
  return result_chirho.stdout.toString().trim();
};
const head_chirho = command_output_chirho(["git", "rev-parse", "HEAD"]);
const binary_sha256_chirho = hash_chirho(await readFile(binary_chirho));
const diff_chirho = command_output_chirho(["git", "diff", "--", "crates/"]);
const source_paths_chirho = () => [...new Set([
  ...command_output_chirho(["git", "diff", "--name-only", "--diff-filter=ACMR", "--", "crates/"]).split("\n"),
  ...command_output_chirho(["git", "ls-files", "--others", "--exclude-standard", "--", "crates/"]).split("\n"),
].filter(Boolean))].sort();
const dirty_source_paths_chirho = source_paths_chirho();
const source_hashes_chirho = async () => Object.fromEntries(await Promise.all(dirty_source_paths_chirho.map(async path_chirho => [path_chirho, hash_chirho(await readFile(path_chirho))])));
const dirty_source_hashes_chirho = await source_hashes_chirho();
const deleted_source_paths_chirho = command_output_chirho(["git", "diff", "--name-only", "--diff-filter=D", "--", "crates/"]).split("\n").filter(Boolean);
const metadata_chirho = {
  kind_chirho: "metadata_chirho", recorded_at_chirho: new Date().toISOString(), head_chirho,
  binary_chirho, binary_sha256_chirho, dirty_source_hashes_chirho, deleted_source_paths_chirho, diff_sha256_chirho: hash_chirho(diff_chirho),
  scope_chirho: "One diagnostic pass per axis; no corpus or oracle edits; error[E or panic detector; panics separately recorded; unexpected exits and unresolved timeouts invalidate the run",
};
const summaries_chirho = [];
for (const [axis_chirho, denominator_chirho] of [["compile", 938], ["fail", 767]] as const) {
  const root_chirho = `ghc-tests-chirho/typecheck-chirho/should_${axis_chirho}`;
  const files_chirho = command_output_chirho(["rg", "--files", root_chirho]).split("\n").filter(path_chirho => path_chirho.endsWith(".hs")).sort();
  if (files_chirho.length !== denominator_chirho) throw new Error(`corpus denominator changed: ${axis_chirho} ${files_chirho.length}`);
  const rows_chirho: Awaited<ReturnType<typeof observe_chirho>>[] = [];
  async function observe_chirho(path_chirho: string, cap_ms_chirho: number) {
    const start_chirho = Date.now();
    const source_sha256_chirho = hash_chirho(await readFile(path_chirho));
    const child_chirho = Bun.spawn([binary_chirho, "check", path_chirho], { stdout: "pipe", stderr: "pipe" });
    let timed_out_chirho = false;
    let output_limit_chirho = false;
    const timer_chirho = setTimeout(() => { timed_out_chirho = true; child_chirho.kill("SIGKILL"); }, cap_ms_chirho);
    async function capture_chirho(stream_chirho: ReadableStream<Uint8Array>) {
      let bytes_chirho = 0;
      const chunks_chirho: Uint8Array[] = [];
      for await (const chunk_chirho of stream_chirho) {
        bytes_chirho += chunk_chirho.byteLength;
        if (bytes_chirho > 1048576) { output_limit_chirho = true; child_chirho.kill("SIGKILL"); break; }
        chunks_chirho.push(chunk_chirho);
      }
      return Buffer.concat(chunks_chirho).toString();
    }
    const [exit_chirho, stdout_chirho, stderr_chirho] = await Promise.all([child_chirho.exited, capture_chirho(child_chirho.stdout), capture_chirho(child_chirho.stderr)]);
    clearTimeout(timer_chirho);
    const diagnostic_chirho = stdout_chirho + stderr_chirho;
    // Diagnostic source excerpts can contain the word "panic" without a crash.
    // Keep the published verdict detector separate from actual runtime failure.
    const panic_chirho = /(?:^|\n)(?:thread .* (?:panicked at|has overflowed its stack)|fatal runtime error: stack overflow)/.test(diagnostic_chirho);
    const verdict_chirho = Bun.spawnSync(["/bin/sh", "-c", 'output_chirho=$(/bin/cat); case "$output_chirho" in *"error[E"*|*panic*) exit 1;; *) exit 0;; esac'], { stdin: Buffer.from(diagnostic_chirho) });
    if (verdict_chirho.exitCode !== 0 && verdict_chirho.exitCode !== 1) throw new Error("shell verdict detector failed");
    const rejected_chirho = verdict_chirho.exitCode === 1;
    if (rejected_chirho !== (diagnostic_chirho.includes("error[E") || diagnostic_chirho.includes("panic"))) throw new Error("independent verdict detectors disagree");
    // An emitted diagnostic cannot excuse a later crash or abnormal exit.
    const unexpected_exit_chirho = exit_chirho !== 0 && exit_chirho !== 1;
    return { name_chirho: path_chirho.slice(root_chirho.length + 1), path_chirho, source_sha256_chirho, cap_ms_chirho, exit_chirho, timed_out_chirho, output_limit_chirho, unexpected_exit_chirho, panic_chirho, rejected_chirho, elapsed_ms_chirho: Date.now() - start_chirho, stdout_chirho, stderr_chirho };
  }
  let next_chirho = 0;
  await Promise.all(Array.from({ length: 4 }, async () => {
    while (next_chirho < files_chirho.length) {
      const index_chirho = next_chirho++;
      rows_chirho[index_chirho] = await observe_chirho(files_chirho[index_chirho], 15000);
      if ((index_chirho + 1) % 100 === 0) console.log(`${axis_chirho}: dispatched ${index_chirho + 1}/${files_chirho.length}`);
    }
  }));
  const initial_timeouts_chirho = rows_chirho.filter(row_chirho => row_chirho.timed_out_chirho);
  for (let index_chirho = 0; index_chirho < rows_chirho.length; index_chirho++) {
    if (rows_chirho[index_chirho].timed_out_chirho) rows_chirho[index_chirho] = await observe_chirho(files_chirho[index_chirho], 60000);
  }
  const baseline_chirho = new Set((await Bun.file(`spec-chirho/ghc-should-${axis_chirho}-measurement-chirho.txt`).text()).split("\n").map(line_chirho => line_chirho.trim()).filter(line_chirho => line_chirho && !line_chirho.startsWith("#")));
  const failing_chirho = rows_chirho.filter(row_chirho => axis_chirho === "compile" ? row_chirho.rejected_chirho : !row_chirho.rejected_chirho).map(row_chirho => row_chirho.name_chirho).sort();
  const failing_set_chirho = new Set(failing_chirho);
  const summary_chirho = {
    kind_chirho: "axis_chirho", axis_chirho, denominator_chirho,
    matching_verdict_count_chirho: denominator_chirho - failing_chirho.length,
    baseline_gains_chirho: [...baseline_chirho].filter(name_chirho => !failing_set_chirho.has(name_chirho)).sort(),
    baseline_losses_chirho: failing_chirho.filter(name_chirho => !baseline_chirho.has(name_chirho)),
    failing_verdict_files_chirho: failing_chirho,
    initial_timeouts_chirho: initial_timeouts_chirho.map(row_chirho => row_chirho.name_chirho),
    unresolved_timeouts_chirho: rows_chirho.filter(row_chirho => row_chirho.timed_out_chirho).map(row_chirho => row_chirho.name_chirho),
    unexpected_exits_chirho: rows_chirho.filter(row_chirho => row_chirho.unexpected_exit_chirho || row_chirho.output_limit_chirho).map(row_chirho => row_chirho.name_chirho),
    panics_chirho: rows_chirho.filter(row_chirho => row_chirho.panic_chirho).map(row_chirho => row_chirho.name_chirho),
  };
  summaries_chirho.push(summary_chirho);
  await Bun.write(join(output_chirho, `${axis_chirho}-observations-chirho.jsonl`), [metadata_chirho, ...rows_chirho].map(row_chirho => JSON.stringify(row_chirho)).join("\n") + "\n");
  await Bun.write(join(output_chirho, `${axis_chirho}-initial-timeouts-chirho.jsonl`), initial_timeouts_chirho.map(row_chirho => JSON.stringify(row_chirho)).join("\n"));
  console.log(JSON.stringify({ axis_chirho, matching_verdict_count_chirho: summary_chirho.matching_verdict_count_chirho, gains_chirho: summary_chirho.baseline_gains_chirho.length, losses_chirho: summary_chirho.baseline_losses_chirho.length, timeouts_chirho: summary_chirho.unresolved_timeouts_chirho, unexpected_exits_chirho: summary_chirho.unexpected_exits_chirho, panics_chirho: summary_chirho.panics_chirho }));
}
if (hash_chirho(await readFile(binary_chirho)) !== binary_sha256_chirho || command_output_chirho(["git", "rev-parse", "HEAD"]) !== head_chirho || hash_chirho(command_output_chirho(["git", "diff", "--", "crates/"])) !== metadata_chirho.diff_sha256_chirho) throw new Error("source or binary changed during diagnostic");
if (JSON.stringify(source_paths_chirho()) !== JSON.stringify(dirty_source_paths_chirho) || JSON.stringify(await source_hashes_chirho()) !== JSON.stringify(dirty_source_hashes_chirho)) throw new Error("new or moved source changed during diagnostic");
await Bun.write(join(output_chirho, "diagnostic-chirho.jsonl"), [metadata_chirho, ...summaries_chirho].map(row_chirho => JSON.stringify(row_chirho)).join("\n") + "\n");
if (summaries_chirho.some(row_chirho => row_chirho.unresolved_timeouts_chirho.length || row_chirho.unexpected_exits_chirho.length || row_chirho.panics_chirho.length)) process.exitCode = 1;
