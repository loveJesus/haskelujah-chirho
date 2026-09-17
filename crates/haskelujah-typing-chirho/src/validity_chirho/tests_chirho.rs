// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
use super::*;
use haskelujah_ast_chirho::decl_chirho::{
    ClassMethodChirho, FieldDeclChirho, StrictnessChirho, TyVarChirho,
};
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ImportDeclChirho, ImportSpecChirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

fn mk_name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn mk_module_chirho(decls_chirho: Vec<DeclChirho>, extensions_chirho: Vec<String>) -> ModuleChirho {
    ModuleChirho {
        name_chirho: mk_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho,
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn mk_var_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(mk_name_chirho(text_chirho))
}

fn mk_con_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::ConChirho(mk_name_chirho(text_chirho))
}

/// `forall a. a -> a`
fn mk_poly_id_chirho() -> TypeChirho {
    TypeChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
        body_chirho: Box::new(TypeChirho::FunChirho {
            arg_chirho: Box::new(mk_var_chirho("a")),
            mult_chirho: None,
            result_chirho: Box::new(mk_var_chirho("a")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn mk_sig_chirho(ty_chirho: TypeChirho) -> DeclChirho {
    DeclChirho::TypeSigChirho {
        name_chirho: mk_name_chirho("f"),
        ty_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn top_level_forall_is_always_allowed_chirho() {
    // `f :: forall a. a -> a` is plain Haskell2010 with ExplicitForAll.
    let module_chirho = mk_module_chirho(vec![mk_sig_chirho(mk_poly_id_chirho())], vec![]);
    assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
}

#[test]
fn rank_two_argument_needs_rank_n_types_chirho() {
    // `f :: (forall a. a -> a) -> Int`
    let ty_chirho = TypeChirho::FunChirho {
        arg_chirho: Box::new(mk_poly_id_chirho()),
        mult_chirho: None,
        result_chirho: Box::new(mk_con_chirho("Int")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let without_chirho = mk_module_chirho(
        vec![mk_sig_chirho(ty_chirho.clone())],
        vec!["Haskell2010".to_string()],
    );
    let errors_chirho = check_module_type_validity_chirho(&without_chirho);
    assert_eq!(errors_chirho.len(), 1);
    assert_eq!(
        errors_chirho[0].suggested_extension_chirho,
        Some("RankNTypes")
    );

    let with_chirho = mk_module_chirho(
        vec![mk_sig_chirho(ty_chirho)],
        vec!["RankNTypes".to_string()],
    );
    assert!(check_module_type_validity_chirho(&with_chirho).is_empty());
}

#[test]
fn polytype_as_constructor_argument_needs_impredicative_types_chirho() {
    // `f :: Maybe (forall a. a -> a)` — RankNTypes is NOT enough here.
    let ty_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(mk_con_chirho("Maybe")),
        arg_chirho: Box::new(mk_poly_id_chirho()),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let rank_n_only_chirho = mk_module_chirho(
        vec![mk_sig_chirho(ty_chirho.clone())],
        vec!["RankNTypes".to_string()],
    );
    let errors_chirho = check_module_type_validity_chirho(&rank_n_only_chirho);
    assert_eq!(errors_chirho.len(), 1);
    assert_eq!(
        errors_chirho[0].suggested_extension_chirho,
        Some("ImpredicativeTypes")
    );

    let impredicative_chirho = mk_module_chirho(
        vec![mk_sig_chirho(ty_chirho)],
        vec!["ImpredicativeTypes".to_string()],
    );
    assert!(check_module_type_validity_chirho(&impredicative_chirho).is_empty());
}

#[test]
fn qualified_type_synonym_rhs_needs_rank_n_types_chirho() {
    // `type Constrd a = Num a => a`
    let rhs_chirho = TypeChirho::QualChirho {
        context_chirho: vec![ConstraintChirho::ClassChirho {
            class_chirho: mk_name_chirho("Num"),
            args_chirho: vec![mk_var_chirho("a")],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        body_chirho: Box::new(mk_var_chirho("a")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let decl_chirho = DeclChirho::TypeAliasDeclChirho {
        name_chirho: mk_name_chirho("Constrd"),
        type_vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
        rhs_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho =
        mk_module_chirho(vec![decl_chirho.clone()], vec!["Haskell2010".to_string()]);
    let errors_chirho = check_module_type_validity_chirho(&module_chirho);
    assert_eq!(errors_chirho.len(), 1);
    assert!(
        errors_chirho[0].message_chirho.contains("Num a"),
        "diagnostic should name the constraint, got {}",
        errors_chirho[0].message_chirho
    );

    let with_chirho = mk_module_chirho(vec![decl_chirho], vec!["RankNTypes".to_string()]);
    assert!(check_module_type_validity_chirho(&with_chirho).is_empty());
}

#[test]
fn impredicative_types_implies_rank_n_types_chirho() {
    // GHC: ImpredicativeTypes implies RankNTypes, so a rank-2 argument is
    // licensed by ImpredicativeTypes alone.
    let ty_chirho = TypeChirho::FunChirho {
        arg_chirho: Box::new(mk_poly_id_chirho()),
        mult_chirho: None,
        result_chirho: Box::new(mk_con_chirho("Int")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = mk_module_chirho(
        vec![mk_sig_chirho(ty_chirho)],
        vec!["ImpredicativeTypes".to_string()],
    );
    assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
}

// ---- walk sites added for the validity-walker lane ----

/// `Con (forall a. a -> a)` as a single positional field.
fn mk_poly_field_con_chirho() -> ConDeclChirho {
    ConDeclChirho::OrdinaryChirho {
        name_chirho: mk_name_chirho("Con"),
        fields_chirho: vec![(StrictnessChirho::LazyChirho, mk_poly_id_chirho())],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn class_method_rank_two_argument_needs_rank_n_types_chirho() {
    // `class C a where m :: (forall b. b -> b) -> a`
    let method_ty_chirho = TypeChirho::FunChirho {
        arg_chirho: Box::new(mk_poly_id_chirho()),
        mult_chirho: None,
        result_chirho: Box::new(mk_var_chirho("a")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let decl_chirho = DeclChirho::ClassDeclChirho {
        context_written_chirho: false,
        minimal_chirho: None,
        context_chirho: vec![],
        name_chirho: mk_name_chirho("C"),
        type_vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
        methods_chirho: vec![ClassMethodChirho {
            name_chirho: mk_name_chirho("m"),
            ty_chirho: method_ty_chirho,
            default_chirho: None,
            default_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = mk_module_chirho(vec![decl_chirho], vec!["Haskell2010".to_string()]);
    let errors_chirho = check_module_type_validity_chirho(&module_chirho);
    assert_eq!(errors_chirho.len(), 1);
    assert_eq!(
        errors_chirho[0].suggested_extension_chirho,
        Some("RankNTypes")
    );
}

#[test]
fn positional_constructor_field_forall_needs_rank_n_types_chirho() {
    // `data T = Con (forall a. a -> a)` — GHC tcfail184.
    let decl_chirho = DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("T"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![mk_poly_field_con_chirho()],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = mk_module_chirho(vec![decl_chirho], vec!["Haskell2010".to_string()]);
    assert_eq!(check_module_type_validity_chirho(&module_chirho).len(), 1);
}

#[test]
fn record_constructor_fields_are_never_judged_chirho() {
    // The parser lowers record fields through a flat path that splits on
    // `=>` before lifting a leading `forall`, manufacturing a constraint the
    // source never wrote. Judging them regresses six files GHC accepts, so
    // this site stays skipped until the parser is fixed.
    let decl_chirho = DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("T"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![ConDeclChirho::RecordChirho {
            name_chirho: mk_name_chirho("Con"),
            fields_chirho: vec![FieldDeclChirho {
                names_chirho: vec![mk_name_chirho("fld")],
                ty_chirho: mk_poly_id_chirho(),
                strictness_chirho: StrictnessChirho::LazyChirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = mk_module_chirho(vec![decl_chirho], vec!["Haskell2010".to_string()]);
    assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
}

#[test]
fn datatype_contexts_skips_constructor_fields_chirho() {
    let decl_chirho = DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("T"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![mk_poly_field_con_chirho()],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = mk_module_chirho(
        vec![decl_chirho],
        vec!["Haskell2010".to_string(), "DatatypeContexts".to_string()],
    );
    assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
}

#[test]
fn quantified_constraint_needs_its_extension_chirho() {
    // `f :: (forall a. Eq a) => Int` — GHC T9196.
    let quantified_chirho = ConstraintChirho::QuantifiedChirho {
        vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
        context_chirho: vec![],
        body_chirho: Box::new(ConstraintChirho::ClassChirho {
            class_chirho: mk_name_chirho("Eq"),
            args_chirho: vec![mk_var_chirho("a")],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let ty_chirho = TypeChirho::QualChirho {
        context_chirho: vec![quantified_chirho],
        body_chirho: Box::new(mk_con_chirho("Int")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let without_chirho = mk_module_chirho(vec![mk_sig_chirho(ty_chirho.clone())], vec![]);
    let errors_chirho = check_module_type_validity_chirho(&without_chirho);
    assert_eq!(errors_chirho.len(), 1);
    assert_eq!(
        errors_chirho[0].suggested_extension_chirho,
        Some("QuantifiedConstraints")
    );

    let with_chirho = mk_module_chirho(
        vec![mk_sig_chirho(ty_chirho)],
        vec!["QuantifiedConstraints".to_string()],
    );
    assert!(check_module_type_validity_chirho(&with_chirho).is_empty());
}

#[test]
fn unresolved_constraint_marker_is_never_judged_chirho() {
    // The parser writes class `?` when it could not lower a constraint
    // shape; its "argument" is the whole unlowered constraint.
    let ty_chirho = TypeChirho::QualChirho {
        context_chirho: vec![ConstraintChirho::ClassChirho {
            class_chirho: mk_name_chirho(UNRESOLVED_CONSTRAINT_MARKER_CHIRHO),
            args_chirho: vec![mk_poly_id_chirho()],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        body_chirho: Box::new(mk_con_chirho("Int")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = mk_module_chirho(
        vec![mk_sig_chirho(ty_chirho)],
        vec!["Haskell2010".to_string()],
    );
    assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
}

// ---- unlifted newtypes ----

fn mk_prim_newtype_module_chirho(
    extensions_chirho: Vec<String>,
    imports_chirho: Vec<ImportDeclChirho>,
    extra_decls_chirho: Vec<DeclChirho>,
) -> ModuleChirho {
    let mut decls_chirho = vec![DeclChirho::NewtypeDeclChirho {
        name_chirho: mk_name_chirho("N"),
        type_vars_chirho: vec![],
        constructor_chirho: ConDeclChirho::OrdinaryChirho {
            name_chirho: mk_name_chirho("MkN"),
            fields_chirho: vec![(StrictnessChirho::LazyChirho, mk_con_chirho("Int#"))],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }];
    decls_chirho.extend(extra_decls_chirho);
    let mut module_chirho = mk_module_chirho(decls_chirho, extensions_chirho);
    module_chirho.imports_chirho = imports_chirho;
    module_chirho
}

fn mk_ghc_exts_import_chirho(hiding_chirho: bool) -> ImportDeclChirho {
    ImportDeclChirho {
        module_chirho: mk_name_chirho("GHC.Exts"),
        source_chirho: false,
        qualified_chirho: false,
        alias_chirho: None,
        spec_chirho: Some(ImportSpecChirho {
            hiding_chirho,
            items_chirho: vec![ImportItemChirho::TyConChirho {
                name_chirho: mk_name_chirho("Int#"),
                members_chirho: ExportMembersChirho::NoneChirho,
            }],
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn unlifted_newtype_field_needs_the_extension_chirho() {
    let module_chirho = mk_prim_newtype_module_chirho(
        vec!["MagicHash".to_string()],
        vec![mk_ghc_exts_import_chirho(false)],
        vec![],
    );
    let errors_chirho = check_module_type_validity_chirho(&module_chirho);
    assert_eq!(errors_chirho.len(), 1);
    assert_eq!(
        errors_chirho[0].suggested_extension_chirho,
        Some("UnliftedNewtypes")
    );
}

#[test]
fn unlifted_newtype_is_fine_with_the_extension_chirho() {
    let module_chirho = mk_prim_newtype_module_chirho(
        vec!["MagicHash".to_string(), "UnliftedNewtypes".to_string()],
        vec![mk_ghc_exts_import_chirho(false)],
        vec![],
    );
    assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
}

#[test]
fn unlifted_newtype_needs_an_explicit_prim_import_chirho() {
    // Without `import GHC.Exts (Int#)` the name cannot mean GHC's
    // primitive, so a user type of that spelling must not be judged.
    let module_chirho =
        mk_prim_newtype_module_chirho(vec!["MagicHash".to_string()], vec![], vec![]);
    assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
}

#[test]
fn hiding_import_does_not_bring_the_primitive_into_scope_chirho() {
    let module_chirho = mk_prim_newtype_module_chirho(
        vec!["MagicHash".to_string()],
        vec![mk_ghc_exts_import_chirho(true)],
        vec![],
    );
    assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
}

#[test]
fn locally_declared_prim_name_shadows_the_primitive_chirho() {
    // `MagicHash` permits a user-written `data Int#`.
    let local_chirho = DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("Int#"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = mk_prim_newtype_module_chirho(
        vec!["MagicHash".to_string()],
        vec![mk_ghc_exts_import_chirho(false)],
        vec![local_chirho],
    );
    assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
}

#[test]
fn extension_override_order_is_honoured_chirho() {
    // GHC: the last mention wins, so `NoMagicHash` cancels `MagicHash`.
    let module_chirho = mk_prim_newtype_module_chirho(
        vec!["MagicHash".to_string(), "NoMagicHash".to_string()],
        vec![mk_ghc_exts_import_chirho(false)],
        vec![],
    );
    assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
}
