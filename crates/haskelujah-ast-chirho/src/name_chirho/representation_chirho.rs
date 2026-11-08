// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Positive compiler representation declarations. Public spellings do not
//! manufacture these tags, and their private keys cannot match opaque names.

use super::TypeConstructorRoleChirho;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompilerRepresentationChirho {
    RuntimeTypeChirho,
    RuntimeConstraintChirho,
    RuntimeRepChirho,
    LevityChirho,
    VecCountChirho,
    VecElemChirho,
    LiftedRepChirho,
    UnliftedRepChirho,
    ZeroBitRepChirho,
    UnliftedTypeChirho,
    ZeroBitTypeChirho,
}

impl CompilerRepresentationChirho {
    pub const ALL_CHIRHO: [Self; 11] = [
        Self::RuntimeTypeChirho,
        Self::RuntimeConstraintChirho,
        Self::RuntimeRepChirho,
        Self::LevityChirho,
        Self::VecCountChirho,
        Self::VecElemChirho,
        Self::LiftedRepChirho,
        Self::UnliftedRepChirho,
        Self::ZeroBitRepChirho,
        Self::UnliftedTypeChirho,
        Self::ZeroBitTypeChirho,
    ];

    pub const fn name_chirho(self) -> &'static str {
        match self {
            Self::RuntimeTypeChirho => "TYPE",
            Self::RuntimeConstraintChirho => "CONSTRAINT",
            Self::RuntimeRepChirho => "RuntimeRep",
            Self::LevityChirho => "Levity",
            Self::VecCountChirho => "VecCount",
            Self::VecElemChirho => "VecElem",
            Self::LiftedRepChirho => "LiftedRep",
            Self::UnliftedRepChirho => "UnliftedRep",
            Self::ZeroBitRepChirho => "ZeroBitRep",
            Self::UnliftedTypeChirho => "UnliftedType",
            Self::ZeroBitTypeChirho => "ZeroBitType",
        }
    }

    pub const fn type_key_chirho(self) -> &'static str {
        match self {
            Self::RuntimeTypeChirho => "%representation:compiler:TYPE",
            Self::RuntimeConstraintChirho => "%representation:compiler:CONSTRAINT",
            Self::RuntimeRepChirho => "%representation:compiler:RuntimeRep",
            Self::LevityChirho => "%representation:compiler:Levity",
            Self::VecCountChirho => "%representation:compiler:VecCount",
            Self::VecElemChirho => "%representation:compiler:VecElem",
            Self::LiftedRepChirho => "%representation:compiler:LiftedRep",
            Self::UnliftedRepChirho => "%representation:compiler:UnliftedRep",
            Self::ZeroBitRepChirho => "%representation:compiler:ZeroBitRep",
            Self::UnliftedTypeChirho => "%representation:compiler:UnliftedType",
            Self::ZeroBitTypeChirho => "%representation:compiler:ZeroBitType",
        }
    }

    pub const fn role_chirho(self) -> TypeConstructorRoleChirho {
        match self {
            Self::RuntimeTypeChirho | Self::RuntimeConstraintChirho => {
                TypeConstructorRoleChirho::UniverseChirho
            }
            Self::RuntimeRepChirho
            | Self::LevityChirho
            | Self::VecCountChirho
            | Self::VecElemChirho => TypeConstructorRoleChirho::DataChirho,
            Self::LiftedRepChirho
            | Self::UnliftedRepChirho
            | Self::ZeroBitRepChirho
            | Self::UnliftedTypeChirho
            | Self::ZeroBitTypeChirho => TypeConstructorRoleChirho::TypeSynonymChirho,
        }
    }

    /// The source declaration's constructors, not a membership-based proof.
    pub const fn constructors_chirho(self) -> &'static [&'static str] {
        match self {
            Self::RuntimeRepChirho => &[
                "BoxedRep",
                "IntRep",
                "Int8Rep",
                "Int16Rep",
                "Int32Rep",
                "Int64Rep",
                "WordRep",
                "Word8Rep",
                "Word16Rep",
                "Word32Rep",
                "Word64Rep",
                "AddrRep",
                "FloatRep",
                "DoubleRep",
                "TupleRep",
                "SumRep",
                "VecRep",
            ],
            Self::LevityChirho => &["Lifted", "Unlifted"],
            Self::VecCountChirho => &["Vec2", "Vec4", "Vec8", "Vec16", "Vec32", "Vec64"],
            Self::VecElemChirho => &[
                "Int8ElemRep",
                "Int16ElemRep",
                "Int32ElemRep",
                "Int64ElemRep",
                "Word8ElemRep",
                "Word16ElemRep",
                "Word32ElemRep",
                "Word64ElemRep",
                "FloatElemRep",
                "DoubleElemRep",
            ],
            _ => &[],
        }
    }
}
