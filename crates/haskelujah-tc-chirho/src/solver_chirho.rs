// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Wanted equality solving with levels and implication ownership.
//! Reference: GHC.Tc.Utils.Unify checkTyEqRhs/touchabilityTest. No legacy calls.
//! workflow: next-pipeline-chirho/check-and-elaborate-chirho

use std::collections::HashSet;

use haskelujah_span_chirho::SpanChirho;

use crate::types_chirho::MetaCellChirho;
use crate::{
    MetaIdChirho, TcErrorChirho, TcErrorKindChirho, TyChirho, TyNodeChirho, TypeArenaChirho,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScopeIdChirho {
    owner_chirho: u64,
    index_chirho: usize,
}

#[derive(Debug, Clone, Copy)]
struct ScopeChirho {
    parent_chirho: Option<ScopeIdChirho>,
    level_chirho: u32,
    given_equalities_chirho: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct EqualityChirho {
    pub left_chirho: TyChirho,
    pub right_chirho: TyChirho,
    pub scope_chirho: ScopeIdChirho,
    pub span_chirho: SpanChirho,
}

#[derive(Debug)]
pub struct SolverChirho {
    pub arena_chirho: TypeArenaChirho,
    scopes_chirho: Vec<ScopeChirho>,
    wanteds_chirho: Vec<EqualityChirho>,
    failure_chirho: Option<TcErrorChirho>,
}

impl SolverChirho {
    pub fn new_chirho(ceiling_chirho: usize) -> Self {
        let mut arena_chirho = TypeArenaChirho::new_chirho(ceiling_chirho);
        let failure_chirho = arena_chirho.reserve_auxiliary_chirho().err();
        Self {
            arena_chirho,
            scopes_chirho: vec![ScopeChirho {
                parent_chirho: None,
                level_chirho: 0,
                given_equalities_chirho: false,
            }],
            wanteds_chirho: Vec::new(),
            failure_chirho,
        }
    }

    pub fn root_chirho(&self) -> ScopeIdChirho {
        ScopeIdChirho {
            owner_chirho: self.arena_chirho.identity_chirho(),
            index_chirho: 0,
        }
    }

    pub fn implication_chirho(
        &mut self,
        parent_chirho: ScopeIdChirho,
        given_equalities_chirho: bool,
    ) -> Result<ScopeIdChirho, TcErrorChirho> {
        if parent_chirho.owner_chirho != self.arena_chirho.identity_chirho() {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "implication parent belongs to another solver",
            ));
        }
        let level_chirho = self
            .scopes_chirho
            .get(parent_chirho.index_chirho)
            .ok_or_else(|| {
                TcErrorChirho::new_chirho(
                    TcErrorKindChirho::InvalidReferenceChirho,
                    "invalid implication parent",
                )
            })?
            .level_chirho
            .checked_add(1)
            .ok_or_else(|| {
                TcErrorChirho::new_chirho(
                    TcErrorKindChirho::ResourceChirho,
                    "implication level overflow",
                )
            })?;
        let scope_chirho = ScopeIdChirho {
            owner_chirho: self.arena_chirho.identity_chirho(),
            index_chirho: self.scopes_chirho.len(),
        };
        self.arena_chirho.reserve_auxiliary_chirho()?;
        self.scopes_chirho.push(ScopeChirho {
            parent_chirho: Some(parent_chirho),
            level_chirho,
            given_equalities_chirho,
        });
        Ok(scope_chirho)
    }

    pub fn want_chirho(&mut self, equality_chirho: EqualityChirho) {
        if self.failure_chirho.is_some() {
            return;
        }
        if let Err(error_chirho) = self.arena_chirho.reserve_auxiliary_chirho() {
            self.failure_chirho = Some(error_chirho);
            return;
        }
        self.wanteds_chirho.push(equality_chirho);
    }

    fn touchable_chirho(&self, level_chirho: u32, mut scope_chirho: ScopeIdChirho) -> bool {
        loop {
            let frame_chirho = self.scopes_chirho[scope_chirho.index_chirho];
            if frame_chirho.level_chirho == level_chirho {
                return true;
            }
            if frame_chirho.level_chirho < level_chirho || frame_chirho.given_equalities_chirho {
                return false;
            }
            let Some(parent_chirho) = frame_chirho.parent_chirho else {
                return false;
            };
            scope_chirho = parent_chirho;
        }
    }

    fn bind_chirho(
        &mut self,
        meta_chirho: MetaIdChirho,
        image_chirho: TyChirho,
        scope_chirho: ScopeIdChirho,
        trail_chirho: &mut Vec<(MetaIdChirho, MetaCellChirho)>,
    ) -> Result<bool, TcErrorChirho> {
        let cell_chirho = self.arena_chirho.meta_cell_chirho(meta_chirho)?;
        if !self.touchable_chirho(cell_chirho.level_chirho, scope_chirho) {
            return Ok(false);
        }
        let mut work_chirho = vec![image_chirho];
        let mut seen_chirho = HashSet::new();
        let mut promote_chirho = Vec::new();
        while let Some(ty_chirho) = work_chirho.pop() {
            let ty_chirho = self.arena_chirho.resolve_chirho(ty_chirho)?;
            if !seen_chirho.insert(ty_chirho) {
                continue;
            }
            match self.arena_chirho.node_chirho(ty_chirho)? {
                TyNodeChirho::MetaChirho(other_chirho) => {
                    if *other_chirho == meta_chirho {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::InfiniteTypeChirho,
                            "occurs check: an infinite type is required",
                        ));
                    }
                    if self
                        .arena_chirho
                        .meta_cell_chirho(*other_chirho)?
                        .level_chirho
                        > cell_chirho.level_chirho
                    {
                        promote_chirho.push(*other_chirho);
                    }
                }
                TyNodeChirho::RigidChirho(rigid_chirho) => {
                    if self
                        .arena_chirho
                        .rigid_cell_chirho(*rigid_chirho)?
                        .level_chirho
                        > cell_chirho.level_chirho
                    {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::SkolemEscapeChirho,
                            "a rigid type would escape its implication",
                        ));
                    }
                }
                TyNodeChirho::ForallChirho { .. } => {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::TypeMismatchChirho,
                        "ordinary metavariables are monotypes",
                    ));
                }
                TyNodeChirho::AppChirho(left_chirho, right_chirho)
                | TyNodeChirho::FunChirho(left_chirho, right_chirho) => {
                    work_chirho.push(*left_chirho);
                    work_chirho.push(*right_chirho);
                }
                TyNodeChirho::TupleChirho(items_chirho) => work_chirho.extend(items_chirho),
                TyNodeChirho::ConChirho(_) => {}
            }
        }
        let actual_kind_chirho = self.arena_chirho.kind_chirho(image_chirho)?;
        if self.arena_chirho.resolve_chirho(cell_chirho.kind_chirho)?
            != self.arena_chirho.resolve_chirho(actual_kind_chirho)?
        {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::KindMismatchChirho,
                "metavariable and image have different checked kinds",
            ));
        }
        for other_chirho in promote_chirho {
            let previous_chirho = self.arena_chirho.meta_cell_chirho(other_chirho)?;
            trail_chirho.push((other_chirho, previous_chirho));
            self.arena_chirho.metas_chirho[other_chirho.index_chirho as usize].level_chirho =
                cell_chirho.level_chirho;
        }
        trail_chirho.push((meta_chirho, cell_chirho));
        self.arena_chirho.metas_chirho[meta_chirho.index_chirho as usize].fill_chirho =
            Some(image_chirho);
        Ok(true)
    }

    fn equation_chirho(&mut self, equality_chirho: EqualityChirho) -> Result<bool, TcErrorChirho> {
        if equality_chirho.scope_chirho.owner_chirho != self.arena_chirho.identity_chirho()
            || self
                .scopes_chirho
                .get(equality_chirho.scope_chirho.index_chirho)
                .is_none()
        {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "invalid equality scope",
            ));
        }
        let mut trail_chirho = Vec::new();
        let result_chirho = (|| {
            let mut work_chirho = vec![(equality_chirho.left_chirho, equality_chirho.right_chirho)];
            let mut seen_chirho = HashSet::new();
            while let Some((left_chirho, right_chirho)) = work_chirho.pop() {
                let left_chirho = self.arena_chirho.resolve_chirho(left_chirho)?;
                let right_chirho = self.arena_chirho.resolve_chirho(right_chirho)?;
                if left_chirho == right_chirho {
                    continue;
                }
                if !seen_chirho.insert((left_chirho, right_chirho)) {
                    continue;
                }
                self.arena_chirho.reserve_auxiliary_chirho()?;
                match (
                    self.arena_chirho.node_chirho(left_chirho)?.clone(),
                    self.arena_chirho.node_chirho(right_chirho)?.clone(),
                ) {
                    (TyNodeChirho::MetaChirho(meta_chirho), _) => {
                        if !self.bind_chirho(
                            meta_chirho,
                            right_chirho,
                            equality_chirho.scope_chirho,
                            &mut trail_chirho,
                        )? {
                            return Ok(false);
                        }
                    }
                    (_, TyNodeChirho::MetaChirho(meta_chirho)) => {
                        if !self.bind_chirho(
                            meta_chirho,
                            left_chirho,
                            equality_chirho.scope_chirho,
                            &mut trail_chirho,
                        )? {
                            return Ok(false);
                        }
                    }
                    (
                        TyNodeChirho::FunChirho(left_arg_chirho, left_res_chirho),
                        TyNodeChirho::FunChirho(right_arg_chirho, right_res_chirho),
                    )
                    | (
                        TyNodeChirho::AppChirho(left_arg_chirho, left_res_chirho),
                        TyNodeChirho::AppChirho(right_arg_chirho, right_res_chirho),
                    ) => {
                        work_chirho.push((left_res_chirho, right_res_chirho));
                        work_chirho.push((left_arg_chirho, right_arg_chirho));
                    }
                    (
                        TyNodeChirho::TupleChirho(left_items_chirho),
                        TyNodeChirho::TupleChirho(right_items_chirho),
                    ) if left_items_chirho.len() == right_items_chirho.len() => work_chirho
                        .extend(left_items_chirho.into_iter().zip(right_items_chirho).rev()),
                    (TyNodeChirho::ForallChirho { .. }, _)
                    | (_, TyNodeChirho::ForallChirho { .. }) => {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::UnsupportedChirho,
                            "next does not yet solve non-reflexive forall equalities",
                        ));
                    }
                    _ => {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::TypeMismatchChirho,
                            "type mismatch in next equality solver",
                        ));
                    }
                }
            }
            Ok(true)
        })();
        if !matches!(result_chirho, Ok(true)) {
            for (meta_chirho, previous_chirho) in trail_chirho.into_iter().rev() {
                self.arena_chirho.metas_chirho[meta_chirho.index_chirho as usize] = previous_chirho;
            }
        }
        result_chirho.map_err(|mut error_chirho| {
            error_chirho.span_chirho = equality_chirho.span_chirho;
            error_chirho
        })
    }

    pub fn solve_chirho(&mut self) -> Result<(), TcErrorChirho> {
        if let Some(error_chirho) = &self.failure_chirho {
            return Err(error_chirho.clone());
        }
        let result_chirho = self.solve_work_chirho();
        if let Err(error_chirho) = &result_chirho {
            self.failure_chirho = Some(error_chirho.clone());
        }
        result_chirho
    }

    fn solve_work_chirho(&mut self) -> Result<(), TcErrorChirho> {
        for _round_chirho in 0..4 {
            let pending_chirho = std::mem::take(&mut self.wanteds_chirho);
            let mut solved_chirho = 0;
            for equality_chirho in pending_chirho {
                if self.equation_chirho(equality_chirho)? {
                    solved_chirho += 1;
                } else {
                    self.wanteds_chirho.push(equality_chirho);
                }
            }
            if self.wanteds_chirho.is_empty() {
                return Ok(());
            }
            if solved_chirho == 0 {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::UntouchableChirho,
                    "wanted equality remains inside an implication; no outer assignment was published",
                ));
            }
        }
        Err(TcErrorChirho::new_chirho(
            TcErrorKindChirho::ResourceChirho,
            "next solver iteration limit exhausted",
        ))
    }
}
