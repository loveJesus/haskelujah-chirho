// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The newtype deriving strategy's dictionaries (GND, row 694).
//!
//! A newtype instance takes its representation type's method rows only when the newtype's
//! own declarations route that class through the newtype strategy, as the deriving pass
//! records in `NewtypeInfoChirho`. A written instance with a context, or one that merely
//! has a newtype head, keeps its own methods. Every method of a GND instance is the
//! representation's, its class defaults included, as GHC coerces each one (G3, D2).
//! Core erases these newtypes' constructors (`simplify_chirho/newtype_erasure_chirho.rs`),
//! so a representation row receives the field itself on every backend.
//! workflow: language-features-chirho/deriving-strategies-chirho

use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::super::{DictLayoutChirho, DictPassCtxChirho};
use crate::expr_chirho::{CoreBindingChirho, CoreExprChirho, InlineAnnotationChirho};

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
    /// such newtype is erased, so the value is the last representation's, and no alias
    /// waits on another newtype's alias being built first. Each step names a distinct
    /// newtype; a longer chain is a cycle, which the checker refuses, and stops.
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
    /// A newtype instance whose class the newtype's own declarations route through the
    /// newtype strategy (`Num Int => Num Age`) gets, for each class method, a
    /// `$prim_Class_method_NewtypeKey` alias of the representation type's row, then its
    /// dictionary. The deriving pass writes a GND instance with no method, and typing refuses
    /// a second instance for its head (GHC-59692), so a newtype row that has a body is the
    /// class default the desugarer filled in: it becomes the representation's row in place.
    /// workflow: language-features-chirho/deriving-strategies-chirho
    pub fn generate_gnd_dicts_chirho(&mut self, class_env_chirho: &ClassEnvChirho) {
        // Collect the instances whose class the newtype's own declarations route
        // through the newtype strategy.
        let mut gnd_instances_chirho: Vec<(String, String, String, DictLayoutChirho)> = Vec::new();

        for (class_name_chirho, instances_chirho) in &class_env_chirho.instances_chirho {
            let layout_chirho = match self.layouts_chirho.get(class_name_chirho) {
                Some(l_chirho) => l_chirho.clone(),
                None => continue,
            };

            for inst_chirho in instances_chirho {
                let type_key_chirho = format!("{}", inst_chirho.head_ty_chirho);
                // The newtype's own declarations are the only proof: an instance's context
                // and head never make it one (row 694).
                if !self.takes_representation_dictionary_chirho(class_name_chirho, &type_key_chirho)
                {
                    continue;
                }
                if let Some(info_chirho) = self.newtype_info_chirho.get(&type_key_chirho) {
                    let representation_key_chirho = self.representation_key_chirho(
                        class_name_chirho,
                        &info_chirho.underlying_key_chirho,
                    );
                    gnd_instances_chirho.push((
                        class_name_chirho.clone(),
                        type_key_chirho,
                        representation_key_chirho,
                        layout_chirho.clone(),
                    ));
                }
            }
        }

        for (class_name_chirho, type_key_chirho, underlying_key_chirho, layout_chirho) in
            gnd_instances_chirho
        {
            // Generate $prim aliases: $prim_Class_method_Newtype = $prim_Class_method_Underlying
            for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                let newtype_prim_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, type_key_chirho
                );
                let underlying_prim_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, underlying_key_chirho
                );
                let underlying_id_chirho = self.resolve_or_missing_method_id_chirho(
                    class_env_chirho,
                    &class_name_chirho,
                    method_name_chirho,
                    &underlying_key_chirho,
                    &underlying_prim_chirho,
                    None,
                );
                // A class default the desugarer filled in becomes the representation's row.
                if let Some(default_row_chirho) = self
                    .body_backed_names_chirho
                    .get(&newtype_prim_chirho)
                    .copied()
                {
                    self.gnd_default_rows_chirho
                        .insert(default_row_chirho, underlying_id_chirho);
                    continue;
                }
                let alias_binder_chirho = self.fresh_binder_chirho(
                    &newtype_prim_chirho,
                    TyChirho::VarChirho(TyVarChirho(self.next_id_chirho)),
                );
                self.generated_bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: alias_binder_chirho,
                    rhs_chirho: CoreExprChirho::VarChirho(underlying_id_chirho),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                });
            }

            // Now build the dictionary (same as ground instances)
            let dict_name_chirho = format!("$f{}{}", class_name_chirho, type_key_chirho);
            let dict_ty_chirho = TyChirho::ConChirho(format!("$Dict_{}", class_name_chirho));
            let dict_binder_chirho = self.fresh_binder_chirho(&dict_name_chirho, dict_ty_chirho);

            let con_name_chirho = format!("$Dict_{}", class_name_chirho);
            let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

            // Superclass dictionary arguments — reference the newtype's superclass dicts
            for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
                let super_dict_name_chirho = format!("$f{}{}", super_name_chirho, type_key_chirho);
                // Check if we already have this dict; if not, try underlying type
                let super_id_chirho = if self
                    .instance_dicts_chirho
                    .contains_key(&(super_name_chirho.clone(), type_key_chirho.clone()))
                {
                    self.resolve_or_fresh_id_chirho(&super_dict_name_chirho)
                } else {
                    let underlying_super_dict_chirho =
                        format!("$f{}{}", super_name_chirho, underlying_key_chirho);
                    self.resolve_or_fresh_id_chirho(&underlying_super_dict_chirho)
                };
                field_args_chirho.push(CoreExprChirho::VarChirho(super_id_chirho));
            }

            // Method arguments — reference the newtype's prim bindings
            for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                let prim_name_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, type_key_chirho
                );
                let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
                field_args_chirho.push(CoreExprChirho::VarChirho(prim_id_chirho));
            }

            let dict_expr_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho: field_args_chirho,
            };

            let dict_id_chirho = dict_binder_chirho.id_chirho;
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: dict_binder_chirho,
                rhs_chirho: dict_expr_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            self.instance_dicts_chirho
                .insert((class_name_chirho, type_key_chirho), dict_id_chirho);
        }
    }
}
