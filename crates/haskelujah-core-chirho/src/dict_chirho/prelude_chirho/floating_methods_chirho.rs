// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Floating methods at Double that the Prelude defines over the unary primops (row 578).
//! The checker typed `logBase` as `Floating a => a -> a -> a`, but no Core bound it, so
//! every engine failed with a missing binding. GHC's `logBase x y` is `log y / log x`, so
//! `logBase 10 1000` is 2.9999999999999996, as GHC prints it.
//! workflow: testing-chirho/native-fail-loud-chirho

use super::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, DictPassCtxChirho,
    InlineAnnotationChirho, SpanChirho, TyChirho,
};

fn log_chirho(id_chirho: CoreIdChirho) -> CoreExprChirho {
    CoreExprChirho::PrimOpChirho {
        name_chirho: "log#".to_string(),
        args_chirho: vec![CoreExprChirho::VarChirho(id_chirho)],
    }
}

impl DictPassCtxChirho {
    /// Generate `$prim_Floating_logBase_Double = \x y -> log# y /.# log# x` and its Prelude
    /// alias `logBase`.
    /// workflow: testing-chirho/native-fail-loud-chirho
    pub(super) fn generate_floating_binary_prelude_chirho(&mut self) {
        let double_ty_chirho = TyChirho::double_chirho();
        let prim_name_chirho = "$prim_Floating_logBase_Double";
        let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
        let base_chirho = self.fresh_binder_chirho("x", double_ty_chirho.clone());
        let value_chirho = self.fresh_binder_chirho("y", double_ty_chirho.clone());
        let body_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "/.#".to_string(),
            args_chirho: vec![
                log_chirho(value_chirho.id_chirho),
                log_chirho(base_chirho.id_chirho),
            ],
        };
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: prim_id_chirho,
                name_chirho: prim_name_chirho.to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![double_ty_chirho.clone(), double_ty_chirho.clone()],
                    double_ty_chirho,
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: base_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: value_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        self.push_prelude_alias_chirho("logBase", prim_name_chirho);
    }
}
