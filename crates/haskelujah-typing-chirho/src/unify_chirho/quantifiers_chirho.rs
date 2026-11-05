// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded alignment of the same checked lexical quantifier records.

use super::*;

/// Align the checked kinds under preceding alpha bindings. Unknown metadata
/// remains unknown and never becomes a wildcard equality proof.
pub(crate) fn align_forall_prefix_chirho(
    left_chirho: &crate::ty_chirho::QuantifiedVariablesChirho,
    right_chirho: &crate::ty_chirho::QuantifiedVariablesChirho,
    span_chirho: SpanChirho,
) -> Result<(SubstChirho, SubstChirho), UnifyErrorChirho> {
    let mut work_chirho = 10_000_000usize;
    let mut output_chirho = 16_384usize;
    let mut charge_chirho = |work_amount_chirho: usize, output_amount_chirho: usize| {
        work_chirho = work_chirho.checked_sub(work_amount_chirho)?;
        output_chirho = output_chirho.checked_sub(output_amount_chirho)?;
        Some(())
    };
    charge_chirho(
        left_chirho.len().saturating_add(right_chirho.len()),
        left_chirho.len(),
    )
    .ok_or(UnifyErrorChirho::ResourceExhaustedChirho { span_chirho })?;
    let mut rename_chirho = SubstChirho::empty_chirho();
    let mut kinds_chirho = SubstChirho::empty_chirho();
    let mut rigid_chirho = std::collections::HashSet::new();
    for (left_chirho, right_chirho) in left_chirho
        .records_chirho()
        .iter()
        .zip(right_chirho.records_chirho())
    {
        if let (Some(left_kind_chirho), Some(right_kind_chirho)) =
            (&left_chirho.kind_chirho, &right_chirho.kind_chirho)
        {
            let left_kind_chirho = kinds_chirho
                .apply_bounded_chirho(left_kind_chirho, &mut charge_chirho)
                .ok_or(UnifyErrorChirho::ResourceExhaustedChirho { span_chirho })?;
            let right_kind_chirho = rename_chirho
                .apply_bounded_chirho(right_kind_chirho, &mut charge_chirho)
                .and_then(|kind_chirho| {
                    kinds_chirho.apply_bounded_chirho(&kind_chirho, &mut charge_chirho)
                })
                .ok_or(UnifyErrorChirho::ResourceExhaustedChirho { span_chirho })?;
            let solution_chirho =
                unify_kind_terms_chirho(&left_kind_chirho, &right_kind_chirho, span_chirho)?;
            protect_prefix_chirho(
                &rigid_chirho,
                &solution_chirho,
                &mut charge_chirho,
                &left_kind_chirho,
                &right_kind_chirho,
                span_chirho,
            )?;
            kinds_chirho = solution_chirho.compose_chirho(&kinds_chirho);
        }
        rigid_chirho.insert(left_chirho.variable_chirho);
        rename_chirho.insert_chirho(
            right_chirho.variable_chirho,
            TyChirho::VarChirho(left_chirho.variable_chirho),
        );
    }
    Ok((rename_chirho, kinds_chirho))
}

fn protect_prefix_chirho(
    rigid_chirho: &std::collections::HashSet<TyVarChirho>,
    solution_chirho: &SubstChirho,
    charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    left_chirho: &TyChirho,
    right_chirho: &TyChirho,
    span_chirho: SpanChirho,
) -> Result<(), UnifyErrorChirho> {
    charge_chirho(solution_chirho.len_chirho(), 0)
        .ok_or(UnifyErrorChirho::ResourceExhaustedChirho { span_chirho })?;
    for (variable_chirho, image_chirho) in solution_chirho.iter_chirho() {
        let free_chirho = SubstChirho::free_variables_bounded_chirho(image_chirho, charge_chirho)
            .ok_or(UnifyErrorChirho::ResourceExhaustedChirho { span_chirho })?;
        charge_chirho(free_chirho.len(), 0)
            .ok_or(UnifyErrorChirho::ResourceExhaustedChirho { span_chirho })?;
        if rigid_chirho.contains(variable_chirho)
            || free_chirho
                .iter()
                .any(|variable_chirho| rigid_chirho.contains(variable_chirho))
        {
            return Err(UnifyErrorChirho::MismatchChirho {
                expected_chirho: left_chirho.clone(),
                actual_chirho: right_chirho.clone(),
                span_chirho,
            });
        }
    }
    Ok(())
}

pub(super) fn protect_kind_body_chirho(
    vars_chirho: &crate::ty_chirho::QuantifiedVariablesChirho,
    solution_chirho: &SubstChirho,
    left_chirho: &TyChirho,
    right_chirho: &TyChirho,
    span_chirho: SpanChirho,
) -> Result<(), UnifyErrorChirho> {
    let mut work_chirho = 10_000_000usize;
    let mut output_chirho = 16_384usize;
    let mut charge_chirho = |work_amount_chirho: usize, output_amount_chirho: usize| {
        work_chirho = work_chirho.checked_sub(work_amount_chirho)?;
        output_chirho = output_chirho.checked_sub(output_amount_chirho)?;
        Some(())
    };
    charge_chirho(vars_chirho.len(), vars_chirho.len())
        .ok_or(UnifyErrorChirho::ResourceExhaustedChirho { span_chirho })?;
    let rigid_chirho = vars_chirho.iter().copied().collect();
    protect_prefix_chirho(
        &rigid_chirho,
        solution_chirho,
        &mut charge_chirho,
        left_chirho,
        right_chirho,
        span_chirho,
    )
}
