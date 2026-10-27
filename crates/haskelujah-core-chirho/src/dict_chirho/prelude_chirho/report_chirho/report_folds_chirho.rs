// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Haskell 2010 report's `scanl1`, `foldl1`, `foldr1` and `until`. The checker typed them,
//! but no Core binding existed, so the interpreter and both native backends failed on a
//! missing binding (row 548; contract-543 rows U01, U02, U03 and U05; `foldr1` in row 558).
//! `scanl1` and `foldl1` are defined over the list `scanl` and `foldl` the other generators
//! already provide, `foldr1` and `until` recurse on themselves, and an empty `foldl1` or
//! `foldr1` fails with GHC's message. `realToFrac` needs the Real and Fractional dictionaries and stays a
//! named missing binding.
//! workflow: testing-chirho/native-fail-loud-chirho

use super::{
    AltConChirho, CoreAltChirho, CoreExprChirho, CoreLitChirho, DictPassCtxChirho, TyChirho,
    TyVarChirho, alt_chirho, apply_chirho, lambdas_chirho, var_chirho,
};

/// GHC 9.14.1's first stderr line for `foldl1` on an empty list.
const EMPTY_FOLDL1_CHIRHO: &str = "Prelude.foldl1: empty list";

/// GHC 9.14.1's first stderr line for `foldr1` on an empty list.
const EMPTY_FOLDR1_CHIRHO: &str = "Prelude.foldr1: empty list";

/// The default alternative an empty list takes: `error` with GHC's message.
fn empty_list_failure_alt_chirho(message_chirho: &str) -> CoreAltChirho {
    CoreAltChirho {
        con_chirho: AltConChirho::DefaultChirho,
        binders_chirho: vec![],
        rhs_chirho: CoreExprChirho::PrimOpChirho {
            name_chirho: "error".to_string(),
            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                message_chirho.to_string(),
            ))],
        },
    }
}

impl DictPassCtxChirho {
    /// Generate the report's `scanl1`, `foldl1`, `foldr1` and `until`.
    /// workflow: testing-chirho/native-fail-loud-chirho
    pub(in crate::dict_chirho::prelude_chirho) fn generate_report_folds_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(9970));
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));
        let step_ty_chirho =
            TyChirho::fun_n_chirho(vec![a_chirho.clone(), a_chirho.clone()], a_chirho.clone());

        // ── scanl1 :: (a -> a -> a) -> [a] -> [a] ──
        // scanl1 _ [] = []; scanl1 f (x:xs) = scanl f x xs
        {
            let scanl1_id_chirho = self.resolve_or_fresh_id_chirho("scanl1");
            let scanl_id_chirho = self.resolve_or_fresh_id_chirho("scanl");
            let f_chirho = self.fresh_binder_chirho("f", step_ty_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let scan_chirho = apply_chirho(
                CoreExprChirho::VarChirho(scanl_id_chirho),
                vec![
                    var_chirho(&f_chirho),
                    var_chirho(&h_chirho),
                    var_chirho(&t_chirho),
                ],
            );
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(var_chirho(&xs_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    alt_chirho(
                        "[]",
                        vec![],
                        CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    ),
                    alt_chirho(":", vec![h_chirho, t_chirho], scan_chirho),
                ],
            };
            self.push_report_binding_chirho(
                scanl1_id_chirho,
                "scanl1",
                TyChirho::fun_n_chirho(
                    vec![step_ty_chirho.clone(), list_a_chirho.clone()],
                    list_a_chirho.clone(),
                ),
                lambdas_chirho(vec![f_chirho, xs_chirho], body_chirho),
                false,
            );
        }

        // ── foldl1 :: (a -> a -> a) -> [a] -> a ──
        // foldl1 f (x:xs) = foldl f x xs; foldl1 _ [] = error "Prelude.foldl1: empty list"
        {
            let foldl1_id_chirho = self.resolve_or_fresh_id_chirho("foldl1");
            let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");
            let f_chirho = self.fresh_binder_chirho("f", step_ty_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let fold_chirho = apply_chirho(
                CoreExprChirho::VarChirho(foldl_id_chirho),
                vec![
                    var_chirho(&f_chirho),
                    var_chirho(&h_chirho),
                    var_chirho(&t_chirho),
                ],
            );
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(var_chirho(&xs_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    alt_chirho(":", vec![h_chirho, t_chirho], fold_chirho),
                    empty_list_failure_alt_chirho(EMPTY_FOLDL1_CHIRHO),
                ],
            };
            self.push_report_binding_chirho(
                foldl1_id_chirho,
                "foldl1",
                TyChirho::fun_n_chirho(
                    vec![step_ty_chirho.clone(), list_a_chirho.clone()],
                    a_chirho.clone(),
                ),
                lambdas_chirho(vec![f_chirho, xs_chirho], body_chirho),
                false,
            );
        }

        // ── foldr1 :: (a -> a -> a) -> [a] -> a ──
        // foldr1 f (x:xs) = case xs of [] -> x; _ -> f x (foldr1 f xs)
        // foldr1 _ [] = error "Prelude.foldr1: empty list"
        // The recursive call is an argument of `f`, so an `f` that ignores it never runs it.
        {
            let foldr1_id_chirho = self.resolve_or_fresh_id_chirho("foldr1");
            let f_chirho = self.fresh_binder_chirho("f", step_ty_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let wt_chirho = self.fresh_binder_chirho("$wt", list_a_chirho.clone());
            let rest_chirho = apply_chirho(
                CoreExprChirho::VarChirho(foldr1_id_chirho),
                vec![var_chirho(&f_chirho), var_chirho(&t_chirho)],
            );
            let step_chirho = apply_chirho(
                var_chirho(&f_chirho),
                vec![var_chirho(&h_chirho), rest_chirho],
            );
            let nonempty_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(var_chirho(&t_chirho)),
                bind_chirho: wt_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    alt_chirho("[]", vec![], var_chirho(&h_chirho)),
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: step_chirho,
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(var_chirho(&xs_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    alt_chirho(":", vec![h_chirho, t_chirho], nonempty_chirho),
                    empty_list_failure_alt_chirho(EMPTY_FOLDR1_CHIRHO),
                ],
            };
            self.push_report_binding_chirho(
                foldr1_id_chirho,
                "foldr1",
                TyChirho::fun_n_chirho(
                    vec![step_ty_chirho.clone(), list_a_chirho.clone()],
                    a_chirho.clone(),
                ),
                lambdas_chirho(vec![f_chirho, xs_chirho], body_chirho),
                true,
            );
        }

        // ── until :: (a -> Bool) -> (a -> a) -> a -> a ──
        // until p f x = if p x then x else until p f (f x)
        {
            let until_id_chirho = self.resolve_or_fresh_id_chirho("until");
            let pred_ty_chirho = TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho());
            let next_ty_chirho = TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone());
            let p_chirho = self.fresh_binder_chirho("p", pred_ty_chirho.clone());
            let f_chirho = self.fresh_binder_chirho("f", next_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());
            let again_chirho = apply_chirho(
                CoreExprChirho::VarChirho(until_id_chirho),
                vec![
                    var_chirho(&p_chirho),
                    var_chirho(&f_chirho),
                    apply_chirho(var_chirho(&f_chirho), vec![var_chirho(&x_chirho)]),
                ],
            );
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(apply_chirho(
                    var_chirho(&p_chirho),
                    vec![var_chirho(&x_chirho)],
                )),
                bind_chirho: w_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    alt_chirho("True", vec![], var_chirho(&x_chirho)),
                    alt_chirho("False", vec![], again_chirho),
                ],
            };
            self.push_report_binding_chirho(
                until_id_chirho,
                "until",
                TyChirho::fun_n_chirho(
                    vec![pred_ty_chirho, next_ty_chirho, a_chirho.clone()],
                    a_chirho,
                ),
                lambdas_chirho(vec![p_chirho, f_chirho, x_chirho], body_chirho),
                true,
            );
        }
    }
}
