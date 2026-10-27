// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The shared link decision on hand-built modules (row 629): each rule, and each refusal's text.
//! workflow: compiler-pipeline-chirho/native-symbol-identity-chirho

use std::collections::HashMap;

use haskelujah_core_chirho::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho,
    InlineAnnotationChirho,
};
use haskelujah_core_chirho::{CoreProvenanceChirho, ImportedReferenceChirho};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::ImportLinkerChirho;

fn binding_chirho(name_chirho: &str, id_chirho: u32) -> CoreBindingChirho {
    CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(i64::from(id_chirho))),
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    }
}

fn reference_chirho(name_chirho: &str, suppliers_chirho: &[&str]) -> ImportedReferenceChirho {
    ImportedReferenceChirho {
        name_chirho: name_chirho.to_string(),
        suppliers_chirho: suppliers_chirho.iter().map(|m| m.to_string()).collect(),
    }
}

/// A hand-built module: its name, its bindings as (name, id), and its compiler-owned ids.
type ModuleSpecChirho<'a> = (&'a str, &'a [(&'a str, u32)], &'a [u32]);

/// A linker over `modules_chirho`, with the references `(id, name, suppliers)` all recorded by
/// the last module.
fn linker_chirho(
    modules_chirho: &[ModuleSpecChirho<'_>],
    references_chirho: &[(u32, &str, &[&str])],
) -> ImportLinkerChirho {
    let mut linker_chirho = ImportLinkerChirho::default();
    for (index_chirho, (module_chirho, bindings_chirho, compiler_chirho)) in
        modules_chirho.iter().enumerate()
    {
        let bindings_chirho: Vec<_> = bindings_chirho
            .iter()
            .map(|(name_chirho, id_chirho)| binding_chirho(name_chirho, *id_chirho))
            .collect();
        let mut provenance_chirho = CoreProvenanceChirho {
            compiler_bindings_chirho: compiler_chirho.iter().copied().map(CoreIdChirho).collect(),
            ..Default::default()
        };
        if index_chirho + 1 == modules_chirho.len() {
            provenance_chirho.imported_references_chirho = references_chirho
                .iter()
                .map(|(id_chirho, name_chirho, suppliers_chirho)| {
                    (
                        CoreIdChirho(*id_chirho),
                        reference_chirho(name_chirho, suppliers_chirho),
                    )
                })
                .collect();
        }
        linker_chirho.add_module_chirho(module_chirho, &bindings_chirho, &provenance_chirho);
    }
    linker_chirho
}

fn link_chirho(linker_chirho: &ImportLinkerChirho) -> Result<HashMap<u32, u32>, String> {
    linker_chirho.link_chirho().map(|remap_chirho| {
        remap_chirho
            .into_iter()
            .map(|(from_chirho, to_chirho)| (from_chirho.0, to_chirho.0))
            .collect()
    })
}

#[test]
fn a_direct_import_names_its_supplier_beside_any_namesake_chirho() {
    let linker_chirho = linker_chirho(
        &[
            ("B", &[("limit", 1)], &[]),
            ("A", &[("limit", 2)], &[]),
            ("Main", &[], &[]),
        ],
        &[(10, "limit", &["A"]), (11, "limit", &["B"])],
    );
    assert_eq!(
        link_chirho(&linker_chirho),
        Ok(HashMap::from([(10, 2), (11, 1)]))
    );
    assert!(linker_chirho.is_imported_reference_chirho(&CoreIdChirho(10)));
    assert!(!linker_chirho.is_imported_reference_chirho(&CoreIdChirho(2)));
}

#[test]
fn a_compiler_binding_is_never_a_candidate_chirho() {
    // A's only `show` is the compiler's own: the reference must not link to it.
    let linker_chirho = linker_chirho(
        &[("A", &[("show", 1)], &[1]), ("Main", &[], &[])],
        &[(10, "show", &["A"])],
    );
    assert_eq!(
        link_chirho(&linker_chirho),
        Err(
            "cannot link `show`, imported from A: no module of the program defines it (row 629)"
                .to_string()
        )
    );
}

#[test]
fn a_re_export_links_only_to_the_one_definition_chirho() {
    let one_chirho = linker_chirho(
        &[
            ("B", &[("g", 1)], &[]),
            ("A", &[("h", 2)], &[]),
            ("Main", &[], &[]),
        ],
        &[(10, "g", &["A"])],
    );
    assert_eq!(link_chirho(&one_chirho), Ok(HashMap::from([(10, 1)])));
    let several_chirho = linker_chirho(
        &[
            ("C", &[("f", 3)], &[]),
            ("B", &[("f", 1)], &[]),
            ("A", &[], &[]),
            ("Main", &[], &[]),
        ],
        &[(10, "f", &["A"])],
    );
    assert_eq!(
        link_chirho(&several_chirho),
        Err(
            "cannot link `f`, imported from A: it is defined by B, C, and without the \
             re-export's origin (row 642) the link would guess (row 629)"
                .to_string()
        )
    );
}

#[test]
fn an_ambiguous_unqualified_import_gets_no_link_chirho() {
    let linker_chirho = linker_chirho(
        &[
            ("A", &[("f", 1)], &[]),
            ("B", &[("f", 2)], &[]),
            ("Main", &[], &[]),
        ],
        &[(10, "f", &["A", "B"])],
    );
    assert_eq!(
        link_chirho(&linker_chirho),
        Err(
            "cannot link `f`: it is imported unqualified from A, B, and each of them defines \
             its own (row 629)"
                .to_string()
        )
    );
}

#[test]
fn refusals_come_in_one_stable_order_chirho() {
    let linker_chirho = linker_chirho(
        &[
            ("A", &[("x", 1)], &[]),
            ("B", &[("x", 2)], &[]),
            ("Main", &[], &[]),
        ],
        &[(20, "x", &["A", "B"]), (10, "missing", &["A"])],
    );
    let refusals_chirho = link_chirho(&linker_chirho).unwrap_err();
    assert_eq!(
        refusals_chirho.lines().collect::<Vec<_>>(),
        [
            "cannot link `missing`, imported from A: no module of the program defines it \
             (row 629)",
            "cannot link `x`: it is imported unqualified from A, B, and each of them defines its \
             own (row 629)",
        ]
    );
}

#[test]
fn identity_names_spell_source_bindings_by_module_chirho() {
    let linker_chirho = linker_chirho(
        &[
            ("A", &[("helper", 1), ("show", 3)], &[3]),
            ("B", &[("helper", 2)], &[]),
        ],
        &[],
    );
    let names_chirho = HashMap::from([
        (CoreIdChirho(1), "helper".to_string()),
        (CoreIdChirho(2), "helper".to_string()),
        (CoreIdChirho(3), "show".to_string()),
        (CoreIdChirho(9), ">>".to_string()),
    ]);
    let identity_chirho = linker_chirho.identity_names_chirho(&names_chirho);
    assert_eq!(identity_chirho[&CoreIdChirho(1)], "A::helper");
    assert_eq!(identity_chirho[&CoreIdChirho(2)], "B::helper");
    assert_eq!(identity_chirho[&CoreIdChirho(3)], "show");
    assert_eq!(identity_chirho[&CoreIdChirho(9)], ">>");
}
