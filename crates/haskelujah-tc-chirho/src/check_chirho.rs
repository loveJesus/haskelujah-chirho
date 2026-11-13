// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Generate wanted equalities, solve, then publish one checked module.
//! workflow: next-pipeline-chirho/check-and-elaborate-chirho

use std::collections::HashMap;

use haskelujah_hir_chirho::{
    BindingChirho, ExprChirho, ExprIdChirho, ExprKindChirho, ModuleChirho, NameIdChirho,
    SurfaceTypeChirho, WiredNameChirho, binding_groups_chirho,
};
use haskelujah_span_chirho::SpanChirho;

use crate::{
    EqualityChirho, EvVarChirho, SchemeChirho, ScopeIdChirho, SolverChirho, TcErrorChirho,
    TcErrorKindChirho, TyChirho, TyConChirho, TyNodeChirho,
};

#[path = "class_chirho/classes_chirho.rs"]
mod classes_chirho;
#[path = "data_chirho.rs"]
mod data_chirho;
#[path = "generalization_chirho.rs"]
mod generalization_chirho;
#[path = "signatures_chirho.rs"]
mod signatures_chirho;

#[derive(Debug, Clone, Copy)]
pub enum WrapStepChirho {
    TypeChirho(TyChirho),
    EvidenceChirho(EvVarChirho),
}

#[derive(Debug, Clone)]
pub struct AbstractionChirho {
    pub type_binders_chirho: Vec<TyChirho>,
    pub dictionary_binders_chirho: Vec<EvVarChirho>,
    pub scope_chirho: ScopeIdChirho,
}

#[derive(Debug)]
pub struct CheckedModuleChirho {
    pub module_chirho: ModuleChirho,
    pub solver_chirho: SolverChirho,
    pub expressions_chirho: HashMap<ExprIdChirho, TyChirho>,
    pub names_chirho: HashMap<NameIdChirho, TyChirho>,
    pub schemes_chirho: HashMap<NameIdChirho, SchemeChirho>,
    pub wrappers_chirho: HashMap<ExprIdChirho, Vec<WrapStepChirho>>,
    pub abstractions_chirho: HashMap<NameIdChirho, AbstractionChirho>,
    pub patterns_chirho: HashMap<ExprIdChirho, TyChirho>,
    pub constructors_chirho: HashMap<NameIdChirho, Vec<TyChirho>>,
    references_chirho: HashMap<ExprIdChirho, NameIdChirho>,
    scope_chirho: ScopeIdChirho,
}

impl CheckedModuleChirho {
    fn fresh_chirho(&mut self) -> Result<TyChirho, TcErrorChirho> {
        let kind_chirho = self
            .solver_chirho
            .arena_chirho
            .con_chirho(TyConChirho::TypeChirho)?;
        let level_chirho = self.solver_chirho.level_chirho(self.scope_chirho)?;
        self.solver_chirho
            .arena_chirho
            .fresh_meta_chirho(level_chirho, kind_chirho)
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
            scope_chirho: self.scope_chirho,
            span_chirho,
        });
    }

    fn surface_chirho(
        &mut self,
        surface_chirho: &SurfaceTypeChirho,
    ) -> Result<TyChirho, TcErrorChirho> {
        self.surface_with_chirho(surface_chirho, &HashMap::new())
    }

    fn bindings_chirho(&mut self, bindings_chirho: &[BindingChirho]) -> Result<(), TcErrorChirho> {
        // Complete signatures are available before dependency inference (Haskell 2010 4.5.1).
        for binding_chirho in bindings_chirho {
            if let Some(surface_chirho) = &binding_chirho.signature_chirho {
                self.signature_chirho(binding_chirho.name_chirho, surface_chirho)?;
            }
        }
        for group_chirho in binding_groups_chirho(bindings_chirho) {
            let parent_chirho = self.scope_chirho;
            let outer_level_chirho = self.solver_chirho.level_chirho(parent_chirho)?;
            self.scope_chirho = self
                .solver_chirho
                .implication_chirho(parent_chirho, false)?;
            let result_chirho =
                self.group_chirho(bindings_chirho, &group_chirho, outer_level_chirho);
            self.scope_chirho = parent_chirho;
            result_chirho?;
        }
        Ok(())
    }

    // workflow: next-pipeline-chirho/ghc-implementation-map-chirho
    fn group_chirho(
        &mut self,
        bindings_chirho: &[BindingChirho],
        group_chirho: &[usize],
        outer_level_chirho: u32,
    ) -> Result<(), TcErrorChirho> {
        for &index_chirho in group_chirho {
            let binding_chirho = &bindings_chirho[index_chirho];
            if binding_chirho.signature_chirho.is_none() {
                let ty_chirho = self.fresh_chirho()?;
                self.names_chirho
                    .insert(binding_chirho.name_chirho, ty_chirho);
            }
        }
        for &index_chirho in group_chirho {
            let binding_chirho = &bindings_chirho[index_chirho];
            if binding_chirho.signature_chirho.is_some() {
                self.check_signature_chirho(binding_chirho)?;
            } else {
                let ty_chirho = self.infer_chirho(&binding_chirho.body_chirho)?;
                self.want_chirho(
                    ty_chirho,
                    self.names_chirho[&binding_chirho.name_chirho],
                    binding_chirho.span_chirho,
                );
            }
        }
        self.close_group_chirho(bindings_chirho, group_chirho, outer_level_chirho)
    }

    fn infer_chirho(&mut self, expression_chirho: &ExprChirho) -> Result<TyChirho, TcErrorChirho> {
        let ty_chirho = match &expression_chirho.kind_chirho {
            ExprKindChirho::VarChirho(name_chirho) => {
                self.references_chirho
                    .insert(expression_chirho.id_chirho, *name_chirho);
                if let Some(scheme_chirho) = self.schemes_chirho.get(name_chirho) {
                    let scheme_chirho = scheme_chirho.clone();
                    let level_chirho = self.solver_chirho.level_chirho(self.scope_chirho)?;
                    let (ty_chirho, arguments_chirho, context_chirho) = self
                        .solver_chirho
                        .arena_chirho
                        .instantiate_chirho(&scheme_chirho, level_chirho)?;
                    let mut steps_chirho = Vec::new();
                    for argument_chirho in arguments_chirho {
                        self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                        steps_chirho.push(WrapStepChirho::TypeChirho(argument_chirho));
                    }
                    for predicate_chirho in context_chirho {
                        let evidence_chirho = self.solver_chirho.want_dictionary_chirho(
                            self.scope_chirho,
                            predicate_chirho,
                            expression_chirho.span_chirho,
                        )?;
                        self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                        steps_chirho.push(WrapStepChirho::EvidenceChirho(evidence_chirho));
                    }
                    self.wrappers_chirho
                        .insert(expression_chirho.id_chirho, steps_chirho);
                    ty_chirho
                } else {
                    *self.names_chirho.get(name_chirho).ok_or_else(|| {
                        TcErrorChirho::new_chirho(
                            TcErrorKindChirho::InvalidReferenceChirho,
                            "HIR reference has no checked binding",
                        )
                    })?
                }
            }
            ExprKindChirho::IntegerChirho(_) => self
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::IntegerChirho)?,
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
            ExprKindChirho::MatchChirho(inputs_chirho, arms_chirho) => {
                self.match_chirho(inputs_chirho, arms_chirho)?
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
    let solver_chirho = SolverChirho::new_chirho(ceiling_chirho);
    let scope_chirho = solver_chirho.root_chirho();
    let mut checked_chirho = CheckedModuleChirho {
        module_chirho,
        solver_chirho,
        expressions_chirho: HashMap::new(),
        names_chirho: HashMap::new(),
        schemes_chirho: HashMap::new(),
        wrappers_chirho: HashMap::new(),
        abstractions_chirho: HashMap::new(),
        patterns_chirho: HashMap::new(),
        constructors_chirho: HashMap::new(),
        references_chirho: HashMap::new(),
        scope_chirho,
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
            let id_chirho = NameIdChirho(index_chirho as u32);
            checked_chirho
                .constructors_chirho
                .insert(id_chirho, Vec::new());
            checked_chirho.schemes_chirho.insert(
                id_chirho,
                SchemeChirho {
                    context_chirho: Vec::new(),
                    body_chirho: bool_chirho,
                    quantified_chirho: Vec::new(),
                },
            );
        }
    }
    checked_chirho.import_interfaces_chirho()?;
    let data_chirho = std::mem::take(&mut checked_chirho.module_chirho.data_chirho);
    let classes_chirho = std::mem::take(&mut checked_chirho.module_chirho.classes_chirho);
    // Class kinds must exist while checking data fields that mention those declarations.
    checked_chirho.class_headers_chirho(&classes_chirho)?;
    checked_chirho.data_chirho(&data_chirho)?;
    checked_chirho.module_chirho.data_chirho = data_chirho;
    checked_chirho.classes_chirho(&classes_chirho)?;
    checked_chirho.module_chirho.classes_chirho = classes_chirho;
    let instances_chirho = std::mem::take(&mut checked_chirho.module_chirho.instances_chirho);
    checked_chirho.instances_chirho(&instances_chirho)?;
    let bindings_chirho = std::mem::take(&mut checked_chirho.module_chirho.bindings_chirho);
    checked_chirho.bindings_chirho(&bindings_chirho)?;
    checked_chirho.module_chirho.bindings_chirho = bindings_chirho;
    checked_chirho.instance_methods_chirho(&instances_chirho)?;
    checked_chirho.module_chirho.instances_chirho = instances_chirho;
    checked_chirho.solver_chirho.solve_chirho()?;
    checked_chirho.finish_dictionaries_chirho()?;
    // Monomorphic recursive occurrences re-apply their group's own type and dictionary binders.
    for (expression_chirho, name_chirho) in &checked_chirho.references_chirho {
        if !checked_chirho
            .wrappers_chirho
            .contains_key(expression_chirho)
            && let Some(abstraction_chirho) = checked_chirho.abstractions_chirho.get(name_chirho)
        {
            let mut steps_chirho = Vec::new();
            for variable_chirho in &abstraction_chirho.type_binders_chirho {
                checked_chirho
                    .solver_chirho
                    .arena_chirho
                    .reserve_auxiliary_chirho()?;
                steps_chirho.push(WrapStepChirho::TypeChirho(*variable_chirho));
            }
            for variable_chirho in &abstraction_chirho.dictionary_binders_chirho {
                checked_chirho
                    .solver_chirho
                    .arena_chirho
                    .reserve_auxiliary_chirho()?;
                steps_chirho.push(WrapStepChirho::EvidenceChirho(*variable_chirho));
            }
            checked_chirho
                .wrappers_chirho
                .insert(*expression_chirho, steps_chirho);
        }
    }
    checked_chirho
        .solver_chirho
        .arena_chirho
        .finish_metas_chirho()?;
    Ok(checked_chirho)
}
