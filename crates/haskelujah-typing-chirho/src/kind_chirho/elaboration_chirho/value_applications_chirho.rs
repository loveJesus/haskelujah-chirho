// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor value applications consume the same checked telescope as
//! promotion, while inferred kind binders stay invisible to written @ arguments.

use super::*;
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

impl TypeElaboratorChirho<'_> {
    pub(super) fn elaborate_constructor_type_application_chirho(
        &mut self,
        expression_chirho: &mut ExprChirho,
    ) -> Result<bool, ElaborationErrorChirho> {
        let span_chirho = expression_chirho.span_chirho();
        let mut base_chirho = expression_chirho;
        let mut arguments_chirho = Vec::new();
        loop {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            match base_chirho {
                ExprChirho::TypeAppChirho {
                    expr_chirho,
                    ty_chirho,
                    ..
                } => {
                    arguments_chirho.push(ty_chirho);
                    base_chirho = expr_chirho;
                }
                ExprChirho::ParenChirho { inner_chirho, .. } => base_chirho = inner_chirho,
                _ => break,
            }
        }
        let ExprChirho::ConChirho(name_chirho) = base_chirho else {
            return Ok(false);
        };
        let Some(key_chirho) = self.constructor_key_chirho(name_chirho)? else {
            return Ok(false);
        };
        let Some(scheme_chirho) = self
            .registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(&key_chirho)
            .filter(|scheme_chirho| scheme_chirho.promoted_chirho && !scheme_chirho.held_chirho)
            .cloned()
        else {
            return Ok(false);
        };
        let (binders_chirho, _, _) = self.instantiate_header_chirho(&scheme_chirho, span_chirho)?;
        let mut cursor_chirho = 0;
        for argument_chirho in arguments_chirho.into_iter().rev() {
            while binders_chirho
                .get(cursor_chirho)
                .is_some_and(|binder_chirho| {
                    binder_chirho.visibility_chirho == TelescopeVisibilityChirho::InferredChirho
                })
            {
                cursor_chirho += 1;
            }
            let Some(binder_chirho) = binders_chirho.get(cursor_chirho).filter(|binder_chirho| {
                binder_chirho.visibility_chirho == TelescopeVisibilityChirho::SpecifiedChirho
            }) else {
                return Err(Self::error_chirho(
                    "no specified constructor type binder remains for written type application",
                    argument_chirho.span_chirho(),
                ));
            };
            let typed_chirho = self.infer_type_chirho(argument_chirho)?;
            self.state_chirho.unify_chirho(
                &typed_chirho.kind_chirho,
                &binder_chirho.kind_chirho,
                argument_chirho.span_chirho(),
            )?;
            self.state_chirho.unify_chirho(
                &TyChirho::VarChirho(binder_chirho.variable_chirho),
                &typed_chirho.term_chirho,
                argument_chirho.span_chirho(),
            )?;
            // This is the already checked kind, not another inference pass in
            // the term converter. It shares this module's owner and supply.
            let argument_span_chirho = argument_chirho.span_chirho();
            let kind_chirho =
                self.surface_chirho(&typed_chirho.kind_chirho, argument_span_chirho)?;
            let original_chirho = std::mem::replace(
                argument_chirho,
                TypeChirho::WildcardChirho {
                    span_chirho: argument_span_chirho,
                },
            );
            *argument_chirho = TypeChirho::KindAnnotationChirho {
                ty_chirho: Box::new(original_chirho),
                kind_chirho: Box::new(kind_chirho),
                span_chirho: argument_span_chirho,
            };
            cursor_chirho += 1;
        }
        Ok(true)
    }
}
