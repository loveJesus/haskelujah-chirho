// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Live variable binding status belongs to the existing State record. Source
//! identity/specificity alone does not distinguish an instantiation meta from
//! a producer-bound universal after the temporary rigid scope has closed.

use super::{ElaborationErrorChirho, ElaborationStateChirho, SpanChirho, TyVarChirho};
use crate::ty_chirho::{QuantifiedVariableChirho, TyChirho};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElaborationVariableBindingChirho {
    MetaChirho,
    UniversalChirho,
    UnknownChirho,
}

#[derive(Debug, Clone)]
pub struct ElaborationVariableChirho {
    pub quantifier_chirho: QuantifiedVariableChirho,
    pub binding_chirho: ElaborationVariableBindingChirho,
}

impl ElaborationVariableChirho {
    pub(crate) fn meta_chirho(quantifier_chirho: QuantifiedVariableChirho) -> Self {
        Self {
            quantifier_chirho,
            binding_chirho: ElaborationVariableBindingChirho::MetaChirho,
        }
    }

    pub(crate) fn unknown_chirho(variable_chirho: TyVarChirho) -> Self {
        Self {
            quantifier_chirho: QuantifiedVariableChirho::unknown_chirho(variable_chirho),
            binding_chirho: ElaborationVariableBindingChirho::UnknownChirho,
        }
    }
}

impl std::ops::Deref for ElaborationVariableChirho {
    type Target = QuantifiedVariableChirho;
    fn deref(&self) -> &Self::Target {
        &self.quantifier_chirho
    }
}

impl From<QuantifiedVariableChirho> for ElaborationVariableChirho {
    fn from(quantifier_chirho: QuantifiedVariableChirho) -> Self {
        Self {
            quantifier_chirho,
            binding_chirho: ElaborationVariableBindingChirho::UnknownChirho,
        }
    }
}

impl std::ops::DerefMut for ElaborationVariableChirho {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.quantifier_chirho
    }
}

impl ElaborationStateChirho {
    pub(crate) fn variable_binding_chirho(
        &self,
        variable_chirho: TyVarChirho,
    ) -> ElaborationVariableBindingChirho {
        self.variable_kinds_chirho.get(&variable_chirho).map_or(
            ElaborationVariableBindingChirho::UnknownChirho,
            |record_chirho| record_chirho.binding_chirho,
        )
    }

    /// Only a genuine fresh-image producer calls this admission. Merely
    /// receiving a kind or copied source metadata does not prove Meta status.
    pub(in crate::kind_chirho::elaboration_chirho) fn set_fresh_variable_kind_chirho(
        &mut self,
        variable_chirho: TyVarChirho,
        kind_chirho: TyChirho,
    ) {
        let record_chirho = self
            .variable_kinds_chirho
            .entry(variable_chirho)
            .or_insert_with(|| ElaborationVariableChirho::unknown_chirho(variable_chirho));
        if record_chirho.binding_chirho != ElaborationVariableBindingChirho::UniversalChirho {
            record_chirho.binding_chirho = ElaborationVariableBindingChirho::MetaChirho;
        }
        record_chirho.kind_chirho = Some(std::sync::Arc::new(kind_chirho));
    }

    /// Called at an actual written/lexical binder or generalization boundary,
    /// never with imported canonical slot ordinals.
    pub(crate) fn mark_universal_chirho(
        &mut self,
        variable_chirho: TyVarChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.charge_chirho(
            1,
            usize::from(!self.variable_kinds_chirho.contains_key(&variable_chirho)),
            span_chirho,
        )?;
        self.variable_kinds_chirho
            .entry(variable_chirho)
            .or_insert_with(|| ElaborationVariableChirho::unknown_chirho(variable_chirho))
            .binding_chirho = ElaborationVariableBindingChirho::UniversalChirho;
        Ok(())
    }

    /// Preserve the barrier on the chosen substitution representative even
    /// when neither side has source specificity. Unknown never proves Meta.
    pub(super) fn link_variable_binding_chirho(
        &mut self,
        source_chirho: TyVarChirho,
        target_chirho: TyVarChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        use ElaborationVariableBindingChirho::{MetaChirho, UniversalChirho, UnknownChirho};
        let target_binding_chirho = self.variable_binding_chirho(target_chirho);
        let binding_chirho = match (
            self.variable_binding_chirho(source_chirho),
            target_binding_chirho,
        ) {
            (UniversalChirho, _) | (_, UniversalChirho) => UniversalChirho,
            (MetaChirho, MetaChirho) => MetaChirho,
            _ => UnknownChirho,
        };
        if binding_chirho != target_binding_chirho {
            self.charge_chirho(
                1,
                usize::from(!self.variable_kinds_chirho.contains_key(&target_chirho)),
                span_chirho,
            )?;
            self.variable_kinds_chirho
                .entry(target_chirho)
                .or_insert_with(|| ElaborationVariableChirho::unknown_chirho(target_chirho))
                .binding_chirho = binding_chirho;
        }
        Ok(())
    }
}
