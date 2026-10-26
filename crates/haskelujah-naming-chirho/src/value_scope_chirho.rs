// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Qualified value scope under explicit import lists (row 536).
//!
//! A qualified value occurrence `Q.x` written in an expression (a variable,
//! an operator, or a constructor, including a record construction's) may name
//! only what the imports behind `Q` supply. When every import whose qualifier
//! is `Q` has an explicit list of represented items, those lists are the
//! whole supply: `x` must be listed, written as a member of a listed type, or
//! be a member of a listed `T(..)` whose member list is complete. Anything
//! else is out of scope, as GHC reports it (GHC-76037), so no later phase can
//! stand the bare `x` in for it.
//!
//! A qualifier stays open, under today's behavior, when any import behind it
//! has no list, hides names, holds an item lowering could not represent, or
//! lists a `T(..)` whose members are not known to be complete. Interfaces
//! here are partial, so only a written list proves a name is not supplied.
//! workflow: compiler-pipeline-chirho/value-scope-resolution-chirho

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::StmtChirho;
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ImportDeclChirho, ImportItemChirho, ImportListModeChirho, ModuleChirho,
};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::occurrences_chirho::{
    OccurrenceMutChirho, visit_decl_and_statements_chirho, visit_decl_chirho,
};
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};

use crate::env_chirho::NameEnvChirho;
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};
use crate::resolve_chirho::UNDEFINED_VALUE_CODE_CHIRHO;
use crate::type_exports_chirho::{canonical_type_name_chirho, canonical_value_name_chirho};

/// What the explicit lists behind one closed qualifier supply.
#[derive(Debug, Default)]
struct ClosedQualifierChirho {
    /// Canonical value names the lists supply.
    names_chirho: HashSet<String>,
    /// The modules whose lists these are, in import order.
    modules_chirho: Vec<String>,
}

/// Check every qualified value occurrence written in the module's expressions
/// against the qualifiers its explicit import lists close.
///
/// Workflow: see the value-scope resolution DAG under
/// `spec-chirho/workflows-chirho/compiler-pipeline-chirho`.
pub(crate) fn check_module_value_scope_chirho(
    module_chirho: &ModuleChirho,
    env_chirho: &NameEnvChirho,
    available_modules_chirho: &[ModuleIfaceChirho],
    diagnostics_chirho: &mut DiagnosticBundleChirho,
) {
    let closed_chirho =
        closed_qualifiers_chirho(&module_chirho.imports_chirho, available_modules_chirho);
    if closed_chirho.is_empty() {
        return;
    }
    let module_name_chirho = module_chirho.name_chirho.full_name_chirho();
    for name_chirho in written_qualified_references_chirho(&module_chirho.decls_chirho) {
        let Some(qualifier_chirho) = name_chirho.qualifier_chirho() else {
            continue;
        };
        let Some(closed_entry_chirho) = closed_chirho.get(qualifier_chirho) else {
            continue;
        };
        let value_name_chirho = canonical_value_name_chirho(name_chirho.text_chirho());
        if closed_entry_chirho
            .names_chirho
            .contains(&value_name_chirho)
            || own_top_level_value_chirho(
                qualifier_chirho,
                &module_name_chirho,
                &value_name_chirho,
                env_chirho,
            )
        {
            continue;
        }
        report_unlisted_chirho(
            diagnostics_chirho,
            &name_chirho,
            qualifier_chirho,
            closed_entry_chirho,
        );
    }
}

/// The qualifiers every one of whose imports is an explicit list of
/// represented items, each with what those lists supply. A qualifier absent
/// from the result is open.
fn closed_qualifiers_chirho(
    imports_chirho: &[ImportDeclChirho],
    available_modules_chirho: &[ModuleIfaceChirho],
) -> HashMap<String, ClosedQualifierChirho> {
    let mut open_chirho: HashSet<String> = HashSet::new();
    let mut closed_chirho: HashMap<String, ClosedQualifierChirho> = HashMap::new();
    for import_chirho in imports_chirho {
        let qualifier_chirho = import_chirho.effective_qualifier_chirho();
        if open_chirho.contains(&qualifier_chirho) {
            continue;
        }
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
        let supplied_chirho = match import_chirho.list_mode_chirho() {
            ImportListModeChirho::UnrestrictedChirho | ImportListModeChirho::HidingChirho(_) => {
                None
            }
            ImportListModeChirho::ExplicitChirho(items_chirho) => {
                // The interface name resolution reads: the latest of that name.
                let exports_chirho = available_modules_chirho
                    .iter()
                    .rev()
                    .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
                    .map(|iface_chirho| &iface_chirho.exports_chirho);
                explicit_list_supply_chirho(items_chirho, exports_chirho)
            }
        };
        match supplied_chirho {
            Some(names_chirho) => {
                let entry_chirho = closed_chirho.entry(qualifier_chirho).or_default();
                entry_chirho.names_chirho.extend(names_chirho);
                if !entry_chirho.modules_chirho.contains(&module_name_chirho) {
                    entry_chirho.modules_chirho.push(module_name_chirho);
                }
            }
            None => {
                closed_chirho.remove(&qualifier_chirho);
                open_chirho.insert(qualifier_chirho);
            }
        }
    }
    closed_chirho
}

/// What one explicit list supplies, or `None` when a listed `T(..)` cannot be
/// read from a complete member list and so leaves the qualifier open.
fn explicit_list_supply_chirho(
    items_chirho: &[ImportItemChirho],
    exports_chirho: Option<&IfaceExportsChirho>,
) -> Option<HashSet<String>> {
    let mut names_chirho = HashSet::new();
    for item_chirho in items_chirho {
        match item_chirho {
            // Directly listed: supplied even where the interface omits it.
            ImportItemChirho::VarChirho(name_chirho) => {
                names_chirho.insert(canonical_value_name_chirho(name_chirho.text_chirho()));
            }
            ImportItemChirho::TyConChirho {
                name_chirho,
                members_chirho,
            } => {
                // The type or class name itself supplies no value: `import M (T)`
                // brings the type, never a constructor that shares its spelling
                // (GHC 9.14.1, GHC-01928). Only members supply values.
                match members_chirho {
                    ExportMembersChirho::NoneChirho => {}
                    ExportMembersChirho::SomeChirho(member_names_chirho) => {
                        names_chirho.extend(member_names_chirho.iter().map(|member_chirho| {
                            canonical_value_name_chirho(member_chirho.text_chirho())
                        }));
                    }
                    ExportMembersChirho::AllChirho => {
                        let type_name_chirho =
                            canonical_type_name_chirho(name_chirho.text_chirho());
                        let entry_chirho = exports_chirho
                            .filter(|iface_exports_chirho| {
                                iface_exports_chirho
                                    .complete_member_types_chirho
                                    .contains(&type_name_chirho)
                            })
                            .and_then(|iface_exports_chirho| {
                                iface_exports_chirho.types_chirho.get(&type_name_chirho)
                            })?;
                        names_chirho.extend(
                            entry_chirho
                                .constructors_chirho
                                .iter()
                                .chain(&entry_chirho.methods_chirho)
                                .map(|member_chirho| {
                                    canonical_value_name_chirho(member_chirho.as_str())
                                }),
                        );
                    }
                }
            }
            // A list classified EXPLICIT holds no such item. Were one to reach
            // here, it would leave the qualifier open, never close it.
            ImportItemChirho::UnrepresentedChirho(_) => return None,
        }
    }
    Some(names_chirho)
}

/// The current module's own name qualifies its own top-level values.
fn own_top_level_value_chirho(
    qualifier_chirho: &str,
    module_name_chirho: &str,
    value_name_chirho: &str,
    env_chirho: &NameEnvChirho,
) -> bool {
    qualifier_chirho == module_name_chirho
        && env_chirho
            .lookup_value_chirho(value_name_chirho)
            .is_some_and(|info_chirho| !info_chirho.imported_chirho)
}

/// Every qualified reference written in these declarations' expressions:
/// variables, operators and constructors, a record construction's included.
/// The operations a do statement selects (`M.do`'s `M.>>=`) are not written
/// there and are left out, as are patterns and record labels, which the
/// occurrence walk does not visit.
fn written_qualified_references_chirho(decls_chirho: &[DeclChirho]) -> Vec<NameChirho> {
    // The shared walk needs mutable declarations; naming reads a copy.
    let mut copies_chirho = decls_chirho.to_vec();
    for decl_chirho in &mut copies_chirho {
        visit_decl_and_statements_chirho(
            decl_chirho,
            &mut |_occurrence_chirho| {},
            // Every statement form is named, so a new one must be classified.
            &mut |stmt_chirho: &mut StmtChirho| match stmt_chirho {
                StmtChirho::ExprChirho { then_chirho, .. } => *then_chirho = None,
                StmtChirho::BindChirho {
                    bind_chirho,
                    fail_chirho,
                    ..
                } => {
                    *bind_chirho = None;
                    *fail_chirho = None;
                }
                StmtChirho::LetChirho { .. } => {}
            },
        );
    }
    let mut references_chirho = Vec::new();
    for decl_chirho in &mut copies_chirho {
        visit_decl_chirho(decl_chirho, &mut |occurrence_chirho| {
            if let OccurrenceMutChirho::ReferenceChirho(name_chirho) = occurrence_chirho
                && name_chirho.qualifier_chirho().is_some()
            {
                references_chirho.push(name_chirho.clone());
            }
        });
    }
    references_chirho
}

fn report_unlisted_chirho(
    diagnostics_chirho: &mut DiagnosticBundleChirho,
    name_chirho: &NameChirho,
    qualifier_chirho: &str,
    closed_entry_chirho: &ClosedQualifierChirho,
) {
    let text_chirho = name_chirho.text_chirho();
    let kind_chirho = if text_chirho
        .starts_with(|char_chirho: char| char_chirho.is_uppercase() || char_chirho == ':')
    {
        "data constructor"
    } else {
        "variable"
    };
    diagnostics_chirho.push_chirho(
        DiagnosticChirho::error_with_code_chirho(
            ErrorCodeChirho::error_chirho(UNDEFINED_VALUE_CODE_CHIRHO),
            format!(
                "{kind_chirho} not in scope: `{}`",
                name_chirho.full_name_chirho()
            ),
            name_chirho.span_chirho(),
        )
        .with_note_chirho(format!(
            "`{qualifier_chirho}` is bound only by explicit import lists ({}); \
             none of them supplies `{text_chirho}`",
            closed_entry_chirho.modules_chirho.join(", ")
        )),
    );
}

#[cfg(test)]
#[path = "value_scope_tests_chirho.rs"]
mod tests_chirho;
