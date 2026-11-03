// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lossless constructor signature decomposition. Ordinary type consumers and
//! GADT lowering share the prefix readers; fields are never ordinary records.
//! Workflow: compiler-pipeline-chirho/gadt-constructor-carrier-chirho.md.

use std::collections::HashSet;

use haskelujah_ast_chirho::decl_chirho::{
    FieldDeclChirho, GadtConArgsChirho, GadtConFieldChirho, GadtConPrefixChirho,
    GadtConSignatureChirho, StrictnessChirho, TyVarChirho,
};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, MultiplicityChirho};
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{ChildChirho, LowerCtxChirho, is_type_kind_chirho};

#[derive(Clone, Copy)]
pub(super) struct SignatureBodyChirho<'tree_chirho> {
    pub(super) node_chirho: &'tree_chirho GreenNodeChirho,
    pub(super) base_chirho: usize,
}

pub(super) struct ForallPartsChirho<'tree_chirho> {
    pub(super) vars_chirho: Vec<TyVarChirho>,
    pub(super) required_chirho: bool,
    pub(super) body_chirho: Option<SignatureBodyChirho<'tree_chirho>>,
}

pub(super) struct ContextPartsChirho<'tree_chirho> {
    pub(super) constraints_chirho: Vec<ConstraintChirho>,
    pub(super) body_chirho: Option<SignatureBodyChirho<'tree_chirho>>,
}

fn type_body_chirho<'tree_chirho>(
    child_chirho: &ChildChirho<'tree_chirho>,
) -> Option<SignatureBodyChirho<'tree_chirho>> {
    if let GreenElementChirho::NodeChirho(node_chirho) = child_chirho.element_chirho
        && (is_type_kind_chirho(node_chirho.kind_chirho())
            || node_chirho.kind_chirho() == SyntaxKindChirho::RecordFieldsChirho)
    {
        Some(SignatureBodyChirho {
            node_chirho,
            base_chirho: child_chirho.start_chirho,
        })
    } else {
        None
    }
}

impl LowerCtxChirho {
    pub(super) fn lower_forall_parts_chirho<'tree_chirho>(
        &self,
        node_chirho: &'tree_chirho GreenNodeChirho,
        base_chirho: usize,
    ) -> ForallPartsChirho<'tree_chirho> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut vars_chirho = Vec::new();
        let mut saw_delimiter_chirho = false;
        let mut required_chirho = false;
        let mut body_chirho = None;
        for child_chirho in &children_chirho {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(token_chirho) => {
                    let kind_chirho = token_chirho.kind_chirho();
                    if kind_chirho == TokenKindChirho::VarIdChirho && !saw_delimiter_chirho {
                        vars_chirho.push(
                            self.name_from_token_chirho(
                                token_chirho,
                                self.span_chirho(
                                    child_chirho.start_chirho,
                                    child_chirho.end_chirho,
                                ),
                            )
                            .into(),
                        );
                    } else if kind_chirho == TokenKindChirho::VarSymChirho
                        && token_chirho.text_chirho() == "."
                    {
                        saw_delimiter_chirho = true;
                    } else if kind_chirho == TokenKindChirho::RightArrowChirho {
                        saw_delimiter_chirho = true;
                        required_chirho = true;
                    }
                }
                GreenElementChirho::NodeChirho(inner_chirho) if !saw_delimiter_chirho => {
                    if inner_chirho.kind_chirho() == SyntaxKindChirho::ParenTypeChirho
                        && let Some(var_chirho) = self
                            .try_lower_kind_annotated_forall_binder_chirho(
                                inner_chirho,
                                child_chirho.start_chirho,
                            )
                    {
                        vars_chirho.push(var_chirho);
                    }
                }
                GreenElementChirho::NodeChirho(_) if body_chirho.is_none() => {
                    body_chirho = type_body_chirho(child_chirho);
                }
                _ => {}
            }
        }
        ForallPartsChirho {
            vars_chirho,
            required_chirho,
            body_chirho,
        }
    }

    pub(super) fn lower_context_parts_chirho<'tree_chirho>(
        &self,
        node_chirho: &'tree_chirho GreenNodeChirho,
        base_chirho: usize,
    ) -> ContextPartsChirho<'tree_chirho> {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut before_chirho = Vec::new();
        let mut after_arrow_chirho = false;
        let mut body_chirho = None;
        for child_chirho in &children_chirho {
            if let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho
                && token_chirho.kind_chirho() == TokenKindChirho::DoubleArrowChirho
            {
                after_arrow_chirho = true;
            } else if !after_arrow_chirho {
                before_chirho.push(child_chirho);
            } else if body_chirho.is_none() {
                body_chirho = type_body_chirho(child_chirho);
            }
        }
        let context_ty_chirho = self.type_from_flat_children_chirho(&before_chirho, span_chirho);
        ContextPartsChirho {
            constraints_chirho: Self::type_to_constraints_chirho(&context_ty_chirho, span_chirho),
            body_chirho,
        }
    }

    pub(super) fn lower_arrow_multiplicity_chirho(
        &self,
        children_chirho: &[ChildChirho<'_>],
    ) -> Option<MultiplicityChirho> {
        let mut after_percent_chirho = false;
        for child_chirho in children_chirho {
            let GreenElementChirho::TokenChirho(token_chirho) = child_chirho.element_chirho else {
                continue;
            };
            if token_chirho.kind_chirho() == TokenKindChirho::LinearArrowChirho {
                return Some(MultiplicityChirho::OneChirho);
            }
            if token_chirho.text_chirho() == "%" {
                after_percent_chirho = true;
            } else if after_percent_chirho {
                return match token_chirho.text_chirho() {
                    "1" | "One" => Some(MultiplicityChirho::OneChirho),
                    "Many" => Some(MultiplicityChirho::ManyChirho),
                    _ if token_chirho.kind_chirho() == TokenKindChirho::VarIdChirho => Some(
                        MultiplicityChirho::MultVarChirho(self.name_from_token_chirho(
                            token_chirho,
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                        )),
                    ),
                    _ => None,
                };
            }
        }
        None
    }

    pub(super) fn lower_record_fields_mode_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        gadt_fields_chirho: bool,
    ) -> Vec<FieldDeclChirho> {
        self.semantic_children_chirho(node_chirho, base_chirho)
            .into_iter()
            .filter_map(|child_chirho| {
                if let GreenElementChirho::NodeChirho(field_chirho) = child_chirho.element_chirho
                    && field_chirho.kind_chirho() == SyntaxKindChirho::FieldDeclChirho
                {
                    Some(if gadt_fields_chirho {
                        self.lower_field_decl_mode_chirho(
                            field_chirho,
                            child_chirho.start_chirho,
                            true,
                        )
                    } else {
                        self.lower_field_decl_chirho(field_chirho, child_chirho.start_chirho)
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub(super) fn lower_gadt_signature_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> GadtConSignatureChirho {
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        let mut signature_chirho = GadtConSignatureChirho {
            prefix_chirho: Vec::new(),
            args_chirho: GadtConArgsChirho::PositionalChirho(Vec::new()),
            result_ty_chirho: self.placeholder_type_chirho(),
            malformed_chirho: false,
            span_chirho,
        };
        let mut pending_chirho = vec![node_chirho];
        let mut remaining_chirho = 65_536_usize;
        while let Some(current_chirho) = pending_chirho.pop() {
            if remaining_chirho == 0 {
                signature_chirho.malformed_chirho = true;
                break;
            }
            remaining_chirho -= 1;
            if current_chirho.kind_chirho() == SyntaxKindChirho::ErrorNodeChirho {
                signature_chirho.malformed_chirho = true;
            }
            for child_chirho in current_chirho.children_chirho() {
                if let GreenElementChirho::NodeChirho(inner_chirho) = child_chirho {
                    pending_chirho.push(inner_chirho);
                }
            }
        }
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let Some(body_chirho) = children_chirho.iter().find_map(type_body_chirho) else {
            signature_chirho.malformed_chirho = true;
            return signature_chirho;
        };
        self.lower_gadt_spine_chirho(body_chirho, &mut signature_chirho, 0);
        let Some(unpacked_chirho) = unpacked_argument_offsets_chirho(node_chirho, base_chirho)
        else {
            signature_chirho.malformed_chirho = true;
            return signature_chirho;
        };
        if let GadtConArgsChirho::PositionalChirho(fields_chirho) =
            &mut signature_chirho.args_chirho
        {
            for field_chirho in fields_chirho {
                if field_chirho.strictness_chirho == StrictnessChirho::StrictChirho
                    && unpacked_chirho.contains(&field_chirho.span_chirho.start_chirho().as_usize_chirho())
                {
                    field_chirho.strictness_chirho = StrictnessChirho::UnpackChirho;
                }
            }
        }
        signature_chirho
    }

    fn lower_gadt_spine_chirho(
        &self,
        body_chirho: SignatureBodyChirho<'_>,
        signature_chirho: &mut GadtConSignatureChirho,
        depth_chirho: usize,
    ) {
        if depth_chirho > 128 {
            signature_chirho.malformed_chirho = true;
            return;
        }
        let node_chirho = body_chirho.node_chirho;
        let base_chirho = body_chirho.base_chirho;
        let span_chirho =
            self.span_chirho(base_chirho, base_chirho + node_chirho.text_len_chirho());
        // A forall or context after an argument belongs to the result. Moving
        // it to the constructor prefix would change its scope and arity.
        if matches!(
            node_chirho.kind_chirho(),
            SyntaxKindChirho::ForallTypeChirho | SyntaxKindChirho::QualTypeChirho
        ) && !matches!(&signature_chirho.args_chirho, GadtConArgsChirho::PositionalChirho(fields_chirho) if fields_chirho.is_empty())
        {
            signature_chirho.result_ty_chirho = self.lower_type_chirho(node_chirho, base_chirho);
            return;
        }
        match node_chirho.kind_chirho() {
            SyntaxKindChirho::ForallTypeChirho => {
                let parts_chirho = self.lower_forall_parts_chirho(node_chirho, base_chirho);
                signature_chirho
                    .prefix_chirho
                    .push(GadtConPrefixChirho::ForallChirho {
                        vars_chirho: parts_chirho.vars_chirho,
                        required_chirho: parts_chirho.required_chirho,
                        span_chirho,
                    });
                if let Some(next_chirho) = parts_chirho.body_chirho {
                    self.lower_gadt_spine_chirho(next_chirho, signature_chirho, depth_chirho + 1);
                } else {
                    signature_chirho.malformed_chirho = true;
                }
            }
            SyntaxKindChirho::QualTypeChirho => {
                let parts_chirho = self.lower_context_parts_chirho(node_chirho, base_chirho);
                signature_chirho
                    .prefix_chirho
                    .push(GadtConPrefixChirho::ContextChirho {
                        constraints_chirho: parts_chirho.constraints_chirho,
                        span_chirho,
                    });
                if let Some(next_chirho) = parts_chirho.body_chirho {
                    self.lower_gadt_spine_chirho(next_chirho, signature_chirho, depth_chirho + 1);
                } else {
                    signature_chirho.malformed_chirho = true;
                }
            }
            SyntaxKindChirho::ParenTypeChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                if let Some(next_chirho) = children_chirho.iter().find_map(type_body_chirho) {
                    self.lower_gadt_spine_chirho(next_chirho, signature_chirho, depth_chirho + 1);
                } else {
                    signature_chirho.malformed_chirho = true;
                }
            }
            SyntaxKindChirho::FunTypeChirho => {
                let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
                let nodes_chirho = children_chirho
                    .iter()
                    .filter_map(type_body_chirho)
                    .collect::<Vec<_>>();
                if nodes_chirho.len() != 2 {
                    signature_chirho.malformed_chirho = true;
                    return;
                }
                let field_chirho = nodes_chirho[0];
                let result_chirho = nodes_chirho[1];
                let multiplicity_chirho = self.lower_arrow_multiplicity_chirho(&children_chirho);
                let wrote_percent_chirho = children_chirho.iter().any(|child_chirho| matches!(
                    child_chirho.element_chirho, GreenElementChirho::TokenChirho(token_chirho) if token_chirho.text_chirho() == "%"
                ));
                if wrote_percent_chirho && multiplicity_chirho.is_none() {
                    signature_chirho.malformed_chirho = true;
                }
                if field_chirho.node_chirho.kind_chirho() == SyntaxKindChirho::RecordFieldsChirho {
                    if !matches!(&signature_chirho.args_chirho, GadtConArgsChirho::PositionalChirho(fields_chirho) if fields_chirho.is_empty())
                        || multiplicity_chirho.is_some()
                    {
                        signature_chirho.malformed_chirho = true;
                    }
                    let fields_chirho = self.lower_record_fields_mode_chirho(
                        field_chirho.node_chirho,
                        field_chirho.base_chirho,
                        true,
                    );
                    for raw_field_chirho in self.semantic_children_chirho(
                        field_chirho.node_chirho,
                        field_chirho.base_chirho,
                    ) {
                        if let GreenElementChirho::NodeChirho(raw_node_chirho) =
                            raw_field_chirho.element_chirho
                            && raw_node_chirho.kind_chirho() == SyntaxKindChirho::FieldDeclChirho
                        {
                            let raw_children_chirho = self.semantic_children_chirho(
                                raw_node_chirho,
                                raw_field_chirho.start_chirho,
                            );
                            let mut multiplicity_chirho = Vec::new();
                            let mut wrote_percent_chirho = false;
                            for child_chirho in raw_children_chirho {
                                if matches!(child_chirho.element_chirho, GreenElementChirho::TokenChirho(token_chirho) if token_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho)
                                {
                                    break;
                                }
                                if matches!(child_chirho.element_chirho, GreenElementChirho::TokenChirho(token_chirho) if token_chirho.text_chirho() == "%")
                                {
                                    wrote_percent_chirho = true;
                                }
                                if wrote_percent_chirho {
                                    multiplicity_chirho.push(child_chirho);
                                }
                            }
                            if wrote_percent_chirho
                                && self
                                    .lower_arrow_multiplicity_chirho(&multiplicity_chirho)
                                    .is_none()
                            {
                                signature_chirho.malformed_chirho = true;
                            }
                        }
                    }
                    signature_chirho.args_chirho = GadtConArgsChirho::RecordChirho(fields_chirho);
                    if matches!(
                        result_chirho.node_chirho.kind_chirho(),
                        SyntaxKindChirho::GadtFieldTypeChirho
                            | SyntaxKindChirho::RecordFieldsChirho
                            | SyntaxKindChirho::FunTypeChirho
                    ) {
                        signature_chirho.malformed_chirho = true;
                    }
                    signature_chirho.result_ty_chirho = self
                        .lower_type_chirho(result_chirho.node_chirho, result_chirho.base_chirho);
                } else {
                    let mut field_body_chirho = field_chirho;
                    let strictness_chirho = if field_chirho.node_chirho.kind_chirho()
                        == SyntaxKindChirho::GadtFieldTypeChirho
                    {
                        let field_children_chirho = self.semantic_children_chirho(
                            field_chirho.node_chirho,
                            field_chirho.base_chirho,
                        );
                        let Some(inner_chirho) =
                            field_children_chirho.iter().find_map(type_body_chirho)
                        else {
                            signature_chirho.malformed_chirho = true;
                            return;
                        };
                        field_body_chirho = inner_chirho;
                        StrictnessChirho::StrictChirho
                    } else {
                        StrictnessChirho::LazyChirho
                    };
                    let ty_chirho = self.lower_type_chirho(
                        field_body_chirho.node_chirho,
                        field_body_chirho.base_chirho,
                    );
                    if let GadtConArgsChirho::PositionalChirho(fields_chirho) =
                        &mut signature_chirho.args_chirho
                    {
                        fields_chirho.push(GadtConFieldChirho {
                            strictness_chirho,
                            multiplicity_chirho,
                            ty_chirho,
                            span_chirho: self.span_chirho(
                                field_chirho.base_chirho,
                                field_chirho.base_chirho
                                    + field_chirho.node_chirho.text_len_chirho(),
                            ),
                        });
                    } else {
                        signature_chirho.malformed_chirho = true;
                    }
                    self.lower_gadt_spine_chirho(result_chirho, signature_chirho, depth_chirho + 1);
                }
            }
            SyntaxKindChirho::GadtFieldTypeChirho | SyntaxKindChirho::RecordFieldsChirho => {
                signature_chirho.malformed_chirho = true;
            }
            _ => {
                signature_chirho.result_ty_chirho = self.lower_type_chirho(node_chirho, base_chirho)
            }
        }
    }
}

/// Pragmas are trivia to ordinary type lowering, but UNPACK immediately before
/// a bang belongs to that constructor argument. The token identity and offset
/// link it to the canonical field without scanning source strings or types.
fn unpacked_argument_offsets_chirho(
    node_chirho: &GreenNodeChirho,
    base_chirho: usize,
) -> Option<HashSet<usize>> {
    let mut tokens_chirho = Vec::new();
    let mut pending_chirho = vec![(node_chirho, base_chirho)];
    let mut remaining_chirho = 65_536_usize;
    while let Some((node_chirho, base_chirho)) = pending_chirho.pop() {
        let mut offset_chirho = base_chirho;
        for child_chirho in node_chirho.children_chirho() {
            if remaining_chirho == 0 {
                return None;
            }
            remaining_chirho -= 1;
            match child_chirho {
                GreenElementChirho::TokenChirho(token_chirho) => {
                    tokens_chirho.push((offset_chirho, token_chirho))
                }
                GreenElementChirho::NodeChirho(inner_chirho) => {
                    pending_chirho.push((inner_chirho, offset_chirho))
                }
            }
            offset_chirho += child_chirho.text_len_chirho();
        }
    }
    tokens_chirho.sort_unstable_by_key(|(offset_chirho, _)| *offset_chirho);
    let mut unpack_chirho = false;
    let mut offsets_chirho = HashSet::new();
    for (offset_chirho, token_chirho) in tokens_chirho {
        match token_chirho.kind_chirho() {
            TokenKindChirho::PragmaChirho => {
                unpack_chirho = token_chirho
                    .text_chirho()
                    .strip_prefix("{-#")
                    .and_then(|body_chirho| body_chirho.strip_suffix("#-}"))
                    .is_some_and(|body_chirho| body_chirho.trim() == "UNPACK");
            }
            TokenKindChirho::WhitespaceTriviaChirho
            | TokenKindChirho::LineCommentTriviaChirho
            | TokenKindChirho::BlockCommentTriviaChirho
            | TokenKindChirho::DocCommentTriviaChirho => {}
            _ => {
                if unpack_chirho && token_chirho.text_chirho() == "!" {
                    offsets_chirho.insert(offset_chirho);
                }
                unpack_chirho = false;
            }
        }
    }
    Some(offsets_chirho)
}
