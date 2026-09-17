// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn associated_star_kind_is_syntax_not_multiplication_with_fabricated_operands_chirho() {
    let source_chirho = "{-# LANGUAGE TypeFamilies #-}\nmodule Test where\nclass MyClass a where\n  type MyFamily a :: *\n  myMethod :: a -> Int\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let green_chirho =
        crate::cst_parser_chirho::ParserChirho::new_chirho(source_chirho, file_chirho)
            .parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_chirho);
    let DeclChirho::ClassDeclChirho {
        associated_tfs_chirho,
        methods_chirho,
        ..
    } = &module_chirho.decls_chirho[0]
    else {
        panic!("class declaration must survive");
    };
    assert!(
        matches!(associated_tfs_chirho[0].result_chirho.kind_sig_chirho.as_ref(), Some(DeclKindSigChirho::ResultChirho(TypeChirho::ConChirho(name_chirho))) if name_chirho.full_name_chirho() == "*"),
        "{:?}",
        associated_tfs_chirho[0].result_chirho
    );
    assert_eq!(methods_chirho[0].name_chirho.text_chirho(), "myMethod");
}

#[test]
fn explicit_empty_class_context_is_not_an_abstract_promise_chirho() {
    for (head_chirho, expected_chirho) in [
        ("class KChirho aChirho", false),
        ("class KChirho aChirho where", false),
        ("class () => KChirho aChirho", true),
        ("class () => KChirho aChirho where", true),
    ] {
        let source_chirho =
            format!("module ClassChirho where\n{head_chirho}\nfollowingChirho = ()\n");
        let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
        let green_chirho =
            crate::cst_parser_chirho::ParserChirho::new_chirho(&source_chirho, file_chirho)
                .parse_chirho();
        let module_chirho = lower_module_chirho(&green_chirho, file_chirho);
        let declaration_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|decl_chirho| matches!(decl_chirho, DeclChirho::ClassDeclChirho { .. }))
            .unwrap();
        let DeclChirho::ClassDeclChirho {
            context_written_chirho,
            context_chirho,
            ..
        } = declaration_chirho
        else {
            unreachable!()
        };
        assert_eq!(*context_written_chirho, expected_chirho, "{source_chirho}");
        assert!(context_chirho.is_empty(), "{source_chirho}");
        assert!(module_chirho.decls_chirho.iter().any(|decl_chirho| matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. } if name_chirho.text_chirho() == "followingChirho")), "{source_chirho}");
    }
}
