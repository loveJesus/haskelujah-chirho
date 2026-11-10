// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use crate::subst_chirho::SubstitutionImageChirho;
use std::sync::Arc;

fn make_images_chirho(entries_chirho: &[(&str, Option<TyChirho>)]) -> SubstChirho {
    let mut images_chirho = SubstChirho::empty_chirho();
    for (index_chirho, (name_chirho, kind_chirho)) in entries_chirho.iter().enumerate() {
        images_chirho.insert_image_chirho(
            TyVarChirho(index_chirho as u32),
            SubstitutionImageChirho {
                image_chirho: TyChirho::ForallVarChirho((*name_chirho).into()),
                kind_chirho: kind_chirho.clone().map(Arc::new),
                source_chirho: None,
            },
        );
    }
    images_chirho
}

type KindProjectionChirho = (Result<Option<TyChirho>, ()>, Vec<(TyChirho, TyChirho)>);

fn project_chirho(
    term_chirho: &TyChirho,
    images_chirho: Option<&SubstChirho>,
    substitution_chirho: &SubstChirho,
    allowance_chirho: usize,
) -> KindProjectionChirho {
    let state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let registry_chirho = TypeFamilyEnvChirho::default();
    let mut equations_chirho = Vec::new();
    let mut remaining_chirho = (allowance_chirho, allowance_chirho);
    let result_chirho = check_monomorphic_kind_in_environment_chirho(
        &registry_chirho,
        state_chirho.owner_chirho.unwrap(),
        term_chirho,
        substitution_chirho,
        CheckedKindEnvironmentChirho {
            variables_chirho: &|_| None,
            lexical_images_chirho: images_chirho,
        },
        &mut equations_chirho,
        &mut |work_chirho, output_chirho| {
            remaining_chirho = remaining_chirho
                .0
                .checked_sub(work_chirho)
                .zip(remaining_chirho.1.checked_sub(output_chirho))?;
            Some(())
        },
    );
    (result_chirho, equations_chirho)
}

#[test]
fn a_rigid_kind_requires_an_exact_present_image_chirho() {
    let name_chirho = crate::skolem_chirho::skolem_name_chirho("BinderChirho", 7);
    let term_chirho = TyChirho::ForallVarChirho(name_chirho.clone());
    let ty_chirho = ElaborationStateChirho::type_universe_chirho();
    let images_chirho = make_images_chirho(&[(&name_chirho, Some(ty_chirho.clone()))]);
    let substitution_chirho = SubstChirho::empty_chirho();
    assert_eq!(
        project_chirho(
            &term_chirho,
            Some(&images_chirho),
            &substitution_chirho,
            4096
        )
        .0,
        Ok(Some(ty_chirho))
    );
    for view_chirho in [None, Some(&SubstChirho::empty_chirho())] {
        assert_eq!(
            project_chirho(&term_chirho, view_chirho, &substitution_chirho, 4096).0,
            Ok(None)
        );
    }
    let missing_chirho = make_images_chirho(&[(&name_chirho, None)]);
    assert_eq!(
        project_chirho(
            &term_chirho,
            Some(&missing_chirho),
            &substitution_chirho,
            4096
        )
        .0,
        Ok(None)
    );
}

#[test]
fn plain_pattern_variables_never_borrow_a_rigid_kind_chirho() {
    let images_chirho = make_images_chirho(&[(
        "PatternChirho",
        Some(ElaborationStateChirho::type_universe_chirho()),
    )]);
    assert_eq!(
        project_chirho(
            &TyChirho::ForallVarChirho("PatternChirho".into()),
            Some(&images_chirho),
            &SubstChirho::empty_chirho(),
            4096,
        )
        .0,
        Ok(None)
    );
}

#[test]
fn alias_kinds_agree_only_after_the_actual_commit_chirho() {
    let name_chirho = crate::skolem_chirho::skolem_name_chirho("AliasChirho", 9);
    let term_chirho = TyChirho::ForallVarChirho(name_chirho.clone());
    let ty_chirho = ElaborationStateChirho::type_universe_chirho();
    let images_chirho = make_images_chirho(&[
        (&name_chirho, Some(TyChirho::VarChirho(TyVarChirho(30)))),
        (&name_chirho, Some(ty_chirho.clone())),
    ]);
    assert_eq!(
        project_chirho(
            &term_chirho,
            Some(&images_chirho),
            &SubstChirho::empty_chirho(),
            4096
        )
        .0,
        Ok(None)
    );
    let mut substitution_chirho = SubstChirho::empty_chirho();
    substitution_chirho.insert_chirho(TyVarChirho(30), ty_chirho.clone());
    assert_eq!(
        project_chirho(
            &term_chirho,
            Some(&images_chirho),
            &substitution_chirho,
            4096
        )
        .0,
        Ok(Some(ty_chirho))
    );
}

#[test]
fn conflicting_rigid_kinds_do_not_publish_agreement_chirho() {
    let name_chirho = crate::skolem_chirho::skolem_name_chirho("ConflictChirho", 11);
    let images_chirho = make_images_chirho(&[
        (
            &name_chirho,
            Some(ElaborationStateChirho::type_universe_chirho()),
        ),
        (
            &name_chirho,
            Some(ElaborationStateChirho::constraint_universe_chirho()),
        ),
    ]);
    assert_eq!(
        project_chirho(
            &TyChirho::ForallVarChirho(name_chirho),
            Some(&images_chirho),
            &SubstChirho::empty_chirho(),
            4096,
        )
        .0,
        Ok(None)
    );
}

#[test]
fn rigid_images_inside_applications_keep_their_kind_equations_chirho() {
    let function_chirho = crate::skolem_chirho::skolem_name_chirho("FunctionChirho", 13);
    let argument_chirho = crate::skolem_chirho::skolem_name_chirho("ArgumentChirho", 14);
    let ty_chirho = ElaborationStateChirho::type_universe_chirho();
    let constraint_chirho = ElaborationStateChirho::constraint_universe_chirho();
    for expected_chirho in [&ty_chirho, &constraint_chirho] {
        let images_chirho = make_images_chirho(&[
            (
                &function_chirho,
                Some(TyChirho::fun_chirho(
                    expected_chirho.clone(),
                    ty_chirho.clone(),
                )),
            ),
            (&argument_chirho, Some(ty_chirho.clone())),
        ]);
        let (result_chirho, equations_chirho) = project_chirho(
            &TyChirho::AppChirho(
                Box::new(TyChirho::ForallVarChirho(function_chirho.clone())),
                Box::new(TyChirho::ForallVarChirho(argument_chirho.clone())),
            ),
            Some(&images_chirho),
            &SubstChirho::empty_chirho(),
            4096,
        );
        assert_eq!(result_chirho, Ok(Some(ty_chirho.clone())));
        if expected_chirho == &ty_chirho {
            assert!(equations_chirho.is_empty());
        } else {
            assert_eq!(
                equations_chirho,
                vec![(constraint_chirho.clone(), ty_chirho.clone())]
            );
        }
    }
}

#[test]
fn rigid_lookup_consumes_the_original_bounded_allowance_chirho() {
    let name_chirho = crate::skolem_chirho::skolem_name_chirho("BoundedChirho", 17);
    let images_chirho = make_images_chirho(&[(
        &name_chirho,
        Some(ElaborationStateChirho::type_universe_chirho()),
    )]);
    let (result_chirho, equations_chirho) = project_chirho(
        &TyChirho::ForallVarChirho(name_chirho),
        Some(&images_chirho),
        &SubstChirho::empty_chirho(),
        2,
    );
    assert_eq!(result_chirho, Err(()));
    assert!(equations_chirho.is_empty());
}
