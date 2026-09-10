// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Arrow annotations must not become another value argument or consume a declaration.
use super::{DeclChirho, FileIdChirho, TypeChirho, lower_module_chirho};
use crate::cst_parser_chirho::parse_to_cst_chirho;
use haskelujah_ast_chirho::ty_chirho::MultiplicityChirho;

#[test]
fn arrow_multiplicity_is_separate_from_argument_and_result_types_chirho() {
    for (written_chirho, linear_chirho) in [
        ("1", true),
        ("'One", true),
        ("('One)", true),
        ("'Many", false),
    ] {
        let source_chirho = format!(
            "module ArrowChirho where\ntype ArrowChirho = Int %{written_chirho} -> Int\ncanaryChirho :: MissingTypeChirho\ncanaryChirho = ()\n"
        );
        let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
        let module_chirho = lower_module_chirho(
            &parse_to_cst_chirho(&source_chirho, file_chirho),
            file_chirho,
        );
        let DeclChirho::TypeAliasDeclChirho { rhs_chirho, .. } = &module_chirho.decls_chirho[0]
        else {
            panic!("{module_chirho:?}")
        };
        let TypeChirho::FunChirho {
            arg_chirho,
            mult_chirho,
            result_chirho,
            ..
        } = rhs_chirho
        else {
            panic!("{rhs_chirho:?}")
        };
        assert!(
            matches!(arg_chirho.as_ref(), TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Int")
        );
        assert!(
            matches!(result_chirho.as_ref(), TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Int"),
            "{written_chirho}: {rhs_chirho:?}"
        );
        let annotation_chirho = mult_chirho.as_ref().expect("multiplicity retained");
        assert_eq!(annotation_chirho.is_explicit_one_chirho(), linear_chirho);
        if written_chirho != "1" {
            let MultiplicityChirho::ExpressionChirho(expression_chirho) = annotation_chirho else {
                panic!("explicit syntax and span were erased: {annotation_chirho:?}")
            };
            let span_chirho = expression_chirho.span_chirho();
            assert_eq!(
                &source_chirho[span_chirho.start_chirho().as_usize_chirho()
                    ..span_chirho.end_chirho().as_usize_chirho()],
                written_chirho,
            );
        }
        assert!(module_chirho.decls_chirho.iter().any(|declaration_chirho| {
            matches!(declaration_chirho, DeclChirho::TypeSigChirho { ty_chirho: TypeChirho::ConChirho(name_chirho), .. }
                if name_chirho.text_chirho() == "MissingTypeChirho")
        }));
    }
}
