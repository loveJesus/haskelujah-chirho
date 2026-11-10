// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Borrow checked telescope/variable kinds for a speculative monotype solution.
//! Unlike value-kind projection, every supplied application argument is checked.
//! The returned equations and copied kinds belong to the caller's local transaction.

use crate::infer_chirho::TypeFamilyEnvChirho;
use crate::kind_chirho::elaboration_chirho::ElaborationStateChirho;
use crate::kind_chirho::elaboration_chirho::state_chirho::{
    CONSTRAINT_UNIVERSE_KEY_CHIRHO, TYPE_UNIVERSE_KEY_CHIRHO,
};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::declarations_chirho::TelescopeVisibilityChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};
use haskelujah_ast_chirho::name_chirho::{
    CompilerTypeConstructorChirho, TypeConstructorRoleChirho,
};
use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ElaborationOwnerChirho;
use std::collections::HashSet;

struct KindCheckChirho<'metadata_chirho, 'operation_chirho, LookupChirho, ChargeChirho> {
    registry_chirho: &'metadata_chirho TypeFamilyEnvChirho,
    owner_chirho: ElaborationOwnerChirho,
    lookup_chirho: &'metadata_chirho LookupChirho,
    substitution_chirho: &'metadata_chirho SubstChirho,
    equations_chirho: &'operation_chirho mut Vec<(TyChirho, TyChirho)>,
    charge_chirho: &'operation_chirho mut ChargeChirho,
}

/// Read actual producer records; absence never defaults to Type. Declaration
/// telescope variables are interpreted simultaneously before entering the
/// consumer's substitution, even when the numeric IDs happen to coincide.
/// workflow: language-features-chirho/fundep-generalization-chirho
pub(crate) fn check_monomorphic_kind_chirho<'kind_chirho>(
    registry_chirho: &TypeFamilyEnvChirho,
    owner_chirho: ElaborationOwnerChirho,
    term_chirho: &TyChirho,
    substitution_chirho: &SubstChirho,
    lookup_chirho: &impl Fn(TyVarChirho) -> Option<&'kind_chirho TyChirho>,
    equations_chirho: &mut Vec<(TyChirho, TyChirho)>,
    charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
) -> Result<Option<TyChirho>, ()> {
    KindCheckChirho {
        registry_chirho,
        owner_chirho,
        lookup_chirho,
        substitution_chirho,
        equations_chirho,
        charge_chirho,
    }
    .term_chirho(term_chirho, 0)
}

impl<'kind_chirho, LookupChirho, ChargeChirho> KindCheckChirho<'_, '_, LookupChirho, ChargeChirho>
where
    LookupChirho: Fn(TyVarChirho) -> Option<&'kind_chirho TyChirho>,
    ChargeChirho: FnMut(usize, usize) -> Option<()>,
{
    fn copy_chirho(&mut self, term_chirho: &TyChirho) -> Result<TyChirho, ()> {
        self.substitution_chirho
            .resolve_owned_bounded_chirho(term_chirho, self.charge_chirho)
            .ok_or(())
    }

    fn equation_chirho(
        &mut self,
        expected_chirho: TyChirho,
        actual_chirho: TyChirho,
    ) -> Result<(), ()> {
        // These two sealed universes are already canonical checked kinds.
        // Reflexivity creates no constraint, binding or copied result image.
        // Arbitrary names, variables and family kinds keep the pending path.
        // workflow: language-features-chirho/fundep-generalization-chirho
        if let (TyChirho::ConChirho(left_chirho), TyChirho::ConChirho(right_chirho)) =
            (&expected_chirho, &actual_chirho)
        {
            (self.charge_chirho)(
                left_chirho
                    .len()
                    .checked_add(right_chirho.len())
                    .ok_or(())?,
                0,
            )
            .ok_or(())?;
            if left_chirho == right_chirho
                && matches!(
                    left_chirho.as_str(),
                    TYPE_UNIVERSE_KEY_CHIRHO | CONSTRAINT_UNIVERSE_KEY_CHIRHO
                )
            {
                return Ok(());
            }
        }
        (self.charge_chirho)(1, 1).ok_or(())?;
        self.equations_chirho.push((expected_chirho, actual_chirho));
        Ok(())
    }

    fn term_chirho(
        &mut self,
        term_chirho: &TyChirho,
        depth_chirho: usize,
    ) -> Result<Option<TyChirho>, ()> {
        (self.charge_chirho)(1, 0).ok_or(())?;
        if depth_chirho > 128 {
            return Err(());
        }
        let next_chirho = depth_chirho + 1;
        match term_chirho {
            TyChirho::VarChirho(variable_chirho) => {
                let Some(kind_chirho) = (self.lookup_chirho)(*variable_chirho) else {
                    return Ok(None);
                };
                self.copy_chirho(kind_chirho).map(Some)
            }
            TyChirho::ForallVarChirho(_)
            | TyChirho::ForallChirho { .. }
            | TyChirho::RequiredForallChirho { .. }
            | TyChirho::QualChirho { .. } => Ok(None),
            TyChirho::ConChirho(key_chirho) => {
                // These sealed internal universes are produced by this module,
                // not inferred from a source type constructor's written name.
                if term_chirho == &ElaborationStateChirho::type_universe_chirho()
                    || term_chirho == &ElaborationStateChirho::constraint_universe_chirho()
                {
                    return Ok(Some(ElaborationStateChirho::type_universe_chirho()));
                }
                self.declaration_chirho(key_chirho, &[], next_chirho)
            }
            TyChirho::AppChirho(_, _) => {
                let mut arguments_chirho = Vec::new();
                let mut head_chirho = term_chirho;
                while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
                    (self.charge_chirho)(1, 1).ok_or(())?;
                    arguments_chirho.push(argument_chirho.as_ref());
                    head_chirho = function_chirho;
                }
                arguments_chirho.reverse();
                if let TyChirho::ConChirho(key_chirho) = head_chirho {
                    return self.declaration_chirho(key_chirho, &arguments_chirho, next_chirho);
                }
                let Some(mut kind_chirho) = self.term_chirho(head_chirho, next_chirho)? else {
                    return Ok(None);
                };
                for argument_chirho in arguments_chirho {
                    let TyChirho::FunChirho(expected_chirho, result_chirho, _) = kind_chirho else {
                        return Ok(None);
                    };
                    let Some(actual_chirho) = self.term_chirho(argument_chirho, next_chirho)?
                    else {
                        return Ok(None);
                    };
                    self.equation_chirho(*expected_chirho, actual_chirho)?;
                    kind_chirho = *result_chirho;
                }
                Ok(Some(kind_chirho))
            }
            TyChirho::ListChirho(element_chirho) => self.intrinsic_chirho(
                CompilerTypeConstructorChirho::ListChirho,
                &[element_chirho.as_ref()],
                next_chirho,
            ),
            TyChirho::FunChirho(argument_chirho, result_chirho, _) => self.intrinsic_chirho(
                CompilerTypeConstructorChirho::FunctionChirho,
                &[argument_chirho.as_ref(), result_chirho.as_ref()],
                next_chirho,
            ),
            TyChirho::TupleChirho(elements_chirho) => {
                let identity_chirho = if elements_chirho.is_empty() {
                    CompilerTypeConstructorChirho::UnitChirho
                } else {
                    let Ok(arity_chirho) = u8::try_from(elements_chirho.len()) else {
                        return Ok(None);
                    };
                    CompilerTypeConstructorChirho::TupleChirho(arity_chirho)
                };
                (self.charge_chirho)(elements_chirho.len(), elements_chirho.len()).ok_or(())?;
                self.intrinsic_chirho(
                    identity_chirho,
                    &elements_chirho.iter().collect::<Vec<_>>(),
                    next_chirho,
                )
            }
        }
    }

    fn intrinsic_chirho(
        &mut self,
        identity_chirho: CompilerTypeConstructorChirho,
        arguments_chirho: &[&TyChirho],
        depth_chirho: usize,
    ) -> Result<Option<TyChirho>, ()> {
        use haskelujah_ast_chirho::name_chirho::TypeConstructorIdentityChirho;
        let key_chirho = identity_chirho.name_chirho();
        let registry_chirho = self.registry_chirho;
        let Some(scheme_chirho) = registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(key_chirho)
        else {
            return Ok(None);
        };
        if !scheme_chirho
            .declaration_chirho
            .as_ref()
            .is_some_and(|declaration_chirho| {
                declaration_chirho.identity_chirho
                    == TypeConstructorIdentityChirho::CompilerChirho(identity_chirho)
            })
        {
            return Ok(None);
        }
        self.declaration_chirho(key_chirho, arguments_chirho, depth_chirho)
    }

    fn declaration_chirho(
        &mut self,
        key_chirho: &str,
        arguments_chirho: &[&TyChirho],
        depth_chirho: usize,
    ) -> Result<Option<TyChirho>, ()> {
        (self.charge_chirho)(key_chirho.len().checked_add(1).ok_or(())?, 0).ok_or(())?;
        let registry_chirho = self.registry_chirho;
        let Some(scheme_chirho) = registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(key_chirho)
        else {
            return Ok(None);
        };
        if scheme_chirho.held_chirho
            || (scheme_chirho.declaration_chirho.is_none()
                && scheme_chirho.promoted_chirho.is_none())
            || scheme_chirho
                .owner_chirho
                .is_some_and(|owner_chirho| owner_chirho != self.owner_chirho)
            || matches!(
                scheme_chirho.role_chirho,
                TypeConstructorRoleChirho::UnknownChirho
                    | TypeConstructorRoleChirho::TypeFamilyChirho
                    | TypeConstructorRoleChirho::DataFamilyChirho
                    | TypeConstructorRoleChirho::TypeSynonymChirho
            )
            || arguments_chirho.len() > scheme_chirho.binders_chirho.len()
        {
            return Ok(None);
        }
        if scheme_chirho.declaration_chirho.as_ref().is_some_and(|declaration_chirho| {
            matches!(declaration_chirho.identity_chirho, haskelujah_ast_chirho::name_chirho::TypeConstructorIdentityChirho::CompilerChirho(identity_chirho) if identity_chirho.scalar_representation_chirho().is_some())
        }) { return Ok(None) }
        (self.charge_chirho)(
            scheme_chirho.binders_chirho.len(),
            scheme_chirho.binders_chirho.len(),
        )
        .ok_or(())?;
        // Prefix images are read only by telescope kinds. Closed Type-kinded
        // binders must still validate every argument, but need no copied type
        // image when no later kind or result can reference that binder.
        let mut kind_dependencies_chirho = HashSet::new();
        for kind_chirho in scheme_chirho
            .binders_chirho
            .iter()
            .map(|binder_chirho| &binder_chirho.kind_chirho)
            .chain(std::iter::once(&scheme_chirho.result_kind_chirho))
        {
            for variable_chirho in
                SubstChirho::free_variables_bounded_chirho(kind_chirho, self.charge_chirho)
                    .ok_or(())?
            {
                (self.charge_chirho)(1, 0).ok_or(())?;
                if !kind_dependencies_chirho.contains(&variable_chirho) {
                    (self.charge_chirho)(1, 1).ok_or(())?;
                    kind_dependencies_chirho.insert(variable_chirho);
                }
            }
        }
        let mut unique_chirho = HashSet::new();
        let mut parameters_chirho = SubstChirho::empty_chirho();
        for (binder_chirho, argument_chirho) in
            scheme_chirho.binders_chirho.iter().zip(arguments_chirho)
        {
            if !unique_chirho.insert(binder_chirho.variable_chirho) {
                return Ok(None);
            }
            // Telescope IDs are producer-local: substitute only preceding
            // slots before the ordinary consumer mapping is consulted.
            let expected_chirho = parameters_chirho
                .apply_bounded_chirho(&binder_chirho.kind_chirho, self.charge_chirho)
                .ok_or(())?;
            let expected_chirho = self.copy_chirho(&expected_chirho)?;
            let Some(actual_chirho) = self.term_chirho(argument_chirho, depth_chirho + 1)? else {
                return Ok(None);
            };
            self.equation_chirho(expected_chirho, actual_chirho)?;
            (self.charge_chirho)(1, 0).ok_or(())?;
            if kind_dependencies_chirho.contains(&binder_chirho.variable_chirho) {
                let argument_chirho = SubstChirho::empty_chirho()
                    .apply_bounded_chirho(argument_chirho, self.charge_chirho)
                    .ok_or(())?;
                parameters_chirho.insert_chirho(binder_chirho.variable_chirho, argument_chirho);
            }
        }
        let mut result_chirho = parameters_chirho
            .apply_bounded_chirho(&scheme_chirho.result_kind_chirho, self.charge_chirho)
            .ok_or(())?;
        for binder_chirho in scheme_chirho.binders_chirho[arguments_chirho.len()..]
            .iter()
            .rev()
        {
            if binder_chirho.visibility_chirho != TelescopeVisibilityChirho::RequiredChirho
                || !unique_chirho.insert(binder_chirho.variable_chirho)
            {
                return Ok(None);
            }
            let kind_chirho = parameters_chirho
                .apply_bounded_chirho(&binder_chirho.kind_chirho, self.charge_chirho)
                .ok_or(())?;
            // A remaining binder may not expose a producer-local dependent ID.
            let dependencies_chirho =
                SubstChirho::free_variables_bounded_chirho(&result_chirho, self.charge_chirho)
                    .ok_or(())?;
            if dependencies_chirho.contains(&binder_chirho.variable_chirho) {
                return Ok(None);
            }
            (self.charge_chirho)(1, 1).ok_or(())?;
            result_chirho = TyChirho::fun_chirho(kind_chirho, result_chirho);
        }
        self.copy_chirho(&result_chirho).map(Some)
    }
}
