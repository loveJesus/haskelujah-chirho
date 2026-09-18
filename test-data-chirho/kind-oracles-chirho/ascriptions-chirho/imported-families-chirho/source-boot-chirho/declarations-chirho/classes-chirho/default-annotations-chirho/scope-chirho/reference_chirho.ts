// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
// Independent default-scope references; workflow: language-features-chirho/declaration-kinds-chirho.
import { mkdir, mkdtemp } from "node:fs/promises";
import { join } from "node:path";
import { tmpdir } from "node:os";

const output_chirho = process.argv[2];
if (!output_chirho) throw new Error("supply output JSONL path");
const prefix_chirho = `-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
`;
const cases_chirho = [
  ["equation_kind_rigid_chirho", false, `class CChirho (aChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho (aChirho :: kChirho) = Int
`],
  ["equation_kind_rhs_rigid_chirho", false, `class CChirho (aChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho (aChirho :: kChirho) = Proxy (aChirho :: kChirho)
`],
  ["hidden_kind_specialization_chirho", false, `class CChirho (aChirho :: kChirho) where
  type FChirho aChirho :: Type
  type FChirho (aChirho :: Type) = Int
`],
  ["class_parameter_rhs_chirho", false, `class CChirho (aChirho :: Type) (bChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho xChirho = bChirho
`],
  ["equation_shadows_class_chirho", true, `class CChirho (aChirho :: Type) (bChirho :: Type -> Type) where
  type FChirho aChirho :: Type
  type FChirho (bChirho :: Type) = bChirho
instance CChirho Bool Maybe
useChirho :: FChirho Bool
useChirho = True
`],
  ["polykind_renamed_equation_chirho", true, `class CChirho (aChirho :: kChirho) where
  type FChirho (aChirho :: kChirho) :: Type
  type FChirho (bChirho :: jChirho) = Proxy bChirho
instance CChirho Maybe
useChirho :: FChirho Maybe -> Proxy Maybe
useChirho xChirho = xChirho
`],
  ["implicit_lhs_kind_chirho", true, `class CChirho (aChirho :: kChirho) where
  type FChirho aChirho :: Type
  type FChirho aChirho = Proxy (aChirho :: kChirho)
instance CChirho Maybe
useChirho :: FChirho Maybe -> Proxy Maybe
useChirho xChirho = xChirho
`],
  ["class_parameter_as_implicit_kind_chirho", true, `class CChirho (aChirho :: Type) where
  type FChirho (xChirho :: aChirho) :: [aChirho]
  type FChirho xChirho = ('[] :: [aChirho])
`],
  ["result_kind_implicit_chirho", true, `class CChirho (aChirho :: Type) where
  type FChirho aChirho :: [kChirho]
  type FChirho aChirho = ('[] :: [jChirho])
`],
  ["rhs_only_kind_unbound_chirho", false, `class CChirho (aChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho aChirho = Proxy ('[] :: [jChirho])
`],
  ["inferred_class_default_chirho", true, `class CChirho aChirho where
  type FChirho aChirho :: Type
  type FChirho aChirho = Maybe aChirho
instance CChirho Int
useChirho :: FChirho Int
useChirho = Just 3
`],
  ["method_fixes_class_kind_chirho", true, `class CChirho aChirho where
  type FChirho aChirho :: Type
  type FChirho aChirho = Maybe aChirho
  methodChirho :: aChirho -> aChirho
`],
  ["repeated_hidden_kind_chirho", false, `class CChirho (aChirho :: jChirho) (bChirho :: kChirho) where
  type FChirho aChirho bChirho :: Type
  type FChirho (aChirho :: zChirho) (bChirho :: zChirho) = Proxy aChirho
`],
  ["distinct_hidden_kinds_chirho", true, `class CChirho (aChirho :: jChirho) (bChirho :: kChirho) where
  type FChirho aChirho bChirho :: Type
  type FChirho (aChirho :: zChirho) (bChirho :: wChirho) = (Proxy aChirho, Proxy bChirho)
`],
  ["shared_hidden_kind_chirho", true, `class CChirho (aChirho :: kChirho) (bChirho :: kChirho) where
  type FChirho aChirho bChirho :: Type
  type FChirho (aChirho :: zChirho) (bChirho :: zChirho) = (Proxy aChirho, Proxy bChirho)
`],
  ["implicit_result_default_chirho", true, `class CChirho (aChirho :: Type) where
  type FChirho aChirho
  type FChirho aChirho = Maybe aChirho
instance CChirho Int
useChirho :: FChirho Int
useChirho = Just 3
`],
  ["method_fixes_result_kind_chirho", true, `class CChirho (aChirho :: Type) where
  type FChirho aChirho
  type FChirho aChirho = Maybe
  methodChirho :: FChirho aChirho Int -> aChirho
`],
] as const;
const directory_chirho = await mkdtemp(join(tmpdir(), "haskelujah-default-scope-chirho-"));
const version_child_chirho = Bun.spawnSync(["ghc", "--numeric-version"]);
if (version_child_chirho.exitCode !== 0) throw new Error("GHC version failed");
const observations_chirho = [];
for (const [name_chirho, predicted_chirho, body_chirho] of cases_chirho) {
  const case_directory_chirho = join(directory_chirho, name_chirho);
  await mkdir(case_directory_chirho);
  const source_chirho = prefix_chirho + body_chirho;
  await Bun.write(join(case_directory_chirho, "ProbeChirho.hs"), source_chirho);
  const command_chirho = ["ghc", "-fforce-recomp", "-fno-code", "-v0", "-outputdir", "build-chirho", "ProbeChirho.hs"];
  const child_chirho = Bun.spawn(command_chirho, { cwd: case_directory_chirho, stdout: "pipe", stderr: "pipe" });
  let timed_out_chirho = false;
  const timer_chirho = setTimeout(() => { timed_out_chirho = true; child_chirho.kill("SIGKILL"); }, 15000);
  const [exit_chirho, stdout_chirho, stderr_chirho] = await Promise.all([child_chirho.exited, new Response(child_chirho.stdout).text(), new Response(child_chirho.stderr).text()]);
  clearTimeout(timer_chirho);
  observations_chirho.push({ name_chirho, predicted_chirho, root_chirho: "ProbeChirho.hs", files_chirho: { "ProbeChirho.hs": source_chirho }, source_sha256_chirho: new Bun.CryptoHasher("sha256").update(source_chirho).digest("hex"), ghc_version_chirho: version_child_chirho.stdout.toString().trim(), command_chirho, exit_chirho, stdout_chirho, stderr_chirho, timed_out_chirho, agrees_chirho: !timed_out_chirho && (exit_chirho === 0) === predicted_chirho });
  console.log(`${name_chirho}: exit=${exit_chirho}, prediction=${predicted_chirho}`);
}
await Bun.write(output_chirho, observations_chirho.map(row_chirho => JSON.stringify(row_chirho)).join("\n") + "\n");
if (observations_chirho.some(row_chirho => row_chirho.timed_out_chirho)) process.exitCode = 1;
