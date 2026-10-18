// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Type inference engine (Algorithm W)
//!
//! Implements Hindley-Milner type inference with let-generalization.
//! Walks the AST and produces typed bindings via unification.

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, MatchArmChirho, RhsChirho, StmtChirho};
use haskelujah_ast_chirho::lit_chirho::{
    LitChirho, parse_haskell_char_body_chirho, render_haskell_char_body_chirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::provenance_chirho::OriginIdChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

use crate::class_chirho::{ClassDeclChirho, ClassEnvChirho, InstDeclChirho, PredChirho};
use crate::env_chirho::TyEnvChirho;
use crate::skolem_chirho::{
    constructor_result_refines_chirho, contains_skolem_chirho, open_skolems_chirho,
    skolem_name_chirho, split_refinement_unifier_chirho,
};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{MultChirho, SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};
use crate::unify_chirho::{UnifyErrorChirho, unify_chirho};

// The AST-to-type conversion modules live together in `infer_chirho/conversion_chirho/`.
#[path = "infer_chirho/conversion_chirho/ast_conversion_chirho.rs"]
mod ast_conversion_chirho;
#[cfg(test)]
#[path = "infer_chirho/conversion_chirho/ast_conversion_tests_chirho.rs"]
mod ast_conversion_tests_chirho;
mod builtins_chirho;
mod conditionals_chirho;
mod do_operations_chirho;
mod equalities_chirho;
mod evidence_chirho;
mod fundep_generalization_chirho;
mod guards_chirho;
// The instance-declaration modules live together in `infer_chirho/instances_chirho/`.
#[path = "infer_chirho/instances_chirho/instance_obligations_chirho.rs"]
mod instance_obligations_chirho;
#[path = "infer_chirho/instances_chirho/instance_signatures_chirho.rs"]
mod instance_signatures_chirho;
mod pattern_bindings_chirho;
mod records_chirho;
mod rigid_chirho;
#[cfg(test)]
mod signature_binder_tests_chirho;
mod signature_binders_chirho;
mod signature_conversion_chirho;

use builtins_chirho::seed_builtins_chirho;

pub use evidence_chirho::{
    LiteralEvidenceChirho, OWN_DICTIONARY_KEY_CHIRHO, ReferenceEvidenceChirho,
    is_own_dictionary_key_chirho, own_dictionary_key_chirho,
};
use fundep_generalization_chirho::{FundepGeneralizationChirho, GeneralizationScopeChirho};

/// Error code range for type inference diagnostics.
const TYPE_MISMATCH_CODE_CHIRHO: u16 = 200;
const OCCURS_CHECK_CODE_CHIRHO: u16 = 201;
const UNBOUND_VAR_CODE_CHIRHO: u16 = 202;
const TUPLE_ARITY_CODE_CHIRHO: u16 = 203;
const UNSATISFIED_CONSTRAINT_CODE_CHIRHO: u16 = 204;
const SIGNATURE_MISMATCH_CODE_CHIRHO: u16 = 205;
/// A record construction or update names a field no constructor of the
/// (fully known) record type declares.
const RECORD_FIELD_CODE_CHIRHO: u16 = 206;
/// A record construction omits a field the constructor declares strict
/// (GHC-95909: "does not have the required strict field").
const RECORD_STRICT_FIELD_CODE_CHIRHO: u16 = 207;

pub type TypeFamilyEnvChirho = HashMap<String, Vec<(Vec<TyChirho>, TyChirho)>>;

/// The default equation of an associated type family, remembered on its
/// class and instantiated for each instance that does not define the family
/// itself (`class C a where type F a; type F a = Maybe a` gives
/// `F Int = Maybe Int` to `instance C Int`, never `F a = Maybe a` in general).
#[derive(Debug, Clone)]
struct AssocTypeDefaultChirho {
    family_chirho: String,
    family_params_chirho: Vec<String>,
    class_params_chirho: Vec<String>,
    rhs_chirho: TypeChirho,
}

fn strip_name_qualifier_chirho(name_text_chirho: &str) -> &str {
    name_text_chirho
        .rsplit('.')
        .next()
        .unwrap_or(name_text_chirho)
}

fn canonical_value_name_text_chirho(name_text_chirho: &str) -> String {
    let (qualifier_chirho, bare_name_chirho) =
        if let Some((qualifier_chirho, suffix_chirho)) = name_text_chirho.rsplit_once('.') {
            (Some(qualifier_chirho), suffix_chirho)
        } else {
            (None, name_text_chirho)
        };
    let canonical_bare_name_chirho = if bare_name_chirho.starts_with('(')
        && bare_name_chirho.ends_with(')')
        && bare_name_chirho.len() > 2
    {
        let inner_chirho = &bare_name_chirho[1..bare_name_chirho.len() - 1];
        if !inner_chirho.is_empty()
            && inner_chirho
                .chars()
                .all(|char_chirho| !char_chirho.is_alphanumeric() && char_chirho != '_')
        {
            inner_chirho.to_string()
        } else {
            bare_name_chirho.to_string()
        }
    } else {
        bare_name_chirho.to_string()
    };
    match qualifier_chirho {
        Some(qualifier_chirho) => format!("{qualifier_chirho}.{canonical_bare_name_chirho}"),
        None => canonical_bare_name_chirho,
    }
}

fn record_field_key_chirho(name_chirho: &NameChirho) -> String {
    strip_name_qualifier_chirho(&name_chirho.full_name_chirho()).to_string()
}

fn mk_type_rep_ty_chirho(arg_ty_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("TypeRep".to_string())),
        Box::new(arg_ty_chirho),
    )
}

/// Result of type inference on a module.
#[derive(Debug)]
pub struct InferResultChirho {
    /// The final substitution after inference.
    pub subst_chirho: SubstChirho,
    /// The type environment after inference (with all top-level bindings).
    pub env_chirho: TyEnvChirho,
    /// The class environment after inference.
    pub class_env_chirho: ClassEnvChirho,
    /// Type family equations available after inferring this module.
    pub type_families_chirho: TypeFamilyEnvChirho,
    /// Diagnostics collected during inference.
    pub diagnostics_chirho: DiagnosticBundleChirho,
    /// Evidence-threading P2 (design-evidence-threading-chirho.md): one record per
    /// reference whose instantiated scheme
    /// carried class predicates, finalized through the module's composed
    /// substitution (unification + Report defaulting). Only records whose
    /// predicate type resolved to a CONCRETE head key survive finalization.
    pub method_occurrences_chirho: Vec<MethodOccurrenceRecordChirho>,
    /// Evidence-threading P2b: total pred-carrying references counted per name
    /// (source order). Consumers must join records only for names whose total
    /// matches their own occurrence count (conservative alignment check).
    pub method_occurrence_totals_chirho: HashMap<String, u32>,
    /// Literal evidence: per source literal (by span), the instance the
    /// checker solved for its overloading class. See `evidence_chirho.rs`.
    pub literal_evidence_chirho: HashMap<SpanChirho, LiteralEvidenceChirho>,
    /// The same literal evidence keyed by the literal's origin. It also holds
    /// generated literals, whose placeholder span the span map skips.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub literal_evidence_by_origin_chirho: HashMap<OriginIdChirho, LiteralEvidenceChirho>,
    /// Reference evidence: per constrained reference (by span), one record per
    /// predicate of the instantiated scheme, in scheme order.
    pub reference_evidence_chirho: HashMap<SpanChirho, Vec<ReferenceEvidenceChirho>>,
    /// The same reference evidence keyed by the reference's origin. It also
    /// holds generated references, whose placeholder span the span map skips.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub reference_evidence_by_origin_chirho: HashMap<OriginIdChirho, Vec<ReferenceEvidenceChirho>>,
}

/// Evidence-threading P2: one concrete class-instantiation fact about one
/// source reference of `name`. `span_chirho` IDENTIFIES the reference; the
/// ordinal is a per-name counter in INFERENCE-VISIT order, which is not source
/// order (instance method bodies are inferred last, in phase 3e), so it may only
/// be used as a fallback where no span is available.
/// `ty_key_chirho` is the instance-head key (e.g. "Int", "[]", "Maybe").
/// `origin_chirho` is the reference's producer-minted identity when it has one.
/// An occurrence with several predicates keeps one record per predicate, all
/// under the same origin.
/// workflow: language-features-chirho/dictionary-evidence-chirho
#[derive(Debug, Clone, PartialEq)]
pub struct MethodOccurrenceRecordChirho {
    pub name_chirho: String,
    pub ordinal_chirho: u32,
    pub class_name_chirho: String,
    pub ty_key_chirho: String,
    pub span_chirho: SpanChirho,
    pub origin_chirho: Option<OriginIdChirho>,
}

/// The inference context — carries mutable state during inference.
pub struct InferCtxChirho {
    /// Fresh type variable counter.
    next_var_chirho: u32,
    /// Type environment (scoped).
    env_chirho: TyEnvChirho,
    /// Class environment (typeclasses and instances).
    class_env_chirho: ClassEnvChirho,
    /// Deferred (wanted) typeclass predicates collected during inference.
    deferred_preds_chirho: Vec<(PredChirho, SpanChirho)>,
    /// Evidence-threading P2: raw occurrence captures — (name, per-name ordinal,
    /// class, instantiated predicate type). Finalized against the composed
    /// substitution when the public entry builds `InferResultChirho`.
    occurrence_captures_chirho: Vec<evidence_chirho::OccurrenceCaptureChirho>,
    /// Evidence-threading P2: default keys learned when generalized predicates
    /// carry additional numeric-class evidence for an occurrence type variable.
    occurrence_default_hints_chirho: HashMap<TyVarChirho, String>,
    /// Evidence-threading P2: variables whose predicates were generalized into
    /// a binding scheme. Unhinted occurrences on these variables must stay
    /// polymorphic so the dict pass can use the binding's dictionary argument.
    occurrence_generalized_vars_chirho: HashSet<TyVarChirho>,
    /// Evidence-threading P2: per-name reference counters (source order).
    occurrence_counters_chirho: HashMap<String, u32>,
    /// Literal evidence captures: (span, overloading class, literal type),
    /// finalized by `finalize_literal_evidence_chirho`.
    literal_captures_chirho: Vec<(SpanChirho, Option<OriginIdChirho>, String, TyChirho)>,
    /// Reference evidence captures: (span, instantiated predicates in scheme
    /// order), finalized by `finalize_reference_evidence_chirho`.
    reference_captures_chirho: Vec<evidence_chirho::ReferenceCaptureChirho>,
    recursive_references_chirho: evidence_chirho::RecursiveReferencesChirho,
    /// (class, skolem name) → index of that predicate in the signature the
    /// skolem was made for; see `record_own_dictionary_indices_chirho`.
    skolem_pred_index_chirho: HashMap<(String, String), usize>,
    /// Variables quantified by a local (`let`/`where`) binding's generalization.
    /// The dictionary pass gives such a binding no dictionary parameter, so an
    /// occurrence at one of these is served by the type the enclosing code
    /// instantiates the binding at; see `local_specialization_key_chirho`.
    local_generalized_vars_chirho: HashSet<TyVarChirho>,
    /// A local binding's quantified variable → the fresh variable of each
    /// instantiation of that binding.
    local_instantiations_chirho: HashMap<TyVarChirho, Vec<TyVarChirho>>,
    /// Given predicates currently in scope from explicit type signatures.
    given_preds_chirho: Vec<PredChirho>,
    /// Accumulated diagnostics.
    diagnostics_chirho: DiagnosticBundleChirho,
    /// Type synonym environment: name → (param names, expanded RHS TyChirho).
    /// Populated from `TypeAliasDeclChirho` declarations before inference.
    type_synonyms_chirho: HashMap<String, (Vec<String>, TyChirho)>,
    /// Type family environment: family name → list of equations (lhs patterns, rhs type).
    /// Each equation is (param type patterns as TyChirho, result TyChirho).
    type_families_chirho: TypeFamilyEnvChirho,
    /// ScopedTypeVariables: when inside a function body whose type signature has
    /// `forall a b.`, maps those names to their TyVarChirho so that where-clause
    /// type annotations and local signatures use the same type variables.
    scoped_tyvars_chirho: HashMap<String, TyVarChirho>,
    /// Next ordinal for a rigid signature variable (see `skolem_chirho`).
    next_skolem_chirho: u32,
    /// Signature variables already made rigid, `var ↦ skolem`. Consulted when a
    /// scoped type variable of an enclosing signature reappears in an inner
    /// signature or annotation, so both name the same rigid constant.
    skolem_bindings_chirho: SubstChirho,
    /// Skolems made while checking an instance method. They prove the method
    /// against its type; they are not the enclosing binding's own dictionary
    /// parameters, so evidence keys them as open, as the flexible variables
    /// they replaced were (`own_parameter_key_chirho`).
    instance_method_skolems_chirho: HashSet<String>,
    /// Whether ScopedTypeVariables is in effect for the module: an explicit
    /// `forall` (or an instance head) then scopes its binders over the body.
    scoped_type_variables_chirho: bool,
    /// RelaxedPolyRec (Haskell 2010 and GHC2021, not Haskell 98): a reference
    /// to a binding with a type signature is not a dependency edge.
    relaxed_poly_rec_chirho: bool,
    /// Expected type handed to the last statement of the `do` block being
    /// inferred next, so a rigid result reaches a final `case` and its
    /// alternatives can meet it through their own GADT refinements.
    do_last_expected_chirho: Option<TyChirho>,
    /// Associated type family defaults, keyed by class name.
    assoc_type_defaults_chirho: HashMap<String, Vec<AssocTypeDefaultChirho>>,
    /// The parameter names each associated family was declared with.
    assoc_type_declared_params_chirho: HashMap<String, Vec<String>>,
    /// Given equalities in scope, as rewrite rules `lhs ↦ rhs` applied during
    /// normalization (`F a ~ Bool =>` rewrites `F a` to `Bool`). Truncated with
    /// the givens they came from.
    /// workflow: language-features-chirho/stuck-family-equalities-chirho
    given_rewrites_chirho: Vec<(TyChirho, TyChirho)>,
    /// Wanted equalities that could not be decided yet because a type-family
    /// application on one side is stuck on an unsolved variable; retried when
    /// givens change and at the end of the module.
    deferred_equalities_chirho: Vec<(TyChirho, TyChirho, SpanChirho)>,
    /// Source spelling of type variables introduced from signatures, so
    /// skolems and diagnostics carry the programmer's names.
    tyvar_source_names_chirho: HashMap<TyVarChirho, String>,
    /// Data type name → every constructor this module declares for it (data and
    /// newtype declarations). Complete for local types only; an imported type
    /// is absent, and the record-update check stays silent for it.
    /// workflow: language-features-chirho/rigid-type-variables-chirho (records)
    data_constructors_chirho: HashMap<String, Vec<String>>,
    /// Instances declared by the module under inference, with their spans,
    /// for the declaration-time obligations (duplicates, superclasses).
    local_instances_chirho: Vec<instance_obligations_chirho::LocalInstanceChirho>,
    /// Positional constructors of this module with at least one strict
    /// argument: `C {}` on one omits a strict field (E0207, tcfail112).
    strict_positional_constructors_chirho: HashSet<String>,
    /// Record constructor → its strict field names (`f :: !Int`). A construction
    /// that omits one of these is an error, not a lazy thunk.
    con_strict_fields_chirho: HashMap<String, Vec<String>>,
    /// Record constructor field names: maps constructor name → ordered list of field names.
    /// Used for RecordWildCards expansion (`Foo{..}` fills in missing fields).
    con_field_names_chirho: HashMap<String, Vec<String>>,
    /// Module-local numeric default declaration candidates from `default (...)`.
    module_default_types_chirho: Vec<TyChirho>,
    /// Whether the current module enables OverloadedStrings.
    overloaded_strings_chirho: bool,
    /// Classes whose full instance universe is visible: the standard-seeded
    /// classes (captured before any imported class env replaces the seed) plus
    /// classes declared in the module being inferred. An imported module can
    /// never declare an instance of a class it cannot see, so for these — and
    /// only these — "no potentially matching instance" is a sound rejection.
    fully_known_class_names_chirho: HashSet<String>,
    /// Whether the class constraints we generate for this module faithfully
    /// reflect the source. Cleared by extensions whose constraints we are
    /// known to mis-generate, so no unsolvable verdict is reached on a wanted
    /// that our own inference invented. See `UNFAITHFUL_CONSTRAINT_EXTENSIONS_CHIRHO`.
    constraint_generation_faithful_chirho: bool,
    /// (class, type-head) pairs mentioned by any deriving syntax in the module
    /// (deriving clauses, standalone deriving = ("C", "*"), DerivingVia).
    /// Deriving is not modeled as real class-env instances yet, so the
    /// certainly-unsolvable check treats these as potential instances.
    derived_instance_heads_chirho: HashSet<(String, String)>,
    /// Imported type constructors that can safely stay unqualified.
    safe_unqualified_imported_type_names_chirho: HashSet<String>,
    /// Canonical module qualifiers for imported type constructors.
    preferred_qualified_type_names_chirho: HashMap<String, String>,
}

impl InferCtxChirho {
    pub fn new_chirho() -> Self {
        let mut env_chirho = TyEnvChirho::new_chirho();
        let mut class_env_chirho = ClassEnvChirho::new_chirho();
        class_env_chirho.seed_standard_chirho();
        let fully_known_class_names_chirho: HashSet<String> =
            class_env_chirho.classes_chirho.keys().cloned().collect();
        seed_builtins_chirho(&mut env_chirho);
        let mut type_synonyms_chirho = HashMap::new();
        // Built-in type synonym: type String = [Char]
        type_synonyms_chirho.insert(
            "String".to_string(),
            (
                vec![],
                TyChirho::ListChirho(Box::new(TyChirho::char_chirho())),
            ),
        );
        // Built-in type synonym: type ShowS = String -> String
        type_synonyms_chirho.insert(
            "ShowS".to_string(),
            (
                vec![],
                TyChirho::fun_chirho(TyChirho::string_chirho(), TyChirho::string_chirho()),
            ),
        );
        // Built-in type synonym: type ReadS a = String -> [(a, String)]
        type_synonyms_chirho.insert(
            "ReadS".to_string(),
            (
                vec!["a".to_string()],
                TyChirho::fun_chirho(
                    TyChirho::string_chirho(),
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::ForallVarChirho("a".to_string()),
                        TyChirho::string_chirho(),
                    ]))),
                ),
            ),
        );
        // GHC.TypeLits: type Nat = Natural
        type_synonyms_chirho.insert(
            "Nat".to_string(),
            (vec![], TyChirho::ConChirho("Natural".to_string())),
        );
        // GHC.Generics metadata aliases:
        // type D1 = M1 D
        // type C1 = M1 C
        // type S1 = M1 S
        for (metadata_alias_chirho, metadata_tag_chirho) in [("D1", "D"), ("C1", "C"), ("S1", "S")]
        {
            type_synonyms_chirho.insert(
                metadata_alias_chirho.to_string(),
                (
                    vec![],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("M1".to_string())),
                        Box::new(TyChirho::ConChirho(metadata_tag_chirho.to_string())),
                    ),
                ),
            );
        }
        // type FilePath = String
        type_synonyms_chirho.insert("FilePath".to_string(), (vec![], TyChirho::string_chirho()));
        // Test.Hspec frontiers: model Spec/Expectation as IO () for type-checking
        // package test suites without implementing the Hspec runner.
        let hspec_action_ty_chirho = TyChirho::io_chirho(TyChirho::unit_chirho());
        type_synonyms_chirho.insert("Spec".to_string(), (vec![], hspec_action_ty_chirho.clone()));
        type_synonyms_chirho.insert("Expectation".to_string(), (vec![], hspec_action_ty_chirho));
        // Monad transformer aliases from transformers / mtl.
        type_synonyms_chirho.insert(
            "State".to_string(),
            (
                vec!["s".to_string(), "a".to_string()],
                TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("StateT".to_string())),
                            Box::new(TyChirho::ForallVarChirho("s".to_string())),
                        )),
                        Box::new(TyChirho::ConChirho("Identity".to_string())),
                    )),
                    Box::new(TyChirho::ForallVarChirho("a".to_string())),
                ),
            ),
        );
        // Template Haskell aliases
        type_synonyms_chirho.insert(
            "Kind".to_string(),
            (vec![], TyChirho::ConChirho("Type".to_string())),
        );
        type_synonyms_chirho.insert(
            "Pred".to_string(),
            (vec![], TyChirho::ConChirho("Type".to_string())),
        );
        type_synonyms_chirho.insert(
            "Cxt".to_string(),
            (
                vec![],
                TyChirho::ListChirho(Box::new(TyChirho::ConChirho("Pred".to_string()))),
            ),
        );
        type_synonyms_chirho.insert(
            "TypeQ".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Q".to_string())),
                    Box::new(TyChirho::ConChirho("Type".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "ExpQ".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Q".to_string())),
                    Box::new(TyChirho::ConChirho("Exp".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "PatQ".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Q".to_string())),
                    Box::new(TyChirho::ConChirho("Pat".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "DecQ".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Q".to_string())),
                    Box::new(TyChirho::ConChirho("Dec".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "DecsQ".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Q".to_string())),
                    Box::new(TyChirho::ListChirho(Box::new(TyChirho::ConChirho(
                        "Dec".to_string(),
                    )))),
                ),
            ),
        );
        // Control.Monad.Signatures
        type_synonyms_chirho.insert(
            "CallCC".to_string(),
            (
                vec!["m".to_string(), "a".to_string(), "b".to_string()],
                TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::ForallVarChirho("a".to_string()),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::ForallVarChirho("m".to_string())),
                                Box::new(TyChirho::ForallVarChirho("b".to_string())),
                            ),
                        ),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::ForallVarChirho("m".to_string())),
                            Box::new(TyChirho::ForallVarChirho("a".to_string())),
                        ),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ForallVarChirho("m".to_string())),
                        Box::new(TyChirho::ForallVarChirho("a".to_string())),
                    ),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "Catch".to_string(),
            (
                vec!["e".to_string(), "m".to_string(), "a".to_string()],
                TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ForallVarChirho("m".to_string())),
                        Box::new(TyChirho::ForallVarChirho("a".to_string())),
                    ),
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::ForallVarChirho("e".to_string()),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::ForallVarChirho("m".to_string())),
                                Box::new(TyChirho::ForallVarChirho("a".to_string())),
                            ),
                        ),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::ForallVarChirho("m".to_string())),
                            Box::new(TyChirho::ForallVarChirho("a".to_string())),
                        ),
                    ),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "Listen".to_string(),
            (
                vec!["w".to_string(), "m".to_string(), "a".to_string()],
                TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ForallVarChirho("m".to_string())),
                        Box::new(TyChirho::ForallVarChirho("a".to_string())),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ForallVarChirho("m".to_string())),
                        Box::new(TyChirho::TupleChirho(vec![
                            TyChirho::ForallVarChirho("a".to_string()),
                            TyChirho::ForallVarChirho("w".to_string()),
                        ])),
                    ),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "Pass".to_string(),
            (
                vec!["w".to_string(), "m".to_string(), "a".to_string()],
                TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ForallVarChirho("m".to_string())),
                        Box::new(TyChirho::TupleChirho(vec![
                            TyChirho::ForallVarChirho("a".to_string()),
                            TyChirho::fun_chirho(
                                TyChirho::ForallVarChirho("w".to_string()),
                                TyChirho::ForallVarChirho("w".to_string()),
                            ),
                        ])),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ForallVarChirho("m".to_string())),
                        Box::new(TyChirho::ForallVarChirho("a".to_string())),
                    ),
                ),
            ),
        );
        // Built-in type synonym: type Buffer = Ptr Word8
        type_synonyms_chirho.insert(
            "Buffer".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Ptr".to_string())),
                    Box::new(TyChirho::ConChirho("Word8".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "CString".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Ptr".to_string())),
                    Box::new(TyChirho::ConChirho("CChar".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "CStringLen".to_string(),
            (
                vec![],
                TyChirho::TupleChirho(vec![
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Ptr".to_string())),
                        Box::new(TyChirho::ConChirho("CChar".to_string())),
                    ),
                    TyChirho::int_chirho(),
                ]),
            ),
        );
        // Built-in type synonym: type CI a = a
        type_synonyms_chirho.insert(
            "CI".to_string(),
            (
                vec!["a".to_string()],
                TyChirho::ForallVarChirho("a".to_string()),
            ),
        );
        // BufSize = Int (from Network.Socket.BufferPool)
        type_synonyms_chirho.insert("BufSize".to_string(), (vec![], TyChirho::int_chirho()));
        // Built-in http-types aliases
        type_synonyms_chirho.insert(
            "HeaderName".to_string(),
            (
                vec![],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("CI".to_string())),
                    Box::new(TyChirho::ConChirho("ByteString".to_string())),
                ),
            ),
        );
        type_synonyms_chirho.insert(
            "Header".to_string(),
            (
                vec![],
                TyChirho::TupleChirho(vec![
                    TyChirho::ConChirho("HeaderName".to_string()),
                    TyChirho::ConChirho("ByteString".to_string()),
                ]),
            ),
        );
        type_synonyms_chirho.insert(
            "ResponseHeaders".to_string(),
            (
                vec![],
                TyChirho::ListChirho(Box::new(TyChirho::ConChirho("Header".to_string()))),
            ),
        );
        Self {
            next_var_chirho: 0,
            env_chirho,
            class_env_chirho,
            deferred_preds_chirho: Vec::new(),
            occurrence_captures_chirho: Vec::new(),
            occurrence_default_hints_chirho: HashMap::new(),
            occurrence_generalized_vars_chirho: HashSet::new(),
            occurrence_counters_chirho: HashMap::new(),
            literal_captures_chirho: Vec::new(),
            reference_captures_chirho: Vec::new(),
            recursive_references_chirho: Default::default(),
            skolem_pred_index_chirho: HashMap::new(),
            local_generalized_vars_chirho: HashSet::new(),
            local_instantiations_chirho: HashMap::new(),
            given_preds_chirho: Vec::new(),
            diagnostics_chirho: DiagnosticBundleChirho::empty_chirho(),
            type_synonyms_chirho,
            type_families_chirho: HashMap::new(),
            scoped_tyvars_chirho: HashMap::new(),
            next_skolem_chirho: 0,
            skolem_bindings_chirho: SubstChirho::empty_chirho(),
            instance_method_skolems_chirho: HashSet::new(),
            scoped_type_variables_chirho: true,
            relaxed_poly_rec_chirho: true,
            do_last_expected_chirho: None,
            given_rewrites_chirho: Vec::new(),
            deferred_equalities_chirho: Vec::new(),
            assoc_type_defaults_chirho: HashMap::new(),
            assoc_type_declared_params_chirho: HashMap::new(),
            tyvar_source_names_chirho: HashMap::new(),
            data_constructors_chirho: HashMap::new(),
            local_instances_chirho: Vec::new(),
            strict_positional_constructors_chirho: HashSet::new(),
            con_strict_fields_chirho: HashMap::new(),
            con_field_names_chirho: HashMap::new(),
            module_default_types_chirho: Vec::new(),
            overloaded_strings_chirho: false,
            fully_known_class_names_chirho,
            constraint_generation_faithful_chirho: true,
            derived_instance_heads_chirho: HashSet::new(),
            safe_unqualified_imported_type_names_chirho: HashSet::new(),
            preferred_qualified_type_names_chirho: HashMap::new(),
        }
    }

    pub fn new_with_imported_class_env_chirho(imported_class_env_chirho: &ClassEnvChirho) -> Self {
        let mut ctx_chirho = Self::new_chirho();
        if !imported_class_env_chirho.classes_chirho.is_empty()
            || !imported_class_env_chirho.instances_chirho.is_empty()
        {
            ctx_chirho.class_env_chirho = imported_class_env_chirho.clone();
            // The seeded env was replaced wholesale; the seeded classes'
            // instance inventories are no longer trustworthy, and imported
            // classes may have instances in modules we cannot see. Only
            // classes declared in THIS module stay fully known.
            ctx_chirho.fully_known_class_names_chirho.clear();
        }
        ctx_chirho
    }

    fn set_imported_type_name_preferences_chirho(
        &mut self,
        safe_unqualified_imported_type_names_chirho: &HashSet<String>,
        preferred_qualified_type_names_chirho: &HashMap<String, String>,
    ) {
        self.safe_unqualified_imported_type_names_chirho =
            safe_unqualified_imported_type_names_chirho.clone();
        self.preferred_qualified_type_names_chirho = preferred_qualified_type_names_chirho.clone();
    }

    fn normalize_imported_type_name_chirho(&self, name_text_chirho: &str) -> String {
        let bare_name_chirho = strip_name_qualifier_chirho(name_text_chirho);
        if self
            .safe_unqualified_imported_type_names_chirho
            .contains(bare_name_chirho)
            || self
                .preferred_qualified_type_names_chirho
                .contains_key(bare_name_chirho)
        {
            bare_name_chirho.to_string()
        } else {
            name_text_chirho.to_string()
        }
    }

    fn normalize_imported_ty_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => {
                TyChirho::ConChirho(self.normalize_imported_type_name_chirho(name_chirho))
            }
            TyChirho::AppChirho(fun_chirho, arg_chirho) => TyChirho::AppChirho(
                Box::new(self.normalize_imported_ty_chirho(fun_chirho)),
                Box::new(self.normalize_imported_ty_chirho(arg_chirho)),
            ),
            TyChirho::FunChirho(arg_chirho, result_chirho, mult_chirho) => TyChirho::FunChirho(
                Box::new(self.normalize_imported_ty_chirho(arg_chirho)),
                Box::new(self.normalize_imported_ty_chirho(result_chirho)),
                *mult_chirho,
            ),
            TyChirho::TupleChirho(elements_chirho) => TyChirho::TupleChirho(
                elements_chirho
                    .iter()
                    .map(|element_chirho| self.normalize_imported_ty_chirho(element_chirho))
                    .collect(),
            ),
            TyChirho::ListChirho(inner_chirho) => {
                TyChirho::ListChirho(Box::new(self.normalize_imported_ty_chirho(inner_chirho)))
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::ForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.normalize_imported_ty_chirho(body_chirho)),
            },
            TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.normalize_imported_ty_chirho(body_chirho)),
            },
            _ => ty_chirho.clone(),
        }
    }

    /// Register a type family (open or closed). For open families this
    /// creates an empty equation list; for closed families it stores all
    /// equations immediately.
    pub fn register_type_family_chirho(
        &mut self,
        name_chirho: String,
        equations_chirho: Vec<(Vec<TyChirho>, TyChirho)>,
    ) {
        let normalized_equations_chirho: Vec<(Vec<TyChirho>, TyChirho)> = equations_chirho
            .into_iter()
            .map(|(lhs_types_chirho, rhs_chirho)| {
                (
                    lhs_types_chirho
                        .into_iter()
                        .map(|lhs_ty_chirho| self.normalize_imported_ty_chirho(&lhs_ty_chirho))
                        .collect(),
                    self.normalize_imported_ty_chirho(&rhs_chirho),
                )
            })
            .collect();
        let registered_equations_chirho = self.type_families_chirho.entry(name_chirho).or_default();
        if normalized_equations_chirho.is_empty() {
            return;
        }

        // Newer registrations should win over older imported equations, but
        // preserve the source order within each registration batch.
        let prior_equations_chirho = std::mem::take(registered_equations_chirho);
        registered_equations_chirho.extend(normalized_equations_chirho);
        registered_equations_chirho.extend(prior_equations_chirho);
    }

    /// Add an equation to an open type family from a `type instance` decl.
    pub fn register_type_family_instance_chirho(
        &mut self,
        family_name_chirho: String,
        lhs_types_chirho: Vec<TyChirho>,
        rhs_chirho: TyChirho,
    ) {
        let normalized_lhs_types_chirho: Vec<TyChirho> = lhs_types_chirho
            .iter()
            .map(|lhs_ty_chirho| self.normalize_imported_ty_chirho(lhs_ty_chirho))
            .collect();
        let normalized_rhs_chirho = self.normalize_imported_ty_chirho(&rhs_chirho);
        self.type_families_chirho
            .entry(family_name_chirho)
            .or_default()
            .insert(0, (normalized_lhs_types_chirho, normalized_rhs_chirho));
    }

    /// Try to reduce a type family application `F args...` by matching
    /// against registered equations. Returns `Some(reduced)` if a match
    /// is found, `None` otherwise (stuck family application).
    pub fn reduce_type_family_chirho(
        &self,
        family_name_chirho: &str,
        args_chirho: &[TyChirho],
    ) -> Option<TyChirho> {
        self.reduce_type_family_application_chirho(family_name_chirho, args_chirho)
    }

    fn reduce_type_family_application_chirho(
        &self,
        family_name_chirho: &str,
        args_chirho: &[TyChirho],
    ) -> Option<TyChirho> {
        let equation_sets_chirho = self.lookup_type_family_equation_sets_chirho(family_name_chirho);
        for equations_chirho in equation_sets_chirho {
            for (lhs_chirho, rhs_chirho) in equations_chirho {
                if lhs_chirho.len() > args_chirho.len() {
                    continue;
                }
                // Try to match each LHS pattern against the corresponding arg
                let mut bindings_chirho: HashMap<String, TyChirho> = HashMap::new();
                let mut matched_chirho = true;
                for (pat_chirho, arg_chirho) in lhs_chirho.iter().zip(args_chirho.iter()) {
                    if !match_type_pattern_chirho(pat_chirho, arg_chirho, &mut bindings_chirho) {
                        matched_chirho = false;
                        break;
                    }
                }
                if matched_chirho {
                    let mut reduced_chirho =
                        substitute_type_vars_chirho(rhs_chirho, &bindings_chirho);
                    for extra_arg_chirho in &args_chirho[lhs_chirho.len()..] {
                        reduced_chirho = TyChirho::AppChirho(
                            Box::new(reduced_chirho),
                            Box::new(extra_arg_chirho.clone()),
                        );
                    }
                    return Some(reduced_chirho);
                }
            }
        }
        reduce_builtin_type_family_application_chirho(family_name_chirho, args_chirho)
    }

    fn lookup_type_family_equation_sets_chirho(
        &self,
        family_name_chirho: &str,
    ) -> Vec<&[(Vec<TyChirho>, TyChirho)]> {
        let bare_name_chirho = family_name_chirho
            .rsplit_once('.')
            .map(|(_prefix_chirho, bare_name_chirho)| bare_name_chirho)
            .unwrap_or(family_name_chirho);
        let mut seen_names_chirho: HashSet<String> = HashSet::new();
        let mut equation_sets_chirho: Vec<&[(Vec<TyChirho>, TyChirho)]> = Vec::new();

        // Prefer the bare family name so local equations can win over empty
        // imported qualified declarations for the same family.
        if seen_names_chirho.insert(bare_name_chirho.to_string()) {
            if let Some(equations_chirho) = self.type_families_chirho.get(bare_name_chirho) {
                equation_sets_chirho.push(equations_chirho.as_slice());
            }
        }
        if seen_names_chirho.insert(family_name_chirho.to_string()) {
            if let Some(equations_chirho) = self.type_families_chirho.get(family_name_chirho) {
                equation_sets_chirho.push(equations_chirho.as_slice());
            }
        }

        let suffix_chirho = format!(".{bare_name_chirho}");
        let suffix_match_names_chirho: Vec<String> = self
            .type_families_chirho
            .keys()
            .filter(|name_chirho| {
                name_chirho.ends_with(&suffix_chirho)
                    && !seen_names_chirho.contains(name_chirho.as_str())
            })
            .cloned()
            .collect();
        if suffix_match_names_chirho.len() == 1 {
            let suffix_name_chirho = &suffix_match_names_chirho[0];
            if seen_names_chirho.insert(suffix_name_chirho.clone()) {
                if let Some(equations_chirho) = self.type_families_chirho.get(suffix_name_chirho) {
                    equation_sets_chirho.push(equations_chirho.as_slice());
                }
            }
        }

        equation_sets_chirho
    }

    /// Register a type synonym from a `TypeAliasDeclChirho`.
    pub fn register_type_synonym_chirho(
        &mut self,
        name_chirho: String,
        params_chirho: Vec<String>,
        rhs_chirho: TyChirho,
    ) {
        let normalized_rhs_chirho = self.normalize_imported_ty_chirho(&rhs_chirho);
        self.type_synonyms_chirho
            .insert(name_chirho, (params_chirho, normalized_rhs_chirho));
    }

    /// Expand type synonyms in a `TyChirho`. Handles both nullary synonyms
    /// (e.g. `String` → `[Char]`) and parameterised synonyms (e.g.
    /// `Pair Int` → `(Int, Int)` for `type Pair a = (a, a)`).
    pub fn expand_type_synonyms_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        self.expand_syn_chirho(ty_chirho, 0)
    }

    fn lookup_type_synonym_chirho(&self, name_chirho: &str) -> Option<&(Vec<String>, TyChirho)> {
        self.type_synonyms_chirho.get(name_chirho).or_else(|| {
            name_chirho
                .rsplit_once('.')
                .and_then(|(_prefix_chirho, bare_name_chirho)| {
                    self.type_synonyms_chirho.get(bare_name_chirho)
                })
        })
    }

    fn expand_syn_chirho(&self, ty_chirho: &TyChirho, depth_chirho: usize) -> TyChirho {
        if depth_chirho > 100 {
            return ty_chirho.clone(); // guard against cycles
        }
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => {
                if let Some((params_chirho, rhs_chirho)) =
                    self.lookup_type_synonym_chirho(name_chirho)
                {
                    if params_chirho.is_empty() {
                        // Nullary synonym — expand and recurse
                        return self.expand_syn_chirho(rhs_chirho, depth_chirho + 1);
                    }
                }
                ty_chirho.clone()
            }
            TyChirho::AppChirho(fun_chirho, arg_chirho) => {
                // Collect the spine: f a1 a2 ... an
                let (head_chirho, args_chirho) = collect_app_spine_chirho(ty_chirho);
                if let TyChirho::ConChirho(name_chirho) = &head_chirho {
                    if let Some((params_chirho, rhs_chirho)) =
                        self.lookup_type_synonym_chirho(name_chirho)
                    {
                        if args_chirho.len() >= params_chirho.len() {
                            // Saturated application — substitute params
                            let expanded_args_chirho: Vec<TyChirho> = args_chirho
                                .iter()
                                .map(|a_chirho| self.expand_syn_chirho(a_chirho, depth_chirho + 1))
                                .collect();
                            let mut body_chirho = rhs_chirho.clone();
                            for (p_chirho, a_chirho) in
                                params_chirho.iter().zip(expanded_args_chirho.iter())
                            {
                                body_chirho =
                                    subst_named_var_chirho(&body_chirho, p_chirho, a_chirho);
                            }
                            // Apply remaining args (over-saturated)
                            let mut result_chirho =
                                self.expand_syn_chirho(&body_chirho, depth_chirho + 1);
                            for a_chirho in &expanded_args_chirho[params_chirho.len()..] {
                                result_chirho = TyChirho::AppChirho(
                                    Box::new(result_chirho),
                                    Box::new(a_chirho.clone()),
                                );
                            }
                            return result_chirho;
                        }
                    }
                }
                // Not a synonym application — just expand sub-parts
                let ef_chirho = self.expand_syn_chirho(fun_chirho, depth_chirho);
                let ea_chirho = self.expand_syn_chirho(arg_chirho, depth_chirho);
                TyChirho::AppChirho(Box::new(ef_chirho), Box::new(ea_chirho))
            }
            TyChirho::FunChirho(a_chirho, b_chirho, _) => TyChirho::FunChirho(
                Box::new(self.expand_syn_chirho(a_chirho, depth_chirho)),
                Box::new(self.expand_syn_chirho(b_chirho, depth_chirho)),
                MultChirho::ManyChirho,
            ),
            TyChirho::ListChirho(el_chirho) => {
                TyChirho::ListChirho(Box::new(self.expand_syn_chirho(el_chirho, depth_chirho)))
            }
            TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
                elems_chirho
                    .iter()
                    .map(|e_chirho| self.expand_syn_chirho(e_chirho, depth_chirho))
                    .collect(),
            ),
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::ForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.expand_syn_chirho(body_chirho, depth_chirho)),
            },
            TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.expand_syn_chirho(body_chirho, depth_chirho)),
            },
            _ => ty_chirho.clone(),
        }
    }

    /// Reduce type family applications in a `TyChirho`. Walks the type and
    /// replaces saturated type family applications with their reduced form.
    /// For example, `F Int` where `type instance F Int = Bool` becomes `Bool`.
    pub fn reduce_type_families_in_ty_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        self.reduce_families_chirho(ty_chirho, 0)
    }

    /// Normalize a type to a fixed point by alternating synonym expansion and
    /// type-family reduction. This catches cases where family reduction
    /// exposes a synonym head such as `Rep f` -> `GRep f`.
    pub fn normalize_ty_chirho(&self, ty_chirho: &TyChirho) -> TyChirho {
        let mut current_ty_chirho = ty_chirho.clone();
        for _iteration_chirho in 0..100 {
            let expanded_ty_chirho = self.expand_type_synonyms_chirho(&current_ty_chirho);
            let reduced_ty_chirho = self.reduce_type_families_in_ty_chirho(&expanded_ty_chirho);
            let normalized_ty_chirho = self.expand_type_synonyms_chirho(&reduced_ty_chirho);
            // Local given equalities from GADT patterns rewrite rigid variables
            // in scope. workflow: language-features-chirho/rigid-type-variables-chirho
            let normalized_ty_chirho = self
                .env_chirho
                .apply_refinements_chirho(&normalized_ty_chirho);
            let normalized_ty_chirho = self.apply_given_rewrites_chirho(&normalized_ty_chirho);
            if normalized_ty_chirho == current_ty_chirho {
                return normalized_ty_chirho;
            }
            current_ty_chirho = normalized_ty_chirho;
        }
        current_ty_chirho
    }

    fn saturated_st_inner_ty_chirho(&self, ty_chirho: &TyChirho) -> Option<TyChirho> {
        if let TyChirho::AppChirho(_, _) = ty_chirho {
            let (head_chirho, args_chirho) = collect_app_spine_chirho(ty_chirho);
            if matches!(&head_chirho, TyChirho::ConChirho(name_chirho) if name_chirho == "ST")
                && args_chirho.len() == 2
            {
                return Some(args_chirho[1].clone());
            }
        }
        None
    }

    fn reduce_families_chirho(&self, ty_chirho: &TyChirho, depth_chirho: usize) -> TyChirho {
        if depth_chirho > 100 {
            return ty_chirho.clone();
        }
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => {
                // Nullary type family (no arguments)
                if let Some(reduced_chirho) = self.reduce_type_family_chirho(name_chirho, &[]) {
                    return self.reduce_families_chirho(&reduced_chirho, depth_chirho + 1);
                }
                ty_chirho.clone()
            }
            TyChirho::AppChirho(_, _) => {
                // Collect spine and check if head is a type family
                let (head_chirho, args_chirho) = collect_app_spine_chirho(ty_chirho);
                if let TyChirho::ConChirho(name_chirho) = &head_chirho {
                    // Recursively reduce arguments first
                    let reduced_args_chirho: Vec<TyChirho> = args_chirho
                        .iter()
                        .map(|a_chirho| self.reduce_families_chirho(a_chirho, depth_chirho + 1))
                        .collect();
                    if let Some(result_chirho) = self
                        .reduce_type_family_application_chirho(name_chirho, &reduced_args_chirho)
                    {
                        return self.reduce_families_chirho(&result_chirho, depth_chirho + 1);
                    }
                    // Not a family — rebuild with reduced args
                    let mut result_chirho = head_chirho.clone();
                    for a_chirho in &reduced_args_chirho {
                        result_chirho = TyChirho::AppChirho(
                            Box::new(result_chirho),
                            Box::new(a_chirho.clone()),
                        );
                    }
                    return result_chirho;
                }
                // Head is not a Con — just reduce sub-parts
                let ef_chirho = self.reduce_families_chirho(
                    match ty_chirho {
                        TyChirho::AppChirho(f, _) => f,
                        _ => unreachable!(),
                    },
                    depth_chirho,
                );
                let ea_chirho = self.reduce_families_chirho(
                    match ty_chirho {
                        TyChirho::AppChirho(_, a) => a,
                        _ => unreachable!(),
                    },
                    depth_chirho,
                );
                TyChirho::AppChirho(Box::new(ef_chirho), Box::new(ea_chirho))
            }
            TyChirho::FunChirho(a_chirho, b_chirho, m_chirho) => TyChirho::FunChirho(
                Box::new(self.reduce_families_chirho(a_chirho, depth_chirho)),
                Box::new(self.reduce_families_chirho(b_chirho, depth_chirho)),
                *m_chirho,
            ),
            TyChirho::ListChirho(el_chirho) => TyChirho::ListChirho(Box::new(
                self.reduce_families_chirho(el_chirho, depth_chirho),
            )),
            TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
                elems_chirho
                    .iter()
                    .map(|e_chirho| self.reduce_families_chirho(e_chirho, depth_chirho))
                    .collect(),
            ),
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::ForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.reduce_families_chirho(body_chirho, depth_chirho)),
            },
            TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.reduce_families_chirho(body_chirho, depth_chirho)),
            },
            _ => ty_chirho.clone(),
        }
    }

    /// Generate a fresh unification variable.
    pub fn fresh_var_chirho(&mut self) -> TyChirho {
        let var_chirho = TyVarChirho(self.next_var_chirho);
        self.next_var_chirho += 1;
        TyChirho::VarChirho(var_chirho)
    }

    fn instantiate_scheme_parts_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
    ) -> (TyChirho, Vec<PredChirho>) {
        if scheme_chirho.vars_chirho.is_empty() && scheme_chirho.preds_chirho.is_empty() {
            return (scheme_chirho.ty_chirho.clone(), Vec::new());
        }

        let mut subst_chirho = SubstChirho::empty_chirho();
        for v_chirho in &scheme_chirho.vars_chirho {
            let fresh_chirho = self.fresh_var_chirho();
            if self.local_generalized_vars_chirho.contains(v_chirho) {
                if let TyChirho::VarChirho(fresh_var_chirho) = fresh_chirho {
                    self.local_instantiations_chirho
                        .entry(*v_chirho)
                        .or_default()
                        .push(fresh_var_chirho);
                }
            }
            subst_chirho.insert_chirho(*v_chirho, fresh_chirho);
        }

        let instantiated_preds_chirho: Vec<PredChirho> = scheme_chirho
            .preds_chirho
            .iter()
            .map(|pred_chirho| {
                let instantiated_ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
                let instantiated_extra_chirho: Vec<TyChirho> = pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
                    .collect();
                let mut pred_inst_chirho =
                    PredChirho::new_chirho(&pred_chirho.class_name_chirho, instantiated_ty_chirho);
                pred_inst_chirho.extra_tys_chirho = instantiated_extra_chirho;
                pred_inst_chirho
            })
            .collect();

        (
            subst_chirho.apply_ty_chirho(&scheme_chirho.ty_chirho),
            instantiated_preds_chirho,
        )
    }

    fn instantiate_scheme_parts_with_type_args_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
        explicit_tys_chirho: &[TyChirho],
    ) -> Option<(TyChirho, Vec<PredChirho>)> {
        let scheme_var_count_chirho = scheme_chirho.vars_chirho.len();
        let nested_arg_count_chirho = explicit_tys_chirho
            .len()
            .saturating_sub(scheme_var_count_chirho);
        let (nested_vars_chirho, nested_body_chirho) = if nested_arg_count_chirho > 0 {
            let (vars_chirho, body_chirho) =
                Self::peel_forall_chirho(scheme_chirho.ty_chirho.clone());
            if nested_arg_count_chirho > vars_chirho.len() {
                return None;
            }
            (vars_chirho, Some(body_chirho))
        } else {
            (Vec::new(), None)
        };

        let mut subst_chirho = SubstChirho::empty_chirho();
        for (index_chirho, v_chirho) in scheme_chirho.vars_chirho.iter().enumerate() {
            let inst_ty_chirho = explicit_tys_chirho
                .get(index_chirho)
                .cloned()
                .unwrap_or_else(|| self.fresh_var_chirho());
            subst_chirho.insert_chirho(*v_chirho, inst_ty_chirho);
        }
        for (index_chirho, v_chirho) in nested_vars_chirho.iter().enumerate() {
            let inst_ty_chirho = explicit_tys_chirho
                .get(scheme_var_count_chirho + index_chirho)
                .cloned()
                .unwrap_or_else(|| self.fresh_var_chirho());
            subst_chirho.insert_chirho(*v_chirho, inst_ty_chirho);
        }

        let instantiated_preds_chirho: Vec<PredChirho> = scheme_chirho
            .preds_chirho
            .iter()
            .map(|pred_chirho| {
                let instantiated_ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
                let instantiated_extra_chirho: Vec<TyChirho> = pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
                    .collect();
                let mut pred_inst_chirho =
                    PredChirho::new_chirho(&pred_chirho.class_name_chirho, instantiated_ty_chirho);
                pred_inst_chirho.extra_tys_chirho = instantiated_extra_chirho;
                pred_inst_chirho
            })
            .collect();

        // A rank-N parameter keeps its quantifier inside `TyChirho`, rather
        // than lifting it into the local scheme. Visible type application may
        // consume that quantifier only while it is outermost; peeling here
        // cannot skip a function arrow or enter a supplied polymorphic type.
        // workflow: language-features-chirho/rank-n-visible-type-application-chirho
        let result_ty_chirho = nested_body_chirho
            .as_ref()
            .unwrap_or(&scheme_chirho.ty_chirho);
        Some((
            subst_chirho.apply_ty_chirho(result_ty_chirho),
            instantiated_preds_chirho,
        ))
    }

    /// Instantiate a type scheme with fresh unification variables.
    /// Any predicates on the scheme are also instantiated and deferred.
    pub fn instantiate_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
        span_chirho: SpanChirho,
    ) -> TyChirho {
        let (instantiated_ty_chirho, instantiated_preds_chirho) =
            self.instantiate_scheme_parts_chirho(scheme_chirho);
        for pred_inst_chirho in instantiated_preds_chirho {
            self.defer_pred_chirho(pred_inst_chirho, span_chirho);
        }
        instantiated_ty_chirho
    }

    /// Defer a wanted predicate: an equality (`~`) becomes a deferred
    /// equality to be unified once its stuck side is known, anything else a
    /// deferred class constraint. Every instantiation site goes through here.
    /// workflow: language-features-chirho/stuck-family-equalities-chirho
    fn defer_pred_chirho(&mut self, pred_chirho: PredChirho, span_chirho: SpanChirho) {
        if pred_chirho.class_name_chirho == "~" {
            if let Some(rhs_chirho) = pred_chirho.extra_tys_chirho.first() {
                self.deferred_equalities_chirho.push((
                    pred_chirho.ty_chirho.clone(),
                    rhs_chirho.clone(),
                    span_chirho,
                ));
            }
            return;
        }
        self.deferred_preds_chirho.push((pred_chirho, span_chirho));
    }

    fn with_given_preds_chirho<TChirho, FChirho>(
        &mut self,
        preds_chirho: Vec<PredChirho>,
        f_chirho: FChirho,
    ) -> TChirho
    where
        FChirho: FnOnce(&mut Self) -> TChirho,
    {
        let given_start_chirho = self.given_preds_chirho.len();
        let rewrites_start_chirho = self.given_rewrites_chirho.len();
        self.install_given_equalities_chirho(&preds_chirho);
        self.given_preds_chirho.extend(preds_chirho);
        let result_chirho = f_chirho(self);
        let _improvement_chirho = self.discharge_deferred_with_current_givens_chirho();
        let _equalities_chirho = self.retry_deferred_equalities_chirho();
        self.given_preds_chirho.truncate(given_start_chirho);
        self.given_rewrites_chirho.truncate(rewrites_start_chirho);
        result_chirho
    }

    /// Generalize a type into a scheme by quantifying over all free variables
    /// not free in the environment. Also partitions deferred predicates:
    /// predicates whose type variables are all being generalized become part
    /// of the scheme; the rest remain deferred.
    ///
    /// Deferred predicates are assumed to already have substitutions applied
    /// (via `apply_subst_all_chirho` during inference).
    pub fn generalize_chirho(&mut self, ty_chirho: &TyChirho) -> SchemeChirho {
        self.generalize_with_io_defaulting_chirho(
            ty_chirho,
            true,
            GeneralizationScopeChirho::TopLevelChirho,
        )
    }

    fn generalize_local_chirho(&mut self, ty_chirho: &TyChirho) -> SchemeChirho {
        self.generalize_with_io_defaulting_chirho(
            ty_chirho,
            false,
            GeneralizationScopeChirho::NestedChirho,
        )
    }

    fn generalize_with_io_defaulting_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        allow_io_defaulting_chirho: bool,
        scope_chirho: GeneralizationScopeChirho,
    ) -> SchemeChirho {
        let mut env_fvs_chirho = self.env_chirho.free_vars_chirho();
        for pred_chirho in &self.given_preds_chirho {
            env_fvs_chirho.extend(pred_chirho.ty_chirho.free_vars_chirho());
            for extra_ty_chirho in &pred_chirho.extra_tys_chirho {
                env_fvs_chirho.extend(extra_ty_chirho.free_vars_chirho());
            }
        }
        // workflow: language-features-chirho/fundep-generalization-chirho
        let newly_mono_chirho = self.fundep_newly_mono_vars_chirho(&env_fvs_chirho);
        let ty_fvs_chirho = ty_chirho.free_vars_chirho();
        let vars_chirho: Vec<TyVarChirho> = ty_fvs_chirho
            .into_iter()
            .filter(|v_chirho| {
                !env_fvs_chirho.contains(v_chirho) && !newly_mono_chirho.contains(v_chirho)
            })
            .collect();
        let fundeps_chirho = FundepGeneralizationChirho {
            newly_mono_chirho,
            quantified_chirho: &vars_chirho,
            scope_chirho,
        };

        // Partition deferred predicates: those whose type vars are all being
        // generalized go into the scheme; the rest stay deferred.
        let mut scheme_preds_chirho = Vec::new();
        let mut remaining_chirho = Vec::new();
        let drained_preds_chirho: Vec<(PredChirho, SpanChirho)> =
            self.deferred_preds_chirho.drain(..).collect();
        for wanted_chirho in drained_preds_chirho {
            let Some((pred_chirho, span_chirho)) = self.place_fundep_wanted_chirho(
                wanted_chirho,
                &fundeps_chirho,
                &mut scheme_preds_chirho,
                &mut remaining_chirho,
            ) else {
                continue;
            };
            let pred_fvs_chirho = pred_chirho.ty_chirho.free_vars_chirho();
            // A predicate over a rigid signature variable belongs to the
            // enclosing signature's context: it is discharged by those givens
            // when their scope ends, or reported. It is never quantified away.
            let mentions_rigid_chirho = contains_skolem_chirho(&pred_chirho.ty_chirho)
                || pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .any(contains_skolem_chirho);
            if !mentions_rigid_chirho
                && pred_fvs_chirho
                    .iter()
                    .all(|v_chirho| vars_chirho.contains(v_chirho))
            {
                // A predicate with no free variables passes the check above
                // vacuously and used to be absorbed into the scheme unchecked
                // — silently accepting programs GHC rejects (`Eq U` with no
                // instance anywhere in the defining module). Groundness makes
                // the verdict final: no later substitution can change it.
                if pred_fvs_chirho.is_empty()
                    && self.report_certainly_unsolvable_ground_chirho(&pred_chirho, span_chirho)
                {
                    continue;
                }
                scheme_preds_chirho.push(SchemePredChirho {
                    class_name_chirho: pred_chirho.class_name_chirho,
                    ty_chirho: pred_chirho.ty_chirho,
                    extra_tys_chirho: pred_chirho.extra_tys_chirho,
                });
            } else {
                remaining_chirho.push((pred_chirho, span_chirho));
            }
        }
        self.deferred_preds_chirho = remaining_chirho;

        // Deduplicate predicates
        scheme_preds_chirho.dedup();

        // Context reduction: remove predicates that are entailed by
        // other predicates via the superclass hierarchy.
        // E.g. if we have (Eq a, Num a), remove Eq a since Num has Eq
        // as a superclass.
        let keep_chirho: Vec<bool> = scheme_preds_chirho
            .iter()
            .map(|pred_chirho| {
                !scheme_preds_chirho.iter().any(|other_chirho| {
                    other_chirho.class_name_chirho != pred_chirho.class_name_chirho
                        && other_chirho.ty_chirho == pred_chirho.ty_chirho
                        && self.is_transitive_superclass_chirho(
                            &pred_chirho.class_name_chirho,
                            &other_chirho.class_name_chirho,
                        )
                })
            })
            .collect();
        scheme_preds_chirho = scheme_preds_chirho
            .into_iter()
            .zip(keep_chirho)
            .filter(|(_, keep_chirho)| *keep_chirho)
            .map(|(p_chirho, _)| p_chirho)
            .collect();

        // Evidence-threading P2: top-level generalization can absorb
        // `(Num a, Fractional a)` into the binding scheme before module-end
        // defaulting sees it. Preserve the fractional evidence for any method
        // occurrence captured on the same bare variable so `negate 5.5`
        // dispatches through the Double row instead of the class-only Int
        // fallback used for truly unconstrained methods.
        let fractional_default_classes_chirho: &[&str] =
            &["Fractional", "Floating", "RealFrac", "RealFloat"];
        let mut pred_classes_by_var_chirho: HashMap<TyVarChirho, Vec<&str>> = HashMap::new();
        for pred_chirho in &scheme_preds_chirho {
            if let TyChirho::VarChirho(var_chirho) = &pred_chirho.ty_chirho {
                pred_classes_by_var_chirho
                    .entry(*var_chirho)
                    .or_default()
                    .push(pred_chirho.class_name_chirho.as_str());
            }
        }
        for (var_chirho, classes_chirho) in pred_classes_by_var_chirho {
            if classes_chirho
                .iter()
                .any(|class_chirho| fractional_default_classes_chirho.contains(class_chirho))
            {
                self.occurrence_default_hints_chirho
                    .insert(var_chirho, "Double".to_string());
            } else {
                self.occurrence_generalized_vars_chirho.insert(var_chirho);
            }
        }

        // IO defaulting: when a predicate is Monad/Applicative/Functor on a
        // bare type variable with no other non-IO-defaultable constraints,
        // default the variable to IO (like GHC's ExtendedDefaultRules).
        // This ensures `f = return 42` gets type `IO Int` rather than
        // `forall m. Monad m => m Int`, avoiding dictionary abstraction.
        let io_defaultable_chirho: &[&str] = &[
            "Monad",
            "Applicative",
            "Functor",
            "MonadIO",
            "MonadFail",
            "MonadFix",
        ];
        let mut io_default_subst_chirho = SubstChirho::empty_chirho();
        // Group predicates by type variable
        let mut var_pred_classes_chirho: std::collections::HashMap<TyVarChirho, Vec<&str>> =
            std::collections::HashMap::new();
        for pred_chirho in &scheme_preds_chirho {
            if let TyChirho::VarChirho(v_chirho) = &pred_chirho.ty_chirho {
                var_pred_classes_chirho
                    .entry(*v_chirho)
                    .or_default()
                    .push(&pred_chirho.class_name_chirho);
            }
        }
        // DETERMINISM: builds the IO-defaulting substitution — same hazard as the
        // defaulting loop in check_deferred_preds_chirho. Sort so the substitution is a
        // function of the program, not of the hasher.
        let mut sorted_var_pred_classes_chirho: Vec<_> = var_pred_classes_chirho.iter().collect();
        sorted_var_pred_classes_chirho.sort_by_key(|(var_chirho, _)| **var_chirho);
        for (var_chirho, classes_chirho) in sorted_var_pred_classes_chirho {
            let all_io_chirho = classes_chirho
                .iter()
                .all(|c_chirho| io_defaultable_chirho.contains(c_chirho));
            if all_io_chirho && !classes_chirho.is_empty() {
                io_default_subst_chirho
                    .insert_chirho(*var_chirho, TyChirho::ConChirho("IO".to_string()));
            }
        }
        if allow_io_defaulting_chirho && !io_default_subst_chirho.is_empty_chirho() {
            // Apply IO defaulting: substitute in the type, remove defaulted
            // vars, and remove satisfied predicates.
            let defaulted_ty_chirho = io_default_subst_chirho.apply_ty_chirho(ty_chirho);
            let defaulted_vars_chirho: Vec<TyVarChirho> = vars_chirho
                .into_iter()
                .filter(|v_chirho| !io_default_subst_chirho.lookup_chirho(v_chirho).is_some())
                .collect();
            let defaulted_preds_chirho: Vec<SchemePredChirho> = scheme_preds_chirho
                .into_iter()
                .filter(|p_chirho| {
                    if let TyChirho::VarChirho(v_chirho) = &p_chirho.ty_chirho {
                        !io_default_subst_chirho.lookup_chirho(v_chirho).is_some()
                    } else {
                        true
                    }
                })
                .collect();
            return SchemeChirho {
                vars_chirho: defaulted_vars_chirho,
                preds_chirho: defaulted_preds_chirho,
                ty_chirho: defaulted_ty_chirho,
            };
        }

        SchemeChirho {
            vars_chirho,
            preds_chirho: scheme_preds_chirho,
            ty_chirho: ty_chirho.clone(),
        }
    }

    /// Check if `potential_super_chirho` is a (transitive) superclass of
    /// `sub_chirho`. For example, `Eq` is a superclass of `Ord` and also
    /// of `Num` (since Num has Eq in its superclass list).
    fn is_transitive_superclass_chirho(
        &self,
        potential_super_chirho: &str,
        sub_chirho: &str,
    ) -> bool {
        let supers_chirho = self.class_env_chirho.superclasses_chirho(sub_chirho);
        for s_chirho in &supers_chirho {
            if s_chirho == potential_super_chirho
                || self.is_transitive_superclass_chirho(potential_super_chirho, s_chirho)
            {
                return true;
            }
        }
        false
    }

    /// Apply a substitution to the type environment and deferred predicates.
    fn apply_subst_all_chirho(&mut self, subst_chirho: &SubstChirho) {
        self.env_chirho.apply_subst_chirho(subst_chirho);
        self.recursive_references_chirho
            .apply_subst_chirho(subst_chirho);
        // Every argument of a multi-parameter predicate must follow the
        // substitution, not only the primary one: a functional-dependency
        // improvement or a solved variable in an extra position would
        // otherwise be lost (`MyReader r v` with `v := Int`).
        for (pred_chirho, _span_chirho) in &mut self.deferred_preds_chirho {
            pred_chirho.ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
            for extra_ty_chirho in &mut pred_chirho.extra_tys_chirho {
                *extra_ty_chirho = subst_chirho.apply_ty_chirho(extra_ty_chirho);
            }
        }
        for pred_chirho in &mut self.given_preds_chirho {
            pred_chirho.ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
            for extra_ty_chirho in &mut pred_chirho.extra_tys_chirho {
                *extra_ty_chirho = subst_chirho.apply_ty_chirho(extra_ty_chirho);
            }
        }
        for (left_chirho, right_chirho, _span_chirho) in &mut self.deferred_equalities_chirho {
            *left_chirho = subst_chirho.apply_ty_chirho(left_chirho);
            *right_chirho = subst_chirho.apply_ty_chirho(right_chirho);
        }
        for (lhs_chirho, rhs_chirho) in &mut self.given_rewrites_chirho {
            *lhs_chirho = subst_chirho.apply_ty_chirho(lhs_chirho);
            *rhs_chirho = subst_chirho.apply_ty_chirho(rhs_chirho);
        }
        for capture_chirho in &mut self.occurrence_captures_chirho {
            capture_chirho.ty_chirho = subst_chirho.apply_ty_chirho(&capture_chirho.ty_chirho);
        }
        if !self.occurrence_default_hints_chirho.is_empty() {
            let mut updated_hints_chirho = HashMap::new();
            // DETERMINISM: substitution can map two distinct vars onto the SAME
            // updated var, and the later insert then wins. Draining a HashMap
            // makes "later" depend on a per-process hash seed, so the surviving
            // defaulting hint — and with it the inferred type — could differ
            // between runs of the same binary on the same source.
            // See spec-chirho/bug-nondeterministic-typecheck-chirho.md
            let mut drained_hints_chirho: Vec<_> =
                self.occurrence_default_hints_chirho.drain().collect();
            drained_hints_chirho.sort_by_key(|(var_chirho, _)| *var_chirho);
            for (var_chirho, key_chirho) in drained_hints_chirho {
                match subst_chirho.apply_ty_chirho(&TyChirho::VarChirho(var_chirho)) {
                    TyChirho::VarChirho(updated_var_chirho) => {
                        updated_hints_chirho.insert(updated_var_chirho, key_chirho);
                    }
                    _ => {}
                }
            }
            self.occurrence_default_hints_chirho = updated_hints_chirho;
        }
        if !self.occurrence_generalized_vars_chirho.is_empty() {
            let mut updated_vars_chirho = HashSet::new();
            for var_chirho in self.occurrence_generalized_vars_chirho.drain() {
                if let TyChirho::VarChirho(updated_var_chirho) =
                    subst_chirho.apply_ty_chirho(&TyChirho::VarChirho(var_chirho))
                {
                    updated_vars_chirho.insert(updated_var_chirho);
                }
            }
            self.occurrence_generalized_vars_chirho = updated_vars_chirho;
        }
        for pred_chirho in &mut self.given_preds_chirho {
            pred_chirho.ty_chirho = subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
            pred_chirho.extra_tys_chirho = pred_chirho
                .extra_tys_chirho
                .iter()
                .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
                .collect();
        }
    }

    /// Extensions under which our wanted constraints do not faithfully mirror
    /// the source, so an "unsolvable" wanted may be our artifact rather than
    /// the program's error. RebindableSyntax rebinds `negate`/literals to
    /// user functions while we still emit `Num` wanteds; the rank-n family
    /// makes us instantiate a `forall`-typed field once and reuse it at two
    /// types, inventing constraints the program never had.
    const UNFAITHFUL_CONSTRAINT_EXTENSIONS_CHIRHO: &'static [&'static str] = &[
        "RebindableSyntax",
        "RankNTypes",
        "Rank2Types",
        "ImpredicativeTypes",
    ];

    /// Classes the certainly-unsolvable check must never fire on: solved by
    /// compiler magic (or a user class colliding with such a name), not by
    /// declared instances.
    const MAGIC_CLASSES_CHIRHO: &'static [&'static str] = &[
        "Coercible",
        "Typeable",
        "KnownNat",
        "KnownSymbol",
        "KnownChar",
        "HasField",
        "HasCallStack",
        "IP",
        "Unsatisfiable",
        "TypeError",
        "DataToTag",
        "WithDict",
    ];

    /// Outer head constructor name of a type: the base of the application
    /// spine when that base is concrete. Lists, tuples, and functions use
    /// canonical internal keys so both instance heads and predicates map to
    /// the same name.
    fn ty_outer_head_name_chirho(ty_chirho: &TyChirho) -> Option<String> {
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => Some(name_chirho.clone()),
            TyChirho::AppChirho(fun_chirho, _) => Self::ty_outer_head_name_chirho(fun_chirho),
            TyChirho::ListChirho(_) => Some("[]".to_string()),
            TyChirho::TupleChirho(elems_chirho) => Some(format!("(tuple/{})", elems_chirho.len())),
            TyChirho::FunChirho(..) => Some("->".to_string()),
            _ => None,
        }
    }

    /// Whether every argument of the predicate is free of unification
    /// variables. A fully-ground predicate is invariant under all later
    /// substitution, so its solvability verdict cannot change.
    fn pred_fully_ground_chirho(pred_chirho: &PredChirho) -> bool {
        pred_chirho.ty_chirho.free_vars_chirho().is_empty()
            && pred_chirho
                .extra_tys_chirho
                .iter()
                .all(|t_chirho| t_chirho.free_vars_chirho().is_empty())
            && !Self::pred_mentions_rigid_chirho(pred_chirho)
    }

    /// Whether an instance could EVER match the predicate, comparing only
    /// outer head constructors per position. A variable-headed instance side
    /// matches anything; concrete heads must agree. Deliberately looser than
    /// real matching so partial matches always suppress the unsolvable error.
    fn instance_could_match_pred_chirho(
        inst_chirho: &InstDeclChirho,
        pred_chirho: &PredChirho,
    ) -> bool {
        if inst_chirho.extra_head_tys_chirho.len() != pred_chirho.extra_tys_chirho.len() {
            return false;
        }
        let position_compatible_chirho = |inst_ty_chirho: &TyChirho, pred_ty_chirho: &TyChirho| {
            match Self::ty_outer_head_name_chirho(inst_ty_chirho) {
                None => true,
                Some(inst_head_chirho) => Self::ty_outer_head_name_chirho(pred_ty_chirho)
                    .map_or(true, |pred_head_chirho| {
                        pred_head_chirho == inst_head_chirho
                    }),
            }
        };
        if !position_compatible_chirho(&inst_chirho.head_ty_chirho, &pred_chirho.ty_chirho) {
            return false;
        }
        inst_chirho
            .extra_head_tys_chirho
            .iter()
            .zip(pred_chirho.extra_tys_chirho.iter())
            .all(|(inst_extra_chirho, pred_extra_chirho)| {
                position_compatible_chirho(inst_extra_chirho, pred_extra_chirho)
            })
    }

    /// A predicate is *certainly unsolvable* when its class's full instance
    /// universe is visible (standard-seeded or declared in this module —
    /// see `fully_known_class_names_chirho`), every argument is ground with a
    /// concrete outer head that is not an unexpanded synonym or stuck type
    /// family, no known or derived instance could ever match those heads, and
    /// the in-scope givens do not entail it. GHC reports exactly these as
    /// "No instance for ..." — accepting them silently is unsound.
    fn is_certainly_unsolvable_pred_chirho(&self, pred_chirho: &PredChirho) -> bool {
        self.is_certainly_unsolvable_pred_depth_chirho(pred_chirho, 0)
    }

    /// The verdict a GROUND wanted gets where it is final: GHC's "No instance
    /// for" when it is certainly unsolvable, otherwise none at all. Top-level
    /// generalization and an instance method's leftovers share it, so a method
    /// is judged exactly as a signature-bound binding is. Returns whether the
    /// wanted was reported.
    /// workflow: language-features-chirho/instance-obligations-chirho
    fn report_certainly_unsolvable_ground_chirho(
        &mut self,
        pred_chirho: &PredChirho,
        span_chirho: SpanChirho,
    ) -> bool {
        if !(Self::pred_fully_ground_chirho(pred_chirho)
            && self.is_certainly_unsolvable_pred_chirho(pred_chirho))
        {
            return false;
        }
        let blamed_chirho = self.unsolvable_blame_pred_chirho(pred_chirho, 0);
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                format!("no instance for `{blamed_chirho}`"),
                span_chirho,
            ));
        true
    }

    /// How deep the sub-goals of matching instances are followed before the
    /// answer becomes "unproved" rather than "unsolvable".
    const UNSOLVABLE_SUB_GOAL_DEPTH_CHIRHO: usize = 8;

    fn is_certainly_unsolvable_pred_depth_chirho(
        &self,
        pred_chirho: &PredChirho,
        depth_chirho: usize,
    ) -> bool {
        if !self.constraint_generation_faithful_chirho {
            return false;
        }
        let class_name_chirho = pred_chirho.class_name_chirho.as_str();
        if class_name_chirho == "~"
            || !self
                .fully_known_class_names_chirho
                .contains(class_name_chirho)
            || Self::MAGIC_CLASSES_CHIRHO.contains(&class_name_chirho)
            || class_name_chirho.starts_with('?')
            || !Self::pred_fully_ground_chirho(pred_chirho)
        {
            return false;
        }
        let normalized_args_chirho: Vec<TyChirho> = std::iter::once(&pred_chirho.ty_chirho)
            .chain(pred_chirho.extra_tys_chirho.iter())
            .map(|t_chirho| self.normalize_ty_chirho(t_chirho))
            .collect();
        let mut head_names_chirho = Vec::new();
        for arg_chirho in &normalized_args_chirho {
            match Self::ty_outer_head_name_chirho(arg_chirho) {
                None => return false,
                Some(head_chirho) => {
                    // Unexpanded synonyms and stuck families could still
                    // reduce to an instance-matching head; `#`-suffixed
                    // primitive tycons (Int#, ByteArray#) go through solver
                    // paths we deliberately mismodel (unboxed literals are
                    // currently typed via Num), so no verdict on either.
                    if self.type_families_chirho.contains_key(&head_chirho)
                        || self.type_synonyms_chirho.contains_key(&head_chirho)
                        || head_chirho.ends_with('#')
                    {
                        return false;
                    }
                    head_names_chirho.push(head_chirho);
                }
            }
        }
        if self
            .derived_instance_heads_chirho
            .contains(&(class_name_chirho.to_string(), head_names_chirho[0].clone()))
            || self
                .derived_instance_heads_chirho
                .contains(&(class_name_chirho.to_string(), "*".to_string()))
        {
            return false;
        }
        let normalized_pred_chirho = PredChirho {
            class_name_chirho: pred_chirho.class_name_chirho.clone(),
            ty_chirho: normalized_args_chirho[0].clone(),
            extra_tys_chirho: normalized_args_chirho[1..].to_vec(),
        };
        if let Some(instances_chirho) = self
            .class_env_chirho
            .instances_chirho
            .get(class_name_chirho)
        {
            let candidates_chirho: Vec<&InstDeclChirho> = instances_chirho
                .iter()
                .filter(|inst_chirho| {
                    Self::instance_could_match_pred_chirho(inst_chirho, &normalized_pred_chirho)
                })
                .collect();
            if !candidates_chirho.is_empty() {
                // An instance whose head matches does not make the predicate
                // solvable: its own context has to hold too. `instance Convert a
                // String => Render [a]` cannot serve `Render [Int]` when no
                // `Convert Int String` exists (GHC-39999 at the use site). The
                // verdict stays conservative: every candidate must carry a
                // sub-goal that is itself certainly unsolvable, a candidate we
                // cannot instantiate gives no verdict, and the depth bound ends
                // in "unproved".
                // workflow: language-features-chirho/instance-obligations-chirho
                if depth_chirho >= Self::UNSOLVABLE_SUB_GOAL_DEPTH_CHIRHO {
                    return false;
                }
                let every_candidate_fails_chirho = candidates_chirho.iter().all(|inst_chirho| {
                    match ClassEnvChirho::instance_sub_goals_chirho(
                        inst_chirho,
                        &normalized_pred_chirho,
                    ) {
                        Some(sub_goals_chirho) => sub_goals_chirho.iter().any(|goal_chirho| {
                            self.is_certainly_unsolvable_pred_depth_chirho(
                                goal_chirho,
                                depth_chirho + 1,
                            )
                        }),
                        None => false,
                    }
                });
                if !every_candidate_fails_chirho {
                    return false;
                }
            }
        }
        let normalized_givens_chirho: Vec<PredChirho> = self
            .given_preds_chirho
            .iter()
            .map(|given_chirho| PredChirho {
                class_name_chirho: given_chirho.class_name_chirho.clone(),
                ty_chirho: self.normalize_ty_chirho(&given_chirho.ty_chirho),
                extra_tys_chirho: given_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| self.normalize_ty_chirho(t_chirho))
                    .collect(),
            })
            .collect();
        if self.pred_entailed_by_givens_chirho(pred_chirho, &self.given_preds_chirho)
            || self
                .pred_entailed_by_givens_chirho(&normalized_pred_chirho, &normalized_givens_chirho)
        {
            return false;
        }
        true
    }

    /// The predicate to NAME when one is certainly unsolvable: the sub-goal that
    /// makes it so, or the predicate itself. GHC reports the missing sub-goal
    /// ("No instance for `Convert Int String` arising from a use of `render`"),
    /// not the constraint that led to it.
    /// workflow: language-features-chirho/instance-obligations-chirho
    fn unsolvable_blame_pred_chirho(
        &self,
        pred_chirho: &PredChirho,
        depth_chirho: usize,
    ) -> PredChirho {
        if depth_chirho >= Self::UNSOLVABLE_SUB_GOAL_DEPTH_CHIRHO {
            return pred_chirho.clone();
        }
        let normalized_pred_chirho = PredChirho {
            class_name_chirho: pred_chirho.class_name_chirho.clone(),
            ty_chirho: self.normalize_ty_chirho(&pred_chirho.ty_chirho),
            extra_tys_chirho: pred_chirho
                .extra_tys_chirho
                .iter()
                .map(|ty_chirho| self.normalize_ty_chirho(ty_chirho))
                .collect(),
        };
        let Some(instances_chirho) = self
            .class_env_chirho
            .instances_chirho
            .get(pred_chirho.class_name_chirho.as_str())
        else {
            return pred_chirho.clone();
        };
        for inst_chirho in instances_chirho.iter().filter(|inst_chirho| {
            Self::instance_could_match_pred_chirho(inst_chirho, &normalized_pred_chirho)
        }) {
            let Some(sub_goals_chirho) =
                ClassEnvChirho::instance_sub_goals_chirho(inst_chirho, &normalized_pred_chirho)
            else {
                continue;
            };
            if let Some(goal_chirho) = sub_goals_chirho.iter().find(|goal_chirho| {
                self.is_certainly_unsolvable_pred_depth_chirho(goal_chirho, depth_chirho + 1)
            }) {
                return self.unsolvable_blame_pred_chirho(goal_chirho, depth_chirho + 1);
            }
        }
        pred_chirho.clone()
    }

    fn pred_entailed_by_givens_chirho(
        &self,
        pred_chirho: &PredChirho,
        givens_chirho: &[PredChirho],
    ) -> bool {
        givens_chirho.iter().any(|given_chirho| {
            pred_chirho.ty_chirho == given_chirho.ty_chirho
                && pred_chirho.extra_tys_chirho == given_chirho.extra_tys_chirho
                && (pred_chirho.class_name_chirho == given_chirho.class_name_chirho
                    || self.is_transitive_superclass_chirho(
                        &pred_chirho.class_name_chirho,
                        &given_chirho.class_name_chirho,
                    ))
        })
    }

    /// Unify two types after normalizing: expand type synonyms and reduce
    /// type families. This ensures that `String` unifies with `[Char]` and
    /// `F T1` unifies with `Char` when `type instance F T1 = Char`.
    /// The recursive cases re-normalize after intermediate substitutions so
    /// reductions like `PrimState (ST s) ~ s` become visible once `m ~ ST s`.
    fn unify_normalized_chirho(
        &mut self,
        ty1_chirho: &TyChirho,
        ty2_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<SubstChirho, UnifyErrorChirho> {
        let n1_chirho = self.normalize_ty_chirho(ty1_chirho);
        let n2_chirho = self.normalize_ty_chirho(ty2_chirho);
        match (&n1_chirho, &n2_chirho) {
            (
                TyChirho::ForallChirho {
                    vars_chirho: vars1_chirho,
                    body_chirho: body1_chirho,
                },
                TyChirho::ForallChirho {
                    vars_chirho: vars2_chirho,
                    body_chirho: body2_chirho,
                },
            ) if vars1_chirho.len() == vars2_chirho.len() => {
                let mut rename_chirho = SubstChirho::empty_chirho();
                for (var1_chirho, var2_chirho) in vars1_chirho.iter().zip(vars2_chirho.iter()) {
                    rename_chirho.insert_chirho(*var2_chirho, TyChirho::VarChirho(*var1_chirho));
                }
                let renamed_body2_chirho = rename_chirho.apply_ty_chirho(body2_chirho);
                return self.unify_normalized_chirho(
                    body1_chirho,
                    &renamed_body2_chirho,
                    span_chirho,
                );
            }
            (
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                },
                other_chirho,
            ) if !matches!(other_chirho, TyChirho::VarChirho(_)) => {
                let mut instantiation_chirho = SubstChirho::empty_chirho();
                for var_chirho in vars_chirho {
                    instantiation_chirho.insert_chirho(*var_chirho, self.fresh_var_chirho());
                }
                let instantiated_body_chirho = instantiation_chirho.apply_ty_chirho(body_chirho);
                return self.unify_normalized_chirho(
                    &instantiated_body_chirho,
                    other_chirho,
                    span_chirho,
                );
            }
            (
                other_chirho,
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                },
            ) if !matches!(other_chirho, TyChirho::VarChirho(_)) => {
                let mut instantiation_chirho = SubstChirho::empty_chirho();
                for var_chirho in vars_chirho {
                    instantiation_chirho.insert_chirho(*var_chirho, self.fresh_var_chirho());
                }
                let instantiated_body_chirho = instantiation_chirho.apply_ty_chirho(body_chirho);
                return self.unify_normalized_chirho(
                    other_chirho,
                    &instantiated_body_chirho,
                    span_chirho,
                );
            }
            _ => {}
        }
        // A type-family application stuck on an unsolved variable cannot be
        // compared yet: defer the equality rather than report a mismatch
        // (`n ~ N t` before `t` is known). This must happen BEFORE any
        // structural decomposition — a family application is not injective,
        // so `F t Bool ~ Maybe Int` never implies `F t ~ Maybe`. It is retried
        // when the variable is solved, when givens change, and at the end of
        // the module.
        // workflow: language-features-chirho/stuck-family-equalities-chirho
        if self.defer_stuck_family_equality_chirho(&n1_chirho, &n2_chirho, span_chirho) {
            return Ok(SubstChirho::empty_chirho());
        }
        if let (
            TyChirho::FunChirho(arg1_chirho, result1_chirho, _),
            TyChirho::FunChirho(arg2_chirho, result2_chirho, _),
        ) = (&n1_chirho, &n2_chirho)
        {
            let arg_first_result_chirho: Result<SubstChirho, UnifyErrorChirho> = (|| {
                let s1_chirho =
                    self.unify_normalized_chirho(arg1_chirho, arg2_chirho, span_chirho)?;
                let result1_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(result1_chirho));
                let result2_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(result2_chirho));
                let s2_chirho = self.unify_normalized_chirho(
                    &result1_sub_chirho,
                    &result2_sub_chirho,
                    span_chirho,
                )?;
                Ok(s2_chirho.compose_chirho(&s1_chirho))
            })();
            if let Ok(subst_chirho) = arg_first_result_chirho {
                return Ok(subst_chirho);
            }

            let result_first_result_chirho: Result<SubstChirho, UnifyErrorChirho> = (|| {
                let s1_chirho =
                    self.unify_normalized_chirho(result1_chirho, result2_chirho, span_chirho)?;
                let arg1_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(arg1_chirho));
                let arg2_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(arg2_chirho));
                let s2_chirho =
                    self.unify_normalized_chirho(&arg1_sub_chirho, &arg2_sub_chirho, span_chirho)?;
                Ok(s2_chirho.compose_chirho(&s1_chirho))
            })(
            );
            if let Ok(subst_chirho) = result_first_result_chirho {
                return Ok(subst_chirho);
            }
        }
        if let (
            TyChirho::AppChirho(fun1_chirho, arg1_chirho),
            TyChirho::AppChirho(fun2_chirho, arg2_chirho),
        ) = (&n1_chirho, &n2_chirho)
        {
            let fun_first_result_chirho: Result<SubstChirho, UnifyErrorChirho> = (|| {
                let s1_chirho =
                    self.unify_normalized_chirho(fun1_chirho, fun2_chirho, span_chirho)?;
                let arg1_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(arg1_chirho));
                let arg2_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(arg2_chirho));
                let s2_chirho =
                    self.unify_normalized_chirho(&arg1_sub_chirho, &arg2_sub_chirho, span_chirho)?;
                Ok(s2_chirho.compose_chirho(&s1_chirho))
            })();
            if let Ok(subst_chirho) = fun_first_result_chirho {
                return Ok(subst_chirho);
            }

            let arg_first_result_chirho: Result<SubstChirho, UnifyErrorChirho> = (|| {
                let s1_chirho =
                    self.unify_normalized_chirho(arg1_chirho, arg2_chirho, span_chirho)?;
                let fun1_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(fun1_chirho));
                let fun2_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(fun2_chirho));
                let s2_chirho =
                    self.unify_normalized_chirho(&fun1_sub_chirho, &fun2_sub_chirho, span_chirho)?;
                Ok(s2_chirho.compose_chirho(&s1_chirho))
            })();
            if let Ok(subst_chirho) = arg_first_result_chirho {
                return Ok(subst_chirho);
            }
        }
        if !matches!(n2_chirho, TyChirho::VarChirho(_))
            && let Some(inner1_chirho) = self.saturated_st_inner_ty_chirho(&n1_chirho)
        {
            if let Ok(subst_chirho) =
                self.unify_normalized_chirho(&inner1_chirho, &n2_chirho, span_chirho)
            {
                return Ok(subst_chirho);
            }
        }
        if !matches!(n1_chirho, TyChirho::VarChirho(_))
            && let Some(inner2_chirho) = self.saturated_st_inner_ty_chirho(&n2_chirho)
        {
            if let Ok(subst_chirho) =
                self.unify_normalized_chirho(&n1_chirho, &inner2_chirho, span_chirho)
            {
                return Ok(subst_chirho);
            }
        }
        if let (TyChirho::ListChirho(inner1_chirho), TyChirho::ListChirho(inner2_chirho)) =
            (&n1_chirho, &n2_chirho)
        {
            return self.unify_normalized_chirho(inner1_chirho, inner2_chirho, span_chirho);
        }
        if let (TyChirho::TupleChirho(elems1_chirho), TyChirho::TupleChirho(elems2_chirho)) =
            (&n1_chirho, &n2_chirho)
        {
            if elems1_chirho.len() != elems2_chirho.len() {
                return unify_chirho(&n1_chirho, &n2_chirho, span_chirho);
            }
            let mut subst_chirho = SubstChirho::empty_chirho();
            for (elem1_chirho, elem2_chirho) in elems1_chirho.iter().zip(elems2_chirho.iter()) {
                let elem1_sub_chirho =
                    self.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(elem1_chirho));
                let elem2_sub_chirho =
                    self.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(elem2_chirho));
                let s_chirho = self.unify_normalized_chirho(
                    &elem1_sub_chirho,
                    &elem2_sub_chirho,
                    span_chirho,
                )?;
                subst_chirho = s_chirho.compose_chirho(&subst_chirho);
            }
            return Ok(subst_chirho);
        }
        unify_chirho(&n1_chirho, &n2_chirho, span_chirho)
    }

    fn subsume_normalized_chirho(
        &mut self,
        actual_chirho: &TyChirho,
        expected_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<SubstChirho, UnifyErrorChirho> {
        match (actual_chirho, expected_chirho) {
            (
                TyChirho::ForallChirho {
                    vars_chirho: actual_vars_chirho,
                    body_chirho: actual_body_chirho,
                },
                TyChirho::ForallChirho {
                    vars_chirho: expected_vars_chirho,
                    body_chirho: expected_body_chirho,
                },
            ) if actual_vars_chirho.len() == expected_vars_chirho.len() => {
                let mut rename_chirho = SubstChirho::empty_chirho();
                for (actual_var_chirho, expected_var_chirho) in
                    actual_vars_chirho.iter().zip(expected_vars_chirho.iter())
                {
                    rename_chirho.insert_chirho(
                        *expected_var_chirho,
                        TyChirho::VarChirho(*actual_var_chirho),
                    );
                }
                let renamed_expected_body_chirho =
                    rename_chirho.apply_ty_chirho(expected_body_chirho);
                self.unify_normalized_chirho(
                    actual_body_chirho,
                    &renamed_expected_body_chirho,
                    span_chirho,
                )
            }
            (
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                },
                _,
            ) => {
                let mut instantiation_chirho = SubstChirho::empty_chirho();
                for var_chirho in vars_chirho {
                    let fresh_var_chirho = TyVarChirho(self.next_var_chirho);
                    self.next_var_chirho += 1;
                    instantiation_chirho
                        .insert_chirho(*var_chirho, TyChirho::VarChirho(fresh_var_chirho));
                }
                let instantiated_actual_chirho = instantiation_chirho.apply_ty_chirho(body_chirho);
                self.subsume_normalized_chirho(
                    &instantiated_actual_chirho,
                    expected_chirho,
                    span_chirho,
                )
            }
            (
                _,
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                },
            ) => {
                let mut instantiation_chirho = SubstChirho::empty_chirho();
                for var_chirho in vars_chirho {
                    let fresh_var_chirho = TyVarChirho(self.next_var_chirho);
                    self.next_var_chirho += 1;
                    instantiation_chirho
                        .insert_chirho(*var_chirho, TyChirho::VarChirho(fresh_var_chirho));
                }
                let instantiated_expected_chirho =
                    instantiation_chirho.apply_ty_chirho(body_chirho);
                self.subsume_normalized_chirho(
                    actual_chirho,
                    &instantiated_expected_chirho,
                    span_chirho,
                )
            }
            _ => self.unify_normalized_chirho(actual_chirho, expected_chirho, span_chirho),
        }
    }

    /// Record a unification error as a diagnostic.
    fn report_unify_error_chirho(&mut self, err_chirho: &UnifyErrorChirho) {
        match err_chirho {
            UnifyErrorChirho::MismatchChirho {
                expected_chirho,
                actual_chirho,
                span_chirho,
            } => {
                let mut diagnostic_chirho = DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(TYPE_MISMATCH_CODE_CHIRHO),
                    format!("type mismatch: expected `{expected_chirho}`, found `{actual_chirho}`"),
                    *span_chirho,
                )
                .with_note_chirho(format!("expected type: {expected_chirho}"))
                .with_note_chirho(format!("   found type: {actual_chirho}"));
                for rigid_chirho in [expected_chirho, actual_chirho] {
                    if let TyChirho::ForallVarChirho(name_chirho) = rigid_chirho
                        && crate::skolem_chirho::is_skolem_name_chirho(name_chirho)
                    {
                        diagnostic_chirho = diagnostic_chirho.with_note_chirho(format!(
                            "`{rigid_chirho}` is a rigid type variable bound by a type signature"
                        ));
                    }
                }
                self.diagnostics_chirho.push_chirho(diagnostic_chirho);
            }
            UnifyErrorChirho::OccursCheckChirho {
                var_chirho,
                ty_chirho,
                span_chirho,
            } => {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(OCCURS_CHECK_CODE_CHIRHO),
                        format!("infinite type: `{var_chirho}` occurs in `{ty_chirho}`"),
                        *span_chirho,
                    ));
            }
            UnifyErrorChirho::TupleArityChirho {
                expected_chirho,
                actual_chirho,
                span_chirho,
            } => {
                self.diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(TUPLE_ARITY_CODE_CHIRHO),
                        format!(
                            "tuple arity mismatch: expected {expected_chirho} elements, found {actual_chirho}"
                        ),
                        *span_chirho,
                    ),
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // AST type conversion
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Class and instance declaration processing
    // -----------------------------------------------------------------------

    /// Process a class declaration from the AST and register it in the class env.
    fn process_class_decl_chirho(&mut self, decl_chirho: &DeclChirho) {
        if let DeclChirho::ClassDeclChirho {
            context_chirho,
            name_chirho,
            type_vars_chirho,
            methods_chirho,
            associated_tfs_chirho,
            fundeps_chirho: ast_fundeps_chirho,
            ..
        } = decl_chirho
        {
            let class_name_chirho = name_chirho.text_chirho().to_string();

            // Generate fresh type variables for all class params
            let class_tv_chirho = TyVarChirho(self.next_var_chirho);
            self.next_var_chirho += 1;

            let extra_vars_chirho: Vec<TyVarChirho> = type_vars_chirho
                .iter()
                .skip(1)
                .map(|_| {
                    let v_chirho = TyVarChirho(self.next_var_chirho);
                    self.next_var_chirho += 1;
                    v_chirho
                })
                .collect();

            let mut class_scoped_tyvars_chirho = HashMap::new();
            if let Some(first_var_chirho) = type_vars_chirho.first() {
                class_scoped_tyvars_chirho
                    .insert(first_var_chirho.text_chirho().to_string(), class_tv_chirho);
            }
            for (ast_var_chirho, ty_var_chirho) in type_vars_chirho
                .iter()
                .skip(1)
                .zip(extra_vars_chirho.iter().copied())
            {
                class_scoped_tyvars_chirho
                    .insert(ast_var_chirho.text_chirho().to_string(), ty_var_chirho);
            }

            // Superclasses from context
            let supers_chirho: Vec<String> = context_chirho
                .iter()
                .filter_map(|c_chirho| {
                    c_chirho
                        .simple_class_chirho()
                        .map(|class_chirho| class_chirho.text_chirho().to_string())
                })
                .collect();

            // Method signatures and optional default implementations
            let mut method_map_chirho = HashMap::new();
            let mut defaults_map_chirho = HashMap::new();
            for method_chirho in methods_chirho {
                let method_name_chirho = method_chirho.name_chirho.text_chirho().to_string();
                let (mut scheme_chirho, _method_var_map_chirho) = self
                    .ast_type_to_scheme_seeded_chirho(
                        &method_chirho.ty_chirho,
                        &class_scoped_tyvars_chirho,
                        false,
                    );

                let class_pred_chirho = SchemePredChirho {
                    class_name_chirho: class_name_chirho.clone(),
                    ty_chirho: TyChirho::VarChirho(class_tv_chirho),
                    extra_tys_chirho: extra_vars_chirho
                        .iter()
                        .copied()
                        .map(TyChirho::VarChirho)
                        .collect(),
                };
                if !scheme_chirho.preds_chirho.contains(&class_pred_chirho) {
                    scheme_chirho.preds_chirho.push(class_pred_chirho);
                }
                // A method's type is `forall <class vars>. C <class vars> =>
                // forall <own vars>. ty`: the class variables are quantified
                // first (visible type application `take @n` reaches `n` even
                // when the method type itself never mentions it), then the
                // method's own variables in their order of appearance.
                let mut ordered_vars_chirho: Vec<TyVarChirho> = vec![class_tv_chirho];
                ordered_vars_chirho.extend(extra_vars_chirho.iter().copied());
                for var_chirho in &scheme_chirho.vars_chirho {
                    if !ordered_vars_chirho.contains(var_chirho) {
                        ordered_vars_chirho.push(*var_chirho);
                    }
                }
                scheme_chirho.vars_chirho = ordered_vars_chirho;
                method_map_chirho.insert(method_name_chirho.clone(), scheme_chirho.clone());

                // Also add method to the type environment so it can be used
                self.env_chirho
                    .bind_chirho(method_name_chirho.clone(), scheme_chirho);

                // Capture default implementation if present
                if let Some(ref default_arms_chirho) = method_chirho.default_chirho {
                    defaults_map_chirho.insert(method_name_chirho, default_arms_chirho.clone());
                }
            }

            // Convert AST fundeps (variable names) to indices into type_vars_chirho
            let var_names_chirho: Vec<String> = type_vars_chirho
                .iter()
                .map(|v_chirho| v_chirho.text_chirho().to_string())
                .collect();
            let resolved_fundeps_chirho: Vec<(Vec<usize>, Vec<usize>)> = ast_fundeps_chirho
                .iter()
                .map(|(from_chirho, to_chirho)| {
                    let from_idx_chirho: Vec<usize> = from_chirho
                        .iter()
                        .filter_map(|n_chirho| {
                            var_names_chirho
                                .iter()
                                .position(|v_chirho| v_chirho == n_chirho)
                        })
                        .collect();
                    let to_idx_chirho: Vec<usize> = to_chirho
                        .iter()
                        .filter_map(|n_chirho| {
                            var_names_chirho
                                .iter()
                                .position(|v_chirho| v_chirho == n_chirho)
                        })
                        .collect();
                    (from_idx_chirho, to_idx_chirho)
                })
                .collect();

            self.fully_known_class_names_chirho
                .insert(class_name_chirho.clone());
            self.class_env_chirho.add_class_chirho(ClassDeclChirho {
                name_chirho: class_name_chirho.clone(),
                supers_chirho,
                var_chirho: class_tv_chirho,
                methods_chirho: method_map_chirho,
                extra_vars_chirho,
                fundeps_chirho: resolved_fundeps_chirho,
                defaults_chirho: defaults_map_chirho,
            });

            // Register associated type families as open type families. A
            // default equation is NOT a general equation of the family (that
            // would reduce `F a` for an abstract `a`); it is applied per
            // instance that leaves the family undefined.
            let class_params_chirho: Vec<String> = type_vars_chirho
                .iter()
                .map(|param_chirho| param_chirho.text_chirho().to_string())
                .collect();
            for atf_chirho in associated_tfs_chirho {
                let tf_name_chirho = atf_chirho.name_chirho.text_chirho().to_string();
                self.register_type_family_chirho(tf_name_chirho.clone(), vec![]);
                self.assoc_type_declared_params_chirho.insert(
                    tf_name_chirho.clone(),
                    atf_chirho
                        .type_vars_chirho
                        .iter()
                        .map(|param_chirho| param_chirho.text_chirho().to_string())
                        .collect(),
                );
                if let Some(default_rhs_chirho) = &atf_chirho.default_rhs_chirho {
                    let binders_chirho = if atf_chirho.default_params_chirho.len()
                        == atf_chirho.type_vars_chirho.len()
                    {
                        &atf_chirho.default_params_chirho
                    } else {
                        &atf_chirho.type_vars_chirho
                    };
                    let param_names_chirho: Vec<String> = binders_chirho
                        .iter()
                        .map(|param_chirho| param_chirho.text_chirho().to_string())
                        .collect();
                    self.assoc_type_defaults_chirho
                        .entry(class_name_chirho.clone())
                        .or_default()
                        .push(AssocTypeDefaultChirho {
                            family_chirho: tf_name_chirho,
                            family_params_chirho: param_names_chirho,
                            class_params_chirho: class_params_chirho.clone(),
                            rhs_chirho: default_rhs_chirho.clone(),
                        });
                }
            }
        }
    }

    /// Process an instance declaration from the AST and register it.
    fn process_instance_decl_chirho(&mut self, decl_chirho: &DeclChirho) {
        if let DeclChirho::InstanceDeclChirho {
            context_chirho,
            class_chirho,
            types_chirho,
            assoc_tf_instances_chirho,
            span_chirho,
            ..
        } = decl_chirho
        {
            let class_name_chirho = class_chirho.text_chirho().to_string();

            let mut var_map_chirho = HashMap::new();

            // Instance head type (e.g., `Int` in `instance Eq Int`, or
            // `[a]` in `instance Eq a => Eq [a]`)
            let head_ty_chirho = if let Some(first_ty_chirho) = types_chirho.first() {
                self.ast_type_to_ty_chirho(first_ty_chirho, &mut var_map_chirho)
            } else {
                self.fresh_var_chirho()
            };

            // Context constraints (e.g., `Eq a` in `instance Eq a => Eq [a]`), through
            // the SAME conversion a signature's context uses: every argument is kept, so
            // `Convert a String` does not behave as `Convert a`, and `a ~ Int` keeps both
            // sides. A quantified premise is still unmodelled and makes the context
            // incomplete rather than empty.
            // workflow: language-features-chirho/instance-obligations-chirho
            let (context_preds_chirho, context_equalities_chirho, _unmodelled_chirho) =
                self.convert_written_context_chirho(context_chirho, &mut var_map_chirho);
            let mut inst_context_chirho: Vec<PredChirho> = context_preds_chirho
                .into_iter()
                .map(|pred_chirho| PredChirho {
                    class_name_chirho: pred_chirho.class_name_chirho,
                    ty_chirho: pred_chirho.ty_chirho,
                    extra_tys_chirho: pred_chirho.extra_tys_chirho,
                })
                .collect();
            inst_context_chirho.extend(context_equalities_chirho.into_iter().map(
                |(lhs_chirho, rhs_chirho)| {
                    PredChirho::new_multi_chirho("~", vec![lhs_chirho, rhs_chirho])
                },
            ));

            // For MPTCs, extra head types come from types_chirho[1..]
            let extra_head_tys_chirho: Vec<TyChirho> = types_chirho
                .iter()
                .skip(1)
                .map(|t_chirho| self.ast_type_to_ty_chirho(t_chirho, &mut var_map_chirho))
                .collect();

            // Expand type synonyms in the instance head (e.g. String → [Char])
            let head_ty_chirho = self.expand_type_synonyms_chirho(&head_ty_chirho);
            let extra_head_tys_chirho: Vec<TyChirho> = extra_head_tys_chirho
                .into_iter()
                .map(|t_chirho| self.expand_type_synonyms_chirho(&t_chirho))
                .collect();

            let instance_chirho = InstDeclChirho {
                class_name_chirho,
                head_ty_chirho,
                extra_head_tys_chirho,
                context_chirho: inst_context_chirho,
            };
            // workflow: language-features-chirho/instance-obligations-chirho
            self.local_instances_chirho
                .push(instance_obligations_chirho::LocalInstanceChirho {
                    instance_chirho: instance_chirho.clone(),
                    nullary_chirho: types_chirho.is_empty(),
                    written_names_chirho: instance_obligations_chirho::written_head_names_chirho(
                        class_chirho,
                        types_chirho,
                    ),
                    span_chirho: *span_chirho,
                });
            self.class_env_chirho.add_instance_chirho(instance_chirho);

            // Register associated type family instances from this instance decl
            for atfi_chirho in assoc_tf_instances_chirho {
                let fname_chirho = atfi_chirho.family_name_chirho.text_chirho().to_string();
                let inst_params_chirho =
                    collect_free_type_vars_from_ast_chirho(&atfi_chirho.lhs_types_chirho);
                let lhs_chirho: Vec<TyChirho> = atfi_chirho
                    .lhs_types_chirho
                    .iter()
                    .map(|t_chirho| ast_type_to_syn_rhs_chirho(t_chirho, &inst_params_chirho))
                    .collect();
                let rhs_ty_chirho =
                    ast_type_to_syn_rhs_chirho(&atfi_chirho.rhs_chirho, &inst_params_chirho);
                self.register_type_family_instance_chirho(fname_chirho, lhs_chirho, rhs_ty_chirho);
            }

            // Associated families this instance leaves undefined take the
            // class's default, specialized to the instance head.
            if let Some(defaults_chirho) = self
                .assoc_type_defaults_chirho
                .get(class_chirho.text_chirho())
                .cloned()
            {
                let inst_free_vars_chirho = collect_free_type_vars_from_ast_chirho(types_chirho);
                for default_chirho in defaults_chirho {
                    let defined_here_chirho = assoc_tf_instances_chirho.iter().any(|atfi_chirho| {
                        atfi_chirho.family_name_chirho.text_chirho() == default_chirho.family_chirho
                    });
                    if defined_here_chirho {
                        continue;
                    }
                    // Family parameter `i` names class parameter `j` when the
                    // family DECLARATION spells position `i` with the class
                    // parameter's name; the default equation may use other
                    // spellings, so the mapping goes through positions.
                    let declared_names_chirho: Vec<String> = self
                        .assoc_type_declared_params_chirho
                        .get(&default_chirho.family_chirho)
                        .cloned()
                        .unwrap_or_else(|| default_chirho.family_params_chirho.clone());
                    let mut bindings_chirho: HashMap<String, TyChirho> = HashMap::new();
                    let mut lhs_chirho = Vec::new();
                    for (position_chirho, param_chirho) in
                        default_chirho.family_params_chirho.iter().enumerate()
                    {
                        let declared_chirho = declared_names_chirho
                            .get(position_chirho)
                            .unwrap_or(param_chirho);
                        match default_chirho
                            .class_params_chirho
                            .iter()
                            .position(|class_param_chirho| class_param_chirho == declared_chirho)
                        {
                            Some(index_chirho) if index_chirho < types_chirho.len() => {
                                let head_ty_chirho = ast_type_to_syn_rhs_chirho(
                                    &types_chirho[index_chirho],
                                    &inst_free_vars_chirho,
                                );
                                bindings_chirho
                                    .insert(param_chirho.clone(), head_ty_chirho.clone());
                                lhs_chirho.push(head_ty_chirho);
                            }
                            _ => lhs_chirho.push(TyChirho::ForallVarChirho(param_chirho.clone())),
                        }
                    }
                    let rhs_pattern_chirho = ast_type_to_syn_rhs_chirho(
                        &default_chirho.rhs_chirho,
                        &default_chirho.family_params_chirho,
                    );
                    let rhs_chirho =
                        substitute_type_vars_chirho(&rhs_pattern_chirho, &bindings_chirho);
                    self.register_type_family_instance_chirho(
                        default_chirho.family_chirho.clone(),
                        lhs_chirho,
                        rhs_chirho,
                    );
                }
            }
        }
    }

    fn lookup_value_scheme_with_qualified_suffix_fallback_chirho(
        &self,
        full_name_chirho: &str,
        bare_name_chirho: &str,
    ) -> Option<SchemeChirho> {
        let canonical_full_name_chirho = canonical_value_name_text_chirho(full_name_chirho);
        let canonical_bare_name_chirho = canonical_value_name_text_chirho(bare_name_chirho);
        if canonical_full_name_chirho == canonical_bare_name_chirho {
            return self
                .env_chirho
                .lookup_chirho(&canonical_bare_name_chirho)
                .cloned();
        }

        if let Some(scheme_chirho) = self.env_chirho.lookup_chirho(&canonical_full_name_chirho) {
            return Some(scheme_chirho.clone());
        }

        let suffix_chirho = format!(".{canonical_full_name_chirho}");
        let mut suffix_matches_chirho = self
            .env_chirho
            .all_bindings_chirho()
            .into_iter()
            .filter(|(name_chirho, _scheme_chirho)| name_chirho.ends_with(&suffix_chirho))
            .map(|(_name_chirho, scheme_chirho)| scheme_chirho.clone());
        let first_match_chirho = suffix_matches_chirho.next();
        if first_match_chirho.is_some() && suffix_matches_chirho.next().is_none() {
            return first_match_chirho;
        }

        self.env_chirho
            .lookup_chirho(&canonical_bare_name_chirho)
            .cloned()
    }

    fn lookup_record_constructor_field_bundle_chirho(
        &mut self,
        con_chirho: &NameChirho,
    ) -> Option<(Vec<String>, Vec<TyChirho>, TyChirho)> {
        let con_text_chirho = con_chirho.text_chirho();
        let con_full_name_chirho = con_chirho.full_name_chirho();
        let scheme_chirho = self.lookup_value_scheme_with_qualified_suffix_fallback_chirho(
            &con_full_name_chirho,
            con_text_chirho,
        )?;
        if is_placeholder_import_scheme_chirho(&scheme_chirho) {
            return None;
        }

        let mut field_names_chirho = self
            .con_field_names_chirho
            .get(con_text_chirho)
            .cloned()
            .or_else(|| {
                self.con_field_names_chirho
                    .get(&con_full_name_chirho)
                    .cloned()
            })?;
        let instantiated_chirho = self.instantiate_chirho(&scheme_chirho, con_chirho.span_chirho());
        let mut remaining_ty_chirho = self.open_constructor_forall_chirho(instantiated_chirho);
        let mut field_tys_chirho = Vec::new();
        while field_tys_chirho.len() < field_names_chirho.len() {
            match remaining_ty_chirho {
                TyChirho::FunChirho(arg_ty_chirho, result_ty_chirho, _) => {
                    field_tys_chirho.push(*arg_ty_chirho);
                    remaining_ty_chirho = *result_ty_chirho;
                }
                _ => break,
            }
        }
        field_names_chirho.truncate(field_tys_chirho.len());
        Some((field_names_chirho, field_tys_chirho, remaining_ty_chirho))
    }

    fn peel_type_app_chain_chirho<'a>(
        expr_chirho: &'a ExprChirho,
        type_args_chirho: &mut Vec<&'a TypeChirho>,
    ) -> &'a ExprChirho {
        match expr_chirho {
            ExprChirho::TypeAppChirho {
                expr_chirho,
                ty_chirho,
                ..
            } => {
                let base_chirho = Self::peel_type_app_chain_chirho(expr_chirho, type_args_chirho);
                type_args_chirho.push(ty_chirho);
                base_chirho
            }
            _ => expr_chirho,
        }
    }

    fn infer_named_type_app_chirho(
        &mut self,
        expr_chirho: &ExprChirho,
        span_chirho: SpanChirho,
    ) -> Option<(SubstChirho, TyChirho)> {
        let mut type_args_ast_chirho = Vec::new();
        let base_expr_chirho =
            Self::peel_type_app_chain_chirho(expr_chirho, &mut type_args_ast_chirho);
        if type_args_ast_chirho.is_empty() {
            return None;
        }

        let scheme_chirho = match base_expr_chirho {
            ExprChirho::VarChirho(name_chirho) | ExprChirho::ConChirho(name_chirho) => self
                .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                    &name_chirho.full_name_chirho(),
                    name_chirho.text_chirho(),
                )?,
            _ => {
                return None;
            }
        };

        // A name known only through a placeholder import scheme has no
        // quantifiers to apply type arguments to; its type stays unknown.
        if is_placeholder_import_scheme_chirho(&scheme_chirho) {
            let fresh_chirho = self.fresh_var_chirho();
            return Some((SubstChirho::empty_chirho(), fresh_chirho));
        }
        let explicit_tys_chirho: Vec<TyChirho> = type_args_ast_chirho
            .into_iter()
            .map(|ty_chirho| self.ast_type_to_ty_in_scope_chirho(ty_chirho))
            .collect();
        let (instantiated_ty_chirho, instantiated_preds_chirho) = self
            .instantiate_scheme_parts_with_type_args_chirho(&scheme_chirho, &explicit_tys_chirho)?;
        for pred_inst_chirho in instantiated_preds_chirho {
            self.defer_pred_chirho(pred_inst_chirho, span_chirho);
        }
        Some((SubstChirho::empty_chirho(), instantiated_ty_chirho))
    }

    // workflow: language-features-chirho/rank-n-visible-type-application-chirho
    fn expr_to_required_type_arg_chirho(&self, expr_chirho: &ExprChirho) -> Option<TyChirho> {
        match expr_chirho {
            ExprChirho::VarChirho(name_chirho) | ExprChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                Some(
                    self.scoped_tyvars_chirho
                        .get(text_chirho)
                        .copied()
                        .map(TyChirho::VarChirho)
                        .unwrap_or_else(|| TyChirho::ConChirho(name_chirho.full_name_chirho())),
                )
            }
            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => Some(TyChirho::AppChirho(
                Box::new(self.expr_to_required_type_arg_chirho(fun_chirho)?),
                Box::new(self.expr_to_required_type_arg_chirho(arg_chirho)?),
            )),
            ExprChirho::ListChirho {
                elements_chirho, ..
            } => {
                // Preserve the old reverse freshening order; the shared
                // builder receives source order and folds from the right.
                let converted_chirho = elements_chirho
                    .iter()
                    .rev()
                    .map(|element_chirho| self.expr_to_required_type_arg_chirho(element_chirho))
                    .collect::<Option<Vec<_>>>()?;
                Some(TyChirho::promoted_list_chirho(
                    converted_chirho.into_iter().rev(),
                ))
            }
            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => elements_chirho
                .iter()
                .map(|element_chirho| self.expr_to_required_type_arg_chirho(element_chirho))
                .collect::<Option<Vec<_>>>()
                .map(TyChirho::TupleChirho),
            ExprChirho::ParenChirho { inner_chirho, .. }
            | ExprChirho::AnnChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.expr_to_required_type_arg_chirho(inner_chirho),
            ExprChirho::LitChirho(LitChirho::IntChirho(value_chirho, _, _)) => {
                Some(TyChirho::ConChirho(value_chirho.to_string()))
            }
            ExprChirho::LitChirho(LitChirho::StringChirho(value_chirho, _, _)) => {
                Some(TyChirho::ConChirho(format!("\"{value_chirho}\"")))
            }
            ExprChirho::LitChirho(LitChirho::CharChirho(value_chirho, _, _)) => {
                Some(TyChirho::ConChirho(format!("'{value_chirho}'")))
            }
            _ => None,
        }
    }

    // workflow: language-features-chirho/rank-n-visible-type-application-chirho
    fn apply_required_type_arg_chirho(
        &self,
        fun_ty_chirho: &TyChirho,
        arg_ty_chirho: TyChirho,
    ) -> Option<TyChirho> {
        let TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } = fun_ty_chirho
        else {
            return None;
        };
        let (first_var_chirho, remaining_vars_chirho) = vars_chirho.split_first()?;
        let mut subst_chirho = SubstChirho::empty_chirho();
        subst_chirho.insert_chirho(*first_var_chirho, arg_ty_chirho);
        let body_chirho = subst_chirho.apply_ty_chirho(body_chirho);
        if remaining_vars_chirho.is_empty() {
            Some(body_chirho)
        } else {
            Some(TyChirho::RequiredForallChirho {
                vars_chirho: remaining_vars_chirho.to_vec(),
                body_chirho: Box::new(body_chirho),
            })
        }
    }

    // workflow: language-features-chirho/rank-n-visible-type-application-chirho
    fn apply_visible_type_arg_chirho(
        &self,
        fun_ty_chirho: &TyChirho,
        arg_ty_chirho: TyChirho,
    ) -> Option<TyChirho> {
        let TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } = fun_ty_chirho
        else {
            return None;
        };
        let (first_var_chirho, remaining_vars_chirho) = vars_chirho.split_first()?;
        let mut subst_chirho = SubstChirho::empty_chirho();
        subst_chirho.insert_chirho(*first_var_chirho, arg_ty_chirho);
        let body_chirho = subst_chirho.apply_ty_chirho(body_chirho);
        if remaining_vars_chirho.is_empty() {
            Some(body_chirho)
        } else {
            Some(TyChirho::ForallChirho {
                vars_chirho: remaining_vars_chirho.to_vec(),
                body_chirho: Box::new(body_chirho),
            })
        }
    }

    // -----------------------------------------------------------------------
    // Expression inference
    // -----------------------------------------------------------------------

    /// Infer the type of an expression, returning (substitution, type).
    pub fn infer_expr_chirho(&mut self, expr_chirho: &ExprChirho) -> (SubstChirho, TyChirho) {
        match expr_chirho {
            ExprChirho::LitChirho(lit_chirho) => match lit_chirho {
                LitChirho::IntChirho(_, span_chirho, _) => {
                    let lit_ty_chirho = self.fresh_var_chirho();
                    self.deferred_preds_chirho.push((
                        PredChirho::new_chirho("Num", lit_ty_chirho.clone()),
                        *span_chirho,
                    ));
                    self.capture_literal_evidence_chirho(lit_chirho, "Num", &lit_ty_chirho);
                    (SubstChirho::empty_chirho(), lit_ty_chirho)
                }
                LitChirho::FloatChirho(_, span_chirho, _) => {
                    let lit_ty_chirho = self.fresh_var_chirho();
                    self.deferred_preds_chirho.push((
                        PredChirho::new_chirho("Fractional", lit_ty_chirho.clone()),
                        *span_chirho,
                    ));
                    self.capture_literal_evidence_chirho(lit_chirho, "Fractional", &lit_ty_chirho);
                    (SubstChirho::empty_chirho(), lit_ty_chirho)
                }
                LitChirho::StringChirho(_, span_chirho, _) if self.overloaded_strings_chirho => {
                    let lit_ty_chirho = self.fresh_var_chirho();
                    self.deferred_preds_chirho.push((
                        PredChirho::new_chirho("IsString", lit_ty_chirho.clone()),
                        *span_chirho,
                    ));
                    self.capture_literal_evidence_chirho(lit_chirho, "IsString", &lit_ty_chirho);
                    (SubstChirho::empty_chirho(), lit_ty_chirho)
                }
                _ => {
                    let ty_chirho = infer_lit_chirho(lit_chirho);
                    (SubstChirho::empty_chirho(), ty_chirho)
                }
            },

            ExprChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                let full_name_chirho = name_chirho.full_name_chirho();
                let span_chirho = name_chirho.span_chirho();
                let scheme_opt_chirho = self
                    .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                        &full_name_chirho,
                        text_chirho,
                    );
                match scheme_opt_chirho {
                    Some(scheme_chirho) => {
                        self.recursive_references_chirho.capture_chirho(
                            text_chirho,
                            span_chirho,
                            name_chirho.origin_chirho(),
                            &scheme_chirho,
                        );
                        // Evidence-threading P2: instantiate via parts so each
                        // predicate's fresh class variable can be captured as an
                        // occurrence record (finalized after solving/defaulting).
                        let (ty_chirho, instantiated_preds_chirho) =
                            self.instantiate_scheme_parts_chirho(&scheme_chirho);
                        self.capture_occurrence_predicates_chirho(
                            name_chirho,
                            &instantiated_preds_chirho,
                        );
                        for pred_inst_chirho in instantiated_preds_chirho {
                            self.defer_pred_chirho(pred_inst_chirho, span_chirho);
                        }
                        (SubstChirho::empty_chirho(), ty_chirho)
                    }
                    None => {
                        // Typed holes: `_` or `_foo` in expression position are
                        // treated as holes — emit a warning, not an error, and
                        // assign a fresh type variable. This lets code with holes
                        // compile (for type exploration / incremental development).
                        if text_chirho.starts_with('?') {
                            // ImplicitParams: `?x` is an implicit parameter.
                            // Assign a fresh type variable so it can unify
                            // with whatever the constraint provides.
                            let ip_ty_chirho = self.fresh_var_chirho();
                            (SubstChirho::empty_chirho(), ip_ty_chirho)
                        } else if text_chirho.starts_with('_') {
                            let hole_ty_chirho = self.fresh_var_chirho();
                            let diag_chirho = DiagnosticChirho::warning_with_code_chirho(
                                ErrorCodeChirho::warning_chirho(4200),
                                format!("found hole: `{text_chirho}` :: _"),
                                span_chirho,
                            );
                            self.diagnostics_chirho.push_chirho(diag_chirho);
                            (SubstChirho::empty_chirho(), hole_ty_chirho)
                        } else {
                            let mut diag_chirho = DiagnosticChirho::error_with_code_chirho(
                                ErrorCodeChirho::error_chirho(UNBOUND_VAR_CODE_CHIRHO),
                                format!("unbound variable: `{text_chirho}`"),
                                span_chirho,
                            );
                            // Add "did you mean?" suggestions from the type env.
                            // Filter candidates: if the target is alphanumeric,
                            // don't suggest operator symbols (and vice versa).
                            let is_alpha_chirho =
                                text_chirho.chars().next().is_some_and(|c_chirho| {
                                    c_chirho.is_alphanumeric() || c_chirho == '_'
                                });
                            let candidates_chirho: Vec<&str> = self
                                .env_chirho
                                .all_names_chirho()
                                .into_iter()
                                .filter(|c_chirho| {
                                    if is_alpha_chirho {
                                        c_chirho.chars().next().is_some_and(|ch_chirho| {
                                            ch_chirho.is_alphanumeric() || ch_chirho == '_'
                                        })
                                    } else {
                                        true
                                    }
                                })
                                .collect();
                            let max_dist_chirho = haskelujah_diagnostics_chirho::suggest_chirho::default_max_distance_chirho(text_chirho.len());
                            let suggestions_chirho = haskelujah_diagnostics_chirho::suggest_chirho::suggest_similar_names_chirho(
                                text_chirho,
                                candidates_chirho.into_iter(),
                                max_dist_chirho,
                                3,
                            );
                            if let Some(note_chirho) = haskelujah_diagnostics_chirho::suggest_chirho::format_did_you_mean_chirho(&suggestions_chirho) {
                                diag_chirho = diag_chirho.with_note_chirho(note_chirho);
                            }
                            self.diagnostics_chirho.push_chirho(diag_chirho);
                            (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                        }
                    }
                }
            }

            ExprChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                let full_name_chirho = name_chirho.full_name_chirho();
                let span_chirho = name_chirho.span_chirho();
                let scheme_opt_chirho = self
                    .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                        &full_name_chirho,
                        text_chirho,
                    );
                match scheme_opt_chirho {
                    Some(scheme_chirho) => {
                        // Evidence-threading P2: instantiate via parts so each
                        // predicate's fresh class variable can be captured as an
                        // occurrence record (finalized after solving/defaulting).
                        let (ty_chirho, instantiated_preds_chirho) =
                            self.instantiate_scheme_parts_chirho(&scheme_chirho);
                        self.capture_occurrence_predicates_chirho(
                            name_chirho,
                            &instantiated_preds_chirho,
                        );
                        for pred_inst_chirho in instantiated_preds_chirho {
                            self.defer_pred_chirho(pred_inst_chirho, span_chirho);
                        }
                        (SubstChirho::empty_chirho(), ty_chirho)
                    }
                    None => (SubstChirho::empty_chirho(), self.fresh_var_chirho()),
                }
            }

            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                span_chirho,
            } => {
                let (s1_chirho, fun_ty_chirho) = self.infer_expr_chirho(fun_chirho);
                self.apply_subst_all_chirho(&s1_chirho);
                let fun_ty_chirho = s1_chirho.apply_ty_chirho(&fun_ty_chirho);

                if let Some(required_arg_ty_chirho) =
                    self.expr_to_required_type_arg_chirho(arg_chirho)
                {
                    if let Some(result_ty_chirho) =
                        self.apply_required_type_arg_chirho(&fun_ty_chirho, required_arg_ty_chirho)
                    {
                        return (s1_chirho, result_ty_chirho);
                    }
                }

                if let ExprChirho::LamChirho {
                    pats_chirho,
                    body_chirho,
                    ..
                } = arg_chirho.as_ref()
                {
                    let expected_arg_ty_chirho = self.fresh_var_chirho();
                    let result_ty_chirho = self.fresh_var_chirho();
                    let expected_fun_chirho = TyChirho::fun_chirho(
                        expected_arg_ty_chirho.clone(),
                        result_ty_chirho.clone(),
                    );
                    match self.unify_normalized_chirho(
                        &fun_ty_chirho,
                        &expected_fun_chirho,
                        *span_chirho,
                    ) {
                        Ok(s2_chirho) => {
                            self.apply_subst_all_chirho(&s2_chirho);
                            let expected_arg_sub_chirho =
                                s2_chirho.apply_ty_chirho(&expected_arg_ty_chirho);
                            let (s3_chirho, arg_ty_chirho) = self
                                .infer_lambda_against_expected_chirho(
                                    pats_chirho,
                                    body_chirho,
                                    &expected_arg_sub_chirho,
                                    *span_chirho,
                                )
                                .unwrap_or_else(|| self.infer_expr_chirho(arg_chirho));
                            let arg_ty_sub_chirho = s3_chirho.apply_ty_chirho(&arg_ty_chirho);
                            let expected_arg_final_chirho =
                                s3_chirho.apply_ty_chirho(&expected_arg_sub_chirho);
                            match self.unify_normalized_chirho(
                                &arg_ty_sub_chirho,
                                &expected_arg_final_chirho,
                                *span_chirho,
                            ) {
                                Ok(s4_chirho) => {
                                    let combined_chirho = s4_chirho.compose_chirho(
                                        &s3_chirho
                                            .compose_chirho(&s2_chirho.compose_chirho(&s1_chirho)),
                                    );
                                    let final_ty_chirho =
                                        combined_chirho.apply_ty_chirho(&result_ty_chirho);
                                    self.apply_subst_all_chirho(&s4_chirho);
                                    (combined_chirho, final_ty_chirho)
                                }
                                Err(err_chirho) => {
                                    self.report_unify_error_chirho(&err_chirho);
                                    (
                                        s3_chirho
                                            .compose_chirho(&s2_chirho.compose_chirho(&s1_chirho)),
                                        self.fresh_var_chirho(),
                                    )
                                }
                            }
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                            (s1_chirho, self.fresh_var_chirho())
                        }
                    }
                } else {
                    let (s2_chirho, arg_ty_chirho) = self.infer_expr_chirho(arg_chirho);
                    let fun_ty_sub_chirho = s2_chirho.apply_ty_chirho(&fun_ty_chirho);

                    let result_ty_chirho = self.fresh_var_chirho();
                    let expected_fun_chirho =
                        TyChirho::fun_chirho(arg_ty_chirho, result_ty_chirho.clone());

                    match self.unify_normalized_chirho(
                        &fun_ty_sub_chirho,
                        &expected_fun_chirho,
                        *span_chirho,
                    ) {
                        Ok(s3_chirho) => {
                            let combined_chirho =
                                s3_chirho.compose_chirho(&s2_chirho.compose_chirho(&s1_chirho));
                            let final_ty_chirho =
                                combined_chirho.apply_ty_chirho(&result_ty_chirho);
                            self.apply_subst_all_chirho(&s3_chirho);
                            (combined_chirho, final_ty_chirho)
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                            (
                                s2_chirho.compose_chirho(&s1_chirho),
                                self.fresh_var_chirho(),
                            )
                        }
                    }
                }
            }

            ExprChirho::LamChirho {
                pats_chirho,
                body_chirho,
                span_chirho,
            } => {
                let param_tys_chirho: Vec<TyChirho> = (0..pats_chirho.len())
                    .map(|_| self.fresh_var_chirho())
                    .collect();
                let result_ty_chirho = self.fresh_var_chirho();
                self.infer_lambda_with_seed_chirho(
                    pats_chirho,
                    body_chirho,
                    param_tys_chirho,
                    result_ty_chirho,
                    *span_chirho,
                )
            }

            ExprChirho::TypeLamChirho {
                binder_chirho,
                body_chirho,
                ..
            } => {
                let ty_var_chirho = TyVarChirho(self.next_var_chirho);
                self.next_var_chirho += 1;
                let binder_text_chirho = binder_chirho.text_chirho().to_string();
                let previous_chirho = self
                    .scoped_tyvars_chirho
                    .insert(binder_text_chirho.clone(), ty_var_chirho);
                let (subst_chirho, body_ty_chirho) = self.infer_expr_chirho(body_chirho);
                if let Some(previous_chirho) = previous_chirho {
                    self.scoped_tyvars_chirho
                        .insert(binder_text_chirho, previous_chirho);
                } else {
                    self.scoped_tyvars_chirho.remove(&binder_text_chirho);
                }
                (
                    subst_chirho.clone(),
                    TyChirho::ForallChirho {
                        vars_chirho: vec![ty_var_chirho],
                        body_chirho: Box::new(subst_chirho.apply_ty_chirho(&body_ty_chirho)),
                    },
                )
            }

            ExprChirho::MultiWayIfChirho { arms_chirho, .. } => {
                self.infer_multiway_if_chirho(arms_chirho, None)
            }

            ExprChirho::IfChirho {
                cond_chirho,
                then_chirho,
                else_chirho,
                span_chirho,
            } => self.infer_if_chirho(cond_chirho, then_chirho, else_chirho, *span_chirho),

            ExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                self.env_chirho.push_scope_chirho();
                let mut subst_chirho = SubstChirho::empty_chirho();
                self.infer_local_binds_chirho(binds_chirho, &mut subst_chirho);

                let (body_s_chirho, body_ty_chirho) = self.infer_expr_chirho(body_chirho);
                self.env_chirho.pop_scope_chirho();

                let combined_chirho = body_s_chirho.compose_chirho(&subst_chirho);
                (combined_chirho, body_ty_chirho)
            }

            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut elem_tys_chirho = Vec::new();

                for elem_chirho in elements_chirho {
                    let (s_chirho, ty_chirho) = self.infer_expr_chirho(elem_chirho);
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&s_chirho);
                    elem_tys_chirho.push(ty_chirho);
                }

                // Apply accumulated substitution to element types
                let final_elems_chirho: Vec<TyChirho> = elem_tys_chirho
                    .iter()
                    .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
                    .collect();

                (subst_chirho, TyChirho::TupleChirho(final_elems_chirho))
            }

            ExprChirho::ListChirho {
                elements_chirho,
                span_chirho,
            } => {
                let elem_ty_chirho = self.fresh_var_chirho();
                let mut subst_chirho = SubstChirho::empty_chirho();

                for elem_chirho in elements_chirho {
                    let (s_chirho, ty_chirho) = self.infer_expr_chirho(elem_chirho);
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&s_chirho);

                    let elem_sub_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                    match self.unify_normalized_chirho(&elem_sub_chirho, &ty_chirho, *span_chirho) {
                        Ok(su_chirho) => {
                            subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&su_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }
                }

                let final_elem_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                (
                    subst_chirho,
                    TyChirho::ListChirho(Box::new(final_elem_chirho)),
                )
            }

            ExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                span_chirho,
            } => {
                let (s1_chirho, scrut_ty_chirho) = self.infer_expr_chirho(scrutinee_chirho);
                self.apply_subst_all_chirho(&s1_chirho);

                let result_ty_chirho = self.fresh_var_chirho();
                let mut subst_chirho = s1_chirho;

                for alt_chirho in alts_chirho {
                    self.env_chirho.push_scope_chirho();

                    self.bind_pat_against_chirho(
                        &alt_chirho.pat_chirho,
                        &scrut_ty_chirho,
                        &mut subst_chirho,
                    );

                    self.infer_local_binds_chirho(
                        &alt_chirho.where_binds_chirho,
                        &mut subst_chirho,
                    );

                    let (sr_chirho, alt_ty_chirho) = self.infer_rhs_chirho(&alt_chirho.rhs_chirho);
                    subst_chirho = sr_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&sr_chirho);

                    // Unify alt result with overall result type.
                    // Use lenient unification: if it fails (e.g. GADT
                    // refinement gives different result types per alt),
                    // silently skip — the signature check catches real
                    // mismatches later.
                    let result_sub_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                    if let Ok(sa_chirho) = self.unify_normalized_chirho(
                        &result_sub_chirho,
                        &alt_ty_chirho,
                        *span_chirho,
                    ) {
                        subst_chirho = sa_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&sa_chirho);
                    }

                    self.env_chirho.pop_scope_chirho();
                }

                let final_ty_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                (subst_chirho, final_ty_chirho)
            }

            ExprChirho::DoChirho {
                stmts_chirho,
                span_chirho,
                ..
            } => {
                // Monadic do-notation:
                //   do { expr }       ≡ expr                             (last stmt)
                //   do { expr; rest } ≡ expr >> do { rest }              (ExprStmt)
                //   do { x <- m; rest } ≡ m >>= \x -> do { rest }       (BindStmt)
                //   do { let binds; rest } ≡ let binds in do { rest }    (LetStmt)
                //
                // All monadic expressions in the block share the same monad
                // constructor `m_chirho`. We use a fresh type variable for it
                // and unify as we go. The bind pattern variable `x` gets the
                // unwrapped type `a` from `m a`.
                let m_chirho = self.fresh_var_chirho(); // the monad constructor
                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut last_ty_chirho = TyChirho::unit_chirho();
                let last_expected_chirho = self.do_last_expected_chirho.take();

                self.env_chirho.push_scope_chirho();

                for (idx_chirho, stmt_chirho) in stmts_chirho.iter().enumerate() {
                    let is_last_chirho = idx_chirho == stmts_chirho.len() - 1;

                    match stmt_chirho {
                        StmtChirho::ExprChirho {
                            expr_chirho,
                            then_chirho,
                        } => {
                            let (s_chirho, ty_chirho) =
                                match (&last_expected_chirho, is_last_chirho) {
                                    (Some(expected_chirho), true) => {
                                        let expected_sub_chirho =
                                            subst_chirho.apply_ty_chirho(expected_chirho);
                                        self.infer_expr_against_expected_chirho(
                                            expr_chirho,
                                            &expected_sub_chirho,
                                            *span_chirho,
                                        )
                                    }
                                    _ => self.infer_expr_chirho(expr_chirho),
                                };
                            subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&s_chirho);

                            if is_last_chirho {
                                // Last statement: its type IS the do-block type.
                                // Unify with `m_chirho a` to extract the monad.
                                let a_chirho = self.fresh_var_chirho();
                                let expected_chirho = TyChirho::AppChirho(
                                    Box::new(m_chirho.clone()),
                                    Box::new(a_chirho.clone()),
                                );
                                match self.unify_normalized_chirho(
                                    &subst_chirho.apply_ty_chirho(&ty_chirho),
                                    &subst_chirho.apply_ty_chirho(&expected_chirho),
                                    *span_chirho,
                                ) {
                                    Ok(su_chirho) => {
                                        subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                                        self.apply_subst_all_chirho(&su_chirho);
                                    }
                                    Err(_) => { /* allow fallthrough for simple expressions */ }
                                }
                                last_ty_chirho = ty_chirho;
                            } else {
                                // Non-last expression stmt: type should be `m_chirho _`
                                let discard_chirho = self.fresh_var_chirho();
                                let expected_chirho = TyChirho::AppChirho(
                                    Box::new(m_chirho.clone()),
                                    Box::new(discard_chirho),
                                );
                                match self.unify_normalized_chirho(
                                    &subst_chirho.apply_ty_chirho(&ty_chirho),
                                    &subst_chirho.apply_ty_chirho(&expected_chirho),
                                    *span_chirho,
                                ) {
                                    Ok(su_chirho) => {
                                        subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                                        self.apply_subst_all_chirho(&su_chirho);
                                    }
                                    Err(_) => {}
                                }
                                last_ty_chirho = ty_chirho;
                            }
                            // The `>>` this statement SELECTED (a tail selects
                            // none): its evidence, under its own origin.
                            if let Some(then_selected_chirho) = then_chirho {
                                self.capture_selected_operation_chirho(
                                    then_selected_chirho,
                                    &m_chirho,
                                    &subst_chirho,
                                    *span_chirho,
                                );
                            }
                        }
                        StmtChirho::BindChirho {
                            pat_chirho,
                            expr_chirho,
                            bind_chirho: bind_selected_chirho,
                            fail_chirho: fail_selected_chirho,
                            ..
                        } => {
                            let (s_chirho, ty_chirho) = self.infer_expr_chirho(expr_chirho);
                            subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&s_chirho);

                            // `x <- m`: m has type `M a`, bind x :: a
                            let elem_ty_chirho = self.fresh_var_chirho();
                            let expected_chirho = TyChirho::AppChirho(
                                Box::new(m_chirho.clone()),
                                Box::new(elem_ty_chirho.clone()),
                            );
                            match self.unify_normalized_chirho(
                                &subst_chirho.apply_ty_chirho(&ty_chirho),
                                &subst_chirho.apply_ty_chirho(&expected_chirho),
                                *span_chirho,
                            ) {
                                Ok(su_chirho) => {
                                    subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                                    self.apply_subst_all_chirho(&su_chirho);
                                }
                                Err(_) => {}
                            }

                            // Bind the pattern variable(s) to the unwrapped type
                            let bound_ty_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                            let sp_chirho = self.bind_pat_chirho(pat_chirho, &bound_ty_chirho);
                            subst_chirho = sp_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&sp_chirho);
                            last_ty_chirho = TyChirho::unit_chirho();
                            // The `>>=` this statement SELECTED, and its `fail`
                            // only if one is still selected: the failability pass
                            // cleared it wherever the pattern cannot fail, so no
                            // MonadFail evidence is invented for an irrefutable
                            // pattern.
                            // workflow: language-features-chirho/dictionary-evidence-chirho
                            for selected_chirho in [bind_selected_chirho, fail_selected_chirho]
                                .into_iter()
                                .flatten()
                            {
                                self.capture_selected_operation_chirho(
                                    selected_chirho,
                                    &m_chirho,
                                    &subst_chirho,
                                    *span_chirho,
                                );
                            }
                        }
                        StmtChirho::LetChirho { binds_chirho, .. } => {
                            self.infer_local_binds_chirho(binds_chirho, &mut subst_chirho);
                        }
                    }
                }

                self.env_chirho.pop_scope_chirho();
                (subst_chirho, last_ty_chirho)
            }

            ExprChirho::ListCompChirho {
                body_chirho,
                quals_chirho,
                parallel_quals_chirho,
                span_chirho,
            } => {
                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut body_bindings_chirho = Vec::new();
                if parallel_quals_chirho.is_empty() {
                    let (group_subst_chirho, group_bindings_chirho) =
                        self.infer_list_comp_qual_group_chirho(quals_chirho, *span_chirho);
                    subst_chirho = group_subst_chirho.compose_chirho(&subst_chirho);
                    body_bindings_chirho.extend(group_bindings_chirho);
                } else {
                    for group_chirho in parallel_quals_chirho {
                        let (group_subst_chirho, group_bindings_chirho) =
                            self.infer_list_comp_qual_group_chirho(group_chirho, *span_chirho);
                        subst_chirho = group_subst_chirho.compose_chirho(&subst_chirho);
                        body_bindings_chirho.extend(group_bindings_chirho);
                    }
                }

                self.env_chirho.push_scope_chirho();
                for (name_chirho, scheme_chirho) in body_bindings_chirho {
                    self.env_chirho.bind_chirho(
                        name_chirho,
                        subst_chirho.apply_scheme_chirho(&scheme_chirho),
                    );
                }
                let (body_subst_chirho, body_ty_chirho) = self.infer_expr_chirho(body_chirho);
                subst_chirho = body_subst_chirho.compose_chirho(&subst_chirho);
                self.apply_subst_all_chirho(&body_subst_chirho);
                self.env_chirho.pop_scope_chirho();

                let final_elem_ty_chirho = subst_chirho.apply_ty_chirho(&body_ty_chirho);
                (
                    subst_chirho,
                    TyChirho::ListChirho(Box::new(final_elem_ty_chirho)),
                )
            }

            ExprChirho::NegChirho {
                expr_chirho,
                span_chirho,
            } => {
                let (s_chirho, expr_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                // negate :: Num a => a -> a
                // Defer a Num constraint on the expression type
                self.deferred_preds_chirho.push((
                    PredChirho::new_chirho("Num", expr_ty_chirho.clone()),
                    *span_chirho,
                ));
                (s_chirho, expr_ty_chirho)
            }

            ExprChirho::ParenChirho { inner_chirho, .. } => self.infer_expr_chirho(inner_chirho),

            ExprChirho::LeftSectionChirho {
                op_chirho,
                arg_chirho,
                span_chirho,
            } => {
                let (s1_chirho, arg_ty_chirho) = self.infer_expr_chirho(arg_chirho);
                self.apply_subst_all_chirho(&s1_chirho);

                let (s2_chirho, op_ty_chirho) =
                    self.infer_expr_chirho(&ExprChirho::VarChirho(op_chirho.clone()));
                self.apply_subst_all_chirho(&s2_chirho);

                let left_ty_chirho = self.fresh_var_chirho();
                let result_ty_chirho = self.fresh_var_chirho();
                let arg_ty_sub_chirho = s2_chirho.apply_ty_chirho(&arg_ty_chirho);
                let expected_op_ty_chirho = TyChirho::fun_chirho(
                    left_ty_chirho.clone(),
                    TyChirho::fun_chirho(arg_ty_sub_chirho, result_ty_chirho.clone()),
                );

                match self.unify_normalized_chirho(
                    &s2_chirho.apply_ty_chirho(&op_ty_chirho),
                    &expected_op_ty_chirho,
                    *span_chirho,
                ) {
                    Ok(s3_chirho) => {
                        let combined_chirho =
                            s3_chirho.compose_chirho(&s2_chirho.compose_chirho(&s1_chirho));
                        self.apply_subst_all_chirho(&s3_chirho);
                        let final_ty_chirho = TyChirho::fun_chirho(
                            combined_chirho.apply_ty_chirho(&left_ty_chirho),
                            combined_chirho.apply_ty_chirho(&result_ty_chirho),
                        );
                        (combined_chirho, final_ty_chirho)
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        (
                            s2_chirho.compose_chirho(&s1_chirho),
                            self.fresh_var_chirho(),
                        )
                    }
                }
            }

            ExprChirho::RightSectionChirho {
                arg_chirho,
                op_chirho,
                span_chirho,
            } => {
                let (s1_chirho, arg_ty_chirho) = self.infer_expr_chirho(arg_chirho);
                self.apply_subst_all_chirho(&s1_chirho);

                let (s2_chirho, op_ty_chirho) =
                    self.infer_expr_chirho(&ExprChirho::VarChirho(op_chirho.clone()));
                self.apply_subst_all_chirho(&s2_chirho);

                let right_ty_chirho = self.fresh_var_chirho();
                let result_ty_chirho = self.fresh_var_chirho();
                let arg_ty_sub_chirho = s2_chirho.apply_ty_chirho(&arg_ty_chirho);
                let expected_op_ty_chirho = TyChirho::fun_chirho(
                    arg_ty_sub_chirho,
                    TyChirho::fun_chirho(right_ty_chirho.clone(), result_ty_chirho.clone()),
                );

                match self.unify_normalized_chirho(
                    &s2_chirho.apply_ty_chirho(&op_ty_chirho),
                    &expected_op_ty_chirho,
                    *span_chirho,
                ) {
                    Ok(s3_chirho) => {
                        let combined_chirho =
                            s3_chirho.compose_chirho(&s2_chirho.compose_chirho(&s1_chirho));
                        self.apply_subst_all_chirho(&s3_chirho);
                        let final_ty_chirho = TyChirho::fun_chirho(
                            combined_chirho.apply_ty_chirho(&right_ty_chirho),
                            combined_chirho.apply_ty_chirho(&result_ty_chirho),
                        );
                        (combined_chirho, final_ty_chirho)
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        (
                            s2_chirho.compose_chirho(&s1_chirho),
                            self.fresh_var_chirho(),
                        )
                    }
                }
            }

            ExprChirho::AnnChirho {
                expr_chirho,
                ty_chirho: ann_ty_chirho,
                span_chirho,
            } => {
                // Type annotation: infer the expression, then check against annotation
                // using subsumption to handle higher-rank annotations properly
                let (s_chirho, inferred_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                self.apply_subst_all_chirho(&s_chirho);

                // ScopedTypeVariables: a variable of an enclosing signature
                // names that signature's rigid variable here.
                let ann_internal_chirho = self.ast_type_to_ty_in_scope_chirho(ann_ty_chirho);
                let ann_normalized_chirho = self.normalize_ty_chirho(&ann_internal_chirho);
                let inferred_normalized_chirho = self.normalize_ty_chirho(&inferred_ty_chirho);

                match self.subsume_normalized_chirho(
                    &inferred_normalized_chirho,
                    &ann_normalized_chirho,
                    *span_chirho,
                ) {
                    Ok(su_chirho) => {
                        let combined_chirho = su_chirho.compose_chirho(&s_chirho);
                        let final_ty_chirho = combined_chirho.apply_ty_chirho(&ann_internal_chirho);
                        self.apply_subst_all_chirho(&su_chirho);
                        (combined_chirho, final_ty_chirho)
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        (s_chirho, ann_internal_chirho)
                    }
                }
            }

            // Infix application: `a op b` is treated as `(op a) b`
            ExprChirho::InfixChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                span_chirho,
            } => {
                let app_expr_chirho =
                    build_infix_app_expr_chirho(left_chirho, op_chirho, right_chirho, *span_chirho);
                self.infer_expr_chirho(&app_expr_chirho)
            }

            // Record construction: Con { f1 = e1, f2 = e2, ... }
            // Treated as constructor application: infer constructor type,
            // then unify each field value with the successive argument types.
            ExprChirho::RecordConChirho {
                con_chirho,
                fields_chirho,
                has_wildcard_chirho,
                span_chirho,
            } => {
                let con_text_chirho = con_chirho.text_chirho();
                let con_full_name_chirho = con_chirho.full_name_chirho();
                let scheme_opt_chirho = self
                    .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                        &con_full_name_chirho,
                        con_text_chirho,
                    );
                match scheme_opt_chirho {
                    Some(scheme_chirho) => {
                        let con_span_chirho = con_chirho.span_chirho();
                        let mut con_ty_chirho =
                            self.instantiate_chirho(&scheme_chirho, con_span_chirho);
                        let mut combined_chirho = SubstChirho::empty_chirho();
                        if let Some((ordered_field_names_chirho, _, _)) =
                            self.lookup_record_constructor_field_bundle_chirho(con_chirho)
                        {
                            // workflow: language-features-chirho/rigid-type-variables-chirho (records)
                            self.report_record_construction_fields_chirho(
                                con_text_chirho,
                                &ordered_field_names_chirho,
                                fields_chirho,
                                *has_wildcard_chirho,
                                *span_chirho,
                            );
                            for field_name_chirho in &ordered_field_names_chirho {
                                let matched_field_chirho =
                                    fields_chirho.iter().find(|field_chirho| {
                                        strip_name_qualifier_chirho(
                                            &field_chirho.name_chirho.full_name_chirho(),
                                        ) == field_name_chirho
                                    });
                                let (s_chirho, field_ty_chirho) = if let Some(field_chirho) =
                                    matched_field_chirho
                                {
                                    self.infer_record_constructor_field_chirho(
                                        &field_chirho.value_chirho,
                                        &con_ty_chirho,
                                        field_chirho.span_chirho,
                                    )
                                } else if *has_wildcard_chirho {
                                    let var_scheme_chirho =
                                        self.env_chirho.lookup_chirho(field_name_chirho).cloned();
                                    if let Some(scheme_chirho) = var_scheme_chirho {
                                        (
                                            SubstChirho::empty_chirho(),
                                            self.instantiate_chirho(&scheme_chirho, *span_chirho),
                                        )
                                    } else {
                                        self.diagnostics_chirho.push_chirho(
                                            DiagnosticChirho::error_with_code_chirho(
                                                ErrorCodeChirho::error_chirho(200),
                                                format!(
                                                    "RecordWildCards: variable `{}` not in scope",
                                                    field_name_chirho
                                                ),
                                                *span_chirho,
                                            ),
                                        );
                                        (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                                    }
                                } else {
                                    // Haskell record construction is keyed by field label, not by
                                    // source order. Consume omitted fields with fresh types so
                                    // later named fields still align to the constructor layout.
                                    (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                                };
                                combined_chirho = s_chirho.compose_chirho(&combined_chirho);
                                con_ty_chirho = combined_chirho.apply_ty_chirho(&con_ty_chirho);
                                let result_ty_chirho = self.fresh_var_chirho();
                                let expected_chirho = TyChirho::FunChirho(
                                    Box::new(field_ty_chirho),
                                    Box::new(result_ty_chirho.clone()),
                                    MultChirho::ManyChirho,
                                );
                                match self.unify_normalized_chirho(
                                    &con_ty_chirho,
                                    &expected_chirho,
                                    *span_chirho,
                                ) {
                                    Ok(s2_chirho) => {
                                        self.apply_subst_all_chirho(&s2_chirho);
                                        combined_chirho =
                                            s2_chirho.compose_chirho(&combined_chirho);
                                        con_ty_chirho =
                                            combined_chirho.apply_ty_chirho(&result_ty_chirho);
                                    }
                                    Err(err_chirho) => {
                                        self.report_unify_error_chirho(&err_chirho);
                                        return (combined_chirho, self.fresh_var_chirho());
                                    }
                                }
                            }
                            (combined_chirho, con_ty_chirho)
                        } else {
                            // workflow: language-features-chirho/rigid-type-variables-chirho (records)
                            // No declared field set. `C {}` supplies nothing, so every
                            // argument is a missing field and the construction has the
                            // constructor's result type; named fields on a local
                            // positional constructor are all undeclared (E0206).
                            if fields_chirho.is_empty() && !*has_wildcard_chirho {
                                self.report_omitted_strict_positional_chirho(
                                    con_text_chirho,
                                    *span_chirho,
                                );
                                let result_ty_chirho =
                                    self.constructor_result_type_chirho(con_ty_chirho);
                                return (combined_chirho, result_ty_chirho);
                            }
                            if self.report_fields_on_positional_constructor_chirho(
                                con_text_chirho,
                                fields_chirho,
                            ) {
                                let result_ty_chirho =
                                    self.constructor_result_type_chirho(con_ty_chirho);
                                return (combined_chirho, result_ty_chirho);
                            }
                            for field_chirho in fields_chirho {
                                let (s_chirho, field_ty_chirho) = self
                                    .infer_record_constructor_field_chirho(
                                        &field_chirho.value_chirho,
                                        &con_ty_chirho,
                                        field_chirho.span_chirho,
                                    );
                                combined_chirho = s_chirho.compose_chirho(&combined_chirho);
                                con_ty_chirho = combined_chirho.apply_ty_chirho(&con_ty_chirho);
                                let result_ty_chirho = self.fresh_var_chirho();
                                let expected_chirho = TyChirho::FunChirho(
                                    Box::new(field_ty_chirho),
                                    Box::new(result_ty_chirho.clone()),
                                    MultChirho::ManyChirho,
                                );
                                match self.unify_normalized_chirho(
                                    &con_ty_chirho,
                                    &expected_chirho,
                                    *span_chirho,
                                ) {
                                    Ok(s2_chirho) => {
                                        self.apply_subst_all_chirho(&s2_chirho);
                                        combined_chirho =
                                            s2_chirho.compose_chirho(&combined_chirho);
                                        con_ty_chirho =
                                            combined_chirho.apply_ty_chirho(&result_ty_chirho);
                                    }
                                    Err(err_chirho) => {
                                        self.report_unify_error_chirho(&err_chirho);
                                        return (combined_chirho, self.fresh_var_chirho());
                                    }
                                }
                            }
                            (combined_chirho, con_ty_chirho)
                        }
                    }
                    None => {
                        // A record construction needs a data constructor in scope;
                        // `Int{}` names a type, not a constructor (T23739c). The
                        // Var arm reports a scheme-less name the same way.
                        // workflow: language-features-chirho/rigid-type-variables-chirho (records)
                        self.diagnostics_chirho.push_chirho(
                            DiagnosticChirho::error_with_code_chirho(
                                ErrorCodeChirho::error_chirho(UNBOUND_VAR_CODE_CHIRHO),
                                format!("unbound constructor: `{con_text_chirho}`"),
                                con_chirho.span_chirho(),
                            ),
                        );
                        (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                    }
                }
            }

            // Record update: expr { f1 = e1, ... }
            // For now, infer the expression type and pass through
            ExprChirho::RecordUpdateChirho {
                expr_chirho,
                fields_chirho,
                span_chirho,
            } => {
                let (s1_chirho, base_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                let mut combined_chirho = s1_chirho;
                self.apply_subst_all_chirho(&combined_chirho);
                let base_ty_chirho = combined_chirho.apply_ty_chirho(&base_ty_chirho);
                if let Some((s_chirho, result_ty_chirho)) = self.infer_record_update_fields_chirho(
                    &base_ty_chirho,
                    fields_chirho,
                    *span_chirho,
                ) {
                    combined_chirho = s_chirho.compose_chirho(&combined_chirho);
                    return (combined_chirho, result_ty_chirho);
                }
                // No constructor declares all the updated fields. When the
                // record type is one this module declares, that is GHC's
                // "constructor does not have field" error; an imported or
                // unknown type keeps the permissive path below.
                // workflow: language-features-chirho/rigid-type-variables-chirho (records)
                self.report_undeclared_record_fields_chirho(&base_ty_chirho, fields_chirho);
                // Unknown constructor layout: type-check the values and keep the
                // record's type.
                for field_chirho in fields_chirho {
                    let (fs_chirho, _field_ty_chirho) =
                        self.infer_expr_chirho(&field_chirho.value_chirho);
                    combined_chirho = fs_chirho.compose_chirho(&combined_chirho);
                    self.apply_subst_all_chirho(&fs_chirho);
                }
                (combined_chirho, base_ty_chirho)
            }

            // TypeApplications: infer the inner expression, then unify
            // the result type with the provided type argument so that the
            // type annotation constrains polymorphic instantiation.
            ExprChirho::TypeAppChirho {
                expr_chirho: inner_expr_chirho,
                ty_chirho,
                span_chirho,
            } => {
                if let Some(result_chirho) =
                    self.infer_named_type_app_chirho(expr_chirho, *span_chirho)
                {
                    return result_chirho;
                }

                let (s1_chirho, inferred_chirho) = self.infer_expr_chirho(inner_expr_chirho);
                let target_chirho = self.ast_type_to_ty_in_scope_chirho(ty_chirho);
                let inferred_sub_chirho = s1_chirho.apply_ty_chirho(&inferred_chirho);
                if let Some(result_chirho) =
                    self.apply_visible_type_arg_chirho(&inferred_sub_chirho, target_chirho.clone())
                {
                    return (s1_chirho, result_chirho);
                }
                match self.unify_normalized_chirho(&inferred_chirho, &target_chirho, *span_chirho) {
                    Ok(s2_chirho) => {
                        let composed_chirho = s2_chirho.compose_chirho(&s1_chirho);
                        let result_chirho = composed_chirho.apply_ty_chirho(&inferred_chirho);
                        (composed_chirho, result_chirho)
                    }
                    Err(_) => {
                        // If unification fails (e.g. applying @Int to a
                        // monomorphic String), just keep the inferred type.
                        // The type argument served as documentation.
                        (s1_chirho, inferred_chirho)
                    }
                }
            }

            // For remaining expression forms, return a fresh variable
            _ => (SubstChirho::empty_chirho(), self.fresh_var_chirho()),
        }
    }

    /// Infer one sequential branch of a list comprehension in isolation.
    ///
    /// ParallelListComp branches share their outer environment, but a branch
    /// cannot see names introduced by a sibling. The returned bindings are
    /// reintroduced together only while inferring the result expression.
    ///
    /// workflow: parallel-list-comprehensions-chirho
    fn infer_list_comp_qual_group_chirho(
        &mut self,
        quals_chirho: &[StmtChirho],
        span_chirho: SpanChirho,
    ) -> (SubstChirho, Vec<(String, SchemeChirho)>) {
        let mut subst_chirho = SubstChirho::empty_chirho();
        let mut bound_names_chirho = Vec::new();
        let mut seen_names_chirho = HashSet::new();
        self.env_chirho.push_scope_chirho();

        for qual_chirho in quals_chirho {
            match qual_chirho {
                StmtChirho::ExprChirho { expr_chirho, .. } => {
                    let (guard_subst_chirho, guard_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                    subst_chirho = guard_subst_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&guard_subst_chirho);
                    let guard_ty_sub_chirho = subst_chirho.apply_ty_chirho(&guard_ty_chirho);
                    if let Ok(unify_subst_chirho) = self.unify_normalized_chirho(
                        &guard_ty_sub_chirho,
                        &TyChirho::bool_chirho(),
                        span_chirho,
                    ) {
                        subst_chirho = unify_subst_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&unify_subst_chirho);
                    }
                }
                StmtChirho::BindChirho {
                    pat_chirho,
                    expr_chirho,
                    ..
                } => {
                    let (source_subst_chirho, source_ty_chirho) =
                        self.infer_expr_chirho(expr_chirho);
                    subst_chirho = source_subst_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&source_subst_chirho);

                    let elem_ty_chirho = self.fresh_var_chirho();
                    let list_ty_chirho = TyChirho::ListChirho(Box::new(elem_ty_chirho.clone()));
                    let source_ty_sub_chirho = subst_chirho.apply_ty_chirho(&source_ty_chirho);
                    if let Ok(unify_subst_chirho) = self.unify_normalized_chirho(
                        &source_ty_sub_chirho,
                        &list_ty_chirho,
                        span_chirho,
                    ) {
                        subst_chirho = unify_subst_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&unify_subst_chirho);
                    }

                    let bound_ty_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                    let pat_subst_chirho = self.bind_pat_chirho(pat_chirho, &bound_ty_chirho);
                    subst_chirho = pat_subst_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&pat_subst_chirho);
                    for name_chirho in crate::linearity_chirho::pat_bound_names_chirho(pat_chirho) {
                        if seen_names_chirho.insert(name_chirho.clone()) {
                            bound_names_chirho.push(name_chirho);
                        }
                    }
                }
                StmtChirho::LetChirho { binds_chirho, .. } => {
                    self.infer_local_binds_chirho(binds_chirho, &mut subst_chirho);
                    for bind_chirho in binds_chirho {
                        match bind_chirho {
                            haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                                name_chirho,
                                ..
                            } => {
                                let name_chirho = canonical_value_name_text_chirho(
                                    name_chirho.text_chirho(),
                                );
                                if seen_names_chirho.insert(name_chirho.clone()) {
                                    bound_names_chirho.push(name_chirho);
                                }
                            }
                            haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                                pat_chirho,
                                ..
                            } => {
                                for name_chirho in
                                    crate::linearity_chirho::pat_bound_names_chirho(pat_chirho)
                                {
                                    if seen_names_chirho.insert(name_chirho.clone()) {
                                        bound_names_chirho.push(name_chirho);
                                    }
                                }
                            }
                            haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                                ..
                            } => {}
                        }
                    }
                }
            }
        }

        let bindings_chirho = bound_names_chirho
            .into_iter()
            .filter_map(|name_chirho| {
                self.env_chirho
                    .lookup_chirho(&name_chirho)
                    .cloned()
                    .map(|scheme_chirho| {
                        (
                            name_chirho,
                            subst_chirho.apply_scheme_chirho(&scheme_chirho),
                        )
                    })
            })
            .collect();
        self.env_chirho.pop_scope_chirho();
        (subst_chirho, bindings_chirho)
    }

    // -----------------------------------------------------------------------
    // Pattern binding
    // -----------------------------------------------------------------------

    /// Pre-bind all variable names in a pattern with fresh type variables.
    /// This enables mutually recursive `where`/`let` pattern bindings
    /// (letrec semantics), e.g. `(x,y) = (y+1, x+2)`.
    fn pre_bind_pat_vars_chirho(&mut self, pat_chirho: &PatChirho) {
        match pat_chirho {
            PatChirho::VarChirho(name_chirho) => {
                let fresh_chirho = self.fresh_var_chirho();
                self.env_chirho.bind_chirho(
                    name_chirho.text_chirho().to_string(),
                    SchemeChirho::mono_chirho(fresh_chirho),
                );
            }
            PatChirho::TupleChirho {
                elements_chirho, ..
            } => {
                for elem_chirho in elements_chirho {
                    self.pre_bind_pat_vars_chirho(elem_chirho);
                }
            }
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho,
                ..
            } => {
                let fresh_chirho = self.fresh_var_chirho();
                self.env_chirho.bind_chirho(
                    name_chirho.text_chirho().to_string(),
                    SchemeChirho::mono_chirho(fresh_chirho),
                );
                self.pre_bind_pat_vars_chirho(pattern_chirho);
            }
            PatChirho::ConChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    self.pre_bind_pat_vars_chirho(arg_chirho);
                }
            }
            PatChirho::InfixConChirho {
                left_chirho,
                right_chirho,
                ..
            } => {
                self.pre_bind_pat_vars_chirho(left_chirho);
                self.pre_bind_pat_vars_chirho(right_chirho);
            }
            PatChirho::ListChirho {
                elements_chirho, ..
            } => {
                for elem_chirho in elements_chirho {
                    self.pre_bind_pat_vars_chirho(elem_chirho);
                }
            }
            PatChirho::ParenChirho { inner_chirho, .. }
            | PatChirho::LazyChirho { inner_chirho, .. }
            | PatChirho::BangChirho { inner_chirho, .. } => {
                self.pre_bind_pat_vars_chirho(inner_chirho);
            }
            PatChirho::RecordChirho { fields_chirho, .. } => {
                for field_chirho in fields_chirho {
                    self.pre_bind_pat_vars_chirho(&field_chirho.pattern_chirho);
                }
            }
            PatChirho::ViewChirho {
                pat_chirho: inner_chirho,
                ..
            }
            | PatChirho::TypeAnnotChirho {
                pat_chirho: inner_chirho,
                ..
            } => {
                self.pre_bind_pat_vars_chirho(inner_chirho);
            }
            // Wildcards, literals, negated literals: no bindings
            _ => {}
        }
    }

    // -----------------------------------------------------------------------
    // RHS and match arm inference
    // -----------------------------------------------------------------------

    /// Infer the type of a right-hand side.
    fn infer_rhs_chirho(&mut self, rhs_chirho: &RhsChirho) -> (SubstChirho, TyChirho) {
        match rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => self.infer_expr_chirho(expr_chirho),
            RhsChirho::GuardedChirho(guarded_chirho) => {
                self.infer_guard_arms_chirho(guarded_chirho)
            }
        }
    }

    fn infer_rhs_against_expected_chirho(
        &mut self,
        rhs_chirho: &RhsChirho,
        expected_ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        match rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => self.infer_expr_against_expected_chirho(
                expr_chirho,
                expected_ty_chirho,
                span_chirho,
            ),
            RhsChirho::GuardedChirho(guarded_chirho) => {
                self.infer_guard_arms_against_expected_chirho(guarded_chirho, expected_ty_chirho)
            }
        }
    }

    fn infer_local_binds_chirho(
        &mut self,
        binds_chirho: &[haskelujah_ast_chirho::expr_chirho::LocalBindChirho],
        subst_chirho: &mut SubstChirho,
    ) {
        let mut local_sigs_chirho: HashMap<String, haskelujah_ast_chirho::ty_chirho::TypeChirho> =
            HashMap::new();
        for bind_chirho in binds_chirho {
            if let haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } = bind_chirho
            {
                local_sigs_chirho.insert(
                    canonical_value_name_text_chirho(name_chirho.text_chirho()),
                    ty_chirho.clone(),
                );
            }
        }

        let mut fun_names_chirho: Vec<String> = Vec::new();
        let mut fun_matches_refs_chirho: Vec<&[MatchArmChirho]> = Vec::new();
        let mut fun_spans_chirho: Vec<SpanChirho> = Vec::new();
        let mut fun_pre_tys_chirho: Vec<TyChirho> = Vec::new();
        let mut local_skolemized_sigs_chirho: HashMap<String, TyChirho> = HashMap::new();

        for bind_chirho in binds_chirho {
            match bind_chirho {
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                } => {
                    let name_str_chirho =
                        canonical_value_name_text_chirho(name_chirho.text_chirho());
                    let pre_ty_chirho =
                        if let Some(sig_ast_chirho) = local_sigs_chirho.get(&name_str_chirho) {
                            let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                            let (sig_ty_chirho, _sig_preds_chirho) =
                                self.instantiate_scheme_parts_chirho(&sig_scheme_chirho);
                            self.env_chirho
                                .bind_chirho(name_str_chirho.clone(), sig_scheme_chirho);
                            sig_ty_chirho
                        } else {
                            let fresh_ty_chirho = self.fresh_var_chirho();
                            self.env_chirho.bind_chirho(
                                name_str_chirho.clone(),
                                SchemeChirho::mono_chirho(fresh_ty_chirho.clone()),
                            );
                            fresh_ty_chirho
                        };
                    fun_names_chirho.push(name_str_chirho);
                    fun_matches_refs_chirho.push(matches_chirho.as_slice());
                    fun_spans_chirho.push(*span_chirho);
                    fun_pre_tys_chirho.push(pre_ty_chirho);
                }
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    pat_chirho,
                    ..
                } => {
                    self.pre_bind_pat_vars_chirho(pat_chirho);
                }
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho { .. } => {}
            }
        }

        let fun_decls_for_scc_chirho: Vec<(usize, &[MatchArmChirho])> = fun_matches_refs_chirho
            .iter()
            .enumerate()
            .map(|(index_chirho, matches_chirho)| (index_chirho, *matches_chirho))
            .collect();
        let signed_names_chirho: HashSet<String> = if self.relaxed_poly_rec_chirho {
            local_sigs_chirho.keys().cloned().collect()
        } else {
            HashSet::new()
        };
        let groups_chirho = binding_groups_chirho(
            &fun_names_chirho,
            &fun_decls_for_scc_chirho,
            &signed_names_chirho,
        );

        for group_chirho in &groups_chirho {
            let mut group_inferred_chirho: Vec<(String, TyChirho, SpanChirho)> = Vec::new();
            for &fun_index_chirho in group_chirho {
                let name_str_chirho = fun_names_chirho[fun_index_chirho].clone();
                let span_chirho = fun_spans_chirho[fun_index_chirho];
                let prev_scoped_chirho = self.scoped_tyvars_chirho.clone();

                let (fun_subst_chirho, inferred_ty_chirho) =
                    if let Some(sig_ast_chirho) = local_sigs_chirho.get(&name_str_chirho) {
                        let (sig_scheme_chirho, sig_var_map_chirho) =
                            self.ast_type_to_scheme_with_var_map_chirho(sig_ast_chirho);
                        let (sig_ty_raw_chirho, sig_preds_chirho) =
                            self.skolemize_scheme_parts_chirho(&sig_scheme_chirho);
                        self.record_own_dictionary_indices_chirho(&sig_preds_chirho);
                        self.scope_signature_for_body_chirho(sig_ast_chirho, &sig_var_map_chirho);
                        local_skolemized_sigs_chirho
                            .insert(name_str_chirho.clone(), sig_ty_raw_chirho.clone());
                        self.with_given_preds_chirho(sig_preds_chirho, |self_chirho| {
                            self_chirho.infer_matches_against_expected_chirho(
                                fun_matches_refs_chirho[fun_index_chirho],
                                span_chirho,
                                &sig_ty_raw_chirho,
                            )
                        })
                    } else {
                        self.infer_matches_chirho(
                            fun_matches_refs_chirho[fun_index_chirho],
                            span_chirho,
                        )
                    };
                *subst_chirho = fun_subst_chirho.compose_chirho(subst_chirho);
                self.apply_subst_all_chirho(&fun_subst_chirho);

                let pre_ty_sub_chirho =
                    subst_chirho.apply_ty_chirho(&fun_pre_tys_chirho[fun_index_chirho]);
                if let Ok(unify_subst_chirho) = self.unify_normalized_chirho(
                    &pre_ty_sub_chirho,
                    &inferred_ty_chirho,
                    span_chirho,
                ) {
                    *subst_chirho = unify_subst_chirho.compose_chirho(subst_chirho);
                    self.apply_subst_all_chirho(&unify_subst_chirho);
                }

                group_inferred_chirho.push((name_str_chirho, inferred_ty_chirho, span_chirho));
                self.scoped_tyvars_chirho = prev_scoped_chirho;
            }

            for (name_str_chirho, inferred_ty_chirho, span_chirho) in group_inferred_chirho {
                let mut binding_scheme_chirho = None;
                if let Some(sig_ast_chirho) = local_sigs_chirho.get(&name_str_chirho) {
                    let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                    let sig_full_chirho = if sig_scheme_chirho.vars_chirho.is_empty() {
                        sig_scheme_chirho.ty_chirho.clone()
                    } else {
                        TyChirho::ForallChirho {
                            vars_chirho: sig_scheme_chirho.vars_chirho.clone(),
                            body_chirho: Box::new(sig_scheme_chirho.ty_chirho.clone()),
                        }
                    };
                    let sig_ty_chirho = match local_skolemized_sigs_chirho.get(&name_str_chirho) {
                        Some(skolemized_chirho) => self
                            .normalize_ty_chirho(&subst_chirho.apply_ty_chirho(skolemized_chirho)),
                        None => self.normalize_ty_chirho(&sig_full_chirho),
                    };
                    let inferred_sub_chirho = subst_chirho.apply_ty_chirho(&inferred_ty_chirho);
                    let inferred_sub_chirho = self.normalize_ty_chirho(&inferred_sub_chirho);
                    match self.subsume_normalized_chirho(
                        &inferred_sub_chirho,
                        &sig_ty_chirho,
                        span_chirho,
                    ) {
                        Ok(sig_subst_chirho) => {
                            *subst_chirho = sig_subst_chirho.compose_chirho(subst_chirho);
                            self.apply_subst_all_chirho(&sig_subst_chirho);
                            binding_scheme_chirho = Some(sig_scheme_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }
                }

                self.env_chirho.remove_chirho(&name_str_chirho);
                let inferred_sub_chirho = subst_chirho.apply_ty_chirho(&inferred_ty_chirho);
                let generalized_chirho = binding_scheme_chirho
                    .unwrap_or_else(|| self.generalize_local_chirho(&inferred_sub_chirho));
                self.local_generalized_vars_chirho
                    .extend(generalized_chirho.vars_chirho.iter().copied());
                self.env_chirho
                    .bind_chirho(name_str_chirho, generalized_chirho);
            }
        }

        for bind_chirho in binds_chirho {
            match bind_chirho {
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    pat_chirho,
                    rhs_chirho,
                    ..
                } => {
                    let (rhs_subst_chirho, rhs_ty_chirho) = self.infer_rhs_chirho(rhs_chirho);
                    *subst_chirho = rhs_subst_chirho.compose_chirho(subst_chirho);
                    self.apply_subst_all_chirho(&rhs_subst_chirho);
                    let pat_subst_chirho = self.bind_pat_chirho(pat_chirho, &rhs_ty_chirho);
                    *subst_chirho = pat_subst_chirho.compose_chirho(subst_chirho);
                    self.apply_subst_all_chirho(&pat_subst_chirho);
                }
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho { .. }
                | haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho { .. } => {}
            }
        }
    }

    /// Infer the type of a set of match arms (function equations).
    fn freshen_match_seed_and_givens_chirho(
        &mut self,
        param_tys_chirho: &[TyChirho],
        result_ty_chirho: &TyChirho,
        preds_chirho: &[PredChirho],
    ) -> Option<(Vec<TyChirho>, TyChirho, Vec<PredChirho>)> {
        let seed_fun_ty_chirho =
            TyChirho::fun_n_chirho(param_tys_chirho.to_vec(), result_ty_chirho.clone());
        let mut seed_vars_chirho: Vec<TyVarChirho> =
            seed_fun_ty_chirho.free_vars_chirho().into_iter().collect();
        for pred_chirho in preds_chirho {
            seed_vars_chirho.extend(pred_chirho.ty_chirho.free_vars_chirho());
            for extra_ty_chirho in &pred_chirho.extra_tys_chirho {
                seed_vars_chirho.extend(extra_ty_chirho.free_vars_chirho());
            }
        }
        seed_vars_chirho.sort_by_key(|tv_chirho| tv_chirho.0);
        seed_vars_chirho.dedup();
        let seed_scheme_chirho = SchemeChirho {
            vars_chirho: seed_vars_chirho,
            preds_chirho: preds_chirho
                .iter()
                .map(|pred_chirho| SchemePredChirho {
                    class_name_chirho: pred_chirho.class_name_chirho.clone(),
                    ty_chirho: pred_chirho.ty_chirho.clone(),
                    extra_tys_chirho: pred_chirho.extra_tys_chirho.clone(),
                })
                .collect(),
            ty_chirho: seed_fun_ty_chirho,
        };
        let (fresh_fun_ty_chirho, fresh_preds_chirho) =
            self.instantiate_scheme_parts_chirho(&seed_scheme_chirho);
        let (fresh_params_chirho, fresh_result_chirho) =
            Self::split_fun_ty_for_arity_chirho(&fresh_fun_ty_chirho, param_tys_chirho.len())?;
        Some((fresh_params_chirho, fresh_result_chirho, fresh_preds_chirho))
    }

    fn infer_matches_with_seed_chirho(
        &mut self,
        matches_chirho: &[MatchArmChirho],
        span_chirho: SpanChirho,
        param_tys_chirho: Vec<TyChirho>,
        result_ty_chirho: TyChirho,
        authoritative_seed_chirho: bool,
    ) -> (SubstChirho, TyChirho) {
        if matches_chirho.is_empty() {
            return (SubstChirho::empty_chirho(), self.fresh_var_chirho());
        }

        let first_chirho = &matches_chirho[0];
        let arity_chirho = first_chirho.pats_chirho.len();
        let mut subst_chirho = SubstChirho::empty_chirho();
        let signature_givens_chirho = if authoritative_seed_chirho {
            self.given_preds_chirho.clone()
        } else {
            Vec::new()
        };

        for match_arm_chirho in matches_chirho {
            self.env_chirho.push_scope_chirho();

            // Each equation gets its own fresh param/result types so GADT
            // pattern matching can refine types independently per equation.
            // For explicit signatures / expected types, freshen the entire
            // seeded function type per equation so relationships like
            // `ASeq f u -> f u` stay linked within that equation, but do not
            // leak refinements such as `u ~ ()` from one equation into the next.
            let (eq_param_tys_chirho, eq_result_ty_chirho, eq_given_preds_chirho) =
                if authoritative_seed_chirho {
                    self.freshen_match_seed_and_givens_chirho(
                        &param_tys_chirho,
                        &result_ty_chirho,
                        &signature_givens_chirho,
                    )
                    .unwrap_or_else(|| {
                        let fresh_params_chirho: Vec<TyChirho> =
                            (0..arity_chirho).map(|_| self.fresh_var_chirho()).collect();
                        let fresh_result_chirho = self.fresh_var_chirho();
                        (fresh_params_chirho, fresh_result_chirho, Vec::new())
                    })
                } else {
                    (
                        (0..arity_chirho).map(|_| self.fresh_var_chirho()).collect(),
                        self.fresh_var_chirho(),
                        Vec::new(),
                    )
                };
            let given_start_chirho = self.given_preds_chirho.len();
            if authoritative_seed_chirho {
                self.given_preds_chirho.extend(eq_given_preds_chirho);
            }

            // Bind pattern variables against per-equation fresh types
            for (pat_chirho, pat_ty_chirho) in match_arm_chirho
                .pats_chirho
                .iter()
                .zip(eq_param_tys_chirho.iter())
            {
                let pat_ty_sub_chirho = subst_chirho.apply_ty_chirho(pat_ty_chirho);
                let sp_chirho = self.bind_pat_chirho(pat_chirho, &pat_ty_sub_chirho);
                subst_chirho = sp_chirho.compose_chirho(&subst_chirho);
                self.apply_subst_all_chirho(&sp_chirho);
            }

            self.infer_local_binds_chirho(&match_arm_chirho.where_binds_chirho, &mut subst_chirho);

            // Infer RHS
            let expected_result_sub_chirho = if authoritative_seed_chirho {
                subst_chirho.apply_ty_chirho(&eq_result_ty_chirho)
            } else {
                subst_chirho.apply_ty_chirho(&result_ty_chirho)
            };
            let (sr_chirho, rhs_ty_chirho) = self.infer_rhs_against_expected_chirho(
                &match_arm_chirho.rhs_chirho,
                &expected_result_sub_chirho,
                span_chirho,
            );
            subst_chirho = sr_chirho.compose_chirho(&subst_chirho);
            self.apply_subst_all_chirho(&sr_chirho);

            // Unify RHS with per-equation result type
            let eq_result_sub_chirho = subst_chirho.apply_ty_chirho(&eq_result_ty_chirho);
            match self.unify_normalized_chirho(&eq_result_sub_chirho, &rhs_ty_chirho, span_chirho) {
                Ok(su_chirho) => {
                    subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&su_chirho);
                }
                Err(err_chirho) => {
                    self.report_unify_error_chirho(&err_chirho);
                }
            }

            if !authoritative_seed_chirho {
                // Unify per-equation function type with overall function type.
                // This propagates non-GADT constraints while allowing GADT
                // equations to have independently refined types.
                for (eq_p_chirho, p_chirho) in
                    eq_param_tys_chirho.iter().zip(param_tys_chirho.iter())
                {
                    let eq_p_sub_chirho = subst_chirho.apply_ty_chirho(eq_p_chirho);
                    let p_sub_chirho = subst_chirho.apply_ty_chirho(p_chirho);
                    if let Ok(s_chirho) =
                        self.unify_normalized_chirho(&eq_p_sub_chirho, &p_sub_chirho, span_chirho)
                    {
                        subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&s_chirho);
                    }
                }
                // Unify per-equation result with overall result type.
                // For non-GADT cases this propagates the body type to the
                // function result; for GADTs the signature check provides
                // the authoritative type.
                let eq_r_sub_chirho = subst_chirho.apply_ty_chirho(&eq_result_ty_chirho);
                let r_sub_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                match self.unify_normalized_chirho(&eq_r_sub_chirho, &r_sub_chirho, span_chirho) {
                    Ok(s_chirho) => {
                        subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&s_chirho);
                    }
                    Err(err_chirho) => {
                        // For arity-0 bindings (e.g. x = True) there are no
                        // GADT patterns to refine types, so a mismatch between
                        // inferred and expected result is a real type error.
                        // For higher-arity matches, GADT pattern matching can
                        // refine per-equation types, so we allow the mismatch.
                        if arity_chirho == 0 {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }
                }
            }

            self.env_chirho.pop_scope_chirho();
            let improvement_chirho = self.discharge_deferred_with_current_givens_chirho();
            if !improvement_chirho.is_empty_chirho() {
                subst_chirho = improvement_chirho.compose_chirho(&subst_chirho);
            }
            let equalities_chirho = self.retry_deferred_equalities_chirho();
            if !equalities_chirho.is_empty_chirho() {
                subst_chirho = equalities_chirho.compose_chirho(&subst_chirho);
            }
            self.given_preds_chirho.truncate(given_start_chirho);
        }

        // Build the function type: param1 -> param2 -> ... -> result
        let final_params_chirho: Vec<TyChirho> = param_tys_chirho
            .iter()
            .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
            .collect();
        let final_result_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
        let fun_ty_chirho = TyChirho::fun_n_chirho(final_params_chirho, final_result_chirho);

        (subst_chirho, fun_ty_chirho)
    }

    fn infer_matches_chirho(
        &mut self,
        matches_chirho: &[MatchArmChirho],
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        if matches_chirho.is_empty() {
            return (SubstChirho::empty_chirho(), self.fresh_var_chirho());
        }

        let arity_chirho = matches_chirho[0].pats_chirho.len();
        let param_tys_chirho: Vec<TyChirho> =
            (0..arity_chirho).map(|_| self.fresh_var_chirho()).collect();
        let result_ty_chirho = self.fresh_var_chirho();
        self.infer_matches_with_seed_chirho(
            matches_chirho,
            span_chirho,
            param_tys_chirho,
            result_ty_chirho,
            false,
        )
    }

    fn split_fun_ty_for_arity_chirho(
        ty_chirho: &TyChirho,
        arity_chirho: usize,
    ) -> Option<(Vec<TyChirho>, TyChirho)> {
        let mut current_chirho = ty_chirho.clone();
        let mut params_chirho = Vec::with_capacity(arity_chirho);

        for _ in 0..arity_chirho {
            match current_chirho {
                TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
                    params_chirho.push(*arg_chirho);
                    current_chirho = *result_chirho;
                }
                TyChirho::ForallChirho { body_chirho, .. } => {
                    current_chirho = *body_chirho;
                    match current_chirho {
                        TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
                            params_chirho.push(*arg_chirho);
                            current_chirho = *result_chirho;
                        }
                        _ => return None,
                    }
                }
                _ => return None,
            }
        }

        Some((params_chirho, current_chirho))
    }

    fn type_matches_synonym_pattern_chirho(
        params_chirho: &[String],
        pattern_chirho: &TyChirho,
        actual_chirho: &TyChirho,
        bindings_chirho: &mut HashMap<String, TyChirho>,
    ) -> bool {
        match pattern_chirho {
            TyChirho::ForallVarChirho(name_chirho) if params_chirho.contains(name_chirho) => {
                if let Some(bound_chirho) = bindings_chirho.get(name_chirho) {
                    bound_chirho == actual_chirho
                } else {
                    bindings_chirho.insert(name_chirho.clone(), actual_chirho.clone());
                    true
                }
            }
            TyChirho::ConChirho(pattern_name_chirho) => {
                matches!(actual_chirho, TyChirho::ConChirho(actual_name_chirho) if actual_name_chirho == pattern_name_chirho)
            }
            TyChirho::VarChirho(pattern_var_chirho) => {
                matches!(actual_chirho, TyChirho::VarChirho(actual_var_chirho) if actual_var_chirho == pattern_var_chirho)
            }
            TyChirho::AppChirho(pattern_fun_chirho, pattern_arg_chirho) => {
                matches!(
                    actual_chirho,
                    TyChirho::AppChirho(actual_fun_chirho, actual_arg_chirho)
                        if Self::type_matches_synonym_pattern_chirho(
                            params_chirho,
                            pattern_fun_chirho,
                            actual_fun_chirho,
                            bindings_chirho,
                        ) && Self::type_matches_synonym_pattern_chirho(
                            params_chirho,
                            pattern_arg_chirho,
                            actual_arg_chirho,
                            bindings_chirho,
                        )
                )
            }
            TyChirho::FunChirho(pattern_arg_chirho, pattern_result_chirho, pattern_mult_chirho) => {
                matches!(
                    actual_chirho,
                    TyChirho::FunChirho(actual_arg_chirho, actual_result_chirho, actual_mult_chirho)
                        if actual_mult_chirho == pattern_mult_chirho
                            && Self::type_matches_synonym_pattern_chirho(
                                params_chirho,
                                pattern_arg_chirho,
                                actual_arg_chirho,
                                bindings_chirho,
                            )
                            && Self::type_matches_synonym_pattern_chirho(
                                params_chirho,
                                pattern_result_chirho,
                                actual_result_chirho,
                                bindings_chirho,
                            )
                )
            }
            TyChirho::ListChirho(pattern_elem_chirho) => {
                matches!(
                    actual_chirho,
                    TyChirho::ListChirho(actual_elem_chirho)
                        if Self::type_matches_synonym_pattern_chirho(
                            params_chirho,
                            pattern_elem_chirho,
                            actual_elem_chirho,
                            bindings_chirho,
                        )
                )
            }
            TyChirho::TupleChirho(pattern_elems_chirho) => {
                matches!(actual_chirho, TyChirho::TupleChirho(actual_elems_chirho)
                if pattern_elems_chirho.len() == actual_elems_chirho.len()
                    && pattern_elems_chirho
                        .iter()
                        .zip(actual_elems_chirho.iter())
                        .all(|(pattern_elem_chirho, actual_elem_chirho)| Self::type_matches_synonym_pattern_chirho(
                            params_chirho,
                            pattern_elem_chirho,
                            actual_elem_chirho,
                            bindings_chirho,
                        )))
            }
            TyChirho::ForallChirho {
                vars_chirho: pattern_vars_chirho,
                body_chirho: pattern_body_chirho,
            } => {
                matches!(
                    actual_chirho,
                    TyChirho::ForallChirho {
                        vars_chirho: actual_vars_chirho,
                        body_chirho: actual_body_chirho,
                    } if pattern_vars_chirho == actual_vars_chirho
                        && Self::type_matches_synonym_pattern_chirho(
                            params_chirho,
                            pattern_body_chirho,
                            actual_body_chirho,
                            bindings_chirho,
                        )
                )
            }
            _ => pattern_chirho == actual_chirho,
        }
    }

    fn matches_function_result_alias_expansion_chirho(&self, ty_chirho: &TyChirho) -> bool {
        let normalized_ty_chirho = self.normalize_ty_chirho(ty_chirho);
        ["ShowS", "ReadS"].iter().any(|synonym_name_chirho| {
            let Some((params_chirho, rhs_chirho)) =
                self.lookup_type_synonym_chirho(synonym_name_chirho)
            else {
                return false;
            };
            let normalized_rhs_chirho = self.normalize_ty_chirho(rhs_chirho);
            let mut bindings_chirho = HashMap::new();
            Self::type_matches_synonym_pattern_chirho(
                params_chirho,
                &normalized_rhs_chirho,
                &normalized_ty_chirho,
                &mut bindings_chirho,
            )
        })
    }

    fn fun_arity_chirho(&self, ty_chirho: &TyChirho) -> usize {
        let mut arity_chirho = 0;
        let mut current_chirho = ty_chirho;
        loop {
            if self.matches_function_result_alias_expansion_chirho(current_chirho) {
                return arity_chirho;
            }
            match current_chirho {
                TyChirho::ForallChirho { body_chirho, .. } => {
                    current_chirho = body_chirho;
                }
                TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    arity_chirho += vars_chirho.len();
                    current_chirho = body_chirho;
                }
                TyChirho::FunChirho(_, result_chirho, _) => {
                    arity_chirho += 1;
                    current_chirho = result_chirho;
                }
                _ => return arity_chirho,
            }
        }
    }

    fn infer_matches_against_expected_chirho(
        &mut self,
        matches_chirho: &[MatchArmChirho],
        span_chirho: SpanChirho,
        expected_ty_chirho: &TyChirho,
    ) -> (SubstChirho, TyChirho) {
        if matches_chirho.is_empty() {
            return (SubstChirho::empty_chirho(), self.fresh_var_chirho());
        }

        let arity_chirho = matches_chirho[0].pats_chirho.len();
        // workflow: language-features-chirho/rank-n-visible-type-application-chirho
        let mut required_groups_chirho = Vec::new();
        let mut required_body_chirho = expected_ty_chirho.clone();
        while let TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } = required_body_chirho
        {
            required_groups_chirho.push(vars_chirho);
            required_body_chirho = *body_chirho;
        }
        let required_arity_chirho: usize = required_groups_chirho.iter().map(Vec::len).sum();
        if required_arity_chirho > 0 {
            if arity_chirho < required_arity_chirho {
                return self.infer_matches_chirho(matches_chirho, span_chirho);
            }
            let term_arity_chirho = arity_chirho - required_arity_chirho;
            if self.fun_arity_chirho(&required_body_chirho) != term_arity_chirho {
                return self.infer_matches_chirho(matches_chirho, span_chirho);
            }

            let trimmed_matches_chirho: Vec<MatchArmChirho> = matches_chirho
                .iter()
                .cloned()
                .map(|mut match_chirho| {
                    match_chirho.pats_chirho.drain(..required_arity_chirho);
                    match_chirho
                })
                .collect();
            let Some((param_tys_chirho, result_ty_chirho)) =
                Self::split_fun_ty_for_arity_chirho(&required_body_chirho, term_arity_chirho)
            else {
                return self.infer_matches_chirho(matches_chirho, span_chirho);
            };
            let (subst_chirho, inferred_body_chirho) = self.infer_matches_with_seed_chirho(
                &trimmed_matches_chirho,
                span_chirho,
                param_tys_chirho,
                result_ty_chirho,
                true,
            );
            let inferred_ty_chirho = required_groups_chirho.into_iter().rev().fold(
                inferred_body_chirho,
                |body_chirho, vars_chirho| TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho: Box::new(body_chirho),
                },
            );
            return (subst_chirho, inferred_ty_chirho);
        }

        if arity_chirho == 0 {
            return self.infer_matches_with_seed_chirho(
                matches_chirho,
                span_chirho,
                vec![],
                expected_ty_chirho.clone(),
                true,
            );
        }
        // A function equation may bind fewer arguments than its signature and
        // return the remaining function (for example `f x = \y -> ...`).
        // Preserve that residual function as the RHS expected type.
        if self.fun_arity_chirho(expected_ty_chirho) < arity_chirho {
            return self.infer_matches_chirho(matches_chirho, span_chirho);
        }
        if let Some((param_tys_chirho, result_ty_chirho)) =
            Self::split_fun_ty_for_arity_chirho(expected_ty_chirho, arity_chirho)
        {
            self.infer_matches_with_seed_chirho(
                matches_chirho,
                span_chirho,
                param_tys_chirho,
                result_ty_chirho,
                true,
            )
        } else {
            self.infer_matches_chirho(matches_chirho, span_chirho)
        }
    }

    fn infer_lambda_with_seed_chirho(
        &mut self,
        pats_chirho: &[PatChirho],
        body_chirho: &ExprChirho,
        param_tys_chirho: Vec<TyChirho>,
        result_ty_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        self.env_chirho.push_scope_chirho();

        let mut param_subst_chirho = SubstChirho::empty_chirho();
        for (pat_chirho, pat_ty_chirho) in pats_chirho.iter().zip(param_tys_chirho.iter()) {
            let pat_ty_sub_chirho = param_subst_chirho.apply_ty_chirho(pat_ty_chirho);
            let s_chirho = self.bind_pat_chirho(pat_chirho, &pat_ty_sub_chirho);
            param_subst_chirho = s_chirho.compose_chirho(&param_subst_chirho);
            self.apply_subst_all_chirho(&s_chirho);
        }

        let expected_result_sub_chirho = param_subst_chirho.apply_ty_chirho(&result_ty_chirho);
        let (body_subst_chirho, body_ty_chirho) = self.infer_expr_against_expected_chirho(
            body_chirho,
            &expected_result_sub_chirho,
            span_chirho,
        );
        let combined_subst_chirho = body_subst_chirho.compose_chirho(&param_subst_chirho);
        let expected_result_final_chirho = combined_subst_chirho.apply_ty_chirho(&result_ty_chirho);

        let mut final_subst_chirho = combined_subst_chirho.clone();
        match self.unify_normalized_chirho(
            &body_ty_chirho,
            &expected_result_final_chirho,
            span_chirho,
        ) {
            Ok(su_chirho) => {
                final_subst_chirho = su_chirho.compose_chirho(&final_subst_chirho);
                self.apply_subst_all_chirho(&su_chirho);
            }
            Err(err_chirho) => {
                self.report_unify_error_chirho(&err_chirho);
            }
        }

        self.env_chirho.pop_scope_chirho();

        let mut result_fun_ty_chirho = final_subst_chirho.apply_ty_chirho(&result_ty_chirho);
        for param_ty_chirho in param_tys_chirho.into_iter().rev() {
            let param_sub_chirho = final_subst_chirho.apply_ty_chirho(&param_ty_chirho);
            result_fun_ty_chirho = TyChirho::fun_chirho(param_sub_chirho, result_fun_ty_chirho);
        }

        (final_subst_chirho, result_fun_ty_chirho)
    }

    fn infer_lambda_against_expected_chirho(
        &mut self,
        pats_chirho: &[PatChirho],
        body_chirho: &ExprChirho,
        expected_ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Option<(SubstChirho, TyChirho)> {
        let arity_chirho = pats_chirho.len();
        let (param_tys_chirho, result_ty_chirho) =
            Self::split_fun_ty_for_arity_chirho(expected_ty_chirho, arity_chirho)?;
        Some(self.infer_lambda_with_seed_chirho(
            pats_chirho,
            body_chirho,
            param_tys_chirho,
            result_ty_chirho,
            span_chirho,
        ))
    }

    fn contains_polymorphic_component_chirho(ty_chirho: &TyChirho) -> bool {
        match ty_chirho {
            TyChirho::ForallChirho { .. } | TyChirho::RequiredForallChirho { .. } => true,
            TyChirho::AppChirho(fun_chirho, arg_chirho)
            | TyChirho::FunChirho(fun_chirho, arg_chirho, _) => {
                Self::contains_polymorphic_component_chirho(fun_chirho)
                    || Self::contains_polymorphic_component_chirho(arg_chirho)
            }
            TyChirho::TupleChirho(elements_chirho) => elements_chirho
                .iter()
                .any(Self::contains_polymorphic_component_chirho),
            TyChirho::ListChirho(element_chirho) => {
                Self::contains_polymorphic_component_chirho(element_chirho)
            }
            TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => false,
        }
    }

    fn report_deferred_numeric_bool_predicates_chirho(&mut self) {
        for (pred_chirho, pred_span_chirho) in &self.deferred_preds_chirho {
            if pred_chirho.ty_chirho == TyChirho::bool_chirho()
                && matches!(
                    pred_chirho.class_name_chirho.as_str(),
                    "Num"
                        | "Integral"
                        | "Fractional"
                        | "Floating"
                        | "Real"
                        | "RealFrac"
                        | "RealFloat"
                )
            {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                        format!("no instance for `{} Bool`", pred_chirho.class_name_chirho),
                        *pred_span_chirho,
                    ));
            }
        }
    }

    fn infer_expr_against_expected_chirho(
        &mut self,
        expr_chirho: &ExprChirho,
        expected_ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        match expr_chirho {
            ExprChirho::TypeLamChirho {
                binder_chirho,
                body_chirho,
                ..
            } => {
                let normalized_expected_chirho = self.normalize_ty_chirho(expected_ty_chirho);
                let TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho: forall_body_chirho,
                } = &normalized_expected_chirho
                else {
                    return self.infer_expr_chirho(expr_chirho);
                };
                let Some((first_var_chirho, remaining_vars_chirho)) = vars_chirho.split_first()
                else {
                    return self.infer_expr_chirho(expr_chirho);
                };

                let expected_body_chirho = if remaining_vars_chirho.is_empty() {
                    forall_body_chirho.as_ref().clone()
                } else {
                    TyChirho::ForallChirho {
                        vars_chirho: remaining_vars_chirho.to_vec(),
                        body_chirho: forall_body_chirho.clone(),
                    }
                };
                let binder_text_chirho = binder_chirho.text_chirho().to_string();
                let previous_chirho = self
                    .scoped_tyvars_chirho
                    .insert(binder_text_chirho.clone(), *first_var_chirho);
                let (subst_chirho, _) = self.infer_expr_against_expected_chirho(
                    body_chirho,
                    &expected_body_chirho,
                    span_chirho,
                );
                if let Some(previous_chirho) = previous_chirho {
                    self.scoped_tyvars_chirho
                        .insert(binder_text_chirho, previous_chirho);
                } else {
                    self.scoped_tyvars_chirho.remove(&binder_text_chirho);
                }
                let final_ty_chirho = subst_chirho.apply_ty_chirho(&normalized_expected_chirho);
                (subst_chirho, final_ty_chirho)
            }
            // As for `if`: polymorphic arm results need bidirectional checking.
            // workflow: language-features-chirho/rank-n-visible-type-application-chirho
            ExprChirho::MultiWayIfChirho { arms_chirho, .. }
                if Self::contains_polymorphic_component_chirho(expected_ty_chirho) =>
            {
                self.infer_multiway_if_chirho(arms_chirho, Some(expected_ty_chirho))
            }
            ExprChirho::IfChirho {
                cond_chirho,
                then_chirho,
                else_chirho,
                span_chirho: if_span_chirho,
            } if Self::contains_polymorphic_component_chirho(expected_ty_chirho) => self
                .check_if_against_expected_chirho(
                    cond_chirho,
                    then_chirho,
                    else_chirho,
                    *if_span_chirho,
                    expected_ty_chirho,
                ),
            ExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } if Self::contains_polymorphic_component_chirho(expected_ty_chirho)
                || contains_skolem_chirho(expected_ty_chirho) =>
            {
                // A rigid expected type must reach every alternative, because a
                // GADT alternative sees it through its own refinement.
                // Preserve ordinary/GADT case leniency by entering this path
                // only when a nested polymorphic type actually needs to reach
                // alternative lambdas.
                // workflow: language-features-chirho/rank-n-visible-type-application-chirho
                let (scrut_subst_chirho, scrut_ty_chirho) =
                    self.infer_expr_chirho(scrutinee_chirho);
                self.apply_subst_all_chirho(&scrut_subst_chirho);
                let mut subst_chirho = scrut_subst_chirho;

                for alt_chirho in alts_chirho {
                    self.env_chirho.push_scope_chirho();

                    self.bind_pat_against_chirho(
                        &alt_chirho.pat_chirho,
                        &scrut_ty_chirho,
                        &mut subst_chirho,
                    );

                    self.infer_local_binds_chirho(
                        &alt_chirho.where_binds_chirho,
                        &mut subst_chirho,
                    );

                    let alt_expected_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
                    let (alt_subst_chirho, alt_ty_chirho) = self.infer_rhs_against_expected_chirho(
                        &alt_chirho.rhs_chirho,
                        &alt_expected_chirho,
                        alt_chirho.span_chirho,
                    );
                    subst_chirho = alt_subst_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&alt_subst_chirho);

                    // A guarded alternative's arms meet the expected type
                    // inside their own scopes. Retain the existing lenient
                    // result merge so GADT-refined alternatives do not gain
                    // false diagnostics.
                    let alt_ty_sub_chirho = subst_chirho.apply_ty_chirho(&alt_ty_chirho);
                    let expected_alt_final_chirho =
                        subst_chirho.apply_ty_chirho(expected_ty_chirho);
                    if let Ok(unify_subst_chirho) = self.unify_normalized_chirho(
                        &alt_ty_sub_chirho,
                        &expected_alt_final_chirho,
                        alt_chirho.span_chirho,
                    ) {
                        subst_chirho = unify_subst_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&unify_subst_chirho);
                    }

                    self.env_chirho.pop_scope_chirho();
                }

                let final_ty_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
                (subst_chirho, final_ty_chirho)
            }
            ExprChirho::LamChirho {
                pats_chirho,
                body_chirho,
                ..
            } => self
                .infer_lambda_against_expected_chirho(
                    pats_chirho,
                    body_chirho,
                    expected_ty_chirho,
                    span_chirho,
                )
                .unwrap_or_else(|| self.infer_expr_chirho(expr_chirho)),
            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                span_chirho: app_span_chirho,
            } => {
                // Bidirectional application checking lets a known result type
                // specialize variables that also occur in the parameter. This
                // is required for `g True :: Char` when `g :: F a -> a` and
                // `F Char ~ Bool`.
                // workflow: language-features-chirho/rank-n-visible-type-application-chirho
                let (fun_subst_chirho, inferred_fun_ty_chirho) = self.infer_expr_chirho(fun_chirho);
                self.apply_subst_all_chirho(&fun_subst_chirho);
                let inferred_fun_sub_chirho =
                    fun_subst_chirho.apply_ty_chirho(&inferred_fun_ty_chirho);

                if let Some(required_arg_ty_chirho) =
                    self.expr_to_required_type_arg_chirho(arg_chirho)
                {
                    if let Some(required_result_ty_chirho) = self.apply_required_type_arg_chirho(
                        &inferred_fun_sub_chirho,
                        required_arg_ty_chirho,
                    ) {
                        return match self.unify_normalized_chirho(
                            &required_result_ty_chirho,
                            expected_ty_chirho,
                            *app_span_chirho,
                        ) {
                            Ok(result_subst_chirho) => {
                                let combined_chirho =
                                    result_subst_chirho.compose_chirho(&fun_subst_chirho);
                                self.apply_subst_all_chirho(&result_subst_chirho);
                                let final_ty_chirho =
                                    combined_chirho.apply_ty_chirho(expected_ty_chirho);
                                (combined_chirho, final_ty_chirho)
                            }
                            Err(err_chirho) => {
                                self.report_unify_error_chirho(&err_chirho);
                                (fun_subst_chirho, required_result_ty_chirho)
                            }
                        };
                    }
                }

                let normalized_fun_ty_chirho = self.normalize_ty_chirho(&inferred_fun_sub_chirho);
                if let TyChirho::FunChirho(expected_arg_ty_chirho, inferred_result_ty_chirho, _) =
                    normalized_fun_ty_chirho
                {
                    let mut subst_chirho = fun_subst_chirho;

                    // Seed from the result when possible. If a family result
                    // cannot reduce until the argument is known, defer this
                    // diagnostic and retry after checking the argument.
                    if let Ok(result_subst_chirho) = self.unify_normalized_chirho(
                        &inferred_result_ty_chirho,
                        expected_ty_chirho,
                        *app_span_chirho,
                    ) {
                        subst_chirho = result_subst_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&result_subst_chirho);
                    }

                    let expected_arg_sub_chirho =
                        subst_chirho.apply_ty_chirho(&expected_arg_ty_chirho);
                    let (arg_subst_chirho, inferred_arg_ty_chirho) = self
                        .infer_expr_against_expected_chirho(
                            arg_chirho,
                            &expected_arg_sub_chirho,
                            *app_span_chirho,
                        );
                    subst_chirho = arg_subst_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&arg_subst_chirho);

                    let inferred_arg_sub_chirho =
                        subst_chirho.apply_ty_chirho(&inferred_arg_ty_chirho);
                    let expected_arg_final_chirho =
                        subst_chirho.apply_ty_chirho(&expected_arg_ty_chirho);
                    match self.unify_normalized_chirho(
                        &inferred_arg_sub_chirho,
                        &expected_arg_final_chirho,
                        *app_span_chirho,
                    ) {
                        Ok(arg_unify_subst_chirho) => {
                            subst_chirho = arg_unify_subst_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&arg_unify_subst_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }

                    let inferred_result_final_chirho =
                        subst_chirho.apply_ty_chirho(&inferred_result_ty_chirho);
                    let expected_result_final_chirho =
                        subst_chirho.apply_ty_chirho(expected_ty_chirho);
                    match self.unify_normalized_chirho(
                        &inferred_result_final_chirho,
                        &expected_result_final_chirho,
                        *app_span_chirho,
                    ) {
                        Ok(result_subst_chirho) => {
                            subst_chirho = result_subst_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&result_subst_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }

                    let final_ty_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
                    return (subst_chirho, final_ty_chirho);
                }

                let (arg_subst_chirho, inferred_arg_ty_chirho) = self.infer_expr_chirho(arg_chirho);
                let mut subst_chirho = arg_subst_chirho.compose_chirho(&fun_subst_chirho);
                self.apply_subst_all_chirho(&arg_subst_chirho);
                let inferred_fun_final_chirho =
                    subst_chirho.apply_ty_chirho(&inferred_fun_sub_chirho);
                let inferred_arg_final_chirho =
                    subst_chirho.apply_ty_chirho(&inferred_arg_ty_chirho);
                let result_ty_chirho = self.fresh_var_chirho();
                let expected_fun_ty_chirho =
                    TyChirho::fun_chirho(inferred_arg_final_chirho, result_ty_chirho.clone());
                match self.unify_normalized_chirho(
                    &inferred_fun_final_chirho,
                    &expected_fun_ty_chirho,
                    *app_span_chirho,
                ) {
                    Ok(app_subst_chirho) => {
                        subst_chirho = app_subst_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&app_subst_chirho);
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        return (subst_chirho, result_ty_chirho);
                    }
                }

                let result_ty_final_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                let expected_result_final_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
                match self.unify_normalized_chirho(
                    &result_ty_final_chirho,
                    &expected_result_final_chirho,
                    *app_span_chirho,
                ) {
                    Ok(result_subst_chirho) => {
                        subst_chirho = result_subst_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&result_subst_chirho);
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                    }
                }
                let final_ty_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
                (subst_chirho, final_ty_chirho)
            }
            ExprChirho::InfixChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                span_chirho: infix_span_chirho,
            } => {
                let app_expr_chirho = build_infix_app_expr_chirho(
                    left_chirho,
                    op_chirho,
                    right_chirho,
                    *infix_span_chirho,
                );
                self.infer_expr_against_expected_chirho(
                    &app_expr_chirho,
                    expected_ty_chirho,
                    *infix_span_chirho,
                )
            }
            ExprChirho::ListChirho {
                elements_chirho,
                span_chirho: list_span_chirho,
            } => {
                let expected_norm_chirho = self.normalize_ty_chirho(expected_ty_chirho);
                let TyChirho::ListChirho(expected_elem_ty_chirho) = expected_norm_chirho else {
                    let (s_chirho, actual_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                    match self.unify_normalized_chirho(
                        &actual_ty_chirho,
                        expected_ty_chirho,
                        span_chirho,
                    ) {
                        Ok(su_chirho) => {
                            let combined_chirho = su_chirho.compose_chirho(&s_chirho);
                            let final_ty_chirho =
                                combined_chirho.apply_ty_chirho(expected_ty_chirho);
                            self.apply_subst_all_chirho(&su_chirho);
                            return (combined_chirho, final_ty_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                            return (s_chirho, actual_ty_chirho);
                        }
                    }
                };

                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut elem_expected_ty_chirho = (*expected_elem_ty_chirho).clone();

                for elem_chirho in elements_chirho {
                    let elem_expected_sub_chirho =
                        subst_chirho.apply_ty_chirho(&elem_expected_ty_chirho);
                    let (elem_subst_chirho, elem_ty_chirho) = self
                        .infer_expr_against_expected_chirho(
                            elem_chirho,
                            &elem_expected_sub_chirho,
                            *list_span_chirho,
                        );
                    subst_chirho = elem_subst_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&elem_subst_chirho);

                    let elem_expected_final_chirho =
                        subst_chirho.apply_ty_chirho(&elem_expected_ty_chirho);
                    let elem_actual_final_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                    match self.unify_normalized_chirho(
                        &elem_actual_final_chirho,
                        &elem_expected_final_chirho,
                        *list_span_chirho,
                    ) {
                        Ok(su_chirho) => {
                            subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&su_chirho);
                            elem_expected_ty_chirho =
                                subst_chirho.apply_ty_chirho(&elem_expected_ty_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }
                }

                let final_ty_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
                (subst_chirho, final_ty_chirho)
            }
            _ => {
                // A `do` block's last statement is checked against the rigid
                // expected type (its alternatives may refine it); the block's
                // own arm consumes the hint.
                // workflow: language-features-chirho/rigid-type-variables-chirho
                if matches!(expr_chirho, ExprChirho::DoChirho { .. })
                    && contains_skolem_chirho(expected_ty_chirho)
                {
                    self.do_last_expected_chirho = Some(expected_ty_chirho.clone());
                }
                let (s_chirho, actual_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                self.do_last_expected_chirho = None;
                match self.unify_normalized_chirho(
                    &actual_ty_chirho,
                    expected_ty_chirho,
                    span_chirho,
                ) {
                    Ok(su_chirho) => {
                        let combined_chirho = su_chirho.compose_chirho(&s_chirho);
                        let final_ty_chirho = combined_chirho.apply_ty_chirho(expected_ty_chirho);
                        self.apply_subst_all_chirho(&su_chirho);
                        (combined_chirho, final_ty_chirho)
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        (s_chirho, actual_ty_chirho)
                    }
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Module-level inference
    // -----------------------------------------------------------------------

    /// Run type inference on a module. Returns the result with environment,
    /// substitution, and diagnostics.
    pub fn infer_module_chirho(&mut self, module_chirho: &ModuleChirho) -> SubstChirho {
        let mut subst_chirho = SubstChirho::empty_chirho();

        // A non-Prelude import can bring instances of SEEDED classes into
        // scope (e.g. a sibling module declaring `instance Show I`) that the
        // class env does not model, so the seeded instance lists stop being
        // the whole universe. Only classes declared in THIS module then keep a
        // fully-known universe — an imported module can never instance a class
        // it cannot see — and the decl walk below re-populates those.
        // Prelude is exempt: it is the instance set `seed_standard_chirho`
        // models, and the driver injects it implicitly into every module.
        let has_non_prelude_import_chirho = module_chirho
            .imports_chirho
            .iter()
            .any(|import_chirho| import_chirho.module_chirho.text_chirho() != "Prelude");
        if has_non_prelude_import_chirho {
            self.fully_known_class_names_chirho.clear();
        }

        self.constraint_generation_faithful_chirho =
            !module_chirho
                .extensions_chirho
                .iter()
                .any(|extension_chirho| {
                    Self::UNFAITHFUL_CONSTRAINT_EXTENSIONS_CHIRHO
                        .contains(&extension_chirho.as_str())
                });
        self.scoped_type_variables_chirho =
            Self::scoped_type_variables_enabled_chirho(module_chirho);
        self.relaxed_poly_rec_chirho =
            !module_chirho
                .extensions_chirho
                .iter()
                .any(|extension_chirho| {
                    extension_chirho == "Haskell98" || extension_chirho == "NoRelaxedPolyRec"
                });

        // Phase -1: Register type synonyms so they can be expanded during
        // type inference. Process in declaration order (handles chains
        // like type FilePath = String where String is already registered).
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                rhs_chirho,
                ..
            } = decl_chirho
            {
                let syn_name_chirho = name_chirho.text_chirho().to_string();
                let params_chirho: Vec<String> = type_vars_chirho
                    .iter()
                    .map(|v_chirho| v_chirho.text_chirho().to_string())
                    .collect();
                let rhs_ty_chirho = ast_type_to_syn_rhs_chirho(rhs_chirho, &params_chirho);
                self.register_type_synonym_chirho(syn_name_chirho, params_chirho, rhs_ty_chirho);
            }
        }

        // Deriving mentions become potential-instance suppressions for the
        // certainly-unsolvable check: `deriving Eq` on a local data decl is a
        // real `instance Eq T` in GHC even though the class env does not model
        // derived instances yet.
        for decl_chirho in &module_chirho.decls_chirho {
            match decl_chirho {
                DeclChirho::DataDeclChirho {
                    name_chirho,
                    deriving_chirho,
                    ..
                }
                | DeclChirho::NewtypeDeclChirho {
                    name_chirho,
                    deriving_chirho,
                    ..
                } => {
                    for class_chirho in deriving_chirho {
                        self.derived_instance_heads_chirho.insert((
                            class_chirho.text_chirho().to_string(),
                            name_chirho.text_chirho().to_string(),
                        ));
                    }
                }
                DeclChirho::StandaloneDerivingDeclChirho { class_chirho, .. } => {
                    self.derived_instance_heads_chirho
                        .insert((class_chirho.text_chirho().to_string(), "*".to_string()));
                }
                _ => {}
            }
        }
        for (first_chirho, second_chirho, _via_chirho) in &module_chirho.deriving_via_chirho {
            // Field order of the via tuple is not load-bearing here: insert
            // both orientations so the suppression works either way.
            self.derived_instance_heads_chirho.insert((
                first_chirho.text_chirho().to_string(),
                second_chirho.text_chirho().to_string(),
            ));
            self.derived_instance_heads_chirho.insert((
                second_chirho.text_chirho().to_string(),
                first_chirho.text_chirho().to_string(),
            ));
        }

        self.module_default_types_chirho.clear();
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::DefaultDeclChirho { types_chirho, .. } = decl_chirho {
                let mut default_var_map_chirho = HashMap::new();
                for default_type_chirho in types_chirho {
                    let default_ty_chirho = self
                        .ast_type_to_ty_chirho(default_type_chirho, &mut default_var_map_chirho);
                    self.module_default_types_chirho.push(default_ty_chirho);
                }
            }
        }

        // Phase -0.5: Register type families and type family instances
        for decl_chirho in &module_chirho.decls_chirho {
            match decl_chirho {
                DeclChirho::TypeFamilyDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    equations_chirho,
                    ..
                } => {
                    let family_name_chirho = name_chirho.text_chirho().to_string();
                    let param_names_chirho: Vec<String> = type_vars_chirho
                        .iter()
                        .map(|v_chirho| v_chirho.text_chirho().to_string())
                        .collect();
                    let eqs_chirho: Vec<(Vec<TyChirho>, TyChirho)> = equations_chirho
                        .iter()
                        .map(|eq_chirho| {
                            let lhs_chirho: Vec<TyChirho> = eq_chirho
                                .lhs_types_chirho
                                .iter()
                                .map(|t_chirho| {
                                    ast_type_to_syn_rhs_chirho(t_chirho, &param_names_chirho)
                                })
                                .collect();
                            let rhs_chirho = ast_type_to_syn_rhs_chirho(
                                &eq_chirho.rhs_chirho,
                                &param_names_chirho,
                            );
                            (lhs_chirho, rhs_chirho)
                        })
                        .collect();
                    self.register_type_family_chirho(family_name_chirho, eqs_chirho);
                }
                DeclChirho::TypeFamilyInstanceDeclChirho {
                    family_name_chirho,
                    lhs_types_chirho,
                    rhs_chirho,
                    ..
                } => {
                    let fname_chirho = family_name_chirho.text_chirho().to_string();
                    // Collect free type variables from LHS types so they become
                    // ForallVarChirho (matchable pattern vars) in the equation.
                    let inst_params_chirho =
                        collect_free_type_vars_from_ast_chirho(lhs_types_chirho);
                    let lhs_chirho: Vec<TyChirho> = lhs_types_chirho
                        .iter()
                        .map(|t_chirho| ast_type_to_syn_rhs_chirho(t_chirho, &inst_params_chirho))
                        .collect();
                    let rhs_ty_chirho = ast_type_to_syn_rhs_chirho(rhs_chirho, &inst_params_chirho);
                    self.register_type_family_instance_chirho(
                        fname_chirho,
                        lhs_chirho,
                        rhs_ty_chirho,
                    );
                }
                _ => {}
            }
        }

        for shadowed_name_chirho in collect_module_shadowed_names_chirho(module_chirho) {
            self.env_chirho.remove_chirho(&shadowed_name_chirho);
        }

        // Phase 0: Collect type signatures for later checking
        let mut type_sigs_chirho: HashMap<String, TypeChirho> = HashMap::new();
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } = decl_chirho
            {
                type_sigs_chirho.insert(
                    canonical_value_name_text_chirho(name_chirho.text_chirho()),
                    ty_chirho.clone(),
                );
            }
        }

        // Phase 0b: Seed foreign imports so later top-level bindings can use
        // them just like ordinary top-level definitions.
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::ForeignDeclChirho {
                direction_chirho:
                    haskelujah_ast_chirho::decl_chirho::ForeignDirectionChirho::ImportChirho,
                name_chirho,
                ty_chirho,
                ..
            } = decl_chirho
            {
                let foreign_scheme_chirho = self.ast_type_to_scheme_chirho(ty_chirho);
                self.env_chirho.bind_chirho(
                    canonical_value_name_text_chirho(name_chirho.text_chirho()),
                    foreign_scheme_chirho,
                );
            }
        }

        // Phase 1a: Process class declarations (before data/instance so methods
        // are available in the environment)
        for decl_chirho in &module_chirho.decls_chirho {
            self.process_class_decl_chirho(decl_chirho);
        }

        // Phase 1b: Process instance declarations
        for decl_chirho in &module_chirho.decls_chirho {
            self.process_instance_decl_chirho(decl_chirho);
        }
        // workflow: language-features-chirho/instance-obligations-chirho
        self.check_local_instance_obligations_chirho(module_chirho);

        // Phase 2: Bind data and newtype constructor types
        for decl_chirho in &module_chirho.decls_chirho {
            match decl_chirho {
                DeclChirho::DataDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    constructors_chirho,
                    ..
                } => {
                    let (result_ty_chirho, mut tv_map_chirho) =
                        self.data_head_type_chirho(name_chirho.text_chirho(), type_vars_chirho);
                    let data_type_name_chirho = name_chirho.text_chirho().to_string();
                    for con_chirho in constructors_chirho {
                        self.data_constructors_chirho
                            .entry(data_type_name_chirho.clone())
                            .or_default()
                            .push(
                                records_chirho::con_decl_name_chirho(con_chirho)
                                    .text_chirho()
                                    .to_string(),
                            );
                    }
                    for con_chirho in constructors_chirho {
                        match con_chirho {
                            haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                                name_chirho,
                                fields_chirho,
                                ..
                            } => {
                                if fields_chirho.iter().any(|(strictness_chirho, _ty_chirho)| {
                                    *strictness_chirho
                                        != haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho
                                }) {
                                    self.strict_positional_constructors_chirho
                                        .insert(name_chirho.text_chirho().to_string());
                                }
                                let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                    .iter()
                                    .map(|(_s_chirho, ty_chirho)| {
                                        self.ast_type_to_ty_chirho(ty_chirho, &mut tv_map_chirho)
                                    })
                                    .collect();
                                let con_ty_chirho = TyChirho::fun_n_chirho(
                                    field_tys_chirho,
                                    result_ty_chirho.clone(),
                                );
                                let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                                self.env_chirho.bind_chirho(
                                    name_chirho.text_chirho().to_string(),
                                    gen_scheme_chirho,
                                );
                            }
                            haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                                name_chirho,
                                fields_chirho,
                                ..
                            } => {
                                let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                    .iter()
                                    .flat_map(|fd_chirho| {
                                        let ty_chirho = self.ast_type_to_ty_chirho(
                                            &fd_chirho.ty_chirho,
                                            &mut tv_map_chirho,
                                        );
                                        std::iter::repeat(ty_chirho)
                                            .take(fd_chirho.names_chirho.len())
                                    })
                                    .collect();
                                let con_ty_chirho = TyChirho::fun_n_chirho(
                                    field_tys_chirho.clone(),
                                    result_ty_chirho.clone(),
                                );
                                let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                                self.env_chirho.bind_chirho(
                                    name_chirho.text_chirho().to_string(),
                                    gen_scheme_chirho,
                                );
                                // Store field names for RecordWildCards expansion
                                let field_names_chirho: Vec<String> = fields_chirho
                                    .iter()
                                    .flat_map(|fd_chirho| {
                                        fd_chirho
                                            .names_chirho
                                            .iter()
                                            .map(|n_chirho| n_chirho.text_chirho().to_string())
                                    })
                                    .collect();
                                self.con_field_names_chirho.insert(
                                    name_chirho.text_chirho().to_string(),
                                    field_names_chirho,
                                );
                                self.con_strict_fields_chirho.insert(
                                    name_chirho.text_chirho().to_string(),
                                    records_chirho::strict_field_names_chirho(fields_chirho),
                                );
                                // Bind field accessor functions: fieldName :: T -> FieldType
                                // Use a running index into field_tys_chirho (which is
                                // flattened by field names, not by field declarations).
                                let mut ft_idx_chirho = 0;
                                for fd_chirho in fields_chirho {
                                    for fname_chirho in &fd_chirho.names_chirho {
                                        if ft_idx_chirho < field_tys_chirho.len() {
                                            let accessor_ty_chirho = TyChirho::FunChirho(
                                                Box::new(result_ty_chirho.clone()),
                                                Box::new(field_tys_chirho[ft_idx_chirho].clone()),
                                                MultChirho::ManyChirho,
                                            );
                                            let accessor_scheme_chirho =
                                                self.generalize_chirho(&accessor_ty_chirho);
                                            self.env_chirho.bind_chirho(
                                                fname_chirho.text_chirho().to_string(),
                                                accessor_scheme_chirho,
                                            );
                                        }
                                        ft_idx_chirho += 1;
                                    }
                                }
                            }
                            haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                                name_chirho,
                                ty_chirho,
                                ..
                            } => {
                                // Convert the full GADT type signature to TyChirho.
                                // The return type comes from the signature itself,
                                // NOT from the auto-constructed `T a b c`.
                                let con_ty_chirho =
                                    self.ast_type_to_ty_chirho(ty_chirho, &mut tv_map_chirho);
                                let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                                self.env_chirho.bind_chirho(
                                    name_chirho.text_chirho().to_string(),
                                    gen_scheme_chirho,
                                );
                            }
                        }
                    }
                }
                DeclChirho::NewtypeDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    constructor_chirho,
                    ..
                } => {
                    self.data_constructors_chirho
                        .entry(name_chirho.text_chirho().to_string())
                        .or_default()
                        .push(
                            records_chirho::con_decl_name_chirho(constructor_chirho)
                                .text_chirho()
                                .to_string(),
                        );
                    let (result_ty_chirho, mut nt_tv_map_chirho) =
                        self.data_head_type_chirho(name_chirho.text_chirho(), type_vars_chirho);
                    match constructor_chirho {
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                            name_chirho: con_name_chirho,
                            fields_chirho,
                            ..
                        } => {
                            // Newtype constructor: field type → result type
                            let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                .iter()
                                .map(|(_s_chirho, ty_chirho)| {
                                    self.ast_type_to_ty_chirho(ty_chirho, &mut nt_tv_map_chirho)
                                })
                                .collect();
                            let con_ty_chirho =
                                TyChirho::fun_n_chirho(field_tys_chirho, result_ty_chirho);
                            let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                            self.env_chirho.bind_chirho(
                                con_name_chirho.text_chirho().to_string(),
                                gen_scheme_chirho,
                            );
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                            name_chirho: con_name_chirho,
                            fields_chirho,
                            ..
                        } => {
                            // Store field names for RecordWildCards expansion
                            let field_names_chirho: Vec<String> = fields_chirho
                                .iter()
                                .flat_map(|fd_chirho| {
                                    fd_chirho
                                        .names_chirho
                                        .iter()
                                        .map(|n_chirho| n_chirho.text_chirho().to_string())
                                })
                                .collect();
                            self.con_field_names_chirho.insert(
                                con_name_chirho.text_chirho().to_string(),
                                field_names_chirho,
                            );
                            self.con_strict_fields_chirho.insert(
                                con_name_chirho.text_chirho().to_string(),
                                records_chirho::strict_field_names_chirho(fields_chirho),
                            );
                            let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                .iter()
                                .flat_map(|fd_chirho| {
                                    let ty_chirho = self.ast_type_to_ty_chirho(
                                        &fd_chirho.ty_chirho,
                                        &mut nt_tv_map_chirho,
                                    );
                                    std::iter::repeat(ty_chirho).take(fd_chirho.names_chirho.len())
                                })
                                .collect();
                            let con_ty_chirho = TyChirho::fun_n_chirho(
                                field_tys_chirho.clone(),
                                result_ty_chirho.clone(),
                            );
                            let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                            self.env_chirho.bind_chirho(
                                con_name_chirho.text_chirho().to_string(),
                                gen_scheme_chirho,
                            );
                            // Bind field accessor functions: fieldName :: T -> FieldType
                            let mut nt_ft_idx_chirho = 0;
                            for fd_chirho in fields_chirho {
                                for fname_chirho in &fd_chirho.names_chirho {
                                    if nt_ft_idx_chirho < field_tys_chirho.len() {
                                        let accessor_ty_chirho = TyChirho::FunChirho(
                                            Box::new(result_ty_chirho.clone()),
                                            Box::new(field_tys_chirho[nt_ft_idx_chirho].clone()),
                                            MultChirho::ManyChirho,
                                        );
                                        let accessor_scheme_chirho =
                                            self.generalize_chirho(&accessor_ty_chirho);
                                        self.env_chirho.bind_chirho(
                                            fname_chirho.text_chirho().to_string(),
                                            accessor_scheme_chirho,
                                        );
                                    }
                                    nt_ft_idx_chirho += 1;
                                }
                            }
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                            name_chirho: con_name_chirho,
                            ty_chirho,
                            ..
                        } => {
                            // Newtype GADT: convert the full type signature.
                            let con_ty_chirho = self.ast_type_to_ty_chirho(
                                ty_chirho,
                                &mut std::collections::HashMap::new(),
                            );
                            let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                            self.env_chirho.bind_chirho(
                                con_name_chirho.text_chirho().to_string(),
                                gen_scheme_chirho,
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        // Phase 3: Infer function bindings using SCC-based binding groups
        // for proper let-polymorphism (forward references, mutual recursion).
        //
        // Phase 3a: Pre-bind ALL top-level function names with fresh type
        // variables so every function can reference every other function.
        // Phase 3b: Process groups in SCC topological order so that leaf
        // functions get generalized first, and functions that depend on
        // them see their polymorphic types.
        // 3a: Collect FunBindChirho declarations and pre-bind ALL of them
        let mut fun_names_chirho: Vec<String> = Vec::new();
        let mut fun_matches_refs_chirho: Vec<&[MatchArmChirho]> = Vec::new();
        let mut fun_spans_chirho: Vec<SpanChirho> = Vec::new();
        let mut fun_pre_tys_chirho: Vec<TyChirho> = Vec::new();

        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                span_chirho,
                ..
            } = decl_chirho
            {
                let binding_name_chirho =
                    canonical_value_name_text_chirho(name_chirho.text_chirho());
                let pre_ty_chirho =
                    if let Some(sig_ast_chirho) = type_sigs_chirho.get(&binding_name_chirho) {
                        let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                        let (sig_ty_chirho, _sig_preds_chirho) =
                            self.instantiate_scheme_parts_chirho(&sig_scheme_chirho);
                        self.env_chirho
                            .bind_chirho(binding_name_chirho.clone(), sig_scheme_chirho);
                        sig_ty_chirho
                    } else {
                        let fresh_ty_chirho = self.fresh_var_chirho();
                        self.env_chirho.bind_chirho(
                            binding_name_chirho.clone(),
                            SchemeChirho::mono_chirho(fresh_ty_chirho.clone()),
                        );
                        self.recursive_references_chirho
                            .assumptions_chirho
                            .insert(binding_name_chirho.clone(), fresh_ty_chirho.clone());
                        fresh_ty_chirho
                    };
                fun_names_chirho.push(binding_name_chirho);
                fun_matches_refs_chirho.push(matches_chirho.as_slice());
                fun_spans_chirho.push(*span_chirho);
                fun_pre_tys_chirho.push(pre_ty_chirho);
            }
        }

        // Also pre-bind PatBindChirho variables. When a type signature
        // exists for a pattern-bound variable, use the signature scheme
        // for proper polymorphism (e.g. `id1 :: a -> a; (id1) = id`).
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::PatBindChirho {
                pat_chirho,
                span_chirho: _,
                ..
            } = decl_chirho
            {
                let names_chirho = crate::linearity_chirho::pat_bound_names_chirho(pat_chirho);
                for name_chirho in names_chirho {
                    if let Some(sig_ast_chirho) = type_sigs_chirho.get(&name_chirho) {
                        let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                        self.env_chirho
                            .bind_chirho(name_chirho.clone(), sig_scheme_chirho);
                    } else {
                        let pre_ty_chirho = self.fresh_var_chirho();
                        self.env_chirho.bind_chirho(
                            name_chirho.clone(),
                            SchemeChirho::mono_chirho(pre_ty_chirho),
                        );
                    }
                }
            }
        }

        // Compute SCC binding groups (topological order: leaves first)
        let fun_decls_for_scc_chirho: Vec<(usize, &[MatchArmChirho])> = fun_matches_refs_chirho
            .iter()
            .enumerate()
            .map(|(i_chirho, m_chirho)| (i_chirho, *m_chirho))
            .collect();
        let signed_names_chirho: HashSet<String> = if self.relaxed_poly_rec_chirho {
            type_sigs_chirho.keys().cloned().collect()
        } else {
            HashSet::new()
        };
        let groups_chirho = binding_groups_chirho(
            &fun_names_chirho,
            &fun_decls_for_scc_chirho,
            &signed_names_chirho,
        );

        // 3b: Process each SCC group: infer → generalize
        // The rigid form of each checked signature, for the Phase 3c check.
        let mut skolemized_sigs_chirho: HashMap<String, TyChirho> = HashMap::new();
        for group_chirho in &groups_chirho {
            // Infer each function body in the group
            let mut group_inferred_chirho: Vec<(usize, String, TyChirho, SpanChirho)> = Vec::new();
            for &fi_chirho in group_chirho {
                let binding_name_chirho = &fun_names_chirho[fi_chirho];
                let pre_ty_chirho = &fun_pre_tys_chirho[fi_chirho];
                let span_chirho = fun_spans_chirho[fi_chirho];

                // ScopedTypeVariables: an explicit `forall` scopes the
                // signature's own binders (the same variables the rigid seed
                // was built from) over the body; restored afterwards.
                let prev_scoped_chirho = self.scoped_tyvars_chirho.clone();

                let (s_chirho, inferred_ty_chirho) =
                    if let Some(sig_ast_chirho) = type_sigs_chirho.get(binding_name_chirho) {
                        let (sig_scheme_chirho, sig_var_map_chirho) =
                            self.ast_type_to_scheme_with_var_map_chirho(sig_ast_chirho);
                        let (sig_ty_raw_chirho, sig_preds_chirho) =
                            self.skolemize_scheme_parts_chirho(&sig_scheme_chirho);
                        self.record_own_dictionary_indices_chirho(&sig_preds_chirho);
                        self.scope_signature_for_body_chirho(sig_ast_chirho, &sig_var_map_chirho);
                        skolemized_sigs_chirho
                            .insert(binding_name_chirho.clone(), sig_ty_raw_chirho.clone());
                        self.with_given_preds_chirho(sig_preds_chirho, |self_chirho| {
                            self_chirho.infer_matches_against_expected_chirho(
                                fun_matches_refs_chirho[fi_chirho],
                                span_chirho,
                                &sig_ty_raw_chirho,
                            )
                        })
                    } else {
                        self.infer_matches_chirho(fun_matches_refs_chirho[fi_chirho], span_chirho)
                    };
                subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                self.apply_subst_all_chirho(&s_chirho);

                // Unify pre-bound type with inferred type
                let pre_sub_chirho = subst_chirho.apply_ty_chirho(pre_ty_chirho);
                match self.unify_normalized_chirho(
                    &pre_sub_chirho,
                    &inferred_ty_chirho,
                    span_chirho,
                ) {
                    Ok(su_chirho) => {
                        subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&su_chirho);
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                    }
                }

                group_inferred_chirho.push((
                    fi_chirho,
                    binding_name_chirho.clone(),
                    inferred_ty_chirho,
                    span_chirho,
                ));

                // Restore scoped type variables
                self.scoped_tyvars_chirho = prev_scoped_chirho;
            }

            // Generalize all functions in the group together
            for (fi_chirho, binding_name_chirho, inferred_ty_chirho, span_chirho) in
                &group_inferred_chirho
            {
                self.env_chirho.remove_chirho(binding_name_chirho);

                let final_ty_chirho = subst_chirho.apply_ty_chirho(inferred_ty_chirho);
                // IO defaulting of a lone Monad/Functor-constrained variable is a
                // `main`-style convenience for argument-less bindings (where the
                // monomorphism restriction would block generalization anyway); a
                // function binding keeps its polymorphism.
                let argument_less_chirho = fun_matches_refs_chirho[*fi_chirho]
                    .first()
                    .is_none_or(|arm_chirho| arm_chirho.pats_chirho.is_empty());
                let inferred_scheme_chirho = self.generalize_with_io_defaulting_chirho(
                    &final_ty_chirho,
                    argument_less_chirho,
                    GeneralizationScopeChirho::TopLevelChirho,
                );
                let mut binding_scheme_chirho = inferred_scheme_chirho.clone();

                // Phase 3c: Check against type signature if one exists
                if let Some(sig_ast_chirho) = type_sigs_chirho.get(binding_name_chirho) {
                    let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                    // Build the full sig type preserving forall structure
                    let sig_full_chirho = if sig_scheme_chirho.vars_chirho.is_empty() {
                        sig_scheme_chirho.ty_chirho.clone()
                    } else {
                        TyChirho::ForallChirho {
                            vars_chirho: sig_scheme_chirho.vars_chirho.clone(),
                            body_chirho: Box::new(sig_scheme_chirho.ty_chirho.clone()),
                        }
                    };
                    // Check against the SAME rigid signature the body was
                    // checked with: a signature variable the body forced to a
                    // concrete type is a mismatch, not an instantiation.
                    // workflow: language-features-chirho/rigid-type-variables-chirho
                    let sig_ty_chirho = match skolemized_sigs_chirho.get(binding_name_chirho) {
                        Some(skolemized_chirho) => self
                            .normalize_ty_chirho(&subst_chirho.apply_ty_chirho(skolemized_chirho)),
                        None => self.normalize_ty_chirho(&sig_full_chirho),
                    };
                    let inferred_sub_chirho = subst_chirho.apply_ty_chirho(inferred_ty_chirho);
                    let inferred_sub_chirho = self.normalize_ty_chirho(&inferred_sub_chirho);
                    // Use subsumption checking to handle higher-rank types
                    match self.subsume_normalized_chirho(
                        &inferred_sub_chirho,
                        &sig_ty_chirho,
                        *span_chirho,
                    ) {
                        Ok(sig_s_chirho) => {
                            subst_chirho = sig_s_chirho.compose_chirho(&subst_chirho);
                            self.apply_subst_all_chirho(&sig_s_chirho);
                            binding_scheme_chirho = sig_scheme_chirho;
                        }
                        Err(_err_chirho) => {
                            self.diagnostics_chirho.push_chirho(
                                DiagnosticChirho::error_with_code_chirho(
                                    ErrorCodeChirho::error_chirho(SIGNATURE_MISMATCH_CODE_CHIRHO),
                                    format!(
                                        "type signature mismatch for `{binding_name_chirho}`: \
                                         inferred `{inferred_sub_chirho}`, \
                                         declared `{sig_ty_chirho}`"
                                    ),
                                    *span_chirho,
                                ),
                            );
                        }
                    }
                }

                self.finish_recursive_references_chirho(
                    binding_name_chirho,
                    &binding_scheme_chirho,
                );
                self.env_chirho
                    .bind_chirho(binding_name_chirho.clone(), binding_scheme_chirho);
            }
        }

        // Phase 3d: Handle top-level pattern bindings
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::PatBindChirho {
                pat_chirho,
                rhs_chirho,
                span_chirho: _,
            } = decl_chirho
            {
                if matches!(rhs_chirho, RhsChirho::GuardedChirho(arms_chirho) if arms_chirho.is_empty())
                {
                    continue;
                }
                // The WHOLE right-hand side: every arm's qualifiers and body,
                // not the first arm's body alone.
                let (s1_chirho, rhs_ty_chirho) = self.infer_rhs_chirho(rhs_chirho);
                subst_chirho = s1_chirho.compose_chirho(&subst_chirho);
                self.apply_subst_all_chirho(&s1_chirho);

                let sp_chirho = self.bind_pat_chirho(pat_chirho, &rhs_ty_chirho);
                subst_chirho = sp_chirho.compose_chirho(&subst_chirho);
                self.apply_subst_all_chirho(&sp_chirho);

                let names_chirho = crate::linearity_chirho::pat_bound_names_chirho(pat_chirho);
                // The whole pattern binding is generalized together: every
                // bound name leaves the environment before any of them is
                // generalized or checked, so a sibling's monomorphic type does
                // not pin the shared variables.
                let resolved_types_chirho: Vec<(String, TyChirho)> = names_chirho
                    .iter()
                    .filter_map(|name_chirho| {
                        self.env_chirho
                            .lookup_chirho(name_chirho)
                            .map(|scheme_chirho| {
                                (
                                    name_chirho.clone(),
                                    subst_chirho.apply_ty_chirho(&scheme_chirho.ty_chirho),
                                )
                            })
                    })
                    .collect();
                for (name_chirho, _resolved_chirho) in &resolved_types_chirho {
                    self.env_chirho.remove_chirho(name_chirho);
                }
                let env_fvs_chirho = self.env_chirho.free_vars_chirho();
                for (name_chirho, resolved_chirho) in resolved_types_chirho {
                    let name_chirho = &name_chirho;
                    {
                        // Check against type signature if one exists
                        if let Some(sig_ast_chirho) = type_sigs_chirho.get(name_chirho) {
                            let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                            let (sig_ty_raw_chirho, sig_preds_chirho) =
                                self.skolemize_scheme_parts_chirho(&sig_scheme_chirho);
                            self.record_own_dictionary_indices_chirho(&sig_preds_chirho);
                            let sig_ty_chirho = self.normalize_ty_chirho(&sig_ty_raw_chirho);
                            // Generalize the monomorphic pattern type over the
                            // variables not free in the environment, instantiate
                            // that afresh, and check it against the rigid
                            // signature: sibling pattern variables share
                            // variables, and each signature is checked on its own.
                            let mut instantiation_chirho = SubstChirho::empty_chirho();
                            for var_chirho in resolved_chirho.free_vars_chirho() {
                                if !env_fvs_chirho.contains(&var_chirho) {
                                    let fresh_chirho = self.fresh_var_chirho();
                                    instantiation_chirho.insert_chirho(var_chirho, fresh_chirho);
                                }
                            }
                            let inferred_inst_chirho =
                                instantiation_chirho.apply_ty_chirho(&resolved_chirho);
                            let inferred_sub_chirho =
                                self.normalize_ty_chirho(&inferred_inst_chirho);
                            // The signature's context is available as givens
                            // while the RHS's wanteds are discharged.
                            let sig_subst_opt_chirho =
                                self.with_given_preds_chirho(sig_preds_chirho, |self_chirho| {
                                    match self_chirho.subsume_normalized_chirho(
                                        &inferred_sub_chirho,
                                        &sig_ty_chirho,
                                        SpanChirho::DUMMY_CHIRHO,
                                    ) {
                                        Ok(sig_s_chirho) => {
                                            self_chirho.apply_subst_all_chirho(&sig_s_chirho);
                                            Some(sig_s_chirho)
                                        }
                                        Err(_err_chirho) => {
                                            self_chirho.diagnostics_chirho.push_chirho(
                                            DiagnosticChirho::error_with_code_chirho(
                                                ErrorCodeChirho::error_chirho(
                                                    SIGNATURE_MISMATCH_CODE_CHIRHO,
                                                ),
                                                format!(
                                                    "type signature mismatch for `{name_chirho}`: \
                                                         inferred `{inferred_sub_chirho}`, \
                                                         declared `{sig_ty_chirho}`"
                                                ),
                                                SpanChirho::DUMMY_CHIRHO,
                                            ),
                                        );
                                            None
                                        }
                                    }
                                });
                            if let Some(sig_s_chirho) = sig_subst_opt_chirho {
                                subst_chirho = sig_s_chirho.compose_chirho(&subst_chirho);
                            }
                            // Use the signature scheme directly for proper
                            // polymorphism (e.g. `id1 :: a -> a; (id1) = id`)
                            self.env_chirho
                                .bind_chirho(name_chirho.clone(), sig_scheme_chirho);
                        } else {
                            let gen_chirho = self.generalize_chirho(&resolved_chirho);
                            self.env_chirho.bind_chirho(name_chirho.clone(), gen_chirho);
                        }
                    }
                }
            }
        }

        // Phase 3e: Check instance method implementations against their
        // class method types specialized to the instance head. This gives
        // instance bodies the same expected-type guidance that top-level
        // signature-annotated wrappers already receive.
        self.check_instance_methods_against_class_chirho(module_chirho);

        // Deferred equalities whose stuck side has been solved by now are
        // decided here; one still stuck against a closed type is reported,
        // one open on both sides is left alone.
        let equalities_chirho = self.retry_deferred_equalities_chirho();
        self.report_unsolved_deferred_equalities_chirho();
        if equalities_chirho.is_empty_chirho() {
            subst_chirho
        } else {
            equalities_chirho.compose_chirho(&subst_chirho)
        }
    }

    /// Check remaining deferred predicates. Those that the class environment
    /// can fully entail are discharged; those that can't produce diagnostics.
    ///
    /// Implements Haskell-style numeric defaulting: ambiguous type variables
    /// constrained only by defaultable classes are resolved to `Integer` (for
    /// Num/Integral/etc.) or `Double` (for Fractional/Floating/etc.) before
    /// the final constraint check.
    fn check_deferred_preds_chirho(&mut self, final_subst_chirho: &SubstChirho) -> SubstChirho {
        let preds_chirho: Vec<(PredChirho, SpanChirho)> =
            self.deferred_preds_chirho.drain(..).collect();
        let resolved_given_preds_chirho: Vec<PredChirho> = self
            .given_preds_chirho
            .iter()
            .map(|pred_chirho| PredChirho {
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: self.normalize_ty_chirho(
                    &final_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                ),
                extra_tys_chirho: pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| {
                        self.normalize_ty_chirho(&final_subst_chirho.apply_ty_chirho(t_chirho))
                    })
                    .collect(),
            })
            .collect();

        let defaultable_classes_chirho: &[&str] = &[
            "Num",
            "Integral",
            "Enum",
            "Bounded",
            "Eq",
            "Ord",
            "Show",
            "Read",
            "Real",
            "Fractional",
            "Floating",
            "RealFrac",
            "RealFloat",
        ];
        let numeric_default_trigger_classes_chirho: &[&str] = &[
            "Num",
            "Integral",
            "Real",
            "Fractional",
            "Floating",
            "RealFrac",
            "RealFloat",
        ];
        let fractional_classes_chirho: &[&str] =
            &["Fractional", "Floating", "RealFrac", "RealFloat"];

        // Phase 1: resolve all predicates and collect constraints per type variable
        let mut resolved_preds_chirho: Vec<(PredChirho, SpanChirho)> = Vec::new();
        let mut var_classes_chirho: std::collections::HashMap<TyVarChirho, Vec<String>> =
            std::collections::HashMap::new();

        for (pred_chirho, span_chirho) in &preds_chirho {
            let resolved_ty_chirho = final_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho);
            let resolved_extra_tys_chirho: Vec<TyChirho> = pred_chirho
                .extra_tys_chirho
                .iter()
                .map(|t_chirho| final_subst_chirho.apply_ty_chirho(t_chirho))
                .collect();
            let resolved_ty_chirho = self.normalize_ty_chirho(&resolved_ty_chirho);
            let resolved_extra_tys_chirho: Vec<TyChirho> = resolved_extra_tys_chirho
                .into_iter()
                .map(|t_chirho| self.normalize_ty_chirho(&t_chirho))
                .collect();
            let mut resolved_pred_chirho = PredChirho {
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: resolved_ty_chirho,
                extra_tys_chirho: resolved_extra_tys_chirho,
            };

            // Apply functional dependency improvement
            let improvement_chirho = self
                .class_env_chirho
                .fundep_improve_chirho(&resolved_pred_chirho);
            if !improvement_chirho.is_empty_chirho() {
                resolved_pred_chirho.ty_chirho =
                    improvement_chirho.apply_ty_chirho(&resolved_pred_chirho.ty_chirho);
                resolved_pred_chirho.extra_tys_chirho = resolved_pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| improvement_chirho.apply_ty_chirho(t_chirho))
                    .collect();
            }

            // Collect bare type-variable constraints for defaulting
            if let TyChirho::VarChirho(v_chirho) = &resolved_pred_chirho.ty_chirho {
                if self.pred_entailed_by_givens_chirho(
                    &resolved_pred_chirho,
                    &resolved_given_preds_chirho,
                ) {
                    continue;
                }
                var_classes_chirho
                    .entry(*v_chirho)
                    .or_default()
                    .push(resolved_pred_chirho.class_name_chirho.clone());
            }
            if let Some(sub_goals_chirho) =
                self.class_env_chirho.resolve_chirho(&resolved_pred_chirho)
            {
                for sub_goal_chirho in sub_goals_chirho {
                    if self.pred_entailed_by_givens_chirho(
                        &sub_goal_chirho,
                        &resolved_given_preds_chirho,
                    ) {
                        continue;
                    }
                    if let TyChirho::VarChirho(v_chirho) = &sub_goal_chirho.ty_chirho {
                        var_classes_chirho
                            .entry(*v_chirho)
                            .or_default()
                            .push(sub_goal_chirho.class_name_chirho.clone());
                    }
                }
            }

            resolved_preds_chirho.push((resolved_pred_chirho, *span_chirho));
        }

        // Phase 2: compute defaults for ambiguous type variables
        let mut default_subst_chirho = SubstChirho::empty_chirho();
        let io_defaultable_classes_chirho: &[&str] = &[
            "Monad",
            "Applicative",
            "Functor",
            "MonadIO",
            "MonadFail",
            "MonadFix",
        ];
        // DETERMINISM: this loop BUILDS a defaulting substitution, so the order in
        // which ambiguous variables are defaulted decides the inferred types. Iterating
        // a HashMap made that order depend on a per-process hash seed.
        // See spec-chirho/bug-nondeterministic-typecheck-chirho.md
        let mut sorted_var_classes_chirho: Vec<_> = var_classes_chirho.iter().collect();
        sorted_var_classes_chirho.sort_by_key(|(var_chirho, _)| **var_chirho);
        for (var_chirho, classes_chirho) in sorted_var_classes_chirho {
            // Check if all constraints are IO-defaultable (Monad/Applicative/Functor)
            let all_io_defaultable_chirho = classes_chirho
                .iter()
                .all(|c_chirho| io_defaultable_classes_chirho.contains(&c_chirho.as_str()));
            if all_io_defaultable_chirho && !classes_chirho.is_empty() {
                default_subst_chirho
                    .insert_chirho(*var_chirho, TyChirho::ConChirho("IO".to_string()));
                continue;
            }
            let has_numeric_trigger_chirho = classes_chirho.iter().any(|c_chirho| {
                numeric_default_trigger_classes_chirho.contains(&c_chirho.as_str())
            });
            // Haskelujah leniency: if a literal-driven type variable also has
            // user-class constraints, prefer Int only when Int satisfies every
            // collected class. This keeps ordinary Num-only defaulting on the
            // Report path while accepting small local examples like
            // `Describable a => Describable [a]` with `[42]`.
            let all_defaultable_chirho = classes_chirho
                .iter()
                .all(|c_chirho| defaultable_classes_chirho.contains(&c_chirho.as_str()));
            if !all_defaultable_chirho {
                if has_numeric_trigger_chirho {
                    let int_ty_chirho = TyChirho::int_chirho();
                    let int_satisfies_all_chirho = classes_chirho.iter().all(|class_name_chirho| {
                        self.class_env_chirho
                            .entails_chirho(&PredChirho::new_chirho(
                                class_name_chirho,
                                int_ty_chirho.clone(),
                            ))
                    });
                    if int_satisfies_all_chirho {
                        default_subst_chirho.insert_chirho(*var_chirho, int_ty_chirho);
                    }
                }
                continue;
            }
            if !has_numeric_trigger_chirho {
                continue;
            }
            if !self.module_default_types_chirho.is_empty() {
                let mut matched_default_chirho = None;
                for candidate_ty_chirho in &self.module_default_types_chirho {
                    let candidate_matches_chirho = classes_chirho.iter().all(|class_name_chirho| {
                        self.class_env_chirho
                            .resolve_chirho(&PredChirho::new_chirho(
                                class_name_chirho,
                                candidate_ty_chirho.clone(),
                            ))
                            .is_some()
                    });
                    if candidate_matches_chirho {
                        matched_default_chirho = Some(candidate_ty_chirho.clone());
                        break;
                    }
                }
                if let Some(default_ty_chirho) = matched_default_chirho {
                    default_subst_chirho.insert_chirho(*var_chirho, default_ty_chirho);
                    continue;
                }
            }
            // If any constraint is a Fractional-group class, default to Double;
            // otherwise default to Integer, matching Haskell's standard default
            // declaration.
            let needs_double_chirho = classes_chirho
                .iter()
                .any(|c_chirho| fractional_classes_chirho.contains(&c_chirho.as_str()));
            let default_ty_chirho = if needs_double_chirho {
                TyChirho::double_chirho()
            } else {
                TyChirho::ConChirho("Integer".to_string())
            };
            default_subst_chirho.insert_chirho(*var_chirho, default_ty_chirho);
        }

        // Phase 3: apply defaults and check all predicates
        for (pred_chirho, span_chirho) in resolved_preds_chirho {
            if self.pred_entailed_by_givens_chirho(&pred_chirho, &resolved_given_preds_chirho) {
                continue;
            }
            let integral_bounded_integer_default_chirho = match &pred_chirho.ty_chirho {
                TyChirho::VarChirho(var_chirho)
                    if pred_chirho.class_name_chirho == "Bounded"
                        && matches!(
                            default_subst_chirho.lookup_chirho(var_chirho),
                            Some(TyChirho::ConChirho(name_chirho)) if name_chirho == "Integer"
                        ) =>
                {
                    var_classes_chirho
                        .get(var_chirho)
                        .is_some_and(|classes_chirho| {
                            classes_chirho
                                .iter()
                                .any(|class_name_chirho| class_name_chirho == "Integral")
                        })
                }
                _ => false,
            };
            if integral_bounded_integer_default_chirho {
                continue;
            }
            let defaulted_pred_chirho = PredChirho {
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: default_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                extra_tys_chirho: pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| default_subst_chirho.apply_ty_chirho(t_chirho))
                    .collect(),
            };

            // workflow: language-features-chirho/fundep-generalization-chirho
            if self.report_unsatisfiable_fundep_wanted_chirho(
                &defaulted_pred_chirho,
                final_subst_chirho,
                &resolved_given_preds_chirho,
                span_chirho,
            ) {
                continue;
            }

            // If still a bare type variable after defaulting, skip (truly ambiguous
            // but no concrete check is possible).
            if matches!(defaulted_pred_chirho.ty_chirho, TyChirho::VarChirho(_)) {
                continue;
            }

            // Certainly-unsolvable predicates are errors even when the class
            // has no registered instances: for a fully-known class the empty
            // instance list IS the whole universe. Apply the final
            // substitution to extras too (apply_subst_all_chirho maintains
            // only the primary type during inference).
            let fully_resolved_pred_chirho = PredChirho {
                class_name_chirho: defaulted_pred_chirho.class_name_chirho.clone(),
                ty_chirho: final_subst_chirho.apply_ty_chirho(&defaulted_pred_chirho.ty_chirho),
                extra_tys_chirho: defaulted_pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|t_chirho| final_subst_chirho.apply_ty_chirho(t_chirho))
                    .collect(),
            };
            if self.is_certainly_unsolvable_pred_chirho(&fully_resolved_pred_chirho) {
                let blamed_chirho =
                    self.unsolvable_blame_pred_chirho(&fully_resolved_pred_chirho, 0);
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                        format!("no instance for `{blamed_chirho}`"),
                        span_chirho,
                    ));
                continue;
            }
            // Skip constraints for classes with no instances in our env.
            // These are user-defined classes where instances are defined in
            // the same module, or given constraints from type signatures.
            let has_instances_chirho = self
                .class_env_chirho
                .instances_chirho
                .get(&defaulted_pred_chirho.class_name_chirho)
                .is_some_and(|insts_chirho| !insts_chirho.is_empty());
            if !has_instances_chirho {
                continue;
            }
            if Self::pred_mentions_rigid_chirho(&defaulted_pred_chirho) {
                // A wanted over a rigid signature variable was not discharged
                // by the signature's givens. Report it only when no instance
                // could ever apply (see rigid_pred_certainly_undeducible_chirho).
                if self.rigid_pred_certainly_undeducible_chirho(&fully_resolved_pred_chirho) {
                    self.diagnostics_chirho
                        .push_chirho(DiagnosticChirho::error_with_code_chirho(
                            ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                            format!(
                                "could not deduce `{fully_resolved_pred_chirho}` from the context of the type signature"
                            ),
                            span_chirho,
                        ));
                }
                continue;
            }
            if !self.class_env_chirho.entails_chirho(&defaulted_pred_chirho) {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                        format!("no instance for `{defaulted_pred_chirho}`"),
                        span_chirho,
                    ));
            }
        }
        default_subst_chirho
    }

    /// Evidence-threading P2: finalize raw occurrence captures against the
    /// composed substitution (unification + Report defaulting), keeping only
    /// records whose predicate type resolved to a concrete instance-head key.
    fn finalize_occurrence_records_chirho(
        &self,
        final_subst_chirho: &SubstChirho,
    ) -> Vec<MethodOccurrenceRecordChirho> {
        fn head_key_chirho(ty_chirho: &TyChirho) -> Option<String> {
            match ty_chirho {
                TyChirho::ConChirho(name_chirho) => Some(name_chirho.clone()),
                TyChirho::ListChirho(_) => Some("[]".to_string()),
                TyChirho::TupleChirho(items_chirho) => Some(format!(
                    "({})",
                    ",".repeat(items_chirho.len().saturating_sub(1))
                )),
                TyChirho::AppChirho(fun_chirho, _) => head_key_chirho(fun_chirho),
                _ => None,
            }
        }
        fn default_occurrence_key_chirho(class_name_chirho: &str) -> Option<String> {
            match class_name_chirho {
                "Fractional" | "Floating" | "RealFrac" | "RealFloat" => Some("Double".to_string()),
                "Num" | "Integral" | "Real" | "Enum" | "Bounded" | "Eq" | "Ord" | "Show"
                | "Read" => Some("Int".to_string()),
                _ => None,
            }
        }
        self.occurrence_captures_chirho
            .iter()
            .filter_map(
                |evidence_chirho::OccurrenceCaptureChirho {
                     name_chirho,
                     ordinal_chirho,
                     class_name_chirho: class_chirho,
                     ty_chirho,
                     span_chirho,
                     origin_chirho,
                 }| {
                    let resolved_chirho = final_subst_chirho.apply_ty_chirho(ty_chirho);
                    let hinted_key_chirho = match &resolved_chirho {
                        TyChirho::VarChirho(var_chirho) => self
                            .occurrence_default_hints_chirho
                            .get(var_chirho)
                            .cloned(),
                        _ => None,
                    };
                    if hinted_key_chirho.is_none()
                        && matches!(
                            &resolved_chirho,
                            TyChirho::VarChirho(var_chirho)
                                if self.occurrence_generalized_vars_chirho.contains(var_chirho)
                        )
                    {
                        // A local binding's variable: the enclosing code's single
                        // instantiation is the proof (workflow:
                        // language-features-chirho/dictionary-evidence-chirho).
                        if let TyChirho::VarChirho(var_chirho) = &resolved_chirho {
                            if let Some(key_chirho) = self
                                .local_specialization_key_chirho(*var_chirho, final_subst_chirho)
                            {
                                return Some(MethodOccurrenceRecordChirho {
                                    name_chirho: name_chirho.clone(),
                                    ordinal_chirho: *ordinal_chirho,
                                    class_name_chirho: class_chirho.clone(),
                                    ty_key_chirho: key_chirho,
                                    span_chirho: *span_chirho,
                                    origin_chirho: *origin_chirho,
                                });
                            }
                        }
                        return None;
                    }
                    // A method used at a rigid variable of the enclosing signature
                    // (`x + 1` inside `f :: Num a => a -> a`) has no instance to
                    // name: its evidence is the binding's own dictionary parameter.
                    // Defaulting it to `Int` here dispatched `f 2.5` to the Int row.
                    // An instance method's skolem is no such parameter and keys
                    // as the open variable it replaced, below.
                    // workflow: language-features-chirho/dictionary-evidence-chirho
                    if let TyChirho::ForallVarChirho(skolem_chirho) = &resolved_chirho
                        && let Some(key_chirho) =
                            self.own_parameter_key_chirho(class_chirho, skolem_chirho)
                    {
                        return Some(MethodOccurrenceRecordChirho {
                            name_chirho: name_chirho.clone(),
                            ordinal_chirho: *ordinal_chirho,
                            class_name_chirho: class_chirho.clone(),
                            ty_key_chirho: key_chirho,
                            span_chirho: *span_chirho,
                            origin_chirho: *origin_chirho,
                        });
                    }
                    let resolved_show_key_chirho = (class_chirho == "Show"
                        && !resolved_chirho.contains_var_chirho())
                    .then(|| evidence_chirho::concrete_show_key_chirho(&resolved_chirho, false))
                    .flatten();
                    let key_chirho = resolved_show_key_chirho
                        .or_else(|| evidence_chirho::ground_list_key_chirho(&resolved_chirho))
                        .or_else(|| head_key_chirho(&resolved_chirho))
                        .or(hinted_key_chirho)
                        .or_else(|| default_occurrence_key_chirho(class_chirho))?;
                    Some(MethodOccurrenceRecordChirho {
                        name_chirho: name_chirho.clone(),
                        ordinal_chirho: *ordinal_chirho,
                        class_name_chirho: class_chirho.clone(),
                        ty_key_chirho: key_chirho,
                        span_chirho: *span_chirho,
                        origin_chirho: *origin_chirho,
                    })
                },
            )
            .collect()
    }

    /// Consume the context and return the final result.
    pub fn finish_chirho(self) -> InferResultChirho {
        InferResultChirho {
            subst_chirho: SubstChirho::empty_chirho(),
            env_chirho: self.env_chirho,
            class_env_chirho: self.class_env_chirho,
            type_families_chirho: self.type_families_chirho,
            diagnostics_chirho: self.diagnostics_chirho,
            method_occurrences_chirho: Vec::new(),
            method_occurrence_totals_chirho: HashMap::new(),
            literal_evidence_chirho: HashMap::new(),
            literal_evidence_by_origin_chirho: HashMap::new(),
            reference_evidence_chirho: HashMap::new(),
            reference_evidence_by_origin_chirho: HashMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Type synonym helpers
// ---------------------------------------------------------------------------

/// Convert an AST `TypeChirho` to a `TyChirho` suitable for storing as a type
/// synonym RHS. Synonym parameters are represented as `ForallVarChirho` so
/// they can be substituted during expansion.
fn ast_type_to_syn_rhs_chirho(ty_chirho: &TypeChirho, params_chirho: &[String]) -> TyChirho {
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho) => {
            let text_chirho = name_chirho.text_chirho().to_string();
            if params_chirho.contains(&text_chirho) {
                TyChirho::ForallVarChirho(text_chirho)
            } else {
                // Unknown type variable — treat as Con (might be a bug upstream)
                TyChirho::ConChirho(text_chirho)
            }
        }
        TypeChirho::ConChirho(name_chirho) => TyChirho::ConChirho(name_chirho.full_name_chirho()),
        TypeChirho::ListChirho { element_chirho, .. } => TyChirho::ListChirho(Box::new(
            ast_type_to_syn_rhs_chirho(element_chirho, params_chirho),
        )),
        TypeChirho::FunChirho {
            arg_chirho,
            mult_chirho,
            result_chirho,
            ..
        } => {
            let m_chirho = match mult_chirho {
                Some(haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::OneChirho) => {
                    MultChirho::OneChirho
                }
                _ => MultChirho::ManyChirho,
            };
            TyChirho::FunChirho(
                Box::new(ast_type_to_syn_rhs_chirho(arg_chirho, params_chirho)),
                Box::new(ast_type_to_syn_rhs_chirho(result_chirho, params_chirho)),
                m_chirho,
            )
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => TyChirho::TupleChirho(
            elements_chirho
                .iter()
                .map(|e_chirho| ast_type_to_syn_rhs_chirho(e_chirho, params_chirho))
                .collect(),
        ),
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => TyChirho::AppChirho(
            Box::new(ast_type_to_syn_rhs_chirho(fun_chirho, params_chirho)),
            Box::new(ast_type_to_syn_rhs_chirho(arg_chirho, params_chirho)),
        ),
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            ast_type_to_syn_rhs_chirho(inner_chirho, params_chirho)
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            // Preserve constraint context but convert the body.
            // For now, just pass through the body since constraints in
            // synonym RHS are handled during instantiation.
            // TODO: properly preserve constraints as deferred predicates
            let _ = context_chirho;
            ast_type_to_syn_rhs_chirho(body_chirho, params_chirho)
        }
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            // Preserve the forall structure in the synonym RHS so that
            // higher-rank type synonyms like `type Locker = forall a. IO a -> IO a`
            // retain their polymorphic structure when expanded.
            let forall_var_names_chirho: Vec<String> = vars_chirho
                .iter()
                .map(|v_chirho| v_chirho.text_chirho().to_string())
                .collect();
            // Extend params with forall-bound vars so they become ForallVarChirho
            let mut extended_params_chirho = params_chirho.to_vec();
            extended_params_chirho.extend(forall_var_names_chirho.iter().cloned());
            let body_ty_chirho = ast_type_to_syn_rhs_chirho(body_chirho, &extended_params_chirho);
            // Create ForallChirho with TyVarChirho identifiers for the bound vars
            let bound_vars_chirho: Vec<TyVarChirho> = forall_var_names_chirho
                .iter()
                .enumerate()
                .map(|(i_chirho, _)| TyVarChirho(8000 + i_chirho as u32))
                .collect();
            // Replace ForallVarChirho names with the TyVarChirho references
            let mut renamed_body_chirho = body_ty_chirho;
            for (name_chirho, tv_chirho) in
                forall_var_names_chirho.iter().zip(bound_vars_chirho.iter())
            {
                renamed_body_chirho = subst_named_var_chirho(
                    &renamed_body_chirho,
                    name_chirho,
                    &TyChirho::VarChirho(*tv_chirho),
                );
            }
            TyChirho::ForallChirho {
                vars_chirho: bound_vars_chirho,
                body_chirho: Box::new(renamed_body_chirho),
            }
        }
        TypeChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            let binder_names_chirho: Vec<String> = vars_chirho
                .iter()
                .map(|var_chirho| var_chirho.text_chirho().to_string())
                .collect();
            let mut extended_params_chirho = params_chirho.to_vec();
            extended_params_chirho.extend(binder_names_chirho.iter().cloned());
            let body_ty_chirho = ast_type_to_syn_rhs_chirho(body_chirho, &extended_params_chirho);
            let bound_vars_chirho: Vec<TyVarChirho> = binder_names_chirho
                .iter()
                .enumerate()
                .map(|(index_chirho, _)| TyVarChirho(8500 + index_chirho as u32))
                .collect();
            let mut renamed_body_chirho = body_ty_chirho;
            for (name_chirho, ty_var_chirho) in
                binder_names_chirho.iter().zip(bound_vars_chirho.iter())
            {
                renamed_body_chirho = subst_named_var_chirho(
                    &renamed_body_chirho,
                    name_chirho,
                    &TyChirho::VarChirho(*ty_var_chirho),
                );
            }
            TyChirho::RequiredForallChirho {
                vars_chirho: bound_vars_chirho,
                body_chirho: Box::new(renamed_body_chirho),
            }
        }
        TypeChirho::PromotedConChirho { name_chirho, .. } => {
            TyChirho::ConChirho(if name_chirho.text_chirho() == ":" {
                ":".to_string()
            } else {
                format!("'{}", name_chirho.text_chirho())
            })
        }
        TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => TyChirho::promoted_list_chirho(
            elements_chirho
                .iter()
                .map(|element_chirho| ast_type_to_syn_rhs_chirho(element_chirho, params_chirho)),
        ),
        // PartialTypeSignatures: wildcard in a type synonym RHS is a fresh
        // anonymous variable (synthesised as "_wildcard_chirho").
        TypeChirho::WildcardChirho { .. } => TyChirho::ConChirho("_wildcard_chirho".to_string()),
        // Type-level literal (DataKinds): treat as a type-level constant.
        TypeChirho::LitChirho { value_chirho, .. } => TyChirho::ConChirho(value_chirho.clone()),
    }
}

fn build_infix_app_expr_chirho(
    left_chirho: &ExprChirho,
    op_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
    right_chirho: &ExprChirho,
    span_chirho: haskelujah_span_chirho::SpanChirho,
) -> ExprChirho {
    let right_arg_chirho = match right_chirho {
        ExprChirho::LamChirho { .. } | ExprChirho::TypeLamChirho { .. } => {
            ExprChirho::ParenChirho {
                inner_chirho: Box::new(right_chirho.clone()),
                span_chirho,
            }
        }
        _ => right_chirho.clone(),
    };
    ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(op_chirho.clone())),
            arg_chirho: Box::new(left_chirho.clone()),
            span_chirho,
        }),
        arg_chirho: Box::new(right_arg_chirho),
        span_chirho,
    }
}

/// Collect variable references from an expression (shallow walk).
fn collect_expr_refs_chirho(
    expr_chirho: &haskelujah_ast_chirho::expr_chirho::ExprChirho,
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    use haskelujah_ast_chirho::expr_chirho::ExprChirho;
    match expr_chirho {
        ExprChirho::VarChirho(name_chirho) => {
            refs_chirho.insert(canonical_value_name_text_chirho(name_chirho.text_chirho()));
        }
        ExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            collect_expr_refs_chirho(fun_chirho, refs_chirho);
            collect_expr_refs_chirho(arg_chirho, refs_chirho);
        }
        ExprChirho::LamChirho { body_chirho, .. } => {
            collect_expr_refs_chirho(body_chirho, refs_chirho);
        }
        ExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for bind_chirho in binds_chirho {
                collect_local_bind_refs_chirho(bind_chirho, refs_chirho);
            }
            collect_expr_refs_chirho(body_chirho, refs_chirho);
        }
        ExprChirho::IfChirho {
            cond_chirho,
            then_chirho,
            else_chirho,
            ..
        } => {
            collect_expr_refs_chirho(cond_chirho, refs_chirho);
            collect_expr_refs_chirho(then_chirho, refs_chirho);
            collect_expr_refs_chirho(else_chirho, refs_chirho);
        }
        ExprChirho::MultiWayIfChirho { arms_chirho, .. } => {
            collect_guarded_arm_refs_chirho(arms_chirho, refs_chirho);
        }
        ExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            collect_expr_refs_chirho(scrutinee_chirho, refs_chirho);
            for alt_chirho in alts_chirho {
                collect_rhs_refs_chirho(&alt_chirho.rhs_chirho, refs_chirho);
                for wb_chirho in &alt_chirho.where_binds_chirho {
                    collect_local_bind_refs_chirho(wb_chirho, refs_chirho);
                }
            }
        }
        ExprChirho::DoChirho { stmts_chirho, .. } => {
            for stmt_chirho in stmts_chirho {
                collect_stmt_refs_chirho(stmt_chirho, refs_chirho);
            }
        }
        ExprChirho::InfixChirho {
            left_chirho,
            op_chirho,
            right_chirho,
            ..
        } => {
            collect_expr_refs_chirho(left_chirho, refs_chirho);
            refs_chirho.insert(op_chirho.text_chirho().to_string());
            collect_expr_refs_chirho(right_chirho, refs_chirho);
        }
        ExprChirho::NegChirho { expr_chirho, .. } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho)
        }
        ExprChirho::TupleChirho {
            elements_chirho, ..
        } => {
            for e_chirho in elements_chirho {
                collect_expr_refs_chirho(e_chirho, refs_chirho);
            }
        }
        ExprChirho::ListChirho {
            elements_chirho, ..
        } => {
            for e_chirho in elements_chirho {
                collect_expr_refs_chirho(e_chirho, refs_chirho);
            }
        }
        ExprChirho::TypeAppChirho { expr_chirho, .. } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho);
        }
        ExprChirho::ArithSeqChirho {
            from_chirho,
            then_chirho,
            to_chirho,
            ..
        } => {
            collect_expr_refs_chirho(from_chirho, refs_chirho);
            if let Some(t_chirho) = then_chirho {
                collect_expr_refs_chirho(t_chirho, refs_chirho);
            }
            if let Some(t_chirho) = to_chirho {
                collect_expr_refs_chirho(t_chirho, refs_chirho);
            }
        }
        ExprChirho::ListCompChirho {
            body_chirho,
            quals_chirho,
            parallel_quals_chirho,
            ..
        } => {
            collect_expr_refs_chirho(body_chirho, refs_chirho);
            for q_chirho in quals_chirho
                .iter()
                .chain(parallel_quals_chirho.iter().flatten())
            {
                collect_stmt_refs_chirho(q_chirho, refs_chirho);
            }
        }
        ExprChirho::LeftSectionChirho {
            op_chirho,
            arg_chirho,
            ..
        } => {
            refs_chirho.insert(op_chirho.text_chirho().to_string());
            collect_expr_refs_chirho(arg_chirho, refs_chirho);
        }
        ExprChirho::RightSectionChirho {
            arg_chirho,
            op_chirho,
            ..
        } => {
            collect_expr_refs_chirho(arg_chirho, refs_chirho);
            refs_chirho.insert(op_chirho.text_chirho().to_string());
        }
        ExprChirho::AnnChirho { expr_chirho, .. }
        | ExprChirho::ParenChirho {
            inner_chirho: expr_chirho,
            ..
        }
        | ExprChirho::SpliceChirho { expr_chirho, .. }
        | ExprChirho::TypedSpliceChirho { expr_chirho, .. }
        | ExprChirho::QuoteExprChirho { expr_chirho, .. } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho);
        }
        ExprChirho::RecordConChirho { fields_chirho, .. } => {
            for f_chirho in fields_chirho {
                collect_expr_refs_chirho(&f_chirho.value_chirho, refs_chirho);
            }
        }
        ExprChirho::RecordUpdateChirho {
            expr_chirho,
            fields_chirho,
            ..
        } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho);
            for f_chirho in fields_chirho {
                collect_expr_refs_chirho(&f_chirho.value_chirho, refs_chirho);
            }
        }
        // Literals, constructors, quote decls/types/pats have no variable refs
        _ => {}
    }
}

fn collect_local_bind_refs_chirho(
    bind_chirho: &haskelujah_ast_chirho::expr_chirho::LocalBindChirho,
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    match bind_chirho {
        haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
            matches_chirho,
            ..
        } => {
            for arm_chirho in matches_chirho {
                collect_rhs_refs_chirho(&arm_chirho.rhs_chirho, refs_chirho);
            }
        }
        haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
            rhs_chirho, ..
        } => {
            collect_rhs_refs_chirho(rhs_chirho, refs_chirho);
        }
        _ => {}
    }
}

fn collect_rhs_refs_chirho(
    rhs_chirho: &haskelujah_ast_chirho::expr_chirho::RhsChirho,
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    use haskelujah_ast_chirho::expr_chirho::RhsChirho;
    match rhs_chirho {
        RhsChirho::UnguardedChirho(expr_chirho) => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho)
        }
        RhsChirho::GuardedChirho(arms_chirho) => {
            collect_guarded_arm_refs_chirho(arms_chirho, refs_chirho)
        }
    }
}

/// Guarded arms, a right-hand side's or a multi-way if's.
fn collect_guarded_arm_refs_chirho(
    arms_chirho: &[haskelujah_ast_chirho::expr_chirho::GuardedExprChirho],
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    for ge_chirho in arms_chirho {
        for qual_chirho in &ge_chirho.quals_chirho {
            collect_stmt_refs_chirho(qual_chirho, refs_chirho);
        }
        collect_expr_refs_chirho(&ge_chirho.body_chirho, refs_chirho);
    }
}

/// A do statement, a comprehension qualifier, or a guard qualifier.
fn collect_stmt_refs_chirho(
    stmt_chirho: &StmtChirho,
    refs_chirho: &mut std::collections::HashSet<String>,
) {
    match stmt_chirho {
        StmtChirho::ExprChirho { expr_chirho, .. } | StmtChirho::BindChirho { expr_chirho, .. } => {
            collect_expr_refs_chirho(expr_chirho, refs_chirho)
        }
        StmtChirho::LetChirho { binds_chirho, .. } => {
            for b_chirho in binds_chirho {
                collect_local_bind_refs_chirho(b_chirho, refs_chirho);
            }
        }
    }
}

fn collect_module_shadowed_names_chirho(
    module_chirho: &ModuleChirho,
) -> std::collections::HashSet<String> {
    let mut names_chirho = std::collections::HashSet::new();
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. }
            | DeclChirho::TypeSigChirho { name_chirho, .. } => {
                names_chirho.insert(canonical_value_name_text_chirho(name_chirho.text_chirho()));
            }
            DeclChirho::PatBindChirho { pat_chirho, .. } => {
                for name_chirho in crate::linearity_chirho::pat_bound_names_chirho(pat_chirho) {
                    names_chirho.insert(name_chirho);
                }
            }
            DeclChirho::DataDeclChirho {
                constructors_chirho,
                ..
            } => {
                for constructor_chirho in constructors_chirho {
                    match constructor_chirho {
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                            name_chirho,
                            ..
                        }
                        | haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                            name_chirho,
                            ..
                        } => {
                            names_chirho.insert(name_chirho.text_chirho().to_string());
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                            name_chirho,
                            fields_chirho,
                            ..
                        } => {
                            names_chirho.insert(name_chirho.text_chirho().to_string());
                            for field_decl_chirho in fields_chirho {
                                for field_name_chirho in &field_decl_chirho.names_chirho {
                                    names_chirho
                                        .insert(field_name_chirho.text_chirho().to_string());
                                }
                            }
                        }
                    }
                }
            }
            DeclChirho::NewtypeDeclChirho {
                constructor_chirho, ..
            } => match constructor_chirho {
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                    name_chirho,
                    ..
                }
                | haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                    name_chirho,
                    ..
                } => {
                    names_chirho.insert(name_chirho.text_chirho().to_string());
                }
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                    name_chirho,
                    fields_chirho,
                    ..
                } => {
                    names_chirho.insert(name_chirho.text_chirho().to_string());
                    for field_decl_chirho in fields_chirho {
                        for field_name_chirho in &field_decl_chirho.names_chirho {
                            names_chirho.insert(field_name_chirho.text_chirho().to_string());
                        }
                    }
                }
            },
            _ => {}
        }
    }
    names_chirho
}

/// Compute binding groups for top-level functions using SCC analysis.
/// Returns groups in topological order (leaf dependencies first).
/// Strongly connected binding groups in dependency order. A reference to a
/// binding that has a type signature is not a dependency edge (Haskell 2010
/// §4.5.1): its uses see the signature, so `f` with a signature and an
/// unsigned `g` that call each other are inferred separately and `g` stays
/// polymorphic.
fn binding_groups_chirho(
    fun_names_chirho: &[String],
    fun_decls_chirho: &[(usize, &[haskelujah_ast_chirho::expr_chirho::MatchArmChirho])],
    signed_names_chirho: &HashSet<String>,
) -> Vec<Vec<usize>> {
    let name_set_chirho: std::collections::HashSet<&str> =
        fun_names_chirho.iter().map(|s| s.as_str()).collect();
    let name_to_idx_chirho: std::collections::HashMap<&str, usize> = fun_names_chirho
        .iter()
        .enumerate()
        .map(|(i, s)| (s.as_str(), i))
        .collect();

    // Build adjacency list
    let n_chirho = fun_names_chirho.len();
    let mut adj_chirho: Vec<Vec<usize>> = vec![Vec::new(); n_chirho];
    for (i_chirho, (_, matches_chirho)) in fun_decls_chirho.iter().enumerate() {
        let mut refs_chirho = std::collections::HashSet::new();
        for arm_chirho in *matches_chirho {
            collect_rhs_refs_chirho(&arm_chirho.rhs_chirho, &mut refs_chirho);
            for wb_chirho in &arm_chirho.where_binds_chirho {
                collect_local_bind_refs_chirho(wb_chirho, &mut refs_chirho);
            }
        }
        // DETERMINISM: `refs_chirho` is a HashSet, so iterating it put the dependency
        // edges into `adj_chirho` in per-process hash order. That order decides the
        // binding-group traversal, which decides the order declarations are inferred,
        // which decides type-variable ALLOCATION — so a hash seed reached the inferred
        // types themselves. This is upstream of the defaulting loops and is why sorting
        // those alone did not make compilation reproducible.
        // See spec-chirho/bug-nondeterministic-typecheck-chirho.md
        let mut sorted_refs_chirho: Vec<_> = refs_chirho.iter().collect();
        sorted_refs_chirho.sort_unstable();
        for ref_name_chirho in sorted_refs_chirho {
            if name_set_chirho.contains(ref_name_chirho.as_str())
                && ref_name_chirho != &fun_names_chirho[i_chirho]
                && !signed_names_chirho.contains(ref_name_chirho.as_str())
            {
                if let Some(&j_chirho) = name_to_idx_chirho.get(ref_name_chirho.as_str()) {
                    adj_chirho[i_chirho].push(j_chirho);
                }
            }
        }
    }

    // Tarjan's SCC
    let mut index_chirho = 0usize;
    let mut stack_chirho: Vec<usize> = Vec::new();
    let mut on_stack_chirho = vec![false; n_chirho];
    let mut indices_chirho: Vec<Option<usize>> = vec![None; n_chirho];
    let mut lowlinks_chirho = vec![0usize; n_chirho];
    let mut sccs_chirho: Vec<Vec<usize>> = Vec::new();

    fn strongconnect_chirho(
        v_chirho: usize,
        adj_chirho: &[Vec<usize>],
        index_chirho: &mut usize,
        stack_chirho: &mut Vec<usize>,
        on_stack_chirho: &mut [bool],
        indices_chirho: &mut [Option<usize>],
        lowlinks_chirho: &mut [usize],
        sccs_chirho: &mut Vec<Vec<usize>>,
    ) {
        indices_chirho[v_chirho] = Some(*index_chirho);
        lowlinks_chirho[v_chirho] = *index_chirho;
        *index_chirho += 1;
        stack_chirho.push(v_chirho);
        on_stack_chirho[v_chirho] = true;

        for &w_chirho in &adj_chirho[v_chirho] {
            if indices_chirho[w_chirho].is_none() {
                strongconnect_chirho(
                    w_chirho,
                    adj_chirho,
                    index_chirho,
                    stack_chirho,
                    on_stack_chirho,
                    indices_chirho,
                    lowlinks_chirho,
                    sccs_chirho,
                );
                lowlinks_chirho[v_chirho] =
                    lowlinks_chirho[v_chirho].min(lowlinks_chirho[w_chirho]);
            } else if on_stack_chirho[w_chirho] {
                lowlinks_chirho[v_chirho] =
                    lowlinks_chirho[v_chirho].min(indices_chirho[w_chirho].unwrap());
            }
        }

        if lowlinks_chirho[v_chirho] == indices_chirho[v_chirho].unwrap() {
            let mut scc_chirho = Vec::new();
            loop {
                let w_chirho = stack_chirho.pop().unwrap();
                on_stack_chirho[w_chirho] = false;
                scc_chirho.push(w_chirho);
                if w_chirho == v_chirho {
                    break;
                }
            }
            sccs_chirho.push(scc_chirho);
        }
    }

    for v_chirho in 0..n_chirho {
        if indices_chirho[v_chirho].is_none() {
            strongconnect_chirho(
                v_chirho,
                &adj_chirho,
                &mut index_chirho,
                &mut stack_chirho,
                &mut on_stack_chirho,
                &mut indices_chirho,
                &mut lowlinks_chirho,
                &mut sccs_chirho,
            );
        }
    }

    // Tarjan naturally produces SCCs with dependencies first (leaves before
    // dependents), which is the order we need for type inference.
    sccs_chirho
}

/// Collect the spine of a left-nested `AppChirho` into (head, [args]).
fn collect_app_spine_chirho(ty_chirho: &TyChirho) -> (TyChirho, Vec<TyChirho>) {
    let mut head_chirho = ty_chirho.clone();
    let mut args_chirho = Vec::new();
    loop {
        match head_chirho {
            TyChirho::AppChirho(f_chirho, a_chirho) => {
                args_chirho.push(*a_chirho);
                head_chirho = *f_chirho;
            }
            _ => break,
        }
    }
    args_chirho.reverse();
    (head_chirho, args_chirho)
}

/// Substitute a named type variable in a `TyChirho`. Used for expanding
/// parameterised type synonyms where the RHS contains named variables
/// (e.g. `type Pair a = (a, a)` — substitute `a` for the applied type).
fn subst_named_var_chirho(
    ty_chirho: &TyChirho,
    name_chirho: &str,
    replacement_chirho: &TyChirho,
) -> TyChirho {
    match ty_chirho {
        TyChirho::ConChirho(n_chirho) if n_chirho == name_chirho => replacement_chirho.clone(),
        TyChirho::ForallVarChirho(n_chirho) if n_chirho == name_chirho => {
            replacement_chirho.clone()
        }
        TyChirho::AppChirho(f_chirho, a_chirho) => TyChirho::AppChirho(
            Box::new(subst_named_var_chirho(
                f_chirho,
                name_chirho,
                replacement_chirho,
            )),
            Box::new(subst_named_var_chirho(
                a_chirho,
                name_chirho,
                replacement_chirho,
            )),
        ),
        TyChirho::FunChirho(a_chirho, b_chirho, _) => TyChirho::FunChirho(
            Box::new(subst_named_var_chirho(
                a_chirho,
                name_chirho,
                replacement_chirho,
            )),
            Box::new(subst_named_var_chirho(
                b_chirho,
                name_chirho,
                replacement_chirho,
            )),
            MultChirho::ManyChirho,
        ),
        TyChirho::ListChirho(el_chirho) => TyChirho::ListChirho(Box::new(subst_named_var_chirho(
            el_chirho,
            name_chirho,
            replacement_chirho,
        ))),
        TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
            elems_chirho
                .iter()
                .map(|e_chirho| subst_named_var_chirho(e_chirho, name_chirho, replacement_chirho))
                .collect(),
        ),
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } => TyChirho::ForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(subst_named_var_chirho(
                body_chirho,
                name_chirho,
                replacement_chirho,
            )),
        },
        _ => ty_chirho.clone(),
    }
}

/// Build the applied type `Map k v` for Data.Map builtin signatures. (WI-005)
///
/// The Core primop bodies (dict_chirho/prelude_chirho.rs) already carry the real
/// map type; several builtin schemes historically faked the map argument/result
/// as `Int`, which typechecks in isolation but rejects any genuine `Map` value
/// (E0200 "expected `Int`, found `((Map k) v)`"). This keeps the type-env schemes
/// in agreement with the real map representation.
fn map_ty_chirho(k_chirho: TyChirho, v_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Map".to_string())),
            Box::new(k_chirho),
        )),
        Box::new(v_chirho),
    )
}

// ---------------------------------------------------------------------------
// Built-in type environment
// ---------------------------------------------------------------------------

pub fn builtin_value_schemes_chirho() -> HashMap<String, SchemeChirho> {
    let mut env_chirho = TyEnvChirho::new_chirho();
    seed_builtins_chirho(&mut env_chirho);
    env_chirho
        .all_bindings_chirho()
        .into_iter()
        .map(|(name_chirho, scheme_chirho)| (name_chirho.clone(), scheme_chirho.clone()))
        .collect()
}

// ---------------------------------------------------------------------------
// Literal type inference
// ---------------------------------------------------------------------------

fn infer_lit_chirho(lit_chirho: &LitChirho) -> TyChirho {
    match lit_chirho {
        LitChirho::IntChirho(..) => TyChirho::int_chirho(),
        LitChirho::FloatChirho(..) => TyChirho::double_chirho(),
        LitChirho::CharChirho(..) => TyChirho::char_chirho(),
        LitChirho::StringChirho(..) => TyChirho::string_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Run type inference on a module. This is the main entry point.
pub fn infer_module_chirho(module_chirho: &ModuleChirho) -> InferResultChirho {
    let safe_unqualified_imported_type_names_chirho = HashSet::new();
    let preferred_qualified_type_names_chirho = HashMap::new();
    infer_module_with_imports_and_type_synonyms_chirho(
        module_chirho,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &safe_unqualified_imported_type_names_chirho,
        &preferred_qualified_type_names_chirho,
    )
}

/// Run type inference on a module with pre-seeded type schemes from imported
/// modules. Each entry maps a name (e.g. `"add1"`) to the type scheme that
/// was inferred in the exporting module.
pub fn infer_module_with_imports_chirho(
    module_chirho: &ModuleChirho,
    imported_types_chirho: &HashMap<String, SchemeChirho>,
) -> InferResultChirho {
    let safe_unqualified_imported_type_names_chirho = HashSet::new();
    let preferred_qualified_type_names_chirho = HashMap::new();
    infer_module_with_imports_and_type_synonyms_chirho(
        module_chirho,
        imported_types_chirho,
        &HashMap::new(),
        &HashMap::new(),
        &safe_unqualified_imported_type_names_chirho,
        &preferred_qualified_type_names_chirho,
    )
}

fn is_placeholder_import_scheme_chirho(scheme_chirho: &SchemeChirho) -> bool {
    if !scheme_chirho.preds_chirho.is_empty() || scheme_chirho.vars_chirho.len() != 1 {
        return false;
    }
    match &scheme_chirho.ty_chirho {
        TyChirho::VarChirho(var_chirho) => {
            *var_chirho == scheme_chirho.vars_chirho[0] && var_chirho.0 >= 9000
        }
        _ => false,
    }
}

/// Run type inference on a module with pre-seeded imported value schemes and
/// imported type synonym definitions from upstream modules.
pub fn infer_module_with_imports_and_type_synonyms_chirho(
    module_chirho: &ModuleChirho,
    imported_types_chirho: &HashMap<String, SchemeChirho>,
    imported_type_synonyms_chirho: &HashMap<String, (Vec<String>, TypeChirho)>,
    imported_record_field_names_chirho: &HashMap<String, Vec<String>>,
    safe_unqualified_imported_type_names_chirho: &HashSet<String>,
    preferred_qualified_type_names_chirho: &HashMap<String, String>,
) -> InferResultChirho {
    let imported_type_families_chirho = TypeFamilyEnvChirho::new();
    infer_module_with_imports_type_synonyms_and_families_chirho(
        module_chirho,
        imported_types_chirho,
        imported_type_synonyms_chirho,
        &imported_type_families_chirho,
        imported_record_field_names_chirho,
        safe_unqualified_imported_type_names_chirho,
        preferred_qualified_type_names_chirho,
    )
}

pub fn infer_module_with_imports_type_synonyms_and_families_chirho(
    module_chirho: &ModuleChirho,
    imported_types_chirho: &HashMap<String, SchemeChirho>,
    imported_type_synonyms_chirho: &HashMap<String, (Vec<String>, TypeChirho)>,
    imported_type_families_chirho: &TypeFamilyEnvChirho,
    imported_record_field_names_chirho: &HashMap<String, Vec<String>>,
    safe_unqualified_imported_type_names_chirho: &HashSet<String>,
    preferred_qualified_type_names_chirho: &HashMap<String, String>,
) -> InferResultChirho {
    let imported_class_env_chirho = ClassEnvChirho::new_chirho();
    infer_module_with_imports_type_synonyms_families_and_class_env_chirho(
        module_chirho,
        imported_types_chirho,
        imported_type_synonyms_chirho,
        imported_type_families_chirho,
        &imported_class_env_chirho,
        imported_record_field_names_chirho,
        safe_unqualified_imported_type_names_chirho,
        preferred_qualified_type_names_chirho,
    )
}

pub fn infer_module_with_imports_type_synonyms_families_and_class_env_chirho(
    module_chirho: &ModuleChirho,
    imported_types_chirho: &HashMap<String, SchemeChirho>,
    imported_type_synonyms_chirho: &HashMap<String, (Vec<String>, TypeChirho)>,
    imported_type_families_chirho: &TypeFamilyEnvChirho,
    imported_class_env_chirho: &ClassEnvChirho,
    imported_record_field_names_chirho: &HashMap<String, Vec<String>>,
    safe_unqualified_imported_type_names_chirho: &HashSet<String>,
    preferred_qualified_type_names_chirho: &HashMap<String, String>,
) -> InferResultChirho {
    let mut ctx_chirho =
        InferCtxChirho::new_with_imported_class_env_chirho(imported_class_env_chirho);
    ctx_chirho.set_imported_type_name_preferences_chirho(
        safe_unqualified_imported_type_names_chirho,
        preferred_qualified_type_names_chirho,
    );
    ctx_chirho.overloaded_strings_chirho = module_chirho
        .extensions_chirho
        .iter()
        .any(|extension_chirho| extension_chirho == "OverloadedStrings");
    // DETERMINISM: these seeding loops iterate `HashMap`s and MUTATE inference
    // state, so a per-process hash seed would otherwise make the resulting
    // environment — and therefore the accept/reject decision — differ between
    // runs of the same binary on the same source. Sort by key so the order is a
    // function of the program, not of the hasher.
    // See spec-chirho/bug-nondeterministic-typecheck-chirho.md
    let mut sorted_type_synonyms_chirho: Vec<_> = imported_type_synonyms_chirho.iter().collect();
    sorted_type_synonyms_chirho.sort_by(|a_chirho, b_chirho| a_chirho.0.cmp(b_chirho.0));
    for (name_chirho, (params_chirho, rhs_ast_chirho)) in sorted_type_synonyms_chirho {
        let rhs_ty_chirho = ast_type_to_syn_rhs_chirho(rhs_ast_chirho, params_chirho);
        ctx_chirho.register_type_synonym_chirho(
            name_chirho.clone(),
            params_chirho.clone(),
            rhs_ty_chirho,
        );
    }
    let mut sorted_type_families_chirho: Vec<_> = imported_type_families_chirho.iter().collect();
    sorted_type_families_chirho.sort_by(|a_chirho, b_chirho| a_chirho.0.cmp(b_chirho.0));
    for (family_name_chirho, equations_chirho) in sorted_type_families_chirho {
        ctx_chirho
            .register_type_family_chirho(family_name_chirho.clone(), equations_chirho.clone());
    }
    // Seed the type environment with imported type schemes, but only if
    // placeholder imports do not override precise built-ins, while real
    // imported schemes from previously checked modules do override built-ins.
    // This lets modules like Text.Parsec.Combinator see Text.Parsec.Prim.try
    // instead of the unrelated builtin Control.Exception.try.
    // DETERMINISM: order matters here more than anywhere else in this function —
    // whether a scheme is bound depends on `lookup_chirho(..).is_none()`, i.e. on
    // what was already bound. Iterating a hash map meant the winner between a
    // placeholder and a real import could change from run to run.
    let mut sorted_imported_types_chirho: Vec<_> = imported_types_chirho.iter().collect();
    sorted_imported_types_chirho.sort_by(|a_chirho, b_chirho| a_chirho.0.cmp(b_chirho.0));
    for (name_chirho, scheme_chirho) in sorted_imported_types_chirho {
        let should_override_chirho = !is_placeholder_import_scheme_chirho(scheme_chirho);
        if should_override_chirho || ctx_chirho.env_chirho.lookup_chirho(name_chirho).is_none() {
            ctx_chirho
                .env_chirho
                .bind_chirho(name_chirho.clone(), scheme_chirho.clone());
        }
    }
    let mut sorted_record_fields_chirho: Vec<_> =
        imported_record_field_names_chirho.iter().collect();
    sorted_record_fields_chirho.sort_by(|a_chirho, b_chirho| a_chirho.0.cmp(b_chirho.0));
    for (constructor_name_chirho, field_names_chirho) in sorted_record_fields_chirho {
        ctx_chirho
            .con_field_names_chirho
            .insert(constructor_name_chirho.clone(), field_names_chirho.clone());
    }
    let subst_chirho = ctx_chirho.infer_module_chirho(module_chirho);
    let default_subst_chirho = ctx_chirho.check_deferred_preds_chirho(&subst_chirho);
    if !default_subst_chirho.is_empty_chirho() {
        ctx_chirho.apply_subst_all_chirho(&default_subst_chirho);
    }
    let composed_subst_chirho = default_subst_chirho.compose_chirho(&subst_chirho);
    // Evidence-threading P2: finalize occurrence records through the composed
    // substitution (so Report defaulting from check_deferred_preds_chirho is
    // reflected) before the context is consumed.
    let method_occurrences_chirho =
        ctx_chirho.finalize_occurrence_records_chirho(&composed_subst_chirho);
    let method_occurrence_totals_chirho = ctx_chirho.occurrence_counters_chirho.clone();
    let (literal_evidence_chirho, literal_evidence_by_origin_chirho) =
        ctx_chirho.finalize_literal_evidence_chirho(&composed_subst_chirho);
    let (reference_evidence_chirho, reference_evidence_by_origin_chirho) =
        ctx_chirho.finalize_reference_evidence_chirho(&composed_subst_chirho);
    let mut result_chirho = ctx_chirho.finish_chirho();
    result_chirho.subst_chirho = composed_subst_chirho;
    result_chirho.method_occurrences_chirho = method_occurrences_chirho;
    result_chirho.method_occurrence_totals_chirho = method_occurrence_totals_chirho;
    result_chirho.literal_evidence_chirho = literal_evidence_chirho;
    result_chirho.literal_evidence_by_origin_chirho = literal_evidence_by_origin_chirho;
    result_chirho.reference_evidence_chirho = reference_evidence_chirho;
    result_chirho.reference_evidence_by_origin_chirho = reference_evidence_by_origin_chirho;
    result_chirho
}

/// Collect free type variable names from AST types (for type family instances).
fn collect_free_type_vars_from_ast_chirho(types_chirho: &[TypeChirho]) -> Vec<String> {
    let mut vars_chirho = Vec::new();
    fn walk_chirho(ty_chirho: &TypeChirho, vars_chirho: &mut Vec<String>) {
        match ty_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho().to_string();
                if !vars_chirho.contains(&text_chirho) {
                    vars_chirho.push(text_chirho);
                }
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                walk_chirho(fun_chirho, vars_chirho);
                walk_chirho(arg_chirho, vars_chirho);
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                walk_chirho(arg_chirho, vars_chirho);
                walk_chirho(result_chirho, vars_chirho);
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                walk_chirho(element_chirho, vars_chirho)
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                for e_chirho in elements_chirho {
                    walk_chirho(e_chirho, vars_chirho);
                }
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                walk_chirho(inner_chirho, vars_chirho);
            }
            _ => {}
        }
    }
    for ty_chirho in types_chirho {
        walk_chirho(ty_chirho, &mut vars_chirho);
    }
    vars_chirho
}

fn canonical_type_family_name_chirho(name_chirho: &str) -> String {
    let bare_name_chirho = strip_name_qualifier_chirho(name_chirho);
    if bare_name_chirho.starts_with('(')
        && bare_name_chirho.ends_with(')')
        && bare_name_chirho.len() > 2
    {
        bare_name_chirho[1..bare_name_chirho.len() - 1].to_string()
    } else {
        bare_name_chirho.to_string()
    }
}

fn nat_type_literal_chirho(ty_chirho: &TyChirho) -> Option<u128> {
    match ty_chirho {
        TyChirho::ConChirho(name_chirho)
            if !name_chirho.is_empty()
                && name_chirho
                    .chars()
                    .all(|char_chirho| char_chirho.is_ascii_digit()) =>
        {
            name_chirho.parse::<u128>().ok()
        }
        _ => None,
    }
}

fn nat_type_con_chirho(value_chirho: u128) -> TyChirho {
    TyChirho::ConChirho(value_chirho.to_string())
}

fn bool_type_con_chirho(value_chirho: bool) -> TyChirho {
    TyChirho::ConChirho(if value_chirho { "True" } else { "False" }.to_string())
}

fn bool_type_literal_chirho(ty_chirho: &TyChirho) -> Option<bool> {
    let TyChirho::ConChirho(name_chirho) = ty_chirho else {
        return None;
    };
    match strip_name_qualifier_chirho(name_chirho).trim_start_matches('\'') {
        "True" => Some(true),
        "False" => Some(false),
        _ => None,
    }
}

fn symbol_type_literal_chirho(ty_chirho: &TyChirho) -> Option<&str> {
    match ty_chirho {
        TyChirho::ConChirho(name_chirho)
            if name_chirho.len() >= 2
                && name_chirho.starts_with('"')
                && name_chirho.ends_with('"') =>
        {
            Some(&name_chirho[1..name_chirho.len() - 1])
        }
        _ => None,
    }
}

fn symbol_type_con_chirho(value_chirho: &str) -> TyChirho {
    let escaped_chirho = value_chirho.replace('\\', "\\\\").replace('"', "\\\"");
    TyChirho::ConChirho(format!("\"{escaped_chirho}\""))
}

fn char_type_literal_chirho(ty_chirho: &TyChirho) -> Option<char> {
    let TyChirho::ConChirho(name_chirho) = ty_chirho else {
        return None;
    };
    let body_chirho = name_chirho.strip_prefix('\'')?.strip_suffix('\'')?;
    parse_haskell_char_body_chirho(body_chirho)
}

fn char_type_con_chirho(value_chirho: char) -> TyChirho {
    TyChirho::ConChirho(format!(
        "'{}'",
        render_haskell_char_body_chirho(value_chirho)
    ))
}

fn apply_extra_type_family_args_chirho(
    mut reduced_chirho: TyChirho,
    args_chirho: &[TyChirho],
    consumed_chirho: usize,
) -> TyChirho {
    for extra_arg_chirho in &args_chirho[consumed_chirho..] {
        reduced_chirho =
            TyChirho::AppChirho(Box::new(reduced_chirho), Box::new(extra_arg_chirho.clone()));
    }
    reduced_chirho
}

fn reduce_builtin_type_family_application_chirho(
    family_name_chirho: &str,
    args_chirho: &[TyChirho],
) -> Option<TyChirho> {
    let family_chirho = canonical_type_family_name_chirho(family_name_chirho);
    let reduced_chirho = match (family_chirho.as_str(), args_chirho) {
        // workflow: language-features-chirho/type-level-character-families-chirho
        ("CharToNat", [value_chirho, ..]) => char_type_literal_chirho(value_chirho)
            .map(|value_chirho| nat_type_con_chirho(u128::from(value_chirho as u32)))
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 1)),
        ("NatToChar", [value_chirho, ..]) => nat_type_literal_chirho(value_chirho)
            .and_then(|value_chirho| u32::try_from(value_chirho).ok())
            .and_then(char::from_u32)
            .map(char_type_con_chirho)
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 1)),
        ("AppendSymbol", [left_chirho, right_chirho, ..]) => {
            let left_symbol_chirho = symbol_type_literal_chirho(left_chirho);
            let right_symbol_chirho = symbol_type_literal_chirho(right_chirho);
            match (left_symbol_chirho, right_symbol_chirho) {
                (Some(""), _) => Some(right_chirho.clone()),
                (_, Some("")) => Some(left_chirho.clone()),
                (Some(left_value_chirho), Some(right_value_chirho)) => Some(
                    symbol_type_con_chirho(&format!("{left_value_chirho}{right_value_chirho}")),
                ),
                _ => None,
            }
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2))
        }
        ("+", [left_chirho, right_chirho, ..]) => match (
            nat_type_literal_chirho(left_chirho),
            nat_type_literal_chirho(right_chirho),
        ) {
            (Some(0), _) => Some(right_chirho.clone()),
            (_, Some(0)) => Some(left_chirho.clone()),
            (Some(left_value_chirho), Some(right_value_chirho)) => left_value_chirho
                .checked_add(right_value_chirho)
                .map(nat_type_con_chirho),
            _ => None,
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        ("*", [left_chirho, right_chirho, ..]) => match (
            nat_type_literal_chirho(left_chirho),
            nat_type_literal_chirho(right_chirho),
        ) {
            (Some(0), _) | (_, Some(0)) => Some(nat_type_con_chirho(0)),
            (Some(1), _) => Some(right_chirho.clone()),
            (_, Some(1)) => Some(left_chirho.clone()),
            (Some(left_value_chirho), Some(right_value_chirho)) => left_value_chirho
                .checked_mul(right_value_chirho)
                .map(nat_type_con_chirho),
            _ => None,
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        ("^", [left_chirho, right_chirho, ..]) => match (
            nat_type_literal_chirho(left_chirho),
            nat_type_literal_chirho(right_chirho),
        ) {
            (_, Some(0)) => Some(nat_type_con_chirho(1)),
            (Some(1), _) => Some(nat_type_con_chirho(1)),
            (_, Some(1)) => Some(left_chirho.clone()),
            (Some(left_value_chirho), Some(right_value_chirho)) => {
                u32::try_from(right_value_chirho)
                    .ok()
                    .and_then(|exp_chirho| left_value_chirho.checked_pow(exp_chirho))
                    .map(nat_type_con_chirho)
            }
            _ => None,
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        ("-", [left_chirho, right_chirho, ..]) => match (
            nat_type_literal_chirho(left_chirho),
            nat_type_literal_chirho(right_chirho),
        ) {
            (_, Some(0)) => Some(left_chirho.clone()),
            (Some(left_value_chirho), Some(right_value_chirho))
                if left_value_chirho >= right_value_chirho =>
            {
                Some(nat_type_con_chirho(left_value_chirho - right_value_chirho))
            }
            _ => None,
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        ("Div", [left_chirho, right_chirho, ..]) => match (
            nat_type_literal_chirho(left_chirho),
            nat_type_literal_chirho(right_chirho),
        ) {
            (_, Some(1)) => Some(left_chirho.clone()),
            (Some(left_value_chirho), Some(right_value_chirho)) if right_value_chirho != 0 => {
                Some(nat_type_con_chirho(left_value_chirho / right_value_chirho))
            }
            _ => None,
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        ("Mod", [left_chirho, right_chirho, ..]) => match (
            nat_type_literal_chirho(left_chirho),
            nat_type_literal_chirho(right_chirho),
        ) {
            (_, Some(1)) => Some(nat_type_con_chirho(0)),
            (Some(left_value_chirho), Some(right_value_chirho)) if right_value_chirho != 0 => {
                Some(nat_type_con_chirho(left_value_chirho % right_value_chirho))
            }
            _ => None,
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        ("Log2", [value_chirho, ..]) => nat_type_literal_chirho(value_chirho)
            .and_then(|value_chirho| {
                if value_chirho == 0 {
                    None
                } else {
                    Some(nat_type_con_chirho(
                        127 - value_chirho.leading_zeros() as u128,
                    ))
                }
            })
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 1)),
        ("<=?", [left_chirho, right_chirho, ..]) => if left_chirho == right_chirho {
            Some(bool_type_con_chirho(true))
        } else {
            match (
                nat_type_literal_chirho(left_chirho),
                nat_type_literal_chirho(right_chirho),
            ) {
                (Some(left_value_chirho), Some(right_value_chirho)) => Some(bool_type_con_chirho(
                    left_value_chirho <= right_value_chirho,
                )),
                _ => None,
            }
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        ("If", [condition_chirho, true_chirho, false_chirho, ..]) => {
            bool_type_literal_chirho(condition_chirho).map(|condition_chirho| {
                if condition_chirho {
                    true_chirho.clone()
                } else {
                    false_chirho.clone()
                }
            })
        }
        .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 3)),
        ("Not", [value_chirho, ..]) => bool_type_literal_chirho(value_chirho)
            .map(|value_chirho| bool_type_con_chirho(!value_chirho))
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 1)),
        ("&&", [left_chirho, right_chirho, ..]) => bool_type_literal_chirho(left_chirho)
            .map(|left_chirho| {
                if left_chirho {
                    right_chirho.clone()
                } else {
                    bool_type_con_chirho(false)
                }
            })
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        ("||", [left_chirho, right_chirho, ..]) => bool_type_literal_chirho(left_chirho)
            .map(|left_chirho| {
                if left_chirho {
                    bool_type_con_chirho(true)
                } else {
                    right_chirho.clone()
                }
            })
            .map(|ty_chirho| apply_extra_type_family_args_chirho(ty_chirho, args_chirho, 2)),
        _ => None,
    };
    reduced_chirho
}

/// Match a type family LHS pattern against a concrete type argument.
/// Type variables in the pattern bind to the corresponding argument types.
/// Type constructors must match exactly.
fn match_type_pattern_chirho(
    pat_chirho: &TyChirho,
    arg_chirho: &TyChirho,
    bindings_chirho: &mut HashMap<String, TyChirho>,
) -> bool {
    match pat_chirho {
        // A type variable in the pattern matches anything
        TyChirho::VarChirho(tv_chirho) => {
            let var_name_chirho = format!("tv{}", tv_chirho.0);
            if let Some(existing_chirho) = bindings_chirho.get(&var_name_chirho) {
                existing_chirho == arg_chirho
            } else {
                bindings_chirho.insert(var_name_chirho, arg_chirho.clone());
                true
            }
        }
        // ForallVar in the pattern — a named type variable from type family LHS
        // (e.g. `a` in `F (Maybe a) = [a]`). Matches any argument type.
        TyChirho::ForallVarChirho(name_chirho) => {
            if let Some(existing_chirho) = bindings_chirho.get(name_chirho) {
                existing_chirho == arg_chirho
            } else {
                bindings_chirho.insert(name_chirho.clone(), arg_chirho.clone());
                true
            }
        }
        // A constructor must match the same constructor
        TyChirho::ConChirho(name_chirho) => {
            matches!(arg_chirho, TyChirho::ConChirho(arg_name_chirho) if arg_name_chirho == name_chirho)
        }
        // Type application: both sides must be apps with matching structure
        TyChirho::AppChirho(f_chirho, a_chirho) => {
            if let TyChirho::AppChirho(af_chirho, aa_chirho) = arg_chirho {
                match_type_pattern_chirho(f_chirho, af_chirho, bindings_chirho)
                    && match_type_pattern_chirho(a_chirho, aa_chirho, bindings_chirho)
            } else {
                false
            }
        }
        TyChirho::ListChirho(inner_chirho) => {
            if let TyChirho::ListChirho(arg_inner_chirho) = arg_chirho {
                match_type_pattern_chirho(inner_chirho, arg_inner_chirho, bindings_chirho)
            } else {
                false
            }
        }
        TyChirho::TupleChirho(elems_chirho) => {
            if let TyChirho::TupleChirho(arg_elems_chirho) = arg_chirho {
                if elems_chirho.len() != arg_elems_chirho.len() {
                    return false;
                }
                elems_chirho
                    .iter()
                    .zip(arg_elems_chirho.iter())
                    .all(|(p_chirho, a_chirho)| {
                        match_type_pattern_chirho(p_chirho, a_chirho, bindings_chirho)
                    })
            } else {
                false
            }
        }
        TyChirho::FunChirho(a_chirho, r_chirho, _) => {
            if let TyChirho::FunChirho(aa_chirho, ar_chirho, _) = arg_chirho {
                match_type_pattern_chirho(a_chirho, aa_chirho, bindings_chirho)
                    && match_type_pattern_chirho(r_chirho, ar_chirho, bindings_chirho)
            } else {
                false
            }
        }
        // Int, Bool, Char literals — exact match only
        _ => pat_chirho == arg_chirho,
    }
}

/// Substitute type variables in a type family equation RHS using the
/// bindings collected during pattern matching.
fn substitute_type_vars_chirho(
    ty_chirho: &TyChirho,
    bindings_chirho: &HashMap<String, TyChirho>,
) -> TyChirho {
    match ty_chirho {
        TyChirho::VarChirho(tv_chirho) => {
            let var_name_chirho = format!("tv{}", tv_chirho.0);
            bindings_chirho
                .get(&var_name_chirho)
                .cloned()
                .unwrap_or_else(|| ty_chirho.clone())
        }
        TyChirho::ForallVarChirho(name_chirho) => bindings_chirho
            .get(name_chirho)
            .cloned()
            .unwrap_or_else(|| ty_chirho.clone()),
        TyChirho::AppChirho(f_chirho, a_chirho) => TyChirho::AppChirho(
            Box::new(substitute_type_vars_chirho(f_chirho, bindings_chirho)),
            Box::new(substitute_type_vars_chirho(a_chirho, bindings_chirho)),
        ),
        TyChirho::FunChirho(a_chirho, r_chirho, _) => TyChirho::FunChirho(
            Box::new(substitute_type_vars_chirho(a_chirho, bindings_chirho)),
            Box::new(substitute_type_vars_chirho(r_chirho, bindings_chirho)),
            MultChirho::ManyChirho,
        ),
        TyChirho::ListChirho(inner_chirho) => TyChirho::ListChirho(Box::new(
            substitute_type_vars_chirho(inner_chirho, bindings_chirho),
        )),
        TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
            elems_chirho
                .iter()
                .map(|e_chirho| substitute_type_vars_chirho(e_chirho, bindings_chirho))
                .collect(),
        ),
        _ => ty_chirho.clone(),
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_ast_chirho::decl_chirho::{AssocTfInstanceChirho, ConDeclChirho, DeclChirho};
    use haskelujah_ast_chirho::expr_chirho::{AltChirho, GuardedExprChirho, LocalBindChirho};
    use haskelujah_ast_chirho::module_chirho::ModuleChirho;
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    use haskelujah_ast_chirho::pat_chirho::PatFieldChirho;

    fn dummy_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    #[test]
    fn infer_literal_int_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expr_chirho =
            ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO, None));
        let (_s_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        assert!(matches!(ty_chirho, TyChirho::VarChirho(_)));
        assert_eq!(ctx_chirho.deferred_preds_chirho.len(), 1);
        assert_eq!(
            ctx_chirho.deferred_preds_chirho[0].0.class_name_chirho,
            "Num"
        );
    }

    #[test]
    fn occurrence_record_captures_concrete_eq_char_chirho() {
        // Evidence-threading P2 pin: `(==) 'a' 'b'` must yield a finalized
        // occurrence record (name "==", ordinal 0, class Eq, key Char).
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let app_chirho = ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("=="))),
                arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::CharChirho(
                    'a',
                    SpanChirho::DUMMY_CHIRHO,
                    None,
                ))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::CharChirho(
                'b',
                SpanChirho::DUMMY_CHIRHO,
                None,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, _ty_chirho) = ctx_chirho.infer_expr_chirho(&app_chirho);
        let records_chirho = ctx_chirho.finalize_occurrence_records_chirho(&subst_chirho);
        assert!(
            records_chirho.iter().any(|record_chirho| {
                record_chirho.name_chirho == "=="
                    && record_chirho.ordinal_chirho == 0
                    && record_chirho.class_name_chirho == "Eq"
                    && record_chirho.ty_key_chirho == "Char"
            }),
            "expected a concrete Eq/Char record for `==`, got: {records_chirho:?}"
        );
    }

    /// A capture as generated code made it before provenance: no span, no origin.
    fn unspanned_capture_chirho(
        name_chirho: &str,
        ordinal_chirho: u32,
        class_name_chirho: &str,
        ty_chirho: TyChirho,
    ) -> evidence_chirho::OccurrenceCaptureChirho {
        evidence_chirho::OccurrenceCaptureChirho {
            name_chirho: name_chirho.to_string(),
            ordinal_chirho,
            class_name_chirho: class_name_chirho.to_string(),
            ty_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_chirho: None,
        }
    }

    #[test]
    fn occurrence_record_defaults_unresolved_standard_class_chirho() {
        // Evidence-threading P2c: local/generalized numeric expressions can
        // leave method occurrence predicates as bare vars even after the module
        // substitution. Use the backed runtime default key rather than letting
        // a free `+`/`==` reach STG.
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let unresolved_ty_chirho = ctx_chirho.fresh_var_chirho();
        ctx_chirho
            .occurrence_captures_chirho
            .push(unspanned_capture_chirho(
                "+",
                0,
                "Num",
                unresolved_ty_chirho,
            ));
        let records_chirho =
            ctx_chirho.finalize_occurrence_records_chirho(&SubstChirho::empty_chirho());
        assert_eq!(
            records_chirho,
            vec![MethodOccurrenceRecordChirho {
                name_chirho: "+".to_string(),
                ordinal_chirho: 0,
                class_name_chirho: "Num".to_string(),
                ty_key_chirho: "Int".to_string(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
                origin_chirho: None,
            }]
        );
    }

    #[test]
    fn occurrence_record_defaults_fractional_literal_method_to_double_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let app_chirho = ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("negate"))),
            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::FloatChirho(
                5.5,
                SpanChirho::DUMMY_CHIRHO,
                None,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, _ty_chirho) = ctx_chirho.infer_expr_chirho(&app_chirho);
        let default_subst_chirho = ctx_chirho.check_deferred_preds_chirho(&subst_chirho);
        if !default_subst_chirho.is_empty_chirho() {
            ctx_chirho.apply_subst_all_chirho(&default_subst_chirho);
        }
        let composed_subst_chirho = default_subst_chirho.compose_chirho(&subst_chirho);
        let records_chirho = ctx_chirho.finalize_occurrence_records_chirho(&composed_subst_chirho);
        assert!(
            records_chirho.iter().any(|record_chirho| {
                record_chirho.name_chirho == "negate"
                    && record_chirho.ordinal_chirho == 0
                    && record_chirho.class_name_chirho == "Num"
                    && record_chirho.ty_key_chirho == "Double"
            }),
            "expected Num/Double evidence for `negate 5.5`, got: {records_chirho:?}"
        );
    }

    #[test]
    fn occurrence_record_retains_concrete_structured_show_keys_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.occurrence_captures_chirho.extend([
            unspanned_capture_chirho(
                "print",
                0,
                "Show",
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
            ),
            unspanned_capture_chirho(
                "print",
                1,
                "Show",
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::string_chirho()),
                ),
            ),
            unspanned_capture_chirho(
                "print",
                2,
                "Show",
                TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::string_chirho()]),
            ),
        ]);

        let records_chirho =
            ctx_chirho.finalize_occurrence_records_chirho(&SubstChirho::empty_chirho());
        let keys_chirho = records_chirho
            .iter()
            .map(|record_chirho| record_chirho.ty_key_chirho.as_str())
            .collect::<Vec<_>>();

        assert_eq!(keys_chirho, vec!["[Int]", "Maybe String", "(Int,String)"]);
    }

    #[test]
    fn print_occurrence_uses_payload_annotation_for_maybe_key_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let annotated_payload_chirho = ExprChirho::AnnChirho {
            expr_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                3,
                SpanChirho::DUMMY_CHIRHO,
                None,
            ))),
            ty_chirho: TypeChirho::ConChirho(dummy_name_chirho("Int")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let maybe_value_chirho = ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("Just"))),
            arg_chirho: Box::new(annotated_payload_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let print_app_chirho = ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("print"))),
            arg_chirho: Box::new(maybe_value_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (subst_chirho, _ty_chirho) = ctx_chirho.infer_expr_chirho(&print_app_chirho);
        let default_subst_chirho = ctx_chirho.check_deferred_preds_chirho(&subst_chirho);
        let composed_subst_chirho = default_subst_chirho.compose_chirho(&subst_chirho);
        let records_chirho = ctx_chirho.finalize_occurrence_records_chirho(&composed_subst_chirho);

        assert!(
            records_chirho.iter().any(|record_chirho| {
                record_chirho.name_chirho == "print"
                    && record_chirho.ordinal_chirho == 0
                    && record_chirho.class_name_chirho == "Show"
                    && record_chirho.ty_key_chirho == "Maybe Int"
            }),
            "payload annotation should resolve print evidence to Maybe Int, got: {records_chirho:?}"
        );
    }

    #[test]
    fn infer_literal_string_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::LitChirho(LitChirho::StringChirho(
            "hello".to_string(),
            SpanChirho::DUMMY_CHIRHO,
            None,
        ));
        let (_s_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        assert_eq!(ty_chirho, TyChirho::string_chirho());
    }

    #[test]
    fn infer_case_unboxed_tuple_hash_primed_binders_stay_in_scope_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.env_chirho.bind_chirho(
            "foo#".to_string(),
            SchemeChirho::mono_chirho(TyChirho::TupleChirho(vec![
                TyChirho::ConChirho("State#".to_string()),
                TyChirho::ConChirho("ByteArray#".to_string()),
            ])),
        );
        ctx_chirho.env_chirho.bind_chirho(
            "ByteArray".to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                TyChirho::ConChirho("ByteArray#".to_string()),
                TyChirho::ConChirho("ByteArray".to_string()),
            )),
        );

        let expr_chirho = ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("foo#"))),
            alts_chirho: vec![AltChirho {
                pat_chirho: PatChirho::TupleChirho {
                    elements_chirho: vec![
                        PatChirho::VarChirho(dummy_name_chirho("s'#")),
                        PatChirho::VarChirho(dummy_name_chirho("arr'#")),
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::TupleChirho {
                    elements_chirho: vec![
                        ExprChirho::VarChirho(dummy_name_chirho("s'#")),
                        ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                                "ByteArray",
                            ))),
                            arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("arr'#"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (_subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        assert!(
            !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
            "hash-primed unboxed-tuple binders should stay in scope inside the case rhs: {:?}",
            ctx_chirho.diagnostics_chirho
        );
        assert_eq!(
            inferred_ty_chirho,
            TyChirho::TupleChirho(vec![
                TyChirho::ConChirho("State#".to_string()),
                TyChirho::ConChirho("ByteArray".to_string()),
            ]),
            "the rhs should see both s'# and arr'# as locally bound names"
        );
    }

    #[test]
    fn infer_top_level_magic_hash_pattern_binder_survives_unsafe_coerce_rhs_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.env_chirho.bind_chirho(
            "primitive".to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(9100)),
                TyChirho::VarChirho(TyVarChirho(9101)),
            )),
        );

        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("PrimitiveMiniChirho"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("ByteArray"),
                    type_vars_chirho: vec![],
                    constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho("ByteArray"),
                        fields_chirho: vec![(
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            TypeChirho::ConChirho(dummy_name_chirho("ByteArray#")),
                        )],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("MutableByteArray"),
                    type_vars_chirho: vec![dummy_name_chirho("s").into()],
                    constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho("MutableByteArray"),
                        fields_chirho: vec![(
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            TypeChirho::ConChirho(dummy_name_chirho("ByteArray#")),
                        )],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("unsafeThawByteArray"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::ConChirho {
                            con_chirho: dummy_name_chirho("ByteArray"),
                            args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("arr#"))],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                "primitive",
                            ))),
                            arg_chirho: Box::new(ExprChirho::ParenChirho {
                                inner_chirho: Box::new(ExprChirho::LamChirho {
                                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho(
                                        "s#",
                                    ))],
                                    body_chirho: Box::new(ExprChirho::TupleChirho {
                                        elements_chirho: vec![
                                            ExprChirho::VarChirho(dummy_name_chirho("s#")),
                                            ExprChirho::AppChirho {
                                                fun_chirho: Box::new(ExprChirho::ConChirho(
                                                    dummy_name_chirho("MutableByteArray"),
                                                )),
                                                arg_chirho: Box::new(ExprChirho::ParenChirho {
                                                    inner_chirho: Box::new(ExprChirho::AppChirho {
                                                        fun_chirho: Box::new(
                                                            ExprChirho::VarChirho(
                                                                dummy_name_chirho("unsafeCoerce#"),
                                                            ),
                                                        ),
                                                        arg_chirho: Box::new(
                                                            ExprChirho::VarChirho(
                                                                dummy_name_chirho("arr#"),
                                                            ),
                                                        ),
                                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                                    }),
                                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                                }),
                                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                                            },
                                        ],
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let _subst_chirho = ctx_chirho.infer_module_chirho(&module_chirho);
        assert!(
            !ctx_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .any(|diag_chirho| diag_chirho
                    .message_chirho
                    .contains("unbound variable: `arr`")),
            "top-level hash binder should not degrade to bare arr in the rhs: {:?}",
            ctx_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn infer_identity_function_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // \x -> x
        let expr_chirho = ExprChirho::LamChirho {
            pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
            body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);

        // Should be `a -> a` for some variable a
        match &final_ty_chirho {
            TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
                assert_eq!(arg_chirho, result_chirho, "identity should have a -> a");
            }
            other_chirho => panic!("expected function type, got {other_chirho}"),
        }
    }

    #[test]
    fn infer_application_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // not True
        let expr_chirho = ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("not"))),
            arg_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(final_ty_chirho, TyChirho::bool_chirho());
    }

    #[test]
    fn infer_visible_type_application_specializes_named_scheme_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let a_chirho = TyVarChirho(9000);
        ctx_chirho.env_chirho.bind_chirho(
            "idTy".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );

        let expr_chirho = ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::TypeAppChirho {
                expr_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("idTy"))),
                ty_chirho: TypeChirho::ConChirho(dummy_name_chirho("Int")),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                1,
                SpanChirho::DUMMY_CHIRHO,
                None,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(final_ty_chirho, TyChirho::int_chirho());
    }

    #[test]
    fn infer_visible_type_application_specializes_rank_n_parameter_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let rank_n_var_chirho = TyVarChirho(9003);
        ctx_chirho.env_chirho.bind_chirho(
            "rankNParameterChirho".to_string(),
            SchemeChirho {
                vars_chirho: vec![],
                preds_chirho: vec![],
                ty_chirho: TyChirho::ForallChirho {
                    vars_chirho: vec![rank_n_var_chirho],
                    body_chirho: Box::new(TyChirho::fun_chirho(
                        TyChirho::VarChirho(rank_n_var_chirho),
                        TyChirho::VarChirho(rank_n_var_chirho),
                    )),
                },
            },
        );

        let expr_chirho = ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::TypeAppChirho {
                expr_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                    "rankNParameterChirho",
                ))),
                ty_chirho: TypeChirho::ConChirho(dummy_name_chirho("Int")),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                1,
                SpanChirho::DUMMY_CHIRHO,
                None,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert_eq!(final_ty_chirho, TyChirho::int_chirho());
    }

    #[test]
    fn infer_required_type_argument_consumes_promoted_list_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let required_var_chirho = TyVarChirho(9004);
        ctx_chirho.env_chirho.bind_chirho(
            "requiredChirho".to_string(),
            SchemeChirho::mono_chirho(TyChirho::RequiredForallChirho {
                vars_chirho: vec![required_var_chirho],
                body_chirho: Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Proxy".to_string())),
                    Box::new(TyChirho::VarChirho(required_var_chirho)),
                )),
            }),
        );
        let expr_chirho = ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("requiredChirho"))),
            arg_chirho: Box::new(ExprChirho::ListChirho {
                elements_chirho: vec![
                    ExprChirho::ConChirho(dummy_name_chirho("Num")),
                    ExprChirho::ConChirho(dummy_name_chirho("Show")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        let expected_list_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho(":".to_string())),
                Box::new(TyChirho::ConChirho("Num".to_string())),
            )),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho(":".to_string())),
                    Box::new(TyChirho::ConChirho("Show".to_string())),
                )),
                Box::new(TyChirho::ConChirho("'[]".to_string())),
            )),
        );
        assert_eq!(
            final_ty_chirho,
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Proxy".to_string())),
                Box::new(expected_list_chirho),
            )
        );
    }

    #[test]
    fn infer_type_lambda_checks_against_outer_forall_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expected_var_chirho = TyVarChirho(9005);
        let expected_ty_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![expected_var_chirho],
            body_chirho: Box::new(TyChirho::fun_chirho(
                TyChirho::VarChirho(expected_var_chirho),
                TyChirho::VarChirho(expected_var_chirho),
            )),
        };
        let value_name_chirho = dummy_name_chirho("valueChirho");
        let expr_chirho = ExprChirho::TypeLamChirho {
            binder_chirho: dummy_name_chirho("typeChirho"),
            body_chirho: Box::new(ExprChirho::LamChirho {
                pats_chirho: vec![PatChirho::VarChirho(value_name_chirho.clone())],
                body_chirho: Box::new(ExprChirho::VarChirho(value_name_chirho)),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (_, inferred_ty_chirho) = ctx_chirho.infer_expr_against_expected_chirho(
            &expr_chirho,
            &expected_ty_chirho,
            SpanChirho::DUMMY_CHIRHO,
        );
        assert_eq!(inferred_ty_chirho, expected_ty_chirho);
        assert!(ctx_chirho.diagnostics_chirho.is_empty_chirho());
    }

    fn rank_n_identity_consumer_ty_chirho() -> TyChirho {
        let identity_var_chirho = TyVarChirho(9006);
        TyChirho::fun_chirho(
            TyChirho::ForallChirho {
                vars_chirho: vec![identity_var_chirho],
                body_chirho: Box::new(TyChirho::fun_chirho(
                    TyChirho::VarChirho(identity_var_chirho),
                    TyChirho::VarChirho(identity_var_chirho),
                )),
            },
            TyChirho::TupleChirho(vec![TyChirho::bool_chirho(), TyChirho::int_chirho()]),
        )
    }

    fn rank_n_identity_consumer_lambda_chirho() -> ExprChirho {
        let function_name_chirho = dummy_name_chirho("functionChirho");
        let apply_function_chirho = |arg_chirho: ExprChirho| ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(function_name_chirho.clone())),
            arg_chirho: Box::new(arg_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        ExprChirho::LamChirho {
            pats_chirho: vec![PatChirho::VarChirho(function_name_chirho.clone())],
            body_chirho: Box::new(ExprChirho::TupleChirho {
                elements_chirho: vec![
                    apply_function_chirho(ExprChirho::ConChirho(dummy_name_chirho("True"))),
                    apply_function_chirho(ExprChirho::LitChirho(LitChirho::IntChirho(
                        5,
                        SpanChirho::DUMMY_CHIRHO,
                        None,
                    ))),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn infer_rank_n_expected_type_through_if_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expected_ty_chirho = rank_n_identity_consumer_ty_chirho();
        let expr_chirho = ExprChirho::IfChirho {
            cond_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
            then_chirho: Box::new(rank_n_identity_consumer_lambda_chirho()),
            else_chirho: Box::new(rank_n_identity_consumer_lambda_chirho()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_expr_against_expected_chirho(
            &expr_chirho,
            &expected_ty_chirho,
            SpanChirho::DUMMY_CHIRHO,
        );

        assert_eq!(
            subst_chirho.apply_ty_chirho(&inferred_ty_chirho),
            expected_ty_chirho
        );
        assert!(ctx_chirho.diagnostics_chirho.is_empty_chirho());
    }

    #[test]
    fn infer_rank_n_expected_type_through_case_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expected_ty_chirho = rank_n_identity_consumer_ty_chirho();
        let bool_alt_chirho = |constructor_chirho: &str| AltChirho {
            pat_chirho: PatChirho::ConChirho {
                con_chirho: dummy_name_chirho(constructor_chirho),
                args_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: RhsChirho::UnguardedChirho(rank_n_identity_consumer_lambda_chirho()),
            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let expr_chirho = ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
            alts_chirho: vec![bool_alt_chirho("True"), bool_alt_chirho("False")],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_expr_against_expected_chirho(
            &expr_chirho,
            &expected_ty_chirho,
            SpanChirho::DUMMY_CHIRHO,
        );

        assert_eq!(
            subst_chirho.apply_ty_chirho(&inferred_ty_chirho),
            expected_ty_chirho
        );
        assert!(ctx_chirho.diagnostics_chirho.is_empty_chirho());
    }

    #[test]
    fn infer_visible_type_application_reuses_scoped_type_variable_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let scoped_m_chirho = TyVarChirho(9001);
        let n_chirho = TyVarChirho(9002);
        ctx_chirho
            .scoped_tyvars_chirho
            .insert("m".to_string(), scoped_m_chirho);
        ctx_chirho.env_chirho.bind_chirho(
            "proof".to_string(),
            SchemeChirho {
                vars_chirho: vec![n_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Dict".to_string())),
                    Box::new(TyChirho::VarChirho(n_chirho)),
                ),
            },
        );

        let expr_chirho = ExprChirho::TypeAppChirho {
            expr_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("proof"))),
            ty_chirho: TypeChirho::VarChirho(dummy_name_chirho("m")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (_subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        assert_eq!(
            ty_chirho,
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Dict".to_string())),
                Box::new(TyChirho::VarChirho(scoped_m_chirho))
            )
        );
    }

    #[test]
    fn infer_visible_type_application_wildcard_uses_signature_equality_chirho() {
        use haskelujah_ast_chirho::ty_chirho::ConstraintChirho as AstConstraintChirho;
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let var_ast_chirho =
            |name_chirho: &str| TypeChirho::VarChirho(dummy_name_chirho(name_chirho));
        let con_ast_chirho =
            |name_chirho: &str| TypeChirho::ConChirho(dummy_name_chirho(name_chirho));
        let app_ast_chirho =
            |fun_chirho: TypeChirho, arg_chirho: TypeChirho| TypeChirho::AppChirho {
                fun_chirho: Box::new(fun_chirho),
                arg_chirho: Box::new(arg_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
        let promoted_ty_list_ast_chirho = || TypeChirho::PromotedListChirho {
            elements_chirho: vec![con_ast_chirho("Ty")],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let expr_ast_chirho = |t_chirho: TypeChirho, lrs_chirho: TypeChirho| {
            app_ast_chirho(app_ast_chirho(con_ast_chirho("Expr"), t_chirho), lrs_chirho)
        };
        let kt_ast_chirho = |lrs_chirho: TypeChirho| {
            app_ast_chirho(
                app_ast_chirho(
                    app_ast_chirho(con_ast_chirho("KT"), con_ast_chirho("A")),
                    lrs_chirho,
                ),
                con_ast_chirho("IO"),
            )
        };

        let sig_chirho = TypeChirho::ForallChirho {
            vars_chirho: vec![
                dummy_name_chirho("m").into(),
                dummy_name_chirho("t").into(),
                dummy_name_chirho("lrs").into(),
                dummy_name_chirho("bind").into(),
            ],
            body_chirho: Box::new(TypeChirho::QualChirho {
                context_chirho: vec![AstConstraintChirho::ClassChirho {
                    class_chirho: dummy_name_chirho("~"),
                    args_chirho: vec![
                        var_ast_chirho("bind"),
                        expr_ast_chirho(var_ast_chirho("t"), var_ast_chirho("lrs")),
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                body_chirho: Box::new(app_ast_chirho(var_ast_chirho("m"), var_ast_chirho("bind"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let scheme_chirho = ctx_chirho.ast_type_to_scheme_chirho(&sig_chirho);
        assert_eq!(scheme_chirho.vars_chirho.len(), 4);
        let m_var_chirho = scheme_chirho.vars_chirho[0];
        let t_var_chirho = scheme_chirho.vars_chirho[1];
        let lrs_var_chirho = scheme_chirho.vars_chirho[2];
        let bind_var_chirho = scheme_chirho.vars_chirho[3];
        let expected_scheme_body_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(m_var_chirho)),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Expr".to_string())),
                    Box::new(TyChirho::VarChirho(t_var_chirho)),
                )),
                Box::new(TyChirho::VarChirho(lrs_var_chirho)),
            )),
        );
        assert_eq!(scheme_chirho.ty_chirho, expected_scheme_body_chirho);
        assert!(
            !scheme_chirho
                .ty_chirho
                .free_vars_chirho()
                .contains(&bind_var_chirho),
            "equality-constrained result should rewrite bind out of the scheme body"
        );

        ctx_chirho
            .env_chirho
            .bind_chirho("test_gr".to_string(), scheme_chirho);
        let type_app_chirho =
            |expr_chirho: ExprChirho, ty_chirho: TypeChirho| ExprChirho::TypeAppChirho {
                expr_chirho: Box::new(expr_chirho),
                ty_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
        let expr_net_ty_list_ast_chirho =
            expr_ast_chirho(con_ast_chirho("Net"), promoted_ty_list_ast_chirho());
        let test_gr_expr_chirho = type_app_chirho(
            type_app_chirho(
                type_app_chirho(
                    type_app_chirho(
                        ExprChirho::VarChirho(dummy_name_chirho("test_gr")),
                        kt_ast_chirho(promoted_ty_list_ast_chirho()),
                    ),
                    TypeChirho::WildcardChirho {
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ),
                promoted_ty_list_ast_chirho(),
            ),
            expr_net_ty_list_ast_chirho,
        );
        let ty_app_chirho = |fun_chirho: TyChirho, arg_chirho: TyChirho| {
            TyChirho::AppChirho(Box::new(fun_chirho), Box::new(arg_chirho))
        };
        let promoted_ty_list_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho(":".to_string())),
                Box::new(TyChirho::ConChirho("Ty".to_string())),
            )),
            Box::new(TyChirho::ConChirho("'[]".to_string())),
        );
        let expr_net_ty_list_ty_chirho = ty_app_chirho(
            ty_app_chirho(
                TyChirho::ConChirho("Expr".to_string()),
                TyChirho::ConChirho("Net".to_string()),
            ),
            promoted_ty_list_ty_chirho.clone(),
        );
        let expected_ty_chirho = ty_app_chirho(
            ty_app_chirho(
                ty_app_chirho(
                    ty_app_chirho(
                        TyChirho::ConChirho("KT".to_string()),
                        TyChirho::ConChirho("A".to_string()),
                    ),
                    promoted_ty_list_ty_chirho,
                ),
                TyChirho::ConChirho("IO".to_string()),
            ),
            expr_net_ty_list_ty_chirho,
        );

        let (subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_expr_against_expected_chirho(
            &test_gr_expr_chirho,
            &expected_ty_chirho,
            SpanChirho::DUMMY_CHIRHO,
        );
        assert_eq!(
            subst_chirho.apply_ty_chirho(&inferred_ty_chirho),
            expected_ty_chirho
        );
    }

    #[test]
    fn infer_if_expression_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // if True then 1 else 2
        let expr_chirho = ExprChirho::IfChirho {
            cond_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("True"))),
            then_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                1,
                SpanChirho::DUMMY_CHIRHO,
                None,
            ))),
            else_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                2,
                SpanChirho::DUMMY_CHIRHO,
                None,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert!(matches!(final_ty_chirho, TyChirho::VarChirho(_)));
        assert_eq!(ctx_chirho.deferred_preds_chirho.len(), 2);
        assert!(
            ctx_chirho
                .deferred_preds_chirho
                .iter()
                .all(|(pred_chirho, _)| pred_chirho.class_name_chirho == "Num")
        );
    }

    #[test]
    fn infer_type_mismatch_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // if 42 then 1 else 2  (condition is Int, not Bool)
        let expr_chirho = ExprChirho::IfChirho {
            cond_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                42,
                SpanChirho::DUMMY_CHIRHO,
                None,
            ))),
            then_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                1,
                SpanChirho::DUMMY_CHIRHO,
                None,
            ))),
            else_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                2,
                SpanChirho::DUMMY_CHIRHO,
                None,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let _ = ctx_chirho.infer_expr_chirho(&expr_chirho);
        ctx_chirho.check_deferred_preds_chirho(&SubstChirho::empty_chirho());
        let result_chirho = ctx_chirho.finish_chirho();
        assert!(
            result_chirho.diagnostics_chirho.has_errors_chirho(),
            "should report type mismatch for a numeric condition"
        );
    }

    #[test]
    fn defaulted_integer_with_integral_and_bounded_skips_bounded_error_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let constrained_var_chirho = match ctx_chirho.fresh_var_chirho() {
            TyChirho::VarChirho(var_chirho) => var_chirho,
            other_chirho => panic!("expected fresh type variable, got {:?}", other_chirho),
        };
        ctx_chirho.deferred_preds_chirho.push((
            PredChirho::new_chirho("Integral", TyChirho::VarChirho(constrained_var_chirho)),
            SpanChirho::DUMMY_CHIRHO,
        ));
        ctx_chirho.deferred_preds_chirho.push((
            PredChirho::new_chirho("Bounded", TyChirho::VarChirho(constrained_var_chirho)),
            SpanChirho::DUMMY_CHIRHO,
        ));

        let default_subst_chirho =
            ctx_chirho.check_deferred_preds_chirho(&SubstChirho::empty_chirho());
        assert!(
            matches!(
                default_subst_chirho.lookup_chirho(&constrained_var_chirho),
                Some(TyChirho::ConChirho(name_chirho)) if name_chirho == "Integer"
            ),
            "Integral+Bounded ambiguity should still default to Integer"
        );
        assert!(
            !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
            "defaulted Integral+Bounded Integer should not report a missing Bounded Integer instance: {:?}",
            ctx_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn infer_module_data_and_function_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("Color"),
                    type_vars_chirho: vec![],
                    constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho("Red"),
                        fields_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("f"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("x"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "module inference should succeed without errors"
        );

        // f should have a function type (with per-equation fresh types for
        // GADT support, the overall result stays as a fresh variable, so the
        // generalized type is `forall a b. a -> b` rather than `forall a. a -> a`).
        let f_scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("f")
            .expect("f should be in environment");
        let f_ty_chirho = result_chirho
            .subst_chirho
            .apply_ty_chirho(&f_scheme_chirho.ty_chirho);
        assert!(
            matches!(&f_ty_chirho, TyChirho::FunChirho(..)),
            "f should have function type, got {f_ty_chirho}"
        );

        // Red should be in environment as Color
        let red_scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("Red")
            .expect("Red constructor should be in environment");
        assert_eq!(
            red_scheme_chirho.ty_chirho,
            TyChirho::ConChirho("Color".to_string())
        );
    }

    #[test]
    fn infer_let_generalization_chirho() {
        // Wrap in a module-level function to get full sig checking:
        // testLetGen = let myId = \x -> x in myId 42
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("LetGen"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("testLetGenChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LetChirho {
                        binds_chirho: vec![
                            haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                                name_chirho: dummy_name_chirho("myId"),
                                matches_chirho: vec![MatchArmChirho {
                                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                                        dummy_name_chirho("x"),
                                    )),
                                    where_binds_chirho: vec![],
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                        ],
                        body_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("myId"))),
                            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                                42,
                                SpanChirho::DUMMY_CHIRHO,
                                None,
                            ))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "let generalization should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
        let _scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("testLetGenChirho")
            .expect("testLetGenChirho should be in env");
        // With per-equation fresh result types (for GADT support), the overall
        // result type stays as a fresh variable. The key assertion is that
        // let-generalization (let id = \x -> x in id 42) doesn't produce errors.
    }

    #[test]
    fn infer_tuple_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // (1, True)
        let expr_chirho = ExprChirho::TupleChirho {
            elements_chirho: vec![
                ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None)),
                ExprChirho::ConChirho(dummy_name_chirho("True")),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert!(matches!(
            final_ty_chirho,
            TyChirho::TupleChirho(ref elems_chirho)
                if elems_chirho.len() == 2
                    && matches!(elems_chirho[0], TyChirho::VarChirho(_))
                    && elems_chirho[1] == TyChirho::bool_chirho()
        ));
        assert_eq!(ctx_chirho.deferred_preds_chirho.len(), 1);
        assert_eq!(
            ctx_chirho.deferred_preds_chirho[0].0.class_name_chirho,
            "Num"
        );
    }

    #[test]
    fn infer_list_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // [1, 2, 3]
        let expr_chirho = ExprChirho::ListChirho {
            elements_chirho: vec![
                ExprChirho::LitChirho(LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None)),
                ExprChirho::LitChirho(LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO, None)),
                ExprChirho::LitChirho(LitChirho::IntChirho(3, SpanChirho::DUMMY_CHIRHO, None)),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        assert!(matches!(
            final_ty_chirho,
            TyChirho::ListChirho(ref elem_chirho)
                if matches!(elem_chirho.as_ref(), TyChirho::VarChirho(_))
        ));
        assert_eq!(ctx_chirho.deferred_preds_chirho.len(), 3);
        assert!(
            ctx_chirho
                .deferred_preds_chirho
                .iter()
                .all(|(pred_chirho, _)| pred_chirho.class_name_chirho == "Num")
        );
    }

    // -----------------------------------------------------------------------
    // Typeclass constraint tests
    // -----------------------------------------------------------------------

    #[test]
    fn overloaded_op_defers_predicate_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expr_chirho = ExprChirho::LamChirho {
            pats_chirho: vec![
                PatChirho::VarChirho(dummy_name_chirho("x")),
                PatChirho::VarChirho(dummy_name_chirho("y")),
            ],
            body_chirho: Box::new(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("+"))),
                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("y"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let (_subst_chirho, _ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        assert!(!ctx_chirho.deferred_preds_chirho.is_empty());
        assert_eq!(
            ctx_chirho.deferred_preds_chirho[0].0.class_name_chirho,
            "Num"
        );
    }

    #[test]
    fn plus_on_ints_satisfies_num_chirho() {
        // f x y = x + y  applied to Int arguments should succeed
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("PlusInts"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("addChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![
                        PatChirho::VarChirho(dummy_name_chirho("x")),
                        PatChirho::VarChirho(dummy_name_chirho("y")),
                    ],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("+"))),
                            arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("y"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "addChirho should infer without errors (Num constraint stays polymorphic): {:?}",
            result_chirho.diagnostics_chirho
        );

        // addChirho should have a Num constraint in its scheme
        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("addChirho")
            .expect("addChirho should be in environment");
        assert!(
            !scheme_chirho.preds_chirho.is_empty(),
            "addChirho should have Num predicate, got: {scheme_chirho}"
        );
        assert_eq!(scheme_chirho.preds_chirho[0].class_name_chirho, "Num");
    }

    #[test]
    fn infer_view_pattern_expression_keeps_num_predicate_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("ViewPattern"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("doubleChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::AppChirho {
                                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("+"))),
                                arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("fChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::ViewChirho {
                            expr_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                "doubleChirho",
                            ))),
                            pat_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("n"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("n"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec!["ViewPatterns".to_string()],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "view pattern should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );
        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("fChirho")
            .expect("fChirho should be in environment");
        assert!(
            scheme_chirho
                .preds_chirho
                .iter()
                .any(|pred_chirho| pred_chirho.class_name_chirho == "Num"),
            "view pattern expression should keep Num predicate, got: {scheme_chirho}"
        );
    }

    #[test]
    fn eq_on_ints_satisfies_constraint_chirho() {
        // eqChirho x y = x == y
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("EqInts"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("eqChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![
                        PatChirho::VarChirho(dummy_name_chirho("x")),
                        PatChirho::VarChirho(dummy_name_chirho("y")),
                    ],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("=="))),
                            arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("y"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "eqChirho should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );

        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("eqChirho")
            .expect("eqChirho should be in environment");
        assert!(
            !scheme_chirho.preds_chirho.is_empty(),
            "eqChirho should have Eq predicate, got: {scheme_chirho}"
        );
        assert_eq!(scheme_chirho.preds_chirho[0].class_name_chirho, "Eq");
    }

    #[test]
    fn class_env_is_seeded_chirho() {
        let result_chirho = infer_module_chirho(&ModuleChirho {
            name_chirho: dummy_name_chirho("Empty"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        });

        assert!(result_chirho.class_env_chirho.has_class_chirho("Eq"));
        assert!(result_chirho.class_env_chirho.has_class_chirho("Ord"));
        assert!(result_chirho.class_env_chirho.has_class_chirho("Show"));
        assert!(result_chirho.class_env_chirho.has_class_chirho("Num"));
        assert!(result_chirho.class_env_chirho.has_class_chirho("Functor"));
    }

    /// Test: case expression with infix cons pattern `(x:xs)` binds both vars.
    /// Haskell: `headChirho xs = case xs of { (y:ys) -> y }`
    #[test]
    fn infer_case_infix_con_pattern_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("InfixConPat"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("headChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("xs"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("xs"))),
                        alts_chirho: vec![AltChirho {
                            pat_chirho: PatChirho::InfixConChirho {
                                left_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("y"))),
                                op_chirho: dummy_name_chirho(":"),
                                right_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho(
                                    "ys",
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                                dummy_name_chirho("y"),
                            )),
                            where_binds_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "headChirho with infix con pattern should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );
        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("headChirho")
            .expect("headChirho should be in environment");
        // headChirho :: a -> b  (since we don't yet unify cons pattern args with list)
        // The key check: no errors — `y` was bound by bind_pat_chirho
        assert!(!format!("{scheme_chirho}").is_empty());
    }

    #[test]
    fn infer_custom_infix_constructor_pattern_uses_constructor_scheme_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("StrictPairPattern"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("StrictPair"),
                    type_vars_chirho: vec![
                        dummy_name_chirho("a").into(),
                        dummy_name_chirho("b").into(),
                    ],
                    constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho(":*:"),
                        fields_chirho: vec![
                            (
                                haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                                TypeChirho::VarChirho(dummy_name_chirho("a")),
                            ),
                            (
                                haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                                TypeChirho::VarChirho(dummy_name_chirho("b")),
                            ),
                        ],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("firstBoolChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("pairChirho"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                "pairChirho",
                            ))),
                            alts_chirho: vec![AltChirho {
                                pat_chirho: PatChirho::InfixConChirho {
                                    left_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho(
                                        "flagChirho",
                                    ))),
                                    op_chirho: dummy_name_chirho(":*:"),
                                    right_chirho: Box::new(PatChirho::WildcardChirho(
                                        SpanChirho::DUMMY_CHIRHO,
                                    )),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                },
                                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::IfChirho {
                                    cond_chirho: Box::new(ExprChirho::VarChirho(
                                        dummy_name_chirho("flagChirho"),
                                    )),
                                    then_chirho: Box::new(ExprChirho::LitChirho(
                                        LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None),
                                    )),
                                    else_chirho: Box::new(ExprChirho::LitChirho(
                                        LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO, None),
                                    )),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                where_binds_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "custom infix constructor patterns should use the constructor scheme: {:?}",
            result_chirho.diagnostics_chirho
        );
    }

    /// Test: case expression with list pattern `[a, b]` binds elements.
    /// Haskell: `sumTwoChirho xs = case xs of { [a, b] -> a }`
    #[test]
    fn infer_case_list_pattern_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("ListPat"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("sumTwoChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("xs"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("xs"))),
                        alts_chirho: vec![AltChirho {
                            pat_chirho: PatChirho::ListChirho {
                                elements_chirho: vec![
                                    PatChirho::VarChirho(dummy_name_chirho("a")),
                                    PatChirho::VarChirho(dummy_name_chirho("b")),
                                ],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                                dummy_name_chirho("a"),
                            )),
                            where_binds_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "sumTwoChirho with list pattern should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );
        result_chirho
            .env_chirho
            .lookup_chirho("sumTwoChirho")
            .expect("sumTwoChirho should be in environment");
    }

    /// Test: case expression with record pattern `Foo { bar = x }` binds x.
    /// Haskell: `getBarChirho v = case v of { Foo { bar = x } -> x }`
    #[test]
    fn infer_case_record_pattern_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("RecPat"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("getBarChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("v"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("v"))),
                        alts_chirho: vec![AltChirho {
                            pat_chirho: PatChirho::RecordChirho {
                                has_wildcard_chirho: false,
                                con_chirho: dummy_name_chirho("Foo"),
                                fields_chirho: vec![PatFieldChirho {
                                    name_chirho: dummy_name_chirho("bar"),
                                    pattern_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                                dummy_name_chirho("x"),
                            )),
                            where_binds_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "getBarChirho with record pattern should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );
        result_chirho
            .env_chirho
            .lookup_chirho("getBarChirho")
            .expect("getBarChirho should be in environment");
    }

    /// Test: negated literal pattern `(-1)` doesn't introduce new bindings.
    /// Haskell: `isNegOneChirho x = case x of { (-1) -> 1; _ -> 0 }`
    #[test]
    fn infer_case_neg_pattern_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("NegPat"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("isNegOneChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                        alts_chirho: vec![
                            AltChirho {
                                pat_chirho: PatChirho::NegChirho {
                                    lit_chirho: LitChirho::IntChirho(
                                        1,
                                        SpanChirho::DUMMY_CHIRHO,
                                        None,
                                    ),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                },
                                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                                    LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None),
                                )),
                                where_binds_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            AltChirho {
                                pat_chirho: PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO),
                                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                                    LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO, None),
                                )),
                                where_binds_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                        ],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "isNegOneChirho with neg pattern should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );
        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("isNegOneChirho")
            .expect("isNegOneChirho should be in environment");
        // isNegOneChirho should have a function type. With per-equation fresh
        // result types (for GADT support), the overall result stays as a fresh
        // variable, so we verify the function shape rather than the exact
        // return type.
        let resolved_ty_chirho = result_chirho
            .subst_chirho
            .apply_ty_chirho(&scheme_chirho.ty_chirho);
        assert!(
            matches!(&resolved_ty_chirho, TyChirho::FunChirho(..)),
            "expected function type, got: {}",
            resolved_ty_chirho
        );
    }

    // -----------------------------------------------------------------------
    // Class, instance, type sig, and newtype tests
    // -----------------------------------------------------------------------

    /// Test: user-defined class declaration registers in class env.
    /// Haskell: `class MyEq a where myEq :: a -> a -> Bool`
    #[test]
    fn infer_user_class_decl_chirho() {
        use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;

        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("UserClass"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: dummy_name_chirho("MyEqChirho"),
                type_vars_chirho: vec![dummy_name_chirho("a").into()],
                methods_chirho: vec![ClassMethodChirho {
                    name_chirho: dummy_name_chirho("myEqChirho"),
                    ty_chirho: TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                "Bool",
                            ))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "User class decl should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
        // Class should be registered
        assert!(
            result_chirho
                .class_env_chirho
                .has_class_chirho("MyEqChirho"),
            "MyEqChirho should be in class env"
        );
        // Method should be in the type environment
        assert!(
            result_chirho
                .env_chirho
                .lookup_chirho("myEqChirho")
                .is_some(),
            "myEqChirho method should be in type env"
        );
    }

    /// Test: user-defined class with superclass context.
    /// Haskell: `class Eq a => Ord a where compare :: a -> a -> Ordering`
    #[test]
    fn infer_user_class_with_superclass_chirho() {
        use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;
        use haskelujah_ast_chirho::ty_chirho::ConstraintChirho as AstConstraintChirho;

        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("SuperClass"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::ClassDeclChirho {
                context_chirho: vec![AstConstraintChirho::ClassChirho {
                    class_chirho: dummy_name_chirho("Eq"),
                    args_chirho: vec![TypeChirho::VarChirho(dummy_name_chirho("a"))],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                name_chirho: dummy_name_chirho("MyOrdChirho"),
                type_vars_chirho: vec![dummy_name_chirho("a").into()],
                methods_chirho: vec![ClassMethodChirho {
                    name_chirho: dummy_name_chirho("myCompareChirho"),
                    ty_chirho: TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                "Ordering",
                            ))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Class with superclass should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
        assert!(
            result_chirho
                .class_env_chirho
                .has_class_chirho("MyOrdChirho")
        );
        let supers_chirho = result_chirho
            .class_env_chirho
            .superclasses_chirho("MyOrdChirho");
        assert_eq!(supers_chirho, vec!["Eq".to_string()]);
    }

    #[test]
    fn infer_user_class_method_scheme_keeps_outer_forall_and_free_vars_chirho() {
        use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;

        let pa_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("p"))),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let pab_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(pa_chirho),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let tp_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("t"))),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("p"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let tpa_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(tp_chirho),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let tpab_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(tpa_chirho),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("HigherRankClassScheme"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: dummy_name_chirho("BifunctorMonadChirho"),
                type_vars_chirho: vec![dummy_name_chirho("t").into()],
                methods_chirho: vec![ClassMethodChirho {
                    name_chirho: dummy_name_chirho("bireturnChirho"),
                    ty_chirho: TypeChirho::ForallChirho {
                        vars_chirho: vec![
                            dummy_name_chirho("a").into(),
                            dummy_name_chirho("b").into(),
                        ],
                        body_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(pab_chirho),
                            mult_chirho: None,
                            result_chirho: Box::new(tpab_chirho),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "higher-rank class method scheme should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );

        let class_decl_chirho = result_chirho
            .class_env_chirho
            .classes_chirho
            .get("BifunctorMonadChirho")
            .expect("class should be registered");
        let scheme_chirho = class_decl_chirho
            .methods_chirho
            .get("bireturnChirho")
            .expect("method scheme should be registered");

        assert_eq!(
            scheme_chirho.vars_chirho.len(),
            4,
            "scheme should quantify a, b, p, and t: {scheme_chirho}"
        );
        assert!(
            scheme_chirho
                .preds_chirho
                .iter()
                .any(|pred_chirho| pred_chirho.class_name_chirho == "BifunctorMonadChirho"),
            "scheme should retain the enclosing class predicate: {scheme_chirho}"
        );
    }

    #[test]
    fn infer_user_class_method_scheme_through_type_alias_keeps_free_vars_chirho() {
        use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;

        let p_to_t_p_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(":->"))),
                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("p"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("t"))),
                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("p"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("HigherRankAliasClassScheme"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::TypeAliasDeclChirho {
                    name_chirho: dummy_name_chirho(":->"),
                    type_vars_chirho: vec![
                        dummy_name_chirho("p").into(),
                        dummy_name_chirho("q").into(),
                    ],
                    rhs_chirho: TypeChirho::ForallChirho {
                        vars_chirho: vec![
                            dummy_name_chirho("a").into(),
                            dummy_name_chirho("b").into(),
                        ],
                        body_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::AppChirho {
                                    fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "p",
                                    ))),
                                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "a",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::AppChirho {
                                    fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "q",
                                    ))),
                                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "a",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::ClassDeclChirho {
                    context_chirho: vec![],
                    name_chirho: dummy_name_chirho("BifunctorMonadChirho"),
                    type_vars_chirho: vec![dummy_name_chirho("t").into()],
                    methods_chirho: vec![ClassMethodChirho {
                        name_chirho: dummy_name_chirho("bireturnChirho"),
                        ty_chirho: p_to_t_p_chirho,
                        default_chirho: None,
                        default_sig_chirho: None,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    associated_tfs_chirho: vec![],
                    fundeps_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "alias-based class method scheme should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );

        let class_decl_chirho = result_chirho
            .class_env_chirho
            .classes_chirho
            .get("BifunctorMonadChirho")
            .expect("class should be registered");
        let scheme_chirho = class_decl_chirho
            .methods_chirho
            .get("bireturnChirho")
            .expect("method scheme should be registered");

        assert_eq!(
            scheme_chirho.vars_chirho.len(),
            4,
            "alias-based scheme should quantify a, b, p, and t: {scheme_chirho}"
        );
    }

    #[test]
    fn ast_scheme_expands_constraint_type_alias_predicates_chirho() {
        use haskelujah_ast_chirho::ty_chirho::ConstraintChirho as AstConstraintChirho;

        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.register_type_synonym_chirho(
            "ShowEq".to_string(),
            vec!["a".to_string()],
            TyChirho::TupleChirho(vec![
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Show".to_string())),
                    Box::new(TyChirho::ForallVarChirho("a".to_string())),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Eq".to_string())),
                    Box::new(TyChirho::ForallVarChirho("a".to_string())),
                ),
            ]),
        );

        let a_ty_chirho = TypeChirho::VarChirho(dummy_name_chirho("a"));
        let sig_chirho = TypeChirho::QualChirho {
            context_chirho: vec![AstConstraintChirho::ClassChirho {
                class_chirho: dummy_name_chirho("ShowEq"),
                args_chirho: vec![a_ty_chirho.clone()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            body_chirho: Box::new(TypeChirho::FunChirho {
                arg_chirho: Box::new(a_ty_chirho),
                mult_chirho: None,
                result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("String"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let scheme_chirho = ctx_chirho.ast_type_to_scheme_chirho(&sig_chirho);
        let pred_classes_chirho: Vec<String> = scheme_chirho
            .preds_chirho
            .iter()
            .map(|pred_chirho| pred_chirho.class_name_chirho.clone())
            .collect();

        assert_eq!(
            pred_classes_chirho,
            vec!["Show".to_string(), "Eq".to_string()],
            "constraint aliases should expand to real class predicates: {scheme_chirho}"
        );
    }

    #[test]
    fn infer_user_class_method_scheme_keeps_applied_class_parameter_chirho() {
        use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;

        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("BiapplicativeScheme"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: dummy_name_chirho("BiapplicativeChirho"),
                type_vars_chirho: vec![dummy_name_chirho("p").into()],
                methods_chirho: vec![ClassMethodChirho {
                    name_chirho: dummy_name_chirho("bipureChirho"),
                    ty_chirho: TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::AppChirho {
                                    fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "p",
                                    ))),
                                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "a",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "biapplicative class scheme should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );

        let class_decl_chirho = result_chirho
            .class_env_chirho
            .classes_chirho
            .get("BiapplicativeChirho")
            .expect("class should be registered");
        let scheme_chirho = class_decl_chirho
            .methods_chirho
            .get("bipureChirho")
            .expect("method scheme should be registered");

        let result_slot_chirho = match &scheme_chirho.ty_chirho {
            TyChirho::FunChirho(_, result_chirho, _) => match result_chirho.as_ref() {
                TyChirho::FunChirho(_, result2_chirho, _) => result2_chirho.as_ref().clone(),
                other_chirho => panic!("expected second function arrow, got: {other_chirho}"),
            },
            other_chirho => panic!("expected function type, got: {other_chirho}"),
        };

        assert!(
            matches!(result_slot_chirho, TyChirho::AppChirho(_, _)),
            "method result should keep p a b application rather than collapsing to bare p: {scheme_chirho}"
        );
    }

    #[test]
    fn instantiate_instance_method_expected_ty_keeps_tuple_application_chirho() {
        use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;

        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("BiapplicativeExpected"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: dummy_name_chirho("BiapplicativeChirho"),
                type_vars_chirho: vec![dummy_name_chirho("p").into()],
                methods_chirho: vec![ClassMethodChirho {
                    name_chirho: dummy_name_chirho("bipureChirho"),
                    ty_chirho: TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::AppChirho {
                                    fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "p",
                                    ))),
                                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "a",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let _ = ctx_chirho.infer_module_chirho(&module_chirho);
        let class_decl_chirho = ctx_chirho
            .class_env_chirho
            .classes_chirho
            .get("BiapplicativeChirho")
            .cloned()
            .expect("class should be registered");
        let method_scheme_chirho = class_decl_chirho
            .methods_chirho
            .get("bipureChirho")
            .expect("method scheme should be registered");
        let expected_ty_chirho = ctx_chirho.instantiate_instance_method_expected_ty_chirho(
            &class_decl_chirho,
            method_scheme_chirho,
            &[TyChirho::ConChirho("(,)".to_string())],
        );

        let result_slot_chirho = match expected_ty_chirho {
            TyChirho::FunChirho(_, result_chirho, _) => match *result_chirho {
                TyChirho::FunChirho(_, result2_chirho, _) => *result2_chirho,
                other_chirho => panic!("expected second function arrow, got: {other_chirho}"),
            },
            other_chirho => panic!("expected function type, got: {other_chirho}"),
        };

        assert!(
            matches!(result_slot_chirho, TyChirho::AppChirho(_, _)),
            "instantiated method result should remain tuple constructor application"
        );
    }

    #[test]
    fn infer_zero_arity_match_against_tuple_constructor_expected_ty_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let a_var_chirho = ctx_chirho.fresh_var_chirho();
        let b_var_chirho = ctx_chirho.fresh_var_chirho();
        let expected_ty_chirho = TyChirho::fun_n_chirho(
            vec![a_var_chirho.clone(), b_var_chirho.clone()],
            TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("(,)".to_string())),
                    Box::new(a_var_chirho),
                )),
                Box::new(b_var_chirho),
            ),
        );
        let match_arm_chirho = MatchArmChirho {
            pats_chirho: vec![],
            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LamChirho {
                pats_chirho: vec![
                    PatChirho::VarChirho(dummy_name_chirho("xChirho")),
                    PatChirho::VarChirho(dummy_name_chirho("yChirho")),
                ],
                body_chirho: Box::new(ExprChirho::TupleChirho {
                    elements_chirho: vec![
                        ExprChirho::VarChirho(dummy_name_chirho("xChirho")),
                        ExprChirho::VarChirho(dummy_name_chirho("yChirho")),
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (_subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_matches_against_expected_chirho(
            &[match_arm_chirho],
            SpanChirho::DUMMY_CHIRHO,
            &expected_ty_chirho,
        );

        assert!(
            !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
            "zero-arity tuple-constructor matches should infer against the expected function type: {:?}",
            ctx_chirho.diagnostics_chirho
        );
        assert!(
            matches!(inferred_ty_chirho, TyChirho::FunChirho(_, _, _)),
            "expected function result, got: {inferred_ty_chirho}"
        );
    }

    #[test]
    fn infer_guarded_match_with_where_keeps_lhs_params_in_scope_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let match_arm_chirho = MatchArmChirho {
            pats_chirho: vec![
                PatChirho::VarChirho(dummy_name_chirho("aChirho")),
                PatChirho::VarChirho(dummy_name_chirho("bChirho")),
            ],
            rhs_chirho: RhsChirho::GuardedChirho(vec![GuardedExprChirho {
                quals_chirho: vec![StmtChirho::expr_stmt_chirho(ExprChirho::VarChirho(
                    dummy_name_chirho("otherwise"),
                ))],
                body_chirho: ExprChirho::InfixChirho {
                    left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("aChirho"))),
                    op_chirho: dummy_name_chirho("+"),
                    right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("rChirho"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }]),
            where_binds_chirho: vec![LocalBindChirho::PatBindChirho {
                pat_chirho: PatChirho::VarChirho(dummy_name_chirho("rChirho")),
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(dummy_name_chirho(
                    "bChirho",
                ))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let expected_ty_chirho = TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        );

        let (_subst_chirho, _inferred_ty_chirho) = ctx_chirho
            .infer_matches_against_expected_chirho(
                &[match_arm_chirho],
                SpanChirho::DUMMY_CHIRHO,
                &expected_ty_chirho,
            );

        assert!(
            !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
            "guarded matches with trailing where bindings should keep lhs params in scope: {:?}",
            ctx_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn infer_guard_expression_records_method_occurrence_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let match_arm_chirho = MatchArmChirho {
            pats_chirho: vec![
                PatChirho::VarChirho(dummy_name_chirho("aChirho")),
                PatChirho::VarChirho(dummy_name_chirho("bChirho")),
            ],
            rhs_chirho: RhsChirho::GuardedChirho(vec![GuardedExprChirho {
                quals_chirho: vec![StmtChirho::expr_stmt_chirho(ExprChirho::InfixChirho {
                    left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("aChirho"))),
                    op_chirho: dummy_name_chirho("=="),
                    right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("bChirho"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                })],
                body_chirho: ExprChirho::VarChirho(dummy_name_chirho("aChirho")),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }]),
            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let expected_ty_chirho = TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        );

        let (subst_chirho, _inferred_ty_chirho) = ctx_chirho.infer_matches_against_expected_chirho(
            &[match_arm_chirho],
            SpanChirho::DUMMY_CHIRHO,
            &expected_ty_chirho,
        );
        let records_chirho = ctx_chirho.finalize_occurrence_records_chirho(&subst_chirho);
        assert!(
            records_chirho.iter().any(|record_chirho| {
                record_chirho.name_chirho == "=="
                    && record_chirho.class_name_chirho == "Eq"
                    && record_chirho.ty_key_chirho == "Int"
            }),
            "guard method occurrence should be recorded, got: {records_chirho:?}"
        );
    }

    /// Test: user-defined instance declaration registers in class env.
    /// Haskell: `instance Eq Int where ...`
    #[test]
    fn infer_user_instance_decl_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("UserInst"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::InstanceDeclChirho {
                context_chirho: vec![],
                class_chirho: dummy_name_chirho("Eq"),
                types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("MyType"))],
                methods_chirho: vec![],
                assoc_tf_instances_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Instance decl should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
        // The instance should now be resolvable
        let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::ConChirho("MyType".to_string()));
        assert!(
            result_chirho.class_env_chirho.entails_chirho(&pred_chirho),
            "Eq MyType should be entailed after instance registration"
        );
    }

    /// Test: type signature that matches inferred type produces no error.
    /// Haskell: `idChirho :: a -> a; idChirho x = x`
    #[test]
    fn infer_matching_type_sig_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("TypeSig"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::TypeSigChirho {
                    name_chirho: dummy_name_chirho("idChirho"),
                    ty_chirho: TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("idChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("x"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Matching type sig should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
    }

    /// Test: type signature with mismatching arity produces an error.
    /// With per-equation fresh result types (for GADT support), the overall
    /// result is a fresh variable that subsumes the declared result type.
    /// So result-type mismatches are no longer caught at this level.
    /// Instead, we test an arity mismatch: `badChirho :: Int -> Int -> Int`
    /// but `badChirho x = x` (one arg instead of two).
    /// Haskell: `badChirho :: Int -> Int -> Int; badChirho x = x`
    #[test]
    fn infer_mismatching_type_sig_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("BadSig"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::TypeSigChirho {
                    name_chirho: dummy_name_chirho("badChirho"),
                    ty_chirho: TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                "Int",
                            ))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("badChirho"),
                    // badChirho x = x  (infers a -> a, sig says Int -> Int -> Int)
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("x"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        // `badChirho x = x` infers `a -> a` but sig says `Int -> Int -> Int`.
        // The per-equation result is now properly unified with the overall
        // result, so the signature mismatch is correctly detected.
        assert!(
            result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Signature mismatch should be detected for `badChirho :: Int -> Int -> Int; badChirho x = x`"
        );
    }

    /// Test: newtype constructor gets proper typing.
    /// Haskell: `newtype Age = MkAge Int`
    #[test]
    fn infer_newtype_constructor_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("NT"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::NewtypeDeclChirho {
                    name_chirho: dummy_name_chirho("Age"),
                    type_vars_chirho: vec![],
                    constructor_chirho: ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho("MkAge"),
                        fields_chirho: vec![(
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            TypeChirho::ConChirho(dummy_name_chirho("Int")),
                        )],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                // Use the constructor: mkAgeChirho = MkAge 42
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("mkAgeChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("MkAge"))),
                            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                                42,
                                SpanChirho::DUMMY_CHIRHO,
                                None,
                            ))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Newtype + usage should not error: {:?}",
            result_chirho.diagnostics_chirho
        );

        // MkAge constructor should be in the environment
        let mk_age_chirho = result_chirho
            .env_chirho
            .lookup_chirho("MkAge")
            .expect("MkAge should be in env");
        // MkAge :: a -> Age (field gets fresh var since we don't resolve AST types yet)
        // Apply subst to resolve type variables from per-equation fresh types
        let mk_age_ty_chirho = result_chirho
            .subst_chirho
            .apply_ty_chirho(&mk_age_chirho.ty_chirho);
        if let TyChirho::FunChirho(_, ref ret_chirho, _) = mk_age_ty_chirho {
            assert_eq!(**ret_chirho, TyChirho::ConChirho("Age".to_string()));
        } else {
            panic!("MkAge should be a function type, got: {}", mk_age_ty_chirho);
        }

        // mkAgeChirho should also be in the environment. With per-equation
        // fresh result types (for GADT support), the overall result type stays
        // as a fresh variable, so we just verify existence and no errors.
        let _mk_age_result_chirho = result_chirho
            .env_chirho
            .lookup_chirho("mkAgeChirho")
            .expect("mkAgeChirho should be in env");
    }

    #[test]
    fn infer_tuple_pattern_function_arg_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("TuplePatternArg"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("writerLikeChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::TupleChirho {
                        elements_chirho: vec![
                            PatChirho::VarChirho(dummy_name_chirho("aChirho")),
                            PatChirho::VarChirho(dummy_name_chirho("wChirho")),
                        ],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                        dummy_name_chirho("aChirho"),
                    )),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Tuple pattern arg should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );

        let writer_like_scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("writerLikeChirho")
            .expect("writerLikeChirho should be inferred");
        let writer_like_ty_chirho = result_chirho
            .subst_chirho
            .apply_ty_chirho(&writer_like_scheme_chirho.ty_chirho);
        match writer_like_ty_chirho {
            TyChirho::FunChirho(arg_chirho, _result_chirho, _) => match *arg_chirho {
                TyChirho::TupleChirho(elems_chirho) => {
                    assert_eq!(elems_chirho.len(), 2, "tuple arg should keep arity 2");
                }
                other_chirho => panic!("expected tuple argument, got {other_chirho}"),
            },
            other_chirho => panic!("expected function type, got {other_chirho}"),
        }
    }

    #[test]
    fn infer_dollar_applies_constructor_to_lambda_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let wrap_r_chirho = TyVarChirho(9100);
        let wrap_s_chirho = TyVarChirho(9101);
        let wrap_a_chirho = TyVarChirho(9102);
        let wrap_result_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("WrapChirho".to_string())),
                    Box::new(TyChirho::VarChirho(wrap_r_chirho)),
                )),
                Box::new(TyChirho::VarChirho(wrap_s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(wrap_a_chirho)),
        );
        let wrap_arg_chirho = TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(wrap_r_chirho),
                TyChirho::VarChirho(wrap_s_chirho),
            ],
            TyChirho::VarChirho(wrap_a_chirho),
        );
        ctx_chirho.env_chirho.bind_chirho(
            "WrapChirho".to_string(),
            SchemeChirho {
                vars_chirho: vec![wrap_r_chirho, wrap_s_chirho, wrap_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(wrap_arg_chirho, wrap_result_chirho.clone()),
            },
        );
        let expr_chirho = ExprChirho::InfixChirho {
            left_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("WrapChirho"))),
            op_chirho: dummy_name_chirho("$"),
            right_chirho: Box::new(ExprChirho::LamChirho {
                pats_chirho: vec![
                    PatChirho::VarChirho(dummy_name_chirho("rChirho")),
                    PatChirho::VarChirho(dummy_name_chirho("sChirho")),
                ],
                body_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("rChirho"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (subst_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&ty_chirho);
        match final_ty_chirho {
            TyChirho::AppChirho(_, _) => {}
            other_chirho => panic!("expected constructor result after $, got {other_chirho}"),
        }
    }

    #[test]
    fn infer_newtype_record_constructor_with_function_field_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("WrapModuleChirho"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::NewtypeDeclChirho {
                    name_chirho: dummy_name_chirho("WrapChirho"),
                    type_vars_chirho: vec![
                        haskelujah_ast_chirho::decl_chirho::TyVarChirho::plain_chirho(
                            dummy_name_chirho("rChirho"),
                        ),
                        haskelujah_ast_chirho::decl_chirho::TyVarChirho::plain_chirho(
                            dummy_name_chirho("sChirho"),
                        ),
                        haskelujah_ast_chirho::decl_chirho::TyVarChirho::plain_chirho(
                            dummy_name_chirho("aChirho"),
                        ),
                    ],
                    constructor_chirho: ConDeclChirho::RecordChirho {
                        name_chirho: dummy_name_chirho("WrapChirho"),
                        fields_chirho: vec![haskelujah_ast_chirho::decl_chirho::FieldDeclChirho {
                            names_chirho: vec![dummy_name_chirho("runWrapChirho")],
                            ty_chirho: TypeChirho::FunChirho {
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "rChirho",
                                ))),
                                mult_chirho: None,
                                result_chirho: Box::new(TypeChirho::FunChirho {
                                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "sChirho",
                                    ))),
                                    mult_chirho: None,
                                    result_chirho: Box::new(TypeChirho::TupleChirho {
                                        elements_chirho: vec![
                                            TypeChirho::VarChirho(dummy_name_chirho("aChirho")),
                                            TypeChirho::VarChirho(dummy_name_chirho("sChirho")),
                                        ],
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            strictness_chirho:
                                haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("mkWrapChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("fChirho"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::InfixChirho {
                            left_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                                "WrapChirho",
                            ))),
                            op_chirho: dummy_name_chirho("$"),
                            right_chirho: Box::new(ExprChirho::LamChirho {
                                pats_chirho: vec![
                                    PatChirho::VarChirho(dummy_name_chirho("rChirho")),
                                    PatChirho::VarChirho(dummy_name_chirho("sChirho")),
                                ],
                                body_chirho: Box::new(ExprChirho::AppChirho {
                                    fun_chirho: Box::new(ExprChirho::AppChirho {
                                        fun_chirho: Box::new(ExprChirho::VarChirho(
                                            dummy_name_chirho("fChirho"),
                                        )),
                                        arg_chirho: Box::new(ExprChirho::VarChirho(
                                            dummy_name_chirho("rChirho"),
                                        )),
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                        "sChirho",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Newtype record constructor with function field should infer: {:?}",
            result_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn infer_writer_newtype_composition_with_tuple_payload_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("WriterModuleChirho"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::NewtypeDeclChirho {
                    name_chirho: dummy_name_chirho("WriterT"),
                    type_vars_chirho: vec![
                        haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                            "wChirho",
                        )),
                        haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                            "mChirho",
                        )),
                        haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                            "aChirho",
                        )),
                    ],
                    constructor_chirho: ConDeclChirho::RecordChirho {
                        name_chirho: dummy_name_chirho("WriterT"),
                        fields_chirho: vec![haskelujah_ast_chirho::decl_chirho::FieldDeclChirho {
                            names_chirho: vec![dummy_name_chirho("runWriterTChirho")],
                            ty_chirho: TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "mChirho",
                                ))),
                                arg_chirho: Box::new(TypeChirho::TupleChirho {
                                    elements_chirho: vec![
                                        TypeChirho::VarChirho(dummy_name_chirho("aChirho")),
                                        TypeChirho::VarChirho(dummy_name_chirho("wChirho")),
                                    ],
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            strictness_chirho:
                                haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("writerChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::InfixChirho {
                            left_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                                "WriterT",
                            ))),
                            op_chirho: dummy_name_chirho("."),
                            right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                "return",
                            ))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "WriterT . return should preserve tuple payload: {:?}",
            result_chirho.diagnostics_chirho
        );

        let writer_scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("writerChirho")
            .expect("writerChirho should be inferred");
        let writer_ty_chirho = result_chirho
            .subst_chirho
            .apply_ty_chirho(&writer_scheme_chirho.ty_chirho);
        match writer_ty_chirho {
            TyChirho::FunChirho(arg_chirho, _result_chirho, _) => match *arg_chirho {
                TyChirho::TupleChirho(elems_chirho) => {
                    assert_eq!(elems_chirho.len(), 2, "writer arg should stay tuple-shaped");
                }
                other_chirho => panic!("expected tuple argument, got {other_chirho}"),
            },
            other_chirho => panic!("expected function type, got {other_chirho}"),
        }
    }

    #[test]
    fn infer_state_newtype_composition_with_function_tuple_payload_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("StateModuleChirho"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::NewtypeDeclChirho {
                    name_chirho: dummy_name_chirho("StateT"),
                    type_vars_chirho: vec![
                        haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                            "sChirho",
                        )),
                        haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                            "mChirho",
                        )),
                        haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                            "aChirho",
                        )),
                    ],
                    constructor_chirho: ConDeclChirho::RecordChirho {
                        name_chirho: dummy_name_chirho("StateT"),
                        fields_chirho: vec![haskelujah_ast_chirho::decl_chirho::FieldDeclChirho {
                            names_chirho: vec![dummy_name_chirho("runStateTChirho")],
                            ty_chirho: TypeChirho::FunChirho {
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "sChirho",
                                ))),
                                mult_chirho: None,
                                result_chirho: Box::new(TypeChirho::AppChirho {
                                    fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "mChirho",
                                    ))),
                                    arg_chirho: Box::new(TypeChirho::TupleChirho {
                                        elements_chirho: vec![
                                            TypeChirho::VarChirho(dummy_name_chirho("aChirho")),
                                            TypeChirho::VarChirho(dummy_name_chirho("sChirho")),
                                        ],
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            strictness_chirho:
                                haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("stateChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("fChirho"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                                "StateT",
                            ))),
                            arg_chirho: Box::new(ExprChirho::InfixChirho {
                                left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                    "return",
                                ))),
                                op_chirho: dummy_name_chirho("."),
                                right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                    "fChirho",
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "StateT (return . f) should preserve tuple payload: {:?}",
            result_chirho.diagnostics_chirho
        );

        let state_scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("stateChirho")
            .expect("stateChirho should be inferred");
        let state_ty_chirho = result_chirho
            .subst_chirho
            .apply_ty_chirho(&state_scheme_chirho.ty_chirho);
        match state_ty_chirho {
            TyChirho::FunChirho(arg_chirho, _result_chirho, _) => match *arg_chirho {
                TyChirho::FunChirho(fun_arg_chirho, fun_result_chirho, _) => {
                    assert!(
                        matches!(*fun_result_chirho, TyChirho::TupleChirho(_)),
                        "state function result should stay tuple-shaped, got {:?}",
                        fun_result_chirho
                    );
                    assert!(
                        !matches!(*fun_arg_chirho, TyChirho::TupleChirho(_)),
                        "state function input should stay the state, not collapse into tuple"
                    );
                }
                other_chirho => panic!("expected function argument, got {other_chirho}"),
            },
            other_chirho => panic!("expected function type, got {other_chirho}"),
        }
    }

    #[test]
    fn infer_gadt_equations_refine_rigid_signature_vars_per_equation_chirho() {
        use haskelujah_ast_chirho::ty_chirho::ConstraintChirho as AstConstraintChirho;
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("FreeASeqMiniChirho"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("ASeq"),
                    type_vars_chirho: vec![
                        haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                            "fChirho",
                        )),
                        haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                            "aChirho",
                        )),
                    ],
                    constructors_chirho: vec![
                        ConDeclChirho::GadtChirho {
                            name_chirho: dummy_name_chirho("ANil"),
                            ty_chirho: TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::AppChirho {
                                    fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                        "ASeq",
                                    ))),
                                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "fChirho",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                arg_chirho: Box::new(TypeChirho::TupleChirho {
                                    elements_chirho: vec![],
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        ConDeclChirho::GadtChirho {
                            name_chirho: dummy_name_chirho("ACons"),
                            ty_chirho: TypeChirho::FunChirho {
                                arg_chirho: Box::new(TypeChirho::AppChirho {
                                    fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "fChirho",
                                    ))),
                                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "aChirho",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                mult_chirho: None,
                                result_chirho: Box::new(TypeChirho::FunChirho {
                                    arg_chirho: Box::new(TypeChirho::AppChirho {
                                        fun_chirho: Box::new(TypeChirho::AppChirho {
                                            fun_chirho: Box::new(TypeChirho::ConChirho(
                                                dummy_name_chirho("ASeq"),
                                            )),
                                            arg_chirho: Box::new(TypeChirho::VarChirho(
                                                dummy_name_chirho("fChirho"),
                                            )),
                                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                                        }),
                                        arg_chirho: Box::new(TypeChirho::VarChirho(
                                            dummy_name_chirho("uChirho"),
                                        )),
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    mult_chirho: None,
                                    result_chirho: Box::new(TypeChirho::AppChirho {
                                        fun_chirho: Box::new(TypeChirho::AppChirho {
                                            fun_chirho: Box::new(TypeChirho::ConChirho(
                                                dummy_name_chirho("ASeq"),
                                            )),
                                            arg_chirho: Box::new(TypeChirho::VarChirho(
                                                dummy_name_chirho("fChirho"),
                                            )),
                                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                                        }),
                                        arg_chirho: Box::new(TypeChirho::TupleChirho {
                                            elements_chirho: vec![
                                                TypeChirho::VarChirho(dummy_name_chirho("aChirho")),
                                                TypeChirho::VarChirho(dummy_name_chirho("uChirho")),
                                            ],
                                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                                        }),
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::TypeSigChirho {
                    name_chirho: dummy_name_chirho("reduceASeqChirho"),
                    // `reduceASeq :: Applicative f => ASeq f u -> f u`, as in
                    // Control.Applicative.Free. Without the context GHC (and now
                    // this checker) reports "could not deduce Applicative f".
                    ty_chirho: TypeChirho::QualChirho {
                        context_chirho: vec![AstConstraintChirho::ClassChirho {
                            class_chirho: dummy_name_chirho("Applicative"),
                            args_chirho: vec![TypeChirho::VarChirho(dummy_name_chirho("fChirho"))],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        body_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::AppChirho {
                                    fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                        "ASeq",
                                    ))),
                                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "fChirho",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "uChirho",
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "fChirho",
                                ))),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "uChirho",
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("reduceASeqChirho"),
                    matches_chirho: vec![
                        MatchArmChirho {
                            pats_chirho: vec![PatChirho::ConChirho {
                                con_chirho: dummy_name_chirho("ANil"),
                                args_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }],
                            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                    "pure",
                                ))),
                                arg_chirho: Box::new(ExprChirho::TupleChirho {
                                    elements_chirho: vec![],
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            where_binds_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        MatchArmChirho {
                            pats_chirho: vec![PatChirho::ConChirho {
                                con_chirho: dummy_name_chirho("ACons"),
                                args_chirho: vec![
                                    PatChirho::VarChirho(dummy_name_chirho("xChirho")),
                                    PatChirho::VarChirho(dummy_name_chirho("xsChirho")),
                                ],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }],
                            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::InfixChirho {
                                left_chirho: Box::new(ExprChirho::InfixChirho {
                                    left_chirho: Box::new(ExprChirho::ConChirho(
                                        dummy_name_chirho("(,)"),
                                    )),
                                    op_chirho: dummy_name_chirho("<$>"),
                                    right_chirho: Box::new(ExprChirho::VarChirho(
                                        dummy_name_chirho("xChirho"),
                                    )),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                op_chirho: dummy_name_chirho("<*>"),
                                right_chirho: Box::new(ExprChirho::AppChirho {
                                    fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                        "reduceASeqChirho",
                                    ))),
                                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                        "xsChirho",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            where_binds_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "GADT equations should refine the rigid signature vars per equation: {:?}",
            result_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn infer_lambda_constructor_body_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("LambdaConstructor"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("R"),
                    type_vars_chirho: vec![],
                    constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho("V"),
                        fields_chirho: vec![(
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            TypeChirho::ConChirho(dummy_name_chirho("Int")),
                        )],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("fChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LamChirho {
                            pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("n"))],
                            body_chirho: Box::new(ExprChirho::AppChirho {
                                fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("V"))),
                                arg_chirho: Box::new(ExprChirho::AppChirho {
                                    fun_chirho: Box::new(ExprChirho::AppChirho {
                                        fun_chirho: Box::new(ExprChirho::VarChirho(
                                            dummy_name_chirho("*"),
                                        )),
                                        arg_chirho: Box::new(ExprChirho::VarChirho(
                                            dummy_name_chirho("n"),
                                        )),
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    arg_chirho: Box::new(ExprChirho::LitChirho(
                                        LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO, None),
                                    )),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "lambda with constructor body should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );

        let f_scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("fChirho")
            .expect("fChirho should be in environment");
        let f_ty_chirho = result_chirho
            .subst_chirho
            .apply_ty_chirho(&f_scheme_chirho.ty_chirho);
        assert_eq!(
            f_ty_chirho,
            TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::ConChirho("R".to_string()),),
            "lambda returning constructor application should infer Int -> R"
        );
    }

    #[test]
    fn infer_result_bind_lambda_constructor_body_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("ResultLambda"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("Result"),
                    type_vars_chirho: vec![dummy_name_chirho("a").into()],
                    constructors_chirho: vec![
                        ConDeclChirho::OrdinaryChirho {
                            name_chirho: dummy_name_chirho("Ok"),
                            fields_chirho: vec![(
                                haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                                TypeChirho::VarChirho(dummy_name_chirho("a")),
                            )],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        ConDeclChirho::OrdinaryChirho {
                            name_chirho: dummy_name_chirho("Err"),
                            fields_chirho: vec![(
                                haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                                TypeChirho::ConChirho(dummy_name_chirho("String")),
                            )],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("bindResultChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![
                            PatChirho::VarChirho(dummy_name_chirho("m")),
                            PatChirho::VarChirho(dummy_name_chirho("f")),
                        ],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                "m",
                            ))),
                            alts_chirho: vec![
                                AltChirho {
                                    pat_chirho: PatChirho::ConChirho {
                                        con_chirho: dummy_name_chirho("Ok"),
                                        args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho(
                                            "x",
                                        ))],
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    },
                                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                                        fun_chirho: Box::new(ExprChirho::VarChirho(
                                            dummy_name_chirho("f"),
                                        )),
                                        arg_chirho: Box::new(ExprChirho::VarChirho(
                                            dummy_name_chirho("x"),
                                        )),
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    where_binds_chirho: vec![],
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                },
                                AltChirho {
                                    pat_chirho: PatChirho::ConChirho {
                                        con_chirho: dummy_name_chirho("Err"),
                                        args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho(
                                            "e",
                                        ))],
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    },
                                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                                        fun_chirho: Box::new(ExprChirho::ConChirho(
                                            dummy_name_chirho("Err"),
                                        )),
                                        arg_chirho: Box::new(ExprChirho::VarChirho(
                                            dummy_name_chirho("e"),
                                        )),
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    where_binds_chirho: vec![],
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                },
                            ],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("useBindResultChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::AppChirho {
                                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                    "bindResultChirho",
                                ))),
                                arg_chirho: Box::new(ExprChirho::AppChirho {
                                    fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                                        "Ok",
                                    ))),
                                    arg_chirho: Box::new(ExprChirho::LitChirho(
                                        LitChirho::IntChirho(21, SpanChirho::DUMMY_CHIRHO, None),
                                    )),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            arg_chirho: Box::new(ExprChirho::LamChirho {
                                pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("n"))],
                                body_chirho: Box::new(ExprChirho::AppChirho {
                                    fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                                        "Ok",
                                    ))),
                                    arg_chirho: Box::new(ExprChirho::AppChirho {
                                        fun_chirho: Box::new(ExprChirho::AppChirho {
                                            fun_chirho: Box::new(ExprChirho::VarChirho(
                                                dummy_name_chirho("*"),
                                            )),
                                            arg_chirho: Box::new(ExprChirho::VarChirho(
                                                dummy_name_chirho("n"),
                                            )),
                                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                                        }),
                                        arg_chirho: Box::new(ExprChirho::LitChirho(
                                            LitChirho::IntChirho(2, SpanChirho::DUMMY_CHIRHO, None),
                                        )),
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Result lambda constructor body should infer without errors: {:?}",
            result_chirho.diagnostics_chirho
        );

        let use_bind_scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("useBindResultChirho")
            .expect("useBindResultChirho should be in environment");
        let use_bind_ty_chirho = result_chirho
            .subst_chirho
            .apply_ty_chirho(&use_bind_scheme_chirho.ty_chirho);
        assert!(
            matches!(
                use_bind_ty_chirho,
                TyChirho::AppChirho(ref fun_chirho, _)
                    if fun_chirho.as_ref() == &TyChirho::ConChirho("Result".to_string())
            ),
            "bindResult with a lambda constructor body should infer Result _"
        );
    }

    /// Test: AST type to internal type conversion roundtrip.
    #[test]
    fn ast_type_conversion_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let mut var_map_chirho = HashMap::new();

        // Convert `a -> Int`
        let ast_ty_chirho = TypeChirho::FunChirho {
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
            mult_chirho: None,
            result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let ty_chirho = ctx_chirho.ast_type_to_ty_chirho(&ast_ty_chirho, &mut var_map_chirho);

        // Should be FunChirho(VarChirho(_), ConChirho("Int"))
        if let TyChirho::FunChirho(ref arg_chirho, ref ret_chirho, _) = ty_chirho {
            assert!(matches!(**arg_chirho, TyChirho::VarChirho(_)));
            assert_eq!(**ret_chirho, TyChirho::int_chirho());
        } else {
            panic!("expected function type, got: {ty_chirho}");
        }

        // Same `a` should reuse the same variable
        let a2_chirho = ctx_chirho.ast_type_to_ty_chirho(
            &TypeChirho::VarChirho(dummy_name_chirho("a")),
            &mut var_map_chirho,
        );
        if let (TyChirho::FunChirho(arg_chirho, _, _), TyChirho::VarChirho(v2_chirho)) =
            (&ty_chirho, &a2_chirho)
        {
            if let TyChirho::VarChirho(v1_chirho) = &**arg_chirho {
                assert_eq!(*v1_chirho, *v2_chirho, "same `a` should yield same var");
            }
        }
    }

    #[test]
    fn ast_type_conversion_preserves_qualified_tycon_spine_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let mut var_map_chirho = HashMap::new();
        let qualified_state_t_chirho = NameChirho::RawChirho(RawNameChirho::qualified_chirho(
            "LazyS",
            "StateT",
            SpanChirho::DUMMY_CHIRHO,
        ));
        let ast_ty_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::AppChirho {
                    fun_chirho: Box::new(TypeChirho::ConChirho(qualified_state_t_chirho)),
                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("s"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("m"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let ty_chirho = ctx_chirho.ast_type_to_ty_chirho(&ast_ty_chirho, &mut var_map_chirho);
        assert_eq!(
            ty_chirho,
            TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("LazyS.StateT".to_string())),
                        Box::new(TyChirho::VarChirho(
                            *var_map_chirho.get("s").expect("s var missing"),
                        )),
                    )),
                    Box::new(TyChirho::VarChirho(
                        *var_map_chirho.get("m").expect("m var missing"),
                    )),
                )),
                Box::new(TyChirho::VarChirho(
                    *var_map_chirho.get("a").expect("a var missing"),
                )),
            ),
            "qualified type constructors in local signatures must preserve their full application spine",
        );
    }

    /// Test: do-notation bind statement unwraps monadic type.
    /// Haskell: `doExample = do { x <- Just 42; pure x }`
    #[test]
    fn infer_do_bind_unwraps_monad_chirho() {
        // Build: doExample = do { x <- expr_m; expr_body }
        // where expr_m :: Maybe Int  (simulated as App(Con("Maybe"), Lit(42)))
        // and expr_body :: uses x (a VarChirho)
        //
        // We seed Maybe Just Nothing in the env.
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("DoNotation"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("doExampleChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::DoChirho {
                        qualifier_chirho: None,
                        stmts_chirho: vec![
                            // x <- Just 42  (simulated: x <- app(Just, 42))
                            StmtChirho::BindChirho {
                                bind_chirho: None,
                                fail_chirho: None,
                                pat_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
                                expr_chirho: ExprChirho::AppChirho {
                                    fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                                        "Just",
                                    ))),
                                    arg_chirho: Box::new(ExprChirho::LitChirho(
                                        LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO, None),
                                    )),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                },
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            // pure x (simplified: just return x)
                            StmtChirho::expr_stmt_chirho(ExprChirho::VarChirho(dummy_name_chirho(
                                "x",
                            ))),
                        ],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "do-notation bind should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
        // doExampleChirho should be in the environment
        let scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("doExampleChirho")
            .expect("doExampleChirho should be in env");
        // The type should involve x :: Int (unwrapped from Just Int)
        // The last stmt `x` has type Int, not wrapped
        assert!(!format!("{scheme_chirho}").is_empty(), "should have a type");
    }

    /// Test: do-notation with let statement.
    /// Haskell: `doLetChirho = do { let y = 10; pure y }`
    #[test]
    fn infer_do_let_statement_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("DoLet"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("doLetChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::DoChirho {
                        qualifier_chirho: None,
                        stmts_chirho: vec![
                            // let y = 10
                            StmtChirho::LetChirho {
                                binds_chirho: vec![
                                    haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                                        pat_chirho: PatChirho::VarChirho(
                                            dummy_name_chirho("y"),
                                        ),
                                        rhs_chirho: RhsChirho::UnguardedChirho(
                                            ExprChirho::LitChirho(LitChirho::IntChirho(
                                                10,
                                                SpanChirho::DUMMY_CHIRHO,
                                                None,
                                            )),
                                        ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    },
                                ],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            // y (return it)
                            StmtChirho::expr_stmt_chirho(ExprChirho::VarChirho(
                                dummy_name_chirho("y"),
                            )),
                        ],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "do-notation with let should not error: {:?}",
            result_chirho.diagnostics_chirho
        );
        let _scheme_chirho = result_chirho
            .env_chirho
            .lookup_chirho("doLetChirho")
            .expect("doLetChirho should be in env");
        // With per-equation fresh result types (for GADT support), the overall
        // result type stays as a fresh variable. The key assertion is that
        // do-notation with let doesn't produce errors.
    }

    #[test]
    fn type_family_registration_and_reduction_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // Register a closed type family: F Int = Bool, F Char = Int
        ctx_chirho.register_type_family_chirho(
            "F".to_string(),
            vec![
                (vec![TyChirho::int_chirho()], TyChirho::bool_chirho()),
                (vec![TyChirho::char_chirho()], TyChirho::int_chirho()),
            ],
        );
        // Reduce F Int → Bool
        let result_chirho = ctx_chirho.reduce_type_family_chirho("F", &[TyChirho::int_chirho()]);
        assert_eq!(
            result_chirho,
            Some(TyChirho::bool_chirho()),
            "F Int should reduce to Bool"
        );
        // Reduce F Char → Int
        let result2_chirho = ctx_chirho.reduce_type_family_chirho("F", &[TyChirho::char_chirho()]);
        assert_eq!(
            result2_chirho,
            Some(TyChirho::int_chirho()),
            "F Char should reduce to Int"
        );
        // F String should not reduce (stuck)
        let result3_chirho =
            ctx_chirho.reduce_type_family_chirho("F", &[TyChirho::string_chirho()]);
        assert_eq!(
            result3_chirho, None,
            "F String should be stuck (no matching equation)"
        );
    }

    #[test]
    fn type_family_instance_registration_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        // Register open type family F
        ctx_chirho.register_type_family_chirho("F".to_string(), vec![]);
        // Add instance: F Int = Bool
        ctx_chirho.register_type_family_instance_chirho(
            "F".to_string(),
            vec![TyChirho::int_chirho()],
            TyChirho::bool_chirho(),
        );
        let result_chirho = ctx_chirho.reduce_type_family_chirho("F", &[TyChirho::int_chirho()]);
        assert_eq!(
            result_chirho,
            Some(TyChirho::bool_chirho()),
            "open F Int should reduce to Bool"
        );
    }

    #[test]
    fn class_assoc_type_family_default_applies_per_instance_chirho() {
        let class_decl_ast_chirho = DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: dummy_name_chirho("Representable"),
            type_vars_chirho: vec![dummy_name_chirho("f").into()],
            methods_chirho: vec![],
            associated_tfs_chirho: vec![
                haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
                    name_chirho: dummy_name_chirho("Rep"),
                    type_vars_chirho: vec![dummy_name_chirho("f")],
                    default_rhs_chirho: Some(TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("GRep"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    default_params_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        // `instance Representable Proxy` leaves `Rep` undefined, so the
        // class default applies to it — and only to it.
        let instance_decl_ast_chirho = DeclChirho::InstanceDeclChirho {
            context_chirho: vec![],
            class_chirho: dummy_name_chirho("Representable"),
            types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Proxy"))],
            methods_chirho: vec![],
            assoc_tf_instances_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.process_class_decl_chirho(&class_decl_ast_chirho);
        ctx_chirho.process_instance_decl_chirho(&instance_decl_ast_chirho);

        let reduced_chirho = ctx_chirho
            .reduce_type_family_chirho("Rep", &[TyChirho::ConChirho("Proxy".to_string())]);
        assert_eq!(
            reduced_chirho,
            Some(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("GRep".to_string())),
                Box::new(TyChirho::ConChirho("Proxy".to_string())),
            )),
            "an instance without its own equation takes the class default"
        );

        // No instance for `Maybe`: the default is not a general equation, so
        // `Rep Maybe` stays stuck (GHC agrees).
        let stuck_chirho = ctx_chirho
            .reduce_type_family_chirho("Rep", &[TyChirho::ConChirho("Maybe".to_string())]);
        assert_eq!(
            stuck_chirho, None,
            "a class default must not reduce for a type without an instance"
        );
    }

    #[test]
    fn instance_assoc_type_family_override_beats_class_default_chirho() {
        let class_decl_ast_chirho = DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: dummy_name_chirho("Representable"),
            type_vars_chirho: vec![dummy_name_chirho("f").into()],
            methods_chirho: vec![],
            associated_tfs_chirho: vec![
                haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
                    name_chirho: dummy_name_chirho("Rep"),
                    type_vars_chirho: vec![dummy_name_chirho("f")],
                    default_rhs_chirho: Some(TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("GRep"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    default_params_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let instance_decl_ast_chirho = DeclChirho::InstanceDeclChirho {
            context_chirho: vec![],
            class_chirho: dummy_name_chirho("Representable"),
            types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Proxy"))],
            methods_chirho: vec![],
            assoc_tf_instances_chirho: vec![AssocTfInstanceChirho {
                family_name_chirho: dummy_name_chirho("Rep"),
                lhs_types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Proxy"))],
                rhs_chirho: TypeChirho::TupleChirho {
                    elements_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.process_class_decl_chirho(&class_decl_ast_chirho);
        ctx_chirho.process_instance_decl_chirho(&instance_decl_ast_chirho);

        let reduced_chirho = ctx_chirho
            .reduce_type_family_chirho("Rep", &[TyChirho::ConChirho("Proxy".to_string())]);

        assert_eq!(
            reduced_chirho,
            Some(TyChirho::TupleChirho(vec![])),
            "instance associated family equation should override the class default"
        );
    }

    #[test]
    fn normalize_ty_expands_synonym_exposed_by_family_reduction_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.register_type_synonym_chirho(
            "GRep".to_string(),
            vec!["f".to_string()],
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("GRep'".to_string())),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep1".to_string())),
                    Box::new(TyChirho::ForallVarChirho("f".to_string())),
                )),
            ),
        );
        ctx_chirho.register_type_family_instance_chirho(
            "Rep".to_string(),
            vec![TyChirho::ForallVarChirho("f".to_string())],
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("GRep".to_string())),
                Box::new(TyChirho::ForallVarChirho("f".to_string())),
            ),
        );

        let normalized_chirho = ctx_chirho.normalize_ty_chirho(&TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Rep".to_string())),
            Box::new(TyChirho::ConChirho("Proxy".to_string())),
        ));

        assert_eq!(
            normalized_chirho,
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("GRep'".to_string())),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep1".to_string())),
                    Box::new(TyChirho::ConChirho("Proxy".to_string())),
                )),
            ),
            "normalization should keep expanding synonyms that appear after family reduction"
        );
    }

    #[test]
    fn normalize_ty_expands_ghc_generics_metadata_aliases_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        let normalized_d1_chirho = ctx_chirho.normalize_ty_chirho(&TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("D1".to_string())),
                    Box::new(TyChirho::ConChirho("MetaData".to_string())),
                )),
                Box::new(TyChirho::ConChirho("Proxy".to_string())),
            )),
            Box::new(TyChirho::ConChirho("Int".to_string())),
        ));
        let expected_d1_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("M1".to_string())),
                        Box::new(TyChirho::ConChirho("D".to_string())),
                    )),
                    Box::new(TyChirho::ConChirho("MetaData".to_string())),
                )),
                Box::new(TyChirho::ConChirho("Proxy".to_string())),
            )),
            Box::new(TyChirho::ConChirho("Int".to_string())),
        );

        assert_eq!(
            normalized_d1_chirho, expected_d1_chirho,
            "normalization should reduce D1 applications to the corresponding M1 metadata form"
        );
    }

    #[test]
    fn seed_builtins_include_ghc_cstring_and_foreign_names_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        for name_chirho in [
            "GHC.CString.unpackCString#",
            "GHC.Base.unpackCStringUtf8#",
            "LittleEndian",
            "BigEndian",
            "GHC.ByteOrder.targetByteOrder",
            "byteSwap32",
            "GHC.Word.byteSwap64",
            "GHC.Foreign.withCStringLen",
            "GHC.Foreign.peekCStringLen",
            "Foreign.C.String.withCStringLen",
            "Data.Text.Array.run",
            "Data.ByteString.packCStringLen",
            "Data.ByteString.useAsCStringLen",
            "assert",
        ] {
            assert!(
                ctx_chirho.env_chirho.lookup_chirho(name_chirho).is_some(),
                "seeded environment should include {name_chirho}"
            );
        }
    }

    #[test]
    fn seed_builtins_include_indexed_traversable_core_methods_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        for name_chirho in [
            "imap",
            "WithIndex.imap",
            "Data.Functor.WithIndex.imap",
            "ifoldMap",
            "WithIndex.ifoldMap",
            "Data.Foldable.WithIndex.ifoldMap",
            "ifoldr",
            "WithIndex.ifoldr",
            "Data.Foldable.WithIndex.ifoldr",
            "ifoldl",
            "WithIndex.ifoldl",
            "Data.Foldable.WithIndex.ifoldl",
            "itraverse",
            "WithIndex.itraverse",
            "Data.Traversable.WithIndex.itraverse",
        ] {
            assert!(
                ctx_chirho.env_chirho.lookup_chirho(name_chirho).is_some(),
                "seeded environment should include {name_chirho}"
            );
        }
    }

    #[test]
    fn seed_builtins_include_data_sequence_snoc_operator_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        for name_chirho in ["|>", "Data.Sequence.|>"] {
            assert!(
                ctx_chirho.env_chirho.lookup_chirho(name_chirho).is_some(),
                "seeded environment should include {name_chirho}"
            );
        }
    }

    #[test]
    fn seed_builtins_include_polymorphic_monadfix_method_chirho() {
        let schemes_chirho = builtin_value_schemes_chirho();
        let mfix_scheme_chirho = schemes_chirho
            .get("Control.Monad.Fix.mfix")
            .expect("qualified mfix builtin should exist");
        let mfix_m_chirho = TyVarChirho(1720);
        let mfix_a_chirho = TyVarChirho(1721);
        let mfix_result_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(mfix_m_chirho)),
            Box::new(TyChirho::VarChirho(mfix_a_chirho)),
        );

        assert_eq!(
            mfix_scheme_chirho,
            &SchemeChirho {
                vars_chirho: vec![mfix_m_chirho, mfix_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "MonadFix".to_string(),
                    ty_chirho: TyChirho::VarChirho(mfix_m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(mfix_a_chirho),
                        mfix_result_chirho.clone(),
                    ),
                    mfix_result_chirho,
                ),
            },
            "mfix should preserve both the MonadFix constructor and result element variables"
        );
    }

    #[test]
    fn assoc_type_family_instance_uses_lhs_free_vars_as_pattern_vars_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let product_lhs_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Product"))),
                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("g"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let assoc_inst_decl_chirho = DeclChirho::InstanceDeclChirho {
            context_chirho: vec![],
            class_chirho: dummy_name_chirho("Representable"),
            types_chirho: vec![product_lhs_chirho.clone()],
            methods_chirho: vec![],
            assoc_tf_instances_chirho: vec![AssocTfInstanceChirho {
                family_name_chirho: dummy_name_chirho("Rep"),
                lhs_types_chirho: vec![product_lhs_chirho],
                rhs_chirho: TypeChirho::TupleChirho {
                    elements_chirho: vec![
                        TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Rep"))),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Rep"))),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("g"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        ctx_chirho.process_instance_decl_chirho(&assoc_inst_decl_chirho);

        let reduced_chirho = ctx_chirho.reduce_type_family_chirho(
            "Rep",
            &[TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Product".to_string())),
                    Box::new(TyChirho::ConChirho("Proxy".to_string())),
                )),
                Box::new(TyChirho::ConChirho("U1".to_string())),
            )],
        );

        assert_eq!(
            reduced_chirho,
            Some(TyChirho::TupleChirho(vec![
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ConChirho("Proxy".to_string())),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ConChirho("U1".to_string())),
                ),
            ])),
            "associated type family instance should treat lhs free vars as pattern variables"
        );
    }

    #[test]
    fn instance_method_expected_ty_reduces_associated_family_rhs_chirho() {
        let product_head_ast_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Product"))),
                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("g"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let class_decl_ast_chirho = DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: dummy_name_chirho("Representable"),
            type_vars_chirho: vec![dummy_name_chirho("f").into()],
            methods_chirho: vec![haskelujah_ast_chirho::decl_chirho::ClassMethodChirho {
                name_chirho: dummy_name_chirho("tabulate"),
                ty_chirho: TypeChirho::FunChirho {
                    arg_chirho: Box::new(TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Rep"))),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    mult_chirho: None,
                    result_chirho: Box::new(TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![
                haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
                    name_chirho: dummy_name_chirho("Rep"),
                    type_vars_chirho: vec![dummy_name_chirho("f")],
                    default_rhs_chirho: None,
                    default_params_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let instance_decl_ast_chirho = DeclChirho::InstanceDeclChirho {
            context_chirho: vec![],
            class_chirho: dummy_name_chirho("Representable"),
            types_chirho: vec![product_head_ast_chirho.clone()],
            methods_chirho: vec![],
            assoc_tf_instances_chirho: vec![AssocTfInstanceChirho {
                family_name_chirho: dummy_name_chirho("Rep"),
                lhs_types_chirho: vec![product_head_ast_chirho],
                rhs_chirho: TypeChirho::TupleChirho {
                    elements_chirho: vec![
                        TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Rep"))),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Rep"))),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("g"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.process_class_decl_chirho(&class_decl_ast_chirho);
        ctx_chirho.process_instance_decl_chirho(&instance_decl_ast_chirho);

        let class_decl_chirho = ctx_chirho
            .class_env_chirho
            .classes_chirho
            .get("Representable")
            .cloned()
            .expect("class should register");
        let method_scheme_chirho = class_decl_chirho
            .methods_chirho
            .get("tabulate")
            .expect("method scheme should register");
        let normalized_expected_ty_chirho = ctx_chirho
            .instantiate_instance_method_expected_ty_chirho(
                &class_decl_chirho,
                method_scheme_chirho,
                &[TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Product".to_string())),
                        Box::new(TyChirho::ConChirho("Proxy".to_string())),
                    )),
                    Box::new(TyChirho::ConChirho("U1".to_string())),
                )],
            );

        match normalized_expected_ty_chirho {
            TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
                match *arg_chirho {
                    TyChirho::FunChirho(_, inner_result_chirho, _) => {
                        assert!(
                            matches!(*inner_result_chirho, TyChirho::TupleChirho(ref elems_chirho) if elems_chirho.len() == 2),
                            "tabulate argument should return a 2-tuple after associated family reduction"
                        );
                    }
                    other_chirho => panic!("expected function argument, got {:?}", other_chirho),
                }
                assert!(
                    matches!(*result_chirho, TyChirho::AppChirho(_, _)),
                    "tabulate result should remain applied Product"
                );
            }
            other_chirho => panic!("expected function type, got {:?}", other_chirho),
        }
    }

    #[test]
    fn infer_proxy_index_instance_method_reduces_rep_to_unit_chirho() {
        use haskelujah_ast_chirho::decl_chirho::{AssocTfInstanceChirho, ClassMethodChirho};
        use haskelujah_ast_chirho::expr_chirho::LocalBindChirho;

        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("RepresentableProxyMethod"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::ClassDeclChirho {
                    context_chirho: vec![],
                    name_chirho: dummy_name_chirho("Representable"),
                    type_vars_chirho: vec![dummy_name_chirho("f").into()],
                    methods_chirho: vec![ClassMethodChirho {
                        name_chirho: dummy_name_chirho("index"),
                        ty_chirho: TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::FunChirho {
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                                mult_chirho: None,
                                result_chirho: Box::new(TypeChirho::AppChirho {
                                    fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                        "Rep",
                                    ))),
                                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "f",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        default_chirho: None,
                        default_sig_chirho: None,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    associated_tfs_chirho: vec![
                        haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
                            name_chirho: dummy_name_chirho("Rep"),
                            type_vars_chirho: vec![dummy_name_chirho("f")],
                            default_rhs_chirho: None,
                            default_params_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    fundeps_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::InstanceDeclChirho {
                    context_chirho: vec![],
                    class_chirho: dummy_name_chirho("Representable"),
                    types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Proxy"))],
                    methods_chirho: vec![LocalBindChirho::FunBindChirho {
                        name_chirho: dummy_name_chirho("index"),
                        matches_chirho: vec![MatchArmChirho {
                            pats_chirho: vec![
                                PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO),
                                PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO),
                            ],
                            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::TupleChirho {
                                elements_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            where_binds_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    assoc_tf_instances_chirho: vec![AssocTfInstanceChirho {
                        family_name_chirho: dummy_name_chirho("Rep"),
                        lhs_types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Proxy"))],
                        rhs_chirho: TypeChirho::TupleChirho {
                            elements_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "Proxy index instance should typecheck once Rep Proxy reduces to (): {:?}",
            result_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn collect_free_type_vars_walks_parenthesized_assoc_family_lhs_chirho() {
        let vars_chirho = collect_free_type_vars_from_ast_chirho(&[TypeChirho::ParenChirho {
            inner_chirho: Box::new(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::AppChirho {
                    fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Product"))),
                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("g"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);
        assert_eq!(vars_chirho, vec!["f".to_string(), "g".to_string()]);
    }

    #[test]
    fn infer_composition_with_tuple_family_projection_preserves_applied_rep_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let arg_ty_chirho = ctx_chirho.fresh_var_chirho();
        let h_ty_chirho = TyChirho::fun_chirho(
            arg_ty_chirho,
            TyChirho::TupleChirho(vec![
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ForallVarChirho("f".to_string())),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ForallVarChirho("g".to_string())),
                ),
            ]),
        );
        ctx_chirho
            .env_chirho
            .bind_chirho("h".to_string(), SchemeChirho::mono_chirho(h_ty_chirho));
        let expr_chirho = ExprChirho::InfixChirho {
            left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("fst"))),
            op_chirho: dummy_name_chirho("."),
            right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("h"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&inferred_ty_chirho);

        match final_ty_chirho {
            TyChirho::FunChirho(_, result_chirho, _) => {
                assert_eq!(
                    *result_chirho,
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Rep".to_string())),
                        Box::new(TyChirho::ForallVarChirho("f".to_string()))
                    )
                );
            }
            other_chirho => panic!("expected composed function type, got {:?}", other_chirho),
        }
    }

    #[test]
    fn infer_tabulate_application_after_tuple_projection_keeps_component_functor_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let class_decl_ast_chirho = DeclChirho::ClassDeclChirho {
            context_chirho: vec![],
            name_chirho: dummy_name_chirho("Representable"),
            type_vars_chirho: vec![dummy_name_chirho("f").into()],
            methods_chirho: vec![haskelujah_ast_chirho::decl_chirho::ClassMethodChirho {
                name_chirho: dummy_name_chirho("tabulate"),
                ty_chirho: TypeChirho::FunChirho {
                    arg_chirho: Box::new(TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Rep"))),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    mult_chirho: None,
                    result_chirho: Box::new(TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![
                haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
                    name_chirho: dummy_name_chirho("Rep"),
                    type_vars_chirho: vec![dummy_name_chirho("f")],
                    default_rhs_chirho: None,
                    default_params_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        ctx_chirho.process_class_decl_chirho(&class_decl_ast_chirho);

        let h_arg_ty_chirho = ctx_chirho.fresh_var_chirho();
        ctx_chirho.env_chirho.bind_chirho(
            "h".to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                h_arg_ty_chirho.clone(),
                TyChirho::TupleChirho(vec![
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Rep".to_string())),
                        Box::new(TyChirho::ForallVarChirho("f".to_string())),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Rep".to_string())),
                        Box::new(TyChirho::ForallVarChirho("g".to_string())),
                    ),
                ]),
            )),
        );

        let projected_fun_chirho = ExprChirho::InfixChirho {
            left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("fst"))),
            op_chirho: dummy_name_chirho("."),
            right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("h"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let expr_chirho = ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("tabulate"))),
            arg_chirho: Box::new(projected_fun_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let (subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
        let final_ty_chirho = ctx_chirho
            .reduce_type_families_in_ty_chirho(&subst_chirho.apply_ty_chirho(&inferred_ty_chirho));

        assert!(
            matches!(final_ty_chirho, TyChirho::AppChirho(_, _)),
            "tabulate (fst . h) should infer an applied component functor, got {:?}",
            final_ty_chirho
        );
    }

    #[test]
    fn type_family_oversaturated_application_reduces_and_reapplies_args_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.register_type_family_instance_chirho(
            "F".to_string(),
            vec![TyChirho::int_chirho()],
            TyChirho::ConChirho("Maybe".to_string()),
        );
        let app_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("F".to_string())),
                Box::new(TyChirho::int_chirho()),
            )),
            Box::new(TyChirho::char_chirho()),
        );
        let reduced_chirho = ctx_chirho.reduce_type_families_in_ty_chirho(&app_ty_chirho);
        assert_eq!(
            reduced_chirho,
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(TyChirho::char_chirho()),
            ),
            "oversaturated family application should reduce the family head and reapply leftover args"
        );
    }

    fn type_family_app2_chirho(
        family_chirho: &str,
        left_chirho: TyChirho,
        right_chirho: TyChirho,
    ) -> TyChirho {
        TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho(family_chirho.to_string())),
                Box::new(left_chirho),
            )),
            Box::new(right_chirho),
        )
    }

    fn type_family_app1_chirho(family_chirho: &str, arg_chirho: TyChirho) -> TyChirho {
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(family_chirho.to_string())),
            Box::new(arg_chirho),
        )
    }

    fn type_lit_chirho(value_chirho: &str) -> TyChirho {
        TyChirho::ConChirho(value_chirho.to_string())
    }

    #[test]
    fn typelits_builtin_append_symbol_reduces_literals_and_identities_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        let x_chirho = TyChirho::VarChirho(TyVarChirho(7001));

        assert_eq!(
            ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app2_chirho(
                "AppendSymbol",
                type_lit_chirho("\"\""),
                x_chirho.clone(),
            )),
            x_chirho,
            "AppendSymbol \"\" x should reduce to x"
        );
        assert_eq!(
            ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app2_chirho(
                "GHC.TypeLits.AppendSymbol",
                type_lit_chirho("\"type\""),
                type_lit_chirho("\"level\""),
            )),
            type_lit_chirho("\"typelevel\""),
            "qualified AppendSymbol should reduce concrete Symbol literals"
        );
    }

    #[test]
    fn typenats_builtin_arithmetic_and_comparison_families_reduce_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        let x_chirho = TyChirho::VarChirho(TyVarChirho(7002));

        for (family_chirho, left_chirho, right_chirho, expected_chirho) in [
            (
                "+",
                type_lit_chirho("2"),
                type_lit_chirho("3"),
                type_lit_chirho("5"),
            ),
            (
                "+",
                type_lit_chirho("0"),
                x_chirho.clone(),
                x_chirho.clone(),
            ),
            (
                "*",
                type_lit_chirho("2"),
                type_lit_chirho("3"),
                type_lit_chirho("6"),
            ),
            (
                "*",
                x_chirho.clone(),
                type_lit_chirho("1"),
                x_chirho.clone(),
            ),
            (
                "^",
                type_lit_chirho("2"),
                type_lit_chirho("3"),
                type_lit_chirho("8"),
            ),
            (
                "^",
                x_chirho.clone(),
                type_lit_chirho("0"),
                type_lit_chirho("1"),
            ),
            (
                "-",
                type_lit_chirho("3"),
                type_lit_chirho("2"),
                type_lit_chirho("1"),
            ),
            (
                "-",
                x_chirho.clone(),
                type_lit_chirho("0"),
                x_chirho.clone(),
            ),
            (
                "<=?",
                type_lit_chirho("1"),
                type_lit_chirho("2"),
                TyChirho::ConChirho("True".to_string()),
            ),
            (
                "<=?",
                type_lit_chirho("2"),
                type_lit_chirho("1"),
                TyChirho::ConChirho("False".to_string()),
            ),
            (
                "<=?",
                x_chirho.clone(),
                x_chirho.clone(),
                TyChirho::ConChirho("True".to_string()),
            ),
            (
                "GHC.TypeNats.Div",
                type_lit_chirho("10"),
                type_lit_chirho("3"),
                type_lit_chirho("3"),
            ),
            (
                "Mod",
                type_lit_chirho("10"),
                type_lit_chirho("3"),
                type_lit_chirho("1"),
            ),
        ] {
            assert_eq!(
                ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app2_chirho(
                    family_chirho,
                    left_chirho,
                    right_chirho,
                )),
                expected_chirho,
                "{family_chirho} should reduce through the TypeNats builtin reducer"
            );
        }

        assert_eq!(
            ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app1_chirho(
                "Log2",
                type_lit_chirho("10"),
            )),
            type_lit_chirho("3"),
            "Log2 10 should reduce to 3"
        );
    }

    #[test]
    fn typelits_character_conversion_families_reduce_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();

        for (character_chirho, codepoint_chirho, canonical_character_chirho) in [
            ("'a'", "97", "'a'"),
            ("'\\1'", "1", "'\\x1'"),
            ("'λ'", "955", "'λ'"),
            ("'\\NUL'", "0", "'\\0'"),
        ] {
            assert_eq!(
                ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app1_chirho(
                    "GHC.TypeLits.CharToNat",
                    type_lit_chirho(character_chirho),
                )),
                type_lit_chirho(codepoint_chirho),
                "CharToNat should reduce concrete character literals"
            );
            assert_eq!(
                ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app1_chirho(
                    "NatToChar",
                    type_lit_chirho(codepoint_chirho),
                )),
                type_lit_chirho(canonical_character_chirho),
                "NatToChar should reduce valid Unicode scalar values"
            );
        }

        let variable_chirho = TyChirho::VarChirho(TyVarChirho(7003));
        let stuck_variable_chirho = type_family_app1_chirho("CharToNat", variable_chirho.clone());
        assert_eq!(
            ctx_chirho.reduce_type_families_in_ty_chirho(&stuck_variable_chirho),
            stuck_variable_chirho,
            "variable character applications should stay stuck"
        );

        let surrogate_chirho = type_family_app1_chirho("NatToChar", type_lit_chirho("55296"));
        assert_eq!(
            ctx_chirho.reduce_type_families_in_ty_chirho(&surrogate_chirho),
            surrogate_chirho,
            "surrogate codepoints should stay stuck"
        );
    }

    #[test]
    fn registered_type_family_equation_precedes_builtin_typelits_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.register_type_family_instance_chirho(
            "AppendSymbol".to_string(),
            vec![type_lit_chirho("\"foo\""), type_lit_chirho("\"bar\"")],
            type_lit_chirho("\"override\""),
        );

        assert_eq!(
            ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app2_chirho(
                "AppendSymbol",
                type_lit_chirho("\"foo\""),
                type_lit_chirho("\"bar\""),
            )),
            type_lit_chirho("\"override\""),
            "explicit equations should win over builtin AppendSymbol fallback"
        );
    }

    #[test]
    fn type_family_latest_registration_wins_for_duplicate_product_rep_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let product_pattern_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Product".to_string())),
                Box::new(TyChirho::ForallVarChirho("f".to_string())),
            )),
            Box::new(TyChirho::ForallVarChirho("g".to_string())),
        );
        let either_rhs_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Either".to_string())),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ForallVarChirho("f".to_string())),
                )),
            )),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Rep".to_string())),
                Box::new(TyChirho::ForallVarChirho("g".to_string())),
            )),
        );
        let tuple_rhs_ty_chirho = TyChirho::TupleChirho(vec![
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Rep".to_string())),
                Box::new(TyChirho::ForallVarChirho("f".to_string())),
            ),
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Rep".to_string())),
                Box::new(TyChirho::ForallVarChirho("g".to_string())),
            ),
        ]);
        ctx_chirho.register_type_family_instance_chirho(
            "Rep".to_string(),
            vec![product_pattern_ty_chirho.clone()],
            either_rhs_ty_chirho,
        );
        ctx_chirho.register_type_family_instance_chirho(
            "Rep".to_string(),
            vec![product_pattern_ty_chirho],
            tuple_rhs_ty_chirho,
        );

        let app_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Rep".to_string())),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Product".to_string())),
                    Box::new(TyChirho::ConChirho("Proxy".to_string())),
                )),
                Box::new(TyChirho::ConChirho("U1".to_string())),
            )),
        );
        let reduced_chirho = ctx_chirho.reduce_type_families_in_ty_chirho(&app_ty_chirho);

        assert_eq!(
            reduced_chirho,
            TyChirho::TupleChirho(vec![
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ConChirho("Proxy".to_string())),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ConChirho("U1".to_string())),
                ),
            ]),
            "later Rep(Product f g) registration should override older colliding equations"
        );
    }

    #[test]
    fn qualified_type_family_head_uses_bare_family_lookup_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.register_type_family_instance_chirho(
            "Rep".to_string(),
            vec![TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Product".to_string())),
                    Box::new(TyChirho::ForallVarChirho("f".to_string())),
                )),
                Box::new(TyChirho::ForallVarChirho("g".to_string())),
            )],
            TyChirho::TupleChirho(vec![
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ForallVarChirho("f".to_string())),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ForallVarChirho("g".to_string())),
                ),
            ]),
        );

        let qualified_app_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(
                "Data.Functor.Contravariant.Rep.Rep".to_string(),
            )),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Product".to_string())),
                    Box::new(TyChirho::ConChirho("Proxy".to_string())),
                )),
                Box::new(TyChirho::ConChirho("U1".to_string())),
            )),
        );
        let reduced_chirho = ctx_chirho.reduce_type_families_in_ty_chirho(&qualified_app_ty_chirho);

        assert_eq!(
            reduced_chirho,
            TyChirho::TupleChirho(vec![
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ConChirho("Proxy".to_string())),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ConChirho("U1".to_string())),
                ),
            ]),
            "qualified family heads should still match bare registered type families"
        );
    }

    #[test]
    fn qualified_type_family_prefers_bare_local_equations_over_empty_exact_import_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho
            .register_type_family_chirho("System.Random.Internal.MutableGen".to_string(), vec![]);
        ctx_chirho.register_type_family_instance_chirho(
            "MutableGen".to_string(),
            vec![
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("STGen".to_string())),
                    Box::new(TyChirho::ForallVarChirho("g".to_string())),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ST".to_string())),
                    Box::new(TyChirho::ForallVarChirho("s".to_string())),
                ),
            ],
            TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("STGenM".to_string())),
                    Box::new(TyChirho::ForallVarChirho("g".to_string())),
                )),
                Box::new(TyChirho::ForallVarChirho("s".to_string())),
            ),
        );

        let qualified_app_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho(
                    "System.Random.Internal.MutableGen".to_string(),
                )),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("STGen".to_string())),
                    Box::new(TyChirho::ConChirho("StdGen".to_string())),
                )),
            )),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::ConChirho("S0".to_string())),
            )),
        );
        let reduced_chirho = ctx_chirho.reduce_type_families_in_ty_chirho(&qualified_app_ty_chirho);

        assert_eq!(
            reduced_chirho,
            TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("STGenM".to_string())),
                    Box::new(TyChirho::ConChirho("StdGen".to_string())),
                )),
                Box::new(TyChirho::ConChirho("S0".to_string())),
            ),
            "a qualified imported family head should still use bare local equations when the exact imported declaration has no equations"
        );
    }

    #[test]
    fn function_unification_retries_after_result_fixes_type_family_args_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.register_type_family_instance_chirho(
            "MutableGen".to_string(),
            vec![
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("STGen".to_string())),
                    Box::new(TyChirho::ForallVarChirho("g".to_string())),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ST".to_string())),
                    Box::new(TyChirho::ForallVarChirho("s".to_string())),
                ),
            ],
            TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("STGenM".to_string())),
                    Box::new(TyChirho::ForallVarChirho("g".to_string())),
                )),
                Box::new(TyChirho::ForallVarChirho("s".to_string())),
            ),
        );

        let monad_ty_chirho = ctx_chirho.fresh_var_chirho();
        let expected_ty_chirho = TyChirho::FunChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("MutableGen".to_string())),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("STGen".to_string())),
                        Box::new(TyChirho::ConChirho("StdGen".to_string())),
                    )),
                )),
                Box::new(monad_ty_chirho.clone()),
            )),
            Box::new(TyChirho::AppChirho(
                Box::new(monad_ty_chirho),
                Box::new(TyChirho::ConChirho("Int".to_string())),
            )),
            MultChirho::ManyChirho,
        );
        let actual_ty_chirho = TyChirho::FunChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("STGenM".to_string())),
                    Box::new(TyChirho::ConChirho("StdGen".to_string())),
                )),
                Box::new(TyChirho::ConChirho("S0".to_string())),
            )),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ST".to_string())),
                    Box::new(TyChirho::ConChirho("S0".to_string())),
                )),
                Box::new(TyChirho::ConChirho("Int".to_string())),
            )),
            MultChirho::ManyChirho,
        );

        let subst_chirho = ctx_chirho
            .unify_normalized_chirho(
                &expected_ty_chirho,
                &actual_ty_chirho,
                SpanChirho::DUMMY_CHIRHO,
            )
            .expect("result-side monad refinement should unlock MutableGen reduction");
        let expected_normalized_chirho =
            ctx_chirho.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(&expected_ty_chirho));
        let actual_normalized_chirho =
            ctx_chirho.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(&actual_ty_chirho));

        assert_eq!(
            expected_normalized_chirho, actual_normalized_chirho,
            "function unification should retry after result-type substitutions make associated family arguments reducible"
        );
    }

    #[test]
    fn unify_normalized_reduces_primstate_before_occurs_check_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.register_type_family_instance_chirho(
            "PrimState".to_string(),
            vec![TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::ForallVarChirho("s".to_string())),
            )],
            TyChirho::ForallVarChirho("s".to_string()),
        );

        let state_var_ty_chirho = ctx_chirho.fresh_var_chirho();
        let prim_state_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("PrimState".to_string())),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(state_var_ty_chirho.clone()),
            )),
        );

        let subst_chirho = ctx_chirho
            .unify_normalized_chirho(
                &state_var_ty_chirho,
                &prim_state_ty_chirho,
                SpanChirho::DUMMY_CHIRHO,
            )
            .expect("PrimState (ST t) should reduce before the occurs check fires");

        assert!(
            subst_chirho.is_empty_chirho(),
            "reducing PrimState (ST t) should make the unification trivial"
        );
        assert_eq!(
            ctx_chirho.normalize_ty_chirho(&state_var_ty_chirho),
            ctx_chirho.normalize_ty_chirho(&prim_state_ty_chirho),
            "PrimState (ST t) should normalize to t before raw unification"
        );
    }

    #[test]
    fn app_unification_renormalizes_after_head_substitution_for_primstate_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.register_type_family_instance_chirho(
            "PrimState".to_string(),
            vec![TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::ForallVarChirho("s".to_string())),
            )],
            TyChirho::ForallVarChirho("s".to_string()),
        );

        let monad_ty_chirho = ctx_chirho.fresh_var_chirho();
        let elem_ty_chirho = ctx_chirho.fresh_var_chirho();
        let expected_ty_chirho = TyChirho::AppChirho(
            Box::new(monad_ty_chirho.clone()),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("MutableArray".to_string())),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("PrimState".to_string())),
                        Box::new(monad_ty_chirho.clone()),
                    )),
                )),
                Box::new(elem_ty_chirho.clone()),
            )),
        );
        let actual_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::ConChirho("S0".to_string())),
            )),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("MutableArray".to_string())),
                    Box::new(TyChirho::ConChirho("S0".to_string())),
                )),
                Box::new(TyChirho::ConChirho("Int".to_string())),
            )),
        );

        let subst_chirho = ctx_chirho
            .unify_normalized_chirho(
                &expected_ty_chirho,
                &actual_ty_chirho,
                SpanChirho::DUMMY_CHIRHO,
            )
            .expect("head substitutions should unlock PrimState reductions inside application arguments");

        let expected_normalized_chirho =
            ctx_chirho.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(&expected_ty_chirho));
        let actual_normalized_chirho =
            ctx_chirho.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(&actual_ty_chirho));

        assert_eq!(
            expected_normalized_chirho, actual_normalized_chirho,
            "application unification should re-normalize after substitutions expose PrimState equations"
        );
    }

    #[test]
    fn unify_normalized_instantiates_forall_before_reducing_primstate_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho.register_type_family_instance_chirho(
            "PrimState".to_string(),
            vec![TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::ForallVarChirho("s".to_string())),
            )],
            TyChirho::ForallVarChirho("s".to_string()),
        );

        let monad_ty_chirho = ctx_chirho.fresh_var_chirho();
        let elem_ty_chirho = TyChirho::ConChirho("Int".to_string());
        let actual_ty_chirho = TyChirho::AppChirho(
            Box::new(monad_ty_chirho.clone()),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("MutableArray".to_string())),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("PrimState".to_string())),
                        Box::new(monad_ty_chirho.clone()),
                    )),
                )),
                Box::new(elem_ty_chirho.clone()),
            )),
        );
        let state_var_chirho = TyVarChirho(8000);
        let expected_ty_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![state_var_chirho],
            body_chirho: Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ST".to_string())),
                    Box::new(TyChirho::VarChirho(state_var_chirho)),
                )),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("MutableArray".to_string())),
                        Box::new(TyChirho::VarChirho(state_var_chirho)),
                    )),
                    Box::new(elem_ty_chirho.clone()),
                )),
            )),
        };

        let subst_chirho = ctx_chirho
            .unify_normalized_chirho(
                &actual_ty_chirho,
                &expected_ty_chirho,
                SpanChirho::DUMMY_CHIRHO,
            )
            .expect("normalized unification should instantiate forall results before PrimState occurs checks");

        let actual_normalized_chirho =
            ctx_chirho.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(&actual_ty_chirho));
        match actual_normalized_chirho {
            TyChirho::AppChirho(fun_chirho, result_chirho) => match (*fun_chirho, *result_chirho) {
                (
                    TyChirho::AppChirho(st_head_chirho, state_arg_chirho),
                    TyChirho::AppChirho(mutable_array_head_chirho, elem_arg_chirho),
                ) => {
                    assert_eq!(
                        *st_head_chirho,
                        TyChirho::ConChirho("ST".to_string()),
                        "the monad should specialize to ST after instantiating the forall result"
                    );
                    match (*mutable_array_head_chirho, *elem_arg_chirho) {
                        (
                            TyChirho::AppChirho(mutable_array_con_chirho, mutable_state_chirho),
                            TyChirho::ConChirho(elem_name_chirho),
                        ) => {
                            assert_eq!(
                                *mutable_array_con_chirho,
                                TyChirho::ConChirho("MutableArray".to_string()),
                                "the result should stay a MutableArray after reduction"
                            );
                            assert_eq!(
                                elem_name_chirho, "Int",
                                "the element type should remain intact through the PrimState reduction"
                            );
                            assert_eq!(
                                *mutable_state_chirho, *state_arg_chirho,
                                "the MutableArray state should match the instantiated ST state variable"
                            );
                        }
                        other_chirho => panic!(
                            "expected ST result to produce a MutableArray state argument, got {:?}",
                            other_chirho
                        ),
                    }
                }
                other_chirho => panic!(
                    "expected normalized actual type to specialize to ST s (MutableArray s Int), got {:?}",
                    other_chirho
                ),
            },
            other_chirho => panic!(
                "expected normalized actual type to remain an application, got {:?}",
                other_chirho
            ),
        }
    }

    #[test]
    fn type_family_nonexistent_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        let result_chirho =
            ctx_chirho.reduce_type_family_chirho("NoSuchFamily", &[TyChirho::int_chirho()]);
        assert_eq!(
            result_chirho, None,
            "unregistered family should return None"
        );
    }

    #[test]
    fn builtin_splitmix_helpers_use_word_sized_signatures_chirho() {
        let schemes_chirho = builtin_value_schemes_chirho();

        let next_word32_scheme_chirho = schemes_chirho
            .get("nextWord32")
            .expect("nextWord32 builtin should exist");
        assert_eq!(
            next_word32_scheme_chirho.ty_chirho,
            TyChirho::fun_chirho(
                TyChirho::ConChirho("SMGen".to_string()),
                TyChirho::TupleChirho(vec![
                    TyChirho::ConChirho("Word32".to_string()),
                    TyChirho::ConChirho("SMGen".to_string()),
                ]),
            ),
            "nextWord32 should return a Word32 and the updated generator"
        );

        let bitmask_scheme_chirho = schemes_chirho
            .get("bitmaskWithRejection64'")
            .expect("bitmaskWithRejection64' builtin should exist");
        assert_eq!(
            bitmask_scheme_chirho.ty_chirho,
            TyChirho::fun_n_chirho(
                vec![
                    TyChirho::ConChirho("Word64".to_string()),
                    TyChirho::ConChirho("SMGen".to_string()),
                ],
                TyChirho::TupleChirho(vec![
                    TyChirho::ConChirho("Word64".to_string()),
                    TyChirho::ConChirho("SMGen".to_string()),
                ]),
            ),
            "bitmaskWithRejection64' should accept a Word64 upper bound"
        );
    }

    #[test]
    fn builtin_enum_from_to_is_enum_polymorphic_chirho() {
        let schemes_chirho = builtin_value_schemes_chirho();
        let enum_from_to_scheme_chirho = schemes_chirho
            .get("enumFromTo")
            .expect("enumFromTo builtin should exist");
        let ef_a_chirho = TyVarChirho(3268);

        assert_eq!(
            enum_from_to_scheme_chirho,
            &SchemeChirho {
                vars_chirho: vec![ef_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Enum".to_string(),
                    ty_chirho: TyChirho::VarChirho(ef_a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(ef_a_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(ef_a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(ef_a_chirho))),
                    ),
                ),
            },
            "enumFromTo should work over any Enum, not just Int"
        );
    }

    #[test]
    fn builtin_swap_supports_binary_constructor_polymorphism_chirho() {
        let schemes_chirho = builtin_value_schemes_chirho();
        let swap_scheme_chirho = schemes_chirho
            .get("swap")
            .expect("swap builtin should exist");
        let sw_p_chirho = TyVarChirho(1035);
        let sw_a_chirho = TyVarChirho(1036);
        let sw_b_chirho = TyVarChirho(1037);

        assert_eq!(
            swap_scheme_chirho,
            &SchemeChirho {
                vars_chirho: vec![sw_p_chirho, sw_a_chirho, sw_b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(sw_p_chirho)),
                            Box::new(TyChirho::VarChirho(sw_a_chirho)),
                        )),
                        Box::new(TyChirho::VarChirho(sw_b_chirho)),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(sw_p_chirho)),
                            Box::new(TyChirho::VarChirho(sw_b_chirho)),
                        )),
                        Box::new(TyChirho::VarChirho(sw_a_chirho)),
                    ),
                ),
            },
            "swap should work over any binary constructor, not only tuples"
        );
    }

    #[test]
    fn builtin_test_tasty_helpers_are_module_qualified_only_chirho() {
        let schemes_chirho = builtin_value_schemes_chirho();

        for bare_name_chirho in [
            "safeRead",
            "Option",
            "lookupOption",
            "defaultValue",
            "parseValue",
            "optionName",
            "optionHelp",
            "defaultMain",
            "defaultMainWithIngredients",
            "testGroup",
            "includingOptions",
        ] {
            assert!(
                !schemes_chirho.contains_key(bare_name_chirho),
                "builtin scheme seeding should not leak {bare_name_chirho} into unrelated modules"
            );
        }

        for qualified_name_chirho in [
            "Test.Tasty.Options.safeRead",
            "Test.Tasty.Options.Option",
            "Test.Tasty.Options.lookupOption",
            "Test.Tasty.Options.defaultValue",
            "Test.Tasty.Options.parseValue",
            "Test.Tasty.Options.optionName",
            "Test.Tasty.Options.optionHelp",
            "Test.Tasty.defaultMain",
            "Test.Tasty.defaultMainWithIngredients",
            "Test.Tasty.testGroup",
            "Test.Tasty.includingOptions",
        ] {
            assert!(
                schemes_chirho.contains_key(qualified_name_chirho),
                "builtin scheme seeding should keep {qualified_name_chirho} available for imported tasty modules"
            );
        }
    }

    #[test]
    fn fun_arity_treats_expanded_shows_alias_as_terminal_result_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let expanded_show_s_chirho =
            ctx_chirho.expand_type_synonyms_chirho(&TyChirho::ConChirho("ShowS".to_string()));
        let fun_ty_chirho = TyChirho::fun_n_chirho(
            vec![
                TyChirho::int_chirho(),
                TyChirho::string_chirho(),
                TyChirho::int_chirho(),
                ctx_chirho.fresh_var_chirho(),
            ],
            expanded_show_s_chirho,
        );

        assert_eq!(
            ctx_chirho.fun_arity_chirho(&fun_ty_chirho),
            4,
            "expanded ShowS should count as a terminal result when matching function equations"
        );
    }

    #[test]
    fn fun_arity_treats_expanded_reads_alias_as_terminal_result_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let read_result_var_chirho = ctx_chirho.fresh_var_chirho();
        let expanded_read_s_chirho = ctx_chirho.expand_type_synonyms_chirho(&TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("ReadS".to_string())),
            Box::new(read_result_var_chirho.clone()),
        ));
        let fun_ty_chirho = TyChirho::fun_n_chirho(
            vec![
                TyChirho::fun_chirho(TyChirho::string_chirho(), expanded_read_s_chirho.clone()),
                TyChirho::int_chirho(),
            ],
            expanded_read_s_chirho,
        );

        assert_eq!(
            ctx_chirho.fun_arity_chirho(&fun_ty_chirho),
            2,
            "expanded ReadS a should count as a terminal result when matching function equations"
        );
    }

    #[test]
    fn infer_expected_show_composition_rhs_with_seeded_params_chirho() {
        let rhs_expr_chirho = ExprChirho::InfixChirho {
            left_chirho: Box::new(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("showParen"))),
                arg_chirho: Box::new(ExprChirho::ParenChirho {
                    inner_chirho: Box::new(ExprChirho::InfixChirho {
                        left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("dChirho"))),
                        op_chirho: dummy_name_chirho(">"),
                        right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                            10,
                            SpanChirho::DUMMY_CHIRHO,
                            None,
                        ))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            op_chirho: dummy_name_chirho("$"),
            right_chirho: Box::new(ExprChirho::InfixChirho {
                left_chirho: Box::new(ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("showString"))),
                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("nameChirho"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                op_chirho: dummy_name_chirho("."),
                right_chirho: Box::new(ExprChirho::InfixChirho {
                    left_chirho: Box::new(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("showChar"))),
                        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::CharChirho(
                            ' ',
                            SpanChirho::DUMMY_CHIRHO,
                            None,
                        ))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    op_chirho: dummy_name_chirho("."),
                    right_chirho: Box::new(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                "spChirho",
                            ))),
                            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                                11,
                                SpanChirho::DUMMY_CHIRHO,
                                None,
                            ))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("xChirho"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let shared_arg_ty_chirho = ctx_chirho.fresh_var_chirho();
        let expanded_show_s_chirho =
            ctx_chirho.expand_type_synonyms_chirho(&TyChirho::ConChirho("ShowS".to_string()));
        ctx_chirho.env_chirho.bind_chirho(
            "spChirho".to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
                vec![TyChirho::int_chirho(), shared_arg_ty_chirho.clone()],
                expanded_show_s_chirho.clone(),
            )),
        );
        ctx_chirho.env_chirho.bind_chirho(
            "nameChirho".to_string(),
            SchemeChirho::mono_chirho(TyChirho::string_chirho()),
        );
        ctx_chirho.env_chirho.bind_chirho(
            "dChirho".to_string(),
            SchemeChirho::mono_chirho(TyChirho::int_chirho()),
        );
        ctx_chirho.env_chirho.bind_chirho(
            "xChirho".to_string(),
            SchemeChirho::mono_chirho(shared_arg_ty_chirho),
        );

        let (_ordinary_subst_chirho, ordinary_ty_chirho) =
            ctx_chirho.infer_expr_chirho(&rhs_expr_chirho);
        assert_eq!(
            ctx_chirho.normalize_ty_chirho(&ordinary_ty_chirho),
            ctx_chirho.normalize_ty_chirho(&expanded_show_s_chirho),
            "ordinary inference with the seeded parameter types should return ShowS"
        );

        let (expected_subst_chirho, expected_ty_chirho) = ctx_chirho
            .infer_expr_against_expected_chirho(
                &rhs_expr_chirho,
                &expanded_show_s_chirho,
                SpanChirho::DUMMY_CHIRHO,
            );
        let final_expected_ty_chirho = expected_subst_chirho.apply_ty_chirho(&expected_ty_chirho);
        assert_eq!(
            ctx_chirho.normalize_ty_chirho(&final_expected_ty_chirho),
            ctx_chirho.normalize_ty_chirho(&expanded_show_s_chirho),
            "expected-type inference with the seeded parameter types should return ShowS"
        );
        assert!(
            ctx_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .is_empty(),
            "seeded composition inference should not emit diagnostics: {:?}",
            ctx_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn infer_matches_against_expected_for_show_composition_signature_chirho() {
        let rhs_expr_chirho = ExprChirho::InfixChirho {
            left_chirho: Box::new(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("showParen"))),
                arg_chirho: Box::new(ExprChirho::ParenChirho {
                    inner_chirho: Box::new(ExprChirho::InfixChirho {
                        left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("dChirho"))),
                        op_chirho: dummy_name_chirho(">"),
                        right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                            10,
                            SpanChirho::DUMMY_CHIRHO,
                            None,
                        ))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            op_chirho: dummy_name_chirho("$"),
            right_chirho: Box::new(ExprChirho::InfixChirho {
                left_chirho: Box::new(ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("showString"))),
                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("nameChirho"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                op_chirho: dummy_name_chirho("."),
                right_chirho: Box::new(ExprChirho::InfixChirho {
                    left_chirho: Box::new(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("showChar"))),
                        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::CharChirho(
                            ' ',
                            SpanChirho::DUMMY_CHIRHO,
                            None,
                        ))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    op_chirho: dummy_name_chirho("."),
                    right_chirho: Box::new(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                "spChirho",
                            ))),
                            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                                11,
                                SpanChirho::DUMMY_CHIRHO,
                                None,
                            ))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("xChirho"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let match_arm_chirho = MatchArmChirho {
            pats_chirho: vec![
                PatChirho::VarChirho(dummy_name_chirho("spChirho")),
                PatChirho::VarChirho(dummy_name_chirho("nameChirho")),
                PatChirho::VarChirho(dummy_name_chirho("dChirho")),
                PatChirho::VarChirho(dummy_name_chirho("xChirho")),
            ],
            rhs_chirho: RhsChirho::UnguardedChirho(rhs_expr_chirho),
            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let shared_arg_ty_chirho = ctx_chirho.fresh_var_chirho();
        let expanded_show_s_chirho =
            ctx_chirho.expand_type_synonyms_chirho(&TyChirho::ConChirho("ShowS".to_string()));
        let expected_ty_chirho = TyChirho::fun_n_chirho(
            vec![
                TyChirho::fun_n_chirho(
                    vec![TyChirho::int_chirho(), shared_arg_ty_chirho.clone()],
                    expanded_show_s_chirho.clone(),
                ),
                TyChirho::string_chirho(),
                TyChirho::int_chirho(),
                shared_arg_ty_chirho,
            ],
            expanded_show_s_chirho.clone(),
        );

        let (subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_matches_against_expected_chirho(
            &[match_arm_chirho],
            SpanChirho::DUMMY_CHIRHO,
            &expected_ty_chirho,
        );
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&inferred_ty_chirho);

        assert_eq!(
            ctx_chirho.normalize_ty_chirho(&final_ty_chirho),
            ctx_chirho.normalize_ty_chirho(&expected_ty_chirho),
            "match inference should keep the ShowS-returning signature intact"
        );
        assert!(
            ctx_chirho
                .diagnostics_chirho
                .diagnostics_chirho()
                .is_empty(),
            "match inference should not emit diagnostics: {:?}",
            ctx_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn infer_module_show_composition_type_signature_chirho() {
        let rhs_expr_chirho = ExprChirho::InfixChirho {
            left_chirho: Box::new(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("showParen"))),
                arg_chirho: Box::new(ExprChirho::ParenChirho {
                    inner_chirho: Box::new(ExprChirho::InfixChirho {
                        left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("dChirho"))),
                        op_chirho: dummy_name_chirho(">"),
                        right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                            10,
                            SpanChirho::DUMMY_CHIRHO,
                            None,
                        ))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            op_chirho: dummy_name_chirho("$"),
            right_chirho: Box::new(ExprChirho::InfixChirho {
                left_chirho: Box::new(ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("showString"))),
                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("nameChirho"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                op_chirho: dummy_name_chirho("."),
                right_chirho: Box::new(ExprChirho::InfixChirho {
                    left_chirho: Box::new(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("showChar"))),
                        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::CharChirho(
                            ' ',
                            SpanChirho::DUMMY_CHIRHO,
                            None,
                        ))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    op_chirho: dummy_name_chirho("."),
                    right_chirho: Box::new(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                "spChirho",
                            ))),
                            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                                11,
                                SpanChirho::DUMMY_CHIRHO,
                                None,
                            ))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("xChirho"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("ShowComposeTypeSig"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::TypeSigChirho {
                    name_chirho: dummy_name_chirho("showsUnaryWithMiniChirho"),
                    ty_chirho: TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::ParenChirho {
                            inner_chirho: Box::new(TypeChirho::FunChirho {
                                arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                    "Int",
                                ))),
                                mult_chirho: None,
                                result_chirho: Box::new(TypeChirho::FunChirho {
                                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "aChirho",
                                    ))),
                                    mult_chirho: None,
                                    result_chirho: Box::new(TypeChirho::ConChirho(
                                        dummy_name_chirho("ShowS"),
                                    )),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                "String",
                            ))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::FunChirho {
                                arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                    "Int",
                                ))),
                                mult_chirho: None,
                                result_chirho: Box::new(TypeChirho::FunChirho {
                                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "aChirho",
                                    ))),
                                    mult_chirho: None,
                                    result_chirho: Box::new(TypeChirho::ConChirho(
                                        dummy_name_chirho("ShowS"),
                                    )),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("showsUnaryWithMiniChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![
                            PatChirho::VarChirho(dummy_name_chirho("spChirho")),
                            PatChirho::VarChirho(dummy_name_chirho("nameChirho")),
                            PatChirho::VarChirho(dummy_name_chirho("dChirho")),
                            PatChirho::VarChirho(dummy_name_chirho("xChirho")),
                        ],
                        rhs_chirho: RhsChirho::UnguardedChirho(rhs_expr_chirho),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_supply_chirho: Default::default(),
        };

        let result_chirho = infer_module_chirho(&module_chirho);
        assert!(
            !result_chirho.diagnostics_chirho.has_errors_chirho(),
            "module inference should accept the ShowS-returning composition signature: {:?}",
            result_chirho.diagnostics_chirho
        );
    }

    #[test]
    fn certainly_unsolvable_seeded_class_ground_no_instance_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        let pred_chirho =
            PredChirho::new_chirho("Eq", TyChirho::ConChirho("ZzNoSuchType".to_string()));
        assert!(
            ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
            "ground Eq on a type with no instance and no deriving must be certainly unsolvable"
        );
    }

    #[test]
    fn certainly_unsolvable_suppressed_by_matching_instance_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho
            .class_env_chirho
            .add_instance_chirho(InstDeclChirho {
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: TyChirho::ConChirho("ZzNoSuchType".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            });
        let pred_chirho =
            PredChirho::new_chirho("Eq", TyChirho::ConChirho("ZzNoSuchType".to_string()));
        assert!(
            !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
            "a head-matching instance must suppress the unsolvable verdict"
        );
    }

    #[test]
    fn certainly_unsolvable_suppressed_by_var_headed_instance_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho
            .class_env_chirho
            .add_instance_chirho(InstDeclChirho {
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: TyChirho::VarChirho(TyVarChirho(0)),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            });
        let pred_chirho =
            PredChirho::new_chirho("Eq", TyChirho::ConChirho("ZzNoSuchType".to_string()));
        assert!(
            !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
            "a variable-headed instance can match anything and must suppress the verdict"
        );
    }

    #[test]
    fn certainly_unsolvable_suppressed_by_deriving_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        ctx_chirho
            .derived_instance_heads_chirho
            .insert(("Eq".to_string(), "ZzNoSuchType".to_string()));
        let pred_chirho =
            PredChirho::new_chirho("Eq", TyChirho::ConChirho("ZzNoSuchType".to_string()));
        assert!(
            !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
            "a deriving mention is a potential instance and must suppress the verdict"
        );
    }

    #[test]
    fn certainly_unsolvable_suppressed_by_given_chirho() {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let pred_chirho =
            PredChirho::new_chirho("Eq", TyChirho::ConChirho("ZzNoSuchType".to_string()));
        ctx_chirho.given_preds_chirho.push(pred_chirho.clone());
        assert!(
            !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
            "a signature given entailing the predicate must suppress the verdict"
        );
    }

    #[test]
    fn certainly_unsolvable_skips_undeclared_class_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        let pred_chirho = PredChirho::new_chirho(
            "ZzClassFromSomeImport",
            TyChirho::ConChirho("Bool".to_string()),
        );
        assert!(
            !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
            "a class that is neither seeded nor module-declared may have unseen instances"
        );
    }

    #[test]
    fn certainly_unsolvable_skips_unboxed_primitive_head_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        let pred_chirho = PredChirho::new_chirho("Num", TyChirho::ConChirho("Int#".to_string()));
        assert!(
            !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
            "unboxed literals are still typed through the Num path, so `#` heads get no verdict"
        );
    }

    #[test]
    fn certainly_unsolvable_skips_magic_class_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        let pred_chirho =
            PredChirho::new_chirho("Coercible", TyChirho::ConChirho("Bool".to_string()));
        assert!(
            !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
            "compiler-solved classes have no declared instances by design"
        );
    }

    #[test]
    fn certainly_unsolvable_skips_nonground_pred_chirho() {
        let ctx_chirho = InferCtxChirho::new_chirho();
        let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::VarChirho(TyVarChirho(7)));
        assert!(
            !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
            "a predicate with unification variables has no final verdict"
        );
    }
}
