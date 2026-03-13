// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Internal type representation for type inference
//!
//! `TyChirho` is the inference engine's view of types. It supports unification
//! variables (`TyVarChirho`) that get filled in during inference, unlike the
//! surface-syntax `TypeChirho` from the AST.

use std::fmt;

/// A unique identifier for a type variable introduced during inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TyVarChirho(pub u32);

impl fmt::Display for TyVarChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display as t0, t1, t2, ...
        write!(f_chirho, "t{}", self.0)
    }
}

/// The internal type representation used by the type inference engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TyChirho {
    /// A unification variable (filled in during inference).
    VarChirho(TyVarChirho),

    /// A named type constructor (`Int`, `Bool`, `Maybe`, `IO`, `(,)`, `[]`).
    ConChirho(String),

    /// Type application (`Maybe Int` = App(Con("Maybe"), Con("Int"))`).
    AppChirho(Box<TyChirho>, Box<TyChirho>),

    /// Function type (`a -> b`). Syntactically sugar for `App(App(Con("->"), a), b)`,
    /// but kept explicit for readability and fast matching.
    FunChirho(Box<TyChirho>, Box<TyChirho>),

    /// Tuple type (`(a, b)` = App(App(Con("(,)"), a), b)` etc.).
    /// Represented explicitly for convenience; arity is `elements.len()`.
    TupleChirho(Vec<TyChirho>),

    /// List type (`[a]`). Sugar for `App(Con("[]"), a)`.
    ListChirho(Box<TyChirho>),

    /// A universally quantified variable bound by a `forall`. Distinguished
    /// from `VarChirho` (which is a unification variable).
    ForallVarChirho(String),
}

impl TyChirho {
    /// Convenience: the `Int` type.
    pub fn int_chirho() -> Self {
        Self::ConChirho("Int".to_string())
    }

    /// Convenience: the `Bool` type.
    pub fn bool_chirho() -> Self {
        Self::ConChirho("Bool".to_string())
    }

    /// Convenience: the `Char` type.
    pub fn char_chirho() -> Self {
        Self::ConChirho("Char".to_string())
    }

    /// Convenience: the `Float` / `Double` type.
    pub fn double_chirho() -> Self {
        Self::ConChirho("Double".to_string())
    }

    /// Convenience: `String` (= `[Char]`).
    pub fn string_chirho() -> Self {
        Self::ListChirho(Box::new(Self::char_chirho()))
    }

    /// Convenience: the unit type `()`.
    pub fn unit_chirho() -> Self {
        Self::TupleChirho(vec![])
    }

    /// Build a function type `a -> b`.
    pub fn fun_chirho(arg_chirho: TyChirho, result_chirho: TyChirho) -> Self {
        Self::FunChirho(Box::new(arg_chirho), Box::new(result_chirho))
    }

    /// Build a multi-argument function type `a -> b -> c -> ... -> result`.
    pub fn fun_n_chirho(args_chirho: impl IntoIterator<Item = TyChirho>, result_chirho: TyChirho) -> Self {
        let mut ty_chirho = result_chirho;
        let args_vec_chirho: Vec<_> = args_chirho.into_iter().collect();
        for arg_chirho in args_vec_chirho.into_iter().rev() {
            ty_chirho = Self::fun_chirho(arg_chirho, ty_chirho);
        }
        ty_chirho
    }

    /// Collect all free type variables (unification variables) in this type.
    pub fn free_vars_chirho(&self) -> Vec<TyVarChirho> {
        let mut vars_chirho = Vec::new();
        self.collect_free_vars_chirho(&mut vars_chirho);
        vars_chirho.sort();
        vars_chirho.dedup();
        vars_chirho
    }

    fn collect_free_vars_chirho(&self, out_chirho: &mut Vec<TyVarChirho>) {
        match self {
            TyChirho::VarChirho(v_chirho) => out_chirho.push(*v_chirho),
            TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {}
            TyChirho::AppChirho(f_chirho, a_chirho) => {
                f_chirho.collect_free_vars_chirho(out_chirho);
                a_chirho.collect_free_vars_chirho(out_chirho);
            }
            TyChirho::FunChirho(a_chirho, b_chirho) => {
                a_chirho.collect_free_vars_chirho(out_chirho);
                b_chirho.collect_free_vars_chirho(out_chirho);
            }
            TyChirho::TupleChirho(elems_chirho) => {
                for e_chirho in elems_chirho {
                    e_chirho.collect_free_vars_chirho(out_chirho);
                }
            }
            TyChirho::ListChirho(inner_chirho) => {
                inner_chirho.collect_free_vars_chirho(out_chirho);
            }
        }
    }
}

impl fmt::Display for TyChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyChirho::VarChirho(v_chirho) => write!(f_chirho, "{v_chirho}"),
            TyChirho::ConChirho(name_chirho) => write!(f_chirho, "{name_chirho}"),
            TyChirho::ForallVarChirho(name_chirho) => write!(f_chirho, "{name_chirho}"),
            TyChirho::AppChirho(fun_chirho, arg_chirho) => {
                write!(f_chirho, "({fun_chirho} {arg_chirho})")
            }
            TyChirho::FunChirho(a_chirho, b_chirho) => {
                write!(f_chirho, "({a_chirho} -> {b_chirho})")
            }
            TyChirho::TupleChirho(elems_chirho) => {
                write!(f_chirho, "(")?;
                for (i_chirho, e_chirho) in elems_chirho.iter().enumerate() {
                    if i_chirho > 0 {
                        write!(f_chirho, ", ")?;
                    }
                    write!(f_chirho, "{e_chirho}")?;
                }
                write!(f_chirho, ")")
            }
            TyChirho::ListChirho(inner_chirho) => write!(f_chirho, "[{inner_chirho}]"),
        }
    }
}

/// A predicate reference used in type schemes. Kept lightweight:
/// just a class name and the type it constrains. Full `PredChirho`
/// lives in `class_chirho`; this avoids a circular dependency.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemePredChirho {
    pub class_name_chirho: String,
    pub ty_chirho: TyChirho,
}

/// A type scheme: `forall a1 a2 ... . (preds =>) ty`. Represents polymorphism.
/// When `vars` is empty, the scheme is monomorphic.
/// When `preds` is non-empty, the scheme is constrained (qualified).
#[derive(Debug, Clone, PartialEq)]
pub struct SchemeChirho {
    pub vars_chirho: Vec<TyVarChirho>,
    /// Typeclass predicates on the quantified variables, e.g. `Num a`.
    pub preds_chirho: Vec<SchemePredChirho>,
    pub ty_chirho: TyChirho,
}

impl SchemeChirho {
    /// A monomorphic scheme (no quantified variables, no predicates).
    pub fn mono_chirho(ty_chirho: TyChirho) -> Self {
        Self {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho,
        }
    }

    /// Collect free type variables (those NOT bound by the forall).
    pub fn free_vars_chirho(&self) -> Vec<TyVarChirho> {
        self.ty_chirho
            .free_vars_chirho()
            .into_iter()
            .filter(|v_chirho| !self.vars_chirho.contains(v_chirho))
            .collect()
    }
}

impl fmt::Display for SchemePredChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{} {}", self.class_name_chirho, self.ty_chirho)
    }
}

impl fmt::Display for SchemeChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.vars_chirho.is_empty() && self.preds_chirho.is_empty() {
            write!(f_chirho, "{}", self.ty_chirho)
        } else {
            if !self.vars_chirho.is_empty() {
                write!(f_chirho, "forall")?;
                for v_chirho in &self.vars_chirho {
                    write!(f_chirho, " {v_chirho}")?;
                }
                write!(f_chirho, ". ")?;
            }
            if !self.preds_chirho.is_empty() {
                write!(f_chirho, "(")?;
                for (i_chirho, p_chirho) in self.preds_chirho.iter().enumerate() {
                    if i_chirho > 0 {
                        write!(f_chirho, ", ")?;
                    }
                    write!(f_chirho, "{p_chirho}")?;
                }
                write!(f_chirho, ") => ")?;
            }
            write!(f_chirho, "{}", self.ty_chirho)
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn ty_display_chirho() {
        let ty_chirho = TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::bool_chirho());
        assert_eq!(ty_chirho.to_string(), "(Int -> Bool)");
    }

    #[test]
    fn ty_free_vars_chirho() {
        let a_chirho = TyVarChirho(0);
        let b_chirho = TyVarChirho(1);
        let ty_chirho = TyChirho::fun_chirho(
            TyChirho::VarChirho(a_chirho),
            TyChirho::VarChirho(b_chirho),
        );
        let fvs_chirho = ty_chirho.free_vars_chirho();
        assert_eq!(fvs_chirho, vec![a_chirho, b_chirho]);
    }

    #[test]
    fn scheme_free_vars_exclude_bound_chirho() {
        let a_chirho = TyVarChirho(0);
        let b_chirho = TyVarChirho(1);
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(a_chirho),
                TyChirho::VarChirho(b_chirho),
            ),
        };
        assert_eq!(scheme_chirho.free_vars_chirho(), vec![b_chirho]);
    }

    #[test]
    fn fun_n_builds_curried_type_chirho() {
        let ty_chirho = TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::bool_chirho()],
            TyChirho::char_chirho(),
        );
        assert_eq!(ty_chirho.to_string(), "(Int -> (Bool -> Char))");
    }

    #[test]
    fn tuple_display_chirho() {
        let ty_chirho = TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::bool_chirho()]);
        assert_eq!(ty_chirho.to_string(), "(Int, Bool)");
    }

    #[test]
    fn list_display_chirho() {
        let ty_chirho = TyChirho::ListChirho(Box::new(TyChirho::int_chirho()));
        assert_eq!(ty_chirho.to_string(), "[Int]");
    }

    #[test]
    fn scheme_display_chirho() {
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![TyVarChirho(0), TyVarChirho(1)],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(0)),
                TyChirho::VarChirho(TyVarChirho(1)),
            ),
        };
        assert_eq!(scheme_chirho.to_string(), "forall t0 t1. (t0 -> t1)");
    }
}
