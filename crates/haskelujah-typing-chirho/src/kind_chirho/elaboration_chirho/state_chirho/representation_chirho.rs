// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Runtime-representation variables use the existing owner, supply, binding
//! records and allowance. Closed compiler keys never come from source spelling.

use super::super::TypeElaboratorChirho;
use super::*;
use crate::infer_chirho::TypeFamilyEnvChirho;
use haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho;

impl ElaborationStateChirho {
    pub(in crate::kind_chirho::elaboration_chirho) fn queue_constructor_field_kinds_chirho(
        &mut self,
        fields_chirho: Vec<(TyChirho, SpanChirho)>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.charge_chirho(fields_chirho.len(), fields_chirho.len(), span_chirho)?;
        self.pending_constructor_fields_chirho.extend(fields_chirho);
        Ok(())
    }

    /// Local family branches join the checked catalogue after elaboration.
    /// Discharge the producer's fields at that existing inference handoff,
    /// without admitting GADT givens as a fixed-layout proof for data fields.
    pub(crate) fn finish_constructor_field_kinds_chirho(
        &mut self,
        catalogue_chirho: &TypeFamilyEnvChirho,
        view_chirho: crate::infer_chirho::EqualityNormalizationViewChirho<'_>,
    ) -> Result<(), ElaborationErrorChirho> {
        let fields_chirho = std::mem::take(&mut self.pending_constructor_fields_chirho);
        self.charge_chirho(0, 0, SpanChirho::DUMMY_CHIRHO)?;
        if fields_chirho.is_empty() {
            return Ok(());
        }
        let mut roots_chirho = Vec::new();
        for (kind_chirho, span_chirho) in &fields_chirho {
            let variables_chirho = kind_chirho.free_vars_chirho();
            self.charge_chirho(variables_chirho.len(), variables_chirho.len(), *span_chirho)?;
            roots_chirho.extend(variables_chirho);
        }
        let span_chirho = fields_chirho
            .first()
            .map_or(SpanChirho::DUMMY_CHIRHO, |(_, span_chirho)| *span_chirho);
        let candidates_chirho = self.kind_dependency_closure_chirho(&roots_chirho, span_chirho)?;
        self.default_representation_variables_chirho(
            &candidates_chirho,
            &HashSet::new(),
            span_chirho,
        )?;
        for (kind_chirho, span_chirho) in fields_chirho {
            let kind_chirho = self.resolve_chirho(&kind_chirho, span_chirho)?;
            let mut budget_chirho = self.lend_equality_budget_chirho(span_chirho)?;
            let initial_chirho = budget_chirho.remaining_chirho();
            let normalized_chirho = view_chirho.normalize_chirho(
                catalogue_chirho,
                self.owner_chirho.is_some(),
                self.next_variable_chirho,
                &kind_chirho,
                &mut budget_chirho,
            );
            self.settle_equality_budget_chirho(
                initial_chirho,
                &budget_chirho,
                normalized_chirho.is_err(),
                span_chirho,
            )?;
            let normalized_chirho = normalized_chirho.map_err(|_| {
                self.resource_failure_chirho(
                    "constructor field representation normalization bound exhausted",
                    span_chirho,
                )
            })?;
            if !self.has_fixed_value_kind_chirho(
                &normalized_chirho,
                catalogue_chirho,
                span_chirho,
            )? {
                return Err(TypeElaboratorChirho::error_chirho(
                    "data constructor field does not have a fixed runtime representation",
                    span_chirho,
                ));
            }
        }
        Ok(())
    }

    /// Ordinary data fields require an already concrete representation. This
    /// check neither reduces a stuck family nor creates equality evidence.
    pub(crate) fn has_fixed_value_kind_chirho(
        &mut self,
        kind_chirho: &TyChirho,
        catalogue_chirho: &TypeFamilyEnvChirho,
        span_chirho: SpanChirho,
    ) -> Result<bool, ElaborationErrorChirho> {
        let kind_chirho = self.resolve_chirho(kind_chirho, span_chirho)?;
        if kind_chirho == Self::type_universe_chirho() {
            return Ok(true);
        }
        let TyChirho::AppChirho(function_chirho, representation_chirho) = &kind_chirho else {
            return Ok(false);
        };
        if !matches!(&**function_chirho, TyChirho::ConChirho(key_chirho)
            if key_chirho == CompilerRepresentationChirho::RuntimeTypeChirho.type_key_chirho())
        {
            return Ok(false);
        }
        let mut pending_chirho = vec![representation_chirho.as_ref()];
        while let Some(term_chirho) = pending_chirho.pop() {
            self.charge_chirho(1, 0, span_chirho)?;
            match term_chirho {
                TyChirho::ConChirho(key_chirho) => {
                    let Some(scheme_chirho) = catalogue_chirho
                        .declaration_schemes_chirho()
                        .get_chirho(key_chirho)
                        .filter(|scheme_chirho| {
                            !scheme_chirho.held_chirho
                                && (scheme_chirho.declaration_chirho.is_some()
                                    || scheme_chirho.promoted_chirho.is_some())
                        })
                    else {
                        return Ok(false);
                    };
                    if !matches!(scheme_chirho.role_chirho,
                        haskelujah_ast_chirho::name_chirho::TypeConstructorRoleChirho::DataChirho
                        | haskelujah_ast_chirho::name_chirho::TypeConstructorRoleChirho::NewtypeChirho
                        | haskelujah_ast_chirho::name_chirho::TypeConstructorRoleChirho::UniverseChirho)
                    {
                        return Ok(false);
                    }
                }
                TyChirho::AppChirho(left_chirho, right_chirho)
                | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
                    self.charge_chirho(2, 2, span_chirho)?;
                    pending_chirho.extend([left_chirho.as_ref(), right_chirho.as_ref()]);
                }
                TyChirho::ListChirho(element_chirho) => {
                    self.charge_chirho(1, 1, span_chirho)?;
                    pending_chirho.push(element_chirho);
                }
                TyChirho::TupleChirho(elements_chirho) => {
                    self.charge_chirho(elements_chirho.len(), elements_chirho.len(), span_chirho)?;
                    pending_chirho.extend(elements_chirho);
                }
                TyChirho::VarChirho(_)
                | TyChirho::ForallVarChirho(_)
                | TyChirho::ForallChirho { .. }
                | TyChirho::RequiredForallChirho { .. }
                | TyChirho::QualChirho { .. } => return Ok(false),
            }
        }
        Ok(true)
    }

    /// Close only the supplied live roots, not the growing module record map.
    /// This is shared by publication/defaulting and protection of outer kinds.
    pub(crate) fn kind_dependency_closure_chirho(
        &mut self,
        roots_chirho: &[TyVarChirho],
        span_chirho: SpanChirho,
    ) -> Result<HashSet<TyVarChirho>, ElaborationErrorChirho> {
        self.charge_chirho(roots_chirho.len(), roots_chirho.len(), span_chirho)?;
        let mut pending_chirho = roots_chirho.to_vec();
        let mut closed_chirho = HashSet::new();
        while let Some(variable_chirho) = pending_chirho.pop() {
            let term_chirho =
                self.resolve_chirho(&TyChirho::VarChirho(variable_chirho), span_chirho)?;
            let variables_chirho = term_chirho.free_vars_chirho();
            self.charge_chirho(variables_chirho.len(), variables_chirho.len(), span_chirho)?;
            for variable_chirho in variables_chirho {
                if !closed_chirho.insert(variable_chirho) {
                    continue;
                }
                let kind_chirho = self
                    .variable_kinds_chirho
                    .get(&variable_chirho)
                    .and_then(|record_chirho| record_chirho.kind_chirho.clone());
                if let Some(kind_chirho) = kind_chirho {
                    let kind_chirho = self.resolve_chirho(&kind_chirho, span_chirho)?;
                    let dependencies_chirho = kind_chirho.free_vars_chirho();
                    self.charge_chirho(
                        dependencies_chirho.len(),
                        dependencies_chirho.len(),
                        span_chirho,
                    )?;
                    pending_chirho.extend(dependencies_chirho);
                }
            }
        }
        Ok(closed_chirho)
    }

    pub(crate) fn fresh_value_kind_chirho(
        &mut self,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        self.charge_chirho(3, 3, span_chirho)?;
        let runtime_rep_chirho = self.name_chirho(
            CompilerRepresentationChirho::RuntimeRepChirho.type_key_chirho(),
            span_chirho,
        )?;
        let variable_chirho =
            self.fresh_chirho(Some(TyChirho::ConChirho(runtime_rep_chirho)), span_chirho)?;
        let runtime_type_chirho = self.name_chirho(
            CompilerRepresentationChirho::RuntimeTypeChirho.type_key_chirho(),
            span_chirho,
        )?;
        Ok(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(runtime_type_chirho)),
            Box::new(TyChirho::VarChirho(variable_chirho)),
        ))
    }

    /// Call only at an actual generalization boundary, after closing both
    /// candidate and protected sets over their checked kind dependencies.
    /// Unknown records and producer-bound universals never acquire a default.
    pub(crate) fn default_representation_variables_chirho(
        &mut self,
        candidates_chirho: &HashSet<TyVarChirho>,
        protected_chirho: &HashSet<TyVarChirho>,
        span_chirho: SpanChirho,
    ) -> Result<SubstChirho, ElaborationErrorChirho> {
        self.charge_chirho(
            candidates_chirho.len(),
            candidates_chirho.len(),
            span_chirho,
        )?;
        let mut ordered_chirho = candidates_chirho.iter().copied().collect::<Vec<_>>();
        ordered_chirho.sort_unstable();
        let mut defaults_chirho = SubstChirho::empty_chirho();
        for variable_chirho in ordered_chirho {
            let root_chirho =
                self.resolve_chirho(&TyChirho::VarChirho(variable_chirho), span_chirho)?;
            let TyChirho::VarChirho(root_chirho) = root_chirho else {
                continue;
            };
            if !candidates_chirho.contains(&root_chirho)
                || protected_chirho.contains(&root_chirho)
                || self.variable_binding_chirho(root_chirho)
                    != ElaborationVariableBindingChirho::MetaChirho
                || self.rigid_variables_chirho.contains(&root_chirho)
            {
                continue;
            }
            self.charge_chirho(self.kind_scope_guards_chirho.len(), 0, span_chirho)?;
            if self.kind_scope_guards_chirho.iter().any(|guard_chirho| {
                guard_chirho.variables_chirho.contains(&root_chirho)
                    || guard_chirho.protected_chirho.contains(&root_chirho)
            }) {
                continue;
            }
            let Some(kind_chirho) = self
                .variable_kinds_chirho
                .get(&root_chirho)
                .and_then(|record_chirho| record_chirho.kind_chirho.clone())
            else {
                continue;
            };
            let kind_chirho = self.resolve_chirho(&kind_chirho, span_chirho)?;
            let TyChirho::ConChirho(kind_key_chirho) = kind_chirho else {
                continue;
            };
            let default_chirho = if kind_key_chirho
                == CompilerRepresentationChirho::RuntimeRepChirho.type_key_chirho()
            {
                self.copy_chirho(
                    crate::ty_chirho::representation_chirho::boxed_representation_chirho(true),
                    span_chirho,
                )?
            } else if kind_key_chirho
                == CompilerRepresentationChirho::LevityChirho.type_key_chirho()
            {
                let key_chirho =
                    haskelujah_ast_chirho::promotion_chirho::PromotedKeyChirho::builtin_chirho(
                        haskelujah_ast_chirho::promotion_chirho::BUILTIN_CONSTRUCTOR_MODULE_CHIRHO,
                        "Lifted",
                    );
                TyChirho::ConChirho(self.name_chirho(key_chirho.as_str_chirho(), span_chirho)?)
            } else {
                continue;
            };
            self.unify_chirho(
                &TyChirho::VarChirho(root_chirho),
                &default_chirho,
                span_chirho,
            )?;
            self.charge_chirho(1, 1, span_chirho)?;
            defaults_chirho.insert_chirho(root_chirho, default_chirho);
        }
        Ok(defaults_chirho)
    }
}
