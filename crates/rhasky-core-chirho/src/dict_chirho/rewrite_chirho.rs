// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Method rewriting: dict param insertion, method reference rewriting,
//! module transformation, and the final pass finish.

#![allow(unused_imports)]
use std::collections::HashMap;

use rhasky_span_chirho::SpanChirho;
use rhasky_typing_chirho::class_chirho::ClassEnvChirho;
use rhasky_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
};
use super::{DictPassCtxChirho, DictLayoutChirho};

use std::cell::RefCell;
use std::collections::HashSet;
use rhasky_typing_chirho::env_chirho::TyEnvChirho;
use rhasky_typing_chirho::ty_chirho::{SchemeChirho, SchemePredChirho};
use super::DictPassResultChirho;

impl DictPassCtxChirho {
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
    pub(super) fn rewrite_method_refs_chirho(
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
                    "IsString" => Some("[Char]"),
                    "IsList" => Some("[t9037]"),
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
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
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

        // Seed local_shadow_ids with top-level user bindings whose name
        // collides with a class method (e.g. user defines `toList` which
        // shadows the IsList class method).  Without this, the dict pass
        // would incorrectly rewrite user calls to `toList` as if they were
        // the IsList method.
        for binding_chirho in &module_chirho.bindings_chirho {
            let name_chirho = &binding_chirho.binder_chirho.name_chirho;
            if self.method_selectors_chirho.contains_key(name_chirho) {
                // Only shadow if the binding is NOT itself a class method
                // (i.e. it has no matching class predicate in its type scheme)
                let is_class_method_chirho = type_env_chirho
                    .lookup_chirho(name_chirho)
                    .map(|s_chirho| {
                        s_chirho.preds_chirho.iter().any(|p_chirho| {
                            let (class_chirho, _) =
                                &self.method_selectors_chirho[name_chirho];
                            &p_chirho.class_name_chirho == class_chirho
                        })
                    })
                    .unwrap_or(false);
                if !is_class_method_chirho {
                    self.local_shadow_ids_chirho
                        .borrow_mut()
                        .insert(binding_chirho.binder_chirho.id_chirho);
                }
            }
        }

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
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
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
