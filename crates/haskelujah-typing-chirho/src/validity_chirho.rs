// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! # Source-type validity checks
//!
//! GHC rejects some *well-kinded* types outright because the extension that
//! would license them is not enabled. This module implements that family of
//! checks (GHC error code `GHC-91510`): a polymorphic type (`forall`) or a
//! qualified type (`ctx =>`) is only legal in certain syntactic positions.
//!
//! Haskell2010 allows a `forall`/context only at the very top of a signature.
//! Nesting one under an arrow is rank-N and needs `RankNTypes`; putting one in
//! a type-constructor argument is impredicative and needs `ImpredicativeTypes`.
//!
//! This lives in its own module rather than inside `infer_chirho.rs` on
//! purpose: these are *syntactic* well-formedness rules that need no
//! inference state, and `infer_chirho.rs` is already far past the size a
//! single file should carry.

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::module_chirho::{ImportItemChirho, ModuleChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

/// `E0206`: a type form that needs an extension which is not enabled.
const ILLEGAL_TYPE_FORM_CODE_CHIRHO: u16 = 206;

/// A source type that is well-kinded but not licensed by the enabled
/// extensions.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidityErrorChirho {
    /// Human-readable message, phrased like GHC's `GHC-91510`.
    pub message_chirho: String,
    /// The extension that would license this type, if any.
    pub suggested_extension_chirho: Option<&'static str>,
    /// Where the offending type appears.
    pub span_chirho: SpanChirho,
}

/// Result of the source-type validity pass.
pub struct ValidityResultChirho {
    /// Diagnostics produced by the pass.
    pub diagnostics_chirho: DiagnosticBundleChirho,
}

/// Run the validity pass and package the findings as diagnostics.
pub fn check_module_type_validity_diagnostics_chirho(
    module_chirho: &ModuleChirho,
) -> ValidityResultChirho {
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();
    for error_chirho in check_module_type_validity_chirho(module_chirho) {
        let mut diagnostic_chirho = DiagnosticChirho::error_with_code_chirho(
            ErrorCodeChirho::error_chirho(ILLEGAL_TYPE_FORM_CODE_CHIRHO),
            error_chirho.message_chirho,
            error_chirho.span_chirho,
        );
        if let Some(extension_chirho) = error_chirho.suggested_extension_chirho {
            diagnostic_chirho = diagnostic_chirho.with_note_chirho(format!(
                "perhaps you intended to use the `{extension_chirho}` extension"
            ));
        }
        diagnostics_chirho.push_chirho(diagnostic_chirho);
    }
    ValidityResultChirho { diagnostics_chirho }
}

/// Where a type sits relative to the enclosing signature. The position
/// decides which extension (if any) is needed to license a `forall` or a
/// context appearing there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TyPositionChirho {
    /// The very top of a signature. Haskell2010 already allows a leading
    /// `forall` and a leading context here.
    TopChirho,
    /// Nested beneath a function arrow — rank-N. Needs `RankNTypes`.
    RankNChirho,
    /// An argument of a type constructor, list, or tuple — impredicative.
    /// Needs `ImpredicativeTypes`.
    ImpredicativeChirho,
}

/// The synthetic class name the parser writes when a constraint shape did not
/// lower into a recognised class application (parser `lower_chirho.rs:4450`).
///
/// Its single "argument" is then the *whole* unlowered constraint rather than a
/// class argument, so it must never be judged as one.
const UNRESOLVED_CONSTRAINT_MARKER_CHIRHO: &str = "?";

/// Extension state that licenses the type forms this pass checks.
///
/// Bundled into one `Copy` value so the walker keeps a short argument list;
/// every flag is decided once, at the top of the pass.
#[derive(Debug, Clone, Copy)]
struct LicenseChirho {
    /// `RankNTypes` (or a spelling/edition that implies it) is on.
    rank_n_chirho: bool,
    /// `ImpredicativeTypes` is on.
    impredicative_chirho: bool,
    /// `QuantifiedConstraints` is on.
    quantified_constraints_chirho: bool,
    /// `DatatypeContexts` is on. When it is, constructor fields are skipped: a
    /// datatype "stupid theta" is written on the declaration *head*, and we
    /// will not risk judging a head context as if it were a field.
    datatype_contexts_chirho: bool,
}

/// Check every source type in a module for extension-licensed forms.
///
/// Walked sites: type signatures, type-synonym right-hand sides, class method
/// signatures, positional and GADT constructor types, and the constraints
/// inside any `ctx =>` we walk through.
///
/// Deliberately NOT walked, because our own lowering cannot represent the form
/// and a check there would be vacuous or actively wrong:
///   * **record** constructor fields — `lower_record_field_chirho` (parser
///     `lower_chirho.rs:7350`) takes a flat token path that splits on the
///     top-level `=>` *before* it lifts a leading `forall`, so
///     `fld :: forall a. C a => t` arrives as a quantified constraint the
///     source never wrote. Judging it wrongly rejects six files GHC accepts
///     (`Vta1`, `LocalGivenEqs`, `T3018`, `T11339`, `T11339b`, `T11339c`).
///     Positional and GADT fields lower structurally and stay covered.
///   * instance head types — `lower_instance_head_types_chirho`
///     (`lower_chirho.rs:3969`) drops the `forall` token entirely;
///   * class/instance declaration contexts —
///     `parse_simple_constraint_segment_chirho` (`lower_chirho.rs:3920`)
///     returns `None` for any segment holding a `forall` or `=>`;
///   * `kind_sig_chirho` and type-variable kind annotations — `AstKindChirho`
///     (`decl_chirho.rs:19`) cannot hold a `forall`, and standalone kind
///     signatures legitimately carry one.
pub fn check_module_type_validity_chirho(module_chirho: &ModuleChirho) -> Vec<ValidityErrorChirho> {
    // GHC: ImpredicativeTypes implies RankNTypes. `Rank2Types` is a legacy
    // spelling of `RankNTypes` and still widely used in real source.
    let impredicative_chirho = extension_enabled_chirho(module_chirho, "ImpredicativeTypes");
    let license_chirho = LicenseChirho {
        rank_n_chirho: impredicative_chirho
            || extension_enabled_chirho(module_chirho, "RankNTypes")
            || extension_enabled_chirho(module_chirho, "Rank2Types")
            // A module that never declares a language edition is compiled by
            // modern GHC under GHC2021, which enables RankNTypes. We do not
            // model language editions, so assuming rank-N is OFF would reject
            // ordinary modern code. Only an explicit legacy edition turns it off.
            || !declares_legacy_edition_chirho(module_chirho),
        impredicative_chirho,
        quantified_constraints_chirho: extension_enabled_chirho(
            module_chirho,
            "QuantifiedConstraints",
        ),
        datatype_contexts_chirho: extension_enabled_chirho(module_chirho, "DatatypeContexts"),
    };

    let mut errors_chirho = Vec::new();
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::TypeSigChirho { ty_chirho, .. } => {
                walk_type_chirho(
                    ty_chirho,
                    TyPositionChirho::TopChirho,
                    license_chirho,
                    &mut errors_chirho,
                );
            }
            DeclChirho::TypeAliasDeclChirho { rhs_chirho, .. } => {
                // A synonym RHS is not a signature top: `type C a = Num a => a`
                // is rejected by GHC without RankNTypes.
                walk_type_chirho(
                    rhs_chirho,
                    TyPositionChirho::RankNChirho,
                    license_chirho,
                    &mut errors_chirho,
                );
            }
            DeclChirho::ClassDeclChirho { methods_chirho, .. } => {
                // A class method signature is a signature top, exactly like a
                // top-level `::` binding (GHC: T12083b).
                for method_chirho in methods_chirho {
                    walk_type_chirho(
                        &method_chirho.ty_chirho,
                        TyPositionChirho::TopChirho,
                        license_chirho,
                        &mut errors_chirho,
                    );
                }
            }
            DeclChirho::DataDeclChirho {
                constructors_chirho,
                ..
            } => {
                for constructor_chirho in constructors_chirho {
                    walk_con_decl_chirho(constructor_chirho, license_chirho, &mut errors_chirho);
                }
            }
            DeclChirho::NewtypeDeclChirho {
                constructor_chirho, ..
            } => {
                walk_con_decl_chirho(constructor_chirho, license_chirho, &mut errors_chirho);
            }
            _ => {}
        }
    }
    errors_chirho.extend(check_unlifted_newtype_fields_chirho(module_chirho));
    errors_chirho
}

/// Walk the source types a data constructor declaration writes down.
///
/// A constructor field behaves like a function argument, so a `forall` or a
/// context there needs `RankNTypes` (GHC: `tcfail184`). A GADT constructor
/// signature is a signature top, so a *leading* `forall`/context is fine but a
/// polytype under an arrow or inside an application is not (GHC: `tcfail195`).
fn walk_con_decl_chirho(
    con_chirho: &ConDeclChirho,
    license_chirho: LicenseChirho,
    errors_chirho: &mut Vec<ValidityErrorChirho>,
) {
    if license_chirho.datatype_contexts_chirho {
        return;
    }
    match con_chirho {
        ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
            for (_, ty_chirho) in fields_chirho {
                walk_type_chirho(
                    ty_chirho,
                    TyPositionChirho::RankNChirho,
                    license_chirho,
                    errors_chirho,
                );
            }
        }
        // Skipped on purpose — see the note on `check_module_type_validity_chirho`.
        ConDeclChirho::RecordChirho { .. } => {}
        ConDeclChirho::GadtChirho { ty_chirho, .. } => {
            walk_type_chirho(
                ty_chirho,
                TyPositionChirho::TopChirho,
                license_chirho,
                errors_chirho,
            );
        }
    }
}

/// Check one constraint written in a `ctx =>` position.
///
/// GHC: "A constraint must be a monotype". A `forall`/`=>` *inside* a
/// constraint is a quantified constraint and needs `QuantifiedConstraints`
/// (`T9196`); a class applied to a polytype is impredicative and needs
/// `ImpredicativeTypes` (`tcfail196`).
fn walk_constraint_chirho(
    constraint_chirho: &ConstraintChirho,
    license_chirho: LicenseChirho,
    errors_chirho: &mut Vec<ValidityErrorChirho>,
) {
    match constraint_chirho {
        ConstraintChirho::QuantifiedChirho { span_chirho, .. } => {
            if !license_chirho.quantified_constraints_chirho {
                errors_chirho.push(ValidityErrorChirho {
                    message_chirho: format!(
                        "Illegal polymorphic type: {} — a constraint must be a monotype",
                        render_constraint_chirho(constraint_chirho)
                    ),
                    suggested_extension_chirho: Some("QuantifiedConstraints"),
                    span_chirho: *span_chirho,
                });
            }
        }
        ConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho,
            ..
        } => {
            if class_chirho.text_chirho() == UNRESOLVED_CONSTRAINT_MARKER_CHIRHO {
                // The parser did not recognise this constraint shape, so its
                // "argument" is the whole unlowered constraint rather than a
                // class argument. Judging it would reject valid programs.
                return;
            }
            for arg_chirho in args_chirho {
                walk_type_chirho(
                    arg_chirho,
                    TyPositionChirho::ImpredicativeChirho,
                    license_chirho,
                    errors_chirho,
                );
            }
        }
    }
}

/// True when the module explicitly asks for a pre-GHC2021 language edition,
/// where rank-N types are genuinely off unless requested.
fn declares_legacy_edition_chirho(module_chirho: &ModuleChirho) -> bool {
    extension_enabled_chirho(module_chirho, "Haskell2010")
        || extension_enabled_chirho(module_chirho, "Haskell98")
}

/// Evaluate an extension flag with GHC's `NoFoo` override order: the last
/// mention wins, so `{-# LANGUAGE MagicHash, NoMagicHash #-}` leaves it off.
///
/// Mirrors `haskelujah_parser_chirho::pragma_chirho::extension_enabled_chirho`;
/// duplicated rather than shared because the typing crate does not depend on
/// the parser.
fn extension_enabled_chirho(module_chirho: &ModuleChirho, name_chirho: &str) -> bool {
    let disabled_name_chirho = format!("No{name_chirho}");
    module_chirho
        .extensions_chirho
        .iter()
        .fold(false, |enabled_chirho, extension_chirho| {
            if extension_chirho == name_chirho {
                true
            } else if extension_chirho == &disabled_name_chirho {
                false
            } else {
                enabled_chirho
            }
        })
}

/// The unlifted primitive type constructors that make a `newtype` field
/// illegal without `UnliftedNewtypes`.
///
/// Deliberately a fixed list rather than "any name ending in `#`": the guard
/// must not fire on a user type that merely borrows the spelling.
const UNLIFTED_PRIM_TYCONS_CHIRHO: &[&str] = &[
    "Int#",
    "Int8#",
    "Int16#",
    "Int32#",
    "Int64#",
    "Word#",
    "Word8#",
    "Word16#",
    "Word32#",
    "Word64#",
    "Char#",
    "Float#",
    "Double#",
    "Addr#",
    "ByteArray#",
    "MutableByteArray#",
    "Array#",
    "MutableArray#",
    "SmallArray#",
    "SmallMutableArray#",
    "ArrayArray#",
    "MutableArrayArray#",
    "MutVar#",
    "MVar#",
    "TVar#",
    "IOPort#",
    "StablePtr#",
    "StableName#",
    "Weak#",
    "ThreadId#",
    "Proxy#",
    "Compact#",
    "StackSnapshot#",
    "BCO#",
];

/// The modules that export the `GHC.Prim` primitive type constructors.
///
/// Nothing else can put `Int#` in scope — `Prelude` does not export it — which
/// is why both GHC tests that trigger this error name it explicitly.
const PRIM_EXPORTING_MODULES_CHIRHO: &[&str] = &["GHC.Exts", "GHC.Prim"];

/// True when the module explicitly imports `wanted_chirho` as a type from a
/// primitive-exporting module.
///
/// Without this conjunct the guard fires on any module that merely *spells* one
/// of the primitive names — including one that defines its own `data Int#` in a
/// sibling file, or a `data family Int#` that our lowering drops
/// (parser `lower_chirho.rs:862`). Both are programs GHC accepts.
fn imports_prim_tycon_chirho(module_chirho: &ModuleChirho, wanted_chirho: &str) -> bool {
    module_chirho.imports_chirho.iter().any(|import_chirho| {
        PRIM_EXPORTING_MODULES_CHIRHO.contains(&import_chirho.module_chirho.text_chirho())
            && import_chirho
                .spec_chirho
                .as_ref()
                .is_some_and(|spec_chirho| {
                    !spec_chirho.hiding_chirho
                        && spec_chirho.items_chirho.iter().any(|item_chirho| {
                            matches!(
                                item_chirho,
                                ImportItemChirho::TyConChirho { name_chirho, .. }
                                    if name_chirho.text_chirho() == wanted_chirho
                            )
                        })
                })
    })
}

/// Every type-level name the module declares itself.
///
/// `MagicHash` permits a user-written `data Int#`, so a locally declared name
/// shadows the primitive and must not be judged.
fn locally_declared_type_names_chirho(module_chirho: &ModuleChirho) -> Vec<&str> {
    module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho { name_chirho, .. }
            | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
            | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
            | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho())
            }
            _ => None,
        })
        .collect()
}

/// Reject a `newtype` over an unlifted primitive when `UnliftedNewtypes` is off.
///
/// GHC `GHC-55233`: "Newtype has non-* return kind". `data` is untouched — GHC
/// allows unlifted *data* fields without the extension.
fn check_unlifted_newtype_fields_chirho(module_chirho: &ModuleChirho) -> Vec<ValidityErrorChirho> {
    if !extension_enabled_chirho(module_chirho, "MagicHash")
        || extension_enabled_chirho(module_chirho, "UnliftedNewtypes")
    {
        return Vec::new();
    }
    let local_type_names_chirho = locally_declared_type_names_chirho(module_chirho);
    let mut errors_chirho = Vec::new();
    for decl_chirho in &module_chirho.decls_chirho {
        let DeclChirho::NewtypeDeclChirho {
            constructor_chirho, ..
        } = decl_chirho
        else {
            continue;
        };
        let field_tys_chirho: Vec<&TypeChirho> = match constructor_chirho {
            ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => fields_chirho
                .iter()
                .map(|(_, ty_chirho)| ty_chirho)
                .collect(),
            ConDeclChirho::RecordChirho { fields_chirho, .. } => fields_chirho
                .iter()
                .map(|field_chirho| &field_chirho.ty_chirho)
                .collect(),
            // GADT-syntax newtypes are not parsed into fields today.
            ConDeclChirho::GadtChirho { .. } => Vec::new(),
        };
        for field_ty_chirho in field_tys_chirho {
            let Some(prim_chirho) = unlifted_prim_tycon_name_chirho(field_ty_chirho) else {
                continue;
            };
            // A locally declared type of that name shadows the primitive, and a
            // primitive that is never imported cannot be in scope at all.
            if local_type_names_chirho.contains(&prim_chirho)
                || !imports_prim_tycon_chirho(module_chirho, prim_chirho)
            {
                continue;
            }
            errors_chirho.push(ValidityErrorChirho {
                message_chirho: format!(
                    "Newtype has non-* return kind: the field type `{prim_chirho}` is unlifted"
                ),
                suggested_extension_chirho: Some("UnliftedNewtypes"),
                span_chirho: field_ty_chirho.span_chirho(),
            });
        }
    }
    errors_chirho
}

/// The primitive type-constructor name a field type refers to, if it is exactly
/// one of them.
///
/// Only parentheses are peeled: the guard never reasons about what a name is
/// *defined* as, and never looks through an application spine, so
/// `newtype G = G (F Int#)` is left alone.
fn unlifted_prim_tycon_name_chirho(ty_chirho: &TypeChirho) -> Option<&str> {
    match ty_chirho {
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            unlifted_prim_tycon_name_chirho(inner_chirho)
        }
        TypeChirho::KindAnnotChirho { type_chirho, .. } => {
            unlifted_prim_tycon_name_chirho(type_chirho)
        }
        TypeChirho::ConChirho(name_chirho) => {
            let text_chirho = name_chirho.text_chirho();
            UNLIFTED_PRIM_TYCONS_CHIRHO
                .contains(&text_chirho)
                .then_some(text_chirho)
        }
        _ => None,
    }
}

fn walk_type_chirho(
    ty_chirho: &TypeChirho,
    position_chirho: TyPositionChirho,
    license_chirho: LicenseChirho,
    errors_chirho: &mut Vec<ValidityErrorChirho>,
) {
    match ty_chirho {
        TypeChirho::ForallChirho {
            body_chirho,
            span_chirho,
            ..
        }
        | TypeChirho::RequiredForallChirho {
            body_chirho,
            span_chirho,
            ..
        } => {
            report_if_unlicensed_chirho(
                "polymorphic",
                ty_chirho,
                position_chirho,
                *span_chirho,
                license_chirho,
                errors_chirho,
            );
            // A leading `forall` does not itself move us out of top position.
            walk_type_chirho(body_chirho, position_chirho, license_chirho, errors_chirho);
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            span_chirho,
            ..
        } => {
            report_if_unlicensed_chirho(
                "qualified",
                ty_chirho,
                position_chirho,
                *span_chirho,
                license_chirho,
                errors_chirho,
            );
            // The constraints themselves are types too: a `forall` inside one
            // is a quantified constraint, and a class applied to a polytype is
            // impredicative. Neither is reachable from `body_chirho`.
            for constraint_chirho in context_chirho {
                walk_constraint_chirho(constraint_chirho, license_chirho, errors_chirho);
            }
            walk_type_chirho(body_chirho, position_chirho, license_chirho, errors_chirho);
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            // Both sides of an arrow are rank-N positions: `(forall a. a) -> b`
            // and `b -> forall a. a` both need RankNTypes.
            for side_chirho in [arg_chirho, result_chirho] {
                walk_type_chirho(
                    side_chirho,
                    TyPositionChirho::RankNChirho,
                    license_chirho,
                    errors_chirho,
                );
            }
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        }
        | TypeChirho::KindAppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            // The head keeps the current position; the argument is
            // impredicative (`Maybe (forall a. a)`).
            walk_type_chirho(fun_chirho, position_chirho, license_chirho, errors_chirho);
            walk_type_chirho(
                arg_chirho,
                TyPositionChirho::ImpredicativeChirho,
                license_chirho,
                errors_chirho,
            );
        }
        TypeChirho::ListChirho { element_chirho, .. } => {
            walk_type_chirho(
                element_chirho,
                TyPositionChirho::ImpredicativeChirho,
                license_chirho,
                errors_chirho,
            );
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                walk_type_chirho(
                    element_chirho,
                    TyPositionChirho::ImpredicativeChirho,
                    license_chirho,
                    errors_chirho,
                );
            }
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            walk_type_chirho(inner_chirho, position_chirho, license_chirho, errors_chirho);
        }
        TypeChirho::KindAnnotChirho { type_chirho, .. } => {
            // The kind pass owns the classifier's quantification rules; do not
            // apply value-type impredicativity licensing to a written kind.
            walk_type_chirho(type_chirho, position_chirho, license_chirho, errors_chirho);
        }
        TypeChirho::VarChirho(_)
        | TypeChirho::ConChirho(_)
        | TypeChirho::PromotedConChirho { .. }
        | TypeChirho::WildcardChirho { .. }
        | TypeChirho::LitChirho { .. } => {}
    }
}

fn report_if_unlicensed_chirho(
    kind_chirho: &str,
    ty_chirho: &TypeChirho,
    position_chirho: TyPositionChirho,
    span_chirho: SpanChirho,
    license_chirho: LicenseChirho,
    errors_chirho: &mut Vec<ValidityErrorChirho>,
) {
    let needed_chirho = match position_chirho {
        TyPositionChirho::TopChirho => None,
        TyPositionChirho::RankNChirho if !license_chirho.rank_n_chirho => Some("RankNTypes"),
        TyPositionChirho::ImpredicativeChirho if !license_chirho.impredicative_chirho => {
            Some("ImpredicativeTypes")
        }
        _ => None,
    };
    let Some(extension_chirho) = needed_chirho else {
        return;
    };
    errors_chirho.push(ValidityErrorChirho {
        message_chirho: format!(
            "Illegal {kind_chirho} type: {}",
            render_type_chirho(ty_chirho)
        ),
        suggested_extension_chirho: Some(extension_chirho),
        span_chirho,
    });
}

/// Compact rendering used only inside diagnostics.
fn render_type_chirho(ty_chirho: &TypeChirho) -> String {
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho) | TypeChirho::ConChirho(name_chirho) => {
            name_chirho.text_chirho().to_string()
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => format!(
            "{} {}",
            render_type_chirho(fun_chirho),
            render_type_chirho(arg_chirho)
        ),
        TypeChirho::KindAppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => format!(
            "{} @{}",
            render_type_chirho(fun_chirho),
            render_type_chirho(arg_chirho)
        ),
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => format!(
            "{} -> {}",
            render_type_chirho(arg_chirho),
            render_type_chirho(result_chirho)
        ),
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => format!(
            "({})",
            elements_chirho
                .iter()
                .map(render_type_chirho)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        TypeChirho::ListChirho { element_chirho, .. } => {
            format!("[{}]", render_type_chirho(element_chirho))
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => render_type_chirho(inner_chirho),
        TypeChirho::KindAnnotChirho {
            type_chirho,
            kind_chirho,
            ..
        } => format!(
            "({} :: {})",
            render_type_chirho(type_chirho),
            render_type_chirho(kind_chirho)
        ),
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            let rendered_chirho: Vec<String> = context_chirho
                .iter()
                .map(render_constraint_chirho)
                .collect();
            let context_text_chirho = match rendered_chirho.len() {
                1 => rendered_chirho[0].clone(),
                _ => format!("({})", rendered_chirho.join(", ")),
            };
            format!(
                "{context_text_chirho} => {}",
                render_type_chirho(body_chirho)
            )
        }
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            let names_chirho: Vec<String> = vars_chirho
                .iter()
                .map(|var_chirho| var_chirho.name_chirho.text_chirho().to_string())
                .collect();
            format!(
                "forall {}. {}",
                names_chirho.join(" "),
                render_type_chirho(body_chirho)
            )
        }
        TypeChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            let names_chirho: Vec<String> = vars_chirho
                .iter()
                .map(|var_chirho| var_chirho.name_chirho.text_chirho().to_string())
                .collect();
            format!(
                "forall {} -> {}",
                names_chirho.join(" "),
                render_type_chirho(body_chirho)
            )
        }
        TypeChirho::PromotedConChirho { name_chirho, .. } => {
            format!("'{}", name_chirho.text_chirho())
        }
        TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => format!(
            "'[{}]",
            elements_chirho
                .iter()
                .map(render_type_chirho)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        TypeChirho::WildcardChirho { .. } => "_".to_string(),
        TypeChirho::LitChirho { value_chirho, .. } => value_chirho.clone(),
    }
}

fn render_constraint_chirho(constraint_chirho: &ConstraintChirho) -> String {
    match constraint_chirho {
        ConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho,
            ..
        } => {
            let mut text_chirho = class_chirho.text_chirho().to_string();
            for arg_chirho in args_chirho {
                text_chirho.push(' ');
                text_chirho.push_str(&render_type_chirho(arg_chirho));
            }
            text_chirho
        }
        ConstraintChirho::QuantifiedChirho {
            vars_chirho,
            context_chirho,
            body_chirho,
            ..
        } => {
            // Render the bound variables rather than an ellipsis, so the
            // message reads like GHC's: `forall a. Eq a`.
            let mut text_chirho = String::from("forall");
            for var_chirho in vars_chirho {
                text_chirho.push(' ');
                text_chirho.push_str(var_chirho.name_chirho.text_chirho());
            }
            text_chirho.push_str(". ");
            if !context_chirho.is_empty() {
                let rendered_chirho: Vec<String> = context_chirho
                    .iter()
                    .map(render_constraint_chirho)
                    .collect();
                if rendered_chirho.len() == 1 {
                    text_chirho.push_str(&rendered_chirho[0]);
                } else {
                    text_chirho.push_str(&format!("({})", rendered_chirho.join(", ")));
                }
                text_chirho.push_str(" => ");
            }
            text_chirho.push_str(&render_constraint_chirho(body_chirho));
            text_chirho
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_ast_chirho::decl_chirho::{
        ClassMethodChirho, FieldDeclChirho, StrictnessChirho, TyVarChirho,
    };
    use haskelujah_ast_chirho::module_chirho::{
        ExportMembersChirho, ImportDeclChirho, ImportSpecChirho,
    };
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

    fn mk_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    fn mk_module_chirho(
        decls_chirho: Vec<DeclChirho>,
        extensions_chirho: Vec<String>,
    ) -> ModuleChirho {
        ModuleChirho {
            name_chirho: mk_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho,
            extensions_chirho,
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_var_chirho(text_chirho: &str) -> TypeChirho {
        TypeChirho::VarChirho(mk_name_chirho(text_chirho))
    }

    fn mk_con_chirho(text_chirho: &str) -> TypeChirho {
        TypeChirho::ConChirho(mk_name_chirho(text_chirho))
    }

    /// `forall a. a -> a`
    fn mk_poly_id_chirho() -> TypeChirho {
        TypeChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
            body_chirho: Box::new(TypeChirho::FunChirho {
                arg_chirho: Box::new(mk_var_chirho("a")),
                mult_chirho: None,
                result_chirho: Box::new(mk_var_chirho("a")),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_sig_chirho(ty_chirho: TypeChirho) -> DeclChirho {
        DeclChirho::TypeSigChirho {
            name_chirho: mk_name_chirho("f"),
            ty_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn top_level_forall_is_always_allowed_chirho() {
        // `f :: forall a. a -> a` is plain Haskell2010 with ExplicitForAll.
        let module_chirho = mk_module_chirho(vec![mk_sig_chirho(mk_poly_id_chirho())], vec![]);
        assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
    }

    #[test]
    fn rank_two_argument_needs_rank_n_types_chirho() {
        // `f :: (forall a. a -> a) -> Int`
        let ty_chirho = TypeChirho::FunChirho {
            arg_chirho: Box::new(mk_poly_id_chirho()),
            mult_chirho: None,
            result_chirho: Box::new(mk_con_chirho("Int")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let without_chirho = mk_module_chirho(
            vec![mk_sig_chirho(ty_chirho.clone())],
            vec!["Haskell2010".to_string()],
        );
        let errors_chirho = check_module_type_validity_chirho(&without_chirho);
        assert_eq!(errors_chirho.len(), 1);
        assert_eq!(
            errors_chirho[0].suggested_extension_chirho,
            Some("RankNTypes")
        );

        let with_chirho = mk_module_chirho(
            vec![mk_sig_chirho(ty_chirho)],
            vec!["RankNTypes".to_string()],
        );
        assert!(check_module_type_validity_chirho(&with_chirho).is_empty());
    }

    #[test]
    fn polytype_as_constructor_argument_needs_impredicative_types_chirho() {
        // `f :: Maybe (forall a. a -> a)` — RankNTypes is NOT enough here.
        let ty_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(mk_con_chirho("Maybe")),
            arg_chirho: Box::new(mk_poly_id_chirho()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let rank_n_only_chirho = mk_module_chirho(
            vec![mk_sig_chirho(ty_chirho.clone())],
            vec!["RankNTypes".to_string()],
        );
        let errors_chirho = check_module_type_validity_chirho(&rank_n_only_chirho);
        assert_eq!(errors_chirho.len(), 1);
        assert_eq!(
            errors_chirho[0].suggested_extension_chirho,
            Some("ImpredicativeTypes")
        );

        let impredicative_chirho = mk_module_chirho(
            vec![mk_sig_chirho(ty_chirho)],
            vec!["ImpredicativeTypes".to_string()],
        );
        assert!(check_module_type_validity_chirho(&impredicative_chirho).is_empty());
    }

    #[test]
    fn qualified_type_synonym_rhs_needs_rank_n_types_chirho() {
        // `type Constrd a = Num a => a`
        let rhs_chirho = TypeChirho::QualChirho {
            context_chirho: vec![ConstraintChirho::ClassChirho {
                class_chirho: mk_name_chirho("Num"),
                args_chirho: vec![mk_var_chirho("a")],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            body_chirho: Box::new(mk_var_chirho("a")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let decl_chirho = DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("Constrd"),
            type_vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
            rhs_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho =
            mk_module_chirho(vec![decl_chirho.clone()], vec!["Haskell2010".to_string()]);
        let errors_chirho = check_module_type_validity_chirho(&module_chirho);
        assert_eq!(errors_chirho.len(), 1);
        assert!(
            errors_chirho[0].message_chirho.contains("Num a"),
            "diagnostic should name the constraint, got {}",
            errors_chirho[0].message_chirho
        );

        let with_chirho = mk_module_chirho(vec![decl_chirho], vec!["RankNTypes".to_string()]);
        assert!(check_module_type_validity_chirho(&with_chirho).is_empty());
    }

    #[test]
    fn impredicative_types_implies_rank_n_types_chirho() {
        // GHC: ImpredicativeTypes implies RankNTypes, so a rank-2 argument is
        // licensed by ImpredicativeTypes alone.
        let ty_chirho = TypeChirho::FunChirho {
            arg_chirho: Box::new(mk_poly_id_chirho()),
            mult_chirho: None,
            result_chirho: Box::new(mk_con_chirho("Int")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = mk_module_chirho(
            vec![mk_sig_chirho(ty_chirho)],
            vec!["ImpredicativeTypes".to_string()],
        );
        assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
    }

    // ---- walk sites added for the validity-walker lane ----

    /// `Con (forall a. a -> a)` as a single positional field.
    fn mk_poly_field_con_chirho() -> ConDeclChirho {
        ConDeclChirho::OrdinaryChirho {
            name_chirho: mk_name_chirho("Con"),
            fields_chirho: vec![(StrictnessChirho::LazyChirho, mk_poly_id_chirho())],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn class_method_rank_two_argument_needs_rank_n_types_chirho() {
        // `class C a where m :: (forall b. b -> b) -> a`
        let method_ty_chirho = TypeChirho::FunChirho {
            arg_chirho: Box::new(mk_poly_id_chirho()),
            mult_chirho: None,
            result_chirho: Box::new(mk_var_chirho("a")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let decl_chirho = DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: mk_name_chirho("C"),
            type_vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
            methods_chirho: vec![ClassMethodChirho {
                name_chirho: mk_name_chirho("m"),
                ty_chirho: method_ty_chirho,
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = mk_module_chirho(vec![decl_chirho], vec!["Haskell2010".to_string()]);
        let errors_chirho = check_module_type_validity_chirho(&module_chirho);
        assert_eq!(errors_chirho.len(), 1);
        assert_eq!(
            errors_chirho[0].suggested_extension_chirho,
            Some("RankNTypes")
        );
    }

    #[test]
    fn positional_constructor_field_forall_needs_rank_n_types_chirho() {
        // `data T = Con (forall a. a -> a)` — GHC tcfail184.
        let decl_chirho = DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("T"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![mk_poly_field_con_chirho()],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = mk_module_chirho(vec![decl_chirho], vec!["Haskell2010".to_string()]);
        assert_eq!(check_module_type_validity_chirho(&module_chirho).len(), 1);
    }

    #[test]
    fn record_constructor_fields_are_never_judged_chirho() {
        // The parser lowers record fields through a flat path that splits on
        // `=>` before lifting a leading `forall`, manufacturing a constraint the
        // source never wrote. Judging them regresses six files GHC accepts, so
        // this site stays skipped until the parser is fixed.
        let decl_chirho = DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("T"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::RecordChirho {
                name_chirho: mk_name_chirho("Con"),
                fields_chirho: vec![FieldDeclChirho {
                    names_chirho: vec![mk_name_chirho("fld")],
                    ty_chirho: mk_poly_id_chirho(),
                    strictness_chirho: StrictnessChirho::LazyChirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = mk_module_chirho(vec![decl_chirho], vec!["Haskell2010".to_string()]);
        assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
    }

    #[test]
    fn datatype_contexts_skips_constructor_fields_chirho() {
        let decl_chirho = DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("T"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![mk_poly_field_con_chirho()],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = mk_module_chirho(
            vec![decl_chirho],
            vec!["Haskell2010".to_string(), "DatatypeContexts".to_string()],
        );
        assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
    }

    #[test]
    fn quantified_constraint_needs_its_extension_chirho() {
        // `f :: (forall a. Eq a) => Int` — GHC T9196.
        let quantified_chirho = ConstraintChirho::QuantifiedChirho {
            vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
            context_chirho: vec![],
            body_chirho: Box::new(ConstraintChirho::ClassChirho {
                class_chirho: mk_name_chirho("Eq"),
                args_chirho: vec![mk_var_chirho("a")],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let ty_chirho = TypeChirho::QualChirho {
            context_chirho: vec![quantified_chirho],
            body_chirho: Box::new(mk_con_chirho("Int")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let without_chirho = mk_module_chirho(vec![mk_sig_chirho(ty_chirho.clone())], vec![]);
        let errors_chirho = check_module_type_validity_chirho(&without_chirho);
        assert_eq!(errors_chirho.len(), 1);
        assert_eq!(
            errors_chirho[0].suggested_extension_chirho,
            Some("QuantifiedConstraints")
        );

        let with_chirho = mk_module_chirho(
            vec![mk_sig_chirho(ty_chirho)],
            vec!["QuantifiedConstraints".to_string()],
        );
        assert!(check_module_type_validity_chirho(&with_chirho).is_empty());
    }

    #[test]
    fn unresolved_constraint_marker_is_never_judged_chirho() {
        // The parser writes class `?` when it could not lower a constraint
        // shape; its "argument" is the whole unlowered constraint.
        let ty_chirho = TypeChirho::QualChirho {
            context_chirho: vec![ConstraintChirho::ClassChirho {
                class_chirho: mk_name_chirho(UNRESOLVED_CONSTRAINT_MARKER_CHIRHO),
                args_chirho: vec![mk_poly_id_chirho()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            body_chirho: Box::new(mk_con_chirho("Int")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = mk_module_chirho(
            vec![mk_sig_chirho(ty_chirho)],
            vec!["Haskell2010".to_string()],
        );
        assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
    }

    // ---- unlifted newtypes ----

    fn mk_prim_newtype_module_chirho(
        extensions_chirho: Vec<String>,
        imports_chirho: Vec<ImportDeclChirho>,
        extra_decls_chirho: Vec<DeclChirho>,
    ) -> ModuleChirho {
        let mut decls_chirho = vec![DeclChirho::NewtypeDeclChirho {
            name_chirho: mk_name_chirho("N"),
            type_vars_chirho: vec![],
            constructor_chirho: ConDeclChirho::OrdinaryChirho {
                name_chirho: mk_name_chirho("MkN"),
                fields_chirho: vec![(StrictnessChirho::LazyChirho, mk_con_chirho("Int#"))],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }];
        decls_chirho.extend(extra_decls_chirho);
        let mut module_chirho = mk_module_chirho(decls_chirho, extensions_chirho);
        module_chirho.imports_chirho = imports_chirho;
        module_chirho
    }

    fn mk_ghc_exts_import_chirho(hiding_chirho: bool) -> ImportDeclChirho {
        ImportDeclChirho {
            module_chirho: mk_name_chirho("GHC.Exts"),
            source_chirho: false,
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: Some(ImportSpecChirho {
                hiding_chirho,
                items_chirho: vec![ImportItemChirho::TyConChirho {
                    name_chirho: mk_name_chirho("Int#"),
                    members_chirho: ExportMembersChirho::NoneChirho,
                }],
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn unlifted_newtype_field_needs_the_extension_chirho() {
        let module_chirho = mk_prim_newtype_module_chirho(
            vec!["MagicHash".to_string()],
            vec![mk_ghc_exts_import_chirho(false)],
            vec![],
        );
        let errors_chirho = check_module_type_validity_chirho(&module_chirho);
        assert_eq!(errors_chirho.len(), 1);
        assert_eq!(
            errors_chirho[0].suggested_extension_chirho,
            Some("UnliftedNewtypes")
        );
    }

    #[test]
    fn unlifted_newtype_is_fine_with_the_extension_chirho() {
        let module_chirho = mk_prim_newtype_module_chirho(
            vec!["MagicHash".to_string(), "UnliftedNewtypes".to_string()],
            vec![mk_ghc_exts_import_chirho(false)],
            vec![],
        );
        assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
    }

    #[test]
    fn unlifted_newtype_needs_an_explicit_prim_import_chirho() {
        // Without `import GHC.Exts (Int#)` the name cannot mean GHC's
        // primitive, so a user type of that spelling must not be judged.
        let module_chirho =
            mk_prim_newtype_module_chirho(vec!["MagicHash".to_string()], vec![], vec![]);
        assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
    }

    #[test]
    fn hiding_import_does_not_bring_the_primitive_into_scope_chirho() {
        let module_chirho = mk_prim_newtype_module_chirho(
            vec!["MagicHash".to_string()],
            vec![mk_ghc_exts_import_chirho(true)],
            vec![],
        );
        assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
    }

    #[test]
    fn locally_declared_prim_name_shadows_the_primitive_chirho() {
        // `MagicHash` permits a user-written `data Int#`.
        let local_chirho = DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Int#"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = mk_prim_newtype_module_chirho(
            vec!["MagicHash".to_string()],
            vec![mk_ghc_exts_import_chirho(false)],
            vec![local_chirho],
        );
        assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
    }

    #[test]
    fn extension_override_order_is_honoured_chirho() {
        // GHC: the last mention wins, so `NoMagicHash` cancels `MagicHash`.
        let module_chirho = mk_prim_newtype_module_chirho(
            vec!["MagicHash".to_string(), "NoMagicHash".to_string()],
            vec![mk_ghc_exts_import_chirho(false)],
            vec![],
        );
        assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
    }
}

#[cfg(test)]
mod tests_edition_chirho {
    use super::*;
    use haskelujah_ast_chirho::decl_chirho::TyVarChirho;
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

    fn mk_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    /// `f :: (forall a. a -> a) -> Int` under the given extensions.
    fn rank_two_is_accepted_with_chirho(extensions_chirho: Vec<String>) -> bool {
        let poly_chirho = TypeChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
            body_chirho: Box::new(TypeChirho::FunChirho {
                arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
                mult_chirho: None,
                result_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let sig_chirho = DeclChirho::TypeSigChirho {
            name_chirho: mk_name_chirho("f"),
            ty_chirho: TypeChirho::FunChirho {
                arg_chirho: Box::new(poly_chirho),
                mult_chirho: None,
                result_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Int"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = ModuleChirho {
            name_chirho: mk_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![sig_chirho],
            extensions_chirho,
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        check_module_type_validity_chirho(&module_chirho).is_empty()
    }

    // Regression guard: the first version of this check assumed rank-N types
    // were off whenever `RankNTypes` was absent. That wrongly rejected 13
    // previously-passing GHC should_compile files, because modern GHC compiles
    // an undeclared module under GHC2021, which enables RankNTypes — and
    // because `Rank2Types` is a legacy spelling of the same extension.
    #[test]
    fn modern_default_edition_allows_rank_n_chirho() {
        assert!(rank_two_is_accepted_with_chirho(vec![]));
    }

    #[test]
    fn legacy_rank2types_spelling_is_honoured_chirho() {
        assert!(rank_two_is_accepted_with_chirho(vec![
            "Rank2Types".to_string()
        ]));
    }

    #[test]
    fn explicit_haskell2010_still_rejects_rank_n_chirho() {
        assert!(!rank_two_is_accepted_with_chirho(vec![
            "Haskell2010".to_string()
        ]));
    }
}
