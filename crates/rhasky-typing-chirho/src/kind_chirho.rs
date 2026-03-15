// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Kind inference
//!
//! Validates that type constructors are applied at the correct kinds.
//! Haskell has a kind system where `*` (Star) is the kind of types and
//! `k1 -> k2` is the kind of type constructors.
//!
//! Examples:
//! - `Int :: *`
//! - `Maybe :: * -> *`
//! - `Either :: * -> * -> *`
//! - `Functor :: (* -> *) -> Constraint`
//!
//! This module implements kind inference with unification variables,
//! analogous to type inference but at the kind level.

use std::collections::HashMap;
use std::fmt;

use rhasky_ast_chirho::decl_chirho::DeclChirho;
use rhasky_ast_chirho::module_chirho::ModuleChirho;
use rhasky_ast_chirho::ty_chirho::TypeChirho;
use rhasky_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use rhasky_span_chirho::SpanChirho;

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

    /// `Constraint` — the kind of typeclass constraints.
    ConstraintChirho,
}

impl KindChirho {
    /// Build an arrow kind `k1 -> k2`.
    pub fn arrow_chirho(from_chirho: KindChirho, to_chirho: KindChirho) -> Self {
        Self::ArrowChirho(Box::new(from_chirho), Box::new(to_chirho))
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
            KindChirho::StarChirho | KindChirho::ConstraintChirho => {}
            KindChirho::ArrowChirho(a_chirho, b_chirho) => {
                a_chirho.collect_free_vars_chirho(out_chirho);
                b_chirho.collect_free_vars_chirho(out_chirho);
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
    map_chirho: HashMap<KindVarChirho, KindChirho>,
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
        match kind_chirho {
            KindChirho::VarChirho(v_chirho) => {
                if let Some(k_chirho) = self.map_chirho.get(v_chirho) {
                    self.apply_chirho(k_chirho)
                } else {
                    kind_chirho.clone()
                }
            }
            KindChirho::StarChirho | KindChirho::ConstraintChirho => kind_chirho.clone(),
            KindChirho::ArrowChirho(a_chirho, b_chirho) => KindChirho::arrow_chirho(
                self.apply_chirho(a_chirho),
                self.apply_chirho(b_chirho),
            ),
        }
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
            result_chirho.entry(*v_chirho).or_insert_with(|| k_chirho.clone());
        }
        KindSubstChirho { map_chirho: result_chirho }
    }
}

// ---------------------------------------------------------------------------
// Kind unification
// ---------------------------------------------------------------------------

/// Error from kind unification.
#[derive(Debug, Clone, PartialEq)]
pub enum KindErrorChirho {
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
fn unify_kind_chirho(
    k1_chirho: &KindChirho,
    k2_chirho: &KindChirho,
    context_chirho: &str,
    span_chirho: SpanChirho,
) -> Result<KindSubstChirho, KindErrorChirho> {
    match (k1_chirho, k2_chirho) {
        (KindChirho::StarChirho, KindChirho::StarChirho) => Ok(KindSubstChirho::empty_chirho()),
        (KindChirho::ConstraintChirho, KindChirho::ConstraintChirho) => {
            Ok(KindSubstChirho::empty_chirho())
        }

        (KindChirho::VarChirho(v_chirho), k_chirho) | (k_chirho, KindChirho::VarChirho(v_chirho)) => {
            bind_kind_var_chirho(*v_chirho, k_chirho, span_chirho)
        }

        (
            KindChirho::ArrowChirho(a1_chirho, b1_chirho),
            KindChirho::ArrowChirho(a2_chirho, b2_chirho),
        ) => {
            let s1_chirho = unify_kind_chirho(a1_chirho, a2_chirho, context_chirho, span_chirho)?;
            let b1_sub_chirho = s1_chirho.apply_chirho(b1_chirho);
            let b2_sub_chirho = s1_chirho.apply_chirho(b2_chirho);
            let s2_chirho =
                unify_kind_chirho(&b1_sub_chirho, &b2_sub_chirho, context_chirho, span_chirho)?;
            Ok(s2_chirho.compose_chirho(&s1_chirho))
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
    if kind_chirho.free_vars_chirho().contains(&var_chirho) {
        return Err(KindErrorChirho::OccursCheckChirho {
            var_chirho,
            kind_chirho: kind_chirho.clone(),
            span_chirho,
        });
    }
    Ok(KindSubstChirho::singleton_chirho(var_chirho, kind_chirho.clone()))
}

// ---------------------------------------------------------------------------
// Kind environment & inference context
// ---------------------------------------------------------------------------

/// Maps type constructor names to their kinds.
#[derive(Debug, Clone, Default)]
pub struct KindEnvChirho {
    kinds_chirho: HashMap<String, KindChirho>,
}

impl KindEnvChirho {
    pub fn new_chirho() -> Self {
        Self::default()
    }

    /// Seed the environment with built-in type constructor kinds.
    pub fn with_builtins_chirho() -> Self {
        let mut env_chirho = Self::new_chirho();

        // Primitive types: kind *
        for name_chirho in &["Int", "Bool", "Char", "Double", "Float", "Integer", "String"] {
            env_chirho.bind_chirho(name_chirho.to_string(), KindChirho::StarChirho);
        }

        // * -> * constructors
        let star_to_star_chirho = KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::StarChirho,
        );
        for name_chirho in &["Maybe", "[]", "IO"] {
            env_chirho.bind_chirho(name_chirho.to_string(), star_to_star_chirho.clone());
        }

        // * -> * -> * constructors
        let star2_chirho = KindChirho::arrow_n_chirho(
            vec![KindChirho::StarChirho, KindChirho::StarChirho],
            KindChirho::StarChirho,
        );
        for name_chirho in &["Either", "(,)", "Map"] {
            env_chirho.bind_chirho(name_chirho.to_string(), star2_chirho.clone());
        }

        // Tuple constructors: (,,) :: * -> * -> * -> *, etc.
        for arity_chirho in 3u8..=7 {
            let name_chirho = format!("({})", ",".repeat(arity_chirho as usize - 1));
            let kind_chirho = KindChirho::arrow_n_chirho(
                (0..arity_chirho).map(|_| KindChirho::StarChirho),
                KindChirho::StarChirho,
            );
            env_chirho.bind_chirho(name_chirho, kind_chirho);
        }

        // (->) :: * -> * -> *
        env_chirho.bind_chirho(
            "->".to_string(),
            star2_chirho.clone(),
        );

        env_chirho
    }

    pub fn bind_chirho(&mut self, name_chirho: String, kind_chirho: KindChirho) {
        self.kinds_chirho.insert(name_chirho, kind_chirho);
    }

    pub fn lookup_chirho(&self, name_chirho: &str) -> Option<&KindChirho> {
        self.kinds_chirho.get(name_chirho)
    }

    /// Apply a substitution to all kinds in the environment.
    pub fn apply_subst_chirho(&mut self, subst_chirho: &KindSubstChirho) {
        for kind_chirho in self.kinds_chirho.values_mut() {
            *kind_chirho = subst_chirho.apply_chirho(kind_chirho);
        }
    }
}

/// Kind inference context with fresh variable generation.
struct KindInferCtxChirho {
    env_chirho: KindEnvChirho,
    subst_chirho: KindSubstChirho,
    next_var_chirho: u32,
    diagnostics_chirho: DiagnosticBundleChirho,
}

/// Error codes for kind diagnostics.
const KIND_MISMATCH_CODE_CHIRHO: u16 = 300;
const KIND_OCCURS_CODE_CHIRHO: u16 = 301;

impl KindInferCtxChirho {
    fn new_chirho(env_chirho: KindEnvChirho) -> Self {
        Self {
            env_chirho,
            subst_chirho: KindSubstChirho::empty_chirho(),
            next_var_chirho: 0,
            diagnostics_chirho: DiagnosticBundleChirho::empty_chirho(),
        }
    }

    fn fresh_var_chirho(&mut self) -> KindVarChirho {
        let v_chirho = KindVarChirho(self.next_var_chirho);
        self.next_var_chirho += 1;
        v_chirho
    }

    fn fresh_kind_chirho(&mut self) -> KindChirho {
        KindChirho::VarChirho(self.fresh_var_chirho())
    }

    /// Unify two kinds and accumulate the substitution.
    fn unify_chirho(
        &mut self,
        k1_chirho: &KindChirho,
        k2_chirho: &KindChirho,
        context_chirho: &str,
        span_chirho: SpanChirho,
    ) {
        let k1_applied_chirho = self.subst_chirho.apply_chirho(k1_chirho);
        let k2_applied_chirho = self.subst_chirho.apply_chirho(k2_chirho);
        match unify_kind_chirho(&k1_applied_chirho, &k2_applied_chirho, context_chirho, span_chirho) {
            Ok(s_chirho) => {
                self.subst_chirho = s_chirho.compose_chirho(&self.subst_chirho);
            }
            Err(err_chirho) => {
                let (msg_chirho, span_chirho, code_chirho) = match err_chirho {
                    KindErrorChirho::MismatchChirho {
                        expected_chirho,
                        actual_chirho,
                        context_chirho,
                        span_chirho,
                    } => {
                        let msg_chirho = format!(
                            "kind mismatch in {context_chirho}: expected `{expected_chirho}`, found `{actual_chirho}`"
                        );
                        (msg_chirho, span_chirho, KIND_MISMATCH_CODE_CHIRHO)
                    }
                    KindErrorChirho::OccursCheckChirho {
                        var_chirho,
                        kind_chirho,
                        span_chirho,
                    } => {
                        let msg_chirho = format!(
                            "infinite kind: `{var_chirho}` occurs in `{kind_chirho}`"
                        );
                        (msg_chirho, span_chirho, KIND_OCCURS_CODE_CHIRHO)
                    }
                };
                self.diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(code_chirho),
                        msg_chirho,
                        span_chirho,
                    ),
                );
            }
        }
    }

    /// Infer the kind of a type expression.
    fn infer_type_kind_chirho(&mut self, ty_chirho: &TypeChirho) -> KindChirho {
        match ty_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                if let Some(k_chirho) = self.env_chirho.lookup_chirho(text_chirho) {
                    k_chirho.clone()
                } else {
                    // Unknown type variable — assign a fresh kind variable.
                    let k_chirho = self.fresh_kind_chirho();
                    self.env_chirho.bind_chirho(text_chirho.to_string(), k_chirho.clone());
                    k_chirho
                }
            }
            TypeChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                if let Some(k_chirho) = self.env_chirho.lookup_chirho(text_chirho) {
                    k_chirho.clone()
                } else {
                    // Unknown type constructor — assign a fresh kind variable.
                    let k_chirho = self.fresh_kind_chirho();
                    self.env_chirho.bind_chirho(text_chirho.to_string(), k_chirho.clone());
                    k_chirho
                }
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                span_chirho,
            } => {
                let k_fun_chirho = self.infer_type_kind_chirho(fun_chirho);
                let k_arg_chirho = self.infer_type_kind_chirho(arg_chirho);
                let k_result_chirho = self.fresh_kind_chirho();
                // fun must have kind (k_arg -> k_result)
                let expected_chirho =
                    KindChirho::arrow_chirho(k_arg_chirho, k_result_chirho.clone());
                self.unify_chirho(
                    &k_fun_chirho,
                    &expected_chirho,
                    "type application",
                    *span_chirho,
                );
                k_result_chirho
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                span_chirho,
            } => {
                let k_a_chirho = self.infer_type_kind_chirho(arg_chirho);
                let k_b_chirho = self.infer_type_kind_chirho(result_chirho);
                // Both sides of -> must be *
                self.unify_chirho(
                    &k_a_chirho,
                    &KindChirho::StarChirho,
                    "function type argument",
                    *span_chirho,
                );
                self.unify_chirho(
                    &k_b_chirho,
                    &KindChirho::StarChirho,
                    "function type result",
                    *span_chirho,
                );
                KindChirho::StarChirho
            }
            TypeChirho::ListChirho {
                element_chirho,
                span_chirho,
            } => {
                let k_elem_chirho = self.infer_type_kind_chirho(element_chirho);
                self.unify_chirho(
                    &k_elem_chirho,
                    &KindChirho::StarChirho,
                    "list element type",
                    *span_chirho,
                );
                KindChirho::StarChirho
            }
            TypeChirho::TupleChirho {
                elements_chirho,
                span_chirho,
            } => {
                for elem_chirho in elements_chirho {
                    let k_chirho = self.infer_type_kind_chirho(elem_chirho);
                    self.unify_chirho(
                        &k_chirho,
                        &KindChirho::StarChirho,
                        "tuple element type",
                        *span_chirho,
                    );
                }
                KindChirho::StarChirho
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.infer_type_kind_chirho(inner_chirho)
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho,
            } => {
                // Bind each quantified variable with a fresh kind variable.
                for v_chirho in vars_chirho {
                    let k_chirho = self.fresh_kind_chirho();
                    self.env_chirho.bind_chirho(v_chirho.text_chirho().to_string(), k_chirho);
                }
                let k_chirho = self.infer_type_kind_chirho(body_chirho);
                // A forall type itself must have kind *
                self.unify_chirho(
                    &k_chirho,
                    &KindChirho::StarChirho,
                    "forall body",
                    *span_chirho,
                );
                KindChirho::StarChirho
            }
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                span_chirho,
            } => {
                // Kind-check each constraint in the context.
                for constraint_chirho in context_chirho {
                    for arg_chirho in &constraint_chirho.args_chirho {
                        let k_chirho = self.infer_type_kind_chirho(arg_chirho);
                        self.unify_chirho(
                            &k_chirho,
                            &KindChirho::StarChirho,
                            "constraint argument",
                            constraint_chirho.span_chirho,
                        );
                    }
                }
                // The body must have kind *
                let k_body_chirho = self.infer_type_kind_chirho(body_chirho);
                self.unify_chirho(
                    &k_body_chirho,
                    &KindChirho::StarChirho,
                    "qualified type body",
                    *span_chirho,
                );
                KindChirho::StarChirho
            }
        }
    }

    /// Process a data declaration to determine the kind of the type constructor.
    fn infer_data_decl_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[rhasky_ast_chirho::name_chirho::NameChirho],
        span_chirho: SpanChirho,
    ) {
        // Each type parameter gets a fresh kind variable.
        let mut param_kinds_chirho = Vec::new();
        for tv_chirho in type_vars_chirho {
            let k_chirho = self.fresh_kind_chirho();
            self.env_chirho
                .bind_chirho(tv_chirho.text_chirho().to_string(), k_chirho.clone());
            param_kinds_chirho.push(k_chirho);
        }

        // The type constructor's kind: k1 -> k2 -> ... -> *
        let kind_chirho =
            KindChirho::arrow_n_chirho(param_kinds_chirho, KindChirho::StarChirho);

        // If already bound (e.g. from a use site), unify.
        if let Some(existing_chirho) = self.env_chirho.lookup_chirho(name_chirho) {
            let existing_chirho = existing_chirho.clone();
            self.unify_chirho(&existing_chirho, &kind_chirho, "data declaration", span_chirho);
        }
        self.env_chirho.bind_chirho(name_chirho.to_string(), kind_chirho);
    }

    /// Process a class declaration to determine the kind of the class.
    fn infer_class_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[rhasky_ast_chirho::name_chirho::NameChirho],
        span_chirho: SpanChirho,
    ) {
        // Each class type parameter gets a fresh kind variable.
        let mut param_kinds_chirho = Vec::new();
        for tv_chirho in type_vars_chirho {
            let k_chirho = self.fresh_kind_chirho();
            self.env_chirho
                .bind_chirho(tv_chirho.text_chirho().to_string(), k_chirho.clone());
            param_kinds_chirho.push(k_chirho);
        }

        // Class kind: k1 -> k2 -> ... -> Constraint
        let kind_chirho =
            KindChirho::arrow_n_chirho(param_kinds_chirho, KindChirho::ConstraintChirho);

        if let Some(existing_chirho) = self.env_chirho.lookup_chirho(name_chirho) {
            let existing_chirho = existing_chirho.clone();
            self.unify_chirho(&existing_chirho, &kind_chirho, "class declaration", span_chirho);
        }
        self.env_chirho.bind_chirho(name_chirho.to_string(), kind_chirho);
    }

    /// Process a type alias to determine its kind.
    fn infer_type_alias_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[rhasky_ast_chirho::name_chirho::NameChirho],
        rhs_ty_chirho: &TypeChirho,
        span_chirho: SpanChirho,
    ) {
        // Each type parameter gets a fresh kind variable.
        let mut param_kinds_chirho = Vec::new();
        for tv_chirho in type_vars_chirho {
            let k_chirho = self.fresh_kind_chirho();
            self.env_chirho
                .bind_chirho(tv_chirho.text_chirho().to_string(), k_chirho.clone());
            param_kinds_chirho.push(k_chirho);
        }

        // Infer the kind of the RHS.
        let rhs_kind_chirho = self.infer_type_kind_chirho(rhs_ty_chirho);

        // The alias kind: k_params -> k_rhs
        let kind_chirho =
            KindChirho::arrow_n_chirho(param_kinds_chirho, rhs_kind_chirho);

        if let Some(existing_chirho) = self.env_chirho.lookup_chirho(name_chirho) {
            let existing_chirho = existing_chirho.clone();
            self.unify_chirho(&existing_chirho, &kind_chirho, "type alias", span_chirho);
        }
        self.env_chirho.bind_chirho(name_chirho.to_string(), kind_chirho);
    }

    /// Finalize: apply substitution to all kinds in the environment and default
    /// unconstrained kind variables to *.
    fn finalize_chirho(&mut self) {
        self.env_chirho.apply_subst_chirho(&self.subst_chirho);
        // Default remaining kind variables to *
        for kind_chirho in self.env_chirho.kinds_chirho.values_mut() {
            *kind_chirho = default_kind_vars_chirho(kind_chirho);
        }
    }
}

/// Replace all remaining kind variables with *.
fn default_kind_vars_chirho(kind_chirho: &KindChirho) -> KindChirho {
    match kind_chirho {
        KindChirho::VarChirho(_) => KindChirho::StarChirho,
        KindChirho::StarChirho | KindChirho::ConstraintChirho => kind_chirho.clone(),
        KindChirho::ArrowChirho(a_chirho, b_chirho) => KindChirho::arrow_chirho(
            default_kind_vars_chirho(a_chirho),
            default_kind_vars_chirho(b_chirho),
        ),
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Result of kind inference on a module.
#[derive(Debug)]
pub struct KindResultChirho {
    /// The kind environment after inference (type constructors → kinds).
    pub env_chirho: KindEnvChirho,
    /// Diagnostics (errors and warnings) from kind checking.
    pub diagnostics_chirho: DiagnosticBundleChirho,
}

/// Run kind inference on a module's type declarations and type signatures.
pub fn infer_module_kinds_chirho(module_chirho: &ModuleChirho) -> KindResultChirho {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho());

    // Phase 1: Process all type/data/newtype/class declarations to establish
    // the kind of each type constructor.
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                span_chirho,
                ..
            } => {
                ctx_chirho.infer_data_decl_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    *span_chirho,
                );
                // Kind-check constructor field types.
                for con_chirho in constructors_chirho {
                    match con_chirho {
                        rhasky_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                            fields_chirho,
                            ..
                        } => {
                            for field_ty_chirho in fields_chirho {
                                let k_chirho = ctx_chirho.infer_type_kind_chirho(field_ty_chirho);
                                ctx_chirho.unify_chirho(
                                    &k_chirho,
                                    &KindChirho::StarChirho,
                                    "data constructor field",
                                    *span_chirho,
                                );
                            }
                        }
                        rhasky_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                            fields_chirho,
                            ..
                        } => {
                            for field_decl_chirho in fields_chirho {
                                let k_chirho = ctx_chirho.infer_type_kind_chirho(&field_decl_chirho.ty_chirho);
                                ctx_chirho.unify_chirho(
                                    &k_chirho,
                                    &KindChirho::StarChirho,
                                    "data constructor field",
                                    *span_chirho,
                                );
                            }
                        }
                    }
                }
            }
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructor_chirho,
                span_chirho,
                ..
            } => {
                ctx_chirho.infer_data_decl_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    *span_chirho,
                );
                // Kind-check the newtype constructor field.
                match constructor_chirho {
                    rhasky_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                        fields_chirho,
                        ..
                    } => {
                        for field_ty_chirho in fields_chirho {
                            let k_chirho = ctx_chirho.infer_type_kind_chirho(field_ty_chirho);
                            ctx_chirho.unify_chirho(
                                &k_chirho,
                                &KindChirho::StarChirho,
                                "newtype constructor field",
                                *span_chirho,
                            );
                        }
                    }
                    rhasky_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                        fields_chirho,
                        ..
                    } => {
                        for field_decl_chirho in fields_chirho {
                            let k_chirho = ctx_chirho.infer_type_kind_chirho(&field_decl_chirho.ty_chirho);
                            ctx_chirho.unify_chirho(
                                &k_chirho,
                                &KindChirho::StarChirho,
                                "newtype constructor field",
                                *span_chirho,
                            );
                        }
                    }
                }
            }
            DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                rhs_chirho,
                span_chirho,
            } => {
                ctx_chirho.infer_type_alias_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    rhs_chirho,
                    *span_chirho,
                );
            }
            DeclChirho::ClassDeclChirho {
                name_chirho,
                type_vars_chirho,
                methods_chirho,
                span_chirho,
                ..
            } => {
                ctx_chirho.infer_class_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    *span_chirho,
                );
                // Kind-check method type signatures.
                for method_chirho in methods_chirho {
                    let k_chirho = ctx_chirho.infer_type_kind_chirho(&method_chirho.ty_chirho);
                    ctx_chirho.unify_chirho(
                        &k_chirho,
                        &KindChirho::StarChirho,
                        "class method type",
                        method_chirho.span_chirho,
                    );
                }
            }
            DeclChirho::TypeSigChirho {
                ty_chirho,
                span_chirho,
                ..
            } => {
                // Kind-check standalone type signatures.
                let k_chirho = ctx_chirho.infer_type_kind_chirho(ty_chirho);
                ctx_chirho.unify_chirho(
                    &k_chirho,
                    &KindChirho::StarChirho,
                    "type signature",
                    *span_chirho,
                );
            }
            _ => {}
        }
    }

    // Phase 2: Finalize — apply substitution and default unconstrained vars.
    ctx_chirho.finalize_chirho();

    KindResultChirho {
        env_chirho: ctx_chirho.env_chirho,
        diagnostics_chirho: ctx_chirho.diagnostics_chirho,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhasky_ast_chirho::decl_chirho::{ClassMethodChirho, ConDeclChirho, DeclChirho};
    use rhasky_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    use rhasky_ast_chirho::ty_chirho::TypeChirho;

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
            inline_pragmas_chirho: std::collections::HashMap::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_fun_chirho(arg_chirho: TypeChirho, result_chirho: TypeChirho) -> TypeChirho {
        TypeChirho::FunChirho {
            arg_chirho: Box::new(arg_chirho),
            result_chirho: Box::new(result_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_app_chirho(fun_chirho: TypeChirho, arg_chirho: TypeChirho) -> TypeChirho {
        TypeChirho::AppChirho {
            fun_chirho: Box::new(fun_chirho),
            arg_chirho: Box::new(arg_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    // -- Kind representation tests --

    #[test]
    fn star_display_chirho() {
        assert_eq!(KindChirho::StarChirho.to_string(), "*");
    }

    #[test]
    fn arrow_display_chirho() {
        let k_chirho = KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
        assert_eq!(k_chirho.to_string(), "* -> *");
    }

    #[test]
    fn arrow_display_nested_chirho() {
        // (* -> *) -> *
        let k_chirho = KindChirho::arrow_chirho(
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
            KindChirho::StarChirho,
        );
        assert_eq!(k_chirho.to_string(), "(* -> *) -> *");
    }

    #[test]
    fn arrow_n_builds_curried_chirho() {
        let k_chirho = KindChirho::arrow_n_chirho(
            vec![KindChirho::StarChirho, KindChirho::StarChirho],
            KindChirho::StarChirho,
        );
        assert_eq!(k_chirho.to_string(), "* -> * -> *");
    }

    #[test]
    fn free_vars_chirho() {
        let v_chirho = KindVarChirho(0);
        let k_chirho = KindChirho::arrow_chirho(
            KindChirho::VarChirho(v_chirho),
            KindChirho::StarChirho,
        );
        assert_eq!(k_chirho.free_vars_chirho(), vec![v_chirho]);
    }

    // -- Kind substitution tests --

    #[test]
    fn subst_applies_chirho() {
        let v_chirho = KindVarChirho(0);
        let subst_chirho =
            KindSubstChirho::singleton_chirho(v_chirho, KindChirho::StarChirho);
        let result_chirho = subst_chirho.apply_chirho(&KindChirho::VarChirho(v_chirho));
        assert_eq!(result_chirho, KindChirho::StarChirho);
    }

    #[test]
    fn subst_compose_chirho() {
        let v0_chirho = KindVarChirho(0);
        let v1_chirho = KindVarChirho(1);
        let s1_chirho =
            KindSubstChirho::singleton_chirho(v0_chirho, KindChirho::VarChirho(v1_chirho));
        let s2_chirho =
            KindSubstChirho::singleton_chirho(v1_chirho, KindChirho::StarChirho);
        let composed_chirho = s2_chirho.compose_chirho(&s1_chirho);
        // v0 should map to * (through v1)
        assert_eq!(
            composed_chirho.apply_chirho(&KindChirho::VarChirho(v0_chirho)),
            KindChirho::StarChirho
        );
    }

    // -- Kind unification tests --

    #[test]
    fn unify_star_star_chirho() {
        let result_chirho = unify_kind_chirho(
            &KindChirho::StarChirho,
            &KindChirho::StarChirho,
            "test",
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(result_chirho.is_ok());
        assert!(result_chirho.unwrap().is_empty_chirho());
    }

    #[test]
    fn unify_var_with_star_chirho() {
        let v_chirho = KindVarChirho(0);
        let result_chirho = unify_kind_chirho(
            &KindChirho::VarChirho(v_chirho),
            &KindChirho::StarChirho,
            "test",
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
        assert_eq!(
            result_chirho.apply_chirho(&KindChirho::VarChirho(v_chirho)),
            KindChirho::StarChirho
        );
    }

    #[test]
    fn unify_arrow_kinds_chirho() {
        let v_chirho = KindVarChirho(0);
        let k1_chirho = KindChirho::arrow_chirho(
            KindChirho::VarChirho(v_chirho),
            KindChirho::StarChirho,
        );
        let k2_chirho = KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::StarChirho,
        );
        let result_chirho =
            unify_kind_chirho(&k1_chirho, &k2_chirho, "test", SpanChirho::DUMMY_CHIRHO)
                .unwrap();
        assert_eq!(
            result_chirho.apply_chirho(&KindChirho::VarChirho(v_chirho)),
            KindChirho::StarChirho
        );
    }

    #[test]
    fn unify_mismatch_chirho() {
        let result_chirho = unify_kind_chirho(
            &KindChirho::StarChirho,
            &KindChirho::ConstraintChirho,
            "test",
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(matches!(
            result_chirho,
            Err(KindErrorChirho::MismatchChirho { .. })
        ));
    }

    #[test]
    fn occurs_check_chirho() {
        let v_chirho = KindVarChirho(0);
        let result_chirho = unify_kind_chirho(
            &KindChirho::VarChirho(v_chirho),
            &KindChirho::arrow_chirho(KindChirho::VarChirho(v_chirho), KindChirho::StarChirho),
            "test",
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(matches!(
            result_chirho,
            Err(KindErrorChirho::OccursCheckChirho { .. })
        ));
    }

    // -- Kind environment tests --

    #[test]
    fn builtins_have_correct_kinds_chirho() {
        let env_chirho = KindEnvChirho::with_builtins_chirho();
        assert_eq!(env_chirho.lookup_chirho("Int"), Some(&KindChirho::StarChirho));
        assert_eq!(
            env_chirho.lookup_chirho("Maybe"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::StarChirho
            ))
        );
        assert_eq!(
            env_chirho.lookup_chirho("Either"),
            Some(&KindChirho::arrow_n_chirho(
                vec![KindChirho::StarChirho, KindChirho::StarChirho],
                KindChirho::StarChirho
            ))
        );
    }

    // -- Module-level kind inference tests --

    #[test]
    fn data_no_params_has_kind_star_chirho() {
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Color"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![
                ConDeclChirho::OrdinaryChirho {
                    name_chirho: mk_name_chirho("Red"),
                    fields_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            deriving_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Color"),
            Some(&KindChirho::StarChirho)
        );
    }

    #[test]
    fn data_one_param_has_kind_star_to_star_chirho() {
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Box"),
            type_vars_chirho: vec![mk_name_chirho("a")],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("MkBox"),
                fields_chirho: vec![TypeChirho::VarChirho(
                    mk_name_chirho("a"),
                )],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Box"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::StarChirho
            ))
        );
    }

    #[test]
    fn data_two_params_chirho() {
        // data Pair a b = MkPair a b
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Pair"),
            type_vars_chirho: vec![mk_name_chirho("a"), mk_name_chirho("b")],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("MkPair"),
                fields_chirho: vec![
                    TypeChirho::VarChirho(mk_name_chirho("a")),
                    TypeChirho::VarChirho(mk_name_chirho("b")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Pair"),
            Some(&KindChirho::arrow_n_chirho(
                vec![KindChirho::StarChirho, KindChirho::StarChirho],
                KindChirho::StarChirho
            ))
        );
    }

    #[test]
    fn higher_kinded_type_param_chirho() {
        // data App f a = MkApp (f a)
        // f :: * -> *, a :: *, App :: (* -> *) -> * -> *
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("App"),
            type_vars_chirho: vec![mk_name_chirho("f"), mk_name_chirho("a")],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("MkApp"),
                fields_chirho: vec![TypeChirho::AppChirho {
                    fun_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("f"))),
                    arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        // App :: (* -> *) -> * -> *
        let expected_chirho = KindChirho::arrow_chirho(
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("App"),
            Some(&expected_chirho)
        );
    }

    #[test]
    fn class_single_param_chirho() {
        // class Eq a where eq :: a -> a -> Bool
        let module_chirho = mk_module_chirho(vec![DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: mk_name_chirho("Eq"),
            type_vars_chirho: vec![mk_name_chirho("a")],
            methods_chirho: vec![ClassMethodChirho {
                name_chirho: mk_name_chirho("eq"),
                ty_chirho: mk_fun_chirho(
                    TypeChirho::VarChirho(mk_name_chirho("a")),
                    mk_fun_chirho(
                        TypeChirho::VarChirho(mk_name_chirho("a")),
                        TypeChirho::ConChirho(mk_name_chirho("Bool")),
                    ),
                ),
                default_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Eq"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::ConstraintChirho
            ))
        );
    }

    #[test]
    fn class_higher_kinded_param_chirho() {
        // class Functor f where fmap :: (a -> b) -> f a -> f b
        // f :: * -> *, Functor :: (* -> *) -> Constraint
        let module_chirho = mk_module_chirho(vec![DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: mk_name_chirho("Functor"),
            type_vars_chirho: vec![mk_name_chirho("f")],
            methods_chirho: vec![ClassMethodChirho {
                name_chirho: mk_name_chirho("fmap"),
                ty_chirho: mk_fun_chirho(
                    // (a -> b)
                    mk_fun_chirho(
                        TypeChirho::VarChirho(mk_name_chirho("a")),
                        TypeChirho::VarChirho(mk_name_chirho("b")),
                    ),
                    // f a -> f b
                    mk_fun_chirho(
                        mk_app_chirho(
                            TypeChirho::VarChirho(mk_name_chirho("f")),
                            TypeChirho::VarChirho(mk_name_chirho("a")),
                        ),
                        mk_app_chirho(
                            TypeChirho::VarChirho(mk_name_chirho("f")),
                            TypeChirho::VarChirho(mk_name_chirho("b")),
                        ),
                    ),
                ),
                default_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        // Functor :: (* -> *) -> Constraint
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Functor"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
                KindChirho::ConstraintChirho
            ))
        );
    }

    #[test]
    fn type_alias_kind_chirho() {
        // type StringPair = (String, String)
        let module_chirho = mk_module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("StringPair"),
            type_vars_chirho: vec![],
            rhs_chirho: TypeChirho::TupleChirho {
                elements_chirho: vec![
                    TypeChirho::ConChirho(mk_name_chirho("String")),
                    TypeChirho::ConChirho(mk_name_chirho("String")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("StringPair"),
            Some(&KindChirho::StarChirho)
        );
    }

    #[test]
    fn newtype_kind_chirho() {
        // newtype Wrapper a = Wrap a
        let module_chirho = mk_module_chirho(vec![DeclChirho::NewtypeDeclChirho {
            name_chirho: mk_name_chirho("Wrapper"),
            type_vars_chirho: vec![mk_name_chirho("a")],
            constructor_chirho: ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("Wrap"),
                fields_chirho: vec![TypeChirho::VarChirho(
                    mk_name_chirho("a"),
                )],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            deriving_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Wrapper"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::StarChirho
            ))
        );
    }

    #[test]
    fn empty_module_no_errors_chirho() {
        let module_chirho = mk_module_chirho(vec![]);
        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    }

    #[test]
    fn default_unconstrained_vars_chirho() {
        let k_chirho = KindChirho::VarChirho(KindVarChirho(99));
        assert_eq!(default_kind_vars_chirho(&k_chirho), KindChirho::StarChirho);
    }

    #[test]
    fn constraint_kind_display_chirho() {
        assert_eq!(KindChirho::ConstraintChirho.to_string(), "Constraint");
    }
}
