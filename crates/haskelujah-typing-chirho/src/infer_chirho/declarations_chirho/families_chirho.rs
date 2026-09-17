// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-local checked family bodies, independent of merged reduction tables.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::*;
use haskelujah_ast_chirho::decl_chirho::TypeFamilyBodyChirho;
#[cfg(test)]
#[path = "family_tests_chirho.rs"]
mod tests_chirho;

#[derive(Debug, Clone)]
pub(super) enum FamilyContractChirho {
    OpenChirho,
    AbstractClosedChirho,
    ClosedChirho(Vec<SchemeChirho>),
}

/// Equation variables are implicitly quantified in occurrence order, not by
/// allocation ID or source spelling. Kind and visible inputs share one scope.
fn equation_scheme_chirho(
    equation_chirho: &TypeFamilyClauseChirho,
    budget_chirho: &mut usize,
) -> Option<SchemeChirho> {
    let mut vars_chirho = Vec::new();
    let mut seen_chirho = HashSet::new();
    let mut pending_chirho: Vec<_> = equation_chirho
        .kind_inputs_chirho
        .iter()
        .chain(&equation_chirho.type_inputs_chirho)
        .chain(std::iter::once(&equation_chirho.result_chirho))
        .map(|term_chirho| (term_chirho, 0usize))
        .collect();
    pending_chirho.reverse();
    while let Some((term_chirho, depth_chirho)) = pending_chirho.pop() {
        *budget_chirho = budget_chirho.checked_sub(1)?;
        if depth_chirho > 256 {
            return None;
        }
        let mut child_chirho = |term_chirho| pending_chirho.push((term_chirho, depth_chirho + 1));
        match term_chirho {
            TyChirho::VarChirho(identity_chirho) => {
                if seen_chirho.insert(*identity_chirho) {
                    vars_chirho.push(*identity_chirho);
                }
            }
            TyChirho::ConChirho(_) => {}
            TyChirho::AppChirho(left_chirho, right_chirho)
            | TyChirho::KindAppChirho(left_chirho, right_chirho)
            | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
                child_chirho(right_chirho.as_ref());
                child_chirho(left_chirho.as_ref());
            }
            TyChirho::ListChirho(element_chirho) => child_chirho(element_chirho.as_ref()),
            TyChirho::TupleChirho(elements_chirho) => {
                for element_chirho in elements_chirho.iter().rev() {
                    child_chirho(element_chirho);
                }
            }
            // Rank-n family equations are illegal, not opaque agreement proofs.
            TyChirho::ForallVarChirho(_)
            | TyChirho::ForallChirho { .. }
            | TyChirho::RequiredForallChirho { .. } => return None,
        }
    }
    Some(SchemeChirho {
        vars_chirho,
        preds_chirho: Vec::new(),
        ty_chirho: TyChirho::TupleChirho(vec![
            TyChirho::TupleChirho(equation_chirho.kind_inputs_chirho.clone()),
            TyChirho::TupleChirho(equation_chirho.type_inputs_chirho.clone()),
            equation_chirho.result_chirho.clone(),
        ]),
    })
}

impl DeclarationContractsChirho {
    pub(in crate::infer_chirho) fn record_family_chirho(
        &mut self,
        name_chirho: &str,
        body_chirho: &TypeFamilyBodyChirho,
        equations_chirho: &[TypeFamilyClauseChirho],
    ) -> Result<(), &'static str> {
        let contract_chirho = match body_chirho {
            TypeFamilyBodyChirho::OpenChirho => FamilyContractChirho::OpenChirho,
            TypeFamilyBodyChirho::AbstractClosedChirho => {
                FamilyContractChirho::AbstractClosedChirho
            }
            TypeFamilyBodyChirho::InvalidChirho => return Err("malformed type family body"),
            TypeFamilyBodyChirho::ClosedChirho {
                equations_chirho: source_chirho,
            } => {
                if source_chirho.len() != equations_chirho.len() {
                    return Err("family contract contains unchecked equations");
                }
                let mut budget_chirho = 16_384;
                let rows_chirho = equations_chirho
                    .iter()
                    .map(|equation_chirho| {
                        equation_scheme_chirho(equation_chirho, &mut budget_chirho)
                    })
                    .collect::<Option<Vec<_>>>()
                    .ok_or("family equation contract is unproved or exceeds its work bound")?;
                FamilyContractChirho::ClosedChirho(rows_chirho)
            }
        };
        self.families_chirho
            .insert(name_chirho.to_owned(), contract_chirho);
        Ok(())
    }

    pub fn check_boot_family_chirho(
        &self,
        name_chirho: &str,
        implementation_chirho: &Self,
    ) -> Result<(), String> {
        let expected_chirho = self.families_chirho.get(name_chirho);
        let actual_chirho = implementation_chirho.families_chirho.get(name_chirho);
        let agrees_chirho = match (expected_chirho, actual_chirho) {
            (Some(FamilyContractChirho::OpenChirho), Some(FamilyContractChirho::OpenChirho)) => {
                true
            }
            (
                Some(FamilyContractChirho::AbstractClosedChirho),
                Some(FamilyContractChirho::ClosedChirho(_)),
            ) => true,
            (
                Some(FamilyContractChirho::ClosedChirho(expected_chirho)),
                Some(FamilyContractChirho::ClosedChirho(actual_chirho)),
            ) => {
                expected_chirho.len() == actual_chirho.len()
                    && expected_chirho.iter().zip(actual_chirho).all(
                        |(expected_chirho, actual_chirho)| {
                            expected_chirho.alpha_equivalent_chirho(actual_chirho)
                        },
                    )
            }
            _ => false,
        };
        if agrees_chirho {
            Ok(())
        } else {
            Err(format!(
                "boot contract family body mismatch for {name_chirho}"
            ))
        }
    }
}
