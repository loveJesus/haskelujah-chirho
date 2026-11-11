// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Temporary adapter into Core's existing type data. No legacy inference/unification is called.
//! The aggregate output ceiling also bounds expansion of the interned DAG into the old tree.
//! workflow: next-pipeline-chirho/check-and-elaborate-chirho

use super::{NextErrorChirho, NextErrorKindChirho};
use haskelujah_hir_chirho::NameIdChirho;
use haskelujah_tc_chirho::{CheckedModuleChirho, TyChirho, TyConChirho, TyNodeChirho};
use haskelujah_typing_chirho::ty_chirho::{TyChirho as CoreTypeChirho, TyVarChirho};

pub(super) struct CoreTypesChirho<'checked_chirho> {
    pub(super) checked_chirho: &'checked_chirho CheckedModuleChirho,
    remaining_chirho: usize,
}

impl<'checked_chirho> CoreTypesChirho<'checked_chirho> {
    pub(super) fn new_chirho(checked_chirho: &'checked_chirho CheckedModuleChirho) -> Self {
        Self {
            checked_chirho,
            remaining_chirho: 1 << 20,
        }
    }

    pub(super) fn convert_chirho(
        &mut self,
        ty_chirho: TyChirho,
    ) -> Result<CoreTypeChirho, NextErrorChirho> {
        self.walk_chirho(ty_chirho, 0)
    }

    pub(super) fn quantified_chirho(
        &self,
        name_chirho: NameIdChirho,
    ) -> Result<Vec<TyVarChirho>, NextErrorChirho> {
        let mut variables_chirho = Vec::new();
        if let Some(scheme_chirho) = self.checked_chirho.schemes_chirho.get(&name_chirho) {
            let arena_chirho = &self.checked_chirho.solver_chirho.arena_chirho;
            for ty_chirho in &scheme_chirho.quantified_chirho {
                let TyNodeChirho::MetaChirho(meta_chirho) = arena_chirho.node_chirho(*ty_chirho)?
                else {
                    return Err(NextErrorChirho::new_chirho(
                        NextErrorKindChirho::InvalidChirho,
                        "scheme binder has no type-variable identity",
                    ));
                };
                variables_chirho.push(TyVarChirho(arena_chirho.meta_index_chirho(*meta_chirho)?));
            }
        }
        Ok(variables_chirho)
    }

    pub(super) fn binding_type_chirho(
        &mut self,
        name_chirho: NameIdChirho,
        body_chirho: TyChirho,
    ) -> Result<CoreTypeChirho, NextErrorChirho> {
        let body_chirho = self.convert_chirho(body_chirho)?;
        let variables_chirho = self.quantified_chirho(name_chirho)?;
        Ok(if variables_chirho.is_empty() {
            body_chirho
        } else {
            CoreTypeChirho::ForallChirho {
                vars_chirho: variables_chirho,
                body_chirho: Box::new(body_chirho),
            }
        })
    }

    fn walk_chirho(
        &mut self,
        ty_chirho: TyChirho,
        depth_chirho: usize,
    ) -> Result<CoreTypeChirho, NextErrorChirho> {
        if self.remaining_chirho == 0 || depth_chirho > 256 {
            return Err(NextErrorChirho::new_chirho(
                NextErrorKindChirho::ResourceChirho,
                "next Core type-tree adapter allocation/depth ceiling exhausted",
            ));
        }
        self.remaining_chirho -= 1;
        let arena_chirho = &self.checked_chirho.solver_chirho.arena_chirho;
        let ty_chirho = arena_chirho.resolve_chirho(ty_chirho)?;
        Ok(match arena_chirho.node_chirho(ty_chirho)? {
            TyNodeChirho::ConChirho(con_chirho) => CoreTypeChirho::ConChirho(
                match con_chirho {
                    TyConChirho::BoolChirho => "Bool",
                    TyConChirho::CharChirho => "Char",
                    TyConChirho::ListChirho => "[]",
                    TyConChirho::AnyChirho => "%next:zonked-any:TypeChirho",
                    TyConChirho::TypeChirho => {
                        return Err(NextErrorChirho::new_chirho(
                            NextErrorKindChirho::InvalidChirho,
                            "a kind was used as a runtime value type",
                        ));
                    }
                }
                .into(),
            ),
            TyNodeChirho::MetaChirho(meta_chirho) => CoreTypeChirho::VarChirho(TyVarChirho(
                arena_chirho.meta_index_chirho(*meta_chirho)?,
            )),
            TyNodeChirho::FunChirho(argument_chirho, result_chirho) => CoreTypeChirho::fun_chirho(
                self.walk_chirho(*argument_chirho, depth_chirho + 1)?,
                self.walk_chirho(*result_chirho, depth_chirho + 1)?,
            ),
            TyNodeChirho::AppChirho(function_chirho, argument_chirho) => CoreTypeChirho::AppChirho(
                Box::new(self.walk_chirho(*function_chirho, depth_chirho + 1)?),
                Box::new(self.walk_chirho(*argument_chirho, depth_chirho + 1)?),
            ),
            TyNodeChirho::TupleChirho(items_chirho) => CoreTypeChirho::TupleChirho(
                items_chirho
                    .iter()
                    .map(|item_chirho| self.walk_chirho(*item_chirho, depth_chirho + 1))
                    .collect::<Result<_, _>>()?,
            ),
            TyNodeChirho::ForallChirho { .. } | TyNodeChirho::RigidChirho(_) => {
                return Err(NextErrorChirho::new_chirho(
                    NextErrorKindChirho::UnsupportedChirho,
                    "next S0 Core adapter does not yet elaborate quantified evidence",
                ));
            }
        })
    }
}
