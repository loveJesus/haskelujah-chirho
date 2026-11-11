// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Runtime term walks erase type wrappers before recognizing application and closure spines.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;

impl LowerCtxChirho {
    /// A complex head has the same per-invocation capture lifetime as a complex argument.
    pub(super) fn lower_complex_application_chirho(
        &mut self,
        head_chirho: &CoreExprChirho,
        arguments_chirho: &[&CoreExprChirho],
    ) -> u32 {
        let head_source_chirho = self.lower_arg_source_chirho(head_chirho);
        let arg_sources_chirho = arguments_chirho
            .iter()
            .map(|argument_chirho| self.lower_arg_source_chirho(argument_chirho))
            .collect();
        match head_source_chirho {
            ArgSourceChirho::StaticChirho(ValueChirho::HeapPtrChirho(fun_chirho)) => self
                .emit_chirho(CodeChirho::AppChirho {
                    fun_chirho,
                    args_chirho: arg_sources_chirho,
                }),
            ArgSourceChirho::ArgRegChirho(fun_arg_index_chirho) => {
                self.emit_chirho(CodeChirho::AppFromArgChirho {
                    fun_arg_index_chirho,
                    arg_sources_chirho,
                })
            }
            ArgSourceChirho::ThunkCodeChirho {
                code_ptr_chirho,
                captures_chirho,
            } => {
                // All argument sources are lowered already. Nothing lowers another
                // expression after this unmapped scratch slot is chosen.
                let dest_reg_chirho = self
                    .arg_param_indices_chirho
                    .values()
                    .max()
                    .map_or(0, |index_chirho| index_chirho + 1);
                let body_chirho = self.emit_chirho(CodeChirho::AppFromArgChirho {
                    fun_arg_index_chirho: dest_reg_chirho,
                    arg_sources_chirho,
                });
                self.emit_chirho(CodeChirho::StoreAllocThunkChirho {
                    code_ptr_chirho,
                    captures_chirho,
                    dest_reg_chirho,
                    body_chirho,
                    name_chirho: "next_application_head_chirho".into(),
                })
            }
            // The checked pipeline cannot apply a non-function literal. Preserve the
            // legacy lowerer's behavior for unchecked Core inputs at this shared boundary.
            ArgSourceChirho::StaticChirho(value_chirho) => {
                self.emit_chirho(CodeChirho::LitChirho(value_chirho))
            }
        }
    }
}

/// Flatten nested `App` into (head, [arg1, arg2, ...]).
pub(super) fn collect_app_chirho(
    expr_chirho: &CoreExprChirho,
) -> (&CoreExprChirho, Vec<&CoreExprChirho>) {
    let mut args_chirho = Vec::new();
    let mut cur_chirho = expr_chirho;
    loop {
        match cur_chirho {
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                args_chirho.push(arg_chirho.as_ref());
                cur_chirho = fun_chirho.as_ref();
            }
            CoreExprChirho::TyAppChirho { expr_chirho, .. } => cur_chirho = expr_chirho,
            _ => break,
        }
    }
    args_chirho.reverse();
    (cur_chirho, args_chirho)
}

/// Flatten nested `Lam` into ([binder1, binder2, ...], body).
pub(super) fn collect_lam_chirho(
    expr_chirho: &CoreExprChirho,
) -> (
    Vec<&haskelujah_core_chirho::expr_chirho::BinderChirho>,
    &CoreExprChirho,
) {
    let mut binders_chirho = Vec::new();
    let mut cur_chirho = expr_chirho;
    loop {
        match cur_chirho {
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                binders_chirho.push(binder_chirho);
                cur_chirho = body_chirho.as_ref();
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => cur_chirho = body_chirho,
            _ => break,
        }
    }
    (binders_chirho, cur_chirho)
}

/// Collect all free variable CoreIds in an expression, excluding those
/// in the `bound_chirho` set. Used by lambda lowering to identify captures.
pub(super) fn free_vars_chirho(
    expr_chirho: &CoreExprChirho,
    bound_chirho: &HashSet<CoreIdChirho>,
) -> HashSet<CoreIdChirho> {
    let mut result_chirho = HashSet::new();
    free_vars_walk_chirho(expr_chirho, bound_chirho, &mut result_chirho);
    result_chirho
}

fn free_vars_walk_chirho(
    expr_chirho: &CoreExprChirho,
    bound_chirho: &HashSet<CoreIdChirho>,
    out_chirho: &mut HashSet<CoreIdChirho>,
) {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            if !bound_chirho.contains(id_chirho) {
                out_chirho.insert(*id_chirho);
            }
        }
        CoreExprChirho::LitChirho(_) => {}
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            free_vars_walk_chirho(fun_chirho, bound_chirho, out_chirho);
            free_vars_walk_chirho(arg_chirho, bound_chirho, out_chirho);
        }
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            let mut inner_bound_chirho = bound_chirho.clone();
            inner_bound_chirho.insert(binder_chirho.id_chirho);
            free_vars_walk_chirho(body_chirho, &inner_bound_chirho, out_chirho);
        }
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            let mut inner_bound_chirho = bound_chirho.clone();
            for (binder_chirho, _) in binds_chirho {
                inner_bound_chirho.insert(binder_chirho.id_chirho);
            }
            for (_, rhs_chirho) in binds_chirho {
                free_vars_walk_chirho(rhs_chirho, &inner_bound_chirho, out_chirho);
            }
            free_vars_walk_chirho(body_chirho, &inner_bound_chirho, out_chirho);
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            free_vars_walk_chirho(scrutinee_chirho, bound_chirho, out_chirho);
            let mut case_bound_chirho = bound_chirho.clone();
            case_bound_chirho.insert(bind_chirho.id_chirho);
            for alt_chirho in alts_chirho {
                let mut alt_bound_chirho = case_bound_chirho.clone();
                for b_chirho in &alt_chirho.binders_chirho {
                    alt_bound_chirho.insert(b_chirho.id_chirho);
                }
                free_vars_walk_chirho(&alt_chirho.rhs_chirho, &alt_bound_chirho, out_chirho);
            }
        }
        CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            for a_chirho in args_chirho {
                free_vars_walk_chirho(a_chirho, bound_chirho, out_chirho);
            }
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
            for a_chirho in args_chirho {
                free_vars_walk_chirho(a_chirho, bound_chirho, out_chirho);
            }
        }
        CoreExprChirho::TyAppChirho {
            expr_chirho: e_chirho,
            ..
        } => {
            free_vars_walk_chirho(e_chirho, bound_chirho, out_chirho);
        }
        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            free_vars_walk_chirho(body_chirho, bound_chirho, out_chirho);
        }
    }
}
