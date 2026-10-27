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
mod parameters_chirho;
pub mod prelude_chirho;
#[cfg(test)]
mod print_evidence_tests_chirho;
pub mod rewrite_chirho;
mod show_chirho;
mod type_keys_chirho;

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::deriving_chirho::DerivedShowChirho;
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

/// A final derived-Show answer (row 533). `Renders` holds at the deepest nesting it was
/// found at and any shallower one; deeper, the key is checked again against the depth
/// bound. A semantic failure holds at every depth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DerivedShowKnownChirho {
    RendersChirho { deepest_chirho: usize },
    FailsChirho,
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
    /// Module `$prim` rows whose instance has a context → that context, in
    /// order: those rows take the context's dictionaries before the method's
    /// arguments. Recorded before any binding is rewritten.
    /// workflow: language-features-chirho/own-class-method-occurrences-chirho
    context_leading_rows_chirho:
        HashMap<CoreIdChirho, Vec<haskelujah_typing_chirho::class_chirho::PredChirho>>,
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
    /// Types whose deriving clause names Show → their constructor metadata (row 533).
    /// workflow: print-show-evidence-chirho
    derived_shows_chirho: HashMap<String, DerivedShowChirho>,
    /// Derived-Show keys whose renderability is being checked, for recursive types.
    derived_show_visiting_chirho: RefCell<Vec<String>>,
    /// Keys assumed renderable while their own check was under way, since the
    /// outermost check began.
    derived_show_assumed_chirho: RefCell<Vec<String>>,
    /// Final answers per derived-Show key.
    derived_show_known_chirho: RefCell<HashMap<String, DerivedShowKnownChirho>>,
    /// Keys that rendered, during the current outermost check, on an assumption about a
    /// key still under way, with the deepest nesting at which they did. They become final
    /// when that check renders and are dropped when it does not.
    derived_show_provisional_chirho: RefCell<HashMap<String, usize>>,
    /// Whether the current outermost check has met the depth bound: a `false` found after
    /// that may rest on the bound rather than on the type, so it is not kept.
    derived_show_bounded_chirho: Cell<bool>,
    /// Derived-Show key → its generated `$prim_Show_showsPrec_<key>` helper.
    derived_show_helpers_chirho: HashMap<String, CoreIdChirho>,
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
            context_leading_rows_chirho: HashMap::new(),
            super_selectors_chirho: HashMap::new(),
            conditional_dicts_chirho: HashMap::new(),
            newtype_info_chirho: HashMap::new(),
            class_param_count_chirho: HashMap::new(),
            local_shadow_ids_chirho: RefCell::new(HashSet::new()),
            monad_context_stack_chirho: RefCell::new(Vec::new()),
            derived_shows_chirho: HashMap::new(),
            derived_show_visiting_chirho: RefCell::new(Vec::new()),
            derived_show_assumed_chirho: RefCell::new(Vec::new()),
            derived_show_known_chirho: RefCell::new(HashMap::new()),
            derived_show_provisional_chirho: RefCell::new(HashMap::new()),
            derived_show_bounded_chirho: Cell::new(false),
            derived_show_helpers_chirho: HashMap::new(),
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
/// the checker's per-reference evidence for constrained references, and the
/// constructor metadata of every type that derives Show (row 533).
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
    derived_shows_chirho: HashMap<String, DerivedShowChirho>,
) -> DictPassResultChirho {
    let max_id_chirho = find_max_id_chirho(module_chirho);
    let mut ctx_chirho =
        DictPassCtxChirho::new_chirho(max_id_chirho + 1, names_chirho, con_types_chirho);
    ctx_chirho.newtype_info_chirho = newtype_info_chirho;
    ctx_chirho.extra_dict_param_names_chirho = extra_dict_param_names_chirho;
    ctx_chirho.method_occurrence_canon_chirho = method_occurrence_canon_chirho;
    ctx_chirho.occurrence_evidence_chirho = occurrence_evidence_chirho;
    ctx_chirho.reference_evidence_chirho = reference_evidence_chirho;
    ctx_chirho.derived_shows_chirho = derived_shows_chirho;
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
#[path = "dict_pass_tests_chirho.rs"]
mod tests_chirho;
