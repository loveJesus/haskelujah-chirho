// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Import transport keeps checked owners and soft metadata refusals while hard
//! semantic transport limits stop the real frontend before error deferral.

use crate::ImportQualificationChirho;
use haskelujah_ast_chirho::decl_chirho::{
    DeclChirho, GadtProjectionBudgetChirho, GadtProjectionLimitsChirho,
};
use haskelujah_ast_chirho::name_chirho::{CompilerClassChirho, PredicateMetadataRefusalChirho};
use haskelujah_span_chirho::{FileIdChirho, SpanChirho};
use haskelujah_typing_chirho::ty_chirho::SchemePredPartsChirho;
use haskelujah_typing_chirho::{SchemeChirho, SchemePredChirho, TyChirho};
use std::collections::{HashMap, HashSet};

fn list_chirho(depth_chirho: usize) -> TyChirho {
    (0..depth_chirho).fold(
        TyChirho::ConChirho("Int".into()),
        |inner_chirho, _index_chirho| TyChirho::ListChirho(Box::new(inner_chirho)),
    )
}

#[test]
fn soft_predicate_metadata_refusal_does_not_become_an_ordinary_import_error_chirho() {
    let deep_chirho = list_chirho(256);
    let mut predicate_chirho = SchemePredChirho::from_compiler_chirho(
        CompilerClassChirho::EqChirho,
        SchemePredPartsChirho {
            class_name_chirho: "Eq".into(),
            kind_tys_chirho: vec![deep_chirho.clone()],
            ty_chirho: deep_chirho.clone(),
            extra_tys_chirho: vec![TyChirho::ConChirho("Bool".into())],
        },
    );
    predicate_chirho
        .refuse_metadata_chirho(PredicateMetadataRefusalChirho::DepthChirho { limit_chirho: 128 });
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![],
        preds_chirho: vec![predicate_chirho],
        ty_chirho: deep_chirho,
    };
    let names_chirho = HashSet::new();
    let preferred_chirho = HashMap::new();
    let mut budget_chirho =
        GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
            work_chirho: 1 << 25,
            output_chirho: 1 << 24,
            depth_chirho: 1024,
        });
    let copied_chirho = ImportQualificationChirho {
        iface_chirho: None,
        type_names_chirho: &names_chirho,
        qualifier_chirho: "CallerChirho",
        safe_chirho: &names_chirho,
        preferred_chirho: &preferred_chirho,
        budget_chirho: &mut budget_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
    .scheme_chirho(&scheme_chirho, false)
    .expect("metadata exhaustion stays soft");
    assert_eq!(copied_chirho.ty_chirho, scheme_chirho.ty_chirho);
    assert_eq!(
        copied_chirho.preds_chirho[0].kind_tys_chirho,
        scheme_chirho.preds_chirho[0].kind_tys_chirho
    );
    assert_eq!(
        copied_chirho.preds_chirho[0].extra_tys_chirho,
        scheme_chirho.preds_chirho[0].extra_tys_chirho
    );
    assert_eq!(
        copied_chirho.preds_chirho[0].class_identity_chirho(),
        scheme_chirho.preds_chirho[0].class_identity_chirho()
    );
}

#[test]
fn hard_import_transport_failure_cannot_be_deferred_by_the_module_chirho() {
    let source_chirho = "module OriginChirho where\nfooChirho = undefined\n";
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let parser_chirho =
        crate::ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO);
    let module_chirho = crate::lower_module_chirho(
        &parser_chirho.parse_chirho(),
        FileIdChirho::SYNTHETIC_CHIRHO,
    );
    ifaces_chirho.push(
        haskelujah_naming_chirho::build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho)
            .expect("actual source producer"),
    );
    let types_chirho = HashMap::from([(
        "OriginChirho.fooChirho".to_owned(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: list_chirho(1030),
        },
    )]);
    let error_chirho = crate::run_frontend_chirho("{-# OPTIONS_GHC -fdefer-type-errors #-}\nmodule Main where\nimport qualified OriginChirho as OChirho\nmainChirho = OChirho.fooChirho\n", FileIdChirho::SYNTHETIC_CHIRHO, &ifaces_chirho, &types_chirho).err().expect("hard semantic transport limit is fatal");
    let rendered_chirho = error_chirho
        .diagnostics_chirho()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        rendered_chirho.contains("error[E0300]"),
        "{rendered_chirho}"
    );
    assert!(
        rendered_chirho.contains("imported type transport exceeded"),
        "{rendered_chirho}"
    );
}

#[test]
fn checked_ast_qualification_keeps_constructor_and_lexical_variable_owners_chirho() {
    let result_chirho = super::ordinary_elaboration_chirho::frontend_chirho("{-# LANGUAGE ScopedTypeVariables, KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\ndata NominalChirho aChirho = MkNominalChirho\nfChirho :: forall (aChirho :: Type). Eq aChirho => NominalChirho aChirho -> NominalChirho aChirho\nfChirho xChirho = xChirho\nmain = pure ()\n").expect("checked producer AST");
    let original_chirho = result_chirho
        .module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| {
            if let DeclChirho::TypeSigChirho { ty_chirho, .. } = declaration_chirho {
                Some(ty_chirho)
            } else {
                None
            }
        })
        .expect("actual checked signature");
    let names_chirho = HashSet::from(["NominalChirho".to_owned(), "Type".to_owned()]);
    let safe_chirho = HashSet::new();
    let preferred_chirho = HashMap::new();
    let mut budget_chirho = GadtProjectionBudgetChirho::new_chirho();
    let copied_chirho = ImportQualificationChirho {
        iface_chirho: None,
        type_names_chirho: &names_chirho,
        qualifier_chirho: "UnrelatedChirho",
        safe_chirho: &safe_chirho,
        preferred_chirho: &preferred_chirho,
        budget_chirho: &mut budget_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
    .ast_chirho(original_chirho)
    .expect("checked transport");
    // Debug includes owner metadata, unlike Name's legacy syntax equality.
    assert_eq!(format!("{copied_chirho:?}"), format!("{original_chirho:?}"));
}
