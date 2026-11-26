# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# Import closure of ghc-internal modules under a small CPP model (POSIX, 64-bit, BIGNUM_NATIVE, GHC 9.14).
# Module-level imports, SOURCE imports followed; an #if it cannot evaluate keeps both branches.
# Header and foreign-import counts read the whole file text, so they are upper bounds.
# Usage: python3 -I import_closure_chirho.py <ghc-9.14.1>/libraries/ghc-internal/src GHC.Internal.Base,GHC.Internal.Show
# Measurement aid for spec-chirho/pathway-chirho/26-11-25-rung0-inputs-chirho.md; the scoreboard replaces it.
import os, re, sys

src_chirho = sys.argv[1]
roots_chirho = sys.argv[2].split(",")
defined_chirho = {"BIGNUM_NATIVE": 1, "WORD_SIZE_IN_BITS": 64, "__GLASGOW_HASKELL__": 914, "darwin_HOST_OS": 1, "aarch64_HOST_ARCH": 1, "SIZEOF_HSWORD": 8, "HAVE_ROUNDING_MODES": 1}
files_chirho = {}
for root_dir_chirho, _dirs_chirho, names_chirho in os.walk(src_chirho):
    for name_chirho in names_chirho:
        if name_chirho.endswith((".hs", ".hsc")):
            path_chirho = os.path.join(root_dir_chirho, name_chirho)
            module_chirho = os.path.relpath(path_chirho, src_chirho).rsplit(".", 1)[0].replace("/", ".")
            files_chirho[module_chirho] = path_chirho


def eval_chirho(expr_chirho):
    text_chirho = re.sub(r"//.*|/\*.*?\*/", "", expr_chirho)
    text_chirho = re.sub(r"defined\s*\(\s*(\w+)\s*\)|defined\s+(\w+)", lambda match_chirho: "1" if (match_chirho.group(1) or match_chirho.group(2)) in defined_chirho else "0", text_chirho)
    text_chirho = re.sub(r"MIN_VERSION_\w+\s*\([^)]*\)", "1", text_chirho)
    text_chirho = re.sub(r"\b([A-Za-z_]\w*)\b", lambda match_chirho: str(defined_chirho.get(match_chirho.group(1), 0)), text_chirho)
    text_chirho = text_chirho.replace("&&", " and ").replace("||", " or ").replace("!=", " != ")
    text_chirho = re.sub(r"!(?!=)", " not ", text_chirho)
    # Every identifier is now a number. Evaluate only integer arithmetic, comparisons and
    # and/or/not, with no builtins; anything else is "unknown" and keeps both branches.
    if not re.fullmatch(r"(?:\s|\d|[()<>=!+\-*/%]|\band\b|\bor\b|\bnot\b)*", text_chirho) or "**" in text_chirho:
        return None
    try:
        return bool(eval(text_chirho, {"__builtins__": {}}, {}))
    except Exception:
        return None


import_re_chirho = re.compile(r"^\s*import\s+(\{-#\s*SOURCE\s*#-\}\s+)?(?:safe\s+)?(?:qualified\s+)?([A-Z][\w.]*)")


def imports_chirho(path_chirho):
    # Each stack entry: [branch value (True, False, or None for unknown), some branch already taken].
    stack_chirho, out_chirho = [], []
    for line_chirho in open(path_chirho, encoding="utf-8", errors="replace"):
        directive_chirho = re.match(r"^\s*#\s*(if|ifdef|ifndef|elif|else|endif)\b(.*)", line_chirho)
        if directive_chirho:
            kind_chirho, rest_chirho = directive_chirho.group(1), directive_chirho.group(2).strip()
            if kind_chirho in ("if", "ifdef", "ifndef"):
                value_chirho = eval_chirho(rest_chirho) if kind_chirho == "if" else (rest_chirho.split()[0] in defined_chirho) ^ (kind_chirho == "ifndef")
                stack_chirho.append([value_chirho, value_chirho is True])
            elif kind_chirho == "elif" and stack_chirho:
                top_chirho = stack_chirho[-1]
                value_chirho = None if top_chirho[0] is None else (False if top_chirho[1] else eval_chirho(rest_chirho))
                top_chirho[0] = value_chirho
                top_chirho[1] = top_chirho[1] or value_chirho is True
            elif kind_chirho == "else" and stack_chirho:
                top_chirho = stack_chirho[-1]
                top_chirho[0] = None if top_chirho[0] is None else not top_chirho[1]
            elif kind_chirho == "endif" and stack_chirho:
                stack_chirho.pop()
            continue
        if any(entry_chirho[0] is False for entry_chirho in stack_chirho):
            continue
        import_chirho = import_re_chirho.match(line_chirho)
        if import_chirho:
            out_chirho.append((import_chirho.group(2), bool(import_chirho.group(1))))
    return out_chirho


seen_chirho, order_chirho, missing_chirho, source_edges_chirho = set(), [], set(), 0
todo_chirho = list(roots_chirho)
while todo_chirho:
    current_chirho = todo_chirho.pop()
    if current_chirho in seen_chirho:
        continue
    seen_chirho.add(current_chirho)
    if current_chirho not in files_chirho:
        missing_chirho.add(current_chirho)
        continue
    order_chirho.append(current_chirho)
    for dep_chirho, is_source_chirho in imports_chirho(files_chirho[current_chirho]):
        source_edges_chirho += is_source_chirho
        if dep_chirho.startswith("GHC.Internal") or dep_chirho in files_chirho:
            todo_chirho.append(dep_chirho)

lines_chirho = sum(sum(1 for _line_chirho in open(files_chirho[mod_chirho], encoding="utf-8", errors="replace")) for mod_chirho in order_chirho)
hsc_chirho = sorted(mod_chirho for mod_chirho in order_chirho if files_chirho[mod_chirho].endswith(".hsc"))
boot_chirho = sorted(mod_chirho for mod_chirho in order_chirho if os.path.exists(files_chirho[mod_chirho].rsplit(".", 1)[0] + ".hs-boot"))
print(f"roots {len(roots_chirho)}; closure {len(order_chirho)} modules, {lines_chirho} lines; SOURCE edges {source_edges_chirho}; not found {sorted(missing_chirho)}")
print("hsc in closure:", hsc_chirho)
print("with hs-boot:", len(boot_chirho))
print("modules:", " ".join(sorted(mod_chirho.replace("GHC.Internal.", "") for mod_chirho in order_chirho)))
includes_chirho, foreign_chirho = {}, {}
for mod_chirho in order_chirho:
    file_text_chirho = open(files_chirho[mod_chirho], encoding="utf-8", errors="replace").read()
    for include_chirho in re.findall(r'^\s*#\s*include\s+[<"]([^>"]+)[>"]', file_text_chirho, re.M):
        includes_chirho[include_chirho] = includes_chirho.get(include_chirho, 0) + 1
    for convention_chirho in re.findall(r'^\s*foreign\s+import\s+(\w+)', file_text_chirho, re.M):
        foreign_chirho[convention_chirho] = foreign_chirho.get(convention_chirho, 0) + 1
print("includes:", sorted(includes_chirho.items(), key=lambda pair_chirho: -pair_chirho[1]))
print("foreign imports by convention:", foreign_chirho)
