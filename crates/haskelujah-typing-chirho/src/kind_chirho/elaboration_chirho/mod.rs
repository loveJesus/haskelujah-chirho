// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A single semantic kind/type elaboration vertical. Kinds are TyChirho
//! terms; the public KindChirho representation is a compatibility view only.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

mod compiler_schemes_chirho;
mod constructors_chirho;
mod declarations_chirho;
mod dependencies_chirho;
pub use dependencies_chirho::source_declaration_dependencies_chirho;
mod module_chirho;
mod state_chirho;
mod surface_chirho;
mod transport_chirho;
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

#[derive(Default)]
struct DeclarationVariablesChirho {
    written_chirho: SourceVariablesChirho,
    publication_chirho: crate::subst_chirho::SubstChirho,
}

struct TypeElaboratorChirho<'scope_chirho> {
    state_chirho: ElaborationStateChirho,
    registry_chirho: TypeFamilyEnvChirho,
    type_scope_chirho: &'scope_chirho TypeConstructorScopeChirho,
    family_scope_chirho: &'scope_chirho TypeFamilyScopeChirho,
    promotion_scope_chirho: &'scope_chirho PromotionScopeChirho,
    builtin_classes_chirho: &'scope_chirho BuiltinClassScopeChirho,
    star_is_type_chirho: bool,
    scoped_type_variables_chirho: bool,
    signature_variables_chirho: HashMap<String, SourceVariablesChirho>,
    signature_undo_chirho: Vec<(String, Option<SourceVariablesChirho>)>,
    type_depth_chirho: usize,
    rechecking_chirho: bool,
    variables_chirho: HashMap<String, (TyVarChirho, TyChirho)>,
    scope_undo_chirho: Vec<(String, Option<(TyVarChirho, TyChirho)>)>,
    declaration_variables_chirho: HashMap<Arc<str>, DeclarationVariablesChirho>,
    current_declaration_chirho: Option<Arc<str>>,
    declaration_contracts_chirho: HashMap<Arc<str>, Arc<DeclarationSchemeChirho>>,
}

#[derive(Debug)]
struct ElaboratedTypeChirho {
    term_chirho: TyChirho,
    kind_chirho: TyChirho,
}

impl TypeElaboratorChirho<'_> {
    fn error_chirho(
        message_chirho: impl Into<String>,
        span_chirho: SpanChirho,
    ) -> ElaborationErrorChirho {
        ElaborationErrorChirho {
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
        let undo_name_chirho = self.state_chirho.name_chirho(&name_chirho, span_chirho)?;
        self.state_chirho.charge_chirho(1, 1, span_chirho)?;
        let previous_chirho = self
            .variables_chirho
            .insert(name_chirho, (variable_chirho, kind_chirho));
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
        if let Some((variable_chirho, kind_chirho)) = self.variables_chirho.get(name_chirho) {
            let variable_chirho = *variable_chirho;
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
    elaborate_module_under_policy_chirho(module_chirho, imports_chirho, true)
}

/// The authoritative cold source path checks declaration kinds and family
/// equations with the same engine, while leaving value bodies uninferred.
pub fn elaborate_declaration_module_chirho(
    module_chirho: &mut ModuleChirho,
    imports_chirho: ElaborationImportsChirho<'_>,
) -> Result<ElaborationResultChirho, ElaborationErrorChirho> {
    elaborate_module_under_policy_chirho(module_chirho, imports_chirho, false)
}

fn elaborate_module_under_policy_chirho(
    module_chirho: &mut ModuleChirho,
    imports_chirho: ElaborationImportsChirho<'_>,
    values_chirho: bool,
) -> Result<ElaborationResultChirho, ElaborationErrorChirho> {
    let mut elaborator_chirho = TypeElaboratorChirho::new_chirho(module_chirho, imports_chirho)?;
    elaborator_chirho.predeclare_chirho(module_chirho)?;
    elaborator_chirho.predeclare_constructors_chirho(module_chirho)?;
    elaborator_chirho.elaborate_declarations_chirho(module_chirho)?;
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
            .contains_key(&variable_chirho)
        {
            let kind_chirho = elaborator_chirho
                .state_chirho
                .variable_kind_chirho(variable_chirho, module_chirho.span_chirho)?;
            let kind_chirho = elaborator_chirho
                .state_chirho
                .resolve_chirho(&kind_chirho, module_chirho.span_chirho)?;
            elaborator_chirho
                .state_chirho
                .variable_kinds_chirho
                .insert(variable_chirho, kind_chirho);
        }
    }
    Ok(ElaborationResultChirho {
        state_chirho: elaborator_chirho.state_chirho,
        registry_chirho: elaborator_chirho.registry_chirho,
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
        Ok(Self {
            state_chirho,
            registry_chirho: imports_chirho.registry_chirho.clone(),
            type_scope_chirho: imports_chirho.type_scope_chirho,
            family_scope_chirho: imports_chirho.family_scope_chirho,
            promotion_scope_chirho: imports_chirho.promotion_scope_chirho,
            builtin_classes_chirho: imports_chirho.builtin_classes_chirho,
            star_is_type_chirho:
                haskelujah_ast_chirho::module_chirho::extension_enabled_with_default_chirho(
                    module_chirho,
                    "StarIsType",
                    true,
                ),
            scoped_type_variables_chirho:
                haskelujah_ast_chirho::module_chirho::extension_enabled_with_default_chirho(
                    module_chirho,
                    "ScopedTypeVariables",
                    false,
                ),
            signature_variables_chirho: HashMap::new(),
            signature_undo_chirho: Vec::new(),
            type_depth_chirho: 0,
            rechecking_chirho: false,
            variables_chirho: HashMap::new(),
            scope_undo_chirho: Vec::new(),
            declaration_variables_chirho: HashMap::new(),
            current_declaration_chirho: None,
            declaration_contracts_chirho: HashMap::new(),
        })
    }
}
