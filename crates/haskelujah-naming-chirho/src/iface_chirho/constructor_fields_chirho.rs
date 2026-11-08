// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Actual constructor and field-label declarations survive interface selection.
//! Workflow: compiler-pipeline-chirho/value-scope-resolution-chirho.

use haskelujah_ast_chirho::decl_chirho::ConDeclChirho;
use haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho;
use haskelujah_span_chirho::SpanChirho;

use super::{IfaceExportsChirho, IfaceValueAvailabilityChirho, IfaceValueChirho};
use crate::type_exports_chirho::canonical_value_name_chirho;

/// Positive declarations and absorbing loss share the existing value payload.
/// The private two-level account cannot mix constructor fields with label owners.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructorFieldsChirho {
    account_chirho: FieldAccountChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FieldAccountChirho {
    ProvenChirho(FieldDeclarationChirho),
    BlockedChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FieldDeclarationChirho {
    ConstructorChirho {
        owner_declaration_chirho: TypeConstructorDeclarationChirho,
        labels_chirho: Vec<String>,
    },
    LabelChirho {
        owner_declarations_chirho: Vec<TypeConstructorDeclarationChirho>,
    },
}

impl ConstructorFieldsChirho {
    fn constructor_chirho(
        owner_declaration_chirho: TypeConstructorDeclarationChirho,
        labels_chirho: Vec<String>,
    ) -> Self {
        Self {
            account_chirho: FieldAccountChirho::ProvenChirho(
                FieldDeclarationChirho::ConstructorChirho {
                    owner_declaration_chirho,
                    labels_chirho,
                },
            ),
        }
    }

    fn label_chirho(owner_declaration_chirho: TypeConstructorDeclarationChirho) -> Self {
        Self {
            account_chirho: FieldAccountChirho::ProvenChirho(FieldDeclarationChirho::LabelChirho {
                owner_declarations_chirho: vec![owner_declaration_chirho],
            }),
        }
    }

    pub(super) fn blocked_chirho() -> Self {
        Self {
            account_chirho: FieldAccountChirho::BlockedChirho,
        }
    }

    pub fn is_blocked_chirho(&self) -> bool {
        matches!(self.account_chirho, FieldAccountChirho::BlockedChirho)
    }

    pub fn known_chirho(&self) -> Option<(&TypeConstructorDeclarationChirho, &[String])> {
        match &self.account_chirho {
            FieldAccountChirho::ProvenChirho(FieldDeclarationChirho::ConstructorChirho {
                owner_declaration_chirho,
                labels_chirho,
            }) => Some((owner_declaration_chirho, labels_chirho)),
            FieldAccountChirho::ProvenChirho(FieldDeclarationChirho::LabelChirho { .. })
            | FieldAccountChirho::BlockedChirho => None,
        }
    }

    pub fn label_owners_chirho(&self) -> Option<&[TypeConstructorDeclarationChirho]> {
        match &self.account_chirho {
            FieldAccountChirho::ProvenChirho(FieldDeclarationChirho::LabelChirho {
                owner_declarations_chirho,
            }) => Some(owner_declarations_chirho),
            FieldAccountChirho::ProvenChirho(FieldDeclarationChirho::ConstructorChirho {
                ..
            })
            | FieldAccountChirho::BlockedChirho => None,
        }
    }

    fn add_source_label_owner_chirho(&mut self, owner_chirho: &TypeConstructorDeclarationChirho) {
        match &mut self.account_chirho {
            FieldAccountChirho::ProvenChirho(FieldDeclarationChirho::LabelChirho {
                owner_declarations_chirho,
            }) => {
                if !owner_declarations_chirho.contains(owner_chirho) {
                    owner_declarations_chirho.push(owner_chirho.clone());
                    owner_declarations_chirho
                        .sort_by_key(TypeConstructorDeclarationChirho::type_key_chirho);
                }
            }
            FieldAccountChirho::ProvenChirho(FieldDeclarationChirho::ConstructorChirho {
                ..
            }) => self.account_chirho = FieldAccountChirho::BlockedChirho,
            FieldAccountChirho::BlockedChirho => {}
        }
    }
}

/// Called at the real declaration producer, before later unrelated bindings
/// can replace an exported value. No post-hoc spelling upgrades a field label.
pub(crate) fn publish_source_constructor_values_chirho(
    exports_chirho: &mut IfaceExportsChirho,
    constructors_chirho: &[ConDeclChirho],
    owner_chirho: Option<&TypeConstructorDeclarationChirho>,
    defining_module_chirho: &Option<String>,
    span_chirho: SpanChirho,
) {
    for constructor_chirho in constructors_chirho {
        let name_chirho = match constructor_chirho {
            ConDeclChirho::OrdinaryChirho { name_chirho, .. }
            | ConDeclChirho::RecordChirho { name_chirho, .. }
            | ConDeclChirho::GadtChirho { name_chirho, .. } => {
                canonical_value_name_chirho(name_chirho.text_chirho())
            }
        };
        let labels_chirho: Vec<String> = constructor_chirho
            .record_fields_chirho()
            .unwrap_or_default()
            .iter()
            .flat_map(|field_chirho| &field_chirho.names_chirho)
            .map(|name_chirho| canonical_value_name_chirho(name_chirho.text_chirho()))
            .collect();
        exports_chirho.values_chirho.insert(
            name_chirho.clone(),
            IfaceValueChirho {
                availability_chirho: IfaceValueAvailabilityChirho::AvailableChirho,
                name_chirho,
                span_chirho,
                defining_module_chirho: defining_module_chirho.clone(),
                constructor_fields_chirho: owner_chirho.map(|owner_chirho| {
                    ConstructorFieldsChirho::constructor_chirho(
                        owner_chirho.clone(),
                        labels_chirho.clone(),
                    )
                }),
                compiler_value_chirho: None,
            },
        );
        for label_chirho in labels_chirho {
            use std::collections::hash_map::Entry;
            match exports_chirho.values_chirho.entry(label_chirho.clone()) {
                Entry::Vacant(entry_chirho) => {
                    entry_chirho.insert(IfaceValueChirho {
                        availability_chirho: IfaceValueAvailabilityChirho::AvailableChirho,
                        name_chirho: label_chirho,
                        span_chirho,
                        defining_module_chirho: defining_module_chirho.clone(),
                        constructor_fields_chirho: owner_chirho.map(|owner_chirho| {
                            ConstructorFieldsChirho::label_chirho(owner_chirho.clone())
                        }),
                        compiler_value_chirho: None,
                    });
                }
                Entry::Occupied(mut entry_chirho) => {
                    let value_chirho = entry_chirho.get_mut();
                    match (&mut value_chirho.constructor_fields_chirho, owner_chirho) {
                        (Some(account_chirho), Some(owner_chirho)) => {
                            account_chirho.add_source_label_owner_chirho(owner_chirho)
                        }
                        (account_chirho, _) => {
                            if account_chirho.is_some() || owner_chirho.is_some() {
                                *account_chirho = Some(ConstructorFieldsChirho::blocked_chirho());
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A fragment union can preserve an identical complete account or lose it;
/// it never broadens owners/fields or revives a later weakened fragment.
pub(super) fn merge_constructor_fields_chirho(
    existing_chirho: &mut IfaceValueChirho,
    incoming_chirho: &IfaceValueChirho,
) {
    let carried_account_chirho = existing_chirho.constructor_fields_chirho.is_some()
        || incoming_chirho.constructor_fields_chirho.is_some();
    if carried_account_chirho
        && (existing_chirho.constructor_fields_chirho != incoming_chirho.constructor_fields_chirho
            || existing_chirho.defining_module_chirho != incoming_chirho.defining_module_chirho)
    {
        existing_chirho.constructor_fields_chirho = Some(ConstructorFieldsChirho::blocked_chirho());
    }
}
