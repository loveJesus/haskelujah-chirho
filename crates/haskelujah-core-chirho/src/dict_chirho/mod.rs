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


pub mod layout_chirho;
pub mod instance_chirho;
pub mod prelude_chirho;
pub mod rewrite_chirho;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::env_chirho::TyEnvChirho;
use haskelujah_typing_chirho::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreModuleChirho,
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
        pred_chirho: &haskelujah_typing_chirho::ty_chirho::SchemePredChirho,
        scheme_chirho: &SchemeChirho,
    ) -> bool {
        let tv_chirho = match &pred_chirho.ty_chirho {
            TyChirho::VarChirho(v_chirho) => *v_chirho,
            _ => return false, // ground type, not defaultable
        };

        // Collect argument types from the function chain a -> b -> c -> r
        fn collect_arg_tys_chirho(ty_chirho: &TyChirho) -> Vec<&TyChirho> {
            match ty_chirho {
                TyChirho::FunChirho(arg_chirho, res_chirho, _) => {
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
                TyChirho::FunChirho(a_chirho, b_chirho, _) => {
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
                        name_chirho if name_chirho == "(,)" || name_chirho.starts_with("$tuple") || name_chirho == "(,,)" || name_chirho == "(,,,)" => {
                            if !args_chirho.is_empty() {
                                let keys_chirho: Vec<String> = args_chirho.iter().map(|a_chirho| {
                                    let k_chirho = self.infer_type_key_chirho(a_chirho).unwrap_or("Int".to_string());
                                    if k_chirho == "[Char]" { "String".to_string() } else { k_chirho }
                                }).collect();
                                return Some(format!("({})", keys_chirho.join(",")));
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
                        // Well-known Prelude function return types:
                        // These functions have fixed return types regardless
                        // of their argument types, so we can infer the result
                        // type for dict selection without full type inference.
                        match name_chirho.as_str() {
                            // :: [a] -> Int
                            "length" | "sum" | "product" => {
                                return Some("Int".to_string());
                            }
                            // :: [a] -> [a]  (element type from arg)
                            "take" | "drop" | "reverse" | "sort"
                            | "init" | "tail" | "nub" | "cycle" => {
                                if let Some(arg_ty_chirho) =
                                    self.infer_type_key_chirho(arg_chirho)
                                {
                                    return Some(arg_ty_chirho);
                                }
                                return None;
                            }
                            // :: [a] -> a  (element type from list arg)
                            "head" | "last" | "minimum" | "maximum" => {
                                if let Some(arg_ty_chirho) =
                                    self.infer_type_key_chirho(arg_chirho)
                                {
                                    // Strip outer list: [Int] → Int
                                    if arg_ty_chirho.starts_with('[')
                                        && arg_ty_chirho.ends_with(']')
                                    {
                                        return Some(
                                            arg_ty_chirho[1..arg_ty_chirho.len() - 1]
                                                .to_string(),
                                        );
                                    }
                                    return Some(arg_ty_chirho);
                                }
                                return None;
                            }
                            // :: a -> Bool
                            "null" | "even" | "odd" | "elem" | "notElem" => {
                                return Some("Bool".to_string());
                            }
                            // :: a -> Int
                            "fromEnum" | "ord" => {
                                return Some("Int".to_string());
                            }
                            // :: Int -> a (for toEnum/chr, we don't know the
                            // result type without context, so skip)
                            _ => {}
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
                            // Two-arg Prelude functions:
                            // take/drop :: Int -> [a] -> [a]
                            // zip :: [a] -> [b] -> [(a,b)]
                            // map :: (a -> b) -> [a] -> [b]
                            // filter :: (a -> Bool) -> [a] -> [a]
                            match name_chirho.as_str() {
                                "take" | "drop" | "filter" | "takeWhile"
                                | "dropWhile" => {
                                    // Result type = list arg type
                                    if let Some(ty_chirho) =
                                        self.infer_type_key_chirho(arg_chirho)
                                    {
                                        return Some(ty_chirho);
                                    }
                                }
                                "zip" | "zipWith" | "map" | "concatMap" => {
                                    // Complex return types; skip for now
                                }
                                "elem" | "notElem" | "any" | "all" => {
                                    return Some("Bool".to_string());
                                }
                                _ => {}
                            }
                        }
                    }
                }
                // If the function is a PrimOp with a known return type (e.g. readInt#
                // returns Int, not [Char]), prefer the function's type over the argument's.
                // This prevents `show (readInt# "10")` from dispatching as ShowStr.
                let fun_ty_chirho = self.infer_type_key_chirho(fun_chirho);
                if let Some(ref fty_chirho) = fun_ty_chirho {
                    // readInt# / readFloat# / readBool# have unambiguous return types —
                    // always use them.  Other primops (like ++#) return [Char] which
                    // already matched in the PrimOpChirho arm above, so we won't reach
                    // here for those.  General function types are inferred below.
                    let is_typed_primop_chirho = matches!(
                        fun_chirho.as_ref(),
                        CoreExprChirho::PrimOpChirho { .. }
                    ) && !fty_chirho.is_empty();
                    if is_typed_primop_chirho {
                        return Some(fty_chirho.clone());
                    }
                }
                // For general App chains like `f 1.5 2.5`, try to infer
                // from the argument first, but ONLY when the outermost
                // function is a literal, primop, or non-list-typed expression.
                // Propagating the arg's list type (e.g. [Int]) for a user
                // function like `myLen []` would be wrong: myLen returns Int
                // but its argument is [Int].  Guard against this: only
                // propagate a list-type inference if the function itself
                // also returns a list type.
                if let Some(tk_chirho) = self.infer_type_key_chirho(arg_chirho) {
                    let is_list_type_chirho = tk_chirho.starts_with('[');
                    // Only propagate list arg types if the function also
                    // returns a list (fun_ty is Some list) or is unknown.
                    // For user-defined functions (fun_ty = None), list types
                    // should NOT be propagated as the result type.
                    if is_list_type_chirho && fun_ty_chirho.is_none() {
                        // Don't propagate: user function likely returns a
                        // scalar, not a list.
                        return None;
                    }
                    return Some(tk_chirho);
                }
                fun_ty_chirho
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
    use crate::expr_chirho::{CoreLitChirho, InlineAnnotationChirho};
    use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
    use haskelujah_typing_chirho::ty_chirho::SchemePredChirho;

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
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
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
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Num".to_string(),
                ty_chirho: TyChirho::VarChirho(
                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(0),
                ),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
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
            TyChirho::FunChirho(_, _, _)
        ));
    }

    #[test]
    fn multiple_predicates_add_nested_dict_lambdas_chirho() {
        let binding_chirho = CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("cmp", 0),
            rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
            is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)],
            preds_chirho: vec![
                SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(0),
                    ),
                    extra_tys_chirho: vec![],
                },
                SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(0),
                    ),
                    extra_tys_chirho: vec![],
                },
            ],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
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
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
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
                vars_chirho: vec![haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Num".to_string(),
                    ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(0),
                    ),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    [
                        TyChirho::VarChirho(
                            haskelujah_typing_chirho::ty_chirho::TyVarChirho(0),
                        ),
                        TyChirho::VarChirho(
                            haskelujah_typing_chirho::ty_chirho::TyVarChirho(0),
                        ),
                    ],
                    TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(0),
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
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
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
