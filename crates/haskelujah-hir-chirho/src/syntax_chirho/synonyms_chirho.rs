// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Preserve synonym identity and lexical arguments; only TC may erase a checked head.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;

impl LowerChirho<'_> {
    pub(super) fn synonym_declarations_chirho(
        &mut self,
        declarations_chirho: &[DeclChirho],
    ) -> ResultChirho<Vec<SynonymDeclChirho>> {
        let mut result_chirho = Vec::new();
        for declaration_chirho in declarations_chirho {
            let DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                rhs_chirho,
                span_chirho,
            } = declaration_chirho
            else {
                continue;
            };
            let id_chirho = self.bound_name_chirho(name_chirho, NamespaceChirho::TypeChirho)?;
            self.env_chirho.push_scope_chirho();
            let lowered_chirho = (|| {
                let mut seen_chirho = HashSet::new();
                let mut parameters_chirho = Vec::new();
                for variable_chirho in type_vars_chirho {
                    if variable_chirho.kind_annotation_chirho.is_some()
                        || !variable_chirho.is_visible_chirho()
                    {
                        return Err(self.error_chirho(
                            "next synonyms require first-order inferred parameter kinds",
                            *span_chirho,
                        ));
                    }
                    if !seen_chirho.insert(variable_chirho.name_chirho.full_name_chirho()) {
                        return Err(
                            self.invalid_class_chirho("duplicate synonym parameter", *span_chirho)
                        );
                    }
                    parameters_chirho.push(self.bind_chirho(
                        variable_chirho.name_chirho.text_chirho(),
                        NamespaceChirho::TypeChirho,
                        variable_chirho.name_chirho.span_chirho(),
                        true,
                        None,
                    )?);
                }
                Ok(SynonymDeclChirho {
                    name_chirho: id_chirho,
                    parameters_chirho,
                    body_chirho: self.type_chirho(rhs_chirho, 0)?,
                    higher_kinds_partial_chirho: self.explicit_forall_chirho,
                })
            })();
            self.env_chirho.pop_scope_chirho();
            result_chirho.push(lowered_chirho?);
        }
        Ok(result_chirho)
    }

    pub(super) fn predicate_name_chirho(
        &self,
        name_chirho: &NameChirho,
    ) -> ResultChirho<NameIdChirho> {
        let id_chirho = self.lookup_chirho(name_chirho, NamespaceChirho::TypeChirho)?;
        if self.synonym_names_chirho.contains(&id_chirho) {
            return Err(self.error_chirho(
                "next does not yet expand constraint synonyms",
                name_chirho.span_chirho(),
            ));
        }
        Ok(id_chirho)
    }

    pub(super) fn instance_synonyms_chirho(
        &mut self,
        head_chirho: &SurfaceTypeChirho,
        span_chirho: SpanChirho,
    ) -> ResultChirho<()> {
        if self.explicit_forall_chirho {
            return Ok(());
        }
        let mut work_chirho = vec![head_chirho];
        while let Some(ty_chirho) = work_chirho.pop() {
            self.room_chirho(0, span_chirho)?;
            self.next_expr_chirho += 1;
            match ty_chirho {
                SurfaceTypeChirho::SynonymChirho(_) => {
                    return Err(self.invalid_class_chirho(
                        "type synonym instances require TypeSynonymInstances (included in GHC2021)",
                        span_chirho,
                    ));
                }
                SurfaceTypeChirho::AppChirho(left_chirho, right_chirho)
                | SurfaceTypeChirho::FunChirho(left_chirho, right_chirho) => {
                    work_chirho.extend([left_chirho.as_ref(), right_chirho.as_ref()])
                }
                SurfaceTypeChirho::ListChirho(item_chirho) => work_chirho.push(item_chirho),
                SurfaceTypeChirho::TupleChirho(items_chirho) => work_chirho.extend(items_chirho),
                _ => {}
            }
        }
        Ok(())
    }
}
