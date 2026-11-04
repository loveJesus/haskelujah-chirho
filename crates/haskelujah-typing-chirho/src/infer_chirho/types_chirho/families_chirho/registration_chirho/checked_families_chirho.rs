// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Checked family producers consume the same declaration boundary and owned
//! variables as ordinary inference. Branch-local names encode quantified slots,
//! never source spellings. Workflow: declaration-kinds-chirho.

use super::*;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

impl InferCtxChirho {
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
        self.elaboration_owner_chirho?;
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
        let mut binders_chirho = HashMap::new();
        let mut work_chirho = 100_000_usize;
        let lhs_chirho = lhs_chirho
            .iter()
            .map(|ty_chirho| {
                quantify_family_term_chirho(
                    ty_chirho,
                    &mut binders_chirho,
                    true,
                    &mut work_chirho,
                    0,
                )
            })
            .collect::<Option<Vec<_>>>();
        let rhs_chirho = quantify_family_term_chirho(
            &rhs_chirho,
            &mut binders_chirho,
            false,
            &mut work_chirho,
            0,
        );
        match lhs_chirho.zip(rhs_chirho) {
            Some((lhs_chirho, rhs_chirho)) => {
                let mut names_chirho = binders_chirho.into_values().collect::<Vec<_>>();
                names_chirho.sort();
                Some((lhs_chirho, rhs_chirho, names_chirho))
            }
            None => {
                let diagnostic_chirho = DiagnosticChirho::error_chirho(
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

fn quantify_family_term_chirho(
    ty_chirho: &TyChirho,
    binders_chirho: &mut HashMap<TyVarChirho, String>,
    introduce_chirho: bool,
    work_chirho: &mut usize,
    depth_chirho: usize,
) -> Option<TyChirho> {
    *work_chirho = work_chirho.checked_sub(1)?;
    if depth_chirho > 128 {
        return None;
    }
    let mut child_chirho = |ty_chirho: &TyChirho| {
        quantify_family_term_chirho(
            ty_chirho,
            binders_chirho,
            introduce_chirho,
            work_chirho,
            depth_chirho + 1,
        )
    };
    Some(match ty_chirho {
        TyChirho::VarChirho(variable_chirho) => {
            if !binders_chirho.contains_key(variable_chirho) {
                if !introduce_chirho {
                    return None;
                }
                binders_chirho.insert(
                    *variable_chirho,
                    format!("%family-quantifier:{}", binders_chirho.len()),
                );
            }
            TyChirho::ForallVarChirho(binders_chirho[variable_chirho].clone())
        }
        TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
            *work_chirho = work_chirho.checked_sub(name_chirho.len())?;
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
                .collect::<Option<Vec<_>>>()?,
        ),
        // Higher-rank equations are not representable by first-order branch
        // matching. No inner universal becomes an equation wildcard.
        TyChirho::ForallChirho { .. } | TyChirho::RequiredForallChirho { .. } => return None,
    })
}
