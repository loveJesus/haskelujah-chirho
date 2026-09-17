// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
// Independent GHC observations: invalid names and set-valued injectivity metadata.
import { mkdtemp, mkdir } from "node:fs/promises";
import { join } from "node:path";
const output_chirho = process.argv[2];
if (!output_chirho) throw new Error("supply an output JSONL path");
const scratch_chirho = await mkdtemp("/private/tmp/haskelujah-injectivity-metadata-chirho.");
const header_chirho = "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures, MultiParamTypeClasses #-}\nmodule BootClassChirho where\nimport Data.Kind (Type)\n";
const aux_chirho = "module AuxChirho where\nimport {-# SOURCE #-} BootClassChirho\n";
const declaration_chirho = (parameters_chirho: string, annotation_chirho: string) => header_chirho + `class () => CChirho ${parameters_chirho.split(" ").map(name_chirho => `(${name_chirho} :: Type)`).join(" ")} where\n  type FChirho ${parameters_chirho} = rChirho | ${annotation_chirho}\n`;
const cases_chirho = [
  ["I1_unknown_parameter_chirho", "aChirho", "rChirho -> missingChirho", null, false],
  ["I2_wrong_result_chirho", "aChirho", "otherChirho -> aChirho", null, false],
  ["I3_duplicate_parameter_chirho", "aChirho", "rChirho -> aChirho aChirho", null, true],
  ["I4_valid_annotation_chirho", "aChirho", "rChirho -> aChirho", null, true],
  ["I5_unknown_boot_and_implementation_chirho", "aChirho", "rChirho -> missingChirho", "rChirho -> missingChirho", false],
  ["I6_wrong_result_boot_and_implementation_chirho", "aChirho", "otherChirho -> aChirho", "otherChirho -> aChirho", false],
  ["I7_duplicate_and_single_chirho", "aChirho", "rChirho -> aChirho aChirho", "rChirho -> aChirho", true],
  ["I8_reordered_dependency_set_chirho", "aChirho bChirho", "rChirho -> aChirho bChirho", "rChirho -> bChirho aChirho", true],
  ["I9_different_dependency_set_chirho", "aChirho bChirho", "rChirho -> aChirho", "rChirho -> bChirho", false],
  ["I10_identical_dependency_set_chirho", "aChirho bChirho", "rChirho -> aChirho bChirho", "rChirho -> aChirho bChirho", true],
] as const;
const version_chirho = Bun.spawnSync(["ghc", "--numeric-version"]);
if (version_chirho.exitCode !== 0) throw new Error("GHC version unavailable");
const ghc_version_chirho = version_chirho.stdout.toString().trim();
const hash_chirho = (source_chirho: string) => new Bun.CryptoHasher("sha256").update(source_chirho).digest("hex");
const records_chirho = [];
for (const [name_chirho, parameters_chirho, boot_annotation_chirho, actual_annotation_chirho, predicted_accept_chirho] of cases_chirho) {
  const directory_chirho = join(scratch_chirho, name_chirho);
  await mkdir(directory_chirho);
  const files_chirho: Record<string, string> = { "BootClassChirho.hs": declaration_chirho(parameters_chirho, actual_annotation_chirho ?? boot_annotation_chirho) };
  if (actual_annotation_chirho !== null) {
    files_chirho["BootClassChirho.hs-boot"] = declaration_chirho(parameters_chirho, boot_annotation_chirho);
    files_chirho["AuxChirho.hs"] = aux_chirho;
  }
  for (const [file_chirho, source_chirho] of Object.entries(files_chirho)) await Bun.write(join(directory_chirho, file_chirho), source_chirho);
  const command_chirho = ["ghc", "--make", "-fforce-recomp", "-fno-code", "-v0", "-outputdir", "build-chirho", "BootClassChirho.hs", ...(actual_annotation_chirho !== null ? ["AuxChirho.hs"] : [])];
  const child_chirho = Bun.spawn(command_chirho, { cwd: directory_chirho, stdout: "pipe", stderr: "pipe" });
  let timed_out_chirho = false;
  const timer_chirho = setTimeout(() => { timed_out_chirho = true; child_chirho.kill("SIGKILL"); }, 60000);
  const [exit_chirho, stdout_chirho, stderr_chirho] = await Promise.all([child_chirho.exited, new Response(child_chirho.stdout).text(), new Response(child_chirho.stderr).text()]);
  clearTimeout(timer_chirho);
  const completed_chirho = !timed_out_chirho && (exit_chirho === 0 || (exit_chirho === 1 && /error: \[GHC-/.test(stderr_chirho)));
  const accepted_chirho = completed_chirho && exit_chirho === 0;
  const record_chirho = { name_chirho, files_chirho, sha256_chirho: Object.fromEntries(Object.entries(files_chirho).map(([name_chirho, source_chirho]) => [name_chirho, hash_chirho(source_chirho)])), command_chirho, directory_chirho, ghc_version_chirho, predicted_accept_chirho, exit_chirho, stdout_chirho, stderr_chirho, timed_out_chirho, completed_chirho, accepted_chirho, agrees_chirho: completed_chirho && accepted_chirho === predicted_accept_chirho };
  records_chirho.push(record_chirho);
  console.log(JSON.stringify({ name_chirho, exit_chirho, agrees_chirho: record_chirho.agrees_chirho }));
}
await Bun.write(output_chirho, records_chirho.map(record_chirho => JSON.stringify(record_chirho)).join("\n") + "\n");
if (records_chirho.some(record_chirho => !record_chirho.completed_chirho)) process.exitCode = 1;
