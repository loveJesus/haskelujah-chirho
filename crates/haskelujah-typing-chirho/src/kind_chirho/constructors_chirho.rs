// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One head/recursion/constructor/publication lifecycle for data and newtype.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::{KindInferCtxChirho, TyVarChirho};
use haskelujah_ast_chirho::decl_chirho::ConDeclChirho;

impl KindInferCtxChirho {
    pub(super) fn check_data_constructors_chirho(
        &mut self,
        type_vars_chirho: &[TyVarChirho],
        constructors_chirho: &[ConDeclChirho],
    ) {
        for constructor_chirho in constructors_chirho {
            match constructor_chirho {
                ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
                    for (_, field_chirho) in fields_chirho {
                        let kind_chirho = self.infer_type_kind_chirho(field_chirho);
                        self.check_runtime_kind_chirho(
                            &kind_chirho,
                            "data constructor field",
                            field_chirho.span_chirho(),
                        );
                    }
                }
                ConDeclChirho::RecordChirho { fields_chirho, .. } => {
                    for field_chirho in fields_chirho {
                        let kind_chirho = self.infer_type_kind_chirho(&field_chirho.ty_chirho);
                        self.check_runtime_kind_chirho(
                            &kind_chirho,
                            "data constructor field",
                            field_chirho.ty_chirho.span_chirho(),
                        );
                    }
                }
                ConDeclChirho::GadtChirho { ty_chirho, .. } => {
                    // GADT constructor type variables are independently quantified;
                    // declaration-head names do not scope over their signatures.
                    self.env_chirho.begin_scope_chirho();
                    for variable_chirho in type_vars_chirho {
                        self.env_chirho.hide_chirho(variable_chirho.text_chirho());
                    }
                    let outer_names_chirho = std::mem::take(&mut self.kind_var_cache_chirho);
                    let kind_chirho = self.infer_type_kind_chirho(ty_chirho);
                    self.check_runtime_kind_chirho(
                        &kind_chirho,
                        "GADT constructor type",
                        ty_chirho.span_chirho(),
                    );
                    self.kind_var_cache_chirho = outer_names_chirho;
                    self.env_chirho.end_scope_chirho();
                }
            }
        }
    }
}

pub(super) fn cusks_enabled_chirho(extensions_chirho: &[String]) -> bool {
    extensions_chirho.iter().fold(
        false,
        |enabled_chirho, extension_chirho| match extension_chirho.as_str() {
            "NoCUSKs" | "StandaloneKindSignatures" | "GHC2021" | "GHC2024" => false,
            "CUSKs" | "Haskell98" | "Haskell2010" => true,
            _ => enabled_chirho,
        },
    )
}

/// Match the default GHC2021 edition used by the other front-end passes.
/// Explicit legacy editions and NoPolyKinds still take effect in source order.
pub(super) fn poly_kinds_enabled_chirho(extensions_chirho: &[String]) -> bool {
    extensions_chirho.iter().fold(
        true,
        |enabled_chirho, extension_chirho| match extension_chirho.as_str() {
            "Haskell98" | "Haskell2010" | "NoPolyKinds" => false,
            "GHC2021" | "GHC2024" | "PolyKinds" | "TypeInType" => true,
            _ => enabled_chirho,
        },
    )
}
