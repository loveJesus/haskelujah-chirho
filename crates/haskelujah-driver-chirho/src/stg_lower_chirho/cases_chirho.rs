// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A case binder shares the actual scrutinee, not a second evaluation with stale
//! registers. Allocate the shared thunk per invocation, including closed effects.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use super::{
    AltConChirho, ArgSourceChirho, ClosureChirho, CodeChirho, CodePtrChirho, CoreExprChirho,
    LowerCtxChirho, ValueChirho, lower_lit_chirho,
};
use haskelujah_core_chirho::{BinderChirho, CoreAltChirho};
use haskelujah_typing_chirho::TyChirho;

impl LowerCtxChirho {
    fn lower_shared_case_chirho(
        &mut self,
        scrutinee_chirho: &CoreExprChirho,
        bind_chirho: &BinderChirho,
        result_ty_chirho: &TyChirho,
        alts_chirho: &[CoreAltChirho],
    ) -> u32 {
        let register_count_chirho = self
            .arg_param_indices_chirho
            .values()
            .max()
            .copied()
            .map_or(0, |index_chirho| index_chirho + 1);
        let scrutinee_code_chirho = self.lower_expr_chirho(scrutinee_chirho);
        let previous_index_chirho = self
            .arg_param_indices_chirho
            .insert(bind_chirho.id_chirho, register_count_chirho);
        let previous_value_chirho = self.env_chirho.remove(&bind_chirho.id_chirho);
        let body_chirho = self.lower_case_chirho(
            &CoreExprChirho::VarChirho(bind_chirho.id_chirho),
            bind_chirho,
            result_ty_chirho,
            alts_chirho,
        );
        if let Some(previous_index_chirho) = previous_index_chirho {
            self.arg_param_indices_chirho
                .insert(bind_chirho.id_chirho, previous_index_chirho);
        } else {
            self.arg_param_indices_chirho.remove(&bind_chirho.id_chirho);
        }
        if let Some(previous_value_chirho) = previous_value_chirho {
            self.env_chirho
                .insert(bind_chirho.id_chirho, previous_value_chirho);
        } else {
            self.env_chirho.remove(&bind_chirho.id_chirho);
        }
        self.emit_chirho(CodeChirho::StoreAllocThunkChirho {
            code_ptr_chirho: scrutinee_code_chirho,
            name_chirho: bind_chirho.name_chirho.clone(),
            captures_chirho: (0..register_count_chirho)
                .map(ArgSourceChirho::ArgRegChirho)
                .collect(),
            dest_reg_chirho: register_count_chirho,
            body_chirho,
        })
    }
    pub(super) fn lower_case_chirho(
        &mut self,
        scrutinee_chirho: &CoreExprChirho,
        bind_chirho: &BinderChirho,
        result_ty_chirho: &TyChirho,
        alts_chirho: &[CoreAltChirho],
    ) -> u32 {
        if !matches!(
            scrutinee_chirho,
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_)
        ) {
            return self.lower_shared_case_chirho(
                scrutinee_chirho,
                bind_chirho,
                result_ty_chirho,
                alts_chirho,
            );
        }
        // Bind the case binder to the scrutinee so that variable
        // patterns like `case e of x -> x` correctly reference
        // the scrutinee value. For variable scrutinees, copy the
        // arg-param index or env entry; for others, allocate a thunk.
        match scrutinee_chirho {
            CoreExprChirho::VarChirho(scrut_id_chirho) => {
                if let Some(&idx_chirho) = self.arg_param_indices_chirho.get(scrut_id_chirho) {
                    self.arg_param_indices_chirho
                        .insert(bind_chirho.id_chirho, idx_chirho);
                } else if let Some(val_chirho) = self.env_chirho.get(scrut_id_chirho).cloned() {
                    self.env_chirho.insert(bind_chirho.id_chirho, val_chirho);
                }
            }
            CoreExprChirho::LitChirho(lit_chirho) => {
                let val_chirho = lower_lit_chirho(lit_chirho);
                self.env_chirho.insert(bind_chirho.id_chirho, val_chirho);
            }
            _ => {
                // Complex scrutinee: allocate a thunk so the case
                // binder can force it if referenced.
                let scrut_entry_chirho = self.lower_expr_chirho(scrutinee_chirho);
                let thunk_chirho = ClosureChirho::thunk_chirho(
                    CodePtrChirho(scrut_entry_chirho),
                    "<case_binder>",
                    vec![],
                );
                let addr_chirho = self.heap_chirho.alloc_chirho(thunk_chirho);
                self.env_chirho.insert(
                    bind_chirho.id_chirho,
                    ValueChirho::HeapPtrChirho(addr_chirho),
                );
            }
        }
        // Newtype case erasure: if the only constructor alt is a
        // newtype constructor, skip case dispatch — the scrutinee
        // IS the inner value (newtype is erased at runtime).
        if alts_chirho.len() == 1 {
            if let AltConChirho::DataConChirho(con_name_chirho) = &alts_chirho[0].con_chirho {
                if self.newtype_cons_chirho.contains(con_name_chirho)
                    && alts_chirho[0].binders_chirho.len() == 1
                {
                    // Map the single field binder to the scrutinee.
                    // The scrutinee value IS the unwrapped value.
                    let binder_id_chirho = alts_chirho[0].binders_chirho[0].id_chirho;
                    // Copy the scrutinee's resolution into the binder
                    match scrutinee_chirho {
                        CoreExprChirho::VarChirho(id_chirho) => {
                            if let Some(idx_chirho) = self.arg_param_indices_chirho.get(id_chirho) {
                                self.arg_param_indices_chirho
                                    .insert(binder_id_chirho, *idx_chirho);
                            } else if let Some(val_chirho) = self.env_chirho.get(id_chirho) {
                                self.env_chirho.insert(binder_id_chirho, val_chirho.clone());
                            }
                        }
                        _ => {
                            // For non-variable scrutinees, allocate a
                            // thunk on the heap and bind the binder to it.
                            let scrut_entry_chirho = self.lower_expr_chirho(scrutinee_chirho);
                            let thunk_chirho = ClosureChirho::thunk_chirho(
                                CodePtrChirho(scrut_entry_chirho),
                                "<newtype_scrut>",
                                vec![],
                            );
                            let addr_chirho = self.heap_chirho.alloc_chirho(thunk_chirho);
                            self.env_chirho
                                .insert(binder_id_chirho, ValueChirho::HeapPtrChirho(addr_chirho));
                        }
                    }
                    return self.lower_expr_chirho(&alts_chirho[0].rhs_chirho);
                }
            }
        }

        // Classify alternatives: constructor-based or literal-based.
        let mut con_entries_chirho: Vec<(u16, u32)> = Vec::new();
        let mut lit_entries_chirho: Vec<(ValueChirho, u32)> = Vec::new();
        let mut default_entry_chirho: Option<u32> = None;

        for alt_chirho in alts_chirho {
            // Bind alt binders as arg param indices so that
            // constructor fields are accessible via ArgChirho
            let saved_arg_indices_chirho = self.arg_param_indices_chirho.clone();
            let saved_env_entries_chirho: Vec<_> = alt_chirho
                .binders_chirho
                .iter()
                .map(|b_chirho| {
                    (
                        b_chirho.id_chirho,
                        self.env_chirho.get(&b_chirho.id_chirho).cloned(),
                    )
                })
                .collect();

            // When the case alt has binders, the runtime prepends
            // the constructor fields to the saved arg regs.
            // Shift existing arg param indices to account for
            // the prepended fields so outer parameters remain
            // accessible at their shifted positions.
            let num_binders_chirho = alt_chirho.binders_chirho.len();
            if num_binders_chirho > 0 {
                for (_id_chirho, idx_chirho) in self.arg_param_indices_chirho.iter_mut() {
                    *idx_chirho += num_binders_chirho;
                }
            }

            for (i_chirho, b_chirho) in alt_chirho.binders_chirho.iter().enumerate() {
                self.arg_param_indices_chirho
                    .insert(b_chirho.id_chirho, i_chirho);
                self.env_chirho.remove(&b_chirho.id_chirho);
            }

            let rhs_entry_chirho = self.lower_expr_chirho(&alt_chirho.rhs_chirho);

            // Restore arg param indices and env
            self.arg_param_indices_chirho = saved_arg_indices_chirho;
            for (id_chirho, prev_chirho) in saved_env_entries_chirho {
                match prev_chirho {
                    Some(v_chirho) => {
                        self.env_chirho.insert(id_chirho, v_chirho);
                    }
                    None => {
                        self.env_chirho.remove(&id_chirho);
                    }
                }
            }

            match &alt_chirho.con_chirho {
                AltConChirho::DataConChirho(name_chirho) => {
                    let tag_chirho = self.con_tag_chirho(name_chirho);
                    con_entries_chirho.push((tag_chirho, rhs_entry_chirho));
                }
                AltConChirho::LitConChirho(lit_chirho) => {
                    let val_chirho = lower_lit_chirho(lit_chirho);
                    lit_entries_chirho.push((val_chirho, rhs_entry_chirho));
                }
                AltConChirho::DefaultChirho => {
                    default_entry_chirho = Some(rhs_entry_chirho);
                }
            }
        }

        // Lower scrutinee as ArgSourceChirho for runtime resolution.
        let scrut_source_chirho = self.lower_arg_source_chirho(scrutinee_chirho);

        if !lit_entries_chirho.is_empty() {
            self.emit_chirho(CodeChirho::CaseLitChirho {
                scrutinee_chirho: scrut_source_chirho,
                alts_chirho: lit_entries_chirho,
                default_chirho: default_entry_chirho,
            })
        } else {
            // Constructor case dispatch
            self.emit_chirho(CodeChirho::CaseChirho {
                scrutinee_chirho: scrut_source_chirho,
                alts_chirho: con_entries_chirho,
                default_chirho: default_entry_chirho,
            })
        }
    }
}
