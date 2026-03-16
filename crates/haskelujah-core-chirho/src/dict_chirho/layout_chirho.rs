// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Dictionary layout building and method/superclass selector generation.

#![allow(unused_imports)]
use std::collections::HashMap;

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
};
use super::{DictPassCtxChirho, DictLayoutChirho};

impl DictPassCtxChirho {
    pub fn build_layouts_chirho(&mut self, class_env_chirho: &ClassEnvChirho) {
        for (name_chirho, decl_chirho) in &class_env_chirho.classes_chirho {
            let mut slot_chirho = 0usize;

            let super_slots_chirho: Vec<(String, usize)> = decl_chirho
                .supers_chirho
                .iter()
                .map(|super_name_chirho| {
                    let pos_chirho = slot_chirho;
                    slot_chirho += 1;
                    (super_name_chirho.clone(), pos_chirho)
                })
                .collect();

            // Sort method names for deterministic layout
            let mut method_names_chirho: Vec<String> =
                decl_chirho.methods_chirho.keys().cloned().collect();
            method_names_chirho.sort();

            let method_slots_chirho: Vec<(String, usize)> = method_names_chirho
                .into_iter()
                .map(|method_name_chirho| {
                    let pos_chirho = slot_chirho;
                    slot_chirho += 1;
                    (method_name_chirho, pos_chirho)
                })
                .collect();

            self.layouts_chirho.insert(
                name_chirho.clone(),
                DictLayoutChirho {
                    class_name_chirho: name_chirho.clone(),
                    super_slots_chirho,
                    method_slots_chirho,
                    field_count_chirho: slot_chirho,
                },
            );

            // Track parameter count for MPTC dispatch
            let param_count_chirho = decl_chirho.all_vars_chirho().len();
            self.class_param_count_chirho
                .insert(name_chirho.clone(), param_count_chirho);
        }
    }

    /// Generate method selector functions for each class.
    ///
    /// For a class `Eq` with method `==` at slot 0 in a 2-field dict:
    /// ```text
    /// $sel_Eq_== = \$dict -> case $dict of
    ///     $DictEq f0 f1 -> f0
    /// ```
    pub fn generate_selectors_chirho(&mut self) {
        let layouts_chirho: Vec<_> = self.layouts_chirho.values().cloned().collect();
        for layout_chirho in &layouts_chirho {
            for (method_name_chirho, slot_idx_chirho) in &layout_chirho.method_slots_chirho {
                let sel_name_chirho = format!(
                    "$sel_{}_{}",
                    layout_chirho.class_name_chirho, method_name_chirho
                );
                let dict_ty_chirho = TyChirho::ConChirho(format!(
                    "$Dict_{}",
                    layout_chirho.class_name_chirho
                ));

                // The dictionary binder for the lambda
                let dict_binder_chirho =
                    self.fresh_binder_chirho("$dict", dict_ty_chirho.clone());
                let dict_id_chirho = dict_binder_chirho.id_chirho;

                // Build field binders for the case alt
                let mut field_binders_chirho = Vec::new();
                let mut selected_id_chirho = CoreIdChirho(0);
                for i_chirho in 0..layout_chirho.field_count_chirho {
                    let field_name_chirho = format!("$f{i_chirho}");
                    let fb_chirho = self.fresh_binder_chirho(
                        &field_name_chirho,
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            9999,
                        )),
                    );
                    if i_chirho == *slot_idx_chirho {
                        selected_id_chirho = fb_chirho.id_chirho;
                    }
                    field_binders_chirho.push(fb_chirho);
                }

                let case_wild_chirho =
                    self.fresh_binder_chirho("$wild", dict_ty_chirho.clone());

                let con_name_chirho =
                    format!("$Dict_{}", layout_chirho.class_name_chirho);

                let case_expr_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
                    bind_chirho: case_wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(9998),
                    ),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(con_name_chirho),
                        binders_chirho: field_binders_chirho,
                        rhs_chirho: CoreExprChirho::VarChirho(selected_id_chirho),
                    }],
                };

                let selector_rhs_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: dict_binder_chirho,
                    body_chirho: Box::new(case_expr_chirho),
                };

                let sel_binder_chirho = self.fresh_binder_chirho(
                    &sel_name_chirho,
                    TyChirho::fun_chirho(
                        dict_ty_chirho,
                        TyChirho::VarChirho(
                            haskelujah_typing_chirho::ty_chirho::TyVarChirho(9998),
                        ),
                    ),
                );
                let sel_id_chirho = sel_binder_chirho.id_chirho;

                self.generated_bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: sel_binder_chirho,
                    rhs_chirho: selector_rhs_chirho,
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                });

                self.method_selectors_chirho.insert(
                    method_name_chirho.clone(),
                    (layout_chirho.class_name_chirho.clone(), sel_id_chirho),
                );
            }

            // Generate superclass selectors:
            // $sel_Ord_super_Eq = \$dict -> case $dict of $DictOrd f0 f1 ... -> f0
            for (super_name_chirho, slot_idx_chirho) in &layout_chirho.super_slots_chirho {
                let sel_name_chirho = format!(
                    "$sel_{}_super_{}",
                    layout_chirho.class_name_chirho, super_name_chirho
                );
                let dict_ty_chirho = TyChirho::ConChirho(format!(
                    "$Dict_{}",
                    layout_chirho.class_name_chirho
                ));

                let dict_binder_chirho =
                    self.fresh_binder_chirho("$dict", dict_ty_chirho.clone());
                let dict_id_chirho = dict_binder_chirho.id_chirho;

                let mut field_binders_chirho = Vec::new();
                let mut selected_id_chirho = CoreIdChirho(0);
                for i_chirho in 0..layout_chirho.field_count_chirho {
                    let field_name_chirho = format!("$f{i_chirho}");
                    let fb_chirho = self.fresh_binder_chirho(
                        &field_name_chirho,
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            9999,
                        )),
                    );
                    if i_chirho == *slot_idx_chirho {
                        selected_id_chirho = fb_chirho.id_chirho;
                    }
                    field_binders_chirho.push(fb_chirho);
                }

                let case_wild_chirho =
                    self.fresh_binder_chirho("$wild", dict_ty_chirho.clone());
                let con_name_chirho =
                    format!("$Dict_{}", layout_chirho.class_name_chirho);

                let case_expr_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
                    bind_chirho: case_wild_chirho,
                    result_ty_chirho: TyChirho::VarChirho(
                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(9998),
                    ),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(con_name_chirho),
                        binders_chirho: field_binders_chirho,
                        rhs_chirho: CoreExprChirho::VarChirho(selected_id_chirho),
                    }],
                };

                let selector_rhs_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: dict_binder_chirho,
                    body_chirho: Box::new(case_expr_chirho),
                };

                let super_dict_ty_chirho = TyChirho::ConChirho(format!(
                    "$Dict_{}",
                    super_name_chirho
                ));
                let sel_binder_chirho = self.fresh_binder_chirho(
                    &sel_name_chirho,
                    TyChirho::fun_chirho(dict_ty_chirho, super_dict_ty_chirho),
                );
                let sel_id_chirho = sel_binder_chirho.id_chirho;

                self.generated_bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: sel_binder_chirho,
                    rhs_chirho: selector_rhs_chirho,
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                });

                self.super_selectors_chirho.insert(
                    (
                        layout_chirho.class_name_chirho.clone(),
                        super_name_chirho.clone(),
                    ),
                    sel_id_chirho,
                );
            }
        }
    }
}
