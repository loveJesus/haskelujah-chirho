// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One compiler-owned class telescope, independent of public spellings and legacy arities.
//! workflow: compiler-pipeline-chirho/builtin-class-identity-chirho

mod identity_chirho;
#[cfg(test)]
mod tests_chirho;
pub use identity_chirho::CompilerClassChirho;
mod binders_chirho;
pub use binders_chirho::{TyVarSpecificityChirho, TyVarVisibilityChirho};

/// Closed compiler kind atoms. No constructor or source-name lookup can mint a new atom.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompilerClassGroundKindChirho(u8);

impl CompilerClassGroundKindChirho {
    pub const TYPE_CHIRHO: Self = Self(0);
    pub const CONSTRAINT_CHIRHO: Self = Self(1);
    pub const NAT_CHIRHO: Self = Self(2);
    pub const SYMBOL_CHIRHO: Self = Self(3);
    pub const CHAR_CHIRHO: Self = Self(4);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilerClassKindAtomChirho {
    GroundChirho(CompilerClassGroundKindChirho),
    /// A preceding slot of this same producer-owned telescope, never a source name.
    BoundSlotChirho(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilerClassKindChirho {
    AtomChirho(CompilerClassKindAtomChirho),
    ArrowChirho(&'static Self, &'static Self),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompilerClassBinderChirho {
    pub kind_chirho: &'static CompilerClassKindChirho,
    pub visibility_chirho: TyVarVisibilityChirho,
    pub specificity_chirho: TyVarSpecificityChirho,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilerClassKindCoverageChirho {
    CompleteChirho,
    /// The existing Symbol-label/lifted-record/lifted-field HasField producer only.
    /// General label kinds and representation-polymorphic fields remain unsupported.
    LiftedSymbolFieldsChirho,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompilerClassSchemaChirho {
    pub binders_chirho: &'static [CompilerClassBinderChirho],
    pub result_kind_chirho: &'static CompilerClassKindChirho,
    pub coverage_chirho: CompilerClassKindCoverageChirho,
}

impl CompilerClassSchemaChirho {
    /// Ordinary source arguments exclude the inferred/specified invisible kind prefix.
    pub fn visible_parameters_chirho(&self) -> impl Iterator<Item = &CompilerClassBinderChirho> {
        self.binders_chirho.iter().filter(|binder_chirho| {
            binder_chirho.visibility_chirho == TyVarVisibilityChirho::VisibleChirho
        })
    }

    pub fn visible_parameter_count_chirho(&self) -> usize {
        self.visible_parameters_chirho().count()
    }

    /// A closed hint retains the complete domain/result tree. An eta count must never
    /// reconstruct this kind: MonadTrans's first domain is itself Type -> Type.
    pub fn closed_parameter_kind_chirho(
        &self,
        index_chirho: usize,
    ) -> Option<&CompilerClassKindChirho> {
        if self.coverage_chirho != CompilerClassKindCoverageChirho::CompleteChirho {
            return None;
        }
        let kind_chirho = self
            .visible_parameters_chirho()
            .nth(index_chirho)?
            .kind_chirho;
        closed_type_arity_chirho(kind_chirho)?;
        Some(kind_chirho)
    }

    /// A closed visible parameter kind is usable as a kind hint without inventing a
    /// telescope instantiation or erasing the hidden prefix from its owning schema.
    pub fn parameter_eta_arity_chirho(&self, index_chirho: usize) -> Option<usize> {
        if self.coverage_chirho != CompilerClassKindCoverageChirho::CompleteChirho {
            return None;
        }
        closed_type_arity_chirho(
            self.visible_parameters_chirho()
                .nth(index_chirho)?
                .kind_chirho,
        )
    }

    /// GND may eta-reduce only one ordinary parameter with no hidden-kind alignment demand.
    /// Count OUTER result arrows: MonadTrans has two, although its domain has an arrow too.
    pub fn single_parameter_eta_arity_chirho(&self) -> Option<usize> {
        let [binder_chirho] = self.binders_chirho else {
            return None;
        };
        if self.coverage_chirho != CompilerClassKindCoverageChirho::CompleteChirho
            || binder_chirho.visibility_chirho != TyVarVisibilityChirho::VisibleChirho
        {
            return None;
        }
        closed_type_arity_chirho(binder_chirho.kind_chirho)
    }
}

fn closed_type_arity_chirho(kind_chirho: &CompilerClassKindChirho) -> Option<usize> {
    let mut pending_chirho = vec![kind_chirho];
    let mut visited_chirho = 0;
    while let Some(kind_chirho) = pending_chirho.pop() {
        visited_chirho += 1;
        if visited_chirho > 128 {
            return None;
        }
        match kind_chirho {
            CompilerClassKindChirho::AtomChirho(CompilerClassKindAtomChirho::GroundChirho(
                CompilerClassGroundKindChirho::TYPE_CHIRHO,
            )) => {}
            CompilerClassKindChirho::ArrowChirho(argument_chirho, result_chirho) => {
                pending_chirho.push(argument_chirho);
                pending_chirho.push(result_chirho);
            }
            _ => return None,
        }
    }
    let mut kind_chirho = kind_chirho;
    let mut arity_chirho = 0;
    while let CompilerClassKindChirho::ArrowChirho(_, result_chirho) = kind_chirho {
        arity_chirho += 1;
        kind_chirho = result_chirho;
    }
    Some(arity_chirho)
}

const TYPE_CHIRHO: CompilerClassKindChirho =
    atom_chirho(CompilerClassGroundKindChirho::TYPE_CHIRHO);
const CONSTRAINT_CHIRHO: CompilerClassKindChirho =
    atom_chirho(CompilerClassGroundKindChirho::CONSTRAINT_CHIRHO);
const NAT_CHIRHO: CompilerClassKindChirho = atom_chirho(CompilerClassGroundKindChirho::NAT_CHIRHO);
const SYMBOL_CHIRHO: CompilerClassKindChirho =
    atom_chirho(CompilerClassGroundKindChirho::SYMBOL_CHIRHO);
const CHAR_CHIRHO: CompilerClassKindChirho =
    atom_chirho(CompilerClassGroundKindChirho::CHAR_CHIRHO);
const KIND_SLOT_CHIRHO: CompilerClassKindChirho =
    CompilerClassKindChirho::AtomChirho(CompilerClassKindAtomChirho::BoundSlotChirho(0));
const CONSTRUCTOR_CHIRHO: CompilerClassKindChirho =
    CompilerClassKindChirho::ArrowChirho(&TYPE_CHIRHO, &TYPE_CHIRHO);
const TRANSFORMER_CHIRHO: CompilerClassKindChirho =
    CompilerClassKindChirho::ArrowChirho(&CONSTRUCTOR_CHIRHO, &CONSTRUCTOR_CHIRHO);

const fn atom_chirho(ground_chirho: CompilerClassGroundKindChirho) -> CompilerClassKindChirho {
    CompilerClassKindChirho::AtomChirho(CompilerClassKindAtomChirho::GroundChirho(ground_chirho))
}
const fn required_chirho(
    kind_chirho: &'static CompilerClassKindChirho,
) -> CompilerClassBinderChirho {
    CompilerClassBinderChirho {
        kind_chirho,
        visibility_chirho: TyVarVisibilityChirho::VisibleChirho,
        specificity_chirho: TyVarSpecificityChirho::SpecifiedChirho,
    }
}
const HIDDEN_KIND_CHIRHO: CompilerClassBinderChirho = CompilerClassBinderChirho {
    kind_chirho: &TYPE_CHIRHO,
    visibility_chirho: TyVarVisibilityChirho::InvisibleChirho,
    specificity_chirho: TyVarSpecificityChirho::InferredChirho,
};
const fn schema_chirho(
    binders_chirho: &'static [CompilerClassBinderChirho],
) -> CompilerClassSchemaChirho {
    CompilerClassSchemaChirho {
        binders_chirho,
        result_kind_chirho: &CONSTRAINT_CHIRHO,
        coverage_chirho: CompilerClassKindCoverageChirho::CompleteChirho,
    }
}
const ORDINARY_CHIRHO: CompilerClassSchemaChirho = schema_chirho(&[required_chirho(&TYPE_CHIRHO)]);
const HIGHER_CHIRHO: CompilerClassSchemaChirho =
    schema_chirho(&[required_chirho(&CONSTRUCTOR_CHIRHO)]);
const TRANSFORMER_SCHEMA_CHIRHO: CompilerClassSchemaChirho =
    schema_chirho(&[required_chirho(&TRANSFORMER_CHIRHO)]);
const NAT_SCHEMA_CHIRHO: CompilerClassSchemaChirho = schema_chirho(&[required_chirho(&NAT_CHIRHO)]);
const SYMBOL_SCHEMA_CHIRHO: CompilerClassSchemaChirho =
    schema_chirho(&[required_chirho(&SYMBOL_CHIRHO)]);
const CHAR_SCHEMA_CHIRHO: CompilerClassSchemaChirho =
    schema_chirho(&[required_chirho(&CHAR_CHIRHO)]);
const TYPEABLE_CHIRHO: CompilerClassSchemaChirho =
    schema_chirho(&[HIDDEN_KIND_CHIRHO, required_chirho(&KIND_SLOT_CHIRHO)]);
const COERCIBLE_CHIRHO: CompilerClassSchemaChirho = schema_chirho(&[
    HIDDEN_KIND_CHIRHO,
    required_chirho(&KIND_SLOT_CHIRHO),
    required_chirho(&KIND_SLOT_CHIRHO),
]);
const HAS_FIELD_CHIRHO: CompilerClassSchemaChirho = CompilerClassSchemaChirho {
    binders_chirho: &[
        required_chirho(&SYMBOL_CHIRHO),
        required_chirho(&TYPE_CHIRHO),
        required_chirho(&TYPE_CHIRHO),
    ],
    result_kind_chirho: &CONSTRAINT_CHIRHO,
    coverage_chirho: CompilerClassKindCoverageChirho::LiftedSymbolFieldsChirho,
};

impl CompilerClassChirho {
    /// The positive compiler identity selects its telescope; there is no reverse spelling map.
    pub const fn schema_chirho(self) -> &'static CompilerClassSchemaChirho {
        match self {
            Self::FunctorChirho
            | Self::ApplicativeChirho
            | Self::MonadChirho
            | Self::FoldableChirho
            | Self::TraversableChirho
            | Self::MonadIoChirho
            | Self::MonadFailChirho
            | Self::MonadFixChirho
            | Self::MonadZipChirho
            | Self::ContravariantChirho
            | Self::AlternativeChirho
            | Self::MonadPlusChirho => &HIGHER_CHIRHO,
            Self::MonadTransChirho => &TRANSFORMER_SCHEMA_CHIRHO,
            Self::KnownNatChirho => &NAT_SCHEMA_CHIRHO,
            Self::KnownSymbolChirho => &SYMBOL_SCHEMA_CHIRHO,
            Self::KnownCharChirho => &CHAR_SCHEMA_CHIRHO,
            Self::TypeableChirho => &TYPEABLE_CHIRHO,
            Self::CoercibleChirho => &COERCIBLE_CHIRHO,
            Self::HasFieldChirho => &HAS_FIELD_CHIRHO,
            Self::EqChirho
            | Self::OrdChirho
            | Self::ShowChirho
            | Self::NumChirho
            | Self::FractionalChirho
            | Self::BitsChirho
            | Self::FiniteBitsChirho
            | Self::ReadChirho
            | Self::IsOptionChirho
            | Self::EnumChirho
            | Self::BoundedChirho
            | Self::FloatingChirho
            | Self::RealFracChirho
            | Self::RealFloatChirho
            | Self::FormatTimeChirho
            | Self::ParseTimeChirho
            | Self::DataChirho
            | Self::RandomGenChirho
            | Self::SplitGenChirho
            | Self::RealChirho
            | Self::IntegralChirho
            | Self::NfDataChirho
            | Self::SemigroupChirho
            | Self::MonoidChirho
            | Self::IsStringChirho
            | Self::IsListChirho
            | Self::GenericChirho => &ORDINARY_CHIRHO,
        }
    }
}
