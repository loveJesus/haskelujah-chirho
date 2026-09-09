// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Execute actual IO primitives; a named zero-returning stub is not an effect.

use super::*;
use std::fs;

fn string_chirho(value_chirho: &str) -> CoreExprChirho {
    CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(value_chirho.to_string()))
}

fn call_chirho(id_chirho: u32, arguments_chirho: Vec<CoreExprChirho>) -> CoreExprChirho {
    arguments_chirho.into_iter().fold(
        CoreExprChirho::VarChirho(CoreIdChirho(id_chirho)),
        |function_chirho, argument_chirho| CoreExprChirho::AppChirho {
            fun_chirho: Box::new(function_chirho),
            arg_chirho: Box::new(argument_chirho),
        },
    )
}

fn module_chirho(
    entry_chirho: CoreExprChirho,
    adapters_chirho: &[(&str, u32, u32)],
) -> CoreModuleChirho {
    let mut bindings_chirho = vec![CoreBindingChirho {
        binder_chirho: dummy_binder_chirho("main", 0),
        rhs_chirho: entry_chirho,
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    }];
    for &(name_chirho, id_chirho, arity_chirho) in adapters_chirho {
        let mut body_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: format!("{name_chirho}#"),
            args_chirho: (0..arity_chirho)
                .map(|parameter_chirho| {
                    CoreExprChirho::VarChirho(CoreIdChirho(100 + id_chirho * 16 + parameter_chirho))
                })
                .collect(),
        };
        for parameter_chirho in (0..arity_chirho).rev() {
            body_chirho = CoreExprChirho::LamChirho {
                binder_chirho: dummy_binder_chirho(
                    "argument_chirho",
                    100 + id_chirho * 16 + parameter_chirho,
                ),
                body_chirho: Box::new(body_chirho),
            };
        }
        bindings_chirho.push(CoreBindingChirho {
            binder_chirho: dummy_binder_chirho(name_chirho, id_chirho),
            rhs_chirho: body_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
    CoreModuleChirho {
        name_chirho: "StringAdaptersChirho".to_string(),
        bindings_chirho,
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
    }
}

#[test]
fn put_str_ln_writes_the_string_and_one_newline_chirho() {
    let module_chirho = module_chirho(
        call_chirho(1, vec![string_chirho("Hello from Haskelujah!")]),
        &[("putStrLn", 1, 1)],
    );
    assert_eq!(
        run_executable_module_result_chirho(&module_chirho),
        (0, "Hello from Haskelujah!\n".to_string(), String::new())
    );
}

#[test]
fn put_str_preserves_the_absence_of_a_newline_chirho() {
    let module_chirho = module_chirho(
        call_chirho(1, vec![string_chirho("Hello")]),
        &[("putStr", 1, 1)],
    );
    assert_eq!(
        run_executable_module_result_chirho(&module_chirho),
        (0, "Hello".to_string(), String::new())
    );
}

#[test]
fn read_file_returns_the_actual_file_contents_chirho() {
    let directory_chirho = tempfile::tempdir().expect("isolated input directory");
    let path_chirho = directory_chirho.path().join("input-chirho.txt");
    let expected_chirho = "first line\nsecond line\n";
    fs::write(&path_chirho, expected_chirho).expect("input fixture");
    let contents_chirho = dummy_binder_chirho("contents_chirho", 1000);
    let module_chirho = module_chirho(
        CoreExprChirho::PrimOpChirho {
            name_chirho: "bindIO#".to_string(),
            args_chirho: vec![
                call_chirho(
                    2,
                    vec![string_chirho(
                        path_chirho.to_str().expect("UTF-8 fixture path"),
                    )],
                ),
                CoreExprChirho::LamChirho {
                    binder_chirho: contents_chirho.clone(),
                    body_chirho: Box::new(call_chirho(
                        1,
                        vec![CoreExprChirho::VarChirho(contents_chirho.id_chirho)],
                    )),
                },
            ],
        },
        &[("putStr", 1, 1), ("readFile", 2, 1)],
    );
    assert_eq!(
        run_executable_module_result_chirho(&module_chirho),
        (0, expected_chirho.to_string(), String::new())
    );
}

#[test]
fn write_file_creates_the_expected_contents_chirho() {
    let directory_chirho = tempfile::tempdir().expect("isolated output directory");
    let path_chirho = directory_chirho.path().join("output-chirho.txt");
    let expected_chirho = "written-chirho\n";
    let module_chirho = module_chirho(
        call_chirho(
            1,
            vec![
                string_chirho(path_chirho.to_str().expect("UTF-8 fixture path")),
                string_chirho(expected_chirho),
            ],
        ),
        &[("writeFile", 1, 2)],
    );
    assert_eq!(
        run_executable_module_result_chirho(&module_chirho),
        (0, String::new(), String::new())
    );
    assert_eq!(
        fs::read_to_string(path_chirho).expect("written file"),
        expected_chirho
    );
}
