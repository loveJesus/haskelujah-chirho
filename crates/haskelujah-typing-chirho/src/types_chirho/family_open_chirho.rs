// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Open rows are unordered only after their complete input spines are compatible.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::{FamilyReductionChirho, FamilyTermChirho, reduce_one_equation_chirho};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OpenFamilyErrorChirho {
    ConflictChirho,
    UnprovedChirho,
}

#[derive(Clone, Debug)]
pub(crate) struct CompatibleOpenRowsChirho<TermChirho> {
    equations_chirho: Vec<(Vec<TermChirho>, TermChirho)>,
}

impl<TermChirho: FamilyTermChirho> CompatibleOpenRowsChirho<TermChirho> {
    pub(crate) fn check_chirho(
        equations_chirho: Vec<(Vec<TermChirho>, TermChirho)>,
        family_chirho: &impl Fn(&str) -> bool,
        budget_chirho: &mut usize,
    ) -> Result<Self, OpenFamilyErrorChirho> {
        let arity_chirho = equations_chirho
            .first()
            .map(|row_chirho| row_chirho.0.len());
        for (patterns_chirho, result_chirho) in &equations_chirho {
            if Some(patterns_chirho.len()) != arity_chirho {
                return Err(OpenFamilyErrorChirho::UnprovedChirho);
            }
            let mut bound_chirho = HashSet::new();
            for pattern_chirho in patterns_chirho {
                collect_variables_chirho(
                    pattern_chirho,
                    &mut bound_chirho,
                    true,
                    family_chirho,
                    budget_chirho,
                )?;
            }
            let mut result_variables_chirho = HashSet::new();
            collect_variables_chirho(
                result_chirho,
                &mut result_variables_chirho,
                false,
                family_chirho,
                budget_chirho,
            )?;
            if !result_variables_chirho.is_subset(&bound_chirho) {
                return Err(OpenFamilyErrorChirho::UnprovedChirho);
            }
        }
        for (index_chirho, later_chirho) in equations_chirho.iter().enumerate() {
            for earlier_chirho in &equations_chirho[..index_chirho] {
                spend_chirho(budget_chirho)?;
                compatible_pair_chirho(earlier_chirho, later_chirho, family_chirho, budget_chirho)?;
            }
        }
        Ok(Self { equations_chirho })
    }

    /// A stuck row cannot shadow a matching compatible row. The private
    /// collection constructor prevents unvalidated source order from doing so.
    pub(crate) fn reduce_chirho(
        &self,
        arguments_chirho: &[TermChirho],
        family_chirho: &impl Fn(&str) -> bool,
        budget_chirho: &mut usize,
    ) -> FamilyReductionChirho<TermChirho> {
        let mut stuck_chirho = false;
        for (patterns_chirho, result_chirho) in &self.equations_chirho {
            if spend_chirho(budget_chirho).is_err() {
                return FamilyReductionChirho::LimitedChirho;
            }
            if patterns_chirho.len() != arguments_chirho.len() {
                return FamilyReductionChirho::StuckChirho;
            }
            match reduce_one_equation_chirho(
                patterns_chirho.iter(),
                result_chirho,
                arguments_chirho,
                family_chirho,
                budget_chirho,
            ) {
                FamilyReductionChirho::ApartChirho => {}
                FamilyReductionChirho::StuckChirho => stuck_chirho = true,
                outcome_chirho => return outcome_chirho,
            }
        }
        if stuck_chirho {
            FamilyReductionChirho::StuckChirho
        } else {
            FamilyReductionChirho::ApartChirho
        }
    }

    pub(crate) fn arity_chirho(&self) -> Option<usize> {
        self.equations_chirho
            .first()
            .map(|row_chirho| row_chirho.0.len())
    }
}

fn spend_chirho(budget_chirho: &mut usize) -> Result<(), OpenFamilyErrorChirho> {
    *budget_chirho = budget_chirho
        .checked_sub(1)
        .ok_or(OpenFamilyErrorChirho::UnprovedChirho)?;
    Ok(())
}

fn collect_variables_chirho<TermChirho: FamilyTermChirho>(
    term_chirho: &TermChirho,
    variables_chirho: &mut HashSet<TermChirho::VariableChirho>,
    pattern_chirho: bool,
    family_chirho: &impl Fn(&str) -> bool,
    budget_chirho: &mut usize,
) -> Result<(), OpenFamilyErrorChirho> {
    let mut pending_chirho = vec![(term_chirho, 0usize)];
    while let Some((term_chirho, depth_chirho)) = pending_chirho.pop() {
        spend_chirho(budget_chirho)?;
        if depth_chirho >= 128 {
            return Err(OpenFamilyErrorChirho::UnprovedChirho);
        }
        if let Some(variable_chirho) = term_chirho.variable_chirho() {
            variables_chirho.insert(variable_chirho);
        } else if term_chirho.unknown_chirho()
            || (pattern_chirho && term_chirho.head_name_chirho().is_some_and(family_chirho))
        {
            return Err(OpenFamilyErrorChirho::UnprovedChirho);
        }
        if let Some((_, children_chirho)) = term_chirho.parts_chirho() {
            if children_chirho.len_chirho() > *budget_chirho {
                *budget_chirho = 0;
                return Err(OpenFamilyErrorChirho::UnprovedChirho);
            }
            pending_chirho.extend(
                children_chirho
                    .into_iter()
                    .map(|child_chirho| (child_chirho, depth_chirho + 1)),
            );
        }
    }
    Ok(())
}

#[derive(Clone)]
enum ShapeChirho<'term_chirho, TermChirho> {
    VariableChirho,
    AtomChirho(&'term_chirho TermChirho),
    StructuredChirho(&'static str, Vec<usize>),
    OpaqueChirho,
}

struct RationalGraphChirho<'term_chirho, TermChirho: FamilyTermChirho> {
    parents_chirho: Vec<usize>,
    sizes_chirho: Vec<usize>,
    shapes_chirho: Vec<ShapeChirho<'term_chirho, TermChirho>>,
    // The same numeric/named variable in different equations is not one binder.
    variables_chirho: HashMap<(bool, TermChirho::VariableChirho), usize>,
}

impl<'term_chirho, TermChirho: FamilyTermChirho> RationalGraphChirho<'term_chirho, TermChirho> {
    fn new_chirho() -> Self {
        Self {
            parents_chirho: Vec::new(),
            sizes_chirho: Vec::new(),
            shapes_chirho: Vec::new(),
            variables_chirho: HashMap::new(),
        }
    }

    fn import_chirho(
        &mut self,
        term_chirho: &'term_chirho TermChirho,
        side_chirho: bool,
        rhs_chirho: bool,
        family_chirho: &impl Fn(&str) -> bool,
        budget_chirho: &mut usize,
        depth_chirho: usize,
    ) -> Result<usize, OpenFamilyErrorChirho> {
        spend_chirho(budget_chirho)?;
        if depth_chirho >= 128 {
            return Err(OpenFamilyErrorChirho::UnprovedChirho);
        }
        let variable_chirho = term_chirho.variable_chirho();
        if let Some(variable_chirho) = &variable_chirho
            && let Some(index_chirho) = self
                .variables_chirho
                .get(&(side_chirho, variable_chirho.clone()))
        {
            return Ok(*index_chirho);
        }
        let shape_chirho = if variable_chirho.is_some() {
            ShapeChirho::VariableChirho
        } else if term_chirho.unknown_chirho()
            || (!rhs_chirho && term_chirho.head_name_chirho().is_some_and(family_chirho))
        {
            // Family-headed inputs do not authorize nominal apartness. RHS
            // equality is read-only: identical stuck applications may agree
            // without being reduced or cancelling their argument structure.
            ShapeChirho::OpaqueChirho
        } else if let Some((tag_chirho, children_chirho)) = term_chirho.parts_chirho() {
            if children_chirho.len_chirho() > *budget_chirho {
                *budget_chirho = 0;
                return Err(OpenFamilyErrorChirho::UnprovedChirho);
            }
            let children_chirho = children_chirho
                .into_iter()
                .map(|child_chirho| {
                    self.import_chirho(
                        child_chirho,
                        side_chirho,
                        rhs_chirho,
                        family_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            ShapeChirho::StructuredChirho(tag_chirho, children_chirho)
        } else {
            ShapeChirho::AtomChirho(term_chirho)
        };
        let index_chirho = self.parents_chirho.len();
        self.parents_chirho.push(index_chirho);
        self.sizes_chirho.push(1);
        self.shapes_chirho.push(shape_chirho);
        if let Some(variable_chirho) = variable_chirho {
            self.variables_chirho
                .insert((side_chirho, variable_chirho), index_chirho);
        }
        Ok(index_chirho)
    }

    fn root_chirho(&self, mut index_chirho: usize) -> usize {
        while self.parents_chirho[index_chirho] != index_chirho {
            index_chirho = self.parents_chirho[index_chirho];
        }
        index_chirho
    }

    /// Only LHS constraints merge classes. Omitting an occurs check permits a
    /// finite graph to represent x = [x], rather than misclassifying it as apart.
    fn unify_chirho(
        &mut self,
        mut pending_chirho: Vec<(usize, usize)>,
        budget_chirho: &mut usize,
    ) -> Result<bool, OpenFamilyErrorChirho> {
        while let Some((left_chirho, right_chirho)) = pending_chirho.pop() {
            spend_chirho(budget_chirho)?;
            let mut left_chirho = self.root_chirho(left_chirho);
            let mut right_chirho = self.root_chirho(right_chirho);
            if left_chirho == right_chirho {
                continue;
            }
            let shape_chirho = match (
                &self.shapes_chirho[left_chirho],
                &self.shapes_chirho[right_chirho],
            ) {
                (ShapeChirho::OpaqueChirho, _) | (_, ShapeChirho::OpaqueChirho) => {
                    return Err(OpenFamilyErrorChirho::UnprovedChirho);
                }
                (ShapeChirho::VariableChirho, other_chirho)
                | (other_chirho, ShapeChirho::VariableChirho) => other_chirho.clone(),
                (
                    ShapeChirho::AtomChirho(left_atom_chirho),
                    ShapeChirho::AtomChirho(right_atom_chirho),
                ) if left_atom_chirho == right_atom_chirho => {
                    self.shapes_chirho[left_chirho].clone()
                }
                (
                    ShapeChirho::StructuredChirho(left_tag_chirho, left_parts_chirho),
                    ShapeChirho::StructuredChirho(right_tag_chirho, right_parts_chirho),
                ) if left_tag_chirho == right_tag_chirho
                    && left_parts_chirho.len() == right_parts_chirho.len() =>
                {
                    pending_chirho.extend(
                        left_parts_chirho
                            .iter()
                            .copied()
                            .zip(right_parts_chirho.iter().copied()),
                    );
                    self.shapes_chirho[left_chirho].clone()
                }
                _ => return Ok(false),
            };
            if self.sizes_chirho[left_chirho] < self.sizes_chirho[right_chirho] {
                std::mem::swap(&mut left_chirho, &mut right_chirho);
            }
            self.parents_chirho[right_chirho] = left_chirho;
            self.sizes_chirho[left_chirho] += self.sizes_chirho[right_chirho];
            self.shapes_chirho[left_chirho] = shape_chirho;
        }
        Ok(true)
    }

    /// Infinite unification decides non-apartness, not compatibility. Only a
    /// finite substitution licenses the equal-RHS exception. Test this AFTER
    /// all inputs unify: a constructor clash elsewhere still proves apartness
    /// even if one input would require x = [x]. No expansion of cycles occurs.
    fn finite_chirho(&self, budget_chirho: &mut usize) -> Result<bool, OpenFamilyErrorChirho> {
        let mut colors_chirho = vec![0u8; self.shapes_chirho.len()];
        for index_chirho in 0..self.shapes_chirho.len() {
            let mut pending_chirho = vec![(self.root_chirho(index_chirho), false)];
            while let Some((index_chirho, leaving_chirho)) = pending_chirho.pop() {
                spend_chirho(budget_chirho)?;
                if leaving_chirho {
                    colors_chirho[index_chirho] = 2;
                    continue;
                }
                match colors_chirho[index_chirho] {
                    1 => return Ok(false),
                    2 => continue,
                    _ => {}
                }
                colors_chirho[index_chirho] = 1;
                pending_chirho.push((index_chirho, true));
                if let ShapeChirho::StructuredChirho(_, children_chirho) =
                    &self.shapes_chirho[index_chirho]
                {
                    pending_chirho.extend(
                        children_chirho
                            .iter()
                            .map(|child_chirho| (self.root_chirho(*child_chirho), false)),
                    );
                }
            }
        }
        Ok(true)
    }

    /// Read-only comparison under the finite LHS substitution. Comparing RHSs must
    /// never unify two distinct unconstrained variables to manufacture agreement.
    fn equal_chirho(
        &self,
        left_chirho: usize,
        right_chirho: usize,
        budget_chirho: &mut usize,
    ) -> Result<bool, OpenFamilyErrorChirho> {
        let mut pending_chirho = vec![(left_chirho, right_chirho)];
        let mut seen_chirho = HashSet::new();
        while let Some((left_chirho, right_chirho)) = pending_chirho.pop() {
            spend_chirho(budget_chirho)?;
            let left_chirho = self.root_chirho(left_chirho);
            let right_chirho = self.root_chirho(right_chirho);
            if left_chirho == right_chirho
                || !seen_chirho
                    .insert((left_chirho.min(right_chirho), left_chirho.max(right_chirho)))
            {
                continue;
            }
            match (
                &self.shapes_chirho[left_chirho],
                &self.shapes_chirho[right_chirho],
            ) {
                (ShapeChirho::OpaqueChirho, _) | (_, ShapeChirho::OpaqueChirho) => {
                    return Err(OpenFamilyErrorChirho::UnprovedChirho);
                }
                (
                    ShapeChirho::AtomChirho(left_atom_chirho),
                    ShapeChirho::AtomChirho(right_atom_chirho),
                ) if left_atom_chirho == right_atom_chirho => {}
                (
                    ShapeChirho::StructuredChirho(left_tag_chirho, left_parts_chirho),
                    ShapeChirho::StructuredChirho(right_tag_chirho, right_parts_chirho),
                ) if left_tag_chirho == right_tag_chirho
                    && left_parts_chirho.len() == right_parts_chirho.len() =>
                {
                    pending_chirho.extend(
                        left_parts_chirho
                            .iter()
                            .copied()
                            .zip(right_parts_chirho.iter().copied()),
                    );
                }
                _ => return Ok(false),
            }
        }
        Ok(true)
    }
}

fn compatible_pair_chirho<TermChirho: FamilyTermChirho>(
    left_chirho: &(Vec<TermChirho>, TermChirho),
    right_chirho: &(Vec<TermChirho>, TermChirho),
    family_chirho: &impl Fn(&str) -> bool,
    budget_chirho: &mut usize,
) -> Result<(), OpenFamilyErrorChirho> {
    let mut graph_chirho = RationalGraphChirho::new_chirho();
    let mut inputs_chirho = Vec::new();
    for (left_chirho, right_chirho) in left_chirho.0.iter().zip(&right_chirho.0) {
        inputs_chirho.push((
            graph_chirho.import_chirho(
                left_chirho,
                false,
                false,
                family_chirho,
                budget_chirho,
                0,
            )?,
            graph_chirho.import_chirho(
                right_chirho,
                true,
                false,
                family_chirho,
                budget_chirho,
                0,
            )?,
        ));
    }
    if !graph_chirho.unify_chirho(inputs_chirho, budget_chirho)? {
        return Ok(());
    }
    if !graph_chirho.finite_chirho(budget_chirho)? {
        return Err(OpenFamilyErrorChirho::ConflictChirho);
    }
    let left_result_chirho =
        graph_chirho.import_chirho(&left_chirho.1, false, true, family_chirho, budget_chirho, 0)?;
    let right_result_chirho =
        graph_chirho.import_chirho(&right_chirho.1, true, true, family_chirho, budget_chirho, 0)?;
    if graph_chirho.equal_chirho(left_result_chirho, right_result_chirho, budget_chirho)? {
        Ok(())
    } else {
        Err(OpenFamilyErrorChirho::ConflictChirho)
    }
}
