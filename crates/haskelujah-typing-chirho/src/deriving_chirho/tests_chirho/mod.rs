// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_ast_chirho::decl_chirho::{DerivedClassChirho, StrictnessChirho};
use support_chirho::{make_enum_module_chirho, unspecified_chirho};

mod api_chirho;
#[path = "generators_chirho/enumeration_chirho.rs"]
mod enumeration_chirho;
#[path = "generators_chirho/generic_chirho.rs"]
mod generic_chirho;
#[path = "generators_chirho/higher_kinded_chirho.rs"]
mod higher_kinded_chirho;
#[path = "generators_chirho/structural_chirho.rs"]
mod structural_chirho;
mod support_chirho;
