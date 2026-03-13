// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life.

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeverityChirho {
    ErrorChirho,
    WarningChirho,
}

impl Display for SeverityChirho {
    fn fmt(&self, formatter_chirho: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ErrorChirho => formatter_chirho.write_str("error"),
            Self::WarningChirho => formatter_chirho.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticChirho {
    pub severity_chirho: SeverityChirho,
    pub message_chirho: String,
    pub line_number_chirho: Option<usize>,
}

impl DiagnosticChirho {
    pub fn error_chirho(
        message_chirho: impl Into<String>,
        line_number_chirho: Option<usize>,
    ) -> Self {
        Self {
            severity_chirho: SeverityChirho::ErrorChirho,
            message_chirho: message_chirho.into(),
            line_number_chirho,
        }
    }
}

impl Display for DiagnosticChirho {
    fn fmt(&self, formatter_chirho: &mut Formatter<'_>) -> fmt::Result {
        match self.line_number_chirho {
            Some(line_number_chirho) => write!(
                formatter_chirho,
                "{}: line {}: {}",
                self.severity_chirho, line_number_chirho, self.message_chirho
            ),
            None => write!(
                formatter_chirho,
                "{}: {}",
                self.severity_chirho, self.message_chirho
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DiagnosticBundleChirho {
    diagnostics_chirho: Vec<DiagnosticChirho>,
}

impl DiagnosticBundleChirho {
    pub fn new_chirho(diagnostics_chirho: Vec<DiagnosticChirho>) -> Self {
        Self { diagnostics_chirho }
    }

    pub fn push_chirho(&mut self, diagnostic_chirho: DiagnosticChirho) {
        self.diagnostics_chirho.push(diagnostic_chirho);
    }

    pub fn into_vec_chirho(self) -> Vec<DiagnosticChirho> {
        self.diagnostics_chirho
    }

    pub fn is_empty_chirho(&self) -> bool {
        self.diagnostics_chirho.is_empty()
    }
}

impl From<DiagnosticChirho> for DiagnosticBundleChirho {
    fn from(diagnostic_chirho: DiagnosticChirho) -> Self {
        Self::new_chirho(vec![diagnostic_chirho])
    }
}

impl Display for DiagnosticBundleChirho {
    fn fmt(&self, formatter_chirho: &mut Formatter<'_>) -> fmt::Result {
        for (index_chirho, diagnostic_chirho) in self.diagnostics_chirho.iter().enumerate() {
            if index_chirho > 0 {
                formatter_chirho.write_str("\n")?;
            }

            write!(formatter_chirho, "{diagnostic_chirho}")?;
        }

        Ok(())
    }
}

impl std::error::Error for DiagnosticBundleChirho {}

