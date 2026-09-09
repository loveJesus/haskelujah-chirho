// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Conditional list dictionaries and their stable, possibly forward-referenced identities.

use super::{DictLayoutChirho, DictPassCtxChirho};
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreLitChirho,
    InlineAnnotationChirho,
};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

impl DictPassCtxChirho {
    /// A superclass reference may reserve this name before its body is built.
    /// Reuse that identity; a fresh ID would leave the earlier reference unbound.
    fn conditional_dict_binder_chirho(
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
    pub fn generate_conditional_ground_dicts_chirho(&mut self, class_env_chirho: &ClassEnvChirho) {
        // Collect conditional list instances: (class, context_classes)
        let mut cond_list_instances_chirho: Vec<(String, Vec<String>)> = Vec::new();

        for (class_name_chirho, instances_chirho) in &class_env_chirho.instances_chirho {
            for inst_chirho in instances_chirho {
                if inst_chirho.context_chirho.is_empty() {
                    continue;
                }
                // Check if this is a list instance: head_ty is List(Var(_))
                if let TyChirho::ListChirho(_) = &inst_chirho.head_ty_chirho {
                    let context_classes_chirho: Vec<String> = inst_chirho
                        .context_chirho
                        .iter()
                        .map(|p_chirho| p_chirho.class_name_chirho.clone())
                        .collect();
                    cond_list_instances_chirho
                        .push((class_name_chirho.clone(), context_classes_chirho));
                }
            }
        }

        // For each conditional list instance, generate ground dicts
        // for known element types that have the required dicts.
        let known_element_types_chirho = vec!["Int", "Char", "Bool", "Double"];

        for (class_name_chirho, context_classes_chirho) in &cond_list_instances_chirho {
            let layout_chirho = match self.layouts_chirho.get(class_name_chirho) {
                Some(l_chirho) => l_chirho.clone(),
                None => continue,
            };

            for elem_type_chirho in &known_element_types_chirho {
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
                self.generate_list_instance_dict_chirho(
                    class_env_chirho,
                    class_name_chirho,
                    elem_type_chirho,
                    &list_type_key_chirho,
                    &layout_chirho,
                    context_classes_chirho,
                );
            }
        }
    }

    fn generate_list_instance_dict_chirho(
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
            "Show" => self.generate_show_list_dict_chirho(
                elem_type_chirho,
                list_type_key_chirho,
                layout_chirho,
            ),
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

        // Superclass dict arguments
        for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
            let super_dict_name_chirho = format!("$f{}{}", super_name_chirho, list_type_key_chirho);
            let super_dict_id_chirho = self.resolve_or_fresh_id_chirho(&super_dict_name_chirho);
            field_args_chirho.push(CoreExprChirho::VarChirho(super_dict_id_chirho));
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
            for tk_chirho in &candidate_type_keys_chirho {
                let prim_name_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, tk_chirho
                );
                if let Some(prim_id_chirho) =
                    self.lookup_body_backed_name_id_chirho(&prim_name_chirho)
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
                let prim_name_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, list_type_key_chirho
                );
                let prim_id_chirho = self.generate_missing_method_binding_chirho(
                    class_env_chirho,
                    class_name_chirho,
                    method_name_chirho,
                    list_type_key_chirho,
                    &prim_name_chirho,
                    context_classes_chirho.len(),
                );
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

    /// Generate `Eq [T]` dict with a recursive list equality function.
    fn generate_eq_list_dict_chirho(
        &mut self,
        elem_type_chirho: &str,
        list_type_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
    ) {
        let str_ty_chirho = TyChirho::string_chirho();

        // Determine the element equality primop
        let elem_eq_primop_chirho = match elem_type_chirho {
            "Double" => "eqFloat#",
            "Bool" => "==#",
            _ => "==#",
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

        let eq_test_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: elem_eq_primop_chirho.to_string(),
            args_chirho: vec![
                CoreExprChirho::VarChirho(x_chirho.id_chirho),
                CoreExprChirho::VarChirho(y_chirho.id_chirho),
            ],
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

        // Generate the Eq dict for this list type
        let dict_name_chirho = format!("$fEq{}", list_type_key_chirho);
        let dict_ty_chirho = TyChirho::ConChirho("$Dict_Eq".to_string());
        let dict_binder_chirho =
            self.conditional_dict_binder_chirho(&dict_name_chirho, dict_ty_chirho);
        let dict_id_chirho = dict_binder_chirho.id_chirho;

        let con_name_chirho = "$Dict_Eq".to_string();
        let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

        // Superclass dicts (Eq has none)
        for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
            let super_dict_name_chirho = format!("$f{}{}", super_name_chirho, list_type_key_chirho);
            let super_dict_id_chirho = self.resolve_or_fresh_id_chirho(&super_dict_name_chirho);
            field_args_chirho.push(CoreExprChirho::VarChirho(super_dict_id_chirho));
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

    /// Generate `Show [T]` dict for element types other than [Int] and [Char].
    fn generate_show_list_dict_chirho(
        &mut self,
        elem_type_chirho: &str,
        list_type_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
    ) {
        let str_ty_chirho = TyChirho::string_chirho();

        // Determine the element show primop
        let elem_show_primop_chirho = match elem_type_chirho {
            "Double" => "showFloat#",
            "Bool" => "showBool#",
            _ => "showInt#",
        };

        // Generate recursive show list function, similar to generate_show_list_int_binding_chirho
        // $showListTail_T = \xs -> case xs of
        //     [] -> "]"
        //     (:) x rest -> ++# "," (++# (showElem# x) ($showListTail_T rest))
        let tail_fn_name_chirho = format!("$showListTail_{}", elem_type_chirho);
        let tail_fn_id_chirho = self.resolve_or_fresh_id_chirho(&tail_fn_name_chirho);

        let tail_xs_chirho = self.fresh_binder_chirho("xs", str_ty_chirho.clone());
        let tail_x_chirho = self.fresh_binder_chirho("x", str_ty_chirho.clone());
        let tail_rest_chirho = self.fresh_binder_chirho("rest", str_ty_chirho.clone());

        let tail_cons_rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "++#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(",".to_string())),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "++#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::PrimOpChirho {
                            name_chirho: elem_show_primop_chirho.to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(tail_x_chirho.id_chirho)],
                        },
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(tail_fn_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                tail_rest_chirho.id_chirho,
                            )),
                        },
                    ],
                },
            ],
        };

        let tail_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(tail_xs_chirho.id_chirho)),
            bind_chirho: self.fresh_binder_chirho("_t", str_ty_chirho.clone()),
            result_ty_chirho: TyChirho::string_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "]".to_string(),
                    )),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![tail_x_chirho.clone(), tail_rest_chirho.clone()],
                    rhs_chirho: tail_cons_rhs_chirho,
                },
            ],
        };

        let tail_fn_rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: tail_xs_chirho,
            body_chirho: Box::new(tail_body_chirho),
        };

        let tail_binder_chirho = BinderChirho {
            id_chirho: tail_fn_id_chirho,
            name_chirho: tail_fn_name_chirho,
            ty_chirho: str_ty_chirho.clone(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: tail_binder_chirho,
            rhs_chirho: tail_fn_rhs_chirho,
            is_rec_chirho: true,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });

        // Main show function:
        // $prim_Show_show_[T] = \xs -> case xs of
        //     [] -> "[]"
        //     (:) x rest -> ++# "[" (++# (showElem# x) ($showListTail_T rest))
        let main_xs_chirho = self.fresh_binder_chirho("xs", str_ty_chirho.clone());
        let main_x_chirho = self.fresh_binder_chirho("x", str_ty_chirho.clone());
        let main_rest_chirho = self.fresh_binder_chirho("rest", str_ty_chirho.clone());

        let main_cons_rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "++#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::StringChirho("[".to_string())),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "++#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::PrimOpChirho {
                            name_chirho: elem_show_primop_chirho.to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(main_x_chirho.id_chirho)],
                        },
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(tail_fn_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                main_rest_chirho.id_chirho,
                            )),
                        },
                    ],
                },
            ],
        };

        let main_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(main_xs_chirho.id_chirho)),
            bind_chirho: self.fresh_binder_chirho("_m", str_ty_chirho.clone()),
            result_ty_chirho: TyChirho::string_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "[]".to_string(),
                    )),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![main_x_chirho.clone(), main_rest_chirho.clone()],
                    rhs_chirho: main_cons_rhs_chirho,
                },
            ],
        };

        let main_fn_rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: main_xs_chirho,
            body_chirho: Box::new(main_body_chirho),
        };

        let prim_name_chirho = format!("$prim_Show_show_{}", list_type_key_chirho);
        let prim_binder_chirho = self.fresh_binder_chirho(&prim_name_chirho, str_ty_chirho.clone());
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: prim_binder_chirho,
            rhs_chirho: main_fn_rhs_chirho,
            is_rec_chirho: true,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });

        // Generate the Show dict for this list type
        let dict_name_chirho = format!("$fShow{}", list_type_key_chirho);
        let dict_ty_chirho = TyChirho::ConChirho("$Dict_Show".to_string());
        let dict_binder_chirho =
            self.conditional_dict_binder_chirho(&dict_name_chirho, dict_ty_chirho);
        let dict_id_chirho = dict_binder_chirho.id_chirho;

        let con_name_chirho = "$Dict_Show".to_string();
        let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

        for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
            let super_dict_name_chirho = format!("$f{}{}", super_name_chirho, list_type_key_chirho);
            let super_dict_id_chirho = self.resolve_or_fresh_id_chirho(&super_dict_name_chirho);
            field_args_chirho.push(CoreExprChirho::VarChirho(super_dict_id_chirho));
        }

        for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
            let impl_name_chirho =
                format!("$prim_Show_{}_{}", method_name_chirho, list_type_key_chirho);
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
            ("Show".to_string(), list_type_key_chirho.to_string()),
            dict_id_chirho,
        );
    }
}
