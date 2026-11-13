// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Reuse one consumer NameId per original name, independently of lexical import scope.
use super::*;
use std::collections::BTreeMap;
use std::sync::Arc;
mod scope_chirho;

impl LowerChirho<'_> {
    pub(super) fn charge_names_chirho(
        &mut self,
        count_chirho: usize,
        span_chirho: SpanChirho,
    ) -> ResultChirho<()> {
        let cost_chirho = u32::try_from(count_chirho).map_err(|_| HirErrorChirho {
            kind_chirho: HirErrorKindChirho::ResourceChirho,
            message_chirho: "HIR name byte ceiling exhausted".into(),
            span_chirho,
        })?;
        self.next_expr_chirho =
            self.next_expr_chirho
                .checked_add(cost_chirho)
                .ok_or_else(|| HirErrorChirho {
                    kind_chirho: HirErrorKindChirho::ResourceChirho,
                    message_chirho: "HIR name byte ceiling exhausted".into(),
                    span_chirho,
                })?;
        self.room_chirho(0, span_chirho)
    }

    pub(super) fn imported_name_chirho(
        &mut self,
        original_chirho: &OriginalNameChirho,
    ) -> ResultChirho<NameIdChirho> {
        if let Some(name_chirho) = self.imported_names_chirho.get(original_chirho) {
            return Ok(*name_chirho);
        }
        self.room_chirho(0, SpanChirho::DUMMY_CHIRHO)?;
        self.charge_names_chirho(
            (original_chirho.module_chirho.len() + original_chirho.occurrence_chirho.len())
                .saturating_mul(2),
            SpanChirho::DUMMY_CHIRHO,
        )?;
        let id_chirho = NameIdChirho(self.env_chirho.fresh_def_id_chirho().0);
        if id_chirho.0 as usize != self.names_chirho.len() {
            return Err(self.error_chirho(
                "interface name catalogue disagrees",
                SpanChirho::DUMMY_CHIRHO,
            ));
        }
        self.names_chirho.push(NameInfoChirho {
            unit_chirho: original_chirho.unit_chirho.clone(),
            module_chirho: original_chirho.module_chirho.clone(),
            namespace_chirho: original_chirho.namespace_chirho,
            occurrence_chirho: original_chirho.occurrence_chirho.clone(),
            local_chirho: false,
            wired_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        });
        self.imported_names_chirho
            .insert(original_chirho.clone(), id_chirho);
        Ok(id_chirho)
    }

    pub(super) fn lookup_text_chirho(
        &self,
        text_chirho: &str,
        namespace_chirho: NamespaceChirho,
        span_chirho: SpanChirho,
    ) -> ResultChirho<NameIdChirho> {
        let own_chirho = self
            .env_chirho
            .lookup_chirho(text_chirho, namespace_chirho)
            .map(|info_chirho| NameIdChirho(info_chirho.def_id_chirho.0));
        let imported_chirho = self
            .import_scope_chirho
            .get(&(text_chirho.to_string(), namespace_chirho));
        let prelude_text_chirho = text_chirho.strip_prefix("Prelude.").unwrap_or(text_chirho);
        let prelude_claim_chirho = if self.implicit_prelude_chirho {
            self.prelude_exports_chirho.as_ref().map(|exports_chirho| {
                exports_chirho.contains(&(prelude_text_chirho.to_string(), namespace_chirho))
            })
        } else {
            Some(false)
        };
        if let Some(own_chirho) = own_chirho {
            if !self.names_chirho[own_chirho.0 as usize].local_chirho
                && self.names_chirho[own_chirho.0 as usize]
                    .wired_chirho
                    .is_none()
            {
                if prelude_claim_chirho == Some(true) {
                    return Err(self.invalid_class_chirho(
                        "ambiguous top-level and implicit Prelude name",
                        span_chirho,
                    ));
                }
                if prelude_claim_chirho.is_none() {
                    return Err(self.error_chirho(
                        "implicit Prelude has no complete export name account",
                        span_chirho,
                    ));
                }
            }
            // Nested binders shadow imports. Top-level declarations clash only at uses.
            if self.names_chirho[own_chirho.0 as usize].local_chirho
                || imported_chirho.is_none_or(|ids_chirho| {
                    ids_chirho.iter().all(|id_chirho| *id_chirho == own_chirho)
                })
            {
                return Ok(own_chirho);
            }
            return Err(
                self.invalid_class_chirho("ambiguous top-level and imported name", span_chirho)
            );
        }
        match imported_chirho.map(Vec::as_slice) {
            Some([id_chirho]) => {
                if prelude_claim_chirho.is_none() {
                    return Err(self.error_chirho(
                        "implicit Prelude has no complete export name account",
                        span_chirho,
                    ));
                }
                if prelude_claim_chirho == Some(true) {
                    if self.names_chirho[id_chirho.0 as usize].unit_chirho
                        == UnitIdentityChirho::StandaloneHomeChirho
                    {
                        return Err(self.invalid_class_chirho(
                            "ambiguous source import and implicit Prelude name",
                            span_chirho,
                        ));
                    }
                    return Err(self.error_chirho("Prelude export identity needs its checked defining interface; names alone supply no type", span_chirho));
                }
                Ok(*id_chirho)
            }
            Some(_) => Err(self.invalid_class_chirho("ambiguous imported name", span_chirho)),
            None => {
                if prelude_claim_chirho != Some(false) {
                    return Err(self.error_chirho(
                        "Prelude exports this name without a checked type or instance interface",
                        span_chirho,
                    ));
                }
                let message_chirho = format!(
                    "next has no checked declaration for `{text_chirho}`; no placeholder type is supplied"
                );
                Err(
                    if self.sliced_units_chirho.is_empty()
                        && (self.rebindable_chirho || !self.imported_names_chirho.is_empty())
                    {
                        self.invalid_class_chirho(&message_chirho, span_chirho)
                    } else {
                        self.error_chirho(message_chirho, span_chirho)
                    },
                )
            }
        }
    }

    fn imported_surface_chirho(
        &mut self,
        ty_chirho: &InterfaceTypeChirho,
        binder_chirho: NameIdChirho,
        depth_chirho: usize,
    ) -> ResultChirho<SurfaceTypeChirho> {
        self.room_chirho(depth_chirho, SpanChirho::DUMMY_CHIRHO)?;
        self.next_expr_chirho += 1;
        Ok(match ty_chirho {
            InterfaceTypeChirho::BuiltinChirho(InterfaceBuiltinChirho::BoolChirho) => {
                SurfaceTypeChirho::BoolChirho
            }
            InterfaceTypeChirho::BuiltinChirho(InterfaceBuiltinChirho::CharChirho) => {
                SurfaceTypeChirho::CharChirho
            }
            InterfaceTypeChirho::BuiltinChirho(InterfaceBuiltinChirho::IntegerChirho) => {
                SurfaceTypeChirho::IntegerChirho
            }
            InterfaceTypeChirho::DeclaredChirho(name_chirho) => {
                SurfaceTypeChirho::DeclaredChirho(self.imported_name_chirho(name_chirho)?)
            }
            InterfaceTypeChirho::BoundChirho(0) => SurfaceTypeChirho::VariableChirho(binder_chirho),
            InterfaceTypeChirho::AppChirho(fun_chirho, arg_chirho) => {
                if matches!(
                    fun_chirho.as_ref(),
                    InterfaceTypeChirho::BuiltinChirho(InterfaceBuiltinChirho::ListChirho)
                ) {
                    SurfaceTypeChirho::ListChirho(Box::new(self.imported_surface_chirho(
                        arg_chirho,
                        binder_chirho,
                        depth_chirho + 1,
                    )?))
                } else {
                    SurfaceTypeChirho::AppChirho(
                        Box::new(self.imported_surface_chirho(
                            fun_chirho,
                            binder_chirho,
                            depth_chirho + 1,
                        )?),
                        Box::new(self.imported_surface_chirho(
                            arg_chirho,
                            binder_chirho,
                            depth_chirho + 1,
                        )?),
                    )
                }
            }
            InterfaceTypeChirho::FunChirho(arg_chirho, result_chirho) => {
                SurfaceTypeChirho::FunChirho(
                    Box::new(self.imported_surface_chirho(
                        arg_chirho,
                        binder_chirho,
                        depth_chirho + 1,
                    )?),
                    Box::new(self.imported_surface_chirho(
                        result_chirho,
                        binder_chirho,
                        depth_chirho + 1,
                    )?),
                )
            }
            InterfaceTypeChirho::TupleChirho(items_chirho) => SurfaceTypeChirho::TupleChirho(
                items_chirho
                    .iter()
                    .map(|item_chirho| {
                        self.imported_surface_chirho(item_chirho, binder_chirho, depth_chirho + 1)
                    })
                    .collect::<ResultChirho<_>>()?,
            ),
            _ => {
                return Err(self.error_chirho(
                    "unsupported checked class surface in interface",
                    SpanChirho::DUMMY_CHIRHO,
                ));
            }
        })
    }

    pub(super) fn import_interfaces_chirho(
        &mut self,
        module_chirho: &AstModuleChirho,
        interfaces_chirho: &BTreeMap<String, Arc<InterfaceChirho>>,
        implicit_chirho: &[Arc<InterfaceChirho>],
    ) -> ResultChirho<Vec<ImportedInterfaceChirho>> {
        let mut result_chirho = Vec::new();
        let mut classes_seen_chirho = HashSet::new();
        let mut selected_seen_chirho = HashSet::new();
        let mut selected_chirho = Vec::new();
        for import_chirho in &module_chirho.imports_chirho {
            if import_chirho.qualified_chirho || import_chirho.alias_chirho.is_some() {
                return Err(self.error_chirho(
                    "next source interfaces currently require a plain import",
                    import_chirho.span_chirho,
                ));
            }
            let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
            if import_chirho.spec_chirho.is_none()
                && !selected_seen_chirho.insert(module_name_chirho.clone())
            {
                continue;
            }
            let Some(interface_chirho) = interfaces_chirho.get(&module_name_chirho) else {
                return Err(self.error_chirho(
                    "imported module has no independently checked source interface",
                    import_chirho.span_chirho,
                ));
            };
            if interface_chirho.module_chirho != module_name_chirho {
                return Err(self.invalid_class_chirho(
                    "interface module identity disagrees with selected import",
                    import_chirho.span_chirho,
                ));
            }
            let exports_chirho = self.selected_exports_chirho(interface_chirho, import_chirho)?;
            selected_chirho.push((
                Arc::clone(interface_chirho),
                exports_chirho,
                import_chirho.span_chirho,
            ));
        }
        for interface_chirho in implicit_chirho {
            selected_chirho.push((
                Arc::clone(interface_chirho),
                Vec::new(),
                SpanChirho::DUMMY_CHIRHO,
            ));
        }
        let mut originals_seen_chirho = HashSet::new();
        for (interface_chirho, exports_chirho, span_chirho) in selected_chirho {
            // Hidden and explicit imports may share a producer; scope is still handled below.
            self.sliced_units_chirho
                .extend(interface_chirho.sliced_units_chirho.iter().cloned());
            let module_name_chirho = &interface_chirho.module_chirho;
            let mut names_chirho = BTreeMap::new();
            for original_chirho in interface_chirho
                .data_chirho
                .keys()
                .chain(interface_chirho.values_chirho.keys())
                .chain(interface_chirho.classes_chirho.keys())
                .chain(interface_chirho.wired_types_chirho.keys())
            {
                let id_chirho = self.imported_name_chirho(original_chirho)?;
                self.charge_names_chirho(
                    original_chirho.module_chirho.len()
                        + original_chirho.occurrence_chirho.len()
                        + 1,
                    span_chirho,
                )?;
                names_chirho.insert(original_chirho.clone(), id_chirho);
            }
            for (original_chirho, wired_chirho) in &interface_chirho.wired_types_chirho {
                if *wired_chirho != WiredNameChirho::IntegerChirho
                    || original_chirho.unit_chirho != UnitIdentityChirho::CompilerChirho
                    || original_chirho.module_chirho != "GHC.Internal.Bignum.Integer"
                    || original_chirho.namespace_chirho != NamespaceChirho::TypeChirho
                    || original_chirho.occurrence_chirho != "Integer"
                {
                    return Err(self
                        .invalid_class_chirho("invalid wired declaration identity", span_chirho));
                }
                let id_chirho = names_chirho[original_chirho];
                self.names_chirho[id_chirho.0 as usize].wired_chirho = Some(*wired_chirho);
            }
            for original_chirho in &exports_chirho {
                let id_chirho = *names_chirho.get(original_chirho).ok_or_else(|| {
                    self.invalid_class_chirho("interface export has no declaration", span_chirho)
                })?;
                for text_chirho in [
                    original_chirho.occurrence_chirho.clone(),
                    format!("{module_name_chirho}.{}", original_chirho.occurrence_chirho),
                ] {
                    self.charge_names_chirho(text_chirho.len() + 1, span_chirho)?;
                    let ids_chirho = self
                        .import_scope_chirho
                        .entry((text_chirho, original_chirho.namespace_chirho))
                        .or_default();
                    if !ids_chirho.contains(&id_chirho) {
                        ids_chirho.push(id_chirho);
                    }
                }
            }
            for class_chirho in interface_chirho.classes_chirho.values() {
                if !classes_seen_chirho.insert(class_chirho.name_chirho.clone()) {
                    continue;
                }
                // This private binder is an HIR surface aid, never a transported identity.
                self.charge_names_chirho(self.module_chirho.len() + 48, SpanChirho::DUMMY_CHIRHO)?;
                let parameter_chirho = NameIdChirho(self.env_chirho.fresh_def_id_chirho().0);
                self.names_chirho.push(NameInfoChirho {
                    unit_chirho: self.unit_chirho.clone(),
                    module_chirho: self.module_chirho.clone(),
                    namespace_chirho: NamespaceChirho::TypeChirho,
                    occurrence_chirho: format!(
                        "%next:import-parameter:{}Chirho",
                        parameter_chirho.0
                    ),
                    local_chirho: true,
                    wired_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                });
                let mut methods_chirho = Vec::new();
                for method_chirho in &class_chirho.methods_chirho {
                    methods_chirho.push(ClassMethodChirho {
                        name_chirho: self.imported_name_chirho(&method_chirho.name_chirho)?,
                        default_chirho: method_chirho
                            .default_chirho
                            .as_ref()
                            .map(|name_chirho| self.imported_name_chirho(name_chirho))
                            .transpose()?,
                        ty_chirho: self.imported_surface_chirho(
                            &method_chirho.scheme_chirho.body_chirho,
                            parameter_chirho,
                            0,
                        )?,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    });
                }
                let name_chirho = self.imported_name_chirho(&class_chirho.name_chirho)?;
                self.imported_classes_chirho.push(ClassDeclChirho {
                    name_chirho,
                    parameter_chirho,
                    superclasses_chirho: Vec::new(),
                    superclass_selectors_chirho: Vec::new(),
                    methods_chirho,
                });
            }
            if !originals_seen_chirho.insert((
                interface_chirho.unit_chirho.clone(),
                interface_chirho.module_chirho.clone(),
            )) {
                continue;
            }
            result_chirho.push(ImportedInterfaceChirho {
                interface_chirho: Arc::clone(&interface_chirho),
                names_chirho,
            });
        }
        Ok(result_chirho)
    }
    pub(super) fn exports_chirho(
        &self,
        module_chirho: &AstModuleChirho,
    ) -> ResultChirho<Vec<NameIdChirho>> {
        use haskelujah_ast_chirho::module_chirho::ExportSpecChirho;
        let mut result_chirho = Vec::new();
        if let Some(exports_chirho) = &module_chirho.exports_chirho {
            for export_chirho in exports_chirho {
                let ExportSpecChirho::VarChirho(name_chirho) = export_chirho else {
                    return Err(self.error_chirho(
                        "next supports only value-only explicit source exports",
                        module_chirho.span_chirho,
                    ));
                };
                let id_chirho = self.lookup_chirho(name_chirho, NamespaceChirho::ValueChirho)?;
                if !result_chirho.contains(&id_chirho) {
                    result_chirho.push(id_chirho);
                }
            }
        } else {
            for (index_chirho, info_chirho) in self.names_chirho.iter().enumerate() {
                if !info_chirho.local_chirho
                    && info_chirho.wired_chirho.is_none()
                    && info_chirho.unit_chirho == self.unit_chirho
                    && info_chirho.module_chirho == self.module_chirho
                {
                    result_chirho.push(NameIdChirho(index_chirho as u32));
                }
            }
        }
        Ok(result_chirho)
    }
}
