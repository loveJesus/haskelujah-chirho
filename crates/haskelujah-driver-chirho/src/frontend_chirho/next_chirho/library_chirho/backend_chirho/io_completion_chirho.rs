// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Only the State#/unboxed-tuple helpers cross this boundary; their payload stays lazy.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::io_bodies_chirho::error_chirho;
use super::*;
use crate::next_chirho::library_chirho::backend_chirho::io_chirho::IoProviderChirho;
use haskelujah_runtime_chirho::io_chirho::RESULT_KEY_CHIRHO;
use haskelujah_typing_chirho::ty_chirho::TyChirho as CoreTypeChirho;

fn app_chirho(fun_chirho: CoreExprChirho, arg_chirho: CoreExprChirho) -> CoreExprChirho {
    CoreExprChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
    }
}
impl ElaboratorChirho<'_> {
    pub(super) fn io_completion_chirho(
        &mut self,
        mut ty_chirho: CoreTypeChirho,
        provider_chirho: IoProviderChirho,
        span_chirho: haskelujah_span_chirho::SpanChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        for _node_chirho in 0..32 {
            self.types_chirho.reserve_term_chirho()?;
        }
        let count_chirho = match provider_chirho {
            IoProviderChirho::ReturnChirho => 2,
            IoProviderChirho::BindChirho | IoProviderChirho::ThenChirho => 3,
            _ => return Err(error_chirho("not an IO completion provider")),
        };
        let mut parameters_chirho = Vec::new();
        for _parameter_chirho in 0..count_chirho {
            let CoreTypeChirho::FunChirho(argument_chirho, result_chirho, _) = ty_chirho else {
                return Err(error_chirho("IO completion contract lost an arrow"));
            };
            parameters_chirho.push(self.fresh_binder_chirho(*argument_chirho, span_chirho)?);
            ty_chirho = *result_chirho;
        }
        let var_chirho =
            |binder_chirho: &BinderChirho| CoreExprChirho::VarChirho(binder_chirho.id_chirho);
        let world_chirho = parameters_chirho
            .last()
            .ok_or_else(|| error_chirho("IO completion lost its world"))?;
        if world_chirho.ty_chirho != crate::next_chirho::io_chirho::world_type_chirho() {
            return Err(error_chirho("IO completion world representation drift"));
        }
        let mut body_chirho = if matches!(provider_chirho, IoProviderChirho::ReturnChirho) {
            CoreExprChirho::ConAppChirho {
                con_name_chirho: RESULT_KEY_CHIRHO.into(),
                args_chirho: vec![var_chirho(&parameters_chirho[0])],
            }
        } else {
            let CoreTypeChirho::FunChirho(_, first_result_chirho, _) =
                &parameters_chirho[0].ty_chirho
            else {
                return Err(error_chirho("IO completion action has no world arrow"));
            };
            let CoreTypeChirho::AppChirho(_, payload_type_chirho) = first_result_chirho.as_ref()
            else {
                return Err(error_chirho("IO completion action has no result box"));
            };
            let payload_chirho =
                self.fresh_binder_chirho(*payload_type_chirho.clone(), span_chirho)?;
            let case_chirho =
                self.fresh_binder_chirho(*first_result_chirho.clone(), span_chirho)?;
            let continuation_chirho = if matches!(provider_chirho, IoProviderChirho::BindChirho) {
                app_chirho(
                    var_chirho(&parameters_chirho[1]),
                    var_chirho(&payload_chirho),
                )
            } else {
                var_chirho(&parameters_chirho[1])
            };
            CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(app_chirho(
                    var_chirho(&parameters_chirho[0]),
                    var_chirho(world_chirho),
                )),
                bind_chirho: case_chirho,
                result_ty_chirho: ty_chirho,
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(RESULT_KEY_CHIRHO.into()),
                    binders_chirho: vec![payload_chirho],
                    rhs_chirho: app_chirho(continuation_chirho, var_chirho(world_chirho)),
                }],
            }
        };
        // All value/world lambdas enclose the case: constructing or seq'ing an action is inert.
        for parameter_chirho in parameters_chirho.into_iter().rev() {
            body_chirho = CoreExprChirho::LamChirho {
                binder_chirho: parameter_chirho,
                body_chirho: Box::new(body_chirho),
            };
        }
        Ok(body_chirho)
    }
}
