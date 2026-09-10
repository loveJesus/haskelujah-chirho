// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A group owns its unsolved kinds until its bodies have supplied constraints.
//! Complete signatures break inference cycles, but not declaration-checking work.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::dependencies_chirho::KindDependenciesChirho;
use super::{
    DeclChirho, KindBindingChirho, KindChirho, KindInferCtxChirho, KindTypeSynonymChirho,
    KindVarChirho, ModuleChirho,
};
use crate::dependency_chirho::dependency_groups_chirho;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct PreparedKindScopeChirho {
    bindings_chirho: Vec<(String, KindBindingChirho)>,
    names_chirho: HashMap<String, KindVarChirho>,
    written_chirho: Vec<(KindVarChirho, super::SpanChirho)>,
}

impl KindInferCtxChirho {
    pub(super) fn check_kind_declarations_chirho(&mut self, module_chirho: &ModuleChirho) {
        let graph_chirho = KindDependenciesChirho::new_chirho(module_chirho);
        for names_chirho in &graph_chirho.names_chirho {
            for &name_chirho in names_chirho {
                self.local_kind_decl_names_chirho
                    .insert(name_chirho.to_owned());
                self.env_chirho.bindings_chirho.remove(name_chirho);
            }
        }
        // Aliases used as kind syntax are expanded independently of source order.
        // Their inferred kinds still belong to the dependency groups below.
        for declaration_chirho in &graph_chirho.declarations_chirho {
            if let DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                rhs_chirho,
                ..
            } = declaration_chirho
            {
                self.type_kind_synonyms_chirho.insert(
                    name_chirho.text_chirho().to_owned(),
                    KindTypeSynonymChirho {
                        params_chirho: type_vars_chirho
                            .iter()
                            .map(|binder_chirho| binder_chirho.text_chirho().to_owned())
                            .collect(),
                        rhs_chirho: rhs_chirho.clone(),
                    },
                );
            }
        }

        for mut group_chirho in dependency_groups_chirho(&graph_chirho.edges_chirho) {
            group_chirho.sort_unstable();
            // A forward LOCAL reference is a shared hole, never an opaque import
            // whose kind may be independently instantiated at every occurrence.
            for &index_chirho in &group_chirho {
                for &name_chirho in &graph_chirho.names_chirho[index_chirho] {
                    let kind_chirho = self.fresh_kind_chirho();
                    self.env_chirho
                        .bind_chirho(name_chirho.to_owned(), kind_chirho);
                }
            }
            let mut scopes_chirho: Vec<_> = group_chirho
                .iter()
                .map(|&index_chirho| {
                    self.prepare_kind_declaration_chirho(
                        graph_chirho.declarations_chirho[index_chirho],
                        &graph_chirho.names_chirho[index_chirho],
                    )
                })
                .collect();

            // All complete heads now exist. An incomplete declaration that refers
            // back to one of them can be inferred before that signed body's uses
            // constrain it. A single SCC pass without this split is too early.
            let local_indices_chirho: HashMap<_, _> = group_chirho
                .iter()
                .enumerate()
                .map(|(local_chirho, &global_chirho)| (global_chirho, local_chirho))
                .collect();
            let complete_chirho: HashSet<_> = group_chirho
                .iter()
                .copied()
                .filter(|&index_chirho| {
                    graph_chirho.names_chirho[index_chirho]
                        .iter()
                        .all(|name_chirho| {
                            matches!(
                                self.env_chirho.lookup_binding_chirho(name_chirho),
                                Some(KindBindingChirho::PolyChirho(_))
                            )
                        })
                })
                .collect();
            let inference_edges_chirho: Vec<Vec<usize>> = group_chirho
                .iter()
                .map(|&index_chirho| {
                    graph_chirho.edges_chirho[index_chirho]
                        .iter()
                        .filter(|target_chirho| !complete_chirho.contains(target_chirho))
                        .filter_map(|target_chirho| {
                            local_indices_chirho.get(target_chirho).copied()
                        })
                        .collect()
                })
                .collect();
            for inference_group_chirho in dependency_groups_chirho(&inference_edges_chirho) {
                let mut written_chirho = Vec::new();
                for &local_chirho in &inference_group_chirho {
                    let scope_chirho = std::mem::take(&mut scopes_chirho[local_chirho]);
                    written_chirho.extend(scope_chirho.written_chirho);
                    self.env_chirho.begin_scope_chirho();
                    for (name_chirho, binding_chirho) in scope_chirho.bindings_chirho {
                        self.env_chirho
                            .bind_entry_chirho(name_chirho, binding_chirho);
                    }
                    self.kind_var_cache_chirho = scope_chirho.names_chirho;
                    self.check_kind_declaration_body_chirho(
                        graph_chirho.declarations_chirho[group_chirho[local_chirho]],
                    );
                    self.kind_var_cache_chirho.clear();
                    self.env_chirho.end_scope_chirho();
                }
                self.check_written_kind_group_chirho(&written_chirho);
                // Resolve the entire recursive group before closing any variables.
                for &local_chirho in &inference_group_chirho {
                    for &name_chirho in &graph_chirho.names_chirho[group_chirho[local_chirho]] {
                        self.publish_kind_chirho(name_chirho);
                    }
                }
            }
        }

        // A value signature consumes finalized declaration kinds. Its text order
        // cannot turn a forward type constructor into an imported placeholder.
        for declaration_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::TypeSigChirho {
                ty_chirho,
                span_chirho,
                ..
            } = declaration_chirho
            {
                self.env_chirho.begin_scope_chirho();
                self.kind_var_cache_chirho.clear();
                let kind_chirho = self.infer_type_kind_chirho(ty_chirho);
                self.check_runtime_kind_chirho(&kind_chirho, "type signature", *span_chirho);
                self.kind_var_cache_chirho.clear();
                self.env_chirho.end_scope_chirho();
            }
        }
    }

    fn prepare_kind_declaration_chirho(
        &mut self,
        declaration_chirho: &DeclChirho,
        declared_names_chirho: &[&str],
    ) -> PreparedKindScopeChirho {
        self.env_chirho.begin_scope_chirho();
        self.kind_var_cache_chirho.clear();
        match declaration_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                kind_sig_chirho,
                span_chirho,
                ..
            }
            | DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                kind_sig_chirho,
                span_chirho,
                ..
            } => {
                self.infer_data_decl_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    kind_sig_chirho.as_ref(),
                    *span_chirho,
                );
            }
            DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                span_chirho,
                ..
            } => {
                self.prepare_type_alias_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    *span_chirho,
                );
            }
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                type_vars_chirho,
                result_kind_chirho,
                span_chirho,
                ..
            } => {
                self.infer_type_family_decl_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    result_kind_chirho.as_ref(),
                    *span_chirho,
                    self.poly_kinds_enabled_chirho,
                );
            }
            DeclChirho::ClassDeclChirho {
                name_chirho,
                type_vars_chirho,
                associated_tfs_chirho,
                span_chirho,
                ..
            } => {
                self.infer_class_kind_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    *span_chirho,
                );
                for family_chirho in associated_tfs_chirho {
                    let mut parameter_kinds_chirho = Vec::new();
                    for parameter_chirho in &family_chirho.type_vars_chirho {
                        let kind_chirho = if let Some(kind_chirho) = self
                            .env_chirho
                            .lookup_chirho(parameter_chirho.text_chirho())
                        {
                            kind_chirho.clone()
                        } else {
                            let kind_chirho = self.fresh_kind_chirho();
                            self.env_chirho.bind_chirho(
                                parameter_chirho.text_chirho().to_owned(),
                                kind_chirho.clone(),
                            );
                            kind_chirho
                        };
                        parameter_kinds_chirho.push(kind_chirho);
                    }
                    let result_chirho = self.fresh_kind_chirho();
                    let kind_chirho =
                        KindChirho::arrow_n_chirho(parameter_kinds_chirho, result_chirho);
                    let existing_chirho = self
                        .env_chirho
                        .lookup_chirho(family_chirho.name_chirho.text_chirho())
                        .cloned()
                        .expect("associated family prebound");
                    self.unify_chirho(
                        &existing_chirho,
                        &kind_chirho,
                        "associated family",
                        family_chirho.span_chirho,
                    );
                    self.env_chirho.bind_chirho(
                        family_chirho.name_chirho.text_chirho().to_owned(),
                        kind_chirho,
                    );
                }
            }
            _ => unreachable!("only kind declarations enter dependency groups"),
        }
        let mut scope_chirho = PreparedKindScopeChirho {
            bindings_chirho: Vec::new(),
            names_chirho: std::mem::take(&mut self.kind_var_cache_chirho),
            written_chirho: std::mem::take(&mut self.pending_written_kinds_chirho),
        };
        for (name_chirho, binding_chirho) in self.env_chirho.capture_scope_chirho() {
            if declared_names_chirho.contains(&name_chirho.as_str()) {
                self.env_chirho
                    .bind_entry_chirho(name_chirho, binding_chirho);
            } else {
                scope_chirho
                    .bindings_chirho
                    .push((name_chirho, binding_chirho));
            }
        }
        scope_chirho
    }

    fn check_written_kind_group_chirho(
        &mut self,
        written_chirho: &[(KindVarChirho, super::SpanChirho)],
    ) {
        for &(variable_chirho, span_chirho) in written_chirho {
            let resolved_chirho = self
                .subst_chirho
                .apply_chirho(&KindChirho::VarChirho(variable_chirho));
            if !matches!(
                resolved_chirho,
                KindChirho::VarChirho(_) | KindChirho::RigidChirho(_)
            ) {
                let rigid_chirho = KindChirho::RigidChirho(self.fresh_var_chirho());
                self.unify_chirho(
                    &rigid_chirho,
                    &resolved_chirho,
                    "written kind variable",
                    span_chirho,
                );
            }
        }
        self.rigidify_kind_variables_chirho(
            written_chirho
                .iter()
                .map(|(variable_chirho, _)| *variable_chirho),
        );
    }

    fn check_kind_declaration_body_chirho(&mut self, declaration_chirho: &DeclChirho) {
        match declaration_chirho {
            DeclChirho::DataDeclChirho {
                type_vars_chirho,
                constructors_chirho,
                ..
            } => {
                self.check_data_constructors_chirho(type_vars_chirho, constructors_chirho);
            }
            DeclChirho::NewtypeDeclChirho {
                type_vars_chirho,
                constructor_chirho,
                ..
            } => {
                self.check_data_constructors_chirho(
                    type_vars_chirho,
                    std::slice::from_ref(constructor_chirho),
                );
            }
            DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                rhs_chirho,
                span_chirho,
            } => {
                self.check_alias_result_chirho(
                    name_chirho.text_chirho(),
                    type_vars_chirho.len(),
                    rhs_chirho,
                    *span_chirho,
                );
            }
            DeclChirho::ClassDeclChirho {
                context_chirho,
                methods_chirho,
                associated_tfs_chirho,
                ..
            } => {
                for family_chirho in associated_tfs_chirho {
                    if let Some(rhs_chirho) = &family_chirho.default_rhs_chirho {
                        self.check_alias_result_chirho(
                            family_chirho.name_chirho.text_chirho(),
                            family_chirho.type_vars_chirho.len(),
                            rhs_chirho,
                            family_chirho.span_chirho,
                        );
                    }
                }
                for constraint_chirho in context_chirho {
                    self.infer_constraint_kind_chirho(constraint_chirho);
                }
                for method_chirho in methods_chirho {
                    let kind_chirho = self.infer_type_kind_chirho(&method_chirho.ty_chirho);
                    self.unify_chirho(
                        &kind_chirho,
                        &KindChirho::StarChirho,
                        "class method type",
                        method_chirho.span_chirho,
                    );
                }
            }
            DeclChirho::TypeFamilyDeclChirho { .. } => {}
            _ => unreachable!("only kind declarations enter dependency groups"),
        }
    }

    fn check_alias_result_chirho(
        &mut self,
        name_chirho: &str,
        arity_chirho: usize,
        rhs_chirho: &super::TypeChirho,
        span_chirho: super::SpanChirho,
    ) {
        let mut result_chirho = self
            .env_chirho
            .lookup_chirho(name_chirho)
            .cloned()
            .expect("alias head prebound");
        for _ in 0..arity_chirho {
            if let KindChirho::ArrowChirho(_, tail_chirho) = result_chirho {
                result_chirho = *tail_chirho;
            }
        }
        let inferred_chirho = self.infer_type_kind_chirho(rhs_chirho);
        self.unify_chirho(
            &result_chirho,
            &inferred_chirho,
            "type alias result",
            span_chirho,
        );
    }
}
