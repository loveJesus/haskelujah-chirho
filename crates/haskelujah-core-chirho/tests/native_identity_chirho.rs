// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 628: the native symbol table is injective and deterministic, only the recorded entry
//! takes the entry symbol, and provenance moves with a module's ids through the merge.
//! workflow: compiler-pipeline-chirho/native-symbol-identity-chirho

use std::collections::{BTreeSet, HashMap};

use haskelujah_core::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho, CoreModuleChirho,
    CoreProvenanceChirho, InlineAnnotationChirho, NATIVE_ENTRY_SYMBOL_CHIRHO, NativeSymbolsChirho,
    native_mangle_chirho,
};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

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

fn module_chirho(bindings_chirho: &[(&str, u32)], entry_chirho: Option<u32>) -> CoreModuleChirho {
    let bindings_chirho: Vec<_> = bindings_chirho
        .iter()
        .map(|(name_chirho, id_chirho)| binding_chirho(name_chirho, *id_chirho))
        .collect();
    let names_chirho = bindings_chirho
        .iter()
        .map(|binding_chirho| {
            (
                binding_chirho.binder_chirho.id_chirho,
                binding_chirho.binder_chirho.name_chirho.clone(),
            )
        })
        .collect();
    CoreModuleChirho {
        name_chirho: "Main".to_string(),
        bindings_chirho,
        names_chirho,
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: CoreProvenanceChirho {
            entry_chirho: entry_chirho.map(CoreIdChirho),
            ..Default::default()
        },
    }
}

/// Spellings the mangler merges, and two modules' namesakes.
const COLLIDING_CHIRHO: [(&str, u32); 8] = [
    ("f'", 1),
    ("f_prime", 2),
    ("a.b", 3),
    ("a_b", 4),
    ("+++", 5),
    ("_u002b_u002b_u002b", 6),
    ("helper", 7),
    ("helper", 8),
];

#[test]
fn symbols_stay_apart_where_spellings_collide_chirho() {
    assert_eq!(native_mangle_chirho("f'"), native_mangle_chirho("f_prime"));
    assert_eq!(native_mangle_chirho("a.b"), native_mangle_chirho("a_b"));
    assert_eq!(
        native_mangle_chirho("+++"),
        native_mangle_chirho("_u002b_u002b_u002b")
    );
    let table_chirho =
        NativeSymbolsChirho::of_module_chirho(&module_chirho(&COLLIDING_CHIRHO, None));
    let symbols_chirho: BTreeSet<&str> = COLLIDING_CHIRHO
        .iter()
        .map(|(_, id_chirho)| {
            table_chirho
                .symbol_chirho(CoreIdChirho(*id_chirho))
                .unwrap()
        })
        .collect();
    assert_eq!(
        symbols_chirho.len(),
        COLLIDING_CHIRHO.len(),
        "{symbols_chirho:?}"
    );
    for symbol_chirho in &symbols_chirho {
        // A name-written symbol (a runtime function, an import) never contains a `.`.
        assert!(symbol_chirho.contains('.'), "{symbol_chirho}");
        assert!(!native_mangle_chirho(symbol_chirho).contains('.'));
    }
}

#[test]
fn the_table_does_not_depend_on_construction_order_chirho() {
    let first_chirho =
        NativeSymbolsChirho::of_module_chirho(&module_chirho(&COLLIDING_CHIRHO, Some(7)));
    assert_eq!(
        first_chirho,
        NativeSymbolsChirho::of_module_chirho(&module_chirho(&COLLIDING_CHIRHO, Some(7)))
    );
    let mut reversed_chirho = COLLIDING_CHIRHO;
    reversed_chirho.reverse();
    let mut rotated_chirho = COLLIDING_CHIRHO;
    rotated_chirho.rotate_left(3);
    for order_chirho in [reversed_chirho, rotated_chirho] {
        let table_chirho =
            NativeSymbolsChirho::of_module_chirho(&module_chirho(&order_chirho, Some(7)));
        assert_eq!(table_chirho, first_chirho);
    }
}

#[test]
fn only_the_recorded_entry_takes_the_entry_symbol_chirho() {
    let mains_chirho = [("main", 1), ("main", 7), ("helper", 3)];
    let table_chirho =
        NativeSymbolsChirho::of_module_chirho(&module_chirho(&mains_chirho, Some(7)));
    assert_eq!(
        table_chirho.symbol_chirho(CoreIdChirho(7)),
        Some(NATIVE_ENTRY_SYMBOL_CHIRHO)
    );
    assert_ne!(
        table_chirho.symbol_chirho(CoreIdChirho(1)),
        Some(NATIVE_ENTRY_SYMBOL_CHIRHO)
    );
    let unrecorded_chirho =
        NativeSymbolsChirho::of_module_chirho(&module_chirho(&mains_chirho, None));
    for (_, id_chirho) in mains_chirho {
        assert_ne!(
            unrecorded_chirho.symbol_chirho(CoreIdChirho(id_chirho)),
            Some(NATIVE_ENTRY_SYMBOL_CHIRHO)
        );
    }
}

#[test]
fn provenance_moves_with_the_merge_chirho() {
    let ids_chirho = |ids_chirho: &[u32]| -> BTreeSet<CoreIdChirho> {
        ids_chirho.iter().copied().map(CoreIdChirho).collect()
    };
    let dependency_chirho = CoreProvenanceChirho {
        entry_chirho: Some(CoreIdChirho(4)),
        compiler_bindings_chirho: ids_chirho(&[1, 2]),
        builtin_references_chirho: ids_chirho(&[9]),
    };
    let moved_chirho = dependency_chirho.offset_chirho(100);
    assert_eq!(moved_chirho.entry_chirho, Some(CoreIdChirho(104)));
    assert_eq!(
        moved_chirho.compiler_bindings_chirho,
        ids_chirho(&[101, 102])
    );
    assert_eq!(moved_chirho.builtin_references_chirho, ids_chirho(&[109]));

    // Joining another module never takes its entry.
    let mut program_chirho = CoreProvenanceChirho {
        entry_chirho: Some(CoreIdChirho(7)),
        ..Default::default()
    };
    program_chirho.absorb_chirho(&moved_chirho);
    assert_eq!(program_chirho.entry_chirho, Some(CoreIdChirho(7)));
    assert_eq!(
        program_chirho.compiler_bindings_chirho,
        ids_chirho(&[101, 102])
    );
    assert_eq!(program_chirho.builtin_references_chirho, ids_chirho(&[109]));

    // Linking and deduplication renames reach every role.
    let renames_chirho = HashMap::from([
        (CoreIdChirho(7), CoreIdChirho(70)),
        (CoreIdChirho(102), CoreIdChirho(1)),
        (CoreIdChirho(109), CoreIdChirho(90)),
    ]);
    program_chirho.remap_chirho(&renames_chirho);
    assert_eq!(program_chirho.entry_chirho, Some(CoreIdChirho(70)));
    assert_eq!(
        program_chirho.compiler_bindings_chirho,
        ids_chirho(&[1, 101])
    );
    assert_eq!(program_chirho.builtin_references_chirho, ids_chirho(&[90]));
}
