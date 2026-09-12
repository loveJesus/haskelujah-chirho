// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Reified binders expose their written kind to TH consumers.
use super::*;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_span_chirho::SpanChirho;

#[test]
fn reified_kind_binders_retain_nominal_names_variables_and_applications_chirho() {
    let variable_chirho = AstKindChirho::VarChirho("kChirho".into());
    let nominal_chirho = AstKindChirho::ConChirho(NameChirho::RawChirho(
        RawNameChirho::qualified_chirho("LibraryChirho", "ApplyChirho", SpanChirho::DUMMY_CHIRHO),
    ));
    for (kind_chirho, expected_chirho) in [
        (AstKindChirho::StarChirho, ThTypeChirho::StarTChirho),
        (
            AstKindChirho::ConstraintChirho,
            ThTypeChirho::ConstraintTChirho,
        ),
        (
            variable_chirho.clone(),
            ThTypeChirho::VarTChirho(ThNameChirho::mk_name_chirho("kChirho")),
        ),
        (
            nominal_chirho.clone(),
            ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho("LibraryChirho.ApplyChirho")),
        ),
        (
            AstKindChirho::KindAnnotChirho {
                type_chirho: Box::new(variable_chirho.clone()),
                kind_chirho: Box::new(AstKindChirho::StarChirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            ThTypeChirho::SigTChirho(
                Box::new(ThTypeChirho::VarTChirho(ThNameChirho::mk_name_chirho(
                    "kChirho",
                ))),
                Box::new(ThTypeChirho::StarTChirho),
            ),
        ),
        (
            AstKindChirho::AppChirho(Box::new(nominal_chirho), Box::new(variable_chirho.clone())),
            ThTypeChirho::AppTChirho(
                Box::new(ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
                    "LibraryChirho.ApplyChirho",
                ))),
                Box::new(ThTypeChirho::VarTChirho(ThNameChirho::mk_name_chirho(
                    "kChirho",
                ))),
            ),
        ),
        (
            AstKindChirho::ArrowChirho(
                Box::new(variable_chirho),
                Box::new(AstKindChirho::StarChirho),
            ),
            ThTypeChirho::AppTChirho(
                Box::new(ThTypeChirho::AppTChirho(
                    Box::new(ThTypeChirho::ArrowTChirho),
                    Box::new(ThTypeChirho::VarTChirho(ThNameChirho::mk_name_chirho(
                        "kChirho",
                    ))),
                )),
                Box::new(ThTypeChirho::StarTChirho),
            ),
        ),
    ] {
        let binder_chirho = TyVarChirho::annotated_chirho(
            NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                "aChirho",
                SpanChirho::DUMMY_CHIRHO,
            )),
            kind_chirho,
        );
        let ThTyVarBndrChirho::KindedTVChirho(name_chirho, recovered_chirho) =
            ast_tyvar_to_th_chirho(&binder_chirho)
        else {
            panic!("reification must not erase the kind annotation")
        };
        assert_eq!(name_chirho.occ_chirho, "aChirho");
        assert_eq!(*recovered_chirho, expected_chirho);
    }
}

#[test]
fn type_ascription_conversion_and_reification_preserve_both_children_chirho() {
    let classifier_chirho = ThTypeChirho::SigTChirho(
        Box::new(ThTypeChirho::VarTChirho(ThNameChirho::mk_name_chirho(
            "keyChirho",
        ))),
        Box::new(ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
            "Type",
        ))),
    );
    for kind_chirho in [
        ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho("Type")),
        classifier_chirho,
    ] {
        let quoted_chirho = ThTypeChirho::SigTChirho(
            Box::new(ThTypeChirho::VarTChirho(ThNameChirho::mk_name_chirho(
                "valueChirho",
            ))),
            Box::new(kind_chirho),
        );
        let converted_chirho = crate::convert_chirho::th_type_to_ast_chirho(&quoted_chirho);
        assert_eq!(ast_type_to_th_chirho(&converted_chirho), quoted_chirho);
    }
}
