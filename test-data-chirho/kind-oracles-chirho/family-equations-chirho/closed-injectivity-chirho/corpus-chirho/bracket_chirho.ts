// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
import { mkdtemp, readFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";

const [main_argument_chirho, candidate_argument_chirho, output_chirho] = process.argv.slice(2);
if (!main_argument_chirho || !candidate_argument_chirho || !output_chirho) throw new Error("main CLI, candidate CLI and output required");
const main_chirho = resolve(main_argument_chirho);
const candidate_chirho = resolve(candidate_argument_chirho);
const scratch_chirho = await mkdtemp(join(tmpdir(), "haskelujah-reason-bracket-chirho-"));
const hash_chirho = async (path_chirho: string) => new Bun.CryptoHasher("sha256").update(await readFile(path_chirho)).digest("hex");
const binary_hashes_chirho = await Promise.all([main_chirho, candidate_chirho].map(hash_chirho));
async function run_chirho(command_chirho: string[]) {
  const child_chirho = Bun.spawn(command_chirho, { stdout: "pipe", stderr: "pipe" });
  let timed_out_chirho = false;
  const timer_chirho = setTimeout(() => { timed_out_chirho = true; child_chirho.kill("SIGKILL"); }, 30000);
  const [exit_chirho, stdout_chirho, stderr_chirho] = await Promise.all([child_chirho.exited, new Response(child_chirho.stdout).text(), new Response(child_chirho.stderr).text()]);
  clearTimeout(timer_chirho);
  return { command_chirho, exit_chirho, stdout_chirho, stderr_chirho, timed_out_chirho };
}
const source_heads_chirho = await Promise.all([main_chirho, candidate_chirho].map(binary_chirho => run_chirho(["git", "-C", dirname(dirname(dirname(binary_chirho))), "rev-parse", "HEAD"])));
const metadata_chirho = { kind_chirho: "metadata_chirho", recorded_at_chirho: new Date().toISOString(), main_chirho, candidate_chirho, source_heads_chirho, binary_hashes_chirho, ghc_chirho: await run_chirho(["ghc", "--numeric-version"]) };
const records_chirho = [];
for (const name_chirho of ["T12102", "T23162c"]) {
  const source_chirho = resolve(`ghc-tests-chirho/typecheck-chirho/should_fail/${name_chirho}.hs`);
  const observations_chirho = [];
  for (const command_chirho of [[main_chirho, "check", source_chirho], [candidate_chirho, "check", source_chirho], ["ghc", "-v0", "-fforce-recomp", "-fno-code", "-outputdir", scratch_chirho, source_chirho]]) observations_chirho.push(await run_chirho(command_chirho));
  records_chirho.push({ kind_chirho: "observation_chirho", name_chirho, source_chirho, source_sha256_chirho: await hash_chirho(source_chirho), observations_chirho });
}
if (JSON.stringify(binary_hashes_chirho) !== JSON.stringify(await Promise.all([main_chirho, candidate_chirho].map(hash_chirho)))) throw new Error("binary changed during bracket");
await Bun.write(output_chirho, [metadata_chirho, ...records_chirho].map(record_chirho => JSON.stringify(record_chirho)).join("\n") + "\n");
console.log(JSON.stringify(records_chirho));
if (records_chirho.some(record_chirho => record_chirho.observations_chirho.some(observation_chirho => observation_chirho.timed_out_chirho || ![0, 1].includes(observation_chirho.exit_chirho)))) process.exitCode = 1;
