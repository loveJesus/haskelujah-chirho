// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # CST → AST lowering
//!
//! Walks the immutable green tree produced by the CST parser and builds
//! the typed AST (`ModuleChirho`). Trivia, virtual layout tokens, and
//! punctuation are discarded; only semantic content survives.
//!
//! The lowering is split by category, one directory each under `lower_chirho/` (row 641):
//! module items, declarations, data types, classes, bindings, expression syntax, type syntax,
//! patterns and occurrences, with the context helpers and the shared test support beside them.
//! This file holds the public shell: `lower_module_chirho`, `LowerCtxChirho`, the literal
//! helpers, and the three items the child modules name as `super::`.

#[cfg(test)]
#[path = "lower_chirho/classes_chirho/associated_instance_tests_chirho.rs"]
mod associated_instance_tests_chirho;
#[cfg(test)]
#[path = "lower_chirho/bindings_chirho/binding_tests_chirho.rs"]
mod binding_tests_chirho;
#[path = "lower_chirho/bindings_chirho/bindings_chirho.rs"]
mod bindings_chirho;
#[path = "lower_chirho/expr_syntax_chirho/bracketed_exprs_chirho.rs"]
mod bracketed_exprs_chirho;
#[cfg(test)]
#[path = "lower_chirho/classes_chirho/class_tests_chirho.rs"]
mod class_tests_chirho;
#[path = "lower_chirho/classes_chirho/classes_chirho.rs"]
mod classes_chirho;
mod context_chirho;
#[cfg(test)]
#[path = "lower_chirho/declarations_chirho/context_tests_chirho.rs"]
mod context_tests_chirho;
#[path = "lower_chirho/declarations_chirho/contexts_chirho.rs"]
mod contexts_chirho;
#[cfg(test)]
#[path = "lower_chirho/data_types_chirho/data_type_tests_chirho.rs"]
mod data_type_tests_chirho;
#[path = "lower_chirho/data_types_chirho/data_types_chirho.rs"]
mod data_types_chirho;
#[path = "lower_chirho/type_syntax_chirho/declaration_heads_chirho.rs"]
mod declaration_heads_chirho;
#[cfg(test)]
#[path = "lower_chirho/declarations_chirho/declaration_kind_tests_chirho.rs"]
mod declaration_kind_tests_chirho;
#[path = "lower_chirho/declarations_chirho/declaration_kinds_chirho.rs"]
mod declaration_kinds_chirho;
#[cfg(test)]
#[path = "lower_chirho/declarations_chirho/declaration_tests_chirho.rs"]
mod declaration_tests_chirho;
#[path = "lower_chirho/declarations_chirho/declarations_chirho.rs"]
mod declarations_chirho;
#[path = "lower_chirho/bindings_chirho/do_selection_chirho.rs"]
mod do_selection_chirho;
#[cfg(test)]
#[path = "lower_chirho/bindings_chirho/do_selection_tests_chirho.rs"]
mod do_selection_tests_chirho;
#[path = "lower_chirho/expr_syntax_chirho/expression_parts_chirho.rs"]
mod expression_parts_chirho;
#[cfg(test)]
#[path = "lower_chirho/expr_syntax_chirho/expression_tests_chirho.rs"]
mod expression_tests_chirho;
#[path = "lower_chirho/expr_syntax_chirho/expressions_chirho.rs"]
mod expressions_chirho;
#[cfg(test)]
#[path = "lower_chirho/classes_chirho/family_equation_tests_chirho.rs"]
mod family_equation_tests_chirho;
#[path = "lower_chirho/classes_chirho/family_equations_chirho.rs"]
mod family_equations_chirho;
#[path = "lower_chirho/expr_syntax_chirho/fixity_chirho.rs"]
mod fixity_chirho;
#[cfg(test)]
#[path = "lower_chirho/type_syntax_chirho/flat_type_tests_chirho.rs"]
mod flat_type_tests_chirho;
#[path = "lower_chirho/type_syntax_chirho/flat_types_chirho.rs"]
mod flat_types_chirho;
#[path = "lower_chirho/declarations_chirho/foreign_chirho.rs"]
mod foreign_chirho;
#[cfg(test)]
#[path = "lower_chirho/bindings_chirho/guard_do_tests_chirho.rs"]
mod guard_do_tests_chirho;
#[cfg(test)]
#[path = "lower_chirho/module_items_chirho/import_item_tests_chirho.rs"]
mod import_item_tests_chirho;
#[cfg(test)]
#[path = "lower_chirho/classes_chirho/instance_signature_tests_chirho.rs"]
mod instance_signature_tests_chirho;
#[cfg(test)]
#[path = "lower_chirho/classes_chirho/instance_tests_chirho.rs"]
mod instance_tests_chirho;
#[cfg(test)]
#[path = "lower_chirho/type_syntax_chirho/kind_tests_chirho.rs"]
mod kind_tests_chirho;
#[cfg(test)]
#[path = "lower_chirho/module_items_chirho/module_item_tests_chirho.rs"]
mod module_item_tests_chirho;
#[path = "lower_chirho/module_items_chirho/module_items_chirho.rs"]
mod module_items_chirho;
#[cfg(test)]
#[path = "lower_chirho/occurrences_chirho/occurrence_tests_chirho.rs"]
mod occurrence_tests_chirho;
#[path = "lower_chirho/occurrences_chirho/occurrences_chirho.rs"]
mod occurrences_chirho;
#[cfg(test)]
#[path = "lower_chirho/patterns_chirho/pattern_tests_chirho.rs"]
mod pattern_tests_chirho;
#[path = "lower_chirho/patterns_chirho/patterns_chirho.rs"]
mod patterns_chirho;
#[path = "lower_chirho/expr_syntax_chirho/prefix_minus_chirho.rs"]
mod prefix_minus_chirho;
#[cfg(test)]
#[path = "lower_chirho/expr_syntax_chirho/prefix_minus_tests_chirho.rs"]
mod prefix_minus_tests_chirho;
#[cfg(test)]
#[path = "lower_chirho/expr_syntax_chirho/qualified_sections_tests_chirho.rs"]
mod qualified_sections_tests_chirho;
#[path = "lower_chirho/patterns_chirho/record_patterns_chirho.rs"]
mod record_patterns_chirho;
#[cfg(test)]
mod test_support_chirho;
#[path = "lower_chirho/classes_chirho/type_families_chirho.rs"]
mod type_families_chirho;
#[cfg(test)]
#[path = "lower_chirho/type_syntax_chirho/type_operator_tests_chirho.rs"]
mod type_operator_tests_chirho;
#[path = "lower_chirho/type_syntax_chirho/type_operators_chirho.rs"]
mod type_operators_chirho;
#[cfg(test)]
#[path = "lower_chirho/type_syntax_chirho/type_tests_chirho.rs"]
mod type_tests_chirho;
#[path = "lower_chirho/type_syntax_chirho/types_chirho.rs"]
mod types_chirho;
#[cfg(test)]
#[path = "lower_chirho/occurrences_chirho/walk_hook_tests_chirho.rs"]
mod walk_hook_tests_chirho;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use haskelujah_ast_chirho::decl_chirho::{DataKindSigChirho, DeclChirho, FixityChirho};
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::provenance_chirho::OriginSupplyChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::{FileIdChirho, SpanChirho};
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{
    GreenElementChirho, GreenNodeChirho, GreenTokenChirho,
};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use context_chirho::is_pat_kind_chirho;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Lower a green CST root into a typed AST module.
pub fn lower_module_chirho(
    root_chirho: &Arc<GreenNodeChirho>,
    file_id_chirho: FileIdChirho,
) -> ModuleChirho {
    let mut ctx_chirho = LowerCtxChirho::new_chirho(file_id_chirho);
    ctx_chirho.lower_source_file_chirho(root_chirho)
}

// ---------------------------------------------------------------------------
// LowerCtxChirho — lowering context that tracks byte offset
// ---------------------------------------------------------------------------

struct LowerCtxChirho {
    file_id_chirho: FileIdChirho,
    offset_chirho: usize,
    fixity_overrides_chirho: HashMap<String, (u8, FixityChirho)>,
    /// The module's origin supply while lowering builds it. Expression lowering
    /// borrows the context immutably, so the supply sits in a cell.
    origin_supply_chirho: RefCell<OriginSupplyChirho>,
}

/// Parse an unsigned integer-literal magnitude that may have a hex (0x/0X),
/// octal (0o/0O), or binary (0b/0B) prefix.
fn parse_integer_magnitude_chirho(text_chirho: &str) -> Option<u64> {
    // Strip MagicHash trailing # and NumericUnderscores
    let clean_chirho: String = text_chirho
        .trim_end_matches('#')
        .chars()
        .filter(|c_chirho| *c_chirho != '_')
        .collect();
    let text_chirho = &clean_chirho;
    if let Some(hex_chirho) = text_chirho
        .strip_prefix("0x")
        .or_else(|| text_chirho.strip_prefix("0X"))
    {
        u64::from_str_radix(hex_chirho, 16).ok()
    } else if let Some(octal_chirho) = text_chirho
        .strip_prefix("0o")
        .or_else(|| text_chirho.strip_prefix("0O"))
    {
        u64::from_str_radix(octal_chirho, 8).ok()
    } else if let Some(binary_chirho) = text_chirho
        .strip_prefix("0b")
        .or_else(|| text_chirho.strip_prefix("0B"))
    {
        u64::from_str_radix(binary_chirho, 2).ok()
    } else {
        text_chirho.parse::<u64>().ok()
    }
}

/// Parse a positive machine-word integer literal. The one extra magnitude that
/// prefix minus can represent is preserved by `preserve_minimum_word_operand_chirho`.
fn parse_integer_literal_chirho(text_chirho: &str) -> i64 {
    parse_integer_magnitude_chirho(text_chirho)
        .and_then(|magnitude_chirho| i64::try_from(magnitude_chirho).ok())
        .unwrap_or(0)
}

struct ChildChirho<'a> {
    element_chirho: &'a GreenElementChirho,
    start_chirho: usize,
    end_chirho: usize,
}

fn is_type_kind_chirho(kind_chirho: SyntaxKindChirho) -> bool {
    matches!(
        kind_chirho,
        SyntaxKindChirho::FunTypeChirho
            | SyntaxKindChirho::AppTypeChirho
            | SyntaxKindChirho::ParenTypeChirho
            | SyntaxKindChirho::TupleTypeChirho
            | SyntaxKindChirho::ListTypeChirho
            | SyntaxKindChirho::VarTypeChirho
            | SyntaxKindChirho::ConTypeChirho
            | SyntaxKindChirho::QualTypeChirho
            | SyntaxKindChirho::ForallTypeChirho
            | SyntaxKindChirho::KindAnnotTypeChirho
            | SyntaxKindChirho::ContextChirho
            | SyntaxKindChirho::PromotedConTypeChirho
            | SyntaxKindChirho::PromotedListTypeChirho
            | SyntaxKindChirho::PromotedTupleTypeChirho
            | SyntaxKindChirho::InfixTypeChirho
            | SyntaxKindChirho::WildcardTypeChirho
            | SyntaxKindChirho::LitTypeChirho
    )
}

/// Associativity of an operator.
#[derive(Clone, Copy, PartialEq)]
#[allow(
    clippy::enum_variant_names,
    reason = "every variant carries the project's Chirho suffix"
)]
enum AssocChirho {
    LeftChirho,
    RightChirho,
    NoneChirho,
}
