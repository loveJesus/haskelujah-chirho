// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One field map value keeps physical order apart from wildcard binder authority.
//! Workflow: compiler-pipeline-chirho/value-scope-resolution-chirho.

use haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructorFieldScopeChirho {
    state_chirho: FieldScopeStateChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FieldScopeStateChirho {
    ReadableChirho(FieldScopeReadChirho),
    BlockedChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FieldScopeReadChirho {
    SelectedChirho {
        owner_declaration_chirho: TypeConstructorDeclarationChirho,
        physical_labels_chirho: Vec<String>,
        eligible_labels_chirho: Vec<String>,
    },
    LegacyUnprovenChirho {
        physical_labels_chirho: Vec<String>,
    },
}

impl ConstructorFieldScopeChirho {
    /// Called from a real declaration/selected-label producer. Eligibility is
    /// a subset, never a replacement for the complete physical sequence.
    pub fn selected_chirho(
        owner_declaration_chirho: TypeConstructorDeclarationChirho,
        physical_labels_chirho: Vec<String>,
        eligible_labels_chirho: Vec<String>,
    ) -> Self {
        assert!(
            eligible_labels_chirho
                .iter()
                .all(|label_chirho| physical_labels_chirho.contains(label_chirho)),
            "compiler invariant: wildcard label outside physical constructor fields Chirho"
        );
        Self {
            state_chirho: FieldScopeStateChirho::ReadableChirho(
                FieldScopeReadChirho::SelectedChirho {
                    owner_declaration_chirho,
                    physical_labels_chirho,
                    eligible_labels_chirho,
                },
            ),
        }
    }

    /// Existing ownerless builtin/raw input stays explicitly unproven. Neither
    /// this compatibility constructor nor its fields grant lexical authority.
    pub fn legacy_unproven_chirho(physical_labels_chirho: Vec<String>) -> Self {
        Self {
            state_chirho: FieldScopeStateChirho::ReadableChirho(
                FieldScopeReadChirho::LegacyUnprovenChirho {
                    physical_labels_chirho,
                },
            ),
        }
    }

    pub fn blocked_chirho() -> Self {
        Self {
            state_chirho: FieldScopeStateChirho::BlockedChirho,
        }
    }

    pub fn is_blocked_chirho(&self) -> bool {
        matches!(self.state_chirho, FieldScopeStateChirho::BlockedChirho)
    }

    pub fn physical_labels_chirho(&self) -> Option<&[String]> {
        match &self.state_chirho {
            FieldScopeStateChirho::ReadableChirho(
                FieldScopeReadChirho::SelectedChirho {
                    physical_labels_chirho,
                    ..
                }
                | FieldScopeReadChirho::LegacyUnprovenChirho {
                    physical_labels_chirho,
                },
            ) => Some(physical_labels_chirho),
            FieldScopeStateChirho::BlockedChirho => None,
        }
    }

    /// The pattern reader preserves the explicitly named legacy route. Future
    /// lexical admission must instead consume proven_wildcard_labels_chirho.
    pub fn wildcard_binding_labels_chirho(&self) -> Option<&[String]> {
        match &self.state_chirho {
            FieldScopeStateChirho::ReadableChirho(FieldScopeReadChirho::SelectedChirho {
                eligible_labels_chirho,
                ..
            }) => Some(eligible_labels_chirho),
            FieldScopeStateChirho::ReadableChirho(FieldScopeReadChirho::LegacyUnprovenChirho {
                physical_labels_chirho,
            }) => Some(physical_labels_chirho),
            FieldScopeStateChirho::BlockedChirho => None,
        }
    }

    pub fn proven_wildcard_labels_chirho(
        &self,
    ) -> Option<(&TypeConstructorDeclarationChirho, &[String])> {
        match &self.state_chirho {
            FieldScopeStateChirho::ReadableChirho(FieldScopeReadChirho::SelectedChirho {
                owner_declaration_chirho,
                eligible_labels_chirho,
                ..
            }) => Some((owner_declaration_chirho, eligible_labels_chirho)),
            FieldScopeStateChirho::ReadableChirho(FieldScopeReadChirho::LegacyUnprovenChirho {
                ..
            })
            | FieldScopeStateChirho::BlockedChirho => None,
        }
    }
}
