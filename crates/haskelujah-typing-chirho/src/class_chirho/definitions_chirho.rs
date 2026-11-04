// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Canonical class/instance payloads and proof-preserving predicate carriers.

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use haskelujah_ast_chirho::name_chirho::{
    ClassIdentityUnknownChirho, InstanceIdentityChirho, PredicateClassProofChirho,
};

use haskelujah_ast_chirho::expr_chirho::MatchArmChirho;

use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

/// A class predicate: `ClassName Type`, e.g. `Eq Int`, `Show a`.
/// For multi-parameter type classes: `Convert Int String`.
#[derive(Debug, Clone)]
pub struct PredChirho {
    class_identity_chirho: PredicateClassProofChirho,
    /// Hidden kind arguments; separate from ordinary class positions.
    pub kind_tys_chirho: Vec<TyChirho>,
    pub class_name_chirho: String,
    /// The primary type argument.
    pub ty_chirho: TyChirho,
    /// Extra type arguments for multi-parameter type classes.
    /// Empty for single-parameter classes.
    pub extra_tys_chirho: Vec<TyChirho>,
}

/// Explicit legacy argument carrier. It carries no identity authority.
#[derive(Debug, Clone)]
pub struct PredPartsChirho {
    pub kind_tys_chirho: Vec<TyChirho>,
    pub class_name_chirho: String,
    pub ty_chirho: TyChirho,
    pub extra_tys_chirho: Vec<TyChirho>,
}

impl PartialEq for PredChirho {
    fn eq(&self, other_chirho: &Self) -> bool {
        self.kind_tys_chirho == other_chirho.kind_tys_chirho
            && self.class_name_chirho == other_chirho.class_name_chirho
            && self.ty_chirho == other_chirho.ty_chirho
            && self.extra_tys_chirho == other_chirho.extra_tys_chirho
    }
}
impl Eq for PredChirho {}
impl Hash for PredChirho {
    fn hash<HasherChirho: Hasher>(&self, state_chirho: &mut HasherChirho) {
        self.kind_tys_chirho.hash(state_chirho);
        self.class_name_chirho.hash(state_chirho);
        self.ty_chirho.hash(state_chirho);
        self.extra_tys_chirho.hash(state_chirho);
    }
}

impl PredChirho {
    pub fn refuse_metadata_chirho(
        &mut self,
        refusal_chirho: haskelujah_ast_chirho::name_chirho::PredicateMetadataRefusalChirho,
    ) {
        self.class_identity_chirho =
            haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho::UnknownChirho(
                haskelujah_ast_chirho::name_chirho::ClassIdentityUnknownChirho::ResourceChirho(
                    refusal_chirho,
                ),
            );
    }

    /// Called only by an explicit compiler producer, never by a spelling resolver.
    pub fn from_compiler_chirho(
        identity_chirho: haskelujah_ast_chirho::name_chirho::CompilerClassChirho,
        parts_chirho: PredPartsChirho,
    ) -> Self {
        Self::from_producer_chirho(
            parts_chirho,
            haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho::ProvenChirho(
                haskelujah_ast_chirho::name_chirho::PredicateClassIdentityChirho::CompilerChirho(
                    identity_chirho,
                ),
            ),
        )
    }

    pub fn from_proof_chirho(
        class_identity_chirho: haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho,
        parts_chirho: PredPartsChirho,
    ) -> Self {
        Self::from_producer_chirho(parts_chirho, class_identity_chirho)
    }

    pub fn legacy_chirho(parts_chirho: PredPartsChirho) -> Self {
        Self::from_producer_chirho(
            parts_chirho,
            PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho),
        )
    }

    pub fn from_producer_chirho(
        parts_chirho: PredPartsChirho,
        class_identity_chirho: PredicateClassProofChirho,
    ) -> Self {
        Self {
            class_identity_chirho,
            kind_tys_chirho: parts_chirho.kind_tys_chirho,
            class_name_chirho: parts_chirho.class_name_chirho,
            ty_chirho: parts_chirho.ty_chirho,
            extra_tys_chirho: parts_chirho.extra_tys_chirho,
        }
    }

    pub fn class_identity_chirho(&self) -> &PredicateClassProofChirho {
        &self.class_identity_chirho
    }

    pub fn meet_class_identity_chirho(&mut self, other_chirho: &PredicateClassProofChirho) {
        self.class_identity_chirho = self
            .class_identity_chirho
            .clone()
            .meet_chirho(other_chirho.clone());
    }

    pub fn rebuilt_chirho(&self, ty_chirho: TyChirho, extra_tys_chirho: Vec<TyChirho>) -> Self {
        Self::from_producer_chirho(
            PredPartsChirho {
                kind_tys_chirho: self.kind_tys_chirho.clone(),
                class_name_chirho: self.class_name_chirho.clone(),
                ty_chirho,
                extra_tys_chirho,
            },
            self.class_identity_chirho.clone(),
        )
    }

    pub fn rebuilt_parts_chirho(&self, parts_chirho: PredPartsChirho) -> Self {
        Self::from_producer_chirho(parts_chirho, self.class_identity_chirho.clone())
    }

    pub fn terms_chirho(&self) -> crate::ty_chirho::PredicateTermsChirho<'_> {
        crate::ty_chirho::PredicateTermsChirho::new_chirho(
            &self.kind_tys_chirho,
            &self.ty_chirho,
            &self.extra_tys_chirho,
        )
    }

    pub fn mapped_chirho(&self, map_chirho: impl FnMut(&TyChirho) -> TyChirho) -> Self {
        let terms_chirho = self.terms_chirho().mapped_chirho(map_chirho);
        Self::from_producer_chirho(
            PredPartsChirho {
                kind_tys_chirho: terms_chirho.kind_tys_chirho,
                class_name_chirho: self.class_name_chirho.clone(),
                ty_chirho: terms_chirho.ty_chirho,
                extra_tys_chirho: terms_chirho.extra_tys_chirho,
            },
            self.class_identity_chirho.clone(),
        )
    }

    pub fn semantic_tys_chirho(&self) -> impl Iterator<Item = &TyChirho> {
        self.terms_chirho().iter_chirho()
    }

    pub fn from_scheme_chirho(predicate_chirho: &crate::ty_chirho::SchemePredChirho) -> Self {
        Self::from_producer_chirho(
            PredPartsChirho {
                kind_tys_chirho: predicate_chirho.kind_tys_chirho.clone(),
                class_name_chirho: predicate_chirho.class_name_chirho.clone(),
                ty_chirho: predicate_chirho.ty_chirho.clone(),
                extra_tys_chirho: predicate_chirho.extra_tys_chirho.clone(),
            },
            predicate_chirho.class_identity_chirho().clone(),
        )
    }

    pub fn compiler_chirho(
        identity_chirho: haskelujah_ast_chirho::name_chirho::CompilerClassChirho,
        ty_chirho: TyChirho,
    ) -> Self {
        Self::from_compiler_chirho(
            identity_chirho,
            PredPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: identity_chirho.legacy_name_chirho().to_owned(),
                ty_chirho,
                extra_tys_chirho: Vec::new(),
            },
        )
    }

    pub fn new_chirho(class_name_chirho: &str, ty_chirho: TyChirho) -> Self {
        Self::legacy_chirho(PredPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: class_name_chirho.to_string(),
            ty_chirho,
            extra_tys_chirho: vec![],
        })
    }

    /// Create a multi-parameter predicate.
    pub fn new_multi_chirho(class_name_chirho: &str, tys_chirho: Vec<TyChirho>) -> Self {
        let ty_chirho = tys_chirho
            .first()
            .cloned()
            .unwrap_or(TyChirho::int_chirho());
        Self::legacy_chirho(PredPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: class_name_chirho.to_string(),
            ty_chirho,
            extra_tys_chirho: tys_chirho.into_iter().skip(1).collect(),
        })
    }

    /// Get all type arguments.
    pub fn all_tys_chirho(&self) -> Vec<TyChirho> {
        let mut tys_chirho = vec![self.ty_chirho.clone()];
        tys_chirho.extend(self.extra_tys_chirho.clone());
        tys_chirho
    }
}

impl fmt::Display for PredChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{} {}", self.class_name_chirho, self.ty_chirho)?;
        for extra_chirho in &self.extra_tys_chirho {
            write!(f_chirho, " {}", extra_chirho)?;
        }
        Ok(())
    }
}

/// A qualified type: a list of predicates (constraints) and a body type.
/// Example: `(Eq a, Show a) => a -> String`
#[derive(Debug, Clone, PartialEq)]
pub struct QualTyChirho {
    pub preds_chirho: Vec<PredChirho>,
    pub ty_chirho: TyChirho,
}

impl QualTyChirho {
    pub fn unqualified_chirho(ty_chirho: TyChirho) -> Self {
        Self {
            preds_chirho: vec![],
            ty_chirho,
        }
    }

    pub fn qualified_chirho(preds_chirho: Vec<PredChirho>, ty_chirho: TyChirho) -> Self {
        Self {
            preds_chirho,
            ty_chirho,
        }
    }
}

impl fmt::Display for QualTyChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.preds_chirho.is_empty() {
            write!(f_chirho, "{}", self.ty_chirho)
        } else {
            write!(f_chirho, "(")?;
            for (i_chirho, pred_chirho) in self.preds_chirho.iter().enumerate() {
                if i_chirho > 0 {
                    write!(f_chirho, ", ")?;
                }
                write!(f_chirho, "{pred_chirho}")?;
            }
            write!(f_chirho, ") => {}", self.ty_chirho)
        }
    }
}

/// A typeclass declaration.
///
/// Example: `class Eq a where (==) :: a -> a -> Bool`
///
/// Supports multi-parameter type classes (MPTCs) via `vars_chirho`.
/// For single-parameter classes, `var_chirho` returns the single parameter.
#[derive(Debug, Clone)]
pub struct ClassDeclChirho {
    class_identity_chirho: PredicateClassProofChirho,
    /// Quantified kind parameters; never dictionary method positions.
    pub kind_vars_chirho: Vec<TyVarChirho>,
    pub name_chirho: String,
    /// Superclass constraints, e.g. `Eq a` for `class Eq a => Ord a`.
    pub supers_chirho: Vec<String>,
    /// The class type variable (e.g. `a` in `class Eq a`).
    /// For single-parameter classes; use `vars_chirho` for MPTCs.
    pub var_chirho: TyVarChirho,
    /// Method signatures: name → type scheme.
    pub methods_chirho: HashMap<String, SchemeChirho>,
    /// All class type variables. For single-param classes, contains just `[var_chirho]`.
    /// For MPTCs like `class Convert a b`, contains `[a, b]`.
    /// Empty means single-param (inferred from `var_chirho`).
    pub extra_vars_chirho: Vec<TyVarChirho>,
    /// Functional dependencies: each entry `(from, to)` means the types
    /// at `from` indices determine the types at `to` indices.
    /// E.g., `class C a b | a -> b` has `fundeps = [([0], [1])]`.
    pub fundeps_chirho: Vec<(Vec<usize>, Vec<usize>)>,
    /// Default method implementations: name → match arms.
    /// Used when an instance omits a method that has a default in the class.
    pub defaults_chirho: HashMap<String, Vec<MatchArmChirho>>,
}

/// Data moved into one canonical declaration by a producer factory.
#[derive(Debug, Clone)]
pub struct ClassDeclPartsChirho {
    pub kind_vars_chirho: Vec<TyVarChirho>,
    pub name_chirho: String,
    pub supers_chirho: Vec<String>,
    pub var_chirho: TyVarChirho,
    pub methods_chirho: HashMap<String, SchemeChirho>,
    pub extra_vars_chirho: Vec<TyVarChirho>,
    pub fundeps_chirho: Vec<(Vec<usize>, Vec<usize>)>,
    pub defaults_chirho: HashMap<String, Vec<MatchArmChirho>>,
}

impl ClassDeclChirho {
    /// A direct API caller changing a declaration is not its nominal producer.
    pub fn without_identity_chirho(mut self) -> Self {
        self.class_identity_chirho =
            PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho);
        self
    }

    pub fn legacy_chirho(parts_chirho: ClassDeclPartsChirho) -> Self {
        Self::from_producer_chirho(
            parts_chirho,
            PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho),
        )
    }

    pub fn from_producer_chirho(
        parts_chirho: ClassDeclPartsChirho,
        class_identity_chirho: PredicateClassProofChirho,
    ) -> Self {
        Self {
            class_identity_chirho,
            kind_vars_chirho: parts_chirho.kind_vars_chirho,
            name_chirho: parts_chirho.name_chirho,
            supers_chirho: parts_chirho.supers_chirho,
            var_chirho: parts_chirho.var_chirho,
            methods_chirho: parts_chirho.methods_chirho,
            extra_vars_chirho: parts_chirho.extra_vars_chirho,
            fundeps_chirho: parts_chirho.fundeps_chirho,
            defaults_chirho: parts_chirho.defaults_chirho,
        }
    }

    pub fn class_identity_chirho(&self) -> &PredicateClassProofChirho {
        &self.class_identity_chirho
    }

    /// Get all type variables for this class.
    /// For single-param classes, returns `[var_chirho]`.
    /// For MPTCs, returns `[var_chirho] ++ extra_vars_chirho`.
    pub fn all_vars_chirho(&self) -> Vec<TyVarChirho> {
        let mut vars_chirho = vec![self.var_chirho];
        vars_chirho.extend_from_slice(&self.extra_vars_chirho);
        vars_chirho
    }

    /// Whether this is a multi-parameter type class.
    pub fn is_mptc_chirho(&self) -> bool {
        !self.extra_vars_chirho.is_empty()
    }
}

/// A typeclass instance declaration.
///
/// Example: `instance Eq Int where (==) = primEqInt`
#[derive(Debug, Clone)]
pub struct InstDeclChirho {
    class_identity_chirho: PredicateClassProofChirho,
    instance_identity_chirho: InstanceIdentityChirho,
    /// Hidden kind prefix, matched before ordinary class arguments.
    pub kind_tys_chirho: Vec<TyChirho>,
    pub class_name_chirho: String,
    /// The instance head type, e.g. `Int` in `instance Eq Int`.
    pub head_ty_chirho: TyChirho,
    /// Extra head types for multi-parameter type classes.
    /// E.g., `Double` in `instance Convert Int Double where ...`
    /// Empty for single-parameter classes.
    pub extra_head_tys_chirho: Vec<TyChirho>,
    /// Required context, e.g. `Eq a` in `instance Eq a => Eq [a]`.
    pub context_chirho: Vec<PredChirho>,
}

/// Instance arguments without an invented class or declaration identity.
#[derive(Debug, Clone)]
pub struct InstDeclPartsChirho {
    pub kind_tys_chirho: Vec<TyChirho>,
    pub class_name_chirho: String,
    pub head_ty_chirho: TyChirho,
    pub extra_head_tys_chirho: Vec<TyChirho>,
    pub context_chirho: Vec<PredChirho>,
}

impl InstDeclChirho {
    pub fn legacy_chirho(parts_chirho: InstDeclPartsChirho) -> Self {
        Self::from_producer_chirho(
            parts_chirho,
            PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho),
            InstanceIdentityChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho),
        )
    }

    pub fn from_producer_chirho(
        parts_chirho: InstDeclPartsChirho,
        class_identity_chirho: PredicateClassProofChirho,
        instance_identity_chirho: InstanceIdentityChirho,
    ) -> Self {
        Self {
            class_identity_chirho,
            instance_identity_chirho,
            kind_tys_chirho: parts_chirho.kind_tys_chirho,
            class_name_chirho: parts_chirho.class_name_chirho,
            head_ty_chirho: parts_chirho.head_ty_chirho,
            extra_head_tys_chirho: parts_chirho.extra_head_tys_chirho,
            context_chirho: parts_chirho.context_chirho,
        }
    }

    pub fn class_identity_chirho(&self) -> &PredicateClassProofChirho {
        &self.class_identity_chirho
    }
    pub fn instance_identity_chirho(&self) -> &InstanceIdentityChirho {
        &self.instance_identity_chirho
    }
}

impl crate::ty_chirho::PredicateRecordChirho for PredChirho {
    fn refuse_metadata_chirho(
        &mut self,
        refusal_chirho: haskelujah_ast_chirho::name_chirho::PredicateMetadataRefusalChirho,
    ) {
        self.refuse_metadata_chirho(refusal_chirho);
    }

    fn terms_chirho(&self) -> crate::ty_chirho::PredicateTermsChirho<'_> {
        self.terms_chirho()
    }
    fn class_name_chirho(&self) -> &str {
        &self.class_name_chirho
    }
    fn class_identity_chirho(
        &self,
    ) -> &haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho {
        self.class_identity_chirho()
    }
    fn meet_class_identity_chirho(
        &mut self,
        other_chirho: &haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho,
    ) {
        self.meet_class_identity_chirho(other_chirho);
    }
}
