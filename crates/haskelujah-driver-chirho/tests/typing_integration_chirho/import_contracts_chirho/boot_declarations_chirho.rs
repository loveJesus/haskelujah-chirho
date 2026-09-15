// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-local boot promises, independently referenced against GHC9.14.1.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
#[path = "boot_declaration_cases_chirho.rs"]
mod cases_chirho;

fn compare_reference_chirho(name_chirho: &str) {
    let record_chirho = cases_chirho::CASES_CHIRHO
        .iter()
        .find(|record_chirho| record_chirho.name_chirho == name_chirho)
        .expect("the named independent reference must exist");
    for result_chirho in super::boot_chirho::boot_paths_chirho(
        record_chirho.provider_chirho,
        Some(record_chirho.boot_chirho),
        record_chirho.consumer_chirho,
    ) {
        if record_chirho.accepted_chirho {
            result_chirho.unwrap_or_else(|error_chirho| panic!("{name_chirho}: {error_chirho}"));
        } else {
            let error_chirho = result_chirho.expect_err(name_chirho);
            assert!(
                error_chirho.contains("boot contract"),
                "{name_chirho}: {error_chirho}"
            );
        }
    }
}

macro_rules! boot_reference_tests_chirho {
    ($($name_chirho:ident),+ $(,)?) => {
        $(#[test] fn $name_chirho() { compare_reference_chirho(stringify!($name_chirho)); })+
    };
}

boot_reference_tests_chirho!(
    derived_instance_chirho,
    missing_instance_chirho,
    wrong_head_chirho,
    explicit_instance_chirho,
    empty_boot_body_chirho,
    populated_boot_body_chirho,
    context_alpha_chirho,
    context_mismatch_chirho,
    extra_context_alpha_chirho,
    extra_context_mismatch_chirho,
    abstract_class_chirho,
    explicit_empty_class_chirho,
    fundep_alpha_chirho,
    fundep_missing_chirho,
    fundep_reversed_chirho,
);
