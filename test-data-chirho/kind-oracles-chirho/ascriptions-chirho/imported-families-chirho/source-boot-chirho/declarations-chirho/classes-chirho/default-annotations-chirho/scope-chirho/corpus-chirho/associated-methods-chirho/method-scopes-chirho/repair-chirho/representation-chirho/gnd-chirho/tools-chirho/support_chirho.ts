// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
import { readFile } from "node:fs/promises";

export const hash_chirho = (bytes_chirho: string | Uint8Array) =>
  new Bun.CryptoHasher("sha256").update(bytes_chirho).digest("hex");
export const git_chirho = (...args_chirho: string[]) => {
  const child_chirho = Bun.spawnSync(["git", ...args_chirho]);
  if (child_chirho.exitCode !== 0) throw new Error(child_chirho.stderr.toString());
  return child_chirho.stdout.toString().trim();
};
export const json_lines_chirho = (text_chirho: string) => text_chirho.trim().split("\n").filter(Boolean).map(line_chirho => JSON.parse(line_chirho));
export const classify_chirho = (exit_chirho: number, output_chirho: string) => {
  const rejected_chirho = output_chirho.includes("error[E");
  const panic_chirho = /(?:^|\n)(?:thread .* (?:panicked at|has overflowed its stack)|fatal runtime error: stack overflow|stack backtrace:)/.test(output_chirho);
  return { rejected_chirho, panic_chirho, unexpected_exit_chirho: exit_chirho !== (rejected_chirho ? 1 : 0) };
};
// Distinguish source text from a runtime header and require exit/diagnostic agreement.
if (classify_chirho(0, 'source = "panic"').rejected_chirho
  || !classify_chirho(1, "").unexpected_exit_chirho
  || !classify_chirho(0, "error[E0200]").unexpected_exit_chirho
  || !classify_chirho(101, "thread 'main' panicked at x.rs:1").panic_chirho) {
  throw new Error("Diagnostic detector controls failed");
}
export async function observe_chirho(binary_chirho: string, path_chirho: string, cap_ms_chirho: number) {
  const source_sha256_chirho = hash_chirho(await readFile(path_chirho));
  const started_chirho = Date.now();
  const command_chirho = [binary_chirho, "check", path_chirho];
  const child_chirho = Bun.spawn(command_chirho, { stdout: "pipe", stderr: "pipe" });
  let timed_out_chirho = false;
  let output_limit_chirho = false;
  const timer_chirho = setTimeout(() => { timed_out_chirho = true; child_chirho.kill("SIGKILL"); }, cap_ms_chirho);
  async function capture_chirho(stream_chirho: ReadableStream<Uint8Array>) {
    let size_chirho = 0;
    const chunks_chirho: Uint8Array[] = [];
    for await (const chunk_chirho of stream_chirho) {
      size_chirho += chunk_chirho.byteLength;
      if (size_chirho > 1048576) { output_limit_chirho = true; child_chirho.kill("SIGKILL"); break; }
      chunks_chirho.push(chunk_chirho);
    }
    return Buffer.concat(chunks_chirho).toString();
  }
  const [exit_chirho, stdout_chirho, stderr_chirho] = await Promise.all([
    child_chirho.exited, capture_chirho(child_chirho.stdout), capture_chirho(child_chirho.stderr),
  ]);
  clearTimeout(timer_chirho);
  if (hash_chirho(await readFile(path_chirho)) !== source_sha256_chirho) throw new Error("Probe changed during execution");
  const classification_chirho = classify_chirho(exit_chirho, stdout_chirho + stderr_chirho);
  return { path_chirho, source_sha256_chirho, command_chirho, cap_ms_chirho, exit_chirho, stdout_chirho, stderr_chirho,
    elapsed_ms_chirho: Date.now() - started_chirho, timed_out_chirho, output_limit_chirho, ...classification_chirho,
    valid_chirho: !timed_out_chirho && !output_limit_chirho && !classification_chirho.panic_chirho && !classification_chirho.unexpected_exit_chirho };
}
