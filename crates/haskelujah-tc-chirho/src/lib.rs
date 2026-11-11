// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! An independent, elaborating checker. It never calls legacy typing.

mod check_chirho;
mod error_chirho;
mod schemes_chirho;
#[cfg(test)]
mod schemes_tests_chirho;
mod solver_chirho;
#[cfg(test)]
mod tests_chirho;
mod types_chirho;

pub use check_chirho::{
    AbstractionChirho, CheckedModuleChirho, WrapStepChirho, check_module_chirho,
};
pub use error_chirho::{TcErrorChirho, TcErrorKindChirho};
pub use schemes_chirho::SchemeChirho;
pub use solver_chirho::{
    ClassInfoChirho, EqualityChirho, EvTermChirho, EvVarChirho, GivenDictChirho,
    InstanceInfoChirho, InstanceMethodInfoChirho, MethodInfoChirho, ScopeIdChirho, ScopeKindChirho,
    SolverChirho, WantedDictChirho, WantedOriginChirho,
};
pub use types_chirho::{
    MetaIdChirho, RigidIdChirho, TyChirho, TyConChirho, TyNodeChirho, TypeArenaChirho,
};
