// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The multi-module Core merge for native executables, with its provenance (rows 628, 634).
//! workflow: compiler-pipeline-chirho/native-symbol-identity-chirho

use super::CompileResultChirho;
use haskelujah_core_chirho::CoreModuleChirho;

pub(crate) fn merge_compile_results_core_chirho(
    results_chirho: &[CompileResultChirho],
) -> Result<CoreModuleChirho, String> {
    use haskelujah_core_chirho::expr_chirho::{CoreExprChirho, CoreIdChirho};
    use std::collections::{HashMap, HashSet};

    if results_chirho.is_empty() {
        return Err("no compiled modules to merge".to_string());
    }

    fn offset_id_chirho(id_chirho: CoreIdChirho, off_chirho: u32) -> CoreIdChirho {
        CoreIdChirho(id_chirho.0 + off_chirho)
    }

    fn offset_binder_chirho(
        binder_chirho: &mut haskelujah_core_chirho::expr_chirho::BinderChirho,
        off_chirho: u32,
    ) {
        binder_chirho.id_chirho = offset_id_chirho(binder_chirho.id_chirho, off_chirho);
    }

    fn offset_expr_chirho(expr_chirho: &mut CoreExprChirho, off_chirho: u32) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                *id_chirho = offset_id_chirho(*id_chirho, off_chirho);
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                offset_expr_chirho(fun_chirho, off_chirho);
                offset_expr_chirho(arg_chirho, off_chirho);
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                offset_binder_chirho(binder_chirho, off_chirho);
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    offset_binder_chirho(binder_chirho, off_chirho);
                    offset_expr_chirho(rhs_chirho, off_chirho);
                }
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                offset_binder_chirho(bind_chirho, off_chirho);
                offset_expr_chirho(scrutinee_chirho, off_chirho);
                for alt_chirho in alts_chirho {
                    for binder_chirho in &mut alt_chirho.binders_chirho {
                        offset_binder_chirho(binder_chirho, off_chirho);
                    }
                    offset_expr_chirho(&mut alt_chirho.rhs_chirho, off_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                offset_expr_chirho(inner_chirho, off_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    offset_expr_chirho(arg_chirho, off_chirho);
                }
            }
        }
    }

    fn remap_expr_chirho(
        expr_chirho: &mut CoreExprChirho,
        remap_chirho: &std::collections::HashMap<CoreIdChirho, CoreIdChirho>,
    ) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                if let Some(&new_id_chirho) = remap_chirho.get(id_chirho) {
                    *id_chirho = new_id_chirho;
                }
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                remap_expr_chirho(fun_chirho, remap_chirho);
                remap_expr_chirho(arg_chirho, remap_chirho);
            }
            CoreExprChirho::LamChirho { body_chirho, .. } => {
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (_binder_chirho, rhs_chirho) in binds_chirho {
                    remap_expr_chirho(rhs_chirho, remap_chirho);
                }
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                remap_expr_chirho(scrutinee_chirho, remap_chirho);
                for alt_chirho in alts_chirho {
                    remap_expr_chirho(&mut alt_chirho.rhs_chirho, remap_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                remap_expr_chirho(inner_chirho, remap_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    remap_expr_chirho(arg_chirho, remap_chirho);
                }
            }
        }
    }

    fn collect_binder_ids_chirho(
        expr_chirho: &CoreExprChirho,
        ids_chirho: &mut HashSet<CoreIdChirho>,
    ) {
        match expr_chirho {
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                collect_binder_ids_chirho(fun_chirho, ids_chirho);
                collect_binder_ids_chirho(arg_chirho, ids_chirho);
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                ids_chirho.insert(binder_chirho.id_chirho);
                collect_binder_ids_chirho(body_chirho, ids_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    ids_chirho.insert(binder_chirho.id_chirho);
                    collect_binder_ids_chirho(rhs_chirho, ids_chirho);
                }
                collect_binder_ids_chirho(body_chirho, ids_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                ids_chirho.insert(bind_chirho.id_chirho);
                collect_binder_ids_chirho(scrutinee_chirho, ids_chirho);
                for alt_chirho in alts_chirho {
                    for binder_chirho in &alt_chirho.binders_chirho {
                        ids_chirho.insert(binder_chirho.id_chirho);
                    }
                    collect_binder_ids_chirho(&alt_chirho.rhs_chirho, ids_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                collect_binder_ids_chirho(body_chirho, ids_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                collect_binder_ids_chirho(inner_chirho, ids_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    collect_binder_ids_chirho(arg_chirho, ids_chirho);
                }
            }
        }
    }

    fn canonical_binder_ty_chirho(
        binder_chirho: &haskelujah_core_chirho::expr_chirho::BinderChirho,
    ) -> String {
        binder_chirho.ty_chirho.to_string()
    }

    fn canonical_expr_chirho(
        expr_chirho: &CoreExprChirho,
        names_chirho: &HashMap<CoreIdChirho, String>,
        local_ids_chirho: &mut HashMap<CoreIdChirho, usize>,
        next_local_id_chirho: &mut usize,
    ) -> String {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                if let Some(local_id_chirho) = local_ids_chirho.get(id_chirho) {
                    format!("local:{local_id_chirho}")
                } else if let Some(name_chirho) = names_chirho.get(id_chirho) {
                    format!("global:{name_chirho}")
                } else {
                    format!("unresolved:{}", id_chirho.0)
                }
            }
            CoreExprChirho::LitChirho(lit_chirho) => format!("lit:{lit_chirho:?}"),
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => format!(
                "app({},{})",
                canonical_expr_chirho(
                    fun_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                ),
                canonical_expr_chirho(
                    arg_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                )
            ),
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                let local_slot_chirho = *next_local_id_chirho;
                *next_local_id_chirho += 1;
                let previous_local_chirho =
                    local_ids_chirho.insert(binder_chirho.id_chirho, local_slot_chirho);
                let body_key_chirho = canonical_expr_chirho(
                    body_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                );
                if let Some(prev_local_chirho) = previous_local_chirho {
                    local_ids_chirho.insert(binder_chirho.id_chirho, prev_local_chirho);
                } else {
                    local_ids_chirho.remove(&binder_chirho.id_chirho);
                }
                format!(
                    "lam({}:{})",
                    canonical_binder_ty_chirho(binder_chirho),
                    body_key_chirho
                )
            }
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => {
                let let_binder_ids_chirho: Vec<CoreIdChirho> = binds_chirho
                    .iter()
                    .map(|(binder_chirho, _)| binder_chirho.id_chirho)
                    .collect();
                let mut previous_ids_chirho = Vec::with_capacity(let_binder_ids_chirho.len());
                for binder_id_chirho in &let_binder_ids_chirho {
                    let local_slot_chirho = *next_local_id_chirho;
                    *next_local_id_chirho += 1;
                    previous_ids_chirho.push((
                        *binder_id_chirho,
                        local_ids_chirho.insert(*binder_id_chirho, local_slot_chirho),
                    ));
                }
                let binds_key_chirho = binds_chirho
                    .iter()
                    .map(|(binder_chirho, rhs_chirho)| {
                        format!(
                            "{}={}",
                            canonical_binder_ty_chirho(binder_chirho),
                            canonical_expr_chirho(
                                rhs_chirho,
                                names_chirho,
                                local_ids_chirho,
                                next_local_id_chirho,
                            )
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(";");
                let body_key_chirho = canonical_expr_chirho(
                    body_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                );
                for (binder_id_chirho, previous_local_chirho) in previous_ids_chirho {
                    if let Some(prev_local_chirho) = previous_local_chirho {
                        local_ids_chirho.insert(binder_id_chirho, prev_local_chirho);
                    } else {
                        local_ids_chirho.remove(&binder_id_chirho);
                    }
                }
                format!("let(rec={rec_chirho};binds=[{binds_key_chirho}];body={body_key_chirho})")
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                result_ty_chirho,
                alts_chirho,
            } => {
                let scrut_key_chirho = canonical_expr_chirho(
                    scrutinee_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                );
                let bind_slot_chirho = *next_local_id_chirho;
                *next_local_id_chirho += 1;
                let previous_bind_chirho =
                    local_ids_chirho.insert(bind_chirho.id_chirho, bind_slot_chirho);
                let alt_keys_chirho = alts_chirho
                    .iter()
                    .map(|alt_chirho| {
                        let alt_binder_ids_chirho: Vec<CoreIdChirho> = alt_chirho
                            .binders_chirho
                            .iter()
                            .map(|binder_chirho| binder_chirho.id_chirho)
                            .collect();
                        let mut previous_alt_ids_chirho =
                            Vec::with_capacity(alt_binder_ids_chirho.len());
                        for binder_id_chirho in &alt_binder_ids_chirho {
                            let local_slot_chirho = *next_local_id_chirho;
                            *next_local_id_chirho += 1;
                            previous_alt_ids_chirho.push((
                                *binder_id_chirho,
                                local_ids_chirho.insert(*binder_id_chirho, local_slot_chirho),
                            ));
                        }
                        let binders_key_chirho = alt_chirho
                            .binders_chirho
                            .iter()
                            .map(canonical_binder_ty_chirho)
                            .collect::<Vec<_>>()
                            .join(",");
                        let rhs_key_chirho = canonical_expr_chirho(
                            &alt_chirho.rhs_chirho,
                            names_chirho,
                            local_ids_chirho,
                            next_local_id_chirho,
                        );
                        for (binder_id_chirho, previous_local_chirho) in previous_alt_ids_chirho {
                            if let Some(prev_local_chirho) = previous_local_chirho {
                                local_ids_chirho.insert(binder_id_chirho, prev_local_chirho);
                            } else {
                                local_ids_chirho.remove(&binder_id_chirho);
                            }
                        }
                        format!(
                            "alt({:?};{};{})",
                            alt_chirho.con_chirho, binders_key_chirho, rhs_key_chirho
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(";");
                if let Some(prev_local_chirho) = previous_bind_chirho {
                    local_ids_chirho.insert(bind_chirho.id_chirho, prev_local_chirho);
                } else {
                    local_ids_chirho.remove(&bind_chirho.id_chirho);
                }
                format!(
                    "case({};{};{};alts=[{}])",
                    scrut_key_chirho, bind_slot_chirho, result_ty_chirho, alt_keys_chirho
                )
            }
            CoreExprChirho::TyLamChirho {
                ty_var_chirho,
                body_chirho,
            } => format!(
                "tylam({};{})",
                ty_var_chirho,
                canonical_expr_chirho(
                    body_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                )
            ),
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
            } => format!(
                "tyapp({};{})",
                canonical_expr_chirho(
                    inner_chirho,
                    names_chirho,
                    local_ids_chirho,
                    next_local_id_chirho,
                ),
                ty_chirho
            ),
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => format!(
                "primop({};[{}])",
                name_chirho,
                args_chirho
                    .iter()
                    .map(|arg_chirho| canonical_expr_chirho(
                        arg_chirho,
                        names_chirho,
                        local_ids_chirho,
                        next_local_id_chirho,
                    ))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } => format!(
                "con({};[{}])",
                con_name_chirho,
                args_chirho
                    .iter()
                    .map(|arg_chirho| canonical_expr_chirho(
                        arg_chirho,
                        names_chirho,
                        local_ids_chirho,
                        next_local_id_chirho,
                    ))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }

    fn canonical_binding_key_chirho(
        binding_chirho: &haskelujah_core_chirho::expr_chirho::CoreBindingChirho,
        names_chirho: &HashMap<CoreIdChirho, String>,
    ) -> String {
        let mut local_ids_chirho = HashMap::new();
        let mut next_local_id_chirho = 0usize;
        format!(
            "binding({};{};rec={};inline={:?};{})",
            binding_chirho.binder_chirho.name_chirho,
            canonical_binder_ty_chirho(&binding_chirho.binder_chirho),
            binding_chirho.is_rec_chirho,
            binding_chirho.inline_chirho,
            canonical_expr_chirho(
                &binding_chirho.rhs_chirho,
                names_chirho,
                &mut local_ids_chirho,
                &mut next_local_id_chirho,
            )
        )
    }

    let mut merged_bindings_chirho = Vec::new();
    let mut merged_names_chirho: HashMap<CoreIdChirho, String> = HashMap::new();
    let mut id_offset_chirho: u32 = 0;
    // Each module's provenance moves with its ids. Only the program's root module, `Main`,
    // supplies the entry (row 628).
    // workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
    let mut merged_provenance_chirho = haskelujah_core_chirho::CoreProvenanceChirho::default();

    for result_chirho in results_chirho {
        let moved_provenance_chirho = result_chirho
            .core_chirho
            .provenance_chirho
            .offset_chirho(id_offset_chirho);
        merged_provenance_chirho.absorb_chirho(&moved_provenance_chirho);
        if result_chirho.module_chirho.name_chirho.full_name_chirho() == "Main" {
            merged_provenance_chirho.entry_chirho = moved_provenance_chirho.entry_chirho;
        }
        let mut bindings_chirho = result_chirho.core_chirho.bindings_chirho.clone();
        if id_offset_chirho > 0 {
            for binding_chirho in &mut bindings_chirho {
                offset_binder_chirho(&mut binding_chirho.binder_chirho, id_offset_chirho);
                offset_expr_chirho(&mut binding_chirho.rhs_chirho, id_offset_chirho);
            }
        }
        for (id_chirho, name_chirho) in &result_chirho.core_chirho.names_chirho {
            merged_names_chirho.insert(
                offset_id_chirho(*id_chirho, id_offset_chirho),
                name_chirho.clone(),
            );
        }
        merged_bindings_chirho.extend(bindings_chirho);
        let max_id_chirho = result_chirho
            .core_chirho
            .names_chirho
            .keys()
            .map(|id_chirho| id_chirho.0)
            .max()
            .unwrap_or(0);
        id_offset_chirho += max_id_chirho + 1;
    }

    let mut def_name_to_id_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
    // A builtin reference links only to the compiler's own body of its name, never to a
    // program's binding that shares the spelling (row 628).
    let mut compiler_name_to_id_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
    for binding_chirho in &merged_bindings_chirho {
        let binder_chirho = &binding_chirho.binder_chirho;
        def_name_to_id_chirho
            .entry(binder_chirho.name_chirho.clone())
            .or_insert(binder_chirho.id_chirho);
        if merged_provenance_chirho
            .compiler_bindings_chirho
            .contains(&binder_chirho.id_chirho)
        {
            compiler_name_to_id_chirho
                .entry(binder_chirho.name_chirho.clone())
                .or_insert(binder_chirho.id_chirho);
        }
    }

    let mut all_binder_ids_chirho: HashSet<CoreIdChirho> = HashSet::new();
    for binding_chirho in &merged_bindings_chirho {
        all_binder_ids_chirho.insert(binding_chirho.binder_chirho.id_chirho);
        collect_binder_ids_chirho(&binding_chirho.rhs_chirho, &mut all_binder_ids_chirho);
    }

    let mut remap_chirho: HashMap<CoreIdChirho, CoreIdChirho> = HashMap::new();
    for (id_chirho, name_chirho) in &merged_names_chirho {
        if all_binder_ids_chirho.contains(id_chirho) {
            continue;
        }
        let definitions_chirho = if merged_provenance_chirho
            .builtin_references_chirho
            .contains(id_chirho)
        {
            &compiler_name_to_id_chirho
        } else {
            &def_name_to_id_chirho
        };
        if let Some(&def_id_chirho) = definitions_chirho.get(name_chirho)
            && def_id_chirho != *id_chirho
        {
            remap_chirho.insert(*id_chirho, def_id_chirho);
        }
    }

    if !remap_chirho.is_empty() {
        for binding_chirho in &mut merged_bindings_chirho {
            remap_expr_chirho(&mut binding_chirho.rhs_chirho, &remap_chirho);
        }
    }

    let mut canonical_binding_to_id_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
    let mut deduped_bindings_chirho = Vec::with_capacity(merged_bindings_chirho.len());
    let mut removed_binding_ids_chirho = HashSet::new();
    let mut duplicate_binding_remap_chirho: HashMap<CoreIdChirho, CoreIdChirho> = HashMap::new();
    for binding_chirho in merged_bindings_chirho {
        let canonical_key_chirho =
            canonical_binding_key_chirho(&binding_chirho, &merged_names_chirho);
        if let Some(&kept_id_chirho) = canonical_binding_to_id_chirho.get(&canonical_key_chirho) {
            duplicate_binding_remap_chirho
                .insert(binding_chirho.binder_chirho.id_chirho, kept_id_chirho);
            removed_binding_ids_chirho.insert(binding_chirho.binder_chirho.id_chirho);
        } else {
            canonical_binding_to_id_chirho
                .insert(canonical_key_chirho, binding_chirho.binder_chirho.id_chirho);
            deduped_bindings_chirho.push(binding_chirho);
        }
    }

    if !duplicate_binding_remap_chirho.is_empty() {
        for binding_chirho in &mut deduped_bindings_chirho {
            remap_expr_chirho(
                &mut binding_chirho.rhs_chirho,
                &duplicate_binding_remap_chirho,
            );
        }
        merged_names_chirho.retain(|id_chirho, _| !removed_binding_ids_chirho.contains(id_chirho));
        merged_provenance_chirho.remap_chirho(&duplicate_binding_remap_chirho);
    }

    Ok(CoreModuleChirho {
        name_chirho: results_chirho
            .last()
            .map(|result_chirho| {
                result_chirho
                    .module_chirho
                    .name_chirho
                    .text_chirho()
                    .to_string()
            })
            .unwrap_or_else(|| "Main".to_string()),
        bindings_chirho: deduped_bindings_chirho,
        names_chirho: merged_names_chirho,
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho: merged_provenance_chirho,
    })
}
