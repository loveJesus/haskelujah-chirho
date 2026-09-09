// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One list renderer for bootstrap and evidence-driven instances. The element's
//! proven Show implementation is composed here; it is never guessed from bits.
//! A single recursive worker renders each element once, even for nested shapes.
//! Workflow: spec-chirho/workflows-chirho/print-show-evidence-chirho.md.

use super::{
    AltConChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, DictPassCtxChirho,
    InlineAnnotationChirho, ShowShapeChirho, TyChirho, join_string_parts_chirho, string_lit_chirho,
};
use haskelujah_typing_chirho::ty_chirho::TyVarChirho;

fn apply_chirho(
    function_chirho: CoreExprChirho,
    argument_chirho: CoreExprChirho,
) -> CoreExprChirho {
    CoreExprChirho::AppChirho {
        fun_chirho: Box::new(function_chirho),
        arg_chirho: Box::new(argument_chirho),
    }
}

impl DictPassCtxChirho {
    pub(super) fn show_list_expr_chirho(
        &mut self,
        element_chirho: &ShowShapeChirho,
        value_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let element_ty_chirho = TyChirho::VarChirho(TyVarChirho(self.next_id_chirho));
        self.next_id_chirho += 1;
        let list_ty_chirho = TyChirho::ListChirho(Box::new(element_ty_chirho.clone()));
        let worker_chirho = self.fresh_binder_chirho(
            &format!("$show_list_tail_{}_chirho", self.next_id_chirho),
            TyChirho::fun_n_chirho(
                [TyChirho::string_chirho(), list_ty_chirho.clone()],
                TyChirho::string_chirho(),
            ),
        );
        let separator_chirho =
            self.fresh_binder_chirho("show_separator_chirho", TyChirho::string_chirho());
        let list_chirho = self.fresh_binder_chirho("show_list_chirho", list_ty_chirho.clone());
        let head_chirho = self.fresh_binder_chirho("show_head_chirho", element_ty_chirho);
        let tail_chirho = self.fresh_binder_chirho("show_tail_chirho", list_ty_chirho);
        let shown_head_chirho = self.show_shape_expr_chirho(
            element_chirho,
            CoreExprChirho::VarChirho(head_chirho.id_chirho),
            0,
        );
        let shown_tail_chirho = apply_chirho(
            apply_chirho(
                CoreExprChirho::VarChirho(worker_chirho.id_chirho),
                string_lit_chirho(","),
            ),
            CoreExprChirho::VarChirho(tail_chirho.id_chirho),
        );
        let body_chirho = self.show_case_chirho(
            CoreExprChirho::VarChirho(list_chirho.id_chirho),
            vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: string_lit_chirho("]"),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![head_chirho, tail_chirho],
                    rhs_chirho: join_string_parts_chirho(vec![
                        CoreExprChirho::VarChirho(separator_chirho.id_chirho),
                        shown_head_chirho,
                        shown_tail_chirho,
                    ]),
                },
            ],
        );
        let initial_chirho = apply_chirho(
            apply_chirho(
                CoreExprChirho::VarChirho(worker_chirho.id_chirho),
                string_lit_chirho(""),
            ),
            value_chirho,
        );
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: worker_chirho,
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: separator_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: list_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            },
            is_rec_chirho: true,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        join_string_parts_chirho(vec![string_lit_chirho("["), initial_chirho])
    }
}
