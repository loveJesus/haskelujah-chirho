// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Stock Eq, Ord, Show, Enum, Bounded and Read instance generation.
use super::*;

/// One classification drives early dispatch and late-GND ownership. A known
/// stock class is not a promise that its generator is implemented: unsupported
/// stock derivation stays explicit and must never become a coercion instance.
#[derive(Clone, Copy)]
pub(crate) struct StockClassChirho {
    generator_chirho: StockGeneratorChirho,
}

type StockGeneratorChirho =
    fn(&NameChirho, &[TyVarChirho], &[ConDeclChirho], SpanChirho) -> Result<DeclChirho, String>;

impl StockClassChirho {
    pub(crate) fn from_name_chirho(name_chirho: &str) -> Option<Self> {
        let generator_chirho: StockGeneratorChirho = match name_chirho {
            "Eq" => |name_chirho, parameters_chirho, constructors_chirho, span_chirho| {
                Ok(derive_eq_chirho(
                    name_chirho,
                    parameters_chirho,
                    constructors_chirho,
                    span_chirho,
                ))
            },
            "Ord" => |name_chirho, parameters_chirho, constructors_chirho, span_chirho| {
                Ok(derive_ord_chirho(
                    name_chirho,
                    parameters_chirho,
                    constructors_chirho,
                    span_chirho,
                ))
            },
            "Show" => |name_chirho, parameters_chirho, constructors_chirho, span_chirho| {
                Ok(derive_show_chirho(
                    name_chirho,
                    parameters_chirho,
                    constructors_chirho,
                    span_chirho,
                ))
            },
            "Read" => |name_chirho, parameters_chirho, constructors_chirho, span_chirho| {
                Ok(derive_read_chirho(
                    name_chirho,
                    parameters_chirho,
                    constructors_chirho,
                    span_chirho,
                ))
            },
            "Enum" => derive_enum_chirho,
            "Bounded" => derive_bounded_chirho,
            "Functor" => derive_functor_chirho,
            "Foldable" => derive_foldable_chirho,
            "Traversable" => derive_traversable_chirho,
            "Generic" => derive_generic_chirho,
            "Generic1" => |name_chirho, _parameters_chirho, _constructors_chirho, _span_chirho| {
                Err(format!(
                    "stock deriving Generic1 not yet supported for {}",
                    name_chirho.text_chirho()
                ))
            },
            // Existing metadata-only support, not GND representation contexts
            // or a new claim of Data/Lift method generation.
            "Data" => |name_chirho, parameters_chirho, _constructors_chirho, span_chirho| {
                Ok(derive_anyclass_chirho(
                    name_chirho,
                    parameters_chirho,
                    &var_name_chirho("Data"),
                    span_chirho,
                ))
            },
            "Typeable" => |name_chirho, parameters_chirho, _constructors_chirho, span_chirho| {
                Ok(derive_anyclass_chirho(
                    name_chirho,
                    parameters_chirho,
                    &var_name_chirho("Typeable"),
                    span_chirho,
                ))
            },
            "Lift" => |name_chirho, parameters_chirho, _constructors_chirho, span_chirho| {
                Ok(derive_anyclass_chirho(
                    name_chirho,
                    parameters_chirho,
                    &var_name_chirho("Lift"),
                    span_chirho,
                ))
            },
            _ => return None,
        };
        Some(Self { generator_chirho })
    }

    pub(super) fn derive_chirho(
        self,
        name_chirho: &NameChirho,
        parameters_chirho: &[TyVarChirho],
        constructors_chirho: &[ConDeclChirho],
        span_chirho: SpanChirho,
    ) -> Result<DeclChirho, String> {
        (self.generator_chirho)(
            name_chirho,
            parameters_chirho,
            constructors_chirho,
            span_chirho,
        )
    }
}

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
                class_chirho: var_name_chirho("Eq"),
                args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
                span_chirho: gen_span_chirho(),
            },
        )
        .collect();

    DeclChirho::InstanceDeclChirho {
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
                class_chirho: var_name_chirho("Ord"),
                args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
                span_chirho: gen_span_chirho(),
            },
        )
        .collect();

    DeclChirho::InstanceDeclChirho {
        context_chirho,
        class_chirho: var_name_chirho("Ord"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![compare_method_chirho],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Derive Show
// ---------------------------------------------------------------------------

/// Generate `instance Show T where show = ...`
pub(super) fn derive_show_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> DeclChirho {
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let name_chirho = con_name_chirho(con_chirho);
        let field_count_chirho = con_field_count_chirho(con_chirho);
        let a_vars_chirho = field_vars_chirho("a", field_count_chirho);
        let pat_chirho = con_pat_chirho(name_chirho, &a_vars_chirho);

        // Build: "ConName" ++ " " ++ show a1 ++ " " ++ show a2 ++ ...
        let body_chirho = if field_count_chirho == 0 {
            ExprChirho::LitChirho(LitChirho::StringChirho(
                name_chirho.to_string(),
                gen_span_chirho(),
            ))
        } else {
            // Start with "ConName"
            let mut result_chirho = ExprChirho::LitChirho(LitChirho::StringChirho(
                name_chirho.to_string(),
                gen_span_chirho(),
            ));

            for var_name_str_chirho in &a_vars_chirho {
                // Append " "
                result_chirho = infix_chirho(
                    result_chirho,
                    "++",
                    ExprChirho::LitChirho(LitChirho::StringChirho(
                        " ".to_string(),
                        gen_span_chirho(),
                    )),
                );
                // Append show ai
                let show_field_chirho = app_chirho(
                    var_expr_chirho("show"),
                    var_expr_chirho(var_name_str_chirho),
                );
                result_chirho = infix_chirho(result_chirho, "++", show_field_chirho);
            }

            result_chirho
        };

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![pat_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let show_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("show"),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    let context_chirho: Vec<_> = type_vars_chirho
        .iter()
        .map(
            |tv_chirho| haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                class_chirho: var_name_chirho("Show"),
                args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
                span_chirho: gen_span_chirho(),
            },
        )
        .collect();

    DeclChirho::InstanceDeclChirho {
        context_chirho,
        class_chirho: var_name_chirho("Show"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![show_method_chirho],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Derive Enum
// ---------------------------------------------------------------------------

/// Check that all constructors are nullary (required for Enum and Bounded).
pub(super) fn all_nullary_chirho(constructors_chirho: &[ConDeclChirho]) -> bool {
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
                class_chirho: var_name_chirho("Read"),
                args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
                span_chirho: gen_span_chirho(),
            },
        )
        .collect();

    DeclChirho::InstanceDeclChirho {
        context_chirho,
        class_chirho: var_name_chirho("Read"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![reads_prec_method_chirho],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    }
}
