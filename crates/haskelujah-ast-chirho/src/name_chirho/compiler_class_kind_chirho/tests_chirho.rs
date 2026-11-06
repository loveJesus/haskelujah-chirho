// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn transformer_parameter_has_a_function_domain_and_two_outer_results_chirho() {
    let schema_chirho = CompilerClassChirho::MonadTransChirho.schema_chirho();
    let [parameter_chirho] = schema_chirho.binders_chirho else {
        panic!("one class parameter");
    };
    assert_eq!(
        parameter_chirho.kind_chirho,
        &CompilerClassKindChirho::ArrowChirho(&CONSTRUCTOR_CHIRHO, &CONSTRUCTOR_CHIRHO)
    );
    assert_eq!(schema_chirho.single_parameter_eta_arity_chirho(), Some(2));
    assert_eq!(schema_chirho.visible_parameter_count_chirho(), 1);
    assert_eq!(
        schema_chirho.closed_parameter_kind_chirho(0),
        Some(parameter_chirho.kind_chirho)
    );
    assert_eq!(schema_chirho.closed_parameter_kind_chirho(1), None);
    for class_chirho in [
        CompilerClassChirho::AlternativeChirho,
        CompilerClassChirho::MonadFixChirho,
        CompilerClassChirho::MonadIoChirho,
    ] {
        assert_eq!(
            class_chirho
                .schema_chirho()
                .single_parameter_eta_arity_chirho(),
            Some(1)
        );
    }
}

#[test]
fn polymorphic_classes_retain_their_shared_inferred_kind_slot_chirho() {
    for (class_chirho, parameter_count_chirho) in [
        (CompilerClassChirho::TypeableChirho, 1),
        (CompilerClassChirho::CoercibleChirho, 2),
    ] {
        let schema_chirho = class_chirho.schema_chirho();
        assert_eq!(
            schema_chirho.binders_chirho.len(),
            parameter_count_chirho + 1
        );
        assert_eq!(schema_chirho.binders_chirho[0], HIDDEN_KIND_CHIRHO);
        assert_eq!(
            schema_chirho.visible_parameter_count_chirho(),
            parameter_count_chirho
        );
        assert_eq!(schema_chirho.closed_parameter_kind_chirho(0), None);
        for parameter_chirho in &schema_chirho.binders_chirho[1..] {
            assert_eq!(parameter_chirho.kind_chirho, &KIND_SLOT_CHIRHO);
            assert_eq!(
                parameter_chirho.visibility_chirho,
                TyVarVisibilityChirho::VisibleChirho
            );
        }
        assert_eq!(schema_chirho.single_parameter_eta_arity_chirho(), None);
    }
}

#[test]
fn literal_witness_kinds_are_not_approximated_as_type_chirho() {
    for (class_chirho, constructor_chirho) in [
        (
            CompilerClassChirho::KnownNatChirho,
            CompilerClassGroundKindChirho::NAT_CHIRHO,
        ),
        (
            CompilerClassChirho::KnownSymbolChirho,
            CompilerClassGroundKindChirho::SYMBOL_CHIRHO,
        ),
        (
            CompilerClassChirho::KnownCharChirho,
            CompilerClassGroundKindChirho::CHAR_CHIRHO,
        ),
    ] {
        let schema_chirho = class_chirho.schema_chirho();
        assert_eq!(
            schema_chirho.binders_chirho[0].kind_chirho,
            &atom_chirho(constructor_chirho)
        );
        assert_eq!(schema_chirho.single_parameter_eta_arity_chirho(), None);
    }
    assert_eq!(
        CompilerClassChirho::HasFieldChirho
            .schema_chirho()
            .coverage_chirho,
        CompilerClassKindCoverageChirho::LiftedSymbolFieldsChirho
    );
    assert_eq!(
        CompilerClassChirho::HasFieldChirho
            .schema_chirho()
            .single_parameter_eta_arity_chirho(),
        None
    );
}
