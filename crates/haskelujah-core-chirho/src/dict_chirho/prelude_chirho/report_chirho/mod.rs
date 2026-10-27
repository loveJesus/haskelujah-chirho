// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Haskell 2010 report's Prelude functions as generated Core, and the Core builders they
//! share: the folds and `until` (`report_folds_chirho`, rows 548 and 558) and the text
//! functions (`report_text_chirho`, row 579).

mod report_folds_chirho;
mod report_text_chirho;

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, DictPassCtxChirho, InlineAnnotationChirho, SpanChirho, TyChirho, TyVarChirho,
};

fn var_chirho(binder_chirho: &BinderChirho) -> CoreExprChirho {
    CoreExprChirho::VarChirho(binder_chirho.id_chirho)
}

/// `fun arg1 arg2 ...` as nested applications.
fn apply_chirho(fun_chirho: CoreExprChirho, args_chirho: Vec<CoreExprChirho>) -> CoreExprChirho {
    args_chirho
        .into_iter()
        .fold(fun_chirho, |fun_chirho, arg_chirho| {
            CoreExprChirho::AppChirho {
                fun_chirho: Box::new(fun_chirho),
                arg_chirho: Box::new(arg_chirho),
            }
        })
}

/// `\b1 -> \b2 -> ... -> body`.
fn lambdas_chirho(
    binders_chirho: Vec<BinderChirho>,
    body_chirho: CoreExprChirho,
) -> CoreExprChirho {
    binders_chirho
        .into_iter()
        .rev()
        .fold(body_chirho, |body_chirho, binder_chirho| {
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho: Box::new(body_chirho),
            }
        })
}

fn alt_chirho(
    con_chirho: &str,
    binders_chirho: Vec<BinderChirho>,
    rhs_chirho: CoreExprChirho,
) -> CoreAltChirho {
    CoreAltChirho {
        con_chirho: AltConChirho::DataConChirho(con_chirho.to_string()),
        binders_chirho,
        rhs_chirho,
    }
}

impl DictPassCtxChirho {
    fn push_report_binding_chirho(
        &mut self,
        id_chirho: CoreIdChirho,
        name_chirho: &str,
        ty_chirho: TyChirho,
        rhs_chirho: CoreExprChirho,
        is_rec_chirho: bool,
    ) {
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho,
                name_chirho: name_chirho.to_string(),
                ty_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho,
            is_rec_chirho,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
