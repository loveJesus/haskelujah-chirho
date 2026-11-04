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
#[path = "desugar_chirho/patterns_chirho/guards_chirho.rs"]
mod guards_chirho;
#[path = "desugar_chirho/patterns_chirho/lazy_patterns_chirho.rs"]
mod lazy_patterns_chirho;
#[path = "desugar_chirho/patterns_chirho/match_rows_chirho.rs"]
mod match_rows_chirho;
#[path = "desugar_chirho/patterns_chirho/matches_chirho.rs"]
mod matches_chirho;
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

    /// Derive the type key string from an AST `TypeChirho`, matching
    /// the format produced by `Display for TyChirho` in the typing layer
    /// so that the `$prim_` binding names align with `dict_chirho.rs`.
    pub fn type_key_from_ast_chirho(ty_chirho: &TypeChirho) -> String {
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) => name_chirho.text_chirho().to_string(),
            TypeChirho::VarChirho(name_chirho) => name_chirho.text_chirho().to_string(),
            TypeChirho::ListChirho { element_chirho, .. } => {
                format!("[{}]", Self::type_key_from_ast_chirho(element_chirho))
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let inner_chirho: Vec<String> = elements_chirho
                    .iter()
                    .map(|e_chirho| Self::type_key_from_ast_chirho(e_chirho))
                    .collect();
                format!("({})", inner_chirho.join(", "))
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                format!(
                    "{} {}",
                    Self::type_key_from_ast_chirho(fun_chirho),
                    Self::type_key_from_ast_chirho(arg_chirho)
                )
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                Self::type_key_from_ast_chirho(inner_chirho)
            }
            _ => "_".to_string(),
        }
    }

    /// Build a type key for an instance declaration from its AST types.
    /// For single-parameter classes: just the first type key.
    /// For multi-parameter classes: head_key + "_" + extra keys joined by "_".
    /// Matches the format used in `dict_chirho.rs::generate_instance_dicts_chirho`.
    pub fn instance_type_key_chirho(types_chirho: &[TypeChirho]) -> String {
        if types_chirho.is_empty() {
            "_".to_string()
        } else if types_chirho.len() == 1 {
            Self::type_key_from_ast_chirho(&types_chirho[0])
        } else {
            let keys_chirho: Vec<String> = types_chirho
                .iter()
                .map(|t_chirho| Self::type_key_from_ast_chirho(t_chirho))
                .collect();
            keys_chirho.join("_")
        }
    }

    /// Desugar an entire AST module into a Core module with name map.
    pub fn desugar_module_chirho(&mut self, module_chirho: &ModuleChirho) -> DesugarOutputChirho {
        self.current_module_name_chirho = Some(module_chirho.name_chirho.text_chirho().to_string());

        // Pass -1: Collect constructor strictness annotations for strict field enforcement
        self.collect_con_strictness_chirho(module_chirho);

        // Pass -0.5: Collect pattern synonym definitions for expansion during desugaring
        self.collect_pat_syns_chirho(module_chirho);

        // Pass -0.25: Seed aliases for built-in qualified imports whose Core
        // bodies use internal Prelude wrapper names.
        self.seed_builtin_import_aliases_chirho(module_chirho);
        // Unit 517: every `toList` an import of Data.List.NonEmpty or Data.Foldable
        // exposes reaches the dual-shape route, ahead of the module's own names.
        self.seed_to_list_routes_chirho(module_chirho);
        // QualifiedDo methods are selected through the source module qualifier,
        // while today's Core linker addresses imported values by bare export
        // name. Snapshot Prelude's methods before local bindings shadow them.
        // Other modules stay qualified and fail loudly until the linker can
        // distinguish same-named exports by defining module.
        // workflow: language-features-chirho/qualified-do-chirho
        self.seed_prelude_qualified_do_aliases_chirho(module_chirho);

        // Pass 0: Build map of class → (method_name → default MatchArms) from class declarations
        let mut class_defaults_chirho: HashMap<String, HashMap<String, Vec<MatchArmChirho>>> =
            HashMap::new();
        let mut class_method_names_chirho: HashMap<String, Vec<String>> = HashMap::new();
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::ClassDeclChirho {
                name_chirho,
                methods_chirho,
                ..
            } = decl_chirho
            {
                let cn_chirho = name_chirho.text_chirho().to_string();
                let mut defaults_chirho = HashMap::new();
                let mut method_names_chirho = Vec::new();
                for method_chirho in methods_chirho {
                    let mn_chirho = method_chirho.name_chirho.text_chirho().to_string();
                    method_names_chirho.push(mn_chirho.clone());
                    if let Some(ref arms_chirho) = method_chirho.default_chirho {
                        defaults_chirho.insert(mn_chirho, arms_chirho.clone());
                    }
                }
                class_defaults_chirho.insert(cn_chirho.clone(), defaults_chirho);
                class_method_names_chirho.insert(cn_chirho, method_names_chirho);
            }
        }

        // Pass 1: Pre-bind all top-level names so they can reference each other
        let mut pre_binders_chirho: Vec<(usize, BinderChirho)> = Vec::new();
        for (idx_chirho, decl_chirho) in module_chirho.decls_chirho.iter().enumerate() {
            match decl_chirho {
                DeclChirho::FunBindChirho {
                    name_chirho,
                    span_chirho,
                    ..
                } => {
                    let binder_chirho = self.fresh_binder_chirho(
                        name_chirho.text_chirho(),
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        *span_chirho,
                    );
                    self.bind_in_scope_chirho(name_chirho.text_chirho(), binder_chirho.id_chirho);
                    pre_binders_chirho.push((idx_chirho, binder_chirho));
                }
                DeclChirho::InstanceDeclChirho {
                    class_chirho,
                    types_chirho,
                    methods_chirho,
                    span_chirho,
                    ..
                } => {
                    let class_name_chirho = class_chirho.text_chirho();
                    let type_key_chirho = Self::instance_type_key_chirho(types_chirho);
                    // Collect provided method names
                    let mut provided_chirho: std::collections::HashSet<String> =
                        std::collections::HashSet::new();
                    for method_chirho in methods_chirho {
                        if let LocalBindChirho::FunBindChirho {
                            name_chirho: method_name_chirho,
                            ..
                        } = method_chirho
                        {
                            let mn_chirho = method_name_chirho.text_chirho().to_string();
                            provided_chirho.insert(mn_chirho.clone());
                            let prim_name_chirho = format!(
                                "$prim_{}_{}_{}",
                                class_name_chirho, mn_chirho, type_key_chirho
                            );
                            let binder_chirho = self.fresh_binder_chirho(
                                &prim_name_chirho,
                                TyChirho::VarChirho(
                                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                *span_chirho,
                            );
                            self.bind_in_scope_chirho(&prim_name_chirho, binder_chirho.id_chirho);
                            pre_binders_chirho.push((idx_chirho, binder_chirho));
                        }
                    }
                    // Pre-bind default methods not provided by the instance
                    if let Some(defaults_chirho) =
                        class_defaults_chirho.get(&class_name_chirho.to_string())
                    {
                        for (default_name_chirho, _) in defaults_chirho {
                            if !provided_chirho.contains(default_name_chirho) {
                                let prim_name_chirho = format!(
                                    "$prim_{}_{}_{}",
                                    class_name_chirho, default_name_chirho, type_key_chirho
                                );
                                let binder_chirho = self.fresh_binder_chirho(
                                    &prim_name_chirho,
                                    TyChirho::VarChirho(
                                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                            self.next_id_chirho,
                                        ),
                                    ),
                                    *span_chirho,
                                );
                                self.bind_in_scope_chirho(
                                    &prim_name_chirho,
                                    binder_chirho.id_chirho,
                                );
                                pre_binders_chirho.push((idx_chirho, binder_chirho));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Pass 1b: Pre-bind record field accessor names so function bodies can reference them
        for decl_chirho in &module_chirho.decls_chirho {
            let constructors_chirho = match decl_chirho {
                DeclChirho::DataDeclChirho {
                    constructors_chirho,
                    ..
                } => constructors_chirho.as_slice(),
                DeclChirho::NewtypeDeclChirho {
                    constructor_chirho, ..
                } => std::slice::from_ref(constructor_chirho),
                _ => continue,
            };
            for con_chirho in constructors_chirho {
                if let haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                    fields_chirho: field_decls_chirho,
                    ..
                } = con_chirho
                {
                    for fd_chirho in field_decls_chirho {
                        for fname_chirho in &fd_chirho.names_chirho {
                            let name_str_chirho = fname_chirho.text_chirho().to_string();
                            let binder_chirho = self.fresh_binder_chirho(
                                &name_str_chirho,
                                TyChirho::VarChirho(
                                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                SpanChirho::DUMMY_CHIRHO,
                            );
                            self.bind_in_scope_chirho(&name_str_chirho, binder_chirho.id_chirho);
                            // Pre-bind setter function
                            let setter_name_chirho = format!("$setField_{}", name_str_chirho);
                            let setter_binder_chirho = self.fresh_binder_chirho(
                                &setter_name_chirho,
                                TyChirho::VarChirho(
                                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                SpanChirho::DUMMY_CHIRHO,
                            );
                            self.bind_in_scope_chirho(
                                &setter_name_chirho,
                                setter_binder_chirho.id_chirho,
                            );
                        }
                    }
                }
            }
        }

        // Convert AST inline pragmas to Core inline annotations
        let inline_pragmas_chirho = &module_chirho.inline_pragmas_chirho;

        // Pass 2: Desugar bodies
        let mut bindings_chirho = Vec::new();
        let mut pre_iter_chirho = pre_binders_chirho.into_iter().peekable();

        for (idx_chirho, decl_chirho) in module_chirho.decls_chirho.iter().enumerate() {
            match decl_chirho {
                DeclChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                } => {
                    let core_rhs_chirho = self.desugar_matches_chirho(matches_chirho, *span_chirho);
                    // Use the pre-bound binder
                    let binder_chirho = if let Some((pre_idx, _)) = pre_iter_chirho.peek() {
                        if *pre_idx == idx_chirho {
                            pre_iter_chirho.next().unwrap().1
                        } else {
                            // Shouldn't happen, but fall back
                            self.fresh_binder_chirho(
                                "_unknown",
                                TyChirho::VarChirho(
                                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                *span_chirho,
                            )
                        }
                    } else {
                        self.fresh_binder_chirho(
                            "_unknown",
                            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            *span_chirho,
                        )
                    };
                    // Look up inline pragma for this binding
                    let inline_annotation_chirho = match inline_pragmas_chirho.get(&name_chirho.text_chirho().to_string()) {
                        Some(haskelujah_ast_chirho::module_chirho::InlinePragmaChirho::InlineChirho) => InlineAnnotationChirho::AlwaysChirho,
                        Some(haskelujah_ast_chirho::module_chirho::InlinePragmaChirho::NoInlineChirho) => InlineAnnotationChirho::NeverChirho,
                        Some(haskelujah_ast_chirho::module_chirho::InlinePragmaChirho::InlinableChirho) => InlineAnnotationChirho::InlinableChirho,
                        None => InlineAnnotationChirho::NoneChirho,
                    };
                    bindings_chirho.push(CoreBindingChirho {
                        binder_chirho,
                        rhs_chirho: core_rhs_chirho,
                        is_rec_chirho: true,
                        inline_chirho: inline_annotation_chirho,
                    });
                }
                DeclChirho::PatBindChirho {
                    pat_chirho,
                    rhs_chirho,
                    span_chirho,
                } => {
                    let core_rhs_chirho = self.desugar_rhs_chirho(rhs_chirho);
                    // Simple variable pattern: just create a direct binding
                    if let PatChirho::VarChirho(name_chirho) = pat_chirho {
                        let binder_chirho = self.fresh_binder_chirho(
                            name_chirho.text_chirho(),
                            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            *span_chirho,
                        );
                        self.bind_in_scope_chirho(
                            name_chirho.text_chirho(),
                            binder_chirho.id_chirho,
                        );
                        bindings_chirho.push(CoreBindingChirho {
                            binder_chirho,
                            rhs_chirho: core_rhs_chirho,
                            is_rec_chirho: false,
                            inline_chirho: InlineAnnotationChirho::NoneChirho,
                        });
                    } else {
                        // A top-level pattern binding, as the Report defines it:
                        // each variable is bound through one shared match of the
                        // WHOLE pattern, performed when any of them is demanded.
                        // (A top-level binding cannot be banged, so there is no
                        // witness to force.)
                        // workflow: language-features-chirho/pattern-matching-chirho
                        let binding_chirho = self.pattern_binding_chirho(
                            pat_chirho,
                            core_rhs_chirho,
                            &HashMap::new(),
                        );
                        for (binder_chirho, rhs_chirho) in binding_chirho.bindings_chirho {
                            bindings_chirho.push(CoreBindingChirho {
                                binder_chirho,
                                rhs_chirho,
                                is_rec_chirho: false,
                                inline_chirho: InlineAnnotationChirho::NoneChirho,
                            });
                        }
                    }
                }
                DeclChirho::InstanceDeclChirho {
                    class_chirho,
                    types_chirho,
                    methods_chirho,
                    span_chirho,
                    ..
                } => {
                    let class_name_chirho = class_chirho.text_chirho();
                    let type_key_chirho = Self::instance_type_key_chirho(types_chirho);
                    let mut provided_chirho: std::collections::HashSet<String> =
                        std::collections::HashSet::new();
                    for method_chirho in methods_chirho {
                        if let LocalBindChirho::FunBindChirho {
                            name_chirho: method_name_chirho,
                            matches_chirho: method_matches_chirho,
                            span_chirho: method_span_chirho,
                        } = method_chirho
                        {
                            provided_chirho.insert(method_name_chirho.text_chirho().to_string());
                            let core_rhs_chirho = self
                                .desugar_matches_chirho(method_matches_chirho, *method_span_chirho);
                            let prim_name_chirho = format!(
                                "$prim_{}_{}_{}",
                                class_name_chirho,
                                method_name_chirho.text_chirho(),
                                type_key_chirho
                            );
                            // Consume pre-bound binder for this instance
                            // (multiple methods share the same idx_chirho)
                            let binder_chirho =
                                if let Some((pre_idx_chirho, _)) = pre_iter_chirho.peek() {
                                    if *pre_idx_chirho == idx_chirho {
                                        pre_iter_chirho.next().unwrap().1
                                    } else {
                                        self.fresh_binder_chirho(
                                            &prim_name_chirho,
                                            TyChirho::VarChirho(
                                                haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                                    self.next_id_chirho,
                                                ),
                                            ),
                                            *span_chirho,
                                        )
                                    }
                                } else {
                                    self.fresh_binder_chirho(
                                        &prim_name_chirho,
                                        TyChirho::VarChirho(
                                            haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                                self.next_id_chirho,
                                            ),
                                        ),
                                        *span_chirho,
                                    )
                                };
                            bindings_chirho.push(CoreBindingChirho {
                                binder_chirho,
                                rhs_chirho: core_rhs_chirho,
                                is_rec_chirho: false,
                                inline_chirho: InlineAnnotationChirho::NoneChirho,
                            });
                        }
                    }
                    // Generate bindings for default methods not provided
                    if let Some(defaults_chirho) =
                        class_defaults_chirho.get(&class_name_chirho.to_string())
                    {
                        for (default_name_chirho, default_arms_chirho) in defaults_chirho {
                            if !provided_chirho.contains(default_name_chirho) {
                                let core_rhs_chirho =
                                    self.desugar_matches_chirho(default_arms_chirho, *span_chirho);
                                let prim_name_chirho = format!(
                                    "$prim_{}_{}_{}",
                                    class_name_chirho, default_name_chirho, type_key_chirho
                                );
                                let binder_chirho =
                                    if let Some((pre_idx_chirho, _)) = pre_iter_chirho.peek() {
                                        if *pre_idx_chirho == idx_chirho {
                                            pre_iter_chirho.next().unwrap().1
                                        } else {
                                            self.fresh_binder_chirho(
                                            &prim_name_chirho,
                                            TyChirho::VarChirho(
                                                haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                                    self.next_id_chirho,
                                                ),
                                            ),
                                            *span_chirho,
                                        )
                                        }
                                    } else {
                                        self.fresh_binder_chirho(
                                            &prim_name_chirho,
                                            TyChirho::VarChirho(
                                                haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                                    self.next_id_chirho,
                                                ),
                                            ),
                                            *span_chirho,
                                        )
                                    };
                                bindings_chirho.push(CoreBindingChirho {
                                    binder_chirho,
                                    rhs_chirho: core_rhs_chirho,
                                    is_rec_chirho: false,
                                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                                });
                            }
                        }
                    }
                }
                // Data, type, class declarations don't produce Core bindings directly
                _ => {}
            }
        }

        // Generate record field accessor functions.
        // For data T = Con { f1 :: T1, f2 :: T2 }, generates:
        //   f1 = \r -> case r of { Con x _ -> x }
        //   f2 = \r -> case r of { Con _ x -> x }
        // Also handles newtype T = Con { f :: T1 } (single-field accessor).
        for decl_chirho in &module_chirho.decls_chirho {
            let constructors_chirho = match decl_chirho {
                DeclChirho::DataDeclChirho {
                    constructors_chirho,
                    ..
                } => constructors_chirho.as_slice(),
                DeclChirho::NewtypeDeclChirho {
                    constructor_chirho, ..
                } => std::slice::from_ref(constructor_chirho),
                _ => continue,
            };
            for con_chirho in constructors_chirho {
                if let haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                    name_chirho: con_name_chirho,
                    fields_chirho: field_decls_chirho,
                    ..
                } = con_chirho
                {
                    // Flatten field names with their position index
                    let mut flat_fields_chirho: Vec<(String, usize)> = Vec::new();
                    for fd_chirho in field_decls_chirho {
                        for fname_chirho in &fd_chirho.names_chirho {
                            flat_fields_chirho.push((
                                fname_chirho.text_chirho().to_string(),
                                flat_fields_chirho.len(),
                            ));
                        }
                    }
                    let total_fields_chirho = flat_fields_chirho.len();
                    let con_str_chirho = con_name_chirho.text_chirho().to_string();

                    for (field_name_chirho, field_idx_chirho) in &flat_fields_chirho {
                        // Build: \r -> case r of { Con b0 b1 ... -> b_idx }
                        let param_binder_chirho = self.fresh_binder_chirho(
                            "$r",
                            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        let mut alt_binders_chirho = Vec::new();
                        let mut result_id_chirho = CoreIdChirho(0);
                        for i_chirho in 0..total_fields_chirho {
                            let b_chirho = self.fresh_binder_chirho(
                                &format!("$f{}", i_chirho),
                                TyChirho::VarChirho(
                                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                SpanChirho::DUMMY_CHIRHO,
                            );
                            if i_chirho == *field_idx_chirho {
                                result_id_chirho = b_chirho.id_chirho;
                            }
                            alt_binders_chirho.push(b_chirho);
                        }

                        let case_bind_chirho = self.fresh_binder_chirho(
                            "$scrut",
                            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        let body_chirho = CoreExprChirho::CaseChirho {
                            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                                param_binder_chirho.id_chirho,
                            )),
                            bind_chirho: case_bind_chirho,
                            result_ty_chirho: TyChirho::VarChirho(
                                haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                    self.next_id_chirho,
                                ),
                            ),
                            alts_chirho: vec![CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho(con_str_chirho.clone()),
                                binders_chirho: alt_binders_chirho,
                                rhs_chirho: CoreExprChirho::VarChirho(result_id_chirho),
                            }],
                        };

                        let accessor_rhs_chirho = CoreExprChirho::LamChirho {
                            binder_chirho: param_binder_chirho.clone(),
                            body_chirho: Box::new(body_chirho),
                        };

                        // Reuse the pre-bound ID from Pass 1b
                        let accessor_id_chirho = self
                            .lookup_scope_chirho(field_name_chirho)
                            .unwrap_or_else(|| self.fresh_id_chirho(field_name_chirho));
                        let accessor_binder_chirho = BinderChirho {
                            id_chirho: accessor_id_chirho,
                            name_chirho: field_name_chirho.clone(),
                            ty_chirho: TyChirho::VarChirho(
                                haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                    self.next_id_chirho,
                                ),
                            ),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        };
                        bindings_chirho.push(CoreBindingChirho {
                            binder_chirho: accessor_binder_chirho,
                            rhs_chirho: accessor_rhs_chirho,
                            is_rec_chirho: false,
                            inline_chirho: InlineAnnotationChirho::NoneChirho,
                        });

                        // Generate setter: $setField_f = \newVal -> \rec -> case rec of { Con b0 b1 ... -> Con ... newVal ... }
                        let setter_name_chirho = format!("$setField_{}", field_name_chirho);
                        let new_val_binder_chirho = self.fresh_binder_chirho(
                            "$newVal",
                            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        let rec_binder_chirho = self.fresh_binder_chirho(
                            "$rec",
                            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        let mut setter_alt_binders_chirho = Vec::new();
                        let mut con_args_chirho = Vec::new();
                        for i_chirho in 0..total_fields_chirho {
                            let b_chirho = self.fresh_binder_chirho(
                                &format!("$sf{}", i_chirho),
                                TyChirho::VarChirho(
                                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                SpanChirho::DUMMY_CHIRHO,
                            );
                            if i_chirho == *field_idx_chirho {
                                // Replace this field with newVal
                                con_args_chirho.push(CoreExprChirho::VarChirho(
                                    new_val_binder_chirho.id_chirho,
                                ));
                            } else {
                                con_args_chirho.push(CoreExprChirho::VarChirho(b_chirho.id_chirho));
                            }
                            setter_alt_binders_chirho.push(b_chirho);
                        }
                        let setter_case_bind_chirho = self.fresh_binder_chirho(
                            "$scrut",
                            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            SpanChirho::DUMMY_CHIRHO,
                        );
                        let setter_body_chirho = CoreExprChirho::CaseChirho {
                            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                                rec_binder_chirho.id_chirho,
                            )),
                            bind_chirho: setter_case_bind_chirho,
                            result_ty_chirho: TyChirho::VarChirho(
                                haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                    self.next_id_chirho,
                                ),
                            ),
                            alts_chirho: vec![CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho(con_str_chirho.clone()),
                                binders_chirho: setter_alt_binders_chirho,
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: con_str_chirho.clone(),
                                    args_chirho: con_args_chirho,
                                },
                            }],
                        };
                        let setter_rhs_chirho = CoreExprChirho::LamChirho {
                            binder_chirho: new_val_binder_chirho,
                            body_chirho: Box::new(CoreExprChirho::LamChirho {
                                binder_chirho: rec_binder_chirho,
                                body_chirho: Box::new(setter_body_chirho),
                            }),
                        };
                        // Reuse pre-bound setter ID from Pass 1b
                        let setter_id_chirho = self
                            .lookup_scope_chirho(&setter_name_chirho)
                            .unwrap_or_else(|| self.fresh_id_chirho(&setter_name_chirho));
                        let setter_binder_chirho = BinderChirho {
                            id_chirho: setter_id_chirho,
                            name_chirho: setter_name_chirho.clone(),
                            ty_chirho: TyChirho::VarChirho(
                                haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                    self.next_id_chirho,
                                ),
                            ),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        };
                        bindings_chirho.push(CoreBindingChirho {
                            binder_chirho: setter_binder_chirho,
                            rhs_chirho: setter_rhs_chirho,
                            is_rec_chirho: false,
                            inline_chirho: InlineAnnotationChirho::NoneChirho,
                        });
                    }
                }
            }
        }

        DesugarOutputChirho {
            module_chirho: CoreModuleChirho {
                name_chirho: module_chirho.name_chirho.text_chirho().to_string(),
                bindings_chirho,
                names_chirho: self.names_chirho.clone(),
                specialize_pragmas_chirho: module_chirho.specialize_pragmas_chirho.clone(),
                foreign_exports_chirho: module_chirho
                    .foreign_exports_chirho
                    .iter()
                    .map(|(h_chirho, c_chirho, cc_chirho)| {
                        crate::expr_chirho::ForeignExportChirho {
                            haskell_name_chirho: h_chirho.clone(),
                            foreign_name_chirho: c_chirho.clone(),
                            calling_conv_chirho: cc_chirho.clone(),
                        }
                    })
                    .collect(),
                provenance_chirho: crate::expr_chirho::CoreProvenanceChirho {
                    imported_references_chirho: self.imported_references_chirho.clone(),
                    ..Default::default()
                },
            },
            names_chirho: self.names_chirho.clone(),
            method_occurrences_chirho: self.method_occurrences_chirho.clone(),
            literal_occurrence_spans_chirho: self.literal_occurrence_spans_chirho.clone(),
            range_occurrence_spans_chirho: self.range_occurrence_spans_chirho.clone(),
            reference_occurrence_spans_chirho: self.reference_occurrence_spans_chirho.clone(),
            method_occurrence_spans_chirho: self.method_occurrence_spans_chirho.clone(),
            occurrence_provenance_chirho: self.occurrence_provenance_chirho.clone(),
            operator_occurrences_chirho: self.operator_occurrences_chirho.clone(),
        }
    }

    /// Desugar an AST expression into a Core expression.
    pub fn desugar_expr_chirho(&mut self, expr_chirho: &ExprChirho) -> CoreExprChirho {
        match expr_chirho {
            ExprChirho::VarChirho(name_chirho) => {
                let (lookup_name_chirho, id_chirho) =
                    self.resolve_written_reference_chirho(name_chirho);
                let id_chirho = self.reference_occurrence_chirho(
                    &lookup_name_chirho,
                    id_chirho,
                    name_chirho.span_chirho(),
                );
                self.record_occurrence_provenance_chirho(
                    id_chirho,
                    name_chirho.origin_chirho(),
                    OccurrenceRoleChirho::Reference,
                );
                CoreExprChirho::VarChirho(id_chirho)
            }
            ExprChirho::ConChirho(name_chirho) => {
                let con_text_chirho = name_chirho.text_chirho();
                // Bidirectional pattern synonyms can be used as expressions.
                if let Some(def_chirho) = self.pat_syns_chirho.get(con_text_chirho).cloned() {
                    if matches!(def_chirho.dir_chirho, PatSynDirChirho::ImplBidirChirho) {
                        // Build: \arg0 arg1 ... -> <expansion_as_expr>
                        let mut param_binders_chirho = Vec::new();
                        let mut subst_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
                        for formal_chirho in &def_chirho.args_chirho {
                            let binder_chirho = self.fresh_binder_chirho(
                                formal_chirho,
                                TyChirho::VarChirho(
                                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                SpanChirho::DUMMY_CHIRHO,
                            );
                            subst_chirho.insert(formal_chirho.clone(), binder_chirho.id_chirho);
                            param_binders_chirho.push(binder_chirho);
                        }
                        let body_chirho =
                            self.pat_to_builder_expr_chirho(&def_chirho.pat_chirho, &subst_chirho);
                        // Wrap body in lambdas (right to left).
                        let mut result_chirho = body_chirho;
                        for binder_chirho in param_binders_chirho.into_iter().rev() {
                            result_chirho = CoreExprChirho::LamChirho {
                                binder_chirho,
                                body_chirho: Box::new(result_chirho),
                            };
                        }
                        return result_chirho;
                    }
                }
                CoreExprChirho::ConAppChirho {
                    con_name_chirho: con_text_chirho.to_string(),
                    args_chirho: vec![],
                }
            }

            ExprChirho::LitChirho(lit_chirho) => {
                let core_lit_chirho = self.desugar_lit_chirho(lit_chirho);
                // Numeric literal overloading (Haskell 2010 §3.2):
                // Integer literals become `fromInteger <lit>`, allowing
                // the dict pass to select the correct Num instance. That
                // `fromInteger` is Num's method, never a binding of the
                // program's own (row 660).
                if matches!(core_lit_chirho, CoreLitChirho::IntChirho(_)) {
                    let from_integer_id_chirho = self.resolve_syntax_method_chirho("fromInteger");
                    self.record_literal_occurrence_chirho(from_integer_id_chirho, lit_chirho);
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(from_integer_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::LitChirho(core_lit_chirho)),
                    }
                } else if matches!(core_lit_chirho, CoreLitChirho::StringChirho(_))
                    && self
                        .extensions_chirho
                        .contains(&"OverloadedStrings".to_string())
                {
                    // OverloadedStrings: string literals become `fromString "lit"`,
                    // IsString's method, never a binding of the program's own (row 660).
                    let from_string_id_chirho = self.resolve_syntax_method_chirho("fromString");
                    self.record_literal_occurrence_chirho(from_string_id_chirho, lit_chirho);
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(from_string_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::LitChirho(core_lit_chirho)),
                    }
                } else {
                    CoreExprChirho::LitChirho(core_lit_chirho)
                }
            }

            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                let desugared_fun_chirho = self.desugar_expr_chirho(fun_chirho);
                let desugared_arg_chirho = self.desugar_expr_chirho(arg_chirho);
                // If applying to a constructor, accumulate into ConAppChirho
                match desugared_fun_chirho {
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho,
                        mut args_chirho,
                    } => {
                        args_chirho.push(desugared_arg_chirho);
                        self.enforce_strict_fields_chirho(con_name_chirho, args_chirho)
                    }
                    // Detect prefix application of binary primops:
                    // App(App(Var("div"), a), b) → PrimOp("div#", [a, b])
                    CoreExprChirho::AppChirho {
                        fun_chirho: ref inner_fun_chirho,
                        arg_chirho: ref first_arg_chirho,
                    } => {
                        // A `div` this module owns keeps its own binding (row 614).
                        if let CoreExprChirho::VarChirho(id_chirho) = inner_fun_chirho.as_ref()
                            && let Some(name_chirho) = self.names_chirho.get(id_chirho)
                            && !self.is_owned_binding_id_chirho(name_chirho, *id_chirho)
                            && let Some(primop_chirho) =
                                prefix_binary_primop_name_chirho(name_chirho)
                        {
                            return CoreExprChirho::PrimOpChirho {
                                name_chirho: primop_chirho.to_string(),
                                args_chirho: vec![
                                    first_arg_chirho.as_ref().clone(),
                                    desugared_arg_chirho,
                                ],
                            };
                        }
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(desugared_fun_chirho),
                            arg_chirho: Box::new(desugared_arg_chirho),
                        }
                    }
                    _ => CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(desugared_fun_chirho),
                        arg_chirho: Box::new(desugared_arg_chirho),
                    },
                }
            }

            ExprChirho::LamChirho {
                pats_chirho,
                body_chirho,
                ..
            } => {
                if let Some(lazy_tuple_chirho) =
                    self.try_desugar_lazy_tuple_lambda_chirho(pats_chirho, body_chirho)
                {
                    return lazy_tuple_chirho;
                }

                // A lambda is one row over its parameters: each pattern is
                // matched left to right, and a mismatch is GHC's lambda failure.
                // workflow: language-features-chirho/pattern-matching-chirho
                let param_binders_chirho: Vec<BinderChirho> = pats_chirho
                    .iter()
                    .enumerate()
                    .map(|(i_chirho, pat_chirho)| {
                        let name_chirho = extract_var_name_from_pat_chirho(pat_chirho)
                            .unwrap_or_else(|| format!("_lam{i_chirho}"));
                        self.row_binder_chirho(&name_chirho)
                    })
                    .collect();
                let param_ids_chirho: Vec<CoreIdChirho> = param_binders_chirho
                    .iter()
                    .map(|binder_chirho| binder_chirho.id_chirho)
                    .collect();
                let row_chirho = MatchRowChirho {
                    pats_chirho: pats_chirho.iter().collect(),
                    body_chirho: RowBodyChirho::Expr(body_chirho),
                    where_binds_chirho: &[],
                };
                let failure_chirho = self.case_failure_chirho("Non-exhaustive patterns in lambda");
                let mut result_chirho =
                    self.compile_rows_chirho(&param_ids_chirho, &[row_chirho], failure_chirho);
                for binder_chirho in param_binders_chirho.into_iter().rev() {
                    result_chirho = CoreExprChirho::LamChirho {
                        binder_chirho,
                        body_chirho: Box::new(result_chirho),
                    };
                }
                result_chirho
            }

            // workflow: language-features-chirho/rank-n-visible-type-application-chirho
            ExprChirho::TypeLamChirho {
                binder_chirho,
                body_chirho,
                ..
            } => CoreExprChirho::TyLamChirho {
                ty_var_chirho: binder_chirho.text_chirho().to_string(),
                body_chirho: Box::new(self.desugar_expr_chirho(body_chirho)),
            },

            // A multi-way if is guarded arms with nowhere to fall to.
            // workflow: language-features-chirho/pattern-matching-chirho
            ExprChirho::MultiWayIfChirho { arms_chirho, .. } => {
                let failure_chirho =
                    self.case_failure_chirho("Non-exhaustive guards in multi-way if");
                self.desugar_guards_chirho(arms_chirho, Some(&failure_chirho))
            }

            ExprChirho::IfChirho {
                cond_chirho,
                then_chirho,
                else_chirho,
                ..
            } => {
                // if c then t else e → case c of { True -> t; False -> e }
                let cond_core_chirho = self.desugar_expr_chirho(cond_chirho);
                let then_core_chirho = self.desugar_expr_chirho(then_chirho);
                let else_core_chirho = self.desugar_expr_chirho(else_chirho);
                let wild_chirho = self.fresh_binder_chirho(
                    "wild",
                    TyChirho::bool_chirho(),
                    SpanChirho::DUMMY_CHIRHO,
                );
                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(cond_core_chirho),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                    ),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: then_core_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("False".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: else_core_chirho,
                        },
                    ],
                }
            }

            ExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => self.desugar_let_around_chirho(binds_chirho, |ctx_chirho| {
                ctx_chirho.desugar_expr_chirho(body_chirho)
            }),

            ExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                let scrut_chirho = self.desugar_expr_chirho(scrutinee_chirho);
                self.desugar_case_expr_with_scrutinee_chirho(scrut_chirho, alts_chirho)
            }

            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => {
                if elements_chirho.is_empty() {
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: "$tuple0".to_string(),
                        args_chirho: vec![],
                    }
                } else {
                    let arity_chirho = elements_chirho.len();
                    let con_args_chirho: Vec<CoreExprChirho> = elements_chirho
                        .iter()
                        .map(|elem_chirho| self.desugar_expr_chirho(elem_chirho))
                        .collect();
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: format!("$tuple{}", arity_chirho),
                        args_chirho: con_args_chirho,
                    }
                }
            }

            ExprChirho::ListChirho {
                elements_chirho, ..
            } => {
                // Desugar [a, b, c] → a : b : c : []
                let mut result_chirho = CoreExprChirho::ConAppChirho {
                    con_name_chirho: "[]".to_string(),
                    args_chirho: vec![],
                };
                for elem_chirho in elements_chirho.iter().rev() {
                    let elem_core_chirho = self.desugar_expr_chirho(elem_chirho);
                    result_chirho = CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![elem_core_chirho, result_chirho],
                    };
                }
                // OverloadedLists: list literals become `fromList [a, b, c]`,
                // IsList's method, never a binding of the program's own (row 660).
                if self
                    .extensions_chirho
                    .contains(&"OverloadedLists".to_string())
                {
                    let from_list_id_chirho = self.resolve_syntax_method_chirho("fromList");
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(from_list_id_chirho)),
                        arg_chirho: Box::new(result_chirho),
                    }
                } else {
                    result_chirho
                }
            }

            ExprChirho::NegChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                // Parser row 592 preserves the direct source magnitude `2^63` as
                // `Neg(Lit(i64::MIN))`: keep its `fromInteger` evidence, but do not
                // execute `negate#` on the already-negative machine-word marker.
                // workflow: compiler-pipeline-chirho/operator-fixity-chirho
                if matches!(
                    inner_chirho.as_ref(),
                    ExprChirho::LitChirho(LitChirho::IntChirho(i64::MIN, ..))
                ) {
                    return self.desugar_expr_chirho(inner_chirho);
                }
                // -x → negate# x (Int) or negateFloat# x (Double)
                let desugared_chirho = self.desugar_expr_chirho(inner_chirho);
                let primop_chirho = if is_float_core_chirho(&desugared_chirho) {
                    "negateFloat#"
                } else {
                    "negate#"
                };
                CoreExprChirho::PrimOpChirho {
                    name_chirho: primop_chirho.to_string(),
                    args_chirho: vec![desugared_chirho],
                }
            }

            ExprChirho::InfixChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                span_chirho,
            } => {
                let op_name_chirho = op_chirho.text_chirho();
                let left_core_chirho = self.desugar_expr_chirho(left_chirho);
                let right_core_chirho = self.desugar_expr_chirho(right_chirho);

                // An operator this module owns, at top level or locally, takes its own
                // binding ahead of every Prelude shortcut below (row 614).
                if self.names_owned_binding_chirho(op_chirho) {
                    return self.written_operator_application_chirho(
                        op_chirho,
                        left_core_chirho,
                        right_core_chirho,
                    );
                }

                // ($) is function application: f $ x = f x
                if op_name_chirho == "$" {
                    return CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(left_core_chirho),
                        arg_chirho: Box::new(right_core_chirho),
                    };
                }

                // ($!) is strict application: f $! x = seq x (f x)
                // Desugars to: case x of { _ -> f x }
                if op_name_chirho == "$!" {
                    let wild_chirho = self.fresh_binder_chirho(
                        "_wild",
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    return CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(right_core_chirho.clone()),
                        bind_chirho: wild_chirho,
                        result_ty_chirho: TyChirho::VarChirho(
                            haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho + 1,
                            ),
                        ),
                        alts_chirho: vec![crate::expr_chirho::CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(left_core_chirho),
                                arg_chirho: Box::new(right_core_chirho),
                            },
                        }],
                    };
                }

                // ($!!) is deep strict application: f $!! x = deepseq x (f x)
                // Desugars to: case x of { _ -> f x }  (same as $! at Core level,
                // since full NF evaluation requires NFData dict which the dict pass handles)
                if op_name_chirho == "$!!" {
                    let wild_chirho = self.fresh_binder_chirho(
                        "_wild",
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        SpanChirho::DUMMY_CHIRHO,
                    );
                    return CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(right_core_chirho.clone()),
                        bind_chirho: wild_chirho,
                        result_ty_chirho: TyChirho::VarChirho(
                            haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho + 1,
                            ),
                        ),
                        alts_chirho: vec![crate::expr_chirho::CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(left_core_chirho),
                                arg_chirho: Box::new(right_core_chirho),
                            },
                        }],
                    };
                }

                // (.) is function composition: (f . g) x = f (g x)
                // At the expression level, f . g desugars to \x -> f (g x)
                if op_name_chirho == "." {
                    let x_id_chirho = self.fresh_id_chirho("x");
                    let x_binder_chirho = BinderChirho {
                        id_chirho: x_id_chirho,
                        name_chirho: "x".to_string(),
                        ty_chirho: TyChirho::VarChirho(
                            haskelujah_typing_chirho::ty_chirho::TyVarChirho(self.next_id_chirho),
                        ),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    };
                    return CoreExprChirho::LamChirho {
                        binder_chirho: x_binder_chirho,
                        body_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(left_core_chirho),
                            arg_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(right_core_chirho),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_id_chirho)),
                            }),
                        }),
                    };
                }

                // (>>) is monadic then: a >> b → (>>) a b as a Var app so the
                // dict pass dispatches it per-type ($prim_Monad_>>_<Head>);
                // unresolved names still lower to ThenIOChirho at STG (INV-001).
                // workflow: monadic-dispatch-chirho
                if op_name_chirho == ">>" {
                    let then_id_chirho = self.resolve_var_chirho(">>");
                    self.record_occurrence_provenance_chirho(
                        then_id_chirho,
                        op_chirho.origin_chirho(),
                        OccurrenceRoleChirho::Reference,
                    );
                    return CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(then_id_chirho)),
                            arg_chirho: Box::new(left_core_chirho),
                        }),
                        arg_chirho: Box::new(right_core_chirho),
                    };
                }

                // (>>=) is monadic bind: a >>= f → (>>=) a f as a Var app;
                // same dispatch/fallback contract as (>>) above.
                // workflow: monadic-dispatch-chirho
                if op_name_chirho == ">>=" {
                    let bind_id_chirho = self.resolve_var_chirho(">>=");
                    self.record_occurrence_provenance_chirho(
                        bind_id_chirho,
                        op_chirho.origin_chirho(),
                        OccurrenceRoleChirho::Reference,
                    );
                    return CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(bind_id_chirho)),
                            arg_chirho: Box::new(left_core_chirho),
                        }),
                        arg_chirho: Box::new(right_core_chirho),
                    };
                }

                // Constructor operators build data; registered names also cover backtick constructors.
                if op_name_chirho.starts_with(':')
                    || self.con_arities_chirho.contains_key(op_name_chirho)
                {
                    return self.enforce_strict_fields_chirho(
                        op_name_chirho.to_string(),
                        vec![left_core_chirho, right_core_chirho],
                    );
                }

                // (&&) short-circuit: a && b → case a of { True → b; False → False }
                if op_name_chirho == "&&" {
                    let wild_chirho =
                        self.fresh_binder_chirho("$wild", TyChirho::bool_chirho(), *span_chirho);
                    return CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(left_core_chirho),
                        bind_chirho: wild_chirho,
                        result_ty_chirho: TyChirho::bool_chirho(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("True".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: right_core_chirho,
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("False".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "False".to_string(),
                                    args_chirho: vec![],
                                },
                            },
                        ],
                    };
                }

                // (||) short-circuit: a || b → case a of { True → True; False → b }
                if op_name_chirho == "||" {
                    let wild_chirho =
                        self.fresh_binder_chirho("$wild", TyChirho::bool_chirho(), *span_chirho);
                    return CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(left_core_chirho),
                        bind_chirho: wild_chirho,
                        result_ty_chirho: TyChirho::bool_chirho(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("True".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "True".to_string(),
                                    args_chirho: vec![],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("False".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: right_core_chirho,
                            },
                        ],
                    };
                }

                // (**) is always floating-point exponentiation
                if op_name_chirho == "**" {
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: "**#".to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // (^) is integer exponentiation, or GHC.Real's repeated squaring at
                // Double (`^.#`, row 570) when the base is a float literal; never `**#`'s
                // `pow`, whose last bits differ. Any other Double base takes `^.#` in the
                // dictionary pass once its key is known.
                // workflow: language-features-chirho/dictionary-evidence-chirho
                if op_name_chirho == "^" {
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: if is_float_core_chirho(&left_core_chirho)
                            || is_float_core_chirho(&right_core_chirho)
                        {
                            "^.#"
                        } else {
                            "^#"
                        }
                        .to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // Float-aware Num dispatch: when at least one operand is
                // a float literal, use the Double primop instead of the
                // default Int one.  This covers `1.5 + 2.5` etc.
                if matches!(op_name_chirho, "+" | "-" | "*" | "/")
                    && (is_float_core_chirho(&left_core_chirho)
                        || is_float_core_chirho(&right_core_chirho))
                {
                    let float_primop_chirho = match op_name_chirho {
                        "+" => "+.#",
                        "-" => "-.#",
                        "*" => "*.#",
                        "/" => "/.#",
                        _ => unreachable!(),
                    };
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: float_primop_chirho.to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // Check if this is a known primitive operator, unless the written
                // operator takes its occurrence's evidence (row 610).
                if !self.takes_operator_evidence_chirho(op_chirho)
                    && let Some(primop_name_chirho) = builtin_primop_name_chirho(op_name_chirho)
                {
                    return CoreExprChirho::PrimOpChirho {
                        name_chirho: primop_name_chirho.to_string(),
                        args_chirho: vec![left_core_chirho, right_core_chirho],
                    };
                }

                // Otherwise: a `op` b → (op a) b
                self.written_operator_application_chirho(
                    op_chirho,
                    left_core_chirho,
                    right_core_chirho,
                )
            }

            ExprChirho::LeftSectionChirho {
                op_chirho,
                arg_chirho,
                ..
            } => {
                // (op e) → \y -> y `op` e → \y -> op y e
                // For primop operators, emit PrimOp in the lambda body
                // so they work without dict-pass bindings.
                let arg_core_chirho = self.desugar_expr_chirho(arg_chirho);
                let y_binder_chirho = self.fresh_binder_chirho(
                    "_sec",
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let y_id_chirho = y_binder_chirho.id_chirho;
                let body_chirho = self.build_section_body_chirho(
                    op_chirho,
                    CoreExprChirho::VarChirho(y_id_chirho),
                    arg_core_chirho,
                );
                CoreExprChirho::LamChirho {
                    binder_chirho: y_binder_chirho,
                    body_chirho: Box::new(body_chirho),
                }
            }

            ExprChirho::RightSectionChirho {
                arg_chirho,
                op_chirho,
                ..
            } => {
                // (e op) → \y -> e `op` y → \y -> op e y
                let arg_core_chirho = self.desugar_expr_chirho(arg_chirho);
                let y_binder_chirho = self.fresh_binder_chirho(
                    "_sec",
                    TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                        self.next_id_chirho,
                    )),
                    SpanChirho::DUMMY_CHIRHO,
                );
                let y_id_chirho = y_binder_chirho.id_chirho;
                let body_chirho = self.build_section_body_chirho(
                    op_chirho,
                    arg_core_chirho,
                    CoreExprChirho::VarChirho(y_id_chirho),
                );
                CoreExprChirho::LamChirho {
                    binder_chirho: y_binder_chirho,
                    body_chirho: Box::new(body_chirho),
                }
            }

            ExprChirho::DoChirho {
                qualifier_chirho,
                stmts_chirho,
                ..
            } => {
                // Desugar do notation:
                //   do { e }              → e
                //   do { e; stmts }       → e >> do { stmts }
                //   do { p <- e; stmts }  → e >>= \p -> do { stmts }
                //   do { let binds; stmts } → let binds in do { stmts }
                self.desugar_do_chirho(stmts_chirho, qualifier_chirho.as_deref())
            }

            ExprChirho::ArithSeqChirho {
                from_chirho,
                then_chirho,
                to_chirho,
                span_chirho,
            } => {
                // Desugar to calls to lazy Prelude functions (not primops)
                // so infinite lists like [1..] are properly lazy.
                //
                // [from ..]       → enumFrom from
                // [from,then ..]  → enumFromThen from then
                // [from .. to]    → enumFromTo from to
                // [from,then..to] → enumFromThenTo from then to
                let from_core_chirho = self.desugar_expr_chirho(from_chirho);
                match (then_chirho, to_chirho) {
                    (None, None) => {
                        let fun_id_chirho =
                            self.resolve_range_method_chirho("enumFrom", *span_chirho);
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_id_chirho)),
                            arg_chirho: Box::new(from_core_chirho),
                        }
                    }
                    (Some(then_e_chirho), None) => {
                        let then_core_chirho = self.desugar_expr_chirho(then_e_chirho);
                        let fun_id_chirho =
                            self.resolve_range_method_chirho("enumFromThen", *span_chirho);
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_id_chirho)),
                                arg_chirho: Box::new(from_core_chirho),
                            }),
                            arg_chirho: Box::new(then_core_chirho),
                        }
                    }
                    (None, Some(to_e_chirho)) => {
                        let to_core_chirho = self.desugar_expr_chirho(to_e_chirho);
                        let fun_id_chirho =
                            self.resolve_range_method_chirho("enumFromTo", *span_chirho);
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_id_chirho)),
                                arg_chirho: Box::new(from_core_chirho),
                            }),
                            arg_chirho: Box::new(to_core_chirho),
                        }
                    }
                    (Some(then_e_chirho), Some(to_e_chirho)) => {
                        let then_core_chirho = self.desugar_expr_chirho(then_e_chirho);
                        let to_core_chirho = self.desugar_expr_chirho(to_e_chirho);
                        let fun_id_chirho =
                            self.resolve_range_method_chirho("enumFromThenTo", *span_chirho);
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::VarChirho(fun_id_chirho)),
                                    arg_chirho: Box::new(from_core_chirho),
                                }),
                                arg_chirho: Box::new(then_core_chirho),
                            }),
                            arg_chirho: Box::new(to_core_chirho),
                        }
                    }
                }
            }

            ExprChirho::ParenChirho { inner_chirho, .. } => self.desugar_expr_chirho(inner_chirho),

            ExprChirho::AnnChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
                ..
            } => {
                let inner_core_chirho = self.desugar_expr_chirho(inner_chirho);
                let type_key_chirho = Self::type_key_from_ast_chirho(ty_chirho);
                if type_key_chirho == "_" {
                    inner_core_chirho
                } else {
                    CoreExprChirho::TyAppChirho {
                        expr_chirho: Box::new(inner_core_chirho),
                        ty_chirho: TyChirho::ConChirho(type_key_chirho),
                    }
                }
            }

            ExprChirho::ListCompChirho {
                body_chirho,
                quals_chirho,
                parallel_quals_chirho,
                span_chirho,
            } => {
                if parallel_quals_chirho.is_empty() {
                    self.desugar_list_comp_chirho(body_chirho, quals_chirho)
                } else {
                    self.desugar_parallel_list_comp_chirho(
                        body_chirho,
                        parallel_quals_chirho,
                        *span_chirho,
                    )
                }
            }

            ExprChirho::RecordConChirho {
                con_chirho,
                fields_chirho,
                has_wildcard_chirho,
                ..
            } => {
                // Con { f1 = e1, f2 = e2 } → ConApp "Con" [e1, e2]
                // RecordWildCards: Con { f1 = e1, .. } → fills missing fields from scope
                let con_name_chirho = con_chirho.text_chirho().to_string();
                let con_args_chirho: Vec<CoreExprChirho> = if *has_wildcard_chirho {
                    self.expand_record_wildcard_expr_chirho(&con_name_chirho, fields_chirho)
                } else {
                    self.record_construction_args_chirho(&con_name_chirho, fields_chirho)
                };
                CoreExprChirho::ConAppChirho {
                    con_name_chirho,
                    args_chirho: con_args_chirho,
                }
            }

            ExprChirho::RecordUpdateChirho {
                expr_chirho: base_chirho,
                fields_chirho,
                ..
            } => {
                // record { f1 = e1 } → setField_f1 e1 record
                // Simplified: for now, produce a chain of setter applications.
                // A real implementation needs the data type layout.
                let base_core_chirho = self.desugar_expr_chirho(base_chirho);
                let mut result_chirho = base_core_chirho;
                for field_chirho in fields_chirho {
                    let setter_name_chirho =
                        format!("$setField_{}", field_chirho.name_chirho.text_chirho());
                    let setter_id_chirho = self.resolve_var_chirho(&setter_name_chirho);
                    let val_chirho = self.desugar_expr_chirho(&field_chirho.value_chirho);
                    result_chirho = CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(setter_id_chirho)),
                            arg_chirho: Box::new(val_chirho),
                        }),
                        arg_chirho: Box::new(result_chirho),
                    };
                }
                result_chirho
            }

            // Type application — erased during desugaring, just desugar the inner expr.
            ExprChirho::TypeAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.desugar_expr_chirho(inner_chirho),

            // TH splice/quote expressions — should have been evaluated before desugaring.
            // Return an error literal instead of panicking so the pipeline doesn't crash.
            ExprChirho::SpliceChirho { .. } | ExprChirho::TypedSpliceChirho { .. } => {
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
            }
            ExprChirho::QuoteExprChirho { .. }
            | ExprChirho::QuoteDeclChirho { .. }
            | ExprChirho::QuoteTypeChirho { .. }
            | ExprChirho::QuotePatChirho { .. } => {
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))
            }
        }
    }

    fn desugar_lit_chirho(&self, lit_chirho: &LitChirho) -> CoreLitChirho {
        match lit_chirho {
            LitChirho::IntChirho(v_chirho, _, _) => CoreLitChirho::IntChirho(*v_chirho),
            LitChirho::FloatChirho(v_chirho, _, _) => CoreLitChirho::FloatChirho(*v_chirho),
            LitChirho::CharChirho(v_chirho, _, _) => CoreLitChirho::CharChirho(*v_chirho),
            LitChirho::StringChirho(v_chirho, _, _) => {
                CoreLitChirho::StringChirho(v_chirho.clone())
            }
        }
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
