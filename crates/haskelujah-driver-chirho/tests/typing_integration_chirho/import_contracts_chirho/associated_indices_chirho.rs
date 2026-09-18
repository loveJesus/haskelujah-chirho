// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Associated rows and consumers must agree on nominal invisible arguments.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use haskelujah_driver::{compile_modules_chirho, typecheck_source_chirho};
use haskelujah_span_chirho::SourceMapChirho;

const PROVIDER_CHIRHO: &str = r####"{-# LANGUAGE TypeFamilies, FlexibleInstances #-}
module IndexedProviderChirho where
class SelectChirho mChirho where
  type StateChirho mChirho
data TaggedChirho sChirho aChirho = TaggedChirho aChirho
data BoxChirho sChirho aChirho = BoxChirho
makeChirho :: SelectChirho mChirho => aChirho -> mChirho (BoxChirho (StateChirho mChirho) aChirho)
makeChirho = undefined
instance SelectChirho (TaggedChirho sChirho) where
  type StateChirho (TaggedChirho sChirho) = sChirho
"####;

const CONSUMER_CHIRHO: &str = r####"module IndexedConsumerChirho where
import IndexedProviderChirho
useChirho :: aChirho -> TaggedChirho sChirho (BoxChirho sChirho aChirho)
useChirho = makeChirho
"####;

fn provider_chirho() -> String {
    PROVIDER_CHIRHO
        .replace(
            "TypeFamilies, FlexibleInstances",
            "TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds",
        )
        .replace(
            "module IndexedProviderChirho where",
            "module IndexedProviderChirho where\nimport Data.Kind (Type)",
        )
        .replace(
            "class SelectChirho mChirho",
            "class SelectChirho (mChirho :: Type -> Type)",
        )
        .replace(
            "instance SelectChirho (TaggedChirho sChirho)",
            "instance SelectChirho (TaggedChirho (sChirho :: Type))",
        )
}

fn consumer_chirho() -> String {
    "{-# LANGUAGE KindSignatures #-}\n".to_owned()
        + &CONSUMER_CHIRHO
            .replace(
                "import IndexedProviderChirho",
                "import IndexedProviderChirho\nimport Data.Kind (Type)",
            )
            .replace(
                "TaggedChirho sChirho (BoxChirho",
                "TaggedChirho (sChirho :: Type) (BoxChirho",
            )
}

#[test]
fn associated_indices_cross_a_module_boundary_chirho() {
    let source_chirho = provider_chirho();
    let use_chirho = consumer_chirho();
    let result_chirho = compile_modules_chirho(
        &[
            ("IndexedProviderChirho.hs", &source_chirho),
            ("IndexedConsumerChirho.hs", &use_chirho),
        ],
        &mut SourceMapChirho::new_chirho(),
    );
    if let Err(error_chirho) = result_chirho {
        let provider_chirho = typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "IndexedProviderChirho.hs",
        )
        .unwrap();
        panic!(
            "{error_chirho}\nprovider rows: {:?}",
            provider_chirho
                .infer_result_chirho
                .type_families_chirho
                .get("StateChirho")
        );
    }
}

#[test]
fn associated_indices_do_not_prove_a_wrong_result_chirho() {
    let provider_chirho = provider_chirho();
    let consumer_chirho = consumer_chirho().replace(
        "(sChirho :: Type) (BoxChirho sChirho",
        "Int (BoxChirho Bool",
    );
    let error_chirho = compile_modules_chirho(
        &[
            ("IndexedProviderChirho.hs", &provider_chirho),
            ("IndexedConsumerChirho.hs", &consumer_chirho),
        ],
        &mut SourceMapChirho::new_chirho(),
    )
    .expect_err("State (Tagged Int) cannot reduce to Bool");
    assert!(
        error_chirho.to_string().contains("type mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn associated_indices_check_the_equation_result_kind_chirho() {
    let source_chirho = provider_chirho().replace("= sChirho\n", "= Maybe\n");
    let error_chirho = typecheck_source_chirho(
        &source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "IndexedProviderChirho.hs",
    )
    .err()
    .expect("an associated equation cannot supply Maybe at kind Type");
    assert!(
        error_chirho.to_string().contains("kind mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn associated_indices_cannot_specialize_the_enclosing_instance_kind_chirho() {
    let error_chirho = typecheck_source_chirho(
        PROVIDER_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "IndexedProviderChirho.hs",
    )
    .err()
    .expect("the family RHS cannot specialize the instance's quantified phantom kind");
    assert!(
        error_chirho.to_string().contains("kind mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn dependent_associated_defaults_follow_the_reference_contracts_chirho() {
    // Independent GHC observations include valid defaults at two kinds, explicit
    // overrides, cross-module use, and contradictory argument/result classifiers.
    type ReferenceCaseChirho = (bool, &'static str, &'static [(&'static str, &'static str)]);
    let cases_chirho: &[ReferenceCaseChirho] = include!(
        "../../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/imported-families-chirho/source-boot-chirho/declarations-chirho/classes-chirho/default-annotations-chirho/scope-chirho/corpus-chirho/associated-instances-chirho/dependent-chirho/fixtures_chirho.rs"
    );
    let mut failures_chirho = Vec::new();
    for (expected_chirho, name_chirho, sources_chirho) in cases_chirho {
        let result_chirho =
            compile_modules_chirho(sources_chirho, &mut SourceMapChirho::new_chirho());
        if result_chirho.is_ok() != *expected_chirho {
            failures_chirho.push(format!(
                "{}: {}",
                name_chirho,
                result_chirho
                    .err()
                    .map(|error_chirho| error_chirho.to_string())
                    .unwrap_or_else(|| "wrongly accepted".into())
            ));
        }
    }
    assert!(failures_chirho.is_empty(), "{}", failures_chirho.join("\n"));
}
