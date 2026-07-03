// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Template Haskell splice expansion
//!
//! This module provides a splice expansion pass that runs after AST lowering
//! but before name resolution. It processes `DeclChirho::SpliceDeclChirho`
//! entries in a module's declaration list and replaces them with generated
//! declarations.
//!
//! Since a full TH evaluator requires compiling and running arbitrary Haskell
//! at compile time, this module provides **built-in splice handlers** for
//! common TH patterns:
//!
//! - `makeLenses` / `makeLenses'` — generate lens accessor functions
//! - `deriveJSON` — placeholder for aeson TH (emits a warning)
//! - Unknown splices — emit a warning and skip
//!
//! The handlers use TH AST types from `haskelujah-th-chirho`, construct
//! `ThDecChirho` values, then convert them to haskelujah AST via
//! `convert_chirho::th_dec_to_ast_chirho`.

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, FieldDeclChirho};
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::lit_chirho::LitChirho;

use haskelujah_th_chirho::convert_chirho::th_dec_to_ast_chirho;
use haskelujah_th_chirho::th_ast_chirho::*;

/// Result of splice expansion for a module.
pub struct SpliceExpansionResultChirho {
    /// The declarations with all splice entries expanded (or removed).
    pub decls_chirho: Vec<DeclChirho>,
    /// Warnings generated during splice expansion (unknown splices, etc.).
    pub warnings_chirho: Vec<String>,
}

/// Expand all `SpliceDeclChirho` entries in a module's declaration list.
///
/// For each splice, the inner expression is inspected to determine which
/// built-in handler to dispatch to. Recognized patterns:
///
/// - `makeLenses ''TypeName` or `makeLenses' ''TypeName`
/// - `deriveJSON defaultOptions ''TypeName`
///
/// Unknown splice expressions produce a warning and are removed.
///
/// The `all_decls_chirho` parameter is the full set of declarations in the
/// module (needed to look up data types referenced by splices).
pub fn expand_splices_chirho(decls_chirho: Vec<DeclChirho>) -> SpliceExpansionResultChirho {
    let mut result_decls_chirho: Vec<DeclChirho> = Vec::new();
    let mut warnings_chirho: Vec<String> = Vec::new();

    for decl_chirho in &decls_chirho {
        match decl_chirho {
            DeclChirho::SpliceDeclChirho {
                expr_chirho,
                span_chirho: _,
            } => {
                // Try to recognize the splice pattern and dispatch to a handler.
                match recognize_splice_chirho(expr_chirho) {
                    SplicePatternChirho::MakeLensesChirho(type_name_chirho) => {
                        // Look up the data type in the current module's declarations.
                        match find_record_type_chirho(&decls_chirho, &type_name_chirho) {
                            Some((data_name_chirho, con_name_chirho, fields_chirho)) => {
                                let generated_chirho = generate_lenses_chirho(
                                    &data_name_chirho,
                                    &con_name_chirho,
                                    &fields_chirho,
                                );
                                for th_dec_chirho in &generated_chirho {
                                    if let Some(ast_dec_chirho) =
                                        th_dec_to_ast_chirho(th_dec_chirho)
                                    {
                                        result_decls_chirho.push(ast_dec_chirho);
                                    }
                                }
                            }
                            None => {
                                warnings_chirho.push(format!(
                                    "makeLenses: data type '{}' not found in module or is not a record type",
                                    type_name_chirho
                                ));
                            }
                        }
                    }
                    SplicePatternChirho::DeriveJsonChirho(type_name_chirho) => {
                        warnings_chirho.push(format!(
                            "deriveJSON: built-in handler not yet implemented for '{}'",
                            type_name_chirho
                        ));
                    }
                    SplicePatternChirho::UnknownChirho(desc_chirho) => {
                        warnings_chirho.push(format!(
                            "Template Haskell splice not recognized, skipping: {}",
                            desc_chirho
                        ));
                    }
                }
            }
            other_chirho => {
                result_decls_chirho.push(other_chirho.clone());
            }
        }
    }

    SpliceExpansionResultChirho {
        decls_chirho: result_decls_chirho,
        warnings_chirho,
    }
}

// ---------------------------------------------------------------------------
// Splice pattern recognition
// ---------------------------------------------------------------------------

/// Recognized splice patterns.
enum SplicePatternChirho {
    /// `makeLenses ''TypeName` or `makeLenses' ''TypeName`.
    MakeLensesChirho(String),
    /// `deriveJSON defaultOptions ''TypeName`.
    DeriveJsonChirho(String),
    /// Unrecognized splice — carries a description string for the warning.
    UnknownChirho(String),
}

/// Inspect a splice expression and classify it as a known pattern or unknown.
///
/// We recognize these shapes in the AST:
///
/// - `App (Var "makeLenses") (Var "''TypeName")` — direct single-arg
/// - `App (Var "makeLenses") (Con "TypeName")` — constructor name
/// - `App (App (Var "makeLenses") ...) (Var "TypeName")` — parser may split
///   `''TypeName` into multiple tokens producing nested applications;
///   the last argument is the type name
/// - `App (App (Var "deriveJSON") (Var "defaultOptions")) (Var "TypeName")`
fn recognize_splice_chirho(expr_chirho: &ExprChirho) -> SplicePatternChirho {
    // Flatten the application spine: `f a b c` → (f, [a, b, c]).
    let (head_chirho, args_chirho) = flatten_app_spine_chirho(expr_chirho);

    if let Some(func_name_chirho) = extract_var_name_chirho(head_chirho) {
        match func_name_chirho.as_str() {
            "makeLenses" | "makeLenses'" => {
                // The type name is the last argument (after stripping
                // any intermediate tick-quote tokens).
                if let Some(type_name_chirho) = extract_type_name_from_args_chirho(&args_chirho) {
                    return SplicePatternChirho::MakeLensesChirho(type_name_chirho);
                }
            }
            "deriveJSON" => {
                // `deriveJSON opts ''Type` → args = [opts, ..., Type]
                if let Some(type_name_chirho) = extract_type_name_from_args_chirho(&args_chirho) {
                    return SplicePatternChirho::DeriveJsonChirho(type_name_chirho);
                }
            }
            _ => {}
        }
    }

    // Second pass: the head itself might be an App with the function deeper
    // (e.g. `App (App (Var "deriveJSON") opts) typeName`).
    // Already handled by flatten_app_spine_chirho above.

    // Fallback: describe the expression for the warning.
    SplicePatternChirho::UnknownChirho(describe_expr_chirho(expr_chirho))
}

/// Flatten an application spine `f a b c` into `(f, [a, b, c])`.
fn flatten_app_spine_chirho(expr_chirho: &ExprChirho) -> (&ExprChirho, Vec<&ExprChirho>) {
    let mut args_chirho: Vec<&ExprChirho> = Vec::new();
    let mut current_chirho = expr_chirho;
    loop {
        match current_chirho {
            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                args_chirho.push(arg_chirho.as_ref());
                current_chirho = fun_chirho.as_ref();
            }
            _ => break,
        }
    }
    args_chirho.reverse();
    (current_chirho, args_chirho)
}

/// Extract the type name from the argument list of a TH splice function call.
///
/// The parser may produce `''TypeName` as:
/// - A single Var/Con named `''TypeName` or `TypeName`
/// - Two tick-character literals followed by a Var/Con for just `TypeName`
///
/// We scan the args from the end, looking for the last identifier that
/// looks like a type name (starts with uppercase, or is prefixed with `''`).
fn extract_type_name_from_args_chirho(args_chirho: &[&ExprChirho]) -> Option<String> {
    // Walk from the last arg backwards looking for a usable name.
    for arg_chirho in args_chirho.iter().rev() {
        if let Some(type_name_chirho) = extract_type_name_arg_chirho(arg_chirho) {
            return Some(type_name_chirho);
        }
    }
    None
}

fn extract_type_name_arg_chirho(arg_chirho: &ExprChirho) -> Option<String> {
    if let ExprChirho::ParenChirho { inner_chirho, .. } = arg_chirho {
        return extract_type_name_arg_chirho(inner_chirho);
    }

    if let Some(name_chirho) =
        extract_var_name_chirho(arg_chirho).or_else(|| extract_con_name_chirho(arg_chirho))
    {
        return normalize_th_type_name_chirho(&name_chirho);
    }

    let ExprChirho::AppChirho {
        fun_chirho,
        arg_chirho: mk_name_arg_chirho,
        ..
    } = arg_chirho
    else {
        return None;
    };

    if !matches!(
        extract_var_name_chirho(fun_chirho.as_ref()).as_deref(),
        Some("mkName")
    ) {
        return None;
    }

    match mk_name_arg_chirho.as_ref() {
        ExprChirho::LitChirho(LitChirho::StringChirho(text_chirho, _)) => {
            normalize_th_type_name_chirho(text_chirho)
        }
        _ => None,
    }
}

fn normalize_th_type_name_chirho(name_chirho: &str) -> Option<String> {
    let stripped_chirho = name_chirho.strip_prefix("''").unwrap_or(name_chirho);
    if stripped_chirho.is_empty() {
        None
    } else {
        Some(stripped_chirho.to_string())
    }
}

/// Extract a variable name from a Var expression.
fn extract_var_name_chirho(expr_chirho: &ExprChirho) -> Option<String> {
    if let ExprChirho::VarChirho(name_chirho) = expr_chirho {
        Some(name_chirho.text_chirho().to_string())
    } else {
        None
    }
}

/// Extract a constructor name from a Con expression.
fn extract_con_name_chirho(expr_chirho: &ExprChirho) -> Option<String> {
    if let ExprChirho::ConChirho(name_chirho) = expr_chirho {
        Some(name_chirho.text_chirho().to_string())
    } else {
        None
    }
}

/// Produce a brief human-readable description of an expression (for warnings).
fn describe_expr_chirho(expr_chirho: &ExprChirho) -> String {
    match expr_chirho {
        ExprChirho::VarChirho(n_chirho) => format!("variable '{}'", n_chirho.text_chirho()),
        ExprChirho::ConChirho(n_chirho) => format!("constructor '{}'", n_chirho.text_chirho()),
        ExprChirho::AppChirho { fun_chirho, .. } => {
            format!("application of {}", describe_expr_chirho(fun_chirho))
        }
        ExprChirho::LitChirho(lit_chirho) => format!("literal {:?}", lit_chirho),
        _ => "unknown expression".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Data type lookup
// ---------------------------------------------------------------------------

/// A record field with its name and type as strings.
struct RecordFieldInfoChirho {
    /// The field name (e.g. `_name`).
    field_name_chirho: String,
    /// The field type as a string (e.g. `String`).
    field_type_chirho: String,
}

/// Look up a record data type in the declaration list by name.
///
/// Returns `(data_type_name, constructor_name, fields)` or `None` if not found
/// or not a record type.
fn find_record_type_chirho(
    decls_chirho: &[DeclChirho],
    type_name_chirho: &str,
) -> Option<(String, String, Vec<RecordFieldInfoChirho>)> {
    for decl_chirho in decls_chirho {
        if let DeclChirho::DataDeclChirho {
            name_chirho,
            constructors_chirho,
            ..
        } = decl_chirho
        {
            if name_chirho.text_chirho() != type_name_chirho {
                continue;
            }
            // Look for a record constructor.
            for con_chirho in constructors_chirho {
                if let ConDeclChirho::RecordChirho {
                    name_chirho: con_name_chirho,
                    fields_chirho,
                    ..
                } = con_chirho
                {
                    let field_infos_chirho: Vec<RecordFieldInfoChirho> = fields_chirho
                        .iter()
                        .flat_map(|fd_chirho: &FieldDeclChirho| {
                            fd_chirho.names_chirho.iter().map(move |fn_chirho| {
                                RecordFieldInfoChirho {
                                    field_name_chirho: fn_chirho.text_chirho().to_string(),
                                    field_type_chirho: type_to_string_chirho(&fd_chirho.ty_chirho),
                                }
                            })
                        })
                        .collect();
                    return Some((
                        name_chirho.text_chirho().to_string(),
                        con_name_chirho.text_chirho().to_string(),
                        field_infos_chirho,
                    ));
                }
            }
        }
    }
    None
}

/// Convert an AST type to a simple string representation (for lens type sigs).
fn type_to_string_chirho(ty_chirho: &haskelujah_ast_chirho::ty_chirho::TypeChirho) -> String {
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;
    match ty_chirho {
        TypeChirho::ConChirho(n_chirho) => n_chirho.text_chirho().to_string(),
        TypeChirho::VarChirho(n_chirho) => n_chirho.text_chirho().to_string(),
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => format!(
            "{} {}",
            type_to_string_chirho(fun_chirho),
            type_to_string_chirho(arg_chirho)
        ),
        TypeChirho::ListChirho { element_chirho, .. } => {
            format!("[{}]", type_to_string_chirho(element_chirho))
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => {
            let inner_chirho: Vec<String> = elements_chirho
                .iter()
                .map(|e_chirho| type_to_string_chirho(e_chirho))
                .collect();
            format!("({})", inner_chirho.join(", "))
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => format!(
            "{} -> {}",
            type_to_string_chirho(arg_chirho),
            type_to_string_chirho(result_chirho)
        ),
        _ => "_".to_string(),
    }
}

// ---------------------------------------------------------------------------
// makeLenses code generation
// ---------------------------------------------------------------------------

/// Generate lens declarations for a record data type.
///
/// For each field whose name starts with `_`, generates:
/// 1. A type signature: `fieldName :: Lens' TypeName FieldType`
/// 2. A function definition implementing the van Laarhoven lens:
///    `fieldName f s = fmap (\x -> s { _fieldName = x }) (f (_fieldName s))`
///
/// Fields that do not start with `_` are skipped (matching the `lens` library
/// convention).
fn generate_lenses_chirho(
    data_name_chirho: &str,
    con_name_chirho: &str,
    fields_chirho: &[RecordFieldInfoChirho],
) -> Vec<ThDecChirho> {
    let mut decs_chirho: Vec<ThDecChirho> = Vec::new();

    for field_chirho in fields_chirho {
        let field_name_chirho = &field_chirho.field_name_chirho;

        // Only generate lenses for fields starting with '_'.
        let lens_name_chirho = match field_name_chirho.strip_prefix('_') {
            Some(rest_chirho) if !rest_chirho.is_empty() => rest_chirho.to_string(),
            _ => continue,
        };

        let field_type_chirho = &field_chirho.field_type_chirho;

        // 1. Type signature: lensName :: Lens' DataType FieldType
        //    Lens' s a = forall f. Functor f => (a -> f a) -> s -> f s
        //    We emit the expanded Lens' type for now.
        let sig_chirho =
            generate_lens_sig_chirho(&lens_name_chirho, data_name_chirho, field_type_chirho);
        decs_chirho.push(sig_chirho);

        // 2. Function body: lensName f s = fmap (\x -> s { _field = x }) (f (_field s))
        let fun_chirho = generate_lens_body_chirho(
            &lens_name_chirho,
            data_name_chirho,
            con_name_chirho,
            field_name_chirho,
        );
        decs_chirho.push(fun_chirho);
    }

    decs_chirho
}

/// Generate the type signature for a lens:
/// `lensName :: Functor f => (a -> f a) -> s -> f s`
fn generate_lens_sig_chirho(
    lens_name_chirho: &str,
    data_name_chirho: &str,
    field_type_chirho: &str,
) -> ThDecChirho {
    // Build: Functor f => (FieldType -> f FieldType) -> DataType -> f DataType
    let f_name_chirho = ThNameChirho::mk_name_chirho("f");
    let s_type_chirho = ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(data_name_chirho));
    let a_type_chirho = ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(field_type_chirho));

    // f a
    let f_a_chirho = ThTypeChirho::AppTChirho(
        Box::new(ThTypeChirho::VarTChirho(f_name_chirho.clone())),
        Box::new(a_type_chirho.clone()),
    );
    // f s
    let f_s_chirho = ThTypeChirho::AppTChirho(
        Box::new(ThTypeChirho::VarTChirho(f_name_chirho.clone())),
        Box::new(s_type_chirho.clone()),
    );

    // a -> f a
    let a_to_fa_chirho = ThTypeChirho::AppTChirho(
        Box::new(ThTypeChirho::AppTChirho(
            Box::new(ThTypeChirho::ArrowTChirho),
            Box::new(a_type_chirho),
        )),
        Box::new(f_a_chirho),
    );

    // s -> f s
    let s_to_fs_chirho = ThTypeChirho::AppTChirho(
        Box::new(ThTypeChirho::AppTChirho(
            Box::new(ThTypeChirho::ArrowTChirho),
            Box::new(s_type_chirho),
        )),
        Box::new(f_s_chirho),
    );

    // (a -> f a) -> s -> f s
    let full_fn_type_chirho = ThTypeChirho::AppTChirho(
        Box::new(ThTypeChirho::AppTChirho(
            Box::new(ThTypeChirho::ArrowTChirho),
            Box::new(a_to_fa_chirho),
        )),
        Box::new(s_to_fs_chirho),
    );

    // Functor f => ...
    let functor_constraint_chirho = ThTypeChirho::AppTChirho(
        Box::new(ThTypeChirho::ConTChirho(ThNameChirho::mk_name_chirho(
            "Functor",
        ))),
        Box::new(ThTypeChirho::VarTChirho(f_name_chirho.clone())),
    );

    let forall_type_chirho = ThTypeChirho::ForallTChirho(
        vec![ThTyVarBndrChirho::PlainTVChirho(f_name_chirho)],
        vec![functor_constraint_chirho],
        Box::new(full_fn_type_chirho),
    );

    ThDecChirho::SigDChirho(
        ThNameChirho::mk_name_chirho(lens_name_chirho),
        Box::new(forall_type_chirho),
    )
}

/// Generate the function body for a lens:
/// `lensName f_param s_param = fmap (\x_param -> s_param { _field = x_param }) (f_param (_field s_param))`
fn generate_lens_body_chirho(
    lens_name_chirho: &str,
    _data_name_chirho: &str,
    _con_name_chirho: &str,
    field_name_chirho: &str,
) -> ThDecChirho {
    let f_param_chirho = ThNameChirho::mk_name_chirho("f_lens_param");
    let s_param_chirho = ThNameChirho::mk_name_chirho("s_lens_param");
    let x_param_chirho = ThNameChirho::mk_name_chirho("x_lens_param");

    // _field s_param  (field accessor applied to s_param)
    let field_access_chirho = ThExpChirho::AppEChirho(
        Box::new(ThExpChirho::VarEChirho(ThNameChirho::mk_name_chirho(
            field_name_chirho,
        ))),
        Box::new(ThExpChirho::VarEChirho(s_param_chirho.clone())),
    );

    // f_param (_field s_param)
    let f_applied_chirho = ThExpChirho::AppEChirho(
        Box::new(ThExpChirho::VarEChirho(f_param_chirho.clone())),
        Box::new(field_access_chirho),
    );

    // s_param { _field = x_param }  (record update)
    let record_update_chirho = ThExpChirho::RecUpdEChirho(
        Box::new(ThExpChirho::VarEChirho(s_param_chirho.clone())),
        vec![ThFieldExpChirho {
            name_chirho: ThNameChirho::mk_name_chirho(field_name_chirho),
            expr_chirho: ThExpChirho::VarEChirho(x_param_chirho.clone()),
        }],
    );

    // \x_param -> s_param { _field = x_param }
    let lambda_chirho = ThExpChirho::LamEChirho(
        vec![ThPatChirho::VarPChirho(x_param_chirho)],
        Box::new(record_update_chirho),
    );

    // fmap (\x_param -> ...) (f_param (_field s_param))
    let fmap_call_chirho = ThExpChirho::AppEChirho(
        Box::new(ThExpChirho::AppEChirho(
            Box::new(ThExpChirho::VarEChirho(ThNameChirho::mk_name_chirho(
                "fmap",
            ))),
            Box::new(lambda_chirho),
        )),
        Box::new(f_applied_chirho),
    );

    ThDecChirho::FunDChirho(
        ThNameChirho::mk_name_chirho(lens_name_chirho),
        vec![ThClauseChirho {
            pats_chirho: vec![
                ThPatChirho::VarPChirho(f_param_chirho),
                ThPatChirho::VarPChirho(s_param_chirho),
            ],
            body_chirho: ThBodyChirho::NormalBChirho(fmap_call_chirho),
            decs_chirho: vec![],
        }],
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, FieldDeclChirho};
    use haskelujah_ast_chirho::expr_chirho::ExprChirho;
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;
    use haskelujah_span_chirho::SpanChirho;

    fn mk_name_chirho(s_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            s_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    fn mk_var_chirho(s_chirho: &str) -> ExprChirho {
        ExprChirho::VarChirho(mk_name_chirho(s_chirho))
    }

    /// Helper: build a Person record data type with `_name :: String` and `_age :: Int`.
    fn person_data_decl_chirho() -> DeclChirho {
        DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Person"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::RecordChirho {
                name_chirho: mk_name_chirho("Person"),
                fields_chirho: vec![
                    FieldDeclChirho {
                        names_chirho: vec![mk_name_chirho("_name")],
                        ty_chirho: TypeChirho::ConChirho(mk_name_chirho("String")),
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    FieldDeclChirho {
                        names_chirho: vec![mk_name_chirho("_age")],
                        ty_chirho: TypeChirho::ConChirho(mk_name_chirho("Int")),
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    /// Helper: build a `$(makeLenses ''Person)` splice decl.
    fn make_lenses_splice_chirho(type_name_chirho: &str) -> DeclChirho {
        DeclChirho::SpliceDeclChirho {
            expr_chirho: ExprChirho::AppChirho {
                fun_chirho: Box::new(mk_var_chirho("makeLenses")),
                arg_chirho: Box::new(mk_var_chirho(type_name_chirho)),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn expand_removes_splice_decl_chirho() {
        // A module with just a splice decl and no matching type should
        // remove the splice and produce a warning.
        let decls_chirho = vec![make_lenses_splice_chirho("NonExistent")];

        let result_chirho = expand_splices_chirho(decls_chirho);

        assert!(
            result_chirho.decls_chirho.is_empty(),
            "splice should be removed even when type not found"
        );
        assert_eq!(result_chirho.warnings_chirho.len(), 1);
        assert!(result_chirho.warnings_chirho[0].contains("NonExistent"));
    }

    #[test]
    fn make_lenses_generates_lens_decls_chirho() {
        // Person has _name and _age, so we expect 4 generated decls
        // (2 type sigs + 2 function bindings).
        let decls_chirho = vec![
            person_data_decl_chirho(),
            make_lenses_splice_chirho("Person"),
        ];

        let result_chirho = expand_splices_chirho(decls_chirho);

        // The Person data decl should be preserved.
        assert!(
            result_chirho.warnings_chirho.is_empty(),
            "no warnings expected"
        );

        // Count generated decls: Person data + 2 type sigs + 2 fun binds = 5
        assert_eq!(
            result_chirho.decls_chirho.len(),
            5,
            "expected Person data decl + 4 lens decls, got {:?}",
            result_chirho.decls_chirho.len()
        );

        // First decl should be the original Person data decl.
        assert!(matches!(
            &result_chirho.decls_chirho[0],
            DeclChirho::DataDeclChirho { name_chirho, .. } if name_chirho.text_chirho() == "Person"
        ));

        // Verify we have a type sig for "name" and a fun bind for "name".
        let has_name_sig_chirho = result_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "name")
        });
        let has_name_fun_chirho = result_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. } if name_chirho.text_chirho() == "name")
        });
        let has_age_sig_chirho = result_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "age")
        });
        let has_age_fun_chirho = result_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. } if name_chirho.text_chirho() == "age")
        });

        assert!(has_name_sig_chirho, "should have TypeSig for 'name'");
        assert!(has_name_fun_chirho, "should have FunBind for 'name'");
        assert!(has_age_sig_chirho, "should have TypeSig for 'age'");
        assert!(has_age_fun_chirho, "should have FunBind for 'age'");
    }

    #[test]
    fn make_lenses_accepts_mk_name_argument_chirho() {
        let decls_chirho = vec![
            person_data_decl_chirho(),
            DeclChirho::SpliceDeclChirho {
                expr_chirho: ExprChirho::AppChirho {
                    fun_chirho: Box::new(mk_var_chirho("makeLenses")),
                    arg_chirho: Box::new(ExprChirho::AppChirho {
                        fun_chirho: Box::new(mk_var_chirho("mkName")),
                        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::StringChirho(
                            "Person".to_string(),
                            SpanChirho::DUMMY_CHIRHO,
                        ))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ];

        let result_chirho = expand_splices_chirho(decls_chirho);

        assert!(
            result_chirho.warnings_chirho.is_empty(),
            "mkName-shaped makeLenses argument should be recognized, got {:?}",
            result_chirho.warnings_chirho
        );
        assert_eq!(result_chirho.decls_chirho.len(), 5);
        assert!(result_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "name")
        }));
        assert!(result_chirho.decls_chirho.iter().any(|d_chirho| {
            matches!(d_chirho, DeclChirho::FunBindChirho { name_chirho, .. } if name_chirho.text_chirho() == "age")
        }));
    }

    #[test]
    fn unknown_splice_produces_warning_chirho() {
        let decls_chirho = vec![DeclChirho::SpliceDeclChirho {
            expr_chirho: mk_var_chirho("someUnknownFunction"),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }];

        let result_chirho = expand_splices_chirho(decls_chirho);

        assert!(result_chirho.decls_chirho.is_empty());
        assert_eq!(result_chirho.warnings_chirho.len(), 1);
        assert!(
            result_chirho.warnings_chirho[0].contains("not recognized"),
            "warning should mention 'not recognized', got: {}",
            result_chirho.warnings_chirho[0]
        );
    }

    #[test]
    fn non_splice_decls_pass_through_chirho() {
        let decls_chirho = vec![
            DeclChirho::TypeSigChirho {
                name_chirho: mk_name_chirho("foo"),
                ty_chirho: TypeChirho::ConChirho(mk_name_chirho("Int")),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: mk_name_chirho("foo"),
                matches_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ];

        let result_chirho = expand_splices_chirho(decls_chirho);

        assert_eq!(result_chirho.decls_chirho.len(), 2);
        assert!(result_chirho.warnings_chirho.is_empty());
    }

    #[test]
    fn make_lenses_skips_non_underscore_fields_chirho() {
        // A record with a field that does NOT start with '_'
        // should not generate a lens for it.
        let decls_chirho = vec![
            DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("Config"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![ConDeclChirho::RecordChirho {
                    name_chirho: mk_name_chirho("Config"),
                    fields_chirho: vec![
                        FieldDeclChirho {
                            names_chirho: vec![mk_name_chirho("verbose")],
                            ty_chirho: TypeChirho::ConChirho(mk_name_chirho("Bool")),
                            strictness_chirho:
                                haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        FieldDeclChirho {
                            names_chirho: vec![mk_name_chirho("_port")],
                            ty_chirho: TypeChirho::ConChirho(mk_name_chirho("Int")),
                            strictness_chirho:
                                haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            make_lenses_splice_chirho("Config"),
        ];

        let result_chirho = expand_splices_chirho(decls_chirho);

        // Config data + 1 type sig (port) + 1 fun bind (port) = 3
        assert_eq!(
            result_chirho.decls_chirho.len(),
            3,
            "expected Config data decl + 2 lens decls for _port only"
        );
        assert!(result_chirho.warnings_chirho.is_empty());
    }

    #[test]
    fn derive_json_produces_warning_chirho() {
        let decls_chirho = vec![
            person_data_decl_chirho(),
            DeclChirho::SpliceDeclChirho {
                expr_chirho: ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::AppChirho {
                        fun_chirho: Box::new(mk_var_chirho("deriveJSON")),
                        arg_chirho: Box::new(mk_var_chirho("defaultOptions")),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    arg_chirho: Box::new(mk_var_chirho("Person")),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ];

        let result_chirho = expand_splices_chirho(decls_chirho);

        // Person data decl should pass through, splice should be removed.
        assert_eq!(result_chirho.decls_chirho.len(), 1);
        assert_eq!(result_chirho.warnings_chirho.len(), 1);
        assert!(result_chirho.warnings_chirho[0].contains("deriveJSON"));
    }

    #[test]
    fn make_lenses_prime_variant_chirho() {
        // `makeLenses'` should work the same as `makeLenses`.
        let decls_chirho = vec![
            person_data_decl_chirho(),
            DeclChirho::SpliceDeclChirho {
                expr_chirho: ExprChirho::AppChirho {
                    fun_chirho: Box::new(mk_var_chirho("makeLenses'")),
                    arg_chirho: Box::new(mk_var_chirho("Person")),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ];

        let result_chirho = expand_splices_chirho(decls_chirho);

        assert!(result_chirho.warnings_chirho.is_empty());
        // Person data + 4 lens decls = 5
        assert_eq!(result_chirho.decls_chirho.len(), 5);
    }
}
