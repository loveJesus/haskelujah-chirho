// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

fn candidates_chirho(
    source_chirho: &str,
    name_chirho: &str,
    parent_chirho: Option<&str>,
) -> Vec<PreludeRouteChirho> {
    let facades_chirho = BTreeMap::from([
        (
            "FacadeChirho".into(),
            read_chirho(source_chirho, "FacadeChirho").unwrap(),
        ),
        (
            TUPLE_CHIRHO.module_chirho.into(),
            read_chirho(TUPLE_CHIRHO.bytes_chirho, TUPLE_CHIRHO.module_chirho).unwrap(),
        ),
        (
            NUM_MODULE_CHIRHO.into(),
            read_chirho(NUM_CHIRHO.bytes_chirho, NUM_MODULE_CHIRHO).unwrap(),
        ),
    ]);
    let mut result_chirho = Vec::new();
    routes_from_chirho(
        "FacadeChirho",
        &(name_chirho.into(), NamespaceChirho::ValueChirho),
        parent_chirho,
        &facades_chirho,
        &mut BTreeSet::new(),
        &mut result_chirho,
    );
    result_chirho
}

#[test]
fn import_lists_hiding_qualification_and_aliases_bound_export_paths_chirho() {
    for import_chirho in [
        "import qualified GHC.Internal.Data.Tuple",
        "import GHC.Internal.Data.Tuple as OtherChirho",
        "import GHC.Internal.Data.Tuple (snd)",
        "import GHC.Internal.Data.Tuple hiding (fst)",
    ] {
        let source_chirho = format!("module FacadeChirho (fst) where\n{import_chirho}\n");
        assert!(
            candidates_chirho(&source_chirho, "fst", None).is_empty(),
            "{import_chirho}"
        );
    }
    assert!(
        !candidates_chirho(
            "module FacadeChirho (fst) where\nimport GHC.Internal.Data.Tuple (fst)\n",
            "fst",
            None
        )
        .is_empty()
    );
}

#[test]
fn module_reexports_need_a_real_unqualified_import_edge_chirho() {
    for source_chirho in [
        "module FacadeChirho (module GHC.Internal.Data.Tuple) where\n",
        "module FacadeChirho (module GHC.Internal.Data.Tuple) where\nimport qualified GHC.Internal.Data.Tuple\n",
    ] {
        assert!(candidates_chirho(source_chirho, "fst", None).is_empty());
    }
    assert!(!candidates_chirho("module FacadeChirho (module GHC.Internal.Data.Tuple) where\nimport GHC.Internal.Data.Tuple\n", "fst", None).is_empty());
}

#[test]
fn importing_a_class_alone_does_not_supply_its_members_chirho() {
    assert!(
        candidates_chirho(
            "module FacadeChirho (Num((+))) where\nimport GHC.Internal.Num (Num)\n",
            "+",
            Some("Num")
        )
        .is_empty()
    );
    assert!(
        !candidates_chirho(
            "module FacadeChirho (Num((+))) where\nimport GHC.Internal.Num (Num((+)))\n",
            "+",
            Some("Num")
        )
        .is_empty()
    );
}

#[test]
fn conditional_or_defining_directives_cannot_supply_facade_authority_chirho() {
    for directive_chirho in [
        "#if FLAG_CHIRHO",
        "#ifdef FLAG_CHIRHO",
        "#define FLAG_CHIRHO 1",
    ] {
        let source_chirho = format!(
            "module FacadeChirho (fst) where\n{directive_chirho}\nimport GHC.Internal.Data.Tuple\n"
        );
        assert!(read_chirho(&source_chirho, "FacadeChirho").is_err());
    }
}
