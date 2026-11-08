// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Typed pre-Core failures; field-account loss is neither a source rejection
//! from GHC nor a projection/resource failure.

use haskelujah_ast_chirho::decl_chirho::{
    ConstructorSignatureErrorChirho, GadtProjectionErrorChirho,
};
use haskelujah_span_chirho::SpanChirho;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesugarInputErrorChirho {
    cause_chirho: InputCauseChirho,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputCauseChirho {
    CheckedChirho(CheckedCauseChirho),
    UnreadChirho(UnreadCauseChirho),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CheckedCauseChirho {
    ConstructorChirho(ConstructorSignatureErrorChirho),
    ProjectionChirho(GadtProjectionErrorChirho),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnreadCauseChirho {
    SyntaxChirho(SyntaxCauseChirho, SpanChirho),
    FieldAccountChirho(SpanChirho),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SyntaxCauseChirho {
    PatternChirho,
    TransformChirho,
}

impl DesugarInputErrorChirho {
    pub fn span_chirho(self) -> SpanChirho {
        match self.cause_chirho {
            InputCauseChirho::CheckedChirho(CheckedCauseChirho::ConstructorChirho(
                error_chirho,
            )) => error_chirho.span_chirho(),
            InputCauseChirho::CheckedChirho(CheckedCauseChirho::ProjectionChirho(error_chirho)) => {
                error_chirho.span_chirho
            }
            InputCauseChirho::UnreadChirho(
                UnreadCauseChirho::SyntaxChirho(_, span_chirho)
                | UnreadCauseChirho::FieldAccountChirho(span_chirho),
            ) => span_chirho,
        }
    }
    pub fn ghc_code_chirho(self) -> Option<&'static str> {
        self.constructor_error_chirho()
            .and_then(ConstructorSignatureErrorChirho::ghc_code_chirho)
    }
    pub fn constructor_error_chirho(self) -> Option<ConstructorSignatureErrorChirho> {
        match self.cause_chirho {
            InputCauseChirho::CheckedChirho(CheckedCauseChirho::ConstructorChirho(
                error_chirho,
            )) => Some(error_chirho),
            _ => None,
        }
    }
    pub fn projection_error_chirho(self) -> Option<GadtProjectionErrorChirho> {
        match self.cause_chirho {
            InputCauseChirho::CheckedChirho(CheckedCauseChirho::ProjectionChirho(error_chirho)) => {
                Some(error_chirho)
            }
            _ => None,
        }
    }
    pub fn is_unexpanded_pattern_chirho(self) -> bool {
        matches!(
            self.cause_chirho,
            InputCauseChirho::UnreadChirho(UnreadCauseChirho::SyntaxChirho(
                SyntaxCauseChirho::PatternChirho,
                _
            ))
        )
    }
    pub fn is_unread_transform_chirho(self) -> bool {
        matches!(
            self.cause_chirho,
            InputCauseChirho::UnreadChirho(UnreadCauseChirho::SyntaxChirho(
                SyntaxCauseChirho::TransformChirho,
                _
            ))
        )
    }
    pub fn is_field_account_chirho(self) -> bool {
        matches!(
            self.cause_chirho,
            InputCauseChirho::UnreadChirho(UnreadCauseChirho::FieldAccountChirho(_))
        )
    }
    pub(super) fn unexpanded_pattern_chirho(span_chirho: SpanChirho) -> Self {
        Self {
            cause_chirho: InputCauseChirho::UnreadChirho(UnreadCauseChirho::SyntaxChirho(
                SyntaxCauseChirho::PatternChirho,
                span_chirho,
            )),
        }
    }
    pub(super) fn unread_transform_chirho(span_chirho: SpanChirho) -> Self {
        Self {
            cause_chirho: InputCauseChirho::UnreadChirho(UnreadCauseChirho::SyntaxChirho(
                SyntaxCauseChirho::TransformChirho,
                span_chirho,
            )),
        }
    }
    pub(super) fn field_account_chirho(span_chirho: SpanChirho) -> Self {
        Self {
            cause_chirho: InputCauseChirho::UnreadChirho(UnreadCauseChirho::FieldAccountChirho(
                span_chirho,
            )),
        }
    }
}
impl From<ConstructorSignatureErrorChirho> for DesugarInputErrorChirho {
    fn from(error_chirho: ConstructorSignatureErrorChirho) -> Self {
        Self {
            cause_chirho: InputCauseChirho::CheckedChirho(CheckedCauseChirho::ConstructorChirho(
                error_chirho,
            )),
        }
    }
}
impl From<GadtProjectionErrorChirho> for DesugarInputErrorChirho {
    fn from(error_chirho: GadtProjectionErrorChirho) -> Self {
        Self {
            cause_chirho: InputCauseChirho::CheckedChirho(CheckedCauseChirho::ProjectionChirho(
                error_chirho,
            )),
        }
    }
}
impl std::fmt::Display for DesugarInputErrorChirho {
    fn fmt(&self, formatter_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.cause_chirho {
            InputCauseChirho::CheckedChirho(CheckedCauseChirho::ConstructorChirho(
                error_chirho,
            )) => error_chirho.fmt(formatter_chirho),
            InputCauseChirho::CheckedChirho(CheckedCauseChirho::ProjectionChirho(error_chirho)) => {
                error_chirho.fmt(formatter_chirho)
            }
            InputCauseChirho::UnreadChirho(UnreadCauseChirho::SyntaxChirho(
                SyntaxCauseChirho::PatternChirho,
                _,
            )) => formatter_chirho.write_str("unsupported unexpanded executable pattern splice"),
            InputCauseChirho::UnreadChirho(UnreadCauseChirho::SyntaxChirho(
                SyntaxCauseChirho::TransformChirho,
                _,
            )) => formatter_chirho.write_str("unread comprehension transformer"),
            InputCauseChirho::UnreadChirho(UnreadCauseChirho::FieldAccountChirho(_)) => {
                formatter_chirho.write_str(
                    "named record field requires an unambiguous constructor field account",
                )
            }
        }
    }
}
impl std::error::Error for DesugarInputErrorChirho {}
