// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Common declaration telescopes for types, promoted constructors, classes,
//! synonyms and families. The registry transports the same immutable schemes.

use super::{TyChirho, TyVarChirho};
use haskelujah_ast_chirho::name_chirho::TypeConstructorRoleChirho;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelescopeVisibilityChirho {
    InferredChirho,
    SpecifiedChirho,
    RequiredChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationBinderChirho {
    pub variable_chirho: TyVarChirho,
    pub kind_chirho: TyChirho,
    pub visibility_chirho: TelescopeVisibilityChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDeclarationVariableChirho {
    pub source_chirho:
        haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho,
    pub variable_chirho: TyVarChirho,
    pub specificity_chirho: haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho,
}

/// A declaration boundary is independent of arrows in its result kind.
/// A nullary family returning an arrow has zero required reduction arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationSchemeChirho {
    pub identity_chirho: Arc<str>,
    /// A provisional local header may mention this solver's unquantified
    /// metas. Published/imported schemes always have None and quantify all IDs.
    pub owner_chirho: Option<
        haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ElaborationOwnerChirho,
    >,
    pub role_chirho: TypeConstructorRoleChirho,
    pub declaration_chirho:
        Option<haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho>,
    pub binders_chirho: Vec<DeclarationBinderChirho>,
    /// Stable source quantifiers are mapped explicitly. Inferred binders can
    /// precede them, so a source index is never a typed telescope ordinal.
    pub source_variables_chirho: Vec<SourceDeclarationVariableChirho>,
    pub result_kind_chirho: TyChirho,
    /// Only a type synonym owns a transparent RHS. Its telescope variables
    /// use exactly the same IDs as the kind and argument binders.
    pub synonym_rhs_chirho: Option<TyChirho>,
    pub visible_arity_chirho: usize,
    pub reduction_arity_chirho: usize,
    pub promoted_chirho: bool,
    /// An unresolved producer is never a nominal/kind proof.
    pub held_chirho: bool,
}

impl DeclarationSchemeChirho {
    pub fn hidden_arity_chirho(&self) -> usize {
        self.binders_chirho
            .iter()
            .filter(|binder_chirho| {
                binder_chirho.visibility_chirho != TelescopeVisibilityChirho::RequiredChirho
            })
            .count()
    }
}

/// Canonical headers only, not a competing spelling environment. Spellings
/// resolve through the selected module's naming scopes before this lookup.
#[derive(Debug, Clone, Default)]
pub struct DeclarationSchemesChirho {
    definitions_chirho: HashMap<Arc<str>, Arc<DeclarationSchemeChirho>>,
}

impl DeclarationSchemesChirho {
    pub fn is_empty_chirho(&self) -> bool {
        self.definitions_chirho.is_empty()
    }

    pub fn get_chirho(&self, identity_chirho: &str) -> Option<&Arc<DeclarationSchemeChirho>> {
        self.definitions_chirho.get(identity_chirho)
    }

    pub fn iter_chirho(&self) -> impl Iterator<Item = (&Arc<str>, &Arc<DeclarationSchemeChirho>)> {
        self.definitions_chirho.iter()
    }

    pub fn insert_chirho(&mut self, scheme_chirho: DeclarationSchemeChirho) -> Result<(), String> {
        validate_published_chirho(&scheme_chirho)?;
        if let Some(previous_chirho) = self.definitions_chirho.get(&scheme_chirho.identity_chirho) {
            if previous_chirho.as_ref() != &scheme_chirho {
                return Err("incompatible declaration telescopes".to_owned());
            }
            return Ok(());
        }
        self.definitions_chirho.insert(
            scheme_chirho.identity_chirho.clone(),
            Arc::new(scheme_chirho),
        );
        Ok(())
    }

    /// The module elaborator stages a local declaration before checking its
    /// retained imported/boot contract at the publication boundary.
    pub fn stage_chirho(
        &mut self,
        scheme_chirho: DeclarationSchemeChirho,
    ) -> Option<Arc<DeclarationSchemeChirho>> {
        self.definitions_chirho.insert(
            scheme_chirho.identity_chirho.clone(),
            Arc::new(scheme_chirho),
        )
    }

    pub fn merge_chirho(&mut self, other_chirho: &Self) -> Result<(), String> {
        // Validate the entire merge before publishing any new header.
        for (identity_chirho, scheme_chirho) in &other_chirho.definitions_chirho {
            validate_published_chirho(scheme_chirho)?;
            if let Some(previous_chirho) = self.definitions_chirho.get(identity_chirho) {
                if previous_chirho != scheme_chirho {
                    return Err("incompatible declaration telescopes".to_owned());
                }
            }
        }
        for (identity_chirho, scheme_chirho) in &other_chirho.definitions_chirho {
            self.definitions_chirho
                .entry(identity_chirho.clone())
                .or_insert_with(|| scheme_chirho.clone());
        }
        Ok(())
    }
}

fn validate_published_chirho(scheme_chirho: &DeclarationSchemeChirho) -> Result<(), String> {
    use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;
    if scheme_chirho.owner_chirho.is_some() {
        return Err("unpublished declaration telescope".to_owned());
    }
    let mut bound_chirho = std::collections::HashSet::new();
    for binder_chirho in &scheme_chirho.binders_chirho {
        if binder_chirho
            .kind_chirho
            .free_vars_chirho()
            .iter()
            .any(|variable_chirho| !bound_chirho.contains(variable_chirho))
        {
            return Err("a declaration binder kind has a free or forward variable".to_owned());
        }
        if !bound_chirho.insert(binder_chirho.variable_chirho) {
            return Err("duplicate declaration telescope binder".to_owned());
        }
    }
    if std::iter::once(&scheme_chirho.result_kind_chirho)
        .chain(scheme_chirho.synonym_rhs_chirho.iter())
        .any(|term_chirho| {
            term_chirho
                .free_vars_chirho()
                .iter()
                .any(|variable_chirho| !bound_chirho.contains(variable_chirho))
        })
    {
        return Err("published declaration contains an unquantified kind/type variable".to_owned());
    }
    let mut source_chirho = std::collections::HashSet::new();
    for mapping_chirho in &scheme_chirho.source_variables_chirho {
        if !matches!(
            mapping_chirho.source_chirho,
            ScopedTypeVariableIdChirho::QuantifiedChirho { .. }
        ) || !bound_chirho.contains(&mapping_chirho.variable_chirho)
            || !source_chirho.insert(&mapping_chirho.source_chirho)
        {
            return Err("invalid source-to-published declaration binder mapping".to_owned());
        }
    }
    let visible_chirho = scheme_chirho
        .binders_chirho
        .iter()
        .filter(|binder_chirho| {
            binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho
        })
        .count();
    if visible_chirho != scheme_chirho.visible_arity_chirho
        || scheme_chirho.reduction_arity_chirho != scheme_chirho.binders_chirho.len()
    {
        return Err("declaration saturation boundary does not match its telescope".to_owned());
    }
    Ok(())
}

#[cfg(test)]
#[path = "declarations_tests_chirho.rs"]
mod tests_chirho;
