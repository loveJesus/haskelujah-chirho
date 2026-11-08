// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Retain quotation headers as raw syntax without expanding constructors,
//! copying output, or manufacturing checked kind ownership.
use super::*;

pub fn check_borrowed_declaration_syntax_chirho(
    declaration_chirho: &DeclChirho,
    budget_chirho: &mut GadtProjectionBudgetChirho,
) -> Result<(), GadtProjectionErrorChirho> {
    checked_chirho(
        declaration_chirho.span_chirho(),
        budget_chirho,
        |budget_chirho| {
            let mut syntax_chirho = |node_chirho| walk_chirho(node_chirho, 1, 0, 1, budget_chirho);
            let (name_chirho, variables_chirho, kind_chirho, deriving_chirho, constructors_chirho) =
                match declaration_chirho {
                    DeclChirho::DataDeclChirho {
                        name_chirho,
                        type_vars_chirho,
                        constructors_chirho,
                        deriving_chirho,
                        kind_sig_chirho,
                        ..
                    } => (
                        name_chirho,
                        type_vars_chirho,
                        kind_sig_chirho,
                        deriving_chirho,
                        constructors_chirho.as_slice(),
                    ),
                    DeclChirho::NewtypeDeclChirho {
                        name_chirho,
                        type_vars_chirho,
                        constructor_chirho,
                        deriving_chirho,
                        kind_sig_chirho,
                        ..
                    } => (
                        name_chirho,
                        type_vars_chirho,
                        kind_sig_chirho,
                        deriving_chirho,
                        std::slice::from_ref(constructor_chirho.as_ref()),
                    ),
                    _ => {
                        return syntax_chirho(ProjectionNodeChirho::DeclarationChirho(
                            declaration_chirho,
                        ));
                    }
                };
            syntax_chirho(ProjectionNodeChirho::NameChirho(name_chirho))?;
            syntax_chirho(ProjectionNodeChirho::BindersChirho(variables_chirho))?;
            syntax_chirho(ProjectionNodeChirho::DerivationsChirho(deriving_chirho))?;
            if let Some(kind_chirho) = kind_chirho {
                for written_chirho in [kind_chirho.standalone_chirho(), kind_chirho.result_chirho()]
                    .into_iter()
                    .flatten()
                {
                    syntax_chirho(ProjectionNodeChirho::TypeChirho(written_chirho))?;
                }
            }
            for constructor_chirho in constructors_chirho {
                match constructor_chirho {
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho,
                        prefix_chirho,
                        fields_chirho,
                        ..
                    } => {
                        syntax_chirho(ProjectionNodeChirho::NameChirho(name_chirho))?;
                        syntax_chirho(ProjectionNodeChirho::BindersChirho(
                            &prefix_chirho.existential_vars_chirho,
                        ))?;
                        syntax_chirho(ProjectionNodeChirho::ConstraintsChirho(
                            &prefix_chirho.context_chirho,
                        ))?;
                        for (_, ty_chirho) in fields_chirho {
                            syntax_chirho(ProjectionNodeChirho::TypeChirho(ty_chirho))?;
                        }
                    }
                    ConDeclChirho::RecordChirho {
                        name_chirho,
                        prefix_chirho,
                        fields_chirho,
                        ..
                    } => {
                        syntax_chirho(ProjectionNodeChirho::NameChirho(name_chirho))?;
                        syntax_chirho(ProjectionNodeChirho::BindersChirho(
                            &prefix_chirho.existential_vars_chirho,
                        ))?;
                        syntax_chirho(ProjectionNodeChirho::ConstraintsChirho(
                            &prefix_chirho.context_chirho,
                        ))?;
                        for field_chirho in fields_chirho {
                            field_syntax_chirho(field_chirho, &mut syntax_chirho)?;
                        }
                    }
                    ConDeclChirho::GadtChirho {
                        name_chirho,
                        signature_chirho,
                        ..
                    } => {
                        if signature_chirho.malformed_chirho
                            == Some(super::super::GadtConMalformedChirho::ResourceChirho)
                        {
                            return Err(GadtProjectionLimitChirho::ProducerResourceChirho);
                        }
                        syntax_chirho(ProjectionNodeChirho::NameChirho(name_chirho))?;
                        for prefix_chirho in &signature_chirho.prefix_chirho {
                            match prefix_chirho {
                                GadtConPrefixChirho::ForallChirho { vars_chirho, .. } => {
                                    syntax_chirho(ProjectionNodeChirho::BindersChirho(vars_chirho))?
                                }
                                GadtConPrefixChirho::ContextChirho {
                                    constraints_chirho, ..
                                } => syntax_chirho(ProjectionNodeChirho::ConstraintsChirho(
                                    constraints_chirho,
                                ))?,
                            }
                        }
                        match &signature_chirho.args_chirho {
                            GadtConArgsChirho::PositionalChirho(fields_chirho) => {
                                for field_chirho in fields_chirho {
                                    syntax_chirho(ProjectionNodeChirho::TypeChirho(
                                        &field_chirho.ty_chirho,
                                    ))?;
                                    if let Some(multiplicity_chirho) =
                                        &field_chirho.multiplicity_chirho
                                    {
                                        syntax_chirho(ProjectionNodeChirho::MultiplicityChirho(
                                            multiplicity_chirho,
                                        ))?;
                                    }
                                }
                            }
                            GadtConArgsChirho::RecordChirho(fields_chirho) => {
                                for field_chirho in fields_chirho {
                                    field_syntax_chirho(field_chirho, &mut syntax_chirho)?;
                                }
                            }
                        }
                        syntax_chirho(ProjectionNodeChirho::TypeChirho(
                            &signature_chirho.result_ty_chirho,
                        ))?;
                    }
                }
            }
            Ok(())
        },
    )
}

fn field_syntax_chirho<'syntax_chirho>(
    field_chirho: &'syntax_chirho FieldDeclChirho,
    syntax_chirho: &mut impl FnMut(
        ProjectionNodeChirho<'syntax_chirho>,
    ) -> Result<(), GadtProjectionLimitChirho>,
) -> Result<(), GadtProjectionLimitChirho> {
    syntax_chirho(ProjectionNodeChirho::NamesChirho(
        &field_chirho.names_chirho,
    ))?;
    syntax_chirho(ProjectionNodeChirho::TypeChirho(&field_chirho.ty_chirho))?;
    if let Some(multiplicity_chirho) = &field_chirho.multiplicity_chirho {
        syntax_chirho(ProjectionNodeChirho::MultiplicityChirho(
            multiplicity_chirho,
        ))?;
    }
    Ok(())
}
