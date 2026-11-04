// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor strictness and record construction, moved whole from the desugar root
//! (row 691, D2): which constructors force which fields, the forcing wrapper around a
//! strict construction, and the arguments of a record construction, with `..` wildcard
//! expansion and the thunk an omitted field becomes. Constructor metadata is a separate
//! module, `constructor_metadata_chirho`.

use super::*;

impl DesugarCtxChirho {
    /// Populate constructor strictness and field name maps from data declarations.

    /// Force strict constructor arguments to WHNF by wrapping the ConApp
    /// in nested `let s = arg in case s of { _ -> ... }` chains.
    /// Uses `let` to bind the argument, then `case` to force it to WHNF.
    /// The ConApp references the let-bound variable (which is forced by the case).
    /// Returns the ConApp unchanged if no fields are strict or arity doesn't match.
    pub(super) fn enforce_strict_fields_chirho(
        &mut self,
        con_name_chirho: String,
        args_chirho: Vec<CoreExprChirho>,
    ) -> CoreExprChirho {
        let strictness_chirho = self.con_strictness_chirho.get(&con_name_chirho).cloned();
        if let Some(strictness_chirho) = strictness_chirho
            && strictness_chirho.len() == args_chirho.len()
        {
            let mut con_args_chirho = Vec::with_capacity(args_chirho.len());
            // Collect (let_binder, case_binder, arg_expr) for each strict field
            let mut strict_bindings_chirho: Vec<(BinderChirho, BinderChirho, CoreExprChirho)> =
                Vec::new();
            for (arg_chirho, s_chirho) in args_chirho.into_iter().zip(strictness_chirho.iter()) {
                if s_chirho.is_strict_chirho() {
                    let ty_chirho = TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    );
                    let let_binder_chirho = self.fresh_binder_chirho(
                        "_strict",
                        ty_chirho.clone(),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    let case_binder_chirho = self.fresh_binder_chirho(
                        "_strict_eval",
                        ty_chirho,
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    con_args_chirho.push(CoreExprChirho::VarChirho(let_binder_chirho.id_chirho));
                    strict_bindings_chirho.push((
                        let_binder_chirho,
                        case_binder_chirho,
                        arg_chirho,
                    ));
                } else {
                    con_args_chirho.push(arg_chirho);
                }
            }
            // Build: let s0 = arg0 in case s0 of { _ ->
            //         let s1 = arg1 in case s1 of { _ ->
            //           ConApp(name, [s0, s1]) } }
            let mut result_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho: con_args_chirho,
            };
            for (let_binder_chirho, case_binder_chirho, arg_expr_chirho) in
                strict_bindings_chirho.into_iter().rev()
            {
                // case let_var of { _ -> result }  — forces the let binding to WHNF
                result_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                        let_binder_chirho.id_chirho,
                    )),
                    bind_chirho: case_binder_chirho.clone(),
                    result_ty_chirho: case_binder_chirho.ty_chirho.clone(),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: result_chirho,
                    }],
                };
                // let s = arg_expr in ...
                result_chirho = CoreExprChirho::LetChirho {
                    rec_chirho: false,
                    binds_chirho: vec![(let_binder_chirho, arg_expr_chirho)],
                    body_chirho: Box::new(result_chirho),
                };
            }
            return result_chirho;
        }
        CoreExprChirho::ConAppChirho {
            con_name_chirho,
            args_chirho,
        }
    }

    /// Expand RecordWildCards in a record construction expression.
    /// `Con { f1 = e1, .. }` → fills missing fields from scope variables.
    /// Returns desugared CoreExprChirho args in constructor field order.
    pub(super) fn expand_record_wildcard_expr_chirho(
        &mut self,
        con_name_chirho: &str,
        explicit_fields_chirho: &[haskelujah_ast_chirho::expr_chirho::FieldAssignChirho],
    ) -> Vec<CoreExprChirho> {
        let all_field_names_chirho = self.con_field_names_chirho.get(con_name_chirho).cloned();
        if let Some(all_fields_chirho) = all_field_names_chirho {
            let explicit_map_chirho: HashMap<String, &ExprChirho> = explicit_fields_chirho
                .iter()
                .map(|f_chirho| {
                    (
                        f_chirho.name_chirho.text_chirho().to_string(),
                        &f_chirho.value_chirho,
                    )
                })
                .collect();
            all_fields_chirho
                .iter()
                .map(|field_name_chirho| {
                    if let Some(expr_chirho) = explicit_map_chirho.get(field_name_chirho) {
                        self.desugar_expr_chirho(expr_chirho)
                    } else {
                        // Fill from scope: variable with same name as the field
                        let id_chirho = self.resolve_var_chirho(field_name_chirho);
                        CoreExprChirho::VarChirho(id_chirho)
                    }
                })
                .collect()
        } else {
            // Fallback: no field info, desugar explicit fields only
            explicit_fields_chirho
                .iter()
                .map(|f_chirho| self.desugar_expr_chirho(&f_chirho.value_chirho))
                .collect()
        }
    }

    /// Record construction by field label: arguments in the constructor's
    /// declared order, an omitted field becoming a thunk that names it when
    /// forced (`Missing field in record construction f`) — laziness kept, the
    /// error actionable. Without field information the explicit fields are
    /// passed positionally as before.
    /// workflow: language-features-chirho/rigid-type-variables-chirho (records)
    pub(super) fn record_construction_args_chirho(
        &mut self,
        con_name_chirho: &str,
        explicit_fields_chirho: &[haskelujah_ast_chirho::expr_chirho::FieldAssignChirho],
    ) -> Vec<CoreExprChirho> {
        let Some(all_fields_chirho) = self.con_field_names_chirho.get(con_name_chirho).cloned()
        else {
            if explicit_fields_chirho.is_empty()
                && let Some(arity_chirho) = self.con_arities_chirho.get(con_name_chirho).copied()
            {
                // `C {}` on a positional constructor: every argument is missing.
                return (0..arity_chirho)
                    .map(|_| self.missing_field_thunk_chirho(None))
                    .collect();
            }
            return explicit_fields_chirho
                .iter()
                .map(|f_chirho| self.desugar_expr_chirho(&f_chirho.value_chirho))
                .collect();
        };
        let explicit_map_chirho: HashMap<String, &ExprChirho> = explicit_fields_chirho
            .iter()
            .map(|f_chirho| {
                (
                    f_chirho.name_chirho.text_chirho().to_string(),
                    &f_chirho.value_chirho,
                )
            })
            .collect();
        all_fields_chirho
            .iter()
            .map(|field_name_chirho| {
                if let Some(expr_chirho) = explicit_map_chirho.get(field_name_chirho) {
                    self.desugar_expr_chirho(expr_chirho)
                } else {
                    self.missing_field_thunk_chirho(Some(field_name_chirho))
                }
            })
            .collect()
    }

    /// The thunk an omitted field becomes: GHC's runtime message, naming the
    /// field when the constructor declares one.
    fn missing_field_thunk_chirho(&mut self, field_name_chirho: Option<&str>) -> CoreExprChirho {
        let error_id_chirho = self.fresh_id_chirho("error");
        let message_chirho = match field_name_chirho {
            Some(name_chirho) => format!("Missing field in record construction {name_chirho}"),
            None => "Missing field in record construction".to_string(),
        };
        CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(error_id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                message_chirho,
            ))),
        }
    }
}
