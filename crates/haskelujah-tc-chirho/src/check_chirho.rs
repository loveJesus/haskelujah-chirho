// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Generate wanted equalities, solve, then publish one checked module.
//! workflow: next-pipeline-chirho/check-and-elaborate-chirho

use std::collections::HashMap;

use haskelujah_hir_chirho::{
    BindingChirho, ExprChirho, ExprIdChirho, ExprKindChirho, ModuleChirho, NameIdChirho,
    SurfaceTypeChirho, WiredNameChirho,
};
use haskelujah_span_chirho::SpanChirho;

use crate::{
    EqualityChirho, SolverChirho, TcErrorChirho, TcErrorKindChirho, TyChirho, TyConChirho,
    TyNodeChirho,
};

#[derive(Debug)]
pub struct CheckedModuleChirho {
    pub module_chirho: ModuleChirho,
    pub solver_chirho: SolverChirho,
    pub expressions_chirho: HashMap<ExprIdChirho, TyChirho>,
    pub names_chirho: HashMap<NameIdChirho, TyChirho>,
}

impl CheckedModuleChirho {
    fn fresh_chirho(&mut self) -> Result<TyChirho, TcErrorChirho> {
        let kind_chirho = self
            .solver_chirho
            .arena_chirho
            .con_chirho(TyConChirho::TypeChirho)?;
        self.solver_chirho
            .arena_chirho
            .fresh_meta_chirho(0, kind_chirho)
    }

    fn want_chirho(
        &mut self,
        left_chirho: TyChirho,
        right_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) {
        self.solver_chirho.want_chirho(EqualityChirho {
            left_chirho,
            right_chirho,
            scope_chirho: self.solver_chirho.root_chirho(),
            span_chirho,
        });
    }

    fn surface_chirho(
        &mut self,
        surface_chirho: &SurfaceTypeChirho,
    ) -> Result<TyChirho, TcErrorChirho> {
        match surface_chirho {
            SurfaceTypeChirho::BoolChirho => self
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::BoolChirho),
            SurfaceTypeChirho::CharChirho => self
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::CharChirho),
            SurfaceTypeChirho::FunChirho(argument_chirho, result_chirho) => {
                let argument_chirho = self.surface_chirho(argument_chirho)?;
                let result_chirho = self.surface_chirho(result_chirho)?;
                self.solver_chirho
                    .arena_chirho
                    .function_chirho(argument_chirho, result_chirho)
            }
            SurfaceTypeChirho::ListChirho(item_chirho) => {
                let item_chirho = self.surface_chirho(item_chirho)?;
                self.solver_chirho.arena_chirho.list_chirho(item_chirho)
            }
            SurfaceTypeChirho::TupleChirho(items_chirho) => {
                let items_chirho = items_chirho
                    .iter()
                    .map(|item_chirho| self.surface_chirho(item_chirho))
                    .collect::<Result<_, _>>()?;
                self.solver_chirho
                    .arena_chirho
                    .intern_chirho(TyNodeChirho::TupleChirho(items_chirho))
            }
        }
    }

    fn prepare_bindings_chirho(
        &mut self,
        bindings_chirho: &[BindingChirho],
    ) -> Result<(), TcErrorChirho> {
        for binding_chirho in bindings_chirho {
            let ty_chirho = match &binding_chirho.signature_chirho {
                Some(surface_chirho) => self.surface_chirho(surface_chirho)?,
                None => self.fresh_chirho()?,
            };
            self.names_chirho
                .insert(binding_chirho.name_chirho, ty_chirho);
        }
        Ok(())
    }

    fn bindings_chirho(&mut self, bindings_chirho: &[BindingChirho]) -> Result<(), TcErrorChirho> {
        self.prepare_bindings_chirho(bindings_chirho)?;
        for binding_chirho in bindings_chirho {
            let ty_chirho = self.infer_chirho(&binding_chirho.body_chirho)?;
            self.want_chirho(
                ty_chirho,
                self.names_chirho[&binding_chirho.name_chirho],
                binding_chirho.span_chirho,
            );
        }
        Ok(())
    }

    fn infer_chirho(&mut self, expression_chirho: &ExprChirho) -> Result<TyChirho, TcErrorChirho> {
        let ty_chirho = match &expression_chirho.kind_chirho {
            ExprKindChirho::VarChirho(name_chirho) => {
                *self.names_chirho.get(name_chirho).ok_or_else(|| {
                    TcErrorChirho::new_chirho(
                        TcErrorKindChirho::InvalidReferenceChirho,
                        "HIR reference has no checked binding",
                    )
                })?
            }
            ExprKindChirho::CharChirho(_) => self
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::CharChirho)?,
            ExprKindChirho::StringChirho(_) => {
                let char_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .con_chirho(TyConChirho::CharChirho)?;
                self.solver_chirho.arena_chirho.list_chirho(char_chirho)?
            }
            ExprKindChirho::AppChirho(function_chirho, argument_chirho) => {
                let function_chirho = self.infer_chirho(function_chirho)?;
                let argument_chirho = self.infer_chirho(argument_chirho)?;
                let result_chirho = self.fresh_chirho()?;
                let expected_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .function_chirho(argument_chirho, result_chirho)?;
                self.want_chirho(
                    function_chirho,
                    expected_chirho,
                    expression_chirho.span_chirho,
                );
                result_chirho
            }
            ExprKindChirho::LamChirho(parameters_chirho, body_chirho) => {
                let mut arguments_chirho = Vec::new();
                for name_chirho in parameters_chirho {
                    let ty_chirho = self.fresh_chirho()?;
                    self.names_chirho.insert(*name_chirho, ty_chirho);
                    arguments_chirho.push(ty_chirho);
                }
                let mut result_chirho = self.infer_chirho(body_chirho)?;
                for argument_chirho in arguments_chirho.into_iter().rev() {
                    result_chirho = self
                        .solver_chirho
                        .arena_chirho
                        .function_chirho(argument_chirho, result_chirho)?;
                }
                result_chirho
            }
            ExprKindChirho::LetChirho(bindings_chirho, body_chirho) => {
                self.bindings_chirho(bindings_chirho)?;
                self.infer_chirho(body_chirho)?
            }
            ExprKindChirho::IfChirho(condition_chirho, then_chirho, else_chirho) => {
                let condition_chirho = self.infer_chirho(condition_chirho)?;
                let bool_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .con_chirho(TyConChirho::BoolChirho)?;
                self.want_chirho(condition_chirho, bool_chirho, expression_chirho.span_chirho);
                let then_chirho = self.infer_chirho(then_chirho)?;
                let else_chirho = self.infer_chirho(else_chirho)?;
                self.want_chirho(then_chirho, else_chirho, expression_chirho.span_chirho);
                then_chirho
            }
            ExprKindChirho::AnnChirho(inner_chirho, surface_chirho) => {
                let inferred_chirho = self.infer_chirho(inner_chirho)?;
                let expected_chirho = self.surface_chirho(surface_chirho)?;
                self.want_chirho(
                    inferred_chirho,
                    expected_chirho,
                    expression_chirho.span_chirho,
                );
                expected_chirho
            }
            ExprKindChirho::ListChirho(items_chirho) => {
                let item_ty_chirho = self.fresh_chirho()?;
                for item_chirho in items_chirho {
                    let actual_chirho = self.infer_chirho(item_chirho)?;
                    self.want_chirho(actual_chirho, item_ty_chirho, item_chirho.span_chirho);
                }
                self.solver_chirho
                    .arena_chirho
                    .list_chirho(item_ty_chirho)?
            }
            ExprKindChirho::TupleChirho(items_chirho) => {
                let items_chirho = items_chirho
                    .iter()
                    .map(|item_chirho| self.infer_chirho(item_chirho))
                    .collect::<Result<_, _>>()?;
                self.solver_chirho
                    .arena_chirho
                    .intern_chirho(TyNodeChirho::TupleChirho(items_chirho))?
            }
        };
        self.expressions_chirho
            .insert(expression_chirho.id_chirho, ty_chirho);
        Ok(ty_chirho)
    }
}

pub fn check_module_chirho(
    module_chirho: ModuleChirho,
    ceiling_chirho: usize,
) -> Result<CheckedModuleChirho, TcErrorChirho> {
    let mut checked_chirho = CheckedModuleChirho {
        module_chirho,
        solver_chirho: SolverChirho::new_chirho(ceiling_chirho),
        expressions_chirho: HashMap::new(),
        names_chirho: HashMap::new(),
    };
    for (index_chirho, name_chirho) in checked_chirho.module_chirho.names_chirho.iter().enumerate()
    {
        if matches!(
            name_chirho.wired_chirho,
            Some(WiredNameChirho::TrueChirho | WiredNameChirho::FalseChirho)
        ) {
            let bool_chirho = checked_chirho
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::BoolChirho)?;
            checked_chirho
                .names_chirho
                .insert(NameIdChirho(index_chirho as u32), bool_chirho);
        }
    }
    let bindings_chirho = std::mem::take(&mut checked_chirho.module_chirho.bindings_chirho);
    checked_chirho.bindings_chirho(&bindings_chirho)?;
    checked_chirho.module_chirho.bindings_chirho = bindings_chirho;
    checked_chirho.solver_chirho.solve_chirho()?;
    Ok(checked_chirho)
}
