// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Module-level AST

use rhasky_span_chirho::SpanChirho;

use crate::decl_chirho::DeclChirho;
use crate::name_chirho::NameChirho;

/// A complete Haskell module.
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleChirho {
    /// Module name (e.g. `Data.List`, `Main`).
    pub name_chirho: NameChirho,
    /// Export list (None = export everything).
    pub exports_chirho: Option<Vec<ExportSpecChirho>>,
    /// Import declarations.
    pub imports_chirho: Vec<ImportDeclChirho>,
    /// Top-level declarations.
    pub decls_chirho: Vec<DeclChirho>,
    /// Span covering the entire module.
    pub span_chirho: SpanChirho,
}

/// An export specification.
#[derive(Debug, Clone, PartialEq)]
pub enum ExportSpecChirho {
    /// Export a variable or type (`foo`).
    VarChirho(NameChirho),
    /// Export a type/class with some or all constructors/methods
    /// (`Foo(..)`, `Foo(A, B)`).
    TyConChirho {
        name_chirho: NameChirho,
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
        name_chirho: NameChirho,
        members_chirho: ExportMembersChirho,
    },
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
            span_chirho: SpanChirho::DUMMY_CHIRHO,
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
}
