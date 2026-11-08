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
    /// Ordinary constructor specificity is a producer proof on this same slot.
    /// Promotion may hide a named parent-kind binder that ordinary term VTA exposes.
    /// None carries no ordinary constructor proof and never upgrades by spelling.
    pub term_specificity_chirho: Option<haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho>,
    pub variable_chirho: TyVarChirho,
    pub kind_chirho: TyChirho,
    pub visibility_chirho: TelescopeVisibilityChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationLocalBinderChirho {
    pub binder_chirho: DeclarationBinderChirho,
    pub specificity_chirho: haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDeclarationVariableChirho {
    /// Written declaration-header position, independent of hidden telescope slots.
    pub header_index_chirho: u32,
    pub source_chirho:
        haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho,
    pub variable_chirho: TyVarChirho,
    pub specificity_chirho: haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho,
}

/// Association is declared by the class producer, independent of which member
/// spellings an importer admits. A default is an equation, never a synonym.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum DeclarationAssociationChirho {
    #[default]
    NoneChirho,
    ClassChirho {
        families_chirho: Vec<Arc<str>>,
    },
    FamilyChirho {
        class_chirho: Arc<str>,
        /// Family slot -> enclosing class slot, including shared kind slots.
        correspondence_chirho: Vec<(TyVarChirho, TyVarChirho)>,
        default_chirho: Option<AssociatedDefaultEquationChirho>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociatedDefaultEquationChirho {
    pub lhs_chirho: Vec<TyChirho>,
    pub rhs_chirho: TyChirho,
}

/// A declaration boundary is independent of arrows in its result kind.
/// A nullary family returning an arrow has zero required reduction arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationSchemeChirho {
    /// Parser-produced absence/presence of family dependencies. Unknown/stub
    /// metadata never acquires annotation absence by a key or constructor spelling.
    pub injectivity_chirho: Option<bool>,
    pub association_chirho: DeclarationAssociationChirho,
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
    /// Lexical forall binders nested in parameter kinds, result kind or RHS.
    pub local_binders_chirho: Arc<[DeclarationLocalBinderChirho]>,
    /// Stable source quantifiers are mapped explicitly. Inferred binders can
    /// precede them, so a source index is never a typed telescope ordinal.
    pub source_variables_chirho: Vec<SourceDeclarationVariableChirho>,
    pub result_kind_chirho: TyChirho,
    /// Only a type synonym owns a transparent RHS. Its telescope variables
    /// use exactly the same IDs as the kind and argument binders.
    pub synonym_rhs_chirho: Option<TyChirho>,
    /// Producer-checked AST shares the canonical telescope and local forall
    /// slots of the typed body. An importer never receives solver-owner IDs.
    pub synonym_syntax_chirho:
        Option<Arc<super::declaration_syntax_chirho::PublishedSynonymSyntaxChirho>>,
    pub visible_arity_chirho: usize,
    pub reduction_arity_chirho: usize,
    /// The producer's typed promotion witness, retained with its full telescope.
    pub promoted_chirho: Option<haskelujah_ast_chirho::promotion_chirho::PromotedKeyChirho>,
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

pub(crate) fn same_local_binders_chirho(
    left_chirho: &[DeclarationLocalBinderChirho],
    right_chirho: &[DeclarationLocalBinderChirho],
    charge_chirho: &mut impl FnMut(usize) -> Option<()>,
) -> Option<bool> {
    charge_chirho(left_chirho.len().checked_add(right_chirho.len())?)?;
    if left_chirho.len() != right_chirho.len() {
        return Some(false);
    }
    for (left_chirho, right_chirho) in left_chirho.iter().zip(right_chirho) {
        if left_chirho.specificity_chirho != right_chirho.specificity_chirho
            || left_chirho.binder_chirho.term_specificity_chirho
                != right_chirho.binder_chirho.term_specificity_chirho
            || left_chirho.binder_chirho.variable_chirho
                != right_chirho.binder_chirho.variable_chirho
            || left_chirho.binder_chirho.visibility_chirho
                != right_chirho.binder_chirho.visibility_chirho
            || !super::same_type_contract_chirho(
                &left_chirho.binder_chirho.kind_chirho,
                &right_chirho.binder_chirho.kind_chirho,
                charge_chirho,
            )?
        {
            return Some(false);
        }
    }
    Some(true)
}

fn contracts_agree_chirho(
    left_chirho: &DeclarationSchemeChirho,
    right_chirho: &DeclarationSchemeChirho,
) -> Result<bool, String> {
    let mut remaining_chirho = 1usize << 25;
    let mut charge_chirho = |weight_chirho| {
        remaining_chirho = remaining_chirho.checked_sub(weight_chirho)?;
        Some(())
    };
    let result_chirho = (|| {
        // Both schemes have already passed the bounded published-scope validator.
        // Retain its proof-aware AST/scalar equality, then strengthen every Ty field.
        if left_chirho != right_chirho {
            return Some(false);
        }
        if !same_local_binders_chirho(
            &left_chirho.local_binders_chirho,
            &right_chirho.local_binders_chirho,
            &mut charge_chirho,
        )? {
            return Some(false);
        }
        for (left_chirho, right_chirho) in left_chirho
            .binders_chirho
            .iter()
            .map(|binder_chirho| &binder_chirho.kind_chirho)
            .chain(std::iter::once(&left_chirho.result_kind_chirho))
            .chain(left_chirho.synonym_rhs_chirho.iter())
            .zip(
                right_chirho
                    .binders_chirho
                    .iter()
                    .map(|binder_chirho| &binder_chirho.kind_chirho)
                    .chain(std::iter::once(&right_chirho.result_kind_chirho))
                    .chain(right_chirho.synonym_rhs_chirho.iter()),
            )
        {
            if !super::same_type_contract_chirho(left_chirho, right_chirho, &mut charge_chirho)? {
                return Some(false);
            }
        }
        if let (
            DeclarationAssociationChirho::FamilyChirho {
                default_chirho: Some(left_chirho),
                ..
            },
            DeclarationAssociationChirho::FamilyChirho {
                default_chirho: Some(right_chirho),
                ..
            },
        ) = (
            &left_chirho.association_chirho,
            &right_chirho.association_chirho,
        ) {
            for (left_chirho, right_chirho) in left_chirho
                .lhs_chirho
                .iter()
                .chain(std::iter::once(&left_chirho.rhs_chirho))
                .zip(
                    right_chirho
                        .lhs_chirho
                        .iter()
                        .chain(std::iter::once(&right_chirho.rhs_chirho)),
                )
            {
                if !super::same_type_contract_chirho(left_chirho, right_chirho, &mut charge_chirho)?
                {
                    return Some(false);
                }
            }
        }
        Some(true)
    })();
    result_chirho
        .ok_or_else(|| "declaration contract comparison resource bound exhausted".to_owned())
}

/// Canonical headers only, not a competing spelling environment. Spellings
/// resolve through the selected module's naming scopes before this lookup.
#[derive(Debug, Clone, Default)]
pub struct DeclarationSchemesChirho {
    definitions_chirho: HashMap<Arc<str>, Arc<DeclarationSchemeChirho>>,
}

impl DeclarationSchemesChirho {
    pub(crate) fn complete_association_chirho(
        &mut self,
        identity_chirho: &str,
        association_chirho: DeclarationAssociationChirho,
    ) -> Result<(), String> {
        let previous_chirho = self
            .definitions_chirho
            .get(identity_chirho)
            .ok_or_else(|| "missing associated declaration telescope".to_owned())?;
        let mut completed_chirho = previous_chirho.as_ref().clone();
        completed_chirho.association_chirho = association_chirho;
        if completed_chirho.owner_chirho.is_none() {
            validate_published_chirho(&completed_chirho)?;
            if let DeclarationAssociationChirho::FamilyChirho {
                class_chirho,
                correspondence_chirho,
                ..
            } = &completed_chirho.association_chirho
            {
                let parent_chirho = self.definitions_chirho.get(class_chirho).ok_or_else(|| {
                    "associated family has no published class telescope".to_owned()
                })?;
                if parent_chirho.owner_chirho.is_some()
                    || parent_chirho.role_chirho != TypeConstructorRoleChirho::ClassChirho
                {
                    return Err("associated owner is not a checked class".to_owned());
                }
                let class_slots_chirho = parent_chirho
                    .binders_chirho
                    .iter()
                    .map(|binder_chirho| binder_chirho.variable_chirho)
                    .collect::<std::collections::HashSet<_>>();
                if correspondence_chirho
                    .iter()
                    .any(|(_, class_slot_chirho)| !class_slots_chirho.contains(class_slot_chirho))
                {
                    return Err("associated correspondence has an unbound class slot".to_owned());
                }
            }
        }
        self.definitions_chirho.insert(
            completed_chirho.identity_chirho.clone(),
            Arc::new(completed_chirho),
        );
        Ok(())
    }

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
            if !contracts_agree_chirho(previous_chirho, &scheme_chirho)? {
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

    /// Replace only a provisional local header after validating the complete
    /// checked telescope. Imported published contracts remain immutable.
    pub fn publish_chirho(&mut self, scheme_chirho: DeclarationSchemeChirho) -> Result<(), String> {
        validate_published_chirho(&scheme_chirho)?;
        if let Some(previous_chirho) = self.definitions_chirho.get(&scheme_chirho.identity_chirho)
            && previous_chirho.owner_chirho.is_none()
            && !contracts_agree_chirho(previous_chirho, &scheme_chirho)?
        {
            return Err("incompatible published declaration telescope".to_owned());
        }
        self.definitions_chirho.insert(
            scheme_chirho.identity_chirho.clone(),
            Arc::new(scheme_chirho),
        );
        Ok(())
    }

    /// A private module elaborator completes an SCC's transparent bodies only
    /// after all its kind headers are checked. The fixed telescope is unchanged.
    pub(crate) fn complete_synonym_body_chirho(
        &mut self,
        identity_chirho: &str,
        body_chirho: TyChirho,
        syntax_chirho: Arc<super::declaration_syntax_chirho::PublishedSynonymSyntaxChirho>,
    ) -> Result<(), String> {
        let previous_chirho = self
            .definitions_chirho
            .get(identity_chirho)
            .ok_or_else(|| "missing checked synonym declaration".to_owned())?;
        if previous_chirho.owner_chirho.is_some()
            || previous_chirho.role_chirho != TypeConstructorRoleChirho::TypeSynonymChirho
        {
            return Err("only a checked synonym owns a transparent body".to_owned());
        }
        let mut completed_chirho = previous_chirho.as_ref().clone();
        completed_chirho.synonym_rhs_chirho = Some(body_chirho);
        completed_chirho.local_binders_chirho = syntax_chirho.binder_table_chirho();
        completed_chirho.synonym_syntax_chirho = Some(syntax_chirho);
        validate_published_chirho(&completed_chirho)?;
        self.definitions_chirho.insert(
            completed_chirho.identity_chirho.clone(),
            Arc::new(completed_chirho),
        );
        Ok(())
    }

    pub fn merge_chirho(&mut self, other_chirho: &Self) -> Result<(), String> {
        // Validate the entire merge before publishing any new header.
        for (identity_chirho, scheme_chirho) in &other_chirho.definitions_chirho {
            validate_published_chirho(scheme_chirho)?;
            if let Some(previous_chirho) = self.definitions_chirho.get(identity_chirho)
                && !contracts_agree_chirho(previous_chirho, scheme_chirho)?
            {
                return Err("incompatible declaration telescopes".to_owned());
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
    // Validate bounded lexical structure before recursive free-variable helpers.
    super::declaration_syntax_chirho::validate_declaration_scopes_chirho(scheme_chirho)?;
    if scheme_chirho.binders_chirho.iter().any(|binder_chirho| {
        binder_chirho.term_specificity_chirho.is_some()
            && (scheme_chirho.promoted_chirho.is_none()
                || binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho)
    }) {
        return Err("ordinary constructor specificity has no constructor quantifier owner".into());
    }
    if let Some(promoted_chirho) = &scheme_chirho.promoted_chirho
        && (promoted_chirho.as_str_chirho() != scheme_chirho.identity_chirho.as_ref()
            || haskelujah_ast_chirho::promotion_chirho::is_unknown_promoted_key_chirho(
                promoted_chirho.as_str_chirho(),
            ))
    {
        return Err(
            "a declaration promotion witness does not name its checked identity".to_owned(),
        );
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
    match &scheme_chirho.association_chirho {
        DeclarationAssociationChirho::ClassChirho { families_chirho } => {
            let unique_chirho = families_chirho
                .iter()
                .collect::<std::collections::HashSet<_>>();
            if scheme_chirho.role_chirho != TypeConstructorRoleChirho::ClassChirho
                || unique_chirho.len() != families_chirho.len()
            {
                return Err("associated owner is not a class or repeats a family".to_owned());
            }
        }
        DeclarationAssociationChirho::FamilyChirho { default_chirho, .. } => {
            if !matches!(
                scheme_chirho.role_chirho,
                TypeConstructorRoleChirho::TypeFamilyChirho
                    | TypeConstructorRoleChirho::DataFamilyChirho
            ) || (default_chirho.is_some()
                && scheme_chirho.role_chirho != TypeConstructorRoleChirho::TypeFamilyChirho)
            {
                return Err("only an associated type family owns a default equation".to_owned());
            }
        }
        DeclarationAssociationChirho::NoneChirho => {}
    }
    if let DeclarationAssociationChirho::FamilyChirho {
        correspondence_chirho,
        default_chirho,
        ..
    } = &scheme_chirho.association_chirho
    {
        let mut family_slots_chirho = std::collections::HashSet::new();
        if correspondence_chirho.iter().any(|(family_slot_chirho, _)| {
            !bound_chirho.contains(family_slot_chirho)
                || !family_slots_chirho.insert(*family_slot_chirho)
        }) {
            return Err(
                "associated correspondence has an unbound or duplicate family slot".to_owned(),
            );
        }
        if let Some(default_chirho) = default_chirho
            && (default_chirho.lhs_chirho.len() != scheme_chirho.reduction_arity_chirho
                || default_chirho
                    .lhs_chirho
                    .iter()
                    .chain(std::iter::once(&default_chirho.rhs_chirho))
                    .any(|term_chirho| {
                        term_chirho
                            .free_vars_chirho()
                            .iter()
                            .any(|variable_chirho| !bound_chirho.contains(variable_chirho))
                    }))
        {
            return Err("associated default lost its complete bound telescope".to_owned());
        }
    }
    let mut header_indices_chirho = std::collections::HashSet::new();
    for mapping_chirho in &scheme_chirho.source_variables_chirho {
        if !matches!(
            &mapping_chirho.source_chirho,
            ScopedTypeVariableIdChirho::QuantifiedChirho { declaration_chirho, index_chirho }
                if declaration_chirho == &scheme_chirho.identity_chirho && *index_chirho == mapping_chirho.variable_chirho.0
        ) || !header_indices_chirho.insert(mapping_chirho.header_index_chirho)
            || !bound_chirho.contains(&mapping_chirho.variable_chirho)
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

/// Map a checked telescope's bound kind variables to the corresponding current
/// declaration variables. Constructor schemes retain the complete hidden prefix.
pub(crate) fn map_telescope_kind_variables_chirho(
    template_chirho: &TyChirho,
    actual_chirho: &TyChirho,
    mapping_chirho: &mut crate::subst_chirho::SubstChirho,
) {
    match (template_chirho, actual_chirho) {
        (TyChirho::VarChirho(variable_chirho), actual_chirho) => {
            if mapping_chirho.lookup_chirho(variable_chirho).is_none() {
                mapping_chirho.insert_chirho(*variable_chirho, actual_chirho.clone());
            }
        }
        (
            TyChirho::AppChirho(left_chirho, right_chirho),
            TyChirho::AppChirho(actual_left_chirho, actual_right_chirho),
        )
        | (
            TyChirho::FunChirho(left_chirho, right_chirho, _),
            TyChirho::FunChirho(actual_left_chirho, actual_right_chirho, _),
        ) => {
            map_telescope_kind_variables_chirho(left_chirho, actual_left_chirho, mapping_chirho);
            map_telescope_kind_variables_chirho(right_chirho, actual_right_chirho, mapping_chirho);
        }
        (TyChirho::ListChirho(element_chirho), TyChirho::ListChirho(actual_element_chirho)) => {
            map_telescope_kind_variables_chirho(
                element_chirho,
                actual_element_chirho,
                mapping_chirho,
            )
        }
        (TyChirho::TupleChirho(elements_chirho), TyChirho::TupleChirho(actual_elements_chirho)) => {
            for (element_chirho, actual_element_chirho) in
                elements_chirho.iter().zip(actual_elements_chirho)
            {
                map_telescope_kind_variables_chirho(
                    element_chirho,
                    actual_element_chirho,
                    mapping_chirho,
                );
            }
        }
        _ => {}
    }
}
