// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type terms adapt to the same equation matcher used by kind terms.

use super::FamilyTermChirho;
use crate::ty_chirho::{MultChirho, TyChirho, TyVarChirho};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum FamilyTypeVariableChirho {
    InferenceChirho(TyVarChirho),
    NamedChirho(String),
}

impl FamilyTermChirho for TyChirho {
    type VariableChirho = FamilyTypeVariableChirho;
    fn variable_chirho(&self) -> Option<FamilyTypeVariableChirho> {
        match self {
            Self::VarChirho(variable_chirho) => {
                Some(FamilyTypeVariableChirho::InferenceChirho(*variable_chirho))
            }
            Self::ForallVarChirho(name_chirho) => {
                Some(FamilyTypeVariableChirho::NamedChirho(name_chirho.clone()))
            }
            _ => None,
        }
    }
    fn unknown_chirho(&self) -> bool {
        matches!(
            self,
            Self::VarChirho(_)
                | Self::ForallVarChirho(_)
                | Self::ForallChirho { .. }
                | Self::RequiredForallChirho { .. }
        )
    }
    fn head_name_chirho(&self) -> Option<&str> {
        match self {
            Self::ConChirho(name_chirho) => Some(name_chirho),
            Self::AppChirho(fun_chirho, _) => fun_chirho.head_name_chirho(),
            _ => None,
        }
    }
    fn parts_chirho(&self) -> Option<(&'static str, Vec<&Self>)> {
        match self {
            Self::AppChirho(fun_chirho, argument_chirho) => {
                Some(("application_chirho", vec![fun_chirho, argument_chirho]))
            }
            Self::FunChirho(argument_chirho, result_chirho, mult_chirho) => Some((
                if *mult_chirho == MultChirho::OneChirho {
                    "linear_chirho"
                } else {
                    "function_chirho"
                },
                vec![argument_chirho, result_chirho],
            )),
            Self::ListChirho(element_chirho) => Some(("list_chirho", vec![element_chirho])),
            Self::TupleChirho(elements_chirho) => {
                Some(("tuple_chirho", elements_chirho.iter().collect()))
            }
            _ => None,
        }
    }
    fn map_children_chirho(&self, map_chirho: &mut impl FnMut(&Self) -> Self) -> Self {
        match self {
            Self::AppChirho(fun_chirho, argument_chirho) => {
                Self::application_chirho(map_chirho(fun_chirho), map_chirho(argument_chirho))
            }
            Self::FunChirho(argument_chirho, result_chirho, mult_chirho) => Self::FunChirho(
                Box::new(map_chirho(argument_chirho)),
                Box::new(map_chirho(result_chirho)),
                *mult_chirho,
            ),
            Self::ListChirho(element_chirho) => {
                Self::ListChirho(Box::new(map_chirho(element_chirho)))
            }
            Self::TupleChirho(elements_chirho) => {
                Self::TupleChirho(elements_chirho.iter().map(map_chirho).collect())
            }
            // Family patterns cannot bind under a forall. Preserve that opaque
            // carrier, rather than accidentally substituting its bound variables.
            _ => self.clone(),
        }
    }
    fn application_chirho(fun_chirho: Self, argument_chirho: Self) -> Self {
        Self::AppChirho(Box::new(fun_chirho), Box::new(argument_chirho))
    }
}
