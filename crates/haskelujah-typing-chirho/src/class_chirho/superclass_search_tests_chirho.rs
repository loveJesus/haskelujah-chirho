// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded superclass entailment (row 694, #31972): a chain proves and a class it cannot
//! reach answers "not entailed"; a changing cycle proves its first edges and ends unknown,
//! never "not entailed"; a doubling cycle stops at the size bound. Row 692, C10 Q (gpt #33099):
//! a compiler class and a program class with equal text never discharge each other's
//! constraints, directly or through a superclass, while an undisplaced spelling still meets by
//! spelling.

use std::collections::HashMap;

use haskelujah_ast_chirho::name_chirho::CompilerClassChirho;

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
    ClassDeclChirho::legacy_chirho(crate::class_chirho::ClassDeclPartsChirho {
        kind_vars_chirho: Vec::new(),
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
    })
}

fn template_chirho(class_name_chirho: &str, ty_chirho: TyChirho) -> SchemePredChirho {
    SchemePredChirho::legacy_chirho(crate::ty_chirho::SchemePredPartsChirho {
        kind_tys_chirho: Vec::new(),
        class_name_chirho: class_name_chirho.to_string(),
        ty_chirho,
        extra_tys_chirho: vec![],
    })
}

fn pred_chirho(class_name_chirho: &str, tys_chirho: Vec<TyChirho>) -> PredChirho {
    let mut tys_chirho = tys_chirho.into_iter();
    PredChirho::legacy_chirho(crate::class_chirho::PredPartsChirho {
        kind_tys_chirho: Vec::new(),
        class_name_chirho: class_name_chirho.to_string(),
        ty_chirho: tys_chirho.next().expect("a predicate has an argument"),
        extra_tys_chirho: tys_chirho.collect(),
    })
}

fn env_chirho(classes_chirho: Vec<ClassDeclChirho>) -> ClassEnvChirho {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    for class_chirho in classes_chirho {
        env_chirho.add_class_chirho(class_chirho);
    }
    env_chirho
}

/// A compiler class seeded as the standard seed writes it: proven, with proven superclasses.
fn seed_compiler_class_chirho(
    env_chirho: &mut ClassEnvChirho,
    identity_chirho: CompilerClassChirho,
    superclasses_chirho: &[CompilerClassChirho],
) {
    let parts_chirho = crate::class_chirho::ClassDeclPartsChirho {
        kind_vars_chirho: Vec::new(),
        name_chirho: identity_chirho.legacy_name_chirho().to_string(),
        supers_chirho: Vec::new(),
        super_templates_chirho: Vec::new(),
        var_chirho: TyVarChirho(0),
        methods_chirho: HashMap::new(),
        extra_vars_chirho: Vec::new(),
        fundeps_chirho: vec![],
        defaults_chirho: HashMap::new(),
    };
    if superclasses_chirho.is_empty() {
        env_chirho.add_seed_class_chirho(identity_chirho, parts_chirho);
    } else {
        env_chirho.add_seed_class_with_superclasses_chirho(
            identity_chirho,
            superclasses_chirho,
            parts_chirho,
        );
    }
}

/// A predicate whose class the compiler's own producer proved.
fn compiler_pred_chirho(identity_chirho: CompilerClassChirho, ty_chirho: TyChirho) -> PredChirho {
    PredChirho::from_compiler_chirho(
        identity_chirho,
        crate::class_chirho::PredPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: identity_chirho.legacy_name_chirho().to_string(),
            ty_chirho,
            extra_tys_chirho: vec![],
        },
    )
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

#[test]
fn a_compiler_given_never_expands_the_namesakes_superclass_chirho() {
    // gpt #33099: the program's own `class Show a => Num a` takes the spelling `Num`. The
    // compiler's Num has no superclass, so its given proves no `Show`, whichever proof it has.
    let mut env_chirho = ClassEnvChirho::new_chirho();
    seed_compiler_class_chirho(&mut env_chirho, CompilerClassChirho::ShowChirho, &[]);
    seed_compiler_class_chirho(&mut env_chirho, CompilerClassChirho::NumChirho, &[]);
    env_chirho.add_class_chirho(class_chirho(
        "Num",
        1,
        vec![template_chirho("Show", var_chirho(0))],
    ));
    let given_chirho = compiler_pred_chirho(CompilerClassChirho::NumChirho, var_chirho(7));
    for wanted_chirho in [
        compiler_pred_chirho(CompilerClassChirho::ShowChirho, var_chirho(7)),
        pred_chirho("Show", vec![var_chirho(7)]),
    ] {
        assert_eq!(
            env_chirho.superclass_entails_chirho(&given_chirho, &wanted_chirho),
            Ok(false)
        );
    }
}

#[test]
fn a_displaced_compiler_given_keeps_its_own_compiler_superclass_chirho() {
    // The program's own `class Ord a`, with no superclass, takes the spelling `Ord`. The
    // compiler's Ord keeps its proven Eq superclass, read from its own record by identity.
    let mut env_chirho = ClassEnvChirho::new_chirho();
    seed_compiler_class_chirho(&mut env_chirho, CompilerClassChirho::EqChirho, &[]);
    seed_compiler_class_chirho(
        &mut env_chirho,
        CompilerClassChirho::OrdChirho,
        &[CompilerClassChirho::EqChirho],
    );
    env_chirho.add_class_chirho(class_chirho("Ord", 1, vec![]));
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &compiler_pred_chirho(CompilerClassChirho::OrdChirho, var_chirho(7)),
            &compiler_pred_chirho(CompilerClassChirho::EqChirho, var_chirho(7))
        ),
        Ok(true)
    );
}

#[test]
fn a_compiler_given_never_serves_the_namesakes_constraint_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    seed_compiler_class_chirho(&mut env_chirho, CompilerClassChirho::NumChirho, &[]);
    env_chirho.add_class_chirho(class_chirho("Num", 1, vec![]));
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &compiler_pred_chirho(CompilerClassChirho::NumChirho, var_chirho(7)),
            &pred_chirho("Num", vec![var_chirho(7)])
        ),
        Ok(false)
    );
}

#[test]
fn the_namesakes_given_never_serves_a_compiler_constraint_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    seed_compiler_class_chirho(&mut env_chirho, CompilerClassChirho::NumChirho, &[]);
    env_chirho.add_class_chirho(class_chirho("Num", 1, vec![]));
    assert_eq!(
        env_chirho.superclass_entails_chirho(
            &pred_chirho("Num", vec![var_chirho(7)]),
            &compiler_pred_chirho(CompilerClassChirho::NumChirho, var_chirho(7))
        ),
        Ok(false)
    );
}

#[test]
fn an_undisplaced_compiler_class_keeps_its_spelling_entailment_chirho() {
    // No program class takes a spelling, so each spelling's active record is the compiler's:
    // proven and unproven predicates still meet by spelling, directly and through a superclass.
    let mut env_chirho = ClassEnvChirho::new_chirho();
    seed_compiler_class_chirho(&mut env_chirho, CompilerClassChirho::EqChirho, &[]);
    seed_compiler_class_chirho(
        &mut env_chirho,
        CompilerClassChirho::OrdChirho,
        &[CompilerClassChirho::EqChirho],
    );
    seed_compiler_class_chirho(&mut env_chirho, CompilerClassChirho::NumChirho, &[]);
    let proven_num_chirho = compiler_pred_chirho(CompilerClassChirho::NumChirho, var_chirho(7));
    let spelled_num_chirho = pred_chirho("Num", vec![var_chirho(7)]);
    let proven_eq_chirho = compiler_pred_chirho(CompilerClassChirho::EqChirho, var_chirho(7));
    for (given_chirho, wanted_chirho) in [
        (&proven_num_chirho, &spelled_num_chirho),
        (&spelled_num_chirho, &proven_num_chirho),
        (
            &compiler_pred_chirho(CompilerClassChirho::OrdChirho, var_chirho(7)),
            &proven_eq_chirho,
        ),
        (&pred_chirho("Ord", vec![var_chirho(7)]), &proven_eq_chirho),
    ] {
        assert_eq!(
            env_chirho.superclass_entails_chirho(given_chirho, wanted_chirho),
            Ok(true)
        );
    }
}
