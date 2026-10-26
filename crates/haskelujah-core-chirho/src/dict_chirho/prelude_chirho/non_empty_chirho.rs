// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The one implementation every routed `toList` reaches (unit 517): Data.List.NonEmpty's,
//! and Data.Foldable's, over the two shapes a NonEmpty value has today. `NE.fromList` and
//! `NE.map` still build ordinary lists, while `x :| xs` builds a real `:|`. So the list
//! alternatives come first and return the list itself, `x :| xs` becomes `x : xs`, and any
//! other value comes back unchanged, exactly as the IsList identity returned it before.
//!
//! The name starts with `$`, so no source program can spell it. It holds no `.` that a
//! qualifier strip could read as a class method, and no prefix a later pass gives meaning
//! (`$prim_`, `$sel_`, `$w`, `$spec_`, `$tuple`, `$Dict_`).
//! workflow: language-features-chirho/own-class-method-occurrences-chirho

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho,
    DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho, TyVarChirho,
};

/// The routed binding's name, which the desugarer's route table refers to.
pub(crate) const TO_LIST_ROUTE_CHIRHO: &str = "$list_or_nonempty_to_list_chirho";

/// The constructor of `NonEmpty`, and of lists.
const NON_EMPTY_CON_CHIRHO: &str = ":|";
const CONS_CHIRHO: &str = ":";

impl DictPassCtxChirho {
    /// `\xs -> case xs of { [] -> xs; _ : _ -> xs; h :| t -> h : t; _ -> xs }`.
    /// workflow: language-features-chirho/own-class-method-occurrences-chirho
    pub(super) fn generate_to_list_route_chirho(&mut self) {
        let element_ty_chirho = TyChirho::VarChirho(TyVarChirho(9990));
        let list_ty_chirho = TyChirho::ListChirho(Box::new(element_ty_chirho.clone()));
        let xs_chirho = self.fresh_binder_chirho("route_xs_chirho", list_ty_chirho.clone());
        let scrutinee_chirho =
            self.fresh_binder_chirho("route_scrutinee_chirho", list_ty_chirho.clone());
        let list_head_chirho =
            self.fresh_binder_chirho("route_list_head_chirho", element_ty_chirho.clone());
        let list_tail_chirho =
            self.fresh_binder_chirho("route_list_tail_chirho", list_ty_chirho.clone());
        let non_empty_head_chirho =
            self.fresh_binder_chirho("route_non_empty_head_chirho", element_ty_chirho);
        let non_empty_tail_chirho =
            self.fresh_binder_chirho("route_non_empty_tail_chirho", list_ty_chirho.clone());
        let unchanged_chirho = || CoreExprChirho::VarChirho(xs_chirho.id_chirho);
        let alts_chirho = vec![
            CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                binders_chirho: vec![],
                rhs_chirho: unchanged_chirho(),
            },
            CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho(CONS_CHIRHO.to_string()),
                binders_chirho: vec![list_head_chirho, list_tail_chirho],
                rhs_chirho: unchanged_chirho(),
            },
            CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho(NON_EMPTY_CON_CHIRHO.to_string()),
                rhs_chirho: CoreExprChirho::ConAppChirho {
                    con_name_chirho: CONS_CHIRHO.to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(non_empty_head_chirho.id_chirho),
                        CoreExprChirho::VarChirho(non_empty_tail_chirho.id_chirho),
                    ],
                },
                binders_chirho: vec![non_empty_head_chirho, non_empty_tail_chirho],
            },
            CoreAltChirho {
                con_chirho: AltConChirho::DefaultChirho,
                binders_chirho: vec![],
                rhs_chirho: unchanged_chirho(),
            },
        ];
        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(unchanged_chirho()),
            bind_chirho: scrutinee_chirho,
            result_ty_chirho: list_ty_chirho.clone(),
            alts_chirho,
        };
        let id_chirho = self.resolve_or_fresh_id_chirho(TO_LIST_ROUTE_CHIRHO);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho,
                name_chirho: TO_LIST_ROUTE_CHIRHO.to_string(),
                ty_chirho: TyChirho::fun_chirho(list_ty_chirho.clone(), list_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
