// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Checked associated defaults are specialized by producer-owned slot edges.
//! No default reduces until an actual class instance supplies that telescope.

use super::*;
use crate::ty_chirho::declarations_chirho::{
    DeclarationAssociationChirho, TelescopeVisibilityChirho,
};
use haskelujah_ast_chirho::decl_chirho::AssocTfInstanceChirho;

impl InferCtxChirho {
    fn associated_default_invariant_chirho(
        &mut self,
        message_chirho: &str,
        span_chirho: SpanChirho,
    ) {
        let diagnostic_chirho = DiagnosticChirho::error_with_code_chirho(
            ErrorCodeChirho::error_chirho(300),
            message_chirho,
            span_chirho,
        );
        self.diagnostics_chirho
            .push_chirho(diagnostic_chirho.clone());
        self.non_deferrable_diagnostics_chirho
            .push_chirho(diagnostic_chirho);
    }

    pub(super) fn instantiate_checked_associated_defaults_chirho(
        &mut self,
        class_chirho: &NameChirho,
        instance_chirho: &InstDeclChirho,
        explicit_chirho: &[AssocTfInstanceChirho],
        span_chirho: SpanChirho,
    ) -> bool {
        let Some(class_header_chirho) = class_chirho
            .type_constructor_chirho()
            .and_then(|declaration_chirho| {
                self.type_families_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(&declaration_chirho.type_key_chirho())
            })
            .filter(|header_chirho| {
                header_chirho.owner_chirho.is_none() && !header_chirho.held_chirho
            })
            .cloned()
        else {
            return false;
        };
        let DeclarationAssociationChirho::ClassChirho { families_chirho } =
            &class_header_chirho.association_chirho
        else {
            return false;
        };
        let mut hidden_chirho = instance_chirho.kind_tys_chirho.iter();
        let mut visible_chirho = std::iter::once(&instance_chirho.head_ty_chirho)
            .chain(&instance_chirho.extra_head_tys_chirho);
        let mut work_chirho = 100_000_usize;
        if class_header_chirho.binders_chirho.len() > work_chirho {
            self.associated_default_invariant_chirho(
                "associated default class telescope resource bound exhausted",
                span_chirho,
            );
            return true;
        }
        work_chirho -= class_header_chirho.binders_chirho.len();
        let mut arguments_chirho = HashMap::new();
        for binder_chirho in &class_header_chirho.binders_chirho {
            let argument_chirho =
                if binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho {
                    visible_chirho.next()
                } else {
                    hidden_chirho.next()
                };
            let Some(argument_chirho) = argument_chirho else {
                self.associated_default_invariant_chirho(
                    "class instance lost an argument of its checked telescope",
                    span_chirho,
                );
                return true;
            };
            arguments_chirho.insert(binder_chirho.variable_chirho, argument_chirho);
        }
        if hidden_chirho.next().is_some()
            || (class_header_chirho.visible_arity_chirho > 0 && visible_chirho.next().is_some())
        {
            self.associated_default_invariant_chirho(
                "class instance has excess checked telescope arguments",
                span_chirho,
            );
            return true;
        }
        let explicit_chirho = explicit_chirho
            .iter()
            .filter_map(|equation_chirho| {
                self.family_reference_key_chirho(&equation_chirho.family_name_chirho)
            })
            .collect::<HashSet<_>>();
        for (ordinal_chirho, family_key_chirho) in families_chirho.iter().enumerate() {
            if explicit_chirho.contains(family_key_chirho.as_ref()) {
                continue;
            }
            let Some(family_chirho) = self
                .type_families_chirho
                .declaration_schemes_chirho()
                .get_chirho(family_key_chirho)
                .cloned()
            else {
                self.associated_default_invariant_chirho(
                    "class lost its associated family declaration",
                    span_chirho,
                );
                return true;
            };
            let DeclarationAssociationChirho::FamilyChirho {
                class_chirho: owner_chirho,
                correspondence_chirho,
                default_chirho,
            } = &family_chirho.association_chirho
            else {
                self.associated_default_invariant_chirho(
                    "associated family has no checked class owner",
                    span_chirho,
                );
                return true;
            };
            if owner_chirho != &class_header_chirho.identity_chirho
                || family_chirho.owner_chirho.is_some()
                || family_chirho.held_chirho
            {
                self.associated_default_invariant_chirho(
                    "associated family owner or checked telescope is invalid",
                    span_chirho,
                );
                return true;
            }
            let Some(default_chirho) = default_chirho else {
                continue;
            };
            if self.has_non_deferrable_failure_chirho() {
                return true;
            }
            if family_chirho.binders_chirho.len() > work_chirho {
                self.associated_default_invariant_chirho(
                    "associated default family telescope resource bound exhausted",
                    span_chirho,
                );
                return true;
            }
            work_chirho -= family_chirho.binders_chirho.len();
            let mut specialization_chirho = HashMap::new();
            let correspondence_chirho = correspondence_chirho
                .iter()
                .copied()
                .collect::<HashMap<_, _>>();
            for binder_chirho in &family_chirho.binders_chirho {
                let argument_chirho = if let Some(class_slot_chirho) =
                    correspondence_chirho.get(&binder_chirho.variable_chirho)
                {
                    let Some(argument_chirho) = arguments_chirho.get(class_slot_chirho) else {
                        self.associated_default_invariant_chirho(
                            "associated family refers to an unbound class slot",
                            span_chirho,
                        );
                        return true;
                    };
                    std::borrow::Cow::Borrowed(*argument_chirho)
                } else {
                    std::borrow::Cow::Owned(self.fresh_var_chirho())
                };
                specialization_chirho.insert(binder_chirho.variable_chirho, argument_chirho);
            }
            let Some((lhs_chirho, rhs_chirho, _)) = self.specialize_checked_family_terms_chirho(
                &default_chirho.lhs_chirho,
                &default_chirho.rhs_chirho,
                &specialization_chirho,
                &mut work_chirho,
            ) else {
                return true;
            };
            // This internal ownership edge is independent of import spelling.
            self.register_family_instance_at_chirho(
                family_key_chirho.to_string(),
                lhs_chirho,
                rhs_chirho,
                span_chirho,
                u32::try_from(ordinal_chirho).ok(),
            );
        }
        true
    }
}
