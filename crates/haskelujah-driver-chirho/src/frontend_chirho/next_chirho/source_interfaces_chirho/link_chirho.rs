// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::rename_chirho::RenamerChirho;
use super::*;
use haskelujah_core_chirho::CoreExprChirho;

fn fresh_chirho(next_chirho: &mut u32) -> Result<CoreIdChirho, NextErrorChirho> {
    let id_chirho = CoreIdChirho(*next_chirho);
    *next_chirho = next_chirho
        .checked_add(1)
        .ok_or_else(|| resource_chirho("linked Core id overflow"))?;
    Ok(id_chirho)
}
fn charge_chirho(left_chirho: &mut usize) -> Result<(), NextErrorChirho> {
    *left_chirho = left_chirho
        .checked_sub(1)
        .ok_or_else(|| resource_chirho("linked Core allocation ceiling exhausted"))?;
    Ok(())
}

fn collect_binders_chirho(
    expression_chirho: &CoreExprChirho,
    ids_chirho: &mut BTreeSet<CoreIdChirho>,
    left_chirho: &mut usize,
) -> Result<(), NextErrorChirho> {
    let mut work_chirho = vec![expression_chirho];
    while let Some(expression_chirho) = work_chirho.pop() {
        charge_chirho(left_chirho)?;
        match expression_chirho {
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                ids_chirho.insert(binder_chirho.id_chirho);
                work_chirho.push(body_chirho);
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. }
            | CoreExprChirho::TyAppChirho {
                expr_chirho: body_chirho,
                ..
            } => work_chirho.push(body_chirho),
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => work_chirho.extend([fun_chirho.as_ref(), arg_chirho.as_ref()]),
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                work_chirho.push(body_chirho);
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    ids_chirho.insert(binder_chirho.id_chirho);
                    work_chirho.push(rhs_chirho);
                }
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                ids_chirho.insert(bind_chirho.id_chirho);
                work_chirho.push(scrutinee_chirho);
                for alt_chirho in alts_chirho {
                    ids_chirho.extend(
                        alt_chirho
                            .binders_chirho
                            .iter()
                            .map(|binder_chirho| binder_chirho.id_chirho),
                    );
                    work_chirho.push(&alt_chirho.rhs_chirho);
                }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. }
            | CoreExprChirho::PrimOpChirho { args_chirho, .. } => work_chirho.extend(args_chirho),
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => {}
        }
    }
    Ok(())
}

pub(super) fn link_chirho(
    mut objects_chirho: Vec<ObjectChirho>,
    entry_chirho: &str,
) -> Result<CoreModuleChirho, NextErrorChirho> {
    let mut definitions_chirho = BTreeMap::new();
    let mut next_chirho = 0;
    let mut next_type_chirho = 0;
    let mut left_chirho = 4 << 20;
    for object_chirho in &objects_chirho {
        for original_chirho in object_chirho.definitions_chirho.values() {
            charge_chirho(&mut left_chirho)?;
            if definitions_chirho
                .insert(original_chirho.clone(), fresh_chirho(&mut next_chirho)?)
                .is_some()
            {
                return Err(invalid_chirho("duplicate original-name Core definition"));
            }
        }
    }
    let mut bindings_chirho = Vec::new();
    let mut names_chirho = std::collections::HashMap::new();
    for object_chirho in &mut objects_chirho {
        let mut ids_chirho = std::collections::HashMap::new();
        for (old_chirho, original_chirho) in &object_chirho.definitions_chirho {
            ids_chirho.insert(*old_chirho, definitions_chirho[original_chirho]);
        }
        for (old_chirho, original_chirho) in &object_chirho.imports_chirho {
            if let Some(new_chirho) = definitions_chirho.get(original_chirho) {
                ids_chirho.insert(*old_chirho, *new_chirho);
            }
        }
        let mut bound_chirho = BTreeSet::new();
        for binding_chirho in &object_chirho.core_chirho.bindings_chirho {
            bound_chirho.insert(binding_chirho.binder_chirho.id_chirho);
            collect_binders_chirho(
                &binding_chirho.rhs_chirho,
                &mut bound_chirho,
                &mut left_chirho,
            )?;
        }
        for old_chirho in bound_chirho {
            if let std::collections::hash_map::Entry::Vacant(entry_chirho) =
                ids_chirho.entry(old_chirho)
            {
                entry_chirho.insert(fresh_chirho(&mut next_chirho)?);
            }
        }
        let mut renamer_chirho =
            RenamerChirho::new_chirho(ids_chirho, &mut next_type_chirho, &mut left_chirho);
        for mut binding_chirho in std::mem::take(&mut object_chirho.core_chirho.bindings_chirho) {
            let original_chirho =
                &object_chirho.definitions_chirho[&binding_chirho.binder_chirho.id_chirho];
            renamer_chirho.binder_chirho(&mut binding_chirho.binder_chirho)?;
            renamer_chirho.expression_chirho(&mut binding_chirho.rhs_chirho, 0)?;
            if object_chirho.core_chirho.name_chirho != entry_chirho {
                binding_chirho.binder_chirho.name_chirho = original_chirho.link_key_chirho("value");
            }
            names_chirho.insert(
                binding_chirho.binder_chirho.id_chirho,
                binding_chirho.binder_chirho.name_chirho.clone(),
            );
            bindings_chirho.push(binding_chirho);
        }
    }
    Ok(CoreModuleChirho {
        name_chirho: entry_chirho.into(),
        bindings_chirho,
        names_chirho,
        specialize_pragmas_chirho: Default::default(),
        foreign_exports_chirho: Vec::new(),
        provenance_chirho: Default::default(),
    })
}
