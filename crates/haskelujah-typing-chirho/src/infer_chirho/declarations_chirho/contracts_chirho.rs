// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-local declaration contracts, never the merged instance inventory.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::*;
use crate::kind_chirho::KindContractChirho;

#[cfg(test)]
#[path = "contract_tests_chirho.rs"]
mod tests_chirho;

const INSTANCE_COMPARISON_LIMIT_CHIRHO: usize = 16_384;

fn ordered_entries_chirho<TChirho>(
    entries_chirho: &HashMap<String, TChirho>,
) -> Vec<(&String, &TChirho)> {
    let mut ordered_chirho: Vec<_> = entries_chirho.iter().collect();
    ordered_chirho.sort_unstable_by(|left_chirho, right_chirho| left_chirho.0.cmp(right_chirho.0));
    ordered_chirho
}

#[derive(Debug, Clone)]
pub(super) struct ClassContractChirho {
    pub(super) abstract_chirho: bool,
    pub(super) fundeps_chirho: Option<Vec<(Vec<usize>, Vec<usize>)>>,
}

#[derive(Debug, Clone, Default)]
pub struct DeclarationContractsChirho {
    kinds_chirho: HashMap<String, KindContractChirho>,
    pub(super) classes_chirho: HashMap<String, ClassContractChirho>,
    pub(super) instances_chirho: HashMap<String, Vec<SchemeChirho>>,
    pub(super) unproved_instances_chirho: HashSet<String>,
}

impl DeclarationContractsChirho {
    /// Attach the kind checker's source-local heads before export dependency
    /// closure adds imported types or removes unused private declarations.
    /// Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
    pub fn set_local_kind_contracts_chirho(
        &mut self,
        kinds_chirho: HashMap<String, KindContractChirho>,
    ) {
        self.kinds_chirho = kinds_chirho;
    }

    pub fn supports_abstract_boot_class_chirho(&self, name_chirho: &str) -> bool {
        self.classes_chirho
            .get(name_chirho)
            .is_some_and(|class_chirho| {
                class_chirho.abstract_chirho && class_chirho.fundeps_chirho.is_some()
            })
    }

    /// Check source-local kind, class and instance promises. Exports and
    /// declaration bodies are checked by their own contract owners.
    pub fn check_boot_promises_chirho(&self, implementation_chirho: &Self) -> Result<(), String> {
        for (name_chirho, expected_chirho) in ordered_entries_chirho(&self.kinds_chirho) {
            let actual_chirho = implementation_chirho
                .kinds_chirho
                .get(name_chirho)
                .ok_or_else(|| {
                    format!("boot contract type {name_chirho} has no local implementation")
                })?;
            expected_chirho
                .check_agreement_chirho(actual_chirho)
                .map_err(|reason_chirho| {
                    format!("boot contract mismatch for {name_chirho}: {reason_chirho}")
                })?;
        }
        let mut budget_chirho = INSTANCE_COMPARISON_LIMIT_CHIRHO;
        for (name_chirho, expected_chirho) in ordered_entries_chirho(&self.classes_chirho) {
            if !expected_chirho.abstract_chirho {
                return Err(format!(
                    "boot contract full class agreement is not represented: {name_chirho}"
                ));
            }
            let Some(actual_chirho) = implementation_chirho.classes_chirho.get(name_chirho) else {
                return Err(format!(
                    "boot contract class {name_chirho} has no local implementation"
                ));
            };
            if expected_chirho.fundeps_chirho.is_none()
                || expected_chirho.fundeps_chirho != actual_chirho.fundeps_chirho
            {
                return Err(format!(
                    "boot contract class {name_chirho} has different functional dependencies"
                ));
            }
        }
        if let Some(name_chirho) = self.unproved_instances_chirho.iter().min() {
            return Err(format!(
                "boot contract instance agreement is not represented: {name_chirho}"
            ));
        }
        for (name_chirho, expected_chirho) in ordered_entries_chirho(&self.instances_chirho) {
            for promise_chirho in expected_chirho {
                let mut found_chirho = false;
                for actual_chirho in implementation_chirho
                    .instances_chirho
                    .get(name_chirho)
                    .into_iter()
                    .flatten()
                {
                    budget_chirho = budget_chirho
                        .checked_sub(1)
                        .ok_or("boot contract instance comparison budget exhausted")?;
                    if promise_chirho.alpha_equivalent_chirho(actual_chirho) {
                        found_chirho = true;
                        break;
                    }
                }
                if !found_chirho {
                    return Err(format!(
                        "boot contract instance {name_chirho} has no matching local implementation"
                    ));
                }
            }
        }
        Ok(())
    }
}
