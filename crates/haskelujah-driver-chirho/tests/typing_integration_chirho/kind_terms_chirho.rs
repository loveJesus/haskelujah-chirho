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
