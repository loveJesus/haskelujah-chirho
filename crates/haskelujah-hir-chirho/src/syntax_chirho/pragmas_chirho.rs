// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Validate class pragma names against their actual signature/default owner.
use super::*;
use haskelujah_ast_chirho::decl_chirho::{
    ClassMethodChirho as AstMethodChirho, UnattachedMemberChirho, UnattachedMemberKindChirho,
};

fn identifier_chirho(text_chirho: &str) -> bool {
    text_chirho
        .chars()
        .next()
        .is_some_and(|character_chirho| character_chirho.is_alphabetic() || character_chirho == '_')
        && text_chirho.chars().all(|character_chirho| {
            character_chirho.is_alphanumeric()
                || character_chirho == '_'
                || character_chirho == '\''
        })
}
struct MinimalChirho<'source_chirho> {
    tokens_chirho: Vec<&'source_chirho str>,
    cursor_chirho: usize,
    names_chirho: Vec<&'source_chirho str>,
}
impl<'source_chirho> MinimalChirho<'source_chirho> {
    fn atom_chirho(&mut self, depth_chirho: usize) -> bool {
        if depth_chirho > 128 {
            return false;
        }
        let Some(token_chirho) = self.tokens_chirho.get(self.cursor_chirho).copied() else {
            return false;
        };
        self.cursor_chirho += 1;
        if token_chirho == "(" {
            if let (Some(operator_chirho), Some(&")")) = (
                self.tokens_chirho.get(self.cursor_chirho),
                self.tokens_chirho.get(self.cursor_chirho + 1),
            ) && !identifier_chirho(operator_chirho)
                && !matches!(*operator_chirho, "(" | ")" | ",")
            {
                self.names_chirho.push(operator_chirho);
                self.cursor_chirho += 2;
                return true;
            }
            if !self.expression_chirho(depth_chirho + 1)
                || self.tokens_chirho.get(self.cursor_chirho) != Some(&")")
            {
                return false;
            }
            self.cursor_chirho += 1;
            true
        } else if identifier_chirho(token_chirho) {
            self.names_chirho.push(token_chirho);
            true
        } else {
            false
        }
    }
    fn expression_chirho(&mut self, depth_chirho: usize) -> bool {
        if !self.atom_chirho(depth_chirho) {
            return false;
        }
        while self
            .tokens_chirho
            .get(self.cursor_chirho)
            .is_some_and(|token_chirho| matches!(*token_chirho, "," | "|"))
        {
            self.cursor_chirho += 1;
            if !self.atom_chirho(depth_chirho) {
                return false;
            }
        }
        true
    }
}
fn tokens_chirho(text_chirho: &str) -> Vec<&str> {
    let mut result_chirho = Vec::new();
    let mut start_chirho = None;
    for (offset_chirho, character_chirho) in text_chirho.char_indices() {
        if character_chirho.is_whitespace() || "(),|".contains(character_chirho) {
            if let Some(start_chirho) = start_chirho.take() {
                result_chirho.push(&text_chirho[start_chirho..offset_chirho]);
            }
            if !character_chirho.is_whitespace() {
                result_chirho
                    .push(&text_chirho[offset_chirho..offset_chirho + character_chirho.len_utf8()]);
            }
        } else if start_chirho.is_none() {
            start_chirho = Some(offset_chirho);
        }
    }
    if let Some(start_chirho) = start_chirho {
        result_chirho.push(&text_chirho[start_chirho..]);
    }
    result_chirho
}
impl LowerChirho<'_> {
    pub(in crate::lower_chirho) fn class_pragma_accounts_chirho(
        &mut self,
        accounts_chirho: &[UnattachedMemberChirho],
        methods_chirho: &[AstMethodChirho],
    ) -> ResultChirho<()> {
        let mut remaining_chirho = Vec::new();
        let mut minimal_seen_chirho = false;
        let mut inline_seen_chirho = HashSet::new();
        for account_chirho in accounts_chirho {
            if !matches!(
                account_chirho.kind_chirho,
                UnattachedMemberKindChirho::MinimalPragmaChirho
                    | UnattachedMemberKindChirho::InlinePragmaChirho
            ) {
                remaining_chirho.push(account_chirho.clone());
                continue;
            }
            let text_chirho = account_chirho.name_chirho.as_deref().ok_or_else(|| {
                self.invalid_class_chirho("pragma has no producer text", account_chirho.span_chirho)
            })?;
            self.charge_names_chirho(text_chirho.len() + 1, account_chirho.span_chirho)?;
            let tokens_chirho = tokens_chirho(text_chirho);
            let names_chirho =
                if account_chirho.kind_chirho == UnattachedMemberKindChirho::MinimalPragmaChirho {
                    if minimal_seen_chirho {
                        return Err(self.invalid_class_chirho(
                            "repeated MINIMAL pragma",
                            account_chirho.span_chirho,
                        ));
                    }
                    minimal_seen_chirho = true;
                    let mut parser_chirho = MinimalChirho {
                        tokens_chirho,
                        cursor_chirho: 0,
                        names_chirho: Vec::new(),
                    };
                    if !parser_chirho.tokens_chirho.is_empty()
                        && (!parser_chirho.expression_chirho(0)
                            || parser_chirho.cursor_chirho != parser_chirho.tokens_chirho.len())
                    {
                        return Err(self.error_chirho(
                            "MINIMAL formula syntax is not supported",
                            account_chirho.span_chirho,
                        ));
                    }
                    parser_chirho.names_chirho
                } else {
                    match tokens_chirho.as_slice() {
                        [name_chirho] if identifier_chirho(name_chirho) => vec![*name_chirho],
                        ["(", name_chirho, ")"] => vec![*name_chirho],
                        _ => {
                            return Err(self.error_chirho(
                                "next requires a plain method INLINE pragma",
                                account_chirho.span_chirho,
                            ));
                        }
                    }
                };
            for name_chirho in names_chirho {
                let method_chirho = methods_chirho
                    .iter()
                    .find(|method_chirho| method_chirho.name_chirho.text_chirho() == name_chirho)
                    .ok_or_else(|| {
                        self.invalid_class_chirho(
                            "pragma names no method of this class",
                            account_chirho.span_chirho,
                        )
                    })?;
                if account_chirho.kind_chirho == UnattachedMemberKindChirho::InlinePragmaChirho {
                    if method_chirho.default_chirho.is_none() {
                        return Err(self.invalid_class_chirho(
                            "INLINE method has no default binding",
                            account_chirho.span_chirho,
                        ));
                    }
                    if !inline_seen_chirho.insert(name_chirho) {
                        return Err(self.invalid_class_chirho(
                            "repeated INLINE pragma",
                            account_chirho.span_chirho,
                        ));
                    }
                }
            }
        }
        self.check_member_accounts_chirho(&remaining_chirho)
    }
}
