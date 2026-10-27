// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Import items lowering cannot represent (row 536), and the `data` namespace.
//!
//! A nameless namespace wildcard (`data ..`, `type ..`), a `pattern P` item and
//! a `data` item with a member list have no faithful AST item. Lowering keeps
//! each as `UnrepresentedChirho`, instead of dropping the wildcard, reading
//! `pattern` as a variable or folding the list into a name, so an explicit list
//! holding one is read as incomplete. A variable that is really named
//! `pattern` stays a variable. The wildcards come from a later GHC testsuite
//! (T25901, copied into ghc-tests-chirho): GHC 9.14.1 itself rejects them as a
//! parse error (GHC-58481, measured), but lowering accepts them, so they must
//! not close a list either.
//!
//! A named `data x` item is a value item whatever the spelling of `x`
//! (gpt_chirho #28517): GHC 9.14.1 imports the constructor for `data Proxy`,
//! never the type of the same name. A named `type T` stays a bare type item.
//! workflow: compiler-pipeline-chirho/value-scope-resolution-chirho

use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ImportDeclChirho, ImportItemChirho, ImportListModeChirho, ModuleChirho,
};
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use crate::cst_parser_chirho::parse_to_cst_chirho;

fn lower_chirho(source_chirho: &str) -> ModuleChirho {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(source_chirho, file_chirho);
    lower_module_chirho(&cst_chirho, file_chirho)
}

/// The module's one import declaration.
fn only_import_chirho(source_chirho: &str) -> ImportDeclChirho {
    let module_chirho = lower_chirho(source_chirho);
    assert_eq!(
        module_chirho.imports_chirho.len(),
        1,
        "one import in {source_chirho:?}"
    );
    module_chirho
        .imports_chirho
        .into_iter()
        .next()
        .expect("one import")
}

fn items_chirho(import_chirho: &ImportDeclChirho) -> &[ImportItemChirho] {
    &import_chirho
        .spec_chirho
        .as_ref()
        .expect("an item list")
        .items_chirho
}

#[test]
fn a_namespace_wildcard_is_kept_as_unrepresented_chirho() {
    for source_chirho in [
        "module M where\nimport qualified Data.Proxy as D (data ..)\n",
        "module M where\nimport qualified Data.Proxy as T (type ..)\n",
    ] {
        let import_chirho = only_import_chirho(source_chirho);
        assert!(
            matches!(
                items_chirho(&import_chirho),
                [ImportItemChirho::UnrepresentedChirho(_)]
            ),
            "{source_chirho:?} lowered to {:?}",
            items_chirho(&import_chirho)
        );
        assert_eq!(
            import_chirho.list_mode_chirho(),
            ImportListModeChirho::UnrestrictedChirho
        );
    }
}

#[test]
fn a_pattern_item_is_unrepresented_but_a_variable_named_pattern_is_not_chirho() {
    let import_chirho =
        only_import_chirho("module M where\nimport Lib (pattern P, head, pattern)\n");
    let lowered_chirho = items_chirho(&import_chirho);
    assert_eq!(lowered_chirho.len(), 3, "{lowered_chirho:?}");
    assert!(matches!(
        lowered_chirho[0],
        ImportItemChirho::UnrepresentedChirho(_)
    ));
    assert!(matches!(
        &lowered_chirho[1],
        ImportItemChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "head"
    ));
    assert!(matches!(
        &lowered_chirho[2],
        ImportItemChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "pattern"
    ));
}

#[test]
fn a_named_data_item_is_a_value_item_and_a_type_item_stays_a_type_chirho() {
    // The driver's GHC-measured `data Proxy` control: one value item, EXPLICIT.
    let proxy_chirho =
        only_import_chirho("module M where\nimport qualified Data.Proxy as P (data Proxy)\n");
    assert!(matches!(
        proxy_chirho.list_mode_chirho(),
        ImportListModeChirho::ExplicitChirho([
            ImportItemChirho::VarChirho(name_chirho)
        ]) if name_chirho.text_chirho() == "Proxy"
    ));

    let import_chirho = only_import_chirho(
        "module M where\nimport Lib (data Proxy, type Proxy, data f, data (:+))\n",
    );
    let lowered_chirho = items_chirho(&import_chirho);
    assert_eq!(lowered_chirho.len(), 4, "{lowered_chirho:?}");
    for (index_chirho, value_text_chirho) in [(0, "Proxy"), (2, "f"), (3, ":+")] {
        assert!(
            matches!(
                &lowered_chirho[index_chirho],
                ImportItemChirho::VarChirho(name_chirho)
                    if name_chirho.text_chirho() == value_text_chirho
            ),
            "{value_text_chirho}: {lowered_chirho:?}"
        );
    }
    assert!(matches!(
        &lowered_chirho[1],
        ImportItemChirho::TyConChirho {
            name_chirho,
            members_chirho: ExportMembersChirho::NoneChirho,
        } if name_chirho.text_chirho() == "Proxy"
    ));
    assert!(matches!(
        import_chirho.list_mode_chirho(),
        ImportListModeChirho::ExplicitChirho(_)
    ));
}

#[test]
fn a_data_item_with_a_member_list_is_unrepresented_chirho() {
    for source_chirho in [
        "module M where\nimport Lib (data T(..))\n",
        "module M where\nimport Lib (data T(MkT))\n",
    ] {
        let import_chirho = only_import_chirho(source_chirho);
        assert!(
            matches!(
                items_chirho(&import_chirho),
                [ImportItemChirho::UnrepresentedChirho(_)]
            ),
            "{source_chirho:?} lowered to {:?}",
            items_chirho(&import_chirho)
        );
        assert_eq!(
            import_chirho.list_mode_chirho(),
            ImportListModeChirho::UnrestrictedChirho
        );
    }
}

#[test]
fn represented_items_keep_their_shapes_beside_a_wildcard_chirho() {
    let import_chirho = only_import_chirho("module M where\nimport Lib (T(..), type .., f)\n");
    let lowered_chirho = items_chirho(&import_chirho);
    assert_eq!(lowered_chirho.len(), 3, "{lowered_chirho:?}");
    assert!(matches!(
        &lowered_chirho[0],
        ImportItemChirho::TyConChirho {
            name_chirho,
            members_chirho: ExportMembersChirho::AllChirho,
        } if name_chirho.text_chirho() == "T"
    ));
    assert!(matches!(
        lowered_chirho[1],
        ImportItemChirho::UnrepresentedChirho(_)
    ));
    assert!(matches!(
        &lowered_chirho[2],
        ImportItemChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "f"
    ));
    // One unrepresented item, and the explicit list makes no exclusion claim.
    assert_eq!(
        import_chirho.list_mode_chirho(),
        ImportListModeChirho::UnrestrictedChirho
    );
}

#[test]
fn hiding_keeps_an_unrepresented_item_and_a_plain_list_stays_explicit_chirho() {
    let hiding_chirho = only_import_chirho("module M where\nimport Lib hiding (data .., f)\n");
    assert!(matches!(
        hiding_chirho.list_mode_chirho(),
        ImportListModeChirho::HidingChirho([
            ImportItemChirho::UnrepresentedChirho(_),
            ImportItemChirho::VarChirho(_),
        ])
    ));
    // I2's own import: a represented explicit list under the alias `NE`.
    let explicit_chirho = only_import_chirho(
        "module M where\nimport qualified Data.List.NonEmpty as NE (head, NonEmpty(..))\n",
    );
    assert!(matches!(
        explicit_chirho.list_mode_chirho(),
        ImportListModeChirho::ExplicitChirho([
            ImportItemChirho::VarChirho(_),
            ImportItemChirho::TyConChirho { .. },
        ])
    ));
    assert_eq!(explicit_chirho.effective_qualifier_chirho(), "NE");
}
