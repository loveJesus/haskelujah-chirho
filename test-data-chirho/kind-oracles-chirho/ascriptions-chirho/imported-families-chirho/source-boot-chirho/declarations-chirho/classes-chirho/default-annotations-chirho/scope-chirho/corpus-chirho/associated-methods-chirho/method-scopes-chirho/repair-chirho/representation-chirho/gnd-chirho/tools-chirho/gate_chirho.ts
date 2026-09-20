// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
// Persistent, revision-specific verification; never treats a partial run as complete.
import { chmod, copyFile, mkdir, readFile } from "node:fs/promises";
import { basename, resolve } from "node:path";

const output_chirho = resolve(process.argv[2] ?? "");
if (!process.argv[2] || await Bun.file(`${output_chirho}/receipt-chirho.json`).exists()) {
  throw new Error("Supply a new receipt directory; existing runs are immutable");
}
const hash_chirho = (bytes_chirho: string | Uint8Array) =>
  new Bun.CryptoHasher("sha256").update(bytes_chirho).digest("hex");
const git_chirho = (...args_chirho: string[]) => {
  const process_chirho = Bun.spawnSync(["git", ...args_chirho]);
  if (process_chirho.exitCode !== 0) throw new Error(process_chirho.stderr.toString());
  return process_chirho.stdout.toString().trim();
};
const paths_chirho = () => [...new Set([
  ...git_chirho("diff", "--name-only", "--diff-filter=ACMR", "--", "crates/").split("\n"),
  ...git_chirho("ls-files", "--others", "--exclude-standard", "--", "crates/").split("\n"),
  "Cargo.toml", "Cargo.lock",
].filter(Boolean))].sort();
const source_chirho = async () => Object.fromEntries(await Promise.all(paths_chirho().map(async path_chirho =>
  [path_chirho, hash_chirho(await readFile(path_chirho))])));
const initial_source_chirho = await source_chirho();
const initial_head_chirho = git_chirho("rev-parse", "HEAD");
const initial_diff_chirho = hash_chirho(git_chirho("diff", "--", "crates/"));
const cabal_chirho = ".haskelujah-packages-chirho/constraints-0.14.4/constraints.cabal";
if (!await Bun.file(cabal_chirho).exists()) throw new Error("Live package prerequisite missing");
const cabal_sha256_chirho = hash_chirho(await readFile(cabal_chirho));
await mkdir(output_chirho, { recursive: true });
const records_chirho: Record<string, unknown>[] = [];
const receipt_chirho = {
  started_chirho: new Date().toISOString(),
  worktree_chirho: process.cwd(),
  head_chirho: initial_head_chirho,
  source_hashes_chirho: initial_source_chirho,
  diff_sha256_chirho: initial_diff_chirho,
  cabal_chirho, cabal_sha256_chirho,
  rust_min_stack_chirho: 16777216,
  records_chirho,
  complete_chirho: false,
  binary_path_chirho: "",
  binary_sha256_chirho: "",
};
const save_chirho = () => Bun.write(`${output_chirho}/receipt-chirho.json`, JSON.stringify(receipt_chirho, null, 2) + "\n");
await save_chirho();
const commands_chirho = [
  ["cargo", "fmt", "--all", "--", "--check"],
  ["cargo", "clippy", "-p", "haskelujah-parser", "-p", "haskelujah-naming", "-p", "haskelujah-typing", "-p", "haskelujah-th", "-p", "haskelujah-driver", "--all-targets", "--jobs", "1", "--message-format=json"],
  ["cargo", "test", "-p", "haskelujah-parser", "-p", "haskelujah-naming", "-p", "haskelujah-typing", "-p", "haskelujah-th", "--lib", "--jobs", "1", "--", "--test-threads=2", "--quiet"],
  ["cargo", "test", "-p", "haskelujah-driver", "--test", "typing_integration_chirho", "--test", "canaries_chirho", "--jobs", "1", "--", "--test-threads=2", "--quiet"],
  ["cargo", "test", "-p", "haskelujah-driver", "--lib", "frontend_constraints_package_regression_chirho", "--jobs", "1", "--", "--test-threads=2", "--quiet"],
  ["cargo", "build", "-p", "haskelujah", "--jobs", "1"],
];
for (const [index_chirho, command_chirho] of commands_chirho.entries()) {
  const started_chirho = new Date().toISOString();
  const child_chirho = Bun.spawn(["nice", "-n", "10", ...command_chirho], {
    env: { ...process.env, RUST_MIN_STACK: "16777216" }, stdout: "pipe", stderr: "pipe",
  });
  console.log(JSON.stringify({ started_chirho, pid_chirho: child_chirho.pid, command_chirho }));
  const [exit_chirho, stdout_chirho, stderr_chirho] = await Promise.all([
    child_chirho.exited, new Response(child_chirho.stdout).text(), new Response(child_chirho.stderr).text(),
  ]);
  await Bun.write(`${output_chirho}/gate-${index_chirho}-stdout-chirho.gz`, Bun.gzipSync(Buffer.from(stdout_chirho)));
  await Bun.write(`${output_chirho}/gate-${index_chirho}-stderr-chirho.gz`, Bun.gzipSync(Buffer.from(stderr_chirho)));
  const record_chirho = {
    command_chirho, started_chirho, finished_chirho: new Date().toISOString(), exit_chirho,
    stdout_sha256_chirho: hash_chirho(stdout_chirho), stderr_sha256_chirho: hash_chirho(stderr_chirho),
    summaries_chirho: stdout_chirho.split("\n").filter(line_chirho => line_chirho.startsWith("test result:")),
    compiler_warning_lines_chirho: stderr_chirho.split("\n").filter(line_chirho => /^warning(?:\[|:)/.test(line_chirho)),
  };
  records_chirho.push(record_chirho);
  await save_chirho();
  console.log(JSON.stringify(record_chirho));
  if (exit_chirho !== 0) process.exit(1);
}
if (git_chirho("rev-parse", "HEAD") !== initial_head_chirho
  || JSON.stringify(await source_chirho()) !== JSON.stringify(initial_source_chirho)
  || hash_chirho(git_chirho("diff", "--", "crates/")) !== initial_diff_chirho) {
  throw new Error("Source changed during verification");
}
if (hash_chirho(await readFile(cabal_chirho)) !== cabal_sha256_chirho) throw new Error("Package prerequisite changed");
const binary_directory_chirho = resolve("tmp-chirho/gnd-resume-chirho", basename(output_chirho));
const binary_chirho = resolve(binary_directory_chirho, "haskelujah-frozen-chirho");
if (await Bun.file(binary_chirho).exists()) throw new Error("Frozen CLI already exists");
await mkdir(binary_directory_chirho, { recursive: true });
const built_hash_chirho = hash_chirho(await readFile("target/debug/haskelujah"));
await copyFile("target/debug/haskelujah", binary_chirho);
await chmod(binary_chirho, 0o555);
receipt_chirho.binary_path_chirho = binary_chirho;
receipt_chirho.binary_sha256_chirho = hash_chirho(await readFile(binary_chirho));
if (receipt_chirho.binary_sha256_chirho !== built_hash_chirho) throw new Error("CLI copy changed");
receipt_chirho.complete_chirho = true;
await save_chirho();
console.log(JSON.stringify({ complete_chirho: true, binary_chirho, sha256_chirho: built_hash_chirho }));
