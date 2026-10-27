// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The fold behind `maximum` and `minimum` (rows 565 and 627). GHC.List defines them as
//! `foldl1' max` and `foldl1' min`: they fold with the element type's own `max` and `min`
//! methods, an instance's override included. Row 565 folded compare-backed picks, which read
//! `compare` where GHC reads `max`. Since row 627 an evidenced occurrence folds the key's
//! `Ord.max` or `Ord.min` row, or the binding's own dictionary slot, with
//! `$ord_fold1_chirho`. The helper starts from the list's head without a `[]` alternative, so
//! an empty list fails the same way on every key, as the Int-shaped bindings do.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho,
    DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho, TyVarChirho,
};

/// Each list extremum and the Ord method its fold steps with.
pub(in crate::dict_chirho) const ORDERING_FOLDS_CHIRHO: [(&str, &str); 2] =
    [("maximum", "max"), ("minimum", "min")];

/// The fold every evidenced `maximum` and `minimum` applies to its method.
pub(in crate::dict_chirho) const ORDERING_FOLD1_CHIRHO: &str = "$ord_fold1_chirho";

fn app_chirho(fun_chirho: CoreExprChirho, arg_chirho: CoreExprChirho) -> CoreExprChirho {
    CoreExprChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
    }
}

impl DictPassCtxChirho {
    /// Generate `$ord_fold1_chirho = \pick xs -> case xs of (:) h t -> foldl pick h t`.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_ordering_extrema_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(9975));
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));
        let pick_ty_chirho =
            TyChirho::fun_n_chirho(vec![a_chirho.clone(), a_chirho.clone()], a_chirho.clone());
        let id_chirho = self.resolve_or_fresh_id_chirho(ORDERING_FOLD1_CHIRHO);
        let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");
        let pick_chirho = self.fresh_binder_chirho("pick", pick_ty_chirho.clone());
        let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
        let head_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
        let tail_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
        let wild_chirho = self.fresh_binder_chirho("$xs", list_a_chirho.clone());
        let folded_chirho = app_chirho(
            app_chirho(
                app_chirho(
                    CoreExprChirho::VarChirho(foldl_id_chirho),
                    CoreExprChirho::VarChirho(pick_chirho.id_chirho),
                ),
                CoreExprChirho::VarChirho(head_chirho.id_chirho),
            ),
            CoreExprChirho::VarChirho(tail_chirho.id_chirho),
        );
        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            bind_chirho: wild_chirho,
            result_ty_chirho: a_chirho.clone(),
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho(":".to_string()),
                binders_chirho: vec![head_chirho, tail_chirho],
                rhs_chirho: folded_chirho,
            }],
        };
        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: pick_chirho,
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            }),
        };
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho,
                name_chirho: ORDERING_FOLD1_CHIRHO.to_string(),
                ty_chirho: TyChirho::fun_n_chirho(vec![pick_ty_chirho, list_a_chirho], a_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
