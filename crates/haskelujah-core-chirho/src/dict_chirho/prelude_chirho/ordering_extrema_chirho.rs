// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The compare-backed extrema behind the Ord aggregate brick (row 565). GHC's `Ord`
//! defaults are `max x y = if x <= y then y else x` and `min x y = if x <= y then x else y`,
//! where `x <= y` is `compare x y /= GT`, and `maximum` and `minimum` fold them over a list.
//! Each helper takes the compare it reads, so an evidenced occurrence at a key with a
//! body-backed compare row becomes one partial application. `maximum` and `minimum` fold
//! as the Int-shaped bindings do, `foldl pick (head xs) (tail xs)`, so an empty list fails
//! the same way on every key.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho, TyVarChirho,
};

/// Each Prelude extremum and the helper an evidenced occurrence of it becomes.
pub(in crate::dict_chirho) const ORDERING_EXTREMA_CHIRHO: [(&str, &str); 4] = [
    ("max", "$ord_max_by_chirho"),
    ("min", "$ord_min_by_chirho"),
    ("maximum", "$ord_maximum_by_chirho"),
    ("minimum", "$ord_minimum_by_chirho"),
];

fn app_chirho(fun_chirho: CoreExprChirho, arg_chirho: CoreExprChirho) -> CoreExprChirho {
    CoreExprChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
    }
}

fn var_chirho(id_chirho: CoreIdChirho) -> CoreExprChirho {
    CoreExprChirho::VarChirho(id_chirho)
}

impl DictPassCtxChirho {
    /// Generate `\cmp x y -> case cmp x y of { GT -> x; _ -> y }` for `max`, its mirror
    /// for `min`, and `\cmp xs -> foldl (pick cmp) (head xs) (tail xs)` for `maximum` and
    /// `minimum` over them.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_ordering_extrema_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(9975));
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));
        let ordering_ty_chirho = TyChirho::ConChirho("Ordering".to_string());
        let compare_ty_chirho = TyChirho::fun_n_chirho(
            vec![a_chirho.clone(), a_chirho.clone()],
            ordering_ty_chirho.clone(),
        );
        let [max_chirho, min_chirho, maximum_chirho, minimum_chirho] =
            ORDERING_EXTREMA_CHIRHO.map(|(_, helper_chirho)| helper_chirho);

        // Binary picks: GT keeps the first argument for `max` and the second for `min`.
        for (helper_chirho, greater_keeps_first_chirho) in [(max_chirho, true), (min_chirho, false)]
        {
            let id_chirho = self.resolve_or_fresh_id_chirho(helper_chirho);
            let compare_chirho = self.fresh_binder_chirho("cmp", compare_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", a_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", ordering_ty_chirho.clone());
            let (greater_chirho, otherwise_chirho) = if greater_keeps_first_chirho {
                (x_chirho.id_chirho, y_chirho.id_chirho)
            } else {
                (y_chirho.id_chirho, x_chirho.id_chirho)
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(app_chirho(
                    app_chirho(
                        var_chirho(compare_chirho.id_chirho),
                        var_chirho(x_chirho.id_chirho),
                    ),
                    var_chirho(y_chirho.id_chirho),
                )),
                bind_chirho: wild_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: var_chirho(greater_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: var_chirho(otherwise_chirho),
                    },
                ],
            };
            let rhs_chirho = [compare_chirho, x_chirho, y_chirho].into_iter().rev().fold(
                body_chirho,
                |body_chirho, binder_chirho| CoreExprChirho::LamChirho {
                    binder_chirho,
                    body_chirho: Box::new(body_chirho),
                },
            );
            self.push_extremum_binding_chirho(
                id_chirho,
                helper_chirho,
                TyChirho::fun_n_chirho(
                    vec![
                        compare_ty_chirho.clone(),
                        a_chirho.clone(),
                        a_chirho.clone(),
                    ],
                    a_chirho.clone(),
                ),
                rhs_chirho,
            );
        }

        // List folds over the picks, shaped as the Int-only `maximum` and `minimum`.
        for (helper_chirho, pick_chirho) in
            [(maximum_chirho, max_chirho), (minimum_chirho, min_chirho)]
        {
            let id_chirho = self.resolve_or_fresh_id_chirho(helper_chirho);
            let pick_id_chirho = self.resolve_or_fresh_id_chirho(pick_chirho);
            let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");
            let head_id_chirho = self.resolve_or_fresh_id_chirho("head");
            let tail_id_chirho = self.resolve_or_fresh_id_chirho("tail");
            let compare_chirho = self.fresh_binder_chirho("cmp", compare_ty_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let body_chirho = app_chirho(
                app_chirho(
                    app_chirho(
                        var_chirho(foldl_id_chirho),
                        app_chirho(
                            var_chirho(pick_id_chirho),
                            var_chirho(compare_chirho.id_chirho),
                        ),
                    ),
                    app_chirho(var_chirho(head_id_chirho), var_chirho(xs_chirho.id_chirho)),
                ),
                app_chirho(var_chirho(tail_id_chirho), var_chirho(xs_chirho.id_chirho)),
            );
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: compare_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.push_extremum_binding_chirho(
                id_chirho,
                helper_chirho,
                TyChirho::fun_n_chirho(
                    vec![compare_ty_chirho.clone(), list_a_chirho.clone()],
                    a_chirho.clone(),
                ),
                rhs_chirho,
            );
        }
    }

    fn push_extremum_binding_chirho(
        &mut self,
        id_chirho: CoreIdChirho,
        name_chirho: &str,
        ty_chirho: TyChirho,
        rhs_chirho: CoreExprChirho,
    ) {
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho,
                name_chirho: name_chirho.to_string(),
                ty_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
