// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Typed input failures and a borrowed pre-Core syntax check. The module's
//! existing projection allowance owns this traversal; no Core is published on
//! error, including patterns that would otherwise bind no names.
//! Workflow: language-features-chirho/pattern-matching-chirho.

use haskelujah_ast_chirho::decl_chirho::{
    ClassMethodChirho, ConstructorSignatureErrorChirho, DeclChirho, GadtProjectionBudgetChirho,
    GadtProjectionErrorChirho, PatSynDirChirho,
};
use haskelujah_ast_chirho::expr_chirho::{
    AltChirho, ComprehensionQualifierChirho, ExprChirho, FieldAssignChirho, GuardedExprChirho,
    LocalBindChirho, MatchArmChirho, RhsChirho, StmtChirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::pat_chirho::{PatChirho, PatFieldChirho};
use haskelujah_span_chirho::SpanChirho;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesugarInputErrorChirho {
    ConstructorChirho(ConstructorSignatureErrorChirho),
    ProjectionChirho(GadtProjectionErrorChirho),
    UnexpandedPatternChirho { span_chirho: SpanChirho },
    UnreadTransformChirho { span_chirho: SpanChirho },
}

impl DesugarInputErrorChirho {
    pub fn span_chirho(self) -> SpanChirho {
        match self {
            Self::ConstructorChirho(error_chirho) => error_chirho.span_chirho(),
            Self::ProjectionChirho(error_chirho) => error_chirho.span_chirho,
            Self::UnexpandedPatternChirho { span_chirho }
            | Self::UnreadTransformChirho { span_chirho } => span_chirho,
        }
    }
    pub fn ghc_code_chirho(self) -> Option<&'static str> {
        match self {
            Self::ConstructorChirho(error_chirho) => error_chirho.ghc_code_chirho(),
            Self::ProjectionChirho(_)
            | Self::UnexpandedPatternChirho { .. }
            | Self::UnreadTransformChirho { .. } => None,
        }
    }
}
impl From<ConstructorSignatureErrorChirho> for DesugarInputErrorChirho {
    fn from(error_chirho: ConstructorSignatureErrorChirho) -> Self {
        Self::ConstructorChirho(error_chirho)
    }
}
impl From<GadtProjectionErrorChirho> for DesugarInputErrorChirho {
    fn from(error_chirho: GadtProjectionErrorChirho) -> Self {
        Self::ProjectionChirho(error_chirho)
    }
}
impl std::fmt::Display for DesugarInputErrorChirho {
    fn fmt(&self, formatter_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConstructorChirho(error_chirho) => error_chirho.fmt(formatter_chirho),
            Self::ProjectionChirho(error_chirho) => error_chirho.fmt(formatter_chirho),
            Self::UnexpandedPatternChirho { .. } => {
                formatter_chirho.write_str("unsupported unexpanded executable pattern splice")
            }
            Self::UnreadTransformChirho { .. } => {
                formatter_chirho.write_str("unread comprehension transformer")
            }
        }
    }
}
impl std::error::Error for DesugarInputErrorChirho {}

enum NodeChirho<'syntax_chirho> {
    DeclarationsChirho(&'syntax_chirho [DeclChirho]),
    DeclarationChirho(&'syntax_chirho DeclChirho),
    ExpressionChirho(&'syntax_chirho ExprChirho),
    ExpressionsChirho(&'syntax_chirho [ExprChirho]),
    PatternChirho(&'syntax_chirho PatChirho),
    PatternsChirho(&'syntax_chirho [PatChirho]),
    LocalBindingsChirho(&'syntax_chirho [LocalBindChirho]),
    LocalBindingChirho(&'syntax_chirho LocalBindChirho),
    MatchesChirho(&'syntax_chirho [MatchArmChirho]),
    MatchChirho(&'syntax_chirho MatchArmChirho),
    RhsChirho(&'syntax_chirho RhsChirho),
    GuardsChirho(&'syntax_chirho [GuardedExprChirho]),
    GuardChirho(&'syntax_chirho GuardedExprChirho),
    StatementsChirho(&'syntax_chirho [StmtChirho]),
    StatementChirho(&'syntax_chirho StmtChirho),
    QualifiersChirho(&'syntax_chirho [ComprehensionQualifierChirho]),
    QualifierChirho(&'syntax_chirho ComprehensionQualifierChirho),
    QualifierProjectionChirho(&'syntax_chirho [ComprehensionQualifierChirho]),
    BranchesChirho(&'syntax_chirho [Vec<ComprehensionQualifierChirho>]),
    AlternativesChirho(&'syntax_chirho [AltChirho]),
    AlternativeChirho(&'syntax_chirho AltChirho),
    FieldsChirho(&'syntax_chirho [FieldAssignChirho]),
    PatternFieldsChirho(&'syntax_chirho [PatFieldChirho]),
    MethodsChirho(&'syntax_chirho [ClassMethodChirho]),
}

fn push_chirho<'syntax_chirho>(
    pending_chirho: &mut Vec<(NodeChirho<'syntax_chirho>, usize, SpanChirho)>,
    budget_chirho: &mut GadtProjectionBudgetChirho,
    node_chirho: NodeChirho<'syntax_chirho>,
    depth_chirho: usize,
    span_chirho: SpanChirho,
) -> Result<(), DesugarInputErrorChirho> {
    budget_chirho.visit_borrowed_syntax_chirho(span_chirho, depth_chirho)?;
    pending_chirho.push((node_chirho, depth_chirho, span_chirho));
    Ok(())
}

/// Quoted patterns are data at the Core boundary. Every executable pattern,
/// including lazy/no-binder and nested record/view patterns, must be expanded.
pub(super) fn check_executable_patterns_chirho(
    module_chirho: &ModuleChirho,
    budget_chirho: &mut GadtProjectionBudgetChirho,
    context_chirho: Option<&super::DesugarCtxChirho>,
) -> Result<bool, DesugarInputErrorChirho> {
    let mut expanded_rows_chirho = false;
    let mut pending_chirho = Vec::new();
    push_chirho(
        &mut pending_chirho,
        budget_chirho,
        NodeChirho::DeclarationsChirho(&module_chirho.decls_chirho),
        1,
        module_chirho.span_chirho,
    )?;
    while let Some((node_chirho, depth_chirho, span_chirho)) = pending_chirho.pop() {
        // Slice cursors retain only one child and a borrowed suffix, bounding
        // pending storage by nesting rather than the width of a declaration.
        macro_rules! push_chirho {
            ($node_chirho:expr) => {
                push_chirho(
                    &mut pending_chirho,
                    budget_chirho,
                    $node_chirho,
                    depth_chirho + 1,
                    span_chirho,
                )?
            };
            ($node_chirho:expr, $depth_chirho:expr, $span_chirho:expr) => {
                push_chirho(
                    &mut pending_chirho,
                    budget_chirho,
                    $node_chirho,
                    $depth_chirho,
                    $span_chirho,
                )?
            };
        }
        macro_rules! slice_chirho {
            ($slice_chirho:expr, $rest_chirho:ident, $first_chirho:ident) => {
                if let Some((first_chirho, rest_chirho)) = $slice_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho!(
                            NodeChirho::$rest_chirho(rest_chirho),
                            depth_chirho,
                            span_chirho
                        );
                    }
                    push_chirho!(NodeChirho::$first_chirho(first_chirho));
                }
            };
        }
        match node_chirho {
            NodeChirho::DeclarationsChirho(slice_chirho) => {
                slice_chirho!(slice_chirho, DeclarationsChirho, DeclarationChirho);
            }
            NodeChirho::ExpressionsChirho(slice_chirho) => {
                slice_chirho!(slice_chirho, ExpressionsChirho, ExpressionChirho);
            }
            NodeChirho::PatternsChirho(slice_chirho) => {
                slice_chirho!(slice_chirho, PatternsChirho, PatternChirho);
            }
            NodeChirho::LocalBindingsChirho(slice_chirho) => {
                slice_chirho!(slice_chirho, LocalBindingsChirho, LocalBindingChirho);
            }
            NodeChirho::MatchesChirho(slice_chirho) => {
                slice_chirho!(slice_chirho, MatchesChirho, MatchChirho);
            }
            NodeChirho::GuardsChirho(slice_chirho) => {
                slice_chirho!(slice_chirho, GuardsChirho, GuardChirho);
            }
            NodeChirho::StatementsChirho(slice_chirho) => {
                slice_chirho!(slice_chirho, StatementsChirho, StatementChirho);
            }
            NodeChirho::QualifiersChirho(slice_chirho) => {
                slice_chirho!(slice_chirho, QualifiersChirho, QualifierChirho);
            }
            NodeChirho::QualifierChirho(qualifier_chirho) => match qualifier_chirho {
                ComprehensionQualifierChirho::StatementChirho(statement_chirho) => {
                    push_chirho!(NodeChirho::StatementChirho(statement_chirho))
                }
                ComprehensionQualifierChirho::TransformChirho(transform_chirho) => {
                    let Some(using_chirho) = &transform_chirho.using_chirho else {
                        return Err(DesugarInputErrorChirho::UnreadTransformChirho {
                            span_chirho: transform_chirho.span_chirho,
                        });
                    };
                    push_chirho!(NodeChirho::ExpressionChirho(using_chirho));
                    if let Some(by_chirho) = &transform_chirho.by_chirho {
                        push_chirho!(NodeChirho::ExpressionChirho(by_chirho));
                    }
                }
            },
            NodeChirho::QualifierProjectionChirho(qualifiers_chirho) => {
                let Some(context_chirho) = context_chirho else {
                    continue;
                };
                let columns_chirho = context_chirho
                    .comprehension_columns_chirho(qualifiers_chirho)
                    .len();
                budget_chirho.reserve_transport_chirho(
                    columns_chirho.saturating_mul(32),
                    depth_chirho,
                    span_chirho,
                )?;
                // All actual prefix patterns have passed the borrowed depth
                // cursor before the matcher's complete column inventory runs.
                for (index_chirho, qualifier_chirho) in qualifiers_chirho.iter().enumerate() {
                    let ComprehensionQualifierChirho::TransformChirho(transform_chirho) =
                        qualifier_chirho
                    else {
                        continue;
                    };
                    let count_chirho = context_chirho
                        .comprehension_columns_chirho(&qualifiers_chirho[..index_chirho])
                        .len();
                    // Each group column maps the same row stream and has one
                    // tuple selector. Reserve its actual quadratic selector
                    // footprint, loop nodes and key/prefix row before Core.
                    let selectors_chirho = if transform_chirho.group_chirho {
                        count_chirho
                            .checked_mul(count_chirho)
                            .and_then(|selectors_chirho| selectors_chirho.checked_mul(128))
                    } else {
                        Some(0)
                    };
                    let weight_chirho = selectors_chirho
                        .and_then(|selectors_chirho| {
                            count_chirho.checked_mul(64).and_then(|columns_chirho| {
                                selectors_chirho.checked_add(columns_chirho)
                            })
                        })
                        .and_then(|weight_chirho| weight_chirho.checked_add(128))
                        .unwrap_or(usize::MAX);
                    budget_chirho.reserve_transport_chirho(
                        weight_chirho,
                        depth_chirho,
                        transform_chirho.span_chirho,
                    )?;
                }
            }
            NodeChirho::AlternativesChirho(slice_chirho) => {
                slice_chirho!(slice_chirho, AlternativesChirho, AlternativeChirho);
            }
            NodeChirho::BranchesChirho(slice_chirho) => {
                if let Some((first_chirho, rest_chirho)) = slice_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho!(
                            NodeChirho::BranchesChirho(rest_chirho),
                            depth_chirho,
                            span_chirho
                        );
                    }
                    push_chirho!(NodeChirho::QualifierProjectionChirho(first_chirho));
                    push_chirho!(NodeChirho::QualifiersChirho(first_chirho));
                }
            }
            NodeChirho::FieldsChirho(slice_chirho) => {
                if let Some((first_chirho, rest_chirho)) = slice_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho!(
                            NodeChirho::FieldsChirho(rest_chirho),
                            depth_chirho,
                            span_chirho
                        );
                    }
                    push_chirho!(
                        NodeChirho::ExpressionChirho(&first_chirho.value_chirho),
                        depth_chirho + 1,
                        first_chirho.span_chirho
                    );
                }
            }
            NodeChirho::PatternFieldsChirho(slice_chirho) => {
                if let Some((first_chirho, rest_chirho)) = slice_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho!(
                            NodeChirho::PatternFieldsChirho(rest_chirho),
                            depth_chirho,
                            span_chirho
                        );
                    }
                    push_chirho!(
                        NodeChirho::PatternChirho(&first_chirho.pattern_chirho),
                        depth_chirho + 1,
                        first_chirho.span_chirho
                    );
                }
            }
            NodeChirho::MethodsChirho(slice_chirho) => {
                if let Some((first_chirho, rest_chirho)) = slice_chirho.split_first() {
                    if !rest_chirho.is_empty() {
                        push_chirho!(
                            NodeChirho::MethodsChirho(rest_chirho),
                            depth_chirho,
                            span_chirho
                        );
                    }
                    if let Some(matches_chirho) = &first_chirho.default_chirho {
                        push_chirho!(
                            NodeChirho::MatchesChirho(matches_chirho),
                            depth_chirho + 1,
                            first_chirho.span_chirho
                        );
                    }
                }
            }
            NodeChirho::DeclarationChirho(declaration_chirho) => match declaration_chirho {
                DeclChirho::FunBindChirho { matches_chirho, .. } => {
                    push_chirho!(NodeChirho::MatchesChirho(matches_chirho));
                }
                DeclChirho::PatBindChirho {
                    pat_chirho,
                    rhs_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::RhsChirho(rhs_chirho));
                    push_chirho!(NodeChirho::PatternChirho(pat_chirho));
                }
                DeclChirho::ClassDeclChirho { methods_chirho, .. } => {
                    push_chirho!(NodeChirho::MethodsChirho(methods_chirho));
                }
                DeclChirho::InstanceDeclChirho { methods_chirho, .. } => {
                    push_chirho!(NodeChirho::LocalBindingsChirho(methods_chirho));
                }
                DeclChirho::PatSynDeclChirho {
                    pat_chirho,
                    dir_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::PatternChirho(pat_chirho));
                    if let PatSynDirChirho::ExplBidirChirho {
                        builder_binds_chirho,
                    } = dir_chirho
                    {
                        push_chirho!(NodeChirho::LocalBindingsChirho(builder_binds_chirho));
                    }
                }
                DeclChirho::SpliceDeclChirho { expr_chirho, .. } => {
                    push_chirho!(NodeChirho::ExpressionChirho(expr_chirho));
                }
                DeclChirho::TypeSigChirho { .. }
                | DeclChirho::DataDeclChirho { .. }
                | DeclChirho::NewtypeDeclChirho { .. }
                | DeclChirho::TypeAliasDeclChirho { .. }
                | DeclChirho::TypeFamilyDeclChirho { .. }
                | DeclChirho::TypeFamilyInstanceDeclChirho { .. }
                | DeclChirho::FixityDeclChirho { .. }
                | DeclChirho::DefaultDeclChirho { .. }
                | DeclChirho::ForeignDeclChirho { .. }
                | DeclChirho::PatSynSigDeclChirho { .. }
                | DeclChirho::RoleAnnotationDeclChirho { .. }
                | DeclChirho::StandaloneDerivingDeclChirho { .. } => {}
            },
            NodeChirho::LocalBindingChirho(binding_chirho) => match binding_chirho {
                LocalBindChirho::FunBindChirho { matches_chirho, .. } => {
                    push_chirho!(NodeChirho::MatchesChirho(matches_chirho));
                }
                LocalBindChirho::PatBindChirho {
                    pat_chirho,
                    rhs_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::RhsChirho(rhs_chirho));
                    push_chirho!(NodeChirho::PatternChirho(pat_chirho));
                }
                LocalBindChirho::TypeSigChirho { .. } => {}
            },
            NodeChirho::MatchChirho(match_chirho) => {
                push_chirho!(NodeChirho::LocalBindingsChirho(
                    &match_chirho.where_binds_chirho
                ));
                push_chirho!(NodeChirho::RhsChirho(&match_chirho.rhs_chirho));
                push_chirho!(NodeChirho::PatternsChirho(&match_chirho.pats_chirho));
            }
            NodeChirho::AlternativeChirho(alternative_chirho) => {
                push_chirho!(NodeChirho::LocalBindingsChirho(
                    &alternative_chirho.where_binds_chirho
                ));
                push_chirho!(NodeChirho::RhsChirho(&alternative_chirho.rhs_chirho));
                push_chirho!(NodeChirho::PatternChirho(&alternative_chirho.pat_chirho));
            }
            NodeChirho::RhsChirho(rhs_chirho) => match rhs_chirho {
                RhsChirho::UnguardedChirho(expression_chirho) => {
                    push_chirho!(NodeChirho::ExpressionChirho(expression_chirho));
                }
                RhsChirho::GuardedChirho(guards_chirho) => {
                    push_chirho!(NodeChirho::GuardsChirho(guards_chirho));
                }
            },
            NodeChirho::GuardChirho(guard_chirho) => {
                push_chirho!(NodeChirho::ExpressionChirho(&guard_chirho.body_chirho));
                push_chirho!(NodeChirho::StatementsChirho(&guard_chirho.quals_chirho));
            }
            NodeChirho::StatementChirho(statement_chirho) => match statement_chirho {
                StmtChirho::ExprChirho { expr_chirho, .. } => {
                    push_chirho!(NodeChirho::ExpressionChirho(expr_chirho));
                }
                StmtChirho::BindChirho {
                    pat_chirho,
                    expr_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::ExpressionChirho(expr_chirho));
                    push_chirho!(NodeChirho::PatternChirho(pat_chirho));
                }
                StmtChirho::LetChirho { binds_chirho, .. } => {
                    push_chirho!(NodeChirho::LocalBindingsChirho(binds_chirho));
                }
            },
            NodeChirho::PatternChirho(pattern_chirho) => match pattern_chirho {
                PatChirho::SpliceChirho { span_chirho, .. } => {
                    return Err(DesugarInputErrorChirho::UnexpandedPatternChirho {
                        span_chirho: *span_chirho,
                    });
                }
                PatChirho::ConChirho { args_chirho, .. } => {
                    push_chirho!(NodeChirho::PatternsChirho(args_chirho));
                }
                PatChirho::TupleChirho {
                    elements_chirho, ..
                }
                | PatChirho::ListChirho {
                    elements_chirho, ..
                } => {
                    push_chirho!(NodeChirho::PatternsChirho(elements_chirho));
                }
                PatChirho::AsChirho { pattern_chirho, .. } => {
                    push_chirho!(NodeChirho::PatternChirho(pattern_chirho));
                }
                PatChirho::ParenChirho { inner_chirho, .. }
                | PatChirho::LazyChirho { inner_chirho, .. }
                | PatChirho::BangChirho { inner_chirho, .. } => {
                    push_chirho!(NodeChirho::PatternChirho(inner_chirho));
                }
                PatChirho::InfixConChirho {
                    left_chirho,
                    right_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::PatternChirho(right_chirho));
                    push_chirho!(NodeChirho::PatternChirho(left_chirho));
                }
                PatChirho::RecordChirho { fields_chirho, .. } => {
                    push_chirho!(NodeChirho::PatternFieldsChirho(fields_chirho));
                }
                PatChirho::ViewChirho {
                    expr_chirho,
                    pat_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::ExpressionChirho(expr_chirho));
                    push_chirho!(NodeChirho::PatternChirho(pat_chirho));
                }
                PatChirho::TypeAnnotChirho { pat_chirho, .. } => {
                    push_chirho!(NodeChirho::PatternChirho(pat_chirho));
                }
                PatChirho::VarChirho(_)
                | PatChirho::LitChirho(_)
                | PatChirho::WildcardChirho(_)
                | PatChirho::NegChirho { .. } => {}
            },
            NodeChirho::ExpressionChirho(expression_chirho) => match expression_chirho {
                ExprChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::ExpressionChirho(arg_chirho));
                    push_chirho!(NodeChirho::ExpressionChirho(fun_chirho));
                }
                ExprChirho::InfixChirho {
                    left_chirho,
                    right_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::ExpressionChirho(right_chirho));
                    push_chirho!(NodeChirho::ExpressionChirho(left_chirho));
                }
                ExprChirho::TypeAppChirho { expr_chirho, .. }
                | ExprChirho::NegChirho { expr_chirho, .. }
                | ExprChirho::AnnChirho { expr_chirho, .. }
                | ExprChirho::SpliceChirho { expr_chirho, .. }
                | ExprChirho::TypedSpliceChirho { expr_chirho, .. } => {
                    push_chirho!(NodeChirho::ExpressionChirho(expr_chirho));
                }
                ExprChirho::LamChirho {
                    pats_chirho,
                    body_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::ExpressionChirho(body_chirho));
                    push_chirho!(NodeChirho::PatternsChirho(pats_chirho));
                }
                ExprChirho::TypeLamChirho { body_chirho, .. } => {
                    push_chirho!(NodeChirho::ExpressionChirho(body_chirho));
                }
                ExprChirho::LetChirho {
                    binds_chirho,
                    body_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::ExpressionChirho(body_chirho));
                    push_chirho!(NodeChirho::LocalBindingsChirho(binds_chirho));
                }
                ExprChirho::IfChirho {
                    cond_chirho,
                    then_chirho,
                    else_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::ExpressionChirho(else_chirho));
                    push_chirho!(NodeChirho::ExpressionChirho(then_chirho));
                    push_chirho!(NodeChirho::ExpressionChirho(cond_chirho));
                }
                ExprChirho::MultiWayIfChirho { arms_chirho, .. } => {
                    push_chirho!(NodeChirho::GuardsChirho(arms_chirho));
                }
                ExprChirho::CaseChirho {
                    scrutinee_chirho,
                    alts_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::AlternativesChirho(alts_chirho));
                    push_chirho!(NodeChirho::ExpressionChirho(scrutinee_chirho));
                }
                ExprChirho::DoChirho { stmts_chirho, .. } => {
                    push_chirho!(NodeChirho::StatementsChirho(stmts_chirho));
                }
                ExprChirho::TupleChirho {
                    elements_chirho, ..
                }
                | ExprChirho::ListChirho {
                    elements_chirho, ..
                } => {
                    push_chirho!(NodeChirho::ExpressionsChirho(elements_chirho));
                }
                ExprChirho::ArithSeqChirho {
                    from_chirho,
                    then_chirho,
                    to_chirho,
                    ..
                } => {
                    if let Some(to_chirho) = to_chirho {
                        push_chirho!(NodeChirho::ExpressionChirho(to_chirho));
                    }
                    if let Some(then_chirho) = then_chirho {
                        push_chirho!(NodeChirho::ExpressionChirho(then_chirho));
                    }
                    push_chirho!(NodeChirho::ExpressionChirho(from_chirho));
                }
                ExprChirho::ListCompChirho {
                    body_chirho,
                    quals_chirho,
                    parallel_quals_chirho,
                    ..
                } => {
                    let has_transform_chirho = quals_chirho.iter().any(|qualifier_chirho| {
                        matches!(
                            qualifier_chirho,
                            ComprehensionQualifierChirho::TransformChirho(_)
                        )
                    });
                    expanded_rows_chirho |=
                        has_transform_chirho || !parallel_quals_chirho.is_empty();
                    if context_chirho.is_some() {
                        budget_chirho.reserve_transport_chirho(
                            parallel_quals_chirho.len().saturating_mul(128),
                            depth_chirho.saturating_add(parallel_quals_chirho.len()),
                            span_chirho,
                        )?;
                    }
                    push_chirho!(NodeChirho::ExpressionChirho(body_chirho));
                    if quals_chirho.iter().any(|qualifier_chirho| {
                        matches!(
                            qualifier_chirho,
                            ComprehensionQualifierChirho::TransformChirho(_)
                        )
                    }) {
                        push_chirho!(NodeChirho::QualifierProjectionChirho(quals_chirho));
                    }
                    push_chirho!(NodeChirho::QualifiersChirho(quals_chirho));
                    push_chirho!(NodeChirho::BranchesChirho(parallel_quals_chirho));
                }
                ExprChirho::LeftSectionChirho { arg_chirho, .. }
                | ExprChirho::RightSectionChirho { arg_chirho, .. } => {
                    push_chirho!(NodeChirho::ExpressionChirho(arg_chirho));
                }
                ExprChirho::ParenChirho { inner_chirho, .. } => {
                    push_chirho!(NodeChirho::ExpressionChirho(inner_chirho));
                }
                ExprChirho::RecordConChirho { fields_chirho, .. } => {
                    push_chirho!(NodeChirho::FieldsChirho(fields_chirho));
                }
                ExprChirho::RecordUpdateChirho {
                    expr_chirho,
                    fields_chirho,
                    ..
                } => {
                    push_chirho!(NodeChirho::FieldsChirho(fields_chirho));
                    push_chirho!(NodeChirho::ExpressionChirho(expr_chirho));
                }
                // These payloads are syntax data, never executable pattern input
                // to this pass. This is not a TH evaluator or runtime proof.
                ExprChirho::QuoteExprChirho { .. }
                | ExprChirho::QuoteDeclChirho { .. }
                | ExprChirho::QuoteTypeChirho { .. }
                | ExprChirho::QuotePatChirho { .. }
                | ExprChirho::VarChirho(_)
                | ExprChirho::ConChirho(_)
                | ExprChirho::LitChirho(_) => {}
            },
        }
    }
    Ok(expanded_rows_chirho)
}
