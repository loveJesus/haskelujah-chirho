// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use std::fmt;

use haskelujah_span_chirho::SpanChirho;

use crate::{NameIdChirho, NameInfoChirho, UnitIdentityChirho};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprIdChirho(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceTypeChirho {
    BoolChirho,
    CharChirho,
    DeclaredChirho(NameIdChirho),
    VariableChirho(NameIdChirho),
    AppChirho(Box<Self>, Box<Self>),
    FunChirho(Box<Self>, Box<Self>),
    ListChirho(Box<Self>),
    TupleChirho(Vec<Self>),
}

#[derive(Debug, Clone)]
pub struct SurfacePredicateChirho {
    pub class_chirho: NameIdChirho,
    pub argument_chirho: NameIdChirho,
}

#[derive(Debug, Clone)]
pub struct SurfaceSignatureChirho {
    pub binders_chirho: Vec<NameIdChirho>,
    pub context_chirho: Vec<SurfacePredicateChirho>,
    pub body_chirho: SurfaceTypeChirho,
}

#[derive(Debug, Clone)]
pub struct ExprChirho {
    pub id_chirho: ExprIdChirho,
    pub kind_chirho: ExprKindChirho,
    pub span_chirho: SpanChirho,
}

#[derive(Debug, Clone)]
pub enum ExprKindChirho {
    VarChirho(NameIdChirho),
    CharChirho(char),
    StringChirho(String),
    AppChirho(Box<ExprChirho>, Box<ExprChirho>),
    LamChirho(Vec<NameIdChirho>, Box<ExprChirho>),
    LetChirho(Vec<BindingChirho>, Box<ExprChirho>),
    IfChirho(Box<ExprChirho>, Box<ExprChirho>, Box<ExprChirho>),
    AnnChirho(Box<ExprChirho>, SurfaceTypeChirho),
    ListChirho(Vec<ExprChirho>),
    TupleChirho(Vec<ExprChirho>),
    MatchChirho(Vec<ExprChirho>, Vec<MatchArmChirho>),
}

#[derive(Debug, Clone)]
pub struct PatternChirho {
    pub id_chirho: ExprIdChirho,
    pub kind_chirho: PatternKindChirho,
    pub span_chirho: SpanChirho,
}

#[derive(Debug, Clone)]
pub enum PatternKindChirho {
    VariableChirho(NameIdChirho),
    WildcardChirho,
    ConstructorChirho(NameIdChirho, Vec<PatternChirho>),
    TupleChirho(Vec<PatternChirho>),
    ListChirho(Vec<PatternChirho>),
    ConsChirho(Box<PatternChirho>, Box<PatternChirho>),
    CharChirho(char),
    AsChirho(NameIdChirho, Box<PatternChirho>),
}

#[derive(Debug, Clone)]
pub struct MatchArmChirho {
    pub patterns_chirho: Vec<PatternChirho>,
    pub body_chirho: ExprChirho,
}

#[derive(Debug, Clone)]
pub struct DataDeclChirho {
    pub name_chirho: NameIdChirho,
    pub parameters_chirho: Vec<NameIdChirho>,
    pub constructors_chirho: Vec<ConstructorChirho>,
}

#[derive(Debug, Clone)]
pub struct ConstructorChirho {
    pub name_chirho: NameIdChirho,
    pub fields_chirho: Vec<SurfaceTypeChirho>,
    pub span_chirho: SpanChirho,
}

#[derive(Debug, Clone)]
pub struct BindingChirho {
    pub name_chirho: NameIdChirho,
    pub body_chirho: ExprChirho,
    pub signature_chirho: Option<SurfaceSignatureChirho>,
    pub restricted_chirho: bool,
    pub span_chirho: SpanChirho,
}

#[derive(Debug, Clone)]
pub struct ClassDeclChirho {
    pub name_chirho: NameIdChirho,
    pub parameter_chirho: NameIdChirho,
    pub methods_chirho: Vec<ClassMethodChirho>,
}

#[derive(Debug, Clone)]
pub struct ClassMethodChirho {
    pub name_chirho: NameIdChirho,
    pub ty_chirho: SurfaceTypeChirho,
    pub span_chirho: SpanChirho,
}

#[derive(Debug, Clone)]
pub struct InstanceDeclChirho {
    pub name_chirho: NameIdChirho,
    pub class_chirho: NameIdChirho,
    pub head_chirho: SurfaceTypeChirho,
    pub methods_chirho: Vec<InstanceMethodChirho>,
    pub span_chirho: SpanChirho,
}

#[derive(Debug, Clone)]
pub struct InstanceMethodChirho {
    pub method_chirho: NameIdChirho,
    pub binding_chirho: BindingChirho,
}

#[derive(Debug)]
pub struct ModuleChirho {
    pub unit_chirho: UnitIdentityChirho,
    pub module_chirho: String,
    pub names_chirho: Vec<NameInfoChirho>,
    pub data_chirho: Vec<DataDeclChirho>,
    pub classes_chirho: Vec<ClassDeclChirho>,
    pub instances_chirho: Vec<InstanceDeclChirho>,
    pub bindings_chirho: Vec<BindingChirho>,
}

#[derive(Debug, Clone, Copy)]
pub struct LimitsChirho {
    pub nodes_chirho: usize,
    pub depth_chirho: usize,
}

impl Default for LimitsChirho {
    fn default() -> Self {
        Self {
            nodes_chirho: 1 << 20,
            depth_chirho: 256,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HirErrorKindChirho {
    InvalidChirho,
    UnsupportedChirho,
    ResourceChirho,
}

#[derive(Debug, Clone)]
pub struct HirErrorChirho {
    pub kind_chirho: HirErrorKindChirho,
    pub message_chirho: String,
    pub span_chirho: SpanChirho,
}

impl fmt::Display for HirErrorChirho {
    fn fmt(&self, formatter_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter_chirho, "{}", self.message_chirho)
    }
}

impl std::error::Error for HirErrorChirho {}
