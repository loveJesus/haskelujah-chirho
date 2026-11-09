// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The S3 join pairs a typed instance with its rows only when each producer reports exactly
//! one thing at the declaration's class and span. A shared span with different heads stays
//! ambiguous, by name, and never becomes a choice.

use super::instance_rows_chirho::{
    InstanceDeclarationChirho, InstanceRowKeyChirho, InstanceRowsChirho, RowKeyAbsenceChirho,
};
use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho, SpanChirho};
use haskelujah_typing_chirho::class_chirho::{InstDeclChirho, PredChirho};
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

fn span_chirho(start_chirho: u32) -> SpanChirho {
    SpanChirho::new_chirho(
        FileIdChirho::SYNTHETIC_CHIRHO,
        ByteOffsetChirho::new_chirho(start_chirho),
        ByteOffsetChirho::new_chirho(start_chirho + 10),
    )
}

/// `instance SizeChirho a => SizeChirho (<head> a)` over the freshened variable `var`.
fn context_instance_chirho(head_chirho: &str, var_chirho: u32) -> InstDeclChirho {
    let a_chirho = TyChirho::VarChirho(TyVarChirho(var_chirho));
    InstDeclChirho::legacy_chirho(
        haskelujah_typing_chirho::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "SizeChirho".to_string(),
            head_ty_chirho: TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho(head_chirho.to_string())),
                Box::new(a_chirho.clone()),
            ),
            extra_head_tys_chirho: Vec::new(),
            context_chirho: vec![PredChirho::new_chirho("SizeChirho", a_chirho)],
        },
    )
}

fn declared_chirho(
    instance_chirho: &InstDeclChirho,
    start_chirho: u32,
) -> InstanceDeclarationChirho {
    InstanceDeclarationChirho {
        span_chirho: span_chirho(start_chirho),
        instance_chirho: std::sync::Arc::new(instance_chirho.clone()),
    }
}

fn row_key_chirho(row_key_chirho: &str, start_chirho: u32) -> InstanceRowKeyChirho {
    InstanceRowKeyChirho {
        class_name_chirho: "SizeChirho".to_string(),
        span_chirho: span_chirho(start_chirho),
        row_key_chirho: row_key_chirho.to_string(),
    }
}

#[test]
fn one_instance_and_one_key_at_a_place_join_chirho() {
    let maybe_chirho = context_instance_chirho("Maybe", 9001);
    let rows_chirho = InstanceRowsChirho::join_chirho(
        &[declared_chirho(&maybe_chirho, 100)],
        &[row_key_chirho("Maybe aChirho", 100)],
    );
    assert_eq!(
        rows_chirho.row_key_chirho(&maybe_chirho),
        Ok("Maybe aChirho")
    );
    assert_eq!(rows_chirho.proven_rows_chirho().count(), 1);
}

#[test]
fn a_shared_span_with_different_heads_is_ambiguous_while_a_normal_place_joins_chirho() {
    let maybe_chirho = context_instance_chirho("Maybe", 9001);
    let box_chirho = context_instance_chirho("BoxChirho", 9002);
    let list_chirho = context_instance_chirho("ListChirho", 9003);
    let rows_chirho = InstanceRowsChirho::join_chirho(
        &[
            declared_chirho(&maybe_chirho, 0),
            declared_chirho(&box_chirho, 0),
            declared_chirho(&list_chirho, 200),
        ],
        &[
            row_key_chirho("Maybe aChirho", 0),
            row_key_chirho("BoxChirho aChirho", 0),
            row_key_chirho("ListChirho aChirho", 200),
        ],
    );
    assert_eq!(
        rows_chirho.row_key_chirho(&maybe_chirho),
        Err(RowKeyAbsenceChirho::AmbiguousChirho)
    );
    assert_eq!(
        rows_chirho.row_key_chirho(&box_chirho),
        Err(RowKeyAbsenceChirho::AmbiguousChirho)
    );
    assert_eq!(
        rows_chirho.row_key_chirho(&list_chirho),
        Ok("ListChirho aChirho")
    );
    let proven_chirho: Vec<&str> = rows_chirho
        .proven_rows_chirho()
        .map(|(_, row_key_chirho)| row_key_chirho)
        .collect();
    assert_eq!(proven_chirho, vec!["ListChirho aChirho"]);
}

#[test]
fn exact_repeats_of_one_object_and_one_key_deduplicate_chirho() {
    let maybe_chirho = context_instance_chirho("Maybe", 9001);
    let rows_chirho = InstanceRowsChirho::join_chirho(
        &[
            declared_chirho(&maybe_chirho, 100),
            declared_chirho(&maybe_chirho, 100),
        ],
        &[
            row_key_chirho("Maybe aChirho", 100),
            row_key_chirho("Maybe aChirho", 100),
        ],
    );
    assert_eq!(
        rows_chirho.row_key_chirho(&maybe_chirho),
        Ok("Maybe aChirho")
    );
}

#[test]
fn two_keys_at_one_instance_place_are_ambiguous_chirho() {
    let maybe_chirho = context_instance_chirho("Maybe", 9001);
    let rows_chirho = InstanceRowsChirho::join_chirho(
        &[declared_chirho(&maybe_chirho, 100)],
        &[
            row_key_chirho("Maybe aChirho", 100),
            row_key_chirho("Maybe bChirho", 100),
        ],
    );
    assert_eq!(
        rows_chirho.row_key_chirho(&maybe_chirho),
        Err(RowKeyAbsenceChirho::AmbiguousChirho)
    );
}

#[test]
fn an_instance_no_declaration_reports_with_rows_lives_elsewhere_chirho() {
    let maybe_chirho = context_instance_chirho("Maybe", 9001);
    let imported_chirho = context_instance_chirho("Maybe", 9004);
    let rows_chirho = InstanceRowsChirho::join_chirho(
        &[declared_chirho(&maybe_chirho, 100)],
        &[row_key_chirho("Maybe aChirho", 100)],
    );
    // Written alike, but another object: its variables were freshened apart.
    assert_eq!(
        rows_chirho.row_key_chirho(&imported_chirho),
        Err(RowKeyAbsenceChirho::ElsewhereChirho)
    );
}

#[test]
fn eager_slots_use_only_their_canonical_declarations_rows_chirho() {
    use std::collections::HashMap;
    use std::sync::Arc;

    use super::DictPassCtxChirho;
    use crate::expr_chirho::{
        BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho,
        CoreModuleChirho, InlineAnnotationChirho,
    };
    use haskelujah_typing_chirho::class_chirho::{
        ClassDeclChirho, ClassDeclPartsChirho, ClassEnvChirho, InstDeclPartsChirho,
    };
    use haskelujah_typing_chirho::ty_chirho::SchemeChirho;

    let char_chirho = TyChirho::ConChirho("Char".to_string());
    let string_chirho = TyChirho::ListChirho(Box::new(char_chirho.clone()));
    let parts_chirho = InstDeclPartsChirho {
        kind_tys_chirho: Vec::new(),
        class_name_chirho: "SizeChirho".to_string(),
        head_ty_chirho: char_chirho.clone(),
        extra_head_tys_chirho: vec![string_chirho.clone()],
        context_chirho: Vec::new(),
    };
    let body_chirho = |name_chirho: &str, number_chirho: u32| CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(number_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::fun_chirho(char_chirho.clone(), string_chirho.clone()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LamChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(number_chirho + 1),
                name_chirho: "inputChirho".to_string(),
                ty_chirho: char_chirho.clone(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                "bodyChirho".to_string(),
            ))),
        },
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    };
    for case_chirho in [
        "joinedChirho",
        "ambiguousChirho",
        "foreignChirho",
        "elsewhereChirho",
        "missingChirho",
        "collisionChirho",
        "writtenOnlyChirho",
        "noRowsChirho",
    ] {
        let instance_chirho = Arc::new(InstDeclChirho::legacy_chirho(parts_chirho.clone()));
        let mut environment_chirho = ClassEnvChirho::new_chirho();
        environment_chirho.add_class_chirho(ClassDeclChirho::legacy_chirho(ClassDeclPartsChirho {
            kind_vars_chirho: Vec::new(),
            name_chirho: "SizeChirho".to_string(),
            supers_chirho: Vec::new(),
            super_templates_chirho: Vec::new(),
            var_chirho: TyVarChirho(1000),
            methods_chirho: HashMap::from([(
                "sizeChirho".to_string(),
                SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                    TyChirho::VarChirho(TyVarChirho(1000)),
                    TyChirho::VarChirho(TyVarChirho(1001)),
                )),
            )]),
            extra_vars_chirho: vec![TyVarChirho(1001)],
            fundeps_chirho: Vec::new(),
            defaults_chirho: HashMap::new(),
        }));
        environment_chirho.add_shared_instance_chirho(instance_chirho.clone());
        let owner_chirho = match case_chirho {
            // Deliberately equal in the OLD structural join, but not the canonical object.
            "foreignChirho" => Arc::new((*instance_chirho).clone()),
            "elsewhereChirho" => {
                let mut foreign_parts_chirho = parts_chirho.clone();
                foreign_parts_chirho.context_chirho =
                    vec![PredChirho::new_chirho("Eq", char_chirho.clone())];
                Arc::new(InstDeclChirho::legacy_chirho(foreign_parts_chirho))
            }
            _ => instance_chirho.clone(),
        };
        let mut declarations_chirho = vec![InstanceDeclarationChirho {
            span_chirho: span_chirho(100),
            instance_chirho: owner_chirho,
        }];
        let mut keys_chirho = vec![row_key_chirho(
            if case_chirho == "missingChirho" {
                "Char_AliasChirho"
            } else {
                "Char_String"
            },
            100,
        )];
        if case_chirho == "ambiguousChirho" {
            keys_chirho.push(row_key_chirho("Char_AliasChirho", 100));
        }
        if case_chirho == "collisionChirho" {
            let mut other_parts_chirho = parts_chirho.clone();
            other_parts_chirho.head_ty_chirho = TyChirho::ConChirho("Bool".to_string());
            declarations_chirho.push(InstanceDeclarationChirho {
                span_chirho: span_chirho(200),
                instance_chirho: Arc::new(InstDeclChirho::legacy_chirho(other_parts_chirho)),
            });
            keys_chirho.push(row_key_chirho("Char_String", 200));
        }
        if case_chirho == "writtenOnlyChirho" {
            declarations_chirho.clear();
            keys_chirho = vec![row_key_chirho("Char_[Char]", 100)];
        }
        if case_chirho == "noRowsChirho" {
            // A declaration with NO row contribution must not invent a negative claim.
            keys_chirho.clear();
        }
        let module_chirho = CoreModuleChirho {
            name_chirho: "EagerRowsChirho".to_string(),
            bindings_chirho: vec![
                body_chirho("$prim_SizeChirho_sizeChirho_Char_String", 10),
                // The exact typed spelling is a real BODY, but it is not this declaration's row.
                body_chirho("$prim_SizeChirho_sizeChirho_Char_[Char]", 20),
            ],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: Vec::new(),
            provenance_chirho: Default::default(),
        };
        let mut pass_chirho = DictPassCtxChirho::new_chirho(100, HashMap::new(), HashMap::new());
        pass_chirho.seed_body_backed_bindings_chirho(&module_chirho);
        pass_chirho.build_layouts_chirho(&environment_chirho);
        pass_chirho.instance_rows_chirho =
            InstanceRowsChirho::join_chirho(&declarations_chirho, &keys_chirho);
        if case_chirho == "foreignChirho" {
            assert_eq!(
                pass_chirho
                    .instance_rows_chirho
                    .row_key_chirho(&instance_chirho),
                Ok("Char_String")
            );
        }
        let mut bootstrap_chirho =
            DictPassCtxChirho::new_chirho(100, HashMap::new(), HashMap::new());
        bootstrap_chirho.seed_body_backed_bindings_chirho(&module_chirho);
        bootstrap_chirho.build_layouts_chirho(&environment_chirho);
        bootstrap_chirho.instance_rows_chirho =
            InstanceRowsChirho::join_chirho(&declarations_chirho, &keys_chirho);
        let rows_chirho = bootstrap_chirho.eager_instance_rows_chirho();
        let bootstrap_route_chirho = bootstrap_chirho.eager_instance_route_chirho(
            &rows_chirho,
            None,
            "SizeChirho",
            "Char_[Char]",
            None,
        );
        let bootstrap_method_chirho = bootstrap_chirho.eager_instance_method_chirho(
            &environment_chirho,
            "SizeChirho",
            "sizeChirho",
            "Char_[Char]",
            None,
            &bootstrap_route_chirho,
        );
        if case_chirho == "noRowsChirho" {
            assert_eq!(bootstrap_route_chirho, Ok(None));
            assert_eq!(bootstrap_method_chirho, CoreIdChirho(20));
        } else {
            // Ambiguity is included unconditionally: no synthetic/None route can
            // inherit the source body merely because positive S3 ownership failed.
            assert!(bootstrap_route_chirho.is_err(), "{case_chirho}");
            assert_ne!(bootstrap_method_chirho, CoreIdChirho(10), "{case_chirho}");
            assert_ne!(bootstrap_method_chirho, CoreIdChirho(20), "{case_chirho}");
            assert!(
                bootstrap_chirho
                    .missing_method_ids_chirho
                    .contains(&bootstrap_method_chirho)
            );
            let mut refusal_chirho = &bootstrap_chirho
                .generated_bindings_chirho
                .iter()
                .find(|binding_chirho| {
                    binding_chirho.binder_chirho.id_chirho == bootstrap_method_chirho
                })
                .unwrap()
                .rhs_chirho;
            while let CoreExprChirho::LamChirho { body_chirho, .. } = refusal_chirho {
                refusal_chirho = body_chirho;
            }
            let CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } = refusal_chirho
            else {
                panic!("the None route must return a callable refusal: {case_chirho}");
            };
            assert_eq!(name_chirho, "error");
            let Some(CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(message_chirho))) =
                args_chirho.first()
            else {
                panic!("the None refusal must name the method and head: {case_chirho}");
            };
            assert!(message_chirho.contains("SizeChirho.sizeChirho"));
            assert!(message_chirho.contains("Char_[Char]"));
        }
        pass_chirho.generate_instance_dicts_chirho(&environment_chirho);
        let dictionary_chirho = pass_chirho
            .generated_bindings_chirho
            .iter()
            .find(|binding_chirho| {
                binding_chirho.binder_chirho.name_chirho == "$fSizeChirhoChar_[Char]"
            })
            .expect("the eager pass builds the complete MPTC key");
        let method_chirho = match &dictionary_chirho.rhs_chirho {
            CoreExprChirho::ConAppChirho { args_chirho, .. } => match args_chirho.first() {
                Some(CoreExprChirho::VarChirho(method_chirho)) => *method_chirho,
                other_chirho => panic!("a method slot must be a binding: {other_chirho:?}"),
            },
            other_chirho => panic!("a dictionary must be constructed: {other_chirho:?}"),
        };
        if case_chirho == "noRowsChirho" {
            assert_eq!(method_chirho, CoreIdChirho(20));
            continue;
        }
        if case_chirho == "joinedChirho" {
            assert_eq!(method_chirho, CoreIdChirho(10));
            let registered_chirho = pass_chirho
                .conditional_dicts_chirho
                .get(&("SizeChirho".to_string(), "Char_[Char]".to_string()))
                .unwrap();
            assert!(Arc::ptr_eq(
                registered_chirho.instance_chirho.as_ref().unwrap(),
                &instance_chirho
            ));
            continue;
        }
        assert_ne!(method_chirho, CoreIdChirho(10), "{case_chirho}");
        assert_ne!(method_chirho, CoreIdChirho(20), "{case_chirho}");
        assert!(
            pass_chirho
                .missing_method_ids_chirho
                .contains(&method_chirho)
        );
        let mut error_chirho = &pass_chirho
            .generated_bindings_chirho
            .iter()
            .find(|binding_chirho| binding_chirho.binder_chirho.id_chirho == method_chirho)
            .unwrap()
            .rhs_chirho;
        while let CoreExprChirho::LamChirho { body_chirho, .. } = error_chirho {
            error_chirho = body_chirho;
        }
        let CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } = error_chirho
        else {
            panic!("the refused slot remains a callable error: {case_chirho}");
        };
        assert_eq!(name_chirho, "error");
        let Some(CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(message_chirho))) =
            args_chirho.first()
        else {
            panic!("the refusal names the instance: {case_chirho}");
        };
        assert!(message_chirho.contains("SizeChirho"));
        assert!(message_chirho.contains("Char"));
        if case_chirho == "missingChirho" {
            assert!(message_chirho.contains("missing method"));
        } else if matches!(case_chirho, "ambiguousChirho" | "collisionChirho") {
            assert!(message_chirho.contains("cannot be told apart"));
        } else {
            assert!(message_chirho.contains("not in this module"));
        }
    }
}
