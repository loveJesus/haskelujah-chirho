// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # The Q monad — Template Haskell's compile-time environment
//!
//! The Q monad provides:
//! - Fresh name generation (`newName`)
//! - Reification of compile-time information (`reify`)
//! - Error/warning reporting (`reportError`, `reportWarning`)
//! - Name lookup (`lookupValueName`, `lookupTypeName`)
//!
//! In the Haskeluya compiler, Q operations are backed by Rust callbacks that
//! read from the compiler's internal state (module interfaces, type environment,
//! AST declarations).

use crate::th_ast_chirho::{ThInfoChirho, ThNameChirho, ThNameFlavourChirho};

/// The state carried by the Q monad during splice evaluation.
pub struct QStateChirho {
    /// Counter for generating unique names via `newName`.
    fresh_counter_chirho: u64,
    /// Accumulated errors from `reportError`.
    errors_chirho: Vec<String>,
    /// Accumulated warnings from `reportWarning`.
    warnings_chirho: Vec<String>,
    /// Callback for reification — provided by the driver at splice time.
    reify_callback_chirho: Option<Box<dyn Fn(&ThNameChirho) -> Option<ThInfoChirho>>>,
    /// Callback for value name lookup.
    lookup_value_callback_chirho: Option<Box<dyn Fn(&str) -> Option<ThNameChirho>>>,
    /// Callback for type name lookup.
    lookup_type_callback_chirho: Option<Box<dyn Fn(&str) -> Option<ThNameChirho>>>,
}

impl QStateChirho {
    /// Create a new Q state with a starting unique counter.
    pub fn new_chirho(start_counter_chirho: u64) -> Self {
        Self {
            fresh_counter_chirho: start_counter_chirho,
            errors_chirho: Vec::new(),
            warnings_chirho: Vec::new(),
            reify_callback_chirho: None,
            lookup_value_callback_chirho: None,
            lookup_type_callback_chirho: None,
        }
    }

    /// Set the reification callback (called by the driver before splice evaluation).
    pub fn set_reify_chirho(&mut self, callback_chirho: Box<dyn Fn(&ThNameChirho) -> Option<ThInfoChirho>>) {
        self.reify_callback_chirho = Some(callback_chirho);
    }

    /// Set the value name lookup callback.
    pub fn set_lookup_value_chirho(&mut self, callback_chirho: Box<dyn Fn(&str) -> Option<ThNameChirho>>) {
        self.lookup_value_callback_chirho = Some(callback_chirho);
    }

    /// Set the type name lookup callback.
    pub fn set_lookup_type_chirho(&mut self, callback_chirho: Box<dyn Fn(&str) -> Option<ThNameChirho>>) {
        self.lookup_type_callback_chirho = Some(callback_chirho);
    }

    // -- Q primitive operations --

    /// Generate a fresh unique name (Q's `newName`).
    pub fn new_name_chirho(&mut self, base_chirho: &str) -> ThNameChirho {
        let id_chirho = self.fresh_counter_chirho;
        self.fresh_counter_chirho += 1;
        ThNameChirho::unique_chirho(base_chirho, id_chirho)
    }

    /// Look up compile-time information about a name (Q's `reify`).
    pub fn reify_chirho(&self, name_chirho: &ThNameChirho) -> Result<ThInfoChirho, String> {
        match &self.reify_callback_chirho {
            Some(cb_chirho) => cb_chirho(name_chirho)
                .ok_or_else(|| format!("reify: name not found: {}", name_chirho)),
            None => Err("reify: no reification environment available".to_string()),
        }
    }

    /// Look up a value-level name by string (Q's `lookupValueName`).
    pub fn lookup_value_name_chirho(&self, name_chirho: &str) -> Option<ThNameChirho> {
        self.lookup_value_callback_chirho.as_ref()?.as_ref()(name_chirho)
    }

    /// Look up a type-level name by string (Q's `lookupTypeName`).
    pub fn lookup_type_name_chirho(&self, name_chirho: &str) -> Option<ThNameChirho> {
        self.lookup_type_callback_chirho.as_ref()?.as_ref()(name_chirho)
    }

    /// Report an error (Q's `reportError`).
    pub fn report_error_chirho(&mut self, msg_chirho: &str) {
        self.errors_chirho.push(msg_chirho.to_string());
    }

    /// Report a warning (Q's `reportWarning`).
    pub fn report_warning_chirho(&mut self, msg_chirho: &str) {
        self.warnings_chirho.push(msg_chirho.to_string());
    }

    /// Check if any errors were reported.
    pub fn has_errors_chirho(&self) -> bool {
        !self.errors_chirho.is_empty()
    }

    /// Get all reported errors.
    pub fn errors_chirho(&self) -> &[String] {
        &self.errors_chirho
    }

    /// Get all reported warnings.
    pub fn warnings_chirho(&self) -> &[String] {
        &self.warnings_chirho
    }

    /// Get the current unique counter value (for hygiene tracking).
    pub fn counter_chirho(&self) -> u64 {
        self.fresh_counter_chirho
    }

    /// Create a name from a string (Q's `mkName` — not fresh, just wraps a string).
    pub fn mk_name_chirho(s_chirho: &str) -> ThNameChirho {
        ThNameChirho {
            occ_chirho: s_chirho.to_string(),
            flavour_chirho: ThNameFlavourChirho::StringChirho,
        }
    }
}

impl Default for QStateChirho {
    fn default() -> Self {
        Self::new_chirho(1000) // Start at 1000 to avoid collision with compiler-internal IDs
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn new_name_produces_unique_ids_chirho() {
        let mut q_chirho = QStateChirho::new_chirho(0);
        let n1_chirho = q_chirho.new_name_chirho("x");
        let n2_chirho = q_chirho.new_name_chirho("x");
        let n3_chirho = q_chirho.new_name_chirho("y");

        assert_ne!(n1_chirho, n2_chirho);
        assert_ne!(n2_chirho, n3_chirho);
        assert_eq!(n1_chirho.occ_chirho, "x");
        assert_eq!(n3_chirho.occ_chirho, "y");
        assert!(matches!(n1_chirho.flavour_chirho, ThNameFlavourChirho::UniqueChirho(0)));
        assert!(matches!(n2_chirho.flavour_chirho, ThNameFlavourChirho::UniqueChirho(1)));
    }

    #[test]
    fn report_error_and_warning_chirho() {
        let mut q_chirho = QStateChirho::default();
        assert!(!q_chirho.has_errors_chirho());

        q_chirho.report_error_chirho("something went wrong");
        assert!(q_chirho.has_errors_chirho());
        assert_eq!(q_chirho.errors_chirho().len(), 1);

        q_chirho.report_warning_chirho("careful");
        assert_eq!(q_chirho.warnings_chirho().len(), 1);
    }

    #[test]
    fn reify_without_callback_returns_error_chirho() {
        let q_chirho = QStateChirho::default();
        let name_chirho = ThNameChirho::mk_name_chirho("Foo");
        let result_chirho = q_chirho.reify_chirho(&name_chirho);
        assert!(result_chirho.is_err());
    }

    #[test]
    fn reify_with_callback_chirho() {
        use crate::th_ast_chirho::*;

        let mut q_chirho = QStateChirho::default();
        q_chirho.set_reify_chirho(Box::new(|name_chirho| {
            if name_chirho.occ_chirho == "Int" {
                Some(ThInfoChirho::PrimTyConIChirho(
                    ThNameChirho::mk_name_chirho("Int"),
                    0,
                    false,
                ))
            } else {
                None
            }
        }));

        let int_name_chirho = ThNameChirho::mk_name_chirho("Int");
        let result_chirho = q_chirho.reify_chirho(&int_name_chirho);
        assert!(result_chirho.is_ok());

        let unknown_chirho = ThNameChirho::mk_name_chirho("Unknown");
        let result_chirho = q_chirho.reify_chirho(&unknown_chirho);
        assert!(result_chirho.is_err());
    }

    #[test]
    fn mk_name_creates_string_flavour_chirho() {
        let name_chirho = QStateChirho::mk_name_chirho("test");
        assert_eq!(name_chirho.occ_chirho, "test");
        assert!(matches!(name_chirho.flavour_chirho, ThNameFlavourChirho::StringChirho));
    }
}
