// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
// Record bounded final gates and exact dirty/new source provenance; not a public measurement.
import { readFile } from "node:fs/promises";
const output_chirho = process.argv[2];
if (!output_chirho) throw new Error("supply a result path");
const hash_chirho = (bytes_chirho: string | Uint8Array) => new Bun.CryptoHasher("sha256").update(bytes_chirho).digest("hex");
const git_chirho = (...arguments_chirho: string[]) => {
  const child_chirho = Bun.spawnSync(["git", ...arguments_chirho]);
  if (child_chirho.exitCode !== 0) throw new Error("git provenance failed");
  return child_chirho.stdout.toString().trim();
};
const head_chirho = git_chirho("rev-parse", "HEAD");
const source_paths_chirho = () => [...new Set([
  ...git_chirho("diff", "--name-only", "--diff-filter=ACMR", "--", "crates/").split("\n"),
  ...git_chirho("ls-files", "--others", "--exclude-standard", "--", "crates/").split("\n"),
].filter(Boolean))].sort();
const paths_chirho = source_paths_chirho();
const hashes_chirho = async () => Object.fromEntries(await Promise.all(paths_chirho.map(async path_chirho => [path_chirho, hash_chirho(await readFile(path_chirho))])));
const code_hashes_chirho = await hashes_chirho();
const diff_sha256_chirho = hash_chirho(git_chirho("diff", "--", "crates/"));
const deleted_source_paths_chirho = git_chirho("diff", "--name-only", "--diff-filter=D", "--", "crates/").split("\n").filter(Boolean);
const records_chirho = [];
const commands_chirho = [
  ["cargo", "fmt", "--all", "--", "--check"],
  ["cargo", "check", "--workspace", "--all-targets", "-j", "3"],
  ["cargo", "test", "-p", "haskelujah-parser", "-p", "haskelujah-naming", "-p", "haskelujah-typing", "--lib", "-j", "3", "--", "--quiet"],
  ["cargo", "test", "-p", "haskelujah-driver", "--test", "typing_integration_chirho", "--test", "canaries_chirho", "-j", "3", "--", "--quiet"],
  ["cargo", "test", "-p", "haskelujah-driver", "--lib", "-j", "3", "--", "--test-threads=12", "--quiet"],
  ["cargo", "build", "-p", "haskelujah", "-j", "3"],
  ["cargo", "clippy", "-p", "haskelujah-parser", "-p", "haskelujah-naming", "-p", "haskelujah-typing", "-p", "haskelujah-driver", "--all-targets", "-j", "3"],
];
for (const command_chirho of commands_chirho) {
  const start_chirho = new Date().toISOString();
  console.log("BEGIN " + command_chirho.join(" "));
  const child_chirho = Bun.spawn(command_chirho, { env: { ...process.env, RUST_MIN_STACK: "16777216" }, stdout: "pipe", stderr: "pipe" });
  let timed_out_chirho = false;
  const cap_ms_chirho = command_chirho.includes("haskelujah-driver") && command_chirho.includes("--lib") ? 1800000 : 900000;
  const timer_chirho = setTimeout(() => { timed_out_chirho = true; child_chirho.kill("SIGKILL"); }, cap_ms_chirho);
  const [exit_chirho, stdout_chirho, stderr_chirho] = await Promise.all([child_chirho.exited, new Response(child_chirho.stdout).text(), new Response(child_chirho.stderr).text()]);
  clearTimeout(timer_chirho);
  records_chirho.push({ command_chirho, start_chirho, end_chirho: new Date().toISOString(), cap_ms_chirho, exit_chirho, timed_out_chirho, stdout_chirho, stderr_chirho });
  console.log("END " + JSON.stringify({ command_chirho, exit_chirho, timed_out_chirho, summaries_chirho: stdout_chirho.split("\n").filter(line_chirho => line_chirho.startsWith("test result:")) }));
  await Bun.write(output_chirho, JSON.stringify({ head_chirho, rust_min_stack_chirho: 16777216, code_hashes_chirho, deleted_source_paths_chirho, diff_sha256_chirho, records_chirho, complete_chirho: false }, null, 2) + "\n");
  if (exit_chirho !== 0 || timed_out_chirho) process.exit(1);
}
if (git_chirho("rev-parse", "HEAD") !== head_chirho || JSON.stringify(source_paths_chirho()) !== JSON.stringify(paths_chirho) || JSON.stringify(await hashes_chirho()) !== JSON.stringify(code_hashes_chirho)) throw new Error("source changed during gates");
if (hash_chirho(git_chirho("diff", "--", "crates/")) !== diff_sha256_chirho) throw new Error("source diff changed during gates");
await Bun.write(output_chirho, JSON.stringify({ head_chirho, rust_min_stack_chirho: 16777216, code_hashes_chirho, deleted_source_paths_chirho, diff_sha256_chirho, records_chirho, complete_chirho: true }, null, 2) + "\n");
console.log("SOURCE HASHES STABLE");
