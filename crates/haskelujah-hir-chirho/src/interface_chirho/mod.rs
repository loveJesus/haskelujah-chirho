// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Arena-free checked-interface data. Visibility is separate from defining identity.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use crate::{NameIdChirho, OriginalNameChirho, UnitIdentityChirho};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceBuiltinChirho {
    TypeChirho,
    ConstraintChirho,
    BoolChirho,
    CharChirho,
    IntegerChirho,
    ListChirho,
    AnyChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceTypeChirho {
    BuiltinChirho(InterfaceBuiltinChirho),
    DeclaredChirho(OriginalNameChirho),
    ClassChirho(OriginalNameChirho),
    BoundChirho(u32),
    AppChirho(Box<Self>, Box<Self>),
    FunChirho(Box<Self>, Box<Self>),
    TupleChirho(Vec<Self>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceSchemeChirho {
    pub kinds_chirho: Vec<InterfaceTypeChirho>,
    pub context_chirho: Vec<InterfaceTypeChirho>,
    pub body_chirho: InterfaceTypeChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceValueChirho {
    pub scheme_chirho: InterfaceSchemeChirho,
    /// None is an ordinary value; Some is the checked constructor's field arity.
    pub constructor_arity_chirho: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceMethodChirho {
    pub name_chirho: OriginalNameChirho,
    pub default_chirho: Option<OriginalNameChirho>,
    pub scheme_chirho: InterfaceSchemeChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceClassChirho {
    pub name_chirho: OriginalNameChirho,
    pub parameter_kind_chirho: InterfaceTypeChirho,
    pub superclasses_chirho: Vec<InterfaceTypeChirho>,
    pub superclass_selectors_chirho: Vec<OriginalNameChirho>,
    pub methods_chirho: Vec<InterfaceMethodChirho>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceInstanceChirho {
    pub dfun_chirho: OriginalNameChirho,
    pub class_chirho: OriginalNameChirho,
    pub scheme_chirho: InterfaceSchemeChirho,
}

/// Only the independent checker projects this in the production source driver.
/// No disk loader or trust in a caller-supplied/legacy interface is implied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceChirho {
    pub unit_chirho: UnitIdentityChirho,
    pub module_chirho: String,
    pub exports_chirho: Vec<OriginalNameChirho>,
    pub data_chirho: BTreeMap<OriginalNameChirho, usize>,
    pub values_chirho: BTreeMap<OriginalNameChirho, InterfaceValueChirho>,
    pub classes_chirho: BTreeMap<OriginalNameChirho, InterfaceClassChirho>,
    pub instances_chirho: BTreeMap<OriginalNameChirho, InterfaceInstanceChirho>,
}

#[derive(Debug, Clone)]
pub struct ImportedInterfaceChirho {
    pub interface_chirho: std::sync::Arc<InterfaceChirho>,
    pub names_chirho: BTreeMap<OriginalNameChirho, NameIdChirho>,
}
