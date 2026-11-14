// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declaration-owned instance binders, contexts and private implementations.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;
use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;

impl LowerChirho<'_> {
    pub(super) fn instance_declarations_chirho(
        &mut self,
        declarations_chirho: &[DeclChirho],
        classes_chirho: &[ClassDeclChirho],
    ) -> ResultChirho<Vec<InstanceDeclChirho>> {
        let mut result_chirho = Vec::new();
        for declaration_chirho in declarations_chirho {
            let DeclChirho::InstanceDeclChirho {
                unattached_members_chirho,
                context_chirho,
                class_chirho,
                types_chirho,
                methods_chirho,
                assoc_tf_instances_chirho,
                span_chirho,
            } = declaration_chirho
            else {
                continue;
            };
            self.check_member_accounts_chirho(unattached_members_chirho)?;
            let [head_chirho] = types_chirho.as_slice() else {
                return Err(self.error_chirho(
                    "next instances currently require one class argument",
                    *span_chirho,
                ));
            };
            if !assoc_tf_instances_chirho.is_empty() {
                return Err(self.error_chirho(
                    "next does not yet check associated type instances",
                    *span_chirho,
                ));
            }
            let class_id_chirho = self.lookup_chirho(class_chirho, NamespaceChirho::TypeChirho)?;
            let class_chirho = classes_chirho
                .iter()
                .find(|class_chirho| class_chirho.name_chirho == class_id_chirho)
                .ok_or_else(|| {
                    self.invalid_class_chirho(
                        "instance head has no source class declaration",
                        *span_chirho,
                    )
                })?;
            self.env_chirho.push_scope_chirho();
            let lowered_chirho = (|| {
                let mut variables_chirho = Vec::new();
                self.signature_variables_chirho(
                    head_chirho,
                    &mut variables_chirho,
                    &mut HashSet::new(),
                    0,
                )?;
                let mut binders_chirho = Vec::new();
                for variable_chirho in variables_chirho {
                    binders_chirho.push(self.bind_chirho(
                        variable_chirho.text_chirho(),
                        NamespaceChirho::TypeChirho,
                        variable_chirho.span_chirho(),
                        true,
                        None,
                    )?);
                }
                let head_chirho = self.type_chirho(head_chirho, 0)?;
                let mut predicates_chirho = Vec::new();
                for predicate_chirho in context_chirho {
                    let ConstraintChirho::ClassChirho {
                        class_chirho,
                        args_chirho,
                        span_chirho,
                    } = predicate_chirho
                    else {
                        return Err(self.error_chirho(
                            "next instance contexts require source class predicates",
                            *span_chirho,
                        ));
                    };
                    let [argument_chirho] = args_chirho.as_slice() else {
                        return Err(self.error_chirho(
                            "next instance contexts require one class argument",
                            *span_chirho,
                        ));
                    };
                    let mut argument_chirho = argument_chirho;
                    while let TypeChirho::ParenChirho { inner_chirho, .. } = argument_chirho {
                        argument_chirho = inner_chirho;
                    }
                    let TypeChirho::VarChirho(argument_chirho) = argument_chirho else {
                        return Err(self.error_chirho(
                            "next instance contexts require bare head variables",
                            *span_chirho,
                        ));
                    };
                    let argument_chirho =
                        self.lookup_chirho(argument_chirho, NamespaceChirho::TypeChirho)?;
                    if !binders_chirho.contains(&argument_chirho) {
                        return Err(self.error_chirho(
                            "next instance context variable must occur in its head",
                            *span_chirho,
                        ));
                    }
                    self.room_chirho(0, *span_chirho)?;
                    self.next_expr_chirho += 1;
                    predicates_chirho.push(SurfacePredicateChirho {
                        class_chirho: self
                            .lookup_chirho(class_chirho, NamespaceChirho::TypeChirho)?,
                        argument_chirho,
                    });
                }
                // GHC2021 scopes an instance's implicit binders over its methods.
                // Haskell2010 keeps those binders in the head/context only.
                if !self.explicit_forall_chirho {
                    self.env_chirho.pop_scope_chirho();
                    self.env_chirho.push_scope_chirho();
                }
                let name_chirho = self.bind_chirho(
                    &format!(
                        "%next:instance:{}:{}Chirho",
                        class_id_chirho.0,
                        self.names_chirho.len()
                    ),
                    NamespaceChirho::ValueChirho,
                    *span_chirho,
                    true,
                    None,
                )?;
                let mut implementations_chirho = Vec::new();
                let mut seen_chirho = HashSet::new();
                for method_chirho in methods_chirho {
                    let LocalBindChirho::FunBindChirho {
                        name_chirho,
                        matches_chirho,
                        span_chirho,
                    } = method_chirho
                    else {
                        return Err(self.error_chirho(
                        "next instance bodies require method equations without local signatures",
                        *span_chirho,
                    ));
                    };
                    let method_id_chirho = class_chirho
                        .methods_chirho
                        .iter()
                        .find(|method_chirho| {
                            self.names_chirho[method_chirho.name_chirho.0 as usize]
                                .occurrence_chirho
                                == name_chirho.full_name_chirho()
                        })
                        .map(|method_chirho| method_chirho.name_chirho)
                        .ok_or_else(|| {
                            self.invalid_class_chirho(
                                "implementation is not a method of this class",
                                *span_chirho,
                            )
                        })?;
                    let info_chirho = &self.names_chirho[method_id_chirho.0 as usize];
                    let visible_chirho = (info_chirho.unit_chirho == self.unit_chirho
                        && info_chirho.module_chirho == self.module_chirho)
                        || self
                            .import_scope_chirho
                            .get(&(name_chirho.full_name_chirho(), NamespaceChirho::ValueChirho))
                            .is_some_and(|ids_chirho| ids_chirho.contains(&method_id_chirho));
                    if !visible_chirho {
                        return Err(self.invalid_class_chirho(
                            "implementation is not a visible method of this class",
                            *span_chirho,
                        ));
                    }
                    if !seen_chirho.insert(method_id_chirho) {
                        return Err(self.invalid_class_chirho(
                            "duplicate instance method implementation",
                            *span_chirho,
                        ));
                    }
                    // Never bind the source method spelling here: references in the RHS remain
                    // the class selector, including calls at a different instance type.
                    let binding_id_chirho = self.bind_chirho(
                        &format!(
                            "%next:instance-method:{}:{}Chirho",
                            method_id_chirho.0,
                            self.names_chirho.len()
                        ),
                        NamespaceChirho::ValueChirho,
                        *span_chirho,
                        true,
                        None,
                    )?;
                    let binding_chirho = BindingChirho {
                        name_chirho: binding_id_chirho,
                        body_chirho: self.equation_chirho(matches_chirho, *span_chirho, 0)?,
                        signature_chirho: None,
                        restricted_chirho: false,
                        span_chirho: *span_chirho,
                    };
                    implementations_chirho.push(InstanceMethodChirho {
                        method_chirho: method_id_chirho,
                        binding_chirho,
                    });
                }
                for method_chirho in &class_chirho.methods_chirho {
                    if seen_chirho.contains(&method_chirho.name_chirho) {
                        continue;
                    }
                    let Some(default_chirho) = method_chirho.default_chirho else {
                        // Keep the absence explicit. TC validates the instance head before
                        // declining its missing-method runtime producer, so a visible duplicate
                        // remains a presence refusal even when its method list is incomplete.
                        continue;
                    };
                    let binding_id_chirho = self.bind_chirho(
                        &format!(
                            "%next:instance-method:{}:{}Chirho",
                            method_chirho.name_chirho.0,
                            self.names_chirho.len()
                        ),
                        NamespaceChirho::ValueChirho,
                        *span_chirho,
                        true,
                        None,
                    )?;
                    // Ordinary instantiation supplies the actual dfun and its context;
                    // there is no synthetic self Given or method-name rebinding.
                    implementations_chirho.push(InstanceMethodChirho {
                        method_chirho: method_chirho.name_chirho,
                        binding_chirho: BindingChirho {
                            name_chirho: binding_id_chirho,
                            body_chirho: self.node_chirho(
                                ExprKindChirho::VarChirho(default_chirho),
                                *span_chirho,
                            )?,
                            signature_chirho: None,
                            restricted_chirho: false,
                            span_chirho: *span_chirho,
                        },
                    });
                }
                Ok(InstanceDeclChirho {
                    name_chirho,
                    class_chirho: class_id_chirho,
                    head_chirho,
                    binders_chirho,
                    context_chirho: predicates_chirho,
                    methods_chirho: implementations_chirho,
                    span_chirho: *span_chirho,
                })
            })();
            self.env_chirho.pop_scope_chirho();
            result_chirho.push(lowered_chirho?);
        }
        Ok(result_chirho)
    }
}
