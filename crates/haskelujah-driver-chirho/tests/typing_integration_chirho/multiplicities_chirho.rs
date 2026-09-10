// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Multiplicity syntax, scope and classifiers; GHC 9.14.1 reference controls.
//! This is not complete linear-use checking or multiplicity-family reduction.
use super::{SourceMapChirho, assert_compile_success_chirho, assert_execution_chirho};
use haskelujah_diagnostics_chirho::ErrorCodeChirho;

#[test]
fn quoted_and_parenthesized_multiplicity_keep_both_value_types_chirho() {
    for annotation_chirho in ["1", "'One", "('One)", "'Many"] {
        assert_compile_success_chirho(
            "QuotedMultiplicityChirho.hs",
            &format!(
                "{{-# LANGUAGE NoDataKinds, LinearTypes #-}}\nmodule QuotedMultiplicityChirho where\nimport GHC.Types (Multiplicity(..))\ntype ArrowChirho = Int %{annotation_chirho} -> Int\nidentityChirho :: ArrowChirho\nidentityChirho xChirho = xChirho\n"
            ),
        );
    }
}

#[test]
fn multiplicity_family_dependencies_do_not_depend_on_text_order_chirho() {
    for declarations_chirho in [
        "type family MulChirho aChirho :: Multiplicity\ntype ArrowChirho aChirho = Int %(MulChirho aChirho) -> Int\n",
        "type ArrowChirho aChirho = Int %(MulChirho aChirho) -> Int\ntype family MulChirho aChirho :: Multiplicity\n",
    ] {
        assert_compile_success_chirho(
            "FamilyMultiplicityChirho.hs",
            &format!(
                "{{-# LANGUAGE LinearTypes, TypeFamilies #-}}\nmodule FamilyMultiplicityChirho where\nimport GHC.Types (Multiplicity)\n{declarations_chirho}"
            ),
        );
    }
}

#[test]
fn arrow_annotations_require_multiplicity_not_a_numeric_or_bool_kind_chirho() {
    for declaration_chirho in [
        "type WrongChirho = Int %2 -> Int",
        "type WrongChirho = Int %(1) -> Int",
        "type WrongChirho = Int %'True -> Int",
        "type WrongChirho (mChirho :: Bool) = Int %mChirho -> Int",
    ] {
        assert_multiplicity_error_chirho(&format!(
            "{{-# LANGUAGE DataKinds, LinearTypes, KindSignatures #-}}\nmodule WrongMultiplicityChirho where\n{declaration_chirho}\n"
        ));
    }
}

#[test]
fn multiplicity_import_members_obey_promotion_licensing_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, LinearTypes #-}
module PlainMultiplicityChirho where
import GHC.Types (Multiplicity(..))
type LinearChirho = Int %One -> Int
type UnrestrictedChirho = Int %Many -> Int
"#;
    assert_compile_success_chirho("PlainMultiplicityChirho.hs", source_chirho);
    assert_missing_name_chirho(&source_chirho.replace("DataKinds", "NoDataKinds"), "One");
}

#[test]
fn multiplicity_scope_checks_the_annotation_and_the_next_declaration_chirho() {
    let source_chirho = r#"{-# LANGUAGE LinearTypes #-}
module UnknownMultiplicityCanaryChirho where
type ArrowChirho = Int %MissingMultiplicityChirho -> Int
canaryChirho :: MissingCanaryChirho
canaryChirho = ()
"#;
    assert_missing_name_chirho(source_chirho, "MissingMultiplicityChirho");
    assert_missing_name_chirho(source_chirho, "MissingCanaryChirho");
    assert_missing_name_chirho(
        r#"{-# LANGUAGE LinearTypes, ExplicitForAll #-}
module WrongMultiplicityScopeChirho where
identityChirho :: forall aChirho. aChirho %missingChirho -> aChirho
identityChirho xChirho = xChirho
"#,
        "missingChirho",
    );
}

#[test]
fn local_constructor_or_alias_names_do_not_invent_builtin_multiplicities_chirho() {
    let local_chirho = r#"{-# LANGUAGE DataKinds, LinearTypes #-}
module WrongLocalMultiplicityChirho where
data ColorChirho = One
type WrongChirho = Int %'One -> Int
"#;
    assert_multiplicity_error_chirho(local_chirho);
    assert_multiplicity_error_chirho(&local_chirho.replace("%'One", "%One"));
    assert_compile_success_chirho(
        "QualifiedMultiplicityChirho.hs",
        r#"{-# LANGUAGE DataKinds, LinearTypes #-}
module QualifiedMultiplicityChirho where
import qualified GHC.Types as TypesChirho
data ColorChirho = One
type LinearChirho = Int %'TypesChirho.One -> Int
type UnrestrictedChirho = Int %TypesChirho.Many -> Int
"#,
    );
    // The type alias is unrestricted despite its name. It must not acquire the
    // fixed-One marker from its spelling before name/kind elaboration.
    assert_compile_success_chirho(
        "ShadowedOneAliasChirho.hs",
        r#"{-# LANGUAGE DataKinds, LinearTypes #-}
module ShadowedOneAliasChirho where
import GHC.Types (Multiplicity(Many))
type One = 'Many
duplicateChirho :: Int %One -> (Int, Int)
duplicateChirho xChirho = (xChirho, xChirho)
"#,
    );
}

#[test]
fn fixed_and_quantified_multiplicity_identities_run_on_every_engine_chirho() {
    // Each unchanged source independently runs as 42 under GHC 9.14.1.
    for source_chirho in [
        r#"{-# LANGUAGE NoDataKinds, LinearTypes #-}
module Main where
import GHC.Types (Multiplicity(One))
identityChirho :: Int %('One) -> Int
identityChirho xChirho = xChirho
main :: IO ()
main = print (identityChirho 42)
"#,
        r#"{-# LANGUAGE LinearTypes, DataKinds, TypeApplications, ExplicitForAll #-}
module Main where
import GHC.Types (Multiplicity(Many))
identityChirho :: forall mChirho aChirho. aChirho %mChirho -> aChirho
identityChirho xChirho = xChirho
main :: IO ()
main = print (identityChirho @'Many @Int 42)
"#,
    ] {
        assert_execution_chirho(source_chirho, "42\n");
    }
}

#[test]
fn class_method_implicit_kinds_are_independent_but_class_binders_are_shared_chirho() {
    let first_chirho =
        "  firstChirho :: ProxyChirho argumentChirho -> Int %multiplicityChirho -> Int\n";
    let second_chirho = "  secondChirho :: ProxyChirho argumentChirho -> ProxyChirho (multiplicityChirho 'True) -> ()\n";
    for methods_chirho in [
        format!("{first_chirho}{second_chirho}"),
        format!("{second_chirho}{first_chirho}"),
    ] {
        assert_compile_success_chirho(
            "IndependentMethodBindersChirho.hs",
            &format!(
                "{{-# LANGUAGE DataKinds, KindSignatures, LinearTypes, PolyKinds #-}}\nmodule IndependentMethodBindersChirho where\ndata ProxyChirho (argumentChirho :: kindChirho) = ProxyChirho\nclass ClassChirho argumentChirho where\n{methods_chirho}"
            ),
        );
    }

    // The class parameter remains shared even though method-local names do not.
    let errors_chirho = haskelujah_driver::typecheck_source_chirho(
        "module SharedClassKindChirho where\nclass ClassChirho constructorChirho where\n  firstChirho :: constructorChirho Int -> ()\n  secondChirho :: constructorChirho Maybe -> ()\n",
        &mut SourceMapChirho::new_chirho(),
        "SharedClassKindChirho.hs",
    )
    .err()
    .expect("a class parameter cannot have incompatible kinds in its methods");
    assert!(
        errors_chirho
            .diagnostics_chirho()
            .iter()
            .any(|error_chirho| {
                error_chirho.code_chirho == Some(ErrorCodeChirho::error_chirho(300))
                    && error_chirho.message_chirho.contains("type application")
            }),
        "{errors_chirho}"
    );
}

fn assert_multiplicity_error_chirho(source_chirho: &str) {
    let errors_chirho = haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongMultiplicityChirho.hs",
    )
    .err()
    .expect("a wrong multiplicity kind must reject");
    assert!(
        errors_chirho
            .diagnostics_chirho()
            .iter()
            .any(|error_chirho| {
                error_chirho.code_chirho == Some(ErrorCodeChirho::error_chirho(300))
                    && error_chirho.message_chirho.contains("arrow multiplicity")
                    && error_chirho.message_chirho.contains("Multiplicity")
            }),
        "{errors_chirho}"
    );
}

fn assert_missing_name_chirho(source_chirho: &str, name_chirho: &str) {
    let errors_chirho = haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "MissingMultiplicityNameChirho.hs",
    )
    .err()
    .expect("the missing name must be checked");
    assert!(
        errors_chirho
            .diagnostics_chirho()
            .iter()
            .any(|error_chirho| {
                error_chirho.code_chirho == Some(ErrorCodeChirho::error_chirho(101))
                    && error_chirho.message_chirho.contains(name_chirho)
            }),
        "{errors_chirho}"
    );
}
