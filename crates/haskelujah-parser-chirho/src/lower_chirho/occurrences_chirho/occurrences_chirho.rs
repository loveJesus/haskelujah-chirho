// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering is the first producer of occurrences. Each expression reference it
//! builds (a variable, an operator in an infix chain or a section, and the
//! references it generates itself, such as the `mfix` of a recursive do) takes
//! a fresh origin here, at construction. Nothing downstream mints or
//! re-enumerates: typing and desugaring read the origin the node carries.
//! Lowering places no code twice: a guard's qualifiers stay qualifiers, and
//! the desugarer joins its fall-through once.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::provenance_chirho::{OriginIdChirho, OriginSupplyChirho};

use super::LowerCtxChirho;

impl LowerCtxChirho {
    /// A fresh origin from the module's supply.
    pub(super) fn fresh_origin_chirho(&self) -> OriginIdChirho {
        self.origin_supply_chirho.borrow_mut().fresh_chirho()
    }

    /// `name_chirho` as a new reference occurrence, with its own origin.
    pub(super) fn reference_name_chirho(&self, name_chirho: NameChirho) -> NameChirho {
        name_chirho.with_origin_chirho(self.fresh_origin_chirho())
    }

    /// A variable expression that is a new reference occurrence.
    pub(super) fn reference_expr_chirho(&self, name_chirho: NameChirho) -> ExprChirho {
        ExprChirho::VarChirho(self.reference_name_chirho(name_chirho))
    }

    /// Hand the supply to the lowered module, which carries it on to the later
    /// producers.
    pub(super) fn take_origin_supply_chirho(&self) -> OriginSupplyChirho {
        self.origin_supply_chirho.take()
    }
}
