// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Nominal family definitions and export aliases. Closed declarations own their
//! branch order; importing a snapshot never concatenates closed declarations.
//! Workflow: language-features-chirho/stuck-family-equalities-chirho.md.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

use super::family_constructor_origins_chirho::FamilyConstructorOriginsChirho;
use crate::ty_chirho::TyChirho;

/// Intrinsics have compiler identities, rather than authority from a spelling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntrinsicTypeFamilyChirho {
    CharToNatChirho,
    NatToCharChirho,
    AppendSymbolChirho,
    AddNatChirho,
    MultiplyNatChirho,
    PowerNatChirho,
    SubtractNatChirho,
    DivideNatChirho,
    ModuloNatChirho,
    Log2NatChirho,
    LessEqualNatChirho,
    IfChirho,
    NotChirho,
    AndChirho,
    OrChirho,
}

impl IntrinsicTypeFamilyChirho {
    pub const fn arity_chirho(self) -> usize {
        match self {
            Self::CharToNatChirho
            | Self::NatToCharChirho
            | Self::Log2NatChirho
            | Self::NotChirho => 1,
            Self::IfChirho => 3,
            _ => 2,
        }
    }
}

/// The current frontend compiles one source module of each name per program.
/// A package unit identity can be added here when that frontend supports it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TypeFamilyIdChirho {
    SourceChirho {
        module_chirho: String,
        name_chirho: String,
    },
    CompilerSeedChirho {
        module_chirho: String,
        name_chirho: String,
    },
    IntrinsicChirho(IntrinsicTypeFamilyChirho),
}

impl TypeFamilyIdChirho {
    /// Injective internal key. No consumer recovers authority by parsing names.
    pub fn key_chirho(&self) -> String {
        match self {
            Self::SourceChirho {
                module_chirho,
                name_chirho,
            } => format!(
                "%family:source:{}:{}:{}:{}",
                module_chirho.len(),
                module_chirho,
                name_chirho.len(),
                name_chirho
            ),
            Self::CompilerSeedChirho {
                module_chirho,
                name_chirho,
            } => format!(
                "%family:seed:{}:{}:{}:{}",
                module_chirho.len(),
                module_chirho,
                name_chirho.len(),
                name_chirho
            ),
            Self::IntrinsicChirho(intrinsic_chirho) => {
                format!("%family:intrinsic:{intrinsic_chirho:?}")
            }
        }
    }
}

pub fn is_type_family_key_chirho(name_chirho: &str) -> bool {
    name_chirho.starts_with("%family:")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeFamilyReductionChirho {
    OpenChirho,
    ClosedChirho,
    IntrinsicChirho(IntrinsicTypeFamilyChirho),
}

/// The identity of a written equation survives repeated import snapshots.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeFamilyBranchIdChirho {
    pub family_chirho: TypeFamilyIdChirho,
    pub producer_module_chirho: String,
    pub declaration_start_chirho: u32,
    pub equation_index_chirho: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeFamilyBranchChirho {
    pub id_chirho: TypeFamilyBranchIdChirho,
    pub binders_chirho: Vec<String>,
    pub lhs_chirho: Vec<TyChirho>,
    pub rhs_chirho: TyChirho,
    /// Indices refer exclusively to earlier branches in this declaration.
    pub blockers_chirho: Vec<usize>,
    /// Unsupported/invalid preserved shapes never establish a reduction.
    pub held_chirho: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeFamilyDefinitionChirho {
    id_chirho: TypeFamilyIdChirho,
    arity_chirho: usize,
    reduction_chirho: TypeFamilyReductionChirho,
    branches_chirho: Vec<TypeFamilyBranchChirho>,
    first_held_branch_chirho: Option<usize>,
    open_coherence_verified_chirho: bool,
}

/// Invalid or conflicting snapshots cannot silently change a family's meaning.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeFamilyDefinitionErrorChirho {
    message_chirho: String,
}

impl fmt::Display for TypeFamilyDefinitionErrorChirho {
    fn fmt(&self, formatter_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter_chirho.write_str(&self.message_chirho)
    }
}

impl std::error::Error for TypeFamilyDefinitionErrorChirho {}

pub(super) fn family_definition_error_chirho(
    message_chirho: impl Into<String>,
) -> TypeFamilyDefinitionErrorChirho {
    TypeFamilyDefinitionErrorChirho {
        message_chirho: message_chirho.into(),
    }
}

impl TypeFamilyDefinitionChirho {
    pub fn new_chirho(
        id_chirho: TypeFamilyIdChirho,
        arity_chirho: usize,
        reduction_chirho: TypeFamilyReductionChirho,
        mut branches_chirho: Vec<TypeFamilyBranchChirho>,
    ) -> Result<Self, TypeFamilyDefinitionErrorChirho> {
        if let TypeFamilyReductionChirho::IntrinsicChirho(intrinsic_chirho) = reduction_chirho {
            if id_chirho != TypeFamilyIdChirho::IntrinsicChirho(intrinsic_chirho)
                || arity_chirho != intrinsic_chirho.arity_chirho()
                || !branches_chirho.is_empty()
            {
                return Err(family_definition_error_chirho(
                    "intrinsic family has inconsistent identity, arity or written branches",
                ));
            }
        } else if matches!(&id_chirho, TypeFamilyIdChirho::IntrinsicChirho(_)) {
            return Err(family_definition_error_chirho(
                "intrinsic identity requires intrinsic reduction",
            ));
        }
        let closed_owner_chirho = branches_chirho.first().map(|branch_chirho| {
            (
                branch_chirho.id_chirho.producer_module_chirho.clone(),
                branch_chirho.id_chirho.declaration_start_chirho,
            )
        });
        if reduction_chirho == TypeFamilyReductionChirho::ClosedChirho
            && let Some((producer_chirho, _offset_chirho)) = &closed_owner_chirho
        {
            let defining_module_chirho = match &id_chirho {
                TypeFamilyIdChirho::SourceChirho { module_chirho, .. }
                | TypeFamilyIdChirho::CompilerSeedChirho { module_chirho, .. } => module_chirho,
                TypeFamilyIdChirho::IntrinsicChirho(_) => {
                    return Err(family_definition_error_chirho(
                        "intrinsic family cannot own closed branches",
                    ));
                }
            };
            if producer_chirho != defining_module_chirho {
                return Err(family_definition_error_chirho(
                    "closed declaration is not owned by its defining module",
                ));
            }
        }
        let mut seen_chirho = HashSet::new();
        for (index_chirho, branch_chirho) in branches_chirho.iter_mut().enumerate() {
            if branch_chirho.id_chirho.family_chirho != id_chirho {
                return Err(family_definition_error_chirho(
                    "branch belongs to another family",
                ));
            }
            if !seen_chirho.insert(branch_chirho.id_chirho.clone()) {
                return Err(family_definition_error_chirho(
                    "duplicate branch identity in declaration",
                ));
            }
            if branch_chirho.lhs_chirho.len() != arity_chirho {
                return Err(family_definition_error_chirho(
                    "branch argument count differs from family arity",
                ));
            }
            if reduction_chirho == TypeFamilyReductionChirho::ClosedChirho {
                let ordinal_chirho = u32::try_from(index_chirho).map_err(|_error_chirho| {
                    family_definition_error_chirho(
                        "closed declaration exceeds branch identity range",
                    )
                })?;
                if closed_owner_chirho.as_ref()
                    != Some(&(
                        branch_chirho.id_chirho.producer_module_chirho.clone(),
                        branch_chirho.id_chirho.declaration_start_chirho,
                    ))
                    || branch_chirho.id_chirho.equation_index_chirho != ordinal_chirho
                {
                    return Err(family_definition_error_chirho(
                        "closed branches must belong to one declaration in source order",
                    ));
                }
                branch_chirho.blockers_chirho.sort_unstable();
                branch_chirho.blockers_chirho.dedup();
            }
            if branch_chirho
                .blockers_chirho
                .iter()
                .any(|blocker_chirho| *blocker_chirho >= index_chirho)
            {
                return Err(family_definition_error_chirho(
                    "blocker is not an earlier branch of this declaration",
                ));
            }
            if reduction_chirho != TypeFamilyReductionChirho::ClosedChirho
                && !branch_chirho.blockers_chirho.is_empty()
            {
                return Err(family_definition_error_chirho(
                    "open family cannot carry closed-declaration blockers",
                ));
            }
        }
        let first_held_branch_chirho =
            if reduction_chirho == TypeFamilyReductionChirho::ClosedChirho {
                branches_chirho
                    .iter()
                    .position(|branch_chirho| branch_chirho.held_chirho)
            } else {
                None
            };
        Ok(Self {
            id_chirho,
            arity_chirho,
            reduction_chirho,
            branches_chirho,
            first_held_branch_chirho,
            open_coherence_verified_chirho: false,
        })
    }

    pub fn id_chirho(&self) -> &TypeFamilyIdChirho {
        &self.id_chirho
    }
    pub fn arity_chirho(&self) -> usize {
        self.arity_chirho
    }
    pub fn reduction_chirho(&self) -> TypeFamilyReductionChirho {
        self.reduction_chirho
    }
    pub fn branches_chirho(&self) -> &[TypeFamilyBranchChirho] {
        &self.branches_chirho
    }

    pub fn open_coherence_verified_chirho(&self) -> bool {
        self.reduction_chirho == TypeFamilyReductionChirho::OpenChirho
            && self.open_coherence_verified_chirho
    }

    /// Perform the cold open-instance overlap check before any hot reduction.
    /// A merge that changes the branch set invalidates this certificate.
    pub fn verify_open_coherence_chirho(
        &self,
        compatible_chirho: &impl Fn(&TypeFamilyBranchChirho, &TypeFamilyBranchChirho) -> bool,
        mut budget_chirho: usize,
    ) -> Result<Self, TypeFamilyDefinitionErrorChirho> {
        if self.reduction_chirho != TypeFamilyReductionChirho::OpenChirho {
            return Err(family_definition_error_chirho(
                "open coherence check requires an open definition",
            ));
        }
        if self
            .branches_chirho
            .iter()
            .any(|branch_chirho| branch_chirho.held_chirho)
        {
            return Err(family_definition_error_chirho(
                "unsupported open branch cannot establish coherence",
            ));
        }
        for (index_chirho, branch_chirho) in self.branches_chirho.iter().enumerate() {
            for prior_chirho in &self.branches_chirho[..index_chirho] {
                budget_chirho = budget_chirho.checked_sub(1).ok_or_else(|| {
                    family_definition_error_chirho("open-family compatibility budget exhausted")
                })?;
                if !compatible_chirho(prior_chirho, branch_chirho) {
                    return Err(family_definition_error_chirho(
                        "open-family branches have incompatible or unresolved overlap",
                    ));
                }
            }
        }
        let mut verified_chirho = self.clone();
        verified_chirho.open_coherence_verified_chirho = true;
        Ok(verified_chirho)
    }

    /// A preserved unsupported closed equation prevents all later fallbacks.
    /// Store one boundary rather than copying every held index into every row.
    pub fn reduction_candidates_chirho(
        &self,
    ) -> impl Iterator<Item = (usize, &TypeFamilyBranchChirho)> {
        self.branches_chirho
            .iter()
            .enumerate()
            .take(
                self.first_held_branch_chirho
                    .unwrap_or(self.branches_chirho.len()),
            )
            .filter(|(_index_chirho, branch_chirho)| !branch_chirho.held_chirho)
    }

    fn merge_chirho(
        &self,
        incoming_chirho: &Self,
    ) -> Result<Self, TypeFamilyDefinitionErrorChirho> {
        if self.id_chirho != incoming_chirho.id_chirho
            || self.arity_chirho != incoming_chirho.arity_chirho
            || self.reduction_chirho != incoming_chirho.reduction_chirho
        {
            return Err(family_definition_error_chirho(
                "conflicting family identity, arity or reduction kind",
            ));
        }
        if self.reduction_chirho != TypeFamilyReductionChirho::OpenChirho {
            return if self == incoming_chirho {
                Ok(self.clone())
            } else {
                Err(family_definition_error_chirho(
                    "conflicting immutable family declaration",
                ))
            };
        }
        let mut branches_chirho = self.branches_chirho.clone();
        let mut positions_chirho: HashMap<_, _> = branches_chirho
            .iter()
            .enumerate()
            .map(|(index_chirho, branch_chirho)| (branch_chirho.id_chirho.clone(), index_chirho))
            .collect();
        for branch_chirho in &incoming_chirho.branches_chirho {
            if let Some(index_chirho) = positions_chirho.get(&branch_chirho.id_chirho) {
                if branches_chirho[*index_chirho] != *branch_chirho {
                    return Err(family_definition_error_chirho(
                        "conflicting contents for one open branch identity",
                    ));
                }
            } else {
                positions_chirho.insert(branch_chirho.id_chirho.clone(), branches_chirho.len());
                branches_chirho.push(branch_chirho.clone());
            }
        }
        Self::new_chirho(
            self.id_chirho.clone(),
            self.arity_chirho,
            self.reduction_chirho,
            branches_chirho,
        )
    }
}

#[derive(Clone, Debug)]
pub enum TypeFamilyAliasResolutionChirho {
    MissingChirho,
    UniqueChirho(Arc<TypeFamilyDefinitionChirho>),
    AmbiguousChirho(Vec<TypeFamilyIdChirho>),
}

/// Definitions are stored once; aliases retain ambiguity without copying rows.
#[derive(Clone, Debug, Default)]
pub struct TypeFamilyRegistryChirho {
    definitions_chirho: HashMap<String, Arc<TypeFamilyDefinitionChirho>>,
    export_aliases_chirho: HashMap<String, BTreeSet<String>>,
    published_modules_chirho: HashMap<String, BTreeSet<String>>,
    constructor_origins_chirho: FamilyConstructorOriginsChirho,
}

impl TypeFamilyRegistryChirho {
    pub fn new_chirho() -> Self {
        Self::default()
    }

    pub fn definition_chirho(&self, key_chirho: &str) -> Option<&Arc<TypeFamilyDefinitionChirho>> {
        self.definitions_chirho.get(key_chirho)
    }

    pub fn definitions_chirho(&self) -> impl Iterator<Item = &Arc<TypeFamilyDefinitionChirho>> {
        self.definitions_chirho.values()
    }

    pub fn definition_count_chirho(&self) -> usize {
        self.definitions_chirho.len()
    }

    pub fn is_empty_chirho(&self) -> bool {
        self.definitions_chirho.is_empty() && self.constructor_origins_chirho.is_empty_chirho()
    }

    pub(super) fn constructor_origins_chirho(&self) -> &FamilyConstructorOriginsChirho {
        &self.constructor_origins_chirho
    }

    pub(super) fn constructor_origins_mut_chirho(&mut self) -> &mut FamilyConstructorOriginsChirho {
        &mut self.constructor_origins_chirho
    }

    pub fn insert_chirho(
        &mut self,
        definition_chirho: TypeFamilyDefinitionChirho,
    ) -> Result<(), TypeFamilyDefinitionErrorChirho> {
        let key_chirho = definition_chirho.id_chirho.key_chirho();
        let replacement_chirho =
            if let Some(existing_chirho) = self.definitions_chirho.get(&key_chirho) {
                if existing_chirho.as_ref() == &definition_chirho {
                    return Ok(());
                }
                Arc::new(existing_chirho.merge_chirho(&definition_chirho)?)
            } else {
                Arc::new(definition_chirho)
            };
        self.definitions_chirho
            .insert(key_chirho, replacement_chirho);
        Ok(())
    }

    pub fn export_alias_chirho(
        &mut self,
        alias_chirho: String,
        family_chirho: &TypeFamilyIdChirho,
    ) -> Result<(), TypeFamilyDefinitionErrorChirho> {
        let key_chirho = family_chirho.key_chirho();
        if !self.definitions_chirho.contains_key(&key_chirho) {
            return Err(family_definition_error_chirho(
                "export alias names an absent family definition",
            ));
        }
        self.export_aliases_chirho
            .entry(alias_chirho)
            .or_default()
            .insert(key_chirho);
        Ok(())
    }

    /// Replace one publication batch after validating every target. Empty
    /// identity sets clear aliases shadowed by ordinary source type exports.
    pub fn replace_export_aliases_chirho(
        &mut self,
        aliases_chirho: &[(String, BTreeSet<TypeFamilyIdChirho>)],
    ) -> Result<(), TypeFamilyDefinitionErrorChirho> {
        let planned_chirho = self.plan_alias_replacement_chirho(aliases_chirho)?;
        self.apply_alias_replacement_chirho(planned_chirho);
        Ok(())
    }

    /// A source module owns its complete publication snapshot. Retire aliases
    /// removed from its export list, without scanning similarly prefixed modules.
    pub fn replace_module_exports_chirho(
        &mut self,
        module_chirho: &str,
        aliases_chirho: &[(String, BTreeSet<TypeFamilyIdChirho>)],
    ) -> Result<(), TypeFamilyDefinitionErrorChirho> {
        let prefix_chirho = format!("{module_chirho}.");
        if aliases_chirho
            .iter()
            .any(|(alias_chirho, _ids_chirho)| !alias_chirho.starts_with(&prefix_chirho))
        {
            return Err(family_definition_error_chirho(
                "module publication contains another module's alias",
            ));
        }
        let mut planned_chirho = self.plan_alias_replacement_chirho(aliases_chirho)?;
        let retained_chirho = planned_chirho
            .iter()
            .filter(|(_alias_chirho, keys_chirho)| !keys_chirho.is_empty())
            .map(|(alias_chirho, _keys_chirho)| alias_chirho.clone())
            .collect::<BTreeSet<_>>();
        if let Some(previous_chirho) = self.published_modules_chirho.get(module_chirho) {
            for alias_chirho in previous_chirho {
                planned_chirho.entry(alias_chirho.clone()).or_default();
            }
        }
        self.apply_alias_replacement_chirho(planned_chirho);
        self.published_modules_chirho
            .insert(module_chirho.to_owned(), retained_chirho);
        Ok(())
    }

    fn plan_alias_replacement_chirho(
        &self,
        aliases_chirho: &[(String, BTreeSet<TypeFamilyIdChirho>)],
    ) -> Result<HashMap<String, BTreeSet<String>>, TypeFamilyDefinitionErrorChirho> {
        let mut planned_chirho = HashMap::new();
        for (alias_chirho, identities_chirho) in aliases_chirho {
            let keys_chirho = identities_chirho
                .iter()
                .map(TypeFamilyIdChirho::key_chirho)
                .collect::<BTreeSet<_>>();
            if keys_chirho
                .iter()
                .any(|key_chirho| !self.definitions_chirho.contains_key(key_chirho))
            {
                return Err(family_definition_error_chirho(
                    "replacement alias names an absent family definition",
                ));
            }
            if planned_chirho
                .insert(alias_chirho.clone(), keys_chirho)
                .is_some()
            {
                return Err(family_definition_error_chirho(
                    "duplicate alias in replacement batch",
                ));
            }
        }
        Ok(planned_chirho)
    }

    fn apply_alias_replacement_chirho(
        &mut self,
        planned_chirho: HashMap<String, BTreeSet<String>>,
    ) {
        for (alias_chirho, keys_chirho) in planned_chirho {
            if keys_chirho.is_empty() {
                self.export_aliases_chirho.remove(&alias_chirho);
            } else {
                self.export_aliases_chirho.insert(alias_chirho, keys_chirho);
            }
        }
    }

    pub fn resolve_export_chirho(&self, alias_chirho: &str) -> TypeFamilyAliasResolutionChirho {
        let Some(keys_chirho) = self.export_aliases_chirho.get(alias_chirho) else {
            return TypeFamilyAliasResolutionChirho::MissingChirho;
        };
        if keys_chirho.len() == 1 {
            let definition_chirho =
                &self.definitions_chirho[keys_chirho.first().expect("one family key")];
            return TypeFamilyAliasResolutionChirho::UniqueChirho(Arc::clone(definition_chirho));
        }
        TypeFamilyAliasResolutionChirho::AmbiguousChirho(
            keys_chirho
                .iter()
                .map(|key_chirho| self.definitions_chirho[key_chirho].id_chirho.clone())
                .collect(),
        )
    }

    /// Merge independently compiled snapshots atomically. A cumulative snapshot
    /// may be reimported repeatedly without duplicating its branches or aliases.
    pub fn merge_chirho(
        &mut self,
        incoming_chirho: &Self,
    ) -> Result<(), TypeFamilyDefinitionErrorChirho> {
        let mut constructor_origins_chirho = self.constructor_origins_chirho.clone();
        constructor_origins_chirho
            .merge_chirho(&incoming_chirho.constructor_origins_chirho)
            .map_err(|error_chirho| family_definition_error_chirho(error_chirho.to_string()))?;
        let mut replacements_chirho = Vec::new();
        for (key_chirho, definition_chirho) in &incoming_chirho.definitions_chirho {
            if let Some(existing_chirho) = self.definitions_chirho.get(key_chirho) {
                if Arc::ptr_eq(existing_chirho, definition_chirho)
                    || existing_chirho == definition_chirho
                {
                    continue;
                }
                replacements_chirho.push((
                    key_chirho.clone(),
                    Arc::new(existing_chirho.merge_chirho(definition_chirho)?),
                ));
            } else {
                replacements_chirho.push((key_chirho.clone(), Arc::clone(definition_chirho)));
            }
        }
        for (key_chirho, definition_chirho) in replacements_chirho {
            self.definitions_chirho
                .insert(key_chirho, definition_chirho);
        }
        for (alias_chirho, keys_chirho) in &incoming_chirho.export_aliases_chirho {
            self.export_aliases_chirho
                .entry(alias_chirho.clone())
                .or_default()
                .extend(keys_chirho.iter().cloned());
        }
        for (module_chirho, aliases_chirho) in &incoming_chirho.published_modules_chirho {
            self.published_modules_chirho
                .entry(module_chirho.clone())
                .or_default()
                .extend(aliases_chirho.iter().cloned());
        }
        self.constructor_origins_chirho = constructor_origins_chirho;
        Ok(())
    }
}

#[cfg(test)]
#[path = "family_definitions_tests_chirho.rs"]
mod tests_chirho;
