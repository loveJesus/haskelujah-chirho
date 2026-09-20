// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Typeclass deriving
//!
//! Generates instance declarations for `deriving` clauses on data type
//! and newtype declarations. Supported classes:
//!
//! - `Eq` — structural equality via `(==)`
//! - `Ord` — structural ordering via `compare`
//! - `Show` — structural pretty-printing via `show`
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

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, TyVarChirho};
use haskelujah_ast_chirho::expr_chirho::{
    AltChirho, ExprChirho, LocalBindChirho, MatchArmChirho, RhsChirho,
};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

mod builders_chirho;
mod higher_chirho;
pub(crate) mod newtype_chirho;
mod stock_chirho;
pub(crate) use stock_chirho::StockClassChirho;
mod via_chirho;
use builders_chirho::*;
use higher_chirho::*;
use via_chirho::*;
#[cfg(test)]
mod newtype_tests_chirho;
#[cfg(test)]
mod tests_chirho;

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

/// Generate stock/via instances before naming. Generalized newtype instances
/// need checked declaration kinds and are inserted inside kind inference.
pub fn derive_instances_chirho(module_chirho: &ModuleChirho) -> DerivingResultChirho {
    let mut instances_chirho = Vec::new();
    let mut warnings_chirho = Vec::new();

    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                deriving_chirho,
                span_chirho,
                ..
            } => {
                let visible_vars_chirho = TyVarChirho::visible_binders_chirho(type_vars_chirho);
                let type_vars_chirho = visible_vars_chirho.as_ref();
                for application_chirho in deriving_chirho {
                    let Some((class_chirho, _)) =
                        application_chirho.constructor_application_chirho()
                    else {
                        warnings_chirho.push(
                            "deriving application is not represented as a class head".to_owned(),
                        );
                        continue;
                    };
                    if StockClassChirho::from_name_chirho(class_chirho.text_chirho()).is_some() {
                        derive_class_for_data_chirho(
                            class_chirho.text_chirho(),
                            name_chirho,
                            type_vars_chirho,
                            constructors_chirho,
                            *span_chirho,
                            &mut instances_chirho,
                            &mut warnings_chirho,
                        );
                        continue;
                    }
                    match class_chirho.text_chirho() {
                        // Preserve the legacy NFData empty-instance path.
                        "NFData" => {
                            instances_chirho.push(derive_anyclass_chirho(
                                name_chirho,
                                type_vars_chirho,
                                class_chirho,
                                *span_chirho,
                            ));
                        }
                        other_chirho => {
                            if module_chirho
                                .extensions_chirho
                                .iter()
                                .any(|e_chirho| e_chirho == "DeriveAnyClass")
                            {
                                // DeriveAnyClass: generate empty instance relying on defaults
                                instances_chirho.push(derive_anyclass_chirho(
                                    name_chirho,
                                    type_vars_chirho,
                                    class_chirho,
                                    *span_chirho,
                                ));
                            } else {
                                warnings_chirho.push(format!(
                                    "deriving {} not yet supported for {}",
                                    other_chirho,
                                    name_chirho.text_chirho()
                                ));
                            }
                        }
                    }
                }
            }
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructor_chirho,
                deriving_chirho,
                span_chirho,
                ..
            } => {
                let cons_chirho = vec![constructor_chirho.clone()];
                let visible_vars_chirho = TyVarChirho::visible_binders_chirho(type_vars_chirho);
                let type_vars_chirho = visible_vars_chirho.as_ref();
                for application_chirho in deriving_chirho {
                    let Some((class_chirho, _)) =
                        application_chirho.constructor_application_chirho()
                    else {
                        warnings_chirho.push(
                            "deriving application is not represented as a class head".to_owned(),
                        );
                        continue;
                    };
                    if StockClassChirho::from_name_chirho(class_chirho.text_chirho()).is_some() {
                        derive_class_for_data_chirho(
                            class_chirho.text_chirho(),
                            name_chirho,
                            type_vars_chirho,
                            &cons_chirho,
                            *span_chirho,
                            &mut instances_chirho,
                            &mut warnings_chirho,
                        );
                    }
                    // The other newtype applications need closed kinds and
                    // are elaborated inside kind inference, before instances.
                }
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

    // Process standalone deriving declarations: `deriving instance Show Foo`
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::StandaloneDerivingDeclChirho {
            class_chirho,
            types_chirho,
            span_chirho,
            ..
        } = decl_chirho
        {
            // Find the first type argument (the target type name)
            let type_name_chirho = types_chirho.first().and_then(|t_chirho| match t_chirho {
                TypeChirho::ConChirho(n_chirho) => Some(n_chirho.text_chirho()),
                _ => None,
            });

            if let Some(target_chirho) = type_name_chirho {
                // Find matching data/newtype decl
                let found_chirho = derive_standalone_for_type_chirho(
                    module_chirho,
                    class_chirho,
                    &target_chirho,
                    *span_chirho,
                    &mut instances_chirho,
                    &mut warnings_chirho,
                );
                if !found_chirho {
                    warnings_chirho.push(format!(
                        "standalone deriving: type {} not found in module",
                        target_chirho
                    ));
                }
            }
        }
    }

    DerivingResultChirho {
        instances_chirho,
        warnings_chirho,
    }
}

/// Process a standalone deriving declaration by finding the target data/newtype
/// in the module and deriving the requested class for it.
fn derive_standalone_for_type_chirho(
    module_chirho: &ModuleChirho,
    class_chirho: &NameChirho,
    target_chirho: &str,
    span_chirho: SpanChirho,
    instances_chirho: &mut Vec<DeclChirho>,
    warnings_chirho: &mut Vec<String>,
) -> bool {
    let class_text_chirho = class_chirho.text_chirho();

    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                ..
            } if name_chirho.text_chirho() == target_chirho => {
                let visible_vars_chirho = TyVarChirho::visible_binders_chirho(type_vars_chirho);
                derive_class_for_data_chirho(
                    &class_text_chirho,
                    name_chirho,
                    visible_vars_chirho.as_ref(),
                    constructors_chirho,
                    span_chirho,
                    instances_chirho,
                    warnings_chirho,
                );
                return true;
            }
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructor_chirho,
                ..
            } if name_chirho.text_chirho() == target_chirho => {
                let cons_chirho = vec![constructor_chirho.clone()];
                let visible_vars_chirho = TyVarChirho::visible_binders_chirho(type_vars_chirho);
                derive_class_for_data_chirho(
                    &class_text_chirho,
                    name_chirho,
                    visible_vars_chirho.as_ref(),
                    &cons_chirho,
                    span_chirho,
                    instances_chirho,
                    warnings_chirho,
                );
                return true;
            }
            _ => {}
        }
    }
    false
}

/// Derive a specific class for a data type (shared by inline deriving and standalone).
fn derive_class_for_data_chirho(
    class_text_chirho: &str,
    name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    span_chirho: SpanChirho,
    instances_chirho: &mut Vec<DeclChirho>,
    warnings_chirho: &mut Vec<String>,
) {
    if let Some(class_chirho) = StockClassChirho::from_name_chirho(class_text_chirho) {
        match class_chirho.derive_chirho(
            name_chirho,
            type_vars_chirho,
            constructors_chirho,
            span_chirho,
        ) {
            Ok(instance_chirho) => instances_chirho.push(instance_chirho),
            Err(message_chirho) => warnings_chirho.push(message_chirho),
        }
    } else if class_text_chirho != "NFData" {
        warnings_chirho.push(format!(
            "standalone deriving {} not yet supported for {}",
            class_text_chirho,
            name_chirho.text_chirho()
        ));
    }
}

/// Insert generated instances into a module's declaration list.
pub fn apply_deriving_chirho(module_chirho: &mut ModuleChirho) -> Vec<String> {
    let result_chirho = derive_instances_chirho(module_chirho);
    module_chirho
        .decls_chirho
        .extend(result_chirho.instances_chirho);
    result_chirho.warnings_chirho
}
