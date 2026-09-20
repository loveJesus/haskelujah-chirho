// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn deriving_retains_class_arguments_and_nested_syntax_chirho() {
    let source_chirho = "module MChirho where\nnewtype TChirho aChirho bChirho = MkTChirho (aChirho, bChirho) deriving (Eq, CChirho aChirho, DChirho (Either Int bChirho))\nafterChirho = True\n";
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(source_chirho, file_chirho);
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    let DeclChirho::NewtypeDeclChirho {
        deriving_chirho, ..
    } = &module_chirho.decls_chirho[0]
    else {
        panic!("newtype missing");
    };
    let heads_chirho: Vec<_> = deriving_chirho
        .iter()
        .map(|application_chirho| {
            let (name_chirho, arguments_chirho) = application_chirho
                .constructor_application_chirho()
                .expect("class application");
            (name_chirho.text_chirho(), arguments_chirho.len())
        })
        .collect();
    assert_eq!(heads_chirho, [("Eq", 0), ("CChirho", 1), ("DChirho", 1)]);
    let (_, arguments_chirho) = deriving_chirho[2].constructor_application_chirho().unwrap();
    let (head_chirho, nested_chirho) = arguments_chirho[0]
        .constructor_application_chirho()
        .unwrap();
    assert_eq!(head_chirho.text_chirho(), "Either");
    assert_eq!(nested_chirho.len(), 2);
    assert!(module_chirho.decls_chirho.iter().any(|declaration_chirho| matches!(declaration_chirho,
        DeclChirho::FunBindChirho { name_chirho, .. } if name_chirho.text_chirho() == "afterChirho")));
}

#[test]
fn empty_deriving_retains_no_class_chirho() {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = crate::cst_parser_chirho::parse_to_cst_chirho(
        "data TChirho = TChirho deriving ()",
        file_chirho,
    );
    let module_chirho = lower_module_chirho(&cst_chirho, file_chirho);
    let DeclChirho::DataDeclChirho {
        deriving_chirho, ..
    } = &module_chirho.decls_chirho[0]
    else {
        panic!("data missing");
    };
    assert!(deriving_chirho.is_empty());
}
