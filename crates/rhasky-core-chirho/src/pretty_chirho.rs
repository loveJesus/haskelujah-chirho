// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Core pretty-printer
//!
//! Renders Core expressions in a GHC-Core-like textual format for debugging
//! and golden tests.

use std::fmt::Write;

use crate::expr_chirho::{
    CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreModuleChirho,
};

/// Pretty-print a Core module to a string.
pub fn pretty_module_chirho(module_chirho: &CoreModuleChirho) -> String {
    let mut out_chirho = String::new();
    writeln!(out_chirho, "-- module {}", module_chirho.name_chirho).unwrap();
    writeln!(out_chirho).unwrap();

    for binding_chirho in &module_chirho.bindings_chirho {
        pretty_binding_chirho(&mut out_chirho, binding_chirho, 0);
        writeln!(out_chirho).unwrap();
    }

    out_chirho
}

fn pretty_binding_chirho(
    out_chirho: &mut String,
    binding_chirho: &CoreBindingChirho,
    indent_chirho: usize,
) {
    let prefix_chirho = " ".repeat(indent_chirho);
    let rec_tag_chirho = if binding_chirho.is_rec_chirho {
        "Rec "
    } else {
        ""
    };
    writeln!(
        out_chirho,
        "{prefix_chirho}{rec_tag_chirho}{} :: {}",
        binding_chirho.binder_chirho.name_chirho, binding_chirho.binder_chirho.ty_chirho
    )
    .unwrap();
    write!(out_chirho, "{prefix_chirho}{} = ", binding_chirho.binder_chirho.name_chirho).unwrap();
    pretty_expr_chirho(out_chirho, &binding_chirho.rhs_chirho, indent_chirho + 2);
    writeln!(out_chirho).unwrap();
}

fn pretty_expr_chirho(
    out_chirho: &mut String,
    expr_chirho: &CoreExprChirho,
    indent_chirho: usize,
) {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            write!(out_chirho, "{id_chirho}").unwrap();
        }

        CoreExprChirho::LitChirho(lit_chirho) => {
            write!(out_chirho, "{lit_chirho}").unwrap();
        }

        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            write!(out_chirho, "(").unwrap();
            pretty_expr_chirho(out_chirho, fun_chirho, indent_chirho);
            write!(out_chirho, " ").unwrap();
            pretty_expr_chirho(out_chirho, arg_chirho, indent_chirho);
            write!(out_chirho, ")").unwrap();
        }

        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            write!(out_chirho, "\\{binder_chirho} -> ").unwrap();
            pretty_expr_chirho(out_chirho, body_chirho, indent_chirho + 2);
        }

        CoreExprChirho::LetChirho {
            rec_chirho,
            binds_chirho,
            body_chirho,
        } => {
            let kw_chirho = if *rec_chirho { "letrec" } else { "let" };
            let prefix_chirho = " ".repeat(indent_chirho);
            writeln!(out_chirho, "{kw_chirho} {{").unwrap();
            for (binder_chirho, rhs_chirho) in binds_chirho {
                write!(out_chirho, "{prefix_chirho}  {} = ", binder_chirho.name_chirho).unwrap();
                pretty_expr_chirho(out_chirho, rhs_chirho, indent_chirho + 4);
                writeln!(out_chirho).unwrap();
            }
            write!(out_chirho, "{prefix_chirho}}} in ").unwrap();
            pretty_expr_chirho(out_chirho, body_chirho, indent_chirho);
        }

        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            let prefix_chirho = " ".repeat(indent_chirho);
            write!(out_chirho, "case ").unwrap();
            pretty_expr_chirho(out_chirho, scrutinee_chirho, indent_chirho);
            writeln!(out_chirho, " of {} {{", bind_chirho.name_chirho).unwrap();
            for alt_chirho in alts_chirho {
                pretty_alt_chirho(out_chirho, alt_chirho, indent_chirho + 2);
            }
            write!(out_chirho, "{prefix_chirho}}}").unwrap();
        }

        CoreExprChirho::TyLamChirho {
            ty_var_chirho,
            body_chirho,
        } => {
            write!(out_chirho, "/\\{ty_var_chirho} -> ").unwrap();
            pretty_expr_chirho(out_chirho, body_chirho, indent_chirho + 2);
        }

        CoreExprChirho::TyAppChirho {
            expr_chirho: inner_chirho,
            ty_chirho,
        } => {
            write!(out_chirho, "(").unwrap();
            pretty_expr_chirho(out_chirho, inner_chirho, indent_chirho);
            write!(out_chirho, " @{ty_chirho})").unwrap();
        }

        CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } => {
            write!(out_chirho, "({name_chirho}").unwrap();
            for arg_chirho in args_chirho {
                write!(out_chirho, " ").unwrap();
                pretty_expr_chirho(out_chirho, arg_chirho, indent_chirho);
            }
            write!(out_chirho, ")").unwrap();
        }

        CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho,
        } => {
            write!(out_chirho, "{con_name_chirho}").unwrap();
            for arg_chirho in args_chirho {
                write!(out_chirho, " ").unwrap();
                pretty_expr_chirho(out_chirho, arg_chirho, indent_chirho);
            }
        }
    }
}

fn pretty_alt_chirho(
    out_chirho: &mut String,
    alt_chirho: &CoreAltChirho,
    indent_chirho: usize,
) {
    let prefix_chirho = " ".repeat(indent_chirho);
    write!(out_chirho, "{prefix_chirho}{}", alt_chirho.con_chirho).unwrap();
    for binder_chirho in &alt_chirho.binders_chirho {
        write!(out_chirho, " {}", binder_chirho.name_chirho).unwrap();
    }
    write!(out_chirho, " -> ").unwrap();
    pretty_expr_chirho(out_chirho, &alt_chirho.rhs_chirho, indent_chirho + 2);
    writeln!(out_chirho).unwrap();
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::expr_chirho::{AltConChirho, BinderChirho, CoreIdChirho, CoreLitChirho, InlineAnnotationChirho};
    use rhasky_typing_chirho::ty_chirho::TyChirho;

    #[test]
    fn pretty_simple_binding_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: CoreIdChirho(0),
                    name_chirho: "x".to_string(),
                    ty_chirho: TyChirho::int_chirho(),
                    span_chirho: rhasky_span_chirho::SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let output_chirho = pretty_module_chirho(&module_chirho);
        assert!(output_chirho.contains("-- module Test"));
        assert!(output_chirho.contains("x :: Int"));
        assert!(output_chirho.contains("x = 42"));
    }

    #[test]
    fn pretty_lambda_chirho() {
        let lam_chirho = CoreExprChirho::LamChirho {
            binder_chirho: BinderChirho {
                id_chirho: CoreIdChirho(0),
                name_chirho: "x".to_string(),
                ty_chirho: TyChirho::int_chirho(),
                span_chirho: rhasky_span_chirho::SpanChirho::DUMMY_CHIRHO,
            },
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
        };
        let mut out_chirho = String::new();
        pretty_expr_chirho(&mut out_chirho, &lam_chirho, 0);
        assert!(out_chirho.contains("\\(x :: Int) -> v0"));
    }

    #[test]
    fn pretty_case_chirho() {
        let case_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            bind_chirho: BinderChirho {
                id_chirho: CoreIdChirho(99),
                name_chirho: "wild".to_string(),
                ty_chirho: TyChirho::bool_chirho(),
                span_chirho: rhasky_span_chirho::SpanChirho::DUMMY_CHIRHO,
            },
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                },
            ],
        };
        let mut out_chirho = String::new();
        pretty_expr_chirho(&mut out_chirho, &case_chirho, 0);
        assert!(out_chirho.contains("case v0 of wild"));
        assert!(out_chirho.contains("True -> 1"));
        assert!(out_chirho.contains("_ -> 0"));
    }
}
