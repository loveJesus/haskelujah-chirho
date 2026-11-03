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

use haskelujah_ast_chirho::decl_chirho::{
    AstKindChirho, DataKindSigChirho, DeclChirho, TyVarChirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{BuiltinClassChirho, BuiltinClassScopeChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

mod class_imports_chirho;
#[cfg(test)]
mod class_imports_tests_chirho;
mod conversion_chirho;
#[cfg(test)]
mod declaration_tests_chirho;
mod declarations_chirho;
mod module_pass_chirho;
mod scope_chirho;
#[cfg(test)]
mod scope_tests_chirho;
mod sorts_chirho;
#[cfg(test)]
mod sorts_tests_chirho;
#[cfg(test)]
mod tests_chirho;

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

fn is_builtin_typelit_or_typenat_kind_name_chirho(name_chirho: &str) -> bool {
    matches!(
        name_chirho,
        "Nat"
            | "Symbol"
            | "+"
            | "*"
            | "^"
            | "-"
            | "Div"
            | "Mod"
            | "<=?"
            | "AppendSymbol"
            | "Log2"
            | "CharToNat"
            | "NatToChar"
    )
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
            "Int",
            "Int8",
            "Int16",
            "Int32",
            "Int64",
            "Word",
            "Word8",
            "Word16",
            "Word32",
            "Word64",
            "Bool",
            "Char",
            "Double",
            "Float",
            "Integer",
            "String",
            "Buffer",
            "BufferPool",
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
        let promoted_cons_elem_kind_chirho = KindVarChirho(10_009);
        let promoted_cons_kind_chirho = KindChirho::arrow_n_chirho(
            vec![
                KindChirho::VarChirho(promoted_cons_elem_kind_chirho),
                KindChirho::StarChirho,
            ],
            KindChirho::StarChirho,
        );
        for name_chirho in &[":", "':"] {
            env_chirho.bind_chirho(name_chirho.to_string(), promoted_cons_kind_chirho.clone());
        }
        env_chirho.bind_chirho("ST".to_string(), star2_chirho.clone());
        env_chirho.bind_chirho(
            "StateT".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::StarChirho,
                    star_to_star_chirho.clone(),
                    KindChirho::StarChirho,
                ],
                KindChirho::StarChirho,
            ),
        );

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

        // Built-in type-level literal families. Nat/Symbol literals and their
        // family results are represented as ordinary type-level constants here.
        for name_chirho in &["Nat", "Symbol"] {
            env_chirho.bind_chirho(name_chirho.to_string(), KindChirho::StarChirho);
        }
        for name_chirho in &["+", "*", "^", "-", "Div", "Mod", "<=?", "AppendSymbol"] {
            env_chirho.bind_chirho(name_chirho.to_string(), star2_chirho.clone());
        }
        // workflow: language-features-chirho/type-level-character-families-chirho
        for name_chirho in &["Log2", "CharToNat", "NatToChar"] {
            env_chirho.bind_chirho(name_chirho.to_string(), star_to_star_chirho.clone());
        }

        // Poly-kinded builtins that appear in imported package signatures.
        // We model them with free kind variables so each use site can
        // instantiate them independently via `instantiate_kind_chirho`.
        let typeable_kind_var_chirho = KindVarChirho(10_000);
        env_chirho.bind_chirho(
            "Typeable".to_string(),
            KindChirho::arrow_chirho(
                KindChirho::VarChirho(typeable_kind_var_chirho),
                KindChirho::ConstraintChirho,
            ),
        );

        let proxy_kind_var_chirho = KindVarChirho(10_001);
        env_chirho.bind_chirho(
            "Proxy".to_string(),
            KindChirho::arrow_chirho(
                KindChirho::VarChirho(proxy_kind_var_chirho),
                KindChirho::StarChirho,
            ),
        );
        env_chirho.bind_chirho(
            "Proxy#".to_string(),
            KindChirho::arrow_chirho(
                KindChirho::VarChirho(proxy_kind_var_chirho),
                KindChirho::StarChirho,
            ),
        );
        let kproxy_kind_var_chirho = KindVarChirho(10_002);
        env_chirho.bind_chirho(
            "KProxy".to_string(),
            KindChirho::arrow_chirho(
                KindChirho::VarChirho(kproxy_kind_var_chirho),
                KindChirho::StarChirho,
            ),
        );
        let type_rep_kind_var_chirho = KindVarChirho(10_003);
        env_chirho.bind_chirho(
            "TypeRep".to_string(),
            KindChirho::arrow_chirho(
                KindChirho::VarChirho(type_rep_kind_var_chirho),
                KindChirho::StarChirho,
            ),
        );
        let equality_kind_var_chirho = KindVarChirho(10_003_1);
        env_chirho.bind_chirho(
            ":~:".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::VarChirho(equality_kind_var_chirho),
                    KindChirho::VarChirho(equality_kind_var_chirho),
                ],
                KindChirho::StarChirho,
            ),
        );
        let hetero_eq_left_kind_chirho = KindVarChirho(10_003_2);
        let hetero_eq_right_kind_chirho = KindVarChirho(10_003_3);
        env_chirho.bind_chirho(
            ":~~:".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::VarChirho(hetero_eq_left_kind_chirho),
                    KindChirho::VarChirho(hetero_eq_right_kind_chirho),
                ],
                KindChirho::StarChirho,
            ),
        );
        let const_second_kind_var_chirho = KindVarChirho(10_004);
        env_chirho.bind_chirho(
            "Const".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::StarChirho,
                    KindChirho::VarChirho(const_second_kind_var_chirho),
                ],
                KindChirho::StarChirho,
            ),
        );
        env_chirho.bind_chirho(
            "CI".to_string(),
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
        );
        let tagged_first_kind_var_chirho = KindVarChirho(10_005);
        env_chirho.bind_chirho(
            "Tagged".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::VarChirho(tagged_first_kind_var_chirho),
                    KindChirho::StarChirho,
                ],
                KindChirho::StarChirho,
            ),
        );

        // GHC.Generics representation constructors and classes.
        let rep_functor_kind_chirho =
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
        let generic_meta_kind_chirho = KindVarChirho(10_006);
        let generic_meta_kind2_chirho = KindVarChirho(10_007);
        env_chirho.bind_chirho("V1".to_string(), rep_functor_kind_chirho.clone());
        env_chirho.bind_chirho("U1".to_string(), rep_functor_kind_chirho.clone());
        env_chirho.bind_chirho("Par1".to_string(), rep_functor_kind_chirho.clone());
        env_chirho.bind_chirho(
            "Rec1".to_string(),
            KindChirho::arrow_chirho(
                rep_functor_kind_chirho.clone(),
                rep_functor_kind_chirho.clone(),
            ),
        );
        env_chirho.bind_chirho(
            "K1".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::VarChirho(generic_meta_kind_chirho),
                    KindChirho::StarChirho,
                    KindChirho::StarChirho,
                ],
                KindChirho::StarChirho,
            ),
        );
        env_chirho.bind_chirho(
            "Rec0".to_string(),
            KindChirho::arrow_n_chirho(
                vec![KindChirho::StarChirho, KindChirho::StarChirho],
                KindChirho::StarChirho,
            ),
        );
        env_chirho.bind_chirho(
            "M1".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::VarChirho(generic_meta_kind_chirho),
                    KindChirho::VarChirho(generic_meta_kind2_chirho),
                    rep_functor_kind_chirho.clone(),
                    KindChirho::StarChirho,
                ],
                KindChirho::StarChirho,
            ),
        );
        for generic_sum_name_chirho in &[":+:", ":*:", ":.:"] {
            env_chirho.bind_chirho(
                (*generic_sum_name_chirho).to_string(),
                KindChirho::arrow_n_chirho(
                    vec![
                        rep_functor_kind_chirho.clone(),
                        rep_functor_kind_chirho.clone(),
                        KindChirho::StarChirho,
                    ],
                    KindChirho::StarChirho,
                ),
            );
        }
        let rep_arg_kind_chirho = KindVarChirho(10_008);
        env_chirho.bind_chirho(
            "Rep".to_string(),
            KindChirho::arrow_chirho(
                KindChirho::VarChirho(rep_arg_kind_chirho),
                rep_functor_kind_chirho.clone(),
            ),
        );
        env_chirho.bind_chirho(
            "Rep1".to_string(),
            KindChirho::arrow_chirho(
                rep_functor_kind_chirho.clone(),
                rep_functor_kind_chirho.clone(),
            ),
        );
        env_chirho.bind_chirho(
            "Generic".to_string(),
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::ConstraintChirho),
        );
        env_chirho.bind_chirho(
            "Generic1".to_string(),
            KindChirho::arrow_chirho(rep_functor_kind_chirho, KindChirho::ConstraintChirho),
        );

        env_chirho
    }

    pub fn bind_chirho(&mut self, name_chirho: String, kind_chirho: KindChirho) {
        self.kinds_chirho.insert(name_chirho, kind_chirho);
    }

    pub fn lookup_chirho(&self, name_chirho: &str) -> Option<&KindChirho> {
        self.kinds_chirho.get(name_chirho).or_else(|| {
            name_chirho
                .rsplit_once('.')
                .and_then(|(_qualifier_chirho, bare_name_chirho)| {
                    if is_builtin_typelit_or_typenat_kind_name_chirho(bare_name_chirho) {
                        self.kinds_chirho.get(bare_name_chirho)
                    } else {
                        None
                    }
                })
        })
    }

    /// Apply a substitution to all kinds in the environment.
    pub fn apply_subst_chirho(&mut self, subst_chirho: &KindSubstChirho) {
        for kind_chirho in self.kinds_chirho.values_mut() {
            *kind_chirho = subst_chirho.apply_chirho(kind_chirho);
        }
    }
}

/// Kind inference context with fresh variable generation.
#[derive(Debug, Clone)]
struct KindTypeSynonymChirho {
    params_chirho: Vec<String>,
    rhs_chirho: TypeChirho,
}

struct KindInferCtxChirho {
    env_chirho: KindEnvChirho,
    subst_chirho: KindSubstChirho,
    next_var_chirho: u32,
    diagnostics_chirho: DiagnosticBundleChirho,
    /// Cache for PolyKinds: maps source-level kind variable names to allocated KindVarChirho.
    kind_var_cache_chirho: std::collections::HashMap<String, KindVarChirho>,
    local_kind_decl_names_chirho: std::collections::HashSet<String>,
    /// Data and newtype names declared in this module, whose kinds end in `*`
    /// (K01 sort judgments).
    local_data_type_names_chirho: std::collections::HashSet<String>,
    /// The builtin class spellings naming proved for this module, with their classes (K01 C).
    builtin_classes_chirho: HashMap<String, BuiltinClassChirho>,
    /// The builtin environment's names, before anything is learned from a use (K01 C).
    builtin_type_names_chirho: std::collections::HashSet<String>,
    type_kind_synonyms_chirho: HashMap<String, KindTypeSynonymChirho>,
    expanding_type_kind_synonyms_chirho: Vec<String>,
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
            local_kind_decl_names_chirho: std::collections::HashSet::new(),
            local_data_type_names_chirho: std::collections::HashSet::new(),
            builtin_classes_chirho: HashMap::new(),
            builtin_type_names_chirho: std::collections::HashSet::new(),
            type_kind_synonyms_chirho: HashMap::new(),
            expanding_type_kind_synonyms_chirho: Vec::new(),
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

    fn lookup_type_kind_synonym_chirho(&self, name_chirho: &str) -> Option<&KindTypeSynonymChirho> {
        self.type_kind_synonyms_chirho.get(name_chirho).or_else(|| {
            name_chirho
                .rsplit_once('.')
                .and_then(|(_qualifier_chirho, bare_chirho)| {
                    self.type_kind_synonyms_chirho.get(bare_chirho)
                })
        })
    }

    fn collect_type_app_spine_chirho(
        ty_chirho: &TypeChirho,
        args_chirho: &mut Vec<TypeChirho>,
    ) -> TypeChirho {
        match ty_chirho {
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                args_chirho.push(arg_chirho.as_ref().clone());
                Self::collect_type_app_spine_chirho(fun_chirho, args_chirho)
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                Self::collect_type_app_spine_chirho(inner_chirho, args_chirho)
            }
            _ => ty_chirho.clone(),
        }
    }

    fn expand_type_kind_synonym_once_chirho(
        &self,
        ty_chirho: &TypeChirho,
    ) -> Option<(String, TypeChirho)> {
        let mut args_chirho = Vec::new();
        let head_chirho = Self::collect_type_app_spine_chirho(ty_chirho, &mut args_chirho);
        args_chirho.reverse();

        let TypeChirho::ConChirho(name_chirho) = head_chirho else {
            return None;
        };
        let synonym_name_chirho = name_chirho.full_name_chirho();
        let synonym_chirho = self.lookup_type_kind_synonym_chirho(&synonym_name_chirho)?;
        if args_chirho.len() < synonym_chirho.params_chirho.len() {
            return None;
        }

        let mut expanded_chirho = synonym_chirho.rhs_chirho.clone();
        for (param_chirho, arg_chirho) in
            synonym_chirho.params_chirho.iter().zip(args_chirho.iter())
        {
            expanded_chirho = Self::substitute_type_kind_synonym_param_chirho(
                &expanded_chirho,
                param_chirho,
                arg_chirho,
            );
        }
        for arg_chirho in args_chirho.iter().skip(synonym_chirho.params_chirho.len()) {
            expanded_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(expanded_chirho),
                arg_chirho: Box::new(arg_chirho.clone()),
                span_chirho: ty_chirho.span_chirho(),
            };
        }

        if &expanded_chirho == ty_chirho {
            None
        } else {
            Some((synonym_name_chirho, expanded_chirho))
        }
    }

    fn substitute_type_kind_synonym_param_chirho(
        ty_chirho: &TypeChirho,
        param_chirho: &str,
        arg_chirho: &TypeChirho,
    ) -> TypeChirho {
        match ty_chirho {
            TypeChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == param_chirho => {
                arg_chirho.clone()
            }
            TypeChirho::VarChirho(_)
            | TypeChirho::ConChirho(_)
            | TypeChirho::WildcardChirho { .. }
            | TypeChirho::LitChirho { .. } => ty_chirho.clone(),
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho: inner_arg_chirho,
                span_chirho,
            } => TypeChirho::AppChirho {
                fun_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    fun_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                arg_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    inner_arg_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                span_chirho: *span_chirho,
            },
            TypeChirho::FunChirho {
                arg_chirho: fun_arg_chirho,
                mult_chirho,
                result_chirho,
                span_chirho,
            } => TypeChirho::FunChirho {
                arg_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    fun_arg_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                mult_chirho: mult_chirho.clone(),
                result_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    result_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                span_chirho: *span_chirho,
            },
            TypeChirho::TupleChirho {
                elements_chirho,
                span_chirho,
            } => TypeChirho::TupleChirho {
                elements_chirho: elements_chirho
                    .iter()
                    .map(|element_chirho| {
                        Self::substitute_type_kind_synonym_param_chirho(
                            element_chirho,
                            param_chirho,
                            arg_chirho,
                        )
                    })
                    .collect(),
                span_chirho: *span_chirho,
            },
            TypeChirho::ListChirho {
                element_chirho,
                span_chirho,
            } => TypeChirho::ListChirho {
                element_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    element_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                span_chirho: *span_chirho,
            },
            TypeChirho::ParenChirho {
                inner_chirho,
                span_chirho,
            } => TypeChirho::ParenChirho {
                inner_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    inner_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                span_chirho: *span_chirho,
            },
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                span_chirho,
            } => TypeChirho::QualChirho {
                context_chirho: context_chirho
                    .iter()
                    .map(|constraint_chirho| {
                        Self::substitute_constraint_kind_synonym_param_chirho(
                            constraint_chirho,
                            param_chirho,
                            arg_chirho,
                        )
                    })
                    .collect(),
                body_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                    body_chirho,
                    param_chirho,
                    arg_chirho,
                )),
                span_chirho: *span_chirho,
            },
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho,
            } => {
                if vars_chirho
                    .iter()
                    .any(|var_chirho| var_chirho.text_chirho() == param_chirho)
                {
                    ty_chirho.clone()
                } else {
                    TypeChirho::ForallChirho {
                        vars_chirho: vars_chirho.clone(),
                        body_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                            body_chirho,
                            param_chirho,
                            arg_chirho,
                        )),
                        span_chirho: *span_chirho,
                    }
                }
            }
            TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                span_chirho,
            } => {
                if vars_chirho
                    .iter()
                    .any(|var_chirho| var_chirho.text_chirho() == param_chirho)
                {
                    ty_chirho.clone()
                } else {
                    TypeChirho::RequiredForallChirho {
                        vars_chirho: vars_chirho.clone(),
                        body_chirho: Box::new(Self::substitute_type_kind_synonym_param_chirho(
                            body_chirho,
                            param_chirho,
                            arg_chirho,
                        )),
                        span_chirho: *span_chirho,
                    }
                }
            }
            TypeChirho::PromotedConChirho { .. } => ty_chirho.clone(),
            TypeChirho::PromotedListChirho {
                elements_chirho,
                span_chirho,
            } => TypeChirho::PromotedListChirho {
                elements_chirho: elements_chirho
                    .iter()
                    .map(|element_chirho| {
                        Self::substitute_type_kind_synonym_param_chirho(
                            element_chirho,
                            param_chirho,
                            arg_chirho,
                        )
                    })
                    .collect(),
                span_chirho: *span_chirho,
            },
        }
    }

    fn substitute_constraint_kind_synonym_param_chirho(
        constraint_chirho: &ConstraintChirho,
        param_chirho: &str,
        arg_chirho: &TypeChirho,
    ) -> ConstraintChirho {
        match constraint_chirho {
            ConstraintChirho::ClassChirho {
                class_chirho,
                args_chirho,
                span_chirho,
            } => ConstraintChirho::ClassChirho {
                class_chirho: class_chirho.clone(),
                args_chirho: args_chirho
                    .iter()
                    .map(|arg_ty_chirho| {
                        Self::substitute_type_kind_synonym_param_chirho(
                            arg_ty_chirho,
                            param_chirho,
                            arg_chirho,
                        )
                    })
                    .collect(),
                span_chirho: *span_chirho,
            },
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                span_chirho,
            } => {
                if vars_chirho
                    .iter()
                    .any(|var_chirho| var_chirho.text_chirho() == param_chirho)
                {
                    constraint_chirho.clone()
                } else {
                    ConstraintChirho::QuantifiedChirho {
                        vars_chirho: vars_chirho.clone(),
                        context_chirho: context_chirho
                            .iter()
                            .map(|inner_chirho| {
                                Self::substitute_constraint_kind_synonym_param_chirho(
                                    inner_chirho,
                                    param_chirho,
                                    arg_chirho,
                                )
                            })
                            .collect(),
                        body_chirho: Box::new(
                            Self::substitute_constraint_kind_synonym_param_chirho(
                                body_chirho,
                                param_chirho,
                                arg_chirho,
                            ),
                        ),
                        span_chirho: *span_chirho,
                    }
                }
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

    /// Number of arguments a kind takes before reaching its result, and whether
    /// that count is final. Only a variable in the RESULT position leaves the
    /// arity open (it may still be instantiated to another arrow); a variable
    /// in an argument position is just an un-annotated parameter — `class C a b`
    /// has arity 2 no matter what kinds `a` and `b` turn out to have.
    fn kind_arity_chirho(kind_chirho: &KindChirho) -> (usize, bool) {
        let mut arity_chirho = 0;
        let mut cursor_chirho = kind_chirho;
        loop {
            match cursor_chirho {
                KindChirho::ArrowChirho(_, result_chirho) => {
                    arity_chirho += 1;
                    cursor_chirho = result_chirho;
                }
                KindChirho::VarChirho(_) => return (arity_chirho, false),
                _ => return (arity_chirho, true),
            }
        }
    }

    /// Whether a kind mentions any unsolved kind variable.
    fn kind_contains_var_chirho(kind_chirho: &KindChirho) -> bool {
        match kind_chirho {
            KindChirho::VarChirho(_) => true,
            KindChirho::ArrowChirho(arg_chirho, result_chirho) => {
                Self::kind_contains_var_chirho(arg_chirho)
                    || Self::kind_contains_var_chirho(result_chirho)
            }
            _ => false,
        }
    }

    /// Report an instance head that applies its class to the wrong number of
    /// arguments (`class MonadReader a b` + `instance MonadReader Int`), which
    /// GHC rejects with "Expecting one more argument to ...".
    ///
    /// Deliberately narrow: the class must be declared in THIS module (an
    /// imported class may arrive through a placeholder interface whose kind we
    /// do not really know) and its kind must be variable-free, so a poly-kinded
    /// or still-inferring class is never judged.
    fn check_instance_head_arity_chirho(
        &mut self,
        class_name_chirho: &str,
        head_types_chirho: &[TypeChirho],
        head_forms_representable_chirho: bool,
        span_chirho: SpanChirho,
    ) {
        let head_arg_count_chirho = head_types_chirho.len();
        // Only classes declared in THIS module: an imported class may arrive
        // through a placeholder interface whose kind we do not really know.
        // (Widening this to any env-known class was measured and gained
        // nothing — builtin classes like Functor carry no usable kind here —
        // so the narrow form is kept.)
        if !self
            .local_kind_decl_names_chirho
            .contains(class_name_chirho)
        {
            return;
        }
        let Some(class_kind_chirho) = self.env_chirho.lookup_chirho(class_name_chirho).cloned()
        else {
            return;
        };
        let resolved_kind_chirho = self.subst_chirho.apply_chirho(&class_kind_chirho);
        let (expected_chirho, spine_known_chirho) = Self::kind_arity_chirho(&resolved_kind_chirho);
        if !spine_known_chirho {
            return;
        }
        if expected_chirho == head_arg_count_chirho {
            if head_forms_representable_chirho {
                self.check_instance_head_arg_kinds_chirho(
                    &resolved_kind_chirho,
                    head_types_chirho,
                    span_chirho,
                );
            }
            return;
        }
        // A head form our lowering drops can only make the head look SHORTER
        // than written, never longer — so over-application stays sound even
        // when some head arguments may be unrepresented.
        if !head_forms_representable_chirho && head_arg_count_chirho < expected_chirho {
            return;
        }
        let message_chirho = if head_arg_count_chirho < expected_chirho {
            format!(
                "expecting {} more argument{} to `{class_name_chirho}` in the instance head",
                expected_chirho - head_arg_count_chirho,
                if expected_chirho - head_arg_count_chirho == 1 {
                    ""
                } else {
                    "s"
                },
            )
        } else {
            format!(
                "`{class_name_chirho}` is applied to {head_arg_count_chirho} arguments, but it takes {expected_chirho}",
            )
        };
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                message_chirho,
                span_chirho,
            ));
    }

    /// With the head arity already correct, check each head argument's kind
    /// against the class parameter it fills (`class C (f :: * -> *)` rejects
    /// `instance C Bool`). Only compares pairs where BOTH kinds are fully
    /// resolved and variable-free, so an un-annotated or poly-kinded parameter
    /// is never judged.
    fn check_instance_head_arg_kinds_chirho(
        &mut self,
        class_kind_chirho: &KindChirho,
        head_types_chirho: &[TypeChirho],
        span_chirho: SpanChirho,
    ) {
        let mut param_kinds_chirho = Vec::new();
        let mut cursor_chirho = class_kind_chirho;
        while let KindChirho::ArrowChirho(arg_chirho, result_chirho) = cursor_chirho {
            param_kinds_chirho.push((**arg_chirho).clone());
            cursor_chirho = result_chirho;
        }
        let env_keys_before_chirho: std::collections::HashSet<String> =
            self.env_chirho.kinds_chirho.keys().cloned().collect();
        for (param_kind_chirho, head_ty_chirho) in
            param_kinds_chirho.iter().zip(head_types_chirho.iter())
        {
            if Self::kind_contains_var_chirho(param_kind_chirho) {
                continue;
            }
            let head_kind_chirho = self.infer_type_kind_chirho(head_ty_chirho);
            let head_kind_chirho = self.subst_chirho.apply_chirho(&head_kind_chirho);
            if Self::kind_contains_var_chirho(&head_kind_chirho)
                || &head_kind_chirho == param_kind_chirho
            {
                continue;
            }
            self.diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                    format!(
                        "expected kind `{param_kind_chirho}` in the instance head, but the argument has kind `{head_kind_chirho}`",
                    ),
                    span_chirho,
                ));
        }
        let keys_to_remove_chirho: Vec<String> = self
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
            self.env_chirho.kinds_chirho.remove(&key_chirho);
        }
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
        self.type_kind_synonyms_chirho.insert(
            name_chirho.to_string(),
            KindTypeSynonymChirho {
                params_chirho: type_vars_chirho
                    .iter()
                    .map(|type_var_chirho| type_var_chirho.text_chirho().to_string())
                    .collect(),
                rhs_chirho: rhs_ty_chirho.clone(),
            },
        );
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
        AstKindChirho::AppChirho(_, _) => KindChirho::StarChirho,
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

pub use module_pass_chirho::{
    KindImportsChirho, KindResultChirho, infer_module_kinds_chirho,
    infer_module_kinds_with_imports_chirho,
};
