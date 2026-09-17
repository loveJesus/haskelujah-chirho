# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""Bounded GHC 9.14.1 probes: when must an unused private hs-boot declaration agree with its implementation, and when may it be absent?
Every case carries its PREDICTION (written before running) so measured verdicts stay separable from the hypothesis.
Hypothesis: GHC keys boot agreement on the boot file's EXPORT availabilities; an unexported boot declaration is never compared."""
import json, os, subprocess, sys, tempfile, shutil

KIND_CHIRHO = "{-# LANGUAGE KindSignatures #-}\n"
CONSUMER_CHIRHO = "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n"
CONSUMER_USES_CHIRHO = "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nxChirho :: SecretChirho Int\nxChirho = undefined\nvalueChirho = ()\n"
BOOT_PRIVATE_CHIRHO = KIND_CHIRHO + "module ProviderChirho () where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n"
BOOT_EXPORTED_CHIRHO = KIND_CHIRHO + "module ProviderChirho (SecretChirho) where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n"
BOOT_NOLIST_CHIRHO = KIND_CHIRHO + "module ProviderChirho where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\n"
BOOT_VIA_SIG_CHIRHO = KIND_CHIRHO + "module ProviderChirho (fChirho) where\nimport Data.Kind (Type)\ndata SecretChirho (aChirho :: Type)\nfChirho :: SecretChirho Int -> Int\n"
IMPL_HEAD_CHIRHO = lambda exports: KIND_CHIRHO + f"module ProviderChirho {exports} where\nimport ConsumerChirho\nimport Data.Kind (Type)\n"
MATCHING_CHIRHO = "data SecretChirho (aChirho :: Type) = SecretChirho aChirho\n"
NULLARY_CHIRHO = "data SecretChirho = SecretChirho\n"

cases_chirho = [
 # name, boot, impl, consumer, prediction, why
 ("A1_private_absent", BOOT_PRIVATE_CHIRHO, IMPL_HEAD_CHIRHO("()"), CONSUMER_CHIRHO, "accept", "gpt's measured case: boot does not export SecretChirho, so it is never compared"),
 ("A2_private_mismatched", BOOT_PRIVATE_CHIRHO, IMPL_HEAD_CHIRHO("()") + NULLARY_CHIRHO, CONSUMER_CHIRHO, "accept", "unexported boot decl is not compared even when a same-named local decl disagrees in kind"),
 ("A3_exported_absent", BOOT_EXPORTED_CHIRHO, IMPL_HEAD_CHIRHO("()"), CONSUMER_CHIRHO, "reject", "boot exports SecretChirho; module neither exports nor defines it"),
 ("A4_exported_defined_but_unexported", BOOT_EXPORTED_CHIRHO, IMPL_HEAD_CHIRHO("()") + MATCHING_CHIRHO, CONSUMER_CHIRHO, "reject", "boot exports it, module defines it but does not export it"),
 ("A5_exported_matching", BOOT_EXPORTED_CHIRHO, IMPL_HEAD_CHIRHO("(SecretChirho)") + MATCHING_CHIRHO, CONSUMER_CHIRHO, "accept", "control: exported, defined, agreeing"),
 ("B1_nolist_absent", BOOT_NOLIST_CHIRHO, IMPL_HEAD_CHIRHO("()"), CONSUMER_CHIRHO, "reject", "no export list exports every boot decl; module lacks it"),
 ("B2_nolist_matching", BOOT_NOLIST_CHIRHO, IMPL_HEAD_CHIRHO("") .replace("module ProviderChirho  where","module ProviderChirho where") + MATCHING_CHIRHO, CONSUMER_CHIRHO, "accept", "control: everything exported and agreeing"),
 ("B3_nolist_kind_mismatch", BOOT_NOLIST_CHIRHO, IMPL_HEAD_CHIRHO("").replace("module ProviderChirho  where","module ProviderChirho where") + NULLARY_CHIRHO, CONSUMER_CHIRHO, "reject", "exported boot decl compared: kinds differ"),
 ("B4_nolist_defined_but_unexported", BOOT_NOLIST_CHIRHO, IMPL_HEAD_CHIRHO("()") + MATCHING_CHIRHO, CONSUMER_CHIRHO, "reject", "isolates the export requirement: defined and agreeing but not exported by the module"),
 ("C1_via_signature_matching", BOOT_VIA_SIG_CHIRHO, IMPL_HEAD_CHIRHO("(fChirho)") + MATCHING_CHIRHO + "fChirho :: SecretChirho Int -> Int\nfChirho _ = 0\n", CONSUMER_CHIRHO, "accept", "private type reachable only through an exported signature; both agree"),
 ("C2_via_signature_type_absent", BOOT_VIA_SIG_CHIRHO, IMPL_HEAD_CHIRHO("(fChirho)") + "fChirho :: Int -> Int\nfChirho _ = 0\n", CONSUMER_CHIRHO, "reject", "the exported fChirho disagrees because the module has no SecretChirho to mention"),
 ("C3_via_signature_private_mismatched", BOOT_VIA_SIG_CHIRHO, IMPL_HEAD_CHIRHO("(fChirho)") + NULLARY_CHIRHO + "fChirho :: Int -> Int\nfChirho _ = 0\n", CONSUMER_CHIRHO, "reject", "rejected through fChirho, not through the private SecretChirho itself"),
 ("D1_private_class_absent", KIND_CHIRHO + "module ProviderChirho () where\nclass CChirho aChirho\n", IMPL_HEAD_CHIRHO("()"), CONSUMER_CHIRHO, "accept", "unexported abstract boot class never compared"),
 ("D2_exported_class_absent", KIND_CHIRHO + "module ProviderChirho (CChirho) where\nclass CChirho aChirho\n", IMPL_HEAD_CHIRHO("()"), CONSUMER_CHIRHO, "reject", "exported abstract boot class must exist and be exported"),
 ("E1_private_value_absent", "module ProviderChirho () where\ngChirho :: Int\n", IMPL_HEAD_CHIRHO("()"), CONSUMER_CHIRHO, "accept", "unexported boot value signature never compared"),
 ("E2_exported_value_absent", "module ProviderChirho (gChirho) where\ngChirho :: Int\n", IMPL_HEAD_CHIRHO("()"), CONSUMER_CHIRHO, "reject", "exported boot value must be exported and defined by the module"),
 ("F1_consumer_uses_private", BOOT_PRIVATE_CHIRHO, IMPL_HEAD_CHIRHO("()"), CONSUMER_USES_CHIRHO, "reject", "a private boot decl is unusable by importers: not in scope"),
 ("F2_consumer_uses_exported", BOOT_EXPORTED_CHIRHO, IMPL_HEAD_CHIRHO("(SecretChirho)") + MATCHING_CHIRHO, CONSUMER_USES_CHIRHO, "accept", "control for F1: exported boot decl is usable"),
 ("G1_private_but_illformed", KIND_CHIRHO + "module ProviderChirho () where\ndata SecretChirho (aChirho :: NoSuchKindChirho)\n", IMPL_HEAD_CHIRHO("()"), CONSUMER_CHIRHO, "reject", "the boot file itself is still checked; privacy skips agreement, not well-formedness"),
]

root_chirho = tempfile.mkdtemp(prefix="haskelujah-boot-private-chirho.", dir="/private/tmp")
out_chirho = os.path.join(root_chirho, "observations-chirho.jsonl")
rows_chirho = []
with open(out_chirho, "w") as fh_chirho:
    for name_chirho, boot_chirho, impl_chirho, consumer_chirho, predicted_chirho, why_chirho in cases_chirho:
        d_chirho = os.path.join(root_chirho, name_chirho); os.makedirs(d_chirho)
        files_chirho = {"ProviderChirho.hs-boot": boot_chirho, "ProviderChirho.hs": impl_chirho, "ConsumerChirho.hs": consumer_chirho}
        for fn_chirho, src_chirho in files_chirho.items():
            open(os.path.join(d_chirho, fn_chirho), "w").write(src_chirho)
        cmd_chirho = ["ghc", "--make", "-fforce-recomp", "-fno-code", "-v0", "-outputdir", "build-chirho", "ConsumerChirho.hs"]
        try:
            p_chirho = subprocess.run(cmd_chirho, cwd=d_chirho, capture_output=True, text=True, timeout=60)
            exit_chirho, so_chirho, se_chirho = p_chirho.returncode, p_chirho.stdout, p_chirho.stderr
        except subprocess.TimeoutExpired:
            exit_chirho, so_chirho, se_chirho = 124, "", "TIMEOUT 60s"
        measured_chirho = "accept" if exit_chirho == 0 else "reject"
        code_chirho = ""
        for tok_chirho in se_chirho.replace("]", " ").split():
            if tok_chirho.startswith("[GHC-"): code_chirho = tok_chirho.strip("[]"); break
        rec_chirho = {"name_chirho": name_chirho, "files_chirho": files_chirho, "command_chirho": cmd_chirho, "directory_chirho": d_chirho,
                      "ghc_version_chirho": "9.14.1", "predicted_chirho": predicted_chirho, "why_predicted_chirho": why_chirho,
                      "exit_chirho": exit_chirho, "measured_chirho": measured_chirho, "agrees_chirho": measured_chirho == predicted_chirho,
                      "ghc_code_chirho": code_chirho, "stdout_chirho": so_chirho, "stderr_chirho": se_chirho}
        fh_chirho.write(json.dumps(rec_chirho) + "\n"); rows_chirho.append(rec_chirho)
print(f"evidence: {out_chirho}")
print(f"{'case':38} {'pred':7} {'meas':7} {'ok':3} code       first stderr line")
for r_chirho in rows_chirho:
    first_chirho = next((l for l in r_chirho["stderr_chirho"].splitlines() if "error" in l or "rror:" in l), r_chirho["stderr_chirho"].splitlines()[0] if r_chirho["stderr_chirho"] else "")
    print(f"{r_chirho['name_chirho']:38} {r_chirho['predicted_chirho']:7} {r_chirho['measured_chirho']:7} {'yes' if r_chirho['agrees_chirho'] else 'NO ':3} {r_chirho['ghc_code_chirho']:10} {first_chirho[:70]}")
print("agreement:", sum(r['agrees_chirho'] for r in rows_chirho), "of", len(rows_chirho))
shutil.copy(out_chirho, os.path.join(os.path.dirname(os.path.abspath(__file__)), "boot_private_observations_chirho.jsonl"))
