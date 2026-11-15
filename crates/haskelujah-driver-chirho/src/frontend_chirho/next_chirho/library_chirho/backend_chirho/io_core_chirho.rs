// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! IO completion is lifted; output demand stays in the machine's ordinary cases.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use crate::next_chirho::library_chirho::backend_chirho::io_chirho::IoProviderChirho;
use haskelujah_runtime_chirho::io_chirho::{
    IoOperationChirho, RESULT_KEY_CHIRHO, STDOUT_KEY_CHIRHO,
};
use haskelujah_typing_chirho::ty_chirho::TyChirho as CoreTypeChirho;
fn app_chirho(fun_chirho: CoreExprChirho, arg_chirho: CoreExprChirho) -> CoreExprChirho {
    CoreExprChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
    }
}
fn force_chirho(
    input_chirho: CoreExprChirho,
    binder_chirho: BinderChirho,
    body_chirho: CoreExprChirho,
    result_chirho: &CoreTypeChirho,
) -> CoreExprChirho {
    CoreExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(input_chirho),
        bind_chirho: binder_chirho,
        result_ty_chirho: result_chirho.clone(),
        alts_chirho: vec![CoreAltChirho {
            con_chirho: AltConChirho::DefaultChirho,
            binders_chirho: Vec::new(),
            rhs_chirho: body_chirho,
        }],
    }
}
impl ElaboratorChirho<'_> {
    pub(super) fn io_expression_chirho(
        &mut self,
        mut ty_chirho: CoreTypeChirho,
        provider_chirho: IoProviderChirho,
        span_chirho: haskelujah_span_chirho::SpanChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        // The template's term count is constant and charged before allocation.
        for _term_chirho in 0..160 {
            self.types_chirho.reserve_term_chirho()?;
        }
        if matches!(provider_chirho, IoProviderChirho::StdoutChirho) {
            return Ok(CoreExprChirho::ConAppChirho {
                con_name_chirho: STDOUT_KEY_CHIRHO.into(),
                args_chirho: Vec::new(),
            });
        }
        let mut parameters_chirho = Vec::new();
        for _parameter_chirho in 0..4 {
            let CoreTypeChirho::FunChirho(argument_chirho, result_chirho, _) = ty_chirho else {
                return Err(error_chirho("IO body lacks its checked source/world arrow"));
            };
            parameters_chirho.push(self.fresh_binder_chirho(*argument_chirho, span_chirho)?);
            ty_chirho = *result_chirho;
        }
        let [handle_chirho, string_chirho, newline_chirho, world_chirho] =
            parameters_chirho.as_slice()
        else {
            return Err(error_chirho("IO parameters lost"));
        };
        let world_type_chirho = crate::next_chirho::io_chirho::world_type_chirho();
        if world_chirho.ty_chirho != world_type_chirho {
            return Err(error_chirho("IO world representation drift"));
        }
        let result_chirho = ty_chirho;
        let evaluated_handle_chirho =
            self.fresh_binder_chirho(handle_chirho.ty_chirho.clone(), span_chirho)?;
        let loop_type_chirho = CoreTypeChirho::fun_chirho(
            string_chirho.ty_chirho.clone(),
            CoreTypeChirho::fun_chirho(world_type_chirho.clone(), result_chirho.clone()),
        );
        let loop_chirho = self.fresh_binder_chirho(loop_type_chirho, span_chirho)?;
        let list_chirho = self.fresh_binder_chirho(string_chirho.ty_chirho.clone(), span_chirho)?;
        let state_chirho = self.fresh_binder_chirho(world_type_chirho.clone(), span_chirho)?;
        let list_case_chirho =
            self.fresh_binder_chirho(string_chirho.ty_chirho.clone(), span_chirho)?;
        let CoreTypeChirho::AppChirho(_, char_type_chirho) = &string_chirho.ty_chirho else {
            return Err(error_chirho("IO string is not a checked list"));
        };
        let head_chirho = self.fresh_binder_chirho(*char_type_chirho.clone(), span_chirho)?;
        let tail_chirho = self.fresh_binder_chirho(string_chirho.ty_chirho.clone(), span_chirho)?;
        let prepared_chirho = self.fresh_binder_chirho(world_type_chirho.clone(), span_chirho)?;
        let char_chirho = self.fresh_binder_chirho(*char_type_chirho.clone(), span_chirho)?;
        let appended_chirho = self.fresh_binder_chirho(world_type_chirho.clone(), span_chirho)?;
        let finished_chirho = self.fresh_binder_chirho(world_type_chirho, span_chirho)?;
        let var_chirho =
            |binder_chirho: &BinderChirho| CoreExprChirho::VarChirho(binder_chirho.id_chirho);
        let call_chirho = app_chirho(
            app_chirho(var_chirho(&loop_chirho), var_chirho(&tail_chirho)),
            var_chirho(&appended_chirho),
        );
        let append_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: IoOperationChirho::APPEND_CHIRHO.primitive_chirho().into(),
            args_chirho: vec![
                var_chirho(&evaluated_handle_chirho),
                var_chirho(&char_chirho),
                var_chirho(&prepared_chirho),
            ],
        };
        let cons_chirho = force_chirho(
            CoreExprChirho::PrimOpChirho {
                name_chirho: IoOperationChirho::PREPARE_CHIRHO.primitive_chirho().into(),
                args_chirho: vec![
                    var_chirho(&evaluated_handle_chirho),
                    var_chirho(&state_chirho),
                ],
            },
            prepared_chirho.clone(),
            force_chirho(
                var_chirho(&head_chirho),
                char_chirho,
                force_chirho(append_chirho, appended_chirho, call_chirho, &result_chirho),
                &result_chirho,
            ),
            &result_chirho,
        );
        let nil_chirho = force_chirho(
            CoreExprChirho::PrimOpChirho {
                name_chirho: IoOperationChirho::FinishChirho.primitive_chirho().into(),
                args_chirho: vec![
                    var_chirho(&evaluated_handle_chirho),
                    var_chirho(newline_chirho),
                    var_chirho(&state_chirho),
                ],
            },
            finished_chirho,
            CoreExprChirho::ConAppChirho {
                con_name_chirho: RESULT_KEY_CHIRHO.into(),
                args_chirho: vec![CoreExprChirho::ConAppChirho {
                    con_name_chirho: "()".into(),
                    args_chirho: Vec::new(),
                }],
            },
            &result_chirho,
        );
        let loop_body_chirho = CoreExprChirho::LamChirho {
            binder_chirho: list_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: state_chirho,
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(var_chirho(&list_chirho)),
                    bind_chirho: list_case_chirho,
                    result_ty_chirho: result_chirho.clone(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("[]".into()),
                            binders_chirho: Vec::new(),
                            rhs_chirho: nil_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho(":".into()),
                            binders_chirho: vec![head_chirho, tail_chirho],
                            rhs_chirho: cons_chirho,
                        },
                    ],
                }),
            }),
        };
        let body_chirho = CoreExprChirho::LetChirho {
            rec_chirho: true,
            binds_chirho: vec![(loop_chirho.clone(), loop_body_chirho)],
            body_chirho: Box::new(app_chirho(
                app_chirho(var_chirho(&loop_chirho), var_chirho(string_chirho)),
                var_chirho(world_chirho),
            )),
        };
        let mut body_chirho = force_chirho(
            var_chirho(handle_chirho),
            evaluated_handle_chirho,
            body_chirho,
            &result_chirho,
        );
        for binder_chirho in parameters_chirho.into_iter().rev() {
            body_chirho = CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho: Box::new(body_chirho),
            };
        }
        Ok(body_chirho)
    }
}

fn error_chirho(message_chirho: &str) -> NextErrorChirho {
    NextErrorChirho::new_chirho(NextErrorKindChirho::UnsupportedChirho, message_chirho)
}
