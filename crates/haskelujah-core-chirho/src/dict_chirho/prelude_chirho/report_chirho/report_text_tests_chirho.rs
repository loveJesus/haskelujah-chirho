// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 579's structural controls, run on the dictionary pass alone:
//! - each of the five text functions is generated once, and its body holds no text primop;
//! - the space ranges are exactly GHC 9.14.1's `isSpace`, measured over every `Char`.
//!
//! workflow: compiler-pipeline-chirho/runtime-primitive-evaluation-chirho

use std::collections::HashMap;

use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::env_chirho::TyEnvChirho;

use super::{GHC_SPACE_RANGES_CHIRHO, IS_SPACE_ROW_CHIRHO, LOW_SPACE_RANGES_CHIRHO};
use crate::dict_chirho::dict_pass_module_chirho;
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho, CoreModuleChirho};
use crate::transform_chirho::children_chirho;

/// The strict text primops the five functions were.
const TEXT_PRIMOPS_CHIRHO: [&str; 5] = [
    "lines#",
    "wordsStr#",
    "unlines#",
    "unwordsStr#",
    "intercalateStr#",
];

/// `[fromEnum c | c <- [minBound .. maxBound :: Char], isSpace c]` under runghc 9.14.1.
const GHC_SPACE_CODE_POINTS_CHIRHO: [i64; 22] = [
    9, 10, 11, 12, 13, 32, 160, 5760, 8192, 8193, 8194, 8195, 8196, 8197, 8198, 8199, 8200, 8201,
    8202, 8239, 8287, 12288,
];

fn var_ids_chirho(expr_chirho: &CoreExprChirho) -> Vec<CoreIdChirho> {
    let mut ids_chirho = Vec::new();
    let mut pending_chirho = vec![expr_chirho];
    while let Some(next_chirho) = pending_chirho.pop() {
        if let CoreExprChirho::VarChirho(id_chirho) = next_chirho {
            ids_chirho.push(*id_chirho);
        }
        children_chirho(next_chirho, &mut |child_chirho| {
            pending_chirho.push(child_chirho)
        });
    }
    ids_chirho
}

fn primop_names_chirho(expr_chirho: &CoreExprChirho) -> Vec<&str> {
    let mut names_chirho = Vec::new();
    let mut pending_chirho = vec![expr_chirho];
    while let Some(next_chirho) = pending_chirho.pop() {
        if let CoreExprChirho::PrimOpChirho { name_chirho, .. } = next_chirho {
            names_chirho.push(name_chirho.as_str());
        }
        children_chirho(next_chirho, &mut |child_chirho| {
            pending_chirho.push(child_chirho)
        });
    }
    names_chirho
}

#[test]
fn each_text_function_is_generated_once_with_no_text_primop_and_shares_is_space_chirho() {
    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    class_env_chirho.seed_standard_chirho();
    let module_chirho = CoreModuleChirho {
        name_chirho: "ReportTextChirho".to_string(),
        bindings_chirho: Vec::new(),
        names_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: Vec::new(),
        provenance_chirho: Default::default(),
    };
    let result_chirho = dict_pass_module_chirho(
        &module_chirho,
        HashMap::new(),
        &TyEnvChirho::new_chirho(),
        &class_env_chirho,
    );
    for name_chirho in ["lines", "words", "unlines", "unwords", "intercalate"] {
        let bindings_chirho: Vec<_> = result_chirho
            .module_chirho
            .bindings_chirho
            .iter()
            .filter(|binding_chirho| binding_chirho.binder_chirho.name_chirho == name_chirho)
            .collect();
        assert_eq!(bindings_chirho.len(), 1, "{name_chirho} is generated once");
        let primops_chirho = primop_names_chirho(&bindings_chirho[0].rhs_chirho);
        for text_primop_chirho in TEXT_PRIMOPS_CHIRHO {
            assert!(
                !primops_chirho.contains(&text_primop_chirho),
                "{name_chirho} still reaches {text_primop_chirho}"
            );
        }
    }
    // The public `isSpace` is the very test `words` drops spaces with, so they agree on every
    // Char, and it no longer reaches `isSpace#`.
    let named_chirho = |name_chirho: &str| {
        let found_chirho: Vec<_> = result_chirho
            .module_chirho
            .bindings_chirho
            .iter()
            .filter(|binding_chirho| binding_chirho.binder_chirho.name_chirho == name_chirho)
            .collect();
        assert_eq!(found_chirho.len(), 1, "{name_chirho} is generated once");
        found_chirho[0]
    };
    let shared_chirho = named_chirho(IS_SPACE_ROW_CHIRHO);
    assert!(matches!(
        named_chirho("isSpace").rhs_chirho,
        CoreExprChirho::VarChirho(id_chirho) if id_chirho == shared_chirho.binder_chirho.id_chirho
    ));
    assert!(!primop_names_chirho(&shared_chirho.rhs_chirho).contains(&"isSpace#"));
    assert!(
        var_ids_chirho(&named_chirho("words").rhs_chirho)
            .contains(&shared_chirho.binder_chirho.id_chirho)
    );
}

#[test]
fn the_space_ranges_are_ghcs_is_space_chirho() {
    let expanded_chirho: Vec<i64> = GHC_SPACE_RANGES_CHIRHO
        .iter()
        .flat_map(|&(low_chirho, high_chirho)| low_chirho..=high_chirho)
        .collect();
    assert_eq!(expanded_chirho, GHC_SPACE_CODE_POINTS_CHIRHO);
    // The fast path settles every code point strictly between the space and U+00A0 as no space.
    let (low_chirho, high_chirho) = GHC_SPACE_RANGES_CHIRHO.split_at(LOW_SPACE_RANGES_CHIRHO);
    assert!(low_chirho.iter().all(|&(_, end_chirho)| end_chirho <= 32));
    assert!(
        high_chirho
            .iter()
            .all(|&(start_chirho, _)| start_chirho >= 160)
    );
}
