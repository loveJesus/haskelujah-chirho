// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! # Declaration validity
//!
//! GHC judges each declaration's written shape before using it. The type forms
//! in the parent module are one family of those checks; this child holds the
//! declaration-shape family (row 683). Every rule reads a shape that lowering
//! keeps:
//!
//! 1. a newtype constructor has exactly one field (`GHC-23517`);
//! 2. a GADT-syntax newtype constructor has no context (`GHC-17440`);
//! 3. one instance body binds a name once (`GHC-10498`);
//! 4. a default signature has a default binding (`GHC-59794`);
//! 5. a newtype constructor's field has no strictness annotation (`GHC-04049`);
//! 6. a lazy field annotation (`~T`) needs `StrictData` or `Strict` (`GHC-81601`);
//! 7. to 15. deriving strategies, the extensions they need and the classes they derive
//!    (`deriving_strategy_chirho.rs`, row 694).
//!
//! Rule 6 judges the written override only. The module-wide `StrictData`
//! default (unannotated fields strict) is not implemented: such fields stay
//! lazy, and no rule here depends on it.
//!
//! The driver runs it on the module as written, after splices and before
//! deriving appends generated instances, and reports it after name resolution.
//! Shapes that lowering drops are not judged here: a Haskell 98 constructor's
//! `forall` and context, a missing newtype constructor, and class-body bindings
//! without a signature wait on the AST
//! (`spec-chirho/bug-existential-constructor-scope-dropped-chirho.md`).
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use haskelujah_ast_chirho::decl_chirho::{
    ClassMethodChirho, ConDeclChirho, DeclChirho, StrictnessChirho,
};
use haskelujah_ast_chirho::expr_chirho::LocalBindChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{BuiltinClassScopeChirho, NameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_diagnostics_chirho::{
    DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho, LabelChirho,
};
use haskelujah_span_chirho::SpanChirho;

use super::ValidityResultChirho;

// Rules 7 to 15: deriving strategies and the classes they derive (row 694).
#[path = "deriving_strategy_chirho.rs"]
mod deriving_strategy_chirho;

/// `E0209`: a declaration whose written shape GHC rejects.
const INVALID_DECLARATION_CODE_CHIRHO: u16 = 209;

/// The code a rule reports for a shape GHC accepts that Haskelujah cannot compile yet, so its
/// diagnostic names the gap instead of a GHC code.
pub(crate) const UNSUPPORTED_SHAPE_CHIRHO: &str = "unsupported";

/// The code a rule reports while GHC's code for its error is still to be measured.
pub(crate) const UNMEASURED_GHC_CODE_CHIRHO: &str = "unmeasured";

/// A declaration that GHC rejects for its written shape, or one whose shape Haskelujah cannot
/// compile yet.
#[derive(Debug, Clone, PartialEq)]
pub struct DeclarationErrorChirho {
    /// GHC's code for the rule that fired, such as `GHC-23517`; `UNSUPPORTED_SHAPE_CHIRHO`
    /// for a shape GHC accepts, or `UNMEASURED_GHC_CODE_CHIRHO` while GHC's code is unknown.
    pub ghc_code_chirho: &'static str,
    /// Human-readable message, phrased like GHC's.
    pub message_chirho: String,
    /// The offending part of the declaration.
    pub span_chirho: SpanChirho,
    /// An earlier part the message refers to: the first of two bindings.
    pub related_span_chirho: Option<SpanChirho>,
}

/// Run the declaration rules and package the findings as diagnostics, with no builtin class
/// proof: a deriving rule judges every class by its written name.
/// workflow: compiler-pipeline-chirho/declaration-validity-chirho
pub fn check_module_declaration_validity_diagnostics_chirho(
    module_chirho: &ModuleChirho,
) -> ValidityResultChirho {
    check_module_declaration_validity_with_builtins_diagnostics_chirho(
        module_chirho,
        &BuiltinClassScopeChirho::default(),
    )
}

/// Run the declaration rules, judging derived classes with the builtin classes naming proved
/// for the module (K01), and package the findings as diagnostics.
/// workflow: compiler-pipeline-chirho/declaration-validity-chirho
pub fn check_module_declaration_validity_with_builtins_diagnostics_chirho(
    module_chirho: &ModuleChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
) -> ValidityResultChirho {
    ValidityResultChirho {
        diagnostics_chirho: declaration_error_diagnostics_chirho(
            check_module_declaration_validity_with_builtins_chirho(
                module_chirho,
                builtin_classes_chirho,
            ),
        ),
    }
}

/// Declaration errors as diagnostics: GHC's code in the note, or, for a shape Haskelujah
/// cannot compile yet or a code still to be measured, a note that says so. Deriving's GND
/// coercion proof reports its refusals through the same packaging (row 694).
/// workflow: compiler-pipeline-chirho/declaration-validity-chirho
pub(crate) fn declaration_error_diagnostics_chirho(
    errors_chirho: Vec<DeclarationErrorChirho>,
) -> DiagnosticBundleChirho {
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();
    for error_chirho in errors_chirho {
        let note_chirho = match error_chirho.ghc_code_chirho {
            UNSUPPORTED_SHAPE_CHIRHO => "Haskelujah does not support this yet".to_string(),
            UNMEASURED_GHC_CODE_CHIRHO => {
                "GHC's code for this error is not measured yet".to_string()
            }
            ghc_code_chirho => format!("GHC reports this as {ghc_code_chirho}"),
        };
        let mut diagnostic_chirho = DiagnosticChirho::error_with_code_chirho(
            ErrorCodeChirho::error_chirho(INVALID_DECLARATION_CODE_CHIRHO),
            error_chirho.message_chirho,
            error_chirho.span_chirho,
        )
        .with_note_chirho(note_chirho);
        if let Some(related_chirho) = error_chirho.related_span_chirho {
            diagnostic_chirho = diagnostic_chirho.with_label_chirho(LabelChirho::secondary_chirho(
                related_chirho,
                "first defined here",
            ));
        }
        diagnostics_chirho.push_chirho(diagnostic_chirho);
    }
    diagnostics_chirho
}

/// Every declaration-shape error in a module, in declaration order, with no builtin class
/// proof.
pub fn check_module_declaration_validity_chirho(
    module_chirho: &ModuleChirho,
) -> Vec<DeclarationErrorChirho> {
    check_module_declaration_validity_with_builtins_chirho(
        module_chirho,
        &BuiltinClassScopeChirho::default(),
    )
}

/// Every declaration-shape error in a module, in declaration order, judging derived classes
/// with the builtin classes naming proved for it.
pub fn check_module_declaration_validity_with_builtins_chirho(
    module_chirho: &ModuleChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
) -> Vec<DeclarationErrorChirho> {
    let mut errors_chirho = Vec::new();
    let strict_data_chirho = strict_data_enabled_chirho(module_chirho);
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::NewtypeDeclChirho {
                constructor_chirho, ..
            } => check_newtype_constructor_chirho(constructor_chirho, &mut errors_chirho),
            DeclChirho::DataDeclChirho {
                constructors_chirho,
                ..
            } if !strict_data_chirho => {
                check_lazy_annotations_chirho(constructors_chirho, &mut errors_chirho);
            }
            DeclChirho::InstanceDeclChirho { methods_chirho, .. } => {
                check_instance_bindings_chirho(methods_chirho, &mut errors_chirho);
            }
            DeclChirho::ClassDeclChirho {
                name_chirho,
                methods_chirho,
                ..
            } => check_default_signatures_chirho(name_chirho, methods_chirho, &mut errors_chirho),
            _ => {}
        }
    }
    deriving_strategy_chirho::check_deriving_strategies_chirho(
        module_chirho,
        builtin_classes_chirho,
        &mut errors_chirho,
    );
    errors_chirho
}

/// Rules 1, 2 and 5, GHC's `checkNewDataCon` on the shapes lowering keeps.
/// Rule 5 holds at any `StrictData` setting and covers `!`, UNPACK and the
/// explicit `~` alike; like GHC, it is judged only once rule 1 holds.
///
/// A Haskell 98 constructor is judged only when lowering found its name. The
/// empty name is lowering's placeholder for a constructor it could not read
/// (`newtype N = (:|) Int`, which GHC accepts, or none at all), and the fields
/// beside a placeholder mean nothing. A GADT-syntax constructor is judged for
/// its context only: its result type and record form still wait on the AST
/// (`spec-chirho/bug-gadt-record-fields-dropped-chirho.md`).
fn check_newtype_constructor_chirho(
    constructor_chirho: &ConDeclChirho,
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) {
    // A written annotation is anything but the unannotated default.
    let (name_chirho, field_count_chirho, annotated_chirho, span_chirho) = match constructor_chirho
    {
        ConDeclChirho::OrdinaryChirho {
            name_chirho,
            fields_chirho,
            span_chirho,
        } => (
            name_chirho,
            fields_chirho.len(),
            fields_chirho.iter().any(|(strictness_chirho, _ty_chirho)| {
                *strictness_chirho != StrictnessChirho::LazyChirho
            }),
            *span_chirho,
        ),
        ConDeclChirho::RecordChirho {
            name_chirho,
            fields_chirho,
            span_chirho,
        } => (
            name_chirho,
            fields_chirho
                .iter()
                .map(|field_chirho| field_chirho.names_chirho.len())
                .sum::<usize>(),
            fields_chirho
                .iter()
                .any(|field_chirho| field_chirho.strictness_chirho != StrictnessChirho::LazyChirho),
            *span_chirho,
        ),
        ConDeclChirho::GadtChirho {
            name_chirho,
            ty_chirho,
            span_chirho,
        } => {
            if signature_has_context_chirho(ty_chirho) {
                errors_chirho.push(DeclarationErrorChirho {
                    ghc_code_chirho: "GHC-17440",
                    message_chirho: format!(
                        "A newtype constructor must not have a context in its type: `{}`",
                        name_chirho.text_chirho()
                    ),
                    span_chirho: *span_chirho,
                    related_span_chirho: None,
                });
            }
            return;
        }
    };
    if name_chirho.text_chirho().is_empty() {
        return;
    }
    if field_count_chirho != 1 {
        errors_chirho.push(DeclarationErrorChirho {
            ghc_code_chirho: "GHC-23517",
            message_chirho: format!(
                "A newtype constructor must have exactly one field, but `{}` has {}",
                name_chirho.text_chirho(),
                spoken_count_chirho(field_count_chirho)
            ),
            span_chirho,
            related_span_chirho: None,
        });
    } else if annotated_chirho {
        errors_chirho.push(DeclarationErrorChirho {
            ghc_code_chirho: "GHC-04049",
            message_chirho: format!(
                "A newtype constructor must not have a strictness annotation, but `{}` has one",
                name_chirho.text_chirho()
            ),
            span_chirho,
            related_span_chirho: None,
        });
    }
}

/// Rule 6: a written lazy annotation (`~T`) on a data constructor's field needs
/// `StrictData`, or `Strict`, which implies it; without it GHC reports
/// `GHC-81601` at the constructor. The caller runs it only when `StrictData` is
/// off. This is the explicit override alone, not the `StrictData` default. A
/// GADT constructor's arguments carry no strictness in the AST and stay unjudged,
/// as does lowering's unnamed placeholder constructor.
fn check_lazy_annotations_chirho(
    constructors_chirho: &[ConDeclChirho],
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) {
    for constructor_chirho in constructors_chirho {
        let (name_chirho, lazy_chirho, span_chirho) = match constructor_chirho {
            ConDeclChirho::OrdinaryChirho {
                name_chirho,
                fields_chirho,
                span_chirho,
            } => (
                name_chirho,
                fields_chirho.iter().any(|(strictness_chirho, _ty_chirho)| {
                    *strictness_chirho == StrictnessChirho::ExplicitLazyChirho
                }),
                *span_chirho,
            ),
            ConDeclChirho::RecordChirho {
                name_chirho,
                fields_chirho,
                span_chirho,
            } => (
                name_chirho,
                fields_chirho.iter().any(|field_chirho| {
                    field_chirho.strictness_chirho == StrictnessChirho::ExplicitLazyChirho
                }),
                *span_chirho,
            ),
            ConDeclChirho::GadtChirho { .. } => continue,
        };
        if lazy_chirho && !name_chirho.text_chirho().is_empty() {
            errors_chirho.push(DeclarationErrorChirho {
                ghc_code_chirho: "GHC-81601",
                message_chirho: format!(
                    "Lazy field annotations (~) are disabled without StrictData, but `{}` has one",
                    name_chirho.text_chirho()
                ),
                span_chirho,
                related_span_chirho: None,
            });
        }
    }
}

/// Whether `StrictData` is on, in GHC's flag order: `Strict` turns it on as an
/// implied flag, `NoStrictData` turns it off, and the last mention wins.
/// `NoStrict` leaves it as it was: GHC applies implied flags only when turning
/// a flag on.
fn strict_data_enabled_chirho(module_chirho: &ModuleChirho) -> bool {
    module_chirho
        .extensions_chirho
        .iter()
        .fold(
            false,
            |enabled_chirho, extension_chirho| match extension_chirho.as_str() {
                "StrictData" | "Strict" => true,
                "NoStrictData" => false,
                _ => enabled_chirho,
            },
        )
}

/// Whether a constructor signature's own context is non-empty, looking through
/// its leading quantifiers. A context inside an argument belongs to a rank-N
/// field type, which GHC allows.
fn signature_has_context_chirho(ty_chirho: &TypeChirho) -> bool {
    match ty_chirho {
        TypeChirho::ForallChirho { body_chirho, .. }
        | TypeChirho::ParenChirho {
            inner_chirho: body_chirho,
            ..
        } => signature_has_context_chirho(body_chirho),
        TypeChirho::QualChirho { context_chirho, .. } => !context_chirho.is_empty(),
        _ => false,
    }
}

/// GHC's spelling of a field count (`speakN`).
fn spoken_count_chirho(count_chirho: usize) -> String {
    const WORDS_CHIRHO: [&str; 7] = ["none", "one", "two", "three", "four", "five", "six"];
    WORDS_CHIRHO.get(count_chirho).map_or_else(
        || count_chirho.to_string(),
        |word_chirho| (*word_chirho).to_string(),
    )
}

/// Rule 3: one instance body binds a name once (`GHC-10498`).
///
/// Lowering merges adjacent equations of one name (`merge_fun_binds_chirho`),
/// so a second binding of a name here was written apart from the first, which
/// GHC rejects as conflicting definitions. Signatures are not bindings.
fn check_instance_bindings_chirho(
    methods_chirho: &[LocalBindChirho],
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) {
    let mut first_bindings_chirho: HashMap<&str, SpanChirho> = HashMap::new();
    for method_chirho in methods_chirho {
        let LocalBindChirho::FunBindChirho {
            name_chirho,
            span_chirho,
            ..
        } = method_chirho
        else {
            continue;
        };
        match first_bindings_chirho.entry(name_chirho.text_chirho()) {
            Entry::Vacant(vacant_chirho) => {
                vacant_chirho.insert(*span_chirho);
            }
            Entry::Occupied(first_chirho) => errors_chirho.push(DeclarationErrorChirho {
                ghc_code_chirho: "GHC-10498",
                message_chirho: format!(
                    "Conflicting definitions for `{}` in one instance declaration",
                    name_chirho.text_chirho()
                ),
                span_chirho: *span_chirho,
                related_span_chirho: Some(*first_chirho.get()),
            }),
        }
    }
}

/// Rule 4: a default signature has a default binding (`GHC-59794`).
///
/// Lowering attaches both to the method by name, and extracts only
/// `default name ::` signatures, so an operator's default signature never
/// reaches this rule. The class member walk (`collect_instance_methods_chirho`)
/// reads past a fixity declaration for the class's own names (row 685), but any
/// other non-method member after a method still ends it, and a binding written
/// after such a member is not seen.
fn check_default_signatures_chirho(
    class_name_chirho: &NameChirho,
    methods_chirho: &[ClassMethodChirho],
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) {
    for method_chirho in methods_chirho {
        if method_chirho.default_sig_chirho.is_some() && method_chirho.default_chirho.is_none() {
            errors_chirho.push(DeclarationErrorChirho {
                ghc_code_chirho: "GHC-59794",
                message_chirho: format!(
                    "Class `{}` has a generic-default signature without a binding `{}`",
                    class_name_chirho.text_chirho(),
                    method_chirho.name_chirho.text_chirho()
                ),
                span_chirho: method_chirho.span_chirho,
                related_span_chirho: None,
            });
        }
    }
}

#[cfg(test)]
#[path = "declarations_tests_chirho.rs"]
mod tests_chirho;
