// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ordered list-comprehension syntax keeps transformations separate from do
//! statements. Every consumer must preserve the actual transform/key payload.
use super::{ExprChirho, StmtChirho};
use haskelujah_span_chirho::SpanChirho;

#[derive(Debug, Clone, PartialEq)]
pub enum ComprehensionQualifierChirho {
    StatementChirho(Box<StmtChirho>),
    TransformChirho(ComprehensionTransformChirho),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComprehensionTransformChirho {
    /// True only for the actual written group grammar.
    pub group_chirho: bool,
    /// None retains an unread/missing payload for checked consumers to refuse.
    pub using_chirho: Option<Box<ExprChirho>>,
    pub by_chirho: Option<Box<ExprChirho>>,
    pub span_chirho: SpanChirho,
}

impl From<StmtChirho> for ComprehensionQualifierChirho {
    fn from(statement_chirho: StmtChirho) -> Self {
        Self::StatementChirho(Box::new(statement_chirho))
    }
}

impl ComprehensionQualifierChirho {
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::StatementChirho(statement_chirho) => match statement_chirho.as_ref() {
                StmtChirho::ExprChirho { expr_chirho, .. } => expr_chirho.span_chirho(),
                StmtChirho::BindChirho { span_chirho, .. }
                | StmtChirho::LetChirho { span_chirho, .. } => *span_chirho,
            },
            Self::TransformChirho(transform_chirho) => transform_chirho.span_chirho,
        }
    }
}
