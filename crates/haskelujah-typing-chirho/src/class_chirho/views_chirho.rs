// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Borrowed legacy spelling views over canonical arena payloads. Outer iteration
//! retains the spelling HashMap order; instance lists retain order and duplicates.

use std::sync::Arc;

use super::arena_chirho::{ClassEnvChirho, InstanceRecordIdChirho};
use super::{ClassDeclChirho, InstDeclChirho};

#[derive(Clone, Copy)]
pub struct ClassNamesViewChirho<'environment_chirho> {
    environment_chirho: &'environment_chirho ClassEnvChirho,
}

impl<'environment_chirho> ClassNamesViewChirho<'environment_chirho> {
    pub(super) fn new_chirho(environment_chirho: &'environment_chirho ClassEnvChirho) -> Self {
        Self { environment_chirho }
    }

    pub fn get_chirho(
        &self,
        name_chirho: &str,
    ) -> Option<&'environment_chirho Arc<ClassDeclChirho>> {
        self.environment_chirho
            .class_names_chirho
            .get(name_chirho)
            .and_then(|id_chirho| self.environment_chirho.class_record_chirho(id_chirho))
    }

    pub fn contains_key_chirho(&self, name_chirho: &str) -> bool {
        self.environment_chirho
            .class_names_chirho
            .contains_key(name_chirho)
    }

    pub fn is_empty_chirho(&self) -> bool {
        self.environment_chirho.class_names_chirho.is_empty()
    }
    pub fn len_chirho(&self) -> usize {
        self.environment_chirho.class_names_chirho.len()
    }

    pub fn keys_chirho(
        &self,
    ) -> impl Iterator<Item = &'environment_chirho String> + use<'environment_chirho> {
        self.environment_chirho.class_names_chirho.keys()
    }

    pub fn iter_chirho(
        &self,
    ) -> impl Iterator<
        Item = (
            &'environment_chirho String,
            &'environment_chirho Arc<ClassDeclChirho>,
        ),
    > + use<'environment_chirho> {
        let environment_chirho = self.environment_chirho;
        environment_chirho
            .class_names_chirho
            .iter()
            .filter_map(move |(name_chirho, id_chirho)| {
                environment_chirho
                    .class_record_chirho(id_chirho)
                    .map(|record_chirho| (name_chirho, record_chirho))
            })
    }

    pub fn values_chirho(
        &self,
    ) -> impl Iterator<Item = &'environment_chirho Arc<ClassDeclChirho>> + use<'environment_chirho>
    {
        self.iter_chirho().map(|(_, record_chirho)| record_chirho)
    }
}

#[derive(Clone, Copy)]
pub struct InstanceListViewChirho<'environment_chirho> {
    environment_chirho: &'environment_chirho ClassEnvChirho,
    identifiers_chirho: &'environment_chirho [InstanceRecordIdChirho],
}

impl<'environment_chirho> InstanceListViewChirho<'environment_chirho> {
    pub fn len_chirho(&self) -> usize {
        self.identifiers_chirho.len()
    }
    pub fn is_empty_chirho(&self) -> bool {
        self.identifiers_chirho.is_empty()
    }

    pub fn iter_chirho(
        &self,
    ) -> impl Iterator<Item = &'environment_chirho Arc<InstDeclChirho>> + use<'environment_chirho>
    {
        let environment_chirho = self.environment_chirho;
        self.identifiers_chirho.iter().map(move |id_chirho| {
            environment_chirho
                .instance_record_chirho(id_chirho)
                .expect("instance view ids belong to their private arena")
        })
    }

    pub fn first_chirho(&self) -> Option<&'environment_chirho Arc<InstDeclChirho>> {
        self.identifiers_chirho
            .first()
            .and_then(|id_chirho| self.environment_chirho.instance_record_chirho(id_chirho))
    }
}

#[derive(Clone, Copy)]
pub struct InstanceNamesViewChirho<'environment_chirho> {
    environment_chirho: &'environment_chirho ClassEnvChirho,
}

impl<'environment_chirho> InstanceNamesViewChirho<'environment_chirho> {
    pub(super) fn new_chirho(environment_chirho: &'environment_chirho ClassEnvChirho) -> Self {
        Self { environment_chirho }
    }

    pub fn get_chirho(
        &self,
        name_chirho: &str,
    ) -> Option<InstanceListViewChirho<'environment_chirho>> {
        self.environment_chirho
            .instance_names_chirho
            .get(name_chirho)
            .map(|ids_chirho| InstanceListViewChirho {
                environment_chirho: self.environment_chirho,
                identifiers_chirho: ids_chirho,
            })
    }

    pub fn contains_key_chirho(&self, name_chirho: &str) -> bool {
        self.environment_chirho
            .instance_names_chirho
            .contains_key(name_chirho)
    }

    pub fn is_empty_chirho(&self) -> bool {
        self.environment_chirho.instance_names_chirho.is_empty()
    }
    pub fn len_chirho(&self) -> usize {
        self.environment_chirho.instance_names_chirho.len()
    }

    pub fn keys_chirho(
        &self,
    ) -> impl Iterator<Item = &'environment_chirho String> + use<'environment_chirho> {
        self.environment_chirho.instance_names_chirho.keys()
    }

    pub fn iter_chirho(
        &self,
    ) -> impl Iterator<
        Item = (
            &'environment_chirho String,
            InstanceListViewChirho<'environment_chirho>,
        ),
    > + use<'environment_chirho> {
        let environment_chirho = self.environment_chirho;
        environment_chirho
            .instance_names_chirho
            .iter()
            .map(move |(name_chirho, ids_chirho)| {
                (
                    name_chirho,
                    InstanceListViewChirho {
                        environment_chirho,
                        identifiers_chirho: ids_chirho,
                    },
                )
            })
    }

    pub fn values_chirho(
        &self,
    ) -> impl Iterator<Item = InstanceListViewChirho<'environment_chirho>> + use<'environment_chirho>
    {
        self.iter_chirho()
            .map(|(_, instances_chirho)| instances_chirho)
    }
}

/// An owned view iterator, without copying any instance payload or id vector.
pub struct InstanceRecordsChirho<'environment_chirho> {
    environment_chirho: &'environment_chirho ClassEnvChirho,
    identifiers_chirho: std::slice::Iter<'environment_chirho, InstanceRecordIdChirho>,
}

impl<'environment_chirho> Iterator for InstanceRecordsChirho<'environment_chirho> {
    type Item = &'environment_chirho Arc<InstDeclChirho>;
    fn next(&mut self) -> Option<Self::Item> {
        self.identifiers_chirho.next().map(|id_chirho| {
            self.environment_chirho
                .instance_record_chirho(id_chirho)
                .expect("instance view ids belong to their private arena")
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.identifiers_chirho.size_hint()
    }
}
impl ExactSizeIterator for InstanceRecordsChirho<'_> {}
impl<'environment_chirho> IntoIterator for InstanceListViewChirho<'environment_chirho> {
    type Item = &'environment_chirho Arc<InstDeclChirho>;
    type IntoIter = InstanceRecordsChirho<'environment_chirho>;
    fn into_iter(self) -> Self::IntoIter {
        InstanceRecordsChirho {
            environment_chirho: self.environment_chirho,
            identifiers_chirho: self.identifiers_chirho.iter(),
        }
    }
}
