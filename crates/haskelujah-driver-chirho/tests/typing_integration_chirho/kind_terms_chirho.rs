// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-level kind terms: independently checked under GHC 9.14.1.
use super::{SourceMapChirho, assert_compile_success_chirho};

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
