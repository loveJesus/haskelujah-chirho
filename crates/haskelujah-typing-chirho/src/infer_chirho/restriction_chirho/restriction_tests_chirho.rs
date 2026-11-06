// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The monomorphism restriction's own decisions (row 692). The programs it changes are pinned
//! on every engine by the driver's `monomorphism_restriction_chirho` controls.

use haskelujah_ast_chirho::name_chirho::CompilerClassChirho;

use super::{
    RestrictionStateChirho, effective_flag_chirho, group_is_restricted_chirho,
    numeric_class_chirho, restriction_enabled_chirho, standard_class_chirho,
};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};

fn flags_chirho(flags_chirho: &[&str]) -> Vec<String> {
    flags_chirho
        .iter()
        .map(|flag_chirho| flag_chirho.to_string())
        .collect()
}

#[test]
fn the_restriction_is_on_unless_its_last_flag_turns_it_off_chirho() {
    assert!(restriction_enabled_chirho(&[]));
    assert!(restriction_enabled_chirho(&flags_chirho(&["GADTs"])));
    assert!(!restriction_enabled_chirho(&flags_chirho(&[
        "NoMonomorphismRestriction"
    ])));
    assert!(restriction_enabled_chirho(&flags_chirho(&[
        "NoMonomorphismRestriction",
        "MonomorphismRestriction",
    ])));
    assert!(!restriction_enabled_chirho(&flags_chirho(&[
        "MonomorphismRestriction",
        "NoMonomorphismRestriction",
    ])));
}

#[test]
fn a_literal_safety_flag_follows_its_last_setting_chirho() {
    // The literal producer mints compiler proof only when neither flag is in effect.
    assert!(!effective_flag_chirho(&[], "RebindableSyntax", false));
    assert!(effective_flag_chirho(
        &flags_chirho(&["RebindableSyntax"]),
        "RebindableSyntax",
        false
    ));
    assert!(!effective_flag_chirho(
        &flags_chirho(&["RebindableSyntax", "NoRebindableSyntax"]),
        "RebindableSyntax",
        false
    ));
    assert!(effective_flag_chirho(
        &flags_chirho(&["NoMagicHash", "MagicHash"]),
        "MagicHash",
        false
    ));
    // Another extension that merely starts with the name is not the flag.
    assert!(!effective_flag_chirho(
        &flags_chirho(&["MagicHashChirho"]),
        "MagicHash",
        false
    ));
}

#[test]
fn only_an_unsigned_argument_less_member_restricts_its_group_chirho() {
    // (argument-less, signed) per member.
    assert!(group_is_restricted_chirho([(true, false)]));
    assert!(!group_is_restricted_chirho([(true, true)]));
    assert!(!group_is_restricted_chirho([(false, false)]));
    // A function-syntax member shares its group's restriction.
    assert!(group_is_restricted_chirho([(false, false), (true, false)]));
    assert!(!group_is_restricted_chirho([(false, false), (true, true)]));
    assert!(!group_is_restricted_chirho(Vec::<(bool, bool)>::new()));
}

#[test]
fn only_standard_classes_with_a_numeric_one_default_chirho() {
    for numeric_chirho in [
        CompilerClassChirho::NumChirho,
        CompilerClassChirho::RealChirho,
        CompilerClassChirho::IntegralChirho,
        CompilerClassChirho::FractionalChirho,
        CompilerClassChirho::FloatingChirho,
        CompilerClassChirho::RealFracChirho,
        CompilerClassChirho::RealFloatChirho,
    ] {
        assert!(numeric_class_chirho(numeric_chirho), "{numeric_chirho:?}");
        assert!(standard_class_chirho(numeric_chirho), "{numeric_chirho:?}");
    }
    for standard_chirho in [
        CompilerClassChirho::ShowChirho,
        CompilerClassChirho::EqChirho,
        CompilerClassChirho::MonadChirho,
        CompilerClassChirho::MonoidChirho,
    ] {
        assert!(
            standard_class_chirho(standard_chirho),
            "{standard_chirho:?}"
        );
        assert!(
            !numeric_class_chirho(standard_chirho),
            "{standard_chirho:?}"
        );
    }
    for outside_chirho in [
        CompilerClassChirho::NfDataChirho,
        CompilerClassChirho::TypeableChirho,
        CompilerClassChirho::BitsChirho,
        CompilerClassChirho::MonadIoChirho,
    ] {
        assert!(!standard_class_chirho(outside_chirho), "{outside_chirho:?}");
    }
}

#[test]
fn a_withheld_variable_follows_every_substitution_chirho() {
    let mut state_chirho = RestrictionStateChirho::default();
    let restricted_chirho = TyVarChirho(7_001);
    let image_chirho = TyVarChirho(7_002);
    state_chirho.withhold_chirho([restricted_chirho]);

    // Unified with another variable: that variable is now the restricted one.
    let mut renamed_chirho = SubstChirho::empty_chirho();
    renamed_chirho.insert_chirho(restricted_chirho, TyChirho::VarChirho(image_chirho));
    state_chirho.follow_substitution_chirho(&renamed_chirho);
    assert_eq!(
        state_chirho
            .withheld_chirho()
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        vec![image_chirho]
    );

    // Fixed by a use: nothing is left to default.
    let mut fixed_chirho = SubstChirho::empty_chirho();
    fixed_chirho.insert_chirho(image_chirho, TyChirho::int_chirho());
    state_chirho.follow_substitution_chirho(&fixed_chirho);
    assert!(state_chirho.withheld_chirho().is_empty());
}

#[test]
fn a_restricted_group_withholds_only_the_residual_variables_it_created_chirho() {
    let mut context_chirho = crate::infer_chirho::InferCtxChirho::new_chirho();
    let earlier_chirho = context_chirho.fresh_var_chirho();
    let mark_chirho = context_chirho.group_variable_mark_chirho();
    let read_chirho = context_chirho.fresh_var_chirho();
    // A later variable of the group that no wanted mentions.
    let _unwanted_chirho = context_chirho.fresh_var_chirho();
    for (class_chirho, ty_chirho) in [
        (CompilerClassChirho::ReadChirho, read_chirho.clone()),
        (CompilerClassChirho::ShowChirho, read_chirho.clone()),
        (CompilerClassChirho::NumChirho, earlier_chirho.clone()),
    ] {
        context_chirho.deferred_preds_chirho.push((
            crate::class_chirho::PredChirho::compiler_chirho(class_chirho, ty_chirho),
            haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
        ));
    }
    context_chirho.withhold_group_residuals_chirho(mark_chirho);
    let TyChirho::VarChirho(read_variable_chirho) = read_chirho else {
        panic!("a fresh variable");
    };
    // Only the group's own residual variable: not an earlier group's, nor one with no wanted.
    assert_eq!(
        context_chirho
            .restriction_chirho
            .withheld_chirho()
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        vec![read_variable_chirho]
    );
}
