#!/usr/bin/env python3
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""Regenerate row 608's qualified builtin alias table from GHC 9.14.1.

Usage, from the repository root:
    python3 -I test-data-chirho/qualified-alias-agreement-chirho/generate_chirho.py

1. Rust (ignored test): list every builtin export with no `module.name` key, by canonical name.
2. GHC: in one ghci session, import every listed module that the packages below expose, and ask
   `:type module.name` for each export of those modules.
3. Rust (ignored test): normalize GHC's answer and the bare builtin scheme under README-chirho.md's
   contract; write the generated table, counts-chirho.txt and disagreements-chirho.tsv.

This is a deliberate regeneration tool: no build or gate runs it, and only it calls ghc-pkg or ghci.
"""

import os
import pathlib
import re
import subprocess
import sys
import tempfile

GHC_VERSION_CHIRHO = "9.14.1"
PACKAGES_CHIRHO = ["array", "base", "bytestring", "containers", "deepseq", "directory", "filepath",
                   "mtl", "process", "stm", "text", "time", "transformers"]
HERE_CHIRHO = pathlib.Path(__file__).resolve().parent
ROOT_CHIRHO = HERE_CHIRHO.parent.parent
STEP_CHIRHO = ["cargo", "test", "-p", "haskelujah-driver", "--test", "typing_integration_chirho", "--",
               "--ignored", "--exact"]


def run_step_chirho(name_chirho, env_chirho):
    result_chirho = subprocess.run(STEP_CHIRHO + [f"qualified_alias_agreement_chirho::{name_chirho}"],
                                   cwd=ROOT_CHIRHO, env=env_chirho, capture_output=True, text=True)
    if result_chirho.returncode != 0 or "1 passed" not in result_chirho.stdout:
        sys.exit(f"{name_chirho} failed:\n{result_chirho.stdout}\n{result_chirho.stderr}")


def qualified_chirho(module_chirho, name_chirho):
    spelled_chirho = f"{module_chirho}.{name_chirho}"
    return spelled_chirho if re.match(r"[A-Za-z_]", name_chirho) else f"({spelled_chirho})"


def main_chirho():
    version_chirho = subprocess.run(["ghc", "--numeric-version"], capture_output=True, text=True,
                                    check=True).stdout.strip()
    if version_chirho != GHC_VERSION_CHIRHO:
        sys.exit(f"GHC {GHC_VERSION_CHIRHO} is required; found {version_chirho}")
    with tempfile.TemporaryDirectory() as work_text_chirho:
        work_chirho = pathlib.Path(work_text_chirho)
        env_chirho = dict(os.environ, QUALIFIED_ALIAS_WORK_CHIRHO=str(work_chirho),
                          QUALIFIED_ALIAS_EVIDENCE_CHIRHO=str(HERE_CHIRHO))
        run_step_chirho("dump_qualified_alias_queries_chirho", env_chirho)
        queries_chirho = [tuple(line_chirho.split("\t"))
                          for line_chirho in (work_chirho / "queries-chirho.tsv").read_text().splitlines()]
        exposed_chirho = set()
        for package_chirho in PACKAGES_CHIRHO:
            fields_chirho = subprocess.run(["ghc-pkg", "field", package_chirho, "exposed-modules", "--simple-output"],
                                           capture_output=True, text=True, check=True).stdout
            exposed_chirho.update(word_chirho.strip(",") for word_chirho in fields_chirho.split())
        asked_chirho = [pair_chirho for pair_chirho in queries_chirho if pair_chirho[0] in exposed_chirho]
        script_chirho = [":set -dppr-cols=100000"]
        script_chirho += [f"import qualified {module_chirho}"
                          for module_chirho in sorted({module_chirho for module_chirho, _ in asked_chirho})]
        script_chirho.append('Prelude.putStrLn "@@IMPORTS"')
        for index_chirho, (module_chirho, name_chirho) in enumerate(asked_chirho):
            script_chirho += [f":type {qualified_chirho(module_chirho, name_chirho)}",
                              f'Prelude.putStrLn "@@{index_chirho}"']
        ghci_chirho = subprocess.run(["ghci", "-v0", "-ignore-dot-ghci"], input="\n".join(script_chirho) + "\n",
                                     capture_output=True, text=True, timeout=3600)
        answers_chirho = {}
        segment_chirho = []
        for line_chirho in ghci_chirho.stdout.splitlines():
            mark_chirho = re.fullmatch(r"@@(\d+|IMPORTS)", line_chirho.strip())
            if not mark_chirho:
                segment_chirho.append(line_chirho)
                continue
            if mark_chirho.group(1) != "IMPORTS":
                typed_chirho = re.match(r"^\S.*? :: (.*)$", " ".join(segment_chirho).strip())
                if typed_chirho:
                    answers_chirho[asked_chirho[int(mark_chirho.group(1))]] = typed_chirho.group(1).strip()
            segment_chirho = []
        rows_chirho = [f"{module_chirho}\t{name_chirho}\t{answers_chirho.get((module_chirho, name_chirho), '')}\n"
                       for module_chirho, name_chirho in queries_chirho]
        (work_chirho / "ghc-types-chirho.tsv").write_text("".join(rows_chirho))
        run_step_chirho("write_qualified_alias_table_chirho", env_chirho)
    print(f"GHC {version_chirho}: {len(queries_chirho)} unkeyed exports, {len(asked_chirho)} in exposed modules, "
          f"{len(answers_chirho)} typed by GHC")
    print((HERE_CHIRHO / "counts-chirho.txt").read_text(), end="")


if __name__ == "__main__":
    main_chirho()
