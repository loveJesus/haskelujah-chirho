// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Recovery must not publish a failed default as checked kind evidence.
//! Source-level acceptance controls live in driver/import_contracts/default_scopes.
use super::*;
use haskelujah_ast_chirho::decl_chirho::{AssocTypeFamilyChirho, TypeFamilyEquationChirho};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho};

fn span_chirho() -> SpanChirho {
    SpanChirho::new_chirho(
        FileIdChirho::SYNTHETIC_CHIRHO,
        ByteOffsetChirho::new_chirho(1),
        ByteOffsetChirho::new_chirho(40),
    )
}

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn var_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(name_chirho(text_chirho))
}

fn annotated_chirho(text_chirho: &str, kind_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::KindAnnotChirho {
        type_chirho: Box::new(var_chirho(text_chirho)),
        kind_chirho: Box::new(kind_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn context_chirho(argument_kinds_chirho: Vec<KindChirho>) -> KindInferCtxChirho {
    let mut environment_chirho = KindEnvChirho::with_builtins_chirho();
    environment_chirho.bind_generalized_chirho(
        "FChirho".into(),
        KindChirho::arrow_n_chirho(argument_kinds_chirho, KindChirho::StarChirho),
    );
    let mut context_chirho = KindInferCtxChirho::new_chirho(environment_chirho);
    context_chirho
        .local_kind_decl_names_chirho
        .insert("FChirho".into());
    context_chirho
}

fn module_chirho(arguments_chirho: Vec<TypeChirho>) -> ModuleChirho {
    let family_chirho = AssocTypeFamilyChirho {
        name_chirho: name_chirho("FChirho"),
        type_vars_chirho: arguments_chirho
            .iter()
            .map(|argument_chirho| {
                let TypeChirho::VarChirho(name_chirho) = argument_chirho.unannotated_chirho()
                else {
                    panic!("this recovery fixture needs variable default arguments")
                };
                TyVarChirho::plain_chirho(name_chirho.clone())
            })
            .collect(),
        result_chirho: Default::default(),
        data_chirho: false,
        head_declared_chirho: true,
        defaults_chirho: vec![TypeFamilyEquationChirho {
            lhs_types_chirho: arguments_chirho,
            rhs_chirho: TypeChirho::ConChirho(name_chirho("Int")),
            span_chirho: span_chirho(),
        }],
        span_chirho: span_chirho(),
    };
    ModuleChirho {
        name_chirho: name_chirho("DefaultRecoveryChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::ClassDeclChirho {
            name_chirho: name_chirho("CChirho"),
            context_chirho: vec![],
            context_written_chirho: false,
            minimal_chirho: None,
            type_vars_chirho: family_chirho.type_vars_chirho.clone(),
            methods_chirho: vec![],
            associated_tfs_chirho: vec![family_chirho],
            fundeps_chirho: vec![],
            span_chirho: span_chirho(),
        }],
        extensions_chirho: vec!["TypeFamilies".into(), "PolyKinds".into()],
        inline_pragmas_chirho: Default::default(),
        specialize_pragmas_chirho: Default::default(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: span_chirho(),
    }
}

fn assert_unpublished_chirho(context_chirho: &KindInferCtxChirho, reason_chirho: &str) {
    assert!(context_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(
        context_chirho
            .diagnostics_chirho
            .to_string()
            .contains(reason_chirho),
        "{:?}",
        context_chirho.diagnostics_chirho
    );
    assert!(
        !context_chirho
            .kind_applications_chirho
            .contains_key(&span_chirho()),
        "a failed default must not be usable as checked kind evidence"
    );
}

#[test]
fn default_kind_error_does_not_publish_an_application_chirho() {
    let mut context_chirho = context_chirho(vec![KindChirho::StarChirho]);
    context_chirho.check_associated_default_kinds_chirho(&module_chirho(vec![annotated_chirho(
        "aChirho",
        var_chirho("kChirho"),
    )]));
    assert_unpublished_chirho(&context_chirho, "kind mismatch");
}

#[test]
fn specialized_hidden_default_kind_does_not_publish_an_application_chirho() {
    let mut context_chirho = context_chirho(vec![KindChirho::VarChirho(KindVarChirho(0))]);
    context_chirho.check_associated_default_kinds_chirho(&module_chirho(vec![annotated_chirho(
        "aChirho",
        TypeChirho::ConChirho(name_chirho("Type")),
    )]));
    assert_unpublished_chirho(&context_chirho, "distinct variable kind arguments");
}

#[test]
fn merged_hidden_default_kinds_do_not_publish_an_application_chirho() {
    let mut context_chirho = context_chirho(vec![
        KindChirho::VarChirho(KindVarChirho(0)),
        KindChirho::VarChirho(KindVarChirho(1)),
    ]);
    context_chirho.check_associated_default_kinds_chirho(&module_chirho(vec![
        annotated_chirho("aChirho", var_chirho("kChirho")),
        annotated_chirho("bChirho", var_chirho("kChirho")),
    ]));
    assert_unpublished_chirho(&context_chirho, "distinct variable kind arguments");
}

#[test]
fn default_annotation_cannot_inherit_a_resolved_outer_identity_chirho() {
    let mut context_chirho = context_chirho(vec![KindChirho::StarChirho]);
    let outer_chirho = context_chirho.type_to_kind_chirho(&var_chirho("kChirho"));
    context_chirho.unify_chirho(
        &outer_chirho,
        &KindChirho::StarChirho,
        "outer checking context",
        SpanChirho::DUMMY_CHIRHO,
    );
    let outer_cache_chirho = context_chirho.kind_var_cache_chirho.clone();
    context_chirho.check_associated_default_kinds_chirho(&module_chirho(vec![annotated_chirho(
        "aChirho",
        var_chirho("kChirho"),
    )]));
    assert_unpublished_chirho(&context_chirho, "kind mismatch");
    assert_eq!(context_chirho.kind_var_cache_chirho, outer_cache_chirho);
    assert_eq!(
        context_chirho.subst_chirho.apply_chirho(&outer_chirho),
        KindChirho::StarChirho
    );
}

#[test]
fn default_argument_shadows_an_outer_type_variable_and_restores_it_chirho() {
    let mut context_chirho = context_chirho(vec![KindChirho::StarChirho]);
    let outer_chirho = KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
    context_chirho
        .env_chirho
        .bind_chirho("aChirho".into(), outer_chirho.clone());
    context_chirho
        .check_associated_default_kinds_chirho(&module_chirho(vec![var_chirho("aChirho")]));
    assert!(
        !context_chirho.diagnostics_chirho.has_errors_chirho(),
        "{:?}",
        context_chirho.diagnostics_chirho
    );
    assert!(
        context_chirho
            .kind_applications_chirho
            .contains_key(&span_chirho())
    );
    assert_eq!(
        context_chirho.env_chirho.lookup_chirho("aChirho"),
        Some(&outer_chirho)
    );
}

#[test]
fn unrelated_prior_error_does_not_prevent_a_valid_default_record_chirho() {
    let mut context_chirho = context_chirho(vec![KindChirho::StarChirho]);
    context_chirho
        .diagnostics_chirho
        .push_chirho(DiagnosticChirho::error_with_code_chirho(
            ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
            "unrelated declaration failed",
            SpanChirho::DUMMY_CHIRHO,
        ));
    context_chirho
        .check_associated_default_kinds_chirho(&module_chirho(vec![var_chirho("aChirho")]));
    assert_eq!(context_chirho.diagnostics_chirho.error_count_chirho(), 1);
    assert!(
        context_chirho
            .kind_applications_chirho
            .contains_key(&span_chirho())
    );
}
