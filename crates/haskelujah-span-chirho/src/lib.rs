// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-span-chirho
//!
//! Foundation crate for source location tracking in the Haskelujah compiler.
//! Provides file identifiers, byte offsets, and spans that all other crates
//! use for diagnostics, error reporting, and source mapping.
//!
//! Zero external dependencies by design — this crate is the root of the
//! dependency graph and must compile fast.

use std::fmt;

// ---------------------------------------------------------------------------
// FileIdChirho — interned file identifier
// ---------------------------------------------------------------------------

/// A lightweight, copyable identifier for a source file.
///
/// File IDs are handed out by a [`SourceMapChirho`] and used inside
/// [`SpanChirho`] values so that spans stay small (two `u32`s + a file id).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileIdChirho(u32);

impl FileIdChirho {
    /// A sentinel value used when no real file is available (e.g. compiler-
    /// generated code or REPL input that hasn't been assigned a file yet).
    pub const SYNTHETIC_CHIRHO: Self = Self(u32::MAX);

    /// Raw numeric value — useful for serialization or arena indexing.
    #[inline]
    pub fn as_raw_chirho(self) -> u32 {
        self.0
    }
}

impl fmt::Debug for FileIdChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::SYNTHETIC_CHIRHO {
            write!(f_chirho, "FileIdChirho(SYNTHETIC)")
        } else {
            write!(f_chirho, "FileIdChirho({})", self.0)
        }
    }
}

// ---------------------------------------------------------------------------
// ByteOffsetChirho — absolute byte position within a file
// ---------------------------------------------------------------------------

/// An absolute byte offset from the start of a source file.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ByteOffsetChirho(u32);

impl ByteOffsetChirho {
    /// Construct from a raw `u32`.
    #[inline]
    pub const fn new_chirho(raw_chirho: u32) -> Self {
        Self(raw_chirho)
    }

    /// Construct from a `usize`, saturating to `u32::MAX`.
    #[inline]
    pub fn from_usize_chirho(value_chirho: usize) -> Self {
        Self(u32::try_from(value_chirho).unwrap_or(u32::MAX))
    }

    /// Raw numeric value.
    #[inline]
    pub const fn as_u32_chirho(self) -> u32 {
        self.0
    }

    /// Convert to `usize` for indexing into string slices.
    #[inline]
    pub const fn as_usize_chirho(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Debug for ByteOffsetChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "ByteOffset({})", self.0)
    }
}

impl fmt::Display for ByteOffsetChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// SpanChirho — a contiguous byte range within a file
// ---------------------------------------------------------------------------

/// A contiguous range of bytes `[start, end)` within a source file, identified
/// by a [`FileIdChirho`].
///
/// Spans are the primary currency for diagnostics. Every AST node, token, and
/// diagnostic label carries a span so that error messages can point to the
/// exact source location.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpanChirho {
    file_id_chirho: FileIdChirho,
    start_chirho: ByteOffsetChirho,
    end_chirho: ByteOffsetChirho,
}

impl SpanChirho {
    /// Create a span covering `[start, end)` in the given file.
    #[inline]
    pub fn new_chirho(
        file_id_chirho: FileIdChirho,
        start_chirho: ByteOffsetChirho,
        end_chirho: ByteOffsetChirho,
    ) -> Self {
        debug_assert!(
            start_chirho <= end_chirho,
            "SpanChirho::new_chirho requires start <= end"
        );
        Self {
            file_id_chirho,
            start_chirho,
            end_chirho,
        }
    }

    /// A zero-width span used as a placeholder when no real location exists.
    pub const DUMMY_CHIRHO: Self = Self {
        file_id_chirho: FileIdChirho::SYNTHETIC_CHIRHO,
        start_chirho: ByteOffsetChirho::new_chirho(0),
        end_chirho: ByteOffsetChirho::new_chirho(0),
    };

    /// The file this span belongs to.
    #[inline]
    pub const fn file_id_chirho(self) -> FileIdChirho {
        self.file_id_chirho
    }

    /// Byte offset of the first byte in the span.
    #[inline]
    pub const fn start_chirho(self) -> ByteOffsetChirho {
        self.start_chirho
    }

    /// Byte offset one past the last byte in the span.
    #[inline]
    pub const fn end_chirho(self) -> ByteOffsetChirho {
        self.end_chirho
    }

    /// Whether this span satisfies the core invariant `start <= end`.
    #[inline]
    pub const fn is_valid_chirho(self) -> bool {
        self.start_chirho.0 <= self.end_chirho.0
    }

    /// Length in bytes.
    #[inline]
    pub const fn len_chirho(self) -> u32 {
        self.end_chirho.0.saturating_sub(self.start_chirho.0)
    }

    /// Whether the span covers zero bytes.
    #[inline]
    pub const fn is_empty_chirho(self) -> bool {
        self.start_chirho.0 == self.end_chirho.0
    }

    /// Whether this span fully contains `other` (same file, start <=, end >=).
    #[inline]
    pub fn contains_chirho(self, other_chirho: SpanChirho) -> bool {
        self.file_id_chirho == other_chirho.file_id_chirho
            && self.start_chirho <= other_chirho.start_chirho
            && self.end_chirho >= other_chirho.end_chirho
    }

    /// Merge two spans in the same file into the smallest span covering both.
    /// Returns `None` if the spans belong to different files.
    #[inline]
    pub fn merge_chirho(self, other_chirho: SpanChirho) -> Option<SpanChirho> {
        if self.file_id_chirho != other_chirho.file_id_chirho {
            return None;
        }
        let start_chirho = if self.start_chirho < other_chirho.start_chirho {
            self.start_chirho
        } else {
            other_chirho.start_chirho
        };
        let end_chirho = if self.end_chirho > other_chirho.end_chirho {
            self.end_chirho
        } else {
            other_chirho.end_chirho
        };
        Some(SpanChirho::new_chirho(
            self.file_id_chirho,
            start_chirho,
            end_chirho,
        ))
    }

    /// Shrink or relocate the span. Useful when desugaring produces a node
    /// that should point at a sub-range of the original.
    #[inline]
    pub const fn subspan_chirho(
        self,
        relative_start_chirho: u32,
        relative_end_chirho: u32,
    ) -> SpanChirho {
        SpanChirho {
            file_id_chirho: self.file_id_chirho,
            start_chirho: ByteOffsetChirho::new_chirho(self.start_chirho.0 + relative_start_chirho),
            end_chirho: ByteOffsetChirho::new_chirho(self.start_chirho.0 + relative_end_chirho),
        }
    }

    /// Clamp the span end to the available source length while preserving the
    /// `start <= end` invariant.
    #[inline]
    pub fn clamp_to_source_chirho(self, source_len_chirho: usize) -> SpanChirho {
        let source_end_chirho = ByteOffsetChirho::from_usize_chirho(source_len_chirho);
        let clamped_end_chirho = if self.end_chirho < source_end_chirho {
            self.end_chirho
        } else {
            source_end_chirho
        };
        let clamped_end_chirho = if clamped_end_chirho < self.start_chirho {
            self.start_chirho
        } else {
            clamped_end_chirho
        };
        SpanChirho::new_chirho(self.file_id_chirho, self.start_chirho, clamped_end_chirho)
    }

    /// Extract the spanned text from source content.
    /// Returns `None` if the byte range is out of bounds.
    #[inline]
    pub fn text_chirho<'a>(&self, source_chirho: &'a str) -> Option<&'a str> {
        let start_chirho = self.start_chirho.as_usize_chirho();
        let end_chirho = self.end_chirho.as_usize_chirho();
        source_chirho.get(start_chirho..end_chirho)
    }
}

impl fmt::Debug for SpanChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f_chirho,
            "Span({:?}, {}..{})",
            self.file_id_chirho, self.start_chirho.0, self.end_chirho.0
        )
    }
}

impl fmt::Display for SpanChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{}..{}", self.start_chirho.0, self.end_chirho.0)
    }
}

// ---------------------------------------------------------------------------
// LineIndexChirho — maps byte offsets to line/column numbers
// ---------------------------------------------------------------------------

/// A precomputed index of newline positions in a source file, enabling
/// efficient byte-offset → line/column conversion for diagnostics.
#[derive(Debug, Clone)]
pub struct LineIndexChirho {
    /// Byte offsets of the start of each line (line 0 starts at offset 0).
    line_starts_chirho: Vec<ByteOffsetChirho>,
}

/// A human-readable line and column number (both 1-based).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineColChirho {
    /// 1-based line number.
    pub line_chirho: u32,
    /// 1-based column number (byte offset from line start + 1).
    pub col_chirho: u32,
}

impl LineIndexChirho {
    /// Build a line index from source text.
    pub fn new_chirho(source_chirho: &str) -> Self {
        let mut line_starts_chirho = vec![ByteOffsetChirho::new_chirho(0)];
        for (byte_index_chirho, byte_chirho) in source_chirho.bytes().enumerate() {
            if byte_chirho == b'\n' {
                line_starts_chirho.push(ByteOffsetChirho::from_usize_chirho(byte_index_chirho + 1));
            }
        }
        Self { line_starts_chirho }
    }

    /// Total number of lines in the source.
    #[inline]
    pub fn line_count_chirho(&self) -> usize {
        self.line_starts_chirho.len()
    }

    /// Convert a byte offset to a 1-based line and column.
    pub fn line_col_chirho(&self, offset_chirho: ByteOffsetChirho) -> LineColChirho {
        let line_index_chirho = self
            .line_starts_chirho
            .partition_point(|&start_chirho| start_chirho <= offset_chirho)
            .saturating_sub(1);
        let line_start_chirho = self.line_starts_chirho[line_index_chirho];
        LineColChirho {
            line_chirho: (line_index_chirho as u32) + 1,
            col_chirho: offset_chirho
                .as_u32_chirho()
                .saturating_sub(line_start_chirho.as_u32_chirho())
                + 1,
        }
    }

    /// Get the byte offset of the start of a 1-based line number.
    /// Returns `None` if the line number is out of range.
    pub fn line_start_chirho(&self, line_number_chirho: u32) -> Option<ByteOffsetChirho> {
        let index_chirho = line_number_chirho.checked_sub(1)? as usize;
        self.line_starts_chirho.get(index_chirho).copied()
    }
}

// ---------------------------------------------------------------------------
// SourceMapChirho — file registry
// ---------------------------------------------------------------------------

/// Registry that assigns [`FileIdChirho`] values and stores file names and
/// contents. All source files loaded by the compiler are registered here so
/// that any span can be resolved back to file name + line/column.
#[derive(Debug, Default)]
pub struct SourceMapChirho {
    files_chirho: Vec<SourceEntryChirho>,
}

/// A single entry in the source map.
#[derive(Debug)]
struct SourceEntryChirho {
    name_chirho: String,
    source_chirho: String,
    line_index_chirho: LineIndexChirho,
}

impl SourceMapChirho {
    /// Create an empty source map.
    pub fn new_chirho() -> Self {
        Self::default()
    }

    /// Register a file and get back its [`FileIdChirho`].
    pub fn add_file_chirho(
        &mut self,
        name_chirho: impl Into<String>,
        source_chirho: impl Into<String>,
    ) -> FileIdChirho {
        let source_str_chirho = source_chirho.into();
        let line_index_chirho = LineIndexChirho::new_chirho(&source_str_chirho);
        let id_chirho = FileIdChirho(self.files_chirho.len() as u32);
        self.files_chirho.push(SourceEntryChirho {
            name_chirho: name_chirho.into(),
            source_chirho: source_str_chirho,
            line_index_chirho,
        });
        id_chirho
    }

    /// Look up the file name for a given ID.
    pub fn file_name_chirho(&self, id_chirho: FileIdChirho) -> Option<&str> {
        self.files_chirho
            .get(id_chirho.0 as usize)
            .map(|entry_chirho| entry_chirho.name_chirho.as_str())
    }

    /// Look up the full source text for a given ID.
    pub fn file_source_chirho(&self, id_chirho: FileIdChirho) -> Option<&str> {
        self.files_chirho
            .get(id_chirho.0 as usize)
            .map(|entry_chirho| entry_chirho.source_chirho.as_str())
    }

    /// Look up the line index for a given ID.
    pub fn line_index_chirho(&self, id_chirho: FileIdChirho) -> Option<&LineIndexChirho> {
        self.files_chirho
            .get(id_chirho.0 as usize)
            .map(|entry_chirho| &entry_chirho.line_index_chirho)
    }

    /// Resolve a span to file name + line/column for the start position.
    pub fn resolve_span_start_chirho(
        &self,
        span_chirho: SpanChirho,
    ) -> Option<(&str, LineColChirho)> {
        let entry_chirho = self
            .files_chirho
            .get(span_chirho.file_id_chirho.0 as usize)?;
        let line_col_chirho = entry_chirho
            .line_index_chirho
            .line_col_chirho(span_chirho.start_chirho);
        Some((&entry_chirho.name_chirho, line_col_chirho))
    }

    /// Extract the source text covered by a span.
    pub fn span_text_chirho(&self, span_chirho: SpanChirho) -> Option<&str> {
        let source_chirho = self.file_source_chirho(span_chirho.file_id_chirho)?;
        span_chirho.text_chirho(source_chirho)
    }

    /// Number of registered files.
    #[inline]
    pub fn file_count_chirho(&self) -> usize {
        self.files_chirho.len()
    }
}

// ---------------------------------------------------------------------------
// Spanned<T> — a value annotated with its source span
// ---------------------------------------------------------------------------

/// A value paired with the source span where it was found.
///
/// This is the standard wrapper for attaching location info to parsed nodes,
/// identifiers, literals, etc.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpannedChirho<T> {
    /// The wrapped value.
    pub node_chirho: T,
    /// The source span associated with the wrapped value.
    pub span_chirho: SpanChirho,
}

impl<T> SpannedChirho<T> {
    /// Construct a spanned wrapper from a value and its source span.
    #[inline]
    pub const fn new_chirho(node_chirho: T, span_chirho: SpanChirho) -> Self {
        Self {
            node_chirho,
            span_chirho,
        }
    }

    /// Map the inner value while preserving the span.
    #[inline]
    pub fn map_chirho<U>(self, f_chirho: impl FnOnce(T) -> U) -> SpannedChirho<U> {
        SpannedChirho {
            node_chirho: f_chirho(self.node_chirho),
            span_chirho: self.span_chirho,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for SpannedChirho<T> {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{:?} @ {:?}", self.node_chirho, self.span_chirho)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn file_id_debug_chirho() {
        let id_chirho = FileIdChirho(0);
        assert_eq!(format!("{id_chirho:?}"), "FileIdChirho(0)");
        assert_eq!(
            format!("{:?}", FileIdChirho::SYNTHETIC_CHIRHO),
            "FileIdChirho(SYNTHETIC)"
        );
    }

    #[test]
    fn span_basics_chirho() {
        let file_chirho = FileIdChirho(0);
        let span_chirho = SpanChirho::new_chirho(
            file_chirho,
            ByteOffsetChirho::new_chirho(5),
            ByteOffsetChirho::new_chirho(10),
        );
        assert_eq!(span_chirho.len_chirho(), 5);
        assert!(!span_chirho.is_empty_chirho());
        assert!(span_chirho.is_valid_chirho());
        assert_eq!(span_chirho.file_id_chirho(), file_chirho);
    }

    #[test]
    fn span_empty_chirho() {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        assert!(span_chirho.is_empty_chirho());
        assert_eq!(span_chirho.len_chirho(), 0);
    }

    #[test]
    fn span_merge_same_file_chirho() {
        let file_chirho = FileIdChirho(0);
        let a_chirho = SpanChirho::new_chirho(
            file_chirho,
            ByteOffsetChirho::new_chirho(2),
            ByteOffsetChirho::new_chirho(5),
        );
        let b_chirho = SpanChirho::new_chirho(
            file_chirho,
            ByteOffsetChirho::new_chirho(8),
            ByteOffsetChirho::new_chirho(12),
        );
        let merged_chirho = a_chirho.merge_chirho(b_chirho).unwrap();
        assert_eq!(
            merged_chirho.start_chirho(),
            ByteOffsetChirho::new_chirho(2)
        );
        assert_eq!(merged_chirho.end_chirho(), ByteOffsetChirho::new_chirho(12));
    }

    #[test]
    fn span_merge_different_files_chirho() {
        let a_chirho = SpanChirho::new_chirho(
            FileIdChirho(0),
            ByteOffsetChirho::new_chirho(0),
            ByteOffsetChirho::new_chirho(5),
        );
        let b_chirho = SpanChirho::new_chirho(
            FileIdChirho(1),
            ByteOffsetChirho::new_chirho(0),
            ByteOffsetChirho::new_chirho(5),
        );
        assert!(a_chirho.merge_chirho(b_chirho).is_none());
    }

    #[test]
    fn span_contains_chirho() {
        let file_chirho = FileIdChirho(0);
        let outer_chirho = SpanChirho::new_chirho(
            file_chirho,
            ByteOffsetChirho::new_chirho(0),
            ByteOffsetChirho::new_chirho(20),
        );
        let inner_chirho = SpanChirho::new_chirho(
            file_chirho,
            ByteOffsetChirho::new_chirho(5),
            ByteOffsetChirho::new_chirho(15),
        );
        assert!(outer_chirho.contains_chirho(inner_chirho));
        assert!(!inner_chirho.contains_chirho(outer_chirho));
    }

    #[test]
    fn span_text_extraction_chirho() {
        let source_chirho = "module Main where";
        let file_chirho = FileIdChirho(0);
        let span_chirho = SpanChirho::new_chirho(
            file_chirho,
            ByteOffsetChirho::new_chirho(7),
            ByteOffsetChirho::new_chirho(11),
        );
        assert_eq!(span_chirho.text_chirho(source_chirho), Some("Main"));
    }

    #[test]
    fn subspan_chirho() {
        let file_chirho = FileIdChirho(0);
        let span_chirho = SpanChirho::new_chirho(
            file_chirho,
            ByteOffsetChirho::new_chirho(10),
            ByteOffsetChirho::new_chirho(20),
        );
        let sub_chirho = span_chirho.subspan_chirho(2, 5);
        assert_eq!(sub_chirho.start_chirho(), ByteOffsetChirho::new_chirho(12));
        assert_eq!(sub_chirho.end_chirho(), ByteOffsetChirho::new_chirho(15));
    }

    #[test]
    fn span_clamp_to_source_chirho() {
        let span_chirho = SpanChirho::new_chirho(
            FileIdChirho(0),
            ByteOffsetChirho::new_chirho(3),
            ByteOffsetChirho::new_chirho(12),
        );
        let clamped_span_chirho = span_chirho.clamp_to_source_chirho(8);
        assert!(clamped_span_chirho.is_valid_chirho());
        assert_eq!(
            clamped_span_chirho.end_chirho(),
            ByteOffsetChirho::new_chirho(8)
        );
    }

    #[test]
    fn merge_contains_inputs_property_chirho() {
        let file_chirho = FileIdChirho(0);
        for start_a_raw_chirho in 0_u32..=8 {
            for end_a_raw_chirho in start_a_raw_chirho..=8 {
                let span_a_chirho = SpanChirho::new_chirho(
                    file_chirho,
                    ByteOffsetChirho::new_chirho(start_a_raw_chirho),
                    ByteOffsetChirho::new_chirho(end_a_raw_chirho),
                );

                for start_b_raw_chirho in 0_u32..=8 {
                    for end_b_raw_chirho in start_b_raw_chirho..=8 {
                        let span_b_chirho = SpanChirho::new_chirho(
                            file_chirho,
                            ByteOffsetChirho::new_chirho(start_b_raw_chirho),
                            ByteOffsetChirho::new_chirho(end_b_raw_chirho),
                        );
                        let merged_span_chirho = span_a_chirho.merge_chirho(span_b_chirho).unwrap();
                        assert!(merged_span_chirho.contains_chirho(span_a_chirho));
                        assert!(merged_span_chirho.contains_chirho(span_b_chirho));
                    }
                }
            }
        }
    }

    #[test]
    fn subspan_stays_within_parent_property_chirho() {
        let parent_span_chirho = SpanChirho::new_chirho(
            FileIdChirho(0),
            ByteOffsetChirho::new_chirho(10),
            ByteOffsetChirho::new_chirho(20),
        );
        let parent_len_chirho = parent_span_chirho.len_chirho();

        for relative_start_chirho in 0_u32..=parent_len_chirho {
            for relative_end_chirho in relative_start_chirho..=parent_len_chirho {
                let subspan_chirho =
                    parent_span_chirho.subspan_chirho(relative_start_chirho, relative_end_chirho);
                assert!(parent_span_chirho.contains_chirho(subspan_chirho));
            }
        }
    }

    #[test]
    fn text_returns_none_for_out_of_bounds_spans_property_chirho() {
        let source_chirho = "module Main where";
        let file_chirho = FileIdChirho(0);
        let source_len_chirho = source_chirho.len() as u32;

        for start_raw_chirho in 0_u32..=source_len_chirho + 2 {
            for extra_len_chirho in 1_u32..=3 {
                let end_raw_chirho = start_raw_chirho + extra_len_chirho;
                if end_raw_chirho <= source_len_chirho {
                    continue;
                }

                let span_chirho = SpanChirho::new_chirho(
                    file_chirho,
                    ByteOffsetChirho::new_chirho(start_raw_chirho),
                    ByteOffsetChirho::new_chirho(end_raw_chirho),
                );
                assert_eq!(span_chirho.text_chirho(source_chirho), None);
            }
        }
    }

    #[test]
    fn line_index_single_line_chirho() {
        let index_chirho = LineIndexChirho::new_chirho("hello world");
        assert_eq!(index_chirho.line_count_chirho(), 1);
        let lc_chirho = index_chirho.line_col_chirho(ByteOffsetChirho::new_chirho(6));
        assert_eq!(lc_chirho.line_chirho, 1);
        assert_eq!(lc_chirho.col_chirho, 7);
    }

    #[test]
    fn line_index_multi_line_chirho() {
        let source_chirho = "line one\nline two\nline three\n";
        let index_chirho = LineIndexChirho::new_chirho(source_chirho);
        // "line one\n" -> newline at 8, "line two\n" -> newline at 17,
        // "line three\n" -> newline at 28. Line starts: [0, 9, 18, 29].
        // That's 3 content lines + 1 trailing empty line = 4 starts.
        // But our implementation counts every \n, giving us a start for
        // the empty trailing content after the final \n.
        let line_count_chirho = index_chirho.line_count_chirho();
        assert!(
            line_count_chirho == 4,
            "expected 4 line starts, got {line_count_chirho}"
        );

        // Start of line 2 (byte offset 9 = 'l' of "line two")
        let lc_chirho = index_chirho.line_col_chirho(ByteOffsetChirho::new_chirho(9));
        assert_eq!(lc_chirho.line_chirho, 2);
        assert_eq!(lc_chirho.col_chirho, 1);

        // 'r' in "three" on line 3 — byte 25
        let lc2_chirho = index_chirho.line_col_chirho(ByteOffsetChirho::new_chirho(25));
        assert_eq!(lc2_chirho.line_chirho, 3);
        assert_eq!(lc2_chirho.col_chirho, 8); // 25 - 18 + 1 = 8
    }

    #[test]
    fn source_map_basics_chirho() {
        let mut map_chirho = SourceMapChirho::new_chirho();
        let id_chirho =
            map_chirho.add_file_chirho("Main.hs", "module Main where\nmain = putStrLn \"hi\"\n");

        assert_eq!(map_chirho.file_count_chirho(), 1);
        assert_eq!(map_chirho.file_name_chirho(id_chirho), Some("Main.hs"));

        let span_chirho = SpanChirho::new_chirho(
            id_chirho,
            ByteOffsetChirho::new_chirho(7),
            ByteOffsetChirho::new_chirho(11),
        );
        assert_eq!(map_chirho.span_text_chirho(span_chirho), Some("Main"));

        let (name_chirho, lc_chirho) = map_chirho.resolve_span_start_chirho(span_chirho).unwrap();
        assert_eq!(name_chirho, "Main.hs");
        assert_eq!(lc_chirho.line_chirho, 1);
        assert_eq!(lc_chirho.col_chirho, 8);
    }

    #[test]
    fn spanned_map_chirho() {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        let spanned_chirho = SpannedChirho::new_chirho(42_i32, span_chirho);
        let mapped_chirho = spanned_chirho.map_chirho(|n_chirho| n_chirho.to_string());
        assert_eq!(mapped_chirho.node_chirho, "42");
        assert_eq!(mapped_chirho.span_chirho, span_chirho);
    }
}
