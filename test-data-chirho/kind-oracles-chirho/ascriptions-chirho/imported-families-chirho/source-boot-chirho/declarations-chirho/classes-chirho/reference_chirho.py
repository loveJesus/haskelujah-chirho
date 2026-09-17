# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""GHC 9.14.1 contracts for hs-boot CLASS agreement (gpt #23147), shaped by T20661 (explicit empty context = concrete class with no
methods) and T20588d (method signature + default body + MINIMAL, associated family + default). Predictions recorded before running.
Layout: BootClassChirho.hs (implementation), BootClassChirho.hs-boot, AuxChirho.hs importing {-# SOURCE #-}; both modules are roots."""
import json, os, subprocess, tempfile, shutil, hashlib
FD = "{-# LANGUAGE FunctionalDependencies #-}\nmodule BootClassChirho where\n"
TF = "{-# LANGUAGE TypeFamilies #-}\nmodule BootClassChirho where\nimport Data.Kind\n"
AUX_INST = "{-# LANGUAGE FunctionalDependencies #-}\nmodule AuxChirho where\nimport {-# SOURCE #-} BootClassChirho\ninstance C Int Bool\n"
AUX_IMPORT = "module AuxChirho where\nimport {-# SOURCE #-} BootClassChirho\n"
C_METH = "class C (a :: Type) where\n  {-# MINIMAL meth #-}\n  meth :: a -> a\n  meth = id\n"
D_FAM = "class D (a :: Type) where\n  type family T a :: Type\n  type instance T a = Int\n"
cases = [
 # name, boot, impl, aux, predicted, why
 ("C0_T20661_verbatim", FD + "class () => C a b | a -> b\n", FD + "class C a b | a -> b\n", AUX_INST, "accept", "corpus: explicit empty context marks a concrete method-less class; implementation agrees"),
 ("C1_abstract_boot_with_fundep_and_instance", FD + "class C a b | a -> b\n", FD + "class C a b | a -> b\n", AUX_INST, "accept", "abstract boot class (no context); instance in aux — uncertain, this is the T20661 question"),
 ("C2_concrete_empty_boot_impl_adds_method", FD + "class () => C a b | a -> b\n", FD + "class C a b | a -> b where\n  meth :: a -> b\n", AUX_INST, "reject", "concrete boot with no methods must match the implementation's method set"),
 ("C3_abstract_boot_impl_adds_method", FD + "class C a b | a -> b\n", FD + "class C a b | a -> b where\n  meth :: a -> b\n", AUX_INST, "accept", "abstract boot class permits any implementation body"),
 ("C4_concrete_boot_fundep_impl_without", FD + "class () => C a b | a -> b\n", FD + "class C a b\n", AUX_INST, "reject", "functional dependencies differ"),
 ("C5_concrete_boot_impl_adds_superclass", FD + "class () => C a\n", FD + "class Eq a => C a\n", AUX_IMPORT, "reject", "superclass contexts differ on a concrete boot class"),
 ("C6_abstract_boot_impl_superclass_and_method", FD + "class C a\n", FD + "class Eq a => C a where\n  meth :: a -> a\n", AUX_IMPORT, "accept", "abstract boot class: context and methods free"),
 ("M0_T20588d_verbatim", TF + C_METH + D_FAM, TF + C_METH + D_FAM, AUX_IMPORT, "accept", "corpus: identical boot and implementation incl. default body, MINIMAL, associated default"),
 ("M1_method_type_differs", TF + C_METH, TF + "class C (a :: Type) where\n  {-# MINIMAL meth #-}\n  meth :: a -> Int\n  meth _ = 0\n", AUX_IMPORT, "reject", "method signatures must agree"),
 ("M2_impl_extra_method", TF + C_METH, TF + "class C (a :: Type) where\n  {-# MINIMAL meth #-}\n  meth :: a -> a\n  meth = id\n  meth2 :: a -> a\n  meth2 = id\n", AUX_IMPORT, "reject", "method sets must agree"),
 ("M3_boot_default_impl_none", TF + C_METH, TF + "class C (a :: Type) where\n  {-# MINIMAL meth #-}\n  meth :: a -> a\n", AUX_IMPORT, "reject", "default-method presence is part of the class contract (DefMeth info)"),
 ("M4_boot_none_impl_default", TF + "class C (a :: Type) where\n  meth :: a -> a\n", TF + "class C (a :: Type) where\n  meth :: a -> a\n  meth = id\n", AUX_IMPORT, "reject", "same in the other direction"),
 ("M5_minimal_boot_only", TF + "class C (a :: Type) where\n  {-# MINIMAL meth #-}\n  meth :: a -> a\n  meth = id\n  meth2 :: a -> a\n  meth2 = id\n", TF + "class C (a :: Type) where\n  meth :: a -> a\n  meth = id\n  meth2 :: a -> a\n  meth2 = id\n", AUX_IMPORT, "reject", "MINIMAL sets differ (boot: meth; impl: nothing required)"),
 ("M6_minimal_both", TF + "class C (a :: Type) where\n  {-# MINIMAL meth #-}\n  meth :: a -> a\n  meth = id\n  meth2 :: a -> a\n  meth2 = id\n", TF + "class C (a :: Type) where\n  {-# MINIMAL meth #-}\n  meth :: a -> a\n  meth = id\n  meth2 :: a -> a\n  meth2 = id\n", AUX_IMPORT, "accept", "control for M5"),
 ("A1_assoc_default_boot_only", TF + D_FAM, TF + "class D (a :: Type) where\n  type family T a :: Type\n", AUX_IMPORT, "reject", "associated-type default presence is compared"),
 ("A2_assoc_default_differs", TF + D_FAM, TF + "class D (a :: Type) where\n  type family T a :: Type\n  type instance T a = Bool\n", AUX_IMPORT, "reject", "associated-type defaults must agree"),
 ("A3_assoc_family_impl_only", TF + "class D (a :: Type) where\n  dmeth :: a -> a\n", TF + "class D (a :: Type) where\n  dmeth :: a -> a\n  type family T a :: Type\n", AUX_IMPORT, "reject", "associated families are part of the class contract"),
 ("A4_assoc_family_no_default_both", TF + "class D (a :: Type) where\n  type family T a :: Type\n", TF + "class D (a :: Type) where\n  type family T a :: Type\n", AUX_IMPORT, "accept", "control for A1/A3"),
]
root = tempfile.mkdtemp(prefix="haskelujah-boot-class-chirho.", dir="/private/tmp")
out = os.path.join(root, "observations-chirho.jsonl"); rows = []
with open(out, "w") as fh:
    for name, boot, impl, aux, pred, why in cases:
        d = os.path.join(root, name); os.makedirs(d)
        files = {"BootClassChirho.hs-boot": boot, "BootClassChirho.hs": impl, "AuxChirho.hs": aux}
        for fn, src in files.items(): open(os.path.join(d, fn), "w").write(src)
        cmd = ["ghc", "--make", "-fforce-recomp", "-fno-code", "-v0", "-outputdir", "build-chirho", "BootClassChirho.hs", "AuxChirho.hs"]
        try:
            p = subprocess.run(cmd, cwd=d, capture_output=True, text=True, timeout=60); ex, so, se = p.returncode, p.stdout, p.stderr
        except subprocess.TimeoutExpired:
            ex, so, se = 124, "", "TIMEOUT 60s"
        meas = "accept" if ex == 0 else "reject"
        code = next((t.strip("[]") for t in se.replace("]", " ").split() if t.startswith("[GHC-")), "")
        hashes = {fn: hashlib.sha256(src.encode()).hexdigest() for fn, src in files.items()}
        rec = {"name_chirho": name, "files_chirho": files, "sha256_chirho": hashes, "command_chirho": cmd, "directory_chirho": d, "ghc_version_chirho": "9.14.1",
               "predicted_chirho": pred, "why_predicted_chirho": why, "exit_chirho": ex, "measured_chirho": meas, "agrees_chirho": meas == pred,
               "ghc_code_chirho": code, "stdout_chirho": so, "stderr_chirho": se}
        fh.write(json.dumps(rec) + "\n"); rows.append(rec)
print("evidence:", out)
for r in rows:
    lines = [l.strip() for l in r["stderr_chirho"].splitlines() if l.strip()]
    key = next((l for l in lines if l.startswith("•") or "rror" in l), lines[0] if lines else "")
    print(f"{r['name_chirho']:46} {r['predicted_chirho']:7} {r['measured_chirho']:7} {'yes' if r['agrees_chirho'] else 'NO ':3} {r['ghc_code_chirho']:10} {key[:80]}")
print("agreement:", sum(r["agrees_chirho"] for r in rows), "of", len(rows))
shutil.copy(os.path.abspath(__file__), os.path.join(root, "boot_class_probes_chirho.py"))
