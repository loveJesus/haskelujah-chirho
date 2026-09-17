// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! GHC9.14.1 boot-class agreement observations, exercised through both filesystem APIs.
#[path = "boot_classes_chirho/associated_cases_chirho.rs"]
mod associated_cases_chirho;
#[path = "boot_classes_chirho/boundary_cases_chirho.rs"]
mod boundary_cases_chirho;
#[path = "boot_classes_chirho/head_cases_chirho.rs"]
mod head_cases_chirho;
#[path = "boot_classes_chirho/injectivity_cases_chirho.rs"]
mod injectivity_cases_chirho;
#[path = "boot_classes_chirho/method_cases_chirho.rs"]
mod method_cases_chirho;

struct ClassCaseChirho {
    name_chirho: &'static str,
    sources_chirho: &'static [(&'static str, &'static str)],
    accepted_chirho: bool,
    reason_chirho: &'static str,
}
fn reference_chirho(name_chirho: &str) {
    let record_chirho = head_cases_chirho::CASES_CHIRHO
        .iter()
        .chain(method_cases_chirho::CASES_CHIRHO)
        .chain(associated_cases_chirho::CASES_CHIRHO)
        .chain(boundary_cases_chirho::CASES_CHIRHO)
        .chain(injectivity_cases_chirho::CASES_CHIRHO)
        .find(|record_chirho| record_chirho.name_chirho == name_chirho)
        .expect("named reference");
    for result_chirho in
        super::boot_chirho::source_graph_paths_chirho(record_chirho.sources_chirho, "AuxChirho.hs")
    {
        if record_chirho.accepted_chirho {
            result_chirho.unwrap_or_else(|error_chirho| panic!("{name_chirho}: {error_chirho}"));
        } else {
            let error_chirho = result_chirho.expect_err(name_chirho);
            assert!(
                error_chirho.contains(record_chirho.reason_chirho)
                    && !error_chirho.contains("not represented"),
                "{name_chirho}: expected {} agreement, got {error_chirho}",
                record_chirho.reason_chirho
            );
        }
    }
}
macro_rules! class_tests_chirho {
    ($( $test_chirho:ident => $reference_chirho:literal ),+ $(,)?) => {
        $(#[test] fn $test_chirho() { reference_chirho($reference_chirho); })+
    };
}
class_tests_chirho!(
    boot_class_j1_unknown_rhs_variable_chirho => "J1_unknown_rhs_variable",
    boot_class_j2_wrong_lhs_variable_chirho => "J2_wrong_lhs_variable",
    boot_class_j3_duplicate_position_chirho => "J3_duplicate_position",
    boot_class_j4_valid_control_chirho => "J4_valid_control",
    boot_class_j5_positions_order_swapped_chirho => "J5_positions_order_swapped",
    boot_class_j6_duplicate_vs_single_chirho => "J6_duplicate_vs_single",
    boot_class_j7_identical_two_control_chirho => "J7_identical_two_control",
    boot_class_j8_subset_negative_chirho => "J8_subset_negative",
    boot_class_v1_boot_data_family_impl_type_family_chirho => "V1_boot_data_family_impl_type_family",
    boot_class_v2_boot_type_family_impl_data_family_chirho => "V2_boot_type_family_impl_data_family",
    boot_class_v3_data_family_both_explicit_control_chirho => "V3_data_family_both_explicit_control",
    boot_class_v4_data_family_both_omitted_spelling_control_chirho => "V4_data_family_both_omitted_spelling_control",
    boot_class_v5_data_family_mixed_spellings_chirho => "V5_data_family_mixed_spellings",
    boot_class_v6_method_a_to_b_vs_b_to_a_no_fundep_chirho => "V6_method_a_to_b_vs_b_to_a_no_fundep",
    boot_class_v7_method_alpha_renamed_control_chirho => "V7_method_alpha_renamed_control",
    boot_class_n1_nocontext_method_boot_impl_no_method_chirho => "N1_nocontext_method_boot_impl_no_method",
    boot_class_n2_nocontext_method_boot_impl_method_retyped_chirho => "N2_nocontext_method_boot_impl_method_retyped",
    boot_class_n3_nocontext_method_boot_impl_extra_method_chirho => "N3_nocontext_method_boot_impl_extra_method",
    boot_class_n4_nocontext_method_boot_impl_same_control_chirho => "N4_nocontext_method_boot_impl_same_control",
    boot_class_n8_nocontext_method_boot_impl_adds_superclass_chirho => "N8_nocontext_method_boot_impl_adds_superclass",
    boot_class_n5_nocontext_family_boot_impl_no_family_chirho => "N5_nocontext_family_boot_impl_no_family",
    boot_class_n6_nocontext_family_boot_impl_family_rekinded_chirho => "N6_nocontext_family_boot_impl_family_rekinded",
    boot_class_n7_nocontext_family_boot_impl_same_control_chirho => "N7_nocontext_family_boot_impl_same_control",
    boot_class_n9_nocontext_family_boot_impl_adds_superclass_chirho => "N9_nocontext_family_boot_impl_adds_superclass",
    boot_class_r1_assoc_family_order_swapped_chirho => "R1_assoc_family_order_swapped",
    boot_class_r2_assoc_family_order_same_control_chirho => "R2_assoc_family_order_same_control",
    boot_class_r3_params_alpha_renamed_same_association_chirho => "R3_params_alpha_renamed_same_association",
    boot_class_r4_params_association_swapped_chirho => "R4_params_association_swapped",
    boot_class_r5_default_only_without_family_head_chirho => "R5_default_only_without_family_head",
    boot_class_r6_default_spelling_type_vs_type_instance_chirho => "R6_default_spelling_type_vs_type_instance",
    boot_class_r7_order_swapped_different_kinds_chirho => "R7_order_swapped_different_kinds",
    boot_class_r8_names_in_place_kinds_exchanged_chirho => "R8_names_in_place_kinds_exchanged",
    boot_class_r9_association_swapped_different_param_kinds_chirho => "R9_association_swapped_different_param_kinds",
    boot_class_c0_t20661_verbatim_chirho => "C0_T20661_verbatim",
    boot_class_c1_abstract_boot_with_fundep_and_instance_chirho => "C1_abstract_boot_with_fundep_and_instance",
    boot_class_c2_concrete_empty_boot_impl_adds_method_chirho => "C2_concrete_empty_boot_impl_adds_method",
    boot_class_c3_abstract_boot_impl_adds_method_chirho => "C3_abstract_boot_impl_adds_method",
    boot_class_c4_concrete_boot_fundep_impl_without_chirho => "C4_concrete_boot_fundep_impl_without",
    boot_class_c5_concrete_boot_impl_adds_superclass_chirho => "C5_concrete_boot_impl_adds_superclass",
    boot_class_c6_abstract_boot_impl_superclass_and_method_chirho => "C6_abstract_boot_impl_superclass_and_method",
    boot_class_m0_t20588d_verbatim_chirho => "M0_T20588d_verbatim",
    boot_class_m1_method_type_differs_chirho => "M1_method_type_differs",
    boot_class_m2_impl_extra_method_chirho => "M2_impl_extra_method",
    boot_class_m3_boot_default_impl_none_chirho => "M3_boot_default_impl_none",
    boot_class_m4_boot_none_impl_default_chirho => "M4_boot_none_impl_default",
    boot_class_m5_minimal_boot_only_chirho => "M5_minimal_boot_only",
    boot_class_m6_minimal_both_chirho => "M6_minimal_both",
    boot_class_a1_assoc_default_boot_only_chirho => "A1_assoc_default_boot_only",
    boot_class_a2_assoc_default_differs_chirho => "A2_assoc_default_differs",
    boot_class_a3_assoc_family_impl_only_chirho => "A3_assoc_family_impl_only",
    boot_class_a4_assoc_family_no_default_both_chirho => "A4_assoc_family_no_default_both",
    boot_class_k1_concrete_empty_annotated_impl_adds_method_chirho => "K1_concrete_empty_annotated_impl_adds_method",
    boot_class_k2_abstract_annotated_impl_adds_method_chirho => "K2_abstract_annotated_impl_adds_method",
    boot_class_k3_abstract_annotated_impl_superclass_method_chirho => "K3_abstract_annotated_impl_superclass_method",
    boot_class_k4_concrete_empty_annotated_impl_superclass_chirho => "K4_concrete_empty_annotated_impl_superclass",
    boot_class_k5_abstract_fundep_boot_impl_no_fundep_chirho => "K5_abstract_fundep_boot_impl_no_fundep",
    boot_class_k6_abstract_no_fundep_boot_impl_fundep_chirho => "K6_abstract_no_fundep_boot_impl_fundep",
    boot_class_k7_abstract_annotated_instance_of_abstract_chirho => "K7_abstract_annotated_instance_of_abstract",
    boot_class_k8_abstract_annotated_instance_missing_method_chirho => "K8_abstract_annotated_instance_missing_method",
    boot_class_o1_method_order_swapped_chirho => "O1_method_order_swapped",
    boot_class_o2_method_order_same_control_chirho => "O2_method_order_same_control",
    boot_class_o3_superclass_order_swapped_chirho => "O3_superclass_order_swapped",
    boot_class_o4_superclass_order_same_control_chirho => "O4_superclass_order_same_control",
    boot_class_d1_default_body_text_differs_chirho => "D1_default_body_text_differs",
    boot_class_d2_default_body_semantically_different_chirho => "D2_default_body_semantically_different",
    boot_class_d3_default_signature_differs_chirho => "D3_default_signature_differs",
    boot_class_d4_default_signature_same_control_chirho => "D4_default_signature_same_control",
    boot_class_w1_empty_where_no_context_impl_adds_method_chirho => "W1_empty_where_no_context_impl_adds_method",
    boot_class_w2_empty_where_explicit_context_impl_adds_method_chirho => "W2_empty_where_explicit_context_impl_adds_method",
    boot_class_w3_empty_where_vs_absent_body_both_empty_chirho => "W3_empty_where_vs_absent_body_both_empty",
    boot_class_w4_absent_body_no_context_impl_adds_method_control_chirho => "W4_absent_body_no_context_impl_adds_method_control",
    boot_class_f1_abstract_boot_impl_adds_assoc_family_chirho => "F1_abstract_boot_impl_adds_assoc_family",
    boot_class_f2_abstract_boot_impl_adds_assoc_family_with_default_chirho => "F2_abstract_boot_impl_adds_assoc_family_with_default",
    boot_class_f3_concrete_empty_boot_impl_adds_assoc_family_chirho => "F3_concrete_empty_boot_impl_adds_assoc_family",
    boot_class_q1_boot_minimal_weaker_than_impl_chirho => "Q1_boot_minimal_weaker_than_impl",
    boot_class_q2_boot_minimal_stronger_disjunction_chirho => "Q2_boot_minimal_stronger_disjunction",
    boot_class_q3_boot_minimal_stronger_conjunction_chirho => "Q3_boot_minimal_stronger_conjunction",
    boot_class_q4_minimal_equal_disjunction_control_chirho => "Q4_minimal_equal_disjunction_control",
);
