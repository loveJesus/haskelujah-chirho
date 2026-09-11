// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Family equations must survive into kinds, not merely erase a kind error.
//! The positive and negative sources were checked independently by GHC 9.14.1.

use super::assert_compile_success_chirho;
use haskelujah_driver::typecheck_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn invisible_family_head_binder_is_not_a_visible_equation_argument_chirho() {
    assert_compile_success_chirho(
        "FamilyInvisibleBinderChirho.hs",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../test-data-chirho/kind-oracles-chirho/FamilyInvisibleBinderChirho.hs"
        )),
    );
}

#[test]
fn family_reduction_can_remove_a_dependent_kind_occurrence_chirho() {
    assert_compile_success_chirho(
        "FamilyDependentConstChirho.hs",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../test-data-chirho/kind-oracles-chirho/FamilyDependentConstChirho.hs"
        )),
    );
}

#[test]
fn family_equation_variables_do_not_inherit_header_scope_chirho() {
    let source_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-data-chirho/kind-oracles-chirho/FamilyEquationScopeChirho.hs"
    ));
    assert_compile_success_chirho("FamilyEquationScopeChirho.hs", source_chirho);
    let invalid_chirho = source_chirho.replace(
        "SecondChirho ('PairChirho _ aChirho) = aChirho",
        "SecondChirho ('PairChirho _ aChirho) = Int",
    );
    let error_chirho = typecheck_source_chirho(
        &invalid_chirho,
        &mut SourceMapChirho::new_chirho(),
        "FamilyEquationScopeChirho.hs",
    )
    .err()
    .expect("the equation result must still have the declared Bool kind");
    assert!(
        error_chirho.to_string().contains("family equation result"),
        "{error_chirho}"
    );
}

#[test]
fn family_wildcards_infer_their_classifiers_independently_chirho() {
    assert_compile_success_chirho(
        "FamilyWildcardKindsChirho.hs",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../test-data-chirho/kind-oracles-chirho/FamilyWildcardKindsChirho.hs"
        )),
    );
}

#[test]
fn polymorphic_family_equations_are_not_silently_opaque_chirho() {
    for equation_chirho in [
        "BadChirho (forall bChirho. bChirho -> bChirho) = Int",
        "BadChirho aChirho = forall bChirho. bChirho -> aChirho",
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE TypeFamilies, RankNTypes #-}}\nmodule BadFamilyChirho where\ntype family BadChirho aChirho where\n  {equation_chirho}\n"
        );
        let error_chirho = typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "BadFamilyChirho.hs",
        )
        .err()
        .expect("GHC-91510 must reject a polymorphic family equation");
        assert!(
            error_chirho
                .to_string()
                .contains("polymorphic type in family equation"),
            "{error_chirho}"
        );
    }
}

const INJECTIVE_CHIRHO: &str = r#"{-# LANGUAGE DataKinds, GADTs, KindSignatures, PolyKinds, RankNTypes, TypeFamilyDependencies #-}
module InjectiveKindFamilyChirho where
import Data.Kind (Type)
data CodeChirho = CodeIChirho
type family InterpChirho (codeChirho :: CodeChirho) = (resultChirho :: Type) | resultChirho -> codeChirho where
  InterpChirho 'CodeIChirho = Bool
data TermChirho :: forall codeChirho. InterpChirho codeChirho -> Type where
  MkTermChirho :: TermChirho 'False
"#;

#[test]
fn synonym_expansion_precedes_injectivity_validation_chirho() {
    let source_chirho = r#"{-# LANGUAGE TypeFamilyDependencies #-}
module SynonymInjectivityChirho where
type AliasChirho aChirho = Bool
type family FamilyChirho aChirho = resultChirho | resultChirho -> aChirho where
  FamilyChirho aChirho = AliasChirho aChirho
"#;
    let error_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "SynonymInjectivityChirho.hs",
    )
    .err()
    .expect("an alias that erases its argument cannot prove injectivity");
    assert!(
        error_chirho.to_string().contains("injectivity loses"),
        "{error_chirho}"
    );
    assert_compile_success_chirho(
        "SynonymInjectivityChirho.hs",
        &source_chirho.replace(
            "type AliasChirho aChirho = Bool",
            "type AliasChirho aChirho = Maybe aChirho",
        ),
    );
}

#[test]
fn closed_family_reduces_inside_a_kind_chirho() {
    let source_chirho = INJECTIVE_CHIRHO
        .replace(
            "= (resultChirho :: Type) | resultChirho -> codeChirho",
            ":: Type",
        )
        .replace(
            "forall codeChirho. InterpChirho codeChirho",
            "InterpChirho 'CodeIChirho",
        );
    assert_compile_success_chirho("ForwardKindFamilyChirho.hs", &source_chirho);
}

#[test]
fn validated_injective_family_improves_its_kind_argument_chirho() {
    assert_compile_success_chirho("InjectiveKindFamilyChirho.hs", INJECTIVE_CHIRHO);
    let source_chirho = INJECTIVE_CHIRHO.replace(
        "= (resultChirho :: Type) | resultChirho -> codeChirho",
        ":: Type",
    );
    assert!(
        typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "NonInjectiveChirho.hs"
        )
        .is_err(),
        "a family without injectivity must not determine an ambiguous argument"
    );
}

#[test]
fn contradictory_injectivity_is_rejected_before_any_use_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, KindSignatures, TypeFamilyDependencies #-}
module InvalidInjectivityChirho where
import Data.Kind (Type)
data CodeChirho = CodeIChirho | CodeJChirho
type family InterpChirho (codeChirho :: CodeChirho) = (resultChirho :: Type) | resultChirho -> codeChirho where
  InterpChirho 'CodeIChirho = Bool
  InterpChirho 'CodeJChirho = Bool
"#;
    let error_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "InvalidInjectivityChirho.hs",
    )
    .err()
    .expect("two distinct arguments cannot be recovered from the same result");
    assert!(
        error_chirho.to_string().contains("injectiv"),
        "{error_chirho}"
    );
}
