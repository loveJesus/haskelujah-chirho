// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Preserve complete and result kind contracts separately, including their spans.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{
    ChildChirho, DeclChirho, DeclKindSigChirho, GreenElementChirho, HashMap, LowerCtxChirho,
    SpanChirho, TokenKindChirho, TypeChirho,
};
use haskelujah_ast_chirho::decl_chirho::{TyVarChirho, TyVarVisibilityChirho};

impl LowerCtxChirho {
    /// Read one whole declaration-head binder. An `@` changes visibility, not
    /// lexical scope. An unreadable kind is skipped as a group, never rescanned
    /// as declaration syntax (the declaration-kind provenance invariant).
    pub(super) fn declaration_head_binder_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        start_chirho: usize,
    ) -> Option<(TyVarChirho, usize)> {
        let mut index_chirho = start_chirho;
        let invisible_chirho = matches!(
            children_chirho.get(index_chirho)?.element_chirho,
            GreenElementChirho::TokenChirho(token_chirho)
                if token_chirho.kind_chirho() == TokenKindChirho::AtSignChirho
        );
        if invisible_chirho {
            index_chirho += 1;
        }
        let child_chirho = children_chirho.get(index_chirho)?;
        let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho else {
            return None;
        };
        let (mut binder_chirho, end_chirho) =
            if Self::is_head_binder_name_chirho(token_chirho.kind_chirho()) {
                let span_chirho =
                    self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                (
                    TyVarChirho::plain_chirho(
                        self.name_from_token_chirho(token_chirho, span_chirho),
                    ),
                    index_chirho + 1,
                )
            } else if token_chirho.kind_chirho() == TokenKindChirho::LeftParenChirho {
                if let Some((binder_chirho, consumed_chirho)) =
                    self.try_parse_kind_annotated_tyvar_chirho(children_chirho, index_chirho)
                {
                    (binder_chirho, index_chirho + consumed_chirho)
                } else {
                    let (name_chirho, name_index_chirho, close_chirho) =
                        Self::unreadable_kind_binder_chirho(children_chirho, index_chirho)?;
                    let name_child_chirho = &children_chirho[name_index_chirho];
                    let span_chirho = self
                        .span_chirho(name_child_chirho.start_chirho, name_child_chirho.end_chirho);
                    (
                        TyVarChirho::plain_chirho(
                            self.name_from_token_chirho(name_chirho, span_chirho),
                        ),
                        close_chirho + 1,
                    )
                }
            } else {
                return None;
            };
        if invisible_chirho {
            binder_chirho.visibility_chirho = TyVarVisibilityChirho::InvisibleChirho;
        }
        Some((binder_chirho, end_chirho - start_chirho))
    }

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
                DeclChirho::TypeFamilyDeclChirho {
                    name_chirho,
                    result_chirho,
                    ..
                } => (name_chirho, &mut result_chirho.kind_sig_chirho),
                _ => continue,
            };
            if let Some(signature_chirho) =
                standalone_kind_sigs_chirho.get(name_chirho.text_chirho())
            {
                let result_chirho = match kind_sig_chirho.take() {
                    Some(DeclKindSigChirho::ResultChirho(result_chirho)) => Some(result_chirho),
                    Some(DeclKindSigChirho::StandaloneChirho { result_chirho, .. }) => {
                        result_chirho
                    }
                    None => None,
                };
                *kind_sig_chirho = Some(DeclKindSigChirho::StandaloneChirho {
                    signature_chirho: signature_chirho.clone(),
                    result_chirho,
                });
            }
        }
        decls_chirho
    }
}
