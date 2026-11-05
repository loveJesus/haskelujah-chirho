// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Cold immutable family registration and coherence checks.
//! Workflow: language-features-chirho/stuck-family-equalities-chirho.md.

use super::InferCtxChirho;
use super::ast_types_chirho::{ast_type_to_syn_rhs_chirho, collect_free_type_vars_from_ast_chirho};
use super::closed_family_apartness_chirho::{
    FamilyHeadRoleChirho, family_branches_compatible_chirho,
};
use super::family_definitions_chirho::{
    TypeFamilyBranchChirho, TypeFamilyBranchIdChirho, TypeFamilyDefinitionChirho,
    TypeFamilyIdChirho, TypeFamilyReductionChirho, TypeFamilyRegistryChirho,
    is_type_family_key_chirho,
};
use super::family_scope_chirho::{TypeFamilyScopeChirho, TypeFamilyScopeResolutionChirho};
pub(super) use super::family_seeds_chirho::family_head_role_chirho;
use super::family_seeds_chirho::legacy_family_scope_chirho;
use super::family_transport_chirho::{
    family_rhs_supported_chirho, prepare_family_instance_syntax_chirho, prepare_pattern_chirho,
    stamp_family_references_chirho, unresolved_family_key_chirho,
};
use crate::ty_chirho::TyChirho;
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;
use std::collections::HashMap;
use std::sync::Arc;

const FAMILY_REGISTRATION_WORK_CHIRHO: usize = 100_000;

impl InferCtxChirho {
    pub(super) fn family_registration_error_chirho(
        &mut self,
        message_chirho: String,
        span_chirho: SpanChirho,
    ) {
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(209),
                message_chirho,
                span_chirho,
            ));
    }

    pub(super) fn prepare_family_scope_chirho(
        &mut self,
        module_chirho: &ModuleChirho,
        supplied_chirho: Option<Arc<TypeFamilyScopeChirho>>,
    ) {
        self.family_module_name_chirho = module_chirho.name_chirho.full_name_chirho();
        let mut scope_chirho = supplied_chirho
            .as_deref()
            .cloned()
            .unwrap_or_else(|| legacy_family_scope_chirho(&self.type_families_chirho));
        let mut declarations_chirho = Vec::new();
        for declaration_chirho in &module_chirho.decls_chirho {
            // GHC's class TyCons are algebraic: at nominal role their heads are
            // generative, including when a family returns a constraint constructor.
            // Synonyms do not provide that authority.
            let (name_chirho, generative_chirho, family_chirho) = match declaration_chirho {
                DeclChirho::DataDeclChirho { name_chirho, .. }
                | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
                | DeclChirho::ClassDeclChirho { name_chirho, .. } => (name_chirho, true, true),
                DeclChirho::TypeAliasDeclChirho { name_chirho, .. } => (name_chirho, false, true),
                DeclChirho::TypeFamilyDeclChirho {
                    name_chirho,
                    data_family_chirho,
                    ..
                } => (name_chirho, *data_family_chirho, *data_family_chirho),
                _ => continue,
            };
            // A written non-synonym declaration owns its bare type name. In
            // particular DATA family D1 must not expand the seeded Generic D1
            // alias while its instance patterns are normalized. Qualified
            // imported aliases remain in the synonym table under their keys.
            if !matches!(declaration_chirho, DeclChirho::TypeAliasDeclChirho { .. }) {
                self.type_synonyms_chirho.remove(name_chirho.text_chirho());
            }
            declarations_chirho.push((name_chirho.text_chirho().to_owned(), generative_chirho));
            let names_chirho = [
                name_chirho.text_chirho().to_owned(),
                format!(
                    "{}.{}",
                    self.family_module_name_chirho,
                    name_chirho.text_chirho()
                ),
            ];
            for written_chirho in names_chirho {
                let result_chirho = if family_chirho {
                    scope_chirho.bind_other_local_type_chirho(written_chirho)
                } else {
                    scope_chirho.bind_local_family_chirho(
                        written_chirho,
                        &TypeFamilyIdChirho::SourceChirho {
                            module_chirho: self.family_module_name_chirho.clone(),
                            name_chirho: name_chirho.text_chirho().to_owned(),
                        },
                    )
                };
                if let Err(error_chirho) = result_chirho {
                    self.family_registration_error_chirho(
                        error_chirho.to_string(),
                        name_chirho.span_chirho(),
                    );
                }
            }
            if let DeclChirho::ClassDeclChirho {
                associated_tfs_chirho,
                ..
            } = declaration_chirho
            {
                // Lowering may retain a written associated header and its default
                // as two records. They declare one non-generative type identity.
                // Do not collapse duplicate headers or duplicate defaults.
                let mut associated_shapes_chirho = HashMap::<(String, bool), (usize, usize)>::new();
                for associated_chirho in associated_tfs_chirho {
                    let counts_chirho = associated_shapes_chirho
                        .entry((
                            associated_chirho.name_chirho.full_name_chirho(),
                            associated_chirho.data_family_chirho,
                        ))
                        .or_default();
                    if associated_chirho.default_rhs_chirho.is_some() {
                        counts_chirho.1 += 1;
                    } else {
                        counts_chirho.0 += 1;
                    }
                }
                for associated_chirho in associated_tfs_chirho {
                    self.type_synonyms_chirho
                        .remove(associated_chirho.name_chirho.text_chirho());
                    if associated_chirho.default_rhs_chirho.is_none()
                        || associated_shapes_chirho.get(&(
                            associated_chirho.name_chirho.full_name_chirho(),
                            associated_chirho.data_family_chirho,
                        )) != Some(&(1, 1))
                    {
                        declarations_chirho.push((
                            associated_chirho.name_chirho.text_chirho().to_owned(),
                            associated_chirho.data_family_chirho,
                        ));
                    }
                    let id_chirho = TypeFamilyIdChirho::SourceChirho {
                        module_chirho: self.family_module_name_chirho.clone(),
                        name_chirho: associated_chirho.name_chirho.text_chirho().to_owned(),
                    };
                    for written_chirho in [
                        associated_chirho.name_chirho.full_name_chirho(),
                        format!(
                            "{}.{}",
                            self.family_module_name_chirho,
                            associated_chirho.name_chirho.text_chirho()
                        ),
                    ] {
                        let result_chirho = if associated_chirho.data_family_chirho {
                            scope_chirho.bind_other_local_type_chirho(written_chirho)
                        } else {
                            scope_chirho.bind_local_family_chirho(written_chirho, &id_chirho)
                        };
                        if let Err(error_chirho) = result_chirho {
                            self.family_registration_error_chirho(
                                error_chirho.to_string(),
                                associated_chirho.span_chirho,
                            );
                        }
                    }
                }
            }
        }
        if let Err(error_chirho) = self
            .type_families_chirho
            .constructor_origins_mut_chirho()
            .replace_source_module_chirho(&self.family_module_name_chirho, &declarations_chirho)
        {
            self.family_registration_error_chirho(
                error_chirho.to_string(),
                SpanChirho::DUMMY_CHIRHO,
            );
        }
        self.family_scope_chirho = Some(Arc::new(scope_chirho));
    }

    pub(super) fn family_reference_key_chirho(&self, name_chirho: &NameChirho) -> Option<String> {
        name_chirho
            .family_key_chirho()
            .map(str::to_owned)
            .or_else(|| self.family_name_key_chirho(&name_chirho.full_name_chirho()))
    }

    pub(super) fn family_name_key_chirho(&self, name_chirho: &str) -> Option<String> {
        family_name_key_in_scope_chirho(
            self.family_scope_chirho.as_deref(),
            &self.type_families_chirho,
            &self.family_module_name_chirho,
            name_chirho,
        )
    }

    pub(super) fn is_registered_family_chirho(&self, name_chirho: &str) -> bool {
        self.family_name_key_chirho(name_chirho)
            .is_some_and(|key_chirho| {
                self.type_families_chirho
                    .definition_chirho(&key_chirho)
                    .is_some()
            })
    }

    fn install_family_definition_chirho(
        &mut self,
        definition_chirho: TypeFamilyDefinitionChirho,
        span_chirho: SpanChirho,
    ) {
        let key_chirho = definition_chirho.id_chirho().key_chirho();
        if let Err(error_chirho) = self.type_families_chirho.insert_chirho(definition_chirho) {
            self.family_registration_error_chirho(error_chirho.to_string(), span_chirho);
            return;
        }
        self.verify_open_family_key_chirho(&key_chirho, span_chirho);
    }

    fn verify_open_family_key_chirho(&mut self, key_chirho: &str, span_chirho: SpanChirho) {
        let Some(definition_chirho) = self
            .type_families_chirho
            .definition_chirho(key_chirho)
            .cloned()
        else {
            return;
        };
        if definition_chirho.reduction_chirho() != TypeFamilyReductionChirho::OpenChirho
            || definition_chirho.open_coherence_verified_chirho()
        {
            return;
        }
        let verified_chirho = definition_chirho.verify_open_coherence_chirho(
            &|left_chirho, right_chirho| {
                family_branches_compatible_chirho(
                    &left_chirho.lhs_chirho,
                    &left_chirho.rhs_chirho,
                    &right_chirho.lhs_chirho,
                    &right_chirho.rhs_chirho,
                    &|name_chirho| family_head_role_chirho(&self.type_families_chirho, name_chirho),
                )
            },
            FAMILY_REGISTRATION_WORK_CHIRHO,
        );
        match verified_chirho {
            Ok(verified_chirho) => {
                if let Err(error_chirho) = self.type_families_chirho.insert_chirho(verified_chirho)
                {
                    self.family_registration_error_chirho(error_chirho.to_string(), span_chirho);
                }
            }
            Err(error_chirho) => {
                self.family_registration_error_chirho(error_chirho.to_string(), span_chirho)
            }
        }
    }

    /// A union of independently certified import snapshots is unverified until the
    /// combined open branch set has passed one cold check. Never check at a redex.
    pub(super) fn verify_imported_open_families_chirho(&mut self) {
        let mut keys_chirho = self
            .type_families_chirho
            .definitions_chirho()
            .filter(|definition_chirho| {
                definition_chirho.reduction_chirho() == TypeFamilyReductionChirho::OpenChirho
                    && !definition_chirho.open_coherence_verified_chirho()
            })
            .map(|definition_chirho| definition_chirho.id_chirho().key_chirho())
            .collect::<Vec<_>>();
        keys_chirho.sort();
        for key_chirho in keys_chirho {
            self.verify_open_family_key_chirho(&key_chirho, SpanChirho::DUMMY_CHIRHO);
        }
    }

    pub(super) fn register_open_family_chirho(
        &mut self,
        name_chirho: String,
        arity_chirho: usize,
        span_chirho: SpanChirho,
    ) {
        let arity_chirho = self
            .checked_family_arity_chirho(&name_chirho)
            .unwrap_or(arity_chirho);
        let id_chirho = TypeFamilyIdChirho::SourceChirho {
            module_chirho: self.family_module_name_chirho.clone(),
            name_chirho: name_chirho.clone(),
        };
        let mut scope_chirho = self
            .family_scope_chirho
            .as_deref()
            .cloned()
            .unwrap_or_else(|| legacy_family_scope_chirho(&self.type_families_chirho));
        for written_chirho in [
            name_chirho.clone(),
            format!("{}.{}", self.family_module_name_chirho, name_chirho),
        ] {
            if let Err(error_chirho) =
                scope_chirho.bind_local_family_chirho(written_chirho, &id_chirho)
            {
                self.family_registration_error_chirho(error_chirho.to_string(), span_chirho);
            }
        }
        self.family_scope_chirho = Some(Arc::new(scope_chirho));
        match TypeFamilyDefinitionChirho::new_chirho(
            id_chirho,
            arity_chirho,
            TypeFamilyReductionChirho::OpenChirho,
            Vec::new(),
        ) {
            Ok(definition_chirho) => {
                self.install_family_definition_chirho(definition_chirho, span_chirho)
            }
            Err(error_chirho) => {
                self.family_registration_error_chirho(error_chirho.to_string(), span_chirho)
            }
        }
    }

    pub fn register_type_family_chirho(
        &mut self,
        name_chirho: String,
        arity_chirho: usize,
        equations_chirho: Vec<(Vec<TyChirho>, TyChirho)>,
    ) {
        if equations_chirho.is_empty() {
            self.register_open_family_chirho(name_chirho, arity_chirho, SpanChirho::DUMMY_CHIRHO);
            return;
        }
        let id_chirho = TypeFamilyIdChirho::SourceChirho {
            module_chirho: self.family_module_name_chirho.clone(),
            name_chirho: name_chirho.clone(),
        };
        let mut scope_chirho = self
            .family_scope_chirho
            .as_deref()
            .cloned()
            .unwrap_or_else(|| legacy_family_scope_chirho(&self.type_families_chirho));
        if let Err(error_chirho) = scope_chirho.bind_local_family_chirho(name_chirho, &id_chirho) {
            self.family_registration_error_chirho(
                error_chirho.to_string(),
                SpanChirho::DUMMY_CHIRHO,
            );
        }
        self.family_scope_chirho = Some(Arc::new(scope_chirho));
        let predeclared_chirho = HashMap::from([(id_chirho.key_chirho(), arity_chirho)]);
        let mut branches_chirho = Vec::new();
        for (index_chirho, (lhs_chirho, rhs_chirho)) in equations_chirho.into_iter().enumerate() {
            let Ok(index_chirho) = u32::try_from(index_chirho) else {
                return;
            };
            let blockers_chirho = branches_chirho
                .iter()
                .enumerate()
                .filter_map(
                    |(prior_chirho, branch_chirho): (usize, &TypeFamilyBranchChirho)| {
                        (!family_branches_compatible_chirho(
                            &branch_chirho.lhs_chirho,
                            &branch_chirho.rhs_chirho,
                            &lhs_chirho,
                            &rhs_chirho,
                            &|name_chirho| {
                                family_head_with_predeclared_chirho(
                                    &self.type_families_chirho,
                                    &predeclared_chirho,
                                    name_chirho,
                                )
                            },
                        ))
                        .then_some(prior_chirho)
                    },
                )
                .collect();
            branches_chirho.push(TypeFamilyBranchChirho {
                id_chirho: TypeFamilyBranchIdChirho {
                    family_chirho: id_chirho.clone(),
                    producer_module_chirho: self.family_module_name_chirho.clone(),
                    declaration_start_chirho: 0,
                    equation_index_chirho: index_chirho,
                },
                binders_chirho: Vec::new(),
                lhs_chirho,
                rhs_chirho,
                blockers_chirho,
                held_chirho: false,
            });
        }
        match TypeFamilyDefinitionChirho::new_chirho(
            id_chirho,
            arity_chirho,
            TypeFamilyReductionChirho::ClosedChirho,
            branches_chirho,
        ) {
            Ok(definition_chirho) => {
                self.install_family_definition_chirho(definition_chirho, SpanChirho::DUMMY_CHIRHO)
            }
            Err(error_chirho) => self.family_registration_error_chirho(
                error_chirho.to_string(),
                SpanChirho::DUMMY_CHIRHO,
            ),
        }
    }

    pub fn register_type_family_instance_chirho(
        &mut self,
        name_chirho: String,
        lhs_chirho: Vec<TyChirho>,
        rhs_chirho: TyChirho,
    ) {
        self.register_family_instance_at_chirho(
            name_chirho,
            lhs_chirho,
            rhs_chirho,
            SpanChirho::DUMMY_CHIRHO,
            None,
        );
    }

    pub(super) fn register_family_instance_at_chirho(
        &mut self,
        name_chirho: String,
        lhs_chirho: Vec<TyChirho>,
        rhs_chirho: TyChirho,
        span_chirho: SpanChirho,
        ordinal_chirho: Option<u32>,
    ) {
        self.register_family_instance_row_chirho(
            name_chirho,
            lhs_chirho,
            rhs_chirho,
            span_chirho,
            ordinal_chirho,
            false,
        );
    }

    pub(super) fn register_held_family_instance_chirho(
        &mut self,
        name_chirho: String,
        span_chirho: SpanChirho,
        ordinal_chirho: Option<u32>,
    ) {
        self.register_family_instance_row_chirho(
            name_chirho,
            Vec::new(),
            TyChirho::ConChirho("%family:unresolved:held-rhs-chirho".to_owned()),
            span_chirho,
            ordinal_chirho,
            true,
        );
    }

    pub(super) fn register_family_instance_ast_chirho(
        &mut self,
        name_chirho: String,
        lhs_chirho: &[TypeChirho],
        rhs_chirho: &TypeChirho,
        span_chirho: SpanChirho,
        ordinal_chirho: Option<u32>,
    ) {
        let Some((mut lhs_chirho, mut rhs_chirho)) =
            prepare_family_instance_syntax_chirho(lhs_chirho, rhs_chirho)
        else {
            self.register_held_family_instance_chirho(name_chirho, span_chirho, ordinal_chirho);
            return;
        };
        if let Some(scope_chirho) = self.family_scope_chirho.as_deref() {
            for ty_chirho in lhs_chirho
                .iter_mut()
                .chain(std::iter::once(&mut rhs_chirho))
            {
                stamp_family_references_chirho(
                    ty_chirho,
                    scope_chirho,
                    &self.type_families_chirho,
                    &self.family_module_name_chirho,
                );
            }
        }
        if self.elaboration_state_chirho.owner_chirho.is_some() {
            if let Some((lhs_chirho, rhs_chirho, _)) =
                self.checked_family_terms_chirho(&lhs_chirho, &rhs_chirho)
            {
                self.register_family_instance_at_chirho(
                    name_chirho,
                    lhs_chirho,
                    rhs_chirho,
                    span_chirho,
                    ordinal_chirho,
                );
            }
            return;
        }
        let parameters_chirho = collect_free_type_vars_from_ast_chirho(&lhs_chirho);
        let lhs_chirho = lhs_chirho
            .iter()
            .map(|ty_chirho| {
                ast_type_to_syn_rhs_chirho(
                    ty_chirho,
                    &parameters_chirho,
                    self.promotion_scope_chirho.as_deref(),
                )
            })
            .collect();
        let rhs_chirho = ast_type_to_syn_rhs_chirho(
            &rhs_chirho,
            &parameters_chirho,
            self.promotion_scope_chirho.as_deref(),
        );
        self.register_family_instance_at_chirho(
            name_chirho,
            lhs_chirho,
            rhs_chirho,
            span_chirho,
            ordinal_chirho,
        );
    }

    fn register_family_instance_row_chirho(
        &mut self,
        name_chirho: String,
        lhs_chirho: Vec<TyChirho>,
        rhs_chirho: TyChirho,
        span_chirho: SpanChirho,
        ordinal_chirho: Option<u32>,
        held_chirho: bool,
    ) {
        let Some(key_chirho) = self.family_name_key_chirho(&name_chirho) else {
            self.family_registration_error_chirho(
                format!("type family `{name_chirho}` is not in scope"),
                span_chirho,
            );
            return;
        };
        let Some(definition_chirho) = self
            .type_families_chirho
            .definition_chirho(&key_chirho)
            .cloned()
        else {
            self.family_registration_error_chirho(
                format!("type family `{name_chirho}` has no definition"),
                span_chirho,
            );
            return;
        };
        if definition_chirho.reduction_chirho() != TypeFamilyReductionChirho::OpenChirho {
            self.family_registration_error_chirho(
                format!("cannot add an instance to closed or intrinsic family `{name_chirho}`"),
                span_chirho,
            );
            return;
        }
        // Associated rows and instantiated defaults arrive through the free AST converter.
        // Resolve their family references in this producer's scope before publishing them;
        // imported opaque keys already survive normalization unchanged.
        let lhs_chirho = lhs_chirho
            .iter()
            .map(|ty_chirho| {
                self.expand_type_synonyms_chirho(&self.normalize_imported_ty_chirho(ty_chirho))
            })
            .collect();
        let rhs_chirho =
            self.expand_type_synonyms_chirho(&self.normalize_imported_ty_chirho(&rhs_chirho));
        let ordinal_chirho = ordinal_chirho
            .or_else(|| u32::try_from(definition_chirho.branches_chirho().len()).ok())
            .unwrap_or(u32::MAX);
        let (lhs_chirho, rhs_chirho) = if held_chirho {
            let (lhs_chirho, rhs_chirho, _) =
                held_equation_inputs_chirho(definition_chirho.arity_chirho(), ordinal_chirho);
            (lhs_chirho, rhs_chirho)
        } else {
            (lhs_chirho, rhs_chirho)
        };
        let branch_chirho = TypeFamilyBranchChirho {
            id_chirho: TypeFamilyBranchIdChirho {
                family_chirho: definition_chirho.id_chirho().clone(),
                producer_module_chirho: self.family_module_name_chirho.clone(),
                declaration_start_chirho: span_chirho.start_chirho().as_u32_chirho(),
                equation_index_chirho: ordinal_chirho,
            },
            binders_chirho: Vec::new(),
            lhs_chirho,
            rhs_chirho,
            blockers_chirho: Vec::new(),
            held_chirho,
        };
        match TypeFamilyDefinitionChirho::new_chirho(
            definition_chirho.id_chirho().clone(),
            definition_chirho.arity_chirho(),
            TypeFamilyReductionChirho::OpenChirho,
            vec![branch_chirho],
        ) {
            Ok(incoming_chirho) => {
                self.install_family_definition_chirho(incoming_chirho, span_chirho)
            }
            Err(error_chirho) => {
                self.family_registration_error_chirho(error_chirho.to_string(), span_chirho)
            }
        }
    }

    pub(super) fn register_module_families_chirho(&mut self, module_chirho: &ModuleChirho) {
        let predeclared_chirho = predeclared_family_arities_chirho(
            module_chirho,
            &self.family_module_name_chirho,
            &self.type_families_chirho,
        );
        for declaration_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::ClassDeclChirho {
                associated_tfs_chirho,
                ..
            } = declaration_chirho
            {
                for associated_chirho in associated_tfs_chirho
                    .iter()
                    .filter(|family_chirho| !family_chirho.data_family_chirho)
                {
                    self.register_open_family_chirho(
                        associated_chirho.name_chirho.text_chirho().to_owned(),
                        associated_chirho.type_vars_chirho.len(),
                        associated_chirho.span_chirho,
                    );
                }
            }
            let DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                type_vars_chirho,
                equations_chirho,
                closed_chirho,
                data_family_chirho,
                span_chirho,
                ..
            } = declaration_chirho
            else {
                continue;
            };
            if *data_family_chirho {
                continue;
            }
            let arity_chirho = type_vars_chirho
                .iter()
                .filter(|variable_chirho| variable_chirho.is_visible_chirho())
                .count();
            let arity_chirho = self
                .checked_family_arity_chirho(&name_chirho.full_name_chirho())
                .unwrap_or(arity_chirho);
            if !closed_chirho {
                self.register_open_family_chirho(
                    name_chirho.text_chirho().to_owned(),
                    arity_chirho,
                    *span_chirho,
                );
                continue;
            }
            let id_chirho = TypeFamilyIdChirho::SourceChirho {
                module_chirho: self.family_module_name_chirho.clone(),
                name_chirho: name_chirho.text_chirho().to_owned(),
            };
            let mut branches_chirho = Vec::new();
            let mut work_chirho = FAMILY_REGISTRATION_WORK_CHIRHO;
            let mut held_boundary_chirho = false;
            for (index_chirho, equation_chirho) in equations_chirho.iter().enumerate() {
                let Ok(index_chirho) = u32::try_from(index_chirho) else {
                    break;
                };
                let mut held_chirho = held_boundary_chirho || equation_chirho.is_held_chirho();
                if !held_chirho {
                    held_chirho = !equation_chirho
                        .kind_args_chirho
                        .iter()
                        .chain(&equation_chirho.lhs_types_chirho)
                        .all(|ty_chirho| {
                            family_rhs_supported_chirho(ty_chirho, true, &mut work_chirho, 0)
                        })
                        || !family_rhs_supported_chirho(
                            &equation_chirho.rhs_chirho,
                            false,
                            &mut work_chirho,
                            0,
                        );
                }
                let (mut lhs_chirho, rhs_chirho, parameters_chirho) = if held_chirho {
                    held_equation_inputs_chirho(arity_chirho, index_chirho)
                } else {
                    let mut lhs_ast_chirho = equation_chirho
                        .kind_args_chirho
                        .iter()
                        .chain(&equation_chirho.lhs_types_chirho)
                        .cloned()
                        .collect::<Vec<_>>();
                    let mut rhs_ast_chirho = equation_chirho.rhs_chirho.clone();
                    let mut wildcard_chirho = 0;
                    for ty_chirho in &mut lhs_ast_chirho {
                        held_chirho |= !prepare_pattern_chirho(
                            ty_chirho,
                            index_chirho,
                            &mut wildcard_chirho,
                            &mut work_chirho,
                            0,
                        );
                        if let Some(scope_chirho) = self.family_scope_chirho.as_deref() {
                            stamp_family_references_chirho(
                                ty_chirho,
                                scope_chirho,
                                &self.type_families_chirho,
                                &self.family_module_name_chirho,
                            );
                        }
                    }
                    if let Some(scope_chirho) = self.family_scope_chirho.as_deref() {
                        stamp_family_references_chirho(
                            &mut rhs_ast_chirho,
                            scope_chirho,
                            &self.type_families_chirho,
                            &self.family_module_name_chirho,
                        );
                    }
                    if held_chirho {
                        held_equation_inputs_chirho(arity_chirho, index_chirho)
                    } else {
                        let parameters_chirho =
                            collect_free_type_vars_from_ast_chirho(&lhs_ast_chirho);
                        if self.elaboration_state_chirho.owner_chirho.is_some() {
                            match self.checked_family_terms_chirho(&lhs_ast_chirho, &rhs_ast_chirho)
                            {
                                Some(terms_chirho) => terms_chirho,
                                None => {
                                    held_chirho = true;
                                    held_equation_inputs_chirho(arity_chirho, index_chirho)
                                }
                            }
                        } else {
                            let lhs_chirho = lhs_ast_chirho
                                .iter()
                                .map(|ty_chirho| {
                                    self.expand_type_synonyms_chirho(&ast_type_to_syn_rhs_chirho(
                                        ty_chirho,
                                        &parameters_chirho,
                                        self.promotion_scope_chirho.as_deref(),
                                    ))
                                })
                                .collect();
                            let rhs_chirho =
                                self.expand_type_synonyms_chirho(&ast_type_to_syn_rhs_chirho(
                                    &rhs_ast_chirho,
                                    &parameters_chirho,
                                    self.promotion_scope_chirho.as_deref(),
                                ));
                            (lhs_chirho, rhs_chirho, parameters_chirho)
                        }
                    }
                };
                if lhs_chirho.len() != arity_chirho {
                    held_chirho = true;
                    lhs_chirho = (0..arity_chirho)
                        .map(|position_chirho| {
                            TyChirho::ForallVarChirho(format!(
                                "%held:{index_chirho}:{position_chirho}"
                            ))
                        })
                        .collect();
                }
                let mut blockers_chirho = Vec::new();
                for (prior_chirho, branch_chirho) in branches_chirho.iter().enumerate() {
                    if work_chirho == 0 {
                        held_chirho = true;
                        break;
                    }
                    work_chirho -= 1;
                    let branch_chirho: &TypeFamilyBranchChirho = branch_chirho;
                    if branch_chirho.held_chirho
                        || !family_branches_compatible_chirho(
                            &branch_chirho.lhs_chirho,
                            &branch_chirho.rhs_chirho,
                            &lhs_chirho,
                            &rhs_chirho,
                            &|name_chirho| {
                                family_head_with_predeclared_chirho(
                                    &self.type_families_chirho,
                                    &predeclared_chirho,
                                    name_chirho,
                                )
                            },
                        )
                    {
                        blockers_chirho.push(prior_chirho);
                    }
                }
                held_boundary_chirho |= held_chirho;
                branches_chirho.push(TypeFamilyBranchChirho {
                    id_chirho: TypeFamilyBranchIdChirho {
                        family_chirho: id_chirho.clone(),
                        producer_module_chirho: self.family_module_name_chirho.clone(),
                        declaration_start_chirho: span_chirho.start_chirho().as_u32_chirho(),
                        equation_index_chirho: index_chirho,
                    },
                    binders_chirho: parameters_chirho,
                    lhs_chirho,
                    rhs_chirho,
                    blockers_chirho,
                    held_chirho,
                });
            }
            match TypeFamilyDefinitionChirho::new_chirho(
                id_chirho,
                arity_chirho,
                TypeFamilyReductionChirho::ClosedChirho,
                branches_chirho,
            ) {
                Ok(definition_chirho) => {
                    self.install_family_definition_chirho(definition_chirho, *span_chirho)
                }
                Err(error_chirho) => {
                    self.family_registration_error_chirho(error_chirho.to_string(), *span_chirho)
                }
            }
        }
        for declaration_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::TypeFamilyInstanceDeclChirho {
                family_name_chirho,
                kind_args_chirho,
                lhs_types_chirho,
                rhs_chirho,
                span_chirho,
            } = declaration_chirho
            {
                self.register_family_instance_ast_chirho(
                    family_name_chirho.full_name_chirho(),
                    &kind_args_chirho
                        .iter()
                        .chain(lhs_types_chirho)
                        .cloned()
                        .collect::<Vec<_>>(),
                    rhs_chirho,
                    *span_chirho,
                    Some(0),
                );
            }
        }
    }
}

fn held_equation_inputs_chirho(
    arity_chirho: usize,
    equation_chirho: u32,
) -> (Vec<TyChirho>, TyChirho, Vec<String>) {
    (
        (0..arity_chirho)
            .map(|position_chirho| {
                TyChirho::ForallVarChirho(format!(
                    "%held:{equation_chirho}:{position_chirho}:chirho"
                ))
            })
            .collect(),
        TyChirho::ConChirho("%family:unresolved:held-rhs-chirho".to_owned()),
        Vec::new(),
    )
}

/// Predeclared exact family keys are opaque before their immutable bodies are installed.
fn family_head_with_predeclared_chirho(
    registry_chirho: &TypeFamilyRegistryChirho,
    predeclared_chirho: &HashMap<String, usize>,
    name_chirho: &str,
) -> FamilyHeadRoleChirho {
    if let Some(arity_chirho) = predeclared_chirho.get(name_chirho) {
        if registry_chirho
            .definition_chirho(name_chirho)
            .is_some_and(|definition_chirho| definition_chirho.arity_chirho() != *arity_chirho)
        {
            return FamilyHeadRoleChirho::UnknownChirho;
        }
        return FamilyHeadRoleChirho::FamilyChirho {
            identity_chirho: name_chirho.to_owned(),
            arity_chirho: *arity_chirho,
        };
    }
    family_head_role_chirho(registry_chirho, name_chirho)
}

fn predeclared_family_arities_chirho(
    module_chirho: &ModuleChirho,
    module_name_chirho: &str,
    registry_chirho: &TypeFamilyRegistryChirho,
) -> HashMap<String, usize> {
    let mut arities_chirho = HashMap::new();
    for declaration_chirho in &module_chirho.decls_chirho {
        match declaration_chirho {
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                type_vars_chirho,
                data_family_chirho: false,
                ..
            } => {
                let id_chirho = TypeFamilyIdChirho::SourceChirho {
                    module_chirho: module_name_chirho.to_owned(),
                    name_chirho: name_chirho.text_chirho().to_owned(),
                };
                arities_chirho.insert(
                    id_chirho.key_chirho(),
                    type_vars_chirho
                        .iter()
                        .filter(|variable_chirho| variable_chirho.is_visible_chirho())
                        .count(),
                );
            }
            DeclChirho::ClassDeclChirho {
                associated_tfs_chirho,
                ..
            } => {
                for family_chirho in associated_tfs_chirho
                    .iter()
                    .filter(|family_chirho| !family_chirho.data_family_chirho)
                {
                    let id_chirho = TypeFamilyIdChirho::SourceChirho {
                        module_chirho: module_name_chirho.to_owned(),
                        name_chirho: family_chirho.name_chirho.text_chirho().to_owned(),
                    };
                    arities_chirho
                        .insert(id_chirho.key_chirho(), family_chirho.type_vars_chirho.len());
                }
            }
            _ => {}
        }
    }
    for (key_chirho, arity_chirho) in &mut arities_chirho {
        if let Some(scheme_chirho) = registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(key_chirho)
        {
            *arity_chirho = scheme_chirho.reduction_arity_chirho;
        }
    }
    arities_chirho
}

pub(super) fn family_name_key_in_scope_chirho(
    family_scope_chirho: Option<&TypeFamilyScopeChirho>,
    catalogue_chirho: &crate::infer_chirho::TypeFamilyEnvChirho,
    module_chirho: &str,
    name_chirho: &str,
) -> Option<String> {
    if is_type_family_key_chirho(name_chirho) {
        return Some(name_chirho.to_owned());
    }
    let legacy_chirho;
    let scope_chirho = match family_scope_chirho {
        Some(scope_chirho) => scope_chirho,
        None => {
            legacy_chirho = legacy_family_scope_chirho(catalogue_chirho);
            &legacy_chirho
        }
    };
    match scope_chirho.resolve_chirho(name_chirho, catalogue_chirho) {
        TypeFamilyScopeResolutionChirho::UniqueChirho(key_chirho) => Some(key_chirho.to_owned()),
        TypeFamilyScopeResolutionChirho::AmbiguousChirho(_) => {
            Some(unresolved_family_key_chirho(module_chirho, name_chirho))
        }
        TypeFamilyScopeResolutionChirho::MissingChirho => None,
    }
}
