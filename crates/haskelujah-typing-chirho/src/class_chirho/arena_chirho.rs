// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Canonical class/instance records and id-only spelling/identity views.
//! Arena ids are local handles, never nominal class or compilation-unit proof.
//! Workflow: language-features-chirho/dictionary-evidence-chirho.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use haskelujah_ast_chirho::name_chirho::{
    ClassIdentityUnknownChirho, CompilerClassChirho, InstanceIdentityChirho,
    PredicateClassIdentityChirho, PredicateClassProofChirho,
};

use super::views_chirho::{ClassNamesViewChirho, InstanceNamesViewChirho};
use super::{ClassDeclChirho, ClassDeclPartsChirho, InstDeclChirho, InstDeclPartsChirho};

const REGISTRY_WORK_CHIRHO: usize = 1 << 25;

#[derive(Debug)]
struct ArenaOwnerChirho;

/// A local reference to one canonical class payload. The owning arena must match.
#[derive(Debug, Clone)]
pub struct ClassRecordIdChirho {
    owner_chirho: Arc<ArenaOwnerChirho>,
    index_chirho: usize,
}

/// A local reference to one canonical instance payload. The owning arena must match.
#[derive(Debug, Clone)]
pub struct InstanceRecordIdChirho {
    owner_chirho: Arc<ArenaOwnerChirho>,
    index_chirho: usize,
}

macro_rules! local_record_id_chirho {
    ($record_chirho:ty) => {
        impl PartialEq for $record_chirho {
            fn eq(&self, other_chirho: &Self) -> bool {
                Arc::ptr_eq(&self.owner_chirho, &other_chirho.owner_chirho)
                    && self.index_chirho == other_chirho.index_chirho
            }
        }
        impl Eq for $record_chirho {}
        impl Hash for $record_chirho {
            fn hash<HasherChirho: Hasher>(&self, state_chirho: &mut HasherChirho) {
                Arc::as_ptr(&self.owner_chirho).hash(state_chirho);
                self.index_chirho.hash(state_chirho);
            }
        }
    };
}

local_record_id_chirho!(ClassRecordIdChirho);
local_record_id_chirho!(InstanceRecordIdChirho);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassRegistryErrorChirho {
    WorkBoundChirho,
    SeedIdentityBoundChirho,
}

/// A definition conflict is explicit; an identity lookup never picks by order.
#[derive(Debug, Clone)]
pub(super) enum ClassIdentityEntryChirho {
    UniqueChirho(ClassRecordIdChirho),
    AmbiguousChirho,
}

/// Full records live once in the vectors. Every other collection is an index.
#[derive(Debug)]
pub struct ClassEnvChirho {
    owner_chirho: Arc<ArenaOwnerChirho>,
    pub(super) class_records_chirho: Vec<Arc<ClassDeclChirho>>,
    pub(super) instance_records_chirho: Vec<Arc<InstDeclChirho>>,
    class_pointer_index_chirho: HashMap<usize, ClassRecordIdChirho>,
    instance_pointer_index_chirho: HashMap<usize, InstanceRecordIdChirho>,
    pub(super) class_names_chirho: HashMap<String, ClassRecordIdChirho>,
    pub(super) instance_names_chirho: HashMap<String, Vec<InstanceRecordIdChirho>>,
    class_identities_chirho: HashMap<PredicateClassIdentityChirho, ClassIdentityEntryChirho>,
    instance_identities_chirho: HashMap<PredicateClassIdentityChirho, Vec<InstanceRecordIdChirho>>,
    next_seed_instance_chirho: usize,
    remaining_work_chirho: usize,
    error_chirho: Option<ClassRegistryErrorChirho>,
}

impl Default for ClassEnvChirho {
    fn default() -> Self {
        Self {
            owner_chirho: Arc::new(ArenaOwnerChirho),
            class_records_chirho: Vec::new(),
            instance_records_chirho: Vec::new(),
            class_pointer_index_chirho: HashMap::new(),
            instance_pointer_index_chirho: HashMap::new(),
            class_names_chirho: HashMap::new(),
            instance_names_chirho: HashMap::new(),
            class_identities_chirho: HashMap::new(),
            instance_identities_chirho: HashMap::new(),
            next_seed_instance_chirho: 0,
            remaining_work_chirho: REGISTRY_WORK_CHIRHO,
            error_chirho: None,
        }
    }
}

impl ClassEnvChirho {
    pub fn try_snapshot_chirho(&self) -> Result<Self, ClassRegistryErrorChirho> {
        if let Some(error_chirho) = self.error_chirho {
            return Err(error_chirho);
        }
        let copy_weight_chirho = self
            .copy_weight_chirho()
            .ok_or(ClassRegistryErrorChirho::WorkBoundChirho)?;
        let remaining_work_chirho = self
            .remaining_work_chirho
            .checked_sub(copy_weight_chirho)
            .ok_or(ClassRegistryErrorChirho::WorkBoundChirho)?;
        let owner_chirho = Arc::new(ArenaOwnerChirho);
        // Clone each legacy table *as a table*, retaining its hasher and bucket
        // order. Rebuilding with collect() would introduce an ordering change.
        let mut class_names_chirho = self.class_names_chirho.clone();
        let mut instance_names_chirho = self.instance_names_chirho.clone();
        let mut class_pointer_index_chirho = self.class_pointer_index_chirho.clone();
        let mut instance_pointer_index_chirho = self.instance_pointer_index_chirho.clone();
        let mut class_identities_chirho = self.class_identities_chirho.clone();
        let mut instance_identities_chirho = self.instance_identities_chirho.clone();
        for id_chirho in class_names_chirho
            .values_mut()
            .chain(class_pointer_index_chirho.values_mut())
        {
            id_chirho.owner_chirho = Arc::clone(&owner_chirho);
        }
        for ids_chirho in instance_names_chirho
            .values_mut()
            .chain(instance_identities_chirho.values_mut())
        {
            for id_chirho in ids_chirho {
                id_chirho.owner_chirho = Arc::clone(&owner_chirho);
            }
        }
        for id_chirho in instance_pointer_index_chirho.values_mut() {
            id_chirho.owner_chirho = Arc::clone(&owner_chirho);
        }
        for entry_chirho in class_identities_chirho.values_mut() {
            if let ClassIdentityEntryChirho::UniqueChirho(id_chirho) = entry_chirho {
                id_chirho.owner_chirho = Arc::clone(&owner_chirho);
            }
        }
        Ok(Self {
            owner_chirho,
            class_records_chirho: self.class_records_chirho.clone(),
            instance_records_chirho: self.instance_records_chirho.clone(),
            class_pointer_index_chirho,
            instance_pointer_index_chirho,
            class_names_chirho,
            instance_names_chirho,
            class_identities_chirho,
            instance_identities_chirho,
            next_seed_instance_chirho: self.next_seed_instance_chirho,
            remaining_work_chirho,
            error_chirho: None,
        })
    }
}

impl ClassEnvChirho {
    /// Bound the *new* copied indices and Arc handles before cloning or hashing.
    /// Immutable class/instance payloads themselves are never copied here.
    fn copy_weight_chirho(&self) -> Option<usize> {
        let mut weight_chirho = self
            .class_records_chirho
            .len()
            .checked_add(self.instance_records_chirho.len())?;
        let mut add_chirho = |amount_chirho: usize| -> Option<()> {
            weight_chirho = weight_chirho.checked_add(amount_chirho)?;
            (weight_chirho <= REGISTRY_WORK_CHIRHO).then_some(())
        };
        for name_chirho in self.class_names_chirho.keys() {
            add_chirho(name_chirho.len().checked_add(1)?)?;
        }
        for (name_chirho, ids_chirho) in &self.instance_names_chirho {
            add_chirho(
                name_chirho
                    .len()
                    .checked_add(ids_chirho.len())?
                    .checked_add(1)?,
            )?;
        }
        for identity_chirho in self.class_identities_chirho.keys() {
            add_chirho(identity_weight_chirho(identity_chirho)?)?;
        }
        for (identity_chirho, ids_chirho) in &self.instance_identities_chirho {
            add_chirho(identity_weight_chirho(identity_chirho)?.checked_add(ids_chirho.len())?)?;
        }
        add_chirho(
            self.class_pointer_index_chirho
                .len()
                .checked_add(self.instance_pointer_index_chirho.len())?,
        )?;
        Some(weight_chirho)
    }

    pub fn new_chirho() -> Self {
        Self::default()
    }

    pub fn classes_chirho(&self) -> ClassNamesViewChirho<'_> {
        ClassNamesViewChirho::new_chirho(self)
    }

    pub fn instances_chirho(&self) -> InstanceNamesViewChirho<'_> {
        InstanceNamesViewChirho::new_chirho(self)
    }

    pub fn registry_error_chirho(&self) -> Option<ClassRegistryErrorChirho> {
        self.error_chirho
    }

    fn checked_weight_chirho(&mut self, weight_chirho: Option<usize>) -> Option<usize> {
        if weight_chirho.is_none() {
            self.error_chirho = Some(ClassRegistryErrorChirho::WorkBoundChirho);
        }
        weight_chirho
    }

    fn charge_chirho(&mut self, weight_chirho: usize) -> bool {
        if self.error_chirho.is_some() {
            return false;
        }
        let Some(remaining_chirho) = self.remaining_work_chirho.checked_sub(weight_chirho) else {
            self.error_chirho = Some(ClassRegistryErrorChirho::WorkBoundChirho);
            return false;
        };
        self.remaining_work_chirho = remaining_chirho;
        true
    }

    pub fn class_record_chirho(
        &self,
        id_chirho: &ClassRecordIdChirho,
    ) -> Option<&Arc<ClassDeclChirho>> {
        Arc::ptr_eq(&self.owner_chirho, &id_chirho.owner_chirho)
            .then(|| self.class_records_chirho.get(id_chirho.index_chirho))
            .flatten()
    }

    pub fn instance_record_chirho(
        &self,
        id_chirho: &InstanceRecordIdChirho,
    ) -> Option<&Arc<InstDeclChirho>> {
        Arc::ptr_eq(&self.owner_chirho, &id_chirho.owner_chirho)
            .then(|| self.instance_records_chirho.get(id_chirho.index_chirho))
            .flatten()
    }

    fn intern_class_chirho(
        &mut self,
        declaration_chirho: Arc<ClassDeclChirho>,
    ) -> Option<ClassRecordIdChirho> {
        let pointer_chirho = Arc::as_ptr(&declaration_chirho) as usize;
        if let Some(id_chirho) = self.class_pointer_index_chirho.get(&pointer_chirho) {
            return Some(id_chirho.clone());
        }
        let identity_chirho = declaration_chirho.class_identity_chirho().identity_chirho();
        let weight_chirho =
            self.checked_weight_chirho(identity_chirho.map_or(Some(1), identity_weight_chirho))?;
        if !self.charge_chirho(weight_chirho) {
            return None;
        }
        let id_chirho = ClassRecordIdChirho {
            owner_chirho: Arc::clone(&self.owner_chirho),
            index_chirho: self.class_records_chirho.len(),
        };
        if let Some(identity_chirho) = identity_chirho {
            let identity_chirho = identity_chirho.clone();
            self.class_identities_chirho
                .entry(identity_chirho)
                .and_modify(|entry_chirho| {
                    *entry_chirho = ClassIdentityEntryChirho::AmbiguousChirho
                })
                .or_insert_with(|| ClassIdentityEntryChirho::UniqueChirho(id_chirho.clone()));
        }
        self.class_records_chirho.push(declaration_chirho);
        self.class_pointer_index_chirho
            .insert(pointer_chirho, id_chirho.clone());
        Some(id_chirho)
    }

    fn intern_instance_chirho(
        &mut self,
        declaration_chirho: Arc<InstDeclChirho>,
    ) -> Option<InstanceRecordIdChirho> {
        let pointer_chirho = Arc::as_ptr(&declaration_chirho) as usize;
        if let Some(id_chirho) = self.instance_pointer_index_chirho.get(&pointer_chirho) {
            return Some(id_chirho.clone());
        }
        let identity_chirho = declaration_chirho.class_identity_chirho().identity_chirho();
        let weight_chirho =
            self.checked_weight_chirho(identity_chirho.map_or(Some(1), identity_weight_chirho))?;
        if !self.charge_chirho(weight_chirho) {
            return None;
        }
        let id_chirho = InstanceRecordIdChirho {
            owner_chirho: Arc::clone(&self.owner_chirho),
            index_chirho: self.instance_records_chirho.len(),
        };
        if let Some(identity_chirho) = identity_chirho {
            self.instance_identities_chirho
                .entry(identity_chirho.clone())
                .or_default()
                .push(id_chirho.clone());
        }
        self.instance_records_chirho.push(declaration_chirho);
        self.instance_pointer_index_chirho
            .insert(pointer_chirho, id_chirho.clone());
        Some(id_chirho)
    }

    pub fn add_class_chirho(&mut self, declaration_chirho: ClassDeclChirho) {
        if self.error_chirho.is_some() {
            return;
        }
        if self
            .add_class_record_chirho(Arc::new(declaration_chirho))
            .is_none()
        {
            debug_assert!(
                self.error_chirho.is_some(),
                "registration failure retains its resource error"
            );
        }
    }

    pub fn add_class_record_chirho(
        &mut self,
        declaration_chirho: Arc<ClassDeclChirho>,
    ) -> Option<ClassRecordIdChirho> {
        let weight_chirho = self.checked_weight_chirho(
            declaration_chirho
                .name_chirho
                .len()
                .checked_mul(2)
                .and_then(|weight_chirho| weight_chirho.checked_add(2)),
        )?;
        if !self.charge_chirho(weight_chirho) {
            return None;
        }
        let name_chirho = declaration_chirho.name_chirho.clone();
        let id_chirho = self.intern_class_chirho(declaration_chirho)?;
        self.class_names_chirho
            .insert(name_chirho.clone(), id_chirho.clone());
        self.instance_names_chirho.entry(name_chirho).or_default();
        Some(id_chirho)
    }

    pub fn add_instance_chirho(&mut self, declaration_chirho: InstDeclChirho) {
        if self.error_chirho.is_some() {
            return;
        }
        self.add_shared_instance_chirho(Arc::new(declaration_chirho));
    }

    pub fn add_shared_instance_chirho(&mut self, declaration_chirho: Arc<InstDeclChirho>) {
        if self
            .add_instance_record_chirho(declaration_chirho)
            .is_none()
        {
            debug_assert!(
                self.error_chirho.is_some(),
                "registration failure retains its resource error"
            );
        }
    }

    pub fn add_instance_record_chirho(
        &mut self,
        declaration_chirho: Arc<InstDeclChirho>,
    ) -> Option<InstanceRecordIdChirho> {
        let weight_chirho =
            self.checked_weight_chirho(declaration_chirho.class_name_chirho.len().checked_add(2))?;
        if !self.charge_chirho(weight_chirho) {
            return None;
        }
        let name_chirho = declaration_chirho.class_name_chirho.clone();
        let id_chirho = self.intern_instance_chirho(declaration_chirho)?;
        self.instance_names_chirho
            .entry(name_chirho)
            .or_default()
            .push(id_chirho.clone());
        Some(id_chirho)
    }

    pub(super) fn add_seed_class_chirho(
        &mut self,
        identity_chirho: CompilerClassChirho,
        parts_chirho: ClassDeclPartsChirho,
    ) {
        self.add_class_chirho(ClassDeclChirho::from_producer_chirho(
            parts_chirho,
            PredicateClassProofChirho::ProvenChirho(PredicateClassIdentityChirho::CompilerChirho(
                identity_chirho,
            )),
        ));
    }

    pub(super) fn add_seed_instance_reference_chirho(
        &mut self,
        identity_chirho: Option<CompilerClassChirho>,
        parts_chirho: InstDeclPartsChirho,
    ) {
        if let Some(identity_chirho) = identity_chirho {
            self.add_seed_instance_chirho(identity_chirho, parts_chirho);
        } else {
            // The producer writes this instance but has no corresponding typed
            // compiler-class declaration. Its class authority stays unknown.
            self.add_instance_chirho(InstDeclChirho::legacy_chirho(parts_chirho));
        }
    }

    pub(super) fn add_seed_instance_chirho(
        &mut self,
        identity_chirho: CompilerClassChirho,
        parts_chirho: InstDeclPartsChirho,
    ) {
        let Some(next_chirho) = self.next_seed_instance_chirho.checked_add(1) else {
            self.error_chirho = Some(ClassRegistryErrorChirho::SeedIdentityBoundChirho);
            return;
        };
        let origin_chirho = InstanceIdentityChirho::SeedChirho(self.next_seed_instance_chirho);
        self.next_seed_instance_chirho = next_chirho;
        let identity_chirho = PredicateClassIdentityChirho::CompilerChirho(identity_chirho);
        let proof_chirho = self
            .class_record_by_identity_chirho(&identity_chirho)
            .map(|class_chirho| class_chirho.class_identity_chirho().clone())
            .unwrap_or(PredicateClassProofChirho::UnknownChirho(
                ClassIdentityUnknownChirho::AmbiguousChirho,
            ));
        self.add_instance_chirho(InstDeclChirho::from_producer_chirho(
            parts_chirho,
            proof_chirho,
            origin_chirho,
        ));
    }

    pub fn class_record_by_identity_chirho(
        &self,
        identity_chirho: &PredicateClassIdentityChirho,
    ) -> Option<&Arc<ClassDeclChirho>> {
        if self.error_chirho.is_some() {
            return None;
        }
        match self.class_identities_chirho.get(identity_chirho)? {
            ClassIdentityEntryChirho::UniqueChirho(id_chirho) => {
                self.class_record_chirho(id_chirho)
            }
            ClassIdentityEntryChirho::AmbiguousChirho => None,
        }
    }

    pub fn instance_records_by_identity_chirho(
        &self,
        identity_chirho: &PredicateClassIdentityChirho,
    ) -> impl Iterator<Item = &Arc<InstDeclChirho>> {
        self.instance_identities_chirho
            .get(identity_chirho)
            .into_iter()
            .flatten()
            .filter_map(|id_chirho| self.instance_record_chirho(id_chirho))
    }
}

fn identity_weight_chirho(identity_chirho: &PredicateClassIdentityChirho) -> Option<usize> {
    match identity_chirho {
        PredicateClassIdentityChirho::CompilerChirho(_) => Some(1),
        PredicateClassIdentityChirho::SourceChirho {
            unit_chirho,
            module_chirho,
            name_chirho,
        } => unit_chirho
            .as_str_chirho()
            .len()
            .checked_add(module_chirho.len())?
            .checked_add(name_chirho.len())?
            .checked_add(1),
    }
}

#[cfg(test)]
#[path = "arena_tests_chirho.rs"]
mod tests_chirho;
