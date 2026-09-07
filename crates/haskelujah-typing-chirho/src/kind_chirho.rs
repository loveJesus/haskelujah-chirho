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
        if let Some((name_chirho, expanded_chirho)) =
            self.expand_type_kind_synonym_once_chirho(ty_chirho)
        {
            if !self
                .expanding_type_kind_synonyms_chirho
                .contains(&name_chirho)
            {
                self.expanding_type_kind_synonyms_chirho
                    .push(name_chirho.clone());
                let kind_chirho = self.type_to_kind_chirho(&expanded_chirho);
                self.expanding_type_kind_synonyms_chirho.pop();
                return kind_chirho;
            }
        }

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
                    self.env_chirho.bind_chirho(text_chirho, k_chirho.clone());
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
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                for v_chirho in vars_chirho {
                    let k_chirho = if let Some(ann_chirho) = &v_chirho.kind_annotation_chirho {
                        self.ast_kind_to_kind_ctx_chirho(ann_chirho)
                    } else {
                        self.fresh_kind_chirho()
                    };
                    self.env_chirho
                        .bind_chirho(v_chirho.text_chirho().to_string(), k_chirho);
                }
                self.type_to_kind_chirho(body_chirho)
            }
            _ => {
                // Fallback: treat unknown shapes as *.
                KindChirho::StarChirho
            }
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
                    // Poly-kinded constructors instantiate at each use site.
                    // This applies to local classes too: sharing `Forall`'s
                    // kind variables across all superclass/signature uses
                    // incorrectly monomorphizes quantified-constraint helpers
                    // such as `ForallF` before later `ForallT` signatures.
                    if !k_chirho.free_vars_chirho().is_empty() {
                        self.instantiate_kind_chirho(&k_chirho)
                    } else {
                        k_chirho
                    }
                } else {
                    // Unknown type constructor — assign a fresh kind variable.
                    let k_chirho = self.fresh_kind_chirho();
                    self.env_chirho.bind_chirho(text_chirho, k_chirho.clone());
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
            }
            | TypeChirho::RequiredForallChirho {
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
                    let k_chirho = k_chirho.clone();
                    if k_chirho == KindChirho::StarChirho {
                        self.fresh_kind_chirho()
                    } else {
                        k_chirho
                    }
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

    /// GHC-55233: a `data`/`newtype` declaration's **return kind** may not be
    /// `Constraint`. GHC rejects `data Foo :: Constraint` unconditionally — no
    /// extension licenses it — while it ACCEPTS `data Foo (_ :: Constraint)`,
    /// whose `Constraint` is a binder's kind, not the declaration's.
    ///
    /// This check was unwritable until the data-head binder fix: the parser
    /// recorded a binder's kind as the declaration's `kind_sig_chirho`, so the two
    /// forms lowered identically in every field a guard can read
    /// (spec-chirho/bug-data-binder-kind-misassigned-chirho.md, four refuted
    /// narrowings). They are distinct now, so the guard is just the arrow tail.
    ///
    /// A module that declares its own type named `Constraint` shadows the wired-in
    /// one, so the check stands down there.
    fn declared_return_kind_is_constraint_chirho(&self, sig_chirho: &TypeChirho) -> bool {
        if self.local_kind_decl_names_chirho.contains("Constraint") {
            return false;
        }
        let mut tail_chirho = sig_chirho;
        loop {
            match tail_chirho {
                TypeChirho::FunChirho { result_chirho, .. } => tail_chirho = result_chirho,
                TypeChirho::ParenChirho { inner_chirho, .. } => tail_chirho = inner_chirho,
                TypeChirho::ForallChirho { body_chirho, .. } => tail_chirho = body_chirho,
                _ => break,
            }
        }
        matches!(
            tail_chirho,
            TypeChirho::ConChirho(n_chirho) if n_chirho.text_chirho() == "Constraint"
        )
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
            if self.declared_return_kind_is_constraint_chirho(sig_chirho) {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                        format!(
                            "data type `{name_chirho}` has non-`*` return kind `Constraint`"
                        ),
                        span_chirho,
                    ));
            }
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

    /// Process a type family declaration to determine the kind of the family.
    fn infer_type_family_decl_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[TyVarChirho],
        result_kind_chirho: Option<&TypeChirho>,
        span_chirho: SpanChirho,
        poly_kinds_enabled_chirho: bool,
    ) {
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

        let result_kind_chirho = result_kind_chirho
            .map(|kind_ty_chirho| self.type_to_kind_chirho(kind_ty_chirho))
            .unwrap_or_else(|| {
                if poly_kinds_enabled_chirho {
                    self.fresh_kind_chirho()
                } else {
                    KindChirho::StarChirho
                }
            });
        let kind_chirho = KindChirho::arrow_n_chirho(param_kinds_chirho, result_kind_chirho);

        if let Some(existing_chirho) = self.env_chirho.lookup_chirho(name_chirho) {
            let existing_chirho = existing_chirho.clone();
            self.unify_chirho(
                &existing_chirho,
                &kind_chirho,
                "type family declaration",
                span_chirho,
            );
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
    let local_kind_decl_names_chirho: std::collections::HashSet<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho { name_chirho, .. }
            | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
            | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
            | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. }
            | DeclChirho::ClassDeclChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect();
    for local_kind_decl_name_chirho in &local_kind_decl_names_chirho {
        ctx_chirho
            .env_chirho
            .kinds_chirho
            .remove(local_kind_decl_name_chirho);
    }
    ctx_chirho.local_kind_decl_names_chirho = local_kind_decl_names_chirho.clone();
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
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                type_vars_chirho,
                result_kind_chirho,
                span_chirho,
                ..
            } => {
                ctx_chirho.infer_type_family_decl_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    result_kind_chirho.as_ref(),
                    *span_chirho,
                    poly_kinds_enabled_chirho,
                );
            }
            DeclChirho::ClassDeclChirho {
                context_chirho,
                name_chirho,
                type_vars_chirho,
                methods_chirho,
                associated_tfs_chirho,
                span_chirho,
                ..
            } => {
                ctx_chirho.infer_class_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    *span_chirho,
                );
                for assoc_tf_chirho in associated_tfs_chirho {
                    let mut param_kinds_chirho = Vec::new();
                    for type_var_name_chirho in &assoc_tf_chirho.type_vars_chirho {
                        let text_chirho = type_var_name_chirho.text_chirho();
                        let kind_chirho = if let Some(existing_chirho) =
                            ctx_chirho.env_chirho.lookup_chirho(text_chirho)
                        {
                            existing_chirho.clone()
                        } else {
                            let fresh_kind_chirho = ctx_chirho.fresh_kind_chirho();
                            ctx_chirho
                                .env_chirho
                                .bind_chirho(text_chirho.to_string(), fresh_kind_chirho.clone());
                            fresh_kind_chirho
                        };
                        param_kinds_chirho.push(kind_chirho);
                    }

                    let result_kind_chirho =
                        if let Some(default_rhs_chirho) = &assoc_tf_chirho.default_rhs_chirho {
                            ctx_chirho.infer_type_kind_chirho(default_rhs_chirho)
                        } else {
                            ctx_chirho.fresh_kind_chirho()
                        };

                    let family_kind_chirho =
                        KindChirho::arrow_n_chirho(param_kinds_chirho, result_kind_chirho.clone());
                    let assoc_name_text_chirho = assoc_tf_chirho.name_chirho.text_chirho();
                    ctx_chirho
                        .env_chirho
                        .bind_chirho(assoc_name_text_chirho.to_string(), family_kind_chirho);
                }
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

    // Phase 1.5: instance heads. Runs as its own pass so every class kind is
    // established regardless of declaration order (an instance may precede its
    // class in the source). Instance heads were never kind-checked at all, so
    // `class MonadReader a b` + `instance MonadReader Int` was accepted.
    //
    // Under-application is only reported when every head form is one our
    // lowering represents faithfully. It is NOT, under these extensions: a head
    // argument may be a type-level literal (`instance A 0`), a promoted list
    // (`instance All c '[]`), an operator type (`instance Category (->)`), or a
    // variable-headed application (`instance MonadReader r (Reader r)`, which
    // lowers to ONE argument instead of two). None of those reach
    // `types_chirho`, so the head looks short and we would report OUR gap as
    // the program's error. Over-application stays sound either way — a dropped
    // argument can only shorten the head.
    let instance_head_forms_representable_chirho =
        !module_chirho.extensions_chirho.iter().any(|e_chirho| {
            e_chirho == "DataKinds"
                || e_chirho == "PolyKinds"
                || e_chirho == "TypeInType"
                || e_chirho == "TypeOperators"
                || e_chirho == "FlexibleInstances"
        });
    for decl_chirho in &module_chirho.decls_chirho {
        let DeclChirho::InstanceDeclChirho {
            class_chirho,
            types_chirho,
            span_chirho,
            ..
        } = decl_chirho
        else {
            continue;
        };
        ctx_chirho.check_instance_head_arity_chirho(
            class_chirho.text_chirho(),
            types_chirho,
            instance_head_forms_representable_chirho,
            *span_chirho,
        );
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
        AssocTypeFamilyChirho, AstKindChirho, ClassMethodChirho, ConDeclChirho, DeclChirho,
        StrictnessChirho, TyVarChirho,
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
            env_chirho.lookup_chirho("Int64"),
            Some(&KindChirho::StarChirho)
        );
        assert_eq!(
            env_chirho.lookup_chirho("Word32"),
            Some(&KindChirho::StarChirho)
        );
        assert_eq!(
            env_chirho.lookup_chirho("Word64"),
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
        assert_eq!(
            env_chirho.lookup_chirho("GHC.TypeNats.*"),
            Some(&KindChirho::arrow_n_chirho(
                vec![KindChirho::StarChirho, KindChirho::StarChirho],
                KindChirho::StarChirho
            )),
            "qualified TypeNats operator lookup should reuse the bare builtin family kind"
        );
        assert_eq!(
            env_chirho.lookup_chirho("AppendSymbol"),
            Some(&KindChirho::arrow_n_chirho(
                vec![KindChirho::StarChirho, KindChirho::StarChirho],
                KindChirho::StarChirho
            ))
        );
        for family_chirho in ["CharToNat", "GHC.TypeLits.NatToChar"] {
            assert_eq!(
                env_chirho.lookup_chirho(family_chirho),
                Some(&KindChirho::arrow_chirho(
                    KindChirho::StarChirho,
                    KindChirho::StarChirho
                )),
                "{family_chirho} should have a unary TypeLits family kind"
            );
        }
        assert!(
            matches!(
                env_chirho.lookup_chirho(":"),
                Some(KindChirho::ArrowChirho(_, tail_chirho))
                    if matches!(
                        tail_chirho.as_ref(),
                        KindChirho::ArrowChirho(tail_arg_chirho, result_chirho)
                            if matches!(tail_arg_chirho.as_ref(), KindChirho::StarChirho)
                                && matches!(result_chirho.as_ref(), KindChirho::StarChirho)
                    )
            ),
            "promoted list cons should accept an element and flattened tail"
        );
    }

    // -- Module-level kind inference tests --

    #[test]
    fn data_return_kind_constraint_is_rejected_chirho() {
        // GHC-55233: `data Foo :: Constraint` — unconditionally rejected, no
        // extension licenses it.
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Foo"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: Some(TypeChirho::ConChirho(mk_name_chirho("Constraint"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);
        assert!(
            infer_module_kinds_chirho(&module_chirho)
                .diagnostics_chirho
                .has_errors_chirho(),
            "a data declaration returning Constraint must be rejected"
        );
    }

    #[test]
    fn data_binder_kind_constraint_is_accepted_chirho() {
        // `data Foo (_ :: Constraint)` — GHC ACCEPTS this: the Constraint is the
        // BINDER's kind. Before the data-head binder fix this lowered identically to
        // the case above, which is why the check could not be written.
        let mut binder_chirho: TyVarChirho = mk_name_chirho("_").into();
        binder_chirho.kind_annotation_chirho = Some(AstKindChirho::ConstraintChirho);
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Foo"),
            type_vars_chirho: vec![binder_chirho],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);
        assert!(
            !infer_module_kinds_chirho(&module_chirho)
                .diagnostics_chirho
                .has_errors_chirho(),
            "a BINDER of kind Constraint is legal and must stay accepted"
        );
    }

    #[test]
    fn data_return_kind_constraint_stands_down_when_shadowed_chirho() {
        // A module declaring its own `Constraint` shadows the wired-in one.
        let module_chirho = mk_module_chirho(vec![
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("Constraint"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("Foo"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
                kind_sig_chirho: Some(TypeChirho::ConChirho(mk_name_chirho("Constraint"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);
        assert!(
            !infer_module_kinds_chirho(&module_chirho)
                .diagnostics_chirho
                .has_errors_chirho(),
            "a locally declared Constraint shadows the wired-in kind"
        );
    }

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
    fn rep_kind_accepts_higher_kinded_argument_chirho() {
        let env_chirho = KindEnvChirho::with_builtins_chirho();
        let mut ctx_chirho = KindInferCtxChirho::new_chirho(env_chirho);
        ctx_chirho.env_chirho.bind_chirho(
            "QChirho".to_string(),
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
            fun_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Rep"))),
            arg_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("QChirho"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let kind_chirho = ctx_chirho.infer_type_kind_chirho(&ty_chirho);
        let normalized_kind_chirho = ctx_chirho.subst_chirho.apply_chirho(&kind_chirho);
        assert_eq!(
            normalized_kind_chirho,
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho)
        );
        assert!(
            !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
            "Rep should accept higher-kinded arguments without diagnostics: {:?}",
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
    fn local_tagged_decl_shadows_builtin_tagged_kind_chirho() {
        let module_chirho = mk_module_chirho(vec![
            DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: mk_name_chirho("SumSize"),
                type_vars_chirho: vec![mk_name_chirho("f").into()],
                methods_chirho: vec![ClassMethodChirho {
                    name_chirho: mk_name_chirho("sumSize"),
                    ty_chirho: mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("Tagged")),
                        TypeChirho::VarChirho(mk_name_chirho("f")),
                    ),
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::NewtypeDeclChirho {
                name_chirho: mk_name_chirho("Tagged"),
                type_vars_chirho: vec![TyVarChirho::annotated_chirho(
                    mk_name_chirho("s"),
                    AstKindChirho::ArrowChirho(
                        Box::new(AstKindChirho::StarChirho),
                        Box::new(AstKindChirho::StarChirho),
                    ),
                )],
                constructor_chirho: ConDeclChirho::RecordChirho {
                    name_chirho: mk_name_chirho("Tagged"),
                    fields_chirho: vec![haskelujah_ast_chirho::decl_chirho::FieldDeclChirho {
                        names_chirho: vec![mk_name_chirho("unTagged")],
                        strictness_chirho: StrictnessChirho::LazyChirho,
                        ty_chirho: TypeChirho::ConChirho(mk_name_chirho("Int")),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "local Tagged should shadow the builtin Tagged kind: {:?}",
            result_chirho.diagnostics_chirho
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("SumSize"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
                KindChirho::ConstraintChirho
            ))
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Tagged"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
                KindChirho::StarChirho
            ))
        );
    }

    #[test]
    fn class_associated_type_family_shadows_builtin_rep_chirho() {
        let module_chirho = mk_module_chirho(vec![DeclChirho::ClassDeclChirho {
            context_chirho: vec![ConstraintChirho::ClassChirho {
                class_chirho: mk_name_chirho("Contravariant"),
                args_chirho: vec![TypeChirho::VarChirho(mk_name_chirho("f"))],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            name_chirho: mk_name_chirho("Representable"),
            type_vars_chirho: vec![mk_name_chirho("f").into()],
            methods_chirho: vec![
                ClassMethodChirho {
                    name_chirho: mk_name_chirho("tabulate"),
                    ty_chirho: mk_fun_chirho(
                        mk_fun_chirho(
                            TypeChirho::VarChirho(mk_name_chirho("a")),
                            mk_app_chirho(
                                TypeChirho::ConChirho(mk_name_chirho("Rep")),
                                TypeChirho::VarChirho(mk_name_chirho("f")),
                            ),
                        ),
                        mk_app_chirho(
                            TypeChirho::VarChirho(mk_name_chirho("f")),
                            TypeChirho::VarChirho(mk_name_chirho("a")),
                        ),
                    ),
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                ClassMethodChirho {
                    name_chirho: mk_name_chirho("index"),
                    ty_chirho: mk_fun_chirho(
                        mk_app_chirho(
                            TypeChirho::VarChirho(mk_name_chirho("f")),
                            TypeChirho::VarChirho(mk_name_chirho("a")),
                        ),
                        mk_fun_chirho(
                            TypeChirho::VarChirho(mk_name_chirho("a")),
                            mk_app_chirho(
                                TypeChirho::ConChirho(mk_name_chirho("Rep")),
                                TypeChirho::VarChirho(mk_name_chirho("f")),
                            ),
                        ),
                    ),
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            associated_tfs_chirho: vec![AssocTypeFamilyChirho {
                name_chirho: mk_name_chirho("Rep"),
                type_vars_chirho: vec![mk_name_chirho("f")],
                default_rhs_chirho: None,
                default_params_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "associated Rep family should kind-check inside class methods: {:?}",
            result_chirho.diagnostics_chirho
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Rep"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
                KindChirho::StarChirho
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
    fn standalone_kind_signature_expands_local_kind_synonym_application_chirho() {
        let module_chirho = mk_module_chirho(vec![
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("Cat"),
                type_vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("k"))],
                rhs_chirho: mk_fun_chirho(
                    TypeChirho::VarChirho(mk_name_chirho("k")),
                    mk_fun_chirho(
                        TypeChirho::VarChirho(mk_name_chirho("k")),
                        TypeChirho::ConChirho(mk_name_chirho("Type")),
                    ),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("FreeCat"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
                kind_sig_chirho: Some(mk_fun_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("Cat")),
                        TypeChirho::VarChirho(mk_name_chirho("k")),
                    ),
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("Cat")),
                        TypeChirho::VarChirho(mk_name_chirho("k")),
                    ),
                )),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "local kind synonym applications should expand in standalone kind signatures: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("FreeCat"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::arrow_chirho(
                    KindChirho::StarChirho,
                    KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho)
                ),
                KindChirho::arrow_chirho(
                    KindChirho::StarChirho,
                    KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho)
                )
            ))
        );
    }

    #[test]
    fn type_alias_visible_kind_binder_proxy_function_rhs_chirho() {
        let module_chirho = mk_module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("S"),
            type_vars_chirho: vec![
                TyVarChirho::annotated_chirho(mk_name_chirho("k"), AstKindChirho::StarChirho),
                TyVarChirho::annotated_chirho(
                    mk_name_chirho("a"),
                    AstKindChirho::VarChirho("k".to_string()),
                ),
            ],
            rhs_chirho: mk_fun_chirho(
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("Proxy")),
                    TypeChirho::VarChirho(mk_name_chirho("a")),
                ),
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("Proxy")),
                    TypeChirho::VarChirho(mk_name_chirho("k")),
                ),
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "proxy function alias should kind-check: {:?}",
            result_chirho.diagnostics_chirho
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
    fn builtin_typeable_accepts_higher_kinded_argument_chirho() {
        let module_chirho = mk_module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("Typeable1Chirho"),
            type_vars_chirho: vec![],
            rhs_chirho: TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Typeable"))),
                arg_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Maybe"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Typeable should accept higher-kinded arguments: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("Typeable1Chirho"),
            Some(&KindChirho::ConstraintChirho)
        );
    }

    #[test]
    fn standalone_forall_kind_signature_preserves_arrow_kind_chirho() {
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("AppChirho"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: Some(TypeChirho::ForallChirho {
                vars_chirho: vec![TyVarChirho::annotated_chirho(
                    mk_name_chirho("fChirho"),
                    AstKindChirho::ArrowChirho(
                        Box::new(AstKindChirho::StarChirho),
                        Box::new(AstKindChirho::StarChirho),
                    ),
                )],
                body_chirho: Box::new(mk_fun_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("Type")),
                    TypeChirho::ConChirho(mk_name_chirho("Type")),
                )),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "forall standalone kind signature should kind-check: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("AppChirho"),
            Some(&KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::StarChirho
            ))
        );
    }

    #[test]
    fn type_family_decl_result_kind_guides_later_applications_chirho() {
        let t_var_chirho = TyVarChirho::plain_chirho(mk_name_chirho("t"));
        let mut module_chirho = mk_module_chirho(vec![
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho: mk_name_chirho("TrivialFamily"),
                type_vars_chirho: vec![t_var_chirho.clone()],
                result_kind_chirho: Some(TypeChirho::ConChirho(mk_name_chirho("Type"))),
                equations_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("ProblemTypeChirho"),
                type_vars_chirho: vec![t_var_chirho],
                rhs_chirho: mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("Proxy")),
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("TrivialFamily")),
                        TypeChirho::VarChirho(mk_name_chirho("t")),
                    ),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);
        module_chirho
            .extensions_chirho
            .push("PolyKinds".to_string());

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "type family result kind should make later applications kind-check: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn polykinds_unannotated_type_family_result_stays_flexible_chirho() {
        let mut module_chirho = mk_module_chirho(vec![
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho: mk_name_chirho("FamilyChirho"),
                type_vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
                result_kind_chirho: None,
                equations_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("IndexedChirho"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
                kind_sig_chirho: Some(mk_fun_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("FamilyChirho")),
                        TypeChirho::VarChirho(mk_name_chirho("k")),
                    ),
                    TypeChirho::ConChirho(mk_name_chirho("Type")),
                )),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeSigChirho {
                name_chirho: mk_name_chirho("useIndexedChirho"),
                ty_chirho: mk_fun_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("IndexedChirho")),
                        TypeChirho::ConChirho(mk_name_chirho("Maybe")),
                    ),
                    mk_fun_chirho(
                        mk_app_chirho(
                            TypeChirho::ConChirho(mk_name_chirho("IndexedChirho")),
                            TypeChirho::ConChirho(mk_name_chirho("Int")),
                        ),
                        TypeChirho::ConChirho(mk_name_chirho("Type")),
                    ),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);
        module_chirho
            .extensions_chirho
            .push("PolyKinds".to_string());

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "unannotated PolyKinds type-family result should remain flexible at use sites: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn promoted_constructor_does_not_reuse_same_name_type_constructor_kind_chirho() {
        let module_chirho = mk_module_chirho(vec![
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("R"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("PromotedRAppChirho"),
                type_vars_chirho: vec![],
                rhs_chirho: mk_app_chirho(
                    TypeChirho::PromotedConChirho {
                        name_chirho: mk_name_chirho("R"),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    TypeChirho::ConChirho(mk_name_chirho("Int")),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "promoted constructor should not reuse its same-name type constructor's `*` kind: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("R"),
            Some(&KindChirho::StarChirho)
        );
    }

    #[test]
    fn promoted_list_cons_accepts_polykinded_head_chirho() {
        let mut module_chirho = mk_module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("ConsMaybeChirho"),
            type_vars_chirho: vec![],
            rhs_chirho: mk_app_chirho(
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho(":")),
                    TypeChirho::ConChirho(mk_name_chirho("Maybe")),
                ),
                TypeChirho::PromotedListChirho {
                    elements_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);
        module_chirho
            .extensions_chirho
            .push("PolyKinds".to_string());

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "promoted list cons should not force its head to kind `*`: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn gadt_return_type_accepts_promoted_list_cons_chirho() {
        let l_chirho = TypeChirho::VarChirho(mk_name_chirho("l"));
        let ls_chirho = TypeChirho::VarChirho(mk_name_chirho("ls"));
        let t_chirho = TypeChirho::VarChirho(mk_name_chirho("t"));
        let promoted_cons_chirho = mk_app_chirho(
            mk_app_chirho(TypeChirho::ConChirho(mk_name_chirho(":")), l_chirho.clone()),
            ls_chirho.clone(),
        );
        let stack_ls_t_chirho = mk_app_chirho(
            mk_app_chirho(TypeChirho::ConChirho(mk_name_chirho("Stack")), ls_chirho),
            t_chirho.clone(),
        );
        let stack_cons_t_chirho = mk_app_chirho(
            mk_app_chirho(
                TypeChirho::ConChirho(mk_name_chirho("Stack")),
                promoted_cons_chirho,
            ),
            t_chirho.clone(),
        );
        let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Stack"),
            type_vars_chirho: vec![
                TyVarChirho::plain_chirho(mk_name_chirho("lrs")),
                TyVarChirho::annotated_chirho(
                    mk_name_chirho("t"),
                    AstKindChirho::ArrowChirho(
                        Box::new(AstKindChirho::StarChirho),
                        Box::new(AstKindChirho::StarChirho),
                    ),
                ),
            ],
            constructors_chirho: vec![ConDeclChirho::GadtChirho {
                name_chirho: mk_name_chirho("SLayer"),
                ty_chirho: mk_fun_chirho(
                    mk_app_chirho(t_chirho, l_chirho),
                    mk_fun_chirho(stack_ls_t_chirho, stack_cons_t_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "GADT return type should accept `Stack (l ': ls) t`: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn builtin_ghc_generics_representations_accept_partial_apps_chirho() {
        let unit_ty_chirho = TypeChirho::TupleChirho {
            elements_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = mk_module_chirho(vec![
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("GK1Chirho"),
                type_vars_chirho: vec![],
                rhs_chirho: mk_app_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("K1")),
                        unit_ty_chirho.clone(),
                    ),
                    TypeChirho::ConChirho(mk_name_chirho("Int")),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("GM1Chirho"),
                type_vars_chirho: vec![],
                rhs_chirho: mk_app_chirho(
                    mk_app_chirho(
                        mk_app_chirho(
                            TypeChirho::ConChirho(mk_name_chirho("M1")),
                            unit_ty_chirho.clone(),
                        ),
                        unit_ty_chirho.clone(),
                    ),
                    TypeChirho::ConChirho(mk_name_chirho("U1")),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("GSumChirho"),
                type_vars_chirho: vec![],
                rhs_chirho: mk_app_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho(":+:")),
                        TypeChirho::ConChirho(mk_name_chirho("U1")),
                    ),
                    TypeChirho::ConChirho(mk_name_chirho("U1")),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: mk_name_chirho("GProdChirho"),
                type_vars_chirho: vec![],
                rhs_chirho: mk_app_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho(":*:")),
                        TypeChirho::ConChirho(mk_name_chirho("U1")),
                    ),
                    TypeChirho::ConChirho(mk_name_chirho("U1")),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);

        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "GHC.Generics representation constructors should kind-check when partially applied: {:?}",
            result_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|diagnostic_chirho| diagnostic_chirho.to_string())
                .collect::<Vec<_>>()
        );
        let expected_rep_functor_kind_chirho =
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("GK1Chirho"),
            Some(&expected_rep_functor_kind_chirho)
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("GM1Chirho"),
            Some(&expected_rep_functor_kind_chirho)
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("GSumChirho"),
            Some(&expected_rep_functor_kind_chirho)
        );
        assert_eq!(
            result_chirho.env_chirho.lookup_chirho("GProdChirho"),
            Some(&expected_rep_functor_kind_chirho)
        );
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

    /// `class C a b` plus `instance C <n args>`.
    fn mk_class_and_instance_module_chirho(
        class_param_names_chirho: &[&str],
        instance_head_types_chirho: Vec<TypeChirho>,
    ) -> ModuleChirho {
        mk_module_chirho(vec![
            DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: mk_name_chirho("CChirho"),
                type_vars_chirho: class_param_names_chirho
                    .iter()
                    .map(|p_chirho| mk_name_chirho(p_chirho).into())
                    .collect(),
                methods_chirho: vec![],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::InstanceDeclChirho {
                context_chirho: vec![],
                class_chirho: mk_name_chirho("CChirho"),
                types_chirho: instance_head_types_chirho,
                methods_chirho: vec![],
                assoc_tf_instances_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ])
    }

    #[test]
    fn instance_head_under_applied_class_is_rejected_chirho() {
        let module_chirho = mk_class_and_instance_module_chirho(
            &["a", "b"],
            vec![TypeChirho::ConChirho(mk_name_chirho("Int"))],
        );
        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            result_chirho.diagnostics_chirho.has_errors_chirho(),
            "a two-parameter class applied to one argument must be rejected"
        );
    }

    #[test]
    fn instance_head_over_applied_class_is_rejected_chirho() {
        let module_chirho = mk_class_and_instance_module_chirho(
            &["a"],
            vec![
                TypeChirho::ConChirho(mk_name_chirho("Int")),
                TypeChirho::ConChirho(mk_name_chirho("Bool")),
            ],
        );
        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            result_chirho.diagnostics_chirho.has_errors_chirho(),
            "a one-parameter class applied to two arguments must be rejected"
        );
    }

    #[test]
    fn instance_head_matching_arity_is_accepted_chirho() {
        let module_chirho = mk_class_and_instance_module_chirho(
            &["a", "b"],
            vec![
                TypeChirho::ConChirho(mk_name_chirho("Int")),
                TypeChirho::ConChirho(mk_name_chirho("Bool")),
            ],
        );
        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "a correctly-saturated instance head must be accepted: {:?}",
            result_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn instance_head_of_undeclared_class_is_not_judged_chirho() {
        // The class is not declared here, so its arity is unknown to us and no
        // verdict may be reached — an imported class could have any arity.
        let module_chirho = mk_module_chirho(vec![DeclChirho::InstanceDeclChirho {
            context_chirho: vec![],
            class_chirho: mk_name_chirho("SomeImportedClassChirho"),
            types_chirho: vec![TypeChirho::ConChirho(mk_name_chirho("Int"))],
            methods_chirho: vec![],
            assoc_tf_instances_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);
        let result_chirho = infer_module_kinds_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "an instance of an undeclared class must not be judged: {:?}",
            result_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn kind_arity_is_final_only_when_tail_is_not_a_variable_chirho() {
        // `k -> *` has arity 1 even though the ARGUMENT kind is a variable —
        // only a variable TAIL leaves the arity open.
        let open_tail_chirho = KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::VarChirho(KindVarChirho(0)),
        );
        assert_eq!(
            KindInferCtxChirho::kind_arity_chirho(&open_tail_chirho),
            (1, false)
        );
        let var_arg_chirho = KindChirho::arrow_chirho(
            KindChirho::VarChirho(KindVarChirho(0)),
            KindChirho::ConstraintChirho,
        );
        assert_eq!(
            KindInferCtxChirho::kind_arity_chirho(&var_arg_chirho),
            (1, true)
        );
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
