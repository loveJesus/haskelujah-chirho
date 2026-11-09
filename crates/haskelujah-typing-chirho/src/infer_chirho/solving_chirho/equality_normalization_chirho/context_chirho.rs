// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Borrow the current producer catalogue and implication evidence for one
//! normalization. The ordinary and late kind paths use these same operations.

use super::*;

pub(in crate::infer_chirho) trait NormalizationContextChirho {
    fn equality_substitutions_chirho(&self) -> &SubstChirho;
    fn catalogue_chirho(&self) -> &TypeFamilyEnvChirho;
    fn owner_present_chirho(&self) -> bool;
    fn variable_floor_chirho(&self) -> u32;
    fn lookup_type_synonym_chirho(&self, name_chirho: &str) -> Option<&(Vec<String>, TyChirho)>;
    fn family_name_key_chirho(&self, name_chirho: &str) -> Option<String>;
    fn promotion_present_chirho(&self) -> bool;
    fn reduce_families_bounded_chirho(
        &self,
        term_chirho: &TyChirho,
        depth_chirho: usize,
        work_chirho: &mut usize,
        output_chirho: &mut usize,
    ) -> Option<NormalizationChangeChirho>;
}

impl NormalizationContextChirho for InferCtxChirho {
    fn equality_substitutions_chirho(&self) -> &SubstChirho {
        &self.equality_substitutions_chirho
    }
    fn catalogue_chirho(&self) -> &TypeFamilyEnvChirho {
        &self.type_families_chirho
    }
    fn owner_present_chirho(&self) -> bool {
        self.elaboration_state_chirho.owner_chirho.is_some()
    }
    fn variable_floor_chirho(&self) -> u32 {
        self.elaboration_state_chirho.next_variable_chirho
    }
    fn lookup_type_synonym_chirho(&self, name_chirho: &str) -> Option<&(Vec<String>, TyChirho)> {
        InferCtxChirho::lookup_type_synonym_chirho(self, name_chirho)
    }
    fn family_name_key_chirho(&self, name_chirho: &str) -> Option<String> {
        InferCtxChirho::family_name_key_chirho(self, name_chirho)
    }
    fn promotion_present_chirho(&self) -> bool {
        self.promotion_scope_chirho.is_some()
    }
    fn reduce_families_bounded_chirho(
        &self,
        term_chirho: &TyChirho,
        depth_chirho: usize,
        work_chirho: &mut usize,
        output_chirho: &mut usize,
    ) -> Option<NormalizationChangeChirho> {
        super::super::type_families_chirho::reduce_families_change_with_context_chirho(
            self,
            term_chirho,
            depth_chirho,
            work_chirho,
            output_chirho,
        )
    }
}

/// All references are to the existing inference fields. The retained kind
/// state and its catalogue are lent separately for each normalization call.
#[derive(Clone, Copy)]
pub(crate) struct EqualityNormalizationViewChirho<'view_chirho> {
    pub substitutions_chirho: &'view_chirho SubstChirho,
    pub legacy_synonyms_chirho: &'view_chirho HashMap<String, (Vec<String>, TyChirho)>,
    pub type_scope_chirho:
        Option<&'view_chirho haskelujah_ast_chirho::name_chirho::TypeConstructorScopeChirho>,
    pub family_scope_chirho: Option<&'view_chirho TypeFamilyScopeChirho>,
    pub family_module_chirho: &'view_chirho str,
    pub promotion_present_chirho: bool,
    pub evidence_chirho:
        super::super::equality_scopes_chirho::EqualityEvidenceViewChirho<'view_chirho>,
}

struct BorrowedNormalizationContextChirho<'call_chirho, 'view_chirho> {
    view_chirho: &'call_chirho EqualityNormalizationViewChirho<'view_chirho>,
    catalogue_chirho: &'call_chirho TypeFamilyEnvChirho,
    owner_present_chirho: bool,
    variable_floor_chirho: u32,
}

impl NormalizationContextChirho for BorrowedNormalizationContextChirho<'_, '_> {
    fn equality_substitutions_chirho(&self) -> &SubstChirho {
        self.view_chirho.substitutions_chirho
    }
    fn catalogue_chirho(&self) -> &TypeFamilyEnvChirho {
        self.catalogue_chirho
    }
    fn owner_present_chirho(&self) -> bool {
        self.owner_present_chirho
    }
    fn variable_floor_chirho(&self) -> u32 {
        self.variable_floor_chirho
    }
    fn lookup_type_synonym_chirho(&self, name_chirho: &str) -> Option<&(Vec<String>, TyChirho)> {
        super::super::type_synonyms_chirho::lookup_type_synonym_in_scope_chirho(
            self.view_chirho.legacy_synonyms_chirho,
            self.catalogue_chirho,
            self.view_chirho.type_scope_chirho,
            name_chirho,
        )
    }
    fn family_name_key_chirho(&self, name_chirho: &str) -> Option<String> {
        super::super::family_registration_chirho::family_name_key_in_scope_chirho(
            self.view_chirho.family_scope_chirho,
            self.catalogue_chirho,
            self.view_chirho.family_module_chirho,
            name_chirho,
        )
    }
    fn promotion_present_chirho(&self) -> bool {
        self.view_chirho.promotion_present_chirho
    }
    fn reduce_families_bounded_chirho(
        &self,
        term_chirho: &TyChirho,
        depth_chirho: usize,
        work_chirho: &mut usize,
        output_chirho: &mut usize,
    ) -> Option<NormalizationChangeChirho> {
        super::super::type_families_chirho::reduce_families_change_with_context_chirho(
            self,
            term_chirho,
            depth_chirho,
            work_chirho,
            output_chirho,
        )
    }
}

impl EqualityNormalizationViewChirho<'_> {
    pub(in crate::infer_chirho) fn normalize_with_probe_rules_chirho(
        &self,
        catalogue_chirho: &TypeFamilyEnvChirho,
        owner_present_chirho: bool,
        variable_floor_chirho: u32,
        term_chirho: &TyChirho,
        rules_chirho: &[(TyChirho, TyChirho)],
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Result<TyChirho, EqualityNormalizationExhaustedChirho> {
        let context_chirho = BorrowedNormalizationContextChirho {
            view_chirho: self,
            catalogue_chirho,
            owner_present_chirho,
            variable_floor_chirho,
        };
        normalize_owned_equality_with_budget_chirho(
            &context_chirho,
            term_chirho,
            std::iter::once(rules_chirho).chain(self.evidence_chirho.iter_chirho()),
            budget_chirho,
        )
    }

    pub(crate) fn normalize_chirho(
        &self,
        catalogue_chirho: &TypeFamilyEnvChirho,
        owner_present_chirho: bool,
        variable_floor_chirho: u32,
        term_chirho: &TyChirho,
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Result<TyChirho, EqualityNormalizationExhaustedChirho> {
        let context_chirho = BorrowedNormalizationContextChirho {
            view_chirho: self,
            catalogue_chirho,
            owner_present_chirho,
            variable_floor_chirho,
        };
        normalize_owned_equality_with_budget_chirho(
            &context_chirho,
            term_chirho,
            self.evidence_chirho.iter_chirho(),
            budget_chirho,
        )
    }
}
