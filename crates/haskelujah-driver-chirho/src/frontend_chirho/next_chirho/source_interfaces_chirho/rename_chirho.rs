// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_core_chirho::{BinderChirho, CoreExprChirho};
use haskelujah_typing_chirho::ty_chirho::TyChirho as CoreTypeChirho;
use std::collections::HashMap;

pub(super) struct RenamerChirho<'budget_chirho> {
    ids_chirho: HashMap<CoreIdChirho, CoreIdChirho>,
    types_chirho: HashMap<u32, u32>,
    next_type_chirho: &'budget_chirho mut u32,
    left_chirho: &'budget_chirho mut usize,
}
impl<'budget_chirho> RenamerChirho<'budget_chirho> {
    pub(super) fn new_chirho(
        ids_chirho: HashMap<CoreIdChirho, CoreIdChirho>,
        next_type_chirho: &'budget_chirho mut u32,
        left_chirho: &'budget_chirho mut usize,
    ) -> Self {
        Self {
            ids_chirho,
            types_chirho: HashMap::new(),
            next_type_chirho,
            left_chirho,
        }
    }
    fn charge_chirho(
        &mut self,
        count_chirho: usize,
        depth_chirho: usize,
    ) -> Result<(), NextErrorChirho> {
        if depth_chirho > 256 {
            return Err(resource_chirho("linked Core depth ceiling exhausted"));
        }
        *self.left_chirho = self
            .left_chirho
            .checked_sub(count_chirho)
            .ok_or_else(|| resource_chirho("linked Core allocation ceiling exhausted"))?;
        Ok(())
    }
    fn type_id_chirho(&mut self, old_chirho: u32) -> Result<u32, NextErrorChirho> {
        if let Some(new_chirho) = self.types_chirho.get(&old_chirho) {
            return Ok(*new_chirho);
        }
        self.charge_chirho(1, 0)?;
        let new_chirho = *self.next_type_chirho;
        *self.next_type_chirho = self
            .next_type_chirho
            .checked_add(1)
            .ok_or_else(|| resource_chirho("linked Core type id overflow"))?;
        self.types_chirho.insert(old_chirho, new_chirho);
        Ok(new_chirho)
    }
    fn ty_chirho(
        &mut self,
        ty_chirho: &mut CoreTypeChirho,
        depth_chirho: usize,
    ) -> Result<(), NextErrorChirho> {
        self.charge_chirho(1, depth_chirho)?;
        match ty_chirho {
            CoreTypeChirho::VarChirho(variable_chirho) => {
                variable_chirho.0 = self.type_id_chirho(variable_chirho.0)?
            }
            CoreTypeChirho::ConChirho(name_chirho) => {
                self.charge_chirho(name_chirho.len(), depth_chirho)?
            }
            CoreTypeChirho::FunChirho(left_chirho, right_chirho, _)
            | CoreTypeChirho::AppChirho(left_chirho, right_chirho) => {
                self.ty_chirho(left_chirho, depth_chirho + 1)?;
                self.ty_chirho(right_chirho, depth_chirho + 1)?;
            }
            CoreTypeChirho::TupleChirho(items_chirho) => {
                for item_chirho in items_chirho {
                    self.ty_chirho(item_chirho, depth_chirho + 1)?;
                }
            }
            CoreTypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                for variable_chirho in vars_chirho {
                    variable_chirho.0 = self.type_id_chirho(variable_chirho.0)?;
                }
                self.ty_chirho(body_chirho, depth_chirho + 1)?;
            }
            _ => return Err(invalid_chirho("unproduced type form during source linking")),
        }
        Ok(())
    }
    pub(super) fn binder_chirho(
        &mut self,
        binder_chirho: &mut BinderChirho,
    ) -> Result<(), NextErrorChirho> {
        binder_chirho.id_chirho = *self
            .ids_chirho
            .get(&binder_chirho.id_chirho)
            .ok_or_else(|| invalid_chirho("Core binder has no linking allocation"))?;
        self.ty_chirho(&mut binder_chirho.ty_chirho, 0)
    }
    pub(super) fn expression_chirho(
        &mut self,
        expression_chirho: &mut CoreExprChirho,
        depth_chirho: usize,
    ) -> Result<(), NextErrorChirho> {
        self.charge_chirho(1, depth_chirho)?;
        match expression_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                *id_chirho = *self
                    .ids_chirho
                    .get(id_chirho)
                    .ok_or_else(|| invalid_chirho("unresolved original-name Core link"))?
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                self.expression_chirho(fun_chirho, depth_chirho + 1)?;
                self.expression_chirho(arg_chirho, depth_chirho + 1)?;
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                self.binder_chirho(binder_chirho)?;
                self.expression_chirho(body_chirho, depth_chirho + 1)?;
            }
            CoreExprChirho::TyLamChirho {
                ty_var_chirho,
                body_chirho,
            } => {
                let old_chirho = ty_var_chirho
                    .strip_prefix("next_type_")
                    .and_then(|name_chirho| name_chirho.strip_suffix("_chirho"))
                    .and_then(|name_chirho| name_chirho.parse::<u32>().ok())
                    .ok_or_else(|| {
                        invalid_chirho("type abstraction has no producing variable identity")
                    })?;
                *ty_var_chirho = format!("next_type_{}_chirho", self.type_id_chirho(old_chirho)?);
                self.expression_chirho(body_chirho, depth_chirho + 1)?;
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho,
                ty_chirho,
            } => {
                self.ty_chirho(ty_chirho, 0)?;
                self.expression_chirho(expr_chirho, depth_chirho + 1)?;
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    self.binder_chirho(binder_chirho)?;
                    self.expression_chirho(rhs_chirho, depth_chirho + 1)?;
                }
                self.expression_chirho(body_chirho, depth_chirho + 1)?;
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                result_ty_chirho,
                alts_chirho,
            } => {
                self.binder_chirho(bind_chirho)?;
                self.ty_chirho(result_ty_chirho, 0)?;
                self.expression_chirho(scrutinee_chirho, depth_chirho + 1)?;
                for alt_chirho in alts_chirho {
                    for binder_chirho in &mut alt_chirho.binders_chirho {
                        self.binder_chirho(binder_chirho)?;
                    }
                    self.expression_chirho(&mut alt_chirho.rhs_chirho, depth_chirho + 1)?;
                }
            }
            CoreExprChirho::PrimOpChirho {
                args_chirho,
                name_chirho,
            }
            | CoreExprChirho::ConAppChirho {
                args_chirho,
                con_name_chirho: name_chirho,
            } => {
                self.charge_chirho(name_chirho.len(), depth_chirho)?;
                for arg_chirho in args_chirho {
                    self.expression_chirho(arg_chirho, depth_chirho + 1)?;
                }
            }
        }
        Ok(())
    }
}
