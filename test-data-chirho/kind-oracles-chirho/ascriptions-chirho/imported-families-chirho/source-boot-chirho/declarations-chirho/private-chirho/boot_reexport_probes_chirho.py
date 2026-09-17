# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""Follow-up matrix (gpt #23103): does hs-boot agreement need the RE-EXPORT shape — a boot-exported name that the implementing
module re-exports from a third module instead of declaring locally? Same Consumer <-> Provider cycle; OtherChirho is a plain third module.
Predictions recorded before running. Hypothesis: GHC compares exported NAMES (module-qualified); a boot re-export needs no local
definition and must be matched by the module exporting the SAME original name; a locally declared same-spelled type is a different name."""
import json, os, subprocess, tempfile, shutil

OTHER_CHIRHO = "module OtherChirho where\ndata OtherTypeChirho = OtherTypeChirho\n"
CONSUMER_CHIRHO = "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nvalueChirho = ()\n"
CONSUMER_USES_CHIRHO = "module ConsumerChirho where\nimport {-# SOURCE #-} ProviderChirho\nxChirho :: OtherTypeChirho\nxChirho = undefined\nvalueChirho = ()\n"
BOOT_REEXPORT_CHIRHO = "module ProviderChirho (OtherTypeChirho) where\nimport OtherChirho (OtherTypeChirho)\n"
BOOT_REEXPORT_MODULE_CHIRHO = "module ProviderChirho (module OtherChirho) where\nimport OtherChirho\n"
BOOT_LOCAL_CHIRHO = "module ProviderChirho (OtherTypeChirho) where\ndata OtherTypeChirho\n"
def impl_chirho(exports, body):
    return f"module ProviderChirho {exports} where\nimport ConsumerChirho\n{body}"

cases_chirho = [
 ("R1_boot_reexport_module_reexport", BOOT_REEXPORT_CHIRHO, impl_chirho("(OtherTypeChirho)", "import OtherChirho (OtherTypeChirho)\n"), CONSUMER_CHIRHO, "accept", "boot re-exports OtherChirho.OtherTypeChirho without defining it; module exports the same original name; nothing to compare"),
 ("R2_boot_reexport_module_declares_local", BOOT_REEXPORT_CHIRHO, impl_chirho("(OtherTypeChirho)", "data OtherTypeChirho = LocalChirho\n"), CONSUMER_CHIRHO, "reject", "module exports ProviderChirho.OtherTypeChirho, a different name from the boot's OtherChirho.OtherTypeChirho"),
 ("R3_boot_declares_module_reexports", BOOT_LOCAL_CHIRHO, impl_chirho("(OtherTypeChirho)", "import OtherChirho (OtherTypeChirho)\n"), CONSUMER_CHIRHO, "reject", "boot defines ProviderChirho.OtherTypeChirho; module exports OtherChirho's name instead and defines nothing"),
 ("R4_module_form_reexport_both", BOOT_REEXPORT_MODULE_CHIRHO, impl_chirho("(module OtherChirho)", "import OtherChirho\n"), CONSUMER_CHIRHO, "accept", "module-form re-export on both sides"),
 ("R5_boot_reexport_module_omits", BOOT_REEXPORT_CHIRHO, impl_chirho("()", "import OtherChirho (OtherTypeChirho)\n"), CONSUMER_CHIRHO, "reject", "boot exports the re-exported name; module imports it but does not export it"),
 ("R6_consumer_uses_boot_reexport", BOOT_REEXPORT_CHIRHO, impl_chirho("(OtherTypeChirho)", "import OtherChirho (OtherTypeChirho)\n"), CONSUMER_USES_CHIRHO, "accept", "a boot re-export is usable by importers of the boot"),
 ("R7_boot_reexport_module_reexports_other_origin", BOOT_REEXPORT_CHIRHO, impl_chirho("(OtherTypeChirho)", "import Other2Chirho (OtherTypeChirho)\n"), CONSUMER_CHIRHO, "reject", "same spelling, different defining module (Other2Chirho): names differ"),
]

root_chirho = "/private/tmp/haskelujah-boot-private-chirho.hee0furr"
out_chirho = os.path.join(root_chirho, "reexport-observations-chirho.jsonl")
rows_chirho = []
with open(out_chirho, "w") as fh_chirho:
    for name_chirho, boot_chirho, impl_src_chirho, consumer_chirho, predicted_chirho, why_chirho in cases_chirho:
        d_chirho = os.path.join(root_chirho, name_chirho); os.makedirs(d_chirho, exist_ok=True)
        files_chirho = {"OtherChirho.hs": OTHER_CHIRHO, "ProviderChirho.hs-boot": boot_chirho, "ProviderChirho.hs": impl_src_chirho, "ConsumerChirho.hs": consumer_chirho}
        if "Other2Chirho" in impl_src_chirho:
            files_chirho["Other2Chirho.hs"] = "module Other2Chirho where\ndata OtherTypeChirho = OtherTypeChirho\n"
        for fn_chirho, src_chirho in files_chirho.items():
            open(os.path.join(d_chirho, fn_chirho), "w").write(src_chirho)
        cmd_chirho = ["ghc", "--make", "-fforce-recomp", "-fno-code", "-v0", "-outputdir", "build-chirho", "ConsumerChirho.hs"]
        try:
            p_chirho = subprocess.run(cmd_chirho, cwd=d_chirho, capture_output=True, text=True, timeout=60)
            exit_chirho, so_chirho, se_chirho = p_chirho.returncode, p_chirho.stdout, p_chirho.stderr
        except subprocess.TimeoutExpired:
            exit_chirho, so_chirho, se_chirho = 124, "", "TIMEOUT 60s"
        measured_chirho = "accept" if exit_chirho == 0 else "reject"
        code_chirho = next((t.strip("[]") for t in se_chirho.replace("]", " ").split() if t.startswith("[GHC-")), "")
        rec_chirho = {"name_chirho": name_chirho, "files_chirho": files_chirho, "command_chirho": cmd_chirho, "directory_chirho": d_chirho,
                      "ghc_version_chirho": "9.14.1", "predicted_chirho": predicted_chirho, "why_predicted_chirho": why_chirho,
                      "exit_chirho": exit_chirho, "measured_chirho": measured_chirho, "agrees_chirho": measured_chirho == predicted_chirho,
                      "ghc_code_chirho": code_chirho, "stdout_chirho": so_chirho, "stderr_chirho": se_chirho}
        fh_chirho.write(json.dumps(rec_chirho) + "\n"); rows_chirho.append(rec_chirho)
print(f"evidence: {out_chirho}")
for r_chirho in rows_chirho:
    lines_chirho = r_chirho["stderr_chirho"].splitlines()
    msg_chirho = " ".join(l.strip() for l in lines_chirho[1:3]) if len(lines_chirho) > 1 else ""
    print(f"{r_chirho['name_chirho']:46} {r_chirho['predicted_chirho']:7} {r_chirho['measured_chirho']:7} {'yes' if r_chirho['agrees_chirho'] else 'NO ':3} {r_chirho['ghc_code_chirho']:10} {msg_chirho[:110]}")
print("agreement:", sum(r['agrees_chirho'] for r in rows_chirho), "of", len(rows_chirho))
shutil.copy(os.path.abspath(__file__), os.path.join(root_chirho, "boot_reexport_probes_chirho.py"))
