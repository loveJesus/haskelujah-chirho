// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Prove Type-pinned data positions before using a GHC2021 header as kind evidence.
//! workflow: next-pipeline-chirho/io-sequencing-chirho
use super::*;
use haskelujah_hir_chirho::DataDeclChirho;
use std::collections::{HashSet, VecDeque};

impl CheckedModuleChirho {
    /// Kind evidence only: build an arena DAG without guessing/checking provisional argument kinds.
    fn kind_evidence_type_chirho(
        &mut self,
        ty_chirho: &SurfaceTypeChirho,
        variables_chirho: &HashMap<NameIdChirho, TyChirho>,
        depth_chirho: usize,
    ) -> Result<TyChirho, TcErrorChirho> {
        self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
        if depth_chirho > 128 {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::ResourceChirho,
                "data kind evidence depth exhausted",
            ));
        }
        let mut head_chirho = ty_chirho;
        let mut arguments_chirho = Vec::new();
        while let SurfaceTypeChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            arguments_chirho.push(argument_chirho.as_ref());
            head_chirho = function_chirho;
        }
        if let SurfaceTypeChirho::SynonymChirho(name_chirho) = head_chirho {
            let synonym_chirho =
                self.synonyms_chirho
                    .get(name_chirho)
                    .cloned()
                    .ok_or_else(|| {
                        TcErrorChirho::new_chirho(
                            TcErrorKindChirho::UnsupportedChirho,
                            "kind evidence needs a checked synonym",
                        )
                    })?;
            arguments_chirho.reverse();
            if arguments_chirho.len() < synonym_chirho.scheme_chirho.quantified_chirho.len() {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::UnsupportedChirho,
                    "kind evidence does not admit an unsaturated synonym",
                ));
            }
            let mut replacements_chirho = HashMap::new();
            for (&parameter_chirho, argument_chirho) in synonym_chirho
                .scheme_chirho
                .quantified_chirho
                .iter()
                .zip(&arguments_chirho)
            {
                replacements_chirho.insert(
                    parameter_chirho,
                    self.kind_evidence_type_chirho(
                        argument_chirho,
                        variables_chirho,
                        depth_chirho + 1,
                    )?,
                );
            }
            let mut body_chirho = self.solver_chirho.arena_chirho.replace_chirho(
                synonym_chirho.scheme_chirho.body_chirho,
                replacements_chirho,
            )?;
            for argument_chirho in arguments_chirho
                .iter()
                .skip(synonym_chirho.scheme_chirho.quantified_chirho.len())
            {
                let argument_chirho = self.kind_evidence_type_chirho(
                    argument_chirho,
                    variables_chirho,
                    depth_chirho + 1,
                )?;
                body_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .intern_chirho(TyNodeChirho::AppChirho(body_chirho, argument_chirho))?;
            }
            return Ok(body_chirho);
        }
        let node_chirho = match ty_chirho {
            SurfaceTypeChirho::BoolChirho => TyNodeChirho::ConChirho(TyConChirho::BoolChirho),
            SurfaceTypeChirho::OrderingChirho => {
                TyNodeChirho::ConChirho(TyConChirho::OrderingChirho)
            }
            SurfaceTypeChirho::CharChirho => TyNodeChirho::ConChirho(TyConChirho::CharChirho),
            SurfaceTypeChirho::IntegerChirho => TyNodeChirho::ConChirho(TyConChirho::IntegerChirho),
            SurfaceTypeChirho::IntChirho => TyNodeChirho::ConChirho(TyConChirho::IntChirho),
            SurfaceTypeChirho::IOChirho => TyNodeChirho::ConChirho(TyConChirho::IOChirho),
            SurfaceTypeChirho::DeclaredChirho(name_chirho) => {
                TyNodeChirho::ConChirho(TyConChirho::DeclaredChirho(*name_chirho))
            }
            SurfaceTypeChirho::VariableChirho(name_chirho) => {
                return variables_chirho.get(name_chirho).copied().ok_or_else(|| {
                    TcErrorChirho::new_chirho(
                        TcErrorKindChirho::UnsupportedChirho,
                        "data kind evidence lost a declared variable",
                    )
                });
            }
            SurfaceTypeChirho::AppChirho(function_chirho, argument_chirho) => {
                TyNodeChirho::AppChirho(
                    self.kind_evidence_type_chirho(
                        function_chirho,
                        variables_chirho,
                        depth_chirho + 1,
                    )?,
                    self.kind_evidence_type_chirho(
                        argument_chirho,
                        variables_chirho,
                        depth_chirho + 1,
                    )?,
                )
            }
            SurfaceTypeChirho::FunChirho(argument_chirho, result_chirho) => {
                TyNodeChirho::FunChirho(
                    self.kind_evidence_type_chirho(
                        argument_chirho,
                        variables_chirho,
                        depth_chirho + 1,
                    )?,
                    self.kind_evidence_type_chirho(
                        result_chirho,
                        variables_chirho,
                        depth_chirho + 1,
                    )?,
                )
            }
            SurfaceTypeChirho::ListChirho(item_chirho) => {
                let list_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .con_chirho(TyConChirho::ListChirho)?;
                TyNodeChirho::AppChirho(
                    list_chirho,
                    self.kind_evidence_type_chirho(
                        item_chirho,
                        variables_chirho,
                        depth_chirho + 1,
                    )?,
                )
            }
            SurfaceTypeChirho::TupleChirho(items_chirho) => TyNodeChirho::TupleChirho(
                items_chirho
                    .iter()
                    .map(|item_chirho| {
                        self.kind_evidence_type_chirho(
                            item_chirho,
                            variables_chirho,
                            depth_chirho + 1,
                        )
                    })
                    .collect::<Result<_, _>>()?,
            ),
            SurfaceTypeChirho::SynonymChirho(_) => unreachable!(),
        };
        self.solver_chirho.arena_chirho.intern_chirho(node_chirho)
    }
    fn recursive_data_kind_guard_chirho(
        &mut self,
        declarations_chirho: &[DataDeclChirho],
        roots_chirho: &[(NameIdChirho, TyChirho)],
    ) -> Result<(), TcErrorChirho> {
        let indices_chirho: HashMap<_, _> = declarations_chirho
            .iter()
            .enumerate()
            .map(|(index_chirho, declaration_chirho)| {
                (declaration_chirho.name_chirho, index_chirho)
            })
            .collect();
        let mut pending_chirho = vec![0usize; declarations_chirho.len()];
        let mut consumers_chirho = vec![Vec::new(); declarations_chirho.len()];
        let mut seen_chirho = HashSet::new();
        let mut dependencies_chirho = HashSet::new();
        let mut work_chirho = roots_chirho.to_vec();
        while let Some((owner_chirho, ty_chirho)) = work_chirho.pop() {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            if !seen_chirho.insert((owner_chirho, ty_chirho)) {
                continue;
            }
            match self
                .solver_chirho
                .arena_chirho
                .node_chirho(ty_chirho)?
                .clone()
            {
                TyNodeChirho::ConChirho(TyConChirho::DeclaredChirho(name_chirho))
                    if indices_chirho.contains_key(&name_chirho) =>
                {
                    let owner_chirho = indices_chirho[&owner_chirho];
                    let producer_chirho = indices_chirho[&name_chirho];
                    if dependencies_chirho.insert((owner_chirho, producer_chirho)) {
                        pending_chirho[owner_chirho] += 1;
                        consumers_chirho[producer_chirho].push(owner_chirho);
                    }
                }
                TyNodeChirho::AppChirho(a_chirho, b_chirho)
                | TyNodeChirho::FunChirho(a_chirho, b_chirho) => {
                    work_chirho.extend([(owner_chirho, a_chirho), (owner_chirho, b_chirho)])
                }
                TyNodeChirho::TupleChirho(items_chirho) => work_chirho.extend(
                    items_chirho
                        .into_iter()
                        .map(|item_chirho| (owner_chirho, item_chirho)),
                ),
                _ => {}
            }
        }
        let mut ready_chirho: VecDeque<_> = pending_chirho
            .iter()
            .enumerate()
            .filter_map(|(index_chirho, &count_chirho)| (count_chirho == 0).then_some(index_chirho))
            .collect();
        while let Some(producer_chirho) = ready_chirho.pop_front() {
            for &consumer_chirho in &consumers_chirho[producer_chirho] {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                pending_chirho[consumer_chirho] -= 1;
                if pending_chirho[consumer_chirho] == 0 {
                    ready_chirho.push_back(consumer_chirho);
                }
            }
        }
        seen_chirho.clear();
        work_chirho.extend_from_slice(roots_chirho);
        let type_chirho = self
            .solver_chirho
            .arena_chirho
            .con_chirho(TyConChirho::TypeChirho)?;
        while let Some((owner_chirho, ty_chirho)) = work_chirho.pop() {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            if pending_chirho[indices_chirho[&owner_chirho]] == 0
                || !seen_chirho.insert((owner_chirho, ty_chirho))
            {
                continue;
            }
            match self
                .solver_chirho
                .arena_chirho
                .node_chirho(ty_chirho)?
                .clone()
            {
                TyNodeChirho::AppChirho(_, _) => {
                    let mut head_chirho = ty_chirho;
                    let mut arguments_chirho = Vec::new();
                    while let TyNodeChirho::AppChirho(function_chirho, argument_chirho) =
                        *self.solver_chirho.arena_chirho.node_chirho(head_chirho)?
                    {
                        self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                        arguments_chirho.push(argument_chirho);
                        head_chirho = function_chirho;
                    }
                    if let TyNodeChirho::ConChirho(TyConChirho::DeclaredChirho(name_chirho)) =
                        *self.solver_chirho.arena_chirho.node_chirho(head_chirho)?
                        && indices_chirho
                            .get(&name_chirho)
                            .is_some_and(|&index_chirho| pending_chirho[index_chirho] > 0)
                    {
                        for &argument_chirho in &arguments_chirho {
                            match self.solver_chirho.arena_chirho.kind_chirho(argument_chirho) {
                                Ok(kind_chirho) if kind_chirho == type_chirho => {}
                                Err(error_chirho)
                                    if error_chirho.kind_chirho
                                        == TcErrorKindChirho::ResourceChirho =>
                                {
                                    return Err(error_chirho);
                                }
                                _ => {
                                    return Err(TcErrorChirho::new_chirho(
                                        TcErrorKindChirho::UnsupportedChirho,
                                        "recursive data headers need declaration-group kind inference",
                                    ));
                                }
                            }
                        }
                    }
                    work_chirho.extend(
                        arguments_chirho
                            .into_iter()
                            .map(|argument_chirho| (owner_chirho, argument_chirho)),
                    );
                }
                TyNodeChirho::FunChirho(a_chirho, b_chirho) => {
                    work_chirho.extend([(owner_chirho, a_chirho), (owner_chirho, b_chirho)])
                }
                TyNodeChirho::TupleChirho(items_chirho) => work_chirho.extend(
                    items_chirho
                        .into_iter()
                        .map(|item_chirho| (owner_chirho, item_chirho)),
                ),
                _ => {}
            }
        }
        Ok(())
    }
    pub(super) fn data_kind_pins_chirho(
        &mut self,
        declarations_chirho: &[DataDeclChirho],
    ) -> Result<(), TcErrorChirho> {
        let type_chirho = self
            .solver_chirho
            .arena_chirho
            .con_chirho(TyConChirho::TypeChirho)?;
        let mut positions_chirho = HashMap::new();
        let mut variables_chirho = HashMap::new();
        let mut slots_chirho = HashMap::new();
        let mut pinned_chirho = Vec::new();
        let mut edges_chirho = Vec::<Vec<usize>>::new();
        for declaration_chirho in declarations_chirho {
            let mut ids_chirho = Vec::new();
            for &parameter_chirho in &declaration_chirho.parameters_chirho {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                let (_, ty_chirho) = self
                    .solver_chirho
                    .arena_chirho
                    .fresh_rigid_chirho(0, type_chirho)?;
                let index_chirho = pinned_chirho.len();
                pinned_chirho.push(false);
                edges_chirho.push(Vec::new());
                variables_chirho.insert(parameter_chirho, ty_chirho);
                slots_chirho.insert(ty_chirho, index_chirho);
                ids_chirho.push(index_chirho);
            }
            positions_chirho.insert(declaration_chirho.name_chirho, ids_chirho);
        }
        let mut work_chirho = Vec::new();
        let mut roots_chirho = Vec::new();
        for declaration_chirho in declarations_chirho {
            for constructor_chirho in &declaration_chirho.constructors_chirho {
                for field_chirho in &constructor_chirho.fields_chirho {
                    let field_chirho =
                        self.kind_evidence_type_chirho(field_chirho, &variables_chirho, 0)?;
                    roots_chirho.push((declaration_chirho.name_chirho, field_chirho));
                    work_chirho.push((field_chirho, true));
                }
            }
        }
        let mut seen_chirho = HashSet::new();
        let mut ready_chirho = VecDeque::new();
        if !self.module_chirho.ghc2021_chirho {
            self.recursive_data_kind_guard_chirho(declarations_chirho, &roots_chirho)?;
            return Ok(());
        }
        let mut seen_types_chirho = HashSet::new();
        while let Some((ty_chirho, demand_type_chirho)) = work_chirho.pop() {
            if !seen_types_chirho.insert((ty_chirho, demand_type_chirho)) {
                continue;
            }
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            if !seen_chirho.insert((ty_chirho, demand_type_chirho)) {
                continue;
            }
            if demand_type_chirho
                && let Some(&slot_chirho) = slots_chirho.get(&ty_chirho)
                && !pinned_chirho[slot_chirho]
            {
                pinned_chirho[slot_chirho] = true;
                ready_chirho.push_back(slot_chirho);
            }
            match self
                .solver_chirho
                .arena_chirho
                .node_chirho(ty_chirho)?
                .clone()
            {
                TyNodeChirho::FunChirho(a_chirho, b_chirho) => {
                    work_chirho.extend([(a_chirho, true), (b_chirho, true)])
                }
                TyNodeChirho::TupleChirho(items_chirho) => work_chirho.extend(
                    items_chirho
                        .into_iter()
                        .map(|item_chirho| (item_chirho, true)),
                ),
                TyNodeChirho::AppChirho(_, _) => {
                    let mut head_chirho = ty_chirho;
                    let mut arguments_chirho = Vec::new();
                    while let TyNodeChirho::AppChirho(function_chirho, argument_chirho) =
                        *self.solver_chirho.arena_chirho.node_chirho(head_chirho)?
                    {
                        self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                        arguments_chirho.push(argument_chirho);
                        head_chirho = function_chirho;
                    }
                    arguments_chirho.reverse();
                    let con_chirho = self
                        .solver_chirho
                        .arena_chirho
                        .node_chirho(head_chirho)?
                        .clone();
                    for (index_chirho, argument_chirho) in arguments_chirho.into_iter().enumerate()
                    {
                        let mut required_chirho = false;
                        match con_chirho {
                            TyNodeChirho::ConChirho(
                                TyConChirho::IOChirho | TyConChirho::ListChirho,
                            ) => required_chirho = index_chirho == 0,
                            TyNodeChirho::ConChirho(TyConChirho::DeclaredChirho(name_chirho)) => {
                                if let Some(&producer_chirho) = positions_chirho
                                    .get(&name_chirho)
                                    .and_then(|positions_chirho| positions_chirho.get(index_chirho))
                                {
                                    if let Some(&consumer_chirho) =
                                        slots_chirho.get(&argument_chirho)
                                    {
                                        self.solver_chirho
                                            .arena_chirho
                                            .reserve_auxiliary_chirho()?;
                                        edges_chirho[producer_chirho].push(consumer_chirho);
                                    }
                                } else {
                                    required_chirho = !self
                                        .solver_chirho
                                        .arena_chirho
                                        .open_data_kinds_chirho(name_chirho)
                                        .and_then(|open_chirho| open_chirho.get(index_chirho))
                                        .copied()
                                        .unwrap_or(false);
                                }
                            }
                            _ => {}
                        }
                        work_chirho.push((argument_chirho, required_chirho));
                    }
                }
                _ => {}
            }
        }
        while let Some(producer_chirho) = ready_chirho.pop_front() {
            for &consumer_chirho in &edges_chirho[producer_chirho] {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                if !pinned_chirho[consumer_chirho] {
                    pinned_chirho[consumer_chirho] = true;
                    ready_chirho.push_back(consumer_chirho);
                }
            }
        }
        for declaration_chirho in declarations_chirho {
            let open_chirho = positions_chirho[&declaration_chirho.name_chirho]
                .iter()
                .map(|&index_chirho| !pinned_chirho[index_chirho])
                .collect();
            self.solver_chirho
                .arena_chirho
                .set_open_data_kinds_chirho(declaration_chirho.name_chirho, open_chirho)?;
        }
        Ok(())
    }
}
