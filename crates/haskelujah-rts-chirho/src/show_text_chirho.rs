// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC's `show` for `Char` and `String`, shared by the interpreter and the native RTS
//! (row 557). Both follow `showLitChar` from GHC's `GHC.Show`:
//! - A printable ASCII character stands for itself, and a backslash doubles.
//! - Seven control characters take one-letter escapes (`\n`, `\t`, ...); the others take
//!   their ASCII names (`\NUL`, `\SOH`, ...), and DEL is `\DEL`.
//! - Every code point above DEL is a decimal escape (`\233`).
//! - `\&` follows a decimal escape when a digit comes next, and `\SO` when an `H` comes
//!   next, so the text reads back as the same string.
//!
//! A `String` also escapes `"`, and a `Char` escapes `'`. The input is code points, not
//! Rust `char`s, because a Haskell `Char` may be a surrogate.
//! workflow: print-show-evidence-chirho

use std::fmt::Write;

/// The escape names of the code points below space, indexed by code point.
const CONTROL_ESCAPES_CHIRHO: [&str; 32] = [
    "NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "a", "b", "t", "n", "v", "f", "r", "SO", "SI",
    "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB", "CAN", "EM", "SUB", "ESC", "FS", "GS",
    "RS", "US",
];

/// GHC's `showLitChar` for one code point, given the code point shown right after it.
fn push_lit_code_point_chirho(
    out_chirho: &mut String,
    code_point_chirho: u32,
    next_chirho: Option<u32>,
) {
    match code_point_chirho {
        0x00..=0x1f => {
            out_chirho.push('\\');
            out_chirho.push_str(CONTROL_ESCAPES_CHIRHO[code_point_chirho as usize]);
            if code_point_chirho == 0x0e && next_chirho == Some(u32::from(b'H')) {
                out_chirho.push_str("\\&");
            }
        }
        0x5c => out_chirho.push_str("\\\\"),
        0x20..=0x5b | 0x5d..=0x7e => out_chirho.push(char::from(code_point_chirho as u8)),
        0x7f => out_chirho.push_str("\\DEL"),
        _ => {
            let _ = write!(out_chirho, "\\{code_point_chirho}");
            if next_chirho.is_some_and(|next_chirho| (0x30..=0x39).contains(&next_chirho)) {
                out_chirho.push_str("\\&");
            }
        }
    }
}

/// GHC's `show` for a `Char`: `'x'`, `'\''`, `'\n'`, `'\1234'`.
/// workflow: print-show-evidence-chirho
pub fn show_char_chirho(code_point_chirho: u32) -> String {
    let mut out_chirho = String::with_capacity(8);
    out_chirho.push('\'');
    if code_point_chirho == u32::from(b'\'') {
        out_chirho.push_str("\\'");
    } else {
        // The closing quote comes next, so no `\&` is ever needed.
        push_lit_code_point_chirho(&mut out_chirho, code_point_chirho, None);
    }
    out_chirho.push('\'');
    out_chirho
}

/// GHC's `show` for a `String` given as code points: `"a\"b"`, `"\1234\&5"`.
/// workflow: print-show-evidence-chirho
pub fn show_string_chirho(code_points_chirho: impl IntoIterator<Item = u32>) -> String {
    let mut code_points_chirho = code_points_chirho.into_iter().peekable();
    let mut out_chirho = String::with_capacity(code_points_chirho.size_hint().0 + 2);
    out_chirho.push('"');
    while let Some(code_point_chirho) = code_points_chirho.next() {
        if code_point_chirho == u32::from(b'"') {
            out_chirho.push_str("\\\"");
        } else {
            let next_chirho = code_points_chirho.peek().copied();
            push_lit_code_point_chirho(&mut out_chirho, code_point_chirho, next_chirho);
        }
    }
    out_chirho.push('"');
    out_chirho
}
