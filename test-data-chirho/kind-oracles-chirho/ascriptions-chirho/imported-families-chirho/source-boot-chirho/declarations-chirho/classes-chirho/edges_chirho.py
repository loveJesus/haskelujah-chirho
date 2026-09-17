# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""Narrow hs-boot class edges (gpt #23153), GHC 9.14.1, kind-annotated heads, predictions recorded before running. Produces edges-observations-chirho.jsonl."""
import json, os, subprocess, hashlib
root='/private/tmp/haskelujah-boot-class-chirho.wytjqlrc'
H = "{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}\nmodule BootClassChirho where\nimport Data.Kind (Type)\n"
AUX = "module AuxChirho where\nimport {-# SOURCE #-} BootClassChirho\n"
def cls(ctx, body):
    return f"class {ctx}C (a :: Type)" + (" where\n" + body if body is not None else "\n")
cases = [
 ("O1_method_order_swapped", H + cls('', "  m1 :: a -> a\n  m2 :: a -> Int\n"), H + cls('', "  m2 :: a -> Int\n  m1 :: a -> a\n"), "reject", "class op items are compared as ordered lists (declaration order) — uncertain"),
 ("O2_method_order_same_control", H + cls('', "  m1 :: a -> a\n  m2 :: a -> Int\n"), H + cls('', "  m1 :: a -> a\n  m2 :: a -> Int\n"), "accept", "control for O1"),
 ("O3_superclass_order_swapped", H + cls('(Eq a, Show a) => ', "  m1 :: a -> a\n"), H + cls('(Show a, Eq a) => ', "  m1 :: a -> a\n"), "reject", "superclass theta compared as an ordered list — uncertain"),
 ("O4_superclass_order_same_control", H + cls('(Eq a, Show a) => ', "  m1 :: a -> a\n"), H + cls('(Eq a, Show a) => ', "  m1 :: a -> a\n"), "accept", "control for O3"),
 ("D1_default_body_text_differs", H + cls('', "  m1 :: a -> a\n  m1 = id\n"), H + cls('', "  m1 :: a -> a\n  m1 x = x\n"), "accept", "only presence of a default is compared, not its body"),
 ("D2_default_body_semantically_different", H + cls('', "  m1 :: a -> a\n  m1 = id\n"), H + cls('', "  m1 :: a -> a\n  m1 = undefined\n"), "accept", "bodies are not compared at all"),
 ("D3_default_signature_differs", H + cls('', "  m1 :: a -> a\n  default m1 :: Show a => a -> a\n  m1 = id\n"), H + cls('', "  m1 :: a -> a\n  default m1 :: Eq a => a -> a\n  m1 = id\n"), "reject", "a generic default's TYPE is part of the DefMeth info"),
 ("D4_default_signature_same_control", H + cls('', "  m1 :: a -> a\n  default m1 :: Show a => a -> a\n  m1 = id\n"), H + cls('', "  m1 :: a -> a\n  default m1 :: Show a => a -> a\n  m1 = id\n"), "accept", "control for D3"),
 ("W1_empty_where_no_context_impl_adds_method", H + cls('', "  {}\n").replace(" where\n  {}\n", " where {}\n"), H + cls('', "  m1 :: a -> a\n"), "reject", "an empty where makes the boot class concrete even without a context"),
 ("W2_empty_where_explicit_context_impl_adds_method", H + cls('() => ', "  {}\n").replace(" where\n  {}\n", " where {}\n"), H + cls('', "  m1 :: a -> a\n"), "reject", "concrete either way"),
 ("W3_empty_where_vs_absent_body_both_empty", H + cls('', "  {}\n").replace(" where\n  {}\n", " where {}\n"), H + cls('', None), "accept", "both have no methods; kinds annotated"),
 ("W4_absent_body_no_context_impl_adds_method_control", H + cls('', None), H + cls('', "  m1 :: a -> a\n"), "accept", "abstract control (K2 shape)"),
 ("F1_abstract_boot_impl_adds_assoc_family", H + cls('', None), H + cls('', "  type family T a :: Type\n"), "accept", "abstract boot class: associated families free, like methods"),
 ("F2_abstract_boot_impl_adds_assoc_family_with_default", H + cls('', None), H + cls('', "  type family T a :: Type\n  type instance T a = Int\n"), "accept", "same with a default"),
 ("F3_concrete_empty_boot_impl_adds_assoc_family", H + cls('() => ', None), H + cls('', "  type family T a :: Type\n"), "reject", "concrete boot: associated-family list must agree"),
]
out=os.path.join(root,'edges-observations-chirho.jsonl')
with open(out,'w') as fh:
    for name, boot, impl, pred, why in cases:
        d=os.path.join(root,name); os.makedirs(d, exist_ok=True)
        files={"BootClassChirho.hs-boot":boot,"BootClassChirho.hs":impl,"AuxChirho.hs":AUX}
        for fn,src in files.items(): open(os.path.join(d,fn),'w').write(src)
        cmd=["ghc","--make","-fforce-recomp","-fno-code","-v0","-outputdir","build-chirho","BootClassChirho.hs","AuxChirho.hs"]
        p=subprocess.run(cmd,cwd=d,capture_output=True,text=True,timeout=60)
        meas='accept' if p.returncode==0 else 'reject'
        code=next((t.strip('[]') for t in p.stderr.replace(']',' ').split() if t.startswith('[GHC-')),'')
        fh.write(json.dumps({"name_chirho":name,"files_chirho":files,"sha256_chirho":{fn:hashlib.sha256(s.encode()).hexdigest() for fn,s in files.items()},"command_chirho":cmd,"directory_chirho":d,"ghc_version_chirho":"9.14.1","predicted_chirho":pred,"why_predicted_chirho":why,"exit_chirho":p.returncode,"measured_chirho":meas,"agrees_chirho":meas==pred,"ghc_code_chirho":code,"stdout_chirho":p.stdout,"stderr_chirho":p.stderr})+"\n")
