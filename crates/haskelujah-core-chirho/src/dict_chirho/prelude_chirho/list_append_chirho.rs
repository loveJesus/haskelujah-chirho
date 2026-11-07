// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One generic lazy append body and forwarding entries backed by selected-value provenance.
//! Workflow: compiler-pipeline-chirho/native-symbol-identity-chirho.md#polymorphic-list-append-chirho.

use haskelujah_ast_chirho::name_chirho::CompilerValueChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreModuleChirho,
    DictPassCtxChirho, InlineAnnotationChirho,
};

impl DictPassCtxChirho {
    pub(super) fn generate_list_append_body_chirho(
        &mut self,
        a_chirho: TyChirho,
        list_a_chirho: TyChirho,
    ) {
        // append :: [a] -> [a] -> [a]
        // append [] ys = ys; append (x:xs) ys = x : append xs ys
        {
            let append_id_chirho = self.resolve_or_fresh_id_chirho("append");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let append_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    append_rec_chirho,
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(ys_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ys_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: append_id_chirho,
                    name_chirho: "append".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        list_a_chirho.clone(),
                        TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// The typed provenance, not a builtin bit, free name or reserved spelling,
    /// selects this compiler body. It is published as compiler-owned only when
    /// its actual forwarding RHS has been generated below.
    pub(crate) fn generate_certified_value_bindings_chirho(
        &mut self,
        module_chirho: &CoreModuleChirho,
    ) {
        for (&id_chirho, &value_chirho) in &module_chirho
            .provenance_chirho
            .compiler_value_references_chirho
        {
            if self.module_source_binder_ids_chirho.contains(&id_chirho) {
                continue;
            }
            match value_chirho {
                CompilerValueChirho::ListAppendChirho => {
                    let target_chirho = self
                        .lookup_compiler_owned_name_id_chirho("append")
                        .expect("certified append requires the compiler-owned lazy body Chirho");
                    let element_chirho = TyChirho::VarChirho(TyVarChirho(9995));
                    let list_chirho = TyChirho::ListChirho(Box::new(element_chirho));
                    let left_chirho =
                        self.fresh_binder_chirho("append_left_chirho", list_chirho.clone());
                    let right_chirho =
                        self.fresh_binder_chirho("append_right_chirho", list_chirho.clone());
                    let rhs_chirho = CoreExprChirho::LamChirho {
                        binder_chirho: left_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: right_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::VarChirho(target_chirho)),
                                    arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                        left_chirho.id_chirho,
                                    )),
                                }),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                    right_chirho.id_chirho,
                                )),
                            }),
                        }),
                    };
                    self.names_chirho
                        .insert(id_chirho, value_chirho.core_name_chirho().to_string());
                    self.generated_bindings_chirho.push(CoreBindingChirho {
                        binder_chirho: BinderChirho {
                            id_chirho,
                            name_chirho: value_chirho.core_name_chirho().to_string(),
                            ty_chirho: TyChirho::fun_n_chirho(
                                vec![list_chirho.clone(), list_chirho.clone()],
                                list_chirho,
                            ),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        rhs_chirho,
                        is_rec_chirho: false,
                        inline_chirho: InlineAnnotationChirho::NeverChirho,
                    });
                }
            }
        }
    }
}
