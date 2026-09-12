// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Import contracts must come from their defining module, not a second AST inference.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use haskelujah_driver::{compile_modules_chirho, typecheck_source_chirho};
use haskelujah_span_chirho::SourceMapChirho;

#[path = "import_contracts_chirho/search_path_chirho.rs"]
mod search_path_chirho;

const FAMILY_PROVIDER_CHIRHO: &str = include_str!(
    "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/imported-families-chirho/ProviderChirho.hs"
);
const FAMILY_CONSUMER_CHIRHO: &str = include_str!(
    "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/imported-families-chirho/ConsumerChirho.hs"
);

#[test]
fn imported_family_equations_keep_their_hidden_matching_inputs_chirho() {
    let qualified_chirho = FAMILY_CONSUMER_CHIRHO
        .replace(
            "import ProviderChirho",
            "import qualified ProviderChirho as PChirho",
        )
        .replace(
            "CanDoChirho (StateChirho",
            "PChirho.CanDoChirho (StateChirho",
        );
    let local_chirho = FAMILY_CONSUMER_CHIRHO.replace(
        "import ProviderChirho",
        "type family CanDoChirho (mChirho :: Type -> Type) (effChirho :: kChirho) :: Bool",
    );
    for source_chirho in [FAMILY_CONSUMER_CHIRHO, &qualified_chirho, &local_chirho] {
        compile_modules_chirho(
            &[
                ("ProviderChirho.hs", FAMILY_PROVIDER_CHIRHO),
                ("ConsumerChirho.hs", source_chirho),
            ],
            &mut SourceMapChirho::new_chirho(),
        )
        .unwrap_or_else(|error_chirho| panic!("{source_chirho}\n{error_chirho}"));
    }
}

#[test]
fn reexported_family_equations_use_the_checked_defining_contract_chirho() {
    let source_chirho =
        FAMILY_CONSUMER_CHIRHO.replace("import ProviderChirho", "import ReexportChirho");
    compile_modules_chirho(
        &[
            ("ProviderChirho.hs", FAMILY_PROVIDER_CHIRHO),
            (
                "ReexportChirho.hs",
                "module ReexportChirho (CanDoChirho) where\nimport ProviderChirho\n",
            ),
            ("ConsumerChirho.hs", &source_chirho),
        ],
        &mut SourceMapChirho::new_chirho(),
    )
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn imported_family_equations_reject_wrong_argument_and_result_kinds_chirho() {
    for (source_chirho, context_chirho) in [
        (
            FAMILY_CONSUMER_CHIRHO.replace("= StateCanDoChirho sChirho effChirho", "= Int"),
            "family equation result",
        ),
        (
            FAMILY_CONSUMER_CHIRHO.replace(
                "type instance CanDoChirho (StateChirho sChirho mChirho)",
                "type instance CanDoChirho (StateChirho sChirho mChirho Int)",
            ),
            "family equation argument",
        ),
    ] {
        let error_chirho = compile_modules_chirho(
            &[
                ("ProviderChirho.hs", FAMILY_PROVIDER_CHIRHO),
                ("ConsumerChirho.hs", &source_chirho),
            ],
            &mut SourceMapChirho::new_chirho(),
        )
        .map(|_| ())
        .expect_err("an imported row must satisfy its defining kind contract");
        assert!(
            error_chirho
                .diagnostics_chirho()
                .iter()
                .any(|diagnostic_chirho| {
                    diagnostic_chirho
                        .code_chirho
                        .as_ref()
                        .is_some_and(|code_chirho| code_chirho.to_string() == "E0300")
                        && diagnostic_chirho.message_chirho.contains(context_chirho)
                        && diagnostic_chirho.message_chirho.contains("kind mismatch")
                }),
            "{error_chirho}"
        );
    }
}

#[test]
fn imported_family_equation_result_is_checked_at_its_use_chirho() {
    let source_chirho = FAMILY_CONSUMER_CHIRHO.replace(":~: 'True", ":~: 'False");
    let error_chirho = compile_modules_chirho(
        &[
            ("ProviderChirho.hs", FAMILY_PROVIDER_CHIRHO),
            ("ConsumerChirho.hs", &source_chirho),
        ],
        &mut SourceMapChirho::new_chirho(),
    )
    .map(|_| ())
    .expect_err("a recovered equation cannot prove the contradictory result");
    assert!(
        error_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| {
                diagnostic_chirho
                    .code_chirho
                    .as_ref()
                    .is_some_and(|code_chirho| code_chirho.to_string() == "E0200")
                    && diagnostic_chirho.message_chirho.contains("True")
                    && diagnostic_chirho.message_chirho.contains("False")
            }),
        "{error_chirho}"
    );
}

#[test]
fn local_nominal_head_replaces_the_unqualified_imported_family_classification_chirho() {
    compile_modules_chirho(
        &[
            ("ProviderChirho.hs", FAMILY_PROVIDER_CHIRHO),
            ("LocalShadowChirho.hs", include_str!(
                "../../../../test-data-chirho/kind-oracles-chirho/ascriptions-chirho/imported-families-chirho/LocalShadowChirho.hs"
            )),
        ],
        &mut SourceMapChirho::new_chirho(),
    ).unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn checked_source_identity_takes_priority_over_the_builtin_classifier_chirho() {
    let provider_chirho = r#"{-# LANGUAGE PolyKinds, KindSignatures #-}
module Control.Monad.Identity (Identity) where
data Identity (aChirho :: kChirho) = MkIdentityChirho
"#;
    let consumer_chirho = r#"{-# LANGUAGE DataKinds #-}
module ShadowedIdentityChirho where
import qualified Control.Monad.Identity as IChirho
type HeldChirho = IChirho.Identity 'True
"#;
    compile_modules_chirho(
        &[
            ("IdentitySourceChirho.hs", provider_chirho),
            ("ShadowedIdentityChirho.hs", consumer_chirho),
        ],
        &mut SourceMapChirho::new_chirho(),
    )
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn qualified_import_does_not_expand_a_same_named_local_data_type_chirho() {
    let provider_chirho =
        "module ShadowProviderChirho (ClashChirho) where\ntype ClashChirho = Int\n";
    let consumer_chirho = r#"module ShadowConsumerChirho where
import qualified ShadowProviderChirho as PChirho
data ClashChirho = LocalClashChirho
localChirho :: ClashChirho
localChirho = LocalClashChirho
importedChirho :: PChirho.ClashChirho
importedChirho = 42
"#;
    compile_modules_chirho(
        &[
            ("ShadowProviderChirho.hs", provider_chirho),
            ("ShadowConsumerChirho.hs", consumer_chirho),
        ],
        &mut SourceMapChirho::new_chirho(),
    )
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn imported_identity_constrains_the_same_classifier_as_a_local_definition_chirho() {
    // These imported/local/explicitly annotated forms were checked with GHC9.14.1.
    let source_chirho = r#"{-# LANGUAGE KindSignatures, PolyKinds, RankNTypes #-}
module ImportedIdentityChirho where
import Control.Monad.Identity (Identity)
newtype ParsecTChirho mChirho = ParsecTChirho (forall bChirho. mChirho bChirho -> mChirho bChirho)
type ParsecChirho = ParsecTChirho Identity
"#;
    let local_chirho = source_chirho.replace(
        "import Control.Monad.Identity (Identity)",
        "newtype Identity aChirho = Identity aChirho",
    );
    let qualified_chirho = source_chirho
        .replace(
            "import Control.Monad.Identity (Identity)",
            "import qualified Control.Monad.Identity as IChirho",
        )
        .replace("ParsecTChirho Identity", "ParsecTChirho IChirho.Identity");
    for case_chirho in [&local_chirho, source_chirho, &qualified_chirho] {
        typecheck_source_chirho(
            case_chirho,
            &mut SourceMapChirho::new_chirho(),
            "ImportedIdentityChirho.hs",
        )
        .unwrap_or_else(|error_chirho| panic!("{case_chirho}\n{error_chirho}"));
    }
    let invalid_chirho = source_chirho.replace("ParsecTChirho Identity", "ParsecTChirho Either");
    let error_chirho = typecheck_source_chirho(
        &invalid_chirho,
        &mut SourceMapChirho::new_chirho(),
        "WrongIdentityChirho.hs",
    )
    .err()
    .expect("Either needs two ordinary type arguments, not one");
    assert!(
        error_chirho.to_string().contains("kind mismatch"),
        "{error_chirho}"
    );
}

#[test]
fn reexported_alias_keeps_private_dependencies_and_independent_kind_arguments_chirho() {
    let provider_chirho = r#"{-# LANGUAGE PolyKinds, KindSignatures #-}
module ContractProviderChirho (BoxChirho(..), AliasChirho, unBoxChirho) where
data BoxChirho (tagChirho :: kChirho) aChirho = BoxChirho aChirho
type AliasChirho tagChirho aChirho = BoxChirho tagChirho aChirho
unBoxChirho :: BoxChirho tagChirho aChirho -> aChirho
unBoxChirho (BoxChirho valueChirho) = valueChirho
"#;
    let reexport_chirho = "module ContractReexportChirho (AliasChirho, unBoxChirho) where\nimport ContractProviderChirho\n";
    let consumer_chirho = r#"{-# LANGUAGE DataKinds #-}
module ContractConsumerChirho where
import qualified ContractReexportChirho as CChirho
boolKindChirho :: CChirho.AliasChirho 'True Int -> Int
boolKindChirho = CChirho.unBoxChirho
typeKindChirho :: CChirho.AliasChirho Int Bool -> Bool
typeKindChirho = CChirho.unBoxChirho
"#;
    let check_chirho = |consumer_chirho: &str| {
        compile_modules_chirho(
            &[
                ("ContractProviderChirho.hs", provider_chirho),
                ("ContractReexportChirho.hs", reexport_chirho),
                ("ContractConsumerChirho.hs", consumer_chirho),
            ],
            &mut SourceMapChirho::new_chirho(),
        )
    };
    check_chirho(consumer_chirho).unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
    let invalid_chirho = consumer_chirho.replace("'True Int -> Int", "'True Int -> Bool");
    let error_chirho = check_chirho(&invalid_chirho)
        .err()
        .expect("alias expansion must still constrain the result");
    assert!(
        error_chirho.to_string().contains("type mismatch"),
        "{error_chirho}"
    );
    let private_chirho = consumer_chirho.to_owned()
        + "privateChirho :: CChirho.BoxChirho Int Bool\nprivateChirho = undefined\n";
    let error_chirho = check_chirho(&private_chirho)
        .err()
        .expect("an alias dependency is not a source-visible export");
    assert!(
        error_chirho.to_string().contains("BoxChirho"),
        "{error_chirho}"
    );
}
