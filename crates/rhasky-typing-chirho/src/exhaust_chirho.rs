// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Pattern match exhaustiveness and redundancy checking
//!
//! Walks the AST after type inference and checks every match site (case
//! expressions and multi-equation function bindings) for:
//!
//! - **Non-exhaustive patterns** (error E0400): a data type is matched but
//!   not all constructors are covered and there is no wildcard/default.
//! - **Redundant patterns** (warning W0400): a pattern that can never be
//!   reached because an earlier pattern already covers all its values.
//!
//! The algorithm operates on a single column of top-level patterns per
//! match site. Nested pattern decomposition (the full Maranget matrix
//! algorithm) is a future enhancement.

use std::collections::{HashMap, HashSet};

use rhasky_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use rhasky_ast_chirho::expr_chirho::{
    AltChirho, ExprChirho, LocalBindChirho, MatchArmChirho, RhsChirho, StmtChirho,
};
use rhasky_ast_chirho::module_chirho::ModuleChirho;
use rhasky_ast_chirho::pat_chirho::PatChirho;
use rhasky_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use rhasky_span_chirho::SpanChirho;

// ---------------------------------------------------------------------------
// Error codes
// ---------------------------------------------------------------------------

/// Non-exhaustive pattern match.
const NON_EXHAUSTIVE_CODE_CHIRHO: u16 = 400;

/// Redundant / unreachable pattern.
const REDUNDANT_CODE_CHIRHO: u16 = 401;

// ---------------------------------------------------------------------------
// Constructor environment
// ---------------------------------------------------------------------------

/// Information about a single data constructor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConInfoChirho {
    /// Constructor name (e.g. `"Just"`, `"Nothing"`).
    pub name_chirho: String,
    /// Number of fields.
    pub arity_chirho: usize,
}

/// Maps a type name to its set of data constructors, and maps each
/// constructor name back to its parent type.
#[derive(Debug, Clone, Default)]
pub struct TypeConEnvChirho {
    /// type name → ordered list of constructors
    type_to_cons_chirho: HashMap<String, Vec<ConInfoChirho>>,
    /// constructor name → type name
    con_to_type_chirho: HashMap<String, String>,
}

impl TypeConEnvChirho {
    /// Build the environment from a module's declarations.
    pub fn from_module_chirho(module_chirho: &ModuleChirho) -> Self {
        let mut env_chirho = Self::default();

        // Seed Bool — it is a built-in that may not have a data decl in user code.
        env_chirho.register_type_chirho(
            "Bool".to_string(),
            vec![
                ConInfoChirho {
                    name_chirho: "False".to_string(),
                    arity_chirho: 0,
                },
                ConInfoChirho {
                    name_chirho: "True".to_string(),
                    arity_chirho: 0,
                },
            ],
        );

        // Seed Maybe
        env_chirho.register_type_chirho(
            "Maybe".to_string(),
            vec![
                ConInfoChirho {
                    name_chirho: "Nothing".to_string(),
                    arity_chirho: 0,
                },
                ConInfoChirho {
                    name_chirho: "Just".to_string(),
                    arity_chirho: 1,
                },
            ],
        );

        // Seed Either
        env_chirho.register_type_chirho(
            "Either".to_string(),
            vec![
                ConInfoChirho {
                    name_chirho: "Left".to_string(),
                    arity_chirho: 1,
                },
                ConInfoChirho {
                    name_chirho: "Right".to_string(),
                    arity_chirho: 1,
                },
            ],
        );

        // Seed Ordering
        env_chirho.register_type_chirho(
            "Ordering".to_string(),
            vec![
                ConInfoChirho {
                    name_chirho: "LT".to_string(),
                    arity_chirho: 0,
                },
                ConInfoChirho {
                    name_chirho: "EQ".to_string(),
                    arity_chirho: 0,
                },
                ConInfoChirho {
                    name_chirho: "GT".to_string(),
                    arity_chirho: 0,
                },
            ],
        );

        // Seed list constructors
        env_chirho.register_type_chirho(
            "[]".to_string(),
            vec![
                ConInfoChirho {
                    name_chirho: "[]".to_string(),
                    arity_chirho: 0,
                },
                ConInfoChirho {
                    name_chirho: ":".to_string(),
                    arity_chirho: 2,
                },
            ],
        );

        // Seed Map constructors (BST representation)
        env_chirho.register_type_chirho(
            "Map".to_string(),
            vec![
                ConInfoChirho {
                    name_chirho: "MapEmpty".to_string(),
                    arity_chirho: 0,
                },
                ConInfoChirho {
                    name_chirho: "MapNode".to_string(),
                    arity_chirho: 4, // key, value, left, right
                },
            ],
        );

        // Seed Set constructors (BST representation)
        env_chirho.register_type_chirho(
            "Set".to_string(),
            vec![
                ConInfoChirho {
                    name_chirho: "SetEmpty".to_string(),
                    arity_chirho: 0,
                },
                ConInfoChirho {
                    name_chirho: "SetNode".to_string(),
                    arity_chirho: 3, // element, left, right
                },
            ],
        );

        for decl_chirho in &module_chirho.decls_chirho {
            match decl_chirho {
                DeclChirho::DataDeclChirho {
                    name_chirho,
                    constructors_chirho,
                    ..
                } => {
                    let cons_chirho = constructors_chirho
                        .iter()
                        .map(|c_chirho| con_decl_to_info_chirho(c_chirho))
                        .collect();
                    env_chirho.register_type_chirho(
                        name_chirho.text_chirho().to_string(),
                        cons_chirho,
                    );
                }
                DeclChirho::NewtypeDeclChirho {
                    name_chirho,
                    constructor_chirho,
                    ..
                } => {
                    env_chirho.register_type_chirho(
                        name_chirho.text_chirho().to_string(),
                        vec![con_decl_to_info_chirho(constructor_chirho)],
                    );
                }
                _ => {}
            }
        }

        env_chirho
    }

    fn register_type_chirho(
        &mut self,
        type_name_chirho: String,
        cons_chirho: Vec<ConInfoChirho>,
    ) {
        for c_chirho in &cons_chirho {
            self.con_to_type_chirho
                .insert(c_chirho.name_chirho.clone(), type_name_chirho.clone());
        }
        self.type_to_cons_chirho
            .insert(type_name_chirho, cons_chirho);
    }

    /// Get the constructors for a type name.
    pub fn constructors_of_chirho(&self, type_name_chirho: &str) -> Option<&[ConInfoChirho]> {
        self.type_to_cons_chirho
            .get(type_name_chirho)
            .map(|v_chirho| v_chirho.as_slice())
    }

    /// Get the parent type name for a constructor.
    pub fn type_of_con_chirho(&self, con_name_chirho: &str) -> Option<&str> {
        self.con_to_type_chirho
            .get(con_name_chirho)
            .map(|s_chirho| s_chirho.as_str())
    }

    /// Get all constructor names for the type that owns `con_name_chirho`.
    pub fn siblings_of_chirho(&self, con_name_chirho: &str) -> Option<&[ConInfoChirho]> {
        self.type_of_con_chirho(con_name_chirho)
            .and_then(|ty_chirho| self.constructors_of_chirho(ty_chirho))
    }
}

fn con_decl_to_info_chirho(decl_chirho: &ConDeclChirho) -> ConInfoChirho {
    match decl_chirho {
        ConDeclChirho::OrdinaryChirho {
            name_chirho,
            fields_chirho,
            ..
        } => ConInfoChirho {
            name_chirho: name_chirho.text_chirho().to_string(),
            arity_chirho: fields_chirho.len(),
        },
        ConDeclChirho::RecordChirho {
            name_chirho,
            fields_chirho,
            ..
        } => ConInfoChirho {
            name_chirho: name_chirho.text_chirho().to_string(),
            arity_chirho: fields_chirho
                .iter()
                .map(|f_chirho| f_chirho.names_chirho.len())
                .sum(),
        },
    }
}

// ---------------------------------------------------------------------------
// Exhaustiveness checker
// ---------------------------------------------------------------------------

/// Result of checking a module for pattern exhaustiveness and redundancy.
#[derive(Debug)]
pub struct ExhaustResultChirho {
    pub diagnostics_chirho: DiagnosticBundleChirho,
}

/// Check a module for non-exhaustive and redundant patterns.
pub fn check_module_exhaustiveness_chirho(
    module_chirho: &ModuleChirho,
) -> ExhaustResultChirho {
    let env_chirho = TypeConEnvChirho::from_module_chirho(module_chirho);
    let mut diags_chirho = DiagnosticBundleChirho::empty_chirho();
    let mut checker_chirho = ExhaustCheckerChirho {
        env_chirho: &env_chirho,
        diags_chirho: &mut diags_chirho,
    };

    // Check top-level declarations
    for decl_chirho in &module_chirho.decls_chirho {
        checker_chirho.check_decl_chirho(decl_chirho);
    }

    ExhaustResultChirho {
        diagnostics_chirho: diags_chirho,
    }
}

struct ExhaustCheckerChirho<'a> {
    env_chirho: &'a TypeConEnvChirho,
    diags_chirho: &'a mut DiagnosticBundleChirho,
}

impl<'a> ExhaustCheckerChirho<'a> {
    // -------------------------------------------------------------------
    // Declaration walking
    // -------------------------------------------------------------------

    fn check_decl_chirho(&mut self, decl_chirho: &DeclChirho) {
        match decl_chirho {
            DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                span_chirho,
            } => {
                self.check_fun_bind_chirho(
                    name_chirho.text_chirho(),
                    matches_chirho,
                    *span_chirho,
                );
                // Recurse into match RHSs
                for arm_chirho in matches_chirho {
                    self.check_rhs_chirho(&arm_chirho.rhs_chirho);
                    self.check_local_binds_chirho(&arm_chirho.where_binds_chirho);
                }
            }
            DeclChirho::PatBindChirho { rhs_chirho, .. } => {
                self.check_rhs_chirho(rhs_chirho);
            }
            _ => {}
        }
    }

    fn check_local_binds_chirho(&mut self, binds_chirho: &[LocalBindChirho]) {
        for bind_chirho in binds_chirho {
            match bind_chirho {
                LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                } => {
                    self.check_fun_bind_chirho(
                        name_chirho.text_chirho(),
                        matches_chirho,
                        *span_chirho,
                    );
                    for arm_chirho in matches_chirho {
                        self.check_rhs_chirho(&arm_chirho.rhs_chirho);
                        self.check_local_binds_chirho(&arm_chirho.where_binds_chirho);
                    }
                }
                LocalBindChirho::PatBindChirho { rhs_chirho, .. } => {
                    self.check_rhs_chirho(rhs_chirho);
                }
                LocalBindChirho::TypeSigChirho { .. } => {}
            }
        }
    }

    fn check_rhs_chirho(&mut self, rhs_chirho: &RhsChirho) {
        match rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => self.check_expr_chirho(expr_chirho),
            RhsChirho::GuardedChirho(guards_chirho) => {
                for g_chirho in guards_chirho {
                    self.check_expr_chirho(&g_chirho.guard_chirho);
                    self.check_expr_chirho(&g_chirho.body_chirho);
                }
            }
        }
    }

    // -------------------------------------------------------------------
    // Expression walking — find nested case exprs
    // -------------------------------------------------------------------

    fn check_expr_chirho(&mut self, expr_chirho: &ExprChirho) {
        match expr_chirho {
            ExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                span_chirho,
            } => {
                self.check_expr_chirho(scrutinee_chirho);
                self.check_case_alts_chirho(alts_chirho, *span_chirho);
                for alt_chirho in alts_chirho {
                    self.check_rhs_chirho(&alt_chirho.rhs_chirho);
                    self.check_local_binds_chirho(&alt_chirho.where_binds_chirho);
                }
            }
            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                self.check_expr_chirho(fun_chirho);
                self.check_expr_chirho(arg_chirho);
            }
            ExprChirho::InfixChirho {
                left_chirho,
                right_chirho,
                ..
            } => {
                self.check_expr_chirho(left_chirho);
                self.check_expr_chirho(right_chirho);
            }
            ExprChirho::NegChirho { expr_chirho, .. } => {
                self.check_expr_chirho(expr_chirho);
            }
            ExprChirho::LamChirho { body_chirho, .. } => {
                self.check_expr_chirho(body_chirho);
            }
            ExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                self.check_local_binds_chirho(binds_chirho);
                self.check_expr_chirho(body_chirho);
            }
            ExprChirho::IfChirho {
                cond_chirho,
                then_chirho,
                else_chirho,
                ..
            } => {
                self.check_expr_chirho(cond_chirho);
                self.check_expr_chirho(then_chirho);
                self.check_expr_chirho(else_chirho);
            }
            ExprChirho::DoChirho { stmts_chirho, .. } => {
                for stmt_chirho in stmts_chirho {
                    self.check_stmt_chirho(stmt_chirho);
                }
            }
            ExprChirho::TupleChirho {
                elements_chirho, ..
            }
            | ExprChirho::ListChirho {
                elements_chirho, ..
            } => {
                for e_chirho in elements_chirho {
                    self.check_expr_chirho(e_chirho);
                }
            }
            ExprChirho::ArithSeqChirho {
                from_chirho,
                then_chirho,
                to_chirho,
                ..
            } => {
                self.check_expr_chirho(from_chirho);
                if let Some(t_chirho) = then_chirho {
                    self.check_expr_chirho(t_chirho);
                }
                if let Some(t_chirho) = to_chirho {
                    self.check_expr_chirho(t_chirho);
                }
            }
            ExprChirho::ListCompChirho {
                body_chirho,
                quals_chirho,
                ..
            } => {
                self.check_expr_chirho(body_chirho);
                for q_chirho in quals_chirho {
                    self.check_stmt_chirho(q_chirho);
                }
            }
            ExprChirho::LeftSectionChirho { arg_chirho, .. }
            | ExprChirho::RightSectionChirho { arg_chirho, .. } => {
                self.check_expr_chirho(arg_chirho);
            }
            ExprChirho::AnnChirho { expr_chirho, .. }
            | ExprChirho::ParenChirho {
                inner_chirho: expr_chirho,
                ..
            } => {
                self.check_expr_chirho(expr_chirho);
            }
            ExprChirho::RecordConChirho { fields_chirho, .. } => {
                for f_chirho in fields_chirho {
                    self.check_expr_chirho(&f_chirho.value_chirho);
                }
            }
            ExprChirho::RecordUpdateChirho {
                expr_chirho,
                fields_chirho,
                ..
            } => {
                self.check_expr_chirho(expr_chirho);
                for f_chirho in fields_chirho {
                    self.check_expr_chirho(&f_chirho.value_chirho);
                }
            }
            // Leaves — no sub-expressions to check
            ExprChirho::VarChirho(_)
            | ExprChirho::ConChirho(_)
            | ExprChirho::LitChirho(_) => {}
        }
    }

    fn check_stmt_chirho(&mut self, stmt_chirho: &StmtChirho) {
        match stmt_chirho {
            StmtChirho::ExprChirho(e_chirho) => self.check_expr_chirho(e_chirho),
            StmtChirho::BindChirho { expr_chirho, .. } => self.check_expr_chirho(expr_chirho),
            StmtChirho::LetChirho { binds_chirho, .. } => {
                self.check_local_binds_chirho(binds_chirho);
            }
        }
    }

    // -------------------------------------------------------------------
    // Exhaustiveness logic
    // -------------------------------------------------------------------

    /// Check a case expression's alternatives for exhaustiveness and
    /// redundancy. Each alt has a single pattern.
    fn check_case_alts_chirho(
        &mut self,
        alts_chirho: &[AltChirho],
        case_span_chirho: SpanChirho,
    ) {
        let pats_chirho: Vec<&PatChirho> =
            alts_chirho.iter().map(|a_chirho| &a_chirho.pat_chirho).collect();
        let spans_chirho: Vec<SpanChirho> =
            alts_chirho.iter().map(|a_chirho| a_chirho.span_chirho).collect();
        self.check_pattern_column_chirho(&pats_chirho, &spans_chirho, case_span_chirho, "case");
    }

    /// Check a multi-equation function binding. Checks the first pattern
    /// column (the first argument's patterns across all equations).
    fn check_fun_bind_chirho(
        &mut self,
        fn_name_chirho: &str,
        matches_chirho: &[MatchArmChirho],
        fn_span_chirho: SpanChirho,
    ) {
        if matches_chirho.is_empty() {
            return;
        }

        let arity_chirho = matches_chirho[0].pats_chirho.len();
        if arity_chirho == 0 {
            return; // nullary function — nothing to check
        }

        // Check each parameter column independently
        for col_chirho in 0..arity_chirho {
            let pats_chirho: Vec<&PatChirho> = matches_chirho
                .iter()
                .filter_map(|arm_chirho| arm_chirho.pats_chirho.get(col_chirho))
                .collect();
            let spans_chirho: Vec<SpanChirho> = matches_chirho
                .iter()
                .map(|arm_chirho| arm_chirho.span_chirho)
                .collect();
            self.check_pattern_column_chirho(
                &pats_chirho,
                &spans_chirho,
                fn_span_chirho,
                fn_name_chirho,
            );
        }
    }

    /// Core logic: given a column of patterns (one per alternative or
    /// equation), check for exhaustiveness and redundancy.
    fn check_pattern_column_chirho(
        &mut self,
        pats_chirho: &[&PatChirho],
        spans_chirho: &[SpanChirho],
        site_span_chirho: SpanChirho,
        site_name_chirho: &str,
    ) {
        if pats_chirho.is_empty() {
            return;
        }

        // Classify the patterns in this column.
        let mut has_wildcard_chirho = false;
        let mut wildcard_index_chirho: Option<usize> = None;
        let mut seen_cons_chirho: Vec<(String, usize)> = Vec::new(); // (name, index)
        let mut seen_lits_chirho: Vec<usize> = Vec::new();
        let mut redundant_indices_chirho: Vec<usize> = Vec::new();

        for (idx_chirho, pat_chirho) in pats_chirho.iter().enumerate() {
            match classify_pattern_chirho(pat_chirho) {
                PatClassChirho::WildcardChirho => {
                    if has_wildcard_chirho {
                        // Second wildcard is always redundant
                        redundant_indices_chirho.push(idx_chirho);
                    } else if all_cons_covered_chirho(
                        &seen_cons_chirho,
                        self.env_chirho,
                    ) {
                        // Wildcard after all constructors covered is redundant
                        redundant_indices_chirho.push(idx_chirho);
                    } else {
                        has_wildcard_chirho = true;
                        wildcard_index_chirho = Some(idx_chirho);
                    }
                }
                PatClassChirho::ConChirho(name_chirho) => {
                    if has_wildcard_chirho {
                        // Any pattern after a wildcard is redundant
                        redundant_indices_chirho.push(idx_chirho);
                    } else if seen_cons_chirho
                        .iter()
                        .any(|(n_chirho, _)| *n_chirho == name_chirho)
                    {
                        // Duplicate constructor
                        redundant_indices_chirho.push(idx_chirho);
                    } else {
                        seen_cons_chirho.push((name_chirho, idx_chirho));
                    }
                }
                PatClassChirho::LitChirho => {
                    if has_wildcard_chirho {
                        redundant_indices_chirho.push(idx_chirho);
                    } else {
                        seen_lits_chirho.push(idx_chirho);
                    }
                }
            }
        }

        // ---- Redundancy warnings ----
        for idx_chirho in &redundant_indices_chirho {
            let span_chirho = spans_chirho
                .get(*idx_chirho)
                .copied()
                .unwrap_or(site_span_chirho);
            self.diags_chirho.push_chirho(
                DiagnosticChirho::warning_chirho(
                    format!(
                        "redundant pattern in `{site_name_chirho}`: \
                         this alternative can never be reached"
                    ),
                    span_chirho,
                )
                .with_code_chirho(ErrorCodeChirho::warning_chirho(REDUNDANT_CODE_CHIRHO)),
            );
        }

        // ---- Exhaustiveness check ----
        // If a wildcard is present, the match is exhaustive.
        if has_wildcard_chirho {
            // But check if patterns after the wildcard make the wildcard
            // itself redundant (already reported above).
            // Also, check if the wildcard is the ONLY pattern and it follows
            // all constructors — that's already caught above.
            let _ = wildcard_index_chirho;
            return;
        }

        // If there are literal patterns but no wildcard, the match is
        // non-exhaustive (we can't enumerate all possible literals).
        if !seen_lits_chirho.is_empty() && seen_cons_chirho.is_empty() {
            self.diags_chirho.push_chirho(
                DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(NON_EXHAUSTIVE_CODE_CHIRHO),
                    format!(
                        "non-exhaustive patterns in `{site_name_chirho}`: \
                         literal patterns require a wildcard or variable \
                         catch-all to be exhaustive"
                    ),
                    site_span_chirho,
                )
                .with_note_chirho("add a wildcard pattern `_` to cover remaining cases"),
            );
            return;
        }

        // If there are constructor patterns, check whether all constructors
        // of the matched type are present.
        if !seen_cons_chirho.is_empty() {
            let first_con_chirho = &seen_cons_chirho[0].0;
            if let Some(siblings_chirho) = self.env_chirho.siblings_of_chirho(first_con_chirho) {
                let covered_chirho: HashSet<&str> = seen_cons_chirho
                    .iter()
                    .map(|(n_chirho, _)| n_chirho.as_str())
                    .collect();
                let missing_chirho: Vec<&str> = siblings_chirho
                    .iter()
                    .filter(|c_chirho| !covered_chirho.contains(c_chirho.name_chirho.as_str()))
                    .map(|c_chirho| c_chirho.name_chirho.as_str())
                    .collect();

                if !missing_chirho.is_empty() {
                    let missing_list_chirho = missing_chirho.join(", ");
                    self.diags_chirho.push_chirho(
                        DiagnosticChirho::error_with_code_chirho(
                            ErrorCodeChirho::error_chirho(NON_EXHAUSTIVE_CODE_CHIRHO),
                            format!(
                                "non-exhaustive patterns in `{site_name_chirho}`: \
                                 missing constructor(s): {missing_list_chirho}"
                            ),
                            site_span_chirho,
                        )
                        .with_note_chirho(format!(
                            "add pattern(s) for {missing_list_chirho} or use a wildcard `_`"
                        )),
                    );
                }
            }
            // If the constructor is unknown (not in env), skip checking —
            // name resolution should have caught it already.
        }
    }
}

// ---------------------------------------------------------------------------
// Pattern classification
// ---------------------------------------------------------------------------

/// Simplified classification of a pattern for single-column analysis.
#[derive(Debug, Clone)]
enum PatClassChirho {
    /// Variable or wildcard — matches everything.
    WildcardChirho,
    /// Constructor pattern — matches one constructor.
    ConChirho(String),
    /// Literal pattern — matches one value.
    LitChirho,
}

/// Classify a pattern, stripping through syntactic sugar.
fn classify_pattern_chirho(pat_chirho: &PatChirho) -> PatClassChirho {
    match pat_chirho {
        PatChirho::VarChirho(_) | PatChirho::WildcardChirho(_) => PatClassChirho::WildcardChirho,
        PatChirho::ConChirho { con_chirho, .. } => {
            PatClassChirho::ConChirho(con_chirho.text_chirho().to_string())
        }
        PatChirho::LitChirho(_) | PatChirho::NegChirho { .. } => PatClassChirho::LitChirho,
        PatChirho::AsChirho { pattern_chirho, .. } => classify_pattern_chirho(pattern_chirho),
        PatChirho::ParenChirho { inner_chirho, .. } => classify_pattern_chirho(inner_chirho),
        PatChirho::LazyChirho { inner_chirho, .. } => classify_pattern_chirho(inner_chirho),
        PatChirho::BangChirho { inner_chirho, .. } => classify_pattern_chirho(inner_chirho),
        PatChirho::TupleChirho { .. } => {
            // A tuple pattern is always a single constructor "(,,...,)"
            // so it is exhaustive by itself (only one tuple constructor).
            PatClassChirho::WildcardChirho
        }
        PatChirho::ListChirho { elements_chirho, .. } => {
            // [] is the nil constructor; [a, b, c] desugars to a:b:c:[]
            // which starts with the (:) constructor.
            if elements_chirho.is_empty() {
                PatClassChirho::ConChirho("[]".to_string())
            } else {
                PatClassChirho::ConChirho(":".to_string())
            }
        }
        PatChirho::InfixConChirho { op_chirho, .. } => {
            PatClassChirho::ConChirho(op_chirho.text_chirho().to_string())
        }
        PatChirho::RecordChirho { con_chirho, .. } => {
            PatClassChirho::ConChirho(con_chirho.text_chirho().to_string())
        }
    }
}

/// Check whether a set of seen constructors already covers all constructors
/// of their type.
fn all_cons_covered_chirho(
    seen_chirho: &[(String, usize)],
    env_chirho: &TypeConEnvChirho,
) -> bool {
    if seen_chirho.is_empty() {
        return false;
    }
    let first_chirho = &seen_chirho[0].0;
    if let Some(siblings_chirho) = env_chirho.siblings_of_chirho(first_chirho) {
        let covered_chirho: HashSet<&str> =
            seen_chirho.iter().map(|(n_chirho, _)| n_chirho.as_str()).collect();
        siblings_chirho
            .iter()
            .all(|c_chirho| covered_chirho.contains(c_chirho.name_chirho.as_str()))
    } else {
        false
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhasky_ast_chirho::expr_chirho::{AltChirho, RhsChirho};
    use rhasky_ast_chirho::lit_chirho::LitChirho;
    use rhasky_ast_chirho::module_chirho::ModuleChirho;
    use rhasky_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    use rhasky_span_chirho::SpanChirho;

    fn mk_name_chirho(s_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            s_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    fn mk_module_chirho(decls_chirho: Vec<DeclChirho>) -> ModuleChirho {
        ModuleChirho {
            name_chirho: mk_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho,
            extensions_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_data_decl_chirho(
        name_chirho: &str,
        cons_chirho: Vec<(&str, usize)>,
    ) -> DeclChirho {
        let constructors_chirho = cons_chirho
            .into_iter()
            .map(|(cn_chirho, arity_chirho)| {
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: mk_name_chirho(cn_chirho),
                    fields_chirho: (0..arity_chirho)
                        .map(|_| {
                            rhasky_ast_chirho::ty_chirho::TypeChirho::ConChirho(
                                mk_name_chirho("Int"),
                            )
                        })
                        .collect(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }
            })
            .collect();
        DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho(name_chirho),
            type_vars_chirho: vec![],
            constructors_chirho,
            deriving_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_con_pat_chirho(name_chirho: &str) -> PatChirho {
        PatChirho::ConChirho {
            con_chirho: mk_name_chirho(name_chirho),
            args_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_var_pat_chirho(name_chirho: &str) -> PatChirho {
        PatChirho::VarChirho(mk_name_chirho(name_chirho))
    }

    fn mk_wild_pat_chirho() -> PatChirho {
        PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)
    }

    fn mk_lit_pat_chirho(val_chirho: i64) -> PatChirho {
        PatChirho::LitChirho(LitChirho::IntChirho(val_chirho, SpanChirho::DUMMY_CHIRHO))
    }

    fn mk_case_decl_chirho(
        fn_name_chirho: &str,
        pats_chirho: Vec<PatChirho>,
    ) -> DeclChirho {
        // Wraps a case expression with the given alt patterns inside a
        // simple function binding: fn = case x of { pats... }
        let alts_chirho: Vec<AltChirho> = pats_chirho
            .into_iter()
            .map(|p_chirho| AltChirho {
                pat_chirho: p_chirho,
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                    LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO),
                )),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            })
            .collect();

        DeclChirho::FunBindChirho {
            name_chirho: mk_name_chirho(fn_name_chirho),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(ExprChirho::VarChirho(mk_name_chirho("x"))),
                    alts_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_fun_decl_chirho(
        fn_name_chirho: &str,
        equations_chirho: Vec<Vec<PatChirho>>,
    ) -> DeclChirho {
        let matches_chirho = equations_chirho
            .into_iter()
            .map(|pats_chirho| MatchArmChirho {
                pats_chirho,
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                    LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO),
                )),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            })
            .collect();

        DeclChirho::FunBindChirho {
            name_chirho: mk_name_chirho(fn_name_chirho),
            matches_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    // ---------------------------------------------------------------
    // Constructor environment tests
    // ---------------------------------------------------------------

    #[test]
    fn con_env_from_module_chirho() {
        let module_chirho = mk_module_chirho(vec![mk_data_decl_chirho(
            "Color",
            vec![("Red", 0), ("Green", 0), ("Blue", 0)],
        )]);
        let env_chirho = TypeConEnvChirho::from_module_chirho(&module_chirho);

        assert_eq!(env_chirho.type_of_con_chirho("Red"), Some("Color"));
        assert_eq!(env_chirho.type_of_con_chirho("Green"), Some("Color"));
        let cons_chirho = env_chirho.constructors_of_chirho("Color").unwrap();
        assert_eq!(cons_chirho.len(), 3);
    }

    #[test]
    fn con_env_builtins_chirho() {
        let module_chirho = mk_module_chirho(vec![]);
        let env_chirho = TypeConEnvChirho::from_module_chirho(&module_chirho);

        assert_eq!(env_chirho.type_of_con_chirho("True"), Some("Bool"));
        assert_eq!(env_chirho.type_of_con_chirho("Nothing"), Some("Maybe"));
        assert_eq!(env_chirho.type_of_con_chirho("Left"), Some("Either"));
        assert_eq!(env_chirho.type_of_con_chirho("LT"), Some("Ordering"));
    }

    #[test]
    fn con_env_siblings_chirho() {
        let module_chirho = mk_module_chirho(vec![mk_data_decl_chirho(
            "Shape",
            vec![("Circle", 1), ("Rect", 2)],
        )]);
        let env_chirho = TypeConEnvChirho::from_module_chirho(&module_chirho);
        let sibs_chirho = env_chirho.siblings_of_chirho("Circle").unwrap();
        assert_eq!(sibs_chirho.len(), 2);
        assert_eq!(sibs_chirho[0].name_chirho, "Circle");
        assert_eq!(sibs_chirho[1].name_chirho, "Rect");
    }

    // ---------------------------------------------------------------
    // Exhaustive case expression tests
    // ---------------------------------------------------------------

    #[test]
    fn case_exhaustive_all_constructors_chirho() {
        let module_chirho = mk_module_chirho(vec![
            mk_data_decl_chirho("Color", vec![("Red", 0), ("Green", 0), ("Blue", 0)]),
            mk_case_decl_chirho(
                "f",
                vec![
                    mk_con_pat_chirho("Red"),
                    mk_con_pat_chirho("Green"),
                    mk_con_pat_chirho("Blue"),
                ],
            ),
        ]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "all constructors covered — no errors expected"
        );
        assert_eq!(result_chirho.diagnostics_chirho.len_chirho(), 0);
    }

    #[test]
    fn case_exhaustive_with_wildcard_chirho() {
        let module_chirho = mk_module_chirho(vec![
            mk_data_decl_chirho("Color", vec![("Red", 0), ("Green", 0), ("Blue", 0)]),
            mk_case_decl_chirho(
                "f",
                vec![mk_con_pat_chirho("Red"), mk_wild_pat_chirho()],
            ),
        ]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(result_chirho.diagnostics_chirho.len_chirho(), 0);
    }

    #[test]
    fn case_exhaustive_with_variable_chirho() {
        let module_chirho = mk_module_chirho(vec![
            mk_data_decl_chirho("Color", vec![("Red", 0), ("Green", 0), ("Blue", 0)]),
            mk_case_decl_chirho(
                "f",
                vec![mk_con_pat_chirho("Red"), mk_var_pat_chirho("other")],
            ),
        ]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    }

    // ---------------------------------------------------------------
    // Non-exhaustive case expression tests
    // ---------------------------------------------------------------

    #[test]
    fn case_non_exhaustive_missing_constructor_chirho() {
        let module_chirho = mk_module_chirho(vec![
            mk_data_decl_chirho("Color", vec![("Red", 0), ("Green", 0), ("Blue", 0)]),
            mk_case_decl_chirho(
                "f",
                vec![mk_con_pat_chirho("Red"), mk_con_pat_chirho("Green")],
            ),
        ]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(result_chirho.diagnostics_chirho.has_errors_chirho());
        let diags_chirho = result_chirho.diagnostics_chirho.diagnostics_chirho();
        assert_eq!(diags_chirho.len(), 1);
        assert!(diags_chirho[0].message_chirho.contains("Blue"));
        assert_eq!(
            diags_chirho[0].code_chirho,
            Some(ErrorCodeChirho::error_chirho(400))
        );
    }

    #[test]
    fn case_non_exhaustive_bool_chirho() {
        let module_chirho = mk_module_chirho(vec![mk_case_decl_chirho(
            "f",
            vec![mk_con_pat_chirho("True")],
        )]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(result_chirho.diagnostics_chirho.has_errors_chirho());
        let msg_chirho = &result_chirho.diagnostics_chirho.diagnostics_chirho()[0].message_chirho;
        assert!(msg_chirho.contains("False"));
    }

    #[test]
    fn case_non_exhaustive_literal_chirho() {
        let module_chirho = mk_module_chirho(vec![mk_case_decl_chirho(
            "f",
            vec![mk_lit_pat_chirho(1), mk_lit_pat_chirho(2)],
        )]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(result_chirho.diagnostics_chirho.has_errors_chirho());
        let msg_chirho = &result_chirho.diagnostics_chirho.diagnostics_chirho()[0].message_chirho;
        assert!(msg_chirho.contains("literal"));
    }

    // ---------------------------------------------------------------
    // Redundancy tests
    // ---------------------------------------------------------------

    #[test]
    fn case_redundant_after_wildcard_chirho() {
        let module_chirho = mk_module_chirho(vec![
            mk_data_decl_chirho("Color", vec![("Red", 0), ("Green", 0), ("Blue", 0)]),
            mk_case_decl_chirho(
                "f",
                vec![
                    mk_con_pat_chirho("Red"),
                    mk_wild_pat_chirho(),
                    mk_con_pat_chirho("Blue"), // unreachable
                ],
            ),
        ]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(result_chirho.diagnostics_chirho.warning_count_chirho(), 1);
    }

    #[test]
    fn case_redundant_duplicate_constructor_chirho() {
        let module_chirho = mk_module_chirho(vec![
            mk_data_decl_chirho("Color", vec![("Red", 0), ("Green", 0), ("Blue", 0)]),
            mk_case_decl_chirho(
                "f",
                vec![
                    mk_con_pat_chirho("Red"),
                    mk_con_pat_chirho("Red"), // duplicate
                    mk_con_pat_chirho("Green"),
                    mk_con_pat_chirho("Blue"),
                ],
            ),
        ]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(result_chirho.diagnostics_chirho.warning_count_chirho(), 1);
    }

    #[test]
    fn case_redundant_wildcard_after_all_cons_chirho() {
        let module_chirho = mk_module_chirho(vec![
            mk_data_decl_chirho("Bool2", vec![("T", 0), ("F", 0)]),
            mk_case_decl_chirho(
                "f",
                vec![
                    mk_con_pat_chirho("T"),
                    mk_con_pat_chirho("F"),
                    mk_wild_pat_chirho(), // redundant — all cons already covered
                ],
            ),
        ]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(result_chirho.diagnostics_chirho.warning_count_chirho(), 1);
    }

    // ---------------------------------------------------------------
    // Function binding tests
    // ---------------------------------------------------------------

    #[test]
    fn fun_exhaustive_all_constructors_chirho() {
        let module_chirho = mk_module_chirho(vec![
            mk_data_decl_chirho("AB", vec![("A", 0), ("B", 0)]),
            mk_fun_decl_chirho(
                "f",
                vec![vec![mk_con_pat_chirho("A")], vec![mk_con_pat_chirho("B")]],
            ),
        ]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(result_chirho.diagnostics_chirho.len_chirho(), 0);
    }

    #[test]
    fn fun_non_exhaustive_chirho() {
        let module_chirho = mk_module_chirho(vec![
            mk_data_decl_chirho("AB", vec![("A", 0), ("B", 0)]),
            mk_fun_decl_chirho("f", vec![vec![mk_con_pat_chirho("A")]]),
        ]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(result_chirho.diagnostics_chirho.has_errors_chirho());
        let msg_chirho = &result_chirho.diagnostics_chirho.diagnostics_chirho()[0].message_chirho;
        assert!(msg_chirho.contains("B"));
    }

    #[test]
    fn fun_with_wildcard_exhaustive_chirho() {
        let module_chirho = mk_module_chirho(vec![
            mk_data_decl_chirho("AB", vec![("A", 0), ("B", 0)]),
            mk_fun_decl_chirho(
                "f",
                vec![vec![mk_con_pat_chirho("A")], vec![mk_var_pat_chirho("x")]],
            ),
        ]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    }

    // ---------------------------------------------------------------
    // Newtype test
    // ---------------------------------------------------------------

    #[test]
    fn newtype_single_constructor_exhaustive_chirho() {
        let module_chirho = mk_module_chirho(vec![
            DeclChirho::NewtypeDeclChirho {
                name_chirho: mk_name_chirho("Wrapper"),
                type_vars_chirho: vec![],
                constructor_chirho: ConDeclChirho::OrdinaryChirho {
                    name_chirho: mk_name_chirho("Wrap"),
                    fields_chirho: vec![rhasky_ast_chirho::ty_chirho::TypeChirho::ConChirho(
                        mk_name_chirho("Int"),
                    )],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                deriving_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            mk_case_decl_chirho("f", vec![mk_con_pat_chirho("Wrap")]),
        ]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    }

    // ---------------------------------------------------------------
    // Pattern classification tests
    // ---------------------------------------------------------------

    #[test]
    fn classify_as_pattern_chirho() {
        // x@Red should classify as Con("Red")
        let pat_chirho = PatChirho::AsChirho {
            name_chirho: mk_name_chirho("x"),
            pattern_chirho: Box::new(mk_con_pat_chirho("Red")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        match classify_pattern_chirho(&pat_chirho) {
            PatClassChirho::ConChirho(n_chirho) => assert_eq!(n_chirho, "Red"),
            other_chirho => panic!("expected Con, got {:?}", other_chirho),
        }
    }

    #[test]
    fn classify_paren_pattern_chirho() {
        let pat_chirho = PatChirho::ParenChirho {
            inner_chirho: Box::new(mk_wild_pat_chirho()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        assert!(matches!(
            classify_pattern_chirho(&pat_chirho),
            PatClassChirho::WildcardChirho
        ));
    }

    // ---------------------------------------------------------------
    // Edge cases
    // ---------------------------------------------------------------

    #[test]
    fn empty_module_no_errors_chirho() {
        let module_chirho = mk_module_chirho(vec![]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    }

    #[test]
    fn single_wildcard_is_exhaustive_chirho() {
        let module_chirho = mk_module_chirho(vec![
            mk_data_decl_chirho("Color", vec![("Red", 0), ("Green", 0), ("Blue", 0)]),
            mk_case_decl_chirho("f", vec![mk_wild_pat_chirho()]),
        ]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    }

    #[test]
    fn literal_with_wildcard_is_exhaustive_chirho() {
        let module_chirho = mk_module_chirho(vec![mk_case_decl_chirho(
            "f",
            vec![mk_lit_pat_chirho(1), mk_wild_pat_chirho()],
        )]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    }

    #[test]
    fn maybe_exhaustive_chirho() {
        let module_chirho = mk_module_chirho(vec![mk_case_decl_chirho(
            "f",
            vec![
                mk_con_pat_chirho("Nothing"),
                mk_con_pat_chirho("Just"),
            ],
        )]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    }

    #[test]
    fn maybe_non_exhaustive_chirho() {
        let module_chirho = mk_module_chirho(vec![mk_case_decl_chirho(
            "f",
            vec![mk_con_pat_chirho("Just")],
        )]);
        let result_chirho = check_module_exhaustiveness_chirho(&module_chirho);
        assert!(result_chirho.diagnostics_chirho.has_errors_chirho());
        assert!(
            result_chirho.diagnostics_chirho.diagnostics_chirho()[0]
                .message_chirho
                .contains("Nothing")
        );
    }
}
