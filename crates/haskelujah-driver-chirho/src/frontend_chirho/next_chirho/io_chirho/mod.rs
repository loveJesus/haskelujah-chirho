// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Main's explicit world entry. Lifted completion never demands its result field.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_core_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho,
    InlineAnnotationChirho,
};
use haskelujah_runtime_chirho::io_chirho::{IoErrorChirho, RESULT_KEY_CHIRHO, WORLD_KEY_CHIRHO};
use haskelujah_runtime_chirho::{CodeChirho, EvalErrorChirho, MachineChirho, PrimErrorChirho};
use haskelujah_typing_chirho::ty_chirho::TyChirho as CoreTypeChirho;
#[derive(Clone, Copy)]
pub struct RunLimitsChirho {
    pub steps_chirho: u64,
    pub output_bytes_chirho: usize,
}
impl Default for RunLimitsChirho {
    fn default() -> Self {
        Self {
            steps_chirho: 5_000_000,
            output_bytes_chirho: 1 << 20,
        }
    }
}
pub struct MainOutcomeChirho {
    pub result_chirho: Result<(), NextErrorChirho>,
    pub machine_chirho: MachineChirho,
}
pub(super) fn world_type_chirho() -> CoreTypeChirho {
    CoreTypeChirho::ConChirho(WORLD_KEY_CHIRHO.into())
}
pub(super) fn result_type_chirho(ty_chirho: CoreTypeChirho) -> CoreTypeChirho {
    CoreTypeChirho::AppChirho(
        Box::new(CoreTypeChirho::ConChirho(RESULT_KEY_CHIRHO.into())),
        Box::new(ty_chirho),
    )
}
fn error_chirho(kind_chirho: NextErrorKindChirho, message_chirho: &str) -> NextErrorChirho {
    NextErrorChirho::new_chirho(kind_chirho, message_chirho)
}
pub(super) fn validate_main_chirho(
    checked_chirho: &CheckedModuleChirho,
) -> Result<(), NextErrorChirho> {
    if checked_chirho.module_chirho.module_chirho != "Main" {
        return Ok(());
    }
    let id_chirho = checked_chirho
        .module_chirho
        .exports_chirho
        .iter()
        .find(|id_chirho| {
            let info_chirho = &checked_chirho.module_chirho.names_chirho[id_chirho.0 as usize];
            info_chirho.occurrence_chirho == "main"
                && info_chirho.namespace_chirho
                    == haskelujah_hir_chirho::NamespaceChirho::ValueChirho
        })
        .ok_or_else(|| error_chirho(NextErrorKindChirho::InvalidChirho, "Main must export main"))?;
    if !checked_chirho
        .module_chirho
        .bindings_chirho
        .iter()
        .any(|binding_chirho| binding_chirho.name_chirho == *id_chirho)
    {
        return Err(error_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "re-exported main needs an original-name entry carrier",
        ));
    }
    let scheme_chirho = checked_chirho
        .schemes_chirho
        .get(id_chirho)
        .ok_or_else(|| {
            error_chirho(
                NextErrorKindChirho::UnsupportedChirho,
                "main has no checked binding scheme",
            )
        })?;
    if !scheme_chirho.context_chirho.is_empty() {
        return Err(error_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "constrained main is outside the checked IO entry profile",
        ));
    }
    let entry_chirho = checked_chirho
        .module_chirho
        .bindings_chirho
        .iter()
        .find(|binding_chirho| {
            checked_chirho.module_chirho.names_chirho[binding_chirho.name_chirho.0 as usize]
                .occurrence_chirho
                == haskelujah_tc_chirho::MAIN_ENTRY_CHIRHO
        })
        .ok_or_else(|| {
            error_chirho(
                NextErrorKindChirho::UnsupportedChirho,
                "main has no checked entry instantiation",
            )
        })?;
    let scheme_chirho = &checked_chirho.schemes_chirho[&entry_chirho.name_chirho];
    if !scheme_chirho.quantified_chirho.is_empty() || !scheme_chirho.context_chirho.is_empty() {
        return Err(error_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "main entry is not monomorphic",
        ));
    }
    let arena_chirho = &checked_chirho.solver_chirho.arena_chirho;
    let haskelujah_tc_chirho::TyNodeChirho::AppChirho(head_chirho, _) =
        arena_chirho.node_chirho(arena_chirho.resolve_chirho(scheme_chirho.body_chirho)?)?
    else {
        return Err(error_chirho(
            NextErrorKindChirho::TypeChirho,
            "main must have the defining IO type",
        ));
    };
    if !matches!(
        arena_chirho.node_chirho(arena_chirho.resolve_chirho(*head_chirho)?)?,
        haskelujah_tc_chirho::TyNodeChirho::ConChirho(haskelujah_tc_chirho::TyConChirho::IOChirho)
    ) {
        return Err(error_chirho(
            NextErrorKindChirho::TypeChirho,
            "main has a different nominal type from IO",
        ));
    }
    Ok(())
}
pub fn run_main_chirho(
    source_chirho: &str,
    limits_chirho: RunLimitsChirho,
) -> Result<MainOutcomeChirho, NextErrorChirho> {
    run_core_chirho(compile_source_chirho(source_chirho)?, limits_chirho)
}
pub fn run_main_file_chirho(
    path_chirho: &std::path::Path,
    limits_chirho: RunLimitsChirho,
) -> Result<MainOutcomeChirho, NextErrorChirho> {
    run_core_chirho(compile_source_file_chirho(path_chirho)?, limits_chirho)
}
pub(super) fn run_core_chirho(
    mut core_chirho: CoreModuleChirho,
    limits_chirho: RunLimitsChirho,
) -> Result<MainOutcomeChirho, NextErrorChirho> {
    if limits_chirho.steps_chirho == 0 {
        return Err(error_chirho(
            NextErrorKindChirho::ResourceChirho,
            "next run requires a finite step allowance",
        ));
    }
    if core_chirho.name_chirho != "Main" {
        return Err(error_chirho(
            NextErrorKindChirho::InvalidChirho,
            "next run requires module Main",
        ));
    }
    let main_chirho = core_chirho
        .bindings_chirho
        .iter()
        .find(|binding_chirho| {
            binding_chirho.binder_chirho.name_chirho == haskelujah_tc_chirho::MAIN_ENTRY_CHIRHO
        })
        .ok_or_else(|| {
            error_chirho(
                NextErrorKindChirho::UnsupportedChirho,
                "checked Main entry was not emitted",
            )
        })?
        .binder_chirho
        .clone();
    let CoreTypeChirho::FunChirho(world_chirho, completion_chirho, _) = &main_chirho.ty_chirho
    else {
        return Err(error_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "main lost its IO world representation",
        ));
    };
    if **world_chirho != world_type_chirho() {
        return Err(error_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "main world type drift",
        ));
    }
    let CoreTypeChirho::AppChirho(box_chirho, payload_chirho) = completion_chirho.as_ref() else {
        return Err(error_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "main completion is not lifted",
        ));
    };
    if **box_chirho != CoreTypeChirho::ConChirho(RESULT_KEY_CHIRHO.into()) {
        return Err(error_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "main completion type drift",
        ));
    }
    let ids_chirho = super::source_interfaces_chirho::entry_ids_chirho(&core_chirho)?;
    let binder_chirho = |id_chirho, name_chirho: &str, ty_chirho| BinderChirho {
        id_chirho,
        name_chirho: name_chirho.into(),
        ty_chirho,
        span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
    };
    let entry_chirho = binder_chirho(
        ids_chirho[0],
        "%next:io-entry-chirho",
        CoreTypeChirho::TupleChirho(Vec::new()),
    );
    let completion_binder_chirho = binder_chirho(
        ids_chirho[1],
        "%next:io-completion-chirho",
        *completion_chirho.clone(),
    );
    let payload_binder_chirho = binder_chirho(
        ids_chirho[2],
        "%next:io-lazy-result-chirho",
        *payload_chirho.clone(),
    );
    let rhs_chirho = CoreExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(main_chirho.id_chirho)),
            arg_chirho: Box::new(CoreExprChirho::ConAppChirho {
                con_name_chirho: WORLD_KEY_CHIRHO.into(),
                args_chirho: Vec::new(),
            }),
        }),
        bind_chirho: completion_binder_chirho,
        result_ty_chirho: CoreTypeChirho::TupleChirho(Vec::new()),
        alts_chirho: vec![CoreAltChirho {
            con_chirho: AltConChirho::DataConChirho(RESULT_KEY_CHIRHO.into()),
            binders_chirho: vec![payload_binder_chirho],
            rhs_chirho: CoreExprChirho::ConAppChirho {
                con_name_chirho: "()".into(),
                args_chirho: Vec::new(),
            },
        }],
    };
    core_chirho.bindings_chirho.push(CoreBindingChirho {
        binder_chirho: entry_chirho.clone(),
        rhs_chirho,
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    });
    // Bypass legacy prepare_io_actions and final force_to_prim. All IO execution
    // and result demand is already explicit in this checked Core entry.
    let program_chirho =
        crate::stg_lower_chirho::lower_module_to_stg_chirho(&core_chirho, Default::default());
    let address_chirho = *program_chirho
        .top_level_env_chirho
        .get(&entry_chirho.id_chirho)
        .ok_or_else(|| {
            error_chirho(
                NextErrorKindChirho::UnsupportedChirho,
                "IO entry has no STG closure",
            )
        })?;
    let mut machine_chirho = MachineChirho::new_chirho(program_chirho.code_table_chirho);
    machine_chirho.heap_chirho = program_chirho.initial_heap_chirho;
    machine_chirho.con_tags_chirho = program_chirho.con_tags_chirho;
    machine_chirho.step_limit_chirho = limits_chirho.steps_chirho;
    machine_chirho.next_io_chirho =
        haskelujah_runtime_chirho::io_chirho::OutputStateChirho::with_limit_chirho(
            limits_chirho.output_bytes_chirho,
        );
    let code_chirho = machine_chirho.code_table_chirho.len() as u32;
    machine_chirho
        .code_table_chirho
        .push(CodeChirho::EnterChirho(address_chirho));
    let result_chirho = machine_chirho.run_chirho(code_chirho);
    machine_chirho.next_io_chirho.discard_pending_chirho();
    let result_chirho = if machine_chirho.next_io_chirho.exhausted_chirho()
        || machine_chirho.integer_budget_chirho.exhausted_chirho()
    {
        Err(error_chirho(
            NextErrorKindChirho::ResourceChirho,
            "next IO or numeric allowance exhausted",
        ))
    } else {
        result_chirho.map(|_| ()).map_err(|failure_chirho| {
            let kind_chirho = match failure_chirho {
                EvalErrorChirho::StepLimitChirho { .. } => NextErrorKindChirho::ResourceChirho,
                EvalErrorChirho::PrimFailChirho(PrimErrorChirho::NextIoChirho(
                    IoErrorChirho::ContractChirho,
                )) => NextErrorKindChirho::UnsupportedChirho,
                _ => NextErrorKindChirho::ExecutionChirho,
            };
            NextErrorChirho::new_chirho(kind_chirho, failure_chirho.to_string())
        })
    };
    Ok(MainOutcomeChirho {
        result_chirho,
        machine_chirho,
    })
}

#[cfg(test)]
mod tests_chirho;

#[cfg(test)]
mod sequencing_tests_chirho;
