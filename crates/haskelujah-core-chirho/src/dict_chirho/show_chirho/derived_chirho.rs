// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Derived Show in portable Core (row 533): GHC's derived precedence rules over the
//! constructor metadata the deriving pass records. Each concrete key gets one recursive
//! helper, `$prim_Show_showsPrec_<key> :: Int -> T -> String`, internal to this pass: the
//! Show class still exposes only `show`, and a monomorphic type's public row
//! `$prim_Show_show_<T>` becomes its helper at precedence 0. A key with any field no shape
//! renders keeps its existing route; nothing is defaulted to Int or guessed from text.
//! Workflow: spec-chirho/workflows-chirho/print-show-evidence-chirho.md.

use std::collections::HashMap;

use haskelujah_typing_chirho::deriving_chirho::{
    DerivedShowConChirho, ShowConFormChirho, ShowFieldTypeChirho,
};

use super::shape_chirho::split_outer_chirho;
use super::{
    AltConChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreLitChirho,
    DictPassCtxChirho, InlineAnnotationChirho, TyChirho, join_string_parts_chirho,
    string_lit_chirho,
};
use crate::expr_chirho::CoreIdChirho;

/// The precedence of a prefix or record constructor application.
const APPLICATION_PRECEDENCE_CHIRHO: u8 = 10;

fn apply_chirho(
    function_chirho: CoreExprChirho,
    argument_chirho: CoreExprChirho,
) -> CoreExprChirho {
    CoreExprChirho::AppChirho {
        fun_chirho: Box::new(function_chirho),
        arg_chirho: Box::new(argument_chirho),
    }
}

fn int_lit_chirho(value_chirho: u8) -> CoreExprChirho {
    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(i64::from(value_chirho)))
}

/// A symbolic name in prefix position is written in parentheses: `(:*)`.
fn prefix_name_chirho(name_chirho: &str) -> String {
    if name_chirho
        .starts_with(|first_chirho: char| first_chirho.is_alphabetic() || first_chirho == '_')
    {
        name_chirho.to_string()
    } else {
        format!("({name_chirho})")
    }
}

/// The precedence above which a constructor is parenthesized.
fn constructor_precedence_chirho(form_chirho: &ShowConFormChirho) -> u8 {
    match form_chirho {
        ShowConFormChirho::InfixChirho(precedence_chirho) => *precedence_chirho,
        ShowConFormChirho::PrefixChirho | ShowConFormChirho::RecordChirho(_) => {
            APPLICATION_PRECEDENCE_CHIRHO
        }
    }
}

/// The precedence a constructor's fields render at: 11 in prefix form, 0 in a record,
/// p + 1 for both operands of an infix constructor.
fn field_precedence_chirho(form_chirho: &ShowConFormChirho) -> u8 {
    match form_chirho {
        ShowConFormChirho::PrefixChirho => APPLICATION_PRECEDENCE_CHIRHO + 1,
        ShowConFormChirho::RecordChirho(_) => 0,
        ShowConFormChirho::InfixChirho(precedence_chirho) => precedence_chirho.saturating_add(1),
    }
}

/// Spell a field type as a Show key, the way the checker spells concrete keys: a nested
/// application in parentheses, tuple parts without spaces. A parameter becomes the key's
/// own argument, which already arrives in that spelling.
fn field_key_chirho(
    field_chirho: &ShowFieldTypeChirho,
    arguments_chirho: &HashMap<String, String>,
    nested_chirho: bool,
) -> Option<String> {
    match field_chirho {
        ShowFieldTypeChirho::VarChirho(name_chirho) => arguments_chirho.get(name_chirho).cloned(),
        ShowFieldTypeChirho::ConChirho(name_chirho) => Some(name_chirho.clone()),
        ShowFieldTypeChirho::ListChirho(element_chirho) => Some(format!(
            "[{}]",
            field_key_chirho(element_chirho, arguments_chirho, true)?
        )),
        ShowFieldTypeChirho::TupleChirho(items_chirho) if items_chirho.len() >= 2 => {
            let keys_chirho = items_chirho
                .iter()
                .map(|item_chirho| field_key_chirho(item_chirho, arguments_chirho, true))
                .collect::<Option<Vec<_>>>()?;
            Some(format!("({})", keys_chirho.join(",")))
        }
        ShowFieldTypeChirho::TupleChirho(_) => None,
        ShowFieldTypeChirho::AppChirho(function_chirho, argument_chirho) => {
            let key_chirho = format!(
                "{} {}",
                field_key_chirho(function_chirho, arguments_chirho, false)?,
                field_key_chirho(argument_chirho, arguments_chirho, true)?
            );
            Some(if nested_chirho {
                format!("({key_chirho})")
            } else {
                key_chirho
            })
        }
    }
}

/// A constructor's fields as (Show key, precedence), under the key's arguments.
fn constructor_field_keys_chirho(
    constructor_chirho: &DerivedShowConChirho,
    arguments_chirho: &HashMap<String, String>,
) -> Option<Vec<(String, u8)>> {
    let precedence_chirho = field_precedence_chirho(&constructor_chirho.form_chirho);
    constructor_chirho
        .fields_chirho
        .iter()
        .map(|field_chirho| {
            Some((
                field_key_chirho(field_chirho, arguments_chirho, false)?,
                precedence_chirho,
            ))
        })
        .collect()
}

impl DictPassCtxChirho {
    /// The type name, and its parameters' arguments, behind a concrete key whose head
    /// derives Show.
    fn derived_show_arguments_chirho(
        &self,
        key_chirho: &str,
    ) -> Option<(String, HashMap<String, String>)> {
        let parts_chirho = split_outer_chirho(key_chirho, ' ')?;
        let (head_chirho, arguments_chirho) = parts_chirho.split_first()?;
        let metadata_chirho = self.derived_shows_chirho.get(*head_chirho)?;
        if metadata_chirho.type_vars_chirho.len() != arguments_chirho.len() {
            return None;
        }
        let substitution_chirho: HashMap<String, String> = metadata_chirho
            .type_vars_chirho
            .iter()
            .cloned()
            .zip(
                arguments_chirho
                    .iter()
                    .map(|argument_chirho| argument_chirho.to_string()),
            )
            .collect();
        Some((head_chirho.to_string(), substitution_chirho))
    }

    /// `key` itself when it names a type that derives Show and every field of every
    /// constructor renders at its precedence; otherwise `None`, which keeps the key on its
    /// existing route. A key whose check is under way counts as rendering, so a recursive
    /// type is admitted exactly when everything else in it renders.
    ///
    /// Each key is checked once. A `false` is final, since an assumption only ever answers
    /// `true`; a `true` is kept unless it rested on an assumption about a key whose own
    /// check is still under way further out.
    pub(super) fn derived_show_key_chirho(&self, key_chirho: &str) -> Option<String> {
        let key_chirho = key_chirho.trim();
        let (type_name_chirho, arguments_chirho) =
            self.derived_show_arguments_chirho(key_chirho)?;
        let known_chirho = self
            .derived_show_known_chirho
            .borrow()
            .get(key_chirho)
            .copied();
        if let Some(renders_chirho) = known_chirho {
            return renders_chirho.then(|| key_chirho.to_string());
        }
        if self
            .derived_show_visiting_chirho
            .borrow()
            .iter()
            .any(|visiting_chirho| visiting_chirho == key_chirho)
        {
            self.derived_show_assumed_chirho
                .borrow_mut()
                .push(key_chirho.to_string());
            return Some(key_chirho.to_string());
        }
        let assumed_from_chirho = self.derived_show_assumed_chirho.borrow().len();
        self.derived_show_visiting_chirho
            .borrow_mut()
            .push(key_chirho.to_string());
        let renders_chirho = self.derived_shows_chirho[&type_name_chirho]
            .constructors_chirho
            .iter()
            .all(|constructor_chirho| {
                constructor_field_keys_chirho(constructor_chirho, &arguments_chirho).is_some_and(
                    |fields_chirho| {
                        fields_chirho
                            .iter()
                            .all(|(field_key_chirho, precedence_chirho)| {
                                self.parse_show_shape_at_chirho(
                                    field_key_chirho,
                                    *precedence_chirho,
                                )
                                .is_some()
                            })
                    },
                )
            });
        self.derived_show_visiting_chirho.borrow_mut().pop();
        let final_chirho = !renders_chirho || {
            let visiting_chirho = self.derived_show_visiting_chirho.borrow();
            self.derived_show_assumed_chirho.borrow()[assumed_from_chirho..]
                .iter()
                .all(|assumed_chirho| !visiting_chirho.contains(assumed_chirho))
        };
        if final_chirho {
            self.derived_show_known_chirho
                .borrow_mut()
                .insert(key_chirho.to_string(), renders_chirho);
        }
        if self.derived_show_visiting_chirho.borrow().is_empty() {
            self.derived_show_assumed_chirho.borrow_mut().clear();
        }
        renders_chirho.then(|| key_chirho.to_string())
    }

    /// A derived-Show value at a precedence: a call to its key's helper.
    pub(super) fn show_derived_expr_chirho(
        &mut self,
        key_chirho: &str,
        value_chirho: CoreExprChirho,
        precedence_chirho: u8,
    ) -> CoreExprChirho {
        let helper_chirho = self.derived_show_helper_chirho(key_chirho);
        apply_chirho(
            apply_chirho(
                CoreExprChirho::VarChirho(helper_chirho),
                int_lit_chirho(precedence_chirho),
            ),
            value_chirho,
        )
    }

    /// The helper of an admitted key, generated on first use. Its id is recorded before
    /// its body is built, so a recursive type's fields call the helper being built.
    fn derived_show_helper_chirho(&mut self, key_chirho: &str) -> CoreIdChirho {
        if let Some(helper_chirho) = self.derived_show_helpers_chirho.get(key_chirho) {
            return *helper_chirho;
        }
        let binder_chirho = self.fresh_binder_chirho(
            &format!("$prim_Show_showsPrec_{key_chirho}"),
            TyChirho::fun_n_chirho(
                [TyChirho::int_chirho(), TyChirho::int_chirho()],
                TyChirho::string_chirho(),
            ),
        );
        let helper_chirho = binder_chirho.id_chirho;
        self.derived_show_helpers_chirho
            .insert(key_chirho.to_string(), helper_chirho);
        let precedence_binder_chirho =
            self.fresh_binder_chirho("show_precedence_chirho", TyChirho::int_chirho());
        let value_binder_chirho =
            self.fresh_binder_chirho("show_derived_value_chirho", TyChirho::int_chirho());
        let (type_name_chirho, arguments_chirho) = self
            .derived_show_arguments_chirho(key_chirho)
            .expect("the parser admits only keys whose head derives Show");
        let constructors_chirho = self.derived_shows_chirho[&type_name_chirho]
            .constructors_chirho
            .clone();
        let alts_chirho = constructors_chirho
            .iter()
            .map(|constructor_chirho| {
                self.derived_show_alt_chirho(
                    constructor_chirho,
                    &arguments_chirho,
                    precedence_binder_chirho.id_chirho,
                )
            })
            .collect();
        let body_chirho = self.show_case_chirho(
            CoreExprChirho::VarChirho(value_binder_chirho.id_chirho),
            alts_chirho,
        );
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: precedence_binder_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: value_binder_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            },
            is_rec_chirho: true,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        helper_chirho
    }

    /// One constructor's alternative: a bare name with no fields; otherwise its text,
    /// parenthesized when the requested precedence is above the constructor's.
    fn derived_show_alt_chirho(
        &mut self,
        constructor_chirho: &DerivedShowConChirho,
        arguments_chirho: &HashMap<String, String>,
        precedence_id_chirho: CoreIdChirho,
    ) -> CoreAltChirho {
        let fields_chirho = constructor_field_keys_chirho(constructor_chirho, arguments_chirho)
            .expect("an admitted key renders every field");
        let binders_chirho: Vec<_> = fields_chirho
            .iter()
            .map(|_| self.fresh_binder_chirho("show_derived_field_chirho", TyChirho::int_chirho()))
            .collect();
        let field_ids_chirho: Vec<CoreIdChirho> = binders_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.id_chirho)
            .collect();
        let rhs_chirho = if fields_chirho.is_empty() {
            string_lit_chirho(&prefix_name_chirho(&constructor_chirho.name_chirho))
        } else {
            let bare_chirho = self.derived_show_text_chirho(
                constructor_chirho,
                &fields_chirho,
                &field_ids_chirho,
            );
            let wrapped_chirho = self.derived_show_text_chirho(
                constructor_chirho,
                &fields_chirho,
                &field_ids_chirho,
            );
            self.show_case_chirho(
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "<#".to_string(),
                    args_chirho: vec![
                        int_lit_chirho(constructor_precedence_chirho(
                            &constructor_chirho.form_chirho,
                        )),
                        CoreExprChirho::VarChirho(precedence_id_chirho),
                    ],
                },
                vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: join_string_parts_chirho(vec![
                            string_lit_chirho("("),
                            wrapped_chirho,
                            string_lit_chirho(")"),
                        ]),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: bare_chirho,
                    },
                ],
            )
        };
        CoreAltChirho {
            con_chirho: AltConChirho::DataConChirho(constructor_chirho.name_chirho.clone()),
            binders_chirho,
            rhs_chirho,
        }
    }

    /// A constructor's text without its outer parentheses: `C a1 a2`, `C {l = a}` or
    /// `a1 op a2`, each field rendered at its form's precedence.
    fn derived_show_text_chirho(
        &mut self,
        constructor_chirho: &DerivedShowConChirho,
        fields_chirho: &[(String, u8)],
        field_ids_chirho: &[CoreIdChirho],
    ) -> CoreExprChirho {
        let mut shown_chirho = Vec::new();
        for ((field_key_chirho, precedence_chirho), field_id_chirho) in
            fields_chirho.iter().zip(field_ids_chirho)
        {
            let shape_chirho = self
                .parse_show_shape_at_chirho(field_key_chirho, *precedence_chirho)
                .expect("an admitted key renders every field");
            shown_chirho.push(self.show_shape_expr_chirho(
                &shape_chirho,
                CoreExprChirho::VarChirho(*field_id_chirho),
                *precedence_chirho,
            ));
        }
        let name_chirho = &constructor_chirho.name_chirho;
        let parts_chirho = match &constructor_chirho.form_chirho {
            ShowConFormChirho::InfixChirho(_) if shown_chirho.len() == 2 => {
                let right_chirho = shown_chirho
                    .pop()
                    .expect("an infix constructor has two operands");
                let left_chirho = shown_chirho
                    .pop()
                    .expect("an infix constructor has two operands");
                vec![
                    left_chirho,
                    string_lit_chirho(&format!(" {name_chirho} ")),
                    right_chirho,
                ]
            }
            ShowConFormChirho::RecordChirho(labels_chirho) => {
                let mut parts_chirho = vec![string_lit_chirho(&format!(
                    "{} {{",
                    prefix_name_chirho(name_chirho)
                ))];
                for (index_chirho, (label_chirho, field_chirho)) in
                    labels_chirho.iter().zip(shown_chirho).enumerate()
                {
                    let separator_chirho = if index_chirho == 0 { "" } else { ", " };
                    parts_chirho.push(string_lit_chirho(&format!(
                        "{separator_chirho}{} = ",
                        prefix_name_chirho(label_chirho)
                    )));
                    parts_chirho.push(field_chirho);
                }
                parts_chirho.push(string_lit_chirho("}"));
                parts_chirho
            }
            _ => {
                let mut parts_chirho = vec![string_lit_chirho(&prefix_name_chirho(name_chirho))];
                for field_chirho in shown_chirho {
                    parts_chirho.push(string_lit_chirho(" "));
                    parts_chirho.push(field_chirho);
                }
                parts_chirho
            }
        };
        join_string_parts_chirho(parts_chirho)
    }

    /// Point each renderable monomorphic derived type's public row, `$prim_Show_show_<T>`
    /// (the derived instance's `show`), at its helper at precedence 0. Evidenced `print`,
    /// `show` occurrences and the instance dictionary then all render as GHC does. A type
    /// with parameters keeps its row; its concrete keys render through the parser.
    pub(in crate::dict_chirho) fn rewrite_derived_show_rows_chirho(
        &mut self,
        bindings_chirho: &mut [CoreBindingChirho],
    ) {
        let mut type_names_chirho: Vec<String> = self
            .derived_shows_chirho
            .iter()
            .filter(|(_, metadata_chirho)| metadata_chirho.type_vars_chirho.is_empty())
            .map(|(type_name_chirho, _)| type_name_chirho.clone())
            .collect();
        type_names_chirho.sort();
        for type_name_chirho in type_names_chirho {
            if self.derived_show_key_chirho(&type_name_chirho).is_none() {
                continue;
            }
            let row_name_chirho = format!("$prim_Show_show_{type_name_chirho}");
            let Some(binding_chirho) = bindings_chirho
                .iter_mut()
                .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == row_name_chirho)
            else {
                continue;
            };
            let argument_chirho =
                self.fresh_binder_chirho("show_arg_chirho", TyChirho::int_chirho());
            let shown_chirho = self.show_derived_expr_chirho(
                &type_name_chirho,
                CoreExprChirho::VarChirho(argument_chirho.id_chirho),
                0,
            );
            binding_chirho.rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: argument_chirho,
                body_chirho: Box::new(shown_chirho),
            };
        }
    }
}

#[cfg(test)]
#[path = "derived_tests_chirho.rs"]
mod tests_chirho;
