// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Immutable checked synonym syntax lives on its declaration record. Equality
//! compares canonical proof metadata and binder semantics, without source spans,
//! local resolution IDs, occurrence IDs, or display spellings of checked names.

use super::declarations_chirho::DeclarationLocalBinderChirho;
use haskelujah_ast_chirho::decl_chirho::TyVarChirho as AstTyVarChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, MultiplicityChirho, TypeChirho};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PublishedSynonymSyntaxChirho {
    parameters_chirho: Vec<AstTyVarChirho>,
    rhs_chirho: TypeChirho,
    local_binders_chirho: Arc<[DeclarationLocalBinderChirho]>,
}

impl PublishedSynonymSyntaxChirho {
    /// The checked kind publisher owns construction; callers can only borrow
    /// the syntax and its producer-owned binder-kind records.
    pub(crate) fn new_chirho(
        parameters_chirho: Vec<AstTyVarChirho>,
        rhs_chirho: TypeChirho,
        local_binders_chirho: Arc<[DeclarationLocalBinderChirho]>,
    ) -> Self {
        Self {
            parameters_chirho,
            rhs_chirho,
            local_binders_chirho,
        }
    }
    pub fn parameters_chirho(&self) -> &[AstTyVarChirho] {
        &self.parameters_chirho
    }
    pub fn rhs_chirho(&self) -> &TypeChirho {
        &self.rhs_chirho
    }
    pub fn local_binders_chirho(&self) -> &[DeclarationLocalBinderChirho] {
        &self.local_binders_chirho
    }
    pub(crate) fn binder_table_chirho(&self) -> Arc<[DeclarationLocalBinderChirho]> {
        self.local_binders_chirho.clone()
    }
}

impl PartialEq for PublishedSynonymSyntaxChirho {
    fn eq(&self, other_chirho: &Self) -> bool {
        std::ptr::eq(self, other_chirho)
            || (self.local_binders_chirho == other_chirho.local_binders_chirho
                && equal_binders_chirho(&self.parameters_chirho, &other_chirho.parameters_chirho)
                && equal_type_chirho(&self.rhs_chirho, &other_chirho.rhs_chirho))
    }
}
impl Eq for PublishedSynonymSyntaxChirho {}

fn equal_name_chirho(left_chirho: &NameChirho, right_chirho: &NameChirho) -> bool {
    let metadata_equal_chirho = left_chirho.type_variable_chirho()
        == right_chirho.type_variable_chirho()
        && left_chirho.type_constructor_chirho() == right_chirho.type_constructor_chirho()
        && left_chirho.promotion_chirho() == right_chirho.promotion_chirho()
        && left_chirho.family_key_chirho() == right_chirho.family_key_chirho();
    if !metadata_equal_chirho {
        return false;
    }
    let canonical_chirho = left_chirho.type_variable_chirho().is_some()
        || left_chirho.type_constructor_chirho().is_some()
        || left_chirho.promotion_chirho().is_some()
        || left_chirho.family_key_chirho().is_some();
    canonical_chirho
        || (left_chirho.text_chirho() == right_chirho.text_chirho()
            && left_chirho.qualifier_chirho() == right_chirho.qualifier_chirho())
}
fn equal_binders_chirho(left_chirho: &[AstTyVarChirho], right_chirho: &[AstTyVarChirho]) -> bool {
    left_chirho.len() == right_chirho.len()
        && left_chirho
            .iter()
            .zip(right_chirho)
            .all(|(left_chirho, right_chirho)| {
                equal_name_chirho(&left_chirho.name_chirho, &right_chirho.name_chirho)
                    && left_chirho.visibility_chirho == right_chirho.visibility_chirho
                    && left_chirho.specificity_chirho == right_chirho.specificity_chirho
                    && match (
                        &left_chirho.kind_annotation_chirho,
                        &right_chirho.kind_annotation_chirho,
                    ) {
                        (None, None) => true,
                        (Some(left_chirho), Some(right_chirho)) => {
                            equal_type_chirho(left_chirho, right_chirho)
                        }
                        _ => false,
                    }
            })
}
fn equal_multiplicity_chirho(
    left_chirho: &Option<MultiplicityChirho>,
    right_chirho: &Option<MultiplicityChirho>,
) -> bool {
    match (left_chirho, right_chirho) {
        (None, None)
        | (Some(MultiplicityChirho::OneChirho), Some(MultiplicityChirho::OneChirho))
        | (Some(MultiplicityChirho::ManyChirho), Some(MultiplicityChirho::ManyChirho)) => true,
        (
            Some(MultiplicityChirho::MultVarChirho(left_chirho)),
            Some(MultiplicityChirho::MultVarChirho(right_chirho)),
        ) => equal_name_chirho(left_chirho, right_chirho),
        _ => false,
    }
}
fn equal_types_chirho(left_chirho: &[TypeChirho], right_chirho: &[TypeChirho]) -> bool {
    left_chirho.len() == right_chirho.len()
        && left_chirho
            .iter()
            .zip(right_chirho)
            .all(|(left_chirho, right_chirho)| equal_type_chirho(left_chirho, right_chirho))
}
fn equal_constraints_chirho(
    left_chirho: &[ConstraintChirho],
    right_chirho: &[ConstraintChirho],
) -> bool {
    left_chirho.len() == right_chirho.len()
        && left_chirho
            .iter()
            .zip(right_chirho)
            .all(|(left_chirho, right_chirho)| equal_constraint_chirho(left_chirho, right_chirho))
}
fn equal_constraint_chirho(
    left_chirho: &ConstraintChirho,
    right_chirho: &ConstraintChirho,
) -> bool {
    match (left_chirho, right_chirho) {
        (
            ConstraintChirho::ClassChirho {
                class_chirho: left_class_chirho,
                args_chirho: left_args_chirho,
                kind_args_chirho: left_kinds_chirho,
                ..
            },
            ConstraintChirho::ClassChirho {
                class_chirho: right_class_chirho,
                args_chirho: right_args_chirho,
                kind_args_chirho: right_kinds_chirho,
                ..
            },
        ) => {
            equal_name_chirho(left_class_chirho, right_class_chirho)
                && equal_types_chirho(left_args_chirho, right_args_chirho)
                && equal_types_chirho(left_kinds_chirho, right_kinds_chirho)
        }
        (
            ConstraintChirho::KindAnnotationChirho {
                constraint_chirho: left_chirho,
                kind_chirho: left_kind_chirho,
                ..
            },
            ConstraintChirho::KindAnnotationChirho {
                constraint_chirho: right_chirho,
                kind_chirho: right_kind_chirho,
                ..
            },
        ) => {
            equal_constraint_chirho(left_chirho, right_chirho)
                && equal_type_chirho(left_kind_chirho, right_kind_chirho)
        }
        (
            ConstraintChirho::QuantifiedChirho {
                vars_chirho: left_vars_chirho,
                context_chirho: left_context_chirho,
                body_chirho: left_body_chirho,
                ..
            },
            ConstraintChirho::QuantifiedChirho {
                vars_chirho: right_vars_chirho,
                context_chirho: right_context_chirho,
                body_chirho: right_body_chirho,
                ..
            },
        ) => {
            equal_binders_chirho(left_vars_chirho, right_vars_chirho)
                && equal_constraints_chirho(left_context_chirho, right_context_chirho)
                && equal_constraint_chirho(left_body_chirho, right_body_chirho)
        }
        _ => false,
    }
}
fn equal_type_chirho(left_chirho: &TypeChirho, right_chirho: &TypeChirho) -> bool {
    match (left_chirho, right_chirho) {
        (TypeChirho::VarChirho(left_chirho), TypeChirho::VarChirho(right_chirho))
        | (TypeChirho::ConChirho(left_chirho), TypeChirho::ConChirho(right_chirho)) => {
            equal_name_chirho(left_chirho, right_chirho)
        }
        (
            TypeChirho::AppChirho {
                fun_chirho: left_fun_chirho,
                arg_chirho: left_arg_chirho,
                ..
            },
            TypeChirho::AppChirho {
                fun_chirho: right_fun_chirho,
                arg_chirho: right_arg_chirho,
                ..
            },
        ) => {
            equal_type_chirho(left_fun_chirho, right_fun_chirho)
                && equal_type_chirho(left_arg_chirho, right_arg_chirho)
        }
        (
            TypeChirho::KindApplicationChirho {
                fun_chirho: left_fun_chirho,
                arg_chirho: left_arg_chirho,
                written_chirho: left_written_chirho,
                ..
            },
            TypeChirho::KindApplicationChirho {
                fun_chirho: right_fun_chirho,
                arg_chirho: right_arg_chirho,
                written_chirho: right_written_chirho,
                ..
            },
        ) => {
            left_written_chirho == right_written_chirho
                && equal_type_chirho(left_fun_chirho, right_fun_chirho)
                && equal_type_chirho(left_arg_chirho, right_arg_chirho)
        }
        (
            TypeChirho::KindAnnotationChirho {
                ty_chirho: left_chirho,
                kind_chirho: left_kind_chirho,
                ..
            },
            TypeChirho::KindAnnotationChirho {
                ty_chirho: right_chirho,
                kind_chirho: right_kind_chirho,
                ..
            },
        ) => {
            equal_type_chirho(left_chirho, right_chirho)
                && equal_type_chirho(left_kind_chirho, right_kind_chirho)
        }
        (
            TypeChirho::FunChirho {
                arg_chirho: left_arg_chirho,
                mult_chirho: left_mult_chirho,
                result_chirho: left_result_chirho,
                ..
            },
            TypeChirho::FunChirho {
                arg_chirho: right_arg_chirho,
                mult_chirho: right_mult_chirho,
                result_chirho: right_result_chirho,
                ..
            },
        ) => {
            equal_multiplicity_chirho(left_mult_chirho, right_mult_chirho)
                && equal_type_chirho(left_arg_chirho, right_arg_chirho)
                && equal_type_chirho(left_result_chirho, right_result_chirho)
        }
        (
            TypeChirho::TupleChirho {
                elements_chirho: left_chirho,
                ..
            },
            TypeChirho::TupleChirho {
                elements_chirho: right_chirho,
                ..
            },
        )
        | (
            TypeChirho::PromotedListChirho {
                elements_chirho: left_chirho,
                ..
            },
            TypeChirho::PromotedListChirho {
                elements_chirho: right_chirho,
                ..
            },
        ) => equal_types_chirho(left_chirho, right_chirho),
        (
            TypeChirho::ListChirho {
                element_chirho: left_chirho,
                ..
            },
            TypeChirho::ListChirho {
                element_chirho: right_chirho,
                ..
            },
        )
        | (
            TypeChirho::ParenChirho {
                inner_chirho: left_chirho,
                ..
            },
            TypeChirho::ParenChirho {
                inner_chirho: right_chirho,
                ..
            },
        ) => equal_type_chirho(left_chirho, right_chirho),
        (
            TypeChirho::QualChirho {
                context_chirho: left_context_chirho,
                body_chirho: left_body_chirho,
                ..
            },
            TypeChirho::QualChirho {
                context_chirho: right_context_chirho,
                body_chirho: right_body_chirho,
                ..
            },
        ) => {
            equal_constraints_chirho(left_context_chirho, right_context_chirho)
                && equal_type_chirho(left_body_chirho, right_body_chirho)
        }
        (
            TypeChirho::ForallChirho {
                vars_chirho: left_vars_chirho,
                body_chirho: left_body_chirho,
                ..
            },
            TypeChirho::ForallChirho {
                vars_chirho: right_vars_chirho,
                body_chirho: right_body_chirho,
                ..
            },
        )
        | (
            TypeChirho::RequiredForallChirho {
                vars_chirho: left_vars_chirho,
                body_chirho: left_body_chirho,
                ..
            },
            TypeChirho::RequiredForallChirho {
                vars_chirho: right_vars_chirho,
                body_chirho: right_body_chirho,
                ..
            },
        ) => {
            equal_binders_chirho(left_vars_chirho, right_vars_chirho)
                && equal_type_chirho(left_body_chirho, right_body_chirho)
        }
        (
            TypeChirho::PromotedConChirho {
                name_chirho: left_chirho,
                ..
            },
            TypeChirho::PromotedConChirho {
                name_chirho: right_chirho,
                ..
            },
        ) => equal_name_chirho(left_chirho, right_chirho),
        (TypeChirho::WildcardChirho { .. }, TypeChirho::WildcardChirho { .. }) => true,
        (
            TypeChirho::LitChirho {
                value_chirho: left_chirho,
                ..
            },
            TypeChirho::LitChirho {
                value_chirho: right_chirho,
                ..
            },
        ) => left_chirho == right_chirho,
        _ => false,
    }
}

struct DeclarationScopeValidationChirho<'schema_chirho> {
    identity_chirho: &'schema_chirho str,
    records_chirho:
        std::collections::HashMap<super::TyVarChirho, &'schema_chirho DeclarationLocalBinderChirho>,
    scope_chirho: Vec<super::TyVarChirho>,
    used_chirho: std::collections::HashSet<super::TyVarChirho>,
    work_chirho: usize,
}
impl DeclarationScopeValidationChirho<'_> {
    fn charge_chirho(&mut self, amount_chirho: usize) -> Result<(), String> {
        self.work_chirho = self
            .work_chirho
            .checked_sub(amount_chirho)
            .ok_or_else(|| "declaration binder validation resource bound exhausted".to_owned())?;
        Ok(())
    }
    fn variable_chirho(&mut self, variable_chirho: super::TyVarChirho) -> Result<(), String> {
        self.charge_chirho(self.scope_chirho.len() + 1)?;
        if self.scope_chirho.contains(&variable_chirho) {
            Ok(())
        } else {
            Err(
                "published declaration contains an unquantified variable outside its lexical scope"
                    .to_owned(),
            )
        }
    }
    fn name_chirho(
        &mut self,
        name_chirho: &NameChirho,
        required_chirho: bool,
    ) -> Result<Option<super::TyVarChirho>, String> {
        use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;
        match name_chirho.type_variable_chirho() {
            Some(ScopedTypeVariableIdChirho::QuantifiedChirho {
                declaration_chirho,
                index_chirho,
            }) if declaration_chirho.as_ref() == self.identity_chirho => {
                if name_chirho.type_constructor_chirho().is_some()
                    || name_chirho.promotion_chirho().is_some()
                    || name_chirho.family_key_chirho().is_some()
                {
                    return Err(
                        "published variable carries inconsistent constructor metadata".to_owned(),
                    );
                }
                Ok(Some(super::TyVarChirho(*index_chirho)))
            }
            None if !required_chirho => Ok(None),
            _ => Err("published syntax retains a foreign or unbound variable owner".to_owned()),
        }
    }
    fn introduce_chirho(&mut self, variables_chirho: &[super::TyVarChirho]) -> Result<(), String> {
        self.charge_chirho(variables_chirho.len())?;
        for variable_chirho in variables_chirho {
            self.charge_chirho(self.scope_chirho.len() + 1)?;
            if self.scope_chirho.contains(variable_chirho)
                || !self.records_chirho.contains_key(variable_chirho)
            {
                return Err("published forall has no distinct checked local binder".to_owned());
            }
            self.scope_chirho.push(*variable_chirho);
            self.used_chirho.insert(*variable_chirho);
        }
        Ok(())
    }
    fn typed_chirho(
        &mut self,
        term_chirho: &super::TyChirho,
        depth_chirho: usize,
    ) -> Result<(), String> {
        use super::TyChirho;
        self.charge_chirho(1)?;
        if depth_chirho > 128 {
            return Err("declaration binder validation depth bound exhausted".to_owned());
        }
        match term_chirho {
            TyChirho::VarChirho(variable_chirho) => self.variable_chirho(*variable_chirho)?,
            TyChirho::ConChirho(name_chirho) => self.charge_chirho(name_chirho.len())?,
            TyChirho::ForallVarChirho(_) => {
                return Err(
                    "rigid solver marker cannot be a published declaration variable".to_owned(),
                );
            }
            TyChirho::AppChirho(left_chirho, right_chirho)
            | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
                self.typed_chirho(left_chirho, depth_chirho + 1)?;
                self.typed_chirho(right_chirho, depth_chirho + 1)?;
            }
            TyChirho::ListChirho(element_chirho) => {
                self.typed_chirho(element_chirho, depth_chirho + 1)?
            }
            TyChirho::TupleChirho(elements_chirho) => {
                self.charge_chirho(elements_chirho.len())?;
                for element_chirho in elements_chirho {
                    self.typed_chirho(element_chirho, depth_chirho + 1)?;
                }
            }
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                self.charge_chirho(context_chirho.len())?;
                for constraint_chirho in context_chirho {
                    self.typed_chirho(constraint_chirho, depth_chirho + 1)?;
                }
                self.typed_chirho(body_chirho, depth_chirho + 1)?;
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                let mark_chirho = self.scope_chirho.len();
                let result_chirho = (|| {
                    for variable_chirho in vars_chirho {
                        let record_chirho = *self
                            .records_chirho
                            .get(variable_chirho)
                            .ok_or_else(|| "published forall lost its kind record".to_owned())?;
                        let required_chirho = record_chirho.binder_chirho.visibility_chirho == super::declarations_chirho::TelescopeVisibilityChirho::RequiredChirho;
                        if required_chirho
                            != matches!(term_chirho, TyChirho::RequiredForallChirho { .. })
                        {
                            return Err(
                                "published typed quantifier and binder visibility disagree"
                                    .to_owned(),
                            );
                        }
                        self.typed_chirho(
                            &record_chirho.binder_chirho.kind_chirho,
                            depth_chirho + 1,
                        )?;
                        self.introduce_chirho(std::slice::from_ref(variable_chirho))?;
                    }
                    self.typed_chirho(body_chirho, depth_chirho + 1)
                })();
                self.scope_chirho.truncate(mark_chirho);
                result_chirho?;
            }
        }
        Ok(())
    }
    fn syntax_binders_chirho(
        &mut self,
        binders_chirho: &[AstTyVarChirho],
        required_chirho: bool,
        depth_chirho: usize,
    ) -> Result<(), String> {
        self.charge_chirho(binders_chirho.len())?;
        let mut variables_chirho = Vec::with_capacity(binders_chirho.len());
        for binder_chirho in binders_chirho {
            variables_chirho.push(
                self.name_chirho(&binder_chirho.name_chirho, true)?
                    .ok_or_else(|| "missing published binder identity".to_owned())?,
            );
        }
        for (binder_chirho, variable_chirho) in binders_chirho.iter().zip(&variables_chirho) {
            let record_chirho = *self
                .records_chirho
                .get(variable_chirho)
                .ok_or_else(|| "published syntax binder lost its kind record".to_owned())?;
            let visibility_chirho = if required_chirho {
                super::declarations_chirho::TelescopeVisibilityChirho::RequiredChirho
            } else if binder_chirho.specificity_chirho
                == haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho
            {
                super::declarations_chirho::TelescopeVisibilityChirho::InferredChirho
            } else {
                super::declarations_chirho::TelescopeVisibilityChirho::SpecifiedChirho
            };
            if record_chirho.specificity_chirho != binder_chirho.specificity_chirho
                || record_chirho.binder_chirho.visibility_chirho != visibility_chirho
            {
                return Err("published syntax and typed binder visibility disagree".to_owned());
            }
            self.typed_chirho(&record_chirho.binder_chirho.kind_chirho, depth_chirho + 1)?;
            if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                self.syntax_chirho(kind_chirho, depth_chirho + 1)?;
            }
            self.introduce_chirho(std::slice::from_ref(variable_chirho))?;
        }
        Ok(())
    }
    fn constraint_chirho(
        &mut self,
        constraint_chirho: &ConstraintChirho,
        depth_chirho: usize,
    ) -> Result<(), String> {
        self.charge_chirho(1)?;
        if depth_chirho > 128 {
            return Err("constraint syntax validation depth bound exhausted".to_owned());
        }
        match constraint_chirho {
            ConstraintChirho::ClassChirho {
                class_chirho,
                args_chirho,
                kind_args_chirho,
                ..
            } => {
                if let Some(variable_chirho) = self.name_chirho(class_chirho, false)? {
                    self.variable_chirho(variable_chirho)?;
                }
                self.charge_chirho(args_chirho.len() + kind_args_chirho.len())?;
                for argument_chirho in kind_args_chirho.iter().chain(args_chirho) {
                    self.syntax_chirho(argument_chirho, depth_chirho + 1)?;
                }
            }
            ConstraintChirho::KindAnnotationChirho {
                constraint_chirho,
                kind_chirho,
                ..
            } => {
                self.constraint_chirho(constraint_chirho, depth_chirho + 1)?;
                self.syntax_chirho(kind_chirho, depth_chirho + 1)?;
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => {
                let mark_chirho = self.scope_chirho.len();
                let result_chirho = (|| {
                    self.syntax_binders_chirho(vars_chirho, false, depth_chirho)?;
                    self.charge_chirho(context_chirho.len())?;
                    for constraint_chirho in context_chirho {
                        self.constraint_chirho(constraint_chirho, depth_chirho + 1)?;
                    }
                    self.constraint_chirho(body_chirho, depth_chirho + 1)
                })();
                self.scope_chirho.truncate(mark_chirho);
                result_chirho?;
            }
        }
        Ok(())
    }
    fn syntax_chirho(
        &mut self,
        term_chirho: &TypeChirho,
        depth_chirho: usize,
    ) -> Result<(), String> {
        self.charge_chirho(1)?;
        if depth_chirho > 128 {
            return Err("published syntax validation depth bound exhausted".to_owned());
        }
        match term_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                let variable_chirho = self
                    .name_chirho(name_chirho, true)?
                    .ok_or_else(|| "published variable has no binder".to_owned())?;
                self.variable_chirho(variable_chirho)?;
            }
            TypeChirho::ConChirho(name_chirho)
            | TypeChirho::PromotedConChirho { name_chirho, .. } => {
                if self.name_chirho(name_chirho, false)?.is_some() {
                    return Err("published constructor carries a variable owner".to_owned());
                }
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            }
            | TypeChirho::KindApplicationChirho {
                fun_chirho,
                arg_chirho,
                ..
            }
            | TypeChirho::KindAnnotationChirho {
                ty_chirho: fun_chirho,
                kind_chirho: arg_chirho,
                ..
            } => {
                self.syntax_chirho(fun_chirho, depth_chirho + 1)?;
                self.syntax_chirho(arg_chirho, depth_chirho + 1)?;
            }
            TypeChirho::FunChirho {
                arg_chirho,
                mult_chirho,
                result_chirho,
                ..
            } => {
                if let Some(MultiplicityChirho::MultVarChirho(name_chirho)) = mult_chirho {
                    let variable_chirho = self
                        .name_chirho(name_chirho, true)?
                        .ok_or_else(|| "published multiplicity has no binder".to_owned())?;
                    self.variable_chirho(variable_chirho)?;
                }
                self.syntax_chirho(arg_chirho, depth_chirho + 1)?;
                self.syntax_chirho(result_chirho, depth_chirho + 1)?;
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                self.charge_chirho(elements_chirho.len())?;
                for element_chirho in elements_chirho {
                    self.syntax_chirho(element_chirho, depth_chirho + 1)?;
                }
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                self.syntax_chirho(element_chirho, depth_chirho + 1)?
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.syntax_chirho(inner_chirho, depth_chirho + 1)?
            }
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                self.charge_chirho(context_chirho.len())?;
                for constraint_chirho in context_chirho {
                    self.constraint_chirho(constraint_chirho, depth_chirho + 1)?;
                }
                self.syntax_chirho(body_chirho, depth_chirho + 1)?;
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
                let mark_chirho = self.scope_chirho.len();
                let result_chirho = (|| {
                    self.syntax_binders_chirho(
                        vars_chirho,
                        matches!(term_chirho, TypeChirho::RequiredForallChirho { .. }),
                        depth_chirho,
                    )?;
                    self.syntax_chirho(body_chirho, depth_chirho + 1)
                })();
                self.scope_chirho.truncate(mark_chirho);
                result_chirho?;
            }
            TypeChirho::WildcardChirho { .. } => {
                return Err("unbound wildcard cannot be published syntax".to_owned());
            }
            TypeChirho::LitChirho { value_chirho, .. } => self.charge_chirho(value_chirho.len())?,
        }
        Ok(())
    }
}

pub(crate) fn validate_declaration_scopes_chirho(
    schema_chirho: &super::declarations_chirho::DeclarationSchemeChirho,
) -> Result<(), String> {
    let mut validator_chirho = DeclarationScopeValidationChirho {
        identity_chirho: &schema_chirho.identity_chirho,
        records_chirho: std::collections::HashMap::new(),
        scope_chirho: Vec::new(),
        used_chirho: std::collections::HashSet::new(),
        work_chirho: 1 << 25,
    };
    validator_chirho.charge_chirho(
        schema_chirho.local_binders_chirho.len() + schema_chirho.binders_chirho.len(),
    )?;
    let top_chirho = schema_chirho
        .binders_chirho
        .iter()
        .map(|binder_chirho| binder_chirho.variable_chirho)
        .collect::<std::collections::HashSet<_>>();
    for record_chirho in schema_chirho.local_binders_chirho.iter() {
        if top_chirho.contains(&record_chirho.binder_chirho.variable_chirho)
            || validator_chirho
                .records_chirho
                .insert(record_chirho.binder_chirho.variable_chirho, record_chirho)
                .is_some()
        {
            return Err("published local binder table repeats a telescope slot".to_owned());
        }
    }
    for binder_chirho in &schema_chirho.binders_chirho {
        validator_chirho.typed_chirho(&binder_chirho.kind_chirho, 0)?;
        validator_chirho
            .scope_chirho
            .push(binder_chirho.variable_chirho);
    }
    validator_chirho.typed_chirho(&schema_chirho.result_kind_chirho, 0)?;
    if let Some(rhs_chirho) = &schema_chirho.synonym_rhs_chirho {
        validator_chirho.typed_chirho(rhs_chirho, 0)?;
    }
    if let Some(syntax_chirho) = &schema_chirho.synonym_syntax_chirho {
        if schema_chirho.synonym_rhs_chirho.is_none()
            || schema_chirho.local_binders_chirho.as_ref()
                != syntax_chirho.local_binders_chirho.as_ref()
            || syntax_chirho.parameters_chirho.len() != schema_chirho.source_variables_chirho.len()
        {
            return Err(
                "published synonym syntax does not share its complete typed producer".to_owned(),
            );
        }
        for (index_chirho, parameter_chirho) in syntax_chirho.parameters_chirho.iter().enumerate() {
            validator_chirho.charge_chirho(
                schema_chirho.binders_chirho.len() + schema_chirho.source_variables_chirho.len(),
            )?;
            let mapping_chirho = schema_chirho
                .source_variables_chirho
                .iter()
                .find(|mapping_chirho| mapping_chirho.header_index_chirho as usize == index_chirho)
                .ok_or_else(|| {
                    "published syntax parameter has no written-position mapping".to_owned()
                })?;
            if validator_chirho.name_chirho(&parameter_chirho.name_chirho, true)?
                != Some(mapping_chirho.variable_chirho)
            {
                return Err("published syntax parameter lost its canonical slot".to_owned());
            }
            validator_chirho.scope_chirho = schema_chirho
                .binders_chirho
                .iter()
                .take_while(|binder_chirho| {
                    binder_chirho.variable_chirho != mapping_chirho.variable_chirho
                })
                .map(|binder_chirho| binder_chirho.variable_chirho)
                .collect();
            if let Some(kind_chirho) = &parameter_chirho.kind_annotation_chirho {
                validator_chirho.syntax_chirho(kind_chirho, 0)?;
            }
        }
        validator_chirho.scope_chirho = schema_chirho
            .binders_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.variable_chirho)
            .collect();
        validator_chirho.syntax_chirho(&syntax_chirho.rhs_chirho, 0)?;
    }
    if validator_chirho.used_chirho.len() != validator_chirho.records_chirho.len() {
        return Err("published local binder table has no lexical introduction".to_owned());
    }
    Ok(())
}
