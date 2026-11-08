// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 692, C10 Q (gpt #33099): a qualified builtin class written in a constraint (`P.Num`,
//! `F.Foldable`, `Prelude.Ord`) names the class naming selected for that spelling. A compiler
//! class and a program class with equal text never discharge each other's constraints, directly
//! or through a superclass. Each program here is in the GHC reference plan
//! `c10-qualified-class-v1`. The positives must print GHC's output on the interpreter, LLVM and
//! Cranelift; the negatives must be refused for GHC's reason.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};

/// P1: `P.Num a` with no namesake, used by the unqualified `+`.
const QUALIFIED_PRELUDE_NUM_CHIRHO: &str = include_str!("qualified_prelude_num_chirho.hs");
/// P2: `F.Foldable t` from `Data.Foldable`, used by `F.foldr`.
const QUALIFIED_FOLDABLE_CHIRHO: &str = include_str!("qualified_foldable_chirho.hs");
/// P3: `Prelude.Ord b` reaches the compiler's `Eq` while the program's own `Ord` takes the spelling.
const QUALIFIED_ORD_SUPERCLASS_UNDER_NAMESAKE_CHIRHO: &str =
    include_str!("qualified_ord_superclass_under_namesake_chirho.hs");
/// N1: `Prelude.Num b` beside a use of the program's own `fromInteger`.
const QUALIFIED_GIVEN_LOCAL_WANTED_CHIRHO: &str =
    include_str!("qualified_given_local_wanted_chirho.hs");
/// N2: the program's own `Num b` beside the Prelude's `+`.
const LOCAL_GIVEN_COMPILER_WANTED_CHIRHO: &str =
    include_str!("local_given_compiler_wanted_chirho.hs");
/// T: `Prelude.Num b` beside a namesake whose superclass is `Show`, used by `show`.
const QUALIFIED_GIVEN_SOURCE_SUPERCLASS_CHIRHO: &str =
    include_str!("qualified_given_source_superclass_chirho.hs");

#[test]
fn a_qualified_prelude_num_given_serves_the_compilers_num_chirho() {
    assert_prints_everywhere_chirho(
        "qualified_prelude_num_chirho.hs",
        QUALIFIED_PRELUDE_NUM_CHIRHO,
        "42\n",
    );
}

#[test]
fn a_qualified_foldable_given_serves_the_compilers_foldable_chirho() {
    assert_prints_everywhere_chirho(
        "qualified_foldable_chirho.hs",
        QUALIFIED_FOLDABLE_CHIRHO,
        "6\n",
    );
}

/// The positive behind any claim about qualified superclass hierarchies: the compiler's `Ord`
/// keeps its own `Eq` superclass even when the program's own `Ord` has none.
#[test]
fn a_qualified_ord_given_keeps_its_eq_superclass_under_a_namesake_chirho() {
    assert_prints_everywhere_chirho(
        "qualified_ord_superclass_under_namesake_chirho.hs",
        QUALIFIED_ORD_SUPERCLASS_UNDER_NAMESAKE_CHIRHO,
        "True\nFalse\n",
    );
}

#[test]
fn a_qualified_compiler_given_never_serves_the_programs_own_class_chirho() {
    rejected_like_ghc_chirho(
        QUALIFIED_GIVEN_LOCAL_WANTED_CHIRHO,
        "qualified_given_local_wanted_chirho.hs",
        &["no instance for `Num b`"],
    );
}

#[test]
fn the_programs_own_given_never_serves_the_compilers_class_chirho() {
    rejected_like_ghc_chirho(
        LOCAL_GIVEN_COMPILER_WANTED_CHIRHO,
        "local_given_compiler_wanted_chirho.hs",
        &["no instance for `Num b`"],
    );
}

/// The traversal negative: a compiler given never expands the namesake's record, so the
/// namesake's `Show` superclass never proves the `Show b` that `show` asks for.
#[test]
fn a_qualified_compiler_given_never_takes_the_namesakes_superclass_chirho() {
    rejected_like_ghc_chirho(
        QUALIFIED_GIVEN_SOURCE_SUPERCLASS_CHIRHO,
        "qualified_given_source_superclass_chirho.hs",
        &["no instance for `Show b`"],
    );
}
