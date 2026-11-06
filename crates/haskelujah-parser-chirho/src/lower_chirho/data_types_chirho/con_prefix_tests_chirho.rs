// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: an H98 constructor's `forall vs. ctx =>` reaches its carrier (row 694,
//! #31811). The first data declaration is root's measured `ConstructorContextRoleChirho`'s.

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, ConPrefixChirho, DeclChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};

use super::test_support_chirho::parse_and_lower_chirho;

/// The prefix of the first constructor of the module's only data declaration.
fn first_prefix_chirho(source_chirho: &str) -> ConPrefixChirho {
    let module_chirho = parse_and_lower_chirho(source_chirho);
    match &module_chirho.decls_chirho[0] {
        DeclChirho::DataDeclChirho {
            constructors_chirho,
            ..
        } => match &constructors_chirho[0] {
            ConDeclChirho::OrdinaryChirho { prefix_chirho, .. }
            | ConDeclChirho::RecordChirho { prefix_chirho, .. } => prefix_chirho.clone(),
            other_chirho => panic!("expected an H98 constructor, got {other_chirho:?}"),
        },
        other_chirho => panic!("expected a data declaration, got {other_chirho:?}"),
    }
}

/// Each constraint's class and the variables its arguments name directly.
fn constraint_shapes_chirho(prefix_chirho: &ConPrefixChirho) -> Vec<(String, Vec<String>)> {
    prefix_chirho
        .context_chirho
        .iter()
        .map(|constraint_chirho| match constraint_chirho {
            ConstraintChirho::ClassChirho {
                class_chirho,
                args_chirho,
                ..
            } => (
                class_chirho.text_chirho().to_string(),
                args_chirho
                    .iter()
                    .filter_map(|argument_chirho| match argument_chirho {
                        TypeChirho::VarChirho(name_chirho) => {
                            Some(name_chirho.text_chirho().to_string())
                        }
                        _ => None,
                    })
                    .collect(),
            ),
            other_chirho => panic!("expected a class constraint, got {other_chirho:?}"),
        })
        .collect()
}

#[test]
fn an_empty_forall_keeps_its_equality_context_chirho() {
    let prefix_chirho = first_prefix_chirho(
        "module M where\ndata HoldChirho aChirho = forall . (aChirho ~ Int) => HoldChirho Int\n",
    );
    assert!(!prefix_chirho.unread_chirho);
    assert!(prefix_chirho.existential_vars_chirho.is_empty());
    assert_eq!(
        constraint_shapes_chirho(&prefix_chirho),
        vec![("~".to_string(), vec!["aChirho".to_string()])]
    );
}

#[test]
fn binders_and_a_class_context_reach_the_carrier_chirho() {
    let prefix_chirho = first_prefix_chirho(
        "module M where\ndata ShownChirho = forall bChirho cChirho. (Show bChirho, Eq cChirho) => ShownChirho bChirho cChirho\n",
    );
    assert!(!prefix_chirho.unread_chirho);
    let binders_chirho: Vec<&str> = prefix_chirho
        .existential_vars_chirho
        .iter()
        .map(|name_chirho| name_chirho.text_chirho())
        .collect();
    assert_eq!(binders_chirho, vec!["bChirho", "cChirho"]);
    assert_eq!(
        constraint_shapes_chirho(&prefix_chirho),
        vec![
            ("Show".to_string(), vec!["bChirho".to_string()]),
            ("Eq".to_string(), vec!["cChirho".to_string()]),
        ]
    );
}

#[test]
fn a_record_constructor_keeps_its_prefix_chirho() {
    let prefix_chirho = first_prefix_chirho(
        "module M where\ndata BoxChirho = forall bChirho. Show bChirho => BoxChirho { boxedChirho :: bChirho }\n",
    );
    assert!(!prefix_chirho.unread_chirho);
    assert_eq!(prefix_chirho.existential_vars_chirho.len(), 1);
    assert_eq!(
        constraint_shapes_chirho(&prefix_chirho),
        vec![("Show".to_string(), vec!["bChirho".to_string()])]
    );
}

#[test]
fn a_forall_without_context_binds_only_chirho() {
    let prefix_chirho =
        first_prefix_chirho("module M where\ndata AnyChirho = forall bChirho. AnyChirho bChirho\n");
    assert!(!prefix_chirho.unread_chirho);
    assert_eq!(prefix_chirho.existential_vars_chirho.len(), 1);
    assert!(prefix_chirho.context_chirho.is_empty());
}

#[test]
fn a_kinded_binder_leaves_the_prefix_unread_chirho() {
    let prefix_chirho = first_prefix_chirho(
        "module M where\ndata AnyChirho = forall (bChirho :: Type). Show bChirho => AnyChirho bChirho\n",
    );
    assert!(prefix_chirho.unread_chirho);
}

#[test]
fn a_constructor_without_forall_has_no_prefix_chirho() {
    let prefix_chirho =
        first_prefix_chirho("module M where\ndata PairChirho aChirho = PairChirho aChirho Int\n");
    assert!(prefix_chirho.is_empty_chirho());
}

#[test]
fn a_gadt_record_constructor_is_unread_chirho() {
    let prefix_chirho = first_prefix_chirho(
        "module M where\ndata TagChirho aChirho where\n  TagChirho :: { tagChirho :: Int } -> TagChirho Int\n",
    );
    assert!(prefix_chirho.unread_chirho);
}
