// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Producer-owned predicate and instance identity vocabulary. Nothing here
//! recognizes a compiler class from source spelling or invents a source unit.
//! Workflow: language-features-chirho/dictionary-evidence-chirho.

use std::sync::Arc;

use haskelujah_span_chirho::SpanChirho;

use crate::provenance_chirho::OriginIdChirho;

/// Compiler identities are minted by explicit compiler producers, never by a
/// source-name lookup. The 45 standard-seed classes are represented here;
/// `HasFieldChirho` additionally names its explicit witness producer, not a
/// registered standard-seed dictionary or an arbitrary record declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompilerClassChirho {
    EqChirho,
    OrdChirho,
    ShowChirho,
    NumChirho,
    FractionalChirho,
    BitsChirho,
    FiniteBitsChirho,
    FunctorChirho,
    ApplicativeChirho,
    MonadChirho,
    FoldableChirho,
    TraversableChirho,
    ReadChirho,
    IsOptionChirho,
    EnumChirho,
    BoundedChirho,
    FloatingChirho,
    RealFracChirho,
    RealFloatChirho,
    MonadIoChirho,
    MonadFailChirho,
    MonadFixChirho,
    FormatTimeChirho,
    ParseTimeChirho,
    TypeableChirho,
    DataChirho,
    KnownNatChirho,
    KnownSymbolChirho,
    KnownCharChirho,
    MonadZipChirho,
    RandomGenChirho,
    SplitGenChirho,
    ContravariantChirho,
    CoercibleChirho,
    RealChirho,
    IntegralChirho,
    NfDataChirho,
    AlternativeChirho,
    MonadPlusChirho,
    SemigroupChirho,
    MonoidChirho,
    MonadTransChirho,
    IsStringChirho,
    IsListChirho,
    GenericChirho,
    HasFieldChirho,
}

/// What an actual compiler-class producer must provide at an evidence use.
/// Equality is not a compiler class and has no representation in this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompilerClassEvidenceChirho {
    DictionaryChirho,
    WitnessChirho,
    ProofRequiredChirho,
}

impl CompilerClassChirho {
    /// Compatibility display text only; there is deliberately no reverse map.
    pub const fn legacy_name_chirho(self) -> &'static str {
        match self {
            Self::EqChirho => "Eq",
            Self::OrdChirho => "Ord",
            Self::ShowChirho => "Show",
            Self::NumChirho => "Num",
            Self::FractionalChirho => "Fractional",
            Self::BitsChirho => "Bits",
            Self::FiniteBitsChirho => "FiniteBits",
            Self::FunctorChirho => "Functor",
            Self::ApplicativeChirho => "Applicative",
            Self::MonadChirho => "Monad",
            Self::FoldableChirho => "Foldable",
            Self::TraversableChirho => "Traversable",
            Self::ReadChirho => "Read",
            Self::IsOptionChirho => "IsOption",
            Self::EnumChirho => "Enum",
            Self::BoundedChirho => "Bounded",
            Self::FloatingChirho => "Floating",
            Self::RealFracChirho => "RealFrac",
            Self::RealFloatChirho => "RealFloat",
            Self::MonadIoChirho => "MonadIO",
            Self::MonadFailChirho => "MonadFail",
            Self::MonadFixChirho => "MonadFix",
            Self::FormatTimeChirho => "FormatTime",
            Self::ParseTimeChirho => "ParseTime",
            Self::TypeableChirho => "Typeable",
            Self::DataChirho => "Data",
            Self::KnownNatChirho => "KnownNat",
            Self::KnownSymbolChirho => "KnownSymbol",
            Self::KnownCharChirho => "KnownChar",
            Self::MonadZipChirho => "MonadZip",
            Self::RandomGenChirho => "RandomGen",
            Self::SplitGenChirho => "SplitGen",
            Self::ContravariantChirho => "Contravariant",
            Self::CoercibleChirho => "Coercible",
            Self::RealChirho => "Real",
            Self::IntegralChirho => "Integral",
            Self::NfDataChirho => "NFData",
            Self::AlternativeChirho => "Alternative",
            Self::MonadPlusChirho => "MonadPlus",
            Self::SemigroupChirho => "Semigroup",
            Self::MonoidChirho => "Monoid",
            Self::MonadTransChirho => "MonadTrans",
            Self::IsStringChirho => "IsString",
            Self::IsListChirho => "IsList",
            Self::GenericChirho => "Generic",
            Self::HasFieldChirho => "HasField",
        }
    }

    pub const fn evidence_chirho(self) -> CompilerClassEvidenceChirho {
        match self {
            Self::KnownNatChirho
            | Self::KnownSymbolChirho
            | Self::KnownCharChirho
            | Self::TypeableChirho
            | Self::HasFieldChirho => CompilerClassEvidenceChirho::WitnessChirho,
            Self::CoercibleChirho => CompilerClassEvidenceChirho::ProofRequiredChirho,
            Self::EqChirho
            | Self::OrdChirho
            | Self::ShowChirho
            | Self::NumChirho
            | Self::FractionalChirho
            | Self::BitsChirho
            | Self::FiniteBitsChirho
            | Self::FunctorChirho
            | Self::ApplicativeChirho
            | Self::MonadChirho
            | Self::FoldableChirho
            | Self::TraversableChirho
            | Self::ReadChirho
            | Self::IsOptionChirho
            | Self::EnumChirho
            | Self::BoundedChirho
            | Self::FloatingChirho
            | Self::RealFracChirho
            | Self::RealFloatChirho
            | Self::MonadIoChirho
            | Self::MonadFailChirho
            | Self::MonadFixChirho
            | Self::FormatTimeChirho
            | Self::ParseTimeChirho
            | Self::DataChirho
            | Self::MonadZipChirho
            | Self::RandomGenChirho
            | Self::SplitGenChirho
            | Self::ContravariantChirho
            | Self::RealChirho
            | Self::IntegralChirho
            | Self::NfDataChirho
            | Self::AlternativeChirho
            | Self::MonadPlusChirho
            | Self::SemigroupChirho
            | Self::MonoidChirho
            | Self::MonadTransChirho
            | Self::IsStringChirho
            | Self::IsListChirho
            | Self::GenericChirho => CompilerClassEvidenceChirho::DictionaryChirho,
        }
    }
}

/// Why this producer supplied no predicate/instance identity proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassIdentityUnknownChirho {
    LegacyChirho,
    WrittenChirho,
    MissingUnitChirho,
    ForeignChirho,
    DerivedChirho,
    BootChirho,
    StubChirho,
    UnsupportedChirho,
    RebindableChirho,
    ConflictChirho,
    AmbiguousChirho,
    ResourceChirho,
}

/// An opaque defining build-unit key from the actual source/package producer.
/// Module names, paths, aliases and elaboration epochs cannot supply this key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceUnitIdentityChirho(Arc<str>);

impl SourceUnitIdentityChirho {
    /// Shape validation only: the caller must be the actual defining-unit
    /// producer. This does not turn an arbitrary spelling into unit authority.
    pub fn from_producer_chirho(
        identity_chirho: Arc<str>,
    ) -> Result<Self, ClassIdentityUnknownChirho> {
        if identity_chirho.is_empty() {
            Err(ClassIdentityUnknownChirho::MissingUnitChirho)
        } else {
            Ok(Self(identity_chirho))
        }
    }

    pub fn as_str_chirho(&self) -> &str {
        &self.0
    }
}

/// Class identities survive source aliases and re-exports unchanged. The source
/// variant is populated only by the defining producer, through `source_chirho`;
/// its module and class names are exact producer data, never inferred qualifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PredicateClassIdentityChirho {
    CompilerChirho(CompilerClassChirho),
    SourceChirho {
        unit_chirho: SourceUnitIdentityChirho,
        module_chirho: Arc<str>,
        name_chirho: Arc<str>,
    },
}

/// Missing or conflicting provenance is not a reflexive class identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PredicateClassProofChirho {
    ProvenChirho(PredicateClassIdentityChirho),
    UnknownChirho(ClassIdentityUnknownChirho),
}

impl PredicateClassProofChirho {
    /// Only the defining source producer calls this after resolving its unit.
    pub fn source_chirho(
        unit_chirho: Option<SourceUnitIdentityChirho>,
        module_chirho: Arc<str>,
        name_chirho: Arc<str>,
    ) -> Self {
        let Some(unit_chirho) = unit_chirho else {
            return Self::UnknownChirho(ClassIdentityUnknownChirho::MissingUnitChirho);
        };
        if module_chirho.is_empty() || name_chirho.is_empty() {
            return Self::UnknownChirho(ClassIdentityUnknownChirho::WrittenChirho);
        }
        Self::ProvenChirho(PredicateClassIdentityChirho::SourceChirho {
            unit_chirho,
            module_chirho,
            name_chirho,
        })
    }

    /// Unknown is absorbing. Preserve the first unknown already encountered;
    /// two different proven identities create a conflict at this meet.
    pub fn meet_chirho(self, other_chirho: Self) -> Self {
        match (self, other_chirho) {
            (left_chirho @ Self::UnknownChirho(_), _) => left_chirho,
            (_, right_chirho @ Self::UnknownChirho(_)) => right_chirho,
            (Self::ProvenChirho(left_chirho), Self::ProvenChirho(right_chirho)) => {
                if left_chirho == right_chirho {
                    Self::ProvenChirho(left_chirho)
                } else {
                    Self::UnknownChirho(ClassIdentityUnknownChirho::ConflictChirho)
                }
            }
        }
    }

    pub fn identity_chirho(&self) -> Option<&PredicateClassIdentityChirho> {
        match self {
            Self::ProvenChirho(identity_chirho) => Some(identity_chirho),
            Self::UnknownChirho(_) => None,
        }
    }

    pub fn unknown_reason_chirho(&self) -> Option<ClassIdentityUnknownChirho> {
        match self {
            Self::UnknownChirho(reason_chirho) => Some(*reason_chirho),
            Self::ProvenChirho(_) => None,
        }
    }
}

/// A producer-retained declaration identity, not a use-site ordinal invented
/// during import or registration. The containing instance also retains unit
/// and module, so an origin remains qualified by its defining compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstanceDeclarationOriginChirho {
    OriginChirho(OriginIdChirho),
    SpanChirho(SpanChirho),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstanceIdentityChirho {
    SeedChirho(usize),
    SourceChirho {
        unit_chirho: SourceUnitIdentityChirho,
        module_chirho: Arc<str>,
        declaration_origin_chirho: InstanceDeclarationOriginChirho,
    },
    UnknownChirho(ClassIdentityUnknownChirho),
}

impl InstanceIdentityChirho {
    /// The actual source producer supplies the declaration's retained origin
    /// or non-placeholder span. Missing source authority stays unknown.
    pub fn source_chirho(
        unit_chirho: Option<SourceUnitIdentityChirho>,
        module_chirho: Arc<str>,
        declaration_origin_chirho: Option<InstanceDeclarationOriginChirho>,
    ) -> Self {
        let Some(unit_chirho) = unit_chirho else {
            return Self::UnknownChirho(ClassIdentityUnknownChirho::MissingUnitChirho);
        };
        if module_chirho.is_empty() {
            return Self::UnknownChirho(ClassIdentityUnknownChirho::WrittenChirho);
        }
        let Some(declaration_origin_chirho) = declaration_origin_chirho else {
            return Self::UnknownChirho(ClassIdentityUnknownChirho::WrittenChirho);
        };
        if let InstanceDeclarationOriginChirho::SpanChirho(span_chirho) = declaration_origin_chirho
            && (!span_chirho.is_valid_chirho() || span_chirho == SpanChirho::DUMMY_CHIRHO)
        {
            return Self::UnknownChirho(ClassIdentityUnknownChirho::WrittenChirho);
        }
        Self::SourceChirho {
            unit_chirho,
            module_chirho,
            declaration_origin_chirho,
        }
    }
}

#[cfg(test)]
#[path = "predicate_identity_tests_chirho.rs"]
mod tests_chirho;
