// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Exact constructor fields follow their declaring owner through interface selection.
//! Workflow: compiler-pipeline-chirho/value-scope-resolution-chirho.

use haskelujah_ast_chirho::decl_chirho::ConDeclChirho;
use haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho;

use super::{IfaceExportsChirho, IfaceValueChirho};
use crate::type_exports_chirho::canonical_value_name_chirho;

/// A complete physical field sequence, independent of which labels are exposed
/// or callable. An explicit empty sequence belongs to a known fieldless
/// constructor; absent metadata supplies no constructor or field proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstructorFieldsChirho {
    KnownChirho {
        owner_declaration_chirho: TypeConstructorDeclarationChirho,
        labels_chirho: Vec<String>,
    },
    /// A merge lost an account that previously claimed positive authority.
    /// This is absorbing, and must never reopen legacy absence fallback.
    BlockedChirho,
}

impl ConstructorFieldsChirho {
    pub fn known_chirho(&self) -> Option<(&TypeConstructorDeclarationChirho, &[String])> {
        match self {
            Self::KnownChirho {
                owner_declaration_chirho,
                labels_chirho,
            } => Some((owner_declaration_chirho, labels_chirho)),
            Self::BlockedChirho => None,
        }
    }
}

/// Called only for an actual constructor declaration and its actual owner.
pub(crate) fn publish_source_constructor_fields_chirho(
    exports_chirho: &mut IfaceExportsChirho,
    constructor_chirho: &ConDeclChirho,
    owner_declaration_chirho: Option<TypeConstructorDeclarationChirho>,
) {
    let Some(owner_declaration_chirho) = owner_declaration_chirho else {
        return;
    };
    let name_chirho = match constructor_chirho {
        ConDeclChirho::OrdinaryChirho { name_chirho, .. }
        | ConDeclChirho::RecordChirho { name_chirho, .. }
        | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho,
    };
    let Some(value_chirho) = exports_chirho
        .values_chirho
        .get_mut(&canonical_value_name_chirho(name_chirho.text_chirho()))
    else {
        return;
    };
    value_chirho.constructor_fields_chirho = Some(ConstructorFieldsChirho::KnownChirho {
        owner_declaration_chirho,
        labels_chirho: constructor_chirho
            .record_fields_chirho()
            .unwrap_or_default()
            .iter()
            .flat_map(|field_chirho| &field_chirho.names_chirho)
            .map(|field_chirho| canonical_value_name_chirho(field_chirho.text_chirho()))
            .collect(),
    });
}

/// Fragment union preserves authority only when the complete accounts agree.
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
        existing_chirho.constructor_fields_chirho = Some(ConstructorFieldsChirho::BlockedChirho);
    }
}
