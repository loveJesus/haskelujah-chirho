// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Binding groups: the references each binding makes, and their strongly connected components.
//! Moved from `infer_chirho.rs` in row 661.

use super::canonical_value_name_text_chirho;
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::StmtChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::stmt_operation_chirho::SelectedOperationChirho;
use std::collections::HashSet;

/// Pattern binders are not references. Retained view/splice expressions are.
fn collect_pattern_refs_chirho(pattern_chirho: &PatChirho, refs_chirho: &mut HashSet<String>) {
    match pattern_chirho {
        PatChirho::SpliceChirho {
            expr_chirho: Some(expr_chirho),
            ..
        } => collect_expr_refs_chirho(expr_chirho, refs_chirho),
        PatChirho::ConChirho { args_chirho, .. }
        | PatChirho::TupleChirho {
            elements_chirho: args_chirho,
            ..
        }
        | PatChirho::ListChirho {
            elements_chirho: args_chirho,
            ..
        } => {
            for pattern_chirho in args_chirho {
                collect_pattern_refs_chirho(pattern_chirho, refs_chirho);
            }
        }
        PatChirho::AsChirho { pattern_chirho, .. }
        | PatChirho::ParenChirho {
            inner_chirho: pattern_chirho,
            ..
        }
        | PatChirho::LazyChirho {
            inner_chirho: pattern_chirho,
            ..
        }
        | PatChirho::BangChirho {
            inner_chirho: pattern_chirho,
            ..
        }
        | PatChirho::TypeAnnotChirho {
            pat_chirho: pattern_chirho,
            ..
        } => collect_pattern_refs_chirho(pattern_chirho, refs_chirho),
        PatChirho::ViewChirho {
            expr_chirho,
            pat_chirho,
            ..
        } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho);
            collect_pattern_refs_chirho(pat_chirho, refs_chirho);
        }
        PatChirho::RecordChirho { fields_chirho, .. } => {
            for field_chirho in fields_chirho {
                collect_pattern_refs_chirho(&field_chirho.pattern_chirho, refs_chirho);
            }
        }
        PatChirho::InfixConChirho {
            left_chirho,
            right_chirho,
            ..
        } => {
            collect_pattern_refs_chirho(left_chirho, refs_chirho);
            collect_pattern_refs_chirho(right_chirho, refs_chirho);
        }
        PatChirho::VarChirho(_)
        | PatChirho::LitChirho(_)
        | PatChirho::WildcardChirho(_)
        | PatChirho::NegChirho { .. }
        | PatChirho::SpliceChirho {
            expr_chirho: None, ..
        } => {}
    }
}

/// Collect variable references from an expression (shallow walk).
fn collect_expr_refs_chirho(
    expr_chirho: &haskelujah_ast_chirho::expr_chirho::ExprChirho,
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    use haskelujah_ast_chirho::expr_chirho::ExprChirho;
    match expr_chirho {
        ExprChirho::VarChirho(name_chirho) => {
            refs_chirho.insert(canonical_value_name_text_chirho(name_chirho.text_chirho()));
        }
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            collect_expr_refs_chirho(fun_chirho, refs_chirho);
            collect_expr_refs_chirho(arg_chirho, refs_chirho);
        }
        ExprChirho::LamChirho {
            pats_chirho,
            body_chirho,
            ..
        } => {
            for pattern_chirho in pats_chirho {
                collect_pattern_refs_chirho(pattern_chirho, refs_chirho);
            }
            collect_expr_refs_chirho(body_chirho, refs_chirho);
        }
        ExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for bind_chirho in binds_chirho {
                collect_local_bind_refs_chirho(bind_chirho, refs_chirho);
            }
            collect_expr_refs_chirho(body_chirho, refs_chirho);
        }
        ExprChirho::IfChirho {
            cond_chirho,
            then_chirho,
            else_chirho,
            ..
        } => {
            collect_expr_refs_chirho(cond_chirho, refs_chirho);
            collect_expr_refs_chirho(then_chirho, refs_chirho);
            collect_expr_refs_chirho(else_chirho, refs_chirho);
        }
        ExprChirho::MultiWayIfChirho { arms_chirho, .. } => {
            collect_guarded_arm_refs_chirho(arms_chirho, refs_chirho);
        }
        ExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            collect_expr_refs_chirho(scrutinee_chirho, refs_chirho);
            for alt_chirho in alts_chirho {
                collect_pattern_refs_chirho(&alt_chirho.pat_chirho, refs_chirho);
                collect_rhs_refs_chirho(&alt_chirho.rhs_chirho, refs_chirho);
                for wb_chirho in &alt_chirho.where_binds_chirho {
                    collect_local_bind_refs_chirho(wb_chirho, refs_chirho);
                }
            }
        }
        ExprChirho::DoChirho { stmts_chirho, .. } => {
            for stmt_chirho in stmts_chirho {
                collect_stmt_refs_chirho(stmt_chirho, refs_chirho);
            }
        }
        ExprChirho::InfixChirho {
            left_chirho,
            op_chirho,
            right_chirho,
            ..
        } => {
            collect_expr_refs_chirho(left_chirho, refs_chirho);
            refs_chirho.insert(op_chirho.text_chirho().to_string());
            collect_expr_refs_chirho(right_chirho, refs_chirho);
        }
        ExprChirho::NegChirho { expr_chirho, .. } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho)
        }
        ExprChirho::TupleChirho {
            elements_chirho, ..
        } => {
            for e_chirho in elements_chirho {
                collect_expr_refs_chirho(e_chirho, refs_chirho);
            }
        }
        ExprChirho::ListChirho {
            elements_chirho, ..
        } => {
            for e_chirho in elements_chirho {
                collect_expr_refs_chirho(e_chirho, refs_chirho);
            }
        }
        ExprChirho::TypeAppChirho { expr_chirho, .. } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho);
        }
        ExprChirho::ArithSeqChirho {
            from_chirho,
            then_chirho,
            to_chirho,
            ..
        } => {
            collect_expr_refs_chirho(from_chirho, refs_chirho);
            if let Some(t_chirho) = then_chirho {
                collect_expr_refs_chirho(t_chirho, refs_chirho);
            }
            if let Some(t_chirho) = to_chirho {
                collect_expr_refs_chirho(t_chirho, refs_chirho);
            }
        }
        ExprChirho::ListCompChirho {
            body_chirho,
            quals_chirho,
            parallel_quals_chirho,
            ..
        } => {
            collect_expr_refs_chirho(body_chirho, refs_chirho);
            for q_chirho in quals_chirho
                .iter()
                .chain(parallel_quals_chirho.iter().flatten())
            {
                match q_chirho {
                    haskelujah_ast_chirho::expr_chirho::ComprehensionQualifierChirho::StatementChirho(statement_chirho) => collect_stmt_refs_chirho(statement_chirho, refs_chirho),
                    haskelujah_ast_chirho::expr_chirho::ComprehensionQualifierChirho::TransformChirho(transform_chirho) => {
                        if let Some(using_chirho) = &transform_chirho.using_chirho { collect_expr_refs_chirho(using_chirho, refs_chirho); }
                        if let Some(by_chirho) = &transform_chirho.by_chirho { collect_expr_refs_chirho(by_chirho, refs_chirho); }
                    }
                }
            }
        }
        ExprChirho::LeftSectionChirho {
            op_chirho,
            arg_chirho,
            ..
        } => {
            refs_chirho.insert(op_chirho.text_chirho().to_string());
            collect_expr_refs_chirho(arg_chirho, refs_chirho);
        }
        ExprChirho::RightSectionChirho {
            arg_chirho,
            op_chirho,
            ..
        } => {
            collect_expr_refs_chirho(arg_chirho, refs_chirho);
            refs_chirho.insert(op_chirho.text_chirho().to_string());
        }
        ExprChirho::AnnChirho { expr_chirho, .. }
        | ExprChirho::ParenChirho {
            inner_chirho: expr_chirho,
            ..
        }
        | ExprChirho::SpliceChirho { expr_chirho, .. }
        | ExprChirho::TypedSpliceChirho { expr_chirho, .. }
        | ExprChirho::QuoteExprChirho { expr_chirho, .. } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho);
        }
        ExprChirho::RecordConChirho { fields_chirho, .. } => {
            for f_chirho in fields_chirho {
                collect_expr_refs_chirho(&f_chirho.value_chirho, refs_chirho);
            }
        }
        ExprChirho::RecordUpdateChirho {
            expr_chirho,
            fields_chirho,
            ..
        } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho);
            for f_chirho in fields_chirho {
                collect_expr_refs_chirho(&f_chirho.value_chirho, refs_chirho);
            }
        }
        ExprChirho::QuotePatChirho { pat_chirho, .. } => {
            collect_pattern_refs_chirho(pat_chirho, refs_chirho)
        }
        // Other syntax payloads keep their existing dependency policy.
        _ => {}
    }
}

fn collect_local_bind_refs_chirho(
    bind_chirho: &haskelujah_ast_chirho::expr_chirho::LocalBindChirho,
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    match bind_chirho {
        haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
            matches_chirho,
            ..
        } => {
            for arm_chirho in matches_chirho {
                for pattern_chirho in &arm_chirho.pats_chirho {
                    collect_pattern_refs_chirho(pattern_chirho, refs_chirho);
                }
                collect_rhs_refs_chirho(&arm_chirho.rhs_chirho, refs_chirho);
            }
        }
        haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
            pat_chirho,
            rhs_chirho,
            ..
        } => {
            collect_pattern_refs_chirho(pat_chirho, refs_chirho);
            collect_rhs_refs_chirho(rhs_chirho, refs_chirho);
        }
        _ => {}
    }
}

fn collect_rhs_refs_chirho(
    rhs_chirho: &haskelujah_ast_chirho::expr_chirho::RhsChirho,
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    use haskelujah_ast_chirho::expr_chirho::RhsChirho;
    match rhs_chirho {
        RhsChirho::UnguardedChirho(expr_chirho) => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho)
        }
        RhsChirho::GuardedChirho(arms_chirho) => {
            collect_guarded_arm_refs_chirho(arms_chirho, refs_chirho)
        }
    }
}

/// Guarded arms, a right-hand side's or a multi-way if's.
fn collect_guarded_arm_refs_chirho(
    arms_chirho: &[haskelujah_ast_chirho::expr_chirho::GuardedExprChirho],
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    for ge_chirho in arms_chirho {
        for qual_chirho in &ge_chirho.quals_chirho {
            collect_stmt_refs_chirho(qual_chirho, refs_chirho);
        }
        collect_expr_refs_chirho(&ge_chirho.body_chirho, refs_chirho);
    }
}

/// A do statement, a comprehension qualifier, or a guard qualifier. The operations a do statement
/// SELECTED are references like any other, so a local or self-qualified `>>=` is generalized
/// before the binding whose block uses it (row 710).
/// workflow: language-features-chirho/qualified-do-chirho
fn collect_stmt_refs_chirho(
    stmt_chirho: &StmtChirho,
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    match stmt_chirho {
        StmtChirho::BindChirho {
            pat_chirho,
            expr_chirho,
            bind_chirho,
            fail_chirho,
            ..
        } => {
            collect_pattern_refs_chirho(pat_chirho, refs_chirho);
            collect_expr_refs_chirho(expr_chirho, refs_chirho);
            collect_selected_refs_chirho(&[bind_chirho, fail_chirho], refs_chirho);
        }
        StmtChirho::ExprChirho {
            expr_chirho,
            then_chirho,
        } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho);
            collect_selected_refs_chirho(&[then_chirho], refs_chirho);
        }
        StmtChirho::LetChirho { binds_chirho, .. } => {
            for b_chirho in binds_chirho {
                collect_local_bind_refs_chirho(b_chirho, refs_chirho);
            }
        }
    }
}

/// The selected operations a statement carries, keyed as a variable reference is.
fn collect_selected_refs_chirho(
    selected_chirho: &[&Option<SelectedOperationChirho>],
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    for operation_chirho in selected_chirho.iter().copied().flatten() {
        refs_chirho.insert(canonical_value_name_text_chirho(
            operation_chirho.name_chirho.text_chirho(),
        ));
    }
}

pub(super) fn collect_module_shadowed_names_chirho(
    module_chirho: &ModuleChirho,
) -> std::collections::HashSet<String> {
    let mut names_chirho = std::collections::HashSet::new();
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. }
            | DeclChirho::TypeSigChirho { name_chirho, .. } => {
                names_chirho.insert(canonical_value_name_text_chirho(name_chirho.text_chirho()));
            }
            DeclChirho::PatBindChirho { pat_chirho, .. } => {
                for name_chirho in crate::linearity_chirho::pat_bound_names_chirho(pat_chirho) {
                    names_chirho.insert(name_chirho);
                }
            }
            DeclChirho::DataDeclChirho {
                constructors_chirho,
                ..
            } => {
                for constructor_chirho in constructors_chirho {
                    if let Some(fields_chirho) = constructor_chirho.record_fields_chirho() {
                        for field_chirho in fields_chirho {
                            names_chirho.extend(
                                field_chirho
                                    .names_chirho
                                    .iter()
                                    .map(|name_chirho| name_chirho.text_chirho().to_string()),
                            );
                        }
                    }
                    match constructor_chirho {
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                            name_chirho,
                            ..
                        }
                        | haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                            name_chirho,
                            ..
                        } => {
                            names_chirho.insert(name_chirho.text_chirho().to_string());
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                            name_chirho,
                            fields_chirho,
                            ..
                        } => {
                            names_chirho.insert(name_chirho.text_chirho().to_string());
                            for field_decl_chirho in fields_chirho {
                                for field_name_chirho in &field_decl_chirho.names_chirho {
                                    names_chirho
                                        .insert(field_name_chirho.text_chirho().to_string());
                                }
                            }
                        }
                    }
                }
            }
            DeclChirho::NewtypeDeclChirho {
                constructor_chirho, ..
            } => {
                if let Some(fields_chirho) = constructor_chirho.record_fields_chirho() {
                    for field_chirho in fields_chirho {
                        names_chirho.extend(
                            field_chirho
                                .names_chirho
                                .iter()
                                .map(|name_chirho| name_chirho.text_chirho().to_string()),
                        );
                    }
                }
                match constructor_chirho.as_ref() {
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                        name_chirho,
                        ..
                    }
                    | haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                        name_chirho,
                        ..
                    } => {
                        names_chirho.insert(name_chirho.text_chirho().to_string());
                    }
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                        name_chirho,
                        fields_chirho,
                        ..
                    } => {
                        names_chirho.insert(name_chirho.text_chirho().to_string());
                        for field_decl_chirho in fields_chirho {
                            for field_name_chirho in &field_decl_chirho.names_chirho {
                                names_chirho.insert(field_name_chirho.text_chirho().to_string());
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    names_chirho
}

/// Compute binding groups for top-level functions using SCC analysis.
/// Returns groups in topological order (leaf dependencies first).
/// Strongly connected binding groups in dependency order. A reference to a
/// binding that has a type signature is not a dependency edge (Haskell 2010
/// §4.5.1): its uses see the signature, so `f` with a signature and an
/// unsigned `g` that call each other are inferred separately and `g` stays
/// polymorphic.
pub(super) fn binding_groups_chirho(
    fun_names_chirho: &[String],
    fun_decls_chirho: &[(usize, &[haskelujah_ast_chirho::expr_chirho::MatchArmChirho])],
    signed_names_chirho: &HashSet<String>,
) -> Vec<Vec<usize>> {
    let name_set_chirho: std::collections::HashSet<&str> =
        fun_names_chirho.iter().map(|s| s.as_str()).collect();
    let name_to_idx_chirho: std::collections::HashMap<&str, usize> = fun_names_chirho
        .iter()
        .enumerate()
        .map(|(i, s)| (s.as_str(), i))
        .collect();

    // Build adjacency list
    let n_chirho = fun_names_chirho.len();
    let mut adj_chirho: Vec<Vec<usize>> = vec![Vec::new(); n_chirho];
    for (i_chirho, (_, matches_chirho)) in fun_decls_chirho.iter().enumerate() {
        let mut refs_chirho = std::collections::HashSet::new();
        for arm_chirho in *matches_chirho {
            for pattern_chirho in &arm_chirho.pats_chirho {
                collect_pattern_refs_chirho(pattern_chirho, &mut refs_chirho);
            }
            collect_rhs_refs_chirho(&arm_chirho.rhs_chirho, &mut refs_chirho);
            for wb_chirho in &arm_chirho.where_binds_chirho {
                collect_local_bind_refs_chirho(wb_chirho, &mut refs_chirho);
            }
        }
        // DETERMINISM: `refs_chirho` is a HashSet, so iterating it put the dependency
        // edges into `adj_chirho` in per-process hash order. That order decides the
        // binding-group traversal, which decides the order declarations are inferred,
        // which decides type-variable ALLOCATION — so a hash seed reached the inferred
        // types themselves. This is upstream of the defaulting loops and is why sorting
        // those alone did not make compilation reproducible.
        // See spec-chirho/bug-nondeterministic-typecheck-chirho.md
        let mut sorted_refs_chirho: Vec<_> = refs_chirho.iter().collect();
        sorted_refs_chirho.sort_unstable();
        for ref_name_chirho in sorted_refs_chirho {
            if name_set_chirho.contains(ref_name_chirho.as_str())
                && ref_name_chirho != &fun_names_chirho[i_chirho]
                && !signed_names_chirho.contains(ref_name_chirho.as_str())
                && let Some(&j_chirho) = name_to_idx_chirho.get(ref_name_chirho.as_str())
            {
                adj_chirho[i_chirho].push(j_chirho);
            }
        }
    }

    dependency_groups_chirho(&adj_chirho)
}

/// Shared dependency graph grouping; callers retain their own reference and
/// binder admission rules. The kind elaborator preflights depth/work first.
pub(crate) fn dependency_groups_chirho(adj_chirho: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n_chirho = adj_chirho.len();
    // Tarjan's SCC
    let mut tarjan_chirho = TarjanStateChirho {
        index_chirho: 0,
        stack_chirho: Vec::new(),
        on_stack_chirho: vec![false; n_chirho],
        indices_chirho: vec![None; n_chirho],
        lowlinks_chirho: vec![0; n_chirho],
        sccs_chirho: Vec::new(),
    };

    /// Tarjan's search state, threaded through `strongconnect_chirho`'s recursion.
    struct TarjanStateChirho {
        index_chirho: usize,
        stack_chirho: Vec<usize>,
        on_stack_chirho: Vec<bool>,
        indices_chirho: Vec<Option<usize>>,
        lowlinks_chirho: Vec<usize>,
        sccs_chirho: Vec<Vec<usize>>,
    }

    impl TarjanStateChirho {
        fn strongconnect_chirho(&mut self, v_chirho: usize, adj_chirho: &[Vec<usize>]) {
            self.indices_chirho[v_chirho] = Some(self.index_chirho);
            self.lowlinks_chirho[v_chirho] = self.index_chirho;
            self.index_chirho += 1;
            self.stack_chirho.push(v_chirho);
            self.on_stack_chirho[v_chirho] = true;

            for &w_chirho in &adj_chirho[v_chirho] {
                if self.indices_chirho[w_chirho].is_none() {
                    self.strongconnect_chirho(w_chirho, adj_chirho);
                    self.lowlinks_chirho[v_chirho] =
                        self.lowlinks_chirho[v_chirho].min(self.lowlinks_chirho[w_chirho]);
                } else if self.on_stack_chirho[w_chirho] {
                    self.lowlinks_chirho[v_chirho] =
                        self.lowlinks_chirho[v_chirho].min(self.indices_chirho[w_chirho].unwrap());
                }
            }

            if self.lowlinks_chirho[v_chirho] == self.indices_chirho[v_chirho].unwrap() {
                let mut scc_chirho = Vec::new();
                loop {
                    let w_chirho = self.stack_chirho.pop().unwrap();
                    self.on_stack_chirho[w_chirho] = false;
                    scc_chirho.push(w_chirho);
                    if w_chirho == v_chirho {
                        break;
                    }
                }
                self.sccs_chirho.push(scc_chirho);
            }
        }
    }

    for v_chirho in 0..n_chirho {
        if tarjan_chirho.indices_chirho[v_chirho].is_none() {
            tarjan_chirho.strongconnect_chirho(v_chirho, adj_chirho);
        }
    }

    // Tarjan naturally produces SCCs with dependencies first (leaves before
    // dependents), which is the order we need for type inference.
    tarjan_chirho.sccs_chirho
}
