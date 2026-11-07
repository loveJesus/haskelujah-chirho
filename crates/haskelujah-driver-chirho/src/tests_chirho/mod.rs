// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Test submodules for haskelujah-driver-chirho.
//!
//! Each submodule groups tests by feature area to keep file sizes manageable.
//!
//! Module paths stay `tests_chirho::<module>`: `#[path]` places interpreter tests in
//! `evaluation_chirho/`, harness-driven suites in `suites_chirho/` and feature tests in
//! `features_chirho/`, as `lib.rs` does for production modules. `proptest_chirho.rs` stays
//! here because proptest finds its committed regression file from this source path.

#[path = "suites_chirho/bench_chirho.rs"]
pub mod bench_chirho;
pub mod compile_chirho;
#[path = "features_chirho/declared_modules_chirho.rs"]
pub mod declared_modules_chirho;
#[path = "suites_chirho/diff_chirho.rs"]
pub mod diff_chirho;
#[path = "evaluation_chirho/eval_advanced_chirho.rs"]
pub mod eval_advanced_chirho;
#[path = "evaluation_chirho/eval_basic_chirho/mod.rs"]
pub mod eval_basic_chirho;
pub mod eval_data_chirho;
#[path = "evaluation_chirho/eval_io_chirho.rs"]
pub mod eval_io_chirho;
#[path = "evaluation_chirho/eval_letrec_chirho.rs"]
pub mod eval_letrec_chirho;
#[path = "evaluation_chirho/eval_list_chirho.rs"]
pub mod eval_list_chirho;
#[path = "evaluation_chirho/eval_pattern_chirho/mod.rs"]
pub mod eval_pattern_chirho;
#[path = "evaluation_chirho/eval_prelude_chirho.rs"]
pub mod eval_prelude_chirho;
#[path = "evaluation_chirho/eval_typeclass_chirho.rs"]
pub mod eval_typeclass_chirho;
pub mod evidence_chirho;
pub mod extensions_chirho;
#[path = "features_chirho/external_bodies_chirho.rs"]
pub mod external_bodies_chirho;
#[path = "features_chirho/install_chirho.rs"]
pub mod install_chirho;
#[path = "features_chirho/install_source_chirho.rs"]
pub mod install_source_chirho;
#[path = "features_chirho/linear_chirho.rs"]
pub mod linear_chirho;
#[path = "features_chirho/module_search_chirho.rs"]
pub mod module_search_chirho;
pub mod project_chirho;
pub mod proptest_chirho;
#[path = "suites_chirho/report_chirho.rs"]
pub mod report_chirho;
#[path = "suites_chirho/roundtrip_chirho.rs"]
pub mod roundtrip_chirho;
#[path = "features_chirho/splice_chirho.rs"]
pub mod splice_chirho;
