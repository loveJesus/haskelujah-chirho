// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 692, C10, design B (gpt #32647): a compiler class whose spelling the program's own class
//! takes keeps its own dictionaries. Each program here is in the GHC reference plan
//! `c10-controls`; the interpreter, LLVM and Cranelift must each print GHC's output. A displaced
//! request this pass cannot serve is refused by name and never answered by the program's own
//! instance (gpt #32666).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_driver::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

use super::assert_prints_everywhere_chirho;

/// No namesake: an instance of the compiler's own Num keeps its literal and arithmetic rows.
const USER_COMPILER_NUM_INSTANCE_CHIRHO: &str =
    include_str!("user_compiler_num_instance_chirho.hs");
/// The program's own `fromInteger`, a compiler literal and `Prelude.fromInteger` in one module.
const SOURCE_COMPILER_COEXIST_CHIRHO: &str = include_str!("source_compiler_coexist_chirho.hs");
/// The Prelude's `sum` is constrained by the compiler's Num whatever the program spells.
const PRELUDE_SUM_UNDER_NAMESAKE_CHIRHO: &str =
    include_str!("prelude_sum_under_namesake_chirho.hs");
/// A literal in a constructor field, the shape the selector form keeps for every backend.
const BOX_UNDER_NAMESAKE_CHIRHO: &str = include_str!("box_under_namesake_chirho.hs");
/// The compiler's `Show [Maybe Int]` while the program's own `Show` takes the spelling.
const SHOW_LIST_UNDER_NAMESAKE_CHIRHO: &str = include_str!("show_list_under_namesake_chirho.hs");

#[test]
fn a_user_instance_of_the_compiler_num_keeps_its_own_body_chirho() {
    assert_prints_everywhere_chirho(
        "user_compiler_num_instance_chirho.hs",
        USER_COMPILER_NUM_INSTANCE_CHIRHO,
        "500m\n",
    );
}

#[test]
fn the_programs_own_num_and_the_compilers_num_coexist_chirho() {
    assert_prints_everywhere_chirho(
        "source_compiler_coexist_chirho.hs",
        SOURCE_COMPILER_COEXIST_CHIRHO,
        "8\n7\n7\n",
    );
}

#[test]
fn the_prelude_sum_keeps_the_compilers_num_under_a_namesake_chirho() {
    assert_prints_everywhere_chirho(
        "prelude_sum_under_namesake_chirho.hs",
        PRELUDE_SUM_UNDER_NAMESAKE_CHIRHO,
        "6\n2\n",
    );
}

#[test]
fn a_literal_in_a_constructor_field_keeps_the_compilers_num_under_a_namesake_chirho() {
    assert_prints_everywhere_chirho(
        "box_under_namesake_chirho.hs",
        BOX_UNDER_NAMESAKE_CHIRHO,
        "42\n",
    );
}

/// GHC prints `[Just 1]`. This pass builds no compiler dictionary for `Show [Maybe Int]` while
/// the program's own `Show` takes the spelling, so the module is refused by name; the program's
/// own instance never answers the compiler's request.
#[test]
fn a_displaced_compiler_show_it_cannot_build_is_refused_by_name_chirho() {
    let message_chirho = match compile_source_chirho(
        SHOW_LIST_UNDER_NAMESAKE_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "show_list_under_namesake_chirho.hs",
    ) {
        Ok(_) => panic!("served, where it must be refused by name"),
        Err(error_chirho) => error_chirho.to_string(),
    };
    assert!(
        message_chirho.contains("the compiler's `Show` method")
            && message_chirho.contains("while the program's own class takes the spelling `Show`"),
        "refused, but not by the displaced compiler request: {message_chirho}"
    );
}
