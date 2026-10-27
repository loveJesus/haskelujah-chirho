// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Which binding a written reference names (row 598).
//!
//! An unqualified name resolves through the scope chain, innermost first. A
//! qualified `Q.x` never takes a local binder, since no `let`, `where`, lambda
//! or `case` binder can be written with a qualifier. Resolving it by its bare
//! text let a local `reverse` capture `L.reverse`, and a local `foo` capture
//! `Main.foo`.
//! workflow: compiler-pipeline-chirho/value-scope-resolution-chirho

use haskelujah_ast_chirho::name_chirho::NameChirho;

use super::DesugarCtxChirho;
use crate::expr_chirho::CoreIdChirho;

impl DesugarCtxChirho {
    /// The name a written reference resolves by, and the id it resolves to.
    ///
    /// - Unqualified: the scope chain.
    /// - Bound under its full spelling, as the seeded Data.Map and QualifiedDo
    ///   aliases are: that binding.
    /// - Qualified by the module's own name: its top-level binding, else a name
    ///   no scope binds, such as one of its own class methods.
    /// - Any other qualifier names an import. The linker addresses an import by
    ///   its bare export name, so the reference skips every scope. Beside a
    ///   top-level binding of the same name, the dictionary pass spells the
    ///   import's body apart (row 597), so both reach the natives.
    ///
    /// workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
    pub(super) fn resolve_written_reference_chirho(
        &mut self,
        name_chirho: &NameChirho,
    ) -> (String, CoreIdChirho) {
        let text_chirho = name_chirho.text_chirho().to_string();
        let Some(qualifier_chirho) = name_chirho.qualifier_chirho() else {
            let id_chirho = self.resolve_var_chirho(&text_chirho);
            return (text_chirho, id_chirho);
        };
        let full_name_chirho = name_chirho.full_name_chirho();
        if self.lookup_scope_chirho(&full_name_chirho).is_some() {
            let id_chirho = self.resolve_var_chirho(&full_name_chirho);
            return (full_name_chirho, id_chirho);
        }
        let own_top_level_chirho =
            if self.current_module_name_chirho.as_deref() == Some(qualifier_chirho) {
                self.scope_chirho
                    .first()
                    .and_then(|scope_chirho| scope_chirho.get(&text_chirho))
                    .copied()
            } else {
                None
            };
        let id_chirho =
            own_top_level_chirho.unwrap_or_else(|| self.resolve_unscoped_var_chirho(&text_chirho));
        (text_chirho, id_chirho)
    }
}
