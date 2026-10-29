// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! InstanceSigs: an instance method's own signature in the instance body. A
//! signature that is a PURE RENAMING of the method's type at the instance
//! scopes its explicitly quantified binders over the body as the method's own
//! quantifiers, and the method is checked rigidly. Any other signature is a
//! declared support boundary, and the method takes main's route
//! (`instance_obligations_chirho`). Checking a body against a signature that
//! may be more general than the class requires is full InstanceSigs support,
//! its own unit (gpt_chirho #27205).
//! workflow: language-features-chirho/instance-obligations-chirho

use super::*;

/// A structural correspondence between an instance signature and the method's
/// type at the instance (`scope_renaming_signature_chirho`): each variable the
/// signature quantifies stands for exactly one of the method's own quantifiers,
/// and every other variable is the same on both sides. Nothing is unified, so
/// no rigid variable can be solved by it.
#[derive(Clone)]
struct RenamingChirho<'a> {
    quantified_chirho: &'a HashSet<TyVarChirho>,
    own_chirho: &'a HashSet<TyVarChirho>,
    forward_chirho: HashMap<TyVarChirho, TyVarChirho>,
    backward_chirho: HashMap<TyVarChirho, TyVarChirho>,
}

impl RenamingChirho<'_> {
    fn corresponds_chirho(
        &mut self,
        signature_chirho: &TyChirho,
        method_chirho: &TyChirho,
    ) -> bool {
        match (signature_chirho, method_chirho) {
            (TyChirho::VarChirho(from_chirho), TyChirho::VarChirho(to_chirho)) => {
                self.pair_chirho(*from_chirho, *to_chirho)
            }
            (TyChirho::ConChirho(left_chirho), TyChirho::ConChirho(right_chirho))
            | (TyChirho::ForallVarChirho(left_chirho), TyChirho::ForallVarChirho(right_chirho)) => {
                left_chirho == right_chirho
            }
            (
                TyChirho::AppChirho(left_fun_chirho, left_arg_chirho),
                TyChirho::AppChirho(right_fun_chirho, right_arg_chirho),
            ) => {
                self.corresponds_chirho(left_fun_chirho, right_fun_chirho)
                    && self.corresponds_chirho(left_arg_chirho, right_arg_chirho)
            }
            (
                TyChirho::FunChirho(left_arg_chirho, left_result_chirho, left_mult_chirho),
                TyChirho::FunChirho(right_arg_chirho, right_result_chirho, right_mult_chirho),
            ) => {
                left_mult_chirho == right_mult_chirho
                    && self.corresponds_chirho(left_arg_chirho, right_arg_chirho)
                    && self.corresponds_chirho(left_result_chirho, right_result_chirho)
            }
            (TyChirho::ListChirho(left_chirho), TyChirho::ListChirho(right_chirho)) => {
                self.corresponds_chirho(left_chirho, right_chirho)
            }
            (TyChirho::TupleChirho(left_chirho), TyChirho::TupleChirho(right_chirho)) => {
                self.corresponds_all_chirho(left_chirho, right_chirho)
            }
            // A nested polytype is outside the supported correspondence.
            _ => false,
        }
    }

    fn corresponds_all_chirho(
        &mut self,
        signature_chirho: &[TyChirho],
        method_chirho: &[TyChirho],
    ) -> bool {
        signature_chirho.len() == method_chirho.len()
            && signature_chirho
                .iter()
                .zip(method_chirho)
                .all(|(left_chirho, right_chirho)| {
                    self.corresponds_chirho(left_chirho, right_chirho)
                })
    }

    /// A quantified variable pairs with one own quantifier, consistently in both
    /// directions; any other variable only with itself, never with one of the
    /// method's own quantifiers (that would capture it).
    fn pair_chirho(&mut self, from_chirho: TyVarChirho, to_chirho: TyVarChirho) -> bool {
        if !self.quantified_chirho.contains(&from_chirho) {
            return from_chirho == to_chirho && !self.own_chirho.contains(&to_chirho);
        }
        if !self.own_chirho.contains(&to_chirho) {
            return false;
        }
        match (
            self.forward_chirho.get(&from_chirho),
            self.backward_chirho.get(&to_chirho),
        ) {
            (None, None) => {
                self.forward_chirho.insert(from_chirho, to_chirho);
                self.backward_chirho.insert(to_chirho, from_chirho);
                true
            }
            (Some(seen_to_chirho), Some(seen_from_chirho)) => {
                *seen_to_chirho == to_chirho && *seen_from_chirho == from_chirho
            }
            _ => false,
        }
    }

    /// Every quantified variable and every own quantifier is paired.
    fn is_bijective_chirho(&self) -> bool {
        self.forward_chirho.len() == self.quantified_chirho.len()
            && self.backward_chirho.len() == self.own_chirho.len()
    }
}

impl InferCtxChirho {
    /// InstanceSigs: a method's signature in the instance body that is a PURE
    /// RENAMING of the method's type at the instance scopes its explicitly
    /// quantified binders over the body as the method's own quantifiers
    /// (ScopedTypeVariables), and returns true. A pure renaming has the same
    /// structure and the same constraints after normalization; each variable it
    /// quantifies stands for exactly one of the method's own, and every other
    /// variable is the instance's, unchanged. The correspondence is structural,
    /// never a unification that could solve a rigid quantifier. Any other
    /// signature returns false and the method takes main's route: checking the
    /// body against a signature that may be more general is full InstanceSigs
    /// support, its own unit (gpt_chirho #27205).
    pub(super) fn scope_renaming_signature_chirho(
        &mut self,
        class_name_chirho: &str,
        head_tys_chirho: &[TyChirho],
        signature_chirho: &TypeChirho,
        method_preds_chirho: &[SchemePredChirho],
        method_ty_chirho: &TyChirho,
        method_vars_chirho: &[TyVarChirho],
    ) -> bool {
        let (signature_scheme_chirho, signature_var_map_chirho) =
            self.ast_type_to_scheme_with_var_map_chirho(signature_chirho);
        let quantified_chirho: HashSet<TyVarChirho> = signature_scheme_chirho
            .vars_chirho
            .iter()
            .copied()
            .collect();
        let own_chirho: HashSet<TyVarChirho> = method_vars_chirho.iter().copied().collect();
        let mut renaming_chirho = RenamingChirho {
            quantified_chirho: &quantified_chirho,
            own_chirho: &own_chirho,
            forward_chirho: HashMap::new(),
            backward_chirho: HashMap::new(),
        };
        if !renaming_chirho.corresponds_chirho(
            &self.normalize_ty_chirho(&signature_scheme_chirho.ty_chirho),
            &self.normalize_ty_chirho(method_ty_chirho),
        ) {
            return false;
        }
        // The same constraints, as a multiset. The class's own predicate at the
        // instance is the instance itself, never written in its signature.
        let instance_args_chirho: Vec<TyChirho> = head_tys_chirho
            .iter()
            .map(|ty_chirho| self.normalize_ty_chirho(ty_chirho))
            .collect();
        let mut method_constraints_chirho: Vec<(String, Vec<TyChirho>)> = method_preds_chirho
            .iter()
            .map(|pred_chirho| {
                (
                    pred_chirho.class_name_chirho.clone(),
                    self.scheme_pred_args_chirho(pred_chirho),
                )
            })
            .filter(|(class_chirho, args_chirho)| {
                class_chirho.as_str() != class_name_chirho || *args_chirho != instance_args_chirho
            })
            .collect();
        for pred_chirho in &signature_scheme_chirho.preds_chirho {
            let args_chirho = self.scheme_pred_args_chirho(pred_chirho);
            let Some(position_chirho) =
                method_constraints_chirho
                    .iter()
                    .position(|(class_chirho, method_args_chirho)| {
                        *class_chirho == pred_chirho.class_name_chirho
                            && renaming_chirho
                                .clone()
                                .corresponds_all_chirho(&args_chirho, method_args_chirho)
                    })
            else {
                return false;
            };
            let (_class_chirho, method_args_chirho) =
                method_constraints_chirho.swap_remove(position_chirho);
            renaming_chirho.corresponds_all_chirho(&args_chirho, &method_args_chirho);
        }
        if !method_constraints_chirho.is_empty() || !renaming_chirho.is_bijective_chirho() {
            return false;
        }
        // The explicitly quantified binders scope over the body, as a
        // signature's do, but as the method's own variables.
        let renamed_chirho: HashMap<String, TyVarChirho> = signature_var_map_chirho
            .into_iter()
            .filter_map(|(name_chirho, var_chirho)| {
                renaming_chirho
                    .forward_chirho
                    .get(&var_chirho)
                    .map(|own_var_chirho| (name_chirho, *own_var_chirho))
            })
            .collect();
        self.scope_signature_for_body_chirho(signature_chirho, &renamed_chirho);
        true
    }

    /// A scheme predicate's arguments, normalized, in order.
    fn scheme_pred_args_chirho(&self, pred_chirho: &SchemePredChirho) -> Vec<TyChirho> {
        std::iter::once(&pred_chirho.ty_chirho)
            .chain(&pred_chirho.extra_tys_chirho)
            .map(|ty_chirho| self.normalize_ty_chirho(ty_chirho))
            .collect()
    }
}
