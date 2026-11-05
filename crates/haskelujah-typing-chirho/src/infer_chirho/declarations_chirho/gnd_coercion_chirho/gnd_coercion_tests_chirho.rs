// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GND method coercibility (row 694, #31784, #31810, #31811), on the shapes root measured
//! with GHC 9.14.1: R1 (an annotated nominal parameter), R2 (a family-induced one) and
//! `ConstructorContextRoleChirho` (a constructor's equality context) are GHC-18872, R3 (a
//! representational one) and R4 (a method that never mentions the parameter under it) prove,
//! and R5 (an associated family) is a named gap. A role no local declaration proves is
//! Unknown: a named gap, never GHC's code.

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, ConPrefixChirho, StrictnessChirho, TyVarChirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
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

/// `qualifier.text` as a source name.
fn qualified_name_chirho(qualifier_chirho: &str, text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::qualified_chirho(
        qualifier_chirho,
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

/// `type family <name> a`.
fn family_chirho(family_name_chirho: &str) -> DeclChirho {
    DeclChirho::TypeFamilyDeclChirho {
        name_chirho: name_chirho(family_name_chirho),
        type_vars_chirho: vec![TyVarChirho::plain_chirho(name_chirho("aChirho"))],
        result_kind_chirho: None,
        equations_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `data <name> <parameters> = <name> <fields>`, each parameter with its written role.
fn data_chirho(
    type_chirho: &str,
    parameters_chirho: &[(&str, Option<&str>)],
    fields_chirho: Vec<TypeChirho>,
) -> DeclChirho {
    data_with_prefix_chirho(
        type_chirho,
        parameters_chirho,
        ConPrefixChirho::default(),
        fields_chirho,
    )
}

/// `data <name> <parameters> = <prefix> <name> <fields>`.
fn data_with_prefix_chirho(
    type_chirho: &str,
    parameters_chirho: &[(&str, Option<&str>)],
    prefix_chirho: ConPrefixChirho,
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
            prefix_chirho,
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
const UNKNOWN_CHIRHO: ParameterRoleChirho = ParameterRoleChirho::UNKNOWN_CHIRHO;

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
        // R2: a local type family's argument is nominal.
        family_chirho("LabelChirho"),
        data_chirho(
            "LabelledChirho",
            &[("aChirho", None)],
            vec![app_chirho(con_chirho("LabelChirho"), var_chirho("aChirho"))],
        ),
        // A constructor the module does not declare proves no role: Unknown, not nominal.
        data_chirho(
            "ImportedChirho",
            &[("aChirho", None)],
            vec![app_chirho(con_chirho("Maybe"), var_chirho("aChirho"))],
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
    assert_eq!(roles_chirho["ImportedChirho"], [UNKNOWN_CHIRHO]);
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

#[test]
fn only_the_local_declaration_has_authority_over_its_roles_chirho() {
    // #31810: `HoldChirho` is phantom here; a qualified `M.HoldChirho` is another module's.
    let qualified_chirho = |qualifier_chirho: &str| {
        app_chirho(
            TypeChirho::ConChirho(qualified_name_chirho(qualifier_chirho, "HoldChirho")),
            var_chirho("aChirho"),
        )
    };
    let roles_chirho = local_parameter_roles_chirho(&module_chirho(vec![
        data_chirho("HoldChirho", &[("aChirho", None)], vec![con_chirho("Int")]),
        data_chirho(
            "ElsewhereChirho",
            &[("aChirho", None)],
            vec![qualified_chirho("M")],
        ),
        data_chirho(
            "OwnChirho",
            &[("aChirho", None)],
            vec![qualified_chirho("Main")],
        ),
        data_chirho(
            "BareChirho",
            &[("aChirho", None)],
            vec![app_chirho(con_chirho("HoldChirho"), var_chirho("aChirho"))],
        ),
    ]));
    assert_eq!(roles_chirho["HoldChirho"], [PHANTOM_CHIRHO]);
    assert_eq!(roles_chirho["ElsewhereChirho"], [UNKNOWN_CHIRHO]);
    assert_eq!(roles_chirho["OwnChirho"], [PHANTOM_CHIRHO]);
    assert_eq!(roles_chirho["BareChirho"], [PHANTOM_CHIRHO]);
}

#[test]
fn a_constructor_prefix_decides_what_its_variables_are_chirho() {
    // #31811, root's measured `ConstructorContextRoleChirho`: `forall . (aChirho ~ Int) =>`.
    let equality_chirho = ConPrefixChirho {
        existential_vars_chirho: vec![],
        context_chirho: vec![ConstraintChirho::ClassChirho {
            class_chirho: name_chirho("~"),
            args_chirho: vec![var_chirho("aChirho"), con_chirho("Int")],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        unread_chirho: false,
    };
    // `forall aChirho. HiddenChirho aChirho`: the field's variable is the binder's.
    let shadowing_chirho = ConPrefixChirho {
        existential_vars_chirho: vec![name_chirho("aChirho")],
        ..ConPrefixChirho::default()
    };
    let roles_chirho = local_parameter_roles_chirho(&module_chirho(vec![
        data_with_prefix_chirho(
            "HoldChirho",
            &[("aChirho", None)],
            equality_chirho,
            vec![con_chirho("Int")],
        ),
        data_with_prefix_chirho(
            "HiddenChirho",
            &[("aChirho", None)],
            shadowing_chirho,
            vec![var_chirho("aChirho")],
        ),
        data_with_prefix_chirho(
            "UnreadChirho",
            &[("aChirho", None)],
            ConPrefixChirho::unread_quantification_chirho(),
            vec![con_chirho("Int")],
        ),
    ]));
    assert_eq!(roles_chirho["HoldChirho"], [NOMINAL_CHIRHO]);
    assert_eq!(roles_chirho["HiddenChirho"], [PHANTOM_CHIRHO]);
    assert_eq!(roles_chirho["UnreadChirho"], [UNKNOWN_CHIRHO]);
}

#[test]
fn below_a_nominal_argument_every_variable_is_nominal_chirho() {
    let roles_chirho = local_parameter_roles_chirho(&module_chirho(vec![
        family_chirho("LabelChirho"),
        data_chirho(
            "PhantomChirho",
            &[("aChirho", None)],
            vec![con_chirho("Int")],
        ),
        // GHC marks every variable below a family's argument nominal, phantom positions too.
        data_chirho(
            "UnderFamilyChirho",
            &[("bChirho", None)],
            vec![app_chirho(
                con_chirho("LabelChirho"),
                app_chirho(con_chirho("PhantomChirho"), var_chirho("bChirho")),
            )],
        ),
        // Below an Unknown argument the family's nominal position might be phantom.
        data_chirho(
            "UnderUnknownChirho",
            &[("bChirho", None)],
            vec![app_chirho(
                con_chirho("Maybe"),
                app_chirho(con_chirho("LabelChirho"), var_chirho("bChirho")),
            )],
        ),
    ]));
    assert_eq!(roles_chirho["UnderFamilyChirho"], [NOMINAL_CHIRHO]);
    assert_eq!(roles_chirho["UnderUnknownChirho"], [UNKNOWN_CHIRHO]);
}

#[test]
fn a_gadt_parameter_is_unknown_unless_annotated_chirho() {
    let gadt_chirho = |annotation_chirho: Option<&str>| {
        let mut binder_chirho = TyVarChirho::plain_chirho(name_chirho("aChirho"));
        binder_chirho.role_annotation_chirho = annotation_chirho.map(name_chirho);
        DeclChirho::DataDeclChirho {
            name_chirho: name_chirho(if annotation_chirho.is_some() {
                "PinnedGadtChirho"
            } else {
                "GadtChirho"
            }),
            type_vars_chirho: vec![binder_chirho],
            constructors_chirho: vec![ConDeclChirho::GadtChirho {
                name_chirho: name_chirho("MkGadtChirho"),
                ty_chirho: con_chirho("Int"),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    };
    let roles_chirho = local_parameter_roles_chirho(&module_chirho(vec![
        gadt_chirho(None),
        gadt_chirho(Some("nominal")),
    ]));
    assert_eq!(roles_chirho["GadtChirho"], [UNKNOWN_CHIRHO]);
    assert_eq!(roles_chirho["PinnedGadtChirho"], [NOMINAL_CHIRHO]);
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
    prove_with_chirho(class_env_chirho, &roles_chirho, &HashSet::new())
}

/// The proof for `WrapChirho` under these roles and local type families.
fn prove_with_chirho(
    class_env_chirho: &ClassEnvChirho,
    roles_chirho: &ParameterRolesChirho,
    families_chirho: &HashSet<String>,
) -> Result<GndCoercionProofChirho, DeclarationErrorChirho> {
    prove_request_chirho(
        &request_chirho("WrapChirho"),
        class_env_chirho,
        &RoleAuthorityChirho {
            roles_chirho,
            families_chirho,
        },
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
    // A constructor whose roles the module cannot prove is a named gap, not GHC's refusal,
    // and so is a local parameter whose role is Unknown.
    let unknown_chirho = prove_chirho(&wrap_chirho, None).expect_err("unknown roles never prove");
    assert_eq!(unknown_chirho.ghc_code_chirho, UNSUPPORTED_SHAPE_CHIRHO);
    let unknown_role_chirho =
        prove_chirho(&wrap_chirho, Some(UNKNOWN_CHIRHO)).expect_err("an Unknown role never proves");
    assert_eq!(
        unknown_role_chirho.ghc_code_chirho,
        UNSUPPORTED_SHAPE_CHIRHO
    );
}

#[test]
fn below_a_blocked_position_every_occurrence_is_blocked_chirho() {
    // `wrapChirho :: a -> HoldChirho (PhantomChirho a)`: the phantom position below a nominal
    // one is nominal too, so GHC refuses it; below a representational one it proves.
    let wrap_chirho = class_env_chirho(
        "WrapChirho",
        "wrapChirho",
        returns_applied_chirho(
            "HoldChirho",
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("PhantomChirho".to_string())),
                Box::new(TyChirho::VarChirho(CLASS_VAR_CHIRHO)),
            ),
        ),
    );
    let roles_chirho = |hold_chirho: ParameterRoleChirho| -> ParameterRolesChirho {
        HashMap::from([
            ("HoldChirho".to_string(), vec![hold_chirho]),
            ("PhantomChirho".to_string(), vec![PHANTOM_CHIRHO]),
        ])
    };
    let refused_chirho =
        prove_with_chirho(&wrap_chirho, &roles_chirho(NOMINAL_CHIRHO), &HashSet::new())
            .expect_err("below a nominal position nothing coerces");
    assert_eq!(refused_chirho.ghc_code_chirho, "GHC-18872");
    assert!(
        prove_with_chirho(
            &wrap_chirho,
            &roles_chirho(REPRESENTATIONAL_CHIRHO),
            &HashSet::new()
        )
        .is_ok()
    );
}

#[test]
fn a_local_family_argument_is_ghcs_refusal_chirho() {
    // `wrapChirho :: a -> LabelChirho a`: nominal when the module declares the family.
    let wrap_chirho = class_env_chirho(
        "WrapChirho",
        "wrapChirho",
        returns_applied_chirho("LabelChirho", TyChirho::VarChirho(CLASS_VAR_CHIRHO)),
    );
    let families_chirho = HashSet::from(["LabelChirho".to_string()]);
    let refused_chirho =
        prove_with_chirho(&wrap_chirho, &ParameterRolesChirho::new(), &families_chirho)
            .expect_err("a family's argument never coerces");
    assert_eq!(refused_chirho.ghc_code_chirho, "GHC-18872");
    let unknown_chirho =
        prove_with_chirho(&wrap_chirho, &ParameterRolesChirho::new(), &HashSet::new())
            .expect_err("an undeclared constructor proves nothing");
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
    let no_roles_chirho = ParameterRolesChirho::new();
    let no_families_chirho = HashSet::new();
    let authority_chirho = RoleAuthorityChirho {
        roles_chirho: &no_roles_chirho,
        families_chirho: &no_families_chirho,
    };
    let refused_chirho = prove_request_chirho(
        &request_chirho("HasChirho"),
        &has_chirho,
        &authority_chirho,
        &associated_chirho,
    )
    .expect_err("R5 waits for the family's forwarding");
    assert_eq!(refused_chirho.ghc_code_chirho, UNSUPPORTED_SHAPE_CHIRHO);
    assert!(refused_chirho.message_chirho.contains("`ItemChirho`"));
    let unknown_chirho = prove_request_chirho(
        &request_chirho("MissingChirho"),
        &has_chirho,
        &authority_chirho,
        &HashMap::new(),
    )
    .expect_err("an unknown class never proves");
    assert_eq!(unknown_chirho.ghc_code_chirho, UNSUPPORTED_SHAPE_CHIRHO);
}
