// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use crate::cst_parser_chirho::ParserChirho;
use crate::lower_chirho::{lower_module_chirho, lower_module_with_extensions_chirho};

fn policy_chirho(options_chirho: &[&str], source_chirho: &str) -> EffectiveExtensionsChirho {
    let options_chirho: Vec<_> = options_chirho
        .iter()
        .map(|option_chirho| option_chirho.to_string())
        .collect();
    EffectiveExtensionsChirho::from_entries_chirho(
        extension_entries_from_options_chirho(&options_chirho)
            .into_iter()
            .chain(
                header_pragmas_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
                    .extensions_chirho,
            ),
    )
}

#[test]
fn bare_modules_use_the_common_ghc2021_baseline_chirho() {
    let extensions_chirho = policy_chirho(&[], "module M where\nvalueChirho = ()\n");
    assert!(extensions_chirho.enabled_chirho("PolyKinds"));
    assert!(extensions_chirho.enabled_chirho("MonomorphismRestriction"));
    assert!(extensions_chirho.enabled_chirho("FieldSelectors"));
    assert!(!extensions_chirho.enabled_chirho("GADTs"));
    assert!(!extensions_chirho.enabled_chirho("DeepSubsumption"));
    assert!(!extensions_chirho.enabled_chirho("ExplicitNamespaces"));
}

#[test]
fn edition_selection_and_explicit_overrides_have_separate_authority_chirho() {
    for source_chirho in [
        "{-# LANGUAGE NoPolyKinds, GHC2024 #-}\nmodule M where\n",
        "{-# LANGUAGE GHC2024, NoPolyKinds #-}\nmodule M where\n",
        "{-# OPTIONS_GHC -XNoPolyKinds \"-XGHC2024\" #-}\nmodule M where\n",
    ] {
        let extensions_chirho = policy_chirho(&[], source_chirho);
        assert!(!extensions_chirho.enabled_chirho("PolyKinds"));
        assert!(
            !extensions_chirho
                .flags_chirho()
                .iter()
                .any(|flag_chirho| flag_chirho == "PolyKinds")
        );
        assert!(extensions_chirho.enabled_chirho("RoleAnnotations"));
        assert!(extensions_chirho.enabled_chirho("DataKinds"));
    }
    let legacy_chirho = policy_chirho(
        &["-XNoPolyKinds", "-XGHC2024"],
        "{-# LANGUAGE Haskell2010 #-}\nmodule M where\n",
    );
    assert!(!legacy_chirho.enabled_chirho("PolyKinds"));
    assert!(!legacy_chirho.enabled_chirho("RoleAnnotations"));
    assert!(legacy_chirho.enabled_chirho("DeepSubsumption"));
    let override_chirho = policy_chirho(
        &["-XNoPolyKinds"],
        "{-# LANGUAGE Haskell2010, PolyKinds #-}\nmodule M where\n",
    );
    assert!(override_chirho.enabled_chirho("PolyKinds"));
}

#[test]
fn explicit_implications_replay_without_closing_the_edition_baseline_chirho() {
    assert!(!policy_chirho(&[], "").enabled_chirho("ExplicitNamespaces"));
    assert!(policy_chirho(&["-XTypeOperators"], "").enabled_chirho("ExplicitNamespaces"));
    let disabled_chirho = policy_chirho(&["-XTypeFamilies", "-XNoTypeFamilies"], "");
    assert!(!disabled_chirho.enabled_chirho("TypeFamilies"));
    assert!(disabled_chirho.enabled_chirho("MonoLocalBinds"));
    let explicit_off_chirho = policy_chirho(&["-XTypeFamilies", "-XNoMonoLocalBinds"], "");
    assert!(!explicit_off_chirho.enabled_chirho("MonoLocalBinds"));
    assert!(
        !policy_chirho(&["-XRebindableSyntax"], "{-# LANGUAGE GHC2024 #-}")
            .enabled_chirho("ImplicitPrelude")
    );
    assert!(
        policy_chirho(&["-XRebindableSyntax", "-XImplicitPrelude"], "")
            .enabled_chirho("ImplicitPrelude")
    );
}

#[test]
fn only_real_header_pragmas_supply_extensions_chirho() {
    let source_chirho = "-- {-# LANGUAGE RecursiveDo #-}\nmodule M where\ntextChirho = \"{-# LANGUAGE QualifiedDo #-}\"\n{-# LANGUAGE RecursiveDo, NoPolyKinds #-}\n";
    let extensions_chirho = policy_chirho(&[], source_chirho);
    assert!(!extensions_chirho.enabled_chirho("RecursiveDo"));
    assert!(!extensions_chirho.enabled_chirho("QualifiedDo"));
    assert!(extensions_chirho.enabled_chirho("PolyKinds"));
    let quoted_chirho = policy_chirho(
        &[],
        "{-# OPTIONS_GHC '-XRecursiveDo' \"-XQualifiedDo\" #-}\nmodule M where\n",
    );
    assert!(!quoted_chirho.enabled_chirho("RecursiveDo"));
    assert!(quoted_chirho.enabled_chirho("QualifiedDo"));
}

#[test]
fn configured_layout_and_lowering_consume_the_identical_payload_chirho() {
    let source_chirho = "module M where\nvalueChirho = mdo\n  rec\n    itemChirho <- pure itemChirho\n  pure itemChirho\n";
    let extensions_chirho = policy_chirho(&["-XRecursiveDo", "-XNoPolyKinds"], source_chirho);
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let green_chirho =
        ParserChirho::with_extensions_chirho(source_chirho, file_chirho, &extensions_chirho)
            .parse_checked_chirho()
            .unwrap();
    let module_chirho =
        lower_module_with_extensions_chirho(&green_chirho, file_chirho, &extensions_chirho);
    assert_eq!(
        module_chirho.extensions_chirho,
        extensions_chirho.flags_chirho()
    );
    assert!(!extension_enabled_chirho(
        &module_chirho.extensions_chirho,
        "PolyKinds"
    ));
}

#[test]
fn cold_default_lowering_agrees_with_the_parser_header_policy_chirho() {
    let source_chirho = "{-# LANGUAGE Haskell2010, PolyKinds #-}\nmodule M where\nvalueChirho = ()\n{-# LANGUAGE NoPolyKinds #-}\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let extensions_chirho =
        EffectiveExtensionsChirho::from_source_chirho(source_chirho, file_chirho);
    let green_chirho = ParserChirho::new_chirho(source_chirho, file_chirho).parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_chirho);
    assert_eq!(
        module_chirho.extensions_chirho,
        extensions_chirho.flags_chirho()
    );
    assert!(extension_enabled_chirho(
        &module_chirho.extensions_chirho,
        "PolyKinds"
    ));
}

#[test]
fn publication_keeps_one_effective_state_for_presence_based_consumers_chirho() {
    let extensions_chirho = policy_chirho(&["-XRebindableSyntax", "-XImplicitPrelude"], "");
    assert!(
        extensions_chirho
            .flags_chirho()
            .iter()
            .any(|flag_chirho| flag_chirho == "ImplicitPrelude")
    );
    assert!(
        !extensions_chirho
            .flags_chirho()
            .iter()
            .any(|flag_chirho| flag_chirho == "NoImplicitPrelude")
    );
    let indentation_chirho = policy_chirho(&["-XHaskell98"], "");
    assert!(indentation_chirho.enabled_chirho("NondecreasingIndentation"));
}

#[test]
fn ghc_aliases_share_the_canonical_state_and_implication_history_chirho() {
    for (alias_chirho, canonical_chirho) in [
        ("DoRec", "RecursiveDo"),
        ("PatternSignatures", "ScopedTypeVariables"),
        ("Rank2Types", "RankNTypes"),
        ("PolymorphicComponents", "RankNTypes"),
        ("RecordPuns", "NamedFieldPuns"),
        ("GeneralisedNewtypeDeriving", "GeneralizedNewtypeDeriving"),
    ] {
        let on_chirho = format!("-X{alias_chirho}");
        let off_chirho = format!("-XNo{alias_chirho}");
        assert!(policy_chirho(&["-XHaskell2010", &on_chirho], "").enabled_chirho(canonical_chirho));
        assert!(
            !policy_chirho(&["-XGHC2024", &on_chirho, &off_chirho], "")
                .enabled_chirho(canonical_chirho)
        );
    }
    let scoped_chirho = policy_chirho(&[], "{-# LANGUAGE Haskell2010, PatternSignatures #-}");
    assert!(scoped_chirho.enabled_chirho("ExplicitForAll"));
    assert!(
        !policy_chirho(&["-XHaskell2010", "-XTypeAbstractions"], "")
            .enabled_chirho("ScopedTypeVariables")
    );
}
