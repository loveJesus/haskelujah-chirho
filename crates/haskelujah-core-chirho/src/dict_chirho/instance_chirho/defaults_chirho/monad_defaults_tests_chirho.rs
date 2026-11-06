// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The compiler's own Monad defaults `>>` from this very instance's `>>=` row (row 699,
//! #32248). A program's namesake never gains the default, and neither does an instance whose
//! `>>=` row is missing or a stub. Over the real seeded classes, RD1's Reader takes its own
//! Applicative and Functor dictionaries in its superclass slots, and a use site finds each one
//! under the constructor.
//!
//! The authority is open (#32268). A namesake with the seed's exact compared fields carries no
//! producer proof, so it must take no default. Its control is RED while the seeded-schema
//! comparison stands, and turns green only when a producer proof reaches Core.

use std::collections::HashMap;

use haskelujah_typing_chirho::class_chirho::{ClassDeclChirho, ClassEnvChirho, InstDeclChirho};
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::{DictPassCtxChirho, monad_default_covers_chirho};
use crate::expr_chirho::{CoreBindingChirho, CoreExprChirho, CoreIdChirho};

/// `ReaderChirho t<variable_chirho>`, as the checker freshens one instance head.
fn reader_head_chirho(variable_chirho: u32) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("ReaderChirho".to_string())),
        Box::new(TyChirho::VarChirho(TyVarChirho(variable_chirho))),
    )
}

/// The compiler's seeded classes, with `Monad` replaced by `monad_chirho` when given, and
/// `instance Functor/Applicative/Monad (ReaderChirho r)`, each over its own variable.
fn reader_env_chirho(monad_chirho: Option<u32>) -> ClassEnvChirho {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();
    if let Some(variable_chirho) = monad_chirho {
        let mut namesake_chirho = env_chirho.classes_chirho["Monad"].clone();
        namesake_chirho.var_chirho = TyVarChirho(variable_chirho);
        env_chirho.add_class_chirho(namesake_chirho);
    }
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

/// RD1's source rows, written under `ReaderChirho r`: every method but `>>`.
const READER_ROWS_CHIRHO: [(&str, &str, u32); 4] = [
    ("Functor", "fmap", 1),
    ("Applicative", "pure", 2),
    ("Applicative", "<*>", 3),
    ("Monad", ">>=", 4),
];

/// A pass over `env_chirho`'s layouts whose source holds these Reader rows.
fn reader_pass_chirho(
    env_chirho: &ClassEnvChirho,
    rows_chirho: &[(&str, &str, u32)],
) -> DictPassCtxChirho {
    let mut pass_chirho = DictPassCtxChirho::new_chirho(100, HashMap::new(), HashMap::new());
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
    let compiler_chirho = reader_env_chirho(None);
    // `>>` reads `>>=`, and only while `>>=` is covered.
    assert!(monad_default_covers_chirho(
        &compiler_chirho,
        "Monad",
        ">>",
        |donor_chirho| donor_chirho == ">>="
    ));
    assert!(!monad_default_covers_chirho(
        &compiler_chirho,
        "Monad",
        ">>",
        |_| false
    ));
    assert!(!monad_default_covers_chirho(
        &compiler_chirho,
        "Monad",
        ">>=",
        |_| true
    ));
    assert!(!monad_default_covers_chirho(
        &compiler_chirho,
        "Applicative",
        ">>",
        |_| true
    ));
    // The program's own `class Monad m` replaces the seed under that name. Its methods and
    // superclasses are the seed's here, but its variable is the program's, so the schema is not
    // the compiler's: a class spelling and a matching pair of method names are no authority.
    let namesake_chirho = reader_env_chirho(Some(77));
    assert!(!monad_default_covers_chirho(
        &namesake_chirho,
        "Monad",
        ">>",
        |_| true
    ));
    let mut pass_chirho = reader_pass_chirho(&namesake_chirho, &READER_ROWS_CHIRHO);
    assert!(!reader_monad_is_generic_chirho(
        &pass_chirho,
        &namesake_chirho
    ));
    assert_eq!(
        then_default_chirho(&mut pass_chirho, &namesake_chirho),
        None
    );
}

#[test]
fn a_missing_or_stub_bind_row_never_donates_then_chirho() {
    let env_chirho = reader_env_chirho(None);
    let without_bind_chirho: Vec<_> = READER_ROWS_CHIRHO
        .into_iter()
        .filter(|(_, method_chirho, _)| *method_chirho != ">>=")
        .collect();
    let mut pass_chirho = reader_pass_chirho(&env_chirho, &without_bind_chirho);
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
    let env_chirho = reader_env_chirho(None);
    let mut pass_chirho = reader_pass_chirho(&env_chirho, &READER_ROWS_CHIRHO);
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
    // producer proof for it, so it takes no default, like a namesake of any other schema. The
    // seeded-schema comparison accepts it today: this control exposes that seam, and is RED
    // until a producer proof reaches Core.
    let mut env_chirho = reader_env_chirho(None);
    let seed_chirho = env_chirho.classes_chirho["Monad"].clone();
    env_chirho.add_class_chirho(ClassDeclChirho {
        name_chirho: "Monad".to_string(),
        supers_chirho: seed_chirho.supers_chirho,
        super_templates_chirho: seed_chirho.super_templates_chirho,
        var_chirho: seed_chirho.var_chirho,
        methods_chirho: seed_chirho.methods_chirho,
        extra_vars_chirho: seed_chirho.extra_vars_chirho,
        fundeps_chirho: seed_chirho.fundeps_chirho,
        defaults_chirho: HashMap::new(),
    });
    assert!(!monad_default_covers_chirho(
        &env_chirho,
        "Monad",
        ">>",
        |_| true
    ));
    let mut pass_chirho = reader_pass_chirho(&env_chirho, &READER_ROWS_CHIRHO);
    assert!(!reader_monad_is_generic_chirho(&pass_chirho, &env_chirho));
    assert_eq!(then_default_chirho(&mut pass_chirho, &env_chirho), None);
}
