// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Backend Round-Trip Smoke Tests (§61)
//!
//! For each test program, evaluates via the STG interpreter to get the
//! reference answer, then compiles through each backend (LLVM, Wasm,
//! Cranelift) and validates that the emitted artifact is structurally
//! correct and encodes the same computation.
//!
//! Native toolchains are required for the LLVM execution gate:
//! - LLVM binaries produce the expected exit result, with bounded execution
//! - Wasm binary has the correct magic number and version
//! - Cranelift object has an ELF/Mach-O header signature
//!
//! Wasm/Cranelift checks below remain artifact smoke tests, not execution proofs.
//! Dedicated native round-trip suites execute both native backends.

use haskelujah_span_chirho::SourceMapChirho;

use crate::{compile_source_chirho, eval_source_with_machine_chirho};

/// Test case: source code + expected STG result + backend validation fns.
struct RoundtripCaseChirho {
    name_chirho: &'static str,
    source_chirho: &'static str,
    expected_int_chirho: i64,
}

const ROUNDTRIP_CASES_CHIRHO: &[RoundtripCaseChirho] = &[
    RoundtripCaseChirho {
        name_chirho: "constant",
        source_chirho: "module Main where\nmain = 42\n",
        expected_int_chirho: 42,
    },
    RoundtripCaseChirho {
        name_chirho: "arithmetic",
        source_chirho: "module Main where\nf x y = x + y\nmain = f 10 32\n",
        expected_int_chirho: 42,
    },
    RoundtripCaseChirho {
        name_chirho: "conditional",
        source_chirho: "module Main where\nmain = if 3 > 2 then 100 else 0\n",
        expected_int_chirho: 100,
    },
    RoundtripCaseChirho {
        name_chirho: "let_binding",
        source_chirho: "module Main where\nmain = let x = 7 in x * 6\n",
        expected_int_chirho: 42,
    },
    RoundtripCaseChirho {
        name_chirho: "case_expr",
        source_chirho: concat!(
            "module Main where\n",
            "classify n = case n of\n",
            "  0 -> 0\n",
            "  1 -> 10\n",
            "  _ -> 99\n",
            "main = classify 1\n",
        ),
        expected_int_chirho: 10,
    },
    RoundtripCaseChirho {
        name_chirho: "recursive",
        source_chirho: concat!(
            "module Main where\n",
            "fact n = if n == 0 then 1 else n * fact (n - 1)\n",
            "main = fact 5\n",
        ),
        expected_int_chirho: 120,
    },
    RoundtripCaseChirho {
        name_chirho: "nested_let",
        source_chirho: concat!(
            "module Main where\n",
            "main = let a = 10\n",
            "           b = 20\n",
            "           c = a + b\n",
            "       in c + 12\n",
        ),
        expected_int_chirho: 42,
    },
    RoundtripCaseChirho {
        name_chirho: "multi_arg",
        source_chirho: concat!(
            "module Main where\n",
            "add3 a b c = a + b + c\n",
            "main = add3 10 20 12\n",
        ),
        expected_int_chirho: 42,
    },
];

// ── STG interpreter baseline ──────────────────────────────────────────

#[test]
fn roundtrip_stg_baseline_all_chirho() {
    for case_chirho in ROUNDTRIP_CASES_CHIRHO {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let (val_chirho, _) = eval_source_with_machine_chirho(
            case_chirho.source_chirho,
            &mut sm_chirho,
            &format!("{}.hs", case_chirho.name_chirho),
            None,
        )
        .unwrap_or_else(|e_chirho| {
            panic!(
                "STG eval failed for '{}': {}",
                case_chirho.name_chirho, e_chirho
            )
        });
        match val_chirho {
            haskelujah_runtime_chirho::ValueChirho::IntChirho(n_chirho) => {
                assert_eq!(
                    n_chirho, case_chirho.expected_int_chirho,
                    "STG baseline mismatch for '{}'",
                    case_chirho.name_chirho
                );
            }
            other_chirho => {
                panic!(
                    "STG eval for '{}' returned {:?}, expected Int({})",
                    case_chirho.name_chirho, other_chirho, case_chirho.expected_int_chirho
                );
            }
        }
    }
}

// ── LLVM round-trip ───────────────────────────────────────────────────

#[test]
fn roundtrip_llvm_all_chirho() {
    for case_chirho in ROUNDTRIP_CASES_CHIRHO {
        let result_chirho =
            haskelujah_test_harness_chirho::native_chirho::native_round_trip_chirho(
                case_chirho.source_chirho,
                haskelujah_test_harness_chirho::native_chirho::NativeBackendChirho::LlvmChirho,
                "",
            )
            .unwrap_or_else(|error_chirho| panic!("{}: {error_chirho}", case_chirho.name_chirho));
        assert_eq!(
            result_chirho,
            (
                case_chirho.expected_int_chirho as i32,
                format!("{}\n", case_chirho.expected_int_chirho)
            ),
            "{}",
            case_chirho.name_chirho
        );
    }
}

// ── Wasm round-trip ───────────────────────────────────────────────────

#[test]
fn roundtrip_wasm_all_chirho() {
    for case_chirho in ROUNDTRIP_CASES_CHIRHO {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(
            case_chirho.source_chirho,
            &mut sm_chirho,
            &format!("{}.hs", case_chirho.name_chirho),
        )
        .unwrap_or_else(|e_chirho| {
            panic!(
                "compile failed for '{}': {}",
                case_chirho.name_chirho, e_chirho
            )
        });

        let wasm_chirho = haskelujah_backend_wasm_chirho::compile_core_to_wasm_executable_chirho(
            &result_chirho.core_chirho,
        );

        // Valid Wasm magic number
        assert_eq!(
            &wasm_chirho[0..4],
            b"\0asm",
            "Wasm magic mismatch for '{}'",
            case_chirho.name_chirho
        );
        // Version 1
        assert_eq!(
            &wasm_chirho[4..8],
            &[1, 0, 0, 0],
            "Wasm version mismatch for '{}'",
            case_chirho.name_chirho
        );
        // Non-trivial size
        assert!(
            wasm_chirho.len() > 20,
            "Wasm binary for '{}' suspiciously small: {} bytes",
            case_chirho.name_chirho,
            wasm_chirho.len()
        );
    }
}

// ── Cranelift round-trip ──────────────────────────────────────────────

#[test]
fn roundtrip_cranelift_all_chirho() {
    for case_chirho in ROUNDTRIP_CASES_CHIRHO {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(
            case_chirho.source_chirho,
            &mut sm_chirho,
            &format!("{}.hs", case_chirho.name_chirho),
        )
        .unwrap_or_else(|e_chirho| {
            panic!(
                "compile failed for '{}': {}",
                case_chirho.name_chirho, e_chirho
            )
        });

        let config_chirho = haskelujah_backend_cranelift_chirho::TargetConfigChirho::default();
        let native_obj_chirho =
            haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
                &result_chirho.core_chirho,
                &config_chirho,
            )
            .unwrap_or_else(|e_chirho| {
                panic!(
                    "Cranelift compile failed for '{}': {}",
                    case_chirho.name_chirho, e_chirho
                )
            });

        let obj_chirho = &native_obj_chirho.object_bytes_chirho;

        // Non-empty object
        assert!(
            !obj_chirho.is_empty(),
            "Cranelift object for '{}' is empty",
            case_chirho.name_chirho
        );

        // Check for ELF (0x7f ELF) or Mach-O (0xFE 0xED or 0xCF 0xFA)
        let is_elf_chirho = obj_chirho.len() >= 4
            && obj_chirho[0] == 0x7f
            && obj_chirho[1] == b'E'
            && obj_chirho[2] == b'L'
            && obj_chirho[3] == b'F';
        let is_macho_chirho = obj_chirho.len() >= 4
            && ((obj_chirho[0] == 0xFE && obj_chirho[1] == 0xED)
                || (obj_chirho[0] == 0xCF && obj_chirho[1] == 0xFA));
        let is_coff_chirho = obj_chirho.len() >= 2
            && ((obj_chirho[0] == 0x64 && obj_chirho[1] == 0x86)   // x86-64
                || (obj_chirho[0] == 0x4C && obj_chirho[1] == 0x01)); // i386
        assert!(
            is_elf_chirho || is_macho_chirho || is_coff_chirho,
            "Cranelift object for '{}' has unrecognized format (first 4 bytes: {:02x?})",
            case_chirho.name_chirho,
            &obj_chirho[..4.min(obj_chirho.len())]
        );
    }
}

// ── Cross-backend consistency ─────────────────────────────────────────

#[test]
fn roundtrip_all_backends_produce_output_chirho() {
    // Verify all 3 backends produce non-empty output for all test cases
    let mut total_chirho = 0;
    for case_chirho in ROUNDTRIP_CASES_CHIRHO {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(
            case_chirho.source_chirho,
            &mut sm_chirho,
            &format!("{}.hs", case_chirho.name_chirho),
        )
        .expect("compile should succeed");

        let llvm_chirho = haskelujah_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
            &result_chirho.core_chirho,
        );
        let wasm_chirho = haskelujah_backend_wasm_chirho::compile_core_to_wasm_executable_chirho(
            &result_chirho.core_chirho,
        );
        let config_chirho = haskelujah_backend_cranelift_chirho::TargetConfigChirho::default();
        let cran_chirho =
            haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
                &result_chirho.core_chirho,
                &config_chirho,
            )
            .expect("Cranelift should compile");

        assert!(
            !llvm_chirho.is_empty(),
            "LLVM empty for {}",
            case_chirho.name_chirho
        );
        assert!(
            !wasm_chirho.is_empty(),
            "Wasm empty for {}",
            case_chirho.name_chirho
        );
        assert!(
            !cran_chirho.object_bytes_chirho.is_empty(),
            "Cranelift empty for {}",
            case_chirho.name_chirho
        );
        total_chirho += 1;
    }
    assert_eq!(total_chirho, ROUNDTRIP_CASES_CHIRHO.len());
}
