// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One dictionary-parameter decision for definitions and their callers.
//! A result-only variable is not sufficient proof of defaulting: the class
//! must have a permitted default AND that dictionary must actually exist.
//! Workflow: language-features-chirho/dictionary-evidence-chirho.md.

use super::DictPassCtxChirho;
use crate::expr_chirho::CoreIdChirho;
use haskelujah_typing_chirho::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho};

impl DictPassCtxChirho {
    pub(super) fn resolved_pred_dictionary_chirho(
        &self,
        predicate_chirho: &SchemePredChirho,
        scheme_chirho: &SchemeChirho,
    ) -> Option<CoreIdChirho> {
        let key_chirho = if matches!(predicate_chirho.ty_chirho, TyChirho::VarChirho(_)) {
            if !Self::is_defaultable_pred_chirho(predicate_chirho, scheme_chirho) {
                return None;
            }
            match predicate_chirho.class_name_chirho.as_str() {
                "Num" | "Eq" | "Ord" | "Show" | "Read" | "Enum" | "Bounded" | "Integral"
                | "Real" | "RealFrac" | "Floating" | "RealFloat" => "Int".to_string(),
                "IsString" => "[Char]".to_string(),
                "IsList" => "[t9037]".to_string(),
                _ => return None,
            }
        } else {
            format!("{}", predicate_chirho.ty_chirho)
        };
        self.instance_dicts_chirho
            .get(&(predicate_chirho.class_name_chirho.clone(), key_chirho))
            .copied()
    }

    pub(super) fn dict_param_classes_for_scheme_chirho(
        &self,
        scheme_chirho: &SchemeChirho,
    ) -> Vec<String> {
        scheme_chirho
            .preds_chirho
            .iter()
            .filter(|predicate_chirho| {
                self.resolved_pred_dictionary_chirho(predicate_chirho, scheme_chirho)
                    .is_none()
            })
            .map(|predicate_chirho| predicate_chirho.class_name_chirho.clone())
            .collect()
    }
}
