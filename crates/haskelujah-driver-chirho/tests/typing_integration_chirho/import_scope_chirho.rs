// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `join` and `&` reach a module only through an import (unit 532, gpt_chirho #28284 from
//! agy_chirho #28275). The builtin seed gave both bare names to every module, so a program
//! that imports neither Control.Monad nor Data.Function checked where GHC 9.14.1 reports
//! GHC-88464 (T12406, T12589). Each scheme now sits under the modules that export the name,
//! and Phase 3.9 hands it to their imports. A qualified import, which found no module key
//! before and typed the name as an accept-anything placeholder, now gets the exact scheme
//! too. Each program's GHC 9.14.1 behavior is recorded beside it; a program GHC rejects is
//! written before its file name.
//! workflow: compiler-pipeline-chirho/builtin-type-environment-chirho

use haskelujah_driver::{compile_source_chirho, eval_source_with_machine_chirho};
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_typing_chirho::infer_chirho::builtin_value_schemes_chirho;

use super::assert_compile_success_chirho;

/// GHC rejects the program, so the checker must too, with a message naming every reason.
fn rejected_like_ghc_chirho(source_chirho: &str, file_name_chirho: &str, reasons_chirho: &[&str]) {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    match compile_source_chirho(source_chirho, &mut source_map_chirho, file_name_chirho) {
        Ok(_) => panic!("{file_name_chirho}: GHC rejects this program; we accepted it"),
        Err(err_chirho) => {
            let message_chirho = err_chirho.to_string();
            for reason_chirho in reasons_chirho {
                assert!(
                    message_chirho.contains(reason_chirho),
                    "{file_name_chirho}: rejected, but not for `{reason_chirho}`: {message_chirho}"
                );
            }
        }
    }
}

fn run_chirho(file_name_chirho: &str, source_chirho: &str) -> String {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    match eval_source_with_machine_chirho(
        source_chirho,
        &mut source_map_chirho,
        file_name_chirho,
        None,
    ) {
        Ok((_value_chirho, machine_chirho)) => machine_chirho.io_output_chirho,
        Err(err_chirho) => {
            panic!("{file_name_chirho}: GHC runs this program; we failed: {err_chirho}")
        }
    }
}

#[test]
fn join_and_amp_are_seeded_only_under_the_modules_that_export_them_chirho() {
    // The named modules carry the function's own scheme. Every builtin interface that
    // exports the name keeps the scheme its import reached through the old bare seed:
    // semigroupoids' `join` is its Bind method, so that group holds a preserved
    // approximation, not an exact scheme. The exporters are read from the interfaces, so a
    // later re-export needs its own key, not an edit to a list here.
    let schemes_chirho = builtin_value_schemes_chirho();
    let ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    for (name_chirho, exact_modules_chirho) in [
        ("join", ["Control.Monad", "GHC.Base"]),
        ("&", ["Data.Function", "Control.Lens"]),
    ] {
        assert!(
            !schemes_chirho.contains_key(name_chirho),
            "`{name_chirho}` must not be seeded into every module"
        );
        let scheme_of_chirho =
            |module_chirho: &str| schemes_chirho.get(&format!("{module_chirho}.{name_chirho}"));
        let exact_chirho = scheme_of_chirho(exact_modules_chirho[0]).unwrap_or_else(|| {
            panic!(
                "`{}.{name_chirho}` carries its scheme",
                exact_modules_chirho[0]
            )
        });
        for module_chirho in exact_modules_chirho {
            assert_eq!(
                scheme_of_chirho(module_chirho),
                Some(exact_chirho),
                "`{module_chirho}.{name_chirho}` carries the function's own scheme"
            );
        }
        let exporters_chirho: Vec<&str> = ifaces_chirho
            .iter()
            .filter(|iface_chirho| {
                iface_chirho
                    .exports_chirho
                    .values_chirho
                    .contains_key(name_chirho)
            })
            .map(|iface_chirho| iface_chirho.name_chirho.as_str())
            .collect();
        assert!(
            !exporters_chirho.is_empty(),
            "some builtin interface exports `{name_chirho}`"
        );
        for module_chirho in exporters_chirho {
            assert_eq!(
                scheme_of_chirho(module_chirho),
                Some(exact_chirho),
                "`{module_chirho}` exports `{name_chirho}`, so its key keeps the preserved scheme"
            );
        }
    }
}

#[test]
fn join_and_amp_without_their_import_are_rejected_like_ghc_chirho() {
    // GHC 9.14.1 rejects each with GHC-88464. Before this unit each one checked and then
    // failed at run with "missing STG binding", because the bare seed reached every module.
    for (program_chirho, file_name_chirho, name_chirho) in [
        (
            "module Main where\nmain :: IO ()\nmain = print (join (Just (Just (1 :: Int))))\n",
            "J01JoinUnimportedChirho.hs",
            "join",
        ),
        (
            "module Main where\nimport Control.Monad (when)\nmain :: IO ()\nmain = when True (print (join (Just (Just (1 :: Int)))))\n",
            "J02JoinListWithoutJoinChirho.hs",
            "join",
        ),
        (
            "module Main where\nimport Control.Monad hiding (join)\nmain :: IO ()\nmain = print (join (Just (Just (1 :: Int))))\n",
            "J03JoinHiddenChirho.hs",
            "join",
        ),
        (
            "module Main where\nimport qualified Control.Monad as CM\nmain :: IO ()\nmain = print (join (Just (Just (1 :: Int))))\n",
            "J04JoinQualifiedBareUseChirho.hs",
            "join",
        ),
        (
            "module Main where\nmain :: IO ()\nmain = print ((5 :: Int) & (+ 1))\n",
            "A01AmpUnimportedChirho.hs",
            "&",
        ),
        (
            "module Main where\nimport Data.Function (on)\nmain :: IO ()\nmain = print ((5 :: Int) & (+ 1))\n",
            "A02AmpListWithoutAmpChirho.hs",
            "&",
        ),
        (
            "module Main where\nimport Data.Function hiding ((&))\nmain :: IO ()\nmain = print ((5 :: Int) & (+ 1))\n",
            "A03AmpHiddenChirho.hs",
            "&",
        ),
        (
            "module Main where\nmain :: IO ()\nmain = print ((&) (5 :: Int) (+ 1))\n",
            "A04AmpPrefixUnimportedChirho.hs",
            "&",
        ),
    ] {
        rejected_like_ghc_chirho(
            program_chirho,
            file_name_chirho,
            &["unbound variable", name_chirho],
        );
    }
}

#[test]
fn misused_imports_of_join_and_amp_are_type_errors_like_ghc_chirho() {
    // GHC 9.14.1 rejects each with GHC-83865. The unqualified imports already reached the
    // exact scheme through the bare fallback, so the first three were rejected before this
    // unit too. A qualified import found no `Control.Monad.join` or `Data.Function.&` key and
    // typed the name as a placeholder that accepts anything, so the last two were accepted.
    for (program_chirho, file_name_chirho, reason_chirho) in [
        (
            "module Main where\nimport Control.Monad (join)\nmain :: IO ()\nmain = print (join (5 :: Int))\n",
            "J20JoinMisusedAtIntChirho.hs",
            "type mismatch",
        ),
        (
            "module Main where\nimport GHC.Base (join)\nmain :: IO ()\nmain = print (join (5 :: Int))\n",
            "J21JoinGhcBaseMisusedChirho.hs",
            "type mismatch",
        ),
        (
            "module Main where\nimport Data.Function ((&))\nmain :: IO ()\nmain = print ((5 :: Int) & True)\n",
            "A20AmpMisusedWithBoolChirho.hs",
            "type mismatch",
        ),
        (
            "module Main where\nimport qualified Control.Monad as CM\nmain :: IO ()\nmain = print (CM.join (5 :: Int))\n",
            "J22JoinQualifiedMisusedChirho.hs",
            "type mismatch",
        ),
        (
            "module Main where\nimport qualified Data.Function as DF\nmain :: IO ()\nmain = print ((5 :: Int) DF.& True)\n",
            "A21AmpQualifiedMisusedChirho.hs",
            "type mismatch",
        ),
    ] {
        rejected_like_ghc_chirho(program_chirho, file_name_chirho, &[reason_chirho]);
    }
}

#[test]
fn imported_join_and_amp_keep_their_schemes_chirho() {
    // Each import keeps the scheme it reached before, so each checks as GHC accepts it.
    // GHC's answers are recorded for the oracle, but no run is asserted: neither name has a
    // runtime implementation yet, and each run fails with "missing STG binding" before and
    // after this unit.
    for (file_name_chirho, program_chirho, _ghc_chirho) in [
        (
            "J10JoinOpenImportChirho.hs",
            "module Main where\nimport Control.Monad\nmain :: IO ()\nmain = print (join (Just (Just (1 :: Int))))\n",
            "Just 1\n",
        ),
        (
            "J11JoinExplicitChirho.hs",
            "module Main where\nimport Control.Monad (join)\nmain :: IO ()\nmain = print (join [[1, 2], [3 :: Int]])\n",
            "[1,2,3]\n",
        ),
        (
            "J12JoinQualifiedChirho.hs",
            "module Main where\nimport qualified Control.Monad as CM\nmain :: IO ()\nmain = print (CM.join (Just (Just (1 :: Int))))\n",
            "Just 1\n",
        ),
        (
            "J13JoinGhcBaseChirho.hs",
            "module Main where\nimport GHC.Base (join)\nmain :: IO ()\nmain = print (join (Just (Just (1 :: Int))))\n",
            "Just 1\n",
        ),
        (
            "A10AmpOpenImportChirho.hs",
            "module Main where\nimport Data.Function\nmain :: IO ()\nmain = print ((5 :: Int) & (+ 1))\n",
            "6\n",
        ),
        (
            "A11AmpExplicitChirho.hs",
            "module Main where\nimport Data.Function ((&))\nmain :: IO ()\nmain = print ([3, 1, 2 :: Int] & map (* 2))\n",
            "[6,2,4]\n",
        ),
        (
            "A12AmpQualifiedChirho.hs",
            "module Main where\nimport qualified Data.Function as DF\nmain :: IO ()\nmain = print ((5 :: Int) DF.& (+ 1))\n",
            "6\n",
        ),
    ] {
        assert_compile_success_chirho(file_name_chirho, program_chirho);
    }
}

#[test]
fn a_module_s_own_join_and_amp_still_shadow_chirho() {
    // A local or top-level definition answers as GHC does. The builtins have no runtime
    // implementation, so a printed answer proves the module's own binding was used.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "J14JoinLocalLetChirho.hs",
            "module Main where\nmain :: IO ()\nmain = let join x = x + 1 in print (join (5 :: Int))\n",
            "6\n",
        ),
        (
            "J15JoinTopLevelChirho.hs",
            "module Main where\njoin :: Int -> Int\njoin x = x * 2\nmain :: IO ()\nmain = print (join 21)\n",
            "42\n",
        ),
        (
            "A13AmpTopLevelChirho.hs",
            "module Main where\n(&) :: Int -> Int -> Int\nx & y = x + y\nmain :: IO ()\nmain = print (1 & 2)\n",
            "3\n",
        ),
    ] {
        assert_eq!(
            run_chirho(file_name_chirho, program_chirho),
            ghc_chirho,
            "{file_name_chirho}"
        );
    }
    // Open boundary, its own unit: GHC rejects an unimported fully qualified name with
    // GHC-76037. These still check, because the type environment seeds every canonical key.
    // GHC's verdict is recorded so the oracle still runs them; nothing here asserts them.
    let _open_boundaries_chirho = [
        (
            "module Main where\nmain :: IO ()\nmain = print (Control.Monad.join (Just (Just (1 :: Int))))\n",
            "Q01QualifiedJoinUnimportedChirho.hs",
            "GHC-76037",
        ),
        (
            "module Main where\nmain :: IO ()\nmain = print ((5 :: Int) Data.Function.& (+ 1))\n",
            "Q02QualifiedAmpUnimportedChirho.hs",
            "GHC-76037",
        ),
    ];
}
