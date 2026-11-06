// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Import qualification for the frontend, moved from the driver root (row 691, L4): qualifying
//! imported types and schemes by module alias, the safe unqualified and preferred qualified type
//! names and synonym keys an import brings, and when an imported scheme overrides a placeholder.

use super::*;

/// The scheme a qualified import gives `name_chirho` when the interface it selected has no
/// `module.name` key (row 608). Only an interface proven builtin reads the generated GHC agreement
/// table. A source module's interface, whatever its name, and any interface of unknown origin
/// keep today's placeholder.
/// workflow: compiler-pipeline-chirho/builtin-type-environment-chirho
pub fn qualified_import_alias_scheme_chirho(
    iface_chirho: &ModuleIfaceChirho,
    name_chirho: &str,
    builtins_chirho: &std::collections::HashMap<String, haskelujah_typing_chirho::SchemeChirho>,
) -> Option<haskelujah_typing_chirho::SchemeChirho> {
    if iface_chirho.exports_chirho.origin_chirho
        != haskelujah_naming_chirho::IfaceOriginChirho::BuiltinChirho
    {
        return None;
    }
    haskelujah_typing_chirho::infer_chirho::qualified_builtin_alias_scheme_chirho(
        &iface_chirho.name_chirho,
        name_chirho,
        builtins_chirho,
    )
}

fn collect_local_type_names_chirho(
    module_chirho: &ModuleChirho,
) -> std::collections::HashSet<String> {
    module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho { name_chirho, .. }
            | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
            | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
            | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. }
            | DeclChirho::ClassDeclChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect()
}

fn default_safe_unqualified_imported_type_names_chirho() -> std::collections::HashSet<String> {
    // These names are already modeled as shared runtime-facing types across
    // multiple re-export modules, so keeping them bare avoids spurious
    // mismatches like B.ByteString vs ByteString, I.IORef vs IORef, and
    // E.SomeException vs SomeException.
    ["ByteString", "Ordering", "IORef", "SomeException"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

pub(crate) fn collect_safe_unqualified_imported_type_names_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> std::collections::HashSet<String> {
    let local_type_names_chirho = collect_local_type_names_chirho(module_chirho);
    let mut safe_type_names_chirho = default_safe_unqualified_imported_type_names_chirho();
    safe_type_names_chirho.retain(|name_chirho| !local_type_names_chirho.contains(name_chirho));
    safe_type_names_chirho.extend(
        module_chirho
            .imports_chirho
            .iter()
            .filter(|import_chirho| !import_chirho.qualified_chirho)
            .flat_map(|import_chirho| {
                haskelujah_naming_chirho::iface_chirho::select_import_iface_chirho(
                    ifaces_chirho,
                    import_chirho,
                )
                .into_iter()
                .flat_map(|iface_chirho| {
                    haskelujah_naming_chirho::resolve_chirho::compute_imported_names_chirho(
                        &iface_chirho.exports_chirho,
                        &import_chirho.spec_chirho,
                    )
                })
            })
            .filter_map(|(name_chirho, namespace_chirho, _span_chirho)| {
                (namespace_chirho
                    == haskelujah_naming_chirho::env_chirho::NamespaceChirho::TypeChirho
                    && !local_type_names_chirho.contains(&name_chirho))
                .then_some(name_chirho)
            }),
    );
    safe_type_names_chirho
}

/// Give each type name a module may spell bare the bare synonym key that spelling needs (row 656).
///
/// A name with exactly one exporter among the module's imports is canonicalized to its bare
/// spelling, by the import rewrite and by inference alike, whether or not an import lists it. A
/// synonym so spelled therefore needs a bare key, or a re-exported alias stops expanding at a
/// parent the module never imported by name. The key is cloned from that exporter's qualified
/// key, only when no bare key exists and the qualified one is already in the merged input. It
/// serves expansion only: naming still decides what the source may write.
/// workflow: compiler-pipeline-chirho/type-scope-resolution-chirho
pub(crate) fn align_preferred_synonym_keys_chirho(
    merged_imported_type_synonyms_chirho: &mut ImportedTypeSynonymsChirho,
    preferred_qualified_type_names_chirho: &std::collections::HashMap<String, String>,
    budget_chirho: &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho,
    span_chirho: haskelujah_span_chirho::SpanChirho,
) -> Result<(), DiagnosticBundleChirho> {
    let names_chirho = std::collections::HashSet::new();
    let preferred_chirho = std::collections::HashMap::new();
    let mut transport_chirho = ImportQualificationChirho {
        iface_chirho: None,
        type_names_chirho: &names_chirho,
        qualifier_chirho: "",
        safe_chirho: &names_chirho,
        preferred_chirho: &preferred_chirho,
        budget_chirho,
        span_chirho,
    };
    for (type_name_chirho, exporter_chirho) in preferred_qualified_type_names_chirho {
        if merged_imported_type_synonyms_chirho.contains_key(type_name_chirho) {
            continue;
        }
        let type_name_chirho = transport_chirho.key_chirho(type_name_chirho)?;
        let exporter_chirho = transport_chirho.key_chirho(exporter_chirho)?;
        if let Some((parameters_chirho, rhs_chirho)) = merged_imported_type_synonyms_chirho
            .get(&format!("{exporter_chirho}.{type_name_chirho}"))
        {
            let copied_chirho = (
                transport_chirho.parameters_chirho(parameters_chirho)?,
                transport_chirho.ast_chirho(rhs_chirho)?,
            );
            merged_imported_type_synonyms_chirho.insert(type_name_chirho, copied_chirho);
        }
    }
    Ok(())
}

pub(crate) fn collect_preferred_qualified_type_names_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> std::collections::HashMap<String, String> {
    let local_type_names_chirho = collect_local_type_names_chirho(module_chirho);
    let mut exporters_by_type_name_chirho: std::collections::HashMap<
        String,
        std::collections::HashSet<(
            String,
            Option<haskelujah_naming_chirho::iface_chirho::IfaceExposureChirho>,
        )>,
    > = std::collections::HashMap::new();

    for import_chirho in &module_chirho.imports_chirho {
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
        let Some(iface_chirho) = haskelujah_naming_chirho::iface_chirho::select_import_iface_chirho(
            ifaces_chirho,
            import_chirho,
        ) else {
            continue;
        };
        for exported_type_name_chirho in iface_chirho.exports_chirho.types_chirho.keys() {
            if local_type_names_chirho.contains(exported_type_name_chirho) {
                continue;
            }
            exporters_by_type_name_chirho
                .entry(exported_type_name_chirho.clone())
                .or_default()
                .insert((
                    module_name_chirho.clone(),
                    iface_chirho.exposure_chirho.clone(),
                ));
        }
    }

    exporters_by_type_name_chirho
        .into_iter()
        .filter_map(|(type_name_chirho, exporters_chirho)| {
            (exporters_chirho.len() == 1).then(|| {
                (
                    type_name_chirho,
                    exporters_chirho
                        .into_iter()
                        .next()
                        .expect("single exporter should exist")
                        .0,
                )
            })
        })
        .collect()
}

/// One selected import and one frontend-wide hard transport allowance. Checked
/// names keep their producer metadata; only legacy lookup syntax is qualified.
pub(super) struct ImportQualificationChirho<'qualification_chirho> {
    pub iface_chirho: Option<&'qualification_chirho ModuleIfaceChirho>,
    pub type_names_chirho: &'qualification_chirho std::collections::HashSet<String>,
    pub qualifier_chirho: &'qualification_chirho str,
    pub safe_chirho: &'qualification_chirho std::collections::HashSet<String>,
    pub preferred_chirho: &'qualification_chirho std::collections::HashMap<String, String>,
    pub budget_chirho:
        &'qualification_chirho mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho,
    pub span_chirho: haskelujah_span_chirho::SpanChirho,
}

impl ImportQualificationChirho<'_> {
    pub fn key_chirho(&mut self, key_chirho: &str) -> Result<String, DiagnosticBundleChirho> {
        self.reserve_chirho(key_chirho.len().saturating_add(1), 0)?;
        Ok(key_chirho.to_owned())
    }

    pub fn parameters_chirho(
        &mut self,
        parameters_chirho: &[String],
    ) -> Result<Vec<String>, DiagnosticBundleChirho> {
        self.reserve_chirho(parameters_chirho.len(), 0)?;
        parameters_chirho
            .iter()
            .map(|parameter_chirho| self.key_chirho(parameter_chirho))
            .collect()
    }

    fn failure_chirho(
        &self,
        _error_chirho: haskelujah_ast_chirho::decl_chirho::GadtProjectionErrorChirho,
    ) -> DiagnosticBundleChirho {
        let mut diagnostics_chirho = DiagnosticBundleChirho::default();
        diagnostics_chirho.push_chirho(
            haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300),
                "imported type transport exceeded its work, output or depth bound",
                self.span_chirho,
            ),
        );
        diagnostics_chirho
    }

    fn reserve_chirho(
        &mut self,
        weight_chirho: usize,
        depth_chirho: usize,
    ) -> Result<(), DiagnosticBundleChirho> {
        self.budget_chirho
            .reserve_transport_chirho(weight_chirho, depth_chirho, self.span_chirho)
            .map_err(|error_chirho| self.failure_chirho(error_chirho))
    }

    fn legacy_qualifier_chirho(&self, bare_chirho: &str) -> Option<Option<&str>> {
        if self.safe_chirho.contains(bare_chirho) || self.preferred_chirho.contains_key(bare_chirho)
        {
            Some(None)
        } else if self.type_names_chirho.contains(bare_chirho) {
            Some(Some(self.qualifier_chirho))
        } else {
            None
        }
    }

    fn ast_name_chirho(
        &mut self,
        name_chirho: &mut haskelujah_ast_chirho::name_chirho::NameChirho,
    ) -> Result<(), DiagnosticBundleChirho> {
        if name_chirho.type_constructor_chirho().is_some()
            || name_chirho.family_key_chirho().is_some()
            || name_chirho.promotion_chirho().is_some()
        {
            return Ok(());
        }
        let qualifier_chirho = self
            .legacy_qualifier_chirho(name_chirho.text_chirho())
            .map(|qualifier_chirho| qualifier_chirho.map(str::len));
        let Some(length_chirho) = qualifier_chirho else {
            return Ok(());
        };
        self.reserve_chirho(length_chirho.unwrap_or(0).saturating_add(1), 0)?;
        let qualifier_chirho = length_chirho.map(|_| self.qualifier_chirho.to_owned());
        match name_chirho {
            haskelujah_ast_chirho::name_chirho::NameChirho::RawChirho(raw_chirho) => {
                raw_chirho.qualifier_chirho = qualifier_chirho
            }
            haskelujah_ast_chirho::name_chirho::NameChirho::ResolvedChirho(resolved_chirho) => {
                resolved_chirho.raw_chirho.qualifier_chirho = qualifier_chirho
            }
        }
        Ok(())
    }

    pub fn ast_chirho(
        &mut self,
        ty_chirho: &TypeChirho,
    ) -> Result<TypeChirho, DiagnosticBundleChirho> {
        haskelujah_ast_chirho::decl_chirho::check_type_projection_chirho(
            ty_chirho,
            self.budget_chirho,
        )
        .map_err(|error_chirho| self.failure_chirho(error_chirho))?;
        let mut copied_chirho = ty_chirho.clone();
        self.ast_inner_chirho(&mut copied_chirho)?;
        Ok(copied_chirho)
    }

    fn binders_chirho(
        &mut self,
        binders_chirho: &mut [haskelujah_ast_chirho::decl_chirho::TyVarChirho],
    ) -> Result<(), DiagnosticBundleChirho> {
        for binder_chirho in binders_chirho {
            if let Some(kind_chirho) = &mut binder_chirho.kind_annotation_chirho {
                self.ast_inner_chirho(kind_chirho)?;
            }
        }
        Ok(())
    }

    fn constraint_chirho(
        &mut self,
        constraint_chirho: &mut haskelujah_ast_chirho::ty_chirho::ConstraintChirho,
    ) -> Result<(), DiagnosticBundleChirho> {
        use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;
        match constraint_chirho {
            ConstraintChirho::ClassChirho {
                class_chirho,
                args_chirho,
                kind_args_chirho,
                ..
            } => {
                self.ast_name_chirho(class_chirho)?;
                for argument_chirho in kind_args_chirho.iter_mut().chain(args_chirho) {
                    self.ast_inner_chirho(argument_chirho)?;
                }
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => {
                self.binders_chirho(vars_chirho)?;
                for constraint_chirho in context_chirho {
                    self.constraint_chirho(constraint_chirho)?;
                }
                self.constraint_chirho(body_chirho)?;
            }
            ConstraintChirho::KindAnnotationChirho {
                constraint_chirho,
                kind_chirho,
                ..
            } => {
                self.constraint_chirho(constraint_chirho)?;
                self.ast_inner_chirho(kind_chirho)?;
            }
        }
        Ok(())
    }

    fn ast_inner_chirho(
        &mut self,
        ty_chirho: &mut TypeChirho,
    ) -> Result<(), DiagnosticBundleChirho> {
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho)
            | TypeChirho::PromotedConChirho { name_chirho, .. } => {
                self.ast_name_chirho(name_chirho)?
            }
            TypeChirho::VarChirho(_)
            | TypeChirho::WildcardChirho { .. }
            | TypeChirho::LitChirho { .. } => {}
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            }
            | TypeChirho::KindApplicationChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                self.ast_inner_chirho(fun_chirho)?;
                self.ast_inner_chirho(arg_chirho)?;
            }
            TypeChirho::KindAnnotationChirho {
                ty_chirho,
                kind_chirho,
                ..
            } => {
                self.ast_inner_chirho(ty_chirho)?;
                self.ast_inner_chirho(kind_chirho)?;
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                self.ast_inner_chirho(arg_chirho)?;
                self.ast_inner_chirho(result_chirho)?;
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    self.ast_inner_chirho(element_chirho)?;
                }
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                self.ast_inner_chirho(element_chirho)?
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => self.ast_inner_chirho(inner_chirho)?,
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            }
            | TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                self.binders_chirho(vars_chirho)?;
                self.ast_inner_chirho(body_chirho)?;
            }
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                for constraint_chirho in context_chirho {
                    self.constraint_chirho(constraint_chirho)?;
                }
                self.ast_inner_chirho(body_chirho)?;
            }
        }
        Ok(())
    }

    fn typed_head_chirho(
        &mut self,
        name_chirho: &str,
        builtin_seed_chirho: bool,
    ) -> Result<String, DiagnosticBundleChirho> {
        if name_chirho.starts_with('%') {
            self.reserve_chirho(name_chirho.len().saturating_add(1), 0)?;
            return Ok(name_chirho.to_owned());
        }
        let bare_chirho = name_chirho.rsplit('.').next().unwrap_or(name_chirho);
        // The positive builtin value producer and its SAME selected interface
        // supply this declaration. A matching global spelling supplies nothing.
        let compiler_chirho = builtin_seed_chirho.then_some(self.iface_chirho).flatten().filter(|iface_chirho| iface_chirho.exports_chirho.origin_chirho == haskelujah_naming_chirho::IfaceOriginChirho::BuiltinChirho).and_then(|iface_chirho| iface_chirho.exports_chirho.types_chirho.get(bare_chirho)).and_then(|ty_chirho| ty_chirho.declaration_chirho.as_ref()).filter(|declaration_chirho| matches!(declaration_chirho.identity_chirho, haskelujah_ast_chirho::name_chirho::TypeConstructorIdentityChirho::CompilerChirho(_)));
        if let Some(compiler_chirho) = compiler_chirho {
            let compiler_chirho = compiler_chirho.clone();
            self.reserve_chirho(128, 0)?;
            return Ok(compiler_chirho.type_key_chirho());
        }
        match self.legacy_qualifier_chirho(bare_chirho) {
            Some(None) => {
                self.reserve_chirho(bare_chirho.len().saturating_add(1), 0)?;
                Ok(bare_chirho.to_owned())
            }
            Some(Some(_)) => {
                self.reserve_chirho(
                    self.qualifier_chirho
                        .len()
                        .saturating_add(bare_chirho.len())
                        .saturating_add(2),
                    0,
                )?;
                Ok(format!("{}.{}", self.qualifier_chirho, bare_chirho))
            }
            None => {
                self.reserve_chirho(name_chirho.len().saturating_add(1), 0)?;
                Ok(name_chirho.to_owned())
            }
        }
    }

    fn typed_chirho(
        &mut self,
        ty_chirho: &haskelujah_typing_chirho::TyChirho,
        builtin_seed_chirho: bool,
        depth_chirho: usize,
    ) -> Result<haskelujah_typing_chirho::TyChirho, DiagnosticBundleChirho> {
        use haskelujah_typing_chirho::TyChirho;
        self.reserve_chirho(1, depth_chirho)?;
        Ok(match ty_chirho {
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                self.reserve_chirho(context_chirho.len(), depth_chirho)?;
                TyChirho::QualChirho {
                    context_chirho: context_chirho
                        .iter()
                        .map(|constraint_chirho| {
                            self.typed_chirho(
                                constraint_chirho,
                                builtin_seed_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Result<_, _>>()?,
                    body_chirho: Box::new(self.typed_chirho(
                        body_chirho,
                        builtin_seed_chirho,
                        depth_chirho + 1,
                    )?),
                }
            }
            TyChirho::VarChirho(variable_chirho) => TyChirho::VarChirho(*variable_chirho),
            TyChirho::ConChirho(name_chirho) => {
                TyChirho::ConChirho(self.typed_head_chirho(name_chirho, builtin_seed_chirho)?)
            }
            TyChirho::ForallVarChirho(name_chirho) => {
                self.reserve_chirho(name_chirho.len(), depth_chirho)?;
                TyChirho::ForallVarChirho(name_chirho.clone())
            }
            TyChirho::AppChirho(left_chirho, right_chirho) => TyChirho::AppChirho(
                Box::new(self.typed_chirho(left_chirho, builtin_seed_chirho, depth_chirho + 1)?),
                Box::new(self.typed_chirho(right_chirho, builtin_seed_chirho, depth_chirho + 1)?),
            ),
            TyChirho::FunChirho(left_chirho, right_chirho, mult_chirho) => TyChirho::FunChirho(
                Box::new(self.typed_chirho(left_chirho, builtin_seed_chirho, depth_chirho + 1)?),
                Box::new(self.typed_chirho(right_chirho, builtin_seed_chirho, depth_chirho + 1)?),
                *mult_chirho,
            ),
            TyChirho::ListChirho(element_chirho) => TyChirho::ListChirho(Box::new(
                self.typed_chirho(element_chirho, builtin_seed_chirho, depth_chirho + 1)?,
            )),
            TyChirho::TupleChirho(elements_chirho) => {
                self.reserve_chirho(elements_chirho.len(), depth_chirho)?;
                TyChirho::TupleChirho(
                    elements_chirho
                        .iter()
                        .map(|element_chirho| {
                            self.typed_chirho(element_chirho, builtin_seed_chirho, depth_chirho + 1)
                        })
                        .collect::<Result<_, _>>()?,
                )
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                let vars_chirho =
                    self.quantified_chirho(vars_chirho, builtin_seed_chirho, depth_chirho)?;
                let body_chirho = Box::new(self.typed_chirho(
                    body_chirho,
                    builtin_seed_chirho,
                    depth_chirho + 1,
                )?);
                if matches!(ty_chirho, TyChirho::RequiredForallChirho { .. }) {
                    TyChirho::RequiredForallChirho {
                        vars_chirho,
                        body_chirho,
                    }
                } else {
                    TyChirho::ForallChirho {
                        vars_chirho,
                        body_chirho,
                    }
                }
            }
        })
    }

    fn quantified_chirho(
        &mut self,
        variables_chirho: &haskelujah_typing_chirho::ty_chirho::QuantifiedVariablesChirho,
        builtin_seed_chirho: bool,
        depth_chirho: usize,
    ) -> Result<
        haskelujah_typing_chirho::ty_chirho::QuantifiedVariablesChirho,
        DiagnosticBundleChirho,
    > {
        self.reserve_chirho(variables_chirho.len(), depth_chirho)?;
        let mut records_chirho = Vec::with_capacity(variables_chirho.len());
        for record_chirho in variables_chirho.records_chirho() {
            let mut record_chirho = record_chirho.clone();
            record_chirho.kind_chirho = if let Some(kind_chirho) = &record_chirho.kind_chirho {
                Some(std::sync::Arc::new(self.typed_chirho(
                    kind_chirho,
                    builtin_seed_chirho,
                    depth_chirho + 1,
                )?))
            } else {
                None
            };
            records_chirho.push(record_chirho);
        }
        Ok(
            haskelujah_typing_chirho::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(
                records_chirho,
            ),
        )
    }

    pub fn scheme_chirho(
        &mut self,
        scheme_chirho: &haskelujah_typing_chirho::SchemeChirho,
        builtin_seed_chirho: bool,
    ) -> Result<haskelujah_typing_chirho::SchemeChirho, DiagnosticBundleChirho> {
        self.reserve_chirho(
            scheme_chirho
                .vars_chirho
                .len()
                .saturating_add(scheme_chirho.preds_chirho.len()),
            0,
        )?;
        let vars_chirho =
            self.quantified_chirho(&scheme_chirho.vars_chirho, builtin_seed_chirho, 0)?;
        let mut preds_chirho = Vec::new();
        for predicate_chirho in &scheme_chirho.preds_chirho {
            self.reserve_chirho(
                predicate_chirho
                    .class_name_chirho
                    .len()
                    .saturating_add(predicate_chirho.kind_tys_chirho.len())
                    .saturating_add(predicate_chirho.extra_tys_chirho.len()),
                0,
            )?;
            preds_chirho.push(predicate_chirho.try_mapped_chirho(|ty_chirho| {
                self.typed_chirho(ty_chirho, builtin_seed_chirho, 0)
            })?);
        }
        Ok(haskelujah_typing_chirho::SchemeChirho {
            vars_chirho,
            preds_chirho,
            ty_chirho: self.typed_chirho(&scheme_chirho.ty_chirho, builtin_seed_chirho, 0)?,
        })
    }
}

/// Compatibility for direct legacy scheme tests. Production selects an actual
/// interface and propagates the hard failure before the deferral boundary.
#[cfg(test)]
pub(crate) fn qualify_imported_scheme_for_iface_chirho(
    scheme_chirho: &haskelujah_typing_chirho::SchemeChirho,
    qualifiable_type_names_chirho: &std::collections::HashSet<String>,
    qualifier_chirho: &str,
    unqualified_type_names_chirho: &std::collections::HashSet<String>,
    preferred_qualified_type_names_chirho: &std::collections::HashMap<String, String>,
) -> haskelujah_typing_chirho::SchemeChirho {
    let mut budget_chirho =
        haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho();
    ImportQualificationChirho {
        iface_chirho: None,
        type_names_chirho: qualifiable_type_names_chirho,
        qualifier_chirho,
        safe_chirho: unqualified_type_names_chirho,
        preferred_chirho: preferred_qualified_type_names_chirho,
        budget_chirho: &mut budget_chirho,
        span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
    }
    .scheme_chirho(scheme_chirho, false)
    .expect("bounded direct legacy test scheme")
}
