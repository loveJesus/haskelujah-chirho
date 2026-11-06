// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pattern results retain written synonym arguments. Unfold a checked supplier
//! only to expose an argument the actual pattern declaration consumes.

use super::*;
use crate::ty_chirho::declarations_chirho::{DeclarationSchemesChirho, TelescopeVisibilityChirho};
use haskelujah_ast_chirho::decl_chirho::TyVarChirho as AstTyVarChirho;
use haskelujah_ast_chirho::name_chirho::{
    CompilerTypeConstructorChirho, NameChirho, TypeConstructorIdentityChirho,
    TypeConstructorRoleChirho,
};
use haskelujah_ast_chirho::ty_chirho::{
    ConstraintChirho, MultiplicityChirho, inference_variables_chirho::ScopedTypeVariableIdChirho,
};
use std::collections::HashSet;

#[cfg(test)]
#[path = "pattern_signature_tests_chirho.rs"]
mod tests_chirho;

type EnvironmentChirho<'syntax_chirho> = Option<Arc<FrameChirho<'syntax_chirho>>>;
type LookupChirho<'syntax_chirho> = Option<(
    Arc<FrameChirho<'syntax_chirho>>,
    Option<ViewChirho<'syntax_chirho>>,
)>;

#[derive(Clone)]
struct ViewChirho<'syntax_chirho> {
    node_chirho: NodeChirho<'syntax_chirho>,
    environment_chirho: EnvironmentChirho<'syntax_chirho>,
}

#[derive(Clone)]
enum NodeChirho<'syntax_chirho> {
    SourceChirho(&'syntax_chirho TypeChirho),
    ApplicationChirho(
        Arc<ViewChirho<'syntax_chirho>>,
        Arc<ViewChirho<'syntax_chirho>>,
        bool,
    ),
}

struct FrameChirho<'syntax_chirho> {
    parent_chirho: EnvironmentChirho<'syntax_chirho>,
    depth_chirho: usize,
    // None masks a lexical binder; an image always retains its CALLER frame.
    bindings_chirho: Vec<(
        ScopedTypeVariableIdChirho,
        Option<ViewChirho<'syntax_chirho>>,
    )>,
}

struct VariableChirho<'syntax_chirho> {
    source_chirho: ScopedTypeVariableIdChirho,
    // The frame is an actual lexical instantiation, not a source-name identity.
    // Retain it while a key exists, even when substitution returns to a caller.
    frame_chirho: EnvironmentChirho<'syntax_chirho>,
}

impl PartialEq for VariableChirho<'_> {
    fn eq(&self, other_chirho: &Self) -> bool {
        self.source_chirho == other_chirho.source_chirho
            && match (&self.frame_chirho, &other_chirho.frame_chirho) {
                (Some(left_chirho), Some(right_chirho)) => Arc::ptr_eq(left_chirho, right_chirho),
                (None, None) => true,
                _ => false,
            }
    }
}

impl Eq for VariableChirho<'_> {}

impl std::hash::Hash for VariableChirho<'_> {
    fn hash<HChirho: std::hash::Hasher>(&self, state_chirho: &mut HChirho) {
        std::hash::Hash::hash(&self.source_chirho, state_chirho);
        std::hash::Hash::hash(&self.frame_chirho.as_ref().map(Arc::as_ptr), state_chirho);
    }
}

enum PendingChirho<'syntax_chirho> {
    TypeChirho(ViewChirho<'syntax_chirho>),
    ConstraintChirho(
        &'syntax_chirho ConstraintChirho,
        EnvironmentChirho<'syntax_chirho>,
    ),
}

struct CheckerChirho<'syntax_chirho, 'state_chirho> {
    catalogue_chirho: &'syntax_chirho DeclarationSchemesChirho,
    state_chirho: &'state_chirho mut ElaborationStateChirho,
    span_chirho: SpanChirho,
}

impl<'syntax_chirho> ViewChirho<'syntax_chirho> {
    fn source_chirho(
        syntax_chirho: &'syntax_chirho TypeChirho,
        environment_chirho: EnvironmentChirho<'syntax_chirho>,
    ) -> Self {
        Self {
            node_chirho: NodeChirho::SourceChirho(syntax_chirho),
            environment_chirho,
        }
    }
    fn child_chirho(&self, syntax_chirho: &'syntax_chirho TypeChirho) -> Self {
        Self::source_chirho(syntax_chirho, self.environment_chirho.clone())
    }
}

impl TypeElaboratorChirho<'_> {
    pub(super) fn check_pattern_signature_result_chirho(
        &mut self,
        syntax_chirho: &TypeChirho,
        arity_chirho: usize,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        // Workflow: compiler-pipeline-chirho/declaration-validity-chirho.
        self.state_chirho
            .check_retained_syntax_chirho(syntax_chirho, 128)?;
        CheckerChirho {
            catalogue_chirho: self.registry_chirho.declaration_schemes_chirho(),
            state_chirho: &mut self.state_chirho,
            span_chirho,
        }
        .check_chirho(syntax_chirho, arity_chirho)
    }
}

impl<'syntax_chirho> CheckerChirho<'syntax_chirho, '_> {
    fn charge_chirho(&mut self, output_chirho: usize) -> Result<(), ElaborationErrorChirho> {
        self.state_chirho
            .charge_chirho(1, output_chirho, self.span_chirho)
    }
    fn error_chirho(&self, message_chirho: &str) -> ElaborationErrorChirho {
        TypeElaboratorChirho::error_chirho(message_chirho, self.span_chirho)
    }
    fn frame_depth_chirho(
        &mut self,
        parent_chirho: &EnvironmentChirho<'syntax_chirho>,
    ) -> Result<usize, ElaborationErrorChirho> {
        self.charge_chirho(0)?;
        let depth_chirho = parent_chirho
            .as_ref()
            .map_or(1, |frame_chirho| frame_chirho.depth_chirho + 1);
        if depth_chirho > 128 {
            return Err(self.state_chirho.resource_failure_chirho(
                "pattern signature lexical expansion depth bound exhausted",
                self.span_chirho,
            ));
        }
        Ok(depth_chirho)
    }
    fn lookup_chirho(
        &mut self,
        source_chirho: &ScopedTypeVariableIdChirho,
        mut environment_chirho: EnvironmentChirho<'syntax_chirho>,
    ) -> Result<LookupChirho<'syntax_chirho>, ElaborationErrorChirho> {
        while let Some(frame_chirho) = environment_chirho {
            self.charge_chirho(0)?;
            for (key_chirho, binding_chirho) in &frame_chirho.bindings_chirho {
                self.charge_chirho(0)?;
                if key_chirho == source_chirho {
                    return Ok(Some((frame_chirho.clone(), binding_chirho.clone())));
                }
            }
            environment_chirho = frame_chirho.parent_chirho.clone();
        }
        Ok(None)
    }
    fn resolve_chirho(
        &mut self,
        mut view_chirho: ViewChirho<'syntax_chirho>,
    ) -> Result<ViewChirho<'syntax_chirho>, ElaborationErrorChirho> {
        loop {
            self.charge_chirho(0)?;
            let NodeChirho::SourceChirho(TypeChirho::VarChirho(name_chirho)) =
                view_chirho.node_chirho
            else {
                return Ok(view_chirho);
            };
            let Some(source_chirho) = name_chirho.type_variable_chirho() else {
                return Err(self.error_chirho("pattern result variable lost its checked owner"));
            };
            let image_chirho = self
                .lookup_chirho(source_chirho, view_chirho.environment_chirho.clone())?
                .and_then(|(_, image_chirho)| image_chirho);
            let Some(image_chirho) = image_chirho else {
                return Ok(view_chirho);
            };
            view_chirho = image_chirho;
        }
    }
    fn variable_chirho(
        &mut self,
        name_chirho: &NameChirho,
        environment_chirho: &EnvironmentChirho<'syntax_chirho>,
    ) -> Result<VariableChirho<'syntax_chirho>, ElaborationErrorChirho> {
        let source_chirho = name_chirho
            .type_variable_chirho()
            .ok_or_else(|| self.error_chirho("pattern result variable lost its checked owner"))?;
        if let Some((frame_chirho, binding_chirho)) =
            self.lookup_chirho(source_chirho, environment_chirho.clone())?
        {
            if binding_chirho.is_some() {
                return Err(self.error_chirho("pattern variable substitution was not exposed"));
            }
            return Ok(VariableChirho {
                source_chirho: source_chirho.clone(),
                frame_chirho: Some(frame_chirho),
            });
        }
        match source_chirho {
            ScopedTypeVariableIdChirho::InferenceChirho { owner_chirho, .. }
                if Some(*owner_chirho) == self.state_chirho.owner_chirho =>
            {
                Ok(VariableChirho {
                    source_chirho: source_chirho.clone(),
                    frame_chirho: None,
                })
            }
            _ => Err(self.error_chirho("pattern result contains an unbound canonical variable")),
        }
    }
    fn scan_name_chirho(
        &mut self,
        name_chirho: &NameChirho,
        environment_chirho: &EnvironmentChirho<'syntax_chirho>,
        existentials_chirho: &HashSet<VariableChirho<'syntax_chirho>>,
        pending_chirho: &mut Vec<PendingChirho<'syntax_chirho>>,
    ) -> Result<(), ElaborationErrorChirho> {
        let Some(source_chirho) = name_chirho.type_variable_chirho() else {
            return Ok(());
        };
        if let Some((_, Some(image_chirho))) =
            self.lookup_chirho(source_chirho, environment_chirho.clone())?
        {
            self.charge_chirho(1)?;
            pending_chirho.push(PendingChirho::TypeChirho(image_chirho));
        } else if existentials_chirho
            .contains(&self.variable_chirho(name_chirho, environment_chirho)?)
        {
            return Err(self.error_chirho("pattern synonym existential occurs in its result type"));
        }
        Ok(())
    }
    fn mask_chirho(
        &mut self,
        binders_chirho: &[AstTyVarChirho],
        parent_chirho: EnvironmentChirho<'syntax_chirho>,
    ) -> Result<EnvironmentChirho<'syntax_chirho>, ElaborationErrorChirho> {
        let depth_chirho = self.frame_depth_chirho(&parent_chirho)?;
        self.state_chirho.charge_chirho(
            binders_chirho.len(),
            binders_chirho.len() + 1,
            self.span_chirho,
        )?;
        let mut bindings_chirho = Vec::with_capacity(binders_chirho.len());
        for binder_chirho in binders_chirho {
            bindings_chirho.push((
                binder_chirho
                    .name_chirho
                    .type_variable_chirho()
                    .ok_or_else(|| self.error_chirho("pattern quantifier lost its checked owner"))?
                    .clone(),
                None,
            ));
        }
        Ok(Some(Arc::new(FrameChirho {
            parent_chirho,
            depth_chirho,
            bindings_chirho,
        })))
    }
    fn unwrapped_chirho(
        &mut self,
        mut view_chirho: ViewChirho<'syntax_chirho>,
    ) -> Result<ViewChirho<'syntax_chirho>, ElaborationErrorChirho> {
        loop {
            view_chirho = self.resolve_chirho(view_chirho)?;
            match view_chirho.node_chirho {
                NodeChirho::SourceChirho(TypeChirho::ParenChirho { inner_chirho, .. }) => {
                    view_chirho = view_chirho.child_chirho(inner_chirho)
                }
                NodeChirho::SourceChirho(TypeChirho::KindAnnotationChirho {
                    ty_chirho, ..
                }) => view_chirho = view_chirho.child_chirho(ty_chirho),
                _ => return Ok(view_chirho),
            }
        }
    }
    fn application_chirho(
        &mut self,
        view_chirho: &ViewChirho<'syntax_chirho>,
    ) -> Result<
        Option<(ViewChirho<'syntax_chirho>, ViewChirho<'syntax_chirho>, bool)>,
        ElaborationErrorChirho,
    > {
        self.charge_chirho(0)?;
        Ok(match &view_chirho.node_chirho {
            NodeChirho::ApplicationChirho(function_chirho, argument_chirho, kind_chirho) => Some((
                (**function_chirho).clone(),
                (**argument_chirho).clone(),
                *kind_chirho,
            )),
            NodeChirho::SourceChirho(TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            }) => Some((
                view_chirho.child_chirho(fun_chirho),
                view_chirho.child_chirho(arg_chirho),
                false,
            )),
            NodeChirho::SourceChirho(TypeChirho::KindApplicationChirho {
                fun_chirho,
                arg_chirho,
                ..
            }) => Some((
                view_chirho.child_chirho(fun_chirho),
                view_chirho.child_chirho(arg_chirho),
                true,
            )),
            _ => None,
        })
    }
    fn head_chirho(
        &mut self,
        mut view_chirho: ViewChirho<'syntax_chirho>,
    ) -> Result<
        (
            ViewChirho<'syntax_chirho>,
            Vec<(bool, ViewChirho<'syntax_chirho>)>,
        ),
        ElaborationErrorChirho,
    > {
        let mut arguments_chirho = Vec::new();
        loop {
            view_chirho = self.unwrapped_chirho(view_chirho)?;
            let Some((function_chirho, argument_chirho, kind_chirho)) =
                self.application_chirho(&view_chirho)?
            else {
                break;
            };
            self.charge_chirho(1)?;
            arguments_chirho.push((kind_chirho, argument_chirho));
            view_chirho = function_chirho;
        }
        arguments_chirho.reverse();
        Ok((view_chirho, arguments_chirho))
    }
    fn unfold_chirho(
        &mut self,
        view_chirho: ViewChirho<'syntax_chirho>,
    ) -> Result<Option<ViewChirho<'syntax_chirho>>, ElaborationErrorChirho> {
        let (head_chirho, mut arguments_chirho) = self.head_chirho(view_chirho)?;
        let NodeChirho::SourceChirho(TypeChirho::ConChirho(name_chirho)) = head_chirho.node_chirho
        else {
            return Ok(None);
        };
        let Some(declaration_chirho) = name_chirho.type_constructor_chirho() else {
            return Ok(None);
        };
        if name_chirho.type_constructor_opaque_chirho()
            || declaration_chirho.role_chirho != TypeConstructorRoleChirho::TypeSynonymChirho
        {
            return Ok(None);
        }
        let key_chirho = declaration_chirho.type_key_chirho();
        let Some(scheme_chirho) = self.catalogue_chirho.get_chirho(&key_chirho) else {
            return Ok(None);
        };
        let Some(syntax_chirho) = scheme_chirho.synonym_syntax_chirho.as_deref() else {
            return Ok(None);
        };
        if scheme_chirho.held_chirho
            || scheme_chirho.owner_chirho.is_some()
            || scheme_chirho.declaration_chirho.as_ref() != Some(declaration_chirho)
            || arguments_chirho.len() < scheme_chirho.binders_chirho.len()
        {
            return Ok(None);
        }
        let depth_chirho = self.frame_depth_chirho(&head_chirho.environment_chirho)?;
        self.state_chirho.check_retained_syntax_chirho(
            syntax_chirho.rhs_chirho(),
            128usize.saturating_sub(arguments_chirho.len() - scheme_chirho.binders_chirho.len()),
        )?;
        self.state_chirho.charge_chirho(
            scheme_chirho.binders_chirho.len(),
            scheme_chirho.binders_chirho.len() + 1,
            self.span_chirho,
        )?;
        let mut bindings_chirho = Vec::with_capacity(scheme_chirho.binders_chirho.len());
        for (binder_chirho, (kind_chirho, argument_chirho)) in
            scheme_chirho.binders_chirho.iter().zip(&arguments_chirho)
        {
            if *kind_chirho
                == (binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho)
            {
                return Err(self.error_chirho(
                    "pattern synonym application lost its complete checked telescope",
                ));
            }
            bindings_chirho.push((
                ScopedTypeVariableIdChirho::QuantifiedChirho {
                    declaration_chirho: scheme_chirho.identity_chirho.clone(),
                    index_chirho: binder_chirho.variable_chirho.0,
                },
                Some(argument_chirho.clone()),
            ));
        }
        let environment_chirho = Some(Arc::new(FrameChirho {
            parent_chirho: head_chirho.environment_chirho,
            depth_chirho,
            bindings_chirho,
        }));
        let mut result_chirho =
            ViewChirho::source_chirho(syntax_chirho.rhs_chirho(), environment_chirho);
        for (kind_chirho, argument_chirho) in
            arguments_chirho.drain(scheme_chirho.binders_chirho.len()..)
        {
            self.charge_chirho(2)?;
            result_chirho = ViewChirho {
                node_chirho: NodeChirho::ApplicationChirho(
                    Arc::new(result_chirho),
                    Arc::new(argument_chirho),
                    kind_chirho,
                ),
                environment_chirho: None,
            };
        }
        Ok(Some(result_chirho))
    }
    fn exposed_chirho(
        &mut self,
        mut view_chirho: ViewChirho<'syntax_chirho>,
    ) -> Result<ViewChirho<'syntax_chirho>, ElaborationErrorChirho> {
        loop {
            view_chirho = self.unwrapped_chirho(view_chirho)?;
            if let Some(unfolded_chirho) = self.unfold_chirho(view_chirho.clone())? {
                view_chirho = unfolded_chirho;
            } else {
                return Ok(view_chirho);
            }
        }
    }
    fn check_chirho(
        &mut self,
        syntax_chirho: &'syntax_chirho TypeChirho,
        arity_chirho: usize,
    ) -> Result<(), ElaborationErrorChirho> {
        let mut result_chirho = ViewChirho::source_chirho(syntax_chirho, None);
        let mut required_chirho = false;
        let mut existentials_chirho = HashSet::new();
        loop {
            let exposed_chirho = self.exposed_chirho(result_chirho.clone())?;
            match exposed_chirho.node_chirho {
                NodeChirho::SourceChirho(
                    TypeChirho::ForallChirho {
                        vars_chirho,
                        body_chirho,
                        ..
                    }
                    | TypeChirho::RequiredForallChirho {
                        vars_chirho,
                        body_chirho,
                        ..
                    },
                ) => {
                    let environment_chirho =
                        self.mask_chirho(vars_chirho, exposed_chirho.environment_chirho)?;
                    if required_chirho {
                        for binder_chirho in vars_chirho {
                            self.charge_chirho(1)?;
                            existentials_chirho.insert(self.variable_chirho(
                                &binder_chirho.name_chirho,
                                &environment_chirho,
                            )?);
                        }
                    }
                    result_chirho = ViewChirho::source_chirho(body_chirho, environment_chirho);
                }
                NodeChirho::SourceChirho(TypeChirho::QualChirho { body_chirho, .. }) => {
                    required_chirho = true;
                    result_chirho = exposed_chirho.child_chirho(body_chirho);
                }
                _ => break,
            }
        }
        if existentials_chirho.is_empty() {
            return Ok(());
        }
        for _ in 0..arity_chirho {
            let exposed_chirho = self.exposed_chirho(result_chirho)?;
            if let NodeChirho::SourceChirho(TypeChirho::FunChirho {
                result_chirho: body_chirho,
                ..
            }) = exposed_chirho.node_chirho
            {
                result_chirho = exposed_chirho.child_chirho(body_chirho);
                continue;
            }
            let (head_chirho, arguments_chirho) = self.head_chirho(exposed_chirho)?;
            let builtin_arrow_chirho = match &head_chirho.node_chirho {
                NodeChirho::SourceChirho(TypeChirho::ConChirho(name_chirho))
                    if !name_chirho.type_constructor_opaque_chirho() =>
                {
                    name_chirho
                        .type_constructor_chirho()
                        .is_some_and(|declaration_chirho| {
                            declaration_chirho.identity_chirho
                                == TypeConstructorIdentityChirho::CompilerChirho(
                                    CompilerTypeConstructorChirho::FunctionChirho,
                                )
                        })
                }
                _ => false,
            };
            if !builtin_arrow_chirho
                || arguments_chirho.len() != 2
                || arguments_chirho.iter().any(|(kind_chirho, _)| *kind_chirho)
            {
                return Err(self.error_chirho(
                    "pattern signature has no checked arrow for a declared argument",
                ));
            }
            result_chirho = arguments_chirho.into_iter().nth(1).unwrap().1;
        }
        self.scan_chirho(result_chirho, &existentials_chirho)
    }
    fn scan_chirho(
        &mut self,
        result_chirho: ViewChirho<'syntax_chirho>,
        existentials_chirho: &HashSet<VariableChirho<'syntax_chirho>>,
    ) -> Result<(), ElaborationErrorChirho> {
        self.charge_chirho(1)?;
        let mut pending_chirho = vec![PendingChirho::TypeChirho(result_chirho)];
        while let Some(item_chirho) = pending_chirho.pop() {
            self.charge_chirho(0)?;
            match item_chirho {
                PendingChirho::TypeChirho(view_chirho) => {
                    let view_chirho = self.resolve_chirho(view_chirho)?;
                    if let Some((function_chirho, argument_chirho, _)) =
                        self.application_chirho(&view_chirho)?
                    {
                        self.charge_chirho(2)?;
                        pending_chirho.extend([
                            PendingChirho::TypeChirho(function_chirho),
                            PendingChirho::TypeChirho(argument_chirho),
                        ]);
                        continue;
                    }
                    let NodeChirho::SourceChirho(syntax_chirho) = view_chirho.node_chirho else {
                        unreachable!();
                    };
                    match syntax_chirho {
                        TypeChirho::VarChirho(name_chirho) => {
                            if existentials_chirho.contains(
                                &self.variable_chirho(
                                    name_chirho,
                                    &view_chirho.environment_chirho,
                                )?,
                            ) {
                                return Err(self.error_chirho(
                                    "pattern synonym existential occurs in its result type",
                                ));
                            }
                        }
                        TypeChirho::ForallChirho {
                            vars_chirho,
                            body_chirho,
                            ..
                        }
                        | TypeChirho::RequiredForallChirho {
                            vars_chirho,
                            body_chirho,
                            ..
                        } => {
                            let environment_chirho =
                                self.mask_chirho(vars_chirho, view_chirho.environment_chirho)?;
                            self.charge_chirho(vars_chirho.len() + 1)?;
                            for binder_chirho in vars_chirho {
                                if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                                    pending_chirho.push(PendingChirho::TypeChirho(
                                        ViewChirho::source_chirho(
                                            kind_chirho,
                                            environment_chirho.clone(),
                                        ),
                                    ));
                                }
                            }
                            pending_chirho.push(PendingChirho::TypeChirho(
                                ViewChirho::source_chirho(body_chirho, environment_chirho),
                            ));
                        }
                        TypeChirho::QualChirho {
                            context_chirho,
                            body_chirho,
                            ..
                        } => {
                            self.charge_chirho(context_chirho.len() + 1)?;
                            pending_chirho.extend(context_chirho.iter().map(|constraint_chirho| {
                                PendingChirho::ConstraintChirho(
                                    constraint_chirho,
                                    view_chirho.environment_chirho.clone(),
                                )
                            }));
                            pending_chirho.push(PendingChirho::TypeChirho(
                                view_chirho.child_chirho(body_chirho),
                            ));
                        }
                        TypeChirho::FunChirho {
                            arg_chirho,
                            result_chirho,
                            mult_chirho,
                            ..
                        } => {
                            self.charge_chirho(2)?;
                            pending_chirho.extend([
                                PendingChirho::TypeChirho(view_chirho.child_chirho(arg_chirho)),
                                PendingChirho::TypeChirho(view_chirho.child_chirho(result_chirho)),
                            ]);
                            if let Some(MultiplicityChirho::MultVarChirho(name_chirho)) =
                                mult_chirho
                            {
                                self.scan_name_chirho(
                                    name_chirho,
                                    &view_chirho.environment_chirho,
                                    existentials_chirho,
                                    &mut pending_chirho,
                                )?;
                            }
                        }
                        TypeChirho::KindAnnotationChirho {
                            ty_chirho,
                            kind_chirho,
                            ..
                        } => {
                            self.charge_chirho(2)?;
                            pending_chirho.extend([
                                PendingChirho::TypeChirho(view_chirho.child_chirho(ty_chirho)),
                                PendingChirho::TypeChirho(view_chirho.child_chirho(kind_chirho)),
                            ]);
                        }
                        TypeChirho::ParenChirho { inner_chirho, .. }
                        | TypeChirho::ListChirho {
                            element_chirho: inner_chirho,
                            ..
                        } => {
                            self.charge_chirho(1)?;
                            pending_chirho.push(PendingChirho::TypeChirho(
                                view_chirho.child_chirho(inner_chirho),
                            ));
                        }
                        TypeChirho::TupleChirho {
                            elements_chirho, ..
                        }
                        | TypeChirho::PromotedListChirho {
                            elements_chirho, ..
                        } => {
                            self.charge_chirho(elements_chirho.len())?;
                            pending_chirho.extend(elements_chirho.iter().map(|element_chirho| {
                                PendingChirho::TypeChirho(view_chirho.child_chirho(element_chirho))
                            }));
                        }
                        TypeChirho::ConChirho(_)
                        | TypeChirho::PromotedConChirho { .. }
                        | TypeChirho::WildcardChirho { .. }
                        | TypeChirho::LitChirho { .. } => {}
                        TypeChirho::AppChirho { .. } | TypeChirho::KindApplicationChirho { .. } => {
                            unreachable!()
                        }
                    }
                }
                PendingChirho::ConstraintChirho(constraint_chirho, environment_chirho) => {
                    match constraint_chirho {
                        ConstraintChirho::KindAnnotationChirho {
                            constraint_chirho,
                            kind_chirho,
                            ..
                        } => {
                            self.charge_chirho(2)?;
                            pending_chirho.extend([
                                PendingChirho::ConstraintChirho(
                                    constraint_chirho,
                                    environment_chirho.clone(),
                                ),
                                PendingChirho::TypeChirho(ViewChirho::source_chirho(
                                    kind_chirho,
                                    environment_chirho,
                                )),
                            ]);
                        }
                        ConstraintChirho::ClassChirho {
                            class_chirho,
                            args_chirho,
                            kind_args_chirho,
                            ..
                        } => {
                            self.scan_name_chirho(
                                class_chirho,
                                &environment_chirho,
                                existentials_chirho,
                                &mut pending_chirho,
                            )?;
                            self.charge_chirho(args_chirho.len() + kind_args_chirho.len())?;
                            pending_chirho.extend(args_chirho.iter().chain(kind_args_chirho).map(
                                |argument_chirho| {
                                    PendingChirho::TypeChirho(ViewChirho::source_chirho(
                                        argument_chirho,
                                        environment_chirho.clone(),
                                    ))
                                },
                            ));
                        }
                        ConstraintChirho::QuantifiedChirho {
                            vars_chirho,
                            context_chirho,
                            body_chirho,
                            ..
                        } => {
                            let environment_chirho =
                                self.mask_chirho(vars_chirho, environment_chirho)?;
                            self.charge_chirho(vars_chirho.len() + context_chirho.len() + 1)?;
                            for binder_chirho in vars_chirho {
                                if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                                    pending_chirho.push(PendingChirho::TypeChirho(
                                        ViewChirho::source_chirho(
                                            kind_chirho,
                                            environment_chirho.clone(),
                                        ),
                                    ));
                                }
                            }
                            pending_chirho.extend(
                                context_chirho
                                    .iter()
                                    .chain(std::iter::once(body_chirho.as_ref()))
                                    .map(|constraint_chirho| {
                                        PendingChirho::ConstraintChirho(
                                            constraint_chirho,
                                            environment_chirho.clone(),
                                        )
                                    }),
                            );
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
