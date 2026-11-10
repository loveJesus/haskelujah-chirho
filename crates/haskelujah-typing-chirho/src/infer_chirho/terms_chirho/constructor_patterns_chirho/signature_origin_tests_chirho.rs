// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn an_unstamped_pattern_annotation_delegates_without_resource_failure_chirho() {
    for binding_chirho in [false, true] {
        let mut context_chirho = InferCtxChirho::new_chirho();
        context_chirho.adopt_elaboration_state_chirho(
            crate::kind_chirho::elaboration_chirho::ElaborationStateChirho::new_chirho().unwrap(),
        );
        context_chirho.scoped_type_variables_chirho = true;
        context_chirho.checking_binding_pattern_annotations_chirho = binding_chirho;
        let annotation_chirho = TypeChirho::VarChirho(NameChirho::RawChirho(
            haskelujah_ast_chirho::name_chirho::RawNameChirho::unqualified_chirho(
                "UnstampedChirho",
                SpanChirho::DUMMY_CHIRHO,
            ),
        ));
        assert!(
            context_chirho
                .prepare_pattern_signature_chirho(&annotation_chirho, SpanChirho::DUMMY_CHIRHO,)
        );
        assert!(!context_chirho.has_non_deferrable_failure_chirho());
        assert!(
            context_chirho
                .elaboration_state_chirho
                .failure_chirho()
                .is_none()
        );
        assert!(context_chirho.scoped_tyvars_chirho.is_empty());
        assert!(context_chirho.skolem_bindings_chirho.is_empty_chirho());
        assert!(context_chirho.type_pattern_alias_undo_chirho.is_empty());
    }
}
