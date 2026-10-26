// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The function an unapplied, evidenced `print` becomes: `\show x -> putStrLn (show x)`.
//! In `mapM_ print xs`, the solved Show predicate names the element type, and the
//! method-reference rewrite passes that type's `$prim_Show_show_<key>` here. Before row
//! 550, such a `print` reached the generic Int-only binding. Natively, it printed a Double's
//! bits, a Bool as 1 or 0, a Char as its code point and a String as a heap address.
//! workflow: print-show-evidence-chirho

use super::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, DictPassCtxChirho, InlineAnnotationChirho,
    SpanChirho, TyChirho, TyVarChirho,
};

/// The generated helper's Core name; the rewrite looks it up by this name.
pub(crate) const PRINT_VIA_SHOW_CHIRHO: &str = "$print_via_show_chirho";

impl DictPassCtxChirho {
    /// Generate `$print_via_show_chirho = \show x -> putStrLn (show x)`.
    /// workflow: print-show-evidence-chirho
    pub(super) fn generate_print_via_show_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(9960));
        let show_ty_chirho = TyChirho::fun_chirho(a_chirho.clone(), TyChirho::string_chirho());
        let id_chirho = self.resolve_or_fresh_id_chirho(PRINT_VIA_SHOW_CHIRHO);
        let put_str_ln_id_chirho = self.resolve_or_fresh_id_chirho("putStrLn");
        let show_chirho = self.fresh_binder_chirho("show", show_ty_chirho.clone());
        let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
        let shown_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(show_chirho.id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
        };
        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: show_chirho,
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(put_str_ln_id_chirho)),
                    arg_chirho: Box::new(shown_chirho),
                }),
            }),
        };
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho,
                name_chirho: PRINT_VIA_SHOW_CHIRHO.to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![show_ty_chirho, a_chirho],
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
