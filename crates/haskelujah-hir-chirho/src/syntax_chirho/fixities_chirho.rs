// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded syntax whose grouping is independent of the parser's spelling-based fixity table.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;

impl LowerChirho<'_> {
    fn fixity_error_chirho(&self, message_chirho: &str, span_chirho: SpanChirho) -> HirErrorChirho {
        if self.unit_chirho == UnitIdentityChirho::CompilerChirho {
            self.error_chirho(message_chirho, span_chirho)
        } else {
            self.invalid_class_chirho(message_chirho, span_chirho)
        }
    }
    pub(super) fn prepare_fixities_chirho(
        &mut self,
        declarations_chirho: &[DeclChirho],
        completeness_chirho: CompletenessChirho,
        imports_chirho: &[crate::ImportedInterfaceChirho],
    ) -> ResultChirho<std::collections::BTreeMap<OriginalNameChirho, InterfaceFixityChirho>> {
        for import_chirho in imports_chirho {
            for (name_chirho, fixity_chirho) in &import_chirho.interface_chirho.fixities_chirho {
                self.note_fixity_chirho(name_chirho.clone(), *fixity_chirho)?;
            }
        }
        let mut own_chirho = self.fixities_chirho(declarations_chirho)?;
        // The complete accepted producer, not an importing or re-exporting module,
        // certifies absence of an explicit declaration. A slice certifies none.
        if completeness_chirho == CompletenessChirho::CompleteChirho {
            for index_chirho in 0..self.names_chirho.len() {
                self.charge_names_chirho(1, SpanChirho::DUMMY_CHIRHO)?;
                let info_chirho = &self.names_chirho[index_chirho];
                if !info_chirho.local_chirho
                    && info_chirho.wired_chirho.is_none()
                    && info_chirho.namespace_chirho == NamespaceChirho::ValueChirho
                    && info_chirho.unit_chirho == self.unit_chirho
                    && info_chirho.module_chirho == self.module_chirho
                {
                    let name_chirho = info_chirho.original_chirho();
                    let cost_chirho =
                        name_chirho.module_chirho.len() + name_chirho.occurrence_chirho.len() + 2;
                    if let std::collections::btree_map::Entry::Vacant(entry_chirho) =
                        own_chirho.entry(name_chirho)
                    {
                        self.charge_names_chirho(cost_chirho, SpanChirho::DUMMY_CHIRHO)?;
                        entry_chirho.insert(InterfaceFixityChirho {
                            associativity_chirho: AssociativityChirho::LeftChirho,
                            precedence_chirho: 9,
                        });
                    }
                }
            }
        }
        for (name_chirho, fixity_chirho) in &own_chirho {
            self.note_fixity_chirho(name_chirho.clone(), *fixity_chirho)?;
        }
        Ok(own_chirho)
    }

    fn note_fixity_chirho(
        &mut self,
        name_chirho: OriginalNameChirho,
        fixity_chirho: InterfaceFixityChirho,
    ) -> ResultChirho<()> {
        self.charge_names_chirho(
            name_chirho.module_chirho.len() + name_chirho.occurrence_chirho.len() + 2,
            SpanChirho::DUMMY_CHIRHO,
        )?;
        if fixity_chirho.precedence_chirho > 9
            || self
                .known_fixities_chirho
                .get(&name_chirho)
                .is_some_and(|prior_chirho| *prior_chirho != fixity_chirho)
        {
            return Err(self.error_chirho(
                "inconsistent checked fixity for one original name",
                SpanChirho::DUMMY_CHIRHO,
            ));
        }
        self.known_fixities_chirho
            .insert(name_chirho, fixity_chirho);
        Ok(())
    }

    pub(super) fn fixities_chirho(
        &mut self,
        declarations_chirho: &[DeclChirho],
    ) -> ResultChirho<std::collections::BTreeMap<OriginalNameChirho, InterfaceFixityChirho>> {
        let mut result_chirho = std::collections::BTreeMap::new();
        for declaration_chirho in declarations_chirho {
            let DeclChirho::FixityDeclChirho {
                fixity_chirho,
                precedence_chirho,
                ops_chirho,
                span_chirho,
            } = declaration_chirho
            else {
                continue;
            };
            let precedence_chirho = precedence_chirho.unwrap_or(9);
            if precedence_chirho > 9 || ops_chirho.is_empty() {
                return Err(self.fixity_error_chirho("invalid fixity declaration", *span_chirho));
            }
            for operator_chirho in ops_chirho {
                let id_chirho = self
                    .bound_name_chirho(operator_chirho, NamespaceChirho::ValueChirho)
                    .map_err(|mut error_chirho| {
                        if self.unit_chirho == UnitIdentityChirho::CompilerChirho
                            && error_chirho.kind_chirho == HirErrorKindChirho::InvalidChirho
                        {
                            error_chirho.kind_chirho = HirErrorKindChirho::UnsupportedChirho;
                        }
                        error_chirho
                    })?;
                let info_chirho = &self.names_chirho[id_chirho.0 as usize];
                if info_chirho.local_chirho
                    || info_chirho.unit_chirho != self.unit_chirho
                    || info_chirho.module_chirho != self.module_chirho
                {
                    return Err(self.fixity_error_chirho(
                        "fixity has no declaration in its own binding group",
                        *span_chirho,
                    ));
                }
                let original_chirho = info_chirho.original_chirho();
                self.charge_names_chirho(
                    original_chirho.module_chirho.len()
                        + original_chirho.occurrence_chirho.len()
                        + 2,
                    *span_chirho,
                )?;
                let associativity_chirho = match fixity_chirho {
                    haskelujah_ast_chirho::decl_chirho::FixityChirho::InfixChirho => {
                        AssociativityChirho::NoneChirho
                    }
                    haskelujah_ast_chirho::decl_chirho::FixityChirho::InfixlChirho => {
                        AssociativityChirho::LeftChirho
                    }
                    haskelujah_ast_chirho::decl_chirho::FixityChirho::InfixrChirho => {
                        AssociativityChirho::RightChirho
                    }
                };
                if result_chirho
                    .insert(
                        original_chirho,
                        InterfaceFixityChirho {
                            associativity_chirho,
                            precedence_chirho,
                        },
                    )
                    .is_some()
                {
                    return Err(
                        self.fixity_error_chirho("repeated fixity declaration", *span_chirho)
                    );
                }
            }
        }
        Ok(result_chirho)
    }
}
