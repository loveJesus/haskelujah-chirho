// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded superclass entailment (row 694, #31972): a chain proves and a class it cannot
//! reach answers "not entailed"; a changing cycle proves its first edges and ends unknown,
//! never "not entailed"; a doubling cycle stops at the size bound.

use std::collections::HashMap;

use super::SuperclassSearchExhaustedChirho;
use crate::class_chirho::{ClassDeclChirho, ClassEnvChirho, PredChirho};
use crate::ty_chirho::{SchemePredChirho, TyChirho, TyVarChirho};

fn var_chirho(id_chirho: u32) -> TyChirho {
    TyChirho::VarChirho(TyVarChirho(id_chirho))
}

fn con_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ConChirho(name_chirho.to_string())
}

fn list_chirho(element_chirho: TyChirho) -> TyChirho {
    TyChirho::ListChirho(Box::new(element_chirho))
}

/// `class <templates> => <name> v0 .. v(arity-1)`, without methods.
fn class_chirho(
    name_chirho: &str,
    arity_chirho: u32,
    templates_chirho: Vec<SchemePredChirho>,
) -> ClassDeclChirho {
    ClassDeclChirho {
        name_chirho: name_chirho.to_string(),
        supers_chirho: templates_chirho
            .iter()
            .map(|template_chirho| template_chirho.class_name_chirho.clone())
            .collect(),
        super_templates_chirho: templates_chirho,
        var_chirho: TyVarChirho(0),
        methods_chirho: HashMap::new(),
        extra_vars_chirho: (1..arity_chirho).map(TyVarChirho).collect(),
        fundeps_chirho: vec![],
        defaults_chirho: HashMap::new(),
    }
}

fn template_chirho(class_name_chirho: &str, ty_chirho: TyChirho) -> SchemePredChirho {
    SchemePredChirho {
        class_name_chirho: class_name_chirho.to_string(),
        ty_chirho,
        extra_tys_chirho: vec![],
    }
}

fn pred_chirho(class_name_chirho: &str, tys_chirho: Vec<TyChirho>) -> PredChirho {
    let mut tys_chirho = tys_chirho.into_iter();
    PredChirho {
        class_name_chirho: class_name_chirho.to_string(),
        ty_chirho: tys_chirho.next().expect("a predicate has an argument"),
        extra_tys_chirho: tys_chirho.collect(),
    }
}

fn env_chirho(classes_chirho: Vec<ClassDeclChirho>) -> ClassEnvChirho {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    for class_chirho in classes_chirho {
        env_chirho.add_class_chirho(class_chirho);
    }
    env_chirho
}

#[test]
fn a_chain_proves_and_an_unreachable_class_is_not_entailed_chirho() {
    let env_chirho = env_chirho(vec![
        class_chirho("EqChirho", 1, vec![]),
        class_chirho(
            "OrdChirho",
            1,
            vec![template_chirho("EqChirho", var_chirho(0))],
        ),
        class_chirho("ShowChirho", 1, vec![]),
    ]);
    let given_chirho = pred_chirho("OrdChirho", vec![var_chirho(7)]);
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &given_chirho,
            &pred_chirho("EqChirho", vec![var_chirho(7)])
        ),
        Ok(true)
    );
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &given_chirho,
            &pred_chirho("ShowChirho", vec![var_chirho(7)])
        ),
        Ok(false)
    );
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &given_chirho,
            &pred_chirho("EqChirho", vec![con_chirho("Int")])
        ),
        Ok(false)
    );
}

#[test]
fn a_multi_parameter_given_entails_its_superclass_at_its_own_argument_chirho() {
    // `class Monad m => AskChirho r m`: `AskChirho Int m` gives `Monad m`, never `Monad Int`.
    let env_chirho = env_chirho(vec![
        class_chirho("MonadChirho", 1, vec![]),
        class_chirho(
            "AskChirho",
            2,
            vec![template_chirho("MonadChirho", var_chirho(1))],
        ),
    ]);
    let given_chirho = pred_chirho("AskChirho", vec![con_chirho("Int"), var_chirho(7)]);
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &given_chirho,
            &pred_chirho("MonadChirho", vec![var_chirho(7)])
        ),
        Ok(true)
    );
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &given_chirho,
            &pred_chirho("MonadChirho", vec![con_chirho("Int")])
        ),
        Ok(false)
    );
}

#[test]
fn a_changing_cycle_proves_its_edges_and_ends_unknown_chirho() {
    // `class C [a] => C a` under UndecidableSuperClasses: no derived predicate repeats.
    let env_chirho = env_chirho(vec![
        class_chirho(
            "CycleChirho",
            1,
            vec![template_chirho("CycleChirho", list_chirho(var_chirho(0)))],
        ),
        class_chirho("OtherChirho", 1, vec![]),
    ]);
    let given_chirho = pred_chirho("CycleChirho", vec![var_chirho(7)]);
    // The first edge, then the second.
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &given_chirho,
            &pred_chirho("CycleChirho", vec![list_chirho(var_chirho(7))])
        ),
        Ok(true)
    );
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &given_chirho,
            &pred_chirho("CycleChirho", vec![list_chirho(list_chirho(var_chirho(7)))])
        ),
        Ok(true)
    );
    // Never reached: the bound stops the search, and the answer is unknown, not "no".
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &given_chirho,
            &pred_chirho("CycleChirho", vec![con_chirho("Int")])
        ),
        Err(SuperclassSearchExhaustedChirho)
    );
    // A class the cycle cannot reach by name is a complete negative.
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &given_chirho,
            &pred_chirho("OtherChirho", vec![var_chirho(7)])
        ),
        Ok(false)
    );
}

#[test]
fn a_doubling_cycle_stops_at_the_size_bound_chirho() {
    // `class D (a, a) => D a`: each step doubles the argument.
    let pair_chirho = TyChirho::TupleChirho(vec![var_chirho(0), var_chirho(0)]);
    let env_chirho = env_chirho(vec![class_chirho(
        "PairCycleChirho",
        1,
        vec![template_chirho("PairCycleChirho", pair_chirho)],
    )]);
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &pred_chirho("PairCycleChirho", vec![var_chirho(7)]),
            &pred_chirho("PairCycleChirho", vec![con_chirho("Int")])
        ),
        Err(SuperclassSearchExhaustedChirho)
    );
}
