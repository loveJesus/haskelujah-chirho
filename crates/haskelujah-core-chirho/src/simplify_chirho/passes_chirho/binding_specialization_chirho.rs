// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! SPECIALIZE binding specialization, phase 4 of the Core simplifier, moved from
//! `simplify_chirho.rs` (row 700).

use crate::expr_chirho::{BinderChirho, CoreBindingChirho, CoreIdChirho, InlineAnnotationChirho};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Phase 4: Specialization — create monomorphized binding copies
// ---------------------------------------------------------------------------

/// For each `{-# SPECIALIZE f :: Type #-}` pragma, clone the original binding
/// of `f`, rename it to `$spec_f_<hash>`, and mark it for aggressive inlining.
/// This allows the simplifier's subsequent passes to inline and optimize the
/// specialized copy without dictionary indirection.
pub(super) fn specialize_bindings_chirho(
    mut bindings_chirho: Vec<CoreBindingChirho>,
    specialize_pragmas_chirho: &HashMap<String, Vec<String>>,
    names_chirho: &HashMap<CoreIdChirho, String>,
) -> Vec<CoreBindingChirho> {
    // Build name→index map for quick lookup
    let name_to_idx_chirho: HashMap<&str, usize> = bindings_chirho
        .iter()
        .enumerate()
        .map(|(idx_chirho, b_chirho)| (b_chirho.binder_chirho.name_chirho.as_str(), idx_chirho))
        .collect();

    // Also build a reverse map from CoreId → name for any binding (reserved
    // for future type-aware specialization that needs to look up binder names)
    let _id_to_name_chirho: HashMap<CoreIdChirho, &str> = bindings_chirho
        .iter()
        .map(|b_chirho| {
            (
                b_chirho.binder_chirho.id_chirho,
                b_chirho.binder_chirho.name_chirho.as_str(),
            )
        })
        .chain(
            names_chirho
                .iter()
                .map(|(id_chirho, name_chirho)| (*id_chirho, name_chirho.as_str())),
        )
        .collect();

    let mut new_bindings_chirho = Vec::new();

    for (func_name_chirho, spec_types_chirho) in specialize_pragmas_chirho {
        // Find the original binding
        let idx_chirho =
            if let Some(&idx_chirho) = name_to_idx_chirho.get(func_name_chirho.as_str()) {
                idx_chirho
            } else {
                continue; // Binding not found, skip
            };

        let original_chirho = &bindings_chirho[idx_chirho];

        for (spec_idx_chirho, spec_type_chirho) in spec_types_chirho.iter().enumerate() {
            // Create a unique name for the specialized binding
            let spec_name_chirho = format!("$spec_{}_{}", func_name_chirho, spec_idx_chirho);

            // Generate a new CoreId for the specialized binding
            // Use a high offset to avoid collisions
            let spec_id_chirho = CoreIdChirho(
                original_chirho
                    .binder_chirho
                    .id_chirho
                    .0
                    .wrapping_add(10000 + spec_idx_chirho as u32),
            );

            // Clone the original binding with a new name and INLINE annotation
            let spec_binding_chirho = CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: spec_id_chirho,
                    name_chirho: spec_name_chirho.clone(),
                    ty_chirho: original_chirho.binder_chirho.ty_chirho.clone(),
                    span_chirho: original_chirho.binder_chirho.span_chirho,
                },
                rhs_chirho: original_chirho.rhs_chirho.clone(),
                is_rec_chirho: original_chirho.is_rec_chirho,
                // Mark specialized copies for aggressive inlining
                inline_chirho: InlineAnnotationChirho::AlwaysChirho,
            };

            new_bindings_chirho.push((
                spec_name_chirho,
                spec_type_chirho.clone(),
                spec_binding_chirho,
            ));
        }
    }

    // Append all specialized bindings to the module
    for (_name_chirho, _ty_chirho, binding_chirho) in new_bindings_chirho {
        bindings_chirho.push(binding_chirho);
    }

    bindings_chirho
}
