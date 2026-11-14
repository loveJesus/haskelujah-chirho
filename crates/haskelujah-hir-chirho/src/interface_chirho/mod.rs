// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Arena-free checked-interface data. Visibility is separate from defining identity.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use crate::{NameIdChirho, OriginalNameChirho, UnitIdentityChirho};
use std::collections::{BTreeMap, BTreeSet};

/// A checked fragment carries presence facts only; it never certifies module absence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompletenessChirho {
    #[default]
    CompleteChirho,
    SliceChirho,
}

pub type PreludeExportsChirho = BTreeSet<(String, crate::NamespaceChirho)>;

#[derive(Debug, Default)]
pub struct LoweringPolicyChirho {
    /// Checked export names only; no Prelude scheme or instance authority.
    pub prelude_exports_chirho: Option<std::sync::Arc<PreludeExportsChirho>>,
    pub completeness_chirho: CompletenessChirho,
    /// Identity dependencies without lexical import scope (known syntax operations).
    pub implicit_interfaces_chirho: Vec<std::sync::Arc<InterfaceChirho>>,
    /// Driver manifests may admit checked signatures with explicitly supplied backend bodies.
    pub primitive_bindings_chirho: BTreeMap<OriginalNameChirho, usize>,
    /// Own wired type declarations, distinct from ordinary backend-implemented value names.
    pub wired_declarations_chirho: BTreeMap<OriginalNameChirho, crate::WiredNameChirho>,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssociativityChirho {
    NoneChirho,
    LeftChirho,
    RightChirho,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterfaceFixityChirho {
    pub associativity_chirho: AssociativityChirho,
    pub precedence_chirho: u8,
}

/// Only the independent checker projects this in the production source driver.
/// No disk loader or trust in a caller-supplied/legacy interface is implied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceChirho {
    pub unit_chirho: UnitIdentityChirho,
    pub module_chirho: String,
    pub exports_chirho: Vec<OriginalNameChirho>,
    pub fixities_chirho: BTreeMap<OriginalNameChirho, InterfaceFixityChirho>,
    pub completeness_chirho: CompletenessChirho,
    pub sliced_units_chirho: BTreeSet<UnitIdentityChirho>,
    /// Driver-provided wired declarations, never inferred from an exported spelling.
    pub wired_types_chirho: BTreeMap<OriginalNameChirho, crate::WiredNameChirho>,
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
