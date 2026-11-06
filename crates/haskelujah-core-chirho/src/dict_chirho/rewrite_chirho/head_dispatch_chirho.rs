// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Dispatch chosen by a type head: higher-kinded methods, Semigroup/Monoid bodies, annotated
//! method applications, contextual return/pure, and the monad head keys they read.

use std::collections::{HashMap, HashSet};

use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::DictPassCtxChirho;
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho};

impl DictPassCtxChirho {
    pub(super) fn annotated_monad_head_key_chirho(&self, ty_chirho: &TyChirho) -> Option<String> {
        self.body_backed_monad_head_key_for_ty_chirho(ty_chirho, false)
    }

    pub(super) fn body_backed_monad_head_key_for_ty_chirho(
        &self,
        ty_chirho: &TyChirho,
        follow_fun_result_chirho: bool,
    ) -> Option<String> {
        let head_key_chirho = Self::raw_type_head_key_chirho(ty_chirho, follow_fun_result_chirho)?;
        let prim_name_chirho = format!("$prim_Applicative_pure_{head_key_chirho}");
        self.lookup_dispatch_body_name_id_chirho(&prim_name_chirho)
            .map(|_| head_key_chirho)
    }

    pub(super) fn raw_type_head_key_chirho(
        ty_chirho: &TyChirho,
        follow_fun_result_chirho: bool,
    ) -> Option<String> {
        match ty_chirho {
            TyChirho::ForallChirho { body_chirho, .. } => {
                Self::raw_type_head_key_chirho(body_chirho, follow_fun_result_chirho)
            }
            TyChirho::FunChirho(_, result_ty_chirho, _) if follow_fun_result_chirho => {
                Self::raw_type_head_key_chirho(result_ty_chirho, true)
            }
            TyChirho::FunChirho(_, _, _) => None,
            TyChirho::ListChirho(_) => Some("[]".to_string()),
            TyChirho::AppChirho(fun_chirho, _) => Self::raw_type_head_key_chirho(fun_chirho, false),
            TyChirho::ConChirho(name_chirho) => {
                Some(Self::normalize_instance_head_key_chirho(name_chirho))
            }
            _ => None,
        }
    }

    pub(super) fn has_direct_return_or_pure_head_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
    ) -> bool {
        let mut cur_chirho = expr_chirho;
        loop {
            match cur_chirho {
                CoreExprChirho::AppChirho { fun_chirho, .. } => {
                    cur_chirho = fun_chirho;
                }
                CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
                    cur_chirho = expr_chirho;
                }
                CoreExprChirho::VarChirho(id_chirho) => {
                    return self
                        .names_chirho
                        .get(id_chirho)
                        .map(|name_chirho| name_chirho == "return" || name_chirho == "pure")
                        .unwrap_or(false);
                }
                _ => return false,
            }
        }
    }

    pub(super) fn try_dispatch_contextual_return_pure_var_chirho(
        &self,
        id_chirho: CoreIdChirho,
    ) -> Option<CoreExprChirho> {
        if self.local_shadow_ids_chirho.borrow().contains(&id_chirho) {
            return None;
        }
        let name_chirho = self.names_chirho.get(&id_chirho)?;
        if name_chirho != "return" && name_chirho != "pure" {
            return None;
        }
        let context_key_chirho = self.monad_context_stack_chirho.borrow().last().cloned()?;
        let inst_id_chirho = self.lookup_dispatch_body_name_id_chirho(&format!(
            "$prim_Applicative_pure_{context_key_chirho}"
        ))?;
        Some(CoreExprChirho::VarChirho(inst_id_chirho))
    }

    fn rebuild_preserved_method_app_chirho(
        &self,
        head_id_chirho: CoreIdChirho,
        args_chirho: &[&CoreExprChirho],
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> CoreExprChirho {
        let mut result_chirho = CoreExprChirho::VarChirho(head_id_chirho);
        for a_chirho in args_chirho {
            result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                    a_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                )),
            };
        }
        result_chirho
    }

    /// Conservative type key for monad-chain dispatch (`>>=` / `>>`): only
    /// shapes that PROVE the monad head are trusted — constructor
    /// applications (Just x, Right y, x : xs), constructor vars, locally
    /// keyed vars, literals, primops with exact result types, and type
    /// annotations. Non-constructor function applications are NOT trusted:
    /// the general inference blindly propagates argument types through Apps,
    /// which routed IO chains into the list monad (`putStrLn "a" >> ...`
    /// inferred as `[Char]` -> `[]`). workflow: monadic-dispatch-chirho
    fn infer_monad_dispatch_key_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<String> {
        match expr_chirho {
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
            } => self
                .annotated_monad_head_key_chirho(ty_chirho)
                .filter(|_| self.has_direct_return_or_pure_head_chirho(inner_chirho))
                .or_else(|| {
                    self.infer_monad_dispatch_key_chirho(inner_chirho, local_type_keys_chirho)
                }),
            CoreExprChirho::VarChirho(id_chirho) => local_type_keys_chirho
                .get(id_chirho)
                .cloned()
                .or_else(|| self.infer_type_key_chirho(expr_chirho)),
            CoreExprChirho::ConAppChirho { .. }
            | CoreExprChirho::LitChirho(_)
            | CoreExprChirho::PrimOpChirho { .. } => self.infer_type_key_chirho(expr_chirho),
            CoreExprChirho::AppChirho { .. } => {
                let mut cur_chirho = expr_chirho;
                while let CoreExprChirho::AppChirho { fun_chirho, .. } = cur_chirho {
                    cur_chirho = fun_chirho;
                }
                while let CoreExprChirho::TyAppChirho {
                    expr_chirho: inner_chirho,
                    ..
                } = cur_chirho
                {
                    cur_chirho = inner_chirho;
                }
                let CoreExprChirho::VarChirho(head_id_chirho) = cur_chirho else {
                    return None;
                };
                let name_chirho = self.names_chirho.get(head_id_chirho)?;
                let is_con_head_chirho = self.con_types_chirho.contains_key(name_chirho)
                    || matches!(
                        name_chirho.as_str(),
                        "Just"
                            | "Left"
                            | "Right"
                            | "Down"
                            | "Endo"
                            | ":"
                            | "(,)"
                            | "(,,)"
                            | "(,,,)"
                    )
                    || name_chirho.starts_with("$tuple");
                if is_con_head_chirho {
                    return self.infer_type_key_chirho(expr_chirho);
                }
                None
            }
            _ => None,
        }
    }

    /// Dispatch a higher-kinded class method to the correct per-type instance
    /// body based on the type key of its dispatch argument.
    /// The method otherwise resolves to a monomorphic default binding, which is
    /// left in place (returns `None`) when the key cannot be determined — so IO
    /// / unknown cases keep their existing behaviour.
    pub(super) fn try_dispatch_hk_method_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        // Collect the application spine: head + ordered args.
        let mut args_rev_chirho: Vec<&CoreExprChirho> = Vec::new();
        let mut cur_chirho = expr_chirho;
        while let CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } = cur_chirho
        {
            args_rev_chirho.push(arg_chirho);
            cur_chirho = fun_chirho;
        }
        let head_id_chirho = match cur_chirho {
            CoreExprChirho::VarChirho(id_chirho) => *id_chirho,
            _ => return None,
        };
        // Locally-shadowed names (`let (>>=) = ...`) are never intercepted.
        if self
            .local_shadow_ids_chirho
            .borrow()
            .contains(&head_id_chirho)
        {
            return None;
        }
        let head_name_chirho = self.names_chirho.get(&head_id_chirho)?.clone();
        let args_chirho: Vec<&CoreExprChirho> = args_rev_chirho.iter().rev().copied().collect();

        // return/pure are return-position polymorphic: no argument carries the
        // monad, so dispatch from the innermost positively-dispatched chain
        // context; with no context they keep their name (ReturnIOChirho
        // fallback, INV-001). workflow: monadic-dispatch-chirho
        if head_name_chirho == "return" || head_name_chirho == "pure" {
            // Both names are served by Applicative.pure; Monad supplies its
            // Applicative superclass. An own dictionary is stronger evidence
            // than an enclosing expression's monad guess, including for return,
            // which need not itself be a class method.
            if let Some((class_chirho, selector_chirho)) =
                self.class_method_selector_for_name_chirho("pure")
                && let Some(dictionary_chirho) = Self::fallback_dict_for_class_chirho(
                    &class_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                )
            {
                let mut result_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(selector_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(dictionary_chirho)),
                };
                for argument_chirho in &args_chirho {
                    result_chirho = CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(result_chirho),
                        arg_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                            argument_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        )),
                    };
                }
                return Some(result_chirho);
            }
            let Some(context_key_chirho) = self.monad_context_stack_chirho.borrow().last().cloned()
            else {
                return Some(self.rebuild_preserved_method_app_chirho(
                    head_id_chirho,
                    &args_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            };
            let Some(inst_id_chirho) = self.lookup_dispatch_body_name_id_chirho(&format!(
                "$prim_Applicative_pure_{context_key_chirho}"
            )) else {
                return Some(self.rebuild_preserved_method_app_chirho(
                    head_id_chirho,
                    &args_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            };
            let mut result_chirho = CoreExprChirho::VarChirho(inst_id_chirho);
            for a_chirho in &args_chirho {
                result_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(result_chirho),
                    arg_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )),
                };
            }
            return Some(result_chirho);
        }

        if head_name_chirho == "fail" {
            let Some(context_key_chirho) = self.monad_context_stack_chirho.borrow().last().cloned()
            else {
                return Some(self.rebuild_preserved_method_app_chirho(
                    head_id_chirho,
                    &args_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            };
            let Some(inst_id_chirho) = self.lookup_dispatch_body_name_id_chirho(&format!(
                "$prim_MonadFail_fail_{context_key_chirho}"
            )) else {
                return Some(self.rebuild_preserved_method_app_chirho(
                    head_id_chirho,
                    &args_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            };
            let mut result_chirho = CoreExprChirho::VarChirho(inst_id_chirho);
            for a_chirho in &args_chirho {
                result_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(result_chirho),
                    arg_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )),
                };
            }
            return Some(result_chirho);
        }

        // (instance-body name prefix, candidate dispatch-argument indexes)
        let (prefix_chirho, dispatch_indices_chirho): (&str, &[usize]) =
            match head_name_chirho.as_str() {
                "fmap" => ("$prim_Functor_fmap_", &[1usize]),
                ">>=" => ("$prim_Monad_>>=_", &[0usize]),
                ">>" => ("$prim_Monad_>>_", &[0usize]),
                "<|>" => ("$prim_Alternative_<|>_", &[1usize, 0usize]),
                "mplus" => ("$prim_MonadPlus_mplus_", &[1usize, 0usize]),
                _ => return None,
            };
        let dispatch_arg_chirho = dispatch_indices_chirho
            .iter()
            .find_map(|idx_chirho| args_chirho.get(*idx_chirho))?;
        // >>= / >> / mplus use the conservative key: a wrong positive key sends an IO
        // chain into another monad's body (worse than no dispatch). fmap and
        // <|> keep permissive value-shape inference because their dispatch
        // operands are plain values, not effectful IO chains.
        let value_key_opt_chirho = if matches!(head_name_chirho.as_str(), ">>=" | ">>" | "mplus") {
            self.infer_monad_dispatch_key_chirho(dispatch_arg_chirho, local_type_keys_chirho)
        } else {
            dispatch_indices_chirho.iter().find_map(|idx_chirho| {
                args_chirho.get(*idx_chirho).and_then(|arg_chirho| {
                    self.infer_type_key_for_rewrite_chirho(arg_chirho, local_type_keys_chirho)
                })
            })
        };
        let Some(value_key_chirho) = value_key_opt_chirho else {
            if matches!(head_name_chirho.as_str(), ">>=" | ">>" | "mplus") {
                return Some(self.rebuild_preserved_method_app_chirho(
                    head_id_chirho,
                    &args_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            }
            return None;
        };
        let head_key_chirho = Self::normalize_instance_head_key_chirho(&value_key_chirho);
        let Some(inst_id_chirho) =
            self.lookup_dispatch_body_name_id_chirho(&format!("{prefix_chirho}{head_key_chirho}"))
        else {
            if matches!(head_name_chirho.as_str(), ">>=" | ">>" | "mplus") {
                return Some(self.rebuild_preserved_method_app_chirho(
                    head_id_chirho,
                    &args_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            }
            return None;
        };
        // Monad chains carry their head key while their arguments are
        // rewritten so return/pure in the continuation dispatch to the same
        // monad. workflow: monadic-dispatch-chirho
        let is_monad_chain_chirho = matches!(head_name_chirho.as_str(), ">>=" | ">>");
        if is_monad_chain_chirho {
            self.monad_context_stack_chirho
                .borrow_mut()
                .push(head_key_chirho.clone());
        }
        let bind_payload_key_chirho = if head_name_chirho == ">>=" {
            self.monad_payload_type_key_chirho(
                dispatch_arg_chirho,
                &value_key_chirho,
                local_type_keys_chirho,
            )
        } else {
            None
        };
        // Rebuild the application with the instance body as head; rewrite args.
        let mut result_chirho = CoreExprChirho::VarChirho(inst_id_chirho);
        for (idx_chirho, a_chirho) in args_chirho.iter().enumerate() {
            let rewritten_arg_chirho = if matches!(head_name_chirho.as_str(), "<|>" | "mplus") {
                if let Some(rewritten_chirho) = self.try_rewrite_typed_method_arg_chirho(
                    a_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    &head_key_chirho,
                ) {
                    rewritten_chirho
                } else {
                    self.rewrite_method_refs_with_locals_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )
                }
            } else if idx_chirho == 1 && head_name_chirho == ">>=" {
                if let Some(payload_key_chirho) = bind_payload_key_chirho.as_deref() {
                    let continuation_type_keys_chirho = Self::extend_first_lambda_type_key_chirho(
                        a_chirho,
                        payload_key_chirho,
                        local_type_keys_chirho,
                    );
                    self.rewrite_method_refs_with_locals_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        &continuation_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )
                } else {
                    self.rewrite_method_refs_with_locals_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )
                }
            } else {
                self.rewrite_method_refs_with_locals_chirho(
                    a_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                )
            };
            result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(rewritten_arg_chirho),
            };
        }
        if is_monad_chain_chirho {
            self.monad_context_stack_chirho.borrow_mut().pop();
        }
        Some(result_chirho)
    }

    pub(super) fn try_dispatch_semigroup_monoid_body_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let (method_id_chirho, args_chirho, type_key_hint_chirho) =
            self.collect_method_app_chirho(expr_chirho)?;
        let method_name_chirho = self.names_chirho.get(&method_id_chirho)?;
        if self
            .local_shadow_ids_chirho
            .borrow()
            .contains(&method_id_chirho)
        {
            return None;
        }
        let short_name_chirho = method_name_chirho
            .rsplit('.')
            .next()
            .unwrap_or(method_name_chirho);
        let method_key_chirho = short_name_chirho
            .strip_prefix('(')
            .and_then(|name_chirho| name_chirho.strip_suffix(')'))
            .unwrap_or(short_name_chirho);

        let (prefix_chirho, dispatch_indices_chirho): (&str, &[usize]) = match method_key_chirho {
            "<>" => ("$prim_Semigroup_<>_", &[0usize, 1usize]),
            // Monoid.mappend has the same operational body as Semigroup.(<>)
            // for every generated ground row, including newtype-erased wrappers.
            "mappend" => ("$prim_Semigroup_<>_", &[0usize, 1usize]),
            "mconcat" => ("$prim_Monoid_mconcat_", &[0usize]),
            _ => return None,
        };

        let mut key_chirho = type_key_hint_chirho.or_else(|| {
            dispatch_indices_chirho.iter().find_map(|idx_chirho| {
                args_chirho.get(*idx_chirho).and_then(|arg_chirho| {
                    self.infer_type_key_for_rewrite_chirho(arg_chirho, local_type_keys_chirho)
                })
            })
        })?;

        if method_key_chirho == "mconcat"
            && key_chirho.starts_with('[')
            && key_chirho.ends_with(']')
            && key_chirho.len() > 2
        {
            key_chirho = key_chirho[1..key_chirho.len() - 1].to_string();
        }

        let inst_id_chirho =
            self.lookup_dispatch_body_name_id_chirho(&format!("{prefix_chirho}{key_chirho}"))?;
        let mut result_chirho = CoreExprChirho::VarChirho(inst_id_chirho);
        for arg_chirho in &args_chirho {
            let rewritten_arg_chirho = self
                .try_rewrite_typed_method_arg_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    &key_chirho,
                )
                .unwrap_or_else(|| {
                    self.rewrite_method_refs_with_locals_chirho(
                        arg_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )
                });
            result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(rewritten_arg_chirho),
            };
        }
        Some(result_chirho)
    }

    pub(super) fn try_rewrite_annotated_method_app_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        ty_chirho: &TyChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let type_key_chirho = Self::raw_type_head_key_chirho(ty_chirho, false)?;
        let mut args_chirho = Vec::new();
        let mut head_chirho = expr_chirho;
        while let CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } = head_chirho
        {
            args_chirho.push(arg_chirho.as_ref());
            head_chirho = fun_chirho.as_ref();
        }
        args_chirho.reverse();
        let CoreExprChirho::VarChirho(head_id_chirho) = head_chirho else {
            return None;
        };
        let method_name_chirho = self.names_chirho.get(head_id_chirho)?;
        let short_method_name_chirho = method_name_chirho
            .rsplit('.')
            .next()
            .unwrap_or(method_name_chirho);
        if short_method_name_chirho != "fail" {
            return None;
        }
        self.class_method_selector_for_name_chirho(method_name_chirho)?;

        let mut result_chirho = self.try_rewrite_method_var_chirho(
            *head_id_chirho,
            dict_vars_chirho,
            evidence_classes_chirho,
            local_instance_dicts_chirho,
            Some(&type_key_chirho),
        );
        if matches!(result_chirho, CoreExprChirho::VarChirho(id_chirho) if id_chirho == *head_id_chirho)
        {
            return None;
        }
        for arg_chirho in args_chirho {
            let rewritten_arg_chirho = self
                .try_rewrite_typed_method_arg_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    &type_key_chirho,
                )
                .unwrap_or_else(|| {
                    self.rewrite_method_refs_with_locals_chirho(
                        arg_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )
                });
            result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(rewritten_arg_chirho),
            };
        }
        Some(result_chirho)
    }
}
