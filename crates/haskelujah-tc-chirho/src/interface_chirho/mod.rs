// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Only closed types over original names cross module boundaries.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use crate::*;
use haskelujah_hir_chirho::*;
use std::collections::{BTreeMap, HashMap};
mod export_chirho;
mod import_chirho;
pub use export_chirho::project_interface_chirho;

fn invalid_interface_chirho(message_chirho: &str) -> TcErrorChirho {
    TcErrorChirho::new_chirho(TcErrorKindChirho::InvalidReferenceChirho, message_chirho)
}

fn builtin_out_chirho(con_chirho: TyConChirho) -> Option<InterfaceBuiltinChirho> {
    Some(match con_chirho {
        TyConChirho::TypeChirho => InterfaceBuiltinChirho::TypeChirho,
        TyConChirho::ConstraintChirho => InterfaceBuiltinChirho::ConstraintChirho,
        TyConChirho::BoolChirho => InterfaceBuiltinChirho::BoolChirho,
        TyConChirho::CharChirho => InterfaceBuiltinChirho::CharChirho,
        TyConChirho::IntegerChirho => InterfaceBuiltinChirho::IntegerChirho,
        TyConChirho::ListChirho => InterfaceBuiltinChirho::ListChirho,
        TyConChirho::AnyChirho => InterfaceBuiltinChirho::AnyChirho,
        _ => return None,
    })
}
fn builtin_in_chirho(con_chirho: InterfaceBuiltinChirho) -> TyConChirho {
    match con_chirho {
        InterfaceBuiltinChirho::TypeChirho => TyConChirho::TypeChirho,
        InterfaceBuiltinChirho::ConstraintChirho => TyConChirho::ConstraintChirho,
        InterfaceBuiltinChirho::BoolChirho => TyConChirho::BoolChirho,
        InterfaceBuiltinChirho::CharChirho => TyConChirho::CharChirho,
        InterfaceBuiltinChirho::IntegerChirho => TyConChirho::IntegerChirho,
        InterfaceBuiltinChirho::ListChirho => TyConChirho::ListChirho,
        InterfaceBuiltinChirho::AnyChirho => TyConChirho::AnyChirho,
    }
}
