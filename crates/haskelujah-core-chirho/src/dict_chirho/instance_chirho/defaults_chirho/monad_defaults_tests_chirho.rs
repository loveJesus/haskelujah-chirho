// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The compiler's own Monad declares `>>`'s default, and an instance that writes only `>>=`
//! takes it from its own `>>=` row (row 699). These controls pin every boundary of that
//! authority (gpt_chirho #32268, #32286). The true builtin with a real donor takes it. None of
//! these does:
//! - a missing proof, or a proof for a qualified spelling only;
//! - a namesake schema;
//! - an exact-schema namesake without its own default;
//! - an absent or changed declared default;
//! - a missing or stub donor.
//!
//! A later module's fresh pass never inherits an earlier admission. Over the real seeded
//! classes, RD1's Reader takes its own Applicative and Functor dictionaries in its superclass
//! slots, and a use site finds each one under the constructor.

use std::collections::HashMap;

use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho};
use haskelujah_ast_chirho::name_chirho::{
    BuiltinClassChirho, BuiltinClassScopeChirho, CompilerClassChirho,
};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_typing_chirho::class_chirho::{
    ClassDeclChirho, ClassEnvChirho, InstDeclChirho, monad_then_default_arms_chirho,
};
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::{DictPassCtxChirho, implements_then_default_chirho, monad_default_covers_chirho};
use crate::expr_chirho::{CoreBindingChirho, CoreExprChirho, CoreIdChirho};

/// `ReaderChirho t<variable_chirho>`, as the checker freshens one instance head.
fn reader_head_chirho(variable_chirho: u32) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("ReaderChirho".to_string())),
        Box::new(TyChirho::VarChirho(TyVarChirho(variable_chirho))),
    )
}

/// The compiler's seeded classes, and `instance Functor/Applicative/Monad (ReaderChirho r)`,
/// each over its own variable.
fn reader_env_chirho() -> ClassEnvChirho {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();
    for (class_chirho, variable_chirho) in [("Functor", 0), ("Applicative", 1), ("Monad", 2)] {
        env_chirho.add_instance_chirho(InstDeclChirho {
            class_name_chirho: class_chirho.to_string(),
            head_ty_chirho: reader_head_chirho(variable_chirho),
            extra_head_tys_chirho: Vec::new(),
            context_chirho: Vec::new(),
        });
    }
    env_chirho
}

/// `reader_env_chirho` with its `Monad` entry changed by `change_chirho`, as another producer
/// would have made it.
fn reader_env_with_monad_chirho(
    change_chirho: impl FnOnce(&mut ClassDeclChirho),
) -> ClassEnvChirho {
    let mut env_chirho = reader_env_chirho();
    let mut monad_chirho = env_chirho.classes_chirho["Monad"].clone();
    change_chirho(&mut monad_chirho);
    env_chirho.add_class_chirho(monad_chirho);
    env_chirho
}

/// A builtin class scope in which naming proves that `spelling_chirho` names the compiler's
/// Monad (K01).
fn monad_scope_chirho(spelling_chirho: &str) -> BuiltinClassScopeChirho {
    BuiltinClassScopeChirho::new_chirho(HashMap::from([(
        spelling_chirho.to_string(),
        BuiltinClassChirho {
            identity_chirho: CompilerClassChirho::MonadChirho,
            module_chirho: "GHC.Base",
            name_chirho: "Monad",
        },
    )]))
}

/// RD1's source rows, written under `ReaderChirho r`: every method but `>>`.
const READER_ROWS_CHIRHO: [(&str, &str, u32); 4] = [
    ("Functor", "fmap", 1),
    ("Applicative", "pure", 2),
    ("Applicative", "<*>", 3),
    ("Monad", ">>=", 4),
];

/// A pass over `env_chirho`'s layouts for a module whose builtin class scope is `scope_chirho`,
/// and whose source holds these Reader rows.
fn reader_pass_chirho(
    env_chirho: &ClassEnvChirho,
    scope_chirho: BuiltinClassScopeChirho,
    rows_chirho: &[(&str, &str, u32)],
) -> DictPassCtxChirho {
    let mut pass_chirho = DictPassCtxChirho::new_chirho(100, HashMap::new(), HashMap::new());
    pass_chirho.builtin_classes_chirho = scope_chirho;
    for (class_chirho, method_chirho, id_chirho) in rows_chirho {
        pass_chirho.body_backed_names_chirho.insert(
            format!("$prim_{class_chirho}_{method_chirho}_ReaderChirho r"),
            CoreIdChirho(*id_chirho),
        );
    }
    pass_chirho.build_layouts_chirho(env_chirho);
    pass_chirho
}

/// Whether the Reader's `Monad` instance is a generic head in this pass.
fn reader_monad_is_generic_chirho(
    pass_chirho: &DictPassCtxChirho,
    env_chirho: &ClassEnvChirho,
) -> bool {
    let instance_chirho = env_chirho.instances_chirho["Monad"]
        .iter()
        .find(|instance_chirho| instance_chirho.head_ty_chirho == reader_head_chirho(2))
        .expect("the Reader's Monad instance");
    pass_chirho
        .generic_head_chirho(
            env_chirho,
            "Monad",
            instance_chirho,
            &pass_chirho.layouts_chirho["Monad"],
            "(ReaderChirho t2)",
        )
        .is_some()
}

/// The `>>` default row for the Reader's Monad, under its typed key.
fn then_default_chirho(
    pass_chirho: &mut DictPassCtxChirho,
    env_chirho: &ClassEnvChirho,
) -> Option<CoreIdChirho> {
    pass_chirho.monad_default_row_chirho(
        env_chirho,
        "Monad",
        ">>",
        "ReaderChirho r",
        "$prim_Monad_>>_(ReaderChirho t2)",
    )
}

/// Whether the Reader takes the `>>` default with RD1's rows in a module whose builtin class
/// scope is `scope_chirho`. G5's coverage, the generic head and the row itself must agree.
fn takes_then_default_chirho(
    env_chirho: &ClassEnvChirho,
    scope_chirho: BuiltinClassScopeChirho,
) -> bool {
    let covered_chirho =
        monad_default_covers_chirho(env_chirho, &scope_chirho, "Monad", ">>", |donor_chirho| {
            donor_chirho == ">>="
        });
    let mut pass_chirho = reader_pass_chirho(env_chirho, scope_chirho, &READER_ROWS_CHIRHO);
    let generic_chirho = reader_monad_is_generic_chirho(&pass_chirho, env_chirho);
    let row_chirho = then_default_chirho(&mut pass_chirho, env_chirho).is_some();
    assert_eq!(
        (covered_chirho, generic_chirho),
        (row_chirho, row_chirho),
        "G5 coverage, the generic head and the default row disagree"
    );
    row_chirho
}

fn generated_chirho<'a>(
    pass_chirho: &'a DictPassCtxChirho,
    name_chirho: &str,
) -> &'a CoreBindingChirho {
    pass_chirho
        .generated_bindings_chirho
        .iter()
        .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == name_chirho)
        .unwrap_or_else(|| panic!("no generated binding {name_chirho}"))
}

/// The fields of the dictionary the pass generated as `name_chirho`.
fn fields_chirho(pass_chirho: &DictPassCtxChirho, name_chirho: &str) -> Vec<CoreExprChirho> {
    match &generated_chirho(pass_chirho, name_chirho).rhs_chirho {
        CoreExprChirho::ConAppChirho { args_chirho, .. } => args_chirho.clone(),
        other_chirho => panic!("{name_chirho} is not a dictionary: {other_chirho:?}"),
    }
}

fn var_chirho(id_chirho: CoreIdChirho) -> CoreExprChirho {
    CoreExprChirho::VarChirho(id_chirho)
}

#[test]
fn only_the_compilers_monad_defaults_then_from_bind_chirho() {
    let compiler_chirho = reader_env_chirho();
    // The seeded Monad declares exactly typing's canonical arms, the ones the row implements.
    assert_eq!(
        compiler_chirho.classes_chirho["Monad"]
            .defaults_chirho
            .get(">>"),
        Some(&monad_then_default_arms_chirho())
    );
    assert!(implements_then_default_chirho(
        &monad_then_default_arms_chirho()
    ));
    // The true builtin with a real donor, under naming's bare proof.
    assert!(takes_then_default_chirho(
        &compiler_chirho,
        monad_scope_chirho("Monad")
    ));
    // `>>` reads `>>=`, only while `>>=` is covered, and for no other method or class.
    let bare_chirho = monad_scope_chirho("Monad");
    for (class_chirho, method_chirho, donor_covered_chirho) in [
        ("Monad", ">>", false),
        ("Monad", ">>=", true),
        ("Applicative", ">>", true),
    ] {
        assert!(
            !monad_default_covers_chirho(
                &compiler_chirho,
                &bare_chirho,
                class_chirho,
                method_chirho,
                |_| donor_covered_chirho
            ),
            "{class_chirho}.{method_chirho}"
        );
    }
    // No proof, and a proof for a qualified spelling only: the bare key is never blessed.
    assert!(!takes_then_default_chirho(
        &compiler_chirho,
        BuiltinClassScopeChirho::default()
    ));
    assert!(!takes_then_default_chirho(
        &compiler_chirho,
        monad_scope_chirho("P.Monad")
    ));
    // A namesake schema, the program's own variable, even under the bare proof.
    let namesake_chirho = reader_env_with_monad_chirho(|monad_chirho| {
        monad_chirho.var_chirho = TyVarChirho(77);
    });
    assert!(!takes_then_default_chirho(
        &namesake_chirho,
        monad_scope_chirho("Monad")
    ));
    // An absent declared default, and a changed one (`m >> k = k`): nothing stands in for it.
    let absent_chirho = reader_env_with_monad_chirho(|monad_chirho| {
        monad_chirho.defaults_chirho.remove(">>");
    });
    assert!(!takes_then_default_chirho(
        &absent_chirho,
        monad_scope_chirho("Monad")
    ));
    let changed_chirho = reader_env_with_monad_chirho(|monad_chirho| {
        let mut arms_chirho = monad_then_default_arms_chirho();
        let PatChirho::VarChirho(next_chirho) = arms_chirho[0].pats_chirho[1].clone() else {
            panic!("the canonical arms bind `k` second");
        };
        arms_chirho[0].rhs_chirho = RhsChirho::UnguardedChirho(ExprChirho::VarChirho(next_chirho));
        assert!(!implements_then_default_chirho(&arms_chirho));
        monad_chirho
            .defaults_chirho
            .insert(">>".to_string(), arms_chirho);
    });
    assert!(!takes_then_default_chirho(
        &changed_chirho,
        monad_scope_chirho("Monad")
    ));
}

#[test]
fn a_missing_or_stub_bind_row_never_donates_then_chirho() {
    let env_chirho = reader_env_chirho();
    let without_bind_chirho: Vec<_> = READER_ROWS_CHIRHO
        .into_iter()
        .filter(|(_, method_chirho, _)| *method_chirho != ">>=")
        .collect();
    let mut pass_chirho = reader_pass_chirho(
        &env_chirho,
        monad_scope_chirho("Monad"),
        &without_bind_chirho,
    );
    assert!(!reader_monad_is_generic_chirho(&pass_chirho, &env_chirho));
    assert_eq!(then_default_chirho(&mut pass_chirho, &env_chirho), None);
    // A missing-method stub under the `>>=` row's name has no body to donate.
    pass_chirho.missing_method_row_chirho(
        None,
        "$prim_Monad_>>=_ReaderChirho r",
        "missing method Monad.>>= for (ReaderChirho t2)".to_string(),
        2,
        None,
    );
    assert_eq!(then_default_chirho(&mut pass_chirho, &env_chirho), None);
}

#[test]
fn a_reader_monad_holds_its_own_superclasses_and_then_from_its_bind_chirho() {
    // RD1's shape: the instance writes `>>=` alone. Monad's Applicative slot holds the Reader's
    // own Applicative dictionary, and that dictionary's Functor slot the Reader's Functor
    // dictionary, each typed over a variable of its own.
    let env_chirho = reader_env_chirho();
    let mut pass_chirho = reader_pass_chirho(
        &env_chirho,
        monad_scope_chirho("Monad"),
        &READER_ROWS_CHIRHO,
    );
    assert!(reader_monad_is_generic_chirho(&pass_chirho, &env_chirho));
    pass_chirho.generate_instance_dicts_chirho(&env_chirho);
    let dictionary_chirho = |name_chirho: &str| {
        generated_chirho(&pass_chirho, name_chirho)
            .binder_chirho
            .id_chirho
    };
    let functor_chirho = dictionary_chirho("$fFunctor(ReaderChirho t0)");
    let applicative_chirho = dictionary_chirho("$fApplicative(ReaderChirho t1)");
    let monad_chirho = dictionary_chirho("$fMonad(ReaderChirho t2)");
    let then_chirho = dictionary_chirho("$prim_Monad_>>_(ReaderChirho t2)");
    assert_eq!(
        fields_chirho(&pass_chirho, "$fApplicative(ReaderChirho t1)")[0],
        var_chirho(functor_chirho)
    );
    // Monad's layout: its Applicative, then `>>` and `>>=` in sorted slots.
    assert_eq!(
        fields_chirho(&pass_chirho, "$fMonad(ReaderChirho t2)"),
        vec![
            var_chirho(applicative_chirho),
            var_chirho(then_chirho),
            var_chirho(CoreIdChirho(4)),
        ]
    );
    // `>>` is `\m k -> bind m (\_ -> k)`, over this instance's own `>>=` row.
    let CoreExprChirho::LamChirho {
        binder_chirho: action_chirho,
        body_chirho: outer_chirho,
    } = &generated_chirho(&pass_chirho, "$prim_Monad_>>_(ReaderChirho t2)").rhs_chirho
    else {
        panic!("`>>` takes the action first");
    };
    let CoreExprChirho::LamChirho {
        binder_chirho: next_chirho,
        body_chirho: bind_chirho,
    } = outer_chirho.as_ref()
    else {
        panic!("`>>` takes the next action second");
    };
    let CoreExprChirho::AppChirho {
        fun_chirho: bound_chirho,
        arg_chirho: continuation_chirho,
    } = bind_chirho.as_ref()
    else {
        panic!("`>>` applies `>>=`");
    };
    assert_eq!(
        bound_chirho.as_ref(),
        &CoreExprChirho::AppChirho {
            fun_chirho: Box::new(var_chirho(CoreIdChirho(4))),
            arg_chirho: Box::new(var_chirho(action_chirho.id_chirho)),
        }
    );
    let CoreExprChirho::LamChirho {
        binder_chirho: ignored_chirho,
        body_chirho: kept_chirho,
    } = continuation_chirho.as_ref()
    else {
        panic!("`>>=` takes a continuation");
    };
    assert_ne!(ignored_chirho.id_chirho, next_chirho.id_chirho);
    assert_eq!(kept_chirho.as_ref(), &var_chirho(next_chirho.id_chirho));
    // A use site finds each dictionary under the constructor it carries.
    for (class_chirho, id_chirho) in [
        ("Functor", functor_chirho),
        ("Applicative", applicative_chirho),
        ("Monad", monad_chirho),
    ] {
        assert_eq!(
            pass_chirho
                .instance_dicts_chirho
                .get(&(class_chirho.to_string(), "ReaderChirho".to_string())),
            Some(&id_chirho),
            "{class_chirho}"
        );
    }
}

#[test]
fn a_same_schema_namesake_without_a_producer_proof_never_takes_the_default_chirho() {
    // #32268: equal compared fields are not identity. A program's own `class Monad m`, made by
    // its declaration with the seed's variable, method schemes, superclasses and templates and
    // no default of its own, replaces the seed's entry under that name. Nothing hands Core a
    // proof for it, so it takes no default. It takes none under a bare proof either, since it
    // declares no default of its own (#32286). This was the seam's RED checkpoint at ce9e9f65.
    let seed_chirho = reader_env_chirho().classes_chirho["Monad"].clone();
    let namesake_chirho = reader_env_with_monad_chirho(|monad_chirho| {
        *monad_chirho = ClassDeclChirho {
            name_chirho: "Monad".to_string(),
            supers_chirho: seed_chirho.supers_chirho,
            super_templates_chirho: seed_chirho.super_templates_chirho,
            var_chirho: seed_chirho.var_chirho,
            methods_chirho: seed_chirho.methods_chirho,
            extra_vars_chirho: seed_chirho.extra_vars_chirho,
            fundeps_chirho: seed_chirho.fundeps_chirho,
            defaults_chirho: HashMap::new(),
        };
    });
    assert!(!takes_then_default_chirho(
        &namesake_chirho,
        BuiltinClassScopeChirho::default()
    ));
    assert!(!takes_then_default_chirho(
        &namesake_chirho,
        monad_scope_chirho("Monad")
    ));
}

#[test]
fn a_later_module_never_inherits_an_earlier_admission_chirho() {
    // The scope is each module's own. After one module's pass admitted the default, a later
    // module's fresh pass over the same class environment, through a legacy entry point and so
    // with an empty scope, admits nothing (#32286).
    let env_chirho = reader_env_chirho();
    assert!(takes_then_default_chirho(
        &env_chirho,
        monad_scope_chirho("Monad")
    ));
    let mut later_chirho = DictPassCtxChirho::new_chirho(100, HashMap::new(), HashMap::new());
    for (class_chirho, method_chirho, id_chirho) in READER_ROWS_CHIRHO {
        later_chirho.body_backed_names_chirho.insert(
            format!("$prim_{class_chirho}_{method_chirho}_ReaderChirho r"),
            CoreIdChirho(id_chirho),
        );
    }
    later_chirho.build_layouts_chirho(&env_chirho);
    assert!(!reader_monad_is_generic_chirho(&later_chirho, &env_chirho));
    assert_eq!(then_default_chirho(&mut later_chirho, &env_chirho), None);
}
