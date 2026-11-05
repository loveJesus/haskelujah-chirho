// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written and implicit deriving heads, isolated from the per-class generators.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::*;

use haskelujah_ast_chirho::decl_chirho::{DerivedClassChirho, DerivingStrategyChirho};

use super::strategy_route_chirho::{DeclaredTypeChirho, DerivingPolicyChirho, derive_class_chirho};

/// One standalone deriving declaration's request: its class, written strategy and span.
struct StandaloneRequestChirho<'a> {
    class_chirho: &'a NameChirho,
    strategy_chirho: Option<&'a DerivingStrategyChirho>,
    span_chirho: SpanChirho,
}

pub(super) fn derive_standalone_instances_chirho(
    module_chirho: &ModuleChirho,
    policy_chirho: &DerivingPolicyChirho<'_>,
    instances_chirho: &mut Vec<DeclChirho>,
    warnings_chirho: &mut Vec<String>,
) {
    // Process standalone deriving declarations: `deriving instance Show Foo`
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::StandaloneDerivingDeclChirho {
            strategy_chirho,
            context_chirho,
            class_chirho,
            types_chirho,
            span_chirho,
        } = decl_chirho
        {
            let type_name_chirho = types_chirho.first().and_then(target_name_chirho);

            if let Some(target_chirho) = type_name_chirho {
                let first_generated_chirho = instances_chirho.len();
                let request_chirho = StandaloneRequestChirho {
                    class_chirho,
                    strategy_chirho: strategy_chirho.as_ref(),
                    span_chirho: *span_chirho,
                };
                let found_chirho = derive_standalone_for_type_chirho(
                    module_chirho,
                    &request_chirho,
                    target_chirho,
                    policy_chirho,
                    instances_chirho,
                    warnings_chirho,
                );
                // Stock generators supply method bodies, but the instance is
                // the declaration the user wrote, including specialization and
                // its context. A kind-signature header may have no AST binders.
                for generated_chirho in &mut instances_chirho[first_generated_chirho..] {
                    if let DeclChirho::InstanceDeclChirho {
                        context_chirho: generated_context_chirho,
                        types_chirho: generated_types_chirho,
                        ..
                    } = generated_chirho
                    {
                        *generated_context_chirho = context_chirho.clone();
                        *generated_types_chirho = types_chirho.clone();
                    }
                }
                if !found_chirho {
                    warnings_chirho.push(format!(
                        "standalone deriving: type {} not found in module",
                        target_chirho
                    ));
                }
            }
        }
    }
}

/// Process a standalone deriving declaration by finding the target data/newtype
/// in the module and deriving the requested class for it.
fn derive_standalone_for_type_chirho(
    module_chirho: &ModuleChirho,
    request_chirho: &StandaloneRequestChirho<'_>,
    target_chirho: &str,
    policy_chirho: &DerivingPolicyChirho<'_>,
    instances_chirho: &mut Vec<DeclChirho>,
    warnings_chirho: &mut Vec<String>,
) -> bool {
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                ..
            } if name_chirho.text_chirho() == target_chirho => {
                let visible_vars_chirho = TyVarChirho::visible_binders_chirho(type_vars_chirho);
                let declared_chirho = DeclaredTypeChirho {
                    name_chirho,
                    type_vars_chirho: visible_vars_chirho.as_ref(),
                    constructors_chirho,
                    newtype_constructor_chirho: None,
                    span_chirho: request_chirho.span_chirho,
                };
                derive_standalone_class_chirho(
                    &declared_chirho,
                    request_chirho,
                    policy_chirho,
                    instances_chirho,
                    warnings_chirho,
                );
                return true;
            }
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructor_chirho,
                ..
            } if name_chirho.text_chirho() == target_chirho => {
                let cons_chirho = vec![constructor_chirho.clone()];
                let visible_vars_chirho = TyVarChirho::visible_binders_chirho(type_vars_chirho);
                let declared_chirho = DeclaredTypeChirho {
                    name_chirho,
                    type_vars_chirho: visible_vars_chirho.as_ref(),
                    constructors_chirho: &cons_chirho,
                    newtype_constructor_chirho: Some(constructor_chirho),
                    span_chirho: request_chirho.span_chirho,
                };
                derive_standalone_class_chirho(
                    &declared_chirho,
                    request_chirho,
                    policy_chirho,
                    instances_chirho,
                    warnings_chirho,
                );
                return true;
            }
            _ => {}
        }
    }
    false
}

/// A written strategy routes like a deriving clause's (row 694); an unwritten one keeps
/// standalone deriving's stock-only route.
/// workflow: language-features-chirho/deriving-strategies-chirho
fn derive_standalone_class_chirho(
    declared_chirho: &DeclaredTypeChirho<'_>,
    request_chirho: &StandaloneRequestChirho<'_>,
    policy_chirho: &DerivingPolicyChirho<'_>,
    instances_chirho: &mut Vec<DeclChirho>,
    warnings_chirho: &mut Vec<String>,
) {
    if let Some(strategy_chirho) = request_chirho.strategy_chirho {
        let derived_chirho = DerivedClassChirho::new_chirho(
            request_chirho.class_chirho.clone(),
            Some(strategy_chirho.clone()),
        );
        derive_class_chirho(
            declared_chirho,
            &derived_chirho,
            policy_chirho,
            instances_chirho,
            warnings_chirho,
        );
        return;
    }
    let class_text_chirho = request_chirho.class_chirho.text_chirho();
    if !derive_stock_class_chirho(
        class_text_chirho,
        declared_chirho.name_chirho,
        declared_chirho.type_vars_chirho,
        declared_chirho.constructors_chirho,
        declared_chirho.span_chirho,
        instances_chirho,
        warnings_chirho,
    ) {
        warnings_chirho.push(format!(
            "standalone deriving {class_text_chirho} not yet supported for {}",
            declared_chirho.name_chirho.text_chirho()
        ));
    }
}

/// Run a class's stock generator for a data or newtype declaration, shared by deriving
/// clauses and standalone deriving. False when the class has no stock generator.
pub(super) fn derive_stock_class_chirho(
    class_text_chirho: &str,
    name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    span_chirho: SpanChirho,
    instances_chirho: &mut Vec<DeclChirho>,
    warnings_chirho: &mut Vec<String>,
) -> bool {
    match class_text_chirho {
        "Eq" => {
            instances_chirho.push(derive_eq_chirho(
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                span_chirho,
            ));
        }
        "Ord" => {
            instances_chirho.push(derive_ord_chirho(
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                span_chirho,
            ));
        }
        "Show" => {
            instances_chirho.push(derive_show_chirho(
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                span_chirho,
            ));
        }
        "Read" => {
            instances_chirho.push(derive_read_chirho(
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                span_chirho,
            ));
        }
        "Enum" => {
            match derive_enum_chirho(
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                span_chirho,
            ) {
                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
            }
        }
        "Bounded" => {
            match derive_bounded_chirho(
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                span_chirho,
            ) {
                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
            }
        }
        "Functor" => {
            match derive_functor_chirho(
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                span_chirho,
            ) {
                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
            }
        }
        "Foldable" => {
            match derive_foldable_chirho(
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                span_chirho,
            ) {
                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
            }
        }
        "Traversable" => {
            match derive_traversable_chirho(
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                span_chirho,
            ) {
                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
            }
        }
        "Generic" => {
            match derive_generic_chirho(
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                span_chirho,
            ) {
                Ok(inst_chirho) => instances_chirho.push(inst_chirho),
                Err(msg_chirho) => warnings_chirho.push(msg_chirho),
            }
        }
        // Standalone deriving writes no instance for these well-known classes; a
        // deriving clause gives them an empty instance before reaching here.
        "Typeable" | "Data" | "Lift" | "NFData" => {}
        _ => return false,
    }
    true
}

/// The type constructor a standalone deriving head targets: `T` in `C (T a)`.
pub(crate) fn target_name_chirho(ty_chirho: &TypeChirho) -> Option<&str> {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => Some(name_chirho.text_chirho()),
        TypeChirho::AppChirho { fun_chirho, .. } => target_name_chirho(fun_chirho),
        TypeChirho::ParenChirho { inner_chirho, .. } => target_name_chirho(inner_chirho),
        _ => None,
    }
}

/// Ordinary arguments missing from a kind-signature header still belong in
/// an inline derived instance. Invisible foralls scope names, not arguments.
/// workflow: language-features-chirho/pattern-matching-chirho
pub(super) fn visible_instance_binders_chirho(
    binders_chirho: &[TyVarChirho],
    kind_sig_chirho: Option<&haskelujah_ast_chirho::decl_chirho::DataKindSigChirho>,
    span_chirho: SpanChirho,
) -> Vec<TyVarChirho> {
    let mut visible_chirho = TyVarChirho::visible_binders_chirho(binders_chirho).into_owned();
    let Some(kind_sig_chirho) = kind_sig_chirho else {
        return visible_chirho;
    };
    let (tail_chirho, consumed_chirho) = if let Some(tail_chirho) = kind_sig_chirho.result_chirho()
    {
        (tail_chirho, 0)
    } else if let Some(whole_chirho) = kind_sig_chirho.standalone_chirho() {
        (whole_chirho, visible_chirho.len())
    } else {
        return visible_chirho;
    };
    let remaining_chirho = visible_kind_arity_chirho(tail_chirho).saturating_sub(consumed_chirho);
    let mut used_chirho: std::collections::HashSet<String> = binders_chirho
        .iter()
        .map(|binder_chirho| binder_chirho.name_chirho.text_chirho().to_string())
        .collect();
    collect_kind_names_chirho(tail_chirho, &mut used_chirho);
    let mut index_chirho = 0usize;
    for _ in 0..remaining_chirho {
        let fresh_chirho = loop {
            let candidate_chirho = format!("derived_arg_{index_chirho}_chirho");
            index_chirho += 1;
            if used_chirho.insert(candidate_chirho.clone()) {
                break candidate_chirho;
            }
        };
        visible_chirho.push(TyVarChirho::plain_chirho(NameChirho::RawChirho(
            RawNameChirho::unqualified_chirho(fresh_chirho, span_chirho),
        )));
    }
    visible_chirho
}

fn visible_kind_arity_chirho(kind_chirho: &TypeChirho) -> usize {
    match kind_chirho {
        TypeChirho::FunChirho { result_chirho, .. } => 1 + visible_kind_arity_chirho(result_chirho),
        TypeChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => vars_chirho.len() + visible_kind_arity_chirho(body_chirho),
        TypeChirho::ForallChirho { body_chirho, .. }
        | TypeChirho::QualChirho { body_chirho, .. } => visible_kind_arity_chirho(body_chirho),
        TypeChirho::ParenChirho { inner_chirho, .. } => visible_kind_arity_chirho(inner_chirho),
        _ => 0,
    }
}

fn collect_kind_names_chirho(
    ty_chirho: &TypeChirho,
    names_chirho: &mut std::collections::HashSet<String>,
) {
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho) => {
            names_chirho.insert(name_chirho.text_chirho().to_string());
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            collect_kind_names_chirho(fun_chirho, names_chirho);
            collect_kind_names_chirho(arg_chirho, names_chirho);
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            collect_kind_names_chirho(arg_chirho, names_chirho);
            collect_kind_names_chirho(result_chirho, names_chirho);
        }
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            ..
        }
        | TypeChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            names_chirho.extend(
                vars_chirho
                    .iter()
                    .map(|name_chirho| name_chirho.text_chirho().to_string()),
            );
            collect_kind_names_chirho(body_chirho, names_chirho);
        }
        TypeChirho::QualChirho { body_chirho, .. } => {
            collect_kind_names_chirho(body_chirho, names_chirho)
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            collect_kind_names_chirho(inner_chirho, names_chirho)
        }
        TypeChirho::ListChirho { element_chirho, .. } => {
            collect_kind_names_chirho(element_chirho, names_chirho)
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                collect_kind_names_chirho(element_chirho, names_chirho);
            }
        }
        _ => {}
    }
}
