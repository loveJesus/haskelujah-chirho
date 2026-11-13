// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source class accounts and private instance implementations. Method occurrences remain selectors.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;
use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;

impl LowerChirho<'_> {
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

    pub(super) fn check_member_accounts_chirho(
        &self,
        accounts_chirho: &[haskelujah_ast_chirho::decl_chirho::UnattachedMemberChirho],
    ) -> ResultChirho<()> {
        use haskelujah_ast_chirho::decl_chirho::UnattachedMemberKindChirho;
        for account_chirho in accounts_chirho {
            let message_chirho = match account_chirho.kind_chirho {
                UnattachedMemberKindChirho::UnmatchedDefaultChirho => {
                    "default is not a declared class method"
                }
                UnattachedMemberKindChirho::RepeatedDefaultChirho => "separate method definitions",
                UnattachedMemberKindChirho::PatternBindingChirho => {
                    "pattern binding is not allowed in a class or instance body"
                }
                UnattachedMemberKindChirho::UnsupportedChirho
                | UnattachedMemberKindChirho::MinimalPragmaChirho
                | UnattachedMemberKindChirho::InlinePragmaChirho => continue,
            };
            return Err(self.invalid_class_chirho(message_chirho, account_chirho.span_chirho));
        }
        if let Some(account_chirho) = accounts_chirho.first() {
            return Err(self.error_chirho(
                "next has an unattached class or instance member",
                account_chirho.span_chirho,
            ));
        }
        Ok(())
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
                unattached_members_chirho,
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
            self.class_pragma_accounts_chirho(unattached_members_chirho, methods_chirho)?;
            let [parameter_chirho] = type_vars_chirho.as_slice() else {
                return Err(self.error_chirho(
                    "next source classes currently require one parameter",
                    *span_chirho,
                ));
            };
            if !associated_tfs_chirho.is_empty()
                || !fundeps_chirho.is_empty()
                || methods_chirho.is_empty()
                || parameter_chirho.kind_annotation_chirho.is_some()
                || !parameter_chirho.is_visible_chirho()
            {
                return Err(self.error_chirho(
                    "next does not yet check associated types, fundeps or kinded class parameters",
                    *span_chirho,
                ));
            }
            let class_chirho = self.bound_name_chirho(name_chirho, NamespaceChirho::TypeChirho)?;
            let mut method_names_chirho = Vec::new();
            for method_chirho in methods_chirho {
                if method_chirho.default_sig_chirho.is_some() {
                    return Err(self.error_chirho(
                        "next does not yet check default signatures",
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
                let mut superclasses_chirho = Vec::new();
                let mut superclass_selectors_chirho = Vec::new();
                for constraint_chirho in context_chirho {
                    let ConstraintChirho::ClassChirho {
                        class_chirho: superclass_chirho,
                        args_chirho,
                        span_chirho,
                    } = constraint_chirho
                    else {
                        return Err(self.error_chirho(
                            "next does not yet check quantified superclasses",
                            *span_chirho,
                        ));
                    };
                    let [argument_chirho] = args_chirho.as_slice() else {
                        return Err(self.error_chirho(
                            "next superclasses require one class argument",
                            *span_chirho,
                        ));
                    };
                    let mut argument_chirho = argument_chirho;
                    while let TypeChirho::ParenChirho { inner_chirho, .. } = argument_chirho {
                        argument_chirho = inner_chirho;
                    }
                    let TypeChirho::VarChirho(variable_chirho) = argument_chirho else {
                        return Err(self.error_chirho(
                            "next superclasses require the class's bare parameter",
                            *span_chirho,
                        ));
                    };
                    let argument_chirho =
                        self.lookup_chirho(variable_chirho, NamespaceChirho::TypeChirho)?;
                    if argument_chirho != parameter_chirho {
                        return Err(self.error_chirho(
                            "next superclasses require the class's own parameter",
                            *span_chirho,
                        ));
                    }
                    superclasses_chirho.push(SurfacePredicateChirho {
                        class_chirho: self
                            .lookup_chirho(superclass_chirho, NamespaceChirho::TypeChirho)?,
                        argument_chirho,
                    });
                    superclass_selectors_chirho.push(self.bind_chirho(
                        &format!(
                            "%next:superclass:{}:{}Chirho",
                            class_chirho.0,
                            superclass_selectors_chirho.len()
                        ),
                        NamespaceChirho::ValueChirho,
                        *span_chirho,
                        true,
                        None,
                    )?);
                }
                let mut lowered_methods_chirho = Vec::new();
                for (method_chirho, name_chirho) in methods_chirho.iter().zip(method_names_chirho) {
                    let default_chirho = if method_chirho.default_chirho.is_some() {
                        Some(self.bind_chirho(
                            &format!("%next:default:{}Chirho", name_chirho.0),
                            NamespaceChirho::ValueChirho,
                            method_chirho.span_chirho,
                            true,
                            None,
                        )?)
                    } else {
                        None
                    };
                    lowered_methods_chirho.push(ClassMethodChirho {
                        name_chirho,
                        default_chirho,
                        ty_chirho: self.type_chirho(&method_chirho.ty_chirho, 0)?,
                        span_chirho: method_chirho.span_chirho,
                    });
                }
                Ok(ClassDeclChirho {
                    name_chirho: class_chirho,
                    parameter_chirho,
                    superclasses_chirho,
                    superclass_selectors_chirho,
                    methods_chirho: lowered_methods_chirho,
                })
            })();
            self.env_chirho.pop_scope_chirho();
            result_chirho.push(lowered_chirho?);
        }
        Ok(result_chirho)
    }
}
