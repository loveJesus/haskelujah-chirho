// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Compiler-independent identities and terms. No legacy type or evidence lookup.

mod groups_chirho;
mod lower_chirho;
mod names_chirho;
mod terms_chirho;

pub use groups_chirho::binding_groups_chirho;
pub use lower_chirho::lower_module_chirho;
pub use names_chirho::{NameIdChirho, NameInfoChirho, UnitIdentityChirho, WiredNameChirho};
pub use terms_chirho::{
    BindingChirho, ConstructorChirho, DataDeclChirho, ExprChirho, ExprIdChirho, ExprKindChirho,
    HirErrorChirho, HirErrorKindChirho, LimitsChirho, MatchArmChirho, ModuleChirho, PatternChirho,
    PatternKindChirho, SurfaceTypeChirho,
};
