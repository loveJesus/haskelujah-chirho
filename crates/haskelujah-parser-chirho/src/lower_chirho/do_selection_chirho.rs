// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lowering chooses the operation each do statement selects, once.
//!
//! A do statement is written without its operator, so somebody has to say which
//! binding it stands for. Lowering is the right somebody: it knows the block's
//! qualifier (`M.do` selects `M.>>=`), which statement is the tail, and what the
//! pattern is. Checking and desugaring then READ that choice instead of making
//! their own, because two phases that select independently can disagree and the
//! disagreement is invisible until something one of them emits fails to resolve.
//!
//! Two things are deliberately NOT decided here.
//!
//! Whether a qualified name resolves to a local binding is name RESOLUTION, not
//! selection: `M.do` in module `M` still selects `M.>>=`, and the resolver's own
//! rule for a self-qualified name stays in the resolver, where it already lives.
//! So the selection carries a properly qualified name and nothing more.
//!
//! Whether a bind actually selects `fail` cannot be known here at all: it depends
//! on how many constructors the pattern's type has, which needs an environment
//! lowering does not have. So an identity is RESERVED for it, and the pass that
//! does have the constructor environment clears it wherever the pattern cannot
//! fail (gpt_chirho #24761: reserve before failability is known, publish and use
//! only when selected). A reserved-then-cleared origin costs nothing: no evidence
//! is ever keyed to it, and the occurrence walk only visits a selection that is
//! present.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::expr_chirho::StmtChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::provenance_chirho::OccurrenceRoleChirho;
use haskelujah_ast_chirho::stmt_operation_chirho::SelectedOperationChirho;
use haskelujah_span_chirho::SpanChirho;

use super::LowerCtxChirho;

impl LowerCtxChirho {
    /// Record what each statement of one do block selects. A tail expression and a
    /// `let` select nothing, which is why this writes `None` for them rather than
    /// leaving whatever was there: a statement that selects nothing must say so.
    ///
    /// Called only for do blocks. A list comprehension's qualifiers are the same
    /// `StmtChirho` type but select no monadic operation, and never reach here.
    pub(super) fn select_do_operations_chirho(
        &self,
        stmts_chirho: &mut [StmtChirho],
        qualifier_chirho: Option<&str>,
    ) {
        let last_index_chirho = stmts_chirho.len().saturating_sub(1);
        for (index_chirho, stmt_chirho) in stmts_chirho.iter_mut().enumerate() {
            let is_tail_chirho = index_chirho == last_index_chirho;
            match stmt_chirho {
                StmtChirho::ExprChirho {
                    expr_chirho,
                    then_chirho,
                } => {
                    let span_chirho = expr_chirho.span_chirho();
                    *then_chirho = (!is_tail_chirho).then(|| {
                        self.selected_operation_chirho(
                            ">>",
                            qualifier_chirho,
                            OccurrenceRoleChirho::Then,
                            span_chirho,
                        )
                    });
                }
                StmtChirho::BindChirho {
                    bind_chirho,
                    fail_chirho,
                    span_chirho,
                    ..
                } => {
                    let span_chirho = *span_chirho;
                    *bind_chirho = Some(self.selected_operation_chirho(
                        ">>=",
                        qualifier_chirho,
                        OccurrenceRoleChirho::Bind,
                        span_chirho,
                    ));
                    // Reserved, not yet selected: see the note at the top.
                    *fail_chirho = Some(self.selected_operation_chirho(
                        "fail",
                        qualifier_chirho,
                        OccurrenceRoleChirho::Fail,
                        span_chirho,
                    ));
                }
                // A `let` introduces no operation at all.
                StmtChirho::LetChirho { .. } => {}
            }
        }
    }

    /// One selected operation: the binding as it must be looked up, carrying a
    /// fresh origin of its own, and the role this use plays.
    fn selected_operation_chirho(
        &self,
        method_chirho: &str,
        qualifier_chirho: Option<&str>,
        role_chirho: OccurrenceRoleChirho,
        span_chirho: SpanChirho,
    ) -> SelectedOperationChirho {
        let raw_chirho = match qualifier_chirho {
            Some(qualifier_chirho) => {
                RawNameChirho::qualified_chirho(qualifier_chirho, method_chirho, span_chirho)
            }
            None => RawNameChirho::unqualified_chirho(method_chirho, span_chirho),
        };
        SelectedOperationChirho::new_chirho(
            NameChirho::RawChirho(raw_chirho).with_origin_chirho(self.fresh_origin_chirho()),
            role_chirho,
        )
    }
}
