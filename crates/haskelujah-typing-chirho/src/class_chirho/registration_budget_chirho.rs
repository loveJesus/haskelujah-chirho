// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! New registration/identity work belongs to one inference module. Same-module
//! restore points share this sticky budget; an admitted next module gets a fresh
//! allowance under the same policy. Snapshot allocation is preflighted separately.
use super::ClassRegistryErrorChirho;
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

pub(super) const MAX_REGISTRATION_WORK_CHIRHO: usize = 1 << 25;

#[derive(Debug)]
pub(super) struct ModuleRegistrationBudgetChirho {
    limit_chirho: usize,
    remaining_chirho: AtomicUsize,
    metadata_remaining_chirho: AtomicUsize,
    failure_chirho: AtomicU8,
}

impl ModuleRegistrationBudgetChirho {
    pub(super) fn new_chirho(limit_chirho: usize) -> Self {
        let limit_chirho = limit_chirho.min(MAX_REGISTRATION_WORK_CHIRHO);
        Self {
            limit_chirho,
            remaining_chirho: AtomicUsize::new(limit_chirho),
            metadata_remaining_chirho: AtomicUsize::new(limit_chirho),
            failure_chirho: AtomicU8::new(0),
        }
    }
    pub(super) fn limit_chirho(&self) -> usize {
        self.limit_chirho
    }
    pub(super) fn remaining_chirho(&self) -> usize {
        self.remaining_chirho.load(Ordering::Relaxed)
    }
    pub(super) fn failure_chirho(&self) -> Option<ClassRegistryErrorChirho> {
        match self.failure_chirho.load(Ordering::Relaxed) {
            0 => None,
            1 => Some(ClassRegistryErrorChirho::WorkBoundChirho),
            2 => Some(ClassRegistryErrorChirho::SeedIdentityBoundChirho),
            _ => Some(ClassRegistryErrorChirho::SnapshotAllocationBoundChirho),
        }
    }
    pub(super) fn fail_chirho(&self, error_chirho: ClassRegistryErrorChirho) {
        let code_chirho = match error_chirho {
            ClassRegistryErrorChirho::WorkBoundChirho => 1,
            ClassRegistryErrorChirho::SeedIdentityBoundChirho => 2,
            ClassRegistryErrorChirho::SnapshotAllocationBoundChirho => 3,
        };
        let _ = self.failure_chirho.compare_exchange(
            0,
            code_chirho,
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
    }
    /// Soft proof work is separately bounded under the same module policy. It
    /// cannot turn an ordinary declaration into an error after proof refusal.
    pub(super) fn charge_metadata_chirho(&self, amount_chirho: usize) -> bool {
        if self.failure_chirho().is_some()
            || self.metadata_remaining_chirho.load(Ordering::Relaxed) == 0
        {
            return false;
        }
        if self
            .metadata_remaining_chirho
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |remaining_chirho| {
                remaining_chirho.checked_sub(amount_chirho)
            })
            .is_err()
        {
            self.metadata_remaining_chirho.store(0, Ordering::Relaxed);
            return false;
        }
        true
    }

    pub(super) fn charge_chirho(&self, amount_chirho: usize) -> bool {
        if self.failure_chirho().is_some() {
            return false;
        }
        if self
            .remaining_chirho
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |remaining_chirho| {
                remaining_chirho.checked_sub(amount_chirho)
            })
            .is_err()
        {
            self.fail_chirho(ClassRegistryErrorChirho::WorkBoundChirho);
            return false;
        }
        true
    }
}
