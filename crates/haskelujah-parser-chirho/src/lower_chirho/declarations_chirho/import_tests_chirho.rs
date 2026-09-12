// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn source_import_mode_survives_real_pragma_tokens_only_chirho() {
    for (import_chirho, source_expected_chirho) in [
        ("import {-# SOURCE #-} qualified AChirho as QChirho", true),
        ("import\n  {-# SOURCE #-}\n  AChirho (TChirho)", true),
        ("import {- ordinary SOURCE comment -} AChirho", false),
        ("import {- nested {-# SOURCE #-} comment -} AChirho", false),
        ("import AChirho", false),
    ] {
        let source_chirho =
            format!("module ConsumerChirho where\n{import_chirho}\nvalueChirho = ()\n");
        let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
        let green_chirho =
            crate::cst_parser_chirho::ParserChirho::new_chirho(&source_chirho, file_chirho)
                .parse_chirho();
        let module_chirho = lower_module_chirho(&green_chirho, file_chirho);
        assert_eq!(module_chirho.imports_chirho.len(), 1, "{source_chirho}");
        assert_eq!(
            module_chirho.imports_chirho[0].source_chirho, source_expected_chirho,
            "{source_chirho}"
        );
        assert_eq!(
            module_chirho.imports_chirho[0]
                .module_chirho
                .full_name_chirho(),
            "AChirho"
        );
        assert_eq!(
            module_chirho.decls_chirho.len(),
            1,
            "following binding must survive"
        );
    }
}
