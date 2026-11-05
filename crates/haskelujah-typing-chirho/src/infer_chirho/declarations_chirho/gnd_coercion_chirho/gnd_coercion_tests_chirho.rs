// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GND method coercibility (row 694, #31784), on the shapes root measured with GHC 9.14.1:
//! R1 (an annotated nominal parameter) and R2 (a family-induced one) are GHC-18872, R3 (a
//! representational one) and R4 (a method that never mentions the parameter under it) prove,
//! and R5 (an associated family) is a named gap.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, StrictnessChirho, TyVarChirho};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

use super::*;
use crate::class_chirho::ClassDeclChirho as TypingClassChirho;
use crate::ty_chirho::{
    MultChirho, SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho as TypingVarChirho,
};

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn con_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::ConChirho(name_chirho(text_chirho))
}

fn var_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(name_chirho(text_chirho))
}

fn app_chirho(fun_chirho: TypeChirho, arg_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `data <name> <parameters> = <name> <fields>`, each parameter with its written role.
fn data_chirho(
    type_chirho: &str,
    parameters_chirho: &[(&str, Option<&str>)],
    fields_chirho: Vec<TypeChirho>,
) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: name_chirho(type_chirho),
        type_vars_chirho: parameters_chirho
            .iter()
            .map(|(parameter_chirho, role_chirho)| {
                let mut binder_chirho = TyVarChirho::plain_chirho(name_chirho(parameter_chirho));
                binder_chirho.role_annotation_chirho = role_chirho.map(name_chirho);
                binder_chirho
            })
            .collect(),
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: name_chirho(type_chirho),
            fields_chirho: fields_chirho
                .into_iter()
                .map(|field_chirho| (StrictnessChirho::LazyChirho, field_chirho))
                .collect(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn module_chirho(decls_chirho: Vec<DeclChirho>) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho("Main"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

const REPRESENTATIONAL_CHIRHO: ParameterRoleChirho = ParameterRoleChirho::REPRESENTATIONAL_CHIRHO;
const NOMINAL_CHIRHO: ParameterRoleChirho = ParameterRoleChirho::NOMINAL_CHIRHO;
const PHANTOM_CHIRHO: ParameterRoleChirho = ParameterRoleChirho::PHANTOM_CHIRHO;

#[test]
fn a_field_use_decides_each_parameter_role_chirho() {
    let roles_chirho = local_parameter_roles_chirho(&module_chirho(vec![
        // R3: the parameter is the field itself.
        data_chirho(
            "HoldChirho",
            &[("aChirho", None)],
            vec![var_chirho("aChirho")],
        ),
        // R1: an annotation can only make a role more restrictive.
        data_chirho(
            "PinnedChirho",
            &[("aChirho", Some("nominal"))],
            vec![var_chirho("aChirho")],
        ),
        // R2: a family's argument, like any constructor's roles cannot prove, is nominal.
        data_chirho(
            "LabelledChirho",
            &[("aChirho", None)],
            vec![app_chirho(con_chirho("LabelChirho"), var_chirho("aChirho"))],
        ),
        data_chirho(
            "UnusedChirho",
            &[("aChirho", None)],
            vec![con_chirho("Int")],
        ),
        // A type variable's application: the head representational, its argument nominal.
        data_chirho(
            "AppliedChirho",
            &[("fChirho", None), ("aChirho", None)],
            vec![app_chirho(var_chirho("fChirho"), var_chirho("aChirho"))],
        ),
    ]));
    assert_eq!(roles_chirho["HoldChirho"], [REPRESENTATIONAL_CHIRHO]);
    assert_eq!(roles_chirho["PinnedChirho"], [NOMINAL_CHIRHO]);
    assert_eq!(roles_chirho["LabelledChirho"], [NOMINAL_CHIRHO]);
    assert_eq!(roles_chirho["UnusedChirho"], [PHANTOM_CHIRHO]);
    assert_eq!(
        roles_chirho["AppliedChirho"],
        [REPRESENTATIONAL_CHIRHO, NOMINAL_CHIRHO]
    );
}

#[test]
fn a_local_type_passes_its_roles_on_whatever_the_declaration_order_chirho() {
    for pinned_role_chirho in [Some("nominal"), None] {
        let roles_chirho = local_parameter_roles_chirho(&module_chirho(vec![
            data_chirho(
                "OuterChirho",
                &[("bChirho", None)],
                vec![app_chirho(con_chirho("InnerChirho"), var_chirho("bChirho"))],
            ),
            data_chirho(
                "InnerChirho",
                &[("aChirho", pinned_role_chirho)],
                vec![var_chirho("aChirho")],
            ),
        ]));
        let expected_chirho = if pinned_role_chirho.is_some() {
            NOMINAL_CHIRHO
        } else {
            REPRESENTATIONAL_CHIRHO
        };
        assert_eq!(roles_chirho["OuterChirho"], [expected_chirho]);
    }
}

/// The class variable every test class uses.
const CLASS_VAR_CHIRHO: TypingVarChirho = TypingVarChirho(0);

/// `class <class> a where <method> :: <ty>`, as the class environment holds it.
fn class_env_chirho(
    class_chirho: &str,
    method_chirho: &str,
    ty_chirho: TyChirho,
) -> ClassEnvChirho {
    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    class_env_chirho.classes_chirho.insert(
        class_chirho.to_string(),
        TypingClassChirho {
            name_chirho: class_chirho.to_string(),
            supers_chirho: vec![],
            var_chirho: CLASS_VAR_CHIRHO,
            methods_chirho: HashMap::from([(
                method_chirho.to_string(),
                SchemeChirho {
                    vars_chirho: vec![CLASS_VAR_CHIRHO],
                    preds_chirho: vec![SchemePredChirho {
                        class_name_chirho: class_chirho.to_string(),
                        ty_chirho: TyChirho::VarChirho(CLASS_VAR_CHIRHO),
                        extra_tys_chirho: vec![],
                    }],
                    ty_chirho,
                },
            )]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        },
    );
    class_env_chirho
}

/// `a -> <head> <argument>`.
fn returns_applied_chirho(head_chirho: &str, argument_chirho: TyChirho) -> TyChirho {
    TyChirho::FunChirho(
        Box::new(TyChirho::VarChirho(CLASS_VAR_CHIRHO)),
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(head_chirho.to_string())),
            Box::new(argument_chirho),
        )),
        MultChirho::ManyChirho,
    )
}

fn request_chirho(class_chirho: &str) -> GndRequestChirho {
    GndRequestChirho {
        newtype_chirho: "AgeChirho".to_string(),
        class_chirho: name_chirho(class_chirho),
    }
}

fn prove_chirho(
    class_env_chirho: &ClassEnvChirho,
    hold_roles_chirho: Option<ParameterRoleChirho>,
) -> Result<GndCoercionProofChirho, DeclarationErrorChirho> {
    let roles_chirho: ParameterRolesChirho = hold_roles_chirho
        .map(|role_chirho| ("HoldChirho".to_string(), vec![role_chirho]))
        .into_iter()
        .collect();
    prove_request_chirho(
        &request_chirho("WrapChirho"),
        class_env_chirho,
        &roles_chirho,
        &HashMap::new(),
    )
}

#[test]
fn a_nominal_position_is_ghcs_refusal_and_a_representational_one_proves_chirho() {
    let wrap_chirho = class_env_chirho(
        "WrapChirho",
        "wrapChirho",
        returns_applied_chirho("HoldChirho", TyChirho::VarChirho(CLASS_VAR_CHIRHO)),
    );
    // R1 and R2: `wrapChirho :: a -> HoldChirho a` with HoldChirho's parameter nominal.
    let refused_chirho = prove_chirho(&wrap_chirho, Some(NOMINAL_CHIRHO))
        .expect_err("a nominal position never proves");
    assert_eq!(refused_chirho.ghc_code_chirho, "GHC-18872");
    assert!(refused_chirho.message_chirho.contains("`wrapChirho`"));
    // R3: the same method with HoldChirho's parameter representational.
    let proof_chirho =
        prove_chirho(&wrap_chirho, Some(REPRESENTATIONAL_CHIRHO)).expect("R3 proves");
    assert_eq!(proof_chirho.newtype_chirho(), "AgeChirho");
    assert_eq!(proof_chirho.class_chirho(), "WrapChirho");
    assert!(proof_chirho.proves_method_chirho("wrapChirho"));
    // A constructor whose roles the module cannot prove is a named gap, not GHC's refusal.
    let unknown_chirho = prove_chirho(&wrap_chirho, None).expect_err("unknown roles never prove");
    assert_eq!(unknown_chirho.ghc_code_chirho, UNSUPPORTED_SHAPE_CHIRHO);
}

#[test]
fn a_parameter_independent_result_is_never_over_rejected_chirho() {
    // R4: `tallyChirho :: a -> HoldChirho Int` proves whatever HoldChirho's role is.
    let tally_chirho = class_env_chirho(
        "WrapChirho",
        "tallyChirho",
        returns_applied_chirho("HoldChirho", TyChirho::ConChirho("Int".to_string())),
    );
    for role_chirho in [Some(NOMINAL_CHIRHO), None] {
        assert!(
            prove_chirho(&tally_chirho, role_chirho).is_ok(),
            "{role_chirho:?}"
        );
    }
}

#[test]
fn an_associated_family_or_an_unknown_class_is_a_named_gap_chirho() {
    let has_chirho = class_env_chirho(
        "HasChirho",
        "getChirho",
        TyChirho::VarChirho(CLASS_VAR_CHIRHO),
    );
    let associated_chirho = HashMap::from([("HasChirho".to_string(), "ItemChirho".to_string())]);
    let refused_chirho = prove_request_chirho(
        &request_chirho("HasChirho"),
        &has_chirho,
        &ParameterRolesChirho::new(),
        &associated_chirho,
    )
    .expect_err("R5 waits for the family's forwarding");
    assert_eq!(refused_chirho.ghc_code_chirho, UNSUPPORTED_SHAPE_CHIRHO);
    assert!(refused_chirho.message_chirho.contains("`ItemChirho`"));
    let unknown_chirho = prove_request_chirho(
        &request_chirho("MissingChirho"),
        &has_chirho,
        &ParameterRolesChirho::new(),
        &HashMap::new(),
    )
    .expect_err("an unknown class never proves");
    assert_eq!(unknown_chirho.ghc_code_chirho, UNSUPPORTED_SHAPE_CHIRHO);
}
