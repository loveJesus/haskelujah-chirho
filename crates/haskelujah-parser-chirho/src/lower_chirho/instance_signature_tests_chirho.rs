// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! An instance body keeps its method signatures (InstanceSigs) beside the
//! method equations, in written order: the instance-method check reads the
//! signature to scope its binders (T5676), and lowering used to drop it.
//! Instances without signatures, and associated type instances, lower as before.
//! workflow: language-features-chirho/instance-obligations-chirho

use haskelujah_ast_chirho::decl_chirho::{AssocTfInstanceChirho, DeclChirho};
use haskelujah_ast_chirho::expr_chirho::LocalBindChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use crate::cst_parser_chirho::parse_to_cst_chirho;

fn lower_chirho(source_chirho: &str) -> ModuleChirho {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(source_chirho, file_chirho);
    lower_module_chirho(&cst_chirho, file_chirho)
}

/// The methods and associated type instances of the module's only instance.
fn instance_body_chirho(
    module_chirho: &ModuleChirho,
) -> (&[LocalBindChirho], &[AssocTfInstanceChirho]) {
    module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::InstanceDeclChirho {
                methods_chirho,
                assoc_tf_instances_chirho,
                ..
            } => Some((
                methods_chirho.as_slice(),
                assoc_tf_instances_chirho.as_slice(),
            )),
            _ => None,
        })
        .expect("an instance declaration")
}

#[test]
fn an_instance_signature_stays_beside_its_equations_chirho() {
    let module_chirho = lower_chirho(
        "{-# LANGUAGE InstanceSigs, ScopedTypeVariables #-}\nmodule M where\nclass CChirho a where\n  fooChirho :: b -> a -> [b]\ninstance CChirho Int where\n  fooChirho :: forall b. b -> Int -> [b]\n  fooChirho x _ = [x]\n",
    );
    let (methods_chirho, _families_chirho) = instance_body_chirho(&module_chirho);
    let [
        LocalBindChirho::TypeSigChirho {
            name_chirho: signed_chirho,
            ty_chirho,
            ..
        },
        LocalBindChirho::FunBindChirho {
            name_chirho: bound_chirho,
            ..
        },
    ] = methods_chirho
    else {
        panic!("expected the signature, then its equations: {methods_chirho:?}");
    };
    assert_eq!(signed_chirho.text_chirho(), "fooChirho");
    assert_eq!(bound_chirho.text_chirho(), "fooChirho");
    // The written `forall` survives: its binder is what scopes over the body.
    let TypeChirho::ForallChirho { vars_chirho, .. } = ty_chirho else {
        panic!("expected the written forall: {ty_chirho:?}");
    };
    let binders_chirho: Vec<&str> = vars_chirho
        .iter()
        .map(|var_chirho| var_chirho.text_chirho())
        .collect();
    assert_eq!(binders_chirho, ["b"]);
}

#[test]
fn instances_without_signatures_lower_as_before_chirho() {
    // A signature-free body is its equations only, and an associated type
    // instance is collected apart from the methods.
    let module_chirho = lower_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule M where\nclass ContainerChirho f where\n  type ElemChirho f\n  emptyChirho :: f\ninstance ContainerChirho [a] where\n  type ElemChirho [a] = a\n  emptyChirho = []\n",
    );
    let (methods_chirho, families_chirho) = instance_body_chirho(&module_chirho);
    let [LocalBindChirho::FunBindChirho { name_chirho, .. }] = methods_chirho else {
        panic!("expected the one equation group: {methods_chirho:?}");
    };
    assert_eq!(name_chirho.text_chirho(), "emptyChirho");
    let [family_chirho] = families_chirho else {
        panic!("expected one associated type instance: {families_chirho:?}");
    };
    assert_eq!(family_chirho.family_name_chirho.text_chirho(), "ElemChirho");
}
