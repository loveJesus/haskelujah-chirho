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
                        "Nothing" => Some("Maybe Int".to_string()),
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
                    "showInt#" | "showFloat#" | "showStr#" | "showList#"
                    | "showMaybe#" | "showTuple2#"
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
                            _ => {}
                        }
                        // Check if the name is a known constructor and return its type
                        if let Some(type_name_chirho) = self.con_types_chirho.get(name_chirho) {
                            return Some(type_name_chirho.clone());
                        }
                    }
                }
                // Detect App(App(Var((,)), a), b) → tuple type
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
            // Show
            ("Show", "show", "Int", 2),
            ("Show", "show", "Char", 0),
            // Show Bool is handled specially below (case True/False -> string)
            // ("Show", "show", "Bool", 0),
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
        ];

        let primop_for_chirho =
            |class_chirho: &str, method_chirho: &str, type_key_chirho: &str| -> &str {
                match (class_chirho, method_chirho, type_key_chirho) {
                    ("Eq", "==", "[Char]") => "eqStr#",
                    ("Eq", "==", "Double") => "eqFloat#",
                    ("Eq", "==", _) => "==#",
                    ("Ord", "compare", "Char") => "compareChar#",
                    ("Ord", "compare", "Double") => "compareFloat#",
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
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let acc_chirho = self.fresh_binder_chirho("acc", list_a_chirho.clone());

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
            ] {
                let key_chirho = ("Show".to_string(), type_key_chirho.to_string());
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
