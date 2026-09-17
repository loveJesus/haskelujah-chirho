// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Checked source-local class promises; absence of a body is not an empty concrete body.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::*;
use haskelujah_ast_chirho::class_chirho::MinimalFormulaChirho;

// Class variables occupy fixed slots in every member contract; otherwise
// independently alpha-normalising a method could exchange the class parameters.
fn close_class_scheme_chirho(
    mut scheme_chirho: SchemeChirho,
    parameters_chirho: &[TyVarChirho],
) -> SchemeChirho {
    let mut variables_chirho = parameters_chirho.to_vec();
    for variable_chirho in &scheme_chirho.vars_chirho {
        if !variables_chirho.contains(variable_chirho) {
            variables_chirho.push(*variable_chirho);
        }
    }
    scheme_chirho.vars_chirho = variables_chirho;
    scheme_chirho
}
impl InferCtxChirho {
    pub(in crate::infer_chirho) fn capture_class_contract_chirho(
        &mut self,
        declaration_chirho: &DeclChirho,
        scope_chirho: &HashMap<String, TyVarChirho>,
        parameters_chirho: &[TyVarChirho],
        method_schemes_chirho: &HashMap<String, SchemeChirho>,
        fundeps_chirho: Option<Vec<(Vec<usize>, Vec<usize>)>>,
    ) -> ClassContractChirho {
        use haskelujah_ast_chirho::ty_chirho::TypeChirho;
        let DeclChirho::ClassDeclChirho {
            context_chirho,
            context_written_chirho,
            methods_chirho,
            associated_tfs_chirho,
            minimal_chirho,
            span_chirho,
            ..
        } = declaration_chirho
        else {
            unreachable!()
        };
        let superclass_source_chirho = TypeChirho::QualChirho {
            context_chirho: context_chirho.clone(),
            body_chirho: Box::new(TypeChirho::TupleChirho {
                elements_chirho: Vec::new(),
                span_chirho: *span_chirho,
            }),
            span_chirho: *span_chirho,
        };
        let superclasses_chirho = close_class_scheme_chirho(
            self.ast_type_to_scheme_seeded_chirho(&superclass_source_chirho, scope_chirho, false)
                .0,
            parameters_chirho,
        );
        let methods_chirho: Vec<_> = methods_chirho
            .iter()
            .map(|method_chirho| {
                let generic_chirho =
                    method_chirho
                        .default_sig_chirho
                        .as_ref()
                        .map(|signature_chirho| {
                            close_class_scheme_chirho(
                                self.ast_type_to_scheme_seeded_chirho(
                                    signature_chirho,
                                    scope_chirho,
                                    false,
                                )
                                .0,
                                parameters_chirho,
                            )
                        });
                MethodContractChirho {
                    name_chirho: method_chirho.name_chirho.text_chirho().to_owned(),
                    scheme_chirho: method_schemes_chirho[method_chirho.name_chirho.text_chirho()]
                        .clone(),
                    default_chirho: method_chirho.default_chirho.is_some(),
                    generic_chirho,
                }
            })
            .collect();
        let minimal_chirho = minimal_chirho.clone().unwrap_or_else(|| {
            MinimalFormulaChirho::AllChirho(
                methods_chirho
                    .iter()
                    .filter(|method_chirho| !method_chirho.default_chirho)
                    .map(|method_chirho| {
                        MinimalFormulaChirho::MethodChirho(method_chirho.name_chirho.clone())
                    })
                    .collect(),
            )
        });
        if let Err(reason_chirho) = minimal_chirho::validate_chirho(
            &minimal_chirho,
            &methods_chirho
                .iter()
                .map(|method_chirho| method_chirho.name_chirho.clone())
                .collect(),
        ) {
            self.diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(206),
                    reason_chirho,
                    *span_chirho,
                ));
        }
        let associated_chirho = associated_tfs_chirho
            .iter()
            .map(|family_chirho| {
                let defaults_chirho = family_chirho
                    .defaults_chirho
                    .iter()
                    .map(|equation_chirho| {
                        // Family equation binders belong to the family's input order,
                        // not to the independently named class head.
                        let source_chirho = TypeChirho::TupleChirho {
                            elements_chirho: equation_chirho
                                .lhs_types_chirho
                                .iter()
                                .cloned()
                                .chain(std::iter::once(equation_chirho.rhs_chirho.clone()))
                                .collect(),
                            span_chirho: equation_chirho.span_chirho,
                        };
                        self.ast_type_to_scheme_seeded_chirho(
                            &source_chirho,
                            &HashMap::new(),
                            false,
                        )
                        .0
                    })
                    .collect();
                AssociatedContractChirho {
                    name_chirho: family_chirho.name_chirho.text_chirho().to_owned(),
                    data_chirho: family_chirho.data_chirho,
                    kind_chirho: None,
                    defaults_chirho,
                    injectivity_chirho: family_chirho
                        .result_chirho
                        .injectivity_chirho
                        .as_ref()
                        .map(|dependency_chirho| {
                            let mut positions_chirho: Vec<_> = dependency_chirho
                                .parameters_chirho
                                .iter()
                                .map(|name_chirho| {
                                    family_chirho.type_vars_chirho.iter().position(
                                        |parameter_chirho| {
                                            parameter_chirho.text_chirho()
                                                == name_chirho.text_chirho()
                                        },
                                    )
                                })
                                .collect();
                            // The annotation denotes a set, not declaration
                            // order. Invalid/unrepresented slots remain None
                            // and cannot establish agreement below.
                            positions_chirho.sort_unstable();
                            positions_chirho.dedup();
                            positions_chirho
                        }),
                }
            })
            .collect();
        ClassContractChirho {
            abstract_chirho: !context_written_chirho
                && context_chirho.is_empty()
                && methods_chirho.is_empty()
                && associated_tfs_chirho.is_empty(),
            fundeps_chirho,
            superclasses_chirho,
            methods_chirho,
            associated_chirho,
            minimal_chirho,
        }
    }
}
#[derive(Debug, Clone)]
pub(in crate::infer_chirho) struct ClassContractChirho {
    pub(in crate::infer_chirho) abstract_chirho: bool,
    pub(in crate::infer_chirho) fundeps_chirho: Option<Vec<(Vec<usize>, Vec<usize>)>>,
    pub(in crate::infer_chirho) superclasses_chirho: SchemeChirho,
    pub(in crate::infer_chirho) methods_chirho: Vec<MethodContractChirho>,
    pub(in crate::infer_chirho) associated_chirho: Vec<AssociatedContractChirho>,
    pub(in crate::infer_chirho) minimal_chirho: MinimalFormulaChirho,
}
#[derive(Debug, Clone)]
pub(in crate::infer_chirho) struct MethodContractChirho {
    pub(in crate::infer_chirho) name_chirho: String,
    pub(in crate::infer_chirho) scheme_chirho: SchemeChirho,
    pub(in crate::infer_chirho) default_chirho: bool,
    pub(in crate::infer_chirho) generic_chirho: Option<SchemeChirho>,
}
#[derive(Debug, Clone)]
pub(in crate::infer_chirho) struct AssociatedContractChirho {
    pub(in crate::infer_chirho) name_chirho: String,
    pub(in crate::infer_chirho) data_chirho: bool,
    pub(in crate::infer_chirho) kind_chirho: Option<KindContractChirho>,
    pub(in crate::infer_chirho) defaults_chirho: Vec<SchemeChirho>,
    pub(in crate::infer_chirho) injectivity_chirho: Option<Vec<Option<usize>>>,
}
impl ClassContractChirho {
    pub(super) fn check_chirho(&self, actual_chirho: &Self) -> Result<(), String> {
        if self.fundeps_chirho.is_none() || self.fundeps_chirho != actual_chirho.fundeps_chirho {
            return Err("different functional dependencies".to_owned());
        }
        if self.abstract_chirho {
            return Ok(());
        }
        if !self
            .superclasses_chirho
            .alpha_equivalent_chirho(&actual_chirho.superclasses_chirho)
        {
            return Err("superclass constraints differ".to_owned());
        }
        if self.methods_chirho.len() != actual_chirho.methods_chirho.len() {
            return Err("number of class methods differs".to_owned());
        }
        for (expected_chirho, actual_chirho) in self
            .methods_chirho
            .iter()
            .zip(&actual_chirho.methods_chirho)
        {
            if expected_chirho.name_chirho != actual_chirho.name_chirho {
                return Err("class method names or order differ".to_owned());
            }
            if !expected_chirho
                .scheme_chirho
                .alpha_equivalent_chirho(&actual_chirho.scheme_chirho)
            {
                return Err(format!(
                    "class method type differs: {}",
                    expected_chirho.name_chirho
                ));
            }
            let generic_agrees_chirho = match (
                &expected_chirho.generic_chirho,
                &actual_chirho.generic_chirho,
            ) {
                (None, None) => true,
                (Some(expected_chirho), Some(actual_chirho)) => {
                    expected_chirho.alpha_equivalent_chirho(actual_chirho)
                }
                _ => false,
            };
            if expected_chirho.default_chirho != actual_chirho.default_chirho
                || !generic_agrees_chirho
            {
                return Err(format!(
                    "default method differs: {}",
                    expected_chirho.name_chirho
                ));
            }
        }
        if self.associated_chirho.len() != actual_chirho.associated_chirho.len() {
            return Err("number of associated types differs".to_owned());
        }
        for (expected_chirho, actual_chirho) in self
            .associated_chirho
            .iter()
            .zip(&actual_chirho.associated_chirho)
        {
            if [
                &expected_chirho.injectivity_chirho,
                &actual_chirho.injectivity_chirho,
            ]
            .into_iter()
            .flatten()
            .any(|positions_chirho| positions_chirho.iter().any(Option::is_none))
            {
                return Err("associated injectivity has unproved parameter slots".to_owned());
            }
            let (Some(expected_kind_chirho), Some(actual_kind_chirho)) =
                (&expected_chirho.kind_chirho, &actual_chirho.kind_chirho)
            else {
                return Err("associated family has no checked kind contract".to_owned());
            };
            if !expected_kind_chirho.alpha_equivalent_chirho(actual_kind_chirho)
                || expected_chirho.data_chirho != actual_chirho.data_chirho
            {
                return Err("associated types have different kinds or forms".to_owned());
            }
            if expected_chirho.injectivity_chirho != actual_chirho.injectivity_chirho
                || expected_chirho.defaults_chirho.len() != actual_chirho.defaults_chirho.len()
                || !expected_chirho
                    .defaults_chirho
                    .iter()
                    .zip(&actual_chirho.defaults_chirho)
                    .all(|(left_chirho, right_chirho)| {
                        left_chirho.alpha_equivalent_chirho(right_chirho)
                    })
            {
                return Err("associated type defaults or dependencies differ".to_owned());
            }
        }
        if !minimal_chirho::implies_chirho(&self.minimal_chirho, &actual_chirho.minimal_chirho)
            .map_err(|reason_chirho| format!("MINIMAL implication unproved: {reason_chirho}"))?
        {
            return Err("MINIMAL promises do not imply implementation requirements".to_owned());
        }
        Ok(())
    }
}
