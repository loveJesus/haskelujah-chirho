// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ordinary declaration accounts, before any field or value reference is resolved.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;
use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, StrictnessChirho};

impl LowerChirho<'_> {
    pub(super) fn data_declarations_chirho(
        &mut self,
        declarations_chirho: &[DeclChirho],
        opaque_policy_chirho: &std::collections::BTreeMap<OriginalNameChirho, Vec<String>>,
    ) -> ResultChirho<Vec<DataDeclChirho>> {
        let mut constructors_seen_chirho = HashSet::new();
        let mut result_chirho = Vec::new();
        let mut consumed_chirho = std::collections::BTreeSet::new();
        for owner_chirho in opaque_policy_chirho.keys() {
            if self.unit_chirho != UnitIdentityChirho::CompilerChirho
                || owner_chirho.unit_chirho != self.unit_chirho
                || owner_chirho.module_chirho != self.module_chirho
                || owner_chirho.namespace_chirho != NamespaceChirho::TypeChirho
            {
                return Err(self.error_chirho(
                    "opaque source layout has no defining-unit manifest",
                    SpanChirho::DUMMY_CHIRHO,
                ));
            }
        }
        for declaration_chirho in declarations_chirho {
            let DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                deriving_chirho,
                kind_sig_chirho,
                span_chirho,
            } = declaration_chirho
            else {
                continue;
            };
            if !deriving_chirho.is_empty()
                || kind_sig_chirho.is_some()
                || constructors_chirho.is_empty()
            {
                return Err(self.error_chirho(
                    "next ordinary data requires constructors without deriving or kind signatures",
                    *span_chirho,
                ));
            }
            let owner_chirho = OriginalNameChirho {
                unit_chirho: self.unit_chirho.clone(),
                module_chirho: self.module_chirho.clone(),
                namespace_chirho: NamespaceChirho::TypeChirho,
                occurrence_chirho: name_chirho.text_chirho().into(),
            };
            if let Some(expected_chirho) = opaque_policy_chirho.get(&owner_chirho) {
                let actual_chirho = constructors_chirho
                    .iter()
                    .map(|constructor_chirho| {
                        if let ConDeclChirho::OrdinaryChirho {
                            name_chirho,
                            prefix_chirho,
                            ..
                        } = constructor_chirho
                        {
                            prefix_chirho
                                .is_empty_chirho()
                                .then(|| name_chirho.text_chirho().to_string())
                        } else {
                            None
                        }
                    })
                    .collect::<Option<Vec<_>>>();
                if !type_vars_chirho.is_empty()
                    || actual_chirho.as_ref() != Some(expected_chirho)
                    || !consumed_chirho.insert(owner_chirho.clone())
                {
                    return Err(self.error_chirho("opaque source header or complete constructor account disagrees with its manifest", *span_chirho));
                }
                self.charge_names_chirho(
                    expected_chirho
                        .iter()
                        .map(|name_chirho| name_chirho.len() + 1)
                        .sum(),
                    *span_chirho,
                )?;
                let withheld_constructors_chirho = expected_chirho
                    .iter()
                    .map(|name_chirho| OriginalNameChirho {
                        namespace_chirho: NamespaceChirho::ValueChirho,
                        occurrence_chirho: name_chirho.clone(),
                        ..owner_chirho.clone()
                    })
                    .collect();
                result_chirho.push(DataDeclChirho {
                    name_chirho: self
                        .bound_name_chirho(name_chirho, NamespaceChirho::TypeChirho)?,
                    parameters_chirho: Vec::new(),
                    constructors_chirho: Vec::new(),
                    withheld_constructors_chirho,
                });
                continue;
            }
            let mut names_chirho = Vec::new();
            for constructor_chirho in constructors_chirho {
                let ConDeclChirho::OrdinaryChirho {
                    name_chirho,
                    fields_chirho,
                    prefix_chirho,
                    ..
                } = constructor_chirho
                else {
                    return Err(self.error_chirho(
                        "next does not yet check record or GADT declarations",
                        *span_chirho,
                    ));
                };
                if !prefix_chirho.is_empty_chirho()
                    || fields_chirho.iter().any(|(strictness_chirho, _)| {
                        *strictness_chirho != StrictnessChirho::LazyChirho
                    })
                {
                    return Err(self.error_chirho("next does not yet check constructor contexts, existentials or field strictness", *span_chirho));
                }
                if !constructors_seen_chirho.insert(name_chirho.full_name_chirho()) {
                    return Err(
                        self.error_chirho("duplicate data constructor", name_chirho.span_chirho())
                    );
                }
                names_chirho.push(self.bind_chirho(
                    name_chirho.text_chirho(),
                    NamespaceChirho::ValueChirho,
                    name_chirho.span_chirho(),
                    false,
                    None,
                )?);
            }
            self.env_chirho.push_scope_chirho();
            let lowered_chirho = (|| {
                let mut seen_chirho = HashSet::new();
                let mut parameters_chirho = Vec::new();
                for variable_chirho in type_vars_chirho {
                    if variable_chirho.kind_annotation_chirho.is_some()
                        || !variable_chirho.is_visible_chirho()
                    {
                        return Err(self.error_chirho(
                            "next ordinary data parameters require inferred Type kinds",
                            variable_chirho.name_chirho.span_chirho(),
                        ));
                    }
                    if !seen_chirho.insert(variable_chirho.name_chirho.full_name_chirho()) {
                        return Err(self.error_chirho(
                            "duplicate data type parameter",
                            variable_chirho.name_chirho.span_chirho(),
                        ));
                    }
                    parameters_chirho.push(self.bind_chirho(
                        variable_chirho.name_chirho.text_chirho(),
                        NamespaceChirho::TypeChirho,
                        variable_chirho.name_chirho.span_chirho(),
                        true,
                        None,
                    )?);
                }
                let mut lowered_constructors_chirho = Vec::new();
                for (constructor_chirho, id_chirho) in constructors_chirho.iter().zip(names_chirho)
                {
                    let ConDeclChirho::OrdinaryChirho {
                        fields_chirho,
                        span_chirho,
                        ..
                    } = constructor_chirho
                    else {
                        unreachable!()
                    };
                    let fields_chirho = fields_chirho
                        .iter()
                        .map(|(_, field_chirho)| self.type_chirho(field_chirho, 0))
                        .collect::<ResultChirho<_>>()?;
                    lowered_constructors_chirho.push(ConstructorChirho {
                        name_chirho: id_chirho,
                        fields_chirho,
                        span_chirho: *span_chirho,
                    });
                }
                Ok(DataDeclChirho {
                    name_chirho: self
                        .bound_name_chirho(name_chirho, NamespaceChirho::TypeChirho)?,
                    parameters_chirho,
                    constructors_chirho: lowered_constructors_chirho,
                    withheld_constructors_chirho: Vec::new(),
                })
            })();
            self.env_chirho.pop_scope_chirho();
            result_chirho.push(lowered_chirho?);
        }
        if consumed_chirho.len() != opaque_policy_chirho.len() {
            return Err(self.error_chirho(
                "unconsumed opaque source declaration policy",
                SpanChirho::DUMMY_CHIRHO,
            ));
        }
        for declaration_chirho in &result_chirho {
            self.charge_names_chirho(
                std::mem::size_of::<NameIdChirho>(),
                SpanChirho::DUMMY_CHIRHO,
            )?;
            self.data_names_chirho
                .insert(declaration_chirho.name_chirho);
        }
        Ok(result_chirho)
    }
}
