// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GND method coercibility (row 694, #31784).
//!
//! The dictionary pass gives a newtype that takes a class through the newtype strategy its
//! representation's method rows. Deriving provenance says which instances those are; it does
//! not say the methods may be shared. This check proves, for each such instance, that every
//! method of the class has one representation at the newtype and at its representation, from
//! the class environment's method schemes and the module's own parameter roles. A solved
//! `Coercible` want proves nothing here: the solver accepts it without roles.
//!
//! The result is a typed proof per (newtype, class), which only this module constructs. The
//! inference result carries it, the driver gives it to `NewtypeInfoChirho`, and the alias loop
//! requires it. An instance with no proof is refused here, before any alias exists:
//!
//! - a method whose class parameter stands under a local type's nominal parameter is GHC's
//!   refusal, GHC-18872 (R1, R2);
//! - a class with an associated family, until GND forwards `type F N = F rep` (R5's valid twin
//!   prints 8 in GHC; the original needs UndecidableInstances, GHC-22979), a multi-parameter
//!   class, and a position whose role the module cannot prove are named Unsupported gaps.
//!
//! A method that never mentions the class parameter needs no proof of it (R4).
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

mod method_roles_chirho;
mod parameter_roles_chirho;

#[cfg(test)]
mod gnd_coercion_tests_chirho;

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::BuiltinClassScopeChirho;
use haskelujah_diagnostics_chirho::DiagnosticBundleChirho;

use crate::class_chirho::ClassEnvChirho;
use crate::deriving_chirho::GndRequestChirho;
use crate::validity_chirho::declarations_chirho::{
    DeclarationErrorChirho, UNSUPPORTED_SHAPE_CHIRHO, declaration_error_diagnostics_chirho,
};

use method_roles_chirho::blocked_method_position_chirho;
pub use parameter_roles_chirho::{
    ParameterRoleChirho, ParameterRolesChirho, local_parameter_roles_chirho,
};

/// A typing proof that every method of one class has one representation at one newtype and
/// at its representation, so the newtype may take the representation's rows (row 694).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GndCoercionProofChirho {
    newtype_chirho: String,
    class_chirho: String,
    methods_chirho: Vec<String>,
}

impl GndCoercionProofChirho {
    /// The newtype the proof is for.
    pub fn newtype_chirho(&self) -> &str {
        &self.newtype_chirho
    }

    /// The class the proof is for, as the class environment names it.
    pub fn class_chirho(&self) -> &str {
        &self.class_chirho
    }

    /// Whether the proof covers `method_chirho`.
    pub fn proves_method_chirho(&self, method_chirho: &str) -> bool {
        self.methods_chirho
            .iter()
            .any(|proven_chirho| proven_chirho == method_chirho)
    }
}

/// Prove every GND instance the module's declarations request, judged with the builtin
/// classes naming proved for it and the class environment inference built. Each refusal is
/// a declaration error at the requested class.
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
pub fn prove_gnd_coercions_chirho(
    module_chirho: &ModuleChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
    class_env_chirho: &ClassEnvChirho,
) -> Result<Vec<GndCoercionProofChirho>, DiagnosticBundleChirho> {
    let requests_chirho = crate::deriving_chirho::newtype_strategy_requests_chirho(
        module_chirho,
        builtin_classes_chirho,
    );
    if requests_chirho.is_empty() {
        return Ok(Vec::new());
    }
    let roles_chirho = local_parameter_roles_chirho(module_chirho);
    let associated_chirho = associated_families_chirho(module_chirho);
    let mut proofs_chirho = Vec::new();
    let mut errors_chirho = Vec::new();
    for request_chirho in &requests_chirho {
        match prove_request_chirho(
            request_chirho,
            class_env_chirho,
            &roles_chirho,
            &associated_chirho,
        ) {
            Ok(proof_chirho) => proofs_chirho.push(proof_chirho),
            Err(error_chirho) => errors_chirho.push(error_chirho),
        }
    }
    if errors_chirho.is_empty() {
        Ok(proofs_chirho)
    } else {
        Err(declaration_error_diagnostics_chirho(errors_chirho))
    }
}

/// Each local class with an associated family, by class name: its first family's name.
fn associated_families_chirho(module_chirho: &ModuleChirho) -> HashMap<String, String> {
    module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::ClassDeclChirho {
                name_chirho,
                associated_tfs_chirho,
                ..
            } => associated_tfs_chirho.first().map(|family_chirho| {
                (
                    name_chirho.text_chirho().to_string(),
                    family_chirho.name_chirho.text_chirho().to_string(),
                )
            }),
            _ => None,
        })
        .collect()
}

/// The proof for one requested instance, or the refusal at its class.
fn prove_request_chirho(
    request_chirho: &GndRequestChirho,
    class_env_chirho: &ClassEnvChirho,
    roles_chirho: &ParameterRolesChirho,
    associated_chirho: &HashMap<String, String>,
) -> Result<GndCoercionProofChirho, DeclarationErrorChirho> {
    let class_text_chirho = request_chirho.class_chirho.text_chirho();
    let head_chirho = format!("`{class_text_chirho} {}`", request_chirho.newtype_chirho);
    let unsupported_chirho = |reason_chirho: String| DeclarationErrorChirho {
        ghc_code_chirho: UNSUPPORTED_SHAPE_CHIRHO,
        message_chirho: format!(
            "Deriving {head_chirho} through the newtype strategy is not supported yet: {reason_chirho}"
        ),
        span_chirho: request_chirho.class_chirho.span_chirho(),
        related_span_chirho: None,
    };
    let Some(class_decl_chirho) = class_env_chirho.classes_chirho.get(class_text_chirho) else {
        return Err(unsupported_chirho(
            "the checker knows no such class".to_string(),
        ));
    };
    if class_decl_chirho.is_mptc_chirho() {
        return Err(unsupported_chirho(
            "the class has several parameters".to_string(),
        ));
    }
    if let Some(family_chirho) = associated_chirho.get(class_text_chirho) {
        return Err(unsupported_chirho(format!(
            "the newtype strategy does not forward the associated family `{family_chirho}` yet"
        )));
    }
    let mut methods_chirho: Vec<&String> = class_decl_chirho.methods_chirho.keys().collect();
    methods_chirho.sort();
    for method_chirho in &methods_chirho {
        let scheme_chirho = &class_decl_chirho.methods_chirho[*method_chirho];
        let Some(blocked_chirho) = blocked_method_position_chirho(
            scheme_chirho,
            class_text_chirho,
            class_decl_chirho.var_chirho,
            roles_chirho,
        ) else {
            continue;
        };
        if blocked_chirho.proven_nominal_chirho {
            return Err(DeclarationErrorChirho {
                ghc_code_chirho: "GHC-18872",
                message_chirho: format!(
                    "Couldn't match the representation of `{}` with its representation type, arising from the coercion of the method `{method_chirho}` for {head_chirho}: the class parameter stands under {}, whose parameter is nominal",
                    request_chirho.newtype_chirho, blocked_chirho.under_chirho
                ),
                span_chirho: request_chirho.class_chirho.span_chirho(),
                related_span_chirho: None,
            });
        }
        return Err(unsupported_chirho(format!(
            "Haskelujah cannot prove the method `{method_chirho}` coercible, because the class parameter stands under {}",
            blocked_chirho.under_chirho
        )));
    }
    Ok(GndCoercionProofChirho {
        newtype_chirho: request_chirho.newtype_chirho.clone(),
        class_chirho: class_text_chirho.to_string(),
        methods_chirho: methods_chirho.into_iter().cloned().collect(),
    })
}
