// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `type role T r1 .. rn` annotations (RoleAnnotations, row 694).
//!
//! The CST keeps a role annotation as a type signature node. Lowering reads it here and writes
//! each role onto the matching visible parameter of the module's own data or newtype
//! declaration, where GND's method coercion proof reads it. An annotation that names a role
//! GHC does not accept, or a different number of roles than the type has parameters, is one
//! GHC refuses; every parameter of that type then reads as nominal, the conservative role.
//! workflow: language-features-chirho/deriving-strategies-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_syntax_chirho::token_chirho::TokenKindChirho;

use super::LowerCtxChirho;

/// The role words of each annotation, by the annotated type's name.
pub(super) type RoleAnnotationsChirho = HashMap<String, Vec<NameChirho>>;

/// The words a role annotation may write; `_` asks for the inferred role.
const ROLE_WORDS_CHIRHO: &[&str] = &["nominal", "representational", "phantom", "_"];

/// The most restrictive role.
const NOMINAL_WORD_CHIRHO: &str = "nominal";

impl LowerCtxChirho {
    /// The annotated type and its role words when `node_chirho` is `type role T r1 .. rn`.
    /// workflow: language-features-chirho/deriving-strategies-chirho
    pub(super) fn lower_role_annotation_chirho(
        &self,
        node_chirho: &GreenNodeChirho,
        base_chirho: usize,
    ) -> Option<(String, Vec<NameChirho>)> {
        let children_chirho = self.semantic_children_chirho(node_chirho, base_chirho);
        let mut tokens_chirho =
            children_chirho
                .iter()
                .filter_map(|child_chirho| match child_chirho.element_chirho {
                    GreenElementChirho::TokenChirho(token_chirho) => {
                        Some((child_chirho, token_chirho))
                    }
                    GreenElementChirho::NodeChirho(_) => None,
                });
        let (_, keyword_chirho) = tokens_chirho.next()?;
        if keyword_chirho.kind_chirho() != TokenKindChirho::TypeKeywordChirho {
            return None;
        }
        let (_, role_chirho) = tokens_chirho.next()?;
        if role_chirho.text_chirho() != "role" {
            return None;
        }
        let (_, target_chirho) = tokens_chirho.next()?;
        if target_chirho.kind_chirho() != TokenKindChirho::ConIdChirho {
            return None;
        }
        let roles_chirho = tokens_chirho
            .map(|(child_chirho, token_chirho)| {
                let span_chirho =
                    self.span_chirho(child_chirho.start_chirho, child_chirho.end_chirho);
                self.name_from_token_chirho(token_chirho, span_chirho)
            })
            .collect();
        Some((target_chirho.text_chirho().to_string(), roles_chirho))
    }
}

/// Write each annotation's roles onto its data or newtype declaration's visible parameters,
/// in order. An unreadable annotation marks every visible parameter nominal.
/// workflow: language-features-chirho/deriving-strategies-chirho
pub(super) fn attach_role_annotations_chirho(
    mut decls_chirho: Vec<DeclChirho>,
    annotations_chirho: &RoleAnnotationsChirho,
) -> Vec<DeclChirho> {
    for decl_chirho in &mut decls_chirho {
        let (name_chirho, type_vars_chirho) = match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                type_vars_chirho,
                ..
            }
            | DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                ..
            } => (name_chirho, type_vars_chirho),
            _ => continue,
        };
        let Some(roles_chirho) = annotations_chirho.get(name_chirho.text_chirho()) else {
            continue;
        };
        let visible_count_chirho = type_vars_chirho
            .iter()
            .filter(|binder_chirho| binder_chirho.is_visible_chirho())
            .count();
        let readable_chirho = visible_count_chirho == roles_chirho.len()
            && roles_chirho
                .iter()
                .all(|role_chirho| ROLE_WORDS_CHIRHO.contains(&role_chirho.text_chirho()));
        let mut written_chirho = roles_chirho.iter();
        for binder_chirho in type_vars_chirho
            .iter_mut()
            .filter(|binder_chirho| binder_chirho.is_visible_chirho())
        {
            let role_chirho = match written_chirho.next() {
                Some(role_chirho) if readable_chirho => role_chirho.clone(),
                _ => NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                    NOMINAL_WORD_CHIRHO,
                    name_chirho.span_chirho(),
                )),
            };
            binder_chirho.role_annotation_chirho = Some(role_chirho);
        }
    }
    decls_chirho
}
