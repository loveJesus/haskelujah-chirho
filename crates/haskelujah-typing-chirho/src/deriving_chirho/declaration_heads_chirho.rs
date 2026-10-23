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
                    &class_text_chirho,
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
                    &class_text_chirho,
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
