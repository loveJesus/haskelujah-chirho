// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Role annotations reach their data or newtype parameters (row 694): GND's method coercion
//! proof reads them, so `type role HoldChirho nominal` must not lower to a dummy signature.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;

use super::test_support_chirho::parse_and_lower_chirho;

/// Each declared type's parameter roles as written, `-` where none is.
fn roles_chirho(source_chirho: &str) -> Vec<(String, Vec<String>)> {
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
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                ..
            }
            | DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                ..
            } => Some((
                name_chirho.text_chirho().to_string(),
                type_vars_chirho
                    .iter()
                    .map(|binder_chirho| {
                        binder_chirho
                            .role_annotation_chirho
                            .as_ref()
                            .map_or("-".to_string(), |role_chirho| {
                                role_chirho.text_chirho().to_string()
                            })
                    })
                    .collect(),
            )),
            _ => None,
        })
        .collect()
}

#[test]
fn a_role_annotation_reaches_its_parameters_chirho() {
    assert_eq!(
        roles_chirho(
            "{-# LANGUAGE RoleAnnotations #-}\nmodule M where\ntype role HoldChirho nominal\ndata HoldChirho aChirho = HoldChirho aChirho\nnewtype PairChirho aChirho bChirho = PairChirho (aChirho, bChirho)\ntype role PairChirho _ phantom\n"
        ),
        [
            ("HoldChirho".to_string(), vec!["nominal".to_string()]),
            (
                "PairChirho".to_string(),
                vec!["_".to_string(), "phantom".to_string()]
            ),
        ]
    );
}

#[test]
fn an_unreadable_role_annotation_reads_as_nominal_chirho() {
    // GHC refuses both: one role too few, and a word that names no role.
    assert_eq!(
        roles_chirho(
            "module M where\ntype role PairChirho phantom\ndata PairChirho aChirho bChirho = PairChirho aChirho bChirho\ntype role BoxChirho shared\ndata BoxChirho aChirho = BoxChirho aChirho\n"
        ),
        [
            (
                "PairChirho".to_string(),
                vec!["nominal".to_string(), "nominal".to_string()]
            ),
            ("BoxChirho".to_string(), vec!["nominal".to_string()]),
        ]
    );
}
