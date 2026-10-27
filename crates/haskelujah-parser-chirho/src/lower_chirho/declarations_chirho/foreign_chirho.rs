// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Foreign imports and exports.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), split by category in row 641.

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, ForeignDirectionChirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho};

impl LowerCtxChirho {
    pub(super) fn lower_foreign_decl_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> DeclChirho {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut direction_chirho = ForeignDirectionChirho::ImportChirho;
        let mut calling_conv_chirho = String::from("ccall");
        let mut safety_chirho: Option<String> = None;
        let mut foreign_name_chirho: Option<String> = None;
        let mut hs_name_chirho: Option<NameChirho> = None;
        let mut double_colon_idx_chirho: Option<usize> = None;

        // Phase tracking: 0=direction, 1=callconv, 2=safety/name, 3=hsName, 4=done
        let mut phase_chirho: u8 = 0;

        for (idx_chirho, child_chirho) in children_chirho.iter().enumerate() {
            if let GreenElementChirho::TokenChirho(tok_chirho) = child_chirho.element_chirho {
                let kind_chirho = tok_chirho.kind_chirho();
                let text_chirho = tok_chirho.text_chirho();
                let s_chirho = self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);

                if kind_chirho == TokenKindChirho::DoubleColonChirho {
                    double_colon_idx_chirho = Some(idx_chirho);
                    break;
                }

                match kind_chirho {
                    TokenKindChirho::ForeignKeywordChirho => {}
                    TokenKindChirho::ImportKeywordChirho => {
                        direction_chirho = ForeignDirectionChirho::ImportChirho;
                        phase_chirho = 1;
                    }
                    TokenKindChirho::VarIdChirho
                        if phase_chirho == 0 && text_chirho == "export" =>
                    {
                        direction_chirho = ForeignDirectionChirho::ExportChirho;
                        phase_chirho = 1;
                    }
                    TokenKindChirho::VarIdChirho if phase_chirho == 1 => match text_chirho {
                        "ccall" | "capi" | "stdcall" | "prim" | "javascript" | "cplusplus" => {
                            calling_conv_chirho = text_chirho.to_string();
                            phase_chirho = 2;
                        }
                        _ => {
                            hs_name_chirho =
                                Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                            phase_chirho = 4;
                        }
                    },
                    TokenKindChirho::VarIdChirho if phase_chirho == 2 => match text_chirho {
                        "safe" | "unsafe" | "interruptible" => {
                            safety_chirho = Some(text_chirho.to_string());
                            phase_chirho = 3;
                        }
                        _ => {
                            hs_name_chirho =
                                Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                            phase_chirho = 4;
                        }
                    },
                    TokenKindChirho::StringLiteralChirho
                        if phase_chirho == 2 || phase_chirho == 3 =>
                    {
                        let raw_chirho = text_chirho.trim_end_matches('#');
                        let trimmed_chirho = raw_chirho
                            .strip_prefix('"')
                            .unwrap_or(raw_chirho)
                            .strip_suffix('"')
                            .unwrap_or(raw_chirho);
                        foreign_name_chirho = Some(trimmed_chirho.to_string());
                        phase_chirho = 3;
                    }
                    TokenKindChirho::VarIdChirho | TokenKindChirho::ConIdChirho
                        if phase_chirho == 3 || (phase_chirho >= 4 && hs_name_chirho.is_none()) =>
                    {
                        hs_name_chirho = Some(self.name_from_token_chirho(tok_chirho, s_chirho));
                        phase_chirho = 4;
                    }
                    _ => {}
                }
            }
        }

        // Build the type from children after ::
        let ty_chirho = if let Some(dc_idx_chirho) = double_colon_idx_chirho {
            self.lower_foreign_type_chirho(&children_chirho[(dc_idx_chirho + 1)..], span_chirho)
        } else {
            self.placeholder_type_chirho()
        };

        DeclChirho::ForeignDeclChirho {
            direction_chirho,
            name_chirho: hs_name_chirho.unwrap_or_else(|| self.dummy_name_chirho()),
            ty_chirho,
            calling_conv_chirho,
            safety_chirho,
            foreign_name_chirho,
            span_chirho,
        }
    }

    /// Build a type from the children after `::` in a foreign declaration.
    fn lower_foreign_type_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
        span_chirho: SpanChirho,
    ) -> TypeChirho {
        let mut types_chirho: Vec<TypeChirho> = Vec::new();
        let mut arrow_positions_chirho: Vec<usize> = Vec::new();
        let fold_type_app_segment_chirho = |segment_chirho: &[TypeChirho]| -> Option<TypeChirho> {
            let mut iter_chirho = segment_chirho.iter();
            let first_chirho = iter_chirho.next()?.clone();
            Some(iter_chirho.fold(first_chirho, |acc_chirho, arg_chirho| {
                TypeChirho::AppChirho {
                    fun_chirho: Box::new(acc_chirho),
                    arg_chirho: Box::new(arg_chirho.clone()),
                    span_chirho,
                }
            }))
        };

        let mut child_idx_chirho = 0usize;
        while child_idx_chirho < children_chirho.len() {
            let child_chirho = &children_chirho[child_idx_chirho];
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho) => {
                    let s_chirho =
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                    match tok_chirho.kind_chirho() {
                        TokenKindChirho::LeftParenChirho
                            if children_chirho
                                .get(child_idx_chirho + 1)
                                .and_then(|next_child_chirho| {
                                    match next_child_chirho.element_chirho {
                                        GreenElementChirho::TokenChirho(next_tok_chirho)
                                            if next_tok_chirho.kind_chirho()
                                                == TokenKindChirho::RightParenChirho =>
                                        {
                                            Some(next_child_chirho)
                                        }
                                        _ => None,
                                    }
                                })
                                .is_some() =>
                        {
                            let closing_child_chirho = &children_chirho[child_idx_chirho + 1];
                            types_chirho.push(TypeChirho::TupleChirho {
                                elements_chirho: Vec::new(),
                                span_chirho: self.span_chirho(
                                    child_chirho.start_chirho,
                                    closing_child_chirho.end_chirho,
                                ),
                            });
                            child_idx_chirho += 2;
                            continue;
                        }
                        TokenKindChirho::ConIdChirho | TokenKindChirho::QualifiedConIdChirho => {
                            let name_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                            types_chirho.push(TypeChirho::ConChirho(name_chirho));
                        }
                        TokenKindChirho::VarIdChirho => {
                            let name_chirho = self.name_from_token_chirho(tok_chirho, s_chirho);
                            types_chirho.push(TypeChirho::VarChirho(name_chirho));
                        }
                        TokenKindChirho::RightArrowChirho => {
                            arrow_positions_chirho.push(types_chirho.len());
                        }
                        _ => {}
                    }
                }
                GreenElementChirho::NodeChirho(sub_node_chirho) => {
                    types_chirho
                        .push(self.lower_type_chirho(sub_node_chirho, child_chirho.start_chirho));
                }
            }
            child_idx_chirho += 1;
        }

        if types_chirho.is_empty() {
            return self.placeholder_type_chirho();
        }

        if !arrow_positions_chirho.is_empty() {
            let mut segments_chirho: Vec<TypeChirho> = Vec::new();
            let mut segment_start_chirho = 0usize;
            for &segment_end_chirho in &arrow_positions_chirho {
                if let Some(segment_ty_chirho) = fold_type_app_segment_chirho(
                    &types_chirho[segment_start_chirho..segment_end_chirho],
                ) {
                    segments_chirho.push(segment_ty_chirho);
                }
                segment_start_chirho = segment_end_chirho;
            }
            if let Some(result_segment_chirho) =
                fold_type_app_segment_chirho(&types_chirho[segment_start_chirho..])
            {
                segments_chirho.push(result_segment_chirho);
            }
            if segments_chirho.is_empty() {
                return self.placeholder_type_chirho();
            }
            let mut result_chirho = segments_chirho.pop().unwrap();
            while let Some(arg_chirho) = segments_chirho.pop() {
                result_chirho = TypeChirho::FunChirho {
                    arg_chirho: Box::new(arg_chirho),
                    mult_chirho: None,
                    result_chirho: Box::new(result_chirho),
                    span_chirho,
                };
            }
            result_chirho
        } else if types_chirho.len() == 1 {
            types_chirho.pop().unwrap()
        } else {
            fold_type_app_segment_chirho(&types_chirho)
                .unwrap_or_else(|| self.placeholder_type_chirho())
        }
    }
}
