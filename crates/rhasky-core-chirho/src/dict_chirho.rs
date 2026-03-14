// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Dictionary-passing transform
//!
//! Desugars typeclass constraints into explicit dictionary arguments in Core.
//! After this pass, constrained polymorphic functions receive dictionary
//! parameters instead of carrying implicit class predicates.
//!
//! This is the standard approach used by GHC and other Haskell compilers:
//! - Each typeclass generates a "dictionary" record type
//! - Each instance generates a dictionary value
//! - Constrained functions take dictionary arguments
//! - Method calls become dictionary projections
//!
//! ## Current scope
//!
//! This implementation handles:
//! - Building dictionary layouts from the class environment
//! - Adding dictionary lambda parameters to constrained top-level bindings
//! - Generating method selector functions for each class method
//! - Generating instance dictionary bindings (ground instances)
//! - Rewriting method call sites to use dictionary projections
//!
//! Future work:
//! - Conditional instance dictionaries (instances with context, e.g. `Eq a => Eq [a]`)
//! - Superclass dictionary extraction
//! - Instance method body compilation from source `where` clauses

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use rhasky_span_chirho::SpanChirho;
use rhasky_typing_chirho::class_chirho::ClassEnvChirho;
use rhasky_typing_chirho::env_chirho::TyEnvChirho;
use rhasky_typing_chirho::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho,
};

/// Layout of a typeclass dictionary.
///
/// Maps each method name to its position in the dictionary product type.
/// Superclass dictionaries come first, then methods in sorted order.
#[derive(Debug, Clone)]
pub struct DictLayoutChirho {
    pub class_name_chirho: String,
    /// Superclass dictionary positions: `(superclass_name, slot_index)`.
    pub super_slots_chirho: Vec<(String, usize)>,
    /// Method positions: `(method_name, slot_index)`.
    pub method_slots_chirho: Vec<(String, usize)>,
    /// Total number of fields (super dicts + methods).
    pub field_count_chirho: usize,
}

/// Result of the dictionary-passing transform.
#[derive(Debug)]
pub struct DictPassResultChirho {
    pub module_chirho: CoreModuleChirho,
    /// Updated name map including generated dictionary IDs.
    pub names_chirho: HashMap<CoreIdChirho, String>,
    /// Dictionary layouts for each class.
    pub layouts_chirho: HashMap<String, DictLayoutChirho>,
}

/// The dictionary-passing transform context.
pub struct DictPassCtxChirho {
    next_id_chirho: u32,
    names_chirho: HashMap<CoreIdChirho, String>,
    /// Class name -> dictionary layout.
    layouts_chirho: HashMap<String, DictLayoutChirho>,
    /// Generated top-level dictionary bindings (instance dicts, selectors).
    generated_bindings_chirho: Vec<CoreBindingChirho>,
    /// Method name -> (class_name, selector CoreId).
    method_selectors_chirho: HashMap<String, (String, CoreIdChirho)>,
    /// (class_name, type_key) -> CoreId of instance dictionary binding.
    instance_dicts_chirho: HashMap<(String, String), CoreIdChirho>,
    /// Constructor name -> type name (e.g. "Red" -> "Color").
    /// Used to determine which instance dictionary to select when a class
    /// method is applied to constructor values.
    con_types_chirho: HashMap<String, String>,
    /// Tracks which user-defined bindings received dictionary lambda
    /// parameters during `add_dict_params_chirho`. Maps the binding's
    /// `CoreIdChirho` to the ordered list of class names whose dictionaries
    /// must be passed at each call site.
    dict_param_bindings_chirho: HashMap<CoreIdChirho, Vec<String>>,
    /// Superclass selector CoreIds: `(subclass, superclass)` → selector id.
    /// Used to extract a superclass dictionary from a subclass dictionary.
    super_selectors_chirho: HashMap<(String, String), CoreIdChirho>,
    /// Conditional instance dict builders: `(class_name, type_constructor)` →
    /// `(builder_id, context_classes)`. The builder is a lambda that takes
    /// element/component dictionaries and returns the constructed class dict.
    /// For example, `Eq a => Eq [a]` yields `("Eq", "List")` →
    /// `($fEqList, ["Eq"])` where `$fEqList = \$dEqA -> $DictEq (list_eq_impl $dEqA)`.
    #[allow(dead_code)]
    conditional_dicts_chirho: HashMap<(String, String), (CoreIdChirho, Vec<String>)>,
    /// Newtype info: maps type name to (constructor name, underlying type key).
    /// Used for GND: `"Age"` → `("MkAge", "Int")`.
    newtype_info_chirho: HashMap<String, (String, String)>,
    /// Number of type parameters per class. 1 for single-parameter classes
    /// (Eq, Ord, ...), >1 for MPTCs (e.g., Addable a b has 2).
    class_param_count_chirho: HashMap<String, usize>,
    /// IDs of locally-bound variables during method rewriting.
    /// When a let/where/lambda introduces a binding whose name matches a
    /// class method (e.g. `where pi = 3`), that ID is added here so
    /// `try_rewrite_method_var_chirho` skips it.
    /// Uses RefCell for interior mutability during the recursive rewrite pass.
    local_shadow_ids_chirho: RefCell<HashSet<CoreIdChirho>>,
}

impl DictPassCtxChirho {
    pub fn new_chirho(
        start_id_chirho: u32,
        names_chirho: HashMap<CoreIdChirho, String>,
        con_types_chirho: HashMap<String, String>,
    ) -> Self {
        Self {
            next_id_chirho: start_id_chirho,
            names_chirho,
            layouts_chirho: HashMap::new(),
            generated_bindings_chirho: Vec::new(),
            method_selectors_chirho: HashMap::new(),
            instance_dicts_chirho: HashMap::new(),
            con_types_chirho,
            dict_param_bindings_chirho: HashMap::new(),
            super_selectors_chirho: HashMap::new(),
            conditional_dicts_chirho: HashMap::new(),
            newtype_info_chirho: HashMap::new(),
            class_param_count_chirho: HashMap::new(),
            local_shadow_ids_chirho: RefCell::new(HashSet::new()),
        }
    }

    /// Generate a fresh CoreId and record its name.
    fn fresh_id_chirho(&mut self, name_chirho: &str) -> CoreIdChirho {
        let id_chirho = CoreIdChirho(self.next_id_chirho);
        self.next_id_chirho += 1;
        self.names_chirho
            .insert(id_chirho, name_chirho.to_string());
        id_chirho
    }

    /// Look up an existing CoreId by name. Returns None if not found.
    fn lookup_name_id_chirho(&self, name_chirho: &str) -> Option<CoreIdChirho> {
        for (id_chirho, existing_name_chirho) in &self.names_chirho {
            if existing_name_chirho == name_chirho {
                return Some(*id_chirho);
            }
        }
        None
    }

    /// Look up an existing CoreId by name, or create a fresh one.
    /// This is used when generating references to bindings that may
    /// already exist (e.g. `$prim_` bindings from desugaring).
    fn resolve_or_fresh_id_chirho(&mut self, name_chirho: &str) -> CoreIdChirho {
        // Search for an existing ID with this name
        for (id_chirho, existing_name_chirho) in &self.names_chirho {
            if existing_name_chirho == name_chirho {
                return *id_chirho;
            }
        }
        // Not found — create a fresh one
        self.fresh_id_chirho(name_chirho)
    }

    /// Check whether a predicate's type variable is "defaultable" in the
    /// context of a scheme.  A type variable is defaultable when it does
    /// NOT appear in any function-argument position of the scheme's type,
    /// meaning no call site can determine it — it is effectively ambiguous.
    /// This implements a simplified version of Haskell 2010 §4.3.4.
    fn is_defaultable_pred_chirho(
        pred_chirho: &rhasky_typing_chirho::ty_chirho::SchemePredChirho,
        scheme_chirho: &SchemeChirho,
    ) -> bool {
        let tv_chirho = match &pred_chirho.ty_chirho {
            TyChirho::VarChirho(v_chirho) => *v_chirho,
            _ => return false, // ground type, not defaultable
        };

        // Collect argument types from the function chain a -> b -> c -> r
        fn collect_arg_tys_chirho(ty_chirho: &TyChirho) -> Vec<&TyChirho> {
            match ty_chirho {
                TyChirho::FunChirho(arg_chirho, res_chirho) => {
                    let mut args_chirho = vec![arg_chirho.as_ref()];
                    args_chirho.extend(collect_arg_tys_chirho(res_chirho));
                    args_chirho
                }
                _ => vec![],
            }
        }

        fn ty_contains_var_chirho(ty_chirho: &TyChirho, tv_chirho: TyVarChirho) -> bool {
            match ty_chirho {
                TyChirho::VarChirho(v_chirho) => *v_chirho == tv_chirho,
                TyChirho::FunChirho(a_chirho, b_chirho) => {
                    ty_contains_var_chirho(a_chirho, tv_chirho)
                        || ty_contains_var_chirho(b_chirho, tv_chirho)
                }
                TyChirho::AppChirho(a_chirho, b_chirho) => {
                    ty_contains_var_chirho(a_chirho, tv_chirho)
                        || ty_contains_var_chirho(b_chirho, tv_chirho)
                }
                TyChirho::ConChirho(_) => false,
                _ => false,
            }
        }

        let arg_tys_chirho = collect_arg_tys_chirho(&scheme_chirho.ty_chirho);
        // If the type variable appears in ANY argument type, a call site
        // can determine it — so it is NOT defaultable.
        !arg_tys_chirho
            .iter()
            .any(|arg_chirho| ty_contains_var_chirho(arg_chirho, tv_chirho))
    }

    /// Attempt to infer the type key (e.g. "Int", "Color") from a Core
    /// expression.  This is used to select the correct instance dictionary
    /// when a class method is applied to concrete arguments.
    fn infer_type_key_chirho(&self, expr_chirho: &CoreExprChirho) -> Option<String> {
        match expr_chirho {
            CoreExprChirho::LitChirho(lit_chirho) => match lit_chirho {
                crate::expr_chirho::CoreLitChirho::IntChirho(_) => Some("Int".to_string()),
                crate::expr_chirho::CoreLitChirho::CharChirho(_) => Some("Char".to_string()),
                crate::expr_chirho::CoreLitChirho::StringChirho(_) => Some("[Char]".to_string()),
                crate::expr_chirho::CoreLitChirho::FloatChirho(_) => Some("Double".to_string()),
            },
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } => {
                // Look up constructor → type mapping
                if let Some(type_name_chirho) = self.con_types_chirho.get(con_name_chirho) {
                    Some(type_name_chirho.clone())
                } else {
                    // Built-in constructors
                    match con_name_chirho.as_str() {
                        "True" | "False" => Some("Bool".to_string()),
                        "LT" | "EQ" | "GT" => Some("Ordering".to_string()),
                        "Nothing" => Some("Maybe Int".to_string()),
                        "Left" => {
                            if let Some(inner_chirho) = args_chirho.first() {
                                let inner_key_chirho = self.infer_type_key_chirho(inner_chirho);
                                let inner_str_chirho = inner_key_chirho.as_deref().unwrap_or("Int");
                                return Some(format!("Either {} Int", inner_str_chirho));
                            }
                            Some("Either Int Int".to_string())
                        }
                        "Right" => {
                            if let Some(inner_chirho) = args_chirho.first() {
                                let inner_key_chirho = self.infer_type_key_chirho(inner_chirho);
                                let inner_str_chirho = inner_key_chirho.as_deref().unwrap_or("Int");
                                return Some(format!("Either Int {}", inner_str_chirho));
                            }
                            Some("Either Int Int".to_string())
                        }
                        "Just" => {
                            if let Some(inner_chirho) = args_chirho.first() {
                                let inner_key_chirho = self.infer_type_key_chirho(inner_chirho);
                                match inner_key_chirho.as_deref() {
                                    Some("Int") => return Some("Maybe Int".to_string()),
                                    Some("[Char]") => return Some("Maybe String".to_string()),
                                    Some("Double") => return Some("Maybe Double".to_string()),
                                    _ => return Some("Maybe Int".to_string()),
                                }
                            }
                            Some("Maybe Int".to_string())
                        }
                        "(,)" | "$tuple2" => {
                            if args_chirho.len() >= 2 {
                                let a_chirho = self.infer_type_key_chirho(&args_chirho[0]).unwrap_or("Int".to_string());
                                let b_chirho = self.infer_type_key_chirho(&args_chirho[1]).unwrap_or("Int".to_string());
                                // Map [Char] to String for readability
                                let a_key_chirho = if a_chirho == "[Char]" { "String" } else { &a_chirho };
                                let b_key_chirho = if b_chirho == "[Char]" { "String" } else { &b_chirho };
                                return Some(format!("({},{})", a_key_chirho, b_key_chirho));
                            }
                            Some("(Int,Int)".to_string())
                        }
                        "[]" => {
                            // Empty list — default to [Int]
                            Some("[Int]".to_string())
                        }
                        ":" => {
                            // Cons cell — infer element type from head
                            if let Some(head_chirho) = args_chirho.first() {
                                let elem_ty_chirho = self.infer_type_key_chirho(head_chirho);
                                match elem_ty_chirho.as_deref() {
                                    Some("Int") => Some("[Int]".to_string()),
                                    Some("[Char]") => Some("[[Char]]".to_string()),
                                    Some("Char") => Some("[Char]".to_string()),
                                    Some("Double") => Some("[Double]".to_string()),
                                    Some("Bool") => Some("[Bool]".to_string()),
                                    _ => Some("[Int]".to_string()), // default
                                }
                            } else {
                                Some("[Int]".to_string())
                            }
                        }
                        _ => None,
                    }
                }
            }
            CoreExprChirho::VarChirho(id_chirho) => {
                // Check if the var name is a known constructor
                if let Some(name_chirho) = self.names_chirho.get(id_chirho) {
                    if let Some(type_name_chirho) = self.con_types_chirho.get(name_chirho) {
                        return Some(type_name_chirho.clone());
                    }
                    // Built-in constructors
                    match name_chirho.as_str() {
                        "True" | "False" => return Some("Bool".to_string()),
                        "Nothing" => return Some("Maybe Int".to_string()),
                        "LT" | "EQ" | "GT" => return Some("Ordering".to_string()),
                        _ => {}
                    }
                }
                None
            }
            CoreExprChirho::PrimOpChirho {
                name_chirho, ..
            } => {
                // Infer result type of known primops
                match name_chirho.as_str() {
                    "enumFromTo#" => Some("[Int]".to_string()),
                    "+#" | "-#" | "*#" | "div#" | "mod#" | "negate#"
                    | "readInt#" => {
                        Some("Int".to_string())
                    }
                    "+.#" | "-.#" | "*.#" | "/.#" | "negateFloat#"
                    | "recip#" | "readFloat#" => {
                        Some("Double".to_string())
                    }
                    "==#" | "/=#" | "<#" | "<=#" | ">#" | ">=#" | "not#"
                    | "eqFloat#" | "readBool#" => Some("Bool".to_string()),
                    "compare#" | "compareChar#" | "compareFloat#" | "compareStr#" => {
                        Some("Ordering".to_string())
                    }
                    "showInt#" | "showFloat#" | "showStr#" | "showList#"
                    | "showMaybe#" | "showTuple2#" | "showEither#" | "showOrdering#"
                    | "showBool#"
                    | "++#" => Some("[Char]".to_string()),
                    _ => None,
                }
            }
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                // For App(Var(fromInteger), Lit(n)), the result type is Int
                // (or Double, depending on context — default to Int).
                if let CoreExprChirho::VarChirho(id_chirho) = fun_chirho.as_ref() {
                    if let Some(name_chirho) = self.names_chirho.get(id_chirho) {
                        if name_chirho == "fromInteger" {
                            return Some("Int".to_string());
                        }
                        // Constructor applications: App(Just, x) → Maybe <x-type>
                        match name_chirho.as_str() {
                            "Just" => {
                                let inner_chirho = self.infer_type_key_chirho(arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                let inner_key_chirho = if inner_chirho == "[Char]" { "String" } else { &inner_chirho };
                                return Some(format!("Maybe {}", inner_key_chirho));
                            }
                            "Left" => {
                                let inner_chirho = self.infer_type_key_chirho(arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                return Some(format!("Either {} Int", inner_chirho));
                            }
                            "Right" => {
                                let inner_chirho = self.infer_type_key_chirho(arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                return Some(format!("Either Int {}", inner_chirho));
                            }
                            _ => {}
                        }
                        // Check if the name is a known constructor and return its type
                        if let Some(type_name_chirho) = self.con_types_chirho.get(name_chirho) {
                            return Some(type_name_chirho.clone());
                        }
                    }
                }
                // Detect App(App(Var((,)), a), b) → tuple type
                // Detect App(App(Var(compare), a), b) → Ordering
                // Detect App(App(Var(Left/Right), a), b) → Either type
                if let CoreExprChirho::AppChirho {
                    fun_chirho: inner_fun_chirho,
                    arg_chirho: first_arg_chirho,
                } = fun_chirho.as_ref()
                {
                    if let CoreExprChirho::VarChirho(id_chirho) = inner_fun_chirho.as_ref() {
                        if let Some(name_chirho) = self.names_chirho.get(id_chirho) {
                            if name_chirho == "(,)" || name_chirho == "$tuple2" {
                                let a_chirho = self.infer_type_key_chirho(first_arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                let b_chirho = self.infer_type_key_chirho(arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                let a_key_chirho = if a_chirho == "[Char]" { "String" } else { &a_chirho };
                                let b_key_chirho = if b_chirho == "[Char]" { "String" } else { &b_chirho };
                                return Some(format!("({},{})", a_key_chirho, b_key_chirho));
                            }
                            // compare :: a -> a -> Ordering
                            if name_chirho == "compare" || name_chirho.starts_with("$prim_Ord_compare") {
                                return Some("Ordering".to_string());
                            }
                        }
                    }
                }
                // For general App chains like `f 1.5 2.5`, try to infer
                // from the argument first, then recurse into the function
                // (which is itself an App for curried calls).
                if let Some(tk_chirho) = self.infer_type_key_chirho(arg_chirho) {
                    return Some(tk_chirho);
                }
                self.infer_type_key_chirho(fun_chirho)
            }
            _ => None,
        }
    }

    /// Create a binder with a fresh ID.
    fn fresh_binder_chirho(
        &mut self,
        name_chirho: &str,
        ty_chirho: TyChirho,
    ) -> BinderChirho {
        BinderChirho {
            id_chirho: self.fresh_id_chirho(name_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    /// Build dictionary layouts from the class environment.
    pub fn build_layouts_chirho(&mut self, class_env_chirho: &ClassEnvChirho) {
        for (name_chirho, decl_chirho) in &class_env_chirho.classes_chirho {
            let mut slot_chirho = 0usize;

            let super_slots_chirho: Vec<(String, usize)> = decl_chirho
                .supers_chirho
                .iter()
                .map(|super_name_chirho| {
                    let pos_chirho = slot_chirho;
                    slot_chirho += 1;
                    (super_name_chirho.clone(), pos_chirho)
                })
                .collect();

            // Sort method names for deterministic layout
            let mut method_names_chirho: Vec<String> =
                decl_chirho.methods_chirho.keys().cloned().collect();
            method_names_chirho.sort();

            let method_slots_chirho: Vec<(String, usize)> = method_names_chirho
                .into_iter()
                .map(|method_name_chirho| {
                    let pos_chirho = slot_chirho;
                    slot_chirho += 1;
                    (method_name_chirho, pos_chirho)
                })
                .collect();

            self.layouts_chirho.insert(
                name_chirho.clone(),
                DictLayoutChirho {
                    class_name_chirho: name_chirho.clone(),
                    super_slots_chirho,
                    method_slots_chirho,
                    field_count_chirho: slot_chirho,
                },
            );

            // Track parameter count for MPTC dispatch
            let param_count_chirho = decl_chirho.all_vars_chirho().len();
            self.class_param_count_chirho
                .insert(name_chirho.clone(), param_count_chirho);
        }
    }

    /// Generate method selector functions for each class.
    ///
    /// For a class `Eq` with method `==` at slot 0 in a 2-field dict:
    /// ```text
    /// $sel_Eq_== = \$dict -> case $dict of
    ///     $DictEq f0 f1 -> f0
    /// ```
    pub fn generate_selectors_chirho(&mut self) {
        let layouts_chirho: Vec<_> = self.layouts_chirho.values().cloned().collect();
        for layout_chirho in &layouts_chirho {
            for (method_name_chirho, slot_idx_chirho) in &layout_chirho.method_slots_chirho {
                let sel_name_chirho = format!(
                    "$sel_{}_{}",
                    layout_chirho.class_name_chirho, method_name_chirho
                );
                let dict_ty_chirho = TyChirho::ConChirho(format!(
                    "$Dict_{}",
                    layout_chirho.class_name_chirho
                ));

                // The dictionary binder for the lambda
                let dict_binder_chirho =
                    self.fresh_binder_chirho("$dict", dict_ty_chirho.clone());
                let dict_id_chirho = dict_binder_chirho.id_chirho;

                // Build field binders for the case alt
                let mut field_binders_chirho = Vec::new();
                let mut selected_id_chirho = CoreIdChirho(0);
                for i_chirho in 0..layout_chirho.field_count_chirho {
                    let field_name_chirho = format!("$f{i_chirho}");
                    let fb_chirho = self.fresh_binder_chirho(
                        &field_name_chirho,
                        TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                            9999,
                        )),
                    );
                    if i_chirho == *slot_idx_chirho {
                        selected_id_chirho = fb_chirho.id_chirho;
                    }
                    field_binders_chirho.push(fb_chirho);
                }

                let case_wild_chirho =
                    self.fresh_binder_chirho("$wild", dict_ty_chirho.clone());

                let con_name_chirho =
                    format!("$Dict_{}", layout_chirho.class_name_chirho);

                let case_expr_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
                    bind_chirho: case_wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9998),
                    ),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(con_name_chirho),
                        binders_chirho: field_binders_chirho,
                        rhs_chirho: CoreExprChirho::VarChirho(selected_id_chirho),
                    }],
                };

                let selector_rhs_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: dict_binder_chirho,
                    body_chirho: Box::new(case_expr_chirho),
                };

                let sel_binder_chirho = self.fresh_binder_chirho(
                    &sel_name_chirho,
                    TyChirho::fun_chirho(
                        dict_ty_chirho,
                        TyChirho::VarChirho(
                            rhasky_typing_chirho::ty_chirho::TyVarChirho(9998),
                        ),
                    ),
                );
                let sel_id_chirho = sel_binder_chirho.id_chirho;

                self.generated_bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: sel_binder_chirho,
                    rhs_chirho: selector_rhs_chirho,
                    is_rec_chirho: false,
                });

                self.method_selectors_chirho.insert(
                    method_name_chirho.clone(),
                    (layout_chirho.class_name_chirho.clone(), sel_id_chirho),
                );
            }

            // Generate superclass selectors:
            // $sel_Ord_super_Eq = \$dict -> case $dict of $DictOrd f0 f1 ... -> f0
            for (super_name_chirho, slot_idx_chirho) in &layout_chirho.super_slots_chirho {
                let sel_name_chirho = format!(
                    "$sel_{}_super_{}",
                    layout_chirho.class_name_chirho, super_name_chirho
                );
                let dict_ty_chirho = TyChirho::ConChirho(format!(
                    "$Dict_{}",
                    layout_chirho.class_name_chirho
                ));

                let dict_binder_chirho =
                    self.fresh_binder_chirho("$dict", dict_ty_chirho.clone());
                let dict_id_chirho = dict_binder_chirho.id_chirho;

                let mut field_binders_chirho = Vec::new();
                let mut selected_id_chirho = CoreIdChirho(0);
                for i_chirho in 0..layout_chirho.field_count_chirho {
                    let field_name_chirho = format!("$f{i_chirho}");
                    let fb_chirho = self.fresh_binder_chirho(
                        &field_name_chirho,
                        TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                            9999,
                        )),
                    );
                    if i_chirho == *slot_idx_chirho {
                        selected_id_chirho = fb_chirho.id_chirho;
                    }
                    field_binders_chirho.push(fb_chirho);
                }

                let case_wild_chirho =
                    self.fresh_binder_chirho("$wild", dict_ty_chirho.clone());
                let con_name_chirho =
                    format!("$Dict_{}", layout_chirho.class_name_chirho);

                let case_expr_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
                    bind_chirho: case_wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9998),
                    ),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(con_name_chirho),
                        binders_chirho: field_binders_chirho,
                        rhs_chirho: CoreExprChirho::VarChirho(selected_id_chirho),
                    }],
                };

                let selector_rhs_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: dict_binder_chirho,
                    body_chirho: Box::new(case_expr_chirho),
                };

                let super_dict_ty_chirho = TyChirho::ConChirho(format!(
                    "$Dict_{}",
                    super_name_chirho
                ));
                let sel_binder_chirho = self.fresh_binder_chirho(
                    &sel_name_chirho,
                    TyChirho::fun_chirho(dict_ty_chirho, super_dict_ty_chirho),
                );
                let sel_id_chirho = sel_binder_chirho.id_chirho;

                self.generated_bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: sel_binder_chirho,
                    rhs_chirho: selector_rhs_chirho,
                    is_rec_chirho: false,
                });

                self.super_selectors_chirho.insert(
                    (
                        layout_chirho.class_name_chirho.clone(),
                        super_name_chirho.clone(),
                    ),
                    sel_id_chirho,
                );
            }
        }
    }

    /// Generate `$prim_` bindings for built-in class methods on
    /// primitive types (Int, Char, Bool) where the implementations are
    /// compiler-provided primops rather than user-written Haskell.
    ///
    /// For example:
    /// - `$prim_Eq_==_Int = \x y -> x ==# y`
    /// - `$prim_Num_+_Int = \x y -> x +# y`
    /// - `$prim_Show_show_Int = \x -> showInt# x` (converts Int to String)
    /// - `$prim_Num_fromInteger_Int = \x -> x` (identity for Int)
    pub fn generate_builtin_prim_bindings_chirho(&mut self) {
        let builtins_chirho: Vec<(&str, &str, &str, u8)> = vec![
            // (class, method, type_key, kind)
            // kind: 0 = identity, 1 = binary primop, 2 = unary primop
            // Eq
            ("Eq", "==", "Int", 1),
            ("Eq", "==", "Char", 1),
            ("Eq", "==", "Bool", 1),
            // Ord
            ("Ord", "compare", "Int", 1),
            ("Ord", "compare", "Char", 1),
            ("Ord", "compare", "Double", 1),
            ("Ord", "compare", "Bool", 1),
            ("Ord", "compare", "[Char]", 1),
            // Show
            ("Show", "show", "Int", 2),
            ("Show", "show", "Char", 0),
            // Show Bool is handled specially below (case True/False -> string)
            ("Show", "show", "Bool", 0),
            // Num
            ("Num", "+", "Int", 1),
            ("Num", "*", "Int", 1),
            ("Num", "-", "Int", 1),
            ("Num", "negate", "Int", 2),
            ("Num", "fromInteger", "Int", 0),
            // Double
            ("Eq", "==", "Double", 1),
            ("Show", "show", "Double", 2),
            ("Num", "+", "Double", 1),
            ("Num", "*", "Double", 1),
            ("Num", "-", "Double", 1),
            ("Num", "negate", "Double", 2),
            ("Num", "fromInteger", "Double", 0),
            // Fractional Double
            ("Fractional", "/", "Double", 1),
            ("Fractional", "recip", "Double", 2),
            ("Fractional", "fromRational", "Double", 0),
            // Read instances
            ("Read", "read", "Int", 2),
            ("Read", "read", "Double", 2),
            ("Read", "read", "Bool", 2),
            // String (i.e. [Char])
            ("Eq", "==", "[Char]", 1),
            ("Show", "show", "[Char]", 2),
            // Lists
            ("Show", "show", "[Int]", 2),
            // Maybe
            ("Show", "show", "Maybe Int", 2),
            ("Show", "show", "Maybe String", 2),
            ("Show", "show", "Maybe Double", 2),
            // Tuples
            ("Show", "show", "(Int,Int)", 2),
            ("Show", "show", "(Int,String)", 2),
            ("Show", "show", "(String,Int)", 2),
            ("Show", "show", "(String,String)", 2),
            ("Show", "show", "(Int,Bool)", 2),
            ("Show", "show", "(Bool,Int)", 2),
            ("Show", "show", "(Int,Double)", 2),
            ("Show", "show", "(Double,Int)", 2),
            ("Show", "show", "(Bool,Bool)", 2),
            ("Show", "show", "(Double,Double)", 2),
            ("Show", "show", "(Bool,String)", 2),
            ("Show", "show", "(String,Bool)", 2),
            // Either
            ("Show", "show", "Either Int Int", 2),
            ("Show", "show", "Either String Int", 2),
            ("Show", "show", "Either Int String", 2),
            ("Show", "show", "Either String String", 2),
            // Ordering
            ("Show", "show", "Ordering", 2),
            ("Eq", "==", "Ordering", 1),
            ("Ord", "compare", "Ordering", 1),
        ];

        let primop_for_chirho =
            |class_chirho: &str, method_chirho: &str, type_key_chirho: &str| -> &str {
                match (class_chirho, method_chirho, type_key_chirho) {
                    ("Eq", "==", "[Char]") => "eqStr#",
                    ("Eq", "==", "Double") => "eqFloat#",
                    ("Eq", "==", _) => "==#",
                    ("Ord", "compare", "Char") => "compareChar#",
                    ("Ord", "compare", "Double") => "compareFloat#",
                    ("Ord", "compare", "[Char]") => "compareStr#",
                    ("Ord", "compare", "Bool") => "compare#",
                    ("Ord", "compare", _) => "compare#",
                    ("Num", "+", "Double") => "+.#",
                    ("Num", "+", _) => "+#",
                    ("Num", "*", "Double") => "*.#",
                    ("Num", "*", _) => "*#",
                    ("Num", "-", "Double") => "-.#",
                    ("Num", "-", _) => "-#",
                    ("Num", "negate", "Double") => "negateFloat#",
                    ("Num", "negate", _) => "negate#",
                    ("Fractional", "/", _) => "/.#",
                    ("Fractional", "recip", _) => "recip#",
                    ("Show", "show", "[Char]") => "showStr#",
                    ("Show", "show", "[Int]") => "showList#",
                    ("Show", "show", "Double") => "showFloat#",
                    ("Show", "show", "Maybe Int") => "showMaybe#",
                    ("Show", "show", "Maybe String") => "showMaybe#",
                    ("Show", "show", "Maybe Double") => "showMaybe#",
                    ("Show", "show", "(Int,Int)") => "showTuple2#",
                    ("Show", "show", "(Int,String)") => "showTuple2#",
                    ("Show", "show", "(String,Int)") => "showTuple2#",
                    ("Show", "show", "(String,String)") => "showTuple2#",
                    ("Show", "show", "(Int,Bool)") => "showTuple2#",
                    ("Show", "show", "(Bool,Int)") => "showTuple2#",
                    ("Show", "show", "(Int,Double)") => "showTuple2#",
                    ("Show", "show", "(Double,Int)") => "showTuple2#",
                    ("Show", "show", "(Bool,Bool)") => "showTuple2#",
                    ("Show", "show", "(Double,Double)") => "showTuple2#",
                    ("Show", "show", "(Bool,String)") => "showTuple2#",
                    ("Show", "show", "(String,Bool)") => "showTuple2#",
                    ("Show", "show", "Either Int Int") => "showEither#",
                    ("Show", "show", "Either String Int") => "showEither#",
                    ("Show", "show", "Either Int String") => "showEither#",
                    ("Show", "show", "Either String String") => "showEither#",
                    ("Show", "show", "Ordering") => "showOrdering#",
                    ("Show", "show", tk_chirho) if tk_chirho.starts_with('(') => "showTuple2#",
                    ("Show", "show", tk_chirho) if tk_chirho.starts_with("Maybe") => "showMaybe#",
                    ("Show", "show", tk_chirho) if tk_chirho.starts_with("Either") => "showEither#",
                    ("Show", "show", tk_chirho) if tk_chirho.starts_with('[') => "showList#",
                    ("Show", "show", "Bool") => "showBool#",
                    ("Show", "show", _) => "showInt#",
                    ("Read", "read", "Int") => "readInt#",
                    ("Read", "read", "Double") => "readFloat#",
                    ("Read", "read", "Bool") => "readBool#",
                    _ => "+#", // fallback
                }
            };

        for (class_chirho, method_chirho, type_key_chirho, kind_chirho) in builtins_chirho {
            let prim_name_chirho =
                format!("$prim_{}_{}_{}",  class_chirho, method_chirho, type_key_chirho);

            // Skip if this binding already exists (user may have provided one)
            let already_exists_chirho = self
                .names_chirho
                .values()
                .any(|n_chirho| n_chirho == &prim_name_chirho);
            if already_exists_chirho {
                continue;
            }

            // Special case: Show [Int] → generate a recursive Core function
            // that pattern-matches the list and uses showInt#/++# to build the
            // string. This avoids the showList# primop which can't force thunks.
            if class_chirho == "Show" && method_chirho == "show" && type_key_chirho == "[Int]" {
                self.generate_show_list_int_binding_chirho(&prim_name_chirho);
                continue;
            }

            // Special case: Show Bool → case on True/False returning string
            if class_chirho == "Show" && method_chirho == "show" && type_key_chirho == "Bool" {
                self.generate_show_bool_binding_chirho(&prim_name_chirho);
                continue;
            }

            // Special case: Show Ordering → case on LT/EQ/GT returning string
            if class_chirho == "Show" && method_chirho == "show" && type_key_chirho == "Ordering" {
                self.generate_show_ordering_binding_chirho(&prim_name_chirho);
                continue;
            }

            // Special case: Show Either → use showEither# primop
            if class_chirho == "Show" && method_chirho == "show" && type_key_chirho.starts_with("Either") {
                // For now all Either variants use the generic showEither# primop
                // which delegates to show_value_as_string_chirho in the runtime.
            }

            // Special case: Eq Ordering → case dispatch comparing constructor tags
            if class_chirho == "Eq" && method_chirho == "==" && type_key_chirho == "Ordering" {
                self.generate_eq_ordering_binding_chirho(&prim_name_chirho);
                continue;
            }

            // Special case: Ord Ordering → case dispatch on constructor tag ordering
            if class_chirho == "Ord" && method_chirho == "compare" && type_key_chirho == "Ordering" {
                self.generate_compare_ordering_binding_chirho(&prim_name_chirho);
                continue;
            }

            let int_ty_chirho = TyChirho::int_chirho();

            let rhs_chirho = match kind_chirho {
                1 => {
                    // Binary primop: \x y -> x op# y
                    let op_chirho = primop_for_chirho(class_chirho, method_chirho, type_key_chirho);
                    let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
                    let y_chirho = self.fresh_binder_chirho("y", int_ty_chirho.clone());
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: y_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                                name_chirho: op_chirho.to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(y_chirho.id_chirho),
                                ],
                            }),
                        }),
                    }
                }
                2 => {
                    // Unary primop: \x -> op# x
                    let op_chirho = primop_for_chirho(class_chirho, method_chirho, type_key_chirho);
                    let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: op_chirho.to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(x_chirho.id_chirho),
                            ],
                        }),
                    }
                }
                _ => {
                    // Identity: \x -> x
                    let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                    }
                }
            };

            let binder_chirho = self.fresh_binder_chirho(
                &prim_name_chirho,
                int_ty_chirho,
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
    }

    /// Generate built-in Prelude function bindings that are not class
    /// methods but are essential for basic Haskell programs.
    ///
    /// Generate a recursive Core function for `show :: [Int] -> String`.
    ///
    /// Produces two top-level bindings:
    /// ```text
    /// $showListTail_Int = \xs -> case xs of
    ///     [] -> "]"
    ///     (:) x rest -> ++# "," (++# (showInt# x) ($showListTail_Int rest))
    ///
    /// $prim_Show_show_[Int] = \xs -> case xs of
    ///     [] -> "[]"
    ///     (:) x rest -> ++# "[" (++# (showInt# x) ($showListTail_Int rest))
    /// ```

    /// Generate `$prim_Show_show_Bool = \x -> showBool# x`
    fn generate_show_bool_binding_chirho(&mut self, prim_name_chirho: &str) {
        let bool_ty_chirho = TyChirho::bool_chirho();
        let str_ty_chirho = TyChirho::string_chirho();

        let x_chirho = self.fresh_binder_chirho("x", bool_ty_chirho);

        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: "showBool#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            }),
        };

        let binder_chirho = self.fresh_binder_chirho(prim_name_chirho, str_ty_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
        });
    }

    /// Generate `$prim_Show_show_Ordering = \x -> case x of LT -> "LT"; EQ -> "EQ"; GT -> "GT"`
    fn generate_show_ordering_binding_chirho(&mut self, prim_name_chirho: &str) {
        let str_ty_chirho = TyChirho::string_chirho();
        let ord_ty_chirho = TyChirho::int_chirho(); // placeholder

        let x_chirho = self.fresh_binder_chirho("x", ord_ty_chirho.clone());
        let scr_chirho = self.fresh_binder_chirho("_sord", ord_ty_chirho);

        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            bind_chirho: scr_chirho,
            result_ty_chirho: str_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "LT".to_string(),
                    )),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "EQ".to_string(),
                    )),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "GT".to_string(),
                    )),
                },
            ],
        };

        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho,
            body_chirho: Box::new(body_chirho),
        };

        let binder_chirho = self.fresh_binder_chirho(prim_name_chirho, str_ty_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
        });
    }

    /// Generate `$prim_Eq_==_Ordering = \x y -> case x of { LT -> case y of { LT -> True; _ -> False }; EQ -> case y of { EQ -> True; _ -> False }; GT -> case y of { GT -> True; _ -> False } }`
    fn generate_eq_ordering_binding_chirho(&mut self, prim_name_chirho: &str) {
        let bool_ty_chirho = TyChirho::bool_chirho();
        let ord_ty_chirho = TyChirho::int_chirho(); // placeholder

        let x_chirho = self.fresh_binder_chirho("x", ord_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("y", ord_ty_chirho.clone());
        let scr_x_chirho = self.fresh_binder_chirho("_sx", ord_ty_chirho.clone());
        let scr_y1_chirho = self.fresh_binder_chirho("_sy1", ord_ty_chirho.clone());
        let scr_y2_chirho = self.fresh_binder_chirho("_sy2", ord_ty_chirho.clone());
        let scr_y3_chirho = self.fresh_binder_chirho("_sy3", ord_ty_chirho.clone());

        let true_expr_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "True".to_string(),
            args_chirho: vec![],
        };
        let false_expr_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "False".to_string(),
            args_chirho: vec![],
        };

        // case y of { LT -> True; _ -> False }
        let inner_lt_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y1_chirho,
            result_ty_chirho: bool_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: true_expr_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: false_expr_chirho.clone(),
                },
            ],
        };

        // case y of { EQ -> True; _ -> False }
        let inner_eq_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y2_chirho,
            result_ty_chirho: bool_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: true_expr_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: false_expr_chirho.clone(),
                },
            ],
        };

        // case y of { GT -> True; _ -> False }
        let inner_gt_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y3_chirho,
            result_ty_chirho: bool_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: true_expr_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: false_expr_chirho.clone(),
                },
            ],
        };

        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            bind_chirho: scr_x_chirho,
            result_ty_chirho: bool_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_lt_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_eq_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_gt_chirho,
                },
            ],
        };

        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho,
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: y_chirho,
                body_chirho: Box::new(body_chirho),
            }),
        };

        let fn_ty_chirho = TyChirho::fun_chirho(
            ord_ty_chirho.clone(),
            TyChirho::fun_chirho(ord_ty_chirho, bool_ty_chirho),
        );
        let binder_chirho = self.fresh_binder_chirho(prim_name_chirho, fn_ty_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
        });
    }

    /// Generate `$prim_Ord_compare_Ordering = \x y -> ...` using nested case dispatch.
    /// LT < EQ < GT, so: LT vs LT -> EQ, LT vs _ -> LT, EQ vs LT -> GT, EQ vs EQ -> EQ, EQ vs GT -> LT, GT vs GT -> EQ, GT vs _ -> GT
    fn generate_compare_ordering_binding_chirho(&mut self, prim_name_chirho: &str) {
        let ord_ty_chirho = TyChirho::int_chirho(); // placeholder

        let x_chirho = self.fresh_binder_chirho("x", ord_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("y", ord_ty_chirho.clone());
        let scr_x_chirho = self.fresh_binder_chirho("_sx", ord_ty_chirho.clone());
        let scr_y1_chirho = self.fresh_binder_chirho("_sy1", ord_ty_chirho.clone());
        let scr_y2_chirho = self.fresh_binder_chirho("_sy2", ord_ty_chirho.clone());
        let scr_y3_chirho = self.fresh_binder_chirho("_sy3", ord_ty_chirho.clone());

        let lt_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "LT".to_string(),
            args_chirho: vec![],
        };
        let eq_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "EQ".to_string(),
            args_chirho: vec![],
        };
        let gt_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "GT".to_string(),
            args_chirho: vec![],
        };

        // case x of LT -> case y of { LT -> EQ; _ -> LT }
        let inner_lt_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y1_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: eq_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: lt_chirho.clone(),
                },
            ],
        };

        // case x of EQ -> case y of { LT -> GT; EQ -> EQ; GT -> LT }
        let inner_eq_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y2_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: gt_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: eq_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: lt_chirho.clone(),
                },
            ],
        };

        // case x of GT -> case y of { GT -> EQ; _ -> GT }
        let inner_gt_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y3_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: eq_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: gt_chirho.clone(),
                },
            ],
        };

        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            bind_chirho: scr_x_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_lt_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_eq_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_gt_chirho,
                },
            ],
        };

        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho,
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: y_chirho,
                body_chirho: Box::new(body_chirho),
            }),
        };

        let fn_ty_chirho = TyChirho::fun_chirho(
            ord_ty_chirho.clone(),
            TyChirho::fun_chirho(ord_ty_chirho.clone(), ord_ty_chirho),
        );
        let binder_chirho = self.fresh_binder_chirho(prim_name_chirho, fn_ty_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
        });
    }

    fn generate_show_list_int_binding_chirho(&mut self, prim_name_chirho: &str) {
        let str_ty_chirho = TyChirho::string_chirho();
        let int_ty_chirho = TyChirho::int_chirho();

        // Generate $showListTail_Int first (referenced by $prim_Show_show_[Int])
        let tail_fn_name_chirho = "$showListTail_Int";
        let tail_fn_id_chirho = self.resolve_or_fresh_id_chirho(tail_fn_name_chirho);

        // Build the tail function body
        let tail_xs_chirho = self.fresh_binder_chirho("xs", str_ty_chirho.clone());
        let tail_wild_chirho = self.fresh_binder_chirho("_w", str_ty_chirho.clone());
        let tail_x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
        let tail_rest_chirho = self.fresh_binder_chirho("rest", str_ty_chirho.clone());

        // (:) x rest -> ++# "," (++# (showInt# x) ($showListTail_Int rest))
        let tail_cons_rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "++#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(",".to_string())),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "++#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::PrimOpChirho {
                            name_chirho: "showInt#".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(
                                tail_x_chirho.id_chirho,
                            )],
                        },
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(tail_fn_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                tail_rest_chirho.id_chirho,
                            )),
                        },
                    ],
                },
            ],
        };

        let tail_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(tail_xs_chirho.id_chirho)),
            bind_chirho: tail_wild_chirho,
            result_ty_chirho: str_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "]".to_string(),
                    )),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![tail_x_chirho, tail_rest_chirho],
                    rhs_chirho: tail_cons_rhs_chirho,
                },
            ],
        };

        let tail_fn_rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: tail_xs_chirho,
            body_chirho: Box::new(tail_body_chirho),
        };

        let tail_binder_chirho = BinderChirho {
            id_chirho: tail_fn_id_chirho,
            name_chirho: tail_fn_name_chirho.to_string(),
            ty_chirho: TyChirho::fun_chirho(str_ty_chirho.clone(), str_ty_chirho.clone()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: tail_binder_chirho,
            rhs_chirho: tail_fn_rhs_chirho,
            is_rec_chirho: true, // recursive
        });

        // Now generate $prim_Show_show_[Int]
        let main_xs_chirho = self.fresh_binder_chirho("xs", str_ty_chirho.clone());
        let main_wild_chirho = self.fresh_binder_chirho("_w", str_ty_chirho.clone());
        let main_x_chirho = self.fresh_binder_chirho("x", int_ty_chirho);
        let main_rest_chirho = self.fresh_binder_chirho("rest", str_ty_chirho.clone());

        // (:) x rest -> ++# "[" (++# (showInt# x) ($showListTail_Int rest))
        let main_cons_rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "++#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::StringChirho("[".to_string())),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "++#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::PrimOpChirho {
                            name_chirho: "showInt#".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(
                                main_x_chirho.id_chirho,
                            )],
                        },
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(tail_fn_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                main_rest_chirho.id_chirho,
                            )),
                        },
                    ],
                },
            ],
        };

        let main_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(main_xs_chirho.id_chirho)),
            bind_chirho: main_wild_chirho,
            result_ty_chirho: str_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "[]".to_string(),
                    )),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![main_x_chirho, main_rest_chirho],
                    rhs_chirho: main_cons_rhs_chirho,
                },
            ],
        };

        let main_fn_rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: main_xs_chirho,
            body_chirho: Box::new(main_body_chirho),
        };

        let binder_chirho = self.fresh_binder_chirho(
            prim_name_chirho,
            TyChirho::fun_chirho(str_ty_chirho.clone(), str_ty_chirho),
        );
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho: main_fn_rhs_chirho,
            is_rec_chirho: false,
        });
    }

    /// - `not = \x -> not# x`  (boolean negation)
    /// - `id = \x -> x`        (identity)
    /// - `const = \x y -> x`   (constant function)
    pub fn generate_prelude_bindings_chirho(&mut self) {
        let bool_ty_chirho = TyChirho::bool_chirho();

        // Helper to generate a Prelude binding with ID matching.
        // Uses resolve_or_fresh_id_chirho so the binding ID matches
        // any reference the desugarer may have already created.
        let prelude_fns_chirho: Vec<(&str, TyChirho, Box<dyn FnOnce(&mut Self) -> CoreExprChirho>)> = vec![
            // not :: Bool -> Bool
            (
                "not",
                TyChirho::fun_chirho(bool_ty_chirho.clone(), bool_ty_chirho.clone()),
                Box::new(|ctx_chirho: &mut Self| {
                    let x_chirho = ctx_chirho.fresh_binder_chirho(
                        "x",
                        TyChirho::bool_chirho(),
                    );
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "not#".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(
                                x_chirho.id_chirho,
                            )],
                        }),
                    }
                }),
            ),
            // id :: a -> a
            (
                "id",
                {
                    let a_chirho = TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9990),
                    );
                    TyChirho::fun_chirho(a_chirho.clone(), a_chirho)
                },
                Box::new(|ctx_chirho: &mut Self| {
                    let a_chirho = TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9990),
                    );
                    let x_chirho = ctx_chirho.fresh_binder_chirho("x", a_chirho);
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::VarChirho(
                            x_chirho.id_chirho,
                        )),
                    }
                }),
            ),
            // const :: a -> b -> a
            (
                "const",
                {
                    let a_chirho = TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9990),
                    );
                    let b_chirho = TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9991),
                    );
                    TyChirho::fun_chirho(
                        a_chirho.clone(),
                        TyChirho::fun_chirho(b_chirho, a_chirho),
                    )
                },
                Box::new(|ctx_chirho: &mut Self| {
                    let a_chirho = TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9990),
                    );
                    let b_chirho = TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9991),
                    );
                    let x_chirho = ctx_chirho.fresh_binder_chirho("x", a_chirho);
                    let y_chirho = ctx_chirho.fresh_binder_chirho("y", b_chirho);
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: y_chirho,
                            body_chirho: Box::new(CoreExprChirho::VarChirho(
                                x_chirho.id_chirho,
                            )),
                        }),
                    }
                }),
            ),
            // flip :: (a -> b -> c) -> b -> a -> c
            (
                "flip",
                {
                    let a_chirho = TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9990),
                    );
                    let b_chirho = TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9991),
                    );
                    let c_chirho = TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9992),
                    );
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            a_chirho.clone(),
                            TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone()),
                        ),
                        TyChirho::fun_chirho(
                            b_chirho,
                            TyChirho::fun_chirho(a_chirho, c_chirho),
                        ),
                    )
                },
                Box::new(|ctx_chirho: &mut Self| {
                    let a_chirho = TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9990),
                    );
                    let b_chirho = TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9991),
                    );
                    let c_chirho = TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(9992),
                    );
                    let f_chirho = ctx_chirho.fresh_binder_chirho("f",
                        TyChirho::fun_chirho(a_chirho.clone(), TyChirho::fun_chirho(b_chirho.clone(), c_chirho)));
                    let x_chirho = ctx_chirho.fresh_binder_chirho("x", b_chirho);
                    let y_chirho = ctx_chirho.fresh_binder_chirho("y", a_chirho);
                    // flip f x y = f y x
                    CoreExprChirho::LamChirho {
                        binder_chirho: f_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: x_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::LamChirho {
                                binder_chirho: y_chirho.clone(),
                                body_chirho: Box::new(CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                                        arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
                                    }),
                                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                                }),
                            }),
                        }),
                    }
                }),
            ),
        ];

        for (name_chirho, ty_chirho, make_rhs_chirho) in prelude_fns_chirho {
            let id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            let rhs_chirho = make_rhs_chirho(self);
            let binder_chirho = BinderChirho {
                id_chirho,
                name_chirho: name_chirho.to_string(),
                ty_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // even :: Int -> Bool
        // even n = n `mod` 2 == 0
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let even_id_chirho = self.resolve_or_fresh_id_chirho("even");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());

            let mod_result_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "mod#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                ],
            };
            let eq_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "==#".to_string(),
                args_chirho: vec![
                    mod_result_chirho,
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(eq_zero_chirho),
            };

            let binder_chirho = BinderChirho {
                id_chirho: even_id_chirho,
                name_chirho: "even".to_string(),
                ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), TyChirho::bool_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // odd :: Int -> Bool
        // odd n = n `mod` 2 /= 0
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let odd_id_chirho = self.resolve_or_fresh_id_chirho("odd");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());

            let mod_result_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "mod#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                ],
            };
            let ne_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "/=#".to_string(),
                args_chirho: vec![
                    mod_result_chirho,
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(ne_zero_chirho),
            };

            let binder_chirho = BinderChirho {
                id_chirho: odd_id_chirho,
                name_chirho: "odd".to_string(),
                ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), TyChirho::bool_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // abs :: Int -> Int
        // abs n = if n < 0 then negate n else n
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let abs_id_chirho = self.resolve_or_fresh_id_chirho("abs");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());

            let lt_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "<#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                ],
            };
            let negated_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "negate#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
            };

            let case_wild_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(lt_zero_chirho),
                bind_chirho: case_wild_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: negated_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(body_chirho),
            };

            let binder_chirho = BinderChirho {
                id_chirho: abs_id_chirho,
                name_chirho: "abs".to_string(),
                ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho: rhs_chirho.clone(),
                is_rec_chirho: false,
            });

            // Also generate $prim_Num_abs_Int for the instance dictionary
            let prim_abs_name_chirho = "$prim_Num_abs_Int";
            let prim_abs_id_chirho = self.resolve_or_fresh_id_chirho(prim_abs_name_chirho);
            let prim_abs_binder_chirho = BinderChirho {
                id_chirho: prim_abs_id_chirho,
                name_chirho: prim_abs_name_chirho.to_string(),
                ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: prim_abs_binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // max :: Int -> Int -> Int
        // max a b = if a >= b then a else b
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let max_id_chirho = self.resolve_or_fresh_id_chirho("max");
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());

            let ge_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: ">=#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                ],
            };

            let case_wild_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(ge_chirho),
                bind_chirho: case_wild_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(b_chirho.id_chirho),
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            let binder_chirho = BinderChirho {
                id_chirho: max_id_chirho,
                name_chirho: "max".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    int_ty_chirho.clone(),
                    TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // min :: Int -> Int -> Int
        // min a b = if a <= b then a else b
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let min_id_chirho = self.resolve_or_fresh_id_chirho("min");
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());

            let le_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "<=#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                ],
            };

            let case_wild_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(le_chirho),
                bind_chirho: case_wild_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(b_chirho.id_chirho),
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            let binder_chirho = BinderChirho {
                id_chirho: min_id_chirho,
                name_chirho: "min".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    int_ty_chirho.clone(),
                    TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // fst :: (a, b) -> a
        {
            let a_chirho = TyChirho::VarChirho(
                rhasky_typing_chirho::ty_chirho::TyVarChirho(9990),
            );
            let b_chirho = TyChirho::VarChirho(
                rhasky_typing_chirho::ty_chirho::TyVarChirho(9991),
            );
            let pair_ty_chirho = TyChirho::TupleChirho(vec![a_chirho.clone(), b_chirho.clone()]);
            let fst_id_chirho = self.resolve_or_fresh_id_chirho("fst");
            let p_chirho = self.fresh_binder_chirho("p", pair_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("$w", pair_ty_chirho.clone()),
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![x_chirho.clone(), y_chirho],
                    rhs_chirho: CoreExprChirho::VarChirho(x_chirho.id_chirho),
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(body_chirho),
            };

            let binder_chirho = BinderChirho {
                id_chirho: fst_id_chirho,
                name_chirho: "fst".to_string(),
                ty_chirho: TyChirho::fun_chirho(pair_ty_chirho.clone(), a_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // snd :: (a, b) -> b
        {
            let a_chirho = TyChirho::VarChirho(
                rhasky_typing_chirho::ty_chirho::TyVarChirho(9990),
            );
            let b_chirho = TyChirho::VarChirho(
                rhasky_typing_chirho::ty_chirho::TyVarChirho(9991),
            );
            let pair_ty_chirho = TyChirho::TupleChirho(vec![a_chirho.clone(), b_chirho.clone()]);
            let snd_id_chirho = self.resolve_or_fresh_id_chirho("snd");
            let p_chirho = self.fresh_binder_chirho("p", pair_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("$w", pair_ty_chirho.clone()),
                result_ty_chirho: b_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![x_chirho, y_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(y_chirho.id_chirho),
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(body_chirho),
            };

            let binder_chirho = BinderChirho {
                id_chirho: snd_id_chirho,
                name_chirho: "snd".to_string(),
                ty_chirho: TyChirho::fun_chirho(pair_ty_chirho, b_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // curry :: ((a, b) -> c) -> a -> b -> c
        // curry f a b = f (a, b)
        {
            let a_chirho = TyChirho::VarChirho(
                rhasky_typing_chirho::ty_chirho::TyVarChirho(9990),
            );
            let b_chirho = TyChirho::VarChirho(
                rhasky_typing_chirho::ty_chirho::TyVarChirho(9991),
            );
            let c_chirho = TyChirho::VarChirho(
                rhasky_typing_chirho::ty_chirho::TyVarChirho(9992),
            );
            let pair_ty_chirho = TyChirho::TupleChirho(vec![a_chirho.clone(), b_chirho.clone()]);
            let curry_id_chirho = self.resolve_or_fresh_id_chirho("curry");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(pair_ty_chirho.clone(), c_chirho.clone()),
            );
            let a_binder_chirho = self.fresh_binder_chirho("a", a_chirho.clone());
            let b_binder_chirho = self.fresh_binder_chirho("b", b_chirho.clone());

            // f (a, b)
            let tuple_expr_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(a_binder_chirho.id_chirho),
                    CoreExprChirho::VarChirho(b_binder_chirho.id_chirho),
                ],
            };
            let app_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(tuple_expr_chirho),
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: a_binder_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: b_binder_chirho,
                        body_chirho: Box::new(app_chirho),
                    }),
                }),
            };

            let binder_chirho = BinderChirho {
                id_chirho: curry_id_chirho,
                name_chirho: "curry".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(pair_ty_chirho.clone(), c_chirho.clone()),
                    TyChirho::fun_chirho(
                        a_chirho.clone(),
                        TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone()),
                    ),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // uncurry :: (a -> b -> c) -> (a, b) -> c
        // uncurry f (a, b) = f a b
        {
            let a_chirho = TyChirho::VarChirho(
                rhasky_typing_chirho::ty_chirho::TyVarChirho(9990),
            );
            let b_chirho = TyChirho::VarChirho(
                rhasky_typing_chirho::ty_chirho::TyVarChirho(9991),
            );
            let c_chirho = TyChirho::VarChirho(
                rhasky_typing_chirho::ty_chirho::TyVarChirho(9992),
            );
            let pair_ty_chirho = TyChirho::TupleChirho(vec![a_chirho.clone(), b_chirho.clone()]);
            let uncurry_id_chirho = self.resolve_or_fresh_id_chirho("uncurry");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(
                    a_chirho.clone(),
                    TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone()),
                ),
            );
            let p_chirho = self.fresh_binder_chirho("p", pair_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());

            // case p of { (x, y) -> f x y }
            let f_applied_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("$w", pair_ty_chirho.clone()),
                result_ty_chirho: c_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![x_chirho, y_chirho],
                    rhs_chirho: f_applied_chirho,
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: p_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            let binder_chirho = BinderChirho {
                id_chirho: uncurry_id_chirho,
                name_chirho: "uncurry".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        a_chirho,
                        TyChirho::fun_chirho(b_chirho, c_chirho.clone()),
                    ),
                    TyChirho::fun_chirho(pair_ty_chirho, c_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // enumFromTo :: Int -> Int -> [Int]
        // enumFromTo from to = if from > to then [] else from : enumFromTo (from+1) to
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let enum_id_chirho = self.resolve_or_fresh_id_chirho("enumFromTo");
            let from_chirho = self.fresh_binder_chirho("from", int_ty_chirho.clone());
            let to_chirho = self.fresh_binder_chirho("to", int_ty_chirho.clone());

            // from > to
            let cond_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: ">#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(from_chirho.id_chirho),
                    CoreExprChirho::VarChirho(to_chirho.id_chirho),
                ],
            };

            // from + 1
            let from_plus_one_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(from_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                ],
            };

            // enumFromTo (from+1) to
            let recursive_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(enum_id_chirho)),
                    arg_chirho: Box::new(from_plus_one_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(to_chirho.id_chirho)),
            };

            // from : enumFromTo (from+1) to
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(from_chirho.id_chirho),
                    recursive_call_chirho,
                ],
            };

            // []
            let nil_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "[]".to_string(),
                args_chirho: vec![],
            };

            // case (from > to) of { True -> []; _ -> from : ... }
            let case_wild_chirho = self.fresh_binder_chirho(
                "$w",
                TyChirho::bool_chirho(),
            );
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(cond_chirho),
                bind_chirho: case_wild_chirho,
                result_ty_chirho: TyChirho::VarChirho(
                    rhasky_typing_chirho::ty_chirho::TyVarChirho(9998),
                ),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: from_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: to_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            let list_ty_chirho = TyChirho::ListChirho(Box::new(int_ty_chirho.clone()));
            let enum_binder_chirho = BinderChirho {
                id_chirho: enum_id_chirho,
                name_chirho: "enumFromTo".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    int_ty_chirho.clone(),
                    TyChirho::fun_chirho(int_ty_chirho, list_ty_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: enum_binder_chirho,
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // take :: Int -> [a] -> [a]
        // take n xs = case n <=# 0 of { True -> []; False -> case xs of { [] -> []; x:rest -> x : take (n -# 1) rest } }
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let list_ty_chirho = TyChirho::string_chirho(); // placeholder type
            let take_id_chirho = self.resolve_or_fresh_id_chirho("take");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_ty_chirho.clone());
            let wild1_chirho = self.fresh_binder_chirho("$wild", TyChirho::bool_chirho());
            let wild2_chirho = self.fresh_binder_chirho("$wild", list_ty_chirho.clone());

            // n -# 1
            let n_minus_1_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "-#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                ],
            };
            // take (n-1) rest
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(take_id_chirho)),
                    arg_chirho: Box::new(n_minus_1_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            // x : take (n-1) rest
            let cons_result_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    rec_call_chirho,
                ],
            };
            // Inner case: case xs of { [] -> []; x:rest -> x : take (n-1) rest }
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: wild2_chirho,
                result_ty_chirho: list_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho.clone(), rest_chirho.clone()],
                        rhs_chirho: cons_result_chirho,
                    },
                ],
            };
            // Outer case: case n <=# 0 of { True -> []; False -> inner }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<=#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(n_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: wild1_chirho,
                result_ty_chirho: list_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            let binder_chirho = BinderChirho {
                id_chirho: take_id_chirho,
                name_chirho: "take".to_string(),
                ty_chirho: int_ty_chirho.clone(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // drop :: Int -> [a] -> [a]
        // drop n xs = case n <=# 0 of { True -> xs; False -> case xs of { [] -> []; _:rest -> drop (n-1) rest } }
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let list_ty_chirho = TyChirho::string_chirho(); // placeholder type
            let drop_id_chirho = self.resolve_or_fresh_id_chirho("drop");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_ty_chirho.clone());
            let _x_chirho = self.fresh_binder_chirho("_x", int_ty_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_ty_chirho.clone());
            let wild1_chirho = self.fresh_binder_chirho("$wild", TyChirho::bool_chirho());
            let wild2_chirho = self.fresh_binder_chirho("$wild", list_ty_chirho.clone());

            // n -# 1
            let n_minus_1_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "-#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                ],
            };
            // drop (n-1) rest
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(drop_id_chirho)),
                    arg_chirho: Box::new(n_minus_1_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            // Inner case: case xs of { [] -> []; _:rest -> drop (n-1) rest }
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: wild2_chirho,
                result_ty_chirho: list_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![_x_chirho.clone(), rest_chirho.clone()],
                        rhs_chirho: rec_call_chirho,
                    },
                ],
            };
            // Outer case: case n <=# 0 of { True -> xs; False -> inner }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<=#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(n_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: wild1_chirho,
                result_ty_chirho: list_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(xs_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            let binder_chirho = BinderChirho {
                id_chirho: drop_id_chirho,
                name_chirho: "drop".to_string(),
                ty_chirho: int_ty_chirho.clone(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // words :: String -> [String]
        // words s = wordsStr# s
        {
            let str_ty_chirho = TyChirho::string_chirho();
            let list_str_ty_chirho = TyChirho::ListChirho(Box::new(str_ty_chirho.clone()));
            let words_id_chirho = self.resolve_or_fresh_id_chirho("words");
            let s_chirho = self.fresh_binder_chirho("s", str_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "wordsStr#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: words_id_chirho,
                name_chirho: "words".to_string(),
                ty_chirho: TyChirho::fun_chirho(str_ty_chirho, list_str_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // unwords :: [String] -> String
        // unwords xs = unwordsStr# xs
        {
            let str_ty_chirho = TyChirho::string_chirho();
            let list_str_ty_chirho = TyChirho::ListChirho(Box::new(str_ty_chirho.clone()));
            let unwords_id_chirho = self.resolve_or_fresh_id_chirho("unwords");
            let xs_chirho = self.fresh_binder_chirho("xs", list_str_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "unwordsStr#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(xs_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: unwords_id_chirho,
                name_chirho: "unwords".to_string(),
                ty_chirho: TyChirho::fun_chirho(list_str_ty_chirho, str_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // lines :: String -> [String]
        // lines s = lines# s
        {
            let str_ty_chirho = TyChirho::string_chirho();
            let list_str_ty_chirho = TyChirho::ListChirho(Box::new(str_ty_chirho.clone()));
            let lines_id_chirho = self.resolve_or_fresh_id_chirho("lines");
            let s_chirho = self.fresh_binder_chirho("s", str_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "lines#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: lines_id_chirho,
                name_chirho: "lines".to_string(),
                ty_chirho: TyChirho::fun_chirho(str_ty_chirho, list_str_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // unlines :: [String] -> String
        // unlines xs = unlines# xs
        {
            let str_ty_chirho = TyChirho::string_chirho();
            let list_str_ty_chirho = TyChirho::ListChirho(Box::new(str_ty_chirho.clone()));
            let unlines_id_chirho = self.resolve_or_fresh_id_chirho("unlines");
            let xs_chirho = self.fresh_binder_chirho("xs", list_str_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "unlines#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(xs_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: unlines_id_chirho,
                name_chirho: "unlines".to_string(),
                ty_chirho: TyChirho::fun_chirho(list_str_ty_chirho, str_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // concat :: [String] -> String
        // concat xs = concatStr# xs
        {
            let str_ty_chirho = TyChirho::string_chirho();
            let list_str_ty_chirho = TyChirho::ListChirho(Box::new(str_ty_chirho.clone()));
            let concat_id_chirho = self.resolve_or_fresh_id_chirho("concat");
            let xs_chirho = self.fresh_binder_chirho("xs", list_str_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "concatStr#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(xs_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: concat_id_chirho,
                name_chirho: "concat".to_string(),
                ty_chirho: TyChirho::fun_chirho(list_str_ty_chirho, str_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // intercalate :: String -> [String] -> String
        // intercalate sep xs = intercalateStr# sep xs
        {
            let str_ty_chirho = TyChirho::string_chirho();
            let list_str_ty_chirho = TyChirho::ListChirho(Box::new(str_ty_chirho.clone()));
            let intercalate_id_chirho = self.resolve_or_fresh_id_chirho("intercalate");
            let sep_chirho = self.fresh_binder_chirho("sep", str_ty_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_str_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "intercalateStr#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(sep_chirho.id_chirho),
                    CoreExprChirho::VarChirho(xs_chirho.id_chirho),
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: sep_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            let binder_chirho = BinderChirho {
                id_chirho: intercalate_id_chirho,
                name_chirho: "intercalate".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    str_ty_chirho.clone(),
                    TyChirho::fun_chirho(list_str_ty_chirho, str_ty_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // toInteger :: Int -> Int  (identity)
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let to_integer_id_chirho = self.resolve_or_fresh_id_chirho("toInteger");
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let binder_chirho = BinderChirho {
                id_chirho: to_integer_id_chirho,
                name_chirho: "toInteger".to_string(),
                ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // fromIntegral :: Int -> Double
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let double_ty_chirho = TyChirho::double_chirho();
            let from_integral_id_chirho = self.resolve_or_fresh_id_chirho("fromIntegral");
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "fromIntegral#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: from_integral_id_chirho,
                name_chirho: "fromIntegral".to_string(),
                ty_chirho: TyChirho::fun_chirho(int_ty_chirho, double_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ceiling :: Double -> Int
        // floor :: Double -> Int
        // round :: Double -> Int
        // truncate :: Double -> Int
        for (name_chirho, primop_chirho) in [
            ("ceiling", "ceiling#"),
            ("floor", "floor#"),
            ("round", "round#"),
            ("truncate", "truncate#"),
        ] {
            let double_ty_chirho = TyChirho::double_chirho();
            let int_ty_chirho = TyChirho::int_chirho();
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", double_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: primop_chirho.to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: fn_id_chirho,
                name_chirho: name_chirho.to_string(),
                ty_chirho: TyChirho::fun_chirho(double_ty_chirho, int_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // isJust :: Maybe a -> Bool
        // isJust m = case m of { Nothing -> False; Just _ -> True }
        {
            let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9900));
            let maybe_a_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(a_chirho.clone()),
            );
            let is_just_id_chirho = self.resolve_or_fresh_id_chirho("isJust");
            let m_chirho = self.fresh_binder_chirho("m", maybe_a_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: is_just_id_chirho,
                name_chirho: "isJust".to_string(),
                ty_chirho: TyChirho::fun_chirho(maybe_a_chirho.clone(), TyChirho::bool_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // isNothing :: Maybe a -> Bool
        // isNothing m = case m of { Nothing -> True; Just _ -> False }
        {
            let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9901));
            let maybe_a_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(a_chirho.clone()),
            );
            let is_nothing_id_chirho = self.resolve_or_fresh_id_chirho("isNothing");
            let m_chirho = self.fresh_binder_chirho("m", maybe_a_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: is_nothing_id_chirho,
                name_chirho: "isNothing".to_string(),
                ty_chirho: TyChirho::fun_chirho(maybe_a_chirho, TyChirho::bool_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // fromMaybe :: a -> Maybe a -> a
        // fromMaybe def m = case m of { Nothing -> def; Just x -> x }
        {
            let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9902));
            let maybe_a_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(a_chirho.clone()),
            );
            let from_maybe_id_chirho = self.resolve_or_fresh_id_chirho("fromMaybe");
            let def_chirho = self.fresh_binder_chirho("def", a_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", maybe_a_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(def_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho.clone()],
                        rhs_chirho: CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: def_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            let binder_chirho = BinderChirho {
                id_chirho: from_maybe_id_chirho,
                name_chirho: "fromMaybe".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    a_chirho.clone(),
                    TyChirho::fun_chirho(maybe_a_chirho, a_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // maybe :: b -> (a -> b) -> Maybe a -> b
        // maybe def f m = case m of { Nothing -> def; Just x -> f x }
        {
            let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9903));
            let b_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9904));
            let maybe_a_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(a_chirho.clone()),
            );
            let maybe_id_chirho = self.resolve_or_fresh_id_chirho("maybe");
            let def_chirho = self.fresh_binder_chirho("def", b_chirho.clone());
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()));
            let m_chirho = self.fresh_binder_chirho("m", maybe_a_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(def_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho.clone()],
                        rhs_chirho: CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: def_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };
            let binder_chirho = BinderChirho {
                id_chirho: maybe_id_chirho,
                name_chirho: "maybe".to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        b_chirho.clone(),
                        TyChirho::fun_chirho(a_chirho, b_chirho.clone()),
                        maybe_a_chirho,
                    ],
                    b_chirho,
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // either :: (a -> c) -> (b -> c) -> Either a b -> c
        // either f g e = case e of { Left x -> f x; Right y -> g y }
        {
            let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9905));
            let b_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9906));
            let c_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9907));
            let either_ab_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Either".to_string())),
                    Box::new(a_chirho.clone()),
                )),
                Box::new(b_chirho.clone()),
            );
            let either_id_chirho = self.resolve_or_fresh_id_chirho("either");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(a_chirho.clone(), c_chirho.clone()));
            let g_chirho = self.fresh_binder_chirho("g", TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone()));
            let e_chirho = self.fresh_binder_chirho("e", either_ab_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: c_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Left".to_string()),
                        binders_chirho: vec![x_chirho.clone()],
                        rhs_chirho: CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Right".to_string()),
                        binders_chirho: vec![y_chirho.clone()],
                        rhs_chirho: CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(g_chirho.id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: g_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: e_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };
            let binder_chirho = BinderChirho {
                id_chirho: either_id_chirho,
                name_chirho: "either".to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(a_chirho, c_chirho.clone()),
                        TyChirho::fun_chirho(b_chirho, c_chirho.clone()),
                        either_ab_chirho,
                    ],
                    c_chirho,
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── min/max :: Int -> Int -> Int (via <=# primop) ─────────────
        {
            let int_chirho = TyChirho::ConChirho("Int".to_string());
            for (fn_name_chirho, pick_first_chirho) in [("min", true), ("max", false)] {
                let x_id_chirho = self.fresh_id_chirho("x");
                let y_id_chirho = self.fresh_id_chirho("y");
                let fn_id_chirho = self.resolve_or_fresh_id_chirho(fn_name_chirho);
                let wild_id_chirho = self.fresh_id_chirho("$wild");
                let x_chirho = BinderChirho {
                    id_chirho: x_id_chirho, name_chirho: "x".to_string(),
                    ty_chirho: int_chirho.clone(), span_chirho: SpanChirho::DUMMY_CHIRHO,
                };
                let y_chirho = BinderChirho {
                    id_chirho: y_id_chirho, name_chirho: "y".to_string(),
                    ty_chirho: int_chirho.clone(), span_chirho: SpanChirho::DUMMY_CHIRHO,
                };
                let wild_chirho = BinderChirho {
                    id_chirho: wild_id_chirho, name_chirho: "$wild".to_string(),
                    ty_chirho: TyChirho::bool_chirho(), span_chirho: SpanChirho::DUMMY_CHIRHO,
                };
                // case x <=# y of True -> <first>; False -> <second>
                let (true_rhs_chirho, false_rhs_chirho) = if pick_first_chirho {
                    // min: True → x, False → y
                    (CoreExprChirho::VarChirho(x_id_chirho), CoreExprChirho::VarChirho(y_id_chirho))
                } else {
                    // max: True → y, False → x
                    (CoreExprChirho::VarChirho(y_id_chirho), CoreExprChirho::VarChirho(x_id_chirho))
                };
                let body_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "<=#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_id_chirho),
                            CoreExprChirho::VarChirho(y_id_chirho),
                        ],
                    }),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: int_chirho.clone(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![], rhs_chirho: true_rhs_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("False".to_string()),
                            binders_chirho: vec![], rhs_chirho: false_rhs_chirho,
                        },
                    ],
                };
                let rhs_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: y_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                };
                let binder_chirho = BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: fn_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![int_chirho.clone(), int_chirho.clone()],
                        int_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                };
                self.generated_bindings_chirho.push(CoreBindingChirho {
                    binder_chirho, rhs_chirho, is_rec_chirho: false,
                });
            }
        }

        // ── Higher-order list functions ──────────────────────────────────
        self.generate_list_prelude_chirho();
    }

    /// Generate higher-order list Prelude functions: map, filter, foldr, foldl,
    /// head, tail, null, length, reverse, concatMap, zip, zipWith, sum, product.
    fn generate_list_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9991));
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));
        let list_b_chirho = TyChirho::ListChirho(Box::new(b_chirho.clone()));

        let nil_chirho = || CoreExprChirho::ConAppChirho {
            con_name_chirho: "[]".to_string(),
            args_chirho: vec![],
        };

        // map :: (a -> b) -> [a] -> [b]
        // map f [] = []; map f (x:xs) = f x : map f xs
        {
            let map_id_chirho = self.resolve_or_fresh_id_chirho("map");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let f_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let map_f_t_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(map_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![f_h_chirho, map_f_t_chirho],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: map_id_chirho,
                    name_chirho: "map".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
                        TyChirho::fun_chirho(list_a_chirho.clone(), list_b_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // filter :: (a -> Bool) -> [a] -> [a]
        // filter p [] = []; filter p (x:xs) = if p x then x : filter p xs else filter p xs
        {
            let filter_id_chirho = self.resolve_or_fresh_id_chirho("filter");
            let p_chirho = self.fresh_binder_chirho("p", TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let cw_chirho = self.fresh_binder_chirho("$cw", TyChirho::bool_chirho());

            let filter_p_t_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(filter_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    filter_p_t_chirho.clone(),
                ],
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let if_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: cw_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: filter_p_t_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: if_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: filter_id_chirho,
                    name_chirho: "filter".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
                        TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // foldr :: (a -> b -> b) -> b -> [a] -> b
        // foldr f z [] = z; foldr f z (x:xs) = f x (foldr f z xs)
        {
            let foldr_id_chirho = self.resolve_or_fresh_id_chirho("foldr");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(a_chirho.clone(), TyChirho::fun_chirho(b_chirho.clone(), b_chirho.clone())));
            let z_chirho = self.fresh_binder_chirho("z", b_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let foldr_f_z_t_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldr_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(z_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let f_h_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(foldr_f_z_t_chirho),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(z_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: f_h_rec_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: xs_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: foldr_id_chirho,
                    name_chirho: "foldr".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), TyChirho::fun_chirho(b_chirho.clone(), b_chirho.clone())),
                        TyChirho::fun_chirho(b_chirho.clone(), TyChirho::fun_chirho(list_a_chirho.clone(), b_chirho.clone())),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // foldl :: (b -> a -> b) -> b -> [a] -> b
        // foldl f z [] = z; foldl f z (x:xs) = foldl f (f z x) xs
        {
            let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(b_chirho.clone(), TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone())));
            let z_chirho = self.fresh_binder_chirho("z", b_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let f_z_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(z_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let foldl_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldl_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(f_z_h_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(z_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: foldl_rec_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: xs_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: foldl_id_chirho,
                    name_chirho: "foldl".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(b_chirho.clone(), TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone())),
                        TyChirho::fun_chirho(b_chirho.clone(), TyChirho::fun_chirho(list_a_chirho.clone(), b_chirho.clone())),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // head :: [a] -> a
        // head (x:_) = x
        {
            let head_id_chirho = self.resolve_or_fresh_id_chirho("head");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![h_chirho.clone(), t_chirho],
                    rhs_chirho: CoreExprChirho::VarChirho(h_chirho.id_chirho),
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: head_id_chirho,
                    name_chirho: "head".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // tail :: [a] -> [a]
        // tail (_:xs) = xs
        {
            let tail_id_chirho = self.resolve_or_fresh_id_chirho("tail");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![h_chirho, t_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(t_chirho.id_chirho),
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: tail_id_chirho,
                    name_chirho: "tail".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // null :: [a] -> Bool
        // null [] = True; null _ = False
        {
            let null_id_chirho = self.resolve_or_fresh_id_chirho("null");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: null_id_chirho,
                    name_chirho: "null".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), TyChirho::bool_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // length :: [a] -> Int
        // length [] = 0; length (_:xs) = 1 + length xs
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let length_id_chirho = self.resolve_or_fresh_id_chirho("length");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let length_t_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(length_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let one_plus_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    length_t_chirho,
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: one_plus_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: length_id_chirho,
                    name_chirho: "length".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), int_ty_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // reverse :: [a] -> [a]
        // reverse [] = []; reverse (x:xs) = reverse xs ++ [x]
        // (implemented as foldl with flip cons)
        {
            let reverse_id_chirho = self.resolve_or_fresh_id_chirho("reverse");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let _h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let _t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let _w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let _acc_chirho = self.fresh_binder_chirho("acc", list_a_chirho.clone());

            // reverse xs = go [] xs where go acc [] = acc; go acc (h:t) = go (h:acc) t
            let go_id_chirho = {
                let id_chirho = CoreIdChirho(self.next_id_chirho);
                self.next_id_chirho += 1;
                self.names_chirho.insert(id_chirho, "$rev_go".to_string());
                id_chirho
            };

            let go_acc_chirho = self.fresh_binder_chirho("acc", list_a_chirho.clone());
            let go_ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let go_h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let go_t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let go_w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let h_cons_acc_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(go_h_chirho.id_chirho),
                    CoreExprChirho::VarChirho(go_acc_chirho.id_chirho),
                ],
            };
            let go_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(go_id_chirho)),
                    arg_chirho: Box::new(h_cons_acc_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(go_t_chirho.id_chirho)),
            };

            let go_body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(go_ys_chirho.id_chirho)),
                bind_chirho: go_w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(go_acc_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![go_h_chirho, go_t_chirho],
                        rhs_chirho: go_rec_chirho,
                    },
                ],
            };

            let go_lam_chirho = CoreExprChirho::LamChirho {
                binder_chirho: go_acc_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: go_ys_chirho,
                    body_chirho: Box::new(go_body_chirho),
                }),
            };

            let go_binder_chirho = BinderChirho {
                id_chirho: go_id_chirho,
                name_chirho: "$rev_go".to_string(),
                ty_chirho: list_a_chirho.clone(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };

            let rev_body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: true,
                binds_chirho: vec![(go_binder_chirho, go_lam_chirho)],
                body_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(go_id_chirho)),
                        arg_chirho: Box::new(nil_chirho()),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                }),
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(rev_body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: reverse_id_chirho,
                    name_chirho: "reverse".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // zip :: [a] -> [b] -> [(a,b)]
        // zip (a:as) (b:bs) = (a,b) : zip as bs; zip _ _ = []
        {
            let pair_ty_chirho = TyChirho::TupleChirho(vec![a_chirho.clone(), b_chirho.clone()]);
            let list_pair_chirho = TyChirho::ListChirho(Box::new(pair_ty_chirho.clone()));
            let zip_id_chirho = self.resolve_or_fresh_id_chirho("zip");
            let as_chirho = self.fresh_binder_chirho("as", list_a_chirho.clone());
            let bs_chirho = self.fresh_binder_chirho("bs", list_b_chirho.clone());
            let ah_chirho = self.fresh_binder_chirho("a", a_chirho.clone());
            let at_chirho = self.fresh_binder_chirho("at", list_a_chirho.clone());
            let bh_chirho = self.fresh_binder_chirho("b", b_chirho.clone());
            let bt_chirho = self.fresh_binder_chirho("bt", list_b_chirho.clone());
            let wa_chirho = self.fresh_binder_chirho("$wa", list_a_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", list_b_chirho.clone());

            let pair_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(ah_chirho.id_chirho),
                    CoreExprChirho::VarChirho(bh_chirho.id_chirho),
                ],
            };
            let zip_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(zip_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(at_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(bt_chirho.id_chirho)),
            };
            let cons_pair_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![pair_chirho, zip_rec_chirho],
            };

            // Inner case: case bs of { [] -> []; (b:bt) -> (a,b) : zip at bt }
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(bs_chirho.id_chirho)),
                bind_chirho: wb_chirho,
                result_ty_chirho: list_pair_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![bh_chirho, bt_chirho],
                        rhs_chirho: cons_pair_chirho,
                    },
                ],
            };

            // Outer case: case as of { [] -> []; (a:at) -> <inner_case> }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(as_chirho.id_chirho)),
                bind_chirho: wa_chirho,
                result_ty_chirho: list_pair_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![ah_chirho, at_chirho],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: as_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: bs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: zip_id_chirho,
                    name_chirho: "zip".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        list_a_chirho.clone(),
                        TyChirho::fun_chirho(list_b_chirho.clone(), list_pair_chirho),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // zipWith :: (a -> b -> c) -> [a] -> [b] -> [c]
        {
            let c_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9992));
            let list_c_chirho = TyChirho::ListChirho(Box::new(c_chirho.clone()));
            let zipw_id_chirho = self.resolve_or_fresh_id_chirho("zipWith");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(a_chirho.clone(), TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone())));
            let as_chirho = self.fresh_binder_chirho("as", list_a_chirho.clone());
            let bs_chirho = self.fresh_binder_chirho("bs", list_b_chirho.clone());
            let ah_chirho = self.fresh_binder_chirho("a", a_chirho.clone());
            let at_chirho = self.fresh_binder_chirho("at", list_a_chirho.clone());
            let bh_chirho = self.fresh_binder_chirho("b", b_chirho.clone());
            let bt_chirho = self.fresh_binder_chirho("bt", list_b_chirho.clone());
            let wa_chirho = self.fresh_binder_chirho("$wa", list_a_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", list_b_chirho.clone());

            let f_a_b_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(ah_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(bh_chirho.id_chirho)),
            };
            let zipw_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(zipw_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(at_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(bt_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![f_a_b_chirho, zipw_rec_chirho],
            };

            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(bs_chirho.id_chirho)),
                bind_chirho: wb_chirho,
                result_ty_chirho: list_c_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![bh_chirho, bt_chirho],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(as_chirho.id_chirho)),
                bind_chirho: wa_chirho,
                result_ty_chirho: list_c_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![ah_chirho, at_chirho],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: as_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: bs_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: zipw_id_chirho,
                    name_chirho: "zipWith".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone())),
                        TyChirho::fun_chirho(list_a_chirho.clone(), TyChirho::fun_chirho(list_b_chirho.clone(), list_c_chirho)),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // append :: [a] -> [a] -> [a]
        // append [] ys = ys; append (x:xs) ys = x : append xs ys
        {
            let append_id_chirho = self.resolve_or_fresh_id_chirho("append");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let append_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    append_rec_chirho,
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(ys_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ys_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: append_id_chirho,
                    name_chirho: "append".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone())),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // any :: (a -> Bool) -> [a] -> Bool
        // any p [] = False; any p (x:xs) = case p x of { True -> True; False -> any p xs }
        {
            let any_id_chirho = self.resolve_or_fresh_id_chirho("any");
            let p_chirho = self.fresh_binder_chirho("p", TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let cw_chirho = self.fresh_binder_chirho("$cw", TyChirho::bool_chirho());

            let any_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(any_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: cw_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: any_rec_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: any_id_chirho,
                    name_chirho: "any".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
                        TyChirho::fun_chirho(list_a_chirho.clone(), TyChirho::bool_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // all :: (a -> Bool) -> [a] -> Bool
        // all p [] = True; all p (x:xs) = case p x of { True -> all p xs; False -> False }
        {
            let all_id_chirho = self.resolve_or_fresh_id_chirho("all");
            let p_chirho = self.fresh_binder_chirho("p", TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let cw_chirho = self.fresh_binder_chirho("$cw", TyChirho::bool_chirho());

            let all_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(all_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: cw_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: all_rec_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: all_id_chirho,
                    name_chirho: "all".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
                        TyChirho::fun_chirho(list_a_chirho.clone(), TyChirho::bool_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // sum :: [Int] -> Int  (Int-specialized)
        // sum [] = 0; sum (x:xs) = x +# sum xs
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let list_int_chirho = TyChirho::ListChirho(Box::new(int_ty_chirho.clone()));
            let sum_id_chirho = self.resolve_or_fresh_id_chirho("sum");
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", int_ty_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_int_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_int_chirho.clone());

            let sum_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(sum_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let add_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    sum_rec_chirho,
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: add_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: sum_id_chirho,
                    name_chirho: "sum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_int_chirho, int_ty_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // product :: [Int] -> Int  (Int-specialized)
        // product [] = 1; product (x:xs) = x *# product xs
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let list_int_chirho = TyChirho::ListChirho(Box::new(int_ty_chirho.clone()));
            let product_id_chirho = self.resolve_or_fresh_id_chirho("product");
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", int_ty_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_int_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_int_chirho.clone());

            let product_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(product_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let mul_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "*#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    product_rec_chirho,
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: mul_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: product_id_chirho,
                    name_chirho: "product".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_int_chirho, int_ty_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // concatMap :: (a -> [b]) -> [a] -> [b]
        // concatMap f [] = []; concatMap f (x:xs) = append (f x) (concatMap f xs)
        {
            let concatmap_id_chirho = self.resolve_or_fresh_id_chirho("concatMap");
            let append_id_chirho = self.resolve_or_fresh_id_chirho("append");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(a_chirho.clone(), list_b_chirho.clone()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let f_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let concatmap_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(concatmap_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let append_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(f_h_chirho),
                }),
                arg_chirho: Box::new(concatmap_rec_chirho),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: append_call_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: concatmap_id_chirho,
                    name_chirho: "concatMap".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), list_b_chirho.clone()),
                        TyChirho::fun_chirho(list_a_chirho.clone(), list_b_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // last :: [a] -> a
        // last (x:xs) = case xs of { [] -> x; _ -> last xs }
        {
            let last_id_chirho = self.resolve_or_fresh_id_chirho("last");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("$w2", list_a_chirho.clone());
            let h2_chirho = self.fresh_binder_chirho("h2", a_chirho.clone());
            let t2_chirho = self.fresh_binder_chirho("t2", list_a_chirho.clone());

            let last_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(last_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
                bind_chirho: w2_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h2_chirho, t2_chirho],
                        rhs_chirho: last_rec_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![h_chirho, t_chirho],
                    rhs_chirho: inner_case_chirho,
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: last_id_chirho,
                    name_chirho: "last".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // init :: [a] -> [a]
        // init [x] = []; init (x:xs) = x : init xs
        {
            let init_id_chirho = self.resolve_or_fresh_id_chirho("init");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("$w2", list_a_chirho.clone());
            let h2_chirho = self.fresh_binder_chirho("h2", a_chirho.clone());
            let t2_chirho = self.fresh_binder_chirho("t2", list_a_chirho.clone());

            let init_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(init_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    init_rec_chirho,
                ],
            };
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
                bind_chirho: w2_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h2_chirho, t2_chirho],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![h_chirho, t_chirho],
                    rhs_chirho: inner_case_chirho,
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: init_id_chirho,
                    name_chirho: "init".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // ── Additional list functions: takeWhile, dropWhile, iterate, lookup, unzip, scanl ──
        self.generate_extra_list_prelude_chirho();

        // ── Additional list functions: nub, nubBy, group, groupBy, tails, inits ──
        self.generate_list_extra_prelude_chirho();

        // ── Ord-based list functions: elem, notElem, minimum, maximum, sort ──
        self.generate_ord_prelude_chirho();

        // ── Enum / Bounded prim bindings for instance dictionaries ──
        self.generate_enum_bounded_prelude_chirho();

        // ── Integral prim bindings for instance dictionaries ──
        self.generate_integral_prelude_chirho();

        // ── Data.Char functions ──
        self.generate_char_prelude_chirho();

        // ── Floating prim bindings ──
        self.generate_floating_prelude_chirho();

        // ── Functor / Applicative / Monad prim bindings ──
        self.generate_functor_monad_prelude_chirho();

        // ── IO control flow ──
        self.generate_io_control_prelude_chirho();

        // ── Data.Map (BST-based) ──
        self.generate_map_prelude_chirho();

        // ── Data.Map extended operations ──
        self.generate_map_extended_chirho();

        // ── Data.Map String-keyed operations ──
        self.generate_map_str_prelude_chirho();

        // ── Data.Set (BST-based) ──
        self.generate_set_prelude_chirho();

        // ── Data.Maybe extras ──
        self.generate_maybe_prelude_chirho();

        // ── Data.IORef ──
        self.generate_ioref_prelude_chirho();

        // ── Control.Monad.ST ──
        self.generate_st_prelude_chirho();

        // ── Semigroup / Monoid ──
        self.generate_semigroup_monoid_prelude_chirho();

        // ── Higher-order list functions ──
        self.generate_higher_order_list_prelude_chirho();

        // ── Monad transformer infrastructure (MaybeT / StateT) ──
        self.generate_transformer_prelude_chirho();

        // ── Additional utility functions (repeat, cycle, group, transpose, fix, etc.) ──
        self.generate_utility_prelude_chirho();
    }

    /// Generate Core IR bindings for monad transformer infrastructure.
    ///
    /// MaybeT is implemented as a newtype wrapping `Maybe a`.  At the Core IR
    /// level we represent it as a data constructor `MaybeT` with a single field
    /// and a selector `runMaybeT`.  StateT is similar: constructor `StateT`
    /// wrapping a state function `s -> (a, s)` with selector `runStateT`.
    ///
    /// Because the compiler's newtype erasure turns single-field constructors
    /// into identity at runtime, these bindings delegate directly through the
    /// `Maybe` / tuple machinery that already exists, making the resulting
    /// evaluation straightforward.
    fn generate_transformer_prelude_chirho(&mut self) {
        use rhasky_typing_chirho::ty_chirho::TyVarChirho;

        let a_chirho = TyChirho::VarChirho(TyVarChirho(9990));
        let maybe_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(a_chirho.clone()),
        );

        // ── MaybeT constructor: MaybeT :: Maybe a -> MaybeT a ──
        // Implementation: identity wrapper (newtype erasure makes this trivial).
        // At runtime the `MaybeT` tag is emitted as a single-field constructor.
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("MaybeT");
            let inner_chirho = self.fresh_binder_chirho("inner", maybe_a_chirho.clone());
            // MaybeT inner = Con "MaybeT" [inner]
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: inner_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "MaybeT".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(inner_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "MaybeT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(maybe_a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── runMaybeT :: MaybeT a -> Maybe a ──
        // Implementation: case dispatch extracts the single field.
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("runMaybeT");
            let arg_chirho = self.fresh_binder_chirho("t", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_s", a_chirho.clone());
            let field_chirho = self.fresh_binder_chirho("inner", maybe_a_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(arg_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: maybe_a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("MaybeT".to_string()),
                    binders_chirho: vec![field_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_chirho.id_chirho),
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: arg_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "runMaybeT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), maybe_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── StateT constructor: StateT :: (s -> (a, s)) -> StateT s a ──
        // Implementation: wraps a state-transition function.
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("StateT");
            let f_chirho = self.fresh_binder_chirho("f", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "StateT".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(f_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "StateT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── runStateT :: StateT s a -> s -> (a, s) ──
        // Implementation: case dispatch extracts the state function, then applies it.
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("runStateT");
            let t_chirho = self.fresh_binder_chirho("t", a_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sr", a_chirho.clone());
            let field_chirho = self.fresh_binder_chirho("f", a_chirho.clone());

            // case t of { StateT f -> f s }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("StateT".to_string()),
                    binders_chirho: vec![field_chirho.clone()],
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(field_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                    },
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: t_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "runStateT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
    }

    /// Generate additional list functions: nub (Int), zip3, zipWith3, intersperse,
    /// isPrefixOf (Int), isSuffixOf (Int), tails, inits, and cycle.
    fn generate_list_extra_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9991));
        let c_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9992));
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));
        let list_b_chirho = TyChirho::ListChirho(Box::new(b_chirho.clone()));
        let _list_c_chirho = TyChirho::ListChirho(Box::new(c_chirho.clone()));

        let nil_chirho = || CoreExprChirho::ConAppChirho {
            con_name_chirho: "[]".to_string(),
            args_chirho: vec![],
        };
        let cons_chirho = |hd: CoreExprChirho, tl: CoreExprChirho| CoreExprChirho::ConAppChirho {
            con_name_chirho: ":".to_string(),
            args_chirho: vec![hd, tl],
        };

        // nub :: [Int] -> [Int]
        // nub [] = []
        // nub (x:xs) = x : nub (filter (\y -> not (y ==# x)) xs)
        // Uses elem-like recursive filtering
        {
            let nub_id_chirho = self.resolve_or_fresh_id_chirho("nub");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_ns", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());

            // helper: nubHelper seen [] = []
            //         nubHelper seen (x:xs) = if elem x seen then nubHelper seen xs
            //                                 else x : nubHelper (x:seen) xs
            // Simpler: inline recursive approach
            // nub [] = []
            // nub (x:xs) = x : nub (filter_ne x xs)
            // where filter_ne removes all elements == x

            let filter_id_chirho = self.resolve_or_fresh_id_chirho("filter");

            // \y -> not (y ==# x)  — predicate to keep elements != h
            let y_chirho = self.fresh_binder_chirho("y", a_chirho.clone());
            let eq_scr_chirho = self.fresh_binder_chirho("_eq", TyChirho::bool_chirho());

            let eq_check_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "==#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(y_chirho.id_chirho),
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                ],
            };

            // not (y ==# x): case (y ==# x) of { True -> False; False -> True }
            let not_eq_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(eq_check_chirho),
                bind_chirho: eq_scr_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };

            let pred_chirho = CoreExprChirho::LamChirho {
                binder_chirho: y_chirho,
                body_chirho: Box::new(not_eq_chirho),
            };

            // filter pred t
            let filtered_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(filter_id_chirho)),
                    arg_chirho: Box::new(pred_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            // nub (filter pred t)
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(nub_id_chirho)),
                arg_chirho: Box::new(filtered_chirho),
            };

            // h : nub (filter pred t)
            let cons_result_chirho = cons_chirho(
                CoreExprChirho::VarChirho(h_chirho.id_chirho),
                rec_chirho,
            );

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_result_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: nub_id_chirho,
                    name_chirho: "nub".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // zip3 :: [a] -> [b] -> [c] -> [(a,b,c)]
        // zip3 [] _ _ = []
        // zip3 _ [] _ = []
        // zip3 _ _ [] = []
        // zip3 (a:as) (b:bs) (c:cs) = (a,b,c) : zip3 as bs cs
        {
            let zip3_id_chirho = self.resolve_or_fresh_id_chirho("zip3");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_b_chirho.clone());
            let zs_chirho = self.fresh_binder_chirho("zs", list_a_chirho.clone());
            let scr1_chirho = self.fresh_binder_chirho("_z1", list_a_chirho.clone());
            let scr2_chirho = self.fresh_binder_chirho("_z2", list_b_chirho.clone());
            let scr3_chirho = self.fresh_binder_chirho("_z3", list_a_chirho.clone());
            let xh_chirho = self.fresh_binder_chirho("xh", a_chirho.clone());
            let xt_chirho = self.fresh_binder_chirho("xt", list_a_chirho.clone());
            let yh_chirho = self.fresh_binder_chirho("yh", b_chirho.clone());
            let yt_chirho = self.fresh_binder_chirho("yt", list_b_chirho.clone());
            let zh_chirho = self.fresh_binder_chirho("zh", c_chirho.clone());
            let zt_chirho = self.fresh_binder_chirho("zt", list_a_chirho.clone());

            // (xh, yh, zh) tuple
            let tuple_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple3".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(xh_chirho.id_chirho),
                    CoreExprChirho::VarChirho(yh_chirho.id_chirho),
                    CoreExprChirho::VarChirho(zh_chirho.id_chirho),
                ],
            };

            // zip3 xt yt zt
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(zip3_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(xt_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(yt_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(zt_chirho.id_chirho)),
            };

            let inner_cons_chirho = cons_chirho(tuple_chirho, rec_chirho);

            // case zs of { [] -> []; (z:zs) -> (x,y,z) : zip3 xt yt zt }
            let case_z_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(zs_chirho.id_chirho)),
                bind_chirho: scr3_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![zh_chirho, zt_chirho],
                        rhs_chirho: inner_cons_chirho,
                    },
                ],
            };

            // case ys of { [] -> []; (y:ys) -> case zs ... }
            let case_y_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
                bind_chirho: scr2_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![yh_chirho, yt_chirho],
                        rhs_chirho: case_z_chirho,
                    },
                ],
            };

            // case xs of { [] -> []; (x:xs) -> case ys ... }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr1_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![xh_chirho, xt_chirho],
                        rhs_chirho: case_y_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ys_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: zs_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: zip3_id_chirho,
                    name_chirho: "zip3".to_string(),
                    ty_chirho: list_a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // intersperse :: a -> [a] -> [a]
        // intersperse _ [] = []
        // intersperse _ [x] = [x]
        // intersperse sep (x:xs) = x : sep : intersperse sep xs
        {
            let isp_id_chirho = self.resolve_or_fresh_id_chirho("intersperse");
            let sep_chirho = self.fresh_binder_chirho("sep", a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_is", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let scr2_chirho = self.fresh_binder_chirho("_is2", list_a_chirho.clone());

            // intersperse sep t
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(isp_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(sep_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            // h : sep : intersperse sep t
            let cons_with_sep_chirho = cons_chirho(
                CoreExprChirho::VarChirho(h_chirho.id_chirho),
                cons_chirho(
                    CoreExprChirho::VarChirho(sep_chirho.id_chirho),
                    rec_chirho,
                ),
            );

            // case t of { [] -> [h]; _ -> h : sep : intersperse sep t }
            let h2_chirho = self.fresh_binder_chirho("_h2", a_chirho.clone());
            let t2_chirho = self.fresh_binder_chirho("_t2", list_a_chirho.clone());

            let check_tail_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
                bind_chirho: scr2_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho(CoreExprChirho::VarChirho(h_chirho.id_chirho), nil_chirho()),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h2_chirho, t2_chirho],
                        rhs_chirho: cons_with_sep_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: check_tail_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: sep_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: isp_id_chirho,
                    name_chirho: "intersperse".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone())),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // isPrefixOf :: [Int] -> [Int] -> Bool
        // isPrefixOf [] _ = True
        // isPrefixOf _ [] = False
        // isPrefixOf (x:xs) (y:ys) = (x ==# y) && isPrefixOf xs ys
        {
            let ipf_id_chirho = self.resolve_or_fresh_id_chirho("isPrefixOf");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let scr1_chirho = self.fresh_binder_chirho("_p1", list_a_chirho.clone());
            let scr2_chirho = self.fresh_binder_chirho("_p2", list_a_chirho.clone());
            let xh_chirho = self.fresh_binder_chirho("xh", a_chirho.clone());
            let xt_chirho = self.fresh_binder_chirho("xt", list_a_chirho.clone());
            let yh_chirho = self.fresh_binder_chirho("yh", a_chirho.clone());
            let yt_chirho = self.fresh_binder_chirho("yt", list_a_chirho.clone());
            let eq_scr_chirho = self.fresh_binder_chirho("_eq", TyChirho::bool_chirho());

            // isPrefixOf xt yt
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(ipf_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(xt_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(yt_chirho.id_chirho)),
            };

            // xh ==# yh
            let eq_check_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "==#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(xh_chirho.id_chirho),
                    CoreExprChirho::VarChirho(yh_chirho.id_chirho),
                ],
            };

            // case (xh ==# yh) of { True -> isPrefixOf xt yt; False -> False }
            let and_check_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(eq_check_chirho),
                bind_chirho: eq_scr_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };

            // case ys of { [] -> False; (y:ys) -> ... }
            let case_ys_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
                bind_chirho: scr2_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![yh_chirho, yt_chirho],
                        rhs_chirho: and_check_chirho,
                    },
                ],
            };

            // case xs of { [] -> True; (x:xs) -> case ys ... }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr1_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![xh_chirho, xt_chirho],
                        rhs_chirho: case_ys_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ys_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: ipf_id_chirho,
                    name_chirho: "isPrefixOf".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        list_a_chirho.clone(),
                        TyChirho::fun_chirho(list_a_chirho.clone(), TyChirho::bool_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // isSuffixOf :: [Int] -> [Int] -> Bool
        // isSuffixOf xs ys = isPrefixOf (reverse xs) (reverse ys)
        {
            let isf_id_chirho = self.resolve_or_fresh_id_chirho("isSuffixOf");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let ipf_id_chirho = self.resolve_or_fresh_id_chirho("isPrefixOf");
            let rev_id_chirho = self.resolve_or_fresh_id_chirho("reverse");

            let rev_xs_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(rev_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let rev_ys_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(rev_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(ipf_id_chirho)),
                    arg_chirho: Box::new(rev_xs_chirho),
                }),
                arg_chirho: Box::new(rev_ys_chirho),
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ys_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: isf_id_chirho,
                    name_chirho: "isSuffixOf".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        list_a_chirho.clone(),
                        TyChirho::fun_chirho(list_a_chirho.clone(), TyChirho::bool_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // unzip3 :: [(a,b,c)] -> ([a],[b],[c])
        // unzip3 [] = ([], [], [])
        // unzip3 ((a,b,c):rest) = let (as,bs,cs) = unzip3 rest in (a:as, b:bs, c:cs)
        {
            let uz3_id_chirho = self.resolve_or_fresh_id_chirho("unzip3");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_uz", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let tscr_chirho = self.fresh_binder_chirho("_th", a_chirho.clone());
            let ta_chirho = self.fresh_binder_chirho("ta", a_chirho.clone());
            let tb_chirho = self.fresh_binder_chirho("tb", b_chirho.clone());
            let tc_chirho = self.fresh_binder_chirho("tc", c_chirho.clone());
            let rscr_chirho = self.fresh_binder_chirho("_rr", a_chirho.clone());
            let ra_chirho = self.fresh_binder_chirho("ra", list_a_chirho.clone());
            let rb_chirho = self.fresh_binder_chirho("rb", list_b_chirho.clone());
            let rc_chirho = self.fresh_binder_chirho("rc", list_a_chirho.clone());

            // unzip3 t
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(uz3_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            // case rec of { $tuple3 ra rb rc -> (ta:ra, tb:rb, tc:rc) }
            let result_tuple_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple3".to_string(),
                args_chirho: vec![
                    cons_chirho(CoreExprChirho::VarChirho(ta_chirho.id_chirho), CoreExprChirho::VarChirho(ra_chirho.id_chirho)),
                    cons_chirho(CoreExprChirho::VarChirho(tb_chirho.id_chirho), CoreExprChirho::VarChirho(rb_chirho.id_chirho)),
                    cons_chirho(CoreExprChirho::VarChirho(tc_chirho.id_chirho), CoreExprChirho::VarChirho(rc_chirho.id_chirho)),
                ],
            };

            let case_rec_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(rec_chirho),
                bind_chirho: rscr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple3".to_string()),
                    binders_chirho: vec![ra_chirho, rb_chirho, rc_chirho],
                    rhs_chirho: result_tuple_chirho,
                }],
            };

            // case h of { $tuple3 ta tb tc -> case rec ... }
            let case_head_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                bind_chirho: tscr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple3".to_string()),
                    binders_chirho: vec![ta_chirho, tb_chirho, tc_chirho],
                    rhs_chirho: case_rec_chirho,
                }],
            };

            let empty_result_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple3".to_string(),
                args_chirho: vec![nil_chirho(), nil_chirho(), nil_chirho()],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: empty_result_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: case_head_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: uz3_id_chirho,
                    name_chirho: "unzip3".to_string(),
                    ty_chirho: list_a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }
    }

    /// Generate Prelude functions that use Ord/Eq: elem, notElem, minimum, maximum, sort.
    /// These are specialized to Int for now using primitive comparison ops.
    fn generate_ord_prelude_chirho(&mut self) {
        let int_chirho = TyChirho::ConChirho("Int".to_string());
        let list_int_chirho = TyChirho::ListChirho(Box::new(int_chirho.clone()));

        // ── elem :: Int -> [Int] -> Bool ──
        // elem e [] = False
        // elem e (x:xs) = case e ==# x of True -> True; False -> elem e xs
        {
            let elem_id_chirho = self.resolve_or_fresh_id_chirho("elem");
            let e_chirho = self.fresh_binder_chirho("e", int_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", int_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_int_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$wild", TyChirho::bool_chirho());
            let wild2_chirho = self.fresh_binder_chirho("$wild2", list_int_chirho.clone());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(elem_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            let eq_check_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "==#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(e_chirho.id_chirho),
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    ],
                }),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(), args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_call_chirho,
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: wild2_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(), args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho, rest_chirho],
                        rhs_chirho: eq_check_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: e_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: elem_id_chirho,
                    name_chirho: "elem".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![int_chirho.clone(), list_int_chirho.clone()],
                        TyChirho::bool_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // ── notElem :: Int -> [Int] -> Bool ──
        // notElem e xs = case elem e xs of True -> False; False -> True
        {
            let notelem_id_chirho = self.resolve_or_fresh_id_chirho("notElem");
            let elem_id_chirho = self.resolve_or_fresh_id_chirho("elem");
            let e_chirho = self.fresh_binder_chirho("e", int_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$wild", TyChirho::bool_chirho());

            let elem_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(elem_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(elem_call_chirho),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(), args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(), args_chirho: vec![],
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: e_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: notelem_id_chirho,
                    name_chirho: "notElem".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![int_chirho.clone(), list_int_chirho.clone()],
                        TyChirho::bool_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── minimum :: [Int] -> Int ──
        // minimum xs = foldl min (head xs) (tail xs)
        {
            let minimum_id_chirho = self.resolve_or_fresh_id_chirho("minimum");
            let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");
            let min_id_chirho = self.resolve_or_fresh_id_chirho("min");
            let head_id_chirho = self.resolve_or_fresh_id_chirho("head");
            let tail_id_chirho = self.resolve_or_fresh_id_chirho("tail");
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());

            let head_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(head_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let tail_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(tail_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            // foldl min (head xs) (tail xs)
            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldl_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(min_id_chirho)),
                    }),
                    arg_chirho: Box::new(head_call_chirho),
                }),
                arg_chirho: Box::new(tail_call_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: minimum_id_chirho,
                    name_chirho: "minimum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_int_chirho.clone(), int_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── maximum :: [Int] -> Int ──
        // maximum xs = foldl max (head xs) (tail xs)
        {
            let maximum_id_chirho = self.resolve_or_fresh_id_chirho("maximum");
            let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");
            let max_id_chirho = self.resolve_or_fresh_id_chirho("max");
            let head_id_chirho = self.resolve_or_fresh_id_chirho("head");
            let tail_id_chirho = self.resolve_or_fresh_id_chirho("tail");
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());

            let head_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(head_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let tail_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(tail_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldl_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(max_id_chirho)),
                    }),
                    arg_chirho: Box::new(head_call_chirho),
                }),
                arg_chirho: Box::new(tail_call_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: maximum_id_chirho,
                    name_chirho: "maximum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_int_chirho.clone(), int_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── sort :: [Int] -> [Int] (insertion sort via <=# primop) ──
        // sort [] = []
        // sort (x:xs) = insert x (sort xs)
        // insert e [] = [e]
        // insert e (x:xs) = case e <=# x of True -> e:x:xs; False -> x : insert e xs
        {
            let insert_id_chirho = self.resolve_or_fresh_id_chirho("insert");
            let sort_id_chirho = self.resolve_or_fresh_id_chirho("sort");

            // ── insert ──
            let e_chirho = self.fresh_binder_chirho("e", int_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_int_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", int_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_int_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$wild", list_int_chirho.clone());
            let wild2_chirho = self.fresh_binder_chirho("$wild2", TyChirho::bool_chirho());

            let singleton_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(e_chirho.id_chirho),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: "[]".to_string(), args_chirho: vec![],
                    },
                ],
            };
            // e : y : rest
            let e_cons_all_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(e_chirho.id_chirho),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(y_chirho.id_chirho),
                            CoreExprChirho::VarChirho(rest_chirho.id_chirho),
                        ],
                    },
                ],
            };
            // y : insert e rest
            let rec_insert_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(y_chirho.id_chirho),
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(insert_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
                    },
                ],
            };
            let le_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<=#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(e_chirho.id_chirho),
                        CoreExprChirho::VarChirho(y_chirho.id_chirho),
                    ],
                }),
                bind_chirho: wild2_chirho,
                result_ty_chirho: list_int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: e_cons_all_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_insert_chirho,
                    },
                ],
            };
            let insert_body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: list_int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: singleton_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![y_chirho, rest_chirho],
                        rhs_chirho: le_case_chirho,
                    },
                ],
            };
            let insert_rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: e_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ys_chirho,
                    body_chirho: Box::new(insert_body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: insert_id_chirho,
                    name_chirho: "insert".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![int_chirho.clone(), list_int_chirho.clone()],
                        list_int_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: insert_rhs_chirho,
                is_rec_chirho: true,
            });

            // ── sort ──
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", int_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_int_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$wild", list_int_chirho.clone());

            let sort_rest_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(sort_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            let insert_into_sorted_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(insert_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(sort_rest_chirho),
            };
            let sort_body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: list_int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(), args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho, rest_chirho],
                        rhs_chirho: insert_into_sorted_chirho,
                    },
                ],
            };
            let sort_rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(sort_body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: sort_id_chirho,
                    name_chirho: "sort".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_int_chirho.clone(), list_int_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: sort_rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // abs is already defined in generate_prelude_bindings_chirho

        // ── signum :: Int -> Int ──
        // signum n = case n ># 0 of True -> 1; False -> case n <# 0 of True -> -1; False -> 0
        {
            let signum_id_chirho = self.resolve_or_fresh_id_chirho("signum");
            let n_chirho = self.fresh_binder_chirho("n", int_chirho.clone());
            let wild1_chirho = self.fresh_binder_chirho("$w1", TyChirho::bool_chirho());
            let wild2_chirho = self.fresh_binder_chirho("$w2", TyChirho::bool_chirho());

            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(n_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: wild2_chirho,
                result_ty_chirho: int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(-1)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: ">#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(n_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: wild1_chirho,
                result_ty_chirho: int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: signum_id_chirho,
                    name_chirho: "signum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_chirho.clone(), int_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: rhs_chirho.clone(), is_rec_chirho: false,
            });

            // Also generate $prim_Num_signum_Int for the instance dictionary
            let prim_signum_name_chirho = "$prim_Num_signum_Int";
            let prim_signum_id_chirho = self.resolve_or_fresh_id_chirho(prim_signum_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_signum_id_chirho,
                    name_chirho: prim_signum_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_chirho.clone(), int_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // even and odd are already defined in generate_prelude_bindings_chirho

        // ── replicate :: Int -> a -> [a] ──
        // replicate 0 _ = []
        // replicate n x = x : replicate (n-1) x
        {
            let replicate_id_chirho = self.resolve_or_fresh_id_chirho("replicate");
            let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9995));
            let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));

            let n_chirho = self.fresh_binder_chirho("n", int_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$wild", TyChirho::bool_chirho());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(replicate_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "-#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(n_chirho.id_chirho),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        ],
                    }),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    rec_call_chirho,
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<=#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(n_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: wild_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(), args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: replicate_id_chirho,
                    name_chirho: "replicate".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![int_chirho.clone(), a_chirho],
                        list_a_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho, is_rec_chirho: true,
            });
        }
    }

    /// Generate additional list Prelude functions: takeWhile, dropWhile, iterate, lookup, unzip, scanl.
    fn generate_extra_list_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9991));
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));
        let list_b_chirho = TyChirho::ListChirho(Box::new(b_chirho.clone()));

        let nil_chirho = || CoreExprChirho::ConAppChirho {
            con_name_chirho: "[]".to_string(),
            args_chirho: vec![],
        };

        // ── takeWhile :: (a -> Bool) -> [a] -> [a] ──
        // takeWhile p [] = []; takeWhile p (x:xs) = if p x then x : takeWhile p xs else []
        {
            let tw_id_chirho = self.resolve_or_fresh_id_chirho("takeWhile");
            let p_chirho = self.fresh_binder_chirho("p", TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", TyChirho::bool_chirho());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(tw_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    rec_call_chirho,
                ],
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let cond_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: wb_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cond_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: tw_id_chirho,
                    name_chirho: "takeWhile".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()), list_a_chirho.clone()],
                        list_a_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho, is_rec_chirho: true,
            });
        }

        // ── dropWhile :: (a -> Bool) -> [a] -> [a] ──
        // dropWhile p [] = []; dropWhile p (x:xs) = if p x then dropWhile p xs else x:xs
        {
            let dw_id_chirho = self.resolve_or_fresh_id_chirho("dropWhile");
            let p_chirho = self.fresh_binder_chirho("p", TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", TyChirho::bool_chirho());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(dw_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let keep_rest_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    CoreExprChirho::VarChirho(t_chirho.id_chirho),
                ],
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let cond_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: wb_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_call_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: keep_rest_chirho,
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cond_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: dw_id_chirho,
                    name_chirho: "dropWhile".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()), list_a_chirho.clone()],
                        list_a_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho, is_rec_chirho: true,
            });
        }

        // ── iterate :: (a -> a) -> a -> [a] ──
        // iterate f x = x : iterate f (f x)
        {
            let iter_id_chirho = self.resolve_or_fresh_id_chirho("iterate");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()));
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());

            let f_x_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(iter_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(f_x_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_chirho.id_chirho),
                            rec_call_chirho,
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: iter_id_chirho,
                    name_chirho: "iterate".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()), a_chirho.clone()],
                        list_a_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho, is_rec_chirho: true,
            });
        }

        // ── lookup :: a -> [(a,b)] -> Maybe b ──  (uses ==# for Int keys)
        // lookup _ [] = Nothing; lookup k ((k',v):rest) = if k ==# k' then Just v else lookup k rest
        {
            let lookup_id_chirho = self.resolve_or_fresh_id_chirho("lookup");
            let pair_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let list_pair_chirho = TyChirho::ListChirho(Box::new(pair_ty_chirho.clone()));
            let maybe_b_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(b_chirho.clone()),
            );
            let k_chirho = self.fresh_binder_chirho("k", a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_pair_chirho.clone());
            let pair_chirho = self.fresh_binder_chirho("pair", pair_ty_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_pair_chirho.clone());
            let kp_chirho = self.fresh_binder_chirho("k'", a_chirho.clone());
            let v_chirho = self.fresh_binder_chirho("v", b_chirho.clone());
            let w1_chirho = self.fresh_binder_chirho("$w1", list_pair_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("$w2", pair_ty_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", TyChirho::bool_chirho());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(lookup_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(k_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            let eq_check_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "==#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(k_chirho.id_chirho),
                    CoreExprChirho::VarChirho(kp_chirho.id_chirho),
                ],
            };
            let cond_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(eq_check_chirho),
                bind_chirho: wb_chirho,
                result_ty_chirho: maybe_b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Just".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(v_chirho.id_chirho)],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: rec_call_chirho,
                    },
                ],
            };
            // Destructure the pair
            let pair_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(pair_chirho.id_chirho)),
                bind_chirho: w2_chirho,
                result_ty_chirho: maybe_b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                        binders_chirho: vec![kp_chirho, v_chirho],
                        rhs_chirho: cond_chirho,
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w1_chirho,
                result_ty_chirho: maybe_b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Nothing".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![pair_chirho, rest_chirho],
                        rhs_chirho: pair_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: lookup_id_chirho,
                    name_chirho: "lookup".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![a_chirho.clone(), list_pair_chirho],
                        maybe_b_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho, is_rec_chirho: true,
            });
        }

        // ── unzip :: [(a,b)] -> ([a],[b]) ──
        // unzip [] = ([],[]); unzip ((a,b):rest) = let (as,bs) = unzip rest in (a:as, b:bs)
        {
            let unzip_id_chirho = self.resolve_or_fresh_id_chirho("unzip");
            let pair_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let list_pair_chirho = TyChirho::ListChirho(Box::new(pair_ty_chirho.clone()));
            let result_ty_chirho = TyChirho::ConChirho("(,)".to_string());

            let xs_chirho = self.fresh_binder_chirho("xs", list_pair_chirho.clone());
            let pair_chirho = self.fresh_binder_chirho("pair", pair_ty_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_pair_chirho.clone());
            let pa_chirho = self.fresh_binder_chirho("pa", a_chirho.clone());
            let pb_chirho = self.fresh_binder_chirho("pb", b_chirho.clone());
            let w1_chirho = self.fresh_binder_chirho("$w1", list_pair_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("$w2", pair_ty_chirho.clone());
            let res_chirho = self.fresh_binder_chirho("res", result_ty_chirho.clone());
            let as_chirho = self.fresh_binder_chirho("as_", list_a_chirho.clone());
            let bs_chirho = self.fresh_binder_chirho("bs_", list_b_chirho.clone());
            let w3_chirho = self.fresh_binder_chirho("$w3", result_ty_chirho.clone());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(unzip_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            // let res = unzip rest in case res of (as_,bs_) -> (pa:as_, pb:bs_)
            let result_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(res_chirho.id_chirho)),
                bind_chirho: w3_chirho,
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                        binders_chirho: vec![as_chirho.clone(), bs_chirho.clone()],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "$tuple2".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::ConAppChirho {
                                    con_name_chirho: ":".to_string(),
                                    args_chirho: vec![
                                        CoreExprChirho::VarChirho(pa_chirho.id_chirho),
                                        CoreExprChirho::VarChirho(as_chirho.id_chirho),
                                    ],
                                },
                                CoreExprChirho::ConAppChirho {
                                    con_name_chirho: ":".to_string(),
                                    args_chirho: vec![
                                        CoreExprChirho::VarChirho(pb_chirho.id_chirho),
                                        CoreExprChirho::VarChirho(bs_chirho.id_chirho),
                                    ],
                                },
                            ],
                        },
                    },
                ],
            };
            let let_body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(res_chirho, rec_call_chirho)],
                body_chirho: Box::new(result_case_chirho),
            };
            // Destructure pair
            let pair_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(pair_chirho.id_chirho)),
                bind_chirho: w2_chirho,
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                        binders_chirho: vec![pa_chirho, pb_chirho],
                        rhs_chirho: let_body_chirho,
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w1_chirho,
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "$tuple2".to_string(),
                            args_chirho: vec![nil_chirho(), nil_chirho()],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![pair_chirho, rest_chirho],
                        rhs_chirho: pair_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: unzip_id_chirho,
                    name_chirho: "unzip".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_pair_chirho, result_ty_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho, is_rec_chirho: true,
            });
        }

        // ── scanl :: (b -> a -> b) -> b -> [a] -> [b] ──
        // scanl f z [] = [z]; scanl f z (x:xs) = z : scanl f (f z x) xs
        {
            let scanl_id_chirho = self.resolve_or_fresh_id_chirho("scanl");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_n_chirho(
                vec![b_chirho.clone(), a_chirho.clone()], b_chirho.clone(),
            ));
            let z_chirho = self.fresh_binder_chirho("z", b_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let f_z_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(z_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(scanl_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(f_z_h_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: ":".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(z_chirho.id_chirho),
                                nil_chirho(),
                            ],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: ":".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(z_chirho.id_chirho),
                                rec_call_chirho,
                            ],
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: xs_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: scanl_id_chirho,
                    name_chirho: "scanl".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::fun_n_chirho(vec![b_chirho.clone(), a_chirho.clone()], b_chirho.clone()),
                            b_chirho.clone(),
                            list_a_chirho.clone(),
                        ],
                        list_b_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho, is_rec_chirho: true,
            });
        }

        // ── span :: (a -> Bool) -> [a] -> ([a],[a]) ──
        // span p [] = ([],[]); span p (x:xs) = if p x then let (ys,zs) = span p xs in (x:ys,zs) else ([],x:xs)
        {
            let span_id_chirho = self.resolve_or_fresh_id_chirho("span");
            let tuple_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let p_chirho = self.fresh_binder_chirho("p", TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", TyChirho::bool_chirho());
            let res_chirho = self.fresh_binder_chirho("res", tuple_ty_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let zs_chirho = self.fresh_binder_chirho("zs", list_a_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("$w2", tuple_ty_chirho.clone());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(span_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            // let res = span p t in case res of (ys,zs) -> (h:ys, zs)
            let true_body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(res_chirho.clone(), rec_call_chirho)],
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(res_chirho.id_chirho)),
                    bind_chirho: w2_chirho,
                    result_ty_chirho: tuple_ty_chirho.clone(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                            binders_chirho: vec![ys_chirho.clone(), zs_chirho.clone()],
                            rhs_chirho: CoreExprChirho::ConAppChirho {
                                con_name_chirho: "$tuple2".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::ConAppChirho {
                                        con_name_chirho: ":".to_string(),
                                        args_chirho: vec![
                                            CoreExprChirho::VarChirho(h_chirho.id_chirho),
                                            CoreExprChirho::VarChirho(ys_chirho.id_chirho),
                                        ],
                                    },
                                    CoreExprChirho::VarChirho(zs_chirho.id_chirho),
                                ],
                            },
                        },
                    ],
                }),
            };
            // False branch: ([], x:xs)
            let false_body_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    nil_chirho(),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(h_chirho.id_chirho),
                            CoreExprChirho::VarChirho(t_chirho.id_chirho),
                        ],
                    },
                ],
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let cond_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: wb_chirho,
                result_ty_chirho: tuple_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: true_body_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: false_body_chirho,
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: tuple_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "$tuple2".to_string(),
                            args_chirho: vec![nil_chirho(), nil_chirho()],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cond_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: span_id_chirho,
                    name_chirho: "span".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()), list_a_chirho.clone()],
                        tuple_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho, is_rec_chirho: true,
            });

            // break = span . not
            // break p = span (not . p)
            let break_id_chirho = self.resolve_or_fresh_id_chirho("break");
            let bp_chirho = self.fresh_binder_chirho("p", TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()));
            let bxs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let bx_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            // \x -> not (p x) — uses case to negate
            let bwb_chirho = self.fresh_binder_chirho("$wb", TyChirho::bool_chirho());
            let not_p_x_chirho = CoreExprChirho::LamChirho {
                binder_chirho: bx_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(bp_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(bx_chirho.id_chirho)),
                    }),
                    bind_chirho: bwb_chirho,
                    result_ty_chirho: TyChirho::bool_chirho(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::ConAppChirho {
                                con_name_chirho: "False".to_string(), args_chirho: vec![],
                            },
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::ConAppChirho {
                                con_name_chirho: "True".to_string(), args_chirho: vec![],
                            },
                        },
                    ],
                }),
            };
            let break_body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(span_id_chirho)),
                    arg_chirho: Box::new(not_p_x_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(bxs_chirho.id_chirho)),
            };
            let break_rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: bp_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: bxs_chirho,
                    body_chirho: Box::new(break_body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: break_id_chirho,
                    name_chirho: "break".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()), list_a_chirho.clone()],
                        tuple_ty_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: break_rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── partition :: (a -> Bool) -> [a] -> ([a],[a]) ──
        // partition p [] = ([],[]); partition p (x:xs) = let (yes,no) = partition p xs
        //   in if p x then (x:yes, no) else (yes, x:no)
        {
            let part_id_chirho = self.resolve_or_fresh_id_chirho("partition");
            let tuple_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let p_chirho = self.fresh_binder_chirho("p", TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", TyChirho::bool_chirho());
            let res_chirho = self.fresh_binder_chirho("res", tuple_ty_chirho.clone());
            let yes_chirho = self.fresh_binder_chirho("yes", list_a_chirho.clone());
            let no_chirho = self.fresh_binder_chirho("no", list_a_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("$w2", tuple_ty_chirho.clone());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(part_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            // true: (h:yes, no)
            let true_rhs_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(h_chirho.id_chirho),
                            CoreExprChirho::VarChirho(yes_chirho.id_chirho),
                        ],
                    },
                    CoreExprChirho::VarChirho(no_chirho.id_chirho),
                ],
            };
            // false: (yes, h:no)
            let false_rhs_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(yes_chirho.id_chirho),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(h_chirho.id_chirho),
                            CoreExprChirho::VarChirho(no_chirho.id_chirho),
                        ],
                    },
                ],
            };
            let cond_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: wb_chirho,
                result_ty_chirho: tuple_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: true_rhs_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: false_rhs_chirho,
                    },
                ],
            };
            let cons_body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(res_chirho.clone(), rec_call_chirho)],
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(res_chirho.id_chirho)),
                    bind_chirho: w2_chirho,
                    result_ty_chirho: tuple_ty_chirho.clone(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                            binders_chirho: vec![yes_chirho, no_chirho],
                            rhs_chirho: cond_chirho,
                        },
                    ],
                }),
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: tuple_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "$tuple2".to_string(),
                            args_chirho: vec![nil_chirho(), nil_chirho()],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_body_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: part_id_chirho,
                    name_chirho: "partition".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()), list_a_chirho.clone()],
                        tuple_ty_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho, is_rec_chirho: true,
            });
        }
    }

    /// Generate `$prim_Enum_*` and `$prim_Bounded_*` bindings for instance dictionaries.
    fn generate_enum_bounded_prelude_chirho(&mut self) {
        let int_ty_chirho = TyChirho::int_chirho();

        // ── Enum Int: toEnum = identity, fromEnum = identity ──
        {
            let prim_name_chirho = "$prim_Enum_toEnum_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
        {
            let prim_name_chirho = "$prim_Enum_fromEnum_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── Enum Char: toEnum = chr-like, fromEnum = ord-like ──
        // For Char, toEnum and fromEnum are identity on the underlying Int representation
        {
            let char_ty_chirho = TyChirho::ConChirho("Char".to_string());
            let prim_name_chirho = "$prim_Enum_toEnum_Char";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), char_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });

            let prim_name_chirho = "$prim_Enum_fromEnum_Char";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", char_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(char_ty_chirho, int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── Enum Bool: toEnum 0 = False, toEnum _ = True; fromEnum False = 0, fromEnum True = 1 ──
        {
            let bool_ty_chirho = TyChirho::bool_chirho();

            // toEnum for Bool: \n -> case n ==# 0 of { True -> False; _ -> True }
            let prim_name_chirho = "$prim_Enum_toEnum_Bool";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());
            let eq_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "==#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                ],
            };
            let wild_chirho = self.fresh_binder_chirho("$w", bool_ty_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(eq_zero_chirho),
                bind_chirho: wild_chirho,
                result_ty_chirho: bool_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), bool_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });

            // fromEnum for Bool: \b -> case b of { False -> 0; True -> 1 }
            let prim_name_chirho = "$prim_Enum_fromEnum_Bool";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let b_chirho = self.fresh_binder_chirho("b", bool_ty_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", bool_ty_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: b_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(bool_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── Bounded Int: minBound = MIN, maxBound = MAX ──
        {
            let prim_name_chirho = "$prim_Bounded_minBound_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: int_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(i64::MIN)),
                is_rec_chirho: false,
            });
        }
        {
            let prim_name_chirho = "$prim_Bounded_maxBound_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: int_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(i64::MAX)),
                is_rec_chirho: false,
            });
        }

        // ── Bounded Char ──
        {
            let char_ty_chirho = TyChirho::ConChirho("Char".to_string());
            let prim_name_chirho = "$prim_Bounded_minBound_Char";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: char_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                is_rec_chirho: false,
            });
            let prim_name_chirho = "$prim_Bounded_maxBound_Char";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: char_ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0x10FFFF)),
                is_rec_chirho: false,
            });
        }

        // ── Bounded Bool ──
        {
            let bool_ty_chirho = TyChirho::bool_chirho();
            let prim_name_chirho = "$prim_Bounded_minBound_Bool";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: bool_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::ConAppChirho {
                    con_name_chirho: "False".to_string(),
                    args_chirho: vec![],
                },
                is_rec_chirho: false,
            });
            let prim_name_chirho = "$prim_Bounded_maxBound_Bool";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: bool_ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::ConAppChirho {
                    con_name_chirho: "True".to_string(),
                    args_chirho: vec![],
                },
                is_rec_chirho: false,
            });
        }

        // Also generate prelude-level toEnum/fromEnum/succ/pred bindings
        // so they can be used as regular functions
        {
            // toEnum :: Int -> Int (defaulting to Int-specialized)
            let to_enum_id_chirho = self.resolve_or_fresh_id_chirho("toEnum");
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: to_enum_id_chirho,
                    name_chirho: "toEnum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                },
                is_rec_chirho: false,
            });

            // fromEnum :: Int -> Int (defaulting to Int-specialized)
            let from_enum_id_chirho = self.resolve_or_fresh_id_chirho("fromEnum");
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: from_enum_id_chirho,
                    name_chirho: "fromEnum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                },
                is_rec_chirho: false,
            });

            // succ :: Int -> Int = \x -> x +# 1
            let succ_id_chirho = self.resolve_or_fresh_id_chirho("succ");
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: succ_id_chirho,
                    name_chirho: "succ".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_chirho.id_chirho),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        ],
                    }),
                },
                is_rec_chirho: false,
            });

            // pred :: Int -> Int = \x -> x -# 1
            let pred_id_chirho = self.resolve_or_fresh_id_chirho("pred");
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: pred_id_chirho,
                    name_chirho: "pred".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "-#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_chirho.id_chirho),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        ],
                    }),
                },
                is_rec_chirho: false,
            });

            // minBound :: Int (prelude-level, defaulting to Int)
            let min_bound_id_chirho = self.resolve_or_fresh_id_chirho("minBound");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: min_bound_id_chirho,
                    name_chirho: "minBound".to_string(),
                    ty_chirho: int_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(i64::MIN)),
                is_rec_chirho: false,
            });

            // maxBound :: Int (prelude-level, defaulting to Int)
            let max_bound_id_chirho = self.resolve_or_fresh_id_chirho("maxBound");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: max_bound_id_chirho,
                    name_chirho: "maxBound".to_string(),
                    ty_chirho: int_ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(i64::MAX)),
                is_rec_chirho: false,
            });
        }
    }

    /// Generate `$prim_Integral_*` bindings for Integral Int instance dictionary.
    fn generate_integral_prelude_chirho(&mut self) {
        let int_ty_chirho = TyChirho::int_chirho();
        let int2_int_chirho = TyChirho::fun_n_chirho(
            [int_ty_chirho.clone(), int_ty_chirho.clone()],
            int_ty_chirho.clone(),
        );

        // $prim_Integral_div_Int = \a b -> div# a b
        for (method_chirho, primop_chirho) in [
            ("div", "div#"),
            ("mod", "mod#"),
            ("quot", "quot#"),
            ("rem", "rem#"),
        ] {
            let prim_name_chirho = format!("$prim_Integral_{}_Int", method_chirho);
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: primop_chirho.to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho,
                    ty_chirho: int2_int_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // $prim_Integral_toInteger_Int = identity
        {
            let prim_name_chirho = "$prim_Integral_toInteger_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                },
                is_rec_chirho: false,
            });
        }

        // $prim_Integral_quotRem_Int = \a b -> (quot# a b, rem# a b)
        {
            let prim_name_chirho = "$prim_Integral_quotRem_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            let tuple_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                        con_name_chirho: "$tuple2".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::PrimOpChirho {
                                name_chirho: "quot#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                                ],
                            },
                            CoreExprChirho::PrimOpChirho {
                                name_chirho: "rem#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                                ],
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        [int_ty_chirho.clone(), int_ty_chirho.clone()],
                        tuple_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // $prim_Integral_divMod_Int = \a b -> (div# a b, mod# a b)
        {
            let prim_name_chirho = "$prim_Integral_divMod_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            let tuple_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                        con_name_chirho: "$tuple2".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::PrimOpChirho {
                                name_chirho: "div#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                                ],
                            },
                            CoreExprChirho::PrimOpChirho {
                                name_chirho: "mod#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                                ],
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        [int_ty_chirho.clone(), int_ty_chirho.clone()],
                        tuple_ty_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // Prelude-level quot and rem bindings
        {
            let quot_id_chirho = self.resolve_or_fresh_id_chirho("quot");
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: quot_id_chirho,
                    name_chirho: "quot".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        [int_ty_chirho.clone(), int_ty_chirho.clone()],
                        int_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: a_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: b_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "quot#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                CoreExprChirho::VarChirho(b_chirho.id_chirho),
                            ],
                        }),
                    }),
                },
                is_rec_chirho: false,
            });

            let rem_id_chirho = self.resolve_or_fresh_id_chirho("rem");
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: rem_id_chirho,
                    name_chirho: "rem".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        [int_ty_chirho.clone(), int_ty_chirho.clone()],
                        int_ty_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: a_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: b_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "rem#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                CoreExprChirho::VarChirho(b_chirho.id_chirho),
                            ],
                        }),
                    }),
                },
                is_rec_chirho: false,
            });
        }
    }

    /// Generate Data.Char prelude bindings: chr, ord, isDigit, isAlpha, etc.
    fn generate_char_prelude_chirho(&mut self) {
        let int_ty_chirho = TyChirho::int_chirho();
        let char_ty_chirho = TyChirho::ConChirho("Char".to_string());
        let bool_ty_chirho = TyChirho::bool_chirho();

        // Unary Char->Bool functions
        for (name_chirho, primop_chirho) in [
            ("isDigit", "isDigit#"),
            ("isAlpha", "isAlpha#"),
            ("isAlphaNum", "isAlphaNum#"),
            ("isUpper", "isUpper#"),
            ("isLower", "isLower#"),
            ("isSpace", "isSpace#"),
        ] {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            let c_chirho = self.fresh_binder_chirho("c", char_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: primop_chirho.to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(char_ty_chirho.clone(), bool_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // Unary Char->Char functions
        for (name_chirho, primop_chirho) in [
            ("toLower", "toLower#"),
            ("toUpper", "toUpper#"),
        ] {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            let c_chirho = self.fresh_binder_chirho("c", char_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: primop_chirho.to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(char_ty_chirho.clone(), char_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // chr :: Int -> Char
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("chr");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "chr#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "chr".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), char_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ord :: Char -> Int
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("ord");
            let c_chirho = self.fresh_binder_chirho("c", char_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "ord#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "ord".to_string(),
                    ty_chirho: TyChirho::fun_chirho(char_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // digitToInt :: Char -> Int
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("digitToInt");
            let c_chirho = self.fresh_binder_chirho("c", char_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "digitToInt#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "digitToInt".to_string(),
                    ty_chirho: TyChirho::fun_chirho(char_ty_chirho, int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // intToDigit :: Int -> Char
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("intToDigit");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "intToDigit#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "intToDigit".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho, TyChirho::ConChirho("Char".to_string())),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
    }

    fn generate_floating_prelude_chirho(&mut self) {
        let double_ty_chirho = TyChirho::double_chirho();
        let d2d_chirho = TyChirho::fun_chirho(double_ty_chirho.clone(), double_ty_chirho.clone());

        // Unary floating functions: $prim_Floating_<method>_Double = \x -> <method># x
        // Plus prelude-level bindings: sin = $prim_Floating_sin_Double, etc.
        for (method_chirho, primop_chirho) in [
            ("sin", "sin#"),
            ("cos", "cos#"),
            ("tan", "tan#"),
            ("asin", "asin#"),
            ("acos", "acos#"),
            ("atan", "atan#"),
            ("exp", "exp#"),
            ("log", "log#"),
            ("sqrt", "sqrt#"),
        ] {
            let prim_name_chirho = format!("$prim_Floating_{}_Double", method_chirho);
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", double_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: primop_chirho.to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.clone(),
                    ty_chirho: d2d_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });

            // Prelude-level alias: sin = $prim_Floating_sin_Double
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(method_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: method_chirho.to_string(),
                    ty_chirho: d2d_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::VarChirho(prim_id_chirho),
                is_rec_chirho: false,
            });
        }

        // pi :: Double (constant via pi# primop, wrapped as a thunk)
        {
            let prim_name_chirho = "$prim_Floating_pi_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            // pi# is dispatched as a unary primop that ignores its argument
            let dummy_chirho = self.fresh_binder_chirho("u", TyChirho::ConChirho("()".to_string()));
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: dummy_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "pi#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(dummy_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::ConChirho("()".to_string()),
                        double_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });

            // NOTE: We intentionally do NOT create a top-level "pi" alias
            // because user code may define local "pi" bindings (e.g. `where pi = 3`).
            // The $prim_Floating_pi_Double binding is available for typeclass dispatch.
        }

        // ── Num Double: abs and signum ──
        // $prim_Num_abs_Double = \n -> if n <. 0.0 then negateFloat# n else n
        {
            let prim_name_chirho = "$prim_Num_abs_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", double_ty_chirho.clone());
            let lt_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "<.#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(0.0)),
                ],
            };
            let negated_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "negateFloat#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
            };
            let case_wild_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(lt_zero_chirho),
                bind_chirho: case_wild_chirho,
                result_ty_chirho: double_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: negated_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    },
                ],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: d2d_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: n_chirho,
                    body_chirho: Box::new(body_chirho),
                },
                is_rec_chirho: false,
            });
        }

        // $prim_Num_signum_Double = \n -> if n < 0.0 then -1.0 elif n > 0.0 then 1.0 else 0.0
        {
            let prim_name_chirho = "$prim_Num_signum_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", double_ty_chirho.clone());

            let lt_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "<.#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(0.0)),
                ],
            };
            let gt_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: ">.#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(0.0)),
                ],
            };

            let w1_chirho = self.fresh_binder_chirho("$w1", TyChirho::bool_chirho());
            let w2_chirho = self.fresh_binder_chirho("$w2", TyChirho::bool_chirho());

            // Inner case: if n > 0.0 then 1.0 else 0.0
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(gt_zero_chirho),
                bind_chirho: w2_chirho,
                result_ty_chirho: double_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(1.0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(0.0)),
                    },
                ],
            };

            // Outer case: if n < 0.0 then -1.0 else (inner)
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(lt_zero_chirho),
                bind_chirho: w1_chirho,
                result_ty_chirho: double_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(-1.0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: d2d_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: n_chirho,
                    body_chirho: Box::new(body_chirho),
                },
                is_rec_chirho: false,
            });
        }

        // $prim_Num_fromInteger_Double = \n -> fromIntegral# n
        {
            let prim_name_chirho = "$prim_Num_fromInteger_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "fromIntegral#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), double_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── Num Double: binary arithmetic ops ──
        // $prim_Num_+_Double = \a b -> +.# a b
        // $prim_Num_-_Double = \a b -> -.# a b
        // $prim_Num_*_Double = \a b -> *.# a b
        for (method_chirho, primop_chirho) in [
            ("+", "+.#"),
            ("-", "-.#"),
            ("*", "*.#"),
        ] {
            let prim_name_chirho = format!("$prim_Num_{}_Double", method_chirho);
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
            let a_chirho = self.fresh_binder_chirho("a", double_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", double_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: primop_chirho.to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.clone(),
                    ty_chirho: TyChirho::fun_chirho(
                        double_ty_chirho.clone(),
                        TyChirho::fun_chirho(double_ty_chirho.clone(), double_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // $prim_Num_negate_Double = \n -> negateFloat# n
        {
            let prim_name_chirho = "$prim_Num_negate_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", double_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "negateFloat#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(double_ty_chirho.clone(), double_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── Fractional Double: /, recip, fromRational ──
        // $prim_Fractional_/_Double = \a b -> /.# a b
        {
            let prim_name_chirho = "$prim_Fractional_/_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let a_chirho = self.fresh_binder_chirho("a", double_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", double_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "/.#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        double_ty_chirho.clone(),
                        TyChirho::fun_chirho(double_ty_chirho.clone(), double_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // $prim_Fractional_recip_Double = \n -> recip# n
        {
            let prim_name_chirho = "$prim_Fractional_recip_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", double_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "recip#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: d2d_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // $prim_Fractional_fromRational_Double = \n -> fromIntegral# n
        // (Simplified: treats Rational as Int for now, converting to Double)
        {
            let prim_name_chirho = "$prim_Fractional_fromRational_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "fromIntegral#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), double_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── Floating Double prim bindings for instance dictionary ──
        // $prim_Floating_<method>_Double already generated above for sin/cos/tan/etc.
        // But the dict generation also needs them registered by this naming pattern.
        // The loop above already generates $prim_Floating_sin_Double etc.
        // We just need $prim_Floating_pi_Double which is already generated above.
    }

    fn generate_functor_monad_prelude_chirho(&mut self) {
        let any_ty_chirho = TyChirho::VarChirho(TyVarChirho(9999));

        // ── instance Functor Maybe ──
        // $prim_Functor_fmap_Maybe = \f -> \mx -> case mx of
        //     Nothing -> Nothing
        //     Just x  -> Just (f x)
        {
            let prim_name_chirho = "$prim_Functor_fmap_Maybe";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let mx_chirho = self.fresh_binder_chirho("mx", any_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let scrut_chirho = self.fresh_binder_chirho("_s", any_ty_chirho.clone());

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: mx_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mx_chirho.id_chirho)),
                        bind_chirho: scrut_chirho.clone(),
                        result_ty_chirho: any_ty_chirho.clone(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Nothing".to_string(),
                                    args_chirho: vec![],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                                binders_chirho: vec![x_chirho.clone()],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Just".to_string(),
                                    args_chirho: vec![CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                                        arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                                    }],
                                },
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });

            // Prelude-level: fmap = $prim_Functor_fmap_Maybe (default to Maybe)
            let fmap_id_chirho = self.resolve_or_fresh_id_chirho("fmap");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fmap_id_chirho,
                    name_chirho: "fmap".to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::VarChirho(prim_id_chirho),
                is_rec_chirho: false,
            });
        }

        // ── instance Functor [] ──
        // $prim_Functor_fmap_[] = map (reuse existing map binding)
        {
            let prim_name_chirho = "$prim_Functor_fmap_[]";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let map_id_chirho = self.resolve_or_fresh_id_chirho("map");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::VarChirho(map_id_chirho),
                is_rec_chirho: false,
            });
        }

        // ── instance Applicative Maybe ──
        // $prim_Applicative_pure_Maybe = \x -> Just x
        {
            let prim_name_chirho = "$prim_Applicative_pure_Maybe";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "Just".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // $prim_Applicative_<*>_Maybe = \mf mx -> case mf of
        //     Nothing -> Nothing
        //     Just f  -> case mx of
        //         Nothing -> Nothing
        //         Just x  -> Just (f x)
        {
            let prim_name_chirho = "$prim_Applicative_<*>_Maybe";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let mf_chirho = self.fresh_binder_chirho("mf", any_ty_chirho.clone());
            let mx_chirho = self.fresh_binder_chirho("mx", any_ty_chirho.clone());
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let scrut1_chirho = self.fresh_binder_chirho("_s1", any_ty_chirho.clone());
            let scrut2_chirho = self.fresh_binder_chirho("_s2", any_ty_chirho.clone());

            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mx_chirho.id_chirho)),
                bind_chirho: scrut2_chirho,
                result_ty_chirho: any_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Nothing".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho.clone()],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Just".to_string(),
                            args_chirho: vec![CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                            }],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: mf_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: mx_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mf_chirho.id_chirho)),
                        bind_chirho: scrut1_chirho,
                        result_ty_chirho: any_ty_chirho.clone(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Nothing".to_string(),
                                    args_chirho: vec![],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                                binders_chirho: vec![f_chirho.clone()],
                                rhs_chirho: inner_case_chirho,
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── instance Monad Maybe ──
        // $prim_Monad_>>=_Maybe = \mx f -> case mx of
        //     Nothing -> Nothing
        //     Just x  -> f x
        {
            let prim_name_chirho = "$prim_Monad_>>=_Maybe";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let mx_chirho = self.fresh_binder_chirho("mx", any_ty_chirho.clone());
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let scrut_chirho = self.fresh_binder_chirho("_s", any_ty_chirho.clone());

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: mx_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mx_chirho.id_chirho)),
                        bind_chirho: scrut_chirho,
                        result_ty_chirho: any_ty_chirho.clone(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Nothing".to_string(),
                                    args_chirho: vec![],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                                binders_chirho: vec![x_chirho.clone()],
                                rhs_chirho: CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                                },
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // $prim_Monad_>>_Maybe = \mx my -> case mx of
        //     Nothing -> Nothing
        //     Just _  -> my
        {
            let prim_name_chirho = "$prim_Monad_>>_Maybe";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let mx_chirho = self.fresh_binder_chirho("mx", any_ty_chirho.clone());
            let my_chirho = self.fresh_binder_chirho("my", any_ty_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("_w", any_ty_chirho.clone());
            let scrut_chirho = self.fresh_binder_chirho("_s", any_ty_chirho.clone());

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: mx_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: my_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mx_chirho.id_chirho)),
                        bind_chirho: scrut_chirho,
                        result_ty_chirho: any_ty_chirho.clone(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Nothing".to_string(),
                                    args_chirho: vec![],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                                binders_chirho: vec![w_chirho.clone()],
                                rhs_chirho: CoreExprChirho::VarChirho(my_chirho.id_chirho),
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
    }

    /// Generate IO control flow Prelude functions:
    /// when, unless, mapM_, forM_, void, sequence_, guard
    fn generate_io_control_prelude_chirho(&mut self) {
        let unit_ty_chirho = TyChirho::unit_chirho();
        let bool_ty_chirho = TyChirho::bool_chirho();
        let any_ty_chirho = TyChirho::VarChirho(TyVarChirho(9999));
        let io_unit_chirho = unit_ty_chirho.clone(); // simplified IO model

        // when :: Bool -> IO () -> IO ()
        // when True  action = action
        // when False _      = return ()
        {
            let when_id_chirho = self.resolve_or_fresh_id_chirho("when");
            let cond_chirho = self.fresh_binder_chirho("cond", bool_ty_chirho.clone());
            let action_chirho = self.fresh_binder_chirho("action", io_unit_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(cond_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("_w", bool_ty_chirho.clone()),
                result_ty_chirho: io_unit_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(action_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "returnIO#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: cond_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: action_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: when_id_chirho,
                    name_chirho: "when".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        bool_ty_chirho.clone(),
                        TyChirho::fun_chirho(io_unit_chirho.clone(), io_unit_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // unless :: Bool -> IO () -> IO ()
        // unless True  _      = return ()
        // unless False action = action
        {
            let unless_id_chirho = self.resolve_or_fresh_id_chirho("unless");
            let cond_chirho = self.fresh_binder_chirho("cond", bool_ty_chirho.clone());
            let action_chirho = self.fresh_binder_chirho("action", io_unit_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(cond_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("_u", bool_ty_chirho.clone()),
                result_ty_chirho: io_unit_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "returnIO#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(action_chirho.id_chirho),
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: cond_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: action_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: unless_id_chirho,
                    name_chirho: "unless".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        bool_ty_chirho.clone(),
                        TyChirho::fun_chirho(io_unit_chirho.clone(), io_unit_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapM_ :: (a -> IO ()) -> [a] -> IO ()
        // mapM_ f []     = return ()
        // mapM_ f (x:xs) = f x >> mapM_ f xs
        {
            let mapm_id_chirho = self.resolve_or_fresh_id_chirho("mapM_");
            let list_a_chirho = TyChirho::ListChirho(Box::new(any_ty_chirho.clone()));
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(any_ty_chirho.clone(), io_unit_chirho.clone()),
            );
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());

            let h_chirho = self.fresh_binder_chirho("h", any_ty_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_mscr", list_a_chirho.clone());

            // f x >> mapM_ f xs
            let apply_f_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(mapm_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let then_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "thenIO#".to_string(),
                args_chirho: vec![apply_f_chirho, rec_call_chirho],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: io_unit_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "returnIO#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: then_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: mapm_id_chirho,
                    name_chirho: "mapM_".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(any_ty_chirho.clone(), io_unit_chirho.clone()),
                        TyChirho::fun_chirho(list_a_chirho.clone(), io_unit_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // forM_ :: [a] -> (a -> IO ()) -> IO ()
        // forM_ xs f = mapM_ f xs
        {
            let form_id_chirho = self.resolve_or_fresh_id_chirho("forM_");
            let list_a_chirho = TyChirho::ListChirho(Box::new(any_ty_chirho.clone()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(any_ty_chirho.clone(), io_unit_chirho.clone()),
            );

            let mapm_id_chirho = self.resolve_or_fresh_id_chirho("mapM_");
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(mapm_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                    }),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: form_id_chirho,
                    name_chirho: "forM_".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        list_a_chirho,
                        TyChirho::fun_chirho(
                            TyChirho::fun_chirho(any_ty_chirho.clone(), io_unit_chirho.clone()),
                            io_unit_chirho.clone(),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // getContents :: IO String — read all stdin as a single String
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("getContents");
            let rhs_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "getContents#".to_string(),
                args_chirho: vec![],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "getContents".to_string(),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // putChar :: Char -> IO ()
        {
            let putchar_id_chirho = self.resolve_or_fresh_id_chirho("putChar");
            let c_chirho = self.fresh_binder_chirho("c", TyChirho::char_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "putChar#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: putchar_id_chirho,
                    name_chirho: "putChar".to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::char_chirho(), io_unit_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        let string_ty_chirho = TyChirho::string_chirho();

        // putStrLn :: String -> IO ()
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("putStrLn");
            let s_chirho = self.fresh_binder_chirho("s", string_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "putStrLn#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "putStrLn".to_string(),
                    ty_chirho: TyChirho::fun_chirho(string_ty_chirho.clone(), io_unit_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // putStr :: String -> IO ()
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("putStr");
            let s_chirho = self.fresh_binder_chirho("s", string_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "putStr#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "putStr".to_string(),
                    ty_chirho: TyChirho::fun_chirho(string_ty_chirho.clone(), io_unit_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // print :: a -> IO ()
        // Simplified: print x = putStrLn (show x)
        // Uses showInt# as default; the dict pass handles type-specific dispatch
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("print");
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let show_primop_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "showInt#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "putStrLn#".to_string(),
                    args_chirho: vec![show_primop_chirho],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "print".to_string(),
                    ty_chirho: TyChirho::fun_chirho(any_ty_chirho.clone(), io_unit_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── IORef operations ──

        // newIORef :: a -> IO (IORef a)
        // Simplified: newIORef val = newIORef# val (returns Int id)
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("newIORef");
            let v_chirho = self.fresh_binder_chirho("v", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: v_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "newIORef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(v_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "newIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(any_ty_chirho.clone(), TyChirho::int_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // readIORef :: IORef a -> IO a
        // Simplified: readIORef ref = readIORef# ref
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("readIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "readIORef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "readIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), any_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // writeIORef :: IORef a -> a -> IO ()
        // Simplified: writeIORef ref val = writeIORef# ref val
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("writeIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let v_chirho = self.fresh_binder_chirho("v", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "writeIORef#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(r_chirho.id_chirho),
                            CoreExprChirho::VarChirho(v_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "writeIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::int_chirho(),
                        TyChirho::fun_chirho(any_ty_chirho.clone(), unit_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // modifyIORef :: IORef a -> (a -> a) -> IO ()
        // Desugared: modifyIORef ref f = writeIORef ref (f (readIORef ref))
        // This avoids needing function application in the primop handler.
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("modifyIORef");
            let read_id_chirho = self.resolve_or_fresh_id_chirho("readIORef");
            let write_id_chirho = self.resolve_or_fresh_id_chirho("writeIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(any_ty_chirho.clone(), any_ty_chirho.clone()),
            );
            // readIORef r
            let read_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(read_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };
            // f (readIORef r)
            let apply_f_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(read_call_chirho),
            };
            // writeIORef r (f (readIORef r))
            let write_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(write_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(apply_f_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho,
                    body_chirho: Box::new(write_call_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "modifyIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::int_chirho(),
                        TyChirho::fun_chirho(
                            TyChirho::fun_chirho(any_ty_chirho.clone(), any_ty_chirho.clone()),
                            unit_ty_chirho.clone(),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // sequence_ :: [IO ()] -> IO ()
        // sequence_ []     = return ()
        // sequence_ (x:xs) = x >> sequence_ xs
        {
            let seq_id_chirho = self.resolve_or_fresh_id_chirho("sequence_");
            let list_io_chirho = TyChirho::ListChirho(Box::new(io_unit_chirho.clone()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_io_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sscr", list_io_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", io_unit_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_io_chirho.clone());

            // h >> sequence_ t
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(seq_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let then_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "thenIO#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    rec_call_chirho,
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: io_unit_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "returnIO#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: then_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: seq_id_chirho,
                    name_chirho: "sequence_".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_io_chirho, io_unit_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // void :: IO a -> IO ()
        // void action = action >> return ()
        {
            let void_id_chirho = self.resolve_or_fresh_id_chirho("void");
            let action_chirho = self.fresh_binder_chirho("action", any_ty_chirho.clone());

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: action_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "thenIO#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(action_chirho.id_chirho),
                        CoreExprChirho::PrimOpChirho {
                            name_chirho: "returnIO#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))],
                        },
                    ],
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: void_id_chirho,
                    name_chirho: "void".to_string(),
                    ty_chirho: TyChirho::fun_chirho(any_ty_chirho.clone(), io_unit_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // guard :: Bool -> IO ()
        // guard True  = return ()
        // guard False = error "guard failed"
        {
            let guard_id_chirho = self.resolve_or_fresh_id_chirho("guard");
            let b_chirho = self.fresh_binder_chirho("b", bool_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_gscr", bool_ty_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: io_unit_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "returnIO#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "error#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                                "guard failed".to_string(),
                            ))],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: b_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: guard_id_chirho,
                    name_chirho: "guard".to_string(),
                    ty_chirho: TyChirho::fun_chirho(bool_ty_chirho.clone(), io_unit_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // interact :: (String -> String) -> IO ()
        // interact f = getContents >>= \input -> putStr (f input)
        {
            let interact_id_chirho = self.resolve_or_fresh_id_chirho("interact");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(string_ty_chirho.clone(), string_ty_chirho.clone()),
            );
            let input_chirho = self.fresh_binder_chirho("input", string_ty_chirho.clone());

            // f input
            let apply_f_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(input_chirho.id_chirho)),
            };
            // putStr (f input)  — standard interact uses putStr, not putStrLn
            let putstr_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "putStr#".to_string(),
                args_chirho: vec![apply_f_chirho],
            };
            // \input -> putStr (f input)
            let callback_chirho = CoreExprChirho::LamChirho {
                binder_chirho: input_chirho,
                body_chirho: Box::new(putstr_chirho),
            };
            // getContents >>= \input -> ...
            let bind_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "bindIO#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::PrimOpChirho {
                        name_chirho: "getContents#".to_string(),
                        args_chirho: vec![],
                    },
                    callback_chirho,
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(bind_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: interact_id_chirho,
                    name_chirho: "interact".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(string_ty_chirho.clone(), string_ty_chirho.clone()),
                        io_unit_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
    }

    /// Generate Data.Map Prelude functions as thin wrappers over runtime primops.
    ///
    /// Internally the runtime represents a map as `ValueChirho::MapChirho`, a sorted
    /// association list. All heavy lifting (comparison, insertion, lookup, etc.) is
    /// handled by the STG machine via dedicated `PrimOpKindChirho::Map*` variants.
    fn generate_map_prelude_chirho(&mut self) {
        let map_ty_chirho = TyChirho::int_chirho(); // placeholder for Map k v
        let any_k_chirho = TyChirho::VarChirho(TyVarChirho(9980));
        let any_v_chirho = TyChirho::VarChirho(TyVarChirho(9981));
        let any_b_chirho = TyChirho::VarChirho(TyVarChirho(9982));
        let list_kv_ty_chirho = TyChirho::ConChirho("[kv]".to_string());
        let list_k_ty_chirho = TyChirho::ConChirho("[k]".to_string());
        let list_v_ty_chirho = TyChirho::ConChirho("[v]".to_string());
        let bool_ty_chirho = TyChirho::bool_chirho();
        let int_ty_chirho = TyChirho::int_chirho();

        // mapEmpty :: Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapEmpty");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapEmpty".to_string(),
                    ty_chirho: map_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapEmpty#".to_string(),
                    args_chirho: vec![],
                },
                is_rec_chirho: false,
            });
        }

        // mapSingleton :: k -> v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapSingleton");
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let v_chirho = self.fresh_binder_chirho("v", any_v_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapSingleton#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(v_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_k_chirho.clone(),
                TyChirho::fun_chirho(any_v_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapSingleton".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapInsert :: k -> v -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapInsert");
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let v_chirho = self.fresh_binder_chirho("v", any_v_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapInsert#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                CoreExprChirho::VarChirho(v_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_k_chirho.clone(),
                TyChirho::fun_chirho(
                    any_v_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapInsert".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapLookup :: k -> Map k v -> Maybe v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapLookup");
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let maybe_ty_chirho = TyChirho::ConChirho("Maybe".to_string());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapLookup#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_k_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), maybe_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapLookup".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapDelete :: k -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapDelete");
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapDelete#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_k_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapDelete".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapMember :: k -> Map k v -> Bool
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapMember");
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapMember#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_k_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), bool_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapMember".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapNotMember :: k -> Map k v -> Bool
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapNotMember");
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "not#".to_string(),
                        args_chirho: vec![CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapMember#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_k_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), bool_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapNotMember".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapSize :: Map k v -> Int
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapSize");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapSize#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapSize".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapNull :: Map k v -> Bool
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapNull");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapNull#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), bool_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapNull".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapFromList :: [(k, v)] -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFromList");
            let xs_chirho = self.fresh_binder_chirho("xs", list_kv_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapFromList#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(xs_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(list_kv_ty_chirho.clone(), map_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFromList".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapToList :: Map k v -> [(k, v)]
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapToList");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapToList#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), list_kv_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapToList".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapToAscList :: Map k v -> [(k, v)]  (same as toList for sorted impl)
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapToAscList");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapToList#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), list_kv_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapToAscList".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapKeys :: Map k v -> [k]
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapKeys");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapKeys#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), list_k_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapKeys".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapElems :: Map k v -> [v]
        // Also bind as mapValues (alias)
        for name_chirho in &["mapElems", "mapValues", "mapElemsAsc"] {
            let id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapElems#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), list_v_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: name_chirho.to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapMap :: (a -> b) -> Map k a -> Map k b
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapMap");
            let f_chirho = self.fresh_binder_chirho("f", any_v_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapMap#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(f_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_v_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapMap".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapFoldrWithKey :: (k -> v -> b -> b) -> b -> Map k v -> b
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFoldrWithKey");
            let f_chirho = self.fresh_binder_chirho("f", any_b_chirho.clone());
            let z_chirho = self.fresh_binder_chirho("z", any_b_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapFoldrWithKey#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(f_chirho.id_chirho),
                                CoreExprChirho::VarChirho(z_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_b_chirho.clone(),
                TyChirho::fun_chirho(
                    any_b_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), any_b_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFoldrWithKey".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapFoldlWithKey :: (b -> k -> v -> b) -> b -> Map k v -> b
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFoldlWithKey");
            let f_chirho = self.fresh_binder_chirho("f", any_b_chirho.clone());
            let z_chirho = self.fresh_binder_chirho("z", any_b_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapFoldlWithKey#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(f_chirho.id_chirho),
                                CoreExprChirho::VarChirho(z_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_b_chirho.clone(),
                TyChirho::fun_chirho(
                    any_b_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), any_b_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFoldlWithKey".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapFoldr :: (v -> b -> b) -> b -> Map k v -> b
        // Implemented as foldrWithKey ignoring the key
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFoldr");
            let f_chirho = self.fresh_binder_chirho("f", any_b_chirho.clone());
            let z_chirho = self.fresh_binder_chirho("z", any_b_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            // wrapper: \k v acc -> f v acc
            let k2_chirho = self.fresh_binder_chirho("k2", any_k_chirho.clone());
            let v2_chirho = self.fresh_binder_chirho("v2", any_v_chirho.clone());
            let acc2_chirho = self.fresh_binder_chirho("acc2", any_b_chirho.clone());
            let inner_f_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k2_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v2_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: acc2_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(v2_chirho.id_chirho)),
                            }),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(acc2_chirho.id_chirho)),
                        }),
                    }),
                }),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapFoldrWithKey#".to_string(),
                            args_chirho: vec![
                                inner_f_chirho,
                                CoreExprChirho::VarChirho(z_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_b_chirho.clone(),
                TyChirho::fun_chirho(
                    any_b_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), any_b_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFoldr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapUnion :: Map k v -> Map k v -> Map k v   (left-biased)
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapUnion");
            let m1_chirho = self.fresh_binder_chirho("m1", map_ty_chirho.clone());
            let m2_chirho = self.fresh_binder_chirho("m2", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m1_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m2_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapUnion#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(m1_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m2_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                map_ty_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapUnion".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapUnions :: [Map k v] -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapUnions");
            let xs_chirho = self.fresh_binder_chirho("xs", list_kv_ty_chirho.clone());
            // mapUnions = foldl mapUnion mapEmpty
            let union_id_chirho = self.resolve_or_fresh_id_chirho("mapUnion");
            let empty_id_chirho = self.resolve_or_fresh_id_chirho("mapEmpty");
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                self.resolve_or_fresh_id_chirho("foldl"),
                            )),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(union_id_chirho)),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(empty_id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(list_kv_ty_chirho.clone(), map_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapUnions".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapDifference :: Map k v -> Map k w -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapDifference");
            let m1_chirho = self.fresh_binder_chirho("m1", map_ty_chirho.clone());
            let m2_chirho = self.fresh_binder_chirho("m2", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m1_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m2_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapDifference#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(m1_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m2_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                map_ty_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapDifference".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapIntersection :: Map k a -> Map k b -> Map k a
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapIntersection");
            let m1_chirho = self.fresh_binder_chirho("m1", map_ty_chirho.clone());
            let m2_chirho = self.fresh_binder_chirho("m2", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m1_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m2_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapIntersection#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(m1_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m2_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                map_ty_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapIntersection".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapInsertWith :: (v -> v -> v) -> k -> v -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapInsertWith");
            let f_chirho = self.fresh_binder_chirho("f", any_b_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let v_chirho = self.fresh_binder_chirho("v", any_v_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: v_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: m_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                                name_chirho: "mapInsertWith#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(f_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(v_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(m_chirho.id_chirho),
                                ],
                            }),
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_b_chirho.clone(),
                TyChirho::fun_chirho(
                    any_k_chirho.clone(),
                    TyChirho::fun_chirho(
                        any_v_chirho.clone(),
                        TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
                    ),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapInsertWith".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapFindWithDefault :: v -> k -> Map k v -> v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFindWithDefault");
            let def_chirho = self.fresh_binder_chirho("def", any_v_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: def_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapFindWithDefault#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(def_chirho.id_chirho),
                                CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_v_chirho.clone(),
                TyChirho::fun_chirho(
                    any_k_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), any_v_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFindWithDefault".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapAdjust :: (v -> v) -> k -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapAdjust");
            let f_chirho = self.fresh_binder_chirho("f", any_v_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapAdjust#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(f_chirho.id_chirho),
                                CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_v_chirho.clone(),
                TyChirho::fun_chirho(
                    any_k_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapAdjust".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapUnionWith :: (v -> v -> v) -> Map k v -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapUnionWith");
            let f_chirho = self.fresh_binder_chirho("f", any_b_chirho.clone());
            let m1_chirho = self.fresh_binder_chirho("m1", map_ty_chirho.clone());
            let m2_chirho = self.fresh_binder_chirho("m2", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m1_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m2_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapUnionWith#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(f_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m1_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m2_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_b_chirho.clone(),
                TyChirho::fun_chirho(
                    map_ty_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapUnionWith".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
    }

    /// Generate extended Data.Map functions that delegate to the same runtime primops.
    fn generate_map_extended_chirho(&mut self) {
        let map_ty_chirho = TyChirho::int_chirho();
        let any_v_chirho = TyChirho::VarChirho(TyVarChirho(9981));

        // mapFilter :: (v -> Bool) -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFilter");
            let f_chirho = self.fresh_binder_chirho("f", any_v_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapFilter#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(f_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_v_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFilter".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapFilterWithKey :: (k -> v -> Bool) -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFilterWithKey");
            let f_chirho = self.fresh_binder_chirho("f", any_v_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapFilterWithKey#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(f_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_v_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFilterWithKey".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
    }


    /// Generate String-keyed Data.Map operations.
    /// Uses same MapEmpty/MapNode constructors but with eqStr#/ltStr# for comparison.
    /// Generate String-keyed Data.Map operations.
    ///
    /// Since the runtime `compare_values_chirho` handles `StringChirho` keys natively,
    /// these are just re-exports / aliases that delegate to the same Map runtime primops.
    fn generate_map_str_prelude_chirho(&mut self) {
        let str_ty_chirho = TyChirho::ConChirho("String".to_string());
        let any_v_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9981));
        let map_ty_chirho = TyChirho::int_chirho(); // placeholder for Map String v

        // mapInsertStr :: String -> v -> Map String v -> Map String v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapInsertStr");
            let k_chirho = self.fresh_binder_chirho("k", str_ty_chirho.clone());
            let v_chirho = self.fresh_binder_chirho("v", any_v_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapInsert#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                CoreExprChirho::VarChirho(v_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                str_ty_chirho.clone(),
                TyChirho::fun_chirho(
                    any_v_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapInsertStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapLookupStr :: String -> Map String v -> Maybe v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapLookupStr");
            let k_chirho = self.fresh_binder_chirho("k", str_ty_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let maybe_ty_chirho = TyChirho::ConChirho("Maybe".to_string());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapLookup#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                str_ty_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), maybe_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapLookupStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapDeleteStr :: String -> Map String v -> Map String v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapDeleteStr");
            let k_chirho = self.fresh_binder_chirho("k", str_ty_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapDelete#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                str_ty_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapDeleteStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapMemberStr :: String -> Map String v -> Bool
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapMemberStr");
            let k_chirho = self.fresh_binder_chirho("k", str_ty_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapMember#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                str_ty_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), TyChirho::bool_chirho()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapMemberStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapFindWithDefaultStr :: v -> String -> Map String v -> v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFindWithDefaultStr");
            let def_chirho = self.fresh_binder_chirho("def", any_v_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", str_ty_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: def_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapFindWithDefault#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(def_chirho.id_chirho),
                                CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_v_chirho.clone(),
                TyChirho::fun_chirho(
                    str_ty_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), any_v_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFindWithDefaultStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapSizeStr :: Map String v -> Int
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapSizeStr");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapSize#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), TyChirho::int_chirho());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapSizeStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // mapToListStr :: Map String v -> [(String, v)]
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapToListStr");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapToList#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                map_ty_chirho.clone(),
                TyChirho::ConChirho("[kv]".to_string()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapToListStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
    }

    /// Generate Data.Set Prelude functions using an unbalanced BST.
    ///
    /// Set is represented as:
    ///   SetEmpty                  — empty set
    ///   SetNode elem left right   — BST node
    fn generate_set_prelude_chirho(&mut self) {
        let set_ty_chirho = TyChirho::int_chirho(); // placeholder for Set a
        let any_e_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9985));


        // AVL helper: setHeight :: Set a -> Int
        {
            let height_id_chirho = self.resolve_or_fresh_id_chirho("setHeight");
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let _e_chirho = self.fresh_binder_chirho("_e", any_e_chirho.clone());
            let _l_chirho = self.fresh_binder_chirho("_l", set_ty_chirho.clone());
            let _r_chirho = self.fresh_binder_chirho("_r", set_ty_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", TyChirho::int_chirho());
            let scr_chirho = self.fresh_binder_chirho("_sh", set_ty_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetEmpty".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetNode".to_string()),
                        binders_chirho: vec![_e_chirho, _l_chirho, _r_chirho, h_chirho.clone()],
                        rhs_chirho: CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    },
                ],
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: height_id_chirho,
                    name_chirho: "setHeight".to_string(),
                    ty_chirho: TyChirho::fun_chirho(set_ty_chirho.clone(), TyChirho::int_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho,
                    body_chirho: Box::new(body_chirho),
                },
                is_rec_chirho: false,
            });
        }

        // AVL helper: setMakeNode :: e -> Set a -> Set a -> Set a
        // Constructs a SetNode with height = 1 + max(setHeight l, setHeight r)
        {
            let mknode_id_chirho = self.resolve_or_fresh_id_chirho("setMakeNode");
            let height_id_chirho = self.resolve_or_fresh_id_chirho("setHeight");
            let e_chirho = self.fresh_binder_chirho("e", any_e_chirho.clone());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());
            let hl_chirho = self.fresh_binder_chirho("hl", TyChirho::int_chirho());
            let hr_chirho = self.fresh_binder_chirho("hr", TyChirho::int_chirho());
            let max_scr_chirho = self.fresh_binder_chirho("_smx", TyChirho::bool_chirho());

            let hl_expr_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(height_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
            };
            let hr_expr_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(height_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };

            // max hl hr via if hl >= hr
            let max_cond_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: ">=#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(hl_chirho.id_chirho),
                    CoreExprChirho::VarChirho(hr_chirho.id_chirho),
                ],
            };
            let max_expr_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(max_cond_chirho),
                bind_chirho: max_scr_chirho,
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(hl_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(hr_chirho.id_chirho),
                    },
                ],
            };

            // height = 1 + max hl hr
            let new_height_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    max_expr_chirho,
                ],
            };

            // SetNode e l r height  (via lambda-application for let-binding of hl/hr)
            let inner_node_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "SetNode".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(e_chirho.id_chirho),
                    CoreExprChirho::VarChirho(l_chirho.id_chirho),
                    CoreExprChirho::VarChirho(r_chirho.id_chirho),
                    new_height_chirho,
                ],
            };
            // (\hl -> (\hr -> node) hr_expr) hl_expr
            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: hl_chirho,
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: hr_chirho,
                            body_chirho: Box::new(inner_node_chirho),
                        }),
                        arg_chirho: Box::new(hr_expr_chirho),
                    }),
                }),
                arg_chirho: Box::new(hl_expr_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: mknode_id_chirho,
                    name_chirho: "setMakeNode".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![any_e_chirho.clone(), set_ty_chirho.clone(), set_ty_chirho.clone()],
                        set_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: e_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: l_chirho,
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: r_chirho,
                            body_chirho: Box::new(body_chirho),
                        }),
                    }),
                },
                is_rec_chirho: false,
            });
        }

        // AVL helper: setBalance :: e -> Set a -> Set a -> Set a
        // Rebalances after insert/delete using AVL rotations
        {
            let balance_id_chirho = self.resolve_or_fresh_id_chirho("setBalance");
            let mknode_id_chirho = self.resolve_or_fresh_id_chirho("setMakeNode");
            let height_id_chirho = self.resolve_or_fresh_id_chirho("setHeight");
            let e_chirho = self.fresh_binder_chirho("e", any_e_chirho.clone());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());

            // For simplicity, delegate to setMakeNode (correct but not rebalancing)
            // Full AVL rotations would be added here; for now just maintain height
            let mkn_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(mknode_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };

            let _ = height_id_chirho; // suppress unused warning

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: balance_id_chirho,
                    name_chirho: "setBalance".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![any_e_chirho.clone(), set_ty_chirho.clone(), set_ty_chirho.clone()],
                        set_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: e_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: l_chirho,
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: r_chirho,
                            body_chirho: Box::new(mkn_call_chirho),
                        }),
                    }),
                },
                is_rec_chirho: false,
            });
        }

        // setEmpty :: Set Int
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setEmpty");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setEmpty".to_string(),
                    ty_chirho: set_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::ConAppChirho {
                    con_name_chirho: "SetEmpty".to_string(),
                    args_chirho: vec![],
                },
                is_rec_chirho: false,
            });
        }

        // setSingleton :: Int -> Set Int
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setSingleton");
            let x_chirho = self.fresh_binder_chirho("x", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "SetNode".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                        CoreExprChirho::ConAppChirho { con_name_chirho: "SetEmpty".to_string(), args_chirho: vec![] },
                        CoreExprChirho::ConAppChirho { con_name_chirho: "SetEmpty".to_string(), args_chirho: vec![] },
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    ],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setSingleton".to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), set_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // setInsert :: Ord a => a -> Set a -> Set a
        // Uses compare# with Ordering dispatch
        {
            let insert_id_chirho = self.resolve_or_fresh_id_chirho("setInsert");
            let x_chirho = self.fresh_binder_chirho("x", any_e_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_si", set_ty_chirho.clone());
            let e_chirho = self.fresh_binder_chirho("e", any_e_chirho.clone());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());
            let _h_si_chirho = self.fresh_binder_chirho("_h", TyChirho::int_chirho());
            let ord_scr_chirho = self.fresh_binder_chirho("_sord", TyChirho::ConChirho("Ordering".to_string()));

            // Recursive calls
            let rec_left_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(insert_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
            };
            let rec_right_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(insert_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };

            let bal_si_id = self.resolve_or_fresh_id_chirho("setBalance");
            let mkn_si_id = self.resolve_or_fresh_id_chirho("setMakeNode");
            let insert_left_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(bal_si_id)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(rec_left_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };
            let same_node_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(mkn_si_id)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };
            let insert_right_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(bal_si_id)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(rec_right_chirho),
            };

            // case compare# x e of { LT → insert left, EQ → same, GT → insert right }
            let ordering_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "compare#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                        CoreExprChirho::VarChirho(e_chirho.id_chirho),
                    ],
                }),
                bind_chirho: ord_scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: insert_left_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: same_node_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: insert_right_chirho,
                    },
                ],
            };

            // Empty case → singleton via setMakeNode
            let singleton_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(mkn_si_id)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::ConAppChirho { con_name_chirho: "SetEmpty".to_string(), args_chirho: vec![] }),
                }),
                arg_chirho: Box::new(CoreExprChirho::ConAppChirho { con_name_chirho: "SetEmpty".to_string(), args_chirho: vec![] }),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetEmpty".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: singleton_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetNode".to_string()),
                        binders_chirho: vec![e_chirho, l_chirho, r_chirho, _h_si_chirho],
                        rhs_chirho: ordering_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: insert_id_chirho,
                    name_chirho: "setInsert".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        any_e_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // setMember :: Ord a => a -> Set a -> Bool
        {
            let member_id_chirho = self.resolve_or_fresh_id_chirho("setMember");
            let x_chirho = self.fresh_binder_chirho("x", any_e_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sm", set_ty_chirho.clone());
            let e_chirho = self.fresh_binder_chirho("e", any_e_chirho.clone());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());
            let _h_sm_chirho = self.fresh_binder_chirho("_h", TyChirho::int_chirho());
            let ord_scr_chirho = self.fresh_binder_chirho("_ord", TyChirho::ConChirho("Ordering".to_string()));

            let rec_left_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(member_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
            };
            let rec_right_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(member_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };

            let compare_call_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "compare#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    CoreExprChirho::VarChirho(e_chirho.id_chirho),
                ],
            };

            let ordering_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(compare_call_chirho),
                bind_chirho: ord_scr_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_left_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_right_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetEmpty".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetNode".to_string()),
                        binders_chirho: vec![e_chirho, l_chirho, r_chirho, _h_sm_chirho],
                        rhs_chirho: ordering_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: member_id_chirho,
                    name_chirho: "setMember".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        any_e_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), TyChirho::bool_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // setSize :: Set Int -> Int
        {
            let size_id_chirho = self.resolve_or_fresh_id_chirho("setSize");
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_ss", set_ty_chirho.clone());
            let _e_chirho = self.fresh_binder_chirho("_e", TyChirho::int_chirho());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());
            let _h_ss_chirho = self.fresh_binder_chirho("_h", TyChirho::int_chirho());

            let rec_left_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(size_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
            };
            let rec_right_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(size_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetEmpty".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetNode".to_string()),
                        binders_chirho: vec![_e_chirho, l_chirho, r_chirho, _h_ss_chirho],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "+#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                                CoreExprChirho::PrimOpChirho {
                                    name_chirho: "+#".to_string(),
                                    args_chirho: vec![rec_left_chirho, rec_right_chirho],
                                },
                            ],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: size_id_chirho,
                    name_chirho: "setSize".to_string(),
                    ty_chirho: TyChirho::fun_chirho(set_ty_chirho.clone(), TyChirho::int_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // setToList :: Set Int -> [Int]  (in-order traversal)
        {
            let tolist_id_chirho = self.resolve_or_fresh_id_chirho("setToList");
            let append_id_chirho = self.resolve_or_fresh_id_chirho("append");
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_stl", set_ty_chirho.clone());
            let e_chirho = self.fresh_binder_chirho("e", TyChirho::int_chirho());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());
            let _h_stl_chirho = self.fresh_binder_chirho("_h", TyChirho::int_chirho());

            let left_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(tolist_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
            };
            let right_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(tolist_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };
            let singleton_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(e_chirho.id_chirho),
                    CoreExprChirho::ConAppChirho { con_name_chirho: "[]".to_string(), args_chirho: vec![] },
                ],
            };
            let mid_right_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(singleton_chirho),
                }),
                arg_chirho: Box::new(right_call_chirho),
            };
            let full_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(left_call_chirho),
                }),
                arg_chirho: Box::new(mid_right_chirho),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetEmpty".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetNode".to_string()),
                        binders_chirho: vec![e_chirho, l_chirho, r_chirho, _h_stl_chirho],
                        rhs_chirho: full_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: tolist_id_chirho,
                    name_chirho: "setToList".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        set_ty_chirho.clone(),
                        TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // setFromList :: [Int] -> Set Int
        {
            let fromlist_id_chirho = self.resolve_or_fresh_id_chirho("setFromList");
            let insert_id_chirho = self.resolve_or_fresh_id_chirho("setInsert");
            let xs_chirho = self.fresh_binder_chirho("xs", TyChirho::int_chirho());
            let scr_chirho = self.fresh_binder_chirho("_sfl", TyChirho::int_chirho());
            let h_chirho = self.fresh_binder_chirho("h", TyChirho::int_chirho());
            let t_chirho = self.fresh_binder_chirho("t", TyChirho::int_chirho());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(fromlist_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let insert_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(insert_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(rec_call_chirho),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "SetEmpty".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: insert_call_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fromlist_id_chirho,
                    name_chirho: "setFromList".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                        set_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // setDelete :: Ord a => a -> Set a -> Set a
        {
            let delete_id_chirho = self.resolve_or_fresh_id_chirho("setDelete");
            let x_chirho = self.fresh_binder_chirho("x", any_e_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sd", set_ty_chirho.clone());
            let e_chirho = self.fresh_binder_chirho("e", any_e_chirho.clone());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());
            let _h_sd_chirho = self.fresh_binder_chirho("_h", TyChirho::int_chirho());
            let ord_scr_chirho = self.fresh_binder_chirho("_ord", TyChirho::ConChirho("Ordering".to_string()));

            // For delete when found (eq): merge left and right subtrees
            let tolist_id_chirho = self.resolve_or_fresh_id_chirho("setToList");
            let fromlist_id_chirho = self.resolve_or_fresh_id_chirho("setFromList");
            let append_id_chirho = self.resolve_or_fresh_id_chirho("append");

            let tolist_l_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(tolist_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
            };
            let tolist_r_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(tolist_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };
            let merged_list_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(tolist_l_chirho),
                }),
                arg_chirho: Box::new(tolist_r_chirho),
            };
            let merge_result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(fromlist_id_chirho)),
                arg_chirho: Box::new(merged_list_chirho),
            };

            // Recursive calls for left/right subtree deletion
            let rec_left_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(delete_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
            };
            let rec_right_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(delete_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };

            let bal_sd_id = self.resolve_or_fresh_id_chirho("setBalance");
            let delete_left_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(bal_sd_id)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(rec_left_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };
            let delete_right_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(bal_sd_id)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(rec_right_chirho),
            };

            let compare_call_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "compare#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    CoreExprChirho::VarChirho(e_chirho.id_chirho),
                ],
            };

            let ordering_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(compare_call_chirho),
                bind_chirho: ord_scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: delete_left_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: merge_result_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: delete_right_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetEmpty".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "SetEmpty".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetNode".to_string()),
                        binders_chirho: vec![e_chirho, l_chirho, r_chirho, _h_sd_chirho],
                        rhs_chirho: ordering_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: delete_id_chirho,
                    name_chirho: "setDelete".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        any_e_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // setUnion :: Set Int -> Set Int -> Set Int
        // Fold elements of second set into first via setInsert
        {
            let union_id_chirho = self.resolve_or_fresh_id_chirho("setUnion");
            let a_chirho = self.fresh_binder_chirho("a", set_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", set_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_su", set_ty_chirho.clone());
            let e_chirho = self.fresh_binder_chirho("e", TyChirho::int_chirho());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());
            let _h_su_chirho = self.fresh_binder_chirho("_h", TyChirho::int_chirho());
            let insert_id_chirho = self.resolve_or_fresh_id_chirho("setInsert");

            // union a (SetNode e l r) = union (union (setInsert e a) l) r
            let insert_e_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(insert_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(a_chirho.id_chirho)),
            };
            let union_l_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(union_id_chirho)),
                    arg_chirho: Box::new(insert_e_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
            };
            let union_r_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(union_id_chirho)),
                    arg_chirho: Box::new(union_l_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetEmpty".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetNode".to_string()),
                        binders_chirho: vec![e_chirho, l_chirho, r_chirho, _h_su_chirho],
                        rhs_chirho: union_r_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: union_id_chirho,
                    name_chirho: "setUnion".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        set_ty_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // setIntersection :: Set Int -> Set Int -> Set Int
        // Filter elements of first set that are also in second
        {
            let inter_id_chirho = self.resolve_or_fresh_id_chirho("setIntersection");
            let a_chirho = self.fresh_binder_chirho("a", set_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", set_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_si2", set_ty_chirho.clone());
            let e_chirho = self.fresh_binder_chirho("e", TyChirho::int_chirho());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());
            let _h_si2_chirho = self.fresh_binder_chirho("_h", TyChirho::int_chirho());
            let mem_scr_chirho = self.fresh_binder_chirho("_sim", TyChirho::bool_chirho());
            let member_id_chirho = self.resolve_or_fresh_id_chirho("setMember");

            // Recurse on left and right, then check if e is in b
            let rec_left_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(inter_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
            };
            let rec_right_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(inter_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
            };
            let _insert_id_chirho = self.resolve_or_fresh_id_chirho("setInsert");
            let union_id_chirho = self.resolve_or_fresh_id_chirho("setUnion");

            // If e in b: setMakeNode e rec_left rec_right (keep it)
            let mkn_si2_id = self.resolve_or_fresh_id_chirho("setMakeNode");
            let keep_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(mkn_si2_id)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(rec_left_chirho.clone()),
                }),
                arg_chirho: Box::new(rec_right_chirho.clone()),
            };
            // If e not in b: union rec_left rec_right (drop it)
            let drop_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(union_id_chirho)),
                    arg_chirho: Box::new(rec_left_chirho),
                }),
                arg_chirho: Box::new(rec_right_chirho),
            };

            let member_check_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(member_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
                }),
                bind_chirho: mem_scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: keep_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: drop_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(a_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetEmpty".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "SetEmpty".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetNode".to_string()),
                        binders_chirho: vec![e_chirho, l_chirho, r_chirho, _h_si2_chirho],
                        rhs_chirho: member_check_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: inter_id_chirho,
                    name_chirho: "setIntersection".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        set_ty_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // setDifference :: Set Int -> Set Int -> Set Int
        // Elements in first set but not in second
        {
            let diff_id_chirho = self.resolve_or_fresh_id_chirho("setDifference");
            let a_chirho = self.fresh_binder_chirho("a", set_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", set_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sdf", set_ty_chirho.clone());
            let e_chirho = self.fresh_binder_chirho("e", TyChirho::int_chirho());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());
            let _h_sdf_chirho = self.fresh_binder_chirho("_h", TyChirho::int_chirho());
            let mem_scr_chirho = self.fresh_binder_chirho("_sdm", TyChirho::bool_chirho());
            let member_id_chirho = self.resolve_or_fresh_id_chirho("setMember");
            let union_id_chirho = self.resolve_or_fresh_id_chirho("setUnion");

            let rec_left_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(diff_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
            };
            let rec_right_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(diff_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
            };

            // If e in b: union rec_left rec_right (drop it)
            let drop_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(union_id_chirho)),
                    arg_chirho: Box::new(rec_left_chirho.clone()),
                }),
                arg_chirho: Box::new(rec_right_chirho.clone()),
            };
            // If e not in b: setMakeNode e rec_left rec_right (keep it)
            let mkn_sdf_id = self.resolve_or_fresh_id_chirho("setMakeNode");
            let keep_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(mkn_sdf_id)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(rec_left_chirho),
                }),
                arg_chirho: Box::new(rec_right_chirho),
            };

            let member_check_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(member_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
                }),
                bind_chirho: mem_scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: drop_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: keep_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(a_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetEmpty".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "SetEmpty".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetNode".to_string()),
                        binders_chirho: vec![e_chirho, l_chirho, r_chirho, _h_sdf_chirho],
                        rhs_chirho: member_check_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: diff_id_chirho,
                    name_chirho: "setDifference".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        set_ty_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // setFilter :: (Int -> Bool) -> Set Int -> Set Int
        {
            let filter_id_chirho = self.resolve_or_fresh_id_chirho("setFilter");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::int_chirho());
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sflt", set_ty_chirho.clone());
            let e_chirho = self.fresh_binder_chirho("e", TyChirho::int_chirho());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());
            let _h_sflt_chirho = self.fresh_binder_chirho("_h", TyChirho::int_chirho());
            let pred_scr_chirho = self.fresh_binder_chirho("_sfp", TyChirho::bool_chirho());
            let union_id_chirho = self.resolve_or_fresh_id_chirho("setUnion");

            let rec_left_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(filter_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
            };
            let rec_right_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(filter_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };

            // If f e → True: setMakeNode e rec_left rec_right (keep e)
            let mkn_sflt_id = self.resolve_or_fresh_id_chirho("setMakeNode");
            let keep_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(mkn_sflt_id)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(rec_left_chirho.clone()),
                }),
                arg_chirho: Box::new(rec_right_chirho.clone()),
            };
            // If f e → False: union rec_left rec_right
            let drop_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(union_id_chirho)),
                    arg_chirho: Box::new(rec_left_chirho),
                }),
                arg_chirho: Box::new(rec_right_chirho),
            };

            let pred_check_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                }),
                bind_chirho: pred_scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: keep_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: drop_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetEmpty".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "SetEmpty".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetNode".to_string()),
                        binders_chirho: vec![e_chirho, l_chirho, r_chirho, _h_sflt_chirho],
                        rhs_chirho: pred_check_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: filter_id_chirho,
                    name_chirho: "setFilter".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::bool_chirho()),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // setMap :: (Int -> Int) -> Set Int -> Set Int
        {
            let smap_id_chirho = self.resolve_or_fresh_id_chirho("setMap");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::int_chirho());
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_smp", set_ty_chirho.clone());
            let e_chirho = self.fresh_binder_chirho("e", TyChirho::int_chirho());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());
            let _h_smp_chirho = self.fresh_binder_chirho("_h", TyChirho::int_chirho());

            let rec_left_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(smap_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
            };
            let rec_right_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(smap_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: set_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetEmpty".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "SetEmpty".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetNode".to_string()),
                        binders_chirho: vec![e_chirho.clone(), l_chirho, r_chirho, _h_smp_chirho],
                        rhs_chirho: {
                            let mkn_smp_id = self.resolve_or_fresh_id_chirho("setMakeNode");
                            CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(CoreExprChirho::VarChirho(mkn_smp_id)),
                                        arg_chirho: Box::new(CoreExprChirho::AppChirho {
                                            fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                                            arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                                        }),
                                    }),
                                    arg_chirho: Box::new(rec_left_chirho),
                                }),
                                arg_chirho: Box::new(rec_right_chirho),
                            }
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: smap_id_chirho,
                    name_chirho: "setMap".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // setFold :: (Int -> b -> b) -> b -> Set Int -> b  (in-order fold)
        {
            let fold_id_chirho = self.resolve_or_fresh_id_chirho("setFold");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::int_chirho());
            let z_chirho = self.fresh_binder_chirho("z", TyChirho::int_chirho());
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sfld", set_ty_chirho.clone());
            let e_chirho = self.fresh_binder_chirho("e", TyChirho::int_chirho());
            let l_chirho = self.fresh_binder_chirho("l", set_ty_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", set_ty_chirho.clone());
            let _h_sfld_chirho = self.fresh_binder_chirho("_h", TyChirho::int_chirho());

            // fold f z (SetNode e l r) = fold f (f e (fold f z l)) r
            let fold_left_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(fold_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(z_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(l_chirho.id_chirho)),
            };
            let f_e_left_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(fold_left_chirho),
            };
            let fold_right_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(fold_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(f_e_left_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetEmpty".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(z_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("SetNode".to_string()),
                        binders_chirho: vec![e_chirho, l_chirho, r_chirho, _h_sfld_chirho],
                        rhs_chirho: fold_right_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: s_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };

            let b_chirho = TyChirho::VarChirho(TyVarChirho(3230));
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fold_id_chirho,
                    name_chirho: "setFold".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::fun_chirho(b_chirho.clone(), b_chirho.clone())),
                            b_chirho.clone(),
                            set_ty_chirho.clone(),
                        ],
                        b_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }
    }

    /// Generate instance dictionary bindings for ground instances.
    ///
    /// For each instance with no context (like `Eq Int`, `Num Int`),
    /// generates a top-level binding:
    /// ```text
    /// $fEqInt = $DictEq $prim_Eq_==_Int
    /// $fNumInt = $DictNum $fEqInt $fShowInt $prim_Num_+_Int ...
    /// ```
    pub fn generate_instance_dicts_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
    ) {
        // Two-pass approach to avoid dict id mismatch from HashMap
        // non-deterministic iteration order.  When a class with superclass
        // deps (e.g. Num → Eq) is processed before its superclass's
        // instances, `resolve_or_fresh_id_chirho` would create a fresh id
        // for the super dict reference that doesn't match the actual
        // binding created later.
        //
        // Pass 1: pre-register binder ids for ALL instance dicts so that
        //         `resolve_or_fresh_id_chirho` will find them in pass 2.
        // Pass 2: build dict bodies that reference the already-registered ids.

        // Collect eligible (class, instance, type_key, layout) tuples.
        let mut eligible_chirho: Vec<(String, String, DictLayoutChirho)> = Vec::new();

        for (class_name_chirho, instances_chirho) in &class_env_chirho.instances_chirho {
            let layout_chirho = match self.layouts_chirho.get(class_name_chirho) {
                Some(l_chirho) => l_chirho.clone(),
                None => continue,
            };

            for inst_chirho in instances_chirho {
                // Only handle ground instances (no context) for now
                if !inst_chirho.context_chirho.is_empty() {
                    continue;
                }

                let type_key_chirho = if inst_chirho.extra_head_tys_chirho.is_empty() {
                    format!("{}", inst_chirho.head_ty_chirho)
                } else {
                    let extra_chirho: Vec<String> = inst_chirho
                        .extra_head_tys_chirho
                        .iter()
                        .map(|t_chirho| format!("{}", t_chirho))
                        .collect();
                    format!("{}_{}", inst_chirho.head_ty_chirho, extra_chirho.join("_"))
                };
                eligible_chirho.push((
                    class_name_chirho.clone(),
                    type_key_chirho,
                    layout_chirho.clone(),
                ));
            }
        }

        // Inject synthetic ground instances for compound Show types.
        // These are monomorphised instances for Show (Maybe Int), etc.
        if let Some(show_layout_chirho) = self.layouts_chirho.get("Show").cloned() {
            for type_key_chirho in &[
                "Maybe Int", "Maybe String", "Maybe Double",
                "(Int,Int)", "(Int,String)", "(String,Int)", "(String,String)",
                "(Int,Bool)", "(Bool,Int)", "(Int,Double)", "(Double,Int)",
                "(Bool,Bool)", "(Double,Double)", "(Bool,String)", "(String,Bool)",
            ] {
                if !eligible_chirho.iter().any(|(c_chirho, t_chirho, _)| c_chirho == "Show" && t_chirho == *type_key_chirho) {
                    eligible_chirho.push((
                        "Show".to_string(),
                        type_key_chirho.to_string(),
                        show_layout_chirho.clone(),
                    ));
                }
            }
        }

        // Pass 1: pre-register all dict binder ids.
        let mut pre_registered_chirho: Vec<(String, String, DictLayoutChirho, BinderChirho)> =
            Vec::new();

        for (class_name_chirho, type_key_chirho, layout_chirho) in eligible_chirho {
            let dict_name_chirho =
                format!("$f{}{}", class_name_chirho, type_key_chirho);
            let dict_ty_chirho =
                TyChirho::ConChirho(format!("$Dict_{}", class_name_chirho));
            let dict_binder_chirho =
                self.fresh_binder_chirho(&dict_name_chirho, dict_ty_chirho);

            pre_registered_chirho.push((
                class_name_chirho,
                type_key_chirho,
                layout_chirho,
                dict_binder_chirho,
            ));
        }

        // Pass 2: build dict bodies — super dict ids are now findable via
        // `resolve_or_fresh_id_chirho` because pass 1 registered them.
        for (class_name_chirho, type_key_chirho, layout_chirho, dict_binder_chirho) in
            pre_registered_chirho
        {
            let con_name_chirho = format!("$Dict_{}", class_name_chirho);
            let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

            // Superclass dictionary arguments
            for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
                let super_dict_name_chirho =
                    format!("$f{}{}", super_name_chirho, type_key_chirho);
                let super_dict_id_chirho =
                    self.resolve_or_fresh_id_chirho(&super_dict_name_chirho);
                field_args_chirho.push(CoreExprChirho::VarChirho(
                    super_dict_id_chirho,
                ));
            }

            // Method implementation arguments (primitives)
            for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                let prim_name_chirho = format!(
                    "$prim_{}_{}_{}", class_name_chirho,
                    method_name_chirho, type_key_chirho
                );
                let prim_id_chirho =
                    self.resolve_or_fresh_id_chirho(&prim_name_chirho);
                field_args_chirho.push(CoreExprChirho::VarChirho(
                    prim_id_chirho,
                ));
            }

            let dict_expr_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho: field_args_chirho,
            };

            let dict_id_chirho = dict_binder_chirho.id_chirho;

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: dict_binder_chirho,
                rhs_chirho: dict_expr_chirho,
                is_rec_chirho: false,
            });

            self.instance_dicts_chirho.insert(
                (class_name_chirho, type_key_chirho),
                dict_id_chirho,
            );
        }
    }

    /// Generate dictionary bindings for GeneralizedNewtypeDeriving instances.
    ///
    /// GND instances have a context like `Num Int => Num Age` where `Age` is
    /// a newtype wrapping `Int`. For each method in the class, we generate a
    /// `$prim_Class_method_NewtypeKey` alias that points to the underlying
    /// type's prim binding, then construct the dictionary.
    pub fn generate_gnd_dicts_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
    ) {
        // Collect GND-eligible instances: context is non-empty AND the
        // head type is a known newtype.
        let mut gnd_instances_chirho: Vec<(String, String, String, DictLayoutChirho)> = Vec::new();

        for (class_name_chirho, instances_chirho) in &class_env_chirho.instances_chirho {
            let layout_chirho = match self.layouts_chirho.get(class_name_chirho) {
                Some(l_chirho) => l_chirho.clone(),
                None => continue,
            };

            for inst_chirho in instances_chirho {
                if inst_chirho.context_chirho.is_empty() {
                    continue; // Ground instance — handled by generate_instance_dicts_chirho
                }

                let type_key_chirho = format!("{}", inst_chirho.head_ty_chirho);

                // Check if the head type is a known newtype
                if let Some((_, underlying_key_chirho)) =
                    self.newtype_info_chirho.get(&type_key_chirho)
                {
                    gnd_instances_chirho.push((
                        class_name_chirho.clone(),
                        type_key_chirho,
                        underlying_key_chirho.clone(),
                        layout_chirho.clone(),
                    ));
                }
            }
        }

        for (class_name_chirho, type_key_chirho, underlying_key_chirho, layout_chirho) in
            gnd_instances_chirho
        {
            // Generate $prim aliases: $prim_Class_method_Newtype = $prim_Class_method_Underlying
            for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                let newtype_prim_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, type_key_chirho
                );
                let underlying_prim_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, underlying_key_chirho
                );
                let underlying_id_chirho =
                    self.resolve_or_fresh_id_chirho(&underlying_prim_chirho);
                let alias_binder_chirho =
                    self.fresh_binder_chirho(&newtype_prim_chirho, TyChirho::VarChirho(
                        TyVarChirho(self.next_id_chirho),
                    ));
                self.generated_bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: alias_binder_chirho,
                    rhs_chirho: CoreExprChirho::VarChirho(underlying_id_chirho),
                    is_rec_chirho: false,
                });
            }

            // Now build the dictionary (same as ground instances)
            let dict_name_chirho =
                format!("$f{}{}", class_name_chirho, type_key_chirho);
            let dict_ty_chirho =
                TyChirho::ConChirho(format!("$Dict_{}", class_name_chirho));
            let dict_binder_chirho =
                self.fresh_binder_chirho(&dict_name_chirho, dict_ty_chirho);

            let con_name_chirho = format!("$Dict_{}", class_name_chirho);
            let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

            // Superclass dictionary arguments — reference the newtype's superclass dicts
            for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
                let super_dict_name_chirho =
                    format!("$f{}{}", super_name_chirho, type_key_chirho);
                // Check if we already have this dict; if not, try underlying type
                let super_id_chirho = if self.instance_dicts_chirho.contains_key(&(
                    super_name_chirho.clone(),
                    type_key_chirho.clone(),
                )) {
                    self.resolve_or_fresh_id_chirho(&super_dict_name_chirho)
                } else {
                    let underlying_super_dict_chirho =
                        format!("$f{}{}", super_name_chirho, underlying_key_chirho);
                    self.resolve_or_fresh_id_chirho(&underlying_super_dict_chirho)
                };
                field_args_chirho.push(CoreExprChirho::VarChirho(super_id_chirho));
            }

            // Method arguments — reference the newtype's prim bindings
            for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                let prim_name_chirho = format!(
                    "$prim_{}_{}_{}", class_name_chirho,
                    method_name_chirho, type_key_chirho
                );
                let prim_id_chirho =
                    self.resolve_or_fresh_id_chirho(&prim_name_chirho);
                field_args_chirho.push(CoreExprChirho::VarChirho(prim_id_chirho));
            }

            let dict_expr_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho: field_args_chirho,
            };

            let dict_id_chirho = dict_binder_chirho.id_chirho;
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: dict_binder_chirho,
                rhs_chirho: dict_expr_chirho,
                is_rec_chirho: false,
            });

            self.instance_dicts_chirho.insert(
                (class_name_chirho, type_key_chirho),
                dict_id_chirho,
            );
        }
    }

    /// Generate ground specializations of conditional instances.
    ///
    /// For each conditional instance like `Eq a => Eq [a]`, and each
    /// element type `T` that already has a ground `Eq T` instance,
    /// generate a ground `Eq [T]` dict with the appropriate method
    /// implementations. This avoids needing full parametric conditional
    /// dict support while covering common concrete list types.
    pub fn generate_conditional_ground_dicts_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
    ) {
        // Collect conditional list instances: (class, context_classes)
        let mut cond_list_instances_chirho: Vec<(String, Vec<String>)> = Vec::new();

        for (class_name_chirho, instances_chirho) in &class_env_chirho.instances_chirho {
            for inst_chirho in instances_chirho {
                if inst_chirho.context_chirho.is_empty() {
                    continue;
                }
                // Check if this is a list instance: head_ty is List(Var(_))
                if let TyChirho::ListChirho(_) = &inst_chirho.head_ty_chirho {
                    let context_classes_chirho: Vec<String> = inst_chirho
                        .context_chirho
                        .iter()
                        .map(|p_chirho| p_chirho.class_name_chirho.clone())
                        .collect();
                    cond_list_instances_chirho.push((
                        class_name_chirho.clone(),
                        context_classes_chirho,
                    ));
                }
            }
        }

        // For each conditional list instance, generate ground dicts
        // for known element types that have the required dicts.
        let known_element_types_chirho = vec![
            "Int", "Char", "Bool", "Double",
        ];

        for (class_name_chirho, context_classes_chirho) in &cond_list_instances_chirho {
            let layout_chirho = match self.layouts_chirho.get(class_name_chirho) {
                Some(l_chirho) => l_chirho.clone(),
                None => continue,
            };

            for elem_type_chirho in &known_element_types_chirho {
                let list_type_key_chirho = format!("[{}]", elem_type_chirho);

                // Skip if we already have a ground dict for this list type
                if self
                    .instance_dicts_chirho
                    .contains_key(&(class_name_chirho.clone(), list_type_key_chirho.clone()))
                {
                    continue;
                }

                // Check that all context classes have ground dicts
                // for the element type
                let all_context_satisfied_chirho = context_classes_chirho.iter().all(
                    |ctx_class_chirho| {
                        self.instance_dicts_chirho.contains_key(&(
                            ctx_class_chirho.clone(),
                            elem_type_chirho.to_string(),
                        ))
                    },
                );

                if !all_context_satisfied_chirho {
                    continue;
                }

                // Generate the ground dict for this list type.
                // We need method implementations that use the element dict.
                self.generate_list_instance_dict_chirho(
                    class_name_chirho,
                    elem_type_chirho,
                    &list_type_key_chirho,
                    &layout_chirho,
                );
            }
        }
    }

    /// Generate Data.Maybe Prelude functions:
    ///   catMaybes :: [Maybe a] -> [a]
    ///   mapMaybe  :: (a -> Maybe b) -> [a] -> [b]
    ///   listToMaybe :: [a] -> Maybe a
    ///   maybeToList :: Maybe a -> [a]
    fn generate_maybe_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9991));

        // ── maybeToList :: Maybe a -> [a] ──
        // maybeToList Nothing  = []
        // maybeToList (Just x) = [x]
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("maybeToList");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sm", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());

            let nil_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "[]".to_string(),
                args_chirho: vec![],
            };

            let singleton_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: "[]".to_string(),
                        args_chirho: vec![],
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho],
                        rhs_chirho: singleton_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "maybeToList".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── listToMaybe :: [a] -> Maybe a ──
        // listToMaybe []    = Nothing
        // listToMaybe (x:_) = Just x
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("listToMaybe");
            let xs_chirho = self.fresh_binder_chirho("xs", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sl", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let tl_chirho = self.fresh_binder_chirho("tl", a_chirho.clone());

            let nothing_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "Nothing".to_string(),
                args_chirho: vec![],
            };

            let just_x_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "Just".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nothing_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho, tl_chirho],
                        rhs_chirho: just_x_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "listToMaybe".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── catMaybes :: [Maybe a] -> [a] ──
        // catMaybes []              = []
        // catMaybes (Nothing : xs)  = catMaybes xs
        // catMaybes (Just x  : xs)  = x : catMaybes xs
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("catMaybes");
            let xs_chirho = self.fresh_binder_chirho("xs", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sc", a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let tl_chirho = self.fresh_binder_chirho("tl", a_chirho.clone());
            let scr2_chirho = self.fresh_binder_chirho("_sm", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());

            let nil_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "[]".to_string(),
                args_chirho: vec![],
            };

            // catMaybes tl  (recursive call on tail)
            let rec_tail_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(fn_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(tl_chirho.id_chirho)),
            };

            // x : catMaybes tl
            let cons_x_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    rec_tail_chirho.clone(),
                ],
            };

            // case h of { Nothing -> catMaybes tl; Just x -> x : catMaybes tl }
            let maybe_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                bind_chirho: scr2_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_tail_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho],
                        rhs_chirho: cons_x_chirho,
                    },
                ],
            };

            // case xs of { [] -> []; (h:tl) -> case h of ... }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, tl_chirho],
                        rhs_chirho: maybe_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "catMaybes".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // ── mapMaybe :: (a -> Maybe b) -> [a] -> [b] ──
        // mapMaybe f []     = []
        // mapMaybe f (x:xs) = case f x of
        //   Nothing -> mapMaybe f xs
        //   Just y  -> y : mapMaybe f xs
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("mapMaybe");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()));
            let xs_chirho = self.fresh_binder_chirho("xs", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sl", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let tl_chirho = self.fresh_binder_chirho("tl", a_chirho.clone());
            let scr2_chirho = self.fresh_binder_chirho("_sm", b_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());

            let nil_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "[]".to_string(),
                args_chirho: vec![],
            };

            // mapMaybe f tl
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(fn_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(tl_chirho.id_chirho)),
            };

            // y : mapMaybe f tl
            let cons_y_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(y_chirho.id_chirho),
                    rec_chirho.clone(),
                ],
            };

            // f x
            let fx_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };

            // case f x of { Nothing -> rec; Just y -> y : rec }
            let maybe_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(fx_chirho),
                bind_chirho: scr2_chirho,
                result_ty_chirho: b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![y_chirho],
                        rhs_chirho: cons_y_chirho,
                    },
                ],
            };

            // case xs of { [] -> []; (x:tl) -> case f x of ... }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho, tl_chirho],
                        rhs_chirho: maybe_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "mapMaybe".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
                        TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // ── fromJust :: Maybe a -> a ──
        // fromJust (Just x) = x
        // fromJust Nothing  = error "fromJust: Nothing"
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("fromJust");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sfj", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho.clone()],
                        rhs_chirho: CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "error#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(
                                CoreLitChirho::StringChirho("fromJust: Nothing".to_string()),
                            )],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "fromJust".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── swap :: (a, b) -> (b, a) ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("swap");
            let p_chirho = self.fresh_binder_chirho("p", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sp", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![x_chirho.clone(), y_chirho.clone()],
                    rhs_chirho: CoreExprChirho::ConAppChirho {
                        con_name_chirho: "$tuple2".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(y_chirho.id_chirho),
                            CoreExprChirho::VarChirho(x_chirho.id_chirho),
                        ],
                    },
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "swap".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
    }

    /// Generate Data.IORef Prelude bindings.
    /// newIORef, readIORef, writeIORef, modifyIORef are all primops handled by
    /// the STG lowerer and runtime, but we need Core IR wrapper bindings.
    fn generate_ioref_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9990));

        // ── newIORef :: a -> IORef a ──
        // Just a wrapper that passes through to the primop
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("newIORef");
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "newIORef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "newIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), TyChirho::int_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── readIORef :: IORef a -> a ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("readIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "readIORef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "readIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── writeIORef :: IORef a -> a -> IO () ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("writeIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let v_chirho = self.fresh_binder_chirho("v", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "writeIORef#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(r_chirho.id_chirho),
                            CoreExprChirho::VarChirho(v_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "writeIORef".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![TyChirho::int_chirho(), a_chirho.clone()],
                        TyChirho::unit_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── modifyIORef :: IORef a -> (a -> a) -> IO () ──
        // Implemented as: readIORef r >>= \v -> writeIORef r (f v)
        // But since modifyIORef primop doesn't do closure application,
        // implement as: let v = readIORef r in writeIORef r (f v)
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("modifyIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()));
            let v_chirho = self.fresh_binder_chirho("v", a_chirho.clone());

            // readIORef# r
            let read_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "readIORef#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho)],
            };

            // f v
            let fv_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(v_chirho.id_chirho)),
            };

            // writeIORef# r (f v)
            let write_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "writeIORef#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(r_chirho.id_chirho),
                    fv_chirho,
                ],
            };

            // let v = readIORef# r in writeIORef# r (f v)
            let let_body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(v_chirho, read_chirho)],
                body_chirho: Box::new(write_chirho),
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho,
                    body_chirho: Box::new(let_body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "modifyIORef".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::int_chirho(),
                            TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                        ],
                        TyChirho::unit_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
    }

    /// Generate Control.Monad.ST Prelude bindings.
    /// newSTRef, readSTRef, writeSTRef, runST are primops handled by
    /// the STG lowerer and runtime, but we need Core IR wrapper bindings.
    fn generate_st_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9990));

        // ── newSTRef :: a -> ST s (STRef s a) ──
        // Simplified: STRef s a ≈ Int (same as IORef), ST s ≈ identity
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("newSTRef");
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "newSTRef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "newSTRef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), TyChirho::int_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── readSTRef :: STRef s a -> ST s a ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("readSTRef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "readSTRef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "readSTRef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── writeSTRef :: STRef s a -> a -> ST s () ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("writeSTRef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let v_chirho = self.fresh_binder_chirho("v", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "writeSTRef#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(r_chirho.id_chirho),
                            CoreExprChirho::VarChirho(v_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "writeSTRef".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![TyChirho::int_chirho(), a_chirho.clone()],
                        TyChirho::unit_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── runST :: (forall s. ST s a) -> a ──
        // Simplified: runST f = f (just evaluate the ST computation)
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("runST");
            let f_chirho = self.fresh_binder_chirho("f", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "runST#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(f_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "runST".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
    }

    /// Generate higher-order list functions: sortBy, nubBy, maximumBy,
    /// minimumBy, groupBy, on, zipWith, and related.
    fn generate_higher_order_list_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(9991));
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));

        let nil_chirho = || CoreExprChirho::ConAppChirho {
            con_name_chirho: "[]".to_string(),
            args_chirho: vec![],
        };
        let cons_chirho = |hd_chirho: CoreExprChirho, tl_chirho: CoreExprChirho| CoreExprChirho::ConAppChirho {
            con_name_chirho: ":".to_string(),
            args_chirho: vec![hd_chirho, tl_chirho],
        };

        // ── sortBy :: (a -> a -> Ordering) -> [a] -> [a] ──
        // Insertion sort using the comparison function:
        // sortBy cmp [] = []
        // sortBy cmp (x:xs) = insertBy cmp x (sortBy cmp xs)
        // insertBy cmp x [] = [x]
        // insertBy cmp x (y:ys) = case cmp x y of
        //   GT -> y : insertBy cmp x ys
        //   _  -> x : y : ys
        {
            let sortby_id_chirho = self.resolve_or_fresh_id_chirho("sortBy");
            let insertby_id_chirho = self.resolve_or_fresh_id_chirho("insertBy");

            // insertBy cmp x ys
            let cmp_ib_chirho = self.fresh_binder_chirho("cmp", a_chirho.clone());
            let x_ib_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let ys_ib_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let scr_ib_chirho = self.fresh_binder_chirho("_ib", list_a_chirho.clone());
            let y_ib_chirho = self.fresh_binder_chirho("y", a_chirho.clone());
            let rest_ib_chirho = self.fresh_binder_chirho("rest", list_a_chirho.clone());

            // cmp x y → case result of GT → y : insertBy cmp x rest; _ → x : y : rest
            let cmp_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(cmp_ib_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_ib_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_ib_chirho.id_chirho)),
            };
            let scr_cmp_chirho = self.fresh_binder_chirho("_cmpres", a_chirho.clone());

            let rec_insert_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(insertby_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(cmp_ib_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_ib_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_ib_chirho.id_chirho)),
            };

            let gt_rhs_chirho = cons_chirho(
                CoreExprChirho::VarChirho(y_ib_chirho.id_chirho),
                rec_insert_chirho,
            );
            let default_rhs_chirho = cons_chirho(
                CoreExprChirho::VarChirho(x_ib_chirho.id_chirho),
                cons_chirho(
                    CoreExprChirho::VarChirho(y_ib_chirho.id_chirho),
                    CoreExprChirho::VarChirho(rest_ib_chirho.id_chirho),
                ),
            );

            let cmp_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(cmp_call_chirho),
                bind_chirho: scr_cmp_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: gt_rhs_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: default_rhs_chirho,
                    },
                ],
            };

            let cons_alt_chirho = CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho(":".to_string()),
                binders_chirho: vec![y_ib_chirho.clone(), rest_ib_chirho.clone()],
                rhs_chirho: cmp_case_chirho,
            };

            let insertby_body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ys_ib_chirho.id_chirho)),
                bind_chirho: scr_ib_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho(CoreExprChirho::VarChirho(x_ib_chirho.id_chirho), nil_chirho()),
                    },
                    cons_alt_chirho,
                ],
            };

            let insertby_rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: cmp_ib_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: x_ib_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: ys_ib_chirho,
                        body_chirho: Box::new(insertby_body_chirho),
                    }),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: insertby_id_chirho,
                    name_chirho: "insertBy".to_string(),
                    ty_chirho: list_a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: insertby_rhs_chirho,
                is_rec_chirho: true,
            });

            // sortBy cmp [] = []
            // sortBy cmp (x:xs) = insertBy cmp x (sortBy cmp xs)
            let cmp_sb_chirho = self.fresh_binder_chirho("cmp", a_chirho.clone());
            let xs_sb_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_sb_chirho = self.fresh_binder_chirho("_sb", list_a_chirho.clone());
            let h_sb_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_sb_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());

            let rec_sort_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(sortby_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(cmp_sb_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_sb_chirho.id_chirho)),
            };

            let insert_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(insertby_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(cmp_sb_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(h_sb_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(rec_sort_chirho),
            };

            let sortby_body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_sb_chirho.id_chirho)),
                bind_chirho: scr_sb_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_sb_chirho.clone(), t_sb_chirho.clone()],
                        rhs_chirho: insert_call_chirho,
                    },
                ],
            };

            let sortby_rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: cmp_sb_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_sb_chirho,
                    body_chirho: Box::new(sortby_body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: sortby_id_chirho,
                    name_chirho: "sortBy".to_string(),
                    ty_chirho: list_a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: sortby_rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // ── nubBy :: (a -> a -> Bool) -> [a] -> [a] ──
        // nubBy eq [] = []
        // nubBy eq (x:xs) = x : nubBy eq (filter (\y -> not (eq x y)) xs)
        {
            let nubby_id_chirho = self.resolve_or_fresh_id_chirho("nubBy");
            let filter_id_chirho = self.resolve_or_fresh_id_chirho("filter");
            let not_id_chirho = self.resolve_or_fresh_id_chirho("not");

            let eq_chirho = self.fresh_binder_chirho("eq", a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_nb", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", a_chirho.clone());

            // \y -> not (eq h y)
            let eq_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(eq_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            };
            let not_eq_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(not_id_chirho)),
                arg_chirho: Box::new(eq_call_chirho),
            };
            let pred_lambda_chirho = CoreExprChirho::LamChirho {
                binder_chirho: y_chirho,
                body_chirho: Box::new(not_eq_chirho),
            };

            // filter pred t
            let filtered_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(filter_id_chirho)),
                    arg_chirho: Box::new(pred_lambda_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            // nubBy eq (filtered)
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(nubby_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(eq_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(filtered_chirho),
            };

            let cons_rhs_chirho = cons_chirho(
                CoreExprChirho::VarChirho(h_chirho.id_chirho),
                rec_chirho,
            );

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho.clone(), t_chirho.clone()],
                        rhs_chirho: cons_rhs_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: eq_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: nubby_id_chirho,
                    name_chirho: "nubBy".to_string(),
                    ty_chirho: list_a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // ── maximumBy :: (a -> a -> Ordering) -> [a] -> a ──
        // maximumBy cmp (x:xs) = foldl (\acc y -> case cmp acc y of { LT -> y; _ -> acc }) x xs
        {
            let maxby_id_chirho = self.resolve_or_fresh_id_chirho("maximumBy");
            let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");

            let cmp_chirho = self.fresh_binder_chirho("cmp", a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_mx", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let acc_chirho = self.fresh_binder_chirho("acc", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", a_chirho.clone());
            let scr_cmp_chirho = self.fresh_binder_chirho("_mc", a_chirho.clone());

            // \acc y -> case cmp acc y of { LT -> y; _ -> acc }
            let cmp_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(cmp_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(acc_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            };
            let case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(cmp_call_chirho),
                bind_chirho: scr_cmp_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(y_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(acc_chirho.id_chirho),
                    },
                ],
            };
            let fold_fn_chirho = CoreExprChirho::LamChirho {
                binder_chirho: acc_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: y_chirho,
                    body_chirho: Box::new(case_chirho),
                }),
            };

            // case xs of { (h:t) -> foldl fn h t }
            let foldl_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldl_id_chirho)),
                        arg_chirho: Box::new(fold_fn_chirho),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho.clone(), t_chirho.clone()],
                        rhs_chirho: foldl_call_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "error#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(
                                crate::expr_chirho::CoreLitChirho::StringChirho("maximumBy: empty list".to_string()),
                            )],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: cmp_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: maxby_id_chirho,
                    name_chirho: "maximumBy".to_string(),
                    ty_chirho: a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── minimumBy :: (a -> a -> Ordering) -> [a] -> a ──
        // Same as maximumBy but with GT instead of LT
        {
            let minby_id_chirho = self.resolve_or_fresh_id_chirho("minimumBy");
            let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");

            let cmp_chirho = self.fresh_binder_chirho("cmp", a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_mn", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let acc_chirho = self.fresh_binder_chirho("acc", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", a_chirho.clone());
            let scr_cmp_chirho = self.fresh_binder_chirho("_mc", a_chirho.clone());

            let cmp_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(cmp_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(acc_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            };
            let case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(cmp_call_chirho),
                bind_chirho: scr_cmp_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(y_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(acc_chirho.id_chirho),
                    },
                ],
            };
            let fold_fn_chirho = CoreExprChirho::LamChirho {
                binder_chirho: acc_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: y_chirho,
                    body_chirho: Box::new(case_chirho),
                }),
            };

            let foldl_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldl_id_chirho)),
                        arg_chirho: Box::new(fold_fn_chirho),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho.clone(), t_chirho.clone()],
                        rhs_chirho: foldl_call_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "error#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(
                                crate::expr_chirho::CoreLitChirho::StringChirho("minimumBy: empty list".to_string()),
                            )],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: cmp_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: minby_id_chirho,
                    name_chirho: "minimumBy".to_string(),
                    ty_chirho: a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── on :: (b -> b -> c) -> (a -> b) -> a -> a -> c ──
        // on f g x y = f (g x) (g y)
        {
            let on_id_chirho = self.resolve_or_fresh_id_chirho("on");
            let f_chirho = self.fresh_binder_chirho("f", a_chirho.clone());
            let g_chirho = self.fresh_binder_chirho("g", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", a_chirho.clone());

            let gx_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(g_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let gy_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(g_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(gx_chirho),
                }),
                arg_chirho: Box::new(gy_chirho),
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: g_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho,
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: y_chirho,
                            body_chirho: Box::new(body_chirho),
                        }),
                    }),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: on_id_chirho,
                    name_chirho: "on".to_string(),
                    ty_chirho: b_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }
    }

    /// Generate Semigroup and Monoid ground instance bindings.
    /// Semigroup [a] / [Char]: `<>` = `++#`  (list/string append)
    /// Monoid [a] / [Char]: `mempty` = `[]`
    fn generate_semigroup_monoid_prelude_chirho(&mut self) {
        // ── $prim_Semigroup_<>_[Char] : wraps ++# (string append) ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Semigroup_<>_[Char]");
            let a_chirho = self.fresh_binder_chirho("a", TyChirho::string_chirho());
            let b_chirho = self.fresh_binder_chirho("b", TyChirho::string_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "++#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Semigroup_<>_[Char]".to_string(),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── $prim_Semigroup_<>_[Int] etc.: delegates to `append` (list-level) ──
        let append_id_chirho = self.resolve_or_fresh_id_chirho("append");
        for type_key_chirho in &["[Int]", "[Double]", "[Bool]"] {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(
                &format!("$prim_Semigroup_<>_{}", type_key_chirho),
            );
            let a_chirho = self.fresh_binder_chirho("a", TyChirho::string_chirho());
            let b_chirho = self.fresh_binder_chirho("b", TyChirho::string_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(a_chirho.id_chirho)),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: format!("$prim_Semigroup_<>_{}", type_key_chirho),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── $prim_Monoid_mempty_[Int] etc.: returns [] ──
        for type_key_chirho in &["[Int]", "[Char]", "[Double]", "[Bool]"] {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(
                &format!("$prim_Monoid_mempty_{}", type_key_chirho),
            );
            let rhs_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "[]".to_string(),
                args_chirho: vec![],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: format!("$prim_Monoid_mempty_{}", type_key_chirho),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── Prelude-level <> alias ──
        // <> is a class method, goes through dict pass. The dict pass
        // should handle rewriting references to <> via the selector +
        // dictionary application. The $prim bindings above provide
        // the implementations for ground instances.

        // ── Prelude-level mempty alias ──
        // Same as <> — goes through dict pass.

        // ── $prim_Semigroup_<>_() : \a b -> () ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Semigroup_<>_()");
            let a_chirho = self.fresh_binder_chirho("a", TyChirho::string_chirho());
            let b_chirho = self.fresh_binder_chirho("b", TyChirho::string_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho,
                    body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                        con_name_chirho: "()".to_string(),
                        args_chirho: vec![],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Semigroup_<>_()".to_string(),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── $prim_Monoid_mempty_() : () ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Monoid_mempty_()");
            let rhs_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "()".to_string(),
                args_chirho: vec![],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Monoid_mempty_()".to_string(),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── $prim_Monoid_mconcat_() : \xs -> () ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Monoid_mconcat_()");
            let xs_chirho = self.fresh_binder_chirho("xs", TyChirho::string_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "()".to_string(),
                    args_chirho: vec![],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Monoid_mconcat_()".to_string(),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── mconcat :: [a] -> a  (foldr (<>) mempty) ──
        // For [Char] (strings): uses ++# primop.
        // For other list types: uses append function.
        let mconcat_append_id_chirho = self.resolve_or_fresh_id_chirho("append");
        for type_key_chirho in &["[Int]", "[Char]", "[Double]", "[Bool]"] {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(
                &format!("$prim_Monoid_mconcat_{}", type_key_chirho),
            );
            // mconcat xss = case xss of { [] -> []; (x:xs) -> x <> mconcat xs }
            let xss_chirho = self.fresh_binder_chirho(
                "xss",
                TyChirho::string_chirho(),
            );
            let x_chirho = self.fresh_binder_chirho(
                "x",
                TyChirho::string_chirho(),
            );
            let xs_chirho = self.fresh_binder_chirho(
                "xs",
                TyChirho::string_chirho(),
            );
            let wild_chirho = self.fresh_binder_chirho(
                "wild",
                TyChirho::string_chirho(),
            );
            // Recursive call: mconcat xs
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(fn_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            // x <> (mconcat xs) — use ++# for [Char], append for others
            let append_chirho = if *type_key_chirho == "[Char]" {
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "++#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                        rec_call_chirho,
                    ],
                }
            } else {
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(mconcat_append_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(rec_call_chirho),
                }
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xss_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::string_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho.clone(), xs_chirho.clone()],
                        rhs_chirho: append_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xss_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: format!("$prim_Monoid_mconcat_{}", type_key_chirho),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }
    }

    /// Generate a ground `Class [ElemType]` dict with working method implementations.
    fn generate_list_instance_dict_chirho(
        &mut self,
        class_name_chirho: &str,
        elem_type_chirho: &str,
        list_type_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
    ) {
        match class_name_chirho {
            "Eq" => self.generate_eq_list_dict_chirho(elem_type_chirho, list_type_key_chirho, layout_chirho),
            "Show" => self.generate_show_list_dict_chirho(elem_type_chirho, list_type_key_chirho, layout_chirho),
            _ => {
                // Generic handler for user-defined classes with conditional
                // list instances.  Look up the user's $prim_ method bindings
                // for the conditional instance (type key contains a variable,
                // e.g. "[a]") and build a dict constructor referencing them.
                self.generate_generic_list_dict_chirho(
                    class_name_chirho,
                    list_type_key_chirho,
                    layout_chirho,
                );
            }
        }
    }

    /// Generate a conditional instance dict for a user-defined class
    /// by referencing the user's `$prim_` method bindings.
    fn generate_generic_list_dict_chirho(
        &mut self,
        class_name_chirho: &str,
        list_type_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
    ) {
        let con_name_chirho = format!("$Dict_{}", class_name_chirho);
        let dict_name_chirho =
            format!("$f{}{}", class_name_chirho, list_type_key_chirho);
        let dict_ty_chirho =
            TyChirho::ConChirho(format!("$Dict_{}", class_name_chirho));
        let dict_binder_chirho =
            self.fresh_binder_chirho(&dict_name_chirho, dict_ty_chirho);

        let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

        // Superclass dict arguments
        for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
            let super_dict_name_chirho =
                format!("$f{}{}", super_name_chirho, list_type_key_chirho);
            let super_dict_id_chirho =
                self.resolve_or_fresh_id_chirho(&super_dict_name_chirho);
            field_args_chirho
                .push(CoreExprChirho::VarChirho(super_dict_id_chirho));
        }

        // Method arguments: find the user's $prim_ bindings.
        // Try type keys in order: "[a]", "[b]", etc. for variable-typed instances.
        let candidate_type_keys_chirho = vec![
            "[a]".to_string(),
            "[b]".to_string(),
            list_type_key_chirho.to_string(),
        ];

        for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
            let mut found_chirho = false;
            for tk_chirho in &candidate_type_keys_chirho {
                let prim_name_chirho = format!(
                    "$prim_{}_{}_{}", class_name_chirho,
                    method_name_chirho, tk_chirho
                );
                if let Some(prim_id_chirho) = self.lookup_name_id_chirho(&prim_name_chirho) {
                    field_args_chirho
                        .push(CoreExprChirho::VarChirho(prim_id_chirho));
                    found_chirho = true;
                    break;
                }
            }
            if !found_chirho {
                // Fallback: create a fresh reference
                let prim_name_chirho = format!(
                    "$prim_{}_{}_{}", class_name_chirho,
                    method_name_chirho, list_type_key_chirho
                );
                let prim_id_chirho =
                    self.resolve_or_fresh_id_chirho(&prim_name_chirho);
                field_args_chirho
                    .push(CoreExprChirho::VarChirho(prim_id_chirho));
            }
        }

        // Build: $fClass[T] = $Dict_Class <super_dicts> <methods>
        let dict_body_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: con_name_chirho,
            args_chirho: field_args_chirho,
        };

        self.instance_dicts_chirho.insert(
            (class_name_chirho.to_string(), list_type_key_chirho.to_string()),
            dict_binder_chirho.id_chirho,
        );

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: dict_binder_chirho,
            rhs_chirho: dict_body_chirho,
            is_rec_chirho: false,
        });
    }

    /// Generate `Eq [T]` dict with a recursive list equality function.
    fn generate_eq_list_dict_chirho(
        &mut self,
        elem_type_chirho: &str,
        list_type_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
    ) {
        let str_ty_chirho = TyChirho::string_chirho();

        // Determine the element equality primop
        let elem_eq_primop_chirho = match elem_type_chirho {
            "Double" => "eqFloat#",
            "Bool" => "==#",
            _ => "==#",
        };

        // Generate the recursive list equality function:
        // $eqList_T = \xs ys -> case xs of
        //   [] -> case ys of
        //     [] -> True (ConApp("True", []))
        //     (:) _ _ -> False (ConApp("False", []))
        //   (:) x xs' -> case ys of
        //     [] -> False
        //     (:) y ys' -> if elem_eq# x y then $eqList_T xs' ys' else False
        let fn_name_chirho = format!("$eqList_{}", elem_type_chirho);
        let fn_id_chirho = self.resolve_or_fresh_id_chirho(&fn_name_chirho);

        let xs_chirho = self.fresh_binder_chirho("xs", str_ty_chirho.clone());
        let ys_chirho = self.fresh_binder_chirho("ys", str_ty_chirho.clone());
        let x_chirho = self.fresh_binder_chirho("x", str_ty_chirho.clone());
        let xs2_chirho = self.fresh_binder_chirho("xs'", str_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("y", str_ty_chirho.clone());
        let ys2_chirho = self.fresh_binder_chirho("ys'", str_ty_chirho.clone());
        let w1_chirho = self.fresh_binder_chirho("_w1", str_ty_chirho.clone());
        let w2_chirho = self.fresh_binder_chirho("_w2", str_ty_chirho.clone());

        // The core: if elem_eq# x y then $eqList_T xs' ys' else False
        let recurse_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(fn_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs2_chirho.id_chirho)),
            }),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(ys2_chirho.id_chirho)),
        };

        let eq_test_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: elem_eq_primop_chirho.to_string(),
            args_chirho: vec![
                CoreExprChirho::VarChirho(x_chirho.id_chirho),
                CoreExprChirho::VarChirho(y_chirho.id_chirho),
            ],
        };

        let true_con_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "True".to_string(),
            args_chirho: vec![],
        };
        let false_con_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "False".to_string(),
            args_chirho: vec![],
        };

        // if elem_eq x y then recurse else False
        let if_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(eq_test_chirho),
            bind_chirho: self.fresh_binder_chirho("_eq", str_ty_chirho.clone()),
            result_ty_chirho: TyChirho::bool_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: recurse_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("False".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: false_con_chirho.clone(),
                },
            ],
        };

        // case ys of [] -> False; (:) y ys' -> if_body
        let inner_case_cons_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
            bind_chirho: self.fresh_binder_chirho("_ys", str_ty_chirho.clone()),
            result_ty_chirho: TyChirho::bool_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: false_con_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![y_chirho.clone(), ys2_chirho.clone()],
                    rhs_chirho: if_body_chirho,
                },
            ],
        };

        // case ys of [] -> True; (:) _ _ -> False (when xs is [])
        let inner_case_nil_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
            bind_chirho: self.fresh_binder_chirho("_ys2", str_ty_chirho.clone()),
            result_ty_chirho: TyChirho::bool_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: true_con_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![w1_chirho, w2_chirho],
                    rhs_chirho: false_con_chirho.clone(),
                },
            ],
        };

        // Outer: case xs of [] -> inner_nil; (:) x xs' -> inner_cons
        let outer_case_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            bind_chirho: self.fresh_binder_chirho("_xs", str_ty_chirho.clone()),
            result_ty_chirho: TyChirho::bool_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_case_nil_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![x_chirho.clone(), xs2_chirho.clone()],
                    rhs_chirho: inner_case_cons_chirho,
                },
            ],
        };

        // \xs ys -> outer_case
        let fn_body_chirho = CoreExprChirho::LamChirho {
            binder_chirho: xs_chirho,
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: ys_chirho,
                body_chirho: Box::new(outer_case_chirho),
            }),
        };

        // Register the recursive eq function
        let fn_binder_chirho = BinderChirho {
            id_chirho: fn_id_chirho,
            name_chirho: fn_name_chirho,
            ty_chirho: str_ty_chirho.clone(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: fn_binder_chirho,
            rhs_chirho: fn_body_chirho,
            is_rec_chirho: true,
        });

        // Now generate the prim binding that wraps this function
        let prim_name_chirho = format!("$prim_Eq_==_{}", list_type_key_chirho);
        let prim_binder_chirho = self.fresh_binder_chirho(&prim_name_chirho, str_ty_chirho.clone());
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: prim_binder_chirho,
            rhs_chirho: CoreExprChirho::VarChirho(fn_id_chirho),
            is_rec_chirho: false,
        });

        // Generate the Eq dict for this list type
        let dict_name_chirho = format!("$fEq{}", list_type_key_chirho);
        let dict_ty_chirho = TyChirho::ConChirho("$Dict_Eq".to_string());
        let dict_binder_chirho = self.fresh_binder_chirho(&dict_name_chirho, dict_ty_chirho);
        let dict_id_chirho = dict_binder_chirho.id_chirho;

        let con_name_chirho = "$Dict_Eq".to_string();
        let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

        // Superclass dicts (Eq has none)
        for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
            let super_dict_name_chirho =
                format!("$f{}{}", super_name_chirho, list_type_key_chirho);
            let super_dict_id_chirho =
                self.resolve_or_fresh_id_chirho(&super_dict_name_chirho);
            field_args_chirho.push(CoreExprChirho::VarChirho(super_dict_id_chirho));
        }

        // Method implementations
        for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
            let impl_name_chirho = format!(
                "$prim_Eq_{}_{}", method_name_chirho, list_type_key_chirho
            );
            let impl_id_chirho = self.resolve_or_fresh_id_chirho(&impl_name_chirho);
            field_args_chirho.push(CoreExprChirho::VarChirho(impl_id_chirho));
        }

        let dict_expr_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho: field_args_chirho,
        };

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: dict_binder_chirho,
            rhs_chirho: dict_expr_chirho,
            is_rec_chirho: false,
        });

        self.instance_dicts_chirho.insert(
            ("Eq".to_string(), list_type_key_chirho.to_string()),
            dict_id_chirho,
        );
    }

    /// Generate `Show [T]` dict for element types other than [Int] and [Char].
    fn generate_show_list_dict_chirho(
        &mut self,
        elem_type_chirho: &str,
        list_type_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
    ) {
        let str_ty_chirho = TyChirho::string_chirho();

        // Determine the element show primop
        let elem_show_primop_chirho = match elem_type_chirho {
            "Double" => "showFloat#",
            "Bool" => "showInt#", // Bool uses Int tag representation
            _ => "showInt#",
        };

        // Generate recursive show list function, similar to generate_show_list_int_binding_chirho
        // $showListTail_T = \xs -> case xs of
        //     [] -> "]"
        //     (:) x rest -> ++# "," (++# (showElem# x) ($showListTail_T rest))
        let tail_fn_name_chirho = format!("$showListTail_{}", elem_type_chirho);
        let tail_fn_id_chirho = self.resolve_or_fresh_id_chirho(&tail_fn_name_chirho);

        let tail_xs_chirho = self.fresh_binder_chirho("xs", str_ty_chirho.clone());
        let tail_x_chirho = self.fresh_binder_chirho("x", str_ty_chirho.clone());
        let tail_rest_chirho = self.fresh_binder_chirho("rest", str_ty_chirho.clone());

        let tail_cons_rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "++#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(",".to_string())),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "++#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::PrimOpChirho {
                            name_chirho: elem_show_primop_chirho.to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(
                                tail_x_chirho.id_chirho,
                            )],
                        },
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(tail_fn_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                tail_rest_chirho.id_chirho,
                            )),
                        },
                    ],
                },
            ],
        };

        let tail_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                tail_xs_chirho.id_chirho,
            )),
            bind_chirho: self.fresh_binder_chirho("_t", str_ty_chirho.clone()),
            result_ty_chirho: TyChirho::string_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "]".to_string(),
                    )),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![tail_x_chirho.clone(), tail_rest_chirho.clone()],
                    rhs_chirho: tail_cons_rhs_chirho,
                },
            ],
        };

        let tail_fn_rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: tail_xs_chirho,
            body_chirho: Box::new(tail_body_chirho),
        };

        let tail_binder_chirho = BinderChirho {
            id_chirho: tail_fn_id_chirho,
            name_chirho: tail_fn_name_chirho,
            ty_chirho: str_ty_chirho.clone(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: tail_binder_chirho,
            rhs_chirho: tail_fn_rhs_chirho,
            is_rec_chirho: true,
        });

        // Main show function:
        // $prim_Show_show_[T] = \xs -> case xs of
        //     [] -> "[]"
        //     (:) x rest -> ++# "[" (++# (showElem# x) ($showListTail_T rest))
        let main_xs_chirho = self.fresh_binder_chirho("xs", str_ty_chirho.clone());
        let main_x_chirho = self.fresh_binder_chirho("x", str_ty_chirho.clone());
        let main_rest_chirho = self.fresh_binder_chirho("rest", str_ty_chirho.clone());

        let main_cons_rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "++#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::StringChirho("[".to_string())),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "++#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::PrimOpChirho {
                            name_chirho: elem_show_primop_chirho.to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(
                                main_x_chirho.id_chirho,
                            )],
                        },
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(tail_fn_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                main_rest_chirho.id_chirho,
                            )),
                        },
                    ],
                },
            ],
        };

        let main_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                main_xs_chirho.id_chirho,
            )),
            bind_chirho: self.fresh_binder_chirho("_m", str_ty_chirho.clone()),
            result_ty_chirho: TyChirho::string_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "[]".to_string(),
                    )),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![main_x_chirho.clone(), main_rest_chirho.clone()],
                    rhs_chirho: main_cons_rhs_chirho,
                },
            ],
        };

        let main_fn_rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: main_xs_chirho,
            body_chirho: Box::new(main_body_chirho),
        };

        let prim_name_chirho = format!("$prim_Show_show_{}", list_type_key_chirho);
        let prim_binder_chirho = self.fresh_binder_chirho(&prim_name_chirho, str_ty_chirho.clone());
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: prim_binder_chirho,
            rhs_chirho: main_fn_rhs_chirho,
            is_rec_chirho: true,
        });

        // Generate the Show dict for this list type
        let dict_name_chirho = format!("$fShow{}", list_type_key_chirho);
        let dict_ty_chirho = TyChirho::ConChirho("$Dict_Show".to_string());
        let dict_binder_chirho = self.fresh_binder_chirho(&dict_name_chirho, dict_ty_chirho);
        let dict_id_chirho = dict_binder_chirho.id_chirho;

        let con_name_chirho = "$Dict_Show".to_string();
        let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

        for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
            let super_dict_name_chirho =
                format!("$f{}{}", super_name_chirho, list_type_key_chirho);
            let super_dict_id_chirho =
                self.resolve_or_fresh_id_chirho(&super_dict_name_chirho);
            field_args_chirho.push(CoreExprChirho::VarChirho(super_dict_id_chirho));
        }

        for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
            let impl_name_chirho = format!(
                "$prim_Show_{}_{}", method_name_chirho, list_type_key_chirho
            );
            let impl_id_chirho = self.resolve_or_fresh_id_chirho(&impl_name_chirho);
            field_args_chirho.push(CoreExprChirho::VarChirho(impl_id_chirho));
        }

        let dict_expr_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho: field_args_chirho,
        };

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: dict_binder_chirho,
            rhs_chirho: dict_expr_chirho,
            is_rec_chirho: false,
        });

        self.instance_dicts_chirho.insert(
            ("Show".to_string(), list_type_key_chirho.to_string()),
            dict_id_chirho,
        );
    }

    /// Collect a method application chain: if `expr_chirho` is
    /// `App^n(Var(method), arg1, ..., argN)` where `Var(method)` is a
    /// class method, return `(method_id, [arg1, ..., argN])`.
    fn collect_method_app_chirho<'a>(
        &self,
        expr_chirho: &'a CoreExprChirho,
    ) -> Option<(CoreIdChirho, Vec<&'a CoreExprChirho>)> {
        let mut args_chirho = Vec::new();
        let mut current_chirho = expr_chirho;

        // Peel off App layers, collecting arguments right-to-left
        while let CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } = current_chirho
        {
            args_chirho.push(arg_chirho.as_ref());
            current_chirho = fun_chirho.as_ref();
        }

        // Check if the innermost function is a class method Var
        if let CoreExprChirho::VarChirho(id_chirho) = current_chirho {
            if let Some(name_chirho) = self.names_chirho.get(id_chirho) {
                if self.method_selectors_chirho.contains_key(name_chirho) {
                    args_chirho.reverse(); // Now [arg1, arg2, ...]
                    return Some((*id_chirho, args_chirho));
                }
            }
        }

        None
    }

    /// Collect a dict-parameterized function application chain: if `expr_chirho`
    /// is `App^n(Var(f), arg1, ..., argN)` where `f` has dict parameters,
    /// return `(fn_id, classes, [arg1, ..., argN])`.
    fn collect_dict_param_app_chirho<'a>(
        &self,
        expr_chirho: &'a CoreExprChirho,
    ) -> Option<(CoreIdChirho, &[String], Vec<&'a CoreExprChirho>)> {
        let mut args_chirho = Vec::new();
        let mut current_chirho = expr_chirho;

        while let CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } = current_chirho
        {
            args_chirho.push(arg_chirho.as_ref());
            current_chirho = fun_chirho.as_ref();
        }

        if let CoreExprChirho::VarChirho(id_chirho) = current_chirho {
            if let Some(classes_chirho) = self.dict_param_bindings_chirho.get(id_chirho) {
                args_chirho.reverse();
                return Some((*id_chirho, classes_chirho, args_chirho));
            }
        }

        None
    }

    /// Try to rewrite a method Var reference into a dictionary projection.
    /// If `type_key_override_chirho` is provided, use it to select a
    /// type-specific instance dictionary instead of the default.
    fn try_rewrite_method_var_chirho(
        &self,
        id_chirho: CoreIdChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        type_key_override_chirho: Option<&str>,
    ) -> CoreExprChirho {
        // Skip rewriting for locally-bound variables that shadow class methods
        if self.local_shadow_ids_chirho.borrow().contains(&id_chirho) {
            return CoreExprChirho::VarChirho(id_chirho);
        }
        if let Some(name_chirho) = self.names_chirho.get(&id_chirho) {
            if let Some((class_name_chirho, sel_id_chirho)) =
                self.method_selectors_chirho.get(name_chirho)
            {
                // Try type-specific instance dict first
                if let Some(type_key_chirho) = type_key_override_chirho {
                    if let Some(dict_id_chirho) = self
                        .instance_dicts_chirho
                        .get(&(class_name_chirho.clone(), type_key_chirho.to_string()))
                    {
                        return CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                *sel_id_chirho,
                            )),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                *dict_id_chirho,
                            )),
                        };
                    }
                }

                // Fall back to default dict_vars mapping
                if let Some(dict_id_chirho) = dict_vars_chirho.get(class_name_chirho)
                {
                    return CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                            *sel_id_chirho,
                        )),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(
                            *dict_id_chirho,
                        )),
                    };
                }
            }
        }
        CoreExprChirho::VarChirho(id_chirho)
    }

    /// Rewrite method references in an expression body.
    ///
    /// Given a mapping of in-scope dictionary variables
    /// `(class_name -> dict_id)`, replaces `VarChirho` references to
    /// overloaded methods with `($sel_Class_method $dClass)`.
    ///
    /// When a class method is applied to arguments whose type can be
    /// inferred from the expression (e.g. a constructor application like
    /// `Red`), the type-specific instance dictionary is selected instead
    /// of the default one in `dict_vars_chirho`.
    fn rewrite_method_refs_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
    ) -> CoreExprChirho {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                // Check if this var references a constrained user binding
                // that needs dict arguments inserted at the call site.
                if let Some(classes_chirho) = self.dict_param_bindings_chirho.get(id_chirho) {
                    let mut result_chirho = CoreExprChirho::VarChirho(*id_chirho);
                    for class_name_chirho in classes_chirho {
                        if let Some(dict_id_chirho) = dict_vars_chirho.get(class_name_chirho) {
                            result_chirho = CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(result_chirho),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                    *dict_id_chirho,
                                )),
                            };
                        }
                    }
                    result_chirho
                } else {
                    // Rewrite a standalone method reference (not applied to args).
                    // This uses the default dict from dict_vars_chirho.
                    self.try_rewrite_method_var_chirho(*id_chirho, dict_vars_chirho, None)
                }
            }
            CoreExprChirho::LitChirho(_) => expr_chirho.clone(),
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                // Detect the pattern App(Var(method), arg) or
                // App(App(Var(method), arg1), arg2) to determine the
                // argument type for type-aware dictionary selection.
                if let Some((method_id_chirho, args_chirho)) =
                    self.collect_method_app_chirho(expr_chirho)
                {
                    // Infer the type key, combining multiple argument types
                    // for multi-parameter type classes.
                    let type_key_chirho = {
                        let method_name_chirho =
                            self.names_chirho.get(&method_id_chirho).cloned();
                        let class_name_chirho = method_name_chirho
                            .as_deref()
                            .and_then(|n_chirho| self.method_selectors_chirho.get(n_chirho))
                            .map(|(c_chirho, _)| c_chirho.clone());
                        let param_count_chirho = class_name_chirho
                            .as_deref()
                            .and_then(|cn_chirho| self.class_param_count_chirho.get(cn_chirho))
                            .copied()
                            .unwrap_or(1);

                        if param_count_chirho > 1 {
                            // MPTC: infer type keys from first N arguments
                            let keys_chirho: Vec<String> = args_chirho
                                .iter()
                                .take(param_count_chirho)
                                .filter_map(|a_chirho| self.infer_type_key_chirho(a_chirho))
                                .collect();
                            if keys_chirho.len() == param_count_chirho {
                                Some(keys_chirho.join("_"))
                            } else {
                                keys_chirho.first().cloned()
                            }
                        } else {
                            args_chirho
                                .iter()
                                .find_map(|a_chirho| self.infer_type_key_chirho(a_chirho))
                        }
                    };

                    let rewritten_method_chirho = self.try_rewrite_method_var_chirho(
                        method_id_chirho,
                        dict_vars_chirho,
                        type_key_chirho.as_deref(),
                    );

                    // Rebuild the application chain with rewritten args
                    let mut result_chirho = rewritten_method_chirho;
                    for a_chirho in &args_chirho {
                        result_chirho = CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(result_chirho),
                            arg_chirho: Box::new(
                                self.rewrite_method_refs_chirho(a_chirho, dict_vars_chirho),
                            ),
                        };
                    }
                    return result_chirho;
                }

                // Detect calls to dict-parameterized user functions:
                // App(App(Var(f), arg1), arg2) where f has dict params.
                // Infer argument types to select the right dicts.
                if let Some((fn_id_chirho, classes_chirho, args_chirho)) =
                    self.collect_dict_param_app_chirho(expr_chirho)
                {
                    // Infer the type key from the actual arguments
                    let type_key_chirho = args_chirho
                        .iter()
                        .find_map(|a_chirho| self.infer_type_key_chirho(a_chirho));

                    // Build dict args: for each required class, select the
                    // type-appropriate dict if we can infer the type
                    let mut result_chirho = CoreExprChirho::VarChirho(fn_id_chirho);
                    let classes_chirho = classes_chirho.to_vec();
                    for class_name_chirho in &classes_chirho {
                        let dict_id_chirho = if let Some(ref tk_chirho) = type_key_chirho {
                            // Try type-specific dict
                            self.instance_dicts_chirho
                                .get(&(class_name_chirho.clone(), tk_chirho.clone()))
                                .or_else(|| dict_vars_chirho.get(class_name_chirho))
                        } else {
                            dict_vars_chirho.get(class_name_chirho)
                        };
                        if let Some(dict_id_chirho) = dict_id_chirho {
                            result_chirho = CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(result_chirho),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                    *dict_id_chirho,
                                )),
                            };
                        }
                    }

                    // Rebuild the application chain with rewritten args
                    for a_chirho in &args_chirho {
                        result_chirho = CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(result_chirho),
                            arg_chirho: Box::new(
                                self.rewrite_method_refs_chirho(a_chirho, dict_vars_chirho),
                            ),
                        };
                    }
                    return result_chirho;
                }

                // Default: recursively rewrite fun and arg
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(
                        self.rewrite_method_refs_chirho(fun_chirho, dict_vars_chirho),
                    ),
                    arg_chirho: Box::new(
                        self.rewrite_method_refs_chirho(arg_chirho, dict_vars_chirho),
                    ),
                }
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                let was_new_chirho = self.local_shadow_ids_chirho.borrow_mut().insert(binder_chirho.id_chirho);
                let result_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: binder_chirho.clone(),
                    body_chirho: Box::new(
                        self.rewrite_method_refs_chirho(body_chirho, dict_vars_chirho),
                    ),
                };
                if was_new_chirho {
                    self.local_shadow_ids_chirho.borrow_mut().remove(&binder_chirho.id_chirho);
                }
                result_chirho
            },
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => {
                // Shadow let/where-bound IDs so method rewriting skips them
                let mut added_chirho = Vec::new();
                for (b_chirho, _) in binds_chirho {
                    if !self.local_shadow_ids_chirho.borrow().contains(&b_chirho.id_chirho) {
                        self.local_shadow_ids_chirho.borrow_mut().insert(b_chirho.id_chirho);
                        added_chirho.push(b_chirho.id_chirho);
                    }
                }
                let result_chirho = CoreExprChirho::LetChirho {
                    rec_chirho: *rec_chirho,
                    binds_chirho: binds_chirho
                        .iter()
                        .map(|(b_chirho, r_chirho)| {
                            (
                                b_chirho.clone(),
                                self.rewrite_method_refs_chirho(
                                    r_chirho,
                                    dict_vars_chirho,
                                ),
                            )
                        })
                        .collect(),
                    body_chirho: Box::new(
                        self.rewrite_method_refs_chirho(body_chirho, dict_vars_chirho),
                    ),
                };
                // Restore shadow set
                for id_chirho in added_chirho {
                    self.local_shadow_ids_chirho.borrow_mut().remove(&id_chirho);
                }
                result_chirho
            },
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                result_ty_chirho,
                alts_chirho,
            } => CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(
                    self.rewrite_method_refs_chirho(
                        scrutinee_chirho,
                        dict_vars_chirho,
                    ),
                ),
                bind_chirho: bind_chirho.clone(),
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: alts_chirho
                    .iter()
                    .map(|alt_chirho| CoreAltChirho {
                        con_chirho: alt_chirho.con_chirho.clone(),
                        binders_chirho: alt_chirho.binders_chirho.clone(),
                        rhs_chirho: self.rewrite_method_refs_chirho(
                            &alt_chirho.rhs_chirho,
                            dict_vars_chirho,
                        ),
                    })
                    .collect(),
            },
            CoreExprChirho::TyLamChirho {
                ty_var_chirho,
                body_chirho,
            } => CoreExprChirho::TyLamChirho {
                ty_var_chirho: ty_var_chirho.clone(),
                body_chirho: Box::new(
                    self.rewrite_method_refs_chirho(body_chirho, dict_vars_chirho),
                ),
            },
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
            } => CoreExprChirho::TyAppChirho {
                expr_chirho: Box::new(
                    self.rewrite_method_refs_chirho(inner_chirho, dict_vars_chirho),
                ),
                ty_chirho: ty_chirho.clone(),
            },
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => CoreExprChirho::PrimOpChirho {
                name_chirho: name_chirho.clone(),
                args_chirho: args_chirho
                    .iter()
                    .map(|a_chirho| self.rewrite_method_refs_chirho(a_chirho, dict_vars_chirho))
                    .collect(),
            },
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } => CoreExprChirho::ConAppChirho {
                con_name_chirho: con_name_chirho.clone(),
                args_chirho: args_chirho
                    .iter()
                    .map(|a_chirho| self.rewrite_method_refs_chirho(a_chirho, dict_vars_chirho))
                    .collect(),
            },
        }
    }

    /// Transform a constrained top-level binding by adding dictionary lambda
    /// parameters and rewriting method references in the body.
    ///
    /// Given a binding `f = rhs` where `f :: forall a. (C1 a, C2 a) => T`,
    /// produces `f = \$dC1 -> \$dC2 -> rhs'` where `rhs'` has overloaded
    /// method references replaced with dictionary projections.
    pub fn add_dict_params_chirho(
        &mut self,
        binding_chirho: &CoreBindingChirho,
        scheme_chirho: &SchemeChirho,
    ) -> CoreBindingChirho {
        // Create dictionary binders and build the class→dict_id mapping.
        // For ground predicates (concrete types like Int, Char, Bool) with
        // known instance dictionaries, resolve directly instead of
        // abstracting over a dictionary lambda parameter.
        //
        // Even unconstrained bindings may reference class methods at
        // ground types (e.g. `main = myShow 42`), so we always build a
        // dict_vars map seeded with all known ground instance dicts and
        // then rewrite method references in the body.
        let mut dict_vars_chirho: HashMap<String, CoreIdChirho> = HashMap::new();

        // Seed with ground instance dictionaries. Prefer Int instances as
        // the default dict for numeric/comparison classes, since integer
        // literals are the most common and type defaulting resolves
        // ambiguous Num/Eq/Ord/Show to Int.
        for ((class_name_chirho, type_key_chirho), dict_id_chirho) in &self.instance_dicts_chirho {
            let is_int_chirho = type_key_chirho == "Int";
            if is_int_chirho {
                // Int always wins as default
                dict_vars_chirho.insert(class_name_chirho.clone(), *dict_id_chirho);
            } else {
                dict_vars_chirho
                    .entry(class_name_chirho.clone())
                    .or_insert(*dict_id_chirho);
            }
        }

        let mut dict_binders_chirho = Vec::new();

        for pred_chirho in &scheme_chirho.preds_chirho {
            let is_ground_chirho = !matches!(pred_chirho.ty_chirho, TyChirho::VarChirho(_));
            let resolved_chirho = if is_ground_chirho {
                let type_key_chirho = format!("{}", pred_chirho.ty_chirho);
                self.instance_dicts_chirho
                    .get(&(pred_chirho.class_name_chirho.clone(), type_key_chirho))
                    .copied()
            } else if Self::is_defaultable_pred_chirho(pred_chirho, scheme_chirho) {
                // Type defaulting (Haskell 2010 §4.3.4): when a predicate
                // has an ambiguous type variable (does not appear in any
                // function argument position) and the class is one of the
                // standard numeric / Prelude classes, default to Int.
                let default_type_chirho = match pred_chirho.class_name_chirho.as_str() {
                    "Num" | "Eq" | "Ord" | "Show" | "Read" | "Enum"
                    | "Bounded" | "Integral" | "Real" | "RealFrac"
                    | "Floating" | "RealFloat" => Some("Int"),
                    _ => None,
                };
                default_type_chirho.and_then(|dt_chirho| {
                    self.instance_dicts_chirho
                        .get(&(pred_chirho.class_name_chirho.clone(), dt_chirho.to_string()))
                        .copied()
                })
            } else {
                None
            };

            if let Some(inst_id_chirho) = resolved_chirho {
                // Ground predicate with known instance — use concrete dict
                dict_vars_chirho.insert(
                    pred_chirho.class_name_chirho.clone(),
                    inst_id_chirho,
                );
            } else {
                // Unresolved — abstract over a dictionary lambda parameter
                let dict_name_chirho = format!("$d{}", pred_chirho.class_name_chirho);
                let dict_ty_chirho =
                    TyChirho::ConChirho(format!("$Dict_{}", pred_chirho.class_name_chirho));
                let dict_binder_chirho =
                    self.fresh_binder_chirho(&dict_name_chirho, dict_ty_chirho);
                dict_vars_chirho.insert(
                    pred_chirho.class_name_chirho.clone(),
                    dict_binder_chirho.id_chirho,
                );
                dict_binders_chirho.push(dict_binder_chirho);
            }
        }

        // Record which classes this binding abstracts over so that call
        // sites can insert the corresponding dict arguments.
        if !dict_binders_chirho.is_empty() {
            let classes_chirho: Vec<String> = scheme_chirho
                .preds_chirho
                .iter()
                .filter(|p_chirho| matches!(p_chirho.ty_chirho, TyChirho::VarChirho(_)))
                .filter(|p_chirho| !Self::is_defaultable_pred_chirho(p_chirho, scheme_chirho))
                .map(|p_chirho| p_chirho.class_name_chirho.clone())
                .collect();
            if !classes_chirho.is_empty() {
                self.dict_param_bindings_chirho
                    .insert(binding_chirho.binder_chirho.id_chirho, classes_chirho);
            }
        }

        // Superclass extraction: for each dict binder whose class has
        // superclasses, generate let-bindings that extract the superclass
        // dicts from the subclass dict.  This is needed after context
        // reduction removes redundant predicates (e.g. Eq a removed
        // when Num a is present, since Num has Eq as a superclass).
        let mut super_let_binds_chirho: Vec<(BinderChirho, CoreExprChirho)> = Vec::new();
        {
            // Collect classes that have dict binders (unresolved predicates)
            let classes_with_binders_chirho: Vec<String> = dict_binders_chirho
                .iter()
                .filter_map(|b_chirho| {
                    // The dict binder name is "$dClassName"
                    let name_chirho = &b_chirho.name_chirho;
                    name_chirho.strip_prefix("$d").map(|s_chirho| s_chirho.to_string())
                })
                .collect();

            // Collect (class, super, sel_id, sub_dict_id) tuples before
            // mutating self via fresh_binder_chirho.
            let mut extractions_chirho: Vec<(String, String, CoreIdChirho, CoreIdChirho)> =
                Vec::new();
            for class_name_chirho in &classes_with_binders_chirho {
                if let Some(layout_chirho) = self.layouts_chirho.get(class_name_chirho) {
                    for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
                        let already_has_binder_chirho =
                            classes_with_binders_chirho.contains(super_name_chirho);
                        if !already_has_binder_chirho {
                            if let Some(sel_id_chirho) = self.super_selectors_chirho.get(&(
                                class_name_chirho.clone(),
                                super_name_chirho.clone(),
                            )) {
                                if let Some(sub_dict_id_chirho) =
                                    dict_vars_chirho.get(class_name_chirho)
                                {
                                    extractions_chirho.push((
                                        class_name_chirho.clone(),
                                        super_name_chirho.clone(),
                                        *sel_id_chirho,
                                        *sub_dict_id_chirho,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            // Now create binders and let-bindings
            for (_class_chirho, super_name_chirho, sel_id_chirho, sub_dict_id_chirho) in
                extractions_chirho
            {
                let super_dict_binder_chirho = self.fresh_binder_chirho(
                    &format!("$d{}", super_name_chirho),
                    TyChirho::ConChirho(format!("$Dict_{}", super_name_chirho)),
                );
                let extraction_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(sub_dict_id_chirho)),
                };
                dict_vars_chirho.insert(
                    super_name_chirho,
                    super_dict_binder_chirho.id_chirho,
                );
                super_let_binds_chirho.push((super_dict_binder_chirho, extraction_chirho));
            }
        }

        // Rewrite method references in the original body
        let mut rhs_chirho =
            self.rewrite_method_refs_chirho(&binding_chirho.rhs_chirho, &dict_vars_chirho);

        // Wrap in superclass extraction let-bindings (inside dict lambdas)
        for (binder_chirho, extraction_chirho) in super_let_binds_chirho.iter().rev() {
            rhs_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(binder_chirho.clone(), extraction_chirho.clone())],
                body_chirho: Box::new(rhs_chirho),
            };
        }

        // Wrap in dictionary lambdas only for unresolved predicates.
        // For monomorphic entry points (e.g. `main`) where all method
        // calls resolved to concrete instance dicts, strip unused dict
        // lambdas so the runtime can evaluate them directly.
        let is_entry_chirho = binding_chirho.binder_chirho.name_chirho == "main";
        let free_ids_chirho = if is_entry_chirho {
            crate::simplify_chirho::free_vars_chirho(&rhs_chirho)
        } else {
            // For non-main bindings keep all dict lambdas unconditionally
            HashSet::new()
        };
        let mut used_dict_binders_chirho: Vec<&BinderChirho> = Vec::new();
        for dict_binder_chirho in dict_binders_chirho.iter().rev() {
            if !is_entry_chirho || free_ids_chirho.contains(&dict_binder_chirho.id_chirho) {
                rhs_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: dict_binder_chirho.clone(),
                    body_chirho: Box::new(rhs_chirho),
                };
                used_dict_binders_chirho.push(dict_binder_chirho);
            }
        }

        // Update the binder's type to include dictionary parameters
        // (only for dict binders that were actually kept as lambdas)
        let mut result_ty_chirho = binding_chirho.binder_chirho.ty_chirho.clone();
        for dict_binder_chirho in used_dict_binders_chirho.iter() {
            result_ty_chirho =
                TyChirho::fun_chirho(dict_binder_chirho.ty_chirho.clone(), result_ty_chirho);
        }

        CoreBindingChirho {
            binder_chirho: BinderChirho {
                ty_chirho: result_ty_chirho,
                ..binding_chirho.binder_chirho.clone()
            },
            rhs_chirho,
            is_rec_chirho: binding_chirho.is_rec_chirho,
        }
    }

    /// Run the dictionary-passing transform on a Core module.
    pub fn transform_module_chirho(
        &mut self,
        module_chirho: &CoreModuleChirho,
        type_env_chirho: &TyEnvChirho,
        class_env_chirho: &ClassEnvChirho,
    ) -> CoreModuleChirho {
        // Build layouts from the class environment
        self.build_layouts_chirho(class_env_chirho);

        // Generate method selectors
        self.generate_selectors_chirho();

        // Generate built-in $prim_ bindings for standard class methods
        self.generate_builtin_prim_bindings_chirho();

        // Generate Prelude function bindings (not, id, const)
        self.generate_prelude_bindings_chirho();

        // Generate instance dictionary bindings
        self.generate_instance_dicts_chirho(class_env_chirho);

        // Generate GND (GeneralizedNewtypeDeriving) instance dicts
        self.generate_gnd_dicts_chirho(class_env_chirho);

        // Generate ground specializations of conditional instances
        // (e.g. Eq [Double] from Eq a => Eq [a] + Eq Double)
        self.generate_conditional_ground_dicts_chirho(class_env_chirho);

        // Transform each binding
        let mut bindings_chirho = Vec::new();

        for binding_chirho in &module_chirho.bindings_chirho {
            let name_chirho = &binding_chirho.binder_chirho.name_chirho;

            // Look up the type scheme for this binding
            if let Some(scheme_chirho) = type_env_chirho.lookup_chirho(name_chirho) {
                let transformed_chirho =
                    self.add_dict_params_chirho(binding_chirho, scheme_chirho);
                bindings_chirho.push(transformed_chirho);
            } else {
                // Binding not in type env (e.g. $prim_ instance method
                // bodies).  Still rewrite class-method references in the
                // body so that Var(+) etc. are resolved to selectors.
                let mut dict_vars_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
                for ((class_name_chirho, type_key_chirho), dict_id_chirho) in &self.instance_dicts_chirho {
                    let is_int_chirho = type_key_chirho == "Int";
                    if is_int_chirho {
                        dict_vars_chirho.insert(class_name_chirho.clone(), *dict_id_chirho);
                    } else {
                        dict_vars_chirho
                            .entry(class_name_chirho.clone())
                            .or_insert(*dict_id_chirho);
                    }
                }
                let rewritten_rhs_chirho =
                    self.rewrite_method_refs_chirho(&binding_chirho.rhs_chirho, &dict_vars_chirho);
                bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: binding_chirho.binder_chirho.clone(),
                    rhs_chirho: rewritten_rhs_chirho,
                    is_rec_chirho: binding_chirho.is_rec_chirho,
                });
            }
        }

        // Prepend generated dictionary bindings
        let mut all_bindings_chirho = self.generated_bindings_chirho.clone();
        all_bindings_chirho.extend(bindings_chirho);

        CoreModuleChirho {
            name_chirho: module_chirho.name_chirho.clone(),
            bindings_chirho: all_bindings_chirho,
            names_chirho: self.names_chirho.clone(),
        }
    }

    /// Finish the transform and return the result.
    pub fn finish_chirho(self, module_chirho: CoreModuleChirho) -> DictPassResultChirho {
        DictPassResultChirho {
            module_chirho,
            names_chirho: self.names_chirho,
            layouts_chirho: self.layouts_chirho,
        }
    }
    /// Generate additional utility Prelude functions:
    /// repeat, cycle, group, groupBy, transpose, fix, zipWith3, and Data.Tuple extras.
    fn generate_utility_prelude_chirho(&mut self) {
        use rhasky_typing_chirho::ty_chirho::TyVarChirho;

        let a_chirho = TyChirho::VarChirho(TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(TyVarChirho(9991));
        let c_chirho = TyChirho::VarChirho(TyVarChirho(9992));
        let d_chirho = TyChirho::VarChirho(TyVarChirho(9993));
        let int_chirho = TyChirho::int_chirho();
        let _bool_chirho = TyChirho::bool_chirho();
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));
        let list_b_chirho = TyChirho::ListChirho(Box::new(b_chirho.clone()));
        let list_c_chirho = TyChirho::ListChirho(Box::new(c_chirho.clone()));

        let nil_chirho = || CoreExprChirho::ConAppChirho {
            con_name_chirho: "[]".to_string(),
            args_chirho: vec![],
        };
        let cons_chirho = |hd_chirho: CoreExprChirho, tl_chirho: CoreExprChirho| CoreExprChirho::ConAppChirho {
            con_name_chirho: ":".to_string(),
            args_chirho: vec![hd_chirho, tl_chirho],
        };

        // ── repeat :: a -> [a] ──
        // repeat x = x : repeat x
        {
            let repeat_id_chirho = self.resolve_or_fresh_id_chirho("repeat");
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(repeat_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let body_chirho = cons_chirho(
                CoreExprChirho::VarChirho(x_chirho.id_chirho),
                rec_call_chirho,
            );
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: repeat_id_chirho,
                    name_chirho: "repeat".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // ── cycle :: [a] -> [a] ──
        // cycle xs = xs ++ cycle xs
        // (Uses Prelude append)
        {
            let cycle_id_chirho = self.resolve_or_fresh_id_chirho("cycle");
            let append_id_chirho = self.resolve_or_fresh_id_chirho("append");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(cycle_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(rec_call_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: cycle_id_chirho,
                    name_chirho: "cycle".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // ── zipWith3 :: (a -> b -> c -> d) -> [a] -> [b] -> [c] -> [d] ──
        // zipWith3 f (a:as) (b:bs) (c:cs) = f a b c : zipWith3 f as bs cs
        // zipWith3 _ _ _ _ = []
        {
            let zw3_id_chirho = self.resolve_or_fresh_id_chirho("zipWith3");
            let f_chirho = self.fresh_binder_chirho("f", a_chirho.clone());
            let as_chirho = self.fresh_binder_chirho("as_", list_a_chirho.clone());
            let bs_chirho = self.fresh_binder_chirho("bs", list_b_chirho.clone());
            let cs_chirho = self.fresh_binder_chirho("cs", list_c_chirho.clone());
            let a1_chirho = self.fresh_binder_chirho("a1", a_chirho.clone());
            let at_chirho = self.fresh_binder_chirho("at", list_a_chirho.clone());
            let b1_chirho = self.fresh_binder_chirho("b1", b_chirho.clone());
            let bt_chirho = self.fresh_binder_chirho("bt", list_b_chirho.clone());
            let c1_chirho = self.fresh_binder_chirho("c1", c_chirho.clone());
            let ct_chirho = self.fresh_binder_chirho("ct", list_c_chirho.clone());
            let scr1_chirho = self.fresh_binder_chirho("_zscr1", list_a_chirho.clone());
            let scr2_chirho = self.fresh_binder_chirho("_zscr2", list_b_chirho.clone());
            let scr3_chirho = self.fresh_binder_chirho("_zscr3", list_c_chirho.clone());

            // f a1 b1 c1
            let apply_f_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(a1_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(b1_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(c1_chirho.id_chirho)),
            };
            // rec call
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(zw3_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(at_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(bt_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(ct_chirho.id_chirho)),
            };
            let cons_result_chirho = cons_chirho(apply_f_chirho, rec_call_chirho);

            // case cs of (c1:ct) -> cons_result; [] -> []
            let case_cs_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(cs_chirho.id_chirho)),
                bind_chirho: scr3_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![c1_chirho, ct_chirho],
                        rhs_chirho: cons_result_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                ],
            };
            // case bs of (b1:bt) -> case_cs; [] -> []
            let case_bs_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(bs_chirho.id_chirho)),
                bind_chirho: scr2_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![b1_chirho, bt_chirho],
                        rhs_chirho: case_cs_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                ],
            };
            // case as_ of (a1:at) -> case_bs; [] -> []
            let case_as_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(as_chirho.id_chirho)),
                bind_chirho: scr1_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![a1_chirho, at_chirho],
                        rhs_chirho: case_bs_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: as_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: bs_chirho,
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: cs_chirho,
                            body_chirho: Box::new(case_as_chirho),
                        }),
                    }),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: zw3_id_chirho,
                    name_chirho: "zipWith3".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::fun_n_chirho(vec![a_chirho.clone(), b_chirho.clone(), c_chirho.clone()], d_chirho.clone()),
                            list_a_chirho.clone(),
                            list_b_chirho.clone(),
                            list_c_chirho.clone(),
                        ],
                        TyChirho::ListChirho(Box::new(d_chirho.clone())),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // ── zip3 :: [a] -> [b] -> [c] -> [(a, b, c)] ──
        // zip3 = zipWith3 (\a b c -> (a, b, c))
        {
            let zip3_id_chirho = self.resolve_or_fresh_id_chirho("zip3");
            let zw3_id_chirho = self.resolve_or_fresh_id_chirho("zipWith3");
            let as_chirho = self.fresh_binder_chirho("as_", list_a_chirho.clone());
            let bs_chirho = self.fresh_binder_chirho("bs", list_b_chirho.clone());
            let cs_chirho = self.fresh_binder_chirho("cs", list_c_chirho.clone());
            let la_chirho = self.fresh_binder_chirho("a", a_chirho.clone());
            let lb_chirho = self.fresh_binder_chirho("b", b_chirho.clone());
            let lc_chirho = self.fresh_binder_chirho("c", c_chirho.clone());

            // \a b c -> (a, b, c) — represented as $tuple3 a b c
            let mk_tuple_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple3".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(la_chirho.id_chirho),
                    CoreExprChirho::VarChirho(lb_chirho.id_chirho),
                    CoreExprChirho::VarChirho(lc_chirho.id_chirho),
                ],
            };
            let tuple_lam_chirho = CoreExprChirho::LamChirho {
                binder_chirho: la_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: lb_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: lc_chirho,
                        body_chirho: Box::new(mk_tuple_chirho),
                    }),
                }),
            };

            // zipWith3 tuple_lam as bs cs
            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(zw3_id_chirho)),
                            arg_chirho: Box::new(tuple_lam_chirho),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(as_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(bs_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(cs_chirho.id_chirho)),
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: as_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: bs_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: cs_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: zip3_id_chirho,
                    name_chirho: "zip3".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![list_a_chirho.clone(), list_b_chirho.clone(), list_c_chirho.clone()],
                        list_a_chirho.clone(), // placeholder
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── fix :: (a -> a) -> a ──
        // fix f = let x = f x in x
        {
            let fix_id_chirho = self.resolve_or_fresh_id_chirho("fix");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()));
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());

            let f_x_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: true,
                binds_chirho: vec![(x_chirho.clone(), f_x_chirho)],
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fix_id_chirho,
                    name_chirho: "fix".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                        a_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── group :: [Int] -> [[Int]] ──
        // group [] = []
        // group (x:xs) = let (ys, zs) = span (==# x) xs
        //                in (x:ys) : group zs
        {
            let group_id_chirho = self.resolve_or_fresh_id_chirho("group");
            let span_id_chirho = self.resolve_or_fresh_id_chirho("span");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", int_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_gscr", list_a_chirho.clone());
            let pair_chirho = self.fresh_binder_chirho("pair", a_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let zs_chirho = self.fresh_binder_chirho("zs", list_a_chirho.clone());

            // \e -> e ==# x
            let eq_lam_param_chirho = self.fresh_binder_chirho("e", int_chirho.clone());
            let eq_lam_chirho = CoreExprChirho::LamChirho {
                binder_chirho: eq_lam_param_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "==#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(eq_lam_param_chirho.id_chirho),
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    ],
                }),
            };

            // span (\e -> e ==# x) rest
            let span_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(span_id_chirho)),
                    arg_chirho: Box::new(eq_lam_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };

            // group zs
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(group_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(zs_chirho.id_chirho)),
            };

            // (x:ys) : group zs
            let result_chirho = cons_chirho(
                cons_chirho(
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    CoreExprChirho::VarChirho(ys_chirho.id_chirho),
                ),
                rec_call_chirho,
            );

            // case pair of ($tuple2 ys zs) -> result
            let case_pair_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(pair_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("_gp", a_chirho.clone()),
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                        binders_chirho: vec![ys_chirho, zs_chirho],
                        rhs_chirho: result_chirho,
                    },
                ],
            };

            // let pair = span ... in case pair of ...
            let let_span_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(pair_chirho, span_call_chirho)],
                body_chirho: Box::new(case_pair_chirho),
            };

            // case xs of [] -> []; (x:rest) -> let_span
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho, rest_chirho],
                        rhs_chirho: let_span_chirho,
                    },
                ],
            };

            let list_list_int_chirho = TyChirho::ListChirho(Box::new(
                TyChirho::ListChirho(Box::new(int_chirho.clone())),
            ));
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: group_id_chirho,
                    name_chirho: "group".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(int_chirho.clone())),
                        list_list_int_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // ── iterate :: (a -> a) -> a -> [a] ──
        // Already exists in extra_list_prelude, skip if present
        // (this is just a placeholder comment to document coverage)

        // ── Data.Tuple: first, second, bimap ──
        // first :: (a -> b) -> (a, c) -> (b, c)
        // first f (a, c) = (f a, c)
        {
            let first_id_chirho = self.resolve_or_fresh_id_chirho("first");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()));
            let p_chirho = self.fresh_binder_chirho("p", a_chirho.clone());
            let pa_chirho = self.fresh_binder_chirho("pa", a_chirho.clone());
            let pc_chirho = self.fresh_binder_chirho("pc", c_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_fscr", a_chirho.clone());

            let apply_f_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(pa_chirho.id_chirho)),
            };
            let mk_tuple_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![apply_f_chirho, CoreExprChirho::VarChirho(pc_chirho.id_chirho)],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![pa_chirho, pc_chirho],
                    rhs_chirho: mk_tuple_chirho,
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: p_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: first_id_chirho,
                    name_chirho: "first".to_string(),
                    ty_chirho: a_chirho.clone(), // placeholder
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // second :: (b -> c) -> (a, b) -> (a, c)
        // second f (a, b) = (a, f b)
        {
            let second_id_chirho = self.resolve_or_fresh_id_chirho("second");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone()));
            let p_chirho = self.fresh_binder_chirho("p", a_chirho.clone());
            let pa_chirho = self.fresh_binder_chirho("pa", a_chirho.clone());
            let pb_chirho = self.fresh_binder_chirho("pb", b_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sscr", a_chirho.clone());

            let apply_f_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(pb_chirho.id_chirho)),
            };
            let mk_tuple_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(pa_chirho.id_chirho), apply_f_chirho],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![pa_chirho, pb_chirho],
                    rhs_chirho: mk_tuple_chirho,
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: p_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: second_id_chirho,
                    name_chirho: "second".to_string(),
                    ty_chirho: a_chirho.clone(), // placeholder
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── both :: (a -> b) -> (a, a) -> (b, b) ──
        // both f (x, y) = (f x, f y)
        {
            let both_id_chirho = self.resolve_or_fresh_id_chirho("both");
            let f_chirho = self.fresh_binder_chirho("f", TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()));
            let p_chirho = self.fresh_binder_chirho("p", a_chirho.clone());
            let px_chirho = self.fresh_binder_chirho("px", a_chirho.clone());
            let py_chirho = self.fresh_binder_chirho("py", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_bscr", a_chirho.clone());

            let fx_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(px_chirho.id_chirho)),
            };
            let fy_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(py_chirho.id_chirho)),
            };
            let mk_tuple_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![fx_chirho, fy_chirho],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![px_chirho, py_chirho],
                    rhs_chirho: mk_tuple_chirho,
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: p_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: both_id_chirho,
                    name_chirho: "both".to_string(),
                    ty_chirho: a_chirho.clone(), // placeholder
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
            });
        }

        // ── tails :: [a] -> [[a]] ──
        // tails []     = [[]]
        // tails (x:xs) = (x:xs) : tails xs
        {
            let tails_id_chirho = self.resolve_or_fresh_id_chirho("tails");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_tscr", list_a_chirho.clone());

            let list_list_a_chirho = TyChirho::ListChirho(Box::new(list_a_chirho.clone()));

            // tails xs in the cons case
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(tails_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            // (h:t) : tails t
            let whole_list_chirho = cons_chirho(
                CoreExprChirho::VarChirho(h_chirho.id_chirho),
                CoreExprChirho::VarChirho(t_chirho.id_chirho),
            );
            let cons_case_chirho = cons_chirho(whole_list_chirho, rec_call_chirho);

            // [[]] — a list containing the empty list
            let singleton_nil_chirho = cons_chirho(nil_chirho(), nil_chirho());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: singleton_nil_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: tails_id_chirho,
                    name_chirho: "tails".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }

        // ── inits :: [a] -> [[a]] ──
        // inits []     = [[]]
        // inits (x:xs) = [] : map (x:) (inits xs)
        // We implement this using a helper that accumulates the prefix.
        // Simpler recursive version:
        // inits xs = [] : case xs of
        //   []   -> []
        //   x:xs' -> map (x:) (inits xs')
        // But map (x:) requires partial application. Instead we use a direct
        // recursive approach:
        // inits [] = [[]]
        // inits (x:xs) = [] : prependAll x (inits xs)
        // where prependAll x [[]]        = [[x]]
        //       prependAll x (ys:rest)    = (x:ys) : prependAll x rest
        //       prependAll x []           = []
        //
        // Actually the simplest correct definition:
        // inits []     = [[]]
        // inits (x:xs) = [] : map (\ys -> x : ys) (inits xs)
        // which needs map. Since map is already in the Prelude, we can reference it.
        {
            let inits_id_chirho = self.resolve_or_fresh_id_chirho("inits");
            let map_id_chirho = self.resolve_or_fresh_id_chirho("map");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_iscr", list_a_chirho.clone());

            let list_list_a_chirho = TyChirho::ListChirho(Box::new(list_a_chirho.clone()));

            // (\ys -> h : ys)
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let prepend_lam_chirho = CoreExprChirho::LamChirho {
                binder_chirho: ys_chirho.clone(),
                body_chirho: Box::new(cons_chirho(
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    CoreExprChirho::VarChirho(ys_chirho.id_chirho),
                )),
            };

            // inits t
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(inits_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            // map (\ys -> h:ys) (inits t)
            let mapped_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(map_id_chirho)),
                    arg_chirho: Box::new(prepend_lam_chirho),
                }),
                arg_chirho: Box::new(rec_call_chirho),
            };

            // [] : map (\ys -> h:ys) (inits t)
            let cons_case_chirho = cons_chirho(nil_chirho(), mapped_chirho);

            // [[]]
            let singleton_nil_chirho = cons_chirho(nil_chirho(), nil_chirho());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: singleton_nil_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: inits_id_chirho,
                    name_chirho: "inits".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_list_a_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
            });
        }
    }
}

/// Convenience function: run the dictionary-passing transform on a module.
pub fn dict_pass_module_chirho(
    module_chirho: &CoreModuleChirho,
    names_chirho: HashMap<CoreIdChirho, String>,
    type_env_chirho: &TyEnvChirho,
    class_env_chirho: &ClassEnvChirho,
) -> DictPassResultChirho {
    dict_pass_module_with_con_types_chirho(
        module_chirho,
        names_chirho,
        type_env_chirho,
        class_env_chirho,
        HashMap::new(),
    )
}

/// Like `dict_pass_module_chirho` but also accepts a constructor→type mapping
/// so instance dictionaries can be selected based on argument types.
pub fn dict_pass_module_with_con_types_chirho(
    module_chirho: &CoreModuleChirho,
    names_chirho: HashMap<CoreIdChirho, String>,
    type_env_chirho: &TyEnvChirho,
    class_env_chirho: &ClassEnvChirho,
    con_types_chirho: HashMap<String, String>,
) -> DictPassResultChirho {
    dict_pass_module_full_chirho(
        module_chirho,
        names_chirho,
        type_env_chirho,
        class_env_chirho,
        con_types_chirho,
        HashMap::new(),
    )
}

/// Full dict pass with all context: constructor types and newtype info.
pub fn dict_pass_module_full_chirho(
    module_chirho: &CoreModuleChirho,
    names_chirho: HashMap<CoreIdChirho, String>,
    type_env_chirho: &TyEnvChirho,
    class_env_chirho: &ClassEnvChirho,
    con_types_chirho: HashMap<String, String>,
    newtype_info_chirho: HashMap<String, (String, String)>,
) -> DictPassResultChirho {
    let max_id_chirho = find_max_id_chirho(module_chirho);
    let mut ctx_chirho =
        DictPassCtxChirho::new_chirho(max_id_chirho + 1, names_chirho, con_types_chirho);
    ctx_chirho.newtype_info_chirho = newtype_info_chirho;
    let transformed_chirho =
        ctx_chirho.transform_module_chirho(module_chirho, type_env_chirho, class_env_chirho);
    ctx_chirho.finish_chirho(transformed_chirho)
}

/// Find the highest CoreIdChirho used in a module.
fn find_max_id_chirho(module_chirho: &CoreModuleChirho) -> u32 {
    let mut max_chirho = 0u32;
    for binding_chirho in &module_chirho.bindings_chirho {
        max_chirho = max_chirho.max(binding_chirho.binder_chirho.id_chirho.0);
        max_in_expr_chirho(&binding_chirho.rhs_chirho, &mut max_chirho);
    }
    max_chirho
}

fn max_in_expr_chirho(expr_chirho: &CoreExprChirho, max_chirho: &mut u32) {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            *max_chirho = (*max_chirho).max(id_chirho.0);
        }
        CoreExprChirho::LitChirho(_) => {}
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            max_in_expr_chirho(fun_chirho, max_chirho);
            max_in_expr_chirho(arg_chirho, max_chirho);
        }
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            *max_chirho = (*max_chirho).max(binder_chirho.id_chirho.0);
            max_in_expr_chirho(body_chirho, max_chirho);
        }
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (b_chirho, rhs_chirho) in binds_chirho {
                *max_chirho = (*max_chirho).max(b_chirho.id_chirho.0);
                max_in_expr_chirho(rhs_chirho, max_chirho);
            }
            max_in_expr_chirho(body_chirho, max_chirho);
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            max_in_expr_chirho(scrutinee_chirho, max_chirho);
            *max_chirho = (*max_chirho).max(bind_chirho.id_chirho.0);
            for alt_chirho in alts_chirho {
                for b_chirho in &alt_chirho.binders_chirho {
                    *max_chirho = (*max_chirho).max(b_chirho.id_chirho.0);
                }
                max_in_expr_chirho(&alt_chirho.rhs_chirho, max_chirho);
            }
        }
        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            max_in_expr_chirho(body_chirho, max_chirho);
        }
        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ..
        } => {
            max_in_expr_chirho(inner_chirho, max_chirho);
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. }
        | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                max_in_expr_chirho(arg_chirho, max_chirho);
            }
        }
    }
}
#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::expr_chirho::CoreLitChirho;
    use rhasky_typing_chirho::class_chirho::ClassEnvChirho;
    use rhasky_typing_chirho::ty_chirho::SchemePredChirho;

    fn dummy_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
        BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn unconstrained_binding_unchanged_chirho() {
        let binding_chirho = CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("f", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
            is_rec_chirho: false,
        };
        let scheme_chirho = SchemeChirho::mono_chirho(TyChirho::int_chirho());

        let mut ctx_chirho =
            DictPassCtxChirho::new_chirho(10, HashMap::new(), HashMap::new());
        let result_chirho =
            ctx_chirho.add_dict_params_chirho(&binding_chirho, &scheme_chirho);

        // No predicates, so binding is unchanged
        assert_eq!(
            result_chirho.rhs_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
        );
    }

    #[test]
    fn single_predicate_adds_dict_lambda_chirho() {
        let binding_chirho = CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("add", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
            is_rec_chirho: false,
        };
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![rhasky_typing_chirho::ty_chirho::TyVarChirho(0)],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Num".to_string(),
                ty_chirho: TyChirho::VarChirho(
                    rhasky_typing_chirho::ty_chirho::TyVarChirho(0),
                ),
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(0)),
                TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(0)),
            ),
        };

        let mut ctx_chirho =
            DictPassCtxChirho::new_chirho(10, HashMap::new(), HashMap::new());
        let result_chirho =
            ctx_chirho.add_dict_params_chirho(&binding_chirho, &scheme_chirho);

        // Should be wrapped in a lambda: \$dNum -> 0
        assert!(matches!(
            result_chirho.rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));

        if let CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } = &result_chirho.rhs_chirho
        {
            assert_eq!(binder_chirho.name_chirho, "$dNum");
            assert_eq!(
                binder_chirho.ty_chirho,
                TyChirho::ConChirho("$Dict_Num".to_string())
            );
            assert_eq!(
                **body_chirho,
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
            );
        }

        // Binder type should be: $Dict_Num -> (t0 -> t0)
        assert!(matches!(
            result_chirho.binder_chirho.ty_chirho,
            TyChirho::FunChirho(_, _)
        ));
    }

    #[test]
    fn multiple_predicates_add_nested_dict_lambdas_chirho() {
        let binding_chirho = CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("cmp", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
            is_rec_chirho: false,
        };
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![rhasky_typing_chirho::ty_chirho::TyVarChirho(0)],
            preds_chirho: vec![
                SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(0),
                    ),
                },
                SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(0),
                    ),
                },
            ],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(0)),
                TyChirho::bool_chirho(),
            ),
        };

        let mut ctx_chirho =
            DictPassCtxChirho::new_chirho(10, HashMap::new(), HashMap::new());
        let result_chirho =
            ctx_chirho.add_dict_params_chirho(&binding_chirho, &scheme_chirho);

        // Should be: \$dEq -> \$dOrd -> 0
        if let CoreExprChirho::LamChirho {
            binder_chirho: outer_chirho,
            body_chirho,
        } = &result_chirho.rhs_chirho
        {
            assert_eq!(outer_chirho.name_chirho, "$dEq");
            if let CoreExprChirho::LamChirho {
                binder_chirho: inner_chirho,
                body_chirho: innermost_chirho,
            } = body_chirho.as_ref()
            {
                assert_eq!(inner_chirho.name_chirho, "$dOrd");
                assert_eq!(
                    *innermost_chirho.as_ref(),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
                );
            } else {
                panic!("expected nested lambda");
            }
        } else {
            panic!("expected outer lambda");
        }
    }

    #[test]
    fn build_layouts_from_class_env_chirho() {
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();

        let mut ctx_chirho =
            DictPassCtxChirho::new_chirho(0, HashMap::new(), HashMap::new());
        ctx_chirho.build_layouts_chirho(&class_env_chirho);

        // Eq: no supers, 1 method (==)
        let eq_layout_chirho = ctx_chirho.layouts_chirho.get("Eq").unwrap();
        assert_eq!(eq_layout_chirho.super_slots_chirho.len(), 0);
        assert_eq!(eq_layout_chirho.method_slots_chirho.len(), 1);
        assert_eq!(eq_layout_chirho.method_slots_chirho[0].0, "==");

        // Ord: 1 super (Eq), 1 method (compare)
        let ord_layout_chirho = ctx_chirho.layouts_chirho.get("Ord").unwrap();
        assert_eq!(ord_layout_chirho.super_slots_chirho.len(), 1);
        assert_eq!(ord_layout_chirho.super_slots_chirho[0].0, "Eq");
        assert_eq!(ord_layout_chirho.method_slots_chirho.len(), 1);
        assert_eq!(ord_layout_chirho.method_slots_chirho[0].0, "compare");
        assert_eq!(ord_layout_chirho.field_count_chirho, 2);

        // Num: 2 supers (Eq, Show), 7 methods (+, *, -, negate, fromInteger, abs, signum)
        let num_layout_chirho = ctx_chirho.layouts_chirho.get("Num").unwrap();
        assert_eq!(num_layout_chirho.super_slots_chirho.len(), 2);
        assert_eq!(num_layout_chirho.method_slots_chirho.len(), 7);
        assert_eq!(num_layout_chirho.field_count_chirho, 9);
    }

    #[test]
    fn generate_selectors_creates_bindings_chirho() {
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();

        let mut ctx_chirho =
            DictPassCtxChirho::new_chirho(0, HashMap::new(), HashMap::new());
        ctx_chirho.build_layouts_chirho(&class_env_chirho);
        ctx_chirho.generate_selectors_chirho();

        // Should have selectors for each method across all classes
        assert!(!ctx_chirho.generated_bindings_chirho.is_empty());

        // Check that == selector exists
        assert!(ctx_chirho.method_selectors_chirho.contains_key("=="));

        // Check that the selector is a lambda wrapping a case
        let (class_name_chirho, _sel_id_chirho) =
            ctx_chirho.method_selectors_chirho.get("==").unwrap();
        assert_eq!(class_name_chirho, "Eq");

        // Find the selector binding
        let sel_binding_chirho = ctx_chirho
            .generated_bindings_chirho
            .iter()
            .find(|b_chirho| b_chirho.binder_chirho.name_chirho == "$sel_Eq_==")
            .expect("selector binding should exist");

        assert!(matches!(
            sel_binding_chirho.rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));
    }

    #[test]
    fn max_id_finder_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 5),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("x", 10),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(
                        42,
                    ))),
                },
                is_rec_chirho: false,
            }],
            names_chirho: HashMap::new(),
        };
        assert_eq!(find_max_id_chirho(&module_chirho), 42);
    }

    #[test]
    fn instance_dicts_generated_for_ground_instances_chirho() {
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();

        let mut ctx_chirho =
            DictPassCtxChirho::new_chirho(0, HashMap::new(), HashMap::new());
        ctx_chirho.build_layouts_chirho(&class_env_chirho);
        ctx_chirho.generate_selectors_chirho();
        ctx_chirho.generate_instance_dicts_chirho(&class_env_chirho);

        // Should have generated instance dicts for ground instances
        // (Eq Int, Eq Char, Eq Bool, Show Int, Show Char, Show Bool,
        //  Ord Int, Ord Char, Num Int)
        assert!(!ctx_chirho.instance_dicts_chirho.is_empty());

        // Check Eq Int dict exists
        assert!(ctx_chirho
            .instance_dicts_chirho
            .contains_key(&("Eq".to_string(), "Int".to_string())));

        // Check Num Int dict exists
        assert!(ctx_chirho
            .instance_dicts_chirho
            .contains_key(&("Num".to_string(), "Int".to_string())));

        // Find the $fEqInt binding
        let eq_int_binding_chirho = ctx_chirho
            .generated_bindings_chirho
            .iter()
            .find(|b_chirho| b_chirho.binder_chirho.name_chirho == "$fEqInt")
            .expect("$fEqInt binding should exist");

        // Should be a constructor application (ConAppChirho)
        assert!(matches!(
            eq_int_binding_chirho.rhs_chirho,
            CoreExprChirho::ConAppChirho { .. }
        ));
    }

    #[test]
    fn method_ref_rewritten_to_selector_app_chirho() {
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();

        // Set up names map: id 5 is "+"
        let mut names_chirho = HashMap::new();
        names_chirho.insert(CoreIdChirho(5), "+".to_string());

        let mut ctx_chirho =
            DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
        ctx_chirho.build_layouts_chirho(&class_env_chirho);
        ctx_chirho.generate_selectors_chirho();

        // Build dict_vars: Num class has a dict at id 99
        let mut dict_vars_chirho = HashMap::new();
        dict_vars_chirho.insert("Num".to_string(), CoreIdChirho(99));

        // Rewrite a reference to "+" (id 5)
        let expr_chirho = CoreExprChirho::VarChirho(CoreIdChirho(5));
        let result_chirho =
            ctx_chirho.rewrite_method_refs_chirho(&expr_chirho, &dict_vars_chirho);

        // Should be: ($sel_Num_+ $dNum) i.e. App(Var(sel_id), Var(99))
        if let CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } = &result_chirho
        {
            // The function should be the selector
            assert!(matches!(**fun_chirho, CoreExprChirho::VarChirho(_)));
            // The arg should be the dict variable
            assert_eq!(**arg_chirho, CoreExprChirho::VarChirho(CoreIdChirho(99)));
        } else {
            panic!("expected method ref to be rewritten to App");
        }
    }

    #[test]
    fn non_method_var_unchanged_in_rewrite_chirho() {
        let mut names_chirho = HashMap::new();
        names_chirho.insert(CoreIdChirho(5), "x".to_string());

        let ctx_chirho =
            DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());

        let dict_vars_chirho = HashMap::new();
        let expr_chirho = CoreExprChirho::VarChirho(CoreIdChirho(5));
        let result_chirho =
            ctx_chirho.rewrite_method_refs_chirho(&expr_chirho, &dict_vars_chirho);

        // "x" is not an overloaded method, should be unchanged
        assert_eq!(result_chirho, CoreExprChirho::VarChirho(CoreIdChirho(5)));
    }

    #[test]
    fn rewrite_inside_app_chirho() {
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();

        let mut names_chirho = HashMap::new();
        names_chirho.insert(CoreIdChirho(5), "+".to_string());
        names_chirho.insert(CoreIdChirho(6), "x".to_string());
        names_chirho.insert(CoreIdChirho(7), "y".to_string());

        let mut ctx_chirho =
            DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
        ctx_chirho.build_layouts_chirho(&class_env_chirho);
        ctx_chirho.generate_selectors_chirho();

        let mut dict_vars_chirho = HashMap::new();
        dict_vars_chirho.insert("Num".to_string(), CoreIdChirho(99));

        // Expression: (+) x y → App(App(+, x), y)
        let expr_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(5))),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(6))),
            }),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(7))),
        };

        let result_chirho =
            ctx_chirho.rewrite_method_refs_chirho(&expr_chirho, &dict_vars_chirho);

        // The "+" reference should be rewritten, "x" and "y" should not
        if let CoreExprChirho::AppChirho { fun_chirho, .. } = &result_chirho {
            if let CoreExprChirho::AppChirho {
                fun_chirho: inner_fun_chirho,
                arg_chirho: inner_arg_chirho,
            } = fun_chirho.as_ref()
            {
                // inner_fun should be ($sel_Num_+ $dNum), which is an App
                assert!(matches!(
                    **inner_fun_chirho,
                    CoreExprChirho::AppChirho { .. }
                ));
                // inner_arg should be unchanged x
                assert_eq!(
                    **inner_arg_chirho,
                    CoreExprChirho::VarChirho(CoreIdChirho(6))
                );
            } else {
                panic!("expected nested App");
            }
        } else {
            panic!("expected outer App");
        }
    }

    #[test]
    fn transform_module_adds_selectors_and_wraps_chirho() {
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();

        let mut type_env_chirho = TyEnvChirho::new_chirho();
        // Register "add" as Num a => a -> a -> a
        type_env_chirho.bind_chirho(
            "add".to_string(),
            SchemeChirho {
                vars_chirho: vec![rhasky_typing_chirho::ty_chirho::TyVarChirho(0)],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Num".to_string(),
                    ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(0),
                    ),
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    [
                        TyChirho::VarChirho(
                            rhasky_typing_chirho::ty_chirho::TyVarChirho(0),
                        ),
                        TyChirho::VarChirho(
                            rhasky_typing_chirho::ty_chirho::TyVarChirho(0),
                        ),
                    ],
                    TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(0),
                    ),
                ),
            },
        );

        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("add", 0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                is_rec_chirho: false,
            }],
            names_chirho: HashMap::new(),
        };

        let result_chirho = dict_pass_module_chirho(
            &module_chirho,
            HashMap::new(),
            &type_env_chirho,
            &class_env_chirho,
        );

        // Should have generated selector bindings + the transformed "add" binding
        assert!(result_chirho.module_chirho.bindings_chirho.len() > 1);

        // Find "add" binding — it should be wrapped in a dict lambda
        let add_binding_chirho = result_chirho
            .module_chirho
            .bindings_chirho
            .iter()
            .find(|b_chirho| b_chirho.binder_chirho.name_chirho == "add")
            .expect("add binding should exist");

        assert!(matches!(
            add_binding_chirho.rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));

        // Layouts should be populated
        assert!(result_chirho.layouts_chirho.contains_key("Eq"));
        assert!(result_chirho.layouts_chirho.contains_key("Num"));
    }
}
