// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

// ---------------------------------------------------------------------------
// Derive Eq
// ---------------------------------------------------------------------------

/// Generate `instance Eq T where (==) = ...`
pub(super) fn derive_eq_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> DeclChirho {
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let name_chirho = con_name_chirho(con_chirho);
        let field_types_chirho = con_field_types_chirho(con_chirho);
        let field_count_chirho = field_types_chirho.len();
        let a_vars_chirho = field_vars_chirho("a", field_count_chirho);
        let b_vars_chirho = field_vars_chirho("b", field_count_chirho);

        let pat_a_chirho = con_pat_chirho(name_chirho, &a_vars_chirho);
        let pat_b_chirho = con_pat_chirho(name_chirho, &b_vars_chirho);

        // Build the body: a1 == b1 && a2 == b2 && ...
        let body_chirho = if field_count_chirho == 0 {
            con_expr_chirho("True")
        } else {
            let mut eq_exprs_chirho: Vec<ExprChirho> = Vec::new();
            for ((a_chirho, b_chirho), field_ty_chirho) in a_vars_chirho
                .iter()
                .zip(b_vars_chirho.iter())
                .zip(field_types_chirho.iter())
            {
                eq_exprs_chirho.push(eq_expr_for_type_chirho(
                    var_expr_chirho(a_chirho),
                    var_expr_chirho(b_chirho),
                    field_ty_chirho,
                ));
            }
            // Chain with &&
            eq_exprs_chirho
                .into_iter()
                .reduce(|acc_chirho, e_chirho| infix_chirho(acc_chirho, "&&", e_chirho))
                .unwrap()
        };

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![pat_a_chirho, pat_b_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    // Add catch-all: _ _ = False
    if constructors_chirho.len() > 1 {
        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![
                PatChirho::WildcardChirho(gen_span_chirho()),
                PatChirho::WildcardChirho(gen_span_chirho()),
            ],
            rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho("False")),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let eq_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("=="),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    // Build context: Eq constraints for type vars
    let context_chirho: Vec<_> = type_vars_chirho
        .iter()
        .map(
            |tv_chirho| haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                kind_args_chirho: Vec::new(),
                class_chirho: var_name_chirho("Eq"),
                args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
                span_chirho: gen_span_chirho(),
            },
        )
        .collect();

    DeclChirho::InstanceDeclChirho {
        kind_args_chirho: Vec::new(),
        context_chirho,
        class_chirho: var_name_chirho("Eq"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![eq_method_chirho],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Derive Ord
// ---------------------------------------------------------------------------

/// Generate `instance Ord T where compare = ...`
pub(super) fn derive_ord_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> DeclChirho {
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for (idx_chirho, con_chirho) in constructors_chirho.iter().enumerate() {
        let name_chirho = con_name_chirho(con_chirho);
        let field_count_chirho = con_field_count_chirho(con_chirho);
        let a_vars_chirho = field_vars_chirho("a", field_count_chirho);
        let b_vars_chirho = field_vars_chirho("b", field_count_chirho);

        let pat_a_chirho = con_pat_chirho(name_chirho, &a_vars_chirho);
        let pat_b_chirho = con_pat_chirho(name_chirho, &b_vars_chirho);

        // Same constructor: compare fields left to right
        let body_chirho = if field_count_chirho == 0 {
            con_expr_chirho("EQ")
        } else {
            // Build nested: case compare a1 b1 of { EQ -> case compare a2 b2 of { EQ -> EQ; r -> r }; r -> r }
            let mut result_chirho = con_expr_chirho("EQ");
            for i_chirho in (0..field_count_chirho).rev() {
                let compare_call_chirho = app_chirho(
                    app_chirho(
                        var_expr_chirho("compare"),
                        var_expr_chirho(&a_vars_chirho[i_chirho]),
                    ),
                    var_expr_chirho(&b_vars_chirho[i_chirho]),
                );
                result_chirho = ExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(compare_call_chirho),
                    alts_chirho: vec![
                        AltChirho {
                            pat_chirho: PatChirho::ConChirho {
                                type_args_chirho: Vec::new(),
                                con_chirho: var_name_chirho("EQ"),
                                args_chirho: vec![],
                                span_chirho: gen_span_chirho(),
                            },
                            rhs_chirho: RhsChirho::UnguardedChirho(result_chirho),
                            where_binds_chirho: vec![],
                            span_chirho: gen_span_chirho(),
                        },
                        AltChirho {
                            pat_chirho: PatChirho::VarChirho(var_name_chirho("r")),
                            rhs_chirho: RhsChirho::UnguardedChirho(var_expr_chirho("r")),
                            where_binds_chirho: vec![],
                            span_chirho: gen_span_chirho(),
                        },
                    ],
                    span_chirho: gen_span_chirho(),
                };
            }
            result_chirho
        };

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![pat_a_chirho, pat_b_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });

        // Different constructors: earlier tag < later tag
        // Add catch-all patterns for this constructor vs wildcards
        if constructors_chirho.len() > 1 {
            let _ = idx_chirho; // tag ordering handled by catch-all below
        }
    }

    // Catch-all: compare constructor indices
    if constructors_chirho.len() > 1 {
        // Generate: Con1 _ .. _ `compare` _ = LT for earlier constructors
        // and: _ `compare` Con1 _ .. _ = GT
        // Simplified approach: match (conIndex a, conIndex b) via case expressions
        // For now, use a simple approach: earlier constructors are LT, later GT
        for (i_chirho, con_i_chirho) in constructors_chirho.iter().enumerate() {
            for (j_chirho, _con_j_chirho) in constructors_chirho.iter().enumerate() {
                if i_chirho >= j_chirho {
                    continue; // same or already covered
                }
                // Con_i < Con_j: Con_i _ _ `compare` Con_j _ _ = LT
                let fields_i_chirho = con_field_count_chirho(con_i_chirho);
                let pat_i_chirho = con_pat_chirho(
                    con_name_chirho(con_i_chirho),
                    &field_vars_chirho("_x", fields_i_chirho),
                );
                let fields_j_chirho = con_field_count_chirho(_con_j_chirho);
                let pat_j_chirho = con_pat_chirho(
                    con_name_chirho(_con_j_chirho),
                    &field_vars_chirho("_y", fields_j_chirho),
                );

                matches_chirho.push(MatchArmChirho {
                    pats_chirho: vec![pat_i_chirho, pat_j_chirho.clone()],
                    rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho("LT")),
                    where_binds_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                });

                matches_chirho.push(MatchArmChirho {
                    pats_chirho: vec![
                        pat_j_chirho,
                        con_pat_chirho(
                            con_name_chirho(con_i_chirho),
                            &field_vars_chirho("_z", fields_i_chirho),
                        ),
                    ],
                    rhs_chirho: RhsChirho::UnguardedChirho(con_expr_chirho("GT")),
                    where_binds_chirho: vec![],
                    span_chirho: gen_span_chirho(),
                });
            }
        }
    }

    let compare_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("compare"),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    let context_chirho: Vec<_> = type_vars_chirho
        .iter()
        .map(
            |tv_chirho| haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                kind_args_chirho: Vec::new(),
                class_chirho: var_name_chirho("Ord"),
                args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
                span_chirho: gen_span_chirho(),
            },
        )
        .collect();

    DeclChirho::InstanceDeclChirho {
        kind_args_chirho: Vec::new(),
        context_chirho,
        class_chirho: var_name_chirho("Ord"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![compare_method_chirho],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    }
}
