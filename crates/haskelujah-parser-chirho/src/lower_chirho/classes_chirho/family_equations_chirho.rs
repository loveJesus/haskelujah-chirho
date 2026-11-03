// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Closed type-family equations: the written head, the arguments in order, the
//! explicit `forall` binders and the binders the left side never uses.
//! Part of CST-to-AST lowering (`lower_chirho.rs`), category classes (row 686).
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use std::collections::BTreeSet;

use haskelujah_ast_chirho::decl_chirho::TypeFamilyEquationChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{
    GreenElementChirho, GreenNodeChirho, GreenTokenChirho,
};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::{LowerCtxChirho, is_type_kind_chirho};

/// An explicit binder as written: its name and the type variables its kind
/// annotation mentions.
struct WrittenBinderChirho {
    name_chirho: NameChirho,
    kind_variables_chirho: BTreeSet<String>,
}

impl LowerCtxChirho {
    /// Lower one `TypeFamilyEquationChirho` node, `[forall bs.] lhs = rhs`.
    pub(super) fn lower_family_equation_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
        span_chirho: SpanChirho,
    ) -> TypeFamilyEquationChirho {
        let mut lhs_chirho = None;
        let mut rhs_chirho = None;
        let mut saw_equals_chirho = false;
        let mut malformed_chirho = false;
        for child_chirho in &self.semantic_children_chirho(node_chirho, base_chirho) {
            match child_chirho.element_chirho {
                GreenElementChirho::NodeChirho(n_chirho)
                    if is_type_kind_chirho(n_chirho.kind_chirho()) =>
                {
                    let side_chirho = if saw_equals_chirho {
                        &mut rhs_chirho
                    } else {
                        &mut lhs_chirho
                    };
                    if side_chirho.is_none() {
                        *side_chirho = Some((n_chirho, child_chirho.start_chirho));
                    } else {
                        malformed_chirho = true;
                    }
                }
                GreenElementChirho::TokenChirho(tok_chirho)
                    if tok_chirho.kind_chirho() == TokenKindChirho::EqualsChirho
                        && !saw_equals_chirho =>
                {
                    saw_equals_chirho = true;
                }
                _ => malformed_chirho = true,
            }
        }
        let malformed_chirho = malformed_chirho || rhs_chirho.is_none();
        let rhs_chirho = match rhs_chirho {
            Some((rhs_node_chirho, rhs_start_chirho)) => {
                self.lower_type_chirho(rhs_node_chirho, rhs_start_chirho)
            }
            None => self.placeholder_type_chirho(),
        };
        let Some((lhs_node_chirho, lhs_start_chirho)) = lhs_chirho else {
            return TypeFamilyEquationChirho {
                head_chirho: None,
                explicit_binders_chirho: None,
                unwritten_binders_chirho: Vec::new(),
                lhs_types_chirho: Vec::new(),
                rhs_chirho,
                malformed_chirho: true,
                span_chirho,
            };
        };
        let unwritten_binders_chirho =
            self.unwritten_family_binders_chirho(lhs_node_chirho, lhs_start_chirho);
        let (explicit_binders_chirho, body_chirho) =
            match self.lower_type_chirho(lhs_node_chirho, lhs_start_chirho) {
                TypeChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                    ..
                } => (Some(vars_chirho), *body_chirho),
                other_chirho => (None, other_chirho),
            };
        // A context is not part of an equation's grammar: GHC stops at `=>`.
        let malformed_chirho =
            malformed_chirho || matches!(body_chirho, TypeChirho::QualChirho { .. });
        let (head_chirho, lhs_types_chirho) = split_equation_head_chirho(body_chirho);
        TypeFamilyEquationChirho {
            head_chirho,
            explicit_binders_chirho,
            unwritten_binders_chirho,
            lhs_types_chirho,
            rhs_chirho,
            malformed_chirho,
            span_chirho,
        }
    }

    /// The explicit binders the left side never uses, by the rule documented on
    /// `TypeFamilyEquationChirho::unwritten_binders_chirho`. The left side's own
    /// `forall` telescope declares them; a binder is used when its name is free in
    /// the body or in the kind of a used binder.
    fn unwritten_family_binders_chirho(
        &self,
        lhs_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Vec<NameChirho> {
        if lhs_chirho.kind_chirho() != SyntaxKindChirho::ForallTypeChirho {
            return Vec::new();
        }
        let mut binders_chirho = Vec::new();
        let mut body_variables_chirho = BTreeSet::new();
        let mut in_telescope_chirho = true;
        for child_chirho in &self.semantic_children_chirho(lhs_chirho, base_chirho) {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho)
                    if in_telescope_chirho
                        && tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho =>
                {
                    binders_chirho.push(WrittenBinderChirho {
                        name_chirho: self.name_from_token_chirho(
                            tok_chirho,
                            self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                        ),
                        kind_variables_chirho: BTreeSet::new(),
                    });
                }
                GreenElementChirho::TokenChirho(tok_chirho)
                    if ends_telescope_chirho(tok_chirho) =>
                {
                    in_telescope_chirho = false;
                }
                GreenElementChirho::NodeChirho(n_chirho)
                    if in_telescope_chirho
                        && n_chirho.kind_chirho() == SyntaxKindChirho::ParenTypeChirho =>
                {
                    if let Some(binder_chirho) =
                        self.written_kinded_binder_chirho(n_chirho, child_chirho.start_chirho)
                    {
                        binders_chirho.push(binder_chirho);
                    }
                }
                element_chirho => collect_free_type_variables_chirho(
                    element_chirho,
                    &mut Vec::new(),
                    &mut body_variables_chirho,
                ),
            }
        }
        let mut used_chirho: BTreeSet<String> = binders_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.name_chirho.text_chirho().to_string())
            .filter(|name_chirho| body_variables_chirho.contains(name_chirho))
            .collect();
        loop {
            let reached_chirho: Vec<String> = binders_chirho
                .iter()
                .filter(|binder_chirho| {
                    used_chirho.contains(binder_chirho.name_chirho.text_chirho())
                })
                .flat_map(|binder_chirho| binder_chirho.kind_variables_chirho.iter().cloned())
                .filter(|name_chirho| {
                    !used_chirho.contains(name_chirho)
                        && binders_chirho.iter().any(|binder_chirho| {
                            binder_chirho.name_chirho.text_chirho() == name_chirho
                        })
                })
                .collect();
            if reached_chirho.is_empty() {
                break;
            }
            used_chirho.extend(reached_chirho);
        }
        binders_chirho
            .into_iter()
            .filter(|binder_chirho| !used_chirho.contains(binder_chirho.name_chirho.text_chirho()))
            .map(|binder_chirho| binder_chirho.name_chirho)
            .collect()
    }

    /// A kind-annotated telescope binder `(a :: k)`: its name, and the variables
    /// free in its kind.
    fn written_kinded_binder_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<WrittenBinderChirho> {
        let mut name_chirho = None;
        let mut kind_variables_chirho = BTreeSet::new();
        let mut in_kind_chirho = false;
        for child_chirho in &self.semantic_children_chirho(node_chirho, base_chirho) {
            match child_chirho.element_chirho {
                GreenElementChirho::TokenChirho(tok_chirho)
                    if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho =>
                {
                    in_kind_chirho = true;
                }
                GreenElementChirho::TokenChirho(tok_chirho)
                    if !in_kind_chirho
                        && name_chirho.is_none()
                        && tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho =>
                {
                    name_chirho = Some(self.name_from_token_chirho(
                        tok_chirho,
                        self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho),
                    ));
                }
                GreenElementChirho::NodeChirho(n_chirho)
                    if !in_kind_chirho
                        && name_chirho.is_none()
                        && n_chirho.kind_chirho() == SyntaxKindChirho::VarTypeChirho =>
                {
                    name_chirho = Some(
                        self.extract_name_from_node_chirho(n_chirho, child_chirho.start_chirho),
                    );
                }
                element_chirho if in_kind_chirho => collect_free_type_variables_chirho(
                    element_chirho,
                    &mut Vec::new(),
                    &mut kind_variables_chirho,
                ),
                _ => {}
            }
        }
        name_chirho.map(|name_chirho| WrittenBinderChirho {
            name_chirho,
            kind_variables_chirho,
        })
    }
}

/// Split a left side into the type constructor heading it and its arguments in
/// order, looking through parentheses around the head application (`(F a) b`).
fn split_equation_head_chirho(lhs_chirho: TypeChirho) -> (Option<NameChirho>, Vec<TypeChirho>) {
    let mut arguments_chirho = Vec::new();
    let mut current_chirho = lhs_chirho;
    let head_chirho = loop {
        match current_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                arguments_chirho.push(*arg_chirho);
                current_chirho = *fun_chirho;
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => current_chirho = *inner_chirho,
            TypeChirho::ConChirho(name_chirho) => break Some(name_chirho),
            _ => break None,
        }
    };
    arguments_chirho.reverse();
    (head_chirho, arguments_chirho)
}

/// The variable a `VarTypeChirho` node spells.
fn first_var_id_chirho(node_chirho: &GreenNodeChirho) -> Option<String> {
    node_chirho
        .children_chirho()
        .iter()
        .find_map(|child_chirho| match child_chirho {
            GreenElementChirho::TokenChirho(tok_chirho)
                if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho =>
            {
                Some(tok_chirho.text_chirho().to_string())
            }
            _ => None,
        })
}

/// The `.` of `forall bs.` or the `->` of a required `forall bs ->`.
fn ends_telescope_chirho(tok_chirho: &GreenTokenChirho) -> bool {
    tok_chirho.kind_chirho() == TokenKindChirho::RightArrowChirho
        || (tok_chirho.kind_chirho() == TokenKindChirho::VarSymChirho
            && tok_chirho.text_chirho() == ".")
}

/// Collect the type variables a CST element mentions free, kind annotations
/// included. A nested `forall` binds its telescope for the kinds after each
/// binder and for its body, hiding an outer name it rebinds.
fn collect_free_type_variables_chirho(
    element_chirho: &GreenElementChirho,
    bound_chirho: &mut Vec<String>,
    free_chirho: &mut BTreeSet<String>,
) {
    let node_chirho = match element_chirho {
        GreenElementChirho::TokenChirho(tok_chirho) => {
            if tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho
                && !bound_chirho
                    .iter()
                    .any(|bound_name_chirho| bound_name_chirho == tok_chirho.text_chirho())
            {
                free_chirho.insert(tok_chirho.text_chirho().to_string());
            }
            return;
        }
        GreenElementChirho::NodeChirho(node_chirho) => node_chirho,
    };
    if node_chirho.kind_chirho() != SyntaxKindChirho::ForallTypeChirho {
        for child_chirho in node_chirho.children_chirho() {
            collect_free_type_variables_chirho(child_chirho, bound_chirho, free_chirho);
        }
        return;
    }
    let outer_len_chirho = bound_chirho.len();
    let mut in_telescope_chirho = true;
    for child_chirho in node_chirho.children_chirho() {
        match child_chirho {
            GreenElementChirho::TokenChirho(tok_chirho)
                if in_telescope_chirho
                    && tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho =>
            {
                bound_chirho.push(tok_chirho.text_chirho().to_string());
            }
            GreenElementChirho::TokenChirho(tok_chirho) if ends_telescope_chirho(tok_chirho) => {
                in_telescope_chirho = false;
            }
            GreenElementChirho::NodeChirho(binder_chirho)
                if in_telescope_chirho
                    && binder_chirho.kind_chirho() == SyntaxKindChirho::ParenTypeChirho =>
            {
                // `(a :: k)`: the kind is read in the telescope so far, then `a` binds.
                let mut declared_chirho = None;
                let mut in_kind_chirho = false;
                for part_chirho in binder_chirho.children_chirho() {
                    match part_chirho {
                        GreenElementChirho::TokenChirho(tok_chirho)
                            if tok_chirho.kind_chirho() == TokenKindChirho::DoubleColonChirho =>
                        {
                            in_kind_chirho = true;
                        }
                        GreenElementChirho::TokenChirho(tok_chirho)
                            if !in_kind_chirho
                                && declared_chirho.is_none()
                                && tok_chirho.kind_chirho() == TokenKindChirho::VarIdChirho =>
                        {
                            declared_chirho = Some(tok_chirho.text_chirho().to_string());
                        }
                        GreenElementChirho::NodeChirho(name_node_chirho)
                            if !in_kind_chirho
                                && declared_chirho.is_none()
                                && name_node_chirho.kind_chirho()
                                    == SyntaxKindChirho::VarTypeChirho =>
                        {
                            declared_chirho = first_var_id_chirho(name_node_chirho);
                        }
                        part_chirho if in_kind_chirho => {
                            collect_free_type_variables_chirho(
                                part_chirho,
                                bound_chirho,
                                free_chirho,
                            );
                        }
                        _ => {}
                    }
                }
                bound_chirho.extend(declared_chirho);
            }
            other_chirho => {
                collect_free_type_variables_chirho(other_chirho, bound_chirho, free_chirho)
            }
        }
    }
    bound_chirho.truncate(outer_len_chirho);
}
