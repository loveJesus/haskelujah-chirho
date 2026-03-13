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
use std::fmt;

use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

/// A class predicate: `ClassName Type`, e.g. `Eq Int`, `Show a`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PredChirho {
    pub class_name_chirho: String,
    pub ty_chirho: TyChirho,
}

impl PredChirho {
    pub fn new_chirho(class_name_chirho: &str, ty_chirho: TyChirho) -> Self {
        Self {
            class_name_chirho: class_name_chirho.to_string(),
            ty_chirho,
        }
    }
}

impl fmt::Display for PredChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{} {}", self.class_name_chirho, self.ty_chirho)
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
#[derive(Debug, Clone)]
pub struct ClassDeclChirho {
    pub name_chirho: String,
    /// Superclass constraints, e.g. `Eq a` for `class Eq a => Ord a`.
    pub supers_chirho: Vec<String>,
    /// The class type variable (e.g. `a` in `class Eq a`).
    pub var_chirho: TyVarChirho,
    /// Method signatures: name → type scheme.
    pub methods_chirho: HashMap<String, SchemeChirho>,
}

/// A typeclass instance declaration.
///
/// Example: `instance Eq Int where (==) = primEqInt`
#[derive(Debug, Clone)]
pub struct InstDeclChirho {
    pub class_name_chirho: String,
    /// The instance head type, e.g. `Int` in `instance Eq Int`.
    pub head_ty_chirho: TyChirho,
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
    pub fn resolve_chirho(&self, pred_chirho: &PredChirho) -> Option<Vec<PredChirho>> {
        let instances_chirho = self.instances_chirho.get(&pred_chirho.class_name_chirho)?;

        for inst_chirho in instances_chirho {
            if let Some(subst_chirho) = match_ty_chirho(&inst_chirho.head_ty_chirho, &pred_chirho.ty_chirho) {
                // Apply the matching substitution to the instance context
                let sub_goals_chirho: Vec<PredChirho> = inst_chirho
                    .context_chirho
                    .iter()
                    .map(|ctx_pred_chirho| PredChirho {
                        class_name_chirho: ctx_pred_chirho.class_name_chirho.clone(),
                        ty_chirho: subst_chirho.apply_ty_chirho(&ctx_pred_chirho.ty_chirho),
                    })
                    .collect();
                return Some(sub_goals_chirho);
            }
        }

        None
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
            ]),
        });

        // Functor
        let f_var_chirho = TyVarChirho(9004);
        self.add_class_chirho(ClassDeclChirho {
            name_chirho: "Functor".to_string(),
            supers_chirho: vec![],
            var_chirho: f_var_chirho,
            methods_chirho: HashMap::new(), // fmap signature requires higher-kinded types
        });

        // Standard instances
        for ty_name_chirho in &["Int", "Char", "Bool"] {
            let ty_chirho = TyChirho::ConChirho(ty_name_chirho.to_string());
            self.add_instance_chirho(InstDeclChirho {
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: ty_chirho.clone(),
                context_chirho: vec![],
            });
            self.add_instance_chirho(InstDeclChirho {
                class_name_chirho: "Show".to_string(),
                head_ty_chirho: ty_chirho.clone(),
                context_chirho: vec![],
            });
        }

        for ty_name_chirho in &["Int", "Char"] {
            let ty_chirho = TyChirho::ConChirho(ty_name_chirho.to_string());
            self.add_instance_chirho(InstDeclChirho {
                class_name_chirho: "Ord".to_string(),
                head_ty_chirho: ty_chirho,
                context_chirho: vec![],
            });
        }

        // instance Num Int
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Num".to_string(),
            head_ty_chirho: TyChirho::int_chirho(),
            context_chirho: vec![],
        });

        // instance Eq a => Eq [a]
        let a_var_chirho = TyVarChirho(9900);
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Eq".to_string(),
            head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_var_chirho))),
            context_chirho: vec![PredChirho::new_chirho(
                "Eq",
                TyChirho::VarChirho(a_var_chirho),
            )],
        });

        // instance Show a => Show [a]
        self.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Show".to_string(),
            head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_var_chirho))),
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
            context_chirho: vec![
                PredChirho::new_chirho("Eq", TyChirho::VarChirho(a_var_chirho)),
                PredChirho::new_chirho("Eq", TyChirho::VarChirho(b_var_chirho)),
            ],
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
        assert!(!env_chirho.has_class_chirho("Monad"));

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
}
