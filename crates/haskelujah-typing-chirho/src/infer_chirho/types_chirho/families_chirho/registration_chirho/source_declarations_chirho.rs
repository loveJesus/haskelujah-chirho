// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded declaration projection for cold kind checking. No value body or
//! original origin supply is copied; this private module never mints uses.

use haskelujah_ast_chirho::decl_chirho::{
    ClassMethodChirho, DeclChirho, GadtProjectionBudgetChirho, check_declaration_projection_chirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho};
use std::collections::HashMap;

pub(super) fn declaration_module_chirho(
    module_chirho: &ModuleChirho,
    signature_headers_chirho: bool,
) -> Result<ModuleChirho, DiagnosticBundleChirho> {
    let error_chirho = |message_chirho: String| {
        DiagnosticBundleChirho::new_chirho(vec![DiagnosticChirho::error_with_code_chirho(
            haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300),
            message_chirho,
            module_chirho.span_chirho,
        )])
    };
    let metadata_size_chirho = module_chirho.extensions_chirho.iter().try_fold(
        module_chirho.name_chirho.text_chirho().len(),
        |size_chirho, extension_chirho| size_chirho.checked_add(extension_chirho.len()),
    );
    if metadata_size_chirho.is_none_or(|size_chirho| size_chirho > 1 << 20) {
        return Err(error_chirho(
            "cold declaration metadata size bound exhausted".to_owned(),
        ));
    }
    let mut budget_chirho = GadtProjectionBudgetChirho::new_chirho();
    let mut declarations_chirho = Vec::new();
    for declaration_chirho in &module_chirho.decls_chirho {
        if !(matches!(
            declaration_chirho,
            DeclChirho::DataDeclChirho { .. }
                | DeclChirho::NewtypeDeclChirho { .. }
                | DeclChirho::TypeAliasDeclChirho { .. }
                | DeclChirho::TypeFamilyDeclChirho { .. }
                | DeclChirho::TypeFamilyInstanceDeclChirho { .. }
                | DeclChirho::ClassDeclChirho { .. }
                | DeclChirho::InstanceDeclChirho { .. }
        ) || signature_headers_chirho
            && matches!(
                declaration_chirho,
                DeclChirho::TypeSigChirho { .. } | DeclChirho::PatSynSigDeclChirho { .. }
            ))
        {
            continue;
        }
        check_declaration_projection_chirho(declaration_chirho, &mut budget_chirho, 1)
            .map_err(|failure_chirho| error_chirho(failure_chirho.to_string()))?;
        let declaration_chirho = match declaration_chirho {
            DeclChirho::ClassDeclChirho {
                context_chirho,
                name_chirho,
                type_vars_chirho,
                methods_chirho,
                associated_tfs_chirho,
                fundeps_chirho,
                span_chirho,
            } => DeclChirho::ClassDeclChirho {
                context_chirho: context_chirho.clone(),
                name_chirho: name_chirho.clone(),
                type_vars_chirho: type_vars_chirho.clone(),
                methods_chirho: methods_chirho
                    .iter()
                    .map(|method_chirho| ClassMethodChirho {
                        name_chirho: method_chirho.name_chirho.clone(),
                        ty_chirho: method_chirho.ty_chirho.clone(),
                        default_chirho: None,
                        default_sig_chirho: None,
                        span_chirho: method_chirho.span_chirho,
                    })
                    .collect(),
                associated_tfs_chirho: associated_tfs_chirho.clone(),
                fundeps_chirho: fundeps_chirho.clone(),
                span_chirho: *span_chirho,
            },
            DeclChirho::InstanceDeclChirho {
                kind_args_chirho,
                context_chirho,
                class_chirho,
                types_chirho,
                assoc_tf_instances_chirho,
                span_chirho,
                ..
            } => DeclChirho::InstanceDeclChirho {
                kind_args_chirho: kind_args_chirho.clone(),
                context_chirho: context_chirho.clone(),
                class_chirho: class_chirho.clone(),
                types_chirho: types_chirho.clone(),
                methods_chirho: Vec::new(),
                assoc_tf_instances_chirho: assoc_tf_instances_chirho.clone(),
                span_chirho: *span_chirho,
            },
            declaration_chirho => declaration_chirho.clone(),
        };
        declarations_chirho.push(declaration_chirho);
    }
    Ok(ModuleChirho {
        name_chirho: module_chirho.name_chirho.clone(),
        exports_chirho: None,
        imports_chirho: Vec::new(),
        decls_chirho: declarations_chirho,
        extensions_chirho: module_chirho.extensions_chirho.clone(),
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: Vec::new(),
        deriving_via_chirho: Vec::new(),
        span_chirho: module_chirho.span_chirho,
        origin_supply_chirho: Default::default(),
    })
}
