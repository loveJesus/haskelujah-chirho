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

pub mod instance_chirho;
pub mod layout_chirho;
mod monad_fix_chirho;
pub mod prelude_chirho;
#[cfg(test)]
mod print_evidence_tests_chirho;
pub mod rewrite_chirho;
mod show_chirho;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::env_chirho::TyEnvChirho;
use haskelujah_typing_chirho::ty_chirho::{SchemeChirho, TyChirho};

use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreModuleChirho,
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
    /// Names that are backed by a real top-level RHS body, not just an entry in
    /// the name map. This prevents missing class methods from silently resolving
    /// to dangling `$prim_*` IDs that later evaluate as placeholder values.
    body_backed_names_chirho: HashMap<String, CoreIdChirho>,
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
    /// Imported names that should be treated as constrained user bindings.
    /// This is intentionally an allowlist: seeding every name in the type env
    /// incorrectly treats Prelude class methods such as `show` as ordinary
    /// dictionary-parameterized functions.
    extra_dict_param_names_chirho: HashSet<String>,
    /// Evidence-threading P1: occurrence id → (method name, canonical shared id)
    /// from `DesugarOutputChirho::method_occurrences_chirho`. Empty by default.
    method_occurrence_canon_chirho: HashMap<CoreIdChirho, (String, CoreIdChirho)>,
    /// Evidence-threading P1: occurrence id → (class name, instance type key).
    /// Populated by later phases (typing bridge / defaulting); when present the
    /// occurrence dispatches directly to `$prim_{class}_{method}_{key}`.
    occurrence_evidence_chirho: HashMap<CoreIdChirho, (String, String)>,
    /// Reference evidence: occurrence id of a constrained reference → one
    /// (class, key) per predicate in scheme order; a `None` key is "nothing
    /// proved for this position". See `evidence_dict_for_class_chirho`.
    reference_evidence_chirho: HashMap<CoreIdChirho, Vec<(String, Option<String>)>>,
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
    /// Stack of monad head keys (e.g. "Maybe") for chains currently being
    /// positively dispatched (`>>=`/`>>`); `return`/`pure` inside those
    /// continuations dispatch to `$prim_Applicative_pure_<key>`. Without a
    /// context they keep their name and fall back to ReturnIOChirho (INV-001).
    /// workflow: monadic-dispatch-chirho
    monad_context_stack_chirho: RefCell<Vec<String>>,
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
            body_backed_names_chirho: HashMap::new(),
            method_selectors_chirho: HashMap::new(),
            instance_dicts_chirho: HashMap::new(),
            con_types_chirho,
            dict_param_bindings_chirho: HashMap::new(),
            extra_dict_param_names_chirho: HashSet::new(),
            method_occurrence_canon_chirho: HashMap::new(),
            occurrence_evidence_chirho: HashMap::new(),
            reference_evidence_chirho: HashMap::new(),
            super_selectors_chirho: HashMap::new(),
            conditional_dicts_chirho: HashMap::new(),
            newtype_info_chirho: HashMap::new(),
            class_param_count_chirho: HashMap::new(),
            local_shadow_ids_chirho: RefCell::new(HashSet::new()),
            monad_context_stack_chirho: RefCell::new(Vec::new()),
        }
    }

    /// Generate a fresh CoreId and record its name.
    fn fresh_id_chirho(&mut self, name_chirho: &str) -> CoreIdChirho {
        let id_chirho = CoreIdChirho(self.next_id_chirho);
        self.next_id_chirho += 1;
        self.names_chirho.insert(id_chirho, name_chirho.to_string());
        id_chirho
    }

    /// Look up an existing CoreId by name, or create a fresh one.
    /// This is used when generating references to bindings that may
    /// already exist (e.g. `$prim_` bindings from desugaring).
    /// Evidence-threading P2b: per-occurrence ids share the method's NAME but
    /// must never be chosen as the binding id for generated prelude bodies —
    /// only the canonical shared id may anchor a generated binding.
    fn resolve_or_fresh_id_chirho(&mut self, name_chirho: &str) -> CoreIdChirho {
        // Search for an existing ID with this name (skipping occurrence ids)
        for (id_chirho, existing_name_chirho) in &self.names_chirho {
            if existing_name_chirho == name_chirho
                && !self.method_occurrence_canon_chirho.contains_key(id_chirho)
            {
                return *id_chirho;
            }
        }
        // Not found — create a fresh one
        self.fresh_id_chirho(name_chirho)
    }

    /// Record top-level source bindings before generated dictionaries are built.
    fn seed_body_backed_bindings_chirho(&mut self, module_chirho: &CoreModuleChirho) {
        for binding_chirho in &module_chirho.bindings_chirho {
            self.body_backed_names_chirho
                .entry(binding_chirho.binder_chirho.name_chirho.clone())
                .or_insert(binding_chirho.binder_chirho.id_chirho);
        }
    }

    /// Look up a binding only if this pass can see an actual RHS body for it.
    ///
    /// `names_chirho` is a global name table and can contain fresh placeholder
    /// IDs. Instance dictionaries must not treat those placeholders as method
    /// implementations.
    fn lookup_body_backed_name_id_chirho(&self, name_chirho: &str) -> Option<CoreIdChirho> {
        if let Some(id_chirho) = self.body_backed_names_chirho.get(name_chirho) {
            return Some(*id_chirho);
        }

        self.generated_bindings_chirho
            .iter()
            .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == name_chirho)
            .map(|binding_chirho| binding_chirho.binder_chirho.id_chirho)
    }

    fn lookup_dispatch_body_name_id_chirho(&self, name_chirho: &str) -> Option<CoreIdChirho> {
        let id_chirho = self.lookup_body_backed_name_id_chirho(name_chirho)?;
        if let Some(binding_chirho) = self
            .generated_bindings_chirho
            .iter()
            .find(|binding_chirho| binding_chirho.binder_chirho.id_chirho == id_chirho)
        {
            if Self::is_generated_missing_method_body_chirho(&binding_chirho.rhs_chirho) {
                return None;
            }
        }
        Some(id_chirho)
    }

    fn is_generated_missing_method_body_chirho(expr_chirho: &CoreExprChirho) -> bool {
        match expr_chirho {
            CoreExprChirho::LamChirho { body_chirho, .. }
            | CoreExprChirho::TyLamChirho { body_chirho, .. }
            | CoreExprChirho::TyAppChirho {
                expr_chirho: body_chirho,
                ..
            } => Self::is_generated_missing_method_body_chirho(body_chirho),
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } if name_chirho == "error" => args_chirho.iter().any(|arg_chirho| {
                matches!(
                    arg_chirho,
                    CoreExprChirho::LitChirho(crate::expr_chirho::CoreLitChirho::StringChirho(
                        msg_chirho
                    )) if msg_chirho.contains("missing method ")
                )
            }),
            _ => false,
        }
    }

    /// Preserve the pass's result-only defaulting policy, but inspect complete
    /// argument types. A variable inside a tuple, list or higher-order argument
    /// is still determined by its caller and must retain its dictionary.
    /// Result-only polymorphism needs checker-directed defaulting; changing it
    /// here alone leaves evaluated bindings with unsupplied dictionary arguments.
    fn is_defaultable_pred_chirho(
        pred_chirho: &haskelujah_typing_chirho::ty_chirho::SchemePredChirho,
        scheme_chirho: &SchemeChirho,
    ) -> bool {
        let TyChirho::VarChirho(var_chirho) = &pred_chirho.ty_chirho else {
            return false;
        };
        let mut ty_chirho = &scheme_chirho.ty_chirho;
        while let TyChirho::FunChirho(argument_chirho, result_chirho, _) = ty_chirho {
            if argument_chirho.free_vars_chirho().contains(var_chirho) {
                return false;
            }
            ty_chirho = result_chirho;
        }
        true
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
                        "()" | "$tuple0" => Some("()".to_string()),
                        "Identity" => Some("Identity".to_string()),
                        "Proxy" => Some("Proxy".to_string()),
                        "Const" => Some("Const".to_string()),
                        "Sum" => Some("Sum".to_string()),
                        "Product" => Some("Product".to_string()),
                        "All" => Some("All".to_string()),
                        "Any" => Some("Any".to_string()),
                        "Min" => Some("Min".to_string()),
                        "Max" => Some("Max".to_string()),
                        "First" => Some("First".to_string()),
                        "Last" => Some("Last".to_string()),
                        "Down" => Some("Down".to_string()),
                        "Endo" => Some("Endo".to_string()),
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
                        name_chirho
                            if name_chirho == "(,)"
                                || name_chirho.starts_with("$tuple")
                                || name_chirho == "(,,)"
                                || name_chirho == "(,,,)" =>
                        {
                            if !args_chirho.is_empty() {
                                let keys_chirho: Vec<String> = args_chirho
                                    .iter()
                                    .map(|a_chirho| {
                                        let k_chirho = self
                                            .infer_type_key_chirho(a_chirho)
                                            .unwrap_or("Int".to_string());
                                        if k_chirho == "[Char]" {
                                            "String".to_string()
                                        } else {
                                            k_chirho
                                        }
                                    })
                                    .collect();
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
                                    Some("()") => Some("[()]".to_string()),
                                    Some("Double") => Some("[Double]".to_string()),
                                    Some("Bool") => Some("[Bool]".to_string()),
                                    Some("Ordering") => Some("[Ordering]".to_string()),
                                    Some("Sum") => Some("[Sum]".to_string()),
                                    Some("Product") => Some("[Product]".to_string()),
                                    Some("All") => Some("[All]".to_string()),
                                    Some("Any") => Some("[Any]".to_string()),
                                    Some("Min") => Some("[Min]".to_string()),
                                    Some("Max") => Some("[Max]".to_string()),
                                    Some("First") => Some("[First]".to_string()),
                                    Some("Last") => Some("[Last]".to_string()),
                                    Some("Down") => Some("[Down]".to_string()),
                                    Some("Endo") => Some("[Endo]".to_string()),
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
                        "()" | "$tuple0" => return Some("()".to_string()),
                        "Identity" => return Some("Identity".to_string()),
                        "Proxy" => return Some("Proxy".to_string()),
                        "Const" => return Some("Const".to_string()),
                        "Sum" => return Some("Sum".to_string()),
                        "Product" => return Some("Product".to_string()),
                        "All" => return Some("All".to_string()),
                        "Any" => return Some("Any".to_string()),
                        "Min" => return Some("Min".to_string()),
                        "Max" => return Some("Max".to_string()),
                        "First" => return Some("First".to_string()),
                        "Last" => return Some("Last".to_string()),
                        "Down" => return Some("Down".to_string()),
                        "Endo" => return Some("Endo".to_string()),
                        _ => {}
                    }
                }
                None
            }
            CoreExprChirho::PrimOpChirho { name_chirho, .. } => {
                // Infer result type of known primops
                match name_chirho.as_str() {
                    "enumFromTo#" => Some("[Int]".to_string()),
                    "+#" | "-#" | "*#" | "div#" | "mod#" | "negate#" | "readInt#" => {
                        Some("Int".to_string())
                    }
                    "chr#" => Some("Char".to_string()),
                    "ord#" => Some("Int".to_string()),
                    "+.#" | "-.#" | "*.#" | "/.#" | "negateFloat#" | "recip#" | "readFloat#" => {
                        Some("Double".to_string())
                    }
                    "==#" | "/=#" | "<#" | "<=#" | ">#" | ">=#" | "not#" | "eqFloat#"
                    | "readBool#" => Some("Bool".to_string()),
                    "compare#" | "compareChar#" | "compareFloat#" | "compareStr#" => {
                        Some("Ordering".to_string())
                    }
                    "showInt#" | "showFloat#" | "showStr#" | "showList#" | "showMaybe#"
                    | "showTuple2#" | "showEither#" | "showOrdering#" | "showBool#" | "++#" => {
                        Some("[Char]".to_string())
                    }
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
                            "take" | "drop" | "reverse" | "sort" | "init" | "tail" | "nub"
                            | "cycle" => {
                                if let Some(arg_ty_chirho) = self.infer_type_key_chirho(arg_chirho)
                                {
                                    return Some(arg_ty_chirho);
                                }
                                return None;
                            }
                            // :: [a] -> a  (element type from list arg)
                            "head" | "last" | "minimum" | "maximum" => {
                                if let Some(arg_ty_chirho) = self.infer_type_key_chirho(arg_chirho)
                                {
                                    // Strip outer list: [Int] → Int
                                    if arg_ty_chirho.starts_with('[')
                                        && arg_ty_chirho.ends_with(']')
                                    {
                                        return Some(
                                            arg_ty_chirho[1..arg_ty_chirho.len() - 1].to_string(),
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
                            // :: Int -> Char
                            "chr" => {
                                return Some("Char".to_string());
                            }
                            // :: Int -> a (for toEnum, we don't know the
                            // result type without context, so skip)
                            _ => {}
                        }
                        // Constructor applications: App(Just, x) → Maybe <x-type>
                        match name_chirho.as_str() {
                            "Identity" | "Const" | "Sum" | "Product" | "All" | "Any" | "Min"
                            | "Max" | "First" | "Last" | "Down" | "Endo" => {
                                return Some(name_chirho.clone());
                            }
                            "Just" => {
                                let inner_chirho = self
                                    .infer_type_key_chirho(arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                let inner_key_chirho = if inner_chirho == "[Char]" {
                                    "String"
                                } else {
                                    &inner_chirho
                                };
                                return Some(format!("Maybe {}", inner_key_chirho));
                            }
                            "Left" => {
                                let inner_chirho = self
                                    .infer_type_key_chirho(arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                return Some(format!("Either {} Int", inner_chirho));
                            }
                            "Right" => {
                                let inner_chirho = self
                                    .infer_type_key_chirho(arg_chirho)
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
                                let a_chirho = self
                                    .infer_type_key_chirho(first_arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                let b_chirho = self
                                    .infer_type_key_chirho(arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                let a_key_chirho = if a_chirho == "[Char]" {
                                    "String"
                                } else {
                                    &a_chirho
                                };
                                let b_key_chirho = if b_chirho == "[Char]" {
                                    "String"
                                } else {
                                    &b_chirho
                                };
                                return Some(format!("({},{})", a_key_chirho, b_key_chirho));
                            }
                            // compare :: a -> a -> Ordering
                            if name_chirho == "compare"
                                || name_chirho.starts_with("$prim_Ord_compare")
                            {
                                return Some("Ordering".to_string());
                            }
                            // Two-arg Prelude functions:
                            // take/drop :: Int -> [a] -> [a]
                            // zip :: [a] -> [b] -> [(a,b)]
                            // map :: (a -> b) -> [a] -> [b]
                            // filter :: (a -> Bool) -> [a] -> [a]
                            match name_chirho.as_str() {
                                "take" | "drop" | "filter" | "takeWhile" | "dropWhile" => {
                                    // Result type = list arg type
                                    if let Some(ty_chirho) = self.infer_type_key_chirho(arg_chirho)
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
                    let is_typed_primop_chirho =
                        matches!(fun_chirho.as_ref(), CoreExprChirho::PrimOpChirho { .. })
                            && !fty_chirho.is_empty();
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
    fn fresh_binder_chirho(&mut self, name_chirho: &str, ty_chirho: TyChirho) -> BinderChirho {
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
    dict_pass_module_full_with_extra_dict_param_names_chirho(
        module_chirho,
        names_chirho,
        type_env_chirho,
        class_env_chirho,
        con_types_chirho,
        newtype_info_chirho,
        HashSet::new(),
    )
}

pub fn dict_pass_module_full_with_extra_dict_param_names_chirho(
    module_chirho: &CoreModuleChirho,
    names_chirho: HashMap<CoreIdChirho, String>,
    type_env_chirho: &TyEnvChirho,
    class_env_chirho: &ClassEnvChirho,
    con_types_chirho: HashMap<String, String>,
    newtype_info_chirho: HashMap<String, (String, String)>,
    extra_dict_param_names_chirho: HashSet<String>,
) -> DictPassResultChirho {
    dict_pass_module_full_with_method_occurrences_chirho(
        module_chirho,
        names_chirho,
        type_env_chirho,
        class_env_chirho,
        con_types_chirho,
        newtype_info_chirho,
        extra_dict_param_names_chirho,
        HashMap::new(),
        HashMap::new(),
    )
}

/// Evidence-threading P1 entry: like the `extra_dict_param_names` variant but
/// also threads the desugar occurrence table and (optional) occurrence
/// evidence. With both maps empty this is behavior-identical to the older
/// entries. workflow: monadic-dispatch-chirho (evidence-threading P1)
#[allow(clippy::too_many_arguments)]
pub fn dict_pass_module_full_with_method_occurrences_chirho(
    module_chirho: &CoreModuleChirho,
    names_chirho: HashMap<CoreIdChirho, String>,
    type_env_chirho: &TyEnvChirho,
    class_env_chirho: &ClassEnvChirho,
    con_types_chirho: HashMap<String, String>,
    newtype_info_chirho: HashMap<String, (String, String)>,
    extra_dict_param_names_chirho: HashSet<String>,
    method_occurrence_canon_chirho: HashMap<CoreIdChirho, (String, CoreIdChirho)>,
    occurrence_evidence_chirho: HashMap<CoreIdChirho, (String, String)>,
) -> DictPassResultChirho {
    let max_id_chirho = find_max_id_chirho(module_chirho);
    let mut ctx_chirho =
        DictPassCtxChirho::new_chirho(max_id_chirho + 1, names_chirho, con_types_chirho);
    ctx_chirho.newtype_info_chirho = newtype_info_chirho;
    ctx_chirho.extra_dict_param_names_chirho = extra_dict_param_names_chirho;
    ctx_chirho.method_occurrence_canon_chirho = method_occurrence_canon_chirho;
    ctx_chirho.occurrence_evidence_chirho = occurrence_evidence_chirho;
    let transformed_chirho =
        ctx_chirho.transform_module_chirho(module_chirho, type_env_chirho, class_env_chirho);
    ctx_chirho.finish_chirho(transformed_chirho)
}

/// Dictionary-evidence entry: like the method-occurrence entry, also threading
/// the checker's per-reference evidence for constrained references.
/// workflow: language-features-chirho/dictionary-evidence-chirho
#[allow(clippy::too_many_arguments)]
pub fn dict_pass_module_full_with_evidence_chirho(
    module_chirho: &CoreModuleChirho,
    names_chirho: HashMap<CoreIdChirho, String>,
    type_env_chirho: &TyEnvChirho,
    class_env_chirho: &ClassEnvChirho,
    con_types_chirho: HashMap<String, String>,
    newtype_info_chirho: HashMap<String, (String, String)>,
    extra_dict_param_names_chirho: HashSet<String>,
    method_occurrence_canon_chirho: HashMap<CoreIdChirho, (String, CoreIdChirho)>,
    occurrence_evidence_chirho: HashMap<CoreIdChirho, (String, String)>,
    reference_evidence_chirho: HashMap<CoreIdChirho, Vec<(String, Option<String>)>>,
) -> DictPassResultChirho {
    let max_id_chirho = find_max_id_chirho(module_chirho);
    let mut ctx_chirho =
        DictPassCtxChirho::new_chirho(max_id_chirho + 1, names_chirho, con_types_chirho);
    ctx_chirho.newtype_info_chirho = newtype_info_chirho;
    ctx_chirho.extra_dict_param_names_chirho = extra_dict_param_names_chirho;
    ctx_chirho.method_occurrence_canon_chirho = method_occurrence_canon_chirho;
    ctx_chirho.occurrence_evidence_chirho = occurrence_evidence_chirho;
    ctx_chirho.reference_evidence_chirho = reference_evidence_chirho;
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
    use haskelujah_typing_chirho::class_chirho::{ClassDeclChirho, ClassEnvChirho, InstDeclChirho};
    use haskelujah_typing_chirho::ty_chirho::{SchemePredChirho, TyVarChirho};

    fn dummy_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
        BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn num_var_scheme_chirho() -> SchemeChirho {
        let var_chirho = TyChirho::VarChirho(TyVarChirho(0));
        SchemeChirho {
            vars_chirho: vec![TyVarChirho(0)],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Num".to_string(),
                ty_chirho: var_chirho.clone(),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho([var_chirho.clone(), var_chirho.clone()], var_chirho),
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

        let mut ctx_chirho = DictPassCtxChirho::new_chirho(10, HashMap::new(), HashMap::new());
        let result_chirho = ctx_chirho.add_dict_params_chirho(&binding_chirho, &scheme_chirho);

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
                ty_chirho: TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
            ),
        };

        let mut ctx_chirho = DictPassCtxChirho::new_chirho(10, HashMap::new(), HashMap::new());
        let result_chirho = ctx_chirho.add_dict_params_chirho(&binding_chirho, &scheme_chirho);

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

        let mut ctx_chirho = DictPassCtxChirho::new_chirho(10, HashMap::new(), HashMap::new());
        let result_chirho = ctx_chirho.add_dict_params_chirho(&binding_chirho, &scheme_chirho);

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

        let mut ctx_chirho = DictPassCtxChirho::new_chirho(0, HashMap::new(), HashMap::new());
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

        let mut ctx_chirho = DictPassCtxChirho::new_chirho(0, HashMap::new(), HashMap::new());
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
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(42))),
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

        let mut ctx_chirho = DictPassCtxChirho::new_chirho(0, HashMap::new(), HashMap::new());
        ctx_chirho.build_layouts_chirho(&class_env_chirho);
        ctx_chirho.generate_selectors_chirho();
        ctx_chirho.generate_instance_dicts_chirho(&class_env_chirho);

        // Should have generated instance dicts for ground instances
        // (Eq Int, Eq Char, Eq Bool, Show Int, Show Char, Show Bool,
        //  Ord Int, Ord Char, Num Int)
        assert!(!ctx_chirho.instance_dicts_chirho.is_empty());

        // Check Eq Int dict exists
        assert!(
            ctx_chirho
                .instance_dicts_chirho
                .contains_key(&("Eq".to_string(), "Int".to_string()))
        );

        // Check Num Int dict exists
        assert!(
            ctx_chirho
                .instance_dicts_chirho
                .contains_key(&("Num".to_string(), "Int".to_string()))
        );

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
    fn missing_method_ignores_dangling_name_map_entry_chirho() {
        let mut methods_chirho = HashMap::new();
        methods_chirho.insert(
            "needed".to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                TyChirho::int_chirho(),
                TyChirho::int_chirho(),
            )),
        );

        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.add_class_chirho(ClassDeclChirho {
            name_chirho: "Needs".to_string(),
            supers_chirho: vec![],
            var_chirho: haskelujah_typing_chirho::ty_chirho::TyVarChirho(0),
            methods_chirho,
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        });
        class_env_chirho.add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Needs".to_string(),
            head_ty_chirho: TyChirho::int_chirho(),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });

        let mut names_chirho = HashMap::new();
        names_chirho.insert(CoreIdChirho(7), "$prim_Needs_needed_Int".to_string());
        let mut ctx_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
        ctx_chirho.build_layouts_chirho(&class_env_chirho);
        ctx_chirho.generate_instance_dicts_chirho(&class_env_chirho);

        let dict_binding_chirho = ctx_chirho
            .generated_bindings_chirho
            .iter()
            .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == "$fNeedsInt")
            .expect("Needs Int dictionary should be generated");
        let method_id_chirho = match &dict_binding_chirho.rhs_chirho {
            CoreExprChirho::ConAppChirho { args_chirho, .. } => match args_chirho.first() {
                Some(CoreExprChirho::VarChirho(id_chirho)) => *id_chirho,
                other_chirho => panic!("expected method var slot, got {other_chirho:?}"),
            },
            other_chirho => panic!("expected dictionary constructor, got {other_chirho:?}"),
        };

        assert_ne!(
            method_id_chirho,
            CoreIdChirho(7),
            "dangling name-map entry must not be used as a method body"
        );
        let missing_binding_chirho = ctx_chirho
            .generated_bindings_chirho
            .iter()
            .find(|binding_chirho| binding_chirho.binder_chirho.id_chirho == method_id_chirho)
            .expect("missing-method body should be generated");
        assert_eq!(
            missing_binding_chirho.binder_chirho.name_chirho,
            "$prim_Needs_needed_Int"
        );
        assert!(
            matches!(
                missing_binding_chirho.rhs_chirho,
                CoreExprChirho::LamChirho { .. }
            ),
            "missing method should be a callable loud-error body"
        );
    }

    #[test]
    fn method_ref_rewritten_to_selector_app_chirho() {
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();

        // Set up names map: id 5 is "+"
        let mut names_chirho = HashMap::new();
        names_chirho.insert(CoreIdChirho(5), "+".to_string());

        let mut ctx_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
        ctx_chirho.build_layouts_chirho(&class_env_chirho);
        ctx_chirho.generate_selectors_chirho();

        let binding_chirho = CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("add", 8),
            rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(5)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let result_chirho =
            ctx_chirho.add_dict_params_chirho(&binding_chirho, &num_var_scheme_chirho());

        // Should be: ($sel_Num_+ $dNum) where $dNum is the local evidence binder.
        if let CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } = &result_chirho.rhs_chirho
        {
            if let CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } = body_chirho.as_ref()
            {
                // The function should be the selector
                assert!(matches!(**fun_chirho, CoreExprChirho::VarChirho(_)));
                // The arg should be the locally-proven Num dictionary
                assert_eq!(
                    **arg_chirho,
                    CoreExprChirho::VarChirho(binder_chirho.id_chirho)
                );
            } else {
                panic!("expected method ref to be rewritten to selector application");
            }
        } else {
            panic!("expected Num dictionary lambda");
        }
    }

    #[test]
    fn method_ref_uses_transitive_superclass_dict_chirho() {
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();

        let mut names_chirho = HashMap::new();
        names_chirho.insert(CoreIdChirho(5), "==".to_string());

        let mut ctx_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
        ctx_chirho.build_layouts_chirho(&class_env_chirho);
        ctx_chirho.generate_selectors_chirho();

        let binding_chirho = CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("eq_from_integral", 8),
            rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(5)),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let integral_var_chirho = TyChirho::VarChirho(TyVarChirho(0));
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![TyVarChirho(0)],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Integral".to_string(),
                ty_chirho: integral_var_chirho.clone(),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(integral_var_chirho, TyChirho::bool_chirho()),
        };

        let result_chirho = ctx_chirho.add_dict_params_chirho(&binding_chirho, &scheme_chirho);

        let rhs_debug_chirho = format!("{:#?}", result_chirho.rhs_chirho);
        assert!(
            rhs_debug_chirho.contains("name_chirho: \"$dNum\""),
            "{rhs_debug_chirho}"
        );
        assert!(
            rhs_debug_chirho.contains("name_chirho: \"$dEq\""),
            "{rhs_debug_chirho}"
        );
    }

    #[test]
    fn non_method_var_unchanged_in_rewrite_chirho() {
        let mut names_chirho = HashMap::new();
        names_chirho.insert(CoreIdChirho(5), "x".to_string());

        let ctx_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());

        let dict_vars_chirho = HashMap::new();
        let expr_chirho = CoreExprChirho::VarChirho(CoreIdChirho(5));
        let result_chirho = ctx_chirho.rewrite_method_refs_chirho(&expr_chirho, &dict_vars_chirho);

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

        let mut ctx_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
        ctx_chirho.build_layouts_chirho(&class_env_chirho);
        ctx_chirho.generate_selectors_chirho();

        // Expression: (+) x y → App(App(+, x), y)
        let binding_chirho = CoreBindingChirho {
            binder_chirho: dummy_binder_chirho("add", 8),
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(5))),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(6))),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(7))),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        };
        let result_chirho =
            ctx_chirho.add_dict_params_chirho(&binding_chirho, &num_var_scheme_chirho());

        // The "+" reference should be rewritten, "x" and "y" should not
        if let CoreExprChirho::LamChirho { body_chirho, .. } = &result_chirho.rhs_chirho {
            if let CoreExprChirho::AppChirho { fun_chirho, .. } = body_chirho.as_ref() {
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
                panic!("expected rewritten method application");
            }
        } else {
            panic!("expected Num dictionary lambda");
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
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
                    ],
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
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

    /// Evidence-threading P1 helpers: a module whose `main` applies a method
    /// OCCURRENCE id (50) of `==` to two Int literals; canonical shared id 60;
    /// a body-backed `$prim_Eq_==_Int` row exists as id 100.
    fn occurrence_test_module_chirho() -> (
        CoreModuleChirho,
        HashMap<CoreIdChirho, String>,
        HashMap<CoreIdChirho, (String, CoreIdChirho)>,
    ) {
        let mut names_chirho = HashMap::new();
        names_chirho.insert(CoreIdChirho(0), "main".to_string());
        names_chirho.insert(CoreIdChirho(50), "==".to_string());
        names_chirho.insert(CoreIdChirho(60), "==".to_string());
        names_chirho.insert(CoreIdChirho(100), "$prim_Eq_==_Int".to_string());
        let main_rhs_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(50))),
                arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))),
            }),
            arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2))),
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "OccTest".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("$prim_Eq_==_Int", 100),
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(7)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("main", 0),
                    rhs_chirho: main_rhs_chirho,
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: names_chirho.clone(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: Vec::new(),
        };
        let mut canon_chirho = HashMap::new();
        canon_chirho.insert(CoreIdChirho(50), ("==".to_string(), CoreIdChirho(60)));
        (module_chirho, names_chirho, canon_chirho)
    }

    fn app_spine_head_id_chirho(expr_chirho: &CoreExprChirho) -> Option<CoreIdChirho> {
        let mut current_chirho = expr_chirho;
        while let CoreExprChirho::AppChirho { fun_chirho, .. } = current_chirho {
            current_chirho = fun_chirho.as_ref();
        }
        match current_chirho {
            CoreExprChirho::VarChirho(id_chirho) => Some(*id_chirho),
            _ => None,
        }
    }

    #[test]
    fn evidence_occurrence_dispatches_to_prim_row_chirho() {
        let (module_chirho, names_chirho, canon_chirho) = occurrence_test_module_chirho();
        let mut evidence_chirho = HashMap::new();
        evidence_chirho.insert(CoreIdChirho(50), ("Eq".to_string(), "Int".to_string()));
        let result_chirho = dict_pass_module_full_with_method_occurrences_chirho(
            &module_chirho,
            names_chirho,
            &TyEnvChirho::new_chirho(),
            &ClassEnvChirho::new_chirho(),
            HashMap::new(),
            HashMap::new(),
            HashSet::new(),
            canon_chirho,
            evidence_chirho,
        );
        let main_chirho = result_chirho
            .module_chirho
            .bindings_chirho
            .iter()
            .find(|b_chirho| b_chirho.binder_chirho.name_chirho == "main")
            .expect("main binding should exist");
        let head_id_chirho = app_spine_head_id_chirho(&main_chirho.rhs_chirho)
            .expect("main body should stay an application spine");
        assert_eq!(
            result_chirho
                .names_chirho
                .get(&head_id_chirho)
                .map(String::as_str),
            Some("$prim_Eq_==_Int"),
            "evidence must dispatch the occurrence head to the $prim row"
        );
    }

    #[test]
    fn occurrence_without_evidence_restores_canonical_chirho() {
        let (module_chirho, names_chirho, canon_chirho) = occurrence_test_module_chirho();
        let result_chirho = dict_pass_module_full_with_method_occurrences_chirho(
            &module_chirho,
            names_chirho,
            &TyEnvChirho::new_chirho(),
            &ClassEnvChirho::new_chirho(),
            HashMap::new(),
            HashMap::new(),
            HashSet::new(),
            canon_chirho,
            HashMap::new(),
        );
        let main_chirho = result_chirho
            .module_chirho
            .bindings_chirho
            .iter()
            .find(|b_chirho| b_chirho.binder_chirho.name_chirho == "main")
            .expect("main binding should exist");
        let head_id_chirho = app_spine_head_id_chirho(&main_chirho.rhs_chirho)
            .expect("main body should stay an application spine");
        assert_eq!(
            head_id_chirho,
            CoreIdChirho(60),
            "without evidence the occurrence must be restored to the canonical id (guaranteed elimination)"
        );
    }
}
