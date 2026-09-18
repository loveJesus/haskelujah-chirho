# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""GHC 9.14.1 reference probes: associated defaults with family-only DEPENDENT parameters.
Every case carries its PREDICTION, written before the run. Output: observations-chirho.jsonl."""
import hashlib, json, os, subprocess, sys, tempfile

HEAD = "{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}\n"
IMPORTS = "import Data.Kind (Type)\nimport Data.Proxy (Proxy)\n"
DEP = """class C (a :: Type) where
  type F a (k :: Type) (b :: k) :: k
  type F a k b = b
"""

def mod(name, body, head=HEAD, imports=IMPORTS):
    return f"{head}module {name} where\n{imports}{body}"

CASES = [
  # Follow-up to the refuted d05: the two uses are separated, predictions written before this run.
  ("d05a_invisible_dependent_kind_implicit_use", "accept",
   "the hidden kind argument is inferred from 'True; no visible application",
   {"D.hs": mod("D", "class C4 (a :: Type) where\n  type F2 a (b :: k) :: k\n  type F2 a b = b\ninstance C4 Int\nok :: Proxy (F2 Int 'True) -> Proxy 'True\nok = id\n")}),
  ("d05b_invisible_dependent_kind_visible_application", "reject",
   "k is an INFERRED variable of F2's kind, so @Bool cannot be applied (GHC-20967), as measured in the combined d05",
   {"D.hs": mod("D", "class C4 (a :: Type) where\n  type F2 a (b :: k) :: k\n  type F2 a b = b\ninstance C4 Int\nokVisible :: Proxy (F2 Int @Bool 'True) -> Proxy 'True\nokVisible = id\n")}),
  ("d05c_class_kind_signature_does_not_specify_k", "reject",
   "a standalone kind signature on the CLASS says nothing about the family's own kind variable; still inferred (uncertain)",
   {"D.hs": mod("D", "import Data.Kind (Constraint)\ntype C5 :: Type -> Constraint\nclass C5 a where\n  type F3 a (b :: k) :: k\n  type F3 a b = b\ninstance C5 Int\nokVisible :: Proxy (F3 Int @Bool 'True) -> Proxy 'True\nokVisible = id\n")}),
  ("d05d_invisible_kind_contradiction", "reject",
   "with the kind hidden, a contradictory result is still a type mismatch (GHC-83865)",
   {"D.hs": mod("D", "class C4 (a :: Type) where\n  type F2 a (b :: k) :: k\n  type F2 a b = b\ninstance C4 Int\nbad :: Proxy (F2 Int 'True) -> Proxy 'LT\nbad = id\n")}),
  ("d05e_invisible_kind_two_kinds", "accept",
   "the hidden kind argument differs per use: Bool at one consumer, Ordering at another",
   {"D.hs": mod("D", "class C4 (a :: Type) where\n  type F2 a (b :: k) :: k\n  type F2 a b = b\ninstance C4 Int\nok1 :: Proxy (F2 Int 'True) -> Proxy 'True\nok1 = id\nok2 :: Proxy (F2 Int 'GT) -> Proxy 'GT\nok2 = id\n")}),
]

def main():
    out = open("observations-followup-chirho.jsonl", "w")
    for name, prediction, reading, files in CASES:
        work = tempfile.mkdtemp(prefix=name + ".")
        hashes = {}
        for fname, text in files.items():
            with open(os.path.join(work, fname), "w") as handle:
                handle.write(text)
            hashes[fname] = hashlib.sha256(text.encode()).hexdigest()
        order = [f for f in files if f != "D.hs"] + ["D.hs"]
        command = ["ghc", "-v0", "-fno-code", "-Wno-missing-methods"] + order
        run = subprocess.run(command, cwd=work, capture_output=True, text=True)
        measured = "accept" if run.returncode == 0 else "reject"
        codes = sorted(set(part.split("]")[0] for part in run.stderr.split("[GHC-")[1:]))
        record = {"case": name, "prediction": prediction, "reading": reading, "sources": files,
                  "sha256": hashes, "command": " ".join(command), "exit": run.returncode,
                  "stderr": run.stderr, "codes": codes, "measured": measured,
                  "agrees": measured == prediction}
        out.write(json.dumps(record, ensure_ascii=False) + "\n")
        first = next((line.strip() for line in run.stderr.splitlines() if line.strip().startswith("•")), "")
        print(f"{name:52s} predicted={prediction:6s} measured={measured:6s} {'OK ' if measured == prediction else 'REFUTED'} {','.join(codes)} {first[:110]}")
    out.close()

if __name__ == "__main__":
    main()
