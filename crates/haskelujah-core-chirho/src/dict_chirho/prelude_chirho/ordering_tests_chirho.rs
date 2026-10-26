// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The four `Ordering -> Bool` tests behind the compare route (row 551). GHC's `Ord`
//! class defaults read `<`, `<=`, `>` and `>=` off one `compare`: `<` accepts LT, `<=`
//! rejects GT, `>` accepts GT and `>=` rejects LT. Each test scrutinizes its Ordering
//! once, so a routed `compare a b` is evaluated once.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho,
    DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho,
};

/// Each ordering primop the desugarer emits, its test binding, the constructor the
/// test decides on, and whether that constructor answers True.
pub(in crate::dict_chirho) const ORDERING_TESTS_CHIRHO: [(&str, &str, &str, bool); 4] = [
    ("<#", "$ordering_lt_chirho", "LT", true),
    ("<=#", "$ordering_le_chirho", "GT", false),
    (">#", "$ordering_gt_chirho", "GT", true),
    (">=#", "$ordering_ge_chirho", "LT", false),
];

fn bool_chirho(value_chirho: bool) -> CoreExprChirho {
    CoreExprChirho::ConAppChirho {
        con_name_chirho: if value_chirho { "True" } else { "False" }.to_string(),
        args_chirho: vec![],
    }
}

impl DictPassCtxChirho {
    /// Generate the four Ordering tests: `\o -> case o of { C -> v; _ -> not v }`.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_ordering_tests_prelude_chirho(&mut self) {
        let ordering_ty_chirho = TyChirho::ConChirho("Ordering".to_string());
        let bool_ty_chirho = TyChirho::ConChirho("Bool".to_string());
        for (_, name_chirho, con_chirho, accepts_chirho) in ORDERING_TESTS_CHIRHO {
            let id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            let ordering_chirho = self.fresh_binder_chirho("o", ordering_ty_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", ordering_ty_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ordering_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: bool_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(con_chirho.to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: bool_chirho(accepts_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: bool_chirho(!accepts_chirho),
                    },
                ],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![ordering_ty_chirho.clone()],
                        bool_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: ordering_chirho,
                    body_chirho: Box::new(body_chirho),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }
}
