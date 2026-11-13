// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Own class pragmas by layout position; token ancestry alone is not a declaration owner.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;

fn semantic_edge_chirho(
    lower_chirho: &LowerCtxChirho,
    node_chirho: &GreenNodeChirho,
    base_chirho: usize,
    last_chirho: bool,
) -> Option<usize> {
    let children_chirho = lower_chirho.semantic_children_chirho(node_chirho, base_chirho);
    let mut edges_chirho =
        children_chirho
            .iter()
            .filter_map(|child_chirho| match child_chirho.element_chirho {
                GreenElementChirho::NodeChirho(node_chirho) => semantic_edge_chirho(
                    lower_chirho,
                    node_chirho,
                    child_chirho.start_chirho,
                    last_chirho,
                ),
                GreenElementChirho::TokenChirho(_) => Some(if last_chirho {
                    child_chirho.end_chirho
                } else {
                    child_chirho.start_chirho
                }),
            });
    if last_chirho {
        edges_chirho.max()
    } else {
        edges_chirho.next()
    }
}

fn pragma_columns_chirho(
    node_chirho: &GreenNodeChirho,
    base_chirho: usize,
    first_chirho: Option<usize>,
) -> (HashMap<usize, usize>, Option<usize>) {
    let mut work_chirho = node_chirho
        .children_chirho()
        .iter()
        .rev()
        .collect::<Vec<_>>();
    let mut offset_chirho = base_chirho;
    let mut column_chirho = None;
    let mut body_column_chirho = None;
    let mut columns_chirho = HashMap::new();
    while let Some(element_chirho) = work_chirho.pop() {
        match element_chirho {
            GreenElementChirho::NodeChirho(node_chirho) => {
                work_chirho.extend(node_chirho.children_chirho().iter().rev())
            }
            GreenElementChirho::TokenChirho(token_chirho) => {
                if Some(offset_chirho) == first_chirho {
                    body_column_chirho = column_chirho;
                }
                if token_chirho.kind_chirho() == TokenKindChirho::PragmaChirho
                    && let Some(column_chirho) = column_chirho
                {
                    columns_chirho.insert(offset_chirho, column_chirho);
                }
                for character_chirho in token_chirho.text_chirho().chars() {
                    if character_chirho == '\n' {
                        column_chirho = Some(0);
                    } else if let Some(current_chirho) = column_chirho.as_mut() {
                        *current_chirho = if character_chirho == '\t' {
                            (*current_chirho / 8 + 1) * 8
                        } else {
                            *current_chirho + 1
                        };
                    }
                }
                offset_chirho += token_chirho.text_chirho().len();
            }
        }
    }
    (columns_chirho, body_column_chirho)
}
fn delimited_pragmas_chirho(node_chirho: &GreenNodeChirho, base_chirho: usize) -> HashSet<usize> {
    let mut work_chirho = node_chirho
        .children_chirho()
        .iter()
        .rev()
        .collect::<Vec<_>>();
    let mut offset_chirho = base_chirho;
    let mut previous_chirho = None;
    let mut pending_chirho = None;
    let mut result_chirho = HashSet::new();
    while let Some(element_chirho) = work_chirho.pop() {
        match element_chirho {
            GreenElementChirho::NodeChirho(node_chirho) => {
                work_chirho.extend(node_chirho.children_chirho().iter().rev())
            }
            GreenElementChirho::TokenChirho(token_chirho) => {
                let kind_chirho = token_chirho.kind_chirho();
                if !matches!(
                    kind_chirho,
                    TokenKindChirho::WhitespaceTriviaChirho
                        | TokenKindChirho::LineCommentTriviaChirho
                        | TokenKindChirho::BlockCommentTriviaChirho
                        | TokenKindChirho::DocCommentTriviaChirho
                        | TokenKindChirho::VirtualLeftBraceChirho
                        | TokenKindChirho::VirtualRightBraceChirho
                        | TokenKindChirho::VirtualSemicolonChirho
                ) {
                    if let Some((start_chirho, before_chirho)) = pending_chirho.take()
                        && matches!(
                            before_chirho,
                            Some(
                                TokenKindChirho::LeftBraceChirho | TokenKindChirho::SemicolonChirho
                            )
                        )
                        && matches!(
                            kind_chirho,
                            TokenKindChirho::SemicolonChirho | TokenKindChirho::RightBraceChirho
                        )
                    {
                        result_chirho.insert(start_chirho);
                    }
                    if kind_chirho == TokenKindChirho::PragmaChirho {
                        pending_chirho = Some((offset_chirho, previous_chirho));
                    }
                    previous_chirho = Some(kind_chirho);
                }
                offset_chirho += token_chirho.text_chirho().len();
            }
        }
    }
    result_chirho
}

impl LowerCtxChirho {
    pub(super) fn collect_member_pragmas_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        class_chirho: bool,
        unattached_members_chirho: &mut Vec<UnattachedMemberChirho>,
    ) {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let first_chirho = children_chirho.iter().find_map(|child_chirho| {
            if let GreenElementChirho::NodeChirho(node_chirho) = child_chirho.element_chirho {
                semantic_edge_chirho(self, node_chirho, child_chirho.start_chirho, false)
            } else {
                None
            }
        });
        let explicit_chirho = node_chirho.children_chirho().iter().any(|child_chirho| matches!(child_chirho,
            GreenElementChirho::TokenChirho(token_chirho) if token_chirho.kind_chirho() == TokenKindChirho::LeftBraceChirho));
        let (columns_chirho, body_column_chirho) =
            pragma_columns_chirho(node_chirho, base_chirho, first_chirho);
        let delimited_chirho = delimited_pragmas_chirho(node_chirho, base_chirho);
        let mut work_chirho = vec![(node_chirho, base_chirho, 0, true)];
        while let Some((node_chirho, mut offset_chirho, tail_chirho, direct_chirho)) =
            work_chirho.pop()
        {
            for child_chirho in node_chirho.children_chirho() {
                let start_chirho = offset_chirho;
                offset_chirho += child_chirho.text_len_chirho();
                match child_chirho {
                    GreenElementChirho::NodeChirho(n_chirho) => {
                        let tail_chirho = if direct_chirho {
                            semantic_edge_chirho(self, n_chirho, start_chirho, true)
                                .unwrap_or(start_chirho)
                        } else {
                            tail_chirho
                        };
                        work_chirho.push((n_chirho.as_ref(), start_chirho, tail_chirho, false));
                    }
                    GreenElementChirho::TokenChirho(token_chirho)
                        if token_chirho.kind_chirho() == TokenKindChirho::PragmaChirho =>
                    {
                        let inner_chirho = token_chirho
                            .text_chirho()
                            .strip_prefix("{-#")
                            .and_then(|text_chirho| text_chirho.strip_suffix("#-}"))
                            .unwrap_or("")
                            .trim();
                        let (keyword_chirho, body_chirho) = inner_chirho
                            .split_once(char::is_whitespace)
                            .unwrap_or((inner_chirho, ""));
                        let layout_chirho = if explicit_chirho {
                            direct_chirho && delimited_chirho.contains(&start_chirho)
                        } else {
                            body_column_chirho.is_some_and(|body_chirho| {
                                columns_chirho.get(&start_chirho) == Some(&body_chirho)
                            })
                        };
                        let admissible_chirho =
                            class_chirho && layout_chirho && start_chirho >= tail_chirho;
                        let kind_chirho = match keyword_chirho {
                            "MINIMAL" if admissible_chirho => {
                                UnattachedMemberKindChirho::MinimalPragmaChirho
                            }
                            "INLINE" if admissible_chirho => {
                                UnattachedMemberKindChirho::InlinePragmaChirho
                            }
                            _ => UnattachedMemberKindChirho::UnsupportedChirho,
                        };
                        unattached_members_chirho.push(UnattachedMemberChirho {
                            kind_chirho,
                            name_chirho: Some(body_chirho.trim().into()),
                            span_chirho: self.span_chirho(start_chirho, offset_chirho),
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}
