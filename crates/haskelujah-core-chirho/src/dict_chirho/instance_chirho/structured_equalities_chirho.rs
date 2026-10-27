// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Body-backed `Eq.==` rows for concrete `Maybe`, list and tuple evidence, and the
//! By-helpers through which the Eq-polymorphic list functions take a key's row (row 603).
//! As in `structured_orders_chirho`, a component without a real equality body leaves its
//! key on the existing named failure path; no placeholder or missing-method stub serves.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::{BTreeSet, HashSet};

use super::DictPassCtxChirho;
use crate::dict_chirho::type_keys_chirho::{
    canonical_type_key_chirho, split_outer_type_key_chirho,
};
use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, InlineAnnotationChirho,
};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::MAX_TUPLE_INSTANCE_ARITY_CHIRHO;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

const EQ_KEY_DEPTH_LIMIT_CHIRHO: usize = 64;

/// Keys whose Int-shaped `==#` equality and ordering are already exact: their Eq and Ord
/// consumers keep today's bodies (rows 565 and 603).
pub(in crate::dict_chirho) const INT_SHAPED_KEYS_CHIRHO: [&str; 3] = ["Int", "Char", "Bool"];

/// The Eq-polymorphic list functions and the helper each takes a key's `Eq.==` row
/// through: compiler-owned By-helpers for five, and the Prelude's `nubBy` and `groupBy`.
pub(in crate::dict_chirho) const EQUALITY_CONSUMERS_CHIRHO: [(&str, &str); 7] = [
    ("elem", "$elemBy"),
    ("notElem", "$notElemBy"),
    ("lookup", "$lookupBy"),
    ("isPrefixOf", "$isPrefixOfBy"),
    ("isSuffixOf", "$isSuffixOfBy"),
    ("nub", "nubBy"),
    ("group", "groupBy"),
];

impl DictPassCtxChirho {
    /// Rows for every concrete Eq key the checker proved, and the By-helpers its
    /// routable consumers take them through. Runs after the conditional list rows, as the
    /// Ord rows do.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn generate_evidenced_eq_bindings_chirho(&mut self) {
        let evidenced_chirho = self
            .occurrence_evidence_chirho
            .iter()
            .filter(|(_, (class_chirho, _))| class_chirho == "Eq")
            .map(|(id_chirho, (_, key_chirho))| (*id_chirho, key_chirho.clone()))
            .collect::<Vec<_>>();
        let keys_chirho = evidenced_chirho
            .iter()
            .map(|(_, key_chirho)| key_chirho.clone())
            .collect::<BTreeSet<_>>();
        for key_chirho in &keys_chirho {
            self.ensure_eq_row_chirho(key_chirho);
        }
        let mut helpers_chirho = BTreeSet::new();
        for (id_chirho, key_chirho) in &evidenced_chirho {
            if INT_SHAPED_KEYS_CHIRHO.contains(&key_chirho.as_str())
                || self.existing_eq_row_chirho(key_chirho).is_none()
            {
                continue;
            }
            let Some((name_chirho, _)) = self.method_occurrence_canon_chirho.get(id_chirho) else {
                continue;
            };
            let base_chirho = name_chirho.rsplit('.').next().unwrap_or(name_chirho);
            if let Some((_, helper_chirho)) = EQUALITY_CONSUMERS_CHIRHO
                .iter()
                .find(|(consumer_chirho, _)| *consumer_chirho == base_chirho)
            {
                helpers_chirho.insert(*helper_chirho);
            }
        }
        for helper_chirho in helpers_chirho {
            self.generate_equality_helper_chirho(helper_chirho);
        }
    }

    /// The body-backed `$prim_Eq_==_K` row already present for `key_chirho`, under its
    /// canonical spelling or with `String` spelled out; never a missing-method stub.
    pub(in crate::dict_chirho) fn existing_eq_row_chirho(
        &self,
        key_chirho: &str,
    ) -> Option<CoreIdChirho> {
        self.existing_keyed_row_chirho("$prim_Eq_==_", key_chirho)
    }

    /// The body-backed `<row_prefix><key>` row already present for `key_chirho`, under
    /// its canonical spelling or with `String` spelled out; never a missing-method stub.
    pub(in crate::dict_chirho) fn existing_keyed_row_chirho(
        &self,
        row_prefix_chirho: &str,
        key_chirho: &str,
    ) -> Option<CoreIdChirho> {
        let canonical_chirho = canonical_type_key_chirho(key_chirho);
        let spelled_chirho = Self::string_spelled_out_chirho(&canonical_chirho);
        [canonical_chirho, spelled_chirho]
            .iter()
            .find_map(|spelling_chirho| {
                self.lookup_dispatch_body_name_id_chirho(&format!(
                    "{row_prefix_chirho}{spelling_chirho}"
                ))
            })
    }

    /// The body-backed `$prim_Eq_==_K` row for `key_chirho`: the existing one, or one
    /// composed for `Maybe`, a list or a tuple of up to 15 fields (row 604) from its components' rows.
    /// None when any component lacks a real body.
    pub(in crate::dict_chirho) fn ensure_eq_row_chirho(
        &mut self,
        key_chirho: &str,
    ) -> Option<CoreIdChirho> {
        self.ensure_eq_row_at_chirho(key_chirho, 0, &mut HashSet::new())
    }

    fn ensure_eq_row_at_chirho(
        &mut self,
        key_chirho: &str,
        depth_chirho: usize,
        visiting_chirho: &mut HashSet<String>,
    ) -> Option<CoreIdChirho> {
        if depth_chirho >= EQ_KEY_DEPTH_LIMIT_CHIRHO {
            return None;
        }
        if let Some(id_chirho) = self.existing_eq_row_chirho(key_chirho) {
            return Some(id_chirho);
        }
        let key_chirho = canonical_type_key_chirho(key_chirho);
        if !visiting_chirho.insert(key_chirho.clone()) {
            return None;
        }
        let mut component_chirho = |component_key_chirho: &str, ctx_chirho: &mut Self| {
            ctx_chirho.ensure_eq_row_at_chirho(
                component_key_chirho.trim(),
                depth_chirho + 1,
                visiting_chirho,
            )
        };
        let row_chirho = if let Some(inner_chirho) = key_chirho.strip_prefix("Maybe ") {
            component_chirho(inner_chirho, self).map(|inner_row_chirho| {
                self.generate_maybe_eq_row_chirho(&key_chirho, inner_row_chirho)
            })
        } else if let Some(element_chirho) = key_chirho
            .strip_prefix('[')
            .and_then(|rest_chirho| rest_chirho.strip_suffix(']'))
        {
            component_chirho(element_chirho, self).map(|element_row_chirho| {
                self.generate_list_eq_row_chirho(&key_chirho, element_row_chirho)
            })
        } else if key_chirho.starts_with('(') && key_chirho.ends_with(')') {
            split_outer_type_key_chirho(&key_chirho[1..key_chirho.len() - 1], ',')
                .filter(|fields_chirho| {
                    (2..=MAX_TUPLE_INSTANCE_ARITY_CHIRHO).contains(&fields_chirho.len())
                })
                .and_then(|fields_chirho| {
                    fields_chirho
                        .iter()
                        .map(|field_chirho| component_chirho(field_chirho, self))
                        .collect::<Option<Vec<_>>>()
                })
                .map(|rows_chirho| self.generate_tuple_eq_row_chirho(&key_chirho, &rows_chirho))
        } else {
            None
        };
        visiting_chirho.remove(&key_chirho);
        row_chirho
    }

    /// `\x y -> case x of Nothing -> (y is Nothing); Just a -> case y of Nothing -> False;
    /// Just b -> inner a b`.
    fn generate_maybe_eq_row_chirho(
        &mut self,
        key_chirho: &str,
        inner_chirho: CoreIdChirho,
    ) -> CoreIdChirho {
        let [x_chirho, y_chirho, a_chirho, b_chirho, ignored_chirho] =
            self.eq_binders_chirho(["maybe_x", "maybe_y", "maybe_a", "maybe_b", "maybe_ignored"]);
        let y_nothing_chirho = self.case_chirho(
            Self::var_expr_chirho(&y_chirho),
            vec![
                Self::constructor_alt_chirho("Nothing", vec![], Self::nullary_con_chirho("True")),
                Self::constructor_alt_chirho(
                    "Just",
                    vec![ignored_chirho],
                    Self::nullary_con_chirho("False"),
                ),
            ],
        );
        let y_just_chirho = self.case_chirho(
            Self::var_expr_chirho(&y_chirho),
            vec![
                Self::constructor_alt_chirho("Nothing", vec![], Self::nullary_con_chirho("False")),
                Self::constructor_alt_chirho(
                    "Just",
                    vec![b_chirho.clone()],
                    Self::apply_two_chirho(
                        inner_chirho,
                        Self::var_expr_chirho(&a_chirho),
                        Self::var_expr_chirho(&b_chirho),
                    ),
                ),
            ],
        );
        let body_chirho = self.case_chirho(
            Self::var_expr_chirho(&x_chirho),
            vec![
                Self::constructor_alt_chirho("Nothing", vec![], y_nothing_chirho),
                Self::constructor_alt_chirho("Just", vec![a_chirho], y_just_chirho),
            ],
        );
        let row_binder_chirho = self.eq_row_binder_chirho(key_chirho);
        self.push_two_argument_row_chirho(row_binder_chirho, x_chirho, y_chirho, body_chirho, false)
    }

    /// `\xs ys -> case xs of [] -> (ys is []); a : as -> case ys of [] -> False;
    /// b : bs -> case element a b of True -> row as bs; False -> False`.
    fn generate_list_eq_row_chirho(
        &mut self,
        key_chirho: &str,
        element_chirho: CoreIdChirho,
    ) -> CoreIdChirho {
        let row_binder_chirho = self.eq_row_binder_chirho(key_chirho);
        let row_id_chirho = row_binder_chirho.id_chirho;
        let [
            xs_chirho,
            ys_chirho,
            a_chirho,
            rest_a_chirho,
            b_chirho,
            rest_b_chirho,
            w1_chirho,
            w2_chirho,
        ] = self.eq_binders_chirho([
            "list_xs",
            "list_ys",
            "list_a",
            "list_rest_a",
            "list_b",
            "list_rest_b",
            "list_w1",
            "list_w2",
        ]);
        let both_tails_chirho = Self::apply_two_chirho(
            row_id_chirho,
            Self::var_expr_chirho(&rest_a_chirho),
            Self::var_expr_chirho(&rest_b_chirho),
        );
        let heads_chirho = self.and_then_chirho(
            Self::apply_two_chirho(
                element_chirho,
                Self::var_expr_chirho(&a_chirho),
                Self::var_expr_chirho(&b_chirho),
            ),
            both_tails_chirho,
        );
        let ys_after_cons_chirho = self.case_chirho(
            Self::var_expr_chirho(&ys_chirho),
            vec![
                Self::constructor_alt_chirho("[]", vec![], Self::nullary_con_chirho("False")),
                Self::constructor_alt_chirho(":", vec![b_chirho, rest_b_chirho], heads_chirho),
            ],
        );
        let ys_after_nil_chirho = self.case_chirho(
            Self::var_expr_chirho(&ys_chirho),
            vec![
                Self::constructor_alt_chirho("[]", vec![], Self::nullary_con_chirho("True")),
                Self::constructor_alt_chirho(
                    ":",
                    vec![w1_chirho, w2_chirho],
                    Self::nullary_con_chirho("False"),
                ),
            ],
        );
        let body_chirho = self.case_chirho(
            Self::var_expr_chirho(&xs_chirho),
            vec![
                Self::constructor_alt_chirho("[]", vec![], ys_after_nil_chirho),
                Self::constructor_alt_chirho(
                    ":",
                    vec![a_chirho, rest_a_chirho],
                    ys_after_cons_chirho,
                ),
            ],
        );
        self.push_two_argument_row_chirho(
            row_binder_chirho,
            xs_chirho,
            ys_chirho,
            body_chirho,
            true,
        )
    }

    /// `\x y -> case x of (a1, …) -> case y of (b1, …) -> eq1 a1 b1 && …`.
    fn generate_tuple_eq_row_chirho(
        &mut self,
        key_chirho: &str,
        fields_chirho: &[CoreIdChirho],
    ) -> CoreIdChirho {
        let [x_chirho, y_chirho] = self.eq_binders_chirho(["tuple_x", "tuple_y"]);
        let any_ty_chirho = TyChirho::int_chirho();
        let x_fields_chirho = (0..fields_chirho.len())
            .map(|index_chirho| {
                self.fresh_binder_chirho(
                    &format!("tuple_eq_x_{index_chirho}_chirho"),
                    any_ty_chirho.clone(),
                )
            })
            .collect::<Vec<_>>();
        let y_fields_chirho = (0..fields_chirho.len())
            .map(|index_chirho| {
                self.fresh_binder_chirho(
                    &format!("tuple_eq_y_{index_chirho}_chirho"),
                    any_ty_chirho.clone(),
                )
            })
            .collect::<Vec<_>>();
        let field_test_chirho = |index_chirho: usize| {
            Self::apply_two_chirho(
                fields_chirho[index_chirho],
                Self::var_expr_chirho(&x_fields_chirho[index_chirho]),
                Self::var_expr_chirho(&y_fields_chirho[index_chirho]),
            )
        };
        let mut all_chirho = field_test_chirho(fields_chirho.len() - 1);
        for index_chirho in (0..fields_chirho.len() - 1).rev() {
            all_chirho = self.and_then_chirho(field_test_chirho(index_chirho), all_chirho);
        }
        let tuple_constructor_chirho = format!("$tuple{}", fields_chirho.len());
        let y_case_chirho = self.case_chirho(
            Self::var_expr_chirho(&y_chirho),
            vec![Self::constructor_alt_chirho(
                &tuple_constructor_chirho,
                y_fields_chirho,
                all_chirho,
            )],
        );
        let body_chirho = self.case_chirho(
            Self::var_expr_chirho(&x_chirho),
            vec![Self::constructor_alt_chirho(
                &tuple_constructor_chirho,
                x_fields_chirho,
                y_case_chirho,
            )],
        );
        let row_binder_chirho = self.eq_row_binder_chirho(key_chirho);
        self.push_two_argument_row_chirho(row_binder_chirho, x_chirho, y_chirho, body_chirho, false)
    }

    fn eq_row_binder_chirho(&mut self, key_chirho: &str) -> BinderChirho {
        self.conditional_dict_binder_chirho(
            &format!("$prim_Eq_==_{key_chirho}"),
            TyChirho::int_chirho(),
        )
    }

    fn push_two_argument_row_chirho(
        &mut self,
        row_binder_chirho: BinderChirho,
        x_chirho: BinderChirho,
        y_chirho: BinderChirho,
        body_chirho: CoreExprChirho,
        is_rec_chirho: bool,
    ) -> CoreIdChirho {
        let row_id_chirho = row_binder_chirho.id_chirho;
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: row_binder_chirho,
            rhs_chirho: Self::lambdas_chirho(vec![x_chirho, y_chirho], body_chirho),
            is_rec_chirho,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        row_id_chirho
    }

    /// A compiler-owned By-helper's body, generated once. Each calls its equality on the
    /// list's element first and the sought value second, as GHC's `elem_by` does.
    fn generate_equality_helper_chirho(&mut self, helper_chirho: &str) {
        if !helper_chirho.starts_with('$')
            || self
                .lookup_compiler_owned_name_id_chirho(helper_chirho)
                .is_some()
        {
            return;
        }
        let helper_id_chirho = self.resolve_or_fresh_id_chirho(helper_chirho);
        let is_rec_chirho = !matches!(helper_chirho, "$notElemBy" | "$isSuffixOfBy");
        let (parameters_chirho, body_chirho) = match helper_chirho {
            "$elemBy" => self.elem_by_body_chirho(helper_id_chirho),
            "$notElemBy" => {
                self.generate_equality_helper_chirho("$elemBy");
                let elem_by_chirho = self.lookup_compiler_owned_name_id_chirho("$elemBy");
                let Some(elem_by_chirho) = elem_by_chirho else {
                    return;
                };
                let [eq_chirho, sought_chirho, xs_chirho] =
                    self.eq_binders_chirho(["by_eq", "by_sought", "by_xs"]);
                let found_chirho = Self::apply_all_chirho(
                    elem_by_chirho,
                    vec![
                        Self::var_expr_chirho(&eq_chirho),
                        Self::var_expr_chirho(&sought_chirho),
                        Self::var_expr_chirho(&xs_chirho),
                    ],
                );
                let negated_chirho = self.case_chirho(
                    found_chirho,
                    vec![
                        Self::constructor_alt_chirho(
                            "True",
                            vec![],
                            Self::nullary_con_chirho("False"),
                        ),
                        Self::constructor_alt_chirho(
                            "False",
                            vec![],
                            Self::nullary_con_chirho("True"),
                        ),
                    ],
                );
                (vec![eq_chirho, sought_chirho, xs_chirho], negated_chirho)
            }
            "$lookupBy" => self.lookup_by_body_chirho(helper_id_chirho),
            "$isPrefixOfBy" => self.is_prefix_of_by_body_chirho(helper_id_chirho),
            "$isSuffixOfBy" => {
                self.generate_equality_helper_chirho("$isPrefixOfBy");
                let Some(prefix_by_chirho) =
                    self.lookup_compiler_owned_name_id_chirho("$isPrefixOfBy")
                else {
                    return;
                };
                let reverse_chirho = self.resolve_or_fresh_id_chirho("reverse");
                let [eq_chirho, xs_chirho, ys_chirho] =
                    self.eq_binders_chirho(["by_eq", "by_xs", "by_ys"]);
                let reversed_chirho = |list_chirho: &BinderChirho| CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(reverse_chirho)),
                    arg_chirho: Box::new(Self::var_expr_chirho(list_chirho)),
                };
                let body_chirho = Self::apply_all_chirho(
                    prefix_by_chirho,
                    vec![
                        Self::var_expr_chirho(&eq_chirho),
                        reversed_chirho(&xs_chirho),
                        reversed_chirho(&ys_chirho),
                    ],
                );
                (vec![eq_chirho, xs_chirho, ys_chirho], body_chirho)
            }
            _ => return,
        };
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: helper_id_chirho,
                name_chirho: helper_chirho.to_string(),
                ty_chirho: TyChirho::int_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: Self::lambdas_chirho(parameters_chirho, body_chirho),
            is_rec_chirho,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }

    /// `$elemBy eq e xs`: `case xs of [] -> False; x : rest -> eq x e || $elemBy eq e rest`.
    fn elem_by_body_chirho(
        &mut self,
        self_id_chirho: CoreIdChirho,
    ) -> (Vec<BinderChirho>, CoreExprChirho) {
        let [eq_chirho, sought_chirho, xs_chirho, x_chirho, rest_chirho] =
            self.eq_binders_chirho(["by_eq", "by_sought", "by_xs", "by_x", "by_rest"]);
        let recurse_chirho = Self::apply_all_chirho(
            self_id_chirho,
            vec![
                Self::var_expr_chirho(&eq_chirho),
                Self::var_expr_chirho(&sought_chirho),
                Self::var_expr_chirho(&rest_chirho),
            ],
        );
        let test_chirho = Self::apply_all_by_chirho(&eq_chirho, &x_chirho, &sought_chirho);
        let found_chirho = self.case_chirho(
            test_chirho,
            vec![
                Self::constructor_alt_chirho("True", vec![], Self::nullary_con_chirho("True")),
                Self::constructor_alt_chirho("False", vec![], recurse_chirho),
            ],
        );
        let body_chirho = self.case_chirho(
            Self::var_expr_chirho(&xs_chirho),
            vec![
                Self::constructor_alt_chirho("[]", vec![], Self::nullary_con_chirho("False")),
                Self::constructor_alt_chirho(":", vec![x_chirho, rest_chirho], found_chirho),
            ],
        );
        (vec![eq_chirho, sought_chirho, xs_chirho], body_chirho)
    }

    /// `$lookupBy eq k ps`: the first pair whose key `eq k key` holds, as GHC's `lookup`.
    fn lookup_by_body_chirho(
        &mut self,
        self_id_chirho: CoreIdChirho,
    ) -> (Vec<BinderChirho>, CoreExprChirho) {
        let [
            eq_chirho,
            sought_chirho,
            pairs_chirho,
            pair_chirho,
            rest_chirho,
            key_chirho,
            value_chirho,
        ] = self.eq_binders_chirho([
            "by_eq",
            "by_sought",
            "by_pairs",
            "by_pair",
            "by_rest",
            "by_key",
            "by_value",
        ]);
        let recurse_chirho = Self::apply_all_chirho(
            self_id_chirho,
            vec![
                Self::var_expr_chirho(&eq_chirho),
                Self::var_expr_chirho(&sought_chirho),
                Self::var_expr_chirho(&rest_chirho),
            ],
        );
        let found_chirho = self.case_chirho(
            Self::apply_all_by_chirho(&eq_chirho, &sought_chirho, &key_chirho),
            vec![
                Self::constructor_alt_chirho(
                    "True",
                    vec![],
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: "Just".to_string(),
                        args_chirho: vec![Self::var_expr_chirho(&value_chirho)],
                    },
                ),
                Self::constructor_alt_chirho("False", vec![], recurse_chirho),
            ],
        );
        let pair_case_chirho = self.case_chirho(
            Self::var_expr_chirho(&pair_chirho),
            vec![Self::constructor_alt_chirho(
                "$tuple2",
                vec![key_chirho, value_chirho],
                found_chirho,
            )],
        );
        let body_chirho = self.case_chirho(
            Self::var_expr_chirho(&pairs_chirho),
            vec![
                Self::constructor_alt_chirho("[]", vec![], Self::nullary_con_chirho("Nothing")),
                Self::constructor_alt_chirho(":", vec![pair_chirho, rest_chirho], pair_case_chirho),
            ],
        );
        (vec![eq_chirho, sought_chirho, pairs_chirho], body_chirho)
    }

    /// `$isPrefixOfBy eq xs ys`: `[]` is a prefix of anything; else heads equal by `eq`
    /// and the tails a prefix.
    fn is_prefix_of_by_body_chirho(
        &mut self,
        self_id_chirho: CoreIdChirho,
    ) -> (Vec<BinderChirho>, CoreExprChirho) {
        let [
            eq_chirho,
            xs_chirho,
            ys_chirho,
            x_chirho,
            rest_x_chirho,
            y_chirho,
            rest_y_chirho,
        ] = self.eq_binders_chirho([
            "by_eq",
            "by_xs",
            "by_ys",
            "by_x",
            "by_rest_x",
            "by_y",
            "by_rest_y",
        ]);
        let recurse_chirho = Self::apply_all_chirho(
            self_id_chirho,
            vec![
                Self::var_expr_chirho(&eq_chirho),
                Self::var_expr_chirho(&rest_x_chirho),
                Self::var_expr_chirho(&rest_y_chirho),
            ],
        );
        let heads_chirho = self.and_then_chirho(
            Self::apply_all_by_chirho(&eq_chirho, &x_chirho, &y_chirho),
            recurse_chirho,
        );
        let ys_case_chirho = self.case_chirho(
            Self::var_expr_chirho(&ys_chirho),
            vec![
                Self::constructor_alt_chirho("[]", vec![], Self::nullary_con_chirho("False")),
                Self::constructor_alt_chirho(":", vec![y_chirho, rest_y_chirho], heads_chirho),
            ],
        );
        let body_chirho = self.case_chirho(
            Self::var_expr_chirho(&xs_chirho),
            vec![
                Self::constructor_alt_chirho("[]", vec![], Self::nullary_con_chirho("True")),
                Self::constructor_alt_chirho(":", vec![x_chirho, rest_x_chirho], ys_case_chirho),
            ],
        );
        (vec![eq_chirho, xs_chirho, ys_chirho], body_chirho)
    }

    /// `case test of True -> rest; False -> False`.
    fn and_then_chirho(
        &mut self,
        test_chirho: CoreExprChirho,
        rest_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        self.case_chirho(
            test_chirho,
            vec![
                Self::constructor_alt_chirho("True", vec![], rest_chirho),
                Self::constructor_alt_chirho("False", vec![], Self::nullary_con_chirho("False")),
            ],
        )
    }

    fn case_chirho(
        &mut self,
        scrutinee_chirho: CoreExprChirho,
        alts_chirho: Vec<crate::expr_chirho::CoreAltChirho>,
    ) -> CoreExprChirho {
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(scrutinee_chirho),
            bind_chirho: self.fresh_binder_chirho("eq_case_chirho", TyChirho::int_chirho()),
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho,
        }
    }

    fn eq_binders_chirho<const N: usize>(&mut self, names_chirho: [&str; N]) -> [BinderChirho; N] {
        names_chirho.map(|name_chirho| {
            self.fresh_binder_chirho(&format!("{name_chirho}_chirho"), TyChirho::int_chirho())
        })
    }

    fn lambdas_chirho(
        parameters_chirho: Vec<BinderChirho>,
        body_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        parameters_chirho
            .into_iter()
            .rev()
            .fold(body_chirho, |inner_chirho, parameter_chirho| {
                CoreExprChirho::LamChirho {
                    binder_chirho: parameter_chirho,
                    body_chirho: Box::new(inner_chirho),
                }
            })
    }

    fn apply_all_chirho(
        function_chirho: CoreIdChirho,
        arguments_chirho: Vec<CoreExprChirho>,
    ) -> CoreExprChirho {
        arguments_chirho.into_iter().fold(
            CoreExprChirho::VarChirho(function_chirho),
            |applied_chirho, argument_chirho| CoreExprChirho::AppChirho {
                fun_chirho: Box::new(applied_chirho),
                arg_chirho: Box::new(argument_chirho),
            },
        )
    }

    /// `eq left right` for an equality held in a binder.
    fn apply_all_by_chirho(
        eq_chirho: &BinderChirho,
        left_chirho: &BinderChirho,
        right_chirho: &BinderChirho,
    ) -> CoreExprChirho {
        Self::apply_all_chirho(
            eq_chirho.id_chirho,
            vec![
                Self::var_expr_chirho(left_chirho),
                Self::var_expr_chirho(right_chirho),
            ],
        )
    }
}

#[cfg(test)]
#[path = "structured_equalities_tests_chirho.rs"]
mod tests_chirho;
