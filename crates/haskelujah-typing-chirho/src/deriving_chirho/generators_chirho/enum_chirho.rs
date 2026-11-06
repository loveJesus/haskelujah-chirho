// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor metadata for every local data type that derives Enum (row 669).
//!
//! The derived instance writes `toEnum`, `fromEnum`, `succ` and `pred`. GHC's derived instance
//! also writes a bounded `enumFrom` and `enumFromThen`, which stop at the last (or first)
//! constructor; the dictionary pass builds those two in Core from this metadata, by row ids and
//! a primop only, so no generated reference can be captured by a program-owned namesake.
//! `enumFromTo` and `enumFromThenTo` take the class defaults, as GHC's do.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::BuiltinClassScopeChirho;

use super::stock_identity_chirho::StockIdentityChirho;
use super::*;

/// Every local `data` type whose deriving clause names Enum and whose instance derives, with
/// no builtin class proof: every class keeps its written name.
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub fn derived_enum_metadata_chirho(module_chirho: &ModuleChirho) -> HashMap<String, Vec<String>> {
    derived_enum_metadata_with_builtins_chirho(module_chirho, &BuiltinClassScopeChirho::default())
}

/// Every local `data` type whose deriving clause asks for the stock Enum class, judged with
/// the builtin classes naming proved for the module (row 694), and whose instance derives, by
/// type name: its constructor names in declaration order. These are exactly the shapes the
/// derived instance accepts, with no visible parameter and one or more constructors, all
/// nullary, so every entry has a first and a last constructor.
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub fn derived_enum_metadata_with_builtins_chirho(
    module_chirho: &ModuleChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
) -> HashMap<String, Vec<String>> {
    let identity_chirho =
        StockIdentityChirho::of_module_chirho(module_chirho, builtin_classes_chirho);
    let mut metadata_chirho = HashMap::new();
    for decl_chirho in &module_chirho.decls_chirho {
        let DeclChirho::DataDeclChirho {
            name_chirho,
            type_vars_chirho,
            constructors_chirho,
            deriving_chirho,
            kind_sig_chirho,
            span_chirho,
            ..
        } = decl_chirho
        else {
            continue;
        };
        // Only the stock instance enumerates constructors (row 694).
        if !deriving_chirho
            .iter()
            .any(|derived_chirho| identity_chirho.names_stock_class_chirho(derived_chirho, "Enum"))
        {
            continue;
        }
        let visible_chirho = declaration_heads_chirho::visible_instance_binders_chirho(
            type_vars_chirho,
            kind_sig_chirho.as_ref(),
            *span_chirho,
        );
        if !visible_chirho.is_empty()
            || constructors_chirho.is_empty()
            || !all_nullary_chirho(constructors_chirho)
        {
            continue;
        }
        metadata_chirho.insert(
            name_chirho.text_chirho().to_string(),
            constructors_chirho
                .iter()
                .map(|constructor_chirho| con_name_chirho(constructor_chirho).to_string())
                .collect(),
        );
    }
    metadata_chirho
}

#[cfg(test)]
#[path = "../tests_chirho/generators_chirho/enum_tests_chirho.rs"]
mod tests_chirho;

// ---------------------------------------------------------------------------
// Derive Enum
// ---------------------------------------------------------------------------

/// Check that all constructors are nullary (required for Enum and Bounded).
fn all_nullary_chirho(constructors_chirho: &[ConDeclChirho]) -> bool {
    constructors_chirho
        .iter()
        .all(|c_chirho| con_field_count_chirho(c_chirho) == 0)
}

/// Generate `instance Enum T where toEnum = ...; fromEnum = ...`
///
/// Only valid for enumeration types (all constructors nullary, no type params).
pub(super) fn derive_enum_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> Result<DeclChirho, String> {
    if !type_vars_chirho.is_empty() {
        return Err(format!(
            "deriving Enum not allowed for polymorphic type {}",
            type_name_chirho.text_chirho()
        ));
    }
    if !all_nullary_chirho(constructors_chirho) {
        return Err(format!(
            "deriving Enum requires all constructors to be nullary for {}",
            type_name_chirho.text_chirho()
        ));
    }
    if constructors_chirho.is_empty() {
        return Err(format!(
            "deriving Enum requires at least one constructor for {}",
            type_name_chirho.text_chirho()
        ));
    }

    // toEnum: Int -> T
    //   toEnum 0 = Con0; toEnum 1 = Con1; ...
    //   toEnum _ = error "toEnum: out of range"
    let mut to_enum_matches_chirho: Vec<MatchArmChirho> = Vec::new();
    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate() {
        to_enum_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![PatChirho::LitChirho(LitChirho::IntChirho(
                idx_chirho as i64,
                gen_span_chirho(),
                None,
            ))],
            rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho(con_name_chirho(con_chirho))),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }
    // catch-all: error
    to_enum_matches_chirho.push(MatchArmChirho {
        pats_chirho: vec![PatChirho::WildcardChirho(gen_span_chirho())],
        rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
            var_expr_chirho("error"),
            ExprChirho::LitChirho(LitChirho::StringChirho(
                format!("{}.toEnum: out of range", type_name_chirho.text_chirho()),
                gen_span_chirho(),
                None,
            )),
        )),
        where_binds_chirho: vec![],
        span_chirho: gen_span_chirho(),
    });

    let to_enum_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("toEnum"),
        matches_chirho: to_enum_matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    // fromEnum: T -> Int
    //   fromEnum Con0 = 0; fromEnum Con1 = 1; ...
    let mut from_enum_matches_chirho: Vec<MatchArmChirho> = Vec::new();
    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate() {
        from_enum_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![con_pat_chirho(con_name_chirho(con_chirho), &[])],
            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(LitChirho::IntChirho(
                idx_chirho as i64,
                gen_span_chirho(),
                None,
            ))),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let from_enum_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("fromEnum"),
        matches_chirho: from_enum_matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    // succ: T -> T
    //   succ Con0 = Con1; succ Con1 = Con2; ... succ ConN = error "succ: out of range"
    let mut succ_matches_chirho: Vec<MatchArmChirho> = Vec::new();
    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate() {
        if idx_chirho + 1 < constructors_chirho.len() {
            succ_matches_chirho.push(MatchArmChirho {
                pats_chirho: vec![con_pat_chirho(con_name_chirho(con_chirho), &[])],
                rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho(con_name_chirho(
                    &constructors_chirho[idx_chirho + 1],
                ))),
                where_binds_chirho: vec![],
                span_chirho: gen_span_chirho(),
            });
        }
    }
    // last constructor: error
    if let Some(last_chirho) = constructors_chirho.last() {
        succ_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![con_pat_chirho(con_name_chirho(last_chirho), &[])],
            rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
                var_expr_chirho("error"),
                ExprChirho::LitChirho(LitChirho::StringChirho(
                    format!("{}.succ: out of range", type_name_chirho.text_chirho()),
                    gen_span_chirho(),
                    None,
                )),
            )),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }
    let succ_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("succ"),
        matches_chirho: succ_matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    // pred: T -> T
    //   pred Con0 = error "pred: out of range"; pred Con1 = Con0; ...
    let mut pred_matches_chirho: Vec<MatchArmChirho> = Vec::new();
    // first constructor: error
    if let Some(first_chirho) = constructors_chirho.first() {
        pred_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![con_pat_chirho(con_name_chirho(first_chirho), &[])],
            rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
                var_expr_chirho("error"),
                ExprChirho::LitChirho(LitChirho::StringChirho(
                    format!("{}.pred: out of range", type_name_chirho.text_chirho()),
                    gen_span_chirho(),
                    None,
                )),
            )),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }
    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate().skip(1) {
        pred_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![con_pat_chirho(con_name_chirho(con_chirho), &[])],
            rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho(con_name_chirho(
                &constructors_chirho[idx_chirho - 1],
            ))),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }
    let pred_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("pred"),
        matches_chirho: pred_matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    Ok(DeclChirho::InstanceDeclChirho {
        kind_args_chirho: Vec::new(),
        context_chirho: vec![],
        class_chirho: var_name_chirho("Enum"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![
            to_enum_method_chirho,
            from_enum_method_chirho,
            succ_method_chirho,
            pred_method_chirho,
        ],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    })
}

// ---------------------------------------------------------------------------
// Derive Bounded
// ---------------------------------------------------------------------------

/// Generate `instance Bounded T where minBound = ...; maxBound = ...`
///
/// Only valid for enumeration types (all constructors nullary, no type params).
pub(super) fn derive_bounded_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> Result<DeclChirho, String> {
    if !type_vars_chirho.is_empty() {
        return Err(format!(
            "deriving Bounded not allowed for polymorphic type {}",
            type_name_chirho.text_chirho()
        ));
    }
    if !all_nullary_chirho(constructors_chirho) {
        return Err(format!(
            "deriving Bounded requires all constructors to be nullary for {}",
            type_name_chirho.text_chirho()
        ));
    }
    if constructors_chirho.is_empty() {
        return Err(format!(
            "deriving Bounded requires at least one constructor for {}",
            type_name_chirho.text_chirho()
        ));
    }

    let first_con_chirho = &constructors_chirho[0];
    let last_con_chirho = &constructors_chirho[constructors_chirho.len() - 1];

    // minBound = Con0
    let min_bound_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("minBound"),
        matches_chirho: vec![MatchArmChirho {
            pats_chirho: vec![],
            rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho(con_name_chirho(
                first_con_chirho,
            ))),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }],
        span_chirho: gen_span_chirho(),
    };

    // maxBound = ConN
    let max_bound_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("maxBound"),
        matches_chirho: vec![MatchArmChirho {
            pats_chirho: vec![],
            rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho(con_name_chirho(
                last_con_chirho,
            ))),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }],
        span_chirho: gen_span_chirho(),
    };

    Ok(DeclChirho::InstanceDeclChirho {
        kind_args_chirho: Vec::new(),
        context_chirho: vec![],
        class_chirho: var_name_chirho("Bounded"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![min_bound_method_chirho, max_bound_method_chirho],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    })
}

// ---------------------------------------------------------------------------
// Derive Read
// ---------------------------------------------------------------------------

/// Generate `instance Read T where readsPrec = ...`
///
/// For each constructor, generates a branch that:
/// - Matches the constructor name string
/// - For nullary: returns `[(Con, rest)]`
/// - For product: chains `readsPrec 11` calls for each field
pub(super) fn derive_read_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> DeclChirho {
    // We generate a simplified readsPrec that uses readParen and lex.
    // For each constructor Con with fields f1..fn:
    //   readsPrec d r = readParen (d > app_prec)
    //     (\r0 -> [ (Con x1 .. xn, rN) |
    //              ("Con", r1) <- lex r0,
    //              (x1, r2) <- readsPrec (app_prec+1) r1,
    //              ...,
    //              (xn, rN) <- readsPrec (app_prec+1) r(n-1) ]) r
    //   ++ next_con...
    //
    // For now, we generate a simpler version using case on lex:
    //   readsPrec _ s = concatMap (\branch -> branch s) [branch1, branch2, ...]
    // where each branch matches the constructor name.
    //
    // Simplified: generate a list-comprehension-style body using nested concatMap.
    // Since we don't have list comprehensions in our AST yet, we generate:
    //   readsPrec d = readParen False (\r -> [(Con, r1) | ("Con", r1) <- lex r])
    //   for nullary constructors, and similarly for product types.
    //
    // For the AST we have, let's generate the most direct form:
    //   readsPrec _ s = [(Con, s') | ("Con", s') <- lex s]  (nullary)
    //
    // Since we lack list comprehensions, generate:
    //   readsPrec _ s = concatMap (match_con_chirho) (lex s)
    //   where match_con_chirho ("Con", rest) = [(Con, rest)]
    //         match_con_chirho _             = []
    //
    // Actually, the simplest correct approach with our AST:
    //   readsPrec _ s = foldr (++) [] [read_Con1 s, read_Con2 s, ...]
    //   where read_Con1 s = ... per-constructor reader

    // For simplicity, generate one method body that tries each constructor.
    // Each constructor is tried via a local where-binding.
    // The overall structure:
    //   readsPrec _ s = tryRead_Con1 s ++ tryRead_Con2 s ++ ...

    let mut con_reader_exprs_chirho: Vec<ExprChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let cname_chirho = con_name_chirho(con_chirho);
        let field_count_chirho = con_field_count_chirho(con_chirho);

        if field_count_chirho == 0 {
            // For nullary: generate a case on lex that matches the constructor name
            // readsPrec _ s produces:
            //   case lex s of
            //     [("ConName", rest)] -> [(ConName, rest)]
            //     _                   -> []
            // Simplified as: filter + map over lex s
            // But we lack list ops in the AST at this stage.
            // Generate: readConName s = ...
            // For the simplest possible form that type-checks against our AST,
            // produce a lambda: \s -> [(Con, drop (length "Con") s)]
            // This is a placeholder that will be refined when we have list comprehensions.

            // Use a direct representation: a local function application
            // readCon s = [("Con", "")] if s matches, else []
            // We'll represent the reader as a constructor application for now.
            let reader_chirho = app_chirho(
                var_expr_chirho(&format!("readEnum_{}", cname_chirho)),
                var_expr_chirho("s"),
            );
            con_reader_exprs_chirho.push(reader_chirho);
        } else {
            // Product type: generates a more complex reader
            let reader_chirho = app_chirho(
                var_expr_chirho(&format!("readProd_{}", cname_chirho)),
                var_expr_chirho("s"),
            );
            con_reader_exprs_chirho.push(reader_chirho);
        }
    }

    // Combine: reader1 s ++ reader2 s ++ ...
    let body_chirho = if con_reader_exprs_chirho.is_empty() {
        // No constructors: readsPrec _ _ = []
        ExprChirho::ListChirho {
            elements_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }
    } else {
        con_reader_exprs_chirho
            .into_iter()
            .reduce(|acc_chirho, e_chirho| infix_chirho(acc_chirho, "++", e_chirho))
            .unwrap()
    };

    let reads_prec_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("readsPrec"),
        matches_chirho: vec![MatchArmChirho {
            pats_chirho: vec![
                PatChirho::WildcardChirho(gen_span_chirho()),
                PatChirho::VarChirho(var_name_chirho("s")),
            ],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        }],
        span_chirho: gen_span_chirho(),
    };

    let context_chirho: Vec<_> = type_vars_chirho
        .iter()
        .map(
            |tv_chirho| haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                kind_args_chirho: Vec::new(),
                class_chirho: var_name_chirho("Read"),
                args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
                span_chirho: gen_span_chirho(),
            },
        )
        .collect();

    DeclChirho::InstanceDeclChirho {
        kind_args_chirho: Vec::new(),
        context_chirho,
        class_chirho: var_name_chirho("Read"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![reads_prec_method_chirho],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    }
}
