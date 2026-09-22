// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering is the first producer of occurrences. Each expression reference it
//! builds (a variable, an operator in an infix chain or a section, and the
//! references it generates itself, such as a guard's `error` fallback or the
//! `mfix` of a recursive do) takes a fresh origin here, at construction. Nothing
//! downstream mints or re-enumerates: typing and desugaring read the origin the
//! node carries.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::occurrences_chirho::remint_expr_chirho;
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

    /// A copy of `expr_chirho` placed as NEW code, as a guard's fall-through is
    /// placed once per failing qualifier. The copy's occurrences take fresh
    /// origins; the original keeps its own.
    pub(super) fn duplicate_as_new_code_chirho(&self, expr_chirho: &ExprChirho) -> ExprChirho {
        let mut copy_chirho = expr_chirho.clone();
        remint_expr_chirho(
            &mut copy_chirho,
            &mut self.origin_supply_chirho.borrow_mut(),
        );
        copy_chirho
    }

    /// Hand the supply to the lowered module, which carries it on to the later
    /// producers.
    pub(super) fn take_origin_supply_chirho(&self) -> OriginSupplyChirho {
        self.origin_supply_chirho.take()
    }
}
