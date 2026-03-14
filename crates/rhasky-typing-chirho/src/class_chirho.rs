// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Typeclass infrastructure
//!
//! Provides the data structures for Haskell-style typeclasses:
//!
//! - `PredChirho` — a class constraint like `Eq a` or `Ord a`
//! - `QualTyChirho` — a qualified type `(Eq a, Show a) => a -> String`
//! - `ClassDeclChirho` — a typeclass declaration with superclasses and methods
//! - `InstDeclChirho` — a typeclass instance with method implementations
//! - `ClassEnvChirho` — the environment of known classes and instances,
//!   supporting instance resolution / context reduction

use std::collections::HashMap;
use rhasky_ast_chirho::expr_chirho::MatchArmChirho;
use std::fmt;

use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

/// A class predicate: `ClassName Type`, e.g. `Eq Int`, `Show a`.
/// For multi-parameter type classes: `Convert Int String`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PredChirho {
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
            class_name_chirho: class_name_chirho.to_string(),
            ty_chirho,
            extra_tys_chirho: vec![],
        }
    }

    /// Create a multi-parameter predicate.
    pub fn new_multi_chirho(
        class_name_chirho: &str,
        tys_chirho: Vec<TyChirho>,
    ) -> Self {
        let ty_chirho = tys_chirho.first().cloned().unwrap_or(TyChirho::int_chirho());
        Self {
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

impl ClassEnvChirho {
    pub fn new_chirho() -> Self {
        Self::default()
    }

    /// Register a class declaration.
    pub fn add_class_chirho(&mut self, decl_chirho: ClassDeclChirho) {
        let name_chirho = decl_chirho.name_chirho.clone();
        self.classes_chirho.insert(name_chirho.clone(), decl_chirho);
        self.instances_chirho
            .entry(name_chirho)
            .or_insert_with(Vec::new);
    }

    /// Register an instance declaration.
    pub fn add_instance_chirho(&mut self, inst_chirho: InstDeclChirho) {
        self.instances_chirho
            .entry(inst_chirho.class_name_chirho.clone())
            .or_insert_with(Vec::new)
            .push(inst_chirho);
    }

    /// Check if a class exists.
    pub fn has_class_chirho(&self, name_chirho: &str) -> bool {
        self.classes_chirho.contains_key(name_chirho)
    }

    /// Get the superclasses of a class.
    pub fn superclasses_chirho(&self, class_name_chirho: &str) -> Vec<String> {
        self.classes_chirho
            .get(class_name_chirho)
            .map(|c_chirho| c_chirho.supers_chirho.clone())
            .unwrap_or_default()
    }

    /// Attempt to resolve a predicate: find an instance that matches and
    /// return the required context (sub-goals).
    ///
    /// For example, resolving `Eq [Int]` against `instance Eq a => Eq [a]`
    /// would match with `a = Int` and return `[Eq Int]` as a sub-goal.
    ///
    /// For MPTCs, all type arguments must match simultaneously. E.g.,
    /// resolving `Convert Int Double` against `instance Convert Int Double`
    /// matches both the primary and extra type parameters.
    pub fn resolve_chirho(&self, pred_chirho: &PredChirho) -> Option<Vec<PredChirho>> {
        let instances_chirho = self.instances_chirho.get(&pred_chirho.class_name_chirho)?;

        'inst: for inst_chirho in instances_chirho {
            // Match the primary head type
            let mut subst_chirho = match match_ty_chirho(&inst_chirho.head_ty_chirho, &pred_chirho.ty_chirho) {
                Some(s_chirho) => s_chirho,
                None => continue,
            };

            // For MPTCs, also match extra type params
            if inst_chirho.extra_head_tys_chirho.len() != pred_chirho.extra_tys_chirho.len() {
                continue;
            }
            for (inst_extra_chirho, pred_extra_chirho) in inst_chirho
                .extra_head_tys_chirho
                .iter()
                .zip(pred_chirho.extra_tys_chirho.iter())
            {
                match match_ty_chirho(inst_extra_chirho, pred_extra_chirho) {
                    Some(extra_subst_chirho) => {
                        // Merge substitutions — check for consistency
                        if !subst_chirho.merge_chirho(&extra_subst_chirho) {
                            continue 'inst;
                        }
                    }
                    None => continue 'inst,
                }
            }

            // Apply the merged substitution to the instance context
            let sub_goals_chirho: Vec<PredChirho> = inst_chirho
                .context_chirho
                .iter()
                .map(|ctx_pred_chirho| PredChirho {
                    class_name_chirho: ctx_pred_chirho.class_name_chirho.clone(),
                    ty_chirho: subst_chirho.apply_ty_chirho(&ctx_pred_chirho.ty_chirho),
                    extra_tys_chirho: ctx_pred_chirho
                        .extra_tys_chirho
                        .iter()
                        .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
                        .collect(),
                })
                .collect();
            return Some(sub_goals_chirho);
        }

        None
    }

    /// Apply functional dependency improvement to a predicate.
    ///
    /// Given a predicate like `Convert Int b` and a class `Convert a b | a -> b`
    /// with an instance `instance Convert Int Double`, the functional dependency
    /// `a -> b` lets us determine that `b = Double`. Returns a substitution
    /// that improves the unknown type variables.
    pub fn fundep_improve_chirho(&self, pred_chirho: &PredChirho) -> SubstChirho {
        let class_chirho = match self.classes_chirho.get(&pred_chirho.class_name_chirho) {
            Some(c_chirho) => c_chirho,
            None => return SubstChirho::empty_chirho(),
        };

        if class_chirho.fundeps_chirho.is_empty() {
            return SubstChirho::empty_chirho();
        }

        let instances_chirho = match self.instances_chirho.get(&pred_chirho.class_name_chirho) {
            Some(i_chirho) => i_chirho,
            None => return SubstChirho::empty_chirho(),
        };

        let pred_tys_chirho = pred_chirho.all_tys_chirho();
        let mut improvement_chirho = SubstChirho::empty_chirho();

        for inst_chirho in instances_chirho {
            let mut inst_tys_chirho = vec![inst_chirho.head_ty_chirho.clone()];
            inst_tys_chirho.extend(inst_chirho.extra_head_tys_chirho.iter().cloned());

            for (from_chirho, to_chirho) in &class_chirho.fundeps_chirho {
                // Check if "from" positions in the predicate match the instance
                let mut from_subst_chirho = SubstChirho::empty_chirho();
                let mut from_matches_chirho = true;

                for &idx_chirho in from_chirho {
                    if idx_chirho >= pred_tys_chirho.len() || idx_chirho >= inst_tys_chirho.len() {
                        from_matches_chirho = false;
                        break;
                    }
                    match match_ty_chirho(&inst_tys_chirho[idx_chirho], &pred_tys_chirho[idx_chirho])
                    {
                        Some(s_chirho) => {
                            if !from_subst_chirho.merge_chirho(&s_chirho) {
                                from_matches_chirho = false;
                                break;
                            }
                        }
                        None => {
                            from_matches_chirho = false;
                            break;
                        }
                    }
                }

                if !from_matches_chirho {
                    continue;
                }

                // "from" positions match — the "to" positions are determined
                for &idx_chirho in to_chirho {
                    if idx_chirho >= pred_tys_chirho.len() || idx_chirho >= inst_tys_chirho.len() {
                        continue;
                    }
                    let determined_ty_chirho =
                        from_subst_chirho.apply_ty_chirho(&inst_tys_chirho[idx_chirho]);
                    let pred_ty_chirho = &pred_tys_chirho[idx_chirho];

                    // If the pred position is a type variable, we can improve it
                    if let TyChirho::VarChirho(v_chirho) = pred_ty_chirho {
                        improvement_chirho.insert_chirho(*v_chirho, determined_ty_chirho);
                    }
                }
            }
        }

        improvement_chirho
    }

    /// Check if a predicate is entailed by the class environment
    /// (i.e., there exists an instance that satisfies it with no remaining goals).
    pub fn entails_chirho(&self, pred_chirho: &PredChirho) -> bool {
        if let Some(sub_goals_chirho) = self.resolve_chirho(pred_chirho) {
            sub_goals_chirho
                .iter()
                .all(|sg_chirho| self.entails_chirho(sg_chirho))
        } else {
            false
        }
    }

    /// Seed the environment with standard Haskell typeclasses.
    pub fn seed_standard_chirho(&mut self) {
        // Eq
        let eq_var_chirho = TyVarChirho(9000);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Eq".to_string(),
            supers_chirho: vec![],
            var_chirho: eq_var_chirho,
            methods_chirho: HashMap::from([(
                "==".to_string(),
                SchemeChirho {
                    vars_chirho: vec![eq_var_chirho],
                    preds_chirho: vec![],
                    ty_chirho: TyChirho::fun_n_chirho(
                        [
                            TyChirho::VarChirho(eq_var_chirho),
                            TyChirho::VarChirho(eq_var_chirho),
                        ],
                        TyChirho::bool_chirho(),
                    ),
                },
            )]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // Ord (superclass: Eq)
        let ord_var_chirho = TyVarChirho(9001);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Ord".to_string(),
            supers_chirho: vec!["Eq".to_string()],
            var_chirho: ord_var_chirho,
            methods_chirho: HashMap::from([(
                "compare".to_string(),
                SchemeChirho {
                    vars_chirho: vec![ord_var_chirho],
                    preds_chirho: vec![],
                    ty_chirho: TyChirho::fun_n_chirho(
                        [
                            TyChirho::VarChirho(ord_var_chirho),
                            TyChirho::VarChirho(ord_var_chirho),
                        ],
                        TyChirho::ConChirho("Ordering".to_string()),
                    ),
                },
            )]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // Show
        let show_var_chirho = TyVarChirho(9002);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Show".to_string(),
            supers_chirho: vec![],
            var_chirho: show_var_chirho,
            methods_chirho: HashMap::from([(
                "show".to_string(),
                SchemeChirho {
                    vars_chirho: vec![show_var_chirho],
                    preds_chirho: vec![],
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::VarChirho(show_var_chirho),
                        TyChirho::string_chirho(),
                    ),
                },
            )]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // Num
        let num_var_chirho = TyVarChirho(9003);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Num".to_string(),
            supers_chirho: vec!["Eq".to_string(), "Show".to_string()],
            var_chirho: num_var_chirho,
            methods_chirho: HashMap::from([
                (
                    "+".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![num_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_n_chirho(
                            [
                                TyChirho::VarChirho(num_var_chirho),
                                TyChirho::VarChirho(num_var_chirho),
                            ],
                            TyChirho::VarChirho(num_var_chirho),
                        ),
                    },
                ),
                (
                    "*".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![num_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_n_chirho(
                            [
                                TyChirho::VarChirho(num_var_chirho),
                                TyChirho::VarChirho(num_var_chirho),
                            ],
                            TyChirho::VarChirho(num_var_chirho),
                        ),
                    },
                ),
                (
                    "-".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![num_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_n_chirho(
                            [
                                TyChirho::VarChirho(num_var_chirho),
                                TyChirho::VarChirho(num_var_chirho),
                            ],
                            TyChirho::VarChirho(num_var_chirho),
                        ),
                    },
                ),
                (
                    "negate".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![num_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(num_var_chirho),
                            TyChirho::VarChirho(num_var_chirho),
                        ),
                    },
                ),
                (
                    "fromInteger".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![num_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::int_chirho(),
                            TyChirho::VarChirho(num_var_chirho),
                        ),
                    },
                ),
                (
                    "abs".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![num_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(num_var_chirho),
                            TyChirho::VarChirho(num_var_chirho),
                        ),
                    },
                ),
                (
                    "signum".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![num_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(num_var_chirho),
                            TyChirho::VarChirho(num_var_chirho),
                        ),
                    },
                ),
            ]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // Fractional (superclass: Num)
        let frac_var_chirho = TyVarChirho(9010);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Fractional".to_string(),
            supers_chirho: vec!["Num".to_string()],
            var_chirho: frac_var_chirho,
            methods_chirho: HashMap::from([
                (
                    "/".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![frac_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_n_chirho(
                            [
                                TyChirho::VarChirho(frac_var_chirho),
                                TyChirho::VarChirho(frac_var_chirho),
                            ],
                            TyChirho::VarChirho(frac_var_chirho),
                        ),
                    },
                ),
                (
                    "recip".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![frac_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(frac_var_chirho),
                            TyChirho::VarChirho(frac_var_chirho),
                        ),
                    },
                ),
                (
                    "fromRational".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![frac_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::ConChirho("Rational".to_string()),
                            TyChirho::VarChirho(frac_var_chirho),
                        ),
                    },
                ),
            ]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // Functor (f :: * -> *)
        // fmap :: (a -> b) -> f a -> f b
        // (<$) :: a -> f b -> f a   (has default: fmap . const)
        let f_var_chirho = TyVarChirho(9004);
        let functor_a_chirho = TyVarChirho(9040);
        let functor_b_chirho = TyVarChirho(9041);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Functor".to_string(),
            supers_chirho: vec![],
            var_chirho: f_var_chirho,
            methods_chirho: HashMap::from([
                // fmap :: (a -> b) -> f a -> f b
                (
                    "fmap".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![f_var_chirho, functor_a_chirho, functor_b_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::FunChirho(
                            Box::new(TyChirho::FunChirho(
                                Box::new(TyChirho::VarChirho(functor_a_chirho)),
                                Box::new(TyChirho::VarChirho(functor_b_chirho)),
                            )),
                            Box::new(TyChirho::FunChirho(
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(f_var_chirho)),
                                    Box::new(TyChirho::VarChirho(functor_a_chirho)),
                                )),
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(f_var_chirho)),
                                    Box::new(TyChirho::VarChirho(functor_b_chirho)),
                                )),
                            )),
                        ),
                    },
                ),
            ]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // Applicative (superclass: Functor)
        // pure :: a -> f a
        // (<*>) :: f (a -> b) -> f a -> f b
        let app_var_chirho = TyVarChirho(9042);
        let app_a_chirho = TyVarChirho(9043);
        let app_b_chirho = TyVarChirho(9044);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Applicative".to_string(),
            supers_chirho: vec!["Functor".to_string()],
            var_chirho: app_var_chirho,
            methods_chirho: HashMap::from([
                // pure :: a -> f a
                (
                    "pure".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![app_var_chirho, app_a_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::FunChirho(
                            Box::new(TyChirho::VarChirho(app_a_chirho)),
                            Box::new(TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(app_var_chirho)),
                                Box::new(TyChirho::VarChirho(app_a_chirho)),
                            )),
                        ),
                    },
                ),
                // (<*>) :: f (a -> b) -> f a -> f b
                (
                    "<*>".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![app_var_chirho, app_a_chirho, app_b_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::FunChirho(
                            Box::new(TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(app_var_chirho)),
                                Box::new(TyChirho::FunChirho(
                                    Box::new(TyChirho::VarChirho(app_a_chirho)),
                                    Box::new(TyChirho::VarChirho(app_b_chirho)),
                                )),
                            )),
                            Box::new(TyChirho::FunChirho(
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(app_var_chirho)),
                                    Box::new(TyChirho::VarChirho(app_a_chirho)),
                                )),
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(app_var_chirho)),
                                    Box::new(TyChirho::VarChirho(app_b_chirho)),
                                )),
                            )),
                        ),
                    },
                ),
            ]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // Monad (superclass: Applicative)
        // (>>=) :: m a -> (a -> m b) -> m b
        // return :: a -> m a  (default: pure)
        let m_var_chirho = TyVarChirho(9045);
        let monad_a_chirho = TyVarChirho(9046);
        let monad_b_chirho = TyVarChirho(9047);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Monad".to_string(),
            supers_chirho: vec!["Applicative".to_string()],
            var_chirho: m_var_chirho,
            methods_chirho: HashMap::from([
                // (>>=) :: m a -> (a -> m b) -> m b
                (
                    ">>=".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![m_var_chirho, monad_a_chirho, monad_b_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::FunChirho(
                            Box::new(TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(m_var_chirho)),
                                Box::new(TyChirho::VarChirho(monad_a_chirho)),
                            )),
                            Box::new(TyChirho::FunChirho(
                                Box::new(TyChirho::FunChirho(
                                    Box::new(TyChirho::VarChirho(monad_a_chirho)),
                                    Box::new(TyChirho::AppChirho(
                                        Box::new(TyChirho::VarChirho(m_var_chirho)),
                                        Box::new(TyChirho::VarChirho(monad_b_chirho)),
                                    )),
                                )),
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(m_var_chirho)),
                                    Box::new(TyChirho::VarChirho(monad_b_chirho)),
                                )),
                            )),
                        ),
                    },
                ),
                // (>>) :: m a -> m b -> m b
                (
                    ">>".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![m_var_chirho, monad_a_chirho, monad_b_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::FunChirho(
                            Box::new(TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(m_var_chirho)),
                                Box::new(TyChirho::VarChirho(monad_a_chirho)),
                            )),
                            Box::new(TyChirho::FunChirho(
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(m_var_chirho)),
                                    Box::new(TyChirho::VarChirho(monad_b_chirho)),
                                )),
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(m_var_chirho)),
                                    Box::new(TyChirho::VarChirho(monad_b_chirho)),
                                )),
                            )),
                        ),
                    },
                ),
            ]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // Read (no superclasses in Haskell 2010)
        // Simplified: use `read` as the single method (String -> a)
        // instead of the full `readsPrec` / `readList` interface.
        let read_var_chirho = TyVarChirho(9011);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Read".to_string(),
            supers_chirho: vec![],
            var_chirho: read_var_chirho,
            methods_chirho: HashMap::from([(
                "read".to_string(),
                SchemeChirho {
                    vars_chirho: vec![read_var_chirho],
                    preds_chirho: vec![],
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::string_chirho(),
                        TyChirho::VarChirho(read_var_chirho),
                    ),
                },
            )]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // Enum
        let enum_var_chirho = TyVarChirho(9006);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Enum".to_string(),
            supers_chirho: vec![],
            var_chirho: enum_var_chirho,
            methods_chirho: HashMap::from([
                (
                    "toEnum".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![enum_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::int_chirho(),
                            TyChirho::VarChirho(enum_var_chirho),
                        ),
                    },
                ),
                (
                    "fromEnum".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![enum_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(enum_var_chirho),
                            TyChirho::int_chirho(),
                        ),
                    },
                ),
            ]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // Bounded
        let bounded_var_chirho = TyVarChirho(9007);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Bounded".to_string(),
            supers_chirho: vec![],
            var_chirho: bounded_var_chirho,
            methods_chirho: HashMap::from([
                (
                    "minBound".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![bounded_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::VarChirho(bounded_var_chirho),
                    },
                ),
                (
                    "maxBound".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![bounded_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::VarChirho(bounded_var_chirho),
                    },
                ),
            ]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // Floating (superclass: Fractional)
        let floating_var_chirho = TyVarChirho(9009);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Floating".to_string(),
            supers_chirho: vec!["Fractional".to_string()],
            var_chirho: floating_var_chirho,
            methods_chirho: HashMap::from([
                (
                    "pi".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![floating_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::VarChirho(floating_var_chirho),
                    },
                ),
                (
                    "exp".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![floating_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(floating_var_chirho),
                            TyChirho::VarChirho(floating_var_chirho),
                        ),
                    },
                ),
                (
                    "log".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![floating_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(floating_var_chirho),
                            TyChirho::VarChirho(floating_var_chirho),
                        ),
                    },
                ),
                (
                    "sqrt".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![floating_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(floating_var_chirho),
                            TyChirho::VarChirho(floating_var_chirho),
                        ),
                    },
                ),
                (
                    "sin".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![floating_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(floating_var_chirho),
                            TyChirho::VarChirho(floating_var_chirho),
                        ),
                    },
                ),
                (
                    "cos".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![floating_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(floating_var_chirho),
                            TyChirho::VarChirho(floating_var_chirho),
                        ),
                    },
                ),
                (
                    "tan".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![floating_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(floating_var_chirho),
                            TyChirho::VarChirho(floating_var_chirho),
                        ),
                    },
                ),
                (
                    "asin".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![floating_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(floating_var_chirho),
                            TyChirho::VarChirho(floating_var_chirho),
                        ),
                    },
                ),
                (
                    "acos".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![floating_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(floating_var_chirho),
                            TyChirho::VarChirho(floating_var_chirho),
                        ),
                    },
                ),
                (
                    "atan".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![floating_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(floating_var_chirho),
                            TyChirho::VarChirho(floating_var_chirho),
                        ),
                    },
                ),
            ]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // instance Floating Double
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Floating".to_string(),
            head_ty_chirho: TyChirho::ConChirho("Double".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // Integral (superclass: Num)
        let integral_var_chirho = TyVarChirho(9008);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Integral".to_string(),
            supers_chirho: vec!["Num".to_string()],
            var_chirho: integral_var_chirho,
            methods_chirho: HashMap::from([
                (
                    "div".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![integral_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_n_chirho(
                            [
                                TyChirho::VarChirho(integral_var_chirho),
                                TyChirho::VarChirho(integral_var_chirho),
                            ],
                            TyChirho::VarChirho(integral_var_chirho),
                        ),
                    },
                ),
                (
                    "mod".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![integral_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_n_chirho(
                            [
                                TyChirho::VarChirho(integral_var_chirho),
                                TyChirho::VarChirho(integral_var_chirho),
                            ],
                            TyChirho::VarChirho(integral_var_chirho),
                        ),
                    },
                ),
                (
                    "quot".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![integral_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_n_chirho(
                            [
                                TyChirho::VarChirho(integral_var_chirho),
                                TyChirho::VarChirho(integral_var_chirho),
                            ],
                            TyChirho::VarChirho(integral_var_chirho),
                        ),
                    },
                ),
                (
                    "rem".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![integral_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_n_chirho(
                            [
                                TyChirho::VarChirho(integral_var_chirho),
                                TyChirho::VarChirho(integral_var_chirho),
                            ],
                            TyChirho::VarChirho(integral_var_chirho),
                        ),
                    },
                ),
                (
                    "toInteger".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![integral_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(integral_var_chirho),
                            TyChirho::int_chirho(),
                        ),
                    },
                ),
            ]),
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        // Standard instances
        for ty_name_chirho in &["Int", "Char", "Bool"] {
            let ty_chirho = TyChirho::ConChirho(ty_name_chirho.to_string());
            self.add_instance_chirho(InstDeclChirho {
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: ty_chirho.clone(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            });
            self.add_instance_chirho(InstDeclChirho {
                class_name_chirho: "Show".to_string(),
                head_ty_chirho: ty_chirho.clone(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            });
        }

        for ty_name_chirho in &["Int", "Char", "Double", "Bool"] {
            let ty_chirho = TyChirho::ConChirho(ty_name_chirho.to_string());
            self.add_instance_chirho(InstDeclChirho {
                class_name_chirho: "Ord".to_string(),
                head_ty_chirho: ty_chirho,
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            });
        }

        // Ord [Char] (String ordering)
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Ord".to_string(),
            head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::ConChirho("Char".to_string()))),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // Eq [Char] (String equality)
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Eq".to_string(),
            head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::ConChirho("Char".to_string()))),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // Show [Char]
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Show".to_string(),
            head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::ConChirho("Char".to_string()))),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // Enum instances
        for ty_name_chirho in &["Int", "Char", "Bool"] {
            self.add_instance_chirho(InstDeclChirho {
                class_name_chirho: "Enum".to_string(),
                head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            });
        }

        // Bounded instances
        for ty_name_chirho in &["Int", "Char", "Bool"] {
            self.add_instance_chirho(InstDeclChirho {
                class_name_chirho: "Bounded".to_string(),
                head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            });
        }

        // Integral instances
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Integral".to_string(),
            head_ty_chirho: TyChirho::int_chirho(),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // Read instances
        for ty_name_chirho in &["Int", "Double", "Bool"] {
            self.add_instance_chirho(InstDeclChirho {
                class_name_chirho: "Read".to_string(),
                head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            });
        }

        // instance Num Int
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Num".to_string(),
            head_ty_chirho: TyChirho::int_chirho(),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // instance Num Double
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Num".to_string(),
            head_ty_chirho: TyChirho::ConChirho("Double".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // instance Fractional Double
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Fractional".to_string(),
            head_ty_chirho: TyChirho::ConChirho("Double".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // instance Eq Double
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Eq".to_string(),
            head_ty_chirho: TyChirho::ConChirho("Double".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // instance Show Double
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Show".to_string(),
            head_ty_chirho: TyChirho::ConChirho("Double".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // Ground instance: Eq String (i.e. Eq [Char])
        // This is a special case of the conditional Eq [a] instance
        // for the common String = [Char] type, avoiding the need for
        // full conditional instance dictionary support.
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Eq".to_string(),
            head_ty_chirho: TyChirho::ConChirho("[Char]".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Show".to_string(),
            head_ty_chirho: TyChirho::ConChirho("[Char]".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // Ground instance: Show [Int]
        // Avoids needing full conditional instance dictionary resolution
        // for the common case of showing integer lists.
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Show".to_string(),
            head_ty_chirho: TyChirho::ConChirho("[Int]".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // instance Eq a => Eq [a]
        let a_var_chirho = TyVarChirho(9900);
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Eq".to_string(),
            head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_var_chirho))),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![PredChirho::new_chirho(
                "Eq",
                TyChirho::VarChirho(a_var_chirho),
            )],
        });

        // instance Show a => Show [a]
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Show".to_string(),
            head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_var_chirho))),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![PredChirho::new_chirho(
                "Show",
                TyChirho::VarChirho(a_var_chirho),
            )],
        });

        // instance (Eq a, Eq b) => Eq (a, b)
        let b_var_chirho = TyVarChirho(9901);
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Eq".to_string(),
            head_ty_chirho: TyChirho::TupleChirho(vec![
                TyChirho::VarChirho(a_var_chirho),
                TyChirho::VarChirho(b_var_chirho),
            ]),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![
                PredChirho::new_chirho("Eq", TyChirho::VarChirho(a_var_chirho)),
                PredChirho::new_chirho("Eq", TyChirho::VarChirho(b_var_chirho)),
            ],
        });

        // instance Show a => Show (Maybe a)
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Show".to_string(),
            head_ty_chirho: TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(TyChirho::VarChirho(a_var_chirho)),
            ),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![PredChirho::new_chirho(
                "Show",
                TyChirho::VarChirho(a_var_chirho),
            )],
        });

        // instance (Show a, Show b) => Show (a, b)
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Show".to_string(),
            head_ty_chirho: TyChirho::TupleChirho(vec![
                TyChirho::VarChirho(a_var_chirho),
                TyChirho::VarChirho(b_var_chirho),
            ]),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![
                PredChirho::new_chirho("Show", TyChirho::VarChirho(a_var_chirho)),
                PredChirho::new_chirho("Show", TyChirho::VarChirho(b_var_chirho)),
            ],
        });

        // ── Functor / Applicative / Monad instances ──

        // instance Functor Maybe
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Functor".to_string(),
            head_ty_chirho: TyChirho::ConChirho("Maybe".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // instance Functor []
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Functor".to_string(),
            head_ty_chirho: TyChirho::ConChirho("[]".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // instance Applicative Maybe
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Applicative".to_string(),
            head_ty_chirho: TyChirho::ConChirho("Maybe".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        // instance Monad Maybe
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Monad".to_string(),
            head_ty_chirho: TyChirho::ConChirho("Maybe".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });
    }
}

/// One-way type matching: check if `pattern` (which may contain type variables)
/// matches `target` (which is ground or more specific). Returns a substitution
/// if successful.
///
/// This is NOT unification — it only instantiates variables in `pattern`,
/// never in `target`.
fn match_ty_chirho(pattern_chirho: &TyChirho, target_chirho: &TyChirho) -> Option<SubstChirho> {
    match (pattern_chirho, target_chirho) {
        (TyChirho::VarChirho(v_chirho), _) => {
            Some(SubstChirho::singleton_chirho(*v_chirho, target_chirho.clone()))
        }

        (TyChirho::ConChirho(a_chirho), TyChirho::ConChirho(b_chirho)) if a_chirho == b_chirho => {
            Some(SubstChirho::empty_chirho())
        }

        (
            TyChirho::FunChirho(pa_chirho, pb_chirho),
            TyChirho::FunChirho(ta_chirho, tb_chirho),
        ) => {
            let s1_chirho = match_ty_chirho(pa_chirho, ta_chirho)?;
            let s2_chirho = match_ty_chirho(pb_chirho, tb_chirho)?;
            Some(s1_chirho.compose_chirho(&s2_chirho))
        }

        (
            TyChirho::AppChirho(pf_chirho, pa_chirho),
            TyChirho::AppChirho(tf_chirho, ta_chirho),
        ) => {
            let s1_chirho = match_ty_chirho(pf_chirho, tf_chirho)?;
            let s2_chirho = match_ty_chirho(pa_chirho, ta_chirho)?;
            Some(s1_chirho.compose_chirho(&s2_chirho))
        }

        (TyChirho::ListChirho(p_chirho), TyChirho::ListChirho(t_chirho)) => {
            match_ty_chirho(p_chirho, t_chirho)
        }

        (TyChirho::TupleChirho(ps_chirho), TyChirho::TupleChirho(ts_chirho))
            if ps_chirho.len() == ts_chirho.len() =>
        {
            let mut subst_chirho = SubstChirho::empty_chirho();
            for (p_chirho, t_chirho) in ps_chirho.iter().zip(ts_chirho.iter()) {
                let s_chirho = match_ty_chirho(p_chirho, t_chirho)?;
                subst_chirho = subst_chirho.compose_chirho(&s_chirho);
            }
            Some(subst_chirho)
        }

        _ => None,
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn pred_display_chirho() {
        let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::int_chirho());
        assert_eq!(pred_chirho.to_string(), "Eq Int");
    }

    #[test]
    fn qual_ty_display_chirho() {
        let qt_chirho = QualTyChirho::qualified_chirho(
            vec![
                PredChirho::new_chirho("Eq", TyChirho::VarChirho(TyVarChirho(0))),
                PredChirho::new_chirho("Show", TyChirho::VarChirho(TyVarChirho(0))),
            ],
            TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(0)),
                TyChirho::string_chirho(),
            ),
        );
        assert_eq!(
            qt_chirho.to_string(),
            "(Eq t0, Show t0) => (t0 -> [Char])"
        );
    }

    #[test]
    fn unqualified_display_chirho() {
        let qt_chirho = QualTyChirho::unqualified_chirho(TyChirho::int_chirho());
        assert_eq!(qt_chirho.to_string(), "Int");
    }

    #[test]
    fn seed_standard_classes_chirho() {
        let mut env_chirho = ClassEnvChirho::new_chirho();
        env_chirho.seed_standard_chirho();

        assert!(env_chirho.has_class_chirho("Eq"));
        assert!(env_chirho.has_class_chirho("Ord"));
        assert!(env_chirho.has_class_chirho("Show"));
        assert!(env_chirho.has_class_chirho("Num"));
        assert!(env_chirho.has_class_chirho("Functor"));
        assert!(env_chirho.has_class_chirho("Applicative"));
        assert!(env_chirho.has_class_chirho("Monad"));

        assert_eq!(
            env_chirho.superclasses_chirho("Ord"),
            vec!["Eq".to_string()]
        );
        assert_eq!(
            env_chirho.superclasses_chirho("Num"),
            vec!["Eq".to_string(), "Show".to_string()]
        );
    }

    #[test]
    fn resolve_eq_int_chirho() {
        let mut env_chirho = ClassEnvChirho::new_chirho();
        env_chirho.seed_standard_chirho();

        let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::int_chirho());
        let result_chirho = env_chirho.resolve_chirho(&pred_chirho);
        assert!(result_chirho.is_some());
        // Eq Int has no context, so sub-goals should be empty
        assert!(result_chirho.unwrap().is_empty());
    }

    #[test]
    fn resolve_eq_list_int_chirho() {
        let mut env_chirho = ClassEnvChirho::new_chirho();
        env_chirho.seed_standard_chirho();

        // Eq [Int] should resolve via `instance Eq a => Eq [a]` with a=Int
        let pred_chirho = PredChirho::new_chirho(
            "Eq",
            TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
        );
        let result_chirho = env_chirho.resolve_chirho(&pred_chirho);
        assert!(result_chirho.is_some());
        let sub_goals_chirho = result_chirho.unwrap();
        // Sub-goal: Eq Int
        assert_eq!(sub_goals_chirho.len(), 1);
        assert_eq!(sub_goals_chirho[0].class_name_chirho, "Eq");
        assert_eq!(sub_goals_chirho[0].ty_chirho, TyChirho::int_chirho());
    }

    #[test]
    fn entails_eq_int_chirho() {
        let mut env_chirho = ClassEnvChirho::new_chirho();
        env_chirho.seed_standard_chirho();

        assert!(env_chirho.entails_chirho(&PredChirho::new_chirho("Eq", TyChirho::int_chirho())));
        assert!(env_chirho.entails_chirho(&PredChirho::new_chirho("Eq", TyChirho::bool_chirho())));
        assert!(env_chirho.entails_chirho(&PredChirho::new_chirho("Num", TyChirho::int_chirho())));
    }

    #[test]
    fn entails_eq_list_int_chirho() {
        let mut env_chirho = ClassEnvChirho::new_chirho();
        env_chirho.seed_standard_chirho();

        // Eq [Int] should be entailed: Eq [a] requires Eq a, and Eq Int exists
        assert!(env_chirho.entails_chirho(&PredChirho::new_chirho(
            "Eq",
            TyChirho::ListChirho(Box::new(TyChirho::int_chirho()))
        )));
    }

    #[test]
    fn no_instance_returns_none_chirho() {
        let mut env_chirho = ClassEnvChirho::new_chirho();
        env_chirho.seed_standard_chirho();

        // No instance Num Bool
        assert!(!env_chirho.entails_chirho(&PredChirho::new_chirho("Num", TyChirho::bool_chirho())));
    }

    #[test]
    fn match_ty_variable_chirho() {
        let subst_chirho =
            match_ty_chirho(&TyChirho::VarChirho(TyVarChirho(0)), &TyChirho::int_chirho());
        assert!(subst_chirho.is_some());
        let s_chirho = subst_chirho.unwrap();
        assert_eq!(
            s_chirho.apply_ty_chirho(&TyChirho::VarChirho(TyVarChirho(0))),
            TyChirho::int_chirho()
        );
    }

    #[test]
    fn match_ty_concrete_mismatch_chirho() {
        let result_chirho = match_ty_chirho(&TyChirho::int_chirho(), &TyChirho::bool_chirho());
        assert!(result_chirho.is_none());
    }

    // ---------------------------------------------------------------
    // MPTC tests
    // ---------------------------------------------------------------

    #[test]
    fn mptc_class_decl_chirho() {
        // class Convert a b where convert :: a -> b
        let a_chirho = TyVarChirho(100);
        let b_chirho = TyVarChirho(101);
        let decl_chirho = ClassDeclChirho {
            name_chirho: "Convert".to_string(),
            supers_chirho: vec![],
            var_chirho: a_chirho,
            methods_chirho: HashMap::from([(
                "convert".to_string(),
                SchemeChirho {
                    vars_chirho: vec![a_chirho, b_chirho],
                    preds_chirho: vec![],
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::VarChirho(b_chirho),
                    ),
                },
            )]),
            extra_vars_chirho: vec![b_chirho],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        };

        assert!(decl_chirho.is_mptc_chirho());
        assert_eq!(decl_chirho.all_vars_chirho(), vec![a_chirho, b_chirho]);
    }

    #[test]
    fn mptc_pred_display_chirho() {
        let pred_chirho = PredChirho::new_multi_chirho(
            "Convert",
            vec![TyChirho::int_chirho(), TyChirho::ConChirho("Double".to_string())],
        );
        assert_eq!(pred_chirho.to_string(), "Convert Int Double");
        assert_eq!(
            pred_chirho.all_tys_chirho(),
            vec![TyChirho::int_chirho(), TyChirho::ConChirho("Double".to_string())]
        );
    }

    #[test]
    fn mptc_instance_resolution_chirho() {
        // Set up: class Convert a b, instance Convert Int Double
        let mut env_chirho = ClassEnvChirho::new_chirho();

        let a_chirho = TyVarChirho(100);
        let b_chirho = TyVarChirho(101);
        env_chirho.add_class_chirho(ClassDeclChirho {
            name_chirho: "Convert".to_string(),
            supers_chirho: vec![],
            var_chirho: a_chirho,
            methods_chirho: HashMap::new(),
            extra_vars_chirho: vec![b_chirho],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        env_chirho.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Convert".to_string(),
            head_ty_chirho: TyChirho::int_chirho(),
            extra_head_tys_chirho: vec![TyChirho::ConChirho("Double".to_string())],
            context_chirho: vec![],
        });

        // Should resolve Convert Int Double
        let pred_chirho = PredChirho::new_multi_chirho(
            "Convert",
            vec![TyChirho::int_chirho(), TyChirho::ConChirho("Double".to_string())],
        );
        let result_chirho = env_chirho.resolve_chirho(&pred_chirho);
        assert!(result_chirho.is_some());
        assert!(result_chirho.unwrap().is_empty());

        // Should NOT resolve Convert Int Int (wrong second param)
        let pred2_chirho = PredChirho::new_multi_chirho(
            "Convert",
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
        );
        let result2_chirho = env_chirho.resolve_chirho(&pred2_chirho);
        assert!(result2_chirho.is_none());

        // Should NOT resolve Convert Double Double (wrong first param)
        let pred3_chirho = PredChirho::new_multi_chirho(
            "Convert",
            vec![
                TyChirho::ConChirho("Double".to_string()),
                TyChirho::ConChirho("Double".to_string()),
            ],
        );
        let result3_chirho = env_chirho.resolve_chirho(&pred3_chirho);
        assert!(result3_chirho.is_none());
    }

    #[test]
    fn mptc_polymorphic_instance_resolution_chirho() {
        // class Convert a b, instance Convert a [a] (wrap in list)
        let mut env_chirho = ClassEnvChirho::new_chirho();

        let a_chirho = TyVarChirho(100);
        let b_chirho = TyVarChirho(101);
        env_chirho.add_class_chirho(ClassDeclChirho {
            name_chirho: "Convert".to_string(),
            supers_chirho: vec![],
            var_chirho: a_chirho,
            methods_chirho: HashMap::new(),
            extra_vars_chirho: vec![b_chirho],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        let inst_var_chirho = TyVarChirho(200);
        env_chirho.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Convert".to_string(),
            head_ty_chirho: TyChirho::VarChirho(inst_var_chirho),
            extra_head_tys_chirho: vec![TyChirho::ListChirho(Box::new(
                TyChirho::VarChirho(inst_var_chirho),
            ))],
            context_chirho: vec![],
        });

        // Convert Int [Int] should match with inst_var = Int
        let pred_chirho = PredChirho::new_multi_chirho(
            "Convert",
            vec![
                TyChirho::int_chirho(),
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
            ],
        );
        let result_chirho = env_chirho.resolve_chirho(&pred_chirho);
        assert!(result_chirho.is_some());

        // Convert Int [Double] should NOT match (inconsistent: inst_var=Int vs inst_var=Double)
        let pred2_chirho = PredChirho::new_multi_chirho(
            "Convert",
            vec![
                TyChirho::int_chirho(),
                TyChirho::ListChirho(Box::new(TyChirho::ConChirho("Double".to_string()))),
            ],
        );
        let result2_chirho = env_chirho.resolve_chirho(&pred2_chirho);
        assert!(result2_chirho.is_none());
    }

    #[test]
    fn mptc_entails_chirho() {
        let mut env_chirho = ClassEnvChirho::new_chirho();

        let a_chirho = TyVarChirho(100);
        let b_chirho = TyVarChirho(101);
        env_chirho.add_class_chirho(ClassDeclChirho {
            name_chirho: "Convert".to_string(),
            supers_chirho: vec![],
            var_chirho: a_chirho,
            methods_chirho: HashMap::new(),
            extra_vars_chirho: vec![b_chirho],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });

        env_chirho.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Convert".to_string(),
            head_ty_chirho: TyChirho::int_chirho(),
            extra_head_tys_chirho: vec![TyChirho::ConChirho("Double".to_string())],
            context_chirho: vec![],
        });

        assert!(env_chirho.entails_chirho(&PredChirho::new_multi_chirho(
            "Convert",
            vec![TyChirho::int_chirho(), TyChirho::ConChirho("Double".to_string())],
        )));
        assert!(!env_chirho.entails_chirho(&PredChirho::new_multi_chirho(
            "Convert",
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
        )));
    }

    #[test]
    fn mptc_fundeps_stored_chirho() {
        // class Convert a b | a -> b
        let a_chirho = TyVarChirho(100);
        let b_chirho = TyVarChirho(101);
        let decl_chirho = ClassDeclChirho {
            name_chirho: "Convert".to_string(),
            supers_chirho: vec![],
            var_chirho: a_chirho,
            methods_chirho: HashMap::new(),
            extra_vars_chirho: vec![b_chirho],
            fundeps_chirho: vec![(vec![0], vec![1])], // a -> b
            defaults_chirho: HashMap::new(),
        };

        assert_eq!(decl_chirho.fundeps_chirho.len(), 1);
        assert_eq!(decl_chirho.fundeps_chirho[0], (vec![0], vec![1]));
    }

    #[test]
    fn mptc_fundep_improvement_chirho() {
        // class Convert a b | a -> b
        // instance Convert Int Double
        // Given predicate Convert Int t42, improvement should yield t42 = Double
        let mut env_chirho = ClassEnvChirho::new_chirho();

        let a_chirho = TyVarChirho(100);
        let b_chirho = TyVarChirho(101);
        env_chirho.add_class_chirho(ClassDeclChirho {
            name_chirho: "Convert".to_string(),
            supers_chirho: vec![],
            var_chirho: a_chirho,
            methods_chirho: HashMap::new(),
            extra_vars_chirho: vec![b_chirho],
            fundeps_chirho: vec![(vec![0], vec![1])], // a -> b
            defaults_chirho: HashMap::new(),
        });

        env_chirho.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Convert".to_string(),
            head_ty_chirho: TyChirho::int_chirho(),
            extra_head_tys_chirho: vec![TyChirho::ConChirho("Double".to_string())],
            context_chirho: vec![],
        });

        // Convert Int ?t42 — fundep should determine ?t42 = Double
        let unknown_chirho = TyVarChirho(42);
        let pred_chirho = PredChirho::new_multi_chirho(
            "Convert",
            vec![TyChirho::int_chirho(), TyChirho::VarChirho(unknown_chirho)],
        );

        let improvement_chirho = env_chirho.fundep_improve_chirho(&pred_chirho);
        assert_eq!(
            improvement_chirho.apply_ty_chirho(&TyChirho::VarChirho(unknown_chirho)),
            TyChirho::ConChirho("Double".to_string())
        );
    }

    #[test]
    fn mptc_fundep_no_improvement_on_mismatch_chirho() {
        // class Convert a b | a -> b
        // instance Convert Int Double
        // Given predicate Convert Bool t42 — no match, no improvement
        let mut env_chirho = ClassEnvChirho::new_chirho();

        let a_chirho = TyVarChirho(100);
        let b_chirho = TyVarChirho(101);
        env_chirho.add_class_chirho(ClassDeclChirho {
            name_chirho: "Convert".to_string(),
            supers_chirho: vec![],
            var_chirho: a_chirho,
            methods_chirho: HashMap::new(),
            extra_vars_chirho: vec![b_chirho],
            fundeps_chirho: vec![(vec![0], vec![1])],
            defaults_chirho: HashMap::new(),
        });

        env_chirho.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Convert".to_string(),
            head_ty_chirho: TyChirho::int_chirho(),
            extra_head_tys_chirho: vec![TyChirho::ConChirho("Double".to_string())],
            context_chirho: vec![],
        });

        let unknown_chirho = TyVarChirho(42);
        let pred_chirho = PredChirho::new_multi_chirho(
            "Convert",
            vec![TyChirho::bool_chirho(), TyChirho::VarChirho(unknown_chirho)],
        );

        let improvement_chirho = env_chirho.fundep_improve_chirho(&pred_chirho);
        // t42 should remain unimproved
        assert_eq!(
            improvement_chirho.apply_ty_chirho(&TyChirho::VarChirho(unknown_chirho)),
            TyChirho::VarChirho(unknown_chirho)
        );
    }
}
