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
