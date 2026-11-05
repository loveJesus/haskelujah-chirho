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
//! Core erases these newtypes' constructors (`simplify_chirho/newtype_erasure_chirho.rs`),
//! so a representation's method receives the field itself on every backend.
//! workflow: language-features-chirho/deriving-strategies-chirho

use haskelujah_typing_chirho::GndCoercionProofChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::super::{DictLayoutChirho, DictPassCtxChirho};
use crate::expr_chirho::{CoreBindingChirho, CoreExprChirho, CoreIdChirho, InlineAnnotationChirho};

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
    /// workflow: language-features-chirho/deriving-strategies-chirho
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
    /// workflow: language-features-chirho/deriving-strategies-chirho
    pub fn generate_gnd_dicts_chirho(&mut self, class_env_chirho: &ClassEnvChirho) {
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
                    Err(reason_chirho) => self.gnd_unsupported_chirho.push(reason_chirho),
                }
            }
            if waiting_chirho.len() == before_chirho {
                for pair_chirho in &waiting_chirho {
                    self.gnd_unsupported_chirho.push(format!(
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

    /// Every GND instance the class environment holds, in (class, newtype) order.
    fn gnd_pairs_chirho(&self, class_env_chirho: &ClassEnvChirho) -> Vec<GndPairChirho> {
        let mut pairs_chirho = Vec::new();
        for (class_name_chirho, instances_chirho) in &class_env_chirho.instances_chirho {
            let Some(layout_chirho) = self.layouts_chirho.get(class_name_chirho) else {
                continue;
            };
            for inst_chirho in instances_chirho {
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
            return Ok(Some(CoreExprChirho::VarChirho(row_chirho)));
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
            .instances_chirho
            .get(&pair_chirho.class_chirho)?
            .iter()
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
    /// default row, then the newtype's dictionary.
    fn build_gnd_dict_chirho(
        &mut self,
        pair_chirho: &GndPairChirho,
        methods_chirho: Vec<CoreExprChirho>,
    ) {
        let class_name_chirho = &pair_chirho.class_chirho;
        let type_key_chirho = &pair_chirho.type_key_chirho;
        let layout_chirho = &pair_chirho.layout_chirho;
        for ((method_name_chirho, _), method_chirho) in
            layout_chirho.method_slots_chirho.iter().zip(methods_chirho)
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
        for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
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

        // Method fields: the newtype's rows bound above.
        for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
            let prim_name_chirho =
                format!("$prim_{class_name_chirho}_{method_name_chirho}_{type_key_chirho}");
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
            field_args_chirho.push(CoreExprChirho::VarChirho(prim_id_chirho));
        }

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
}
