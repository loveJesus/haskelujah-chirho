// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Typeclass deriving
//!
//! Generates instance declarations for `deriving` clauses on data type
//! and newtype declarations. Supported classes:
//!
//! - `Eq` — structural equality via `(==)`
//! - `Ord` — structural ordering via `compare`
//! - `Show` — `show`, rendered at GHC's precedence from constructor metadata (`show_chirho.rs`)
//! - `Enum` — `toEnum`/`fromEnum` for enumeration types (nullary constructors only)
//! - `Bounded` — `minBound`/`maxBound` for enumeration types
//! - `Read` — `readsPrec` for structural parsing from strings
//!
//! ## Approach
//!
//! Given a data type like:
//! ```haskell
//! data Color = Red | Green | Blue deriving (Eq, Ord, Show)
//! ```
//!
//! This module generates AST-level `InstanceDeclChirho` nodes equivalent to:
//! ```haskell
//! instance Eq Color where
//!   (==) Red   Red   = True
//!   (==) Green Green = True
//!   (==) Blue  Blue  = True
//!   (==) _     _     = False
//!
//! instance Ord Color where
//!   compare x y = compare (conIndex x) (conIndex y)
//!
//! instance Show Color where
//!   show Red   = "Red"
//!   show Green = "Green"
//!   show Blue  = "Blue"
//! ```
//!
//! The generated instances are inserted into the module's declaration list
//! before type inference runs, so they go through the normal pipeline.
//!
//! Each class carries its clause's deriving strategy; `strategy_route_chirho.rs` decides
//! whether the class's generator, an empty anyclass instance or the representation's
//! instance (`newtype_route_chirho.rs`, GND) produces it (row 694).

pub(crate) mod class_shapes_chirho;
mod declaration_heads_chirho;
#[path = "generators_chirho/enum_chirho.rs"]
mod enum_chirho;
pub(crate) mod extension_policy_chirho;
mod newtype_route_chirho;
#[path = "generators_chirho/show_chirho.rs"]
mod show_chirho;
pub(crate) mod stock_identity_chirho;
mod strategy_route_chirho;
#[cfg(test)]
#[path = "tests_chirho/identity_chirho/strategy_route_tests_chirho.rs"]
mod strategy_route_tests_chirho;

pub(crate) use declaration_heads_chirho::target_name_chirho;
pub use enum_chirho::{derived_enum_metadata_chirho, derived_enum_metadata_with_builtins_chirho};
#[path = "generators_chirho/comparison_chirho.rs"]
mod comparison_chirho;
#[path = "generators_chirho/generic_chirho.rs"]
mod generic_chirho;
#[path = "generators_chirho/higher_kinded_chirho.rs"]
mod higher_kinded_chirho;
#[path = "generators_chirho/via_chirho.rs"]
mod via_chirho;
use comparison_chirho::{derive_eq_chirho, derive_ord_chirho};
use enum_chirho::{derive_bounded_chirho, derive_enum_chirho, derive_read_chirho};
use generic_chirho::derive_generic_chirho;
use higher_kinded_chirho::{
    derive_foldable_chirho, derive_functor_chirho, derive_traversable_chirho,
};
use show_chirho::derive_show_chirho;
pub use show_chirho::{
    DerivedShowChirho, DerivedShowConChirho, ShowConFormChirho, ShowFieldTypeChirho,
    derived_show_metadata_chirho, derived_show_metadata_with_builtins_chirho,
};
use strategy_route_chirho::{DeclaredTypeChirho, DerivingPolicyChirho};
pub use strategy_route_chirho::{
    GndRequestChirho, newtype_strategy_classes_chirho, newtype_strategy_requests_chirho,
};
use via_chirho::derive_via_chirho;

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, TyVarChirho};
use haskelujah_ast_chirho::expr_chirho::{
    AltChirho, ExprChirho, LocalBindChirho, MatchArmChirho, RhsChirho,
};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{BuiltinClassScopeChirho, NameChirho, RawNameChirho};
use haskelujah_ast_chirho::occurrences_chirho::remint_decls_chirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Result of the deriving pass.
#[derive(Debug, Clone)]
pub struct DerivingResultChirho {
    /// Generated instance declarations.
    pub instances_chirho: Vec<DeclChirho>,
    /// Warnings/errors (class not supported, etc.).
    pub warnings_chirho: Vec<String>,
}

/// Process all `deriving` clauses in a module, generating instance declarations.
/// Without the builtin classes naming proved, a GND head keeps every type variable.
pub fn derive_instances_chirho(module_chirho: &ModuleChirho) -> DerivingResultChirho {
    derive_instances_with_builtins_chirho(module_chirho, &BuiltinClassScopeChirho::default())
}

/// Process all `deriving` clauses, eta-reducing a GND head by the arity of the builtin
/// class naming proved (`Functor` drops one variable).
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
pub fn derive_instances_with_builtins_chirho(
    module_chirho: &ModuleChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
) -> DerivingResultChirho {
    derive_instances_with_type_scope_chirho(module_chirho, builtin_classes_chirho, None)
}

fn derive_instances_with_type_scope_chirho(
    module_chirho: &ModuleChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
    type_scope_chirho: Option<&haskelujah_ast_chirho::name_chirho::TypeConstructorScopeChirho>,
) -> DerivingResultChirho {
    let mut instances_chirho = Vec::new();
    let mut warnings_chirho = Vec::new();
    let policy_chirho = DerivingPolicyChirho::with_type_scope_chirho(
        module_chirho,
        builtin_classes_chirho,
        type_scope_chirho,
    );

    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                deriving_chirho,
                kind_sig_chirho,
                span_chirho,
                ..
            } => {
                let visible_vars_chirho = declaration_heads_chirho::visible_instance_binders_chirho(
                    type_vars_chirho,
                    kind_sig_chirho.as_ref(),
                    *span_chirho,
                );
                let declared_chirho = DeclaredTypeChirho {
                    name_chirho,
                    type_vars_chirho: &visible_vars_chirho,
                    constructors_chirho,
                    newtype_constructor_chirho: None,
                    span_chirho: *span_chirho,
                };
                strategy_route_chirho::derive_declared_classes_chirho(
                    &declared_chirho,
                    deriving_chirho,
                    &policy_chirho,
                    &mut instances_chirho,
                    &mut warnings_chirho,
                );
            }
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructor_chirho,
                deriving_chirho,
                kind_sig_chirho,
                span_chirho,
                ..
            } => {
                let cons_chirho = vec![constructor_chirho.as_ref().clone()];
                let visible_vars_chirho = declaration_heads_chirho::visible_instance_binders_chirho(
                    type_vars_chirho,
                    kind_sig_chirho.as_ref(),
                    *span_chirho,
                );
                let declared_chirho = DeclaredTypeChirho {
                    name_chirho,
                    type_vars_chirho: &visible_vars_chirho,
                    constructors_chirho: &cons_chirho,
                    newtype_constructor_chirho: Some(constructor_chirho),
                    span_chirho: *span_chirho,
                };
                strategy_route_chirho::derive_declared_classes_chirho(
                    &declared_chirho,
                    deriving_chirho,
                    &policy_chirho,
                    &mut instances_chirho,
                    &mut warnings_chirho,
                );
            }
            _ => {}
        }
    }

    // Process DerivingVia entries: `deriving (Class) via ViaType`
    for (type_name_chirho, class_name_chirho, via_type_chirho) in &module_chirho.deriving_via_chirho
    {
        instances_chirho.push(derive_via_chirho(
            module_chirho,
            type_name_chirho,
            class_name_chirho,
            via_type_chirho,
        ));
    }

    declaration_heads_chirho::derive_standalone_instances_chirho(
        module_chirho,
        &policy_chirho,
        &mut instances_chirho,
        &mut warnings_chirho,
    );

    DerivingResultChirho {
        instances_chirho,
        warnings_chirho,
    }
}

/// Generate a DeriveAnyClass instance: empty methods list relying on defaults. The head is
/// the class's written arguments, then the applied type: `C args (T vs)` (row 694).
fn derive_anyclass_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    class_name_chirho: &NameChirho,
    class_arguments_chirho: &[TypeChirho],
    span_chirho: SpanChirho,
) -> DeclChirho {
    let mut types_chirho = class_arguments_chirho.to_vec();
    types_chirho.push(instance_type_chirho(type_name_chirho, type_vars_chirho));
    DeclChirho::InstanceDeclChirho {
        kind_args_chirho: Vec::new(),
        context_chirho: vec![],
        class_chirho: class_name_chirho.clone(),
        types_chirho,
        methods_chirho: vec![], // rely on default methods
        assoc_tf_instances_chirho: vec![],
        span_chirho,
    }
}

/// Insert generated instances into a module's declaration list.
pub fn apply_deriving_chirho(module_chirho: &mut ModuleChirho) -> Vec<String> {
    apply_deriving_with_builtins_chirho(module_chirho, &BuiltinClassScopeChirho::default())
}

/// Insert generated instances, eta-reducing GND heads by the builtin classes naming proved.
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
pub fn apply_deriving_with_builtins_chirho(
    module_chirho: &mut ModuleChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
) -> Vec<String> {
    apply_deriving_with_type_scope_chirho(module_chirho, builtin_classes_chirho, None)
}

/// The frontend's one naming scope also proves written kind atoms used by deriving.
/// Workflow: compiler-pipeline-chirho/builtin-class-identity-chirho.
pub fn apply_deriving_with_type_scope_chirho(
    module_chirho: &mut ModuleChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
    type_scope_chirho: Option<&haskelujah_ast_chirho::name_chirho::TypeConstructorScopeChirho>,
) -> Vec<String> {
    let mut result_chirho = derive_instances_with_type_scope_chirho(
        module_chirho,
        builtin_classes_chirho,
        type_scope_chirho,
    );
    // The instances are fresh output: every occurrence in them takes a new
    // origin from the module's supply before anything can read them, including
    // any subtree a generator copied from user code.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    remint_decls_chirho(
        &mut result_chirho.instances_chirho,
        &mut module_chirho.origin_supply_chirho,
    );
    module_chirho
        .decls_chirho
        .extend(result_chirho.instances_chirho);
    result_chirho.warnings_chirho
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Dummy span for generated code.
fn gen_span_chirho() -> SpanChirho {
    SpanChirho::DUMMY_CHIRHO
}

/// Create a simple variable name.
fn var_name_chirho(s_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        s_chirho,
        gen_span_chirho(),
    ))
}

/// Create a variable expression.
fn var_expr_chirho(s_chirho: &str) -> ExprChirho {
    ExprChirho::VarChirho(var_name_chirho(s_chirho))
}

/// Create a constructor expression.
fn con_expr_chirho(s_chirho: &str) -> ExprChirho {
    ExprChirho::ConChirho(var_name_chirho(s_chirho))
}

/// Create a simple application `f x`.
fn app_chirho(f_chirho: ExprChirho, x_chirho: ExprChirho) -> ExprChirho {
    ExprChirho::AppChirho {
        fun_chirho: Box::new(f_chirho),
        arg_chirho: Box::new(x_chirho),
        span_chirho: gen_span_chirho(),
    }
}

/// Create an infix application `a op b`.
fn infix_chirho(left_chirho: ExprChirho, op_chirho: &str, right_chirho: ExprChirho) -> ExprChirho {
    ExprChirho::InfixChirho {
        left_chirho: Box::new(left_chirho),
        op_chirho: var_name_chirho(op_chirho),
        right_chirho: Box::new(right_chirho),
        span_chirho: gen_span_chirho(),
    }
}

fn eq_primop_for_type_chirho(ty_chirho: &TypeChirho) -> Option<&'static str> {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => match name_chirho.text_chirho() {
            "Int" | "Integer" => Some("==#"),
            "Double" | "Float" => Some("eqFloat#"),
            "String" => Some("eqStr#"),
            _ => None,
        },
        TypeChirho::ListChirho { element_chirho, .. } => match element_chirho.as_ref() {
            TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Char" => {
                Some("eqStr#")
            }
            _ => None,
        },
        _ => None,
    }
}

fn eq_expr_for_type_chirho(
    left_chirho: ExprChirho,
    right_chirho: ExprChirho,
    ty_chirho: &TypeChirho,
) -> ExprChirho {
    let op_chirho = eq_primop_for_type_chirho(ty_chirho).unwrap_or("==");
    infix_chirho(left_chirho, op_chirho, right_chirho)
}

/// Get the constructor name from a ConDeclChirho.
fn con_name_chirho(con_chirho: &ConDeclChirho) -> &str {
    match con_chirho {
        ConDeclChirho::OrdinaryChirho { name_chirho, .. }
        | ConDeclChirho::RecordChirho { name_chirho, .. }
        | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho.text_chirho(),
    }
}

/// Get the number of fields for a constructor.
fn con_field_count_chirho(con_chirho: &ConDeclChirho) -> usize {
    match con_chirho {
        ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => fields_chirho.len(),
        ConDeclChirho::RecordChirho { fields_chirho, .. } => fields_chirho
            .iter()
            .map(|field_chirho| field_chirho.names_chirho.len())
            .sum(),
        ConDeclChirho::GadtChirho {
            signature_chirho, ..
        } => signature_chirho.positional_fields_chirho().count(),
    }
}

/// Create field variable names like `a1`, `a2`, `b1`, `b2`.
fn field_vars_chirho(prefix_chirho: &str, count_chirho: usize) -> Vec<String> {
    (1..=count_chirho)
        .map(|i_chirho| format!("{}{}", prefix_chirho, i_chirho))
        .collect()
}

/// Build a constructor pattern with field bindings.
fn con_pat_chirho(con_name_str_chirho: &str, field_names_chirho: &[String]) -> PatChirho {
    if field_names_chirho.is_empty() {
        PatChirho::ConChirho {
            type_args_chirho: Vec::new(),
            con_chirho: var_name_chirho(con_name_str_chirho),
            args_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }
    } else {
        PatChirho::ConChirho {
            type_args_chirho: Vec::new(),
            con_chirho: var_name_chirho(con_name_str_chirho),
            args_chirho: field_names_chirho
                .iter()
                .map(|n_chirho| PatChirho::VarChirho(var_name_chirho(n_chirho)))
                .collect(),
            span_chirho: gen_span_chirho(),
        }
    }
}

/// Build the instance head type: `T a b c` from type name and type vars.
fn instance_type_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
) -> TypeChirho {
    if type_vars_chirho.is_empty() {
        TypeChirho::ConChirho(type_name_chirho.clone())
    } else {
        let mut result_chirho = TypeChirho::ConChirho(type_name_chirho.clone());
        for tv_chirho in type_vars_chirho {
            result_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(TypeChirho::VarChirho(tv_chirho.name_chirho.clone())),
                span_chirho: gen_span_chirho(),
            };
        }
        result_chirho
    }
}

/// Get the field types for a constructor.
fn con_field_types_chirho(con_chirho: &ConDeclChirho) -> Vec<TypeChirho> {
    match con_chirho {
        ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => fields_chirho
            .iter()
            .map(|(_s_chirho, ty_chirho)| ty_chirho.clone())
            .collect(),
        ConDeclChirho::RecordChirho { fields_chirho, .. } => fields_chirho
            .iter()
            .map(|f_chirho| f_chirho.ty_chirho.clone())
            .collect(),
        ConDeclChirho::GadtChirho {
            signature_chirho, ..
        } => signature_chirho
            .positional_fields_chirho()
            .map(|field_chirho| field_chirho.ty_chirho.clone())
            .collect(),
    }
}

/// Check whether a type IS exactly the given type variable (by name, ignoring span).
fn type_is_var_chirho(ty_chirho: &TypeChirho, var_chirho: &str) -> bool {
    matches!(ty_chirho, TypeChirho::VarChirho(n_chirho) if n_chirho.text_chirho() == var_chirho)
}

/// Check whether a type mentions a given type variable name.
fn type_mentions_var_chirho(ty_chirho: &TypeChirho, var_chirho: &str) -> bool {
    match ty_chirho {
        TypeChirho::VarChirho(n_chirho) => n_chirho.text_chirho() == var_chirho,
        TypeChirho::ConChirho(_) => false,
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            type_mentions_var_chirho(fun_chirho, var_chirho)
                || type_mentions_var_chirho(arg_chirho, var_chirho)
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            type_mentions_var_chirho(arg_chirho, var_chirho)
                || type_mentions_var_chirho(result_chirho, var_chirho)
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => elements_chirho
            .iter()
            .any(|e_chirho| type_mentions_var_chirho(e_chirho, var_chirho)),
        TypeChirho::ListChirho { element_chirho, .. } => {
            type_mentions_var_chirho(element_chirho, var_chirho)
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            type_mentions_var_chirho(inner_chirho, var_chirho)
        }
        _ => false,
    }
}

#[cfg(test)]
#[path = "tests_chirho/mod.rs"]
mod tests_chirho;
