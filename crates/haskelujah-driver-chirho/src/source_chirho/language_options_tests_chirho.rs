// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Paired GHC 9.14.1 contracts for the common language producer and real source entry.

use crate::{DiagnosticBundleChirho, FrontendOptionsChirho, FrontendResultChirho, SourceMapChirho};
use haskelujah_parser_chirho::pragma_chirho::extension_enabled_chirho;

pub(super) fn check_chirho(
    source_chirho: &str,
    options_chirho: &[&str],
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    let mut map_chirho = SourceMapChirho::new_chirho();
    super::typecheck_source_with_ifaces_and_options_chirho(
        source_chirho,
        &mut map_chirho,
        "OptionsChirho.hs",
        &FrontendOptionsChirho::from_ghc_options_chirho(options_chirho.iter().copied()),
    )
    .map(|(frontend_chirho, _ifaces_chirho)| frontend_chirho)
}

#[test]
fn default_edition_and_caller_modes_reach_actual_kind_elaboration_chirho() {
    let source_chirho = "module PolyHeaderChirho where\ndata ProxyChirho (a :: k) = ProxyChirho\n";
    let default_chirho = check_chirho(source_chirho, &[]).unwrap();
    assert!(extension_enabled_chirho(
        &default_chirho.module_chirho.extensions_chirho,
        "PolyKinds"
    ));
    assert!(check_chirho(source_chirho, &["-XNoPolyKinds"]).is_err());
    assert!(check_chirho(source_chirho, &["-fdefer-type-errors", "-XNoPolyKinds"]).is_err());
    assert!(check_chirho(source_chirho, &["-XHaskell2010", "-XKindSignatures"]).is_err());
}

#[test]
fn explicit_overrides_survive_a_later_language_edition_chirho() {
    let off_chirho = "{-# LANGUAGE NoPolyKinds, GHC2024 #-}\nmodule ExplicitPolyOffChirho where\ndata ProxyChirho (a :: k) = ProxyChirho\n";
    assert!(check_chirho(off_chirho, &[]).is_err());
    let on_chirho = "{-# LANGUAGE Haskell2010, PolyKinds, KindSignatures #-}\nmodule ExplicitPolyOnChirho where\ndata ProxyChirho (a :: k) = ProxyChirho\n";
    assert!(check_chirho(on_chirho, &[]).is_ok());
}

#[test]
fn caller_recursive_do_is_used_before_layout_and_published_after_lowering_chirho() {
    let source_chirho =
        "module RecursiveCallerChirho where\nvalueChirho :: IO ()\nvalueChirho = mdo\n  pure ()\n";
    let enabled_chirho = check_chirho(source_chirho, &["-XRecursiveDo"]).unwrap();
    assert!(extension_enabled_chirho(
        &enabled_chirho.module_chirho.extensions_chirho,
        "RecursiveDo"
    ));
    assert!(check_chirho(source_chirho, &[]).is_err());
}

#[test]
fn caller_cpp_selects_the_real_post_cpp_header_chirho() {
    let source_chirho = "#if 1\n{-# LANGUAGE Haskell2010 #-}\n#endif\nmodule CppCallerChirho where\nvalueChirho :: ()\nvalueChirho = ()\n";
    let enabled_chirho = check_chirho(source_chirho, &["-XCPP"]).unwrap();
    assert!(extension_enabled_chirho(
        &enabled_chirho.module_chirho.extensions_chirho,
        "Haskell2010"
    ));
    assert!(!extension_enabled_chirho(
        &enabled_chirho.module_chirho.extensions_chirho,
        "PolyKinds"
    ));
    assert!(check_chirho(source_chirho, &[]).is_err());
}

#[test]
fn cpp_mentions_in_comments_and_strings_do_not_authorize_preprocessing_chirho() {
    let source_chirho = "-- {-# LANGUAGE CPP #-}\nmodule M where\ntextChirho = \"{-# LANGUAGE CPP #-}\"\n#define KEEP_CHIRHO 1\n";
    assert!(crate::preprocess_cpp_chirho(source_chirho).contains("#define KEEP_CHIRHO 1"));
    let disabled_chirho = "{-# LANGUAGE CPP, NoCPP #-}\nmodule M where\n#define KEEP_CHIRHO 1\n";
    assert!(crate::preprocess_cpp_chirho(disabled_chirho).contains("#define KEEP_CHIRHO 1"));
}

#[test]
fn real_headers_and_existing_deferral_categories_share_the_resolved_payload_chirho() {
    let source_chirho = "{-# OPTIONS_GHC -fno-defer-type-errors -XNoPolyKinds #-}\nmodule M where\nvalueChirho :: Int\nvalueChirho = True\n";
    assert!(check_chirho(source_chirho, &["-fdefer-type-errors", "-XPolyKinds"]).is_err());
    let deferred_chirho = check_chirho(
        "{-# OPTIONS_GHC -fdefer-type-errors -XNoPolyKinds #-}\nmodule M where\nvalueChirho :: Int\nvalueChirho = True\n",
        &[],
    ).unwrap();
    assert!(!extension_enabled_chirho(
        &deferred_chirho.module_chirho.extensions_chirho,
        "PolyKinds"
    ));
    assert!(!deferred_chirho.warnings_chirho.is_empty());
}

#[test]
fn no_poly_kinds_refuses_bound_kind_variables_but_retains_concrete_kinds_chirho() {
    let bound_chirho = "{-# LANGUAGE NoPolyKinds, KindSignatures #-}\nmodule BoundKindVariableOffChirho where\ndata ProxyChirho k (a :: k) = ProxyChirho\n";
    assert!(check_chirho(bound_chirho, &[]).is_err());
    let forall_chirho = "{-# LANGUAGE NoPolyKinds, ExplicitForAll, StandaloneKindSignatures #-}\nmodule ExplicitKindForallOffChirho where\nimport Data.Kind (Type)\ntype ProxyChirho :: forall k. k -> Type\ndata ProxyChirho a = ProxyChirho\n";
    assert!(check_chirho(forall_chirho, &[]).is_err());
    let concrete_chirho = "{-# LANGUAGE NoPolyKinds, DataKinds, KindSignatures #-}\nmodule ConcreteKindOffChirho where\nimport GHC.TypeLits (Nat)\ndata ProxyChirho (n :: Nat) = ProxyChirho\n";
    assert!(check_chirho(concrete_chirho, &[]).is_ok());
}

#[test]
fn standalone_kind_quantifiers_do_not_bind_h98_constructor_fields_chirho() {
    let source_chirho = "{-# LANGUAGE KindSignatures, PolyKinds, StandaloneKindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\ntype AChirho :: forall kChirho. kChirho -> Type\ndata AChirho aChirho = AChirho (ProxyChirho kChirho)\ndata ProxyChirho aChirho = ProxyChirho\nmain :: IO ()\nmain = pure ()\n";
    assert!(check_chirho(source_chirho, &[]).is_err());
}

#[test]
fn existential_syntax_requires_written_binders_for_ordinary_and_record_fields_chirho() {
    for field_chirho in ["BoxChirho b", "BoxChirho { fieldChirho :: b }"] {
        let free_chirho = format!(
            "{{-# LANGUAGE ExistentialQuantification #-}}\nmodule M where\ndata BoxChirho a = {field_chirho}\n"
        );
        assert!(check_chirho(&free_chirho, &[]).is_err());
        let bound_chirho = format!(
            "{{-# LANGUAGE ExistentialQuantification #-}}\nmodule M where\ndata BoxChirho a = forall b. {field_chirho}\n"
        );
        assert!(check_chirho(&bound_chirho, &[]).is_ok());
    }
}

#[test]
fn documented_aliases_control_layout_and_ordinary_scoped_variables_chirho() {
    let recursive_chirho =
        "module RecursiveCallerChirho where\nvalueChirho :: IO ()\nvalueChirho = mdo\n  pure ()\n";
    assert!(check_chirho(recursive_chirho, &["-XDoRec"]).is_ok());
    assert!(check_chirho(recursive_chirho, &["-XNoDoRec"]).is_err());
    let alias_chirho = "{-# LANGUAGE Haskell2010, PatternSignatures #-}\nmodule PatternSignaturesAliasChirho where\nvalueChirho :: forall a. a -> a\nvalueChirho x = (x :: a)\n";
    assert!(check_chirho(alias_chirho, &[]).is_ok());
    let abstraction_chirho = "{-# LANGUAGE Haskell2010, TypeAbstractions, ExplicitForAll #-}\nmodule TypeAbstractionsDoesNotScopeChirho where\nvalueChirho :: forall a. a -> a\nvalueChirho x = (x :: a)\n";
    assert!(check_chirho(abstraction_chirho, &[]).is_err());
    assert!(check_chirho(abstraction_chirho, &["-XScopedTypeVariables"]).is_ok());
}

#[test]
fn datatype_context_flag_does_not_license_a_polymorphic_field_chirho() {
    let field_chirho = "{-# LANGUAGE Haskell2010, ExplicitForAll #-}\nmodule LegacyExplicitRankDisabledChirho where\nnewtype BoxChirho = BoxChirho (forall aChirho. aChirho -> aChirho)\n";
    let head_chirho = "{-# LANGUAGE Haskell2010, ExplicitForAll #-}\nmodule DatatypeContextRankDisabledChirho where\ndata Eq aChirho => BoxChirho aChirho = BoxChirho (forall bChirho. bChirho -> bChirho)\n";
    let outcomes_chirho = [
        check_chirho(field_chirho, &[]).is_err(),
        check_chirho(head_chirho, &[]).is_err(),
    ];
    assert_eq!(outcomes_chirho, [true, true]);
}

#[test]
fn datatype_head_context_and_enabled_rank_field_remain_accepted_chirho() {
    let head_chirho = "{-# LANGUAGE Haskell2010 #-}\nmodule DatatypeContextMonomorphicChirho where\ndata Eq aChirho => BoxChirho aChirho = BoxChirho aChirho\n";
    let field_chirho = "{-# LANGUAGE Haskell2010, RankNTypes #-}\nmodule LegacyRankEnabledChirho where\nnewtype BoxChirho = BoxChirho (forall aChirho. aChirho -> aChirho)\n";
    let outcomes_chirho = [
        check_chirho(head_chirho, &[]).is_ok(),
        check_chirho(field_chirho, &[]).is_ok(),
    ];
    assert_eq!(outcomes_chirho, [true, true]);
}

#[test]
fn effective_rank_flag_overrides_default_edition_and_deferral_chirho() {
    let source_chirho = "module ModernRankDefaultChirho where\nnewtype BoxChirho = BoxChirho (forall aChirho. aChirho -> aChirho)\n";
    let outcomes_chirho = [
        check_chirho(source_chirho, &[]).is_ok(),
        check_chirho(source_chirho, &["-XNoRankNTypes"]).is_err(),
        check_chirho(source_chirho, &["-XNoRankNTypes", "-fdefer-type-errors"]).is_err(),
        check_chirho(source_chirho, &["-XNoRankNTypes", "-XRankNTypes"]).is_ok(),
    ];
    assert_eq!(outcomes_chirho, [true, true, true, true]);
}

#[test]
fn pattern_result_restriction_uses_written_arity_and_checked_binder_scope_chirho() {
    let cases_chirho = [
        (
            "{-# language PatternSynonyms #-}\nmodule PatSynExistential where\npattern P :: () => forall x. x -> Maybe x\npattern P <- _\n",
            false,
        ),
        (
            "{-# LANGUAGE PatternSynonyms #-}\nmodule PatternUniversalResultChirho where\npattern PChirho :: forall aChirho. aChirho -> Maybe aChirho\npattern PChirho aChirho = Just aChirho\n",
            true,
        ),
        (
            "{-# LANGUAGE PatternSynonyms, GADTs #-}\nmodule PatternExistentialArgumentChirho where\ndata SomeChirho where\n  SomeChirho :: aChirho -> SomeChirho\npattern PChirho :: () => forall aChirho. aChirho -> SomeChirho\npattern PChirho aChirho <- SomeChirho aChirho\n",
            true,
        ),
        (
            "{-# LANGUAGE PatternSynonyms #-}\nmodule PatternUniversalFunctionChirho where\npattern PChirho :: forall aChirho. aChirho -> Int\npattern PChirho <- _\n",
            true,
        ),
        (
            "{-# LANGUAGE PatternSynonyms #-}\nmodule PatternExistentialFunctionChirho where\npattern PChirho :: () => forall aChirho. aChirho -> Int\npattern PChirho <- _\n",
            false,
        ),
        (
            "{-# LANGUAGE PatternSynonyms #-}\nmodule PatternErasedExistentialChirho where\ntype IgnoreChirho aChirho = Int\npattern PChirho :: () => forall aChirho. IgnoreChirho aChirho\npattern PChirho <- _\n",
            false,
        ),
        (
            "{-# LANGUAGE PatternSynonyms #-}\n{-# OPTIONS_GHC -fdefer-type-errors #-}\nmodule PatternExistentialDeferredChirho where\npattern PChirho :: () => forall aChirho. aChirho -> Maybe aChirho\npattern PChirho <- _\n",
            false,
        ),
    ];
    let outcomes_chirho = cases_chirho.map(|(source_chirho, expected_chirho)| {
        let result_chirho = check_chirho(source_chirho, &[])
            .map(|_| ())
            .map_err(|error_chirho| format!("{error_chirho:?}"));
        (expected_chirho, result_chirho)
    });
    assert!(
        outcomes_chirho
            .iter()
            .all(|(expected_chirho, result_chirho)| *expected_chirho == result_chirho.is_ok()),
        "{outcomes_chirho:?}"
    );
}

#[test]
fn pattern_alias_cursor_consumes_only_declared_arguments_chirho() {
    let cases_chirho = [
        (
            r#####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PatternSynonyms, GADTs #-}
module PatternAliasArgumentChirho where
data SomeChirho where
  SomeChirho :: aChirho -> SomeChirho
type ArgumentChirho aChirho = aChirho -> SomeChirho
pattern PChirho :: () => forall aChirho. ArgumentChirho aChirho
pattern PChirho aChirho <- SomeChirho aChirho
"#####,
            true,
        ),
        (
            r#####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PatternSynonyms #-}
module PatternAliasResultChirho where
type IgnoreChirho aChirho = Int
type ArgumentChirho aChirho = Int -> IgnoreChirho aChirho
pattern PChirho :: () => forall aChirho. ArgumentChirho aChirho
pattern PChirho xChirho <- xChirho
"#####,
            false,
        ),
        (
            r#####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PatternSynonyms, GADTs #-}
module PatternRepeatedCallerAliasChirho where
data SomeChirho where
  SomeChirho :: aChirho -> SomeChirho
type KeepChirho aChirho = aChirho
type ArgumentChirho aChirho = aChirho -> SomeChirho
pattern PChirho :: () => forall aChirho. ArgumentChirho (KeepChirho (KeepChirho aChirho))
pattern PChirho aChirho <- SomeChirho aChirho
"#####,
            true,
        ),
        (
            r#####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PatternSynonyms, GADTs, PolyKinds, KindSignatures, ExplicitForAll #-}
module PatternHiddenPrefixChirho where
import Data.Proxy
data SomeChirho where
  SomeChirho :: forall kChirho (aChirho :: kChirho). Proxy aChirho -> SomeChirho
type ArgumentChirho (aChirho :: kChirho) = Proxy aChirho -> SomeChirho
pattern PChirho :: () => forall kChirho (aChirho :: kChirho). ArgumentChirho aChirho
pattern PChirho aChirho <- SomeChirho aChirho
"#####,
            true,
        ),
        (
            r#####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PatternSynonyms, RankNTypes, ConstraintKinds, FlexibleContexts, ViewPatterns #-}
module PatternClassHeadImageTypedChirho where
data SomeChirho
type ArgumentChirho cChirho aChirho = Int -> ((cChirho Int => Int) -> SomeChirho)
viewChirho :: ((Eq Int => Int) -> SomeChirho) -> Int
viewChirho _ = 1
pattern PChirho :: () => forall aChirho. ArgumentChirho Eq aChirho
pattern PChirho xChirho <- (viewChirho -> xChirho)
"#####,
            true,
        ),
    ];
    let outcomes_chirho = cases_chirho.map(|(source_chirho, expected_chirho)| {
        let result_chirho = check_chirho(source_chirho, &[])
            .map(|_| ())
            .map_err(|error_chirho| format!("{error_chirho:?}"));
        (expected_chirho, result_chirho)
    });
    assert!(
        outcomes_chirho
            .iter()
            .all(|(expected_chirho, result_chirho)| *expected_chirho == result_chirho.is_ok()),
        "{outcomes_chirho:?}"
    );
}
