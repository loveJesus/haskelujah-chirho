// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Which source occurrence a minted occurrence id stands for. The desugarer
//! never mints an origin: it copies the one the producer put on the AST node,
//! together with the role the use plays, so the evidence join can match the
//! checker's record to this id by identity instead of by span or position.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::provenance_chirho::{
    OccurrenceRoleChirho, OriginIdChirho, ProvenanceChirho,
};

use super::DesugarCtxChirho;
use crate::expr_chirho::CoreIdChirho;

impl DesugarCtxChirho {
    /// Record that occurrence id `id_chirho` is the use of `origin_chirho` in
    /// `role_chirho`. Only a minted occurrence is recorded: a shared canonical
    /// id stands for every use of a name at once, so it is no one occurrence.
    pub(super) fn record_occurrence_provenance_chirho(
        &mut self,
        id_chirho: CoreIdChirho,
        origin_chirho: Option<OriginIdChirho>,
        role_chirho: OccurrenceRoleChirho,
    ) {
        let Some(origin_chirho) = origin_chirho else {
            return;
        };
        if self.method_occurrences_chirho.contains_key(&id_chirho) {
            self.occurrence_provenance_chirho.insert(
                id_chirho,
                ProvenanceChirho::new_chirho(origin_chirho, role_chirho),
            );
        }
    }
}
