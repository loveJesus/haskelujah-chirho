// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `Ord [T]` from `Ord a => Ord [a]`: one recursive list comparison that every
//! element type shares, given that type's own compare row.

use super::{DictLayoutChirho, DictPassCtxChirho};
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    InlineAnnotationChirho,
};
use haskelujah_typing_chirho::ty_chirho::TyChirho;

/// The list comparison every generated `Ord [T]` row applies to its element's row.
const COMPARE_LIST_BY_CHIRHO: &str = "$compareListBy";

impl DictPassCtxChirho {
    /// `Ord [T]` from the element's compare row:
    /// `$prim_Ord_compare_[T] = $compareListBy $prim_Ord_compare_T` and
    /// `$fOrd[T] = $Dict_Ord $fEq[T] $prim_Ord_compare_[T]`.
    ///
    /// Nothing is generated, and `false` comes back, when the element's row is
    /// missing or is itself a missing-method stub, or when the class holds a
    /// method besides `compare`: the generic path then names what is missing.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_ord_list_dict_chirho(
        &mut self,
        elem_type_chirho: &str,
        list_type_key_chirho: &str,
        layout_chirho: &DictLayoutChirho,
    ) -> bool {
        if !matches!(
            layout_chirho.method_slots_chirho.as_slice(),
            [(method_name_chirho, _)] if method_name_chirho == "compare"
        ) {
            return false;
        }
        let Some(elem_compare_id_chirho) = self
            .lookup_dispatch_body_name_id_chirho(&format!("$prim_Ord_compare_{elem_type_chirho}"))
        else {
            return false;
        };
        let compare_list_by_id_chirho = self.compare_list_by_id_chirho();

        // Rows and their binders carry placeholder types, as the builtin rows do.
        let row_binder_chirho = self.conditional_dict_binder_chirho(
            &format!("$prim_Ord_compare_{list_type_key_chirho}"),
            TyChirho::int_chirho(),
        );
        let row_id_chirho = row_binder_chirho.id_chirho;
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: row_binder_chirho,
            rhs_chirho: CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(compare_list_by_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(elem_compare_id_chirho)),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });

        let dict_binder_chirho = self.conditional_dict_binder_chirho(
            &format!("$fOrd{list_type_key_chirho}"),
            TyChirho::ConChirho("$Dict_Ord".to_string()),
        );
        let dict_id_chirho = dict_binder_chirho.id_chirho;
        let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();
        for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
            let super_dict_name_chirho = format!("$f{super_name_chirho}{list_type_key_chirho}");
            let super_dict_id_chirho = self.resolve_or_fresh_id_chirho(&super_dict_name_chirho);
            field_args_chirho.push(CoreExprChirho::VarChirho(super_dict_id_chirho));
        }
        field_args_chirho.push(CoreExprChirho::VarChirho(row_id_chirho));
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: dict_binder_chirho,
            rhs_chirho: CoreExprChirho::ConAppChirho {
                con_name_chirho: "$Dict_Ord".to_string(),
                args_chirho: field_args_chirho,
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });

        self.instance_dicts_chirho.insert(
            ("Ord".to_string(), list_type_key_chirho.to_string()),
            dict_id_chirho,
        );
        true
    }

    /// `$compareListBy`, built on first use:
    /// ```text
    /// $compareListBy = \cmp xs ys -> case xs of
    ///   [] -> case ys of { [] -> EQ; (:) _ _ -> LT }
    ///   (:) x xs' -> case ys of
    ///     [] -> GT
    ///     (:) y ys' -> case cmp x y of
    ///       LT -> LT
    ///       EQ -> $compareListBy cmp xs' ys'
    ///       GT -> GT
    /// ```
    /// No alternative is a default, so an element comparison that returns
    /// anything but an `Ordering` fails instead of choosing a branch. It forces
    /// only what the answer needs: `compare (1 : undefined) [2]` is LT.
    fn compare_list_by_id_chirho(&mut self) -> CoreIdChirho {
        if let Some(id_chirho) = self.lookup_body_backed_name_id_chirho(COMPARE_LIST_BY_CHIRHO) {
            return id_chirho;
        }
        let placeholder_ty_chirho = TyChirho::int_chirho();
        let ordering_ty_chirho = TyChirho::ConChirho("Ordering".to_string());
        let fn_binder_chirho = self
            .conditional_dict_binder_chirho(COMPARE_LIST_BY_CHIRHO, placeholder_ty_chirho.clone());
        let fn_id_chirho = fn_binder_chirho.id_chirho;
        let mut fresh_chirho = |name_chirho: &str| {
            self.fresh_binder_chirho(name_chirho, placeholder_ty_chirho.clone())
        };
        let cmp_chirho = fresh_chirho("cmp");
        let xs_chirho = fresh_chirho("xs");
        let ys_chirho = fresh_chirho("ys");
        let x_chirho = fresh_chirho("x");
        let xs_tail_chirho = fresh_chirho("xs'");
        let y_chirho = fresh_chirho("y");
        let ys_tail_chirho = fresh_chirho("ys'");
        let wild_head_chirho = fresh_chirho("_w1");
        let wild_tail_chirho = fresh_chirho("_w2");
        let xs_scrutinee_chirho = fresh_chirho("_xs");
        let ys_cons_scrutinee_chirho = fresh_chirho("_ys1");
        let ys_nil_scrutinee_chirho = fresh_chirho("_ys2");
        let ordering_scrutinee_chirho = fresh_chirho("_ord");

        let var_chirho =
            |binder_chirho: &BinderChirho| CoreExprChirho::VarChirho(binder_chirho.id_chirho);
        let app_chirho =
            |fun_chirho: CoreExprChirho, arg_chirho: CoreExprChirho| CoreExprChirho::AppChirho {
                fun_chirho: Box::new(fun_chirho),
                arg_chirho: Box::new(arg_chirho),
            };
        let con_chirho = |name_chirho: &str| CoreExprChirho::ConAppChirho {
            con_name_chirho: name_chirho.to_string(),
            args_chirho: vec![],
        };
        let alt_chirho =
            |name_chirho: &str, binders_chirho: Vec<BinderChirho>, rhs_chirho: CoreExprChirho| {
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(name_chirho.to_string()),
                    binders_chirho,
                    rhs_chirho,
                }
            };

        // EQ -> $compareListBy cmp xs' ys'
        let recurse_chirho = app_chirho(
            app_chirho(
                app_chirho(
                    CoreExprChirho::VarChirho(fn_id_chirho),
                    var_chirho(&cmp_chirho),
                ),
                var_chirho(&xs_tail_chirho),
            ),
            var_chirho(&ys_tail_chirho),
        );
        let element_case_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(app_chirho(
                app_chirho(var_chirho(&cmp_chirho), var_chirho(&x_chirho)),
                var_chirho(&y_chirho),
            )),
            bind_chirho: ordering_scrutinee_chirho,
            result_ty_chirho: ordering_ty_chirho.clone(),
            alts_chirho: vec![
                alt_chirho("LT", vec![], con_chirho("LT")),
                alt_chirho("EQ", vec![], recurse_chirho),
                alt_chirho("GT", vec![], con_chirho("GT")),
            ],
        };
        let cons_case_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(var_chirho(&ys_chirho)),
            bind_chirho: ys_cons_scrutinee_chirho,
            result_ty_chirho: ordering_ty_chirho.clone(),
            alts_chirho: vec![
                alt_chirho("[]", vec![], con_chirho("GT")),
                alt_chirho(":", vec![y_chirho, ys_tail_chirho], element_case_chirho),
            ],
        };
        let nil_case_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(var_chirho(&ys_chirho)),
            bind_chirho: ys_nil_scrutinee_chirho,
            result_ty_chirho: ordering_ty_chirho.clone(),
            alts_chirho: vec![
                alt_chirho("[]", vec![], con_chirho("EQ")),
                alt_chirho(
                    ":",
                    vec![wild_head_chirho, wild_tail_chirho],
                    con_chirho("LT"),
                ),
            ],
        };
        let outer_case_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(var_chirho(&xs_chirho)),
            bind_chirho: xs_scrutinee_chirho,
            result_ty_chirho: ordering_ty_chirho,
            alts_chirho: vec![
                alt_chirho("[]", vec![], nil_case_chirho),
                alt_chirho(":", vec![x_chirho, xs_tail_chirho], cons_case_chirho),
            ],
        };
        let lam_chirho =
            |binder_chirho: BinderChirho, body_chirho: CoreExprChirho| CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho: Box::new(body_chirho),
            };
        let body_chirho = lam_chirho(
            cmp_chirho,
            lam_chirho(xs_chirho, lam_chirho(ys_chirho, outer_case_chirho)),
        );

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: fn_binder_chirho,
            rhs_chirho: body_chirho,
            is_rec_chirho: true,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        fn_id_chirho
    }
}
