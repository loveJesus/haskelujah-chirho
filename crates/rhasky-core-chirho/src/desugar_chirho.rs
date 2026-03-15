// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # AST-to-Core desugaring
//!
//! Translates the surface AST into the Core IR. This pass:
//! - Converts multi-equation function bindings into case expressions
//! - Translates pattern matching into flat Core case expressions
//! - Converts if/where/guards into Core let/case
//! - Removes syntactic sugar (do notation, list comprehensions, etc.)

use std::collections::{HashMap, HashSet};

use rhasky_ast_chirho::decl_chirho::DeclChirho;
use rhasky_ast_chirho::expr_chirho::{ExprChirho, LocalBindChirho, MatchArmChirho, RhsChirho};
use rhasky_ast_chirho::lit_chirho::LitChirho;
use rhasky_ast_chirho::module_chirho::ModuleChirho;
use rhasky_ast_chirho::pat_chirho::PatChirho;
use rhasky_ast_chirho::ty_chirho::TypeChirho;
use rhasky_span_chirho::SpanChirho;
use rhasky_typing_chirho::ty_chirho::TyChirho;

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
};
use crate::simplify_chirho::free_vars_chirho;

/// Result of desugaring: the Core module plus a symbol table mapping
/// CoreIdChirho → source name for use by downstream passes.
#[derive(Debug)]
pub struct DesugarOutputChirho {
    pub module_chirho: CoreModuleChirho,
    /// Maps every CoreIdChirho to the source name it originated from.
    pub names_chirho: HashMap<CoreIdChirho, String>,
}

/// The desugaring context — generates fresh Core IDs.
pub struct DesugarCtxChirho {
    next_id_chirho: u32,
    /// Tracks CoreId → source name associations.
    names_chirho: HashMap<CoreIdChirho, String>,
    /// Scope: maps source name → CoreIdChirho for variables currently in scope.
    /// Newer entries shadow older ones (last-wins in the Vec of scopes).
    scope_chirho: Vec<HashMap<String, CoreIdChirho>>,
    /// Cache for free variable references: ensures each unscoped name gets
    /// the same CoreId across multiple references, so the dict pass can
    /// create a single binding that all references share.
    free_var_cache_chirho: HashMap<String, CoreIdChirho>,
    /// Active LANGUAGE extensions (e.g. "OverloadedStrings").
    extensions_chirho: Vec<String>,
}

impl DesugarCtxChirho {
    pub fn new_chirho() -> Self {
        Self {
            next_id_chirho: 0,
            names_chirho: HashMap::new(),
            scope_chirho: vec![HashMap::new()],
            free_var_cache_chirho: HashMap::new(),
            extensions_chirho: Vec::new(),
        }
    }

    /// Generate a fresh Core ID and record its name.
    fn fresh_id_chirho(&mut self, name_chirho: &str) -> CoreIdChirho {
        let id_chirho = CoreIdChirho(self.next_id_chirho);
        self.next_id_chirho += 1;
        self.names_chirho
            .insert(id_chirho, name_chirho.to_string());
        id_chirho
    }

    /// Look up a name in the current scope chain.
    fn lookup_scope_chirho(&self, name_chirho: &str) -> Option<CoreIdChirho> {
        for scope_chirho in self.scope_chirho.iter().rev() {
            if let Some(&id_chirho) = scope_chirho.get(name_chirho) {
                return Some(id_chirho);
            }
        }
        None
    }

    /// Bind a name in the current (innermost) scope.
    fn bind_in_scope_chirho(&mut self, name_chirho: &str, id_chirho: CoreIdChirho) {
        if let Some(scope_chirho) = self.scope_chirho.last_mut() {
            scope_chirho.insert(name_chirho.to_string(), id_chirho);
        }
    }

    /// Push a new scope level.
    fn push_scope_chirho(&mut self) {
        self.scope_chirho.push(HashMap::new());
    }

    /// Pop the innermost scope level.
    fn pop_scope_chirho(&mut self) {
        self.scope_chirho.pop();
    }

    /// Look up or create a CoreId for a variable reference.
    /// If the name is in scope (bound by a lambda, let, etc.), return its ID.
    /// Otherwise, check the free-variable cache so repeated references to the
    /// same unscoped name share a single CoreId.  If not cached, create a
    /// fresh ID and cache it.
    fn resolve_var_chirho(&mut self, name_chirho: &str) -> CoreIdChirho {
        if let Some(id_chirho) = self.lookup_scope_chirho(name_chirho) {
            id_chirho
        } else if let Some(&id_chirho) = self.free_var_cache_chirho.get(name_chirho) {
            id_chirho
        } else {
            let id_chirho = self.fresh_id_chirho(name_chirho);
            self.free_var_cache_chirho.insert(name_chirho.to_string(), id_chirho);
            id_chirho
        }
    }

    /// Build the body of an operator section, emitting PrimOp for
    /// known built-in operators so they work without dict-pass
    /// bindings. `left_chirho` is the first arg, `right_chirho` is
    /// the second arg (already in correct application order).
    fn build_section_body_chirho(
        &mut self,
        op_name_chirho: &str,
        left_chirho: CoreExprChirho,
        right_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        // Direct primop operators
        let primop_chirho = match op_name_chirho {
            "+" => Some("+#"),
            "-" => Some("-#"),
            "*" => Some("*#"),
            "==" => Some("==#"),
            "/=" => Some("/=#"),
            "<" => Some("<#"),
            "<=" => Some("<=#"),
            ">" => Some(">#"),
            ">=" => Some(">=#"),
            "++" => Some("++#"),
            "^" => Some("^#"),
            "**" => Some("**#"),
            _ => None,
        };
        if let Some(primop_name_chirho) = primop_chirho {
            return CoreExprChirho::PrimOpChirho {
                name_chirho: primop_name_chirho.to_string(),
                args_chirho: vec![left_chirho, right_chirho],
            };
        }
        // (:) cons constructor: left : right → ConApp(":", [left, right])
        if op_name_chirho == ":" {
            return CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![left_chirho, right_chirho],
            };
        }
        // Fallback: operator as variable reference
        let op_id_chirho = self.resolve_var_chirho(op_name_chirho);
        CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(op_id_chirho)),
                arg_chirho: Box::new(left_chirho),
            }),
            arg_chirho: Box::new(right_chirho),
        }
    }

    /// Create a binder with a fresh ID.
    fn fresh_binder_chirho(
        &mut self,
        name_chirho: &str,
        ty_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> BinderChirho {
        BinderChirho {
            id_chirho: self.fresh_id_chirho(name_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho,
            span_chirho,
        }
    }

    /// Check whether a binding group is (mutually) recursive.
    ///
    /// A binding group is recursive if any RHS references any binder
    /// defined in the same group.
    fn is_recursive_binds_chirho(
        binds_chirho: &[(BinderChirho, CoreExprChirho)],
    ) -> bool {
        let binder_ids_chirho: HashSet<CoreIdChirho> = binds_chirho
            .iter()
            .map(|(b_chirho, _)| b_chirho.id_chirho)
            .collect();
        for (_, rhs_chirho) in binds_chirho {
            let fvs_chirho = free_vars_chirho(rhs_chirho);
            if fvs_chirho.iter().any(|id_chirho| binder_ids_chirho.contains(id_chirho)) {
                return true;
            }
        }
        false
    }

    /// Derive the type key string from an AST `TypeChirho`, matching
    /// the format produced by `Display for TyChirho` in the typing layer
    /// so that the `$prim_` binding names align with `dict_chirho.rs`.
    pub fn type_key_from_ast_chirho(ty_chirho: &TypeChirho) -> String {
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) => name_chirho.text_chirho().to_string(),
            TypeChirho::VarChirho(name_chirho) => name_chirho.text_chirho().to_string(),
            TypeChirho::ListChirho { element_chirho, .. } => {
                format!("[{}]", Self::type_key_from_ast_chirho(element_chirho))
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let inner_chirho: Vec<String> = elements_chirho
                    .iter()
                    .map(|e_chirho| Self::type_key_from_ast_chirho(e_chirho))
                    .collect();
                format!("({})", inner_chirho.join(", "))
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                format!(
                    "{} {}",
                    Self::type_key_from_ast_chirho(fun_chirho),
                    Self::type_key_from_ast_chirho(arg_chirho)
                )
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                Self::type_key_from_ast_chirho(inner_chirho)
            }
            _ => "_".to_string(),
        }
    }

    /// Build a type key for an instance declaration from its AST types.
    /// For single-parameter classes: just the first type key.
    /// For multi-parameter classes: head_key + "_" + extra keys joined by "_".
    /// Matches the format used in `dict_chirho.rs::generate_instance_dicts_chirho`.
    pub fn instance_type_key_chirho(types_chirho: &[TypeChirho]) -> String {
        if types_chirho.is_empty() {
            "_".to_string()
        } else if types_chirho.len() == 1 {
            Self::type_key_from_ast_chirho(&types_chirho[0])
        } else {
            let keys_chirho: Vec<String> = types_chirho
                .iter()
                .map(|t_chirho| Self::type_key_from_ast_chirho(t_chirho))
                .collect();
            keys_chirho.join("_")
        }
    }

    /// Desugar an entire AST module into a Core module with name map.
    pub fn desugar_module_chirho(&mut self, module_chirho: &ModuleChirho) -> DesugarOutputChirho {
        // Pass 0: Build map of class → (method_name → default MatchArms) from class declarations
        let mut class_defaults_chirho: HashMap<String, HashMap<String, Vec<MatchArmChirho>>> =
            HashMap::new();
        let mut class_method_names_chirho: HashMap<String, Vec<String>> = HashMap::new();
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::ClassDeclChirho {
                name_chirho,
                methods_chirho,
                ..
            } = decl_chirho
            {
                let cn_chirho = name_chirho.text_chirho().to_string();
                let mut defaults_chirho = HashMap::new();
                let mut method_names_chirho = Vec::new();
                for method_chirho in methods_chirho {
                    let mn_chirho = method_chirho.name_chirho.text_chirho().to_string();
                    method_names_chirho.push(mn_chirho.clone());
                    if let Some(ref arms_chirho) = method_chirho.default_chirho {
                        defaults_chirho.insert(mn_chirho, arms_chirho.clone());
                    }
                }
                class_defaults_chirho.insert(cn_chirho.clone(), defaults_chirho);
                class_method_names_chirho.insert(cn_chirho, method_names_chirho);
            }
        }

        // Pass 1: Pre-bind all top-level names so they can reference each other
        let mut pre_binders_chirho: Vec<(usize, BinderChirho)> = Vec::new();
        for (idx_chirho, decl_chirho) in module_chirho.decls_chirho.iter().enumerate() {
            match decl_chirho {
                DeclChirho::FunBindChirho {
                    name_chirho,
                    span_chirho,
                    ..
                } => {
                    let binder_chirho = self.fresh_binder_chirho(
                        name_chirho.text_chirho(),
                        TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        *span_chirho,
                    );
                    self.bind_in_scope_chirho(
                        name_chirho.text_chirho(),
                        binder_chirho.id_chirho,
                    );
                    pre_binders_chirho.push((idx_chirho, binder_chirho));
                }
                DeclChirho::InstanceDeclChirho {
                    class_chirho,
                    types_chirho,
                    methods_chirho,
                    span_chirho,
                    ..
                } => {
                    let class_name_chirho = class_chirho.text_chirho();
                    let type_key_chirho =
                        Self::instance_type_key_chirho(types_chirho);
                    // Collect provided method names
                    let mut provided_chirho: std::collections::HashSet<String> =
                        std::collections::HashSet::new();
                    for method_chirho in methods_chirho {
                        if let LocalBindChirho::FunBindChirho {
                            name_chirho: method_name_chirho,
                            ..
                        } = method_chirho
                        {
                            let mn_chirho = method_name_chirho.text_chirho().to_string();
                            provided_chirho.insert(mn_chirho.clone());
                            let prim_name_chirho = format!(
                                "$prim_{}_{}_{}", class_name_chirho,
                                mn_chirho, type_key_chirho
                            );
                            let binder_chirho = self.fresh_binder_chirho(
                                &prim_name_chirho,
                                TyChirho::VarChirho(
                                    rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                *span_chirho,
                            );
                            self.bind_in_scope_chirho(
                                &prim_name_chirho,
                                binder_chirho.id_chirho,
                            );
                            pre_binders_chirho.push((idx_chirho, binder_chirho));
                        }
                    }
                    // Pre-bind default methods not provided by the instance
                    if let Some(defaults_chirho) =
                        class_defaults_chirho.get(&class_name_chirho.to_string())
                    {
                        for (default_name_chirho, _) in defaults_chirho {
                            if !provided_chirho.contains(default_name_chirho) {
                                let prim_name_chirho = format!(
                                    "$prim_{}_{}_{}", class_name_chirho,
                                    default_name_chirho, type_key_chirho
                                );
                                let binder_chirho = self.fresh_binder_chirho(
                                    &prim_name_chirho,
                                    TyChirho::VarChirho(
                                        rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                            self.next_id_chirho,
                                        ),
                                    ),
                                    *span_chirho,
                                );
                                self.bind_in_scope_chirho(
                                    &prim_name_chirho,
                                    binder_chirho.id_chirho,
                                );
                                pre_binders_chirho.push((idx_chirho, binder_chirho));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Pass 1b: Pre-bind record field accessor names so function bodies can reference them
        for decl_chirho in &module_chirho.decls_chirho {
            let constructors_chirho = match decl_chirho {
                DeclChirho::DataDeclChirho {
                    constructors_chirho, ..
                } => constructors_chirho.as_slice(),
                _ => continue,
            };
            for con_chirho in constructors_chirho {
                if let rhasky_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                    fields_chirho: field_decls_chirho,
                    ..
                } = con_chirho
                {
                    for fd_chirho in field_decls_chirho {
                        for fname_chirho in &fd_chirho.names_chirho {
                            let name_str_chirho = fname_chirho.text_chirho().to_string();
                            let binder_chirho = self.fresh_binder_chirho(
                                &name_str_chirho,
                                TyChirho::VarChirho(
                                    rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                SpanChirho::DUMMY_CHIRHO,
                            );
                            self.bind_in_scope_chirho(
                                &name_str_chirho,
                                binder_chirho.id_chirho,
                            );
                            // Pre-bind setter function
                            let setter_name_chirho =
                                format!("$setField_{}", name_str_chirho);
                            let setter_binder_chirho = self.fresh_binder_chirho(
                                &setter_name_chirho,
                                TyChirho::VarChirho(
                                    rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                SpanChirho::DUMMY_CHIRHO,
                            );
                            self.bind_in_scope_chirho(
                                &setter_name_chirho,
                                setter_binder_chirho.id_chirho,
                            );
                        }
                    }
                }
            }
        }

        // Convert AST inline pragmas to Core inline annotations
        let inline_pragmas_chirho = &module_chirho.inline_pragmas_chirho;

        // Pass 2: Desugar bodies
        let mut bindings_chirho = Vec::new();
        let mut pre_iter_chirho = pre_binders_chirho.into_iter().peekable();

        for (idx_chirho, decl_chirho) in module_chirho.decls_chirho.iter().enumerate() {
            match decl_chirho {
                DeclChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                } => {
                    let core_rhs_chirho =
                        self.desugar_matches_chirho(matches_chirho, *span_chirho);
                    // Use the pre-bound binder
                    let binder_chirho = if let Some((pre_idx, _)) = pre_iter_chirho.peek() {
                        if *pre_idx == idx_chirho {
                            pre_iter_chirho.next().unwrap().1
                        } else {
                            // Shouldn't happen, but fall back
                            self.fresh_binder_chirho(
                                "_unknown",
                                TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                    self.next_id_chirho,
                                )),
                                *span_chirho,
                            )
                        }
                    } else {
                        self.fresh_binder_chirho(
                            "_unknown",
                            TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            *span_chirho,
                        )
                    };
                    // Look up inline pragma for this binding
                    let inline_annotation_chirho = match inline_pragmas_chirho.get(&name_chirho.text_chirho().to_string()) {
                        Some(rhasky_ast_chirho::module_chirho::InlinePragmaChirho::InlineChirho) => InlineAnnotationChirho::AlwaysChirho,
                        Some(rhasky_ast_chirho::module_chirho::InlinePragmaChirho::NoInlineChirho) => InlineAnnotationChirho::NeverChirho,
                        Some(rhasky_ast_chirho::module_chirho::InlinePragmaChirho::InlinableChirho) => InlineAnnotationChirho::InlinableChirho,
                        None => InlineAnnotationChirho::NoneChirho,
                    };
                    bindings_chirho.push(CoreBindingChirho {
                        binder_chirho,
                        rhs_chirho: core_rhs_chirho,
                        is_rec_chirho: true,
                        inline_chirho: inline_annotation_chirho,
                    });
                }
                DeclChirho::PatBindChirho {
                    rhs_chirho,
                    span_chirho,
                    ..
                } => {
                    let core_rhs_chirho = self.desugar_rhs_chirho(rhs_chirho);
                    let binder_chirho = self.fresh_binder_chirho(
                        "_patbind",
                        TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        *span_chirho,
                    );
                    bindings_chirho.push(CoreBindingChirho {
                        binder_chirho,
                        rhs_chirho: core_rhs_chirho,
                        is_rec_chirho: true,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                    });
                }
                DeclChirho::InstanceDeclChirho {
                    class_chirho,
                    types_chirho,
                    methods_chirho,
                    span_chirho,
                    ..
                } => {
                    let class_name_chirho = class_chirho.text_chirho();
                    let type_key_chirho =
                        Self::instance_type_key_chirho(types_chirho);
                    let mut provided_chirho: std::collections::HashSet<String> =
                        std::collections::HashSet::new();
                    for method_chirho in methods_chirho {
                        if let LocalBindChirho::FunBindChirho {
                            name_chirho: method_name_chirho,
                            matches_chirho: method_matches_chirho,
                            span_chirho: method_span_chirho,
                        } = method_chirho
                        {
                            provided_chirho.insert(
                                method_name_chirho.text_chirho().to_string(),
                            );
                            let core_rhs_chirho = self.desugar_matches_chirho(
                                method_matches_chirho,
                                *method_span_chirho,
                            );
                            let prim_name_chirho = format!(
                                "$prim_{}_{}_{}", class_name_chirho,
                                method_name_chirho.text_chirho(), type_key_chirho
                            );
                            // Consume pre-bound binder for this instance
                            // (multiple methods share the same idx_chirho)
                            let binder_chirho =
                                if let Some((pre_idx_chirho, _)) = pre_iter_chirho.peek() {
                                    if *pre_idx_chirho == idx_chirho {
                                        pre_iter_chirho.next().unwrap().1
                                    } else {
                                        self.fresh_binder_chirho(
                                            &prim_name_chirho,
                                            TyChirho::VarChirho(
                                                rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                                    self.next_id_chirho,
                                                ),
                                            ),
                                            *span_chirho,
                                        )
                                    }
                                } else {
                                    self.fresh_binder_chirho(
                                        &prim_name_chirho,
                                        TyChirho::VarChirho(
                                            rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                                self.next_id_chirho,
                                            ),
                                        ),
                                        *span_chirho,
                                    )
                                };
                            bindings_chirho.push(CoreBindingChirho {
                                binder_chirho,
                                rhs_chirho: core_rhs_chirho,
                                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                            });
                        }
                    }
                    // Generate bindings for default methods not provided
                    if let Some(defaults_chirho) =
                        class_defaults_chirho.get(&class_name_chirho.to_string())
                    {
                        for (default_name_chirho, default_arms_chirho) in defaults_chirho {
                            if !provided_chirho.contains(default_name_chirho) {
                                let core_rhs_chirho = self.desugar_matches_chirho(
                                    default_arms_chirho,
                                    *span_chirho,
                                );
                                let prim_name_chirho = format!(
                                    "$prim_{}_{}_{}", class_name_chirho,
                                    default_name_chirho, type_key_chirho
                                );
                                let binder_chirho = if let Some((pre_idx_chirho, _)) =
                                    pre_iter_chirho.peek()
                                {
                                    if *pre_idx_chirho == idx_chirho {
                                        pre_iter_chirho.next().unwrap().1
                                    } else {
                                        self.fresh_binder_chirho(
                                            &prim_name_chirho,
                                            TyChirho::VarChirho(
                                                rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                                    self.next_id_chirho,
                                                ),
                                            ),
                                            *span_chirho,
                                        )
                                    }
                                } else {
                                    self.fresh_binder_chirho(
                                        &prim_name_chirho,
                                        TyChirho::VarChirho(
                                            rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                                self.next_id_chirho,
                                            ),
                                        ),
                                        *span_chirho,
                                    )
                                };
                                bindings_chirho.push(CoreBindingChirho {
                                    binder_chirho,
                                    rhs_chirho: core_rhs_chirho,
                                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                                });
                            }
                        }
                    }
                }
                // Data, type, class declarations don't produce Core bindings directly
                _ => {}
            }
        }

        // Generate record field accessor functions.
        // For data T = Con { f1 :: T1, f2 :: T2 }, generates:
        //   f1 = \r -> case r of { Con x _ -> x }
        //   f2 = \r -> case r of { Con _ x -> x }
        for decl_chirho in &module_chirho.decls_chirho {
            let constructors_chirho = match decl_chirho {
                DeclChirho::DataDeclChirho {
                    constructors_chirho, ..
                } => constructors_chirho.as_slice(),
                _ => continue,
            };
            for con_chirho in constructors_chirho {
                if let rhasky_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                    name_chirho: con_name_chirho,
                    fields_chirho: field_decls_chirho,
                    ..
                } = con_chirho
                {
                    // Flatten field names with their position index
                    let mut flat_fields_chirho: Vec<(String, usize)> = Vec::new();
                    for fd_chirho in field_decls_chirho {
                        for fname_chirho in &fd_chirho.names_chirho {
                            flat_fields_chirho.push((
                                fname_chirho.text_chirho().to_string(),
                                flat_fields_chirho.len(),
                            ));
                        }
                    }
                    let total_fields_chirho = flat_fields_chirho.len();
                    let con_str_chirho = con_name_chirho.text_chirho().to_string();

                    for (field_name_chirho, field_idx_chirho) in &flat_fields_chirho {
                        // Build: \r -> case r of { Con b0 b1 ... -> b_idx }
                        let param_binder_chirho = self.fresh_binder_chirho(
                            "$r",
                            TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        let mut alt_binders_chirho = Vec::new();
                        let mut result_id_chirho = CoreIdChirho(0);
                        for i_chirho in 0..total_fields_chirho {
                            let b_chirho = self.fresh_binder_chirho(
                                &format!("$f{}", i_chirho),
                                TyChirho::VarChirho(
                                    rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                SpanChirho::DUMMY_CHIRHO,
                            );
                            if i_chirho == *field_idx_chirho {
                                result_id_chirho = b_chirho.id_chirho;
                            }
                            alt_binders_chirho.push(b_chirho);
                        }

                        let case_bind_chirho = self.fresh_binder_chirho(
                            "$scrut",
                            TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        let body_chirho = CoreExprChirho::CaseChirho {
                            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                                param_binder_chirho.id_chirho,
                            )),
                            bind_chirho: case_bind_chirho,
                            result_ty_chirho: TyChirho::VarChirho(
                                rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                            ),
                            alts_chirho: vec![CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho(con_str_chirho.clone()),
                                binders_chirho: alt_binders_chirho,
                                rhs_chirho: CoreExprChirho::VarChirho(result_id_chirho),
                            }],
                        };

                        let accessor_rhs_chirho = CoreExprChirho::LamChirho {
                            binder_chirho: param_binder_chirho.clone(),
                            body_chirho: Box::new(body_chirho),
                        };

                        // Reuse the pre-bound ID from Pass 1b
                        let accessor_id_chirho = self.lookup_scope_chirho(field_name_chirho)
                            .unwrap_or_else(|| self.fresh_id_chirho(field_name_chirho));
                        let accessor_binder_chirho = BinderChirho {
                            id_chirho: accessor_id_chirho,
                            name_chirho: field_name_chirho.clone(),
                            ty_chirho: TyChirho::VarChirho(
                                rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                            ),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        };
                        bindings_chirho.push(CoreBindingChirho {
                            binder_chirho: accessor_binder_chirho,
                            rhs_chirho: accessor_rhs_chirho,
                            is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                        });

                        // Generate setter: $setField_f = \newVal -> \rec -> case rec of { Con b0 b1 ... -> Con ... newVal ... }
                        let setter_name_chirho =
                            format!("$setField_{}", field_name_chirho);
                        let new_val_binder_chirho = self.fresh_binder_chirho(
                            "$newVal",
                            TyChirho::VarChirho(
                                rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                            ),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        let rec_binder_chirho = self.fresh_binder_chirho(
                            "$rec",
                            TyChirho::VarChirho(
                                rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                            ),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        let mut setter_alt_binders_chirho = Vec::new();
                        let mut con_args_chirho = Vec::new();
                        for i_chirho in 0..total_fields_chirho {
                            let b_chirho = self.fresh_binder_chirho(
                                &format!("$sf{}", i_chirho),
                                TyChirho::VarChirho(
                                    rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                SpanChirho::DUMMY_CHIRHO,
                            );
                            if i_chirho == *field_idx_chirho {
                                // Replace this field with newVal
                                con_args_chirho.push(CoreExprChirho::VarChirho(
                                    new_val_binder_chirho.id_chirho,
                                ));
                            } else {
                                con_args_chirho.push(CoreExprChirho::VarChirho(b_chirho.id_chirho));
                            }
                            setter_alt_binders_chirho.push(b_chirho);
                        }
                        let setter_case_bind_chirho = self.fresh_binder_chirho(
                            "$scrut",
                            TyChirho::VarChirho(
                                rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                            ),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        let setter_body_chirho = CoreExprChirho::CaseChirho {
                            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                                rec_binder_chirho.id_chirho,
                            )),
                            bind_chirho: setter_case_bind_chirho,
                            result_ty_chirho: TyChirho::VarChirho(
                                rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                            ),
                            alts_chirho: vec![CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho(con_str_chirho.clone()),
                                binders_chirho: setter_alt_binders_chirho,
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: con_str_chirho.clone(),
                                    args_chirho: con_args_chirho,
                                },
                            }],
                        };
                        let setter_rhs_chirho = CoreExprChirho::LamChirho {
                            binder_chirho: new_val_binder_chirho,
                            body_chirho: Box::new(CoreExprChirho::LamChirho {
                                binder_chirho: rec_binder_chirho,
                                body_chirho: Box::new(setter_body_chirho),
                            }),
                        };
                        // Reuse pre-bound setter ID from Pass 1b
                        let setter_id_chirho =
                            self.lookup_scope_chirho(&setter_name_chirho)
                                .unwrap_or_else(|| self.fresh_id_chirho(&setter_name_chirho));
                        let setter_binder_chirho = BinderChirho {
                            id_chirho: setter_id_chirho,
                            name_chirho: setter_name_chirho.clone(),
                            ty_chirho: TyChirho::VarChirho(
                                rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                            ),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        };
                        bindings_chirho.push(CoreBindingChirho {
                            binder_chirho: setter_binder_chirho,
                            rhs_chirho: setter_rhs_chirho,
                            is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                        });
                    }
                }
            }
        }

        DesugarOutputChirho {
            module_chirho: CoreModuleChirho {
                name_chirho: module_chirho.name_chirho.text_chirho().to_string(),
                bindings_chirho,
                names_chirho: self.names_chirho.clone(),
                specialize_pragmas_chirho: module_chirho.specialize_pragmas_chirho.clone(),
            },
            names_chirho: self.names_chirho.clone(),
        }
    }

    /// Desugar a set of match arms into a Core expression.
    /// For single-equation functions: `\p1 p2 -> rhs`
    /// For multi-equation: case expressions.
    fn desugar_matches_chirho(
        &mut self,
        matches_chirho: &[MatchArmChirho],
        _span_chirho: SpanChirho,
    ) -> CoreExprChirho {
        if matches_chirho.is_empty() {
            return CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0));
        }

        let first_chirho = &matches_chirho[0];
        let arity_chirho = first_chirho.pats_chirho.len();

        if arity_chirho == 0 {
            // Simple value binding: no patterns — still process where-binds
            return self.desugar_arm_rhs_chirho(first_chirho);
        }

        // Build lambdas for each parameter, with the body being
        // a case expression (or the RHS if patterns are simple vars).
        self.push_scope_chirho();
        let param_binders_chirho: Vec<BinderChirho> = (0..arity_chirho)
            .map(|i_chirho| {
                let name_chirho = extract_var_name_from_pat_chirho(&first_chirho.pats_chirho[i_chirho])
                    .unwrap_or_else(|| format!("_arg{i_chirho}"));
                let binder_chirho = self.fresh_binder_chirho(&name_chirho, TyChirho::VarChirho(
                    rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                ), SpanChirho::DUMMY_CHIRHO);
                self.bind_in_scope_chirho(&name_chirho, binder_chirho.id_chirho);
                binder_chirho
            })
            .collect();

        // Build the body (for single-equation, just desugar RHS;
        // for multi-equation, build case alts)
        let body_chirho = if matches_chirho.len() == 1 && all_var_pats_chirho(&first_chirho.pats_chirho) {
            self.desugar_arm_rhs_chirho(first_chirho)
        } else {
            // Multi-equation or non-trivial patterns: case on first arg,
            // with nested cases for subsequent patterns when arity > 1.
            self.compile_multi_pattern_case_chirho(
                matches_chirho,
                &param_binders_chirho,
                0,
            )
        };

        // Pre-allocate fresh binders for bang-pattern seq-forcing before popping scope.
        let bang_wild_binders_chirho: Vec<Option<BinderChirho>> = first_chirho
            .pats_chirho
            .iter()
            .map(|pat_chirho| {
                if is_bang_pat_chirho(pat_chirho) {
                    Some(self.fresh_binder_chirho(
                        "_bang_wild",
                        TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        SpanChirho::DUMMY_CHIRHO,
                    ))
                } else {
                    None
                }
            })
            .collect();

        self.pop_scope_chirho();

        // Wrap body in seq-forcing for bang-patterned parameters.
        // `f !x !y = body` → `\x -> \y -> case x of { _ -> case y of { _ -> body }}`
        let mut result_chirho = body_chirho;
        for (i_chirho, wild_opt_chirho) in bang_wild_binders_chirho.iter().enumerate().rev() {
            if let Some(wild_binder_chirho) = wild_opt_chirho {
                result_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                        param_binders_chirho[i_chirho].id_chirho,
                    )),
                    bind_chirho: wild_binder_chirho.clone(),
                    result_ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(0),
                    ),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: result_chirho,
                    }],
                };
            }
        }

        // Wrap body in lambdas
        for binder_chirho in param_binders_chirho.into_iter().rev() {
            result_chirho = CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho: Box::new(result_chirho),
            };
        }

        result_chirho
    }

    /// Compile multi-pattern matching by recursively casing on each pattern
    /// column. Groups match arms by their constructor at `pat_idx_chirho`,
    /// generates a `case` on `param_binders_chirho[pat_idx_chirho]`, and
    /// for each group recurses on `pat_idx_chirho + 1` for subsequent
    /// patterns.
    fn compile_multi_pattern_case_chirho(
        &mut self,
        matches_chirho: &[MatchArmChirho],
        param_binders_chirho: &[BinderChirho],
        pat_idx_chirho: usize,
    ) -> CoreExprChirho {
        let arity_chirho = param_binders_chirho.len();

        // Base case: all patterns consumed — desugar the RHS of the first arm.
        if pat_idx_chirho >= arity_chirho {
            if let Some(arm_chirho) = matches_chirho.first() {
                return self.desugar_arm_rhs_chirho(arm_chirho);
            }
            // No arms (shouldn't happen) — produce a bottom value.
            return CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0));
        }

        let scrut_id_chirho = param_binders_chirho[pat_idx_chirho].id_chirho;
        let wild_chirho = self.fresh_binder_chirho(
            "wild",
            TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                self.next_id_chirho,
            )),
            SpanChirho::DUMMY_CHIRHO,
        );

        // Group arms by the constructor at pat_idx_chirho.
        // Preserve ordering: each group is (AltCon, Vec<&MatchArm>).
        let mut groups_chirho: Vec<(AltConChirho, Vec<&MatchArmChirho>)> = Vec::new();
        // Collect default/wildcard arms separately — they must be appended
        // as fallthroughs to every non-default group so that inner cases
        // have a catch-all alternative.
        let mut default_arms_chirho: Vec<&MatchArmChirho> = Vec::new();
        for arm_chirho in matches_chirho {
            let con_chirho = if arm_chirho.pats_chirho.len() > pat_idx_chirho {
                self.pat_to_alt_con_chirho(&arm_chirho.pats_chirho[pat_idx_chirho])
            } else {
                AltConChirho::DefaultChirho
            };
            if con_chirho == AltConChirho::DefaultChirho {
                default_arms_chirho.push(arm_chirho);
            }
            if let Some(group_chirho) = groups_chirho
                .iter_mut()
                .find(|(c_chirho, _)| *c_chirho == con_chirho)
            {
                group_chirho.1.push(arm_chirho);
            } else {
                groups_chirho.push((con_chirho, vec![arm_chirho]));
            }
        }

        // Optimization: if ALL groups at this pattern column are DefaultChirho
        // (every arm has a Var/Wildcard pattern), skip generating a case
        // expression entirely. Just bind the variable names to the scrutinee
        // and recurse or desugar the RHS directly.  This avoids emitting a
        // redundant `case scrut of { DEFAULT -> ... }` which corrupts
        // arg_regs in the STG runtime when boxing/unboxing through
        // return_con_chirho.
        let all_default_chirho = groups_chirho.iter().all(|(c_chirho, _)| *c_chirho == AltConChirho::DefaultChirho);
        if all_default_chirho && !groups_chirho.is_empty() {
            let all_arms_chirho: Vec<&MatchArmChirho> = groups_chirho
                .iter()
                .flat_map(|(_, arms_chirho)| arms_chirho.iter().copied())
                .collect();
            // Bind any VarChirho pattern at this column to the scrutinee
            if let Some(first_arm_chirho) = all_arms_chirho.first() {
                if let PatChirho::VarChirho(n_chirho) = &first_arm_chirho.pats_chirho[pat_idx_chirho] {
                    self.bind_in_scope_chirho(n_chirho.text_chirho(), scrut_id_chirho);
                }
            }
            if pat_idx_chirho + 1 >= arity_chirho {
                return self.desugar_arm_rhs_chirho(all_arms_chirho[0]);
            } else {
                let sub_arms_chirho: Vec<MatchArmChirho> =
                    all_arms_chirho.iter().map(|a_chirho| (*a_chirho).clone()).collect();
                return self.compile_multi_pattern_case_chirho(
                    &sub_arms_chirho,
                    param_binders_chirho,
                    pat_idx_chirho + 1,
                );
            }
        }

        let alts_chirho: Vec<CoreAltChirho> = groups_chirho
            .into_iter()
            .map(|(con_chirho, arms_chirho)| {
                self.push_scope_chirho();
                let pat_ref_chirho = &arms_chirho[0].pats_chirho[pat_idx_chirho];
                let binders_chirho = self.pat_to_binders_chirho(pat_ref_chirho);
                self.prebind_all_pat_vars_chirho(pat_ref_chirho);

                // For Default alt with a VarChirho pattern, bind the variable
                // name to the scrutinee ID so references in the RHS resolve to
                // the actual argument value (e.g. `f x = x + 1` with `f 0 = 100`
                // as the literal arm — `x` must refer to the param binder holding
                // the scrutinee, not a fresh unbound id).
                if con_chirho == AltConChirho::DefaultChirho {
                    if let PatChirho::VarChirho(n_chirho) = pat_ref_chirho {
                        self.bind_in_scope_chirho(n_chirho.text_chirho(), scrut_id_chirho);
                    }
                }

                // Recurse: compile remaining pattern columns for this group.
                let rhs_chirho = if pat_idx_chirho + 1 >= arity_chirho {
                    // Last pattern column — desugar the RHS.
                    self.desugar_arm_rhs_chirho(arms_chirho[0])
                } else {
                    // More pattern columns remain — recurse.
                    // For non-default groups, append default arms as
                    // fallthroughs so inner cases have catch-all alternatives.
                    let mut sub_arms_chirho: Vec<MatchArmChirho> =
                        arms_chirho.iter().map(|a_chirho| (*a_chirho).clone()).collect();
                    if con_chirho != AltConChirho::DefaultChirho {
                        for da_chirho in &default_arms_chirho {
                            sub_arms_chirho.push((*da_chirho).clone());
                        }
                    }
                    self.compile_multi_pattern_case_chirho(
                        &sub_arms_chirho,
                        param_binders_chirho,
                        pat_idx_chirho + 1,
                    )
                };

                // Wrap with nested case expressions for nested patterns
                // (e.g. `f (Just x) = ...` where `x` is bound inside the
                // constructor pattern at this column).
                let rhs_nested_chirho = self.wrap_nested_cases_chirho(
                    rhs_chirho,
                    &binders_chirho,
                    pat_ref_chirho,
                );
                // Wrap with as-pattern let-bindings if applicable.
                let rhs_final_chirho = self.wrap_as_bindings_chirho(
                    rhs_nested_chirho,
                    pat_ref_chirho,
                    scrut_id_chirho,
                );

                self.pop_scope_chirho();
                CoreAltChirho {
                    con_chirho,
                    binders_chirho,
                    rhs_chirho: rhs_final_chirho,
                }
            })
            .collect();

        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(scrut_id_chirho)),
            bind_chirho: wild_chirho,
            result_ty_chirho: TyChirho::VarChirho(
                rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
            ),
            alts_chirho,
        }
    }

    /// Desugar the RHS of a match arm, wrapping it in a let if where-binds
    /// are present.  Where-clause names are bound *before* the body is
    /// desugared so that references in the body resolve to the Let binders.
    fn desugar_arm_rhs_chirho(&mut self, arm_chirho: &MatchArmChirho) -> CoreExprChirho {
        if arm_chirho.where_binds_chirho.is_empty() {
            return self.desugar_rhs_chirho(&arm_chirho.rhs_chirho);
        }

        self.push_scope_chirho();

        // Pass 1: Pre-bind all where-clause names so body and
        // mutual references resolve to the fresh Let binders.
        let pre_binders_chirho = self.prebind_where_names_chirho(&arm_chirho.where_binds_chirho);

        // Desugar body *after* where names are in scope.
        let body_core_chirho = self.desugar_rhs_chirho(&arm_chirho.rhs_chirho);

        // Pass 2: Desugar where-clause RHSes.
        let core_binds_chirho = self.desugar_where_rhs_chirho(
            &arm_chirho.where_binds_chirho,
            pre_binders_chirho,
        );

        let rec_chirho = Self::is_recursive_binds_chirho(&core_binds_chirho);
        let result_chirho = CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho: core_binds_chirho,
            body_chirho: Box::new(body_core_chirho),
        };
        self.pop_scope_chirho();
        result_chirho
    }

    /// Pre-bind all where-clause names in the current scope so that both
    /// the body and the where-clause RHSes can reference them.
    fn prebind_where_names_chirho(
        &mut self,
        where_binds_chirho: &[rhasky_ast_chirho::expr_chirho::LocalBindChirho],
    ) -> Vec<(usize, BinderChirho)> {
        let mut pre_binders_chirho: Vec<(usize, BinderChirho)> = Vec::new();
        for (idx_chirho, bind_chirho) in where_binds_chirho.iter().enumerate() {
            match bind_chirho {
                rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    name_chirho,
                    span_chirho,
                    ..
                } => {
                    let binder_chirho = self.fresh_binder_chirho(
                        name_chirho.text_chirho(),
                        TyChirho::VarChirho(
                            rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            ),
                        ),
                        *span_chirho,
                    );
                    self.bind_in_scope_chirho(
                        name_chirho.text_chirho(),
                        binder_chirho.id_chirho,
                    );
                    pre_binders_chirho.push((idx_chirho, binder_chirho));
                }
                rhasky_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    span_chirho,
                    ..
                } => {
                    let binder_chirho = self.fresh_binder_chirho(
                        "_where",
                        TyChirho::VarChirho(
                            rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            ),
                        ),
                        *span_chirho,
                    );
                    pre_binders_chirho.push((idx_chirho, binder_chirho));
                }
                _ => {}
            }
        }
        pre_binders_chirho
    }

    /// Desugar all where-clause RHSes (names must already be in scope via
    /// `prebind_where_names_chirho`).
    fn desugar_where_rhs_chirho(
        &mut self,
        where_binds_chirho: &[rhasky_ast_chirho::expr_chirho::LocalBindChirho],
        pre_binders_chirho: Vec<(usize, BinderChirho)>,
    ) -> Vec<(BinderChirho, CoreExprChirho)> {
        let mut core_binds_chirho: Vec<(BinderChirho, CoreExprChirho)> = Vec::new();
        for (idx_chirho, binder_chirho) in pre_binders_chirho {
            let rhs_chirho = match &where_binds_chirho[idx_chirho] {
                rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    matches_chirho,
                    span_chirho,
                    ..
                } => self.desugar_matches_chirho(matches_chirho, *span_chirho),
                rhasky_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    rhs_chirho,
                    ..
                } => self.desugar_rhs_chirho(rhs_chirho),
                _ => continue,
            };
            core_binds_chirho.push((binder_chirho, rhs_chirho));
        }
        core_binds_chirho
    }

    /// Convert a pattern to a Core alt constructor.
    fn pat_to_alt_con_chirho(&mut self, pat_chirho: &PatChirho) -> AltConChirho {
        match pat_chirho {
            PatChirho::ConChirho { con_chirho, .. } => {
                AltConChirho::DataConChirho(con_chirho.text_chirho().to_string())
            }
            PatChirho::RecordChirho { con_chirho, .. } => {
                AltConChirho::DataConChirho(con_chirho.text_chirho().to_string())
            }
            PatChirho::InfixConChirho { op_chirho, .. } => {
                AltConChirho::DataConChirho(op_chirho.text_chirho().to_string())
            }
            PatChirho::TupleChirho { elements_chirho, .. } => {
                let arity_chirho = elements_chirho.len();
                AltConChirho::DataConChirho(format!("$tuple{}", arity_chirho))
            }
            // List pattern `[]` → DataCon("[]"); `[a,b,c]` → DataCon(":") (matches head).
            PatChirho::ListChirho { elements_chirho, .. } => {
                if elements_chirho.is_empty() {
                    AltConChirho::DataConChirho("[]".to_string())
                } else {
                    AltConChirho::DataConChirho(":".to_string())
                }
            }
            PatChirho::LitChirho(lit_chirho) => {
                AltConChirho::LitConChirho(self.desugar_lit_chirho(lit_chirho))
            }
            PatChirho::NegChirho { lit_chirho, .. } => {
                let core_lit_chirho = self.desugar_lit_chirho(lit_chirho);
                match core_lit_chirho {
                    CoreLitChirho::IntChirho(n_chirho) => {
                        AltConChirho::LitConChirho(CoreLitChirho::IntChirho(-n_chirho))
                    }
                    CoreLitChirho::FloatChirho(n_chirho) => {
                        AltConChirho::LitConChirho(CoreLitChirho::FloatChirho(-n_chirho))
                    }
                    other_chirho => AltConChirho::LitConChirho(other_chirho),
                }
            }
            PatChirho::ParenChirho { inner_chirho, .. } => {
                self.pat_to_alt_con_chirho(inner_chirho)
            }
            PatChirho::AsChirho { pattern_chirho, .. } => {
                self.pat_to_alt_con_chirho(pattern_chirho)
            }
            // Bang pattern: delegate to inner — case scrutinee already
            // forces to WHNF.
            PatChirho::BangChirho { inner_chirho, .. } => {
                self.pat_to_alt_con_chirho(inner_chirho)
            }
            // Lazy (irrefutable) pattern: always matches.
            PatChirho::LazyChirho { .. } => AltConChirho::DefaultChirho,
            PatChirho::WildcardChirho(_) => AltConChirho::DefaultChirho,
            PatChirho::VarChirho(_) => AltConChirho::DefaultChirho,
        }
    }

    /// Extract binders from a constructor pattern's sub-patterns,
    /// binding them in the current scope. Returns binders in positional order.
    ///
    /// For `Con a b c` → binders `[a, b, c]`
    /// For `x:xs` (InfixConChirho) → binders `[x, xs]`
    /// For `Con { f1 = p1, f2 = p2 }` (RecordChirho) → binders `[p1, p2]`
    /// For `(a, b)` (TupleChirho) → binders `[a, b]`
    /// For `x@Con a b` (AsChirho) → binder `x` + inner binders
    fn pat_to_binders_chirho(&mut self, pat_chirho: &PatChirho) -> Vec<BinderChirho> {
        match pat_chirho {
            PatChirho::ConChirho { args_chirho, .. } => {
                self.sub_pats_to_binders_chirho(args_chirho)
            }
            PatChirho::RecordChirho { fields_chirho, .. } => {
                let sub_pats_chirho: Vec<&PatChirho> = fields_chirho
                    .iter()
                    .map(|f_chirho| &f_chirho.pattern_chirho)
                    .collect();
                sub_pats_chirho
                    .into_iter()
                    .map(|p_chirho| self.pat_to_single_binder_chirho(p_chirho))
                    .collect()
            }
            PatChirho::InfixConChirho {
                left_chirho,
                right_chirho,
                ..
            } => {
                let left_binder_chirho = self.pat_to_single_binder_chirho(left_chirho);
                let right_binder_chirho = self.pat_to_single_binder_chirho(right_chirho);
                vec![left_binder_chirho, right_binder_chirho]
            }
            // List pattern: `[]` has no binders; `[a, b, c]` desugars to
            // `a : [b, c]` — two binders: the head element and the tail.
            PatChirho::ListChirho {
                elements_chirho,
                span_chirho,
            } => {
                if elements_chirho.is_empty() {
                    // `[]` — no fields
                    vec![]
                } else {
                    // `[head, rest..]` — head binder + tail binder (tail is
                    // a synthetic ListChirho of the remaining elements)
                    let head_binder_chirho =
                        self.pat_to_single_binder_chirho(&elements_chirho[0]);
                    let tail_pat_chirho = PatChirho::ListChirho {
                        elements_chirho: elements_chirho[1..].to_vec(),
                        span_chirho: *span_chirho,
                    };
                    let tail_binder_chirho =
                        self.pat_to_single_binder_chirho(&tail_pat_chirho);
                    vec![head_binder_chirho, tail_binder_chirho]
                }
            }
            PatChirho::TupleChirho {
                elements_chirho, ..
            } => self.sub_pats_to_binders_chirho(elements_chirho),
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho,
                ..
            } => {
                // Bind the as-name, then extract inner binders.
                let as_binder_chirho = self.fresh_binder_chirho(
                    name_chirho.text_chirho(),
                    TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                self.bind_in_scope_chirho(
                    name_chirho.text_chirho(),
                    as_binder_chirho.id_chirho,
                );
                // Note: in Core, as-patterns need special treatment.
                // For now, return inner binders (the as-binding would be
                // handled as a let-binding wrapping the alt RHS in a
                // later, more complete implementation).
                self.pat_to_binders_chirho(pattern_chirho)
            }
            PatChirho::ParenChirho { inner_chirho, .. } => {
                self.pat_to_binders_chirho(inner_chirho)
            }
            // Bang pattern: delegate to inner (strictness handled by case).
            PatChirho::BangChirho { inner_chirho, .. } => {
                self.pat_to_binders_chirho(inner_chirho)
            }
            // Lazy pattern: delegate to inner (bindings extracted lazily).
            PatChirho::LazyChirho { inner_chirho, .. } => {
                self.pat_to_binders_chirho(inner_chirho)
            }
            // Var, Wildcard, Lit, etc. — no sub-binders
            _ => vec![],
        }
    }

    /// Convert a list of sub-patterns to binders (for constructor args, tuple elements).
    fn sub_pats_to_binders_chirho(&mut self, pats_chirho: &[PatChirho]) -> Vec<BinderChirho> {
        pats_chirho
            .iter()
            .map(|p_chirho| self.pat_to_single_binder_chirho(p_chirho))
            .collect()
    }

    /// Convert a single sub-pattern into a binder and bind it in scope.
    /// For variable patterns, uses the variable name.
    /// For wildcards, generates a fresh name.
    /// For nested constructor patterns, generates a fresh name (nested
    /// matching would require further decomposition in a full implementation).
    fn pat_to_single_binder_chirho(&mut self, pat_chirho: &PatChirho) -> BinderChirho {
        let name_chirho = match pat_chirho {
            PatChirho::VarChirho(n_chirho) => n_chirho.text_chirho().to_string(),
            PatChirho::AsChirho { name_chirho, .. } => name_chirho.text_chirho().to_string(),
            PatChirho::WildcardChirho(_) => "_wild".to_string(),
            // Bang/lazy: delegate to inner pattern's name.
            PatChirho::BangChirho { inner_chirho, .. }
            | PatChirho::LazyChirho { inner_chirho, .. } => {
                return self.pat_to_single_binder_chirho(inner_chirho);
            }
            _ => "_pat".to_string(),
        };
        let binder_chirho = self.fresh_binder_chirho(
            &name_chirho,
            TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                self.next_id_chirho,
            )),
            SpanChirho::DUMMY_CHIRHO,
        );
        self.bind_in_scope_chirho(&name_chirho, binder_chirho.id_chirho);
        binder_chirho
    }

    /// Recursively bind ALL variable names in a pattern tree (including
    /// nested constructor sub-patterns) so that the RHS can reference them.
    /// Does NOT create binders; just binds names to pre-allocated IDs.
    fn prebind_all_pat_vars_chirho(&mut self, pat_chirho: &PatChirho) {
        // Only recurse into sub-patterns that are nested constructor patterns.
        // Simple VarChirho/WildcardChirho/LitChirho at the immediate child
        // level are already handled by `pat_to_binders_chirho`.
        match pat_chirho {
            PatChirho::VarChirho(_) | PatChirho::WildcardChirho(_) | PatChirho::LitChirho(_)
            | PatChirho::NegChirho { .. } => {}
            PatChirho::ConChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    if Self::is_nested_con_pat_chirho(arg_chirho) {
                        self.prebind_nested_pat_vars_chirho(arg_chirho);
                    }
                }
            }
            PatChirho::InfixConChirho {
                left_chirho,
                right_chirho,
                ..
            } => {
                if Self::is_nested_con_pat_chirho(left_chirho) {
                    self.prebind_nested_pat_vars_chirho(left_chirho);
                }
                if Self::is_nested_con_pat_chirho(right_chirho) {
                    self.prebind_nested_pat_vars_chirho(right_chirho);
                }
            }
            PatChirho::TupleChirho {
                elements_chirho, ..
            } => {
                for elem_chirho in elements_chirho {
                    if Self::is_nested_con_pat_chirho(elem_chirho) {
                        self.prebind_nested_pat_vars_chirho(elem_chirho);
                    }
                }
            }
            PatChirho::RecordChirho {
                fields_chirho, ..
            } => {
                for field_chirho in fields_chirho {
                    if Self::is_nested_con_pat_chirho(&field_chirho.pattern_chirho) {
                        self.prebind_nested_pat_vars_chirho(&field_chirho.pattern_chirho);
                    }
                }
            }
            // List pattern: recurse into each element to bind variables.
            // Empty list `[]` has no variables. Non-empty `[a, b, c]` are
            // treated as head `a` plus tail `[b, c]` recursively.
            PatChirho::ListChirho { elements_chirho, .. } => {
                for elem_chirho in elements_chirho {
                    self.prebind_nested_pat_vars_chirho(elem_chirho);
                }
            }
            PatChirho::AsChirho { pattern_chirho, .. } => {
                self.prebind_all_pat_vars_chirho(pattern_chirho);
            }
            PatChirho::ParenChirho { inner_chirho, .. }
            | PatChirho::BangChirho { inner_chirho, .. }
            | PatChirho::LazyChirho { inner_chirho, .. } => {
                self.prebind_all_pat_vars_chirho(inner_chirho);
            }
        }
    }

    /// For a sub-pattern that appears inside a constructor pattern, bind
    /// its variables in scope.  If it's a variable, bind it; if it's a
    /// constructor pattern, recurse into its children.
    fn prebind_nested_pat_vars_chirho(&mut self, pat_chirho: &PatChirho) {
        match pat_chirho {
            PatChirho::VarChirho(name_chirho) => {
                let binder_chirho = self.fresh_binder_chirho(
                    name_chirho.text_chirho(),
                    TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                self.bind_in_scope_chirho(
                    name_chirho.text_chirho(),
                    binder_chirho.id_chirho,
                );
            }
            PatChirho::ConChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    self.prebind_nested_pat_vars_chirho(arg_chirho);
                }
            }
            PatChirho::InfixConChirho {
                left_chirho,
                right_chirho,
                ..
            } => {
                self.prebind_nested_pat_vars_chirho(left_chirho);
                self.prebind_nested_pat_vars_chirho(right_chirho);
            }
            PatChirho::TupleChirho {
                elements_chirho, ..
            } => {
                for elem_chirho in elements_chirho {
                    self.prebind_nested_pat_vars_chirho(elem_chirho);
                }
            }
            PatChirho::RecordChirho {
                fields_chirho, ..
            } => {
                for field_chirho in fields_chirho {
                    self.prebind_nested_pat_vars_chirho(&field_chirho.pattern_chirho);
                }
            }
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho,
                ..
            } => {
                let binder_chirho = self.fresh_binder_chirho(
                    name_chirho.text_chirho(),
                    TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                self.bind_in_scope_chirho(
                    name_chirho.text_chirho(),
                    binder_chirho.id_chirho,
                );
                self.prebind_nested_pat_vars_chirho(pattern_chirho);
            }
            PatChirho::ParenChirho { inner_chirho, .. }
            | PatChirho::BangChirho { inner_chirho, .. }
            | PatChirho::LazyChirho { inner_chirho, .. } => {
                self.prebind_nested_pat_vars_chirho(inner_chirho);
            }
            PatChirho::WildcardChirho(_) | PatChirho::LitChirho(_)
            | PatChirho::NegChirho { .. } => {}
            // List pattern: recurse into each element.
            PatChirho::ListChirho { elements_chirho, .. } => {
                for elem_chirho in elements_chirho {
                    self.prebind_nested_pat_vars_chirho(elem_chirho);
                }
            }
        }
    }

    /// Check whether a pattern is a "nested" constructor pattern that needs
    /// further case decomposition (i.e. not a simple var/wildcard/lit).
    fn is_nested_con_pat_chirho(pat_chirho: &PatChirho) -> bool {
        match pat_chirho {
            PatChirho::ConChirho { .. }
            | PatChirho::InfixConChirho { .. }
            | PatChirho::TupleChirho { .. }
            | PatChirho::RecordChirho { .. }
            // List patterns always need case decomposition — even `[]` needs
            // a DataCon("[]") alt rather than being treated as a wildcard.
            | PatChirho::ListChirho { .. } => true,
            PatChirho::ParenChirho { inner_chirho, .. } => {
                Self::is_nested_con_pat_chirho(inner_chirho)
            }
            _ => false,
        }
    }

    /// Get the sub-patterns of an outer constructor pattern.
    fn outer_sub_pats_chirho(pat_chirho: &PatChirho) -> Vec<&PatChirho> {
        match pat_chirho {
            PatChirho::ConChirho { args_chirho, .. } => args_chirho.iter().collect(),
            PatChirho::InfixConChirho {
                left_chirho,
                right_chirho,
                ..
            } => vec![left_chirho.as_ref(), right_chirho.as_ref()],
            PatChirho::TupleChirho {
                elements_chirho, ..
            } => elements_chirho.iter().collect(),
            PatChirho::RecordChirho {
                fields_chirho, ..
            } => fields_chirho.iter().map(|f_chirho| &f_chirho.pattern_chirho).collect(),
            PatChirho::AsChirho { pattern_chirho, .. } => Self::outer_sub_pats_chirho(pattern_chirho),
            PatChirho::ParenChirho { inner_chirho, .. } => Self::outer_sub_pats_chirho(inner_chirho),
            PatChirho::BangChirho { inner_chirho, .. } => Self::outer_sub_pats_chirho(inner_chirho),
            _ => vec![],
        }
    }

    /// Wrap an RHS expression with nested case expressions for any nested
    /// constructor sub-patterns.  `binders_chirho` are the flat binders
    /// produced by `pat_to_binders_chirho`, and `outer_pat_chirho` is the
    /// outer pattern from which they were extracted.  Nested variable IDs
    /// are resolved from the current scope (pre-bound by
    /// `prebind_all_pat_vars_chirho`).
    fn wrap_nested_cases_chirho(
        &mut self,
        rhs_chirho: CoreExprChirho,
        binders_chirho: &[BinderChirho],
        outer_pat_chirho: &PatChirho,
    ) -> CoreExprChirho {
        let sub_pats_chirho = Self::outer_sub_pats_chirho(outer_pat_chirho);
        if sub_pats_chirho.len() != binders_chirho.len() {
            return rhs_chirho;
        }

        let mut result_chirho = rhs_chirho;
        // Process in reverse so innermost nested cases wrap first
        for (binder_chirho, sub_pat_chirho) in
            binders_chirho.iter().zip(sub_pats_chirho.iter()).rev()
        {
            if !Self::is_nested_con_pat_chirho(sub_pat_chirho) {
                continue;
            }
            // Build a nested case: `case binder of { SubCon a b -> result }`
            let nested_con_chirho = self.pat_to_alt_con_chirho(sub_pat_chirho);
            // Reuse pre-bound IDs from scope for nested binders
            let nested_binders_chirho =
                self.nested_pat_to_binders_chirho(sub_pat_chirho);

            // Recursively wrap for deeper nesting
            let inner_rhs_chirho = self.wrap_nested_cases_chirho(
                result_chirho,
                &nested_binders_chirho,
                sub_pat_chirho,
            );

            let wild_chirho = self.fresh_binder_chirho(
                "_nested_wild",
                TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                    self.next_id_chirho,
                )),
                SpanChirho::DUMMY_CHIRHO,
            );
            let alt_chirho = CoreAltChirho {
                con_chirho: nested_con_chirho,
                binders_chirho: nested_binders_chirho,
                rhs_chirho: inner_rhs_chirho,
            };
            result_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                    binder_chirho.id_chirho,
                )),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::VarChirho(
                    rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                ),
                alts_chirho: vec![alt_chirho],
            };
        }
        result_chirho
    }

    /// Wrap `rhs_chirho` with `let as_name = scrutinee in rhs` for every
    /// as-pattern (`name@pat`) found in the outer pattern.
    fn wrap_as_bindings_chirho(
        &self,
        rhs_chirho: CoreExprChirho,
        pat_chirho: &PatChirho,
        scrutinee_id_chirho: CoreIdChirho,
    ) -> CoreExprChirho {
        match pat_chirho {
            PatChirho::AsChirho {
                name_chirho, ..
            } => {
                let name_str_chirho = name_chirho.text_chirho();
                if let Some(as_id_chirho) = self.lookup_scope_chirho(name_str_chirho) {
                    let as_binder_chirho = BinderChirho {
                        id_chirho: as_id_chirho,
                        name_chirho: name_str_chirho.to_string(),
                        ty_chirho: TyChirho::VarChirho(
                            rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                        ),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    };
                    CoreExprChirho::LetChirho {
                        rec_chirho: false,
                        binds_chirho: vec![(
                            as_binder_chirho,
                            CoreExprChirho::VarChirho(scrutinee_id_chirho),
                        )],
                        body_chirho: Box::new(rhs_chirho),
                    }
                } else {
                    rhs_chirho
                }
            }
            PatChirho::ParenChirho { inner_chirho, .. } => {
                self.wrap_as_bindings_chirho(rhs_chirho, inner_chirho, scrutinee_id_chirho)
            }
            _ => rhs_chirho,
        }
    }

    /// Like `pat_to_binders_chirho` but reuses pre-bound IDs from scope
    /// (created by `prebind_all_pat_vars_chirho`) instead of creating fresh
    /// binders.
    fn nested_pat_to_binders_chirho(&mut self, pat_chirho: &PatChirho) -> Vec<BinderChirho> {
        let sub_pats_chirho = Self::outer_sub_pats_chirho(pat_chirho);
        sub_pats_chirho
            .into_iter()
            .map(|p_chirho| self.nested_single_binder_chirho(p_chirho))
            .collect()
    }

    /// Create a binder for a nested sub-pattern, reusing the pre-bound ID
    /// from scope for variables, or creating a fresh one for wildcards and
    /// nested constructors.
    fn nested_single_binder_chirho(&mut self, pat_chirho: &PatChirho) -> BinderChirho {
        match pat_chirho {
            PatChirho::VarChirho(name_chirho) => {
                let name_str_chirho = name_chirho.text_chirho();
                if let Some(id_chirho) = self.lookup_scope_chirho(name_str_chirho) {
                    // Reuse the pre-bound ID
                    BinderChirho {
                        id_chirho,
                        name_chirho: name_str_chirho.to_string(),
                        ty_chirho: TyChirho::VarChirho(
                            rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            ),
                        ),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }
                } else {
                    // Fallback: create fresh
                    let binder_chirho = self.fresh_binder_chirho(
                        name_str_chirho,
                        TyChirho::VarChirho(
                            rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            ),
                        ),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    self.bind_in_scope_chirho(
                        name_str_chirho,
                        binder_chirho.id_chirho,
                    );
                    binder_chirho
                }
            }
            PatChirho::WildcardChirho(_) => self.fresh_binder_chirho(
                "_wild",
                TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                    self.next_id_chirho,
                )),
                SpanChirho::DUMMY_CHIRHO,
            ),
            PatChirho::ParenChirho { inner_chirho, .. }
            | PatChirho::BangChirho { inner_chirho, .. }
            | PatChirho::LazyChirho { inner_chirho, .. } => {
                self.nested_single_binder_chirho(inner_chirho)
            }
            _ => {
                // Nested constructor or other — create fresh "_pat" binder
                let binder_chirho = self.fresh_binder_chirho(
                    "_pat",
                    TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        ),
                    ),
                    SpanChirho::DUMMY_CHIRHO,
                );
                self.bind_in_scope_chirho("_pat", binder_chirho.id_chirho);
                binder_chirho
            }
        }
    }

    /// Desugar a right-hand side.
    fn desugar_rhs_chirho(&mut self, rhs_chirho: &RhsChirho) -> CoreExprChirho {
        match rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => self.desugar_expr_chirho(expr_chirho),
            RhsChirho::GuardedChirho(guards_chirho) => {
                // Desugar guards into nested if-then-else:
                //   | g1 = e1        → case g1 of True -> e1; _ -> case g2 of ...
                //   | g2 = e2
                //   | otherwise = e3 → e3
                self.desugar_guards_chirho(guards_chirho)
            }
        }
    }

    /// Desugar an AST expression into a Core expression.
    pub fn desugar_expr_chirho(&mut self, expr_chirho: &ExprChirho) -> CoreExprChirho {
        match expr_chirho {
            ExprChirho::VarChirho(name_chirho) => {
                let id_chirho = self.resolve_var_chirho(name_chirho.text_chirho());
                CoreExprChirho::VarChirho(id_chirho)
            }
            ExprChirho::ConChirho(name_chirho) => {
                CoreExprChirho::ConAppChirho {
                    con_name_chirho: name_chirho.text_chirho().to_string(),
                    args_chirho: vec![],
                }
            }

            ExprChirho::LitChirho(lit_chirho) => {
                let core_lit_chirho = self.desugar_lit_chirho(lit_chirho);
                // Numeric literal overloading (Haskell 2010 §3.2):
                // Integer literals become `fromInteger <lit>`, allowing
                // the dict pass to select the correct Num instance.
                if matches!(core_lit_chirho, CoreLitChirho::IntChirho(_)) {
                    let from_integer_id_chirho =
                        self.resolve_var_chirho("fromInteger");
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                            from_integer_id_chirho,
                        )),
                        arg_chirho: Box::new(CoreExprChirho::LitChirho(
                            core_lit_chirho,
                        )),
                    }
                } else if matches!(core_lit_chirho, CoreLitChirho::StringChirho(_))
                    && self.extensions_chirho.contains(&"OverloadedStrings".to_string())
                {
                    // OverloadedStrings: string literals become `fromString "lit"`
                    let from_string_id_chirho =
                        self.resolve_var_chirho("fromString");
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                            from_string_id_chirho,
                        )),
                        arg_chirho: Box::new(CoreExprChirho::LitChirho(
                            core_lit_chirho,
                        )),
                    }
                } else {
                    CoreExprChirho::LitChirho(core_lit_chirho)
                }
            }

            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                let desugared_fun_chirho = self.desugar_expr_chirho(fun_chirho);
                let desugared_arg_chirho = self.desugar_expr_chirho(arg_chirho);
                // If applying to a constructor, accumulate into ConAppChirho
                match desugared_fun_chirho {
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho,
                        mut args_chirho,
                    } => {
                        args_chirho.push(desugared_arg_chirho);
                        CoreExprChirho::ConAppChirho {
                            con_name_chirho,
                            args_chirho,
                        }
                    }
                    // Detect prefix application of binary primops:
                    // App(App(Var("div"), a), b) → PrimOp("div#", [a, b])
                    CoreExprChirho::AppChirho {
                        fun_chirho: ref inner_fun_chirho,
                        arg_chirho: ref first_arg_chirho,
                    } => {
                        if let CoreExprChirho::VarChirho(id_chirho) = inner_fun_chirho.as_ref() {
                            if let Some(name_chirho) = self.names_chirho.get(id_chirho) {
                                if let Some(primop_chirho) = prefix_binary_primop_name_chirho(name_chirho) {
                                    return CoreExprChirho::PrimOpChirho {
                                        name_chirho: primop_chirho.to_string(),
                                        args_chirho: vec![
                                            first_arg_chirho.as_ref().clone(),
                                            desugared_arg_chirho,
                                        ],
                                    };
                                }
                            }
                        }
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(desugared_fun_chirho),
                            arg_chirho: Box::new(desugared_arg_chirho),
                        }
                    }
                    _ => CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(desugared_fun_chirho),
                        arg_chirho: Box::new(desugared_arg_chirho),
                    },
                }
            }

            ExprChirho::LamChirho {
                pats_chirho,
                body_chirho,
                ..
            } => {
                // Create binders and bind them in scope before desugaring body.
                // For non-variable patterns (tuples, constructors, etc.), we
                // bind a fresh name and wrap the body in a case expression to
                // destructure the argument.
                self.push_scope_chirho();

                let mut complex_pats_chirho: Vec<(BinderChirho, &PatChirho)> = Vec::new();
                let binders_chirho: Vec<BinderChirho> = pats_chirho
                    .iter()
                    .enumerate()
                    .map(|(i_chirho, pat_chirho)| {
                        let name_chirho = match pat_chirho {
                            PatChirho::VarChirho(n_chirho) => n_chirho.text_chirho().to_string(),
                            PatChirho::WildcardChirho { .. } => format!("_lam{}", i_chirho),
                            _ => format!("_lam{}", i_chirho),
                        };
                        let binder_chirho = self.fresh_binder_chirho(
                            &name_chirho,
                            TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        match pat_chirho {
                            PatChirho::VarChirho(n_chirho) => {
                                self.bind_in_scope_chirho(
                                    &n_chirho.text_chirho(),
                                    binder_chirho.id_chirho,
                                );
                            }
                            PatChirho::WildcardChirho { .. } => {}
                            _ => {
                                // Non-trivial pattern: record for wrapping
                                complex_pats_chirho
                                    .push((binder_chirho.clone(), pat_chirho));
                            }
                        }
                        binder_chirho
                    })
                    .collect();

                // For complex patterns, create binders FIRST (via
                // pat_to_binders_chirho which also binds them in scope),
                // then desugar the body so body references match alt
                // binder IDs.
                let mut pat_binder_info_chirho: Vec<(
                    &BinderChirho,
                    &PatChirho,
                    Vec<BinderChirho>,
                )> = Vec::new();
                for (binder_chirho, pat_chirho) in &complex_pats_chirho {
                    let alt_binders_chirho =
                        self.pat_to_binders_chirho(pat_chirho);
                    pat_binder_info_chirho.push((
                        binder_chirho,
                        pat_chirho,
                        alt_binders_chirho,
                    ));
                }

                let mut result_chirho = self.desugar_expr_chirho(body_chirho);

                // Wrap body in case expressions for complex patterns
                // (innermost first, so process in reverse)
                for (binder_chirho, pat_chirho, alt_binders_chirho) in
                    pat_binder_info_chirho.iter().rev()
                {
                    let con_chirho = self.pat_to_alt_con_chirho(pat_chirho);
                    // Wrap with nested cases for sub-patterns
                    let rhs_nested_chirho = self.wrap_nested_cases_chirho(
                        result_chirho,
                        alt_binders_chirho,
                        pat_chirho,
                    );
                    let rhs_final_chirho = self.wrap_as_bindings_chirho(
                        rhs_nested_chirho,
                        pat_chirho,
                        binder_chirho.id_chirho,
                    );
                    let wild_chirho = self.fresh_binder_chirho(
                        "wild",
                        TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    result_chirho = CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                            binder_chirho.id_chirho,
                        )),
                        bind_chirho: wild_chirho,
                        result_ty_chirho: TyChirho::VarChirho(
                            rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                        ),
                        alts_chirho: vec![CoreAltChirho {
                            con_chirho,
                            binders_chirho: alt_binders_chirho.clone(),
                            rhs_chirho: rhs_final_chirho,
                        }],
                    };
                }

                self.pop_scope_chirho();

                for binder_chirho in binders_chirho.into_iter().rev() {
                    result_chirho = CoreExprChirho::LamChirho {
                        binder_chirho,
                        body_chirho: Box::new(result_chirho),
                    };
                }
                result_chirho
            }

            ExprChirho::IfChirho {
                cond_chirho,
                then_chirho,
                else_chirho,
                ..
            } => {
                // if c then t else e → case c of { True -> t; False -> e }
                let cond_core_chirho = self.desugar_expr_chirho(cond_chirho);
                let then_core_chirho = self.desugar_expr_chirho(then_chirho);
                let else_core_chirho = self.desugar_expr_chirho(else_chirho);
                let wild_chirho = self.fresh_binder_chirho(
                    "wild",
                    TyChirho::bool_chirho(),
                    SpanChirho::DUMMY_CHIRHO,
                );
                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(cond_core_chirho),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: then_core_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("False".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: else_core_chirho,
                        },
                    ],
                }
            }

            ExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                self.push_scope_chirho();

                // Separate function bindings (which become Core let) from
                // pattern bindings (which become case expressions wrapping
                // the body).
                let mut fun_bind_indices_chirho: Vec<usize> = Vec::new();
                let mut pat_bind_indices_chirho: Vec<usize> = Vec::new();
                for (idx_chirho, bind_chirho) in binds_chirho.iter().enumerate() {
                    match bind_chirho {
                        rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho { .. } => {
                            fun_bind_indices_chirho.push(idx_chirho);
                        }
                        rhasky_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                            pat_chirho,
                            ..
                        } => {
                            // Simple variable patterns can be handled as
                            // regular let bindings (equivalent to 0-arg
                            // function binding).
                            if matches!(pat_chirho, PatChirho::VarChirho(_)) {
                                fun_bind_indices_chirho.push(idx_chirho);
                            } else {
                                pat_bind_indices_chirho.push(idx_chirho);
                            }
                        }
                        _ => {}
                    }
                }

                // Pre-bind pattern variables from complex pattern bindings
                // so the body and other bindings can reference them.
                // Store binders for reuse when building the case alt later.
                let mut pat_binders_map_chirho: std::collections::HashMap<usize, Vec<BinderChirho>> =
                    std::collections::HashMap::new();
                for &idx_chirho in &pat_bind_indices_chirho {
                    if let rhasky_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                        pat_chirho,
                        ..
                    } = &binds_chirho[idx_chirho]
                    {
                        self.prebind_all_pat_vars_chirho(pat_chirho);
                        // Create binders and bind them in scope — these same
                        // binders will be reused in the case alt.
                        let top_binders_chirho = self.pat_to_binders_chirho(pat_chirho);
                        for b_chirho in &top_binders_chirho {
                            self.bind_in_scope_chirho(
                                &b_chirho.name_chirho,
                                b_chirho.id_chirho,
                            );
                        }
                        pat_binders_map_chirho.insert(idx_chirho, top_binders_chirho);
                    }
                }

                // Pass 1: Pre-bind function binding names.
                let mut pre_binders_chirho: Vec<(usize, BinderChirho)> = Vec::new();
                for &idx_chirho in &fun_bind_indices_chirho {
                    match &binds_chirho[idx_chirho] {
                        rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                            name_chirho,
                            span_chirho,
                            ..
                        } => {
                            let binder_chirho = self.fresh_binder_chirho(
                                name_chirho.text_chirho(),
                                TyChirho::VarChirho(
                                    rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                *span_chirho,
                            );
                            self.bind_in_scope_chirho(
                                name_chirho.text_chirho(),
                                binder_chirho.id_chirho,
                            );
                            pre_binders_chirho.push((idx_chirho, binder_chirho));
                        }
                        // Simple var PatBindChirho treated as fun bind
                        rhasky_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                            pat_chirho: PatChirho::VarChirho(name_chirho),
                            span_chirho,
                            ..
                        } => {
                            let binder_chirho = self.fresh_binder_chirho(
                                name_chirho.text_chirho(),
                                TyChirho::VarChirho(
                                    rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                *span_chirho,
                            );
                            self.bind_in_scope_chirho(
                                name_chirho.text_chirho(),
                                binder_chirho.id_chirho,
                            );
                            pre_binders_chirho.push((idx_chirho, binder_chirho));
                        }
                        _ => {}
                    }
                }

                // Pass 2: Desugar function-binding RHSes.
                let mut core_binds_chirho: Vec<(BinderChirho, CoreExprChirho)> = Vec::new();
                for (idx_chirho, binder_chirho) in pre_binders_chirho {
                    let rhs_chirho = match &binds_chirho[idx_chirho] {
                        rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                            matches_chirho,
                            span_chirho,
                            ..
                        } => self.desugar_matches_chirho(matches_chirho, *span_chirho),
                        rhasky_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                            rhs_chirho,
                            ..
                        } => self.desugar_rhs_chirho(rhs_chirho),
                        _ => continue,
                    };
                    core_binds_chirho.push((binder_chirho, rhs_chirho));
                }

                // Desugar body, then wrap with case expressions for each
                // complex pattern binding:
                //   let (a, b) = rhs in body
                // becomes:
                //   case rhs of { (a, b) -> body }
                let mut result_body_chirho = self.desugar_expr_chirho(body_chirho);

                for &idx_chirho in pat_bind_indices_chirho.iter().rev() {
                    if let rhasky_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                        pat_chirho,
                        rhs_chirho,
                        ..
                    } = &binds_chirho[idx_chirho]
                    {
                        let core_scrut_chirho = self.desugar_rhs_chirho(rhs_chirho);
                        let con_chirho = self.pat_to_alt_con_chirho(pat_chirho);
                        // Reuse the binders created during pre-binding so
                        // that the case alt binders match the ids the body
                        // references.
                        let binders_chirho = pat_binders_map_chirho
                            .remove(&idx_chirho)
                            .unwrap_or_default();
                        let rhs_nested_chirho = self.wrap_nested_cases_chirho(
                            result_body_chirho,
                            &binders_chirho,
                            pat_chirho,
                        );
                        let wild_chirho = self.fresh_binder_chirho(
                            "_patscrut",
                            TyChirho::VarChirho(
                                rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                    self.next_id_chirho,
                                ),
                            ),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        result_body_chirho = CoreExprChirho::CaseChirho {
                            scrutinee_chirho: Box::new(core_scrut_chirho),
                            bind_chirho: wild_chirho,
                            result_ty_chirho: TyChirho::VarChirho(
                                rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                    self.next_id_chirho,
                                ),
                            ),
                            alts_chirho: vec![CoreAltChirho {
                                con_chirho,
                                binders_chirho,
                                rhs_chirho: rhs_nested_chirho,
                            }],
                        };
                    }
                }

                let result_chirho = if core_binds_chirho.is_empty() {
                    result_body_chirho
                } else {
                    let rec_chirho =
                        Self::is_recursive_binds_chirho(&core_binds_chirho);
                    CoreExprChirho::LetChirho {
                        rec_chirho,
                        binds_chirho: core_binds_chirho,
                        body_chirho: Box::new(result_body_chirho),
                    }
                };
                self.pop_scope_chirho();
                result_chirho
            }

            ExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                let scrut_chirho = self.desugar_expr_chirho(scrutinee_chirho);
                let wild_chirho = self.fresh_binder_chirho(
                    "wild",
                    TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );

                let core_alts_chirho: Vec<CoreAltChirho> = alts_chirho
                    .iter()
                    .map(|alt_chirho| {
                        self.push_scope_chirho();
                        let con_chirho = self.pat_to_alt_con_chirho(&alt_chirho.pat_chirho);
                        let binders_chirho = self.pat_to_binders_chirho(&alt_chirho.pat_chirho);
                        // Pre-bind ALL nested pattern variables so the
                        // RHS can reference them during desugaring.
                        self.prebind_all_pat_vars_chirho(&alt_chirho.pat_chirho);
                        let rhs_with_where_chirho = if alt_chirho.where_binds_chirho.is_empty() {
                            self.desugar_rhs_chirho(&alt_chirho.rhs_chirho)
                        } else {
                            self.push_scope_chirho();
                            let pre_b_chirho = self.prebind_where_names_chirho(
                                &alt_chirho.where_binds_chirho,
                            );
                            let body_chirho = self.desugar_rhs_chirho(&alt_chirho.rhs_chirho);
                            let binds_chirho = self.desugar_where_rhs_chirho(
                                &alt_chirho.where_binds_chirho,
                                pre_b_chirho,
                            );
                            let rec_chirho = Self::is_recursive_binds_chirho(&binds_chirho);
                            let result_chirho = CoreExprChirho::LetChirho {
                                rec_chirho,
                                binds_chirho,
                                body_chirho: Box::new(body_chirho),
                            };
                            self.pop_scope_chirho();
                            result_chirho
                        };
                        // Wrap RHS with nested case expressions for any
                        // nested constructor sub-patterns.
                        let rhs_nested_chirho = self.wrap_nested_cases_chirho(
                            rhs_with_where_chirho,
                            &binders_chirho,
                            &alt_chirho.pat_chirho,
                        );
                        // Wrap RHS with let-bindings for any as-pattern
                        // names: `xs@pat` binds `xs` to the scrutinee.
                        let rhs_final_chirho = self.wrap_as_bindings_chirho(
                            rhs_nested_chirho,
                            &alt_chirho.pat_chirho,
                            wild_chirho.id_chirho,
                        );
                        self.pop_scope_chirho();
                        CoreAltChirho {
                            con_chirho,
                            binders_chirho,
                            rhs_chirho: rhs_final_chirho,
                        }
                    })
                    .collect();

                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(scrut_chirho),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: core_alts_chirho,
                }
            }

            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => {
                if elements_chirho.is_empty() {
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)) // unit
                } else {
                    let arity_chirho = elements_chirho.len();
                    let con_args_chirho: Vec<CoreExprChirho> = elements_chirho
                        .iter()
                        .map(|elem_chirho| self.desugar_expr_chirho(elem_chirho))
                        .collect();
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: format!("$tuple{}", arity_chirho),
                        args_chirho: con_args_chirho,
                    }
                }
            }

            ExprChirho::ListChirho {
                elements_chirho, ..
            } => {
                // Desugar [a, b, c] → a : b : c : []
                let mut result_chirho = CoreExprChirho::ConAppChirho {
                    con_name_chirho: "[]".to_string(),
                    args_chirho: vec![],
                };
                for elem_chirho in elements_chirho.iter().rev() {
                    let elem_core_chirho = self.desugar_expr_chirho(elem_chirho);
                    result_chirho = CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![elem_core_chirho, result_chirho],
                    };
                }
                // OverloadedLists: list literals become `fromList [a, b, c]`
                if self.extensions_chirho.contains(&"OverloadedLists".to_string()) {
                    let from_list_id_chirho = self.resolve_var_chirho("fromList");
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(from_list_id_chirho)),
                        arg_chirho: Box::new(result_chirho),
                    }
                } else {
                    result_chirho
                }
            }

            ExprChirho::NegChirho { expr_chirho: inner_chirho, .. } => {
                // -x → negate# x (Int) or negateFloat# x (Double)
                let desugared_chirho = self.desugar_expr_chirho(inner_chirho);
                let primop_chirho = if is_float_core_chirho(&desugared_chirho) {
                    "negateFloat#"
                } else {
                    "negate#"
                };
                CoreExprChirho::PrimOpChirho {
                    name_chirho: primop_chirho.to_string(),
                    args_chirho: vec![desugared_chirho],
                }
            }

            ExprChirho::InfixChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                span_chirho,
            } => {
                let op_name_chirho = op_chirho.text_chirho();
                let left_core_chirho = self.desugar_expr_chirho(left_chirho);
                let right_core_chirho = self.desugar_expr_chirho(right_chirho);

                // ($) is function application: f $ x = f x
                if op_name_chirho == "$" {
                    return CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(left_core_chirho),
                        arg_chirho: Box::new(right_core_chirho),
                    };
                }

                // ($!) is strict application: f $! x = seq x (f x)
                // Desugars to: case x of { _ -> f x }
                if op_name_chirho == "$!" {
                    let wild_chirho = self.fresh_binder_chirho(
                        "_wild",
                        TyChirho::VarChirho(
                            rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                        ),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    return CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(right_core_chirho.clone()),
                        bind_chirho: wild_chirho,
                        result_ty_chirho: TyChirho::VarChirho(
                            rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho + 1),
                        ),
                        alts_chirho: vec![
                            crate::expr_chirho::CoreAltChirho {
                                con_chirho: AltConChirho::DefaultChirho,
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(left_core_chirho),
                                    arg_chirho: Box::new(right_core_chirho),
                                },
                            },
                        ],
                    };
                }

                // (.) is function composition: (f . g) x = f (g x)
                // At the expression level, f . g desugars to \x -> f (g x)
                if op_name_chirho == "." {
                    let x_id_chirho = self.fresh_id_chirho("x");
                    let x_binder_chirho = BinderChirho {
                        id_chirho: x_id_chirho,
                        name_chirho: "x".to_string(),
                        ty_chirho: TyChirho::VarChirho(
                            rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                        ),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    };
                    return CoreExprChirho::LamChirho {
                        binder_chirho: x_binder_chirho,
                        body_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(left_core_chirho),
                            arg_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(right_core_chirho),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_id_chirho)),
                            }),
                        }),
                    };
                }

                // (/=) desugars to not (==): x /= y → not (x == y)
                if op_name_chirho == "/=" {
                    let eq_id_chirho = self.resolve_var_chirho("==");
                    let not_id_chirho = self.resolve_var_chirho("not");
                    let eq_app_chirho = CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(eq_id_chirho)),
                            arg_chirho: Box::new(left_core_chirho),
                        }),
                        arg_chirho: Box::new(right_core_chirho),
                    };
                    return CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(not_id_chirho)),
                        arg_chirho: Box::new(eq_app_chirho),
                    };
                }

                // (>>) is monadic then: a >> b → PrimOp >>
                if op_name_chirho == ">>" {
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: ">>".to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // (>>=) is monadic bind: a >>= f → PrimOp >>=
                if op_name_chirho == ">>=" {
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: ">>=".to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // (:) is the list cons constructor: x : xs → ConApp(":", [x, xs])
                if op_name_chirho == ":" {
                    return CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // (&&) short-circuit: a && b → case a of { True → b; False → False }
                if op_name_chirho == "&&" {
                    let wild_chirho = self.fresh_binder_chirho(
                        "$wild",
                        TyChirho::bool_chirho(),
                        *span_chirho,
                    );
                    return CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(left_core_chirho),
                        bind_chirho: wild_chirho,
                        result_ty_chirho: TyChirho::bool_chirho(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("True".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: right_core_chirho,
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
                }

                // (||) short-circuit: a || b → case a of { True → True; False → b }
                if op_name_chirho == "||" {
                    let wild_chirho = self.fresh_binder_chirho(
                        "$wild",
                        TyChirho::bool_chirho(),
                        *span_chirho,
                    );
                    return CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(left_core_chirho),
                        bind_chirho: wild_chirho,
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
                                rhs_chirho: right_core_chirho,
                            },
                        ],
                    };
                }

                // (**) is always floating-point exponentiation
                if op_name_chirho == "**" {
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: "**#".to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // (^) is integer exponentiation (or float if operands are float)
                if op_name_chirho == "^" {
                    if is_float_core_chirho(&left_core_chirho) || is_float_core_chirho(&right_core_chirho) {
                        return CoreExprChirho::PrimOpChirho {
                            name_chirho: "**#".to_string(),
                            args_chirho: vec![left_core_chirho, right_core_chirho],
                        };
                    }
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: "^#".to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // Float-aware Num dispatch: when at least one operand is
                // a float literal, use the Double primop instead of the
                // default Int one.  This covers `1.5 + 2.5` etc.
                if matches!(op_name_chirho, "+" | "-" | "*" | "/")
                    && (is_float_core_chirho(&left_core_chirho)
                        || is_float_core_chirho(&right_core_chirho))
                {
                    let float_primop_chirho = match op_name_chirho {
                        "+" => "+.#",
                        "-" => "-.#",
                        "*" => "*.#",
                        "/" => "/.#",
                        _ => unreachable!(),
                    };
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: float_primop_chirho.to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // Check if this is a known primitive operator
                if let Some(primop_name_chirho) = builtin_primop_name_chirho(op_name_chirho) {
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: primop_name_chirho.to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // Otherwise: a `op` b → (op a) b
                let op_core_chirho = {
                    let id_chirho = self.resolve_var_chirho(op_name_chirho);
                    CoreExprChirho::VarChirho(id_chirho)
                };
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(op_core_chirho),
                        arg_chirho: Box::new(left_core_chirho),
                    }),
                    arg_chirho: Box::new(right_core_chirho),
                }
            }

            ExprChirho::LeftSectionChirho {
                op_chirho,
                arg_chirho,
                ..
            } => {
                // (op e) → \y -> y `op` e → \y -> op y e
                // For primop operators, emit PrimOp in the lambda body
                // so they work without dict-pass bindings.
                let op_name_chirho = op_chirho.text_chirho();
                let arg_core_chirho = self.desugar_expr_chirho(arg_chirho);
                let y_binder_chirho = self.fresh_binder_chirho(
                    "_sec",
                    TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let y_id_chirho = y_binder_chirho.id_chirho;
                let body_chirho = self.build_section_body_chirho(
                    op_name_chirho,
                    CoreExprChirho::VarChirho(y_id_chirho),
                    arg_core_chirho,
                );
                CoreExprChirho::LamChirho {
                    binder_chirho: y_binder_chirho,
                    body_chirho: Box::new(body_chirho),
                }
            }

            ExprChirho::RightSectionChirho {
                arg_chirho,
                op_chirho,
                ..
            } => {
                // (e op) → \y -> e `op` y → \y -> op e y
                let op_name_chirho = op_chirho.text_chirho();
                let arg_core_chirho = self.desugar_expr_chirho(arg_chirho);
                let y_binder_chirho = self.fresh_binder_chirho(
                    "_sec",
                    TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let y_id_chirho = y_binder_chirho.id_chirho;
                let body_chirho = self.build_section_body_chirho(
                    op_name_chirho,
                    arg_core_chirho,
                    CoreExprChirho::VarChirho(y_id_chirho),
                );
                CoreExprChirho::LamChirho {
                    binder_chirho: y_binder_chirho,
                    body_chirho: Box::new(body_chirho),
                }
            }

            ExprChirho::DoChirho {
                stmts_chirho, ..
            } => {
                // Desugar do notation:
                //   do { e }              → e
                //   do { e; stmts }       → e >> do { stmts }
                //   do { p <- e; stmts }  → e >>= \p -> do { stmts }
                //   do { let binds; stmts } → let binds in do { stmts }
                self.desugar_do_chirho(stmts_chirho)
            }

            ExprChirho::ArithSeqChirho {
                from_chirho,
                then_chirho,
                to_chirho,
                ..
            } => {
                // Desugar to calls to lazy Prelude functions (not primops)
                // so infinite lists like [1..] are properly lazy.
                //
                // [from ..]       → enumFrom from
                // [from,then ..]  → enumFromThen from then
                // [from .. to]    → enumFromTo from to
                // [from,then..to] → enumFromThenTo from then to
                let from_core_chirho = self.desugar_expr_chirho(from_chirho);
                match (then_chirho, to_chirho) {
                    (None, None) => {
                        let fun_id_chirho = self.resolve_var_chirho("enumFrom");
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_id_chirho)),
                            arg_chirho: Box::new(from_core_chirho),
                        }
                    }
                    (Some(then_e_chirho), None) => {
                        let then_core_chirho = self.desugar_expr_chirho(then_e_chirho);
                        let fun_id_chirho = self.resolve_var_chirho("enumFromThen");
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_id_chirho)),
                                arg_chirho: Box::new(from_core_chirho),
                            }),
                            arg_chirho: Box::new(then_core_chirho),
                        }
                    }
                    (None, Some(to_e_chirho)) => {
                        let to_core_chirho = self.desugar_expr_chirho(to_e_chirho);
                        let fun_id_chirho = self.resolve_var_chirho("enumFromTo");
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_id_chirho)),
                                arg_chirho: Box::new(from_core_chirho),
                            }),
                            arg_chirho: Box::new(to_core_chirho),
                        }
                    }
                    (Some(then_e_chirho), Some(to_e_chirho)) => {
                        let then_core_chirho = self.desugar_expr_chirho(then_e_chirho);
                        let to_core_chirho = self.desugar_expr_chirho(to_e_chirho);
                        let fun_id_chirho = self.resolve_var_chirho("enumFromThenTo");
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_id_chirho)),
                                    arg_chirho: Box::new(from_core_chirho),
                                }),
                                arg_chirho: Box::new(then_core_chirho),
                            }),
                            arg_chirho: Box::new(to_core_chirho),
                        }
                    }
                }
            }

            ExprChirho::ParenChirho { inner_chirho, .. } => {
                self.desugar_expr_chirho(inner_chirho)
            }

            ExprChirho::AnnChirho { expr_chirho: inner_chirho, .. } => {
                self.desugar_expr_chirho(inner_chirho)
            }

            ExprChirho::ListCompChirho {
                body_chirho,
                quals_chirho,
                ..
            } => {
                self.desugar_list_comp_chirho(body_chirho, quals_chirho)
            }

            ExprChirho::RecordConChirho {
                con_chirho,
                fields_chirho,
                ..
            } => {
                // Con { f1 = e1, f2 = e2 } → ConApp "Con" [e1, e2]
                let con_args_chirho: Vec<CoreExprChirho> = fields_chirho
                    .iter()
                    .map(|field_chirho| {
                        self.desugar_expr_chirho(&field_chirho.value_chirho)
                    })
                    .collect();
                CoreExprChirho::ConAppChirho {
                    con_name_chirho: con_chirho.text_chirho().to_string(),
                    args_chirho: con_args_chirho,
                }
            }

            ExprChirho::RecordUpdateChirho {
                expr_chirho: base_chirho,
                fields_chirho,
                ..
            } => {
                // record { f1 = e1 } → setField_f1 e1 record
                // Simplified: for now, produce a chain of setter applications.
                // A real implementation needs the data type layout.
                let base_core_chirho = self.desugar_expr_chirho(base_chirho);
                let mut result_chirho = base_core_chirho;
                for field_chirho in fields_chirho {
                    let setter_name_chirho =
                        format!("$setField_{}", field_chirho.name_chirho.text_chirho());
                    let setter_id_chirho = self.resolve_var_chirho(&setter_name_chirho);
                    let val_chirho = self.desugar_expr_chirho(&field_chirho.value_chirho);
                    result_chirho = CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(setter_id_chirho)),
                            arg_chirho: Box::new(val_chirho),
                        }),
                        arg_chirho: Box::new(result_chirho),
                    };
                }
                result_chirho
            }

            // Type application — erased during desugaring, just desugar the inner expr.
            ExprChirho::TypeAppChirho { expr_chirho: inner_chirho, .. } => {
                self.desugar_expr_chirho(inner_chirho)
            }
        }
    }

    /// Desugar list comprehension body + qualifiers.
    ///
    /// ```haskell
    /// [e | p <- xs, guard, let b = x, rest]
    /// ```
    /// becomes (recursively):
    /// - Generator: `concatMap (\p -> [e | rest]) xs`
    /// - Guard: `if guard then [e | rest] else []`
    /// - Let: `let binds in [e | rest]`
    /// - Empty quals: `[e]` (singleton list)
    fn desugar_list_comp_chirho(
        &mut self,
        body_chirho: &ExprChirho,
        quals_chirho: &[rhasky_ast_chirho::expr_chirho::StmtChirho],
    ) -> CoreExprChirho {
        use rhasky_ast_chirho::expr_chirho::StmtChirho;

        if quals_chirho.is_empty() {
            // Base case: [e] — singleton list (:) e []
            let body_core_chirho = self.desugar_expr_chirho(body_chirho);
            let nil_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "[]".to_string(),
                args_chirho: vec![],
            };
            return CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![body_core_chirho, nil_chirho],
            };
        }

        let rest_chirho = &quals_chirho[1..];

        match &quals_chirho[0] {
            StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho,
                ..
            } => {
                // p <- xs  →  letrec go = \$xs -> case $xs of
                //                [] -> []
                //                (:) p $rest -> <inner> ++ go $rest
                //   in go xs
                //
                // where <inner> is [e | rest_quals] (a list).
                //
                // For the common case of no remaining quals, <inner> is
                // the singleton [e], so the result is just (:) e (go $rest).
                // When there ARE remaining quals, <inner> can be empty (guards
                // may reject), so we need list append.  We inline a simple
                // recursive append to avoid needing a global concatMap.

                let xs_core_chirho = self.desugar_expr_chirho(expr_chirho);

                let var_name_chirho = match pat_chirho {
                    PatChirho::VarChirho(n_chirho) => n_chirho.text_chirho().to_string(),
                    _ => "_gen".to_string(),
                };

                // Create the recursive "go" function
                let go_name_chirho = format!("$lc_go_{}", self.next_id_chirho);
                let go_id_chirho = {
                    let id_chirho = CoreIdChirho(self.next_id_chirho);
                    self.next_id_chirho += 1;
                    self.names_chirho.insert(id_chirho, go_name_chirho.clone());
                    id_chirho
                };

                let list_ty_chirho = TyChirho::VarChirho(
                    rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                );

                // \$xs -> case $xs of ...
                let xs_binder_chirho = self.fresh_binder_chirho(
                    "$xs",
                    list_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let xs_id_chirho = xs_binder_chirho.id_chirho;

                let wild_chirho = self.fresh_binder_chirho(
                    "$w",
                    list_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                );

                // Element binder (p) and tail binder ($rest)
                self.push_scope_chirho();
                let elem_binder_chirho = self.fresh_binder_chirho(
                    &var_name_chirho,
                    TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                self.bind_in_scope_chirho(&var_name_chirho, elem_binder_chirho.id_chirho);

                let tail_binder_chirho = self.fresh_binder_chirho(
                    "$rest",
                    list_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                );

                // go $rest (recursive call on tail)
                let go_rest_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(go_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(tail_binder_chirho.id_chirho)),
                };

                // Build the cons-case RHS while var is in scope.
                // If rest_quals is empty, inner is a singleton [e] = (:) e [].
                // We optimize: (:) e (go $rest) directly.
                // If rest_quals is non-empty, we thread the continuation
                // (go $rest) as the nil-case of the inner loop, avoiding
                // the need for list append and nested letrecs with captures.
                let cons_rhs_chirho = if rest_chirho.is_empty() {
                    // Simple case: (:) e (go $rest) — body desugared here
                    // while the loop variable is still in scope.
                    let body_core_chirho = self.desugar_expr_chirho(body_chirho);
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![body_core_chirho, go_rest_chirho],
                    }
                } else {
                    // Multi-generator: thread the continuation through.
                    // [e | y <- ys, ...rest] with continuation k becomes:
                    //   letrec inner_go = \$ys -> case $ys of
                    //     [] -> k
                    //     (:) y $rest2 -> [e | ...rest] with cont (inner_go $rest2)
                    //   in inner_go ys
                    //
                    // For a single remaining bind qualifier y <- ys:
                    //   letrec inner_go = \$ys -> case $ys of
                    //     [] -> go $rest
                    //     (:) y $rest2 -> e : inner_go $rest2
                    //   in inner_go ys
                    self.desugar_list_comp_with_cont_chirho(
                        body_chirho,
                        rest_chirho,
                        go_rest_chirho,
                    )
                };
                self.pop_scope_chirho();

                // case $xs of { [] -> []; (:) p $rest -> <cons_rhs> }
                let case_body_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_id_chirho)),
                    bind_chirho: wild_chirho,
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
                            binders_chirho: vec![elem_binder_chirho, tail_binder_chirho],
                            rhs_chirho: cons_rhs_chirho,
                        },
                    ],
                };

                let go_body_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: xs_binder_chirho,
                    body_chirho: Box::new(case_body_chirho),
                };

                let go_binder_chirho = BinderChirho {
                    id_chirho: go_id_chirho,
                    name_chirho: go_name_chirho,
                    ty_chirho: list_ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                };

                // letrec go = \$xs -> case ... in go xs
                CoreExprChirho::LetChirho {
                    rec_chirho: true,
                    binds_chirho: vec![(go_binder_chirho, go_body_chirho)],
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(go_id_chirho)),
                        arg_chirho: Box::new(xs_core_chirho),
                    }),
                }
            }
            StmtChirho::ExprChirho(guard_expr_chirho) => {
                // Guard: if guard then [e | rest] else []
                let guard_core_chirho = self.desugar_expr_chirho(guard_expr_chirho);
                let then_chirho = self.desugar_list_comp_chirho(body_chirho, rest_chirho);
                let else_chirho = CoreExprChirho::ConAppChirho {
                    con_name_chirho: "[]".to_string(),
                    args_chirho: vec![],
                };

                let wild_chirho = self.fresh_binder_chirho(
                    "wild",
                    TyChirho::bool_chirho(),
                    SpanChirho::DUMMY_CHIRHO,
                );

                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(guard_core_chirho),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: then_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("False".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: else_chirho,
                        },
                    ],
                }
            }
            StmtChirho::LetChirho {
                binds_chirho, ..
            } => {
                // let binds in [e | rest]
                self.push_scope_chirho();
                let core_binds_chirho: Vec<(BinderChirho, CoreExprChirho)> = binds_chirho
                    .iter()
                    .filter_map(|bind_chirho| {
                        match bind_chirho {
                            rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                                name_chirho,
                                matches_chirho,
                                span_chirho,
                            } => {
                                let rhs_chirho =
                                    self.desugar_matches_chirho(matches_chirho, *span_chirho);
                                let binder_chirho = self.fresh_binder_chirho(
                                    name_chirho.text_chirho(),
                                    TyChirho::VarChirho(
                                        rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                            self.next_id_chirho,
                                        ),
                                    ),
                                    *span_chirho,
                                );
                                self.bind_in_scope_chirho(
                                    name_chirho.text_chirho(),
                                    binder_chirho.id_chirho,
                                );
                                Some((binder_chirho, rhs_chirho))
                            }
                            _ => None,
                        }
                    })
                    .collect();

                let inner_chirho = self.desugar_list_comp_chirho(body_chirho, rest_chirho);
                self.pop_scope_chirho();

                let rec_chirho =
                    Self::is_recursive_binds_chirho(&core_binds_chirho);
                CoreExprChirho::LetChirho {
                    rec_chirho,
                    binds_chirho: core_binds_chirho,
                    body_chirho: Box::new(inner_chirho),
                }
            }
        }
    }

    /// Desugar a list comprehension with a continuation expression.
    ///
    /// Instead of producing a standalone list and appending it to the
    /// continuation, this threads the continuation as the nil case of
    /// the innermost loop, avoiding the need for list append and
    /// the associated letrec-with-captures issue in the STG lowerer.
    ///
    /// `[e | q1, q2, ...] with continuation k` produces code where
    /// exhausting the list returns `k` instead of `[]`.
    fn desugar_list_comp_with_cont_chirho(
        &mut self,
        body_chirho: &ExprChirho,
        quals_chirho: &[rhasky_ast_chirho::expr_chirho::StmtChirho],
        cont_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        use rhasky_ast_chirho::expr_chirho::StmtChirho;

        if quals_chirho.is_empty() {
            // Base case: [e] with continuation k = (:) e k
            let body_core_chirho = self.desugar_expr_chirho(body_chirho);
            return CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![body_core_chirho, cont_chirho],
            };
        }

        let rest_chirho = &quals_chirho[1..];

        match &quals_chirho[0] {
            StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho,
                ..
            } => {
                // p <- xs with continuation k →
                //   letrec go = \$xs -> case $xs of
                //     [] -> k
                //     (:) p $rest -> [e | rest_quals] with cont (go $rest)
                //   in go xs

                let xs_core_chirho = self.desugar_expr_chirho(expr_chirho);

                let var_name_chirho = match pat_chirho {
                    PatChirho::VarChirho(n_chirho) => n_chirho.text_chirho().to_string(),
                    _ => "_gen".to_string(),
                };

                let go_name_chirho = format!("$lc_go_{}", self.next_id_chirho);
                let go_id_chirho = {
                    let id_chirho = CoreIdChirho(self.next_id_chirho);
                    self.next_id_chirho += 1;
                    self.names_chirho.insert(id_chirho, go_name_chirho.clone());
                    id_chirho
                };

                let list_ty_chirho = TyChirho::VarChirho(
                    rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                );

                let xs_binder_chirho = self.fresh_binder_chirho(
                    "$xs",
                    list_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let xs_id_chirho = xs_binder_chirho.id_chirho;

                let wild_chirho = self.fresh_binder_chirho(
                    "$w",
                    list_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                );

                self.push_scope_chirho();
                let elem_binder_chirho = self.fresh_binder_chirho(
                    &var_name_chirho,
                    TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                self.bind_in_scope_chirho(&var_name_chirho, elem_binder_chirho.id_chirho);

                let tail_binder_chirho = self.fresh_binder_chirho(
                    "$rest",
                    list_ty_chirho.clone(),
                    SpanChirho::DUMMY_CHIRHO,
                );

                let go_rest_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(go_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(
                        tail_binder_chirho.id_chirho,
                    )),
                };

                let cons_rhs_chirho = if rest_chirho.is_empty() {
                    let body_core_chirho = self.desugar_expr_chirho(body_chirho);
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![body_core_chirho, go_rest_chirho],
                    }
                } else {
                    self.desugar_list_comp_with_cont_chirho(
                        body_chirho,
                        rest_chirho,
                        go_rest_chirho,
                    )
                };
                self.pop_scope_chirho();

                let case_body_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_id_chirho)),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: list_ty_chirho.clone(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: cont_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho(":".to_string()),
                            binders_chirho: vec![elem_binder_chirho, tail_binder_chirho],
                            rhs_chirho: cons_rhs_chirho,
                        },
                    ],
                };

                let go_body_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: xs_binder_chirho,
                    body_chirho: Box::new(case_body_chirho),
                };

                let go_binder_chirho = BinderChirho {
                    id_chirho: go_id_chirho,
                    name_chirho: go_name_chirho,
                    ty_chirho: list_ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                };

                CoreExprChirho::LetChirho {
                    rec_chirho: true,
                    binds_chirho: vec![(go_binder_chirho, go_body_chirho)],
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(go_id_chirho)),
                        arg_chirho: Box::new(xs_core_chirho),
                    }),
                }
            }
            StmtChirho::ExprChirho(guard_expr_chirho) => {
                // Guard with continuation: if guard then [e | rest] with k else k
                let guard_core_chirho = self.desugar_expr_chirho(guard_expr_chirho);
                let then_chirho = self.desugar_list_comp_with_cont_chirho(
                    body_chirho,
                    rest_chirho,
                    cont_chirho.clone(),
                );

                let wild_chirho = self.fresh_binder_chirho(
                    "wild",
                    TyChirho::bool_chirho(),
                    SpanChirho::DUMMY_CHIRHO,
                );

                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(guard_core_chirho),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: then_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("False".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: cont_chirho,
                        },
                    ],
                }
            }
            StmtChirho::LetChirho {
                binds_chirho, span_chirho: _,
            } => {
                // let binds in [e | rest] with continuation k
                self.push_scope_chirho();
                let core_binds_chirho: Vec<(BinderChirho, CoreExprChirho)> = binds_chirho
                    .iter()
                    .filter_map(|bind_chirho| {
                        match bind_chirho {
                            rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                                name_chirho,
                                matches_chirho,
                                span_chirho,
                            } => {
                                let rhs_chirho =
                                    self.desugar_matches_chirho(matches_chirho, *span_chirho);
                                let binder_chirho = self.fresh_binder_chirho(
                                    name_chirho.text_chirho(),
                                    TyChirho::VarChirho(
                                        rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                            self.next_id_chirho,
                                        ),
                                    ),
                                    *span_chirho,
                                );
                                self.bind_in_scope_chirho(
                                    name_chirho.text_chirho(),
                                    binder_chirho.id_chirho,
                                );
                                Some((binder_chirho, rhs_chirho))
                            }
                            _ => None,
                        }
                    })
                    .collect();

                let inner_chirho = self.desugar_list_comp_with_cont_chirho(
                    body_chirho,
                    rest_chirho,
                    cont_chirho,
                );
                self.pop_scope_chirho();

                let rec_chirho =
                    Self::is_recursive_binds_chirho(&core_binds_chirho);
                CoreExprChirho::LetChirho {
                    rec_chirho,
                    binds_chirho: core_binds_chirho,
                    body_chirho: Box::new(inner_chirho),
                }
            }
        }
    }

    /// Build an inline list append: `xs ++ ys`.
    ///
    /// Produces:
    /// ```text
    /// letrec $append = \$as -> case $as of
    ///     [] -> ys
    ///     (:) $h $t -> (:) $h ($append $t)
    /// in $append xs
    /// ```
    #[allow(dead_code)]
    fn build_list_append_chirho(
        &mut self,
        xs_chirho: CoreExprChirho,
        ys_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let list_ty_chirho = TyChirho::VarChirho(
            rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
        );

        let append_name_chirho = format!("$lc_app_{}", self.next_id_chirho);
        let append_id_chirho = {
            let id_chirho = CoreIdChirho(self.next_id_chirho);
            self.next_id_chirho += 1;
            self.names_chirho.insert(id_chirho, append_name_chirho.clone());
            id_chirho
        };

        // Two-argument append: \$as $ys -> case $as of { ... }
        // This avoids capturing `ys` as a free variable which causes
        // issues with the STG letrec lowering inside function bodies.
        let as_binder_chirho = self.fresh_binder_chirho(
            "$as",
            list_ty_chirho.clone(),
            SpanChirho::DUMMY_CHIRHO,
        );
        let as_id_chirho = as_binder_chirho.id_chirho;

        let ys_binder_chirho = self.fresh_binder_chirho(
            "$ys",
            list_ty_chirho.clone(),
            SpanChirho::DUMMY_CHIRHO,
        );
        let ys_id_chirho = ys_binder_chirho.id_chirho;

        let wild_chirho = self.fresh_binder_chirho(
            "$w",
            list_ty_chirho.clone(),
            SpanChirho::DUMMY_CHIRHO,
        );

        let h_binder_chirho = self.fresh_binder_chirho(
            "$h",
            list_ty_chirho.clone(),
            SpanChirho::DUMMY_CHIRHO,
        );
        let t_binder_chirho = self.fresh_binder_chirho(
            "$t",
            list_ty_chirho.clone(),
            SpanChirho::DUMMY_CHIRHO,
        );

        // (:) $h (append $t $ys)
        let cons_rhs_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: ":".to_string(),
            args_chirho: vec![
                CoreExprChirho::VarChirho(h_binder_chirho.id_chirho),
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(
                            t_binder_chirho.id_chirho,
                        )),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(ys_id_chirho)),
                },
            ],
        };

        let case_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(as_id_chirho)),
            bind_chirho: wild_chirho,
            result_ty_chirho: list_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::VarChirho(ys_id_chirho),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![h_binder_chirho, t_binder_chirho],
                    rhs_chirho: cons_rhs_chirho,
                },
            ],
        };

        // \$as -> \$ys -> case $as of { ... }
        let append_body_chirho = CoreExprChirho::LamChirho {
            binder_chirho: as_binder_chirho,
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: ys_binder_chirho,
                body_chirho: Box::new(case_chirho),
            }),
        };

        let append_binder_chirho = BinderChirho {
            id_chirho: append_id_chirho,
            name_chirho: append_name_chirho,
            ty_chirho: list_ty_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        // letrec append = \$as $ys -> ... in append xs ys
        CoreExprChirho::LetChirho {
            rec_chirho: true,
            binds_chirho: vec![(append_binder_chirho, append_body_chirho)],
            body_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(xs_chirho),
                }),
                arg_chirho: Box::new(ys_chirho),
            }),
        }
    }

    /// Desugar guarded RHS into nested case on Bool.
    ///
    /// ```haskell
    /// | g1 = e1
    /// | g2 = e2
    /// | otherwise = e3
    /// ```text
    /// becomes:
    /// ```text
    /// case g1 of { True -> e1; _ -> case g2 of { True -> e2; _ -> e3 } }
    /// ```
    fn desugar_guards_chirho(
        &mut self,
        guards_chirho: &[rhasky_ast_chirho::expr_chirho::GuardedExprChirho],
    ) -> CoreExprChirho {
        if guards_chirho.is_empty() {
            // No guards: produce a runtime error (pattern match failure)
            let error_id_chirho = self.fresh_id_chirho("error");
            let msg_chirho =
                CoreExprChirho::LitChirho(CoreLitChirho::StringChirho("Non-exhaustive guards".to_string()));
            return CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(error_id_chirho)),
                arg_chirho: Box::new(msg_chirho),
            };
        }

        let guard_chirho = &guards_chirho[0];

        // `otherwise` is Prelude's `otherwise = True`; treat it as unconditional
        let is_otherwise_chirho = matches!(
            &guard_chirho.guard_chirho,
            rhasky_ast_chirho::expr_chirho::ExprChirho::VarChirho(n_chirho)
                if n_chirho.text_chirho() == "otherwise"
        );
        if is_otherwise_chirho {
            return self.desugar_expr_chirho(&guard_chirho.body_chirho);
        }

        let cond_core_chirho = self.desugar_expr_chirho(&guard_chirho.guard_chirho);
        let body_core_chirho = self.desugar_expr_chirho(&guard_chirho.body_chirho);
        let rest_core_chirho = self.desugar_guards_chirho(&guards_chirho[1..]);

        let wild_chirho = self.fresh_binder_chirho(
            "wild",
            TyChirho::bool_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        );

        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(cond_core_chirho),
            bind_chirho: wild_chirho,
            result_ty_chirho: TyChirho::VarChirho(
                rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
            ),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: body_core_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: rest_core_chirho,
                },
            ],
        }
    }

    /// Desugar do-notation statements into Core.
    fn desugar_do_chirho(
        &mut self,
        stmts_chirho: &[rhasky_ast_chirho::expr_chirho::StmtChirho],
    ) -> CoreExprChirho {
        use rhasky_ast_chirho::expr_chirho::StmtChirho;

        if stmts_chirho.is_empty() {
            return CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0));
        }

        if stmts_chirho.len() == 1 {
            return match &stmts_chirho[0] {
                StmtChirho::ExprChirho(expr_chirho) => {
                    self.desugar_expr_chirho(expr_chirho)
                }
                StmtChirho::BindChirho {
                    expr_chirho, ..
                } => self.desugar_expr_chirho(expr_chirho),
                StmtChirho::LetChirho { .. } => {
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
                }
            };
        }

        match &stmts_chirho[0] {
            StmtChirho::ExprChirho(expr_chirho) => {
                // e; stmts → case e of { _ -> do stmts }
                // Using case forces evaluation of e (for side effects)
                let e_chirho = self.desugar_expr_chirho(expr_chirho);
                let rest_chirho = self.desugar_do_chirho(&stmts_chirho[1..]);
                let seq_binder_chirho = self.fresh_binder_chirho(
                    "_seq",
                    TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    SpanChirho::DUMMY_CHIRHO,
                );
                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(e_chirho),
                    bind_chirho: seq_binder_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        rhasky_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: rest_chirho,
                    }],
                }
            }
            StmtChirho::BindChirho {
                pat_chirho,
                expr_chirho,
                ..
            } => {
                // p <- e; stmts → let p = e in (do stmts)
                let e_chirho = self.desugar_expr_chirho(expr_chirho);
                let var_name_chirho = match pat_chirho {
                    PatChirho::VarChirho(n_chirho) => n_chirho.text_chirho().to_string(),
                    _ => "_bind".to_string(),
                };
                let binder_chirho = self.fresh_binder_chirho(
                    &var_name_chirho,
                    TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                self.bind_in_scope_chirho(&var_name_chirho, binder_chirho.id_chirho);
                // Compute rest AFTER binding is in scope so subsequent
                // statements can reference the bound variable.
                let rest_chirho = self.desugar_do_chirho(&stmts_chirho[1..]);
                CoreExprChirho::LetChirho {
                    rec_chirho: false,
                    binds_chirho: vec![(binder_chirho, e_chirho)],
                    body_chirho: Box::new(rest_chirho),
                }
            }
            StmtChirho::LetChirho {
                binds_chirho, ..
            } => {
                // let binds; stmts → let binds in do stmts
                self.push_scope_chirho();
                let core_binds_chirho: Vec<(BinderChirho, CoreExprChirho)> = binds_chirho
                    .iter()
                    .filter_map(|bind_chirho| {
                        match bind_chirho {
                            rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                                name_chirho,
                                matches_chirho,
                                span_chirho,
                            } => {
                                let rhs_chirho =
                                    self.desugar_matches_chirho(matches_chirho, *span_chirho);
                                let binder_chirho = self.fresh_binder_chirho(
                                    name_chirho.text_chirho(),
                                    TyChirho::VarChirho(
                                        rhasky_typing_chirho::ty_chirho::TyVarChirho(
                                            self.next_id_chirho,
                                        ),
                                    ),
                                    *span_chirho,
                                );
                                self.bind_in_scope_chirho(
                                    name_chirho.text_chirho(),
                                    binder_chirho.id_chirho,
                                );
                                Some((binder_chirho, rhs_chirho))
                            }
                            _ => None,
                        }
                    })
                    .collect();

                // Compute rest AFTER bindings are in scope so subsequent
                // do-statements can reference the let-bound variables.
                let rest_chirho = self.desugar_do_chirho(&stmts_chirho[1..]);
                let rec_chirho =
                    Self::is_recursive_binds_chirho(&core_binds_chirho);
                let result_chirho = CoreExprChirho::LetChirho {
                    rec_chirho,
                    binds_chirho: core_binds_chirho,
                    body_chirho: Box::new(rest_chirho),
                };
                self.pop_scope_chirho();
                result_chirho
            }
        }
    }

    fn desugar_lit_chirho(&self, lit_chirho: &LitChirho) -> CoreLitChirho {
        match lit_chirho {
            LitChirho::IntChirho(v_chirho, _) => CoreLitChirho::IntChirho(*v_chirho),
            LitChirho::FloatChirho(v_chirho, _) => CoreLitChirho::FloatChirho(*v_chirho),
            LitChirho::CharChirho(v_chirho, _) => CoreLitChirho::CharChirho(*v_chirho),
            LitChirho::StringChirho(v_chirho, _) => CoreLitChirho::StringChirho(v_chirho.clone()),
        }
    }
}

/// Check if all patterns are simple variable patterns (including bang/lazy
/// wrappers around variables).
fn all_var_pats_chirho(pats_chirho: &[PatChirho]) -> bool {
    pats_chirho.iter().all(|p_chirho| is_var_pat_chirho(p_chirho))
}

/// Check if a single pattern is a simple variable (peeling bang/lazy/paren).
fn is_var_pat_chirho(pat_chirho: &PatChirho) -> bool {
    match pat_chirho {
        PatChirho::VarChirho(_) => true,
        PatChirho::BangChirho { inner_chirho, .. }
        | PatChirho::LazyChirho { inner_chirho, .. }
        | PatChirho::ParenChirho { inner_chirho, .. } => is_var_pat_chirho(inner_chirho),
        _ => false,
    }
}

/// Check if a pattern is a bang pattern (including through paren wrappers).
fn is_bang_pat_chirho(pat_chirho: &PatChirho) -> bool {
    match pat_chirho {
        PatChirho::BangChirho { .. } => true,
        PatChirho::ParenChirho { inner_chirho, .. } => is_bang_pat_chirho(inner_chirho),
        _ => false,
    }
}

/// Extract the variable name from a pattern, peeling through bang/lazy/paren.
fn extract_var_name_from_pat_chirho(pat_chirho: &PatChirho) -> Option<String> {
    match pat_chirho {
        PatChirho::VarChirho(n_chirho) => Some(n_chirho.text_chirho().to_string()),
        PatChirho::BangChirho { inner_chirho, .. }
        | PatChirho::LazyChirho { inner_chirho, .. }
        | PatChirho::ParenChirho { inner_chirho, .. } => {
            extract_var_name_from_pat_chirho(inner_chirho)
        }
        _ => None,
    }
}

/// Desugar a module from AST to Core, returning the Core module and a name map.
pub fn desugar_module_chirho(module_chirho: &ModuleChirho) -> DesugarOutputChirho {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    ctx_chirho.extensions_chirho = module_chirho.extensions_chirho.clone();
    ctx_chirho.desugar_module_chirho(module_chirho)
}

/// Map a source-level operator name to a primop name, if it is a known
/// built-in primitive. Returns `None` for user-defined operators.
///
/// Note: `==` and `/=` are intentionally NOT listed here — they are Eq
/// class methods and must go through the dictionary-passing transform so
/// that the correct instance (e.g. `Eq Color` vs `Eq Int`) is selected
/// Map infix operator names to their primitive machine operation names.
///
/// `+`, `-`, `*` are Num class methods and go through the dictionary-
/// passing transform instead of being lowered to primops directly.
/// `==`, `/=` are Eq class methods and handled separately in the
/// desugarer. The Ord operators remain as primops for now.
fn builtin_primop_name_chirho(op_chirho: &str) -> Option<&'static str> {
    match op_chirho {
        // +, -, * are handled by the Num class dict transform
        // / is handled by the Fractional class dict transform
        "div" => Some("div#"),
        "mod" => Some("mod#"),
        "<" => Some("<#"),
        "<=" => Some("<=#"),
        ">" => Some(">#"),
        ">=" => Some(">=#"),
        "++" => Some("++#"),
        _ => None,
    }
}

/// Map prefix function names to binary primop names.
/// These are functions like `div` and `mod` that can be called in
/// prefix position: `div a b` or `mod a b`.
fn prefix_binary_primop_name_chirho(name_chirho: &str) -> Option<&'static str> {
    match name_chirho {
        "div" => Some("div#"),
        "mod" => Some("mod#"),
        "quot" => Some("quot#"),
        "rem" => Some("rem#"),
        _ => None,
    }
}

/// Check whether a Core expression is a float literal (used for
/// type-directed Num dispatch at the desugaring level).
fn is_float_core_chirho(expr_chirho: &CoreExprChirho) -> bool {
    matches!(
        expr_chirho,
        CoreExprChirho::LitChirho(crate::expr_chirho::CoreLitChirho::FloatChirho(_))
    )
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhasky_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

    fn dummy_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    #[test]
    fn desugar_simple_function_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
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
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let output_chirho = desugar_module_chirho(&module_chirho);
        let core_chirho = &output_chirho.module_chirho;
        assert_eq!(core_chirho.name_chirho, "Test");
        assert_eq!(core_chirho.bindings_chirho.len(), 1);
        assert_eq!(core_chirho.bindings_chirho[0].binder_chirho.name_chirho, "f");
        assert!(matches!(
            core_chirho.bindings_chirho[0].rhs_chirho,
            CoreExprChirho::LamChirho { .. }
        ));
        // Name map should contain the binder's name
        assert!(output_chirho.names_chirho.values().any(|n_chirho| n_chirho == "f"));
    }

    #[test]
    fn desugar_if_to_case_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::IfChirho {
            cond_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
            then_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                1,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            else_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                0,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        assert!(matches!(core_chirho, CoreExprChirho::CaseChirho { .. }));
        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
            assert_eq!(alts_chirho.len(), 2);
            assert!(matches!(
                alts_chirho[0].con_chirho,
                AltConChirho::DataConChirho(ref s_chirho) if s_chirho == "True"
            ));
        }
    }

    #[test]
    fn desugar_literal_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO));
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // Integer literals are wrapped with fromInteger for numeric
        // overloading: `42` → `fromInteger 42`
        match &core_chirho {
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                assert!(matches!(fun_chirho.as_ref(), CoreExprChirho::VarChirho(_)));
                assert_eq!(
                    **arg_chirho,
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42))
                );
            }
            other_chirho => panic!(
                "expected App(fromInteger, 42), got {:?}",
                other_chirho
            ),
        }
    }

    #[test]
    fn desugar_lambda_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::LamChirho {
            pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
            body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        assert!(matches!(core_chirho, CoreExprChirho::LamChirho { .. }));
    }

    #[test]
    fn desugar_list_to_cons_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::ListChirho {
            elements_chirho: vec![
                ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO)),
                ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO)),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // Should be nested ConApp(`:`, [1, ConApp(`:`, [2, ConApp(`[]`, [])])])
        assert!(matches!(core_chirho, CoreExprChirho::ConAppChirho { .. }));
    }

    #[test]
    fn desugar_pretty_prints_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("Pretty"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("main"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                        LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO),
                    )),
                    where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let desugar_out_chirho = desugar_module_chirho(&module_chirho);
        let output_chirho = crate::pretty_chirho::pretty_module_chirho(&desugar_out_chirho.module_chirho);
        assert!(output_chirho.contains("-- module Pretty"));
        assert!(output_chirho.contains("main"));
        assert!(output_chirho.contains("0"));
    }

    #[test]
    fn desugar_infix_to_primop_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // a + b (Int operands) → App(App(Var(+), fromInteger(1)), fromInteger(2))
        // +, -, * go through the dict transform for type-aware dispatch
        let expr_chirho = ExprChirho::InfixChirho {
            left_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                1,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            op_chirho: dummy_name_chirho("+"),
            right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                2,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // +, -, * are class methods (Num), so they desugar to App chains
        assert!(matches!(core_chirho, CoreExprChirho::AppChirho { .. }));

        // Float operands should use the float primop directly
        let float_expr_chirho = ExprChirho::InfixChirho {
            left_chirho: Box::new(ExprChirho::LitChirho(LitChirho::FloatChirho(
                1.5,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            op_chirho: dummy_name_chirho("+"),
            right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::FloatChirho(
                2.5,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let float_core_chirho = ctx_chirho.desugar_expr_chirho(&float_expr_chirho);
        assert!(matches!(float_core_chirho, CoreExprChirho::PrimOpChirho { .. }));
        if let CoreExprChirho::PrimOpChirho { name_chirho, .. } = &float_core_chirho {
            assert_eq!(name_chirho, "+.#");
        }
    }

    #[test]
    fn desugar_infix_user_op_to_app_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // a `foo` b → App(App(Var(foo), a), b)  — non-builtin operator
        let expr_chirho = ExprChirho::InfixChirho {
            left_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                1,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            op_chirho: dummy_name_chirho("foo"),
            right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                2,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // Non-builtin should still produce App
        assert!(matches!(core_chirho, CoreExprChirho::AppChirho { .. }));
    }

    #[test]
    fn desugar_left_section_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // (+ 1) → \y -> (+) y 1  -- Wait, no, LeftSection is (op arg): \y -> op arg y
        // Actually per Haskell: (+ 1) means \y -> (+) y 1 (right section)
        // But in our AST LeftSection is (op arg): the operator comes first.
        // Let's just check it produces a lambda.
        let expr_chirho = ExprChirho::LeftSectionChirho {
            op_chirho: dummy_name_chirho("+"),
            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                1,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        assert!(matches!(core_chirho, CoreExprChirho::LamChirho { .. }));
    }

    #[test]
    fn desugar_right_section_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::RightSectionChirho {
            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                1,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            op_chirho: dummy_name_chirho("+"),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        assert!(matches!(core_chirho, CoreExprChirho::LamChirho { .. }));
    }

    #[test]
    fn desugar_guards_chirho() {
        use rhasky_ast_chirho::expr_chirho::GuardedExprChirho;

        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        let rhs_chirho = RhsChirho::GuardedChirho(vec![
            GuardedExprChirho {
                guard_chirho: ExprChirho::ConChirho(dummy_name_chirho("True")),
                body_chirho: ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO)),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            GuardedExprChirho {
                guard_chirho: ExprChirho::ConChirho(dummy_name_chirho("True")),
                body_chirho: ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO)),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]);
        let core_chirho = ctx_chirho.desugar_rhs_chirho(&rhs_chirho);
        // Should be nested case: case True of { True -> 1; _ -> case True of { True -> 2; _ -> error } }
        assert!(matches!(core_chirho, CoreExprChirho::CaseChirho { .. }));
        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
            assert_eq!(alts_chirho.len(), 2); // True branch + default
            // Default branch should be another case
            assert!(matches!(alts_chirho[1].rhs_chirho, CoreExprChirho::CaseChirho { .. }));
        }
    }

    #[test]
    fn desugar_do_notation_chirho() {
        use rhasky_ast_chirho::expr_chirho::StmtChirho;

        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // do { x <- getLine; putStrLn x }
        let expr_chirho = ExprChirho::DoChirho {
            stmts_chirho: vec![
                StmtChirho::BindChirho {
                    pat_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
                    expr_chirho: ExprChirho::VarChirho(dummy_name_chirho("getLine")),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                StmtChirho::ExprChirho(ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("putStrLn"))),
                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // Should be let x = getLine in (putStrLn x)
        assert!(matches!(core_chirho, CoreExprChirho::LetChirho { .. }));
    }

    #[test]
    fn desugar_arith_seq_from_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // [1..]
        let expr_chirho = ExprChirho::ArithSeqChirho {
            from_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                1,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            then_chirho: None,
            to_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // Should be App(enumFrom, 1) — a function call, not a primop
        assert!(matches!(core_chirho, CoreExprChirho::AppChirho { .. }));
    }

    #[test]
    fn desugar_where_binds_chirho() {
        use rhasky_ast_chirho::expr_chirho::LocalBindChirho;

        // f x = y where y = x
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("f"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                        dummy_name_chirho("y"),
                    )),
                    where_binds_chirho: vec![LocalBindChirho::FunBindChirho {
                        name_chirho: dummy_name_chirho("y"),
                        matches_chirho: vec![MatchArmChirho {
                            pats_chirho: vec![],
                            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                                dummy_name_chirho("x"),
                            )),
                            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let output_chirho = desugar_module_chirho(&module_chirho);
        let binding_chirho = &output_chirho.module_chirho.bindings_chirho[0];
        assert_eq!(binding_chirho.binder_chirho.name_chirho, "f");

        // The body should be Lam { body: Let { ... } }
        if let CoreExprChirho::LamChirho { body_chirho, .. } = &binding_chirho.rhs_chirho {
            assert!(
                matches!(**body_chirho, CoreExprChirho::LetChirho { .. }),
                "expected where clause to produce a Let, got {:?}",
                body_chirho
            );
        } else {
            panic!("expected LamChirho");
        }
    }

    #[test]
    fn desugar_record_con_chirho() {
        use rhasky_ast_chirho::expr_chirho::FieldAssignChirho;

        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // MkPoint { x = 1, y = 2 }
        let expr_chirho = ExprChirho::RecordConChirho {
            con_chirho: dummy_name_chirho("MkPoint"),
            fields_chirho: vec![
                FieldAssignChirho {
                    name_chirho: dummy_name_chirho("x"),
                    value_chirho: ExprChirho::LitChirho(LitChirho::IntChirho(
                        1,
                        SpanChirho::DUMMY_CHIRHO,
                    )),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                FieldAssignChirho {
                    name_chirho: dummy_name_chirho("y"),
                    value_chirho: ExprChirho::LitChirho(LitChirho::IntChirho(
                        2,
                        SpanChirho::DUMMY_CHIRHO,
                    )),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // Should be ConApp("MkPoint", [1, 2])
        assert!(matches!(core_chirho, CoreExprChirho::ConAppChirho { .. }));
        if let CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho,
        } = &core_chirho
        {
            assert_eq!(con_name_chirho, "MkPoint");
            assert_eq!(args_chirho.len(), 2);
            // Args are fromInteger-wrapped: App(fromInteger, Lit(2))
            assert!(matches!(
                &args_chirho[1],
                CoreExprChirho::AppChirho { arg_chirho, .. }
                    if matches!(arg_chirho.as_ref(), CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)))
            ));
        }
    }

    #[test]
    fn desugar_record_update_chirho() {
        use rhasky_ast_chirho::expr_chirho::FieldAssignChirho;

        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // p { x = 10 }
        let expr_chirho = ExprChirho::RecordUpdateChirho {
            expr_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("p"))),
            fields_chirho: vec![FieldAssignChirho {
                name_chirho: dummy_name_chirho("x"),
                value_chirho: ExprChirho::LitChirho(LitChirho::IntChirho(
                    10,
                    SpanChirho::DUMMY_CHIRHO,
                )),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // Should produce an application involving $setField_x
        assert!(matches!(core_chirho, CoreExprChirho::AppChirho { .. }));
    }

    #[test]
    fn desugar_list_comp_chirho() {
        use rhasky_ast_chirho::expr_chirho::StmtChirho;

        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // [x | x <- xs]
        let expr_chirho = ExprChirho::ListCompChirho {
            body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            quals_chirho: vec![StmtChirho::BindChirho {
                pat_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
                expr_chirho: ExprChirho::VarChirho(dummy_name_chirho("xs")),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // [x | x <- xs] → letrec go = \$xs -> case $xs of ... in go xs
        assert!(matches!(core_chirho, CoreExprChirho::LetChirho { rec_chirho: true, .. }));
    }

    #[test]
    fn desugar_list_comp_with_guard_chirho() {
        use rhasky_ast_chirho::expr_chirho::StmtChirho;

        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // [x | x <- xs, even x]
        let expr_chirho = ExprChirho::ListCompChirho {
            body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            quals_chirho: vec![
                StmtChirho::BindChirho {
                    pat_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
                    expr_chirho: ExprChirho::VarChirho(dummy_name_chirho("xs")),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                StmtChirho::ExprChirho(ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("even"))),
                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // [x | x <- xs, even x] → letrec go = \$xs -> case $xs of { ... } in go xs
        // The generator produces a LetChirho wrapping the recursive go function
        assert!(matches!(core_chirho, CoreExprChirho::LetChirho { rec_chirho: true, .. }));
    }

    #[test]
    fn desugar_arith_seq_from_to_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // [1..10]
        let expr_chirho = ExprChirho::ArithSeqChirho {
            from_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                1,
                SpanChirho::DUMMY_CHIRHO,
            ))),
            then_chirho: None,
            to_chirho: Some(Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                10,
                SpanChirho::DUMMY_CHIRHO,
            )))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // Should be App(App(enumFromTo, 1), 10) — a function call, not a primop
        assert!(matches!(core_chirho, CoreExprChirho::AppChirho { .. }));
    }

    // -------------------------------------------------------------------
    // Constructor field binding tests
    // -------------------------------------------------------------------

    #[test]
    fn desugar_case_con_binders_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // case x of { Just a -> a; Nothing -> 0 }
        let expr_chirho = ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            alts_chirho: vec![
                rhasky_ast_chirho::expr_chirho::AltChirho {
                    pat_chirho: PatChirho::ConChirho {
                        con_chirho: dummy_name_chirho("Just"),
                        args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("a"))],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                        dummy_name_chirho("a"),
                    )),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhasky_ast_chirho::expr_chirho::AltChirho {
                    pat_chirho: PatChirho::ConChirho {
                        con_chirho: dummy_name_chirho("Nothing"),
                        args_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                        LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO),
                    )),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
            assert_eq!(alts_chirho.len(), 2);
            // Just a → binders should have 1 binder named "a"
            assert_eq!(alts_chirho[0].binders_chirho.len(), 1);
            assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "a");
            assert!(matches!(
                alts_chirho[0].con_chirho,
                AltConChirho::DataConChirho(ref s_chirho) if s_chirho == "Just"
            ));
            // Nothing → no binders
            assert_eq!(alts_chirho[1].binders_chirho.len(), 0);
            // The RHS of the Just branch should reference "a"'s id
            let a_id_chirho = alts_chirho[0].binders_chirho[0].id_chirho;
            assert!(matches!(
                alts_chirho[0].rhs_chirho,
                CoreExprChirho::VarChirho(id_chirho) if id_chirho == a_id_chirho
            ));
        } else {
            panic!("expected CaseChirho, got {:?}", core_chirho);
        }
    }

    #[test]
    fn desugar_case_multi_field_binders_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // case p of { MkPoint x y -> x }
        let expr_chirho = ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("p"))),
            alts_chirho: vec![rhasky_ast_chirho::expr_chirho::AltChirho {
                pat_chirho: PatChirho::ConChirho {
                    con_chirho: dummy_name_chirho("MkPoint"),
                    args_chirho: vec![
                        PatChirho::VarChirho(dummy_name_chirho("x")),
                        PatChirho::VarChirho(dummy_name_chirho("y")),
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                    dummy_name_chirho("x"),
                )),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
            assert_eq!(alts_chirho.len(), 1);
            assert_eq!(alts_chirho[0].binders_chirho.len(), 2);
            assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "x");
            assert_eq!(alts_chirho[0].binders_chirho[1].name_chirho, "y");
            // RHS should reference x's binder
            let x_id_chirho = alts_chirho[0].binders_chirho[0].id_chirho;
            assert!(matches!(
                alts_chirho[0].rhs_chirho,
                CoreExprChirho::VarChirho(id_chirho) if id_chirho == x_id_chirho
            ));
        } else {
            panic!("expected CaseChirho");
        }
    }

    #[test]
    fn desugar_case_infix_con_binders_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // case xs of { x : rest -> x }
        let expr_chirho = ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("xs"))),
            alts_chirho: vec![rhasky_ast_chirho::expr_chirho::AltChirho {
                pat_chirho: PatChirho::InfixConChirho {
                    left_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("x"))),
                    op_chirho: dummy_name_chirho(":"),
                    right_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("rest"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                    dummy_name_chirho("x"),
                )),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
            assert_eq!(alts_chirho.len(), 1);
            assert!(matches!(
                alts_chirho[0].con_chirho,
                AltConChirho::DataConChirho(ref s_chirho) if s_chirho == ":"
            ));
            assert_eq!(alts_chirho[0].binders_chirho.len(), 2);
            assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "x");
            assert_eq!(alts_chirho[0].binders_chirho[1].name_chirho, "rest");
        } else {
            panic!("expected CaseChirho");
        }
    }

    #[test]
    fn desugar_case_tuple_binders_chirho() {
        use rhasky_ast_chirho::expr_chirho::AltChirho as AstAltChirho;
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // case t of { (a, b) -> a }
        let expr_chirho = ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("t"))),
            alts_chirho: vec![AstAltChirho {
                pat_chirho: PatChirho::TupleChirho {
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
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
            assert_eq!(alts_chirho.len(), 1);
            assert!(matches!(
                alts_chirho[0].con_chirho,
                AltConChirho::DataConChirho(ref s_chirho) if s_chirho == "$tuple2"
            ));
            assert_eq!(alts_chirho[0].binders_chirho.len(), 2);
            assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "a");
            assert_eq!(alts_chirho[0].binders_chirho[1].name_chirho, "b");
        } else {
            panic!("expected CaseChirho");
        }
    }

    #[test]
    fn desugar_case_record_binders_chirho() {
        use rhasky_ast_chirho::pat_chirho::PatFieldChirho;
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // case p of { MkPoint { xCoord = a, yCoord = b } -> a }
        let expr_chirho = ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("p"))),
            alts_chirho: vec![rhasky_ast_chirho::expr_chirho::AltChirho {
                pat_chirho: PatChirho::RecordChirho {
                    con_chirho: dummy_name_chirho("MkPoint"),
                    fields_chirho: vec![
                        PatFieldChirho {
                            name_chirho: dummy_name_chirho("xCoord"),
                            pattern_chirho: PatChirho::VarChirho(dummy_name_chirho("a")),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        PatFieldChirho {
                            name_chirho: dummy_name_chirho("yCoord"),
                            pattern_chirho: PatChirho::VarChirho(dummy_name_chirho("b")),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
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
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
            assert_eq!(alts_chirho.len(), 1);
            assert!(matches!(
                alts_chirho[0].con_chirho,
                AltConChirho::DataConChirho(ref s_chirho) if s_chirho == "MkPoint"
            ));
            assert_eq!(alts_chirho[0].binders_chirho.len(), 2);
            assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "a");
            assert_eq!(alts_chirho[0].binders_chirho[1].name_chirho, "b");
        } else {
            panic!("expected CaseChirho");
        }
    }

    #[test]
    fn desugar_case_wildcard_binders_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // case x of { Just _ -> 1; Nothing -> 0 }
        let expr_chirho = ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            alts_chirho: vec![
                rhasky_ast_chirho::expr_chirho::AltChirho {
                    pat_chirho: PatChirho::ConChirho {
                        con_chirho: dummy_name_chirho("Just"),
                        args_chirho: vec![PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                        LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO),
                    )),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhasky_ast_chirho::expr_chirho::AltChirho {
                    pat_chirho: PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO),
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                        LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO),
                    )),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);

        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
            assert_eq!(alts_chirho.len(), 2);
            // Just _ → 1 binder (wildcard → "_wild")
            assert_eq!(alts_chirho[0].binders_chirho.len(), 1);
            assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "_wild");
            // _ → default, no binders
            assert_eq!(alts_chirho[1].binders_chirho.len(), 0);
        } else {
            panic!("expected CaseChirho");
        }
    }

    #[test]
    fn desugar_function_con_pattern_binders_chirho() {
        // Test that function definition with constructor pattern also extracts binders
        // f (Just x) = x
        // f Nothing = 0
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("f"),
                matches_chirho: vec![
                    MatchArmChirho {
                        pats_chirho: vec![PatChirho::ConChirho {
                            con_chirho: dummy_name_chirho("Just"),
                            args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("x"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    MatchArmChirho {
                        pats_chirho: vec![PatChirho::ConChirho {
                            con_chirho: dummy_name_chirho("Nothing"),
                            args_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                            LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let output_chirho = desugar_module_chirho(&module_chirho);
        let f_rhs_chirho = &output_chirho.module_chirho.bindings_chirho[0].rhs_chirho;
        // f = \arg0 -> case arg0 of { Just x -> x; Nothing -> 0 }
        if let CoreExprChirho::LamChirho { body_chirho, .. } = f_rhs_chirho {
            if let CoreExprChirho::CaseChirho { alts_chirho, .. } = body_chirho.as_ref() {
                assert_eq!(alts_chirho.len(), 2);
                // Just x → 1 binder
                assert_eq!(alts_chirho[0].binders_chirho.len(), 1);
                assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "x");
                // Nothing → 0 binders
                assert_eq!(alts_chirho[1].binders_chirho.len(), 0);
            } else {
                panic!("expected CaseChirho body");
            }
        } else {
            panic!("expected LamChirho");
        }
    }

    #[test]
    fn desugar_let_non_recursive_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // let x = 10 in x
        let expr_chirho = ExprChirho::LetChirho {
            binds_chirho: vec![
                rhasky_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    pat_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                        LitChirho::IntChirho(10, SpanChirho::DUMMY_CHIRHO),
                    )),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        if let CoreExprChirho::LetChirho { rec_chirho, .. } = &core_chirho {
            assert!(!rec_chirho, "non-recursive let should have rec=false");
        } else {
            panic!("expected LetChirho");
        }
    }

    #[test]
    fn desugar_let_recursive_detected_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // let f x = f x in f 0
        let expr_chirho = ExprChirho::LetChirho {
            binds_chirho: vec![
                rhasky_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("f"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(
                                dummy_name_chirho("f"),
                            )),
                            arg_chirho: Box::new(ExprChirho::VarChirho(
                                dummy_name_chirho("x"),
                            )),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            body_chirho: Box::new(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("f"))),
                arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                    0,
                    SpanChirho::DUMMY_CHIRHO,
                ))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        if let CoreExprChirho::LetChirho { rec_chirho, .. } = &core_chirho {
            assert!(rec_chirho, "self-recursive let should have rec=true");
        } else {
            panic!("expected LetChirho");
        }
    }

    #[test]
    fn desugar_top_level_is_rec_chirho() {
        // Top-level bindings in Haskell are always implicitly mutually recursive
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
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
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let output_chirho = desugar_module_chirho(&module_chirho);
        assert!(
            output_chirho.module_chirho.bindings_chirho[0].is_rec_chirho,
            "top-level bindings should be marked recursive"
        );
    }

    #[test]
    fn desugar_bang_pattern_case_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // case x of { !y -> y }
        let expr_chirho = ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            alts_chirho: vec![rhasky_ast_chirho::expr_chirho::AltChirho {
                pat_chirho: PatChirho::BangChirho {
                    inner_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("y"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                    dummy_name_chirho("y"),
                )),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // Should produce a case expression (bang delegates to variable → default alt)
        assert!(matches!(core_chirho, CoreExprChirho::CaseChirho { .. }));
        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
            assert_eq!(alts_chirho.len(), 1);
            assert!(matches!(
                alts_chirho[0].con_chirho,
                AltConChirho::DefaultChirho
            ));
        }
    }

    #[test]
    fn desugar_lazy_pattern_case_chirho() {
        let mut ctx_chirho = DesugarCtxChirho::new_chirho();
        // case x of { ~(Just a) -> a }
        let expr_chirho = ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            alts_chirho: vec![rhasky_ast_chirho::expr_chirho::AltChirho {
                pat_chirho: PatChirho::LazyChirho {
                    inner_chirho: Box::new(PatChirho::ConChirho {
                        con_chirho: dummy_name_chirho("Just"),
                        args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("a"))],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                    dummy_name_chirho("a"),
                )),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let core_chirho = ctx_chirho.desugar_expr_chirho(&expr_chirho);
        // Lazy pattern → default alt (irrefutable match)
        assert!(matches!(core_chirho, CoreExprChirho::CaseChirho { .. }));
        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &core_chirho {
            assert_eq!(alts_chirho.len(), 1);
            assert!(matches!(
                alts_chirho[0].con_chirho,
                AltConChirho::DefaultChirho
            ));
            // Lazy pattern should still extract binders from the inner pattern
            assert_eq!(alts_chirho[0].binders_chirho.len(), 1);
            assert_eq!(alts_chirho[0].binders_chirho[0].name_chirho, "a");
        }
    }

    #[test]
    fn desugar_bang_pattern_fun_arg_chirho() {
        // f !x = x  → should produce a Lam with the var bound
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("f"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::BangChirho {
                        inner_chirho: Box::new(PatChirho::VarChirho(
                            dummy_name_chirho("x"),
                        )),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                        dummy_name_chirho("x"),
                    )),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let output_chirho = desugar_module_chirho(&module_chirho);
        let f_rhs_chirho = &output_chirho.module_chirho.bindings_chirho[0].rhs_chirho;
        // Should be \x -> x (bang on simple var pattern is a simple lambda)
        assert!(matches!(f_rhs_chirho, CoreExprChirho::LamChirho { .. }));
    }
}
