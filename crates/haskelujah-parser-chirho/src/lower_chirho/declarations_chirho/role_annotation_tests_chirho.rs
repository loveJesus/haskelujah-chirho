// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Role annotations lower to their own declaration with every word as written (row 694,
//! #31844). Lowering judges nothing and rewrites nothing: declaration checking refuses what
//! GHC refuses (GHC-09009, GHC-54298, GHC-29178), so the programs here are lowering inputs,
//! not accepted programs.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;

use super::test_support_chirho::parse_and_lower_chirho;

/// Each role annotation's type and words, as written, in declaration order.
fn annotations_chirho(source_chirho: &str) -> Vec<(String, Vec<String>)> {
    let module_chirho = parse_and_lower_chirho(source_chirho);
    assert!(
        !module_chirho
            .decls_chirho
            .iter()
            .any(|decl_chirho| matches!(decl_chirho, DeclChirho::TypeSigChirho { .. })),
        "a role annotation is no type signature: {:?}",
        module_chirho.decls_chirho
    );
    module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::RoleAnnotationDeclChirho {
                type_name_chirho,
                roles_chirho,
                ..
            } => Some((
                type_name_chirho.text_chirho().to_string(),
                roles_chirho
                    .iter()
                    .map(|role_chirho| role_chirho.text_chirho().to_string())
                    .collect(),
            )),
            _ => None,
        })
        .collect()
}

fn words_chirho(words_chirho: &[&str]) -> Vec<String> {
    words_chirho
        .iter()
        .map(|word_chirho| word_chirho.to_string())
        .collect()
}

#[test]
fn a_role_annotation_keeps_its_words_chirho() {
    // GHC refuses the second annotation at declaration checking (GHC-29178: the stored
    // `bChirho` is representational), never at lowering.
    assert_eq!(
        annotations_chirho(
            "{-# LANGUAGE RoleAnnotations #-}\nmodule M where\ntype role HoldChirho nominal\ndata HoldChirho aChirho = HoldChirho aChirho\nnewtype PairChirho aChirho bChirho = PairChirho (aChirho, bChirho)\ntype role PairChirho _ phantom\n"
        ),
        [
            ("HoldChirho".to_string(), words_chirho(&["nominal"])),
            ("PairChirho".to_string(), words_chirho(&["_", "phantom"])),
        ]
    );
}

#[test]
fn a_malformed_role_annotation_is_kept_as_written_chirho() {
    // One role too few and a word that names no role: kept, never rewritten as nominal.
    assert_eq!(
        annotations_chirho(
            "module M where\ntype role PairChirho phantom\ndata PairChirho aChirho bChirho = PairChirho aChirho bChirho\ntype role BoxChirho shared\ndata BoxChirho aChirho = BoxChirho aChirho\n"
        ),
        [
            ("PairChirho".to_string(), words_chirho(&["phantom"])),
            ("BoxChirho".to_string(), words_chirho(&["shared"])),
        ]
    );
}

#[test]
fn an_operator_or_qualified_target_is_kept_as_written_chirho() {
    // T26805a's `(:-)`; declaration checking judges the qualified one.
    assert_eq!(
        annotations_chirho(
            "{-# LANGUAGE RoleAnnotations, TypeOperators #-}\nmodule M where\nnewtype a :- b = SubChirho (a, b)\ntype role (:-) nominal nominal\ndata HoldChirho aChirho = HoldChirho aChirho\ntype role M.HoldChirho representational\n"
        ),
        [
            (":-".to_string(), words_chirho(&["nominal", "nominal"])),
            (
                "HoldChirho".to_string(),
                words_chirho(&["representational"])
            ),
        ]
    );
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE RoleAnnotations #-}\nmodule M where\ndata HoldChirho aChirho = HoldChirho aChirho\ntype role M.HoldChirho representational\n",
    );
    assert!(
        module_chirho
            .decls_chirho
            .iter()
            .any(|decl_chirho| matches!(
                decl_chirho,
                DeclChirho::RoleAnnotationDeclChirho { type_name_chirho, .. }
                    if type_name_chirho.qualifier_chirho() == Some("M")
            ))
    );
}

#[test]
fn an_annotation_on_a_type_without_parameters_is_kept_chirho() {
    assert_eq!(
        annotations_chirho(
            "{-# LANGUAGE RoleAnnotations #-}\nmodule M where\ndata TagChirho = OneChirho | TwoChirho\ntype role TagChirho nominal\n"
        ),
        [("TagChirho".to_string(), words_chirho(&["nominal"]))]
    );
}
