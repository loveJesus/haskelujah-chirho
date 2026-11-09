// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! End-to-end extension controls with bounded topic leaves.

use crate::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

#[path = "extensions_chirho/legacy_chirho/bindings_control_flow_chirho.rs"]
mod bindings_control_flow_chirho;
#[path = "extensions_chirho/legacy_chirho/builtin_values_chirho.rs"]
mod builtin_values_chirho;
mod corpus_kind_transport_chirho;
#[path = "extensions_chirho/policy_chirho/deferral_flags_chirho.rs"]
mod deferral_flags_chirho;
#[path = "extensions_chirho/policy_chirho/deriving_policy_chirho.rs"]
mod deriving_policy_chirho;
#[path = "extensions_chirho/policy_chirho/deriving_strategies_chirho.rs"]
mod deriving_strategies_chirho;
mod expression_annotations_chirho;
#[path = "extensions_chirho/policy_chirho/gnd_coercion_chirho.rs"]
mod gnd_coercion_chirho;
mod impredicative_application_chirho;
#[path = "extensions_chirho/legacy_chirho/interfaces_chirho.rs"]
mod interfaces_chirho;
#[path = "extensions_chirho/legacy_chirho/kind_declarations_chirho.rs"]
mod kind_declarations_chirho;
mod kind_import_transport_chirho;
mod kind_qualified_context_chirho;
mod kind_repair_chirho;
#[path = "extensions_chirho/legacy_chirho/language_policy_chirho.rs"]
mod language_policy_chirho;
mod ordinary_elaboration_chirho;
mod qualified_do_chirho;
mod recursive_do_chirho;
mod required_type_arguments_chirho;
#[path = "extensions_chirho/policy_chirho/role_annotation_checks_chirho.rs"]
mod role_annotation_checks_chirho;
#[path = "extensions_chirho/policy_chirho/role_surface_checks_chirho.rs"]
mod role_surface_checks_chirho;
#[path = "extensions_chirho/policy_chirho/source_unit_authority_chirho.rs"]
mod source_unit_authority_chirho;
#[path = "extensions_chirho/legacy_chirho/syntax_chirho.rs"]
mod syntax_chirho;
mod type_level_character_families_chirho;
#[path = "extensions_chirho/policy_chirho/typed_hole_policy_chirho.rs"]
mod typed_hole_policy_chirho;

#[path = "extensions_chirho/policy_chirho/quoted_patterns_chirho.rs"]
mod quoted_patterns_chirho;

#[path = "extensions_chirho/legacy_chirho/transform_comprehensions_chirho.rs"]
mod transform_comprehensions_chirho;
