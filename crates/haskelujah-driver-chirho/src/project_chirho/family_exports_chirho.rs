// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written family scope and identity-preserving export publication (row 686b).
//! Import admission belongs to naming; exported family aliases retain defining
//! identities rather than selecting the first matching unqualified spelling.
//! Workflow: language-features-chirho/stuck-family-equalities-chirho.md.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use haskelujah_ast_chirho::decl_chirho::{ConstructorSignatureErrorChirho, DeclChirho};
use haskelujah_ast_chirho::module_chirho::{ExportMembersChirho, ExportSpecChirho, ModuleChirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_naming_chirho::env_chirho::NamespaceChirho;
use haskelujah_naming_chirho::iface_chirho::ModuleIfaceChirho;
use haskelujah_naming_chirho::resolve_chirho::compute_imported_names_chirho;
use haskelujah_typing_chirho::infer_chirho::{
    InferResultChirho, TypeFamilyDefinitionErrorChirho, TypeFamilyIdChirho,
    TypeFamilyRegistryChirho, TypeFamilyScopeChirho, TypeFamilyScopeResolutionChirho,
};

#[derive(Debug)]
pub(crate) enum FamilyExportsErrorChirho {
    ConstructorSignatureChirho {
        module_chirho: String,
        error_chirho: ConstructorSignatureErrorChirho,
    },
    ScopeChirho(TypeFamilyDefinitionErrorChirho),
    ConflictingLocalTypeChirho(String),
    MissingDefinitionChirho(String),
    UnstructuredQualifiedNameChirho(String),
    MismatchedInterfaceChirho {
        module_chirho: String,
        interface_chirho: String,
    },
    UnadmittedAssociatedFamilyChirho {
        parent_chirho: String,
        member_chirho: String,
    },
}

impl fmt::Display for FamilyExportsErrorChirho {
    fn fmt(&self, formatter_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConstructorSignatureChirho {
                module_chirho,
                error_chirho,
            } => write!(
                formatter_chirho,
                "constructor signature in module {module_chirho}: {error_chirho}"
            ),
            Self::ScopeChirho(error_chirho) => fmt::Display::fmt(error_chirho, formatter_chirho),
            Self::ConflictingLocalTypeChirho(name_chirho) => write!(
                formatter_chirho,
                "conflicting local type declarations for family scope: {name_chirho}"
            ),
            Self::MissingDefinitionChirho(key_chirho) => write!(
                formatter_chirho,
                "cannot publish a family export before its definition exists: {key_chirho}"
            ),
            Self::UnstructuredQualifiedNameChirho(name_chirho) => write!(
                formatter_chirho,
                "family export needs a structurally retained qualifier: {name_chirho}"
            ),
            Self::MismatchedInterfaceChirho {
                module_chirho,
                interface_chirho,
            } => write!(
                formatter_chirho,
                "cannot publish {module_chirho}'s families from {interface_chirho}'s interface"
            ),
            Self::UnadmittedAssociatedFamilyChirho {
                parent_chirho,
                member_chirho,
            } => write!(
                formatter_chirho,
                "associated type family {member_chirho} is not in scope for {parent_chirho}"
            ),
        }
    }
}

impl std::error::Error for FamilyExportsErrorChirho {}

impl FamilyExportsErrorChirho {
    /// Project callers return strings, so preserve the real source path and
    /// signature location before crossing that existing error boundary.
    pub(crate) fn source_message_chirho(
        &self,
        path_chirho: &str,
        source_map_chirho: &haskelujah_span_chirho::SourceMapChirho,
    ) -> String {
        if let Self::ConstructorSignatureChirho { error_chirho, .. } = self {
            let span_chirho = error_chirho.span_chirho();
            if let Some(index_chirho) =
                source_map_chirho.line_index_chirho(span_chirho.file_id_chirho())
            {
                let location_chirho = index_chirho.line_col_chirho(span_chirho.start_chirho());
                return format!(
                    "{path_chirho}:{}:{}: {self}",
                    location_chirho.line_chirho, location_chirho.col_chirho
                );
            }
            return format!(
                "{path_chirho}: {self} (signature byte range {}..{})",
                span_chirho.start_chirho().as_u32_chirho(),
                span_chirho.end_chirho().as_u32_chirho()
            );
        }
        format!("{path_chirho}: {self}")
    }

    /// Signature failures retain their producer's span. Other family errors
    /// still use the caller's existing declaration context.
    pub(crate) fn into_diagnostics_chirho(
        self,
        fallback_span_chirho: haskelujah_span_chirho::SpanChirho,
    ) -> DiagnosticBundleChirho {
        match self {
            Self::ConstructorSignatureChirho {
                module_chirho,
                error_chirho,
            } => DiagnosticBundleChirho::new_chirho(vec![constructor_signature_diagnostic_chirho(
                error_chirho,
                &module_chirho,
            )]),
            error_chirho => family_frontend_diagnostics_chirho(error_chirho, fallback_span_chirho),
        }
    }
}

/// Malformed source and a bounded projection refusal remain distinct errors.
/// The caller owns any foreign-source rendering note; this preserves the
/// producer's actual span and never substitutes the importing module's span.
pub(crate) fn constructor_signature_diagnostic_chirho(
    error_chirho: ConstructorSignatureErrorChirho,
    module_chirho: &str,
) -> DiagnosticChirho {
    let message_chirho = format!("constructor signature in module {module_chirho}: {error_chirho}");
    let mut diagnostic_chirho = match error_chirho {
        ConstructorSignatureErrorChirho::MalformedChirho { .. } => {
            DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(209),
                message_chirho,
                error_chirho.span_chirho(),
            )
        }
        ConstructorSignatureErrorChirho::ProjectionChirho(_) => {
            DiagnosticChirho::error_chirho(message_chirho, error_chirho.span_chirho())
        }
    };
    if let Some(code_chirho) = error_chirho.ghc_code_chirho() {
        diagnostic_chirho =
            diagnostic_chirho.with_note_chirho(format!("GHC reports this as {code_chirho}"));
    }
    diagnostic_chirho
}

impl From<TypeFamilyDefinitionErrorChirho> for FamilyExportsErrorChirho {
    fn from(error_chirho: TypeFamilyDefinitionErrorChirho) -> Self {
        Self::ScopeChirho(error_chirho)
    }
}

pub(crate) fn family_frontend_diagnostics_chirho(
    error_chirho: impl fmt::Display,
    span_chirho: haskelujah_span_chirho::SpanChirho,
) -> DiagnosticBundleChirho {
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();
    diagnostics_chirho.push_chirho(DiagnosticChirho::error_with_code_chirho(
        ErrorCodeChirho::error_chirho(209),
        format!("type-family identity: {error_chirho}"),
        span_chirho,
    ));
    diagnostics_chirho
}

pub(crate) fn stamp_exported_family_references_chirho(
    ty_chirho: &mut TypeChirho,
    infer_result_chirho: &InferResultChirho,
    module_name_chirho: &str,
) {
    if let Some(scope_chirho) = infer_result_chirho.family_scope_chirho.as_deref() {
        haskelujah_typing_chirho::infer_chirho::stamp_family_references_chirho(
            ty_chirho,
            scope_chirho,
            &infer_result_chirho.type_families_chirho,
            module_name_chirho,
        );
    }
}

/// One frontend publication point covers checks, projects and dependency builds.
pub(crate) fn publish_inferred_family_exports_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    infer_result_chirho: &mut InferResultChirho,
) -> Result<(), FamilyExportsErrorChirho> {
    let scope_chirho = if let Some(scope_chirho) = &infer_result_chirho.family_scope_chirho {
        std::sync::Arc::clone(scope_chirho)
    } else {
        std::sync::Arc::new(build_type_family_scope_chirho(
            module_chirho,
            ifaces_chirho,
            &infer_result_chirho.type_families_chirho,
        )?)
    };
    let iface_chirho = build_family_aware_iface_chirho(
        module_chirho,
        ifaces_chirho,
        &infer_result_chirho.type_families_chirho,
    )?;
    publish_type_family_exports_chirho(
        module_chirho,
        &iface_chirho,
        ifaces_chirho,
        &scope_chirho,
        &mut infer_result_chirho.type_families_chirho,
    )
}

/// Typed callers and publication use the same interface builder. Family origin
/// evidence comes only from the registry; naming never interprets its keys.
pub(crate) fn build_family_aware_iface_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    registry_chirho: &TypeFamilyRegistryChirho,
) -> Result<ModuleIfaceChirho, FamilyExportsErrorChirho> {
    let resolve_chirho = |module_name_chirho: &str, name_chirho: &str| {
        match registry_chirho.resolve_export_chirho(&qualified_name_chirho(module_name_chirho, name_chirho)) {
            haskelujah_typing_chirho::infer_chirho::TypeFamilyAliasResolutionChirho::MissingChirho => BTreeSet::new(),
            haskelujah_typing_chirho::infer_chirho::TypeFamilyAliasResolutionChirho::UniqueChirho(definition_chirho) => BTreeSet::from([definition_chirho.id_chirho().key_chirho()]),
            haskelujah_typing_chirho::infer_chirho::TypeFamilyAliasResolutionChirho::AmbiguousChirho(identities_chirho) => identities_chirho.iter().map(TypeFamilyIdChirho::key_chirho).collect(),
        }
    };
    haskelujah_naming_chirho::iface_chirho::build_iface_with_imports_and_family_identities_chirho(
        module_chirho,
        ifaces_chirho,
        &resolve_chirho,
    )
    .map_err(
        |error_chirho| FamilyExportsErrorChirho::ConstructorSignatureChirho {
            module_chirho: module_chirho.name_chirho.full_name_chirho(),
            error_chirho,
        },
    )
}

/// Only syntax wrappers are removed. A qualifier is never recovered by suffix.
fn canonical_type_text_chirho(text_chirho: &str) -> String {
    let text_chirho = text_chirho.strip_prefix("type ").unwrap_or(text_chirho);
    if let Some(inner_chirho) = text_chirho
        .strip_prefix('(')
        .and_then(|inner_chirho| inner_chirho.strip_suffix(')'))
        && !inner_chirho.is_empty()
        && inner_chirho
            .chars()
            .all(|character_chirho| !character_chirho.is_alphanumeric() && character_chirho != '_')
    {
        inner_chirho.to_owned()
    } else {
        text_chirho.to_owned()
    }
}

fn type_name_parts_chirho(
    name_chirho: &NameChirho,
    local_chirho: &LocalTypeNamesChirho,
    imports_chirho: &[AdmittedFamilyImportChirho<'_>],
    module_name_chirho: &str,
) -> Result<(Option<String>, String), FamilyExportsErrorChirho> {
    let text_chirho = canonical_type_text_chirho(name_chirho.text_chirho());
    if name_chirho.qualifier_chirho().is_none()
        && text_chirho.contains('.')
        && text_chirho.chars().next().is_some_and(char::is_uppercase)
    {
        // Existing lowering can leave Q.:.: as one raw spelling. Match that
        // complete spelling against admitted qualifier/name pairs; never take
        // its last dot as evidence of where an operator's qualifier ends.
        let mut candidates_chirho = BTreeSet::new();
        for import_chirho in imports_chirho {
            for admitted_chirho in &import_chirho.types_chirho {
                if qualified_name_chirho(&import_chirho.qualifier_chirho, admitted_chirho)
                    == text_chirho
                {
                    candidates_chirho.insert((
                        import_chirho.qualifier_chirho.clone(),
                        admitted_chirho.clone(),
                    ));
                }
            }
        }
        for local_name_chirho in local_chirho.bindings_chirho.keys() {
            if qualified_name_chirho(module_name_chirho, local_name_chirho) == text_chirho {
                candidates_chirho
                    .insert((module_name_chirho.to_owned(), local_name_chirho.clone()));
            }
        }
        if candidates_chirho.len() == 1 {
            let (qualifier_chirho, text_chirho) = candidates_chirho
                .pop_first()
                .expect("one admitted type spelling");
            return Ok((Some(qualifier_chirho), text_chirho));
        }
        return Err(FamilyExportsErrorChirho::UnstructuredQualifiedNameChirho(
            name_chirho.full_name_chirho(),
        ));
    }
    Ok((
        name_chirho.qualifier_chirho().map(str::to_owned),
        text_chirho,
    ))
}

fn qualified_name_chirho(qualifier_chirho: &str, name_chirho: &str) -> String {
    format!("{qualifier_chirho}.{name_chirho}")
}

#[derive(Default)]
struct LocalTypeNamesChirho {
    bindings_chirho: BTreeMap<String, Option<TypeFamilyIdChirho>>,
    associated_chirho: BTreeMap<String, BTreeSet<String>>,
}

impl LocalTypeNamesChirho {
    fn insert_chirho(
        &mut self,
        name_chirho: String,
        family_chirho: Option<TypeFamilyIdChirho>,
    ) -> Result<(), FamilyExportsErrorChirho> {
        if let Some(previous_chirho) = self.bindings_chirho.get(&name_chirho) {
            if previous_chirho != &family_chirho {
                return Err(FamilyExportsErrorChirho::ConflictingLocalTypeChirho(
                    name_chirho,
                ));
            }
        } else {
            self.bindings_chirho.insert(name_chirho, family_chirho);
        }
        Ok(())
    }
}

fn local_type_names_chirho(
    module_chirho: &ModuleChirho,
) -> Result<LocalTypeNamesChirho, FamilyExportsErrorChirho> {
    let module_name_chirho = module_chirho.name_chirho.full_name_chirho();
    let mut names_chirho = LocalTypeNamesChirho::default();
    for declaration_chirho in &module_chirho.decls_chirho {
        match declaration_chirho {
            DeclChirho::DataDeclChirho { name_chirho, .. }
            | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
            | DeclChirho::TypeAliasDeclChirho { name_chirho, .. } => {
                names_chirho
                    .insert_chirho(canonical_type_text_chirho(name_chirho.text_chirho()), None)?;
            }
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                data_family_chirho,
                ..
            } => {
                let name_chirho = canonical_type_text_chirho(name_chirho.text_chirho());
                names_chirho.insert_chirho(
                    name_chirho.clone(),
                    (!*data_family_chirho).then_some(TypeFamilyIdChirho::SourceChirho {
                        module_chirho: module_name_chirho.clone(),
                        name_chirho,
                    }),
                )?;
            }
            DeclChirho::ClassDeclChirho {
                name_chirho,
                associated_tfs_chirho,
                ..
            } => {
                let parent_chirho = canonical_type_text_chirho(name_chirho.text_chirho());
                names_chirho.insert_chirho(parent_chirho.clone(), None)?;
                for family_chirho in associated_tfs_chirho {
                    let name_chirho =
                        canonical_type_text_chirho(family_chirho.name_chirho.text_chirho());
                    names_chirho.insert_chirho(
                        name_chirho.clone(),
                        (!family_chirho.data_family_chirho).then_some(
                            TypeFamilyIdChirho::SourceChirho {
                                module_chirho: module_name_chirho.clone(),
                                name_chirho: name_chirho.clone(),
                            },
                        ),
                    )?;
                    names_chirho
                        .associated_chirho
                        .entry(parent_chirho.clone())
                        .or_default()
                        .insert(name_chirho);
                }
            }
            _ => {}
        }
    }
    Ok(names_chirho)
}

struct AdmittedFamilyImportChirho<'iface_chirho> {
    iface_chirho: &'iface_chirho ModuleIfaceChirho,
    qualifier_chirho: String,
    unqualified_chirho: bool,
    types_chirho: BTreeSet<String>,
}

fn admitted_family_imports_chirho<'iface_chirho>(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &'iface_chirho [ModuleIfaceChirho],
) -> Vec<AdmittedFamilyImportChirho<'iface_chirho>> {
    module_chirho
        .imports_chirho
        .iter()
        .filter_map(|import_chirho| {
            // Match the frontend's last-interface selection. Missing modules are
            // diagnosed by naming; they cannot contribute guessed family names.
            let iface_chirho = haskelujah_naming_chirho::iface_chirho::select_import_iface_chirho(
                ifaces_chirho,
                import_chirho,
            )?;
            let types_chirho = compute_imported_names_chirho(
                &iface_chirho.exports_chirho,
                &import_chirho.spec_chirho,
            )
            .into_iter()
            .filter_map(|(name_chirho, namespace_chirho, _span_chirho)| {
                (namespace_chirho == NamespaceChirho::TypeChirho).then_some(name_chirho)
            })
            .collect();
            Some(AdmittedFamilyImportChirho {
                iface_chirho,
                qualifier_chirho: import_chirho.effective_qualifier_chirho(),
                unqualified_chirho: !import_chirho.qualified_chirho,
                types_chirho,
            })
        })
        .collect()
}

/// Build before branch registration: local source IDs already identify forward
/// references, and any local type declaration shadows an imported family.
pub(crate) fn build_type_family_scope_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    registry_chirho: &TypeFamilyRegistryChirho,
) -> Result<TypeFamilyScopeChirho, FamilyExportsErrorChirho> {
    let mut scope_chirho = TypeFamilyScopeChirho::new_chirho();
    for import_chirho in admitted_family_imports_chirho(module_chirho, ifaces_chirho) {
        for name_chirho in &import_chirho.types_chirho {
            let export_chirho =
                qualified_name_chirho(&import_chirho.iface_chirho.name_chirho, name_chirho);
            scope_chirho.import_export_chirho(
                qualified_name_chirho(&import_chirho.qualifier_chirho, name_chirho),
                &export_chirho,
                registry_chirho,
            );
            if import_chirho.unqualified_chirho {
                scope_chirho.import_export_chirho(
                    name_chirho.clone(),
                    &export_chirho,
                    registry_chirho,
                );
            }
        }
    }
    let module_name_chirho = module_chirho.name_chirho.full_name_chirho();
    for (name_chirho, family_chirho) in local_type_names_chirho(module_chirho)?.bindings_chirho {
        for written_chirho in [
            name_chirho.clone(),
            qualified_name_chirho(&module_name_chirho, &name_chirho),
        ] {
            if let Some(identity_chirho) = &family_chirho {
                scope_chirho.bind_local_family_chirho(written_chirho, identity_chirho)?;
            } else {
                scope_chirho.bind_other_local_type_chirho(written_chirho)?;
            }
        }
    }
    Ok(scope_chirho)
}

fn scoped_family_keys_chirho(
    scope_chirho: &TypeFamilyScopeChirho,
    written_chirho: &str,
    registry_chirho: &TypeFamilyRegistryChirho,
) -> BTreeSet<String> {
    match scope_chirho.resolve_chirho(written_chirho, registry_chirho) {
        TypeFamilyScopeResolutionChirho::MissingChirho => BTreeSet::new(),
        TypeFamilyScopeResolutionChirho::UniqueChirho(key_chirho) => {
            BTreeSet::from([key_chirho.to_owned()])
        }
        TypeFamilyScopeResolutionChirho::AmbiguousChirho(keys_chirho) => keys_chirho.clone(),
    }
}

fn selected_member_chirho(members_chirho: &ExportMembersChirho, name_chirho: &str) -> bool {
    match members_chirho {
        ExportMembersChirho::AllChirho => true,
        ExportMembersChirho::SomeChirho(names_chirho) => {
            names_chirho.iter().any(|selected_chirho| {
                canonical_type_text_chirho(selected_chirho.text_chirho()) == name_chirho
                    && (selected_chirho.is_explicit_type_member_chirho()
                        || name_chirho.starts_with(':')
                        || name_chirho.chars().any(|character_chirho| {
                            character_chirho.is_alphanumeric() || character_chirho == '_'
                        }))
            })
        }
        ExportMembersChirho::NoneChirho => false,
    }
}

type FamilyExportKeysChirho = BTreeMap<String, BTreeSet<String>>;

fn add_export_keys_chirho(
    exports_chirho: &mut FamilyExportKeysChirho,
    iface_chirho: &ModuleIfaceChirho,
    name_chirho: &str,
    keys_chirho: BTreeSet<String>,
) {
    if iface_chirho
        .exports_chirho
        .types_chirho
        .contains_key(name_chirho)
    {
        exports_chirho
            .entry(name_chirho.to_owned())
            .or_default()
            .extend(keys_chirho);
    }
}

/// Follow the selected parent's association, intersected with the member's
/// admitted scope under any qualifier. GHC allows Q.C(F) when the same F is
/// imported as R.F or unqualified; the selected parent still fixes its origin.
fn associated_export_keys_chirho(
    parent_chirho: (&Option<String>, &str),
    members_chirho: &ExportMembersChirho,
    local_chirho: &LocalTypeNamesChirho,
    imports_chirho: &[AdmittedFamilyImportChirho<'_>],
    module_name_chirho: &str,
    scope_chirho: &TypeFamilyScopeChirho,
    registry_chirho: &TypeFamilyRegistryChirho,
) -> Result<FamilyExportKeysChirho, FamilyExportsErrorChirho> {
    let (qualifier_chirho, name_chirho) = parent_chirho;
    let mut exports_chirho = FamilyExportKeysChirho::new();
    if qualifier_chirho
        .as_deref()
        .is_none_or(|qualifier_chirho| qualifier_chirho == module_name_chirho)
        && local_chirho.bindings_chirho.contains_key(name_chirho)
    {
        if let Some(associated_chirho) = local_chirho.associated_chirho.get(name_chirho) {
            for member_chirho in associated_chirho {
                if selected_member_chirho(members_chirho, member_chirho)
                    && let Some(Some(identity_chirho)) =
                        local_chirho.bindings_chirho.get(member_chirho)
                {
                    exports_chirho
                        .entry(member_chirho.clone())
                        .or_default()
                        .insert(identity_chirho.key_chirho());
                }
            }
        }
        return Ok(exports_chirho);
    }
    for import_chirho in imports_chirho {
        let supplies_parent_chirho = qualifier_chirho
            .as_deref()
            .map_or(import_chirho.unqualified_chirho, |qualifier_chirho| {
                qualifier_chirho == import_chirho.qualifier_chirho
            });
        if !supplies_parent_chirho || !import_chirho.types_chirho.contains(name_chirho) {
            continue;
        }
        let Some(associated_chirho) = import_chirho
            .iface_chirho
            .exports_chirho
            .associated_types_chirho
            .get(name_chirho)
        else {
            continue;
        };
        for member_chirho in associated_chirho {
            if !selected_member_chirho(members_chirho, member_chirho) {
                continue;
            }
            let mut imported_scope_chirho = TypeFamilyScopeChirho::new_chirho();
            imported_scope_chirho.import_export_chirho(
                member_chirho.clone(),
                &qualified_name_chirho(&import_chirho.iface_chirho.name_chirho, member_chirho),
                registry_chirho,
            );
            let origins_chirho =
                scoped_family_keys_chirho(&imported_scope_chirho, member_chirho, registry_chirho);
            let mut admitted_chirho = BTreeSet::new();
            for supplied_chirho in imports_chirho {
                if supplied_chirho.types_chirho.contains(member_chirho) {
                    admitted_chirho.extend(scoped_family_keys_chirho(
                        scope_chirho,
                        &qualified_name_chirho(&supplied_chirho.qualifier_chirho, member_chirho),
                        registry_chirho,
                    ));
                }
            }
            let selected_chirho = origins_chirho
                .intersection(&admitted_chirho)
                .cloned()
                .collect::<BTreeSet<_>>();
            if !origins_chirho.is_empty()
                && selected_chirho.is_empty()
                && matches!(members_chirho, ExportMembersChirho::SomeChirho(_))
            {
                return Err(FamilyExportsErrorChirho::UnadmittedAssociatedFamilyChirho {
                    parent_chirho: qualifier_chirho.as_ref().map_or_else(
                        || name_chirho.to_owned(),
                        |qualifier_chirho| qualified_name_chirho(qualifier_chirho, name_chirho),
                    ),
                    member_chirho: member_chirho.clone(),
                });
            }
            exports_chirho
                .entry(member_chirho.clone())
                .or_default()
                .extend(selected_chirho);
        }
    }
    Ok(exports_chirho)
}

/// Publish only the families that the actual interface and written export list
/// both export. Every target is checked before the registry changes. Replacing
/// this module's owned publication retires removed aliases without a prefix scan.
pub(crate) fn publish_type_family_exports_chirho(
    module_chirho: &ModuleChirho,
    iface_chirho: &ModuleIfaceChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    scope_chirho: &TypeFamilyScopeChirho,
    registry_chirho: &mut TypeFamilyRegistryChirho,
) -> Result<(), FamilyExportsErrorChirho> {
    let module_name_chirho = module_chirho.name_chirho.full_name_chirho();
    if iface_chirho.name_chirho != module_name_chirho {
        return Err(FamilyExportsErrorChirho::MismatchedInterfaceChirho {
            module_chirho: module_name_chirho,
            interface_chirho: iface_chirho.name_chirho.clone(),
        });
    }
    let local_chirho = local_type_names_chirho(module_chirho)?;
    let imports_chirho = admitted_family_imports_chirho(module_chirho, ifaces_chirho);
    let mut exports_chirho = FamilyExportKeysChirho::new();
    match &module_chirho.exports_chirho {
        None => {
            for (name_chirho, family_chirho) in &local_chirho.bindings_chirho {
                if let Some(identity_chirho) = family_chirho {
                    add_export_keys_chirho(
                        &mut exports_chirho,
                        iface_chirho,
                        name_chirho,
                        BTreeSet::from([identity_chirho.key_chirho()]),
                    );
                }
            }
        }
        Some(specifications_chirho) => {
            for specification_chirho in specifications_chirho {
                match specification_chirho {
                    ExportSpecChirho::VarChirho(name_chirho)
                    | ExportSpecChirho::TyConChirho { name_chirho, .. } => {
                        if matches!(specification_chirho, ExportSpecChirho::VarChirho(_))
                            && haskelujah_naming_chirho::iface_chirho::is_qualified_variable_symbol_export_chirho(name_chirho)
                        {
                            continue;
                        }
                        if matches!(specification_chirho, ExportSpecChirho::VarChirho(_))
                            && !iface_chirho.exports_chirho.types_chirho.contains_key(
                                &canonical_type_text_chirho(name_chirho.text_chirho()),
                            )
                            && scoped_family_keys_chirho(
                                scope_chirho,
                                &name_chirho.full_name_chirho(),
                                registry_chirho,
                            )
                            .is_empty()
                        {
                            // Bare export syntax also carries values. A lossy
                            // value-operator name is not this layer's type use.
                            continue;
                        }
                        let (qualifier_chirho, name_chirho) = type_name_parts_chirho(
                            name_chirho,
                            &local_chirho,
                            &imports_chirho,
                            &module_name_chirho,
                        )?;
                        let written_chirho = qualifier_chirho.as_ref().map_or_else(
                            || name_chirho.clone(),
                            |qualifier_chirho| {
                                qualified_name_chirho(qualifier_chirho, &name_chirho)
                            },
                        );
                        add_export_keys_chirho(
                            &mut exports_chirho,
                            iface_chirho,
                            &name_chirho,
                            scoped_family_keys_chirho(
                                scope_chirho,
                                &written_chirho,
                                registry_chirho,
                            ),
                        );
                        if let ExportSpecChirho::TyConChirho { members_chirho, .. } =
                            specification_chirho
                        {
                            for (member_chirho, keys_chirho) in associated_export_keys_chirho(
                                (&qualifier_chirho, &name_chirho),
                                members_chirho,
                                &local_chirho,
                                &imports_chirho,
                                &module_name_chirho,
                                scope_chirho,
                                registry_chirho,
                            )? {
                                add_export_keys_chirho(
                                    &mut exports_chirho,
                                    iface_chirho,
                                    &member_chirho,
                                    keys_chirho,
                                );
                            }
                        }
                    }
                    ExportSpecChirho::ModuleChirho(name_chirho) => {
                        let qualifier_chirho = name_chirho.full_name_chirho();
                        for name_chirho in iface_chirho.exports_chirho.types_chirho.keys() {
                            let bare_chirho = scoped_family_keys_chirho(
                                scope_chirho,
                                name_chirho,
                                registry_chirho,
                            );
                            let qualified_chirho = scoped_family_keys_chirho(
                                scope_chirho,
                                &qualified_name_chirho(&qualifier_chirho, name_chirho),
                                registry_chirho,
                            );
                            add_export_keys_chirho(
                                &mut exports_chirho,
                                iface_chirho,
                                name_chirho,
                                bare_chirho
                                    .intersection(&qualified_chirho)
                                    .cloned()
                                    .collect(),
                            );
                        }
                    }
                }
            }
        }
    }
    // Replacement, including empty sets, prevents a previous M.F origin from
    // surviving when F becomes an ordinary type or changes forwarding path.
    let mut plan_chirho = Vec::new();
    for name_chirho in iface_chirho
        .exports_chirho
        .types_chirho
        .keys()
        .collect::<BTreeSet<_>>()
    {
        let mut identities_chirho = BTreeSet::new();
        for key_chirho in exports_chirho.get(name_chirho).into_iter().flatten() {
            let definition_chirho =
                registry_chirho
                    .definition_chirho(key_chirho)
                    .ok_or_else(|| {
                        FamilyExportsErrorChirho::MissingDefinitionChirho(key_chirho.clone())
                    })?;
            identities_chirho.insert(definition_chirho.id_chirho().clone());
        }
        plan_chirho.push((
            qualified_name_chirho(&module_name_chirho, name_chirho),
            identities_chirho,
        ));
    }
    registry_chirho.replace_module_exports_chirho(&iface_chirho.name_chirho, &plan_chirho)?;
    Ok(())
}

#[cfg(test)]
#[path = "family_exports_tests_chirho.rs"]
mod tests_chirho;
