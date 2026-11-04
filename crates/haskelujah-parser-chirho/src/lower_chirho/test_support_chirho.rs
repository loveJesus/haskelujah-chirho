// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Helpers the lowering tests share: parsing and lowering a source, and reading shapes and
//! placeholders back out of the AST.
//! Moved from `lower_chirho.rs`'s inline tests in row 641.

use haskelujah_ast_chirho::decl_chirho::{ClassMethodChirho, DeclChirho};
use haskelujah_ast_chirho::expr_chirho::{
    ExprChirho, GuardedExprChirho, LocalBindChirho, MatchArmChirho, RhsChirho, StmtChirho,
};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use crate::cst_parser_chirho::ParserChirho;

#[cfg(test)]
pub(super) fn collect_char_literals_from_expr_chirho(
    expr_chirho: &ExprChirho,
    found_punctuation_chars_chirho: &mut Vec<char>,
) {
    match expr_chirho {
        ExprChirho::LitChirho(LitChirho::CharChirho(char_chirho, _, _)) => {
            found_punctuation_chars_chirho.push(*char_chirho);
        }
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            collect_char_literals_from_expr_chirho(fun_chirho, found_punctuation_chars_chirho);
            collect_char_literals_from_expr_chirho(arg_chirho, found_punctuation_chars_chirho);
        }
        ExprChirho::InfixChirho {
            left_chirho,
            right_chirho,
            ..
        } => {
            collect_char_literals_from_expr_chirho(left_chirho, found_punctuation_chars_chirho);
            collect_char_literals_from_expr_chirho(right_chirho, found_punctuation_chars_chirho);
        }
        ExprChirho::ParenChirho { inner_chirho, .. } => {
            collect_char_literals_from_expr_chirho(inner_chirho, found_punctuation_chars_chirho);
        }
        _ => {}
    }
}

/// Guarded arms, a right-hand side's or a multi-way if's:
/// `{prefix}.guard[i].qual[j]` and `{prefix}.body[i]`.
fn collect_placeholder_expr_paths_in_arms_chirho(
    arms_chirho: &[GuardedExprChirho],
    path_prefix_chirho: &str,
    paths_chirho: &mut Vec<String>,
) {
    for (guard_idx_chirho, guard_chirho) in arms_chirho.iter().enumerate() {
        for (qual_idx_chirho, qual_chirho) in guard_chirho.quals_chirho.iter().enumerate() {
            collect_placeholder_expr_paths_in_stmt_chirho(
                qual_chirho,
                &format!("{path_prefix_chirho}.guard[{guard_idx_chirho}].qual[{qual_idx_chirho}]"),
                paths_chirho,
            );
        }
        collect_placeholder_expr_paths_in_expr_chirho(
            &guard_chirho.body_chirho,
            &format!("{path_prefix_chirho}.body[{guard_idx_chirho}]"),
            paths_chirho,
        );
    }
}

pub(super) fn collect_placeholder_expr_paths_in_decl_chirho(
    decl_chirho: &DeclChirho,
    path_prefix_chirho: &str,
    paths_chirho: &mut Vec<String>,
) {
    match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => {
            for (match_idx_chirho, match_chirho) in matches_chirho.iter().enumerate() {
                collect_placeholder_expr_paths_in_match_chirho(
                    match_chirho,
                    &format!("{path_prefix_chirho}.match[{match_idx_chirho}]"),
                    paths_chirho,
                );
            }
        }
        DeclChirho::PatBindChirho { rhs_chirho, .. } => {
            collect_placeholder_expr_paths_in_rhs_chirho(
                rhs_chirho,
                &format!("{path_prefix_chirho}.rhs"),
                paths_chirho,
            );
        }
        _ => {}
    }
}

fn collect_placeholder_expr_paths_in_expr_chirho(
    expr_chirho: &ExprChirho,
    path_prefix_chirho: &str,
    paths_chirho: &mut Vec<String>,
) {
    match expr_chirho {
        ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho().is_empty() => {
            paths_chirho.push(format!(
                "{path_prefix_chirho}@{:?}",
                expr_chirho.span_chirho()
            ));
        }
        ExprChirho::VarChirho(_) | ExprChirho::ConChirho(_) | ExprChirho::LitChirho(_) => {}
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            collect_placeholder_expr_paths_in_expr_chirho(
                fun_chirho,
                &format!("{path_prefix_chirho}.fun"),
                paths_chirho,
            );
            collect_placeholder_expr_paths_in_expr_chirho(
                arg_chirho,
                &format!("{path_prefix_chirho}.arg"),
                paths_chirho,
            );
        }
        ExprChirho::TypeAppChirho { expr_chirho, .. }
        | ExprChirho::NegChirho { expr_chirho, .. }
        | ExprChirho::AnnChirho { expr_chirho, .. } => {
            collect_placeholder_expr_paths_in_expr_chirho(
                expr_chirho,
                &format!("{path_prefix_chirho}.expr"),
                paths_chirho,
            );
        }
        ExprChirho::InfixChirho {
            left_chirho,
            right_chirho,
            ..
        } => {
            collect_placeholder_expr_paths_in_expr_chirho(
                left_chirho,
                &format!("{path_prefix_chirho}.left"),
                paths_chirho,
            );
            collect_placeholder_expr_paths_in_expr_chirho(
                right_chirho,
                &format!("{path_prefix_chirho}.right"),
                paths_chirho,
            );
        }
        ExprChirho::LamChirho { body_chirho, .. }
        | ExprChirho::TypeLamChirho { body_chirho, .. }
        | ExprChirho::ParenChirho {
            inner_chirho: body_chirho,
            ..
        }
        | ExprChirho::SpliceChirho {
            expr_chirho: body_chirho,
            ..
        }
        | ExprChirho::TypedSpliceChirho {
            expr_chirho: body_chirho,
            ..
        }
        | ExprChirho::QuoteExprChirho {
            expr_chirho: body_chirho,
            ..
        } => {
            collect_placeholder_expr_paths_in_expr_chirho(
                body_chirho,
                &format!("{path_prefix_chirho}.body"),
                paths_chirho,
            );
        }
        ExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (bind_idx_chirho, bind_chirho) in binds_chirho.iter().enumerate() {
                collect_placeholder_expr_paths_in_local_bind_chirho(
                    bind_chirho,
                    &format!("{path_prefix_chirho}.let[{bind_idx_chirho}]"),
                    paths_chirho,
                );
            }
            collect_placeholder_expr_paths_in_expr_chirho(
                body_chirho,
                &format!("{path_prefix_chirho}.body"),
                paths_chirho,
            );
        }
        ExprChirho::IfChirho {
            cond_chirho,
            then_chirho,
            else_chirho,
            ..
        } => {
            collect_placeholder_expr_paths_in_expr_chirho(
                cond_chirho,
                &format!("{path_prefix_chirho}.cond"),
                paths_chirho,
            );
            collect_placeholder_expr_paths_in_expr_chirho(
                then_chirho,
                &format!("{path_prefix_chirho}.then"),
                paths_chirho,
            );
            collect_placeholder_expr_paths_in_expr_chirho(
                else_chirho,
                &format!("{path_prefix_chirho}.else"),
                paths_chirho,
            );
        }
        ExprChirho::MultiWayIfChirho { arms_chirho, .. } => {
            collect_placeholder_expr_paths_in_arms_chirho(
                arms_chirho,
                &format!("{path_prefix_chirho}.multiway"),
                paths_chirho,
            );
        }
        ExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            collect_placeholder_expr_paths_in_expr_chirho(
                scrutinee_chirho,
                &format!("{path_prefix_chirho}.scrutinee"),
                paths_chirho,
            );
            for (alt_idx_chirho, alt_chirho) in alts_chirho.iter().enumerate() {
                collect_placeholder_expr_paths_in_rhs_chirho(
                    &alt_chirho.rhs_chirho,
                    &format!("{path_prefix_chirho}.alt[{alt_idx_chirho}].rhs"),
                    paths_chirho,
                );
                for (bind_idx_chirho, bind_chirho) in
                    alt_chirho.where_binds_chirho.iter().enumerate()
                {
                    collect_placeholder_expr_paths_in_local_bind_chirho(
                        bind_chirho,
                        &format!(
                            "{path_prefix_chirho}.alt[{alt_idx_chirho}].where[{bind_idx_chirho}]"
                        ),
                        paths_chirho,
                    );
                }
            }
        }
        ExprChirho::DoChirho { stmts_chirho, .. } => {
            for (stmt_idx_chirho, stmt_chirho) in stmts_chirho.iter().enumerate() {
                collect_placeholder_expr_paths_in_stmt_chirho(
                    stmt_chirho,
                    &format!("{path_prefix_chirho}.stmt[{stmt_idx_chirho}]"),
                    paths_chirho,
                );
            }
        }
        ExprChirho::TupleChirho {
            elements_chirho, ..
        }
        | ExprChirho::ListChirho {
            elements_chirho, ..
        } => {
            for (element_idx_chirho, element_chirho) in elements_chirho.iter().enumerate() {
                collect_placeholder_expr_paths_in_expr_chirho(
                    element_chirho,
                    &format!("{path_prefix_chirho}.element[{element_idx_chirho}]"),
                    paths_chirho,
                );
            }
        }
        ExprChirho::ArithSeqChirho {
            from_chirho,
            then_chirho,
            to_chirho,
            ..
        } => {
            collect_placeholder_expr_paths_in_expr_chirho(
                from_chirho,
                &format!("{path_prefix_chirho}.from"),
                paths_chirho,
            );
            if let Some(then_expr_chirho) = then_chirho {
                collect_placeholder_expr_paths_in_expr_chirho(
                    then_expr_chirho,
                    &format!("{path_prefix_chirho}.then"),
                    paths_chirho,
                );
            }
            if let Some(to_expr_chirho) = to_chirho {
                collect_placeholder_expr_paths_in_expr_chirho(
                    to_expr_chirho,
                    &format!("{path_prefix_chirho}.to"),
                    paths_chirho,
                );
            }
        }
        ExprChirho::ListCompChirho {
            body_chirho,
            quals_chirho,
            parallel_quals_chirho,
            ..
        } => {
            collect_placeholder_expr_paths_in_expr_chirho(
                body_chirho,
                &format!("{path_prefix_chirho}.body"),
                paths_chirho,
            );
            for (qual_idx_chirho, qual_chirho) in quals_chirho.iter().enumerate() {
                collect_placeholder_expr_paths_in_stmt_chirho(
                    qual_chirho,
                    &format!("{path_prefix_chirho}.qual[{qual_idx_chirho}]"),
                    paths_chirho,
                );
            }
            for (group_idx_chirho, group_chirho) in parallel_quals_chirho.iter().enumerate() {
                for (qual_idx_chirho, qual_chirho) in group_chirho.iter().enumerate() {
                    collect_placeholder_expr_paths_in_stmt_chirho(
                        qual_chirho,
                        &format!(
                            "{path_prefix_chirho}.parallel[{group_idx_chirho}][{qual_idx_chirho}]"
                        ),
                        paths_chirho,
                    );
                }
            }
        }
        ExprChirho::LeftSectionChirho { arg_chirho, .. }
        | ExprChirho::RightSectionChirho { arg_chirho, .. } => {
            collect_placeholder_expr_paths_in_expr_chirho(
                arg_chirho,
                &format!("{path_prefix_chirho}.arg"),
                paths_chirho,
            );
        }
        ExprChirho::RecordConChirho { fields_chirho, .. } => {
            for (field_idx_chirho, field_chirho) in fields_chirho.iter().enumerate() {
                collect_placeholder_expr_paths_in_expr_chirho(
                    &field_chirho.value_chirho,
                    &format!("{path_prefix_chirho}.field[{field_idx_chirho}]"),
                    paths_chirho,
                );
            }
        }
        ExprChirho::RecordUpdateChirho {
            expr_chirho,
            fields_chirho,
            ..
        } => {
            collect_placeholder_expr_paths_in_expr_chirho(
                expr_chirho,
                &format!("{path_prefix_chirho}.record"),
                paths_chirho,
            );
            for (field_idx_chirho, field_chirho) in fields_chirho.iter().enumerate() {
                collect_placeholder_expr_paths_in_expr_chirho(
                    &field_chirho.value_chirho,
                    &format!("{path_prefix_chirho}.field[{field_idx_chirho}]"),
                    paths_chirho,
                );
            }
        }
        ExprChirho::QuoteDeclChirho { decls_chirho, .. } => {
            for (decl_idx_chirho, inner_decl_chirho) in decls_chirho.iter().enumerate() {
                collect_placeholder_expr_paths_in_decl_chirho(
                    inner_decl_chirho,
                    &format!("{path_prefix_chirho}.decl[{decl_idx_chirho}]"),
                    paths_chirho,
                );
            }
        }
        ExprChirho::QuoteTypeChirho { .. } | ExprChirho::QuotePatChirho { .. } => {}
    }
}

fn collect_placeholder_expr_paths_in_local_bind_chirho(
    bind_chirho: &LocalBindChirho,
    path_prefix_chirho: &str,
    paths_chirho: &mut Vec<String>,
) {
    match bind_chirho {
        LocalBindChirho::FunBindChirho { matches_chirho, .. } => {
            for (match_idx_chirho, match_chirho) in matches_chirho.iter().enumerate() {
                collect_placeholder_expr_paths_in_match_chirho(
                    match_chirho,
                    &format!("{path_prefix_chirho}.match[{match_idx_chirho}]"),
                    paths_chirho,
                );
            }
        }
        LocalBindChirho::PatBindChirho { rhs_chirho, .. } => {
            collect_placeholder_expr_paths_in_rhs_chirho(
                rhs_chirho,
                &format!("{path_prefix_chirho}.rhs"),
                paths_chirho,
            );
        }
        LocalBindChirho::TypeSigChirho { .. } => {}
    }
}

fn collect_placeholder_expr_paths_in_match_chirho(
    match_chirho: &MatchArmChirho,
    path_prefix_chirho: &str,
    paths_chirho: &mut Vec<String>,
) {
    collect_placeholder_expr_paths_in_rhs_chirho(
        &match_chirho.rhs_chirho,
        &format!("{path_prefix_chirho}.rhs"),
        paths_chirho,
    );
    for (bind_idx_chirho, bind_chirho) in match_chirho.where_binds_chirho.iter().enumerate() {
        collect_placeholder_expr_paths_in_local_bind_chirho(
            bind_chirho,
            &format!("{path_prefix_chirho}.where[{bind_idx_chirho}]"),
            paths_chirho,
        );
    }
}

fn collect_placeholder_expr_paths_in_rhs_chirho(
    rhs_chirho: &RhsChirho,
    path_prefix_chirho: &str,
    paths_chirho: &mut Vec<String>,
) {
    match rhs_chirho {
        RhsChirho::UnguardedChirho(expr_chirho) => {
            collect_placeholder_expr_paths_in_expr_chirho(
                expr_chirho,
                path_prefix_chirho,
                paths_chirho,
            );
        }
        RhsChirho::GuardedChirho(guards_chirho) => {
            collect_placeholder_expr_paths_in_arms_chirho(
                guards_chirho,
                path_prefix_chirho,
                paths_chirho,
            );
        }
    }
}

/// One statement or qualifier: `{prefix}.expr`, `{prefix}.bind` or
/// `{prefix}.let[i]`.
fn collect_placeholder_expr_paths_in_stmt_chirho(
    stmt_chirho: &StmtChirho,
    path_prefix_chirho: &str,
    paths_chirho: &mut Vec<String>,
) {
    match stmt_chirho {
        StmtChirho::ExprChirho { expr_chirho, .. } => {
            collect_placeholder_expr_paths_in_expr_chirho(
                expr_chirho,
                &format!("{path_prefix_chirho}.expr"),
                paths_chirho,
            );
        }
        StmtChirho::BindChirho { expr_chirho, .. } => {
            collect_placeholder_expr_paths_in_expr_chirho(
                expr_chirho,
                &format!("{path_prefix_chirho}.bind"),
                paths_chirho,
            );
        }
        StmtChirho::LetChirho { binds_chirho, .. } => {
            for (bind_idx_chirho, bind_chirho) in binds_chirho.iter().enumerate() {
                collect_placeholder_expr_paths_in_local_bind_chirho(
                    bind_chirho,
                    &format!("{path_prefix_chirho}.let[{bind_idx_chirho}]"),
                    paths_chirho,
                );
            }
        }
    }
}

fn constraint_shape_chirho(constraint_chirho: &ConstraintChirho) -> String {
    match constraint_chirho {
        ConstraintChirho::ClassChirho {
            class_chirho,
            kind_args_chirho,
            args_chirho,
            ..
        } => std::iter::once(class_chirho.text_chirho().to_string())
            .chain(
                kind_args_chirho
                    .iter()
                    .map(|kind_chirho| format!("@{}", type_shape_chirho(kind_chirho))),
            )
            .chain(args_chirho.iter().map(type_shape_chirho))
            .collect::<Vec<_>>()
            .join(" "),
        ConstraintChirho::KindAnnotationChirho {
            constraint_chirho,
            kind_chirho,
            ..
        } => format!(
            "({} :: {})",
            constraint_shape_chirho(constraint_chirho),
            type_shape_chirho(kind_chirho)
        ),
        ConstraintChirho::QuantifiedChirho {
            vars_chirho,
            context_chirho,
            body_chirho,
            ..
        } => format!(
            "forall {}. ({}) => {}",
            vars_chirho
                .iter()
                .map(|var_chirho| var_chirho.name_chirho.text_chirho().to_string())
                .collect::<Vec<_>>()
                .join(" "),
            context_chirho
                .iter()
                .map(constraint_shape_chirho)
                .collect::<Vec<_>>()
                .join(", "),
            constraint_shape_chirho(body_chirho)
        ),
    }
}

pub(super) fn find_class_decl_chirho<'a>(
    module_chirho: &'a ModuleChirho,
    class_name_chirho: &str,
) -> &'a DeclChirho {
    module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::ClassDeclChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == class_name_chirho
            )
        })
        .unwrap_or_else(|| panic!("should lower class declaration for {class_name_chirho}"))
}

pub(super) fn find_class_method_chirho<'a>(
    methods_chirho: &'a [ClassMethodChirho],
    method_name_chirho: &str,
) -> &'a ClassMethodChirho {
    methods_chirho
        .iter()
        .find(|method_chirho| method_chirho.name_chirho.text_chirho() == method_name_chirho)
        .unwrap_or_else(|| panic!("should lower method declaration for {method_name_chirho}"))
}

pub(super) fn find_fun_decl_chirho<'a>(
    module_chirho: &'a ModuleChirho,
    fun_name_chirho: &str,
) -> &'a DeclChirho {
    let decl_summaries_chirho = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                Some(format!("FunBind:{}", name_chirho.text_chirho()))
            }
            DeclChirho::TypeSigChirho { name_chirho, .. } => {
                Some(format!("TypeSig:{}", name_chirho.text_chirho()))
            }
            DeclChirho::PatBindChirho { .. } => Some("PatBind".to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();
    module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(
                decl_chirho,
                DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == fun_name_chirho
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "should lower function declaration for {fun_name_chirho}; decl summaries: {:?}",
                decl_summaries_chirho
            )
        })
}

pub(super) fn instance_head_shape_chirho(types_chirho: &[TypeChirho]) -> String {
    types_chirho
        .iter()
        .map(type_shape_chirho)
        .collect::<Vec<_>>()
        .join(" | ")
}

pub(super) fn parse_and_lower_chirho(source_chirho: &str) -> ModuleChirho {
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let parser_chirho = ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();
    lower_module_chirho(&green_chirho, file_id_chirho)
}

pub(super) fn type_shape_chirho(ty_chirho: &TypeChirho) -> String {
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho) | TypeChirho::ConChirho(name_chirho) => {
            name_chirho.text_chirho().to_string()
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => format!(
            "({} {})",
            type_shape_chirho(fun_chirho),
            type_shape_chirho(arg_chirho)
        ),
        TypeChirho::KindApplicationChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => format!(
            "({} @{})",
            type_shape_chirho(fun_chirho),
            type_shape_chirho(arg_chirho)
        ),
        TypeChirho::KindAnnotationChirho {
            ty_chirho,
            kind_chirho,
            ..
        } => format!(
            "({} :: {})",
            type_shape_chirho(ty_chirho),
            type_shape_chirho(kind_chirho)
        ),
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => format!(
            "({} -> {})",
            type_shape_chirho(arg_chirho),
            type_shape_chirho(result_chirho)
        ),
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => format!(
            "({})",
            elements_chirho
                .iter()
                .map(type_shape_chirho)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        TypeChirho::ListChirho { element_chirho, .. } => {
            format!("[{}]", type_shape_chirho(element_chirho))
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => type_shape_chirho(inner_chirho),
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => format!(
            "({}) => {}",
            context_chirho
                .iter()
                .map(constraint_shape_chirho)
                .collect::<Vec<_>>()
                .join(", "),
            type_shape_chirho(body_chirho)
        ),
        TypeChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => format!(
            "forall {} -> {}",
            vars_chirho
                .iter()
                .map(|var_chirho| var_chirho.name_chirho.text_chirho().to_string())
                .collect::<Vec<_>>()
                .join(" "),
            type_shape_chirho(body_chirho)
        ),
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => format!(
            "forall {}. {}",
            vars_chirho
                .iter()
                .map(|var_chirho| var_chirho.name_chirho.text_chirho().to_string())
                .collect::<Vec<_>>()
                .join(" "),
            type_shape_chirho(body_chirho)
        ),
        TypeChirho::PromotedConChirho { name_chirho, .. } => {
            format!("'{}", name_chirho.text_chirho())
        }
        TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => format!(
            "'[{}]",
            elements_chirho
                .iter()
                .map(type_shape_chirho)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        TypeChirho::WildcardChirho { .. } => "_".to_string(),
        TypeChirho::LitChirho { value_chirho, .. } => value_chirho.clone(),
    }
}
