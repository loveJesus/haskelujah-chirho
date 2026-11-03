// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Positive constructor authority for family comparisons. This catalogue does
//! not change ordinary inference types or expose hidden source names. It keeps
//! source declarations, including hidden ones, so a bare spelling shared by two
//! declarations never supplies a nominal-apartness proof.

use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ConstructorOriginChirho {
    CompilerChirho(String),
    SourceChirho {
        module_chirho: String,
        name_chirho: String,
        generative_chirho: bool,
    },
}

impl ConstructorOriginChirho {
    fn key_chirho(&self) -> String {
        match self {
            Self::CompilerChirho(identity_chirho) => format!(
                "%constructor:compiler:{}:{identity_chirho}",
                identity_chirho.len()
            ),
            Self::SourceChirho {
                module_chirho,
                name_chirho,
                ..
            } => format!(
                "%constructor:source:{}:{module_chirho}:{}:{name_chirho}",
                module_chirho.len(),
                name_chirho.len()
            ),
        }
    }

    fn generative_chirho(&self) -> bool {
        match self {
            Self::CompilerChirho(_) => true,
            Self::SourceChirho {
                generative_chirho, ..
            } => *generative_chirho,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FamilyConstructorOriginErrorChirho(String);

impl fmt::Display for FamilyConstructorOriginErrorChirho {
    fn fmt(&self, formatter_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter_chirho.write_str(&self.0)
    }
}

impl std::error::Error for FamilyConstructorOriginErrorChirho {}

#[derive(Clone, Debug, Default)]
pub struct FamilyConstructorOriginsChirho {
    source_modules_chirho: HashMap<String, Arc<HashMap<String, bool>>>,
    heads_chirho: HashMap<String, BTreeSet<ConstructorOriginChirho>>,
}

impl FamilyConstructorOriginsChirho {
    pub fn is_empty_chirho(&self) -> bool {
        self.heads_chirho.is_empty()
    }

    /// Called only by the compiler's actual seed producer, never by inspecting
    /// an arbitrary name and assuming every unfamiliar head is generative.
    pub fn insert_compiler_chirho(&mut self, spelling_chirho: String, identity_chirho: String) {
        let origin_chirho = ConstructorOriginChirho::CompilerChirho(identity_chirho);
        self.insert_head_chirho(origin_chirho.key_chirho(), &origin_chirho);
        self.insert_head_chirho(spelling_chirho, &origin_chirho);
    }

    /// Replace the comparison declarations owned by one source module. Data
    /// and newtype heads are generative; synonyms/classes are recorded too but
    /// cannot provide that proof. Duplicate declarations are not normalized away.
    pub fn replace_source_module_chirho(
        &mut self,
        module_chirho: &str,
        declarations_chirho: &[(String, bool)],
    ) -> Result<(), FamilyConstructorOriginErrorChirho> {
        let mut declarations_by_name_chirho = HashMap::new();
        for (name_chirho, generative_chirho) in declarations_chirho {
            if name_chirho.is_empty()
                || declarations_by_name_chirho
                    .insert(name_chirho.clone(), *generative_chirho)
                    .is_some()
            {
                return Err(FamilyConstructorOriginErrorChirho(
                    "empty or duplicate source type declaration in family comparison catalogue"
                        .to_owned(),
                ));
            }
        }
        if let Some(previous_chirho) = self.source_modules_chirho.remove(module_chirho) {
            for (name_chirho, generative_chirho) in previous_chirho.iter() {
                let origin_chirho = ConstructorOriginChirho::SourceChirho {
                    module_chirho: module_chirho.to_owned(),
                    name_chirho: name_chirho.clone(),
                    generative_chirho: *generative_chirho,
                };
                self.remove_head_chirho(name_chirho, &origin_chirho);
                self.remove_head_chirho(&format!("{module_chirho}.{name_chirho}"), &origin_chirho);
                self.remove_head_chirho(&origin_chirho.key_chirho(), &origin_chirho);
            }
        }
        for (name_chirho, generative_chirho) in &declarations_by_name_chirho {
            let origin_chirho = ConstructorOriginChirho::SourceChirho {
                module_chirho: module_chirho.to_owned(),
                name_chirho: name_chirho.clone(),
                generative_chirho: *generative_chirho,
            };
            self.insert_head_chirho(name_chirho.clone(), &origin_chirho);
            self.insert_head_chirho(format!("{module_chirho}.{name_chirho}"), &origin_chirho);
            self.insert_head_chirho(origin_chirho.key_chirho(), &origin_chirho);
        }
        self.source_modules_chirho.insert(
            module_chirho.to_owned(),
            Arc::new(declarations_by_name_chirho),
        );
        Ok(())
    }

    /// The returned identity is positive and unique. Missing, ambiguous and
    /// non-generative declarations all remain unknown to the comparison kernel.
    pub fn identity_chirho(&self, name_chirho: &str) -> Option<String> {
        let origins_chirho = self.heads_chirho.get(name_chirho)?;
        if origins_chirho.len() != 1 {
            return None;
        }
        let origin_chirho = origins_chirho.first()?;
        origin_chirho
            .generative_chirho()
            .then(|| origin_chirho.key_chirho())
    }

    /// Check the complete source snapshot before changing either index.
    pub fn merge_chirho(
        &mut self,
        incoming_chirho: &Self,
    ) -> Result<(), FamilyConstructorOriginErrorChirho> {
        for (module_chirho, declarations_chirho) in &incoming_chirho.source_modules_chirho {
            if let Some(previous_chirho) = self.source_modules_chirho.get(module_chirho)
                && previous_chirho != declarations_chirho
            {
                return Err(FamilyConstructorOriginErrorChirho(format!(
                    "conflicting type declaration snapshots for module {module_chirho}"
                )));
            }
        }
        for (module_chirho, declarations_chirho) in &incoming_chirho.source_modules_chirho {
            self.source_modules_chirho
                .entry(module_chirho.clone())
                .or_insert_with(|| Arc::clone(declarations_chirho));
        }
        for (name_chirho, origins_chirho) in &incoming_chirho.heads_chirho {
            self.heads_chirho
                .entry(name_chirho.clone())
                .or_default()
                .extend(origins_chirho.iter().cloned());
        }
        Ok(())
    }

    fn insert_head_chirho(&mut self, name_chirho: String, origin_chirho: &ConstructorOriginChirho) {
        self.heads_chirho
            .entry(name_chirho)
            .or_default()
            .insert(origin_chirho.clone());
    }

    fn remove_head_chirho(&mut self, name_chirho: &str, origin_chirho: &ConstructorOriginChirho) {
        if let Some(origins_chirho) = self.heads_chirho.get_mut(name_chirho) {
            origins_chirho.remove(origin_chirho);
            if origins_chirho.is_empty() {
                self.heads_chirho.remove(name_chirho);
            }
        }
    }
}

#[cfg(test)]
#[path = "family_constructor_origins_tests_chirho.rs"]
mod tests_chirho;
