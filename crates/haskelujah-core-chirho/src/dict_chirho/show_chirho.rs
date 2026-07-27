// For God so loved the world, that he gave his only begotten Son, that whosoever believeth
// in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Concrete `Show` instance bodies expressed in portable Core.
//!
//! The STG interpreter understands compound `show*#` primops that the native
//! backends do not. These generators keep shared instances backend-agnostic by
//! using cases, scalar show primops, and string append instead.

use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::DictPassCtxChirho;
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreLitChirho,
    InlineAnnotationChirho,
};

fn string_lit_chirho(value_chirho: &str) -> CoreExprChirho {
    CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(value_chirho.to_string()))
}

fn append_strings_chirho(
    left_chirho: CoreExprChirho,
    right_chirho: CoreExprChirho,
) -> CoreExprChirho {
    CoreExprChirho::PrimOpChirho {
        name_chirho: "++#".to_string(),
        args_chirho: vec![left_chirho, right_chirho],
    }
}

fn join_string_parts_chirho(parts_chirho: Vec<CoreExprChirho>) -> CoreExprChirho {
    parts_chirho
        .into_iter()
        .rev()
        .fold(string_lit_chirho(""), |tail_chirho, part_chirho| {
            append_strings_chirho(part_chirho, tail_chirho)
        })
}

fn scalar_show_expr_chirho(
    type_key_chirho: &str,
    value_chirho: CoreExprChirho,
) -> Option<CoreExprChirho> {
    let primop_chirho = match type_key_chirho {
        "Int" => "showInt#",
        "Bool" => "showBool#",
        "Char" => "showChar#",
        "Double" => "showFloat#",
        "String" | "[Char]" => {
            return Some(join_string_parts_chirho(vec![
                string_lit_chirho("\""),
                value_chirho,
                string_lit_chirho("\""),
            ]));
        }
        _ => return None,
    };
    Some(CoreExprChirho::PrimOpChirho {
        name_chirho: primop_chirho.to_string(),
        args_chirho: vec![value_chirho],
    })
}

fn placeholder_ty_chirho(type_key_chirho: &str) -> TyChirho {
    match type_key_chirho {
        "Bool" => TyChirho::bool_chirho(),
        "Char" => TyChirho::char_chirho(),
        "Double" => TyChirho::double_chirho(),
        "String" | "[Char]" => TyChirho::string_chirho(),
        _ => TyChirho::int_chirho(),
    }
}

impl DictPassCtxChirho {
    fn push_show_binding_chirho(
        &mut self,
        prim_name_chirho: &str,
        arg_chirho: BinderChirho,
        body_chirho: CoreExprChirho,
    ) {
        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: arg_chirho.clone(),
            body_chirho: Box::new(body_chirho),
        };
        let binder_chirho = self.fresh_binder_chirho(
            prim_name_chirho,
            TyChirho::fun_chirho(arg_chirho.ty_chirho, TyChirho::string_chirho()),
        );
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }

    pub(super) fn generate_show_string_binding_chirho(&mut self, prim_name_chirho: &str) {
        let arg_chirho = self.fresh_binder_chirho("show_string_arg", TyChirho::string_chirho());
        let body_chirho =
            scalar_show_expr_chirho("String", CoreExprChirho::VarChirho(arg_chirho.id_chirho))
                .expect("String has a portable Show renderer");
        self.push_show_binding_chirho(prim_name_chirho, arg_chirho, body_chirho);
    }

    pub(super) fn generate_show_maybe_binding_chirho(
        &mut self,
        prim_name_chirho: &str,
        type_key_chirho: &str,
    ) {
        let inner_key_chirho = type_key_chirho
            .strip_prefix("Maybe ")
            .expect("Maybe Show key should include its argument");
        let arg_chirho = self.fresh_binder_chirho("show_maybe_arg", TyChirho::int_chirho());
        let case_binder_chirho =
            self.fresh_binder_chirho("show_maybe_case", TyChirho::int_chirho());
        let inner_chirho =
            self.fresh_binder_chirho("show_maybe_inner", placeholder_ty_chirho(inner_key_chirho));
        let shown_inner_chirho = scalar_show_expr_chirho(
            inner_key_chirho,
            CoreExprChirho::VarChirho(inner_chirho.id_chirho),
        )
        .expect("enumerated Maybe Show key should have a scalar renderer");
        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(arg_chirho.id_chirho)),
            bind_chirho: case_binder_chirho,
            result_ty_chirho: TyChirho::string_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                    binders_chirho: Vec::new(),
                    rhs_chirho: string_lit_chirho("Nothing"),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                    binders_chirho: vec![inner_chirho],
                    rhs_chirho: append_strings_chirho(
                        string_lit_chirho("Just "),
                        shown_inner_chirho,
                    ),
                },
            ],
        };
        self.push_show_binding_chirho(prim_name_chirho, arg_chirho, body_chirho);
    }

    pub(super) fn generate_show_tuple_binding_chirho(
        &mut self,
        prim_name_chirho: &str,
        type_key_chirho: &str,
    ) {
        let inner_keys_chirho = type_key_chirho
            .strip_prefix('(')
            .and_then(|key_chirho| key_chirho.strip_suffix(')'))
            .expect("tuple Show key should be parenthesized")
            .split(',')
            .collect::<Vec<_>>();
        let arg_chirho = self.fresh_binder_chirho("show_tuple_arg", TyChirho::int_chirho());
        let case_binder_chirho =
            self.fresh_binder_chirho("show_tuple_case", TyChirho::int_chirho());
        let field_binders_chirho = inner_keys_chirho
            .iter()
            .enumerate()
            .map(|(index_chirho, key_chirho)| {
                self.fresh_binder_chirho(
                    &format!("show_tuple_field_{index_chirho}"),
                    placeholder_ty_chirho(key_chirho),
                )
            })
            .collect::<Vec<_>>();
        let mut parts_chirho = vec![string_lit_chirho("(")];
        for (index_chirho, (key_chirho, field_chirho)) in inner_keys_chirho
            .iter()
            .zip(&field_binders_chirho)
            .enumerate()
        {
            if index_chirho > 0 {
                parts_chirho.push(string_lit_chirho(","));
            }
            let shown_field_chirho = if *key_chirho == "String" {
                let show_string_id_chirho =
                    self.resolve_or_fresh_id_chirho("$prim_Show_show_[Char]");
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(show_string_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(field_chirho.id_chirho)),
                }
            } else {
                scalar_show_expr_chirho(
                    key_chirho,
                    CoreExprChirho::VarChirho(field_chirho.id_chirho),
                )
                .expect("enumerated tuple Show field should have a scalar renderer")
            };
            parts_chirho.push(shown_field_chirho);
        }
        parts_chirho.push(string_lit_chirho(")"));
        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(arg_chirho.id_chirho)),
            bind_chirho: case_binder_chirho,
            result_ty_chirho: TyChirho::string_chirho(),
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho(format!(
                    "$tuple{}",
                    inner_keys_chirho.len()
                )),
                binders_chirho: field_binders_chirho,
                rhs_chirho: join_string_parts_chirho(parts_chirho),
            }],
        };
        self.push_show_binding_chirho(prim_name_chirho, arg_chirho, body_chirho);
    }

    pub(super) fn generate_show_either_binding_chirho(
        &mut self,
        prim_name_chirho: &str,
        type_key_chirho: &str,
    ) {
        let mut inner_keys_chirho = type_key_chirho
            .strip_prefix("Either ")
            .expect("Either Show key should include its arguments")
            .split_whitespace();
        let left_key_chirho = inner_keys_chirho
            .next()
            .expect("Either Show key should include its left argument");
        let right_key_chirho = inner_keys_chirho
            .next()
            .expect("Either Show key should include its right argument");
        let arg_chirho = self.fresh_binder_chirho("show_either_arg", TyChirho::int_chirho());
        let case_binder_chirho =
            self.fresh_binder_chirho("show_either_case", TyChirho::int_chirho());
        let left_chirho =
            self.fresh_binder_chirho("show_left", placeholder_ty_chirho(left_key_chirho));
        let right_chirho =
            self.fresh_binder_chirho("show_right", placeholder_ty_chirho(right_key_chirho));
        let shown_left_chirho = scalar_show_expr_chirho(
            left_key_chirho,
            CoreExprChirho::VarChirho(left_chirho.id_chirho),
        )
        .expect("enumerated Either left type should have a scalar renderer");
        let shown_right_chirho = scalar_show_expr_chirho(
            right_key_chirho,
            CoreExprChirho::VarChirho(right_chirho.id_chirho),
        )
        .expect("enumerated Either right type should have a scalar renderer");
        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(arg_chirho.id_chirho)),
            bind_chirho: case_binder_chirho,
            result_ty_chirho: TyChirho::string_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("Left".to_string()),
                    binders_chirho: vec![left_chirho],
                    rhs_chirho: append_strings_chirho(
                        string_lit_chirho("Left "),
                        shown_left_chirho,
                    ),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("Right".to_string()),
                    binders_chirho: vec![right_chirho],
                    rhs_chirho: append_strings_chirho(
                        string_lit_chirho("Right "),
                        shown_right_chirho,
                    ),
                },
            ],
        };
        self.push_show_binding_chirho(prim_name_chirho, arg_chirho, body_chirho);
    }
}
