# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""GHC 9.14.1 reference contracts for T6018's closed injective families Bak/Foo/Bar (gpt #23119): original reductions with consumers,
then invalid mutations — remove the covering prior branch, change it to a disjoint pattern, move it after the conflicting row — plus
GHC's own failing siblings and one case separating "pair-partner coverage" from "whole prior prefix coverage".
Predictions are written before running. Reading used for the predictions: for equations i<j whose RHSs unify with substitution s,
the LHSs must agree under s unless s(lhs_j) is already matched by SOME earlier equation k<j (not only i)."""
import json, os, subprocess, tempfile, shutil

HDR_CHIRHO = "{-# LANGUAGE TypeFamilyDependencies #-}\nmodule ProbeChirho where\n"
BAK_USE_CHIRHO = "bak :: Bak a -> Bak a\nbak x = x\nbakapp1 :: Char\nbakapp1 = bak 'c'\nbakapp2 :: Double\nbakapp2 = bak 1.0\nbakapp3 :: ()\nbakapp3 = bak ()\n"
FOO_USE_CHIRHO = "foo :: Foo a -> Foo a\nfoo x = x\nfooapp1 :: Bool\nfooapp1 = foo True\nfooRow2 :: Foo Bool\nfooRow2 = (1 :: Int)\n"
BAR_USE_CHIRHO = "bar :: Bar a -> Bar a\nbar x = x\nbarapp1 :: Bool\nbarapp1 = bar True\nbarapp2 :: Int\nbarapp2 = bar 1\n"
def fam_chirho(name, rows):
    return f"type family {name} a = r | r -> a where\n" + "".join(f"    {name} {l} = {r}\n" for l, r in rows)

cases_chirho = [
 ("K0_bak_original_with_consumers", HDR_CHIRHO + fam_chirho("Bak", [("Int","Char"),("Char","Int"),("a","a")]) + BAK_USE_CHIRHO, "accept", "corpus content: each conflict of the catch-all is covered by an earlier row"),
 ("K1_bak_remove_covering_prior", HDR_CHIRHO + fam_chirho("Bak", [("Int","Char"),("a","a")]) + BAK_USE_CHIRHO, "reject", "rows (Int=Char, a=a): a:=Char, Bak Char uncovered"),
 ("K2_bak_covering_made_disjoint", HDR_CHIRHO + fam_chirho("Bak", [("Int","Char"),("Double","Int"),("a","a")]) + BAK_USE_CHIRHO, "reject", "Bak Double no longer covers Bak Char"),
 ("K3_bak_covering_moved_after", HDR_CHIRHO + fam_chirho("Bak", [("Int","Char"),("a","a"),("Char","Int")]) + BAK_USE_CHIRHO, "reject", "coverage must come from EARLIER rows; Bak Char=Int now follows the catch-all"),
 ("K4_bak_cover_in_prefix_not_partner", HDR_CHIRHO + fam_chirho("Bak", [("Char","Int"),("Bool","Bool"),("Int","Char"),("a","a")]) + BAK_USE_CHIRHO, "accept", "pair (Int=Char, a=a) is covered by row 1 (Char=Int), which is neither adjacent nor the pair partner: whole-prefix coverage"),
 ("K5_ghc_E2_catch_all_concrete_rhs", "{-# LANGUAGE TypeFamilyDependencies, DataKinds #-}\nmodule ProbeChirho where\ntype family E2 (a :: Bool) = r | r -> a where\n  E2 False = True\n  E2 True  = False\n  E2 a     = False\n", "reject", "GHC's own T6018failclosed case: catch-all with concrete RHS, instance E2 a not covered"),
 ("F0_foo_original_with_consumers", HDR_CHIRHO + fam_chirho("Foo", [("Int","Bool"),("Bool","Int"),("Bool","Bool")]) + FOO_USE_CHIRHO, "accept", "third row unreachable, covered by row 2; consumers use rows 1 and 2"),
 ("F1_foo_remove_covering_prior", HDR_CHIRHO + fam_chirho("Foo", [("Int","Bool"),("Bool","Bool")]) + "foo :: Foo a -> Foo a\nfoo x = x\n", "reject", "Int=Bool and Bool=Bool: RHS equal, LHS differ, nothing covers Foo Bool"),
 ("F2_foo_covering_made_disjoint", HDR_CHIRHO + fam_chirho("Foo", [("Int","Bool"),("Char","Int"),("Bool","Bool")]) + "foo :: Foo a -> Foo a\nfoo x = x\n", "reject", "Foo Char does not cover Foo Bool"),
 ("F3_foo_covering_moved_after", HDR_CHIRHO + fam_chirho("Foo", [("Int","Bool"),("Bool","Bool"),("Bool","Int")]) + "foo :: Foo a -> Foo a\nfoo x = x\n", "reject", "the covering row now follows the conflicting row"),
 ("F4_foo_consumer_of_unreachable_row", HDR_CHIRHO + fam_chirho("Foo", [("Int","Bool"),("Bool","Int"),("Bool","Bool")]) + "x :: Foo Bool\nx = True\n", "reject", "the dropped third row never fires: Foo Bool is Int"),
 ("B0_bar_original_with_consumers", HDR_CHIRHO + fam_chirho("Bar", [("Int","Bool"),("Bool","Int"),("Bool","Char")]) + BAR_USE_CHIRHO, "accept", "corpus content; no two RHSs unify"),
 ("B1_bar_remove_covering_prior", HDR_CHIRHO + fam_chirho("Bar", [("Int","Bool"),("Bool","Char")]) + "bar :: Bar a -> Bar a\nbar x = x\n", "accept", "control: Bar never relied on coverage, its RHSs are pairwise distinct"),
 ("B3_bar_covering_moved_after", HDR_CHIRHO + fam_chirho("Bar", [("Int","Bool"),("Bool","Char"),("Bool","Int")]) + "bar :: Bar a -> Bar a\nbar x = x\n", "accept", "still pairwise-distinct RHSs; third row merely unreachable"),
 ("B4_ghc_failclosed2_consumer_of_unreachable", HDR_CHIRHO + fam_chirho("Bar", [("Int","Bool"),("Bool","Int"),("Bool","Char")]) + "bar :: Bar a -> Bar a\nbar x = x\nbarapp :: Char\nbarapp = bar 'c'\n", "reject", "GHC's own T6018failclosed2: consumer needs the unreachable row, a0 ambiguous (GHC-83865)"),
]

root_chirho = tempfile.mkdtemp(prefix="haskelujah-closed-family-chirho.", dir="/private/tmp")
out_chirho = os.path.join(root_chirho, "observations-chirho.jsonl")
rows_chirho = []
with open(out_chirho, "w") as fh_chirho:
    for name_chirho, src_chirho, predicted_chirho, why_chirho in cases_chirho:
        d_chirho = os.path.join(root_chirho, name_chirho); os.makedirs(d_chirho)
        open(os.path.join(d_chirho, "ProbeChirho.hs"), "w").write(src_chirho)
        cmd_chirho = ["ghc", "-fforce-recomp", "-fno-code", "-outputdir", "build-chirho", "ProbeChirho.hs"]
        try:
            p_chirho = subprocess.run(cmd_chirho, cwd=d_chirho, capture_output=True, text=True, timeout=60)
            exit_chirho, so_chirho, se_chirho = p_chirho.returncode, p_chirho.stdout, p_chirho.stderr
        except subprocess.TimeoutExpired:
            exit_chirho, so_chirho, se_chirho = 124, "", "TIMEOUT 60s"
        measured_chirho = "accept" if exit_chirho == 0 else "reject"
        code_chirho = next((t.strip("[]") for t in se_chirho.replace("]", " ").split() if t.startswith("[GHC-")), "")
        warn_chirho = "warning" in se_chirho
        rec_chirho = {"name_chirho": name_chirho, "files_chirho": {"ProbeChirho.hs": src_chirho}, "command_chirho": cmd_chirho, "directory_chirho": d_chirho,
                      "ghc_version_chirho": "9.14.1", "predicted_chirho": predicted_chirho, "why_predicted_chirho": why_chirho,
                      "exit_chirho": exit_chirho, "measured_chirho": measured_chirho, "agrees_chirho": measured_chirho == predicted_chirho,
                      "ghc_code_chirho": code_chirho, "warning_present_chirho": warn_chirho, "stdout_chirho": so_chirho, "stderr_chirho": se_chirho}
        fh_chirho.write(json.dumps(rec_chirho) + "\n"); rows_chirho.append(rec_chirho)
print(f"evidence: {out_chirho}")
for r_chirho in rows_chirho:
    lines_chirho = [l.strip() for l in r_chirho["stderr_chirho"].splitlines() if l.strip()]
    key_chirho = next((l for l in lines_chirho if l.startswith("•") or "warning" in l or "rror" in l), "")
    print(f"{r_chirho['name_chirho']:44} {r_chirho['predicted_chirho']:7} {r_chirho['measured_chirho']:7} {'yes' if r_chirho['agrees_chirho'] else 'NO ':3} {r_chirho['ghc_code_chirho']:10} {'warn' if r_chirho['warning_present_chirho'] else '    '} {key_chirho[:80]}")
print("agreement:", sum(r['agrees_chirho'] for r in rows_chirho), "of", len(rows_chirho))
shutil.copy(os.path.abspath(__file__), os.path.join(root_chirho, "t6018_closed_family_probes_chirho.py"))
