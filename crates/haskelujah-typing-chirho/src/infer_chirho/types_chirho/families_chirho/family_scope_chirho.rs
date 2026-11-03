// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The written family names admitted in one module. The frontend supplies only
//! type names admitted by naming's existing import-specification computation.
//! This module does not reinterpret import lists or perform suffix lookups.

use std::collections::{BTreeSet, HashMap};

use super::family_definitions_chirho::{
    TypeFamilyAliasResolutionChirho, TypeFamilyDefinitionErrorChirho, TypeFamilyIdChirho,
    TypeFamilyRegistryChirho, family_definition_error_chirho, is_type_family_key_chirho,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum LocalTypeBindingChirho {
    FamilyChirho(String),
    OtherTypeChirho,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeFamilyScopeResolutionChirho<'scope_chirho> {
    MissingChirho,
    UniqueChirho(&'scope_chirho str),
    AmbiguousChirho(&'scope_chirho BTreeSet<String>),
}

#[derive(Clone, Debug, Default)]
pub struct TypeFamilyScopeChirho {
    imported_chirho: HashMap<String, BTreeSet<String>>,
    local_chirho: HashMap<String, LocalTypeBindingChirho>,
}

impl TypeFamilyScopeChirho {
    pub fn new_chirho() -> Self {
        Self::default()
    }

    /// Naming has already decided which type names this import admits. Apply
    /// its chosen written qualifier, retaining all possible defining identities.
    pub fn import_export_chirho(
        &mut self,
        written_name_chirho: String,
        export_alias_chirho: &str,
        registry_chirho: &TypeFamilyRegistryChirho,
    ) {
        match registry_chirho.resolve_export_chirho(export_alias_chirho) {
            TypeFamilyAliasResolutionChirho::MissingChirho => {}
            TypeFamilyAliasResolutionChirho::UniqueChirho(definition_chirho) => {
                self.imported_chirho
                    .entry(written_name_chirho)
                    .or_default()
                    .insert(definition_chirho.id_chirho().key_chirho());
            }
            TypeFamilyAliasResolutionChirho::AmbiguousChirho(identities_chirho) => {
                self.imported_chirho
                    .entry(written_name_chirho)
                    .or_default()
                    .extend(identities_chirho.iter().map(TypeFamilyIdChirho::key_chirho));
            }
        }
    }

    /// Local type declarations shadow imported bare type names independently
    /// of the order in which imports and declarations are registered.
    pub fn bind_local_family_chirho(
        &mut self,
        written_name_chirho: String,
        identity_chirho: &TypeFamilyIdChirho,
    ) -> Result<(), TypeFamilyDefinitionErrorChirho> {
        self.bind_local_chirho(
            written_name_chirho,
            LocalTypeBindingChirho::FamilyChirho(identity_chirho.key_chirho()),
        )
    }

    /// A local data type, class or synonym also shadows an imported family.
    pub fn bind_other_local_type_chirho(
        &mut self,
        written_name_chirho: String,
    ) -> Result<(), TypeFamilyDefinitionErrorChirho> {
        self.bind_local_chirho(written_name_chirho, LocalTypeBindingChirho::OtherTypeChirho)
    }

    fn bind_local_chirho(
        &mut self,
        name_chirho: String,
        binding_chirho: LocalTypeBindingChirho,
    ) -> Result<(), TypeFamilyDefinitionErrorChirho> {
        if let Some(existing_chirho) = self.local_chirho.get(&name_chirho) {
            if existing_chirho != &binding_chirho {
                return Err(family_definition_error_chirho(
                    "conflicting local type bindings in family scope",
                ));
            }
        } else {
            self.local_chirho.insert(name_chirho, binding_chirho);
        }
        Ok(())
    }

    /// Resolve before generic imported-type normalization can erase a qualifier.
    /// Opaque keys must already exist in the registry; a source spelling cannot
    /// create a definition by looking like an internal key.
    pub fn resolve_chirho<'scope_chirho>(
        &'scope_chirho self,
        name_chirho: &'scope_chirho str,
        registry_chirho: &TypeFamilyRegistryChirho,
    ) -> TypeFamilyScopeResolutionChirho<'scope_chirho> {
        if is_type_family_key_chirho(name_chirho) {
            return if registry_chirho.definition_chirho(name_chirho).is_some() {
                TypeFamilyScopeResolutionChirho::UniqueChirho(name_chirho)
            } else {
                TypeFamilyScopeResolutionChirho::MissingChirho
            };
        }
        if let Some(binding_chirho) = self.local_chirho.get(name_chirho) {
            return match binding_chirho {
                LocalTypeBindingChirho::FamilyChirho(key_chirho) => {
                    TypeFamilyScopeResolutionChirho::UniqueChirho(key_chirho)
                }
                LocalTypeBindingChirho::OtherTypeChirho => {
                    TypeFamilyScopeResolutionChirho::MissingChirho
                }
            };
        }
        match self.imported_chirho.get(name_chirho) {
            Some(keys_chirho) if keys_chirho.len() == 1 => {
                TypeFamilyScopeResolutionChirho::UniqueChirho(
                    keys_chirho.first().expect("one family identity"),
                )
            }
            Some(keys_chirho) => TypeFamilyScopeResolutionChirho::AmbiguousChirho(keys_chirho),
            None => TypeFamilyScopeResolutionChirho::MissingChirho,
        }
    }
}

#[cfg(test)]
#[path = "family_scope_tests_chirho.rs"]
mod tests_chirho;
