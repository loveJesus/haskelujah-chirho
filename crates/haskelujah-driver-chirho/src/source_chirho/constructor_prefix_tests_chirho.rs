// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Fresh GHC-paired H98 constructor scope, predicates and selector controls.

use super::language_options_tests_chirho::check_chirho;
use haskelujah_diagnostics_chirho::ErrorCodeChirho;

const HEADER_CHIRHO: &str = "{-# LANGUAGE ExistentialQuantification, ExplicitForAll, TypeOperators, PolyKinds, KindSignatures #-}\nmodule ConstructorPrefixChirho where\n";

#[test]
fn written_constructor_prefixes_reach_the_real_frontend_chirho() {
    for body_chirho in [
        "data PairChirho aChirho = forall bChirho. aChirho :> bChirho\n",
        "data PairChirho aChirho = forall bChirho. MkFirstChirho aChirho\n",
        "data KProxyChirho (aChirho :: kChirho) = KProxyChirho\ndata PairChirho = forall kChirho (aChirho :: kChirho). KProxyChirho aChirho :> ()\n",
        "data HoldChirho = forall aChirho. MkHoldChirho { recordChirho :: aChirho }\n",
        "data HoldChirho = forall aChirho. Show aChirho => MkHoldChirho aChirho\ngoodChirho = MkHoldChirho True\n",
        "data HoldChirho = forall . MkHoldChirho\n",
    ] {
        let source_chirho = format!("{HEADER_CHIRHO}{body_chirho}");
        let result_chirho = check_chirho(&source_chirho, &[]);
        if let Err(error_chirho) = result_chirho {
            panic!("{body_chirho}\n{error_chirho}");
        }
    }
}

#[test]
fn constructor_binders_do_not_admit_unwritten_or_sibling_variables_chirho() {
    for body_chirho in [
        "data PairChirho aChirho = aChirho :> bChirho\n",
        "data PairChirho aChirho = forall bChirho. MkFirstChirho bChirho | MkSecondChirho bChirho\n",
        "data KProxyChirho (aChirho :: kChirho) = KProxyChirho\ndata PairChirho = forall kChirho (aChirho :: kChirho). KProxyChirho aChirho :> KProxyChirho bChirho\n",
        "data HoldChirho = Show aChirho => MkHoldChirho aChirho\n",
    ] {
        let source_chirho = format!("{HEADER_CHIRHO}{body_chirho}");
        for options_chirho in [
            &[][..],
            &["-fdefer-type-errors", "-fdefer-out-of-scope-variables"][..],
        ] {
            let error_chirho = check_chirho(&source_chirho, options_chirho)
                .err()
                .expect("the constructor-scope control must reject");
            assert!(
                error_chirho
                    .diagnostics_chirho()
                    .iter()
                    .any(|diagnostic_chirho| {
                        diagnostic_chirho.code_chirho == Some(ErrorCodeChirho::error_chirho(101))
                    }),
                "{body_chirho}\n{error_chirho}"
            );
        }
    }
}

#[test]
fn existential_record_selectors_cannot_release_the_hidden_type_chirho() {
    let source_chirho = format!(
        "{HEADER_CHIRHO}data HoldChirho = forall aChirho. MkHoldChirho {{ recordChirho :: aChirho }}\nescapeChirho valueChirho = recordChirho valueChirho\n"
    );
    let error_chirho = check_chirho(&source_chirho, &[])
        .err()
        .expect("the constructor-scope control must reject");
    assert!(
        error_chirho
            .diagnostics_chirho()
            .iter()
            .all(|diagnostic_chirho| {
                diagnostic_chirho.code_chirho != Some(ErrorCodeChirho::error_chirho(300))
            }),
        "{error_chirho}"
    );
}

#[test]
fn constructor_contexts_are_wanted_at_constructor_uses_chirho() {
    let source_chirho = format!(
        "{HEADER_CHIRHO}data HoldChirho = forall aChirho. Show aChirho => MkHoldChirho aChirho\nbadChirho = MkHoldChirho (\\xChirho -> xChirho)\n"
    );
    let error_chirho = match check_chirho(&source_chirho, &[]) {
        Err(error_chirho) => error_chirho,
        Ok(result_chirho) => panic!(
            "the constructor-scope control must reject; constructor scheme {:?}; binding scheme {:?}",
            result_chirho
                .infer_result_chirho
                .env_chirho
                .lookup_chirho("MkHoldChirho"),
            result_chirho
                .infer_result_chirho
                .env_chirho
                .lookup_chirho("badChirho"),
        ),
    };
    assert!(
        error_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| {
                diagnostic_chirho.code_chirho == Some(ErrorCodeChirho::error_chirho(204))
            }),
        "{error_chirho}"
    );
}

#[test]
fn constructor_universals_precede_existentials_and_retain_hidden_kinds_chirho() {
    for body_chirho in [
        "data HoldChirho aChirho = forall bChirho. MkHoldChirho bChirho\nxChirho :: HoldChirho Bool\nxChirho = MkHoldChirho @Bool @Int (7 :: Int)\n",
        "import Data.Kind (Type)\ndata HoldChirho (aChirho :: kChirho) = forall bChirho. MkHoldChirho bChirho\nxChirho :: HoldChirho Maybe\nxChirho = MkHoldChirho @(Type -> Type) @Maybe @Int (7 :: Int)\n",
        "data HoldChirho aChirho = forall aChirho. MkHoldChirho aChirho\nxChirho :: HoldChirho Bool\nxChirho = MkHoldChirho @Bool @Int (7 :: Int)\n",
        "data HoldChirho aChirho = Show aChirho => MkHoldChirho aChirho\n",
    ] {
        if let Err(error_chirho) = check_chirho(&format!("{HEADER_CHIRHO}{body_chirho}"), &[]) {
            panic!("{body_chirho}\n{error_chirho}");
        }
    }
    let reversed_chirho = format!(
        "{HEADER_CHIRHO}data HoldChirho aChirho = forall bChirho. MkHoldChirho bChirho\nxChirho :: HoldChirho Bool\nxChirho = MkHoldChirho @Int @Bool True\n"
    );
    assert!(check_chirho(&reversed_chirho, &[]).is_err());
    let shadow_escape_chirho = format!(
        "{HEADER_CHIRHO}data HoldChirho aChirho = forall aChirho. MkHoldChirho {{ recordChirho :: aChirho }}\nescapeChirho valueChirho = recordChirho valueChirho\n"
    );
    assert!(check_chirho(&shadow_escape_chirho, &[]).is_err());
}

#[test]
fn source_constructor_contexts_use_actual_instances_and_scoped_givens_chirho() {
    let class_chirho = "class CChirho aChirho where\ndata HoldChirho = forall aChirho. CChirho aChirho => MkHoldChirho aChirho\n";
    for body_chirho in [
        "packChirho :: CChirho aChirho => aChirho -> HoldChirho\npackChirho valueChirho = MkHoldChirho valueChirho\n",
        "instance CChirho (aChirho -> aChirho) where\ngoodChirho = MkHoldChirho (\\valueChirho -> valueChirho)\n",
        "packChirho :: CChirho (aChirho -> aChirho) => (aChirho -> aChirho) -> HoldChirho\npackChirho valueChirho = MkHoldChirho valueChirho\n",
    ] {
        if let Err(error_chirho) =
            check_chirho(&format!("{HEADER_CHIRHO}{class_chirho}{body_chirho}"), &[])
        {
            panic!("{body_chirho}\n{error_chirho}");
        }
    }
    for body_chirho in [
        "badChirho = MkHoldChirho (\\valueChirho -> valueChirho)\n",
        "badChirho = MkHoldChirho (not :: Bool -> Bool)\n",
    ] {
        let error_chirho =
            check_chirho(&format!("{HEADER_CHIRHO}{class_chirho}{body_chirho}"), &[])
                .err()
                .expect("a complete source class has no matching function instance");
        assert!(
            error_chirho
                .diagnostics_chirho()
                .iter()
                .any(|diagnostic_chirho| {
                    diagnostic_chirho.code_chirho == Some(ErrorCodeChirho::error_chirho(204))
                }),
            "{error_chirho}"
        );
    }
}

#[test]
fn newtype_constructor_prefixes_obey_written_shape_rules_chirho() {
    let accepted_chirho =
        format!("{HEADER_CHIRHO}newtype HoldChirho = forall . MkHoldChirho Int\n");
    if let Err(error_chirho) = check_chirho(&accepted_chirho, &[]) {
        panic!("{error_chirho}");
    }
    for body_chirho in [
        "newtype HoldChirho = forall aChirho. MkHoldChirho Int\n",
        "newtype HoldChirho = forall aChirho. MkHoldChirho aChirho\n",
        "newtype HoldChirho aChirho = Show aChirho => MkHoldChirho aChirho\n",
    ] {
        let error_chirho = check_chirho(&format!("{HEADER_CHIRHO}{body_chirho}"), &[])
            .err()
            .expect("a newtype cannot carry existential or context evidence");
        assert!(
            error_chirho
                .diagnostics_chirho()
                .iter()
                .any(|diagnostic_chirho| {
                    diagnostic_chirho.code_chirho == Some(ErrorCodeChirho::error_chirho(209))
                }),
            "{body_chirho}\n{error_chirho}"
        );
    }
}
