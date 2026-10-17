// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Written and implicit deriving heads, isolated from the per-class generators.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::*;

pub(super) fn derive_standalone_instances_chirho(
    module_chirho: &ModuleChirho,
    instances_chirho: &mut Vec<DeclChirho>,
    warnings_chirho: &mut Vec<String>,
) {
    // Process standalone deriving declarations: `deriving instance Show Foo`
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::StandaloneDerivingDeclChirho {
            context_chirho,
            class_chirho,
            types_chirho,
            span_chirho,
            ..
        } = decl_chirho
        {
            let type_name_chirho = types_chirho.first().and_then(target_name_chirho);

            if let Some(target_chirho) = type_name_chirho {
                let first_generated_chirho = instances_chirho.len();
                let found_chirho = derive_standalone_for_type_chirho(
                    module_chirho,
                    class_chirho,
                    target_chirho,
                    *span_chirho,
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
    class_chirho: &NameChirho,
    target_chirho: &str,
    span_chirho: SpanChirho,
    instances_chirho: &mut Vec<DeclChirho>,
    warnings_chirho: &mut Vec<String>,
) -> bool {
    let class_text_chirho = class_chirho.text_chirho();

    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                constructors_chirho,
                ..
            } if name_chirho.text_chirho() == target_chirho => {
                let visible_vars_chirho = TyVarChirho::visible_binders_chirho(type_vars_chirho);
                derive_class_for_data_chirho(
                    class_text_chirho,
                    name_chirho,
                    visible_vars_chirho.as_ref(),
                    constructors_chirho,
                    span_chirho,
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
                derive_class_for_data_chirho(
                    class_text_chirho,
                    name_chirho,
                    visible_vars_chirho.as_ref(),
                    &cons_chirho,
                    span_chirho,
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

/// Derive a specific class for a data type (shared by inline deriving and standalone).
fn derive_class_for_data_chirho(
    class_text_chirho: &str,
    name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    span_chirho: SpanChirho,
    instances_chirho: &mut Vec<DeclChirho>,
    warnings_chirho: &mut Vec<String>,
) {
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
        "Typeable" | "Data" | "Lift" | "NFData" => {
            // Generate empty instance for these well-known classes
        }
        other_chirho => {
            warnings_chirho.push(format!(
                "standalone deriving {} not yet supported for {}",
                other_chirho,
                name_chirho.text_chirho()
            ));
        }
    }
}

fn target_name_chirho(ty_chirho: &TypeChirho) -> Option<&str> {
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
