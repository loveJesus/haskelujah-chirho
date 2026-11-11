// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Reuse the lossless parser, rejecting recovery nodes before the AST lowerer can omit them.
//! workflow: next-pipeline-chirho/check-and-elaborate-chirho

use super::{NextErrorChirho, NextErrorKindChirho};
use haskelujah_parser_chirho::{
    cst_parser_chirho::ParserChirho,
    lexer_chirho::{LexerChirho, RawTokenKindChirho},
};
use haskelujah_span_chirho::FileIdChirho;
use haskelujah_syntax_chirho::{cst_chirho::SyntaxKindChirho, green_chirho::GreenElementChirho};

pub(super) fn parse_chirho(
    source_chirho: &str,
    file_chirho: FileIdChirho,
) -> Result<haskelujah_ast_chirho::ModuleChirho, NextErrorChirho> {
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
    // The reused parser can retain a missing closing delimiter without an ErrorNode.
    // Balance actual lexer tokens, so comments and literal contents do not affect it.
    let mut delimiters_chirho = Vec::new();
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
    let mut work_chirho = vec![(green_chirho.as_ref(), 0_usize, false)];
    let mut nodes_chirho = 0_usize;
    while let Some((node_chirho, depth_chirho, in_class_chirho)) = work_chirho.pop() {
        nodes_chirho += 1;
        if nodes_chirho > 1 << 20 || depth_chirho > 256 {
            return Err(NextErrorChirho::new_chirho(
                NextErrorKindChirho::ResourceChirho,
                "next syntax node/depth ceiling exhausted",
            ));
        }
        if node_chirho.kind_chirho() == SyntaxKindChirho::ErrorNodeChirho {
            return Err(NextErrorChirho::new_chirho(
                NextErrorKindChirho::InvalidChirho,
                "parse error; recovered syntax is never accepted by next",
            ));
        }
        // The shared AST lowerer pairs defaults with signatures and drops an unpaired
        // body. Unit A supports no defaults, so decline their lossless source form.
        if in_class_chirho && node_chirho.kind_chirho() == SyntaxKindChirho::FunBindChirho {
            return Err(NextErrorChirho::new_chirho(
                NextErrorKindChirho::UnsupportedChirho,
                "next class default bodies require a retained, checked declaration account",
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
                work_chirho.push((
                    child_chirho.as_ref(),
                    depth_chirho + 1,
                    in_class_chirho
                        || node_chirho.kind_chirho() == SyntaxKindChirho::ClassDeclChirho,
                ));
            }
        }
    }
    if green_chirho.text_len_chirho() != source_chirho.len() {
        return Err(NextErrorChirho::new_chirho(
            NextErrorKindChirho::InvalidChirho,
            "parser did not retain the complete source",
        ));
    }
    Ok(haskelujah_parser_chirho::lower_chirho::lower_module_chirho(
        &green_chirho,
        file_chirho,
    ))
}
