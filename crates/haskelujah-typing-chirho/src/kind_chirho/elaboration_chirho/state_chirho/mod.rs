// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One module-owned supply and substitution, transferred to term inference.

use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{MultChirho, TyChirho, TyVarChirho};
use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ElaborationOwnerChirho;
use haskelujah_span_chirho::SpanChirho;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};

mod continuation_chirho;

static NEXT_ELABORATION_OWNER_CHIRHO: AtomicU64 = AtomicU64::new(1);
pub const TYPE_UNIVERSE_KEY_CHIRHO: &str = "%universe:type";
pub const CONSTRAINT_UNIVERSE_KEY_CHIRHO: &str = "%universe:constraint";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElaborationErrorChirho {
    /// Only a real unification mismatch owns these complete resolved terms.
    /// Structural, ownership and resource failures never acquire this payload.
    pub(crate) mismatch_chirho: Option<std::sync::Arc<(TyChirho, TyChirho)>>,
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
struct KindScopeGuardChirho {
    cutoff_chirho: u32,
    variables_chirho: HashSet<TyVarChirho>,
    protected_chirho: HashSet<TyVarChirho>,
}

/// Immutable authority from the actual module/options and naming handoff.
/// Late required arguments reuse it; a legacy context has no such authority.
#[derive(Debug)]
pub(crate) struct ElaborationPolicyChirho {
    pub star_is_type_chirho: bool,
    pub poly_kinds_chirho: bool,
    pub cusks_chirho: bool,
    pub unlifted_newtypes_chirho: bool,
    pub scoped_type_variables_chirho: bool,
    pub builtin_classes_chirho:
        std::sync::Arc<haskelujah_ast_chirho::name_chirho::BuiltinClassScopeChirho>,
}

#[derive(Debug)]
pub struct ElaborationStateChirho {
    /// Only the real module elaborator mints an owner. An ordinary legacy
    /// inference context has none and cannot stamp a positive source proof.
    pub owner_chirho: Option<ElaborationOwnerChirho>,
    pub next_variable_chirho: u32,
    pub substitution_chirho: SubstChirho,
    pub variable_kinds_chirho: HashMap<TyVarChirho, crate::ty_chirho::QuantifiedVariableChirho>,
    pub(crate) source_policy_chirho: Option<std::sync::Arc<ElaborationPolicyChirho>>,
    rigid_variables_chirho: HashSet<TyVarChirho>,
    kind_scope_guards_chirho: Vec<KindScopeGuardChirho>,
    work_left_chirho: usize,
    output_left_chirho: usize,
    error_chirho: Option<ElaborationErrorChirho>,
    inference_links_chirho: Option<Vec<TyVarChirho>>,
    /// An early body pass stamps syntax but cannot specialize its universals.
    /// Its scoped obligations are checked later under the real term evidence.
    pub(super) body_scope_chirho: bool,
}

impl ElaborationStateChirho {
    /// The original module allowance and its terminal failure survive every
    /// continuation, including a consumer that needs no further kind work.
    pub(crate) fn failure_chirho(&self) -> Option<&ElaborationErrorChirho> {
        self.error_chirho.as_ref()
    }

    pub(super) fn resource_failure_chirho(
        &mut self,
        message_chirho: &str,
        span_chirho: SpanChirho,
    ) -> ElaborationErrorChirho {
        self.error_chirho
            .get_or_insert_with(|| ElaborationErrorChirho {
                mismatch_chirho: None,
                message_chirho: message_chirho.to_owned(),
                span_chirho,
            })
            .clone()
    }

    pub(super) fn owner_id_chirho(
        &self,
        span_chirho: SpanChirho,
    ) -> Result<ElaborationOwnerChirho, ElaborationErrorChirho> {
        self.owner_chirho.ok_or_else(|| ElaborationErrorChirho {
            mismatch_chirho: None,
            message_chirho: "unowned inference state cannot mint a source kind variable".into(),
            span_chirho,
        })
    }

    pub(crate) fn unowned_chirho() -> Self {
        Self {
            owner_chirho: None,
            next_variable_chirho: 0,
            substitution_chirho: SubstChirho::empty_chirho(),
            variable_kinds_chirho: HashMap::new(),
            source_policy_chirho: None,
            rigid_variables_chirho: HashSet::new(),
            kind_scope_guards_chirho: Vec::new(),
            work_left_chirho: 1 << 25,
            output_left_chirho: 1 << 24,
            error_chirho: None,
            inference_links_chirho: None,
            body_scope_chirho: false,
        }
    }

    pub fn new_chirho() -> Result<Self, ElaborationErrorChirho> {
        let owner_chirho = NEXT_ELABORATION_OWNER_CHIRHO
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next_chirho| {
                next_chirho.checked_add(1)
            })
            .map_err(|_| ElaborationErrorChirho {
                mismatch_chirho: None,
                message_chirho: "kind elaboration owner supply exhausted".to_owned(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            })?;
        let mut state_chirho = Self::unowned_chirho();
        state_chirho.owner_chirho = Some(ElaborationOwnerChirho(owner_chirho));
        Ok(state_chirho)
    }

    /// Witnessed source variables keep their specificity when unification
    /// chooses another representative. Generated metas acquire none by spelling.
    pub(super) fn note_source_variable_chirho(
        &mut self,
        variable_chirho: TyVarChirho,
        specificity_chirho: haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho,
        explicit_chirho: bool,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.charge_chirho(1, 1, span_chirho)?;
        let owner_chirho = self.owner_chirho.ok_or_else(|| ElaborationErrorChirho {
            mismatch_chirho: None,
            message_chirho: "unowned inference state cannot mint a source kind variable".into(),
            span_chirho,
        })?;
        let record_chirho = self
            .variable_kinds_chirho
            .entry(variable_chirho)
            .or_insert_with(|| {
                crate::ty_chirho::QuantifiedVariableChirho::unknown_chirho(variable_chirho)
            });
        record_chirho.source_chirho.get_or_insert(haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho::InferenceChirho {
                owner_chirho, variable_chirho });
        if explicit_chirho || record_chirho.specificity_chirho.is_none() {
            record_chirho.specificity_chirho = Some(specificity_chirho);
        }
        Ok(())
    }

    pub(super) fn source_specificity_chirho(
        &self,
        variable_chirho: TyVarChirho,
    ) -> Option<haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho> {
        self.variable_kinds_chirho
            .get(&variable_chirho)
            .and_then(|record_chirho| record_chirho.specificity_chirho)
    }

    fn link_source_variable_chirho(
        &mut self,
        source_chirho: TyVarChirho,
        target_chirho: TyVarChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        if let Some(specificity_chirho) = self.source_specificity_chirho(source_chirho) {
            let source_origin_chirho = self
                .variable_kinds_chirho
                .get(&source_chirho)
                .and_then(|record_chirho| record_chirho.source_chirho.clone());
            let target_had_origin_chirho = self
                .variable_kinds_chirho
                .get(&target_chirho)
                .is_some_and(|record_chirho| record_chirho.source_chirho.is_some());
            self.note_source_variable_chirho(
                target_chirho,
                specificity_chirho,
                false,
                span_chirho,
            )?;
            if !target_had_origin_chirho
                && let Some(record_chirho) = self.variable_kinds_chirho.get_mut(&target_chirho)
            {
                record_chirho.source_chirho = source_origin_chirho;
            }
        }
        Ok(())
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
                    mismatch_chirho: None,
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
        self.next_variable_chirho = self.next_variable_chirho.checked_add(1).ok_or_else(|| {
            self.resource_failure_chirho("kind/type variable supply exhausted", span_chirho)
        })?;
        if let Some(kind_chirho) = kind_chirho {
            self.set_variable_kind_chirho(variable_chirho, kind_chirho);
        }
        Ok(variable_chirho)
    }

    pub(super) fn set_variable_kind_chirho(
        &mut self,
        variable_chirho: TyVarChirho,
        kind_chirho: TyChirho,
    ) {
        self.variable_kinds_chirho
            .entry(variable_chirho)
            .or_insert_with(|| {
                crate::ty_chirho::QuantifiedVariableChirho::unknown_chirho(variable_chirho)
            })
            .kind_chirho = Some(std::sync::Arc::new(kind_chirho));
    }

    /// Measure borrowed terms before cloning. The recursive scan itself has a
    /// fixed stack bound and checked arithmetic; it allocates no child queue.
    pub fn copy_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        let weight_chirho = borrowed_weight_chirho(ty_chirho, 0).ok_or_else(|| {
            self.resource_failure_chirho("kind/type term size bound exhausted", span_chirho)
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
            .and_then(|record_chirho| record_chirho.kind_chirho.as_ref())
            .and_then(|kind_chirho| borrowed_weight_chirho(kind_chirho, 0))
            .ok_or_else(|| ElaborationErrorChirho {
                mismatch_chirho: None,
                message_chirho: "scoped variable has no bounded producer kind".to_owned(),
                span_chirho,
            })?;
        self.charge_chirho(weight_chirho, weight_chirho, span_chirho)?;
        Ok(self.variable_kinds_chirho[&variable_chirho]
            .kind_chirho
            .as_ref()
            .expect("kind was admitted above")
            .as_ref()
            .clone())
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

    pub(super) fn open_kind_guard_chirho(
        &mut self,
        variables_chirho: &[TyVarChirho],
        cutoff_chirho: u32,
        span_chirho: SpanChirho,
    ) -> Result<usize, ElaborationErrorChirho> {
        self.charge_chirho(
            variables_chirho.len() + 1,
            variables_chirho.len() + 1,
            span_chirho,
        )?;
        let mark_chirho = self.kind_scope_guards_chirho.len();
        if mark_chirho >= 128 {
            return Err(ElaborationErrorChirho {
                mismatch_chirho: None,
                message_chirho: "universal kind scope depth bound exhausted".to_owned(),
                span_chirho,
            });
        }
        self.kind_scope_guards_chirho.push(KindScopeGuardChirho {
            cutoff_chirho,
            variables_chirho: variables_chirho.iter().copied().collect(),
            protected_chirho: HashSet::new(),
        });
        Ok(mark_chirho)
    }

    pub(super) fn close_kind_guard_chirho(&mut self, mark_chirho: usize) {
        self.kind_scope_guards_chirho.truncate(mark_chirho);
    }

    fn guard_kind_image_chirho(
        &mut self,
        variable_chirho: TyVarChirho,
        image_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        if self.kind_scope_guards_chirho.is_empty() {
            return Ok(());
        }
        let variables_chirho = image_chirho.free_vars_chirho();
        let weight_chirho = variables_chirho
            .len()
            .saturating_add(1)
            .saturating_mul(self.kind_scope_guards_chirho.len());
        self.charge_chirho(weight_chirho, weight_chirho, span_chirho)?;
        for guard_chirho in &mut self.kind_scope_guards_chirho {
            if variable_chirho.0 < guard_chirho.cutoff_chirho
                || guard_chirho.protected_chirho.contains(&variable_chirho)
            {
                if variables_chirho
                    .iter()
                    .any(|variable_chirho| guard_chirho.variables_chirho.contains(variable_chirho))
                {
                    return Err(ElaborationErrorChirho {
                        mismatch_chirho: None,
                        message_chirho:
                            "kind mismatch: a universal binder escaped into an enclosing kind"
                                .to_owned(),
                        span_chirho,
                    });
                }
                // Protect fresh dependencies too, so an indirect X -> fresh Y
                // followed by Y -> skolem cannot evade the enclosing boundary.
                guard_chirho
                    .protected_chirho
                    .extend(variables_chirho.iter().copied());
            }
        }
        Ok(())
    }

    /// Normalize under the one current substitution. Unlike the compatibility
    /// substitution helper, this follows chains and never turns an unsolved
    /// variable into a constructor chosen from its printed name.
    pub fn resolve_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        self.resolve_depth_chirho(
            ty_chirho,
            span_chirho,
            0,
            &mut HashSet::new(),
            &mut Vec::new(),
        )
    }

    fn resolve_depth_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
        depth_chirho: usize,
        active_chirho: &mut HashSet<TyVarChirho>,
        bound_chirho: &mut Vec<TyVarChirho>,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        self.charge_chirho(1, 1, span_chirho)?;
        if depth_chirho > 128 {
            return Err(
                self.resource_failure_chirho("kind elaboration depth bound exhausted", span_chirho)
            );
        }
        Ok(match ty_chirho {
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                self.charge_chirho(context_chirho.len(), context_chirho.len(), span_chirho)?;
                TyChirho::QualChirho {
                    context_chirho: context_chirho
                        .iter()
                        .map(|constraint_chirho| {
                            self.resolve_depth_chirho(
                                constraint_chirho,
                                span_chirho,
                                depth_chirho + 1,
                                active_chirho,
                                bound_chirho,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                    body_chirho: Box::new(self.resolve_depth_chirho(
                        body_chirho,
                        span_chirho,
                        depth_chirho + 1,
                        active_chirho,
                        bound_chirho,
                    )?),
                }
            }
            TyChirho::VarChirho(variable_chirho) => {
                self.charge_chirho(bound_chirho.len(), 0, span_chirho)?;
                if bound_chirho.contains(variable_chirho) {
                    return Ok(TyChirho::VarChirho(*variable_chirho));
                }
                if !active_chirho.insert(*variable_chirho) {
                    return Err(ElaborationErrorChirho {
                        mismatch_chirho: None,
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
                        return Err(self.resource_failure_chirho(
                            "kind substitution size bound exhausted",
                            span_chirho,
                        ));
                    }
                };
                let result_chirho = if let Some(image_chirho) = image_chirho {
                    self.resolve_depth_chirho(
                        &image_chirho,
                        span_chirho,
                        depth_chirho + 1,
                        active_chirho,
                        bound_chirho,
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
                    bound_chirho,
                )?),
                Box::new(self.resolve_depth_chirho(
                    arg_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    active_chirho,
                    bound_chirho,
                )?),
            ),
            TyChirho::FunChirho(arg_chirho, res_chirho, mult_chirho) => TyChirho::FunChirho(
                Box::new(self.resolve_depth_chirho(
                    arg_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    active_chirho,
                    bound_chirho,
                )?),
                Box::new(self.resolve_depth_chirho(
                    res_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    active_chirho,
                    bound_chirho,
                )?),
                *mult_chirho,
            ),
            TyChirho::ListChirho(element_chirho) => {
                TyChirho::ListChirho(Box::new(self.resolve_depth_chirho(
                    element_chirho,
                    span_chirho,
                    depth_chirho + 1,
                    active_chirho,
                    bound_chirho,
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
                                bound_chirho,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                )
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                self.charge_chirho(vars_chirho.len(), vars_chirho.len(), span_chirho)?;
                let mark_chirho = bound_chirho.len();
                let result_chirho = (|| {
                    let mut records_chirho = Vec::new();
                    for record_chirho in vars_chirho.records_chirho() {
                        let mut mapped_chirho = record_chirho.clone();
                        mapped_chirho.kind_chirho = record_chirho
                            .kind_chirho
                            .as_ref()
                            .map(|kind_chirho| {
                                self.resolve_depth_chirho(
                                    kind_chirho,
                                    span_chirho,
                                    depth_chirho + 1,
                                    active_chirho,
                                    bound_chirho,
                                )
                                .map(std::sync::Arc::new)
                            })
                            .transpose()?;
                        records_chirho.push(mapped_chirho);
                        bound_chirho.push(record_chirho.variable_chirho);
                    }
                    let body_chirho = Box::new(self.resolve_depth_chirho(
                        body_chirho,
                        span_chirho,
                        depth_chirho + 1,
                        active_chirho,
                        bound_chirho,
                    )?);
                    let vars_chirho =
                        crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(
                            records_chirho,
                        );
                    Ok(
                        if matches!(ty_chirho, TyChirho::RequiredForallChirho { .. }) {
                            TyChirho::RequiredForallChirho {
                                vars_chirho,
                                body_chirho,
                            }
                        } else {
                            TyChirho::ForallChirho {
                                vars_chirho,
                                body_chirho,
                            }
                        },
                    )
                })();
                bound_chirho.truncate(mark_chirho);
                result_chirho?
            }
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
                    self.guard_kind_image_chirho(
                        right_chirho,
                        &TyChirho::VarChirho(left_chirho),
                        span_chirho,
                    )?;
                    self.link_source_variable_chirho(right_chirho, left_chirho, span_chirho)?;
                    self.record_link_chirho(
                        right_chirho,
                        TyChirho::VarChirho(left_chirho),
                        span_chirho,
                    )?;
                }
                (TyChirho::VarChirho(variable_chirho), ty_chirho)
                | (ty_chirho, TyChirho::VarChirho(variable_chirho)) => {
                    if self.rigid_variables_chirho.contains(&variable_chirho) {
                        if self.body_scope_chirho {
                            continue;
                        }
                        return Err(ElaborationErrorChirho { mismatch_chirho: Some(std::sync::Arc::new((TyChirho::VarChirho(variable_chirho), ty_chirho))),
                    message_chirho: "kind mismatch: an explicitly polymorphic kind cannot be specialized".to_owned(), span_chirho });
                    }
                    if occurs_chirho(variable_chirho, &ty_chirho) {
                        return Err(ElaborationErrorChirho {
                            mismatch_chirho: None,
                            message_chirho: "infinite kind in declaration telescope".to_owned(),
                            span_chirho,
                        });
                    }
                    self.guard_kind_image_chirho(variable_chirho, &ty_chirho, span_chirho)?;
                    if let TyChirho::VarChirho(target_chirho) = &ty_chirho {
                        self.link_source_variable_chirho(
                            variable_chirho,
                            *target_chirho,
                            span_chirho,
                        )?;
                    }
                    self.record_link_chirho(variable_chirho, ty_chirho, span_chirho)?;
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
                (
                    TyChirho::ForallChirho {
                        vars_chirho: left_vars_chirho,
                        body_chirho: left_body_chirho,
                    },
                    TyChirho::ForallChirho {
                        vars_chirho: right_vars_chirho,
                        body_chirho: right_body_chirho,
                    },
                )
                | (
                    TyChirho::RequiredForallChirho {
                        vars_chirho: left_vars_chirho,
                        body_chirho: left_body_chirho,
                    },
                    TyChirho::RequiredForallChirho {
                        vars_chirho: right_vars_chirho,
                        body_chirho: right_body_chirho,
                    },
                ) if left_vars_chirho.len() == right_vars_chirho.len() => {
                    self.charge_chirho(
                        left_vars_chirho.len(),
                        left_vars_chirho.len(),
                        span_chirho,
                    )?;
                    let mut mapping_chirho = crate::subst_chirho::SubstChirho::empty_chirho();
                    // A binder's kind sees the preceding prefix only. Inline checked
                    // records carry imported/local ownership through alpha conversion.
                    for (left_chirho, right_chirho) in left_vars_chirho
                        .records_chirho()
                        .iter()
                        .zip(right_vars_chirho.records_chirho())
                    {
                        let left_kind_chirho = match &left_chirho.kind_chirho {
                            Some(kind_chirho) => self.resolve_chirho(kind_chirho, span_chirho)?,
                            None => {
                                self.variable_kind_chirho(left_chirho.variable_chirho, span_chirho)?
                            }
                        };
                        let right_kind_chirho = match &right_chirho.kind_chirho {
                            Some(kind_chirho) => self.resolve_chirho(kind_chirho, span_chirho)?,
                            None => self
                                .variable_kind_chirho(right_chirho.variable_chirho, span_chirho)?,
                        };
                        let right_kind_chirho = self.instantiate_term_chirho(
                            &right_kind_chirho,
                            &mapping_chirho,
                            true,
                            span_chirho,
                        )?;
                        self.check_rigid_chirho(
                            &left_kind_chirho,
                            &right_kind_chirho,
                            &left_vars_chirho,
                            span_chirho,
                        )?;
                        mapping_chirho.insert_chirho(
                            right_chirho.variable_chirho,
                            TyChirho::VarChirho(left_chirho.variable_chirho),
                        );
                    }
                    let right_body_chirho = self.instantiate_term_chirho(
                        &right_body_chirho,
                        &mapping_chirho,
                        true,
                        span_chirho,
                    )?;
                    let outside_chirho = left_body_chirho
                        .free_vars_chirho()
                        .into_iter()
                        .chain(right_body_chirho.free_vars_chirho())
                        .filter(|variable_chirho| !left_vars_chirho.contains(variable_chirho))
                        .collect::<HashSet<_>>();
                    self.check_rigid_chirho(
                        &left_body_chirho,
                        &right_body_chirho,
                        &left_vars_chirho,
                        span_chirho,
                    )?;
                    for variable_chirho in outside_chirho {
                        let image_chirho = self
                            .resolve_chirho(&TyChirho::VarChirho(variable_chirho), span_chirho)?;
                        if image_chirho
                            .free_vars_chirho()
                            .iter()
                            .any(|variable_chirho| left_vars_chirho.contains(variable_chirho))
                        {
                            return Err(ElaborationErrorChirho {
                                mismatch_chirho: None,
                                message_chirho:
                                    "kind mismatch: a universal binder escaped its quantifier"
                                        .to_owned(),
                                span_chirho,
                            });
                        }
                    }
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
                        mismatch_chirho: Some(std::sync::Arc::new((
                            self.copy_chirho(&left_chirho, span_chirho)?,
                            self.copy_chirho(&right_chirho, span_chirho)?,
                        ))),
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
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => context_chirho.iter().try_fold(
            child_chirho(body_chirho)?.checked_add(1)?,
            |weight_chirho, constraint_chirho| {
                weight_chirho.checked_add(child_chirho(constraint_chirho)?)
            },
        ),
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
        } => vars_chirho.records_chirho().iter().try_fold(
            child_chirho(body_chirho)?
                .checked_add(vars_chirho.len())?
                .checked_add(1)?,
            |weight_chirho, record_chirho| match &record_chirho.kind_chirho {
                Some(kind_chirho) => weight_chirho.checked_add(child_chirho(kind_chirho)?),
                None => Some(weight_chirho),
            },
        ),
    }?;
    (weight_chirho <= 1 << 16).then_some(weight_chirho)
}

fn occurs_chirho(variable_chirho: TyVarChirho, ty_chirho: &TyChirho) -> bool {
    ty_chirho.free_vars_chirho().contains(&variable_chirho)
}

#[cfg(test)]
mod tests_chirho;
