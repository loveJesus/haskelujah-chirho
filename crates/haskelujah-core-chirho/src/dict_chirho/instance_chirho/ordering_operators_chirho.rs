// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `<`, `<=`, `>` and `>=` at the key the checker proved (row 610).
//!
//! A written infix or section Ord operator reaches the pass as its own occurrence, as a
//! first-class `(<)` always did (`desugar_chirho/operator_evidence_chirho.rs`). At a
//! concrete key its answer is:
//! - the program's own operator row, where its instance defines the operator;
//! - at Int, Char and Bool, the Int primop desugaring chose, which is exact there;
//! - at Double, row 547's IEEE primop: a NaN on either side gives False, and -0.0 equals 0.0;
//! - at a key with a body-backed `Ord.compare` row, that row read by the operator's
//!   Ordering test, as row 551's compare route reads it.
//!
//! A written operator applied to its operands takes its answer in place, before any walker
//! reads its binding: the typed walkers of higher-order arguments, such as `filter (< "b")`'s
//! section, never see a whole application spine. A first-class or
//! prefix use takes the same answer through a generated `$prim_Ord_<op>_<key>` row, which
//! its proof names. Without an answer (the binding's own dictionary key, no evidence, or a
//! key whose compare row is missing or a stub), a written operator takes back the primop
//! desugaring once gave it, and a first-class one stays loud as before. A program's own
//! operator keeps its binding.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::borrow::Cow;
use std::collections::BTreeSet;

use haskelujah_typing_chirho::infer_chirho::is_own_dictionary_key_chirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::DictPassCtxChirho;
use super::structured_equalities_chirho::INT_SHAPED_KEYS_CHIRHO;
use crate::dict_chirho::prelude_chirho::ordering_tests_chirho::ORDERING_TESTS_CHIRHO;
use crate::dict_chirho::rewrite_chirho::double_primops_chirho::double_counterpart_chirho;
use crate::expr_chirho::{CoreBindingChirho, CoreExprChirho, CoreIdChirho, InlineAnnotationChirho};

/// Each Ord operator and the Int primop desugaring gave it.
const ORDERING_OPERATORS_CHIRHO: [(&str, &str); 4] =
    [("<", "<#"), ("<=", "<=#"), (">", ">#"), (">=", ">=#")];

/// An Ord operator's answer at one proof key.
enum OrderingAnswerChirho {
    /// A primop exact at the key.
    PrimopChirho(&'static str),
    /// A row over both operands: the program's own operator row, or the key's compare row
    /// read by the operator's Ordering test.
    RowChirho {
        row_chirho: CoreIdChirho,
        test_chirho: Option<CoreIdChirho>,
    },
}

impl OrderingAnswerChirho {
    fn applied_chirho(
        &self,
        left_chirho: CoreExprChirho,
        right_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        match self {
            Self::PrimopChirho(primop_chirho) => CoreExprChirho::PrimOpChirho {
                name_chirho: (*primop_chirho).to_string(),
                args_chirho: vec![left_chirho, right_chirho],
            },
            Self::RowChirho {
                row_chirho,
                test_chirho,
            } => {
                let applied_chirho =
                    DictPassCtxChirho::apply_two_chirho(*row_chirho, left_chirho, right_chirho);
                match test_chirho {
                    Some(test_chirho) => CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(*test_chirho)),
                        arg_chirho: Box::new(applied_chirho),
                    },
                    None => applied_chirho,
                }
            }
        }
    }
}

impl DictPassCtxChirho {
    /// The Ord operator an occurrence names, with its Int primop.
    fn ordering_operator_chirho(
        &self,
        id_chirho: CoreIdChirho,
    ) -> Option<(&'static str, &'static str)> {
        let (name_chirho, _canonical_id_chirho) =
            self.method_occurrence_canon_chirho.get(&id_chirho)?;
        let base_chirho = name_chirho.rsplit('.').next().unwrap_or(name_chirho);
        ORDERING_OPERATORS_CHIRHO
            .into_iter()
            .find(|(operator_chirho, _)| *operator_chirho == base_chirho)
    }

    /// The `Ord` key the checker proved for an occurrence, unless it is the binding's own
    /// dictionary.
    fn proved_ordering_key_chirho(&self, id_chirho: CoreIdChirho) -> Option<&str> {
        let (class_name_chirho, key_chirho) = self.occurrence_evidence_chirho.get(&id_chirho)?;
        (class_name_chirho == "Ord" && !is_own_dictionary_key_chirho(key_chirho))
            .then_some(key_chirho.as_str())
    }

    /// The operator's answer at `key_chirho`, or None.
    fn ordering_answer_chirho(
        &self,
        (operator_chirho, primop_chirho): (&str, &'static str),
        key_chirho: &str,
    ) -> Option<OrderingAnswerChirho> {
        if INT_SHAPED_KEYS_CHIRHO.contains(&key_chirho) {
            return Some(OrderingAnswerChirho::PrimopChirho(primop_chirho));
        }
        if key_chirho == "Double" {
            return double_counterpart_chirho(primop_chirho)
                .map(OrderingAnswerChirho::PrimopChirho);
        }
        if let Some(row_chirho) = self
            .body_backed_names_chirho
            .get(&format!("$prim_Ord_{operator_chirho}_{key_chirho}"))
            .filter(|row_chirho| !self.context_leading_rows_chirho.contains_key(row_chirho))
        {
            return Some(OrderingAnswerChirho::RowChirho {
                row_chirho: *row_chirho,
                test_chirho: None,
            });
        }
        let compare_chirho = self.existing_keyed_row_chirho("$prim_Ord_compare_", key_chirho)?;
        let (_, test_name_chirho, ..) = ORDERING_TESTS_CHIRHO
            .iter()
            .find(|(test_primop_chirho, ..)| *test_primop_chirho == primop_chirho)?;
        let test_chirho = self.lookup_body_backed_name_id_chirho(test_name_chirho)?;
        Some(OrderingAnswerChirho::RowChirho {
            row_chirho: compare_chirho,
            test_chirho: Some(test_chirho),
        })
    }

    /// A written infix or section Ord operator applied to its operands: its answer at the
    /// key its proof names, else the primop desugaring once gave it. None for any other
    /// head, and for a program's own operator, whose binding stays.
    fn written_ordering_operator_chirho(
        &self,
        head_id_chirho: CoreIdChirho,
        left_chirho: &CoreExprChirho,
        right_chirho: &CoreExprChirho,
    ) -> Option<CoreExprChirho> {
        if !self.operator_occurrences_chirho.contains(&head_id_chirho)
            || self.names_programs_own_binder_chirho(head_id_chirho)
        {
            return None;
        }
        let operator_chirho = self.ordering_operator_chirho(head_id_chirho)?;
        let answer_chirho = self
            .proved_ordering_key_chirho(head_id_chirho)
            .and_then(|key_chirho| self.ordering_answer_chirho(operator_chirho, key_chirho))
            .unwrap_or(OrderingAnswerChirho::PrimopChirho(operator_chirho.1));
        Some(answer_chirho.applied_chirho(left_chirho.clone(), right_chirho.clone()))
    }

    /// The module's bindings with each written Ord operator applied to its operands
    /// replaced by its answer; borrowed as they are when the module writes none.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn written_orderings_resolved_chirho<'a>(
        &self,
        bindings_chirho: &'a [CoreBindingChirho],
    ) -> Cow<'a, [CoreBindingChirho]> {
        if self.operator_occurrences_chirho.is_empty() {
            return Cow::Borrowed(bindings_chirho);
        }
        let mut resolved_chirho = bindings_chirho.to_vec();
        for binding_chirho in &mut resolved_chirho {
            self.resolve_written_orderings_chirho(&mut binding_chirho.rhs_chirho);
        }
        Cow::Owned(resolved_chirho)
    }

    /// Replace, throughout `expr_chirho`, each written Ord operator applied to its operands
    /// by its answer, the operands included.
    fn resolve_written_orderings_chirho(&self, expr_chirho: &mut CoreExprChirho) {
        let replacement_chirho = match &*expr_chirho {
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho: right_chirho,
            } => match fun_chirho.as_ref() {
                CoreExprChirho::AppChirho {
                    fun_chirho: head_chirho,
                    arg_chirho: left_chirho,
                } => match head_chirho.as_ref() {
                    CoreExprChirho::VarChirho(head_id_chirho) => self
                        .written_ordering_operator_chirho(
                            *head_id_chirho,
                            left_chirho,
                            right_chirho,
                        ),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        };
        if let Some(replacement_chirho) = replacement_chirho {
            *expr_chirho = replacement_chirho;
        }
        match expr_chirho {
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                self.resolve_written_orderings_chirho(fun_chirho);
                self.resolve_written_orderings_chirho(arg_chirho);
            }
            CoreExprChirho::LamChirho { body_chirho, .. }
            | CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                self.resolve_written_orderings_chirho(body_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.resolve_written_orderings_chirho(inner_chirho),
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (_binder_chirho, rhs_chirho) in binds_chirho.iter_mut() {
                    self.resolve_written_orderings_chirho(rhs_chirho);
                }
                self.resolve_written_orderings_chirho(body_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                self.resolve_written_orderings_chirho(scrutinee_chirho);
                for alt_chirho in alts_chirho.iter_mut() {
                    self.resolve_written_orderings_chirho(&mut alt_chirho.rhs_chirho);
                }
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho.iter_mut() {
                    self.resolve_written_orderings_chirho(arg_chirho);
                }
            }
        }
    }

    /// A `$prim_Ord_<op>_<key>` row for each key the checker proved at a first-class or
    /// prefix Ord operator, holding the answer a written one takes in place. A key without
    /// an answer gets no row, so such a use stays loud.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn generate_ordering_operator_rows_chirho(&mut self) {
        let wanted_chirho = self
            .occurrence_evidence_chirho
            .keys()
            .filter(|id_chirho| {
                !self.operator_occurrences_chirho.contains(id_chirho)
                    && !self.names_programs_own_binder_chirho(**id_chirho)
            })
            .filter_map(|id_chirho| {
                let operator_chirho = self.ordering_operator_chirho(*id_chirho)?;
                let key_chirho = self.proved_ordering_key_chirho(*id_chirho)?;
                Some((operator_chirho, key_chirho.to_string()))
            })
            .collect::<BTreeSet<_>>();
        for (operator_chirho, key_chirho) in wanted_chirho {
            let row_name_chirho = format!("$prim_Ord_{}_{key_chirho}", operator_chirho.0);
            if self
                .lookup_body_backed_name_id_chirho(&row_name_chirho)
                .is_some()
            {
                continue;
            }
            let Some(answer_chirho) = self.ordering_answer_chirho(operator_chirho, &key_chirho)
            else {
                continue;
            };
            let x_chirho = self.fresh_binder_chirho("x", TyChirho::int_chirho());
            let y_chirho = self.fresh_binder_chirho("y", TyChirho::int_chirho());
            let body_chirho = answer_chirho.applied_chirho(
                Self::var_expr_chirho(&x_chirho),
                Self::var_expr_chirho(&y_chirho),
            );
            let row_binder_chirho =
                self.conditional_dict_binder_chirho(&row_name_chirho, TyChirho::int_chirho());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: row_binder_chirho,
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: y_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }
}
