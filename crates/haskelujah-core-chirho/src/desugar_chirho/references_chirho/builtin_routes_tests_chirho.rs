// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Which written import lists let a routed `toList` through (unit 517, #28489, #28494):
//! every form gpt_chirho named, for both routed modules, in pairs that differ only in the
//! item under test.

use super::*;
use haskelujah_ast_chirho::module_chirho::ImportSpecChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_span_chirho::SpanChirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn var_chirho(text_chirho: &str) -> ImportItemChirho {
    ImportItemChirho::VarChirho(name_chirho(text_chirho))
}

fn class_chirho(text_chirho: &str, members_chirho: ExportMembersChirho) -> ImportItemChirho {
    ImportItemChirho::TyConChirho {
        name_chirho: name_chirho(text_chirho),
        members_chirho,
    }
}

fn some_chirho(texts_chirho: &[&str]) -> ExportMembersChirho {
    ExportMembersChirho::SomeChirho(texts_chirho.iter().copied().map(name_chirho).collect())
}

fn unrepresented_chirho() -> ImportItemChirho {
    ImportItemChirho::UnrepresentedChirho(SpanChirho::DUMMY_CHIRHO)
}

/// An unqualified import of `module`; `None` is the unrestricted form.
fn import_chirho(
    module_chirho: &str,
    list_chirho: Option<(bool, Vec<ImportItemChirho>)>,
) -> ImportDeclChirho {
    ImportDeclChirho {
        module_chirho: name_chirho(module_chirho),
        qualified_chirho: false,
        alias_chirho: None,
        spec_chirho: list_chirho.map(|(hiding_chirho, items_chirho)| ImportSpecChirho {
            hiding_chirho,
            items_chirho,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn exposes_chirho(module_chirho: &str, list_chirho: Option<(bool, Vec<ImportItemChirho>)>) -> bool {
    exposes_routed_to_list_chirho(&import_chirho(module_chirho, list_chirho))
}

const NON_EMPTY_CHIRHO: &str = "Data.List.NonEmpty";
const FOLDABLE_CHIRHO: &str = "Data.Foldable";

#[test]
fn an_unrestricted_import_of_either_routed_module_exposes_to_list_chirho() {
    assert!(exposes_chirho(NON_EMPTY_CHIRHO, None));
    assert!(exposes_chirho(FOLDABLE_CHIRHO, None));
}

#[test]
fn an_explicit_list_exposes_to_list_only_when_an_item_names_it_chirho() {
    let non_empty_type_chirho = || class_chirho("NonEmpty", ExportMembersChirho::AllChirho);
    assert!(exposes_chirho(
        NON_EMPTY_CHIRHO,
        Some((false, vec![var_chirho("toList")]))
    ));
    assert!(!exposes_chirho(
        NON_EMPTY_CHIRHO,
        Some((false, vec![var_chirho("head")]))
    ));
    // `NonEmpty(..)` brings the constructor, never the top-level `toList`.
    assert!(!exposes_chirho(
        NON_EMPTY_CHIRHO,
        Some((false, vec![non_empty_type_chirho()]))
    ));
    assert!(exposes_chirho(
        NON_EMPTY_CHIRHO,
        Some((false, vec![non_empty_type_chirho(), var_chirho("toList")]))
    ));
    assert!(!exposes_chirho(NON_EMPTY_CHIRHO, Some((false, vec![]))));
}

#[test]
fn a_hiding_list_exposes_to_list_unless_an_item_names_it_chirho() {
    assert!(!exposes_chirho(
        NON_EMPTY_CHIRHO,
        Some((true, vec![var_chirho("toList")]))
    ));
    assert!(exposes_chirho(
        NON_EMPTY_CHIRHO,
        Some((true, vec![var_chirho("head")]))
    ));
    // An unrepresented item hides nothing.
    assert!(exposes_chirho(
        NON_EMPTY_CHIRHO,
        Some((true, vec![unrepresented_chirho()]))
    ));
}

#[test]
fn an_explicit_list_with_an_unrepresented_item_is_unrestricted_chirho() {
    assert!(exposes_chirho(
        NON_EMPTY_CHIRHO,
        Some((false, vec![var_chirho("head"), unrepresented_chirho()]))
    ));
}

#[test]
fn foldable_member_forms_name_its_to_list_chirho() {
    for (members_chirho, named_chirho) in [
        (ExportMembersChirho::AllChirho, true),
        (some_chirho(&["toList"]), true),
        (some_chirho(&["foldr", "toList"]), true),
        (some_chirho(&["foldr"]), false),
        (ExportMembersChirho::NoneChirho, false),
    ] {
        let foldable_chirho = class_chirho("Foldable", members_chirho);
        assert_eq!(
            exposes_chirho(
                FOLDABLE_CHIRHO,
                Some((false, vec![foldable_chirho.clone()]))
            ),
            named_chirho,
            "{foldable_chirho:?}"
        );
        assert_eq!(
            exposes_chirho(FOLDABLE_CHIRHO, Some((true, vec![foldable_chirho.clone()]))),
            !named_chirho,
            "hiding {foldable_chirho:?}"
        );
    }
    assert!(exposes_chirho(
        FOLDABLE_CHIRHO,
        Some((false, vec![var_chirho("toList")]))
    ));
    assert!(!exposes_chirho(
        FOLDABLE_CHIRHO,
        Some((true, vec![var_chirho("toList")]))
    ));
    assert!(!exposes_chirho(
        FOLDABLE_CHIRHO,
        Some((false, vec![var_chirho("foldr")]))
    ));
}

#[test]
fn a_class_member_list_does_not_name_non_empty_to_list_chirho() {
    // Data.List.NonEmpty exports `toList` as a plain value, so no class item names it.
    assert!(!exposes_chirho(
        NON_EMPTY_CHIRHO,
        Some((
            false,
            vec![class_chirho("Foldable", ExportMembersChirho::AllChirho)]
        ))
    ));
}

#[test]
fn other_modules_are_never_routed_chirho() {
    for module_chirho in ["Data.Map", "GHC.Exts", "Data.List", "Prelude"] {
        assert!(!exposes_chirho(module_chirho, None), "{module_chirho}");
        assert!(
            !exposes_chirho(module_chirho, Some((false, vec![var_chirho("toList")]))),
            "{module_chirho}"
        );
    }
}
