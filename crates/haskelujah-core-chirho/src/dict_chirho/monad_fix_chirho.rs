// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Generated MonadFix instance bodies.
//!
//! IO delegates knot tying to the reusable-action execution lowering. Maybe follows
//! `base`'s lazy `unJust` knot, and list follows the recursive `head`/`tail` construction from
//! `Control.Monad.Fix`.
//!
//! workflow: recursive-do-chirho

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::DictPassCtxChirho;
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho,
    InlineAnnotationChirho,
};

impl DictPassCtxChirho {
    pub(super) fn generate_monad_fix_bindings_chirho(&mut self) {
        let any_ty_chirho = TyChirho::VarChirho(TyVarChirho(9950));
        self.generate_monad_fix_io_chirho(&any_ty_chirho);
        self.generate_monad_fix_maybe_chirho(&any_ty_chirho);
        self.generate_monad_fix_list_chirho(&any_ty_chirho);
    }

    fn generate_monad_fix_io_chirho(&mut self, any_ty_chirho: &TyChirho) {
        let prim_name_chirho = "$prim_MonadFix_mfix_IO";
        let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
        let function_chirho = self.fresh_binder_chirho("function_chirho", any_ty_chirho.clone());
        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: function_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: "mfixIO#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(function_chirho.id_chirho)],
            }),
        };
        self.push_monad_fix_binding_chirho(
            prim_name_chirho,
            prim_id_chirho,
            rhs_chirho,
            any_ty_chirho,
            false,
        );
    }

    fn generate_monad_fix_maybe_chirho(&mut self, any_ty_chirho: &TyChirho) {
        let prim_name_chirho = "$prim_MonadFix_mfix_Maybe";
        let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
        let function_chirho = self.fresh_binder_chirho("function_chirho", any_ty_chirho.clone());
        let result_chirho = self.fresh_binder_chirho("result_chirho", any_ty_chirho.clone());
        let value_chirho = self.fresh_binder_chirho("value_chirho", any_ty_chirho.clone());
        let just_value_chirho =
            self.fresh_binder_chirho("just_value_chirho", any_ty_chirho.clone());
        let case_binder_chirho =
            self.fresh_binder_chirho("maybe_case_chirho", any_ty_chirho.clone());

        let result_rhs_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(function_chirho.id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(value_chirho.id_chirho)),
        };
        let value_rhs_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(result_chirho.id_chirho)),
            bind_chirho: case_binder_chirho,
            result_ty_chirho: any_ty_chirho.clone(),
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                binders_chirho: vec![just_value_chirho.clone()],
                rhs_chirho: CoreExprChirho::VarChirho(just_value_chirho.id_chirho),
            }],
        };
        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: function_chirho,
            body_chirho: Box::new(CoreExprChirho::LetChirho {
                rec_chirho: true,
                binds_chirho: vec![
                    (result_chirho.clone(), result_rhs_chirho),
                    (value_chirho, value_rhs_chirho),
                ],
                body_chirho: Box::new(CoreExprChirho::VarChirho(result_chirho.id_chirho)),
            }),
        };
        self.push_monad_fix_binding_chirho(
            prim_name_chirho,
            prim_id_chirho,
            rhs_chirho,
            any_ty_chirho,
            false,
        );
    }

    fn generate_monad_fix_list_chirho(&mut self, any_ty_chirho: &TyChirho) {
        let prim_name_chirho = "$prim_MonadFix_mfix_[]";
        let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
        let function_chirho = self.fresh_binder_chirho("function_chirho", any_ty_chirho.clone());
        let seed_chirho = self.fresh_binder_chirho("seed_chirho", any_ty_chirho.clone());
        let first_chirho = self.fresh_binder_chirho("first_chirho", any_ty_chirho.clone());

        let seed_rhs_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(function_chirho.id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(first_chirho.id_chirho)),
        };
        let head_chirho = self.fresh_binder_chirho("head_chirho", any_ty_chirho.clone());
        let ignored_tail_chirho =
            self.fresh_binder_chirho("ignored_tail_chirho", any_ty_chirho.clone());
        let first_case_chirho =
            self.fresh_binder_chirho("first_case_chirho", any_ty_chirho.clone());
        let first_rhs_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(seed_chirho.id_chirho)),
            bind_chirho: first_case_chirho,
            result_ty_chirho: any_ty_chirho.clone(),
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho(":".to_string()),
                binders_chirho: vec![head_chirho.clone(), ignored_tail_chirho],
                rhs_chirho: CoreExprChirho::VarChirho(head_chirho.id_chirho),
            }],
        };

        let result_case_chirho =
            self.fresh_binder_chirho("result_case_chirho", any_ty_chirho.clone());
        let result_head_chirho =
            self.fresh_binder_chirho("result_head_chirho", any_ty_chirho.clone());
        let result_tail_chirho =
            self.fresh_binder_chirho("result_tail_chirho", any_ty_chirho.clone());
        let next_value_chirho =
            self.fresh_binder_chirho("next_value_chirho", any_ty_chirho.clone());
        let next_list_chirho = self.fresh_binder_chirho("next_list_chirho", any_ty_chirho.clone());
        let next_head_chirho = self.fresh_binder_chirho("next_head_chirho", any_ty_chirho.clone());
        let next_tail_chirho = self.fresh_binder_chirho("next_tail_chirho", any_ty_chirho.clone());
        let next_case_chirho = self.fresh_binder_chirho("next_case_chirho", any_ty_chirho.clone());

        let next_list_rhs_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(function_chirho.id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(next_value_chirho.id_chirho)),
        };
        let tail_function_chirho = CoreExprChirho::LamChirho {
            binder_chirho: next_value_chirho,
            body_chirho: Box::new(CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(next_list_chirho.clone(), next_list_rhs_chirho)],
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                        next_list_chirho.id_chirho,
                    )),
                    bind_chirho: next_case_chirho,
                    result_ty_chirho: any_ty_chirho.clone(),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![next_head_chirho, next_tail_chirho.clone()],
                        rhs_chirho: CoreExprChirho::VarChirho(next_tail_chirho.id_chirho),
                    }],
                }),
            }),
        };
        let recursive_tail_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(prim_id_chirho)),
            arg_chirho: Box::new(tail_function_chirho),
        };
        let result_rhs_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(seed_chirho.id_chirho)),
            bind_chirho: result_case_chirho,
            result_ty_chirho: any_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::ConAppChirho {
                        con_name_chirho: "[]".to_string(),
                        args_chirho: vec![],
                    },
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![result_head_chirho.clone(), result_tail_chirho],
                    rhs_chirho: CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(result_head_chirho.id_chirho),
                            recursive_tail_chirho,
                        ],
                    },
                },
            ],
        };

        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: function_chirho,
            body_chirho: Box::new(CoreExprChirho::LetChirho {
                rec_chirho: true,
                binds_chirho: vec![
                    (seed_chirho, seed_rhs_chirho),
                    (first_chirho, first_rhs_chirho),
                ],
                body_chirho: Box::new(result_rhs_chirho),
            }),
        };
        self.push_monad_fix_binding_chirho(
            prim_name_chirho,
            prim_id_chirho,
            rhs_chirho,
            any_ty_chirho,
            true,
        );
    }

    fn push_monad_fix_binding_chirho(
        &mut self,
        prim_name_chirho: &str,
        prim_id_chirho: crate::expr_chirho::CoreIdChirho,
        rhs_chirho: CoreExprChirho,
        any_ty_chirho: &TyChirho,
        is_rec_chirho: bool,
    ) {
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: prim_id_chirho,
                name_chirho: prim_name_chirho.to_string(),
                ty_chirho: any_ty_chirho.clone(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho,
            is_rec_chirho,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn generates_three_backed_monadfix_bodies_chirho() {
        let mut ctx_chirho = DictPassCtxChirho::new_chirho(0, HashMap::new(), HashMap::new());
        ctx_chirho.generate_monad_fix_bindings_chirho();

        let names_chirho = ctx_chirho
            .generated_bindings_chirho
            .iter()
            .map(|binding_chirho| binding_chirho.binder_chirho.name_chirho.as_str())
            .collect::<Vec<_>>();
        assert!(names_chirho.contains(&"$prim_MonadFix_mfix_IO"));
        assert!(names_chirho.contains(&"$prim_MonadFix_mfix_Maybe"));
        assert!(names_chirho.contains(&"$prim_MonadFix_mfix_[]"));
        assert!(
            ctx_chirho
                .generated_bindings_chirho
                .iter()
                .all(|binding_chirho| !matches!(
                    binding_chirho.rhs_chirho,
                    CoreExprChirho::LitChirho(_)
                ))
        );
    }
}
