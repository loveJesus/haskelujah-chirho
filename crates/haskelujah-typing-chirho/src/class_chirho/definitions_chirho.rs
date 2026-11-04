// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Typeclass and instance carriers, moved without semantic changes.

use std::collections::HashMap;
use std::fmt;

use haskelujah_ast_chirho::expr_chirho::MatchArmChirho;

use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

/// A class predicate: `ClassName Type`, e.g. `Eq Int`, `Show a`.
/// For multi-parameter type classes: `Convert Int String`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PredChirho {
    /// Hidden kind arguments; separate from ordinary class positions.
    pub kind_tys_chirho: Vec<TyChirho>,
    pub class_name_chirho: String,
    /// The primary type argument.
    pub ty_chirho: TyChirho,
    /// Extra type arguments for multi-parameter type classes.
    /// Empty for single-parameter classes.
    pub extra_tys_chirho: Vec<TyChirho>,
}

impl PredChirho {
    pub fn new_chirho(class_name_chirho: &str, ty_chirho: TyChirho) -> Self {
        Self {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: class_name_chirho.to_string(),
            ty_chirho,
            extra_tys_chirho: vec![],
        }
    }

    /// Create a multi-parameter predicate.
    pub fn new_multi_chirho(class_name_chirho: &str, tys_chirho: Vec<TyChirho>) -> Self {
        let ty_chirho = tys_chirho
            .first()
            .cloned()
            .unwrap_or(TyChirho::int_chirho());
        Self {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: class_name_chirho.to_string(),
            ty_chirho,
            extra_tys_chirho: tys_chirho.into_iter().skip(1).collect(),
        }
    }

    /// Get all type arguments.
    pub fn all_tys_chirho(&self) -> Vec<TyChirho> {
        let mut tys_chirho = vec![self.ty_chirho.clone()];
        tys_chirho.extend(self.extra_tys_chirho.clone());
        tys_chirho
    }
}

impl fmt::Display for PredChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{} {}", self.class_name_chirho, self.ty_chirho)?;
        for extra_chirho in &self.extra_tys_chirho {
            write!(f_chirho, " {}", extra_chirho)?;
        }
        Ok(())
    }
}

/// A qualified type: a list of predicates (constraints) and a body type.
/// Example: `(Eq a, Show a) => a -> String`
#[derive(Debug, Clone, PartialEq)]
pub struct QualTyChirho {
    pub preds_chirho: Vec<PredChirho>,
    pub ty_chirho: TyChirho,
}

impl QualTyChirho {
    pub fn unqualified_chirho(ty_chirho: TyChirho) -> Self {
        Self {
            preds_chirho: vec![],
            ty_chirho,
        }
    }

    pub fn qualified_chirho(preds_chirho: Vec<PredChirho>, ty_chirho: TyChirho) -> Self {
        Self {
            preds_chirho,
            ty_chirho,
        }
    }
}

impl fmt::Display for QualTyChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.preds_chirho.is_empty() {
            write!(f_chirho, "{}", self.ty_chirho)
        } else {
            write!(f_chirho, "(")?;
            for (i_chirho, pred_chirho) in self.preds_chirho.iter().enumerate() {
                if i_chirho > 0 {
                    write!(f_chirho, ", ")?;
                }
                write!(f_chirho, "{pred_chirho}")?;
            }
            write!(f_chirho, ") => {}", self.ty_chirho)
        }
    }
}

/// A typeclass declaration.
///
/// Example: `class Eq a where (==) :: a -> a -> Bool`
///
/// Supports multi-parameter type classes (MPTCs) via `vars_chirho`.
/// For single-parameter classes, `var_chirho` returns the single parameter.
#[derive(Debug, Clone)]
pub struct ClassDeclChirho {
    /// Quantified kind parameters; never dictionary method positions.
    pub kind_vars_chirho: Vec<TyVarChirho>,
    pub name_chirho: String,
    /// Superclass constraints, e.g. `Eq a` for `class Eq a => Ord a`.
    pub supers_chirho: Vec<String>,
    /// The class type variable (e.g. `a` in `class Eq a`).
    /// For single-parameter classes; use `vars_chirho` for MPTCs.
    pub var_chirho: TyVarChirho,
    /// Method signatures: name → type scheme.
    pub methods_chirho: HashMap<String, SchemeChirho>,
    /// All class type variables. For single-param classes, contains just `[var_chirho]`.
    /// For MPTCs like `class Convert a b`, contains `[a, b]`.
    /// Empty means single-param (inferred from `var_chirho`).
    pub extra_vars_chirho: Vec<TyVarChirho>,
    /// Functional dependencies: each entry `(from, to)` means the types
    /// at `from` indices determine the types at `to` indices.
    /// E.g., `class C a b | a -> b` has `fundeps = [([0], [1])]`.
    pub fundeps_chirho: Vec<(Vec<usize>, Vec<usize>)>,
    /// Default method implementations: name → match arms.
    /// Used when an instance omits a method that has a default in the class.
    pub defaults_chirho: HashMap<String, Vec<MatchArmChirho>>,
}

impl ClassDeclChirho {
    /// Get all type variables for this class.
    /// For single-param classes, returns `[var_chirho]`.
    /// For MPTCs, returns `[var_chirho] ++ extra_vars_chirho`.
    pub fn all_vars_chirho(&self) -> Vec<TyVarChirho> {
        let mut vars_chirho = vec![self.var_chirho];
        vars_chirho.extend_from_slice(&self.extra_vars_chirho);
        vars_chirho
    }

    /// Whether this is a multi-parameter type class.
    pub fn is_mptc_chirho(&self) -> bool {
        !self.extra_vars_chirho.is_empty()
    }
}

/// A typeclass instance declaration.
///
/// Example: `instance Eq Int where (==) = primEqInt`
#[derive(Debug, Clone)]
pub struct InstDeclChirho {
    /// Hidden kind prefix, matched before ordinary class arguments.
    pub kind_tys_chirho: Vec<TyChirho>,
    pub class_name_chirho: String,
    /// The instance head type, e.g. `Int` in `instance Eq Int`.
    pub head_ty_chirho: TyChirho,
    /// Extra head types for multi-parameter type classes.
    /// E.g., `Double` in `instance Convert Int Double where ...`
    /// Empty for single-parameter classes.
    pub extra_head_tys_chirho: Vec<TyChirho>,
    /// Required context, e.g. `Eq a` in `instance Eq a => Eq [a]`.
    pub context_chirho: Vec<PredChirho>,
}

/// The class environment: tracks all known classes and their instances.
#[derive(Debug, Clone, Default)]
pub struct ClassEnvChirho {
    /// Class name → declaration.
    pub classes_chirho: HashMap<String, ClassDeclChirho>,
    /// Class name → list of instances.
    pub instances_chirho: HashMap<String, Vec<InstDeclChirho>>,
}
