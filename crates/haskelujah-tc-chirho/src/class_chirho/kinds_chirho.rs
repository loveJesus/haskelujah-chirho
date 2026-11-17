// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Haskell 98 monomorphic kind inference. Only checked Type/arrow kinds escape.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_hir_chirho::{ClassDeclChirho, SurfacePredicateChirho, SurfaceSignatureChirho};

#[derive(Clone, Copy)]
enum KindNodeChirho {
    TypeChirho,
    ConstraintChirho,
    VariableChirho(Option<usize>),
    ArrowChirho(usize, usize),
}
struct KindInferenceChirho<'checked_chirho> {
    checked_chirho: &'checked_chirho mut CheckedModuleChirho,
    nodes_chirho: Vec<KindNodeChirho>,
    variables_chirho: HashMap<NameIdChirho, usize>,
    classes_chirho: HashMap<NameIdChirho, usize>,
    finished_chirho: HashMap<usize, TyChirho>,
}
impl<'checked_chirho> KindInferenceChirho<'checked_chirho> {
    fn new_chirho(checked_chirho: &'checked_chirho mut CheckedModuleChirho) -> Self {
        Self {
            checked_chirho,
            nodes_chirho: Vec::new(),
            variables_chirho: HashMap::new(),
            classes_chirho: HashMap::new(),
            finished_chirho: HashMap::new(),
        }
    }
    fn mismatch_chirho() -> TcErrorChirho {
        TcErrorChirho::new_chirho(
            TcErrorKindChirho::KindMismatchChirho,
            "inconsistent monomorphic kind constraints",
        )
    }
    fn charge_chirho(&mut self) -> Result<(), TcErrorChirho> {
        self.checked_chirho
            .solver_chirho
            .arena_chirho
            .reserve_auxiliary_chirho()
    }
    fn node_chirho(&mut self, node_chirho: KindNodeChirho) -> Result<usize, TcErrorChirho> {
        self.charge_chirho()?;
        let id_chirho = self.nodes_chirho.len();
        self.nodes_chirho.push(node_chirho);
        Ok(id_chirho)
    }
    fn variable_chirho(&mut self, name_chirho: NameIdChirho) -> Result<usize, TcErrorChirho> {
        if let Some(&id_chirho) = self.variables_chirho.get(&name_chirho) {
            return Ok(id_chirho);
        }
        let id_chirho = self.node_chirho(KindNodeChirho::VariableChirho(None))?;
        self.variables_chirho.insert(name_chirho, id_chirho);
        Ok(id_chirho)
    }
    fn resolve_chirho(&mut self, mut id_chirho: usize) -> Result<usize, TcErrorChirho> {
        let mut path_chirho = Vec::new();
        while let KindNodeChirho::VariableChirho(Some(next_chirho)) = self.nodes_chirho[id_chirho] {
            self.charge_chirho()?;
            path_chirho.push(id_chirho);
            id_chirho = next_chirho;
        }
        for previous_chirho in path_chirho {
            self.nodes_chirho[previous_chirho] = KindNodeChirho::VariableChirho(Some(id_chirho));
        }
        Ok(id_chirho)
    }
    fn unify_chirho(
        &mut self,
        left_chirho: usize,
        right_chirho: usize,
    ) -> Result<(), TcErrorChirho> {
        let mut work_chirho = vec![(left_chirho, right_chirho)];
        while let Some((left_chirho, right_chirho)) = work_chirho.pop() {
            self.charge_chirho()?;
            let left_chirho = self.resolve_chirho(left_chirho)?;
            let right_chirho = self.resolve_chirho(right_chirho)?;
            if left_chirho == right_chirho {
                continue;
            }
            match (
                self.nodes_chirho[left_chirho],
                self.nodes_chirho[right_chirho],
            ) {
                (KindNodeChirho::VariableChirho(None), _)
                | (_, KindNodeChirho::VariableChirho(None)) => {
                    let (variable_chirho, image_chirho) = if matches!(
                        self.nodes_chirho[left_chirho],
                        KindNodeChirho::VariableChirho(None)
                    ) {
                        (left_chirho, right_chirho)
                    } else {
                        (right_chirho, left_chirho)
                    };
                    let mut occurs_chirho = vec![image_chirho];
                    let mut seen_chirho = std::collections::HashSet::new();
                    while let Some(id_chirho) = occurs_chirho.pop() {
                        self.charge_chirho()?;
                        let id_chirho = self.resolve_chirho(id_chirho)?;
                        if id_chirho == variable_chirho {
                            return Err(Self::mismatch_chirho());
                        }
                        if seen_chirho.insert(id_chirho)
                            && let KindNodeChirho::ArrowChirho(a_chirho, b_chirho) =
                                self.nodes_chirho[id_chirho]
                        {
                            occurs_chirho.extend([a_chirho, b_chirho]);
                        }
                    }
                    self.nodes_chirho[variable_chirho] =
                        KindNodeChirho::VariableChirho(Some(image_chirho));
                }
                (
                    KindNodeChirho::ArrowChirho(a_chirho, b_chirho),
                    KindNodeChirho::ArrowChirho(c_chirho, d_chirho),
                ) => work_chirho.extend([(a_chirho, c_chirho), (b_chirho, d_chirho)]),
                (KindNodeChirho::TypeChirho, KindNodeChirho::TypeChirho)
                | (KindNodeChirho::ConstraintChirho, KindNodeChirho::ConstraintChirho) => {}
                _ => return Err(Self::mismatch_chirho()),
            }
        }
        Ok(())
    }
    fn import_chirho(
        &mut self,
        kind_chirho: TyChirho,
        depth_chirho: usize,
    ) -> Result<usize, TcErrorChirho> {
        self.depth_chirho(depth_chirho)?;
        let arena_chirho = &self.checked_chirho.solver_chirho.arena_chirho;
        let node_chirho = arena_chirho
            .node_chirho(arena_chirho.resolve_chirho(kind_chirho)?)?
            .clone();
        let node_chirho = match node_chirho {
            TyNodeChirho::ConChirho(TyConChirho::TypeChirho) => KindNodeChirho::TypeChirho,
            TyNodeChirho::ConChirho(TyConChirho::ConstraintChirho) => {
                KindNodeChirho::ConstraintChirho
            }
            TyNodeChirho::FunChirho(a_chirho, b_chirho) => KindNodeChirho::ArrowChirho(
                self.import_chirho(a_chirho, depth_chirho + 1)?,
                self.import_chirho(b_chirho, depth_chirho + 1)?,
            ),
            _ => {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::UnsupportedChirho,
                    "next requires a checked monomorphic kind",
                ));
            }
        };
        self.node_chirho(node_chirho)
    }
    fn depth_chirho(&mut self, depth_chirho: usize) -> Result<(), TcErrorChirho> {
        self.charge_chirho()?;
        if depth_chirho > 128 {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::ResourceChirho,
                "kind inference depth exhausted",
            ));
        }
        Ok(())
    }
    fn class_chirho(&mut self, name_chirho: NameIdChirho) -> Result<usize, TcErrorChirho> {
        if let Some(&id_chirho) = self.classes_chirho.get(&name_chirho) {
            return Ok(id_chirho);
        }
        let kind_chirho = self
            .checked_chirho
            .solver_chirho
            .arena_chirho
            .class_parameter_kind_chirho(name_chirho)?;
        self.import_chirho(kind_chirho, 0)
    }
    fn declared_kind_chirho(&mut self, name_chirho: NameIdChirho) -> Result<usize, TcErrorChirho> {
        if let Some(open_chirho) = self
            .checked_chirho
            .solver_chirho
            .arena_chirho
            .open_data_kinds_chirho(name_chirho)
            .map(<[bool]>::to_vec)
        {
            let mut result_chirho = self.node_chirho(KindNodeChirho::TypeChirho)?;
            for open_chirho in open_chirho.into_iter().rev() {
                let parameter_chirho = self.node_chirho(if open_chirho {
                    KindNodeChirho::VariableChirho(None)
                } else {
                    KindNodeChirho::TypeChirho
                })?;
                result_chirho =
                    self.node_chirho(KindNodeChirho::ArrowChirho(parameter_chirho, result_chirho))?;
            }
            return Ok(result_chirho);
        }
        let con_chirho = self
            .checked_chirho
            .solver_chirho
            .arena_chirho
            .con_chirho(TyConChirho::DeclaredChirho(name_chirho))?;
        let kind_chirho = self
            .checked_chirho
            .solver_chirho
            .arena_chirho
            .kind_chirho(con_chirho)?;
        self.import_chirho(kind_chirho, 0)
    }
    fn partial_alias_profile_chirho(
        &mut self,
        root_chirho: &SurfaceTypeChirho,
    ) -> Result<(), TcErrorChirho> {
        let mut work_chirho = vec![root_chirho];
        let mut applied_variable_chirho = false;
        let mut aliases_chirho = Vec::new();
        while let Some(ty_chirho) = work_chirho.pop() {
            self.charge_chirho()?;
            match ty_chirho {
                SurfaceTypeChirho::AppChirho(function_chirho, argument_chirho) => {
                    applied_variable_chirho |= matches!(
                        function_chirho.as_ref(),
                        SurfaceTypeChirho::VariableChirho(_)
                    );
                    work_chirho.extend([function_chirho.as_ref(), argument_chirho.as_ref()]);
                }
                SurfaceTypeChirho::FunChirho(a_chirho, b_chirho) => {
                    work_chirho.extend([a_chirho.as_ref(), b_chirho.as_ref()])
                }
                SurfaceTypeChirho::ListChirho(item_chirho) => work_chirho.push(item_chirho),
                SurfaceTypeChirho::TupleChirho(items_chirho) => work_chirho.extend(items_chirho),
                SurfaceTypeChirho::SynonymChirho(name_chirho) => {
                    if let Some(alias_chirho) = self
                        .checked_chirho
                        .synonyms_chirho
                        .get(name_chirho)
                        .filter(|alias_chirho| alias_chirho.higher_kinds_partial_chirho)
                    {
                        aliases_chirho.push(alias_chirho.scheme_chirho.clone());
                    }
                }
                _ => {}
            }
        }
        for alias_chirho in aliases_chirho {
            if applied_variable_chirho
                || self
                    .checked_chirho
                    .solver_chirho
                    .arena_chirho
                    .kind_is_partial_chirho(alias_chirho.body_chirho)?
            {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::UnsupportedChirho,
                    "higher-kind uses of a polymorphic-kind synonym need kind generalization",
                ));
            }
            // Type-only synonym headers cannot constrain a formal passed into an open data slot.
            let mut work_chirho = vec![alias_chirho.body_chirho];
            let mut seen_chirho = std::collections::HashSet::new();
            while let Some(ty_chirho) = work_chirho.pop() {
                self.charge_chirho()?;
                if !seen_chirho.insert(ty_chirho) {
                    continue;
                }
                let node_chirho = self
                    .checked_chirho
                    .solver_chirho
                    .arena_chirho
                    .node_chirho(ty_chirho)?
                    .clone();
                match node_chirho {
                    TyNodeChirho::AppChirho(_, _) => {
                        let mut head_chirho = ty_chirho;
                        let mut arguments_chirho = Vec::new();
                        while let TyNodeChirho::AppChirho(function_chirho, argument_chirho) = *self
                            .checked_chirho
                            .solver_chirho
                            .arena_chirho
                            .node_chirho(head_chirho)?
                        {
                            self.charge_chirho()?;
                            arguments_chirho.push(argument_chirho);
                            head_chirho = function_chirho;
                        }
                        arguments_chirho.reverse();
                        if let TyNodeChirho::ConChirho(TyConChirho::DeclaredChirho(name_chirho)) =
                            *self
                                .checked_chirho
                                .solver_chirho
                                .arena_chirho
                                .node_chirho(head_chirho)?
                        {
                            let open_chirho = self
                                .checked_chirho
                                .solver_chirho
                                .arena_chirho
                                .open_data_kinds_chirho(name_chirho)
                                .map(<[bool]>::to_vec)
                                .unwrap_or_default();
                            for (index_chirho, argument_chirho) in
                                arguments_chirho.iter().enumerate()
                            {
                                if open_chirho.get(index_chirho).copied().unwrap_or(false)
                                    && alias_chirho.quantified_chirho.contains(argument_chirho)
                                {
                                    return Err(TcErrorChirho::new_chirho(
                                        TcErrorKindChirho::UnsupportedChirho,
                                        "synonym kind depends on an open data parameter",
                                    ));
                                }
                            }
                        }
                        work_chirho.extend(arguments_chirho);
                    }
                    TyNodeChirho::FunChirho(a_chirho, b_chirho) => {
                        work_chirho.extend([a_chirho, b_chirho])
                    }
                    TyNodeChirho::TupleChirho(items_chirho) => work_chirho.extend(items_chirho),
                    _ => {}
                }
            }
        }
        Ok(())
    }
    fn surface_chirho(
        &mut self,
        ty_chirho: &SurfaceTypeChirho,
        depth_chirho: usize,
    ) -> Result<usize, TcErrorChirho> {
        self.depth_chirho(depth_chirho)?;
        match ty_chirho {
            SurfaceTypeChirho::VariableChirho(name_chirho) => {
                if let Some(&id_chirho) = self.variables_chirho.get(name_chirho) {
                    return Ok(id_chirho);
                }
                let variable_chirho = self
                    .checked_chirho
                    .surface_variable_chirho(*name_chirho, &HashMap::new())?
                    .ok_or_else(|| {
                        TcErrorChirho::new_chirho(
                            TcErrorKindChirho::UnsupportedChirho,
                            "kind inference has no lexical variable",
                        )
                    })?;
                let kind_chirho = self
                    .checked_chirho
                    .solver_chirho
                    .arena_chirho
                    .kind_chirho(variable_chirho)?;
                self.import_chirho(kind_chirho, 0)
            }
            SurfaceTypeChirho::DeclaredChirho(name_chirho)
            | SurfaceTypeChirho::SynonymChirho(name_chirho) => {
                if let Some(synonym_chirho) = self
                    .checked_chirho
                    .synonyms_chirho
                    .get(name_chirho)
                    .cloned()
                {
                    let mut kind_chirho = self
                        .checked_chirho
                        .solver_chirho
                        .arena_chirho
                        .kind_chirho(synonym_chirho.scheme_chirho.body_chirho)?;
                    for parameter_chirho in
                        synonym_chirho.scheme_chirho.quantified_chirho.iter().rev()
                    {
                        let parameter_kind_chirho = self
                            .checked_chirho
                            .solver_chirho
                            .arena_chirho
                            .kind_chirho(*parameter_chirho)?;
                        kind_chirho = self
                            .checked_chirho
                            .solver_chirho
                            .arena_chirho
                            .intern_chirho(TyNodeChirho::FunChirho(
                                parameter_kind_chirho,
                                kind_chirho,
                            ))?;
                    }
                    self.import_chirho(kind_chirho, 0)
                } else {
                    self.declared_kind_chirho(*name_chirho)
                }
            }
            SurfaceTypeChirho::IOChirho => {
                let a_chirho = self.node_chirho(KindNodeChirho::TypeChirho)?;
                self.node_chirho(KindNodeChirho::ArrowChirho(a_chirho, a_chirho))
            }
            SurfaceTypeChirho::AppChirho(fun_chirho, arg_chirho) => {
                let fun_chirho = self.surface_chirho(fun_chirho, depth_chirho + 1)?;
                let arg_chirho = self.surface_chirho(arg_chirho, depth_chirho + 1)?;
                let result_chirho = self.node_chirho(KindNodeChirho::VariableChirho(None))?;
                let arrow_chirho =
                    self.node_chirho(KindNodeChirho::ArrowChirho(arg_chirho, result_chirho))?;
                self.unify_chirho(fun_chirho, arrow_chirho)?;
                Ok(result_chirho)
            }
            SurfaceTypeChirho::FunChirho(a_chirho, b_chirho) => {
                let a_chirho = self.surface_chirho(a_chirho, depth_chirho + 1)?;
                let b_chirho = self.surface_chirho(b_chirho, depth_chirho + 1)?;
                let type_chirho = self.node_chirho(KindNodeChirho::TypeChirho)?;
                self.unify_chirho(a_chirho, type_chirho)?;
                self.unify_chirho(b_chirho, type_chirho)?;
                Ok(type_chirho)
            }
            SurfaceTypeChirho::ListChirho(item_chirho) => {
                let item_chirho = self.surface_chirho(item_chirho, depth_chirho + 1)?;
                let type_chirho = self.node_chirho(KindNodeChirho::TypeChirho)?;
                self.unify_chirho(item_chirho, type_chirho)?;
                Ok(type_chirho)
            }
            SurfaceTypeChirho::TupleChirho(items_chirho) => {
                let type_chirho = self.node_chirho(KindNodeChirho::TypeChirho)?;
                for item_chirho in items_chirho {
                    let item_chirho = self.surface_chirho(item_chirho, depth_chirho + 1)?;
                    self.unify_chirho(item_chirho, type_chirho)?;
                }
                Ok(type_chirho)
            }
            SurfaceTypeChirho::BoolChirho
            | SurfaceTypeChirho::OrderingChirho
            | SurfaceTypeChirho::CharChirho
            | SurfaceTypeChirho::IntegerChirho
            | SurfaceTypeChirho::IntChirho => self.node_chirho(KindNodeChirho::TypeChirho),
        }
    }
    fn predicates_chirho(
        &mut self,
        predicates_chirho: &[SurfacePredicateChirho],
    ) -> Result<(), TcErrorChirho> {
        for predicate_chirho in predicates_chirho {
            let expected_chirho = self.class_chirho(predicate_chirho.class_chirho)?;
            let actual_chirho = self.surface_chirho(
                &SurfaceTypeChirho::VariableChirho(predicate_chirho.argument_chirho),
                0,
            )?;
            self.unify_chirho(actual_chirho, expected_chirho)?;
        }
        Ok(())
    }
    fn finish_chirho(
        &mut self,
        id_chirho: usize,
        partial_chirho: bool,
        depth_chirho: usize,
    ) -> Result<TyChirho, TcErrorChirho> {
        self.depth_chirho(depth_chirho)?;
        let id_chirho = self.resolve_chirho(id_chirho)?;
        if let Some(&ty_chirho) = self.finished_chirho.get(&id_chirho) {
            return Ok(ty_chirho);
        }
        let ty_chirho = match self.nodes_chirho[id_chirho] {
            KindNodeChirho::VariableChirho(None) if partial_chirho => {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::UnsupportedChirho,
                    "next does not yet generalize polymorphic kinds",
                ));
            }
            KindNodeChirho::TypeChirho | KindNodeChirho::VariableChirho(None) => self
                .checked_chirho
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::TypeChirho)?,
            KindNodeChirho::ConstraintChirho => self
                .checked_chirho
                .solver_chirho
                .arena_chirho
                .con_chirho(TyConChirho::ConstraintChirho)?,
            KindNodeChirho::ArrowChirho(a_chirho, b_chirho) => {
                let a_chirho = self.finish_chirho(a_chirho, partial_chirho, depth_chirho + 1)?;
                let b_chirho = self.finish_chirho(b_chirho, partial_chirho, depth_chirho + 1)?;
                self.checked_chirho
                    .solver_chirho
                    .arena_chirho
                    .intern_chirho(TyNodeChirho::FunChirho(a_chirho, b_chirho))?
            }
            KindNodeChirho::VariableChirho(Some(_)) => unreachable!(),
        };
        self.finished_chirho.insert(id_chirho, ty_chirho);
        Ok(ty_chirho)
    }
}
impl CheckedModuleChirho {
    pub(super) fn infer_class_kinds_chirho(
        &mut self,
        declarations_chirho: &[ClassDeclChirho],
    ) -> Result<(), TcErrorChirho> {
        // Superclass dependencies are the only class-to-class kind edges in this profile.
        // Finish each producer before its consumers: H98 defaults may not be refined later.
        // Cyclic superclasses already require an unmodeled extension and keep their refusal.
        let mut indices_chirho = HashMap::new();
        let mut pending_chirho = vec![0usize; declarations_chirho.len()];
        let mut dependents_chirho = vec![Vec::new(); declarations_chirho.len()];
        for (index_chirho, declaration_chirho) in declarations_chirho.iter().enumerate() {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            indices_chirho.insert(declaration_chirho.name_chirho, index_chirho);
        }
        for (index_chirho, declaration_chirho) in declarations_chirho.iter().enumerate() {
            for superclass_chirho in &declaration_chirho.superclasses_chirho {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                if let Some(&producer_chirho) = indices_chirho.get(&superclass_chirho.class_chirho)
                {
                    pending_chirho[index_chirho] += 1;
                    dependents_chirho[producer_chirho].push(index_chirho);
                }
            }
        }
        let mut ready_chirho: std::collections::VecDeque<_> = pending_chirho
            .iter()
            .enumerate()
            .filter_map(|(index_chirho, &count_chirho)| (count_chirho == 0).then_some(index_chirho))
            .collect();
        let mut finished_chirho = 0;
        while let Some(index_chirho) = ready_chirho.pop_front() {
            let declaration_chirho = &declarations_chirho[index_chirho];
            let mut inference_chirho = KindInferenceChirho::new_chirho(self);
            let id_chirho =
                inference_chirho.variable_chirho(declaration_chirho.parameter_chirho)?;
            inference_chirho
                .classes_chirho
                .insert(declaration_chirho.name_chirho, id_chirho);
            inference_chirho.predicates_chirho(&declaration_chirho.superclasses_chirho)?;
            for method_chirho in &declaration_chirho.methods_chirho {
                for &binder_chirho in &method_chirho.binders_chirho {
                    inference_chirho.variable_chirho(binder_chirho)?;
                }
                inference_chirho.partial_alias_profile_chirho(&method_chirho.ty_chirho)?;
                let actual_chirho = inference_chirho.surface_chirho(&method_chirho.ty_chirho, 0)?;
                let type_chirho = inference_chirho.node_chirho(KindNodeChirho::TypeChirho)?;
                inference_chirho.unify_chirho(actual_chirho, type_chirho)?;
            }
            let kind_chirho = inference_chirho.finish_chirho(
                id_chirho,
                declaration_chirho.higher_kinds_partial_chirho,
                0,
            )?;
            self.solver_chirho
                .arena_chirho
                .set_class_kind_chirho(declaration_chirho.name_chirho, kind_chirho)?;
            finished_chirho += 1;
            for &consumer_chirho in &dependents_chirho[index_chirho] {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                pending_chirho[consumer_chirho] -= 1;
                if pending_chirho[consumer_chirho] == 0 {
                    ready_chirho.push_back(consumer_chirho);
                }
            }
        }
        if finished_chirho != declarations_chirho.len() {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::TypeMismatchChirho,
                "cyclic superclass declarations",
            ));
        }
        Ok(())
    }
    pub(super) fn signature_kinds_chirho(
        &mut self,
        signature_chirho: &SurfaceSignatureChirho,
        captured_chirho: &HashMap<NameIdChirho, TyChirho>,
    ) -> Result<HashMap<NameIdChirho, TyChirho>, TcErrorChirho> {
        let partial_chirho = self.module_chirho.ghc2021_chirho;
        let mut inference_chirho = KindInferenceChirho::new_chirho(self);
        for (&name_chirho, &ty_chirho) in captured_chirho {
            let kind_chirho = inference_chirho
                .checked_chirho
                .solver_chirho
                .arena_chirho
                .kind_chirho(ty_chirho)?;
            let kind_chirho = inference_chirho.import_chirho(kind_chirho, 0)?;
            inference_chirho
                .variables_chirho
                .insert(name_chirho, kind_chirho);
        }
        for &binder_chirho in &signature_chirho.binders_chirho {
            inference_chirho.variable_chirho(binder_chirho)?;
        }
        inference_chirho.partial_alias_profile_chirho(&signature_chirho.body_chirho)?;
        inference_chirho.predicates_chirho(&signature_chirho.context_chirho)?;
        let actual_chirho = inference_chirho.surface_chirho(&signature_chirho.body_chirho, 0)?;
        let type_chirho = inference_chirho.node_chirho(KindNodeChirho::TypeChirho)?;
        inference_chirho.unify_chirho(actual_chirho, type_chirho)?;
        let mut kinds_chirho = HashMap::new();
        for &binder_chirho in &signature_chirho.binders_chirho {
            let id_chirho = inference_chirho.variables_chirho[&binder_chirho];
            let kind_chirho = inference_chirho.finish_chirho(id_chirho, partial_chirho, 0)?;
            kinds_chirho.insert(binder_chirho, kind_chirho);
        }
        Ok(kinds_chirho)
    }
}
