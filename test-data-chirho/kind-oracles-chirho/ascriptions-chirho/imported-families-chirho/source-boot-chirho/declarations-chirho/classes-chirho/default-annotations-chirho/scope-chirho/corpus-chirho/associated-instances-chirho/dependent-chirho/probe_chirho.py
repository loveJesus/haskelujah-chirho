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
  # ---- valid controls first -------------------------------------------------------------
  ("d01_dependent_default_consumer", "accept",
   "family-only dependent parameters are legal in an associated family; instance C Int takes the default; F Int Bool 'True reduces to 'True",
   {"D.hs": mod("D", DEP + "instance C Int\nok :: Proxy (F Int Bool 'True) -> Proxy 'True\nok = id\n")}),
  ("d02_nondependent_default_consumer", "accept",
   "ordinary family-only nondependent parameter with a default",
   {"D.hs": mod("D", "class C2 (a :: Type) where\n  type G a (b :: Type) :: Type\n  type G a b = [b]\ninstance C2 Int\nok :: Proxy (G Int Bool) -> Proxy [Bool]\nok = id\n")}),
  ("d03_two_kinds_same_instance", "accept",
   "one instance, the default used at two different kinds for k",
   {"D.hs": mod("D", DEP + "instance C Int\nokB :: Proxy (F Int Bool 'False) -> Proxy 'False\nokB = id\nokO :: Proxy (F Int Ordering 'GT) -> Proxy 'GT\nokO = id\n")}),
  ("d04_polykinded_class_two_instances", "accept",
   "class parameter of kind j; instances at Type and at Type -> Type both take the dependent default",
   {"D.hs": mod("D", "class C3 (a :: j) where\n  type H a (k :: Type) (b :: k) :: k\n  type H a k b = b\ninstance C3 Int\ninstance C3 Maybe\nok1 :: Proxy (H Int Bool 'True) -> Proxy 'True\nok1 = id\nok2 :: Proxy (H Maybe Ordering 'LT) -> Proxy 'LT\nok2 = id\n")}),
  ("d05_invisible_dependent_kind", "accept",
   "the kind variable is an implicit (hidden) family parameter, not a written one",
   {"D.hs": mod("D", "class C4 (a :: Type) where\n  type F2 a (b :: k) :: k\n  type F2 a b = b\ninstance C4 Int\nok :: Proxy (F2 Int 'True) -> Proxy 'True\nok = id\nokVisible :: Proxy (F2 Int @Bool 'True) -> Proxy 'True\nokVisible = id\n")}),
  ("d06_explicit_equations_specialise_family_only_kind", "accept",
   "an instance may give several equations that differ in the family-only (dependent) parameter",
   {"D.hs": mod("D", DEP + "instance C Char where\n  type F Char Bool b = 'False\n  type F Char Ordering b = 'LT\nok1 :: Proxy (F Char Bool 'True) -> Proxy 'False\nok1 = id\nok2 :: Proxy (F Char Ordering 'GT) -> Proxy 'LT\nok2 = id\n")}),
  ("d07_imported_dependent_default", "accept",
   "provider/consumer split: the default row and its hidden kind argument cross the module boundary",
   {"Provider.hs": mod("Provider", DEP + "instance C Int\n"),
    "D.hs": mod("D", "import Provider\nok :: Proxy (F Int Bool 'True) -> Proxy 'True\nok = id\n")}),
  # ---- contradictions -------------------------------------------------------------------
  ("d08_contradictory_result", "reject",
   "F Int Bool 'True is 'True, not 'False: a type mismatch (GHC-83865)",
   {"D.hs": mod("D", DEP + "instance C Int\nbad :: Proxy (F Int Bool 'True) -> Proxy 'False\nbad = id\n")}),
  ("d09_argument_kind_contradicts_k", "reject",
   "the third argument must have kind k = Bool; 'LT has kind Ordering (GHC-83865)",
   {"D.hs": mod("D", DEP + "instance C Int\nbad :: Proxy (F Int Bool 'LT) -> Proxy 'LT\nbad = id\n")}),
  ("d10_result_used_at_wrong_kind", "reject",
   "F Int Bool 'True has kind Bool, so it cannot be the type of a value (GHC-83865)",
   {"D.hs": mod("D", DEP + "instance C Int\nbad :: F Int Bool 'True\nbad = undefined\n")}),
  ("d11_default_rhs_contradicts_dependent_result", "reject",
   "in the default, k is a variable of the LHS, so the RHS 'True :: Bool does not have kind k (GHC-83865)",
   {"D.hs": mod("D", "class C (a :: Type) where\n  type F a (k :: Type) (b :: k) :: k\n  type F a k b = 'True\n")}),
  ("d12_default_lhs_specialises_k", "reject",
   "a default's arguments must be distinct type variables; Bool is not one",
   {"D.hs": mod("D", "class C (a :: Type) where\n  type F a (k :: Type) (b :: k) :: k\n  type F a Bool b = b\n")}),
  ("d13_partial_equations_disable_the_default", "reject",
   "an instance that gives ANY equation for F does not also get the default, so F Char () '() is stuck and does not match '()",
   {"D.hs": mod("D", DEP + "instance C Char where\n  type F Char Bool b = 'False\nbad :: Proxy (F Char () '()) -> Proxy '()\nbad = id\n")}),
  ("d14_explicit_equation_wrong_kind_rhs", "reject",
   "an explicit equation at k = Bool must return a Bool: 'LT has kind Ordering (GHC-83865)",
   {"D.hs": mod("D", DEP + "instance C Char where\n  type F Char Bool b = 'LT\n")}),
]

def main():
    out = open("observations-chirho.jsonl", "w")
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
