// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Quoted declarations retain headers, and visit escapes in their actual
//! value bodies, without publishing signatures or declaration owners.
use super::*;

impl TypeElaboratorChirho<'_> {
    pub(super) fn quoted_declaration_chirho(
        &mut self,
        declaration_chirho: &mut DeclChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho.check_retained_declaration_syntax_chirho(
            declaration_chirho,
            128usize.saturating_sub(self.type_depth_chirho),
        )?;
        match declaration_chirho {
            DeclChirho::FunBindChirho { matches_chirho, .. } => {
                self.matches_with_signature_chirho(matches_chirho, None)?
            }
            DeclChirho::PatBindChirho {
                pat_chirho,
                rhs_chirho,
                ..
            } => {
                self.pattern_chirho(pat_chirho)?;
                self.rhs_chirho(rhs_chirho)?;
            }
            DeclChirho::ClassDeclChirho { methods_chirho, .. } => {
                for method_chirho in methods_chirho {
                    if let Some(matches_chirho) = &mut method_chirho.default_chirho {
                        self.scoped_chirho(|this_chirho| {
                            this_chirho.matches_with_signature_chirho(matches_chirho, None)
                        })?;
                    }
                }
            }
            DeclChirho::InstanceDeclChirho { methods_chirho, .. } => {
                self.local_binds_chirho(methods_chirho)?
            }
            DeclChirho::PatSynDeclChirho {
                pat_chirho,
                dir_chirho,
                ..
            } => {
                self.pattern_chirho(pat_chirho)?;
                if let PatSynDirChirho::ExplBidirChirho {
                    builder_binds_chirho,
                } = dir_chirho
                {
                    self.local_binds_chirho(builder_binds_chirho)?;
                }
            }
            DeclChirho::SpliceDeclChirho { expr_chirho, .. } => self
                .at_quote_stage_chirho(self.quote_stage_chirho - 1, |this_chirho| {
                    this_chirho.expression_chirho(expr_chirho)
                })?,
            _ => {}
        }
        Ok(())
    }
}
