// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Body-backed `Ord.compare` rows for concrete `Maybe` and tuple evidence.
//! Their components must already have real compare bodies; a missing component
//! leaves the structured key on the existing named failure path.

use std::collections::{BTreeSet, HashSet};

use super::DictPassCtxChirho;
use crate::dict_chirho::type_keys_chirho::{
    canonical_type_key_chirho, split_outer_type_key_chirho,
};
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreModuleChirho, InlineAnnotationChirho,
};
use haskelujah_typing_chirho::class_chirho::MAX_TUPLE_INSTANCE_ARITY_CHIRHO;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

const ORD_KEY_DEPTH_LIMIT_CHIRHO: usize = 64;

impl DictPassCtxChirho {
    /// Generate rows only for concrete structured keys the checker proved at an
    /// `Ord` occurrence. This runs after conditional list rows exist, so a
    /// component such as `[Int]` can participate without duplicating its
    /// generator. Workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn generate_evidenced_ord_bindings_chirho(
        &mut self,
        module_chirho: &CoreModuleChirho,
        module_type_keys_chirho: &std::collections::HashMap<CoreIdChirho, String>,
    ) {
        let mut keys_chirho = self
            .occurrence_evidence_chirho
            .values()
            .filter_map(|(class_chirho, key_chirho)| {
                (class_chirho == "Ord").then_some(key_chirho.clone())
            })
            .collect::<BTreeSet<_>>();
        for binding_chirho in &module_chirho.bindings_chirho {
            self.collect_ordering_primop_keys_chirho(
                &binding_chirho.rhs_chirho,
                module_type_keys_chirho,
                &mut keys_chirho,
            );
        }
        let mut visiting_chirho = HashSet::new();
        for key_chirho in keys_chirho {
            self.ensure_ord_compare_row_chirho(&key_chirho, 0, &mut visiting_chirho);
        }
    }

    fn ensure_ord_compare_row_chirho(
        &mut self,
        key_chirho: &str,
        depth_chirho: usize,
        visiting_chirho: &mut HashSet<String>,
    ) -> Option<CoreIdChirho> {
        if depth_chirho >= ORD_KEY_DEPTH_LIMIT_CHIRHO {
            return None;
        }
        let canonical_chirho = canonical_type_key_chirho(key_chirho);
        let key_chirho = canonical_chirho.as_str();
        let spelled_chirho = Self::string_spelled_out_chirho(key_chirho);
        let spellings_chirho = if spelled_chirho == key_chirho {
            vec![key_chirho.to_string()]
        } else {
            vec![key_chirho.to_string(), spelled_chirho]
        };
        for spelling_chirho in spellings_chirho {
            if let Some(id_chirho) = self.lookup_dispatch_body_name_id_chirho(&format!(
                "$prim_Ord_compare_{spelling_chirho}"
            )) {
                return Some(id_chirho);
            }
        }
        if !visiting_chirho.insert(key_chirho.to_string()) {
            return None;
        }

        let result_chirho = (|| {
            if let Some(inner_chirho) = key_chirho.strip_prefix("Maybe ") {
                let compare_chirho = self.ensure_ord_compare_row_chirho(
                    inner_chirho.trim(),
                    depth_chirho + 1,
                    visiting_chirho,
                )?;
                return Some(self.generate_maybe_compare_row_chirho(key_chirho, compare_chirho));
            }
            if key_chirho.starts_with('(') && key_chirho.ends_with(')') {
                let fields_chirho =
                    split_outer_type_key_chirho(&key_chirho[1..key_chirho.len() - 1], ',')?;
                if !(2..=MAX_TUPLE_INSTANCE_ARITY_CHIRHO).contains(&fields_chirho.len()) {
                    return None;
                }
                let compares_chirho = fields_chirho
                    .iter()
                    .map(|field_chirho| {
                        self.ensure_ord_compare_row_chirho(
                            field_chirho,
                            depth_chirho + 1,
                            visiting_chirho,
                        )
                    })
                    .collect::<Option<Vec<_>>>()?;
                return Some(self.generate_tuple_compare_row_chirho(key_chirho, &compares_chirho));
            }
            None
        })();
        visiting_chirho.remove(key_chirho);
        result_chirho
    }

    fn collect_ordering_primop_keys_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        local_type_keys_chirho: &std::collections::HashMap<CoreIdChirho, String>,
        keys_chirho: &mut BTreeSet<String>,
    ) {
        match expr_chirho {
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                self.collect_ordering_primop_keys_chirho(
                    fun_chirho,
                    local_type_keys_chirho,
                    keys_chirho,
                );
                self.collect_ordering_primop_keys_chirho(
                    arg_chirho,
                    local_type_keys_chirho,
                    keys_chirho,
                );
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                let mut nested_keys_chirho = local_type_keys_chirho.clone();
                if let Some(key_chirho) = self
                    .binder_type_key_chirho(binder_chirho)
                    .and_then(Self::concrete_local_type_key_chirho)
                {
                    nested_keys_chirho.insert(
                        binder_chirho.id_chirho,
                        canonical_type_key_chirho(&key_chirho),
                    );
                }
                self.collect_ordering_primop_keys_chirho(
                    body_chirho,
                    &nested_keys_chirho,
                    keys_chirho,
                );
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                let mut nested_keys_chirho = local_type_keys_chirho.clone();
                for (binder_chirho, _) in binds_chirho {
                    if let Some(key_chirho) = self
                        .binder_type_key_chirho(binder_chirho)
                        .and_then(Self::concrete_local_type_key_chirho)
                    {
                        nested_keys_chirho.insert(
                            binder_chirho.id_chirho,
                            canonical_type_key_chirho(&key_chirho),
                        );
                    }
                }
                for (_, rhs_chirho) in binds_chirho {
                    self.collect_ordering_primop_keys_chirho(
                        rhs_chirho,
                        &nested_keys_chirho,
                        keys_chirho,
                    );
                }
                self.collect_ordering_primop_keys_chirho(
                    body_chirho,
                    &nested_keys_chirho,
                    keys_chirho,
                );
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                self.collect_ordering_primop_keys_chirho(
                    scrutinee_chirho,
                    local_type_keys_chirho,
                    keys_chirho,
                );
                for alt_chirho in alts_chirho {
                    let mut nested_keys_chirho = local_type_keys_chirho.clone();
                    for binder_chirho in
                        std::iter::once(bind_chirho).chain(alt_chirho.binders_chirho.iter())
                    {
                        if let Some(key_chirho) = self
                            .binder_type_key_chirho(binder_chirho)
                            .and_then(Self::concrete_local_type_key_chirho)
                        {
                            nested_keys_chirho.insert(
                                binder_chirho.id_chirho,
                                canonical_type_key_chirho(&key_chirho),
                            );
                        }
                    }
                    self.collect_ordering_primop_keys_chirho(
                        &alt_chirho.rhs_chirho,
                        &nested_keys_chirho,
                        keys_chirho,
                    );
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => self
                .collect_ordering_primop_keys_chirho(
                    body_chirho,
                    local_type_keys_chirho,
                    keys_chirho,
                ),
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.collect_ordering_primop_keys_chirho(
                inner_chirho,
                local_type_keys_chirho,
                keys_chirho,
            ),
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => {
                if matches!(name_chirho.as_str(), "<#" | "<=#" | ">#" | ">=#")
                    && let Some(key_chirho) = args_chirho.iter().find_map(|arg_chirho| {
                        self.infer_type_key_for_rewrite_chirho(arg_chirho, local_type_keys_chirho)
                            .and_then(Self::concrete_local_type_key_chirho)
                    })
                {
                    keys_chirho.insert(canonical_type_key_chirho(&key_chirho));
                }
                for arg_chirho in args_chirho {
                    self.collect_ordering_primop_keys_chirho(
                        arg_chirho,
                        local_type_keys_chirho,
                        keys_chirho,
                    );
                }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    self.collect_ordering_primop_keys_chirho(
                        arg_chirho,
                        local_type_keys_chirho,
                        keys_chirho,
                    );
                }
            }
        }
    }

    fn generate_maybe_compare_row_chirho(
        &mut self,
        key_chirho: &str,
        element_compare_chirho: CoreIdChirho,
    ) -> CoreIdChirho {
        let any_ty_chirho = TyChirho::int_chirho();
        let x_chirho = self.fresh_binder_chirho("maybe_x_chirho", any_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("maybe_y_chirho", any_ty_chirho.clone());
        let x_field_chirho =
            self.fresh_binder_chirho("maybe_x_field_chirho", any_ty_chirho.clone());
        let y_field_chirho =
            self.fresh_binder_chirho("maybe_y_field_chirho", any_ty_chirho.clone());
        let ignored_chirho =
            self.fresh_binder_chirho("maybe_ignored_chirho", any_ty_chirho.clone());
        let x_case_chirho = self.fresh_binder_chirho("maybe_x_case_chirho", any_ty_chirho.clone());
        let nothing_y_case_chirho =
            self.fresh_binder_chirho("maybe_nothing_y_case_chirho", any_ty_chirho.clone());
        let just_y_case_chirho =
            self.fresh_binder_chirho("maybe_just_y_case_chirho", any_ty_chirho.clone());

        let nothing_rhs_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(Self::var_expr_chirho(&y_chirho)),
            bind_chirho: nothing_y_case_chirho,
            result_ty_chirho: any_ty_chirho.clone(),
            alts_chirho: vec![
                Self::constructor_alt_chirho("Nothing", vec![], Self::nullary_con_chirho("EQ")),
                Self::constructor_alt_chirho(
                    "Just",
                    vec![ignored_chirho],
                    Self::nullary_con_chirho("LT"),
                ),
            ],
        };
        let just_rhs_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(Self::var_expr_chirho(&y_chirho)),
            bind_chirho: just_y_case_chirho,
            result_ty_chirho: any_ty_chirho.clone(),
            alts_chirho: vec![
                Self::constructor_alt_chirho("Nothing", vec![], Self::nullary_con_chirho("GT")),
                Self::constructor_alt_chirho(
                    "Just",
                    vec![y_field_chirho.clone()],
                    Self::apply_two_chirho(
                        element_compare_chirho,
                        Self::var_expr_chirho(&x_field_chirho),
                        Self::var_expr_chirho(&y_field_chirho),
                    ),
                ),
            ],
        };
        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(Self::var_expr_chirho(&x_chirho)),
            bind_chirho: x_case_chirho,
            result_ty_chirho: any_ty_chirho.clone(),
            alts_chirho: vec![
                Self::constructor_alt_chirho("Nothing", vec![], nothing_rhs_chirho),
                Self::constructor_alt_chirho("Just", vec![x_field_chirho], just_rhs_chirho),
            ],
        };
        self.push_structured_compare_row_chirho(key_chirho, x_chirho, y_chirho, body_chirho)
    }

    fn generate_tuple_compare_row_chirho(
        &mut self,
        key_chirho: &str,
        compares_chirho: &[CoreIdChirho],
    ) -> CoreIdChirho {
        let any_ty_chirho = TyChirho::int_chirho();
        let x_chirho = self.fresh_binder_chirho("tuple_x_chirho", any_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("tuple_y_chirho", any_ty_chirho.clone());
        let x_fields_chirho = (0..compares_chirho.len())
            .map(|index_chirho| {
                self.fresh_binder_chirho(
                    &format!("tuple_x_{index_chirho}_chirho"),
                    any_ty_chirho.clone(),
                )
            })
            .collect::<Vec<_>>();
        let y_fields_chirho = (0..compares_chirho.len())
            .map(|index_chirho| {
                self.fresh_binder_chirho(
                    &format!("tuple_y_{index_chirho}_chirho"),
                    any_ty_chirho.clone(),
                )
            })
            .collect::<Vec<_>>();
        let mut comparison_chirho = Self::apply_two_chirho(
            *compares_chirho.last().expect("a tuple has fields"),
            Self::var_expr_chirho(x_fields_chirho.last().expect("a tuple has fields")),
            Self::var_expr_chirho(y_fields_chirho.last().expect("a tuple has fields")),
        );
        for index_chirho in (0..compares_chirho.len() - 1).rev() {
            let ordering_case_chirho =
                self.fresh_binder_chirho("tuple_ordering_case_chirho", any_ty_chirho.clone());
            comparison_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(Self::apply_two_chirho(
                    compares_chirho[index_chirho],
                    Self::var_expr_chirho(&x_fields_chirho[index_chirho]),
                    Self::var_expr_chirho(&y_fields_chirho[index_chirho]),
                )),
                bind_chirho: ordering_case_chirho,
                result_ty_chirho: any_ty_chirho.clone(),
                alts_chirho: vec![
                    Self::constructor_alt_chirho("LT", vec![], Self::nullary_con_chirho("LT")),
                    Self::constructor_alt_chirho("EQ", vec![], comparison_chirho),
                    Self::constructor_alt_chirho("GT", vec![], Self::nullary_con_chirho("GT")),
                ],
            };
        }
        let tuple_constructor_chirho = format!("$tuple{}", compares_chirho.len());
        let y_case_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(Self::var_expr_chirho(&y_chirho)),
            bind_chirho: self.fresh_binder_chirho("tuple_y_case_chirho", any_ty_chirho.clone()),
            result_ty_chirho: any_ty_chirho.clone(),
            alts_chirho: vec![Self::constructor_alt_chirho(
                &tuple_constructor_chirho,
                y_fields_chirho,
                comparison_chirho,
            )],
        };
        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(Self::var_expr_chirho(&x_chirho)),
            bind_chirho: self.fresh_binder_chirho("tuple_x_case_chirho", any_ty_chirho),
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![Self::constructor_alt_chirho(
                &tuple_constructor_chirho,
                x_fields_chirho,
                y_case_chirho,
            )],
        };
        self.push_structured_compare_row_chirho(key_chirho, x_chirho, y_chirho, body_chirho)
    }

    fn push_structured_compare_row_chirho(
        &mut self,
        key_chirho: &str,
        x_chirho: BinderChirho,
        y_chirho: BinderChirho,
        body_chirho: CoreExprChirho,
    ) -> CoreIdChirho {
        let row_name_chirho = format!("$prim_Ord_compare_{key_chirho}");
        let row_binder_chirho =
            self.conditional_dict_binder_chirho(&row_name_chirho, TyChirho::int_chirho());
        let row_id_chirho = row_binder_chirho.id_chirho;
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: row_binder_chirho,
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: y_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        row_id_chirho
    }

    pub(super) fn var_expr_chirho(binder_chirho: &BinderChirho) -> CoreExprChirho {
        CoreExprChirho::VarChirho(binder_chirho.id_chirho)
    }

    pub(super) fn nullary_con_chirho(name_chirho: &str) -> CoreExprChirho {
        CoreExprChirho::ConAppChirho {
            con_name_chirho: name_chirho.to_string(),
            args_chirho: vec![],
        }
    }

    pub(super) fn constructor_alt_chirho(
        name_chirho: &str,
        binders_chirho: Vec<BinderChirho>,
        rhs_chirho: CoreExprChirho,
    ) -> CoreAltChirho {
        CoreAltChirho {
            con_chirho: AltConChirho::DataConChirho(name_chirho.to_string()),
            binders_chirho,
            rhs_chirho,
        }
    }

    pub(super) fn apply_two_chirho(
        function_chirho: CoreIdChirho,
        left_chirho: CoreExprChirho,
        right_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(function_chirho)),
                arg_chirho: Box::new(left_chirho),
            }),
            arg_chirho: Box::new(right_chirho),
        }
    }
}

#[cfg(test)]
#[path = "structured_orders_tests_chirho.rs"]
mod tests_chirho;
