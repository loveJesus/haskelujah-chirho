// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source class accounts and private instance implementations. Method occurrences remain selectors.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;

impl LowerChirho {
    pub(super) fn invalid_class_chirho(
        &self,
        message_chirho: &str,
        span_chirho: SpanChirho,
    ) -> HirErrorChirho {
        HirErrorChirho {
            kind_chirho: HirErrorKindChirho::InvalidChirho,
            message_chirho: message_chirho.into(),
            span_chirho,
        }
    }

    pub(super) fn predeclare_type_headers_chirho(
        &mut self,
        declarations_chirho: &[DeclChirho],
    ) -> ResultChirho<()> {
        let mut seen_chirho = HashSet::new();
        for declaration_chirho in declarations_chirho {
            let name_chirho = match declaration_chirho {
                DeclChirho::DataDeclChirho { name_chirho, .. }
                | DeclChirho::ClassDeclChirho { name_chirho, .. } => name_chirho,
                _ => continue,
            };
            if !seen_chirho.insert(name_chirho.full_name_chirho()) {
                return Err(self.invalid_class_chirho(
                    "duplicate type/class declaration",
                    name_chirho.span_chirho(),
                ));
            }
            self.bind_chirho(
                name_chirho.text_chirho(),
                NamespaceChirho::TypeChirho,
                name_chirho.span_chirho(),
                false,
                None,
            )?;
        }
        Ok(())
    }

    pub(super) fn class_declarations_chirho(
        &mut self,
        declarations_chirho: &[DeclChirho],
        data_chirho: &[DataDeclChirho],
    ) -> ResultChirho<Vec<ClassDeclChirho>> {
        let mut seen_chirho: HashSet<_> = data_chirho
            .iter()
            .flat_map(|data_chirho| &data_chirho.constructors_chirho)
            .map(|constructor_chirho| {
                self.names_chirho[constructor_chirho.name_chirho.0 as usize]
                    .occurrence_chirho
                    .clone()
            })
            .collect();
        let mut result_chirho = Vec::new();
        for declaration_chirho in declarations_chirho {
            let DeclChirho::ClassDeclChirho {
                context_chirho,
                name_chirho,
                type_vars_chirho,
                methods_chirho,
                associated_tfs_chirho,
                fundeps_chirho,
                span_chirho,
            } = declaration_chirho
            else {
                continue;
            };
            let [parameter_chirho] = type_vars_chirho.as_slice() else {
                return Err(self.error_chirho(
                    "next source classes currently require one parameter",
                    *span_chirho,
                ));
            };
            if !context_chirho.is_empty()
                || !associated_tfs_chirho.is_empty()
                || !fundeps_chirho.is_empty()
                || methods_chirho.is_empty()
                || parameter_chirho.kind_annotation_chirho.is_some()
                || !parameter_chirho.is_visible_chirho()
            {
                return Err(self.error_chirho("next does not yet check superclasses, associated types, fundeps or kinded class parameters", *span_chirho));
            }
            let class_chirho = self.lookup_chirho(name_chirho, NamespaceChirho::TypeChirho)?;
            let mut method_names_chirho = Vec::new();
            for method_chirho in methods_chirho {
                if method_chirho.default_chirho.is_some()
                    || method_chirho.default_sig_chirho.is_some()
                {
                    return Err(self.error_chirho(
                        "next does not yet check default methods or signatures",
                        method_chirho.span_chirho,
                    ));
                }
                if !seen_chirho.insert(method_chirho.name_chirho.full_name_chirho()) {
                    return Err(self.invalid_class_chirho(
                        "duplicate class method declaration",
                        method_chirho.span_chirho,
                    ));
                }
                method_names_chirho.push(self.bind_chirho(
                    method_chirho.name_chirho.text_chirho(),
                    NamespaceChirho::ValueChirho,
                    method_chirho.span_chirho,
                    false,
                    None,
                )?);
            }
            self.env_chirho.push_scope_chirho();
            let lowered_chirho = (|| {
                let parameter_chirho = self.bind_chirho(
                    parameter_chirho.name_chirho.text_chirho(),
                    NamespaceChirho::TypeChirho,
                    parameter_chirho.name_chirho.span_chirho(),
                    true,
                    None,
                )?;
                let mut lowered_methods_chirho = Vec::new();
                for (method_chirho, name_chirho) in methods_chirho.iter().zip(method_names_chirho) {
                    lowered_methods_chirho.push(ClassMethodChirho {
                        name_chirho,
                        ty_chirho: self.type_chirho(&method_chirho.ty_chirho, 0)?,
                        span_chirho: method_chirho.span_chirho,
                    });
                }
                Ok(ClassDeclChirho {
                    name_chirho: class_chirho,
                    parameter_chirho,
                    methods_chirho: lowered_methods_chirho,
                })
            })();
            self.env_chirho.pop_scope_chirho();
            result_chirho.push(lowered_chirho?);
        }
        Ok(result_chirho)
    }
}
