// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One pattern against one value: succeed with its variables bound, or fail.
//!
//! This is the single-row matcher every match should be built from. It exists
//! because the older machinery is a set of special cases that each drop
//! something: a literal below a constructor is never tested, a record binds its
//! listed fields by position, a list or a nested constructor in some positions
//! binds nothing at all. Each of those is a silent wrong answer somewhere.
//!
//! The contract is small and total. `match_row_chirho(p, v, failure, success)`
//! builds Core that matches `p` against the value bound to `v`, left to right
//! and depth first as Haskell specifies. When the whole pattern matches,
//! it continues with `success`, which is called exactly once, at the point
//! where every variable of `p` is bound in the CURRENT scope. Anything that
//! does not match evaluates `failure`.
//!
//! `failure` is cloned into every alternative that can fail, so callers pass
//! something small: an `error` call or a variable bound to the real failure.
//! workflow: language-features-chirho/pattern-matching-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::DesugarCtxChirho;
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho,
};

/// What a successful match continues with. It runs once every variable of the
/// pattern is in scope, which is what lets it refer to them by name.
pub(super) type MatchSuccessChirho<'s> = dyn FnMut(&mut DesugarCtxChirho) -> CoreExprChirho + 's;

impl DesugarCtxChirho {
    /// Match `pat_chirho` against the value bound to `scrutinee_chirho`.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn match_row_chirho(
        &mut self,
        pat_chirho: &PatChirho,
        scrutinee_chirho: CoreIdChirho,
        failure_chirho: &CoreExprChirho,
        success_chirho: &mut MatchSuccessChirho<'_>,
    ) -> CoreExprChirho {
        if let Some(expanded_chirho) = self.expand_pat_syn_chirho(pat_chirho) {
            return self.match_row_chirho(
                &expanded_chirho,
                scrutinee_chirho,
                failure_chirho,
                success_chirho,
            );
        }
        match pat_chirho {
            PatChirho::VarChirho(name_chirho) => {
                self.bind_in_scope_chirho(name_chirho.text_chirho(), scrutinee_chirho);
                success_chirho(self)
            }
            PatChirho::WildcardChirho(_) => success_chirho(self),
            PatChirho::ParenChirho { inner_chirho, .. } => {
                self.match_row_chirho(inner_chirho, scrutinee_chirho, failure_chirho, success_chirho)
            }
            PatChirho::TypeAnnotChirho { pat_chirho, .. } => {
                self.match_row_chirho(pat_chirho, scrutinee_chirho, failure_chirho, success_chirho)
            }
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho,
                ..
            } => {
                self.bind_in_scope_chirho(name_chirho.text_chirho(), scrutinee_chirho);
                self.match_row_chirho(
                    pattern_chirho,
                    scrutinee_chirho,
                    failure_chirho,
                    success_chirho,
                )
            }
            // A bang forces this position before anything inside it is
            // matched, even when what is inside could never fail.
            PatChirho::BangChirho { inner_chirho, .. } => {
                let matched_chirho = self.match_row_chirho(
                    inner_chirho,
                    scrutinee_chirho,
                    failure_chirho,
                    success_chirho,
                );
                self.force_then_chirho(scrutinee_chirho, matched_chirho)
            }
            // A lazy pattern always matches here. Its variables are bound
            // through their own pattern binding over this position, so nothing
            // below the `~` is looked at until one of them is demanded.
            PatChirho::LazyChirho { inner_chirho, .. } => {
                let binding_chirho = self.pattern_binding_chirho(
                    inner_chirho,
                    CoreExprChirho::VarChirho(scrutinee_chirho),
                    &HashMap::new(),
                );
                let body_chirho = success_chirho(self);
                Self::let_all_chirho(binding_chirho.bindings_chirho, body_chirho)
            }
            PatChirho::LitChirho(lit_chirho) => self.match_literal_chirho(
                lit_chirho,
                false,
                scrutinee_chirho,
                failure_chirho,
                success_chirho,
            ),
            PatChirho::NegChirho { lit_chirho, .. } => self.match_literal_chirho(
                lit_chirho,
                true,
                scrutinee_chirho,
                failure_chirho,
                success_chirho,
            ),
            PatChirho::ConChirho {
                con_chirho,
                args_chirho,
                ..
            } => {
                let fields_chirho: Vec<Option<&PatChirho>> = args_chirho.iter().map(Some).collect();
                self.match_constructor_chirho(
                    con_chirho.text_chirho(),
                    &fields_chirho,
                    scrutinee_chirho,
                    failure_chirho,
                    success_chirho,
                )
            }
            PatChirho::InfixConChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                ..
            } => self.match_constructor_chirho(
                op_chirho.text_chirho(),
                &[Some(left_chirho.as_ref()), Some(right_chirho.as_ref())],
                scrutinee_chirho,
                failure_chirho,
                success_chirho,
            ),
            PatChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let fields_chirho: Vec<Option<&PatChirho>> =
                    elements_chirho.iter().map(Some).collect();
                self.match_constructor_chirho(
                    &format!("$tuple{}", elements_chirho.len()),
                    &fields_chirho,
                    scrutinee_chirho,
                    failure_chirho,
                    success_chirho,
                )
            }
            PatChirho::RecordChirho { .. } => {
                self.match_record_chirho(pat_chirho, scrutinee_chirho, failure_chirho, success_chirho)
            }
            PatChirho::ListChirho {
                elements_chirho, ..
            } => self.match_list_chirho(
                elements_chirho,
                scrutinee_chirho,
                failure_chirho,
                success_chirho,
            ),
            // `(f -> p)` matches `p` against `f v`. The application is bound
            // lazily, so an irrefutable `p` never calls `f` unless a variable
            // is demanded, and it is desugared here, after every variable to
            // its left is in scope, because a view may refer to them.
            PatChirho::ViewChirho {
                expr_chirho,
                pat_chirho: inner_chirho,
                ..
            } => {
                let view_chirho = self.desugar_expr_chirho(expr_chirho);
                let viewed_chirho = self.row_binder_chirho("$view");
                let matched_chirho = self.match_row_chirho(
                    inner_chirho,
                    viewed_chirho.id_chirho,
                    failure_chirho,
                    success_chirho,
                );
                CoreExprChirho::LetChirho {
                    rec_chirho: false,
                    binds_chirho: vec![(
                        viewed_chirho,
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(view_chirho),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(scrutinee_chirho)),
                        },
                    )],
                    body_chirho: Box::new(matched_chirho),
                }
            }
        }
    }

    /// Match the fields of one constructor, in order. `None` is a field the
    /// pattern does not mention: it gets a binder, because the constructor has
    /// it, and nothing is matched against it.
    fn match_constructor_chirho(
        &mut self,
        con_chirho: &str,
        fields_chirho: &[Option<&PatChirho>],
        scrutinee_chirho: CoreIdChirho,
        failure_chirho: &CoreExprChirho,
        success_chirho: &mut MatchSuccessChirho<'_>,
    ) -> CoreExprChirho {
        let binders_chirho: Vec<BinderChirho> = (0..fields_chirho.len())
            .map(|index_chirho| self.row_binder_chirho(&format!("$field{index_chirho}")))
            .collect();
        let pending_chirho: Vec<(&PatChirho, CoreIdChirho)> = fields_chirho
            .iter()
            .zip(&binders_chirho)
            .filter_map(|(field_chirho, binder_chirho)| {
                field_chirho.map(|pat_chirho| (pat_chirho, binder_chirho.id_chirho))
            })
            .collect();
        let matched_chirho =
            self.match_sequence_chirho(&pending_chirho, failure_chirho, success_chirho);
        // A tuple is its type's only constructor, so it cannot fail here and
        // gets no failure alternative.
        let can_fail_chirho = !con_chirho.starts_with("$tuple");
        self.case_on_chirho(
            scrutinee_chirho,
            AltConChirho::DataConChirho(con_chirho.to_string()),
            binders_chirho,
            matched_chirho,
            can_fail_chirho.then(|| failure_chirho.clone()),
        )
    }

    /// Match each pattern against its value, left to right, then succeed.
    fn match_sequence_chirho(
        &mut self,
        pending_chirho: &[(&PatChirho, CoreIdChirho)],
        failure_chirho: &CoreExprChirho,
        success_chirho: &mut MatchSuccessChirho<'_>,
    ) -> CoreExprChirho {
        let Some(((pat_chirho, value_chirho), rest_chirho)) = pending_chirho.split_first() else {
            return success_chirho(self);
        };
        self.match_row_chirho(pat_chirho, *value_chirho, failure_chirho, &mut |ctx_chirho| {
            ctx_chirho.match_sequence_chirho(rest_chirho, failure_chirho, success_chirho)
        })
    }

    /// A record pattern names its fields, in any order and any subset. Each is
    /// matched at its DECLARED position; the listed order means nothing.
    fn match_record_chirho(
        &mut self,
        pat_chirho: &PatChirho,
        scrutinee_chirho: CoreIdChirho,
        failure_chirho: &CoreExprChirho,
        success_chirho: &mut MatchSuccessChirho<'_>,
    ) -> CoreExprChirho {
        let expanded_chirho = self.expand_record_wildcard_pat_chirho(pat_chirho);
        let PatChirho::RecordChirho {
            con_chirho,
            fields_chirho,
            ..
        } = expanded_chirho.as_ref().unwrap_or(pat_chirho)
        else {
            unreachable!("match_record_chirho is only called on a record pattern");
        };
        let con_name_chirho = con_chirho.text_chirho();
        let declared_chirho = self.con_field_names_chirho.get(con_name_chirho).cloned();
        let positional_chirho: Vec<Option<&PatChirho>> = match declared_chirho {
            Some(declared_chirho) => {
                let mut positional_chirho = vec![None; declared_chirho.len()];
                for field_chirho in fields_chirho {
                    if let Some(index_chirho) = declared_chirho
                        .iter()
                        .position(|name_chirho| name_chirho == field_chirho.name_chirho.text_chirho())
                    {
                        positional_chirho[index_chirho] = Some(&field_chirho.pattern_chirho);
                    }
                }
                positional_chirho
            }
            // A constructor declared outside this module: its field order is
            // not known here, so the listed order is the only information
            // there is. An empty pattern still needs the constructor's arity.
            None if fields_chirho.is_empty() => {
                let arity_chirho = self
                    .con_arities_chirho
                    .get(con_name_chirho)
                    .copied()
                    .unwrap_or(0);
                vec![None; arity_chirho]
            }
            None => fields_chirho
                .iter()
                .map(|field_chirho| Some(&field_chirho.pattern_chirho))
                .collect(),
        };
        self.match_constructor_chirho(
            con_name_chirho,
            &positional_chirho,
            scrutinee_chirho,
            failure_chirho,
            success_chirho,
        )
    }

    /// `[p1, .., pn]` is `p1 : (.. : (pn : []))`, and is matched as exactly
    /// that, so its length is part of the match.
    fn match_list_chirho(
        &mut self,
        elements_chirho: &[PatChirho],
        scrutinee_chirho: CoreIdChirho,
        failure_chirho: &CoreExprChirho,
        success_chirho: &mut MatchSuccessChirho<'_>,
    ) -> CoreExprChirho {
        let Some((head_chirho, tail_chirho)) = elements_chirho.split_first() else {
            let matched_chirho = success_chirho(self);
            return self.case_on_chirho(
                scrutinee_chirho,
                AltConChirho::DataConChirho("[]".to_string()),
                vec![],
                matched_chirho,
                Some(failure_chirho.clone()),
            );
        };
        let head_binder_chirho = self.row_binder_chirho("$head");
        let tail_binder_chirho = self.row_binder_chirho("$tail");
        let tail_id_chirho = tail_binder_chirho.id_chirho;
        let matched_chirho = self.match_row_chirho(
            head_chirho,
            head_binder_chirho.id_chirho,
            failure_chirho,
            &mut |ctx_chirho| {
                ctx_chirho.match_list_chirho(
                    tail_chirho,
                    tail_id_chirho,
                    failure_chirho,
                    success_chirho,
                )
            },
        );
        self.case_on_chirho(
            scrutinee_chirho,
            AltConChirho::DataConChirho(":".to_string()),
            vec![head_binder_chirho, tail_binder_chirho],
            matched_chirho,
            Some(failure_chirho.clone()),
        )
    }

    /// A literal is a comparison. A string is compared with `eqStr#`, as the
    /// string case path does; anything else is a literal alternative, as a
    /// literal at the top of a case alternative already is.
    fn match_literal_chirho(
        &mut self,
        lit_chirho: &LitChirho,
        negated_chirho: bool,
        scrutinee_chirho: CoreIdChirho,
        failure_chirho: &CoreExprChirho,
        success_chirho: &mut MatchSuccessChirho<'_>,
    ) -> CoreExprChirho {
        if let (LitChirho::StringChirho(text_chirho, _, _), false) = (lit_chirho, negated_chirho) {
            let matched_chirho = success_chirho(self);
            let equal_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "eqStr#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(scrutinee_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(text_chirho.clone())),
                ],
            };
            let equal_binder_chirho = self.row_binder_chirho("$streq");
            return CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(equal_chirho),
                bind_chirho: equal_binder_chirho,
                result_ty_chirho: self.row_ty_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: matched_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: failure_chirho.clone(),
                    },
                ],
            };
        }
        let core_lit_chirho = match (self.desugar_lit_chirho(lit_chirho), negated_chirho) {
            (CoreLitChirho::IntChirho(value_chirho), true) => CoreLitChirho::IntChirho(-value_chirho),
            (CoreLitChirho::FloatChirho(value_chirho), true) => {
                CoreLitChirho::FloatChirho(-value_chirho)
            }
            (other_chirho, _) => other_chirho,
        };
        let matched_chirho = success_chirho(self);
        self.case_on_chirho(
            scrutinee_chirho,
            AltConChirho::LitConChirho(core_lit_chirho),
            vec![],
            matched_chirho,
            Some(failure_chirho.clone()),
        )
    }

    /// `case v of { con binders -> matched; _ -> failure }`, the failure
    /// alternative omitted when the match cannot fail.
    fn case_on_chirho(
        &mut self,
        scrutinee_chirho: CoreIdChirho,
        con_chirho: AltConChirho,
        binders_chirho: Vec<BinderChirho>,
        matched_chirho: CoreExprChirho,
        failure_chirho: Option<CoreExprChirho>,
    ) -> CoreExprChirho {
        let mut alts_chirho = vec![CoreAltChirho {
            con_chirho,
            binders_chirho,
            rhs_chirho: matched_chirho,
        }];
        if let Some(failure_chirho) = failure_chirho {
            alts_chirho.push(CoreAltChirho {
                con_chirho: AltConChirho::DefaultChirho,
                binders_chirho: vec![],
                rhs_chirho: failure_chirho,
            });
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(scrutinee_chirho)),
            bind_chirho: self.row_binder_chirho("$rowwild"),
            result_ty_chirho: self.row_ty_chirho(),
            alts_chirho,
        }
    }

    /// Evaluate `value_chirho` to weak head normal form, then `body_chirho`.
    pub(super) fn force_then_chirho(
        &mut self,
        value_chirho: CoreIdChirho,
        body_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(value_chirho)),
            bind_chirho: self.row_binder_chirho("$forced"),
            result_ty_chirho: self.row_ty_chirho(),
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DefaultChirho,
                binders_chirho: vec![],
                rhs_chirho: body_chirho,
            }],
        }
    }

    /// Wrap `body_chirho` in one `let` of `bindings_chirho`, recursive only when
    /// a binding refers to the group.
    pub(super) fn let_all_chirho(
        bindings_chirho: Vec<(BinderChirho, CoreExprChirho)>,
        body_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        if bindings_chirho.is_empty() {
            return body_chirho;
        }
        let rec_chirho = Self::is_recursive_binds_chirho(&bindings_chirho);
        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho: bindings_chirho,
            body_chirho: Box::new(body_chirho),
        }
    }

    pub(super) fn row_binder_chirho(&mut self, name_chirho: &str) -> BinderChirho {
        let ty_chirho = self.row_ty_chirho();
        self.fresh_binder_chirho(name_chirho, ty_chirho, SpanChirho::DUMMY_CHIRHO)
    }

    pub(super) fn row_ty_chirho(&self) -> TyChirho {
        TyChirho::VarChirho(TyVarChirho(self.next_id_chirho))
    }
}
