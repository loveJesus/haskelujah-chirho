// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Main's entry instantiates its checked scheme without changing other uses of main.
//! workflow: next-pipeline-chirho/io-sequencing-chirho
use super::*;
use haskelujah_hir_chirho::{ExprIdChirho, NameInfoChirho, NamespaceChirho};

impl CheckedModuleChirho {
    pub(super) fn main_instantiation_chirho(
        &mut self,
        source_chirho: NameIdChirho,
        ty_chirho: TyChirho,
        arguments_chirho: Vec<TyChirho>,
    ) -> Result<(), TcErrorChirho> {
        for _entry_chirho in 0..10 + arguments_chirho.len() {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
        }
        let name_chirho = NameIdChirho(
            u32::try_from(self.module_chirho.names_chirho.len()).map_err(|_| {
                TcErrorChirho::new_chirho(
                    TcErrorKindChirho::ResourceChirho,
                    "main entry name capacity exhausted",
                )
            })?,
        );
        let mut expression_chirho = 0u32;
        for id_chirho in self
            .expressions_chirho
            .keys()
            .chain(self.patterns_chirho.keys())
        {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            expression_chirho =
                expression_chirho.max(id_chirho.0.checked_add(1).ok_or_else(|| {
                    TcErrorChirho::new_chirho(
                        TcErrorKindChirho::ResourceChirho,
                        "main entry expression capacity exhausted",
                    )
                })?);
        }
        let expression_chirho = ExprIdChirho(expression_chirho);
        let span_chirho = self.module_chirho.names_chirho[source_chirho.0 as usize].span_chirho;
        self.module_chirho.names_chirho.push(NameInfoChirho {
            unit_chirho: self.module_chirho.unit_chirho.clone(),
            module_chirho: self.module_chirho.module_chirho.clone(),
            namespace_chirho: NamespaceChirho::ValueChirho,
            occurrence_chirho: crate::MAIN_ENTRY_CHIRHO.into(),
            local_chirho: false,
            wired_chirho: None,
            span_chirho,
        });
        let scope_chirho = self.solver_chirho.implication_kind_chirho(
            self.solver_chirho.root_chirho(),
            false,
            crate::ScopeKindChirho::SignatureChirho,
        )?;
        self.names_chirho.insert(name_chirho, ty_chirho);
        self.expressions_chirho.insert(expression_chirho, ty_chirho);
        self.schemes_chirho.insert(
            name_chirho,
            SchemeChirho {
                body_chirho: ty_chirho,
                quantified_chirho: Vec::new(),
                context_chirho: Vec::new(),
            },
        );
        self.abstractions_chirho.insert(
            name_chirho,
            AbstractionChirho {
                type_binders_chirho: Vec::new(),
                dictionary_binders_chirho: Vec::new(),
                scope_chirho,
            },
        );
        self.wrappers_chirho.insert(
            expression_chirho,
            arguments_chirho
                .into_iter()
                .map(WrapStepChirho::TypeChirho)
                .collect(),
        );
        self.module_chirho.bindings_chirho.push(BindingChirho {
            name_chirho,
            body_chirho: ExprChirho {
                id_chirho: expression_chirho,
                kind_chirho: ExprKindChirho::VarChirho(source_chirho),
                span_chirho,
            },
            signature_chirho: None,
            restricted_chirho: false,
            span_chirho,
        });
        Ok(())
    }
}
