// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The language baselines and explicit extension implications of GHC 9.14.1.
//! Sources: compiler/GHC/Driver/DynFlags.hs `languageExtensions` and
//! compiler/GHC/Driver/Flags.hs `impliedXFlags` at tag ghc-9.14.1-release.
//! Baselines deliberately do not take the implication closure of their members.

const HASKELL98_CHIRHO: &[&str] = &[
    "Haskell98",
    "ImplicitPrelude",
    "StarIsType",
    "CUSKs",
    "MonomorphismRestriction",
    "NPlusKPatterns",
    "DatatypeContexts",
    "TraditionalRecordSyntax",
    "FieldSelectors",
    "NondecreasingIndentation",
    "DeepSubsumption",
    "ListTuplePuns",
    "ImplicitStagePersistence",
];
const HASKELL2010_CHIRHO: &[&str] = &[
    "Haskell2010",
    "ImplicitPrelude",
    "StarIsType",
    "CUSKs",
    "MonomorphismRestriction",
    "DatatypeContexts",
    "TraditionalRecordSyntax",
    "EmptyDataDecls",
    "ForeignFunctionInterface",
    "PatternGuards",
    "DoAndIfThenElse",
    "FieldSelectors",
    "RelaxedPolyRec",
    "DeepSubsumption",
    "ListTuplePuns",
    "ImplicitStagePersistence",
];
const GHC2021_CHIRHO: &[&str] = &[
    "GHC2021",
    "ImplicitPrelude",
    "StarIsType",
    "MonomorphismRestriction",
    "TraditionalRecordSyntax",
    "EmptyDataDecls",
    "ForeignFunctionInterface",
    "PatternGuards",
    "DoAndIfThenElse",
    "FieldSelectors",
    "RelaxedPolyRec",
    "ListTuplePuns",
    "BangPatterns",
    "BinaryLiterals",
    "ConstrainedClassMethods",
    "ConstraintKinds",
    "DeriveDataTypeable",
    "DeriveFoldable",
    "DeriveFunctor",
    "DeriveGeneric",
    "DeriveLift",
    "DeriveTraversable",
    "EmptyCase",
    "EmptyDataDeriving",
    "ExistentialQuantification",
    "ExplicitForAll",
    "FlexibleContexts",
    "FlexibleInstances",
    "GADTSyntax",
    "GeneralizedNewtypeDeriving",
    "HexFloatLiterals",
    "ImportQualifiedPost",
    "InstanceSigs",
    "KindSignatures",
    "MultiParamTypeClasses",
    "NamedFieldPuns",
    "NamedWildCards",
    "NumericUnderscores",
    "PolyKinds",
    "PostfixOperators",
    "RankNTypes",
    "ScopedTypeVariables",
    "StandaloneDeriving",
    "StandaloneKindSignatures",
    "TupleSections",
    "TypeApplications",
    "TypeOperators",
    "TypeSynonymInstances",
    "ImplicitStagePersistence",
];
const GHC2024_ADDITIONS_CHIRHO: &[&str] = &[
    "DataKinds",
    "DerivingStrategies",
    "DisambiguateRecordFields",
    "ExplicitNamespaces",
    "GADTs",
    "MonoLocalBinds",
    "LambdaCase",
    "RoleAnnotations",
];

/// The first item records the selected edition for existing edition-aware consumers.
pub(super) fn edition_flags_chirho(name_chirho: &str) -> Option<(&'static str, Vec<&'static str>)> {
    match name_chirho {
        "Haskell98" => Some(("Haskell98", HASKELL98_CHIRHO.to_vec())),
        "Haskell2010" => Some(("Haskell2010", HASKELL2010_CHIRHO.to_vec())),
        "GHC2021" => Some(("GHC2021", GHC2021_CHIRHO.to_vec())),
        "GHC2024" => Some((
            "GHC2024",
            std::iter::once("GHC2024")
                .chain(GHC2021_CHIRHO[1..].iter().copied())
                .chain(GHC2024_ADDITIONS_CHIRHO.iter().copied())
                .collect(),
        )),
        _ => None,
    }
}

/// Enabling recurses through a finite acyclic implication graph. Disabling changes only
/// the named flag, preserving earlier implications, as GHC's `setExtensionFlag'` does.
pub(super) fn expand_written_extension_chirho(name_chirho: &str, flags_chirho: &mut Vec<String>) {
    let canonical_chirho = canonical_extension_chirho(name_chirho);
    let name_chirho = canonical_chirho.as_ref();
    flags_chirho.push(name_chirho.to_string());
    let implied_chirho: &[&str] = match name_chirho {
        "RankNTypes"
        | "QuantifiedConstraints"
        | "ScopedTypeVariables"
        | "LiberalTypeSynonyms"
        | "ExistentialQuantification" => &["ExplicitForAll"],
        "FlexibleInstances" => &["TypeSynonymInstances"],
        "FunctionalDependencies" => &["MultiParamTypeClasses"],
        "MultiParamTypeClasses" => &["ConstrainedClassMethods"],
        "TypeFamilyDependencies" => &["TypeFamilies"],
        "RebindableSyntax" => &["NoImplicitPrelude"],
        "DerivingVia" => &["DerivingStrategies"],
        "GADTs" => &["GADTSyntax", "MonoLocalBinds"],
        "TypeFamilies" => &["MonoLocalBinds", "KindSignatures", "ExplicitNamespaces"],
        "PolyKinds" => &["KindSignatures"],
        "TypeInType" => &["DataKinds", "PolyKinds", "KindSignatures"],
        "StandaloneKindSignatures" => &["NoCUSKs"],
        "AutoDeriveTypeable" => &["DeriveDataTypeable"],
        "TypeOperators" => &["ExplicitNamespaces"],
        "ImpredicativeTypes" => &["RankNTypes"],
        "RecordWildCards" | "DuplicateRecordFields" => &["DisambiguateRecordFields"],
        "ParallelArrays" | "MonadComprehensions" => &["ParallelListComp"],
        "JavaScriptFFI" => &["InterruptibleFFI"],
        "DeriveTraversable" => &["DeriveFunctor", "DeriveFoldable"],
        "TemplateHaskell" => &["TemplateHaskellQuotes"],
        "Strict" => &["StrictData"],
        "UnboxedTuples" => &["UnboxedSums"],
        "UnliftedDatatypes" => &["DataKinds", "StandaloneKindSignatures"],
        "LinearTypes" => &["MonoLocalBinds"],
        "ExplicitLevelImports" => &["NoImplicitStagePersistence"],
        _ => &[],
    };
    for flag_chirho in implied_chirho {
        expand_written_extension_chirho(flag_chirho, flags_chirho);
    }
}

/// Documented GHC spellings share one canonical extension state, including No aliases.
pub(super) fn canonical_extension_chirho(name_chirho: &str) -> std::borrow::Cow<'_, str> {
    let (disabled_chirho, base_chirho) = name_chirho
        .strip_prefix("No")
        .map_or((false, name_chirho), |base_chirho| (true, base_chirho));
    let canonical_chirho = match base_chirho {
        "DoRec" => "RecursiveDo",
        "PatternSignatures" => "ScopedTypeVariables",
        "Rank2Types" | "PolymorphicComponents" => "RankNTypes",
        "RecordPuns" => "NamedFieldPuns",
        "GeneralisedNewtypeDeriving" => "GeneralizedNewtypeDeriving",
        _ => return std::borrow::Cow::Borrowed(name_chirho),
    };
    if disabled_chirho {
        std::borrow::Cow::Owned(format!("No{canonical_chirho}"))
    } else {
        std::borrow::Cow::Borrowed(canonical_chirho)
    }
}
