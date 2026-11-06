// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

// ---------------------------------------------------------------------------
// Derive Generic
// ---------------------------------------------------------------------------

/// Generate `instance Generic T where from = ...; to = ...`
///
/// Uses a sum-of-products representation:
/// - **Sum**: multiple constructors → right-nested `Either`
///   - 1 con: product directly
///   - 2 cons: `Either prod0 prod1`
///   - 3+ cons: `Either prod0 (Either prod1 (Either prod2 ...))`
/// - **Product**: constructor fields → tuples
///   - 0 fields: `()`
///   - 1 field: just the value
///   - 2+ fields: `(f1, f2, ...)` tuple
pub(super) fn derive_generic_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> Result<DeclChirho, String> {
    if constructors_chirho.is_empty() {
        return Err(format!(
            "cannot derive Generic for {} — no constructors",
            type_name_chirho.text_chirho()
        ));
    }

    let num_cons_chirho = constructors_chirho.len();

    // --- Generate `from` method ---
    let mut from_matches_chirho: Vec<MatchArmChirho> = Vec::new();
    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate() {
        let cname_chirho = con_name_chirho(con_chirho);
        let field_count_chirho = con_field_count_chirho(con_chirho);
        let x_vars_chirho = field_vars_chirho("x", field_count_chirho);
        let pat_chirho = con_pat_chirho(cname_chirho, &x_vars_chirho);

        // Build the product representation for this constructor
        let product_chirho = generic_product_expr_chirho(&x_vars_chirho);

        // Wrap in sum encoding (Left/Right nesting)
        let sum_chirho = generic_sum_wrap_chirho(product_chirho, idx_chirho, num_cons_chirho);

        from_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![pat_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(sum_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let from_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("from"),
        matches_chirho: from_matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    // --- Generate `to` method ---
    let mut to_matches_chirho: Vec<MatchArmChirho> = Vec::new();
    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate() {
        let cname_chirho = con_name_chirho(con_chirho);
        let field_count_chirho = con_field_count_chirho(con_chirho);
        let x_vars_chirho = field_vars_chirho("x", field_count_chirho);

        // Build the product pattern for this constructor
        let product_pat_chirho = generic_product_pat_chirho(&x_vars_chirho);

        // Wrap in sum pattern (Left/Right nesting)
        let sum_pat_chirho =
            generic_sum_pat_chirho(product_pat_chirho, idx_chirho, num_cons_chirho);

        // Build the data constructor application: Con x1 x2 ...
        let mut body_chirho: ExprChirho = con_expr_chirho(cname_chirho);
        for var_chirho in &x_vars_chirho {
            body_chirho = app_chirho(body_chirho, var_expr_chirho(var_chirho));
        }

        to_matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![sum_pat_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let to_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("to"),
        matches_chirho: to_matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    Ok(DeclChirho::InstanceDeclChirho {
        kind_args_chirho: Vec::new(),
        context_chirho: vec![],
        class_chirho: var_name_chirho("Generic"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![from_method_chirho, to_method_chirho],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    })
}

/// Build the product expression for a constructor's fields.
/// - 0 fields → `()`
/// - 1 field → `x1`
/// - 2+ fields → `(x1, x2, ...)`
fn generic_product_expr_chirho(vars_chirho: &[String]) -> ExprChirho {
    match vars_chirho.len() {
        0 => con_expr_chirho("()"),
        1 => var_expr_chirho(&vars_chirho[0]),
        _ => ExprChirho::TupleChirho {
            elements_chirho: vars_chirho
                .iter()
                .map(|v_chirho| var_expr_chirho(v_chirho))
                .collect(),
            span_chirho: gen_span_chirho(),
        },
    }
}

/// Build the product pattern for a constructor's fields.
/// - 0 fields → `()`
/// - 1 field → `x1`
/// - 2+ fields → `(x1, x2, ...)`
fn generic_product_pat_chirho(vars_chirho: &[String]) -> PatChirho {
    match vars_chirho.len() {
        0 => PatChirho::ConChirho {
            type_args_chirho: Vec::new(),
            con_chirho: var_name_chirho("()"),
            args_chirho: vec![],
            span_chirho: gen_span_chirho(),
        },
        1 => PatChirho::VarChirho(var_name_chirho(&vars_chirho[0])),
        _ => PatChirho::TupleChirho {
            elements_chirho: vars_chirho
                .iter()
                .map(|v_chirho| PatChirho::VarChirho(var_name_chirho(v_chirho)))
                .collect(),
            span_chirho: gen_span_chirho(),
        },
    }
}

/// Wrap a product expression in the sum encoding for constructor at `idx` out of `total`.
/// - total == 1 → just the product (no wrapping)
/// - idx == 0 → `Left product`
/// - idx == total-1 → nested `Right (Right (... (Right product)))`
/// - otherwise → nested Right wrapping then Left
fn generic_sum_wrap_chirho(
    product_chirho: ExprChirho,
    idx_chirho: usize,
    total_chirho: usize,
) -> ExprChirho {
    if total_chirho == 1 {
        return product_chirho;
    }
    if idx_chirho == 0 {
        return app_chirho(con_expr_chirho("Left"), product_chirho);
    }
    // For idx > 0: wrap in Right, then recurse with total-1 and idx-1
    let inner_chirho = generic_sum_wrap_chirho(product_chirho, idx_chirho - 1, total_chirho - 1);
    app_chirho(con_expr_chirho("Right"), inner_chirho)
}

/// Build the sum pattern for constructor at `idx` out of `total`.
/// - total == 1 → just the product pattern
/// - idx == 0 → `Left product_pat`
/// - idx > 0 → `Right (recurse with idx-1, total-1)`
fn generic_sum_pat_chirho(
    product_pat_chirho: PatChirho,
    idx_chirho: usize,
    total_chirho: usize,
) -> PatChirho {
    if total_chirho == 1 {
        return product_pat_chirho;
    }
    if idx_chirho == 0 {
        return PatChirho::ConChirho {
            type_args_chirho: Vec::new(),
            con_chirho: var_name_chirho("Left"),
            args_chirho: vec![product_pat_chirho],
            span_chirho: gen_span_chirho(),
        };
    }
    let inner_chirho = generic_sum_pat_chirho(product_pat_chirho, idx_chirho - 1, total_chirho - 1);
    PatChirho::ConChirho {
        type_args_chirho: Vec::new(),
        con_chirho: var_name_chirho("Right"),
        args_chirho: vec![inner_chirho],
        span_chirho: gen_span_chirho(),
    }
}
