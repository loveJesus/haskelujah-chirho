// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Name representation
//!
//! Before name resolution, names are simple strings (`RawNameChirho`).
//! After resolution, they carry a unique ID (`ResolvedNameChirho`).

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use haskelujah_span_chirho::SpanChirho;

use crate::promotion_chirho::PromotionResolutionChirho;
use crate::provenance_chirho::OriginIdChirho;

mod predicate_identity_chirho;
pub use predicate_identity_chirho::{
    ClassIdentityUnknownChirho, CompilerClassChirho, CompilerClassEvidenceChirho,
    InstanceDeclarationOriginChirho, InstanceIdentityChirho, PredicateClassIdentityChirho,
    PredicateClassProofChirho, PredicateMetadataRefusalChirho, SourceUnitIdentityChirho,
};

/// A name as written in source code, before resolution.
///
/// Equality and hashing cover the text, the qualifier and the span, exactly as
/// they did before occurrences carried an origin. The origin is use-site
/// metadata for evidence transport, not part of what the name is: every AST
/// comparison and name lookup keeps its meaning, and the evidence join reads
/// `origin_chirho` directly instead of comparing names.
#[derive(Clone)]
pub struct RawNameChirho {
    /// The textual name as it appears in source.
    pub text_chirho: String,
    /// Optional module qualifier (e.g. `Data.List` in `Data.List.sort`).
    pub qualifier_chirho: Option<String>,
    /// Source location of this name occurrence.
    pub span_chirho: SpanChirho,
    /// The producer-minted origin of this occurrence, on an expression reference
    /// whose use can demand class evidence. `None` identifies nothing: an
    /// occurrence without an origin receives no evidence by position or by span.
    /// Moving or cloning the same occurrence keeps its origin; a genuinely new
    /// occurrence takes a fresh one from the module's supply.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub origin_chirho: Option<OriginIdChirho>,
    /// The promoted constructor this spelling names, stamped only where an AST leaves
    /// its module (an exported synonym), so an importer never resolves it in its own
    /// scope. `None` is unannotated: a module's own names are resolved by its
    /// `PromotionScopeChirho`. Like the origin, it is metadata that equality, hashing and
    /// lookups ignore.
    /// workflow: compiler-pipeline-chirho/promoted-constructor-identity-chirho
    pub promotion_chirho: Option<PromotionResolutionChirho>,
    /// Family identity stamped before an exported type leaves its defining scope.
    /// This is transport metadata, ignored by source-name equality and hashing.
    pub family_key_chirho: Option<String>,
    /// An explicit `type` marker on a subordinate export member. This is
    /// namespace syntax, ignored by source-name equality and hashing.
    pub explicit_type_member_chirho: bool,
    /// The parser's positive account of a written defining declaration. This is
    /// never inferred from an exported constructor list or a lowered AST variant:
    /// skipped data instances can retain a data-shaped recovery declaration.
    /// Like the other transport metadata, equality and hashing ignore it.
    pub written_type_declaration_role_chirho: Option<TypeConstructorRoleChirho>,
    /// A variable identity produced by the shared elaborator. Metadata is
    /// cloned through exports and ignored by source Eq/Hash.
    pub type_variable_chirho:
        Option<crate::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho>,
    /// Positive defining identity stamped while this name's source scope is
    /// available; importers never reinterpret an exported synonym's spelling.
    pub type_constructor_chirho: Option<TypeConstructorDeclarationChirho>,
}

impl PartialEq for RawNameChirho {
    fn eq(&self, other_chirho: &Self) -> bool {
        self.text_chirho == other_chirho.text_chirho
            && self.qualifier_chirho == other_chirho.qualifier_chirho
            && self.span_chirho == other_chirho.span_chirho
    }
}

impl Eq for RawNameChirho {}

impl Hash for RawNameChirho {
    fn hash<StateChirho: Hasher>(&self, state_chirho: &mut StateChirho) {
        self.text_chirho.hash(state_chirho);
        self.qualifier_chirho.hash(state_chirho);
        self.span_chirho.hash(state_chirho);
    }
}

impl fmt::Debug for RawNameChirho {
    /// Renders as the derived form did, and names the origin only when there is
    /// one, so output that never involved an origin is unchanged.
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_chirho = f_chirho.debug_struct("RawNameChirho");
        debug_chirho
            .field("text_chirho", &self.text_chirho)
            .field("qualifier_chirho", &self.qualifier_chirho)
            .field("span_chirho", &self.span_chirho);
        if let Some(origin_chirho) = &self.origin_chirho {
            debug_chirho.field("origin_chirho", origin_chirho);
        }
        if let Some(promotion_chirho) = &self.promotion_chirho {
            debug_chirho.field("promotion_chirho", promotion_chirho);
        }
        if let Some(family_key_chirho) = &self.family_key_chirho {
            debug_chirho.field("family_key_chirho", family_key_chirho);
        }
        if self.explicit_type_member_chirho {
            debug_chirho.field("explicit_type_member_chirho", &true);
        }
        if let Some(role_chirho) = &self.written_type_declaration_role_chirho {
            debug_chirho.field("written_type_declaration_role_chirho", role_chirho);
        }
        if let Some(variable_chirho) = &self.type_variable_chirho {
            debug_chirho.field("type_variable_chirho", variable_chirho);
        }
        if let Some(constructor_chirho) = &self.type_constructor_chirho {
            debug_chirho.field("type_constructor_chirho", constructor_chirho);
        }
        debug_chirho.finish()
    }
}

impl RawNameChirho {
    /// Construct an unqualified source name such as `map`.
    pub fn unqualified_chirho(text_chirho: impl Into<String>, span_chirho: SpanChirho) -> Self {
        Self {
            text_chirho: text_chirho.into(),
            qualifier_chirho: None,
            span_chirho,
            origin_chirho: None,
            promotion_chirho: None,
            family_key_chirho: None,
            explicit_type_member_chirho: false,
            written_type_declaration_role_chirho: None,
            type_variable_chirho: None,
            type_constructor_chirho: None,
        }
    }

    /// Construct a qualified source name such as `Data.List.map`.
    pub fn qualified_chirho(
        qualifier_chirho: impl Into<String>,
        text_chirho: impl Into<String>,
        span_chirho: SpanChirho,
    ) -> Self {
        Self {
            text_chirho: text_chirho.into(),
            qualifier_chirho: Some(qualifier_chirho.into()),
            span_chirho,
            origin_chirho: None,
            promotion_chirho: None,
            family_key_chirho: None,
            explicit_type_member_chirho: false,
            written_type_declaration_role_chirho: None,
            type_variable_chirho: None,
            type_constructor_chirho: None,
        }
    }

    /// The same name as an occurrence with a producer-minted origin.
    pub fn with_origin_chirho(mut self, origin_chirho: OriginIdChirho) -> Self {
        self.origin_chirho = Some(origin_chirho);
        self
    }

    /// Return the fully qualified textual form of the name.
    pub fn full_name_chirho(&self) -> String {
        match &self.qualifier_chirho {
            Some(q_chirho) => format!("{}.{}", q_chirho, self.text_chirho),
            None => self.text_chirho.clone(),
        }
    }
}

/// A unique identifier assigned during name resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DefIdChirho(pub u32);

impl DefIdChirho {
    /// Construct a definition identifier from its raw numeric value.
    pub fn new_chirho(id_chirho: u32) -> Self {
        Self(id_chirho)
    }
}

/// A name after resolution, carrying both source text and a unique ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvedNameChirho {
    /// The original source spelling and span information.
    pub raw_chirho: RawNameChirho,
    /// The unique definition this occurrence resolves to.
    pub def_id_chirho: DefIdChirho,
}

/// Generic name type — before or after resolution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NameChirho {
    RawChirho(RawNameChirho),
    ResolvedChirho(ResolvedNameChirho),
}

impl NameChirho {
    pub fn type_variable_chirho(
        &self,
    ) -> Option<&crate::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho> {
        match self {
            Self::RawChirho(raw_chirho) => raw_chirho.type_variable_chirho.as_ref(),
            Self::ResolvedChirho(name_chirho) => {
                name_chirho.raw_chirho.type_variable_chirho.as_ref()
            }
        }
    }

    pub fn set_type_variable_chirho(
        &mut self,
        variable_chirho: Option<
            crate::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho,
        >,
    ) {
        match self {
            Self::RawChirho(raw_chirho) => raw_chirho.type_variable_chirho = variable_chirho,
            Self::ResolvedChirho(name_chirho) => {
                name_chirho.raw_chirho.type_variable_chirho = variable_chirho
            }
        }
    }

    pub fn type_constructor_chirho(&self) -> Option<&TypeConstructorDeclarationChirho> {
        match self {
            Self::RawChirho(raw_chirho) => raw_chirho.type_constructor_chirho.as_ref(),
            Self::ResolvedChirho(name_chirho) => {
                name_chirho.raw_chirho.type_constructor_chirho.as_ref()
            }
        }
    }

    pub fn set_type_constructor_chirho(
        &mut self,
        constructor_chirho: Option<TypeConstructorDeclarationChirho>,
    ) {
        match self {
            Self::RawChirho(raw_chirho) => raw_chirho.type_constructor_chirho = constructor_chirho,
            Self::ResolvedChirho(name_chirho) => {
                name_chirho.raw_chirho.type_constructor_chirho = constructor_chirho
            }
        }
    }

    /// Return the source span covering this name occurrence.
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::RawChirho(n_chirho) => n_chirho.span_chirho,
            Self::ResolvedChirho(n_chirho) => n_chirho.raw_chirho.span_chirho,
        }
    }

    /// Return the unqualified text of the name as written in source.
    pub fn text_chirho(&self) -> &str {
        match self {
            Self::RawChirho(n_chirho) => &n_chirho.text_chirho,
            Self::ResolvedChirho(n_chirho) => &n_chirho.raw_chirho.text_chirho,
        }
    }

    /// Return the fully qualified name, including any module qualifier.
    pub fn full_name_chirho(&self) -> String {
        match self {
            Self::RawChirho(n_chirho) => n_chirho.full_name_chirho(),
            Self::ResolvedChirho(n_chirho) => n_chirho.raw_chirho.full_name_chirho(),
        }
    }

    /// The module qualifier this name was written with, if any.
    pub fn qualifier_chirho(&self) -> Option<&str> {
        match self {
            Self::RawChirho(n_chirho) => n_chirho.qualifier_chirho.as_deref(),
            Self::ResolvedChirho(n_chirho) => n_chirho.raw_chirho.qualifier_chirho.as_deref(),
        }
    }

    /// The producer-minted origin of this occurrence, if it has one.
    pub fn origin_chirho(&self) -> Option<OriginIdChirho> {
        match self {
            Self::RawChirho(n_chirho) => n_chirho.origin_chirho,
            Self::ResolvedChirho(n_chirho) => n_chirho.raw_chirho.origin_chirho,
        }
    }

    /// Give this occurrence `origin_chirho`, or take its origin away.
    pub fn set_origin_chirho(&mut self, origin_chirho: Option<OriginIdChirho>) {
        match self {
            Self::RawChirho(n_chirho) => n_chirho.origin_chirho = origin_chirho,
            Self::ResolvedChirho(n_chirho) => n_chirho.raw_chirho.origin_chirho = origin_chirho,
        }
    }

    /// The same name as an occurrence with a producer-minted origin.
    pub fn with_origin_chirho(mut self, origin_chirho: OriginIdChirho) -> Self {
        self.set_origin_chirho(Some(origin_chirho));
        self
    }

    /// The same name with no origin: a reference re-read as a binder is no
    /// longer an occurrence.
    pub fn without_origin_chirho(mut self) -> Self {
        self.set_origin_chirho(None);
        self
    }

    /// The promoted constructor stamped on this spelling, if it left its module with one.
    pub fn promotion_chirho(&self) -> Option<&PromotionResolutionChirho> {
        match self {
            Self::RawChirho(n_chirho) => n_chirho.promotion_chirho.as_ref(),
            Self::ResolvedChirho(n_chirho) => n_chirho.raw_chirho.promotion_chirho.as_ref(),
        }
    }

    /// Stamp the promoted constructor this spelling names.
    pub fn set_promotion_chirho(&mut self, promotion_chirho: PromotionResolutionChirho) {
        match self {
            Self::RawChirho(n_chirho) => n_chirho.promotion_chirho = Some(promotion_chirho),
            Self::ResolvedChirho(n_chirho) => {
                n_chirho.raw_chirho.promotion_chirho = Some(promotion_chirho);
            }
        }
    }

    /// The defining family carried from the scope in which this type was written.
    pub fn family_key_chirho(&self) -> Option<&str> {
        match self {
            Self::RawChirho(name_chirho) => name_chirho.family_key_chirho.as_deref(),
            Self::ResolvedChirho(name_chirho) => {
                name_chirho.raw_chirho.family_key_chirho.as_deref()
            }
        }
    }

    pub fn set_family_key_chirho(&mut self, family_key_chirho: String) {
        match self {
            Self::RawChirho(name_chirho) => name_chirho.family_key_chirho = Some(family_key_chirho),
            Self::ResolvedChirho(name_chirho) => {
                name_chirho.raw_chirho.family_key_chirho = Some(family_key_chirho)
            }
        }
    }

    /// The parser's declaration-category account on this defining name.
    pub fn written_type_declaration_role_chirho(&self) -> Option<TypeConstructorRoleChirho> {
        match self {
            Self::RawChirho(name_chirho) => name_chirho.written_type_declaration_role_chirho,
            Self::ResolvedChirho(name_chirho) => {
                name_chirho.raw_chirho.written_type_declaration_role_chirho
            }
        }
    }

    pub fn set_written_type_declaration_role_chirho(
        &mut self,
        role_chirho: TypeConstructorRoleChirho,
    ) {
        match self {
            Self::RawChirho(name_chirho) => {
                name_chirho.written_type_declaration_role_chirho = Some(role_chirho)
            }
            Self::ResolvedChirho(name_chirho) => {
                name_chirho.raw_chirho.written_type_declaration_role_chirho = Some(role_chirho)
            }
        }
    }

    /// Whether this subordinate export member was explicitly marked `type`.
    pub fn is_explicit_type_member_chirho(&self) -> bool {
        match self {
            Self::RawChirho(name_chirho) => name_chirho.explicit_type_member_chirho,
            Self::ResolvedChirho(name_chirho) => name_chirho.raw_chirho.explicit_type_member_chirho,
        }
    }

    pub fn set_explicit_type_member_chirho(&mut self, explicit_type_member_chirho: bool) {
        match self {
            Self::RawChirho(name_chirho) => {
                name_chirho.explicit_type_member_chirho = explicit_type_member_chirho
            }
            Self::ResolvedChirho(name_chirho) => {
                name_chirho.raw_chirho.explicit_type_member_chirho = explicit_type_member_chirho
            }
        }
    }
}

/// A defining type declaration's semantic category. Constructor membership
/// supplies no category proof: data families can export constructors too.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeConstructorRoleChirho {
    DataChirho,
    NewtypeChirho,
    ClassChirho,
    TypeSynonymChirho,
    TypeFamilyChirho,
    DataFamilyChirho,
    UniverseChirho,
    UnknownChirho,
}

/// Compiler-owned ordinary data declarations, selected at their actual seed
/// producer. Generic builtin interface entries do not acquire one by spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompilerTypeConstructorChirho {
    BoolChirho,
    OrderingChirho,
    MaybeChirho,
    EitherChirho,
    UnitChirho,
    ProxyChirho,
    TypeUniverseChirho,
    ConstraintUniverseChirho,
    IntChirho,
    IntegerChirho,
    CharChirho,
    FloatChirho,
    DoubleChirho,
    IoChirho,
    ListChirho,
    FunctionChirho,
    TupleChirho(u8),
}

impl CompilerTypeConstructorChirho {
    pub fn name_chirho(self) -> &'static str {
        match self {
            Self::BoolChirho => "Bool",
            Self::OrderingChirho => "Ordering",
            Self::MaybeChirho => "Maybe",
            Self::EitherChirho => "Either",
            Self::UnitChirho => "()",
            Self::ProxyChirho => "Proxy",
            Self::TypeUniverseChirho => "Type",
            Self::ConstraintUniverseChirho => "Constraint",
            Self::IntChirho => "Int",
            Self::IntegerChirho => "Integer",
            Self::CharChirho => "Char",
            Self::FloatChirho => "Float",
            Self::DoubleChirho => "Double",
            Self::IoChirho => "IO",
            Self::ListChirho => "[]",
            Self::FunctionChirho => "->",
            Self::TupleChirho(arity_chirho) => match arity_chirho {
                2 => "(,)",
                3 => "(,,)",
                4 => "(,,,)",
                5 => "(,,,,)",
                6 => "(,,,,,)",
                7 => "(,,,,,,)",
                8 => "(,,,,,,,)",
                9 => "(,,,,,,,,)",
                10 => "(,,,,,,,,,)",
                11 => "(,,,,,,,,,,)",
                12 => "(,,,,,,,,,,,)",
                13 => "(,,,,,,,,,,,,)",
                14 => "(,,,,,,,,,,,,,)",
                15 => "(,,,,,,,,,,,,,,)",
                16 => "(,,,,,,,,,,,,,,,)",
                17 => "(,,,,,,,,,,,,,,,,)",
                18 => "(,,,,,,,,,,,,,,,,,)",
                19 => "(,,,,,,,,,,,,,,,,,,)",
                20 => "(,,,,,,,,,,,,,,,,,,,)",
                21 => "(,,,,,,,,,,,,,,,,,,,,)",
                22 => "(,,,,,,,,,,,,,,,,,,,,,)",
                23 => "(,,,,,,,,,,,,,,,,,,,,,,)",
                24 => "(,,,,,,,,,,,,,,,,,,,,,,,)",
                25 => "(,,,,,,,,,,,,,,,,,,,,,,,,)",
                26 => "(,,,,,,,,,,,,,,,,,,,,,,,,,)",
                27 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                28 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                29 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                30 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                31 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                32 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                33 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                34 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                35 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                36 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                37 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                38 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                39 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                40 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                41 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                42 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                43 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                44 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                45 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                46 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                47 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                48 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                49 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                50 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                51 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                52 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                53 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                54 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                55 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                56 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                57 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                58 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                59 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                60 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                61 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                62 => "(,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)",
                _ => "__invalid_tuple_chirho",
            },
        }
    }
}

/// Definition identity survives aliases and re-exports. Source qualifiers are
/// lookup syntax; only the actual defining producer creates this identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeConstructorIdentityChirho {
    SourceChirho {
        defining_module_chirho: String,
        name_chirho: String,
    },
    CompilerChirho(CompilerTypeConstructorChirho),
}

impl TypeConstructorIdentityChirho {
    /// Injective key compatible with the compiler's existing constructor-origin
    /// catalogue. Its spelling is transport, never a source of proof.
    pub fn key_chirho(&self) -> String {
        match self {
            Self::SourceChirho {
                defining_module_chirho,
                name_chirho,
            } => format!(
                "%constructor:source:{}:{defining_module_chirho}:{}:{name_chirho}",
                defining_module_chirho.len(),
                name_chirho.len()
            ),
            Self::CompilerChirho(constructor_chirho) => {
                let identity_chirho =
                    format!("compiler-standard:{}", constructor_chirho.name_chirho());
                format!(
                    "%constructor:compiler:{}:{identity_chirho}",
                    identity_chirho.len()
                )
            }
        }
    }
}

/// Producer-backed account of one type declaration, not a list of available
/// members. Filtering constructors never removes or manufactures this account.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeConstructorDeclarationChirho {
    pub identity_chirho: TypeConstructorIdentityChirho,
    pub role_chirho: TypeConstructorRoleChirho,
}

impl TypeConstructorDeclarationChirho {
    pub fn source_chirho(
        defining_module_chirho: &str,
        name_chirho: &str,
        role_chirho: TypeConstructorRoleChirho,
    ) -> Self {
        Self {
            identity_chirho: TypeConstructorIdentityChirho::SourceChirho {
                defining_module_chirho: defining_module_chirho.to_owned(),
                name_chirho: name_chirho.to_owned(),
            },
            role_chirho,
        }
    }

    pub fn compiler_data_chirho(constructor_chirho: CompilerTypeConstructorChirho) -> Self {
        Self {
            identity_chirho: TypeConstructorIdentityChirho::CompilerChirho(constructor_chirho),
            role_chirho: match constructor_chirho {
                CompilerTypeConstructorChirho::TypeUniverseChirho
                | CompilerTypeConstructorChirho::ConstraintUniverseChirho => {
                    TypeConstructorRoleChirho::UniverseChirho
                }
                CompilerTypeConstructorChirho::IoChirho => TypeConstructorRoleChirho::NewtypeChirho,
                _ => TypeConstructorRoleChirho::DataChirho,
            },
        }
    }

    pub fn is_ordinary_data_chirho(&self) -> bool {
        matches!(
            self.role_chirho,
            TypeConstructorRoleChirho::DataChirho | TypeConstructorRoleChirho::NewtypeChirho
        )
    }

    /// Nominal decomposition uses the declaration's positive role. Data
    /// families are nominally generative too; synonym families are not.
    /// This does not assert representational injectivity or expose constructors.
    pub fn is_nominally_generative_chirho(&self) -> bool {
        matches!(
            self.role_chirho,
            TypeConstructorRoleChirho::DataChirho
                | TypeConstructorRoleChirho::NewtypeChirho
                | TypeConstructorRoleChirho::ClassChirho
                | TypeConstructorRoleChirho::DataFamilyChirho
        )
    }
}

/// A module's admitted type spellings. A conflicting or unproven binding is
/// monotonic: later imports cannot restore a proof that a binding removed.
#[derive(Debug, Clone, Default)]
pub struct TypeConstructorScopeChirho {
    declarations_chirho: HashMap<String, Option<TypeConstructorDeclarationChirho>>,
}

impl TypeConstructorScopeChirho {
    pub fn new_chirho() -> Self {
        Self::default()
    }

    pub fn declaration_chirho(
        &self,
        spelling_chirho: &str,
    ) -> Option<&TypeConstructorDeclarationChirho> {
        self.declarations_chirho
            .get(spelling_chirho)
            .and_then(Option::as_ref)
    }

    pub fn bind_chirho(
        &mut self,
        spelling_chirho: String,
        declaration_chirho: Option<TypeConstructorDeclarationChirho>,
    ) {
        self.declarations_chirho
            .entry(spelling_chirho)
            .and_modify(|existing_chirho| {
                if existing_chirho.as_ref() != declaration_chirho.as_ref() {
                    *existing_chirho = None;
                }
            })
            .or_insert(declaration_chirho);
    }

    pub fn iter_chirho(&self) -> impl Iterator<Item = (&str, &TypeConstructorDeclarationChirho)> {
        self.declarations_chirho
            .iter()
            .filter_map(|(spelling_chirho, declaration_chirho)| {
                declaration_chirho
                    .as_ref()
                    .map(|declaration_chirho| (spelling_chirho.as_str(), declaration_chirho))
            })
    }
}

/// A class the compiler itself defines, as naming proves it from a trusted builtin
/// interface (K01). Each parameter's kind is the number of `Type` arguments it takes:
/// `[0]` is `Type -> Constraint` and `[1]` is `(Type -> Type) -> Constraint`.
/// workflow: compiler-pipeline-chirho/builtin-class-identity-chirho
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinClassChirho {
    /// The base module documented as the class's home.
    pub module_chirho: &'static str,
    /// The class name as interfaces list it.
    pub name_chirho: &'static str,
    /// One entry per class parameter, in order.
    pub parameter_arities_chirho: &'static [u8],
}

/// The class spellings one module can write (`Eq`, `P.Eq`) that name a builtin class.
/// A spelling that names anything else, or was ever bound ambiguously, is absent.
/// workflow: compiler-pipeline-chirho/builtin-class-identity-chirho
#[derive(Debug, Clone, Default)]
pub struct BuiltinClassScopeChirho {
    classes_chirho: HashMap<String, BuiltinClassChirho>,
}

impl BuiltinClassScopeChirho {
    /// A scope binding each spelling to the builtin class naming proved for it.
    pub fn new_chirho(classes_chirho: HashMap<String, BuiltinClassChirho>) -> Self {
        Self { classes_chirho }
    }

    /// The builtin class `spelling_chirho` names, if naming proved one.
    pub fn class_chirho(&self, spelling_chirho: &str) -> Option<&BuiltinClassChirho> {
        self.classes_chirho.get(spelling_chirho)
    }

    /// Every proven spelling with its class.
    pub fn iter_chirho(&self) -> impl Iterator<Item = (&str, &BuiltinClassChirho)> {
        self.classes_chirho
            .iter()
            .map(|(spelling_chirho, class_chirho)| (spelling_chirho.as_str(), class_chirho))
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn raw_name_unqualified_chirho() {
        let name_chirho = RawNameChirho::unqualified_chirho("foo", SpanChirho::DUMMY_CHIRHO);
        assert_eq!(name_chirho.full_name_chirho(), "foo");
        assert!(name_chirho.qualifier_chirho.is_none());
    }

    #[test]
    fn raw_name_qualified_chirho() {
        let name_chirho =
            RawNameChirho::qualified_chirho("Data.List", "sort", SpanChirho::DUMMY_CHIRHO);
        assert_eq!(name_chirho.full_name_chirho(), "Data.List.sort");
    }

    #[test]
    fn name_enum_text_chirho() {
        let raw_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "bar",
            SpanChirho::DUMMY_CHIRHO,
        ));
        assert_eq!(raw_chirho.text_chirho(), "bar");
    }

    #[test]
    fn explicit_member_namespace_is_metadata_chirho() {
        let ordinary_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "#",
            SpanChirho::DUMMY_CHIRHO,
        ));
        let mut typed_chirho = ordinary_chirho.clone();
        assert!(!typed_chirho.is_explicit_type_member_chirho());
        typed_chirho.set_explicit_type_member_chirho(true);
        assert!(typed_chirho.is_explicit_type_member_chirho());
        assert_eq!(ordinary_chirho, typed_chirho);
        let mut ordinary_hash_chirho = std::collections::hash_map::DefaultHasher::new();
        let mut typed_hash_chirho = std::collections::hash_map::DefaultHasher::new();
        ordinary_chirho.hash(&mut ordinary_hash_chirho);
        typed_chirho.hash(&mut typed_hash_chirho);
        assert_eq!(ordinary_hash_chirho.finish(), typed_hash_chirho.finish());
    }
}
