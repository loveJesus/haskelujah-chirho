// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
// Read the exact regression sources; retain predictions separately from measured verdicts.
import { mkdtemp, mkdir } from "node:fs/promises";
import { join } from "node:path";
import { tmpdir } from "node:os";

const output_chirho = process.argv[2];
if (!output_chirho) throw new Error("supply output path");
const rust_chirho = await Bun.file("crates/haskelujah-driver-chirho/tests/typing_integration_chirho/import_contracts_chirho/associated_indices_chirho.rs").text();
const read_source_chirho = (name_chirho: string) => {
  const source_chirho = rust_chirho.match(new RegExp(`const ${name_chirho}: &str = r####"([\\s\\S]*?)"####;`))?.[1];
  if (!source_chirho) throw new Error(`missing ${name_chirho}`);
  return source_chirho;
};
const provider_chirho = read_source_chirho("PROVIDER_CHIRHO");
const consumer_chirho = read_source_chirho("CONSUMER_CHIRHO");
const legacy_commit_chirho = "02e747617de6caa11c9d5a12004460d37097545b";
const legacy_child_chirho = Bun.spawnSync(["git", "show", legacy_commit_chirho + ":crates/haskelujah-driver-chirho/src/tests_chirho/compile_chirho.rs"]);
if (legacy_child_chirho.exitCode !== 0) throw new Error("original fixture unavailable");
const legacy_chirho = legacy_child_chirho.stdout.toString();
const original_provider_chirho = JSON.parse(legacy_chirho.match(/"PrimFamilyProviderChirho.hs",\s*("(?:[^"\\]|\\.)*")/)?.[1] ?? "null") as string;
const original_consumer_chirho = JSON.parse(legacy_chirho.match(/"PrimFamilyConsumerChirho.hs",\s*("(?:[^"\\]|\\.)*")/)?.[1] ?? "null") as string;
if (!original_provider_chirho || !original_consumer_chirho) throw new Error("original test source missing");
const cases_chirho = [
  ["cross_module_chirho", true, provider_chirho, consumer_chirho],
  ["wrong_result_chirho", false, provider_chirho, consumer_chirho.replace("sChirho (BoxChirho sChirho", "Int (BoxChirho Bool")],
  ["wrong_equation_kind_chirho", false, provider_chirho.replace("= sChirho\n", "= Maybe\n"), consumer_chirho],
  ["explicit_type_instance_chirho", true, provider_chirho.replace("TypeFamilies, FlexibleInstances", "TypeFamilies, FlexibleInstances, KindSignatures").replace("module IndexedProviderChirho where", "module IndexedProviderChirho where\nimport Data.Kind (Type)").replace("instance SelectChirho (TaggedChirho sChirho)", "instance SelectChirho (TaggedChirho (sChirho :: Type))"), consumer_chirho],
  ["non_polykinded_data_chirho", true, "{-# LANGUAGE NoPolyKinds #-}\n" + provider_chirho, consumer_chirho],
  ["original_driver_source_chirho", false, original_provider_chirho, original_consumer_chirho],
  ["original_no_polykinds_chirho", true, "{-# LANGUAGE NoPolyKinds #-}\n" + original_provider_chirho, original_consumer_chirho],
  ["explicit_source_and_use_chirho", true,
    provider_chirho.replace("TypeFamilies, FlexibleInstances", "TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds")
      .replace("module IndexedProviderChirho where", "module IndexedProviderChirho where\nimport Data.Kind (Type)")
      .replace("class SelectChirho mChirho", "class SelectChirho (mChirho :: Type -> Type)")
      .replace("instance SelectChirho (TaggedChirho sChirho)", "instance SelectChirho (TaggedChirho (sChirho :: Type))"),
    "{-# LANGUAGE KindSignatures #-}\n" + consumer_chirho.replace("import IndexedProviderChirho", "import IndexedProviderChirho\nimport Data.Kind (Type)").replace("TaggedChirho sChirho (BoxChirho", "TaggedChirho (sChirho :: Type) (BoxChirho")],
  ["original_explicit_declarations_chirho", true,
    original_provider_chirho.replace("{-# LANGUAGE TypeFamilies #-}", "{-# LANGUAGE TypeFamilies, KindSignatures #-}")
      .replace("module PrimFamilyProviderChirho where", "module PrimFamilyProviderChirho where\nimport Data.Kind (Type)")
      .replace("class PrimMonadChirho mChirho", "class PrimMonadChirho (mChirho :: Type -> Type)")
      .replace("data STChirho sChirho aChirho", "data STChirho (sChirho :: Type) aChirho")
      .replace("data BoxChirho sChirho aChirho", "data BoxChirho (sChirho :: Type) (aChirho :: Type)"), original_consumer_chirho],
] as const;
const valid_chirho = cases_chirho.find(case_chirho => case_chirho[0] === "explicit_source_and_use_chirho")!;
const counterparts_chirho = [
  ["explicit_wrong_result_chirho", false, valid_chirho[2], valid_chirho[3].replace("(sChirho :: Type) (BoxChirho sChirho", "Int (BoxChirho Bool")],
  ["explicit_wrong_kind_chirho", false, valid_chirho[2].replace("= sChirho\n", "= Maybe\n"), valid_chirho[3]],
] as const;
const directory_chirho = await mkdtemp(join(tmpdir(), "haskelujah-associated-indices-chirho-"));
const version_chirho = Bun.spawnSync(["ghc", "--numeric-version"]);
if (version_chirho.exitCode !== 0) throw new Error("GHC version failed");
const observations_chirho = [];
for (const [name_chirho, predicted_chirho, source_chirho, use_chirho] of [...cases_chirho, ...counterparts_chirho]) {
  const case_directory_chirho = join(directory_chirho, name_chirho);
  await mkdir(case_directory_chirho);
  const provider_name_chirho = source_chirho.match(/module (\w+) where/)?.[1];
  const consumer_name_chirho = use_chirho.match(/module (\w+) where/)?.[1];
  if (!provider_name_chirho || !consumer_name_chirho) throw new Error("module name missing");
  const root_chirho = consumer_name_chirho + ".hs";
  const files_chirho = { [provider_name_chirho + ".hs"]: source_chirho, [root_chirho]: use_chirho };
  for (const [name_chirho, source_chirho] of Object.entries(files_chirho)) await Bun.write(join(case_directory_chirho, name_chirho), source_chirho);
  const command_chirho = ["ghc", "--make", "-fforce-recomp", "-fno-code", "-v0", "-outputdir", "build-chirho", root_chirho];
  const child_chirho = Bun.spawn(command_chirho, { cwd: case_directory_chirho, stdout: "pipe", stderr: "pipe" });
  let timed_out_chirho = false;
  const timer_chirho = setTimeout(() => { timed_out_chirho = true; child_chirho.kill("SIGKILL"); }, 15000);
  const [exit_chirho, stdout_chirho, stderr_chirho] = await Promise.all([child_chirho.exited, new Response(child_chirho.stdout).text(), new Response(child_chirho.stderr).text()]);
  clearTimeout(timer_chirho);
  observations_chirho.push({ name_chirho, predicted_chirho, root_chirho, files_chirho, source_sha256_chirho: Object.fromEntries(Object.entries(files_chirho).map(([name_chirho, source_chirho]) => [name_chirho, new Bun.CryptoHasher("sha256").update(source_chirho).digest("hex")])), ghc_version_chirho: version_chirho.stdout.toString().trim(), command_chirho, exit_chirho, stdout_chirho, stderr_chirho, timed_out_chirho, agrees_chirho: !timed_out_chirho && (exit_chirho === 0) === predicted_chirho });
  console.log(`${name_chirho}: exit=${exit_chirho}, prediction=${predicted_chirho}`);
}
await Bun.write(output_chirho, observations_chirho.map(row_chirho => JSON.stringify(row_chirho)).join("\n") + "\n");
if (observations_chirho.some(row_chirho => row_chirho.timed_out_chirho)) process.exitCode = 1;
