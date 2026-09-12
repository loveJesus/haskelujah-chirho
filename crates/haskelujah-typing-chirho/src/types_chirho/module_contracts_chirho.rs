// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Checked module semantics, separate from naming's import/export authority.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use crate::infer_chirho::TypeSynonymChirho;
use crate::kind_chirho::KindContractChirho;
use crate::{SchemeChirho, TyChirho};
use std::collections::HashMap;

/// Names in this scope have checked contracts; absence does not authorize a
/// guessed kind or a fabricated alias. The driver supplies visible names and
/// canonical private dependencies without making those dependencies exports.
#[derive(Clone, Debug, Default)]
pub struct ModuleTypeContractsChirho {
    pub kinds_chirho: HashMap<String, KindContractChirho>,
    pub synonyms_chirho: HashMap<String, TypeSynonymChirho>,
}

impl SchemeChirho {
    /// Rename type constructor references without changing quantifier identities,
    /// predicates' class identities, or their order.
    pub fn map_constructor_names_chirho(
        &self,
        rename_chirho: &mut impl FnMut(&str) -> String,
    ) -> Self {
        let mut mapped_chirho = self.clone();
        mapped_chirho.ty_chirho = self.ty_chirho.map_constructor_names_chirho(rename_chirho);
        for predicate_chirho in &mut mapped_chirho.preds_chirho {
            predicate_chirho.ty_chirho = predicate_chirho
                .ty_chirho
                .map_constructor_names_chirho(rename_chirho);
            for argument_chirho in &mut predicate_chirho.extra_tys_chirho {
                *argument_chirho = argument_chirho.map_constructor_names_chirho(rename_chirho);
            }
        }
        mapped_chirho
    }
}

impl TyChirho {
    /// One exhaustive constructor-name traversal for import qualification and
    /// closed-alias transport. Quantifier identities and visibility do not change.
    pub fn map_constructor_names_chirho(
        &self,
        rename_chirho: &mut impl FnMut(&str) -> String,
    ) -> Self {
        match self {
            Self::ConChirho(name_chirho) => Self::ConChirho(rename_chirho(name_chirho)),
            Self::VarChirho(_) | Self::ForallVarChirho(_) => self.clone(),
            Self::AppChirho(fun_chirho, arg_chirho) => Self::AppChirho(
                Box::new(fun_chirho.map_constructor_names_chirho(rename_chirho)),
                Box::new(arg_chirho.map_constructor_names_chirho(rename_chirho)),
            ),
            Self::KindAppChirho(fun_chirho, arg_chirho) => Self::KindAppChirho(
                Box::new(fun_chirho.map_constructor_names_chirho(rename_chirho)),
                Box::new(arg_chirho.map_constructor_names_chirho(rename_chirho)),
            ),
            Self::FunChirho(arg_chirho, result_chirho, multiplicity_chirho) => Self::FunChirho(
                Box::new(arg_chirho.map_constructor_names_chirho(rename_chirho)),
                Box::new(result_chirho.map_constructor_names_chirho(rename_chirho)),
                *multiplicity_chirho,
            ),
            Self::TupleChirho(elements_chirho) => Self::TupleChirho(
                elements_chirho
                    .iter()
                    .map(|element_chirho| {
                        element_chirho.map_constructor_names_chirho(rename_chirho)
                    })
                    .collect(),
            ),
            Self::ListChirho(element_chirho) => Self::ListChirho(Box::new(
                element_chirho.map_constructor_names_chirho(rename_chirho),
            )),
            Self::ForallChirho {
                vars_chirho,
                body_chirho,
            } => Self::ForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(body_chirho.map_constructor_names_chirho(rename_chirho)),
            },
            Self::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => Self::RequiredForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(body_chirho.map_constructor_names_chirho(rename_chirho)),
            },
        }
    }
}
