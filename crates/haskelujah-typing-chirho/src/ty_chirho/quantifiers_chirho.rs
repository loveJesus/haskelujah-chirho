// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lexical quantifier records shared by nested types and outer schemes.
//! The ID slice is an immutable derived view, never another binder authority.
//! Workflow: language-features-chirho/rank-n-visible-type-application-chirho.md.

use super::{TyChirho, TyVarChirho};
use haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho;
use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantifiedVariableChirho {
    pub variable_chirho: TyVarChirho,
    pub kind_chirho: Option<Arc<TyChirho>>,
    pub source_chirho: Option<ScopedTypeVariableIdChirho>,
    pub specificity_chirho: Option<TyVarSpecificityChirho>,
}

impl QuantifiedVariableChirho {
    /// Legacy inference has no source-kind/visibility proof to publish.
    pub fn unknown_chirho(variable_chirho: TyVarChirho) -> Self {
        Self {
            variable_chirho,
            kind_chirho: None,
            source_chirho: None,
            specificity_chirho: None,
        }
    }
}

#[derive(Debug)]
struct QuantifierGroupChirho {
    records_chirho: Vec<QuantifiedVariableChirho>,
    variables_chirho: Vec<TyVarChirho>,
}

/// An immutable lexical group. Copies share its records; a transformer must
/// explicitly rebuild records after mapping their variables AND dependent kinds.
#[derive(Debug, Clone)]
pub struct QuantifiedVariablesChirho(Arc<QuantifierGroupChirho>);

impl QuantifiedVariablesChirho {
    pub fn from_records_chirho(records_chirho: Vec<QuantifiedVariableChirho>) -> Self {
        let variables_chirho = records_chirho
            .iter()
            .map(|record_chirho| record_chirho.variable_chirho)
            .collect();
        Self(Arc::new(QuantifierGroupChirho {
            records_chirho,
            variables_chirho,
        }))
    }

    /// Callers charge the group length before bounded transformations. Kinds
    /// stay immutable/shared until the supplied transformer maps them explicitly.
    pub fn map_records_chirho(
        &self,
        action_chirho: impl FnMut(&QuantifiedVariableChirho) -> QuantifiedVariableChirho,
    ) -> Self {
        Self::from_records_chirho(self.records_chirho().iter().map(action_chirho).collect())
    }

    pub fn slice_chirho(&self, range_chirho: std::ops::Range<usize>) -> Self {
        Self::from_records_chirho(self.records_chirho()[range_chirho].to_vec())
    }

    pub fn concat_chirho(&self, other_chirho: &Self) -> Self {
        Self::from_records_chirho(
            self.records_chirho()
                .iter()
                .chain(other_chirho.records_chirho())
                .cloned()
                .collect(),
        )
    }

    pub fn filter_chirho(
        &self,
        mut keep_chirho: impl FnMut(&QuantifiedVariableChirho) -> bool,
    ) -> Self {
        Self::from_records_chirho(
            self.records_chirho()
                .iter()
                .filter(|record_chirho| keep_chirho(record_chirho))
                .cloned()
                .collect(),
        )
    }

    pub fn records_chirho(&self) -> &[QuantifiedVariableChirho] {
        &self.0.records_chirho
    }

    pub fn record_chirho(&self, index_chirho: usize) -> Option<&QuantifiedVariableChirho> {
        self.0.records_chirho.get(index_chirho)
    }

    /// ID-only construction is reserved for genuinely unproved legacy/fresh
    /// binders. Checked copies and alpha-renaming use from_records_chirho.
    pub fn unknown_chirho(variables_chirho: Vec<TyVarChirho>) -> Self {
        Self::from_records_chirho(
            variables_chirho
                .into_iter()
                .map(QuantifiedVariableChirho::unknown_chirho)
                .collect(),
        )
    }

    /// Semantic equality ignores producer origin and inferred/specified flags.
    /// Strict producer compatibility supplies a recursive metadata comparator.
    pub fn same_contract_chirho<ErrorChirho>(
        &self,
        other_chirho: &Self,
        mut kinds_equal_chirho: impl FnMut(&TyChirho, &TyChirho) -> Result<bool, ErrorChirho>,
    ) -> Result<bool, ErrorChirho> {
        if self.len() != other_chirho.len() {
            return Ok(false);
        }
        for (left_chirho, right_chirho) in self
            .records_chirho()
            .iter()
            .zip(other_chirho.records_chirho())
        {
            if left_chirho.variable_chirho != right_chirho.variable_chirho
                || left_chirho.source_chirho != right_chirho.source_chirho
                || left_chirho.specificity_chirho != right_chirho.specificity_chirho
            {
                return Ok(false);
            }
            match (&left_chirho.kind_chirho, &right_chirho.kind_chirho) {
                (Some(left_chirho), Some(right_chirho))
                    if kinds_equal_chirho(left_chirho, right_chirho)? => {}
                (None, None) => {}
                _ => return Ok(false),
            }
        }
        Ok(true)
    }
}

impl Default for QuantifiedVariablesChirho {
    fn default() -> Self {
        Self::from_records_chirho(Vec::new())
    }
}

impl From<Vec<TyVarChirho>> for QuantifiedVariablesChirho {
    fn from(variables_chirho: Vec<TyVarChirho>) -> Self {
        Self::unknown_chirho(variables_chirho)
    }
}

// Read-only compatibility with existing ID algorithms. No DerefMut or owned
// ID iterator exists: a checked reconstruction cannot silently erase records.
impl Deref for QuantifiedVariablesChirho {
    type Target = [TyVarChirho];
    fn deref(&self) -> &Self::Target {
        &self.0.variables_chirho
    }
}

impl<'group_chirho> IntoIterator for &'group_chirho QuantifiedVariablesChirho {
    type Item = &'group_chirho TyVarChirho;
    type IntoIter = std::slice::Iter<'group_chirho, TyVarChirho>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.variables_chirho.iter()
    }
}

impl PartialEq for QuantifiedVariablesChirho {
    fn eq(&self, other_chirho: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other_chirho.0)
            || (self.len() == other_chirho.len()
                && self
                    .records_chirho()
                    .iter()
                    .zip(other_chirho.records_chirho())
                    .all(|(left_chirho, right_chirho)| {
                        left_chirho.variable_chirho == right_chirho.variable_chirho
                            && left_chirho.kind_chirho == right_chirho.kind_chirho
                    }))
    }
}
impl Eq for QuantifiedVariablesChirho {}
impl Hash for QuantifiedVariablesChirho {
    fn hash<HasherChirho: Hasher>(&self, state_chirho: &mut HasherChirho) {
        self.len().hash(state_chirho);
        for record_chirho in self.records_chirho() {
            record_chirho.variable_chirho.hash(state_chirho);
            record_chirho.kind_chirho.hash(state_chirho);
        }
    }
}

/// Strict producer compatibility differs from ordinary type equality: it also
/// protects inline origins and visibility. The caller owns the whole allowance.
pub(crate) fn same_type_contract_chirho(
    left_chirho: &TyChirho,
    right_chirho: &TyChirho,
    charge_chirho: &mut impl FnMut(usize) -> Option<()>,
) -> Option<bool> {
    fn compare_chirho(
        left_chirho: &TyChirho,
        right_chirho: &TyChirho,
        charge_chirho: &mut impl FnMut(usize) -> Option<()>,
        depth_chirho: usize,
    ) -> Option<bool> {
        charge_chirho(1)?;
        if depth_chirho > 128 {
            return None;
        }
        Some(match (left_chirho, right_chirho) {
            (TyChirho::VarChirho(left_chirho), TyChirho::VarChirho(right_chirho)) => {
                left_chirho == right_chirho
            }
            (TyChirho::ConChirho(left_chirho), TyChirho::ConChirho(right_chirho))
            | (TyChirho::ForallVarChirho(left_chirho), TyChirho::ForallVarChirho(right_chirho)) => {
                charge_chirho(left_chirho.len().checked_add(right_chirho.len())?)?;
                left_chirho == right_chirho
            }
            (
                TyChirho::AppChirho(left_fun_chirho, left_arg_chirho),
                TyChirho::AppChirho(right_fun_chirho, right_arg_chirho),
            ) => {
                compare_chirho(
                    left_fun_chirho,
                    right_fun_chirho,
                    charge_chirho,
                    depth_chirho + 1,
                )? && compare_chirho(
                    left_arg_chirho,
                    right_arg_chirho,
                    charge_chirho,
                    depth_chirho + 1,
                )?
            }
            (
                TyChirho::FunChirho(left_arg_chirho, left_result_chirho, left_mult_chirho),
                TyChirho::FunChirho(right_arg_chirho, right_result_chirho, right_mult_chirho),
            ) => {
                left_mult_chirho == right_mult_chirho
                    && compare_chirho(
                        left_arg_chirho,
                        right_arg_chirho,
                        charge_chirho,
                        depth_chirho + 1,
                    )?
                    && compare_chirho(
                        left_result_chirho,
                        right_result_chirho,
                        charge_chirho,
                        depth_chirho + 1,
                    )?
            }
            (TyChirho::TupleChirho(left_chirho), TyChirho::TupleChirho(right_chirho)) => {
                charge_chirho(left_chirho.len().checked_add(right_chirho.len())?)?;
                if left_chirho.len() != right_chirho.len() {
                    return Some(false);
                }
                for (left_chirho, right_chirho) in left_chirho.iter().zip(right_chirho) {
                    if !compare_chirho(left_chirho, right_chirho, charge_chirho, depth_chirho + 1)?
                    {
                        return Some(false);
                    }
                }
                true
            }
            (TyChirho::ListChirho(left_chirho), TyChirho::ListChirho(right_chirho)) => {
                compare_chirho(left_chirho, right_chirho, charge_chirho, depth_chirho + 1)?
            }
            (
                TyChirho::QualChirho {
                    context_chirho: left_context_chirho,
                    body_chirho: left_body_chirho,
                },
                TyChirho::QualChirho {
                    context_chirho: right_context_chirho,
                    body_chirho: right_body_chirho,
                },
            ) => {
                charge_chirho(
                    left_context_chirho
                        .len()
                        .checked_add(right_context_chirho.len())?,
                )?;
                if left_context_chirho.len() != right_context_chirho.len() {
                    return Some(false);
                }
                for (left_chirho, right_chirho) in
                    left_context_chirho.iter().zip(right_context_chirho)
                {
                    if !compare_chirho(left_chirho, right_chirho, charge_chirho, depth_chirho + 1)?
                    {
                        return Some(false);
                    }
                }
                compare_chirho(
                    left_body_chirho,
                    right_body_chirho,
                    charge_chirho,
                    depth_chirho + 1,
                )?
            }
            (
                TyChirho::ForallChirho {
                    vars_chirho: left_vars_chirho,
                    body_chirho: left_body_chirho,
                },
                TyChirho::ForallChirho {
                    vars_chirho: right_vars_chirho,
                    body_chirho: right_body_chirho,
                },
            )
            | (
                TyChirho::RequiredForallChirho {
                    vars_chirho: left_vars_chirho,
                    body_chirho: left_body_chirho,
                },
                TyChirho::RequiredForallChirho {
                    vars_chirho: right_vars_chirho,
                    body_chirho: right_body_chirho,
                },
            ) => {
                charge_chirho(
                    left_vars_chirho
                        .len()
                        .checked_add(right_vars_chirho.len())?,
                )?;
                for record_chirho in left_vars_chirho
                    .records_chirho()
                    .iter()
                    .chain(right_vars_chirho.records_chirho())
                {
                    if let Some(ScopedTypeVariableIdChirho::QuantifiedChirho {
                        declaration_chirho,
                        ..
                    }) = &record_chirho.source_chirho
                    {
                        charge_chirho(declaration_chirho.len())?;
                    }
                }
                let same_chirho = left_vars_chirho
                    .same_contract_chirho(right_vars_chirho, |left_chirho, right_chirho| {
                        compare_chirho(left_chirho, right_chirho, charge_chirho, depth_chirho + 1)
                            .ok_or(())
                    })
                    .ok()?;
                same_chirho
                    && compare_chirho(
                        left_body_chirho,
                        right_body_chirho,
                        charge_chirho,
                        depth_chirho + 1,
                    )?
            }
            _ => false,
        })
    }
    compare_chirho(left_chirho, right_chirho, charge_chirho, 0)
}
