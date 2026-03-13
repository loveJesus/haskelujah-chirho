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

use std::collections::HashMap;

use rhasky_span_chirho::SpanChirho;
use rhasky_typing_chirho::class_chirho::ClassEnvChirho;
use rhasky_typing_chirho::env_chirho::TyEnvChirho;
use rhasky_typing_chirho::ty_chirho::{SchemeChirho, TyChirho};

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
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
}

impl DictPassCtxChirho {
    pub fn new_chirho(
        start_id_chirho: u32,
        names_chirho: HashMap<CoreIdChirho, String>,
    ) -> Self {
        Self {
            next_id_chirho: start_id_chirho,
            names_chirho,
            layouts_chirho: HashMap::new(),
            generated_bindings_chirho: Vec::new(),
            method_selectors_chirho: HashMap::new(),
            instance_dicts_chirho: HashMap::new(),
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

                let type_key_chirho = format!("{}", inst_chirho.head_ty_chirho);
                let dict_name_chirho =
                    format!("$f{}{}", class_name_chirho, type_key_chirho);
                let dict_ty_chirho =
                    TyChirho::ConChirho(format!("$Dict_{}", class_name_chirho));

                // Build: $DictClass super1 super2 ... method1 method2 ...
                let con_id_chirho =
                    self.fresh_id_chirho(&format!("$Dict_{}", class_name_chirho));
                let mut dict_expr_chirho = CoreExprChirho::VarChirho(con_id_chirho);

                // Superclass dictionary arguments
                for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
                    let super_dict_name_chirho =
                        format!("$f{}{}", super_name_chirho, type_key_chirho);
                    let super_dict_id_chirho =
                        self.fresh_id_chirho(&super_dict_name_chirho);
                    dict_expr_chirho = CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(dict_expr_chirho),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(
                            super_dict_id_chirho,
                        )),
                    };
                }

                // Method implementation arguments (primitives)
                for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                    let prim_name_chirho = format!(
                        "$prim_{}_{}_{}", class_name_chirho,
                        method_name_chirho, type_key_chirho
                    );
                    let prim_id_chirho =
                        self.fresh_id_chirho(&prim_name_chirho);
                    dict_expr_chirho = CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(dict_expr_chirho),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(
                            prim_id_chirho,
                        )),
                    };
                }

                let dict_binder_chirho =
                    self.fresh_binder_chirho(&dict_name_chirho, dict_ty_chirho);
                let dict_id_chirho = dict_binder_chirho.id_chirho;

                self.generated_bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: dict_binder_chirho,
                    rhs_chirho: dict_expr_chirho,
                    is_rec_chirho: false,
                });

                self.instance_dicts_chirho.insert(
                    (class_name_chirho.clone(), type_key_chirho),
                    dict_id_chirho,
                );
            }
        }
    }

    /// Rewrite method references in an expression body.
    ///
    /// Given a mapping of in-scope dictionary variables
    /// `(class_name -> dict_id)`, replaces `VarChirho` references to
    /// overloaded methods with `($sel_Class_method $dClass)`.
    fn rewrite_method_refs_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
    ) -> CoreExprChirho {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                if let Some(name_chirho) = self.names_chirho.get(id_chirho) {
                    if let Some((class_name_chirho, sel_id_chirho)) =
                        self.method_selectors_chirho.get(name_chirho)
                    {
                        if let Some(dict_id_chirho) =
                            dict_vars_chirho.get(class_name_chirho)
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
                expr_chirho.clone()
            }
            CoreExprChirho::LitChirho(_) => expr_chirho.clone(),
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => CoreExprChirho::AppChirho {
                fun_chirho: Box::new(
                    self.rewrite_method_refs_chirho(fun_chirho, dict_vars_chirho),
                ),
                arg_chirho: Box::new(
                    self.rewrite_method_refs_chirho(arg_chirho, dict_vars_chirho),
                ),
            },
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => CoreExprChirho::LamChirho {
                binder_chirho: binder_chirho.clone(),
                body_chirho: Box::new(
                    self.rewrite_method_refs_chirho(body_chirho, dict_vars_chirho),
                ),
            },
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => CoreExprChirho::LetChirho {
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
        if scheme_chirho.preds_chirho.is_empty() {
            return binding_chirho.clone();
        }

        // Create dictionary binders and build the class→dict_id mapping
        let mut dict_vars_chirho = HashMap::new();
        let mut dict_binders_chirho = Vec::new();

        for pred_chirho in &scheme_chirho.preds_chirho {
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

        // Rewrite method references in the original body
        let mut rhs_chirho =
            self.rewrite_method_refs_chirho(&binding_chirho.rhs_chirho, &dict_vars_chirho);

        // Wrap in dictionary lambdas (reverse order so first pred is outermost)
        for dict_binder_chirho in dict_binders_chirho.into_iter().rev() {
            rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: dict_binder_chirho,
                body_chirho: Box::new(rhs_chirho),
            };
        }

        // Update the binder's type to include dictionary parameters
        let mut result_ty_chirho = binding_chirho.binder_chirho.ty_chirho.clone();
        for pred_chirho in scheme_chirho.preds_chirho.iter().rev() {
            let dict_ty_chirho =
                TyChirho::ConChirho(format!("$Dict_{}", pred_chirho.class_name_chirho));
            result_ty_chirho = TyChirho::fun_chirho(dict_ty_chirho, result_ty_chirho);
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

        // Generate instance dictionary bindings
        self.generate_instance_dicts_chirho(class_env_chirho);

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
                bindings_chirho.push(binding_chirho.clone());
            }
        }

        // Prepend generated dictionary bindings
        let mut all_bindings_chirho = self.generated_bindings_chirho.clone();
        all_bindings_chirho.extend(bindings_chirho);

        CoreModuleChirho {
            name_chirho: module_chirho.name_chirho.clone(),
            bindings_chirho: all_bindings_chirho,
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
    let max_id_chirho = find_max_id_chirho(module_chirho);
    let mut ctx_chirho =
        DictPassCtxChirho::new_chirho(max_id_chirho + 1, names_chirho);
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
            DictPassCtxChirho::new_chirho(10, HashMap::new());
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
            DictPassCtxChirho::new_chirho(10, HashMap::new());
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
            DictPassCtxChirho::new_chirho(10, HashMap::new());
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
            DictPassCtxChirho::new_chirho(0, HashMap::new());
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

        // Num: 2 supers (Eq, Show), 3 methods (+, *, fromInteger)
        let num_layout_chirho = ctx_chirho.layouts_chirho.get("Num").unwrap();
        assert_eq!(num_layout_chirho.super_slots_chirho.len(), 2);
        assert_eq!(num_layout_chirho.method_slots_chirho.len(), 3);
        assert_eq!(num_layout_chirho.field_count_chirho, 5);
    }

    #[test]
    fn generate_selectors_creates_bindings_chirho() {
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();

        let mut ctx_chirho =
            DictPassCtxChirho::new_chirho(0, HashMap::new());
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
        };
        assert_eq!(find_max_id_chirho(&module_chirho), 42);
    }

    #[test]
    fn instance_dicts_generated_for_ground_instances_chirho() {
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();

        let mut ctx_chirho =
            DictPassCtxChirho::new_chirho(0, HashMap::new());
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

        // Should be a constructor application
        assert!(matches!(
            eq_int_binding_chirho.rhs_chirho,
            CoreExprChirho::AppChirho { .. }
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
            DictPassCtxChirho::new_chirho(100, names_chirho);
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
            DictPassCtxChirho::new_chirho(100, names_chirho);

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
            DictPassCtxChirho::new_chirho(100, names_chirho);
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
