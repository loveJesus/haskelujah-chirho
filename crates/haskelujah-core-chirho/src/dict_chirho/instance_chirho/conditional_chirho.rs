// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Conditional list dictionaries and their stable, possibly forward-referenced identities.
//! Each conditional list instance is routed by its class proof and its owner, never by its
//! spelling alone (row 692, C10, #33179).

use std::collections::HashSet;

use super::{DictLayoutChirho, DictPassCtxChirho};
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreLitChirho,
    InlineAnnotationChirho,
};
use haskelujah_ast_chirho::name_chirho::{
    CompilerClassChirho, InstanceIdentityChirho, PredicateClassIdentityChirho,
};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, InstDeclChirho};
use haskelujah_typing_chirho::ty_chirho::TyChirho;

/// One conditional list instance's dictionaries: the namespace they are named and filed under,
/// its layout, its context predicates' namespaces and the builder that fills them.
struct ConditionalListEntryChirho {
    class_name_chirho: String,
    layout_chirho: DictLayoutChirho,
    context_classes_chirho: Vec<String>,
    route_chirho: ConditionalListRouteChirho,
}

/// The builder of a conditional list instance's dictionaries.
#[derive(Clone, Copy)]
enum ConditionalListRouteChirho {
    /// By the class's spelling (`generate_list_instance_dict_chirho`), as before.
    SpelledChirho,
    /// A source or unproven instance under the spelling `Eq`, `Show` or `Ord`: the generic
    /// builder, which reads that instance's own rows and never a compiler-generated body.
    InstanceRowsChirho,
    /// The compiler's own `Show a => Show [a]` while a source class takes the spelling `Show`:
    /// the Show list builder in that compiler class's namespace, its methods from its own rows.
    CompilerShowChirho(CompilerClassChirho),
}

/// Whether an instance's head is a list of one type variable, `[v]`, with no hidden kind or
/// extra argument, and each context predicate constrains exactly that variable: an element type
/// is then that instance's whole instantiation (#33179).
fn complete_list_head_chirho(inst_chirho: &InstDeclChirho) -> bool {
    let TyChirho::ListChirho(element_chirho) = &inst_chirho.head_ty_chirho else {
        return false;
    };
    let TyChirho::VarChirho(variable_chirho) = element_chirho.as_ref() else {
        return false;
    };
    inst_chirho.kind_tys_chirho.is_empty()
        && inst_chirho.extra_head_tys_chirho.is_empty()
        && inst_chirho.context_chirho.iter().all(|predicate_chirho| {
            predicate_chirho.kind_tys_chirho.is_empty()
                && predicate_chirho.extra_tys_chirho.is_empty()
                && matches!(
                    &predicate_chirho.ty_chirho,
                    TyChirho::VarChirho(context_variable_chirho)
                        if context_variable_chirho == variable_chirho
                )
        })
}

impl DictPassCtxChirho {
    /// A superclass reference may reserve this name before its body is built.
    /// Reuse that identity; a fresh ID would leave the earlier reference unbound.
    pub(super) fn conditional_dict_binder_chirho(
        &mut self,
        name_chirho: &str,
        ty_chirho: TyChirho,
    ) -> BinderChirho {
        BinderChirho {
            id_chirho: self.resolve_or_fresh_id_chirho(name_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    /// Generate ground specializations of conditional instances.
    ///
    /// For each conditional instance like `Eq a => Eq [a]`, and each
    /// element type `T` that already has a ground `Eq T` instance,
    /// generate a ground `Eq [T]` dict with the appropriate method
    /// implementations. This avoids needing full parametric conditional
    /// dict support while covering common concrete list types.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub fn generate_conditional_ground_dicts_chirho(&mut self, class_env_chirho: &ClassEnvChirho) {
        // Collect conditional list instances, each routed by its class proof and its owner.
        let mut entries_chirho: Vec<ConditionalListEntryChirho> = Vec::new();

        for (class_name_chirho, instances_chirho) in
            class_env_chirho.instances_chirho().iter_chirho()
        {
            for inst_chirho in instances_chirho {
                if inst_chirho.context_chirho.is_empty() {
                    continue;
                }
                // Check if this is a list instance: head_ty is List(Var(_))
                if let TyChirho::ListChirho(_) = &inst_chirho.head_ty_chirho
                    && let Some(entry_chirho) =
                        self.conditional_list_entry_chirho(class_name_chirho, inst_chirho)
                {
                    entries_chirho.push(entry_chirho);
                }
            }
        }

        // For each conditional list instance, generate ground dicts
        // for known element types that have the required dicts.
        let known_element_types_chirho = vec!["Int", "Char", "Bool", "Double"];

        for entry_chirho in &entries_chirho {
            let class_name_chirho = &entry_chirho.class_name_chirho;
            let layout_chirho = &entry_chirho.layout_chirho;
            let context_classes_chirho = &entry_chirho.context_classes_chirho;
            // A displaced compiler Show's method fields take only that class's own rows.
            let namesake_rows_chirho = match entry_chirho.route_chirho {
                ConditionalListRouteChirho::CompilerShowChirho(identity_chirho) => {
                    self.namesake_rows_chirho(class_env_chirho, identity_chirho)
                }
                _ => HashSet::new(),
            };

            // `Eq` and `Ord` also cover string elements. `Eq [String]` compares
            // each string with the generator's `==#`, measured exact on two
            // strings; `Ord [String]` compares them with the `compareStr#` row.
            // Without them a proven `[String]` key named no dictionary, and the
            // old `==#` section, applied to whole `[String]` values, compared
            // only their first strings.
            // workflow: language-features-chirho/dictionary-evidence-chirho
            let string_element_types_chirho: &[&str] = match class_name_chirho.as_str() {
                "Eq" | "Ord" => &["[Char]"],
                _ => &[],
            };

            for elem_type_chirho in known_element_types_chirho
                .iter()
                .chain(string_element_types_chirho)
            {
                let list_type_key_chirho = format!("[{}]", elem_type_chirho);

                // Skip if we already have a ground dict for this list type
                if self
                    .instance_dicts_chirho
                    .contains_key(&(class_name_chirho.clone(), list_type_key_chirho.clone()))
                {
                    continue;
                }

                // Check that all context classes have ground dicts
                // for the element type
                let all_context_satisfied_chirho =
                    context_classes_chirho.iter().all(|ctx_class_chirho| {
                        self.instance_dicts_chirho
                            .contains_key(&(ctx_class_chirho.clone(), elem_type_chirho.to_string()))
                    });

                if !all_context_satisfied_chirho {
                    continue;
                }

                // Generate the ground dict for this list type.
                // We need method implementations that use the element dict.
                match entry_chirho.route_chirho {
                    ConditionalListRouteChirho::SpelledChirho => self
                        .generate_list_instance_dict_chirho(
                            class_env_chirho,
                            class_name_chirho,
                            elem_type_chirho,
                            &list_type_key_chirho,
                            layout_chirho,
                            context_classes_chirho,
                        ),
                    ConditionalListRouteChirho::InstanceRowsChirho => self
                        .generate_generic_list_dict_chirho(
                            class_env_chirho,
                            class_name_chirho,
                            elem_type_chirho,
                            &list_type_key_chirho,
                            layout_chirho,
                            context_classes_chirho,
                        ),
                    ConditionalListRouteChirho::CompilerShowChirho(identity_chirho) => self
                        .generate_show_list_dict_chirho(
                            &list_type_key_chirho,
                            layout_chirho,
                            Some((identity_chirho, &namesake_rows_chirho)),
                        ),
                }
            }
        }
    }

    /// The route of one conditional list instance, read from its class proof and its owner, not
    /// its spelling (row 692, C10, #33179). A class proof alone never certifies a
    /// compiler-generated body: that takes the compiler's own instance (a seed), proven of the
    /// class its spelling names, at a complete `[v]` head. A displaced compiler class's instance
    /// never enters the spelling: only the compiler's own Show list instance is built, in that
    /// class's namespace with its context predicates' own namespaces; any other is a named gap,
    /// refused by name where it is requested. `None` builds nothing for the instance.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    fn conditional_list_entry_chirho(
        &self,
        class_name_chirho: &str,
        inst_chirho: &InstDeclChirho,
    ) -> Option<ConditionalListEntryChirho> {
        let compiler_owned_chirho = matches!(
            inst_chirho.instance_identity_chirho(),
            InstanceIdentityChirho::SeedChirho(_)
        ) && complete_list_head_chirho(inst_chirho);
        // Each context predicate's namespace, read from its own class proof.
        let context_classes_chirho = inst_chirho
            .context_chirho
            .iter()
            .map(|predicate_chirho| {
                self.proof_layout_chirho(
                    &predicate_chirho.class_name_chirho,
                    predicate_chirho.class_identity_chirho(),
                )
                .map(|context_layout_chirho| context_layout_chirho.class_name_chirho.clone())
            })
            .collect::<Option<Vec<_>>>()?;
        if let Some(identity_chirho) =
            self.displaced_identity_chirho(inst_chirho.class_identity_chirho())
        {
            if identity_chirho != CompilerClassChirho::ShowChirho || !compiler_owned_chirho {
                return None;
            }
            let layout_chirho = self
                .proof_layout_chirho(class_name_chirho, inst_chirho.class_identity_chirho())?
                .clone();
            return Some(ConditionalListEntryChirho {
                class_name_chirho: layout_chirho.class_name_chirho.clone(),
                layout_chirho,
                context_classes_chirho,
                route_chirho: ConditionalListRouteChirho::CompilerShowChirho(identity_chirho),
            });
        }
        let layout_chirho = self.layouts_chirho.get(class_name_chirho)?.clone();
        let proves_spelling_chirho = matches!(
            inst_chirho.class_identity_chirho().identity_chirho(),
            Some(PredicateClassIdentityChirho::CompilerChirho(identity_chirho))
                if identity_chirho.legacy_name_chirho() == class_name_chirho
        );
        let route_chirho = if matches!(class_name_chirho, "Eq" | "Show" | "Ord")
            && !(compiler_owned_chirho && proves_spelling_chirho)
        {
            ConditionalListRouteChirho::InstanceRowsChirho
        } else {
            ConditionalListRouteChirho::SpelledChirho
        };
        Some(ConditionalListEntryChirho {
            class_name_chirho: class_name_chirho.to_string(),
            layout_chirho,
            context_classes_chirho,
            route_chirho,
        })
    }

    /// Build the `[elem]` dictionary of a conditional list instance; the GND pass calls it for
    /// a list representation once the element's context dictionaries exist (row 694).
    pub(super) fn generate_list_instance_dict_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        elem_type_chirho: &str,
        list_type_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
        context_classes_chirho: &[String],
    ) {
        match class_name_chirho {
            "Eq" => self.generate_eq_list_dict_chirho(
                elem_type_chirho,
                list_type_key_chirho,
                layout_chirho,
            ),
            "Show" => {
                self.generate_show_list_dict_chirho(list_type_key_chirho, layout_chirho, None)
            }
            "Ord" => {
                // A real `Ord [T]` when the element's compare row has a body;
                // the generic path names the method otherwise.
                if !self.generate_ord_list_dict_chirho(
                    class_env_chirho,
                    elem_type_chirho,
                    list_type_key_chirho,
                    layout_chirho,
                ) {
                    self.generate_generic_list_dict_chirho(
                        class_env_chirho,
                        class_name_chirho,
                        elem_type_chirho,
                        list_type_key_chirho,
                        layout_chirho,
                        context_classes_chirho,
                    );
                }
            }
            _ => {
                // Generic handler for user-defined classes with conditional
                // list instances.  Look up the user's $prim_ method bindings
                // for the conditional instance (type key contains a variable,
                // e.g. "[a]") and build a dict constructor referencing them.
                self.generate_generic_list_dict_chirho(
                    class_env_chirho,
                    class_name_chirho,
                    elem_type_chirho,
                    list_type_key_chirho,
                    layout_chirho,
                    context_classes_chirho,
                );
            }
        }
    }

    /// Generate a conditional instance dict for a user-defined class
    /// by referencing the user's `$prim_` method bindings.
    fn generate_generic_list_dict_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        elem_type_chirho: &str,
        list_type_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
        context_classes_chirho: &[String],
    ) {
        let con_name_chirho = format!("$Dict_{}", class_name_chirho);
        let dict_name_chirho = format!("$f{}{}", class_name_chirho, list_type_key_chirho);
        let dict_ty_chirho = TyChirho::ConChirho(format!("$Dict_{}", class_name_chirho));
        let dict_binder_chirho =
            self.conditional_dict_binder_chirho(&dict_name_chirho, dict_ty_chirho);

        let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

        // Superclass dict arguments, whose refusals this speculative dictionary owns (#33201)
        for slot_chirho in &layout_chirho.super_slots_chirho {
            field_args_chirho.push(self.superclass_field_owned_chirho(
                slot_chirho,
                list_type_key_chirho,
                Some(dict_binder_chirho.id_chirho),
            ));
        }

        // Method arguments: find the user's $prim_ bindings.
        // Try type keys in order: "[a]", "[b]", etc. for variable-typed instances.
        let candidate_type_keys_chirho = vec![
            "[a]".to_string(),
            "[b]".to_string(),
            list_type_key_chirho.to_string(),
        ];

        for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
            let mut found_chirho = false;
            // The instance's own row, under whatever variable its head is written with
            // (`[aChirho]`), when the fixed spellings name none.
            let written_row_chirho =
                self.conditional_list_row_chirho(class_name_chirho, method_name_chirho);
            for tk_chirho in &candidate_type_keys_chirho {
                let prim_name_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, tk_chirho
                );
                if let Some(prim_id_chirho) = self
                    .lookup_body_backed_name_id_chirho(&prim_name_chirho)
                    .or(written_row_chirho)
                {
                    let mut method_expr_chirho = CoreExprChirho::VarChirho(prim_id_chirho);
                    for context_class_chirho in context_classes_chirho {
                        if let Some(context_dict_id_chirho) = self
                            .instance_dicts_chirho
                            .get(&(context_class_chirho.clone(), elem_type_chirho.to_string()))
                        {
                            method_expr_chirho = CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(method_expr_chirho),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                    *context_dict_id_chirho,
                                )),
                            };
                        }
                    }
                    field_args_chirho.push(method_expr_chirho);
                    found_chirho = true;
                    break;
                }
            }
            if !found_chirho {
                // The stub is named for the ground key, so it takes no element
                // dictionaries: a caller that finds it by name passes only the
                // method's arguments. With a leading dictionary parameter the
                // first argument bound as the dictionary, and the partial
                // application that came back was taken silently by a `case`
                // with a default branch (`map neededChirho [[1]]` printed [[1]]).
                // workflow: language-features-chirho/dictionary-evidence-chirho
                let prim_name_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, list_type_key_chirho
                );
                // GHC's defaults first: Eq's (row 606) and Ord's (row 612).
                let prim_id_chirho = match self.method_default_row_chirho(
                    class_name_chirho,
                    method_name_chirho,
                    list_type_key_chirho,
                    &prim_name_chirho,
                ) {
                    Some(default_id_chirho) => default_id_chirho,
                    None => self.generate_missing_method_binding_chirho(
                        class_env_chirho,
                        class_name_chirho,
                        method_name_chirho,
                        list_type_key_chirho,
                        &prim_name_chirho,
                    ),
                };
                field_args_chirho.push(CoreExprChirho::VarChirho(prim_id_chirho));
            }
        }

        // Build: $fClass[T] = $Dict_Class <super_dicts> <methods>
        let dict_body_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho: field_args_chirho,
        };

        self.instance_dicts_chirho.insert(
            (
                class_name_chirho.to_string(),
                list_type_key_chirho.to_string(),
            ),
            dict_binder_chirho.id_chirho,
        );

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: dict_binder_chirho,
            rhs_chirho: dict_body_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }

    /// The row a conditional list instance writes for `method_chirho`, under the variable its
    /// head is written with (`[a]`, `[aChirho]`): the class's only body-backed row keyed by a
    /// list of one type variable. `None` when there is none, or more than one.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    fn conditional_list_row_chirho(
        &self,
        class_name_chirho: &str,
        method_name_chirho: &str,
    ) -> Option<crate::expr_chirho::CoreIdChirho> {
        let prefix_chirho = format!("$prim_{class_name_chirho}_{method_name_chirho}_[");
        let mut rows_chirho =
            self.body_backed_names_chirho
                .iter()
                .filter_map(|(row_name_chirho, row_id_chirho)| {
                    let variable_chirho = row_name_chirho
                        .strip_prefix(&prefix_chirho)?
                        .strip_suffix(']')?;
                    let is_variable_chirho =
                        variable_chirho.chars().next().is_some_and(|first_chirho| {
                            first_chirho.is_lowercase() || first_chirho == '_'
                        }) && variable_chirho.chars().all(|character_chirho| {
                            character_chirho.is_alphanumeric()
                                || character_chirho == '_'
                                || character_chirho == '\''
                        });
                    is_variable_chirho.then_some(*row_id_chirho)
                });
        let only_chirho = rows_chirho.next()?;
        rows_chirho.next().is_none().then_some(only_chirho)
    }

    /// Generate `Eq [T]` dict with a recursive list equality function.
    fn generate_eq_list_dict_chirho(
        &mut self,
        elem_type_chirho: &str,
        list_type_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
    ) {
        let str_ty_chirho = TyChirho::string_chirho();

        // Int, Char and Bool compare with `==#` and Double with `eqFloat#`. Any other
        // element takes its own body-backed `Eq.==` row: `[String]` compared String
        // addresses natively (row 603). An element without a row keeps today's `==#`.
        let element_row_chirho = match elem_type_chirho {
            "Double" | "Int" | "Char" | "Bool" => None,
            _ => self.ensure_eq_row_chirho(elem_type_chirho),
        };
        let elem_eq_primop_chirho = if elem_type_chirho == "Double" {
            "eqFloat#"
        } else {
            "==#"
        };

        // Generate the recursive list equality function:
        // $eqList_T = \xs ys -> case xs of
        //   [] -> case ys of
        //     [] -> True (ConApp("True", []))
        //     (:) _ _ -> False (ConApp("False", []))
        //   (:) x xs' -> case ys of
        //     [] -> False
        //     (:) y ys' -> if elem_eq# x y then $eqList_T xs' ys' else False
        let fn_name_chirho = format!("$eqList_{}", elem_type_chirho);
        let fn_id_chirho = self.resolve_or_fresh_id_chirho(&fn_name_chirho);

        let xs_chirho = self.fresh_binder_chirho("xs", str_ty_chirho.clone());
        let ys_chirho = self.fresh_binder_chirho("ys", str_ty_chirho.clone());
        let x_chirho = self.fresh_binder_chirho("x", str_ty_chirho.clone());
        let xs2_chirho = self.fresh_binder_chirho("xs'", str_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("y", str_ty_chirho.clone());
        let ys2_chirho = self.fresh_binder_chirho("ys'", str_ty_chirho.clone());
        let w1_chirho = self.fresh_binder_chirho("_w1", str_ty_chirho.clone());
        let w2_chirho = self.fresh_binder_chirho("_w2", str_ty_chirho.clone());

        // The core: if elem_eq# x y then $eqList_T xs' ys' else False
        let recurse_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(fn_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs2_chirho.id_chirho)),
            }),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(ys2_chirho.id_chirho)),
        };

        let eq_test_chirho = match element_row_chirho {
            Some(row_chirho) => Self::apply_two_chirho(
                row_chirho,
                CoreExprChirho::VarChirho(x_chirho.id_chirho),
                CoreExprChirho::VarChirho(y_chirho.id_chirho),
            ),
            None => CoreExprChirho::PrimOpChirho {
                name_chirho: elem_eq_primop_chirho.to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    CoreExprChirho::VarChirho(y_chirho.id_chirho),
                ],
            },
        };

        let true_con_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "True".to_string(),
            args_chirho: vec![],
        };
        let false_con_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "False".to_string(),
            args_chirho: vec![],
        };

        // if elem_eq x y then recurse else False
        let if_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(eq_test_chirho),
            bind_chirho: self.fresh_binder_chirho("_eq", str_ty_chirho.clone()),
            result_ty_chirho: TyChirho::bool_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: recurse_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("False".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: false_con_chirho.clone(),
                },
            ],
        };

        // case ys of [] -> False; (:) y ys' -> if_body
        let inner_case_cons_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
            bind_chirho: self.fresh_binder_chirho("_ys", str_ty_chirho.clone()),
            result_ty_chirho: TyChirho::bool_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: false_con_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![y_chirho.clone(), ys2_chirho.clone()],
                    rhs_chirho: if_body_chirho,
                },
            ],
        };

        // case ys of [] -> True; (:) _ _ -> False (when xs is [])
        let inner_case_nil_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
            bind_chirho: self.fresh_binder_chirho("_ys2", str_ty_chirho.clone()),
            result_ty_chirho: TyChirho::bool_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: true_con_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![w1_chirho, w2_chirho],
                    rhs_chirho: false_con_chirho.clone(),
                },
            ],
        };

        // Outer: case xs of [] -> inner_nil; (:) x xs' -> inner_cons
        let outer_case_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            bind_chirho: self.fresh_binder_chirho("_xs", str_ty_chirho.clone()),
            result_ty_chirho: TyChirho::bool_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_case_nil_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![x_chirho.clone(), xs2_chirho.clone()],
                    rhs_chirho: inner_case_cons_chirho,
                },
            ],
        };

        // \xs ys -> outer_case
        let fn_body_chirho = CoreExprChirho::LamChirho {
            binder_chirho: xs_chirho,
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: ys_chirho,
                body_chirho: Box::new(outer_case_chirho),
            }),
        };

        // Register the recursive eq function
        let fn_binder_chirho = BinderChirho {
            id_chirho: fn_id_chirho,
            name_chirho: fn_name_chirho,
            ty_chirho: str_ty_chirho.clone(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: fn_binder_chirho,
            rhs_chirho: fn_body_chirho,
            is_rec_chirho: true,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });

        // Now generate the prim binding that wraps this function
        let prim_name_chirho = format!("$prim_Eq_==_{}", list_type_key_chirho);
        let prim_binder_chirho = self.fresh_binder_chirho(&prim_name_chirho, str_ty_chirho.clone());
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: prim_binder_chirho,
            rhs_chirho: CoreExprChirho::VarChirho(fn_id_chirho),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });

        // Its `/=` row defaults from that `==` (row 606).
        self.equality_default_row_chirho(
            "Eq",
            "/=",
            list_type_key_chirho,
            &format!("$prim_Eq_/=_{list_type_key_chirho}"),
        );

        // Generate the Eq dict for this list type
        let dict_name_chirho = format!("$fEq{}", list_type_key_chirho);
        let dict_ty_chirho = TyChirho::ConChirho("$Dict_Eq".to_string());
        let dict_binder_chirho =
            self.conditional_dict_binder_chirho(&dict_name_chirho, dict_ty_chirho);
        let dict_id_chirho = dict_binder_chirho.id_chirho;

        let con_name_chirho = "$Dict_Eq".to_string();
        let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

        // Superclass dicts (Eq has none), whose refusals this speculative dictionary owns (#33201)
        for slot_chirho in &layout_chirho.super_slots_chirho {
            field_args_chirho.push(self.superclass_field_owned_chirho(
                slot_chirho,
                list_type_key_chirho,
                Some(dict_id_chirho),
            ));
        }

        // Method implementations
        for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
            let impl_name_chirho =
                format!("$prim_Eq_{}_{}", method_name_chirho, list_type_key_chirho);
            let impl_id_chirho = self.resolve_or_fresh_id_chirho(&impl_name_chirho);
            field_args_chirho.push(CoreExprChirho::VarChirho(impl_id_chirho));
        }

        let dict_expr_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho: field_args_chirho,
        };

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: dict_binder_chirho,
            rhs_chirho: dict_expr_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });

        self.instance_dicts_chirho.insert(
            ("Eq".to_string(), list_type_key_chirho.to_string()),
            dict_id_chirho,
        );
    }

    /// Generate `Show [T]` dict for element types other than [Int] and [Char], named in its
    /// layout's namespace: the spelling `Show`, or the reserved namespace of a displaced
    /// compiler Show, whose method fields only that class's own rows fill
    /// (`compiler_method_row_chirho`, row 692, C10, #33179).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    fn generate_show_list_dict_chirho(
        &mut self,
        list_type_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
        compiler_rows_chirho: Option<(CompilerClassChirho, &HashSet<(String, String)>)>,
    ) {
        let namespace_chirho = &layout_chirho.class_name_chirho;
        self.generate_show_key_binding_chirho(
            &format!("$prim_{namespace_chirho}_show_{list_type_key_chirho}"),
            list_type_key_chirho,
        );

        // Generate the Show dict for this list type
        let dict_name_chirho = format!("$f{namespace_chirho}{list_type_key_chirho}");
        let dict_ty_chirho = TyChirho::ConChirho(format!("$Dict_{namespace_chirho}"));
        let dict_binder_chirho =
            self.conditional_dict_binder_chirho(&dict_name_chirho, dict_ty_chirho);
        let dict_id_chirho = dict_binder_chirho.id_chirho;

        let con_name_chirho = format!("$Dict_{namespace_chirho}");
        let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

        // A speculative dictionary owns its superclass refusals (#33201).
        for slot_chirho in &layout_chirho.super_slots_chirho {
            field_args_chirho.push(self.superclass_field_owned_chirho(
                slot_chirho,
                list_type_key_chirho,
                Some(dict_id_chirho),
            ));
        }

        for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
            let impl_name_chirho =
                format!("$prim_{namespace_chirho}_{method_name_chirho}_{list_type_key_chirho}");
            let impl_id_chirho = match compiler_rows_chirho {
                Some((identity_chirho, namesake_rows_chirho)) => self.compiler_method_row_chirho(
                    identity_chirho,
                    &impl_name_chirho,
                    method_name_chirho,
                    list_type_key_chirho,
                    namesake_rows_chirho,
                ),
                None => self.resolve_or_fresh_id_chirho(&impl_name_chirho),
            };
            field_args_chirho.push(CoreExprChirho::VarChirho(impl_id_chirho));
        }

        let dict_expr_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho: field_args_chirho,
        };

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: dict_binder_chirho,
            rhs_chirho: dict_expr_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });

        self.instance_dicts_chirho.insert(
            (namespace_chirho.clone(), list_type_key_chirho.to_string()),
            dict_id_chirho,
        );
    }
}
