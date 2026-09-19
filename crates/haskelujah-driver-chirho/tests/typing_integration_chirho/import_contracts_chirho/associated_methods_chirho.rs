// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Class-method specialization preserves checked hidden kinds across providers.
use haskelujah_driver::{check_source_path_chirho, compile_modules_chirho};
use haskelujah_runtime_chirho::ExecutionModeChirho;
use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn class_method_kind_scope_and_universals_chirho() {
    let cases_chirho: &[(&str, bool, &str)] = &include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/imported-families-chirho/source-boot-chirho/declarations-chirho/classes-chirho/default-annotations-chirho/scope-chirho/corpus-chirho/associated-methods-chirho/scope_fixtures_chirho.rs"
    ));
    let mut failures_chirho = Vec::new();
    for (name_chirho, accepts_chirho, source_chirho) in cases_chirho {
        let result_chirho = haskelujah_driver::typecheck_source_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            name_chirho,
        );
        match result_chirho {
            Ok(_) if !accepts_chirho => {
                failures_chirho.push(format!("{name_chirho}: wrongly accepted"))
            }
            Err(error_chirho) if *accepts_chirho => {
                failures_chirho.push(format!("{name_chirho}: {error_chirho}"))
            }
            Err(error_chirho) => assert!(
                error_chirho
                    .to_string()
                    .contains(if name_chirho.starts_with("K5_") {
                        "E0300"
                    } else {
                        "E0200"
                    }),
                "{name_chirho}: {error_chirho}",
            ),
            _ => {}
        }
    }
    assert!(failures_chirho.is_empty(), "{}", failures_chirho.join("\n"));
}

#[test]
fn class_methods_specialize_their_parameter_kinds_chirho() {
    type CaseChirho = (&'static str, bool, &'static [(&'static str, &'static str)]);
    let cases_chirho: &[CaseChirho] = &include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/imported-families-chirho/source-boot-chirho/declarations-chirho/classes-chirho/default-annotations-chirho/scope-chirho/corpus-chirho/associated-methods-chirho/fixtures_chirho.rs"
    ));
    let mut failures_chirho = Vec::new();
    for (name_chirho, accepts_chirho, files_chirho) in cases_chirho {
        let directory_chirho = tempfile::tempdir().unwrap();
        for (file_chirho, source_chirho) in *files_chirho {
            std::fs::write(directory_chirho.path().join(file_chirho), source_chirho).unwrap();
        }
        let root_chirho = directory_chirho.path().join(files_chirho.last().unwrap().0);
        let results_chirho = [
            (
                "file",
                check_source_path_chirho(&root_chirho, ExecutionModeChirho::BatchChirho)
                    .map(|_| ()),
            ),
            (
                "batch",
                compile_modules_chirho(files_chirho, &mut SourceMapChirho::new_chirho())
                    .map(|_| ()),
            ),
        ];
        for (entry_chirho, result_chirho) in results_chirho {
            match result_chirho {
                Ok(_) if !accepts_chirho => {
                    failures_chirho.push(format!("{name_chirho}/{entry_chirho}: wrongly accepted"))
                }
                Err(error_chirho)
                    if *accepts_chirho || !error_chirho.to_string().contains("E0200") =>
                {
                    failures_chirho.push(format!("{name_chirho}/{entry_chirho}: {error_chirho}"))
                }
                _ => {}
            }
        }
    }
    assert!(failures_chirho.is_empty(), "{}", failures_chirho.join("\n"));
}
