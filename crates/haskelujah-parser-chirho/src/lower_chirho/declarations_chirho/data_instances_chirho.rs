// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Nominal family instances retain their actual heads and constructor bodies.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::*;

impl LowerCtxChirho {
    pub(super) fn lower_data_instance_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
        newtype_chirho: bool,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut head_children_chirho = Vec::new();
        let mut constructors_chirho = Vec::new();
        let mut deriving_chirho = Vec::new();
        let mut in_head_chirho = false;
        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(token_chirho)
                    if token_chirho.text_chirho() == "instance" =>
                {
                    in_head_chirho = true;
                }
                GreenElementChirho::TokenChirho(token_chirho)
                    if matches!(
                        token_chirho.kind_chirho(),
                        TokenKindChirho::EqualsChirho | TokenKindChirho::WhereKeywordChirho
                    ) =>
                {
                    in_head_chirho = false;
                }
                GreenElementChirho::NodeChirho(constructor_chirho)
                    if constructor_chirho.kind_chirho() == SyntaxKindChirho::ConDeclChirho =>
                {
                    constructors_chirho.push(
                        self.lower_con_decl_chirho(constructor_chirho, child_chirho.start_chirho),
                    );
                }
                GreenElementChirho::NodeChirho(constructor_chirho)
                    if constructor_chirho.kind_chirho() == SyntaxKindChirho::GadtConDeclChirho =>
                {
                    constructors_chirho.push(
                        self.lower_gadt_con_decl_chirho(
                            constructor_chirho,
                            child_chirho.start_chirho,
                        ),
                    );
                }
                GreenElementChirho::NodeChirho(deriving_node_chirho)
                    if deriving_node_chirho.kind_chirho()
                        == SyntaxKindChirho::DerivingClauseChirho =>
                {
                    in_head_chirho = false;
                    deriving_chirho =
                        self.lower_deriving_chirho(deriving_node_chirho, child_chirho.start_chirho);
                }
                _ if in_head_chirho => head_children_chirho.push(child_chirho),
                _ => {}
            }
        }
        DeclChirho::DataFamilyInstanceDeclChirho {
            head_chirho: self.type_from_flat_children_chirho(&head_children_chirho, span_chirho),
            newtype_chirho,
            constructors_chirho,
            deriving_chirho,
            span_chirho,
        }
    }
}
