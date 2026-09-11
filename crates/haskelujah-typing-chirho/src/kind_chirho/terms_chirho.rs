// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Kind terms, capture-safe substitution and equality.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use haskelujah_span_chirho::SpanChirho;
use std::collections::HashMap;
use std::fmt;

// ---------------------------------------------------------------------------
// Kind representation
// ---------------------------------------------------------------------------

/// A kind variable (for kind inference unification).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KindVarChirho(pub u32);

impl fmt::Display for KindVarChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "k{}", self.0)
    }
}

/// The kind of a type expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KindChirho {
    /// `*` — the kind of types (also called `Type`).
    StarChirho,

    /// `k1 -> k2` — the kind of type constructors.
    ArrowChirho(Box<KindChirho>, Box<KindChirho>),

    /// A kind unification variable (filled in during inference).
    VarChirho(KindVarChirho),

    /// A source-quantified kind opened for checking, never a substitution key.
    RigidChirho(KindVarChirho),

    /// `Constraint` — the kind of typeclass constraints.
    ConstraintChirho,

    /// A nominal type-level term used as a kind, not the kind of that term.
    ConChirho(String),

    /// Application in the kind language, such as `TYPE representation`.
    AppChirho(Box<KindChirho>, Box<KindChirho>),

    /// A de Bruijn index in a dependent result: zero denotes its nearest
    /// enclosing binder. Unlike inference ids, these are lexical positions.
    BoundChirho(u32),

    /// Visible dependent quantification: supplying the argument substitutes its
    /// term for this binder in the result, not merely its inferred kind.
    DependentChirho {
        argument_chirho: Box<KindChirho>,
        result_chirho: Box<KindChirho>,
    },
}

impl KindChirho {
    /// Build an arrow kind `k1 -> k2`.
    pub fn arrow_chirho(from_chirho: KindChirho, to_chirho: KindChirho) -> Self {
        Self::ArrowChirho(Box::new(from_chirho), Box::new(to_chirho))
    }

    pub(super) fn app_chirho(fun_chirho: KindChirho, arg_chirho: KindChirho) -> Self {
        if matches!(&fun_chirho, Self::ConChirho(name_chirho) if name_chirho == super::runtime_chirho::TYPE_CHIRHO)
            && arg_chirho == super::runtime_chirho::boxed_rep_chirho("Lifted")
        {
            return Self::StarChirho;
        }
        Self::AppChirho(Box::new(fun_chirho), Box::new(arg_chirho))
    }

    /// Map atomic terms without duplicating the tree walk across substitution,
    /// scheme abstraction and defaulting. Bound identities remain a distinct case.
    pub(super) fn map_leaves_chirho(&self, mapper_chirho: &mut impl FnMut(&Self) -> Self) -> Self {
        self.map_scoped_leaves_chirho(0, &mut |term_chirho, _depth_chirho| {
            mapper_chirho(term_chirho)
        })
    }

    /// The one traversal that knows the binding boundary. The parameter's
    /// annotation lies outside its scope; only the result increases the depth.
    fn map_scoped_leaves_chirho(
        &self,
        depth_chirho: u32,
        mapper_chirho: &mut impl FnMut(&Self, u32) -> Self,
    ) -> Self {
        match self {
            Self::ArrowChirho(argument_chirho, result_chirho) => Self::arrow_chirho(
                argument_chirho.map_scoped_leaves_chirho(depth_chirho, mapper_chirho),
                result_chirho.map_scoped_leaves_chirho(depth_chirho, mapper_chirho),
            ),
            Self::AppChirho(fun_chirho, arg_chirho) => Self::app_chirho(
                fun_chirho.map_scoped_leaves_chirho(depth_chirho, mapper_chirho),
                arg_chirho.map_scoped_leaves_chirho(depth_chirho, mapper_chirho),
            ),
            Self::DependentChirho {
                argument_chirho,
                result_chirho,
            } => Self::DependentChirho {
                argument_chirho: Box::new(
                    argument_chirho.map_scoped_leaves_chirho(depth_chirho, mapper_chirho),
                ),
                result_chirho: Box::new(
                    result_chirho.map_scoped_leaves_chirho(depth_chirho + 1, mapper_chirho),
                ),
            },
            _ => mapper_chirho(self, depth_chirho),
        }
    }

    pub(super) fn abstract_variable_chirho(&self, variable_chirho: KindVarChirho) -> Self {
        self.map_scoped_leaves_chirho(0, &mut |term_chirho, depth_chirho| match term_chirho {
            Self::VarChirho(found_chirho) if *found_chirho == variable_chirho => {
                Self::BoundChirho(depth_chirho)
            }
            Self::BoundChirho(index_chirho) if *index_chirho >= depth_chirho => {
                Self::BoundChirho(index_chirho + 1)
            }
            _ => term_chirho.clone(),
        })
    }

    pub(super) fn substitute_bound_chirho(&self, value_chirho: &Self) -> Self {
        self.map_scoped_leaves_chirho(0, &mut |term_chirho, depth_chirho| match term_chirho {
            Self::BoundChirho(index_chirho) if *index_chirho == depth_chirho => {
                // Moving the argument under an inner forall shifts its FREE
                // bound references, but never that argument's own local binders.
                value_chirho.map_scoped_leaves_chirho(0, &mut |inner_chirho, inner_depth_chirho| {
                    match inner_chirho {
                        Self::BoundChirho(inner_index_chirho)
                            if *inner_index_chirho >= inner_depth_chirho =>
                        {
                            Self::BoundChirho(inner_index_chirho + depth_chirho)
                        }
                        _ => inner_chirho.clone(),
                    }
                })
            }
            Self::BoundChirho(index_chirho) if *index_chirho > depth_chirho => {
                Self::BoundChirho(index_chirho - 1)
            }
            _ => term_chirho.clone(),
        })
    }

    fn has_free_bound_chirho(&self, depth_chirho: u32) -> bool {
        match self {
            Self::BoundChirho(index_chirho) => *index_chirho >= depth_chirho,
            Self::ArrowChirho(left_chirho, right_chirho)
            | Self::AppChirho(left_chirho, right_chirho) => {
                left_chirho.has_free_bound_chirho(depth_chirho)
                    || right_chirho.has_free_bound_chirho(depth_chirho)
            }
            Self::DependentChirho {
                argument_chirho,
                result_chirho,
            } => {
                argument_chirho.has_free_bound_chirho(depth_chirho)
                    || result_chirho.has_free_bound_chirho(depth_chirho + 1)
            }
            _ => false,
        }
    }

    /// Whether this term uses one particular surrounding binder. Inner binders
    /// shift that position in their body, but not in their own annotation.
    pub(super) fn references_bound_chirho(&self, index_chirho: u32) -> bool {
        match self {
            Self::BoundChirho(found_chirho) => *found_chirho == index_chirho,
            Self::ArrowChirho(left_chirho, right_chirho)
            | Self::AppChirho(left_chirho, right_chirho) => {
                left_chirho.references_bound_chirho(index_chirho)
                    || right_chirho.references_bound_chirho(index_chirho)
            }
            Self::DependentChirho {
                argument_chirho,
                result_chirho,
            } => {
                argument_chirho.references_bound_chirho(index_chirho)
                    || result_chirho.references_bound_chirho(index_chirho + 1)
            }
            _ => false,
        }
    }

    /// Build a multi-argument arrow kind `k1 -> k2 -> ... -> result`.
    pub fn arrow_n_chirho(
        args_chirho: impl IntoIterator<Item = KindChirho>,
        result_chirho: KindChirho,
    ) -> Self {
        let mut k_chirho = result_chirho;
        let args_vec_chirho: Vec<_> = args_chirho.into_iter().collect();
        for arg_chirho in args_vec_chirho.into_iter().rev() {
            k_chirho = Self::arrow_chirho(arg_chirho, k_chirho);
        }
        k_chirho
    }

    /// Collect free kind variables.
    pub fn free_vars_chirho(&self) -> Vec<KindVarChirho> {
        let mut vars_chirho = Vec::new();
        self.collect_free_vars_chirho(&mut vars_chirho);
        vars_chirho.sort();
        vars_chirho.dedup();
        vars_chirho
    }

    fn collect_free_vars_chirho(&self, out_chirho: &mut Vec<KindVarChirho>) {
        match self {
            KindChirho::VarChirho(v_chirho) => out_chirho.push(*v_chirho),
            KindChirho::StarChirho
            | KindChirho::ConstraintChirho
            | KindChirho::RigidChirho(_)
            | KindChirho::BoundChirho(_)
            | KindChirho::ConChirho(_) => {}
            KindChirho::ArrowChirho(a_chirho, b_chirho)
            | KindChirho::AppChirho(a_chirho, b_chirho) => {
                a_chirho.collect_free_vars_chirho(out_chirho);
                b_chirho.collect_free_vars_chirho(out_chirho);
            }
            KindChirho::DependentChirho {
                argument_chirho,
                result_chirho,
                ..
            } => {
                argument_chirho.collect_free_vars_chirho(out_chirho);
                result_chirho.collect_free_vars_chirho(out_chirho);
            }
        }
    }
}

impl fmt::Display for KindChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KindChirho::StarChirho => write!(f_chirho, "*"),
            KindChirho::ConstraintChirho => write!(f_chirho, "Constraint"),
            KindChirho::VarChirho(v_chirho) => write!(f_chirho, "{v_chirho}"),
            KindChirho::RigidChirho(v_chirho) => write!(f_chirho, "rigid {v_chirho}"),
            KindChirho::BoundChirho(v_chirho) => write!(f_chirho, "bound {v_chirho}"),
            KindChirho::ConChirho(name_chirho) => write!(f_chirho, "{name_chirho}"),
            KindChirho::AppChirho(fun_chirho, arg_chirho) => {
                write!(f_chirho, "({fun_chirho} {arg_chirho})")
            }
            KindChirho::DependentChirho {
                argument_chirho,
                result_chirho,
            } => {
                write!(
                    f_chirho,
                    "forall (_ :: {argument_chirho}) -> {result_chirho}"
                )
            }
            KindChirho::ArrowChirho(a_chirho, b_chirho) => {
                // Parenthesize the left side if it's an arrow
                match a_chirho.as_ref() {
                    KindChirho::ArrowChirho(_, _) => write!(f_chirho, "({a_chirho}) -> {b_chirho}"),
                    _ => write!(f_chirho, "{a_chirho} -> {b_chirho}"),
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Kind substitution
// ---------------------------------------------------------------------------

/// A substitution mapping kind variables to kinds.
#[derive(Debug, Clone, Default)]
pub struct KindSubstChirho {
    pub(super) map_chirho: HashMap<KindVarChirho, KindChirho>,
}

impl KindSubstChirho {
    pub fn empty_chirho() -> Self {
        Self::default()
    }

    pub fn singleton_chirho(var_chirho: KindVarChirho, kind_chirho: KindChirho) -> Self {
        let mut map_chirho = HashMap::new();
        map_chirho.insert(var_chirho, kind_chirho);
        Self { map_chirho }
    }

    pub fn is_empty_chirho(&self) -> bool {
        self.map_chirho.is_empty()
    }

    /// Apply this substitution to a kind.
    pub fn apply_chirho(&self, kind_chirho: &KindChirho) -> KindChirho {
        kind_chirho.map_leaves_chirho(&mut |term_chirho| match term_chirho {
            KindChirho::VarChirho(v_chirho) => {
                if let Some(k_chirho) = self.map_chirho.get(v_chirho) {
                    self.apply_chirho(k_chirho)
                } else {
                    term_chirho.clone()
                }
            }
            _ => term_chirho.clone(),
        })
    }

    /// Compose two substitutions: `self ∘ other`.
    /// Applying `compose(s2, s1)` is the same as applying `s1` then `s2`.
    pub fn compose_chirho(&self, other_chirho: &KindSubstChirho) -> KindSubstChirho {
        let mut result_chirho: HashMap<KindVarChirho, KindChirho> = other_chirho
            .map_chirho
            .iter()
            .map(|(v_chirho, k_chirho)| (*v_chirho, self.apply_chirho(k_chirho)))
            .collect();
        for (v_chirho, k_chirho) in &self.map_chirho {
            result_chirho
                .entry(*v_chirho)
                .or_insert_with(|| k_chirho.clone());
        }
        KindSubstChirho {
            map_chirho: result_chirho,
        }
    }
}

// ---------------------------------------------------------------------------
// Kind unification
// ---------------------------------------------------------------------------

/// Error from kind unification.
#[derive(Debug, Clone, PartialEq)]
pub enum KindErrorChirho {
    ReductionLimitChirho {
        span_chirho: SpanChirho,
    },
    MismatchChirho {
        expected_chirho: KindChirho,
        actual_chirho: KindChirho,
        context_chirho: String,
        span_chirho: SpanChirho,
    },
    OccursCheckChirho {
        var_chirho: KindVarChirho,
        kind_chirho: KindChirho,
        span_chirho: SpanChirho,
    },
}

/// Unify two kinds.
pub(super) fn unify_kind_chirho(
    k1_chirho: &KindChirho,
    k2_chirho: &KindChirho,
    context_chirho: &str,
    span_chirho: SpanChirho,
) -> Result<KindSubstChirho, KindErrorChirho> {
    match (k1_chirho, k2_chirho) {
        (
            KindChirho::ArrowChirho(argument_chirho, result_chirho),
            application_chirho @ KindChirho::AppChirho(_, _),
        )
        | (
            application_chirho @ KindChirho::AppChirho(_, _),
            KindChirho::ArrowChirho(argument_chirho, result_chirho),
        ) => {
            let arrow_term_chirho = KindChirho::app_chirho(
                KindChirho::app_chirho(
                    KindChirho::ConChirho("->".into()),
                    argument_chirho.as_ref().clone(),
                ),
                result_chirho.as_ref().clone(),
            );
            unify_kind_chirho(
                &arrow_term_chirho,
                application_chirho,
                context_chirho,
                span_chirho,
            )
        }
        (KindChirho::StarChirho, KindChirho::AppChirho(fun_chirho, representation_chirho))
        | (KindChirho::AppChirho(fun_chirho, representation_chirho), KindChirho::StarChirho)
            if matches!(fun_chirho.as_ref(), KindChirho::ConChirho(name_chirho) if name_chirho == super::runtime_chirho::TYPE_CHIRHO) =>
        {
            unify_kind_chirho(
                representation_chirho,
                &super::runtime_chirho::boxed_rep_chirho("Lifted"),
                context_chirho,
                span_chirho,
            )
        }
        (KindChirho::ConChirho(left_chirho), KindChirho::ConChirho(right_chirho))
            if left_chirho == right_chirho =>
        {
            Ok(KindSubstChirho::empty_chirho())
        }
        (KindChirho::BoundChirho(left_chirho), KindChirho::BoundChirho(right_chirho))
            if left_chirho == right_chirho =>
        {
            Ok(KindSubstChirho::empty_chirho())
        }
        (KindChirho::RigidChirho(left_chirho), KindChirho::RigidChirho(right_chirho))
            if left_chirho == right_chirho =>
        {
            Ok(KindSubstChirho::empty_chirho())
        }
        (KindChirho::StarChirho, KindChirho::StarChirho) => Ok(KindSubstChirho::empty_chirho()),
        (KindChirho::ConstraintChirho, KindChirho::ConstraintChirho) => {
            Ok(KindSubstChirho::empty_chirho())
        }

        // ConstraintKinds: GHC treats Constraint and Type (*) as interchangeable
        // in many contexts. Allow unification between them to handle patterns like
        // `Dict :: Constraint -> Type` and constraint tuples in type positions.
        (KindChirho::StarChirho, KindChirho::ConstraintChirho)
        | (KindChirho::ConstraintChirho, KindChirho::StarChirho) => {
            Ok(KindSubstChirho::empty_chirho())
        }

        (KindChirho::VarChirho(v_chirho), k_chirho)
        | (k_chirho, KindChirho::VarChirho(v_chirho)) => {
            bind_kind_var_chirho(*v_chirho, k_chirho, span_chirho)
        }

        (
            KindChirho::ArrowChirho(a1_chirho, b1_chirho),
            KindChirho::ArrowChirho(a2_chirho, b2_chirho),
        )
        | (
            KindChirho::AppChirho(a1_chirho, b1_chirho),
            KindChirho::AppChirho(a2_chirho, b2_chirho),
        ) => {
            let s1_chirho = unify_kind_chirho(a1_chirho, a2_chirho, context_chirho, span_chirho)?;
            let b1_sub_chirho = s1_chirho.apply_chirho(b1_chirho);
            let b2_sub_chirho = s1_chirho.apply_chirho(b2_chirho);
            let s2_chirho =
                unify_kind_chirho(&b1_sub_chirho, &b2_sub_chirho, context_chirho, span_chirho)?;
            Ok(s2_chirho.compose_chirho(&s1_chirho))
        }

        (
            KindChirho::DependentChirho {
                argument_chirho: left_arg_chirho,
                result_chirho: left_result_chirho,
            },
            KindChirho::DependentChirho {
                argument_chirho: right_arg_chirho,
                result_chirho: right_result_chirho,
            },
        ) => {
            let argument_subst_chirho = unify_kind_chirho(
                left_arg_chirho,
                right_arg_chirho,
                context_chirho,
                span_chirho,
            )?;
            // Equal lexical positions are alpha-equivalent. A free meta cannot
            // be assigned a local bound position (checked by bind_kind_var).
            let body_subst_chirho = unify_kind_chirho(
                &argument_subst_chirho.apply_chirho(left_result_chirho),
                &argument_subst_chirho.apply_chirho(right_result_chirho),
                context_chirho,
                span_chirho,
            )?;
            Ok(body_subst_chirho.compose_chirho(&argument_subst_chirho))
        }

        _ => Err(KindErrorChirho::MismatchChirho {
            expected_chirho: k1_chirho.clone(),
            actual_chirho: k2_chirho.clone(),
            context_chirho: context_chirho.to_string(),
            span_chirho,
        }),
    }
}

/// Bind a kind variable to a kind, with occurs check.
fn bind_kind_var_chirho(
    var_chirho: KindVarChirho,
    kind_chirho: &KindChirho,
    span_chirho: SpanChirho,
) -> Result<KindSubstChirho, KindErrorChirho> {
    if *kind_chirho == KindChirho::VarChirho(var_chirho) {
        return Ok(KindSubstChirho::empty_chirho());
    }
    if kind_chirho.has_free_bound_chirho(0) {
        return Err(KindErrorChirho::MismatchChirho {
            expected_chirho: KindChirho::VarChirho(var_chirho),
            actual_chirho: kind_chirho.clone(),
            context_chirho: "escaping dependent kind binder".into(),
            span_chirho,
        });
    }
    if kind_chirho.free_vars_chirho().contains(&var_chirho) {
        return Err(KindErrorChirho::OccursCheckChirho {
            var_chirho,
            kind_chirho: kind_chirho.clone(),
            span_chirho,
        });
    }
    Ok(KindSubstChirho::singleton_chirho(
        var_chirho,
        kind_chirho.clone(),
    ))
}
