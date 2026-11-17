// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Reuse the lossless parser, rejecting recovery nodes before the AST lowerer can omit them.
//! workflow: next-pipeline-chirho/check-and-elaborate-chirho

use super::{NextErrorChirho, NextErrorKindChirho};
use haskelujah_parser_chirho::{
    cst_parser_chirho::ParserChirho,
    lexer_chirho::{LexerChirho, RawTokenKindChirho},
};
use haskelujah_span_chirho::FileIdChirho;
#[path = "io_chirho/literals_chirho.rs"]
mod literal_profile_chirho;
use haskelujah_syntax_chirho::{cst_chirho::SyntaxKindChirho, green_chirho::GreenElementChirho};

#[cfg(test)]
pub(super) fn parse_chirho(
    source_chirho: &str,
    file_chirho: FileIdChirho,
) -> Result<haskelujah_parser_chirho::lower_chirho::LoweredModuleChirho, NextErrorChirho> {
    parse_with_dependencies_chirho(source_chirho, file_chirho)
        .map(|(parsed_chirho, _, _, _)| parsed_chirho)
}

pub(super) fn parse_with_dependencies_chirho(
    source_chirho: &str,
    file_chirho: FileIdChirho,
) -> Result<
    (
        haskelujah_parser_chirho::lower_chirho::LoweredModuleChirho,
        bool,
        bool,
        std::collections::BTreeSet<String>,
    ),
    NextErrorChirho,
> {
    parse_with_permitted_pragmas_chirho(source_chirho, file_chirho, &Default::default(), false)
}

/// Called only by the driver-minted manifest source path. Home input supplies no ranges.
pub(super) fn parse_with_permitted_pragmas_chirho(
    source_chirho: &str,
    file_chirho: FileIdChirho,
    permitted_pragmas_chirho: &std::collections::BTreeSet<(usize, usize)>,
    compiler_source_chirho: bool,
) -> Result<
    (
        haskelujah_parser_chirho::lower_chirho::LoweredModuleChirho,
        bool,
        bool,
        std::collections::BTreeSet<String>,
    ),
    NextErrorChirho,
> {
    if source_chirho.len() > 1 << 20 {
        return Err(NextErrorChirho::new_chirho(
            NextErrorKindChirho::ResourceChirho,
            "next source byte ceiling exhausted",
        ));
    }
    // Error raw tokens are represented as identifiers in the legacy CST API.
    // Check the actual lexer output before that lossy mapping.
    let tokens_chirho = LexerChirho::new_chirho(source_chirho, file_chirho).lex_all_chirho();
    if tokens_chirho
        .iter()
        .any(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::ErrorChirho)
    {
        return Err(NextErrorChirho::new_chirho(
            NextErrorKindChirho::InvalidChirho,
            "lexical error",
        ));
    }
    if tokens_chirho
        .iter()
        .any(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::RecChirho)
    {
        return Err(NextErrorChirho::new_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "recursive statements require the RecursiveDo semantics",
        ));
    }
    for token_chirho in &tokens_chirho {
        if matches!(
            token_chirho.kind_chirho,
            RawTokenKindChirho::StringLitChirho | RawTokenKindChirho::CharLitChirho
        ) {
            literal_profile_chirho::validate_chirho(
                token_chirho
                    .span_chirho
                    .text_chirho(source_chirho)
                    .unwrap_or(""),
                token_chirho.kind_chirho == RawTokenKindChirho::StringLitChirho,
            )?;
        }
    }
    // The reused parser can retain a missing closing delimiter without an ErrorNode.
    // Balance actual lexer tokens, so comments and literal contents do not affect it.
    let mut delimiters_chirho = Vec::new();
    let mut owned_pragmas_chirho = Vec::new();
    for token_chirho in &tokens_chirho {
        if token_chirho.kind_chirho == RawTokenKindChirho::PragmaChirho {
            let inner_chirho = token_chirho
                .span_chirho
                .text_chirho(source_chirho)
                .and_then(|text_chirho| text_chirho.strip_prefix("{-#"))
                .and_then(|text_chirho| text_chirho.strip_suffix("#-}"))
                .ok_or_else(|| {
                    NextErrorChirho::new_chirho(
                        NextErrorKindChirho::InvalidChirho,
                        "unterminated pragma",
                    )
                })?
                .trim();
            let Some((keyword_chirho, flags_chirho)) = inner_chirho.split_once(char::is_whitespace)
            else {
                return Err(NextErrorChirho::new_chirho(
                    NextErrorKindChirho::UnsupportedChirho,
                    "next does not implement this pragma",
                ));
            };
            if matches!(keyword_chirho, "MINIMAL" | "INLINE") {
                owned_pragmas_chirho.push(token_chirho.span_chirho);
                continue;
            }
            if keyword_chirho == "NOINLINE"
                && permitted_pragmas_chirho.contains(&(
                    token_chirho.span_chirho.start_chirho().as_usize_chirho(),
                    token_chirho.span_chirho.end_chirho().as_usize_chirho(),
                ))
            {
                continue;
            }
            if keyword_chirho != "LANGUAGE" {
                return Err(NextErrorChirho::new_chirho(
                    NextErrorKindChirho::UnsupportedChirho,
                    "next does not yet check this pragma; ignored pragma bodies cannot establish acceptance",
                ));
            }
            if flags_chirho
                .split(',')
                .any(|flag_chirho| flag_chirho.trim().is_empty())
            {
                return Err(NextErrorChirho::new_chirho(
                    NextErrorKindChirho::InvalidChirho,
                    "empty LANGUAGE flag",
                ));
            }
        }
        use RawTokenKindChirho::{
            LeftBraceChirho, LeftBracketChirho, LeftParenChirho, RightBraceChirho,
            RightBracketChirho, RightParenChirho,
        };
        match token_chirho.kind_chirho {
            LeftParenChirho => delimiters_chirho.push(RightParenChirho),
            LeftBracketChirho => delimiters_chirho.push(RightBracketChirho),
            LeftBraceChirho => delimiters_chirho.push(RightBraceChirho),
            RightParenChirho | RightBracketChirho | RightBraceChirho => {
                if delimiters_chirho.pop() != Some(token_chirho.kind_chirho) {
                    return Err(NextErrorChirho::new_chirho(
                        NextErrorKindChirho::InvalidChirho,
                        "unmatched source delimiter",
                    ));
                }
            }
            _ => {}
        }
        if delimiters_chirho.len() > 128 {
            return Err(NextErrorChirho::new_chirho(
                NextErrorKindChirho::ResourceChirho,
                "next source delimiter depth ceiling exhausted",
            ));
        }
    }
    if !delimiters_chirho.is_empty() {
        return Err(NextErrorChirho::new_chirho(
            NextErrorKindChirho::InvalidChirho,
            "unclosed source delimiter",
        ));
    }
    let green_chirho = ParserChirho::new_chirho(source_chirho, file_chirho).parse_chirho();
    let mut work_chirho = vec![(green_chirho.as_ref(), 0_usize)];
    let mut nodes_chirho = 0_usize;
    let mut syntax_fixities_chirho = 0_usize;
    let mut negation_chirho = false;
    let mut prelude_mentions_chirho = std::collections::BTreeSet::new();
    while let Some((node_chirho, depth_chirho)) = work_chirho.pop() {
        nodes_chirho += 1;
        syntax_fixities_chirho +=
            usize::from(node_chirho.kind_chirho() == SyntaxKindChirho::FixityDeclChirho);
        negation_chirho |= node_chirho.kind_chirho() == SyntaxKindChirho::NegateExprChirho;
        // A loading hint only: identities still require pinned facade routes and checked exports.
        if matches!(
            node_chirho.kind_chirho(),
            SyntaxKindChirho::NameExprChirho
                | SyntaxKindChirho::InfixExprChirho
                | SyntaxKindChirho::ConTypeChirho
                | SyntaxKindChirho::ContextChirho
                | SyntaxKindChirho::ImportSpecChirho
                | SyntaxKindChirho::ExportSpecChirho
                | SyntaxKindChirho::InstanceDeclChirho
        ) {
            for child_chirho in node_chirho.children_chirho() {
                if let GreenElementChirho::TokenChirho(token_chirho) = child_chirho {
                    prelude_mentions_chirho.insert(token_chirho.text_chirho().to_string());
                }
            }
        }
        if nodes_chirho > 1 << 20 || depth_chirho > 256 {
            return Err(NextErrorChirho::new_chirho(
                NextErrorKindChirho::ResourceChirho,
                "next syntax node/depth ceiling exhausted",
            ));
        }
        if node_chirho.kind_chirho() == SyntaxKindChirho::ErrorNodeChirho {
            return Err(NextErrorChirho::new_chirho(
                if owned_pragmas_chirho.is_empty() {
                    NextErrorKindChirho::InvalidChirho
                } else {
                    NextErrorKindChirho::UnsupportedChirho
                },
                "parse error or unsupported pragma position; recovered syntax is never accepted by next",
            ));
        }
        if node_chirho.kind_chirho() == SyntaxKindChirho::ImportDeclChirho
            && node_chirho.children_chirho().iter().any(|child_chirho| matches!(
                child_chirho, GreenElementChirho::TokenChirho(token_chirho)
                if token_chirho.kind_chirho() == haskelujah_syntax_chirho::token_chirho::TokenKindChirho::StringLiteralChirho
            ))
        {
            return Err(NextErrorChirho::new_chirho(NextErrorKindChirho::UnsupportedChirho, "next needs a defining unit for a package-qualified import"));
        }
        for child_chirho in node_chirho.children_chirho() {
            if let GreenElementChirho::NodeChirho(child_chirho) = child_chirho {
                work_chirho.push((child_chirho.as_ref(), depth_chirho + 1));
            }
        }
    }
    if green_chirho.text_len_chirho() != source_chirho.len() {
        return Err(NextErrorChirho::new_chirho(
            NextErrorKindChirho::InvalidChirho,
            "parser did not retain the complete source",
        ));
    }
    let mut parsed_chirho =
        haskelujah_parser_chirho::lower_chirho::lower_module_with_integer_literals_chirho(
            &green_chirho,
            file_chirho,
        );
    if !tokens_chirho
        .iter()
        .any(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::ModuleChirho)
    {
        // Haskell's implicit header exports only main, rather than every helper.
        parsed_chirho.module_chirho.exports_chirho = Some(vec![
            haskelujah_ast_chirho::module_chirho::ExportSpecChirho::VarChirho(
                haskelujah_ast_chirho::name_chirho::NameChirho::RawChirho(
                    haskelujah_ast_chirho::name_chirho::RawNameChirho::unqualified_chirho(
                        "main",
                        haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
                    ),
                ),
            ),
        ]);
    }
    let represented_fixities_chirho = parsed_chirho
        .module_chirho
        .decls_chirho
        .iter()
        .filter(|declaration_chirho| {
            matches!(
                declaration_chirho,
                haskelujah_ast_chirho::decl_chirho::DeclChirho::FixityDeclChirho { .. }
            )
        })
        .count();
    if represented_fixities_chirho != syntax_fixities_chirho {
        return Err(NextErrorChirho::new_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "scoped or unretained fixity declarations need their own checked binding group",
        ));
    }
    for span_chirho in owned_pragmas_chirho {
        let owned_chirho =
            parsed_chirho
                .module_chirho
                .decls_chirho
                .iter()
                .any(|declaration_chirho| {
                    if let haskelujah_ast_chirho::decl_chirho::DeclChirho::ClassDeclChirho {
                        unattached_members_chirho,
                        ..
                    } = declaration_chirho
                    {
                        unattached_members_chirho
                            .iter()
                            .any(|account_chirho| account_chirho.span_chirho == span_chirho)
                    } else if compiler_source_chirho && let haskelujah_ast_chirho::decl_chirho::DeclChirho::InstanceDeclChirho { unattached_members_chirho,.. } = declaration_chirho {
                        unattached_members_chirho.iter().any(|account_chirho| account_chirho.span_chirho == span_chirho && account_chirho.kind_chirho == haskelujah_ast_chirho::decl_chirho::UnattachedMemberKindChirho::InlinePragmaChirho)
                    } else { false }
                });
        if !owned_chirho {
            return Err(NextErrorChirho::new_chirho(
                NextErrorKindChirho::UnsupportedChirho,
                "MINIMAL/INLINE pragma has no supported class owner",
            ));
        }
    }
    let numeric_chirho = (negation_chirho || !parsed_chirho.integer_literals_chirho.is_empty())
        && !parsed_chirho
            .module_chirho
            .extensions_chirho
            .iter()
            .any(|flag_chirho| flag_chirho == "RebindableSyntax");
    let do_syntax_chirho = tokens_chirho
        .iter()
        .any(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::DoChirho);
    Ok((
        parsed_chirho,
        numeric_chirho,
        do_syntax_chirho,
        prelude_mentions_chirho,
    ))
}
