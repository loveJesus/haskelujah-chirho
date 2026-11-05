// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The newtype deriving strategy (GND): a newtype's instance is its representation's.
//!
//! For `newtype N v1 .. vn = K rep` deriving `C`, the instance is `C rep => C (N ..)` with
//! no methods: the dictionary pass gives it the representation's method rows, and only for
//! classes the newtype's own declarations route here (row 694). When `C`'s last parameter
//! takes `k` type arguments (`Functor` takes one), GHC drops the last `k` variables from the
//! head and the same `k` from the representation: `newtype App a = App (IO a)` derives
//! `Functor IO => Functor App`. A representation that does not end in exactly those
//! variables cannot be reduced, and nothing is derived (GHC rejects it, GHC-38996).
//! workflow: language-features-chirho/deriving-strategies-chirho

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, TyVarChirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};

use super::{
    extract_gadt_args_chirho, gen_span_chirho, instance_type_chirho, type_is_var_chirho,
    type_mentions_var_chirho, var_name_chirho,
};

/// The GND instance of `class_name_chirho` for a newtype, its head reduced by
/// `eta_arity_chirho` trailing variables. `None` when the representation cannot drop them.
/// workflow: language-features-chirho/deriving-strategies-chirho
pub(super) fn derive_newtype_instance_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructor_chirho: &ConDeclChirho,
    class_name_chirho: &NameChirho,
    eta_arity_chirho: usize,
) -> Option<DeclChirho> {
    let kept_count_chirho = type_vars_chirho.len().checked_sub(eta_arity_chirho)?;
    let (kept_vars_chirho, dropped_vars_chirho) = type_vars_chirho.split_at(kept_count_chirho);
    let mut representation_chirho = representation_type_chirho(constructor_chirho);
    for dropped_chirho in dropped_vars_chirho.iter().rev() {
        representation_chirho =
            drop_last_argument_chirho(representation_chirho, dropped_chirho.text_chirho())?;
    }
    if dropped_vars_chirho.iter().any(|dropped_chirho| {
        type_mentions_var_chirho(&representation_chirho, dropped_chirho.text_chirho())
    }) {
        return None;
    }
    Some(DeclChirho::InstanceDeclChirho {
        context_chirho: vec![ConstraintChirho::ClassChirho {
            class_chirho: class_name_chirho.clone(),
            args_chirho: vec![representation_chirho],
            span_chirho: gen_span_chirho(),
        }],
        class_chirho: class_name_chirho.clone(),
        types_chirho: vec![instance_type_chirho(type_name_chirho, kept_vars_chirho)],
        // The dictionary pass gives this instance the representation's method rows.
        methods_chirho: vec![],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    })
}

/// The type a newtype's constructor wraps: its single field, or a GADT signature's first
/// argument. A constructor lowering could not read wraps `()`.
fn representation_type_chirho(constructor_chirho: &ConDeclChirho) -> TypeChirho {
    let unit_chirho = || TypeChirho::ConChirho(var_name_chirho("()"));
    match constructor_chirho {
        ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => fields_chirho
            .first()
            .map(|(_strictness_chirho, ty_chirho)| ty_chirho.clone())
            .unwrap_or_else(unit_chirho),
        ConDeclChirho::RecordChirho { fields_chirho, .. } => fields_chirho
            .first()
            .map(|field_chirho| field_chirho.ty_chirho.clone())
            .unwrap_or_else(unit_chirho),
        ConDeclChirho::GadtChirho { ty_chirho, .. } => extract_gadt_args_chirho(ty_chirho)
            .into_iter()
            .next()
            .unwrap_or_else(unit_chirho),
    }
}

/// `f .. var` without its last argument, when that argument is exactly `var`.
fn drop_last_argument_chirho(ty_chirho: TypeChirho, var_chirho: &str) -> Option<TypeChirho> {
    match ty_chirho {
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            drop_last_argument_chirho(*inner_chirho, var_chirho)
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } if type_is_var_chirho(&arg_chirho, var_chirho) => Some(*fun_chirho),
        _ => None,
    }
}
