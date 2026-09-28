// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor names are values; an instance never rebinds its family's type name.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::*;

pub(super) fn bind_constructor_names_chirho(
    env_chirho: &mut NameEnvChirho,
    constructors_chirho: &[haskelujah_ast_chirho::decl_chirho::ConDeclChirho],
) {
    for con_chirho in constructors_chirho {
        match con_chirho {
            haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                name_chirho,
                ..
            }
            | haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                name_chirho, ..
            } => {
                bind_name_chirho(env_chirho, name_chirho, NamespaceChirho::ValueChirho);
            }
            haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                name_chirho,
                fields_chirho,
                ..
            } => {
                bind_name_chirho(env_chirho, name_chirho, NamespaceChirho::ValueChirho);
                // Record field accessor functions are value bindings
                for field_chirho in fields_chirho {
                    for field_name_chirho in &field_chirho.names_chirho {
                        bind_name_chirho(
                            env_chirho,
                            field_name_chirho,
                            NamespaceChirho::ValueChirho,
                        );
                    }
                }
            }
        }
    }
}
