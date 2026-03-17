// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Type inference engine (Algorithm W)
//!
//! Implements Hindley-Milner type inference with let-generalization.
//! Walks the AST and produces typed bindings via unification.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, MatchArmChirho, RhsChirho, StmtChirho};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho as AstConstraintChirho, TypeChirho};
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

use crate::class_chirho::{ClassDeclChirho, ClassEnvChirho, InstDeclChirho, PredChirho};
use crate::env_chirho::TyEnvChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{MultChirho, SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};
use crate::unify_chirho::{UnifyErrorChirho, unify_chirho};

/// Error code range for type inference diagnostics.
const TYPE_MISMATCH_CODE_CHIRHO: u16 = 200;
const OCCURS_CHECK_CODE_CHIRHO: u16 = 201;
const UNBOUND_VAR_CODE_CHIRHO: u16 = 202;
const TUPLE_ARITY_CODE_CHIRHO: u16 = 203;
const UNSATISFIED_CONSTRAINT_CODE_CHIRHO: u16 = 204;
const SIGNATURE_MISMATCH_CODE_CHIRHO: u16 = 205;

/// Result of type inference on a module.
#[derive(Debug)]
pub struct InferResultChirho {
    /// The final substitution after inference.
    pub subst_chirho: SubstChirho,
    /// The type environment after inference (with all top-level bindings).
    pub env_chirho: TyEnvChirho,
    /// The class environment after inference.
    pub class_env_chirho: ClassEnvChirho,
    /// Diagnostics collected during inference.
    pub diagnostics_chirho: DiagnosticBundleChirho,
}

/// The inference context — carries mutable state during inference.
pub struct InferCtxChirho {
    /// Fresh type variable counter.
    next_var_chirho: u32,
    /// Type environment (scoped).
    env_chirho: TyEnvChirho,
    /// Class environment (typeclasses and instances).
    class_env_chirho: ClassEnvChirho,
    /// Deferred (wanted) typeclass predicates collected during inference.
    deferred_preds_chirho: Vec<(PredChirho, SpanChirho)>,
    /// Accumulated diagnostics.
    diagnostics_chirho: DiagnosticBundleChirho,
    /// Type synonym environment: name → (param names, expanded RHS TyChirho).
    /// Populated from `TypeAliasDeclChirho` declarations before inference.
    type_synonyms_chirho: HashMap<String, (Vec<String>, TyChirho)>,
    /// Type family environment: family name → list of equations (lhs patterns, rhs type).
    /// Each equation is (param type patterns as TyChirho, result TyChirho).
    type_families_chirho: HashMap<String, Vec<(Vec<TyChirho>, TyChirho)>>,
    /// ScopedTypeVariables: when inside a function body whose type signature has
    /// `forall a b.`, maps those names to their TyVarChirho so that where-clause
    /// type annotations and local signatures use the same type variables.
    scoped_tyvars_chirho: HashMap<String, TyVarChirho>,
    /// Record constructor field names: maps constructor name → ordered list of field names.
    /// Used for RecordWildCards expansion (`Foo{..}` fills in missing fields).
    con_field_names_chirho: HashMap<String, Vec<String>>,
}

impl InferCtxChirho {
    pub fn new_chirho() -> Self {
        let mut env_chirho = TyEnvChirho::new_chirho();
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();
        seed_builtins_chirho(&mut env_chirho);
        let mut type_synonyms_chirho = HashMap::new();
        // Built-in type synonym: type String = [Char]
        type_synonyms_chirho.insert(
            "String".to_string(),
            (vec![], TyChirho::ListChirho(Box::new(TyChirho::char_chirho()))),
        );
        // Built-in type synonym: type ShowS = String -> String
        type_synonyms_chirho.insert(
            "ShowS".to_string(),
            (vec![], TyChirho::fun_chirho(TyChirho::string_chirho(), TyChirho::string_chirho())),
        );
        Self {
            next_var_chirho: 0,
            env_chirho,
            class_env_chirho,
            deferred_preds_chirho: Vec::new(),
            diagnostics_chirho: DiagnosticBundleChirho::empty_chirho(),
            type_synonyms_chirho,
            type_families_chirho: HashMap::new(),
            scoped_tyvars_chirho: HashMap::new(),
            con_field_names_chirho: HashMap::new(),
        }
    }

    /// Register a type family (open or closed). For open families this
    /// creates an empty equation list; for closed families it stores all
    /// equations immediately.
    pub fn register_type_family_chirho(
        &mut self,
        name_chirho: String,
        equations_chirho: Vec<(Vec<TyChirho>, TyChirho)>,
    ) {
        self.type_families_chirho
            .entry(name_chirho)
            .or_default()
            .extend(equations_chirho);
    }

    /// Add an equation to an open type family from a `type instance` decl.
    pub fn register_type_family_instance_chirho(
        &mut self,
        family_name_chirho: String,
        lhs_types_chirho: Vec<TyChirho>,
        rhs_chirho: TyChirho,
    ) {
        self.type_families_chirho
            .entry(family_name_chirho)
            .or_default()
            .push((lhs_types_chirho, rhs_chirho));
    }

    /// Try to reduce a type family application `F args...` by matching
    /// against registered equations. Returns `Some(reduced)` if a match
    /// is found, `None` otherwise (stuck family application).
    pub fn reduce_type_family_chirho(
        &self,
        family_name_chirho: &str,
        args_chirho: &[TyChirho],
    ) -> Option<TyChirho> {
        let equations_chirho = self.type_families_chirho.get(family_name_chirho)?;
        for (lhs_chirho, rhs_chirho) in equations_chirho {
            if lhs_chirho.len() != args_chirho.len() {
                continue;
            }
            // Try to match each LHS pattern against the corresponding arg
            let mut bindings_chirho: HashMap<String, TyChirho> = HashMap::new();
            let mut matched_chirho = true;
            for (pat_chirho, arg_chirho) in lhs_chirho.iter().zip(args_chirho.iter()) {
                if !match_type_pattern_chirho(pat_chirho, arg_chirho, &mut bindings_chirho) {
                    matched_chirho = false;
                    break;
                }
            }
            if matched_chirho {
                return Some(substitute_type_vars_chirho(rhs_chirho, &bindings_chirho));
            }
        }
        None
    }

    /// Register a type synonym from a `TypeAliasDeclChirho`.
    pub fn register_type_synonym_chirho(
        &mut self,
        name_chirho: String,
        params_chirho: Vec<String>,
        rhs_chirho: TyChirho,
    ) {
        self.type_synonyms_chirho
            .insert(name_chirho, (params_chirho, rhs_chirho));
    }

    /// Expand type synonyms in a `TyChirho`. Handles both nullary synonyms
    /// (e.g. `String` → `[Char]`) and parameterised synonyms (e.g.
    /// `Pair Int` → `(Int, Int)` for `type Pair a = (a, a)`).
    pub fn expand_type_synonyms_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        self.expand_syn_chirho(ty_chirho, 0)
    }

    fn expand_syn_chirho(&self, ty_chirho: &TyChirho, depth_chirho: usize) -> TyChirho {
        if depth_chirho > 100 {
            return ty_chirho.clone(); // guard against cycles
        }
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => {
                if let Some((params_chirho, rhs_chirho)) =
                    self.type_synonyms_chirho.get(name_chirho)
                {
                    if params_chirho.is_empty() {
                        // Nullary synonym — expand and recurse
                        return self.expand_syn_chirho(rhs_chirho, depth_chirho + 1);
                    }
                }
                ty_chirho.clone()
            }
            TyChirho::AppChirho(fun_chirho, arg_chirho) => {
                // Collect the spine: f a1 a2 ... an
                let (head_chirho, args_chirho) = collect_app_spine_chirho(ty_chirho);
                if let TyChirho::ConChirho(name_chirho) = &head_chirho {
                    if let Some((params_chirho, rhs_chirho)) =
                        self.type_synonyms_chirho.get(name_chirho)
                    {
                        if args_chirho.len() >= params_chirho.len() {
                            // Saturated application — substitute params
                            let expanded_args_chirho: Vec<TyChirho> = args_chirho
                                .iter()
                                .map(|a_chirho| self.expand_syn_chirho(a_chirho, depth_chirho + 1))
                                .collect();
                            let mut body_chirho = rhs_chirho.clone();
                            for (p_chirho, a_chirho) in
                                params_chirho.iter().zip(expanded_args_chirho.iter())
                            {
                                body_chirho = subst_named_var_chirho(
                                    &body_chirho, p_chirho, a_chirho,
                                );
                            }
                            // Apply remaining args (over-saturated)
                            let mut result_chirho =
                                self.expand_syn_chirho(&body_chirho, depth_chirho + 1);
                            for a_chirho in &expanded_args_chirho[params_chirho.len()..] {
                                result_chirho = TyChirho::AppChirho(
                                    Box::new(result_chirho),
                                    Box::new(a_chirho.clone()),
                                );
                            }
                            return result_chirho;
                        }
                    }
                }
                // Not a synonym application — just expand sub-parts
                let ef_chirho = self.expand_syn_chirho(fun_chirho, depth_chirho);
                let ea_chirho = self.expand_syn_chirho(arg_chirho, depth_chirho);
                TyChirho::AppChirho(Box::new(ef_chirho), Box::new(ea_chirho))
            }
            TyChirho::FunChirho(a_chirho, b_chirho, _) => TyChirho::FunChirho(
                Box::new(self.expand_syn_chirho(a_chirho, depth_chirho)),
                Box::new(self.expand_syn_chirho(b_chirho, depth_chirho)),
             MultChirho::ManyChirho,),
            TyChirho::ListChirho(el_chirho) => TyChirho::ListChirho(
                Box::new(self.expand_syn_chirho(el_chirho, depth_chirho)),
            ),
            TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
                elems_chirho
                    .iter()
                    .map(|e_chirho| self.expand_syn_chirho(e_chirho, depth_chirho))
                    .collect(),
            ),
            _ => ty_chirho.clone(),
        }
    }

    /// Reduce type family applications in a `TyChirho`. Walks the type and
    /// replaces saturated type family applications with their reduced form.
    /// For example, `F Int` where `type instance F Int = Bool` becomes `Bool`.
    pub fn reduce_type_families_in_ty_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        self.reduce_families_chirho(ty_chirho, 0)
    }

    fn reduce_families_chirho(&self, ty_chirho: &TyChirho, depth_chirho: usize) -> TyChirho {
        if depth_chirho > 100 || self.type_families_chirho.is_empty() {
            return ty_chirho.clone();
        }
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => {
                // Nullary type family (no arguments)
                if let Some(reduced_chirho) = self.reduce_type_family_chirho(name_chirho, &[]) {
                    return self.reduce_families_chirho(&reduced_chirho, depth_chirho + 1);
                }
                ty_chirho.clone()
            }
            TyChirho::AppChirho(_, _) => {
                // Collect spine and check if head is a type family
                let (head_chirho, args_chirho) = collect_app_spine_chirho(ty_chirho);
                if let TyChirho::ConChirho(name_chirho) = &head_chirho {
                    // Recursively reduce arguments first
                    let reduced_args_chirho: Vec<TyChirho> = args_chirho
                        .iter()
                        .map(|a_chirho| self.reduce_families_chirho(a_chirho, depth_chirho + 1))
                        .collect();
                    if let Some(result_chirho) =
                        self.reduce_type_family_chirho(name_chirho, &reduced_args_chirho)
                    {
                        return self.reduce_families_chirho(&result_chirho, depth_chirho + 1);
                    }
                    // Not a family — rebuild with reduced args
                    let mut result_chirho = head_chirho.clone();
                    for a_chirho in &reduced_args_chirho {
                        result_chirho =
                            TyChirho::AppChirho(Box::new(result_chirho), Box::new(a_chirho.clone()));
                    }
                    return result_chirho;
                }
                // Head is not a Con — just reduce sub-parts
                let ef_chirho = self.reduce_families_chirho(
                    match ty_chirho {
                        TyChirho::AppChirho(f, _) => f,
                        _ => unreachable!(),
                    },
                    depth_chirho,
                );
                let ea_chirho = self.reduce_families_chirho(
                    match ty_chirho {
                        TyChirho::AppChirho(_, a) => a,
                        _ => unreachable!(),
                    },
                    depth_chirho,
                );
                TyChirho::AppChirho(Box::new(ef_chirho), Box::new(ea_chirho))
            }
            TyChirho::FunChirho(a_chirho, b_chirho, m_chirho) => TyChirho::FunChirho(
                Box::new(self.reduce_families_chirho(a_chirho, depth_chirho)),
                Box::new(self.reduce_families_chirho(b_chirho, depth_chirho)),
                *m_chirho,
            ),
            TyChirho::ListChirho(el_chirho) => TyChirho::ListChirho(
                Box::new(self.reduce_families_chirho(el_chirho, depth_chirho)),
            ),
            TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
                elems_chirho
                    .iter()
                    .map(|e_chirho| self.reduce_families_chirho(e_chirho, depth_chirho))
                    .collect(),
            ),
            TyChirho::ForallChirho { vars_chirho, body_chirho } => TyChirho::ForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.reduce_families_chirho(body_chirho, depth_chirho)),
            },
            _ => ty_chirho.clone(),
        }
    }

    /// Generate a fresh unification variable.
    pub fn fresh_var_chirho(&mut self) -> TyChirho {
        let var_chirho = TyVarChirho(self.next_var_chirho);
        self.next_var_chirho += 1;
        TyChirho::VarChirho(var_chirho)
    }

    /// Instantiate a type scheme with fresh unification variables.
    /// Any predicates on the scheme are also instantiated and deferred.
    pub fn instantiate_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
        span_chirho: SpanChirho,
    ) -> TyChirho {
        if scheme_chirho.vars_chirho.is_empty() && scheme_chirho.preds_chirho.is_empty() {
            return scheme_chirho.ty_chirho.clone();
        }

        let mut subst_chirho = SubstChirho::empty_chirho();
        for v_chirho in &scheme_chirho.vars_chirho {
            subst_chirho.insert_chirho(*v_chirho, self.fresh_var_chirho());
        }

        // Instantiate and defer predicates
        for pred_chirho in &scheme_chirho.preds_chirho {
            let instantiated_ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
            self.deferred_preds_chirho.push((
                PredChirho::new_chirho(&pred_chirho.class_name_chirho, instantiated_ty_chirho),
                span_chirho,
            ));
        }

        subst_chirho.apply_ty_chirho(&scheme_chirho.ty_chirho)
    }

    /// Generalize a type into a scheme by quantifying over all free variables
    /// not free in the environment. Also partitions deferred predicates:
    /// predicates whose type variables are all being generalized become part
    /// of the scheme; the rest remain deferred.
    ///
    /// Deferred predicates are assumed to already have substitutions applied
    /// (via `apply_subst_all_chirho` during inference).
    pub fn generalize_chirho(&mut self, ty_chirho: &TyChirho) -> SchemeChirho {
        let env_fvs_chirho = self.env_chirho.free_vars_chirho();
        let ty_fvs_chirho = ty_chirho.free_vars_chirho();
        let vars_chirho: Vec<TyVarChirho> = ty_fvs_chirho
            .into_iter()
            .filter(|v_chirho| !env_fvs_chirho.contains(v_chirho))
            .collect();

        // Partition deferred predicates: those whose type vars are all being
        // generalized go into the scheme; the rest stay deferred.
        let mut scheme_preds_chirho = Vec::new();
        let mut remaining_chirho = Vec::new();
        for (pred_chirho, span_chirho) in self.deferred_preds_chirho.drain(..) {
            let pred_fvs_chirho = pred_chirho.ty_chirho.free_vars_chirho();
            if pred_fvs_chirho
                .iter()
                .all(|v_chirho| vars_chirho.contains(v_chirho))
            {
                scheme_preds_chirho.push(SchemePredChirho {
                    class_name_chirho: pred_chirho.class_name_chirho,
                    ty_chirho: pred_chirho.ty_chirho,
                });
            } else {
                remaining_chirho.push((pred_chirho, span_chirho));
            }
        }
        self.deferred_preds_chirho = remaining_chirho;

        // Deduplicate predicates
        scheme_preds_chirho.dedup();

        // Context reduction: remove predicates that are entailed by
        // other predicates via the superclass hierarchy.
        // E.g. if we have (Eq a, Num a), remove Eq a since Num has Eq
        // as a superclass.
        let keep_chirho: Vec<bool> = scheme_preds_chirho
            .iter()
            .map(|pred_chirho| {
                !scheme_preds_chirho.iter().any(|other_chirho| {
                    other_chirho.class_name_chirho != pred_chirho.class_name_chirho
                        && other_chirho.ty_chirho == pred_chirho.ty_chirho
                        && self.is_transitive_superclass_chirho(
                            &pred_chirho.class_name_chirho,
                            &other_chirho.class_name_chirho,
                        )
                })
            })
            .collect();
        scheme_preds_chirho = scheme_preds_chirho
            .into_iter()
            .zip(keep_chirho)
            .filter(|(_, keep_chirho)| *keep_chirho)
            .map(|(p_chirho, _)| p_chirho)
            .collect();

        SchemeChirho {
            vars_chirho,
            preds_chirho: scheme_preds_chirho,
            ty_chirho: ty_chirho.clone(),
        }
    }

    /// Check if `potential_super_chirho` is a (transitive) superclass of
    /// `sub_chirho`. For example, `Eq` is a superclass of `Ord` and also
    /// of `Num` (since Num has Eq in its superclass list).
    fn is_transitive_superclass_chirho(
        &self,
        potential_super_chirho: &str,
        sub_chirho: &str,
    ) -> bool {
        let supers_chirho = self.class_env_chirho.superclasses_chirho(sub_chirho);
        for s_chirho in &supers_chirho {
            if s_chirho == potential_super_chirho
                || self.is_transitive_superclass_chirho(potential_super_chirho, s_chirho)
            {
                return true;
            }
        }
        false
    }

    /// Apply a substitution to the type environment and deferred predicates.
    fn apply_subst_all_chirho(&mut self, subst_chirho: &SubstChirho) {
        self.env_chirho.apply_subst_chirho(subst_chirho);
        for (pred_chirho, _span_chirho) in &mut self.deferred_preds_chirho {
            pred_chirho.ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
        }
    }

    /// Record a unification error as a diagnostic.
    fn report_unify_error_chirho(&mut self, err_chirho: &UnifyErrorChirho) {
        match err_chirho {
            UnifyErrorChirho::MismatchChirho {
                expected_chirho,
                actual_chirho,
                span_chirho,
            } => {
                self.diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(TYPE_MISMATCH_CODE_CHIRHO),
                        format!(
                            "type mismatch: expected `{expected_chirho}`, found `{actual_chirho}`"
                        ),
                        *span_chirho,
                    )
                    .with_note_chirho(format!("expected type: {expected_chirho}"))
                    .with_note_chirho(format!("   found type: {actual_chirho}")),
                );
            }
            UnifyErrorChirho::OccursCheckChirho {
                var_chirho,
                ty_chirho,
                span_chirho,
            } => {
                self.diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(OCCURS_CHECK_CODE_CHIRHO),
                        format!(
                            "infinite type: `{var_chirho}` occurs in `{ty_chirho}`"
                        ),
                        *span_chirho,
                    ),
                );
            }
            UnifyErrorChirho::TupleArityChirho {
                expected_chirho,
                actual_chirho,
                span_chirho,
            } => {
                self.diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(TUPLE_ARITY_CODE_CHIRHO),
                        format!(
                            "tuple arity mismatch: expected {expected_chirho} elements, found {actual_chirho}"
                        ),
                        *span_chirho,
                    ),
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // AST type conversion
    // -----------------------------------------------------------------------

    /// Convert an AST `TypeChirho` (surface syntax) to an internal `TyChirho`.
    ///
    /// Named type variables are mapped to fresh unification variables via
    /// `var_map_chirho`. This ensures that `a -> a` uses the same variable
    /// for both occurrences of `a`.
    pub fn ast_type_to_ty_chirho(
        &mut self,
        ast_ty_chirho: &TypeChirho,
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> TyChirho {
        match ast_ty_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho().to_string();
                let tv_chirho = var_map_chirho
                    .entry(text_chirho)
                    .or_insert_with(|| {
                        let v_chirho = TyVarChirho(self.next_var_chirho);
                        self.next_var_chirho += 1;
                        v_chirho
                    });
                TyChirho::VarChirho(*tv_chirho)
            }
            TypeChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho().to_string();
                let raw_chirho = TyChirho::ConChirho(text_chirho);
                // Eagerly expand nullary type synonyms (e.g. String → [Char])
                self.expand_type_synonyms_chirho(&raw_chirho)
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                let f_chirho = self.ast_type_to_ty_chirho(fun_chirho, var_map_chirho);
                let a_chirho = self.ast_type_to_ty_chirho(arg_chirho, var_map_chirho);
                let raw_chirho = TyChirho::AppChirho(Box::new(f_chirho), Box::new(a_chirho));
                // Expand parameterised type synonyms (e.g. Pair Int → (Int, Int))
                let expanded_chirho = self.expand_type_synonyms_chirho(&raw_chirho);
                // Reduce type family applications (e.g. F Int → Bool)
                self.reduce_type_families_in_ty_chirho(&expanded_chirho)
            }
            TypeChirho::FunChirho {
                arg_chirho,
                mult_chirho,
                result_chirho,
                ..
            } => {
                let a_chirho = self.ast_type_to_ty_chirho(arg_chirho, var_map_chirho);
                let r_chirho = self.ast_type_to_ty_chirho(result_chirho, var_map_chirho);
                let m_chirho = match mult_chirho {
                    Some(haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::OneChirho) => MultChirho::OneChirho,
                    _ => MultChirho::ManyChirho,
                };
                TyChirho::FunChirho(Box::new(a_chirho), Box::new(r_chirho), m_chirho)
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let elems_chirho: Vec<TyChirho> = elements_chirho
                    .iter()
                    .map(|e_chirho| self.ast_type_to_ty_chirho(e_chirho, var_map_chirho))
                    .collect();
                TyChirho::TupleChirho(elems_chirho)
            }
            TypeChirho::ListChirho {
                element_chirho, ..
            } => {
                let elem_chirho = self.ast_type_to_ty_chirho(element_chirho, var_map_chirho);
                TyChirho::ListChirho(Box::new(elem_chirho))
            }
            TypeChirho::ParenChirho {
                inner_chirho, ..
            } => self.ast_type_to_ty_chirho(inner_chirho, var_map_chirho),
            TypeChirho::QualChirho {
                body_chirho, ..
            } => {
                // Qualified types: convert the body, constraints are handled separately
                self.ast_type_to_ty_chirho(body_chirho, var_map_chirho)
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                // Register forall-bound variables as fresh type vars and produce
                // TyChirho::ForallChirho to preserve higher-rank type structure.
                let mut bound_vars_chirho = Vec::new();
                for v_chirho in vars_chirho {
                    let tv_chirho = TyVarChirho(self.next_var_chirho);
                    self.next_var_chirho += 1;
                    var_map_chirho.insert(v_chirho.text_chirho().to_string(), tv_chirho);
                    bound_vars_chirho.push(tv_chirho);
                }
                let body_ty_chirho = self.ast_type_to_ty_chirho(body_chirho, var_map_chirho);
                TyChirho::ForallChirho {
                    vars_chirho: bound_vars_chirho,
                    body_chirho: Box::new(body_ty_chirho),
                }
            }
            // DataKinds: promoted constructor is a type-level constant
            TypeChirho::PromotedConChirho { name_chirho, .. } => {
                TyChirho::ConChirho(name_chirho.text_chirho().to_string())
            }
            // DataKinds: promoted list is represented as nested type application
            TypeChirho::PromotedListChirho { elements_chirho, .. } => {
                // '[] → Con("'[]"), '[a, b] → App(App(Con("':"), a), App(App(Con("':"), b), Con("'[]")))
                let nil_chirho = TyChirho::ConChirho("'[]".to_string());
                elements_chirho.iter().rev().fold(nil_chirho, |acc_chirho, elem_chirho| {
                    let elem_ty_chirho = self.ast_type_to_ty_chirho(elem_chirho, var_map_chirho);
                    let cons_chirho = TyChirho::ConChirho("':".to_string());
                    TyChirho::AppChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(cons_chirho),
                            Box::new(elem_ty_chirho),
                        )),
                        Box::new(acc_chirho),
                    )
                })
            }
            // PartialTypeSignatures: `_` in a type signature is a wildcard that
            // becomes a fresh unification variable. Emit warning W4201 so the
            // programmer is informed of the inferred type position.
            TypeChirho::WildcardChirho { span_chirho } => {
                let fresh_ty_chirho = self.fresh_var_chirho();
                let diag_chirho = DiagnosticChirho::warning_with_code_chirho(
                    ErrorCodeChirho::warning_chirho(4201),
                    "found wildcard `_` in type signature (PartialTypeSignatures)".to_string(),
                    *span_chirho,
                );
                self.diagnostics_chirho.push_chirho(diag_chirho);
                fresh_ty_chirho
            }
        }
    }

    /// Convert an AST `TypeChirho` to a `SchemeChirho` (with quantified variables
    /// and constraints extracted).
    pub fn ast_type_to_scheme_chirho(&mut self, ast_ty_chirho: &TypeChirho) -> SchemeChirho {
        // Seed var_map with scoped type variables so that where-clause annotations
        // referring to the enclosing function's forall-bound vars reuse the same TyVarChirho.
        let mut var_map_chirho: HashMap<String, TyVarChirho> =
            self.scoped_tyvars_chirho.clone();
        let mut preds_chirho = Vec::new();

        // Extract constraints from QualChirho wrapping
        let body_ast_chirho = Self::extract_constraints_chirho(ast_ty_chirho, &mut preds_chirho);
        let ty_chirho = self.ast_type_to_ty_chirho(body_ast_chirho, &mut var_map_chirho);

        let scheme_preds_chirho: Vec<SchemePredChirho> = preds_chirho
            .iter()
            .map(|c_chirho| {
                let class_name_chirho = c_chirho.class_chirho.text_chirho().to_string();
                let pred_ty_chirho = if let Some(first_arg_chirho) = c_chirho.args_chirho.first() {
                    self.ast_type_to_ty_chirho(first_arg_chirho, &mut var_map_chirho)
                } else {
                    self.fresh_var_chirho()
                };
                SchemePredChirho {
                    class_name_chirho,
                    ty_chirho: pred_ty_chirho,
                }
            })
            .collect();

        // Peel off the outermost ForallChirho produced by ast_type_to_ty_chirho.
        // The vars in a top-level ForallChirho become scheme vars; nested ForallChirho
        // (inside function args) are preserved for rank-N polymorphism.
        let (outer_forall_vars_chirho, inner_ty_chirho) = Self::peel_forall_chirho(ty_chirho);

        // Only quantify over vars that are free in the resulting type.
        // Vars bound by inner ForallChirho are NOT free and should not be in scheme vars.
        let free_in_ty_chirho = inner_ty_chirho.free_vars_chirho();
        let vars_chirho: Vec<TyVarChirho> = if !outer_forall_vars_chirho.is_empty() {
            // Top-level forall: use those vars as the scheme vars
            outer_forall_vars_chirho
        } else {
            // No explicit forall: quantify over all free vars from var_map
            var_map_chirho
                .values()
                .copied()
                .filter(|v_chirho| free_in_ty_chirho.contains(v_chirho))
                .collect()
        };

        SchemeChirho {
            vars_chirho,
            preds_chirho: scheme_preds_chirho,
            ty_chirho: inner_ty_chirho,
        }
    }

    /// Peel off the outermost ForallChirho from a TyChirho, collecting bound vars.
    /// Returns (outer_vars, body). If no ForallChirho at top, returns (empty, original).
    fn peel_forall_chirho(ty_chirho: TyChirho) -> (Vec<TyVarChirho>, TyChirho) {
        match ty_chirho {
            TyChirho::ForallChirho { vars_chirho, body_chirho } => {
                let (mut inner_vars_chirho, inner_body_chirho) =
                    Self::peel_forall_chirho(*body_chirho);
                let mut all_vars_chirho = vars_chirho;
                all_vars_chirho.append(&mut inner_vars_chirho);
                (all_vars_chirho, inner_body_chirho)
            }
            other_chirho => (vec![], other_chirho),
        }
    }

    /// Check whether an AST type has an explicit `forall` at the top level.
    fn has_explicit_forall_chirho(ast_ty_chirho: &TypeChirho) -> bool {
        match ast_ty_chirho {
            TypeChirho::ForallChirho { .. } => true,
            TypeChirho::QualChirho { body_chirho, .. } => {
                Self::has_explicit_forall_chirho(body_chirho)
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                Self::has_explicit_forall_chirho(inner_chirho)
            }
            _ => false,
        }
    }

    /// Extract constraint context from a qualified type, returning the body.
    fn extract_constraints_chirho<'a>(
        ast_ty_chirho: &'a TypeChirho,
        out_chirho: &mut Vec<&'a AstConstraintChirho>,
    ) -> &'a TypeChirho {
        match ast_ty_chirho {
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                for c_chirho in context_chirho {
                    out_chirho.push(c_chirho);
                }
                Self::extract_constraints_chirho(body_chirho, out_chirho)
            }
            TypeChirho::ForallChirho {
                body_chirho, ..
            } => Self::extract_constraints_chirho(body_chirho, out_chirho),
            other_chirho => other_chirho,
        }
    }

    // -----------------------------------------------------------------------
    // Class and instance declaration processing
    // -----------------------------------------------------------------------

    /// Process a class declaration from the AST and register it in the class env.
    fn process_class_decl_chirho(&mut self, decl_chirho: &DeclChirho) {
        if let DeclChirho::ClassDeclChirho {
            context_chirho,
            name_chirho,
            type_vars_chirho,
            methods_chirho,
            associated_tfs_chirho,
            fundeps_chirho: ast_fundeps_chirho,
            ..
        } = decl_chirho
        {
            let class_name_chirho = name_chirho.text_chirho().to_string();

            // Generate fresh type variables for all class params
            let class_tv_chirho = TyVarChirho(self.next_var_chirho);
            self.next_var_chirho += 1;

            let extra_vars_chirho: Vec<TyVarChirho> = type_vars_chirho
                .iter()
                .skip(1)
                .map(|_| {
                    let v_chirho = TyVarChirho(self.next_var_chirho);
                    self.next_var_chirho += 1;
                    v_chirho
                })
                .collect();

            // Superclasses from context
            let supers_chirho: Vec<String> = context_chirho
                .iter()
                .map(|c_chirho| c_chirho.class_chirho.text_chirho().to_string())
                .collect();

            // Method signatures and optional default implementations
            let mut method_map_chirho = HashMap::new();
            let mut defaults_map_chirho = HashMap::new();
            for method_chirho in methods_chirho {
                let method_name_chirho = method_chirho.name_chirho.text_chirho().to_string();
                let scheme_chirho = self.ast_type_to_scheme_chirho(&method_chirho.ty_chirho);
                method_map_chirho.insert(method_name_chirho.clone(), scheme_chirho.clone());

                // Also add method to the type environment so it can be used
                self.env_chirho
                    .bind_chirho(method_name_chirho.clone(), scheme_chirho);

                // Capture default implementation if present
                if let Some(ref default_arms_chirho) = method_chirho.default_chirho {
                    defaults_map_chirho
                        .insert(method_name_chirho, default_arms_chirho.clone());
                }
            }

            // Convert AST fundeps (variable names) to indices into type_vars_chirho
            let var_names_chirho: Vec<String> = type_vars_chirho
                .iter()
                .map(|v_chirho| v_chirho.text_chirho().to_string())
                .collect();
            let resolved_fundeps_chirho: Vec<(Vec<usize>, Vec<usize>)> = ast_fundeps_chirho
                .iter()
                .map(|(from_chirho, to_chirho)| {
                    let from_idx_chirho: Vec<usize> = from_chirho
                        .iter()
                        .filter_map(|n_chirho| var_names_chirho.iter().position(|v_chirho| v_chirho == n_chirho))
                        .collect();
                    let to_idx_chirho: Vec<usize> = to_chirho
                        .iter()
                        .filter_map(|n_chirho| var_names_chirho.iter().position(|v_chirho| v_chirho == n_chirho))
                        .collect();
                    (from_idx_chirho, to_idx_chirho)
                })
                .collect();

            self.class_env_chirho.add_class_chirho(ClassDeclChirho {
                name_chirho: class_name_chirho.clone(),
                supers_chirho,
                var_chirho: class_tv_chirho,
                methods_chirho: method_map_chirho,
                extra_vars_chirho,
                fundeps_chirho: resolved_fundeps_chirho,
                defaults_chirho: defaults_map_chirho,
            });

            // Register associated type families as open type families
            for atf_chirho in associated_tfs_chirho {
                let tf_name_chirho = atf_chirho.name_chirho.text_chirho().to_string();
                self.register_type_family_chirho(tf_name_chirho, vec![]);
            }
        }
    }

    /// Process an instance declaration from the AST and register it.
    fn process_instance_decl_chirho(&mut self, decl_chirho: &DeclChirho) {
        if let DeclChirho::InstanceDeclChirho {
            context_chirho,
            class_chirho,
            types_chirho,
            assoc_tf_instances_chirho,
            ..
        } = decl_chirho
        {
            let class_name_chirho = class_chirho.text_chirho().to_string();

            let mut var_map_chirho = HashMap::new();

            // Instance head type (e.g., `Int` in `instance Eq Int`, or
            // `[a]` in `instance Eq a => Eq [a]`)
            let head_ty_chirho = if let Some(first_ty_chirho) = types_chirho.first() {
                self.ast_type_to_ty_chirho(first_ty_chirho, &mut var_map_chirho)
            } else {
                self.fresh_var_chirho()
            };

            // Context constraints (e.g., `Eq a` in `instance Eq a => Eq [a]`)
            let inst_context_chirho: Vec<PredChirho> = context_chirho
                .iter()
                .map(|c_chirho| {
                    let cn_chirho = c_chirho.class_chirho.text_chirho().to_string();
                    let ct_chirho = if let Some(arg_chirho) = c_chirho.args_chirho.first() {
                        self.ast_type_to_ty_chirho(arg_chirho, &mut var_map_chirho)
                    } else {
                        self.fresh_var_chirho()
                    };
                    PredChirho::new_chirho(&cn_chirho, ct_chirho)
                })
                .collect();

            // For MPTCs, extra head types come from types_chirho[1..]
            let extra_head_tys_chirho: Vec<TyChirho> = types_chirho
                .iter()
                .skip(1)
                .map(|t_chirho| self.ast_type_to_ty_chirho(t_chirho, &mut var_map_chirho))
                .collect();

            // Expand type synonyms in the instance head (e.g. String → [Char])
            let head_ty_chirho = self.expand_type_synonyms_chirho(&head_ty_chirho);
            let extra_head_tys_chirho: Vec<TyChirho> = extra_head_tys_chirho
                .into_iter()
                .map(|t_chirho| self.expand_type_synonyms_chirho(&t_chirho))
                .collect();

            self.class_env_chirho.add_instance_chirho(InstDeclChirho {
                class_name_chirho,
                head_ty_chirho,
                extra_head_tys_chirho,
                context_chirho: inst_context_chirho,
            });

            // Register associated type family instances from this instance decl
            for atfi_chirho in assoc_tf_instances_chirho {
                let fname_chirho = atfi_chirho.family_name_chirho.text_chirho().to_string();
                let lhs_chirho: Vec<TyChirho> = atfi_chirho
                    .lhs_types_chirho
                    .iter()
                    .map(|t_chirho| ast_type_to_syn_rhs_chirho(t_chirho, &[]))
                    .collect();
                let rhs_ty_chirho = ast_type_to_syn_rhs_chirho(&atfi_chirho.rhs_chirho, &[]);
                self.register_type_family_instance_chirho(fname_chirho, lhs_chirho, rhs_ty_chirho);
            }
        }
    }

    // -----------------------------------------------------------------------
    // Expression inference
    // -----------------------------------------------------------------------

    /// Infer the type of an expression, returning (substitution, type).
    pub fn infer_expr_chirho(
        &mut self,
        expr_chirho: &ExprChirho,
    ) -> (SubstChirho, TyChirho) {
        match expr_chirho {
            ExprChirho::LitChirho(lit_chirho) => {
                let ty_chirho = infer_lit_chirho(lit_chirho);
                (SubstChirho::empty_chirho(), ty_chirho)
            }

            ExprChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                let span_chirho = name_chirho.span_chirho();
                let scheme_opt_chirho = self.env_chirho.lookup_chirho(text_chirho).cloned();
                match scheme_opt_chirho {
                    Some(scheme_chirho) => {
                        let ty_chirho =
                            self.instantiate_chirho(&scheme_chirho, span_chirho);
                        (SubstChirho::empty_chirho(), ty_chirho)
                    }
                    None => {
                        // Typed holes: `_` or `_foo` in expression position are
                        // treated as holes — emit a warning, not an error, and
                        // assign a fresh type variable. This lets code with holes
                        // compile (for type exploration / incremental development).
                        if text_chirho.starts_with('_') {
                            let hole_ty_chirho = self.fresh_var_chirho();
                            let diag_chirho = DiagnosticChirho::warning_with_code_chirho(
                                ErrorCodeChirho::warning_chirho(4200),
                                format!("found hole: `{text_chirho}` :: _"),
                                span_chirho,
                            );
                            self.diagnostics_chirho.push_chirho(diag_chirho);
                            (SubstChirho::empty_chirho(), hole_ty_chirho)
                        } else {
                            let mut diag_chirho = DiagnosticChirho::error_with_code_chirho(
                                ErrorCodeChirho::error_chirho(UNBOUND_VAR_CODE_CHIRHO),
                                format!("unbound variable: `{text_chirho}`"),
                                span_chirho,
                            );
                            // Add "did you mean?" suggestions from the type env.
                            let candidates_chirho = self.env_chirho.all_names_chirho();
                            let max_dist_chirho = haskelujah_diagnostics_chirho::suggest_chirho::default_max_distance_chirho(text_chirho.len());
                            let suggestions_chirho = haskelujah_diagnostics_chirho::suggest_chirho::suggest_similar_names_chirho(
                                text_chirho,
                                candidates_chirho.into_iter(),
                                max_dist_chirho,
                                3,
                            );
                            if let Some(note_chirho) = haskelujah_diagnostics_chirho::suggest_chirho::format_did_you_mean_chirho(&suggestions_chirho) {
                                diag_chirho = diag_chirho.with_note_chirho(note_chirho);
                            }
                            self.diagnostics_chirho.push_chirho(diag_chirho);
                            (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                        }
                    }
                }
            }

            ExprChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                let span_chirho = name_chirho.span_chirho();
                let scheme_opt_chirho = self.env_chirho.lookup_chirho(text_chirho).cloned();
                match scheme_opt_chirho {
                    Some(scheme_chirho) => {
                        let ty_chirho =
                            self.instantiate_chirho(&scheme_chirho, span_chirho);
                        (SubstChirho::empty_chirho(), ty_chirho)
                    }
                    None => {
                        (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                    }
                }
            }

            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                span_chirho,
            } => {
                let (s1_chirho, fun_ty_chirho) = self.infer_expr_chirho(fun_chirho);
                self.apply_subst_all_chirho(&s1_chirho);

                let (s2_chirho, arg_ty_chirho) = self.infer_expr_chirho(arg_chirho);
                let fun_ty_sub_chirho = s2_chirho.apply_ty_chirho(&fun_ty_chirho);

                let result_ty_chirho = self.fresh_var_chirho();
                let expected_fun_chirho =
                    TyChirho::fun_chirho(arg_ty_chirho, result_ty_chirho.clone());

                match unify_chirho(&fun_ty_sub_chirho, &expected_fun_chirho, *span_chirho) {
                    Ok(s3_chirho) => {
                        let combined_chirho = s3_chirho.compose_chirho(&s2_chirho.compose_chirho(&s1_chirho));
                        let final_ty_chirho = combined_chirho.apply_ty_chirho(&result_ty_chirho);
                        self.apply_subst_all_chirho(&s3_chirho);
                        (combined_chirho, final_ty_chirho)
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        (s2_chirho.compose_chirho(&s1_chirho), self.fresh_var_chirho())
                    }
                }
            }

            ExprChirho::LamChirho {
                pats_chirho,
                body_chirho,
                ..
            } => {
                self.env_chirho.push_scope_chirho();

                let mut param_tys_chirho = Vec::new();
                for pat_chirho in pats_chirho {
                    let pat_ty_chirho = self.fresh_var_chirho();
                    self.bind_pat_chirho(pat_chirho, &pat_ty_chirho);
                    param_tys_chirho.push(pat_ty_chirho);
                }

                let (body_subst_chirho, body_ty_chirho) = self.infer_expr_chirho(body_chirho);
                self.env_chirho.pop_scope_chirho();

                let mut result_chirho = body_ty_chirho;
                for param_ty_chirho in param_tys_chirho.into_iter().rev() {
                    let param_sub_chirho = body_subst_chirho.apply_ty_chirho(&param_ty_chirho);
                    result_chirho = TyChirho::fun_chirho(param_sub_chirho, result_chirho);
                }

                (body_subst_chirho, result_chirho)
            }

            ExprChirho::IfChirho {
                cond_chirho,
                then_chirho,
                else_chirho,
                span_chirho,
            } => {
                let (s1_chirho, cond_ty_chirho) = self.infer_expr_chirho(cond_chirho);
                self.apply_subst_all_chirho(&s1_chirho);

                // Condition must be Bool
                match unify_chirho(&cond_ty_chirho, &TyChirho::bool_chirho(), *span_chirho) {
                    Ok(sc_chirho) => {
                        self.apply_subst_all_chirho(&sc_chirho);
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                    }
                }

                let (s2_chirho, then_ty_chirho) = self.infer_expr_chirho(then_chirho);
                self.apply_subst_all_chirho(&s2_chirho);

                let (s3_chirho, else_ty_chirho) = self.infer_expr_chirho(else_chirho);

                // Then and else branches must have the same type
                let then_sub_chirho = s3_chirho.apply_ty_chirho(&then_ty_chirho);
                match unify_chirho(&then_sub_chirho, &else_ty_chirho, *span_chirho) {
                    Ok(s4_chirho) => {
                        let combined_chirho =
                            s4_chirho.compose_chirho(&s3_chirho.compose_chirho(&s2_chirho.compose_chirho(&s1_chirho)));
                        let final_ty_chirho = combined_chirho.apply_ty_chirho(&else_ty_chirho);
                        self.apply_subst_all_chirho(&s4_chirho);
                        (combined_chirho, final_ty_chirho)
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        let combined_chirho = s3_chirho.compose_chirho(&s2_chirho.compose_chirho(&s1_chirho));
                        (combined_chirho, else_ty_chirho)
                    }
                }
            }

            ExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                self.env_chirho.push_scope_chirho();
                let mut subst_chirho = SubstChirho::empty_chirho();

                // Collect local type signatures
                let mut local_sigs_chirho: HashMap<String, haskelujah_ast_chirho::ty_chirho::TypeChirho> = HashMap::new();
                for bind_chirho in binds_chirho {
                    if let haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                        name_chirho, ty_chirho, ..
                    } = bind_chirho
                    {
                        local_sigs_chirho.insert(
                            name_chirho.text_chirho().to_string(),
                            ty_chirho.clone(),
                        );
                    }
                }

                // Pre-bind function names with fresh types (letrec)
                let mut pre_let_tys_chirho: Vec<(String, TyChirho)> = Vec::new();
                for bind_chirho in binds_chirho {
                    if let haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                        name_chirho, ..
                    } = bind_chirho
                    {
                        let fresh_ty_chirho = self.fresh_var_chirho();
                        let name_str_chirho = name_chirho.text_chirho().to_string();
                        self.env_chirho.bind_chirho(
                            name_str_chirho.clone(),
                            SchemeChirho::mono_chirho(fresh_ty_chirho.clone()),
                        );
                        pre_let_tys_chirho.push((name_str_chirho, fresh_ty_chirho));
                    }
                }

                for bind_chirho in binds_chirho {
                    match bind_chirho {
                        haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                            name_chirho,
                            matches_chirho,
                            span_chirho,
                        } => {
                            let (s_chirho, ty_chirho) =
                                self.infer_matches_chirho(matches_chirho, *span_chirho);
                            subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&s_chirho);

                            // Unify pre-bound type with inferred type
                            let name_str_chirho = name_chirho.text_chirho().to_string();
                            if let Some((_, pre_ty_chirho)) = pre_let_tys_chirho
                                .iter()
                                .find(|(n_chirho, _)| n_chirho == &name_str_chirho)
                            {
                                let pre_ty_sub_chirho = subst_chirho.apply_ty_chirho(pre_ty_chirho);
                                if let Ok(us_chirho) = unify_chirho(
                                    &pre_ty_sub_chirho,
                                    &ty_chirho,
                                    *span_chirho,
                                ) {
                                    subst_chirho = us_chirho.compose_chirho(&subst_chirho);
                                    self.apply_subst_all_chirho(&us_chirho);
                                }
                            }

                            // Check against local type signature if present
                            if let Some(sig_ast_chirho) = local_sigs_chirho.get(&name_str_chirho) {
                                let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                                let sig_ty_raw_chirho = self.instantiate_chirho(&sig_scheme_chirho, *span_chirho);
                                let sig_ty_chirho = self.reduce_type_families_in_ty_chirho(&sig_ty_raw_chirho);
                                let inferred_sub_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
                                let inferred_sub_chirho = self.reduce_type_families_in_ty_chirho(&inferred_sub_chirho);
                                match unify_chirho(&inferred_sub_chirho, &sig_ty_chirho, *span_chirho) {
                                    Ok(sig_s_chirho) => {
                                        subst_chirho = sig_s_chirho.compose_chirho(&subst_chirho);
                                        self.apply_subst_all_chirho(&sig_s_chirho);
                                    }
                                    Err(err_chirho) => {
                                        self.report_unify_error_chirho(&err_chirho);
                                    }
                                }
                            }

                            let gen_ty_chirho = self.generalize_chirho(&ty_chirho);
                            self.env_chirho
                                .bind_chirho(name_str_chirho, gen_ty_chirho);
                        }
                        haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                            pat_chirho,
                            rhs_chirho,
                            ..
                        } => {
                            let (s_chirho, rhs_ty_chirho) = self.infer_rhs_chirho(rhs_chirho);
                            subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&s_chirho);
                            self.bind_pat_chirho(pat_chirho, &rhs_ty_chirho);
                        }
                        haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho { .. } => {
                            // Handled by local_sigs_chirho collection above
                        }
                    }
                }

                let (body_s_chirho, body_ty_chirho) = self.infer_expr_chirho(body_chirho);
                self.env_chirho.pop_scope_chirho();

                let combined_chirho = body_s_chirho.compose_chirho(&subst_chirho);
                (combined_chirho, body_ty_chirho)
            }

            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut elem_tys_chirho = Vec::new();

                for elem_chirho in elements_chirho {
                    let (s_chirho, ty_chirho) = self.infer_expr_chirho(elem_chirho);
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&s_chirho);
                    elem_tys_chirho.push(ty_chirho);
                }

                // Apply accumulated substitution to element types
                let final_elems_chirho: Vec<TyChirho> = elem_tys_chirho
                    .iter()
                    .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
                    .collect();

                (subst_chirho, TyChirho::TupleChirho(final_elems_chirho))
            }

            ExprChirho::ListChirho {
                elements_chirho,
                span_chirho,
            } => {
                let elem_ty_chirho = self.fresh_var_chirho();
                let mut subst_chirho = SubstChirho::empty_chirho();

                for elem_chirho in elements_chirho {
                    let (s_chirho, ty_chirho) = self.infer_expr_chirho(elem_chirho);
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&s_chirho);

                    let elem_sub_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                    match unify_chirho(&elem_sub_chirho, &ty_chirho, *span_chirho) {
                        Ok(su_chirho) => {
                            subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&su_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }
                }

                let final_elem_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                (subst_chirho, TyChirho::ListChirho(Box::new(final_elem_chirho)))
            }

            ExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                span_chirho,
            } => {
                let (s1_chirho, scrut_ty_chirho) = self.infer_expr_chirho(scrutinee_chirho);
                self.apply_subst_all_chirho(&s1_chirho);

                let result_ty_chirho = self.fresh_var_chirho();
                let mut subst_chirho = s1_chirho;

                for alt_chirho in alts_chirho {
                    self.env_chirho.push_scope_chirho();

                    let pat_ty_chirho = self.fresh_var_chirho();
                    self.bind_pat_chirho(&alt_chirho.pat_chirho, &pat_ty_chirho);

                    // Unify pattern type with scrutinee
                    let scrut_sub_chirho = subst_chirho.apply_ty_chirho(&scrut_ty_chirho);
                    match unify_chirho(&pat_ty_chirho, &scrut_sub_chirho, *span_chirho) {
                        Ok(sp_chirho) => {
                            subst_chirho = sp_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&sp_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }

                    // Collect where-clause type signatures
                    let mut wb_sigs_chirho: HashMap<String, haskelujah_ast_chirho::ty_chirho::TypeChirho> = HashMap::new();
                    for wb_chirho in &alt_chirho.where_binds_chirho {
                        if let haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                            name_chirho, ty_chirho, ..
                        } = wb_chirho
                        {
                            wb_sigs_chirho.insert(
                                name_chirho.text_chirho().to_string(),
                                ty_chirho.clone(),
                            );
                        }
                    }

                    // Pre-bind where-clause function names with fresh
                    // types so recursive references resolve (letrec).
                    let mut pre_wb_tys_chirho: Vec<(String, TyChirho)> = Vec::new();
                    for wb_chirho in &alt_chirho.where_binds_chirho {
                        if let haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                            name_chirho, ..
                        } = wb_chirho
                        {
                            let fresh_ty_chirho = self.fresh_var_chirho();
                            let name_str_chirho = name_chirho.text_chirho().to_string();
                            self.env_chirho.bind_chirho(
                                name_str_chirho.clone(),
                                SchemeChirho::mono_chirho(fresh_ty_chirho.clone()),
                            );
                            pre_wb_tys_chirho.push((name_str_chirho, fresh_ty_chirho));
                        }
                    }

                    // Bind where-clause bindings before inferring RHS
                    for wb_chirho in &alt_chirho.where_binds_chirho {
                        match wb_chirho {
                            haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                                name_chirho,
                                matches_chirho: wb_matches_chirho,
                                span_chirho: wb_span_chirho,
                            } => {
                                let (ws_chirho, wt_chirho) =
                                    self.infer_matches_chirho(wb_matches_chirho, *wb_span_chirho);
                                subst_chirho = ws_chirho.compose_chirho(&subst_chirho);
                                self.apply_subst_all_chirho(&ws_chirho);

                                let name_str_chirho = name_chirho.text_chirho().to_string();
                                if let Some((_, pre_ty_chirho)) = pre_wb_tys_chirho
                                    .iter()
                                    .find(|(n_chirho, _)| n_chirho == &name_str_chirho)
                                {
                                    let pre_ty_sub_chirho =
                                        subst_chirho.apply_ty_chirho(pre_ty_chirho);
                                    if let Ok(us_chirho) = unify_chirho(
                                        &pre_ty_sub_chirho,
                                        &wt_chirho,
                                        *wb_span_chirho,
                                    ) {
                                        subst_chirho = us_chirho.compose_chirho(&subst_chirho);
                                        self.apply_subst_all_chirho(&us_chirho);
                                    }
                                }

                                // Check against where-clause type signature if present
                                if let Some(sig_ast_chirho) = wb_sigs_chirho.get(&name_str_chirho) {
                                    let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                                    let sig_ty_raw_chirho = self.instantiate_chirho(&sig_scheme_chirho, *wb_span_chirho);
                                    let sig_ty_chirho = self.reduce_type_families_in_ty_chirho(&sig_ty_raw_chirho);
                                    let inferred_sub_chirho = subst_chirho.apply_ty_chirho(&wt_chirho);
                                    let inferred_sub_chirho = self.reduce_type_families_in_ty_chirho(&inferred_sub_chirho);
                                    match unify_chirho(&inferred_sub_chirho, &sig_ty_chirho, *wb_span_chirho) {
                                        Ok(sig_s_chirho) => {
                                            subst_chirho = sig_s_chirho.compose_chirho(&subst_chirho);
                                            self.apply_subst_all_chirho(&sig_s_chirho);
                                        }
                                        Err(err_chirho) => {
                                            self.report_unify_error_chirho(&err_chirho);
                                        }
                                    }
                                }

                                let gen_chirho = self.generalize_chirho(&wt_chirho);
                                self.env_chirho
                                    .bind_chirho(name_str_chirho, gen_chirho);
                            }
                            haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                                pat_chirho: wb_pat_chirho,
                                rhs_chirho: wb_rhs_chirho,
                                ..
                            } => {
                                let (ws_chirho, rhs_ty_chirho) =
                                    self.infer_rhs_chirho(wb_rhs_chirho);
                                subst_chirho = ws_chirho.compose_chirho(&subst_chirho);
                                self.apply_subst_all_chirho(&ws_chirho);
                                self.bind_pat_chirho(wb_pat_chirho, &rhs_ty_chirho);
                            }
                            _ => {}
                        }
                    }

                    let (sr_chirho, alt_ty_chirho) = self.infer_rhs_chirho(&alt_chirho.rhs_chirho);
                    subst_chirho = sr_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&sr_chirho);

                    // Unify alt result with overall result type
                    let result_sub_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                    match unify_chirho(&result_sub_chirho, &alt_ty_chirho, *span_chirho) {
                        Ok(sa_chirho) => {
                            subst_chirho = sa_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&sa_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }

                    self.env_chirho.pop_scope_chirho();
                }

                let final_ty_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                (subst_chirho, final_ty_chirho)
            }

            ExprChirho::DoChirho {
                stmts_chirho,
                span_chirho,
            } => {
                // Monadic do-notation:
                //   do { expr }       ≡ expr                             (last stmt)
                //   do { expr; rest } ≡ expr >> do { rest }              (ExprStmt)
                //   do { x <- m; rest } ≡ m >>= \x -> do { rest }       (BindStmt)
                //   do { let binds; rest } ≡ let binds in do { rest }    (LetStmt)
                //
                // All monadic expressions in the block share the same monad
                // constructor `m_chirho`. We use a fresh type variable for it
                // and unify as we go. The bind pattern variable `x` gets the
                // unwrapped type `a` from `m a`.
                let m_chirho = self.fresh_var_chirho(); // the monad constructor
                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut last_ty_chirho = TyChirho::unit_chirho();

                self.env_chirho.push_scope_chirho();

                for (idx_chirho, stmt_chirho) in stmts_chirho.iter().enumerate() {
                    let is_last_chirho = idx_chirho == stmts_chirho.len() - 1;

                    match stmt_chirho {
                        StmtChirho::ExprChirho(expr_chirho) => {
                            let (s_chirho, ty_chirho) = self.infer_expr_chirho(expr_chirho);
                            subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&s_chirho);

                            if is_last_chirho {
                                // Last statement: its type IS the do-block type.
                                // Unify with `m_chirho a` to extract the monad.
                                let a_chirho = self.fresh_var_chirho();
                                let expected_chirho = TyChirho::AppChirho(
                                    Box::new(m_chirho.clone()),
                                    Box::new(a_chirho.clone()),
                                );
                                match unify_chirho(
                                    &subst_chirho.apply_ty_chirho(&ty_chirho),
                                    &subst_chirho.apply_ty_chirho(&expected_chirho),
                                    *span_chirho,
                                ) {
                                    Ok(su_chirho) => {
                                        subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                                        self.apply_subst_all_chirho(&su_chirho);
                                    }
                                    Err(_) => { /* allow fallthrough for simple expressions */ }
                                }
                                last_ty_chirho = ty_chirho;
                            } else {
                                // Non-last expression stmt: type should be `m_chirho _`
                                let discard_chirho = self.fresh_var_chirho();
                                let expected_chirho = TyChirho::AppChirho(
                                    Box::new(m_chirho.clone()),
                                    Box::new(discard_chirho),
                                );
                                match unify_chirho(
                                    &subst_chirho.apply_ty_chirho(&ty_chirho),
                                    &subst_chirho.apply_ty_chirho(&expected_chirho),
                                    *span_chirho,
                                ) {
                                    Ok(su_chirho) => {
                                        subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                                        self.apply_subst_all_chirho(&su_chirho);
                                    }
                                    Err(_) => {}
                                }
                                last_ty_chirho = ty_chirho;
                            }
                        }
                        StmtChirho::BindChirho {
                            pat_chirho,
                            expr_chirho,
                            ..
                        } => {
                            let (s_chirho, ty_chirho) = self.infer_expr_chirho(expr_chirho);
                            subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&s_chirho);

                            // `x <- m`: m has type `M a`, bind x :: a
                            let elem_ty_chirho = self.fresh_var_chirho();
                            let expected_chirho = TyChirho::AppChirho(
                                Box::new(m_chirho.clone()),
                                Box::new(elem_ty_chirho.clone()),
                            );
                            match unify_chirho(
                                &subst_chirho.apply_ty_chirho(&ty_chirho),
                                &subst_chirho.apply_ty_chirho(&expected_chirho),
                                *span_chirho,
                            ) {
                                Ok(su_chirho) => {
                                    subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                                    self.apply_subst_all_chirho(&su_chirho);
                                }
                                Err(_) => {}
                            }

                            // Bind the pattern variable(s) to the unwrapped type
                            let bound_ty_chirho =
                                subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                            self.bind_pat_chirho(pat_chirho, &bound_ty_chirho);
                            last_ty_chirho = TyChirho::unit_chirho();
                        }
                        StmtChirho::LetChirho {
                            binds_chirho, ..
                        } => {
                            // let in do: introduce local bindings
                            for bind_chirho in binds_chirho {
                                match bind_chirho {
                                    haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                                        name_chirho,
                                        matches_chirho,
                                        span_chirho: bind_span_chirho,
                                    } => {
                                        let (s_chirho, ty_chirho) = self
                                            .infer_matches_chirho(matches_chirho, *bind_span_chirho);
                                        subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                                        self.apply_subst_all_chirho(&s_chirho);
                                        let gen_chirho = self.generalize_chirho(&ty_chirho);
                                        self.env_chirho.bind_chirho(
                                            name_chirho.text_chirho().to_string(),
                                            gen_chirho,
                                        );
                                    }
                                    haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                                        pat_chirho,
                                        rhs_chirho,
                                        ..
                                    } => {
                                        let (s_chirho, rhs_ty_chirho) =
                                            self.infer_rhs_chirho(rhs_chirho);
                                        subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                                        self.apply_subst_all_chirho(&s_chirho);
                                        self.bind_pat_chirho(pat_chirho, &rhs_ty_chirho);
                                    }
                                    haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                                        ..
                                    } => {}
                                }
                            }
                        }
                    }
                }

                self.env_chirho.pop_scope_chirho();
                (subst_chirho, last_ty_chirho)
            }

            ExprChirho::NegChirho {
                expr_chirho,
                span_chirho,
            } => {
                let (s_chirho, expr_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                // negate :: Num a => a -> a
                // Defer a Num constraint on the expression type
                self.deferred_preds_chirho.push((
                    PredChirho::new_chirho("Num", expr_ty_chirho.clone()),
                    *span_chirho,
                ));
                (s_chirho, expr_ty_chirho)
            }

            ExprChirho::ParenChirho { inner_chirho, .. } => self.infer_expr_chirho(inner_chirho),

            ExprChirho::AnnChirho {
                expr_chirho,
                ty_chirho: ann_ty_chirho,
                span_chirho,
            } => {
                // Type annotation: infer the expression, then unify with the annotation
                let (s_chirho, inferred_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                self.apply_subst_all_chirho(&s_chirho);

                let mut var_map_chirho = HashMap::new();
                let ann_internal_chirho =
                    self.ast_type_to_ty_chirho(ann_ty_chirho, &mut var_map_chirho);

                match unify_chirho(&inferred_ty_chirho, &ann_internal_chirho, *span_chirho) {
                    Ok(su_chirho) => {
                        let combined_chirho = su_chirho.compose_chirho(&s_chirho);
                        let final_ty_chirho = combined_chirho.apply_ty_chirho(&ann_internal_chirho);
                        self.apply_subst_all_chirho(&su_chirho);
                        (combined_chirho, final_ty_chirho)
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        (s_chirho, ann_internal_chirho)
                    }
                }
            }

            // Infix application: `a op b` is treated as `(op a) b`
            ExprChirho::InfixChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                span_chirho,
            } => {
                // Look up the operator's type scheme
                let op_text_chirho = op_chirho.text_chirho();
                let op_span_chirho = op_chirho.span_chirho();
                let op_scheme_chirho = self.env_chirho.lookup_chirho(op_text_chirho).cloned();
                let (s0_chirho, op_ty_chirho) = match op_scheme_chirho {
                    Some(scheme_chirho) => {
                        let ty_chirho = self.instantiate_chirho(&scheme_chirho, op_span_chirho);
                        (SubstChirho::empty_chirho(), ty_chirho)
                    }
                    None => {
                        // Unknown operator — degrade gracefully
                        (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                    }
                };
                self.apply_subst_all_chirho(&s0_chirho);

                // Infer left operand
                let (s1_chirho, left_ty_chirho) = self.infer_expr_chirho(left_chirho);
                let op_ty_chirho = s1_chirho.apply_ty_chirho(&op_ty_chirho);
                self.apply_subst_all_chirho(&s1_chirho);

                // Unify: op :: left_ty -> (right_ty -> result_ty)
                let mid_ty_chirho = self.fresh_var_chirho();
                let expected_chirho = TyChirho::fun_chirho(left_ty_chirho, mid_ty_chirho.clone());
                let s2_chirho = match unify_chirho(&op_ty_chirho, &expected_chirho, *span_chirho) {
                    Ok(s_chirho) => {
                        self.apply_subst_all_chirho(&s_chirho);
                        s_chirho
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        SubstChirho::empty_chirho()
                    }
                };

                let mid_ty_chirho = s2_chirho.apply_ty_chirho(&mid_ty_chirho);

                // Infer right operand
                let (s3_chirho, right_ty_chirho) = self.infer_expr_chirho(right_chirho);
                let mid_ty_chirho = s3_chirho.apply_ty_chirho(&mid_ty_chirho);
                self.apply_subst_all_chirho(&s3_chirho);

                // Unify: mid_ty :: right_ty -> result_ty
                let result_ty_chirho = self.fresh_var_chirho();
                let expected2_chirho = TyChirho::fun_chirho(right_ty_chirho, result_ty_chirho.clone());
                let s4_chirho = match unify_chirho(&mid_ty_chirho, &expected2_chirho, *span_chirho) {
                    Ok(s_chirho) => {
                        self.apply_subst_all_chirho(&s_chirho);
                        s_chirho
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        SubstChirho::empty_chirho()
                    }
                };

                let combined_chirho = s4_chirho.compose_chirho(
                    &s3_chirho.compose_chirho(
                        &s2_chirho.compose_chirho(
                            &s1_chirho.compose_chirho(&s0_chirho))));
                let final_ty_chirho = combined_chirho.apply_ty_chirho(&result_ty_chirho);
                (combined_chirho, final_ty_chirho)
            }

            // Record construction: Con { f1 = e1, f2 = e2, ... }
            // Treated as constructor application: infer constructor type,
            // then unify each field value with the successive argument types.
            ExprChirho::RecordConChirho {
                con_chirho,
                fields_chirho,
                has_wildcard_chirho,
                span_chirho,
            } => {
                let con_text_chirho = con_chirho.text_chirho();
                let scheme_opt_chirho = self.env_chirho.lookup_chirho(con_text_chirho).cloned();
                match scheme_opt_chirho {
                    Some(scheme_chirho) => {
                        let con_span_chirho = con_chirho.span_chirho();
                        let mut con_ty_chirho =
                            self.instantiate_chirho(&scheme_chirho, con_span_chirho);
                        let mut combined_chirho = SubstChirho::empty_chirho();
                        // For wildcards, infer each field in constructor order
                        if *has_wildcard_chirho {
                            if let Some(all_fields_chirho) = self.con_field_names_chirho.get(con_text_chirho).cloned() {
                                for fname_chirho in &all_fields_chirho {
                                    let (s_chirho, field_ty_chirho) = if let Some(f_chirho) = fields_chirho.iter().find(|f_chirho| f_chirho.name_chirho.text_chirho() == fname_chirho) {
                                        self.infer_expr_chirho(&f_chirho.value_chirho)
                                    } else {
                                        // Wildcard-expanded: look up variable from scope
                                        let var_scheme_chirho = self.env_chirho.lookup_chirho(fname_chirho).cloned();
                                        if let Some(scheme_chirho) = var_scheme_chirho {
                                            (SubstChirho::empty_chirho(), self.instantiate_chirho(&scheme_chirho, *span_chirho))
                                        } else {
                                            self.diagnostics_chirho.push_chirho(
                                                DiagnosticChirho::error_with_code_chirho(
                                                    ErrorCodeChirho::error_chirho(200),
                                                    format!("RecordWildCards: variable `{}` not in scope", fname_chirho),
                                                    *span_chirho,
                                                ),
                                            );
                                            (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                                        }
                                    };
                                    combined_chirho = s_chirho.compose_chirho(&combined_chirho);
                                    con_ty_chirho = combined_chirho.apply_ty_chirho(&con_ty_chirho);
                                    let result_ty_chirho = self.fresh_var_chirho();
                                    let expected_chirho = TyChirho::FunChirho(
                                        Box::new(field_ty_chirho),
                                        Box::new(result_ty_chirho.clone()),
                                     MultChirho::ManyChirho,);
                                    match unify_chirho(&con_ty_chirho, &expected_chirho, *span_chirho) {
                                        Ok(s2_chirho) => {
                                            self.apply_subst_all_chirho(&s2_chirho);
                                            combined_chirho = s2_chirho.compose_chirho(&combined_chirho);
                                            con_ty_chirho =
                                                combined_chirho.apply_ty_chirho(&result_ty_chirho);
                                        }
                                        Err(err_chirho) => {
                                            self.report_unify_error_chirho(&err_chirho);
                                            return (combined_chirho, self.fresh_var_chirho());
                                        }
                                    }
                                }
                                (combined_chirho, con_ty_chirho)
                            } else {
                                // No field info — fall through to default handling
                                for field_chirho in fields_chirho {
                                    let (s_chirho, field_ty_chirho) =
                                        self.infer_expr_chirho(&field_chirho.value_chirho);
                                    combined_chirho = s_chirho.compose_chirho(&combined_chirho);
                                    con_ty_chirho = combined_chirho.apply_ty_chirho(&con_ty_chirho);
                                    let result_ty_chirho = self.fresh_var_chirho();
                                    let expected_chirho = TyChirho::FunChirho(
                                        Box::new(field_ty_chirho),
                                        Box::new(result_ty_chirho.clone()),
                                     MultChirho::ManyChirho,);
                                    match unify_chirho(&con_ty_chirho, &expected_chirho, *span_chirho) {
                                        Ok(s2_chirho) => {
                                            self.apply_subst_all_chirho(&s2_chirho);
                                            combined_chirho = s2_chirho.compose_chirho(&combined_chirho);
                                            con_ty_chirho =
                                                combined_chirho.apply_ty_chirho(&result_ty_chirho);
                                        }
                                        Err(err_chirho) => {
                                            self.report_unify_error_chirho(&err_chirho);
                                            return (combined_chirho, self.fresh_var_chirho());
                                        }
                                    }
                                }
                                (combined_chirho, con_ty_chirho)
                            }
                        } else {
                            // Non-wildcard: apply each explicit field
                            for field_chirho in fields_chirho {
                                let (s_chirho, field_ty_chirho) =
                                    self.infer_expr_chirho(&field_chirho.value_chirho);
                                combined_chirho = s_chirho.compose_chirho(&combined_chirho);
                                con_ty_chirho = combined_chirho.apply_ty_chirho(&con_ty_chirho);
                                let result_ty_chirho = self.fresh_var_chirho();
                                let expected_chirho = TyChirho::FunChirho(
                                    Box::new(field_ty_chirho),
                                    Box::new(result_ty_chirho.clone()),
                                 MultChirho::ManyChirho,);
                                match unify_chirho(&con_ty_chirho, &expected_chirho, *span_chirho) {
                                    Ok(s2_chirho) => {
                                        self.apply_subst_all_chirho(&s2_chirho);
                                        combined_chirho = s2_chirho.compose_chirho(&combined_chirho);
                                        con_ty_chirho =
                                            combined_chirho.apply_ty_chirho(&result_ty_chirho);
                                    }
                                    Err(err_chirho) => {
                                        self.report_unify_error_chirho(&err_chirho);
                                        return (combined_chirho, self.fresh_var_chirho());
                                    }
                                }
                            }
                            (combined_chirho, con_ty_chirho)
                        }
                    }
                    None => (SubstChirho::empty_chirho(), self.fresh_var_chirho()),
                }
            }

            // Record update: expr { f1 = e1, ... }
            // For now, infer the expression type and pass through
            ExprChirho::RecordUpdateChirho {
                expr_chirho,
                ..
            } => self.infer_expr_chirho(expr_chirho),

            // TypeApplications: infer the inner expression, then unify
            // the result type with the provided type argument so that the
            // type annotation constrains polymorphic instantiation.
            ExprChirho::TypeAppChirho { expr_chirho, ty_chirho, span_chirho } => {
                let (s1_chirho, inferred_chirho) = self.infer_expr_chirho(expr_chirho);
                let mut var_map_chirho = HashMap::new();
                let target_chirho = self.ast_type_to_ty_chirho(ty_chirho, &mut var_map_chirho);
                match crate::unify_chirho::unify_chirho(&inferred_chirho, &target_chirho, *span_chirho) {
                    Ok(s2_chirho) => {
                        let composed_chirho = s2_chirho.compose_chirho(&s1_chirho);
                        let result_chirho = composed_chirho.apply_ty_chirho(&inferred_chirho);
                        (composed_chirho, result_chirho)
                    }
                    Err(_) => {
                        // If unification fails (e.g. applying @Int to a
                        // monomorphic String), just keep the inferred type.
                        // The type argument served as documentation.
                        (s1_chirho, inferred_chirho)
                    }
                }
            }

            // For remaining expression forms, return a fresh variable
            _ => (SubstChirho::empty_chirho(), self.fresh_var_chirho()),
        }
    }

    // -----------------------------------------------------------------------
    // Pattern binding
    // -----------------------------------------------------------------------

    /// Bind pattern variables to the given type in the current scope.
    fn bind_pat_chirho(&mut self, pat_chirho: &PatChirho, ty_chirho: &TyChirho) {
        match pat_chirho {
            PatChirho::VarChirho(name_chirho) => {
                // For higher-rank types: if the type is ForallChirho, bind as a
                // polymorphic scheme so each use gets a fresh instantiation.
                let scheme_chirho = match ty_chirho {
                    TyChirho::ForallChirho { vars_chirho, body_chirho } => SchemeChirho {
                        vars_chirho: vars_chirho.clone(),
                        preds_chirho: vec![],
                        ty_chirho: (**body_chirho).clone(),
                    },
                    _ => SchemeChirho::mono_chirho(ty_chirho.clone()),
                };
                self.env_chirho.bind_chirho(
                    name_chirho.text_chirho().to_string(),
                    scheme_chirho,
                );
            }
            PatChirho::WildcardChirho { .. } => {
                // Wildcards don't bind anything
            }
            PatChirho::LitChirho { .. } => {
                // Literal patterns don't introduce bindings
            }
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho: inner_chirho,
                ..
            } => {
                self.env_chirho.bind_chirho(
                    name_chirho.text_chirho().to_string(),
                    SchemeChirho::mono_chirho(ty_chirho.clone()),
                );
                self.bind_pat_chirho(inner_chirho, ty_chirho);
            }
            PatChirho::TupleChirho {
                elements_chirho, ..
            } => {
                // Each element gets a fresh type variable
                let elem_tys_chirho: Vec<TyChirho> = elements_chirho
                    .iter()
                    .map(|_| self.fresh_var_chirho())
                    .collect();
                for (elem_pat_chirho, elem_ty_chirho) in
                    elements_chirho.iter().zip(elem_tys_chirho.iter())
                {
                    self.bind_pat_chirho(elem_pat_chirho, elem_ty_chirho);
                }
            }
            PatChirho::ConChirho {
                args_chirho, ..
            } => {
                for arg_chirho in args_chirho {
                    let fresh_chirho = self.fresh_var_chirho();
                    self.bind_pat_chirho(arg_chirho, &fresh_chirho);
                }
            }
            PatChirho::RecordChirho { con_chirho, fields_chirho, has_wildcard_chirho, .. } => {
                // Each record field pattern introduces a binding
                for field_chirho in fields_chirho {
                    let fresh_chirho = self.fresh_var_chirho();
                    self.bind_pat_chirho(&field_chirho.pattern_chirho, &fresh_chirho);
                }
                // RecordWildCards: bind remaining fields as variables
                if *has_wildcard_chirho {
                    let explicit_names_chirho: std::collections::HashSet<String> = fields_chirho
                        .iter()
                        .map(|f_chirho| f_chirho.name_chirho.text_chirho().to_string())
                        .collect();
                    if let Some(all_fields_chirho) = self.con_field_names_chirho.get(con_chirho.text_chirho()).cloned() {
                        for field_name_chirho in &all_fields_chirho {
                            if !explicit_names_chirho.contains(field_name_chirho) {
                                let fresh_chirho = self.fresh_var_chirho();
                                self.env_chirho.bind_chirho(
                                    field_name_chirho.clone(),
                                    SchemeChirho::mono_chirho(fresh_chirho),
                                );
                            }
                        }
                    }
                }
            }
            PatChirho::InfixConChirho {
                left_chirho,
                right_chirho,
                ..
            } => {
                // x : xs — both sides get fresh type variables
                let left_ty_chirho = self.fresh_var_chirho();
                let right_ty_chirho = self.fresh_var_chirho();
                self.bind_pat_chirho(left_chirho, &left_ty_chirho);
                self.bind_pat_chirho(right_chirho, &right_ty_chirho);
            }
            PatChirho::ListChirho {
                elements_chirho, ..
            } => {
                let elem_ty_chirho = self.fresh_var_chirho();
                for elem_pat_chirho in elements_chirho {
                    self.bind_pat_chirho(elem_pat_chirho, &elem_ty_chirho);
                }
            }
            PatChirho::NegChirho { .. } => {
                // Negated literal: no new bindings
            }
            PatChirho::ParenChirho { inner_chirho, .. } => {
                self.bind_pat_chirho(inner_chirho, ty_chirho);
            }
            PatChirho::LazyChirho { inner_chirho, .. } | PatChirho::BangChirho { inner_chirho, .. } => {
                self.bind_pat_chirho(inner_chirho, ty_chirho);
            }
            PatChirho::ViewChirho { pat_chirho: inner_pat_chirho, .. } => {
                // View pattern (expr -> pat): bind the result pattern's variables.
                // The result of applying the view expression has a fresh type.
                let result_ty_chirho = self.fresh_var_chirho();
                self.bind_pat_chirho(inner_pat_chirho, &result_ty_chirho);
            }
            PatChirho::TypeAnnotChirho { pat_chirho: inner_pat_chirho, .. } => {
                // Type-annotated pattern (x :: T): bind inner pattern with the
                // given type. The annotation is used for scoped type variables
                // but the underlying binding semantics are unchanged.
                self.bind_pat_chirho(inner_pat_chirho, ty_chirho);
            }
        }
    }

    // -----------------------------------------------------------------------
    // RHS and match arm inference
    // -----------------------------------------------------------------------

    /// Infer the type of a right-hand side.
    fn infer_rhs_chirho(&mut self, rhs_chirho: &RhsChirho) -> (SubstChirho, TyChirho) {
        match rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => self.infer_expr_chirho(expr_chirho),
            RhsChirho::GuardedChirho(guarded_chirho) => {
                // Infer the first guard's body as the type
                if let Some(first_chirho) = guarded_chirho.first() {
                    self.infer_expr_chirho(&first_chirho.body_chirho)
                } else {
                    (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                }
            }
        }
    }

    /// Infer the type of a set of match arms (function equations).
    fn infer_matches_chirho(
        &mut self,
        matches_chirho: &[MatchArmChirho],
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        if matches_chirho.is_empty() {
            return (SubstChirho::empty_chirho(), self.fresh_var_chirho());
        }

        let first_chirho = &matches_chirho[0];
        let arity_chirho = first_chirho.pats_chirho.len();

        // Create fresh vars for parameters and result
        let param_tys_chirho: Vec<TyChirho> =
            (0..arity_chirho).map(|_| self.fresh_var_chirho()).collect();
        let result_ty_chirho = self.fresh_var_chirho();
        let mut subst_chirho = SubstChirho::empty_chirho();

        for match_arm_chirho in matches_chirho {
            self.env_chirho.push_scope_chirho();

            // Bind pattern variables
            for (pat_chirho, pat_ty_chirho) in match_arm_chirho
                .pats_chirho
                .iter()
                .zip(param_tys_chirho.iter())
            {
                let pat_ty_sub_chirho = subst_chirho.apply_ty_chirho(pat_ty_chirho);
                self.bind_pat_chirho(pat_chirho, &pat_ty_sub_chirho);
            }

            // Collect where-clause type signatures
            let mut wb_sigs_chirho: HashMap<String, haskelujah_ast_chirho::ty_chirho::TypeChirho> = HashMap::new();
            for wb_chirho in &match_arm_chirho.where_binds_chirho {
                if let haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                    name_chirho, ty_chirho, ..
                } = wb_chirho
                {
                    wb_sigs_chirho.insert(
                        name_chirho.text_chirho().to_string(),
                        ty_chirho.clone(),
                    );
                }
            }

            // Pre-bind where-clause function names with fresh types
            // so that recursive references resolve (letrec semantics).
            let mut pre_wb_tys_chirho: Vec<(String, TyChirho)> = Vec::new();
            for wb_chirho in &match_arm_chirho.where_binds_chirho {
                if let haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    name_chirho, ..
                } = wb_chirho
                {
                    let fresh_ty_chirho = self.fresh_var_chirho();
                    let name_str_chirho = name_chirho.text_chirho().to_string();
                    self.env_chirho.bind_chirho(
                        name_str_chirho.clone(),
                        SchemeChirho::mono_chirho(fresh_ty_chirho.clone()),
                    );
                    pre_wb_tys_chirho.push((name_str_chirho, fresh_ty_chirho));
                }
            }

            // Bind where-clause bindings before inferring the RHS
            for wb_chirho in &match_arm_chirho.where_binds_chirho {
                match wb_chirho {
                    haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                        name_chirho,
                        matches_chirho: wb_matches_chirho,
                        span_chirho: wb_span_chirho,
                    } => {
                        let (ws_chirho, wt_chirho) =
                            self.infer_matches_chirho(wb_matches_chirho, *wb_span_chirho);
                        subst_chirho = ws_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&ws_chirho);

                        // Unify pre-bound type with inferred type
                        let name_str_chirho = name_chirho.text_chirho().to_string();
                        if let Some((_, pre_ty_chirho)) = pre_wb_tys_chirho
                            .iter()
                            .find(|(n_chirho, _)| n_chirho == &name_str_chirho)
                        {
                            let pre_ty_sub_chirho = subst_chirho.apply_ty_chirho(pre_ty_chirho);
                            if let Ok(us_chirho) = unify_chirho(
                                &pre_ty_sub_chirho,
                                &wt_chirho,
                                *wb_span_chirho,
                            ) {
                                subst_chirho = us_chirho.compose_chirho(&subst_chirho);
                                self.apply_subst_all_chirho(&us_chirho);
                            }
                        }

                        // Check against where-clause type signature if present
                        if let Some(sig_ast_chirho) = wb_sigs_chirho.get(&name_str_chirho) {
                            let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                            let sig_ty_raw_chirho = self.instantiate_chirho(&sig_scheme_chirho, *wb_span_chirho);
                            let sig_ty_chirho = self.reduce_type_families_in_ty_chirho(&sig_ty_raw_chirho);
                            let inferred_sub_chirho = subst_chirho.apply_ty_chirho(&wt_chirho);
                            let inferred_sub_chirho = self.reduce_type_families_in_ty_chirho(&inferred_sub_chirho);
                            match unify_chirho(&inferred_sub_chirho, &sig_ty_chirho, *wb_span_chirho) {
                                Ok(sig_s_chirho) => {
                                    subst_chirho = sig_s_chirho.compose_chirho(&subst_chirho);
                                    self.apply_subst_all_chirho(&sig_s_chirho);
                                }
                                Err(err_chirho) => {
                                    self.report_unify_error_chirho(&err_chirho);
                                }
                            }
                        }

                        let gen_chirho = self.generalize_chirho(&wt_chirho);
                        self.env_chirho
                            .bind_chirho(name_str_chirho, gen_chirho);
                    }
                    haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                        pat_chirho,
                        rhs_chirho: wb_rhs_chirho,
                        ..
                    } => {
                        let (ws_chirho, rhs_ty_chirho) = self.infer_rhs_chirho(wb_rhs_chirho);
                        subst_chirho = ws_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&ws_chirho);
                        self.bind_pat_chirho(pat_chirho, &rhs_ty_chirho);
                    }
                    _ => {}
                }
            }

            // Infer RHS
            let (sr_chirho, rhs_ty_chirho) = self.infer_rhs_chirho(&match_arm_chirho.rhs_chirho);
            subst_chirho = sr_chirho.compose_chirho(&subst_chirho);
            self.apply_subst_all_chirho(&sr_chirho);

            // Unify with result type
            let result_sub_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
            match unify_chirho(&result_sub_chirho, &rhs_ty_chirho, span_chirho) {
                Ok(su_chirho) => {
                    subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&su_chirho);
                }
                Err(err_chirho) => {
                    self.report_unify_error_chirho(&err_chirho);
                }
            }

            self.env_chirho.pop_scope_chirho();
        }

        // Build the function type: param1 -> param2 -> ... -> result
        let final_params_chirho: Vec<TyChirho> = param_tys_chirho
            .iter()
            .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
            .collect();
        let final_result_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
        let fun_ty_chirho = TyChirho::fun_n_chirho(final_params_chirho, final_result_chirho);

        (subst_chirho, fun_ty_chirho)
    }

    // -----------------------------------------------------------------------
    // Module-level inference
    // -----------------------------------------------------------------------

    /// Run type inference on a module. Returns the result with environment,
    /// substitution, and diagnostics.
    pub fn infer_module_chirho(&mut self, module_chirho: &ModuleChirho) -> SubstChirho {
        let mut subst_chirho = SubstChirho::empty_chirho();

        // Phase -1: Register type synonyms so they can be expanded during
        // type inference. Process in declaration order (handles chains
        // like type FilePath = String where String is already registered).
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                rhs_chirho,
                ..
            } = decl_chirho
            {
                let syn_name_chirho = name_chirho.text_chirho().to_string();
                let params_chirho: Vec<String> = type_vars_chirho
                    .iter()
                    .map(|v_chirho| v_chirho.text_chirho().to_string())
                    .collect();
                let rhs_ty_chirho =
                    ast_type_to_syn_rhs_chirho(rhs_chirho, &params_chirho);
                self.register_type_synonym_chirho(
                    syn_name_chirho,
                    params_chirho,
                    rhs_ty_chirho,
                );
            }
        }

        // Phase -0.5: Register type families and type family instances
        for decl_chirho in &module_chirho.decls_chirho {
            match decl_chirho {
                DeclChirho::TypeFamilyDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    equations_chirho,
                    ..
                } => {
                    let family_name_chirho = name_chirho.text_chirho().to_string();
                    let param_names_chirho: Vec<String> = type_vars_chirho
                        .iter()
                        .map(|v_chirho| v_chirho.text_chirho().to_string())
                        .collect();
                    let eqs_chirho: Vec<(Vec<TyChirho>, TyChirho)> = equations_chirho
                        .iter()
                        .map(|eq_chirho| {
                            let lhs_chirho: Vec<TyChirho> = eq_chirho
                                .lhs_types_chirho
                                .iter()
                                .map(|t_chirho| ast_type_to_syn_rhs_chirho(t_chirho, &param_names_chirho))
                                .collect();
                            let rhs_chirho =
                                ast_type_to_syn_rhs_chirho(&eq_chirho.rhs_chirho, &param_names_chirho);
                            (lhs_chirho, rhs_chirho)
                        })
                        .collect();
                    self.register_type_family_chirho(family_name_chirho, eqs_chirho);
                }
                DeclChirho::TypeFamilyInstanceDeclChirho {
                    family_name_chirho,
                    lhs_types_chirho,
                    rhs_chirho,
                    ..
                } => {
                    let fname_chirho = family_name_chirho.text_chirho().to_string();
                    let lhs_chirho: Vec<TyChirho> = lhs_types_chirho
                        .iter()
                        .map(|t_chirho| ast_type_to_syn_rhs_chirho(t_chirho, &[]))
                        .collect();
                    let rhs_ty_chirho = ast_type_to_syn_rhs_chirho(rhs_chirho, &[]);
                    self.register_type_family_instance_chirho(fname_chirho, lhs_chirho, rhs_ty_chirho);
                }
                _ => {}
            }
        }

        // Phase 0: Collect type signatures for later checking
        let mut type_sigs_chirho: HashMap<String, TypeChirho> = HashMap::new();
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } = decl_chirho
            {
                type_sigs_chirho.insert(
                    name_chirho.text_chirho().to_string(),
                    ty_chirho.clone(),
                );
            }
        }

        // Phase 1a: Process class declarations (before data/instance so methods
        // are available in the environment)
        for decl_chirho in &module_chirho.decls_chirho {
            self.process_class_decl_chirho(decl_chirho);
        }

        // Phase 1b: Process instance declarations
        for decl_chirho in &module_chirho.decls_chirho {
            self.process_instance_decl_chirho(decl_chirho);
        }

        // Phase 2: Bind data and newtype constructor types
        for decl_chirho in &module_chirho.decls_chirho {
            match decl_chirho {
                DeclChirho::DataDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    constructors_chirho,
                    ..
                } => {
                    // Build the fully-applied result type: T a b c ...
                    let base_ty_chirho = TyChirho::ConChirho(name_chirho.text_chirho().to_string());
                    let mut tv_map_chirho: HashMap<String, TyVarChirho> = HashMap::new();
                    let tv_vars_chirho: Vec<TyVarChirho> = type_vars_chirho
                        .iter()
                        .map(|tv_chirho| {
                            let v_chirho = TyVarChirho(self.next_var_chirho);
                            self.next_var_chirho += 1;
                            tv_map_chirho.insert(tv_chirho.text_chirho().to_string(), v_chirho);
                            v_chirho
                        })
                        .collect();
                    let result_ty_chirho = tv_vars_chirho.iter().fold(
                        base_ty_chirho,
                        |acc_chirho, tv_chirho| {
                            TyChirho::AppChirho(
                                Box::new(acc_chirho),
                                Box::new(TyChirho::VarChirho(*tv_chirho)),
                            )
                        },
                    );
                    for con_chirho in constructors_chirho {
                        match con_chirho {
                            haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                                name_chirho,
                                fields_chirho,
                                ..
                            } => {
                                let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                    .iter()
                                    .map(|(_s_chirho, ty_chirho)| {
                                        self.ast_type_to_ty_chirho(ty_chirho, &mut tv_map_chirho)
                                    })
                                    .collect();
                                let con_ty_chirho = TyChirho::fun_n_chirho(
                                    field_tys_chirho,
                                    result_ty_chirho.clone(),
                                );
                                let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                                self.env_chirho.bind_chirho(
                                    name_chirho.text_chirho().to_string(),
                                    gen_scheme_chirho,
                                );
                            }
                            haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                                name_chirho,
                                fields_chirho,
                                ..
                            } => {
                                let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                    .iter()
                                    .flat_map(|fd_chirho| {
                                        let ty_chirho = self.ast_type_to_ty_chirho(
                                            &fd_chirho.ty_chirho,
                                            &mut tv_map_chirho,
                                        );
                                        std::iter::repeat(ty_chirho)
                                            .take(fd_chirho.names_chirho.len())
                                    })
                                    .collect();
                                let con_ty_chirho = TyChirho::fun_n_chirho(
                                    field_tys_chirho.clone(),
                                    result_ty_chirho.clone(),
                                );
                                let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                                self.env_chirho.bind_chirho(
                                    name_chirho.text_chirho().to_string(),
                                    gen_scheme_chirho,
                                );
                                // Store field names for RecordWildCards expansion
                                let field_names_chirho: Vec<String> = fields_chirho
                                    .iter()
                                    .flat_map(|fd_chirho| {
                                        fd_chirho.names_chirho.iter().map(|n_chirho| n_chirho.text_chirho().to_string())
                                    })
                                    .collect();
                                self.con_field_names_chirho.insert(
                                    name_chirho.text_chirho().to_string(),
                                    field_names_chirho,
                                );
                                // Bind field accessor functions: fieldName :: T -> FieldType
                                for (i_chirho, fd_chirho) in fields_chirho.iter().enumerate() {
                                    for fname_chirho in &fd_chirho.names_chirho {
                                        let accessor_ty_chirho = TyChirho::FunChirho(
                                            Box::new(result_ty_chirho.clone()),
                                            Box::new(field_tys_chirho[i_chirho].clone()),
                                         MultChirho::ManyChirho,);
                                        let accessor_scheme_chirho =
                                            self.generalize_chirho(&accessor_ty_chirho);
                                        self.env_chirho.bind_chirho(
                                            fname_chirho.text_chirho().to_string(),
                                            accessor_scheme_chirho,
                                        );
                                    }
                                }
                            }
                            haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                                name_chirho,
                                ty_chirho,
                                ..
                            } => {
                                // Convert the full GADT type signature to TyChirho.
                                // The return type comes from the signature itself,
                                // NOT from the auto-constructed `T a b c`.
                                let con_ty_chirho = self.ast_type_to_ty_chirho(ty_chirho, &mut tv_map_chirho);
                                let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                                self.env_chirho.bind_chirho(
                                    name_chirho.text_chirho().to_string(),
                                    gen_scheme_chirho,
                                );
                            }
                        }
                    }
                }
                DeclChirho::NewtypeDeclChirho {
                    name_chirho,
                    constructor_chirho,
                    ..
                } => {
                    let result_ty_chirho =
                        TyChirho::ConChirho(name_chirho.text_chirho().to_string());
                    match constructor_chirho {
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                            name_chirho: con_name_chirho,
                            fields_chirho,
                            ..
                        } => {
                            // Newtype constructor: exactly one field → result type
                            let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                .iter()
                                .map(|_| self.fresh_var_chirho())
                                .collect();
                            let con_ty_chirho = TyChirho::fun_n_chirho(
                                field_tys_chirho,
                                result_ty_chirho,
                            );
                            let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                            self.env_chirho.bind_chirho(
                                con_name_chirho.text_chirho().to_string(),
                                gen_scheme_chirho,
                            );
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                            name_chirho: con_name_chirho,
                            fields_chirho,
                            ..
                        } => {
                            let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                .iter()
                                .flat_map(|fd_chirho| {
                                    let ty_chirho = self.ast_type_to_ty_chirho(
                                        &fd_chirho.ty_chirho,
                                        &mut std::collections::HashMap::new(),
                                    );
                                    std::iter::repeat(ty_chirho)
                                        .take(fd_chirho.names_chirho.len())
                                })
                                .collect();
                            let con_ty_chirho = TyChirho::fun_n_chirho(
                                field_tys_chirho,
                                result_ty_chirho,
                            );
                            let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                            self.env_chirho.bind_chirho(
                                con_name_chirho.text_chirho().to_string(),
                                gen_scheme_chirho,
                            );
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                            name_chirho: con_name_chirho,
                            ty_chirho,
                            ..
                        } => {
                            // Newtype GADT: convert the full type signature.
                            let con_ty_chirho = self.ast_type_to_ty_chirho(
                                ty_chirho,
                                &mut std::collections::HashMap::new(),
                            );
                            let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                            self.env_chirho.bind_chirho(
                                con_name_chirho.text_chirho().to_string(),
                                gen_scheme_chirho,
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        // Phase 3: Infer function bindings using SCC-based binding groups
        // for proper let-polymorphism (forward references, mutual recursion).
        //
        // Phase 3a: Pre-bind ALL top-level function names with fresh type
        // variables so every function can reference every other function.
        // Phase 3b: Process groups in SCC topological order so that leaf
        // functions get generalized first, and functions that depend on
        // them see their polymorphic types.

        // 3a: Collect FunBindChirho declarations and pre-bind ALL of them
        let mut fun_names_chirho: Vec<String> = Vec::new();
        let mut fun_matches_refs_chirho: Vec<&[MatchArmChirho]> = Vec::new();
        let mut fun_spans_chirho: Vec<SpanChirho> = Vec::new();
        let mut fun_pre_tys_chirho: Vec<TyChirho> = Vec::new();

        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                span_chirho,
                ..
            } = decl_chirho
            {
                let binding_name_chirho = name_chirho.text_chirho().to_string();
                let pre_ty_chirho = self.fresh_var_chirho();
                self.env_chirho.bind_chirho(
                    binding_name_chirho.clone(),
                    SchemeChirho::mono_chirho(pre_ty_chirho.clone()),
                );
                fun_names_chirho.push(binding_name_chirho);
                fun_matches_refs_chirho.push(matches_chirho.as_slice());
                fun_spans_chirho.push(*span_chirho);
                fun_pre_tys_chirho.push(pre_ty_chirho);
            }
        }

        // Also pre-bind PatBindChirho variables
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::PatBindChirho {
                pat_chirho,
                span_chirho: _,
                ..
            } = decl_chirho
            {
                let names_chirho = crate::linearity_chirho::pat_bound_names_chirho(pat_chirho);
                for name_chirho in names_chirho {
                    let pre_ty_chirho = self.fresh_var_chirho();
                    self.env_chirho.bind_chirho(
                        name_chirho.clone(),
                        SchemeChirho::mono_chirho(pre_ty_chirho),
                    );
                }
            }
        }

        // Compute SCC binding groups (topological order: leaves first)
        let fun_decls_for_scc_chirho: Vec<(usize, &[MatchArmChirho])> =
            fun_matches_refs_chirho
                .iter()
                .enumerate()
                .map(|(i_chirho, m_chirho)| (i_chirho, *m_chirho))
                .collect();
        let groups_chirho = binding_groups_chirho(&fun_names_chirho, &fun_decls_for_scc_chirho);

        // 3b: Process each SCC group: infer → generalize
        for group_chirho in &groups_chirho {
            // Infer each function body in the group
            let mut group_inferred_chirho: Vec<(usize, String, TyChirho, SpanChirho)> = Vec::new();
            for &fi_chirho in group_chirho {
                let binding_name_chirho = &fun_names_chirho[fi_chirho];
                let pre_ty_chirho = &fun_pre_tys_chirho[fi_chirho];
                let span_chirho = fun_spans_chirho[fi_chirho];

                // ScopedTypeVariables
                let prev_scoped_chirho = self.scoped_tyvars_chirho.clone();
                if let Some(sig_ast_chirho) = type_sigs_chirho.get(binding_name_chirho) {
                    let mut sig_var_map_chirho = HashMap::new();
                    let _ = self.ast_type_to_ty_chirho(sig_ast_chirho, &mut sig_var_map_chirho);
                    if Self::has_explicit_forall_chirho(sig_ast_chirho) {
                        self.scoped_tyvars_chirho = sig_var_map_chirho;
                    }
                }

                let (s_chirho, inferred_ty_chirho) =
                    self.infer_matches_chirho(fun_matches_refs_chirho[fi_chirho], span_chirho);
                subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                self.apply_subst_all_chirho(&s_chirho);

                // Unify pre-bound type with inferred type
                let pre_sub_chirho = subst_chirho.apply_ty_chirho(pre_ty_chirho);
                match unify_chirho(&pre_sub_chirho, &inferred_ty_chirho, span_chirho) {
                    Ok(su_chirho) => {
                        subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&su_chirho);
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                    }
                }

                group_inferred_chirho.push((
                    fi_chirho,
                    binding_name_chirho.clone(),
                    inferred_ty_chirho,
                    span_chirho,
                ));

                // Restore scoped type variables
                self.scoped_tyvars_chirho = prev_scoped_chirho;
            }

            // Generalize all functions in the group together
            for (_, binding_name_chirho, inferred_ty_chirho, span_chirho) in &group_inferred_chirho
            {
                self.env_chirho.remove_chirho(binding_name_chirho);

                let final_ty_chirho = subst_chirho.apply_ty_chirho(inferred_ty_chirho);
                let gen_chirho = self.generalize_chirho(&final_ty_chirho);
                self.env_chirho
                    .bind_chirho(binding_name_chirho.clone(), gen_chirho);

                // Phase 3c: Check against type signature if one exists
                if let Some(sig_ast_chirho) = type_sigs_chirho.get(binding_name_chirho) {
                    let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                    let sig_ty_raw_chirho =
                        self.instantiate_chirho(&sig_scheme_chirho, *span_chirho);
                    let sig_ty_chirho =
                        self.reduce_type_families_in_ty_chirho(&sig_ty_raw_chirho);
                    let inferred_sub_chirho = subst_chirho.apply_ty_chirho(inferred_ty_chirho);
                    let inferred_sub_chirho =
                        self.reduce_type_families_in_ty_chirho(&inferred_sub_chirho);
                    match unify_chirho(&inferred_sub_chirho, &sig_ty_chirho, *span_chirho) {
                        Ok(sig_s_chirho) => {
                            subst_chirho = sig_s_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&sig_s_chirho);
                        }
                        Err(_err_chirho) => {
                            self.diagnostics_chirho.push_chirho(
                                DiagnosticChirho::error_with_code_chirho(
                                    ErrorCodeChirho::error_chirho(SIGNATURE_MISMATCH_CODE_CHIRHO),
                                    format!(
                                        "type signature mismatch for `{binding_name_chirho}`: \
                                         inferred `{inferred_sub_chirho}`, \
                                         declared `{sig_ty_chirho}`"
                                    ),
                                    *span_chirho,
                                ),
                            );
                        }
                    }
                }
            }
        }

        // Phase 3d: Handle top-level pattern bindings
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::PatBindChirho {
                pat_chirho,
                rhs_chirho,
                span_chirho: _,
            } = decl_chirho
            {
                let rhs_expr_chirho = match rhs_chirho {
                    RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
                    RhsChirho::GuardedChirho(arms_chirho) => {
                        if let Some(ge_chirho) = arms_chirho.first() {
                            &ge_chirho.body_chirho
                        } else {
                            continue;
                        }
                    }
                };
                let (s1_chirho, rhs_ty_chirho) = self.infer_expr_chirho(rhs_expr_chirho);
                subst_chirho = s1_chirho.compose_chirho(&subst_chirho);
                self.apply_subst_all_chirho(&s1_chirho);

                self.bind_pat_chirho(pat_chirho, &rhs_ty_chirho);

                let names_chirho = crate::linearity_chirho::pat_bound_names_chirho(pat_chirho);
                for name_chirho in &names_chirho {
                    if let Some(scheme_chirho) = self.env_chirho.lookup_chirho(name_chirho) {
                        let resolved_chirho =
                            subst_chirho.apply_ty_chirho(&scheme_chirho.ty_chirho);
                        let gen_chirho = self.generalize_chirho(&resolved_chirho);
                        self.env_chirho.bind_chirho(name_chirho.clone(), gen_chirho);
                    }
                }
            }
        }

        subst_chirho
    }

    /// Check remaining deferred predicates. Those that the class environment
    /// can fully entail are discharged; those that can't produce diagnostics.
    ///
    /// Implements Haskell-style numeric defaulting: ambiguous type variables
    /// constrained only by defaultable classes are resolved to `Int` (for
    /// Num/Integral/etc.) or `Double` (for Fractional/Floating/etc.) before
    /// the final constraint check.
    fn check_deferred_preds_chirho(&mut self, final_subst_chirho: &SubstChirho) {
        let preds_chirho: Vec<(PredChirho, SpanChirho)> =
            self.deferred_preds_chirho.drain(..).collect();

        let defaultable_classes_chirho: &[&str] = &[
            "Num", "Integral", "Enum", "Bounded", "Eq", "Ord", "Show", "Read",
            "Real", "Fractional", "Floating", "RealFrac", "RealFloat",
        ];
        let fractional_classes_chirho: &[&str] = &[
            "Fractional", "Floating", "RealFrac", "RealFloat",
        ];

        // Phase 1: resolve all predicates and collect constraints per type variable
        let mut resolved_preds_chirho: Vec<(PredChirho, SpanChirho)> = Vec::new();
        let mut var_classes_chirho: std::collections::HashMap<TyVarChirho, Vec<String>> =
            std::collections::HashMap::new();

        for (pred_chirho, span_chirho) in &preds_chirho {
            let resolved_ty_chirho =
                final_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
            let resolved_extra_tys_chirho: Vec<TyChirho> = pred_chirho
                .extra_tys_chirho
                .iter()
                .map(|t_chirho| final_subst_chirho.apply_ty_chirho(t_chirho))
                .collect();
            // Expand type synonyms in predicate type (e.g. String → [Char])
            let resolved_ty_chirho = self.expand_type_synonyms_chirho(&resolved_ty_chirho);
            let resolved_extra_tys_chirho: Vec<TyChirho> = resolved_extra_tys_chirho
                .into_iter()
                .map(|t_chirho| self.expand_type_synonyms_chirho(&t_chirho))
                .collect();
            let mut resolved_pred_chirho = PredChirho {
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: resolved_ty_chirho,
                extra_tys_chirho: resolved_extra_tys_chirho,
            };

            // Apply functional dependency improvement
            let improvement_chirho = self
                .class_env_chirho
                .fundep_improve_chirho(&resolved_pred_chirho);
            if !improvement_chirho.is_empty_chirho() {
                resolved_pred_chirho.ty_chirho =
                    improvement_chirho.apply_ty_chirho(&resolved_pred_chirho.ty_chirho);
                resolved_pred_chirho.extra_tys_chirho = resolved_pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| improvement_chirho.apply_ty_chirho(t_chirho))
                    .collect();
            }

            // Collect bare type-variable constraints for defaulting
            if let TyChirho::VarChirho(v_chirho) = &resolved_pred_chirho.ty_chirho {
                var_classes_chirho
                    .entry(*v_chirho)
                    .or_default()
                    .push(resolved_pred_chirho.class_name_chirho.clone());
            }

            resolved_preds_chirho.push((resolved_pred_chirho, *span_chirho));
        }

        // Phase 2: compute defaults for ambiguous type variables
        let mut default_subst_chirho = SubstChirho::empty_chirho();
        for (var_chirho, classes_chirho) in &var_classes_chirho {
            // All constraints on this var must be defaultable classes
            let all_defaultable_chirho = classes_chirho.iter().all(|c_chirho| {
                defaultable_classes_chirho.contains(&c_chirho.as_str())
            });
            if !all_defaultable_chirho {
                continue;
            }
            // If any constraint is a Fractional-group class, default to Double;
            // otherwise default to Int.
            let needs_double_chirho = classes_chirho.iter().any(|c_chirho| {
                fractional_classes_chirho.contains(&c_chirho.as_str())
            });
            let default_ty_chirho = if needs_double_chirho {
                TyChirho::double_chirho()
            } else {
                TyChirho::int_chirho()
            };
            default_subst_chirho.insert_chirho(*var_chirho, default_ty_chirho);
        }

        // Phase 3: apply defaults and check all predicates
        for (pred_chirho, span_chirho) in resolved_preds_chirho {
            let defaulted_pred_chirho = PredChirho {
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: default_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                extra_tys_chirho: pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| default_subst_chirho.apply_ty_chirho(t_chirho))
                    .collect(),
            };

            // If still a bare type variable after defaulting, skip (truly ambiguous
            // but no concrete check is possible).
            if matches!(defaulted_pred_chirho.ty_chirho, TyChirho::VarChirho(_)) {
                continue;
            }

            if !self.class_env_chirho.entails_chirho(&defaulted_pred_chirho) {
                self.diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                        format!(
                            "no instance for `{}`",
                            defaulted_pred_chirho
                        ),
                        span_chirho,
                    ),
                );
            }
        }

    }

    /// Consume the context and return the final result.
    pub fn finish_chirho(self) -> InferResultChirho {
        InferResultChirho {
            subst_chirho: SubstChirho::empty_chirho(),
            env_chirho: self.env_chirho,
            class_env_chirho: self.class_env_chirho,
            diagnostics_chirho: self.diagnostics_chirho,
        }
    }
}

// ---------------------------------------------------------------------------
// Type synonym helpers
// ---------------------------------------------------------------------------

/// Convert an AST `TypeChirho` to a `TyChirho` suitable for storing as a type
/// synonym RHS. Synonym parameters are represented as `ForallVarChirho` so
/// they can be substituted during expansion.
fn ast_type_to_syn_rhs_chirho(
    ty_chirho: &TypeChirho,
    params_chirho: &[String],
) -> TyChirho {
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho) => {
            let text_chirho = name_chirho.text_chirho().to_string();
            if params_chirho.contains(&text_chirho) {
                TyChirho::ForallVarChirho(text_chirho)
            } else {
                // Unknown type variable — treat as Con (might be a bug upstream)
                TyChirho::ConChirho(text_chirho)
            }
        }
        TypeChirho::ConChirho(name_chirho) => {
            TyChirho::ConChirho(name_chirho.text_chirho().to_string())
        }
        TypeChirho::ListChirho { element_chirho, .. } => TyChirho::ListChirho(
            Box::new(ast_type_to_syn_rhs_chirho(element_chirho, params_chirho)),
        ),
        TypeChirho::FunChirho {
            arg_chirho,
            mult_chirho,
            result_chirho,
            ..
        } => {
            let m_chirho = match mult_chirho {
                Some(haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::OneChirho) => MultChirho::OneChirho,
                _ => MultChirho::ManyChirho,
            };
            TyChirho::FunChirho(
                Box::new(ast_type_to_syn_rhs_chirho(arg_chirho, params_chirho)),
                Box::new(ast_type_to_syn_rhs_chirho(result_chirho, params_chirho)),
                m_chirho,
            )
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => TyChirho::TupleChirho(
            elements_chirho
                .iter()
                .map(|e_chirho| ast_type_to_syn_rhs_chirho(e_chirho, params_chirho))
                .collect(),
        ),
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => TyChirho::AppChirho(
            Box::new(ast_type_to_syn_rhs_chirho(fun_chirho, params_chirho)),
            Box::new(ast_type_to_syn_rhs_chirho(arg_chirho, params_chirho)),
        ),
        TypeChirho::ParenChirho {
            inner_chirho, ..
        } => ast_type_to_syn_rhs_chirho(inner_chirho, params_chirho),
        TypeChirho::QualChirho {
            body_chirho, ..
        } => ast_type_to_syn_rhs_chirho(body_chirho, params_chirho),
        TypeChirho::ForallChirho {
            body_chirho, ..
        } => ast_type_to_syn_rhs_chirho(body_chirho, params_chirho),
        TypeChirho::PromotedConChirho { name_chirho, .. } => {
            TyChirho::ConChirho(format!("'{}", name_chirho.text_chirho()))
        }
        TypeChirho::PromotedListChirho { elements_chirho, .. } => {
            let nil_chirho = TyChirho::ConChirho("'[]".to_string());
            elements_chirho.iter().rev().fold(nil_chirho, |acc_chirho, elem_chirho| {
                let elem_ty_chirho = ast_type_to_syn_rhs_chirho(elem_chirho, params_chirho);
                let cons_chirho = TyChirho::ConChirho("':".to_string());
                TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(cons_chirho),
                        Box::new(elem_ty_chirho),
                    )),
                    Box::new(acc_chirho),
                )
            })
        }
        // PartialTypeSignatures: wildcard in a type synonym RHS is a fresh
        // anonymous variable (synthesised as "_wildcard_chirho").
        TypeChirho::WildcardChirho { .. } => {
            TyChirho::ConChirho("_wildcard_chirho".to_string())
        }
    }
}

/// Collect variable references from an expression (shallow walk).
fn collect_expr_refs_chirho(
    expr_chirho: &haskelujah_ast_chirho::expr_chirho::ExprChirho,
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    use haskelujah_ast_chirho::expr_chirho::ExprChirho;
    match expr_chirho {
        ExprChirho::VarChirho(name_chirho) => {
            refs_chirho.insert(name_chirho.text_chirho().to_string());
        }
        ExprChirho::AppChirho { fun_chirho, arg_chirho, .. } => {
            collect_expr_refs_chirho(fun_chirho, refs_chirho);
            collect_expr_refs_chirho(arg_chirho, refs_chirho);
        }
        ExprChirho::LamChirho { body_chirho, .. } => {
            collect_expr_refs_chirho(body_chirho, refs_chirho);
        }
        ExprChirho::LetChirho { binds_chirho, body_chirho, .. } => {
            for bind_chirho in binds_chirho {
                collect_local_bind_refs_chirho(bind_chirho, refs_chirho);
            }
            collect_expr_refs_chirho(body_chirho, refs_chirho);
        }
        ExprChirho::IfChirho { cond_chirho, then_chirho, else_chirho, .. } => {
            collect_expr_refs_chirho(cond_chirho, refs_chirho);
            collect_expr_refs_chirho(then_chirho, refs_chirho);
            collect_expr_refs_chirho(else_chirho, refs_chirho);
        }
        ExprChirho::CaseChirho { scrutinee_chirho, alts_chirho, .. } => {
            collect_expr_refs_chirho(scrutinee_chirho, refs_chirho);
            for alt_chirho in alts_chirho {
                collect_rhs_refs_chirho(&alt_chirho.rhs_chirho, refs_chirho);
                for wb_chirho in &alt_chirho.where_binds_chirho {
                    collect_local_bind_refs_chirho(wb_chirho, refs_chirho);
                }
            }
        }
        ExprChirho::DoChirho { stmts_chirho, .. } => {
            for stmt_chirho in stmts_chirho {
                use haskelujah_ast_chirho::expr_chirho::StmtChirho;
                match stmt_chirho {
                    StmtChirho::ExprChirho(e_chirho) => collect_expr_refs_chirho(e_chirho, refs_chirho),
                    StmtChirho::BindChirho { expr_chirho, .. } => collect_expr_refs_chirho(expr_chirho, refs_chirho),
                    StmtChirho::LetChirho { binds_chirho, .. } => {
                        for b_chirho in binds_chirho {
                            collect_local_bind_refs_chirho(b_chirho, refs_chirho);
                        }
                    }
                }
            }
        }
        ExprChirho::InfixChirho { left_chirho, op_chirho, right_chirho, .. } => {
            collect_expr_refs_chirho(left_chirho, refs_chirho);
            refs_chirho.insert(op_chirho.text_chirho().to_string());
            collect_expr_refs_chirho(right_chirho, refs_chirho);
        }
        ExprChirho::NegChirho { expr_chirho, .. } => collect_expr_refs_chirho(expr_chirho, refs_chirho),
        ExprChirho::TupleChirho { elements_chirho, .. } => {
            for e_chirho in elements_chirho {
                collect_expr_refs_chirho(e_chirho, refs_chirho);
            }
        }
        ExprChirho::ListChirho { elements_chirho, .. } => {
            for e_chirho in elements_chirho {
                collect_expr_refs_chirho(e_chirho, refs_chirho);
            }
        }
        ExprChirho::TypeAppChirho { expr_chirho, .. } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho);
        }
        ExprChirho::ArithSeqChirho { from_chirho, then_chirho, to_chirho, .. } => {
            collect_expr_refs_chirho(from_chirho, refs_chirho);
            if let Some(t_chirho) = then_chirho { collect_expr_refs_chirho(t_chirho, refs_chirho); }
            if let Some(t_chirho) = to_chirho { collect_expr_refs_chirho(t_chirho, refs_chirho); }
        }
        ExprChirho::ListCompChirho { body_chirho, quals_chirho, .. } => {
            collect_expr_refs_chirho(body_chirho, refs_chirho);
            for q_chirho in quals_chirho {
                use haskelujah_ast_chirho::expr_chirho::StmtChirho;
                match q_chirho {
                    StmtChirho::ExprChirho(e_chirho) => collect_expr_refs_chirho(e_chirho, refs_chirho),
                    StmtChirho::BindChirho { expr_chirho, .. } => collect_expr_refs_chirho(expr_chirho, refs_chirho),
                    StmtChirho::LetChirho { binds_chirho, .. } => {
                        for b_chirho in binds_chirho { collect_local_bind_refs_chirho(b_chirho, refs_chirho); }
                    }
                }
            }
        }
        ExprChirho::LeftSectionChirho { op_chirho, arg_chirho, .. } => {
            refs_chirho.insert(op_chirho.text_chirho().to_string());
            collect_expr_refs_chirho(arg_chirho, refs_chirho);
        }
        ExprChirho::RightSectionChirho { arg_chirho, op_chirho, .. } => {
            collect_expr_refs_chirho(arg_chirho, refs_chirho);
            refs_chirho.insert(op_chirho.text_chirho().to_string());
        }
        ExprChirho::AnnChirho { expr_chirho, .. }
        | ExprChirho::ParenChirho { inner_chirho: expr_chirho, .. }
        | ExprChirho::SpliceChirho { expr_chirho, .. }
        | ExprChirho::TypedSpliceChirho { expr_chirho, .. }
        | ExprChirho::QuoteExprChirho { expr_chirho, .. } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho);
        }
        ExprChirho::RecordConChirho { fields_chirho, .. } => {
            for f_chirho in fields_chirho {
                collect_expr_refs_chirho(&f_chirho.value_chirho, refs_chirho);
            }
        }
        ExprChirho::RecordUpdateChirho { expr_chirho, fields_chirho, .. } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho);
            for f_chirho in fields_chirho {
                collect_expr_refs_chirho(&f_chirho.value_chirho, refs_chirho);
            }
        }
        // Literals, constructors, quote decls/types/pats have no variable refs
        _ => {}
    }
}

fn collect_local_bind_refs_chirho(
    bind_chirho: &haskelujah_ast_chirho::expr_chirho::LocalBindChirho,
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    match bind_chirho {
        haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
            matches_chirho, ..
        } => {
            for arm_chirho in matches_chirho {
                collect_rhs_refs_chirho(&arm_chirho.rhs_chirho, refs_chirho);
            }
        }
        haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
            rhs_chirho, ..
        } => {
            collect_rhs_refs_chirho(rhs_chirho, refs_chirho);
        }
        _ => {}
    }
}

fn collect_rhs_refs_chirho(
    rhs_chirho: &haskelujah_ast_chirho::expr_chirho::RhsChirho,
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    use haskelujah_ast_chirho::expr_chirho::RhsChirho;
    match rhs_chirho {
        RhsChirho::UnguardedChirho(expr_chirho) => collect_expr_refs_chirho(expr_chirho, refs_chirho),
        RhsChirho::GuardedChirho(arms_chirho) => {
            for ge_chirho in arms_chirho {
                collect_expr_refs_chirho(&ge_chirho.guard_chirho, refs_chirho);
                collect_expr_refs_chirho(&ge_chirho.body_chirho, refs_chirho);
            }
        }
    }
}

/// Compute binding groups for top-level functions using SCC analysis.
/// Returns groups in topological order (leaf dependencies first).
fn binding_groups_chirho(
    fun_names_chirho: &[String],
    fun_decls_chirho: &[(usize, &[haskelujah_ast_chirho::expr_chirho::MatchArmChirho])],
) -> Vec<Vec<usize>> {
    let name_set_chirho: std::collections::HashSet<&str> = fun_names_chirho.iter().map(|s| s.as_str()).collect();
    let name_to_idx_chirho: std::collections::HashMap<&str, usize> = fun_names_chirho.iter().enumerate().map(|(i, s)| (s.as_str(), i)).collect();

    // Build adjacency list
    let n_chirho = fun_names_chirho.len();
    let mut adj_chirho: Vec<Vec<usize>> = vec![Vec::new(); n_chirho];
    for (i_chirho, (_, matches_chirho)) in fun_decls_chirho.iter().enumerate() {
        let mut refs_chirho = std::collections::HashSet::new();
        for arm_chirho in *matches_chirho {
            collect_rhs_refs_chirho(&arm_chirho.rhs_chirho, &mut refs_chirho);
            for wb_chirho in &arm_chirho.where_binds_chirho {
                collect_local_bind_refs_chirho(wb_chirho, &mut refs_chirho);
            }
        }
        for ref_name_chirho in &refs_chirho {
            if name_set_chirho.contains(ref_name_chirho.as_str()) && ref_name_chirho != &fun_names_chirho[i_chirho] {
                if let Some(&j_chirho) = name_to_idx_chirho.get(ref_name_chirho.as_str()) {
                    adj_chirho[i_chirho].push(j_chirho);
                }
            }
        }
    }

    // Tarjan's SCC
    let mut index_chirho = 0usize;
    let mut stack_chirho: Vec<usize> = Vec::new();
    let mut on_stack_chirho = vec![false; n_chirho];
    let mut indices_chirho: Vec<Option<usize>> = vec![None; n_chirho];
    let mut lowlinks_chirho = vec![0usize; n_chirho];
    let mut sccs_chirho: Vec<Vec<usize>> = Vec::new();

    fn strongconnect_chirho(
        v_chirho: usize,
        adj_chirho: &[Vec<usize>],
        index_chirho: &mut usize,
        stack_chirho: &mut Vec<usize>,
        on_stack_chirho: &mut [bool],
        indices_chirho: &mut [Option<usize>],
        lowlinks_chirho: &mut [usize],
        sccs_chirho: &mut Vec<Vec<usize>>,
    ) {
        indices_chirho[v_chirho] = Some(*index_chirho);
        lowlinks_chirho[v_chirho] = *index_chirho;
        *index_chirho += 1;
        stack_chirho.push(v_chirho);
        on_stack_chirho[v_chirho] = true;

        for &w_chirho in &adj_chirho[v_chirho] {
            if indices_chirho[w_chirho].is_none() {
                strongconnect_chirho(
                    w_chirho, adj_chirho, index_chirho, stack_chirho,
                    on_stack_chirho, indices_chirho, lowlinks_chirho, sccs_chirho,
                );
                lowlinks_chirho[v_chirho] = lowlinks_chirho[v_chirho].min(lowlinks_chirho[w_chirho]);
            } else if on_stack_chirho[w_chirho] {
                lowlinks_chirho[v_chirho] = lowlinks_chirho[v_chirho].min(indices_chirho[w_chirho].unwrap());
            }
        }

        if lowlinks_chirho[v_chirho] == indices_chirho[v_chirho].unwrap() {
            let mut scc_chirho = Vec::new();
            loop {
                let w_chirho = stack_chirho.pop().unwrap();
                on_stack_chirho[w_chirho] = false;
                scc_chirho.push(w_chirho);
                if w_chirho == v_chirho { break; }
            }
            sccs_chirho.push(scc_chirho);
        }
    }

    for v_chirho in 0..n_chirho {
        if indices_chirho[v_chirho].is_none() {
            strongconnect_chirho(
                v_chirho, &adj_chirho, &mut index_chirho, &mut stack_chirho,
                &mut on_stack_chirho, &mut indices_chirho, &mut lowlinks_chirho, &mut sccs_chirho,
            );
        }
    }

    // Tarjan naturally produces SCCs with dependencies first (leaves before
    // dependents), which is the order we need for type inference.
    sccs_chirho
}

/// Collect the spine of a left-nested `AppChirho` into (head, [args]).
fn collect_app_spine_chirho(ty_chirho: &TyChirho) -> (TyChirho, Vec<TyChirho>) {
    let mut head_chirho = ty_chirho.clone();
    let mut args_chirho = Vec::new();
    loop {
        match head_chirho {
            TyChirho::AppChirho(f_chirho, a_chirho) => {
                args_chirho.push(*a_chirho);
                head_chirho = *f_chirho;
            }
            _ => break,
        }
    }
    args_chirho.reverse();
    (head_chirho, args_chirho)
}

/// Substitute a named type variable in a `TyChirho`. Used for expanding
/// parameterised type synonyms where the RHS contains named variables
/// (e.g. `type Pair a = (a, a)` — substitute `a` for the applied type).
fn subst_named_var_chirho(
    ty_chirho: &TyChirho,
    name_chirho: &str,
    replacement_chirho: &TyChirho,
) -> TyChirho {
    match ty_chirho {
        TyChirho::ConChirho(n_chirho) if n_chirho == name_chirho => replacement_chirho.clone(),
        TyChirho::ForallVarChirho(n_chirho) if n_chirho == name_chirho => {
            replacement_chirho.clone()
        }
        TyChirho::AppChirho(f_chirho, a_chirho) => TyChirho::AppChirho(
            Box::new(subst_named_var_chirho(f_chirho, name_chirho, replacement_chirho)),
            Box::new(subst_named_var_chirho(a_chirho, name_chirho, replacement_chirho)),
        ),
        TyChirho::FunChirho(a_chirho, b_chirho, _) => TyChirho::FunChirho(
            Box::new(subst_named_var_chirho(a_chirho, name_chirho, replacement_chirho)),
            Box::new(subst_named_var_chirho(b_chirho, name_chirho, replacement_chirho)),
         MultChirho::ManyChirho,),
        TyChirho::ListChirho(el_chirho) => TyChirho::ListChirho(
            Box::new(subst_named_var_chirho(el_chirho, name_chirho, replacement_chirho)),
        ),
        TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
            elems_chirho
                .iter()
                .map(|e_chirho| subst_named_var_chirho(e_chirho, name_chirho, replacement_chirho))
                .collect(),
        ),
        _ => ty_chirho.clone(),
    }
}

// ---------------------------------------------------------------------------
// Built-in type environment
// ---------------------------------------------------------------------------

/// Seed the type environment with Haskell Prelude types.
fn seed_builtins_chirho(env_chirho: &mut TyEnvChirho) {
    // True :: Bool, False :: Bool
    env_chirho.bind_chirho(
        "True".to_string(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );
    env_chirho.bind_chirho(
        "False".to_string(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );

    // not :: Bool -> Bool
    env_chirho.bind_chirho(
        "not".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::bool_chirho(),
            TyChirho::bool_chirho(),
        )),
    );

    // (&&) :: Bool -> Bool -> Bool
    env_chirho.bind_chirho(
        "&&".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::bool_chirho(), TyChirho::bool_chirho()],
            TyChirho::bool_chirho(),
        )),
    );

    // (||) :: Bool -> Bool -> Bool
    env_chirho.bind_chirho(
        "||".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::bool_chirho(), TyChirho::bool_chirho()],
            TyChirho::bool_chirho(),
        )),
    );

    // (++) :: String -> String -> String  (simplified; full Haskell: [a] -> [a] -> [a])
    env_chirho.bind_chirho(
        "++".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::string_chirho(), TyChirho::string_chirho()],
            TyChirho::string_chirho(),
        )),
    );

    // Numeric operators: forall a. Num a => a -> a -> a
    let num_v_chirho = TyVarChirho(1100);
    let num_binop_chirho = SchemeChirho {
        vars_chirho: vec![num_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Num".to_string(),
            ty_chirho: TyChirho::VarChirho(num_v_chirho),
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(num_v_chirho),
                TyChirho::VarChirho(num_v_chirho),
            ],
            TyChirho::VarChirho(num_v_chirho),
        ),
    };
    env_chirho.bind_chirho("+".to_string(), num_binop_chirho.clone());
    env_chirho.bind_chirho("-".to_string(), num_binop_chirho.clone());
    env_chirho.bind_chirho("*".to_string(), num_binop_chirho);

    // Equality operators: forall a. Eq a => a -> a -> Bool
    let eq_v_chirho = TyVarChirho(1200);
    let eq_cmp_chirho = SchemeChirho {
        vars_chirho: vec![eq_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Eq".to_string(),
            ty_chirho: TyChirho::VarChirho(eq_v_chirho),
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(eq_v_chirho),
                TyChirho::VarChirho(eq_v_chirho),
            ],
            TyChirho::bool_chirho(),
        ),
    };
    env_chirho.bind_chirho("==".to_string(), eq_cmp_chirho.clone());
    env_chirho.bind_chirho("/=".to_string(), eq_cmp_chirho);

    // Ordering operators: forall a. Ord a => a -> a -> Bool
    let ord_v_chirho = TyVarChirho(1300);
    let ord_cmp_chirho = SchemeChirho {
        vars_chirho: vec![ord_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Ord".to_string(),
            ty_chirho: TyChirho::VarChirho(ord_v_chirho),
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(ord_v_chirho),
                TyChirho::VarChirho(ord_v_chirho),
            ],
            TyChirho::bool_chirho(),
        ),
    };
    env_chirho.bind_chirho(">".to_string(), ord_cmp_chirho.clone());
    env_chirho.bind_chirho("<".to_string(), ord_cmp_chirho.clone());
    env_chirho.bind_chirho(">=".to_string(), ord_cmp_chirho.clone());
    env_chirho.bind_chirho("<=".to_string(), ord_cmp_chirho);

    // compare :: forall a. Ord a => a -> a -> Ordering
    let cmp_v_chirho = TyVarChirho(1350);
    env_chirho.bind_chirho(
        "compare".to_string(),
        SchemeChirho {
            vars_chirho: vec![cmp_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Ord".to_string(),
                ty_chirho: TyChirho::VarChirho(cmp_v_chirho),
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(cmp_v_chirho),
                    TyChirho::VarChirho(cmp_v_chirho),
                ],
                TyChirho::ConChirho("Ordering".to_string()),
            ),
        },
    );

    // min, max :: forall a. Ord a => a -> a -> a
    let minmax_v_chirho = TyVarChirho(1360);
    let minmax_scheme_chirho = SchemeChirho {
        vars_chirho: vec![minmax_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Ord".to_string(),
            ty_chirho: TyChirho::VarChirho(minmax_v_chirho),
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(minmax_v_chirho),
                TyChirho::VarChirho(minmax_v_chirho),
            ],
            TyChirho::VarChirho(minmax_v_chirho),
        ),
    };
    env_chirho.bind_chirho("min".to_string(), minmax_scheme_chirho.clone());
    env_chirho.bind_chirho("max".to_string(), minmax_scheme_chirho);

    // show :: forall a. Show a => a -> String
    let show_v_chirho = TyVarChirho(1400);
    env_chirho.bind_chirho(
        "show".to_string(),
        SchemeChirho {
            vars_chirho: vec![show_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Show".to_string(),
                ty_chirho: TyChirho::VarChirho(show_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(show_v_chirho),
                TyChirho::string_chirho(),
            ),
        },
    );

    // negate :: forall a. Num a => a -> a
    let negate_v_chirho = TyVarChirho(1150);
    env_chirho.bind_chirho(
        "negate".to_string(),
        SchemeChirho {
            vars_chirho: vec![negate_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Num".to_string(),
                ty_chirho: TyChirho::VarChirho(negate_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(negate_v_chirho),
                TyChirho::VarChirho(negate_v_chirho),
            ),
        },
    );

    // fromInteger :: forall a. Num a => Int -> a
    let fi_v_chirho = TyVarChirho(1160);
    env_chirho.bind_chirho(
        "fromInteger".to_string(),
        SchemeChirho {
            vars_chirho: vec![fi_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Num".to_string(),
                ty_chirho: TyChirho::VarChirho(fi_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::int_chirho(),
                TyChirho::VarChirho(fi_v_chirho),
            ),
        },
    );

    // (/) :: forall a. Fractional a => a -> a -> a
    let div_v_chirho = TyVarChirho(1170);
    env_chirho.bind_chirho(
        "/".to_string(),
        SchemeChirho {
            vars_chirho: vec![div_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Fractional".to_string(),
                ty_chirho: TyChirho::VarChirho(div_v_chirho),
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(div_v_chirho),
                    TyChirho::VarChirho(div_v_chirho),
                ],
                TyChirho::VarChirho(div_v_chirho),
            ),
        },
    );

    // recip :: forall a. Fractional a => a -> a
    let recip_v_chirho = TyVarChirho(1180);
    env_chirho.bind_chirho(
        "recip".to_string(),
        SchemeChirho {
            vars_chirho: vec![recip_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Fractional".to_string(),
                ty_chirho: TyChirho::VarChirho(recip_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(recip_v_chirho),
                TyChirho::VarChirho(recip_v_chirho),
            ),
        },
    );

    // fromRational :: forall a. Fractional a => Rational -> a
    let fr_v_chirho = TyVarChirho(1190);
    env_chirho.bind_chirho(
        "fromRational".to_string(),
        SchemeChirho {
            vars_chirho: vec![fr_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Fractional".to_string(),
                ty_chirho: TyChirho::VarChirho(fr_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ConChirho("Rational".to_string()),
                TyChirho::VarChirho(fr_v_chirho),
            ),
        },
    );

    // read :: forall a. Read a => String -> a
    let read_v_chirho = TyVarChirho(1195);
    env_chirho.bind_chirho(
        "read".to_string(),
        SchemeChirho {
            vars_chirho: vec![read_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Read".to_string(),
                ty_chirho: TyChirho::VarChirho(read_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                TyChirho::VarChirho(read_v_chirho),
            ),
        },
    );

    // fromString :: forall a. IsString a => String -> a
    let fs_v_chirho = TyVarChirho(1197);
    env_chirho.bind_chirho(
        "fromString".to_string(),
        SchemeChirho {
            vars_chirho: vec![fs_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "IsString".to_string(),
                ty_chirho: TyChirho::VarChirho(fs_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                TyChirho::VarChirho(fs_v_chirho),
            ),
        },
    );

    // fromList :: forall a l. IsList l => [a] -> l
    let fl_v_chirho = TyVarChirho(1198);
    let fl_a_chirho = TyVarChirho(1199);
    env_chirho.bind_chirho(
        "fromList".to_string(),
        SchemeChirho {
            vars_chirho: vec![fl_v_chirho, fl_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "IsList".to_string(),
                ty_chirho: TyChirho::VarChirho(fl_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(fl_a_chirho))),
                TyChirho::VarChirho(fl_v_chirho),
            ),
        },
    );

    // toList :: forall a l. IsList l => l -> [a]
    env_chirho.bind_chirho(
        "toList".to_string(),
        SchemeChirho {
            vars_chirho: vec![fl_v_chirho, fl_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "IsList".to_string(),
                ty_chirho: TyChirho::VarChirho(fl_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(fl_v_chirho),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(fl_a_chirho))),
            ),
        },
    );

    // div :: Int -> Int -> Int  (Integral-specialized to Int)
    env_chirho.bind_chirho(
        "div".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // mod :: Int -> Int -> Int  (Integral-specialized to Int)
    env_chirho.bind_chirho(
        "mod".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // quot :: Int -> Int -> Int  (truncating division toward zero)
    env_chirho.bind_chirho(
        "quot".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // rem :: Int -> Int -> Int  (remainder of truncating division)
    env_chirho.bind_chirho(
        "rem".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // pure :: forall a. a -> a (simplified — no Applicative class yet)
    let pure_v_chirho = TyVarChirho(1500);
    env_chirho.bind_chirho(
        "pure".to_string(),
        SchemeChirho {
            vars_chirho: vec![pure_v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(pure_v_chirho),
                TyChirho::VarChirho(pure_v_chirho),
            ),
        },
    );

    // id :: forall a. a -> a
    let id_var_chirho = TyVarChirho(1000);
    env_chirho.bind_chirho(
        "id".to_string(),
        SchemeChirho {
            vars_chirho: vec![id_var_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(id_var_chirho),
                TyChirho::VarChirho(id_var_chirho),
            ),
        },
    );

    // const :: forall a b. a -> b -> a
    let const_a_chirho = TyVarChirho(1001);
    let const_b_chirho = TyVarChirho(1002);
    env_chirho.bind_chirho(
        "const".to_string(),
        SchemeChirho {
            vars_chirho: vec![const_a_chirho, const_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(const_a_chirho),
                    TyChirho::VarChirho(const_b_chirho),
                ],
                TyChirho::VarChirho(const_a_chirho),
            ),
        },
    );

    // flip :: forall a b c. (a -> b -> c) -> b -> a -> c
    let flip_a_chirho = TyVarChirho(1003);
    let flip_b_chirho = TyVarChirho(1004);
    let flip_c_chirho = TyVarChirho(1005);
    env_chirho.bind_chirho(
        "flip".to_string(),
        SchemeChirho {
            vars_chirho: vec![flip_a_chirho, flip_b_chirho, flip_c_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(flip_a_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(flip_b_chirho),
                        TyChirho::VarChirho(flip_c_chirho),
                    ),
                ),
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(flip_b_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(flip_a_chirho),
                        TyChirho::VarChirho(flip_c_chirho),
                    ),
                ),
            ),
        },
    );

    // even :: Int -> Bool
    env_chirho.bind_chirho(
        "even".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::bool_chirho(),
        )),
    );

    // odd :: Int -> Bool
    env_chirho.bind_chirho(
        "odd".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::bool_chirho(),
        )),
    );

    // max :: Int -> Int -> Int
    env_chirho.bind_chirho(
        "max".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // min :: Int -> Int -> Int
    env_chirho.bind_chirho(
        "min".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // fst :: forall a b. (a, b) -> a
    let fst_a_chirho = TyVarChirho(1003);
    let fst_b_chirho = TyVarChirho(1004);
    env_chirho.bind_chirho(
        "fst".to_string(),
        SchemeChirho {
            vars_chirho: vec![fst_a_chirho, fst_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(fst_a_chirho),
                    TyChirho::VarChirho(fst_b_chirho),
                ]),
                TyChirho::VarChirho(fst_a_chirho),
            ),
        },
    );

    // snd :: forall a b. (a, b) -> b
    let snd_a_chirho = TyVarChirho(1005);
    let snd_b_chirho = TyVarChirho(1006);
    env_chirho.bind_chirho(
        "snd".to_string(),
        SchemeChirho {
            vars_chirho: vec![snd_a_chirho, snd_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(snd_a_chirho),
                    TyChirho::VarChirho(snd_b_chirho),
                ]),
                TyChirho::VarChirho(snd_b_chirho),
            ),
        },
    );

    // curry :: forall a b c. ((a, b) -> c) -> a -> b -> c
    let curry_a_chirho = TyVarChirho(1007);
    let curry_b_chirho = TyVarChirho(1008);
    let curry_c_chirho = TyVarChirho(1009);
    env_chirho.bind_chirho(
        "curry".to_string(),
        SchemeChirho {
            vars_chirho: vec![curry_a_chirho, curry_b_chirho, curry_c_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::TupleChirho(vec![
                            TyChirho::VarChirho(curry_a_chirho),
                            TyChirho::VarChirho(curry_b_chirho),
                        ]),
                        TyChirho::VarChirho(curry_c_chirho),
                    ),
                    TyChirho::VarChirho(curry_a_chirho),
                    TyChirho::VarChirho(curry_b_chirho),
                ],
                TyChirho::VarChirho(curry_c_chirho),
            ),
        },
    );

    // uncurry :: forall a b c. (a -> b -> c) -> (a, b) -> c
    let uncurry_a_chirho = TyVarChirho(1010);
    let uncurry_b_chirho = TyVarChirho(1011);
    let uncurry_c_chirho = TyVarChirho(1012);
    env_chirho.bind_chirho(
        "uncurry".to_string(),
        SchemeChirho {
            vars_chirho: vec![uncurry_a_chirho, uncurry_b_chirho, uncurry_c_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::VarChirho(uncurry_a_chirho),
                        TyChirho::VarChirho(uncurry_b_chirho),
                    ],
                    TyChirho::VarChirho(uncurry_c_chirho),
                ),
                TyChirho::fun_chirho(
                    TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(uncurry_a_chirho),
                        TyChirho::VarChirho(uncurry_b_chirho),
                    ]),
                    TyChirho::VarChirho(uncurry_c_chirho),
                ),
            ),
        },
    );

    // take :: forall a. Int -> [a] -> [a]
    {
        let take_a_chirho = TyVarChirho(3200);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(take_a_chirho)));
        env_chirho.bind_chirho(
            "take".to_string(),
            SchemeChirho {
                vars_chirho: vec![take_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho),
                ),
            },
        );
    }

    // drop :: forall a. Int -> [a] -> [a]
    {
        let drop_a_chirho = TyVarChirho(3210);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(drop_a_chirho)));
        env_chirho.bind_chirho(
            "drop".to_string(),
            SchemeChirho {
                vars_chirho: vec![drop_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho),
                ),
            },
        );
    }

    // words :: String -> [String]
    env_chirho.bind_chirho(
        "words".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::ListChirho(Box::new(TyChirho::string_chirho())),
        )),
    );

    // unwords :: [String] -> String
    env_chirho.bind_chirho(
        "unwords".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ListChirho(Box::new(TyChirho::string_chirho())),
            TyChirho::string_chirho(),
        )),
    );

    // concat :: [String] -> String
    env_chirho.bind_chirho(
        "concat".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ListChirho(Box::new(TyChirho::string_chirho())),
            TyChirho::string_chirho(),
        )),
    );

    // intercalate :: String -> [String] -> String
    env_chirho.bind_chirho(
        "intercalate".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::string_chirho())),
                TyChirho::string_chirho(),
            ),
        )),
    );

    // toInteger :: Int -> Int
    env_chirho.bind_chirho(
        "toInteger".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::int_chirho(),
        )),
    );

    // fromIntegral :: Int -> Double
    env_chirho.bind_chirho(
        "fromIntegral".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::double_chirho(),
        )),
    );

    // ceiling :: Double -> Int
    env_chirho.bind_chirho(
        "ceiling".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::double_chirho(),
            TyChirho::int_chirho(),
        )),
    );

    // floor :: Double -> Int
    env_chirho.bind_chirho(
        "floor".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::double_chirho(),
            TyChirho::int_chirho(),
        )),
    );

    // round :: Double -> Int
    env_chirho.bind_chirho(
        "round".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::double_chirho(),
            TyChirho::int_chirho(),
        )),
    );

    // truncate :: Double -> Int
    env_chirho.bind_chirho(
        "truncate".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::double_chirho(),
            TyChirho::int_chirho(),
        )),
    );

    // isJust :: forall a. Maybe a -> Bool
    let is_just_a_chirho = TyVarChirho(1020);
    env_chirho.bind_chirho(
        "isJust".to_string(),
        SchemeChirho {
            vars_chirho: vec![is_just_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(is_just_a_chirho)),
                ),
                TyChirho::bool_chirho(),
            ),
        },
    );

    // isNothing :: forall a. Maybe a -> Bool
    let is_nothing_a_chirho = TyVarChirho(1021);
    env_chirho.bind_chirho(
        "isNothing".to_string(),
        SchemeChirho {
            vars_chirho: vec![is_nothing_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(is_nothing_a_chirho)),
                ),
                TyChirho::bool_chirho(),
            ),
        },
    );

    // fromMaybe :: forall a. a -> Maybe a -> a
    let from_maybe_a_chirho = TyVarChirho(1022);
    env_chirho.bind_chirho(
        "fromMaybe".to_string(),
        SchemeChirho {
            vars_chirho: vec![from_maybe_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(from_maybe_a_chirho),
                TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(TyChirho::VarChirho(from_maybe_a_chirho)),
                    ),
                    TyChirho::VarChirho(from_maybe_a_chirho),
                ),
            ),
        },
    );

    // maybe :: forall a b. b -> (a -> b) -> Maybe a -> b
    let maybe_a_chirho = TyVarChirho(1023);
    let maybe_b_chirho = TyVarChirho(1024);
    env_chirho.bind_chirho(
        "maybe".to_string(),
        SchemeChirho {
            vars_chirho: vec![maybe_a_chirho, maybe_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(maybe_b_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(maybe_a_chirho),
                        TyChirho::VarChirho(maybe_b_chirho),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(TyChirho::VarChirho(maybe_a_chirho)),
                    ),
                ],
                TyChirho::VarChirho(maybe_b_chirho),
            ),
        },
    );

    // maybeToList :: forall a. Maybe a -> [a]
    let mtl_a_chirho = TyVarChirho(1030);
    env_chirho.bind_chirho(
        "maybeToList".to_string(),
        SchemeChirho {
            vars_chirho: vec![mtl_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(mtl_a_chirho)),
                ),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mtl_a_chirho))),
            ),
        },
    );

    // listToMaybe :: forall a. [a] -> Maybe a
    let ltm_a_chirho = TyVarChirho(1031);
    env_chirho.bind_chirho(
        "listToMaybe".to_string(),
        SchemeChirho {
            vars_chirho: vec![ltm_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(ltm_a_chirho))),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(ltm_a_chirho)),
                ),
            ),
        },
    );

    // catMaybes :: forall a. [Maybe a] -> [a]
    let cm_a_chirho = TyVarChirho(1032);
    env_chirho.bind_chirho(
        "catMaybes".to_string(),
        SchemeChirho {
            vars_chirho: vec![cm_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(cm_a_chirho)),
                ))),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cm_a_chirho))),
            ),
        },
    );

    // mapMaybe :: forall a b. (a -> Maybe b) -> [a] -> [b]
    let mm_a_chirho = TyVarChirho(1033);
    let mm_b_chirho = TyVarChirho(1034);
    env_chirho.bind_chirho(
        "mapMaybe".to_string(),
        SchemeChirho {
            vars_chirho: vec![mm_a_chirho, mm_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(mm_a_chirho),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("Maybe".to_string())),
                            Box::new(TyChirho::VarChirho(mm_b_chirho)),
                        ),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mm_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mm_b_chirho))),
            ),
        },
    );

    // fromJust :: forall a. Maybe a -> a
    let fj_a_chirho = TyVarChirho(1035);
    env_chirho.bind_chirho(
        "fromJust".to_string(),
        SchemeChirho {
            vars_chirho: vec![fj_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(fj_a_chirho)),
                ),
                TyChirho::VarChirho(fj_a_chirho),
            ),
        },
    );

    // swap :: forall a b. (a, b) -> (b, a)
    let sw_a_chirho = TyVarChirho(1036);
    let sw_b_chirho = TyVarChirho(1037);
    env_chirho.bind_chirho(
        "swap".to_string(),
        SchemeChirho {
            vars_chirho: vec![sw_a_chirho, sw_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(sw_a_chirho),
                    TyChirho::VarChirho(sw_b_chirho),
                ]),
                TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(sw_b_chirho),
                    TyChirho::VarChirho(sw_a_chirho),
                ]),
            ),
        },
    );

    // either :: forall a b c. (a -> c) -> (b -> c) -> Either a b -> c
    let either_a_chirho = TyVarChirho(1025);
    let either_b_chirho = TyVarChirho(1026);
    let either_c_chirho = TyVarChirho(1027);
    env_chirho.bind_chirho(
        "either".to_string(),
        SchemeChirho {
            vars_chirho: vec![either_a_chirho, either_b_chirho, either_c_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(either_a_chirho),
                        TyChirho::VarChirho(either_c_chirho),
                    ),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(either_b_chirho),
                        TyChirho::VarChirho(either_c_chirho),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("Either".to_string())),
                            Box::new(TyChirho::VarChirho(either_a_chirho)),
                        )),
                        Box::new(TyChirho::VarChirho(either_b_chirho)),
                    ),
                ],
                TyChirho::VarChirho(either_c_chirho),
            ),
        },
    );

    // otherwise :: Bool  (otherwise = True)
    env_chirho.bind_chirho(
        "otherwise".to_string(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );

    // ($) :: forall a b. (a -> b) -> a -> b
    let dollar_a_chirho = TyVarChirho(2000);
    let dollar_b_chirho = TyVarChirho(2001);
    env_chirho.bind_chirho(
        "$".to_string(),
        SchemeChirho {
            vars_chirho: vec![dollar_a_chirho, dollar_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(dollar_a_chirho),
                        TyChirho::VarChirho(dollar_b_chirho),
                    ),
                    TyChirho::VarChirho(dollar_a_chirho),
                ],
                TyChirho::VarChirho(dollar_b_chirho),
            ),
        },
    );

    // ($!) :: forall a b. (a -> b) -> a -> b  (strict application)
    let bang_dollar_a_chirho = TyVarChirho(2010);
    let bang_dollar_b_chirho = TyVarChirho(2011);
    env_chirho.bind_chirho(
        "$!".to_string(),
        SchemeChirho {
            vars_chirho: vec![bang_dollar_a_chirho, bang_dollar_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(bang_dollar_a_chirho),
                        TyChirho::VarChirho(bang_dollar_b_chirho),
                    ),
                    TyChirho::VarChirho(bang_dollar_a_chirho),
                ],
                TyChirho::VarChirho(bang_dollar_b_chirho),
            ),
        },
    );

    // ($!!) :: forall a b. NFData a => (a -> b) -> a -> b
    // Deep strict application: f $!! x = deepseq x (f x)
    let dblbang_a_chirho = TyVarChirho(2090);
    let dblbang_b_chirho = TyVarChirho(2091);
    env_chirho.bind_chirho(
        "$!!".to_string(),
        SchemeChirho {
            vars_chirho: vec![dblbang_a_chirho, dblbang_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(dblbang_a_chirho),
                        TyChirho::VarChirho(dblbang_b_chirho),
                    ),
                    TyChirho::VarChirho(dblbang_a_chirho),
                ],
                TyChirho::VarChirho(dblbang_b_chirho),
            ),
        },
    );

    // (.) :: forall a b c. (b -> c) -> (a -> b) -> a -> c
    let dot_a_chirho = TyVarChirho(2100);
    let dot_b_chirho = TyVarChirho(2101);
    let dot_c_chirho = TyVarChirho(2102);
    env_chirho.bind_chirho(
        ".".to_string(),
        SchemeChirho {
            vars_chirho: vec![dot_a_chirho, dot_b_chirho, dot_c_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(dot_b_chirho),
                        TyChirho::VarChirho(dot_c_chirho),
                    ),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(dot_a_chirho),
                        TyChirho::VarChirho(dot_b_chirho),
                    ),
                ],
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(dot_a_chirho),
                    TyChirho::VarChirho(dot_c_chirho),
                ),
            ),
        },
    );

    // negate :: forall a. Num a => a -> a
    let neg_v_chirho = TyVarChirho(2200);
    env_chirho.bind_chirho(
        "negate".to_string(),
        SchemeChirho {
            vars_chirho: vec![neg_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Num".to_string(),
                ty_chirho: TyChirho::VarChirho(neg_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(neg_v_chirho),
                TyChirho::VarChirho(neg_v_chirho),
            ),
        },
    );

    // -- I/O functions (IO a is a proper type constructor) --

    // putStrLn :: String -> IO ()
    env_chirho.bind_chirho(
        "putStrLn".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // putStr :: String -> IO ()
    env_chirho.bind_chirho(
        "putStr".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // print :: forall a. Show a => a -> IO ()
    let print_v_chirho = TyVarChirho(1600);
    env_chirho.bind_chirho(
        "print".to_string(),
        SchemeChirho {
            vars_chirho: vec![print_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Show".to_string(),
                ty_chirho: TyChirho::VarChirho(print_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(print_v_chirho),
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            ),
        },
    );

    // return :: forall a. a -> IO a
    let return_v_chirho = TyVarChirho(1700);
    env_chirho.bind_chirho(
        "return".to_string(),
        SchemeChirho {
            vars_chirho: vec![return_v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(return_v_chirho),
                TyChirho::io_chirho(TyChirho::VarChirho(return_v_chirho)),
            ),
        },
    );

    // (>>=) :: forall a b. IO a -> (a -> IO b) -> IO b
    let bind_a_chirho = TyVarChirho(1800);
    let bind_b_chirho = TyVarChirho(1801);
    env_chirho.bind_chirho(
        ">>=".to_string(),
        SchemeChirho {
            vars_chirho: vec![bind_a_chirho, bind_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::io_chirho(TyChirho::VarChirho(bind_a_chirho)),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(bind_a_chirho),
                        TyChirho::io_chirho(TyChirho::VarChirho(bind_b_chirho)),
                    ),
                ],
                TyChirho::io_chirho(TyChirho::VarChirho(bind_b_chirho)),
            ),
        },
    );

    // (>>) :: forall a b. IO a -> IO b -> IO b
    let then_a_chirho = TyVarChirho(1900);
    let then_b_chirho = TyVarChirho(1901);
    env_chirho.bind_chirho(
        ">>".to_string(),
        SchemeChirho {
            vars_chirho: vec![then_a_chirho, then_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::io_chirho(TyChirho::VarChirho(then_a_chirho)),
                    TyChirho::io_chirho(TyChirho::VarChirho(then_b_chirho)),
                ],
                TyChirho::io_chirho(TyChirho::VarChirho(then_b_chirho)),
            ),
        },
    );

    // getLine :: IO String
    env_chirho.bind_chirho(
        "getLine".to_string(),
        SchemeChirho::mono_chirho(TyChirho::io_chirho(TyChirho::string_chirho())),
    );

    // getChar :: IO Char
    env_chirho.bind_chirho(
        "getChar".to_string(),
        SchemeChirho::mono_chirho(TyChirho::io_chirho(TyChirho::char_chirho())),
    );

    // getContents :: IO String — read all stdin as a single lazy String
    env_chirho.bind_chirho(
        "getContents".to_string(),
        SchemeChirho::mono_chirho(TyChirho::io_chirho(TyChirho::string_chirho())),
    );

    // readFile :: String -> IO String
    env_chirho.bind_chirho(
        "readFile".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::io_chirho(TyChirho::string_chirho()),
        )),
    );

    // writeFile :: String -> String -> IO ()
    env_chirho.bind_chirho(
        "writeFile".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::string_chirho(), TyChirho::string_chirho()],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // appendFile :: String -> String -> IO ()
    env_chirho.bind_chirho(
        "appendFile".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::string_chirho(), TyChirho::string_chirho()],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // interact :: (String -> String) -> IO ()
    env_chirho.bind_chirho(
        "interact".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::fun_chirho(TyChirho::string_chirho(), TyChirho::string_chirho()),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // newIORef :: a -> IO (IORef a)  (IORef a ≈ Int at runtime)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3200));
        env_chirho.bind_chirho(
            "newIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3200)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    a_chirho,
                    TyChirho::io_chirho(TyChirho::int_chirho()),
                ),
            },
        );
    }

    // readIORef :: IORef a -> IO a  (IORef a ≈ Int at runtime)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3201));
        env_chirho.bind_chirho(
            "readIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3201)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // writeIORef :: IORef a -> a -> IO ()
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3202));
        env_chirho.bind_chirho(
            "writeIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3202)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::int_chirho(), a_chirho],
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // modifyIORef :: IORef a -> (a -> a) -> IO ()
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3203));
        env_chirho.bind_chirho(
            "modifyIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3203)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::int_chirho(),
                        TyChirho::fun_chirho(a_chirho.clone(), a_chirho),
                    ],
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // ── Control.Monad.ST operations ──
    // newSTRef :: a -> ST s (STRef s a)  (simplified: STRef s a ≈ Int, ST s ≈ identity)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3280));
        env_chirho.bind_chirho(
            "newSTRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3280)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(a_chirho, TyChirho::int_chirho()),
            },
        );
    }

    // readSTRef :: STRef s a -> ST s a  (simplified: STRef s a ≈ Int)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3281));
        env_chirho.bind_chirho(
            "readSTRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3281)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), a_chirho),
            },
        );
    }

    // writeSTRef :: STRef s a -> a -> ST s ()  (simplified)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3282));
        env_chirho.bind_chirho(
            "writeSTRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3282)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::int_chirho(), a_chirho],
                    TyChirho::unit_chirho(),
                ),
            },
        );
    }

    // modifySTRef :: STRef s a -> (a -> a) -> ST s ()  (simplified: STRef s a ≈ Int)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3284));
        env_chirho.bind_chirho(
            "modifySTRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3284)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::int_chirho(),
                        TyChirho::fun_chirho(a_chirho.clone(), a_chirho),
                    ],
                    TyChirho::unit_chirho(),
                ),
            },
        );
    }

    // runST :: (forall s. ST s a) -> a  (simplified: runST f = f)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3283));
        env_chirho.bind_chirho(
            "runST".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3283)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho),
            },
        );
    }

    // ── STM (Software Transactional Memory) operations ──
    // newTVar :: a -> IO (TVar a)  (TVar a ≈ Int at runtime)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3290));
        env_chirho.bind_chirho(
            "newTVar".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3290)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    a_chirho,
                    TyChirho::io_chirho(TyChirho::int_chirho()),
                ),
            },
        );
    }

    // newTVarIO :: a -> IO (TVar a)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3291));
        env_chirho.bind_chirho(
            "newTVarIO".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3291)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    a_chirho,
                    TyChirho::io_chirho(TyChirho::int_chirho()),
                ),
            },
        );
    }

    // readTVar :: TVar a -> IO a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3292));
        env_chirho.bind_chirho(
            "readTVar".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3292)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // readTVarIO :: TVar a -> IO a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3293));
        env_chirho.bind_chirho(
            "readTVarIO".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3293)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // writeTVar :: TVar a -> a -> IO ()
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3294));
        env_chirho.bind_chirho(
            "writeTVar".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3294)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::int_chirho(), a_chirho],
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // atomically :: STM a -> IO a  (single-threaded: identity)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3295));
        env_chirho.bind_chirho(
            "atomically".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3295)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::io_chirho(a_chirho.clone()),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // retry :: STM a  (returns IO a in our single-threaded model)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3296));
        env_chirho.bind_chirho(
            "retry".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3296)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::io_chirho(a_chirho),
            },
        );
    }

    // orElse :: STM a -> STM a -> STM a  (simplified: IO a -> IO a -> IO a)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3297));
        env_chirho.bind_chirho(
            "orElse".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3297)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::io_chirho(a_chirho.clone()),
                        TyChirho::io_chirho(a_chirho.clone()),
                    ],
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // ── Monad Transformer operations ──

    // runStateT :: (IO a) -> s -> IO a  (returns pair at runtime, simplified type)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3400));
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3401));
        env_chirho.bind_chirho(
            "runStateT".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3400), TyVarChirho(3401)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::io_chirho(a_chirho.clone()), s_chirho],
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // evalStateT :: (IO a) -> s -> IO a
    for name_chirho in &["evalStateT", "evalState"] {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3402));
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3403));
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3402), TyVarChirho(3403)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::io_chirho(a_chirho.clone()), s_chirho],
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // execStateT :: (IO a) -> s -> IO s
    for name_chirho in &["execStateT", "execState"] {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3404));
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3405));
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3404), TyVarChirho(3405)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::io_chirho(a_chirho), s_chirho.clone()],
                    TyChirho::io_chirho(s_chirho),
                ),
            },
        );
    }

    // get :: IO s
    {
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3406));
        env_chirho.bind_chirho(
            "get".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3406)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::io_chirho(s_chirho),
            },
        );
    }

    // put :: s -> IO ()
    {
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3407));
        env_chirho.bind_chirho(
            "put".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3407)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(s_chirho, TyChirho::io_chirho(TyChirho::unit_chirho())),
            },
        );
    }

    // modify :: (s -> s) -> IO ()
    {
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3408));
        env_chirho.bind_chirho(
            "modify".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3408)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(s_chirho.clone(), s_chirho),
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // runReaderT :: (IO a) -> r -> IO a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3410));
        let r_chirho = TyChirho::VarChirho(TyVarChirho(3411));
        env_chirho.bind_chirho(
            "runReaderT".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3410), TyVarChirho(3411)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::io_chirho(a_chirho.clone()), r_chirho],
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // ask :: IO r
    {
        let r_chirho = TyChirho::VarChirho(TyVarChirho(3412));
        env_chirho.bind_chirho(
            "ask".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3412)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::io_chirho(r_chirho),
            },
        );
    }

    // local :: (r -> r) -> IO a -> IO a
    {
        let r_chirho = TyChirho::VarChirho(TyVarChirho(3413));
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3414));
        env_chirho.bind_chirho(
            "local".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3413), TyVarChirho(3414)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(r_chirho.clone(), r_chirho),
                        TyChirho::io_chirho(a_chirho.clone()),
                    ],
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // runExceptT :: IO a -> IO (Either e a)  (simplified)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3420));
        env_chirho.bind_chirho(
            "runExceptT".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3420)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::io_chirho(a_chirho.clone()),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // throwE :: e -> IO a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3421));
        let e_chirho = TyChirho::VarChirho(TyVarChirho(3422));
        env_chirho.bind_chirho(
            "throwE".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3421), TyVarChirho(3422)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(e_chirho, TyChirho::io_chirho(a_chirho)),
            },
        );
    }

    // catchE :: IO a -> (e -> IO a) -> IO a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3423));
        let e_chirho = TyChirho::VarChirho(TyVarChirho(3424));
        env_chirho.bind_chirho(
            "catchE".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3423), TyVarChirho(3424)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::io_chirho(a_chirho.clone()),
                        TyChirho::fun_chirho(e_chirho, TyChirho::io_chirho(a_chirho.clone())),
                    ],
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // runWriterT :: IO a -> IO (a, [w])
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3430));
        env_chirho.bind_chirho(
            "runWriterT".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3430)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::io_chirho(a_chirho.clone()),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // tell :: w -> IO ()
    {
        let w_chirho = TyChirho::VarChirho(TyVarChirho(3431));
        env_chirho.bind_chirho(
            "tell".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3431)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(w_chirho, TyChirho::io_chirho(TyChirho::unit_chirho())),
            },
        );
    }

    // runMaybeT :: IO a -> IO a  (simplified; wraps in Maybe at runtime)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3440));
        env_chirho.bind_chirho(
            "runMaybeT".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3440)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::io_chirho(a_chirho.clone()),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // ── Data.Map operations ──
    // mapEmpty :: Map k v  (simplified as Int)
    env_chirho.bind_chirho(
        "mapEmpty".to_string(),
        SchemeChirho::mono_chirho(TyChirho::int_chirho()),
    );

    // mapSingleton :: k -> v -> Map k v
    {
        let k_chirho = TyChirho::VarChirho(TyVarChirho(3210));
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3211));
        env_chirho.bind_chirho(
            "mapSingleton".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3210), TyVarChirho(3211)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    k_chirho,
                    TyChirho::fun_chirho(v_chirho, TyChirho::int_chirho()),
                ),
            },
        );
    }

    // mapInsert :: Ord k => k -> v -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3212);
        let v_chirho = TyVarChirho(3800);
        env_chirho.bind_chirho(
            "mapInsert".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::VarChirho(k_chirho), TyChirho::VarChirho(v_chirho), TyChirho::int_chirho()],
                    TyChirho::int_chirho(),
                ),
            },
        );
    }

    // mapLookup :: Ord k => k -> Map k v -> Maybe v
    {
        let k_chirho = TyVarChirho(3801);
        let v_chirho = TyVarChirho(3213);
        env_chirho.bind_chirho(
            "mapLookup".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(k_chirho),
                    TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::VarChirho(v_chirho)),
                ),
            },
        );
    }

    // mapSize :: Map k v -> Int
    env_chirho.bind_chirho(
        "mapSize".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::int_chirho(),
        )),
    );

    // mapMember :: Ord k => k -> Map k v -> Bool
    {
        let k_chirho = TyVarChirho(3802);
        env_chirho.bind_chirho(
            "mapMember".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(k_chirho),
                    TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::bool_chirho()),
                ),
            },
        );
    }

    // mapFromList :: Ord k => [(k, v)] -> Map k v
    {
        let k_chirho = TyVarChirho(3803);
        let v_chirho = TyVarChirho(3214);
        env_chirho.bind_chirho(
            "mapFromList".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(k_chirho),
                        TyChirho::VarChirho(v_chirho),
                    ]))),
                    TyChirho::int_chirho(),
                ),
            },
        );
    }

    // mapDelete :: Ord k => k -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3804);
        env_chirho.bind_chirho(
            "mapDelete".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::VarChirho(k_chirho), TyChirho::int_chirho()],
                    TyChirho::int_chirho(),
                ),
            },
        );
    }

    // mapToList :: Map -> [(Int, v)]
    {
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3220));
        env_chirho.bind_chirho(
            "mapToList".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3220)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::int_chirho(),
                        v_chirho,
                    ]))),
                ),
            },
        );
    }

    // mapKeys :: Map -> [Int]
    env_chirho.bind_chirho(
        "mapKeys".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
        )),
    );

    // mapElems :: Map -> [v]
    {
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3222));
        env_chirho.bind_chirho(
            "mapElems".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3222)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::ListChirho(Box::new(v_chirho)),
                ),
            },
        );
    }

    // mapNull :: Map -> Bool
    env_chirho.bind_chirho(
        "mapNull".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::bool_chirho(),
        )),
    );

    // mapMap :: (v -> w) -> Map -> Map
    {
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3224));
        let w_chirho = TyChirho::VarChirho(TyVarChirho(3225));
        env_chirho.bind_chirho(
            "mapMap".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3224), TyVarChirho(3225)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(v_chirho, w_chirho),
                        TyChirho::int_chirho(),
                    ],
                    TyChirho::int_chirho(),
                ),
            },
        );
    }

    // mapFoldlWithKey :: (b -> Int -> v -> b) -> b -> Map -> b
    {
        let b_chirho = TyChirho::VarChirho(TyVarChirho(3226));
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3227));
        env_chirho.bind_chirho(
            "mapFoldlWithKey".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3226), TyVarChirho(3227)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![b_chirho.clone(), TyChirho::int_chirho(), v_chirho],
                            b_chirho.clone(),
                        ),
                        b_chirho.clone(),
                        TyChirho::int_chirho(),
                    ],
                    b_chirho,
                ),
            },
        );
    }

    // mapFoldrWithKey :: (k -> v -> b -> b) -> b -> Map k v -> b
    {
        let b_chirho = TyChirho::VarChirho(TyVarChirho(3840));
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3841));
        env_chirho.bind_chirho(
            "mapFoldrWithKey".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3840), TyVarChirho(3841)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::int_chirho(), v_chirho, b_chirho.clone()],
                            b_chirho.clone(),
                        ),
                        b_chirho.clone(),
                        TyChirho::int_chirho(),
                    ],
                    b_chirho,
                ),
            },
        );
    }

    // mapFoldlWithKey' :: (b -> k -> v -> b) -> b -> Map k v -> b
    // Strict variant — same signature as mapFoldlWithKey in our strict runtime.
    {
        let b_chirho = TyChirho::VarChirho(TyVarChirho(3845));
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3846));
        env_chirho.bind_chirho(
            "mapFoldlWithKey'".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3845), TyVarChirho(3846)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![b_chirho.clone(), TyChirho::int_chirho(), v_chirho],
                            b_chirho.clone(),
                        ),
                        b_chirho.clone(),
                        TyChirho::int_chirho(),
                    ],
                    b_chirho,
                ),
            },
        );
    }

    // mapInsertWith :: Ord k => (v -> v -> v) -> k -> v -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3805);
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3230));
        env_chirho.bind_chirho(
            "mapInsertWith".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, TyVarChirho(3230)],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(v_chirho.clone(), TyChirho::fun_chirho(v_chirho.clone(), v_chirho.clone())),
                        TyChirho::VarChirho(k_chirho),
                        v_chirho,
                        TyChirho::int_chirho(),
                    ],
                    TyChirho::int_chirho(),
                ),
            },
        );
    }

    // mapFindWithDefault :: Ord k => v -> k -> Map k v -> v
    {
        let k_chirho = TyVarChirho(3806);
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3231));
        env_chirho.bind_chirho(
            "mapFindWithDefault".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, TyVarChirho(3231)],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![v_chirho.clone(), TyChirho::VarChirho(k_chirho), TyChirho::int_chirho()],
                    v_chirho,
                ),
            },
        );
    }

    // mapAdjust :: Ord k => (v -> v) -> k -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3807);
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3232));
        env_chirho.bind_chirho(
            "mapAdjust".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, TyVarChirho(3232)],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(v_chirho.clone(), v_chirho),
                        TyChirho::VarChirho(k_chirho),
                        TyChirho::int_chirho(),
                    ],
                    TyChirho::int_chirho(),
                ),
            },
        );
    }

    // mapUnionWith :: (v -> v -> v) -> Map -> Map -> Map
    {
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3233));
        env_chirho.bind_chirho(
            "mapUnionWith".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3233)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(v_chirho.clone(), TyChirho::fun_chirho(v_chirho.clone(), v_chirho)),
                        TyChirho::int_chirho(),
                        TyChirho::int_chirho(),
                    ],
                    TyChirho::int_chirho(),
                ),
            },
        );
    }

    // mapUnion :: Map -> Map -> Map
    env_chirho.bind_chirho(
        "mapUnion".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // mapDifference :: Map -> Map -> Map
    env_chirho.bind_chirho(
        "mapDifference".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // mapIntersectionWith :: (v -> v -> v) -> Map -> Map -> Map
    {
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3234));
        env_chirho.bind_chirho(
            "mapIntersectionWith".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3234)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(v_chirho.clone(), TyChirho::fun_chirho(v_chirho.clone(), v_chirho)),
                        TyChirho::int_chirho(),
                        TyChirho::int_chirho(),
                    ],
                    TyChirho::int_chirho(),
                ),
            },
        );
    }

    // mapFilter :: (v -> Bool) -> Map -> Map
    {
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3235));
        env_chirho.bind_chirho(
            "mapFilter".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3235)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(v_chirho, TyChirho::bool_chirho()),
                        TyChirho::int_chirho(),
                    ],
                    TyChirho::int_chirho(),
                ),
            },
        );
    }

    // setEmpty :: Set Int  (represented as Int placeholder)
    env_chirho.bind_chirho(
        "setEmpty".to_string(),
        SchemeChirho::mono_chirho(TyChirho::int_chirho()),
    );

    // setSingleton :: Int -> Set Int
    env_chirho.bind_chirho(
        "setSingleton".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho())),
    );

    // setInsert :: Int -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setInsert".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // setMember :: Int -> Set Int -> Bool
    env_chirho.bind_chirho(
        "setMember".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::bool_chirho(),
        )),
    );

    // setSize :: Set Int -> Int
    env_chirho.bind_chirho(
        "setSize".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho())),
    );

    // setToList :: Set Int -> [Int]
    env_chirho.bind_chirho(
        "setToList".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
        )),
    );

    // setFromList :: [Int] -> Set Int
    env_chirho.bind_chirho(
        "setFromList".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
            TyChirho::int_chirho(),
        )),
    );

    // setDelete :: Int -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setDelete".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // setUnion :: Set Int -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setUnion".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // setIntersection :: Set Int -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setIntersection".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // setDifference :: Set Int -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setDifference".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // setFilter :: (Int -> Bool) -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setFilter".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::bool_chirho()),
            TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
        )),
    );

    // setMap :: (Int -> Int) -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setMap".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
            TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
        )),
    );

    // setFold :: (Int -> b -> b) -> b -> Set Int -> b
    {
        let b_chirho = TyChirho::VarChirho(TyVarChirho(3230));
        env_chirho.bind_chirho(
            "setFold".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3230)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::fun_chirho(b_chirho.clone(), b_chirho.clone())),
                        b_chirho.clone(),
                        TyChirho::int_chirho(),
                    ],
                    b_chirho,
                ),
            },
        );
    }

    // setFoldr :: (Int -> b -> b) -> b -> Set Int -> b (alias for setFold)
    {
        let b_chirho = TyChirho::VarChirho(TyVarChirho(3231));
        env_chirho.bind_chirho(
            "setFoldr".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3231)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::fun_chirho(b_chirho.clone(), b_chirho.clone())),
                        b_chirho.clone(),
                        TyChirho::int_chirho(),
                    ],
                    b_chirho,
                ),
            },
        );
    }

    // when :: Bool -> IO () -> IO ()
    env_chirho.bind_chirho(
        "when".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::bool_chirho(), TyChirho::io_chirho(TyChirho::unit_chirho())],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // unless :: Bool -> IO () -> IO ()
    env_chirho.bind_chirho(
        "unless".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::bool_chirho(), TyChirho::io_chirho(TyChirho::unit_chirho())],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // mapM_ :: forall a. (a -> IO ()) -> [a] -> IO ()
    let mapm_a_chirho = TyVarChirho(3500);
    env_chirho.bind_chirho(
        "mapM_".to_string(),
        SchemeChirho {
            vars_chirho: vec![mapm_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(mapm_a_chirho),
                        TyChirho::io_chirho(TyChirho::unit_chirho()),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mapm_a_chirho))),
                ],
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            ),
        },
    );

    // forM_ :: forall a. [a] -> (a -> IO ()) -> IO ()
    let form_a_chirho = TyVarChirho(3501);
    env_chirho.bind_chirho(
        "forM_".to_string(),
        SchemeChirho {
            vars_chirho: vec![form_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(form_a_chirho))),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(form_a_chirho),
                        TyChirho::io_chirho(TyChirho::unit_chirho()),
                    ),
                ],
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            ),
        },
    );

    // putChar :: Char -> IO ()
    env_chirho.bind_chirho(
        "putChar".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::char_chirho(),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // sequence_ :: [IO ()] -> IO ()
    env_chirho.bind_chirho(
        "sequence_".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ListChirho(Box::new(TyChirho::io_chirho(TyChirho::unit_chirho()))),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // void :: forall a. IO a -> IO ()
    let void_a_chirho = TyVarChirho(3510);
    env_chirho.bind_chirho(
        "void".to_string(),
        SchemeChirho {
            vars_chirho: vec![void_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::io_chirho(TyChirho::VarChirho(void_a_chirho)),
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            ),
        },
    );

    // guard :: Bool -> IO ()
    env_chirho.bind_chirho(
        "guard".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::bool_chirho(),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // interact :: (String -> String) -> IO ()
    env_chirho.bind_chirho(
        "interact".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::fun_chirho(TyChirho::string_chirho(), TyChirho::string_chirho()),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // lines :: String -> [String]
    env_chirho.bind_chirho(
        "lines".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::ListChirho(Box::new(TyChirho::string_chirho())),
        )),
    );

    // unlines :: [String] -> String
    env_chirho.bind_chirho(
        "unlines".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ListChirho(Box::new(TyChirho::string_chirho())),
            TyChirho::string_chirho(),
        )),
    );

    // error :: forall a. String -> a
    let error_v_chirho = TyVarChirho(3400);
    env_chirho.bind_chirho(
        "error".to_string(),
        SchemeChirho {
            vars_chirho: vec![error_v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                TyChirho::VarChirho(error_v_chirho),
            ),
        },
    );

    // undefined :: forall a. a
    let undef_v_chirho = TyVarChirho(3401);
    env_chirho.bind_chirho(
        "undefined".to_string(),
        SchemeChirho {
            vars_chirho: vec![undef_v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::VarChirho(undef_v_chirho),
        },
    );

    // seq :: forall a b. a -> b -> b
    let seq_a_chirho = TyVarChirho(3402);
    let seq_b_chirho = TyVarChirho(3403);
    env_chirho.bind_chirho(
        "seq".to_string(),
        SchemeChirho {
            vars_chirho: vec![seq_a_chirho, seq_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(seq_a_chirho),
                    TyChirho::VarChirho(seq_b_chirho),
                ],
                TyChirho::VarChirho(seq_b_chirho),
            ),
        },
    );

    // -----------------------------------------------------------------------
    // NFData / deepseq / force / evaluate
    // -----------------------------------------------------------------------

    // deepseq :: forall a b. NFData a => a -> b -> b
    let ds_a_chirho = TyVarChirho(3410);
    let ds_b_chirho = TyVarChirho(3411);
    env_chirho.bind_chirho(
        "deepseq".to_string(),
        SchemeChirho {
            vars_chirho: vec![ds_a_chirho, ds_b_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "NFData".to_string(),
                ty_chirho: TyChirho::VarChirho(ds_a_chirho),
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(ds_a_chirho),
                    TyChirho::VarChirho(ds_b_chirho),
                ],
                TyChirho::VarChirho(ds_b_chirho),
            ),
        },
    );

    // force :: forall a. NFData a => a -> a
    let force_a_chirho = TyVarChirho(3412);
    env_chirho.bind_chirho(
        "force".to_string(),
        SchemeChirho {
            vars_chirho: vec![force_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "NFData".to_string(),
                ty_chirho: TyChirho::VarChirho(force_a_chirho),
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::VarChirho(force_a_chirho)],
                TyChirho::VarChirho(force_a_chirho),
            ),
        },
    );

    // evaluate :: forall a. a -> IO a
    let eval_a_chirho = TyVarChirho(3413);
    env_chirho.bind_chirho(
        "evaluate".to_string(),
        SchemeChirho {
            vars_chirho: vec![eval_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::VarChirho(eval_a_chirho)],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("IO".to_string())),
                    Box::new(TyChirho::VarChirho(eval_a_chirho)),
                ),
            ),
        },
    );

    // rnf :: forall a. NFData a => a -> ()
    let rnf_a_chirho = TyVarChirho(3414);
    env_chirho.bind_chirho(
        "rnf".to_string(),
        SchemeChirho {
            vars_chirho: vec![rnf_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "NFData".to_string(),
                ty_chirho: TyChirho::VarChirho(rnf_a_chirho),
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::VarChirho(rnf_a_chirho)],
                TyChirho::TupleChirho(vec![]),
            ),
        },
    );

    // -----------------------------------------------------------------------
    // Semigroup / Monoid
    // -----------------------------------------------------------------------

    // (<>) :: forall a. Semigroup a => a -> a -> a
    let sg_v_chirho = TyVarChirho(3600);
    env_chirho.bind_chirho(
        "<>".to_string(),
        SchemeChirho {
            vars_chirho: vec![sg_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Semigroup".to_string(),
                ty_chirho: TyChirho::VarChirho(sg_v_chirho),
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                [
                    TyChirho::VarChirho(sg_v_chirho),
                    TyChirho::VarChirho(sg_v_chirho),
                ],
                TyChirho::VarChirho(sg_v_chirho),
            ),
        },
    );

    // mempty :: forall a. Monoid a => a
    let mon_v_chirho = TyVarChirho(3601);
    env_chirho.bind_chirho(
        "mempty".to_string(),
        SchemeChirho {
            vars_chirho: vec![mon_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Monoid".to_string(),
                ty_chirho: TyChirho::VarChirho(mon_v_chirho),
            }],
            ty_chirho: TyChirho::VarChirho(mon_v_chirho),
        },
    );

    // mconcat :: forall a. Monoid a => [a] -> a
    let mc_v_chirho = TyVarChirho(3602);
    env_chirho.bind_chirho(
        "mconcat".to_string(),
        SchemeChirho {
            vars_chirho: vec![mc_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Monoid".to_string(),
                ty_chirho: TyChirho::VarChirho(mc_v_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mc_v_chirho))),
                TyChirho::VarChirho(mc_v_chirho),
            ),
        },
    );

    // -----------------------------------------------------------------------
    // Higher-order *By list Prelude functions
    // -----------------------------------------------------------------------

    // sortBy :: forall a. (a -> a -> Ordering) -> [a] -> [a]
    {
        let sb_a_chirho = TyVarChirho(3700);
        let ordering_chirho = TyChirho::ConChirho("Ordering".to_string());
        env_chirho.bind_chirho(
            "sortBy".to_string(),
            SchemeChirho {
                vars_chirho: vec![sb_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(sb_a_chirho), TyChirho::VarChirho(sb_a_chirho)],
                            ordering_chirho.clone(),
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(sb_a_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(sb_a_chirho))),
                ),
            },
        );
    }

    // insertBy :: forall a. (a -> a -> Ordering) -> a -> [a] -> [a]
    {
        let ib_a_chirho = TyVarChirho(3701);
        let ordering_chirho = TyChirho::ConChirho("Ordering".to_string());
        env_chirho.bind_chirho(
            "insertBy".to_string(),
            SchemeChirho {
                vars_chirho: vec![ib_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(ib_a_chirho), TyChirho::VarChirho(ib_a_chirho)],
                            ordering_chirho,
                        ),
                        TyChirho::VarChirho(ib_a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(ib_a_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(ib_a_chirho))),
                ),
            },
        );
    }

    // nubBy :: forall a. (a -> a -> Bool) -> [a] -> [a]
    {
        let nb_a_chirho = TyVarChirho(3702);
        env_chirho.bind_chirho(
            "nubBy".to_string(),
            SchemeChirho {
                vars_chirho: vec![nb_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(nb_a_chirho), TyChirho::VarChirho(nb_a_chirho)],
                            TyChirho::bool_chirho(),
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(nb_a_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(nb_a_chirho))),
                ),
            },
        );
    }

    // maximumBy :: forall a. (a -> a -> Ordering) -> [a] -> a
    {
        let mx_a_chirho = TyVarChirho(3703);
        let ordering_chirho = TyChirho::ConChirho("Ordering".to_string());
        env_chirho.bind_chirho(
            "maximumBy".to_string(),
            SchemeChirho {
                vars_chirho: vec![mx_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(mx_a_chirho), TyChirho::VarChirho(mx_a_chirho)],
                            ordering_chirho,
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mx_a_chirho))),
                    ],
                    TyChirho::VarChirho(mx_a_chirho),
                ),
            },
        );
    }

    // minimumBy :: forall a. (a -> a -> Ordering) -> [a] -> a
    {
        let mn_a_chirho = TyVarChirho(3704);
        let ordering_chirho = TyChirho::ConChirho("Ordering".to_string());
        env_chirho.bind_chirho(
            "minimumBy".to_string(),
            SchemeChirho {
                vars_chirho: vec![mn_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(mn_a_chirho), TyChirho::VarChirho(mn_a_chirho)],
                            ordering_chirho,
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mn_a_chirho))),
                    ],
                    TyChirho::VarChirho(mn_a_chirho),
                ),
            },
        );
    }

    // on :: forall a b c. (b -> b -> c) -> (a -> b) -> a -> a -> c
    {
        let on_a_chirho = TyVarChirho(3705);
        let on_b_chirho = TyVarChirho(3706);
        let on_c_chirho = TyVarChirho(3707);
        env_chirho.bind_chirho(
            "on".to_string(),
            SchemeChirho {
                vars_chirho: vec![on_a_chirho, on_b_chirho, on_c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(on_b_chirho), TyChirho::VarChirho(on_b_chirho)],
                            TyChirho::VarChirho(on_c_chirho),
                        ),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(on_a_chirho),
                            TyChirho::VarChirho(on_b_chirho),
                        ),
                        TyChirho::VarChirho(on_a_chirho),
                        TyChirho::VarChirho(on_a_chirho),
                    ],
                    TyChirho::VarChirho(on_c_chirho),
                ),
            },
        );
    }

    // find :: forall a. (a -> Bool) -> [a] -> Maybe a
    {
        let find_a_chirho = TyVarChirho(3710);
        env_chirho.bind_chirho(
            "find".to_string(),
            SchemeChirho {
                vars_chirho: vec![find_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(find_a_chirho),
                            TyChirho::bool_chirho(),
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(find_a_chirho))),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(TyChirho::VarChirho(find_a_chirho)),
                    ),
                ),
            },
        );
    }

    // groupBy :: forall a. (a -> a -> Bool) -> [a] -> [[a]]
    {
        let gb_a_chirho = TyVarChirho(3711);
        env_chirho.bind_chirho(
            "groupBy".to_string(),
            SchemeChirho {
                vars_chirho: vec![gb_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![
                                TyChirho::VarChirho(gb_a_chirho),
                                TyChirho::VarChirho(gb_a_chirho),
                            ],
                            TyChirho::bool_chirho(),
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(gb_a_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(gb_a_chirho))),
                    )),
                ),
            },
        );
    }

    // -----------------------------------------------------------------------
    // Higher-order list Prelude functions
    // -----------------------------------------------------------------------

    // map :: forall a b. (a -> b) -> [a] -> [b]
    let map_a_chirho = TyVarChirho(3000);
    let map_b_chirho = TyVarChirho(3001);
    env_chirho.bind_chirho(
        "map".to_string(),
        SchemeChirho {
            vars_chirho: vec![map_a_chirho, map_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(map_a_chirho),
                        TyChirho::VarChirho(map_b_chirho),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(map_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(map_b_chirho))),
            ),
        },
    );

    // filter :: forall a. (a -> Bool) -> [a] -> [a]
    let filter_a_chirho = TyVarChirho(3010);
    env_chirho.bind_chirho(
        "filter".to_string(),
        SchemeChirho {
            vars_chirho: vec![filter_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(filter_a_chirho),
                        TyChirho::bool_chirho(),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(filter_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(filter_a_chirho))),
            ),
        },
    );

    // foldr :: forall a b. (a -> b -> b) -> b -> [a] -> b
    let foldr_a_chirho = TyVarChirho(3020);
    let foldr_b_chirho = TyVarChirho(3021);
    env_chirho.bind_chirho(
        "foldr".to_string(),
        SchemeChirho {
            vars_chirho: vec![foldr_a_chirho, foldr_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(foldr_a_chirho),
                            TyChirho::VarChirho(foldr_b_chirho),
                        ],
                        TyChirho::VarChirho(foldr_b_chirho),
                    ),
                    TyChirho::VarChirho(foldr_b_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(foldr_a_chirho))),
                ],
                TyChirho::VarChirho(foldr_b_chirho),
            ),
        },
    );

    // foldl :: forall a b. (b -> a -> b) -> b -> [a] -> b
    let foldl_a_chirho = TyVarChirho(3030);
    let foldl_b_chirho = TyVarChirho(3031);
    env_chirho.bind_chirho(
        "foldl".to_string(),
        SchemeChirho {
            vars_chirho: vec![foldl_a_chirho, foldl_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(foldl_b_chirho),
                            TyChirho::VarChirho(foldl_a_chirho),
                        ],
                        TyChirho::VarChirho(foldl_b_chirho),
                    ),
                    TyChirho::VarChirho(foldl_b_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(foldl_a_chirho))),
                ],
                TyChirho::VarChirho(foldl_b_chirho),
            ),
        },
    );

    // head :: forall a. [a] -> a
    let head_a_chirho = TyVarChirho(3040);
    env_chirho.bind_chirho(
        "head".to_string(),
        SchemeChirho {
            vars_chirho: vec![head_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(head_a_chirho))),
                TyChirho::VarChirho(head_a_chirho),
            ),
        },
    );

    // tail :: forall a. [a] -> [a]
    let tail_a_chirho = TyVarChirho(3050);
    env_chirho.bind_chirho(
        "tail".to_string(),
        SchemeChirho {
            vars_chirho: vec![tail_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(tail_a_chirho))),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(tail_a_chirho))),
            ),
        },
    );

    // null :: forall a. [a] -> Bool
    let null_a_chirho = TyVarChirho(3060);
    env_chirho.bind_chirho(
        "null".to_string(),
        SchemeChirho {
            vars_chirho: vec![null_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(null_a_chirho))),
                TyChirho::bool_chirho(),
            ),
        },
    );

    // length :: forall a. [a] -> Int
    let length_a_chirho = TyVarChirho(3070);
    env_chirho.bind_chirho(
        "length".to_string(),
        SchemeChirho {
            vars_chirho: vec![length_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(length_a_chirho))),
                TyChirho::int_chirho(),
            ),
        },
    );

    // reverse :: forall a. [a] -> [a]
    let reverse_a_chirho = TyVarChirho(3080);
    env_chirho.bind_chirho(
        "reverse".to_string(),
        SchemeChirho {
            vars_chirho: vec![reverse_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(reverse_a_chirho))),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(reverse_a_chirho))),
            ),
        },
    );

    // zip :: forall a b. [a] -> [b] -> [(a, b)]
    let zip_a_chirho = TyVarChirho(3090);
    let zip_b_chirho = TyVarChirho(3091);
    env_chirho.bind_chirho(
        "zip".to_string(),
        SchemeChirho {
            vars_chirho: vec![zip_a_chirho, zip_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zip_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zip_b_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(zip_a_chirho),
                    TyChirho::VarChirho(zip_b_chirho),
                ]))),
            ),
        },
    );

    // zipWith :: forall a b c. (a -> b -> c) -> [a] -> [b] -> [c]
    let zipw_a_chirho = TyVarChirho(3100);
    let zipw_b_chirho = TyVarChirho(3101);
    let zipw_c_chirho = TyVarChirho(3102);
    env_chirho.bind_chirho(
        "zipWith".to_string(),
        SchemeChirho {
            vars_chirho: vec![zipw_a_chirho, zipw_b_chirho, zipw_c_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(zipw_a_chirho),
                            TyChirho::VarChirho(zipw_b_chirho),
                        ],
                        TyChirho::VarChirho(zipw_c_chirho),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zipw_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zipw_b_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zipw_c_chirho))),
            ),
        },
    );

    // -----------------------------------------------------------------------
    // Additional Prelude list functions
    // -----------------------------------------------------------------------

    // (++) :: forall a. [a] -> [a] -> [a]
    let pp_a_chirho = TyVarChirho(3105);
    env_chirho.bind_chirho(
        "++".to_string(),
        SchemeChirho {
            vars_chirho: vec![pp_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(pp_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(pp_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(pp_a_chirho))),
            ),
        },
    );

    // append :: forall a. [a] -> [a] -> [a]
    let append_a_chirho = TyVarChirho(3110);
    env_chirho.bind_chirho(
        "append".to_string(),
        SchemeChirho {
            vars_chirho: vec![append_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(append_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(append_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(append_a_chirho))),
            ),
        },
    );

    // any :: forall a. (a -> Bool) -> [a] -> Bool
    let any_a_chirho = TyVarChirho(3120);
    env_chirho.bind_chirho(
        "any".to_string(),
        SchemeChirho {
            vars_chirho: vec![any_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(any_a_chirho),
                        TyChirho::bool_chirho(),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(any_a_chirho))),
                ],
                TyChirho::bool_chirho(),
            ),
        },
    );

    // all :: forall a. (a -> Bool) -> [a] -> Bool
    let all_a_chirho = TyVarChirho(3130);
    env_chirho.bind_chirho(
        "all".to_string(),
        SchemeChirho {
            vars_chirho: vec![all_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(all_a_chirho),
                        TyChirho::bool_chirho(),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(all_a_chirho))),
                ],
                TyChirho::bool_chirho(),
            ),
        },
    );

    // sum :: [Int] -> Int  (Int-specialized)
    env_chirho.bind_chirho(
        "sum".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                TyChirho::int_chirho(),
            ),
        },
    );

    // product :: [Int] -> Int  (Int-specialized)
    env_chirho.bind_chirho(
        "product".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                TyChirho::int_chirho(),
            ),
        },
    );

    // concatMap :: forall a b. (a -> [b]) -> [a] -> [b]
    let cm_a_chirho = TyVarChirho(3140);
    let cm_b_chirho = TyVarChirho(3141);
    env_chirho.bind_chirho(
        "concatMap".to_string(),
        SchemeChirho {
            vars_chirho: vec![cm_a_chirho, cm_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(cm_a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cm_b_chirho))),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cm_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cm_b_chirho))),
            ),
        },
    );

    // last :: forall a. [a] -> a
    let last_a_chirho = TyVarChirho(3150);
    env_chirho.bind_chirho(
        "last".to_string(),
        SchemeChirho {
            vars_chirho: vec![last_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(last_a_chirho))),
                TyChirho::VarChirho(last_a_chirho),
            ),
        },
    );

    // init :: forall a. [a] -> [a]
    let init_a_chirho = TyVarChirho(3160);
    env_chirho.bind_chirho(
        "init".to_string(),
        SchemeChirho {
            vars_chirho: vec![init_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(init_a_chirho))),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(init_a_chirho))),
            ),
        },
    );

    // elem :: forall a. a -> [a] -> Bool
    let elem_a_chirho = TyVarChirho(3500);
    env_chirho.bind_chirho(
        "elem".to_string(),
        SchemeChirho {
            vars_chirho: vec![elem_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(elem_a_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(elem_a_chirho))),
                ],
                TyChirho::bool_chirho(),
            ),
        },
    );

    // notElem :: forall a. a -> [a] -> Bool
    let notelem_a_chirho = TyVarChirho(3501);
    env_chirho.bind_chirho(
        "notElem".to_string(),
        SchemeChirho {
            vars_chirho: vec![notelem_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(notelem_a_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(notelem_a_chirho))),
                ],
                TyChirho::bool_chirho(),
            ),
        },
    );

    // minimum :: [Int] -> Int  (Int-specialized)
    env_chirho.bind_chirho(
        "minimum".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                TyChirho::int_chirho(),
            ),
        },
    );

    // maximum :: [Int] -> Int  (Int-specialized)
    env_chirho.bind_chirho(
        "maximum".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                TyChirho::int_chirho(),
            ),
        },
    );

    // sort :: [Int] -> [Int]  (Int-specialized insertion sort)
    env_chirho.bind_chirho(
        "sort".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
            ),
        },
    );

    // insert :: Int -> [Int] -> [Int]  (insertion for sort)
    env_chirho.bind_chirho(
        "insert".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::int_chirho(),
                    TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
            ),
        },
    );

    // abs :: forall a. Num a => a -> a
    {
        let abs_v_chirho = TyVarChirho(2210);
        env_chirho.bind_chirho(
            "abs".to_string(),
            SchemeChirho {
                vars_chirho: vec![abs_v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Num".to_string(),
                    ty_chirho: TyChirho::VarChirho(abs_v_chirho),
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(abs_v_chirho),
                    TyChirho::VarChirho(abs_v_chirho),
                ),
            },
        );
    }

    // signum :: forall a. Num a => a -> a
    {
        let sig_v_chirho = TyVarChirho(2211);
        env_chirho.bind_chirho(
            "signum".to_string(),
            SchemeChirho {
                vars_chirho: vec![sig_v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Num".to_string(),
                    ty_chirho: TyChirho::VarChirho(sig_v_chirho),
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(sig_v_chirho),
                    TyChirho::VarChirho(sig_v_chirho),
                ),
            },
        );
    }

    // even :: Int -> Bool
    env_chirho.bind_chirho(
        "even".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::bool_chirho())),
    );

    // odd :: Int -> Bool
    env_chirho.bind_chirho(
        "odd".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::bool_chirho())),
    );

    // replicate :: forall a. Int -> a -> [a]
    let rep_a_chirho = TyVarChirho(3250);
    env_chirho.bind_chirho(
        "replicate".to_string(),
        SchemeChirho {
            vars_chirho: vec![rep_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::int_chirho(), TyChirho::VarChirho(rep_a_chirho)],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(rep_a_chirho))),
            ),
        },
    );

    // takeWhile :: forall a. (a -> Bool) -> [a] -> [a]
    let tw_a_chirho = TyVarChirho(3300);
    env_chirho.bind_chirho(
        "takeWhile".to_string(),
        SchemeChirho {
            vars_chirho: vec![tw_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(TyChirho::VarChirho(tw_a_chirho), TyChirho::bool_chirho()),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(tw_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(tw_a_chirho))),
            ),
        },
    );

    // dropWhile :: forall a. (a -> Bool) -> [a] -> [a]
    let dw_a_chirho = TyVarChirho(3301);
    env_chirho.bind_chirho(
        "dropWhile".to_string(),
        SchemeChirho {
            vars_chirho: vec![dw_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(TyChirho::VarChirho(dw_a_chirho), TyChirho::bool_chirho()),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(dw_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(dw_a_chirho))),
            ),
        },
    );

    // iterate :: forall a. (a -> a) -> a -> [a]
    let iter_a_chirho = TyVarChirho(3302);
    env_chirho.bind_chirho(
        "iterate".to_string(),
        SchemeChirho {
            vars_chirho: vec![iter_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(TyChirho::VarChirho(iter_a_chirho), TyChirho::VarChirho(iter_a_chirho)),
                    TyChirho::VarChirho(iter_a_chirho),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(iter_a_chirho))),
            ),
        },
    );

    // lookup :: forall a b. a -> [(a,b)] -> Maybe b
    let lu_a_chirho = TyVarChirho(3303);
    let lu_b_chirho = TyVarChirho(3304);
    env_chirho.bind_chirho(
        "lookup".to_string(),
        SchemeChirho {
            vars_chirho: vec![lu_a_chirho, lu_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(lu_a_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(lu_a_chirho),
                        TyChirho::VarChirho(lu_b_chirho),
                    ]))),
                ],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(lu_b_chirho)),
                ),
            ),
        },
    );

    // unzip :: forall a b. [(a,b)] -> ([a],[b])
    let uz_a_chirho = TyVarChirho(3305);
    let uz_b_chirho = TyVarChirho(3306);
    env_chirho.bind_chirho(
        "unzip".to_string(),
        SchemeChirho {
            vars_chirho: vec![uz_a_chirho, uz_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(uz_a_chirho),
                    TyChirho::VarChirho(uz_b_chirho),
                ]))),
                TyChirho::TupleChirho(vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(uz_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(uz_b_chirho))),
                ]),
            ),
        },
    );

    // scanl :: forall a b. (b -> a -> b) -> b -> [a] -> [b]
    let sl_a_chirho = TyVarChirho(3307);
    let sl_b_chirho = TyVarChirho(3308);
    env_chirho.bind_chirho(
        "scanl".to_string(),
        SchemeChirho {
            vars_chirho: vec![sl_a_chirho, sl_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![TyChirho::VarChirho(sl_b_chirho), TyChirho::VarChirho(sl_a_chirho)],
                        TyChirho::VarChirho(sl_b_chirho),
                    ),
                    TyChirho::VarChirho(sl_b_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(sl_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(sl_b_chirho))),
            ),
        },
    );

    // span :: forall a. (a -> Bool) -> [a] -> ([a],[a])
    let span_a_chirho = TyVarChirho(3309);
    env_chirho.bind_chirho(
        "span".to_string(),
        SchemeChirho {
            vars_chirho: vec![span_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(TyChirho::VarChirho(span_a_chirho), TyChirho::bool_chirho()),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(span_a_chirho))),
                ],
                TyChirho::TupleChirho(vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(span_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(span_a_chirho))),
                ]),
            ),
        },
    );

    // break :: forall a. (a -> Bool) -> [a] -> ([a],[a])
    let break_a_chirho = TyVarChirho(3310);
    env_chirho.bind_chirho(
        "break".to_string(),
        SchemeChirho {
            vars_chirho: vec![break_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(TyChirho::VarChirho(break_a_chirho), TyChirho::bool_chirho()),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(break_a_chirho))),
                ],
                TyChirho::TupleChirho(vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(break_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(break_a_chirho))),
                ]),
            ),
        },
    );

    // partition :: forall a. (a -> Bool) -> [a] -> ([a],[a])
    let part_a_chirho = TyVarChirho(3311);
    env_chirho.bind_chirho(
        "partition".to_string(),
        SchemeChirho {
            vars_chirho: vec![part_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(TyChirho::VarChirho(part_a_chirho), TyChirho::bool_chirho()),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(part_a_chirho))),
                ],
                TyChirho::TupleChirho(vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(part_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(part_a_chirho))),
                ]),
            ),
        },
    );

    // mapInsertStr :: String -> v -> Map -> Map
    {
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3240));
        env_chirho.bind_chirho(
            "mapInsertStr".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3240)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::string_chirho(), v_chirho, TyChirho::int_chirho()],
                    TyChirho::int_chirho(),
                ),
            },
        );
    }

    // mapLookupStr :: String -> Map -> Maybe v
    {
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3241));
        env_chirho.bind_chirho(
            "mapLookupStr".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3241)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::string_chirho(),
                    TyChirho::fun_chirho(TyChirho::int_chirho(), v_chirho),
                ),
            },
        );
    }

    // mapMemberStr :: String -> Map -> Bool
    env_chirho.bind_chirho(
        "mapMemberStr".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::bool_chirho()),
        )),
    );

    // mapFindWithDefaultStr :: v -> String -> Map -> v
    {
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3242));
        env_chirho.bind_chirho(
            "mapFindWithDefaultStr".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3242)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![v_chirho.clone(), TyChirho::string_chirho(), TyChirho::int_chirho()],
                    v_chirho,
                ),
            },
        );
    }

    // nub :: [a] -> [a]
    {
        let nub_a_chirho = TyVarChirho(3400);
        env_chirho.bind_chirho(
            "nub".to_string(),
            SchemeChirho {
                vars_chirho: vec![nub_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(nub_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(nub_a_chirho))),
                ),
            },
        );
    }

    // zip3 :: [a] -> [b] -> [c] -> [(a,b,c)]
    {
        let z3a_chirho = TyVarChirho(3401);
        let z3b_chirho = TyVarChirho(3402);
        let z3c_chirho = TyVarChirho(3403);
        env_chirho.bind_chirho(
            "zip3".to_string(),
            SchemeChirho {
                vars_chirho: vec![z3a_chirho, z3b_chirho, z3c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(z3a_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(z3b_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(z3c_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(z3a_chirho),
                        TyChirho::VarChirho(z3b_chirho),
                        TyChirho::VarChirho(z3c_chirho),
                    ]))),
                ),
            },
        );
    }

    // intersperse :: a -> [a] -> [a]
    {
        let isp_a_chirho = TyVarChirho(3404);
        env_chirho.bind_chirho(
            "intersperse".to_string(),
            SchemeChirho {
                vars_chirho: vec![isp_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(isp_a_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(isp_a_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(isp_a_chirho))),
                    ),
                ),
            },
        );
    }

    // isPrefixOf :: [a] -> [a] -> Bool
    {
        let ipf_a_chirho = TyVarChirho(3405);
        env_chirho.bind_chirho(
            "isPrefixOf".to_string(),
            SchemeChirho {
                vars_chirho: vec![ipf_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(ipf_a_chirho))),
                    TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(ipf_a_chirho))),
                        TyChirho::bool_chirho(),
                    ),
                ),
            },
        );
    }

    // isSuffixOf :: [a] -> [a] -> Bool
    {
        let isf_a_chirho = TyVarChirho(3406);
        env_chirho.bind_chirho(
            "isSuffixOf".to_string(),
            SchemeChirho {
                vars_chirho: vec![isf_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(isf_a_chirho))),
                    TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(isf_a_chirho))),
                        TyChirho::bool_chirho(),
                    ),
                ),
            },
        );
    }

    // unzip3 :: [(a,b,c)] -> ([a],[b],[c])
    {
        let uz3a_chirho = TyVarChirho(3407);
        let uz3b_chirho = TyVarChirho(3408);
        let uz3c_chirho = TyVarChirho(3409);
        env_chirho.bind_chirho(
            "unzip3".to_string(),
            SchemeChirho {
                vars_chirho: vec![uz3a_chirho, uz3b_chirho, uz3c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(uz3a_chirho),
                        TyChirho::VarChirho(uz3b_chirho),
                        TyChirho::VarChirho(uz3c_chirho),
                    ]))),
                    TyChirho::TupleChirho(vec![
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(uz3a_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(uz3b_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(uz3c_chirho))),
                    ]),
                ),
            },
        );
    }

    // toEnum :: forall a. Enum a => Int -> a
    let enum_a_chirho = TyVarChirho(3260);
    env_chirho.bind_chirho(
        "toEnum".to_string(),
        SchemeChirho {
            vars_chirho: vec![enum_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(enum_a_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::int_chirho(),
                TyChirho::VarChirho(enum_a_chirho),
            ),
        },
    );

    // fromEnum :: forall a. Enum a => a -> Int
    let from_enum_a_chirho = TyVarChirho(3261);
    env_chirho.bind_chirho(
        "fromEnum".to_string(),
        SchemeChirho {
            vars_chirho: vec![from_enum_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(from_enum_a_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(from_enum_a_chirho),
                TyChirho::int_chirho(),
            ),
        },
    );

    // succ :: forall a. Enum a => a -> a
    let succ_a_chirho = TyVarChirho(3262);
    env_chirho.bind_chirho(
        "succ".to_string(),
        SchemeChirho {
            vars_chirho: vec![succ_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(succ_a_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(succ_a_chirho),
                TyChirho::VarChirho(succ_a_chirho),
            ),
        },
    );

    // pred :: forall a. Enum a => a -> a
    let pred_a_chirho = TyVarChirho(3263);
    env_chirho.bind_chirho(
        "pred".to_string(),
        SchemeChirho {
            vars_chirho: vec![pred_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(pred_a_chirho),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(pred_a_chirho),
                TyChirho::VarChirho(pred_a_chirho),
            ),
        },
    );

    // enumFrom :: Int -> [Int]
    env_chirho.bind_chirho(
        "enumFrom".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
        )),
    );

    // enumFromThen :: Int -> Int -> [Int]
    env_chirho.bind_chirho(
        "enumFromThen".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::fun_chirho(
                TyChirho::int_chirho(),
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
            ),
        )),
    );

    // enumFromTo :: Int -> Int -> [Int]
    env_chirho.bind_chirho(
        "enumFromTo".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::fun_chirho(
                TyChirho::int_chirho(),
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
            ),
        )),
    );

    // enumFromThenTo :: Int -> Int -> Int -> [Int]
    env_chirho.bind_chirho(
        "enumFromThenTo".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::fun_chirho(
                TyChirho::int_chirho(),
                TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                ),
            ),
        )),
    );

    // minBound :: forall a. Bounded a => a
    let min_bound_a_chirho = TyVarChirho(3264);
    env_chirho.bind_chirho(
        "minBound".to_string(),
        SchemeChirho {
            vars_chirho: vec![min_bound_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Bounded".to_string(),
                ty_chirho: TyChirho::VarChirho(min_bound_a_chirho),
            }],
            ty_chirho: TyChirho::VarChirho(min_bound_a_chirho),
        },
    );

    // maxBound :: forall a. Bounded a => a
    let max_bound_a_chirho = TyVarChirho(3265);
    env_chirho.bind_chirho(
        "maxBound".to_string(),
        SchemeChirho {
            vars_chirho: vec![max_bound_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Bounded".to_string(),
                ty_chirho: TyChirho::VarChirho(max_bound_a_chirho),
            }],
            ty_chirho: TyChirho::VarChirho(max_bound_a_chirho),
        },
    );

    // Data.Char functions
    let char_ty_chirho = TyChirho::ConChirho("Char".to_string());

    // chr :: Int -> Char
    env_chirho.bind_chirho(
        "chr".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            char_ty_chirho.clone(),
        )),
    );

    // ord :: Char -> Int
    env_chirho.bind_chirho(
        "ord".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            char_ty_chirho.clone(),
            TyChirho::int_chirho(),
        )),
    );

    // Char -> Bool functions
    for name_chirho in &["isDigit", "isAlpha", "isAlphaNum", "isUpper", "isLower", "isSpace"] {
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                char_ty_chirho.clone(),
                TyChirho::bool_chirho(),
            )),
        );
    }

    // Char -> Char functions
    for name_chirho in &["toLower", "toUpper"] {
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                char_ty_chirho.clone(),
                char_ty_chirho.clone(),
            )),
        );
    }

    // digitToInt :: Char -> Int
    env_chirho.bind_chirho(
        "digitToInt".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            char_ty_chirho,
            TyChirho::int_chirho(),
        )),
    );

    // intToDigit :: Int -> Char
    env_chirho.bind_chirho(
        "intToDigit".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::ConChirho("Char".to_string()),
        )),
    );

    // ── Floating functions (monomorphic at Double for now) ──
    let double_ty_chirho = TyChirho::double_chirho();

    // Unary: sin, cos, tan, asin, acos, atan, exp, log, sqrt :: Double -> Double
    for name_chirho in [
        "sin", "cos", "tan", "asin", "acos", "atan", "exp", "log", "sqrt",
    ] {
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                double_ty_chirho.clone(),
                double_ty_chirho.clone(),
            )),
        );
    }

    // NOTE: pi is not bound at the top level to avoid shadowing user-defined
    // local "pi" bindings. It is available via $prim_Floating_pi_Double.

    // ── Functor / Applicative / Monad ──
    // fmap :: Functor f => (a -> b) -> f a -> f b  (polymorphic)
    {
        let fmap_f_chirho = TyVarChirho(9004);
        let fmap_a_chirho = TyVarChirho(9040);
        let fmap_b_chirho = TyVarChirho(9041);
        env_chirho.bind_chirho(
            "fmap".to_string(),
            SchemeChirho {
                vars_chirho: vec![fmap_f_chirho, fmap_a_chirho, fmap_b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::VarChirho(fmap_a_chirho)),
                        Box::new(TyChirho::VarChirho(fmap_b_chirho)),
                     MultChirho::ManyChirho,)),
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(fmap_f_chirho)),
                            Box::new(TyChirho::VarChirho(fmap_a_chirho)),
                        )),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(fmap_f_chirho)),
                            Box::new(TyChirho::VarChirho(fmap_b_chirho)),
                        )),
                     MultChirho::ManyChirho,)),
                 MultChirho::ManyChirho,),
            },
        );
    }

    // foldMap :: (a -> m) -> t a -> m  (simplified, Monoid m not enforced)
    {
        let fold_a_chirho = TyVarChirho(9081);
        let fold_m_chirho = TyVarChirho(9082);
        let fold_t_chirho = TyVarChirho(9080);
        env_chirho.bind_chirho(
            "foldMap".to_string(),
            SchemeChirho {
                vars_chirho: vec![fold_t_chirho, fold_a_chirho, fold_m_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::VarChirho(fold_a_chirho)),
                        Box::new(TyChirho::VarChirho(fold_m_chirho)),
                     MultChirho::ManyChirho,)),
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(fold_t_chirho)),
                            Box::new(TyChirho::VarChirho(fold_a_chirho)),
                        )),
                        Box::new(TyChirho::VarChirho(fold_m_chirho)),
                     MultChirho::ManyChirho,)),
                 MultChirho::ManyChirho,),
            },
        );
    }

    // traverse :: (a -> f b) -> t a -> f (t b)  (simplified, Applicative f not enforced)
    {
        let trav_t_chirho = TyVarChirho(9083);
        let trav_a_chirho = TyVarChirho(9084);
        let trav_b_chirho = TyVarChirho(9085);
        let trav_f_chirho = TyVarChirho(9086);
        env_chirho.bind_chirho(
            "traverse".to_string(),
            SchemeChirho {
                vars_chirho: vec![trav_t_chirho, trav_a_chirho, trav_b_chirho, trav_f_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::VarChirho(trav_a_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(trav_f_chirho)),
                            Box::new(TyChirho::VarChirho(trav_b_chirho)),
                        )),
                     MultChirho::ManyChirho,)),
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(trav_t_chirho)),
                            Box::new(TyChirho::VarChirho(trav_a_chirho)),
                        )),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(trav_f_chirho)),
                            Box::new(TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(trav_t_chirho)),
                                Box::new(TyChirho::VarChirho(trav_b_chirho)),
                            )),
                        )),
                     MultChirho::ManyChirho,)),
                 MultChirho::ManyChirho,),
            },
        );
    }

    // ── Monad transformer infrastructure ──
    //
    // MaybeT is a newtype: newtype MaybeT m a = MaybeT { runMaybeT :: m (Maybe a) }
    // StateT is a newtype: newtype StateT s m a = StateT { runStateT :: s -> m (a, s) }
    //
    // We seed simplified type signatures for the constructor and accessor
    // functions so the type checker can resolve references in user programs.
    // These simplified versions treat the inner monad as the identity and are
    // sufficient for basic end-to-end evaluation tests.

    // MaybeT :: Maybe a -> MaybeT a  (simplified; no inner-monad parameter)
    // runMaybeT :: MaybeT a -> Maybe a  (simplified)
    {
        let mt_a_chirho = TyVarChirho(3500);
        let maybe_a_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(TyChirho::VarChirho(mt_a_chirho)),
        );
        let mayet_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("MaybeT".to_string())),
            Box::new(TyChirho::VarChirho(mt_a_chirho)),
        );
        env_chirho.bind_chirho(
            "MaybeT".to_string(),
            SchemeChirho {
                vars_chirho: vec![mt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    maybe_a_ty_chirho.clone(),
                    mayet_ty_chirho.clone(),
                ),
            },
        );
        env_chirho.bind_chirho(
            "runMaybeT".to_string(),
            SchemeChirho {
                vars_chirho: vec![mt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    mayet_ty_chirho,
                    maybe_a_ty_chirho,
                ),
            },
        );
    }

    // StateT :: (s -> (a, s)) -> StateT s a  (simplified; no inner-monad parameter)
    // runStateT :: StateT s a -> s -> (a, s)  (simplified)
    {
        let st_s_chirho = TyVarChirho(3501);
        let st_a_chirho = TyVarChirho(3502);
        let st_fn_ty_chirho = TyChirho::fun_chirho(
            TyChirho::VarChirho(st_s_chirho),
            TyChirho::TupleChirho(vec![
                TyChirho::VarChirho(st_a_chirho),
                TyChirho::VarChirho(st_s_chirho),
            ]),
        );
        let statet_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(TyChirho::VarChirho(st_s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(st_a_chirho)),
        );
        env_chirho.bind_chirho(
            "StateT".to_string(),
            SchemeChirho {
                vars_chirho: vec![st_s_chirho, st_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    st_fn_ty_chirho.clone(),
                    statet_ty_chirho.clone(),
                ),
            },
        );
        env_chirho.bind_chirho(
            "runStateT".to_string(),
            SchemeChirho {
                vars_chirho: vec![st_s_chirho, st_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    statet_ty_chirho,
                    st_fn_ty_chirho,
                ),
            },
        );
    }

    // lift :: forall t m a. m a -> t m a
    // Simplified: polymorphic binding for MonadTrans-like usage
    {
        let lt_m_chirho = TyVarChirho(3503);
        let lt_t_chirho = TyVarChirho(3504);
        let lt_a_chirho = TyVarChirho(3505);
        let lt_ma_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(lt_m_chirho)),
            Box::new(TyChirho::VarChirho(lt_a_chirho)),
        );
        let lt_tma_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(lt_t_chirho)),
                Box::new(TyChirho::VarChirho(lt_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(lt_a_chirho)),
        );
        env_chirho.bind_chirho(
            "lift".to_string(),
            SchemeChirho {
                vars_chirho: vec![lt_m_chirho, lt_t_chirho, lt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(lt_ma_chirho, lt_tma_chirho),
            },
        );
    }

    // -----------------------------------------------------------------------
    // StateT monad operations
    // -----------------------------------------------------------------------

    // get :: StateT s s
    {
        let gs_chirho = TyVarChirho(3510);
        let state_t_s_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(TyChirho::VarChirho(gs_chirho)),
            )),
            Box::new(TyChirho::VarChirho(gs_chirho)),
        );
        env_chirho.bind_chirho(
            "get".to_string(),
            SchemeChirho {
                vars_chirho: vec![gs_chirho],
                preds_chirho: vec![],
                ty_chirho: state_t_s_chirho,
            },
        );
    }

    // put :: s -> StateT s ()
    {
        let ps_chirho = TyVarChirho(3511);
        let state_t_unit_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(TyChirho::VarChirho(ps_chirho)),
            )),
            Box::new(TyChirho::TupleChirho(vec![])),
        );
        env_chirho.bind_chirho(
            "put".to_string(),
            SchemeChirho {
                vars_chirho: vec![ps_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(ps_chirho),
                    state_t_unit_chirho,
                ),
            },
        );
    }

    // modify :: (s -> s) -> StateT s ()
    {
        let ms_chirho = TyVarChirho(3512);
        let state_t_unit_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(TyChirho::VarChirho(ms_chirho)),
            )),
            Box::new(TyChirho::TupleChirho(vec![])),
        );
        env_chirho.bind_chirho(
            "modify".to_string(),
            SchemeChirho {
                vars_chirho: vec![ms_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(ms_chirho),
                        TyChirho::VarChirho(ms_chirho),
                    ),
                    state_t_unit_chirho,
                ),
            },
        );
    }

    // evalState :: StateT s a -> s -> a
    {
        let es_s_chirho = TyVarChirho(3513);
        let es_a_chirho = TyVarChirho(3514);
        let state_t_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(TyChirho::VarChirho(es_s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(es_a_chirho)),
        );
        env_chirho.bind_chirho(
            "evalState".to_string(),
            SchemeChirho {
                vars_chirho: vec![es_s_chirho, es_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    state_t_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(es_s_chirho),
                        TyChirho::VarChirho(es_a_chirho),
                    ),
                ),
            },
        );
    }

    // execState :: StateT s a -> s -> s
    {
        let xs_s_chirho = TyVarChirho(3515);
        let xs_a_chirho = TyVarChirho(3516);
        let state_t_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(TyChirho::VarChirho(xs_s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(xs_a_chirho)),
        );
        env_chirho.bind_chirho(
            "execState".to_string(),
            SchemeChirho {
                vars_chirho: vec![xs_s_chirho, xs_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    state_t_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(xs_s_chirho),
                        TyChirho::VarChirho(xs_s_chirho),
                    ),
                ),
            },
        );
    }

    // bindStateT :: StateT s a -> (a -> StateT s b) -> StateT s b
    {
        let bs_s_chirho = TyVarChirho(3517);
        let bs_a_chirho = TyVarChirho(3518);
        let bs_b_chirho = TyVarChirho(3519);
        let state_t_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(TyChirho::VarChirho(bs_s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(bs_a_chirho)),
        );
        let state_t_b_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(TyChirho::VarChirho(bs_s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(bs_b_chirho)),
        );
        env_chirho.bind_chirho(
            "bindStateT".to_string(),
            SchemeChirho {
                vars_chirho: vec![bs_s_chirho, bs_a_chirho, bs_b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    state_t_a_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(bs_a_chirho),
                            state_t_b_chirho.clone(),
                        ),
                        state_t_b_chirho,
                    ),
                ),
            },
        );
    }

    // returnStateT :: a -> StateT s a
    {
        let rs_s_chirho = TyVarChirho(3520);
        let rs_a_chirho = TyVarChirho(3521);
        let state_t_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(TyChirho::VarChirho(rs_s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(rs_a_chirho)),
        );
        env_chirho.bind_chirho(
            "returnStateT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rs_s_chirho, rs_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(rs_a_chirho),
                    state_t_chirho,
                ),
            },
        );
    }

    // runState :: StateT s a -> s -> (a, s)  (alias for runStateT)
    {
        let rns_s_chirho = TyVarChirho(3522);
        let rns_a_chirho = TyVarChirho(3523);
        let state_t_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(TyChirho::VarChirho(rns_s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(rns_a_chirho)),
        );
        env_chirho.bind_chirho(
            "runState".to_string(),
            SchemeChirho {
                vars_chirho: vec![rns_s_chirho, rns_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    state_t_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(rns_s_chirho),
                        TyChirho::TupleChirho(vec![
                            TyChirho::VarChirho(rns_a_chirho),
                            TyChirho::VarChirho(rns_s_chirho),
                        ]),
                    ),
                ),
            },
        );
    }

    // -----------------------------------------------------------------------
    // ReaderT monad transformer
    // -----------------------------------------------------------------------

    // ReaderT :: (r -> a) -> ReaderT r a
    // runReaderT :: ReaderT r a -> r -> a
    {
        let rt_r_chirho = TyVarChirho(3530);
        let rt_a_chirho = TyVarChirho(3531);
        let rt_fn_ty_chirho = TyChirho::fun_chirho(
            TyChirho::VarChirho(rt_r_chirho),
            TyChirho::VarChirho(rt_a_chirho),
        );
        let readert_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                Box::new(TyChirho::VarChirho(rt_r_chirho)),
            )),
            Box::new(TyChirho::VarChirho(rt_a_chirho)),
        );
        env_chirho.bind_chirho(
            "ReaderT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rt_r_chirho, rt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    rt_fn_ty_chirho.clone(),
                    readert_ty_chirho.clone(),
                ),
            },
        );
        env_chirho.bind_chirho(
            "runReaderT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rt_r_chirho, rt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    readert_ty_chirho.clone(),
                    rt_fn_ty_chirho.clone(),
                ),
            },
        );
    }

    // ask :: ReaderT r r
    {
        let ar_chirho = TyVarChirho(3532);
        let ask_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                Box::new(TyChirho::VarChirho(ar_chirho)),
            )),
            Box::new(TyChirho::VarChirho(ar_chirho)),
        );
        env_chirho.bind_chirho(
            "ask".to_string(),
            SchemeChirho {
                vars_chirho: vec![ar_chirho],
                preds_chirho: vec![],
                ty_chirho: ask_ty_chirho,
            },
        );
    }

    // local :: (r -> r) -> ReaderT r a -> ReaderT r a
    {
        let lr_chirho = TyVarChirho(3533);
        let la_chirho = TyVarChirho(3534);
        let readert_ra_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                Box::new(TyChirho::VarChirho(lr_chirho)),
            )),
            Box::new(TyChirho::VarChirho(la_chirho)),
        );
        env_chirho.bind_chirho(
            "local".to_string(),
            SchemeChirho {
                vars_chirho: vec![lr_chirho, la_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(lr_chirho),
                        TyChirho::VarChirho(lr_chirho),
                    ),
                    TyChirho::fun_chirho(
                        readert_ra_chirho.clone(),
                        readert_ra_chirho,
                    ),
                ),
            },
        );
    }

    // bindReaderT :: ReaderT r a -> (a -> ReaderT r b) -> ReaderT r b
    {
        let br_chirho = TyVarChirho(3535);
        let ba_chirho = TyVarChirho(3536);
        let bb_chirho = TyVarChirho(3537);
        let readert_ra_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                Box::new(TyChirho::VarChirho(br_chirho)),
            )),
            Box::new(TyChirho::VarChirho(ba_chirho)),
        );
        let readert_rb_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                Box::new(TyChirho::VarChirho(br_chirho)),
            )),
            Box::new(TyChirho::VarChirho(bb_chirho)),
        );
        env_chirho.bind_chirho(
            "bindReaderT".to_string(),
            SchemeChirho {
                vars_chirho: vec![br_chirho, ba_chirho, bb_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    readert_ra_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(ba_chirho),
                            readert_rb_chirho.clone(),
                        ),
                        readert_rb_chirho,
                    ),
                ),
            },
        );
    }

    // returnReaderT :: a -> ReaderT r a
    {
        let rr_chirho = TyVarChirho(3538);
        let ra_chirho = TyVarChirho(3539);
        let readert_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                Box::new(TyChirho::VarChirho(rr_chirho)),
            )),
            Box::new(TyChirho::VarChirho(ra_chirho)),
        );
        env_chirho.bind_chirho(
            "returnReaderT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rr_chirho, ra_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(ra_chirho),
                    readert_ty_chirho,
                ),
            },
        );
    }

    // runReader :: ReaderT r a -> r -> a  (alias)
    {
        let rnr_r_chirho = TyVarChirho(3540);
        let rnr_a_chirho = TyVarChirho(3541);
        let readert_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                Box::new(TyChirho::VarChirho(rnr_r_chirho)),
            )),
            Box::new(TyChirho::VarChirho(rnr_a_chirho)),
        );
        env_chirho.bind_chirho(
            "runReader".to_string(),
            SchemeChirho {
                vars_chirho: vec![rnr_r_chirho, rnr_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    readert_ty_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(rnr_r_chirho),
                        TyChirho::VarChirho(rnr_a_chirho),
                    ),
                ),
            },
        );
    }

    // -----------------------------------------------------------------------
    // ExceptT monad transformer
    // -----------------------------------------------------------------------

    // ExceptT :: Either e a -> ExceptT e a
    // runExceptT :: ExceptT e a -> Either e a
    {
        let et_e_chirho = TyVarChirho(3550);
        let et_a_chirho = TyVarChirho(3551);
        let either_ea_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Either".to_string())),
                Box::new(TyChirho::VarChirho(et_e_chirho)),
            )),
            Box::new(TyChirho::VarChirho(et_a_chirho)),
        );
        let exceptt_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                Box::new(TyChirho::VarChirho(et_e_chirho)),
            )),
            Box::new(TyChirho::VarChirho(et_a_chirho)),
        );
        env_chirho.bind_chirho(
            "ExceptT".to_string(),
            SchemeChirho {
                vars_chirho: vec![et_e_chirho, et_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    either_ea_chirho.clone(),
                    exceptt_ty_chirho.clone(),
                ),
            },
        );
        env_chirho.bind_chirho(
            "runExceptT".to_string(),
            SchemeChirho {
                vars_chirho: vec![et_e_chirho, et_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    exceptt_ty_chirho,
                    either_ea_chirho,
                ),
            },
        );
    }

    // throwE :: e -> ExceptT e a
    {
        let te_e_chirho = TyVarChirho(3552);
        let te_a_chirho = TyVarChirho(3553);
        let exceptt_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                Box::new(TyChirho::VarChirho(te_e_chirho)),
            )),
            Box::new(TyChirho::VarChirho(te_a_chirho)),
        );
        env_chirho.bind_chirho(
            "throwE".to_string(),
            SchemeChirho {
                vars_chirho: vec![te_e_chirho, te_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(te_e_chirho),
                    exceptt_ty_chirho,
                ),
            },
        );
    }

    // returnExceptT :: a -> ExceptT e a
    {
        let re_e_chirho = TyVarChirho(3554);
        let re_a_chirho = TyVarChirho(3555);
        let exceptt_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                Box::new(TyChirho::VarChirho(re_e_chirho)),
            )),
            Box::new(TyChirho::VarChirho(re_a_chirho)),
        );
        env_chirho.bind_chirho(
            "returnExceptT".to_string(),
            SchemeChirho {
                vars_chirho: vec![re_e_chirho, re_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(re_a_chirho),
                    exceptt_ty_chirho,
                ),
            },
        );
    }

    // bindExceptT :: ExceptT e a -> (a -> ExceptT e b) -> ExceptT e b
    {
        let be_e_chirho = TyVarChirho(3556);
        let be_a_chirho = TyVarChirho(3557);
        let be_b_chirho = TyVarChirho(3558);
        let exceptt_ea_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                Box::new(TyChirho::VarChirho(be_e_chirho)),
            )),
            Box::new(TyChirho::VarChirho(be_a_chirho)),
        );
        let exceptt_eb_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                Box::new(TyChirho::VarChirho(be_e_chirho)),
            )),
            Box::new(TyChirho::VarChirho(be_b_chirho)),
        );
        env_chirho.bind_chirho(
            "bindExceptT".to_string(),
            SchemeChirho {
                vars_chirho: vec![be_e_chirho, be_a_chirho, be_b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    exceptt_ea_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(be_a_chirho),
                            exceptt_eb_chirho.clone(),
                        ),
                        exceptt_eb_chirho,
                    ),
                ),
            },
        );
    }

    // catchE :: ExceptT e a -> (e -> ExceptT e a) -> ExceptT e a
    {
        let ce_e_chirho = TyVarChirho(3559);
        let ce_a_chirho = TyVarChirho(3560);
        let exceptt_ea_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                Box::new(TyChirho::VarChirho(ce_e_chirho)),
            )),
            Box::new(TyChirho::VarChirho(ce_a_chirho)),
        );
        env_chirho.bind_chirho(
            "catchE".to_string(),
            SchemeChirho {
                vars_chirho: vec![ce_e_chirho, ce_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    exceptt_ea_chirho.clone(),
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(ce_e_chirho),
                            exceptt_ea_chirho.clone(),
                        ),
                        exceptt_ea_chirho,
                    ),
                ),
            },
        );
    }

    // -----------------------------------------------------------------------
    // MaybeT operations (returnMaybeT, bindMaybeT)
    // -----------------------------------------------------------------------

    // returnMaybeT :: forall a. a -> MaybeT a
    {
        let rmt_a_chirho = TyVarChirho(3570);
        let maybet_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("MaybeT".to_string())),
            Box::new(TyChirho::VarChirho(rmt_a_chirho)),
        );
        env_chirho.bind_chirho(
            "returnMaybeT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rmt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(rmt_a_chirho),
                    maybet_a_chirho,
                ),
            },
        );
    }

    // bindMaybeT :: forall a b. MaybeT a -> (a -> MaybeT b) -> MaybeT b
    {
        let bmt_a_chirho = TyVarChirho(3571);
        let bmt_b_chirho = TyVarChirho(3572);
        let maybet_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("MaybeT".to_string())),
            Box::new(TyChirho::VarChirho(bmt_a_chirho)),
        );
        let maybet_b_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("MaybeT".to_string())),
            Box::new(TyChirho::VarChirho(bmt_b_chirho)),
        );
        env_chirho.bind_chirho(
            "bindMaybeT".to_string(),
            SchemeChirho {
                vars_chirho: vec![bmt_a_chirho, bmt_b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    maybet_a_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(bmt_a_chirho),
                            maybet_b_chirho.clone(),
                        ),
                        maybet_b_chirho,
                    ),
                ),
            },
        );
    }

    // -----------------------------------------------------------------------
    // WriterT operations (WriterT, runWriterT, runWriter, tell, returnWriterT,
    //   bindWriterT, execWriterT, execWriter)
    // -----------------------------------------------------------------------

    // WriterT :: forall w a. (a, w) -> WriterT w a
    {
        let wt_w_chirho = TyVarChirho(3580);
        let wt_a_chirho = TyVarChirho(3581);
        let writert_wa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WriterT".to_string())),
                Box::new(TyChirho::VarChirho(wt_w_chirho)),
            )),
            Box::new(TyChirho::VarChirho(wt_a_chirho)),
        );
        let tuple_aw_chirho = TyChirho::TupleChirho(vec![
            TyChirho::VarChirho(wt_a_chirho),
            TyChirho::VarChirho(wt_w_chirho),
        ]);
        env_chirho.bind_chirho(
            "WriterT".to_string(),
            SchemeChirho {
                vars_chirho: vec![wt_w_chirho, wt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(tuple_aw_chirho, writert_wa_chirho),
            },
        );
    }

    // runWriterT :: forall w a. WriterT w a -> (a, w)
    {
        let rwt_w_chirho = TyVarChirho(3582);
        let rwt_a_chirho = TyVarChirho(3583);
        let writert_wa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WriterT".to_string())),
                Box::new(TyChirho::VarChirho(rwt_w_chirho)),
            )),
            Box::new(TyChirho::VarChirho(rwt_a_chirho)),
        );
        let tuple_aw_chirho = TyChirho::TupleChirho(vec![
            TyChirho::VarChirho(rwt_a_chirho),
            TyChirho::VarChirho(rwt_w_chirho),
        ]);
        env_chirho.bind_chirho(
            "runWriterT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rwt_w_chirho, rwt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(writert_wa_chirho, tuple_aw_chirho),
            },
        );
    }

    // runWriter :: forall w a. WriterT w a -> (a, w)  (alias)
    {
        let rw_w_chirho = TyVarChirho(3584);
        let rw_a_chirho = TyVarChirho(3585);
        let writert_wa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WriterT".to_string())),
                Box::new(TyChirho::VarChirho(rw_w_chirho)),
            )),
            Box::new(TyChirho::VarChirho(rw_a_chirho)),
        );
        let tuple_aw_chirho = TyChirho::TupleChirho(vec![
            TyChirho::VarChirho(rw_a_chirho),
            TyChirho::VarChirho(rw_w_chirho),
        ]);
        env_chirho.bind_chirho(
            "runWriter".to_string(),
            SchemeChirho {
                vars_chirho: vec![rw_w_chirho, rw_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(writert_wa_chirho, tuple_aw_chirho),
            },
        );
    }

    // tell :: forall w. w -> WriterT w ()
    {
        let t_w_chirho = TyVarChirho(3586);
        let writert_w_unit_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WriterT".to_string())),
                Box::new(TyChirho::VarChirho(t_w_chirho)),
            )),
            Box::new(TyChirho::unit_chirho()),
        );
        env_chirho.bind_chirho(
            "tell".to_string(),
            SchemeChirho {
                vars_chirho: vec![t_w_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(t_w_chirho),
                    writert_w_unit_chirho,
                ),
            },
        );
    }

    // returnWriterT :: forall w a. a -> WriterT w a
    {
        let rwt_w_chirho = TyVarChirho(3587);
        let rwt_a_chirho = TyVarChirho(3588);
        let writert_wa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WriterT".to_string())),
                Box::new(TyChirho::VarChirho(rwt_w_chirho)),
            )),
            Box::new(TyChirho::VarChirho(rwt_a_chirho)),
        );
        env_chirho.bind_chirho(
            "returnWriterT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rwt_w_chirho, rwt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(rwt_a_chirho),
                    writert_wa_chirho,
                ),
            },
        );
    }

    // bindWriterT :: forall w a b. WriterT w a -> (a -> WriterT w b) -> WriterT w b
    {
        let bwt_w_chirho = TyVarChirho(3589);
        let bwt_a_chirho = TyVarChirho(3590);
        let bwt_b_chirho = TyVarChirho(3591);
        let writert_wa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WriterT".to_string())),
                Box::new(TyChirho::VarChirho(bwt_w_chirho)),
            )),
            Box::new(TyChirho::VarChirho(bwt_a_chirho)),
        );
        let writert_wb_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WriterT".to_string())),
                Box::new(TyChirho::VarChirho(bwt_w_chirho)),
            )),
            Box::new(TyChirho::VarChirho(bwt_b_chirho)),
        );
        env_chirho.bind_chirho(
            "bindWriterT".to_string(),
            SchemeChirho {
                vars_chirho: vec![bwt_w_chirho, bwt_a_chirho, bwt_b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    writert_wa_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(bwt_a_chirho),
                            writert_wb_chirho.clone(),
                        ),
                        writert_wb_chirho,
                    ),
                ),
            },
        );
    }

    // execWriterT :: forall w a. WriterT w a -> w
    {
        let ewt_w_chirho = TyVarChirho(3592);
        let ewt_a_chirho = TyVarChirho(3593);
        let writert_wa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WriterT".to_string())),
                Box::new(TyChirho::VarChirho(ewt_w_chirho)),
            )),
            Box::new(TyChirho::VarChirho(ewt_a_chirho)),
        );
        env_chirho.bind_chirho(
            "execWriterT".to_string(),
            SchemeChirho {
                vars_chirho: vec![ewt_w_chirho, ewt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    writert_wa_chirho,
                    TyChirho::VarChirho(ewt_w_chirho),
                ),
            },
        );
    }

    // execWriter :: forall w a. WriterT w a -> w  (alias)
    {
        let ew_w_chirho = TyVarChirho(3594);
        let ew_a_chirho = TyVarChirho(3595);
        let writert_wa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WriterT".to_string())),
                Box::new(TyChirho::VarChirho(ew_w_chirho)),
            )),
            Box::new(TyChirho::VarChirho(ew_a_chirho)),
        );
        env_chirho.bind_chirho(
            "execWriter".to_string(),
            SchemeChirho {
                vars_chirho: vec![ew_w_chirho, ew_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    writert_wa_chirho,
                    TyChirho::VarChirho(ew_w_chirho),
                ),
            },
        );
    }

    // -----------------------------------------------------------------------
    // Additional utility functions (repeat, cycle, fix, group, etc.)
    // -----------------------------------------------------------------------

    // repeat :: forall a. a -> [a]
    {
        let rep_a_chirho = TyVarChirho(4100);
        env_chirho.bind_chirho(
            "repeat".to_string(),
            SchemeChirho {
                vars_chirho: vec![rep_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(rep_a_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(rep_a_chirho))),
                ),
            },
        );
    }

    // cycle :: forall a. [a] -> [a]
    {
        let cyc_a_chirho = TyVarChirho(4101);
        env_chirho.bind_chirho(
            "cycle".to_string(),
            SchemeChirho {
                vars_chirho: vec![cyc_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cyc_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cyc_a_chirho))),
                ),
            },
        );
    }

    // fix :: forall a. (a -> a) -> a
    {
        let fix_a_chirho = TyVarChirho(4102);
        env_chirho.bind_chirho(
            "fix".to_string(),
            SchemeChirho {
                vars_chirho: vec![fix_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(fix_a_chirho),
                        TyChirho::VarChirho(fix_a_chirho),
                    ),
                    TyChirho::VarChirho(fix_a_chirho),
                ),
            },
        );
    }

    // group :: [Int] -> [[Int]]
    env_chirho.bind_chirho(
        "group".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                TyChirho::ListChirho(Box::new(
                    TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                )),
            ),
        },
    );

    // zipWith3 :: forall a b c d. (a -> b -> c -> d) -> [a] -> [b] -> [c] -> [d]
    {
        let zw3_a_chirho = TyVarChirho(4110);
        let zw3_b_chirho = TyVarChirho(4111);
        let zw3_c_chirho = TyVarChirho(4112);
        let zw3_d_chirho = TyVarChirho(4113);
        env_chirho.bind_chirho(
            "zipWith3".to_string(),
            SchemeChirho {
                vars_chirho: vec![zw3_a_chirho, zw3_b_chirho, zw3_c_chirho, zw3_d_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![
                                TyChirho::VarChirho(zw3_a_chirho),
                                TyChirho::VarChirho(zw3_b_chirho),
                                TyChirho::VarChirho(zw3_c_chirho),
                            ],
                            TyChirho::VarChirho(zw3_d_chirho),
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zw3_a_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zw3_b_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zw3_c_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zw3_d_chirho))),
                ),
            },
        );
    }

    // first :: forall a b c. (a -> b) -> (a, c) -> (b, c)
    {
        let fi_a_chirho = TyVarChirho(4120);
        let fi_b_chirho = TyVarChirho(4121);
        let fi_c_chirho = TyVarChirho(4122);
        env_chirho.bind_chirho(
            "first".to_string(),
            SchemeChirho {
                vars_chirho: vec![fi_a_chirho, fi_b_chirho, fi_c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(fi_a_chirho),
                            TyChirho::VarChirho(fi_b_chirho),
                        ),
                        TyChirho::TupleChirho(vec![
                            TyChirho::VarChirho(fi_a_chirho),
                            TyChirho::VarChirho(fi_c_chirho),
                        ]),
                    ],
                    TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(fi_b_chirho),
                        TyChirho::VarChirho(fi_c_chirho),
                    ]),
                ),
            },
        );
    }

    // second :: forall a b c. (b -> c) -> (a, b) -> (a, c)
    {
        let se_a_chirho = TyVarChirho(4123);
        let se_b_chirho = TyVarChirho(4124);
        let se_c_chirho = TyVarChirho(4125);
        env_chirho.bind_chirho(
            "second".to_string(),
            SchemeChirho {
                vars_chirho: vec![se_a_chirho, se_b_chirho, se_c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(se_b_chirho),
                            TyChirho::VarChirho(se_c_chirho),
                        ),
                        TyChirho::TupleChirho(vec![
                            TyChirho::VarChirho(se_a_chirho),
                            TyChirho::VarChirho(se_b_chirho),
                        ]),
                    ],
                    TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(se_a_chirho),
                        TyChirho::VarChirho(se_c_chirho),
                    ]),
                ),
            },
        );
    }

    // -----------------------------------------------------------------------
    // Exception handling
    // -----------------------------------------------------------------------

    // catch :: forall a. IO a -> (String -> IO a) -> IO a
    {
        let catch_a_chirho = TyVarChirho(4130);
        env_chirho.bind_chirho(
            "catch".to_string(),
            SchemeChirho {
                vars_chirho: vec![catch_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::io_chirho(TyChirho::VarChirho(catch_a_chirho)),
                        TyChirho::fun_chirho(
                            TyChirho::string_chirho(),
                            TyChirho::io_chirho(TyChirho::VarChirho(catch_a_chirho)),
                        ),
                    ],
                    TyChirho::io_chirho(TyChirho::VarChirho(catch_a_chirho)),
                ),
            },
        );
    }

    // throw :: forall a. String -> a
    {
        let throw_a_chirho = TyVarChirho(4131);
        env_chirho.bind_chirho(
            "throw".to_string(),
            SchemeChirho {
                vars_chirho: vec![throw_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::string_chirho(),
                    TyChirho::VarChirho(throw_a_chirho),
                ),
            },
        );
    }

    // try :: forall a. IO a -> IO (Either String a)
    {
        let try_a_chirho = TyVarChirho(4132);
        // Either String a = App (App (Con "Either") String) (Var a)
        let either_string_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Either".to_string())),
                Box::new(TyChirho::string_chirho()),
            )),
            Box::new(TyChirho::VarChirho(try_a_chirho)),
        );
        env_chirho.bind_chirho(
            "try".to_string(),
            SchemeChirho {
                vars_chirho: vec![try_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::io_chirho(TyChirho::VarChirho(try_a_chirho)),
                    TyChirho::io_chirho(either_string_a_chirho),
                ),
            },
        );
    }

    // throwIO :: forall a. String -> IO a
    {
        let throwio_a_chirho = TyVarChirho(4160);
        env_chirho.bind_chirho(
            "throwIO".to_string(),
            SchemeChirho {
                vars_chirho: vec![throwio_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::string_chirho(),
                    TyChirho::io_chirho(TyChirho::VarChirho(throwio_a_chirho)),
                ),
            },
        );
    }

    // bracket :: forall a b c. IO a -> (a -> IO b) -> (a -> IO c) -> IO c
    {
        let br_a_chirho = TyVarChirho(4161);
        let br_b_chirho = TyVarChirho(4162);
        let br_c_chirho = TyVarChirho(4163);
        env_chirho.bind_chirho(
            "bracket".to_string(),
            SchemeChirho {
                vars_chirho: vec![br_a_chirho, br_b_chirho, br_c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::io_chirho(TyChirho::VarChirho(br_a_chirho)),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(br_a_chirho),
                            TyChirho::io_chirho(TyChirho::VarChirho(br_b_chirho)),
                        ),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(br_a_chirho),
                            TyChirho::io_chirho(TyChirho::VarChirho(br_c_chirho)),
                        ),
                    ],
                    TyChirho::io_chirho(TyChirho::VarChirho(br_c_chirho)),
                ),
            },
        );
    }

    // finally :: forall a b. IO a -> IO b -> IO a
    {
        let fin_a_chirho = TyVarChirho(4164);
        let fin_b_chirho = TyVarChirho(4165);
        env_chirho.bind_chirho(
            "finally".to_string(),
            SchemeChirho {
                vars_chirho: vec![fin_a_chirho, fin_b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::io_chirho(TyChirho::VarChirho(fin_a_chirho)),
                        TyChirho::io_chirho(TyChirho::VarChirho(fin_b_chirho)),
                    ],
                    TyChirho::io_chirho(TyChirho::VarChirho(fin_a_chirho)),
                ),
            },
        );
    }

    // both :: forall a b. (a -> b) -> (a, a) -> (b, b)
    {
        let bo_a_chirho = TyVarChirho(4126);
        let bo_b_chirho = TyVarChirho(4127);
        env_chirho.bind_chirho(
            "both".to_string(),
            SchemeChirho {
                vars_chirho: vec![bo_a_chirho, bo_b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(bo_a_chirho),
                            TyChirho::VarChirho(bo_b_chirho),
                        ),
                        TyChirho::TupleChirho(vec![
                            TyChirho::VarChirho(bo_a_chirho),
                            TyChirho::VarChirho(bo_a_chirho),
                        ]),
                    ],
                    TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(bo_b_chirho),
                        TyChirho::VarChirho(bo_b_chirho),
                    ]),
                ),
            },
        );
    }

    // tails :: forall a. [a] -> [[a]]
    {
        let tl_a_chirho = TyVarChirho(4130);
        env_chirho.bind_chirho(
            "tails".to_string(),
            SchemeChirho {
                vars_chirho: vec![tl_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(tl_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::ListChirho(Box::new(
                        TyChirho::VarChirho(tl_a_chirho),
                    )))),
                ),
            },
        );
    }

    // inits :: forall a. [a] -> [[a]]
    {
        let in_a_chirho = TyVarChirho(4131);
        env_chirho.bind_chirho(
            "inits".to_string(),
            SchemeChirho {
                vars_chirho: vec![in_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(in_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::ListChirho(Box::new(
                        TyChirho::VarChirho(in_a_chirho),
                    )))),
                ),
            },
        );
    }

    // ── Library function type schemes (imported modules) ────────────────

    // coerce :: Coercible a b => a -> b
    {
        let a_chirho = TyVarChirho(7100);
        let b_chirho = TyVarChirho(7101);
        env_chirho.bind_chirho(
            "coerce".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    TyChirho::VarChirho(b_chirho),
                ),
            },
        );
    }

    // unsafeCoerce :: a -> b
    {
        let a_chirho = TyVarChirho(7110);
        let b_chirho = TyVarChirho(7111);
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![a_chirho, b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(a_chirho),
                TyChirho::VarChirho(b_chirho),
            ),
        };
        env_chirho.bind_chirho("unsafeCoerce".to_string(), scheme_chirho.clone());
        env_chirho.bind_chirho("unsafeCoerce#".to_string(), scheme_chirho);
    }

    // cast :: (Typeable a, Typeable b) => a -> Maybe b
    {
        let a_chirho = TyVarChirho(7120);
        let b_chirho = TyVarChirho(7121);
        env_chirho.bind_chirho(
            "cast".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(TyChirho::VarChirho(b_chirho)),
                    ),
                ),
            },
        );
    }

    // eqT :: (Typeable a, Typeable b) => Maybe (a :~: b)
    // Simplified: eqT :: Maybe a (since :~: is complex)
    {
        let a_chirho = TyVarChirho(7130);
        env_chirho.bind_chirho(
            "eqT".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(a_chirho)),
                ),
            },
        );
    }

    // typeOf :: Typeable a => a -> TypeRep
    {
        let a_chirho = TyVarChirho(7140);
        env_chirho.bind_chirho(
            "typeOf".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    TyChirho::ConChirho("TypeRep".to_string()),
                ),
            },
        );
    }

    // typeRep :: Typeable a => Proxy a -> TypeRep
    {
        let a_chirho = TyVarChirho(7150);
        env_chirho.bind_chirho(
            "typeRep".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Proxy".to_string())),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                    TyChirho::ConChirho("TypeRep".to_string()),
                ),
            },
        );
    }

    // Proxy :: Proxy a (constructor)
    {
        let a_chirho = TyVarChirho(7160);
        env_chirho.bind_chirho(
            "Proxy".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Proxy".to_string())),
                    Box::new(TyChirho::VarChirho(a_chirho)),
                ),
            },
        );
    }

    // oneShot :: (a -> b) -> a -> b
    {
        let a_chirho = TyVarChirho(7170);
        let b_chirho = TyVarChirho(7171);
        env_chirho.bind_chirho(
            "oneShot".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::VarChirho(b_chirho),
                    ),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::VarChirho(b_chirho),
                    ),
                ),
            },
        );
    }

    // comparing :: Ord b => (a -> b) -> a -> a -> Ordering
    {
        let a_chirho = TyVarChirho(7180);
        let b_chirho = TyVarChirho(7181);
        env_chirho.bind_chirho(
            "comparing".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(b_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(b_chirho)),
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::VarChirho(a_chirho),
                    ],
                    TyChirho::ConChirho("Ordering".to_string()),
                ),
            },
        );
    }

    // liftIO :: MonadIO m => IO a -> m a
    {
        let m_chirho = TyVarChirho(7190);
        let a_chirho = TyVarChirho(7191);
        env_chirho.bind_chirho(
            "liftIO".to_string(),
            SchemeChirho {
                vars_chirho: vec![m_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("IO".to_string())),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(m_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                ),
            },
        );
    }

    // on :: (b -> b -> c) -> (a -> b) -> a -> a -> c
    {
        let a_chirho = TyVarChirho(7200);
        let b_chirho = TyVarChirho(7201);
        let c_chirho = TyVarChirho(7202);
        env_chirho.bind_chirho(
            "on".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho, c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(vec![TyChirho::VarChirho(b_chirho), TyChirho::VarChirho(b_chirho)], TyChirho::VarChirho(c_chirho)),
                        TyChirho::fun_chirho(TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(b_chirho)),
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::VarChirho(a_chirho),
                    ],
                    TyChirho::VarChirho(c_chirho),
                ),
            },
        );
    }

    // fix :: (a -> a) -> a
    {
        let a_chirho = TyVarChirho(7210);
        env_chirho.bind_chirho(
            "fix".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(a_chirho)),
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );
    }

    // void :: Functor f => f a -> f ()
    {
        let f_chirho = TyVarChirho(7220);
        let a_chirho = TyVarChirho(7221);
        env_chirho.bind_chirho(
            "void".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho, a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Functor".to_string(),
                    ty_chirho: TyChirho::VarChirho(f_chirho),
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::TupleChirho(vec![])),
                    ),
                ),
            },
        );
    }

    // when :: Applicative f => Bool -> f () -> f ()
    {
        let f_chirho = TyVarChirho(7230);
        env_chirho.bind_chirho(
            "when".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::bool_chirho(),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(f_chirho)),
                            Box::new(TyChirho::TupleChirho(vec![])),
                        ),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::TupleChirho(vec![])),
                    ),
                ),
            },
        );
    }

    // unless :: Applicative f => Bool -> f () -> f ()
    {
        let f_chirho = TyVarChirho(7240);
        env_chirho.bind_chirho(
            "unless".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::bool_chirho(),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(f_chirho)),
                            Box::new(TyChirho::TupleChirho(vec![])),
                        ),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::TupleChirho(vec![])),
                    ),
                ),
            },
        );
    }

    // join :: Monad m => m (m a) -> m a
    {
        let m_chirho = TyVarChirho(7250);
        let a_chirho = TyVarChirho(7251);
        env_chirho.bind_chirho(
            "join".to_string(),
            SchemeChirho {
                vars_chirho: vec![m_chirho, a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(m_chirho),
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(m_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(m_chirho)),
                            Box::new(TyChirho::VarChirho(a_chirho)),
                        )),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(m_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                ),
            },
        );
    }

    // guard :: Alternative f => Bool -> f ()
    {
        let f_chirho = TyVarChirho(7260);
        env_chirho.bind_chirho(
            "guard".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::bool_chirho(),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::TupleChirho(vec![])),
                    ),
                ),
            },
        );
    }

    // forever :: Applicative f => f a -> f b
    {
        let f_chirho = TyVarChirho(7270);
        let a_chirho = TyVarChirho(7271);
        let b_chirho = TyVarChirho(7272);
        env_chirho.bind_chirho(
            "forever".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho, a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::VarChirho(b_chirho)),
                    ),
                ),
            },
        );
    }

    // Data.Function: (&) :: a -> (a -> b) -> b
    {
        let a_chirho = TyVarChirho(7280);
        let b_chirho = TyVarChirho(7281);
        env_chirho.bind_chirho(
            "&".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::fun_chirho(TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(b_chirho)),
                    ],
                    TyChirho::VarChirho(b_chirho),
                ),
            },
        );
    }

    // Semigroup: (<>) :: Semigroup a => a -> a -> a
    {
        let a_chirho = TyVarChirho(7290);
        env_chirho.bind_chirho(
            "<>".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Semigroup".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(a_chirho)],
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );
    }

    // mappend :: Monoid a => a -> a -> a (same as <>)
    {
        let a_chirho = TyVarChirho(7300);
        env_chirho.bind_chirho(
            "mappend".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monoid".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(a_chirho)],
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );
    }

    // mempty :: Monoid a => a
    {
        let a_chirho = TyVarChirho(7310);
        env_chirho.bind_chirho(
            "mempty".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monoid".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                }],
                ty_chirho: TyChirho::VarChirho(a_chirho),
            },
        );
    }

    // mconcat :: Monoid a => [a] -> a
    {
        let a_chirho = TyVarChirho(7320);
        env_chirho.bind_chirho(
            "mconcat".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monoid".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );
    }

    // getDual :: Dual a -> a, Dual :: a -> Dual a
    {
        let a_chirho = TyVarChirho(7330);
        env_chirho.bind_chirho(
            "Dual".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Dual".to_string())),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                ),
            },
        );
        env_chirho.bind_chirho(
            "getDual".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Dual".to_string())),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );
    }

    // nub :: Eq a => [a] -> [a]
    {
        let a_chirho = TyVarChirho(7340);
        env_chirho.bind_chirho(
            "nub".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                ),
            },
        );
    }

    // group :: Eq a => [a] -> [[a]]
    {
        let a_chirho = TyVarChirho(7350);
        env_chirho.bind_chirho(
            "group".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))))),
                ),
            },
        );
    }

    // forM :: Monad m => [a] -> (a -> m b) -> m [b]  (simplified as mapM with args swapped)
    {
        let m_chirho = TyVarChirho(7360);
        let a_chirho = TyVarChirho(7361);
        let b_chirho = TyVarChirho(7362);
        env_chirho.bind_chirho(
            "forM".to_string(),
            SchemeChirho {
                vars_chirho: vec![m_chirho, a_chirho, b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(m_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::AppChirho(Box::new(TyChirho::VarChirho(m_chirho)), Box::new(TyChirho::VarChirho(b_chirho))),
                        ),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(m_chirho)),
                        Box::new(TyChirho::ListChirho(Box::new(TyChirho::VarChirho(b_chirho)))),
                    ),
                ),
            },
        );
    }

    // forM_ :: Monad m => [a] -> (a -> m b) -> m ()
    {
        let m_chirho = TyVarChirho(7370);
        let a_chirho = TyVarChirho(7371);
        let b_chirho = TyVarChirho(7372);
        env_chirho.bind_chirho(
            "forM_".to_string(),
            SchemeChirho {
                vars_chirho: vec![m_chirho, a_chirho, b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(m_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::AppChirho(Box::new(TyChirho::VarChirho(m_chirho)), Box::new(TyChirho::VarChirho(b_chirho))),
                        ),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(m_chirho)),
                        Box::new(TyChirho::TupleChirho(vec![])),
                    ),
                ),
            },
        );
    }

    // Data.Coerce: Coercible (type class — accept as constraint)
    // withDict :: (Class => r) -> Dict Class -> r (simplified)

    // Data.Type.Equality: (:~:) constructor — Refl :: a :~: a
    {
        let a_chirho = TyVarChirho(7380);
        env_chirho.bind_chirho(
            "Refl".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho(":~:".to_string())),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    )),
                    Box::new(TyChirho::VarChirho(a_chirho)),
                ),
            },
        );
    }

    // realToFrac :: (Real a, Fractional b) => a -> b
    {
        let a_chirho = TyVarChirho(7390);
        let b_chirho = TyVarChirho(7391);
        env_chirho.bind_chirho(
            "realToFrac".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![
                    SchemePredChirho { class_name_chirho: "Real".to_string(), ty_chirho: TyChirho::VarChirho(a_chirho) },
                    SchemePredChirho { class_name_chirho: "Fractional".to_string(), ty_chirho: TyChirho::VarChirho(b_chirho) },
                ],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::VarChirho(b_chirho)),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // fromIntegral :: (Integral a, Num b) => a -> b
    {
        let a_chirho = TyVarChirho(7392);
        let b_chirho = TyVarChirho(7393);
        env_chirho.bind_chirho(
            "fromIntegral".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![
                    SchemePredChirho { class_name_chirho: "Integral".to_string(), ty_chirho: TyChirho::VarChirho(a_chirho) },
                    SchemePredChirho { class_name_chirho: "Num".to_string(), ty_chirho: TyChirho::VarChirho(b_chirho) },
                ],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::VarChirho(b_chirho)),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // toInteger :: Integral a => a -> Integer
    {
        let a_chirho = TyVarChirho(7394);
        env_chirho.bind_chirho(
            "toInteger".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![
                    SchemePredChirho { class_name_chirho: "Integral".to_string(), ty_chirho: TyChirho::VarChirho(a_chirho) },
                ],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::ConChirho("Integer".to_string())),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // toRational :: Real a => a -> Rational
    {
        let a_chirho = TyVarChirho(7395);
        env_chirho.bind_chirho(
            "toRational".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![
                    SchemePredChirho { class_name_chirho: "Real".to_string(), ty_chirho: TyChirho::VarChirho(a_chirho) },
                ],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::ConChirho("Rational".to_string())),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // sequence/sequence_/mapM/mapM_ — already defined with list-specific types earlier in seed_builtins

    // error :: forall a. [Char] -> a (already exists, but ensure errorWithoutStackTrace too)
    {
        let a_chirho = TyVarChirho(7410);
        env_chirho.bind_chirho(
            "errorWithoutStackTrace".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::ListChirho(Box::new(TyChirho::char_chirho()))),
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // Data.Coerce: coerce :: Coercible a b => a -> b
    // (already have coerce :: a -> b, add with Coercible constraint)

    // Integral class methods: div, mod, divMod, quot, rem, quotRem
    for (name_chirho, _) in &[
        ("div", ()), ("mod", ()), ("quot", ()), ("rem", ()),
    ] {
        let a_chirho = TyVarChirho(7411);
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![
                    SchemePredChirho { class_name_chirho: "Integral".to_string(), ty_chirho: TyChirho::VarChirho(a_chirho) },
                ],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::VarChirho(a_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                        MultChirho::ManyChirho,
                    )),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // divMod, quotRem :: Integral a => a -> a -> (a, a)
    for name_chirho in &["divMod", "quotRem"] {
        let a_chirho = TyVarChirho(7412);
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![
                    SchemePredChirho { class_name_chirho: "Integral".to_string(), ty_chirho: TyChirho::VarChirho(a_chirho) },
                ],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::VarChirho(a_chirho)),
                        Box::new(TyChirho::TupleChirho(vec![
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::VarChirho(a_chirho),
                        ])),
                        MultChirho::ManyChirho,
                    )),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // Data.IORef (already covered in runtime, ensure newIORef/readIORef/writeIORef/modifyIORef types)
    // IORef a constructor
    {
        let a_chirho = TyVarChirho(7413);
        env_chirho.bind_chirho(
            "IORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("IORef".to_string())),
                    Box::new(TyChirho::VarChirho(a_chirho)),
                ),
            },
        );
    }

    // Data.STRef: STRef s a
    {
        let s_chirho = TyVarChirho(7414);
        let a_chirho = TyVarChirho(7415);
        env_chirho.bind_chirho(
            "STRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![s_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("STRef".to_string())),
                        Box::new(TyChirho::VarChirho(s_chirho)),
                    )),
                    Box::new(TyChirho::VarChirho(a_chirho)),
                ),
            },
        );
    }

    // ── Additional missing Prelude / Data.List type schemes ──

    // splitAt :: forall a. Int -> [a] -> ([a], [a])
    {
        let a_chirho = TyVarChirho(7500);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "splitAt".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::int_chirho(), list_a_chirho.clone()],
                    TyChirho::TupleChirho(vec![list_a_chirho.clone(), list_a_chirho]),
                ),
            },
        );
    }

    // and :: [Bool] -> Bool
    env_chirho.bind_chirho(
        "and".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::bool_chirho())),
                TyChirho::bool_chirho(),
            ),
        },
    );

    // or :: [Bool] -> Bool
    env_chirho.bind_chirho(
        "or".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::bool_chirho())),
                TyChirho::bool_chirho(),
            ),
        },
    );

    // sortOn :: forall a b. Ord b => (a -> b) -> [a] -> [a]
    {
        let a_chirho = TyVarChirho(7501);
        let b_chirho = TyVarChirho(7502);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "sortOn".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(b_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(b_chirho)),
                        list_a_chirho.clone(),
                    ],
                    list_a_chirho,
                ),
            },
        );
    }

    // isInfixOf :: forall a. Eq a => [a] -> [a] -> Bool
    {
        let a_chirho = TyVarChirho(7503);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "isInfixOf".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![list_a_chirho.clone(), list_a_chirho],
                    TyChirho::bool_chirho(),
                ),
            },
        );
    }

    // stripPrefix :: forall a. Eq a => [a] -> [a] -> Maybe [a]
    {
        let a_chirho = TyVarChirho(7504);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "stripPrefix".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![list_a_chirho.clone(), list_a_chirho.clone()],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(list_a_chirho),
                    ),
                ),
            },
        );
    }

    // findIndex :: forall a. (a -> Bool) -> [a] -> Maybe Int
    {
        let a_chirho = TyVarChirho(7505);
        env_chirho.bind_chirho(
            "findIndex".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(TyChirho::VarChirho(a_chirho), TyChirho::bool_chirho()),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(TyChirho::int_chirho()),
                    ),
                ),
            },
        );
    }

    // elemIndex :: forall a. Eq a => a -> [a] -> Maybe Int
    {
        let a_chirho = TyVarChirho(7506);
        env_chirho.bind_chirho(
            "elemIndex".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(TyChirho::int_chirho()),
                    ),
                ),
            },
        );
    }

    // scanl1 :: forall a. (a -> a -> a) -> [a] -> [a]
    {
        let a_chirho = TyVarChirho(7507);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "scanl1".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(a_chirho)],
                            TyChirho::VarChirho(a_chirho),
                        ),
                        list_a_chirho.clone(),
                    ],
                    list_a_chirho,
                ),
            },
        );
    }

    // scanr :: forall a b. (a -> b -> b) -> b -> [a] -> [b]
    {
        let a_chirho = TyVarChirho(7508);
        let b_chirho = TyVarChirho(7509);
        env_chirho.bind_chirho(
            "scanr".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(b_chirho)],
                            TyChirho::VarChirho(b_chirho),
                        ),
                        TyChirho::VarChirho(b_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(b_chirho))),
                ),
            },
        );
    }

    // scanr1 :: forall a. (a -> a -> a) -> [a] -> [a]
    {
        let a_chirho = TyVarChirho(7510);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "scanr1".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(a_chirho)],
                            TyChirho::VarChirho(a_chirho),
                        ),
                        list_a_chirho.clone(),
                    ],
                    list_a_chirho,
                ),
            },
        );
    }

    // unfoldr :: forall a b. (b -> Maybe (a, b)) -> b -> [a]
    {
        let a_chirho = TyVarChirho(7511);
        let b_chirho = TyVarChirho(7512);
        env_chirho.bind_chirho(
            "unfoldr".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(b_chirho),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                                Box::new(TyChirho::TupleChirho(vec![
                                    TyChirho::VarChirho(a_chirho),
                                    TyChirho::VarChirho(b_chirho),
                                ])),
                            ),
                        ),
                        TyChirho::VarChirho(b_chirho),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                ),
            },
        );
    }

    // transpose :: forall a. [[a]] -> [[a]]
    {
        let a_chirho = TyVarChirho(7513);
        let list_list_a_chirho = TyChirho::ListChirho(Box::new(
            TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
        ));
        env_chirho.bind_chirho(
            "transpose".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(list_list_a_chirho.clone(), list_list_a_chirho),
            },
        );
    }

    // stdin, stdout, stderr :: Handle
    for handle_name_chirho in &["stdin", "stdout", "stderr"] {
        env_chirho.bind_chirho(
            handle_name_chirho.to_string(),
            SchemeChirho::mono_chirho(TyChirho::ConChirho("Handle".to_string())),
        );
    }

    // hFlush :: Handle -> IO ()
    env_chirho.bind_chirho(
        "hFlush".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho("Handle".to_string()),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // hSetBuffering :: Handle -> BufferMode -> IO ()
    env_chirho.bind_chirho(
        "hSetBuffering".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::ConChirho("Handle".to_string()),
                TyChirho::ConChirho("BufferMode".to_string()),
            ],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // hSetEncoding :: Handle -> TextEncoding -> IO ()
    env_chirho.bind_chirho(
        "hSetEncoding".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::ConChirho("Handle".to_string()),
                TyChirho::ConChirho("TextEncoding".to_string()),
            ],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // hPutStr :: Handle -> String -> IO ()
    env_chirho.bind_chirho(
        "hPutStr".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::ConChirho("Handle".to_string()), TyChirho::string_chirho()],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // hPutStrLn :: Handle -> String -> IO ()
    env_chirho.bind_chirho(
        "hPutStrLn".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::ConChirho("Handle".to_string()), TyChirho::string_chirho()],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // hClose :: Handle -> IO ()
    env_chirho.bind_chirho(
        "hClose".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho("Handle".to_string()),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // hGetContents :: Handle -> IO String
    env_chirho.bind_chirho(
        "hGetContents".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho("Handle".to_string()),
            TyChirho::io_chirho(TyChirho::string_chirho()),
        )),
    );

    // openFile :: FilePath -> IOMode -> IO Handle
    env_chirho.bind_chirho(
        "openFile".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::string_chirho(), TyChirho::ConChirho("IOMode".to_string())],
            TyChirho::io_chirho(TyChirho::ConChirho("Handle".to_string())),
        )),
    );

    // Data.IORef functions with type schemes
    // readIORef :: forall a. IORef a -> IO a
    {
        let a_chirho = TyVarChirho(7520);
        let ioref_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IORef".to_string())),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        env_chirho.bind_chirho(
            "readIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(ioref_a_chirho, TyChirho::io_chirho(TyChirho::VarChirho(a_chirho))),
            },
        );
    }

    // writeIORef :: forall a. IORef a -> a -> IO ()
    {
        let a_chirho = TyVarChirho(7521);
        let ioref_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IORef".to_string())),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        env_chirho.bind_chirho(
            "writeIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![ioref_a_chirho, TyChirho::VarChirho(a_chirho)],
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // modifyIORef :: forall a. IORef a -> (a -> a) -> IO ()
    {
        let a_chirho = TyVarChirho(7522);
        let ioref_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IORef".to_string())),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        env_chirho.bind_chirho(
            "modifyIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        ioref_a_chirho,
                        TyChirho::fun_chirho(TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(a_chirho)),
                    ],
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // newIORef :: forall a. a -> IO (IORef a)
    {
        let a_chirho = TyVarChirho(7523);
        let ioref_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IORef".to_string())),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        env_chirho.bind_chirho(
            "newIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    TyChirho::io_chirho(ioref_a_chirho),
                ),
            },
        );
    }

    // mapAccumL :: forall a b c. (a -> b -> (a, c)) -> a -> [b] -> (a, [c])
    {
        let a_chirho = TyVarChirho(7530);
        let b_chirho = TyVarChirho(7531);
        let c_chirho = TyVarChirho(7532);
        env_chirho.bind_chirho(
            "mapAccumL".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho, c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(b_chirho)],
                            TyChirho::TupleChirho(vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(c_chirho)]),
                        ),
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(b_chirho))),
                    ],
                    TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(c_chirho))),
                    ]),
                ),
            },
        );
    }

    // mapAccumR :: forall a b c. (a -> b -> (a, c)) -> a -> [b] -> (a, [c])
    {
        let a_chirho = TyVarChirho(7533);
        let b_chirho = TyVarChirho(7534);
        let c_chirho = TyVarChirho(7535);
        env_chirho.bind_chirho(
            "mapAccumR".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho, c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(b_chirho)],
                            TyChirho::TupleChirho(vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(c_chirho)]),
                        ),
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(b_chirho))),
                    ],
                    TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(c_chirho))),
                    ]),
                ),
            },
        );
    }
}

// ---------------------------------------------------------------------------
// Literal type inference
// ---------------------------------------------------------------------------

fn infer_lit_chirho(lit_chirho: &LitChirho) -> TyChirho {
    match lit_chirho {
        LitChirho::IntChirho(..) => TyChirho::int_chirho(),
        LitChirho::FloatChirho(..) => TyChirho::double_chirho(),
        LitChirho::CharChirho(..) => TyChirho::char_chirho(),
        LitChirho::StringChirho(..) => TyChirho::string_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Run type inference on a module. This is the main entry point.
pub fn infer_module_chirho(module_chirho: &ModuleChirho) -> InferResultChirho {
    infer_module_with_imports_chirho(module_chirho, &HashMap::new())
}

/// Run type inference on a module with pre-seeded type schemes from imported
/// modules. Each entry maps a name (e.g. `"add1"`) to the type scheme that
/// was inferred in the exporting module.
pub fn infer_module_with_imports_chirho(
    module_chirho: &ModuleChirho,
    imported_types_chirho: &HashMap<String, SchemeChirho>,
) -> InferResultChirho {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    // Seed the type environment with imported type schemes.
    for (name_chirho, scheme_chirho) in imported_types_chirho {
        ctx_chirho.env_chirho.bind_chirho(name_chirho.clone(), scheme_chirho.clone());
    }
    let subst_chirho = ctx_chirho.infer_module_chirho(module_chirho);
    ctx_chirho.check_deferred_preds_chirho(&subst_chirho);
    let mut result_chirho = ctx_chirho.finish_chirho();
    result_chirho.subst_chirho = subst_chirho;
    result_chirho
}

/// Match a type family LHS pattern against a concrete type argument.
/// Type variables in the pattern bind to the corresponding argument types.
/// Type constructors must match exactly.
fn match_type_pattern_chirho(
    pat_chirho: &TyChirho,
    arg_chirho: &TyChirho,
    bindings_chirho: &mut HashMap<String, TyChirho>,
) -> bool {
    match pat_chirho {
        // A type variable in the pattern matches anything
        TyChirho::VarChirho(tv_chirho) => {
            let var_name_chirho = format!("tv{}", tv_chirho.0);
            if let Some(existing_chirho) = bindings_chirho.get(&var_name_chirho) {
                existing_chirho == arg_chirho
            } else {
                bindings_chirho.insert(var_name_chirho, arg_chirho.clone());
                true
            }
        }
        // A constructor must match the same constructor
        TyChirho::ConChirho(name_chirho) => {
            matches!(arg_chirho, TyChirho::ConChirho(arg_name_chirho) if arg_name_chirho == name_chirho)
        }
        // Type application: both sides must be apps with matching structure
        TyChirho::AppChirho(f_chirho, a_chirho) => {
            if let TyChirho::AppChirho(af_chirho, aa_chirho) = arg_chirho {
                match_type_pattern_chirho(f_chirho, af_chirho, bindings_chirho)
                    && match_type_pattern_chirho(a_chirho, aa_chirho, bindings_chirho)
            } else {
                false
            }
        }
        TyChirho::ListChirho(inner_chirho) => {
            if let TyChirho::ListChirho(arg_inner_chirho) = arg_chirho {
                match_type_pattern_chirho(inner_chirho, arg_inner_chirho, bindings_chirho)
            } else {
                false
            }
        }
        TyChirho::TupleChirho(elems_chirho) => {
            if let TyChirho::TupleChirho(arg_elems_chirho) = arg_chirho {
                if elems_chirho.len() != arg_elems_chirho.len() {
                    return false;
                }
                elems_chirho.iter().zip(arg_elems_chirho.iter()).all(
                    |(p_chirho, a_chirho)| {
                        match_type_pattern_chirho(p_chirho, a_chirho, bindings_chirho)
                    },
                )
            } else {
                false
            }
        }
        TyChirho::FunChirho(a_chirho, r_chirho, _) => {
            if let TyChirho::FunChirho(aa_chirho, ar_chirho, _) = arg_chirho {
                match_type_pattern_chirho(a_chirho, aa_chirho, bindings_chirho)
                    && match_type_pattern_chirho(r_chirho, ar_chirho, bindings_chirho)
            } else {
                false
            }
        }
        // Int, Bool, Char literals — exact match only
        _ => pat_chirho == arg_chirho,
    }
}

/// Substitute type variables in a type family equation RHS using the
/// bindings collected during pattern matching.
fn substitute_type_vars_chirho(
    ty_chirho: &TyChirho,
    bindings_chirho: &HashMap<String, TyChirho>,
) -> TyChirho {
    match ty_chirho {
        TyChirho::VarChirho(tv_chirho) => {
            let var_name_chirho = format!("tv{}", tv_chirho.0);
            bindings_chirho
                .get(&var_name_chirho)
                .cloned()
                .unwrap_or_else(|| ty_chirho.clone())
        }
        TyChirho::AppChirho(f_chirho, a_chirho) => TyChirho::AppChirho(
            Box::new(substitute_type_vars_chirho(f_chirho, bindings_chirho)),
            Box::new(substitute_type_vars_chirho(a_chirho, bindings_chirho)),
        ),
        TyChirho::FunChirho(a_chirho, r_chirho, _) => TyChirho::FunChirho(
            Box::new(substitute_type_vars_chirho(a_chirho, bindings_chirho)),
            Box::new(substitute_type_vars_chirho(r_chirho, bindings_chirho)),
         MultChirho::ManyChirho,),
        TyChirho::ListChirho(inner_chirho) => TyChirho::ListChirho(Box::new(
            substitute_type_vars_chirho(inner_chirho, bindings_chirho),
        )),
        TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
            elems_chirho
                .iter()
                .map(|e_chirho| substitute_type_vars_chirho(e_chirho, bindings_chirho))
                .collect(),
        ),
        _ => ty_chirho.clone(),
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
    use haskelujah_ast_chirho::expr_chirho::AltChirho;
    use haskelujah_ast_chirho::module_chirho::ModuleChirho;
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    use haskelujah_ast_chirho::pat_chirho::PatFieldChirho;

    fn dummy_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    #[test]
    fn infer_literal_int_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO));
        let (_s_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        assert_eq!(ty_chirho, TyChirho::int_chirho());
    }

    #[test]
    fn infer_literal_string_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::LitChirho(LitChirho::StringChirho("hello".to_string(), SpanChirho::DUMMY_CHIRHO));
        let (_s_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        assert_eq!(ty_chirho, TyChirho::string_chirho());
    }

    #[test]
    fn infer_identity_function_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // \x -> x
        let expr_chirho = ExprChirho::LamChirho {
            pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
            body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);

        // Should be `a -> a` for some variable a
        match &final_ty_chirho {
            TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
                assert_eq!(arg_chirho, result_chirho, "identity should have a -> a");
            }
            other_chirho => panic!("expected function type, got {other_chirho}"),
        }
    }

    #[test]
    fn infer_application_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // not True
        let expr_chirho = ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("not"))),
            arg_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(final_ty_chirho, TyChirho::bool_chirho());
    }

    #[test]
    fn infer_if_expression_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // if True then 1 else 2
        let expr_chirho = ExprChirho::IfChirho {
            cond_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
            then_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO))),
            else_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(final_ty_chirho, TyChirho::int_chirho());
    }

    #[test]
    fn infer_type_mismatch_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // if 42 then 1 else 2  (condition is Int, not Bool)
        let expr_chirho = ExprChirho::IfChirho {
            cond_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO))),
            then_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO))),
            else_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let _ = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let result_chirho = ctx_chirho.finish_chirho();
        assert!(
            result_chirho.diagnostics_chirho.has_errors_chirho(),
            "should report type mismatch for Int condition"
        );
    }

    #[test]
    fn infer_module_data_and_function_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("Color"),
                    type_vars_chirho: vec![],
                    constructors_chirho: vec![
                        ConDeclChirho::OrdinaryChirho {
                            name_chirho: dummy_name_chirho("Red"),
                            fields_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("f"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("x"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "module inference should succeed without errors"
        );

        // f should have type a -> a (identity)
        let f_scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("f")
            .expect("f should be in environment");
        match &f_scheme_chirho.ty_chirho {
            TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
                assert_eq!(
                    arg_chirho, result_chirho,
                    "f should have type a -> a"
                );
            }
            other_chirho => panic!("expected f to have function type, got {other_chirho}"),
        }

        // Red should be in environment as Color
        let red_scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("Red")
            .expect("Red constructor should be in environment");
        assert_eq!(
            red_scheme_chirho.ty_chirho,
            TyChirho::ConChirho("Color".to_string())
        );
    }

    #[test]
    fn infer_let_generalization_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // let id = \x -> x in id 42
        let expr_chirho = ExprChirho::LetChirho {
            binds_chirho: vec![
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("myId"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("x"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            body_chirho: Box::new(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("myId"))),
                arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(
            final_ty_chirho,
            TyChirho::int_chirho(),
            "let id = \\x -> x in id 42 should have type Int"
        );
    }

    #[test]
    fn infer_tuple_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // (1, True)
        let expr_chirho = ExprChirho::TupleChirho {
            elements_chirho: vec![
                ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO)),
                ExprChirho::ConChirho(dummy_name_chirho("True")),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(
            final_ty_chirho,
            TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::bool_chirho()])
        );
    }

    #[test]
    fn infer_list_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // [1, 2, 3]
        let expr_chirho = ExprChirho::ListChirho {
            elements_chirho: vec![
                ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO)),
                ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO)),
                ExprChirho::LitChirho(LitChirho::IntChirho(3, SpanChirho::DUMMY_CHIRHO)),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(
            final_ty_chirho,
            TyChirho::ListChirho(Box::new(TyChirho::int_chirho()))
        );
    }

    // -----------------------------------------------------------------------
    // Typeclass constraint tests
    // -----------------------------------------------------------------------

    #[test]
    fn overloaded_op_defers_predicate_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::LamChirho {
            pats_chirho: vec![
                PatChirho::VarChirho(dummy_name_chirho("x")),
                PatChirho::VarChirho(dummy_name_chirho("y")),
            ],
            body_chirho: Box::new(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("+"))),
                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("y"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (_subst_chirho, _ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        assert!(!ctx_chirho.deferred_preds_chirho.is_empty());
        assert_eq!(ctx_chirho.deferred_preds_chirho[0].0.class_name_chirho, "Num");
    }

    #[test]
    fn plus_on_ints_satisfies_num_chirho() {
        // f x y = x + y  applied to Int arguments should succeed
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("PlusInts"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("addChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![
                        PatChirho::VarChirho(dummy_name_chirho("x")),
                        PatChirho::VarChirho(dummy_name_chirho("y")),
                    ],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("+"))),
                            arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("y"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "addChirho should infer without errors (Num constraint stays polymorphic): {:?}",
            result_chirho.diagnostics_chirho
        );

        // addChirho should have a Num constraint in its scheme
        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("addChirho")
            .expect("addChirho should be in environment");
        assert!(
            !scheme_chirho.preds_chirho.is_empty(),
            "addChirho should have Num predicate, got: {scheme_chirho}"
        );
        assert_eq!(scheme_chirho.preds_chirho[0].class_name_chirho, "Num");
    }

    #[test]
    fn eq_on_ints_satisfies_constraint_chirho() {
        // eqChirho x y = x == y
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("EqInts"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("eqChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![
                        PatChirho::VarChirho(dummy_name_chirho("x")),
                        PatChirho::VarChirho(dummy_name_chirho("y")),
                    ],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("=="))),
                            arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("y"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "eqChirho should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );

        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("eqChirho")
            .expect("eqChirho should be in environment");
        assert!(
            !scheme_chirho.preds_chirho.is_empty(),
            "eqChirho should have Eq predicate, got: {scheme_chirho}"
        );
        assert_eq!(scheme_chirho.preds_chirho[0].class_name_chirho, "Eq");
    }

    #[test]
    fn class_env_is_seeded_chirho() {
        let result_chirho = infer_module_chirho(&ModuleChirho {
            name_chirho: dummy_name_chirho("Empty"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        });

        assert!(result_chirho.class_env_chirho.has_class_chirho("Eq"));
        assert!(result_chirho.class_env_chirho.has_class_chirho("Ord"));
        assert!(result_chirho.class_env_chirho.has_class_chirho("Show"));
        assert!(result_chirho.class_env_chirho.has_class_chirho("Num"));
        assert!(result_chirho.class_env_chirho.has_class_chirho("Functor"));
    }

    /// Test: case expression with infix cons pattern `(x:xs)` binds both vars.
    /// Haskell: `headChirho xs = case xs of { (y:ys) -> y }`
    #[test]
    fn infer_case_infix_con_pattern_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("InfixConPat"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("headChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("xs"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(ExprChirho::VarChirho(
                            dummy_name_chirho("xs"),
                        )),
                        alts_chirho: vec![AltChirho {
                            pat_chirho: PatChirho::InfixConChirho {
                                left_chirho: Box::new(PatChirho::VarChirho(
                                    dummy_name_chirho("y"),
                                )),
                                op_chirho: dummy_name_chirho(":"),
                                right_chirho: Box::new(PatChirho::VarChirho(
                                    dummy_name_chirho("ys"),
                                )),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                                dummy_name_chirho("y"),
                            )),
                            where_binds_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "headChirho with infix con pattern should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );
        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("headChirho")
            .expect("headChirho should be in environment");
        // headChirho :: a -> b  (since we don't yet unify cons pattern args with list)
        // The key check: no errors — `y` was bound by bind_pat_chirho
        assert!(!format!("{scheme_chirho}").is_empty());
    }

    /// Test: case expression with list pattern `[a, b]` binds elements.
    /// Haskell: `sumTwoChirho xs = case xs of { [a, b] -> a }`
    #[test]
    fn infer_case_list_pattern_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("ListPat"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("sumTwoChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("xs"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(ExprChirho::VarChirho(
                            dummy_name_chirho("xs"),
                        )),
                        alts_chirho: vec![AltChirho {
                            pat_chirho: PatChirho::ListChirho {
                                elements_chirho: vec![
                                    PatChirho::VarChirho(dummy_name_chirho("a")),
                                    PatChirho::VarChirho(dummy_name_chirho("b")),
                                ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                                dummy_name_chirho("a"),
                            )),
                            where_binds_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "sumTwoChirho with list pattern should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );
        result_chirho
            .env_chirho
            .lookup_chirho("sumTwoChirho")
            .expect("sumTwoChirho should be in environment");
    }

    /// Test: case expression with record pattern `Foo { bar = x }` binds x.
    /// Haskell: `getBarChirho v = case v of { Foo { bar = x } -> x }`
    #[test]
    fn infer_case_record_pattern_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("RecPat"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("getBarChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("v"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(ExprChirho::VarChirho(
                            dummy_name_chirho("v"),
                        )),
                        alts_chirho: vec![AltChirho {
                            pat_chirho: PatChirho::RecordChirho { has_wildcard_chirho: false,
                                con_chirho: dummy_name_chirho("Foo"),
                                fields_chirho: vec![PatFieldChirho {
                                    name_chirho: dummy_name_chirho("bar"),
                                    pattern_chirho: PatChirho::VarChirho(
                                        dummy_name_chirho("x"),
                                    ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                                dummy_name_chirho("x"),
                            )),
                            where_binds_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "getBarChirho with record pattern should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );
        result_chirho
            .env_chirho
            .lookup_chirho("getBarChirho")
            .expect("getBarChirho should be in environment");
    }

    /// Test: negated literal pattern `(-1)` doesn't introduce new bindings.
    /// Haskell: `isNegOneChirho x = case x of { (-1) -> 1; _ -> 0 }`
    #[test]
    fn infer_case_neg_pattern_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("NegPat"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("isNegOneChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(ExprChirho::VarChirho(
                            dummy_name_chirho("x"),
                        )),
                        alts_chirho: vec![
                            AltChirho {
                                pat_chirho: PatChirho::NegChirho {
                                    lit_chirho: LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                                },
                                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                                    LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO),
                                )),
                                where_binds_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            AltChirho {
                                pat_chirho: PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO),
                                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                                    LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO),
                                )),
                                where_binds_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                        ],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "isNegOneChirho with neg pattern should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );
        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("isNegOneChirho")
            .expect("isNegOneChirho should be in environment");
        // isNegOneChirho :: a -> Int (parameter type stays polymorphic)
        // Just verify it's a function returning Int
        if let TyChirho::FunChirho(_, ref ret_chirho, _) = scheme_chirho.ty_chirho {
            assert_eq!(**ret_chirho, TyChirho::int_chirho());
        } else {
            panic!("expected function type, got: {}", scheme_chirho.ty_chirho);
        }
    }

    // -----------------------------------------------------------------------
    // Class, instance, type sig, and newtype tests
    // -----------------------------------------------------------------------

    /// Test: user-defined class declaration registers in class env.
    /// Haskell: `class MyEq a where myEq :: a -> a -> Bool`
    #[test]
    fn infer_user_class_decl_chirho() {
        use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;

        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("UserClass"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: dummy_name_chirho("MyEqChirho"),
                type_vars_chirho: vec![dummy_name_chirho("a").into()],
                methods_chirho: vec![ClassMethodChirho {
                    name_chirho: dummy_name_chirho("myEqChirho"),
                    ty_chirho: TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::ConChirho(
                                dummy_name_chirho("Bool"),
                            )),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "User class decl should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
        // Class should be registered
        assert!(
            result_chirho.class_env_chirho.has_class_chirho("MyEqChirho"),
            "MyEqChirho should be in class env"
        );
        // Method should be in the type environment
        assert!(
            result_chirho.env_chirho.lookup_chirho("myEqChirho").is_some(),
            "myEqChirho method should be in type env"
        );
    }

    /// Test: user-defined class with superclass context.
    /// Haskell: `class Eq a => Ord a where compare :: a -> a -> Ordering`
    #[test]
    fn infer_user_class_with_superclass_chirho() {
        use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;
        use haskelujah_ast_chirho::ty_chirho::ConstraintChirho as AstConstraintChirho;

        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("SuperClass"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::ClassDeclChirho {
                context_chirho: vec![AstConstraintChirho {
                    class_chirho: dummy_name_chirho("Eq"),
                    args_chirho: vec![TypeChirho::VarChirho(dummy_name_chirho("a"))],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                name_chirho: dummy_name_chirho("MyOrdChirho"),
                type_vars_chirho: vec![dummy_name_chirho("a").into()],
                methods_chirho: vec![ClassMethodChirho {
                    name_chirho: dummy_name_chirho("myCompareChirho"),
                    ty_chirho: TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::ConChirho(
                                dummy_name_chirho("Ordering"),
                            )),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Class with superclass should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
        assert!(result_chirho.class_env_chirho.has_class_chirho("MyOrdChirho"));
        let supers_chirho = result_chirho
            .class_env_chirho
            .superclasses_chirho("MyOrdChirho");
        assert_eq!(supers_chirho, vec!["Eq".to_string()]);
    }

    /// Test: user-defined instance declaration registers in class env.
    /// Haskell: `instance Eq Int where ...`
    #[test]
    fn infer_user_instance_decl_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("UserInst"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::InstanceDeclChirho {
                context_chirho: vec![],
                class_chirho: dummy_name_chirho("Eq"),
                types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("MyType"))],
                methods_chirho: vec![],
            assoc_tf_instances_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Instance decl should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
        // The instance should now be resolvable
        let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::ConChirho("MyType".to_string()));
        assert!(
            result_chirho.class_env_chirho.entails_chirho(&pred_chirho),
            "Eq MyType should be entailed after instance registration"
        );
    }

    /// Test: type signature that matches inferred type produces no error.
    /// Haskell: `idChirho :: a -> a; idChirho x = x`
    #[test]
    fn infer_matching_type_sig_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("TypeSig"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::TypeSigChirho {
                    name_chirho: dummy_name_chirho("idChirho"),
                    ty_chirho: TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("idChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("x"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Matching type sig should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
    }

    /// Test: type signature mismatch produces an error.
    /// Haskell: `badChirho :: Int -> Int; badChirho x = True`
    #[test]
    fn infer_mismatching_type_sig_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("BadSig"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::TypeSigChirho {
                    name_chirho: dummy_name_chirho("badChirho"),
                    ty_chirho: TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("badChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("True"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Type sig mismatch should produce an error"
        );
    }

    /// Test: newtype constructor gets proper typing.
    /// Haskell: `newtype Age = MkAge Int`
    #[test]
    fn infer_newtype_constructor_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("NT"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::NewtypeDeclChirho {
                    name_chirho: dummy_name_chirho("Age"),
                    type_vars_chirho: vec![],
                    constructor_chirho: ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho("MkAge"),
                        fields_chirho: vec![(haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho, TypeChirho::ConChirho(dummy_name_chirho("Int")))],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                // Use the constructor: mkAgeChirho = MkAge 42
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("mkAgeChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::ConChirho(
                                dummy_name_chirho("MkAge"),
                            )),
                            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                                42,
                                SpanChirho::DUMMY_CHIRHO,
                            ))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Newtype + usage should not error: {:?}",
            result_chirho.diagnostics_chirho
        );

        // MkAge constructor should be in the environment
        let mk_age_chirho = result_chirho
            .env_chirho
            .lookup_chirho("MkAge")
            .expect("MkAge should be in env");
        // MkAge :: a -> Age (field gets fresh var since we don't resolve AST types yet)
        if let TyChirho::FunChirho(_, ref ret_chirho, _) = mk_age_chirho.ty_chirho {
            assert_eq!(**ret_chirho, TyChirho::ConChirho("Age".to_string()));
        } else {
            panic!("MkAge should be a function type, got: {}", mk_age_chirho);
        }

        // mkAgeChirho should also be in the environment with type Age
        let mk_age_result_chirho = result_chirho
            .env_chirho
            .lookup_chirho("mkAgeChirho")
            .expect("mkAgeChirho should be in env");
        assert_eq!(
            mk_age_result_chirho.ty_chirho,
            TyChirho::ConChirho("Age".to_string())
        );
    }

    /// Test: AST type to internal type conversion roundtrip.
    #[test]
    fn ast_type_conversion_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let mut var_map_chirho = HashMap::new();

        // Convert `a -> Int`
        let ast_ty_chirho = TypeChirho::FunChirho {
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
            mult_chirho: None,
            result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let ty_chirho = ctx_chirho.ast_type_to_ty_chirho(&ast_ty_chirho, &mut var_map_chirho);

        // Should be FunChirho(VarChirho(_), ConChirho("Int"))
        if let TyChirho::FunChirho(ref arg_chirho, ref ret_chirho, _) = ty_chirho {
            assert!(matches!(**arg_chirho, TyChirho::VarChirho(_)));
            assert_eq!(**ret_chirho, TyChirho::int_chirho());
        } else {
            panic!("expected function type, got: {ty_chirho}");
        }

        // Same `a` should reuse the same variable
        let a2_chirho =
            ctx_chirho.ast_type_to_ty_chirho(
                &TypeChirho::VarChirho(dummy_name_chirho("a")),
                &mut var_map_chirho,
            );
        if let (TyChirho::FunChirho(arg_chirho, _, _), TyChirho::VarChirho(v2_chirho)) =
            (&ty_chirho, &a2_chirho)
        {
            if let TyChirho::VarChirho(v1_chirho) = &**arg_chirho {
                assert_eq!(*v1_chirho, *v2_chirho, "same `a` should yield same var");
            }
        }
    }

    /// Test: do-notation bind statement unwraps monadic type.
    /// Haskell: `doExample = do { x <- Just 42; pure x }`
    #[test]
    fn infer_do_bind_unwraps_monad_chirho() {
        // Build: doExample = do { x <- expr_m; expr_body }
        // where expr_m :: Maybe Int  (simulated as App(Con("Maybe"), Lit(42)))
        // and expr_body :: uses x (a VarChirho)
        //
        // We seed Maybe Just Nothing in the env.
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("DoNotation"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("doExampleChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::DoChirho {
                        stmts_chirho: vec![
                            // x <- Just 42  (simulated: x <- app(Just, 42))
                            StmtChirho::BindChirho {
                                pat_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
                                expr_chirho: ExprChirho::AppChirho {
                                    fun_chirho: Box::new(ExprChirho::ConChirho(
                                        dummy_name_chirho("Just"),
                                    )),
                                    arg_chirho: Box::new(ExprChirho::LitChirho(
                                        LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO),
                                    )),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                                },
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            // pure x (simplified: just return x)
                            StmtChirho::ExprChirho(ExprChirho::VarChirho(
                                dummy_name_chirho("x"),
                            )),
                        ],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "do-notation bind should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
        // doExampleChirho should be in the environment
        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("doExampleChirho")
            .expect("doExampleChirho should be in env");
        // The type should involve x :: Int (unwrapped from Just Int)
        // The last stmt `x` has type Int, not wrapped
        assert!(
            !format!("{scheme_chirho}").is_empty(),
            "should have a type"
        );
    }

    /// Test: do-notation with let statement.
    /// Haskell: `doLetChirho = do { let y = 10; pure y }`
    #[test]
    fn infer_do_let_statement_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("DoLet"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("doLetChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::DoChirho {
                        stmts_chirho: vec![
                            // let y = 10
                            StmtChirho::LetChirho {
                                binds_chirho: vec![
                                    haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                                        pat_chirho: PatChirho::VarChirho(
                                            dummy_name_chirho("y"),
                                        ),
                                        rhs_chirho: RhsChirho::UnguardedChirho(
                                            ExprChirho::LitChirho(LitChirho::IntChirho(
                                                10,
                                                SpanChirho::DUMMY_CHIRHO,
                                            )),
                                        ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    },
                                ],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            // y (return it)
                            StmtChirho::ExprChirho(ExprChirho::VarChirho(
                                dummy_name_chirho("y"),
                            )),
                        ],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "do-notation with let should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("doLetChirho")
            .expect("doLetChirho should be in env");
        // The result should be Int (y = 10, return y)
        assert_eq!(
            scheme_chirho.ty_chirho,
            TyChirho::int_chirho(),
            "do-let block returning Int literal should infer Int"
        );
    }

    #[test]
    fn type_family_registration_and_reduction_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // Register a closed type family: F Int = Bool, F Char = Int
        ctx_chirho.register_type_family_chirho(
            "F".to_string(),
            vec![
                (vec![TyChirho::int_chirho()], TyChirho::bool_chirho()),
                (vec![TyChirho::char_chirho()], TyChirho::int_chirho()),
            ],
        );
        // Reduce F Int → Bool
        let result_chirho = ctx_chirho.reduce_type_family_chirho("F", &[TyChirho::int_chirho()]);
        assert_eq!(result_chirho, Some(TyChirho::bool_chirho()), "F Int should reduce to Bool");
        // Reduce F Char → Int
        let result2_chirho = ctx_chirho.reduce_type_family_chirho("F", &[TyChirho::char_chirho()]);
        assert_eq!(result2_chirho, Some(TyChirho::int_chirho()), "F Char should reduce to Int");
        // F String should not reduce (stuck)
        let result3_chirho = ctx_chirho.reduce_type_family_chirho("F", &[TyChirho::string_chirho()]);
        assert_eq!(result3_chirho, None, "F String should be stuck (no matching equation)");
    }

    #[test]
    fn type_family_instance_registration_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // Register open type family F
        ctx_chirho.register_type_family_chirho("F".to_string(), vec![]);
        // Add instance: F Int = Bool
        ctx_chirho.register_type_family_instance_chirho(
            "F".to_string(),
            vec![TyChirho::int_chirho()],
            TyChirho::bool_chirho(),
        );
        let result_chirho = ctx_chirho.reduce_type_family_chirho("F", &[TyChirho::int_chirho()]);
        assert_eq!(result_chirho, Some(TyChirho::bool_chirho()), "open F Int should reduce to Bool");
    }

    #[test]
    fn type_family_nonexistent_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        let result_chirho = ctx_chirho.reduce_type_family_chirho("NoSuchFamily", &[TyChirho::int_chirho()]);
        assert_eq!(result_chirho, None, "unregistered family should return None");
    }
}
