// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! An associated default is scoped by its own equation (row 686, GHC-76037).
//! workflow: compiler-pipeline-chirho/type-scope-resolution-chirho

use super::*;
use haskelujah_ast_chirho::decl_chirho::{
    AssocTypeFamilyChirho, TyVarChirho, TyVarVisibilityChirho,
};
use haskelujah_ast_chirho::name_chirho::RawNameChirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn binder_chirho(text_chirho: &str) -> TyVarChirho {
    TyVarChirho::plain_chirho(name_chirho(text_chirho))
}

fn kind_binder_chirho(text_chirho: &str) -> TyVarChirho {
    let mut binder_chirho = binder_chirho(text_chirho);
    binder_chirho.visibility_chirho = TyVarVisibilityChirho::InvisibleChirho;
    binder_chirho
}

fn variable_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(name_chirho(text_chirho))
}

/// A class over `class_params_chirho` whose one associated family has a
/// written default with the given right side and recorded scope.
fn class_with_default_chirho(
    class_params_chirho: &[&str],
    default_rhs_chirho: TypeChirho,
    default_binders_chirho: Option<Vec<TyVarChirho>>,
) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho("DefaultScopeChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: name_chirho("ClassChirho"),
            type_vars_chirho: class_params_chirho
                .iter()
                .map(|param_chirho| binder_chirho(param_chirho))
                .collect(),
            methods_chirho: vec![],
            associated_tfs_chirho: vec![AssocTypeFamilyChirho {
                name_chirho: name_chirho("FamilyChirho"),
                type_vars_chirho: vec![name_chirho("aChirho")],
                data_family_chirho: false,
                default_rhs_chirho: Some(default_rhs_chirho),
                default_params_chirho: vec![],
                default_binders_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

fn check_chirho(module_chirho: &ModuleChirho) -> DiagnosticBundleChirho {
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();
    check_module_type_scope_chirho(
        module_chirho,
        &NameEnvChirho::new_chirho(),
        &[],
        &mut diagnostics_chirho,
    );
    diagnostics_chirho
}

#[test]
fn a_written_default_does_not_see_other_class_parameters_chirho() {
    // T5481 and D01: GHC 9.14.1 reports `b` not in scope in `type X a = b`.
    let diagnostics_chirho = check_chirho(&class_with_default_chirho(
        &["aChirho", "bChirho"],
        variable_chirho("bChirho"),
        Some(vec![binder_chirho("aChirho")]),
    ));
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("type variable not in scope: `bChirho`"));
}

#[test]
fn a_written_default_sees_the_kind_variables_its_binders_bind_chirho() {
    // T18585: `type Dom (f :: i -> j) = SelfDom f (DefaultCat i)` uses `i`.
    let diagnostics_chirho = check_chirho(&class_with_default_chirho(
        &["fChirho"],
        TypeChirho::AppChirho {
            fun_chirho: Box::new(variable_chirho("fChirho")),
            arg_chirho: Box::new(variable_chirho("iChirho")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        Some(vec![
            binder_chirho("fChirho"),
            kind_binder_chirho("iChirho"),
        ]),
    ));
    assert!(diagnostics_chirho.is_empty_chirho());
}

#[test]
fn a_default_without_recorded_scope_keeps_the_class_binders_chirho() {
    // A directly built AST records no default scope; it keeps the legacy walk.
    let diagnostics_chirho = check_chirho(&class_with_default_chirho(
        &["aChirho", "bChirho"],
        variable_chirho("bChirho"),
        None,
    ));
    assert!(diagnostics_chirho.is_empty_chirho());
}
