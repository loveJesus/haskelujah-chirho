// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A pattern binding, as the Haskell Report defines it.
//!
//! `p = e` binds each variable `x_i` of `p` as `case e of p -> x_i` (Haskell
//! 2010 §3.17.3, rule (d)): the WHOLE pattern is matched when ANY variable is
//! demanded, and not before. Both halves are observable:
//!
//! - nothing is forced at the binding site, so `let (Just n) = undefined in 5`
//!   is 5, and an unused variable never fails;
//! - demanding `x` in `(x, Just y) = (1, Nothing)` fails, because the match of
//!   the whole pattern fails, even though `x`'s own position matched.
//!
//! An earlier version bound each variable through a selector that checked only
//! the path to that variable. It got the first half right and the second half
//! silently wrong, in every binding position. So the match is now built once,
//! by the single-row matcher, and shared:
//!
//! ```text
//! $patbind = e
//! $m       = case $patbind of p -> (x1, .., xn)     -- the full match, on demand
//! x_i      = case $m of (t1, .., tn) -> t_i
//! ```
//!
//! A banged binding `!p = e` is strict: the MATCH is forced before the body,
//! and the variables are not (`let !(Just n) = Just undefined in 5` is 5,
//! `let !(Just n) = Nothing in 5` fails). The caller forces the returned
//! witness.
//! workflow: language-features-chirho/pattern-matching-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::expr_chirho::LocalBindChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;

use super::DesugarCtxChirho;
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho,
};

/// The Core for one pattern binding.
pub(super) struct PatternBindingChirho {
    /// Bindings for one `let` group: the right-hand side, the shared match, and
    /// one selector per variable.
    pub(super) bindings_chirho: Vec<(BinderChirho, CoreExprChirho)>,
    /// For a banged binding, the value to force before the body. Forcing it
    /// performs the match and nothing more.
    pub(super) force_chirho: Option<CoreIdChirho>,
}

impl DesugarCtxChirho {
    /// Bind every variable of `pat_chirho` in the CURRENT scope to a fresh
    /// binder, and return them by name.
    ///
    /// A binding group's body and siblings are desugared before the pattern
    /// binding itself is built, and must already resolve its variables; the
    /// binding then reuses exactly these binders.
    pub(super) fn prebind_pattern_variables_chirho(
        &mut self,
        pat_chirho: &PatChirho,
    ) -> HashMap<String, BinderChirho> {
        let mut prebound_chirho = HashMap::new();
        for name_chirho in self.pattern_variables_chirho(pat_chirho) {
            let binder_chirho = self.row_binder_chirho(&name_chirho);
            self.bind_in_scope_chirho(&name_chirho, binder_chirho.id_chirho);
            prebound_chirho.insert(name_chirho, binder_chirho);
        }
        prebound_chirho
    }

    /// The Core for `pat_chirho = rhs_chirho`. A variable found in
    /// `prebound_chirho` keeps that binder; any other gets a fresh one, bound
    /// in the current scope.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn pattern_binding_chirho(
        &mut self,
        pat_chirho: &PatChirho,
        rhs_chirho: CoreExprChirho,
        prebound_chirho: &HashMap<String, BinderChirho>,
    ) -> PatternBindingChirho {
        let (pat_chirho, strict_chirho) = binding_shape_chirho(pat_chirho);
        let names_chirho = self.pattern_variables_chirho(pat_chirho);

        // `x = e`, `!x = e`: the variable IS the right-hand side.
        if let Some(name_chirho) = bare_variable_chirho(pat_chirho) {
            let binder_chirho = self.binding_binder_chirho(name_chirho, prebound_chirho);
            let force_chirho = strict_chirho.then_some(binder_chirho.id_chirho);
            return PatternBindingChirho {
                bindings_chirho: vec![(binder_chirho, rhs_chirho)],
                force_chirho,
            };
        }

        let rhs_binder_chirho = self.row_binder_chirho("$patbind");
        let rhs_id_chirho = rhs_binder_chirho.id_chirho;
        let mut bindings_chirho = vec![(rhs_binder_chirho, rhs_chirho)];

        // The witness of a banged binding: the right-hand side forced, then the
        // match, yielding `()`. Forcing it cannot force a variable, and it
        // forces the right-hand side even when the pattern could never fail
        // (`!_`, `!~p`), as a bang must.
        let force_chirho = if strict_chirho {
            let matched_chirho = self.full_match_chirho(pat_chirho, rhs_id_chirho, &mut |_| {
                CoreExprChirho::ConAppChirho {
                    con_name_chirho: "$tuple0".to_string(),
                    args_chirho: vec![],
                }
            });
            let witness_chirho = self.force_then_chirho(rhs_id_chirho, matched_chirho);
            let witness_binder_chirho = self.row_binder_chirho("$matched");
            let witness_id_chirho = witness_binder_chirho.id_chirho;
            bindings_chirho.push((witness_binder_chirho, witness_chirho));
            Some(witness_id_chirho)
        } else {
            None
        };

        match names_chirho.as_slice() {
            // Binds nothing: only the witness, if any, can ever demand it.
            [] => {}
            // One variable: the match returns it directly.
            [name_chirho] => {
                let selected_chirho =
                    self.full_match_chirho(pat_chirho, rhs_id_chirho, &mut |ctx_chirho| {
                        CoreExprChirho::VarChirho(ctx_chirho.bound_id_chirho(name_chirho))
                    });
                let binder_chirho = self.binding_binder_chirho(name_chirho, prebound_chirho);
                bindings_chirho.push((binder_chirho, selected_chirho));
            }
            // Several: one shared match returns them all, and each variable
            // selects its own.
            _ => {
                let arity_chirho = names_chirho.len();
                let tuple_con_chirho = format!("$tuple{arity_chirho}");
                let matched_chirho =
                    self.full_match_chirho(pat_chirho, rhs_id_chirho, &mut |ctx_chirho| {
                        CoreExprChirho::ConAppChirho {
                            con_name_chirho: tuple_con_chirho.clone(),
                            args_chirho: names_chirho
                                .iter()
                                .map(|name_chirho| {
                                    CoreExprChirho::VarChirho(
                                        ctx_chirho.bound_id_chirho(name_chirho),
                                    )
                                })
                                .collect(),
                        }
                    });
                let matched_binder_chirho = self.row_binder_chirho("$match");
                let matched_id_chirho = matched_binder_chirho.id_chirho;
                bindings_chirho.push((matched_binder_chirho, matched_chirho));
                for (index_chirho, name_chirho) in names_chirho.iter().enumerate() {
                    let fields_chirho: Vec<BinderChirho> = (0..arity_chirho)
                        .map(|position_chirho| {
                            self.row_binder_chirho(&format!("$sel{position_chirho}"))
                        })
                        .collect();
                    let chosen_chirho = fields_chirho[index_chirho].id_chirho;
                    let selector_chirho = CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(matched_id_chirho)),
                        bind_chirho: self.row_binder_chirho("$selwild"),
                        result_ty_chirho: self.row_ty_chirho(),
                        alts_chirho: vec![CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho(tuple_con_chirho.clone()),
                            binders_chirho: fields_chirho,
                            rhs_chirho: CoreExprChirho::VarChirho(chosen_chirho),
                        }],
                    };
                    let binder_chirho = self.binding_binder_chirho(name_chirho, prebound_chirho);
                    bindings_chirho.push((binder_chirho, selector_chirho));
                }
            }
        }
        PatternBindingChirho {
            bindings_chirho,
            force_chirho,
        }
    }

    /// Pre-bind the variables of each pattern binding among `binds_chirho` at
    /// `indices_chirho`, before the group's body and siblings are desugared.
    pub(super) fn prebind_pattern_bindings_chirho(
        &mut self,
        binds_chirho: &[LocalBindChirho],
        indices_chirho: &[usize],
    ) -> HashMap<usize, HashMap<String, BinderChirho>> {
        let mut prebound_chirho = HashMap::new();
        for &index_chirho in indices_chirho {
            if let LocalBindChirho::PatBindChirho { pat_chirho, .. } = &binds_chirho[index_chirho] {
                let variables_chirho = self.prebind_pattern_variables_chirho(pat_chirho);
                prebound_chirho.insert(index_chirho, variables_chirho);
            }
        }
        prebound_chirho
    }

    /// Desugar each pattern binding among `binds_chirho` at `indices_chirho`
    /// into `core_binds_chirho`, and return the witnesses of the banged ones in
    /// source order, for the caller to force before its body.
    ///
    /// The right-hand sides are desugared last binding first, the order these
    /// sites have always used, so no occurrence is renumbered by this change.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn desugar_pattern_bindings_chirho(
        &mut self,
        binds_chirho: &[LocalBindChirho],
        indices_chirho: &[usize],
        prebound_chirho: &HashMap<usize, HashMap<String, BinderChirho>>,
        core_binds_chirho: &mut Vec<(BinderChirho, CoreExprChirho)>,
    ) -> Vec<CoreIdChirho> {
        let none_prebound_chirho = HashMap::new();
        let mut witnesses_chirho = Vec::new();
        for &index_chirho in indices_chirho.iter().rev() {
            let LocalBindChirho::PatBindChirho {
                pat_chirho,
                rhs_chirho,
                ..
            } = &binds_chirho[index_chirho]
            else {
                continue;
            };
            let core_rhs_chirho = self.desugar_rhs_chirho(rhs_chirho);
            let binding_chirho = self.pattern_binding_chirho(
                pat_chirho,
                core_rhs_chirho,
                prebound_chirho
                    .get(&index_chirho)
                    .unwrap_or(&none_prebound_chirho),
            );
            core_binds_chirho.extend(binding_chirho.bindings_chirho);
            witnesses_chirho.extend(binding_chirho.force_chirho);
        }
        witnesses_chirho.reverse();
        witnesses_chirho
    }

    /// For a do statement `~p <- m`: the lambda parameter that receives the
    /// monadic result, and the bindings of `p` over it. `None` when the
    /// statement's pattern is not lazy at the top.
    ///
    /// ORDER MATTERS, and getting it wrong is silent: building these BINDS each
    /// variable in scope, so the rest of the block must be desugared
    /// AFTERWARDS, or its references resolve to ids nothing binds, which only
    /// shows up at run time as a missing STG binding.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn lazy_bind_prepare_chirho(
        &mut self,
        pat_chirho: &PatChirho,
    ) -> Option<(BinderChirho, Vec<(BinderChirho, CoreExprChirho)>)> {
        let PatChirho::LazyChirho { inner_chirho, .. } = strip_parens_chirho(pat_chirho) else {
            return None;
        };
        let parameter_chirho = self.row_binder_chirho("$lazybind");
        // The outer `~` makes the statement lazy whatever is inside it, so a
        // bang below it forces nothing at the statement: its witness is not
        // forced here, and its position is forced when the match runs.
        let binding_chirho = self.pattern_binding_chirho(
            inner_chirho,
            CoreExprChirho::VarChirho(parameter_chirho.id_chirho),
            &HashMap::new(),
        );
        Some((parameter_chirho, binding_chirho.bindings_chirho))
    }

    /// `\parameter -> let bindings in body`.
    pub(super) fn lazy_bind_lambda_chirho(
        parameter_chirho: BinderChirho,
        bindings_chirho: Vec<(BinderChirho, CoreExprChirho)>,
        body_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        CoreExprChirho::LamChirho {
            binder_chirho: parameter_chirho,
            body_chirho: Box::new(Self::let_all_chirho(bindings_chirho, body_chirho)),
        }
    }

    /// Force each witness, in source order, before `body_chirho`.
    pub(super) fn force_witnesses_chirho(
        &mut self,
        witnesses_chirho: &[CoreIdChirho],
        body_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        witnesses_chirho
            .iter()
            .rev()
            .fold(body_chirho, |body_chirho, witness_chirho| {
                self.force_then_chirho(*witness_chirho, body_chirho)
            })
    }

    /// Every variable a pattern binds, in source order, with record wildcards
    /// and pattern synonyms expanded exactly as the matcher expands them.
    pub(super) fn pattern_variables_chirho(&self, pat_chirho: &PatChirho) -> Vec<String> {
        let mut names_chirho = Vec::new();
        self.collect_pattern_variables_chirho(pat_chirho, &mut names_chirho);
        names_chirho
    }

    fn collect_pattern_variables_chirho(
        &self,
        pat_chirho: &PatChirho,
        out_chirho: &mut Vec<String>,
    ) {
        if let Some(expanded_chirho) = self.expand_pat_syn_chirho(pat_chirho) {
            return self.collect_pattern_variables_chirho(&expanded_chirho, out_chirho);
        }
        match pat_chirho {
            PatChirho::VarChirho(name_chirho) => {
                out_chirho.push(name_chirho.text_chirho().to_string())
            }
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho,
                ..
            } => {
                out_chirho.push(name_chirho.text_chirho().to_string());
                self.collect_pattern_variables_chirho(pattern_chirho, out_chirho);
            }
            PatChirho::WildcardChirho(_)
            | PatChirho::LitChirho(_)
            | PatChirho::NegChirho { .. } => {}
            PatChirho::ParenChirho { inner_chirho, .. }
            | PatChirho::BangChirho { inner_chirho, .. }
            | PatChirho::LazyChirho { inner_chirho, .. } => {
                self.collect_pattern_variables_chirho(inner_chirho, out_chirho);
            }
            PatChirho::TypeAnnotChirho { pat_chirho, .. }
            | PatChirho::ViewChirho { pat_chirho, .. } => {
                self.collect_pattern_variables_chirho(pat_chirho, out_chirho);
            }
            PatChirho::ConChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    self.collect_pattern_variables_chirho(arg_chirho, out_chirho);
                }
            }
            PatChirho::TupleChirho {
                elements_chirho, ..
            }
            | PatChirho::ListChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    self.collect_pattern_variables_chirho(element_chirho, out_chirho);
                }
            }
            PatChirho::InfixConChirho {
                left_chirho,
                right_chirho,
                ..
            } => {
                self.collect_pattern_variables_chirho(left_chirho, out_chirho);
                self.collect_pattern_variables_chirho(right_chirho, out_chirho);
            }
            PatChirho::RecordChirho { .. } => {
                let expanded_chirho = self.expand_record_wildcard_pat_chirho(pat_chirho);
                if let PatChirho::RecordChirho { fields_chirho, .. } =
                    expanded_chirho.as_ref().unwrap_or(pat_chirho)
                {
                    for field_chirho in fields_chirho {
                        self.collect_pattern_variables_chirho(
                            &field_chirho.pattern_chirho,
                            out_chirho,
                        );
                    }
                }
            }
        }
    }

    /// Match the whole of `pat_chirho` against `rhs_id_chirho` in a scope of
    /// its own, continuing with `success_chirho`, and failing with GHC's reason.
    fn full_match_chirho(
        &mut self,
        pat_chirho: &PatChirho,
        rhs_id_chirho: CoreIdChirho,
        success_chirho: &mut super::row_match_chirho::MatchSuccessChirho<'_>,
    ) -> CoreExprChirho {
        let failure_chirho = self.pattern_binding_failure_chirho();
        self.push_scope_chirho();
        let matched_chirho =
            self.match_row_chirho(pat_chirho, rhs_id_chirho, &failure_chirho, success_chirho);
        self.pop_scope_chirho();
        matched_chirho
    }

    /// `error "Non-exhaustive patterns in ..."`: the failure GHC 9.14.1 raises
    /// when a pattern binding's match is demanded and fails. GHC names the
    /// pattern's source text after `in`; this names the construct.
    fn pattern_binding_failure_chirho(&mut self) -> CoreExprChirho {
        let error_id_chirho = self.fresh_id_chirho("error");
        CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(error_id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                "Non-exhaustive patterns in pattern binding".to_string(),
            ))),
        }
    }

    /// The binder a pattern binding gives `name_chirho`: the group's pre-bound
    /// one when there is one, otherwise a fresh one bound in the current scope.
    /// It never reuses a same-named binder from an OUTER scope, which would be
    /// capture rather than shadowing.
    fn binding_binder_chirho(
        &mut self,
        name_chirho: &str,
        prebound_chirho: &HashMap<String, BinderChirho>,
    ) -> BinderChirho {
        if let Some(binder_chirho) = prebound_chirho.get(name_chirho) {
            return binder_chirho.clone();
        }
        let binder_chirho = self.row_binder_chirho(name_chirho);
        self.bind_in_scope_chirho(name_chirho, binder_chirho.id_chirho);
        binder_chirho
    }

    /// The id a variable is bound to at this point of a match. The matcher binds
    /// every variable `pattern_variables_chirho` names, so the fallback is not
    /// expected; if it is ever reached, the program fails LOUDLY at run time on
    /// an unbound name, rather than the compiler panicking or binding a wrong one.
    fn bound_id_chirho(&mut self, name_chirho: &str) -> CoreIdChirho {
        match self.lookup_scope_chirho(name_chirho) {
            Some(id_chirho) => id_chirho,
            None => self.resolve_var_chirho(name_chirho),
        }
    }
}

/// The pattern a binding matches, and whether the binding is strict. Outer
/// parentheses and an outer `~` change nothing about a pattern binding, which
/// is lazy already; an outer `!` makes it strict. A bang anywhere deeper is
/// part of the match, and the matcher forces that position when the match runs.
fn binding_shape_chirho(pat_chirho: &PatChirho) -> (&PatChirho, bool) {
    match pat_chirho {
        PatChirho::ParenChirho { inner_chirho, .. } => binding_shape_chirho(inner_chirho),
        PatChirho::LazyChirho { inner_chirho, .. } => (strip_parens_chirho(inner_chirho), false),
        PatChirho::BangChirho { inner_chirho, .. } => (strip_parens_chirho(inner_chirho), true),
        other_chirho => (other_chirho, false),
    }
}

fn strip_parens_chirho(pat_chirho: &PatChirho) -> &PatChirho {
    match pat_chirho {
        PatChirho::ParenChirho { inner_chirho, .. } => strip_parens_chirho(inner_chirho),
        other_chirho => other_chirho,
    }
}

/// The variable a pattern is, when it is nothing more than one.
fn bare_variable_chirho(pat_chirho: &PatChirho) -> Option<&str> {
    match pat_chirho {
        PatChirho::VarChirho(name_chirho) => Some(name_chirho.text_chirho()),
        PatChirho::ParenChirho { inner_chirho, .. } => bare_variable_chirho(inner_chirho),
        PatChirho::TypeAnnotChirho { pat_chirho, .. } => bare_variable_chirho(pat_chirho),
        _ => None,
    }
}
