// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Haskell 2010 Report Conformance Tracker
//!
//! Provides a structured mapping of Haskell 2010 Report sections to
//! implementation status in Haskeluya. Each section gets a status tag
//! (Done, Partial, NotStarted) and optional notes on what's missing.

/// Conformance status for a Report section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusChirho {
    /// Fully implemented and tested.
    DoneChirho,
    /// Partially implemented — some features missing.
    PartialChirho,
    /// Not yet started.
    NotStartedChirho,
}

/// A single Haskell 2010 Report section entry.
#[derive(Debug, Clone)]
pub struct ReportSectionChirho {
    /// Report section number (e.g. "3.1").
    pub section_chirho: String,
    /// Section title.
    pub title_chirho: String,
    /// Implementation status.
    pub status_chirho: StatusChirho,
    /// Notes on what's implemented or missing.
    pub notes_chirho: String,
}

/// The full conformance tracker.
#[derive(Debug, Clone)]
pub struct ConformanceTrackerChirho {
    pub sections_chirho: Vec<ReportSectionChirho>,
}

impl ConformanceTrackerChirho {
    /// Count sections by status.
    pub fn count_by_status_chirho(&self, status_chirho: StatusChirho) -> usize {
        self.sections_chirho
            .iter()
            .filter(|s_chirho| s_chirho.status_chirho == status_chirho)
            .count()
    }

    /// Compute pass rate as a percentage.
    pub fn pass_rate_chirho(&self) -> f64 {
        let total_chirho = self.sections_chirho.len();
        if total_chirho == 0 {
            return 100.0;
        }
        let done_chirho = self.count_by_status_chirho(StatusChirho::DoneChirho);
        let partial_chirho = self.count_by_status_chirho(StatusChirho::PartialChirho);
        (done_chirho as f64 + partial_chirho as f64 * 0.5) / total_chirho as f64 * 100.0
    }

    /// Format a summary table.
    pub fn summary_table_chirho(&self) -> String {
        let mut lines_chirho = Vec::new();
        lines_chirho.push(format!(
            "{:<8} {:<45} {:<12} {}",
            "Section", "Title", "Status", "Notes"
        ));
        lines_chirho.push("-".repeat(100));
        for s_chirho in &self.sections_chirho {
            let status_str_chirho = match s_chirho.status_chirho {
                StatusChirho::DoneChirho => "DONE",
                StatusChirho::PartialChirho => "PARTIAL",
                StatusChirho::NotStartedChirho => "TODO",
            };
            lines_chirho.push(format!(
                "{:<8} {:<45} {:<12} {}",
                s_chirho.section_chirho,
                s_chirho.title_chirho,
                status_str_chirho,
                s_chirho.notes_chirho,
            ));
        }
        lines_chirho.push("-".repeat(100));
        let done_chirho = self.count_by_status_chirho(StatusChirho::DoneChirho);
        let partial_chirho = self.count_by_status_chirho(StatusChirho::PartialChirho);
        let todo_chirho = self.count_by_status_chirho(StatusChirho::NotStartedChirho);
        lines_chirho.push(format!(
            "Done: {}, Partial: {}, TODO: {}, Pass rate: {:.1}%",
            done_chirho, partial_chirho, todo_chirho, self.pass_rate_chirho()
        ));
        lines_chirho.join("\n")
    }
}

/// Build the Haskell 2010 Report conformance tracker with current Haskeluya status.
pub fn haskell_2010_conformance_chirho() -> ConformanceTrackerChirho {
    use StatusChirho::*;

    let sections_chirho = vec![
        // Chapter 2: Lexical Structure
        sec_chirho("2.1", "Notational Conventions", DoneChirho, ""),
        sec_chirho("2.2", "Lexical Program Structure", DoneChirho, "lexer + layout rule"),
        sec_chirho("2.3", "Comments", DoneChirho, "line and block comments"),
        sec_chirho("2.4", "Identifiers and Operators", DoneChirho, "qualified names, operators"),
        sec_chirho("2.5", "Numeric Literals", DoneChirho, "int, float, hex, octal"),
        sec_chirho("2.6", "Character and String Literals", DoneChirho, "escape sequences"),
        sec_chirho("2.7", "Layout", DoneChirho, "offside rule, brace insertion"),

        // Chapter 3: Expressions
        sec_chirho("3.1", "Errors", PartialChirho, "error function works, undefined partial"),
        sec_chirho("3.2", "Variables, Constructors, Operators, Literals", DoneChirho, ""),
        sec_chirho("3.3", "Curried Applications and Lambda Abstractions", DoneChirho, ""),
        sec_chirho("3.4", "Operator Applications", DoneChirho, "sections, fixity"),
        sec_chirho("3.5", "Sections", DoneChirho, "left and right sections"),
        sec_chirho("3.6", "Conditionals", DoneChirho, "if-then-else"),
        sec_chirho("3.7", "Lists", DoneChirho, "list literals, cons, concat"),
        sec_chirho("3.8", "Tuples", DoneChirho, "pair tuples, fst, snd"),
        sec_chirho("3.9", "Unit Expressions and Parenthesized Expressions", DoneChirho, ""),
        sec_chirho("3.10", "Arithmetic Sequences", DoneChirho, "[1..], [1,3..], [1..10], [1,3..10]"),
        sec_chirho("3.11", "List Comprehensions", DoneChirho, "generators, guards, let"),
        sec_chirho("3.12", "Let Expressions", DoneChirho, "let/in, where"),
        sec_chirho("3.13", "Case Expressions", DoneChirho, "with exhaustiveness checking"),
        sec_chirho("3.14", "Do Expressions", DoneChirho, "bind, return, let in do"),
        sec_chirho("3.15", "Datatypes with Field Labels", DoneChirho, "record construction, update, pattern match"),
        sec_chirho("3.16", "Expression Type-Signatures", DoneChirho, ":: annotations"),
        sec_chirho("3.17", "Pattern Matching", DoneChirho, "constructors, literals, wildcards, as-patterns, guards"),

        // Chapter 4: Declarations and Bindings
        sec_chirho("4.1", "Overview of Types and Classes", DoneChirho, ""),
        sec_chirho("4.2", "User-Defined Datatypes", DoneChirho, "data, newtype, type aliases"),
        sec_chirho("4.3", "Type Classes and Overloading", DoneChirho, "class, instance, default methods, deriving"),
        sec_chirho("4.4", "Nested Declarations", DoneChirho, "type sigs, fixity, function/pattern bindings"),
        sec_chirho("4.5", "Static Semantics of Function and Pattern Bindings", DoneChirho, ""),
        sec_chirho("4.6", "Kind Inference", DoneChirho, "kind inference with unification"),

        // Chapter 5: Modules
        sec_chirho("5.1", "Module Structure", DoneChirho, "module header, export list"),
        sec_chirho("5.2", "Export Lists", DoneChirho, "var, tycon(..), module re-export"),
        sec_chirho("5.3", "Import Declarations", DoneChirho, "qualified, as, hiding, import specs"),
        sec_chirho("5.4", "Importing and Exporting Instance Declarations", PartialChirho, "instances auto-export, orphan detection done"),
        sec_chirho("5.5", "Name Clashes and Closure", PartialChirho, "qualified disambiguation works"),
        sec_chirho("5.6", "Standard Prelude", DoneChirho, "automatic Prelude import, NoImplicitPrelude"),
        sec_chirho("5.7", "Separate Compilation", DoneChirho, "multi-module, incremental, hierarchical"),

        // Chapter 6: Predefined Types and Classes
        sec_chirho("6.1", "Standard Haskell Types", DoneChirho, "Bool, Char, Int, Integer, Float, Double, Maybe, Either, Ordering, tuples, lists"),
        sec_chirho("6.2", "Strict Evaluation", DoneChirho, "seq, $!, $!!, deepseq, force, evaluate"),
        sec_chirho("6.3", "Standard Haskell Classes", DoneChirho, "Eq, Ord, Show, Read, Num, Enum, Bounded, Functor, Foldable, Traversable"),
        sec_chirho("6.4", "Numbers", PartialChirho, "Int/Double done, Integer/Rational partial"),

        // Chapter 7: Basic I/O
        sec_chirho("7.1", "Standard I/O Functions", DoneChirho, "putStr, putStrLn, print, getLine, getChar"),
        sec_chirho("7.2", "Sequencing I/O Operations", DoneChirho, "do notation, >>= , >>"),
        sec_chirho("7.3", "Exception Handling in the I/O Monad", DoneChirho, "catch, throw, try, bracket, finally"),

        // Chapter 8: Foreign Function Interface (addendum)
        sec_chirho("8.1", "Foreign Declarations", DoneChirho, "foreign import ccall, foreign export ccall"),
        sec_chirho("8.2", "Marshalling", NotStartedChirho, "Storable, Ptr, ForeignPtr not yet implemented"),
        sec_chirho("8.3", "Foreign Types", NotStartedChirho, "CInt, CDouble, etc. not yet mapped"),
    ];

    ConformanceTrackerChirho { sections_chirho }
}

fn sec_chirho(
    section_chirho: &str,
    title_chirho: &str,
    status_chirho: StatusChirho,
    notes_chirho: &str,
) -> ReportSectionChirho {
    ReportSectionChirho {
        section_chirho: section_chirho.to_string(),
        title_chirho: title_chirho.to_string(),
        status_chirho,
        notes_chirho: notes_chirho.to_string(),
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn conformance_tracker_has_sections_chirho() {
        let tracker_chirho = haskell_2010_conformance_chirho();
        assert!(
            tracker_chirho.sections_chirho.len() >= 35,
            "should have at least 35 sections, got {}",
            tracker_chirho.sections_chirho.len()
        );
    }

    #[test]
    fn conformance_pass_rate_positive_chirho() {
        let tracker_chirho = haskell_2010_conformance_chirho();
        let rate_chirho = tracker_chirho.pass_rate_chirho();
        assert!(
            rate_chirho > 50.0,
            "pass rate should be above 50%, got {:.1}%",
            rate_chirho
        );
    }

    #[test]
    fn conformance_done_count_chirho() {
        let tracker_chirho = haskell_2010_conformance_chirho();
        let done_chirho = tracker_chirho.count_by_status_chirho(StatusChirho::DoneChirho);
        assert!(
            done_chirho >= 25,
            "should have at least 25 DONE sections, got {}",
            done_chirho
        );
    }

    #[test]
    fn conformance_summary_table_format_chirho() {
        let tracker_chirho = haskell_2010_conformance_chirho();
        let table_chirho = tracker_chirho.summary_table_chirho();
        assert!(table_chirho.contains("Section"));
        assert!(table_chirho.contains("DONE"));
        assert!(table_chirho.contains("Pass rate"));
    }

    #[test]
    fn conformance_count_by_status_chirho() {
        let tracker_chirho = ConformanceTrackerChirho {
            sections_chirho: vec![
                sec_chirho("1.0", "A", StatusChirho::DoneChirho, ""),
                sec_chirho("2.0", "B", StatusChirho::PartialChirho, ""),
                sec_chirho("3.0", "C", StatusChirho::NotStartedChirho, ""),
            ],
        };
        assert_eq!(tracker_chirho.count_by_status_chirho(StatusChirho::DoneChirho), 1);
        assert_eq!(tracker_chirho.count_by_status_chirho(StatusChirho::PartialChirho), 1);
        assert_eq!(tracker_chirho.count_by_status_chirho(StatusChirho::NotStartedChirho), 1);
    }

    #[test]
    fn conformance_pass_rate_computation_chirho() {
        let tracker_chirho = ConformanceTrackerChirho {
            sections_chirho: vec![
                sec_chirho("1.0", "A", StatusChirho::DoneChirho, ""),
                sec_chirho("2.0", "B", StatusChirho::PartialChirho, ""),
                sec_chirho("3.0", "C", StatusChirho::NotStartedChirho, ""),
                sec_chirho("4.0", "D", StatusChirho::DoneChirho, ""),
            ],
        };
        // Done=2 (2.0), Partial=1 (0.5), Total=4 → (2+0.5)/4 = 62.5%
        let rate_chirho = tracker_chirho.pass_rate_chirho();
        assert!((rate_chirho - 62.5).abs() < 0.1);
    }

    #[test]
    fn conformance_empty_tracker_chirho() {
        let tracker_chirho = ConformanceTrackerChirho {
            sections_chirho: vec![],
        };
        assert_eq!(tracker_chirho.pass_rate_chirho(), 100.0);
    }
}
