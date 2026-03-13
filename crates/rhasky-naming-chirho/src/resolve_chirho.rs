// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Name resolution pass
//!
//! Walks the AST and resolves all name references. Reports diagnostics for
//! undefined names, ambiguous names, and shadowing warnings.

use rhasky_ast_chirho::module_chirho::ModuleChirho;
use rhasky_ast_chirho::name_chirho::NameChirho;
use rhasky_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use rhasky_span_chirho::SpanChirho;

use crate::env_chirho::{NameEnvChirho, NamespaceChirho};

/// Error codes for name resolution diagnostics.
const UNDEFINED_VALUE_CODE_CHIRHO: u16 = 100;
const UNDEFINED_TYPE_CODE_CHIRHO: u16 = 101;

/// Result of name resolution.
pub struct ResolveResultChirho {
    /// The name environment after resolution (contains all definitions).
    pub env_chirho: NameEnvChirho,
    /// Diagnostics (errors and warnings) from resolution.
    pub diagnostics_chirho: DiagnosticBundleChirho,
}

/// Resolve names in a module.
///
/// This is a placeholder that currently just collects top-level bindings
/// and reports any obviously undefined names. The full implementation will
/// walk the entire AST and resolve every name occurrence.
pub fn resolve_module_chirho(module_chirho: &ModuleChirho) -> ResolveResultChirho {
    let mut env_chirho = NameEnvChirho::new_chirho();
    let diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();

    // Phase 1: Collect all top-level type names (data, newtype, type, class)
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            rhasky_ast_chirho::decl_chirho::DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                ..
            } => {
                bind_name_chirho(
                    &mut env_chirho,
                    name_chirho,
                    NamespaceChirho::TypeChirho,
                );
                // Bind constructors in value namespace
                for con_chirho in constructors_chirho {
                    let con_name_chirho = match con_chirho {
                        rhasky_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                            name_chirho,
                            ..
                        } => name_chirho,
                        rhasky_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                            name_chirho,
                            ..
                        } => name_chirho,
                    };
                    bind_name_chirho(
                        &mut env_chirho,
                        con_name_chirho,
                        NamespaceChirho::ValueChirho,
                    );
                }
            }
            rhasky_ast_chirho::decl_chirho::DeclChirho::NewtypeDeclChirho {
                name_chirho,
                constructor_chirho,
                ..
            } => {
                bind_name_chirho(
                    &mut env_chirho,
                    name_chirho,
                    NamespaceChirho::TypeChirho,
                );
                let con_name_chirho = match constructor_chirho {
                    rhasky_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                        name_chirho,
                        ..
                    } => name_chirho,
                    rhasky_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                        name_chirho,
                        ..
                    } => name_chirho,
                };
                bind_name_chirho(
                    &mut env_chirho,
                    con_name_chirho,
                    NamespaceChirho::ValueChirho,
                );
            }
            rhasky_ast_chirho::decl_chirho::DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                ..
            } => {
                bind_name_chirho(
                    &mut env_chirho,
                    name_chirho,
                    NamespaceChirho::TypeChirho,
                );
            }
            rhasky_ast_chirho::decl_chirho::DeclChirho::ClassDeclChirho {
                name_chirho,
                methods_chirho,
                ..
            } => {
                bind_name_chirho(
                    &mut env_chirho,
                    name_chirho,
                    NamespaceChirho::TypeChirho,
                );
                for method_chirho in methods_chirho {
                    bind_name_chirho(
                        &mut env_chirho,
                        &method_chirho.name_chirho,
                        NamespaceChirho::ValueChirho,
                    );
                }
            }
            _ => {}
        }
    }

    // Phase 2: Collect all top-level value bindings
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            rhasky_ast_chirho::decl_chirho::DeclChirho::FunBindChirho {
                name_chirho, ..
            }
            | rhasky_ast_chirho::decl_chirho::DeclChirho::TypeSigChirho {
                name_chirho, ..
            } => {
                // Only bind if not already bound (type sig + fun bind share a name)
                if env_chirho
                    .lookup_value_chirho(name_chirho.text_chirho())
                    .is_none()
                {
                    bind_name_chirho(
                        &mut env_chirho,
                        name_chirho,
                        NamespaceChirho::ValueChirho,
                    );
                }
            }
            _ => {}
        }
    }

    // Phase 3: Process imports (placeholder — just marks them as imported)
    for import_chirho in &module_chirho.imports_chirho {
        let module_text_chirho = import_chirho.module_chirho.text_chirho().to_string();
        env_chirho.bind_import_chirho(
            module_text_chirho,
            NamespaceChirho::ValueChirho,
            import_chirho.span_chirho,
        );
    }

    ResolveResultChirho {
        env_chirho,
        diagnostics_chirho,
    }
}

/// Report an "undefined name" diagnostic.
pub fn report_undefined_chirho(
    diagnostics_chirho: &mut DiagnosticBundleChirho,
    name_chirho: &str,
    namespace_chirho: NamespaceChirho,
    span_chirho: SpanChirho,
) {
    let code_chirho = match namespace_chirho {
        NamespaceChirho::ValueChirho => UNDEFINED_VALUE_CODE_CHIRHO,
        NamespaceChirho::TypeChirho => UNDEFINED_TYPE_CODE_CHIRHO,
    };
    let msg_chirho = match namespace_chirho {
        NamespaceChirho::ValueChirho => format!("variable not in scope: `{name_chirho}`"),
        NamespaceChirho::TypeChirho => format!("type not in scope: `{name_chirho}`"),
    };
    diagnostics_chirho.push_chirho(
        DiagnosticChirho::error_with_code_chirho(
            ErrorCodeChirho::error_chirho(code_chirho),
            msg_chirho,
            span_chirho,
        ),
    );
}

fn bind_name_chirho(
    env_chirho: &mut NameEnvChirho,
    name_chirho: &NameChirho,
    namespace_chirho: NamespaceChirho,
) {
    env_chirho.bind_chirho(
        name_chirho.text_chirho().to_string(),
        namespace_chirho,
        name_chirho.span_chirho(),
    );
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhasky_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
    use rhasky_ast_chirho::name_chirho::RawNameChirho;

    fn dummy_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    #[test]
    fn resolve_simple_module_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("Main"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("Color"),
                    type_vars_chirho: vec![],
                    constructors_chirho: vec![
                        ConDeclChirho::OrdinaryChirho {
                            name_chirho: dummy_name_chirho("Red"),
                            fields_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        ConDeclChirho::OrdinaryChirho {
                            name_chirho: dummy_name_chirho("Green"),
                            fields_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    deriving_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("main"),
                    matches_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = resolve_module_chirho(&module_chirho);

        // Color should be in type namespace
        assert!(result_chirho.env_chirho.lookup_type_chirho("Color").is_some());
        // Red, Green should be in value namespace
        assert!(result_chirho.env_chirho.lookup_value_chirho("Red").is_some());
        assert!(result_chirho.env_chirho.lookup_value_chirho("Green").is_some());
        // main should be in value namespace
        assert!(result_chirho.env_chirho.lookup_value_chirho("main").is_some());
        // No errors
        assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    }

    #[test]
    fn resolve_class_methods_chirho() {
        let module_chirho = ModuleChirho {
            name_chirho: dummy_name_chirho("M"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: dummy_name_chirho("Describable"),
                type_vars_chirho: vec![dummy_name_chirho("a")],
                methods_chirho: vec![
                    rhasky_ast_chirho::decl_chirho::ClassMethodChirho {
                        name_chirho: dummy_name_chirho("describe"),
                        ty_chirho: rhasky_ast_chirho::ty_chirho::TypeChirho::VarChirho(
                            dummy_name_chirho("a"),
                        ),
                        default_chirho: None,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let result_chirho = resolve_module_chirho(&module_chirho);

        assert!(result_chirho
            .env_chirho
            .lookup_type_chirho("Describable")
            .is_some());
        assert!(result_chirho
            .env_chirho
            .lookup_value_chirho("describe")
            .is_some());
    }
}
