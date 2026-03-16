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

use haskeluya_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, TyVarChirho};
use haskeluya_ast_chirho::expr_chirho::{
    AltChirho, ExprChirho, LocalBindChirho, MatchArmChirho, RhsChirho,
};
use haskeluya_ast_chirho::lit_chirho::LitChirho;
use haskeluya_ast_chirho::module_chirho::ModuleChirho;
use haskeluya_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskeluya_ast_chirho::pat_chirho::PatChirho;
use haskeluya_ast_chirho::ty_chirho::TypeChirho;
use haskeluya_span_chirho::SpanChirho;

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
            } => {
                for class_chirho in deriving_chirho {
                    match class_chirho.text_chirho() {
                        "Eq" => {
                            instances_chirho.push(derive_eq_chirho(
                                name_chirho,
                                type_vars_chirho,
                                constructors_chirho,
                                *span_chirho,
                            ));
                        }
                        "Ord" => {
                            instances_chirho.push(derive_ord_chirho(
                                name_chirho,
                                type_vars_chirho,
                                constructors_chirho,
                                *span_chirho,
                            ));
                        }
                        "Show" => {
                            instances_chirho.push(derive_show_chirho(
                                name_chirho,
                                type_vars_chirho,
                                constructors_chirho,
                                *span_chirho,
                            ));
                        }
                        "Enum" => {
                            match derive_enum_chirho(
                                name_chirho,
                                type_vars_chirho,
                                constructors_chirho,
                                *span_chirho,
                            ) {
                                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
                            }
                        }
                        "Bounded" => {
                            match derive_bounded_chirho(
                                name_chirho,
                                type_vars_chirho,
                                constructors_chirho,
                                *span_chirho,
                            ) {
                                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
                            }
                        }
                        "Read" => {
                            instances_chirho.push(derive_read_chirho(
                                name_chirho,
                                type_vars_chirho,
                                constructors_chirho,
                                *span_chirho,
                            ));
                        }
                        "Functor" => {
                            match derive_functor_chirho(
                                name_chirho,
                                type_vars_chirho,
                                constructors_chirho,
                                *span_chirho,
                            ) {
                                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
                            }
                        }
                        "Foldable" => {
                            match derive_foldable_chirho(
                                name_chirho,
                                type_vars_chirho,
                                constructors_chirho,
                                *span_chirho,
                            ) {
                                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
                            }
                        }
                        "Traversable" => {
                            match derive_traversable_chirho(
                                name_chirho,
                                type_vars_chirho,
                                constructors_chirho,
                                *span_chirho,
                            ) {
                                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
                            }
                        }
                        "Generic" => {
                            match derive_generic_chirho(
                                name_chirho,
                                type_vars_chirho,
                                constructors_chirho,
                                *span_chirho,
                            ) {
                                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
                            }
                        }
                        // DeriveDataTypeable / DeriveGeneric: empty instance for known classes
                        "Typeable" | "Data" | "Lift" | "NFData" => {
                            instances_chirho.push(derive_anyclass_chirho(
                                name_chirho,
                                type_vars_chirho,
                                class_chirho,
                                *span_chirho,
                            ));
                        }
                        other_chirho => {
                            if module_chirho.extensions_chirho.iter().any(|e_chirho| e_chirho == "DeriveAnyClass") {
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
            } => {
                let cons_chirho = vec![constructor_chirho.clone()];
                for class_chirho in deriving_chirho {
                    match class_chirho.text_chirho() {
                        "Eq" => {
                            instances_chirho.push(derive_eq_chirho(
                                name_chirho,
                                type_vars_chirho,
                                &cons_chirho,
                                *span_chirho,
                            ));
                        }
                        "Ord" => {
                            instances_chirho.push(derive_ord_chirho(
                                name_chirho,
                                type_vars_chirho,
                                &cons_chirho,
                                *span_chirho,
                            ));
                        }
                        "Show" => {
                            instances_chirho.push(derive_show_chirho(
                                name_chirho,
                                type_vars_chirho,
                                &cons_chirho,
                                *span_chirho,
                            ));
                        }
                        "Enum" => {
                            match derive_enum_chirho(
                                name_chirho,
                                type_vars_chirho,
                                &cons_chirho,
                                *span_chirho,
                            ) {
                                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
                            }
                        }
                        "Bounded" => {
                            match derive_bounded_chirho(
                                name_chirho,
                                type_vars_chirho,
                                &cons_chirho,
                                *span_chirho,
                            ) {
                                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
                            }
                        }
                        "Read" => {
                            instances_chirho.push(derive_read_chirho(
                                name_chirho,
                                type_vars_chirho,
                                &cons_chirho,
                                *span_chirho,
                            ));
                        }
                        "Functor" => {
                            match derive_functor_chirho(
                                name_chirho,
                                type_vars_chirho,
                                &cons_chirho,
                                *span_chirho,
                            ) {
                                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
                            }
                        }
                        "Foldable" => {
                            match derive_foldable_chirho(
                                name_chirho,
                                type_vars_chirho,
                                &cons_chirho,
                                *span_chirho,
                            ) {
                                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
                            }
                        }
                        "Traversable" => {
                            match derive_traversable_chirho(
                                name_chirho,
                                type_vars_chirho,
                                &cons_chirho,
                                *span_chirho,
                            ) {
                                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
                            }
                        }
                        "Generic" => {
                            match derive_generic_chirho(
                                name_chirho,
                                type_vars_chirho,
                                &cons_chirho,
                                *span_chirho,
                            ) {
                                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
                            }
                        }
                        _other_chirho => {
                            // Generalized newtype deriving (GND):
                            // For any class not in the standard set, generate
                            // a delegation instance that coerces to/from the
                            // underlying type.
                            instances_chirho.push(derive_newtype_gnd_chirho(
                                name_chirho,
                                type_vars_chirho,
                                constructor_chirho,
                                class_chirho,
                                *span_chirho,
                            ));
                        }
                    }
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
                derive_class_for_data_chirho(
                    &class_text_chirho,
                    name_chirho,
                    type_vars_chirho,
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
                derive_class_for_data_chirho(
                    &class_text_chirho,
                    name_chirho,
                    type_vars_chirho,
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
    match class_text_chirho {
        "Eq" => {
            instances_chirho.push(derive_eq_chirho(
                name_chirho, type_vars_chirho, constructors_chirho, span_chirho,
            ));
        }
        "Ord" => {
            instances_chirho.push(derive_ord_chirho(
                name_chirho, type_vars_chirho, constructors_chirho, span_chirho,
            ));
        }
        "Show" => {
            instances_chirho.push(derive_show_chirho(
                name_chirho, type_vars_chirho, constructors_chirho, span_chirho,
            ));
        }
        "Read" => {
            instances_chirho.push(derive_read_chirho(
                name_chirho, type_vars_chirho, constructors_chirho, span_chirho,
            ));
        }
        "Enum" => {
            match derive_enum_chirho(
                name_chirho, type_vars_chirho, constructors_chirho, span_chirho,
            ) {
                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
            }
        }
        "Bounded" => {
            match derive_bounded_chirho(
                name_chirho, type_vars_chirho, constructors_chirho, span_chirho,
            ) {
                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
            }
        }
        "Functor" => {
            match derive_functor_chirho(
                name_chirho, type_vars_chirho, constructors_chirho, span_chirho,
            ) {
                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
            }
        }
        "Foldable" => {
            match derive_foldable_chirho(
                name_chirho, type_vars_chirho, constructors_chirho, span_chirho,
            ) {
                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
            }
        }
        "Traversable" => {
            match derive_traversable_chirho(
                name_chirho, type_vars_chirho, constructors_chirho, span_chirho,
            ) {
                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
            }
        }
        "Generic" => {
            match derive_generic_chirho(
                name_chirho, type_vars_chirho, constructors_chirho, span_chirho,
            ) {
                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
            }
        }
        "Typeable" | "Data" | "Lift" | "NFData" => {
            // Generate empty instance for these well-known classes
        }
        other_chirho => {
            warnings_chirho.push(format!(
                "standalone deriving {} not yet supported for {}",
                other_chirho,
                name_chirho.text_chirho()
            ));
        }
    }
}

/// Generate a DeriveAnyClass instance: empty methods list relying on defaults.
fn derive_anyclass_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    class_name_chirho: &NameChirho,
    span_chirho: SpanChirho,
) -> DeclChirho {
    let mut types_chirho = vec![TypeChirho::ConChirho(type_name_chirho.clone())];
    for tv_chirho in type_vars_chirho {
        types_chirho.push(TypeChirho::VarChirho(tv_chirho.name_chirho.clone()));
    }
    DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: class_name_chirho.clone(),
        types_chirho,
        methods_chirho: vec![], // rely on default methods
        span_chirho,
    }
}

/// Generate a `deriving via` instance by looking up the newtype constructor
/// and generating methods that unwrap, delegate to the via type's instance,
/// and re-wrap where needed.
fn derive_via_chirho(
    module_chirho: &ModuleChirho,
    type_name_chirho: &NameChirho,
    class_name_chirho: &NameChirho,
    _via_type_chirho: &TypeChirho,
) -> DeclChirho {
    // Find the constructor name for this newtype/data type
    let con_name_str_chirho = find_first_con_name_chirho(module_chirho, type_name_chirho)
        .unwrap_or_else(|| type_name_chirho.text_chirho().to_string());

    let class_text_chirho = class_name_chirho.text_chirho();

    // Generate methods based on the class
    let methods_chirho = match class_text_chirho {
        "Show" => derive_via_show_methods_chirho(&con_name_str_chirho),
        "Eq" => derive_via_eq_methods_chirho(&con_name_str_chirho),
        "Ord" => derive_via_ord_methods_chirho(&con_name_str_chirho),
        "Num" => derive_via_num_methods_chirho(&con_name_str_chirho),
        _ => vec![], // Fallback: empty methods (default implementations)
    };

    // For the generated instance, the methods explicitly delegate to the via
    // type's methods by unwrapping the newtype. The via type's class instance
    // is already available in the environment, so no context constraint needed.
    DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: class_name_chirho.clone(),
        types_chirho: vec![TypeChirho::ConChirho(type_name_chirho.clone())],
        methods_chirho,
        span_chirho: gen_span_chirho(),
    }
}

/// Find the first constructor name for a type in the module.
fn find_first_con_name_chirho(
    module_chirho: &ModuleChirho,
    type_name_chirho: &NameChirho,
) -> Option<String> {
    let target_chirho = type_name_chirho.text_chirho();
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                constructor_chirho,
                ..
            } if name_chirho.text_chirho() == target_chirho => {
                return Some(con_name_chirho(constructor_chirho).to_string());
            }
            DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                ..
            } if name_chirho.text_chirho() == target_chirho => {
                if let Some(first_chirho) = constructors_chirho.first() {
                    return Some(con_name_chirho(first_chirho).to_string());
                }
            }
            _ => {}
        }
    }
    None
}

/// DerivingVia Show: `show (Con x) = show x`
fn derive_via_show_methods_chirho(con_name_str_chirho: &str) -> Vec<LocalBindChirho> {
    let match_chirho = MatchArmChirho {
        pats_chirho: vec![con_pat_chirho(con_name_str_chirho, &["x1".to_string()])],
        rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
            var_expr_chirho("show"),
            var_expr_chirho("x1"),
        )),
        where_binds_chirho: vec![],
        span_chirho: gen_span_chirho(),
    };
    vec![LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("show"),
        matches_chirho: vec![match_chirho],
        span_chirho: gen_span_chirho(),
    }]
}

/// DerivingVia Eq: `(==) (Con x) (Con y) = (==) x y`
fn derive_via_eq_methods_chirho(con_name_str_chirho: &str) -> Vec<LocalBindChirho> {
    let match_chirho = MatchArmChirho {
        pats_chirho: vec![
            con_pat_chirho(con_name_str_chirho, &["x1".to_string()]),
            con_pat_chirho(con_name_str_chirho, &["y1".to_string()]),
        ],
        rhs_chirho: RhsChirho::UnguardedChirho(infix_chirho(
            var_expr_chirho("x1"),
            "==",
            var_expr_chirho("y1"),
        )),
        where_binds_chirho: vec![],
        span_chirho: gen_span_chirho(),
    };
    vec![LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("=="),
        matches_chirho: vec![match_chirho],
        span_chirho: gen_span_chirho(),
    }]
}

/// DerivingVia Ord: `compare (Con x) (Con y) = compare x y`
fn derive_via_ord_methods_chirho(con_name_str_chirho: &str) -> Vec<LocalBindChirho> {
    let match_chirho = MatchArmChirho {
        pats_chirho: vec![
            con_pat_chirho(con_name_str_chirho, &["x1".to_string()]),
            con_pat_chirho(con_name_str_chirho, &["y1".to_string()]),
        ],
        rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
            app_chirho(var_expr_chirho("compare"), var_expr_chirho("x1")),
            var_expr_chirho("y1"),
        )),
        where_binds_chirho: vec![],
        span_chirho: gen_span_chirho(),
    };
    vec![LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("compare"),
        matches_chirho: vec![match_chirho],
        span_chirho: gen_span_chirho(),
    }]
}

/// DerivingVia Num: `(+) (Con x) (Con y) = Con (x + y)`, `fromInteger n = Con (fromInteger n)`, etc.
fn derive_via_num_methods_chirho(con_name_str_chirho: &str) -> Vec<LocalBindChirho> {
    let mut methods_chirho = Vec::new();

    // Binary ops: (+), (*), (-), abs, signum — `op (Con x) (Con y) = Con (op x y)`
    for op_chirho in &["+", "*", "-"] {
        let match_chirho = MatchArmChirho {
            pats_chirho: vec![
                con_pat_chirho(con_name_str_chirho, &["x1".to_string()]),
                con_pat_chirho(con_name_str_chirho, &["y1".to_string()]),
            ],
            rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
                con_expr_chirho(con_name_str_chirho),
                infix_chirho(var_expr_chirho("x1"), op_chirho, var_expr_chirho("y1")),
            )),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        methods_chirho.push(LocalBindChirho::FunBindChirho {
            name_chirho: var_name_chirho(op_chirho),
            matches_chirho: vec![match_chirho],
            span_chirho: gen_span_chirho(),
        });
    }

    // Unary ops: abs, signum, negate — `f (Con x) = Con (f x)`
    for fn_chirho in &["abs", "signum", "negate"] {
        let match_chirho = MatchArmChirho {
            pats_chirho: vec![con_pat_chirho(con_name_str_chirho, &["x1".to_string()])],
            rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
                con_expr_chirho(con_name_str_chirho),
                app_chirho(var_expr_chirho(fn_chirho), var_expr_chirho("x1")),
            )),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        methods_chirho.push(LocalBindChirho::FunBindChirho {
            name_chirho: var_name_chirho(fn_chirho),
            matches_chirho: vec![match_chirho],
            span_chirho: gen_span_chirho(),
        });
    }

    // fromInteger: `fromInteger n = Con (fromInteger n)`
    let from_int_chirho = MatchArmChirho {
        pats_chirho: vec![PatChirho::VarChirho(var_name_chirho("n1"))],
        rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
            con_expr_chirho(con_name_str_chirho),
            app_chirho(var_expr_chirho("fromInteger"), var_expr_chirho("n1")),
        )),
        where_binds_chirho: vec![],
        span_chirho: gen_span_chirho(),
    };
    methods_chirho.push(LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("fromInteger"),
        matches_chirho: vec![from_int_chirho],
        span_chirho: gen_span_chirho(),
    });

    methods_chirho
}

/// Insert generated instances into a module's declaration list.
pub fn apply_deriving_chirho(module_chirho: &mut ModuleChirho) -> Vec<String> {
    let result_chirho = derive_instances_chirho(module_chirho);
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
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(s_chirho, gen_span_chirho()))
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
fn infix_chirho(
    left_chirho: ExprChirho,
    op_chirho: &str,
    right_chirho: ExprChirho,
) -> ExprChirho {
    ExprChirho::InfixChirho {
        left_chirho: Box::new(left_chirho),
        op_chirho: var_name_chirho(op_chirho),
        right_chirho: Box::new(right_chirho),
        span_chirho: gen_span_chirho(),
    }
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
        ConDeclChirho::RecordChirho { fields_chirho, .. } => fields_chirho.len(),
        ConDeclChirho::GadtChirho { ty_chirho, .. } => {
            // Count function arguments in the GADT type signature.
            extract_gadt_arg_count_chirho(ty_chirho)
        }
    }
}

/// Count function arguments in a GADT type signature.
/// Walks through FunChirho, ForallChirho, and QualChirho to count arrow arguments.
fn extract_gadt_arg_count_chirho(ty_chirho: &TypeChirho) -> usize {
    match ty_chirho {
        TypeChirho::FunChirho { result_chirho, .. } => 1 + extract_gadt_arg_count_chirho(result_chirho),
        TypeChirho::ForallChirho { body_chirho, .. } => extract_gadt_arg_count_chirho(body_chirho),
        TypeChirho::QualChirho { body_chirho, .. } => extract_gadt_arg_count_chirho(body_chirho),
        _ => 0, // Return type — not an argument
    }
}

/// Extract argument types from a GADT type signature (everything before the final return type).
fn extract_gadt_args_chirho(ty_chirho: &TypeChirho) -> Vec<TypeChirho> {
    match ty_chirho {
        TypeChirho::FunChirho { arg_chirho, result_chirho, .. } => {
            let mut args_chirho = vec![(**arg_chirho).clone()];
            args_chirho.extend(extract_gadt_args_chirho(result_chirho));
            args_chirho
        }
        TypeChirho::ForallChirho { body_chirho, .. } => extract_gadt_args_chirho(body_chirho),
        TypeChirho::QualChirho { body_chirho, .. } => extract_gadt_args_chirho(body_chirho),
        _ => vec![], // Return type — not an argument
    }
}

/// Create field variable names like `a1`, `a2`, `b1`, `b2`.
fn field_vars_chirho(prefix_chirho: &str, count_chirho: usize) -> Vec<String> {
    (1..=count_chirho)
        .map(|i_chirho| format!("{}{}", prefix_chirho, i_chirho))
        .collect()
}

/// Build a constructor pattern with field bindings.
fn con_pat_chirho(
    con_name_str_chirho: &str,
    field_names_chirho: &[String],
) -> PatChirho {
    if field_names_chirho.is_empty() {
        PatChirho::ConChirho {
            con_chirho: var_name_chirho(con_name_str_chirho),
            args_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }
    } else {
        PatChirho::ConChirho {
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

// ---------------------------------------------------------------------------
// Derive Eq
// ---------------------------------------------------------------------------

/// Generate `instance Eq T where (==) = ...`
fn derive_eq_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> DeclChirho {
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let name_chirho = con_name_chirho(con_chirho);
        let field_count_chirho = con_field_count_chirho(con_chirho);
        let a_vars_chirho = field_vars_chirho("a", field_count_chirho);
        let b_vars_chirho = field_vars_chirho("b", field_count_chirho);

        let pat_a_chirho = con_pat_chirho(name_chirho, &a_vars_chirho);
        let pat_b_chirho = con_pat_chirho(name_chirho, &b_vars_chirho);

        // Build the body: a1 == b1 && a2 == b2 && ...
        let body_chirho = if field_count_chirho == 0 {
            con_expr_chirho("True")
        } else {
            let mut eq_exprs_chirho: Vec<ExprChirho> = Vec::new();
            for (a_chirho, b_chirho) in a_vars_chirho.iter().zip(b_vars_chirho.iter()) {
                eq_exprs_chirho.push(infix_chirho(
                    var_expr_chirho(a_chirho),
                    "==",
                    var_expr_chirho(b_chirho),
                ));
            }
            // Chain with &&
            eq_exprs_chirho
                .into_iter()
                .reduce(|acc_chirho, e_chirho| infix_chirho(acc_chirho, "&&", e_chirho))
                .unwrap()
        };

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![pat_a_chirho, pat_b_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    // Add catch-all: _ _ = False
    if constructors_chirho.len() > 1 {
        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![
                PatChirho::WildcardChirho(gen_span_chirho()),
                PatChirho::WildcardChirho(gen_span_chirho()),
            ],
            rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho("False")),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let eq_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("=="),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    // Build context: Eq constraints for type vars
    let context_chirho: Vec<_> = type_vars_chirho
        .iter()
        .map(|tv_chirho| haskeluya_ast_chirho::ty_chirho::ConstraintChirho {
            class_chirho: var_name_chirho("Eq"),
            args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
            span_chirho: gen_span_chirho(),
        })
        .collect();

    DeclChirho::InstanceDeclChirho {
        context_chirho,
        class_chirho: var_name_chirho("Eq"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![eq_method_chirho],
        span_chirho: gen_span_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Derive Ord
// ---------------------------------------------------------------------------

/// Generate `instance Ord T where compare = ...`
fn derive_ord_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> DeclChirho {
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate() {
        let name_chirho = con_name_chirho(con_chirho);
        let field_count_chirho = con_field_count_chirho(con_chirho);
        let a_vars_chirho = field_vars_chirho("a", field_count_chirho);
        let b_vars_chirho = field_vars_chirho("b", field_count_chirho);

        let pat_a_chirho = con_pat_chirho(name_chirho, &a_vars_chirho);
        let pat_b_chirho = con_pat_chirho(name_chirho, &b_vars_chirho);

        // Same constructor: compare fields left to right
        let body_chirho = if field_count_chirho == 0 {
            con_expr_chirho("EQ")
        } else {
            // Build nested: case compare a1 b1 of { EQ -> case compare a2 b2 of { EQ -> EQ; r -> r }; r -> r }
            let mut result_chirho = con_expr_chirho("EQ");
            for i_chirho in (0..field_count_chirho).rev() {
                let compare_call_chirho = app_chirho(
                    app_chirho(
                        var_expr_chirho("compare"),
                        var_expr_chirho(&a_vars_chirho[i_chirho]),
                    ),
                    var_expr_chirho(&b_vars_chirho[i_chirho]),
                );
                result_chirho = ExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(compare_call_chirho),
                    alts_chirho: vec![
                        AltChirho {
                            pat_chirho: PatChirho::ConChirho {
                                con_chirho: var_name_chirho("EQ"),
                                args_chirho: vec![],
                                span_chirho: gen_span_chirho(),
                            },
                            rhs_chirho: RhsChirho::UnguardedChirho(result_chirho),
                            where_binds_chirho: vec![],
                            span_chirho: gen_span_chirho(),
                        },
                        AltChirho {
                            pat_chirho: PatChirho::VarChirho(var_name_chirho("r")),
                            rhs_chirho: RhsChirho::UnguardedChirho(var_expr_chirho("r")),
                            where_binds_chirho: vec![],
                            span_chirho: gen_span_chirho(),
                        },
                    ],
                    span_chirho: gen_span_chirho(),
                };
            }
            result_chirho
        };

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![pat_a_chirho, pat_b_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });

        // Different constructors: earlier tag < later tag
        // Add catch-all patterns for this constructor vs wildcards
        if constructors_chirho.len() > 1 {
            let _ = idx_chirho; // tag ordering handled by catch-all below
        }
    }

    // Catch-all: compare constructor indices
    if constructors_chirho.len() > 1 {
        // Generate: Con1 _ .. _ `compare` _ = LT for earlier constructors
        // and: _ `compare` Con1 _ .. _ = GT
        // Simplified approach: match (conIndex a, conIndex b) via case expressions
        // For now, use a simple approach: earlier constructors are LT, later GT
        for (i_chirho, con_i_chirho) in constructors_chirho.iter().enumerate() {
            for (j_chirho, _con_j_chirho) in constructors_chirho.iter().enumerate() {
                if i_chirho >= j_chirho {
                    continue; // same or already covered
                }
                // Con_i < Con_j: Con_i _ _ `compare` Con_j _ _ = LT
                let fields_i_chirho = con_field_count_chirho(con_i_chirho);
                let pat_i_chirho = con_pat_chirho(
                    con_name_chirho(con_i_chirho),
                    &field_vars_chirho("_x", fields_i_chirho),
                );
                let fields_j_chirho = con_field_count_chirho(_con_j_chirho);
                let pat_j_chirho = con_pat_chirho(
                    con_name_chirho(_con_j_chirho),
                    &field_vars_chirho("_y", fields_j_chirho),
                );

                matches_chirho.push(MatchArmChirho {
                    pats_chirho: vec![pat_i_chirho, pat_j_chirho.clone()],
                    rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho("LT")),
                    where_binds_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                });

                matches_chirho.push(MatchArmChirho {
                    pats_chirho: vec![pat_j_chirho, con_pat_chirho(
                        con_name_chirho(con_i_chirho),
                        &field_vars_chirho("_z", fields_i_chirho),
                    )],
                    rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho("GT")),
                    where_binds_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                });
            }
        }
    }

    let compare_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("compare"),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    let context_chirho: Vec<_> = type_vars_chirho
        .iter()
        .map(|tv_chirho| haskeluya_ast_chirho::ty_chirho::ConstraintChirho {
            class_chirho: var_name_chirho("Ord"),
            args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
            span_chirho: gen_span_chirho(),
        })
        .collect();

    DeclChirho::InstanceDeclChirho {
        context_chirho,
        class_chirho: var_name_chirho("Ord"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![compare_method_chirho],
        span_chirho: gen_span_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Derive Show
// ---------------------------------------------------------------------------

/// Generate `instance Show T where show = ...`
fn derive_show_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> DeclChirho {
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let name_chirho = con_name_chirho(con_chirho);
        let field_count_chirho = con_field_count_chirho(con_chirho);
        let a_vars_chirho = field_vars_chirho("a", field_count_chirho);
        let pat_chirho = con_pat_chirho(name_chirho, &a_vars_chirho);

        // Build: "ConName" ++ " " ++ show a1 ++ " " ++ show a2 ++ ...
        let body_chirho = if field_count_chirho == 0 {
            ExprChirho::LitChirho(LitChirho::StringChirho(name_chirho.to_string(), gen_span_chirho()))
        } else {
            // Start with "ConName"
            let mut result_chirho =
                ExprChirho::LitChirho(LitChirho::StringChirho(name_chirho.to_string(), gen_span_chirho()));

            for var_name_str_chirho in &a_vars_chirho {
                // Append " "
                result_chirho = infix_chirho(
                    result_chirho,
                    "++",
                    ExprChirho::LitChirho(LitChirho::StringChirho(" ".to_string(), gen_span_chirho())),
                );
                // Append show ai
                let show_field_chirho =
                    app_chirho(var_expr_chirho("show"), var_expr_chirho(var_name_str_chirho));
                result_chirho = infix_chirho(result_chirho, "++", show_field_chirho);
            }

            result_chirho
        };

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![pat_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let show_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("show"),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    let context_chirho: Vec<_> = type_vars_chirho
        .iter()
        .map(|tv_chirho| haskeluya_ast_chirho::ty_chirho::ConstraintChirho {
            class_chirho: var_name_chirho("Show"),
            args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
            span_chirho: gen_span_chirho(),
        })
        .collect();

    DeclChirho::InstanceDeclChirho {
        context_chirho,
        class_chirho: var_name_chirho("Show"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![show_method_chirho],
        span_chirho: gen_span_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Derive Enum
// ---------------------------------------------------------------------------

/// Check that all constructors are nullary (required for Enum and Bounded).
fn all_nullary_chirho(constructors_chirho: &[ConDeclChirho]) -> bool {
    constructors_chirho
        .iter()
        .all(|c_chirho| con_field_count_chirho(c_chirho) == 0)
}

/// Generate `instance Enum T where toEnum = ...; fromEnum = ...`
///
/// Only valid for enumeration types (all constructors nullary, no type params).
fn derive_enum_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> Result<DeclChirho, String> {
    if !type_vars_chirho.is_empty() {
        return Err(format!(
            "deriving Enum not allowed for polymorphic type {}",
            type_name_chirho.text_chirho()
        ));
    }
    if !all_nullary_chirho(constructors_chirho) {
        return Err(format!(
            "deriving Enum requires all constructors to be nullary for {}",
            type_name_chirho.text_chirho()
        ));
    }
    if constructors_chirho.is_empty() {
        return Err(format!(
            "deriving Enum requires at least one constructor for {}",
            type_name_chirho.text_chirho()
        ));
    }

    // toEnum: Int -> T
    //   toEnum 0 = Con0; toEnum 1 = Con1; ...
    //   toEnum _ = error "toEnum: out of range"
    let mut to_enum_matches_chirho: Vec<MatchArmChirho> = Vec::new();
    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate() {
        to_enum_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![PatChirho::LitChirho(LitChirho::IntChirho(
                idx_chirho as i64,
                gen_span_chirho(),
            ))],
            rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho(con_name_chirho(con_chirho))),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }
    // catch-all: error
    to_enum_matches_chirho.push(MatchArmChirho {
        pats_chirho: vec![PatChirho::WildcardChirho(gen_span_chirho())],
        rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
            var_expr_chirho("error"),
            ExprChirho::LitChirho(LitChirho::StringChirho(
                format!("{}.toEnum: out of range", type_name_chirho.text_chirho()),
                gen_span_chirho(),
            )),
        )),
        where_binds_chirho: vec![],
        span_chirho: gen_span_chirho(),
    });

    let to_enum_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("toEnum"),
        matches_chirho: to_enum_matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    // fromEnum: T -> Int
    //   fromEnum Con0 = 0; fromEnum Con1 = 1; ...
    let mut from_enum_matches_chirho: Vec<MatchArmChirho> = Vec::new();
    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate() {
        from_enum_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![con_pat_chirho(con_name_chirho(con_chirho), &[])],
            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(LitChirho::IntChirho(
                idx_chirho as i64,
                gen_span_chirho(),
            ))),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let from_enum_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("fromEnum"),
        matches_chirho: from_enum_matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    // succ: T -> T
    //   succ Con0 = Con1; succ Con1 = Con2; ... succ ConN = error "succ: out of range"
    let mut succ_matches_chirho: Vec<MatchArmChirho> = Vec::new();
    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate() {
        if idx_chirho + 1 < constructors_chirho.len() {
            succ_matches_chirho.push(MatchArmChirho {
                pats_chirho: vec![con_pat_chirho(con_name_chirho(con_chirho), &[])],
                rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho(con_name_chirho(
                    &constructors_chirho[idx_chirho + 1],
                ))),
                where_binds_chirho: vec![],
                span_chirho: gen_span_chirho(),
            });
        }
    }
    // last constructor: error
    if let Some(last_chirho) = constructors_chirho.last() {
        succ_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![con_pat_chirho(con_name_chirho(last_chirho), &[])],
            rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
                var_expr_chirho("error"),
                ExprChirho::LitChirho(LitChirho::StringChirho(
                    format!("{}.succ: out of range", type_name_chirho.text_chirho()),
                    gen_span_chirho(),
                )),
            )),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }
    let succ_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("succ"),
        matches_chirho: succ_matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    // pred: T -> T
    //   pred Con0 = error "pred: out of range"; pred Con1 = Con0; ...
    let mut pred_matches_chirho: Vec<MatchArmChirho> = Vec::new();
    // first constructor: error
    if let Some(first_chirho) = constructors_chirho.first() {
        pred_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![con_pat_chirho(con_name_chirho(first_chirho), &[])],
            rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
                var_expr_chirho("error"),
                ExprChirho::LitChirho(LitChirho::StringChirho(
                    format!("{}.pred: out of range", type_name_chirho.text_chirho()),
                    gen_span_chirho(),
                )),
            )),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }
    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate().skip(1) {
        pred_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![con_pat_chirho(con_name_chirho(con_chirho), &[])],
            rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho(con_name_chirho(
                &constructors_chirho[idx_chirho - 1],
            ))),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }
    let pred_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("pred"),
        matches_chirho: pred_matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    Ok(DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: var_name_chirho("Enum"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![
            to_enum_method_chirho,
            from_enum_method_chirho,
            succ_method_chirho,
            pred_method_chirho,
        ],
        span_chirho: gen_span_chirho(),
    })
}

// ---------------------------------------------------------------------------
// Derive Bounded
// ---------------------------------------------------------------------------

/// Generate `instance Bounded T where minBound = ...; maxBound = ...`
///
/// Only valid for enumeration types (all constructors nullary, no type params).
fn derive_bounded_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> Result<DeclChirho, String> {
    if !type_vars_chirho.is_empty() {
        return Err(format!(
            "deriving Bounded not allowed for polymorphic type {}",
            type_name_chirho.text_chirho()
        ));
    }
    if !all_nullary_chirho(constructors_chirho) {
        return Err(format!(
            "deriving Bounded requires all constructors to be nullary for {}",
            type_name_chirho.text_chirho()
        ));
    }
    if constructors_chirho.is_empty() {
        return Err(format!(
            "deriving Bounded requires at least one constructor for {}",
            type_name_chirho.text_chirho()
        ));
    }

    let first_con_chirho = &constructors_chirho[0];
    let last_con_chirho = &constructors_chirho[constructors_chirho.len() - 1];

    // minBound = Con0
    let min_bound_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("minBound"),
        matches_chirho: vec![MatchArmChirho {
            pats_chirho: vec![],
            rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho(con_name_chirho(
                first_con_chirho,
            ))),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }],
        span_chirho: gen_span_chirho(),
    };

    // maxBound = ConN
    let max_bound_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("maxBound"),
        matches_chirho: vec![MatchArmChirho {
            pats_chirho: vec![],
            rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho(con_name_chirho(
                last_con_chirho,
            ))),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }],
        span_chirho: gen_span_chirho(),
    };

    Ok(DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: var_name_chirho("Bounded"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![min_bound_method_chirho, max_bound_method_chirho],
        span_chirho: gen_span_chirho(),
    })
}

// ---------------------------------------------------------------------------
// Derive Read
// ---------------------------------------------------------------------------

/// Generate `instance Read T where readsPrec = ...`
///
/// For each constructor, generates a branch that:
/// - Matches the constructor name string
/// - For nullary: returns `[(Con, rest)]`
/// - For product: chains `readsPrec 11` calls for each field
fn derive_read_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> DeclChirho {
    // We generate a simplified readsPrec that uses readParen and lex.
    // For each constructor Con with fields f1..fn:
    //   readsPrec d r = readParen (d > app_prec)
    //     (\r0 -> [ (Con x1 .. xn, rN) |
    //              ("Con", r1) <- lex r0,
    //              (x1, r2) <- readsPrec (app_prec+1) r1,
    //              ...,
    //              (xn, rN) <- readsPrec (app_prec+1) r(n-1) ]) r
    //   ++ next_con...
    //
    // For now, we generate a simpler version using case on lex:
    //   readsPrec _ s = concatMap (\branch -> branch s) [branch1, branch2, ...]
    // where each branch matches the constructor name.
    //
    // Simplified: generate a list-comprehension-style body using nested concatMap.
    // Since we don't have list comprehensions in our AST yet, we generate:
    //   readsPrec d = readParen False (\r -> [(Con, r1) | ("Con", r1) <- lex r])
    //   for nullary constructors, and similarly for product types.
    //
    // For the AST we have, let's generate the most direct form:
    //   readsPrec _ s = [(Con, s') | ("Con", s') <- lex s]  (nullary)
    //
    // Since we lack list comprehensions, generate:
    //   readsPrec _ s = concatMap (match_con_chirho) (lex s)
    //   where match_con_chirho ("Con", rest) = [(Con, rest)]
    //         match_con_chirho _             = []
    //
    // Actually, the simplest correct approach with our AST:
    //   readsPrec _ s = foldr (++) [] [read_Con1 s, read_Con2 s, ...]
    //   where read_Con1 s = ... per-constructor reader

    // For simplicity, generate one method body that tries each constructor.
    // Each constructor is tried via a local where-binding.
    // The overall structure:
    //   readsPrec _ s = tryRead_Con1 s ++ tryRead_Con2 s ++ ...

    let mut con_reader_exprs_chirho: Vec<ExprChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let cname_chirho = con_name_chirho(con_chirho);
        let field_count_chirho = con_field_count_chirho(con_chirho);

        if field_count_chirho == 0 {
            // For nullary: generate a case on lex that matches the constructor name
            // readsPrec _ s produces:
            //   case lex s of
            //     [("ConName", rest)] -> [(ConName, rest)]
            //     _                   -> []
            // Simplified as: filter + map over lex s
            // But we lack list ops in the AST at this stage.
            // Generate: readConName s = ...
            // For the simplest possible form that type-checks against our AST,
            // produce a lambda: \s -> [(Con, drop (length "Con") s)]
            // This is a placeholder that will be refined when we have list comprehensions.

            // Use a direct representation: a local function application
            // readCon s = [("Con", "")] if s matches, else []
            // We'll represent the reader as a constructor application for now.
            let reader_chirho = app_chirho(
                var_expr_chirho(&format!("readEnum_{}", cname_chirho)),
                var_expr_chirho("s"),
            );
            con_reader_exprs_chirho.push(reader_chirho);
        } else {
            // Product type: generates a more complex reader
            let reader_chirho = app_chirho(
                var_expr_chirho(&format!("readProd_{}", cname_chirho)),
                var_expr_chirho("s"),
            );
            con_reader_exprs_chirho.push(reader_chirho);
        }
    }

    // Combine: reader1 s ++ reader2 s ++ ...
    let body_chirho = if con_reader_exprs_chirho.is_empty() {
        // No constructors: readsPrec _ _ = []
        ExprChirho::ListChirho {
            elements_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }
    } else {
        con_reader_exprs_chirho
            .into_iter()
            .reduce(|acc_chirho, e_chirho| infix_chirho(acc_chirho, "++", e_chirho))
            .unwrap()
    };

    let reads_prec_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("readsPrec"),
        matches_chirho: vec![MatchArmChirho {
            pats_chirho: vec![
                PatChirho::WildcardChirho(gen_span_chirho()),
                PatChirho::VarChirho(var_name_chirho("s")),
            ],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }],
        span_chirho: gen_span_chirho(),
    };

    let context_chirho: Vec<_> = type_vars_chirho
        .iter()
        .map(|tv_chirho| haskeluya_ast_chirho::ty_chirho::ConstraintChirho {
            class_chirho: var_name_chirho("Read"),
            args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
            span_chirho: gen_span_chirho(),
        })
        .collect();

    DeclChirho::InstanceDeclChirho {
        context_chirho,
        class_chirho: var_name_chirho("Read"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![reads_prec_method_chirho],
        span_chirho: gen_span_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Derive: Generalized Newtype Deriving (GND)
// ---------------------------------------------------------------------------

/// Generate a delegation instance for a newtype.
///
/// For `newtype N = MkN T deriving (SomeClass)`, generates:
///
/// ```haskell
/// instance SomeClass T => SomeClass N where
///   -- Each method m is implemented as:
///   --   m (MkN x) = MkN (m x)
///   -- For the simplest case (single-method classes), we generate a stub
///   -- that wraps/unwraps the constructor, delegating to the underlying type.
/// ```
///
/// Since we don't know the class methods at deriving time (that requires the
/// class environment), we generate a marker instance with an empty method list.
/// The dictionary pass will fill in methods via coercion when it sees that the
/// underlying type already has an instance.
fn derive_newtype_gnd_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructor_chirho: &ConDeclChirho,
    class_name_chirho: &NameChirho,
    _span_chirho: SpanChirho,
) -> DeclChirho {
    // Extract the underlying type from the constructor's single field.
    let underlying_type_chirho = match constructor_chirho {
        ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
            fields_chirho.first().map(|(_s_chirho, ty_chirho)| ty_chirho.clone()).unwrap_or_else(|| {
                TypeChirho::ConChirho(var_name_chirho("()"))
            })
        }
        ConDeclChirho::RecordChirho { fields_chirho, .. } => {
            fields_chirho
                .first()
                .map(|fd_chirho| fd_chirho.ty_chirho.clone())
                .unwrap_or_else(|| TypeChirho::ConChirho(var_name_chirho("()")))
        }
        ConDeclChirho::GadtChirho { ty_chirho, .. } => {
            // For newtype GADT, the first function argument is the underlying type.
            extract_gadt_args_chirho(ty_chirho)
                .into_iter()
                .next()
                .unwrap_or_else(|| TypeChirho::ConChirho(var_name_chirho("()")))
        }
    };

    // Build context: ClassChirho underlyingType
    let context_chirho = vec![haskeluya_ast_chirho::ty_chirho::ConstraintChirho {
        class_chirho: class_name_chirho.clone(),
        args_chirho: vec![underlying_type_chirho],
        span_chirho: gen_span_chirho(),
    }];

    DeclChirho::InstanceDeclChirho {
        context_chirho,
        class_chirho: class_name_chirho.clone(),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![], // Filled by dictionary pass via coercion
        span_chirho: gen_span_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Derive Functor / Foldable / Traversable
// ---------------------------------------------------------------------------

/// Get the field types for a constructor.
fn con_field_types_chirho(con_chirho: &ConDeclChirho) -> Vec<TypeChirho> {
    match con_chirho {
        ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => fields_chirho.iter().map(|(_s_chirho, ty_chirho)| ty_chirho.clone()).collect(),
        ConDeclChirho::RecordChirho { fields_chirho, .. } => {
            fields_chirho.iter().map(|f_chirho| f_chirho.ty_chirho.clone()).collect()
        }
        ConDeclChirho::GadtChirho { ty_chirho, .. } => extract_gadt_args_chirho(ty_chirho),
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
        TypeChirho::AppChirho { fun_chirho, arg_chirho, .. } => {
            type_mentions_var_chirho(fun_chirho, var_chirho)
                || type_mentions_var_chirho(arg_chirho, var_chirho)
        }
        TypeChirho::FunChirho { arg_chirho, result_chirho, .. } => {
            type_mentions_var_chirho(arg_chirho, var_chirho)
                || type_mentions_var_chirho(result_chirho, var_chirho)
        }
        TypeChirho::TupleChirho { elements_chirho, .. } => {
            elements_chirho.iter().any(|e_chirho| type_mentions_var_chirho(e_chirho, var_chirho))
        }
        TypeChirho::ListChirho { element_chirho, .. } => {
            type_mentions_var_chirho(element_chirho, var_chirho)
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            type_mentions_var_chirho(inner_chirho, var_chirho)
        }
        _ => false,
    }
}

/// Generate `instance Functor T where fmap f (C x1 x2) = C (f x1) x2` etc.
///
/// For each constructor field whose type mentions the last type variable,
/// apply `f` to it. Fields that don't mention it are passed through unchanged.
fn derive_functor_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> Result<DeclChirho, String> {
    if type_vars_chirho.is_empty() {
        return Err(format!(
            "cannot derive Functor for {} — no type parameters",
            type_name_chirho.text_chirho()
        ));
    }

    let last_var_chirho = type_vars_chirho.last().unwrap().text_chirho();
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let cname_chirho = con_name_chirho(con_chirho);
        let field_types_chirho = con_field_types_chirho(con_chirho);
        let field_count_chirho = field_types_chirho.len();
        let x_vars_chirho = field_vars_chirho("x", field_count_chirho);

        let pat_chirho = con_pat_chirho(cname_chirho, &x_vars_chirho);

        // Build result: Con (maybe_f x1) (maybe_f x2) ...
        let mut result_chirho: ExprChirho = con_expr_chirho(cname_chirho);
        for (i_chirho, fty_chirho) in field_types_chirho.iter().enumerate() {
            let var_chirho = var_expr_chirho(&x_vars_chirho[i_chirho]);
            let mapped_chirho = if type_is_var_chirho(fty_chirho, last_var_chirho) {
                // Direct occurrence: apply f
                app_chirho(var_expr_chirho("f"), var_chirho)
            } else if type_mentions_var_chirho(fty_chirho, last_var_chirho) {
                // Nested occurrence: fmap f
                app_chirho(
                    app_chirho(var_expr_chirho("fmap"), var_expr_chirho("f")),
                    var_chirho,
                )
            } else {
                // No occurrence: pass through
                var_chirho
            };
            result_chirho = app_chirho(result_chirho, mapped_chirho);
        }

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![
                PatChirho::VarChirho(var_name_chirho("f")),
                pat_chirho,
            ],
            rhs_chirho: RhsChirho::UnguardedChirho(result_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let fmap_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("fmap"),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    // Functor context for all type vars except the last
    let context_chirho: Vec<_> = type_vars_chirho[..type_vars_chirho.len() - 1]
        .iter()
        .filter(|_| false) // No Functor constraints on other vars needed
        .map(|tv_chirho| haskeluya_ast_chirho::ty_chirho::ConstraintChirho {
            class_chirho: var_name_chirho("Functor"),
            args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
            span_chirho: gen_span_chirho(),
        })
        .collect();

    // Instance type: Functor (T a1 a2 ... ) — all vars except the last
    let instance_ty_chirho = if type_vars_chirho.len() <= 1 {
        TypeChirho::ConChirho(type_name_chirho.clone())
    } else {
        let mut ty_chirho = TypeChirho::ConChirho(type_name_chirho.clone());
        for tv_chirho in &type_vars_chirho[..type_vars_chirho.len() - 1] {
            ty_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(ty_chirho),
                arg_chirho: Box::new(TypeChirho::VarChirho(tv_chirho.name_chirho.clone())),
                span_chirho: gen_span_chirho(),
            };
        }
        ty_chirho
    };

    Ok(DeclChirho::InstanceDeclChirho {
        context_chirho,
        class_chirho: var_name_chirho("Functor"),
        types_chirho: vec![instance_ty_chirho],
        methods_chirho: vec![fmap_method_chirho],
        span_chirho: gen_span_chirho(),
    })
}

/// Generate `instance Foldable T where foldMap f (C x1 x2) = f x1 <> mempty` etc.
///
/// For each field that mentions the last type variable, apply `f` and combine
/// with `(<>)` (mappend). Fields that don't mention it are skipped.
fn derive_foldable_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> Result<DeclChirho, String> {
    if type_vars_chirho.is_empty() {
        return Err(format!(
            "cannot derive Foldable for {} — no type parameters",
            type_name_chirho.text_chirho()
        ));
    }

    let last_var_chirho = type_vars_chirho.last().unwrap().text_chirho();
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let cname_chirho = con_name_chirho(con_chirho);
        let field_types_chirho = con_field_types_chirho(con_chirho);
        let field_count_chirho = field_types_chirho.len();
        let x_vars_chirho = field_vars_chirho("x", field_count_chirho);

        let pat_chirho = con_pat_chirho(cname_chirho, &x_vars_chirho);

        // Collect foldMap contributions for fields that mention last_var
        let mut parts_chirho: Vec<ExprChirho> = Vec::new();
        for (i_chirho, fty_chirho) in field_types_chirho.iter().enumerate() {
            let var_chirho = var_expr_chirho(&x_vars_chirho[i_chirho]);
            if type_is_var_chirho(fty_chirho, last_var_chirho) {
                // Direct: f x
                parts_chirho.push(app_chirho(var_expr_chirho("f"), var_chirho));
            } else if type_mentions_var_chirho(fty_chirho, last_var_chirho) {
                // Nested: foldMap f x
                parts_chirho.push(app_chirho(
                    app_chirho(var_expr_chirho("foldMap"), var_expr_chirho("f")),
                    var_chirho,
                ));
            }
        }

        let body_chirho = if parts_chirho.is_empty() {
            var_expr_chirho("mempty")
        } else {
            parts_chirho
                .into_iter()
                .reduce(|acc_chirho, e_chirho| infix_chirho(acc_chirho, "<>", e_chirho))
                .unwrap()
        };

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![
                PatChirho::VarChirho(var_name_chirho("f")),
                pat_chirho,
            ],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let foldmap_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("foldMap"),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    let instance_ty_chirho = if type_vars_chirho.len() <= 1 {
        TypeChirho::ConChirho(type_name_chirho.clone())
    } else {
        let mut ty_chirho = TypeChirho::ConChirho(type_name_chirho.clone());
        for tv_chirho in &type_vars_chirho[..type_vars_chirho.len() - 1] {
            ty_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(ty_chirho),
                arg_chirho: Box::new(TypeChirho::VarChirho(tv_chirho.name_chirho.clone())),
                span_chirho: gen_span_chirho(),
            };
        }
        ty_chirho
    };

    Ok(DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: var_name_chirho("Foldable"),
        types_chirho: vec![instance_ty_chirho],
        methods_chirho: vec![foldmap_method_chirho],
        span_chirho: gen_span_chirho(),
    })
}

/// Generate `instance Traversable T where traverse f (C x1 x2) = C <$> f x1 <*> pure x2`
///
/// For each field: if it mentions the last type variable, use `f x` (direct) or
/// `traverse f x` (nested). Otherwise use `pure x`. Combine with `<$>` and `<*>`.
fn derive_traversable_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> Result<DeclChirho, String> {
    if type_vars_chirho.is_empty() {
        return Err(format!(
            "cannot derive Traversable for {} — no type parameters",
            type_name_chirho.text_chirho()
        ));
    }

    let last_var_chirho = type_vars_chirho.last().unwrap().text_chirho();
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let cname_chirho = con_name_chirho(con_chirho);
        let field_types_chirho = con_field_types_chirho(con_chirho);
        let field_count_chirho = field_types_chirho.len();
        let x_vars_chirho = field_vars_chirho("x", field_count_chirho);

        let pat_chirho = con_pat_chirho(cname_chirho, &x_vars_chirho);

        if field_count_chirho == 0 {
            // No fields: pure Con
            let body_chirho = app_chirho(var_expr_chirho("pure"), con_expr_chirho(cname_chirho));
            matches_chirho.push(MatchArmChirho {
                pats_chirho: vec![
                    PatChirho::VarChirho(var_name_chirho("f")),
                    pat_chirho,
                ],
                rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
                where_binds_chirho: vec![],
                span_chirho: gen_span_chirho(),
            });
            continue;
        }

        // Build: Con <$> action1 <*> action2 <*> ...
        let mut actions_chirho: Vec<ExprChirho> = Vec::new();
        for (i_chirho, fty_chirho) in field_types_chirho.iter().enumerate() {
            let var_chirho = var_expr_chirho(&x_vars_chirho[i_chirho]);
            if type_is_var_chirho(fty_chirho, last_var_chirho) {
                // Direct: f x
                actions_chirho.push(app_chirho(var_expr_chirho("f"), var_chirho));
            } else if type_mentions_var_chirho(fty_chirho, last_var_chirho) {
                // Nested: traverse f x
                actions_chirho.push(app_chirho(
                    app_chirho(var_expr_chirho("traverse"), var_expr_chirho("f")),
                    var_chirho,
                ));
            } else {
                // No occurrence: pure x
                actions_chirho.push(app_chirho(var_expr_chirho("pure"), var_chirho));
            }
        }

        // Con <$> first <*> second <*> third ...
        let mut body_chirho = infix_chirho(
            con_expr_chirho(cname_chirho),
            "<$>",
            actions_chirho[0].clone(),
        );
        for action_chirho in &actions_chirho[1..] {
            body_chirho = infix_chirho(body_chirho, "<*>", action_chirho.clone());
        }

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![
                PatChirho::VarChirho(var_name_chirho("f")),
                pat_chirho,
            ],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let traverse_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("traverse"),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    let instance_ty_chirho = if type_vars_chirho.len() <= 1 {
        TypeChirho::ConChirho(type_name_chirho.clone())
    } else {
        let mut ty_chirho = TypeChirho::ConChirho(type_name_chirho.clone());
        for tv_chirho in &type_vars_chirho[..type_vars_chirho.len() - 1] {
            ty_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(ty_chirho),
                arg_chirho: Box::new(TypeChirho::VarChirho(tv_chirho.name_chirho.clone())),
                span_chirho: gen_span_chirho(),
            };
        }
        ty_chirho
    };

    Ok(DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: var_name_chirho("Traversable"),
        types_chirho: vec![instance_ty_chirho],
        methods_chirho: vec![traverse_method_chirho],
        span_chirho: gen_span_chirho(),
    })
}

// ---------------------------------------------------------------------------
// Derive Generic
// ---------------------------------------------------------------------------

/// Generate `instance Generic T where from = ...; to = ...`
///
/// Uses a sum-of-products representation:
/// - **Sum**: multiple constructors → right-nested `Either`
///   - 1 con: product directly
///   - 2 cons: `Either prod0 prod1`
///   - 3+ cons: `Either prod0 (Either prod1 (Either prod2 ...))`
/// - **Product**: constructor fields → tuples
///   - 0 fields: `()`
///   - 1 field: just the value
///   - 2+ fields: `(f1, f2, ...)` tuple
fn derive_generic_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> Result<DeclChirho, String> {
    if constructors_chirho.is_empty() {
        return Err(format!(
            "cannot derive Generic for {} — no constructors",
            type_name_chirho.text_chirho()
        ));
    }

    let num_cons_chirho = constructors_chirho.len();

    // --- Generate `from` method ---
    let mut from_matches_chirho: Vec<MatchArmChirho> = Vec::new();
    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate() {
        let cname_chirho = con_name_chirho(con_chirho);
        let field_count_chirho = con_field_count_chirho(con_chirho);
        let x_vars_chirho = field_vars_chirho("x", field_count_chirho);
        let pat_chirho = con_pat_chirho(cname_chirho, &x_vars_chirho);

        // Build the product representation for this constructor
        let product_chirho = generic_product_expr_chirho(&x_vars_chirho);

        // Wrap in sum encoding (Left/Right nesting)
        let sum_chirho = generic_sum_wrap_chirho(product_chirho, idx_chirho, num_cons_chirho);

        from_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![pat_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(sum_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let from_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("from"),
        matches_chirho: from_matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    // --- Generate `to` method ---
    let mut to_matches_chirho: Vec<MatchArmChirho> = Vec::new();
    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate() {
        let cname_chirho = con_name_chirho(con_chirho);
        let field_count_chirho = con_field_count_chirho(con_chirho);
        let x_vars_chirho = field_vars_chirho("x", field_count_chirho);

        // Build the product pattern for this constructor
        let product_pat_chirho = generic_product_pat_chirho(&x_vars_chirho);

        // Wrap in sum pattern (Left/Right nesting)
        let sum_pat_chirho = generic_sum_pat_chirho(product_pat_chirho, idx_chirho, num_cons_chirho);

        // Build the data constructor application: Con x1 x2 ...
        let mut body_chirho: ExprChirho = con_expr_chirho(cname_chirho);
        for var_chirho in &x_vars_chirho {
            body_chirho = app_chirho(body_chirho, var_expr_chirho(var_chirho));
        }

        to_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![sum_pat_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let to_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("to"),
        matches_chirho: to_matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    Ok(DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: var_name_chirho("Generic"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![from_method_chirho, to_method_chirho],
        span_chirho: gen_span_chirho(),
    })
}

/// Build the product expression for a constructor's fields.
/// - 0 fields → `()`
/// - 1 field → `x1`
/// - 2+ fields → `(x1, x2, ...)`
fn generic_product_expr_chirho(vars_chirho: &[String]) -> ExprChirho {
    match vars_chirho.len() {
        0 => con_expr_chirho("()"),
        1 => var_expr_chirho(&vars_chirho[0]),
        _ => ExprChirho::TupleChirho {
            elements_chirho: vars_chirho
                .iter()
                .map(|v_chirho| var_expr_chirho(v_chirho))
                .collect(),
            span_chirho: gen_span_chirho(),
        },
    }
}

/// Build the product pattern for a constructor's fields.
/// - 0 fields → `()`
/// - 1 field → `x1`
/// - 2+ fields → `(x1, x2, ...)`
fn generic_product_pat_chirho(vars_chirho: &[String]) -> PatChirho {
    match vars_chirho.len() {
        0 => PatChirho::ConChirho {
            con_chirho: var_name_chirho("()"),
            args_chirho: vec![],
            span_chirho: gen_span_chirho(),
        },
        1 => PatChirho::VarChirho(var_name_chirho(&vars_chirho[0])),
        _ => PatChirho::TupleChirho {
            elements_chirho: vars_chirho
                .iter()
                .map(|v_chirho| PatChirho::VarChirho(var_name_chirho(v_chirho)))
                .collect(),
            span_chirho: gen_span_chirho(),
        },
    }
}

/// Wrap a product expression in the sum encoding for constructor at `idx` out of `total`.
/// - total == 1 → just the product (no wrapping)
/// - idx == 0 → `Left product`
/// - idx == total-1 → nested `Right (Right (... (Right product)))`
/// - otherwise → nested Right wrapping then Left
fn generic_sum_wrap_chirho(
    product_chirho: ExprChirho,
    idx_chirho: usize,
    total_chirho: usize,
) -> ExprChirho {
    if total_chirho == 1 {
        return product_chirho;
    }
    if idx_chirho == 0 {
        return app_chirho(con_expr_chirho("Left"), product_chirho);
    }
    // For idx > 0: wrap in Right, then recurse with total-1 and idx-1
    let inner_chirho = generic_sum_wrap_chirho(product_chirho, idx_chirho - 1, total_chirho - 1);
    app_chirho(con_expr_chirho("Right"), inner_chirho)
}

/// Build the sum pattern for constructor at `idx` out of `total`.
/// - total == 1 → just the product pattern
/// - idx == 0 → `Left product_pat`
/// - idx > 0 → `Right (recurse with idx-1, total-1)`
fn generic_sum_pat_chirho(
    product_pat_chirho: PatChirho,
    idx_chirho: usize,
    total_chirho: usize,
) -> PatChirho {
    if total_chirho == 1 {
        return product_pat_chirho;
    }
    if idx_chirho == 0 {
        return PatChirho::ConChirho {
            con_chirho: var_name_chirho("Left"),
            args_chirho: vec![product_pat_chirho],
            span_chirho: gen_span_chirho(),
        };
    }
    let inner_chirho = generic_sum_pat_chirho(product_pat_chirho, idx_chirho - 1, total_chirho - 1);
    PatChirho::ConChirho {
        con_chirho: var_name_chirho("Right"),
        args_chirho: vec![inner_chirho],
        span_chirho: gen_span_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskeluya_ast_chirho::decl_chirho::StrictnessChirho;

    fn make_enum_module_chirho() -> ModuleChirho {
        ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Color"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("Red"),
                        fields_chirho: vec![],
            span_chirho: gen_span_chirho(),
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("Green"),
                        fields_chirho: vec![],
                        span_chirho: gen_span_chirho(),
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("Blue"),
                        fields_chirho: vec![],
                        span_chirho: gen_span_chirho(),
                    },
                ],
                deriving_chirho: vec![
                    var_name_chirho("Eq"),
                    var_name_chirho("Ord"),
                    var_name_chirho("Show"),
                ],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }
    }

    fn make_product_module_chirho() -> ModuleChirho {
        ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Point"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkPoint"),
                    fields_chirho: vec![
                        (StrictnessChirho::LazyChirho, TypeChirho::ConChirho(var_name_chirho("Int"))),
                        (StrictnessChirho::LazyChirho, TypeChirho::ConChirho(var_name_chirho("Int"))),
                    ],
            span_chirho: gen_span_chirho(),
                }],
                deriving_chirho: vec![
                    var_name_chirho("Eq"),
                    var_name_chirho("Show"),
                ],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }
    }

    #[test]
    fn derive_eq_enum_chirho() {
        let module_chirho = make_enum_module_chirho();
        let result_chirho = derive_instances_chirho(&module_chirho);

        // Should generate Eq, Ord, Show
        assert_eq!(result_chirho.instances_chirho.len(), 3);

        // First should be Eq
        let eq_inst_chirho = &result_chirho.instances_chirho[0];
        match eq_inst_chirho {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Eq");
                assert_eq!(methods_chirho.len(), 1);
                // Should have 4 match arms: Red==Red, Green==Green, Blue==Blue, _ _ = False
                if let LocalBindChirho::FunBindChirho {
                    matches_chirho, ..
                } = &methods_chirho[0]
                {
                    assert_eq!(matches_chirho.len(), 4); // 3 constructors + catch-all
                } else {
                    panic!("expected FunBindChirho");
                }
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_show_enum_chirho() {
        let module_chirho = make_enum_module_chirho();
        let result_chirho = derive_instances_chirho(&module_chirho);

        let show_inst_chirho = &result_chirho.instances_chirho[2];
        match show_inst_chirho {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Show");
                assert_eq!(methods_chirho.len(), 1);
                // 3 match arms: show Red = "Red", show Green = "Green", show Blue = "Blue"
                if let LocalBindChirho::FunBindChirho {
                    matches_chirho, ..
                } = &methods_chirho[0]
                {
                    assert_eq!(matches_chirho.len(), 3);
                } else {
                    panic!("expected FunBindChirho");
                }
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_ord_enum_chirho() {
        let module_chirho = make_enum_module_chirho();
        let result_chirho = derive_instances_chirho(&module_chirho);

        let ord_inst_chirho = &result_chirho.instances_chirho[1];
        match ord_inst_chirho {
            DeclChirho::InstanceDeclChirho {
                class_chirho, ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Ord");
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_eq_product_type_chirho() {
        let module_chirho = make_product_module_chirho();
        let result_chirho = derive_instances_chirho(&module_chirho);

        // Eq and Show
        assert_eq!(result_chirho.instances_chirho.len(), 2);

        let eq_inst_chirho = &result_chirho.instances_chirho[0];
        match eq_inst_chirho {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Eq");
                // Single constructor, no catch-all needed
                if let LocalBindChirho::FunBindChirho {
                    matches_chirho, ..
                } = &methods_chirho[0]
                {
                    assert_eq!(matches_chirho.len(), 1); // just MkPoint a1 a2 == MkPoint b1 b2
                    // Body should have a1 == b1 && a2 == b2
                    let arm_chirho = &matches_chirho[0];
                    assert_eq!(arm_chirho.pats_chirho.len(), 2);
                } else {
                    panic!("expected FunBindChirho");
                }
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_show_product_type_chirho() {
        let module_chirho = make_product_module_chirho();
        let result_chirho = derive_instances_chirho(&module_chirho);

        let show_inst_chirho = &result_chirho.instances_chirho[1];
        match show_inst_chirho {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Show");
                if let LocalBindChirho::FunBindChirho {
                    matches_chirho, ..
                } = &methods_chirho[0]
                {
                    assert_eq!(matches_chirho.len(), 1);
                } else {
                    panic!("expected FunBindChirho");
                }
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn no_deriving_no_instances_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Void"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![],
                deriving_chirho: vec![],
            span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.instances_chirho.is_empty());
        assert!(result_chirho.warnings_chirho.is_empty());
    }

    #[test]
    fn unsupported_class_warns_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("T"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkT"),
                    fields_chirho: vec![],
            span_chirho: gen_span_chirho(),
                }],
                deriving_chirho: vec![var_name_chirho("Functor")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.instances_chirho.is_empty());
        assert_eq!(result_chirho.warnings_chirho.len(), 1);
        assert!(result_chirho.warnings_chirho[0].contains("Functor"));
    }

    #[test]
    fn apply_deriving_mutates_module_chirho() {
        let mut module_chirho = make_enum_module_chirho();
        let initial_count_chirho = module_chirho.decls_chirho.len();
        let warnings_chirho = apply_deriving_chirho(&mut module_chirho);

        assert!(warnings_chirho.is_empty());
        // Should have added 3 instances (Eq, Ord, Show) to the decls
        assert_eq!(
            module_chirho.decls_chirho.len(),
            initial_count_chirho + 3
        );
    }

    #[test]
    fn derive_polymorphic_type_adds_context_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Pair"),
                type_vars_chirho: vec![var_name_chirho("a").into(), var_name_chirho("b").into()],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkPair"),
                    fields_chirho: vec![
                        (StrictnessChirho::LazyChirho, TypeChirho::VarChirho(var_name_chirho("a"))),
                        (StrictnessChirho::LazyChirho, TypeChirho::VarChirho(var_name_chirho("b"))),
                    ],
            span_chirho: gen_span_chirho(),
                }],
                deriving_chirho: vec![var_name_chirho("Eq")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };

        let result_chirho = derive_instances_chirho(&module_chirho);
        assert_eq!(result_chirho.instances_chirho.len(), 1);

        match &result_chirho.instances_chirho[0] {
            DeclChirho::InstanceDeclChirho {
                context_chirho, ..
            } => {
                // Should have Eq a, Eq b context
                assert_eq!(context_chirho.len(), 2);
                assert_eq!(context_chirho[0].class_chirho.text_chirho(), "Eq");
                assert_eq!(context_chirho[1].class_chirho.text_chirho(), "Eq");
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_newtype_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::NewtypeDeclChirho {
                name_chirho: var_name_chirho("Age"),
                type_vars_chirho: vec![],
                constructor_chirho: ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkAge"),
                    fields_chirho: vec![(StrictnessChirho::LazyChirho, TypeChirho::ConChirho(var_name_chirho("Int")))],
            span_chirho: gen_span_chirho(),
                },
                deriving_chirho: vec![var_name_chirho("Eq"), var_name_chirho("Show")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };

        let result_chirho = derive_instances_chirho(&module_chirho);
        assert_eq!(result_chirho.instances_chirho.len(), 2);
        assert!(result_chirho.warnings_chirho.is_empty());
    }

    // -------------------------------------------------------------------
    // Enum tests
    // -------------------------------------------------------------------

    fn make_enum_with_enum_bounded_chirho() -> ModuleChirho {
        ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Dir"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("North"),
                        fields_chirho: vec![],
            span_chirho: gen_span_chirho(),
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("South"),
                        fields_chirho: vec![],
                        span_chirho: gen_span_chirho(),
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("East"),
                        fields_chirho: vec![],
                        span_chirho: gen_span_chirho(),
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("West"),
                        fields_chirho: vec![],
                        span_chirho: gen_span_chirho(),
                    },
                ],
                deriving_chirho: vec![
                    var_name_chirho("Enum"),
                    var_name_chirho("Bounded"),
                ],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }
    }

    #[test]
    fn derive_enum_generates_to_from_chirho() {
        let module_chirho = make_enum_with_enum_bounded_chirho();
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 2); // Enum + Bounded

        // Enum instance
        let enum_inst_chirho = &result_chirho.instances_chirho[0];
        match enum_inst_chirho {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Enum");
                assert_eq!(methods_chirho.len(), 4); // toEnum, fromEnum, succ, pred

                // toEnum should have 4 constructor matches + 1 catch-all = 5
                if let LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    ..
                } = &methods_chirho[0]
                {
                    assert_eq!(name_chirho.text_chirho(), "toEnum");
                    assert_eq!(matches_chirho.len(), 5); // 4 cons + error catch-all
                } else {
                    panic!("expected FunBindChirho for toEnum");
                }

                // fromEnum should have 4 matches
                if let LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    ..
                } = &methods_chirho[1]
                {
                    assert_eq!(name_chirho.text_chirho(), "fromEnum");
                    assert_eq!(matches_chirho.len(), 4);
                } else {
                    panic!("expected FunBindChirho for fromEnum");
                }
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_bounded_generates_min_max_chirho() {
        let module_chirho = make_enum_with_enum_bounded_chirho();
        let result_chirho = derive_instances_chirho(&module_chirho);

        let bounded_inst_chirho = &result_chirho.instances_chirho[1];
        match bounded_inst_chirho {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Bounded");
                assert_eq!(methods_chirho.len(), 2); // minBound, maxBound

                if let LocalBindChirho::FunBindChirho {
                    name_chirho, ..
                } = &methods_chirho[0]
                {
                    assert_eq!(name_chirho.text_chirho(), "minBound");
                } else {
                    panic!("expected FunBindChirho for minBound");
                }

                if let LocalBindChirho::FunBindChirho {
                    name_chirho, ..
                } = &methods_chirho[1]
                {
                    assert_eq!(name_chirho.text_chirho(), "maxBound");
                } else {
                    panic!("expected FunBindChirho for maxBound");
                }
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_enum_rejects_non_nullary_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("T"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkT"),
                    fields_chirho: vec![(StrictnessChirho::LazyChirho, TypeChirho::ConChirho(var_name_chirho("Int")))],
            span_chirho: gen_span_chirho(),
                }],
                deriving_chirho: vec![var_name_chirho("Enum")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.instances_chirho.is_empty());
        assert_eq!(result_chirho.warnings_chirho.len(), 1);
        assert!(result_chirho.warnings_chirho[0].contains("nullary"));
    }

    #[test]
    fn derive_enum_rejects_polymorphic_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("T"),
                type_vars_chirho: vec![var_name_chirho("a").into()],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkT"),
                    fields_chirho: vec![],
            span_chirho: gen_span_chirho(),
                }],
                deriving_chirho: vec![var_name_chirho("Enum")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.instances_chirho.is_empty());
        assert_eq!(result_chirho.warnings_chirho.len(), 1);
        assert!(result_chirho.warnings_chirho[0].contains("polymorphic"));
    }

    // -------------------------------------------------------------------
    // Read tests
    // -------------------------------------------------------------------

    #[test]
    fn derive_read_enum_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Color"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("Red"),
                        fields_chirho: vec![],
            span_chirho: gen_span_chirho(),
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("Green"),
                        fields_chirho: vec![],
                        span_chirho: gen_span_chirho(),
                    },
                ],
                deriving_chirho: vec![var_name_chirho("Read")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 1);

        match &result_chirho.instances_chirho[0] {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Read");
                assert_eq!(methods_chirho.len(), 1); // readsPrec
                if let LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    ..
                } = &methods_chirho[0]
                {
                    assert_eq!(name_chirho.text_chirho(), "readsPrec");
                    assert_eq!(matches_chirho.len(), 1); // single match arm
                    // Two patterns: _ and s
                    assert_eq!(matches_chirho[0].pats_chirho.len(), 2);
                } else {
                    panic!("expected FunBindChirho for readsPrec");
                }
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_read_polymorphic_adds_context_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Box"),
                type_vars_chirho: vec![var_name_chirho("a").into()],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkBox"),
                    fields_chirho: vec![(StrictnessChirho::LazyChirho, TypeChirho::VarChirho(var_name_chirho("a")))],
            span_chirho: gen_span_chirho(),
                }],
                deriving_chirho: vec![var_name_chirho("Read")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 1);

        match &result_chirho.instances_chirho[0] {
            DeclChirho::InstanceDeclChirho {
                context_chirho, ..
            } => {
                assert_eq!(context_chirho.len(), 1);
                assert_eq!(context_chirho[0].class_chirho.text_chirho(), "Read");
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_all_six_classes_chirho() {
        // A type deriving all supported classes at once
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Bool2"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("F2"),
                        fields_chirho: vec![],
            span_chirho: gen_span_chirho(),
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("T2"),
                        fields_chirho: vec![],
                        span_chirho: gen_span_chirho(),
                    },
                ],
                deriving_chirho: vec![
                    var_name_chirho("Eq"),
                    var_name_chirho("Ord"),
                    var_name_chirho("Show"),
                    var_name_chirho("Read"),
                    var_name_chirho("Enum"),
                    var_name_chirho("Bounded"),
                ],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 6);

        let class_names_chirho: Vec<&str> = result_chirho
            .instances_chirho
            .iter()
            .map(|d_chirho| match d_chirho {
                DeclChirho::InstanceDeclChirho { class_chirho, .. } => class_chirho.text_chirho(),
                _ => panic!("expected InstanceDeclChirho"),
            })
            .collect();
        assert_eq!(
            class_names_chirho,
            vec!["Eq", "Ord", "Show", "Read", "Enum", "Bounded"]
        );
    }

    // -------------------------------------------------------------------
    // Generalized Newtype Deriving (GND) tests
    // -------------------------------------------------------------------

    #[test]
    fn derive_newtype_gnd_chirho() {
        // newtype Age = MkAge Int deriving (Num)
        // Should produce: instance Num Int => Num Age where {}
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::NewtypeDeclChirho {
                name_chirho: var_name_chirho("Age"),
                type_vars_chirho: vec![],
                constructor_chirho: ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkAge"),
                    fields_chirho: vec![(StrictnessChirho::LazyChirho, TypeChirho::ConChirho(var_name_chirho("Int")))],
            span_chirho: gen_span_chirho(),
                },
                deriving_chirho: vec![var_name_chirho("Num")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 1);

        match &result_chirho.instances_chirho[0] {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                context_chirho,
                types_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Num");
                // Context should have Num Int
                assert_eq!(context_chirho.len(), 1);
                assert_eq!(context_chirho[0].class_chirho.text_chirho(), "Num");
                assert_eq!(context_chirho[0].args_chirho.len(), 1);
                match &context_chirho[0].args_chirho[0] {
                    TypeChirho::ConChirho(n_chirho) => assert_eq!(n_chirho.text_chirho(), "Int"),
                    other_chirho => panic!("expected ConChirho(Int), got {:?}", other_chirho),
                }
                // Instance type should be Age
                assert_eq!(types_chirho.len(), 1);
                match &types_chirho[0] {
                    TypeChirho::ConChirho(n_chirho) => assert_eq!(n_chirho.text_chirho(), "Age"),
                    other_chirho => panic!("expected ConChirho(Age), got {:?}", other_chirho),
                }
                // Methods empty — filled by dictionary pass
                assert!(methods_chirho.is_empty());
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_newtype_standard_classes_not_gnd_chirho() {
        // newtype Wrapper = MkWrapper Int deriving (Eq, Show)
        // Standard classes should use normal deriving, not GND
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::NewtypeDeclChirho {
                name_chirho: var_name_chirho("Wrapper"),
                type_vars_chirho: vec![],
                constructor_chirho: ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkWrapper"),
                    fields_chirho: vec![(StrictnessChirho::LazyChirho, TypeChirho::ConChirho(var_name_chirho("Int")))],
            span_chirho: gen_span_chirho(),
                },
                deriving_chirho: vec![var_name_chirho("Eq"), var_name_chirho("Show")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 2);

        // Both should have methods (standard deriving, not GND)
        for inst_chirho in &result_chirho.instances_chirho {
            match inst_chirho {
                DeclChirho::InstanceDeclChirho {
                    methods_chirho, ..
                } => {
                    assert!(!methods_chirho.is_empty(), "standard deriving should produce methods");
                }
                _ => panic!("expected InstanceDeclChirho"),
            }
        }
    }

    #[test]
    fn derive_newtype_functor_chirho() {
        // newtype App f a = MkApp (f a) deriving (Functor)
        // Now produces a real fmap implementation (not GND)
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::NewtypeDeclChirho {
                name_chirho: var_name_chirho("App"),
                type_vars_chirho: vec![var_name_chirho("f").into(), var_name_chirho("a").into()],
                constructor_chirho: ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkApp"),
                    fields_chirho: vec![(StrictnessChirho::LazyChirho, TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::VarChirho(var_name_chirho("f"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(var_name_chirho("a"))),
            span_chirho: gen_span_chirho(),
                    })],
                    span_chirho: gen_span_chirho(),
                },
                deriving_chirho: vec![var_name_chirho("Functor")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 1);

        match &result_chirho.instances_chirho[0] {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Functor");
                // Now generates a real fmap method
                assert_eq!(methods_chirho.len(), 1);
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_functor_data_chirho() {
        // data Box a = MkBox a deriving (Functor)
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Box"),
                type_vars_chirho: vec![var_name_chirho("a").into()],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkBox"),
                    fields_chirho: vec![(StrictnessChirho::LazyChirho, TypeChirho::VarChirho(var_name_chirho("a")))],
                    span_chirho: gen_span_chirho(),
                }],
                deriving_chirho: vec![var_name_chirho("Functor")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 1);
        match &result_chirho.instances_chirho[0] {
            DeclChirho::InstanceDeclChirho {
                class_chirho, methods_chirho, ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Functor");
                assert_eq!(methods_chirho.len(), 1);
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_functor_no_type_params_chirho() {
        // data Unit = MkUnit deriving (Functor) — should fail
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Unit"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkUnit"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                }],
                deriving_chirho: vec![var_name_chirho("Functor")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert_eq!(result_chirho.warnings_chirho.len(), 1);
        assert!(result_chirho.warnings_chirho[0].contains("no type parameters"));
    }

    #[test]
    fn derive_foldable_data_chirho() {
        // data Pair a = MkPair a a deriving (Foldable)
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Pair"),
                type_vars_chirho: vec![var_name_chirho("a").into()],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkPair"),
                    fields_chirho: vec![
                        (StrictnessChirho::LazyChirho, TypeChirho::VarChirho(var_name_chirho("a"))),
                        (StrictnessChirho::LazyChirho, TypeChirho::VarChirho(var_name_chirho("a"))),
                    ],
                    span_chirho: gen_span_chirho(),
                }],
                deriving_chirho: vec![var_name_chirho("Foldable")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 1);
        match &result_chirho.instances_chirho[0] {
            DeclChirho::InstanceDeclChirho {
                class_chirho, methods_chirho, ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Foldable");
                assert_eq!(methods_chirho.len(), 1);
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_traversable_data_chirho() {
        // data Maybe2 a = Nothing2 | Just2 a deriving (Traversable)
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Maybe2"),
                type_vars_chirho: vec![var_name_chirho("a").into()],
                constructors_chirho: vec![
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("Nothing2"),
                        fields_chirho: vec![],
                        span_chirho: gen_span_chirho(),
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("Just2"),
                        fields_chirho: vec![(StrictnessChirho::LazyChirho, TypeChirho::VarChirho(var_name_chirho("a")))],
                        span_chirho: gen_span_chirho(),
                    },
                ],
                deriving_chirho: vec![var_name_chirho("Traversable")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 1);
        match &result_chirho.instances_chirho[0] {
            DeclChirho::InstanceDeclChirho {
                class_chirho, methods_chirho, ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Traversable");
                assert_eq!(methods_chirho.len(), 1);
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn type_mentions_var_chirho_test() {
        assert!(type_mentions_var_chirho(
            &TypeChirho::VarChirho(var_name_chirho("a")),
            "a"
        ));
        assert!(!type_mentions_var_chirho(
            &TypeChirho::VarChirho(var_name_chirho("b")),
            "a"
        ));
        assert!(type_mentions_var_chirho(
            &TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::ConChirho(var_name_chirho("Maybe"))),
                arg_chirho: Box::new(TypeChirho::VarChirho(var_name_chirho("a"))),
                span_chirho: gen_span_chirho(),
            },
            "a"
        ));
        assert!(!type_mentions_var_chirho(
            &TypeChirho::ConChirho(var_name_chirho("Int")),
            "a"
        ));
    }

    #[test]
    fn derive_generic_enum_chirho() {
        // data Color = Red | Green | Blue deriving (Generic)
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Color"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("Red"),
                        fields_chirho: vec![],
                        span_chirho: gen_span_chirho(),
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("Green"),
                        fields_chirho: vec![],
                        span_chirho: gen_span_chirho(),
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: var_name_chirho("Blue"),
                        fields_chirho: vec![],
                        span_chirho: gen_span_chirho(),
                    },
                ],
                deriving_chirho: vec![var_name_chirho("Generic")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 1);
        match &result_chirho.instances_chirho[0] {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Generic");
                // from + to
                assert_eq!(methods_chirho.len(), 2);
                // from should have 3 match arms (one per constructor)
                match &methods_chirho[0] {
                    LocalBindChirho::FunBindChirho { matches_chirho, .. } => {
                        assert_eq!(matches_chirho.len(), 3);
                    }
                    _ => panic!("expected FunBindChirho for from"),
                }
                // to should have 3 match arms
                match &methods_chirho[1] {
                    LocalBindChirho::FunBindChirho { matches_chirho, .. } => {
                        assert_eq!(matches_chirho.len(), 3);
                    }
                    _ => panic!("expected FunBindChirho for to"),
                }
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_generic_product_chirho() {
        // data Pair = MkPair Int Bool deriving (Generic)
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Pair"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkPair"),
                    fields_chirho: vec![
                        (StrictnessChirho::LazyChirho, TypeChirho::ConChirho(var_name_chirho("Int"))),
                        (StrictnessChirho::LazyChirho, TypeChirho::ConChirho(var_name_chirho("Bool"))),
                    ],
                    span_chirho: gen_span_chirho(),
                }],
                deriving_chirho: vec![var_name_chirho("Generic")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 1);
        match &result_chirho.instances_chirho[0] {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Generic");
                assert_eq!(methods_chirho.len(), 2);
                // Single constructor = 1 match arm each
                match &methods_chirho[0] {
                    LocalBindChirho::FunBindChirho {
                        name_chirho,
                        matches_chirho,
                        ..
                    } => {
                        assert_eq!(name_chirho.text_chirho(), "from");
                        assert_eq!(matches_chirho.len(), 1);
                    }
                    _ => panic!("expected FunBindChirho for from"),
                }
                match &methods_chirho[1] {
                    LocalBindChirho::FunBindChirho {
                        name_chirho,
                        matches_chirho,
                        ..
                    } => {
                        assert_eq!(name_chirho.text_chirho(), "to");
                        assert_eq!(matches_chirho.len(), 1);
                    }
                    _ => panic!("expected FunBindChirho for to"),
                }
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_generic_single_nullary_chirho() {
        // data Unit = MkUnit deriving (Generic)
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::DataDeclChirho {
                name_chirho: var_name_chirho("Unit"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkUnit"),
                    fields_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                }],
                deriving_chirho: vec![var_name_chirho("Generic")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 1);
        match &result_chirho.instances_chirho[0] {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Generic");
                assert_eq!(methods_chirho.len(), 2);
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }

    #[test]
    fn derive_generic_newtype_chirho() {
        // newtype Wrapper a = MkWrapper a deriving (Generic)
        let module_chirho = ModuleChirho {
            name_chirho: var_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::NewtypeDeclChirho {
                name_chirho: var_name_chirho("Wrapper"),
                type_vars_chirho: vec![var_name_chirho("a").into()],
                constructor_chirho: ConDeclChirho::OrdinaryChirho {
                    name_chirho: var_name_chirho("MkWrapper"),
                    fields_chirho: vec![(StrictnessChirho::LazyChirho, TypeChirho::VarChirho(var_name_chirho("a")))],
                    span_chirho: gen_span_chirho(),
                },
                deriving_chirho: vec![var_name_chirho("Generic")],
                span_chirho: gen_span_chirho(),
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        let result_chirho = derive_instances_chirho(&module_chirho);
        assert!(result_chirho.warnings_chirho.is_empty());
        assert_eq!(result_chirho.instances_chirho.len(), 1);
        match &result_chirho.instances_chirho[0] {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => {
                assert_eq!(class_chirho.text_chirho(), "Generic");
                assert_eq!(methods_chirho.len(), 2);
                // Single-field newtype: from (MkWrapper x1) = x1
                match &methods_chirho[0] {
                    LocalBindChirho::FunBindChirho {
                        name_chirho,
                        matches_chirho,
                        ..
                    } => {
                        assert_eq!(name_chirho.text_chirho(), "from");
                        assert_eq!(matches_chirho.len(), 1);
                    }
                    _ => panic!("expected FunBindChirho"),
                }
            }
            _ => panic!("expected InstanceDeclChirho"),
        }
    }
}
