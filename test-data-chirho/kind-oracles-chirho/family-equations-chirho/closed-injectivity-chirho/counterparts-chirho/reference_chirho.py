# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""Counterpart contracts for closed-family injectivity IMPROVEMENT (gpt #23139): no annotation, partially injective (other argument fixed /
ambiguous / contradicting forward reduction), refuted annotation, hidden kind arguments. Predictions recorded before running."""
import json, os, subprocess, shutil
H = "{-# LANGUAGE TypeFamilyDependencies #-}\nmodule ProbeChirho where\n"
cases_chirho = [
 ("N1_no_annotation_consumer", H + "type family Foo a where\n    Foo Int = Bool\n    Foo Bool = Int\nfoo :: Foo a -> Foo a\nfoo x = x\nx :: Bool\nx = foo True\n", "reject", "no injectivity annotation: Foo a ~ Bool must not determine a (ambiguous a0)"),
 ("N2_annotation_makes_it_accept", H + "type family Foo a = r | r -> a where\n    Foo Int = Bool\n    Foo Bool = Int\nfoo :: Foo a -> Foo a\nfoo x = x\nx :: Bool\nx = foo True\n", "accept", "control for N1: the annotation alone makes the same consumer accepted"),
 ("P1_partial_other_argument_fixed", H + "type family P a b = r | r -> a where\n    P Int Char = Bool\n    P Bool Int = Int\np :: P a Char -> P a Char\np x = x\ny :: Bool\ny = p True\n", "accept", "r -> a determines a := Int; b is fixed to Char by the signature"),
 ("P2_partial_other_argument_ambiguous", H + "type family P a b = r | r -> a where\n    P Int Char = Bool\n    P Bool Int = Int\np :: P a b -> P a b\np x = x\ny :: Bool\ny = p True\n", "reject", "a := Int is improved but b is not injective: ambiguous b0"),
 ("P3_partial_contradicts_forward_reduction", H + "type family P a b = r | r -> a where\n    P Int Char = Bool\n    P Bool Int = Int\np :: P a Char -> P a Char\np x = x\nz :: Int\nz = p True\n", "reject", "P a Char ~ Int improves a := Bool, but P Bool Char does not reduce to Int"),
 ("W1_refuted_annotation_declaration", H + "type family W a = r | r -> a where\n    W Int = Bool\n    W Char = Bool\n", "reject", "annotation refuted: two rows with equal RHS and different arguments (GHC-05175)"),
 ("H1_hidden_kind_no_signature", "{-# LANGUAGE TypeFamilyDependencies, PolyKinds, NoMonomorphismRestriction #-}\nmodule ProbeChirho where\nimport Data.Kind (Type)\ntype family IC (a :: k) b (c :: k) = r | r -> a b where\n    IC Int Char Bool = Bool\n    IC Int Bool Int  = Int\nic :: IC a b c -> IC a b c\nic x = x\nicapp = ic True\n", "accept", "T6018 shape: a and b improved through the hidden kind slot; c stays generalized with the equality"),
 ("H2_hidden_kind_with_signature", "{-# LANGUAGE TypeFamilyDependencies, PolyKinds #-}\nmodule ProbeChirho where\nimport Data.Kind (Type)\ntype family IC (a :: k) b (c :: k) = r | r -> a b where\n    IC Int Char Bool = Bool\n    IC Int Bool Int  = Int\nic :: IC a b c -> IC a b c\nic x = x\nicapp :: Bool\nicapp = ic True\n", "reject", "c is not injective, so IC Int Char c0 ~ Bool stays ambiguous under a monomorphic signature"),
]
root_chirho = "/private/tmp/haskelujah-closed-family-chirho.zyynecek"
out_chirho = os.path.join(root_chirho, "counterpart-observations-chirho.jsonl")
rows_chirho = []
with open(out_chirho, "w") as fh_chirho:
    for name_chirho, src_chirho, predicted_chirho, why_chirho in cases_chirho:
        d_chirho = os.path.join(root_chirho, name_chirho); os.makedirs(d_chirho, exist_ok=True)
        open(os.path.join(d_chirho, "ProbeChirho.hs"), "w").write(src_chirho)
        cmd_chirho = ["ghc", "-fforce-recomp", "-fno-code", "-outputdir", "build-chirho", "ProbeChirho.hs"]
        try:
            p_chirho = subprocess.run(cmd_chirho, cwd=d_chirho, capture_output=True, text=True, timeout=60)
            exit_chirho, so_chirho, se_chirho = p_chirho.returncode, p_chirho.stdout, p_chirho.stderr
        except subprocess.TimeoutExpired:
            exit_chirho, so_chirho, se_chirho = 124, "", "TIMEOUT 60s"
        measured_chirho = "accept" if exit_chirho == 0 else "reject"
        code_chirho = next((t.strip("[]") for t in se_chirho.replace("]", " ").split() if t.startswith("[GHC-")), "")
        rec_chirho = {"name_chirho": name_chirho, "files_chirho": {"ProbeChirho.hs": src_chirho}, "command_chirho": cmd_chirho, "directory_chirho": d_chirho,
                      "ghc_version_chirho": "9.14.1", "predicted_chirho": predicted_chirho, "why_predicted_chirho": why_chirho, "exit_chirho": exit_chirho,
                      "measured_chirho": measured_chirho, "agrees_chirho": measured_chirho == predicted_chirho, "ghc_code_chirho": code_chirho,
                      "stdout_chirho": so_chirho, "stderr_chirho": se_chirho}
        fh_chirho.write(json.dumps(rec_chirho) + "\n"); rows_chirho.append(rec_chirho)
print(f"evidence: {out_chirho}")
for r_chirho in rows_chirho:
    lines_chirho = [l.strip() for l in r_chirho["stderr_chirho"].splitlines() if l.strip()]
    key_chirho = next((l for l in lines_chirho if l.startswith("•")), lines_chirho[0] if lines_chirho else "")
    print(f"{r_chirho['name_chirho']:42} {r_chirho['predicted_chirho']:7} {r_chirho['measured_chirho']:7} {'yes' if r_chirho['agrees_chirho'] else 'NO ':3} {r_chirho['ghc_code_chirho']:10} {key_chirho[:90]}")
print("agreement:", sum(r['agrees_chirho'] for r in rows_chirho), "of", len(rows_chirho))
shutil.copy(os.path.abspath(__file__), os.path.join(root_chirho, "injectivity_counterpart_probes_chirho.py"))
