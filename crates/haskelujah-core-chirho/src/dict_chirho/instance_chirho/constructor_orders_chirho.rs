// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Comparisons by constructor order for enumerations whose constructors carry no fields.

use super::DictPassCtxChirho;
use crate::expr_chirho::{
    AltConChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, InlineAnnotationChirho,
};
use haskelujah_typing_chirho::ty_chirho::TyChirho;

impl DictPassCtxChirho {
    /// Generate `$prim_Ord_compare_Ordering = \x y -> ...` using nested case dispatch.
    /// LT < EQ < GT, so: LT vs LT -> EQ, LT vs _ -> LT, EQ vs LT -> GT, EQ vs EQ -> EQ, EQ vs GT -> LT, GT vs GT -> EQ, GT vs _ -> GT
    pub(super) fn generate_compare_ordering_binding_chirho(&mut self, prim_name_chirho: &str) {
        let ord_ty_chirho = TyChirho::int_chirho(); // placeholder

        let x_chirho = self.fresh_binder_chirho("x", ord_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("y", ord_ty_chirho.clone());
        let scr_x_chirho = self.fresh_binder_chirho("_sx", ord_ty_chirho.clone());
        let scr_y1_chirho = self.fresh_binder_chirho("_sy1", ord_ty_chirho.clone());
        let scr_y2_chirho = self.fresh_binder_chirho("_sy2", ord_ty_chirho.clone());
        let scr_y3_chirho = self.fresh_binder_chirho("_sy3", ord_ty_chirho.clone());

        let lt_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "LT".to_string(),
            args_chirho: vec![],
        };
        let eq_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "EQ".to_string(),
            args_chirho: vec![],
        };
        let gt_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "GT".to_string(),
            args_chirho: vec![],
        };

        // case x of LT -> case y of { LT -> EQ; _ -> LT }
        let inner_lt_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y1_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: eq_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: lt_chirho.clone(),
                },
            ],
        };

        // case x of EQ -> case y of { LT -> GT; EQ -> EQ; GT -> LT }
        let inner_eq_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y2_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: gt_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: eq_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: lt_chirho.clone(),
                },
            ],
        };

        // case x of GT -> case y of { GT -> EQ; _ -> GT }
        let inner_gt_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y3_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: eq_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: gt_chirho.clone(),
                },
            ],
        };

        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            bind_chirho: scr_x_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_lt_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_eq_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_gt_chirho,
                },
            ],
        };

        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho,
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: y_chirho,
                body_chirho: Box::new(body_chirho),
            }),
        };

        let fn_ty_chirho = TyChirho::fun_chirho(
            ord_ty_chirho.clone(),
            TyChirho::fun_chirho(ord_ty_chirho.clone(), ord_ty_chirho),
        );
        let binder_chirho = self.fresh_binder_chirho(prim_name_chirho, fn_ty_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
