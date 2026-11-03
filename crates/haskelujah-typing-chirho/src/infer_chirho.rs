// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Type inference engine (Algorithm W)
//!
//! Implements Hindley-Milner type inference with let-generalization.
//! Walks the AST and produces typed bindings via unification.

use crate::class_chirho::{ClassDeclChirho, ClassEnvChirho, InstDeclChirho, PredChirho};
use crate::env_chirho::TyEnvChirho;
use crate::skolem_chirho::{
    constructor_result_refines_chirho, contains_skolem_chirho, open_skolems_chirho,
    skolem_name_chirho, split_refinement_unifier_chirho,
};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};
use crate::unify_chirho::{UnifyErrorChirho, unify_chirho};
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, MatchArmChirho};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::promotion_chirho::{
    PromotionScopeChirho, readable_promoted_keys_chirho,
};
use haskelujah_ast_chirho::provenance_chirho::OriginIdChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

mod builtins_chirho;
#[path = "infer_chirho/context_chirho.rs"]
mod context_chirho;
mod evidence_chirho;
// Declarations and modules live together in `infer_chirho/declarations_chirho/`.
#[path = "infer_chirho/declarations_chirho/class_instance_chirho.rs"]
mod class_instance_chirho;
#[path = "infer_chirho/declarations_chirho/dependency_chirho.rs"]
mod dependency_chirho;
#[path = "infer_chirho/declarations_chirho/instance_obligations_chirho.rs"]
mod instance_obligations_chirho;
#[path = "infer_chirho/declarations_chirho/instance_signatures_chirho.rs"]
mod instance_signatures_chirho;
#[path = "infer_chirho/declarations_chirho/module_chirho.rs"]
mod module_chirho;
#[cfg(test)]
#[path = "infer_chirho/declarations_chirho/signature_binder_tests_chirho.rs"]
mod signature_binder_tests_chirho;
#[path = "infer_chirho/declarations_chirho/signature_binders_chirho.rs"]
mod signature_binders_chirho;
#[path = "infer_chirho/declarations_chirho/signature_conversion_chirho.rs"]
mod signature_conversion_chirho;
// Schemes, unification and predicate solving live together in `infer_chirho/solving_chirho/`.
#[path = "infer_chirho/solving_chirho/deferred_predicates_chirho.rs"]
mod deferred_predicates_chirho;
#[path = "infer_chirho/solving_chirho/equalities_chirho.rs"]
mod equalities_chirho;
#[path = "infer_chirho/solving_chirho/fundep_generalization_chirho.rs"]
mod fundep_generalization_chirho;
#[path = "infer_chirho/solving_chirho/inferred_dictionary_owners_chirho.rs"]
mod inferred_dictionary_owners_chirho;
#[path = "infer_chirho/solving_chirho/rigid_chirho.rs"]
mod rigid_chirho;
#[path = "infer_chirho/solving_chirho/schemes_chirho.rs"]
mod schemes_chirho;
#[path = "infer_chirho/solving_chirho/signature_wanteds_chirho.rs"]
mod signature_wanteds_chirho;
#[path = "infer_chirho/solving_chirho/unification_chirho.rs"]
mod unification_chirho;
#[path = "infer_chirho/solving_chirho/validity_chirho.rs"]
mod validity_chirho;
// Expressions, matches and bindings live together in `infer_chirho/terms_chirho/`.
#[path = "infer_chirho/terms_chirho/bindings_chirho.rs"]
mod bindings_chirho;
#[path = "infer_chirho/terms_chirho/conditionals_chirho.rs"]
mod conditionals_chirho;
#[path = "infer_chirho/terms_chirho/do_operations_chirho.rs"]
mod do_operations_chirho;
#[path = "infer_chirho/terms_chirho/expected_chirho.rs"]
mod expected_chirho;
#[path = "infer_chirho/terms_chirho/expressions_chirho.rs"]
mod expressions_chirho;
#[path = "infer_chirho/terms_chirho/guards_chirho.rs"]
mod guards_chirho;
#[path = "infer_chirho/terms_chirho/matches_chirho.rs"]
mod matches_chirho;
#[path = "infer_chirho/terms_chirho/pattern_bindings_chirho.rs"]
mod pattern_bindings_chirho;
#[path = "infer_chirho/terms_chirho/ranges_chirho.rs"]
mod ranges_chirho;
#[path = "infer_chirho/terms_chirho/records_chirho.rs"]
mod records_chirho;
#[path = "infer_chirho/terms_chirho/type_applications_chirho.rs"]
mod type_applications_chirho;
// The inference tests live together in `infer_chirho/tests_chirho/`.
#[cfg(test)]
#[path = "infer_chirho/tests_chirho/builtin_expected_tests_chirho.rs"]
mod builtin_expected_tests_chirho;
#[cfg(test)]
#[path = "infer_chirho/tests_chirho/class_instance_tests_chirho.rs"]
mod class_instance_tests_chirho;
#[cfg(test)]
#[path = "infer_chirho/tests_chirho/evidence_validity_tests_chirho.rs"]
mod evidence_validity_tests_chirho;
#[cfg(test)]
#[path = "infer_chirho/tests_chirho/expression_tests_chirho.rs"]
mod expression_tests_chirho;
#[cfg(test)]
#[path = "infer_chirho/tests_chirho/inferred_dictionary_owners_tests_chirho.rs"]
mod inferred_dictionary_owners_tests_chirho;
#[cfg(test)]
#[path = "infer_chirho/tests_chirho/pattern_tests_chirho.rs"]
mod pattern_tests_chirho;
#[cfg(test)]
#[path = "infer_chirho/tests_chirho/promotion_identity_tests_chirho.rs"]
mod promotion_identity_tests_chirho;
#[cfg(test)]
#[path = "infer_chirho/tests_chirho/signature_newtype_tests_chirho.rs"]
mod signature_newtype_tests_chirho;
#[cfg(test)]
#[path = "infer_chirho/tests_chirho/test_support_chirho.rs"]
mod test_support_chirho;
#[cfg(test)]
#[path = "infer_chirho/tests_chirho/type_application_tests_chirho.rs"]
mod type_application_tests_chirho;
#[cfg(test)]
#[path = "infer_chirho/tests_chirho/type_family_tests_chirho.rs"]
mod type_family_tests_chirho;
#[cfg(test)]
#[path = "infer_chirho/tests_chirho/type_normalization_tests_chirho.rs"]
mod type_normalization_tests_chirho;
// Type-level conversion, synonyms and families live together in `infer_chirho/types_chirho/`.
#[path = "infer_chirho/types_chirho/ast_conversion_chirho.rs"]
mod ast_conversion_chirho;
#[cfg(test)]
#[path = "infer_chirho/types_chirho/ast_conversion_tests_chirho.rs"]
mod ast_conversion_tests_chirho;
#[path = "infer_chirho/types_chirho/ast_types_chirho.rs"]
mod ast_types_chirho;
#[path = "infer_chirho/types_chirho/families_chirho/closed_family_apartness_chirho.rs"]
mod closed_family_apartness_chirho;
#[cfg(test)]
#[path = "infer_chirho/types_chirho/families_chirho/closed_family_apartness_tests_chirho.rs"]
mod closed_family_apartness_tests_chirho;
#[path = "infer_chirho/types_chirho/families_chirho/family_constructor_origins_chirho.rs"]
mod family_constructor_origins_chirho;
#[path = "infer_chirho/types_chirho/families_chirho/family_definitions_chirho.rs"]
mod family_definitions_chirho;
#[path = "infer_chirho/types_chirho/families_chirho/family_reduction_chirho.rs"]
mod family_reduction_chirho;
#[cfg(test)]
#[path = "infer_chirho/types_chirho/families_chirho/family_reduction_tests_chirho.rs"]
mod family_reduction_tests_chirho;
#[path = "infer_chirho/types_chirho/families_chirho/registration_chirho/family_registration_chirho.rs"]
mod family_registration_chirho;
#[cfg(test)]
#[path = "infer_chirho/types_chirho/families_chirho/registration_chirho/family_registration_tests_chirho.rs"]
mod family_registration_tests_chirho;
#[path = "infer_chirho/types_chirho/families_chirho/family_scope_chirho.rs"]
mod family_scope_chirho;
#[path = "infer_chirho/types_chirho/families_chirho/registration_chirho/family_seeds_chirho.rs"]
mod family_seeds_chirho;
#[path = "infer_chirho/types_chirho/families_chirho/registration_chirho/family_transport_chirho.rs"]
mod family_transport_chirho;
#[path = "infer_chirho/types_chirho/families_chirho/registration_chirho/source_family_registration_chirho.rs"]
mod source_family_registration_chirho;
#[path = "infer_chirho/types_chirho/type_families_chirho.rs"]
mod type_families_chirho;
#[path = "infer_chirho/types_chirho/type_synonyms_chirho.rs"]
mod type_synonyms_chirho;

use ast_types_chirho::{
    ast_type_to_syn_rhs_chirho, collect_free_type_vars_from_ast_chirho, mk_type_rep_ty_chirho,
};
use builtins_chirho::seed_builtins_chirho;
pub use builtins_chirho::{
    QUALIFIED_ALIAS_AGREEMENT_CHIRHO, qualified_builtin_alias_scheme_chirho,
};
use dependency_chirho::{binding_groups_chirho, collect_module_shadowed_names_chirho};
pub use evidence_chirho::{
    LiteralEvidenceChirho, OWN_DICTIONARY_KEY_CHIRHO, ReferenceEvidenceChirho,
    is_own_dictionary_key_chirho, own_dictionary_key_chirho,
};
use expressions_chirho::build_infix_app_expr_chirho;
use fundep_generalization_chirho::{FundepGeneralizationChirho, GeneralizationScopeChirho};
use type_families_chirho::substitute_type_vars_chirho;
use type_synonyms_chirho::{collect_app_spine_chirho, map_ty_chirho, subst_named_var_chirho};

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

pub use family_definitions_chirho::{
    IntrinsicTypeFamilyChirho, TypeFamilyAliasResolutionChirho, TypeFamilyBranchChirho,
    TypeFamilyBranchIdChirho, TypeFamilyDefinitionChirho, TypeFamilyDefinitionErrorChirho,
    TypeFamilyIdChirho, TypeFamilyReductionChirho, TypeFamilyRegistryChirho,
    is_type_family_key_chirho,
};
pub use family_scope_chirho::{TypeFamilyScopeChirho, TypeFamilyScopeResolutionChirho};
pub use family_seeds_chirho::seed_type_family_registry_chirho;
pub use family_transport_chirho::stamp_family_references_chirho;
pub use source_family_registration_chirho::{
    SourceFamilyRegistrationContextChirho, SourceFamilyRegistrationResultChirho,
    predeclare_source_open_families_chirho, register_source_module_families_chirho,
};

pub type TypeFamilyEnvChirho = TypeFamilyRegistryChirho;

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
    /// The promoted-constructor scope inference used (row 686), so a later exporter
    /// stamps this module's synonym right sides from it instead of re-resolving them.
    pub promotion_scope_chirho: Option<Arc<PromotionScopeChirho>>,
    pub family_scope_chirho: Option<Arc<TypeFamilyScopeChirho>>,
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
    /// The complete closed goal after final substitution and normalization.
    /// Missing argument captures or an own dictionary never supply this proof.
    pub ground_pred_chirho: Option<PredChirho>,
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
    /// Own dictionary indices agreed by every top-level scheme mentioning a variable.
    /// An empty map remains a veto against a later conflicting generalization.
    inferred_dictionary_owners_chirho: HashMap<TyVarChirho, HashMap<String, usize>>,
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
    family_module_name_chirho: String,
    family_scope_chirho: Option<Arc<TypeFamilyScopeChirho>>,
    /// ScopedTypeVariables: when inside a function body whose type signature has
    /// `forall a b.`, maps those names to their TyVarChirho so that where-clause
    /// type annotations and local signatures use the same type variables.
    scoped_tyvars_chirho: HashMap<String, TyVarChirho>,
    /// Pattern annotations are checked while binding a declaration's pattern. Argument and
    /// lambda patterns retain their existing inference route.
    checking_binding_pattern_annotations_chirho: bool,
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
    /// The class names the standard seed itself produced, captured beside the fully-known set.
    /// An ordinary import never clears it: only a whole-environment replacement does, and a
    /// source declaration of a class removes that name. Base declares no bare-variable instance
    /// head for these classes; an orphan universal instance in an uncompiled package is the
    /// explicit boundary (row 686, C08).
    seeded_class_names_chirho: HashSet<String>,
    /// Whether a pattern has matched a constructor whose context or refined result the checker
    /// does not install as local evidence. Once set it stays set for the module, and every later
    /// signature-scope closure stays off, nested scopes included (row 686, C08).
    unmodeled_pattern_evidence_seen_chirho: bool,
    /// Constructors whose written GADT type carries a leading context. Their schemes drop
    /// it, so a pattern on one brings evidence the checker does not install (row 686, C08).
    context_constructors_chirho: HashSet<String>,
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
    /// The module's promoted-constructor scope from naming (row 686). `None` for a
    /// direct caller that never ran naming: promoted spellings then keep their legacy
    /// form, which is not the corrected identity.
    /// workflow: compiler-pipeline-chirho/promoted-constructor-identity-chirho
    promotion_scope_chirho: Option<Arc<PromotionScopeChirho>>,
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
    let imported_type_families_chirho = TypeFamilyEnvChirho::new_chirho();
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

/// Compatibility entry for callers that never ran naming: no promoted-constructor
/// scope, so promoted spellings keep their legacy form.
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
    infer_module_with_import_context_chirho(
        module_chirho,
        InferenceImportContextChirho {
            imported_types_chirho,
            imported_type_synonyms_chirho,
            imported_type_families_chirho,
            imported_class_env_chirho,
            imported_record_field_names_chirho,
            safe_unqualified_imported_type_names_chirho,
            preferred_qualified_type_names_chirho,
            promotion_scope_chirho: None,
            family_scope_chirho: None,
        },
    )
}

/// What a module's inference brings in from the rest of the program, borrowed for one
/// call, with the module's own promoted-constructor scope from naming.
/// workflow: compiler-pipeline-chirho/promoted-constructor-identity-chirho
pub struct InferenceImportContextChirho<'imports_chirho> {
    pub imported_types_chirho: &'imports_chirho HashMap<String, SchemeChirho>,
    pub imported_type_synonyms_chirho: &'imports_chirho HashMap<String, (Vec<String>, TypeChirho)>,
    pub imported_type_families_chirho: &'imports_chirho TypeFamilyEnvChirho,
    pub imported_class_env_chirho: &'imports_chirho ClassEnvChirho,
    pub imported_record_field_names_chirho: &'imports_chirho HashMap<String, Vec<String>>,
    pub safe_unqualified_imported_type_names_chirho: &'imports_chirho HashSet<String>,
    pub preferred_qualified_type_names_chirho: &'imports_chirho HashMap<String, String>,
    /// `None` when naming did not run; see `InferCtxChirho::promotion_scope_chirho`.
    pub promotion_scope_chirho: Option<Arc<PromotionScopeChirho>>,
    pub family_scope_chirho: Option<Arc<TypeFamilyScopeChirho>>,
}

/// Run type inference on a module with everything it imports.
pub fn infer_module_with_import_context_chirho(
    module_chirho: &ModuleChirho,
    context_chirho: InferenceImportContextChirho<'_>,
) -> InferResultChirho {
    let InferenceImportContextChirho {
        imported_types_chirho,
        imported_type_synonyms_chirho,
        imported_type_families_chirho,
        imported_class_env_chirho,
        imported_record_field_names_chirho,
        safe_unqualified_imported_type_names_chirho,
        preferred_qualified_type_names_chirho,
        promotion_scope_chirho,
        family_scope_chirho,
    } = context_chirho;
    let mut ctx_chirho =
        InferCtxChirho::new_with_imported_class_env_chirho(imported_class_env_chirho);
    ctx_chirho.set_imported_type_name_preferences_chirho(
        safe_unqualified_imported_type_names_chirho,
        preferred_qualified_type_names_chirho,
    );
    ctx_chirho.promotion_scope_chirho = promotion_scope_chirho;
    if let Err(error_chirho) = ctx_chirho
        .type_families_chirho
        .merge_chirho(imported_type_families_chirho)
    {
        ctx_chirho
            .family_registration_error_chirho(error_chirho.to_string(), SpanChirho::DUMMY_CHIRHO);
    }
    ctx_chirho.prepare_family_scope_chirho(module_chirho, family_scope_chirho);
    ctx_chirho.verify_imported_open_families_chirho();
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
    // An imported right side is resolved by its exporter: its names carry the stamps.
    // Under a scope it still converts keyed (compiler nil and cons), with nothing to
    // resolve locally.
    let imported_promotion_scope_chirho = ctx_chirho
        .promotion_scope_chirho
        .as_ref()
        .map(|_| PromotionScopeChirho::default());
    let mut sorted_type_synonyms_chirho: Vec<_> = imported_type_synonyms_chirho.iter().collect();
    sorted_type_synonyms_chirho.sort_by(|a_chirho, b_chirho| a_chirho.0.cmp(b_chirho.0));
    for (name_chirho, (params_chirho, rhs_ast_chirho)) in sorted_type_synonyms_chirho {
        let rhs_ty_chirho = ast_type_to_syn_rhs_chirho(
            rhs_ast_chirho,
            params_chirho,
            imported_promotion_scope_chirho.as_ref(),
        );
        ctx_chirho.register_type_synonym_chirho(
            name_chirho.clone(),
            params_chirho.clone(),
            rhs_ty_chirho,
        );
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
    // Promoted keys are the checker's identities; people read the constructors (row 686).
    // workflow: compiler-pipeline-chirho/promoted-constructor-identity-chirho
    result_chirho
        .diagnostics_chirho
        .map_text_chirho(readable_promoted_keys_chirho);
    let families_chirho = &result_chirho.type_families_chirho;
    result_chirho
        .diagnostics_chirho
        .map_text_chirho(|text_chirho| families_chirho.readable_keys_chirho(text_chirho));
    result_chirho.subst_chirho = composed_subst_chirho;
    result_chirho.method_occurrences_chirho = method_occurrences_chirho;
    result_chirho.method_occurrence_totals_chirho = method_occurrence_totals_chirho;
    result_chirho.literal_evidence_chirho = literal_evidence_chirho;
    result_chirho.literal_evidence_by_origin_chirho = literal_evidence_by_origin_chirho;
    result_chirho.reference_evidence_chirho = reference_evidence_chirho;
    result_chirho.reference_evidence_by_origin_chirho = reference_evidence_by_origin_chirho;
    result_chirho
}
