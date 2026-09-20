// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Stock higher-kinded and Generic instance generation.
use super::*;

/// Generate `instance Functor T where fmap f (C x1 x2) = C (f x1) x2` etc.
///
/// For each constructor field whose type mentions the last type variable,
/// apply `f` to it. Fields that don't mention it are passed through unchanged.
pub(super) fn derive_functor_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> Result<DeclChirho, String> {
    if type_vars_chirho.is_empty() {
        return Err(format!(
            "cannot derive Functor for {} — no type parameters",
            type_name_chirho.text_chirho()
        ));
    }

    let last_var_chirho = type_vars_chirho.last().unwrap().text_chirho();
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let cname_chirho = con_name_chirho(con_chirho);
        let field_types_chirho = con_field_types_chirho(con_chirho);
        let field_count_chirho = field_types_chirho.len();
        let x_vars_chirho = field_vars_chirho("x", field_count_chirho);

        let pat_chirho = con_pat_chirho(cname_chirho, &x_vars_chirho);

        // Build result: Con (maybe_f x1) (maybe_f x2) ...
        let mut result_chirho: ExprChirho = con_expr_chirho(cname_chirho);
        for (i_chirho, fty_chirho) in field_types_chirho.iter().enumerate() {
            let var_chirho = var_expr_chirho(&x_vars_chirho[i_chirho]);
            let mapped_chirho = if type_is_var_chirho(fty_chirho, last_var_chirho) {
                // Direct occurrence: apply f
                app_chirho(var_expr_chirho("f"), var_chirho)
            } else if type_mentions_var_chirho(fty_chirho, last_var_chirho) {
                // Nested occurrence: fmap f
                app_chirho(
                    app_chirho(var_expr_chirho("fmap"), var_expr_chirho("f")),
                    var_chirho,
                )
            } else {
                // No occurrence: pass through
                var_chirho
            };
            result_chirho = app_chirho(result_chirho, mapped_chirho);
        }

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![PatChirho::VarChirho(var_name_chirho("f")), pat_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(result_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let fmap_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("fmap"),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    // Functor context for all type vars except the last
    let context_chirho: Vec<_> = type_vars_chirho[..type_vars_chirho.len() - 1]
        .iter()
        .filter(|_| false) // No Functor constraints on other vars needed
        .map(
            |tv_chirho| haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                class_chirho: var_name_chirho("Functor"),
                args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
                span_chirho: gen_span_chirho(),
            },
        )
        .collect();

    // Instance type: Functor (T a1 a2 ... ) — all vars except the last
    let instance_ty_chirho = if type_vars_chirho.len() <= 1 {
        TypeChirho::ConChirho(type_name_chirho.clone())
    } else {
        let mut ty_chirho = TypeChirho::ConChirho(type_name_chirho.clone());
        for tv_chirho in &type_vars_chirho[..type_vars_chirho.len() - 1] {
            ty_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(ty_chirho),
                arg_chirho: Box::new(TypeChirho::VarChirho(tv_chirho.name_chirho.clone())),
                span_chirho: gen_span_chirho(),
            };
        }
        ty_chirho
    };

    Ok(DeclChirho::InstanceDeclChirho {
        context_chirho,
        class_chirho: var_name_chirho("Functor"),
        types_chirho: vec![instance_ty_chirho],
        methods_chirho: vec![fmap_method_chirho],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    })
}

/// Generate `instance Foldable T where foldMap f (C x1 x2) = f x1 <> mempty` etc.
///
/// For each field that mentions the last type variable, apply `f` and combine
/// with `(<>)` (mappend). Fields that don't mention it are skipped.
pub(super) fn derive_foldable_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> Result<DeclChirho, String> {
    if type_vars_chirho.is_empty() {
        return Err(format!(
            "cannot derive Foldable for {} — no type parameters",
            type_name_chirho.text_chirho()
        ));
    }

    let last_var_chirho = type_vars_chirho.last().unwrap().text_chirho();
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let cname_chirho = con_name_chirho(con_chirho);
        let field_types_chirho = con_field_types_chirho(con_chirho);
        let field_count_chirho = field_types_chirho.len();
        let x_vars_chirho = field_vars_chirho("x", field_count_chirho);

        let pat_chirho = con_pat_chirho(cname_chirho, &x_vars_chirho);

        // Collect foldMap contributions for fields that mention last_var
        let mut parts_chirho: Vec<ExprChirho> = Vec::new();
        for (i_chirho, fty_chirho) in field_types_chirho.iter().enumerate() {
            let var_chirho = var_expr_chirho(&x_vars_chirho[i_chirho]);
            if type_is_var_chirho(fty_chirho, last_var_chirho) {
                // Direct: f x
                parts_chirho.push(app_chirho(var_expr_chirho("f"), var_chirho));
            } else if type_mentions_var_chirho(fty_chirho, last_var_chirho) {
                // Nested: foldMap f x
                parts_chirho.push(app_chirho(
                    app_chirho(var_expr_chirho("foldMap"), var_expr_chirho("f")),
                    var_chirho,
                ));
            }
        }

        let body_chirho = if parts_chirho.is_empty() {
            var_expr_chirho("mempty")
        } else {
            parts_chirho
                .into_iter()
                .reduce(|acc_chirho, e_chirho| infix_chirho(acc_chirho, "<>", e_chirho))
                .unwrap()
        };

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![PatChirho::VarChirho(var_name_chirho("f")), pat_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let foldmap_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("foldMap"),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    let instance_ty_chirho = if type_vars_chirho.len() <= 1 {
        TypeChirho::ConChirho(type_name_chirho.clone())
    } else {
        let mut ty_chirho = TypeChirho::ConChirho(type_name_chirho.clone());
        for tv_chirho in &type_vars_chirho[..type_vars_chirho.len() - 1] {
            ty_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(ty_chirho),
                arg_chirho: Box::new(TypeChirho::VarChirho(tv_chirho.name_chirho.clone())),
                span_chirho: gen_span_chirho(),
            };
        }
        ty_chirho
    };

    Ok(DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: var_name_chirho("Foldable"),
        types_chirho: vec![instance_ty_chirho],
        methods_chirho: vec![foldmap_method_chirho],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    })
}

/// Generate `instance Traversable T where traverse f (C x1 x2) = C <$> f x1 <*> pure x2`
///
/// For each field: if it mentions the last type variable, use `f x` (direct) or
/// `traverse f x` (nested). Otherwise use `pure x`. Combine with `<$>` and `<*>`.
pub(super) fn derive_traversable_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> Result<DeclChirho, String> {
    if type_vars_chirho.is_empty() {
        return Err(format!(
            "cannot derive Traversable for {} — no type parameters",
            type_name_chirho.text_chirho()
        ));
    }

    let last_var_chirho = type_vars_chirho.last().unwrap().text_chirho();
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let cname_chirho = con_name_chirho(con_chirho);
        let field_types_chirho = con_field_types_chirho(con_chirho);
        let field_count_chirho = field_types_chirho.len();
        let x_vars_chirho = field_vars_chirho("x", field_count_chirho);

        let pat_chirho = con_pat_chirho(cname_chirho, &x_vars_chirho);

        if field_count_chirho == 0 {
            // No fields: pure Con
            let body_chirho = app_chirho(var_expr_chirho("pure"), con_expr_chirho(cname_chirho));
            matches_chirho.push(MatchArmChirho {
                pats_chirho: vec![PatChirho::VarChirho(var_name_chirho("f")), pat_chirho],
                rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
                where_binds_chirho: vec![],
                span_chirho: gen_span_chirho(),
            });
            continue;
        }

        // Build: Con <$> action1 <*> action2 <*> ...
        let mut actions_chirho: Vec<ExprChirho> = Vec::new();
        for (i_chirho, fty_chirho) in field_types_chirho.iter().enumerate() {
            let var_chirho = var_expr_chirho(&x_vars_chirho[i_chirho]);
            if type_is_var_chirho(fty_chirho, last_var_chirho) {
                // Direct: f x
                actions_chirho.push(app_chirho(var_expr_chirho("f"), var_chirho));
            } else if type_mentions_var_chirho(fty_chirho, last_var_chirho) {
                // Nested: traverse f x
                actions_chirho.push(app_chirho(
                    app_chirho(var_expr_chirho("traverse"), var_expr_chirho("f")),
                    var_chirho,
                ));
            } else {
                // No occurrence: pure x
                actions_chirho.push(app_chirho(var_expr_chirho("pure"), var_chirho));
            }
        }

        // Con <$> first <*> second <*> third ...
        let mut body_chirho = infix_chirho(
            con_expr_chirho(cname_chirho),
            "<$>",
            actions_chirho[0].clone(),
        );
        for action_chirho in &actions_chirho[1..] {
            body_chirho = infix_chirho(body_chirho, "<*>", action_chirho.clone());
        }

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![PatChirho::VarChirho(var_name_chirho("f")), pat_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let traverse_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("traverse"),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    let instance_ty_chirho = if type_vars_chirho.len() <= 1 {
        TypeChirho::ConChirho(type_name_chirho.clone())
    } else {
        let mut ty_chirho = TypeChirho::ConChirho(type_name_chirho.clone());
        for tv_chirho in &type_vars_chirho[..type_vars_chirho.len() - 1] {
            ty_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(ty_chirho),
                arg_chirho: Box::new(TypeChirho::VarChirho(tv_chirho.name_chirho.clone())),
                span_chirho: gen_span_chirho(),
            };
        }
        ty_chirho
    };

    Ok(DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: var_name_chirho("Traversable"),
        types_chirho: vec![instance_ty_chirho],
        methods_chirho: vec![traverse_method_chirho],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    })
}

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
pub(super) fn generic_product_expr_chirho(vars_chirho: &[String]) -> ExprChirho {
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
pub(super) fn generic_product_pat_chirho(vars_chirho: &[String]) -> PatChirho {
    match vars_chirho.len() {
        0 => PatChirho::ConChirho {
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
pub(super) fn generic_sum_wrap_chirho(
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
pub(super) fn generic_sum_pat_chirho(
    product_pat_chirho: PatChirho,
    idx_chirho: usize,
    total_chirho: usize,
) -> PatChirho {
    if total_chirho == 1 {
        return product_pat_chirho;
    }
    if idx_chirho == 0 {
        return PatChirho::ConChirho {
            con_chirho: var_name_chirho("Left"),
            args_chirho: vec![product_pat_chirho],
            span_chirho: gen_span_chirho(),
        };
    }
    let inner_chirho = generic_sum_pat_chirho(product_pat_chirho, idx_chirho - 1, total_chirho - 1);
    PatChirho::ConChirho {
        con_chirho: var_name_chirho("Right"),
        args_chirho: vec![inner_chirho],
        span_chirho: gen_span_chirho(),
    }
}
