// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor signatures retain their written metadata and module boundary.

use super::test_support_chirho::{parse_and_lower_chirho, type_shape_chirho};
use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DeclChirho, GadtConArgsChirho, GadtConPrefixChirho, GadtConSignatureChirho,
    StrictnessChirho,
};
use haskelujah_ast_chirho::ty_chirho::MultiplicityChirho;

fn first_gadt_chirho(source_chirho: &str) -> GadtConSignatureChirho {
    let module_chirho = parse_and_lower_chirho(source_chirho);
    let DeclChirho::DataDeclChirho {
        constructors_chirho,
        ..
    } = &module_chirho.decls_chirho[0]
    else {
        panic!("expected data declaration");
    };
    let ConDeclChirho::GadtChirho {
        signature_chirho, ..
    } = &constructors_chirho[0]
    else {
        panic!("expected GADT constructor");
    };
    signature_chirho.clone()
}

#[test]
fn forall_context_record_keeps_its_signature_and_following_declarations_chirho() {
    let source_chirho = "{-# LANGUAGE GADTs, TypeFamilies, ExplicitForAll #-}\nmodule Main where\ndata BoxChirho aChirho where\n  MkBoxChirho :: forall aChirho. (OpaqueChirho aChirho ~ Int) => { payloadChirho :: !(OpaqueChirho aChirho) } -> BoxChirho aChirho\nsentinelChirho = 17\nmain = print sentinelChirho\n";
    let module_chirho = parse_and_lower_chirho(source_chirho);
    assert_eq!(module_chirho.decls_chirho.len(), 3);
    let signature_chirho = first_gadt_chirho(source_chirho);
    assert!(signature_chirho.malformed_chirho.is_none());
    assert!(
        matches!(&signature_chirho.prefix_chirho[..], [GadtConPrefixChirho::ForallChirho { vars_chirho, .. }, GadtConPrefixChirho::ContextChirho { .. }] if vars_chirho.len() == 1)
    );
    let fields_chirho = signature_chirho
        .record_fields_chirho()
        .expect("named fields");
    assert_eq!(
        fields_chirho[0].names_chirho[0].text_chirho(),
        "payloadChirho"
    );
    assert_eq!(
        fields_chirho[0].strictness_chirho,
        StrictnessChirho::StrictChirho
    );
    assert_eq!(
        type_shape_chirho(&signature_chirho.result_ty_chirho),
        "(BoxChirho aChirho)"
    );
}

#[test]
fn grouped_record_names_share_one_type_before_the_field_separator_chirho() {
    for source_chirho in [
        "module Main where\ndata PairChirho = PairChirho { leftChirho, rightChirho :: Int, flagChirho :: Bool }\n",
        "{-# LANGUAGE GADTs #-}\nmodule Main where\ndata PairChirho where\n  PairChirho :: { leftChirho, rightChirho :: Int, flagChirho :: Bool } -> PairChirho\n",
    ] {
        let module_chirho = parse_and_lower_chirho(source_chirho);
        let DeclChirho::DataDeclChirho {
            constructors_chirho,
            ..
        } = &module_chirho.decls_chirho[0]
        else {
            panic!("data");
        };
        let fields_chirho = constructors_chirho[0]
            .record_fields_chirho()
            .expect("record fields");
        assert_eq!(fields_chirho.len(), 2);
        assert_eq!(
            fields_chirho[0]
                .names_chirho
                .iter()
                .map(|name_chirho| name_chirho.text_chirho())
                .collect::<Vec<_>>(),
            ["leftChirho", "rightChirho"]
        );
        assert_eq!(type_shape_chirho(&fields_chirho[0].ty_chirho), "Int");
        assert_eq!(type_shape_chirho(&fields_chirho[1].ty_chirho), "Bool");
    }
}

#[test]
fn positional_and_named_gadt_multiplicities_are_distinct_from_record_arrow_chirho() {
    let linear_chirho = first_gadt_chirho(
        "{-# LANGUAGE GADTs, LinearTypes #-}\nmodule Main where\ndata BoxChirho aChirho where\n  MkBoxChirho :: !aChirho %1 -> BoxChirho aChirho\n",
    );
    let fields_chirho = linear_chirho.positional_fields_chirho().collect::<Vec<_>>();
    assert_eq!(
        fields_chirho[0].strictness_chirho,
        StrictnessChirho::StrictChirho
    );
    assert_eq!(
        fields_chirho[0].multiplicity_chirho,
        Some(&MultiplicityChirho::OneChirho)
    );
    let record_chirho = first_gadt_chirho(
        "{-# LANGUAGE GADTs, LinearTypes #-}\nmodule Main where\ndata BoxChirho aChirho where\n  MkBoxChirho :: { payloadChirho %1 :: aChirho } -> BoxChirho aChirho\n",
    );
    assert_eq!(
        record_chirho.record_fields_chirho().unwrap()[0].multiplicity_chirho,
        Some(MultiplicityChirho::OneChirho)
    );
    let illegal_chirho = first_gadt_chirho(
        "{-# LANGUAGE GADTs, LinearTypes #-}\nmodule Main where\ndata BoxChirho aChirho where\n  MkBoxChirho :: { payloadChirho :: aChirho } %1 -> BoxChirho aChirho\n",
    );
    assert_eq!(
        illegal_chirho.malformed_chirho,
        Some(haskelujah_ast_chirho::decl_chirho::GadtConMalformedChirho::IllegalRecordArrowChirho)
    );
}

#[test]
fn balanced_forall_annotations_keep_binders_without_approximating_their_kinds_chirho() {
    for annotation_chirho in [
        "forall kChirho. kChirho -> Type",
        "[Type]",
        "[[Type]]",
        "Maybe kChirho",
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE GADTs, PolyKinds, RankNTypes, DataKinds #-}}\nmodule Main where\ndata BoxChirho where\n  MkBoxChirho :: forall (fChirho :: {annotation_chirho}). Int -> BoxChirho\n"
        );
        let signature_chirho = first_gadt_chirho(&source_chirho);
        assert!(
            signature_chirho.malformed_chirho.is_none(),
            "{annotation_chirho}"
        );
        assert!(
            matches!(&signature_chirho.prefix_chirho[0], GadtConPrefixChirho::ForallChirho { vars_chirho, .. } if vars_chirho.len() == 1 && vars_chirho[0].name_chirho.text_chirho() == "fChirho")
        );
        assert_eq!(signature_chirho.positional_fields_chirho().count(), 1);
    }
    let malformed_chirho = first_gadt_chirho(
        "{-# LANGUAGE GADTs, ExplicitForAll #-}\nmodule Main where\ndata BoxChirho where\n  MkBoxChirho :: forall (aChirho :: Type) Int -> BoxChirho\n",
    );
    assert!(malformed_chirho.malformed_chirho.is_some());
}

#[test]
fn strict_and_lazy_gadt_arguments_do_not_become_type_applications_chirho() {
    let signature_chirho = first_gadt_chirho(
        "{-# LANGUAGE GADTs #-}\nmodule Main where\ndata BoxChirho where\n  MkBoxChirho :: !Int -> Bool -> BoxChirho\n",
    );
    let GadtConArgsChirho::PositionalChirho(fields_chirho) = signature_chirho.args_chirho else {
        panic!("positional");
    };
    assert_eq!(fields_chirho.len(), 2);
    assert_eq!(
        fields_chirho[0].strictness_chirho,
        StrictnessChirho::StrictChirho
    );
    assert_eq!(
        fields_chirho[1].strictness_chirho,
        StrictnessChirho::LazyChirho
    );
    assert_eq!(type_shape_chirho(&fields_chirho[0].ty_chirho), "Int");
}

#[test]
fn retained_corpus_strict_fields_keep_their_real_arities_chirho() {
    let cases_chirho = [
        (
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T4444.hs"
            ),
            "ArrInt",
            2_usize,
            1_usize,
        ),
        (
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T4524.hs"
            ),
            "FP",
            2,
            2,
        ),
        (
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T4524.hs"
            ),
            "FlippedSeal",
            1,
            1,
        ),
    ];
    for (source_chirho, name_chirho, arity_chirho, strict_chirho) in cases_chirho {
        let module_chirho = parse_and_lower_chirho(source_chirho);
        let signature_chirho = module_chirho
            .decls_chirho
            .iter()
            .find_map(|decl_chirho| {
                if let DeclChirho::DataDeclChirho {
                    constructors_chirho,
                    ..
                } = decl_chirho
                {
                    constructors_chirho
                        .iter()
                        .find_map(|con_chirho| match con_chirho {
                            ConDeclChirho::GadtChirho {
                                name_chirho: con_name_chirho,
                                signature_chirho,
                                ..
                            } if con_name_chirho.text_chirho() == name_chirho => {
                                Some(signature_chirho)
                            }
                            _ => None,
                        })
                } else {
                    None
                }
            })
            .expect("retained corpus constructor");
        assert!(signature_chirho.malformed_chirho.is_none(), "{name_chirho}");
        assert_eq!(
            signature_chirho.positional_fields_chirho().count(),
            arity_chirho,
            "{name_chirho}"
        );
        assert_eq!(
            signature_chirho
                .positional_fields_chirho()
                .filter(
                    |field_chirho| field_chirho.strictness_chirho != StrictnessChirho::LazyChirho
                )
                .count(),
            strict_chirho,
            "{name_chirho}"
        );
    }
}

#[test]
fn higher_rank_forall_binders_keep_the_real_corpus_constructor_fields_chirho() {
    for source_chirho in [
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T18986a.hs"),
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T18986b.hs"),
    ] {
        let signature_chirho = first_gadt_chirho(source_chirho);
        assert!(signature_chirho.malformed_chirho.is_none());
        assert_eq!(signature_chirho.positional_fields_chirho().count(), 2);
    }
}

#[test]
fn unpack_annotations_stay_on_their_positional_or_named_gadt_field_chirho() {
    for source_chirho in [
        "{-# LANGUAGE GADTs #-}\nmodule Main where\ndata BoxChirho where\n  MkBoxChirho :: {-# UNPACK #-} !Int -> BoxChirho\n",
        "{-# LANGUAGE GADTs #-}\nmodule Main where\ndata BoxChirho where\n  MkBoxChirho :: { payloadChirho :: {-# UNPACK #-} !Int } -> BoxChirho\n",
    ] {
        let signature_chirho = first_gadt_chirho(source_chirho);
        assert_eq!(
            signature_chirho
                .positional_fields_chirho()
                .next()
                .unwrap()
                .strictness_chirho,
            StrictnessChirho::UnpackChirho
        );
    }
}
