// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The two spellings of the Prelude `String` synonym. The checker registers an
//! instance by its head as typed, the synonym expanded (`instance TagChirho
//! [String]` is `$fTagChirho[[Char]]`), while evidence keys spell a nested
//! string `String`, as `Show`'s own rows do. Every lookup that crosses between
//! the two spellings uses the one rule here.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::DictPassCtxChirho;
use crate::expr_chirho::CoreIdChirho;

impl DictPassCtxChirho {
    /// The source row that fills a ground dictionary's method slot when the
    /// instance head was written with `String`. The desugarer names an
    /// instance's method rows by the head AS WRITTEN
    /// (`$prim_TagChirho_tagChirho_[String]`, `$prim_TagChirho_tagChirho_String`),
    /// the dictionary by the head AS TYPED (`[[Char]]`, `[Char]`), so the exact
    /// row name missed and the slot became a missing-method stub (agy_chirho
    /// #27325). A row qualifies when its written key, spelled out, IS the typed
    /// key, so only a typed key holding `[Char]` can have one; two qualifying
    /// rows would be duplicate instances, and the lowest id keeps the choice
    /// independent of map order. Still missed: another synonym
    /// (`type NameChirho = [Char]`, invisible to the desugarer), an
    /// application head (written `Maybe Int`, typed `(Maybe Int)`), and a
    /// later argument of a multi-parameter head (`Int_String` is one word).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn written_method_row_chirho(
        &self,
        class_name_chirho: &str,
        method_name_chirho: &str,
        type_key_chirho: &str,
    ) -> Option<CoreIdChirho> {
        if !type_key_chirho.contains("[Char]") {
            return None;
        }
        let row_prefix_chirho = format!("$prim_{class_name_chirho}_{method_name_chirho}_");
        self.body_backed_names_chirho
            .iter()
            .filter_map(|(row_name_chirho, row_id_chirho)| {
                let written_key_chirho = row_name_chirho.strip_prefix(&row_prefix_chirho)?;
                (Self::string_spelled_out_chirho(written_key_chirho) == type_key_chirho)
                    .then_some(*row_id_chirho)
            })
            .min()
    }

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
