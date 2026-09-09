// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Close lazy field and nonrecursive-let computations over an explicit environment
//! before native lowering. Both native backends can thunk a saturated known call,
//! but neither may evaluate a lazy binding just because it is a primitive or case.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use std::collections::HashMap;

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
};
use crate::simplify_chirho::free_vars_chirho;
use crate::transform_chirho::{children_mut_chirho, max_expr_id_chirho};

/// Keep the shared Core input unchanged. Added functions are closed and always
/// have one argument, regardless of capture count, so no trampoline arity limit
/// can silently switch a field from lazy to eager.
pub fn prepare_native_thunks_chirho(module_chirho: &CoreModuleChirho) -> CoreModuleChirho {
    let mut prepared_chirho = module_chirho.clone();
    let mut highest_id_chirho = prepared_chirho
        .names_chirho
        .keys()
        .map(|id_chirho| id_chirho.0)
        .max()
        .unwrap_or(0);
    for binding_chirho in &prepared_chirho.bindings_chirho {
        highest_id_chirho = highest_id_chirho.max(binding_chirho.binder_chirho.id_chirho.0);
        max_expr_id_chirho(&binding_chirho.rhs_chirho, &mut highest_id_chirho);
    }
    let mut context_chirho = NativeThunksChirho {
        next_id_chirho: highest_id_chirho.checked_add(1).expect("Core id space"),
        names_chirho: HashMap::new(),
        lifted_chirho: Vec::new(),
    };
    for binding_chirho in &mut prepared_chirho.bindings_chirho {
        context_chirho.rewrite_chirho(&mut binding_chirho.rhs_chirho, &mut HashMap::new());
    }
    prepared_chirho
        .bindings_chirho
        .extend(context_chirho.lifted_chirho);
    prepared_chirho
        .names_chirho
        .extend(context_chirho.names_chirho);
    prepared_chirho
}

struct NativeThunksChirho {
    next_id_chirho: u32,
    names_chirho: HashMap<CoreIdChirho, String>,
    lifted_chirho: Vec<CoreBindingChirho>,
}

impl NativeThunksChirho {
    fn binder_chirho(&mut self, prefix_chirho: &str, ty_chirho: TyChirho) -> BinderChirho {
        let id_chirho = CoreIdChirho(self.next_id_chirho);
        self.next_id_chirho = self.next_id_chirho.checked_add(1).expect("Core id space");
        let name_chirho = format!("{prefix_chirho}_{}_chirho", id_chirho.0);
        self.names_chirho.insert(id_chirho, name_chirho.clone());
        BinderChirho {
            id_chirho,
            name_chirho,
            ty_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn rewrite_chirho(
        &mut self,
        expression_chirho: &mut CoreExprChirho,
        scope_chirho: &mut HashMap<CoreIdChirho, BinderChirho>,
    ) {
        match expression_chirho {
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                let previous_chirho =
                    scope_chirho.insert(binder_chirho.id_chirho, binder_chirho.clone());
                self.rewrite_chirho(body_chirho, scope_chirho);
                restore_chirho(scope_chirho, binder_chirho.id_chirho, previous_chirho);
            }
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => {
                let mut previous_chirho = Vec::with_capacity(binds_chirho.len());
                if *rec_chirho {
                    for (binder_chirho, _) in binds_chirho.iter() {
                        previous_chirho.push((
                            binder_chirho.id_chirho,
                            scope_chirho.insert(binder_chirho.id_chirho, binder_chirho.clone()),
                        ));
                    }
                }
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    self.rewrite_chirho(rhs_chirho, scope_chirho);
                    if !*rec_chirho {
                        if needs_thunk_chirho(rhs_chirho) {
                            let deferred_chirho = std::mem::replace(
                                rhs_chirho,
                                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                            );
                            *rhs_chirho = self.close_thunk_chirho(deferred_chirho, scope_chirho);
                        }
                        previous_chirho.push((
                            binder_chirho.id_chirho,
                            scope_chirho.insert(binder_chirho.id_chirho, binder_chirho.clone()),
                        ));
                    }
                }
                self.rewrite_chirho(body_chirho, scope_chirho);
                for (id_chirho, old_chirho) in previous_chirho.into_iter().rev() {
                    restore_chirho(scope_chirho, id_chirho, old_chirho);
                }
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                self.rewrite_chirho(scrutinee_chirho, scope_chirho);
                let previous_chirho =
                    scope_chirho.insert(bind_chirho.id_chirho, bind_chirho.clone());
                for alternative_chirho in alts_chirho {
                    let mut saved_chirho = Vec::new();
                    for binder_chirho in &alternative_chirho.binders_chirho {
                        saved_chirho.push((
                            binder_chirho.id_chirho,
                            scope_chirho.insert(binder_chirho.id_chirho, binder_chirho.clone()),
                        ));
                    }
                    self.rewrite_chirho(&mut alternative_chirho.rhs_chirho, scope_chirho);
                    for (id_chirho, old_chirho) in saved_chirho.into_iter().rev() {
                        restore_chirho(scope_chirho, id_chirho, old_chirho);
                    }
                }
                restore_chirho(scope_chirho, bind_chirho.id_chirho, previous_chirho);
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for field_chirho in args_chirho {
                    self.rewrite_chirho(field_chirho, scope_chirho);
                    if needs_thunk_chirho(field_chirho) {
                        let body_chirho = std::mem::replace(
                            field_chirho,
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                        );
                        *field_chirho = self.close_thunk_chirho(body_chirho, scope_chirho);
                    }
                }
            }
            _ => children_mut_chirho(expression_chirho, &mut |child_chirho| {
                self.rewrite_chirho(child_chirho, scope_chirho)
            }),
        }
    }

    fn close_thunk_chirho(
        &mut self,
        mut body_chirho: CoreExprChirho,
        scope_chirho: &HashMap<CoreIdChirho, BinderChirho>,
    ) -> CoreExprChirho {
        let mut captures_chirho: Vec<_> = free_vars_chirho(&body_chirho)
            .into_iter()
            .filter_map(|id_chirho| scope_chirho.get(&id_chirho).cloned())
            .collect();
        captures_chirho.sort_by_key(|binder_chirho| binder_chirho.id_chirho);
        let environment_ty_chirho = TyChirho::ConChirho("$NativeThunkEnvChirho".to_string());
        let parameter_chirho =
            self.binder_chirho("thunk_environment", environment_ty_chirho.clone());
        let result_ty_chirho = TyChirho::VarChirho(TyVarChirho(self.next_id_chirho));
        let function_chirho = self.binder_chirho(
            "thunk_deferred",
            TyChirho::fun_chirho(environment_ty_chirho.clone(), result_ty_chirho.clone()),
        );
        let argument_chirho = if captures_chirho.is_empty() {
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
        } else {
            let mut renames_chirho = HashMap::new();
            let fields_chirho: Vec<_> = captures_chirho
                .iter()
                .map(|capture_chirho| {
                    let field_chirho =
                        self.binder_chirho("thunk_capture", capture_chirho.ty_chirho.clone());
                    renames_chirho.insert(capture_chirho.id_chirho, field_chirho.id_chirho);
                    field_chirho
                })
                .collect();
            rename_references_chirho(&mut body_chirho, &renames_chirho);
            body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(parameter_chirho.id_chirho)),
                bind_chirho: self.binder_chirho("thunk_environment_value", environment_ty_chirho),
                result_ty_chirho,
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$NativeThunkEnvChirho".to_string()),
                    binders_chirho: fields_chirho,
                    rhs_chirho: body_chirho,
                }],
            };
            CoreExprChirho::ConAppChirho {
                con_name_chirho: "$NativeThunkEnvChirho".to_string(),
                args_chirho: captures_chirho
                    .iter()
                    .map(|capture_chirho| CoreExprChirho::VarChirho(capture_chirho.id_chirho))
                    .collect(),
            }
        };
        self.lifted_chirho.push(CoreBindingChirho {
            binder_chirho: function_chirho.clone(),
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: parameter_chirho,
                body_chirho: Box::new(body_chirho),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NeverChirho,
        });
        CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(function_chirho.id_chirho)),
            arg_chirho: Box::new(argument_chirho),
        }
    }
}

fn restore_chirho(
    scope_chirho: &mut HashMap<CoreIdChirho, BinderChirho>,
    id_chirho: CoreIdChirho,
    previous_chirho: Option<BinderChirho>,
) {
    if let Some(previous_chirho) = previous_chirho {
        scope_chirho.insert(id_chirho, previous_chirho);
    } else {
        scope_chirho.remove(&id_chirho);
    }
}

fn needs_thunk_chirho(expression_chirho: &CoreExprChirho) -> bool {
    match expression_chirho {
        CoreExprChirho::AppChirho { .. }
        | CoreExprChirho::PrimOpChirho { .. }
        | CoreExprChirho::CaseChirho { .. }
        | CoreExprChirho::LetChirho { .. } => true,
        CoreExprChirho::TyAppChirho { expr_chirho, .. }
        | CoreExprChirho::TyLamChirho {
            body_chirho: expr_chirho,
            ..
        } => needs_thunk_chirho(expr_chirho),
        CoreExprChirho::VarChirho(_)
        | CoreExprChirho::LitChirho(_)
        | CoreExprChirho::LamChirho { .. }
        | CoreExprChirho::ConAppChirho { .. } => false,
    }
}

fn rename_references_chirho(
    expression_chirho: &mut CoreExprChirho,
    renames_chirho: &HashMap<CoreIdChirho, CoreIdChirho>,
) {
    if let CoreExprChirho::VarChirho(id_chirho) = expression_chirho {
        if let Some(replacement_chirho) = renames_chirho.get(id_chirho) {
            *id_chirho = *replacement_chirho;
        }
    } else {
        children_mut_chirho(expression_chirho, &mut |child_chirho| {
            rename_references_chirho(child_chirho, renames_chirho)
        });
    }
}
