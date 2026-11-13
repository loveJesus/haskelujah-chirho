// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

struct ProjectionChirho<'checked_chirho, 'budget_chirho> {
    checked_chirho: &'checked_chirho CheckedModuleChirho,
    left_chirho: &'budget_chirho mut usize,
}
impl ProjectionChirho<'_, '_> {
    fn charge_chirho(&mut self, count_chirho: usize) -> Result<(), TcErrorChirho> {
        *self.left_chirho = self.left_chirho.checked_sub(count_chirho).ok_or_else(|| {
            TcErrorChirho::new_chirho(
                TcErrorKindChirho::ResourceChirho,
                "checked interface output ceiling exhausted",
            )
        })?;
        Ok(())
    }
    fn original_chirho(
        &mut self,
        name_chirho: NameIdChirho,
    ) -> Result<OriginalNameChirho, TcErrorChirho> {
        let info_chirho = self
            .checked_chirho
            .module_chirho
            .names_chirho
            .get(name_chirho.0 as usize)
            .ok_or_else(|| invalid_interface_chirho("export has no original declaration name"))?;
        self.charge_chirho(
            info_chirho.module_chirho.len() + info_chirho.occurrence_chirho.len() + 1,
        )?;
        Ok(info_chirho.original_chirho())
    }
    fn ty_chirho(
        &mut self,
        ty_chirho: TyChirho,
        binders_chirho: &HashMap<TyChirho, u32>,
        depth_chirho: usize,
    ) -> Result<InterfaceTypeChirho, TcErrorChirho> {
        self.charge_chirho(1)?;
        if depth_chirho > 128 {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::ResourceChirho,
                "checked interface type depth exhausted",
            ));
        }
        let arena_chirho = &self.checked_chirho.solver_chirho.arena_chirho;
        let ty_chirho = arena_chirho.resolve_chirho(ty_chirho)?;
        if let Some(index_chirho) = binders_chirho.get(&ty_chirho) {
            return Ok(InterfaceTypeChirho::BoundChirho(*index_chirho));
        }
        Ok(match arena_chirho.node_chirho(ty_chirho)?.clone() {
            TyNodeChirho::ConChirho(TyConChirho::DeclaredChirho(name_chirho)) => {
                InterfaceTypeChirho::DeclaredChirho(self.original_chirho(name_chirho)?)
            }
            TyNodeChirho::ConChirho(TyConChirho::ClassChirho(name_chirho)) => {
                InterfaceTypeChirho::ClassChirho(self.original_chirho(name_chirho)?)
            }
            TyNodeChirho::ConChirho(con_chirho) => InterfaceTypeChirho::BuiltinChirho(
                builtin_out_chirho(con_chirho)
                    .ok_or_else(|| invalid_interface_chirho("unrecognized export constructor"))?,
            ),
            TyNodeChirho::FunChirho(left_chirho, right_chirho) => InterfaceTypeChirho::FunChirho(
                Box::new(self.ty_chirho(left_chirho, binders_chirho, depth_chirho + 1)?),
                Box::new(self.ty_chirho(right_chirho, binders_chirho, depth_chirho + 1)?),
            ),
            TyNodeChirho::AppChirho(left_chirho, right_chirho) => InterfaceTypeChirho::AppChirho(
                Box::new(self.ty_chirho(left_chirho, binders_chirho, depth_chirho + 1)?),
                Box::new(self.ty_chirho(right_chirho, binders_chirho, depth_chirho + 1)?),
            ),
            TyNodeChirho::TupleChirho(items_chirho) => InterfaceTypeChirho::TupleChirho(
                items_chirho
                    .into_iter()
                    .map(|item_chirho| {
                        self.ty_chirho(item_chirho, binders_chirho, depth_chirho + 1)
                    })
                    .collect::<Result<_, _>>()?,
            ),
            _ => {
                return Err(invalid_interface_chirho(
                    "only closed finally-zonked schemes may leave a module",
                ));
            }
        })
    }
    fn variable_kind_chirho(&self, ty_chirho: TyChirho) -> Result<TyChirho, TcErrorChirho> {
        let arena_chirho = &self.checked_chirho.solver_chirho.arena_chirho;
        Ok(
            match arena_chirho.node_chirho(arena_chirho.resolve_chirho(ty_chirho)?)? {
                TyNodeChirho::MetaChirho(meta_chirho) => {
                    arena_chirho.meta_cell_chirho(*meta_chirho)?.kind_chirho
                }
                TyNodeChirho::RigidChirho(rigid_chirho) => {
                    arena_chirho.rigid_cell_chirho(*rigid_chirho)?.kind_chirho
                }
                _ => {
                    return Err(invalid_interface_chirho(
                        "export scheme binder has no creation-time kind",
                    ));
                }
            },
        )
    }
    fn scheme_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
    ) -> Result<InterfaceSchemeChirho, TcErrorChirho> {
        let binders_chirho: HashMap<_, _> = scheme_chirho
            .quantified_chirho
            .iter()
            .enumerate()
            .map(|(index_chirho, ty_chirho)| (*ty_chirho, index_chirho as u32))
            .collect();
        let mut kinds_chirho = Vec::new();
        for variable_chirho in &scheme_chirho.quantified_chirho {
            let kind_chirho = self.variable_kind_chirho(*variable_chirho)?;
            kinds_chirho.push(self.ty_chirho(kind_chirho, &binders_chirho, 0)?);
        }
        let context_chirho = scheme_chirho
            .context_chirho
            .iter()
            .map(|predicate_chirho| self.ty_chirho(*predicate_chirho, &binders_chirho, 0))
            .collect::<Result<_, _>>()?;
        Ok(InterfaceSchemeChirho {
            kinds_chirho,
            context_chirho,
            body_chirho: self.ty_chirho(scheme_chirho.body_chirho, &binders_chirho, 0)?,
        })
    }
}

pub(super) fn merge_chirho<TChirho: Clone + PartialEq>(
    target_chirho: &mut BTreeMap<OriginalNameChirho, TChirho>,
    source_chirho: &BTreeMap<OriginalNameChirho, TChirho>,
) -> Result<(), TcErrorChirho> {
    for (name_chirho, value_chirho) in source_chirho {
        if let Some(existing_chirho) = target_chirho.get(name_chirho) {
            if existing_chirho != value_chirho {
                return Err(invalid_interface_chirho(
                    "conflicting checked declarations for one original name",
                ));
            }
        } else {
            target_chirho.insert(name_chirho.clone(), value_chirho.clone());
        }
    }
    Ok(())
}

/// Produce arena-free data only after check_module has solved and finished the module.
/// The caller shares this output allowance across the source graph.
pub fn project_interface_chirho(
    checked_chirho: &CheckedModuleChirho,
    left_chirho: &mut usize,
) -> Result<InterfaceChirho, TcErrorChirho> {
    let mut projection_chirho = ProjectionChirho {
        checked_chirho,
        left_chirho,
    };
    let module_chirho = &checked_chirho.module_chirho;
    projection_chirho.charge_chirho(
        module_chirho
            .fixities_chirho
            .keys()
            .map(|name_chirho| {
                name_chirho.module_chirho.len() + name_chirho.occurrence_chirho.len() + 2
            })
            .sum::<usize>()
            + module_chirho.sliced_units_chirho.len(),
    )?;
    let mut result_chirho = InterfaceChirho {
        unit_chirho: module_chirho.unit_chirho.clone(),
        module_chirho: module_chirho.module_chirho.clone(),
        exports_chirho: Vec::new(),
        fixities_chirho: module_chirho.fixities_chirho.clone(),
        completeness_chirho: module_chirho.completeness_chirho,
        sliced_units_chirho: module_chirho.sliced_units_chirho.clone(),
        wired_types_chirho: BTreeMap::new(),
        data_chirho: BTreeMap::new(),
        values_chirho: BTreeMap::new(),
        classes_chirho: BTreeMap::new(),
        instances_chirho: BTreeMap::new(),
    };
    for import_chirho in &module_chirho.imports_chirho {
        let iface_chirho = &import_chirho.interface_chirho;
        // Inherited records were already closed by their producer. Count every retained copy.
        projection_chirho.charge_chirho(interface_weight_chirho(iface_chirho)?)?;
        merge_chirho(
            &mut result_chirho.fixities_chirho,
            &iface_chirho.fixities_chirho,
        )?;
        merge_chirho(
            &mut result_chirho.wired_types_chirho,
            &iface_chirho.wired_types_chirho,
        )?;
        merge_chirho(&mut result_chirho.data_chirho, &iface_chirho.data_chirho)?;
        merge_chirho(
            &mut result_chirho.values_chirho,
            &iface_chirho.values_chirho,
        )?;
        merge_chirho(
            &mut result_chirho.classes_chirho,
            &iface_chirho.classes_chirho,
        )?;
        merge_chirho(
            &mut result_chirho.instances_chirho,
            &iface_chirho.instances_chirho,
        )?;
    }
    let mut values_chirho = Vec::new();
    for declaration_chirho in &module_chirho.data_chirho {
        let name_chirho = projection_chirho.original_chirho(declaration_chirho.name_chirho)?;
        result_chirho
            .data_chirho
            .insert(name_chirho, declaration_chirho.parameters_chirho.len());
        values_chirho.extend(
            declaration_chirho
                .constructors_chirho
                .iter()
                .map(|constructor_chirho| constructor_chirho.name_chirho),
        );
    }
    for declaration_chirho in &module_chirho.classes_chirho {
        let info_chirho = checked_chirho
            .solver_chirho
            .class_info_chirho(declaration_chirho.name_chirho)?;
        let name_chirho = projection_chirho.original_chirho(declaration_chirho.name_chirho)?;
        let binders_chirho = HashMap::from([(info_chirho.parameter_chirho, 0)]);
        let parameter_kind_chirho = projection_chirho.ty_chirho(
            projection_chirho.variable_kind_chirho(info_chirho.parameter_chirho)?,
            &binders_chirho,
            0,
        )?;
        let superclasses_chirho = info_chirho
            .superclasses_chirho
            .iter()
            .map(|ty_chirho| projection_chirho.ty_chirho(*ty_chirho, &binders_chirho, 0))
            .collect::<Result<_, _>>()?;
        let superclass_selectors_chirho = info_chirho
            .superclass_selectors_chirho
            .iter()
            .map(|id_chirho| projection_chirho.original_chirho(*id_chirho))
            .collect::<Result<_, _>>()?;
        let mut methods_chirho = Vec::new();
        for method_chirho in &declaration_chirho.methods_chirho {
            methods_chirho.push(InterfaceMethodChirho {
                name_chirho: projection_chirho.original_chirho(method_chirho.name_chirho)?,
                default_chirho: method_chirho
                    .default_chirho
                    .map(|id_chirho| projection_chirho.original_chirho(id_chirho))
                    .transpose()?,
                scheme_chirho: projection_chirho
                    .scheme_chirho(&checked_chirho.schemes_chirho[&method_chirho.name_chirho])?,
            });
            values_chirho.push(method_chirho.name_chirho);
        }
        values_chirho.extend(info_chirho.superclass_selectors_chirho.iter().copied());
        result_chirho.classes_chirho.insert(
            name_chirho.clone(),
            InterfaceClassChirho {
                name_chirho,
                parameter_kind_chirho,
                superclasses_chirho,
                superclass_selectors_chirho,
                methods_chirho,
            },
        );
    }
    for declaration_chirho in &module_chirho.instances_chirho {
        let dfun_chirho = projection_chirho.original_chirho(declaration_chirho.name_chirho)?;
        result_chirho.instances_chirho.insert(
            dfun_chirho.clone(),
            InterfaceInstanceChirho {
                dfun_chirho,
                class_chirho: projection_chirho.original_chirho(declaration_chirho.class_chirho)?,
                scheme_chirho: projection_chirho.scheme_chirho(
                    &checked_chirho.schemes_chirho[&declaration_chirho.name_chirho],
                )?,
            },
        );
        values_chirho.push(declaration_chirho.name_chirho);
    }
    values_chirho.extend(
        module_chirho
            .bindings_chirho
            .iter()
            .map(|binding_chirho| binding_chirho.name_chirho),
    );
    for id_chirho in values_chirho {
        let name_chirho = projection_chirho.original_chirho(id_chirho)?;
        let scheme_chirho = checked_chirho
            .schemes_chirho
            .get(&id_chirho)
            .ok_or_else(|| invalid_interface_chirho("exported global has no checked scheme"))?;
        result_chirho.values_chirho.insert(
            name_chirho,
            InterfaceValueChirho {
                scheme_chirho: projection_chirho.scheme_chirho(scheme_chirho)?,
                constructor_arity_chirho: checked_chirho
                    .constructors_chirho
                    .get(&id_chirho)
                    .map(Vec::len),
            },
        );
    }
    for id_chirho in &module_chirho.exports_chirho {
        result_chirho
            .exports_chirho
            .push(projection_chirho.original_chirho(*id_chirho)?);
    }
    Ok(result_chirho)
}

/// Size accounting is iterative so a pre-existing tree cannot bypass the graph allowance.
fn interface_weight_chirho(interface_chirho: &InterfaceChirho) -> Result<usize, TcErrorChirho> {
    let mut count_chirho = interface_chirho
        .fixities_chirho
        .keys()
        .map(|name_chirho| {
            name_chirho.module_chirho.len() + name_chirho.occurrence_chirho.len() + 2
        })
        .sum::<usize>()
        + interface_chirho
            .wired_types_chirho
            .keys()
            .map(|name_chirho| {
                name_chirho.module_chirho.len() + name_chirho.occurrence_chirho.len() + 1
            })
            .sum::<usize>()
        + interface_chirho.sliced_units_chirho.len()
        + interface_chirho.data_chirho.len()
        + interface_chirho.values_chirho.len()
        + interface_chirho.classes_chirho.len()
        + interface_chirho.instances_chirho.len();
    let mut trees_chirho = Vec::new();
    for (name_chirho, value_chirho) in &interface_chirho.values_chirho {
        count_chirho = count_chirho
            .checked_add(name_chirho.module_chirho.len() + name_chirho.occurrence_chirho.len() + 1)
            .ok_or_else(|| invalid_interface_chirho("interface size overflow"))?;
        trees_chirho.extend(value_chirho.scheme_chirho.kinds_chirho.iter());
        trees_chirho.extend(value_chirho.scheme_chirho.context_chirho.iter());
        trees_chirho.push(&value_chirho.scheme_chirho.body_chirho);
    }
    for class_chirho in interface_chirho.classes_chirho.values() {
        trees_chirho.push(&class_chirho.parameter_kind_chirho);
        trees_chirho.extend(class_chirho.superclasses_chirho.iter());
        for method_chirho in &class_chirho.methods_chirho {
            trees_chirho.extend(method_chirho.scheme_chirho.kinds_chirho.iter());
            trees_chirho.extend(method_chirho.scheme_chirho.context_chirho.iter());
            trees_chirho.push(&method_chirho.scheme_chirho.body_chirho);
        }
    }
    for instance_chirho in interface_chirho.instances_chirho.values() {
        trees_chirho.extend(instance_chirho.scheme_chirho.kinds_chirho.iter());
        trees_chirho.extend(instance_chirho.scheme_chirho.context_chirho.iter());
        trees_chirho.push(&instance_chirho.scheme_chirho.body_chirho);
    }
    while let Some(ty_chirho) = trees_chirho.pop() {
        count_chirho = count_chirho
            .checked_add(1)
            .ok_or_else(|| invalid_interface_chirho("interface size overflow"))?;
        match ty_chirho {
            InterfaceTypeChirho::DeclaredChirho(name_chirho)
            | InterfaceTypeChirho::ClassChirho(name_chirho) => {
                count_chirho +=
                    name_chirho.module_chirho.len() + name_chirho.occurrence_chirho.len()
            }
            InterfaceTypeChirho::AppChirho(left_chirho, right_chirho)
            | InterfaceTypeChirho::FunChirho(left_chirho, right_chirho) => {
                trees_chirho.extend([left_chirho.as_ref(), right_chirho.as_ref()])
            }
            InterfaceTypeChirho::TupleChirho(items_chirho) => trees_chirho.extend(items_chirho),
            _ => {}
        }
    }
    Ok(count_chirho)
}
