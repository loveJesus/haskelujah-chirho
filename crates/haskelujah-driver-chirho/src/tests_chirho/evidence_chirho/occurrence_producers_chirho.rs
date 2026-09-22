// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The producers after lowering: splice expansion and deriving draw fresh
//! origins from the module's one supply for exactly the declarations they
//! generate, and every declaration that passes through keeps the identity
//! lowering gave it.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashSet;

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::occurrences_chirho::visit_decl_chirho;
use haskelujah_ast_chirho::provenance_chirho::OriginIdChirho;
use haskelujah_span_chirho::SourceMapChirho;

fn lower_chirho(source_chirho: &str) -> ModuleChirho {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let file_id_chirho = source_map_chirho.add_file_chirho("ProducersChirho.hs", source_chirho);
    let green_chirho = haskelujah_parser_chirho::cst_parser_chirho::ParserChirho::new_chirho(
        source_chirho,
        file_id_chirho,
    )
    .parse_chirho();
    haskelujah_parser_chirho::lower_chirho::lower_module_chirho(&green_chirho, file_id_chirho)
}

/// The origins of one declaration's occurrences, in source order. `None` marks
/// an occurrence that has no origin.
fn origins_chirho(decl_chirho: &mut DeclChirho) -> Vec<Option<OriginIdChirho>> {
    let mut found_chirho = Vec::new();
    visit_decl_chirho(decl_chirho, &mut |occurrence_chirho| {
        found_chirho.push(occurrence_chirho.origin_chirho());
    });
    found_chirho
}

fn module_origins_chirho(module_chirho: &mut ModuleChirho) -> Vec<Vec<Option<OriginIdChirho>>> {
    module_chirho
        .decls_chirho
        .iter_mut()
        .map(origins_chirho)
        .collect()
}

/// Every occurrence in the module has an origin and no two share one.
fn assert_all_distinct_chirho(module_chirho: &mut ModuleChirho) -> usize {
    let all_chirho: Vec<OriginIdChirho> = module_origins_chirho(module_chirho)
        .into_iter()
        .flatten()
        .map(|origin_chirho| origin_chirho.expect("an occurrence without an origin"))
        .collect();
    let distinct_chirho: HashSet<OriginIdChirho> = all_chirho.iter().copied().collect();
    assert_eq!(distinct_chirho.len(), all_chirho.len(), "{all_chirho:?}");
    all_chirho.len()
}

#[test]
fn derived_instances_take_fresh_origins_and_user_code_keeps_its_own_chirho() {
    let mut module_chirho = lower_chirho(
        "module Main where\n\
         data PairChirho = PairChirho Bool Int deriving (Show, Eq, Ord)\n\
         main = print (PairChirho True 1 == PairChirho False 2)\n",
    );
    let before_chirho = module_origins_chirho(&mut module_chirho);
    let lowered_count_chirho = before_chirho.len();

    haskelujah_typing_chirho::deriving_chirho::apply_deriving_chirho(&mut module_chirho);

    let after_chirho = module_origins_chirho(&mut module_chirho);
    assert!(
        after_chirho.len() > lowered_count_chirho,
        "deriving generated nothing"
    );
    // Passthrough identity: what lowering stamped is untouched.
    assert_eq!(after_chirho[..lowered_count_chirho], before_chirho[..]);
    // The generated instances reference `showsPrec`, `==`, `compare` and the
    // like many times under one placeholder span, each with its own origin, and
    // none reuses an origin lowering minted.
    let generated_chirho: usize = after_chirho[lowered_count_chirho..]
        .iter()
        .map(Vec::len)
        .sum();
    assert!(generated_chirho > 3, "{after_chirho:?}");
    assert_all_distinct_chirho(&mut module_chirho);
}

#[test]
fn a_spliced_declaration_takes_fresh_origins_and_passthrough_keeps_its_own_chirho() {
    let mut module_chirho = lower_chirho(
        "module Main where\n\
         data PersonChirho = PersonChirho { _nameChirho :: String, _ageChirho :: Int }\n\
         $(makeLenses ''PersonChirho)\n\
         main = putStrLn (show (1 + 2))\n",
    );
    let main_before_chirho = module_origins_chirho(&mut module_chirho)
        .last()
        .cloned()
        .expect("main");
    assert!(!main_before_chirho.is_empty());

    let decls_chirho = std::mem::take(&mut module_chirho.decls_chirho);
    let expanded_chirho = crate::splice_chirho::expand_splices_chirho(
        decls_chirho,
        &mut module_chirho.origin_supply_chirho,
    );
    module_chirho.decls_chirho = expanded_chirho.decls_chirho;

    let after_chirho = module_origins_chirho(&mut module_chirho);
    // `main` passed through the expansion and keeps every origin it had.
    let main_index_chirho = module_chirho
        .decls_chirho
        .iter()
        .position(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "main")
        })
        .expect("main survives expansion");
    assert_eq!(after_chirho[main_index_chirho], main_before_chirho);
    // The lenses are new code with origins of their own.
    let lens_occurrences_chirho: usize = module_chirho
        .decls_chirho
        .iter_mut()
        .filter(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho().ends_with("Chirho")
                    && name_chirho.text_chirho() != "main")
        })
        .map(|decl_chirho| origins_chirho(decl_chirho).len())
        .sum();
    assert!(
        lens_occurrences_chirho > 0,
        "{:?}",
        module_chirho.decls_chirho
    );
    assert_all_distinct_chirho(&mut module_chirho);
}

#[test]
fn lowering_splicing_and_deriving_continue_one_supply_chirho() {
    // Three producers at three times, one sequence: an origin minted early is
    // never minted again later.
    let mut module_chirho = lower_chirho(
        "module Main where\n\
         data ToyChirho = ToyChirho { _sizeChirho :: Int } deriving (Show, Eq)\n\
         $(makeLenses ''ToyChirho)\n\
         main = print (ToyChirho 3 == ToyChirho 4)\n",
    );
    let minted_by_lowering_chirho = module_chirho.origin_supply_chirho.minted_chirho();
    let decls_chirho = std::mem::take(&mut module_chirho.decls_chirho);
    module_chirho.decls_chirho = crate::splice_chirho::expand_splices_chirho(
        decls_chirho,
        &mut module_chirho.origin_supply_chirho,
    )
    .decls_chirho;
    let minted_by_splicing_chirho = module_chirho.origin_supply_chirho.minted_chirho();
    haskelujah_typing_chirho::deriving_chirho::apply_deriving_chirho(&mut module_chirho);
    let minted_by_deriving_chirho = module_chirho.origin_supply_chirho.minted_chirho();
    assert!(minted_by_lowering_chirho < minted_by_splicing_chirho);
    assert!(minted_by_splicing_chirho < minted_by_deriving_chirho);
    assert_all_distinct_chirho(&mut module_chirho);
}
