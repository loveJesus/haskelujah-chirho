// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Method rewriting: dict param insertion, method reference rewriting,
//! module transformation, and the final pass finish.

use std::collections::{HashMap, HashSet};

use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::env_chirho::TyEnvChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::{DictPassCtxChirho, DictPassResultChirho};
use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho, CoreModuleChirho,
    InlineAnnotationChirho,
};

mod call_keys_chirho;
mod constrained_references_chirho;
mod dict_params_chirho;
mod head_dispatch_chirho;
mod method_args_chirho;
mod method_refs_chirho;
mod numeric_defaults_chirho;
mod occurrence_rows_chirho;
mod return_chirho;
mod type_keys_chirho;

impl DictPassCtxChirho {
    fn parse_prim_binding_info_chirho(
        &self,
        binding_name_chirho: &str,
    ) -> Option<(String, String, String)> {
        for (class_name_chirho, layout_chirho) in &self.layouts_chirho {
            for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                let prefix_chirho = format!("$prim_{}_{}_", class_name_chirho, method_name_chirho);
                if let Some(type_key_chirho) = binding_name_chirho.strip_prefix(&prefix_chirho) {
                    return Some((
                        class_name_chirho.clone(),
                        method_name_chirho.clone(),
                        type_key_chirho.to_string(),
                    ));
                }
            }
        }
        None
    }

    /// The context classes a module `$prim` row takes dictionaries for, by the
    /// one rule that also marks it context-leading (`occurrence_rows_chirho`).
    /// workflow: language-features-chirho/own-class-method-occurrences-chirho
    fn conditional_context_classes_for_prim_binding_chirho(
        &self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        parsed_type_key_chirho: &str,
    ) -> Option<Vec<String>> {
        Self::conditional_context_preds_chirho(
            class_env_chirho,
            class_name_chirho,
            parsed_type_key_chirho,
        )
        .map(|context_chirho| {
            context_chirho
                .into_iter()
                .map(|pred_chirho| pred_chirho.class_name_chirho)
                .collect()
        })
    }

    fn expr_contains_numeric_default_marker_chirho(&self, expr_chirho: &CoreExprChirho) -> bool {
        match expr_chirho {
            CoreExprChirho::LitChirho(crate::expr_chirho::CoreLitChirho::IntChirho(_)) => true,
            CoreExprChirho::VarChirho(id_chirho) => {
                self.names_chirho.get(id_chirho).is_some_and(|name_chirho| {
                    let short_name_chirho = name_chirho.rsplit('.').next().unwrap_or(name_chirho);
                    let marker_chirho = short_name_chirho
                        .strip_prefix('(')
                        .and_then(|name_chirho| name_chirho.strip_suffix(')'))
                        .unwrap_or(short_name_chirho);
                    matches!(
                        marker_chirho,
                        "+" | "-" | "*" | "negate" | "abs" | "signum" | "fromInteger" | "toInteger"
                    )
                })
            }
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => {
                matches!(
                    name_chirho.as_str(),
                    "+#" | "-#" | "*#" | "negate#" | "fromInteger#" | "toInteger#"
                ) || args_chirho
                    .iter()
                    .any(|arg_chirho| self.expr_contains_numeric_default_marker_chirho(arg_chirho))
            }
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                self.expr_contains_numeric_default_marker_chirho(fun_chirho)
                    || self.expr_contains_numeric_default_marker_chirho(arg_chirho)
            }
            CoreExprChirho::LamChirho { body_chirho, .. }
            | CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                self.expr_contains_numeric_default_marker_chirho(body_chirho)
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.expr_contains_numeric_default_marker_chirho(inner_chirho),
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                binds_chirho.iter().any(|(_, rhs_chirho)| {
                    self.expr_contains_numeric_default_marker_chirho(rhs_chirho)
                }) || self.expr_contains_numeric_default_marker_chirho(body_chirho)
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                self.expr_contains_numeric_default_marker_chirho(scrutinee_chirho)
                    || alts_chirho.iter().any(|alt_chirho| {
                        self.expr_contains_numeric_default_marker_chirho(&alt_chirho.rhs_chirho)
                    })
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => args_chirho
                .iter()
                .any(|arg_chirho| self.expr_contains_numeric_default_marker_chirho(arg_chirho)),
            _ => false,
        }
    }

    fn collect_method_app_chirho<'a>(
        &self,
        expr_chirho: &'a CoreExprChirho,
    ) -> Option<(CoreIdChirho, Vec<&'a CoreExprChirho>, Option<String>)> {
        let mut args_chirho = Vec::new();
        let mut current_chirho = expr_chirho;
        let mut type_key_hint_chirho = None;

        // Peel off value applications and interleaved type applications.
        // Interface-imported overloaded methods can arrive as
        // `v @Sum arg1 @Sum arg2`; the type app is the reliable instance key
        // when interface newtype erasure makes the value args look like Ints.
        loop {
            match current_chirho {
                CoreExprChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                } => {
                    args_chirho.push(arg_chirho.as_ref());
                    current_chirho = fun_chirho.as_ref();
                }
                CoreExprChirho::TyAppChirho {
                    expr_chirho: inner_chirho,
                    ty_chirho,
                } => {
                    type_key_hint_chirho = Some(format!("{}", ty_chirho));
                    current_chirho = inner_chirho.as_ref();
                }
                _ => break,
            }
        }

        // Check if the innermost function is a class method Var
        if let CoreExprChirho::VarChirho(id_chirho) = current_chirho {
            if self
                .dict_param_bindings_chirho
                .contains_key(&self.canonical_id_chirho(*id_chirho))
            {
                return None;
            }
            if let Some(name_chirho) = self.names_chirho.get(id_chirho) {
                if self
                    .class_method_selector_for_name_chirho(name_chirho)
                    .is_some()
                {
                    args_chirho.reverse(); // Now [arg1, arg2, ...]
                    return Some((*id_chirho, args_chirho, type_key_hint_chirho));
                }
            }
        }

        None
    }

    fn class_method_selector_for_name_chirho(
        &self,
        name_chirho: &str,
    ) -> Option<(String, CoreIdChirho)> {
        if let Some((class_name_chirho, sel_id_chirho)) =
            self.method_selectors_chirho.get(name_chirho)
        {
            return Some((class_name_chirho.clone(), *sel_id_chirho));
        }

        let short_name_chirho = name_chirho.rsplit('.').next()?;
        if let Some(stripped_chirho) = short_name_chirho
            .strip_prefix('(')
            .and_then(|n_chirho| n_chirho.strip_suffix(')'))
        {
            if let Some((class_name_chirho, sel_id_chirho)) =
                self.method_selectors_chirho.get(stripped_chirho)
            {
                return Some((class_name_chirho.clone(), *sel_id_chirho));
            }
        }
        if short_name_chirho == name_chirho {
            return None;
        }
        self.method_selectors_chirho
            .get(short_name_chirho)
            .map(|(class_name_chirho, sel_id_chirho)| (class_name_chirho.clone(), *sel_id_chirho))
    }

    /// Collect a dict-parameterized function application chain: if `expr_chirho`
    /// is `App^n(Var(f), arg1, ..., argN)` where `f` has dict parameters,
    /// return `(fn_id, classes, [arg1, ..., argN])`.
    /// The canonical id behind a reference occurrence id (itself otherwise).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    fn canonical_id_chirho(&self, id_chirho: CoreIdChirho) -> CoreIdChirho {
        self.method_occurrence_canon_chirho
            .get(&id_chirho)
            .map(|(_name_chirho, canonical_chirho)| *canonical_chirho)
            .unwrap_or(id_chirho)
    }

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

        while let CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ..
        } = current_chirho
        {
            current_chirho = inner_chirho.as_ref();
        }

        if let CoreExprChirho::VarChirho(id_chirho) = current_chirho {
            if let Some(classes_chirho) = self
                .dict_param_bindings_chirho
                .get(&self.canonical_id_chirho(*id_chirho))
            {
                args_chirho.reverse();
                return Some((*id_chirho, classes_chirho, args_chirho));
            }
        }

        None
    }

    fn is_class_method_name_chirho(&self, name_chirho: &str) -> bool {
        let short_name_chirho = name_chirho.rsplit('.').next().unwrap_or(name_chirho);
        let stripped_name_chirho = short_name_chirho
            .strip_prefix('(')
            .and_then(|n_chirho| n_chirho.strip_suffix(')'))
            .unwrap_or(short_name_chirho);

        self.method_selectors_chirho.contains_key(name_chirho)
            || self.method_selectors_chirho.contains_key(short_name_chirho)
            || self
                .method_selectors_chirho
                .contains_key(stripped_name_chirho)
            || self
                .class_method_selector_for_name_chirho(name_chirho)
                .is_some()
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
    /// Evidence-threading P1: immutable name → id scan over the ctx name map
    /// (used to resolve `$prim_{class}_{method}_{key}` targets for occurrence
    /// evidence; returns None when no such generated row exists).
    fn find_global_id_by_name_chirho(&self, name_chirho: &str) -> Option<CoreIdChirho> {
        self.names_chirho
            .iter()
            .find(|(_, existing_chirho)| existing_chirho.as_str() == name_chirho)
            .map(|(id_chirho, _)| *id_chirho)
    }

    pub(super) fn rewrite_method_refs_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
    ) -> CoreExprChirho {
        self.rewrite_method_refs_with_locals_chirho(
            expr_chirho,
            dict_vars_chirho,
            &HashSet::new(),
            &HashMap::new(),
            &HashMap::new(),
        )
    }

    /// Run the dictionary-passing transform on a Core module.
    pub fn transform_module_chirho(
        &mut self,
        module_chirho: &CoreModuleChirho,
        type_env_chirho: &TyEnvChirho,
        class_env_chirho: &ClassEnvChirho,
    ) -> CoreModuleChirho {
        self.seed_body_backed_bindings_chirho(module_chirho);

        // Build layouts from the class environment
        self.build_layouts_chirho(class_env_chirho);

        // Context-leading rows, before any binding is rewritten.
        // workflow: language-features-chirho/own-class-method-occurrences-chirho
        self.record_context_leading_rows_chirho(module_chirho, class_env_chirho);

        // Generate method selectors
        self.generate_selectors_chirho();

        // Generate built-in $prim_ bindings for standard class methods
        self.generate_builtin_prim_bindings_chirho();

        // Proven occurrence evidence can demand a concrete structured row that
        // is not part of the finite bootstrap table (for example
        // `Show (Either Int Bool)`). Generate only portable, body-backed rows.
        self.generate_evidenced_show_bindings_chirho();

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
                            let (class_chirho, _) = &self.method_selectors_chirho[name_chirho];
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
            if let Some(scheme_chirho) = type_env_chirho.lookup_chirho(name_chirho) {
                let classes_chirho = self.dict_param_classes_for_scheme_chirho(scheme_chirho);
                if !classes_chirho.is_empty() {
                    self.dict_param_bindings_chirho
                        .insert(binding_chirho.binder_chirho.id_chirho, classes_chirho);
                }
            }
        }
        for (id_chirho, name_chirho) in self.names_chirho.clone() {
            let short_name_chirho = name_chirho
                .rsplit('.')
                .next()
                .unwrap_or(name_chirho.as_str());
            let stripped_name_chirho = short_name_chirho
                .strip_prefix('(')
                .and_then(|n_chirho| n_chirho.strip_suffix(')'))
                .unwrap_or(short_name_chirho);
            let is_extra_imported_chirho =
                self.extra_dict_param_names_chirho.contains(&name_chirho)
                    || self
                        .extra_dict_param_names_chirho
                        .contains(short_name_chirho)
                    || self
                        .extra_dict_param_names_chirho
                        .contains(stripped_name_chirho);
            if !is_extra_imported_chirho {
                continue;
            }
            if self.is_class_method_name_chirho(&name_chirho) {
                continue;
            }
            if let Some(scheme_chirho) = type_env_chirho.lookup_chirho(&name_chirho) {
                let classes_chirho = self.dict_param_classes_for_scheme_chirho(scheme_chirho);
                if !classes_chirho.is_empty() {
                    self.dict_param_bindings_chirho
                        .entry(id_chirho)
                        .or_insert(classes_chirho);
                }
            }
        }

        // Transform each binding
        let mut bindings_chirho = Vec::new();
        let module_call_arg_type_keys_chirho =
            self.collect_module_call_arg_type_keys_chirho(module_chirho);

        for binding_chirho in &module_chirho.bindings_chirho {
            let name_chirho = &binding_chirho.binder_chirho.name_chirho;

            // Look up the type scheme for this binding
            if let Some(scheme_chirho) = type_env_chirho.lookup_chirho(name_chirho) {
                let transformed_chirho = self.add_dict_params_with_local_type_keys_chirho(
                    binding_chirho,
                    scheme_chirho,
                    &module_call_arg_type_keys_chirho,
                );
                bindings_chirho.push(transformed_chirho);
            } else {
                // Binding not in type env (e.g. $prim_ instance method
                // bodies).  Still rewrite class-method references in the
                // body so that Var(+) etc. are resolved to selectors.
                let mut dict_vars_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
                let mut evidence_classes_chirho: HashSet<String> = HashSet::new();
                for ((class_name_chirho, type_key_chirho), dict_id_chirho) in
                    &self.instance_dicts_chirho
                {
                    let is_int_chirho = type_key_chirho == "Int";
                    if is_int_chirho {
                        dict_vars_chirho.insert(class_name_chirho.clone(), *dict_id_chirho);
                    } else {
                        dict_vars_chirho
                            .entry(class_name_chirho.clone())
                            .or_insert(*dict_id_chirho);
                    }
                }
                if let Some((class_name_chirho, _method_name_chirho, parsed_type_key_chirho)) =
                    self.parse_prim_binding_info_chirho(name_chirho)
                {
                    if let Some(context_classes_chirho) = self
                        .conditional_context_classes_for_prim_binding_chirho(
                            class_env_chirho,
                            &class_name_chirho,
                            &parsed_type_key_chirho,
                        )
                    {
                        let mut dict_binders_chirho = Vec::new();
                        for context_class_chirho in context_classes_chirho {
                            let dict_binder_chirho = self.fresh_binder_chirho(
                                &format!("$d{}", context_class_chirho),
                                TyChirho::ConChirho(format!("$Dict_{}", context_class_chirho)),
                            );
                            evidence_classes_chirho.insert(context_class_chirho.clone());
                            dict_vars_chirho
                                .insert(context_class_chirho, dict_binder_chirho.id_chirho);
                            dict_binders_chirho.push(dict_binder_chirho);
                        }

                        let mut local_type_keys_chirho: HashMap<CoreIdChirho, String> =
                            HashMap::new();
                        let mut current_expr_chirho = &binding_chirho.rhs_chirho;
                        while let CoreExprChirho::TyLamChirho { body_chirho, .. } =
                            current_expr_chirho
                        {
                            current_expr_chirho = body_chirho;
                        }
                        if let CoreExprChirho::LamChirho { binder_chirho, .. } = current_expr_chirho
                        {
                            local_type_keys_chirho
                                .insert(binder_chirho.id_chirho, parsed_type_key_chirho.clone());
                        }

                        let mut local_instance_dicts_chirho: HashMap<
                            (String, String),
                            CoreIdChirho,
                        > = HashMap::new();
                        let mut self_dict_let_binds_chirho = Vec::new();
                        let mut binding_is_rec_chirho = binding_chirho.is_rec_chirho;

                        if !self.instance_dicts_chirho.contains_key(&(
                            class_name_chirho.clone(),
                            parsed_type_key_chirho.clone(),
                        )) {
                            if let Some((has_no_supers_chirho, method_slot_names_chirho)) = self
                                .layouts_chirho
                                .get(&class_name_chirho)
                                .map(|layout_chirho| {
                                    (
                                        layout_chirho.super_slots_chirho.is_empty(),
                                        layout_chirho
                                            .method_slots_chirho
                                            .iter()
                                            .map(|(slot_method_name_chirho, _)| {
                                                slot_method_name_chirho.clone()
                                            })
                                            .collect::<Vec<_>>(),
                                    )
                                })
                            {
                                if has_no_supers_chirho {
                                    let self_dict_binder_chirho = self.fresh_binder_chirho(
                                        &format!("$d{}Self", class_name_chirho),
                                        TyChirho::ConChirho(format!("$Dict_{}", class_name_chirho)),
                                    );
                                    let mut field_args_chirho = Vec::new();
                                    for slot_method_name_chirho in &method_slot_names_chirho {
                                        let prim_name_chirho = format!(
                                            "$prim_{}_{}_{}",
                                            class_name_chirho,
                                            slot_method_name_chirho,
                                            parsed_type_key_chirho
                                        );
                                        let prim_id_chirho =
                                            self.resolve_or_fresh_id_chirho(&prim_name_chirho);
                                        let mut method_expr_chirho =
                                            CoreExprChirho::VarChirho(prim_id_chirho);
                                        for dict_binder_chirho in &dict_binders_chirho {
                                            method_expr_chirho = CoreExprChirho::AppChirho {
                                                fun_chirho: Box::new(method_expr_chirho),
                                                arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                                    dict_binder_chirho.id_chirho,
                                                )),
                                            };
                                        }
                                        field_args_chirho.push(method_expr_chirho);
                                    }
                                    self_dict_let_binds_chirho.push((
                                        self_dict_binder_chirho.clone(),
                                        CoreExprChirho::ConAppChirho {
                                            con_name_chirho: format!("$Dict_{}", class_name_chirho),
                                            args_chirho: field_args_chirho,
                                        },
                                    ));
                                    local_instance_dicts_chirho.insert(
                                        (class_name_chirho.clone(), parsed_type_key_chirho.clone()),
                                        self_dict_binder_chirho.id_chirho,
                                    );
                                    binding_is_rec_chirho = true;
                                }
                            }
                        }

                        let mut rewritten_rhs_chirho = self.rewrite_method_refs_with_locals_chirho(
                            &binding_chirho.rhs_chirho,
                            &dict_vars_chirho,
                            &evidence_classes_chirho,
                            &local_type_keys_chirho,
                            &local_instance_dicts_chirho,
                        );
                        for (binder_chirho, self_dict_expr_chirho) in
                            self_dict_let_binds_chirho.iter().rev()
                        {
                            rewritten_rhs_chirho = CoreExprChirho::LetChirho {
                                rec_chirho: false,
                                binds_chirho: vec![(
                                    binder_chirho.clone(),
                                    self_dict_expr_chirho.clone(),
                                )],
                                body_chirho: Box::new(rewritten_rhs_chirho),
                            };
                        }
                        let mut result_ty_chirho = binding_chirho.binder_chirho.ty_chirho.clone();
                        for dict_binder_chirho in dict_binders_chirho.iter().rev() {
                            rewritten_rhs_chirho = CoreExprChirho::LamChirho {
                                binder_chirho: dict_binder_chirho.clone(),
                                body_chirho: Box::new(rewritten_rhs_chirho),
                            };
                            result_ty_chirho = TyChirho::fun_chirho(
                                dict_binder_chirho.ty_chirho.clone(),
                                result_ty_chirho,
                            );
                        }

                        bindings_chirho.push(CoreBindingChirho {
                            binder_chirho: BinderChirho {
                                ty_chirho: result_ty_chirho,
                                ..binding_chirho.binder_chirho.clone()
                            },
                            rhs_chirho: rewritten_rhs_chirho,
                            is_rec_chirho: binding_is_rec_chirho,
                            inline_chirho: InlineAnnotationChirho::NoneChirho,
                        });
                        continue;
                    }
                }

                let rewritten_rhs_chirho = if let Some((
                    _class_name_chirho,
                    _method_name_chirho,
                    parsed_type_key_chirho,
                )) = self.parse_prim_binding_info_chirho(name_chirho)
                {
                    let mut local_type_keys_chirho: HashMap<CoreIdChirho, String> = HashMap::new();
                    let mut current_expr_chirho = &binding_chirho.rhs_chirho;
                    while let CoreExprChirho::TyLamChirho { body_chirho, .. } = current_expr_chirho
                    {
                        current_expr_chirho = body_chirho;
                    }
                    if let CoreExprChirho::LamChirho { binder_chirho, .. } = current_expr_chirho {
                        local_type_keys_chirho
                            .insert(binder_chirho.id_chirho, parsed_type_key_chirho);
                    }
                    self.rewrite_method_refs_with_locals_chirho(
                        &binding_chirho.rhs_chirho,
                        &dict_vars_chirho,
                        &evidence_classes_chirho,
                        &local_type_keys_chirho,
                        &HashMap::new(),
                    )
                } else {
                    self.rewrite_method_refs_chirho(&binding_chirho.rhs_chirho, &dict_vars_chirho)
                };
                bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: binding_chirho.binder_chirho.clone(),
                    rhs_chirho: rewritten_rhs_chirho,
                    is_rec_chirho: binding_chirho.is_rec_chirho,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                });
            }
        }

        // A renderable monomorphic derived-Show type's public row becomes its
        // precedence helper at 0, before the generated helpers are prepended.
        // workflow: print-show-evidence-chirho
        self.rewrite_derived_show_rows_chirho(&mut bindings_chirho);

        // Prepend generated dictionary bindings
        let mut all_bindings_chirho = self.generated_bindings_chirho.clone();
        all_bindings_chirho.extend(bindings_chirho);

        CoreModuleChirho {
            name_chirho: module_chirho.name_chirho.clone(),
            bindings_chirho: all_bindings_chirho,
            names_chirho: self.names_chirho.clone(),
            specialize_pragmas_chirho: module_chirho.specialize_pragmas_chirho.clone(),
            foreign_exports_chirho: module_chirho.foreign_exports_chirho.clone(),
        }
    }

    /// Finish the transform and return the result.
    /// Evidence-threading P2b: total safety-net sweep enforcing the guaranteed-
    /// elimination invariant — any occurrence id that survived rewriting (some
    /// specialized paths clone subtrees without the occurrence intercepts) is
    /// restored to its canonical shared id so no occ id ever reaches STG.
    fn canonicalize_surviving_occurrences_chirho(&self, expr_chirho: &mut CoreExprChirho) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                if let Some((_name_chirho, canonical_id_chirho)) =
                    self.method_occurrence_canon_chirho.get(id_chirho)
                {
                    *id_chirho = *canonical_id_chirho;
                }
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                self.canonicalize_surviving_occurrences_chirho(fun_chirho);
                self.canonicalize_surviving_occurrences_chirho(arg_chirho);
            }
            CoreExprChirho::LamChirho { body_chirho, .. }
            | CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                self.canonicalize_surviving_occurrences_chirho(body_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                self.canonicalize_surviving_occurrences_chirho(inner_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (_binder_chirho, rhs_chirho) in binds_chirho.iter_mut() {
                    self.canonicalize_surviving_occurrences_chirho(rhs_chirho);
                }
                self.canonicalize_surviving_occurrences_chirho(body_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                self.canonicalize_surviving_occurrences_chirho(scrutinee_chirho);
                for alt_chirho in alts_chirho.iter_mut() {
                    self.canonicalize_surviving_occurrences_chirho(&mut alt_chirho.rhs_chirho);
                }
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho.iter_mut() {
                    self.canonicalize_surviving_occurrences_chirho(arg_chirho);
                }
            }
        }
    }

    pub fn finish_chirho(self, mut module_chirho: CoreModuleChirho) -> DictPassResultChirho {
        if !self.method_occurrence_canon_chirho.is_empty() {
            for binding_chirho in module_chirho.bindings_chirho.iter_mut() {
                self.canonicalize_surviving_occurrences_chirho(&mut binding_chirho.rhs_chirho);
            }
        }
        DictPassResultChirho {
            module_chirho,
            names_chirho: self.names_chirho,
            layouts_chirho: self.layouts_chirho,
        }
    }
}
