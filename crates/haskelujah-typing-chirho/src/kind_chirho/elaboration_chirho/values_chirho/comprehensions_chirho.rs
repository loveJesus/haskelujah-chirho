// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Using expressions see the outer lexical type scope; keys see the actual
//! preceding pattern binders. Every walk spends the original state allowance.
use super::*;
use haskelujah_ast_chirho::expr_chirho::ComprehensionQualifierChirho;

impl TypeElaboratorChirho<'_> {
    pub(super) fn comprehension_using_chirho(
        &mut self,
        qualifiers_chirho: &mut [ComprehensionQualifierChirho],
    ) -> Result<(), ElaborationErrorChirho> {
        for qualifier_chirho in qualifiers_chirho {
            if let ComprehensionQualifierChirho::TransformChirho(transform_chirho) =
                qualifier_chirho
            {
                self.state_chirho
                    .charge_chirho(1, 0, transform_chirho.span_chirho)?;
                if !self.transform_list_comp_chirho {
                    return Err(Self::error_chirho(
                        "transform qualifiers require TransformListComp",
                        transform_chirho.span_chirho,
                    ));
                }
                let expression_chirho =
                    transform_chirho.using_chirho.as_mut().ok_or_else(|| {
                        Self::error_chirho(
                            "transform qualifier has no using expression",
                            transform_chirho.span_chirho,
                        )
                    })?;
                self.scoped_chirho(|this_chirho| this_chirho.expression_chirho(expression_chirho))?;
            }
        }
        Ok(())
    }

    pub(super) fn comprehension_qualifiers_chirho(
        &mut self,
        qualifiers_chirho: &mut [ComprehensionQualifierChirho],
    ) -> Result<(), ElaborationErrorChirho> {
        for qualifier_chirho in qualifiers_chirho {
            self.state_chirho
                .charge_chirho(1, 0, qualifier_chirho.span_chirho())?;
            match qualifier_chirho {
                ComprehensionQualifierChirho::StatementChirho(statement_chirho) => {
                    self.statements_chirho(std::slice::from_mut(statement_chirho.as_mut()))?
                }
                ComprehensionQualifierChirho::TransformChirho(transform_chirho) => {
                    if let Some(by_chirho) = &mut transform_chirho.by_chirho {
                        self.scoped_chirho(|this_chirho| this_chirho.expression_chirho(by_chirho))?;
                    }
                }
            }
        }
        Ok(())
    }
}
