// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! An IO action is a reusable value, not an updatable thunk of its effect.
//! Lower actions to a constructor containing a function; executing that function
//! returns a separate result constructor whose payload remains lazy.
//! This is an execution lowering, after dictionary selection and simplification.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use std::collections::{HashMap, HashSet};

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
};
use crate::transform_chirho::{children_mut_chirho, max_expr_id_chirho};

const ACTION_CON_CHIRHO: &str = "$IOActionChirho";
const RESULT_CON_CHIRHO: &str = "$IOResultChirho";

/// Preserve the original Core and its reusable entry value. A separate wrapper
/// executes only the requested entry; references to that binding still denote
/// the action, including recursive references.
pub fn prepare_io_actions_chirho(
    module_chirho: &CoreModuleChirho,
    entry_name_chirho: &str,
) -> CoreModuleChirho {
    let mut prepared_chirho = module_chirho.clone();
    let mut maximum_chirho = module_chirho
        .names_chirho
        .keys()
        .map(|id_chirho| id_chirho.0)
        .max()
        .unwrap_or(0);
    for binding_chirho in &module_chirho.bindings_chirho {
        maximum_chirho = maximum_chirho.max(binding_chirho.binder_chirho.id_chirho.0);
        max_expr_id_chirho(&binding_chirho.rhs_chirho, &mut maximum_chirho);
    }
    let mut context_chirho = ActionsChirho {
        next_id_chirho: maximum_chirho.checked_add(1).expect("Core id space"),
        raw_effects_chirho: HashMap::new(),
        raw_bindings_chirho: Vec::new(),
    };
    let defined_chirho: HashSet<_> = prepared_chirho
        .bindings_chirho
        .iter()
        .map(|binding_chirho| binding_chirho.binder_chirho.id_chirho)
        .collect();
    let referenced_chirho: HashSet<_> = prepared_chirho
        .bindings_chirho
        .iter()
        .flat_map(|binding_chirho| {
            crate::simplify_chirho::free_vars_chirho(&binding_chirho.rhs_chirho)
        })
        .collect();
    let mut primitives_chirho: Vec<_> = module_chirho
        .names_chirho
        .iter()
        .filter_map(|(id_chirho, name_chirho)| {
            (!defined_chirho.contains(id_chirho) && referenced_chirho.contains(id_chirho))
                .then(|| {
                    operation_chirho(name_chirho)
                        .map(|op_chirho| (*id_chirho, name_chirho.clone(), op_chirho))
                })
                .flatten()
        })
        .collect();
    primitives_chirho.sort_by_key(|(id_chirho, _, _)| *id_chirho);
    for (id_chirho, name_chirho, operation_chirho) in primitives_chirho {
        let arguments_chirho: Vec<_> = (0..operation_chirho.arity_chirho())
            .map(|_| context_chirho.binder_chirho("io_argument"))
            .collect();
        let mut body_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho: arguments_chirho.iter().map(var_chirho).collect(),
        };
        for argument_chirho in arguments_chirho.into_iter().rev() {
            body_chirho = lambda_chirho(argument_chirho, body_chirho);
        }
        prepared_chirho.bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho,
                name_chirho: format!("$io_primitive_{}_chirho", id_chirho.0),
                ty_chirho: TyChirho::VarChirho(TyVarChirho(id_chirho.0)),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: body_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NeverChirho,
        });
    }
    for binding_chirho in &mut prepared_chirho.bindings_chirho {
        context_chirho.rewrite_chirho(&mut binding_chirho.rhs_chirho);
        // Existing backend name adapters implement raw effects. The generated
        // functions now construct actions and must not enter those adapters.
        if operation_chirho(&binding_chirho.binder_chirho.name_chirho).is_some() {
            binding_chirho.binder_chirho.name_chirho = format!(
                "$io_binding_{}_chirho",
                binding_chirho.binder_chirho.id_chirho.0
            );
        }
        prepared_chirho.names_chirho.insert(
            binding_chirho.binder_chirho.id_chirho,
            binding_chirho.binder_chirho.name_chirho.clone(),
        );
    }
    if let Some(index_chirho) = prepared_chirho
        .bindings_chirho
        .iter()
        .position(|binding_chirho| binding_chirho.binder_chirho.name_chirho == entry_name_chirho)
    {
        let entry_chirho = &mut prepared_chirho.bindings_chirho[index_chirho];
        let value_chirho = var_chirho(&entry_chirho.binder_chirho);
        entry_chirho.binder_chirho.name_chirho = format!(
            "$io_entry_value_{}_chirho",
            entry_chirho.binder_chirho.id_chirho.0
        );
        prepared_chirho.names_chirho.insert(
            entry_chirho.binder_chirho.id_chirho,
            entry_chirho.binder_chirho.name_chirho.clone(),
        );
        let mut wrapper_chirho = context_chirho.binder_chirho("io_entry");
        wrapper_chirho.name_chirho = entry_name_chirho.to_string();
        wrapper_chirho.ty_chirho = entry_chirho.binder_chirho.ty_chirho.clone();
        let result_chirho = context_chirho.execute_entry_chirho(value_chirho);
        prepared_chirho
            .names_chirho
            .insert(wrapper_chirho.id_chirho, entry_name_chirho.to_string());
        prepared_chirho.bindings_chirho.push(CoreBindingChirho {
            binder_chirho: wrapper_chirho,
            rhs_chirho: result_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NeverChirho,
        });
    }
    for binding_chirho in context_chirho.raw_bindings_chirho {
        prepared_chirho.names_chirho.insert(
            binding_chirho.binder_chirho.id_chirho,
            binding_chirho.binder_chirho.name_chirho.clone(),
        );
        prepared_chirho.bindings_chirho.push(binding_chirho);
    }
    prepared_chirho
}

#[derive(Clone, Copy)]
enum OperationChirho {
    ReturnChirho,
    BindChirho,
    ThenChirho,
    EffectChirho(&'static str, usize),
}

impl OperationChirho {
    fn arity_chirho(self) -> usize {
        match self {
            Self::ReturnChirho => 1,
            Self::BindChirho | Self::ThenChirho => 2,
            Self::EffectChirho(_, arity_chirho) => arity_chirho,
        }
    }
}

fn operation_chirho(name_chirho: &str) -> Option<OperationChirho> {
    use OperationChirho::{BindChirho, EffectChirho, ReturnChirho, ThenChirho};
    Some(match name_chirho {
        "return" | "pure" | "returnIO#" => ReturnChirho,
        ">>=" | "bindIO#" => BindChirho,
        ">>" | "thenIO#" => ThenChirho,
        "putStrLn" | "putStrLn#" => EffectChirho("putStrLn#", 1),
        "putStr" | "putStr#" => EffectChirho("putStr#", 1),
        "putChar" | "putChar#" => EffectChirho("putChar#", 1),
        "print" | "print#" => EffectChirho("print#", 1),
        "getLine" | "getLine#" => EffectChirho("getLine#", 0),
        "getContents" | "getContents#" => EffectChirho("getContents#", 0),
        "getChar" | "getChar#" => EffectChirho("getChar#", 0),
        "readFile" | "readFile#" => EffectChirho("readFile#", 1),
        "writeFile" | "writeFile#" => EffectChirho("writeFile#", 2),
        "appendFile" | "appendFile#" => EffectChirho("appendFile#", 2),
        "newIORef" | "newIORef#" => EffectChirho("newIORef#", 1),
        "readIORef" | "readIORef#" => EffectChirho("readIORef#", 1),
        "writeIORef" | "writeIORef#" => EffectChirho("writeIORef#", 2),
        "modifyIORef" | "modifyIORef#" => EffectChirho("modifyIORef#", 2),
        "evaluate" => EffectChirho("force#", 1),
        "throwIO" | "throwIO#" => EffectChirho("throw#", 1),
        _ => return None,
    })
}

struct ActionsChirho {
    next_id_chirho: u32,
    raw_effects_chirho: HashMap<&'static str, BinderChirho>,
    raw_bindings_chirho: Vec<CoreBindingChirho>,
}

impl ActionsChirho {
    fn binder_chirho(&mut self, prefix_chirho: &str) -> BinderChirho {
        let id_chirho = CoreIdChirho(self.next_id_chirho);
        self.next_id_chirho = self.next_id_chirho.checked_add(1).expect("Core id space");
        BinderChirho {
            id_chirho,
            name_chirho: format!("\u{24}{prefix_chirho}_{}_chirho", id_chirho.0),
            ty_chirho: TyChirho::VarChirho(TyVarChirho(id_chirho.0)),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn rewrite_chirho(&mut self, expression_chirho: &mut CoreExprChirho) {
        children_mut_chirho(expression_chirho, &mut |child_chirho| {
            self.rewrite_chirho(child_chirho)
        });
        let CoreExprChirho::PrimOpChirho {
            name_chirho,
            args_chirho,
        } = expression_chirho
        else {
            return;
        };
        let Some(operation_chirho) = operation_chirho(name_chirho) else {
            return;
        };
        if operation_chirho.arity_chirho() != args_chirho.len() {
            return; // The backend's unsupported-primitive diagnostic retains the malformed call.
        }
        let mut arguments_chirho = std::mem::take(args_chirho).into_iter();
        let execution_chirho = match operation_chirho {
            OperationChirho::ReturnChirho => result_chirho(arguments_chirho.next().unwrap()),
            OperationChirho::BindChirho | OperationChirho::ThenChirho => {
                let first_chirho = arguments_chirho.next().unwrap();
                let second_chirho = arguments_chirho.next().unwrap();
                let value_chirho = self.binder_chirho("io_bound");
                let next_chirho = if matches!(operation_chirho, OperationChirho::BindChirho) {
                    app_chirho(second_chirho, var_chirho(&value_chirho))
                } else {
                    second_chirho
                };
                let next_chirho = self.run_chirho(next_chirho);
                let first_chirho = self.run_chirho(first_chirho);
                self.unpack_chirho(first_chirho, RESULT_CON_CHIRHO, value_chirho, next_chirho)
            }
            OperationChirho::EffectChirho(raw_name_chirho, _) => {
                let value_chirho = self.binder_chirho("io_effect_result");
                let result_chirho = result_chirho(var_chirho(&value_chirho));
                let effect_chirho =
                    self.raw_effect_chirho(raw_name_chirho, arguments_chirho.collect());
                self.case_chirho(
                    effect_chirho,
                    value_chirho,
                    vec![CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: result_chirho,
                    }],
                )
            }
        };
        let token_chirho = self.binder_chirho("io_execution");
        *expression_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: ACTION_CON_CHIRHO.to_string(),
            args_chirho: vec![lambda_chirho(token_chirho, execution_chirho)],
        };
    }

    fn raw_effect_chirho(
        &mut self,
        name_chirho: &'static str,
        arguments_chirho: Vec<CoreExprChirho>,
    ) -> CoreExprChirho {
        let arity_chirho = arguments_chirho.len();
        let function_chirho =
            if let Some(function_chirho) = self.raw_effects_chirho.get(name_chirho) {
                function_chirho.clone()
            } else {
                let mut function_chirho = self.binder_chirho("io_raw");
                function_chirho.name_chirho = name_chirho.to_string();
                let parameters_chirho: Vec<_> = (0..arity_chirho.max(1))
                    .map(|_| self.binder_chirho("io_raw_argument"))
                    .collect();
                let mut body_chirho = CoreExprChirho::PrimOpChirho {
                    name_chirho: name_chirho.to_string(),
                    args_chirho: parameters_chirho
                        .iter()
                        .take(arity_chirho)
                        .map(var_chirho)
                        .collect(),
                };
                for parameter_chirho in parameters_chirho.into_iter().rev() {
                    body_chirho = lambda_chirho(parameter_chirho, body_chirho);
                }
                self.raw_bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: function_chirho.clone(),
                    rhs_chirho: body_chirho,
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NeverChirho,
                });
                self.raw_effects_chirho
                    .insert(name_chirho, function_chirho.clone());
                function_chirho
            };
        let mut call_chirho = var_chirho(&function_chirho);
        if arity_chirho == 0 {
            call_chirho = app_chirho(
                call_chirho,
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
            );
        } else {
            for argument_chirho in arguments_chirho {
                call_chirho = app_chirho(call_chirho, argument_chirho);
            }
        }
        call_chirho
    }

    fn case_chirho(
        &mut self,
        scrutinee_chirho: CoreExprChirho,
        bind_chirho: BinderChirho,
        alts_chirho: Vec<CoreAltChirho>,
    ) -> CoreExprChirho {
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(scrutinee_chirho),
            result_ty_chirho: TyChirho::VarChirho(TyVarChirho(bind_chirho.id_chirho.0)),
            bind_chirho,
            alts_chirho,
        }
    }

    fn unpack_chirho(
        &mut self,
        scrutinee_chirho: CoreExprChirho,
        constructor_chirho: &str,
        field_chirho: BinderChirho,
        body_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let bind_chirho = self.binder_chirho("io_packet");
        self.case_chirho(
            scrutinee_chirho,
            bind_chirho,
            vec![CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho(constructor_chirho.to_string()),
                binders_chirho: vec![field_chirho],
                rhs_chirho: body_chirho,
            }],
        )
    }

    fn run_chirho(&mut self, action_chirho: CoreExprChirho) -> CoreExprChirho {
        let function_chirho = self.binder_chirho("io_function");
        let call_chirho = app_chirho(
            var_chirho(&function_chirho),
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
        );
        self.unpack_chirho(
            action_chirho,
            ACTION_CON_CHIRHO,
            function_chirho,
            call_chirho,
        )
    }

    fn execute_entry_chirho(&mut self, value_chirho: CoreExprChirho) -> CoreExprChirho {
        let function_chirho = self.binder_chirho("io_function");
        let payload_chirho = self.binder_chirho("io_result");
        let packet_chirho = app_chirho(
            var_chirho(&function_chirho),
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
        );
        let unwrapped_chirho = self.unpack_chirho(
            packet_chirho,
            RESULT_CON_CHIRHO,
            payload_chirho.clone(),
            var_chirho(&payload_chirho),
        );
        let original_chirho = self.binder_chirho("entry_value");
        let action_case_chirho = self.case_chirho(
            value_chirho.clone(),
            original_chirho.clone(),
            vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(ACTION_CON_CHIRHO.to_string()),
                    binders_chirho: vec![function_chirho],
                    rhs_chirho: unwrapped_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: var_chirho(&original_chirho),
                },
            ],
        );
        // Unboxed integers are not constructor tags, even when their bits happen
        // to equal our constructor's tag. Classify before constructor dispatch.
        let classification_chirho = self.binder_chirho("entry_boxed");
        self.case_chirho(
            CoreExprChirho::PrimOpChirho {
                name_chirho: "isHeapObjectChirho#".to_string(),
                args_chirho: vec![value_chirho.clone()],
            },
            classification_chirho,
            vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(0)),
                    binders_chirho: vec![],
                    rhs_chirho: value_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: action_case_chirho,
                },
            ],
        )
    }
}

fn var_chirho(binder_chirho: &BinderChirho) -> CoreExprChirho {
    CoreExprChirho::VarChirho(binder_chirho.id_chirho)
}

fn app_chirho(function_chirho: CoreExprChirho, argument_chirho: CoreExprChirho) -> CoreExprChirho {
    CoreExprChirho::AppChirho {
        fun_chirho: Box::new(function_chirho),
        arg_chirho: Box::new(argument_chirho),
    }
}

fn lambda_chirho(binder_chirho: BinderChirho, body_chirho: CoreExprChirho) -> CoreExprChirho {
    CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho: Box::new(body_chirho),
    }
}

fn result_chirho(value_chirho: CoreExprChirho) -> CoreExprChirho {
    CoreExprChirho::ConAppChirho {
        con_name_chirho: RESULT_CON_CHIRHO.to_string(),
        args_chirho: vec![value_chirho],
    }
}
