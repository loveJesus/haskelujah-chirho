// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskeluya-diagnostics-chirho
//!
//! Structured, format-agnostic compiler diagnostics for the Haskeluya compiler.
//! Supports byte-offset spans, error codes, labeled secondary spans, fix
//! suggestions, and multiple severity levels.
//!
//! Diagnostics are the same data structure regardless of output target (CLI,
//! LSP, JSON, REPL). The [`render_chirho`] module provides a Rust/Elm-style
//! text renderer with source code snippets and ANSI color support.

pub mod render_chirho;
pub mod suggest_chirho;

use std::fmt::{self, Display, Formatter};

use haskeluya_span_chirho::SpanChirho;

// ---------------------------------------------------------------------------
// SeverityChirho
// ---------------------------------------------------------------------------

/// How serious a diagnostic is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SeverityChirho {
    /// Informational hint — not a problem, just a suggestion.
    HintChirho,
    /// Informational note attached to another diagnostic.
    InfoChirho,
    /// Something that should be addressed but does not prevent compilation.
    WarningChirho,
    /// A fatal problem that prevents further progress in this phase.
    ErrorChirho,
}

impl Display for SeverityChirho {
    fn fmt(&self, f_chirho: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::HintChirho => f_chirho.write_str("hint"),
            Self::InfoChirho => f_chirho.write_str("info"),
            Self::WarningChirho => f_chirho.write_str("warning"),
            Self::ErrorChirho => f_chirho.write_str("error"),
        }
    }
}

// ---------------------------------------------------------------------------
// ErrorCodeChirho
// ---------------------------------------------------------------------------

/// A machine-readable error code that classifies the diagnostic.
///
/// Codes are organized by phase:
/// - `E0001`–`E0099`: lexer / parse errors
/// - `E0100`–`E0199`: name resolution errors
/// - `E0200`–`E0299`: type errors
/// - `E0300`–`E0399`: kind errors
/// - `E0400`–`E0499`: pattern / exhaustiveness errors
/// - `E0500`–`E0599`: package / module errors
/// - `W0001`–`W0999`: warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorCodeChirho {
    prefix_chirho: char,
    number_chirho: u16,
}

impl ErrorCodeChirho {
    /// Create a new error code.
    #[inline]
    pub const fn new_chirho(prefix_chirho: char, number_chirho: u16) -> Self {
        Self {
            prefix_chirho,
            number_chirho,
        }
    }

    /// Convenience for error codes (`E____`).
    #[inline]
    pub const fn error_chirho(number_chirho: u16) -> Self {
        Self::new_chirho('E', number_chirho)
    }

    /// Convenience for warning codes (`W____`).
    #[inline]
    pub const fn warning_chirho(number_chirho: u16) -> Self {
        Self::new_chirho('W', number_chirho)
    }
}

impl Display for ErrorCodeChirho {
    fn fmt(&self, f_chirho: &mut Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{}{:04}", self.prefix_chirho, self.number_chirho)
    }
}

// ---------------------------------------------------------------------------
// LabelChirho — a labeled span (primary or secondary)
// ---------------------------------------------------------------------------

/// A span with an optional message, used to annotate diagnostic locations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelChirho {
    /// The source span this label points to.
    pub span_chirho: SpanChirho,
    /// Optional message shown next to the span (e.g. "first defined here").
    pub message_chirho: Option<String>,
    /// Whether this is the primary span of the diagnostic.
    pub is_primary_chirho: bool,
}

impl LabelChirho {
    /// Create a primary label (the main location of the problem).
    pub fn primary_chirho(span_chirho: SpanChirho, message_chirho: impl Into<String>) -> Self {
        Self {
            span_chirho,
            message_chirho: Some(message_chirho.into()),
            is_primary_chirho: true,
        }
    }

    /// Create a primary label without a message.
    pub fn primary_bare_chirho(span_chirho: SpanChirho) -> Self {
        Self {
            span_chirho,
            message_chirho: None,
            is_primary_chirho: true,
        }
    }

    /// Create a secondary label (a related location).
    pub fn secondary_chirho(span_chirho: SpanChirho, message_chirho: impl Into<String>) -> Self {
        Self {
            span_chirho,
            message_chirho: Some(message_chirho.into()),
            is_primary_chirho: false,
        }
    }
}

// ---------------------------------------------------------------------------
// FixSuggestionChirho — a concrete text replacement
// ---------------------------------------------------------------------------

/// A suggested fix: replace the text at `span_chirho` with `replacement_chirho`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixSuggestionChirho {
    /// Human-readable description of the fix.
    pub description_chirho: String,
    /// The span to replace.
    pub span_chirho: SpanChirho,
    /// The replacement text.
    pub replacement_chirho: String,
}

// ---------------------------------------------------------------------------
// DiagnosticChirho — a single diagnostic
// ---------------------------------------------------------------------------

/// A single compiler diagnostic with all the information needed to render
/// it in any output format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticChirho {
    /// Severity level.
    pub severity_chirho: SeverityChirho,
    /// Machine-readable error code (optional but recommended).
    pub code_chirho: Option<ErrorCodeChirho>,
    /// The main human-readable message.
    pub message_chirho: String,
    /// Labeled spans pointing to relevant source locations.
    pub labels_chirho: Vec<LabelChirho>,
    /// Additional notes shown after the main message.
    pub notes_chirho: Vec<String>,
    /// Suggested fixes with concrete text replacements.
    pub suggestions_chirho: Vec<FixSuggestionChirho>,
}

impl DiagnosticChirho {
    /// Create an error diagnostic with a primary span.
    pub fn error_chirho(
        message_chirho: impl Into<String>,
        span_chirho: SpanChirho,
    ) -> Self {
        Self {
            severity_chirho: SeverityChirho::ErrorChirho,
            code_chirho: None,
            message_chirho: message_chirho.into(),
            labels_chirho: vec![LabelChirho::primary_bare_chirho(span_chirho)],
            notes_chirho: Vec::new(),
            suggestions_chirho: Vec::new(),
        }
    }

    /// Create an error diagnostic with a code and primary span.
    pub fn error_with_code_chirho(
        code_chirho: ErrorCodeChirho,
        message_chirho: impl Into<String>,
        span_chirho: SpanChirho,
    ) -> Self {
        Self {
            severity_chirho: SeverityChirho::ErrorChirho,
            code_chirho: Some(code_chirho),
            message_chirho: message_chirho.into(),
            labels_chirho: vec![LabelChirho::primary_bare_chirho(span_chirho)],
            notes_chirho: Vec::new(),
            suggestions_chirho: Vec::new(),
        }
    }

    /// Create a warning diagnostic with a primary span.
    pub fn warning_chirho(
        message_chirho: impl Into<String>,
        span_chirho: SpanChirho,
    ) -> Self {
        Self {
            severity_chirho: SeverityChirho::WarningChirho,
            code_chirho: None,
            message_chirho: message_chirho.into(),
            labels_chirho: vec![LabelChirho::primary_bare_chirho(span_chirho)],
            notes_chirho: Vec::new(),
            suggestions_chirho: Vec::new(),
        }
    }

    /// Create a warning diagnostic with a code and primary span.
    pub fn warning_with_code_chirho(
        code_chirho: ErrorCodeChirho,
        message_chirho: impl Into<String>,
        span_chirho: SpanChirho,
    ) -> Self {
        Self {
            severity_chirho: SeverityChirho::WarningChirho,
            code_chirho: Some(code_chirho),
            message_chirho: message_chirho.into(),
            labels_chirho: vec![LabelChirho::primary_bare_chirho(span_chirho)],
            notes_chirho: Vec::new(),
            suggestions_chirho: Vec::new(),
        }
    }

    /// Create an error with no span (for file-level errors like "cannot read file").
    pub fn error_no_span_chirho(message_chirho: impl Into<String>) -> Self {
        Self {
            severity_chirho: SeverityChirho::ErrorChirho,
            code_chirho: None,
            message_chirho: message_chirho.into(),
            labels_chirho: Vec::new(),
            notes_chirho: Vec::new(),
            suggestions_chirho: Vec::new(),
        }
    }

    /// Builder: set the error code.
    pub fn with_code_chirho(mut self, code_chirho: ErrorCodeChirho) -> Self {
        self.code_chirho = Some(code_chirho);
        self
    }

    /// Builder: add a secondary label.
    pub fn with_label_chirho(mut self, label_chirho: LabelChirho) -> Self {
        self.labels_chirho.push(label_chirho);
        self
    }

    /// Builder: add a note.
    pub fn with_note_chirho(mut self, note_chirho: impl Into<String>) -> Self {
        self.notes_chirho.push(note_chirho.into());
        self
    }

    /// Builder: add a fix suggestion.
    pub fn with_suggestion_chirho(mut self, suggestion_chirho: FixSuggestionChirho) -> Self {
        self.suggestions_chirho.push(suggestion_chirho);
        self
    }

    /// Get the primary span, if any.
    pub fn primary_span_chirho(&self) -> Option<SpanChirho> {
        self.labels_chirho
            .iter()
            .find(|label_chirho| label_chirho.is_primary_chirho)
            .map(|label_chirho| label_chirho.span_chirho)
    }

    /// Whether this is an error-severity diagnostic.
    pub fn is_error_chirho(&self) -> bool {
        self.severity_chirho == SeverityChirho::ErrorChirho
    }
}

impl Display for DiagnosticChirho {
    fn fmt(&self, f_chirho: &mut Formatter<'_>) -> fmt::Result {
        // Minimal text rendering for CLI/debug. Full rendering with source
        // snippets is done by a dedicated renderer that has access to the
        // SourceMapChirho.
        write!(f_chirho, "{}", self.severity_chirho)?;
        if let Some(ref code_chirho) = self.code_chirho {
            write!(f_chirho, "[{code_chirho}]")?;
        }
        write!(f_chirho, ": {}", self.message_chirho)?;

        if let Some(span_chirho) = self.primary_span_chirho() {
            if span_chirho != SpanChirho::DUMMY_CHIRHO {
                write!(f_chirho, " (at {span_chirho})")?;
            }
        }

        for note_chirho in &self.notes_chirho {
            write!(f_chirho, "\n  note: {note_chirho}")?;
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// DiagnosticBundleChirho — a collection of diagnostics from a phase
// ---------------------------------------------------------------------------

/// A collection of diagnostics produced by a compiler phase.
///
/// The bundle accumulates diagnostics so the compiler can continue past
/// non-fatal errors and report everything at once.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DiagnosticBundleChirho {
    diagnostics_chirho: Vec<DiagnosticChirho>,
}

impl DiagnosticBundleChirho {
    /// Create a bundle from a vec of diagnostics.
    pub fn new_chirho(diagnostics_chirho: Vec<DiagnosticChirho>) -> Self {
        Self { diagnostics_chirho }
    }

    /// Create an empty bundle.
    pub fn empty_chirho() -> Self {
        Self::default()
    }

    /// Add a diagnostic to the bundle.
    pub fn push_chirho(&mut self, diagnostic_chirho: DiagnosticChirho) {
        self.diagnostics_chirho.push(diagnostic_chirho);
    }

    /// Consume the bundle and return the inner vec.
    pub fn into_vec_chirho(self) -> Vec<DiagnosticChirho> {
        self.diagnostics_chirho
    }

    /// Borrow the diagnostics.
    pub fn diagnostics_chirho(&self) -> &[DiagnosticChirho] {
        &self.diagnostics_chirho
    }

    /// Whether there are no diagnostics.
    pub fn is_empty_chirho(&self) -> bool {
        self.diagnostics_chirho.is_empty()
    }

    /// Whether any diagnostic is an error.
    pub fn has_errors_chirho(&self) -> bool {
        self.diagnostics_chirho
            .iter()
            .any(|d_chirho| d_chirho.is_error_chirho())
    }

    /// Count of error-severity diagnostics.
    pub fn error_count_chirho(&self) -> usize {
        self.diagnostics_chirho
            .iter()
            .filter(|d_chirho| d_chirho.is_error_chirho())
            .count()
    }

    /// Count of warning-severity diagnostics.
    pub fn warning_count_chirho(&self) -> usize {
        self.diagnostics_chirho
            .iter()
            .filter(|d_chirho| d_chirho.severity_chirho == SeverityChirho::WarningChirho)
            .count()
    }

    /// Total number of diagnostics.
    pub fn len_chirho(&self) -> usize {
        self.diagnostics_chirho.len()
    }

    /// Merge another bundle into this one.
    pub fn extend_chirho(&mut self, other_chirho: DiagnosticBundleChirho) {
        self.diagnostics_chirho.extend(other_chirho.diagnostics_chirho);
    }
}

impl From<DiagnosticChirho> for DiagnosticBundleChirho {
    fn from(diagnostic_chirho: DiagnosticChirho) -> Self {
        Self::new_chirho(vec![diagnostic_chirho])
    }
}

impl Display for DiagnosticBundleChirho {
    fn fmt(&self, f_chirho: &mut Formatter<'_>) -> fmt::Result {
        for (index_chirho, diagnostic_chirho) in self.diagnostics_chirho.iter().enumerate() {
            if index_chirho > 0 {
                f_chirho.write_str("\n")?;
            }
            write!(f_chirho, "{diagnostic_chirho}")?;
        }
        Ok(())
    }
}

impl std::error::Error for DiagnosticBundleChirho {}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskeluya_span_chirho::{ByteOffsetChirho, FileIdChirho};

    fn test_span_chirho() -> SpanChirho {
        SpanChirho::new_chirho(
            FileIdChirho::SYNTHETIC_CHIRHO,
            ByteOffsetChirho::new_chirho(10),
            ByteOffsetChirho::new_chirho(20),
        )
    }

    #[test]
    fn error_with_code_displays_correctly_chirho() {
        let diag_chirho = DiagnosticChirho::error_with_code_chirho(
            ErrorCodeChirho::error_chirho(1),
            "unexpected token",
            test_span_chirho(),
        );
        let rendered_chirho = format!("{diag_chirho}");
        assert!(rendered_chirho.contains("error[E0001]"));
        assert!(rendered_chirho.contains("unexpected token"));
    }

    #[test]
    fn warning_without_code_chirho() {
        let diag_chirho = DiagnosticChirho::warning_chirho(
            "unused variable",
            test_span_chirho(),
        );
        assert_eq!(diag_chirho.severity_chirho, SeverityChirho::WarningChirho);
        assert!(diag_chirho.code_chirho.is_none());
        assert!(!diag_chirho.is_error_chirho());
    }

    #[test]
    fn builder_adds_labels_and_notes_chirho() {
        let span_a_chirho = test_span_chirho();
        let span_b_chirho = SpanChirho::new_chirho(
            FileIdChirho::SYNTHETIC_CHIRHO,
            ByteOffsetChirho::new_chirho(30),
            ByteOffsetChirho::new_chirho(40),
        );

        let diag_chirho = DiagnosticChirho::error_chirho("type mismatch", span_a_chirho)
            .with_code_chirho(ErrorCodeChirho::error_chirho(200))
            .with_label_chirho(LabelChirho::secondary_chirho(
                span_b_chirho,
                "expected because of this",
            ))
            .with_note_chirho("expected Int, found String");

        assert_eq!(diag_chirho.labels_chirho.len(), 2);
        assert_eq!(diag_chirho.notes_chirho.len(), 1);
        assert_eq!(diag_chirho.code_chirho, Some(ErrorCodeChirho::error_chirho(200)));
    }

    #[test]
    fn bundle_counts_chirho() {
        let mut bundle_chirho = DiagnosticBundleChirho::empty_chirho();
        assert!(bundle_chirho.is_empty_chirho());
        assert!(!bundle_chirho.has_errors_chirho());

        bundle_chirho.push_chirho(DiagnosticChirho::error_chirho("e1", test_span_chirho()));
        bundle_chirho.push_chirho(DiagnosticChirho::warning_chirho("w1", test_span_chirho()));
        bundle_chirho.push_chirho(DiagnosticChirho::error_chirho("e2", test_span_chirho()));

        assert_eq!(bundle_chirho.len_chirho(), 3);
        assert_eq!(bundle_chirho.error_count_chirho(), 2);
        assert_eq!(bundle_chirho.warning_count_chirho(), 1);
        assert!(bundle_chirho.has_errors_chirho());
    }

    #[test]
    fn bundle_extend_chirho() {
        let mut a_chirho = DiagnosticBundleChirho::from(
            DiagnosticChirho::error_chirho("a", test_span_chirho()),
        );
        let b_chirho = DiagnosticBundleChirho::from(
            DiagnosticChirho::warning_chirho("b", test_span_chirho()),
        );
        a_chirho.extend_chirho(b_chirho);
        assert_eq!(a_chirho.len_chirho(), 2);
    }

    #[test]
    fn error_no_span_chirho() {
        let diag_chirho = DiagnosticChirho::error_no_span_chirho("cannot read file");
        assert!(diag_chirho.primary_span_chirho().is_none());
        assert!(diag_chirho.is_error_chirho());
    }

    #[test]
    fn fix_suggestion_chirho() {
        let diag_chirho = DiagnosticChirho::error_chirho("missing where", test_span_chirho())
            .with_suggestion_chirho(FixSuggestionChirho {
                description_chirho: "add 'where' keyword".to_string(),
                span_chirho: test_span_chirho(),
                replacement_chirho: " where".to_string(),
            });
        assert_eq!(diag_chirho.suggestions_chirho.len(), 1);
    }

    #[test]
    fn error_code_display_chirho() {
        assert_eq!(format!("{}", ErrorCodeChirho::error_chirho(1)), "E0001");
        assert_eq!(format!("{}", ErrorCodeChirho::error_chirho(42)), "E0042");
        assert_eq!(format!("{}", ErrorCodeChirho::warning_chirho(1)), "W0001");
    }
}
