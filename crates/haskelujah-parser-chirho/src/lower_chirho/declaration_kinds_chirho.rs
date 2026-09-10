// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Preserve complete and result kind contracts separately, including their spans.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{
    ChildChirho, DataKindSigChirho, DeclChirho, HashMap, LowerCtxChirho, SpanChirho, TypeChirho,
};

impl LowerCtxChirho {
    /// Composite kinds own only their written children, not the surrounding
    /// declaration. Keep diagnostic provenance when flat lowering needs a span.
    pub(super) fn declaration_kind_from_children_chirho(
        &self,
        children_chirho: &[&ChildChirho<'_>],
        fallback_chirho: SpanChirho,
    ) -> TypeChirho {
        let span_chirho = children_chirho
            .first()
            .zip(children_chirho.last())
            .map(|(first_chirho, last_chirho)| {
                self.span_chirho(first_chirho.start_chirho, last_chirho.end_chirho)
            })
            .unwrap_or(fallback_chirho);
        self.type_from_flat_children_chirho(children_chirho, span_chirho)
    }

    pub(super) fn attach_standalone_kind_sigs_chirho(
        &self,
        mut decls_chirho: Vec<DeclChirho>,
        standalone_kind_sigs_chirho: &HashMap<String, TypeChirho>,
    ) -> Vec<DeclChirho> {
        for decl_chirho in &mut decls_chirho {
            let (name_chirho, kind_sig_chirho) = match decl_chirho {
                DeclChirho::DataDeclChirho {
                    name_chirho,
                    kind_sig_chirho,
                    ..
                }
                | DeclChirho::NewtypeDeclChirho {
                    name_chirho,
                    kind_sig_chirho,
                    ..
                } => (name_chirho, kind_sig_chirho),
                _ => continue,
            };
            if let Some(signature_chirho) =
                standalone_kind_sigs_chirho.get(name_chirho.text_chirho())
            {
                let result_chirho = match kind_sig_chirho.take() {
                    Some(DataKindSigChirho::ResultChirho(result_chirho)) => Some(result_chirho),
                    Some(DataKindSigChirho::StandaloneChirho { result_chirho, .. }) => {
                        result_chirho
                    }
                    None => None,
                };
                *kind_sig_chirho = Some(DataKindSigChirho::StandaloneChirho {
                    signature_chirho: signature_chirho.clone(),
                    result_chirho,
                });
            }
        }
        decls_chirho
    }
}
