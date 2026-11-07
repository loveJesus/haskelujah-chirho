// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The newtype deriving strategy's dictionaries (GND, row 694).
//!
//! A newtype instance takes its representation type's methods only when the newtype's own
//! declarations route that class through the newtype strategy, as the deriving pass records
//! in `NewtypeInfoChirho`, and typing proved every method of the class coercible
//! (`GndCoercionProofChirho`). A written instance with a context, or one that merely has a
//! newtype head, keeps its own methods. Every method of a GND instance is the
//! representation's, its class defaults included, as GHC coerces each one (G3, D2).
//!
//! The representation's method is its real row; else the method its dictionary holds; else,
//! for a list representation whose class instance is conditional, the method of the
//! conditional dictionary the existing list machinery builds once the element's context
//! dictionaries exist, which may be another GND newtype's (`[Age]` needs `Age`'s). A worklist
//! builds each instance as soon as what it needs exists, so neither pass order nor table order
//! decides it. A generated missing-method stub is never a representation's method, a row the
//! module wrote beside a GND derivation is never replaced, and an instance whose methods
//! cannot be built is reported as unsupported, never aliased.
//!
//! A GND instance is `C rep => C N`, so the module rows the desugarer filled for it take that
//! context's dictionaries first, as every caller passes them (`context_leading_rows_chirho`);
//! a replaced default row keeps those leading parameters and ignores them. The dictionary's
//! fields are the representation's methods themselves.
//!
//! A GND instance over a parametric newtype (`Show a => Show (BoxChirho a)`, P1, P2) has no
//! one dictionary: it is a factory, a lambda over the instance's context dictionaries. Its
//! dictionary reads every method from the representation's, the context dictionary of the same
//! class, and holds the newtype's own dictionary for each superclass, never the
//! representation's (#31851). It is registered as that conditional instance's builder with the
//! head's formal binders, and a use applies it at the arguments the checker proved.
//!
//! A multi-parameter GND instance derives its newtype at the last head position (row 694,
//! #31909): a methodless class's factory whose superclass fields are the newtype's own
//! dictionaries, registered by its whole head. Any other shape refuses the module by name.
//!
//! Core erases these newtypes' constructors (`simplify_chirho/newtype_erasure_chirho.rs`),
//! so a representation's method receives the field itself on every backend.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use haskelujah_typing_chirho::GndCoercionProofChirho;
use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, InstDeclChirho, PredChirho};
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::super::{ConditionalDictChirho, DictLayoutChirho, DictPassCtxChirho};
use super::generic_heads_chirho::{head_key_chirho, superclass_at_head_chirho};
use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, InlineAnnotationChirho,
};

/// What the dictionary pass knows about one newtype declaration.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NewtypeInfoChirho {
    /// The newtype's constructor.
    pub constructor_chirho: String,
    /// The key of the type its constructor wraps, such as `Int`.
    pub underlying_key_chirho: String,
    /// The classes the newtype's own declarations route through the newtype strategy:
    /// only these take the representation's dictionary.
    pub newtype_derived_classes_chirho: Vec<String>,
    /// Typing's proof, per class, that the class's methods may be the representation's.
    pub coercion_proofs_chirho: Vec<GndCoercionProofChirho>,
    /// The newtype's rows the desugarer filled from class defaults: the only rows with a body
    /// that a GND method replaces.
    pub default_method_rows_chirho: Vec<String>,
}

/// One GND instance the pass builds.
#[derive(Debug, Clone)]
struct GndPairChirho {
    class_chirho: String,
    type_key_chirho: String,
    representation_key_chirho: String,
    layout_chirho: DictLayoutChirho,
}

impl GndPairChirho {
    /// How diagnostics name the instance.
    fn label_chirho(&self) -> String {
        format!(
            "deriving {} for {} through the newtype strategy",
            self.class_chirho, self.type_key_chirho
        )
    }
}

impl DictPassCtxChirho {
    /// Whether the newtype keyed `type_key_chirho` routes `class_name_chirho` through the
    /// newtype strategy, so its dictionary is the representation's.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(in crate::dict_chirho) fn takes_representation_dictionary_chirho(
        &self,
        class_name_chirho: &str,
        type_key_chirho: &str,
    ) -> bool {
        self.newtype_info_chirho
            .get(type_key_chirho)
            .is_some_and(|info_chirho| {
                info_chirho
                    .newtype_derived_classes_chirho
                    .iter()
                    .any(|derived_chirho| derived_chirho == class_name_chirho)
            })
    }

    /// The key whose rows a newtype's rows alias: its representation's, or, while that is
    /// itself a newtype taking the class through the newtype strategy, the next
    /// representation's (`newtype Older = Older Age` with GND `Age` reaches `Int`). Every
    /// such newtype is erased, so the value is the last representation's. Each step names a
    /// distinct newtype; a longer chain is a cycle, which the checker refuses, and stops.
    fn representation_key_chirho(&self, class_name_chirho: &str, key_chirho: &str) -> String {
        let mut representation_chirho = key_chirho.to_string();
        for _ in 0..self.newtype_info_chirho.len() {
            if !self
                .takes_representation_dictionary_chirho(class_name_chirho, &representation_chirho)
            {
                break;
            }
            match self.newtype_info_chirho.get(&representation_chirho) {
                Some(info_chirho) => {
                    representation_chirho = info_chirho.underlying_key_chirho.clone();
                }
                None => break,
            }
        }
        representation_chirho
    }

    /// Generate dictionary bindings for GeneralizedNewtypeDeriving instances.
    ///
    /// Each GND instance (`Num Int => Num Age`) gets, for each class method, a
    /// `$prim_Class_method_NewtypeKey` row holding the representation's method, then its
    /// dictionary. The deriving pass writes a GND instance with no method, so a newtype row
    /// that has a body is the class default the desugarer filled in, and it becomes the
    /// representation's method in place. Instances wait for each other until none can
    /// progress; what is left, and what no row or dictionary serves, is unsupported.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub fn generate_gnd_dicts_chirho(&mut self, class_env_chirho: &ClassEnvChirho) {
        self.generate_parametric_gnd_factories_chirho(class_env_chirho);
        let mut pending_chirho = self.gnd_pairs_chirho(class_env_chirho);
        while !pending_chirho.is_empty() {
            let before_chirho = pending_chirho.len();
            let mut waiting_chirho = Vec::new();
            for pair_chirho in pending_chirho {
                match self.gnd_methods_chirho(class_env_chirho, &pair_chirho) {
                    Ok(Some(methods_chirho)) => {
                        self.build_gnd_dict_chirho(&pair_chirho, methods_chirho)
                    }
                    Ok(None) => waiting_chirho.push(pair_chirho),
                    Err(reason_chirho) => self.unsupported_dictionaries_chirho.push(reason_chirho),
                }
            }
            if waiting_chirho.len() == before_chirho {
                for pair_chirho in &waiting_chirho {
                    self.unsupported_dictionaries_chirho.push(format!(
                        "{}: the `{}` dictionary for its representation `{}` needs a context dictionary no instance supplies",
                        pair_chirho.label_chirho(),
                        pair_chirho.class_chirho,
                        pair_chirho.representation_key_chirho
                    ));
                }
                break;
            }
            pending_chirho = waiting_chirho;
        }
    }

    /// Each GND instance that applies its newtype to type variables (its head is no newtype's
    /// name, so no pair below builds its dictionary) gets a factory, registered as its
    /// conditional instance's builder, whenever one can be built. Every such (class, newtype)
    /// is recorded, so a use the factory cannot serve is refused by name.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    fn generate_parametric_gnd_factories_chirho(&mut self, class_env_chirho: &ClassEnvChirho) {
        let mut parametric_chirho: Vec<(String, String)> = Vec::new();
        let mut multi_heads_chirho: Vec<(String, String, std::sync::Arc<InstDeclChirho>)> =
            Vec::new();
        for (class_name_chirho, instances_chirho) in
            class_env_chirho.instances_chirho().iter_chirho()
        {
            for inst_chirho in instances_chirho.iter_chirho() {
                if let Some(newtype_chirho) =
                    self.parametric_gnd_newtype_chirho(class_name_chirho, inst_chirho)
                {
                    parametric_chirho.push((class_name_chirho.clone(), newtype_chirho.to_string()));
                }
                if let Some(newtype_chirho) =
                    self.multi_head_gnd_newtype_chirho(class_name_chirho, inst_chirho)
                {
                    multi_heads_chirho.push((
                        class_name_chirho.clone(),
                        newtype_chirho.to_string(),
                        inst_chirho.clone(),
                    ));
                }
            }
        }
        parametric_chirho.sort();
        parametric_chirho.dedup();
        for (class_name_chirho, newtype_chirho) in &parametric_chirho {
            self.parametric_gnd_factory_chirho(
                class_env_chirho,
                class_name_chirho,
                newtype_chirho,
                &mut Vec::new(),
            );
        }
        self.gnd_parametric_chirho = parametric_chirho.into_iter().collect();
        // Each multi-parameter GND instance's factory, in (class, head) order; one this pass
        // cannot build refuses the module by name (row 694, #31909).
        multi_heads_chirho.sort_by_cached_key(|(class_name_chirho, _, inst_chirho)| {
            (
                class_name_chirho.clone(),
                head_key_chirho(
                    &inst_chirho.head_ty_chirho,
                    &inst_chirho.extra_head_tys_chirho,
                ),
            )
        });
        for (class_name_chirho, newtype_chirho, inst_chirho) in &multi_heads_chirho {
            match self.multi_head_gnd_factory_chirho(class_env_chirho, newtype_chirho, inst_chirho)
            {
                Ok(builder_chirho) => self.register_multi_head_chirho(inst_chirho, builder_chirho),
                Err(reason_chirho) => self.unsupported_dictionaries_chirho.push(format!(
                    "deriving {class_name_chirho} for {newtype_chirho} through the newtype strategy: {reason_chirho}"
                )),
            }
        }
    }

    /// The newtype a multi-parameter GND instance derives last (`TChirho` in `AskChirho a
    /// (ReaderChirho a) => AskChirho a (TChirho a)`, row 694, #31909): the newtype routes the
    /// class through the newtype strategy, and the context holds the representation's dictionary.
    fn multi_head_gnd_newtype_chirho<'a>(
        &self,
        class_name_chirho: &str,
        inst_chirho: &'a InstDeclChirho,
    ) -> Option<&'a str> {
        let TyChirho::ConChirho(newtype_chirho) =
            head_application_chirho(inst_chirho.extra_head_tys_chirho.last()?).0
        else {
            return None;
        };
        (self.takes_representation_dictionary_chirho(class_name_chirho, newtype_chirho)
            && inst_chirho
                .context_chirho
                .iter()
                .any(|predicate_chirho| predicate_chirho.class_name_chirho == class_name_chirho))
        .then_some(newtype_chirho.as_str())
    }

    /// The factory of a multi-parameter GND instance `C x.. (N a..)` (row 694, #31909): a lambda
    /// over its context's dictionaries returning `$Dict_C`. Its class has no method; each
    /// superclass must constrain the newtype's position, the last, and is the newtype's own
    /// dictionary there (`parametric_superclass_chirho`). Other shapes are refused by name.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    fn multi_head_gnd_factory_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        newtype_chirho: &str,
        inst_chirho: &InstDeclChirho,
    ) -> Result<CoreIdChirho, String> {
        let class_name_chirho = inst_chirho.class_name_chirho.as_str();
        let layout_chirho = self
            .layouts_chirho
            .get(class_name_chirho)
            .ok_or_else(|| format!("the class `{class_name_chirho}` has no dictionary layout"))?
            .clone();
        if !layout_chirho.method_slots_chirho.is_empty() {
            return Err(
                "its class has several parameters and methods, whose coercion this pass does not build"
                    .to_string(),
            );
        }
        let newtype_head_chirho = inst_chirho
            .extra_head_tys_chirho
            .last()
            .ok_or_else(|| "its class has one parameter".to_string())?;
        let representation_index_chirho = inst_chirho
            .context_chirho
            .iter()
            .position(|predicate_chirho| predicate_chirho.class_name_chirho == class_name_chirho)
            .ok_or_else(|| {
                format!(
                    "its context names no `{class_name_chirho}` dictionary of the representation"
                )
            })?;
        let parameters_chirho = self.context_parameters_chirho(&inst_chirho.context_chirho);
        let representation_chirho = parameters_chirho[representation_index_chirho].id_chirho;
        let mut fields_chirho = Vec::new();
        for index_chirho in 0..layout_chirho.super_slots_chirho.len() {
            let superclass_chirho = superclass_at_head_chirho(
                class_env_chirho,
                inst_chirho,
                &layout_chirho,
                index_chirho,
            )?;
            if !superclass_chirho.extra_tys_chirho.is_empty()
                || superclass_chirho.ty_chirho != *newtype_head_chirho
            {
                return Err(format!(
                    "its superclass `{superclass_chirho}` constrains another position than the newtype's, whose dictionary this pass does not build"
                ));
            }
            fields_chirho.push(self.parametric_superclass_chirho(
                class_env_chirho,
                class_name_chirho,
                &superclass_chirho.class_name_chirho,
                newtype_chirho,
                representation_chirho,
                &mut Vec::new(),
            )?);
        }
        Ok(self.bind_factory_chirho(
            &format!(
                "$fFactory{class_name_chirho}{}",
                head_key_chirho(
                    &inst_chirho.head_ty_chirho,
                    &inst_chirho.extra_head_tys_chirho
                )
            ),
            class_name_chirho,
            &parameters_chirho,
            fields_chirho,
        ))
    }

    /// One dictionary parameter per context predicate, in order (`$dShow` for `Show a`).
    fn context_parameters_chirho(&mut self, context_chirho: &[PredChirho]) -> Vec<BinderChirho> {
        context_chirho
            .iter()
            .map(|predicate_chirho| {
                self.fresh_binder_chirho(
                    &format!("$d{}", predicate_chirho.class_name_chirho),
                    TyChirho::ConChirho(format!("$Dict_{}", predicate_chirho.class_name_chirho)),
                )
            })
            .collect()
    }

    /// Binds the factory `name_chirho`: `$Dict_C fields` under a lambda over each of
    /// `parameters_chirho`, in order. Returns the factory's id.
    fn bind_factory_chirho(
        &mut self,
        name_chirho: &str,
        class_name_chirho: &str,
        parameters_chirho: &[BinderChirho],
        fields_chirho: Vec<CoreExprChirho>,
    ) -> CoreIdChirho {
        let mut factory_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: format!("$Dict_{class_name_chirho}"),
            args_chirho: fields_chirho,
        };
        let mut factory_ty_chirho = TyChirho::ConChirho(format!("$Dict_{class_name_chirho}"));
        for parameter_chirho in parameters_chirho.iter().rev() {
            factory_ty_chirho =
                TyChirho::fun_chirho(parameter_chirho.ty_chirho.clone(), factory_ty_chirho);
            factory_chirho = CoreExprChirho::LamChirho {
                binder_chirho: parameter_chirho.clone(),
                body_chirho: Box::new(factory_chirho),
            };
        }
        let builder_chirho = self.fresh_binder_chirho(name_chirho, factory_ty_chirho);
        let builder_id_chirho = builder_chirho.id_chirho;
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: builder_chirho,
            rhs_chirho: factory_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        builder_id_chirho
    }

    /// The newtype a GND instance applies to type variables, when `inst_chirho` is one. A
    /// multi-parameter instance never is: its newtype stands last
    /// (`multi_head_gnd_newtype_chirho`).
    fn parametric_gnd_newtype_chirho<'a>(
        &self,
        class_name_chirho: &str,
        inst_chirho: &'a InstDeclChirho,
    ) -> Option<&'a str> {
        if !inst_chirho.extra_head_tys_chirho.is_empty() {
            return None;
        }
        let (head_chirho, arguments_chirho) = head_application_chirho(&inst_chirho.head_ty_chirho);
        match head_chirho {
            TyChirho::ConChirho(newtype_chirho)
                if !arguments_chirho.is_empty()
                    && self.takes_representation_dictionary_chirho(
                        class_name_chirho,
                        newtype_chirho,
                    ) =>
            {
                Some(newtype_chirho)
            }
            _ => None,
        }
    }

    /// The builder of the parametric GND instance `class (newtype a..)`, built once and
    /// registered in `conditional_dicts_chirho`. A factory that cannot be built is recorded in
    /// `gnd_factory_gaps_chirho` with its reason, so a use is refused by name.
    /// `building_chirho` holds the instances whose factories wait on this one.
    fn parametric_gnd_factory_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        newtype_chirho: &str,
        building_chirho: &mut Vec<(String, String)>,
    ) -> Option<CoreIdChirho> {
        let key_chirho = (class_name_chirho.to_string(), newtype_chirho.to_string());
        if let Some(factory_chirho) = self.conditional_dicts_chirho.get(&key_chirho) {
            return Some(factory_chirho.builder_chirho);
        }
        if self.gnd_factory_gaps_chirho.contains_key(&key_chirho)
            || building_chirho.contains(&key_chirho)
        {
            return None;
        }
        let inst_chirho = class_env_chirho
            .instances_chirho()
            .get_chirho(class_name_chirho)?
            .iter_chirho()
            .find(|inst_chirho| {
                self.parametric_gnd_newtype_chirho(class_name_chirho, inst_chirho)
                    == Some(newtype_chirho)
            })?
            .clone();
        building_chirho.push(key_chirho.clone());
        let built_chirho = self.gnd_factory_chirho(
            class_env_chirho,
            class_name_chirho,
            newtype_chirho,
            &inst_chirho,
            building_chirho,
        );
        building_chirho.pop();
        match built_chirho {
            Ok(factory_chirho) => {
                let builder_chirho = factory_chirho.builder_chirho;
                self.conditional_dicts_chirho
                    .insert(key_chirho, factory_chirho);
                Some(builder_chirho)
            }
            Err(reason_chirho) => {
                self.gnd_factory_gaps_chirho
                    .insert(key_chirho, reason_chirho);
                None
            }
        }
    }

    /// One parametric GND instance's factory: a lambda over its context's dictionaries, in
    /// context order, returning `$Dict_C`. Every method field is read from the context
    /// dictionary of class `C` itself, the representation's. Every superclass field is the
    /// newtype's own `S (N a)` dictionary (`parametric_superclass_chirho`), never the
    /// representation's, which answers for another type. The error says why it cannot be built.
    fn gnd_factory_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        newtype_chirho: &str,
        inst_chirho: &InstDeclChirho,
        building_chirho: &mut Vec<(String, String)>,
    ) -> Result<ConditionalDictChirho, String> {
        let layout_chirho = self
            .layouts_chirho
            .get(class_name_chirho)
            .ok_or_else(|| format!("the class `{class_name_chirho}` has no dictionary layout"))?
            .clone();
        if !inst_chirho.extra_head_tys_chirho.is_empty()
            || inst_chirho
                .context_chirho
                .iter()
                .any(|predicate_chirho| !predicate_chirho.extra_tys_chirho.is_empty())
        {
            return Err("its class has several parameters".to_string());
        }
        let (_, arguments_chirho) = head_application_chirho(&inst_chirho.head_ty_chirho);
        let mut formals_chirho: Vec<TyVarChirho> = Vec::new();
        for argument_chirho in arguments_chirho {
            let TyChirho::VarChirho(formal_chirho) = argument_chirho else {
                return Err("its head applies the newtype to more than variables".to_string());
            };
            if formals_chirho.contains(formal_chirho) {
                return Err("its head repeats a variable".to_string());
            }
            formals_chirho.push(*formal_chirho);
        }
        let representation_index_chirho = inst_chirho
            .context_chirho
            .iter()
            .position(|predicate_chirho| predicate_chirho.class_name_chirho == class_name_chirho)
            .ok_or_else(|| {
                format!(
                    "its context names no `{class_name_chirho}` dictionary of the representation"
                )
            })?;
        let parameters_chirho = self.context_parameters_chirho(&inst_chirho.context_chirho);
        let representation_chirho = parameters_chirho[representation_index_chirho].id_chirho;
        let mut fields_chirho = Vec::new();
        for slot_chirho in &layout_chirho.super_slots_chirho {
            // This route finds a superclass by its spelling's instances, so a displaced compiler
            // superclass is refused by name here, never served by the program's own class (row
            // 692, C10, gpt #32919).
            if let Some(identity_chirho) =
                self.displaced_identity_chirho(&slot_chirho.class_proof_chirho)
            {
                return Err(format!(
                    "its superclass is the compiler's `{}`, which the program's own class displaces, and this route reaches a superclass only by its spelling",
                    identity_chirho.legacy_name_chirho()
                ));
            }
            fields_chirho.push(self.parametric_superclass_chirho(
                class_env_chirho,
                class_name_chirho,
                &slot_chirho.namespace_chirho,
                newtype_chirho,
                representation_chirho,
                building_chirho,
            )?);
        }
        for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
            let selector_chirho = match self.method_selectors_chirho.get(method_name_chirho) {
                Some((selector_class_chirho, selector_chirho))
                    if selector_class_chirho == class_name_chirho =>
                {
                    *selector_chirho
                }
                _ => {
                    return Err(format!(
                        "the method `{method_name_chirho}` has no selector of `{class_name_chirho}`"
                    ));
                }
            };
            fields_chirho.push(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(selector_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(representation_chirho)),
            });
        }
        let builder_id_chirho = self.bind_factory_chirho(
            &format!("$fFactory{class_name_chirho}{newtype_chirho}"),
            class_name_chirho,
            &parameters_chirho,
            fields_chirho,
        );
        Ok(ConditionalDictChirho {
            builder_chirho: builder_id_chirho,
            formals_chirho,
            context_chirho: inst_chirho.context_chirho.clone(),
            instance_chirho: None,
        })
    }

    /// The `S (N a)` dictionary the parametric GND instance `C (N a)` holds for its superclass
    /// `S` (#31851): the newtype's own `S` instance, found by its record. A written instance
    /// without a context is the dictionary registered under the newtype; one the newtype also
    /// takes through the newtype strategy is `S`'s factory applied to the representation's `S`
    /// dictionary, read from the representation's `C` dictionary. A written instance with a
    /// context, several instances or none refuse the factory by name.
    fn parametric_superclass_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        super_name_chirho: &str,
        newtype_chirho: &str,
        representation_chirho: CoreIdChirho,
        building_chirho: &mut Vec<(String, String)>,
    ) -> Result<CoreExprChirho, String> {
        let own_chirho: Vec<&std::sync::Arc<InstDeclChirho>> = class_env_chirho
            .instances_chirho()
            .get_chirho(super_name_chirho)
            .into_iter()
            .flatten()
            .filter(|inst_chirho| {
                matches!(
                    head_application_chirho(&inst_chirho.head_ty_chirho).0,
                    TyChirho::ConChirho(head_chirho) if head_chirho == newtype_chirho
                )
            })
            .collect();
        match own_chirho.as_slice() {
            [] => Err(format!(
                "the newtype has no `{super_name_chirho}` instance for the superclass of `{class_name_chirho}`"
            )),
            [_] if self.takes_representation_dictionary_chirho(super_name_chirho, newtype_chirho) => {
                let builder_chirho = self
                    .parametric_gnd_factory_chirho(
                        class_env_chirho,
                        super_name_chirho,
                        newtype_chirho,
                        building_chirho,
                    )
                    .ok_or_else(|| {
                        format!("its superclass `{super_name_chirho}` has no factory this pass can build")
                    })?;
                let single_context_chirho = self
                    .conditional_dicts_chirho
                    .get(&(super_name_chirho.to_string(), newtype_chirho.to_string()))
                    .is_some_and(|factory_chirho| factory_chirho.context_chirho.len() == 1);
                let selector_chirho = self
                    .super_selectors_chirho
                    .get(&(class_name_chirho.to_string(), super_name_chirho.to_string()))
                    .copied()
                    .filter(|_| single_context_chirho)
                    .ok_or_else(|| {
                        format!("the representation's `{super_name_chirho}` dictionary cannot be read from its `{class_name_chirho}` dictionary")
                    })?;
                Ok(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(builder_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(selector_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(representation_chirho)),
                    }),
                })
            }
            [written_chirho] if written_chirho.context_chirho.is_empty() => self
                .instance_dicts_chirho
                .get(&(super_name_chirho.to_string(), newtype_chirho.to_string()))
                .copied()
                .map(CoreExprChirho::VarChirho)
                .ok_or_else(|| {
                    format!("its written `{super_name_chirho}` instance has no dictionary under the newtype")
                }),
            [_] => Err(format!(
                "its written `{super_name_chirho}` instance has a context, whose dictionaries this pass does not build"
            )),
            _ => Err(format!("the newtype has several `{super_name_chirho}` instances")),
        }
    }

    /// Every GND instance the class environment holds, in (class, newtype) order.
    fn gnd_pairs_chirho(&self, class_env_chirho: &ClassEnvChirho) -> Vec<GndPairChirho> {
        let mut pairs_chirho = Vec::new();
        for (class_name_chirho, instances_chirho) in
            class_env_chirho.instances_chirho().iter_chirho()
        {
            let Some(layout_chirho) = self.layouts_chirho.get(class_name_chirho) else {
                continue;
            };
            for inst_chirho in instances_chirho.iter_chirho() {
                // A multi-parameter GND instance is a factory by its whole head, never a pair
                // keyed by its first argument (row 694, #31909).
                if !inst_chirho.extra_head_tys_chirho.is_empty() {
                    continue;
                }
                let type_key_chirho = format!("{}", inst_chirho.head_ty_chirho);
                // The newtype's own declarations are the only proof of the route: an
                // instance's context and head never make it one (row 694).
                if !self.takes_representation_dictionary_chirho(class_name_chirho, &type_key_chirho)
                {
                    continue;
                }
                if let Some(info_chirho) = self.newtype_info_chirho.get(&type_key_chirho) {
                    pairs_chirho.push(GndPairChirho {
                        class_chirho: class_name_chirho.clone(),
                        representation_key_chirho: self.representation_key_chirho(
                            class_name_chirho,
                            &info_chirho.underlying_key_chirho,
                        ),
                        type_key_chirho,
                        layout_chirho: layout_chirho.clone(),
                    });
                }
            }
        }
        pairs_chirho.sort_by(|left_chirho, right_chirho| {
            (&left_chirho.class_chirho, &left_chirho.type_key_chirho)
                .cmp(&(&right_chirho.class_chirho, &right_chirho.type_key_chirho))
        });
        pairs_chirho.dedup_by(|left_chirho, right_chirho| {
            left_chirho.class_chirho == right_chirho.class_chirho
                && left_chirho.type_key_chirho == right_chirho.type_key_chirho
        });
        pairs_chirho
    }

    /// The representation's method for each of the class's slots, in layout order; `None`
    /// while one waits for a context dictionary another instance has still to build.
    fn gnd_methods_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        pair_chirho: &GndPairChirho,
    ) -> Result<Option<Vec<CoreExprChirho>>, String> {
        let info_chirho = self
            .newtype_info_chirho
            .get(&pair_chirho.type_key_chirho)
            .cloned()
            .unwrap_or_default();
        let Some(proof_chirho) = info_chirho
            .coercion_proofs_chirho
            .iter()
            .find(|proof_chirho| proof_chirho.class_chirho() == pair_chirho.class_chirho)
        else {
            return Err(format!(
                "{}: typing proved no coercion of the class's methods",
                pair_chirho.label_chirho()
            ));
        };
        let mut methods_chirho = Vec::new();
        for (method_chirho, _) in &pair_chirho.layout_chirho.method_slots_chirho {
            if !proof_chirho.proves_method_chirho(method_chirho) {
                return Err(format!(
                    "{}: typing proved no coercion of the method `{method_chirho}`",
                    pair_chirho.label_chirho()
                ));
            }
            let newtype_row_chirho = format!(
                "$prim_{}_{method_chirho}_{}",
                pair_chirho.class_chirho, pair_chirho.type_key_chirho
            );
            if self
                .body_backed_names_chirho
                .contains_key(&newtype_row_chirho)
                && !info_chirho
                    .default_method_rows_chirho
                    .contains(&newtype_row_chirho)
            {
                return Err(format!(
                    "{}: the module also writes the method `{method_chirho}` for it",
                    pair_chirho.label_chirho()
                ));
            }
            match self.representation_method_chirho(class_env_chirho, pair_chirho, method_chirho)? {
                Some(representation_chirho) => methods_chirho.push(representation_chirho),
                None => return Ok(None),
            }
        }
        Ok(Some(methods_chirho))
    }

    /// The representation's `method_chirho`: its real row, or its dictionary's field, building
    /// a conditional list dictionary through the existing list machinery when the
    /// representation is a list. `None` while that dictionary waits for its element's.
    fn representation_method_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        pair_chirho: &GndPairChirho,
        method_chirho: &str,
    ) -> Result<Option<CoreExprChirho>, String> {
        let representation_chirho = &pair_chirho.representation_key_chirho;
        let row_name_chirho = format!(
            "$prim_{}_{method_chirho}_{representation_chirho}",
            pair_chirho.class_chirho
        );
        if let Some(row_chirho) = self.real_row_id_chirho(&row_name_chirho) {
            // A row whose instance has a context is reached with that context's dictionaries,
            // as `proof_row_chirho` gives them, once they exist.
            if !self.context_leading_rows_chirho.contains_key(&row_chirho) {
                return Ok(Some(CoreExprChirho::VarChirho(row_chirho)));
            }
            return Ok(self.proof_row_chirho(&row_name_chirho));
        }
        let dictionary_key_chirho = (
            pair_chirho.class_chirho.clone(),
            representation_chirho.clone(),
        );
        let dictionary_chirho = match self
            .instance_dicts_chirho
            .get(&dictionary_key_chirho)
            .copied()
        {
            Some(dictionary_chirho) => dictionary_chirho,
            None => match self.conditional_list_dictionary_chirho(class_env_chirho, pair_chirho) {
                Some(Some(dictionary_chirho)) => dictionary_chirho,
                Some(None) => return Ok(None),
                None => {
                    return Err(format!(
                        "{}: its representation `{representation_chirho}` has no `{method_chirho}` row and no `{}` dictionary this pass can build",
                        pair_chirho.label_chirho(),
                        pair_chirho.class_chirho
                    ));
                }
            },
        };
        match self.dictionary_method_chirho(
            dictionary_chirho,
            &pair_chirho.layout_chirho,
            method_chirho,
        ) {
            Some(CoreExprChirho::VarChirho(field_chirho))
                if self.missing_method_ids_chirho.contains(&field_chirho) =>
            {
                Err(format!(
                    "{}: the `{}` dictionary of its representation `{representation_chirho}` has no `{method_chirho}` method",
                    pair_chirho.label_chirho(),
                    pair_chirho.class_chirho
                ))
            }
            Some(field_chirho) => Ok(Some(field_chirho)),
            None => Err(format!(
                "{}: the `{}` dictionary of its representation `{representation_chirho}` holds no `{method_chirho}` field",
                pair_chirho.label_chirho(),
                pair_chirho.class_chirho
            )),
        }
    }

    /// A row with a body that is no generated missing-method stub.
    fn real_row_id_chirho(&self, row_name_chirho: &str) -> Option<CoreIdChirho> {
        self.lookup_body_backed_name_id_chirho(row_name_chirho)
            .filter(|row_chirho| !self.missing_method_ids_chirho.contains(row_chirho))
    }

    /// The `[E]` dictionary of a list representation whose class instance is conditional
    /// (`C a => C [a]`), built through the existing list machinery once every context
    /// dictionary exists for `E`. `None` when the representation is no list or the class has
    /// no conditional list instance; `Some(None)` while a context dictionary is missing.
    fn conditional_list_dictionary_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        pair_chirho: &GndPairChirho,
    ) -> Option<Option<CoreIdChirho>> {
        let element_chirho = pair_chirho
            .representation_key_chirho
            .strip_prefix('[')?
            .strip_suffix(']')?;
        let context_classes_chirho: Vec<String> = class_env_chirho
            .instances_chirho()
            .get_chirho(&pair_chirho.class_chirho)?
            .iter_chirho()
            .find(|inst_chirho| {
                !inst_chirho.context_chirho.is_empty()
                    && matches!(inst_chirho.head_ty_chirho, TyChirho::ListChirho(_))
            })?
            .context_chirho
            .iter()
            .map(|predicate_chirho| predicate_chirho.class_name_chirho.clone())
            .collect();
        let element_ready_chirho = context_classes_chirho.iter().all(|context_class_chirho| {
            self.instance_dicts_chirho
                .contains_key(&(context_class_chirho.clone(), element_chirho.to_string()))
        });
        if !element_ready_chirho {
            return Some(None);
        }
        self.generate_list_instance_dict_chirho(
            class_env_chirho,
            &pair_chirho.class_chirho,
            element_chirho,
            &pair_chirho.representation_key_chirho,
            &pair_chirho.layout_chirho,
            &context_classes_chirho,
        );
        Some(
            self.instance_dicts_chirho
                .get(&(
                    pair_chirho.class_chirho.clone(),
                    pair_chirho.representation_key_chirho.clone(),
                ))
                .copied(),
        )
    }

    /// The method field of the generated dictionary `dictionary_chirho` at `method_chirho`'s
    /// slot.
    fn dictionary_method_chirho(
        &self,
        dictionary_chirho: CoreIdChirho,
        layout_chirho: &DictLayoutChirho,
        method_chirho: &str,
    ) -> Option<CoreExprChirho> {
        let slot_chirho = layout_chirho
            .method_slots_chirho
            .iter()
            .find(|(name_chirho, _)| name_chirho == method_chirho)?
            .1;
        let binding_chirho = self
            .generated_bindings_chirho
            .iter()
            .find(|binding_chirho| binding_chirho.binder_chirho.id_chirho == dictionary_chirho)?;
        let CoreExprChirho::ConAppChirho { args_chirho, .. } = &binding_chirho.rhs_chirho else {
            return None;
        };
        args_chirho.get(slot_chirho).cloned()
    }

    /// Bind each of the newtype's rows to the representation's method, in place for a
    /// default row, then the newtype's dictionary, whose fields are those methods.
    fn build_gnd_dict_chirho(
        &mut self,
        pair_chirho: &GndPairChirho,
        methods_chirho: Vec<CoreExprChirho>,
    ) {
        let class_name_chirho = &pair_chirho.class_chirho;
        let type_key_chirho = &pair_chirho.type_key_chirho;
        let layout_chirho = &pair_chirho.layout_chirho;
        for ((method_name_chirho, _), method_chirho) in layout_chirho
            .method_slots_chirho
            .iter()
            .zip(methods_chirho.iter().cloned())
        {
            let newtype_row_chirho =
                format!("$prim_{class_name_chirho}_{method_name_chirho}_{type_key_chirho}");
            if let Some(default_row_chirho) = self
                .body_backed_names_chirho
                .get(&newtype_row_chirho)
                .copied()
            {
                self.gnd_default_rows_chirho
                    .insert(default_row_chirho, method_chirho);
                continue;
            }
            let alias_binder_chirho = self.fresh_binder_chirho(
                &newtype_row_chirho,
                TyChirho::VarChirho(TyVarChirho(self.next_id_chirho)),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: alias_binder_chirho,
                rhs_chirho: method_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        let dict_name_chirho = format!("$f{class_name_chirho}{type_key_chirho}");
        let dict_ty_chirho = TyChirho::ConChirho(format!("$Dict_{class_name_chirho}"));
        let dict_binder_chirho = self.fresh_binder_chirho(&dict_name_chirho, dict_ty_chirho);
        let con_name_chirho = format!("$Dict_{class_name_chirho}");
        let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

        // Superclass dictionaries: the newtype's own where it has one, else the
        // representation's, which has the newtype's representation.
        for slot_chirho in &layout_chirho.super_slots_chirho {
            // A displaced compiler superclass: the newtype's own namespace dictionary, else the
            // representation's, else its named refusal; never the spelling (row 692, C10).
            if let Some(identity_chirho) =
                self.displaced_identity_chirho(&slot_chirho.class_proof_chirho)
            {
                let served_chirho = self
                    .compiler_superclass_id_chirho(identity_chirho, &type_key_chirho)
                    .or_else(|| {
                        self.compiler_superclass_id_chirho(
                            identity_chirho,
                            &pair_chirho.representation_key_chirho,
                        )
                    });
                field_args_chirho.push(match served_chirho {
                    Some(id_chirho) => CoreExprChirho::VarChirho(id_chirho),
                    None => self.displaced_refusal_chirho(
                        identity_chirho,
                        "superclass slot",
                        &type_key_chirho,
                    ),
                });
                continue;
            }
            let super_name_chirho = &slot_chirho.namespace_chirho;
            let super_dict_name_chirho = format!("$f{super_name_chirho}{type_key_chirho}");
            let super_id_chirho = if self
                .instance_dicts_chirho
                .contains_key(&(super_name_chirho.clone(), type_key_chirho.clone()))
            {
                self.resolve_or_fresh_id_chirho(&super_dict_name_chirho)
            } else {
                let underlying_super_dict_chirho = format!(
                    "$f{super_name_chirho}{}",
                    pair_chirho.representation_key_chirho
                );
                self.resolve_or_fresh_id_chirho(&underlying_super_dict_chirho)
            };
            field_args_chirho.push(CoreExprChirho::VarChirho(super_id_chirho));
        }

        // Method fields: the representation's methods, which the rows bound above hold. A
        // default row takes leading context dictionaries; a field is reached without them.
        field_args_chirho.extend(methods_chirho);

        let dict_id_chirho = dict_binder_chirho.id_chirho;
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: dict_binder_chirho,
            rhs_chirho: CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho: field_args_chirho,
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        self.instance_dicts_chirho.insert(
            (class_name_chirho.clone(), type_key_chirho.clone()),
            dict_id_chirho,
        );
    }

    /// A GND newtype's default-filled row as the representation's method, behind one ignored
    /// parameter per dictionary of its instance's context (`C rep`), which every caller of a
    /// context-leading row passes first. `None` for any other row.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(in crate::dict_chirho) fn gnd_default_row_chirho(
        &mut self,
        binding_chirho: &CoreBindingChirho,
    ) -> Option<CoreBindingChirho> {
        let row_id_chirho = binding_chirho.binder_chirho.id_chirho;
        let mut rhs_chirho = self.gnd_default_rows_chirho.get(&row_id_chirho)?.clone();
        let context_classes_chirho: Vec<String> = self
            .context_leading_rows_chirho
            .get(&row_id_chirho)
            .map(|context_chirho| {
                context_chirho
                    .iter()
                    .map(|predicate_chirho| predicate_chirho.class_name_chirho.clone())
                    .collect()
            })
            .unwrap_or_default();
        let mut ty_chirho = binding_chirho.binder_chirho.ty_chirho.clone();
        for class_chirho in context_classes_chirho.iter().rev() {
            let dictionary_binder_chirho = self.fresh_binder_chirho(
                &format!("$d{class_chirho}"),
                TyChirho::ConChirho(format!("$Dict_{class_chirho}")),
            );
            ty_chirho = TyChirho::fun_chirho(dictionary_binder_chirho.ty_chirho.clone(), ty_chirho);
            rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: dictionary_binder_chirho,
                body_chirho: Box::new(rhs_chirho),
            };
        }
        Some(CoreBindingChirho {
            binder_chirho: BinderChirho {
                ty_chirho,
                ..binding_chirho.binder_chirho.clone()
            },
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        })
    }
}

/// An instance head's type constructor and the arguments it applies it to, in order.
fn head_application_chirho(head_ty_chirho: &TyChirho) -> (&TyChirho, Vec<&TyChirho>) {
    let mut arguments_chirho = Vec::new();
    let mut head_chirho = head_ty_chirho;
    while let TyChirho::AppChirho(function_chirho, argument_chirho) = head_chirho {
        arguments_chirho.push(argument_chirho.as_ref());
        head_chirho = function_chirho.as_ref();
    }
    arguments_chirho.reverse();
    (head_chirho, arguments_chirho)
}
