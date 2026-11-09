// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Instance-row identity (S3): which source instance declaration's `$prim` rows a typed
//! instance owns. The desugarer names a declaration's rows by its head as written
//! (`$prim_SizeChirho_sizeChirho_Maybe aChirho`). The checker registers the same declaration
//! with a freshened head (`Maybe t17`). Neither spelling is parsed or compared here: each
//! producer reports the declaration by its class and span, and this join pairs them.
//!
//! A place (a class and a span) joins only when each producer reports exactly one thing
//! there: one typed instance object and one row key, exact repeats dropped. Any other count
//! makes every instance at that place ambiguous, by name. Derived, spliced and synthetic
//! declarations can share a span, so nothing is ever chosen by iteration order.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::{HashMap, HashSet};

use super::generic_heads_chirho::head_key_chirho;
use std::sync::Arc;

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::InstDeclChirho;

/// One source instance declaration as the checker registered it (S3, typing producer).
#[derive(Debug, Clone)]
pub struct InstanceDeclarationChirho {
    pub span_chirho: SpanChirho,
    pub instance_chirho: Arc<InstDeclChirho>,
}

/// The key one source instance declaration's rows were named with (S3, desugar producer).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstanceRowKeyChirho {
    pub class_name_chirho: String,
    pub span_chirho: SpanChirho,
    pub row_key_chirho: String,
}

/// Why a typed instance has no row key in this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::dict_chirho) enum RowKeyAbsenceChirho {
    /// No declaration of this module reports the instance together with its rows.
    ElsewhereChirho,
    /// Its place holds several instances or several row keys, so no pairing is proven.
    AmbiguousChirho,
}

/// The checked join of the two producers.
#[derive(Debug, Default)]
pub(in crate::dict_chirho) struct InstanceRowsChirho {
    joined_chirho: Vec<(Arc<InstDeclChirho>, String)>,
    ambiguous_chirho: Vec<Arc<InstDeclChirho>>,
    /// Denial-only source routes, including declarations/row keys the positive join
    /// could not pair. Membership grants no identity, owner or method body.
    source_route_claims_chirho: HashSet<(String, String)>,
}

/// Whether two typed instances are the same object: class, head, extra heads and context
/// all equal, variables included. Two declarations written alike are two objects, because
/// the checker freshened their variables apart.
pub(in crate::dict_chirho) fn same_instance_chirho(
    left_chirho: &InstDeclChirho,
    right_chirho: &InstDeclChirho,
) -> bool {
    left_chirho.class_name_chirho == right_chirho.class_name_chirho
        && left_chirho.head_ty_chirho == right_chirho.head_ty_chirho
        && left_chirho.extra_head_tys_chirho == right_chirho.extra_head_tys_chirho
        && left_chirho.context_chirho == right_chirho.context_chirho
}

impl InstanceRowsChirho {
    /// Pair each place's one typed instance with its one row key.
    pub(in crate::dict_chirho) fn join_chirho(
        declarations_chirho: &[InstanceDeclarationChirho],
        row_keys_chirho: &[InstanceRowKeyChirho],
    ) -> Self {
        let mut instances_by_place_chirho: HashMap<(&str, SpanChirho), Vec<&Arc<InstDeclChirho>>> =
            HashMap::new();
        for declaration_chirho in declarations_chirho {
            let instance_chirho = &declaration_chirho.instance_chirho;
            let instances_chirho = instances_by_place_chirho
                .entry((
                    instance_chirho.class_name_chirho.as_str(),
                    declaration_chirho.span_chirho,
                ))
                .or_default();
            if !instances_chirho
                .iter()
                .any(|known_chirho| same_instance_chirho(known_chirho, instance_chirho))
            {
                instances_chirho.push(instance_chirho);
            }
        }
        let mut keys_by_place_chirho: HashMap<(&str, SpanChirho), Vec<&str>> = HashMap::new();
        for row_key_chirho in row_keys_chirho {
            let keys_chirho = keys_by_place_chirho
                .entry((
                    row_key_chirho.class_name_chirho.as_str(),
                    row_key_chirho.span_chirho,
                ))
                .or_default();
            if !keys_chirho.contains(&row_key_chirho.row_key_chirho.as_str()) {
                keys_chirho.push(row_key_chirho.row_key_chirho.as_str());
            }
        }
        let mut rows_chirho = Self {
            source_route_claims_chirho: row_keys_chirho
                .iter()
                .map(|row_chirho| {
                    (
                        row_chirho.class_name_chirho.clone(),
                        row_chirho.row_key_chirho.clone(),
                    )
                })
                .chain(
                    declarations_chirho
                        .iter()
                        .filter(|declaration_chirho| {
                            keys_by_place_chirho.contains_key(&(
                                declaration_chirho
                                    .instance_chirho
                                    .class_name_chirho
                                    .as_str(),
                                declaration_chirho.span_chirho,
                            ))
                        })
                        .map(|declaration_chirho| {
                            let instance_chirho = &declaration_chirho.instance_chirho;
                            (
                                instance_chirho.class_name_chirho.clone(),
                                head_key_chirho(
                                    &instance_chirho.head_ty_chirho,
                                    &instance_chirho.extra_head_tys_chirho,
                                ),
                            )
                        }),
                )
                .collect(),
            ..Self::default()
        };
        for (place_chirho, instances_chirho) in &instances_by_place_chirho {
            match (
                instances_chirho.as_slice(),
                keys_by_place_chirho.get(place_chirho).map(Vec::as_slice),
            ) {
                (_, None) => {}
                ([instance_chirho], Some([row_key_chirho])) => rows_chirho
                    .joined_chirho
                    .push(((*instance_chirho).clone(), (*row_key_chirho).to_string())),
                (instances_chirho, Some(_)) => rows_chirho.ambiguous_chirho.extend(
                    instances_chirho
                        .iter()
                        .map(|instance_chirho| (*instance_chirho).clone()),
                ),
            }
        }
        rows_chirho
    }

    /// The row key of exactly this typed instance object, or why it has none. An object
    /// paired with two different keys is ambiguous too.
    pub(in crate::dict_chirho) fn row_key_chirho(
        &self,
        instance_chirho: &InstDeclChirho,
    ) -> Result<&str, RowKeyAbsenceChirho> {
        if self
            .ambiguous_chirho
            .iter()
            .any(|known_chirho| same_instance_chirho(known_chirho, instance_chirho))
        {
            return Err(RowKeyAbsenceChirho::AmbiguousChirho);
        }
        let mut keys_chirho = self
            .joined_chirho
            .iter()
            .filter(|(known_chirho, _)| same_instance_chirho(known_chirho, instance_chirho))
            .map(|(_, row_key_chirho)| row_key_chirho.as_str());
        let Some(row_key_chirho) = keys_chirho.next() else {
            return Err(RowKeyAbsenceChirho::ElsewhereChirho);
        };
        if keys_chirho.any(|other_chirho| other_chirho != row_key_chirho) {
            return Err(RowKeyAbsenceChirho::AmbiguousChirho);
        }
        Ok(row_key_chirho)
    }

    /// Borrow original source claims only to deny unowned fallback. Ambiguous or
    /// unpaired inputs stay here; they never enter the positive row iterator.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn source_route_claims_chirho(
        &self,
    ) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.source_route_claims_chirho
            .iter()
            .map(|(class_chirho, route_chirho)| (class_chirho.as_str(), route_chirho.as_str()))
    }

    /// Each instance with a proven row key, and that key, for the context-leading marks.
    pub(in crate::dict_chirho) fn proven_rows_chirho(
        &self,
    ) -> impl Iterator<Item = (&InstDeclChirho, &str)> + '_ {
        self.joined_chirho
            .iter()
            .filter_map(|(instance_chirho, _)| {
                self.row_key_chirho(instance_chirho)
                    .ok()
                    .map(|row_key_chirho| (instance_chirho.as_ref(), row_key_chirho))
            })
    }
}
