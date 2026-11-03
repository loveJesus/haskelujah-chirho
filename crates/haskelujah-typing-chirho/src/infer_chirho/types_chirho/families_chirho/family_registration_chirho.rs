// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Cold family registration, source identity transport and authoritative comparison heads.
//! Workflow: language-features-chirho/stuck-family-equalities-chirho.md.

use super::InferCtxChirho;
use super::ast_types_chirho::{ast_type_to_syn_rhs_chirho, collect_free_type_vars_from_ast_chirho};
use super::closed_family_apartness_chirho::{
    FamilyHeadRoleChirho, GenerativeIdentityChirho, family_branches_compatible_chirho,
};
use super::family_definitions_chirho::{
    IntrinsicTypeFamilyChirho, TypeFamilyBranchChirho, TypeFamilyBranchIdChirho,
    TypeFamilyDefinitionChirho, TypeFamilyIdChirho, TypeFamilyReductionChirho,
    TypeFamilyRegistryChirho, is_type_family_key_chirho,
};
use super::family_scope_chirho::{TypeFamilyScopeChirho, TypeFamilyScopeResolutionChirho};
use crate::ty_chirho::TyChirho;
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::promotion_chirho::{
    is_promoted_key_chirho, is_unknown_promoted_key_chirho,
};
use haskelujah_ast_chirho::ty_chirho::{MultiplicityChirho, TypeChirho};
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;
use std::sync::Arc;

const FAMILY_REGISTRATION_WORK_CHIRHO: usize = 100_000;

fn intrinsic_seeds_chirho() -> Vec<(&'static str, IntrinsicTypeFamilyChirho, Vec<&'static str>)> {
    use IntrinsicTypeFamilyChirho as IChirho;
    let numeric_modules_chirho = vec!["GHC.TypeLits", "GHC.TypeNats"];
    vec![
        ("+", IChirho::AddNatChirho, numeric_modules_chirho.clone()),
        (
            "*",
            IChirho::MultiplyNatChirho,
            numeric_modules_chirho.clone(),
        ),
        ("^", IChirho::PowerNatChirho, numeric_modules_chirho.clone()),
        (
            "-",
            IChirho::SubtractNatChirho,
            numeric_modules_chirho.clone(),
        ),
        (
            "Div",
            IChirho::DivideNatChirho,
            numeric_modules_chirho.clone(),
        ),
        (
            "Mod",
            IChirho::ModuloNatChirho,
            numeric_modules_chirho.clone(),
        ),
        (
            "Log2",
            IChirho::Log2NatChirho,
            numeric_modules_chirho.clone(),
        ),
        ("<=?", IChirho::LessEqualNatChirho, numeric_modules_chirho),
        (
            "AppendSymbol",
            IChirho::AppendSymbolChirho,
            vec!["GHC.TypeLits"],
        ),
        ("CharToNat", IChirho::CharToNatChirho, vec!["GHC.TypeLits"]),
        ("NatToChar", IChirho::NatToCharChirho, vec!["GHC.TypeLits"]),
        ("If", IChirho::IfChirho, vec!["Data.Type.Bool"]),
        ("Not", IChirho::NotChirho, vec!["Data.Type.Bool"]),
        ("&&", IChirho::AndChirho, vec!["Data.Type.Bool"]),
        ("||", IChirho::OrChirho, vec!["Data.Type.Bool"]),
    ]
}

/// Only the actual compiler seed producer creates these identities and export aliases.
pub fn seed_type_family_registry_chirho() -> TypeFamilyRegistryChirho {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    for (name_chirho, intrinsic_chirho, modules_chirho) in intrinsic_seeds_chirho() {
        let id_chirho = TypeFamilyIdChirho::IntrinsicChirho(intrinsic_chirho);
        registry_chirho
            .insert_chirho(
                TypeFamilyDefinitionChirho::new_chirho(
                    id_chirho.clone(),
                    intrinsic_chirho.arity_chirho(),
                    TypeFamilyReductionChirho::IntrinsicChirho(intrinsic_chirho),
                    Vec::new(),
                )
                .expect("static intrinsic seed is consistent"),
            )
            .expect("static intrinsic identities are unique");
        for module_chirho in modules_chirho {
            registry_chirho
                .export_alias_chirho(format!("{module_chirho}.{name_chirho}"), &id_chirho)
                .expect("intrinsic seed exists");
        }
    }
    for name_chirho in [
        "Int",
        "Integer",
        "Double",
        "Float",
        "Bool",
        "Char",
        "Ordering",
        "Maybe",
        "Either",
        "IO",
        "ST",
        "RealWorld",
        "Word",
        "Word8",
        "Word16",
        "Word32",
        "Word64",
        "Int8",
        "Int16",
        "Int32",
        "Int64",
        "Natural",
        "Proxy",
        "True",
        "False",
        ":",
        "'[]",
        "[]",
        "()",
    ] {
        registry_chirho
            .constructor_origins_mut_chirho()
            .insert_compiler_chirho(
                name_chirho.to_owned(),
                format!("compiler-standard:{name_chirho}"),
            );
    }
    let id_chirho = TypeFamilyIdChirho::CompilerSeedChirho {
        module_chirho: "Control.Monad.Primitive".to_owned(),
        name_chirho: "PrimState".to_owned(),
    };
    let branch_chirho = |index_chirho, lhs_chirho, rhs_chirho| TypeFamilyBranchChirho {
        id_chirho: TypeFamilyBranchIdChirho {
            family_chirho: id_chirho.clone(),
            producer_module_chirho: "Control.Monad.Primitive".to_owned(),
            declaration_start_chirho: 0,
            equation_index_chirho: index_chirho,
        },
        binders_chirho: vec!["sChirho".to_owned()],
        lhs_chirho: vec![lhs_chirho],
        rhs_chirho,
        blockers_chirho: Vec::new(),
        held_chirho: false,
    };
    let s_chirho = TyChirho::ForallVarChirho("sChirho".to_owned());
    let definition_chirho = TypeFamilyDefinitionChirho::new_chirho(
        id_chirho.clone(),
        1,
        TypeFamilyReductionChirho::OpenChirho,
        vec![
            branch_chirho(
                0,
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ST".to_owned())),
                    Box::new(s_chirho.clone()),
                ),
                s_chirho,
            ),
            branch_chirho(
                1,
                TyChirho::ConChirho("IO".to_owned()),
                TyChirho::ConChirho("RealWorld".to_owned()),
            ),
        ],
    )
    .expect("static PrimState seed is well formed");
    let definition_chirho = definition_chirho
        .verify_open_coherence_chirho(
            &|left_chirho, right_chirho| {
                family_branches_compatible_chirho(
                    &left_chirho.lhs_chirho,
                    &left_chirho.rhs_chirho,
                    &right_chirho.lhs_chirho,
                    &right_chirho.rhs_chirho,
                    &|name_chirho| family_head_role_chirho(&registry_chirho, name_chirho),
                )
            },
            FAMILY_REGISTRATION_WORK_CHIRHO,
        )
        .expect("static PrimState rows are apart");
    registry_chirho
        .insert_chirho(definition_chirho)
        .expect("static PrimState seed is unique");
    registry_chirho
        .export_alias_chirho("Control.Monad.Primitive.PrimState".to_owned(), &id_chirho)
        .expect("PrimState seed exists");
    // These real compiler families have no evaluator here. Their sealed empty
    // definitions still prevent treating a stuck application as an injective tycon.
    for (name_chirho, modules_chirho) in [
        ("CmpNat", vec!["GHC.TypeLits", "GHC.TypeNats"]),
        ("CmpSymbol", vec!["GHC.TypeLits"]),
    ] {
        let id_chirho = TypeFamilyIdChirho::CompilerSeedChirho {
            module_chirho: "GHC.TypeLits".to_owned(),
            name_chirho: name_chirho.to_owned(),
        };
        registry_chirho
            .insert_chirho(
                TypeFamilyDefinitionChirho::new_chirho(
                    id_chirho.clone(),
                    2,
                    TypeFamilyReductionChirho::ClosedChirho,
                    Vec::new(),
                )
                .expect("sealed compiler family seed is consistent"),
            )
            .expect("sealed compiler family seed is unique");
        for module_chirho in modules_chirho {
            registry_chirho
                .export_alias_chirho(format!("{module_chirho}.{name_chirho}"), &id_chirho)
                .expect("sealed compiler family exists");
        }
    }
    registry_chirho
}

fn legacy_family_scope_chirho(registry_chirho: &TypeFamilyRegistryChirho) -> TypeFamilyScopeChirho {
    let mut scope_chirho = TypeFamilyScopeChirho::new_chirho();
    for (name_chirho, _intrinsic_chirho, modules_chirho) in intrinsic_seeds_chirho() {
        for module_chirho in modules_chirho {
            let exported_chirho = format!("{module_chirho}.{name_chirho}");
            scope_chirho.import_export_chirho(
                name_chirho.to_owned(),
                &exported_chirho,
                registry_chirho,
            );
            scope_chirho.import_export_chirho(
                exported_chirho.clone(),
                &exported_chirho,
                registry_chirho,
            );
        }
    }
    for (name_chirho, export_chirho) in [
        ("PrimState", "Control.Monad.Primitive.PrimState"),
        ("CmpNat", "GHC.TypeLits.CmpNat"),
        ("CmpSymbol", "GHC.TypeLits.CmpSymbol"),
    ] {
        scope_chirho.import_export_chirho(name_chirho.to_owned(), export_chirho, registry_chirho);
        scope_chirho.import_export_chirho(export_chirho.to_owned(), export_chirho, registry_chirho);
    }
    scope_chirho
}

pub(super) fn family_head_role_chirho(
    registry_chirho: &TypeFamilyRegistryChirho,
    name_chirho: &str,
) -> FamilyHeadRoleChirho {
    if let Some(definition_chirho) = registry_chirho.definition_chirho(name_chirho) {
        return FamilyHeadRoleChirho::FamilyChirho {
            identity_chirho: definition_chirho.id_chirho().key_chirho(),
            arity_chirho: definition_chirho.arity_chirho(),
        };
    }
    if is_type_family_key_chirho(name_chirho) || is_unknown_promoted_key_chirho(name_chirho) {
        return FamilyHeadRoleChirho::UnknownChirho;
    }
    if is_promoted_key_chirho(name_chirho) {
        return FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho(name_chirho.to_owned()),
        };
    }
    if let Some(identity_chirho) = registry_chirho
        .constructor_origins_chirho()
        .identity_chirho(name_chirho)
    {
        return FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho(identity_chirho),
        };
    }
    if name_chirho.parse::<u128>().is_ok()
        || (name_chirho.starts_with('"') && name_chirho.ends_with('"'))
        || (name_chirho.starts_with('\'') && name_chirho.ends_with('\'') && name_chirho.len() >= 3)
    {
        return FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho(format!(
                "literal:{name_chirho}"
            )),
        };
    }
    FamilyHeadRoleChirho::UnknownChirho
}

fn unresolved_family_key_chirho(module_chirho: &str, name_chirho: &str) -> String {
    format!(
        "%family:unresolved:{}:{module_chirho}:{}:{name_chirho}",
        module_chirho.len(),
        name_chirho.len()
    )
}

/// Preserve exporter's stamps, including unresolved stamps. Never resolve one in its importer.
pub fn stamp_family_references_chirho(
    ty_chirho: &mut TypeChirho,
    scope_chirho: &TypeFamilyScopeChirho,
    registry_chirho: &TypeFamilyRegistryChirho,
    module_name_chirho: &str,
) {
    fn walk_chirho(
        ty_chirho: &mut TypeChirho,
        scope_chirho: &TypeFamilyScopeChirho,
        registry_chirho: &TypeFamilyRegistryChirho,
        module_chirho: &str,
        budget_chirho: &mut usize,
        depth_chirho: usize,
    ) -> bool {
        if depth_chirho > 128 || *budget_chirho == 0 {
            return false;
        }
        *budget_chirho -= 1;
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) if name_chirho.family_key_chirho().is_none() => {
                let written_chirho = name_chirho.full_name_chirho();
                match scope_chirho.resolve_chirho(&written_chirho, registry_chirho) {
                    TypeFamilyScopeResolutionChirho::UniqueChirho(key_chirho) => {
                        name_chirho.set_family_key_chirho(key_chirho.to_owned())
                    }
                    TypeFamilyScopeResolutionChirho::AmbiguousChirho(_) => name_chirho
                        .set_family_key_chirho(unresolved_family_key_chirho(
                            module_chirho,
                            &written_chirho,
                        )),
                    TypeFamilyScopeResolutionChirho::MissingChirho => {}
                }
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            }
            | TypeChirho::FunChirho {
                arg_chirho: fun_chirho,
                result_chirho: arg_chirho,
                ..
            } => {
                return walk_chirho(
                    fun_chirho,
                    scope_chirho,
                    registry_chirho,
                    module_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                ) && walk_chirho(
                    arg_chirho,
                    scope_chirho,
                    registry_chirho,
                    module_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                );
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                return elements_chirho.iter_mut().all(|element_chirho| {
                    walk_chirho(
                        element_chirho,
                        scope_chirho,
                        registry_chirho,
                        module_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )
                });
            }
            TypeChirho::ListChirho { element_chirho, .. }
            | TypeChirho::ParenChirho {
                inner_chirho: element_chirho,
                ..
            }
            | TypeChirho::QualChirho {
                body_chirho: element_chirho,
                ..
            }
            | TypeChirho::ForallChirho {
                body_chirho: element_chirho,
                ..
            }
            | TypeChirho::RequiredForallChirho {
                body_chirho: element_chirho,
                ..
            } => {
                return walk_chirho(
                    element_chirho,
                    scope_chirho,
                    registry_chirho,
                    module_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                );
            }
            _ => {}
        }
        true
    }
    let mut budget_chirho = FAMILY_REGISTRATION_WORK_CHIRHO;
    if !walk_chirho(
        ty_chirho,
        scope_chirho,
        registry_chirho,
        module_name_chirho,
        &mut budget_chirho,
        0,
    ) {
        let span_chirho = ty_chirho.span_chirho();
        let key_chirho = unresolved_family_key_chirho(module_name_chirho, "resource-bound");
        let mut name_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            key_chirho.clone(),
            span_chirho,
        ));
        name_chirho.set_family_key_chirho(key_chirho);
        *ty_chirho = TypeChirho::ConChirho(name_chirho);
    }
}

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
            let (name_chirho, generative_chirho, family_chirho) = match declaration_chirho {
                DeclChirho::DataDeclChirho { name_chirho, .. }
                | DeclChirho::NewtypeDeclChirho { name_chirho, .. } => (name_chirho, true, true),
                DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
                | DeclChirho::ClassDeclChirho { name_chirho, .. } => (name_chirho, false, true),
                DeclChirho::TypeFamilyDeclChirho { name_chirho, .. } => (name_chirho, false, false),
                _ => continue,
            };
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
                for associated_chirho in associated_tfs_chirho {
                    declarations_chirho.push((
                        associated_chirho.name_chirho.text_chirho().to_owned(),
                        false,
                    ));
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
                        if let Err(error_chirho) =
                            scope_chirho.bind_local_family_chirho(written_chirho, &id_chirho)
                        {
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
        if is_type_family_key_chirho(name_chirho) {
            return Some(name_chirho.to_owned());
        }
        let legacy_chirho;
        let scope_chirho = match self.family_scope_chirho.as_deref() {
            Some(scope_chirho) => scope_chirho,
            None => {
                legacy_chirho = legacy_family_scope_chirho(&self.type_families_chirho);
                &legacy_chirho
            }
        };
        match scope_chirho.resolve_chirho(name_chirho, &self.type_families_chirho) {
            TypeFamilyScopeResolutionChirho::UniqueChirho(key_chirho) => {
                Some(key_chirho.to_owned())
            }
            TypeFamilyScopeResolutionChirho::AmbiguousChirho(_) => Some(
                unresolved_family_key_chirho(&self.family_module_name_chirho, name_chirho),
            ),
            TypeFamilyScopeResolutionChirho::MissingChirho => None,
        }
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
        let id_chirho = definition_chirho.id_chirho().key_chirho();
        if let Err(error_chirho) = self.type_families_chirho.insert_chirho(definition_chirho) {
            self.family_registration_error_chirho(error_chirho.to_string(), span_chirho);
            return;
        }
        let Some(definition_chirho) = self
            .type_families_chirho
            .definition_chirho(&id_chirho)
            .cloned()
        else {
            return;
        };
        if definition_chirho.reduction_chirho() == TypeFamilyReductionChirho::OpenChirho {
            match definition_chirho.verify_open_coherence_chirho(
                &|left_chirho, right_chirho| {
                    family_branches_compatible_chirho(
                        &left_chirho.lhs_chirho,
                        &left_chirho.rhs_chirho,
                        &right_chirho.lhs_chirho,
                        &right_chirho.rhs_chirho,
                        &|name_chirho| {
                            family_head_role_chirho(&self.type_families_chirho, name_chirho)
                        },
                    )
                },
                FAMILY_REGISTRATION_WORK_CHIRHO,
            ) {
                Ok(verified_chirho) => {
                    if let Err(error_chirho) =
                        self.type_families_chirho.insert_chirho(verified_chirho)
                    {
                        self.family_registration_error_chirho(
                            error_chirho.to_string(),
                            span_chirho,
                        );
                    }
                }
                Err(error_chirho) => {
                    self.family_registration_error_chirho(error_chirho.to_string(), span_chirho)
                }
            }
        }
    }

    pub(super) fn register_open_family_chirho(
        &mut self,
        name_chirho: String,
        arity_chirho: usize,
        span_chirho: SpanChirho,
    ) {
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
                                family_head_role_chirho(&self.type_families_chirho, name_chirho)
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
            held_chirho: false,
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
        for declaration_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::ClassDeclChirho {
                associated_tfs_chirho,
                ..
            } = declaration_chirho
            {
                for associated_chirho in associated_tfs_chirho {
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
                span_chirho,
                ..
            } = declaration_chirho
            else {
                continue;
            };
            let arity_chirho = type_vars_chirho
                .iter()
                .filter(|variable_chirho| variable_chirho.is_visible_chirho())
                .count();
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
                    held_chirho = !equation_chirho.lhs_types_chirho.iter().all(|ty_chirho| {
                        family_rhs_supported_chirho(ty_chirho, true, &mut work_chirho, 0)
                    }) || !family_rhs_supported_chirho(
                        &equation_chirho.rhs_chirho,
                        false,
                        &mut work_chirho,
                        0,
                    );
                }
                let (mut lhs_chirho, rhs_chirho, parameters_chirho) = if held_chirho {
                    held_equation_inputs_chirho(arity_chirho, index_chirho)
                } else {
                    let mut lhs_ast_chirho = equation_chirho.lhs_types_chirho.clone();
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
                                family_head_role_chirho(&self.type_families_chirho, name_chirho)
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
                lhs_types_chirho,
                rhs_chirho,
                span_chirho,
                ..
            } = declaration_chirho
            {
                let mut lhs_ast_chirho = lhs_types_chirho.clone();
                let mut rhs_ast_chirho = rhs_chirho.clone();
                let mut wildcard_chirho = 0;
                let mut work_chirho = FAMILY_REGISTRATION_WORK_CHIRHO;
                let mut valid_chirho = true;
                for ty_chirho in &mut lhs_ast_chirho {
                    valid_chirho &= prepare_pattern_chirho(
                        ty_chirho,
                        0,
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
                if !valid_chirho {
                    self.family_registration_error_chirho(
                        "unsupported open family instance pattern".to_owned(),
                        *span_chirho,
                    );
                    continue;
                }
                if let Some(scope_chirho) = self.family_scope_chirho.as_deref() {
                    stamp_family_references_chirho(
                        &mut rhs_ast_chirho,
                        scope_chirho,
                        &self.type_families_chirho,
                        &self.family_module_name_chirho,
                    );
                }
                let parameters_chirho = collect_free_type_vars_from_ast_chirho(&lhs_ast_chirho);
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
                let rhs_chirho = self.expand_type_synonyms_chirho(&ast_type_to_syn_rhs_chirho(
                    &rhs_ast_chirho,
                    &parameters_chirho,
                    self.promotion_scope_chirho.as_deref(),
                ));
                self.register_family_instance_at_chirho(
                    family_name_chirho.full_name_chirho(),
                    lhs_chirho,
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

/// Reject shapes the internal type loses, before cloning or converting their bodies.
fn family_rhs_supported_chirho(
    ty_chirho: &TypeChirho,
    pattern_chirho: bool,
    work_chirho: &mut usize,
    depth_chirho: usize,
) -> bool {
    if depth_chirho > 128 || *work_chirho == 0 {
        return false;
    }
    *work_chirho -= 1;
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho)
        | TypeChirho::ConChirho(name_chirho)
        | TypeChirho::PromotedConChirho { name_chirho, .. } => {
            let bytes_chirho = name_chirho
                .text_chirho()
                .len()
                .saturating_add(name_chirho.qualifier_chirho().map_or(0, str::len))
                .saturating_add(name_chirho.family_key_chirho().map_or(0, str::len));
            if let Some(remaining_chirho) = work_chirho.checked_sub(bytes_chirho) {
                *work_chirho = remaining_chirho;
                true
            } else {
                false
            }
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            family_rhs_supported_chirho(fun_chirho, pattern_chirho, work_chirho, depth_chirho + 1)
                && family_rhs_supported_chirho(
                    arg_chirho,
                    pattern_chirho,
                    work_chirho,
                    depth_chirho + 1,
                )
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            mult_chirho,
            ..
        } => {
            !matches!(mult_chirho, Some(MultiplicityChirho::MultVarChirho(_)))
                && family_rhs_supported_chirho(
                    arg_chirho,
                    pattern_chirho,
                    work_chirho,
                    depth_chirho + 1,
                )
                && family_rhs_supported_chirho(
                    result_chirho,
                    pattern_chirho,
                    work_chirho,
                    depth_chirho + 1,
                )
        }
        TypeChirho::ListChirho { element_chirho, .. }
        | TypeChirho::ParenChirho {
            inner_chirho: element_chirho,
            ..
        } => family_rhs_supported_chirho(
            element_chirho,
            pattern_chirho,
            work_chirho,
            depth_chirho + 1,
        ),
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => elements_chirho.iter().all(|element_chirho| {
            family_rhs_supported_chirho(
                element_chirho,
                pattern_chirho,
                work_chirho,
                depth_chirho + 1,
            )
        }),
        TypeChirho::LitChirho { value_chirho, .. } => {
            if let Some(remaining_chirho) = work_chirho.checked_sub(value_chirho.len()) {
                *work_chirho = remaining_chirho;
                true
            } else {
                false
            }
        }
        TypeChirho::WildcardChirho { .. } => pattern_chirho,
        TypeChirho::QualChirho { .. }
        | TypeChirho::ForallChirho { .. }
        | TypeChirho::RequiredForallChirho { .. } => false,
    }
}

fn prepare_pattern_chirho(
    ty_chirho: &mut TypeChirho,
    equation_chirho: u32,
    wildcard_chirho: &mut u32,
    work_chirho: &mut usize,
    depth_chirho: usize,
) -> bool {
    if depth_chirho > 128 || *work_chirho == 0 {
        return false;
    }
    *work_chirho -= 1;
    match ty_chirho {
        TypeChirho::WildcardChirho { span_chirho } => {
            let name_chirho = format!("%wildcard:{equation_chirho}:{}", *wildcard_chirho);
            let Some(next_chirho) = wildcard_chirho.checked_add(1) else {
                return false;
            };
            *wildcard_chirho = next_chirho;
            *ty_chirho = TypeChirho::VarChirho(NameChirho::RawChirho(
                RawNameChirho::unqualified_chirho(name_chirho, *span_chirho),
            ));
            true
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            prepare_pattern_chirho(
                fun_chirho,
                equation_chirho,
                wildcard_chirho,
                work_chirho,
                depth_chirho + 1,
            ) && prepare_pattern_chirho(
                arg_chirho,
                equation_chirho,
                wildcard_chirho,
                work_chirho,
                depth_chirho + 1,
            )
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            mult_chirho,
            ..
        } => {
            !matches!(mult_chirho, Some(MultiplicityChirho::MultVarChirho(_)))
                && prepare_pattern_chirho(
                    arg_chirho,
                    equation_chirho,
                    wildcard_chirho,
                    work_chirho,
                    depth_chirho + 1,
                )
                && prepare_pattern_chirho(
                    result_chirho,
                    equation_chirho,
                    wildcard_chirho,
                    work_chirho,
                    depth_chirho + 1,
                )
        }
        TypeChirho::ListChirho { element_chirho, .. }
        | TypeChirho::ParenChirho {
            inner_chirho: element_chirho,
            ..
        } => prepare_pattern_chirho(
            element_chirho,
            equation_chirho,
            wildcard_chirho,
            work_chirho,
            depth_chirho + 1,
        ),
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => elements_chirho.iter_mut().all(|element_chirho| {
            prepare_pattern_chirho(
                element_chirho,
                equation_chirho,
                wildcard_chirho,
                work_chirho,
                depth_chirho + 1,
            )
        }),
        TypeChirho::QualChirho { .. }
        | TypeChirho::ForallChirho { .. }
        | TypeChirho::RequiredForallChirho { .. } => false,
        _ => true,
    }
}
