// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Bounded semantic MINIMAL implication. Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use haskelujah_ast_chirho::class_chirho::MinimalFormulaChirho;
use std::collections::{BTreeSet, HashMap};
const LIMIT_CHIRHO: usize = 16_384;
fn names_chirho(
    formula_chirho: &MinimalFormulaChirho,
    budget_chirho: &mut usize,
) -> Result<BTreeSet<String>, &'static str> {
    let mut names_chirho = BTreeSet::new();
    let mut pending_chirho = vec![(formula_chirho, 0usize)];
    while let Some((node_chirho, depth_chirho)) = pending_chirho.pop() {
        *budget_chirho = budget_chirho
            .checked_sub(1)
            .ok_or("MINIMAL comparison budget exhausted")?;
        if depth_chirho > 128 {
            return Err("MINIMAL nesting limit exceeded");
        }
        match node_chirho {
            MinimalFormulaChirho::MethodChirho(name_chirho) => {
                names_chirho.insert(name_chirho.clone());
            }
            MinimalFormulaChirho::AllChirho(children_chirho)
            | MinimalFormulaChirho::AnyChirho(children_chirho) => {
                if children_chirho.len() > *budget_chirho {
                    return Err("MINIMAL comparison budget exhausted");
                }
                pending_chirho.extend(
                    children_chirho
                        .iter()
                        .map(|child_chirho| (child_chirho, depth_chirho + 1)),
                );
            }
            MinimalFormulaChirho::InvalidChirho => return Err("invalid MINIMAL formula"),
        }
    }
    Ok(names_chirho)
}
pub(super) fn validate_chirho(
    formula_chirho: &MinimalFormulaChirho,
    methods_chirho: &BTreeSet<String>,
) -> Result<(), &'static str> {
    let mut budget_chirho = LIMIT_CHIRHO;
    if names_chirho(formula_chirho, &mut budget_chirho)?.is_subset(methods_chirho) {
        Ok(())
    } else {
        Err("MINIMAL formula names an undeclared method")
    }
}
fn evaluate_chirho(
    formula_chirho: &MinimalFormulaChirho,
    assignment_chirho: &HashMap<&str, bool>,
    budget_chirho: &mut usize,
) -> Result<Option<bool>, &'static str> {
    *budget_chirho = budget_chirho
        .checked_sub(1)
        .ok_or("MINIMAL comparison budget exhausted")?;
    Ok(match formula_chirho {
        MinimalFormulaChirho::MethodChirho(name_chirho) => {
            assignment_chirho.get(name_chirho.as_str()).copied()
        }
        MinimalFormulaChirho::AllChirho(children_chirho)
        | MinimalFormulaChirho::AnyChirho(children_chirho) => {
            let conjunction_chirho = matches!(formula_chirho, MinimalFormulaChirho::AllChirho(_));
            let mut unresolved_chirho = false;
            for child_chirho in children_chirho {
                match evaluate_chirho(child_chirho, assignment_chirho, budget_chirho)? {
                    Some(value_chirho) if value_chirho != conjunction_chirho => {
                        return Ok(Some(value_chirho));
                    }
                    None => unresolved_chirho = true,
                    _ => {}
                }
            }
            if unresolved_chirho {
                None
            } else {
                Some(conjunction_chirho)
            }
        }
        MinimalFormulaChirho::InvalidChirho => return Err("invalid MINIMAL formula"),
    })
}
fn search_chirho<'names_chirho>(
    boot_chirho: &MinimalFormulaChirho,
    implementation_chirho: &MinimalFormulaChirho,
    names_chirho: &'names_chirho [String],
    assignment_chirho: &mut HashMap<&'names_chirho str, bool>,
    index_chirho: usize,
    budget_chirho: &mut usize,
) -> Result<bool, &'static str> {
    let promised_chirho = evaluate_chirho(boot_chirho, assignment_chirho, budget_chirho)?;
    let required_chirho = evaluate_chirho(implementation_chirho, assignment_chirho, budget_chirho)?;
    if promised_chirho == Some(false) || required_chirho == Some(true) {
        return Ok(true);
    }
    if promised_chirho == Some(true) && required_chirho == Some(false) {
        return Ok(false);
    }
    if index_chirho >= 128 {
        return Err("MINIMAL decision depth exceeded");
    }
    let name_chirho = names_chirho
        .get(index_chirho)
        .ok_or("unresolved MINIMAL decision")?
        .as_str();
    for value_chirho in [false, true] {
        assignment_chirho.insert(name_chirho, value_chirho);
        if !search_chirho(
            boot_chirho,
            implementation_chirho,
            names_chirho,
            assignment_chirho,
            index_chirho + 1,
            budget_chirho,
        )? {
            return Ok(false);
        }
    }
    assignment_chirho.remove(name_chirho);
    Ok(true)
}
pub(super) fn implies_chirho(
    boot_chirho: &MinimalFormulaChirho,
    implementation_chirho: &MinimalFormulaChirho,
) -> Result<bool, &'static str> {
    let mut budget_chirho = LIMIT_CHIRHO;
    let mut names_chirho = names_chirho(boot_chirho, &mut budget_chirho)?;
    names_chirho.extend(self::names_chirho(
        implementation_chirho,
        &mut budget_chirho,
    )?);
    if boot_chirho == implementation_chirho {
        return Ok(true);
    }
    search_chirho(
        boot_chirho,
        implementation_chirho,
        &names_chirho.into_iter().collect::<Vec<_>>(),
        &mut HashMap::new(),
        0,
        &mut budget_chirho,
    )
}
#[cfg(test)]
#[path = "minimal_tests_chirho.rs"]
mod tests_chirho;
