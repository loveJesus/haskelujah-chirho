// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

struct ReinternChirho<'checked_chirho> {
    checked_chirho: &'checked_chirho mut CheckedModuleChirho,
    names_chirho: BTreeMap<OriginalNameChirho, NameIdChirho>,
}
impl ReinternChirho<'_> {
    fn id_chirho(&self, name_chirho: &OriginalNameChirho) -> Result<NameIdChirho, TcErrorChirho> {
        self.names_chirho.get(name_chirho).copied().ok_or_else(|| {
            invalid_interface_chirho("interface reference lacks its original declaration")
        })
    }
    fn ty_chirho(
        &mut self,
        ty_chirho: &InterfaceTypeChirho,
        binders_chirho: &[TyChirho],
        depth_chirho: usize,
    ) -> Result<TyChirho, TcErrorChirho> {
        self.checked_chirho
            .solver_chirho
            .arena_chirho
            .reserve_auxiliary_chirho()?;
        if depth_chirho > 128 {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::ResourceChirho,
                "interface import type depth exhausted",
            ));
        }
        let node_chirho = match ty_chirho {
            InterfaceTypeChirho::BuiltinChirho(con_chirho) => {
                TyNodeChirho::ConChirho(builtin_in_chirho(*con_chirho))
            }
            InterfaceTypeChirho::DeclaredChirho(name_chirho) => {
                TyNodeChirho::ConChirho(TyConChirho::DeclaredChirho(self.id_chirho(name_chirho)?))
            }
            InterfaceTypeChirho::ClassChirho(name_chirho) => {
                TyNodeChirho::ConChirho(TyConChirho::ClassChirho(self.id_chirho(name_chirho)?))
            }
            InterfaceTypeChirho::BoundChirho(index_chirho) => {
                return binders_chirho
                    .get(*index_chirho as usize)
                    .copied()
                    .ok_or_else(|| {
                        invalid_interface_chirho("interface binder position is outside its scheme")
                    });
            }
            InterfaceTypeChirho::AppChirho(left_chirho, right_chirho) => TyNodeChirho::AppChirho(
                self.ty_chirho(left_chirho, binders_chirho, depth_chirho + 1)?,
                self.ty_chirho(right_chirho, binders_chirho, depth_chirho + 1)?,
            ),
            InterfaceTypeChirho::FunChirho(left_chirho, right_chirho) => TyNodeChirho::FunChirho(
                self.ty_chirho(left_chirho, binders_chirho, depth_chirho + 1)?,
                self.ty_chirho(right_chirho, binders_chirho, depth_chirho + 1)?,
            ),
            InterfaceTypeChirho::TupleChirho(items_chirho) => TyNodeChirho::TupleChirho(
                items_chirho
                    .iter()
                    .map(|item_chirho| {
                        self.ty_chirho(item_chirho, binders_chirho, depth_chirho + 1)
                    })
                    .collect::<Result<_, _>>()?,
            ),
        };
        let result_chirho = self
            .checked_chirho
            .solver_chirho
            .arena_chirho
            .intern_chirho(node_chirho)?;
        self.checked_chirho
            .solver_chirho
            .arena_chirho
            .kind_chirho(result_chirho)?;
        Ok(result_chirho)
    }
    fn scheme_with_chirho(
        &mut self,
        scheme_chirho: &InterfaceSchemeChirho,
        binders_chirho: Vec<TyChirho>,
    ) -> Result<SchemeChirho, TcErrorChirho> {
        if binders_chirho.len() != scheme_chirho.kinds_chirho.len() {
            return Err(invalid_interface_chirho(
                "class method binder catalogue disagrees",
            ));
        }
        let context_chirho = scheme_chirho
            .context_chirho
            .iter()
            .map(|predicate_chirho| self.ty_chirho(predicate_chirho, &binders_chirho, 0))
            .collect::<Result<Vec<_>, _>>()?;
        let body_chirho = self.ty_chirho(&scheme_chirho.body_chirho, &binders_chirho, 0)?;
        Ok(SchemeChirho {
            quantified_chirho: binders_chirho,
            context_chirho,
            body_chirho,
        })
    }
    fn scheme_chirho(
        &mut self,
        scheme_chirho: &InterfaceSchemeChirho,
    ) -> Result<SchemeChirho, TcErrorChirho> {
        let mut binders_chirho = Vec::new();
        for kind_chirho in &scheme_chirho.kinds_chirho {
            let kind_chirho = self.ty_chirho(kind_chirho, &binders_chirho, 0)?;
            let (_, variable_chirho) = self
                .checked_chirho
                .solver_chirho
                .arena_chirho
                .fresh_rigid_chirho(0, kind_chirho)?;
            binders_chirho.push(variable_chirho);
        }
        self.scheme_with_chirho(scheme_chirho, binders_chirho)
    }
}

impl CheckedModuleChirho {
    /// Import into this fresh solver only. Nothing is published when a record fails.
    pub(crate) fn import_interfaces_chirho(&mut self) -> Result<(), TcErrorChirho> {
        let imports_chirho = self.module_chirho.imports_chirho.clone();
        let mut names_chirho = BTreeMap::new();
        let mut reverse_chirho = HashMap::new();
        for imported_chirho in &imports_chirho {
            for (original_chirho, id_chirho) in &imported_chirho.names_chirho {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                if self
                    .module_chirho
                    .names_chirho
                    .get(id_chirho.0 as usize)
                    .map(NameInfoChirho::original_chirho)
                    .as_ref()
                    != Some(original_chirho)
                {
                    return Err(invalid_interface_chirho(
                        "import name does not retain defining identity",
                    ));
                }
                if let Some(prior_chirho) = names_chirho.insert(original_chirho.clone(), *id_chirho)
                {
                    if prior_chirho != *id_chirho {
                        return Err(invalid_interface_chirho(
                            "one original name acquired two consumer ids",
                        ));
                    }
                }
                if let Some(prior_chirho) =
                    reverse_chirho.insert(*id_chirho, original_chirho.clone())
                {
                    if prior_chirho != *original_chirho {
                        return Err(invalid_interface_chirho(
                            "two original names acquired one consumer id",
                        ));
                    }
                }
            }
        }
        let mut importer_chirho = ReinternChirho {
            checked_chirho: self,
            names_chirho,
        };
        let mut data_chirho = BTreeMap::new();
        let mut classes_chirho = BTreeMap::new();
        let mut values_chirho = BTreeMap::new();
        let mut instances_chirho = BTreeMap::new();
        for imported_chirho in &imports_chirho {
            super::export_chirho::merge_chirho(
                &mut data_chirho,
                &imported_chirho.interface_chirho.data_chirho,
            )?;
            super::export_chirho::merge_chirho(
                &mut classes_chirho,
                &imported_chirho.interface_chirho.classes_chirho,
            )?;
            super::export_chirho::merge_chirho(
                &mut values_chirho,
                &imported_chirho.interface_chirho.values_chirho,
            )?;
            super::export_chirho::merge_chirho(
                &mut instances_chirho,
                &imported_chirho.interface_chirho.instances_chirho,
            )?;
        }
        for (name_chirho, arity_chirho) in data_chirho {
            let id_chirho = importer_chirho.id_chirho(&name_chirho)?;
            importer_chirho
                .checked_chirho
                .solver_chirho
                .arena_chirho
                .declare_chirho(id_chirho, arity_chirho)?;
        }
        for name_chirho in classes_chirho.keys() {
            let id_chirho = importer_chirho.id_chirho(name_chirho)?;
            importer_chirho
                .checked_chirho
                .solver_chirho
                .arena_chirho
                .declare_class_chirho(id_chirho)?;
        }
        for (name_chirho, value_chirho) in values_chirho {
            let id_chirho = importer_chirho.id_chirho(&name_chirho)?;
            let scheme_chirho = importer_chirho.scheme_chirho(&value_chirho.scheme_chirho)?;
            if let Some(arity_chirho) = value_chirho.constructor_arity_chirho {
                let mut body_chirho = scheme_chirho.body_chirho;
                let mut fields_chirho = Vec::new();
                for _field_chirho in 0..arity_chirho {
                    let TyNodeChirho::FunChirho(field_chirho, result_chirho) = importer_chirho
                        .checked_chirho
                        .solver_chirho
                        .arena_chirho
                        .node_chirho(body_chirho)?
                    else {
                        return Err(invalid_interface_chirho(
                            "constructor field catalogue disagrees with scheme",
                        ));
                    };
                    fields_chirho.push(*field_chirho);
                    body_chirho = *result_chirho;
                }
                importer_chirho
                    .checked_chirho
                    .constructors_chirho
                    .insert(id_chirho, fields_chirho);
            }
            importer_chirho
                .checked_chirho
                .names_chirho
                .insert(id_chirho, scheme_chirho.body_chirho);
            importer_chirho
                .checked_chirho
                .schemes_chirho
                .insert(id_chirho, scheme_chirho);
        }
        for (original_chirho, class_chirho) in classes_chirho {
            let name_chirho = importer_chirho.id_chirho(&original_chirho)?;
            if class_chirho.name_chirho != original_chirho {
                return Err(invalid_interface_chirho(
                    "class record has a different owner",
                ));
            }
            let kind_chirho =
                importer_chirho.ty_chirho(&class_chirho.parameter_kind_chirho, &[], 0)?;
            let (_, parameter_chirho) = importer_chirho
                .checked_chirho
                .solver_chirho
                .arena_chirho
                .fresh_rigid_chirho(0, kind_chirho)?;
            let superclasses_chirho = class_chirho
                .superclasses_chirho
                .iter()
                .map(|ty_chirho| importer_chirho.ty_chirho(ty_chirho, &[parameter_chirho], 0))
                .collect::<Result<Vec<_>, _>>()?;
            let superclass_selectors_chirho = class_chirho
                .superclass_selectors_chirho
                .iter()
                .map(|name_chirho| importer_chirho.id_chirho(name_chirho))
                .collect::<Result<Vec<_>, _>>()?;
            if superclasses_chirho.len() != superclass_selectors_chirho.len() {
                return Err(invalid_interface_chirho(
                    "class superclass children disagree",
                ));
            }
            let mut methods_chirho = Vec::new();
            for method_chirho in &class_chirho.methods_chirho {
                let method_id_chirho = importer_chirho.id_chirho(&method_chirho.name_chirho)?;
                let scheme_chirho = importer_chirho
                    .scheme_with_chirho(&method_chirho.scheme_chirho, vec![parameter_chirho])?;
                importer_chirho
                    .checked_chirho
                    .names_chirho
                    .insert(method_id_chirho, scheme_chirho.body_chirho);
                importer_chirho
                    .checked_chirho
                    .schemes_chirho
                    .insert(method_id_chirho, scheme_chirho.clone());
                methods_chirho.push(MethodInfoChirho {
                    name_chirho: method_id_chirho,
                    scheme_chirho,
                });
            }
            importer_chirho
                .checked_chirho
                .solver_chirho
                .classes_chirho
                .insert(
                    name_chirho,
                    ClassInfoChirho {
                        name_chirho,
                        parameter_chirho,
                        superclasses_chirho,
                        superclass_selectors_chirho,
                        methods_chirho,
                    },
                );
        }
        // Class headers permit decoding; validate predicate owners after all class records exist.
        let contexts_chirho: Vec<_> = importer_chirho
            .checked_chirho
            .schemes_chirho
            .values()
            .flat_map(|scheme_chirho| scheme_chirho.context_chirho.iter().copied())
            .collect();
        for predicate_chirho in contexts_chirho {
            importer_chirho
                .checked_chirho
                .solver_chirho
                .predicate_view_chirho(predicate_chirho)?;
        }
        for (original_chirho, instance_chirho) in instances_chirho {
            if instance_chirho.dfun_chirho != original_chirho {
                return Err(invalid_interface_chirho(
                    "instance record has a different dfun owner",
                ));
            }
            let name_chirho = importer_chirho.id_chirho(&original_chirho)?;
            let class_chirho = importer_chirho.id_chirho(&instance_chirho.class_chirho)?;
            let scheme_chirho = importer_chirho
                .checked_chirho
                .schemes_chirho
                .get(&name_chirho)
                .cloned()
                .ok_or_else(|| invalid_interface_chirho("instance dfun has no imported scheme"))?;
            let (actual_class_chirho, head_chirho) =
                importer_chirho
                    .checked_chirho
                    .solver_chirho
                    .predicate_view_chirho(scheme_chirho.body_chirho)?;
            if actual_class_chirho != class_chirho {
                return Err(invalid_interface_chirho(
                    "instance predicate disagrees with owning class",
                ));
            }
            if let Some((shape_chirho, _)) = importer_chirho
                .checked_chirho
                .solver_chirho
                .instance_shape_chirho(head_chirho)?
            {
                if importer_chirho
                    .checked_chirho
                    .solver_chirho
                    .constructor_instances_chirho
                    .contains_key(&(class_chirho, shape_chirho))
                {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::UnsupportedChirho,
                        "overlapping imported instances need demand-aware selection; no instance is chosen",
                    ));
                }
            }
            importer_chirho
                .checked_chirho
                .solver_chirho
                .register_instance_head_chirho(
                    class_chirho,
                    head_chirho,
                    &scheme_chirho.quantified_chirho,
                    name_chirho,
                )?;
            let scope_chirho = importer_chirho.checked_chirho.solver_chirho.root_chirho();
            importer_chirho
                .checked_chirho
                .solver_chirho
                .instances_chirho
                .insert(
                    name_chirho,
                    InstanceInfoChirho {
                        name_chirho,
                        class_chirho,
                        binders_chirho: scheme_chirho.quantified_chirho,
                        context_chirho: scheme_chirho.context_chirho,
                        head_chirho,
                        scope_chirho,
                        superclasses_chirho: Vec::new(),
                        methods_chirho: Vec::new(),
                    },
                );
        }
        Ok(())
    }
}
