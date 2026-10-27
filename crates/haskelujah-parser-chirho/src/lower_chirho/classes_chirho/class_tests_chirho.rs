// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering tests: Classes, their methods and type families.
//! Moved from `lower_chirho.rs`'s tests in row 641.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;

use super::test_support_chirho::{
    find_class_decl_chirho, find_class_method_chirho, parse_and_lower_chirho, type_shape_chirho,
};
use crate::test_fixtures_chirho::CONTRAVARIANT_REP_SOURCE_CHIRHO;

#[test]
fn lower_class_kind_annotation_does_not_collect_kind_vars_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE ConstraintKinds #-}\n{-# LANGUAGE PolyKinds #-}\n{-# LANGUAGE QuantifiedConstraints #-}\nmodule M where\nclass (forall a. p a) => Forall (p :: k -> Constraint)\nclass ForallF (p :: k2 -> Constraint) (f :: k1 -> k2)\nclass ForallV' p => ForallV_ (p :: k)\n",
    );

    let forall_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::ClassDeclChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "Forall"
            )
        })
        .expect("expected Forall class declaration");
    match forall_decl_chirho {
        DeclChirho::ClassDeclChirho {
            type_vars_chirho, ..
        } => {
            assert_eq!(type_vars_chirho.len(), 1);
            assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "p");
            assert!(type_vars_chirho[0].kind_annotation_chirho.is_some());
        }
        other_chirho => panic!("expected class declaration, got {:?}", other_chirho),
    }

    let forall_f_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::ClassDeclChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "ForallF"
            )
        })
        .expect("expected ForallF class declaration");
    match forall_f_decl_chirho {
        DeclChirho::ClassDeclChirho {
            type_vars_chirho, ..
        } => {
            assert_eq!(type_vars_chirho.len(), 2);
            assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "p");
            assert_eq!(type_vars_chirho[1].name_chirho.text_chirho(), "f");
            assert!(type_vars_chirho[0].kind_annotation_chirho.is_some());
            assert!(type_vars_chirho[1].kind_annotation_chirho.is_some());
        }
        other_chirho => panic!("expected class declaration, got {:?}", other_chirho),
    }

    let forall_v_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::ClassDeclChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "ForallV_"
            )
        })
        .expect("expected ForallV_ class declaration");
    match forall_v_decl_chirho {
        DeclChirho::ClassDeclChirho {
            type_vars_chirho, ..
        } => {
            assert_eq!(type_vars_chirho.len(), 1);
            assert_eq!(type_vars_chirho[0].name_chirho.text_chirho(), "p");
            assert!(type_vars_chirho[0].kind_annotation_chirho.is_some());
        }
        other_chirho => panic!("expected class declaration, got {:?}", other_chirho),
    }
}

#[test]
fn lower_default_signature_chirho() {
    let src_chirho = "\
module M where
class Describable a where
  describe :: a -> Int
  default describe :: a -> Int
  describe x = 42
";
    let module_chirho = parse_and_lower_chirho(src_chirho);
    match &module_chirho.decls_chirho[0] {
        DeclChirho::ClassDeclChirho { methods_chirho, .. } => {
            assert_eq!(methods_chirho.len(), 1);
            let method_chirho = &methods_chirho[0];
            assert_eq!(method_chirho.name_chirho.text_chirho(), "describe");
            assert!(
                method_chirho.default_chirho.is_some(),
                "default implementation should be present"
            );
            assert!(
                method_chirho.default_sig_chirho.is_some(),
                "default signature should be present"
            );
            let sig_text_chirho = method_chirho.default_sig_chirho.as_ref().unwrap();
            assert!(
                sig_text_chirho.contains("Int"),
                "default sig should contain 'Int': {}",
                sig_text_chirho
            );
        }
        other_chirho => panic!("expected ClassDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_class_no_default_signature_chirho() {
    let src_chirho = "\
module M where
class Describable a where
  describe :: a -> Int
  describe x = 42
";
    let module_chirho = parse_and_lower_chirho(src_chirho);
    match &module_chirho.decls_chirho[0] {
        DeclChirho::ClassDeclChirho { methods_chirho, .. } => {
            assert_eq!(methods_chirho.len(), 1);
            assert!(
                methods_chirho[0].default_sig_chirho.is_none(),
                "no default sig without 'default' keyword"
            );
        }
        other_chirho => panic!("expected ClassDecl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_open_type_family_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\ntype family F a\n");
    assert!(module_chirho.decls_chirho.iter().any(|d_chirho| {
        matches!(d_chirho, DeclChirho::TypeFamilyDeclChirho { name_chirho, equations_chirho, .. }
            if name_chirho.text_chirho() == "F" && equations_chirho.is_empty())
    }), "should have open type family F with no equations");
}

#[test]
fn lower_closed_type_family_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\ntype family F a where\n  F Int = Bool\n  F Char = Int\n",
    );
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| matches!(d_chirho, DeclChirho::TypeFamilyDeclChirho { .. }));
    assert!(decl_chirho.is_some(), "should have TypeFamilyDecl");
    match decl_chirho.unwrap() {
        DeclChirho::TypeFamilyDeclChirho {
            name_chirho,
            equations_chirho,
            ..
        } => {
            assert_eq!(name_chirho.text_chirho(), "F");
            assert_eq!(equations_chirho.len(), 2, "should have 2 equations");
        }
        _ => unreachable!(),
    }
}

#[test]
fn lower_closed_type_family_operator_rhs_does_not_leak_value_binds_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "{-# LANGUAGE DataKinds #-}\n{-# LANGUAGE TypeFamilies #-}\n{-# LANGUAGE TypeOperators #-}\nmodule M where\nimport Data.Type.Bool\nimport GHC.TypeNats\ntype family Min (m :: Nat) (n :: Nat) :: Nat where\n  Min m n = If (n <=? m) n m\ntype family Max (m :: Nat) (n :: Nat) :: Nat where\n  Max m n = If (n <=? m) m n\n",
    );
    let family_names_chirho: Vec<&str> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                equations_chirho,
                ..
            } if equations_chirho.len() == 1 => Some(name_chirho.text_chirho()),
            _ => None,
        })
        .collect();
    assert!(
        family_names_chirho.contains(&"Min") && family_names_chirho.contains(&"Max"),
        "expected Min and Max closed type families, got {:?}",
        module_chirho.decls_chirho
    );
    assert!(
        !module_chirho
            .decls_chirho
            .iter()
            .any(|decl_chirho| matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "Min" || name_chirho.text_chirho() == "Max"
            )),
        "closed type-family equations should not lower as value bindings: {:?}",
        module_chirho.decls_chirho
    );
}

#[test]
fn lower_type_family_with_kind_sig_chirho() {
    let module_chirho = parse_and_lower_chirho("module M where\ntype family G a :: Type\n");
    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| matches!(d_chirho, DeclChirho::TypeFamilyDeclChirho { .. }));
    assert!(decl_chirho.is_some(), "should have TypeFamilyDecl");
    match decl_chirho.unwrap() {
        DeclChirho::TypeFamilyDeclChirho {
            name_chirho,
            result_kind_chirho,
            ..
        } => {
            assert_eq!(name_chirho.text_chirho(), "G");
            assert!(
                result_kind_chirho.is_some(),
                "should have result kind annotation"
            );
        }
        _ => unreachable!(),
    }
}

#[test]
fn lower_class_method_type_preserves_assoc_family_application_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nclass Representable f where\n  type Rep f\n  tabulate :: (a -> Rep f) -> f a\n  index :: f a -> a -> Rep f\n",
    );
    let class_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| matches!(decl_chirho, DeclChirho::ClassDeclChirho { .. }))
        .expect("should lower class decl");
    match class_decl_chirho {
        DeclChirho::ClassDeclChirho {
            methods_chirho,
            associated_tfs_chirho,
            ..
        } => {
            assert_eq!(associated_tfs_chirho.len(), 1);
            assert_eq!(associated_tfs_chirho[0].name_chirho.text_chirho(), "Rep");
            assert_eq!(methods_chirho.len(), 2);
            let tabulate_ty_chirho =
                &find_class_method_chirho(methods_chirho, "tabulate").ty_chirho;
            let index_ty_chirho = &find_class_method_chirho(methods_chirho, "index").ty_chirho;
            let tabulate_ty_debug_chirho = format!("{:?}", tabulate_ty_chirho);
            assert!(
                tabulate_ty_debug_chirho.contains("Rep") && tabulate_ty_debug_chirho.contains("f"),
                "class method type should preserve `Rep f`, got {:?}",
                tabulate_ty_chirho
            );
            assert_eq!(
                type_shape_chirho(tabulate_ty_chirho),
                "((a -> (Rep f)) -> (f a))",
                "tabulate should lower with a function argument returning Rep f"
            );
            assert_eq!(
                type_shape_chirho(index_ty_chirho),
                "((f a) -> (a -> (Rep f)))",
                "index should lower as f a -> a -> Rep f"
            );
        }
        other_chirho => panic!("expected class decl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_assoc_type_family_placeholder_rhs_does_not_become_default_chirho() {
    let module_chirho = parse_and_lower_chirho(
        "module M where\nclass FrozenGen f m where\n  type MutableGen f m = (g :: Type)\n  thawGen :: f -> m (MutableGen f m)\n",
    );
    let class_decl_chirho = find_class_decl_chirho(&module_chirho, "FrozenGen");
    match class_decl_chirho {
        DeclChirho::ClassDeclChirho {
            associated_tfs_chirho,
            ..
        } => {
            assert_eq!(associated_tfs_chirho.len(), 1);
            assert_eq!(
                associated_tfs_chirho[0].name_chirho.text_chirho(),
                "MutableGen"
            );
            assert!(
                associated_tfs_chirho[0].default_rhs_chirho.is_none(),
                "placeholder associated type family rhs should not lower as a default equation: {:?}",
                associated_tfs_chirho[0].default_rhs_chirho
            );
        }
        other_chirho => panic!("expected class decl, got {:?}", other_chirho),
    }
}

#[test]
fn lower_real_contravariant_representable_class_matches_expected_shapes_chirho() {
    let module_chirho = parse_and_lower_chirho(CONTRAVARIANT_REP_SOURCE_CHIRHO);
    let class_decl_chirho = find_class_decl_chirho(&module_chirho, "Representable");
    match class_decl_chirho {
        DeclChirho::ClassDeclChirho {
            context_chirho,
            methods_chirho,
            associated_tfs_chirho,
            ..
        } => {
            assert_eq!(context_chirho.len(), 1);
            assert_eq!(associated_tfs_chirho.len(), 1);
            assert_eq!(associated_tfs_chirho[0].name_chirho.text_chirho(), "Rep");
            assert_eq!(associated_tfs_chirho[0].type_vars_chirho.len(), 1);
            assert_eq!(
                associated_tfs_chirho[0].type_vars_chirho[0].text_chirho(),
                "f"
            );
            assert_eq!(methods_chirho.len(), 3);

            let tabulate_ty_chirho =
                &find_class_method_chirho(methods_chirho, "tabulate").ty_chirho;
            let index_ty_chirho = &find_class_method_chirho(methods_chirho, "index").ty_chirho;
            let contramap_with_rep_ty_chirho =
                &find_class_method_chirho(methods_chirho, "contramapWithRep").ty_chirho;

            assert_eq!(
                type_shape_chirho(tabulate_ty_chirho),
                "((a -> (Rep f)) -> (f a))",
                "real tabulate signature should lower identically to the synthetic shape"
            );
            assert_eq!(
                type_shape_chirho(index_ty_chirho),
                "((f a) -> (a -> (Rep f)))",
                "real index signature should lower identically to the synthetic shape"
            );
            assert_eq!(
                type_shape_chirho(contramap_with_rep_ty_chirho),
                "((b -> ((Either a) (Rep f))) -> ((f a) -> (f b)))",
                "real contramapWithRep signature should preserve Either a (Rep f)"
            );
        }
        other_chirho => panic!("expected class decl, got {:?}", other_chirho),
    }
}
