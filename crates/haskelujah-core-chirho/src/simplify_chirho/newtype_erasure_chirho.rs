// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Newtype constructor erasure in Core (row 694).
//!
//! A newtype that takes a class through the newtype strategy (GND) reuses its
//! representation type's method rows: the dictionary pass makes
//! `$prim_Show_show_Age` the row of `Int`. That is only sound when an `Age` value IS its
//! field at run time, as in GHC. The interpreter's STG lowering erases every newtype, but
//! the native backends box a constructor as a tag word plus its field, so `showInt#` would
//! receive the box. This pass removes the listed constructors from Core itself, so every
//! backend sees one representation:
//!
//! - `K x` becomes `x`;
//! - an unsaturated `K` becomes `\v -> v`;
//! - `case e of b { K y -> rhs; ... }` becomes `let b = e in let y = b in rhs`. Matching a
//!   newtype constructor never forces the scrutinee, as in GHC, and a newtype has no other
//!   constructor, so the remaining alternatives are unreachable.
//!
//! The caller lists the constructors; nothing here is chosen by spelling.
//! workflow: language-features-chirho/deriving-strategies-chirho

use std::collections::HashSet;

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho, CoreModuleChirho,
};
use crate::transform_chirho::{children_mut_chirho, max_expr_id_chirho};

/// The name given to the binder of an erased constructor's identity function.
const ERASED_FIELD_NAME_CHIRHO: &str = "$newtype_field_chirho";

/// Erase every constructor in `constructors_chirho` from `module_chirho`'s bindings.
/// workflow: language-features-chirho/deriving-strategies-chirho
pub fn erase_newtype_constructors_chirho(
    module_chirho: &mut CoreModuleChirho,
    constructors_chirho: &HashSet<String>,
) {
    if constructors_chirho.is_empty() {
        return;
    }
    let mut maximum_chirho = 0u32;
    for binding_chirho in &module_chirho.bindings_chirho {
        maximum_chirho = maximum_chirho.max(binding_chirho.binder_chirho.id_chirho.0);
        max_expr_id_chirho(&binding_chirho.rhs_chirho, &mut maximum_chirho);
    }
    if let Some(named_maximum_chirho) = module_chirho
        .names_chirho
        .keys()
        .map(|id_chirho| id_chirho.0)
        .max()
    {
        maximum_chirho = maximum_chirho.max(named_maximum_chirho);
    }
    let mut eraser_chirho = NewtypeEraserChirho {
        constructors_chirho,
        next_id_chirho: maximum_chirho + 1,
        minted_chirho: Vec::new(),
    };
    for binding_chirho in &mut module_chirho.bindings_chirho {
        eraser_chirho.erase_chirho(&mut binding_chirho.rhs_chirho);
    }
    for (id_chirho, name_chirho) in eraser_chirho.minted_chirho {
        module_chirho.names_chirho.insert(id_chirho, name_chirho);
    }
}

struct NewtypeEraserChirho<'a> {
    constructors_chirho: &'a HashSet<String>,
    next_id_chirho: u32,
    /// Binders this pass created, for the module's id-to-name table.
    minted_chirho: Vec<(CoreIdChirho, String)>,
}

impl NewtypeEraserChirho<'_> {
    /// Erase below `expression_chirho` first, then the node itself.
    fn erase_chirho(&mut self, expression_chirho: &mut CoreExprChirho) {
        children_mut_chirho(expression_chirho, &mut |child_chirho| {
            self.erase_chirho(child_chirho)
        });
        let replacement_chirho = match expression_chirho {
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } if args_chirho.len() <= 1
                && self.constructors_chirho.contains(con_name_chirho.as_str()) =>
            {
                match args_chirho.pop() {
                    Some(field_chirho) => field_chirho,
                    None => self.identity_chirho(),
                }
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                let Some(position_chirho) = alts_chirho.iter().position(|alt_chirho| {
                    matches!(
                        &alt_chirho.con_chirho,
                        AltConChirho::DataConChirho(name_chirho)
                            if self.constructors_chirho.contains(name_chirho.as_str())
                    )
                }) else {
                    return;
                };
                let alt_chirho = alts_chirho.swap_remove(position_chirho);
                let scrutinee_chirho = std::mem::replace(
                    scrutinee_chirho.as_mut(),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                );
                let case_binder_chirho = bind_chirho.clone();
                let mut body_chirho = alt_chirho.rhs_chirho;
                if let Some(field_binder_chirho) = alt_chirho.binders_chirho.into_iter().next() {
                    body_chirho = let_chirho(
                        field_binder_chirho,
                        CoreExprChirho::VarChirho(case_binder_chirho.id_chirho),
                        body_chirho,
                    );
                }
                let_chirho(case_binder_chirho, scrutinee_chirho, body_chirho)
            }
            _ => return,
        };
        *expression_chirho = replacement_chirho;
    }

    /// `\v -> v`, for a constructor used as a function value.
    fn identity_chirho(&mut self) -> CoreExprChirho {
        let id_chirho = CoreIdChirho(self.next_id_chirho);
        self.next_id_chirho += 1;
        self.minted_chirho
            .push((id_chirho, ERASED_FIELD_NAME_CHIRHO.to_string()));
        CoreExprChirho::LamChirho {
            binder_chirho: BinderChirho {
                id_chirho,
                name_chirho: ERASED_FIELD_NAME_CHIRHO.to_string(),
                ty_chirho: TyChirho::VarChirho(TyVarChirho(id_chirho.0)),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            body_chirho: Box::new(CoreExprChirho::VarChirho(id_chirho)),
        }
    }
}

/// A lazy, non-recursive `let binder = rhs in body`.
fn let_chirho(
    binder_chirho: BinderChirho,
    rhs_chirho: CoreExprChirho,
    body_chirho: CoreExprChirho,
) -> CoreExprChirho {
    CoreExprChirho::LetChirho {
        rec_chirho: false,
        binds_chirho: vec![(binder_chirho, rhs_chirho)],
        body_chirho: Box::new(body_chirho),
    }
}
