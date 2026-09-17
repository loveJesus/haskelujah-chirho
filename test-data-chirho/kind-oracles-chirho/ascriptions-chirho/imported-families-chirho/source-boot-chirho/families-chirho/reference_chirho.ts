// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Independent closed-family boot references. Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
import { mkdir, mkdtemp } from "node:fs/promises";
import { join } from "node:path";
import { tmpdir } from "node:os";

type CaseChirho = {
  name_chirho: string;
  files_chirho: Record<string, string>;
  root_chirho: string;
  predicted_chirho: boolean;
};

const extensions_chirho = "{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}\n";
const head_chirho = "type family FChirho (aChirho :: Type) :: Type";
const full_chirho = `${head_chirho} where\n  FChirho Int = Bool\n  FChirho aChirho = aChirho\n`;
const alpha_chirho = `${head_chirho} where\n  FChirho Int = Bool\n  FChirho bChirho = bChirho\n`;
const empty_chirho = `${head_chirho} where {}\n`;
const abstract_chirho = `${head_chirho} where ..\n`;
const open_chirho = `${head_chirho}\n`;
const cases_chirho: CaseChirho[] = [];

function cycle_chirho(name_chirho: string, boot_body_chirho: string, body_chirho: string, predicted_chirho: boolean, use_chirho = "valueChirho = ()\n") {
  cases_chirho.push({
    name_chirho, predicted_chirho, root_chirho: "ConsumerChirho.hs",
    files_chirho: {
      "ProviderChirho.hs": `${extensions_chirho}module ProviderChirho where\nimport ConsumerChirho\nimport Data.Kind (Type)\n${body_chirho}`,
      "ProviderChirho.hs-boot": `${extensions_chirho}module ProviderChirho where\nimport Data.Kind (Type)\n${boot_body_chirho}`,
      "ConsumerChirho.hs": `${extensions_chirho}module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\n${use_chirho}`,
    },
  });
}

cycle_chirho("abstract_nonempty_chirho", abstract_chirho, full_chirho, true);
cycle_chirho("abstract_empty_chirho", abstract_chirho, empty_chirho, true);
cycle_chirho("empty_matching_chirho", empty_chirho, empty_chirho, true);
cycle_chirho("full_matching_chirho", full_chirho, full_chirho, true);
cycle_chirho("full_alpha_chirho", alpha_chirho, full_chirho, true);
cycle_chirho("empty_nonempty_chirho", empty_chirho, full_chirho, false);
cycle_chirho("abstract_open_chirho", abstract_chirho, open_chirho, false);
cycle_chirho("open_closed_chirho", open_chirho, full_chirho, false);
cycle_chirho("full_changed_result_chirho", full_chirho.replace("= Bool", "= Char"), full_chirho, false);
cycle_chirho("full_missing_row_chirho", `${head_chirho} where\n  FChirho Int = Bool\n`, full_chirho, false);
cycle_chirho("full_reordered_rows_chirho", `${head_chirho} where\n  FChirho aChirho = aChirho\n  FChirho Int = Bool\n`, full_chirho, false);
const use_chirho = "readBackChirho :: FChirho Int -> Bool\nreadBackChirho xChirho = xChirho\n";
cycle_chirho("full_equation_readback_chirho", full_chirho, full_chirho, true, use_chirho);
cycle_chirho("abstract_equation_unavailable_chirho", abstract_chirho, full_chirho, false, use_chirho);
const injective_head_chirho = "type family FChirho (aChirho :: Type) = (rChirho :: Type) | rChirho -> aChirho";
cycle_chirho("abstract_injective_chirho", `${injective_head_chirho} where ..\n`, `${injective_head_chirho} where\n  FChirho aChirho = [aChirho]\n`, true);
cycle_chirho("abstract_injectivity_mismatch_chirho", `${injective_head_chirho} where ..\n`, full_chirho, false);
cycle_chirho("abstract_invalid_implementation_chirho", abstract_chirho, `${head_chirho} where\n  FChirho aChirho = NoSuchTypeChirho\n`, false);
cases_chirho.push({name_chirho: "abstract_outside_boot_chirho", predicted_chirho: false, root_chirho: "ProviderChirho.hs", files_chirho: {
  "ProviderChirho.hs": `${extensions_chirho}module ProviderChirho where\nimport Data.Kind (Type)\n${abstract_chirho}`,
}});

const directory_chirho = await mkdtemp(join(tmpdir(), "haskelujah-closed-boot-chirho-"));
const output_chirho = process.argv[2];
if (!output_chirho) throw new Error("supply the observations JSONL path");
const version_child_chirho = Bun.spawn(["ghc", "--numeric-version"], { stdout: "pipe", stderr: "pipe" });
const version_chirho = (await new Response(version_child_chirho.stdout).text()).trim();
if (await version_child_chirho.exited !== 0) throw new Error("GHC version failed");
const observations_chirho = [];
for (const case_chirho of cases_chirho) {
  const case_directory_chirho = join(directory_chirho, case_chirho.name_chirho);
  await mkdir(case_directory_chirho);
  for (const [name_chirho, source_chirho] of Object.entries(case_chirho.files_chirho)) {
    await Bun.write(join(case_directory_chirho, name_chirho), source_chirho);
  }
  const command_chirho = ["ghc", "--make", "-fforce-recomp", "-fno-code", "-v0", "-outputdir", "build-chirho", case_chirho.root_chirho];
  const child_chirho = Bun.spawn(command_chirho, { cwd: case_directory_chirho, stdout: "pipe", stderr: "pipe" });
  let timed_out_chirho = false;
  const timeout_chirho = setTimeout(() => { timed_out_chirho = true; child_chirho.kill("SIGKILL"); }, 15000);
  const [exit_chirho, stdout_chirho, stderr_chirho] = await Promise.all([child_chirho.exited, new Response(child_chirho.stdout).text(), new Response(child_chirho.stderr).text()]);
  clearTimeout(timeout_chirho);
  observations_chirho.push({ ...case_chirho, ghc_version_chirho: version_chirho, command_chirho, directory_chirho: case_directory_chirho, exit_chirho, stdout_chirho, stderr_chirho, timed_out_chirho,
    agrees_chirho: !timed_out_chirho && (exit_chirho === 0) === case_chirho.predicted_chirho,
    hashes_chirho: Object.fromEntries(Object.entries(case_chirho.files_chirho).map(([name_chirho, source_chirho]) => [name_chirho, new Bun.CryptoHasher("sha256").update(source_chirho).digest("hex")])),
  });
  console.log(`${case_chirho.name_chirho}: exit=${exit_chirho}, agrees=${observations_chirho.at(-1)?.agrees_chirho}, timeout=${timed_out_chirho}`);
}
await Bun.write(output_chirho, observations_chirho.map(record_chirho => JSON.stringify(record_chirho)).join("\n") + "\n");
console.log(`Evidence: ${output_chirho}; scratch: ${directory_chirho}`);
if (observations_chirho.some(record_chirho => record_chirho.timed_out_chirho)) process.exitCode = 1;
