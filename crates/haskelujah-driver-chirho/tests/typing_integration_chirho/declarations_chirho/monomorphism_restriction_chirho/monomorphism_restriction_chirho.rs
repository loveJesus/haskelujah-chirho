// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The monomorphism restriction (Haskell 2010 section 4.5.5) as GHC applies it in every
//! compiled module (row 692, gpt_chirho #32492, #32508-#32510).
//!
//! The restriction is decided for each declaration group:
//! - A group is restricted when any member is a pattern binding, or a simple pattern
//!   binding without a signature.
//! - A restricted group never generalizes its constrained type variables. A later use may
//!   fix them; otherwise the module's defaulting resolves them at its end.
//! - A variable that defaulting cannot resolve is ambiguous, and the program is refused.
//!
//! Before this rule, an unsigned `main` was generalized over its literal's `Num` and kept
//! that dictionary as a lambda. A pure entry then evaluated to a function, and an IO entry
//! never ran its action.
//!
//! Each program sits byte for byte in `accepted_chirho/` (GHC prints the expected output) or
//! `refused_chirho/` (GHC rejects). The expectations are GHC predictions, UNEXECUTED when
//! written. Their reference-only plan lists every file's sha256.
//!
//! What the controls pin:
//! - IO entries: an entry whose result a literal constrains still runs its action (the IO
//!   twin of the evaluator's numeric entries). Only the executable entry's role makes a
//!   bare `main = return 5` IO, and no default reaches an unused `return` in another module.
//! - Defaults: Integer before Double, and the module's own `default` list. A list with no
//!   match, or an empty list, never falls back to another default.
//! - Groups: function syntax contrasted with a lambda-valued simple pattern binding, a
//!   signature that frees a group, one restricted member that restricts its whole group,
//!   and a non-simple pattern binding that stays restricted despite signatures.
//! - Ownership: a program's own class spelled `Num` is never the compiler's numeric class.
//!   A use may fix it; nothing defaults it.
//!
//! - Program entries: the all-engine helpers compile each program as an executable, so its
//!   `main` must be an IO action (Report section 5). The entry's role fixes a restricted
//!   entry's monad, and pure entries are refused, under NoMonomorphismRestriction too.
//!
//! Open by name: the restriction's first brick covers top-level function-syntax and simple
//! pattern bindings. These controls stay RED until their own bricks land (gpt_chirho #32518,
//! #32531):
//! - Non-simple pattern bindings are not yet restricted
//!   (`a_pattern_binding_stays_restricted_despite_signatures_chirho`).
//! - A polymorphic entry, overloaded by its signature or by NoMonomorphismRestriction, passes
//!   typing at IO. Its Core binding still takes a dictionary that the entry wrapper never
//!   supplies, so its action does not run. That needs a typed entry instantiation carried to
//!   the wrapper (`an_overloaded_entry_runs_at_its_instantiated_type_chirho`,
//!   `an_unrestricted_entry_runs_at_its_instantiated_type_chirho`).
//! - The Int/Double lambda control (`an_unsigned_lambda_binding_has_one_type_chirho`) also
//!   meets the unifier's Int/Double widening, a separate unsoundness finding (gpt_chirho
//!   #32611); its Int/Integer sibling isolates Rule 1.
//!
//! workflow: language-features-chirho/fundep-generalization-chirho

use haskelujah_driver::{FrontendOptionsChirho, compile_source_with_options_chirho};
use haskelujah_span_chirho::SourceMapChirho;

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};

/// GHC rejects the program. Compiled as an executable, whose `main` is its IO entry, so must
/// the compiler, for every named reason.
fn rejected_as_program_like_ghc_chirho(
    source_chirho: &str,
    file_name_chirho: &str,
    reasons_chirho: &[&str],
) {
    match compile_source_with_options_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
        &FrontendOptionsChirho::for_program_chirho("main"),
    ) {
        Ok(_) => panic!("{file_name_chirho}: GHC rejects this program; we accepted it"),
        Err(error_chirho) => {
            let message_chirho = error_chirho.to_string();
            for reason_chirho in reasons_chirho {
                assert!(
                    message_chirho.contains(reason_chirho),
                    "{file_name_chirho}: rejected, but not for `{reason_chirho}`: {message_chirho}"
                );
            }
        }
    }
}

/// The function binding is generalized; the unsigned entry is restricted to `IO Integer`.
const RESTRICTED_IO_ENTRY_CHIRHO: &str =
    include_str!("accepted_chirho/restricted_io_entry_chirho.hs");
/// `print 1 >> return 5`: IO comes from `print`, Integer from the default.
const RESTRICTED_ACTION_ENTRY_CHIRHO: &str =
    include_str!("accepted_chirho/restricted_action_entry_chirho.hs");
/// A later use fixes both the monad and the number of `return 5` at `Maybe Int`.
const RESTRICTED_FIXED_BY_USE_CHIRHO: &str =
    include_str!("accepted_chirho/restricted_fixed_by_use_chirho.hs");
/// The program's own `Num`, fixed at Int by a use, answers through its own instance (8).
const SOURCE_NUM_FIXED_CHIRHO: &str = include_str!("accepted_chirho/source_num_fixed_chirho.hs");
/// Fractional and Show: Integer is skipped, Double chosen.
const RESTRICTED_DOUBLE_DEFAULT_CHIRHO: &str =
    include_str!("accepted_chirho/restricted_double_default_chirho.hs");
/// `default (Double)` chooses Double for a plain literal.
const DECLARED_DOUBLE_DEFAULT_CHIRHO: &str =
    include_str!("accepted_chirho/declared_double_default_chirho.hs");
/// Function syntax is never restricted.
const FUNCTION_BINDING_GENERALIZES_CHIRHO: &str =
    include_str!("accepted_chirho/function_binding_generalizes_chirho.hs");
/// A signature frees a simple pattern binding.
const SIGNED_SIMPLE_PATTERN_CHIRHO: &str =
    include_str!("accepted_chirho/signed_simple_pattern_chirho.hs");
/// A signature on the lambda-valued member frees the recursive group.
const SIGNED_SCC_MEMBER_CHIRHO: &str = include_str!("accepted_chirho/signed_scc_member_chirho.hs");
/// The executable entry's role, not a default, makes `main = return 5` IO.
const EXECUTABLE_ENTRY_RETURN_CHIRHO: &str =
    include_str!("accepted_chirho/executable_entry_return_chirho.hs");
/// `Show [t]` reduces through the list instance to `Show t`, which defaults to Integer.
const RESTRICTED_LIST_SHOWN_CHIRHO: &str =
    include_str!("accepted_chirho/restricted_list_shown_chirho.hs");

/// One restricted type cannot be both Fractional and Int.
const RESTRICTED_TWO_TYPES_CHIRHO: &str =
    include_str!("refused_chirho/restricted_two_types_chirho.hs");
/// Monoid has no default.
const RESTRICTED_AMBIGUOUS_MONOID_CHIRHO: &str =
    include_str!("refused_chirho/restricted_ambiguous_monoid_chirho.hs");
/// The program's own `Num` is not a standard numeric class, so it has no default.
const SOURCE_NUM_UNFIXED_CHIRHO: &str = include_str!("refused_chirho/source_num_unfixed_chirho.hs");
/// An unsigned lambda-valued simple pattern binding has one type. At Int and Double it also
/// meets the unifier's Int/Double widening (`unify_chirho.rs`), a separate unsoundness finding.
const LAMBDA_SIMPLE_PATTERN_CHIRHO: &str =
    include_str!("refused_chirho/lambda_simple_pattern_chirho.hs");
/// The same binding at Int and Integer, which are never unified: Rule 1 alone.
const LAMBDA_INTEGER_PATTERN_CHIRHO: &str =
    include_str!("refused_chirho/lambda_integer_pattern_chirho.hs");
/// One unsigned simple pattern binding restricts its function-syntax partner.
const MIXED_SCC_CHIRHO: &str = include_str!("refused_chirho/mixed_scc_chirho.hs");
/// A non-simple pattern binding stays restricted despite signatures.
const SIGNED_PATTERN_BINDING_CHIRHO: &str =
    include_str!("refused_chirho/signed_pattern_binding_chirho.hs");
/// `default (Int)` has no Fractional match.
const DECLARED_DEFAULT_NO_MATCH_CHIRHO: &str =
    include_str!("refused_chirho/declared_default_no_match_chirho.hs");
/// `default ()` disables defaulting.
const DECLARED_EMPTY_DEFAULT_CHIRHO: &str =
    include_str!("refused_chirho/declared_empty_default_chirho.hs");
/// An unused `return` in a library module: no default and no entry role.
const UNRESOLVED_MONAD_CHIRHO: &str = include_str!("refused_chirho/unresolved_monad_chirho.hs");
/// NoMonomorphismRestriction `main = return 7`: the entry's role instantiates it at IO.
const NO_RESTRICTION_RETURN_ENTRY_CHIRHO: &str =
    include_str!("accepted_chirho/no_restriction_return_entry_chirho.hs");
/// NoMonomorphismRestriction `main = print 7 >> return 7`: GHC prints 7.
const NO_RESTRICTION_PRINTING_ENTRY_CHIRHO: &str =
    include_str!("accepted_chirho/no_restriction_printing_entry_chirho.hs");
/// `main :: Num a => IO a`: GHC instantiates and defaults it, then prints 1.
const OVERLOADED_ENTRY_CHIRHO: &str = include_str!("accepted_chirho/overloaded_entry_chirho.hs");
/// A restricted String binding leaves `Read a, Show a` on a variable its type never holds.
const AMBIGUOUS_READ_SHOW_CHIRHO: &str =
    include_str!("refused_chirho/ambiguous_read_show_chirho.hs");
/// `main = 7` as a program: no IO action.
const PURE_PROGRAM_ENTRY_CHIRHO: &str = include_str!("refused_chirho/pure_program_entry_chirho.hs");
/// `main :: Int` as a program: the signature does not make it an action.
const SIGNED_PURE_PROGRAM_ENTRY_CHIRHO: &str =
    include_str!("refused_chirho/signed_pure_program_entry_chirho.hs");
/// NoMonomorphismRestriction `main = 7` as a program: no `Num (IO t)` instance.
const NO_RESTRICTION_PURE_ENTRY_CHIRHO: &str =
    include_str!("refused_chirho/no_restriction_pure_entry_chirho.hs");

#[test]
fn a_restricted_io_entry_runs_its_action_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "restricted_io_entry_chirho.hs",
        RESTRICTED_IO_ENTRY_CHIRHO,
        "hello\nworld\n",
    );
}

#[test]
fn a_restricted_action_entry_runs_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "restricted_action_entry_chirho.hs",
        RESTRICTED_ACTION_ENTRY_CHIRHO,
        "1\n",
    );
}

#[test]
fn a_later_use_fixes_a_restricted_binding_chirho() {
    assert_prints_everywhere_chirho(
        "restricted_fixed_by_use_chirho.hs",
        RESTRICTED_FIXED_BY_USE_CHIRHO,
        "Just 5\n",
    );
}

#[test]
fn a_use_fixes_the_programs_own_num_through_its_own_instance_chirho() {
    assert_prints_everywhere_chirho("source_num_fixed_chirho.hs", SOURCE_NUM_FIXED_CHIRHO, "8\n");
}

#[test]
fn a_restricted_fractional_binding_defaults_to_double_chirho() {
    assert_prints_everywhere_chirho(
        "restricted_double_default_chirho.hs",
        RESTRICTED_DOUBLE_DEFAULT_CHIRHO,
        "2.5\n",
    );
}

#[test]
fn a_declared_default_list_chooses_the_restricted_type_chirho() {
    assert_prints_everywhere_chirho(
        "declared_double_default_chirho.hs",
        DECLARED_DOUBLE_DEFAULT_CHIRHO,
        "5.0\n",
    );
}

#[test]
fn function_syntax_is_never_restricted_chirho() {
    assert_prints_everywhere_chirho(
        "function_binding_generalizes_chirho.hs",
        FUNCTION_BINDING_GENERALIZES_CHIRHO,
        "(2,3.5)\n",
    );
}

#[test]
fn a_signature_frees_a_simple_pattern_binding_chirho() {
    assert_prints_everywhere_chirho(
        "signed_simple_pattern_chirho.hs",
        SIGNED_SIMPLE_PATTERN_CHIRHO,
        "(2,3.5)\n",
    );
}

#[test]
fn a_signed_member_frees_its_recursive_group_chirho() {
    assert_prints_everywhere_chirho(
        "signed_scc_member_chirho.hs",
        SIGNED_SCC_MEMBER_CHIRHO,
        "(True,False)\n",
    );
}

#[test]
fn the_executable_entry_role_makes_a_bare_return_io_chirho() {
    assert_prints_everywhere_chirho(
        "executable_entry_return_chirho.hs",
        EXECUTABLE_ENTRY_RETURN_CHIRHO,
        "",
    );
}

#[test]
fn a_restricted_list_reduces_through_its_instance_chirho() {
    assert_prints_everywhere_chirho(
        "restricted_list_shown_chirho.hs",
        RESTRICTED_LIST_SHOWN_CHIRHO,
        "[1,2,3]\n",
    );
}

#[test]
fn a_restricted_binding_has_one_type_chirho() {
    rejected_like_ghc_chirho(
        RESTRICTED_TWO_TYPES_CHIRHO,
        "restricted_two_types_chirho.hs",
        &["Fractional"],
    );
}

#[test]
fn a_restricted_monoid_without_a_use_is_ambiguous_chirho() {
    rejected_like_ghc_chirho(
        RESTRICTED_AMBIGUOUS_MONOID_CHIRHO,
        "restricted_ambiguous_monoid_chirho.hs",
        &["Monoid"],
    );
}

#[test]
fn the_programs_own_num_is_never_defaulted_chirho() {
    rejected_like_ghc_chirho(
        SOURCE_NUM_UNFIXED_CHIRHO,
        "source_num_unfixed_chirho.hs",
        &["Num"],
    );
}

#[test]
fn an_unsigned_lambda_binding_has_one_type_chirho() {
    rejected_like_ghc_chirho(
        LAMBDA_SIMPLE_PATTERN_CHIRHO,
        "lambda_simple_pattern_chirho.hs",
        &[],
    );
}

#[test]
fn an_unsigned_lambda_binding_has_one_type_at_int_and_integer_chirho() {
    rejected_like_ghc_chirho(
        LAMBDA_INTEGER_PATTERN_CHIRHO,
        "lambda_integer_pattern_chirho.hs",
        &[],
    );
}

#[test]
fn one_restricted_member_restricts_its_whole_group_chirho() {
    rejected_like_ghc_chirho(MIXED_SCC_CHIRHO, "mixed_scc_chirho.hs", &[]);
}

#[test]
fn a_pattern_binding_stays_restricted_despite_signatures_chirho() {
    rejected_like_ghc_chirho(
        SIGNED_PATTERN_BINDING_CHIRHO,
        "signed_pattern_binding_chirho.hs",
        &[],
    );
}

#[test]
fn a_declared_default_list_without_a_match_never_falls_back_chirho() {
    rejected_like_ghc_chirho(
        DECLARED_DEFAULT_NO_MATCH_CHIRHO,
        "declared_default_no_match_chirho.hs",
        &["Fractional"],
    );
}

#[test]
fn an_empty_default_list_disables_defaulting_chirho() {
    rejected_like_ghc_chirho(
        DECLARED_EMPTY_DEFAULT_CHIRHO,
        "declared_empty_default_chirho.hs",
        &["Num"],
    );
}

#[test]
fn an_unused_restricted_return_has_no_monad_default_chirho() {
    rejected_like_ghc_chirho(
        UNRESOLVED_MONAD_CHIRHO,
        "unresolved_monad_chirho.hs",
        &["Monad"],
    );
}

#[test]
fn an_unrestricted_return_entry_is_instantiated_at_io_chirho() {
    assert_prints_everywhere_chirho(
        "no_restriction_return_entry_chirho.hs",
        NO_RESTRICTION_RETURN_ENTRY_CHIRHO,
        "",
    );
}

#[test]
fn an_unrestricted_entry_runs_at_its_instantiated_type_chirho() {
    assert_prints_everywhere_chirho(
        "no_restriction_printing_entry_chirho.hs",
        NO_RESTRICTION_PRINTING_ENTRY_CHIRHO,
        "7\n",
    );
}

#[test]
fn an_overloaded_entry_runs_at_its_instantiated_type_chirho() {
    assert_prints_everywhere_chirho("overloaded_entry_chirho.hs", OVERLOADED_ENTRY_CHIRHO, "1\n");
}

#[test]
fn a_pure_program_entry_is_refused_chirho() {
    rejected_as_program_like_ghc_chirho(
        PURE_PROGRAM_ENTRY_CHIRHO,
        "pure_program_entry_chirho.hs",
        &["IO"],
    );
}

#[test]
fn a_signed_pure_program_entry_is_refused_chirho() {
    rejected_as_program_like_ghc_chirho(
        SIGNED_PURE_PROGRAM_ENTRY_CHIRHO,
        "signed_pure_program_entry_chirho.hs",
        &["IO"],
    );
}

#[test]
fn an_unrestricted_pure_program_entry_is_refused_chirho() {
    rejected_as_program_like_ghc_chirho(
        NO_RESTRICTION_PURE_ENTRY_CHIRHO,
        "no_restriction_pure_entry_chirho.hs",
        &["Num"],
    );
}

#[test]
fn a_restricted_groups_unconnected_wanteds_are_still_ambiguous_chirho() {
    rejected_as_program_like_ghc_chirho(
        AMBIGUOUS_READ_SHOW_CHIRHO,
        "ambiguous_read_show_chirho.hs",
        &["Read"],
    );
}
