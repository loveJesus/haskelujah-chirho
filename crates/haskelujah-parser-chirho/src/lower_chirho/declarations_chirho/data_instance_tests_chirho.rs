// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn data_instances_preserve_gadt_signatures_and_following_declarations_chirho() {
    let source_chirho = "{-# LANGUAGE GADTs, PolyKinds, TypeFamilies #-}\nmodule InstancesChirho where\ndata family SingChirho :: kChirho -> Type\ndata instance SingChirho :: Bool -> Type where\n  FalseChirho :: SingChirho False\n  TrueChirho :: SingChirho True\ndata instance SingChirho :: forall aChirho. Maybe aChirho -> Type where\n  NothingChirho :: SingChirho Nothing\ncanaryChirho = True\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let module_chirho = lower_module_chirho(
        &crate::cst_parser_chirho::parse_to_cst_chirho(source_chirho, file_chirho),
        file_chirho,
    );
    let instances_chirho: Vec<_> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| {
            if let DeclChirho::DataFamilyInstanceDeclChirho {
                head_chirho,
                constructors_chirho,
                ..
            } = decl_chirho
            {
                Some((head_chirho, constructors_chirho))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(instances_chirho.len(), 2, "{module_chirho:#?}");
    assert_eq!(instances_chirho[0].1.len(), 2);
    assert_eq!(instances_chirho[1].1.len(), 1);
    assert!(
        matches!(instances_chirho[1].0, TypeChirho::KindAnnotChirho { kind_chirho, .. } if matches!(kind_chirho.as_ref(), TypeChirho::ForallChirho { .. })),
        "{module_chirho:#?}"
    );
    assert!(instances_chirho.iter().all(|(_, constructors_chirho)| {
        constructors_chirho
            .iter()
            .all(|con_chirho| matches!(con_chirho, ConDeclChirho::GadtChirho { .. }))
    }));
    assert!(
        matches!(module_chirho.decls_chirho.last(), Some(DeclChirho::FunBindChirho { name_chirho, .. }) if name_chirho.text_chirho() == "canaryChirho")
    );
}

#[test]
fn data_instances_retain_application_and_newtype_form_chirho() {
    let source_chirho = "module InstancesChirho where\ndata family FamilyChirho aChirho\ndata instance FamilyChirho [aChirho] = ListChirho aChirho\nnewtype instance FamilyChirho Int = IntChirho Int\ncanaryChirho = True\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let module_chirho = lower_module_chirho(
        &crate::cst_parser_chirho::parse_to_cst_chirho(source_chirho, file_chirho),
        file_chirho,
    );
    let DeclChirho::DataFamilyInstanceDeclChirho {
        head_chirho,
        newtype_chirho,
        constructors_chirho,
        ..
    } = &module_chirho.decls_chirho[1]
    else {
        panic!("{module_chirho:#?}")
    };
    let (name_chirho, arguments_chirho) = head_chirho
        .constructor_application_chirho()
        .expect("family application");
    assert_eq!(name_chirho.text_chirho(), "FamilyChirho");
    assert!(matches!(
        arguments_chirho.as_slice(),
        [TypeChirho::ListChirho { .. }]
    ));
    assert!(!newtype_chirho);
    assert_eq!(constructors_chirho.len(), 1);
    assert!(matches!(
        &module_chirho.decls_chirho[2],
        DeclChirho::DataFamilyInstanceDeclChirho {
            newtype_chirho: true,
            ..
        }
    ));
}
