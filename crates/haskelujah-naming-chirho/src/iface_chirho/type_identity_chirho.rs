// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Admitted ordinary type identities and declaration categories. Constructor
//! membership never supplies a declaration role.
//! Workflow: compiler-pipeline-chirho/type-scope-resolution-chirho#ordinary-type-definition-proof-chirho.

use std::collections::HashSet;

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{
    TypeConstructorDeclarationChirho, TypeConstructorRoleChirho, TypeConstructorScopeChirho,
};

use super::ModuleIfaceChirho;
use crate::env_chirho::NamespaceChirho;
use crate::promotion_scope_chirho::{declared_type_names_chirho, spelling_chirho};
use crate::resolve_chirho::compute_imported_names_chirho;
use crate::type_exports_chirho::source_type_declaration_chirho;

/// Build one scope from producer-backed definitions and exactly the type names
/// each import admits. A source alias is lookup syntax, never a defining origin.
/// A local declaration owns its bare spelling; qualified import conflicts and
/// unproven entries remain unknown regardless of import order.
pub fn build_type_constructor_scope_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> TypeConstructorScopeChirho {
    let mut scope_chirho = TypeConstructorScopeChirho::new_chirho();
    let mut local_names_chirho = HashSet::new();
    let module_name_chirho = module_chirho.name_chirho.full_name_chirho();
    for declaration_chirho in &module_chirho.decls_chirho {
        for (index_chirho, name_chirho) in declared_type_names_chirho(declaration_chirho)
            .into_iter()
            .enumerate()
        {
            if name_chirho.is_empty() {
                continue;
            }
            local_names_chirho.insert(name_chirho.to_owned());
            let account_chirho = if index_chirho == 0 {
                source_type_declaration_chirho(module_chirho, declaration_chirho)
            } else if let DeclChirho::ClassDeclChirho {
                associated_tfs_chirho,
                ..
            } = declaration_chirho
            {
                associated_tfs_chirho
                    .iter()
                    .find(|family_chirho| family_chirho.name_chirho.text_chirho() == name_chirho)
                    .map(|family_chirho| {
                        TypeConstructorDeclarationChirho::source_chirho(
                            &module_name_chirho,
                            name_chirho,
                            if family_chirho.data_family_chirho {
                                TypeConstructorRoleChirho::DataFamilyChirho
                            } else {
                                TypeConstructorRoleChirho::TypeFamilyChirho
                            },
                        )
                    })
            } else {
                None
            };
            scope_chirho.bind_chirho(name_chirho.to_owned(), account_chirho.clone());
            scope_chirho.bind_chirho(
                spelling_chirho(Some(&module_name_chirho), name_chirho),
                account_chirho,
            );
        }
    }
    for import_chirho in &module_chirho.imports_chirho {
        let imported_module_chirho = import_chirho.module_chirho.full_name_chirho();
        let Some(iface_chirho) =
            crate::iface_chirho::select_import_iface_chirho(ifaces_chirho, import_chirho)
        else {
            continue;
        };
        let qualifier_chirho = import_chirho
            .alias_chirho
            .as_ref()
            .map(|alias_chirho| alias_chirho.full_name_chirho())
            .unwrap_or(imported_module_chirho);
        for (name_chirho, namespace_chirho, _) in
            compute_imported_names_chirho(&iface_chirho.exports_chirho, &import_chirho.spec_chirho)
        {
            if namespace_chirho != NamespaceChirho::TypeChirho {
                continue;
            }
            let account_chirho = iface_chirho
                .exports_chirho
                .types_chirho
                .get(&name_chirho)
                .and_then(|ty_chirho| ty_chirho.declaration_chirho.clone());
            scope_chirho.bind_chirho(
                spelling_chirho(Some(&qualifier_chirho), &name_chirho),
                account_chirho.clone(),
            );
            if !import_chirho.qualified_chirho && !local_names_chirho.contains(&name_chirho) {
                scope_chirho.bind_chirho(name_chirho, account_chirho);
            }
        }
    }
    scope_chirho
}

#[cfg(test)]
#[path = "type_identity_tests_chirho.rs"]
mod tests_chirho;
