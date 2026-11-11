// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Interned DAGs and creation-time kinds/levels, following GHC.Tc.Utils.TcMType.
//! The module ceiling covers nodes, meta cells and rigid records together.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::{TcErrorChirho, TcErrorKindChirho};

static NEXT_ARENA_CHIRHO: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TyChirho {
    arena_chirho: u64,
    index_chirho: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MetaIdChirho {
    arena_chirho: u64,
    pub(crate) index_chirho: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RigidIdChirho {
    arena_chirho: u64,
    pub(crate) index_chirho: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TyConChirho {
    TypeChirho,
    BoolChirho,
    CharChirho,
    ListChirho,
    /// Internal final-zonking image; source never resolves this constructor by spelling.
    AnyChirho,
    DeclaredChirho(haskelujah_hir_chirho::NameIdChirho),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TyNodeChirho {
    ConChirho(TyConChirho),
    MetaChirho(MetaIdChirho),
    RigidChirho(RigidIdChirho),
    AppChirho(TyChirho, TyChirho),
    FunChirho(TyChirho, TyChirho),
    TupleChirho(Vec<TyChirho>),
    ForallChirho {
        binder_chirho: RigidIdChirho,
        body_chirho: TyChirho,
    },
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MetaCellChirho {
    pub(crate) level_chirho: u32,
    pub(crate) kind_chirho: TyChirho,
    pub(crate) fill_chirho: Option<TyChirho>,
    pub(crate) generalized_chirho: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RigidCellChirho {
    pub(crate) level_chirho: u32,
    pub(crate) kind_chirho: TyChirho,
}

#[derive(Debug)]
pub struct TypeArenaChirho {
    arena_chirho: u64,
    ceiling_chirho: usize,
    auxiliary_chirho: usize,
    nodes_chirho: Vec<TyNodeChirho>,
    index_chirho: HashMap<TyNodeChirho, TyChirho>,
    pub(crate) metas_chirho: Vec<MetaCellChirho>,
    pub(crate) rigids_chirho: Vec<RigidCellChirho>,
    declared_kinds_chirho: HashMap<haskelujah_hir_chirho::NameIdChirho, TyChirho>,
}

impl TypeArenaChirho {
    pub fn new_chirho(ceiling_chirho: usize) -> Self {
        Self {
            arena_chirho: NEXT_ARENA_CHIRHO.fetch_add(1, Ordering::Relaxed),
            ceiling_chirho,
            auxiliary_chirho: 0,
            nodes_chirho: Vec::new(),
            index_chirho: HashMap::new(),
            metas_chirho: Vec::new(),
            rigids_chirho: Vec::new(),
            declared_kinds_chirho: HashMap::new(),
        }
    }

    fn invalid_chirho() -> TcErrorChirho {
        TcErrorChirho::new_chirho(
            TcErrorKindChirho::InvalidReferenceChirho,
            "type reference belongs to another module arena or is invalid",
        )
    }

    fn room_chirho(&self) -> Result<(), TcErrorChirho> {
        if self.allocation_count_chirho() >= self.ceiling_chirho {
            Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::ResourceChirho,
                "next module type allocation ceiling exhausted",
            ))
        } else {
            Ok(())
        }
    }

    pub fn node_chirho(&self, ty_chirho: TyChirho) -> Result<&TyNodeChirho, TcErrorChirho> {
        if ty_chirho.arena_chirho != self.arena_chirho {
            return Err(Self::invalid_chirho());
        }
        self.nodes_chirho
            .get(ty_chirho.index_chirho as usize)
            .ok_or_else(Self::invalid_chirho)
    }

    pub(crate) fn intern_chirho(
        &mut self,
        node_chirho: TyNodeChirho,
    ) -> Result<TyChirho, TcErrorChirho> {
        match &node_chirho {
            TyNodeChirho::AppChirho(left_chirho, right_chirho)
            | TyNodeChirho::FunChirho(left_chirho, right_chirho) => {
                self.node_chirho(*left_chirho)?;
                self.node_chirho(*right_chirho)?;
            }
            TyNodeChirho::TupleChirho(items_chirho) => {
                for ty_chirho in items_chirho {
                    self.node_chirho(*ty_chirho)?;
                }
            }
            TyNodeChirho::ForallChirho {
                binder_chirho,
                body_chirho,
            } => {
                self.rigid_cell_chirho(*binder_chirho)?;
                self.node_chirho(*body_chirho)?;
            }
            TyNodeChirho::MetaChirho(meta_chirho) => {
                self.meta_cell_chirho(*meta_chirho)?;
            }
            TyNodeChirho::RigidChirho(rigid_chirho) => {
                self.rigid_cell_chirho(*rigid_chirho)?;
            }
            TyNodeChirho::ConChirho(_) => {}
        }
        if let Some(ty_chirho) = self.index_chirho.get(&node_chirho) {
            return Ok(*ty_chirho);
        }
        self.room_chirho()?;
        let ty_chirho = TyChirho {
            arena_chirho: self.arena_chirho,
            index_chirho: u32::try_from(self.nodes_chirho.len())
                .map_err(|_| Self::invalid_chirho())?,
        };
        self.nodes_chirho.push(node_chirho.clone());
        self.index_chirho.insert(node_chirho, ty_chirho);
        Ok(ty_chirho)
    }

    pub fn con_chirho(&mut self, con_chirho: TyConChirho) -> Result<TyChirho, TcErrorChirho> {
        self.intern_chirho(TyNodeChirho::ConChirho(con_chirho))
    }

    pub(crate) fn declare_chirho(
        &mut self,
        name_chirho: haskelujah_hir_chirho::NameIdChirho,
        arity_chirho: usize,
    ) -> Result<(), TcErrorChirho> {
        if self.declared_kinds_chirho.contains_key(&name_chirho) {
            return Err(Self::invalid_chirho());
        }
        let type_chirho = self.con_chirho(TyConChirho::TypeChirho)?;
        let mut kind_chirho = type_chirho;
        for _parameter_chirho in 0..arity_chirho {
            kind_chirho = self.intern_chirho(TyNodeChirho::FunChirho(type_chirho, kind_chirho))?;
        }
        self.reserve_auxiliary_chirho()?;
        self.declared_kinds_chirho.insert(name_chirho, kind_chirho);
        Ok(())
    }

    pub fn fresh_meta_chirho(
        &mut self,
        level_chirho: u32,
        kind_chirho: TyChirho,
    ) -> Result<TyChirho, TcErrorChirho> {
        self.node_chirho(kind_chirho)?;
        self.room_chirho()?;
        let meta_chirho = MetaIdChirho {
            arena_chirho: self.arena_chirho,
            index_chirho: u32::try_from(self.metas_chirho.len())
                .map_err(|_| Self::invalid_chirho())?,
        };
        self.metas_chirho.push(MetaCellChirho {
            level_chirho,
            kind_chirho,
            fill_chirho: None,
            generalized_chirho: false,
        });
        self.intern_chirho(TyNodeChirho::MetaChirho(meta_chirho))
    }

    pub fn fresh_rigid_chirho(
        &mut self,
        level_chirho: u32,
        kind_chirho: TyChirho,
    ) -> Result<(RigidIdChirho, TyChirho), TcErrorChirho> {
        self.node_chirho(kind_chirho)?;
        self.room_chirho()?;
        let rigid_chirho = RigidIdChirho {
            arena_chirho: self.arena_chirho,
            index_chirho: u32::try_from(self.rigids_chirho.len())
                .map_err(|_| Self::invalid_chirho())?,
        };
        self.rigids_chirho.push(RigidCellChirho {
            level_chirho,
            kind_chirho,
        });
        Ok((
            rigid_chirho,
            self.intern_chirho(TyNodeChirho::RigidChirho(rigid_chirho))?,
        ))
    }

    pub(crate) fn meta_cell_chirho(
        &self,
        meta_chirho: MetaIdChirho,
    ) -> Result<MetaCellChirho, TcErrorChirho> {
        if meta_chirho.arena_chirho != self.arena_chirho {
            return Err(Self::invalid_chirho());
        }
        self.metas_chirho
            .get(meta_chirho.index_chirho as usize)
            .copied()
            .ok_or_else(Self::invalid_chirho)
    }

    pub(crate) fn rigid_cell_chirho(
        &self,
        rigid_chirho: RigidIdChirho,
    ) -> Result<RigidCellChirho, TcErrorChirho> {
        if rigid_chirho.arena_chirho != self.arena_chirho {
            return Err(Self::invalid_chirho());
        }
        self.rigids_chirho
            .get(rigid_chirho.index_chirho as usize)
            .copied()
            .ok_or_else(Self::invalid_chirho)
    }

    pub fn resolve_chirho(&self, mut ty_chirho: TyChirho) -> Result<TyChirho, TcErrorChirho> {
        for _step_chirho in 0..=self.metas_chirho.len() {
            let TyNodeChirho::MetaChirho(meta_chirho) = self.node_chirho(ty_chirho)? else {
                return Ok(ty_chirho);
            };
            let Some(next_chirho) = self.meta_cell_chirho(*meta_chirho)?.fill_chirho else {
                return Ok(ty_chirho);
            };
            ty_chirho = next_chirho;
        }
        Err(Self::invalid_chirho())
    }

    pub fn kind_chirho(&mut self, ty_chirho: TyChirho) -> Result<TyChirho, TcErrorChirho> {
        let resolved_chirho = self.resolve_chirho(ty_chirho)?;
        match self.node_chirho(resolved_chirho)?.clone() {
            TyNodeChirho::ConChirho(TyConChirho::DeclaredChirho(name_chirho)) => self
                .declared_kinds_chirho
                .get(&name_chirho)
                .copied()
                .ok_or_else(Self::invalid_chirho),
            TyNodeChirho::MetaChirho(meta_chirho) => {
                Ok(self.meta_cell_chirho(meta_chirho)?.kind_chirho)
            }
            TyNodeChirho::RigidChirho(rigid_chirho) => {
                Ok(self.rigid_cell_chirho(rigid_chirho)?.kind_chirho)
            }
            TyNodeChirho::ConChirho(TyConChirho::ListChirho) => {
                let type_chirho = self.con_chirho(TyConChirho::TypeChirho)?;
                self.intern_chirho(TyNodeChirho::FunChirho(type_chirho, type_chirho))
            }
            TyNodeChirho::AppChirho(_, _) => {
                // Walk a wide constructor spine iteratively, instead of recursing per parameter.
                let mut head_chirho = resolved_chirho;
                let mut arguments_chirho = Vec::new();
                while let TyNodeChirho::AppChirho(function_chirho, argument_chirho) =
                    self.node_chirho(head_chirho)?
                {
                    arguments_chirho.push(*argument_chirho);
                    head_chirho = self.resolve_chirho(*function_chirho)?;
                }
                let mut kind_chirho = self.kind_chirho(head_chirho)?;
                for argument_chirho in arguments_chirho.into_iter().rev() {
                    let TyNodeChirho::FunChirho(expected_chirho, result_chirho) =
                        self.node_chirho(self.resolve_chirho(kind_chirho)?)?.clone()
                    else {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::KindMismatchChirho,
                            "type application head has no function kind",
                        ));
                    };
                    let actual_chirho = self.kind_chirho(argument_chirho)?;
                    if self.resolve_chirho(expected_chirho)?
                        != self.resolve_chirho(actual_chirho)?
                    {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::KindMismatchChirho,
                            "type argument has the wrong kind",
                        ));
                    }
                    kind_chirho = result_chirho;
                }
                Ok(kind_chirho)
            }
            _ => self.con_chirho(TyConChirho::TypeChirho),
        }
    }

    pub fn function_chirho(
        &mut self,
        argument_chirho: TyChirho,
        result_chirho: TyChirho,
    ) -> Result<TyChirho, TcErrorChirho> {
        let type_chirho = self.con_chirho(TyConChirho::TypeChirho)?;
        if self.kind_chirho(argument_chirho)? != type_chirho
            || self.kind_chirho(result_chirho)? != type_chirho
        {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::KindMismatchChirho,
                "function arguments and results must have kind Type",
            ));
        }
        self.intern_chirho(TyNodeChirho::FunChirho(argument_chirho, result_chirho))
    }

    pub fn list_chirho(&mut self, item_chirho: TyChirho) -> Result<TyChirho, TcErrorChirho> {
        let list_chirho = self.con_chirho(TyConChirho::ListChirho)?;
        let result_chirho =
            self.intern_chirho(TyNodeChirho::AppChirho(list_chirho, item_chirho))?;
        self.kind_chirho(result_chirho)?;
        Ok(result_chirho)
    }

    pub(crate) fn identity_chirho(&self) -> u64 {
        self.arena_chirho
    }

    pub fn allocation_count_chirho(&self) -> usize {
        self.nodes_chirho.len()
            + self.metas_chirho.len()
            + self.rigids_chirho.len()
            + self.auxiliary_chirho
    }

    /// Constraint and implication records share the module's aggregate allocation ceiling.
    pub(crate) fn reserve_auxiliary_chirho(&mut self) -> Result<(), TcErrorChirho> {
        self.room_chirho()?;
        self.auxiliary_chirho += 1;
        Ok(())
    }
    pub fn meta_index_chirho(&self, meta_chirho: MetaIdChirho) -> Result<u32, TcErrorChirho> {
        self.meta_cell_chirho(meta_chirho)?;
        Ok(meta_chirho.index_chirho)
    }
}
