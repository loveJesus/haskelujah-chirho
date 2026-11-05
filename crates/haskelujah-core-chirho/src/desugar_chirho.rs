// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # AST-to-Core desugaring
//!
//! Translates the surface AST into the Core IR. This pass:
//! - Converts multi-equation function bindings into case expressions
//! - Translates pattern matching into flat Core case expressions
//! - Converts if/where/guards into Core let/case
//! - Removes syntactic sugar (do notation, list comprehensions, etc.)

#[path = "desugar_chirho/bindings_chirho.rs"]
mod bindings_chirho;
#[path = "desugar_chirho/references_chirho/builtin_routes_chirho.rs"]
mod builtin_routes_chirho;
#[path = "desugar_chirho/sugar_chirho/comprehensions_chirho.rs"]
mod comprehensions_chirho;
#[path = "desugar_chirho/constructors_chirho.rs"]
mod constructors_chirho;
#[path = "desugar_chirho/sugar_chirho/do_notation_chirho.rs"]
mod do_notation_chirho;
#[path = "desugar_chirho/expressions_chirho.rs"]
mod expressions_chirho;
#[path = "desugar_chirho/patterns_chirho/guards_chirho.rs"]
mod guards_chirho;
#[path = "desugar_chirho/patterns_chirho/lazy_patterns_chirho.rs"]
mod lazy_patterns_chirho;
#[path = "desugar_chirho/patterns_chirho/match_rows_chirho.rs"]
mod match_rows_chirho;
#[path = "desugar_chirho/patterns_chirho/matches_chirho.rs"]
mod matches_chirho;
#[path = "desugar_chirho/module_chirho.rs"]
mod module_chirho;
#[path = "desugar_chirho/references_chirho/operator_evidence_chirho.rs"]
mod operator_evidence_chirho;
#[path = "desugar_chirho/references_chirho/provenance_chirho.rs"]
mod provenance_chirho;
#[cfg(test)]
#[path = "desugar_chirho/references_chirho/provenance_tests_chirho.rs"]
mod provenance_tests_chirho;
#[path = "desugar_chirho/references_chirho/qualified_references_chirho.rs"]
mod qualified_references_chirho;
pub use qualified_references_chirho::ImportSuppliersChirho;
#[path = "desugar_chirho/sugar_chirho/rec_desugar_chirho.rs"]
mod rec_desugar_chirho;
#[path = "desugar_chirho/patterns_chirho/row_match_chirho.rs"]
mod row_match_chirho;
#[path = "desugar_chirho/patterns_chirho/synonyms_chirho.rs"]
mod synonyms_chirho;
#[path = "desugar_chirho/references_chirho/syntax_methods_chirho.rs"]
mod syntax_methods_chirho;

use match_rows_chirho::{MatchRowChirho, RowBodyChirho};

use std::collections::{BTreeMap, HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DeclChirho, PatSynDirChirho, StrictnessChirho,
};
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, LocalBindChirho, MatchArmChirho, RhsChirho};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::provenance_chirho::{OccurrenceRoleChirho, ProvenanceChirho};
use haskelujah_ast_chirho::stmt_operation_chirho::SelectedOperationChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, ImportedReferenceChirho, InlineAnnotationChirho,
};
use crate::simplify_chirho::free_vars_chirho;

/// Result of desugaring: the Core module plus a symbol table mapping
/// CoreIdChirho → source name for use by downstream passes.
#[derive(Debug)]
pub struct DesugarOutputChirho {
    pub module_chirho: CoreModuleChirho,
    /// Maps every CoreIdChirho to the source name it originated from.
    pub names_chirho: HashMap<CoreIdChirho, String>,
    /// Evidence-threading P1 (design-evidence-threading-chirho.md): per-occurrence
    /// ids for class-method references. Maps each fresh occurrence CoreId to
    /// `(method name, canonical shared CoreId)`. Empty unless the caller opted in
    /// via `set_method_occurrence_names_chirho` — behavior-neutral by default.
    pub method_occurrences_chirho: HashMap<CoreIdChirho, (String, CoreIdChirho)>,
    /// Literal evidence: the `fromInteger` / `fromString` occurrence id a source
    /// literal was wrapped in → the literal's span, so the driver can join the
    /// checker's per-literal instance to it.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub literal_occurrence_spans_chirho: HashMap<CoreIdChirho, SpanChirho>,
    /// Compiler-selected Enum method occurrences, each identified by its range span.
    /// This producer channel keeps their proof independent of lexical method namesakes.
    pub range_occurrence_spans_chirho: HashMap<CoreIdChirho, SpanChirho>,
    /// Reference evidence: the occurrence id minted for a reference to a
    /// constrained top-level name → the reference's source span.
    pub reference_occurrence_spans_chirho: HashMap<CoreIdChirho, SpanChirho>,
    /// Per method-occurrence id, the span of the source reference it stands for.
    /// The evidence join needs this: an occurrence's identity is its span, never
    /// its position among the occurrences of that name.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub method_occurrence_spans_chirho: HashMap<CoreIdChirho, SpanChirho>,
    /// Per minted occurrence id, the producer-minted origin and role of the use
    /// it stands for, copied from the AST. The join matches by this first.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub occurrence_provenance_chirho: HashMap<CoreIdChirho, ProvenanceChirho>,
    /// The name of every instance row the desugarer filled from its class's default, never a
    /// row an instance wrote. A GND newtype's default rows take its representation's methods
    /// (row 694); every other row keeps its body.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub default_method_rows_chirho: std::collections::HashSet<String>,
    /// The minted occurrences that stand for a written infix or section Ord operator
    /// (row 610). Desugaring once lowered each to an Int primop; the dictionary pass
    /// gives one that primop back where no row serves its proof.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub operator_occurrences_chirho: HashSet<CoreIdChirho>,
}

/// A registered pattern synonym definition used during desugaring.
#[derive(Debug, Clone)]
struct PatSynDefChirho {
    /// Formal parameter names of the synonym (`pattern P x y <- ...` → `["x", "y"]`).
    args_chirho: Vec<String>,
    /// The expansion pattern.
    pat_chirho: PatChirho,
    /// Directionality (implicit bidir, unidir, explicit bidir).
    dir_chirho: PatSynDirChirho,
}

/// The desugaring context — generates fresh Core IDs.
pub struct DesugarCtxChirho {
    next_id_chirho: u32,
    /// Tracks CoreId → source name associations.
    names_chirho: HashMap<CoreIdChirho, String>,
    /// Scope: maps source name → CoreIdChirho for variables currently in scope.
    /// Newer entries shadow older ones (last-wins in the Vec of scopes).
    scope_chirho: Vec<HashMap<String, CoreIdChirho>>,
    /// Cache for free variable references: ensures each unscoped name gets
    /// the same CoreId across multiple references, so the dict pass can
    /// create a single binding that all references share.
    free_var_cache_chirho: HashMap<String, CoreIdChirho>,
    /// Which program modules define each imported value name (rows 629 and 642).
    import_suppliers_chirho: ImportSuppliersChirho,
    /// The id of each imported user reference, by its identity (row 629).
    imported_reference_ids_chirho: HashMap<ImportedReferenceChirho, CoreIdChirho>,
    /// Each imported user reference's identity, for the output's provenance (row 629).
    imported_references_chirho: BTreeMap<CoreIdChirho, ImportedReferenceChirho>,
    /// Active LANGUAGE extensions (e.g. "OverloadedStrings").
    extensions_chirho: Vec<String>,
    /// Name of the module currently being desugared.
    current_module_name_chirho: Option<String>,
    /// Constructor strictness: maps constructor name → list of field strictness annotations.
    /// Used to enforce strict fields by wrapping them in `case arg of { _ -> arg }`.
    con_strictness_chirho: HashMap<String, Vec<StrictnessChirho>>,
    /// Record constructor field names: maps constructor name → ordered list of field names.
    /// Used for RecordWildCards expansion (`Foo{..}` fills in missing fields).
    con_field_names_chirho: HashMap<String, Vec<String>>,
    /// Constructor arities for the ordinary and record constructors declared
    /// in this module: `C {}` on a positional constructor supplies no fields,
    /// so every argument becomes a missing-field thunk.
    con_arities_chirho: HashMap<String, usize>,
    /// Pattern synonym definitions: maps synonym name → definition.
    pat_syns_chirho: HashMap<String, PatSynDefChirho>,
    /// Evidence-threading P1: names (class methods) whose free references get a
    /// FRESH occurrence CoreId per reference instead of the shared cache id.
    /// Empty by default — `resolve_var_chirho` then behaves exactly as before.
    method_occurrence_names_chirho: HashSet<String>,
    /// Evidence-threading P1: occurrence id → (method name, canonical shared id).
    method_occurrences_chirho: HashMap<CoreIdChirho, (String, CoreIdChirho)>,
    /// Literal evidence: literal occurrence id → the literal's source span.
    literal_occurrence_spans_chirho: HashMap<CoreIdChirho, SpanChirho>,
    range_occurrence_spans_chirho: HashMap<CoreIdChirho, SpanChirho>,
    /// Reference evidence: names whose scheme carries class predicates; a
    /// reference to one gets its own occurrence id (canonical = the binding).
    constrained_names_chirho: HashSet<String>,
    /// Reference evidence: reference occurrence id → the reference's span.
    reference_occurrence_spans_chirho: HashMap<CoreIdChirho, SpanChirho>,
    method_occurrence_spans_chirho: HashMap<CoreIdChirho, SpanChirho>,
    occurrence_provenance_chirho: HashMap<CoreIdChirho, ProvenanceChirho>,
    /// Written infix and section Ord operators' minted occurrences (row 610).
    operator_occurrences_chirho: HashSet<CoreIdChirho>,
}

impl DesugarCtxChirho {
    pub fn new_chirho() -> Self {
        Self {
            next_id_chirho: 0,
            names_chirho: HashMap::new(),
            scope_chirho: vec![HashMap::new()],
            free_var_cache_chirho: HashMap::new(),
            import_suppliers_chirho: ImportSuppliersChirho::default(),
            imported_reference_ids_chirho: HashMap::new(),
            imported_references_chirho: BTreeMap::new(),
            extensions_chirho: Vec::new(),
            current_module_name_chirho: None,
            con_strictness_chirho: HashMap::new(),
            con_field_names_chirho: HashMap::new(),
            con_arities_chirho: HashMap::new(),
            pat_syns_chirho: HashMap::new(),
            method_occurrence_names_chirho: HashSet::new(),
            method_occurrences_chirho: HashMap::new(),
            literal_occurrence_spans_chirho: HashMap::new(),
            range_occurrence_spans_chirho: HashMap::new(),
            constrained_names_chirho: HashSet::new(),
            reference_occurrence_spans_chirho: HashMap::new(),
            method_occurrence_spans_chirho: HashMap::new(),
            occurrence_provenance_chirho: HashMap::new(),
            operator_occurrences_chirho: HashSet::new(),
        }
    }

    /// Opt in to evidence-threading occurrence ids for the given method names.
    /// workflow: monadic-dispatch-chirho (evidence-threading P1)
    pub fn set_method_occurrence_names_chirho(&mut self, names_chirho: HashSet<String>) {
        self.method_occurrence_names_chirho = names_chirho;
    }

    /// Names whose scheme carries class predicates (the driver reads them off
    /// the checker's environment): a reference to one gets its own occurrence
    /// id so the checker's per-reference evidence can reach the dictionary pass.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub fn set_constrained_names_chirho(&mut self, names_chirho: HashSet<String>) {
        self.constrained_names_chirho = names_chirho;
    }

    /// Constructor arities the checker knows, imported constructors included;
    /// this module's own declarations are registered on top of them.
    /// workflow: language-features-chirho/rigid-type-variables-chirho (records)
    pub fn set_constructor_arities_chirho(&mut self, arities_chirho: HashMap<String, usize>) {
        self.con_arities_chirho = arities_chirho;
    }

    /// Mint an occurrence id for a reference to a constrained bound name; any
    /// other reference keeps its id. An occurrence the pass finds no evidence
    /// for is restored to its canonical id before STG.
    fn reference_occurrence_chirho(
        &mut self,
        name_chirho: &str,
        id_chirho: CoreIdChirho,
        span_chirho: SpanChirho,
    ) -> CoreIdChirho {
        if self.method_occurrences_chirho.contains_key(&id_chirho) {
            // `resolve_var_chirho` already minted this occurrence for an opted-in
            // class method, but it had no span there. The span IS the occurrence's
            // identity for the evidence join, so record it here.
            // workflow: language-features-chirho/dictionary-evidence-chirho
            self.method_occurrence_spans_chirho
                .insert(id_chirho, span_chirho);
            return id_chirho;
        }
        if !self.constrained_names_chirho.contains(name_chirho) {
            return id_chirho;
        }
        let occurrence_id_chirho = self.fresh_id_chirho(name_chirho);
        self.method_occurrences_chirho
            .insert(occurrence_id_chirho, (name_chirho.to_string(), id_chirho));
        self.reference_occurrence_spans_chirho
            .insert(occurrence_id_chirho, span_chirho);
        occurrence_id_chirho
    }

    /// Remember which source literal a `fromInteger` / `fromString` occurrence
    /// id wraps, when the name is opted in (a shared canonical id is not one
    /// literal and gets no span).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    fn record_literal_occurrence_chirho(
        &mut self,
        id_chirho: CoreIdChirho,
        lit_chirho: &LitChirho,
    ) {
        if self.method_occurrences_chirho.contains_key(&id_chirho) {
            self.literal_occurrence_spans_chirho
                .insert(id_chirho, lit_chirho.span_chirho());
            if let Some(role_chirho) = lit_chirho.role_chirho() {
                self.record_occurrence_provenance_chirho(
                    id_chirho,
                    lit_chirho.origin_chirho(),
                    role_chirho,
                );
            }
        }
    }

    /// Generate a fresh Core ID and record its name.
    fn fresh_id_chirho(&mut self, name_chirho: &str) -> CoreIdChirho {
        let id_chirho = CoreIdChirho(self.next_id_chirho);
        self.next_id_chirho += 1;
        self.names_chirho.insert(id_chirho, name_chirho.to_string());
        id_chirho
    }

    /// Look up a name in the current scope chain.
    fn lookup_scope_chirho(&self, name_chirho: &str) -> Option<CoreIdChirho> {
        for scope_chirho in self.scope_chirho.iter().rev() {
            if let Some(&id_chirho) = scope_chirho.get(name_chirho) {
                return Some(id_chirho);
            }
        }
        None
    }

    /// Bind a name in the current (innermost) scope.
    fn bind_in_scope_chirho(&mut self, name_chirho: &str, id_chirho: CoreIdChirho) {
        if let Some(scope_chirho) = self.scope_chirho.last_mut() {
            scope_chirho.insert(name_chirho.to_string(), id_chirho);
        }
    }

    fn data_map_import_aliases_chirho() -> [(&'static str, &'static str); 30] {
        [
            ("empty", "mapEmpty"),
            ("singleton", "mapSingleton"),
            ("insert", "mapInsert"),
            ("delete", "mapDelete"),
            ("lookup", "mapLookup"),
            ("member", "mapMember"),
            ("notMember", "mapNotMember"),
            ("findWithDefault", "mapFindWithDefault"),
            ("union", "mapUnion"),
            ("unionWith", "mapUnionWith"),
            ("unions", "mapUnions"),
            ("intersection", "mapIntersection"),
            ("intersectionWith", "mapIntersectionWith"),
            ("difference", "mapDifference"),
            ("map", "mapMap"),
            ("filter", "mapFilter"),
            ("filterWithKey", "mapFilterWithKey"),
            ("foldlWithKey", "mapFoldlWithKey"),
            ("foldlWithKey'", "mapFoldlWithKey'"),
            ("foldrWithKey", "mapFoldrWithKey"),
            ("toList", "mapToList"),
            ("assocs", "mapToList"),
            ("toAscList", "mapToAscList"),
            ("fromList", "mapFromList"),
            ("elems", "mapElems"),
            ("keys", "mapKeys"),
            ("size", "mapSize"),
            ("null", "mapNull"),
            ("adjust", "mapAdjust"),
            ("insertWith", "mapInsertWith"),
        ]
    }

    fn seed_builtin_import_aliases_chirho(&mut self, module_chirho: &ModuleChirho) {
        for import_chirho in &module_chirho.imports_chirho {
            let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
            if !matches!(
                module_name_chirho.as_str(),
                "Data.Map" | "Data.Map.Strict" | "Data.Map.Lazy"
            ) {
                continue;
            }

            let qualifier_chirho = import_chirho
                .alias_chirho
                .as_ref()
                .map(|alias_chirho| alias_chirho.text_chirho().to_string())
                .unwrap_or_else(|| module_name_chirho.clone());

            for (export_name_chirho, source_name_chirho) in Self::data_map_import_aliases_chirho() {
                let source_id_chirho = self.resolve_var_chirho(source_name_chirho);
                self.bind_in_scope_chirho(
                    &format!("{}.{}", qualifier_chirho, export_name_chirho),
                    source_id_chirho,
                );
            }
        }
    }

    fn seed_prelude_qualified_do_aliases_chirho(&mut self, module_chirho: &ModuleChirho) {
        if !module_chirho
            .extensions_chirho
            .iter()
            .any(|extension_chirho| extension_chirho.eq_ignore_ascii_case("QualifiedDo"))
        {
            return;
        }

        for import_chirho in &module_chirho.imports_chirho {
            let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
            if module_name_chirho != "Prelude" {
                continue;
            }
            let qualifier_chirho = import_chirho
                .alias_chirho
                .as_ref()
                .map(|alias_chirho| alias_chirho.text_chirho().to_string())
                .unwrap_or(module_name_chirho);

            for method_chirho in [">>=", ">>", "fail"] {
                let source_id_chirho = self.resolve_var_chirho(method_chirho);
                self.bind_in_scope_chirho(
                    &format!("{qualifier_chirho}.{method_chirho}"),
                    source_id_chirho,
                );
            }
        }
    }

    /// Push a new scope level.
    fn push_scope_chirho(&mut self) {
        self.scope_chirho.push(HashMap::new());
    }

    /// Pop the innermost scope level.
    fn pop_scope_chirho(&mut self) {
        self.scope_chirho.pop();
    }

    /// Look up or create a CoreId for a variable reference.
    /// If the name is in scope (bound by a lambda, let, etc.), return its ID;
    /// otherwise it resolves as a name no scope binds.
    fn resolve_var_chirho(&mut self, name_chirho: &str) -> CoreIdChirho {
        match self.lookup_scope_chirho(name_chirho) {
            Some(id_chirho) => id_chirho,
            None => self.resolve_unscoped_var_chirho(name_chirho),
        }
    }

    /// Resolve a name no scope binds. Check the free-variable cache so repeated
    /// references to the same unscoped name share a single CoreId. If not
    /// cached, create a fresh ID and cache it. A qualified reference to an
    /// import comes here past every scope (row 598).
    fn resolve_unscoped_var_chirho(&mut self, name_chirho: &str) -> CoreIdChirho {
        let canonical_id_chirho = self.canonical_free_id_chirho(name_chirho);
        if self.method_occurrence_names_chirho.contains(name_chirho) {
            // Evidence-threading P1: give each free reference of an opted-in
            // class method its OWN occurrence id (the shared cache id merges
            // all `==`/`+` references, which blinds per-occurrence evidence).
            // The canonical shared id is still allocated/cached so the dict
            // pass can fall back to today's behavior.
            self.mint_method_occurrence_chirho(name_chirho, canonical_id_chirho)
        } else {
            canonical_id_chirho
        }
    }

    /// The one id every unscoped reference to `name_chirho` shares, from the free-variable
    /// cache.
    fn canonical_free_id_chirho(&mut self, name_chirho: &str) -> CoreIdChirho {
        if let Some(&id_chirho) = self.free_var_cache_chirho.get(name_chirho) {
            return id_chirho;
        }
        let id_chirho = self.fresh_id_chirho(name_chirho);
        self.free_var_cache_chirho
            .insert(name_chirho.to_string(), id_chirho);
        id_chirho
    }

    /// A fresh occurrence id for one reference to method `name_chirho`, with the canonical id
    /// the dictionary pass falls back to when no evidence answers it.
    fn mint_method_occurrence_chirho(
        &mut self,
        name_chirho: &str,
        canonical_id_chirho: CoreIdChirho,
    ) -> CoreIdChirho {
        let occurrence_id_chirho = self.fresh_id_chirho(name_chirho);
        self.method_occurrences_chirho.insert(
            occurrence_id_chirho,
            (name_chirho.to_string(), canonical_id_chirho),
        );
        occurrence_id_chirho
    }

    /// Resolve the operation a do statement SELECTED. Lowering chose it once,
    /// recording the binding with its qualifier; this looks that binding up and
    /// does not choose again, so the checker and Core cannot disagree about
    /// which operation a statement stands for.
    ///
    /// A statement carrying no selection never went through lowering's selector
    /// - Template Haskell and anything else that builds statements directly -
    /// and falls back to the name this resolver would itself have chosen. That
    /// is a producer gap to close, not a second opinion: nothing here overrides
    /// a selection that exists.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    /// workflow: language-features-chirho/qualified-do-chirho
    fn resolve_selected_operation_chirho(
        &mut self,
        selected_chirho: Option<&SelectedOperationChirho>,
        qualifier_chirho: Option<&str>,
        method_chirho: &str,
    ) -> CoreIdChirho {
        match selected_chirho {
            Some(selected_chirho) => {
                let name_chirho = &selected_chirho.name_chirho;
                let text_chirho = name_chirho.text_chirho().to_string();
                let carried_qualifier_chirho = name_chirho.qualifier_chirho().map(str::to_string);
                self.resolve_do_method_chirho(carried_qualifier_chirho.as_deref(), &text_chirho)
            }
            None => self.resolve_do_method_chirho(qualifier_chirho, method_chirho),
        }
    }

    /// Resolve one of QualifiedDo's sequencing methods.
    ///
    /// Self-qualification names the current module's ordinary top-level
    /// binding. Other qualifiers remain in the Core name so imported-module
    /// linking can resolve them, and a missing qualified method stays loud.
    /// workflow: language-features-chirho/qualified-do-chirho
    fn resolve_do_method_chirho(
        &mut self,
        qualifier_chirho: Option<&str>,
        method_chirho: &str,
    ) -> CoreIdChirho {
        match qualifier_chirho {
            Some(qualifier_chirho)
                if self.current_module_name_chirho.as_deref() == Some(qualifier_chirho) =>
            {
                self.resolve_var_chirho(method_chirho)
            }
            Some(qualifier_chirho) => {
                self.resolve_var_chirho(&format!("{qualifier_chirho}.{method_chirho}"))
            }
            None => self.resolve_var_chirho(method_chirho),
        }
    }

    /// Build the body of an operator section, emitting PrimOp for
    /// known built-in operators so they work without dict-pass
    /// bindings. `left_chirho` is the first arg, `right_chirho` is
    /// the second arg (already in correct application order).
    /// A written class-method operator with its origin is the exception: it
    /// takes the variable path below, as the infix operator does (R1, rows
    /// 609 and 610; `desugar_chirho/references_chirho/operator_evidence_chirho.rs`).
    fn build_section_body_chirho(
        &mut self,
        op_chirho: &NameChirho,
        left_chirho: CoreExprChirho,
        right_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let op_name_chirho = op_chirho.text_chirho();
        // Direct primop operators
        let primop_chirho = match op_name_chirho {
            "+" => Some("+#"),
            "-" => Some("-#"),
            "*" => Some("*#"),
            "==" => Some("==#"),
            "/=" => Some("/=#"),
            "<" => Some("<#"),
            "<=" => Some("<=#"),
            ">" => Some(">#"),
            ">=" => Some(">=#"),
            "++" => Some("++#"),
            "^" => Some("^#"),
            "**" => Some("**#"),
            _ => None,
        };
        // A written class-method operator takes its occurrence's evidence instead. The Int
        // primop compared `(== [1])`'s lists as empty string prefixes (R1, gpt_chirho
        // #27508), added `(+ 0.5)`'s Doubles as integer bits (row 609), and ordered computed
        // Strings as words (row 610).
        // workflow: language-features-chirho/dictionary-evidence-chirho
        // An operator this module owns keeps its own binding over the table (row 614).
        if let Some(primop_name_chirho) = primop_chirho
            && !self.takes_operator_evidence_chirho(op_chirho)
            && !self.names_owned_binding_chirho(op_chirho)
        {
            return CoreExprChirho::PrimOpChirho {
                name_chirho: primop_name_chirho.to_string(),
                args_chirho: vec![left_chirho, right_chirho],
            };
        }
        // Colon-led operators are constructors, including GADTs absent from the arity table.
        if op_name_chirho.starts_with(':') || self.con_arities_chirho.contains_key(op_name_chirho) {
            return self.enforce_strict_fields_chirho(
                op_name_chirho.to_string(),
                vec![left_chirho, right_chirho],
            );
        }
        // Fallback: operator as variable reference, by a written name's route,
        // so a qualified operator never takes a local binder (row 598).
        self.written_operator_application_chirho(op_chirho, left_chirho, right_chirho)
    }

    /// Create a binder with a fresh ID.
    fn fresh_binder_chirho(
        &mut self,
        name_chirho: &str,
        ty_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> BinderChirho {
        BinderChirho {
            id_chirho: self.fresh_id_chirho(name_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho,
            span_chirho,
        }
    }

    /// Check whether a binding group is (mutually) recursive.
    ///
    /// A binding group is recursive if any RHS references any binder
    /// defined in the same group.
    fn is_recursive_binds_chirho(binds_chirho: &[(BinderChirho, CoreExprChirho)]) -> bool {
        let binder_ids_chirho: HashSet<CoreIdChirho> = binds_chirho
            .iter()
            .map(|(b_chirho, _)| b_chirho.id_chirho)
            .collect();
        for (_, rhs_chirho) in binds_chirho {
            let fvs_chirho = free_vars_chirho(rhs_chirho);
            if fvs_chirho
                .iter()
                .any(|id_chirho| binder_ids_chirho.contains(id_chirho))
            {
                return true;
            }
        }
        false
    }
}

/// Extract the variable name from a pattern, peeling through bang/lazy/paren.
fn extract_var_name_from_pat_chirho(pat_chirho: &PatChirho) -> Option<String> {
    match pat_chirho {
        PatChirho::VarChirho(n_chirho) => Some(n_chirho.text_chirho().to_string()),
        PatChirho::BangChirho { inner_chirho, .. }
        | PatChirho::LazyChirho { inner_chirho, .. }
        | PatChirho::ParenChirho { inner_chirho, .. } => {
            extract_var_name_from_pat_chirho(inner_chirho)
        }
        PatChirho::TypeAnnotChirho { pat_chirho, .. } => {
            extract_var_name_from_pat_chirho(pat_chirho)
        }
        _ => None,
    }
}

/// Desugar a module from AST to Core, returning the Core module and a name map.
pub fn desugar_module_chirho(module_chirho: &ModuleChirho) -> DesugarOutputChirho {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    ctx_chirho.extensions_chirho = module_chirho.extensions_chirho.clone();
    ctx_chirho.desugar_module_chirho(module_chirho)
}

/// Evidence-threading P2b: like [`desugar_module_chirho`] but allocates fresh
/// per-occurrence CoreIds for free references of the given class-method names
/// (recorded in `DesugarOutputChirho::method_occurrences_chirho`).
/// workflow: monadic-dispatch-chirho (evidence-threading)
pub fn desugar_module_with_method_occurrences_chirho(
    module_chirho: &ModuleChirho,
    method_names_chirho: HashSet<String>,
) -> DesugarOutputChirho {
    desugar_module_with_inputs_chirho(
        module_chirho,
        DesugarInputsChirho {
            method_names_chirho,
            ..DesugarInputsChirho::default()
        },
    )
}

/// What the driver hands the desugarer besides the AST: facts read off the
/// checker's environment that the AST alone does not carry.
#[derive(Debug, Default)]
pub struct DesugarInputsChirho {
    /// Class methods whose free references each get a fresh occurrence id
    /// (evidence-threading P1).
    pub method_names_chirho: HashSet<String>,
    /// Names whose scheme carries class predicates: each reference gets an
    /// occurrence id the checker's per-reference evidence joins to.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub constrained_names_chirho: HashSet<String>,
    /// Constructor arities, imported constructors included, so `C {}` on a
    /// positional constructor fills every argument with a missing-field thunk.
    /// workflow: language-features-chirho/rigid-type-variables-chirho (records)
    pub constructor_arities_chirho: HashMap<String, usize>,
    /// Which program modules define each imported value name, so a reference to one takes its
    /// definer's identity (rows 629 and 642).
    /// workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
    pub import_suppliers_chirho: ImportSuppliersChirho,
}

/// Like [`desugar_module_with_method_occurrences_chirho`] with the full set of
/// checker-derived inputs.
pub fn desugar_module_with_inputs_chirho(
    module_chirho: &ModuleChirho,
    inputs_chirho: DesugarInputsChirho,
) -> DesugarOutputChirho {
    let mut ctx_chirho = DesugarCtxChirho::new_chirho();
    ctx_chirho.extensions_chirho = module_chirho.extensions_chirho.clone();
    ctx_chirho.set_method_occurrence_names_chirho(inputs_chirho.method_names_chirho);
    ctx_chirho.set_constrained_names_chirho(inputs_chirho.constrained_names_chirho);
    ctx_chirho.set_constructor_arities_chirho(inputs_chirho.constructor_arities_chirho);
    ctx_chirho.import_suppliers_chirho = inputs_chirho.import_suppliers_chirho;
    ctx_chirho.desugar_module_chirho(module_chirho)
}

/// Map a source-level operator name to a primop name, if it is a known
/// built-in primitive. Returns `None` for user-defined operators.
///
/// Note: `==` and `/=` are intentionally NOT listed here — they are Eq
/// class methods and must go through the dictionary-passing transform so
/// that the correct instance (e.g. `Eq Color` vs `Eq Int`) is selected
/// Map infix operator names to their primitive machine operation names.
///
/// `+`, `-`, `*` are Num class methods and go through the dictionary-
/// passing transform instead of being lowered to primops directly.
/// `==`, `/=` are Eq class methods and handled separately in the
/// desugarer. The Ord operators are primops here only where the written
/// operator takes no evidence (row 610).
fn builtin_primop_name_chirho(op_chirho: &str) -> Option<&'static str> {
    match op_chirho {
        // +, -, * are handled by the Num class dict transform
        // / is handled by the Fractional class dict transform
        "==#" => Some("==#"),
        "eqFloat#" => Some("eqFloat#"),
        "eqStr#" => Some("eqStr#"),
        "div" => Some("div#"),
        "mod" => Some("mod#"),
        "<" => Some("<#"),
        "<=" => Some("<=#"),
        ">" => Some(">#"),
        ">=" => Some(">=#"),
        "++" => Some("++#"),
        _ => None,
    }
}

/// Map prefix function names to binary primop names.
/// These are functions like `div` and `mod` that can be called in
/// prefix position: `div a b` or `mod a b`.
fn prefix_binary_primop_name_chirho(name_chirho: &str) -> Option<&'static str> {
    match name_chirho {
        "div" => Some("div#"),
        "mod" => Some("mod#"),
        "quot" => Some("quot#"),
        "rem" => Some("rem#"),
        _ => None,
    }
}

/// Check whether a Core expression is a float literal (used for
/// type-directed Num dispatch at the desugaring level).
fn is_float_core_chirho(expr_chirho: &CoreExprChirho) -> bool {
    matches!(
        expr_chirho,
        CoreExprChirho::LitChirho(crate::expr_chirho::CoreLitChirho::FloatChirho(_))
    )
}

#[cfg(test)]
mod tests_chirho;
