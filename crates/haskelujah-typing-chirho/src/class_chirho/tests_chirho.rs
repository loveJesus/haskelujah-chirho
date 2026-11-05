// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use crate::ty_chirho::SchemeChirho;

#[test]
fn pred_display_chirho() {
    let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::int_chirho());
    assert_eq!(pred_chirho.to_string(), "Eq Int");
}

#[test]
fn qual_ty_display_chirho() {
    let qt_chirho = QualTyChirho::qualified_chirho(
        vec![
            PredChirho::new_chirho("Eq", TyChirho::VarChirho(TyVarChirho(0))),
            PredChirho::new_chirho("Show", TyChirho::VarChirho(TyVarChirho(0))),
        ],
        TyChirho::fun_chirho(
            TyChirho::VarChirho(TyVarChirho(0)),
            TyChirho::string_chirho(),
        ),
    );
    assert_eq!(qt_chirho.to_string(), "(Eq t0, Show t0) => (t0 -> [Char])");
}

#[test]
fn unqualified_display_chirho() {
    let qt_chirho = QualTyChirho::unqualified_chirho(TyChirho::int_chirho());
    assert_eq!(qt_chirho.to_string(), "Int");
}

#[test]
fn seed_standard_classes_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();

    assert!(env_chirho.has_class_chirho("Eq"));
    assert!(env_chirho.has_class_chirho("Ord"));
    assert!(env_chirho.has_class_chirho("Show"));
    assert!(env_chirho.has_class_chirho("Num"));
    assert!(env_chirho.has_class_chirho("Functor"));
    assert!(env_chirho.has_class_chirho("Applicative"));
    assert!(env_chirho.has_class_chirho("Monad"));
    assert!(env_chirho.has_class_chirho("MonadFail"));
    assert!(env_chirho.has_class_chirho("MonadFix"));
    assert!(env_chirho.has_class_chirho("Semigroup"));
    assert!(env_chirho.has_class_chirho("Monoid"));

    assert_eq!(
        env_chirho.superclasses_chirho("Ord"),
        vec!["Eq".to_string()]
    );
    assert_eq!(
        env_chirho.superclasses_chirho("Num"),
        // GHC's Num has no superclasses; Eq and Show left it in GHC 7.4.
        Vec::<String>::new()
    );
    assert_eq!(
        env_chirho.superclasses_chirho("Monoid"),
        vec!["Semigroup".to_string()]
    );
    assert_eq!(
        env_chirho.superclasses_chirho("MonadFail"),
        vec!["Monad".to_string()]
    );
    assert_eq!(
        env_chirho.superclasses_chirho("MonadFix"),
        vec!["Monad".to_string()]
    );
}

#[test]
fn resolve_monadfail_io_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();

    let pred_chirho = PredChirho::new_chirho("MonadFail", TyChirho::ConChirho("IO".to_string()));
    let result_chirho = env_chirho.resolve_chirho(&pred_chirho);
    assert!(result_chirho.is_some());
    assert!(result_chirho.unwrap().is_empty());
}

#[test]
fn resolve_monadfix_standard_instances_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();

    for type_name_chirho in ["IO", "Maybe", "[]"] {
        let pred_chirho = PredChirho::new_chirho(
            "MonadFix",
            TyChirho::ConChirho(type_name_chirho.to_string()),
        );
        assert!(
            env_chirho
                .resolve_chirho(&pred_chirho)
                .is_some_and(|subgoals_chirho| subgoals_chirho.is_empty()),
            "expected a ground MonadFix instance for {type_name_chirho}"
        );
    }
}

#[test]
fn resolve_eq_int_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();

    let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::int_chirho());
    let result_chirho = env_chirho.resolve_chirho(&pred_chirho);
    assert!(result_chirho.is_some());
    // Eq Int has no context, so sub-goals should be empty
    assert!(result_chirho.unwrap().is_empty());
}

#[test]
fn resolve_enum_ordering_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();

    let pred_chirho = PredChirho::new_chirho("Enum", TyChirho::ConChirho("Ordering".to_string()));
    assert!(
        env_chirho
            .resolve_chirho(&pred_chirho)
            .is_some_and(|subgoals_chirho| subgoals_chirho.is_empty()),
        "Ordering has GHC's ground Enum instance"
    );
}

#[test]
fn resolve_bounded_ordering_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();

    let pred_chirho =
        PredChirho::new_chirho("Bounded", TyChirho::ConChirho("Ordering".to_string()));
    assert!(
        env_chirho
            .resolve_chirho(&pred_chirho)
            .is_some_and(|subgoals_chirho| subgoals_chirho.is_empty()),
        "Ordering has GHC's ground Bounded instance"
    );
}

#[test]
fn resolve_eq_list_int_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();

    // Eq [Int] should resolve via `instance Eq a => Eq [a]` with a=Int
    let pred_chirho =
        PredChirho::new_chirho("Eq", TyChirho::ListChirho(Box::new(TyChirho::int_chirho())));
    let result_chirho = env_chirho.resolve_chirho(&pred_chirho);
    assert!(result_chirho.is_some());
    let sub_goals_chirho = result_chirho.unwrap();
    // Sub-goal: Eq Int
    assert_eq!(sub_goals_chirho.len(), 1);
    assert_eq!(sub_goals_chirho[0].class_name_chirho, "Eq");
    assert_eq!(sub_goals_chirho[0].ty_chirho, TyChirho::int_chirho());
}

#[test]
fn entails_eq_int_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();

    assert!(env_chirho.entails_chirho(&PredChirho::new_chirho("Eq", TyChirho::int_chirho())));
    assert!(env_chirho.entails_chirho(&PredChirho::new_chirho("Eq", TyChirho::bool_chirho())));
    assert!(env_chirho.entails_chirho(&PredChirho::new_chirho("Num", TyChirho::int_chirho())));
}

#[test]
fn entails_eq_list_int_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();

    // Eq [Int] should be entailed: Eq [a] requires Eq a, and Eq Int exists
    assert!(env_chirho.entails_chirho(&PredChirho::new_chirho(
        "Eq",
        TyChirho::ListChirho(Box::new(TyChirho::int_chirho()))
    )));
}

#[test]
fn no_instance_returns_none_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();

    // No instance Num Bool
    assert!(!env_chirho.entails_chirho(&PredChirho::new_chirho("Num", TyChirho::bool_chirho())));
}

#[test]
fn match_ty_variable_chirho() {
    let subst_chirho = match_ty_chirho(
        &TyChirho::VarChirho(TyVarChirho(0)),
        &TyChirho::int_chirho(),
    );
    assert!(subst_chirho.is_some());
    let s_chirho = subst_chirho.unwrap();
    assert_eq!(
        s_chirho.apply_ty_chirho(&TyChirho::VarChirho(TyVarChirho(0))),
        TyChirho::int_chirho()
    );
}

#[test]
fn match_ty_concrete_mismatch_chirho() {
    let result_chirho = match_ty_chirho(&TyChirho::int_chirho(), &TyChirho::bool_chirho());
    assert!(result_chirho.is_none());
}

// ---------------------------------------------------------------
// MPTC tests
// ---------------------------------------------------------------

#[test]
fn mptc_class_decl_chirho() {
    // class Convert a b where convert :: a -> b
    let a_chirho = TyVarChirho(100);
    let b_chirho = TyVarChirho(101);
    let decl_chirho = ClassDeclChirho::legacy_chirho(crate::class_chirho::ClassDeclPartsChirho {
        kind_vars_chirho: Vec::new(),
        name_chirho: "Convert".to_string(),
        supers_chirho: vec![],
        var_chirho: a_chirho,
        methods_chirho: HashMap::from([(
            "convert".to_string(),
            SchemeChirho {
                source_binders_chirho: Vec::new(),
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    TyChirho::VarChirho(b_chirho),
                ),
            },
        )]),
        extra_vars_chirho: vec![b_chirho],
        fundeps_chirho: vec![],
        defaults_chirho: HashMap::new(),
    });

    assert!(decl_chirho.is_mptc_chirho());
    assert_eq!(decl_chirho.all_vars_chirho(), vec![a_chirho, b_chirho]);
}

#[test]
fn mptc_pred_display_chirho() {
    let pred_chirho = PredChirho::new_multi_chirho(
        "Convert",
        vec![
            TyChirho::int_chirho(),
            TyChirho::ConChirho("Double".to_string()),
        ],
    );
    assert_eq!(pred_chirho.to_string(), "Convert Int Double");
    assert_eq!(
        pred_chirho.all_tys_chirho(),
        vec![
            TyChirho::int_chirho(),
            TyChirho::ConChirho("Double".to_string())
        ]
    );
}

#[test]
fn mptc_instance_resolution_chirho() {
    // Set up: class Convert a b, instance Convert Int Double
    let mut env_chirho = ClassEnvChirho::new_chirho();

    let a_chirho = TyVarChirho(100);
    let b_chirho = TyVarChirho(101);
    env_chirho.add_class_chirho(ClassDeclChirho::legacy_chirho(
        crate::class_chirho::ClassDeclPartsChirho {
            kind_vars_chirho: Vec::new(),
            name_chirho: "Convert".to_string(),
            supers_chirho: vec![],
            var_chirho: a_chirho,
            methods_chirho: HashMap::new(),
            extra_vars_chirho: vec![b_chirho],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        },
    ));

    env_chirho.add_instance_chirho(InstDeclChirho::legacy_chirho(
        crate::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "Convert".to_string(),
            head_ty_chirho: TyChirho::int_chirho(),
            extra_head_tys_chirho: vec![TyChirho::ConChirho("Double".to_string())],
            context_chirho: vec![],
        },
    ));

    // Should resolve Convert Int Double
    let pred_chirho = PredChirho::new_multi_chirho(
        "Convert",
        vec![
            TyChirho::int_chirho(),
            TyChirho::ConChirho("Double".to_string()),
        ],
    );
    let result_chirho = env_chirho.resolve_chirho(&pred_chirho);
    assert!(result_chirho.is_some());
    assert!(result_chirho.unwrap().is_empty());

    // Should NOT resolve Convert Int Int (wrong second param)
    let pred2_chirho = PredChirho::new_multi_chirho(
        "Convert",
        vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
    );
    let result2_chirho = env_chirho.resolve_chirho(&pred2_chirho);
    assert!(result2_chirho.is_none());

    // Should NOT resolve Convert Double Double (wrong first param)
    let pred3_chirho = PredChirho::new_multi_chirho(
        "Convert",
        vec![
            TyChirho::ConChirho("Double".to_string()),
            TyChirho::ConChirho("Double".to_string()),
        ],
    );
    let result3_chirho = env_chirho.resolve_chirho(&pred3_chirho);
    assert!(result3_chirho.is_none());
}

#[test]
fn mptc_polymorphic_instance_resolution_chirho() {
    // class Convert a b, instance Convert a [a] (wrap in list)
    let mut env_chirho = ClassEnvChirho::new_chirho();

    let a_chirho = TyVarChirho(100);
    let b_chirho = TyVarChirho(101);
    env_chirho.add_class_chirho(ClassDeclChirho::legacy_chirho(
        crate::class_chirho::ClassDeclPartsChirho {
            kind_vars_chirho: Vec::new(),
            name_chirho: "Convert".to_string(),
            supers_chirho: vec![],
            var_chirho: a_chirho,
            methods_chirho: HashMap::new(),
            extra_vars_chirho: vec![b_chirho],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        },
    ));

    let inst_var_chirho = TyVarChirho(200);
    env_chirho.add_instance_chirho(InstDeclChirho::legacy_chirho(
        crate::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "Convert".to_string(),
            head_ty_chirho: TyChirho::VarChirho(inst_var_chirho),
            extra_head_tys_chirho: vec![TyChirho::ListChirho(Box::new(TyChirho::VarChirho(
                inst_var_chirho,
            )))],
            context_chirho: vec![],
        },
    ));

    // Convert Int [Int] should match with inst_var = Int
    let pred_chirho = PredChirho::new_multi_chirho(
        "Convert",
        vec![
            TyChirho::int_chirho(),
            TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
        ],
    );
    let result_chirho = env_chirho.resolve_chirho(&pred_chirho);
    assert!(result_chirho.is_some());

    // Convert Int [Double] should NOT match (inconsistent: inst_var=Int vs inst_var=Double)
    let pred2_chirho = PredChirho::new_multi_chirho(
        "Convert",
        vec![
            TyChirho::int_chirho(),
            TyChirho::ListChirho(Box::new(TyChirho::ConChirho("Double".to_string()))),
        ],
    );
    let result2_chirho = env_chirho.resolve_chirho(&pred2_chirho);
    assert!(result2_chirho.is_none());
}

#[test]
fn mptc_entails_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();

    let a_chirho = TyVarChirho(100);
    let b_chirho = TyVarChirho(101);
    env_chirho.add_class_chirho(ClassDeclChirho::legacy_chirho(
        crate::class_chirho::ClassDeclPartsChirho {
            kind_vars_chirho: Vec::new(),
            name_chirho: "Convert".to_string(),
            supers_chirho: vec![],
            var_chirho: a_chirho,
            methods_chirho: HashMap::new(),
            extra_vars_chirho: vec![b_chirho],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        },
    ));

    env_chirho.add_instance_chirho(InstDeclChirho::legacy_chirho(
        crate::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "Convert".to_string(),
            head_ty_chirho: TyChirho::int_chirho(),
            extra_head_tys_chirho: vec![TyChirho::ConChirho("Double".to_string())],
            context_chirho: vec![],
        },
    ));

    assert!(env_chirho.entails_chirho(&PredChirho::new_multi_chirho(
        "Convert",
        vec![
            TyChirho::int_chirho(),
            TyChirho::ConChirho("Double".to_string())
        ],
    )));
    assert!(!env_chirho.entails_chirho(&PredChirho::new_multi_chirho(
        "Convert",
        vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
    )));
}

#[test]
fn mptc_fundeps_stored_chirho() {
    // class Convert a b | a -> b
    let a_chirho = TyVarChirho(100);
    let b_chirho = TyVarChirho(101);
    let decl_chirho = ClassDeclChirho::legacy_chirho(crate::class_chirho::ClassDeclPartsChirho {
        kind_vars_chirho: Vec::new(),
        name_chirho: "Convert".to_string(),
        supers_chirho: vec![],
        var_chirho: a_chirho,
        methods_chirho: HashMap::new(),
        extra_vars_chirho: vec![b_chirho],
        fundeps_chirho: vec![(vec![0], vec![1])], // a -> b
        defaults_chirho: HashMap::new(),
    });

    assert_eq!(decl_chirho.fundeps_chirho.len(), 1);
    assert_eq!(decl_chirho.fundeps_chirho[0], (vec![0], vec![1]));
}

#[test]
fn mptc_fundep_improvement_chirho() {
    // class Convert a b | a -> b
    // instance Convert Int Double
    // Given predicate Convert Int t42, improvement should yield t42 = Double
    let mut env_chirho = ClassEnvChirho::new_chirho();

    let a_chirho = TyVarChirho(100);
    let b_chirho = TyVarChirho(101);
    env_chirho.add_class_chirho(ClassDeclChirho::legacy_chirho(
        crate::class_chirho::ClassDeclPartsChirho {
            kind_vars_chirho: Vec::new(),
            name_chirho: "Convert".to_string(),
            supers_chirho: vec![],
            var_chirho: a_chirho,
            methods_chirho: HashMap::new(),
            extra_vars_chirho: vec![b_chirho],
            fundeps_chirho: vec![(vec![0], vec![1])], // a -> b
            defaults_chirho: HashMap::new(),
        },
    ));

    env_chirho.add_instance_chirho(InstDeclChirho::legacy_chirho(
        crate::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "Convert".to_string(),
            head_ty_chirho: TyChirho::int_chirho(),
            extra_head_tys_chirho: vec![TyChirho::ConChirho("Double".to_string())],
            context_chirho: vec![],
        },
    ));

    // Convert Int ?t42 — fundep should determine ?t42 = Double
    let unknown_chirho = TyVarChirho(42);
    let pred_chirho = PredChirho::new_multi_chirho(
        "Convert",
        vec![TyChirho::int_chirho(), TyChirho::VarChirho(unknown_chirho)],
    );

    let improvement_chirho = env_chirho.fundep_improve_chirho(&pred_chirho);
    assert_eq!(
        improvement_chirho.apply_ty_chirho(&TyChirho::VarChirho(unknown_chirho)),
        TyChirho::ConChirho("Double".to_string())
    );
}

#[test]
fn mptc_fundep_no_improvement_on_mismatch_chirho() {
    // class Convert a b | a -> b
    // instance Convert Int Double
    // Given predicate Convert Bool t42 — no match, no improvement
    let mut env_chirho = ClassEnvChirho::new_chirho();

    let a_chirho = TyVarChirho(100);
    let b_chirho = TyVarChirho(101);
    env_chirho.add_class_chirho(ClassDeclChirho::legacy_chirho(
        crate::class_chirho::ClassDeclPartsChirho {
            kind_vars_chirho: Vec::new(),
            name_chirho: "Convert".to_string(),
            supers_chirho: vec![],
            var_chirho: a_chirho,
            methods_chirho: HashMap::new(),
            extra_vars_chirho: vec![b_chirho],
            fundeps_chirho: vec![(vec![0], vec![1])],
            defaults_chirho: HashMap::new(),
        },
    ));

    env_chirho.add_instance_chirho(InstDeclChirho::legacy_chirho(
        crate::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "Convert".to_string(),
            head_ty_chirho: TyChirho::int_chirho(),
            extra_head_tys_chirho: vec![TyChirho::ConChirho("Double".to_string())],
            context_chirho: vec![],
        },
    ));

    let unknown_chirho = TyVarChirho(42);
    let pred_chirho = PredChirho::new_multi_chirho(
        "Convert",
        vec![TyChirho::bool_chirho(), TyChirho::VarChirho(unknown_chirho)],
    );

    let improvement_chirho = env_chirho.fundep_improve_chirho(&pred_chirho);
    // t42 should remain unimproved
    assert_eq!(
        improvement_chirho.apply_ty_chirho(&TyChirho::VarChirho(unknown_chirho)),
        TyChirho::VarChirho(unknown_chirho)
    );
}
