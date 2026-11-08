// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Checked family producers consume the same declaration boundary and owned
//! variables as ordinary inference. Branch-local names encode quantified slots,
//! never source spellings. Workflow: declaration-kinds-chirho.

use super::*;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

#[cfg(test)]
#[path = "checked_family_scope_tests_chirho.rs"]
mod scope_tests_chirho;

impl InferCtxChirho {
    /// The adopted declaration solver already published this exact local
    /// closed producer. Reuse its immutable entry, including held branches,
    /// before AST conversion can spend another allowance or remint variables.
    pub(super) fn has_published_checked_closed_family_chirho(
        &self,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
        arity_chirho: usize,
        equation_count_chirho: usize,
    ) -> bool {
        use haskelujah_ast_chirho::name_chirho::{
            TypeConstructorIdentityChirho, TypeConstructorRoleChirho,
        };
        if self.elaboration_state_chirho.owner_chirho.is_none() {
            return false;
        }
        let Some(proof_chirho) = name_chirho.type_constructor_chirho() else {
            return false;
        };
        let Some(key_chirho) = self.family_name_key_chirho(&name_chirho.full_name_chirho()) else {
            return false;
        };
        let Some(header_chirho) = self
            .type_families_chirho
            .declaration_schemes_chirho()
            .get_chirho(&key_chirho)
        else {
            return false;
        };
        if header_chirho.owner_chirho.is_some()
            || header_chirho.held_chirho
            || header_chirho.role_chirho != TypeConstructorRoleChirho::TypeFamilyChirho
            || header_chirho.declaration_chirho.as_ref() != Some(proof_chirho)
            || header_chirho.reduction_arity_chirho != arity_chirho
        {
            return false;
        }
        let Some(definition_chirho) = self.type_families_chirho.definition_chirho(&key_chirho)
        else {
            return false;
        };
        let TypeConstructorIdentityChirho::SourceChirho {
            defining_module_chirho,
            name_chirho: defining_name_chirho,
        } = &proof_chirho.identity_chirho
        else {
            return false;
        };
        matches!(definition_chirho.id_chirho(), TypeFamilyIdChirho::SourceChirho { module_chirho, name_chirho }
            if module_chirho == defining_module_chirho
                && module_chirho == &self.family_module_name_chirho
                && name_chirho == defining_name_chirho)
            && definition_chirho.reduction_chirho() == TypeFamilyReductionChirho::ClosedChirho
            && definition_chirho.arity_chirho() == arity_chirho
            && definition_chirho.branches_chirho().len() == equation_count_chirho
    }

    pub(super) fn checked_family_arity_chirho(&self, name_chirho: &str) -> Option<usize> {
        let key_chirho = self.family_name_key_chirho(name_chirho)?;
        let scheme_chirho = self
            .type_families_chirho
            .declaration_schemes_chirho()
            .get_chirho(&key_chirho)?;
        (scheme_chirho.owner_chirho.is_none() && !scheme_chirho.held_chirho)
            .then_some(scheme_chirho.reduction_arity_chirho)
    }

    pub(super) fn checked_family_terms_chirho(
        &mut self,
        lhs_chirho: &[TypeChirho],
        rhs_chirho: &TypeChirho,
    ) -> Option<(Vec<TyChirho>, TyChirho, Vec<String>)> {
        self.elaboration_state_chirho.owner_chirho?;
        let previous_chirho = self.definition_type_conversion_chirho;
        self.definition_type_conversion_chirho = true;
        let mut variables_chirho = HashMap::new();
        let lhs_chirho = lhs_chirho
            .iter()
            .map(|ty_chirho| self.ast_type_to_ty_chirho(ty_chirho, &mut variables_chirho))
            .collect::<Vec<_>>();
        let rhs_chirho = self.ast_type_to_ty_chirho(rhs_chirho, &mut variables_chirho);
        self.definition_type_conversion_chirho = previous_chirho;
        if self.has_non_deferrable_failure_chirho() {
            return None;
        }
        let lhs_chirho = lhs_chirho
            .iter()
            .map(|ty_chirho| self.expand_type_synonyms_chirho(ty_chirho))
            .collect::<Vec<_>>();
        let rhs_chirho = self.expand_type_synonyms_chirho(&rhs_chirho);
        self.abstract_checked_family_terms_chirho(&lhs_chirho, &rhs_chirho)
    }

    pub(super) fn abstract_checked_family_terms_chirho(
        &mut self,
        lhs_chirho: &[TyChirho],
        rhs_chirho: &TyChirho,
    ) -> Option<(Vec<TyChirho>, TyChirho, Vec<String>)> {
        self.specialize_checked_family_terms_chirho(
            lhs_chirho,
            rhs_chirho,
            &HashMap::new(),
            &mut 100_000_usize,
        )
    }

    pub(super) fn specialize_checked_family_terms_chirho(
        &mut self,
        lhs_chirho: &[TyChirho],
        rhs_chirho: &TyChirho,
        images_chirho: &HashMap<TyVarChirho, std::borrow::Cow<'_, TyChirho>>,
        work_chirho: &mut usize,
    ) -> Option<(Vec<TyChirho>, TyChirho, Vec<String>)> {
        let result_chirho = abstract_checked_family_terms_with_budget_chirho(
            lhs_chirho,
            rhs_chirho,
            images_chirho,
            &mut |weight_chirho, output_chirho, depth_chirho, preflight_chirho| {
                if depth_chirho > 128 || (preflight_chirho && output_chirho > *work_chirho) {
                    return Err(CheckedFamilyAbstractionFailureChirho::ResourceChirho);
                }
                *work_chirho = work_chirho
                    .checked_sub(weight_chirho)
                    .ok_or(CheckedFamilyAbstractionFailureChirho::ResourceChirho)?;
                Ok(())
            },
        );
        match result_chirho {
            Ok(terms_chirho) => Some(terms_chirho),
            Err(_) => {
                let diagnostic_chirho = DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(300),
                    "checked family variable scope or conversion resource bound failed",
                    SpanChirho::DUMMY_CHIRHO,
                );
                self.diagnostics_chirho
                    .push_chirho(diagnostic_chirho.clone());
                self.non_deferrable_diagnostics_chirho
                    .push_chirho(diagnostic_chirho);
                None
            }
        }
    }
}

/// The same first-order abstraction serves ordinary registration and the
/// producer's early checked declaration publication. No RHS-only variable is
/// silently freshened, and the caller owns every work/output/depth refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CheckedFamilyAbstractionFailureChirho {
    UnsupportedChirho,
    UnboundChirho,
    ResourceChirho,
}

pub(crate) type CheckedFamilyTermsChirho = (Vec<TyChirho>, TyChirho, Vec<String>);

pub(crate) fn abstract_checked_family_terms_with_budget_chirho(
    lhs_chirho: &[TyChirho],
    rhs_chirho: &TyChirho,
    images_chirho: &HashMap<TyVarChirho, std::borrow::Cow<'_, TyChirho>>,
    charge_chirho: &mut impl FnMut(
        usize,
        usize,
        usize,
        bool,
    ) -> Result<(), CheckedFamilyAbstractionFailureChirho>,
) -> Result<CheckedFamilyTermsChirho, CheckedFamilyAbstractionFailureChirho> {
    let mut binders_chirho = HashMap::new();
    let lhs_chirho = lhs_chirho
        .iter()
        .map(|ty_chirho| {
            quantify_family_term_chirho(
                ty_chirho,
                Some(images_chirho),
                &mut binders_chirho,
                true,
                charge_chirho,
                0,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let rhs_chirho = quantify_family_term_chirho(
        rhs_chirho,
        Some(images_chirho),
        &mut binders_chirho,
        false,
        charge_chirho,
        0,
    )?;
    let mut names_chirho = binders_chirho.into_values().collect::<Vec<_>>();
    names_chirho.sort();
    Ok((lhs_chirho, rhs_chirho, names_chirho))
}

fn quantify_family_term_chirho(
    ty_chirho: &TyChirho,
    images_chirho: Option<&HashMap<TyVarChirho, std::borrow::Cow<'_, TyChirho>>>,
    binders_chirho: &mut HashMap<TyVarChirho, String>,
    introduce_chirho: bool,
    charge_chirho: &mut impl FnMut(
        usize,
        usize,
        usize,
        bool,
    ) -> Result<(), CheckedFamilyAbstractionFailureChirho>,
    depth_chirho: usize,
) -> Result<TyChirho, CheckedFamilyAbstractionFailureChirho> {
    charge_chirho(1, 1, depth_chirho, false)?;
    if let TyChirho::TupleChirho(elements_chirho) = ty_chirho {
        charge_chirho(0, elements_chirho.len(), depth_chirho, true)?;
    }
    if let TyChirho::VarChirho(variable_chirho) = ty_chirho
        && let Some(image_chirho) =
            images_chirho.and_then(|images_chirho| images_chirho.get(variable_chirho))
    {
        return quantify_family_term_chirho(
            image_chirho.as_ref(),
            None,
            binders_chirho,
            introduce_chirho,
            charge_chirho,
            depth_chirho + 1,
        );
    }
    let mut child_chirho = |ty_chirho: &TyChirho| {
        quantify_family_term_chirho(
            ty_chirho,
            images_chirho,
            binders_chirho,
            introduce_chirho,
            charge_chirho,
            depth_chirho + 1,
        )
    };
    Ok(match ty_chirho {
        TyChirho::VarChirho(variable_chirho) => {
            if !binders_chirho.contains_key(variable_chirho) {
                if !introduce_chirho {
                    return Err(CheckedFamilyAbstractionFailureChirho::UnboundChirho);
                }
                // The generated slot name is bounded independently of input
                // spelling; reserve both the map and result string first.
                charge_chirho(
                    0,
                    2 * ("%family-quantifier:".len() + 20),
                    depth_chirho,
                    false,
                )?;
                binders_chirho.insert(
                    *variable_chirho,
                    format!("%family-quantifier:{}", binders_chirho.len()),
                );
            } else {
                charge_chirho(
                    0,
                    binders_chirho[variable_chirho].len(),
                    depth_chirho,
                    false,
                )?;
            }
            TyChirho::ForallVarChirho(binders_chirho[variable_chirho].clone())
        }
        TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
            charge_chirho(name_chirho.len(), name_chirho.len(), depth_chirho, false)?;
            ty_chirho.clone()
        }
        TyChirho::AppChirho(fun_chirho, argument_chirho) => TyChirho::AppChirho(
            Box::new(child_chirho(fun_chirho)?),
            Box::new(child_chirho(argument_chirho)?),
        ),
        TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
            TyChirho::FunChirho(
                Box::new(child_chirho(argument_chirho)?),
                Box::new(child_chirho(result_chirho)?),
                *multiplicity_chirho,
            )
        }
        TyChirho::ListChirho(element_chirho) => {
            TyChirho::ListChirho(Box::new(child_chirho(element_chirho)?))
        }
        TyChirho::TupleChirho(elements_chirho) => TyChirho::TupleChirho(
            elements_chirho
                .iter()
                .map(&mut child_chirho)
                .collect::<Result<Vec<_>, _>>()?,
        ),
        TyChirho::QualChirho { .. }
        | TyChirho::ForallChirho { .. }
        | TyChirho::RequiredForallChirho { .. } => {
            return Err(CheckedFamilyAbstractionFailureChirho::UnsupportedChirho);
        }
    })
}
