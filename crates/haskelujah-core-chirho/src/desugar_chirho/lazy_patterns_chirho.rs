// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A lazy pattern binds through selectors, and scrutinises nothing.
//!
//! `~(Just n) <- m` and `let (Just n) = e` must not look at the scrutinee at the
//! binding site at all. Each variable becomes its own thunk that selects its own
//! field when demanded, so:
//!
//! - an UNUSED variable forces nothing, and `let (Just n) = undefined` is fine;
//! - DEMANDING a variable selects its field;
//! - a MISMATCHING constructor fails when a variable is demanded, not before.
//!
//! The obvious repair — deriving the alternative from the pattern inside the `~`
//! — gets the field right and breaks all three of those, because the generated
//! case scrutinises eagerly. gpt_chirho named that trap directly (#25099), and it
//! is why this builds bindings rather than a match.
//!
//! The `let`-as-thunk technique is the one the view-pattern path already uses
//! here, for the same reason: the STG lowerer does not bind case binders, so a
//! `let` is what makes a field lazy.
//! workflow: monadic-dispatch-chirho

use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;

use super::DesugarCtxChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use crate::expr_chirho::{BinderChirho, CoreAltChirho, CoreExprChirho};

impl DesugarCtxChirho {
    /// One binding per variable the pattern binds, each a selector thunk over
    /// `scrutinee_chirho`. An empty result means the pattern binds nothing, and
    /// the caller needs no bindings at all.
    ///
    /// `None` means this pattern shape is not carried by the selector path yet
    /// (see `sub_pats_chirho`), and the caller must keep its existing behaviour
    /// rather than silently binding the wrong thing.
    pub(super) fn lazy_selector_bindings_chirho(
        &mut self,
        pat_chirho: &PatChirho,
        scrutinee_chirho: &CoreExprChirho,
    ) -> Option<Vec<(BinderChirho, CoreExprChirho)>> {
        let mut bindings_chirho = Vec::new();
        self.collect_selectors_chirho(pat_chirho, scrutinee_chirho, &mut bindings_chirho)?;
        Some(bindings_chirho)
    }

    /// The scrutinee binder and one selector binding per variable, ready for a
    /// caller to wrap around a body.
    ///
    /// ORDER MATTERS, and getting it wrong is silent: building these BINDS each
    /// variable in scope, and the body must be desugared AFTERWARDS so its
    /// references resolve to these thunks. The normal pattern path takes its
    /// variables from case alternative binders, so `prebind_all_pat_vars_chirho`
    /// deliberately does not bind an immediate variable child — this path has no
    /// alternative binders, so it must bind them itself. Desugaring the body
    /// first leaves it pointing at an id nothing binds, which shows up as
    /// "missing STG binding" at run time rather than as a compile error.
    ///
    /// `None` when this pattern shape is not carried yet, so the caller keeps its
    /// existing path rather than binding the wrong field.
    pub(super) fn lazy_bind_prepare_chirho(
        &mut self,
        pat_chirho: &PatChirho,
    ) -> Option<(BinderChirho, Vec<(BinderChirho, CoreExprChirho)>)> {
        let PatChirho::LazyChirho { .. } = pat_chirho else {
            return None;
        };
        let scrutinee_binder_chirho = self.fresh_binder_chirho(
            "$lazybind",
            self.fresh_selector_ty_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        );
        let scrutinee_chirho = CoreExprChirho::VarChirho(scrutinee_binder_chirho.id_chirho);
        let binds_chirho = self.lazy_selector_bindings_chirho(pat_chirho, &scrutinee_chirho)?;
        Some((scrutinee_binder_chirho, binds_chirho))
    }

    /// Wrap `body_chirho` in the selector bindings and the scrutinee lambda.
    pub(super) fn lazy_bind_lambda_chirho(
        scrutinee_binder_chirho: BinderChirho,
        binds_chirho: Vec<(BinderChirho, CoreExprChirho)>,
        body_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let body_chirho = if binds_chirho.is_empty() {
            body_chirho
        } else {
            CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho,
                body_chirho: Box::new(body_chirho),
            }
        };
        CoreExprChirho::LamChirho {
            binder_chirho: scrutinee_binder_chirho,
            body_chirho: Box::new(body_chirho),
        }
    }

    fn collect_selectors_chirho(
        &mut self,
        pat_chirho: &PatChirho,
        selector_chirho: &CoreExprChirho,
        bindings_chirho: &mut Vec<(BinderChirho, CoreExprChirho)>,
    ) -> Option<()> {
        match pat_chirho {
            // The variable IS the thing selected so far.
            PatChirho::VarChirho(name_chirho) => {
                let binder_chirho = self.selector_binder_chirho(name_chirho.text_chirho());
                bindings_chirho.push((binder_chirho, selector_chirho.clone()));
                Some(())
            }
            // Binds nothing, demands nothing.
            PatChirho::WildcardChirho(_) => Some(()),
            // A literal inside a lazy pattern binds nothing. Its comparison is
            // never reached, because nothing selects through it — which is also
            // GHC's behaviour: `~(Just 3) <- m` with nothing demanded never fails.
            PatChirho::LitChirho(_) | PatChirho::NegChirho { .. } => Some(()),
            // Transparent: a lazy pattern inside a lazy pattern is still one
            // lazy pattern, and a bang inside one does not make the binding
            // site strict.
            PatChirho::ParenChirho { inner_chirho, .. }
            | PatChirho::BangChirho { inner_chirho, .. }
            | PatChirho::LazyChirho { inner_chirho, .. } => {
                self.collect_selectors_chirho(inner_chirho, selector_chirho, bindings_chirho)
            }
            PatChirho::TypeAnnotChirho { pat_chirho, .. } => {
                self.collect_selectors_chirho(pat_chirho, selector_chirho, bindings_chirho)
            }
            // `v@p` binds v to what has been selected, and then p through it.
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho,
                ..
            } => {
                let binder_chirho = self.selector_binder_chirho(name_chirho.text_chirho());
                bindings_chirho.push((binder_chirho, selector_chirho.clone()));
                self.collect_selectors_chirho(pattern_chirho, selector_chirho, bindings_chirho)
            }
            // A pattern with fields: select each field that binds something.
            _ => {
                let sub_pats_chirho = sub_pats_chirho(pat_chirho)?;
                let arity_chirho = sub_pats_chirho.len();
                for (index_chirho, sub_pat_chirho) in sub_pats_chirho.into_iter().enumerate() {
                    if !binds_anything_chirho(sub_pat_chirho) {
                        continue;
                    }
                    let field_chirho = self.field_selector_chirho(
                        pat_chirho,
                        selector_chirho,
                        index_chirho,
                        arity_chirho,
                    );
                    self.collect_selectors_chirho(sub_pat_chirho, &field_chirho, bindings_chirho)?;
                }
                Some(())
            }
        }
    }

    /// `case <selector> of { <con> f0 .. fn -> f_index }` — the one field, and
    /// nothing else forced.
    fn field_selector_chirho(
        &mut self,
        pat_chirho: &PatChirho,
        selector_chirho: &CoreExprChirho,
        index_chirho: usize,
        arity_chirho: usize,
    ) -> CoreExprChirho {
        let con_chirho = self.pat_to_alt_con_chirho(pat_chirho);
        let field_binders_chirho: Vec<BinderChirho> = (0..arity_chirho)
            .map(|position_chirho| self.selector_binder_chirho(&format!("$sel{position_chirho}")))
            .collect();
        let chosen_chirho = field_binders_chirho[index_chirho].id_chirho;
        let wild_chirho = self.selector_binder_chirho("$selwild");
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(selector_chirho.clone()),
            bind_chirho: wild_chirho,
            result_ty_chirho: self.fresh_selector_ty_chirho(),
            alts_chirho: vec![CoreAltChirho {
                con_chirho,
                binders_chirho: field_binders_chirho,
                rhs_chirho: CoreExprChirho::VarChirho(chosen_chirho),
            }],
        }
    }

    /// A binder for a selector. A variable the enclosing scope already pre-bound
    /// keeps that id, so references in the body resolve to this thunk.
    fn selector_binder_chirho(&mut self, name_chirho: &str) -> BinderChirho {
        if let Some(id_chirho) = self.lookup_scope_chirho(name_chirho) {
            return BinderChirho {
                id_chirho,
                name_chirho: name_chirho.to_string(),
                ty_chirho: self.fresh_selector_ty_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
        }
        let fresh_chirho = self.fresh_binder_chirho(
            name_chirho,
            self.fresh_selector_ty_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        );
        self.bind_in_scope_chirho(name_chirho, fresh_chirho.id_chirho);
        fresh_chirho
    }

    fn fresh_selector_ty_chirho(&self) -> TyChirho {
        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
            self.next_id_chirho,
        ))
    }
}

/// The sub-patterns a pattern selects through, in FIELD order.
///
/// `None` for the shapes this path does not carry yet, so the caller keeps its
/// existing behaviour instead of binding the wrong field:
/// - a RECORD pattern may list its fields in any order, so a field's index is
///   the constructor's, not the listed one, and that mapping is not here;
/// - a LIST pattern is a nested cons chain rather than one constructor;
/// - a VIEW pattern applies a function first, which the view path already does.
fn sub_pats_chirho(pat_chirho: &PatChirho) -> Option<Vec<&PatChirho>> {
    match pat_chirho {
        PatChirho::ConChirho { args_chirho, .. } => Some(args_chirho.iter().collect()),
        PatChirho::TupleChirho {
            elements_chirho, ..
        } => Some(elements_chirho.iter().collect()),
        PatChirho::InfixConChirho {
            left_chirho,
            right_chirho,
            ..
        } => Some(vec![left_chirho, right_chirho]),
        PatChirho::RecordChirho { .. }
        | PatChirho::ListChirho { .. }
        | PatChirho::ViewChirho { .. } => None,
        // Every remaining shape is handled before this point.
        _ => None,
    }
}

/// Whether any variable is bound anywhere inside this pattern. A field that
/// binds nothing needs no selector, so nothing is built that could force it.
fn binds_anything_chirho(pat_chirho: &PatChirho) -> bool {
    match pat_chirho {
        PatChirho::VarChirho(_) | PatChirho::AsChirho { .. } => true,
        PatChirho::WildcardChirho(_) | PatChirho::LitChirho(_) | PatChirho::NegChirho { .. } => {
            false
        }
        PatChirho::ParenChirho { inner_chirho, .. }
        | PatChirho::BangChirho { inner_chirho, .. }
        | PatChirho::LazyChirho { inner_chirho, .. } => binds_anything_chirho(inner_chirho),
        PatChirho::TypeAnnotChirho { pat_chirho, .. } => binds_anything_chirho(pat_chirho),
        PatChirho::ConChirho { args_chirho, .. } => {
            args_chirho.iter().any(binds_anything_chirho)
        }
        PatChirho::TupleChirho {
            elements_chirho, ..
        }
        | PatChirho::ListChirho {
            elements_chirho, ..
        } => elements_chirho.iter().any(binds_anything_chirho),
        PatChirho::InfixConChirho {
            left_chirho,
            right_chirho,
            ..
        } => binds_anything_chirho(left_chirho) || binds_anything_chirho(right_chirho),
        PatChirho::RecordChirho { fields_chirho, .. } => fields_chirho
            .iter()
            .any(|field_chirho| binds_anything_chirho(&field_chirho.pattern_chirho)),
        PatChirho::ViewChirho { pat_chirho, .. } => binds_anything_chirho(pat_chirho),
    }
}
