// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The S3 join pairs a typed instance with its rows only when each producer reports exactly
//! one thing at the declaration's class and span. A shared span with different heads stays
//! ambiguous, by name, and never becomes a choice.

use super::instance_rows_chirho::{
    InstanceDeclarationChirho, InstanceRowKeyChirho, InstanceRowsChirho, RowKeyAbsenceChirho,
};
use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho, SpanChirho};
use haskelujah_typing_chirho::class_chirho::{InstDeclChirho, PredChirho};
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

fn span_chirho(start_chirho: u32) -> SpanChirho {
    SpanChirho::new_chirho(
        FileIdChirho::SYNTHETIC_CHIRHO,
        ByteOffsetChirho::new_chirho(start_chirho),
        ByteOffsetChirho::new_chirho(start_chirho + 10),
    )
}

/// `instance SizeChirho a => SizeChirho (<head> a)` over the freshened variable `var`.
fn context_instance_chirho(head_chirho: &str, var_chirho: u32) -> InstDeclChirho {
    let a_chirho = TyChirho::VarChirho(TyVarChirho(var_chirho));
    InstDeclChirho::legacy_chirho(
        haskelujah_typing_chirho::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "SizeChirho".to_string(),
            head_ty_chirho: TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho(head_chirho.to_string())),
                Box::new(a_chirho.clone()),
            ),
            extra_head_tys_chirho: Vec::new(),
            context_chirho: vec![PredChirho::new_chirho("SizeChirho", a_chirho)],
        },
    )
}

fn declared_chirho(
    instance_chirho: &InstDeclChirho,
    start_chirho: u32,
) -> InstanceDeclarationChirho {
    InstanceDeclarationChirho {
        span_chirho: span_chirho(start_chirho),
        instance_chirho: std::sync::Arc::new(instance_chirho.clone()),
    }
}

fn row_key_chirho(row_key_chirho: &str, start_chirho: u32) -> InstanceRowKeyChirho {
    InstanceRowKeyChirho {
        class_name_chirho: "SizeChirho".to_string(),
        span_chirho: span_chirho(start_chirho),
        row_key_chirho: row_key_chirho.to_string(),
    }
}

#[test]
fn one_instance_and_one_key_at_a_place_join_chirho() {
    let maybe_chirho = context_instance_chirho("Maybe", 9001);
    let rows_chirho = InstanceRowsChirho::join_chirho(
        &[declared_chirho(&maybe_chirho, 100)],
        &[row_key_chirho("Maybe aChirho", 100)],
    );
    assert_eq!(
        rows_chirho.row_key_chirho(&maybe_chirho),
        Ok("Maybe aChirho")
    );
    assert_eq!(rows_chirho.proven_rows_chirho().count(), 1);
}

#[test]
fn a_shared_span_with_different_heads_is_ambiguous_while_a_normal_place_joins_chirho() {
    let maybe_chirho = context_instance_chirho("Maybe", 9001);
    let box_chirho = context_instance_chirho("BoxChirho", 9002);
    let list_chirho = context_instance_chirho("ListChirho", 9003);
    let rows_chirho = InstanceRowsChirho::join_chirho(
        &[
            declared_chirho(&maybe_chirho, 0),
            declared_chirho(&box_chirho, 0),
            declared_chirho(&list_chirho, 200),
        ],
        &[
            row_key_chirho("Maybe aChirho", 0),
            row_key_chirho("BoxChirho aChirho", 0),
            row_key_chirho("ListChirho aChirho", 200),
        ],
    );
    assert_eq!(
        rows_chirho.row_key_chirho(&maybe_chirho),
        Err(RowKeyAbsenceChirho::AmbiguousChirho)
    );
    assert_eq!(
        rows_chirho.row_key_chirho(&box_chirho),
        Err(RowKeyAbsenceChirho::AmbiguousChirho)
    );
    assert_eq!(
        rows_chirho.row_key_chirho(&list_chirho),
        Ok("ListChirho aChirho")
    );
    let proven_chirho: Vec<&str> = rows_chirho
        .proven_rows_chirho()
        .map(|(_, row_key_chirho)| row_key_chirho)
        .collect();
    assert_eq!(proven_chirho, vec!["ListChirho aChirho"]);
}

#[test]
fn exact_repeats_of_one_object_and_one_key_deduplicate_chirho() {
    let maybe_chirho = context_instance_chirho("Maybe", 9001);
    let rows_chirho = InstanceRowsChirho::join_chirho(
        &[
            declared_chirho(&maybe_chirho, 100),
            declared_chirho(&maybe_chirho, 100),
        ],
        &[
            row_key_chirho("Maybe aChirho", 100),
            row_key_chirho("Maybe aChirho", 100),
        ],
    );
    assert_eq!(
        rows_chirho.row_key_chirho(&maybe_chirho),
        Ok("Maybe aChirho")
    );
}

#[test]
fn two_keys_at_one_instance_place_are_ambiguous_chirho() {
    let maybe_chirho = context_instance_chirho("Maybe", 9001);
    let rows_chirho = InstanceRowsChirho::join_chirho(
        &[declared_chirho(&maybe_chirho, 100)],
        &[
            row_key_chirho("Maybe aChirho", 100),
            row_key_chirho("Maybe bChirho", 100),
        ],
    );
    assert_eq!(
        rows_chirho.row_key_chirho(&maybe_chirho),
        Err(RowKeyAbsenceChirho::AmbiguousChirho)
    );
}

#[test]
fn an_instance_no_declaration_reports_with_rows_lives_elsewhere_chirho() {
    let maybe_chirho = context_instance_chirho("Maybe", 9001);
    let imported_chirho = context_instance_chirho("Maybe", 9004);
    let rows_chirho = InstanceRowsChirho::join_chirho(
        &[declared_chirho(&maybe_chirho, 100)],
        &[row_key_chirho("Maybe aChirho", 100)],
    );
    // Written alike, but another object: its variables were freshened apart.
    assert_eq!(
        rows_chirho.row_key_chirho(&imported_chirho),
        Err(RowKeyAbsenceChirho::ElsewhereChirho)
    );
}
