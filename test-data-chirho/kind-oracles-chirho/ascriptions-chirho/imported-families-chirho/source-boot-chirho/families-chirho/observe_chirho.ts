// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
// Replay reference source bytes through a freshly built CLI; timeout is not rejection.
import { mkdir, mkdtemp, readFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { tmpdir } from "node:os";
const [binary_argument_chirho, output_chirho, ...references_chirho] = process.argv.slice(2);
if (!binary_argument_chirho || !output_chirho || !references_chirho.length) throw new Error("supply binary, output JSONL and reference JSONL paths");
const binary_chirho = resolve(binary_argument_chirho);
const hash_chirho = (bytes_chirho: string | Uint8Array) => new Bun.CryptoHasher("sha256").update(bytes_chirho).digest("hex");
const binary_sha256_chirho = hash_chirho(await readFile(binary_chirho));
const scratch_chirho = await mkdtemp(join(tmpdir(), "haskelujah-boot-observations-chirho-"));
const records_chirho = [];
for (const reference_chirho of references_chirho) {
  const rows_chirho = (await Bun.file(reference_chirho).text()).trim().split("\n").map(line_chirho => JSON.parse(line_chirho));
  for (const row_chirho of rows_chirho) {
    const directory_chirho = join(scratch_chirho, row_chirho.name_chirho);
    await mkdir(directory_chirho);
    const files_chirho = row_chirho.files_chirho as Record<string, string>;
    for (const [name_chirho, source_chirho] of Object.entries(files_chirho)) await Bun.write(join(directory_chirho, name_chirho), source_chirho);
    const root_chirho = row_chirho.root_chirho ?? row_chirho.command_chirho.at(-1);
    const command_chirho = [binary_chirho, "check", join(directory_chirho, root_chirho)];
    const start_chirho = Date.now();
    const child_chirho = Bun.spawn(command_chirho, { cwd: directory_chirho, stdout: "pipe", stderr: "pipe" });
    let timed_out_chirho = false;
    const timer_chirho = setTimeout(() => { timed_out_chirho = true; child_chirho.kill("SIGKILL"); }, 15000);
    const [exit_chirho, stdout_chirho, stderr_chirho] = await Promise.all([child_chirho.exited, new Response(child_chirho.stdout).text(), new Response(child_chirho.stderr).text()]);
    clearTimeout(timer_chirho);
    const diagnostic_chirho = /error\[E|Error checking|error:/.test(stdout_chirho + stderr_chirho);
    const accepted_chirho = !timed_out_chirho && exit_chirho === 0 && !diagnostic_chirho;
    const completed_chirho = !timed_out_chirho && (exit_chirho === 0 || (exit_chirho === 1 && diagnostic_chirho)) && !/panicked|stack overflow/i.test(stdout_chirho + stderr_chirho);
    const agrees_chirho = completed_chirho && accepted_chirho === (row_chirho.exit_chirho === 0);
    records_chirho.push({ name_chirho: row_chirho.name_chirho, reference_chirho, command_chirho, binary_sha256_chirho, source_hashes_chirho: Object.fromEntries(Object.entries(files_chirho).map(([name_chirho, source_chirho]) => [name_chirho, hash_chirho(source_chirho)])), exit_chirho, stdout_chirho, stderr_chirho, timed_out_chirho, elapsed_ms_chirho: Date.now() - start_chirho, completed_chirho, accepted_chirho, agrees_chirho });
    console.log(`${row_chirho.name_chirho}: accepted=${accepted_chirho}, agrees=${agrees_chirho}, timeout=${timed_out_chirho}`);
  }
}
if (hash_chirho(await readFile(binary_chirho)) !== binary_sha256_chirho) throw new Error("CLI changed during observations");
await Bun.write(output_chirho, records_chirho.map(record_chirho => JSON.stringify(record_chirho)).join("\n") + "\n");
if (records_chirho.some(record_chirho => !record_chirho.agrees_chirho)) process.exitCode = 1;
