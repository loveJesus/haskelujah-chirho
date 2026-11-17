// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Defaults are ordinary signed bindings, checked even when every instance overrides them.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;

impl LowerChirho<'_> {
    pub(super) fn default_bindings_chirho(
        &mut self,
        declarations_chirho: &[DeclChirho],
        classes_chirho: &[ClassDeclChirho],
    ) -> ResultChirho<Vec<BindingChirho>> {
        let source_classes_chirho = declarations_chirho.iter().filter_map(|declaration_chirho| {
            if let DeclChirho::ClassDeclChirho {
                type_vars_chirho,
                methods_chirho,
                ..
            } = declaration_chirho
            {
                Some((type_vars_chirho, methods_chirho))
            } else {
                None
            }
        });
        let mut bindings_chirho = Vec::new();
        for ((parameters_chirho, methods_chirho), class_chirho) in
            source_classes_chirho.zip(classes_chirho)
        {
            for (method_chirho, lowered_chirho) in
                methods_chirho.iter().zip(&class_chirho.methods_chirho)
            {
                let Some(matches_chirho) = &method_chirho.default_chirho else {
                    continue;
                };
                let Some(first_chirho) = matches_chirho.first() else {
                    return Err(self.invalid_class_chirho(
                        "default has no equation",
                        method_chirho.span_chirho,
                    ));
                };
                if matches_chirho.iter().any(|arm_chirho| {
                    arm_chirho.pats_chirho.len() != first_chirho.pats_chirho.len()
                }) {
                    return Err(self.invalid_class_chirho(
                        "default equations have inconsistent arity",
                        method_chirho.span_chirho,
                    ));
                }
                let name_chirho = lowered_chirho.default_chirho.ok_or_else(|| {
                    self.invalid_class_chirho(
                        "default has no declaration-owned binding",
                        method_chirho.span_chirho,
                    )
                })?;
                self.env_chirho.push_scope_chirho();
                let binding_chirho = (|| {
                    // A fresh alpha-copy makes this signature own its rigid parameter.
                    // Reusing the class header's binder would conflate lexical scopes.
                    let parameter_chirho = &parameters_chirho[0].name_chirho;
                    let parameter_chirho = self.bind_chirho(
                        parameter_chirho.text_chirho(),
                        NamespaceChirho::TypeChirho,
                        parameter_chirho.span_chirho(),
                        true,
                        None,
                    )?;
                    self.env_chirho.push_scope_chirho();
                    let signature_chirho = self.signature_body_chirho(&method_chirho.ty_chirho);
                    let (mut signature_chirho, scoped_chirho) = match signature_chirho {
                        Ok(signature_chirho) => signature_chirho,
                        Err(error_chirho) => {
                            self.env_chirho.pop_scope_chirho();
                            return Err(error_chirho);
                        }
                    };
                    signature_chirho.binders_chirho.insert(0, parameter_chirho);
                    signature_chirho.context_chirho.insert(
                        0,
                        SurfacePredicateChirho {
                            class_chirho: class_chirho.name_chirho,
                            argument_chirho: parameter_chirho,
                        },
                    );
                    if !scoped_chirho {
                        self.env_chirho.pop_scope_chirho();
                        self.env_chirho.push_scope_chirho();
                    }
                    if !self.ghc2021_chirho {
                        self.env_chirho.pop_scope_chirho();
                        self.env_chirho.pop_scope_chirho();
                        self.env_chirho.push_scope_chirho();
                        self.env_chirho.push_scope_chirho();
                    }
                    let body_chirho =
                        self.equation_chirho(matches_chirho, method_chirho.span_chirho, 0);
                    self.env_chirho.pop_scope_chirho();
                    Ok(BindingChirho {
                        name_chirho,
                        body_chirho: body_chirho?,
                        signature_chirho: Some(signature_chirho),
                        restricted_chirho: false,
                        span_chirho: method_chirho.span_chirho,
                    })
                })();
                self.env_chirho.pop_scope_chirho();
                bindings_chirho.push(binding_chirho?);
            }
        }
        Ok(bindings_chirho)
    }
}
