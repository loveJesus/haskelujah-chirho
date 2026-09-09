// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Concrete Show instances in portable Core. Constructor arguments are rendered
//! at precedence 11, not by inspecting the spaces in their printed text.
//! Workflow: spec-chirho/workflows-chirho/print-show-evidence-chirho.md.

use super::DictPassCtxChirho;
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreLitChirho,
    InlineAnnotationChirho,
};
use haskelujah_typing_chirho::ty_chirho::TyChirho;
use std::collections::BTreeSet;

mod lists_chirho;
mod shape_chirho;
use shape_chirho::ShowShapeChirho;

fn string_lit_chirho(value_chirho: &str) -> CoreExprChirho {
    CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(value_chirho.to_string()))
}

fn join_string_parts_chirho(parts_chirho: Vec<CoreExprChirho>) -> CoreExprChirho {
    parts_chirho
        .into_iter()
        .rev()
        .fold(string_lit_chirho(""), |tail_chirho, part_chirho| {
            CoreExprChirho::PrimOpChirho {
                name_chirho: "++#".to_string(),
                args_chirho: vec![part_chirho, tail_chirho],
            }
        })
}

fn parenthesize_chirho(body_chirho: CoreExprChirho, required_chirho: bool) -> CoreExprChirho {
    if required_chirho {
        join_string_parts_chirho(vec![
            string_lit_chirho("("),
            body_chirho,
            string_lit_chirho(")"),
        ])
    } else {
        body_chirho
    }
}

impl DictPassCtxChirho {
    /// Generate only instances with a complete, understood evidence shape. An
    /// unsupported shape keeps its dictionary; no leaf is defaulted to Int.
    pub(super) fn generate_evidenced_show_bindings_chirho(&mut self) {
        let keys_chirho = self
            .occurrence_evidence_chirho
            .values()
            .filter_map(|(class_chirho, key_chirho)| {
                (class_chirho == "Show").then_some(key_chirho.clone())
            })
            .collect::<BTreeSet<_>>();
        for key_chirho in keys_chirho {
            let name_chirho = format!("$prim_Show_show_{key_chirho}");
            if self
                .lookup_body_backed_name_id_chirho(&name_chirho)
                .is_none()
            {
                if let Some(shape_chirho) = self.parse_show_shape_chirho(&key_chirho) {
                    self.generate_show_shape_binding_chirho(&name_chirho, &shape_chirho);
                }
            }
        }
    }

    fn generate_show_shape_binding_chirho(
        &mut self,
        name_chirho: &str,
        shape_chirho: &ShowShapeChirho,
    ) {
        let arg_chirho = self.fresh_binder_chirho("show_arg_chirho", TyChirho::int_chirho());
        let body_chirho = self.show_shape_expr_chirho(
            shape_chirho,
            CoreExprChirho::VarChirho(arg_chirho.id_chirho),
            0,
        );
        let binder_chirho = BinderChirho {
            id_chirho: self.resolve_or_fresh_id_chirho(name_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::fun_chirho(
                arg_chirho.ty_chirho.clone(),
                TyChirho::string_chirho(),
            ),
            span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
        };
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: arg_chirho,
                body_chirho: Box::new(body_chirho),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }

    fn show_shape_expr_chirho(
        &mut self,
        shape_chirho: &ShowShapeChirho,
        value_chirho: CoreExprChirho,
        precedence_chirho: u8,
    ) -> CoreExprChirho {
        match shape_chirho {
            ShowShapeChirho::ScalarChirho(key_chirho) => {
                self.show_scalar_expr_chirho(key_chirho, value_chirho, precedence_chirho)
            }
            ShowShapeChirho::BackedChirho(function_chirho) => CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(*function_chirho)),
                arg_chirho: Box::new(value_chirho),
            },
            ShowShapeChirho::ListChirho(element_chirho) => {
                self.show_list_expr_chirho(element_chirho, value_chirho)
            }
            ShowShapeChirho::MaybeChirho(inner_chirho) => {
                let field_chirho =
                    self.fresh_binder_chirho("show_just_field_chirho", TyChirho::int_chirho());
                let shown_chirho = self.show_shape_expr_chirho(
                    inner_chirho,
                    CoreExprChirho::VarChirho(field_chirho.id_chirho),
                    11,
                );
                let body_chirho = parenthesize_chirho(
                    join_string_parts_chirho(vec![string_lit_chirho("Just "), shown_chirho]),
                    precedence_chirho > 10,
                );
                self.show_case_chirho(
                    value_chirho,
                    vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: string_lit_chirho("Nothing"),
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                            binders_chirho: vec![field_chirho],
                            rhs_chirho: body_chirho,
                        },
                    ],
                )
            }
            ShowShapeChirho::EitherChirho(left_chirho, right_chirho) => {
                let mut alts_chirho = Vec::new();
                for (constructor_chirho, field_shape_chirho) in
                    [("Left", left_chirho), ("Right", right_chirho)]
                {
                    let field_chirho = self
                        .fresh_binder_chirho("show_either_field_chirho", TyChirho::int_chirho());
                    let shown_chirho = self.show_shape_expr_chirho(
                        field_shape_chirho,
                        CoreExprChirho::VarChirho(field_chirho.id_chirho),
                        11,
                    );
                    alts_chirho.push(CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(constructor_chirho.to_string()),
                        binders_chirho: vec![field_chirho],
                        rhs_chirho: parenthesize_chirho(
                            join_string_parts_chirho(vec![
                                string_lit_chirho(&format!("{constructor_chirho} ")),
                                shown_chirho,
                            ]),
                            precedence_chirho > 10,
                        ),
                    });
                }
                self.show_case_chirho(value_chirho, alts_chirho)
            }
            ShowShapeChirho::TupleChirho(fields_chirho) => {
                let mut binders_chirho = Vec::new();
                let mut parts_chirho = vec![string_lit_chirho("(")];
                for (index_chirho, shape_chirho) in fields_chirho.iter().enumerate() {
                    if index_chirho != 0 {
                        parts_chirho.push(string_lit_chirho(","));
                    }
                    let binder_chirho =
                        self.fresh_binder_chirho("show_tuple_field_chirho", TyChirho::int_chirho());
                    parts_chirho.push(self.show_shape_expr_chirho(
                        shape_chirho,
                        CoreExprChirho::VarChirho(binder_chirho.id_chirho),
                        0,
                    ));
                    binders_chirho.push(binder_chirho);
                }
                parts_chirho.push(string_lit_chirho(")"));
                self.show_case_chirho(
                    value_chirho,
                    vec![CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(format!(
                            "$tuple{}",
                            fields_chirho.len()
                        )),
                        binders_chirho,
                        rhs_chirho: join_string_parts_chirho(parts_chirho),
                    }],
                )
            }
        }
    }

    fn show_case_chirho(
        &mut self,
        value_chirho: CoreExprChirho,
        alts_chirho: Vec<CoreAltChirho>,
    ) -> CoreExprChirho {
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(value_chirho),
            bind_chirho: self.fresh_binder_chirho("show_case_chirho", TyChirho::int_chirho()),
            result_ty_chirho: TyChirho::string_chirho(),
            alts_chirho,
        }
    }

    fn show_scalar_expr_chirho(
        &mut self,
        key_chirho: &str,
        value_chirho: CoreExprChirho,
        precedence_chirho: u8,
    ) -> CoreExprChirho {
        if matches!(key_chirho, "String" | "[Char]") {
            return join_string_parts_chirho(vec![
                string_lit_chirho("\""),
                value_chirho,
                string_lit_chirho("\""),
            ]);
        }
        let primop_chirho = match key_chirho {
            "Int" | "Integer" => "showInt#",
            "Bool" => "showBool#",
            "Char" => "showChar#",
            "Double" => "showFloat#",
            _ => unreachable!("ShowShape only admits implemented scalar instances"),
        };
        let shown_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: primop_chirho.to_string(),
            args_chirho: vec![value_chirho.clone()],
        };
        if precedence_chirho <= 6 || !matches!(key_chirho, "Int" | "Integer" | "Double") {
            return shown_chirho;
        }
        let (compare_chirho, zero_chirho) = if key_chirho == "Double" {
            ("<.#", CoreLitChirho::FloatChirho(0.0))
        } else {
            ("<#", CoreLitChirho::IntChirho(0))
        };
        self.show_case_chirho(
            CoreExprChirho::PrimOpChirho {
                name_chirho: compare_chirho.to_string(),
                args_chirho: vec![value_chirho, CoreExprChirho::LitChirho(zero_chirho)],
            },
            vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: parenthesize_chirho(shown_chirho.clone(), true),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("False".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: shown_chirho,
                },
            ],
        )
    }

    pub(super) fn generate_show_string_binding_chirho(&mut self, name_chirho: &str) {
        self.generate_show_shape_binding_chirho(
            name_chirho,
            &ShowShapeChirho::ScalarChirho("String".to_string()),
        );
    }

    fn parse_show_shape_chirho(&self, key_chirho: &str) -> Option<ShowShapeChirho> {
        ShowShapeChirho::parse_chirho(key_chirho, &|key_chirho| {
            self.lookup_dispatch_body_name_id_chirho(&format!("$prim_Show_show_{key_chirho}"))
        })
    }

    pub(super) fn generate_show_key_binding_chirho(&mut self, name_chirho: &str, key_chirho: &str) {
        if self
            .lookup_body_backed_name_id_chirho(name_chirho)
            .is_some()
        {
            return;
        }
        let shape_chirho = self
            .parse_show_shape_chirho(key_chirho)
            .expect("bootstrap Show key must have a complete renderer");
        self.generate_show_shape_binding_chirho(name_chirho, &shape_chirho);
    }

    pub(super) fn generate_show_maybe_binding_chirho(
        &mut self,
        name_chirho: &str,
        key_chirho: &str,
    ) {
        self.generate_show_key_binding_chirho(name_chirho, key_chirho);
    }

    pub(super) fn generate_show_tuple_binding_chirho(
        &mut self,
        name_chirho: &str,
        key_chirho: &str,
    ) {
        self.generate_show_key_binding_chirho(name_chirho, key_chirho);
    }

    pub(super) fn generate_show_either_binding_chirho(
        &mut self,
        name_chirho: &str,
        key_chirho: &str,
    ) {
        self.generate_show_key_binding_chirho(name_chirho, key_chirho);
    }
}
