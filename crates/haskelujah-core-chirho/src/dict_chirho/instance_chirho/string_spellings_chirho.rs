// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The two spellings of the Prelude `String` synonym. The checker registers an
//! instance by its head as typed, the synonym expanded (`instance TagChirho
//! [String]` is `$fTagChirho[[Char]]`), while evidence keys spell a nested
//! string `String`, as `Show`'s own rows do. Every lookup that crosses between
//! the two spellings uses the one rule here.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::DictPassCtxChirho;

impl DictPassCtxChirho {
    /// Each whole-word `String` in a key spelled `[Char]`: `[String]` →
    /// `[[Char]]`, `Maybe String` → `Maybe [Char]`. A word is a maximal run of
    /// identifier characters (letters, digits, `_`, `'`, `.`), so `ByteString`
    /// or `StringBoxChirho` is never touched.
    pub(in crate::dict_chirho) fn string_spelled_out_chirho(key_chirho: &str) -> String {
        let mut spelled_chirho = String::with_capacity(key_chirho.len() + 8);
        let mut word_chirho = String::new();
        let identifier_char_chirho =
            |ch_chirho: char| ch_chirho.is_alphanumeric() || matches!(ch_chirho, '_' | '\'' | '.');
        for ch_chirho in key_chirho.chars().chain(std::iter::once(' ')) {
            if identifier_char_chirho(ch_chirho) {
                word_chirho.push(ch_chirho);
                continue;
            }
            if word_chirho == "String" {
                spelled_chirho.push_str("[Char]");
            } else {
                spelled_chirho.push_str(&word_chirho);
            }
            word_chirho.clear();
            spelled_chirho.push(ch_chirho);
        }
        spelled_chirho.pop();
        spelled_chirho
    }
}
