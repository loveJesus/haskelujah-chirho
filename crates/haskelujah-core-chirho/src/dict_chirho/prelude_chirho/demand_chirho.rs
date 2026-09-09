// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A visible seq reference is a real Core function, not a backend-specific
//! missing-name fallback. A case demands only its first operand's WHNF.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho,
    DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho, TyVarChirho,
};

impl DictPassCtxChirho {
    pub(super) fn generate_demand_prelude_chirho(&mut self) {
        let first_ty_chirho = TyChirho::VarChirho(TyVarChirho(9990));
        let second_ty_chirho = TyChirho::VarChirho(TyVarChirho(9991));
        let first_chirho = self.fresh_binder_chirho("seq_first_chirho", first_ty_chirho.clone());
        let second_chirho = self.fresh_binder_chirho("seq_second_chirho", second_ty_chirho.clone());
        let demanded_chirho =
            self.fresh_binder_chirho("seq_demanded_chirho", first_ty_chirho.clone());
        let id_chirho = self.resolve_or_fresh_id_chirho("seq");
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho,
                name_chirho: "seq".to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![first_ty_chirho, second_ty_chirho.clone()],
                    second_ty_chirho.clone(),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: first_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: second_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                            first_chirho.id_chirho,
                        )),
                        bind_chirho: demanded_chirho,
                        result_ty_chirho: second_ty_chirho,
                        alts_chirho: vec![CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::VarChirho(second_chirho.id_chirho),
                        }],
                    }),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
