// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Structural selection of a ground goal's instance: the unique head match with its substituted
//! context, a refusal by name when none or several match, and the ground test.

use super::ground_selection_chirho::{
    GroundRefusalChirho, pred_is_ground_chirho, select_ground_instance_chirho,
};
use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, InstDeclChirho, PredChirho};
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

fn con_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ConChirho(name_chirho.to_string())
}

fn succ_chirho(inner_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(Box::new(con_chirho("SuccChirho")), Box::new(inner_chirho))
}

fn known_nat_env_chirho() -> ClassEnvChirho {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.add_instance_chirho(InstDeclChirho {
        class_name_chirho: "KnownNatChirho".to_string(),
        head_ty_chirho: con_chirho("ZeroChirho"),
        extra_head_tys_chirho: Vec::new(),
        context_chirho: Vec::new(),
    });
    let n_chirho = TyChirho::VarChirho(TyVarChirho(9001));
    env_chirho.add_instance_chirho(InstDeclChirho {
        class_name_chirho: "KnownNatChirho".to_string(),
        head_ty_chirho: succ_chirho(n_chirho.clone()),
        extra_head_tys_chirho: Vec::new(),
        context_chirho: vec![PredChirho::new_chirho("KnownNatChirho", n_chirho)],
    });
    env_chirho
}

#[test]
fn a_ground_goal_selects_its_unique_instance_with_the_substituted_context_chirho() {
    let env_chirho = known_nat_env_chirho();
    let goal_chirho = PredChirho::new_chirho(
        "KnownNatChirho",
        succ_chirho(succ_chirho(con_chirho("ZeroChirho"))),
    );
    let (instance_chirho, obligations_chirho) =
        select_ground_instance_chirho(&env_chirho, &goal_chirho).expect("one instance matches");
    assert!(!instance_chirho.context_chirho.is_empty());
    assert_eq!(
        obligations_chirho,
        vec![PredChirho::new_chirho(
            "KnownNatChirho",
            succ_chirho(con_chirho("ZeroChirho"))
        )]
    );
}

#[test]
fn a_goal_no_head_matches_is_refused_by_name_chirho() {
    let env_chirho = known_nat_env_chirho();
    let goal_chirho = PredChirho::new_chirho("KnownNatChirho", con_chirho("BoolChirho"));
    assert_eq!(
        select_ground_instance_chirho(&env_chirho, &goal_chirho).map(|_| ()),
        Err(GroundRefusalChirho::UnresolvedChirho(goal_chirho))
    );
}

#[test]
fn several_matching_heads_are_refused_never_chosen_chirho() {
    let mut env_chirho = known_nat_env_chirho();
    env_chirho.add_instance_chirho(InstDeclChirho {
        class_name_chirho: "KnownNatChirho".to_string(),
        head_ty_chirho: TyChirho::VarChirho(TyVarChirho(9002)),
        extra_head_tys_chirho: Vec::new(),
        context_chirho: Vec::new(),
    });
    let goal_chirho = PredChirho::new_chirho("KnownNatChirho", con_chirho("ZeroChirho"));
    assert!(matches!(
        select_ground_instance_chirho(&env_chirho, &goal_chirho),
        Err(GroundRefusalChirho::OverlapChirho(_, heads_chirho)) if heads_chirho.len() == 2
    ));
}

#[test]
fn a_goal_with_a_meta_or_a_rigid_variable_is_not_ground_chirho() {
    let meta_chirho = PredChirho::new_chirho(
        "KnownNatChirho",
        succ_chirho(TyChirho::VarChirho(TyVarChirho(7))),
    );
    let rigid_chirho = PredChirho::new_chirho(
        "KnownNatChirho",
        succ_chirho(TyChirho::ForallVarChirho("n%1".to_string())),
    );
    let ground_chirho =
        PredChirho::new_chirho("KnownNatChirho", succ_chirho(con_chirho("ZeroChirho")));
    assert!(!pred_is_ground_chirho(&meta_chirho));
    assert!(!pred_is_ground_chirho(&rigid_chirho));
    assert!(pred_is_ground_chirho(&ground_chirho));
}
