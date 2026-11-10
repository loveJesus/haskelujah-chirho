// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Expression desugaring, moved whole from the desugar root (row 691, D8): the dispatcher
//! over every expression form, and literal translation. The dispatcher is unchanged; its
//! later split by expression family is a separate measured row.

use super::*;

impl DesugarCtxChirho {
    /// Desugar an AST expression into a Core expression.
    pub fn desugar_expr_chirho(&mut self, expr_chirho: &ExprChirho) -> CoreExprChirho {
        let body_chirho = self.desugar_unwrapped_expr_chirho(expr_chirho);
        self.abstract_sigma_expression_chirho(expr_chirho, body_chirho)
    }

    fn desugar_unwrapped_expr_chirho(&mut self, expr_chirho: &ExprChirho) -> CoreExprChirho {
        match expr_chirho {
            ExprChirho::VarChirho(name_chirho) => {
                let (lookup_name_chirho, id_chirho) =
                    self.resolve_written_reference_chirho(name_chirho);
                let id_chirho = self.reference_occurrence_chirho(
                    &lookup_name_chirho,
                    id_chirho,
                    name_chirho.span_chirho(),
                );
                self.record_occurrence_provenance_chirho(
                    id_chirho,
                    name_chirho.origin_chirho(),
                    OccurrenceRoleChirho::Reference,
                );
                CoreExprChirho::VarChirho(id_chirho)
            }
            ExprChirho::ConChirho(name_chirho) => {
                let con_text_chirho = name_chirho.text_chirho();
                // Bidirectional pattern synonyms can be used as expressions.
                if let Some(def_chirho) = self.pat_syns_chirho.get(con_text_chirho).cloned()
                    && matches!(def_chirho.dir_chirho, PatSynDirChirho::ImplBidirChirho)
                {
                    // Build: \arg0 arg1 ... -> <expansion_as_expr>
                    let mut param_binders_chirho = Vec::new();
                    let mut subst_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
                    for formal_chirho in &def_chirho.args_chirho {
                        let binder_chirho = self.fresh_binder_chirho(
                            formal_chirho,
                            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        subst_chirho.insert(formal_chirho.clone(), binder_chirho.id_chirho);
                        param_binders_chirho.push(binder_chirho);
                    }
                    let body_chirho =
                        self.pat_to_builder_expr_chirho(&def_chirho.pat_chirho, &subst_chirho);
                    // Wrap body in lambdas (right to left).
                    let mut result_chirho = body_chirho;
                    for binder_chirho in param_binders_chirho.into_iter().rev() {
                        result_chirho = CoreExprChirho::LamChirho {
                            binder_chirho,
                            body_chirho: Box::new(result_chirho),
                        };
                    }
                    return result_chirho;
                }
                CoreExprChirho::ConAppChirho {
                    con_name_chirho: con_text_chirho.to_string(),
                    args_chirho: vec![],
                }
            }

            ExprChirho::LitChirho(lit_chirho) => {
                let core_lit_chirho = self.desugar_lit_chirho(lit_chirho);
                // Numeric literal overloading (Haskell 2010 §3.2):
                // Integer literals become `fromInteger <lit>`, allowing
                // the dict pass to select the correct Num instance. That
                // `fromInteger` is Num's method, never a binding of the
                // program's own (row 660).
                if matches!(core_lit_chirho, CoreLitChirho::IntChirho(_)) {
                    let from_integer_id_chirho = self.resolve_syntax_method_chirho("fromInteger");
                    self.record_literal_occurrence_chirho(from_integer_id_chirho, lit_chirho);
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(from_integer_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::LitChirho(core_lit_chirho)),
                    }
                } else if matches!(core_lit_chirho, CoreLitChirho::StringChirho(_))
                    && self
                        .extensions_chirho
                        .contains(&"OverloadedStrings".to_string())
                {
                    // OverloadedStrings: string literals become `fromString "lit"`,
                    // IsString's method, never a binding of the program's own (row 660).
                    let from_string_id_chirho = self.resolve_syntax_method_chirho("fromString");
                    self.record_literal_occurrence_chirho(from_string_id_chirho, lit_chirho);
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(from_string_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::LitChirho(core_lit_chirho)),
                    }
                } else {
                    CoreExprChirho::LitChirho(core_lit_chirho)
                }
            }

            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                let desugared_fun_chirho = self.desugar_expr_chirho(fun_chirho);
                let desugared_arg_chirho = self.desugar_expr_chirho(arg_chirho);
                // If applying to a constructor, accumulate into ConAppChirho
                match desugared_fun_chirho {
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho,
                        mut args_chirho,
                    } => {
                        args_chirho.push(desugared_arg_chirho);
                        self.enforce_strict_fields_chirho(con_name_chirho, args_chirho)
                    }
                    // Detect prefix application of binary primops:
                    // App(App(Var("div"), a), b) → PrimOp("div#", [a, b])
                    CoreExprChirho::AppChirho {
                        fun_chirho: ref inner_fun_chirho,
                        arg_chirho: ref first_arg_chirho,
                    } => {
                        // A `div` this module owns keeps its own binding (row 614).
                        if let CoreExprChirho::VarChirho(id_chirho) = inner_fun_chirho.as_ref()
                            && let Some(name_chirho) = self.names_chirho.get(id_chirho)
                            && !self.is_owned_binding_id_chirho(name_chirho, *id_chirho)
                            && let Some(primop_chirho) =
                                prefix_binary_primop_name_chirho(name_chirho)
                        {
                            return CoreExprChirho::PrimOpChirho {
                                name_chirho: primop_chirho.to_string(),
                                args_chirho: vec![
                                    first_arg_chirho.as_ref().clone(),
                                    desugared_arg_chirho,
                                ],
                            };
                        }
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(desugared_fun_chirho),
                            arg_chirho: Box::new(desugared_arg_chirho),
                        }
                    }
                    _ => CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(desugared_fun_chirho),
                        arg_chirho: Box::new(desugared_arg_chirho),
                    },
                }
            }

            ExprChirho::LamChirho {
                pats_chirho,
                body_chirho,
                ..
            } => {
                if let Some(lazy_tuple_chirho) =
                    self.try_desugar_lazy_tuple_lambda_chirho(pats_chirho, body_chirho)
                {
                    return lazy_tuple_chirho;
                }

                // A lambda is one row over its parameters: each pattern is
                // matched left to right, and a mismatch is GHC's lambda failure.
                // workflow: language-features-chirho/pattern-matching-chirho
                let param_binders_chirho: Vec<BinderChirho> = pats_chirho
                    .iter()
                    .enumerate()
                    .map(|(i_chirho, pat_chirho)| {
                        let name_chirho = extract_var_name_from_pat_chirho(pat_chirho)
                            .unwrap_or_else(|| format!("_lam{i_chirho}"));
                        self.row_binder_chirho(&name_chirho)
                    })
                    .collect();
                let param_ids_chirho: Vec<CoreIdChirho> = param_binders_chirho
                    .iter()
                    .map(|binder_chirho| binder_chirho.id_chirho)
                    .collect();
                let row_chirho = MatchRowChirho {
                    pats_chirho: pats_chirho.iter().collect(),
                    body_chirho: RowBodyChirho::Expr(body_chirho),
                    where_binds_chirho: &[],
                };
                let failure_chirho = self.case_failure_chirho("Non-exhaustive patterns in lambda");
                let mut result_chirho =
                    self.compile_rows_chirho(&param_ids_chirho, &[row_chirho], failure_chirho);
                for binder_chirho in param_binders_chirho.into_iter().rev() {
                    result_chirho = CoreExprChirho::LamChirho {
                        binder_chirho,
                        body_chirho: Box::new(result_chirho),
                    };
                }
                result_chirho
            }

            // workflow: language-features-chirho/rank-n-visible-type-application-chirho
            ExprChirho::TypeLamChirho {
                binder_chirho,
                body_chirho,
                ..
            } => CoreExprChirho::TyLamChirho {
                ty_var_chirho: binder_chirho.text_chirho().to_string(),
                body_chirho: Box::new(self.desugar_expr_chirho(body_chirho)),
            },

            // A multi-way if is guarded arms with nowhere to fall to.
            // workflow: language-features-chirho/pattern-matching-chirho
            ExprChirho::MultiWayIfChirho { arms_chirho, .. } => {
                let failure_chirho =
                    self.case_failure_chirho("Non-exhaustive guards in multi-way if");
                self.desugar_guards_chirho(arms_chirho, Some(&failure_chirho))
            }

            ExprChirho::IfChirho {
                cond_chirho,
                then_chirho,
                else_chirho,
                ..
            } => {
                // if c then t else e → case c of { True -> t; False -> e }
                let cond_core_chirho = self.desugar_expr_chirho(cond_chirho);
                let then_core_chirho = self.desugar_expr_chirho(then_chirho);
                let else_core_chirho = self.desugar_expr_chirho(else_chirho);
                let wild_chirho = self.fresh_binder_chirho(
                    "wild",
                    TyChirho::bool_chirho(),
                    SpanChirho::DUMMY_CHIRHO,
                );
                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(cond_core_chirho),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: then_core_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("False".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: else_core_chirho,
                        },
                    ],
                }
            }

            ExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => self.desugar_let_around_chirho(binds_chirho, |ctx_chirho| {
                ctx_chirho.desugar_expr_chirho(body_chirho)
            }),

            ExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                let scrut_chirho = self.desugar_expr_chirho(scrutinee_chirho);
                self.desugar_case_expr_with_scrutinee_chirho(scrut_chirho, alts_chirho)
            }

            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => {
                if elements_chirho.is_empty() {
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: "$tuple0".to_string(),
                        args_chirho: vec![],
                    }
                } else {
                    let arity_chirho = elements_chirho.len();
                    let con_args_chirho: Vec<CoreExprChirho> = elements_chirho
                        .iter()
                        .map(|elem_chirho| self.desugar_expr_chirho(elem_chirho))
                        .collect();
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: format!("$tuple{}", arity_chirho),
                        args_chirho: con_args_chirho,
                    }
                }
            }

            ExprChirho::ListChirho {
                elements_chirho, ..
            } => {
                // Desugar [a, b, c] → a : b : c : []
                let mut result_chirho = CoreExprChirho::ConAppChirho {
                    con_name_chirho: "[]".to_string(),
                    args_chirho: vec![],
                };
                for elem_chirho in elements_chirho.iter().rev() {
                    let elem_core_chirho = self.desugar_expr_chirho(elem_chirho);
                    result_chirho = CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![elem_core_chirho, result_chirho],
                    };
                }
                // OverloadedLists: list literals become `fromList [a, b, c]`,
                // IsList's method, never a binding of the program's own (row 660).
                if self
                    .extensions_chirho
                    .contains(&"OverloadedLists".to_string())
                {
                    let from_list_id_chirho = self.resolve_syntax_method_chirho("fromList");
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(from_list_id_chirho)),
                        arg_chirho: Box::new(result_chirho),
                    }
                } else {
                    result_chirho
                }
            }

            ExprChirho::NegChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                // Parser row 592 preserves the direct source magnitude `2^63` as
                // `Neg(Lit(i64::MIN))`: keep its `fromInteger` evidence, but do not
                // execute `negate#` on the already-negative machine-word marker.
                // workflow: compiler-pipeline-chirho/operator-fixity-chirho
                if matches!(
                    inner_chirho.as_ref(),
                    ExprChirho::LitChirho(LitChirho::IntChirho(i64::MIN, ..))
                ) {
                    return self.desugar_expr_chirho(inner_chirho);
                }
                // -x → negate# x (Int) or negateFloat# x (Double)
                let desugared_chirho = self.desugar_expr_chirho(inner_chirho);
                let primop_chirho = if is_float_core_chirho(&desugared_chirho) {
                    "negateFloat#"
                } else {
                    "negate#"
                };
                CoreExprChirho::PrimOpChirho {
                    name_chirho: primop_chirho.to_string(),
                    args_chirho: vec![desugared_chirho],
                }
            }

            ExprChirho::InfixChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                span_chirho,
            } => {
                let op_name_chirho = op_chirho.text_chirho();
                let left_core_chirho = self.desugar_expr_chirho(left_chirho);
                let right_core_chirho = self.desugar_expr_chirho(right_chirho);

                // An operator this module owns, at top level or locally, takes its own
                // binding ahead of every Prelude shortcut below (row 614).
                if self.names_owned_binding_chirho(op_chirho) {
                    return self.written_operator_application_chirho(
                        op_chirho,
                        left_core_chirho,
                        right_core_chirho,
                    );
                }

                // ($!) is strict application: f $! x = seq x (f x)
                // Desugars to: case x of { _ -> f x }
                if op_name_chirho == "$!" {
                    let wild_chirho = self.fresh_binder_chirho(
                        "_wild",
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    return CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(right_core_chirho.clone()),
                        bind_chirho: wild_chirho,
                        result_ty_chirho: TyChirho::VarChirho(
                            haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho + 1,
                            ),
                        ),
                        alts_chirho: vec![crate::expr_chirho::CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(left_core_chirho),
                                arg_chirho: Box::new(right_core_chirho),
                            },
                        }],
                    };
                }

                // ($!!) is deep strict application: f $!! x = deepseq x (f x)
                // Desugars to: case x of { _ -> f x }  (same as $! at Core level,
                // since full NF evaluation requires NFData dict which the dict pass handles)
                if op_name_chirho == "$!!" {
                    let wild_chirho = self.fresh_binder_chirho(
                        "_wild",
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    return CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(right_core_chirho.clone()),
                        bind_chirho: wild_chirho,
                        result_ty_chirho: TyChirho::VarChirho(
                            haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho + 1,
                            ),
                        ),
                        alts_chirho: vec![crate::expr_chirho::CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(left_core_chirho),
                                arg_chirho: Box::new(right_core_chirho),
                            },
                        }],
                    };
                }

                // (.) is function composition: (f . g) x = f (g x)
                // At the expression level, f . g desugars to \x -> f (g x)
                if op_name_chirho == "." {
                    let x_id_chirho = self.fresh_id_chirho("x");
                    let x_binder_chirho = BinderChirho {
                        id_chirho: x_id_chirho,
                        name_chirho: "x".to_string(),
                        ty_chirho: TyChirho::VarChirho(
                            haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                        ),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    };
                    return CoreExprChirho::LamChirho {
                        binder_chirho: x_binder_chirho,
                        body_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(left_core_chirho),
                            arg_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(right_core_chirho),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_id_chirho)),
                            }),
                        }),
                    };
                }

                // (>>) is monadic then: a >> b → (>>) a b as a Var app so the
                // dict pass dispatches it per-type ($prim_Monad_>>_<Head>);
                // unresolved names still lower to ThenIOChirho at STG (INV-001).
                // workflow: monadic-dispatch-chirho
                if op_name_chirho == ">>" {
                    let then_id_chirho = self.resolve_var_chirho(">>");
                    self.record_occurrence_provenance_chirho(
                        then_id_chirho,
                        op_chirho.origin_chirho(),
                        OccurrenceRoleChirho::Reference,
                    );
                    return CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(then_id_chirho)),
                            arg_chirho: Box::new(left_core_chirho),
                        }),
                        arg_chirho: Box::new(right_core_chirho),
                    };
                }

                // (>>=) is monadic bind: a >>= f → (>>=) a f as a Var app;
                // same dispatch/fallback contract as (>>) above.
                // workflow: monadic-dispatch-chirho
                if op_name_chirho == ">>=" {
                    let bind_id_chirho = self.resolve_var_chirho(">>=");
                    self.record_occurrence_provenance_chirho(
                        bind_id_chirho,
                        op_chirho.origin_chirho(),
                        OccurrenceRoleChirho::Reference,
                    );
                    return CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(bind_id_chirho)),
                            arg_chirho: Box::new(left_core_chirho),
                        }),
                        arg_chirho: Box::new(right_core_chirho),
                    };
                }

                // Constructor operators build data; registered names also cover backtick constructors.
                if op_name_chirho.starts_with(':')
                    || self.con_arities_chirho.contains_key(op_name_chirho)
                {
                    return self.enforce_strict_fields_chirho(
                        op_name_chirho.to_string(),
                        vec![left_core_chirho, right_core_chirho],
                    );
                }

                // (&&) short-circuit: a && b → case a of { True → b; False → False }
                if op_name_chirho == "&&" {
                    let wild_chirho =
                        self.fresh_binder_chirho("$wild", TyChirho::bool_chirho(), *span_chirho);
                    return CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(left_core_chirho),
                        bind_chirho: wild_chirho,
                        result_ty_chirho: TyChirho::bool_chirho(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("True".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: right_core_chirho,
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("False".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "False".to_string(),
                                    args_chirho: vec![],
                                },
                            },
                        ],
                    };
                }

                // (||) short-circuit: a || b → case a of { True → True; False → b }
                if op_name_chirho == "||" {
                    let wild_chirho =
                        self.fresh_binder_chirho("$wild", TyChirho::bool_chirho(), *span_chirho);
                    return CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(left_core_chirho),
                        bind_chirho: wild_chirho,
                        result_ty_chirho: TyChirho::bool_chirho(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("True".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "True".to_string(),
                                    args_chirho: vec![],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("False".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: right_core_chirho,
                            },
                        ],
                    };
                }

                // (**) is always floating-point exponentiation
                if op_name_chirho == "**" {
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: "**#".to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // (^) is integer exponentiation, or GHC.Real's repeated squaring at
                // Double (`^.#`, row 570) when the base is a float literal; never `**#`'s
                // `pow`, whose last bits differ. Any other Double base takes `^.#` in the
                // dictionary pass once its key is known.
                // workflow: language-features-chirho/dictionary-evidence-chirho
                if op_name_chirho == "^" {
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: if is_float_core_chirho(&left_core_chirho)
                            || is_float_core_chirho(&right_core_chirho)
                        {
                            "^.#"
                        } else {
                            "^#"
                        }
                        .to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // Float-aware Num dispatch: when at least one operand is
                // a float literal, use the Double primop instead of the
                // default Int one.  This covers `1.5 + 2.5` etc.
                if matches!(op_name_chirho, "+" | "-" | "*" | "/")
                    && (is_float_core_chirho(&left_core_chirho)
                        || is_float_core_chirho(&right_core_chirho))
                {
                    let float_primop_chirho = match op_name_chirho {
                        "+" => "+.#",
                        "-" => "-.#",
                        "*" => "*.#",
                        "/" => "/.#",
                        _ => unreachable!(),
                    };
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: float_primop_chirho.to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // Check if this is a known primitive operator, unless the written
                // operator takes its occurrence's evidence (row 610).
                if !self.takes_operator_evidence_chirho(op_chirho)
                    && let Some(primop_name_chirho) = builtin_primop_name_chirho(op_name_chirho)
                {
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: primop_name_chirho.to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // Otherwise: a `op` b → (op a) b
                self.written_operator_application_chirho(
                    op_chirho,
                    left_core_chirho,
                    right_core_chirho,
                )
            }

            ExprChirho::LeftSectionChirho {
                op_chirho,
                arg_chirho,
                ..
            } => {
                // (op e) → \y -> y `op` e → \y -> op y e
                // For primop operators, emit PrimOp in the lambda body
                // so they work without dict-pass bindings.
                let arg_core_chirho = self.desugar_expr_chirho(arg_chirho);
                let y_binder_chirho = self.fresh_binder_chirho(
                    "_sec",
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let y_id_chirho = y_binder_chirho.id_chirho;
                let body_chirho = self.build_section_body_chirho(
                    op_chirho,
                    CoreExprChirho::VarChirho(y_id_chirho),
                    arg_core_chirho,
                );
                CoreExprChirho::LamChirho {
                    binder_chirho: y_binder_chirho,
                    body_chirho: Box::new(body_chirho),
                }
            }

            ExprChirho::RightSectionChirho {
                arg_chirho,
                op_chirho,
                ..
            } => {
                // (e op) → \y -> e `op` y → \y -> op e y
                let arg_core_chirho = self.desugar_expr_chirho(arg_chirho);
                let y_binder_chirho = self.fresh_binder_chirho(
                    "_sec",
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let y_id_chirho = y_binder_chirho.id_chirho;
                let body_chirho = self.build_section_body_chirho(
                    op_chirho,
                    arg_core_chirho,
                    CoreExprChirho::VarChirho(y_id_chirho),
                );
                CoreExprChirho::LamChirho {
                    binder_chirho: y_binder_chirho,
                    body_chirho: Box::new(body_chirho),
                }
            }

            ExprChirho::DoChirho {
                qualifier_chirho,
                stmts_chirho,
                ..
            } => {
                // Desugar do notation:
                //   do { e }              → e
                //   do { e; stmts }       → e >> do { stmts }
                //   do { p <- e; stmts }  → e >>= \p -> do { stmts }
                //   do { let binds; stmts } → let binds in do { stmts }
                self.desugar_do_chirho(stmts_chirho, qualifier_chirho.as_deref())
            }

            ExprChirho::ArithSeqChirho {
                from_chirho,
                then_chirho,
                to_chirho,
                span_chirho,
            } => {
                // Desugar to calls to lazy Prelude functions (not primops)
                // so infinite lists like [1..] are properly lazy.
                //
                // [from ..]       → enumFrom from
                // [from,then ..]  → enumFromThen from then
                // [from .. to]    → enumFromTo from to
                // [from,then..to] → enumFromThenTo from then to
                let from_core_chirho = self.desugar_expr_chirho(from_chirho);
                match (then_chirho, to_chirho) {
                    (None, None) => {
                        let fun_id_chirho =
                            self.resolve_range_method_chirho("enumFrom", *span_chirho);
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_id_chirho)),
                            arg_chirho: Box::new(from_core_chirho),
                        }
                    }
                    (Some(then_e_chirho), None) => {
                        let then_core_chirho = self.desugar_expr_chirho(then_e_chirho);
                        let fun_id_chirho =
                            self.resolve_range_method_chirho("enumFromThen", *span_chirho);
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_id_chirho)),
                                arg_chirho: Box::new(from_core_chirho),
                            }),
                            arg_chirho: Box::new(then_core_chirho),
                        }
                    }
                    (None, Some(to_e_chirho)) => {
                        let to_core_chirho = self.desugar_expr_chirho(to_e_chirho);
                        let fun_id_chirho =
                            self.resolve_range_method_chirho("enumFromTo", *span_chirho);
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_id_chirho)),
                                arg_chirho: Box::new(from_core_chirho),
                            }),
                            arg_chirho: Box::new(to_core_chirho),
                        }
                    }
                    (Some(then_e_chirho), Some(to_e_chirho)) => {
                        let then_core_chirho = self.desugar_expr_chirho(then_e_chirho);
                        let to_core_chirho = self.desugar_expr_chirho(to_e_chirho);
                        let fun_id_chirho =
                            self.resolve_range_method_chirho("enumFromThenTo", *span_chirho);
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_id_chirho)),
                                    arg_chirho: Box::new(from_core_chirho),
                                }),
                                arg_chirho: Box::new(then_core_chirho),
                            }),
                            arg_chirho: Box::new(to_core_chirho),
                        }
                    }
                }
            }

            ExprChirho::ParenChirho { inner_chirho, .. } => self.desugar_expr_chirho(inner_chirho),

            ExprChirho::AnnChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
                ..
            } => {
                let inner_core_chirho = self.desugar_expr_chirho(inner_chirho);
                let type_key_chirho = Self::type_key_from_ast_chirho(
                    ty_chirho,
                    self.promotion_scope_chirho.as_deref(),
                );
                if type_key_chirho == "_" {
                    inner_core_chirho
                } else {
                    CoreExprChirho::TyAppChirho {
                        expr_chirho: Box::new(inner_core_chirho),
                        ty_chirho: TyChirho::ConChirho(type_key_chirho),
                    }
                }
            }

            ExprChirho::ListCompChirho {
                body_chirho,
                quals_chirho,
                parallel_quals_chirho,
                span_chirho,
            } => {
                if parallel_quals_chirho.is_empty() {
                    self.desugar_list_comp_chirho(body_chirho, quals_chirho)
                } else {
                    self.desugar_parallel_list_comp_chirho(
                        body_chirho,
                        parallel_quals_chirho,
                        *span_chirho,
                    )
                }
            }

            ExprChirho::RecordConChirho {
                con_chirho,
                fields_chirho,
                has_wildcard_chirho,
                ..
            } => {
                // Con { f1 = e1, f2 = e2 } → ConApp "Con" [e1, e2]
                // RecordWildCards: Con { f1 = e1, .. } → fills missing fields from scope
                let con_name_chirho = con_chirho.text_chirho().to_string();
                let field_key_chirho = con_chirho.full_name_chirho();
                let con_args_chirho: Vec<CoreExprChirho> = if *has_wildcard_chirho {
                    self.expand_record_wildcard_expr_chirho(&field_key_chirho, fields_chirho)
                } else {
                    self.record_construction_args_chirho(&field_key_chirho, fields_chirho)
                };
                self.enforce_strict_fields_chirho(con_name_chirho, con_args_chirho)
            }

            ExprChirho::RecordUpdateChirho {
                expr_chirho: base_chirho,
                fields_chirho,
                ..
            } => {
                // record { f1 = e1 } → setField_f1 e1 record
                // Simplified: for now, produce a chain of setter applications.
                // A real implementation needs the data type layout.
                let base_core_chirho = self.desugar_expr_chirho(base_chirho);
                let mut result_chirho = base_core_chirho;
                for field_chirho in fields_chirho {
                    let setter_name_chirho =
                        format!("$setField_{}", field_chirho.name_chirho.text_chirho());
                    let setter_id_chirho = self.resolve_var_chirho(&setter_name_chirho);
                    let val_chirho = self.desugar_expr_chirho(&field_chirho.value_chirho);
                    result_chirho = CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(setter_id_chirho)),
                            arg_chirho: Box::new(val_chirho),
                        }),
                        arg_chirho: Box::new(result_chirho),
                    };
                }
                result_chirho
            }

            // Type application — erased during desugaring, just desugar the inner expr.
            ExprChirho::TypeAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.desugar_expr_chirho(inner_chirho),

            // TH splice/quote expressions — should have been evaluated before desugaring.
            // Return an error literal instead of panicking so the pipeline doesn't crash.
            ExprChirho::SpliceChirho { .. } | ExprChirho::TypedSpliceChirho { .. } => {
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
            }
            ExprChirho::QuoteExprChirho { .. }
            | ExprChirho::QuoteDeclChirho { .. }
            | ExprChirho::QuoteTypeChirho { .. }
            | ExprChirho::QuotePatChirho { .. } => {
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
            }
        }
    }

    pub(super) fn desugar_lit_chirho(&self, lit_chirho: &LitChirho) -> CoreLitChirho {
        match lit_chirho {
            LitChirho::IntChirho(v_chirho, _, _) => CoreLitChirho::IntChirho(*v_chirho),
            LitChirho::FloatChirho(v_chirho, _, _) => CoreLitChirho::FloatChirho(*v_chirho),
            LitChirho::CharChirho(v_chirho, _, _) => CoreLitChirho::CharChirho(*v_chirho),
            LitChirho::StringChirho(v_chirho, _, _) => {
                CoreLitChirho::StringChirho(v_chirho.clone())
            }
        }
    }
}
