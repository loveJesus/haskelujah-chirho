// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One module-owned supply and substitution, transferred to term inference.

use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{MultChirho, TyChirho, TyVarChirho};
use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ElaborationOwnerChirho;
use haskelujah_span_chirho::SpanChirho;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ELABORATION_OWNER_CHIRHO: AtomicU64 = AtomicU64::new(1);
pub const TYPE_UNIVERSE_KEY_CHIRHO: &str = "%universe:type";
pub const CONSTRAINT_UNIVERSE_KEY_CHIRHO: &str = "%universe:constraint";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElaborationErrorChirho {
    pub message_chirho: String,
    pub span_chirho: SpanChirho,
}

impl std::fmt::Display for ElaborationErrorChirho {
    fn fmt(&self, formatter_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter_chirho.write_str(&self.message_chirho)
    }
}
impl std::error::Error for ElaborationErrorChirho {}

impl ElaborationErrorChirho {
    /// Preserve the kind phase's public diagnostic protocol at every frontend.
    pub fn into_diagnostic_chirho(self) -> haskelujah_diagnostics_chirho::DiagnosticChirho {
        haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
            haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300),
            self.message_chirho,
            self.span_chirho,
        )
    }
}

/// Bounds apply before each new copy/expansion. Exhaustion is terminal for
/// this state: callers publish neither a telescope nor an elaborated module.
#[derive(Debug)]
pub struct ElaborationStateChirho {
    pub owner_chirho: ElaborationOwnerChirho,
    pub next_variable_chirho: u32,
    pub substitution_chirho: SubstChirho,
    pub variable_kinds_chirho: HashMap<TyVarChirho, TyChirho>,
    rigid_variables_chirho: HashSet<TyVarChirho>,
    work_left_chirho: usize,
    output_left_chirho: usize,
    error_chirho: Option<ElaborationErrorChirho>,
}

impl ElaborationStateChirho {
    pub fn new_chirho() -> Result<Self, ElaborationErrorChirho> {
        let owner_chirho = NEXT_ELABORATION_OWNER_CHIRHO
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next_chirho| {
                next_chirho.checked_add(1)
            })
            .map_err(|_| ElaborationErrorChirho {
                message_chirho: "kind elaboration owner supply exhausted".to_owned(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            })?;
        Ok(Self {
            owner_chirho: ElaborationOwnerChirho(owner_chirho),
            next_variable_chirho: 0,
            substitution_chirho: SubstChirho::empty_chirho(),
            variable_kinds_chirho: HashMap::new(),
            rigid_variables_chirho: HashSet::new(),
            work_left_chirho: 1 << 25,
            output_left_chirho: 1 << 24,
            error_chirho: None,
        })
    }

    pub fn charge_chirho(
        &mut self,
        work_chirho: usize,
        output_chirho: usize,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        if let Some(error_chirho) = &self.error_chirho {
            return Err(error_chirho.clone());
        }
        let left_chirho = self
            .work_left_chirho
            .checked_sub(work_chirho)
            .zip(self.output_left_chirho.checked_sub(output_chirho));
        match left_chirho {
            Some((work_left_chirho, output_left_chirho)) => {
                self.work_left_chirho = work_left_chirho;
                self.output_left_chirho = output_left_chirho;
                Ok(())
            }
            None => {
                let error_chirho = ElaborationErrorChirho {
                    message_chirho: "kind elaboration work or output bound exhausted".to_owned(),
                    span_chirho,
                };
                self.error_chirho = Some(error_chirho.clone());
                Err(error_chirho)
            }
        }
    }

    pub fn name_chirho(
        &mut self,
        name_chirho: &str,
        span_chirho: SpanChirho,
    ) -> Result<String, ElaborationErrorChirho> {
        self.charge_chirho(
            name_chirho.len().saturating_add(1),
            name_chirho.len(),
            span_chirho,
        )?;
        Ok(name_chirho.to_owned())
    }

    pub fn fresh_chirho(
        &mut self,
        kind_chirho: Option<TyChirho>,
        span_chirho: SpanChirho,
    ) -> Result<TyVarChirho, ElaborationErrorChirho> {
        self.charge_chirho(1, 1, span_chirho)?;
        let variable_chirho = TyVarChirho(self.next_variable_chirho);
        self.next_variable_chirho =
            self.next_variable_chirho
                .checked_add(1)
                .ok_or_else(|| ElaborationErrorChirho {
                    message_chirho: "kind/type variable supply exhausted".to_owned(),
                    span_chirho,
                })?;
        if let Some(kind_chirho) = kind_chirho {
            self.variable_kinds_chirho
                .insert(variable_chirho, kind_chirho);
        }
        Ok(variable_chirho)
    }

    /// Measure borrowed terms before cloning. The recursive scan itself has a
    /// fixed stack bound and checked arithmetic; it allocates no child queue.
    pub fn copy_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let weight_chirho =
            borrowed_weight_chirho(ty_chirho, 0).ok_or_else(|| ElaborationErrorChirho {
                message_chirho: "kind/type term size bound exhausted".to_owned(),
                span_chirho,
            })?;
        self.charge_chirho(weight_chirho, weight_chirho, span_chirho)?;
        Ok(ty_chirho.clone())
    }

    pub fn variable_kind_chirho(
        &mut self,
        variable_chirho: TyVarChirho,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let weight_chirho = self
            .variable_kinds_chirho
            .get(&variable_chirho)
            .and_then(|kind_chirho| borrowed_weight_chirho(kind_chirho, 0))
            .ok_or_else(|| ElaborationErrorChirho {
                message_chirho: "scoped variable has no bounded producer kind".to_owned(),
                span_chirho,
            })?;
        self.charge_chirho(weight_chirho, weight_chirho, span_chirho)?;
        Ok(self.variable_kinds_chirho[&variable_chirho].clone())
    }

    pub fn type_universe_chirho() -> TyChirho {
        TyChirho::ConChirho(TYPE_UNIVERSE_KEY_CHIRHO.to_owned())
    }
    pub fn constraint_universe_chirho() -> TyChirho {
        TyChirho::ConChirho(CONSTRAINT_UNIVERSE_KEY_CHIRHO.to_owned())
    }

    /// Check an explicitly quantified kind universally. The same canonical
    /// variables remain in the successful header; no rigid marker escapes.
    pub fn check_rigid_chirho(
        &mut self,
        left_chirho: &TyChirho,
        right_chirho: &TyChirho,
        variables_chirho: &[TyVarChirho],
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.charge_chirho(variables_chirho.len(), variables_chirho.len(), span_chirho)?;
        let mut added_chirho = Vec::new();
        for variable_chirho in variables_chirho {
            if self.rigid_variables_chirho.insert(*variable_chirho) {
                added_chirho.push(*variable_chirho);
            }
        }
        let result_chirho = self.unify_chirho(left_chirho, right_chirho, span_chirho);
        for variable_chirho in added_chirho {
            self.rigid_variables_chirho.remove(&variable_chirho);
        }
        result_chirho
    }

    pub(super) fn open_rigid_scope_chirho(
        &mut self,
        variables_chirho: &[TyVarChirho],
        span_chirho: SpanChirho,
    ) -> Result<Vec<TyVarChirho>, ElaborationErrorChirho> {
        self.charge_chirho(variables_chirho.len(), variables_chirho.len(), span_chirho)?;
        Ok(variables_chirho
            .iter()
            .copied()
            .filter(|variable_chirho| self.rigid_variables_chirho.insert(*variable_chirho))
            .collect())
    }

    pub(super) fn close_rigid_scope_chirho(&mut self, added_chirho: Vec<TyVarChirho>) {
        for variable_chirho in added_chirho {
            self.rigid_variables_chirho.remove(&variable_chirho);
        }
    }

    /// Normalize under the one current substitution. Unlike the compatibility
    /// substitution helper, this follows chains and never turns an unsolved
    /// variable into a constructor chosen from its printed name.
    pub fn resolve_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        self.resolve_depth_chirho(ty_chirho, span_chirho, 0, &mut HashSet::new())
    }

    fn resolve_depth_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
        depth_chirho: usize,
        active_chirho: &mut HashSet<TyVarChirho>,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        self.charge_chirho(1, 1, span_chirho)?;
        if depth_chirho > 128 {
            return Err(ElaborationErrorChirho {
                message_chirho: "kind elaboration depth bound exhausted".to_owned(),
                span_chirho,
            });
        }
        Ok(match ty_chirho {
            TyChirho::VarChirho(variable_chirho) => {
                if !active_chirho.insert(*variable_chirho) {
                    return Err(ElaborationErrorChirho {
                        message_chirho: "cyclic kind substitution".to_owned(),
                        span_chirho,
                    });
                }
                let weight_chirho = match self.substitution_chirho.lookup_chirho(variable_chirho) {
                    Some(image_chirho) => borrowed_weight_chirho(image_chirho, 0).map(Some),
                    None => Some(None),
                };
                let image_chirho = match weight_chirho {
                    Some(Some(weight_chirho)) => {
                        self.charge_chirho(weight_chirho, weight_chirho, span_chirho)?;
                        self.substitution_chirho
                            .lookup_chirho(variable_chirho)
                            .cloned()
                    }
                    Some(None) => None,
                    None => {
                        return Err(ElaborationErrorChirho {
                            message_chirho: "kind substitution size bound exhausted".to_owned(),
                            span_chirho,
                        });
                    }
                };
                let result_chirho = if let Some(image_chirho) = image_chirho {
                    self.resolve_depth_chirho(
                        &image_chirho,
                        span_chirho,
                        depth_chirho + 1,
                        active_chirho,
                    )?
                } else {
                    TyChirho::VarChirho(*variable_chirho)
                };
                active_chirho.remove(variable_chirho);
                result_chirho
            }
            TyChirho::ConChirho(name_chirho) => {
                TyChirho::ConChirho(self.name_chirho(name_chirho, span_chirho)?)
            }
            TyChirho::ForallVarChirho(name_chirho) => {
                TyChirho::ForallVarChirho(self.name_chirho(name_chirho, span_chirho)?)
            }
            TyChirho::AppChirho(fun_chirho, arg_chirho) => TyChirho::AppChirho(
                Box::new(self.resolve_depth_chirho(
                    fun_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    active_chirho,
                )?),
                Box::new(self.resolve_depth_chirho(
                    arg_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    active_chirho,
                )?),
            ),
            TyChirho::FunChirho(arg_chirho, res_chirho, mult_chirho) => TyChirho::FunChirho(
                Box::new(self.resolve_depth_chirho(
                    arg_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    active_chirho,
                )?),
                Box::new(self.resolve_depth_chirho(
                    res_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    active_chirho,
                )?),
                *mult_chirho,
            ),
            TyChirho::ListChirho(element_chirho) => {
                TyChirho::ListChirho(Box::new(self.resolve_depth_chirho(
                    element_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    active_chirho,
                )?))
            }
            TyChirho::TupleChirho(elements_chirho) => {
                self.charge_chirho(elements_chirho.len(), elements_chirho.len(), span_chirho)?;
                TyChirho::TupleChirho(
                    elements_chirho
                        .iter()
                        .map(|element_chirho| {
                            self.resolve_depth_chirho(
                                element_chirho,
                                span_chirho,
                                depth_chirho + 1,
                                active_chirho,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                )
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::ForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.resolve_depth_chirho(
                    body_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    active_chirho,
                )?),
            },
            TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.resolve_depth_chirho(
                    body_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    active_chirho,
                )?),
            },
        })
    }

    /// Strict kind unification uses the shared type terms, with neither numeric
    /// widening nor qualified suffix equality. Those term-typing conveniences
    /// cannot identify two nominal kinds.
    pub fn unify_chirho(
        &mut self,
        left_chirho: &TyChirho,
        right_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let left_chirho = self.resolve_chirho(left_chirho, span_chirho)?;
        let right_chirho = self.resolve_chirho(right_chirho, span_chirho)?;
        let mut pending_chirho = vec![(left_chirho, right_chirho)];
        while let Some((left_chirho, right_chirho)) = pending_chirho.pop() {
            self.charge_chirho(1, 0, span_chirho)?;
            let left_chirho = self.resolve_chirho(&left_chirho, span_chirho)?;
            let right_chirho = self.resolve_chirho(&right_chirho, span_chirho)?;
            if left_chirho == right_chirho {
                continue;
            }
            match (left_chirho, right_chirho) {
                (TyChirho::VarChirho(left_chirho), TyChirho::VarChirho(right_chirho))
                    if self.rigid_variables_chirho.contains(&left_chirho)
                        && !self.rigid_variables_chirho.contains(&right_chirho) =>
                {
                    self.substitution_chirho
                        .insert_chirho(right_chirho, TyChirho::VarChirho(left_chirho));
                }
                (TyChirho::VarChirho(variable_chirho), ty_chirho)
                | (ty_chirho, TyChirho::VarChirho(variable_chirho)) => {
                    if self.rigid_variables_chirho.contains(&variable_chirho) {
                        return Err(ElaborationErrorChirho { message_chirho: "kind mismatch: an explicitly polymorphic kind cannot be specialized".to_owned(), span_chirho });
                    }
                    if occurs_chirho(variable_chirho, &ty_chirho) {
                        return Err(ElaborationErrorChirho {
                            message_chirho: "infinite kind in declaration telescope".to_owned(),
                            span_chirho,
                        });
                    }
                    self.substitution_chirho
                        .insert_chirho(variable_chirho, ty_chirho);
                }
                (
                    TyChirho::AppChirho(left_fun_chirho, left_arg_chirho),
                    TyChirho::AppChirho(right_fun_chirho, right_arg_chirho),
                ) => {
                    pending_chirho.push((*left_fun_chirho, *right_fun_chirho));
                    pending_chirho.push((*left_arg_chirho, *right_arg_chirho));
                }
                (
                    TyChirho::FunChirho(left_arg_chirho, left_res_chirho, left_mult_chirho),
                    TyChirho::FunChirho(right_arg_chirho, right_res_chirho, right_mult_chirho),
                ) if left_mult_chirho == right_mult_chirho => {
                    pending_chirho.push((*left_arg_chirho, *right_arg_chirho));
                    pending_chirho.push((*left_res_chirho, *right_res_chirho));
                }
                (TyChirho::ListChirho(left_chirho), TyChirho::ListChirho(right_chirho)) => {
                    pending_chirho.push((*left_chirho, *right_chirho))
                }
                (TyChirho::TupleChirho(left_chirho), TyChirho::TupleChirho(right_chirho))
                    if left_chirho.len() == right_chirho.len() =>
                {
                    self.charge_chirho(left_chirho.len(), left_chirho.len(), span_chirho)?;
                    pending_chirho.extend(left_chirho.into_iter().zip(right_chirho));
                }
                (left_chirho, right_chirho) => {
                    return Err(ElaborationErrorChirho {
                        message_chirho: format!(
                            "kind mismatch: expected `{left_chirho}`, found `{right_chirho}`"
                        ),
                        span_chirho,
                    });
                }
            }
        }
        Ok(())
    }

    pub fn arrow_chirho(argument_chirho: TyChirho, result_chirho: TyChirho) -> TyChirho {
        TyChirho::FunChirho(
            Box::new(argument_chirho),
            Box::new(result_chirho),
            MultChirho::ManyChirho,
        )
    }
}

fn borrowed_weight_chirho(ty_chirho: &TyChirho, depth_chirho: usize) -> Option<usize> {
    if depth_chirho > 128 {
        return None;
    }
    let child_chirho = |ty_chirho: &TyChirho| borrowed_weight_chirho(ty_chirho, depth_chirho + 1);
    let weight_chirho = match ty_chirho {
        TyChirho::VarChirho(_) => Some(1),
        TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
            name_chirho.len().checked_add(1)
        }
        TyChirho::AppChirho(fun_chirho, arg_chirho)
        | TyChirho::FunChirho(fun_chirho, arg_chirho, _) => child_chirho(fun_chirho)?
            .checked_add(child_chirho(arg_chirho)?)?
            .checked_add(1),
        TyChirho::ListChirho(element_chirho) => child_chirho(element_chirho)?.checked_add(1),
        TyChirho::TupleChirho(elements_chirho) => {
            elements_chirho
                .iter()
                .try_fold(1usize, |weight_chirho, element_chirho| {
                    weight_chirho.checked_add(child_chirho(element_chirho)?)
                })
        }
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => child_chirho(body_chirho)?
            .checked_add(vars_chirho.len())?
            .checked_add(1),
    }?;
    (weight_chirho <= 1 << 16).then_some(weight_chirho)
}

fn occurs_chirho(variable_chirho: TyVarChirho, ty_chirho: &TyChirho) -> bool {
    match ty_chirho {
        TyChirho::VarChirho(found_chirho) => variable_chirho == *found_chirho,
        TyChirho::AppChirho(fun_chirho, arg_chirho)
        | TyChirho::FunChirho(fun_chirho, arg_chirho, _) => {
            occurs_chirho(variable_chirho, fun_chirho) || occurs_chirho(variable_chirho, arg_chirho)
        }
        TyChirho::ListChirho(element_chirho) => occurs_chirho(variable_chirho, element_chirho),
        TyChirho::TupleChirho(elements_chirho) => elements_chirho
            .iter()
            .any(|element_chirho| occurs_chirho(variable_chirho, element_chirho)),
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => !vars_chirho.contains(&variable_chirho) && occurs_chirho(variable_chirho, body_chirho),
        TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => false,
    }
}
