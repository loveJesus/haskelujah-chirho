// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The class a GND request's spelling names (#31891): the module's own class, a proven builtin
//! whose entry is the compiler's seed, or a named gap. Claude's H1 and H2 write `F.WrapChirho`
//! beside a local `WrapChirho`; their GHC verdicts are root's to measure, and both are gaps here.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, TyVarChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{
    BuiltinClassChirho, BuiltinClassScopeChirho, NameChirho, RawNameChirho,
};
use haskelujah_span_chirho::SpanChirho;

use super::*;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn qualified_chirho(qualifier_chirho: &str, text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::qualified_chirho(
        qualifier_chirho,
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

/// `class <class> aChirho`.
fn class_decl_chirho(class_name_chirho: &str) -> DeclChirho {
    DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: name_chirho(class_name_chirho),
        type_vars_chirho: vec![TyVarChirho::plain_chirho(name_chirho("aChirho"))],
        methods_chirho: vec![],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
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

/// K01 proves `Show`, `P.Show` (under `import qualified Prelude as P`) and `Ix`.
fn scope_chirho() -> BuiltinClassScopeChirho {
    let show_chirho = BuiltinClassChirho {
        module_chirho: "GHC.Show",
        name_chirho: "Show",
        parameter_arities_chirho: &[0],
    };
    let ix_chirho = BuiltinClassChirho {
        module_chirho: "GHC.Ix",
        name_chirho: "Ix",
        parameter_arities_chirho: &[0],
    };
    BuiltinClassScopeChirho::new_chirho(HashMap::from([
        ("Show".to_string(), show_chirho),
        ("P.Show".to_string(), show_chirho),
        ("Ix".to_string(), ix_chirho),
    ]))
}

fn seeded_chirho() -> ClassEnvChirho {
    let mut seeded_chirho = ClassEnvChirho::new_chirho();
    seeded_chirho.seed_standard_chirho();
    seeded_chirho
}

/// The gap for `class_chirho` in a module of these declarations, judged against this class
/// environment.
fn gap_chirho(
    decls_chirho: Vec<DeclChirho>,
    class_env_chirho: &ClassEnvChirho,
    class_chirho: &NameChirho,
) -> Option<String> {
    let module_chirho = module_chirho(decls_chirho);
    let scope_chirho = scope_chirho();
    let identity_chirho = StockIdentityChirho::of_module_chirho(&module_chirho, &scope_chirho);
    class_authority_gap_chirho(
        class_chirho,
        &identity_chirho,
        class_env_chirho,
        &seeded_chirho(),
    )
}

#[test]
fn a_local_spelling_names_the_modules_own_class_and_a_foreign_one_never_borrows_it_chirho() {
    let local_chirho = || vec![class_decl_chirho("WrapChirho")];
    let env_chirho = seeded_chirho();
    assert_eq!(
        gap_chirho(local_chirho(), &env_chirho, &name_chirho("WrapChirho")),
        None
    );
    assert_eq!(
        gap_chirho(
            local_chirho(),
            &env_chirho,
            &qualified_chirho("Main", "WrapChirho")
        ),
        None
    );
    // H1 and H2: the foreign class is not the local homonym, whatever its GHC verdict.
    let foreign_chirho = gap_chirho(
        local_chirho(),
        &env_chirho,
        &qualified_chirho("F", "WrapChirho"),
    )
    .expect("a foreign spelling never borrows the local class");
    assert!(
        foreign_chirho.contains("`F.WrapChirho`"),
        "{foreign_chirho}"
    );
}

#[test]
fn a_proven_builtin_names_only_its_seeded_entry_chirho() {
    let env_chirho = seeded_chirho();
    assert_eq!(gap_chirho(vec![], &env_chirho, &name_chirho("Show")), None);
    assert_eq!(
        gap_chirho(vec![], &env_chirho, &qualified_chirho("P", "Show")),
        None
    );
    // A local `Show` holds that name in the checker: `P.Show` may not read it, and the
    // unqualified spelling names the local class itself.
    assert!(
        gap_chirho(
            vec![class_decl_chirho("Show")],
            &env_chirho,
            &qualified_chirho("P", "Show")
        )
        .is_some()
    );
    assert_eq!(
        gap_chirho(
            vec![class_decl_chirho("Show")],
            &env_chirho,
            &name_chirho("Show")
        ),
        None
    );
    // A proven builtin the seed does not hold has no compiler-proven entry.
    assert!(gap_chirho(vec![], &env_chirho, &name_chirho("Ix")).is_some());
}

#[test]
fn a_carried_homonym_under_a_builtin_name_is_a_gap_chirho() {
    // The module-sources path carries the previous module's whole environment: its own `Show`
    // replaced the seed there, and no local homonym or absence of one shows it.
    let mut carried_chirho = seeded_chirho();
    let homonym_chirho = carried_chirho
        .classes_chirho
        .get_mut("Show")
        .expect("the seed holds Show");
    homonym_chirho.methods_chirho.clear();
    let gap_chirho_text = gap_chirho(vec![], &carried_chirho, &name_chirho("Show"))
        .expect("a carried homonym is not the builtin");
    assert!(
        gap_chirho_text.contains("another module's class"),
        "{gap_chirho_text}"
    );
}

#[test]
fn an_imported_user_class_is_a_named_gap_chirho() {
    let env_chirho = seeded_chirho();
    assert!(gap_chirho(vec![], &env_chirho, &name_chirho("PrettyChirho")).is_some());
    assert!(
        gap_chirho(
            vec![],
            &env_chirho,
            &qualified_chirho("Lib", "PrettyChirho")
        )
        .is_some()
    );
}
