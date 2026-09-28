// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Family-instance bodies consume a closed family kind without redeclaring it.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::*;

impl KindInferCtxChirho {
    pub(super) fn check_data_family_instances_chirho(&mut self, module_chirho: &ModuleChirho) {
        let local_contracts_chirho = self.export_kind_contracts_chirho(module_chirho);
        for declaration_chirho in &module_chirho.decls_chirho {
            let DeclChirho::DataFamilyInstanceDeclChirho {
                head_chirho,
                constructors_chirho,
                newtype_chirho,
                deriving_chirho,
                span_chirho,
            } = declaration_chirho
            else {
                continue;
            };
            // Preserving syntax is not implementation of representation erasure or
            // deriving. Do not silently run a newtype as a tagged data constructor.
            if *newtype_chirho || !deriving_chirho.is_empty() {
                self.data_instance_error_chirho(
                    "data-family instance newtype representation and deriving are not implemented",
                    *span_chirho,
                );
                continue;
            }
            let Some((name_chirho, _)) = head_chirho.constructor_application_chirho() else {
                self.data_instance_error_chirho(
                    "data family instance has no represented family head",
                    *span_chirho,
                );
                continue;
            };
            let name_text_chirho = name_chirho.full_name_chirho();
            let shape_chirho = local_contracts_chirho
                .get(&name_text_chirho)
                .map(|contract_chirho| contract_chirho.shape_chirho)
                .or_else(|| {
                    self.imported_kind_shapes_chirho
                        .get(&name_text_chirho)
                        .copied()
                });
            if shape_chirho != Some(imports_chirho::KindHeadShapeChirho::DataFamilyChirho) {
                self.data_instance_error_chirho(
                    &format!("`{name_text_chirho}` is not a data family"),
                    name_chirho.span_chirho(),
                );
                continue;
            }
            let outer_names_chirho = std::mem::take(&mut self.kind_var_cache_chirho);
            self.env_chirho.begin_scope_chirho();
            let residual_chirho = self.infer_type_kind_chirho(head_chirho);
            if constructors_chirho.iter().any(|constructor_chirho| {
                !matches!(
                    constructor_chirho,
                    haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho { .. }
                )
            }) {
                self.check_runtime_kind_chirho(
                    &residual_chirho,
                    "data family instance head",
                    *span_chirho,
                );
            }
            self.check_data_constructors_chirho(&[], constructors_chirho);
            self.env_chirho.end_scope_chirho();
            self.kind_var_cache_chirho = outer_names_chirho;
        }
    }

    fn data_instance_error_chirho(&mut self, message_chirho: &str, span_chirho: SpanChirho) {
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                message_chirho,
                span_chirho,
            ));
    }
}
