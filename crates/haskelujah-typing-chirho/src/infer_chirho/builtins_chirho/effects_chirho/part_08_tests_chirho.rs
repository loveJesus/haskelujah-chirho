// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

#[test]
fn gcd_and_lcm_keep_their_integral_result_type_chirho() {
    let mut env_chirho = TyEnvChirho::new_chirho();
    super::part_08_chirho::seed_part_08_chirho(&mut env_chirho);

    let integral_v_chirho = TyVarChirho(2212);
    let expected_chirho = SchemeChirho {
        vars_chirho: vec![integral_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Integral".to_string(),
            ty_chirho: TyChirho::VarChirho(integral_v_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(integral_v_chirho),
                TyChirho::VarChirho(integral_v_chirho),
            ],
            TyChirho::VarChirho(integral_v_chirho),
        ),
    };

    assert_eq!(env_chirho.lookup_chirho("gcd"), Some(&expected_chirho));
    assert_eq!(env_chirho.lookup_chirho("lcm"), Some(&expected_chirho));
}
