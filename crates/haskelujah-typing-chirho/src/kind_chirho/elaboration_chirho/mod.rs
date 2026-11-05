// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A single semantic kind/type elaboration vertical. Kinds are TyChirho
//! terms; the public KindChirho representation is a compatibility view only.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

mod associated_chirho;
mod compiler_schemes_chirho;
mod constructors_chirho;
mod declarations_chirho;
mod dependencies_chirho;
mod instantiation_chirho;
pub use dependencies_chirho::source_declaration_dependencies_chirho;
mod module_chirho;
mod publication_chirho;
mod state_chirho;
mod surface_chirho;
mod transport_chirho;
mod value_applications_chirho;
pub(crate) use value_applications_chirho::{
    ArgumentElaborationContextChirho, CheckedBodyTypeChirho, elaborate_body_type_chirho,
    elaborate_required_argument_chirho,
};
mod values_chirho;
pub use state_chirho::{ElaborationErrorChirho, ElaborationStateChirho};

use crate::infer_chirho::{TypeFamilyEnvChirho, TypeFamilyScopeChirho};
use crate::ty_chirho::declarations_chirho::{
    DeclarationBinderChirho, DeclarationSchemeChirho, TelescopeVisibilityChirho,
};
use crate::ty_chirho::{TyChirho, TyVarChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{BuiltinClassScopeChirho, TypeConstructorScopeChirho};
use haskelujah_ast_chirho::promotion_chirho::PromotionScopeChirho;
use haskelujah_span_chirho::SpanChirho;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ElaborationImportsChirho<'imports_chirho> {
    pub registry_chirho: &'imports_chirho TypeFamilyEnvChirho,
    pub type_scope_chirho: &'imports_chirho TypeConstructorScopeChirho,
    pub family_scope_chirho: &'imports_chirho TypeFamilyScopeChirho,
    pub promotion_scope_chirho: &'imports_chirho PromotionScopeChirho,
    pub builtin_classes_chirho: &'imports_chirho BuiltinClassScopeChirho,
}

/// The caller keeps the module it already owns, so there is no unchecked
/// whole-module clone before the elaboration resource boundary. An error leaves
/// that module unpublished: the frontend returns its diagnostic immediately.
pub struct ElaborationResultChirho {
    pub state_chirho: ElaborationStateChirho,
    pub registry_chirho: TypeFamilyEnvChirho,
}

type SourceVariablesChirho = Vec<(String, TyVarChirho, TyChirho)>;

struct ValueSignatureScopeChirho {
    variables_chirho: SourceVariablesChirho,
    checked_chirho: Arc<TyChirho>,
}

#[derive(Default)]
struct DeclarationVariablesChirho {
    constructor_result_origin_chirho: Option<constructors_chirho::ConstructorResultOriginChirho>,
    written_signature_chirho: Option<haskelujah_ast_chirho::decl_chirho::GadtConSignatureChirho>,
    owned_result_kind_chirho: Option<TyChirho>,
    written_chirho: SourceVariablesChirho,
    publication_chirho: crate::subst_chirho::SubstChirho,
    constructor_inferred_chirho: std::collections::HashSet<TyVarChirho>,
}

/// The module owns its staged catalogue; a late argument borrows the SAME
/// catalogue. Lazy positive compiler producers need no whole-catalogue clone.
enum ElaborationCatalogueChirho<'scope_chirho> {
    OwnedChirho(Box<TypeFamilyEnvChirho>),
    BorrowedChirho(&'scope_chirho mut TypeFamilyEnvChirho),
}

impl std::ops::Deref for ElaborationCatalogueChirho<'_> {
    type Target = TypeFamilyEnvChirho;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::OwnedChirho(catalogue_chirho) => catalogue_chirho,
            Self::BorrowedChirho(catalogue_chirho) => catalogue_chirho,
        }
    }
}

impl std::ops::DerefMut for ElaborationCatalogueChirho<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::OwnedChirho(catalogue_chirho) => catalogue_chirho,
            Self::BorrowedChirho(catalogue_chirho) => catalogue_chirho,
        }
    }
}

impl ElaborationCatalogueChirho<'_> {
    fn into_owned_chirho(self) -> Result<TypeFamilyEnvChirho, ElaborationErrorChirho> {
        match self {
            Self::OwnedChirho(catalogue_chirho) => Ok(*catalogue_chirho),
            Self::BorrowedChirho(_) => Err(ElaborationErrorChirho {
                mismatch_chirho: None,
                message_chirho: "late argument attempted to publish an owned module catalogue"
                    .into(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
        }
    }
}

struct TypeElaboratorChirho<'scope_chirho> {
    state_chirho: ElaborationStateChirho,
    registry_chirho: ElaborationCatalogueChirho<'scope_chirho>,
    type_scope_chirho: &'scope_chirho TypeConstructorScopeChirho,
    family_scope_chirho: &'scope_chirho TypeFamilyScopeChirho,
    promotion_scope_chirho: &'scope_chirho PromotionScopeChirho,
    builtin_classes_chirho: &'scope_chirho BuiltinClassScopeChirho,
    star_is_type_chirho: bool,
    poly_kinds_chirho: bool,
    cusks_chirho: bool,
    unlifted_newtypes_chirho: bool,
    scoped_type_variables_chirho: bool,
    signature_variables_chirho: HashMap<String, ValueSignatureScopeChirho>,
    signature_undo_chirho: Vec<(String, Option<ValueSignatureScopeChirho>)>,
    type_depth_chirho: usize,
    rechecking_chirho: bool,
    variables_chirho: HashMap<String, crate::ty_chirho::QuantifiedVariableChirho>,
    scope_undo_chirho: Vec<(String, Option<crate::ty_chirho::QuantifiedVariableChirho>)>,
    declaration_variables_chirho: HashMap<Arc<str>, DeclarationVariablesChirho>,
    current_declaration_chirho: Option<Arc<str>>,
    declaration_contracts_chirho: HashMap<Arc<str>, Arc<DeclarationSchemeChirho>>,
    lexical_images_chirho: Option<&'scope_chirho crate::subst_chirho::SubstChirho>,
    lexical_refinements_chirho: Option<&'scope_chirho crate::env_chirho::TyEnvChirho>,
    lexical_equality_chirho:
        Option<crate::infer_chirho::EqualityNormalizationViewChirho<'scope_chirho>>,
    strict_argument_chirho: bool,
}

#[derive(Debug)]
struct ElaboratedTypeChirho {
    term_chirho: TyChirho,
    kind_chirho: TyChirho,
}

impl TypeElaboratorChirho<'_> {
    /// A forall shadows its binders; free variables discovered in its body
    /// still belong to the surrounding signature/declaration scope.
    fn close_forall_scope_chirho(
        &mut self,
        mark_chirho: usize,
        bound_chirho: &[TyVarChirho],
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let mut retained_chirho = Vec::new();
        let mut seen_chirho = std::collections::HashSet::new();
        self.state_chirho
            .charge_chirho(bound_chirho.len(), bound_chirho.len(), span_chirho)?;
        let bound_chirho = bound_chirho
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>();
        for (name_chirho, previous_chirho) in &self.scope_undo_chirho[mark_chirho..] {
            if previous_chirho.is_some() || seen_chirho.contains(name_chirho) {
                continue;
            }
            let Some(record_chirho) = self.variables_chirho.get(name_chirho) else {
                continue;
            };
            if bound_chirho.contains(&record_chirho.variable_chirho) {
                continue;
            }
            let name_chirho = self.state_chirho.name_chirho(name_chirho, span_chirho)?;
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            let record_chirho = record_chirho.clone();
            seen_chirho.insert(name_chirho.clone());
            retained_chirho.push((name_chirho, record_chirho));
        }
        self.close_scope_chirho(mark_chirho);
        for (name_chirho, record_chirho) in retained_chirho {
            self.bind_lexical_record_chirho(name_chirho, record_chirho, span_chirho)?;
        }
        Ok(())
    }

    fn error_chirho(
        message_chirho: impl Into<String>,
        span_chirho: SpanChirho,
    ) -> ElaborationErrorChirho {
        ElaborationErrorChirho {
            mismatch_chirho: None,
            message_chirho: message_chirho.into(),
            span_chirho,
        }
    }

    fn fresh_kind_chirho(
        &mut self,
        span_chirho: SpanChirho,
    ) -> Result<TyChirho, ElaborationErrorChirho> {
        Ok(TyChirho::VarChirho(self.state_chirho.fresh_chirho(
            Some(ElaborationStateChirho::type_universe_chirho()),
            span_chirho,
        )?))
    }

    fn bind_variable_chirho(
        &mut self,
        name_chirho: String,
        variable_chirho: TyVarChirho,
        kind_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let mut record_chirho = self
            .state_chirho
            .variable_kinds_chirho
            .get(&variable_chirho)
            .cloned()
            .unwrap_or_else(|| {
                crate::ty_chirho::QuantifiedVariableChirho::unknown_chirho(variable_chirho)
            });
        record_chirho.kind_chirho = Some(Arc::new(kind_chirho));
        self.bind_lexical_record_chirho(name_chirho, record_chirho, span_chirho)
    }

    /// The lexical view holds the SAME checked binder vocabulary. Its source
    /// correspondence is restored with its name; solver meta ownership stays
    /// local and is never replaced by a synonym's producer origin.
    fn bind_lexical_record_chirho(
        &mut self,
        name_chirho: String,
        record_chirho: crate::ty_chirho::QuantifiedVariableChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let undo_name_chirho = self.state_chirho.name_chirho(&name_chirho, span_chirho)?;
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        let previous_chirho = self.variables_chirho.insert(name_chirho, record_chirho);
        self.scope_undo_chirho
            .push((undo_name_chirho, previous_chirho));
        Ok(())
    }

    fn hide_variable_chirho(
        &mut self,
        name_chirho: &str,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let name_chirho = self.state_chirho.name_chirho(name_chirho, span_chirho)?;
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        let previous_chirho = self.variables_chirho.remove(&name_chirho);
        self.scope_undo_chirho.push((name_chirho, previous_chirho));
        Ok(())
    }

    fn close_scope_chirho(&mut self, mark_chirho: usize) {
        while self.scope_undo_chirho.len() > mark_chirho {
            let (name_chirho, previous_chirho) =
                self.scope_undo_chirho.pop().expect("scope undo entry");
            if let Some(previous_chirho) = previous_chirho {
                self.variables_chirho.insert(name_chirho, previous_chirho);
            } else {
                self.variables_chirho.remove(&name_chirho);
            }
        }
    }

    fn variable_chirho(
        &mut self,
        name_chirho: &str,
        span_chirho: SpanChirho,
    ) -> Result<(TyVarChirho, TyChirho), ElaborationErrorChirho> {
        if let Some(record_chirho) = self.variables_chirho.get(name_chirho) {
            let variable_chirho = record_chirho.variable_chirho;
            let kind_chirho = record_chirho.kind_chirho.as_ref().ok_or_else(|| {
                Self::error_chirho("lexical binder lost its checked kind", span_chirho)
            })?;
            self.state_chirho.note_source_variable_chirho(
                variable_chirho,
                haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho,
                false,
                span_chirho,
            )?;
            return Ok((
                variable_chirho,
                self.state_chirho.copy_chirho(kind_chirho, span_chirho)?,
            ));
        }
        let kind_chirho = self.fresh_kind_chirho(span_chirho)?;
        let stored_kind_chirho = self.state_chirho.copy_chirho(&kind_chirho, span_chirho)?;
        let variable_chirho = self
            .state_chirho
            .fresh_chirho(Some(stored_kind_chirho), span_chirho)?;
        self.state_chirho.note_source_variable_chirho(
            variable_chirho,
            haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho,
            false,
            span_chirho,
        )?;
        let name_chirho = self.state_chirho.name_chirho(name_chirho, span_chirho)?;
        let stored_kind_chirho = self.state_chirho.copy_chirho(&kind_chirho, span_chirho)?;
        self.bind_variable_chirho(
            name_chirho,
            variable_chirho,
            stored_kind_chirho,
            span_chirho,
        )?;
        Ok((variable_chirho, kind_chirho))
    }
}

/// Full source entry: ordinary type and promoted constructor schemes,
/// declaration telescopes, written type/kind applications and constraints all
/// use the same solver and immutable imported declaration catalogue.
pub fn elaborate_module_chirho(
    module_chirho: &mut ModuleChirho,
    imports_chirho: ElaborationImportsChirho<'_>,
) -> Result<ElaborationResultChirho, ElaborationErrorChirho> {
    elaborate_module_under_policy_chirho(module_chirho, imports_chirho, true, true)
}

/// The authoritative cold source path checks declaration kinds and family
/// equations with the same engine, while leaving value bodies uninferred.
pub fn elaborate_declaration_module_chirho(
    module_chirho: &mut ModuleChirho,
    imports_chirho: ElaborationImportsChirho<'_>,
    signature_headers_chirho: bool,
) -> Result<ElaborationResultChirho, ElaborationErrorChirho> {
    elaborate_module_under_policy_chirho(
        module_chirho,
        imports_chirho,
        false,
        signature_headers_chirho,
    )
}

fn elaborate_module_under_policy_chirho(
    module_chirho: &mut ModuleChirho,
    imports_chirho: ElaborationImportsChirho<'_>,
    values_chirho: bool,
    signature_headers_chirho: bool,
) -> Result<ElaborationResultChirho, ElaborationErrorChirho> {
    let mut elaborator_chirho = TypeElaboratorChirho::new_chirho(module_chirho, imports_chirho)?;
    elaborator_chirho.predeclare_chirho(module_chirho)?;
    elaborator_chirho.predeclare_constructors_chirho(module_chirho)?;
    elaborator_chirho.elaborate_declarations_chirho(module_chirho)?;
    if signature_headers_chirho {
        elaborator_chirho.elaborate_signature_headers_chirho(module_chirho)?;
    }
    if values_chirho {
        elaborator_chirho.elaborate_values_chirho(module_chirho)?;
    } else {
        elaborator_chirho.elaborate_family_instances_chirho(module_chirho)?;
    }
    for index_chirho in 0..elaborator_chirho.state_chirho.next_variable_chirho {
        let variable_chirho = TyVarChirho(index_chirho);
        let term_chirho = elaborator_chirho.state_chirho.resolve_chirho(
            &TyChirho::VarChirho(variable_chirho),
            module_chirho.span_chirho,
        )?;
        if term_chirho != TyChirho::VarChirho(variable_chirho) {
            elaborator_chirho
                .state_chirho
                .substitution_chirho
                .insert_chirho(variable_chirho, term_chirho);
        }
        if elaborator_chirho
            .state_chirho
            .variable_kinds_chirho
            .get(&variable_chirho)
            .and_then(|record_chirho| record_chirho.kind_chirho.as_ref())
            .is_some()
        {
            let kind_chirho = elaborator_chirho
                .state_chirho
                .variable_kind_chirho(variable_chirho, module_chirho.span_chirho)?;
            let kind_chirho = elaborator_chirho
                .state_chirho
                .resolve_chirho(&kind_chirho, module_chirho.span_chirho)?;
            elaborator_chirho
                .state_chirho
                .set_variable_kind_chirho(variable_chirho, kind_chirho);
        }
    }
    Ok(ElaborationResultChirho {
        state_chirho: elaborator_chirho.state_chirho,
        registry_chirho: elaborator_chirho.registry_chirho.into_owned_chirho()?,
    })
}

impl<'scope_chirho> TypeElaboratorChirho<'scope_chirho> {
    fn new_chirho(
        module_chirho: &ModuleChirho,
        imports_chirho: ElaborationImportsChirho<'scope_chirho>,
    ) -> Result<Self, ElaborationErrorChirho> {
        let mut state_chirho = ElaborationStateChirho::new_chirho()?;
        // Snapshots share branches and schemes through Arc. Charge their map keys
        // before cloning the maps; no source/interface is guessed from scan order.
        for definition_chirho in imports_chirho.registry_chirho.definitions_chirho() {
            let key_chirho = definition_chirho.id_chirho().key_chirho();
            state_chirho.charge_chirho(
                key_chirho.len() + 1,
                key_chirho.len() + 1,
                module_chirho.span_chirho,
            )?;
        }
        for (key_chirho, _) in imports_chirho
            .registry_chirho
            .declaration_schemes_chirho()
            .iter_chirho()
        {
            state_chirho.charge_chirho(
                key_chirho.len() + 1,
                key_chirho.len() + 1,
                module_chirho.span_chirho,
            )?;
        }
        for (spelling_chirho, _) in imports_chirho.builtin_classes_chirho.iter_chirho() {
            state_chirho.charge_chirho(
                spelling_chirho.len().saturating_add(1),
                spelling_chirho.len().saturating_add(1),
                module_chirho.span_chirho,
            )?;
        }
        let policy_chirho = Arc::new(state_chirho::ElaborationPolicyChirho {
            builtin_classes_chirho: Arc::new(imports_chirho.builtin_classes_chirho.clone()),
            star_is_type_chirho:
                haskelujah_ast_chirho::module_chirho::extension_enabled_with_default_chirho(
                    module_chirho,
                    "StarIsType",
                    true,
                ),
            poly_kinds_chirho:
                haskelujah_ast_chirho::module_chirho::extension_enabled_with_default_chirho(
                    module_chirho,
                    "PolyKinds",
                    false,
                ),
            cusks_chirho:
                haskelujah_ast_chirho::module_chirho::extension_enabled_with_default_chirho(
                    module_chirho,
                    "CUSKs",
                    false,
                ),
            unlifted_newtypes_chirho:
                haskelujah_ast_chirho::module_chirho::extension_enabled_with_default_chirho(
                    module_chirho,
                    "UnliftedNewtypes",
                    false,
                ),
            scoped_type_variables_chirho:
                haskelujah_ast_chirho::module_chirho::extension_enabled_with_default_chirho(
                    module_chirho,
                    "ScopedTypeVariables",
                    false,
                ),
        });
        state_chirho.source_policy_chirho = Some(policy_chirho.clone());
        Ok(Self {
            state_chirho,
            registry_chirho: ElaborationCatalogueChirho::OwnedChirho(Box::new(
                imports_chirho.registry_chirho.clone(),
            )),
            type_scope_chirho: imports_chirho.type_scope_chirho,
            family_scope_chirho: imports_chirho.family_scope_chirho,
            promotion_scope_chirho: imports_chirho.promotion_scope_chirho,
            builtin_classes_chirho: imports_chirho.builtin_classes_chirho,
            star_is_type_chirho: policy_chirho.star_is_type_chirho,
            poly_kinds_chirho: policy_chirho.poly_kinds_chirho,
            cusks_chirho: policy_chirho.cusks_chirho,
            unlifted_newtypes_chirho: policy_chirho.unlifted_newtypes_chirho,
            scoped_type_variables_chirho: policy_chirho.scoped_type_variables_chirho,
            signature_variables_chirho: HashMap::new(),
            signature_undo_chirho: Vec::new(),
            type_depth_chirho: 0,
            rechecking_chirho: false,
            variables_chirho: HashMap::new(),
            scope_undo_chirho: Vec::new(),
            declaration_variables_chirho: HashMap::new(),
            current_declaration_chirho: None,
            declaration_contracts_chirho: HashMap::new(),
            lexical_images_chirho: None,
            lexical_refinements_chirho: None,
            lexical_equality_chirho: None,
            strict_argument_chirho: false,
        })
    }
}
