// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Module-level AST

use haskelujah_span_chirho::SpanChirho;

use crate::decl_chirho::DeclChirho;
use crate::name_chirho::NameChirho;
use crate::provenance_chirho::OriginSupplyChirho;
use crate::ty_chirho::TypeChirho;

/// Inline pragma annotation for a binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlinePragmaChirho {
    /// `{-# INLINE f #-}` — always inline this binding.
    InlineChirho,
    /// `{-# NOINLINE f #-}` — never inline this binding.
    NoInlineChirho,
    /// `{-# INLINABLE f #-}` — make available for inlining at call sites.
    InlinableChirho,
}

/// A complete Haskell module.
///
/// Deliberately not `Clone`: the module owns its origin supply, and a copy of
/// the module would be a second supply able to mint the origins the first one
/// mints. Nothing in the workspace clones a module (measured 2026-09-22 by
/// removing the derive and checking every target).
#[derive(Debug, PartialEq)]
pub struct ModuleChirho {
    /// Module name (e.g. `Data.List`, `Main`).
    pub name_chirho: NameChirho,
    /// Export list (None = export everything).
    pub exports_chirho: Option<Vec<ExportSpecChirho>>,
    /// Import declarations.
    pub imports_chirho: Vec<ImportDeclChirho>,
    /// Top-level declarations.
    pub decls_chirho: Vec<DeclChirho>,
    /// LANGUAGE extensions enabled via `{-# LANGUAGE ... #-}` pragmas.
    pub extensions_chirho: Vec<String>,
    /// INLINE/NOINLINE/INLINABLE pragmas: maps binding name → pragma.
    pub inline_pragmas_chirho: std::collections::HashMap<String, InlinePragmaChirho>,
    /// SPECIALIZE pragmas: maps binding name → list of specialization type strings.
    /// Each string is the raw type text from `{-# SPECIALIZE f :: Type #-}`.
    pub specialize_pragmas_chirho: std::collections::HashMap<String, Vec<String>>,
    /// Foreign export declarations extracted from `foreign export ccall ...` decls.
    pub foreign_exports_chirho: Vec<(String, String, String)>,
    /// DerivingVia entries: `(type_name, class_name, via_type)`.
    /// Populated from `deriving (Class) via ViaType` clauses on data/newtype decls.
    pub deriving_via_chirho: Vec<(NameChirho, NameChirho, TypeChirho)>,
    /// Span covering the entire module.
    pub span_chirho: SpanChirho,
    /// Mints occurrence origins for this module. Lowering creates it; TH
    /// expansion, deriving and the late GND pass borrow it through the module,
    /// so every producer continues one sequence and none restarts it.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub origin_supply_chirho: OriginSupplyChirho,
}

/// An export specification.
#[derive(Debug, Clone, PartialEq)]
pub enum ExportSpecChirho {
    /// Export a variable or type (`foo`).
    VarChirho(NameChirho),
    /// Export a type/class with some or all constructors/methods
    /// (`Foo(..)`, `Foo(A, B)`).
    TyConChirho {
        /// The exported type or class name.
        name_chirho: NameChirho,
        /// Which constructors or methods are re-exported.
        members_chirho: ExportMembersChirho,
    },
    /// Re-export an entire module (`module Data.List`).
    ModuleChirho(NameChirho),
}

/// Which members of a type/class to export.
#[derive(Debug, Clone, PartialEq)]
pub enum ExportMembersChirho {
    /// Export all constructors/methods (`(..)`).
    AllChirho,
    /// Export specific members.
    SomeChirho(Vec<NameChirho>),
    /// Export no members (just the type name).
    NoneChirho,
}

/// An import declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ImportDeclChirho {
    /// The imported module name.
    pub module_chirho: NameChirho,
    /// Whether this is a qualified import.
    pub qualified_chirho: bool,
    /// Optional alias (`as Alias`).
    pub alias_chirho: Option<NameChirho>,
    /// Import specification (None = import everything).
    pub spec_chirho: Option<ImportSpecChirho>,
    /// Span covering the entire import declaration.
    pub span_chirho: SpanChirho,
}

/// Import specification — either importing or hiding specific items.
#[derive(Debug, Clone, PartialEq)]
pub struct ImportSpecChirho {
    /// Whether this is a hiding spec (`hiding (...)`) or an explicit list.
    pub hiding_chirho: bool,
    /// The items to import or hide.
    pub items_chirho: Vec<ImportItemChirho>,
}

/// A single item in an import list.
#[derive(Debug, Clone, PartialEq)]
pub enum ImportItemChirho {
    /// Import a variable (`foo`).
    VarChirho(NameChirho),
    /// Import a type/class with some or all constructors/methods.
    TyConChirho {
        /// The imported type or class name.
        name_chirho: NameChirho,
        /// Which constructors or methods are imported.
        members_chirho: ExportMembersChirho,
    },
    /// An item lowering cannot represent: a namespace wildcard such as
    /// `data ..` or `type ..`, or a `pattern P` item. It names nothing, so it
    /// imports and hides nothing, as when lowering dropped it. It is kept so
    /// the list it sits in can be read as incomplete.
    /// workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
    UnrepresentedChirho(SpanChirho),
}

impl ImportItemChirho {
    /// Whether lowering represented this item. A new item form must be
    /// classified here before an explicit list holding it can be trusted.
    /// workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
    pub fn is_represented_chirho(&self) -> bool {
        match self {
            Self::VarChirho(_) | Self::TyConChirho { .. } => true,
            Self::UnrepresentedChirho(_) => false,
        }
    }
}

/// How an import declaration's item list restricts it, read from the syntax
/// alone. Which names an item actually brings depends on the imported
/// module's interface, so that question stays with name resolution.
/// workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImportListModeChirho<'a> {
    /// No item list, or an explicit list holding an item lowering could not
    /// represent. The declaration makes no exclusion claim.
    UnrestrictedChirho,
    /// `hiding (...)`, with every item as written.
    HidingChirho(&'a [ImportItemChirho]),
    /// An explicit list, every item of which is represented.
    ExplicitChirho(&'a [ImportItemChirho]),
}

impl ImportDeclChirho {
    /// The qualifier this import binds its names under: the `as` alias,
    /// otherwise the module name.
    pub fn effective_qualifier_chirho(&self) -> String {
        self.alias_chirho
            .as_ref()
            .unwrap_or(&self.module_chirho)
            .full_name_chirho()
    }

    /// How this declaration's item list restricts it.
    pub fn list_mode_chirho(&self) -> ImportListModeChirho<'_> {
        match &self.spec_chirho {
            None => ImportListModeChirho::UnrestrictedChirho,
            Some(spec_chirho) if spec_chirho.hiding_chirho => {
                ImportListModeChirho::HidingChirho(&spec_chirho.items_chirho)
            }
            Some(spec_chirho)
                if spec_chirho
                    .items_chirho
                    .iter()
                    .all(ImportItemChirho::is_represented_chirho) =>
            {
                ImportListModeChirho::ExplicitChirho(&spec_chirho.items_chirho)
            }
            Some(_) => ImportListModeChirho::UnrestrictedChirho,
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::name_chirho::RawNameChirho;

    #[test]
    fn module_basics_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                "Main",
                SpanChirho::DUMMY_CHIRHO,
            )),
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
        };
        assert_eq!(module_chirho.name_chirho.text_chirho(), "Main");
        assert!(module_chirho.exports_chirho.is_none());
    }

    #[test]
    fn import_decl_basics_chirho() {
        let import_chirho = ImportDeclChirho {
            module_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                "Data.List",
                SpanChirho::DUMMY_CHIRHO,
            )),
            qualified_chirho: true,
            alias_chirho: Some(NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                "L",
                SpanChirho::DUMMY_CHIRHO,
            ))),
            spec_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        assert!(import_chirho.qualified_chirho);
        assert_eq!(
            import_chirho.alias_chirho.as_ref().unwrap().text_chirho(),
            "L"
        );
    }

    fn plain_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    /// `import qualified Data.List.NonEmpty as NE` with the given spec.
    fn non_empty_import_chirho(spec_chirho: Option<ImportSpecChirho>) -> ImportDeclChirho {
        ImportDeclChirho {
            module_chirho: plain_name_chirho("Data.List.NonEmpty"),
            qualified_chirho: true,
            alias_chirho: Some(plain_name_chirho("NE")),
            spec_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn items_spec_chirho(
        hiding_chirho: bool,
        items_chirho: Vec<ImportItemChirho>,
    ) -> ImportSpecChirho {
        ImportSpecChirho {
            hiding_chirho,
            items_chirho,
        }
    }

    // Row 536 (#28493): the three syntactic modes, read from the written list.
    #[test]
    fn import_list_modes_follow_the_written_syntax_chirho() {
        let head_chirho = ImportItemChirho::VarChirho(plain_name_chirho("head"));
        let wildcard_chirho = ImportItemChirho::UnrepresentedChirho(SpanChirho::DUMMY_CHIRHO);

        assert_eq!(
            non_empty_import_chirho(None).list_mode_chirho(),
            ImportListModeChirho::UnrestrictedChirho
        );
        let explicit_chirho =
            non_empty_import_chirho(Some(items_spec_chirho(false, vec![head_chirho.clone()])));
        assert_eq!(
            explicit_chirho.list_mode_chirho(),
            ImportListModeChirho::ExplicitChirho(&[head_chirho.clone()])
        );
        // `()` is an explicit list, and every one of its zero items is
        // represented: it supplies nothing.
        let empty_chirho = non_empty_import_chirho(Some(items_spec_chirho(false, vec![])));
        assert_eq!(
            empty_chirho.list_mode_chirho(),
            ImportListModeChirho::ExplicitChirho(&[])
        );
        // One unrepresented item and the explicit list claims nothing.
        let partial_chirho = non_empty_import_chirho(Some(items_spec_chirho(
            false,
            vec![head_chirho.clone(), wildcard_chirho.clone()],
        )));
        assert_eq!(
            partial_chirho.list_mode_chirho(),
            ImportListModeChirho::UnrestrictedChirho
        );
        // `hiding` keeps every written item, the unrepresented one included.
        let hiding_chirho = non_empty_import_chirho(Some(items_spec_chirho(
            true,
            vec![head_chirho.clone(), wildcard_chirho.clone()],
        )));
        assert_eq!(
            hiding_chirho.list_mode_chirho(),
            ImportListModeChirho::HidingChirho(&[head_chirho, wildcard_chirho])
        );
    }

    #[test]
    fn the_effective_qualifier_is_the_alias_or_else_the_module_chirho() {
        assert_eq!(
            non_empty_import_chirho(None).effective_qualifier_chirho(),
            "NE"
        );
        let unaliased_chirho = ImportDeclChirho {
            alias_chirho: None,
            ..non_empty_import_chirho(None)
        };
        assert_eq!(
            unaliased_chirho.effective_qualifier_chirho(),
            "Data.List.NonEmpty"
        );
        // Lowering splits a dotted alias like any qualified name; the whole
        // alias is still the qualifier, as `A.B.x` is written.
        let dotted_chirho = ImportDeclChirho {
            alias_chirho: Some(NameChirho::RawChirho(RawNameChirho::qualified_chirho(
                "A",
                "B",
                SpanChirho::DUMMY_CHIRHO,
            ))),
            ..non_empty_import_chirho(None)
        };
        assert_eq!(dotted_chirho.effective_qualifier_chirho(), "A.B");
    }

    #[test]
    fn only_the_unrepresented_item_is_unrepresented_chirho() {
        assert!(ImportItemChirho::VarChirho(plain_name_chirho("head")).is_represented_chirho());
        assert!(
            ImportItemChirho::TyConChirho {
                name_chirho: plain_name_chirho("NonEmpty"),
                members_chirho: ExportMembersChirho::AllChirho,
            }
            .is_represented_chirho()
        );
        assert!(
            !ImportItemChirho::UnrepresentedChirho(SpanChirho::DUMMY_CHIRHO)
                .is_represented_chirho()
        );
    }
}
