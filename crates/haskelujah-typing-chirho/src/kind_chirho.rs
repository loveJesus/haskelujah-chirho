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

use haskelujah_ast_chirho::decl_chirho::{AstKindChirho, DeclChirho, TyVarChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

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
            KindChirho::ArrowChirho(a_chirho, b_chirho) => {
                KindChirho::arrow_chirho(self.apply_chirho(a_chirho), self.apply_chirho(b_chirho))
            }
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
    Ok(KindSubstChirho::singleton_chirho(
        var_chirho,
        kind_chirho.clone(),
    ))
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
        for name_chirho in &[
            "Int", "Bool", "Char", "Double", "Float", "Integer", "String",
        ] {
            env_chirho.bind_chirho(name_chirho.to_string(), KindChirho::StarChirho);
        }

        // * -> * constructors
        let star_to_star_chirho =
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
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
        env_chirho.bind_chirho("->".to_string(), star2_chirho.clone());

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
    /// Cache for PolyKinds: maps source-level kind variable names to allocated KindVarChirho.
    kind_var_cache_chirho: std::collections::HashMap<String, KindVarChirho>,
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
            kind_var_cache_chirho: std::collections::HashMap::new(),
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

    /// Instantiate a kind by replacing all `VarChirho` with fresh variables.
    /// This is used when looking up a poly-kinded type constructor so each
    /// use site gets its own copy of the kind variables.
    fn instantiate_kind_chirho(&mut self, kind_chirho: &KindChirho) -> KindChirho {
        let mut var_map_chirho: HashMap<KindVarChirho, KindVarChirho> = HashMap::new();
        self.instantiate_kind_inner_chirho(kind_chirho, &mut var_map_chirho)
    }

    fn instantiate_kind_inner_chirho(
        &mut self,
        kind_chirho: &KindChirho,
        var_map_chirho: &mut HashMap<KindVarChirho, KindVarChirho>,
    ) -> KindChirho {
        match kind_chirho {
            KindChirho::VarChirho(v_chirho) => {
                // First resolve through the substitution
                if let Some(resolved_chirho) = self.subst_chirho.map_chirho.get(v_chirho) {
                    let resolved_chirho = resolved_chirho.clone();
                    return self.instantiate_kind_inner_chirho(&resolved_chirho, var_map_chirho);
                }
                let new_var_chirho = *var_map_chirho
                    .entry(*v_chirho)
                    .or_insert_with(|| self.fresh_var_chirho());
                KindChirho::VarChirho(new_var_chirho)
            }
            KindChirho::StarChirho | KindChirho::ConstraintChirho => kind_chirho.clone(),
            KindChirho::ArrowChirho(a_chirho, b_chirho) => KindChirho::arrow_chirho(
                self.instantiate_kind_inner_chirho(a_chirho, var_map_chirho),
                self.instantiate_kind_inner_chirho(b_chirho, var_map_chirho),
            ),
        }
    }

    /// Convert AST kind to internal kind, allocating fresh kind vars for PolyKinds.
    /// Same kind variable name maps to the same KindVarChirho within a declaration.
    fn ast_kind_to_kind_ctx_chirho(&mut self, ast_chirho: &AstKindChirho) -> KindChirho {
        match ast_chirho {
            AstKindChirho::StarChirho => KindChirho::StarChirho,
            AstKindChirho::ArrowChirho(a_chirho, b_chirho) => KindChirho::arrow_chirho(
                self.ast_kind_to_kind_ctx_chirho(a_chirho),
                self.ast_kind_to_kind_ctx_chirho(b_chirho),
            ),
            AstKindChirho::ConstraintChirho => KindChirho::ConstraintChirho,
            AstKindChirho::VarChirho(name_chirho) => {
                if let Some(&var_chirho) = self.kind_var_cache_chirho.get(name_chirho) {
                    KindChirho::VarChirho(var_chirho)
                } else {
                    let var_chirho = self.fresh_var_chirho();
                    self.kind_var_cache_chirho
                        .insert(name_chirho.clone(), var_chirho);
                    KindChirho::VarChirho(var_chirho)
                }
            }
        }
    }

    fn infer_constraint_kind_chirho(&mut self, constraint_chirho: &ConstraintChirho) -> KindChirho {
        match constraint_chirho {
            ConstraintChirho::ClassChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    let _k_chirho = self.infer_type_kind_chirho(arg_chirho);
                }
                KindChirho::ConstraintChirho
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => {
                for var_chirho in vars_chirho {
                    let kind_chirho = if let Some(ann_chirho) = &var_chirho.kind_annotation_chirho {
                        self.ast_kind_to_kind_ctx_chirho(ann_chirho)
                    } else {
                        self.fresh_kind_chirho()
                    };
                    self.env_chirho
                        .bind_chirho(var_chirho.text_chirho().to_string(), kind_chirho);
                }
                for inner_constraint_chirho in context_chirho {
                    let _k_chirho = self.infer_constraint_kind_chirho(inner_constraint_chirho);
                }
                let _k_chirho = self.infer_constraint_kind_chirho(body_chirho);
                KindChirho::ConstraintChirho
            }
        }
    }

    /// Interpret a `TypeChirho` as a kind (for standalone kind signatures like
    /// `data V :: N -> Type where`).  This converts the *type-level*
    /// representation of a kind back into a `KindChirho`.
    fn type_to_kind_chirho(&mut self, ty_chirho: &TypeChirho) -> KindChirho {
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho)
                if name_chirho.text_chirho() == "Type" || name_chirho.text_chirho() == "*" =>
            {
                KindChirho::StarChirho
            }
            TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Constraint" => {
                KindChirho::ConstraintChirho
            }
            TypeChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.full_name_chirho();
                // A named kind (e.g. `N` in `N -> Type`): look it up or
                // create a fresh kind variable.
                if let Some(k_chirho) = self.env_chirho.lookup_chirho(&text_chirho) {
                    // DataKinds: if this name has kind * in the env, it's a
                    // data type being used as a kind. At the kind level, data
                    // types are opaque sorts that classify promoted constructors.
                    // Treat them as equivalent to * for kind-checking purposes,
                    // since promoted constructors ultimately have kind *.
                    let k_chirho = k_chirho.clone();
                    match &k_chirho {
                        KindChirho::StarChirho => KindChirho::StarChirho,
                        _ => k_chirho,
                    }
                } else {
                    let k_chirho = self.fresh_kind_chirho();
                    self.env_chirho
                        .bind_chirho(text_chirho, k_chirho.clone());
                    k_chirho
                }
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => KindChirho::arrow_chirho(
                self.type_to_kind_chirho(arg_chirho),
                self.type_to_kind_chirho(result_chirho),
            ),
            TypeChirho::VarChirho(name_chirho) => {
                let name_str_chirho = name_chirho.text_chirho();
                if let Some(&var_chirho) = self.kind_var_cache_chirho.get(name_str_chirho) {
                    KindChirho::VarChirho(var_chirho)
                } else {
                    let var_chirho = self.fresh_var_chirho();
                    self.kind_var_cache_chirho
                        .insert(name_str_chirho.to_string(), var_chirho);
                    KindChirho::VarChirho(var_chirho)
                }
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                span_chirho,
            } => {
                // Kind application: if fun has kind (a -> r), apply arg of kind a
                // to get result kind r.
                let f_kind_chirho = self.type_to_kind_chirho(fun_chirho);
                let a_kind_chirho = self.type_to_kind_chirho(arg_chirho);
                let result_chirho = self.fresh_kind_chirho();
                let expected_fun_chirho =
                    KindChirho::arrow_chirho(a_kind_chirho, result_chirho.clone());
                self.unify_chirho(
                    &f_kind_chirho,
                    &expected_fun_chirho,
                    "kind application",
                    *span_chirho,
                );
                result_chirho
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => self.type_to_kind_chirho(inner_chirho),
            _ => {
                // Fallback: treat unknown shapes as *.
                KindChirho::StarChirho
            }
        }
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
        match unify_kind_chirho(
            &k1_applied_chirho,
            &k2_applied_chirho,
            context_chirho,
            span_chirho,
        ) {
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
                        let msg_chirho =
                            format!("infinite kind: `{var_chirho}` occurs in `{kind_chirho}`");
                        (msg_chirho, span_chirho, KIND_OCCURS_CODE_CHIRHO)
                    }
                };
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(code_chirho),
                        msg_chirho,
                        span_chirho,
                    ));
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
                    self.env_chirho
                        .bind_chirho(text_chirho.to_string(), k_chirho.clone());
                    k_chirho
                }
            }
            TypeChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.full_name_chirho();
                if let Some(k_chirho) = self.env_chirho.lookup_chirho(&text_chirho) {
                    let k_chirho = k_chirho.clone();
                    // Instantiate fresh kind variables for each use of a
                    // poly-kinded type constructor (e.g. `Proxy :: k -> Type`
                    // gets fresh `k` each time it appears).
                    if k_chirho.free_vars_chirho().is_empty() {
                        k_chirho
                    } else {
                        self.instantiate_kind_chirho(&k_chirho)
                    }
                } else {
                    // Unknown type constructor — assign a fresh kind variable.
                    let k_chirho = self.fresh_kind_chirho();
                    self.env_chirho
                        .bind_chirho(text_chirho, k_chirho.clone());
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
                ..
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
                // Tuple elements can all be * (value tuple) or all Constraint
                // (constraint tuple, e.g. (Show a, Eq a)). Use a fresh kind
                // variable and unify each element against it so the tuple's
                // kind is determined by its contents.
                let elem_kind_chirho = self.fresh_kind_chirho();
                for elem_chirho in elements_chirho {
                    let k_chirho = self.infer_type_kind_chirho(elem_chirho);
                    self.unify_chirho(
                        &k_chirho,
                        &elem_kind_chirho,
                        "tuple element type",
                        *span_chirho,
                    );
                }
                // Resolve the element kind — defaults to * if unconstrained.
                let resolved_chirho = self.subst_chirho.apply_chirho(&elem_kind_chirho);
                match &resolved_chirho {
                    KindChirho::ConstraintChirho => KindChirho::ConstraintChirho,
                    _ => KindChirho::StarChirho,
                }
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.infer_type_kind_chirho(inner_chirho)
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho: _,
            } => {
                // Bind each quantified variable: use annotation if present,
                // otherwise a fresh kind variable.
                for v_chirho in vars_chirho {
                    let k_chirho = if let Some(ann_chirho) = &v_chirho.kind_annotation_chirho {
                        self.ast_kind_to_kind_ctx_chirho(ann_chirho)
                    } else {
                        self.fresh_kind_chirho()
                    };
                    self.env_chirho
                        .bind_chirho(v_chirho.text_chirho().to_string(), k_chirho);
                }
                let k_chirho = self.infer_type_kind_chirho(body_chirho);
                // A forall type has the same kind as its body:
                // - forall a. a -> a  has kind * (body is *)
                // - forall a. C a => D (f a)  has kind Constraint (quantified constraint)
                k_chirho
            }
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                span_chirho: _,
            } => {
                // Kind-check each constraint in the context.
                // NOTE: We do NOT force constraint arguments to kind *.
                // Constraint arguments like `f` in `Functor f` have kind `* -> *`.
                // We just infer their kinds and let unification propagate.
                for constraint_chirho in context_chirho {
                    let _k_chirho = self.infer_constraint_kind_chirho(constraint_chirho);
                }
                // The body determines the kind of the qualified type:
                // - Num a => a -> a  has kind * (normal qualified type)
                // - NFData a => NFData (f a)  has kind Constraint (quantified constraint)
                let k_body_chirho = self.infer_type_kind_chirho(body_chirho);
                k_body_chirho
            }
            // DataKinds: promoted constructors ('True, 'Just, 'Proxy, etc.)
            // have kinds determined by their data constructor types. Since we
            // don't track constructor types in the kind env, assign a fresh
            // kind variable so they can unify with whatever context expects.
            TypeChirho::PromotedConChirho { name_chirho, .. } => {
                let text_chirho = name_chirho.text_chirho();
                if let Some(k_chirho) = self.env_chirho.lookup_chirho(text_chirho) {
                    k_chirho.clone()
                } else {
                    let k_chirho = self.fresh_kind_chirho();
                    self.env_chirho
                        .bind_chirho(text_chirho.to_string(), k_chirho.clone());
                    k_chirho
                }
            }
            // DataKinds: promoted list '[a, b] has kind [*] which we represent as *.
            TypeChirho::PromotedListChirho {
                elements_chirho,
                span_chirho,
            } => {
                for elem_chirho in elements_chirho {
                    let k_chirho = self.infer_type_kind_chirho(elem_chirho);
                    self.unify_chirho(
                        &k_chirho,
                        &KindChirho::StarChirho,
                        "promoted list element",
                        *span_chirho,
                    );
                }
                KindChirho::StarChirho
            }
            // PartialTypeSignatures: `_` is a wildcard that will be filled in
            // during type inference. Kind-wise it is treated as * (a regular
            // monotype position).
            TypeChirho::WildcardChirho { .. } => KindChirho::StarChirho,
            // Type-level literal (DataKinds): literals have kind *.
            TypeChirho::LitChirho { .. } => KindChirho::StarChirho,
        }
    }

    /// Process a data declaration to determine the kind of the type constructor.
    fn infer_data_decl_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[TyVarChirho],
        kind_sig_chirho: Option<&TypeChirho>,
        span_chirho: SpanChirho,
    ) {
        // If a standalone kind signature is given (e.g. `data V :: N -> Type where`),
        // interpret it directly as the type constructor's kind.
        let kind_chirho = if let Some(sig_chirho) = kind_sig_chirho {
            let k_chirho = self.type_to_kind_chirho(sig_chirho);
            // Still bind any explicit type variable names that appear.
            for tv_chirho in type_vars_chirho {
                let tvk_chirho = if let Some(ann_chirho) = &tv_chirho.kind_annotation_chirho {
                    self.ast_kind_to_kind_ctx_chirho(ann_chirho)
                } else {
                    self.fresh_kind_chirho()
                };
                self.env_chirho
                    .bind_chirho(tv_chirho.text_chirho().to_string(), tvk_chirho);
            }
            k_chirho
        } else {
            // Each type parameter gets a kind: use the annotation if present,
            // otherwise create a fresh kind variable for inference.
            let mut param_kinds_chirho = Vec::new();
            for tv_chirho in type_vars_chirho {
                let k_chirho = if let Some(ann_chirho) = &tv_chirho.kind_annotation_chirho {
                    self.ast_kind_to_kind_ctx_chirho(ann_chirho)
                } else {
                    self.fresh_kind_chirho()
                };
                self.env_chirho
                    .bind_chirho(tv_chirho.text_chirho().to_string(), k_chirho.clone());
                param_kinds_chirho.push(k_chirho);
            }
            // The type constructor's kind: k1 -> k2 -> ... -> *
            KindChirho::arrow_n_chirho(param_kinds_chirho, KindChirho::StarChirho)
        };

        // If already bound (e.g. from a use site), unify.
        if let Some(existing_chirho) = self.env_chirho.lookup_chirho(name_chirho) {
            let existing_chirho = existing_chirho.clone();
            self.unify_chirho(
                &existing_chirho,
                &kind_chirho,
                "data declaration",
                span_chirho,
            );
        }
        self.env_chirho
            .bind_chirho(name_chirho.to_string(), kind_chirho);
    }

    /// Process a class declaration to determine the kind of the class.
    fn infer_class_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[TyVarChirho],
        span_chirho: SpanChirho,
    ) {
        // Each class type parameter gets a kind: use the annotation if present,
        // otherwise create a fresh kind variable for inference.
        let mut param_kinds_chirho = Vec::new();
        for tv_chirho in type_vars_chirho {
            let k_chirho = if let Some(ann_chirho) = &tv_chirho.kind_annotation_chirho {
                self.ast_kind_to_kind_ctx_chirho(ann_chirho)
            } else {
                self.fresh_kind_chirho()
            };
            self.env_chirho
                .bind_chirho(tv_chirho.text_chirho().to_string(), k_chirho.clone());
            param_kinds_chirho.push(k_chirho);
        }

        // Class kind: k1 -> k2 -> ... -> Constraint
        let kind_chirho =
            KindChirho::arrow_n_chirho(param_kinds_chirho, KindChirho::ConstraintChirho);

        if let Some(existing_chirho) = self.env_chirho.lookup_chirho(name_chirho) {
            let existing_chirho = existing_chirho.clone();
            self.unify_chirho(
                &existing_chirho,
                &kind_chirho,
                "class declaration",
                span_chirho,
            );
        }
        self.env_chirho
            .bind_chirho(name_chirho.to_string(), kind_chirho);
    }

    /// Process a type alias to determine its kind.
    fn infer_type_alias_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[TyVarChirho],
        rhs_ty_chirho: &TypeChirho,
        span_chirho: SpanChirho,
    ) {
        // Each type parameter gets a kind: use the annotation if present,
        // otherwise create a fresh kind variable for inference.
        let mut param_kinds_chirho = Vec::new();
        for tv_chirho in type_vars_chirho {
            let k_chirho = if let Some(ann_chirho) = &tv_chirho.kind_annotation_chirho {
                self.ast_kind_to_kind_ctx_chirho(ann_chirho)
            } else {
                self.fresh_kind_chirho()
            };
            self.env_chirho
                .bind_chirho(tv_chirho.text_chirho().to_string(), k_chirho.clone());
            param_kinds_chirho.push(k_chirho);
        }

        // Infer the kind of the RHS.
        let rhs_kind_chirho = self.infer_type_kind_chirho(rhs_ty_chirho);

        // The alias kind: k_params -> k_rhs
        let kind_chirho = KindChirho::arrow_n_chirho(param_kinds_chirho, rhs_kind_chirho);

        if let Some(existing_chirho) = self.env_chirho.lookup_chirho(name_chirho) {
            let existing_chirho = existing_chirho.clone();
            self.unify_chirho(&existing_chirho, &kind_chirho, "type alias", span_chirho);
        }
        self.env_chirho
            .bind_chirho(name_chirho.to_string(), kind_chirho);
    }

    /// Finalize: apply substitution to all kinds in the environment.
    /// When PolyKinds is NOT enabled, default unconstrained kind variables to *.
    /// When PolyKinds IS enabled, preserve kind variables to allow polymorphic kinds.
    fn finalize_chirho(&mut self, _poly_kinds_enabled_chirho: bool) {
        self.env_chirho.apply_subst_chirho(&self.subst_chirho);
        // Default remaining kind variables to *.
        // Even with PolyKinds, unsolved kind variables default to * at the
        // module boundary — the polymorphism is achieved by instantiating
        // fresh vars at each use site in infer_type_kind_chirho.
        for kind_chirho in self.env_chirho.kinds_chirho.values_mut() {
            *kind_chirho = default_kind_vars_chirho(kind_chirho);
        }
    }
}

/// Convert an AST-level kind annotation to the internal [`KindChirho`] representation.
/// Kind variables (PolyKinds) default to `*` in this standalone version;
/// use `ast_kind_to_kind_ctx_chirho` for proper kind variable allocation.
#[cfg(test)]
fn ast_kind_to_kind_chirho(ast_chirho: &AstKindChirho) -> KindChirho {
    match ast_chirho {
        AstKindChirho::StarChirho => KindChirho::StarChirho,
        AstKindChirho::ArrowChirho(a_chirho, b_chirho) => KindChirho::arrow_chirho(
            ast_kind_to_kind_chirho(a_chirho),
            ast_kind_to_kind_chirho(b_chirho),
        ),
        AstKindChirho::ConstraintChirho => KindChirho::ConstraintChirho,
        // PolyKinds: kind variables default to * when used outside a context
        AstKindChirho::VarChirho(_) => KindChirho::StarChirho,
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
    let poly_kinds_enabled_chirho = module_chirho
        .extensions_chirho
        .iter()
        .any(|e_chirho| e_chirho == "PolyKinds" || e_chirho == "TypeInType");

    // Phase 1: Process all type/data/newtype/class declarations to establish
    // the kind of each type constructor.
    for decl_chirho in &module_chirho.decls_chirho {
        // Reset kind variable cache between declarations so that kind
        // variables from one declaration don't leak into the next.
        ctx_chirho.kind_var_cache_chirho.clear();

        // Snapshot the env keys so we can remove per-declaration locals
        // (type variables) after processing this declaration, preventing
        // scope leaks into subsequent declarations.
        let env_keys_before_chirho: std::collections::HashSet<String> =
            ctx_chirho.env_chirho.kinds_chirho.keys().cloned().collect();
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                kind_sig_chirho,
                span_chirho,
                ..
            } => {
                ctx_chirho.infer_data_decl_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    kind_sig_chirho.as_ref(),
                    *span_chirho,
                );
                // Kind-check constructor field types.
                for con_chirho in constructors_chirho {
                    match con_chirho {
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                            fields_chirho,
                            ..
                        } => {
                            for (_strictness_chirho, field_ty_chirho) in fields_chirho {
                                let k_chirho = ctx_chirho.infer_type_kind_chirho(field_ty_chirho);
                                ctx_chirho.unify_chirho(
                                    &k_chirho,
                                    &KindChirho::StarChirho,
                                    "data constructor field",
                                    *span_chirho,
                                );
                            }
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                            fields_chirho,
                            ..
                        } => {
                            for field_decl_chirho in fields_chirho {
                                let k_chirho =
                                    ctx_chirho.infer_type_kind_chirho(&field_decl_chirho.ty_chirho);
                                ctx_chirho.unify_chirho(
                                    &k_chirho,
                                    &KindChirho::StarChirho,
                                    "data constructor field",
                                    *span_chirho,
                                );
                            }
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                            ty_chirho,
                            ..
                        } => {
                            // Kind-check the full GADT type signature.
                            let k_chirho = ctx_chirho.infer_type_kind_chirho(ty_chirho);
                            ctx_chirho.unify_chirho(
                                &k_chirho,
                                &KindChirho::StarChirho,
                                "GADT constructor type",
                                *span_chirho,
                            );
                        }
                    }
                }
            }
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructor_chirho,
                kind_sig_chirho,
                span_chirho,
                ..
            } => {
                ctx_chirho.infer_data_decl_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    kind_sig_chirho.as_ref(),
                    *span_chirho,
                );
                // Kind-check the newtype constructor field.
                match constructor_chirho {
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                        fields_chirho,
                        ..
                    } => {
                        for (_strictness_chirho, field_ty_chirho) in fields_chirho {
                            let k_chirho = ctx_chirho.infer_type_kind_chirho(field_ty_chirho);
                            ctx_chirho.unify_chirho(
                                &k_chirho,
                                &KindChirho::StarChirho,
                                "newtype constructor field",
                                *span_chirho,
                            );
                        }
                    }
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                        fields_chirho,
                        ..
                    } => {
                        for field_decl_chirho in fields_chirho {
                            let k_chirho =
                                ctx_chirho.infer_type_kind_chirho(&field_decl_chirho.ty_chirho);
                            ctx_chirho.unify_chirho(
                                &k_chirho,
                                &KindChirho::StarChirho,
                                "newtype constructor field",
                                *span_chirho,
                            );
                        }
                    }
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                        ty_chirho,
                        ..
                    } => {
                        // Kind-check the full GADT type signature.
                        let k_chirho = ctx_chirho.infer_type_kind_chirho(ty_chirho);
                        ctx_chirho.unify_chirho(
                            &k_chirho,
                            &KindChirho::StarChirho,
                            "newtype GADT constructor type",
                            *span_chirho,
                        );
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
                context_chirho,
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
                // Kind-check superclass constraints — do NOT force args to *,
                // since constraint args like `f` in `Applicative f` may be `* -> *`.
                for constraint_chirho in context_chirho {
                    let _k_chirho = ctx_chirho.infer_constraint_kind_chirho(constraint_chirho);
                }
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

        // Remove per-declaration locals (type variables like `a`, `b`, `k`)
        // that were added during this declaration but shouldn't persist.
        // Keep only type constructor names that were in the env before or
        // that look like type constructors (uppercase first char).
        let keys_to_remove_chirho: Vec<String> = ctx_chirho
            .env_chirho
            .kinds_chirho
            .keys()
            .filter(|k_chirho| {
                !env_keys_before_chirho.contains(*k_chirho)
                    && k_chirho
                        .chars()
                        .next()
                        .map_or(true, |c_chirho| c_chirho.is_lowercase())
            })
            .cloned()
            .collect();
        for key_chirho in keys_to_remove_chirho {
            ctx_chirho.env_chirho.kinds_chirho.remove(&key_chirho);
        }
    }

    // Phase 2: Finalize — apply substitution and default unconstrained vars.
    ctx_chirho.finalize_chirho(poly_kinds_enabled_chirho);

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
    use haskelujah_ast_chirho::decl_chirho::{
        ClassMethodChirho, ConDeclChirho, DeclChirho, StrictnessChirho,
    };
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;

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
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_fun_chirho(arg_chirho: TypeChirho, result_chirho: TypeChirho) -> TypeChirho {
        TypeChirho::FunChirho {
            arg_chirho: Box::new(arg_chirho),
            mult_chirho: None,
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
        let k_chirho =
            KindChirho::arrow_chirho(KindChirho::VarChirho(v_chirho), KindChirho::StarChirho);
        assert_eq!(k_chirho.free_vars_chirho(), vec![v_chirho]);
    }

    // -- Kind substitution tests --

    #[test]
    fn subst_applies_chirho() {
        let v_chirho = KindVarChirho(0);
        let subst_chirho = KindSubstChirho::singleton_chirho(v_chirho, KindChirho::StarChirho);
        let result_chirho = subst_chirho.apply_chirho(&KindChirho::VarChirho(v_chirho));
        assert_eq!(result_chirho, KindChirho::StarChirho);
    }

    #[test]
    fn subst_compose_chirho() {
        let v0_chirho = KindVarChirho(0);
        let v1_chirho = KindVarChirho(1);
        let s1_chirho =
            KindSubstChirho::singleton_chirho(v0_chirho, KindChirho::VarChirho(v1_chirho));
        let s2_chirho = KindSubstChirho::singleton_chirho(v1_chirho, KindChirho::StarChirho);
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
        let k1_chirho =
            KindChirho::arrow_chirho(KindChirho::VarChirho(v_chirho), KindChirho::StarChirho);
        let k2_chirho = KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
        let result_chirho =
            unify_kind_chirho(&k1_chirho, &k2_chirho, "test", SpanChirho::DUMMY_CHIRHO).unwrap();
        assert_eq!(
            result_chirho.apply_chirho(&KindChirho::VarChirho(v_chirho)),
            KindChirho::StarChirho
        );
    }

    #[test]
    fn unify_mismatch_chirho() {
        // Star and Constraint now unify (ConstraintKinds behavior).
        let result_chirho = unify_kind_chirho(
            &KindChirho::StarChirho,
            &KindChirho::ConstraintChirho,
            "test",
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(result_chirho.is_ok());

        // Arrow vs Star is a genuine mismatch.
        let result2_chirho = unify_kind_chirho(
            &KindChirho::StarChirho,
            &KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
            "test",
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(matches!(
            result2_chirho,
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
        assert_eq!(
            env_chirho.lookup_chirho("Int"),
            Some(&KindChirho::StarChirho)
        );
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
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("Red"),
                fields_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
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
            type_vars_chirho: vec![mk_name_chirho("a").into()],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("MkBox"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::VarChirho(mk_name_chirho("a")),
                )],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
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
            type_vars_chirho: vec![mk_name_chirho("a").into(), mk_name_chirho("b").into()],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("MkPair"),
                fields_chirho: vec![
                    (
                        StrictnessChirho::LazyChirho,
                        TypeChirho::VarChirho(mk_name_chirho("a")),
                    ),
                    (
                        StrictnessChirho::LazyChirho,
                        TypeChirho::VarChirho(mk_name_chirho("b")),
                    ),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
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
            type_vars_chirho: vec![mk_name_chirho("f").into(), mk_name_chirho("a").into()],
            constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("MkApp"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("f"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                )],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
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
    fn qualified_type_constructor_kind_does_not_collide_chirho() {
        let env_chirho = KindEnvChirho::with_builtins_chirho();
        let mut ctx_chirho = KindInferCtxChirho::new_chirho(env_chirho);
        ctx_chirho.env_chirho.bind_chirho(
            "Operator".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::StarChirho,
                    KindChirho::StarChirho,
                    KindChirho::StarChirho,
                ],
                KindChirho::StarChirho,
            ),
        );

        let ty_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::AppChirho {
                    fun_chirho: Box::new(TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::ConChirho(NameChirho::RawChirho(
                            haskelujah_ast_chirho::name_chirho::RawNameChirho::qualified_chirho(
                                "N",
                                "Operator",
                                SpanChirho::DUMMY_CHIRHO,
                            ),
                        ))),
                        arg_chirho: Box::new(TypeChirho::ListChirho {
                            element_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("tok"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("st"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                arg_chirho: Box::new(TypeChirho::ConChirho(NameChirho::RawChirho(
                    haskelujah_ast_chirho::name_chirho::RawNameChirho::qualified_chirho(
                        "Control.Monad",
                        "Identity",
                        SpanChirho::DUMMY_CHIRHO,
                    ),
                ))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let _kind_chirho = ctx_chirho.infer_type_kind_chirho(&ty_chirho);
        assert!(
            !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
            "qualified imported type constructors should not collide with local unqualified ones: {:?}",
            ctx_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn class_single_param_chirho() {
        // class Eq a where eq :: a -> a -> Bool
        let module_chirho = mk_module_chirho(vec![DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: mk_name_chirho("Eq"),
            type_vars_chirho: vec![mk_name_chirho("a").into()],
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
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
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
            type_vars_chirho: vec![mk_name_chirho("f").into()],
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
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
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
            type_vars_chirho: vec![mk_name_chirho("a").into()],
            constructor_chirho: ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("Wrap"),
                fields_chirho: vec![(
                    StrictnessChirho::LazyChirho,
                    TypeChirho::VarChirho(mk_name_chirho("a")),
                )],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            deriving_chirho: vec![],
            kind_sig_chirho: None,
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

    #[test]
    fn ast_kind_constraint_converts_chirho() {
        let ast_chirho = AstKindChirho::ConstraintChirho;
        assert_eq!(
            ast_kind_to_kind_chirho(&ast_chirho),
            KindChirho::ConstraintChirho
        );
    }

    #[test]
    fn ast_kind_constraint_arrow_converts_chirho() {
        let ast_chirho = AstKindChirho::ArrowChirho(
            Box::new(AstKindChirho::StarChirho),
            Box::new(AstKindChirho::ConstraintChirho),
        );
        let expected_chirho =
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::ConstraintChirho);
        assert_eq!(ast_kind_to_kind_chirho(&ast_chirho), expected_chirho);
    }

    #[test]
    fn ast_kind_var_defaults_to_star_chirho() {
        // PolyKinds: standalone ast_kind_to_kind_chirho defaults kind vars to *
        let ast_chirho = AstKindChirho::VarChirho("k".to_string());
        assert_eq!(ast_kind_to_kind_chirho(&ast_chirho), KindChirho::StarChirho);
    }

    #[test]
    fn kind_var_cache_reuses_same_var_chirho() {
        // PolyKinds: context-aware conversion maps same name to same kind var
        let env_chirho = KindEnvChirho::new_chirho();
        let mut ctx_chirho = KindInferCtxChirho::new_chirho(env_chirho);
        let k1_chirho =
            ctx_chirho.ast_kind_to_kind_ctx_chirho(&AstKindChirho::VarChirho("k".to_string()));
        let k2_chirho =
            ctx_chirho.ast_kind_to_kind_ctx_chirho(&AstKindChirho::VarChirho("k".to_string()));
        assert_eq!(k1_chirho, k2_chirho);
        // Different name gets different var
        let j_chirho =
            ctx_chirho.ast_kind_to_kind_ctx_chirho(&AstKindChirho::VarChirho("j".to_string()));
        assert_ne!(k1_chirho, j_chirho);
    }
}
