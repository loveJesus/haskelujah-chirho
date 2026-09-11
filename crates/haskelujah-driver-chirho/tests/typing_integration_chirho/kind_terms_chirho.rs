// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-level kind terms: independently checked under GHC 9.14.1.
use super::{SourceMapChirho, assert_compile_success_chirho};

#[test]
fn star_syntax_and_qualified_multiplication_keep_distinct_kinds_chirho() {
    let source_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/StarKindContractsChirho.hs"
    ));
    assert_compile_success_chirho("StarKindContractsChirho.hs", source_chirho);
    let no_star_chirho =
        source_chirho.replace("LANGUAGE DataKinds", "LANGUAGE NoStarIsType, DataKinds");
    let errors_chirho = haskelujah_driver::typecheck_source_chirho(
        &no_star_chirho,
        &mut SourceMapChirho::new_chirho(),
        "UnboundStarChirho.hs",
    )
    .err()
    .expect("NoStarIsType requires an actual operator binding");
    assert!(
        errors_chirho
            .diagnostics_chirho()
            .iter()
            .any(|error_chirho| {
                error_chirho.code_chirho
                    == Some(haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(101))
                    && error_chirho.message_chirho.contains('*')
            }),
        "{errors_chirho}"
    );
    assert_compile_success_chirho(
        "QualifiedMultiplicationChirho.hs",
        &no_star_chirho.replace(":: * -> *", ":: Type -> Type"),
    );
}

#[test]
fn frontend_type_binder_type_runtime_rep_annotation_preserves_newtype_application_kind_chirho() {
    // The former driver test declared its own TYPE. GHC rejects that use;
    // the intended runtime-representation control must import the real TYPE.
    let source_chirho = r#"{-# LANGUAGE PolyKinds, KindSignatures #-}
module CodeKindMiniChirho where
import GHC.Exts (TYPE)
newtype CodeChirho mChirho (aChirho :: TYPE rChirho) = CodeChirho (mChirho aChirho)
valueChirho :: CodeChirho Maybe Int
valueChirho = undefined
"#;
    assert_compile_success_chirho("CodeKindMiniChirho.hs", source_chirho);
    let shadowed_chirho = source_chirho
        .replace("LANGUAGE PolyKinds", "LANGUAGE DataKinds, PolyKinds")
        .replace(
            "import GHC.Exts (TYPE)",
            "data RuntimeRep\ndata TYPE (rChirho :: RuntimeRep)",
        );
    assert_kind_error_chirho(&shadowed_chirho, "TYPE");
}

#[test]
fn required_kind_argument_substitutes_its_term_not_the_kind_of_that_term_chirho() {
    let source_chirho = r#"{-# LANGUAGE PolyKinds, RankNTypes, StandaloneKindSignatures #-}
module DependentChirho where
import Data.Kind (Type)
type ShapeChirho = forall (kChirho :: Type) -> kChirho -> kChirho
type ConsumerChirho :: ShapeChirho -> Type
newtype ConsumerChirho fChirho = ConChirho (fChirho Type Bool)
"#;
    assert_compile_success_chirho("DependentChirho.hs", source_chirho);
    // Both Type and Bool themselves have kind Type, but only the TERM Type
    // makes Int a legal second argument. Freshening k would miss this error.
    assert_kind_error_chirho(
        &source_chirho.replace("fChirho Type Bool", "fChirho Bool Int"),
        "Bool",
    );
}

#[test]
fn nested_required_kind_binders_keep_their_own_argument_scopes_chirho() {
    let source_chirho = r#"{-# LANGUAGE PolyKinds, RankNTypes, StandaloneKindSignatures #-}
module NestedDependentChirho where
import Data.Kind (Type)
type ShapeChirho = forall (outerChirho :: Type) -> outerChirho -> (forall (innerChirho :: Type) -> innerChirho -> Type)
type ConsumerChirho :: ShapeChirho -> Type
newtype ConsumerChirho fChirho = ConChirho (fChirho Type Bool Type Int)
"#;
    assert_compile_success_chirho("NestedDependentChirho.hs", source_chirho);
}

#[test]
fn a_family_used_as_a_kind_is_not_a_fresh_hole_at_each_application_chirho() {
    let declarations_chirho = r#"{-# LANGUAGE PolyKinds, TypeFamilies, EmptyDataDecls, KindSignatures #-}
module FamilyKindChirho where
import Data.Kind (Type)
type family FamilyChirho aChirho
data IndexedChirho :: FamilyChirho kChirho -> Type
"#;
    let valid_chirho = format!(
        "{declarations_chirho}useIndexedChirho :: IndexedChirho aChirho -> IndexedChirho aChirho\nuseIndexedChirho = id\n"
    );
    assert_compile_success_chirho("FamilyKindChirho.hs", &valid_chirho);
    let invalid_chirho = format!(
        "{declarations_chirho}useIndexedChirho :: IndexedChirho Maybe -> IndexedChirho Int -> Type\nuseIndexedChirho = undefined\n"
    );
    assert_kind_error_chirho(&invalid_chirho, "FamilyChirho");
}

#[test]
fn runtime_kind_consumers_accept_unlifted_values_without_accepting_bad_representations_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, StandaloneKindSignatures, UnliftedDatatypes, MagicHash #-}
module RuntimeChirho where
import GHC.Exts (TYPE, LiftedRep, UnliftedType, Int#)
type BoxChirho :: TYPE LiftedRep
data BoxChirho = BoxChirho Int
type RawChirho :: UnliftedType
data RawChirho = RawChirho Int#
identityChirho :: RawChirho -> RawChirho
identityChirho xChirho = xChirho
"#;
    assert_compile_success_chirho("RuntimeChirho.hs", source_chirho);
    assert_compile_success_chirho(
        "RuntimeChirho.hs",
        &source_chirho.replace(
            "import GHC.Exts (TYPE, LiftedRep, UnliftedType, Int#)",
            "import GHC.Exts (TYPE, LiftedRep, Int#)\nimport GHC.Types (UnliftedType)",
        ),
    );
    assert_kind_error_chirho(
        r#"{-# LANGUAGE DataKinds, StandaloneKindSignatures, GADTs #-}
module WrongRuntimeChirho where
import GHC.Exts (TYPE)
type BadChirho :: TYPE Bool
data BadChirho = BadChirho
"#,
        "RuntimeRep",
    );
    assert_kind_error_chirho(
        r#"{-# LANGUAGE MagicHash #-}
module BoxedListRejectsPrimitiveChirho where
import GHC.Exts (Int#)
data BadChirho = BadChirho [Int#]
"#,
        "list element",
    );
    assert_kind_error_chirho(
        r#"{-# LANGUAGE StandaloneKindSignatures #-}
module RuntimeDoesNotInventHeadArgumentsChirho where
import Data.Kind (Type)
type BadChirho :: Type -> Type
data BadChirho = BadChirho
"#,
        "kind",
    );
}

#[test]
fn nominal_binder_kinds_constrain_arguments_instead_of_becoming_quantified_holes_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, KindSignatures #-}
module NominalKindsChirho where
data ColorChirho = RedChirho
data IndexedChirho (colorChirho :: ColorChirho) = IndexedChirho
useChirho :: IndexedChirho 'RedChirho -> Int
useChirho _ = 42
"#;
    assert_compile_success_chirho("NominalKindsChirho.hs", source_chirho);
    assert_kind_error_chirho(
        &source_chirho.replace("IndexedChirho 'RedChirho ->", "IndexedChirho Int ->"),
        "ColorChirho",
    );
}

#[test]
fn missing_nominal_kinds_are_reported_in_declaration_and_forall_binders_chirho() {
    for declaration_chirho in [
        "data IndexedChirho (colorChirho :: MissingKindChirho) = IndexedChirho",
        "useChirho :: forall (colorChirho :: MissingKindChirho). Int\nuseChirho = 42",
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE DataKinds, KindSignatures, ExplicitForAll #-}}\nmodule MissingKindChirho where\n{declaration_chirho}\n"
        );
        let errors_chirho = haskelujah_driver::typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "MissingKindChirho.hs",
        )
        .err()
        .expect("a nominal kind must be in scope");
        assert!(
            errors_chirho
                .diagnostics_chirho()
                .iter()
                .any(|error_chirho| error_chirho.code_chirho
                    == Some(haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(101))
                    && error_chirho.message_chirho.contains("MissingKindChirho")),
            "{errors_chirho}"
        );
    }
}

#[test]
fn qualified_builtin_kinds_follow_declared_import_aliases_chirho() {
    let source_chirho = r#"{-# LANGUAGE KindSignatures #-}
module AliasedTypeChirho where
import qualified Data.Kind as KChirho
data BoxChirho (fChirho :: KChirho.Type -> KChirho.Type) = BoxChirho (fChirho Int)
useChirho :: BoxChirho Maybe -> Int
useChirho _ = 42
"#;
    assert_compile_success_chirho("AliasedTypeChirho.hs", source_chirho);
    assert_kind_error_chirho(
        &source_chirho.replace("BoxChirho Maybe ->", "BoxChirho Int ->"),
        "type application",
    );
    assert_kind_error_chirho(
        r#"{-# LANGUAGE StandaloneKindSignatures, UnliftedDatatypes #-}
module WrongAliasedRuntimeChirho where
import qualified GHC.Exts as RuntimeChirho
type BadChirho :: RuntimeChirho.TYPE Bool
data BadChirho = BadChirho
"#,
        "RuntimeRep",
    );
}

#[test]
fn prefix_function_kind_accepts_primitive_arguments_but_defaults_inferred_representations_chirho() {
    let source_chirho = r#"{-# LANGUAGE MagicHash #-}
module PrimitiveArrowChirho where
import GHC.Exts (Int#)
type ArrowChirho = (->) Int#
constantChirho :: ArrowChirho Int
constantChirho _ = 42
"#;
    assert_compile_success_chirho("PrimitiveArrowChirho.hs", source_chirho);
    assert_kind_error_chirho(
        &source_chirho.replace(
            "constantChirho :: ArrowChirho Int\nconstantChirho _ = 42",
            "identityChirho :: ArrowChirho Int#\nidentityChirho xChirho = xChirho",
        ),
        "type application",
    );
    assert_compile_success_chirho(
        "ExplicitPrimitiveArrowChirho.hs",
        r#"{-# LANGUAGE MagicHash #-}
module ExplicitPrimitiveArrowChirho where
import GHC.Exts (Int#)
type ArrowChirho = (->) Int# Int#
identityChirho :: ArrowChirho
identityChirho xChirho = xChirho
"#,
    );
    assert_compile_success_chirho(
        "WrittenRuntimeParameterChirho.hs",
        r#"{-# LANGUAGE MagicHash, PolyKinds, DataKinds #-}
module WrittenRuntimeParameterChirho where
import GHC.Exts (TYPE, Int#)
type ArrowChirho (aChirho :: TYPE rChirho) = aChirho -> Int
constantChirho :: ArrowChirho Int#
constantChirho _ = 42
"#,
    );
    assert_kind_error_chirho(
        r#"{-# LANGUAGE DataKinds #-}
module WrongPrimitiveArrowChirho where
type BadChirho = (->) 'True
"#,
        "Bool",
    );
}

#[test]
fn flexible_kind_application_can_be_an_arrow_but_a_nominal_head_cannot_chirho() {
    let source_chirho = r#"{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, EmptyDataDecls #-}
module FlexibleKindApplicationChirho where
data FlurmpChirho
type family PureChirho (xChirho :: aChirho) :: fChirho aChirho
type ResultChirho = PureChirho FlurmpChirho FlurmpChirho
"#;
    assert_compile_success_chirho("FlexibleKindApplicationChirho.hs", source_chirho);
    assert_kind_error_chirho(
        &source_chirho.replace(":: fChirho aChirho", ":: Maybe aChirho"),
        "Maybe",
    );
}

#[test]
fn dependent_declaration_heads_follow_the_complete_kind_telescope_chirho() {
    let source_chirho = r#"{-# LANGUAGE PolyKinds, RankNTypes, StandaloneKindSignatures #-}
module DependentHeadChirho where
import Data.Kind (Type)
type TChirho :: forall (kChirho :: Type) -> kChirho -> Type
data TChirho kChirho aChirho = MkTChirho
"#;
    assert_compile_success_chirho("DependentHeadChirho.hs", source_chirho);
    assert_kind_error_chirho(
        &source_chirho.replace(
            "data TChirho kChirho aChirho",
            "data TChirho kChirho (aChirho :: Type)",
        ),
        "data declaration signature",
    );
}

#[test]
fn runtime_representation_constructors_have_their_actual_classifier_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, GADTs, StandaloneKindSignatures #-}
module RuntimeClassificationChirho where
import GHC.Exts (TYPE, LiftedRep, Multiplicity(Many))
type BoxChirho :: TYPE LiftedRep
data BoxChirho = MkBoxChirho
"#;
    assert_compile_success_chirho("RuntimeClassificationChirho.hs", source_chirho);
    assert_kind_error_chirho(
        &source_chirho.replace("TYPE LiftedRep", "TYPE Many"),
        "Multiplicity",
    );
}

#[test]
fn tuple_runtime_representations_classify_every_element_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, TypeFamilies #-}
module TupleRepresentationChirho where
import GHC.Exts (TYPE, RuntimeRep(TupleRep, IntRep))
type family CarrierChirho :: TYPE ('TupleRep '[ 'IntRep ])
"#;
    assert_compile_success_chirho("TupleRepresentationChirho.hs", source_chirho);
    assert_kind_error_chirho(
        &source_chirho.replace("'[ 'IntRep ]", "'[ 'True ]"),
        "RuntimeRep",
    );
}

#[test]
fn vector_runtime_representations_distinguish_count_and_element_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, TypeFamilies #-}
module VectorRepresentationChirho where
import GHC.Exts (TYPE, RuntimeRep(VecRep), VecCount(Vec4), VecElem(FloatElemRep))
type family CarrierChirho :: TYPE ('VecRep 'Vec4 'FloatElemRep)
"#;
    assert_compile_success_chirho("VectorRepresentationChirho.hs", source_chirho);
    assert_kind_error_chirho(
        &source_chirho.replace("'Vec4 'FloatElemRep", "'FloatElemRep 'Vec4"),
        "VecCount",
    );
}

#[test]
fn represented_local_instance_heads_use_instantiated_kind_equality_chirho() {
    // No explicit PolyKinds/FlexibleInstances: those names activate a separate
    // historical lowering guard. The default GHC2021 edition is still polykinded.
    let source_chirho = r#"{-# LANGUAGE KindSignatures #-}
module RuntimeKindInstanceDefaultEditionChirho where
import Data.Kind (Type)
import GHC.Exts (TYPE)
class CChirho (fChirho :: Type -> TYPE rChirho)
data FChirho aChirho = MkFChirho aChirho
instance CChirho FChirho
"#;
    assert_compile_success_chirho("RuntimeKindInstanceDefaultEditionChirho.hs", source_chirho);
    assert_kind_error_chirho(
        &source_chirho.replace("instance CChirho FChirho", "instance CChirho Int"),
        "instance head",
    );
}

fn assert_kind_error_chirho(source_chirho: &str, subject_chirho: &str) {
    let errors_chirho = haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongKindTermChirho.hs",
    )
    .err()
    .expect("the independently GHC-rejected kind application must fail");
    assert!(
        errors_chirho
            .diagnostics_chirho()
            .iter()
            .any(|error_chirho| {
                error_chirho.code_chirho
                    == Some(haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300))
                    && error_chirho.message_chirho.contains(subject_chirho)
            }),
        "{errors_chirho}"
    );
}
