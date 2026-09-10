// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Arrow annotations own their syntax; they are never value arguments.
use super::{
    GreenElementChirho, GreenNodeChirho, LowerCtxChirho, SyntaxKindChirho, TokenKindChirho,
    TypeChirho, is_type_kind_chirho,
};
use haskelujah_ast_chirho::ty_chirho::MultiplicityChirho;

impl LowerCtxChirho {
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    pub(super) fn lower_function_type_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> TypeChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut types_chirho = children_chirho.iter().filter(|child_chirho| {
            matches!(child_chirho.element_chirho, GreenElementChirho::NodeChirho(type_chirho)
                if is_type_kind_chirho(type_chirho.kind_chirho()))
        });
        let (Some(argument_chirho), Some(result_chirho)) =
            (types_chirho.next(), types_chirho.next())
        else {
            return self.placeholder_type_chirho();
        };
        let mut multiplicity_chirho = None;
        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(token_chirho)
                    if token_chirho.kind_chirho() == TokenKindChirho::LinearArrowChirho =>
                {
                    multiplicity_chirho = Some(MultiplicityChirho::OneChirho);
                }
                GreenElementChirho::NodeChirho(annotation_chirho)
                    if annotation_chirho.kind_chirho()
                        == SyntaxKindChirho::ArrowMultiplicityChirho =>
                {
                    let annotation_children_chirho =
                        self.semantic_children_chirho(annotation_chirho, child_chirho.start_chirho);
                    let expression_chirho = annotation_children_chirho.iter().find(|atom_chirho| {
                        matches!(atom_chirho.element_chirho, GreenElementChirho::NodeChirho(type_chirho)
                            if is_type_kind_chirho(type_chirho.kind_chirho()))
                    });
                    if let Some(expression_chirho) = expression_chirho {
                        let type_chirho = self.lower_type_from_child_chirho(expression_chirho);
                        // Only the bare `%1` token is arrow sugar. Parenthesized
                        // numeric types and other literals must pass kind checking.
                        multiplicity_chirho = Some(match &type_chirho {
                            TypeChirho::LitChirho { value_chirho, .. } if value_chirho == "1" => {
                                MultiplicityChirho::OneChirho
                            }
                            _ => MultiplicityChirho::ExpressionChirho(Box::new(type_chirho)),
                        });
                    }
                }
                _ => {}
            }
        }
        TypeChirho::FunChirho {
            arg_chirho: Box::new(self.lower_type_from_child_chirho(argument_chirho)),
            mult_chirho: multiplicity_chirho,
            result_chirho: Box::new(self.lower_type_from_child_chirho(result_chirho)),
            span_chirho: self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho()),
        }
    }
}
