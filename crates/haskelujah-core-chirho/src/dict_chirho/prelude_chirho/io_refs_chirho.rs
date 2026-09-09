// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One IORef Prelude definition; modifications sequence read and write actions.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use super::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, DictPassCtxChirho, InlineAnnotationChirho,
    SpanChirho, TyChirho,
};

impl DictPassCtxChirho {
    /// Generate Data.IORef Prelude bindings.
    /// newIORef, readIORef, writeIORef, modifyIORef are all primops handled by
    /// the STG lowerer and runtime, but we need Core IR wrapper bindings.
    pub(super) fn generate_ioref_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));

        // ── newIORef :: a -> IORef a ──
        // Just a wrapper that passes through to the primop
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("newIORef");
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "newIORef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "newIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), TyChirho::int_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── readIORef :: IORef a -> a ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("readIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "readIORef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "readIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── writeIORef :: IORef a -> a -> IO () ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("writeIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let v_chirho = self.fresh_binder_chirho("v", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "writeIORef#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(r_chirho.id_chirho),
                            CoreExprChirho::VarChirho(v_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "writeIORef".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![TyChirho::int_chirho(), a_chirho.clone()],
                        TyChirho::unit_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── modifyIORef :: IORef a -> (a -> a) -> IO () ──
        // Implemented as: readIORef r >>= \v -> writeIORef r (f v)
        // Reading the reference is an action, not a pure value expression.
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("modifyIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
            );
            let v_chirho = self.fresh_binder_chirho("v", a_chirho.clone());

            // readIORef# r
            let read_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "readIORef#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho)],
            };

            // f v
            let fv_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(v_chirho.id_chirho)),
            };

            // writeIORef# r (f v)
            let write_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "writeIORef#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho), fv_chirho],
            };

            // Sequence the read before supplying its lazy result to the updater.
            let update_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "bindIO#".to_string(),
                args_chirho: vec![
                    read_chirho,
                    CoreExprChirho::LamChirho {
                        binder_chirho: v_chirho,
                        body_chirho: Box::new(write_chirho),
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho,
                    body_chirho: Box::new(update_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "modifyIORef".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::int_chirho(),
                            TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                        ],
                        TyChirho::unit_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }
}
