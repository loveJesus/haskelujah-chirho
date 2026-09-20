// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Via and anyclass instance builders; strategy selection stays with the parent.
use super::*;

/// Generate a DeriveAnyClass instance: empty methods list relying on defaults.
pub(super) fn derive_anyclass_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    class_name_chirho: &NameChirho,
    span_chirho: SpanChirho,
) -> DeclChirho {
    let mut types_chirho = vec![TypeChirho::ConChirho(type_name_chirho.clone())];
    for tv_chirho in type_vars_chirho {
        types_chirho.push(TypeChirho::VarChirho(tv_chirho.name_chirho.clone()));
    }
    DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: class_name_chirho.clone(),
        types_chirho,
        methods_chirho: vec![], // rely on default methods
        assoc_tf_instances_chirho: vec![],
        span_chirho,
    }
}

/// Generate a `deriving via` instance by looking up the newtype constructor
/// and generating methods that unwrap, delegate to the via type's instance,
/// and re-wrap where needed.
pub(super) fn derive_via_chirho(
    module_chirho: &ModuleChirho,
    type_name_chirho: &NameChirho,
    class_name_chirho: &NameChirho,
    via_type_chirho: &TypeChirho,
) -> DeclChirho {
    // Find the constructor name for this newtype/data type
    let con_name_str_chirho = find_first_con_name_chirho(module_chirho, type_name_chirho)
        .unwrap_or_else(|| type_name_chirho.text_chirho().to_string());

    let class_text_chirho = class_name_chirho.text_chirho();

    // Generate methods based on the class
    let methods_chirho = match class_text_chirho {
        "Show" => derive_via_show_methods_chirho(&con_name_str_chirho),
        "Eq" => derive_via_eq_methods_chirho(&con_name_str_chirho, via_type_chirho),
        "Ord" => derive_via_ord_methods_chirho(&con_name_str_chirho),
        "Num" => derive_via_num_methods_chirho(&con_name_str_chirho),
        _ => vec![], // Fallback: empty methods (default implementations)
    };

    // For the generated instance, the methods explicitly delegate to the via
    // type's methods by unwrapping the newtype. The via type's class instance
    // is already available in the environment, so no context constraint needed.
    DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: class_name_chirho.clone(),
        types_chirho: vec![TypeChirho::ConChirho(type_name_chirho.clone())],
        assoc_tf_instances_chirho: vec![],
        methods_chirho,
        span_chirho: gen_span_chirho(),
    }
}

/// Find the first constructor name for a type in the module.
pub(super) fn find_first_con_name_chirho(
    module_chirho: &ModuleChirho,
    type_name_chirho: &NameChirho,
) -> Option<String> {
    let target_chirho = type_name_chirho.text_chirho();
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                constructor_chirho,
                ..
            } if name_chirho.text_chirho() == target_chirho => {
                return Some(con_name_chirho(constructor_chirho).to_string());
            }
            DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                ..
            } if name_chirho.text_chirho() == target_chirho => {
                if let Some(first_chirho) = constructors_chirho.first() {
                    return Some(con_name_chirho(first_chirho).to_string());
                }
            }
            _ => {}
        }
    }
    None
}

/// DerivingVia Show: `show (Con x) = show x`
pub(super) fn derive_via_show_methods_chirho(con_name_str_chirho: &str) -> Vec<LocalBindChirho> {
    let match_chirho = MatchArmChirho {
        pats_chirho: vec![con_pat_chirho(con_name_str_chirho, &["x1".to_string()])],
        rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
            var_expr_chirho("show"),
            var_expr_chirho("x1"),
        )),
        where_binds_chirho: vec![],
        span_chirho: gen_span_chirho(),
    };
    vec![LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("show"),
        matches_chirho: vec![match_chirho],
        span_chirho: gen_span_chirho(),
    }]
}

/// DerivingVia Eq: `(==) (Con x) (Con y) = (==) x y`
pub(super) fn derive_via_eq_methods_chirho(
    con_name_str_chirho: &str,
    via_type_chirho: &TypeChirho,
) -> Vec<LocalBindChirho> {
    let match_chirho = MatchArmChirho {
        pats_chirho: vec![
            con_pat_chirho(con_name_str_chirho, &["x1".to_string()]),
            con_pat_chirho(con_name_str_chirho, &["y1".to_string()]),
        ],
        rhs_chirho: RhsChirho::UnguardedChirho(eq_expr_for_type_chirho(
            var_expr_chirho("x1"),
            var_expr_chirho("y1"),
            via_type_chirho,
        )),
        where_binds_chirho: vec![],
        span_chirho: gen_span_chirho(),
    };
    vec![LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("=="),
        matches_chirho: vec![match_chirho],
        span_chirho: gen_span_chirho(),
    }]
}

/// DerivingVia Ord: `compare (Con x) (Con y) = compare x y`
pub(super) fn derive_via_ord_methods_chirho(con_name_str_chirho: &str) -> Vec<LocalBindChirho> {
    let match_chirho = MatchArmChirho {
        pats_chirho: vec![
            con_pat_chirho(con_name_str_chirho, &["x1".to_string()]),
            con_pat_chirho(con_name_str_chirho, &["y1".to_string()]),
        ],
        rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
            app_chirho(var_expr_chirho("compare"), var_expr_chirho("x1")),
            var_expr_chirho("y1"),
        )),
        where_binds_chirho: vec![],
        span_chirho: gen_span_chirho(),
    };
    vec![LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("compare"),
        matches_chirho: vec![match_chirho],
        span_chirho: gen_span_chirho(),
    }]
}

/// DerivingVia Num: `(+) (Con x) (Con y) = Con (x + y)`, `fromInteger n = Con (fromInteger n)`, etc.
pub(super) fn derive_via_num_methods_chirho(con_name_str_chirho: &str) -> Vec<LocalBindChirho> {
    let mut methods_chirho = Vec::new();

    // Binary ops: (+), (*), (-), abs, signum — `op (Con x) (Con y) = Con (op x y)`
    for op_chirho in &["+", "*", "-"] {
        let match_chirho = MatchArmChirho {
            pats_chirho: vec![
                con_pat_chirho(con_name_str_chirho, &["x1".to_string()]),
                con_pat_chirho(con_name_str_chirho, &["y1".to_string()]),
            ],
            rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
                con_expr_chirho(con_name_str_chirho),
                infix_chirho(var_expr_chirho("x1"), op_chirho, var_expr_chirho("y1")),
            )),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        methods_chirho.push(LocalBindChirho::FunBindChirho {
            name_chirho: var_name_chirho(op_chirho),
            matches_chirho: vec![match_chirho],
            span_chirho: gen_span_chirho(),
        });
    }

    // Unary ops: abs, signum, negate — `f (Con x) = Con (f x)`
    for fn_chirho in &["abs", "signum", "negate"] {
        let match_chirho = MatchArmChirho {
            pats_chirho: vec![con_pat_chirho(con_name_str_chirho, &["x1".to_string()])],
            rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
                con_expr_chirho(con_name_str_chirho),
                app_chirho(var_expr_chirho(fn_chirho), var_expr_chirho("x1")),
            )),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        };
        methods_chirho.push(LocalBindChirho::FunBindChirho {
            name_chirho: var_name_chirho(fn_chirho),
            matches_chirho: vec![match_chirho],
            span_chirho: gen_span_chirho(),
        });
    }

    // fromInteger: `fromInteger n = Con (fromInteger n)`
    let from_int_chirho = MatchArmChirho {
        pats_chirho: vec![PatChirho::VarChirho(var_name_chirho("n1"))],
        rhs_chirho: RhsChirho::UnguardedChirho(app_chirho(
            con_expr_chirho(con_name_str_chirho),
            app_chirho(var_expr_chirho("fromInteger"), var_expr_chirho("n1")),
        )),
        where_binds_chirho: vec![],
        span_chirho: gen_span_chirho(),
    };
    methods_chirho.push(LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("fromInteger"),
        matches_chirho: vec![from_int_chirho],
        span_chirho: gen_span_chirho(),
    });

    methods_chirho
}
