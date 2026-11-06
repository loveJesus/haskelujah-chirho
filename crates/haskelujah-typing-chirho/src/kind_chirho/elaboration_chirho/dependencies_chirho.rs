// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Dependency admission for declaration kind groups. The graph grouping
//! algorithm is shared with binding inference; source spellings resolve first.

use super::*;
use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DataKindSigChirho, DeclChirho, GadtConArgsChirho, GadtConPrefixChirho,
};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use std::collections::{BTreeSet, HashMap};

pub(super) struct DeclarationOrderChirho {
    pub(super) groups_chirho: Vec<Vec<usize>>,
    pub(super) aliases_chirho: Vec<usize>,
}

impl TypeElaboratorChirho<'_> {
    pub(super) fn declaration_groups_chirho(
        &mut self,
        module_chirho: &ModuleChirho,
    ) -> Result<DeclarationOrderChirho, ElaborationErrorChirho> {
        let mut declarations_chirho = Vec::new();
        let mut indices_chirho = HashMap::new();
        let mut constructor_owners_chirho = HashMap::new();
        for (source_index_chirho, declaration_chirho) in
            module_chirho.decls_chirho.iter().enumerate()
        {
            let name_chirho = match declaration_chirho {
                DeclChirho::DataDeclChirho { name_chirho, .. }
                | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
                | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
                | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. }
                | DeclChirho::ClassDeclChirho { name_chirho, .. } => name_chirho,
                _ => continue,
            };
            let Some(proof_chirho) = self.declaration_chirho(name_chirho)? else {
                continue;
            };
            if declarations_chirho.len() >= 512 {
                return Err(Self::error_chirho(
                    "kind declaration group size bound exhausted",
                    name_chirho.span_chirho(),
                ));
            }
            let key_chirho =
                self.declaration_key_chirho(&proof_chirho, name_chirho.span_chirho())?;
            self.state_chirho
                .charge_chirho(1, 1, name_chirho.span_chirho())?;
            indices_chirho.insert(key_chirho, declarations_chirho.len());
            declarations_chirho.push(source_index_chirho);
            let constructors_chirho: &[ConDeclChirho] = match declaration_chirho {
                DeclChirho::DataDeclChirho {
                    constructors_chirho,
                    ..
                } => constructors_chirho,
                DeclChirho::NewtypeDeclChirho {
                    constructor_chirho, ..
                } => std::slice::from_ref(constructor_chirho),
                _ => &[],
            };
            for constructor_chirho in constructors_chirho {
                let constructor_name_chirho = match constructor_chirho {
                    ConDeclChirho::OrdinaryChirho { name_chirho, .. }
                    | ConDeclChirho::RecordChirho { name_chirho, .. }
                    | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho,
                };
                let written_chirho = self.written_name_chirho(constructor_name_chirho)?;
                constructor_owners_chirho.insert(written_chirho, declarations_chirho.len() - 1);
            }
        }
        self.state_chirho.charge_chirho(
            declarations_chirho.len() * 4,
            declarations_chirho.len() * 4,
            module_chirho.span_chirho,
        )?;
        let mut adjacency_chirho = vec![Vec::new(); declarations_chirho.len()];
        for (node_chirho, source_index_chirho) in declarations_chirho.iter().enumerate() {
            let mut references_chirho = BTreeSet::new();
            self.declaration_references_chirho(
                &module_chirho.decls_chirho[*source_index_chirho],
                &indices_chirho,
                &constructor_owners_chirho,
                &mut references_chirho,
            )?;
            adjacency_chirho[node_chirho].extend(references_chirho);
        }
        // A nominal data/class boundary breaks synonym expansion. Reject only
        // cycles made entirely of transparent aliases, using the shared SCC code.
        let mut aliases_chirho = vec![Vec::new(); declarations_chirho.len()];
        for (node_chirho, source_index_chirho) in declarations_chirho.iter().enumerate() {
            if matches!(
                module_chirho.decls_chirho[*source_index_chirho],
                DeclChirho::TypeAliasDeclChirho { .. }
            ) {
                for dependency_chirho in &adjacency_chirho[node_chirho] {
                    self.state_chirho
                        .charge_chirho(1, 1, module_chirho.span_chirho)?;
                    if matches!(
                        module_chirho.decls_chirho[declarations_chirho[*dependency_chirho]],
                        DeclChirho::TypeAliasDeclChirho { .. }
                    ) {
                        aliases_chirho[node_chirho].push(*dependency_chirho);
                    }
                }
            }
        }
        let alias_groups_chirho = crate::infer_chirho::dependency_groups_chirho(&aliases_chirho);
        for group_chirho in &alias_groups_chirho {
            if group_chirho.len() > 1
                || group_chirho
                    .first()
                    .is_some_and(|node_chirho| aliases_chirho[*node_chirho].contains(node_chirho))
            {
                return Err(Self::error_chirho(
                    "recursive type synonym declarations have no finite elaboration",
                    module_chirho.span_chirho,
                ));
            }
        }
        let aliases_chirho = alias_groups_chirho
            .into_iter()
            .flatten()
            .filter_map(|node_chirho| {
                let source_index_chirho = declarations_chirho[node_chirho];
                matches!(
                    module_chirho.decls_chirho[source_index_chirho],
                    DeclChirho::TypeAliasDeclChirho { .. }
                )
                .then_some(source_index_chirho)
            })
            .collect();
        let groups_chirho = crate::infer_chirho::dependency_groups_chirho(&adjacency_chirho)
            .into_iter()
            .map(|group_chirho| {
                group_chirho
                    .into_iter()
                    .map(|node_chirho| declarations_chirho[node_chirho])
                    .collect()
            })
            .collect();
        Ok(DeclarationOrderChirho {
            groups_chirho,
            aliases_chirho,
        })
    }

    fn reference_chirho(
        &mut self,
        name_chirho: &NameChirho,
        promoted_chirho: bool,
        indices_chirho: &HashMap<Arc<str>, usize>,
        constructor_owners_chirho: &HashMap<String, usize>,
        references_chirho: &mut BTreeSet<usize>,
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho
            .charge_chirho(1, 1, name_chirho.span_chirho())?;
        if promoted_chirho {
            let written_chirho = self.written_name_chirho(name_chirho)?;
            if let Some(index_chirho) = constructor_owners_chirho.get(&written_chirho) {
                references_chirho.insert(*index_chirho);
            }
            if let Some(key_chirho) =
                haskelujah_ast_chirho::promotion_chirho::quoted_promoted_spelling_chirho(
                    name_chirho,
                    Some(self.promotion_scope_chirho),
                )
                && let Some(index_chirho) = constructor_owners_chirho.get(&key_chirho)
            {
                references_chirho.insert(*index_chirho);
            }
            return Ok(());
        }
        if let Some(proof_chirho) = self.declaration_chirho(name_chirho)? {
            let key_chirho =
                self.declaration_key_chirho(&proof_chirho, name_chirho.span_chirho())?;
            if let Some(index_chirho) = indices_chirho.get(&key_chirho) {
                references_chirho.insert(*index_chirho);
            }
        }
        if let Some(key_chirho) =
            haskelujah_ast_chirho::promotion_chirho::unquoted_promoted_spelling_chirho(
                name_chirho,
                Some(self.promotion_scope_chirho),
            )
            && let Some(index_chirho) = constructor_owners_chirho.get(&key_chirho)
        {
            references_chirho.insert(*index_chirho);
        }
        Ok(())
    }

    fn type_references_chirho(
        &mut self,
        ty_chirho: &TypeChirho,
        indices_chirho: &HashMap<Arc<str>, usize>,
        constructor_owners_chirho: &HashMap<String, usize>,
        references_chirho: &mut BTreeSet<usize>,
        depth_chirho: usize,
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho
            .charge_chirho(1, 0, ty_chirho.span_chirho())?;
        if depth_chirho > 128 {
            return Err(Self::error_chirho(
                "kind dependency syntax depth bound exhausted",
                ty_chirho.span_chirho(),
            ));
        }
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) => self.reference_chirho(
                name_chirho,
                false,
                indices_chirho,
                constructor_owners_chirho,
                references_chirho,
            )?,
            TypeChirho::PromotedConChirho { name_chirho, .. } => self.reference_chirho(
                name_chirho,
                true,
                indices_chirho,
                constructor_owners_chirho,
                references_chirho,
            )?,
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
                self.type_references_chirho(
                    fun_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    depth_chirho + 1,
                )?;
                self.type_references_chirho(
                    arg_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    depth_chirho + 1,
                )?;
            }
            TypeChirho::KindAnnotationChirho {
                ty_chirho,
                kind_chirho,
                ..
            } => {
                self.type_references_chirho(
                    ty_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    depth_chirho + 1,
                )?;
                self.type_references_chirho(
                    kind_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    depth_chirho + 1,
                )?;
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                self.type_references_chirho(
                    arg_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    depth_chirho + 1,
                )?;
                self.type_references_chirho(
                    result_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    depth_chirho + 1,
                )?;
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    self.type_references_chirho(
                        element_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        depth_chirho + 1,
                    )?;
                }
            }
            TypeChirho::ListChirho { element_chirho, .. } => self.type_references_chirho(
                element_chirho,
                indices_chirho,
                constructor_owners_chirho,
                references_chirho,
                depth_chirho + 1,
            )?,
            TypeChirho::ParenChirho { inner_chirho, .. } => self.type_references_chirho(
                inner_chirho,
                indices_chirho,
                constructor_owners_chirho,
                references_chirho,
                depth_chirho + 1,
            )?,
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                for constraint_chirho in context_chirho {
                    self.constraint_references_chirho(
                        constraint_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        depth_chirho + 1,
                    )?;
                }
                self.type_references_chirho(
                    body_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    depth_chirho + 1,
                )?;
            }
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
                for variable_chirho in vars_chirho {
                    if let Some(kind_chirho) = &variable_chirho.kind_annotation_chirho {
                        self.type_references_chirho(
                            kind_chirho,
                            indices_chirho,
                            constructor_owners_chirho,
                            references_chirho,
                            depth_chirho + 1,
                        )?;
                    }
                }
                self.type_references_chirho(
                    body_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    depth_chirho + 1,
                )?;
            }
            TypeChirho::VarChirho(_)
            | TypeChirho::WildcardChirho { .. }
            | TypeChirho::LitChirho { .. } => {}
        }
        Ok(())
    }

    fn constraint_references_chirho(
        &mut self,
        constraint_chirho: &ConstraintChirho,
        indices_chirho: &HashMap<Arc<str>, usize>,
        constructor_owners_chirho: &HashMap<String, usize>,
        references_chirho: &mut BTreeSet<usize>,
        depth_chirho: usize,
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho
            .charge_chirho(1, 0, constraint_chirho.span_chirho())?;
        if depth_chirho > 128 {
            return Err(Self::error_chirho(
                "kind dependency constraint depth bound exhausted",
                constraint_chirho.span_chirho(),
            ));
        }
        match constraint_chirho {
            ConstraintChirho::ClassChirho {
                class_chirho,
                args_chirho,
                kind_args_chirho,
                ..
            } => {
                self.reference_chirho(
                    class_chirho,
                    false,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                )?;
                for arg_chirho in kind_args_chirho.iter().chain(args_chirho) {
                    self.type_references_chirho(
                        arg_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        depth_chirho + 1,
                    )?;
                }
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => {
                for binder_chirho in vars_chirho {
                    if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                        self.type_references_chirho(
                            kind_chirho,
                            indices_chirho,
                            constructor_owners_chirho,
                            references_chirho,
                            depth_chirho + 1,
                        )?;
                    }
                }
                for constraint_chirho in context_chirho {
                    self.constraint_references_chirho(
                        constraint_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        depth_chirho + 1,
                    )?;
                }
                self.constraint_references_chirho(
                    body_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    depth_chirho + 1,
                )?;
            }
            ConstraintChirho::KindAnnotationChirho {
                constraint_chirho,
                kind_chirho,
                ..
            } => {
                self.constraint_references_chirho(
                    constraint_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    depth_chirho + 1,
                )?;
                self.type_references_chirho(
                    kind_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    depth_chirho + 1,
                )?;
            }
        }
        Ok(())
    }

    fn declaration_references_chirho(
        &mut self,
        declaration_chirho: &DeclChirho,
        indices_chirho: &HashMap<Arc<str>, usize>,
        constructor_owners_chirho: &HashMap<String, usize>,
        references_chirho: &mut BTreeSet<usize>,
    ) -> Result<(), ElaborationErrorChirho> {
        let (binders_chirho, kind_sig_chirho, constructors_chirho) = match declaration_chirho {
            DeclChirho::DataDeclChirho {
                type_vars_chirho,
                kind_sig_chirho,
                constructors_chirho,
                ..
            } => (
                type_vars_chirho.as_slice(),
                kind_sig_chirho.as_ref(),
                constructors_chirho.as_slice(),
            ),
            DeclChirho::NewtypeDeclChirho {
                type_vars_chirho,
                kind_sig_chirho,
                constructor_chirho,
                ..
            } => (
                type_vars_chirho.as_slice(),
                kind_sig_chirho.as_ref(),
                std::slice::from_ref(constructor_chirho),
            ),
            DeclChirho::TypeFamilyDeclChirho {
                type_vars_chirho,
                result_kind_chirho,
                standalone_kind_chirho,
                equations_chirho,
                ..
            } => {
                for kind_chirho in result_kind_chirho.iter().chain(standalone_kind_chirho) {
                    self.type_references_chirho(
                        kind_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        0,
                    )?;
                }
                for equation_chirho in equations_chirho {
                    for arg_chirho in equation_chirho
                        .kind_args_chirho
                        .iter()
                        .chain(&equation_chirho.lhs_types_chirho)
                        .chain(std::iter::once(&equation_chirho.rhs_chirho))
                    {
                        self.type_references_chirho(
                            arg_chirho,
                            indices_chirho,
                            constructor_owners_chirho,
                            references_chirho,
                            0,
                        )?;
                    }
                }
                (type_vars_chirho.as_slice(), None, &[][..])
            }
            DeclChirho::TypeAliasDeclChirho {
                type_vars_chirho,
                rhs_chirho,
                ..
            } => {
                self.type_references_chirho(
                    rhs_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    0,
                )?;
                (type_vars_chirho.as_slice(), None, &[][..])
            }
            DeclChirho::ClassDeclChirho {
                standalone_kind_chirho,
                type_vars_chirho,
                context_chirho,
                methods_chirho,
                associated_tfs_chirho,
                ..
            } => {
                if let Some(kind_chirho) = standalone_kind_chirho {
                    self.type_references_chirho(
                        kind_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        0,
                    )?;
                }
                for constraint_chirho in context_chirho {
                    self.constraint_references_chirho(
                        constraint_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        0,
                    )?;
                }
                for method_chirho in methods_chirho {
                    self.type_references_chirho(
                        &method_chirho.ty_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        0,
                    )?;
                }
                for family_chirho in associated_tfs_chirho {
                    if let Some((kinds_chirho, types_chirho)) =
                        &family_chirho.default_arguments_chirho
                    {
                        for argument_chirho in kinds_chirho.iter().chain(types_chirho) {
                            self.type_references_chirho(
                                argument_chirho,
                                indices_chirho,
                                constructor_owners_chirho,
                                references_chirho,
                                0,
                            )?;
                        }
                    }
                    for binder_chirho in &family_chirho.type_vars_chirho {
                        if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                            self.type_references_chirho(
                                kind_chirho,
                                indices_chirho,
                                constructor_owners_chirho,
                                references_chirho,
                                0,
                            )?;
                        }
                    }
                    if let Some(rhs_chirho) = &family_chirho.default_rhs_chirho {
                        self.type_references_chirho(
                            rhs_chirho,
                            indices_chirho,
                            constructor_owners_chirho,
                            references_chirho,
                            0,
                        )?;
                    }
                    if let Some(kind_chirho) = &family_chirho.result_kind_chirho {
                        self.type_references_chirho(
                            kind_chirho,
                            indices_chirho,
                            constructor_owners_chirho,
                            references_chirho,
                            0,
                        )?;
                    }
                }
                (type_vars_chirho.as_slice(), None, &[][..])
            }
            DeclChirho::TypeFamilyInstanceDeclChirho {
                family_name_chirho,
                kind_args_chirho,
                lhs_types_chirho,
                rhs_chirho,
                ..
            } => {
                self.reference_chirho(
                    family_name_chirho,
                    false,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                )?;
                for ty_chirho in kind_args_chirho
                    .iter()
                    .chain(lhs_types_chirho)
                    .chain(std::iter::once(rhs_chirho))
                {
                    self.type_references_chirho(
                        ty_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        0,
                    )?;
                }
                (&[][..], None, &[][..])
            }
            DeclChirho::InstanceDeclChirho {
                context_chirho,
                class_chirho,
                kind_args_chirho,
                types_chirho,
                assoc_tf_instances_chirho,
                ..
            } => {
                self.reference_chirho(
                    class_chirho,
                    false,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                )?;
                for constraint_chirho in context_chirho {
                    self.constraint_references_chirho(
                        constraint_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        0,
                    )?;
                }
                for ty_chirho in kind_args_chirho.iter().chain(types_chirho) {
                    self.type_references_chirho(
                        ty_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        0,
                    )?;
                }
                for equation_chirho in assoc_tf_instances_chirho {
                    self.reference_chirho(
                        &equation_chirho.family_name_chirho,
                        false,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                    )?;
                    for ty_chirho in equation_chirho
                        .kind_args_chirho
                        .iter()
                        .chain(&equation_chirho.lhs_types_chirho)
                        .chain(std::iter::once(&equation_chirho.rhs_chirho))
                    {
                        self.type_references_chirho(
                            ty_chirho,
                            indices_chirho,
                            constructor_owners_chirho,
                            references_chirho,
                            0,
                        )?;
                    }
                }
                (&[][..], None, &[][..])
            }
            _ => return Ok(()),
        };
        for binder_chirho in binders_chirho {
            if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                self.type_references_chirho(
                    kind_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    0,
                )?;
            }
        }
        if let Some(kind_sig_chirho) = kind_sig_chirho {
            match kind_sig_chirho {
                DataKindSigChirho::ResultChirho(kind_chirho) => self.type_references_chirho(
                    kind_chirho,
                    indices_chirho,
                    constructor_owners_chirho,
                    references_chirho,
                    0,
                )?,
                DataKindSigChirho::StandaloneChirho {
                    signature_chirho,
                    result_chirho,
                } => {
                    self.type_references_chirho(
                        signature_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        0,
                    )?;
                    if let Some(kind_chirho) = result_chirho {
                        self.type_references_chirho(
                            kind_chirho,
                            indices_chirho,
                            constructor_owners_chirho,
                            references_chirho,
                            0,
                        )?;
                    }
                }
            }
        }
        for constructor_chirho in constructors_chirho {
            if let ConDeclChirho::OrdinaryChirho { prefix_chirho, .. }
            | ConDeclChirho::RecordChirho { prefix_chirho, .. } = constructor_chirho
            {
                for binder_chirho in &prefix_chirho.existential_vars_chirho {
                    if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                        self.type_references_chirho(
                            kind_chirho,
                            indices_chirho,
                            constructor_owners_chirho,
                            references_chirho,
                            0,
                        )?;
                    }
                }
                for constraint_chirho in &prefix_chirho.context_chirho {
                    self.constraint_references_chirho(
                        constraint_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        0,
                    )?;
                }
            }
            match constructor_chirho {
                ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
                    for (_, field_chirho) in fields_chirho {
                        self.type_references_chirho(
                            field_chirho,
                            indices_chirho,
                            constructor_owners_chirho,
                            references_chirho,
                            0,
                        )?;
                    }
                }
                ConDeclChirho::RecordChirho { fields_chirho, .. } => {
                    for field_chirho in fields_chirho {
                        self.type_references_chirho(
                            &field_chirho.ty_chirho,
                            indices_chirho,
                            constructor_owners_chirho,
                            references_chirho,
                            0,
                        )?;
                    }
                }
                ConDeclChirho::GadtChirho {
                    signature_chirho, ..
                } => {
                    for prefix_chirho in &signature_chirho.prefix_chirho {
                        match prefix_chirho {
                            GadtConPrefixChirho::ForallChirho { vars_chirho, .. } => {
                                for binder_chirho in vars_chirho {
                                    if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho
                                    {
                                        self.type_references_chirho(
                                            kind_chirho,
                                            indices_chirho,
                                            constructor_owners_chirho,
                                            references_chirho,
                                            0,
                                        )?;
                                    }
                                }
                            }
                            GadtConPrefixChirho::ContextChirho {
                                constraints_chirho, ..
                            } => {
                                for constraint_chirho in constraints_chirho {
                                    self.constraint_references_chirho(
                                        constraint_chirho,
                                        indices_chirho,
                                        constructor_owners_chirho,
                                        references_chirho,
                                        0,
                                    )?;
                                }
                            }
                        }
                    }
                    match &signature_chirho.args_chirho {
                        GadtConArgsChirho::PositionalChirho(fields_chirho) => {
                            for field_chirho in fields_chirho {
                                self.type_references_chirho(
                                    &field_chirho.ty_chirho,
                                    indices_chirho,
                                    constructor_owners_chirho,
                                    references_chirho,
                                    0,
                                )?;
                            }
                        }
                        GadtConArgsChirho::RecordChirho(fields_chirho) => {
                            for field_chirho in fields_chirho {
                                self.type_references_chirho(
                                    &field_chirho.ty_chirho,
                                    indices_chirho,
                                    constructor_owners_chirho,
                                    references_chirho,
                                    0,
                                )?;
                            }
                        }
                    }
                    self.type_references_chirho(
                        &signature_chirho.result_ty_chirho,
                        indices_chirho,
                        constructor_owners_chirho,
                        references_chirho,
                        0,
                    )?;
                }
            }
        }
        Ok(())
    }
}

/// Checked source scheduling uses the same bounded declaration walker as local
/// SCC elaboration. Importing a module alone does not create a type dependency.
pub fn source_declaration_dependencies_chirho(
    module_chirho: &ModuleChirho,
    imports_chirho: ElaborationImportsChirho<'_>,
    modules_chirho: &[&ModuleChirho],
) -> Result<BTreeSet<String>, ElaborationErrorChirho> {
    use haskelujah_ast_chirho::name_chirho::TypeConstructorIdentityChirho;
    let mut elaborator_chirho = TypeElaboratorChirho::new_chirho(module_chirho, imports_chirho)?;
    let mut names_chirho = Vec::new();
    let mut indices_chirho = HashMap::new();
    let mut constructors_chirho = HashMap::new();
    for (index_chirho, source_chirho) in modules_chirho.iter().enumerate() {
        let module_name_chirho = source_chirho.name_chirho.full_name_chirho();
        elaborator_chirho.state_chirho.charge_chirho(
            module_name_chirho.len() + 1,
            module_name_chirho.len() + 1,
            module_chirho.span_chirho,
        )?;
        names_chirho.push(module_name_chirho.clone());
        for declaration_chirho in &source_chirho.decls_chirho {
            let members_chirho = match declaration_chirho {
                DeclChirho::DataDeclChirho {
                    constructors_chirho,
                    ..
                } => constructors_chirho.as_slice(),
                DeclChirho::NewtypeDeclChirho {
                    constructor_chirho, ..
                } => std::slice::from_ref(constructor_chirho),
                _ => &[],
            };
            for constructor_chirho in members_chirho {
                let name_chirho = match constructor_chirho {
                    ConDeclChirho::OrdinaryChirho { name_chirho, .. }
                    | ConDeclChirho::RecordChirho { name_chirho, .. }
                    | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho,
                };
                elaborator_chirho.state_chirho.charge_chirho(
                    module_name_chirho.len() + name_chirho.text_chirho().len() + 128,
                    module_name_chirho.len() + name_chirho.text_chirho().len() + 128,
                    name_chirho.span_chirho(),
                )?;
                let key_chirho =
                    haskelujah_ast_chirho::promotion_chirho::PromotedKeyChirho::source_chirho(
                        &module_name_chirho,
                        name_chirho.text_chirho(),
                    );
                constructors_chirho.insert(key_chirho.as_str_chirho().to_owned(), index_chirho);
            }
        }
    }
    for (_, declaration_chirho) in elaborator_chirho.type_scope_chirho.iter_chirho() {
        if let TypeConstructorIdentityChirho::SourceChirho {
            defining_module_chirho,
            ..
        } = &declaration_chirho.identity_chirho
            && let Some(index_chirho) = names_chirho
                .iter()
                .position(|name_chirho| name_chirho == defining_module_chirho)
        {
            let key_chirho = declaration_chirho.type_key_chirho();
            elaborator_chirho.state_chirho.charge_chirho(
                key_chirho.len() + 1,
                key_chirho.len() + 1,
                module_chirho.span_chirho,
            )?;
            indices_chirho.insert(Arc::from(key_chirho), index_chirho);
        }
    }
    let mut references_chirho = BTreeSet::new();
    for declaration_chirho in &module_chirho.decls_chirho {
        elaborator_chirho.declaration_references_chirho(
            declaration_chirho,
            &indices_chirho,
            &constructors_chirho,
            &mut references_chirho,
        )?;
    }
    let own_chirho = module_chirho.name_chirho.full_name_chirho();
    Ok(references_chirho
        .into_iter()
        .filter(|index_chirho| names_chirho[*index_chirho] != own_chirho)
        .map(|index_chirho| names_chirho[index_chirho].clone())
        .collect())
}
