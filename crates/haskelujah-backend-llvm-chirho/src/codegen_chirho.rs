// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # LLVM IR code generation
//!
//! Translates Core IR to textual LLVM IR. Currently implements a strict
//! evaluation model for the subset of Core we can handle:
//!
//! - Integer literals → i64 constants
//! - Function definitions → LLVM functions
//! - Function application → direct calls (when function is known)
//! - Case expressions → br/switch on scrutinee
//! - Let bindings → alloca + store/load
//! - Lambdas → lifted to top-level functions (lambda lifting)
//!
//! The full STG machine with lazy evaluation, thunks, closures, and
//! info tables will be added incrementally.

mod lazy_chirho;
mod primitives_chirho;

use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use haskelujah_core_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho,
};
use haskelujah_typing_chirho::ty_chirho::TyChirho;

#[cfg(test)]
use haskelujah_rts_chirho::{ObjectKindChirho, pack_native_header_chirho};

const BOXED_CONSTRUCTOR_TAG_MASK_CHIRHO: i64 = 1;
const BOXED_CONSTRUCTOR_PTR_MASK_CHIRHO: i64 = !1_i64;
const NATIVE_OBJECT_HEADER_PTR_MASK_CHIRHO: i64 = !0b11_i64;
const NATIVE_FUNCTION_HEADER_KIND_CHIRHO: i64 = 0b01;

/// LLVM IR generation context.
pub struct LlvmCodegenChirho {
    /// The LLVM IR output buffer.
    output_chirho: String,
    unsupported_primitives_chirho: HashSet<String>,
    /// Module-level globals emitted ahead of function bodies.
    global_defs_chirho: String,
    /// Counter for generating unique LLVM temporaries (%t0, %t1, ...).
    next_tmp_chirho: u32,
    /// Counter for generating unique LLVM labels.
    next_label_chirho: u32,
    /// Counter for generating unique LLVM string globals.
    next_string_chirho: u32,
    /// Counter for generating unique lifted local function symbols.
    next_lifted_function_chirho: u32,
    /// Lifted lambda functions accumulated during codegen.
    lifted_functions_chirho: Vec<String>,
    /// Track emitted function names to prevent duplicate definitions.
    emitted_names_chirho: HashSet<String>,
    /// Deduplicated mapping from string literal contents to LLVM global names.
    string_globals_chirho: HashMap<String, String>,
    /// Maps CoreId → name for top-level bindings, used to resolve cross-references.
    toplevel_names_chirho: HashMap<CoreIdChirho, String>,
    /// Maps CoreId → resolved Haskell name for any identifier referenced by Core.
    resolved_names_chirho: HashMap<CoreIdChirho, String>,
    /// Maps CoreId → lambda arity for top-level bindings.
    toplevel_arities_chirho: HashMap<CoreIdChirho, usize>,
    /// Known basic runtime kinds for top-level binders.
    toplevel_value_kinds_chirho: HashMap<CoreIdChirho, ShowBuiltinKindChirho>,
    /// Tracks CoreIds that are lambda parameters or let-bound in the current function scope.
    local_scope_chirho: HashSet<CoreIdChirho>,
    /// Known basic runtime kinds for local binders in the current function scope.
    local_value_kinds_chirho: HashMap<CoreIdChirho, ShowBuiltinKindChirho>,
    /// Current LLVM SSA name for a local Core id when it has been rebound from a case/let path.
    local_value_names_chirho: HashMap<CoreIdChirho, String>,
    /// Maps let/where-bound lambda CoreIds to their lifted LLVM symbol names.
    lifted_local_names_chirho: HashMap<CoreIdChirho, String>,
    /// Maps let/where-bound lambda CoreIds to their runtime arity.
    lifted_local_arities_chirho: HashMap<CoreIdChirho, usize>,
    /// Maps let/where-bound lambda CoreIds to the outer locals they capture.
    lifted_local_capture_ids_chirho: HashMap<CoreIdChirho, Vec<CoreIdChirho>>,
    /// Core id of the function currently eligible for self-tail-call elimination.
    current_tco_self_id_chirho: Option<CoreIdChirho>,
    /// Loop label for the current self-tail-call-eliminated function.
    current_tco_loop_label_chirho: Option<String>,
    /// Mutable parameter slots used to rebind arguments on self-tail jumps.
    current_tco_param_slots_chirho: HashMap<CoreIdChirho, String>,
    /// Original parameter order for self-tail-call slot rebinding.
    current_tco_param_order_chirho: Vec<CoreIdChirho>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ShowBuiltinKindChirho {
    IntChirho,
    BoolChirho,
    OrderingChirho,
    CharChirho,
    DoubleChirho,
    ListChirho(Box<ShowBuiltinKindChirho>),
}

enum TailCompileOutcomeChirho {
    ValueChirho(String),
    TerminatedChirho,
}

impl LlvmCodegenChirho {
    pub fn new_chirho() -> Self {
        Self {
            output_chirho: String::new(),
            unsupported_primitives_chirho: HashSet::new(),
            global_defs_chirho: String::new(),
            next_tmp_chirho: 0,
            next_label_chirho: 0,
            next_string_chirho: 0,
            next_lifted_function_chirho: 0,
            lifted_functions_chirho: Vec::new(),
            emitted_names_chirho: HashSet::new(),
            string_globals_chirho: HashMap::new(),
            toplevel_names_chirho: HashMap::new(),
            resolved_names_chirho: HashMap::new(),
            toplevel_arities_chirho: HashMap::new(),
            toplevel_value_kinds_chirho: HashMap::new(),
            local_scope_chirho: HashSet::new(),
            local_value_kinds_chirho: HashMap::new(),
            local_value_names_chirho: HashMap::new(),
            lifted_local_names_chirho: HashMap::new(),
            lifted_local_arities_chirho: HashMap::new(),
            lifted_local_capture_ids_chirho: HashMap::new(),
            current_tco_self_id_chirho: None,
            current_tco_loop_label_chirho: None,
            current_tco_param_slots_chirho: HashMap::new(),
            current_tco_param_order_chirho: Vec::new(),
        }
    }

    /// Generate a fresh LLVM temporary name.
    fn fresh_tmp_chirho(&mut self) -> String {
        let tmp_chirho = format!("%t{}", self.next_tmp_chirho);
        self.next_tmp_chirho += 1;
        tmp_chirho
    }

    fn emit_floor_div_mod_chirho(
        &mut self,
        lhs_chirho: &str,
        rhs_chirho: &str,
        want_mod_chirho: bool,
    ) -> String {
        let quot_chirho = self.fresh_tmp_chirho();
        let rem_chirho = self.fresh_tmp_chirho();
        let rem_nonzero_chirho = self.fresh_tmp_chirho();
        let rem_negative_chirho = self.fresh_tmp_chirho();
        let rhs_negative_chirho = self.fresh_tmp_chirho();
        let signs_differ_chirho = self.fresh_tmp_chirho();
        let adjust_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {quot_chirho} = sdiv i64 {lhs_chirho}, {rhs_chirho}"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {rem_chirho} = srem i64 {lhs_chirho}, {rhs_chirho}"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {rem_nonzero_chirho} = icmp ne i64 {rem_chirho}, 0"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {rem_negative_chirho} = icmp slt i64 {rem_chirho}, 0"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {rhs_negative_chirho} = icmp slt i64 {rhs_chirho}, 0"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {signs_differ_chirho} = xor i1 {rem_negative_chirho}, {rhs_negative_chirho}"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {adjust_chirho} = and i1 {rem_nonzero_chirho}, {signs_differ_chirho}"
        )
        .unwrap();

        if want_mod_chirho {
            let adjusted_rem_chirho = self.fresh_tmp_chirho();
            let result_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {adjusted_rem_chirho} = add i64 {rem_chirho}, {rhs_chirho}"
            )
            .unwrap();
            writeln!(
                self.output_chirho,
                "  {result_chirho} = select i1 {adjust_chirho}, i64 {adjusted_rem_chirho}, i64 {rem_chirho}"
            )
            .unwrap();
            result_chirho
        } else {
            let adjusted_quot_chirho = self.fresh_tmp_chirho();
            let result_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {adjusted_quot_chirho} = sub i64 {quot_chirho}, 1"
            )
            .unwrap();
            writeln!(
                self.output_chirho,
                "  {result_chirho} = select i1 {adjust_chirho}, i64 {adjusted_quot_chirho}, i64 {quot_chirho}"
            )
            .unwrap();
            result_chirho
        }
    }

    /// Generate a fresh LLVM label.
    fn fresh_label_chirho(&mut self, prefix_chirho: &str) -> String {
        let label_chirho = format!("{prefix_chirho}.{}", self.next_label_chirho);
        self.next_label_chirho += 1;
        label_chirho
    }

    /// Force a value through enter_thunk. Returns the forced value.
    /// enter_thunk returns non-thunks as-is after checking the low boxed tag.
    fn emit_force_thunk_chirho(&mut self, value_chirho: &str) -> String {
        let tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {tmp_chirho} = call i64 @haskelujah_enter_thunk_chirho(i64 {value_chirho})"
        )
        .unwrap();
        tmp_chirho
    }

    /// Try to create a lazy thunk for a function application expression.
    /// Returns Some(thunk_val) if the expression is an App chain with a
    /// known top-level function head; None otherwise.
    fn try_create_thunk_for_app_chirho(&mut self, expr_chirho: &CoreExprChirho) -> Option<String> {
        // Collect the application chain: head + args
        let (head_chirho, args_chirho) = collect_app_chain_llvm_chirho(expr_chirho);
        if args_chirho.is_empty() || args_chirho.len() > 8 {
            return None;
        }
        // Head must be a known top-level function
        let func_id_chirho = match head_chirho {
            CoreExprChirho::VarChirho(id_chirho) => *id_chirho,
            _ => return None,
        };
        let func_name_chirho = self.toplevel_names_chirho.get(&func_id_chirho)?;
        let func_arity_chirho = self
            .toplevel_arities_chirho
            .get(&func_id_chirho)
            .copied()
            .unwrap_or(0);
        if args_chirho.len() != func_arity_chirho {
            return None;
        }
        let mangled_chirho = mangle_name_chirho(func_name_chirho);

        // Get the function address
        let fn_addr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {fn_addr_tmp_chirho} = ptrtoint ptr @{mangled_chirho} to i64"
        )
        .unwrap();

        // Capturing an argument does not demand it.
        let mut fv_vals_chirho = Vec::with_capacity(args_chirho.len() + 1);
        fv_vals_chirho.push(fn_addr_tmp_chirho.clone());
        for arg_chirho in &args_chirho {
            let arg_val_chirho = self.compile_lazy_value_chirho(arg_chirho);
            fv_vals_chirho.push(arg_val_chirho);
        }

        // Get trampoline address
        let trampoline_name_chirho =
            format!("haskelujah_thunk_trampoline_{}_chirho", args_chirho.len());
        let tramp_addr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {tramp_addr_tmp_chirho} = ptrtoint ptr @{trampoline_name_chirho} to i64"
        )
        .unwrap();

        // Stack-allocate fvs array
        let num_fvs_chirho = fv_vals_chirho.len();
        let fvs_alloca_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {fvs_alloca_chirho} = alloca i64, i64 {num_fvs_chirho}"
        )
        .unwrap();

        // Store each free variable
        for (idx_chirho, fv_val_chirho) in fv_vals_chirho.iter().enumerate() {
            let fv_ptr_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {fv_ptr_chirho} = getelementptr i64, ptr {fvs_alloca_chirho}, i64 {idx_chirho}"
            )
            .unwrap();
            writeln!(
                self.output_chirho,
                "  store i64 {fv_val_chirho}, ptr {fv_ptr_chirho}"
            )
            .unwrap();
        }

        // Call alloc_thunk(trampoline_addr, num_fvs, fvs_ptr)
        let thunk_val_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {thunk_val_chirho} = call i64 @haskelujah_alloc_thunk_chirho(i64 {tramp_addr_tmp_chirho}, i64 {num_fvs_chirho}, ptr {fvs_alloca_chirho})"
        )
        .unwrap();

        // Push as GC root
        self.emit_gc_root_push_i64_chirho(&thunk_val_chirho);

        Some(thunk_val_chirho)
    }

    fn emit_gc_root_push_i64_chirho(&mut self, value_chirho: &str) {
        let root_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {root_ptr_tmp_chirho} = inttoptr i64 {value_chirho} to ptr"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  call void @haskelujah_gc_root_push_chirho(ptr {root_ptr_tmp_chirho})"
        )
        .unwrap();
    }

    // Native LLVM currently uses conservative process-lifetime roots for
    // produced heap values; popping here can race callee-installed roots.
    fn emit_gc_root_push_heap_i64_chirho(&mut self, value_chirho: &str) {
        let is_heap_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {is_heap_tmp_chirho} = call i64 @haskelujah_is_heap_ptr_chirho(i64 {value_chirho})"
        )
        .unwrap();
        let should_push_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {should_push_tmp_chirho} = icmp ne i64 {is_heap_tmp_chirho}, 0"
        )
        .unwrap();
        let push_label_chirho = self.fresh_label_chirho("gc.root.push");
        let done_label_chirho = self.fresh_label_chirho("gc.root.done");
        writeln!(
            self.output_chirho,
            "  br i1 {should_push_tmp_chirho}, label %{push_label_chirho}, label %{done_label_chirho}"
        )
        .unwrap();
        writeln!(self.output_chirho, "{push_label_chirho}:").unwrap();
        self.emit_gc_root_push_i64_chirho(value_chirho);
        writeln!(self.output_chirho, "  br label %{done_label_chirho}").unwrap();
        writeln!(self.output_chirho, "{done_label_chirho}:").unwrap();
    }

    fn emit_gc_root_pop_count_chirho(&mut self, count_chirho: usize) {
        for _ in 0..count_chirho {
            writeln!(
                self.output_chirho,
                "  call void @haskelujah_gc_root_pop_chirho()"
            )
            .unwrap();
        }
    }

    fn remember_local_binder_chirho(&mut self, binder_chirho: &BinderChirho) {
        self.local_scope_chirho.insert(binder_chirho.id_chirho);
        if let Some(value_kind_chirho) = classify_basic_ty_chirho(&binder_chirho.ty_chirho) {
            self.local_value_kinds_chirho
                .insert(binder_chirho.id_chirho, value_kind_chirho);
        } else {
            self.local_value_kinds_chirho
                .remove(&binder_chirho.id_chirho);
        }
    }

    fn remember_local_alias_chirho(
        &mut self,
        alias_id_chirho: CoreIdChirho,
        source_id_chirho: CoreIdChirho,
    ) {
        self.local_scope_chirho.insert(alias_id_chirho);
        if let Some(value_kind_chirho) = self.local_value_kinds_chirho.get(&source_id_chirho) {
            self.local_value_kinds_chirho
                .insert(alias_id_chirho, value_kind_chirho.clone());
        } else {
            self.local_value_kinds_chirho.remove(&alias_id_chirho);
        }
    }

    fn bind_local_value_name_chirho(&mut self, binder_chirho: &BinderChirho, value_chirho: &str) {
        self.remember_local_binder_chirho(binder_chirho);
        let alias_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {alias_tmp_chirho} = add i64 0, {value_chirho}"
        )
        .unwrap();
        self.local_value_names_chirho
            .insert(binder_chirho.id_chirho, alias_tmp_chirho);
    }

    fn should_entry_alias_free_id_chirho(&self, id_chirho: CoreIdChirho) -> bool {
        if self.toplevel_names_chirho.contains_key(&id_chirho)
            || self.lifted_local_names_chirho.contains_key(&id_chirho)
        {
            return false;
        }

        self.resolved_names_chirho
            .get(&id_chirho)
            .is_none_or(|name_chirho| builtin_runtime_arity_by_name_chirho(name_chirho).is_none())
    }

    fn collect_free_runtime_var_ids_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
    ) -> Vec<CoreIdChirho> {
        let mut bound_ids_chirho = HashSet::new();
        let mut seen_ids_chirho = HashSet::new();
        let mut free_ids_chirho = Vec::new();
        self.collect_free_runtime_var_ids_inner_chirho(
            expr_chirho,
            &mut bound_ids_chirho,
            &mut seen_ids_chirho,
            &mut free_ids_chirho,
        );
        free_ids_chirho
    }

    fn collect_free_runtime_var_ids_inner_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        bound_ids_chirho: &mut HashSet<CoreIdChirho>,
        seen_ids_chirho: &mut HashSet<CoreIdChirho>,
        free_ids_chirho: &mut Vec<CoreIdChirho>,
    ) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                if !bound_ids_chirho.contains(id_chirho)
                    && self.should_entry_alias_free_id_chirho(*id_chirho)
                    && seen_ids_chirho.insert(*id_chirho)
                {
                    free_ids_chirho.push(*id_chirho);
                }
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                self.collect_free_runtime_var_ids_inner_chirho(
                    fun_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    free_ids_chirho,
                );
                self.collect_free_runtime_var_ids_inner_chirho(
                    arg_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    free_ids_chirho,
                );
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                let inserted_chirho = bound_ids_chirho.insert(binder_chirho.id_chirho);
                self.collect_free_runtime_var_ids_inner_chirho(
                    body_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    free_ids_chirho,
                );
                if inserted_chirho {
                    bound_ids_chirho.remove(&binder_chirho.id_chirho);
                }
            }
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => {
                let let_ids_chirho: Vec<CoreIdChirho> = binds_chirho
                    .iter()
                    .map(|(binder_chirho, _)| binder_chirho.id_chirho)
                    .collect();
                if *rec_chirho {
                    for let_id_chirho in &let_ids_chirho {
                        bound_ids_chirho.insert(*let_id_chirho);
                    }
                    for (_, rhs_chirho) in binds_chirho {
                        self.collect_free_runtime_var_ids_inner_chirho(
                            rhs_chirho,
                            bound_ids_chirho,
                            seen_ids_chirho,
                            free_ids_chirho,
                        );
                    }
                    self.collect_free_runtime_var_ids_inner_chirho(
                        body_chirho,
                        bound_ids_chirho,
                        seen_ids_chirho,
                        free_ids_chirho,
                    );
                    for let_id_chirho in &let_ids_chirho {
                        bound_ids_chirho.remove(let_id_chirho);
                    }
                } else {
                    for (_, rhs_chirho) in binds_chirho {
                        self.collect_free_runtime_var_ids_inner_chirho(
                            rhs_chirho,
                            bound_ids_chirho,
                            seen_ids_chirho,
                            free_ids_chirho,
                        );
                    }
                    for let_id_chirho in &let_ids_chirho {
                        bound_ids_chirho.insert(*let_id_chirho);
                    }
                    self.collect_free_runtime_var_ids_inner_chirho(
                        body_chirho,
                        bound_ids_chirho,
                        seen_ids_chirho,
                        free_ids_chirho,
                    );
                    for let_id_chirho in &let_ids_chirho {
                        bound_ids_chirho.remove(let_id_chirho);
                    }
                }
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                self.collect_free_runtime_var_ids_inner_chirho(
                    scrutinee_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    free_ids_chirho,
                );
                let bind_inserted_chirho = bound_ids_chirho.insert(bind_chirho.id_chirho);
                for alt_chirho in alts_chirho {
                    let mut inserted_ids_chirho = Vec::new();
                    for binder_chirho in &alt_chirho.binders_chirho {
                        if bound_ids_chirho.insert(binder_chirho.id_chirho) {
                            inserted_ids_chirho.push(binder_chirho.id_chirho);
                        }
                    }
                    self.collect_free_runtime_var_ids_inner_chirho(
                        &alt_chirho.rhs_chirho,
                        bound_ids_chirho,
                        seen_ids_chirho,
                        free_ids_chirho,
                    );
                    for inserted_id_chirho in inserted_ids_chirho {
                        bound_ids_chirho.remove(&inserted_id_chirho);
                    }
                }
                if bind_inserted_chirho {
                    bound_ids_chirho.remove(&bind_chirho.id_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                self.collect_free_runtime_var_ids_inner_chirho(
                    body_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    free_ids_chirho,
                );
            }
            CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
                self.collect_free_runtime_var_ids_inner_chirho(
                    expr_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    free_ids_chirho,
                );
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    self.collect_free_runtime_var_ids_inner_chirho(
                        arg_chirho,
                        bound_ids_chirho,
                        seen_ids_chirho,
                        free_ids_chirho,
                    );
                }
            }
        }
    }

    fn emit_missing_param_aliases_chirho(
        &mut self,
        param_binders_chirho: &[&BinderChirho],
        body_chirho: &CoreExprChirho,
        capture_ids_chirho: &[CoreIdChirho],
    ) {
        let free_ids_chirho = self.collect_free_runtime_var_ids_chirho(body_chirho);
        let free_id_set_chirho: HashSet<CoreIdChirho> = free_ids_chirho.iter().copied().collect();
        let param_ids_chirho: HashSet<CoreIdChirho> = param_binders_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.id_chirho)
            .collect();

        let unresolved_ids_chirho: Vec<CoreIdChirho> = free_ids_chirho
            .iter()
            .copied()
            .filter(|id_chirho| {
                !param_ids_chirho.contains(id_chirho) && !capture_ids_chirho.contains(id_chirho)
            })
            .collect();
        if unresolved_ids_chirho.is_empty() {
            return;
        }

        let candidate_param_ids_chirho: Vec<CoreIdChirho> = param_binders_chirho
            .iter()
            .filter(|binder_chirho| !is_dictionary_param_name_chirho(&binder_chirho.name_chirho))
            .map(|binder_chirho| binder_chirho.id_chirho)
            .filter(|id_chirho| !free_id_set_chirho.contains(id_chirho))
            .collect();
        if candidate_param_ids_chirho.is_empty() {
            return;
        }

        let alias_pairs_chirho: Vec<(CoreIdChirho, CoreIdChirho)> =
            if candidate_param_ids_chirho.len() == 1 {
                unresolved_ids_chirho
                    .iter()
                    .copied()
                    .map(|alias_id_chirho| (alias_id_chirho, candidate_param_ids_chirho[0]))
                    .collect()
            } else if unresolved_ids_chirho.len() <= candidate_param_ids_chirho.len() {
                unresolved_ids_chirho
                    .iter()
                    .copied()
                    .zip(candidate_param_ids_chirho.iter().copied())
                    .collect()
            } else {
                Vec::new()
            };
        if alias_pairs_chirho.is_empty() {
            return;
        }

        for (alias_id_chirho, source_id_chirho) in alias_pairs_chirho {
            self.remember_local_alias_chirho(alias_id_chirho, source_id_chirho);
            writeln!(
                self.output_chirho,
                "  %v{} = add i64 0, %v{}",
                alias_id_chirho.0, source_id_chirho.0
            )
            .unwrap();
        }
    }

    fn collect_captured_local_ids_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        excluded_ids_chirho: &HashSet<CoreIdChirho>,
    ) -> Vec<CoreIdChirho> {
        let mut bound_ids_chirho = excluded_ids_chirho.clone();
        let mut seen_ids_chirho = HashSet::new();
        let mut captured_ids_chirho = Vec::new();
        let mut visiting_lifted_ids_chirho = HashSet::new();
        self.collect_captured_local_ids_inner_chirho(
            expr_chirho,
            &mut bound_ids_chirho,
            &mut seen_ids_chirho,
            &mut captured_ids_chirho,
            &mut visiting_lifted_ids_chirho,
        );
        captured_ids_chirho
    }

    fn collect_capture_dependency_id_chirho(
        &self,
        id_chirho: CoreIdChirho,
        bound_ids_chirho: &HashSet<CoreIdChirho>,
        seen_ids_chirho: &mut HashSet<CoreIdChirho>,
        captured_ids_chirho: &mut Vec<CoreIdChirho>,
        visiting_lifted_ids_chirho: &mut HashSet<CoreIdChirho>,
    ) {
        if self.local_scope_chirho.contains(&id_chirho) && !bound_ids_chirho.contains(&id_chirho) {
            if seen_ids_chirho.insert(id_chirho) {
                captured_ids_chirho.push(id_chirho);
            }
            return;
        }

        let Some(lifted_capture_ids_chirho) = self
            .lifted_local_capture_ids_chirho
            .get(&id_chirho)
            .cloned()
        else {
            return;
        };

        if !visiting_lifted_ids_chirho.insert(id_chirho) {
            return;
        }
        for lifted_capture_id_chirho in lifted_capture_ids_chirho {
            self.collect_capture_dependency_id_chirho(
                lifted_capture_id_chirho,
                bound_ids_chirho,
                seen_ids_chirho,
                captured_ids_chirho,
                visiting_lifted_ids_chirho,
            );
        }
        visiting_lifted_ids_chirho.remove(&id_chirho);
    }

    fn collect_captured_local_ids_inner_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        bound_ids_chirho: &mut HashSet<CoreIdChirho>,
        seen_ids_chirho: &mut HashSet<CoreIdChirho>,
        captured_ids_chirho: &mut Vec<CoreIdChirho>,
        visiting_lifted_ids_chirho: &mut HashSet<CoreIdChirho>,
    ) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                self.collect_capture_dependency_id_chirho(
                    *id_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    captured_ids_chirho,
                    visiting_lifted_ids_chirho,
                );
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                self.collect_captured_local_ids_inner_chirho(
                    fun_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    captured_ids_chirho,
                    visiting_lifted_ids_chirho,
                );
                self.collect_captured_local_ids_inner_chirho(
                    arg_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    captured_ids_chirho,
                    visiting_lifted_ids_chirho,
                );
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                let inserted_chirho = bound_ids_chirho.insert(binder_chirho.id_chirho);
                self.collect_captured_local_ids_inner_chirho(
                    body_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    captured_ids_chirho,
                    visiting_lifted_ids_chirho,
                );
                if inserted_chirho {
                    bound_ids_chirho.remove(&binder_chirho.id_chirho);
                }
            }
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => {
                let let_ids_chirho: Vec<CoreIdChirho> = binds_chirho
                    .iter()
                    .map(|(binder_chirho, _)| binder_chirho.id_chirho)
                    .collect();
                if *rec_chirho {
                    for let_id_chirho in &let_ids_chirho {
                        bound_ids_chirho.insert(*let_id_chirho);
                    }
                    for (_, rhs_chirho) in binds_chirho {
                        self.collect_captured_local_ids_inner_chirho(
                            rhs_chirho,
                            bound_ids_chirho,
                            seen_ids_chirho,
                            captured_ids_chirho,
                            visiting_lifted_ids_chirho,
                        );
                    }
                    self.collect_captured_local_ids_inner_chirho(
                        body_chirho,
                        bound_ids_chirho,
                        seen_ids_chirho,
                        captured_ids_chirho,
                        visiting_lifted_ids_chirho,
                    );
                    for let_id_chirho in &let_ids_chirho {
                        bound_ids_chirho.remove(let_id_chirho);
                    }
                } else {
                    for (_, rhs_chirho) in binds_chirho {
                        self.collect_captured_local_ids_inner_chirho(
                            rhs_chirho,
                            bound_ids_chirho,
                            seen_ids_chirho,
                            captured_ids_chirho,
                            visiting_lifted_ids_chirho,
                        );
                    }
                    for let_id_chirho in &let_ids_chirho {
                        bound_ids_chirho.insert(*let_id_chirho);
                    }
                    self.collect_captured_local_ids_inner_chirho(
                        body_chirho,
                        bound_ids_chirho,
                        seen_ids_chirho,
                        captured_ids_chirho,
                        visiting_lifted_ids_chirho,
                    );
                    for let_id_chirho in &let_ids_chirho {
                        bound_ids_chirho.remove(let_id_chirho);
                    }
                }
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                self.collect_captured_local_ids_inner_chirho(
                    scrutinee_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    captured_ids_chirho,
                    visiting_lifted_ids_chirho,
                );
                let bind_inserted_chirho = bound_ids_chirho.insert(bind_chirho.id_chirho);
                for alt_chirho in alts_chirho {
                    let mut alt_inserted_ids_chirho = Vec::new();
                    for binder_chirho in &alt_chirho.binders_chirho {
                        if bound_ids_chirho.insert(binder_chirho.id_chirho) {
                            alt_inserted_ids_chirho.push(binder_chirho.id_chirho);
                        }
                    }
                    self.collect_captured_local_ids_inner_chirho(
                        &alt_chirho.rhs_chirho,
                        bound_ids_chirho,
                        seen_ids_chirho,
                        captured_ids_chirho,
                        visiting_lifted_ids_chirho,
                    );
                    for alt_id_chirho in alt_inserted_ids_chirho {
                        bound_ids_chirho.remove(&alt_id_chirho);
                    }
                }
                if bind_inserted_chirho {
                    bound_ids_chirho.remove(&bind_chirho.id_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                self.collect_captured_local_ids_inner_chirho(
                    body_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    captured_ids_chirho,
                    visiting_lifted_ids_chirho,
                );
            }
            CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
                self.collect_captured_local_ids_inner_chirho(
                    expr_chirho,
                    bound_ids_chirho,
                    seen_ids_chirho,
                    captured_ids_chirho,
                    visiting_lifted_ids_chirho,
                );
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    self.collect_captured_local_ids_inner_chirho(
                        arg_chirho,
                        bound_ids_chirho,
                        seen_ids_chirho,
                        captured_ids_chirho,
                        visiting_lifted_ids_chirho,
                    );
                }
            }
        }
    }

    fn fresh_lifted_name_chirho(&mut self) -> String {
        let lifted_name_chirho = format!("haskelujah_lambda_{}", self.next_lifted_function_chirho);
        self.next_lifted_function_chirho += 1;
        lifted_name_chirho
    }

    fn compile_lifted_lambda_value_chirho(
        &mut self,
        lifted_name_chirho: &str,
        expr_chirho: &CoreExprChirho,
        excluded_capture_ids_chirho: &HashSet<CoreIdChirho>,
    ) -> String {
        let (param_binders_chirho, body_chirho) = collect_lambda_binders_chirho(expr_chirho);
        let captured_ids_chirho =
            self.collect_captured_local_ids_chirho(expr_chirho, excluded_capture_ids_chirho);
        let self_binder_id_chirho =
            self.lifted_local_names_chirho
                .iter()
                .find_map(|(id_chirho, name_chirho)| {
                    if name_chirho == lifted_name_chirho {
                        Some(*id_chirho)
                    } else {
                        None
                    }
                });
        let mut params_vec_chirho = param_binders_chirho
            .iter()
            .map(|binder_chirho| format!("i64 %v{}.entry_chirho", binder_chirho.id_chirho.0))
            .collect::<Vec<_>>();
        if !captured_ids_chirho.is_empty() {
            params_vec_chirho.insert(0, "i64 %env_chirho".to_string());
        }
        let params_str_chirho = params_vec_chirho.join(", ");

        let mut inner_codegen_chirho = LlvmCodegenChirho::new_chirho();
        inner_codegen_chirho.next_string_chirho = self.next_string_chirho;
        inner_codegen_chirho.next_lifted_function_chirho = self.next_lifted_function_chirho;
        inner_codegen_chirho.string_globals_chirho = self.string_globals_chirho.clone();
        inner_codegen_chirho.toplevel_names_chirho = self.toplevel_names_chirho.clone();
        inner_codegen_chirho.resolved_names_chirho = self.resolved_names_chirho.clone();
        inner_codegen_chirho.toplevel_arities_chirho = self.toplevel_arities_chirho.clone();
        inner_codegen_chirho.toplevel_value_kinds_chirho = self.toplevel_value_kinds_chirho.clone();
        inner_codegen_chirho.local_value_kinds_chirho = self.local_value_kinds_chirho.clone();
        inner_codegen_chirho.lifted_local_names_chirho = self.lifted_local_names_chirho.clone();
        inner_codegen_chirho.lifted_local_arities_chirho = self.lifted_local_arities_chirho.clone();
        inner_codegen_chirho.lifted_local_capture_ids_chirho =
            self.lifted_local_capture_ids_chirho.clone();
        for binder_chirho in &param_binders_chirho {
            inner_codegen_chirho.remember_local_binder_chirho(binder_chirho);
        }

        writeln!(
            inner_codegen_chirho.output_chirho,
            "define i64 @{lifted_name_chirho}({params_str_chirho}) {{"
        )
        .unwrap();
        writeln!(inner_codegen_chirho.output_chirho, "entry:").unwrap();
        inner_codegen_chirho.next_tmp_chirho = 0;
        if !captured_ids_chirho.is_empty() {
            if let Some(self_id_chirho) = self_binder_id_chirho {
                inner_codegen_chirho
                    .local_scope_chirho
                    .insert(self_id_chirho);
                writeln!(
                    inner_codegen_chirho.output_chirho,
                    "  %v{} = add i64 0, %env_chirho",
                    self_id_chirho.0
                )
                .unwrap();
            }
        }
        for (capture_idx_chirho, capture_id_chirho) in captured_ids_chirho.iter().enumerate() {
            inner_codegen_chirho
                .local_scope_chirho
                .insert(*capture_id_chirho);
            let capture_value_tmp_chirho = inner_codegen_chirho
                .load_boxed_constructor_field_chirho("%env_chirho", capture_idx_chirho + 1);
            writeln!(
                inner_codegen_chirho.output_chirho,
                "  %v{} = add i64 0, {capture_value_tmp_chirho}",
                capture_id_chirho.0
            )
            .unwrap();
        }

        let loop_label_chirho = inner_codegen_chirho.fresh_label_chirho("tail.loop");
        let mut param_slots_chirho = HashMap::new();
        for binder_chirho in &param_binders_chirho {
            let slot_tmp_chirho = inner_codegen_chirho.fresh_tmp_chirho();
            writeln!(
                inner_codegen_chirho.output_chirho,
                "  {slot_tmp_chirho} = alloca i64"
            )
            .unwrap();
            writeln!(
                inner_codegen_chirho.output_chirho,
                "  store i64 %v{}.entry_chirho, ptr {slot_tmp_chirho}",
                binder_chirho.id_chirho.0
            )
            .unwrap();
            param_slots_chirho.insert(binder_chirho.id_chirho, slot_tmp_chirho);
        }
        writeln!(
            inner_codegen_chirho.output_chirho,
            "  br label %{loop_label_chirho}"
        )
        .unwrap();
        writeln!(inner_codegen_chirho.output_chirho, "{loop_label_chirho}:").unwrap();
        for binder_chirho in &param_binders_chirho {
            let slot_tmp_chirho = param_slots_chirho
                .get(&binder_chirho.id_chirho)
                .expect("lifted lambda param slot");
            writeln!(
                inner_codegen_chirho.output_chirho,
                "  %v{} = load i64, ptr {slot_tmp_chirho}",
                binder_chirho.id_chirho.0
            )
            .unwrap();
        }
        inner_codegen_chirho.emit_missing_param_aliases_chirho(
            &param_binders_chirho,
            body_chirho,
            &captured_ids_chirho,
        );

        inner_codegen_chirho.current_tco_self_id_chirho = self_binder_id_chirho;
        inner_codegen_chirho.current_tco_loop_label_chirho = Some(loop_label_chirho);
        inner_codegen_chirho.current_tco_param_slots_chirho = param_slots_chirho;
        inner_codegen_chirho.current_tco_param_order_chirho = param_binders_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.id_chirho)
            .collect();

        match inner_codegen_chirho.compile_tail_expr_chirho(body_chirho) {
            TailCompileOutcomeChirho::ValueChirho(result_chirho) => {
                writeln!(
                    inner_codegen_chirho.output_chirho,
                    "  ret i64 {result_chirho}"
                )
                .unwrap();
            }
            TailCompileOutcomeChirho::TerminatedChirho => {}
        }
        writeln!(inner_codegen_chirho.output_chirho, "}}").unwrap();

        self.next_string_chirho = inner_codegen_chirho.next_string_chirho;
        self.unsupported_primitives_chirho
            .extend(inner_codegen_chirho.unsupported_primitives_chirho);
        self.next_lifted_function_chirho = inner_codegen_chirho.next_lifted_function_chirho;
        self.string_globals_chirho = inner_codegen_chirho.string_globals_chirho.clone();
        self.global_defs_chirho
            .push_str(&inner_codegen_chirho.global_defs_chirho);
        self.lifted_functions_chirho
            .extend(inner_codegen_chirho.lifted_functions_chirho.clone());
        self.lifted_functions_chirho
            .push(inner_codegen_chirho.output_chirho);

        if captured_ids_chirho.is_empty() {
            let tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {tmp_chirho} = ptrtoint ptr @{lifted_name_chirho} to i64"
            )
            .unwrap();
            return tmp_chirho;
        }

        self.emit_closure_value_chirho(lifted_name_chirho, &captured_ids_chirho)
    }

    fn classify_expr_show_kind_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
    ) -> Option<ShowBuiltinKindChirho> {
        match expr_chirho {
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(_)) => {
                Some(ShowBuiltinKindChirho::IntChirho)
            }
            CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(_)) => {
                Some(ShowBuiltinKindChirho::DoubleChirho)
            }
            CoreExprChirho::LitChirho(CoreLitChirho::CharChirho(_)) => {
                Some(ShowBuiltinKindChirho::CharChirho)
            }
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho: _,
            } if matches!(
                name_chirho.as_str(),
                "==#" | "/=#" | "<#" | "<=#" | ">#" | ">=#" | "not#"
            ) =>
            {
                Some(ShowBuiltinKindChirho::BoolChirho)
            }
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho: _,
            } if matches!(
                name_chirho.as_str(),
                "compare#" | "compareChar#" | "compareFloat#" | "compareStr#"
            ) =>
            {
                Some(ShowBuiltinKindChirho::OrderingChirho)
            }
            CoreExprChirho::AppChirho {
                fun_chirho: _,
                arg_chirho: _,
            } => {
                let (callee_chirho, args_chirho) = flatten_app_chirho(expr_chirho);
                match strip_runtime_tyapps_chirho(callee_chirho) {
                    CoreExprChirho::VarChirho(id_chirho) => self
                        .toplevel_names_chirho
                        .get(id_chirho)
                        .and_then(
                            |name_chirho| match (name_chirho.as_str(), args_chirho.len()) {
                                (name_chirho, 2) if name_chirho.starts_with("$prim_Eq_==_") => {
                                    Some(ShowBuiltinKindChirho::BoolChirho)
                                }
                                ("not", 1)
                                | ("not#", 1)
                                | ("==", 2)
                                | ("/=", 2)
                                | ("<", 2)
                                | ("<=", 2)
                                | (">", 2)
                                | (">=", 2) => Some(ShowBuiltinKindChirho::BoolChirho),
                                (name_chirho, 2)
                                    if name_chirho.starts_with("$prim_Ord_compare_") =>
                                {
                                    Some(ShowBuiltinKindChirho::OrderingChirho)
                                }
                                ("compare", 2)
                                | ("$sel_Ord_compare", 2)
                                | ("$sel_Ord_compare", 3) => {
                                    Some(ShowBuiltinKindChirho::OrderingChirho)
                                }
                                _ => None,
                            },
                        ),
                    _ => None,
                }
            }
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } => match (con_name_chirho.as_str(), args_chirho.as_slice()) {
                ("True", []) | ("False", []) => Some(ShowBuiltinKindChirho::BoolChirho),
                ("LT", []) | ("EQ", []) | ("GT", []) => Some(ShowBuiltinKindChirho::OrderingChirho),
                (":", [head_expr_chirho, tail_expr_chirho]) => {
                    let head_kind_chirho = self
                        .classify_expr_show_kind_chirho(head_expr_chirho)
                        .unwrap_or(ShowBuiltinKindChirho::IntChirho);
                    match self.classify_expr_show_kind_chirho(tail_expr_chirho) {
                        Some(ShowBuiltinKindChirho::ListChirho(tail_kind_chirho))
                            if *tail_kind_chirho == head_kind_chirho =>
                        {
                            Some(ShowBuiltinKindChirho::ListChirho(Box::new(
                                head_kind_chirho,
                            )))
                        }
                        None if matches!(
                            tail_expr_chirho,
                            CoreExprChirho::ConAppChirho {
                                con_name_chirho,
                                args_chirho,
                            } if con_name_chirho == "[]" && args_chirho.is_empty()
                        ) =>
                        {
                            Some(ShowBuiltinKindChirho::ListChirho(Box::new(
                                head_kind_chirho,
                            )))
                        }
                        _ => None,
                    }
                }
                _ => None,
            },
            CoreExprChirho::VarChirho(id_chirho) => self
                .local_value_kinds_chirho
                .get(id_chirho)
                .cloned()
                .or_else(|| self.toplevel_value_kinds_chirho.get(id_chirho).cloned())
                .or_else(|| {
                    self.toplevel_names_chirho
                        .get(id_chirho)
                        .and_then(|name_chirho| match name_chirho.as_str() {
                            "True" | "False" => Some(ShowBuiltinKindChirho::BoolChirho),
                            _ => None,
                        })
                }),
            CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
                self.classify_expr_show_kind_chirho(expr_chirho)
            }
            _ => None,
        }
    }

    fn emit_print_value_chirho(
        &mut self,
        arg_value_chirho: &str,
        show_kind_chirho: ShowBuiltinKindChirho,
    ) {
        let ptr_tmp_chirho = self.emit_show_value_ptr_chirho(arg_value_chirho, show_kind_chirho);
        writeln!(self.output_chirho, "  call i32 @puts(ptr {ptr_tmp_chirho})").unwrap();
    }

    fn emit_print_via_show_fallback_chirho(&mut self, arg_value_chirho: &str) {
        let forced_arg_value_chirho = self.emit_force_thunk_chirho(arg_value_chirho);
        let shown_i64_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {shown_i64_tmp_chirho} = call i64 @{}(i64 {forced_arg_value_chirho})",
            mangle_name_chirho("show")
        )
        .unwrap();
        let shown_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {shown_ptr_tmp_chirho} = inttoptr i64 {shown_i64_tmp_chirho} to ptr"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  call i32 @puts(ptr {shown_ptr_tmp_chirho})"
        )
        .unwrap();
    }

    fn emit_print_nullary_constructor_chirho(&mut self, con_name_chirho: &str) {
        let global_name_chirho = self.intern_string_global_name_chirho(con_name_chirho);
        writeln!(
            self.output_chirho,
            "  call i32 @puts(ptr @{global_name_chirho})"
        )
        .unwrap();
    }

    fn emit_show_expr_i64_direct_chirho(&mut self, expr_chirho: &CoreExprChirho) -> Option<String> {
        if let Some(show_kind_chirho) = self.classify_expr_show_kind_chirho(expr_chirho) {
            let arg_value_chirho = self.compile_expr_chirho(expr_chirho);
            let ptr_tmp_chirho =
                self.emit_show_value_ptr_chirho(&arg_value_chirho, show_kind_chirho);
            let ptr_i64_tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {ptr_i64_tmp_chirho} = ptrtoint ptr {ptr_tmp_chirho} to i64"
            )
            .unwrap();
            return Some(ptr_i64_tmp_chirho);
        }

        match strip_runtime_tyapps_chirho(expr_chirho) {
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } if !args_chirho.is_empty() => {
                self.emit_show_constructor_expr_i64_chirho(con_name_chirho, args_chirho)
            }
            _ => None,
        }
    }

    fn emit_show_constructor_expr_i64_chirho(
        &mut self,
        con_name_chirho: &str,
        args_chirho: &[CoreExprChirho],
    ) -> Option<String> {
        let con_global_name_chirho = self.intern_string_global_name_chirho(con_name_chirho);
        let con_i64_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {con_i64_tmp_chirho} = ptrtoint ptr @{con_global_name_chirho} to i64"
        )
        .unwrap();
        let space_global_name_chirho = self.intern_string_global_name_chirho(" ");
        let space_i64_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {space_i64_tmp_chirho} = ptrtoint ptr @{space_global_name_chirho} to i64"
        )
        .unwrap();

        let mut current_i64_tmp_chirho = con_i64_tmp_chirho;
        for arg_expr_chirho in args_chirho {
            let with_space_tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {with_space_tmp_chirho} = call i64 @haskelujah_append_str_chirho(i64 {current_i64_tmp_chirho}, i64 {space_i64_tmp_chirho})"
            )
            .unwrap();
            let shown_arg_i64_tmp_chirho =
                self.emit_show_expr_i64_direct_chirho(arg_expr_chirho)?;
            let appended_arg_tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {appended_arg_tmp_chirho} = call i64 @haskelujah_append_str_chirho(i64 {with_space_tmp_chirho}, i64 {shown_arg_i64_tmp_chirho})"
            )
            .unwrap();
            current_i64_tmp_chirho = appended_arg_tmp_chirho;
        }

        Some(current_i64_tmp_chirho)
    }

    fn emit_print_direct_constructor_expr_chirho(&mut self, expr_chirho: &CoreExprChirho) -> bool {
        let Some(shown_i64_tmp_chirho) = self.emit_show_expr_i64_direct_chirho(expr_chirho) else {
            return false;
        };
        let shown_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {shown_ptr_tmp_chirho} = inttoptr i64 {shown_i64_tmp_chirho} to ptr"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  call i32 @puts(ptr {shown_ptr_tmp_chirho})"
        )
        .unwrap();
        true
    }

    fn classify_nullary_constructor_print_name_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
    ) -> Option<String> {
        match strip_runtime_tyapps_chirho(expr_chirho) {
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } if args_chirho.is_empty() => Some(con_name_chirho.clone()),
            _ => None,
        }
    }

    fn resolve_builtin_runtime_function_chirho(
        &mut self,
        name_chirho: &str,
    ) -> Option<(String, usize)> {
        let arity_chirho = builtin_runtime_arity_by_name_chirho(name_chirho)?;
        let fn_name_chirho = mangle_name_chirho(name_chirho);
        if self.emitted_names_chirho.insert(fn_name_chirho.clone()) {
            let mut def_chirho = String::new();
            match name_chirho {
                "undefined" | "undefined#" => {
                    writeln!(def_chirho, "define i64 @{fn_name_chirho}() {{").unwrap();
                    writeln!(def_chirho, "entry:").unwrap();
                    writeln!(def_chirho, "  call void @abort()").unwrap();
                    writeln!(def_chirho, "  unreachable").unwrap();
                    writeln!(def_chirho, "}}").unwrap();
                    writeln!(def_chirho).unwrap();
                }
                "error" | "error#" => {
                    writeln!(def_chirho, "define i64 @{fn_name_chirho}(i64 %v0) {{").unwrap();
                    writeln!(def_chirho, "entry:").unwrap();
                    writeln!(def_chirho, "  %msg_ptr = inttoptr i64 %v0 to ptr").unwrap();
                    writeln!(def_chirho, "  call i32 @puts(ptr %msg_ptr)").unwrap();
                    writeln!(def_chirho, "  call void @abort()").unwrap();
                    writeln!(def_chirho, "  unreachable").unwrap();
                    writeln!(def_chirho, "}}").unwrap();
                    writeln!(def_chirho).unwrap();
                }
                "getLine" | "getLine#" => {
                    writeln!(def_chirho, "define i64 @{fn_name_chirho}() {{").unwrap();
                    writeln!(def_chirho, "entry:").unwrap();
                    writeln!(
                        def_chirho,
                        "  %line = call i64 @haskelujah_get_line_chirho()"
                    )
                    .unwrap();
                    writeln!(def_chirho, "  ret i64 %line").unwrap();
                    writeln!(def_chirho, "}}").unwrap();
                    writeln!(def_chirho).unwrap();
                }
                "readFile" | "readFile#" => {
                    writeln!(def_chirho, "define i64 @{fn_name_chirho}(i64 %v0) {{").unwrap();
                    writeln!(def_chirho, "entry:").unwrap();
                    writeln!(
                        def_chirho,
                        "  %contents = call i64 @haskelujah_read_file_chirho(i64 %v0)"
                    )
                    .unwrap();
                    writeln!(def_chirho, "  ret i64 %contents").unwrap();
                    writeln!(def_chirho, "}}").unwrap();
                    writeln!(def_chirho).unwrap();
                }
                "writeFile" | "writeFile#" => {
                    writeln!(
                        def_chirho,
                        "define i64 @{fn_name_chirho}(i64 %v0, i64 %v1) {{"
                    )
                    .unwrap();
                    writeln!(def_chirho, "entry:").unwrap();
                    writeln!(
                        def_chirho,
                        "  %status = call i64 @haskelujah_write_file_chirho(i64 %v0, i64 %v1)"
                    )
                    .unwrap();
                    writeln!(def_chirho, "  ret i64 %status").unwrap();
                    writeln!(def_chirho, "}}").unwrap();
                    writeln!(def_chirho).unwrap();
                }
                "+" => {
                    writeln!(
                        def_chirho,
                        "define i64 @{fn_name_chirho}(i64 %v0, i64 %v1) {{"
                    )
                    .unwrap();
                    writeln!(def_chirho, "entry:").unwrap();
                    writeln!(def_chirho, "  %result = add i64 %v0, %v1").unwrap();
                    writeln!(def_chirho, "  ret i64 %result").unwrap();
                    writeln!(def_chirho, "}}").unwrap();
                    writeln!(def_chirho).unwrap();
                }
                "-" => {
                    writeln!(
                        def_chirho,
                        "define i64 @{fn_name_chirho}(i64 %v0, i64 %v1) {{"
                    )
                    .unwrap();
                    writeln!(def_chirho, "entry:").unwrap();
                    writeln!(def_chirho, "  %result = sub i64 %v0, %v1").unwrap();
                    writeln!(def_chirho, "  ret i64 %result").unwrap();
                    writeln!(def_chirho, "}}").unwrap();
                    writeln!(def_chirho).unwrap();
                }
                "*" => {
                    writeln!(
                        def_chirho,
                        "define i64 @{fn_name_chirho}(i64 %v0, i64 %v1) {{"
                    )
                    .unwrap();
                    writeln!(def_chirho, "entry:").unwrap();
                    writeln!(def_chirho, "  %result = mul i64 %v0, %v1").unwrap();
                    writeln!(def_chirho, "  ret i64 %result").unwrap();
                    writeln!(def_chirho, "}}").unwrap();
                    writeln!(def_chirho).unwrap();
                }
                "return" | "pure" | "returnIO#" => {
                    writeln!(def_chirho, "define i64 @{fn_name_chirho}(i64 %v0) {{").unwrap();
                    writeln!(def_chirho, "entry:").unwrap();
                    writeln!(def_chirho, "  ret i64 %v0").unwrap();
                    writeln!(def_chirho, "}}").unwrap();
                    writeln!(def_chirho).unwrap();
                }
                ">>" | "thenIO#" => {
                    writeln!(
                        def_chirho,
                        "define i64 @{fn_name_chirho}(i64 %v0, i64 %v1) {{"
                    )
                    .unwrap();
                    writeln!(def_chirho, "entry:").unwrap();
                    writeln!(def_chirho, "  ret i64 %v1").unwrap();
                    writeln!(def_chirho, "}}").unwrap();
                    writeln!(def_chirho).unwrap();
                }
                ">>=" | "bindIO#" => {
                    writeln!(
                        def_chirho,
                        "define i64 @{fn_name_chirho}(i64 %v0, i64 %v1) {{"
                    )
                    .unwrap();
                    writeln!(def_chirho, "entry:").unwrap();
                    writeln!(def_chirho, "  %cont_ptr = inttoptr i64 %v1 to ptr").unwrap();
                    writeln!(def_chirho, "  %result = call i64 %cont_ptr(i64 %v0)").unwrap();
                    writeln!(def_chirho, "  ret i64 %result").unwrap();
                    writeln!(def_chirho, "}}").unwrap();
                    writeln!(def_chirho).unwrap();
                }
                _ => {}
            }
            self.lifted_functions_chirho.push(def_chirho);
        }
        Some((format!("@{fn_name_chirho}"), arity_chirho))
    }

    /// Compile a Core module to LLVM IR text.
    pub fn compile_module_chirho(&mut self, module_chirho: &CoreModuleChirho) -> String {
        let actions_chirho = haskelujah_core_chirho::io_actions_chirho::prepare_io_actions_chirho(
            module_chirho,
            "main",
        );
        let prepared_chirho =
            haskelujah_core_chirho::native_thunks_chirho::prepare_native_thunks_chirho(
                &actions_chirho,
            );
        let module_chirho = &prepared_chirho;
        self.output_chirho.clear();
        self.global_defs_chirho.clear();
        self.lifted_functions_chirho.clear();
        self.string_globals_chirho.clear();
        self.next_string_chirho = 0;
        self.next_lifted_function_chirho = 0;

        // Module header
        writeln!(
            self.output_chirho,
            "; ModuleID = '{}'",
            module_chirho.name_chirho
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "source_filename = \"{}.hs\"",
            module_chirho.name_chirho
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "target triple = \"{}\"",
            haskelujah_rts_chirho::target_chirho::native_target_chirho()
        )
        .unwrap();
        writeln!(self.output_chirho).unwrap();

        let has_toplevel_show_chirho = module_chirho.bindings_chirho.iter().any(|binding_chirho| {
            mangle_name_chirho(&binding_chirho.binder_chirho.name_chirho) == "haskelujah_show"
        });

        // Declare external functions we might call
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_negate_chirho(i64)"
        )
        .unwrap();
        writeln!(self.output_chirho, "declare i32 @puts(ptr)").unwrap();
        writeln!(self.output_chirho, "declare i32 @printf(ptr, ...)").unwrap();
        writeln!(
            self.output_chirho,
            "declare i32 @snprintf(ptr, i64, ptr, ...)"
        )
        .unwrap();
        writeln!(self.output_chirho, "declare i32 @putchar(i32)").unwrap();
        writeln!(
            self.output_chirho,
            "declare ptr @haskelujah_alloc_chirho(i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_alloc_thunk_chirho(i64, i64, ptr)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_enter_thunk_chirho(i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_is_heap_ptr_chirho(i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare void @haskelujah_update_thunk_chirho(i64, i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare void @haskelujah_gc_root_push_chirho(ptr)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare void @haskelujah_gc_root_pop_chirho()"
        )
        .unwrap();
        writeln!(self.output_chirho, "declare void @abort() noreturn").unwrap();
        writeln!(self.output_chirho, "declare i32 @strcmp(ptr, ptr)").unwrap();
        writeln!(self.output_chirho, "declare i64 @strlen(ptr)").unwrap();
        writeln!(
            self.output_chirho,
            "declare void @haskelujah_error_chirho(ptr, i64) noreturn"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "define internal i64 @raise_error_chirho(ptr %message_chirho) {{"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  %length_chirho = call i64 @strlen(ptr %message_chirho)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  call void @haskelujah_error_chirho(ptr %message_chirho, i64 %length_chirho)"
        )
        .unwrap();
        writeln!(self.output_chirho, "  unreachable\n}}").unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_show_int_chirho(i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_show_bool_chirho(i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_show_char_chirho(i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_show_float_chirho(i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_append_str_chirho(i64, i64)"
        )
        .unwrap();
        // Fallback: if print's show call isn't elided, provide a default
        // wrapper. LLVM aliases must point at definitions, but the RTS show
        // helpers are external declarations.
        if !has_toplevel_show_chirho {
            writeln!(
                self.output_chirho,
                "define i64 @haskelujah_show(i64 %value_chirho) {{"
            )
            .unwrap();
            writeln!(self.output_chirho, "entry:").unwrap();
            writeln!(
                self.output_chirho,
                "  %shown_chirho = call i64 @haskelujah_show_int_chirho(i64 %value_chirho)"
            )
            .unwrap();
            writeln!(self.output_chirho, "  ret i64 %shown_chirho").unwrap();
            writeln!(self.output_chirho, "}}").unwrap();
        }
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_put_str_chirho(i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_get_line_chirho()"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_write_file_chirho(i64, i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_read_file_chirho(i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_unpack_string_chirho(i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_read_int_chirho(i64)"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "declare i64 @haskelujah_main_with_large_stack_chirho(i64)"
        )
        .unwrap();
        writeln!(self.output_chirho, "declare i32 @fprintf(ptr, ...)").unwrap();
        writeln!(self.output_chirho, "declare ptr @fdopen(i32, ptr)").unwrap();
        writeln!(self.output_chirho).unwrap();

        let globals_insert_offset_chirho = self.output_chirho.len();

        // Collect top-level binding CoreIds for cross-reference resolution
        let mut toplevel_names_chirho = HashMap::new();
        let mut resolved_names_chirho = module_chirho.names_chirho.clone();
        let mut toplevel_arities_chirho = HashMap::new();
        let mut toplevel_value_kinds_chirho = HashMap::new();
        for binding_chirho in &module_chirho.bindings_chirho {
            toplevel_names_chirho.insert(
                binding_chirho.binder_chirho.id_chirho,
                binding_chirho.binder_chirho.name_chirho.clone(),
            );
            resolved_names_chirho.insert(
                binding_chirho.binder_chirho.id_chirho,
                binding_chirho.binder_chirho.name_chirho.clone(),
            );
            let (params_chirho, _body_chirho) =
                collect_lambda_params_chirho(&binding_chirho.rhs_chirho);
            toplevel_arities_chirho
                .insert(binding_chirho.binder_chirho.id_chirho, params_chirho.len());
            if let Some(value_kind_chirho) =
                classify_basic_ty_chirho(&binding_chirho.binder_chirho.ty_chirho)
            {
                toplevel_value_kinds_chirho
                    .insert(binding_chirho.binder_chirho.id_chirho, value_kind_chirho);
            }
        }
        self.toplevel_names_chirho = toplevel_names_chirho;
        self.resolved_names_chirho = resolved_names_chirho;
        self.toplevel_arities_chirho = toplevel_arities_chirho;
        self.toplevel_value_kinds_chirho = toplevel_value_kinds_chirho;

        // Compile each top-level binding
        for binding_chirho in &module_chirho.bindings_chirho {
            self.compile_binding_chirho(binding_chirho);
        }

        // Append any lifted lambdas
        for lifted_chirho in &self.lifted_functions_chirho.clone() {
            self.output_chirho.push_str(lifted_chirho);
            self.output_chirho.push('\n');
        }

        // Emit thunk trampoline functions (arities 0..8).
        // Each trampoline takes fvs_ptr, loads func_ptr + args, calls indirectly.
        for arity_chirho in 0..=8usize {
            writeln!(self.output_chirho).unwrap();
            writeln!(
                self.output_chirho,
                "define i64 @haskelujah_thunk_trampoline_{arity_chirho}_chirho(ptr %fvs_chirho) {{"
            )
            .unwrap();
            writeln!(self.output_chirho, "entry:").unwrap();
            // Load function pointer from fvs[0]
            writeln!(
                self.output_chirho,
                "  %fp_ptr_chirho = getelementptr i64, ptr %fvs_chirho, i64 0"
            )
            .unwrap();
            writeln!(
                self.output_chirho,
                "  %fp_chirho = load i64, ptr %fp_ptr_chirho"
            )
            .unwrap();
            writeln!(
                self.output_chirho,
                "  %fn_chirho = inttoptr i64 %fp_chirho to ptr"
            )
            .unwrap();
            // Load args from fvs[1..arity+1]
            for arg_idx_chirho in 0..arity_chirho {
                writeln!(
                    self.output_chirho,
                    "  %a{arg_idx_chirho}_ptr_chirho = getelementptr i64, ptr %fvs_chirho, i64 {}",
                    arg_idx_chirho + 1
                )
                .unwrap();
                writeln!(
                    self.output_chirho,
                    "  %a{arg_idx_chirho}_chirho = load i64, ptr %a{arg_idx_chirho}_ptr_chirho"
                )
                .unwrap();
            }
            // Build call with args
            let args_str_chirho: String = (0..arity_chirho)
                .map(|i_chirho| format!("i64 %a{i_chirho}_chirho"))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(
                self.output_chirho,
                "  %result_chirho = call i64 %fn_chirho({args_str_chirho})"
            )
            .unwrap();
            writeln!(self.output_chirho, "  ret i64 %result_chirho").unwrap();
            writeln!(self.output_chirho, "}}").unwrap();
        }

        if !self.global_defs_chirho.is_empty() {
            self.output_chirho
                .insert_str(globals_insert_offset_chirho, &self.global_defs_chirho);
        }

        // Emit foreign export stubs: wrapper functions with C linkage
        // that call into the corresponding Haskell binding.
        for export_chirho in &module_chirho.foreign_exports_chirho {
            let haskell_fn_chirho = mangle_name_chirho(&export_chirho.haskell_name_chirho);
            let c_name_chirho = &export_chirho.foreign_name_chirho;
            writeln!(self.output_chirho).unwrap();
            writeln!(
                self.output_chirho,
                "; foreign export {} \"{}\" = {}",
                export_chirho.calling_conv_chirho, c_name_chirho, export_chirho.haskell_name_chirho
            )
            .unwrap();
            writeln!(self.output_chirho, "define i64 @{c_name_chirho}() {{").unwrap();
            writeln!(
                self.output_chirho,
                "  %result = call i64 @{haskell_fn_chirho}()"
            )
            .unwrap();
            writeln!(self.output_chirho, "  ret i64 %result").unwrap();
            writeln!(self.output_chirho, "}}").unwrap();
        }

        self.output_chirho.clone()
    }

    /// Compile a top-level binding to an LLVM function.
    fn compile_binding_chirho(&mut self, binding_chirho: &CoreBindingChirho) {
        let fn_name_chirho = mangle_name_chirho(&binding_chirho.binder_chirho.name_chirho);

        // Skip duplicate function definitions (e.g. Prelude `min` and `$prim_Ord_min_Int`
        // can both mangle to the same name)
        if !self.emitted_names_chirho.insert(fn_name_chirho.clone()) {
            return;
        }

        // Collect lambda parameters
        let (param_binders_chirho, body_chirho) =
            collect_lambda_binders_chirho(&binding_chirho.rhs_chirho);
        let params_chirho: Vec<CoreIdChirho> = param_binders_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.id_chirho)
            .collect();

        // Track local scope: lambda parameters are local
        self.local_scope_chirho.clear();
        self.local_value_kinds_chirho.clear();
        self.local_value_names_chirho.clear();
        self.lifted_local_names_chirho.clear();
        self.lifted_local_arities_chirho.clear();
        self.lifted_local_capture_ids_chirho.clear();
        for binder_chirho in &param_binders_chirho {
            self.remember_local_binder_chirho(binder_chirho);
        }

        // Build parameter list
        let params_str_chirho: String = params_chirho
            .iter()
            .enumerate()
            .map(|(_i_chirho, id_chirho)| format!("i64 %v{}", id_chirho.0))
            .collect::<Vec<_>>()
            .join(", ");

        if self.compile_builtin_binding_chirho(
            &binding_chirho.binder_chirho.name_chirho,
            &fn_name_chirho,
            &param_binders_chirho,
            &params_chirho,
            &params_str_chirho,
        ) {
            writeln!(self.output_chirho).unwrap();
            return;
        }

        writeln!(
            self.output_chirho,
            "define i64 @{fn_name_chirho}({}) {{",
            param_binders_chirho
                .iter()
                .map(|binder_chirho| format!("i64 %v{}.entry_chirho", binder_chirho.id_chirho.0))
                .collect::<Vec<_>>()
                .join(", ")
        )
        .unwrap();
        writeln!(self.output_chirho, "entry:").unwrap();

        // Reset temporaries for this function
        self.next_tmp_chirho = 0;
        let loop_label_chirho = self.fresh_label_chirho("tail.loop");
        let mut param_slots_chirho = HashMap::new();
        for binder_chirho in &param_binders_chirho {
            let slot_tmp_chirho = self.fresh_tmp_chirho();
            writeln!(self.output_chirho, "  {slot_tmp_chirho} = alloca i64").unwrap();
            writeln!(
                self.output_chirho,
                "  store i64 %v{}.entry_chirho, ptr {slot_tmp_chirho}",
                binder_chirho.id_chirho.0
            )
            .unwrap();
            param_slots_chirho.insert(binder_chirho.id_chirho, slot_tmp_chirho);
        }
        writeln!(self.output_chirho, "  br label %{loop_label_chirho}").unwrap();
        writeln!(self.output_chirho, "{loop_label_chirho}:").unwrap();
        for binder_chirho in &param_binders_chirho {
            let slot_tmp_chirho = param_slots_chirho
                .get(&binder_chirho.id_chirho)
                .expect("top-level param slot");
            writeln!(
                self.output_chirho,
                "  %v{} = load i64, ptr {slot_tmp_chirho}",
                binder_chirho.id_chirho.0
            )
            .unwrap();
        }
        self.emit_missing_param_aliases_chirho(&param_binders_chirho, body_chirho, &[]);
        self.current_tco_self_id_chirho = Some(binding_chirho.binder_chirho.id_chirho);
        self.current_tco_loop_label_chirho = Some(loop_label_chirho);
        self.current_tco_param_slots_chirho = param_slots_chirho;
        self.current_tco_param_order_chirho = param_binders_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.id_chirho)
            .collect();

        match self.compile_tail_expr_chirho(body_chirho) {
            TailCompileOutcomeChirho::ValueChirho(result_chirho) => {
                writeln!(self.output_chirho, "  ret i64 {result_chirho}").unwrap();
            }
            TailCompileOutcomeChirho::TerminatedChirho => {}
        }
        self.current_tco_self_id_chirho = None;
        self.current_tco_loop_label_chirho = None;
        self.current_tco_param_slots_chirho.clear();
        self.current_tco_param_order_chirho.clear();
        writeln!(self.output_chirho, "}}").unwrap();
        writeln!(self.output_chirho).unwrap();
    }

    fn compile_builtin_binding_chirho(
        &mut self,
        binding_name_chirho: &str,
        fn_name_chirho: &str,
        param_binders_chirho: &[&BinderChirho],
        params_chirho: &[CoreIdChirho],
        params_str_chirho: &str,
    ) -> bool {
        match binding_name_chirho {
            "putStr" | "putStr#" => {
                let Some(arg_id_chirho) = params_chirho.first() else {
                    return false;
                };
                writeln!(
                    self.output_chirho,
                    "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
                )
                .unwrap();
                writeln!(self.output_chirho, "entry:").unwrap();
                self.next_tmp_chirho = 0;
                let argument_chirho =
                    self.emit_force_thunk_chirho(&format!("%v{}", arg_id_chirho.0));
                writeln!(
                    self.output_chirho,
                    "  call i64 @haskelujah_put_str_chirho(i64 {argument_chirho})"
                )
                .unwrap();
                writeln!(self.output_chirho, "  ret i64 0").unwrap();
                writeln!(self.output_chirho, "}}").unwrap();
                true
            }
            "putStrLn" | "putStrLn#" => {
                let Some(arg_id_chirho) = params_chirho.first() else {
                    return false;
                };
                writeln!(
                    self.output_chirho,
                    "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
                )
                .unwrap();
                writeln!(self.output_chirho, "entry:").unwrap();
                self.next_tmp_chirho = 0;
                let argument_chirho =
                    self.emit_force_thunk_chirho(&format!("%v{}", arg_id_chirho.0));
                let arg_ptr_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {arg_ptr_tmp_chirho} = inttoptr i64 {argument_chirho} to ptr"
                )
                .unwrap();
                writeln!(
                    self.output_chirho,
                    "  call i32 @puts(ptr {arg_ptr_tmp_chirho})"
                )
                .unwrap();
                writeln!(self.output_chirho, "  ret i64 0").unwrap();
                writeln!(self.output_chirho, "}}").unwrap();
                true
            }
            "getLine" | "getLine#" => {
                if !params_chirho.is_empty() {
                    return false;
                }
                writeln!(
                    self.output_chirho,
                    "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
                )
                .unwrap();
                writeln!(self.output_chirho, "entry:").unwrap();
                self.next_tmp_chirho = 0;
                let line_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {line_tmp_chirho} = call i64 @haskelujah_get_line_chirho()"
                )
                .unwrap();
                writeln!(self.output_chirho, "  ret i64 {line_tmp_chirho}").unwrap();
                writeln!(self.output_chirho, "}}").unwrap();
                true
            }
            "readFile" | "readFile#" => {
                let [path_id_chirho] = params_chirho else {
                    return false;
                };
                writeln!(
                    self.output_chirho,
                    "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
                )
                .unwrap();
                writeln!(self.output_chirho, "entry:").unwrap();
                self.next_tmp_chirho = 0;
                let path_chirho = self.emit_force_thunk_chirho(&format!("%v{}", path_id_chirho.0));
                let contents_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {contents_tmp_chirho} = call i64 @haskelujah_read_file_chirho(i64 {path_chirho})"
                )
                .unwrap();
                writeln!(self.output_chirho, "  ret i64 {contents_tmp_chirho}").unwrap();
                writeln!(self.output_chirho, "}}").unwrap();
                true
            }
            "writeFile" | "writeFile#" => {
                let [path_id_chirho, content_id_chirho] = params_chirho else {
                    return false;
                };
                writeln!(
                    self.output_chirho,
                    "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
                )
                .unwrap();
                writeln!(self.output_chirho, "entry:").unwrap();
                self.next_tmp_chirho = 0;
                let path_chirho = self.emit_force_thunk_chirho(&format!("%v{}", path_id_chirho.0));
                let content_chirho =
                    self.emit_force_thunk_chirho(&format!("%v{}", content_id_chirho.0));
                let status_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {status_tmp_chirho} = call i64 @haskelujah_write_file_chirho(i64 {path_chirho}, i64 {content_chirho})"
                )
                .unwrap();
                writeln!(self.output_chirho, "  ret i64 {status_tmp_chirho}").unwrap();
                writeln!(self.output_chirho, "}}").unwrap();
                true
            }
            "print" | "print#" => {
                let Some(arg_id_chirho) = params_chirho.last() else {
                    return false;
                };
                writeln!(
                    self.output_chirho,
                    "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
                )
                .unwrap();
                writeln!(self.output_chirho, "entry:").unwrap();
                self.next_tmp_chirho = 0;
                let arg_value_chirho = format!("%v{}", arg_id_chirho.0);
                if let Some(print_kind_chirho) =
                    classify_print_builtin_kind_chirho(param_binders_chirho)
                {
                    self.emit_print_value_chirho(&arg_value_chirho, print_kind_chirho);
                } else {
                    self.emit_print_via_show_fallback_chirho(&arg_value_chirho);
                }
                writeln!(self.output_chirho, "  ret i64 0").unwrap();
                writeln!(self.output_chirho, "}}").unwrap();
                true
            }
            "show" | "showInt#" | "showBool#" | "showChar#" | "showFloat#" => {
                let Some(arg_id_chirho) = params_chirho.last() else {
                    return false;
                };
                let Some(show_kind_chirho) =
                    classify_show_builtin_kind_chirho(binding_name_chirho, param_binders_chirho)
                else {
                    return false;
                };
                self.compile_show_builtin_chirho(
                    fn_name_chirho,
                    params_str_chirho,
                    &format!("%v{}", arg_id_chirho.0),
                    show_kind_chirho,
                );
                true
            }
            "error" | "error#" => {
                // error :: String -> a — print message to stderr and abort
                let Some(arg_id_chirho) = params_chirho.first() else {
                    return false;
                };
                writeln!(
                    self.output_chirho,
                    "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
                )
                .unwrap();
                writeln!(self.output_chirho, "entry:").unwrap();
                let msg_ptr_chirho = format!("%msg_ptr");
                writeln!(
                    self.output_chirho,
                    "  {msg_ptr_chirho} = inttoptr i64 %v{} to ptr",
                    arg_id_chirho.0
                )
                .unwrap();
                writeln!(self.output_chirho, "  call i32 @puts(ptr {msg_ptr_chirho})").unwrap();
                writeln!(self.output_chirho, "  call void @abort()").unwrap();
                writeln!(self.output_chirho, "  unreachable").unwrap();
                writeln!(self.output_chirho, "}}").unwrap();
                true
            }
            "undefined" | "undefined#" => {
                // undefined :: a — abort immediately
                writeln!(
                    self.output_chirho,
                    "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
                )
                .unwrap();
                writeln!(self.output_chirho, "entry:").unwrap();
                writeln!(self.output_chirho, "  call void @abort()").unwrap();
                writeln!(self.output_chirho, "  unreachable").unwrap();
                writeln!(self.output_chirho, "}}").unwrap();
                true
            }
            "return" | "pure" | "returnIO#" => {
                let Some(arg_id_chirho) = params_chirho.last() else {
                    return false;
                };
                writeln!(
                    self.output_chirho,
                    "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
                )
                .unwrap();
                writeln!(self.output_chirho, "entry:").unwrap();
                writeln!(self.output_chirho, "  ret i64 %v{}", arg_id_chirho.0).unwrap();
                writeln!(self.output_chirho, "}}").unwrap();
                true
            }
            ">>" | "thenIO#" => {
                let Some(arg_id_chirho) = params_chirho.last() else {
                    return false;
                };
                writeln!(
                    self.output_chirho,
                    "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
                )
                .unwrap();
                writeln!(self.output_chirho, "entry:").unwrap();
                writeln!(self.output_chirho, "  ret i64 %v{}", arg_id_chirho.0).unwrap();
                writeln!(self.output_chirho, "}}").unwrap();
                true
            }
            ">>=" | "bindIO#" => {
                if params_chirho.len() < 2 {
                    return false;
                }
                let value_id_chirho = params_chirho[0];
                let cont_id_chirho = params_chirho[1];
                writeln!(
                    self.output_chirho,
                    "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
                )
                .unwrap();
                writeln!(self.output_chirho, "entry:").unwrap();
                self.next_tmp_chirho = 0;
                let cont_ptr_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {cont_ptr_tmp_chirho} = inttoptr i64 %v{} to ptr",
                    cont_id_chirho.0
                )
                .unwrap();
                let result_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {result_tmp_chirho} = call i64 {cont_ptr_tmp_chirho}(i64 %v{})",
                    value_id_chirho.0
                )
                .unwrap();
                writeln!(self.output_chirho, "  ret i64 {result_tmp_chirho}").unwrap();
                writeln!(self.output_chirho, "}}").unwrap();
                true
            }
            _ => false,
        }
    }

    /// Check if an expression references a specific CoreId (for dependency sorting).
    #[allow(dead_code)]
    fn expr_references_id_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        target_chirho: CoreIdChirho,
    ) -> bool {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => *id_chirho == target_chirho,
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                self.expr_references_id_chirho(fun_chirho, target_chirho)
                    || self.expr_references_id_chirho(arg_chirho, target_chirho)
            }
            CoreExprChirho::LamChirho { body_chirho, .. } => {
                self.expr_references_id_chirho(body_chirho, target_chirho)
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                binds_chirho.iter().any(|(_, rhs_chirho)| {
                    self.expr_references_id_chirho(rhs_chirho, target_chirho)
                }) || self.expr_references_id_chirho(body_chirho, target_chirho)
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                self.expr_references_id_chirho(scrutinee_chirho, target_chirho)
                    || alts_chirho.iter().any(|alt_chirho| {
                        self.expr_references_id_chirho(&alt_chirho.rhs_chirho, target_chirho)
                    })
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => args_chirho
                .iter()
                .any(|a_chirho| self.expr_references_id_chirho(a_chirho, target_chirho)),
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                self.expr_references_id_chirho(body_chirho, target_chirho)
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.expr_references_id_chirho(inner_chirho, target_chirho),
            _ => false,
        }
    }

    /// Compile a Core expression, returning the LLVM value name holding the result.
    fn compile_expr_chirho(&mut self, expr_chirho: &CoreExprChirho) -> String {
        match expr_chirho {
            CoreExprChirho::LitChirho(lit_chirho) => self.compile_lit_chirho(lit_chirho),

            CoreExprChirho::VarChirho(id_chirho) => {
                if self.local_scope_chirho.contains(id_chirho) {
                    // Local variable (parameter, let-bound, case binder)
                    self.local_value_names_chirho
                        .get(id_chirho)
                        .cloned()
                        .unwrap_or_else(|| format!("%v{}", id_chirho.0))
                } else if let Some(lifted_name_chirho) =
                    self.lifted_local_names_chirho.get(id_chirho).cloned()
                {
                    self.compile_lifted_local_value_chirho(*id_chirho, &lifted_name_chirho)
                } else if let Some(name_chirho) = self.toplevel_names_chirho.get(id_chirho) {
                    let mangled_chirho = mangle_name_chirho(name_chirho);
                    let arity_chirho = self
                        .toplevel_arities_chirho
                        .get(id_chirho)
                        .copied()
                        .unwrap_or(0);
                    if arity_chirho > 0 {
                        let tmp_chirho = self.fresh_tmp_chirho();
                        writeln!(
                            self.output_chirho,
                            "  {tmp_chirho} = ptrtoint ptr @{mangled_chirho} to i64"
                        )
                        .unwrap();
                        tmp_chirho
                    } else {
                        // Reference to a nullary top-level binding — call it to obtain its value.
                        let tmp_chirho = self.fresh_tmp_chirho();
                        writeln!(
                            self.output_chirho,
                            "  {tmp_chirho} = call i64 @{mangled_chirho}()"
                        )
                        .unwrap();
                        tmp_chirho
                    }
                } else if let Some(name_chirho) = self.resolved_names_chirho.get(id_chirho).cloned()
                {
                    if let Some((fn_ref_chirho, arity_chirho)) =
                        self.resolve_builtin_runtime_function_chirho(&name_chirho)
                    {
                        if arity_chirho == 0 {
                            let tmp_chirho = self.fresh_tmp_chirho();
                            writeln!(
                                self.output_chirho,
                                "  {tmp_chirho} = call i64 {fn_ref_chirho}()"
                            )
                            .unwrap();
                            tmp_chirho
                        } else {
                            let tmp_chirho = self.fresh_tmp_chirho();
                            writeln!(
                                self.output_chirho,
                                "  {tmp_chirho} = ptrtoint ptr {fn_ref_chirho} to i64"
                            )
                            .unwrap();
                            tmp_chirho
                        }
                    } else {
                        format!("%v{}", id_chirho.0)
                    }
                } else {
                    // Unknown variable — use as local (may be from outer scope
                    // like case binder, which we've already bound)
                    format!("%v{}", id_chirho.0)
                }
            }

            CoreExprChirho::AppChirho {
                fun_chirho: _,
                arg_chirho: _,
            } => {
                // Try to detect a direct call pattern: f arg1 arg2 ...
                let (callee_chirho, args_chirho) = flatten_app_chirho(expr_chirho);
                let callee_chirho = strip_runtime_tyapps_chirho(callee_chirho);
                let arg_vals_chirho: Vec<String> = args_chirho
                    .iter()
                    .map(|a_chirho| self.compile_lazy_value_chirho(a_chirho))
                    .collect();

                if let CoreExprChirho::VarChirho(id_chirho) = callee_chirho {
                    if let Some(lifted_name_chirho) =
                        self.lifted_local_names_chirho.get(id_chirho).cloned()
                    {
                        if let Some(result_chirho) = self.compile_known_lifted_local_call_chirho(
                            *id_chirho,
                            &lifted_name_chirho,
                            &arg_vals_chirho,
                        ) {
                            return result_chirho;
                        }
                    }
                    if let Some(name_chirho) = self.toplevel_names_chirho.get(id_chirho).cloned() {
                        if matches!(name_chirho.as_str(), "print" | "print#") {
                            if let Some(arg_expr_chirho) = args_chirho.last() {
                                if let Some(show_kind_chirho) =
                                    self.classify_expr_show_kind_chirho(arg_expr_chirho)
                                {
                                    let arg_value_chirho =
                                        self.compile_expr_chirho(arg_expr_chirho);
                                    let arg_value_chirho =
                                        self.emit_force_thunk_chirho(&arg_value_chirho);
                                    self.emit_print_value_chirho(
                                        &arg_value_chirho,
                                        show_kind_chirho,
                                    );
                                    return "0".to_string();
                                }
                                if matches!(
                                    strip_runtime_tyapps_chirho(arg_expr_chirho),
                                    CoreExprChirho::ConAppChirho { args_chirho, .. } if !args_chirho.is_empty()
                                ) && self
                                    .emit_print_direct_constructor_expr_chirho(arg_expr_chirho)
                                {
                                    return "0".to_string();
                                }
                                if let Some(con_name_chirho) = self
                                    .classify_nullary_constructor_print_name_chirho(arg_expr_chirho)
                                {
                                    self.emit_print_nullary_constructor_chirho(&con_name_chirho);
                                    return "0".to_string();
                                }
                            }
                        }
                        if matches!(
                            name_chirho.as_str(),
                            "show" | "showInt#" | "showBool#" | "showChar#" | "showFloat#"
                        ) {
                            if let Some(arg_expr_chirho) = args_chirho.last() {
                                if let Some(show_kind_chirho) =
                                    self.classify_expr_show_kind_chirho(arg_expr_chirho)
                                {
                                    let arg_value_chirho =
                                        self.compile_expr_chirho(arg_expr_chirho);
                                    let arg_value_chirho =
                                        self.emit_force_thunk_chirho(&arg_value_chirho);
                                    let ptr_tmp_chirho = self.emit_show_value_ptr_chirho(
                                        &arg_value_chirho,
                                        show_kind_chirho,
                                    );
                                    let result_tmp_chirho = self.fresh_tmp_chirho();
                                    writeln!(
                                        self.output_chirho,
                                        "  {result_tmp_chirho} = ptrtoint ptr {ptr_tmp_chirho} to i64"
                                    )
                                    .unwrap();
                                    return result_tmp_chirho;
                                }
                            }
                        }
                        let fn_ref_chirho = format!("@{}", mangle_name_chirho(&name_chirho));
                        let arity_chirho = self
                            .toplevel_arities_chirho
                            .get(id_chirho)
                            .copied()
                            .unwrap_or(0);
                        if let Some(result_chirho) = self.compile_known_call_chirho(
                            &fn_ref_chirho,
                            arity_chirho,
                            &arg_vals_chirho,
                        ) {
                            return result_chirho;
                        }
                    }
                    if let Some(name_chirho) = self.resolved_names_chirho.get(id_chirho).cloned() {
                        if let Some((fn_ref_chirho, arity_chirho)) =
                            self.resolve_builtin_runtime_function_chirho(&name_chirho)
                        {
                            if let Some(result_chirho) = self.compile_known_call_chirho(
                                &fn_ref_chirho,
                                arity_chirho,
                                &arg_vals_chirho,
                            ) {
                                return result_chirho;
                            }
                        }
                    }
                }

                let fun_val_chirho = self.compile_expr_chirho(callee_chirho);
                let fun_val_chirho = self.emit_force_thunk_chirho(&fun_val_chirho);
                self.compile_curried_indirect_apps_chirho(&fun_val_chirho, &arg_vals_chirho)
            }

            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                // Lambda that wasn't collected as a top-level function parameter.
                // Lift it to a separate function.
                let lifted_name_chirho = self.fresh_lifted_name_chirho();
                let lam_expr_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: binder_chirho.clone(),
                    body_chirho: body_chirho.clone(),
                };
                self.compile_lifted_lambda_value_chirho(
                    &lifted_name_chirho,
                    &lam_expr_chirho,
                    &HashSet::new(),
                )
            }

            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                let lifted_let_bound_ids_chirho: HashSet<CoreIdChirho> = binds_chirho
                    .iter()
                    .filter_map(|(binder_chirho, rhs_chirho)| {
                        if is_runtime_lambda_chirho(rhs_chirho) {
                            Some(binder_chirho.id_chirho)
                        } else {
                            None
                        }
                    })
                    .collect();
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    if is_runtime_lambda_chirho(rhs_chirho) {
                        let lifted_name_chirho = self.fresh_lifted_name_chirho();
                        self.lifted_local_names_chirho
                            .insert(binder_chirho.id_chirho, lifted_name_chirho);
                        self.lifted_local_arities_chirho.insert(
                            binder_chirho.id_chirho,
                            collect_lambda_params_chirho(rhs_chirho).0.len(),
                        );
                        self.lifted_local_capture_ids_chirho.insert(
                            binder_chirho.id_chirho,
                            self.collect_captured_local_ids_chirho(
                                rhs_chirho,
                                &lifted_let_bound_ids_chirho,
                            ),
                        );
                    }
                }

                // Compile each binding, assigning the result to the binder's variable
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    self.remember_local_binder_chirho(binder_chirho);
                    let val_chirho = if let Some(lifted_name_chirho) = self
                        .lifted_local_names_chirho
                        .get(&binder_chirho.id_chirho)
                        .cloned()
                    {
                        self.lifted_local_capture_ids_chirho.insert(
                            binder_chirho.id_chirho,
                            self.collect_captured_local_ids_chirho(
                                rhs_chirho,
                                &lifted_let_bound_ids_chirho,
                            ),
                        );
                        self.compile_lifted_lambda_value_chirho(
                            &lifted_name_chirho,
                            rhs_chirho,
                            &lifted_let_bound_ids_chirho,
                        )
                    } else {
                        if let Some(thunk_val_chirho) =
                            self.try_create_thunk_for_app_chirho(rhs_chirho)
                        {
                            thunk_val_chirho
                        } else {
                            self.compile_expr_chirho(rhs_chirho)
                        }
                    };
                    writeln!(
                        self.output_chirho,
                        "  ; let {} = {}",
                        binder_chirho.name_chirho, val_chirho
                    )
                    .unwrap();
                    // We need to alias %v{id} to the computed value.
                    // In LLVM SSA, we use an identity add.
                    writeln!(
                        self.output_chirho,
                        "  %v{} = add i64 0, {val_chirho}",
                        binder_chirho.id_chirho.0
                    )
                    .unwrap();
                }
                self.compile_expr_chirho(body_chirho)
            }

            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                let scrut_raw_chirho = self.compile_expr_chirho(scrutinee_chirho);
                // Force thunks: case is the strict evaluation point in Haskell.
                // enter_thunk returns non-thunks as-is, so this is always safe.
                let forced_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {forced_tmp_chirho} = call i64 @haskelujah_enter_thunk_chirho(i64 {scrut_raw_chirho})"
                )
                .unwrap();
                let scrut_val_chirho = forced_tmp_chirho;

                // Bind the case binder to the scrutinee value so %v{id} is defined
                self.bind_local_value_name_chirho(bind_chirho, &scrut_val_chirho);

                if alts_chirho.is_empty() {
                    return scrut_val_chirho;
                }

                // Check if we have literal alts
                let has_lit_alts_chirho = alts_chirho
                    .iter()
                    .any(|a_chirho| matches!(a_chirho.con_chirho, AltConChirho::LitConChirho(_)));

                if has_lit_alts_chirho {
                    self.compile_case_lit_chirho(&scrut_val_chirho, alts_chirho)
                } else {
                    // Data constructor or default-only: compile each alt as a branch
                    self.compile_case_data_chirho(&scrut_val_chirho, alts_chirho)
                }
            }

            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                // Type abstraction is erased at runtime
                self.compile_expr_chirho(body_chirho)
            }

            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                // Type application is erased at runtime
                self.compile_expr_chirho(inner_chirho)
            }

            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => self.compile_primop_chirho(name_chirho, args_chirho),

            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } => self.compile_constructor_app_chirho(con_name_chirho, args_chirho),
        }
    }

    fn compile_tail_expr_chirho(
        &mut self,
        expr_chirho: &CoreExprChirho,
    ) -> TailCompileOutcomeChirho {
        match expr_chirho {
            CoreExprChirho::AppChirho { .. } => {
                let (callee_chirho, args_chirho) = flatten_app_chirho(expr_chirho);
                let callee_chirho = strip_runtime_tyapps_chirho(callee_chirho);
                if self.try_compile_self_tail_call_chirho(callee_chirho, &args_chirho) {
                    TailCompileOutcomeChirho::TerminatedChirho
                } else {
                    TailCompileOutcomeChirho::ValueChirho(self.compile_expr_chirho(expr_chirho))
                }
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                let lifted_let_bound_ids_chirho: HashSet<CoreIdChirho> = binds_chirho
                    .iter()
                    .filter_map(|(binder_chirho, rhs_chirho)| {
                        if is_runtime_lambda_chirho(rhs_chirho) {
                            Some(binder_chirho.id_chirho)
                        } else {
                            None
                        }
                    })
                    .collect();
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    if is_runtime_lambda_chirho(rhs_chirho) {
                        let lifted_name_chirho = self.fresh_lifted_name_chirho();
                        self.lifted_local_names_chirho
                            .insert(binder_chirho.id_chirho, lifted_name_chirho);
                        self.lifted_local_arities_chirho.insert(
                            binder_chirho.id_chirho,
                            collect_lambda_params_chirho(rhs_chirho).0.len(),
                        );
                        self.lifted_local_capture_ids_chirho.insert(
                            binder_chirho.id_chirho,
                            self.collect_captured_local_ids_chirho(
                                rhs_chirho,
                                &lifted_let_bound_ids_chirho,
                            ),
                        );
                    }
                }

                for (binder_chirho, rhs_chirho) in binds_chirho {
                    self.remember_local_binder_chirho(binder_chirho);
                    let val_chirho = if let Some(lifted_name_chirho) = self
                        .lifted_local_names_chirho
                        .get(&binder_chirho.id_chirho)
                        .cloned()
                    {
                        self.lifted_local_capture_ids_chirho.insert(
                            binder_chirho.id_chirho,
                            self.collect_captured_local_ids_chirho(
                                rhs_chirho,
                                &lifted_let_bound_ids_chirho,
                            ),
                        );
                        self.compile_lifted_lambda_value_chirho(
                            &lifted_name_chirho,
                            rhs_chirho,
                            &lifted_let_bound_ids_chirho,
                        )
                    } else {
                        if let Some(thunk_val_chirho) =
                            self.try_create_thunk_for_app_chirho(rhs_chirho)
                        {
                            thunk_val_chirho
                        } else {
                            self.compile_expr_chirho(rhs_chirho)
                        }
                    };
                    writeln!(
                        self.output_chirho,
                        "  ; let {} = {}",
                        binder_chirho.name_chirho, val_chirho
                    )
                    .unwrap();
                    writeln!(
                        self.output_chirho,
                        "  %v{} = add i64 0, {val_chirho}",
                        binder_chirho.id_chirho.0
                    )
                    .unwrap();
                }
                self.compile_tail_expr_chirho(body_chirho)
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                let scrut_raw_chirho = self.compile_expr_chirho(scrutinee_chirho);
                // Force thunks at case scrutinees (strict evaluation point).
                let forced_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {forced_tmp_chirho} = call i64 @haskelujah_enter_thunk_chirho(i64 {scrut_raw_chirho})"
                )
                .unwrap();
                let scrut_val_chirho = forced_tmp_chirho;
                self.bind_local_value_name_chirho(bind_chirho, &scrut_val_chirho);

                if alts_chirho.is_empty() {
                    return TailCompileOutcomeChirho::ValueChirho(scrut_val_chirho);
                }

                let has_lit_alts_chirho = alts_chirho
                    .iter()
                    .any(|a_chirho| matches!(a_chirho.con_chirho, AltConChirho::LitConChirho(_)));
                if has_lit_alts_chirho {
                    self.compile_case_lit_tail_chirho(&scrut_val_chirho, alts_chirho)
                } else {
                    self.compile_case_data_tail_chirho(&scrut_val_chirho, alts_chirho)
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                self.compile_tail_expr_chirho(body_chirho)
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.compile_tail_expr_chirho(inner_chirho),
            _ => TailCompileOutcomeChirho::ValueChirho(self.compile_expr_chirho(expr_chirho)),
        }
    }

    fn try_compile_self_tail_call_chirho(
        &mut self,
        callee_chirho: &CoreExprChirho,
        args_chirho: &[&CoreExprChirho],
    ) -> bool {
        let Some(self_id_chirho) = self.current_tco_self_id_chirho else {
            return false;
        };
        let Some(loop_label_chirho) = self.current_tco_loop_label_chirho.clone() else {
            return false;
        };
        let CoreExprChirho::VarChirho(callee_id_chirho) = callee_chirho else {
            return false;
        };
        if *callee_id_chirho != self_id_chirho {
            return false;
        }

        let param_ids_chirho = self.current_tco_param_order_chirho.clone();
        if args_chirho.len() != param_ids_chirho.len() {
            return false;
        }

        let arg_vals_chirho: Vec<String> = args_chirho
            .iter()
            .map(|arg_chirho| self.compile_lazy_value_chirho(arg_chirho))
            .collect();
        for (param_id_chirho, arg_val_chirho) in param_ids_chirho.iter().zip(arg_vals_chirho.iter())
        {
            let Some(slot_tmp_chirho) = self.current_tco_param_slots_chirho.get(param_id_chirho)
            else {
                return false;
            };
            writeln!(
                self.output_chirho,
                "  store i64 {arg_val_chirho}, ptr {slot_tmp_chirho}"
            )
            .unwrap();
        }
        writeln!(self.output_chirho, "  br label %{loop_label_chirho}").unwrap();
        true
    }

    fn compile_lit_chirho(&mut self, lit_chirho: &CoreLitChirho) -> String {
        match lit_chirho {
            CoreLitChirho::IntChirho(v_chirho) => format!("{v_chirho}"),
            CoreLitChirho::FloatChirho(v_chirho) => encode_float_literal_chirho(*v_chirho),
            CoreLitChirho::CharChirho(c_chirho) => format!("{}", *c_chirho as i64),
            CoreLitChirho::StringChirho(value_chirho) => {
                self.intern_string_literal_chirho(value_chirho)
            }
        }
    }

    fn intern_string_literal_chirho(&mut self, value_chirho: &str) -> String {
        let global_name_chirho = self.intern_string_global_name_chirho(value_chirho);
        format!("ptrtoint (ptr @{global_name_chirho} to i64)")
    }

    fn intern_string_global_name_chirho(&mut self, value_chirho: &str) -> String {
        if let Some(global_name_chirho) = self.string_globals_chirho.get(value_chirho) {
            return global_name_chirho.clone();
        }

        let global_name_chirho = format!(".str.{}", self.next_string_chirho);
        self.next_string_chirho += 1;

        let escaped_bytes_chirho = escape_llvm_string_bytes_chirho(value_chirho.as_bytes());
        let len_chirho = value_chirho.len() + 1;
        writeln!(
            self.global_defs_chirho,
            "@{global_name_chirho} = private unnamed_addr constant [{len_chirho} x i8] c\"{escaped_bytes_chirho}\\00\", align 1"
        )
        .unwrap();

        self.string_globals_chirho
            .insert(value_chirho.to_string(), global_name_chirho.clone());

        global_name_chirho
    }

    fn compile_indirect_call_chirho(
        &mut self,
        fun_val_chirho: &str,
        args_str_chirho: &str,
    ) -> String {
        let forced_fun_chirho = self.emit_force_thunk_chirho(fun_val_chirho);
        let fun_val_chirho = forced_fun_chirho.as_str();
        let boxed_label_chirho = self.fresh_label_chirho("call.boxed");
        let direct_label_chirho = self.fresh_label_chirho("call.direct");
        let join_label_chirho = self.fresh_label_chirho("call.join");
        let is_boxed_tmp_chirho = self.fresh_tmp_chirho();
        let boxed_probe_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {boxed_probe_tmp_chirho} = call i64 @haskelujah_is_heap_ptr_chirho(i64 {fun_val_chirho})"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {is_boxed_tmp_chirho} = icmp ne i64 {boxed_probe_tmp_chirho}, 0"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  br i1 {is_boxed_tmp_chirho}, label %{boxed_label_chirho}, label %{direct_label_chirho}"
        )
        .unwrap();

        writeln!(self.output_chirho, "{direct_label_chirho}:").unwrap();
        let direct_fun_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {direct_fun_ptr_tmp_chirho} = inttoptr i64 {fun_val_chirho} to ptr"
        )
        .unwrap();
        let direct_result_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {direct_result_tmp_chirho} = call i64 {direct_fun_ptr_tmp_chirho}({args_str_chirho})"
        )
        .unwrap();
        writeln!(self.output_chirho, "  br label %{join_label_chirho}").unwrap();

        writeln!(self.output_chirho, "{boxed_label_chirho}:").unwrap();
        let closure_fun_bits_tmp_chirho =
            self.load_boxed_constructor_field_chirho(fun_val_chirho, 0);
        let closure_entry_bits_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {closure_entry_bits_tmp_chirho} = and i64 {closure_fun_bits_tmp_chirho}, {NATIVE_OBJECT_HEADER_PTR_MASK_CHIRHO}"
        )
        .unwrap();
        let closure_fun_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {closure_fun_ptr_tmp_chirho} = inttoptr i64 {closure_entry_bits_tmp_chirho} to ptr"
        )
        .unwrap();
        let closure_result_tmp_chirho = self.fresh_tmp_chirho();
        let closure_args_chirho = if args_str_chirho.is_empty() {
            format!("i64 {fun_val_chirho}")
        } else {
            format!("i64 {fun_val_chirho}, {args_str_chirho}")
        };
        writeln!(
            self.output_chirho,
            "  {closure_result_tmp_chirho} = call i64 {closure_fun_ptr_tmp_chirho}({closure_args_chirho})"
        )
        .unwrap();
        writeln!(self.output_chirho, "  br label %{join_label_chirho}").unwrap();

        writeln!(self.output_chirho, "{join_label_chirho}:").unwrap();
        let result_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {result_tmp_chirho} = phi i64 [{direct_result_tmp_chirho}, %{direct_label_chirho}], [{closure_result_tmp_chirho}, %{boxed_label_chirho}]"
        )
        .unwrap();
        result_tmp_chirho
    }

    fn load_boxed_constructor_field_chirho(
        &mut self,
        boxed_value_chirho: &str,
        field_index_chirho: usize,
    ) -> String {
        let boxed_ptr_tmp_chirho = self.decode_boxed_constructor_ptr_chirho(boxed_value_chirho);
        let field_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {field_ptr_tmp_chirho} = getelementptr i64, ptr {boxed_ptr_tmp_chirho}, i64 {field_index_chirho}"
        )
        .unwrap();
        let field_val_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {field_val_tmp_chirho} = load i64, ptr {field_ptr_tmp_chirho}"
        )
        .unwrap();
        field_val_tmp_chirho
    }

    fn compile_curried_indirect_apps_chirho(
        &mut self,
        fun_val_chirho: &str,
        arg_vals_chirho: &[String],
    ) -> String {
        if arg_vals_chirho.is_empty() {
            return fun_val_chirho.to_string();
        }
        // Try a saturated multi-arg call first: pass ALL arguments to the
        // function at once.  For bare function pointers (direct path) this
        // is correct when the callee's arity matches the number of args.
        // For closures (boxed path) the entry code already expects the
        // closure env + all real args.
        //
        // If the callee has FEWER parameters we get UB, so fall-through
        // curried application is the safe default.  However, the vast
        // majority of indirect calls at the call-site have the right
        // arity, so this is the common fast path.
        let all_args_str_chirho = arg_vals_chirho
            .iter()
            .map(|v_chirho| format!("i64 {v_chirho}"))
            .collect::<Vec<_>>()
            .join(", ");
        self.compile_indirect_call_chirho(fun_val_chirho, &all_args_str_chirho)
    }

    fn compile_known_call_chirho(
        &mut self,
        fn_ref_chirho: &str,
        arity_chirho: usize,
        arg_vals_chirho: &[String],
    ) -> Option<String> {
        if arity_chirho == 0 {
            let tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {tmp_chirho} = tail call i64 {fn_ref_chirho}()"
            )
            .unwrap();
            self.emit_gc_root_push_heap_i64_chirho(&tmp_chirho);
            return Some(self.compile_curried_indirect_apps_chirho(&tmp_chirho, arg_vals_chirho));
        }

        if arg_vals_chirho.len() < arity_chirho {
            return Some(self.emit_partial_application_closure_chirho(
                fn_ref_chirho,
                None,
                arg_vals_chirho,
                arity_chirho - arg_vals_chirho.len(),
            ));
        }

        let direct_args_str_chirho = arg_vals_chirho[..arity_chirho]
            .iter()
            .map(|v_chirho| format!("i64 {v_chirho}"))
            .collect::<Vec<_>>()
            .join(", ");
        let tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {tmp_chirho} = tail call i64 {fn_ref_chirho}({direct_args_str_chirho})"
        )
        .unwrap();
        self.emit_gc_root_push_heap_i64_chirho(&tmp_chirho);
        if arg_vals_chirho.len() == arity_chirho {
            return Some(tmp_chirho);
        }

        Some(
            self.compile_curried_indirect_apps_chirho(
                &tmp_chirho,
                &arg_vals_chirho[arity_chirho..],
            ),
        )
    }

    fn compile_known_closure_call_chirho(
        &mut self,
        fn_ref_chirho: &str,
        closure_val_chirho: &str,
        arity_chirho: usize,
        arg_vals_chirho: &[String],
    ) -> Option<String> {
        if arity_chirho == 0 {
            return None;
        }

        if arg_vals_chirho.len() < arity_chirho {
            return Some(self.emit_partial_application_closure_chirho(
                fn_ref_chirho,
                Some(closure_val_chirho),
                arg_vals_chirho,
                arity_chirho - arg_vals_chirho.len(),
            ));
        }

        let mut direct_args_chirho = vec![format!("i64 {closure_val_chirho}")];
        direct_args_chirho.extend(
            arg_vals_chirho[..arity_chirho]
                .iter()
                .map(|arg_val_chirho| format!("i64 {arg_val_chirho}")),
        );
        let tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {tmp_chirho} = tail call i64 {fn_ref_chirho}({})",
            direct_args_chirho.join(", ")
        )
        .unwrap();
        self.emit_gc_root_push_heap_i64_chirho(&tmp_chirho);
        if arg_vals_chirho.len() == arity_chirho {
            return Some(tmp_chirho);
        }

        Some(
            self.compile_curried_indirect_apps_chirho(
                &tmp_chirho,
                &arg_vals_chirho[arity_chirho..],
            ),
        )
    }

    fn compile_lifted_local_value_chirho(
        &mut self,
        id_chirho: CoreIdChirho,
        lifted_name_chirho: &str,
    ) -> String {
        let captured_ids_chirho = self
            .lifted_local_capture_ids_chirho
            .get(&id_chirho)
            .cloned()
            .unwrap_or_default();
        if captured_ids_chirho.is_empty() {
            let tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {tmp_chirho} = ptrtoint ptr @{lifted_name_chirho} to i64"
            )
            .unwrap();
            return tmp_chirho;
        }

        if self.local_scope_chirho.contains(&id_chirho) {
            return format!("%v{}", id_chirho.0);
        }

        self.emit_closure_value_chirho(lifted_name_chirho, &captured_ids_chirho)
    }

    fn compile_known_lifted_local_call_chirho(
        &mut self,
        id_chirho: CoreIdChirho,
        lifted_name_chirho: &str,
        arg_vals_chirho: &[String],
    ) -> Option<String> {
        let arity_chirho = self
            .lifted_local_arities_chirho
            .get(&id_chirho)
            .copied()
            .unwrap_or(0);
        let captured_ids_chirho = self
            .lifted_local_capture_ids_chirho
            .get(&id_chirho)
            .cloned()
            .unwrap_or_default();
        if captured_ids_chirho.is_empty() {
            return self.compile_known_call_chirho(
                &format!("@{lifted_name_chirho}"),
                arity_chirho,
                arg_vals_chirho,
            );
        }

        let closure_val_chirho = if self.local_scope_chirho.contains(&id_chirho) {
            format!("%v{}", id_chirho.0)
        } else {
            self.emit_closure_value_chirho(lifted_name_chirho, &captured_ids_chirho)
        };
        self.compile_known_closure_call_chirho(
            &format!("@{lifted_name_chirho}"),
            &closure_val_chirho,
            arity_chirho,
            arg_vals_chirho,
        )
    }

    fn emit_closure_value_chirho(
        &mut self,
        lifted_name_chirho: &str,
        captured_ids_chirho: &[CoreIdChirho],
    ) -> String {
        let captured_vals_chirho: Vec<String> = captured_ids_chirho
            .iter()
            .map(|capture_id_chirho| {
                self.compile_expr_chirho(&CoreExprChirho::VarChirho(*capture_id_chirho))
            })
            .collect();
        self.emit_captured_value_closure_chirho(lifted_name_chirho, &captured_vals_chirho)
    }

    fn emit_captured_value_closure_chirho(
        &mut self,
        lifted_name_chirho: &str,
        captured_vals_chirho: &[String],
    ) -> String {
        for captured_val_chirho in captured_vals_chirho {
            self.emit_gc_root_push_i64_chirho(captured_val_chirho);
        }
        let alloc_size_chirho = ((captured_vals_chirho.len() + 1) * 8) as i64;
        let closure_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {closure_ptr_tmp_chirho} = call ptr @haskelujah_alloc_chirho(i64 {alloc_size_chirho})"
        )
        .unwrap();
        let fun_bits_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {fun_bits_tmp_chirho} = ptrtoint ptr @{lifted_name_chirho} to i64"
        )
        .unwrap();
        let fun_header_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {fun_header_tmp_chirho} = or i64 {fun_bits_tmp_chirho}, {NATIVE_FUNCTION_HEADER_KIND_CHIRHO}"
        )
        .unwrap();
        let fun_slot_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {fun_slot_tmp_chirho} = getelementptr i64, ptr {closure_ptr_tmp_chirho}, i64 0"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  store i64 {fun_header_tmp_chirho}, ptr {fun_slot_tmp_chirho}"
        )
        .unwrap();
        for (capture_idx_chirho, capture_value_chirho) in captured_vals_chirho.iter().enumerate() {
            let capture_slot_tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {capture_slot_tmp_chirho} = getelementptr i64, ptr {closure_ptr_tmp_chirho}, i64 {}",
                capture_idx_chirho + 1
            )
            .unwrap();
            writeln!(
                self.output_chirho,
                "  store i64 {capture_value_chirho}, ptr {capture_slot_tmp_chirho}"
            )
            .unwrap();
        }
        let closure_bits_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {closure_bits_tmp_chirho} = ptrtoint ptr {closure_ptr_tmp_chirho} to i64"
        )
        .unwrap();
        let tagged_closure_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {tagged_closure_tmp_chirho} = or i64 {closure_bits_tmp_chirho}, {BOXED_CONSTRUCTOR_TAG_MASK_CHIRHO}"
        )
        .unwrap();
        self.emit_gc_root_pop_count_chirho(captured_vals_chirho.len());
        // Like constructor/PAP results, a freshly allocated closure must stay
        // reachable while its caller evaluates later, allocating arguments.
        self.emit_gc_root_push_i64_chirho(&tagged_closure_tmp_chirho);
        tagged_closure_tmp_chirho
    }

    fn emit_partial_application_closure_chirho(
        &mut self,
        fn_ref_chirho: &str,
        closure_env_val_chirho: Option<&str>,
        applied_arg_vals_chirho: &[String],
        remaining_arity_chirho: usize,
    ) -> String {
        let wrapper_name_chirho = self.fresh_lifted_name_chirho();
        let mut wrapper_ir_chirho = String::new();
        let mut next_tmp_idx_chirho = 0usize;
        let mut fresh_wrapper_tmp_chirho = || {
            let tmp_chirho = format!("%t{next_tmp_idx_chirho}");
            next_tmp_idx_chirho += 1;
            tmp_chirho
        };

        let mut params_chirho = vec!["i64 %env_chirho".to_string()];
        for arg_idx_chirho in 0..remaining_arity_chirho {
            params_chirho.push(format!("i64 %partial_arg_{arg_idx_chirho}_chirho"));
        }
        writeln!(
            wrapper_ir_chirho,
            "define i64 @{wrapper_name_chirho}({}) {{",
            params_chirho.join(", ")
        )
        .unwrap();
        writeln!(wrapper_ir_chirho, "entry:").unwrap();

        let env_ptr_bits_tmp_chirho = fresh_wrapper_tmp_chirho();
        writeln!(
            wrapper_ir_chirho,
            "  {env_ptr_bits_tmp_chirho} = and i64 %env_chirho, {BOXED_CONSTRUCTOR_PTR_MASK_CHIRHO}"
        )
        .unwrap();
        let env_ptr_tmp_chirho = fresh_wrapper_tmp_chirho();
        writeln!(
            wrapper_ir_chirho,
            "  {env_ptr_tmp_chirho} = inttoptr i64 {env_ptr_bits_tmp_chirho} to ptr"
        )
        .unwrap();

        let mut env_field_idx_chirho = 1usize;
        let mut call_args_chirho = Vec::new();
        if closure_env_val_chirho.is_some() {
            let closure_slot_ptr_tmp_chirho = fresh_wrapper_tmp_chirho();
            writeln!(
                wrapper_ir_chirho,
                "  {closure_slot_ptr_tmp_chirho} = getelementptr i64, ptr {env_ptr_tmp_chirho}, i64 {env_field_idx_chirho}"
            )
            .unwrap();
            let closure_val_tmp_chirho = fresh_wrapper_tmp_chirho();
            writeln!(
                wrapper_ir_chirho,
                "  {closure_val_tmp_chirho} = load i64, ptr {closure_slot_ptr_tmp_chirho}"
            )
            .unwrap();
            call_args_chirho.push(format!("i64 {closure_val_tmp_chirho}"));
            env_field_idx_chirho += 1;
        }

        for _ in applied_arg_vals_chirho {
            let capture_slot_ptr_tmp_chirho = fresh_wrapper_tmp_chirho();
            writeln!(
                wrapper_ir_chirho,
                "  {capture_slot_ptr_tmp_chirho} = getelementptr i64, ptr {env_ptr_tmp_chirho}, i64 {env_field_idx_chirho}"
            )
            .unwrap();
            let capture_val_tmp_chirho = fresh_wrapper_tmp_chirho();
            writeln!(
                wrapper_ir_chirho,
                "  {capture_val_tmp_chirho} = load i64, ptr {capture_slot_ptr_tmp_chirho}"
            )
            .unwrap();
            call_args_chirho.push(format!("i64 {capture_val_tmp_chirho}"));
            env_field_idx_chirho += 1;
        }

        for arg_idx_chirho in 0..remaining_arity_chirho {
            call_args_chirho.push(format!("i64 %partial_arg_{arg_idx_chirho}_chirho"));
        }

        let call_result_tmp_chirho = fresh_wrapper_tmp_chirho();
        writeln!(
            wrapper_ir_chirho,
            "  {call_result_tmp_chirho} = tail call i64 {fn_ref_chirho}({})",
            call_args_chirho.join(", ")
        )
        .unwrap();
        writeln!(wrapper_ir_chirho, "  ret i64 {call_result_tmp_chirho}").unwrap();
        writeln!(wrapper_ir_chirho, "}}").unwrap();

        self.lifted_functions_chirho.push(wrapper_ir_chirho);

        let mut captured_vals_chirho = Vec::new();
        if let Some(closure_env_val_chirho) = closure_env_val_chirho {
            captured_vals_chirho.push(closure_env_val_chirho.to_string());
        }
        captured_vals_chirho.extend(applied_arg_vals_chirho.iter().cloned());
        self.emit_captured_value_closure_chirho(&wrapper_name_chirho, &captured_vals_chirho)
    }

    fn compile_constructor_app_chirho(
        &mut self,
        con_name_chirho: &str,
        args_chirho: &[CoreExprChirho],
    ) -> String {
        let tag_chirho = constructor_tag_chirho(con_name_chirho);
        if args_chirho.is_empty() {
            return format!("{tag_chirho}");
        }

        let field_vals_chirho: Vec<String> = args_chirho
            .iter()
            .map(|arg_chirho| self.compile_lazy_value_chirho(arg_chirho))
            .collect();
        for field_val_chirho in &field_vals_chirho {
            self.emit_gc_root_push_i64_chirho(field_val_chirho);
        }
        let alloc_tmp_chirho = self.fresh_tmp_chirho();
        let alloc_size_chirho = ((field_vals_chirho.len() + 1) * 8) as i64;
        writeln!(
            self.output_chirho,
            "  {alloc_tmp_chirho} = call ptr @haskelujah_alloc_chirho(i64 {alloc_size_chirho})"
        )
        .unwrap();

        let tag_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {tag_ptr_tmp_chirho} = getelementptr i64, ptr {alloc_tmp_chirho}, i64 0"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  store i64 {tag_chirho}, ptr {tag_ptr_tmp_chirho}"
        )
        .unwrap();

        for (field_idx_chirho, field_val_chirho) in field_vals_chirho.iter().enumerate() {
            let field_ptr_tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {field_ptr_tmp_chirho} = getelementptr i64, ptr {alloc_tmp_chirho}, i64 {}",
                field_idx_chirho + 1
            )
            .unwrap();
            writeln!(
                self.output_chirho,
                "  store i64 {field_val_chirho}, ptr {field_ptr_tmp_chirho}"
            )
            .unwrap();
        }

        let ptr_i64_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {ptr_i64_tmp_chirho} = ptrtoint ptr {alloc_tmp_chirho} to i64"
        )
        .unwrap();
        let tagged_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {tagged_ptr_tmp_chirho} = or i64 {ptr_i64_tmp_chirho}, {BOXED_CONSTRUCTOR_TAG_MASK_CHIRHO}"
        )
        .unwrap();
        self.emit_gc_root_pop_count_chirho(field_vals_chirho.len());
        self.emit_gc_root_push_i64_chirho(&tagged_ptr_tmp_chirho);
        tagged_ptr_tmp_chirho
    }

    fn compile_show_builtin_chirho(
        &mut self,
        fn_name_chirho: &str,
        params_str_chirho: &str,
        arg_value_chirho: &str,
        show_kind_chirho: ShowBuiltinKindChirho,
    ) {
        writeln!(
            self.output_chirho,
            "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
        )
        .unwrap();
        writeln!(self.output_chirho, "entry:").unwrap();
        self.next_tmp_chirho = 0;

        match show_kind_chirho {
            ShowBuiltinKindChirho::IntChirho
            | ShowBuiltinKindChirho::BoolChirho
            | ShowBuiltinKindChirho::OrderingChirho
            | ShowBuiltinKindChirho::CharChirho
            | ShowBuiltinKindChirho::DoubleChirho
            | ShowBuiltinKindChirho::ListChirho(_) => {
                let ptr_tmp_chirho =
                    self.emit_show_value_ptr_chirho(arg_value_chirho, show_kind_chirho);
                let result_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {result_tmp_chirho} = ptrtoint ptr {ptr_tmp_chirho} to i64"
                )
                .unwrap();
                writeln!(self.output_chirho, "  ret i64 {result_tmp_chirho}").unwrap();
            }
        }

        writeln!(self.output_chirho, "}}").unwrap();
    }

    fn emit_show_value_ptr_chirho(
        &mut self,
        arg_value_chirho: &str,
        show_kind_chirho: ShowBuiltinKindChirho,
    ) -> String {
        let arg_value_chirho = self.emit_force_thunk_chirho(arg_value_chirho);
        match show_kind_chirho {
            ShowBuiltinKindChirho::IntChirho => {
                let fmt_name_chirho = self.intern_string_global_name_chirho("%ld");
                let buf_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {buf_tmp_chirho} = call ptr @haskelujah_alloc_chirho(i64 32)"
                )
                .unwrap();
                writeln!(
                    self.output_chirho,
                    "  call i32 (ptr, i64, ptr, ...) @snprintf(ptr {buf_tmp_chirho}, i64 32, ptr @{fmt_name_chirho}, i64 {arg_value_chirho})"
                )
                .unwrap();
                buf_tmp_chirho
            }
            ShowBuiltinKindChirho::BoolChirho => {
                let true_name_chirho = self.intern_string_global_name_chirho("True");
                let false_name_chirho = self.intern_string_global_name_chirho("False");
                let cmp_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {cmp_tmp_chirho} = icmp ne i64 {arg_value_chirho}, 0"
                )
                .unwrap();
                let ptr_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {ptr_tmp_chirho} = select i1 {cmp_tmp_chirho}, ptr @{true_name_chirho}, ptr @{false_name_chirho}"
                )
                .unwrap();
                ptr_tmp_chirho
            }
            ShowBuiltinKindChirho::OrderingChirho => {
                let lt_name_chirho = self.intern_string_global_name_chirho("LT");
                let eq_name_chirho = self.intern_string_global_name_chirho("EQ");
                let gt_name_chirho = self.intern_string_global_name_chirho("GT");
                let is_lt_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {is_lt_tmp_chirho} = icmp eq i64 {arg_value_chirho}, 0"
                )
                .unwrap();
                let is_eq_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {is_eq_tmp_chirho} = icmp eq i64 {arg_value_chirho}, 1"
                )
                .unwrap();
                let eq_or_gt_ptr_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {eq_or_gt_ptr_tmp_chirho} = select i1 {is_eq_tmp_chirho}, ptr @{eq_name_chirho}, ptr @{gt_name_chirho}"
                )
                .unwrap();
                let ptr_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {ptr_tmp_chirho} = select i1 {is_lt_tmp_chirho}, ptr @{lt_name_chirho}, ptr {eq_or_gt_ptr_tmp_chirho}"
                )
                .unwrap();
                ptr_tmp_chirho
            }
            ShowBuiltinKindChirho::CharChirho => {
                let fmt_name_chirho = self.intern_string_global_name_chirho("'%c'");
                let buf_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {buf_tmp_chirho} = call ptr @haskelujah_alloc_chirho(i64 8)"
                )
                .unwrap();
                let char_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {char_tmp_chirho} = trunc i64 {arg_value_chirho} to i32"
                )
                .unwrap();
                writeln!(
                    self.output_chirho,
                    "  call i32 (ptr, i64, ptr, ...) @snprintf(ptr {buf_tmp_chirho}, i64 8, ptr @{fmt_name_chirho}, i32 {char_tmp_chirho})"
                )
                .unwrap();
                buf_tmp_chirho
            }
            ShowBuiltinKindChirho::DoubleChirho => {
                let fmt_name_chirho = self.intern_string_global_name_chirho("%.15g");
                let buf_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {buf_tmp_chirho} = call ptr @haskelujah_alloc_chirho(i64 32)"
                )
                .unwrap();
                let float_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {float_tmp_chirho} = bitcast i64 {arg_value_chirho} to double"
                )
                .unwrap();
                writeln!(
                    self.output_chirho,
                    "  call i32 (ptr, i64, ptr, ...) @snprintf(ptr {buf_tmp_chirho}, i64 32, ptr @{fmt_name_chirho}, double {float_tmp_chirho})"
                )
                .unwrap();
                buf_tmp_chirho
            }
            ShowBuiltinKindChirho::ListChirho(element_kind_chirho) => self
                .emit_show_list_value_ptr_chirho(&arg_value_chirho, element_kind_chirho.as_ref()),
        }
    }

    fn emit_store_byte_at_index_chirho(
        &mut self,
        buffer_ptr_chirho: &str,
        index_value_chirho: &str,
        byte_value_chirho: u8,
    ) {
        let dst_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {dst_ptr_tmp_chirho} = getelementptr i8, ptr {buffer_ptr_chirho}, i64 {index_value_chirho}"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  store i8 {byte_value_chirho}, ptr {dst_ptr_tmp_chirho}"
        )
        .unwrap();
    }

    fn emit_push_byte_chirho(
        &mut self,
        buffer_ptr_chirho: &str,
        index_ptr_chirho: &str,
        byte_value_chirho: u8,
    ) {
        let index_value_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {index_value_tmp_chirho} = load i64, ptr {index_ptr_chirho}"
        )
        .unwrap();
        self.emit_store_byte_at_index_chirho(
            buffer_ptr_chirho,
            &index_value_tmp_chirho,
            byte_value_chirho,
        );
        let next_index_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {next_index_tmp_chirho} = add i64 {index_value_tmp_chirho}, 1"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  store i64 {next_index_tmp_chirho}, ptr {index_ptr_chirho}"
        )
        .unwrap();
    }

    fn emit_append_c_string_chirho(
        &mut self,
        buffer_ptr_chirho: &str,
        index_ptr_chirho: &str,
        source_ptr_chirho: &str,
    ) {
        let src_ptr_alloca_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {src_ptr_alloca_tmp_chirho} = alloca ptr"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  store ptr {source_ptr_chirho}, ptr {src_ptr_alloca_tmp_chirho}"
        )
        .unwrap();

        let loop_label_chirho = self.fresh_label_chirho("show.append.loop");
        let copy_label_chirho = self.fresh_label_chirho("show.append.copy");
        let done_label_chirho = self.fresh_label_chirho("show.append.done");
        writeln!(self.output_chirho, "  br label %{loop_label_chirho}").unwrap();

        writeln!(self.output_chirho, "{loop_label_chirho}:").unwrap();
        let src_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {src_ptr_tmp_chirho} = load ptr, ptr {src_ptr_alloca_tmp_chirho}"
        )
        .unwrap();
        let src_char_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {src_char_tmp_chirho} = load i8, ptr {src_ptr_tmp_chirho}"
        )
        .unwrap();
        let is_end_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {is_end_tmp_chirho} = icmp eq i8 {src_char_tmp_chirho}, 0"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  br i1 {is_end_tmp_chirho}, label %{done_label_chirho}, label %{copy_label_chirho}"
        )
        .unwrap();

        writeln!(self.output_chirho, "{copy_label_chirho}:").unwrap();
        let index_value_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {index_value_tmp_chirho} = load i64, ptr {index_ptr_chirho}"
        )
        .unwrap();
        let dst_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {dst_ptr_tmp_chirho} = getelementptr i8, ptr {buffer_ptr_chirho}, i64 {index_value_tmp_chirho}"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  store i8 {src_char_tmp_chirho}, ptr {dst_ptr_tmp_chirho}"
        )
        .unwrap();
        let next_index_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {next_index_tmp_chirho} = add i64 {index_value_tmp_chirho}, 1"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  store i64 {next_index_tmp_chirho}, ptr {index_ptr_chirho}"
        )
        .unwrap();
        let next_src_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {next_src_ptr_tmp_chirho} = getelementptr i8, ptr {src_ptr_tmp_chirho}, i64 1"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  store ptr {next_src_ptr_tmp_chirho}, ptr {src_ptr_alloca_tmp_chirho}"
        )
        .unwrap();
        writeln!(self.output_chirho, "  br label %{loop_label_chirho}").unwrap();

        writeln!(self.output_chirho, "{done_label_chirho}:").unwrap();
    }

    fn emit_show_list_value_ptr_chirho(
        &mut self,
        arg_value_chirho: &str,
        element_kind_chirho: &ShowBuiltinKindChirho,
    ) -> String {
        let buf_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {buf_tmp_chirho} = call ptr @haskelujah_alloc_chirho(i64 4096)"
        )
        .unwrap();
        let index_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(self.output_chirho, "  {index_ptr_tmp_chirho} = alloca i64").unwrap();
        writeln!(
            self.output_chirho,
            "  store i64 0, ptr {index_ptr_tmp_chirho}"
        )
        .unwrap();
        self.emit_push_byte_chirho(&buf_tmp_chirho, &index_ptr_tmp_chirho, b'[');

        let current_list_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {current_list_ptr_tmp_chirho} = alloca i64"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  store i64 {arg_value_chirho}, ptr {current_list_ptr_tmp_chirho}"
        )
        .unwrap();
        let first_elem_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {first_elem_ptr_tmp_chirho} = alloca i1"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  store i1 true, ptr {first_elem_ptr_tmp_chirho}"
        )
        .unwrap();

        let loop_label_chirho = self.fresh_label_chirho("show.list.loop");
        let elem_label_chirho = self.fresh_label_chirho("show.list.elem");
        let sep_label_chirho = self.fresh_label_chirho("show.list.sep");
        let body_label_chirho = self.fresh_label_chirho("show.list.body");
        let done_label_chirho = self.fresh_label_chirho("show.list.done");
        writeln!(self.output_chirho, "  br label %{loop_label_chirho}").unwrap();

        writeln!(self.output_chirho, "{loop_label_chirho}:").unwrap();
        let current_list_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {current_list_tmp_chirho} = load i64, ptr {current_list_ptr_tmp_chirho}"
        )
        .unwrap();
        let current_tag_tmp_chirho = self.load_constructor_tag_chirho(&current_list_tmp_chirho);
        let is_empty_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {is_empty_tmp_chirho} = icmp eq i64 {current_tag_tmp_chirho}, {}",
            constructor_tag_chirho("[]")
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  br i1 {is_empty_tmp_chirho}, label %{done_label_chirho}, label %{elem_label_chirho}"
        )
        .unwrap();

        writeln!(self.output_chirho, "{elem_label_chirho}:").unwrap();
        let is_first_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {is_first_tmp_chirho} = load i1, ptr {first_elem_ptr_tmp_chirho}"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  br i1 {is_first_tmp_chirho}, label %{body_label_chirho}, label %{sep_label_chirho}"
        )
        .unwrap();

        writeln!(self.output_chirho, "{sep_label_chirho}:").unwrap();
        self.emit_push_byte_chirho(&buf_tmp_chirho, &index_ptr_tmp_chirho, b',');
        writeln!(self.output_chirho, "  br label %{body_label_chirho}").unwrap();

        writeln!(self.output_chirho, "{body_label_chirho}:").unwrap();
        writeln!(
            self.output_chirho,
            "  store i1 false, ptr {first_elem_ptr_tmp_chirho}"
        )
        .unwrap();
        let head_value_tmp_chirho =
            self.load_boxed_constructor_field_chirho(&current_list_tmp_chirho, 1);
        let head_ptr_tmp_chirho =
            self.emit_show_value_ptr_chirho(&head_value_tmp_chirho, element_kind_chirho.clone());
        self.emit_append_c_string_chirho(
            &buf_tmp_chirho,
            &index_ptr_tmp_chirho,
            &head_ptr_tmp_chirho,
        );
        let tail_value_tmp_chirho =
            self.load_boxed_constructor_field_chirho(&current_list_tmp_chirho, 2);
        writeln!(
            self.output_chirho,
            "  store i64 {tail_value_tmp_chirho}, ptr {current_list_ptr_tmp_chirho}"
        )
        .unwrap();
        writeln!(self.output_chirho, "  br label %{loop_label_chirho}").unwrap();

        writeln!(self.output_chirho, "{done_label_chirho}:").unwrap();
        self.emit_push_byte_chirho(&buf_tmp_chirho, &index_ptr_tmp_chirho, b']');
        let final_index_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {final_index_tmp_chirho} = load i64, ptr {index_ptr_tmp_chirho}"
        )
        .unwrap();
        self.emit_store_byte_at_index_chirho(&buf_tmp_chirho, &final_index_tmp_chirho, 0);
        buf_tmp_chirho
    }

    fn compile_case_lit_chirho(
        &mut self,
        scrut_val_chirho: &str,
        alts_chirho: &[CoreAltChirho],
    ) -> String {
        // Build a switch on the scrutinee for literal alts
        let end_label_chirho = self.fresh_label_chirho("case.end");
        let default_label_chirho = self.fresh_label_chirho("case.default");

        let mut switch_arms_chirho = Vec::new();
        let mut alt_labels_chirho = Vec::new();

        for alt_chirho in alts_chirho {
            match &alt_chirho.con_chirho {
                AltConChirho::LitConChirho(CoreLitChirho::IntChirho(v_chirho)) => {
                    let label_chirho = self.fresh_label_chirho("case.lit");
                    switch_arms_chirho.push(format!("    i64 {v_chirho}, label %{label_chirho}"));
                    alt_labels_chirho.push((label_chirho, alt_chirho));
                }
                AltConChirho::DefaultChirho => {
                    alt_labels_chirho.push((default_label_chirho.clone(), alt_chirho));
                }
                _ => {}
            }
        }

        // Emit switch
        writeln!(
            self.output_chirho,
            "  switch i64 {scrut_val_chirho}, label %{default_label_chirho} ["
        )
        .unwrap();
        for arm_chirho in &switch_arms_chirho {
            writeln!(self.output_chirho, "{arm_chirho}").unwrap();
        }
        writeln!(self.output_chirho, "  ]").unwrap();

        let mut incoming_values_chirho = Vec::new();
        // Emit each alt block
        for (label_chirho, alt_chirho) in &alt_labels_chirho {
            writeln!(self.output_chirho, "{label_chirho}:").unwrap();
            if alt_chirho.con_chirho == AltConChirho::DefaultChirho {
                for binder_chirho in &alt_chirho.binders_chirho {
                    self.bind_local_value_name_chirho(binder_chirho, scrut_val_chirho);
                }
            }
            let val_chirho = self.compile_expr_chirho(&alt_chirho.rhs_chirho);
            // Landing pad: compile_expr may emit many blocks for nested
            // cases, so phi must reference the actual predecessor block.
            let landing_chirho = self.fresh_label_chirho("case.lit.land");
            writeln!(self.output_chirho, "  br label %{landing_chirho}").unwrap();
            writeln!(self.output_chirho, "{landing_chirho}:").unwrap();
            incoming_values_chirho.push((landing_chirho, val_chirho));
            writeln!(self.output_chirho, "  br label %{end_label_chirho}").unwrap();
        }

        // If no default alt was present, emit an unreachable default
        if !alts_chirho
            .iter()
            .any(|a_chirho| a_chirho.con_chirho == AltConChirho::DefaultChirho)
        {
            writeln!(self.output_chirho, "{default_label_chirho}:").unwrap();
            writeln!(self.output_chirho, "  unreachable").unwrap();
        }

        writeln!(self.output_chirho, "{end_label_chirho}:").unwrap();
        let phi_tmp_chirho = self.fresh_tmp_chirho();
        let phi_args_chirho = incoming_values_chirho
            .iter()
            .map(|(label_chirho, value_chirho)| format!("[{value_chirho}, %{label_chirho}]"))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            self.output_chirho,
            "  {phi_tmp_chirho} = phi i64 {phi_args_chirho}"
        )
        .unwrap();
        phi_tmp_chirho
    }

    fn compile_case_lit_tail_chirho(
        &mut self,
        scrut_val_chirho: &str,
        alts_chirho: &[CoreAltChirho],
    ) -> TailCompileOutcomeChirho {
        let end_label_chirho = self.fresh_label_chirho("case.end");

        let default_label_chirho = self.fresh_label_chirho("case.default");
        let mut switch_arms_chirho = Vec::new();
        let mut alt_labels_chirho = Vec::new();

        for alt_chirho in alts_chirho {
            match &alt_chirho.con_chirho {
                AltConChirho::LitConChirho(CoreLitChirho::IntChirho(v_chirho)) => {
                    let label_chirho = self.fresh_label_chirho("case.lit");
                    switch_arms_chirho.push(format!("    i64 {v_chirho}, label %{label_chirho}"));
                    alt_labels_chirho.push((label_chirho, alt_chirho));
                }
                AltConChirho::DefaultChirho => {
                    alt_labels_chirho.push((default_label_chirho.clone(), alt_chirho));
                }
                _ => {}
            }
        }

        writeln!(
            self.output_chirho,
            "  switch i64 {scrut_val_chirho}, label %{default_label_chirho} ["
        )
        .unwrap();
        for arm_chirho in &switch_arms_chirho {
            writeln!(self.output_chirho, "{arm_chirho}").unwrap();
        }
        writeln!(self.output_chirho, "  ]").unwrap();

        let mut incoming_values_chirho = Vec::new();
        for (label_chirho, alt_chirho) in &alt_labels_chirho {
            writeln!(self.output_chirho, "{label_chirho}:").unwrap();
            match self.compile_tail_expr_chirho(&alt_chirho.rhs_chirho) {
                TailCompileOutcomeChirho::ValueChirho(val_chirho) => {
                    // Landing pad: compile_tail_expr may emit many blocks,
                    // so phi must reference the actual predecessor block.
                    let landing_chirho = self.fresh_label_chirho("case.lit.land");
                    writeln!(self.output_chirho, "  br label %{landing_chirho}").unwrap();
                    writeln!(self.output_chirho, "{landing_chirho}:").unwrap();
                    incoming_values_chirho.push((landing_chirho, val_chirho));
                    writeln!(self.output_chirho, "  br label %{end_label_chirho}").unwrap();
                }
                TailCompileOutcomeChirho::TerminatedChirho => {}
            }
        }

        if !alts_chirho
            .iter()
            .any(|a_chirho| a_chirho.con_chirho == AltConChirho::DefaultChirho)
        {
            writeln!(self.output_chirho, "{default_label_chirho}:").unwrap();
            writeln!(self.output_chirho, "  unreachable").unwrap();
        }

        if !incoming_values_chirho.is_empty() {
            writeln!(self.output_chirho, "{end_label_chirho}:").unwrap();
            let phi_tmp_chirho = self.fresh_tmp_chirho();
            let phi_args_chirho = incoming_values_chirho
                .iter()
                .map(|(label_chirho, value_chirho)| format!("[{value_chirho}, %{label_chirho}]"))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(
                self.output_chirho,
                "  {phi_tmp_chirho} = phi i64 {phi_args_chirho}"
            )
            .unwrap();
            TailCompileOutcomeChirho::ValueChirho(phi_tmp_chirho)
        } else {
            TailCompileOutcomeChirho::TerminatedChirho
        }
    }

    fn compile_case_data_chirho(
        &mut self,
        scrut_val_chirho: &str,
        alts_chirho: &[CoreAltChirho],
    ) -> String {
        let scrut_val_chirho =
            self.maybe_unpack_list_case_string_scrutinee_chirho(scrut_val_chirho, alts_chirho);
        let end_label_chirho = self.fresh_label_chirho("case.end");
        // For data constructors, we use the tag (encoded as i64) to branch.
        // For now, handle default-only case by just compiling the default RHS.
        if alts_chirho.len() == 1 && alts_chirho[0].con_chirho == AltConChirho::DefaultChirho {
            // Bind any alt binders to scrutinee
            for binder_chirho in &alts_chirho[0].binders_chirho {
                self.bind_local_value_name_chirho(binder_chirho, &scrut_val_chirho);
            }
            return self.compile_expr_chirho(&alts_chirho[0].rhs_chirho);
        }

        // Multi-alt with data constructors: use tag comparison chain
        // Each constructor gets a tag (True=1, False=0, etc.)
        let default_label_chirho = self.fresh_label_chirho("case.default");
        let scrut_tag_tmp_chirho = self.load_constructor_tag_chirho(&scrut_val_chirho);
        let mut incoming_values_chirho = Vec::new();

        for (i_chirho, alt_chirho) in alts_chirho.iter().enumerate() {
            match &alt_chirho.con_chirho {
                AltConChirho::DataConChirho(name_chirho) => {
                    let tag_chirho = constructor_tag_chirho(name_chirho);
                    let cmp_tmp_chirho = self.fresh_tmp_chirho();
                    let then_label_chirho = self.fresh_label_chirho("case.con");
                    let else_label_chirho = if i_chirho + 1 < alts_chirho.len() {
                        self.fresh_label_chirho("case.next")
                    } else {
                        default_label_chirho.clone()
                    };

                    writeln!(
                        self.output_chirho,
                        "  {cmp_tmp_chirho} = icmp eq i64 {scrut_tag_tmp_chirho}, {tag_chirho}"
                    )
                    .unwrap();
                    writeln!(
                        self.output_chirho,
                        "  br i1 {cmp_tmp_chirho}, label %{then_label_chirho}, label %{else_label_chirho}"
                    )
                    .unwrap();

                    writeln!(self.output_chirho, "{then_label_chirho}:").unwrap();
                    self.bind_constructor_fields_chirho(
                        &scrut_val_chirho,
                        &alt_chirho.binders_chirho,
                    );
                    let val_chirho = self.compile_expr_chirho(&alt_chirho.rhs_chirho);
                    // Use a landing pad label so the phi references the
                    // correct predecessor (compile_expr may emit many blocks).
                    let landing_chirho = self.fresh_label_chirho("case.con.land");
                    writeln!(self.output_chirho, "  br label %{landing_chirho}").unwrap();
                    writeln!(self.output_chirho, "{landing_chirho}:").unwrap();
                    incoming_values_chirho.push((landing_chirho, val_chirho));
                    writeln!(self.output_chirho, "  br label %{end_label_chirho}").unwrap();

                    if i_chirho + 1 < alts_chirho.len() {
                        writeln!(self.output_chirho, "{else_label_chirho}:").unwrap();
                    }
                }
                AltConChirho::DefaultChirho => {
                    // Bind any alt binders to scrutinee
                    for binder_chirho in &alt_chirho.binders_chirho {
                        self.bind_local_value_name_chirho(binder_chirho, &scrut_val_chirho);
                    }
                    let val_chirho = self.compile_expr_chirho(&alt_chirho.rhs_chirho);
                    // Use a landing pad label so the phi references the
                    // correct predecessor (compile_expr may emit many blocks).
                    let landing_chirho = self.fresh_label_chirho("case.def.land");
                    writeln!(self.output_chirho, "  br label %{landing_chirho}").unwrap();
                    writeln!(self.output_chirho, "{landing_chirho}:").unwrap();
                    incoming_values_chirho.push((landing_chirho, val_chirho));
                    writeln!(self.output_chirho, "  br label %{end_label_chirho}").unwrap();
                }
                _ => {}
            }
        }

        // If no default, emit unreachable
        if !alts_chirho
            .iter()
            .any(|a_chirho| a_chirho.con_chirho == AltConChirho::DefaultChirho)
        {
            writeln!(self.output_chirho, "{default_label_chirho}:").unwrap();
            writeln!(self.output_chirho, "  unreachable").unwrap();
        }

        writeln!(self.output_chirho, "{end_label_chirho}:").unwrap();
        let phi_tmp_chirho = self.fresh_tmp_chirho();
        let phi_args_chirho = incoming_values_chirho
            .iter()
            .map(|(label_chirho, value_chirho)| format!("[{value_chirho}, %{label_chirho}]"))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            self.output_chirho,
            "  {phi_tmp_chirho} = phi i64 {phi_args_chirho}"
        )
        .unwrap();
        phi_tmp_chirho
    }

    fn compile_case_data_tail_chirho(
        &mut self,
        scrut_val_chirho: &str,
        alts_chirho: &[CoreAltChirho],
    ) -> TailCompileOutcomeChirho {
        let scrut_val_chirho =
            self.maybe_unpack_list_case_string_scrutinee_chirho(scrut_val_chirho, alts_chirho);
        let end_label_chirho = self.fresh_label_chirho("case.end");

        if alts_chirho.len() == 1 && alts_chirho[0].con_chirho == AltConChirho::DefaultChirho {
            for binder_chirho in &alts_chirho[0].binders_chirho {
                self.bind_local_value_name_chirho(binder_chirho, &scrut_val_chirho);
            }
            return match self.compile_tail_expr_chirho(&alts_chirho[0].rhs_chirho) {
                TailCompileOutcomeChirho::ValueChirho(val_chirho) => {
                    TailCompileOutcomeChirho::ValueChirho(val_chirho)
                }
                TailCompileOutcomeChirho::TerminatedChirho => {
                    TailCompileOutcomeChirho::TerminatedChirho
                }
            };
        }

        let default_label_chirho = self.fresh_label_chirho("case.default");
        let scrut_tag_tmp_chirho = self.load_constructor_tag_chirho(&scrut_val_chirho);
        let mut incoming_values_chirho = Vec::new();
        let data_alts_chirho = alts_chirho
            .iter()
            .filter_map(|alt_chirho| match &alt_chirho.con_chirho {
                AltConChirho::DataConChirho(name_chirho) => Some((name_chirho, alt_chirho)),
                _ => None,
            })
            .collect::<Vec<_>>();

        for (i_chirho, (name_chirho, alt_chirho)) in data_alts_chirho.iter().enumerate() {
            let tag_chirho = constructor_tag_chirho(name_chirho);
            let cmp_tmp_chirho = self.fresh_tmp_chirho();
            let then_label_chirho = self.fresh_label_chirho("case.con");
            let else_label_chirho = if i_chirho + 1 < data_alts_chirho.len() {
                self.fresh_label_chirho("case.next")
            } else {
                default_label_chirho.clone()
            };
            writeln!(
                self.output_chirho,
                "  {cmp_tmp_chirho} = icmp eq i64 {scrut_tag_tmp_chirho}, {tag_chirho}"
            )
            .unwrap();
            writeln!(
                self.output_chirho,
                "  br i1 {cmp_tmp_chirho}, label %{then_label_chirho}, label %{else_label_chirho}"
            )
            .unwrap();

            writeln!(self.output_chirho, "{then_label_chirho}:").unwrap();
            self.bind_constructor_fields_chirho(&scrut_val_chirho, &alt_chirho.binders_chirho);
            match self.compile_tail_expr_chirho(&alt_chirho.rhs_chirho) {
                TailCompileOutcomeChirho::ValueChirho(val_chirho) => {
                    // Landing pad: compile_tail_expr may emit many blocks,
                    // so the phi must reference the actual predecessor.
                    let landing_chirho = self.fresh_label_chirho("case.con.land");
                    writeln!(self.output_chirho, "  br label %{landing_chirho}").unwrap();
                    writeln!(self.output_chirho, "{landing_chirho}:").unwrap();
                    incoming_values_chirho.push((landing_chirho, val_chirho));
                    writeln!(self.output_chirho, "  br label %{end_label_chirho}").unwrap();
                }
                TailCompileOutcomeChirho::TerminatedChirho => {}
            }

            if i_chirho + 1 < data_alts_chirho.len() {
                writeln!(self.output_chirho, "{else_label_chirho}:").unwrap();
            }
        }

        if let Some(default_alt_chirho) = alts_chirho
            .iter()
            .find(|alt_chirho| alt_chirho.con_chirho == AltConChirho::DefaultChirho)
        {
            writeln!(self.output_chirho, "{default_label_chirho}:").unwrap();
            for binder_chirho in &default_alt_chirho.binders_chirho {
                self.bind_local_value_name_chirho(binder_chirho, &scrut_val_chirho);
            }
            match self.compile_tail_expr_chirho(&default_alt_chirho.rhs_chirho) {
                TailCompileOutcomeChirho::ValueChirho(val_chirho) => {
                    // Landing pad for default alt phi predecessor.
                    let landing_chirho = self.fresh_label_chirho("case.def.land");
                    writeln!(self.output_chirho, "  br label %{landing_chirho}").unwrap();
                    writeln!(self.output_chirho, "{landing_chirho}:").unwrap();
                    incoming_values_chirho.push((landing_chirho, val_chirho));
                    writeln!(self.output_chirho, "  br label %{end_label_chirho}").unwrap();
                }
                TailCompileOutcomeChirho::TerminatedChirho => {}
            }
        } else {
            writeln!(self.output_chirho, "{default_label_chirho}:").unwrap();
            writeln!(self.output_chirho, "  unreachable").unwrap();
        }

        if !incoming_values_chirho.is_empty() {
            writeln!(self.output_chirho, "{end_label_chirho}:").unwrap();
            let phi_tmp_chirho = self.fresh_tmp_chirho();
            let phi_args_chirho = incoming_values_chirho
                .iter()
                .map(|(label_chirho, value_chirho)| format!("[{value_chirho}, %{label_chirho}]"))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(
                self.output_chirho,
                "  {phi_tmp_chirho} = phi i64 {phi_args_chirho}"
            )
            .unwrap();
            TailCompileOutcomeChirho::ValueChirho(phi_tmp_chirho)
        } else {
            TailCompileOutcomeChirho::TerminatedChirho
        }
    }

    fn load_constructor_tag_chirho(&mut self, scrut_val_chirho: &str) -> String {
        let boxed_label_chirho = self.fresh_label_chirho("case.tag.boxed");
        let immediate_label_chirho = self.fresh_label_chirho("case.tag.immediate");
        let join_label_chirho = self.fresh_label_chirho("case.tag.join");
        let is_boxed_tmp_chirho = self.fresh_tmp_chirho();
        let boxed_probe_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {boxed_probe_tmp_chirho} = call i64 @haskelujah_is_heap_ptr_chirho(i64 {scrut_val_chirho})"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {is_boxed_tmp_chirho} = icmp ne i64 {boxed_probe_tmp_chirho}, 0"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  br i1 {is_boxed_tmp_chirho}, label %{boxed_label_chirho}, label %{immediate_label_chirho}"
        )
        .unwrap();

        writeln!(self.output_chirho, "{immediate_label_chirho}:").unwrap();
        writeln!(self.output_chirho, "  br label %{join_label_chirho}").unwrap();

        writeln!(self.output_chirho, "{boxed_label_chirho}:").unwrap();
        let boxed_ptr_tmp_chirho = self.decode_boxed_constructor_ptr_chirho(scrut_val_chirho);
        let tag_ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {tag_ptr_tmp_chirho} = getelementptr i64, ptr {boxed_ptr_tmp_chirho}, i64 0"
        )
        .unwrap();
        let boxed_tag_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {boxed_tag_tmp_chirho} = load i64, ptr {tag_ptr_tmp_chirho}"
        )
        .unwrap();
        writeln!(self.output_chirho, "  br label %{join_label_chirho}").unwrap();

        writeln!(self.output_chirho, "{join_label_chirho}:").unwrap();
        let scrut_tag_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {scrut_tag_tmp_chirho} = phi i64 [{scrut_val_chirho}, %{immediate_label_chirho}], [{boxed_tag_tmp_chirho}, %{boxed_label_chirho}]"
        )
        .unwrap();
        scrut_tag_tmp_chirho
    }

    fn alts_match_list_constructors_chirho(&self, alts_chirho: &[CoreAltChirho]) -> bool {
        let mut saw_list_alt_chirho = false;
        for alt_chirho in alts_chirho {
            if let AltConChirho::DataConChirho(name_chirho) = &alt_chirho.con_chirho {
                if !matches!(name_chirho.as_str(), "[]" | ":") {
                    return false;
                }
                saw_list_alt_chirho = true;
            }
        }
        saw_list_alt_chirho
    }

    fn maybe_unpack_list_case_string_scrutinee_chirho(
        &mut self,
        scrut_val_chirho: &str,
        alts_chirho: &[CoreAltChirho],
    ) -> String {
        if !self.alts_match_list_constructors_chirho(alts_chirho) {
            return scrut_val_chirho.to_string();
        }

        let boxed_label_chirho = self.fresh_label_chirho("case.list.boxed");
        let immediate_label_chirho = self.fresh_label_chirho("case.list.immediate");
        let unpack_label_chirho = self.fresh_label_chirho("case.list.unpack");
        let join_label_chirho = self.fresh_label_chirho("case.list.join");
        let boxed_probe_tmp_chirho = self.fresh_tmp_chirho();
        let is_boxed_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {boxed_probe_tmp_chirho} = call i64 @haskelujah_is_heap_ptr_chirho(i64 {scrut_val_chirho})"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {is_boxed_tmp_chirho} = icmp ne i64 {boxed_probe_tmp_chirho}, 0"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  br i1 {is_boxed_tmp_chirho}, label %{boxed_label_chirho}, label %{immediate_label_chirho}"
        )
        .unwrap();

        writeln!(self.output_chirho, "{boxed_label_chirho}:").unwrap();
        writeln!(self.output_chirho, "  br label %{join_label_chirho}").unwrap();

        writeln!(self.output_chirho, "{immediate_label_chirho}:").unwrap();
        let is_zero_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {is_zero_tmp_chirho} = icmp eq i64 {scrut_val_chirho}, 0"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  br i1 {is_zero_tmp_chirho}, label %{join_label_chirho}, label %{unpack_label_chirho}"
        )
        .unwrap();

        writeln!(self.output_chirho, "{unpack_label_chirho}:").unwrap();
        let unpacked_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {unpacked_tmp_chirho} = call i64 @haskelujah_unpack_string_chirho(i64 {scrut_val_chirho})"
        )
        .unwrap();
        writeln!(self.output_chirho, "  br label %{join_label_chirho}").unwrap();

        writeln!(self.output_chirho, "{join_label_chirho}:").unwrap();
        let normalized_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {normalized_tmp_chirho} = phi i64 [{scrut_val_chirho}, %{boxed_label_chirho}], [0, %{immediate_label_chirho}], [{unpacked_tmp_chirho}, %{unpack_label_chirho}]"
        )
        .unwrap();
        normalized_tmp_chirho
    }

    fn decode_boxed_constructor_ptr_chirho(&mut self, scrut_val_chirho: &str) -> String {
        let ptr_bits_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {ptr_bits_tmp_chirho} = and i64 {scrut_val_chirho}, {BOXED_CONSTRUCTOR_PTR_MASK_CHIRHO}"
        )
        .unwrap();
        let ptr_tmp_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {ptr_tmp_chirho} = inttoptr i64 {ptr_bits_tmp_chirho} to ptr"
        )
        .unwrap();
        ptr_tmp_chirho
    }

    fn bind_constructor_fields_chirho(
        &mut self,
        scrut_val_chirho: &str,
        binders_chirho: &[haskelujah_core_chirho::BinderChirho],
    ) {
        if binders_chirho.is_empty() {
            return;
        }

        let boxed_ptr_tmp_chirho = self.decode_boxed_constructor_ptr_chirho(scrut_val_chirho);
        for (field_idx_chirho, binder_chirho) in binders_chirho.iter().enumerate() {
            let field_ptr_tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {field_ptr_tmp_chirho} = getelementptr i64, ptr {boxed_ptr_tmp_chirho}, i64 {}",
                field_idx_chirho + 1
            )
            .unwrap();
            let field_val_tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {field_val_tmp_chirho} = load i64, ptr {field_ptr_tmp_chirho}"
            )
            .unwrap();
            self.bind_local_value_name_chirho(binder_chirho, &field_val_tmp_chirho);
        }
    }
}

/// Collect all lambda parameters from a nested chain of Lam nodes.
fn collect_lambda_params_chirho(
    expr_chirho: &CoreExprChirho,
) -> (Vec<CoreIdChirho>, &CoreExprChirho) {
    let (binders_chirho, body_chirho) = collect_lambda_binders_chirho(expr_chirho);
    let params_chirho = binders_chirho
        .into_iter()
        .map(|binder_chirho| binder_chirho.id_chirho)
        .collect();
    (params_chirho, body_chirho)
}

fn is_runtime_lambda_chirho(expr_chirho: &CoreExprChirho) -> bool {
    !collect_lambda_binders_chirho(expr_chirho).0.is_empty()
}

fn collect_lambda_binders_chirho(
    expr_chirho: &CoreExprChirho,
) -> (Vec<&BinderChirho>, &CoreExprChirho) {
    let mut binders_chirho = Vec::new();
    let mut current_chirho = expr_chirho;

    loop {
        match current_chirho {
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                binders_chirho.push(binder_chirho);
                current_chirho = body_chirho;
            }
            // Type lambdas are erased at runtime but can wrap real value
            // parameters on dictionary selectors and polymorphic helpers.
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                current_chirho = body_chirho;
            }
            _ => break,
        }
    }

    (binders_chirho, current_chirho)
}

fn builtin_runtime_arity_by_name_chirho(name_chirho: &str) -> Option<usize> {
    match name_chirho {
        "getLine" | "getLine#" => Some(0),
        "readFile" | "readFile#" => Some(1),
        "writeFile" | "writeFile#" => Some(2),
        "return" | "pure" | "returnIO#" => Some(1),
        ">>" | "thenIO#" => Some(2),
        ">>=" | "bindIO#" => Some(2),
        "undefined" | "undefined#" => Some(0),
        "error" | "error#" => Some(1),
        "+" | "-" | "*" => Some(2),
        _ => None,
    }
}

fn is_dictionary_param_name_chirho(name_chirho: &str) -> bool {
    name_chirho.starts_with("$d") || name_chirho.starts_with("$dict")
}

/// Flatten nested applications into callee + argument list.
fn flatten_app_chirho(expr_chirho: &CoreExprChirho) -> (&CoreExprChirho, Vec<&CoreExprChirho>) {
    let mut args_chirho = Vec::new();
    let mut current_chirho = expr_chirho;

    while let CoreExprChirho::AppChirho {
        fun_chirho,
        arg_chirho,
    } = current_chirho
    {
        args_chirho.push(arg_chirho.as_ref());
        current_chirho = fun_chirho;
    }

    args_chirho.reverse();
    (current_chirho, args_chirho)
}

fn strip_runtime_tyapps_chirho(mut expr_chirho: &CoreExprChirho) -> &CoreExprChirho {
    while let CoreExprChirho::TyAppChirho {
        expr_chirho: inner_chirho,
        ..
    } = expr_chirho
    {
        expr_chirho = inner_chirho;
    }
    expr_chirho
}

fn encode_float_literal_chirho(value_chirho: f64) -> String {
    let bits_chirho = value_chirho.to_bits();
    let signed_bits_chirho = i64::from_ne_bytes(bits_chirho.to_ne_bytes());
    signed_bits_chirho.to_string()
}

fn classify_basic_ty_chirho(ty_chirho: &TyChirho) -> Option<ShowBuiltinKindChirho> {
    match ty_chirho {
        TyChirho::ConChirho(name_chirho) if name_chirho == "Int" => {
            Some(ShowBuiltinKindChirho::IntChirho)
        }
        TyChirho::ConChirho(name_chirho) if name_chirho == "Bool" => {
            Some(ShowBuiltinKindChirho::BoolChirho)
        }
        TyChirho::ConChirho(name_chirho) if name_chirho == "Ordering" => {
            Some(ShowBuiltinKindChirho::OrderingChirho)
        }
        TyChirho::ConChirho(name_chirho) if name_chirho == "Char" => {
            Some(ShowBuiltinKindChirho::CharChirho)
        }
        TyChirho::ConChirho(name_chirho) if name_chirho == "Double" || name_chirho == "Float" => {
            Some(ShowBuiltinKindChirho::DoubleChirho)
        }
        TyChirho::ListChirho(inner_ty_chirho) => {
            classify_basic_ty_chirho(inner_ty_chirho).map(|inner_kind_chirho| {
                ShowBuiltinKindChirho::ListChirho(Box::new(inner_kind_chirho))
            })
        }
        _ => None,
    }
}

fn classify_show_builtin_kind_chirho(
    binding_name_chirho: &str,
    param_binders_chirho: &[&BinderChirho],
) -> Option<ShowBuiltinKindChirho> {
    match binding_name_chirho {
        "showInt#" => Some(ShowBuiltinKindChirho::IntChirho),
        "showBool#" => Some(ShowBuiltinKindChirho::BoolChirho),
        "showOrdering#" => Some(ShowBuiltinKindChirho::OrderingChirho),
        "showChar#" => Some(ShowBuiltinKindChirho::CharChirho),
        "showFloat#" => Some(ShowBuiltinKindChirho::DoubleChirho),
        "show" => {
            let param_ty_chirho = &param_binders_chirho.last()?.ty_chirho;
            classify_basic_ty_chirho(param_ty_chirho)
        }
        _ => None,
    }
}

fn classify_print_builtin_kind_chirho(
    param_binders_chirho: &[&BinderChirho],
) -> Option<ShowBuiltinKindChirho> {
    let param_ty_chirho = &param_binders_chirho.last()?.ty_chirho;
    classify_basic_ty_chirho(param_ty_chirho)
}

fn escape_llvm_string_bytes_chirho(bytes_chirho: &[u8]) -> String {
    let mut escaped_chirho = String::new();
    for byte_chirho in bytes_chirho {
        match byte_chirho {
            b' '..=b'~' if *byte_chirho != b'\\' && *byte_chirho != b'"' => {
                escaped_chirho.push(char::from(*byte_chirho));
            }
            _ => {
                write!(escaped_chirho, "\\{:02X}", byte_chirho).unwrap();
            }
        }
    }
    escaped_chirho
}

/// Get a numeric tag for a data constructor name.
fn constructor_tag_chirho(name_chirho: &str) -> i64 {
    match name_chirho {
        "False" => 0,
        "True" => 1,
        "Nothing" => 0,
        "Just" => 1,
        "Left" => 0,
        "Right" => 1,
        "LT" => 0,
        "EQ" => 1,
        "GT" => 2,
        "[]" => 0,
        ":" => 1,
        "()" | "$tuple0" => 0,
        "(,)" => 0,
        "(,,)" => 0,
        _ => {
            // Hash the name to get a tag (placeholder strategy)
            let mut hash_chirho: i64 = 0;
            for byte_chirho in name_chirho.bytes() {
                hash_chirho = hash_chirho
                    .wrapping_mul(31)
                    .wrapping_add(byte_chirho as i64);
            }
            hash_chirho.abs()
        }
    }
}

/// Mangle a Haskell name for use as an LLVM symbol.
/// Collect the head function and arguments from a curried App chain.
fn collect_app_chain_llvm_chirho(
    expr_chirho: &CoreExprChirho,
) -> (&CoreExprChirho, Vec<&CoreExprChirho>) {
    let mut args_chirho = Vec::new();
    let mut cur_chirho = expr_chirho;
    loop {
        match cur_chirho {
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                args_chirho.push(arg_chirho.as_ref());
                cur_chirho = fun_chirho.as_ref();
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                cur_chirho = inner_chirho.as_ref();
            }
            _ => break,
        }
    }
    args_chirho.reverse();
    (cur_chirho, args_chirho)
}

fn mangle_name_chirho(name_chirho: &str) -> String {
    let mut mangled_chirho = String::with_capacity(name_chirho.len() + 8);
    mangled_chirho.push_str("haskelujah_");
    for ch_chirho in name_chirho.chars() {
        match ch_chirho {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => mangled_chirho.push(ch_chirho),
            '\'' => mangled_chirho.push_str("_prime"),
            '.' => mangled_chirho.push('_'),
            _ => {
                mangled_chirho.push_str(&format!("_u{:04x}", ch_chirho as u32));
            }
        }
    }
    mangled_chirho
}

/// Compile a Core module to LLVM IR text.
pub fn compile_core_to_llvm_chirho(module_chirho: &CoreModuleChirho) -> String {
    let mut codegen_chirho = LlvmCodegenChirho::new_chirho();
    codegen_chirho.compile_module_chirho(module_chirho)
}

/// Compile a Core module to LLVM IR text with a C-compatible `main()` entry
/// point. Pure programs print the resulting `i64` via `printf`, while simple
/// Prelude IO programs run `haskelujah_main()` for side effects and return `0`.
/// The resulting IR can be linked with the RTS static library to produce a
/// native executable.
///
/// Currently emits only bindings transitively reachable from `main` since the
/// LLVM backend doesn't yet support closures/heap needed by the full Prelude.
pub fn compile_core_to_llvm_executable_chirho(module_chirho: &CoreModuleChirho) -> String {
    try_compile_core_to_llvm_executable_chirho(module_chirho)
        .expect("LLVM executable contains an unsupported primitive")
}

/// Produce executable IR only if every emitted reachable primitive is implemented.
/// The non-executable IR preview may include unused Prelude definitions; those
/// cannot turn an unsupported operation in an actual executable into a false result.
pub fn try_compile_core_to_llvm_executable_chirho(
    module_chirho: &CoreModuleChirho,
) -> Result<String, String> {
    // Dictionary elision + reachability filtering via shared Core utility
    let mut filtered_module_chirho =
        haskelujah_core_chirho::elide_dicts_and_filter_chirho(module_chirho);
    restore_selector_bindings_chirho(module_chirho, &mut filtered_module_chirho);

    let mut codegen_chirho = LlvmCodegenChirho::new_chirho();
    let mut ir_chirho = codegen_chirho.compile_module_chirho(&filtered_module_chirho);
    let has_builtin_io_chirho =
        filtered_module_chirho
            .bindings_chirho
            .iter()
            .any(|binding_chirho| {
                matches!(
                    binding_chirho.binder_chirho.name_chirho.as_str(),
                    "putStrLn"
                        | "putStrLn#"
                        | "putStr"
                        | "putStr#"
                        | "putChar"
                        | "putChar#"
                        | "print"
                        | "interact"
                        | "getLine"
                        | "getLine#"
                        | "getChar"
                        | "getContents"
                        | "getContents#"
                        | "readFile"
                        | "writeFile"
                        | "appendFile"
                        | "return"
                        | "pure"
                        | ">>="
                        | "bindIO#"
                        | ">>"
                        | "thenIO#"
                )
            });

    // Check if there's a binding named "main" — that's the Haskell entry point
    let has_main_chirho = module_chirho
        .bindings_chirho
        .iter()
        .any(|b_chirho| b_chirho.binder_chirho.name_chirho == "main");

    if has_main_chirho {
        // Add a C main() entry point.
        writeln!(ir_chirho).unwrap();
        writeln!(ir_chirho, "; ── RTS entry point ──").unwrap();
        if !has_builtin_io_chirho {
            writeln!(
                ir_chirho,
                "@.fmt_int = private unnamed_addr constant [5 x i8] c\"%ld\\0A\\00\""
            )
            .unwrap();
        }
        writeln!(ir_chirho).unwrap();
        writeln!(ir_chirho, "define i32 @main() {{").unwrap();
        writeln!(ir_chirho, "entry:").unwrap();
        writeln!(
            ir_chirho,
            "  %main_fn = ptrtoint ptr @haskelujah_main to i64"
        )
        .unwrap();
        if has_builtin_io_chirho {
            writeln!(
                ir_chirho,
                "  call i64 @haskelujah_main_with_large_stack_chirho(i64 %main_fn)"
            )
            .unwrap();
            writeln!(ir_chirho, "  ret i32 0").unwrap();
        } else {
            writeln!(
                ir_chirho,
                "  %result = call i64 @haskelujah_main_with_large_stack_chirho(i64 %main_fn)"
            )
            .unwrap();
            writeln!(
                ir_chirho,
                "  call i32 (ptr, ...) @printf(ptr @.fmt_int, i64 %result)"
            )
            .unwrap();
            writeln!(ir_chirho, "  %exitcode = trunc i64 %result to i32").unwrap();
            writeln!(ir_chirho, "  ret i32 %exitcode").unwrap();
        }
        writeln!(ir_chirho, "}}").unwrap();
    }

    if codegen_chirho.unsupported_primitives_chirho.is_empty() {
        Ok(ir_chirho)
    } else {
        let mut unsupported_chirho = codegen_chirho
            .unsupported_primitives_chirho
            .into_iter()
            .collect::<Vec<_>>();
        unsupported_chirho.sort();
        Err(format!(
            "LLVM does not support primitive(s): {}",
            unsupported_chirho.join(", ")
        ))
    }
}

fn restore_selector_bindings_chirho(
    original_module_chirho: &CoreModuleChirho,
    filtered_module_chirho: &mut CoreModuleChirho,
) {
    let original_bindings_by_id_chirho: HashMap<CoreIdChirho, &CoreBindingChirho> =
        original_module_chirho
            .bindings_chirho
            .iter()
            .map(|binding_chirho| (binding_chirho.binder_chirho.id_chirho, binding_chirho))
            .collect();

    for binding_chirho in &mut filtered_module_chirho.bindings_chirho {
        if !binding_chirho
            .binder_chirho
            .name_chirho
            .starts_with("$sel_")
        {
            continue;
        }
        if let Some(original_binding_chirho) =
            original_bindings_by_id_chirho.get(&binding_chirho.binder_chirho.id_chirho)
        {
            binding_chirho.rhs_chirho = original_binding_chirho.rhs_chirho.clone();
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    mod closures_chirho;
    mod io_chirho;

    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    use haskelujah_core_chirho::CoreBindingChirho;
    use haskelujah_core_chirho::{BinderChirho, CoreIdChirho, InlineAnnotationChirho};
    use haskelujah_typing_chirho::ty_chirho::TyChirho;

    fn dummy_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
        BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn int_lit_chirho(v_chirho: i64) -> CoreExprChirho {
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(v_chirho))
    }

    fn wildcard_first_column_safe_divide_module_chirho() -> CoreModuleChirho {
        let safe_divide_binder_chirho = dummy_binder_chirho("safeDivideChirho", 7);
        let first_arg_binder_chirho = dummy_binder_chirho("_arg0", 2);
        let second_arg_binder_chirho = dummy_binder_chirho("_arg1", 3);
        let case_binder_chirho = dummy_binder_chirho("wild", 4);
        let print_binder_chirho = dummy_binder_chirho("print", 5);
        let print_arg_binder_chirho = dummy_binder_chirho("printArgChirho", 6);
        let result_binder_chirho = dummy_binder_chirho("resultChirho", 9);

        CoreModuleChirho {
            name_chirho: "WildcardFirstColumn".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("main", 0),
                    rhs_chirho: CoreExprChirho::LetChirho {
                        rec_chirho: false,
                        binds_chirho: vec![(
                            result_binder_chirho.clone(),
                            CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                        safe_divide_binder_chirho.id_chirho,
                                    )),
                                    arg_chirho: Box::new(int_lit_chirho(10)),
                                }),
                                arg_chirho: Box::new(int_lit_chirho(2)),
                            },
                        )],
                        body_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                print_binder_chirho.id_chirho,
                            )),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                result_binder_chirho.id_chirho,
                            )),
                        }),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: safe_divide_binder_chirho,
                    rhs_chirho: CoreExprChirho::LamChirho {
                        binder_chirho: first_arg_binder_chirho,
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: second_arg_binder_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::CaseChirho {
                                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                                    second_arg_binder_chirho.id_chirho,
                                )),
                                bind_chirho: case_binder_chirho,
                                result_ty_chirho: TyChirho::int_chirho(),
                                alts_chirho: vec![
                                    CoreAltChirho {
                                        con_chirho: AltConChirho::LitConChirho(
                                            CoreLitChirho::IntChirho(0),
                                        ),
                                        binders_chirho: vec![],
                                        rhs_chirho: int_lit_chirho(0),
                                    },
                                    CoreAltChirho {
                                        con_chirho: AltConChirho::DefaultChirho,
                                        binders_chirho: vec![],
                                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                                            name_chirho: "div#".to_string(),
                                            args_chirho: vec![
                                                CoreExprChirho::VarChirho(CoreIdChirho(8)),
                                                CoreExprChirho::VarChirho(
                                                    second_arg_binder_chirho.id_chirho,
                                                ),
                                            ],
                                        },
                                    },
                                ],
                            }),
                        }),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: print_binder_chirho,
                    rhs_chirho: CoreExprChirho::LamChirho {
                        binder_chirho: print_arg_binder_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "print#".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(
                                print_arg_binder_chirho.id_chirho,
                            )],
                        }),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        }
    }

    fn run_executable_module_result_chirho(
        module_chirho: &CoreModuleChirho,
    ) -> (i32, String, String) {
        let ir_chirho = compile_core_to_llvm_executable_chirho(module_chirho);
        let unique_suffix_chirho = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let temp_dir_chirho = std::env::temp_dir().join(format!(
            "haskelujah-backend-llvm-test-{}-{}",
            std::process::id(),
            unique_suffix_chirho
        ));
        fs::create_dir_all(&temp_dir_chirho).expect("should create temp dir for llvm test");

        let ll_path_chirho = temp_dir_chirho.join("main.ll");
        let exe_path_chirho = temp_dir_chirho.join("main.out");
        fs::write(&ll_path_chirho, ir_chirho).expect("should write llvm ir");

        let rts_lib_dir_chirho = ensure_rts_staticlib_chirho();
        let clang_output_chirho = Command::new("clang")
            .args([
                "-target",
                &haskelujah_rts_chirho::target_chirho::native_target_chirho(),
            ])
            .arg("-O0")
            .arg(&ll_path_chirho)
            .arg("-o")
            .arg(&exe_path_chirho)
            .arg("-L")
            .arg(&rts_lib_dir_chirho)
            .arg("-lhaskelujah_rts")
            .output()
            .expect("clang should be available for llvm backend tests");
        assert!(
            clang_output_chirho.status.success(),
            "clang failed:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&clang_output_chirho.stdout),
            String::from_utf8_lossy(&clang_output_chirho.stderr)
        );
        assert!(
            clang_output_chirho.stderr.is_empty(),
            "native link diagnostics: {}",
            String::from_utf8_lossy(&clang_output_chirho.stderr)
        );

        let run_output_chirho = Command::new(&exe_path_chirho)
            .output()
            .expect("linked executable should run");
        let _ = fs::remove_dir_all(&temp_dir_chirho);

        (
            run_output_chirho.status.code().unwrap_or(-1),
            String::from_utf8(run_output_chirho.stdout).expect("program stdout should be utf-8"),
            String::from_utf8(run_output_chirho.stderr).expect("program stderr should be utf-8"),
        )
    }

    fn run_executable_module_chirho(module_chirho: &CoreModuleChirho) -> String {
        let (exit_code_chirho, stdout_chirho, stderr_chirho) =
            run_executable_module_result_chirho(module_chirho);
        assert!(
            exit_code_chirho == 0,
            "executable failed with exit code {exit_code_chirho}:\nstdout:\n{stdout_chirho}\nstderr:\n{stderr_chirho}"
        );
        stdout_chirho
    }

    fn ensure_rts_staticlib_chirho() -> PathBuf {
        let workspace_root_chirho = workspace_root_chirho();
        let cargo_status_chirho = Command::new("cargo")
            .current_dir(&workspace_root_chirho)
            .args(["build", "-p", "haskelujah-rts", "--quiet"])
            .status()
            .expect("cargo should be available to build the RTS staticlib");
        assert!(
            cargo_status_chirho.success(),
            "cargo build -p haskelujah-rts failed with exit code {}",
            cargo_status_chirho.code().unwrap_or(-1)
        );
        workspace_root_chirho.join("target").join("debug")
    }

    fn workspace_root_chirho() -> PathBuf {
        let crate_dir_chirho = Path::new(env!("CARGO_MANIFEST_DIR"));
        crate_dir_chirho
            .parent()
            .and_then(Path::parent)
            .expect("backend llvm crate should live under workspace/crates")
            .to_path_buf()
    }

    #[test]
    fn compile_simple_constant_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 0),
                rhs_chirho: int_lit_chirho(42),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("; ModuleID = 'Test'"));
        assert!(ir_chirho.contains("define i64 @haskelujah_main()"));
        assert!(ir_chirho.contains("ret i64 42"));
        assert!(ir_chirho.contains("declare i64 @haskelujah_alloc_thunk_chirho(i64, i64, ptr)"));
        assert!(ir_chirho.contains("declare i64 @haskelujah_enter_thunk_chirho(i64)"));
        assert!(ir_chirho.contains("declare void @haskelujah_update_thunk_chirho(i64, i64)"));
    }

    #[test]
    fn compile_identity_function_chirho() {
        // f x = x
        let module_chirho = CoreModuleChirho {
            name_chirho: "Identity".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 10),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("x", 0),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("define i64 @haskelujah_f(i64 %v0.entry_chirho)"));
        assert!(ir_chirho.contains("tail.loop."));
        assert!(ir_chirho.contains("ret i64 %v0"));
    }

    #[test]
    fn compile_let_binding_chirho() {
        // main = let x = 42 in x
        let module_chirho = CoreModuleChirho {
            name_chirho: "LetTest".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 10),
                rhs_chirho: CoreExprChirho::LetChirho {
                    rec_chirho: false,
                    binds_chirho: vec![(dummy_binder_chirho("x", 0), int_lit_chirho(42))],
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("define i64 @haskelujah_main()"));
        assert!(ir_chirho.contains("; let x = 42"));
    }

    #[test]
    fn compile_float_literal_as_i64_bits_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "FloatTest".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(3.5)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("ret i64 4615063718147915776"));
        assert!(!ir_chirho.contains("ret i64 0"));
    }

    #[test]
    fn compile_string_literal_as_global_ptr_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "StringTest".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                    "Hello\n".to_string(),
                )),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains(
            "@.str.0 = private unnamed_addr constant [7 x i8] c\"Hello\\0A\\00\", align 1"
        ));
        assert!(ir_chirho.contains("ret i64 ptrtoint (ptr @.str.0 to i64)"));
    }

    #[test]
    fn compile_undefined_argument_resolves_to_runtime_abort_chirho() {
        let ignore_binder_chirho = dummy_binder_chirho("ignore", 1);
        let arg_binder_chirho = dummy_binder_chirho("arg", 2);
        let undefined_id_chirho = CoreIdChirho(5);
        let mut names_chirho = HashMap::new();
        names_chirho.insert(undefined_id_chirho, "undefined".to_string());
        let module_chirho = CoreModuleChirho {
            name_chirho: "UndefinedArgTest".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("main", 0),
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                            ignore_binder_chirho.id_chirho,
                        )),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(undefined_id_chirho)),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: ignore_binder_chirho,
                    rhs_chirho: CoreExprChirho::LamChirho {
                        binder_chirho: arg_binder_chirho,
                        body_chirho: Box::new(int_lit_chirho(1)),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho,
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("define i64 @haskelujah_undefined()"));
        assert!(ir_chirho.contains("call i64 @haskelujah_undefined()"));
        assert!(!ir_chirho.contains("@haskelujah_ignore(i64 %v5)"));
    }

    #[test]
    fn compile_recursive_local_lambda_uses_lifted_symbol_chirho() {
        let factorial_binder_chirho = dummy_binder_chirho("factorial", 1);
        let arg_binder_chirho = dummy_binder_chirho("n", 0);
        let wild_binder_chirho = dummy_binder_chirho("wild", 2);
        let recursive_call_arg_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "-#".to_string(),
            args_chirho: vec![
                CoreExprChirho::VarChirho(arg_binder_chirho.id_chirho),
                int_lit_chirho(1),
            ],
        };
        let recursive_call_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(factorial_binder_chirho.id_chirho)),
            arg_chirho: Box::new(recursive_call_arg_chirho),
        };
        let factorial_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(arg_binder_chirho.id_chirho)),
            bind_chirho: wild_binder_chirho,
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(0)),
                    binders_chirho: vec![],
                    rhs_chirho: int_lit_chirho(1),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::PrimOpChirho {
                        name_chirho: "*#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(arg_binder_chirho.id_chirho),
                            recursive_call_chirho,
                        ],
                    },
                },
            ],
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "RecursiveLetTest".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 10),
                rhs_chirho: CoreExprChirho::LetChirho {
                    rec_chirho: true,
                    binds_chirho: vec![(
                        factorial_binder_chirho.clone(),
                        CoreExprChirho::LamChirho {
                            binder_chirho: arg_binder_chirho.clone(),
                            body_chirho: Box::new(factorial_body_chirho),
                        },
                    )],
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                            factorial_binder_chirho.id_chirho,
                        )),
                        arg_chirho: Box::new(int_lit_chirho(10)),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("define i64 @haskelujah_lambda_0(i64 %v0.entry_chirho)"));
        assert!(ir_chirho.contains("ptrtoint ptr @haskelujah_lambda_0 to i64"));
        assert!(
            !ir_chirho.contains("inttoptr i64 %v1 to ptr"),
            "recursive local helper should resolve through lifted symbol, got:\n{ir_chirho}"
        );
    }

    #[test]
    fn compile_nested_local_lambda_emits_nested_lifted_defs_chirho() {
        let sum_to_binder_chirho = dummy_binder_chirho("sumTo", 10);
        let outer_binder_chirho = dummy_binder_chirho("outer", 1);
        let go_binder_chirho = dummy_binder_chirho("go", 2);
        let n_binder_chirho = dummy_binder_chirho("n", 0);
        let outer_arg_binder_chirho = dummy_binder_chirho("m", 3);
        let acc_binder_chirho = dummy_binder_chirho("acc", 4);
        let wild_binder_chirho = dummy_binder_chirho("wild", 5);
        let go_arg_binder_chirho = dummy_binder_chirho("k", 6);
        let go_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(go_arg_binder_chirho.id_chirho)),
            bind_chirho: wild_binder_chirho,
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(0)),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::VarChirho(acc_binder_chirho.id_chirho),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                go_binder_chirho.id_chirho,
                            )),
                            arg_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                                name_chirho: "-#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(go_arg_binder_chirho.id_chirho),
                                    int_lit_chirho(1),
                                ],
                            }),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "+#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(acc_binder_chirho.id_chirho),
                                CoreExprChirho::VarChirho(go_arg_binder_chirho.id_chirho),
                            ],
                        }),
                    },
                },
            ],
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "NestedLetTest".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: sum_to_binder_chirho,
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: n_binder_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LetChirho {
                        rec_chirho: false,
                        binds_chirho: vec![(
                            outer_binder_chirho.clone(),
                            CoreExprChirho::LamChirho {
                                binder_chirho: outer_arg_binder_chirho.clone(),
                                body_chirho: Box::new(CoreExprChirho::LetChirho {
                                    rec_chirho: true,
                                    binds_chirho: vec![(
                                        go_binder_chirho.clone(),
                                        CoreExprChirho::LamChirho {
                                            binder_chirho: go_arg_binder_chirho.clone(),
                                            body_chirho: Box::new(CoreExprChirho::LamChirho {
                                                binder_chirho: acc_binder_chirho.clone(),
                                                body_chirho: Box::new(go_body_chirho),
                                            }),
                                        },
                                    )],
                                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                            fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                                go_binder_chirho.id_chirho,
                                            )),
                                            arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                                outer_arg_binder_chirho.id_chirho,
                                            )),
                                        }),
                                        arg_chirho: Box::new(int_lit_chirho(0)),
                                    }),
                                }),
                            },
                        )],
                        body_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                outer_binder_chirho.id_chirho,
                            )),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                n_binder_chirho.id_chirho,
                            )),
                        }),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(
            ir_chirho.matches("define i64 @haskelujah_lambda_").count() >= 2,
            "nested where should emit both outer and inner lifted helpers, got:\n{ir_chirho}"
        );
        assert!(
            ir_chirho.contains("ptrtoint ptr @haskelujah_lambda_"),
            "nested where body should reference a lifted helper symbol, got:\n{ir_chirho}"
        );
    }

    #[test]
    fn compile_case_multi_field_constructor_binders_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "CtorCaseTest".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 10),
                rhs_chirho: CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::ConAppChirho {
                        con_name_chirho: "Pair".to_string(),
                        args_chirho: vec![int_lit_chirho(10), int_lit_chirho(32)],
                    }),
                    bind_chirho: dummy_binder_chirho("pair", 99),
                    result_ty_chirho: TyChirho::int_chirho(),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Pair".to_string()),
                        binders_chirho: vec![
                            dummy_binder_chirho("x", 0),
                            dummy_binder_chirho("y", 1),
                        ],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "+#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(CoreIdChirho(0)),
                                CoreExprChirho::VarChirho(CoreIdChirho(1)),
                            ],
                        },
                    }],
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("declare ptr @haskelujah_alloc_chirho(i64)"));
        assert!(ir_chirho.contains("call ptr @haskelujah_alloc_chirho(i64 24)"));
        assert!(ir_chirho.contains("call void @haskelujah_gc_root_push_chirho(ptr"));
        assert!(ir_chirho.contains("call void @haskelujah_gc_root_pop_chirho()"));
        assert!(ir_chirho.contains("phi i64"));
        assert!(ir_chirho.contains("load i64, ptr"));
        // The +# primop produces an add on the constructor fields.
        // Variable names may vary depending on codegen path.
        assert!(ir_chirho.contains("add i64"));
        assert!(!ir_chirho.contains("stub field"));
    }

    #[test]
    fn compile_case_literal_chirho() {
        // main = case 1 of { 1 -> 10; _ -> 20 }
        let module_chirho = CoreModuleChirho {
            name_chirho: "CaseTest".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 10),
                rhs_chirho: CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(int_lit_chirho(1)),
                    bind_chirho: dummy_binder_chirho("wild", 99),
                    result_ty_chirho: TyChirho::int_chirho(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(1)),
                            binders_chirho: vec![],
                            rhs_chirho: int_lit_chirho(10),
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: int_lit_chirho(20),
                        },
                    ],
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("switch i64"));
        assert!(ir_chirho.contains("i64 1, label"));
    }

    #[test]
    fn mangle_name_basic_chirho() {
        assert_eq!(mangle_name_chirho("main"), "haskelujah_main");
        assert_eq!(mangle_name_chirho("f'"), "haskelujah_f_prime");
        assert_eq!(mangle_name_chirho("Data.Map"), "haskelujah_Data_Map");
    }

    #[test]
    fn constructor_tags_chirho() {
        assert_eq!(constructor_tag_chirho("False"), 0);
        assert_eq!(constructor_tag_chirho("True"), 1);
        assert_eq!(constructor_tag_chirho("Nothing"), 0);
        assert_eq!(constructor_tag_chirho("Just"), 1);
        assert_eq!(constructor_tag_chirho("LT"), 0);
        assert_eq!(constructor_tag_chirho("EQ"), 1);
        assert_eq!(constructor_tag_chirho("GT"), 2);
        assert_eq!(constructor_tag_chirho("[]"), 0);
        assert_eq!(constructor_tag_chirho(":"), 1);
    }

    #[test]
    fn shared_native_thunk_header_matches_cranelift_layout_chirho() {
        let thunk_header_chirho = pack_native_header_chirho(0x1234, ObjectKindChirho::ThunkChirho);
        let fun_header_chirho = pack_native_header_chirho(0x1234, ObjectKindChirho::FunChirho);
        let con_header_chirho = pack_native_header_chirho(0x1234, ObjectKindChirho::ConChirho);
        let pap_header_chirho = pack_native_header_chirho(0x1234, ObjectKindChirho::PapChirho);

        assert_eq!(thunk_header_chirho & 0b11, 0b00);
        assert_eq!(fun_header_chirho & 0b11, 0b01);
        assert_eq!(con_header_chirho & 0b11, 0b10);
        assert_eq!(pap_header_chirho & 0b11, 0b11);
    }

    #[test]
    fn compile_executable_with_main_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Main".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 0),
                rhs_chirho: int_lit_chirho(42),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_executable_chirho(&module_chirho);
        // Should contain the haskelujah_main function
        assert!(ir_chirho.contains("define i64 @haskelujah_main()"));
        // Should contain the C main entry point
        assert!(ir_chirho.contains("define i32 @main()"));
        assert!(ir_chirho.contains("ptrtoint ptr @haskelujah_main to i64"));
        assert!(
            ir_chirho.contains("call i64 @haskelujah_main_with_large_stack_chirho(i64 %main_fn)")
        );
        assert!(ir_chirho.contains("@printf"));
        assert!(ir_chirho.contains("ret i32 %exitcode"));
    }

    #[test]
    fn compile_executable_without_main_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Lib".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("helper", 0),
                rhs_chirho: int_lit_chirho(99),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_executable_chirho(&module_chirho);
        // Without a `main` binding, no user bindings are reachable,
        // so no functions should be emitted (only module header)
        assert!(!ir_chirho.contains("define i64 @haskelujah_helper()"));
        // Should NOT contain C main entry point since no "main" binding
        assert!(!ir_chirho.contains("define i32 @main()"));
    }

    #[test]
    fn compile_executable_with_arithmetic_chirho() {
        // main = 2 + 3
        let module_chirho = CoreModuleChirho {
            name_chirho: "Arith".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 10),
                rhs_chirho: CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![int_lit_chirho(2), int_lit_chirho(3)],
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let (status_chirho, _stdout_chirho, stderr_chirho) =
            run_executable_module_result_chirho(&module_chirho);
        assert_eq!(status_chirho, 5, "{stderr_chirho}");
    }

    #[test]
    fn compile_executable_with_all_basic_arithmetic_primops_chirho() {
        let cases_chirho = [
            ("+#", 5, 2, 3),
            ("-#", 42, 50, 8),
            ("*#", 42, 6, 7),
            ("div#", 42, 84, 2),
            ("mod#", 42, 127, 85),
        ];

        for (prim_name_chirho, expected_chirho, lhs_chirho, rhs_chirho) in cases_chirho {
            let module_chirho = CoreModuleChirho {
                name_chirho: format!("PrimOp{}", prim_name_chirho),
                bindings_chirho: vec![CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("main", 10),
                    rhs_chirho: CoreExprChirho::PrimOpChirho {
                        name_chirho: prim_name_chirho.to_string(),
                        args_chirho: vec![int_lit_chirho(lhs_chirho), int_lit_chirho(rhs_chirho)],
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                }],
                names_chirho: HashMap::new(),
                specialize_pragmas_chirho: HashMap::new(),
                foreign_exports_chirho: vec![],
            };

            let (status_chirho, _stdout_chirho, stderr_chirho) =
                run_executable_module_result_chirho(&module_chirho);
            assert_eq!(
                status_chirho, expected_chirho,
                "{prim_name_chirho}: {stderr_chirho}"
            );
        }
    }

    #[test]
    fn compile_executable_int_div_mod_aliases_use_floor_helper_chirho() {
        for (prim_name_chirho, expected_chirho) in [("divInt#", 252), ("modInt#", 1)] {
            let module_chirho = CoreModuleChirho {
                name_chirho: format!("PrimOp{}", prim_name_chirho),
                bindings_chirho: vec![CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("main", 10),
                    rhs_chirho: CoreExprChirho::PrimOpChirho {
                        name_chirho: prim_name_chirho.to_string(),
                        args_chirho: vec![int_lit_chirho(-7), int_lit_chirho(2)],
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                }],
                names_chirho: HashMap::new(),
                specialize_pragmas_chirho: HashMap::new(),
                foreign_exports_chirho: vec![],
            };

            // -7 `div` 2 is -4 (exit status 252); `mod` is 1. Truncating
            // division would produce -3 and -1, so this checks floor semantics
            // even when operands reach the instruction through forced thunks.
            let (status_chirho, _stdout_chirho, stderr_chirho) =
                run_executable_module_result_chirho(&module_chirho);
            assert_eq!(
                status_chirho, expected_chirho,
                "{prim_name_chirho}: {stderr_chirho}"
            );
        }
    }

    #[test]
    fn compile_executable_with_all_basic_comparison_primops_chirho() {
        let cases_chirho = [
            ("==#", 1, 5, 5),
            ("==#", 0, 5, 4),
            ("/=#", 1, 5, 4),
            ("/=#", 0, 5, 5),
            ("<#", 1, 2, 3),
            ("<#", 0, 3, 2),
            ("<=#", 1, 2, 2),
            ("<=#", 0, 3, 2),
            (">#", 1, 3, 2),
            (">#", 0, 2, 3),
            (">=#", 1, 3, 3),
            (">=#", 0, 2, 3),
        ];

        for (prim_name_chirho, expected_chirho, lhs_chirho, rhs_chirho) in cases_chirho {
            let module_chirho = CoreModuleChirho {
                name_chirho: format!("Cmp{}", prim_name_chirho),
                bindings_chirho: vec![CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("main", 10),
                    rhs_chirho: CoreExprChirho::PrimOpChirho {
                        name_chirho: prim_name_chirho.to_string(),
                        args_chirho: vec![int_lit_chirho(lhs_chirho), int_lit_chirho(rhs_chirho)],
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                }],
                names_chirho: HashMap::new(),
                specialize_pragmas_chirho: HashMap::new(),
                foreign_exports_chirho: vec![],
            };

            let (status_chirho, _stdout_chirho, stderr_chirho) =
                run_executable_module_result_chirho(&module_chirho);
            assert_eq!(
                status_chirho, expected_chirho,
                "{prim_name_chirho}: {stderr_chirho}"
            );
        }
    }

    #[test]
    fn compile_executable_not_bool_primop_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Not".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 10),
                rhs_chirho: CoreExprChirho::PrimOpChirho {
                    name_chirho: "not#".to_string(),
                    args_chirho: vec![int_lit_chirho(1)],
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let (status_chirho, _stdout_chirho, stderr_chirho) =
            run_executable_module_result_chirho(&module_chirho);
        assert_eq!(status_chirho, 0, "{stderr_chirho}");
    }

    #[test]
    fn compile_wildcard_first_column_function_aliases_unbound_rhs_var_chirho() {
        let module_chirho = wildcard_first_column_safe_divide_module_chirho();

        assert_eq!(run_executable_module_chirho(&module_chirho).trim(), "5");
    }

    #[test]
    fn run_executable_wildcard_first_column_function_prints_division_result_chirho() {
        let module_chirho = wildcard_first_column_safe_divide_module_chirho();

        let stdout_chirho = run_executable_module_chirho(&module_chirho);
        assert_eq!(stdout_chirho.trim(), "5");
    }

    #[test]
    fn compile_executable_show_fallback_is_definition_not_alias_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Main".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 0),
                rhs_chirho: int_lit_chirho(42),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_executable_chirho(&module_chirho);
        assert!(ir_chirho.contains("define i64 @haskelujah_show(i64 %value_chirho)"));
        assert!(ir_chirho.contains("call i64 @haskelujah_show_int_chirho"));
        assert!(
            !ir_chirho.contains("@haskelujah_show = alias"),
            "LLVM aliases cannot target external declarations:\n{ir_chirho}"
        );
    }

    #[test]
    fn compile_executable_show_int_uses_snprintf_chirho() {
        let show_binder_chirho = dummy_binder_chirho("show", 1);
        let arg_binder_chirho = dummy_binder_chirho("arg", 2);
        let module_chirho = CoreModuleChirho {
            name_chirho: "Main".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("main", 0),
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                            show_binder_chirho.id_chirho,
                        )),
                        arg_chirho: Box::new(int_lit_chirho(42)),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: show_binder_chirho.clone(),
                    rhs_chirho: CoreExprChirho::LamChirho {
                        binder_chirho: arg_binder_chirho,
                        body_chirho: Box::new(int_lit_chirho(0)),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_executable_chirho(&module_chirho);
        assert!(ir_chirho.contains("declare i32 @snprintf(ptr, i64, ptr, ...)"));
        assert!(ir_chirho.contains("define i64 @haskelujah_show(i64 %v2)"));
        assert!(ir_chirho.contains("call ptr @haskelujah_alloc_chirho(i64 32)"));
        assert!(ir_chirho.contains("@snprintf(ptr"));
    }

    #[test]
    fn run_executable_partial_application_of_lambda_function_outputs_value_chirho() {
        let print_binder_chirho = dummy_binder_chirho("print", 5);
        let print_arg_binder_chirho = dummy_binder_chirho("printArgChirho", 6);
        let make_adder_binder_chirho = dummy_binder_chirho("makeAdderChirho", 1);
        let x_binder_chirho = dummy_binder_chirho("xChirho", 2);
        let y_binder_chirho = dummy_binder_chirho("yChirho", 3);
        let add1_binder_chirho = dummy_binder_chirho("add1Chirho", 4);
        let module_chirho = CoreModuleChirho {
            name_chirho: "ReturnedClosureRun".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("main", 0),
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                            print_binder_chirho.id_chirho,
                        )),
                        arg_chirho: Box::new(CoreExprChirho::LetChirho {
                            rec_chirho: false,
                            binds_chirho: vec![(
                                add1_binder_chirho.clone(),
                                CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                        make_adder_binder_chirho.id_chirho,
                                    )),
                                    arg_chirho: Box::new(int_lit_chirho(41)),
                                },
                            )],
                            body_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                    add1_binder_chirho.id_chirho,
                                )),
                                arg_chirho: Box::new(int_lit_chirho(1)),
                            }),
                        }),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: make_adder_binder_chirho,
                    rhs_chirho: CoreExprChirho::LamChirho {
                        binder_chirho: x_binder_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: y_binder_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                                name_chirho: "+#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(x_binder_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(y_binder_chirho.id_chirho),
                                ],
                            }),
                        }),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: print_binder_chirho,
                    rhs_chirho: CoreExprChirho::LamChirho {
                        binder_chirho: print_arg_binder_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "print#".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(
                                print_arg_binder_chirho.id_chirho,
                            )],
                        }),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let stdout_chirho = run_executable_module_chirho(&module_chirho);
        assert_eq!(stdout_chirho.trim(), "42");
    }

    #[test]
    fn compile_ty_lam_wrapped_runtime_params_chirho() {
        let dict_binder_chirho = dummy_binder_chirho("dNum", 1);
        let arg_binder_chirho = dummy_binder_chirho("x", 2);
        let module_chirho = CoreModuleChirho {
            name_chirho: "Selector".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("$sel_Num_fromInteger", 0),
                rhs_chirho: CoreExprChirho::TyLamChirho {
                    ty_var_chirho: "a".to_string(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: dict_binder_chirho,
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: arg_binder_chirho,
                            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(2))),
                        }),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(
            ir_chirho.contains(
                "define i64 @haskelujah__u0024sel_Num_fromInteger(i64 %v1.entry_chirho, i64 %v2.entry_chirho)"
            )
        );
        assert!(ir_chirho.contains("ret i64 %v2"));
    }

    #[test]
    fn compile_overapplied_selector_as_direct_then_indirect_call_chirho() {
        let dict_binder_chirho = BinderChirho {
            id_chirho: CoreIdChirho(1),
            name_chirho: "d".to_string(),
            ty_chirho: TyChirho::ConChirho("$Dict_Num".to_string()),
            span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
        };
        let method_binder_chirho = BinderChirho {
            id_chirho: CoreIdChirho(2),
            name_chirho: "fromInteger".to_string(),
            ty_chirho: TyChirho::ConChirho("Int".to_string()),
            span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
        };
        let arg_binder_chirho = dummy_binder_chirho("x", 3);
        let selector_binder_chirho = BinderChirho {
            id_chirho: CoreIdChirho(10),
            name_chirho: "$sel_Num_fromInteger".to_string(),
            ty_chirho: TyChirho::ConChirho("Selector".to_string()),
            span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "SelectorApply".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: selector_binder_chirho.clone(),
                    rhs_chirho: CoreExprChirho::LamChirho {
                        binder_chirho: dict_binder_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::VarChirho(
                            method_binder_chirho.id_chirho,
                        )),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: method_binder_chirho.clone(),
                    rhs_chirho: CoreExprChirho::LamChirho {
                        binder_chirho: arg_binder_chirho,
                        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(3))),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("main", 0),
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                selector_binder_chirho.id_chirho,
                            )),
                            arg_chirho: Box::new(CoreExprChirho::ConAppChirho {
                                con_name_chirho: "$Dict_Num".to_string(),
                                args_chirho: vec![int_lit_chirho(0)],
                            }),
                        }),
                        arg_chirho: Box::new(int_lit_chirho(42)),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("call i64 @haskelujah__u0024sel_Num_fromInteger(i64"));
        assert!(ir_chirho.contains("inttoptr i64 %t"));
        assert!(
            !ir_chirho.contains("call i64 @haskelujah__u0024sel_Num_fromInteger(i64 %t0, i64 42)")
        );
    }

    #[test]
    fn restore_selector_bindings_restores_original_rhs_chirho() {
        let dict_binder_chirho = BinderChirho {
            id_chirho: CoreIdChirho(1),
            name_chirho: "$dict".to_string(),
            ty_chirho: TyChirho::ConChirho("$Dict_Num".to_string()),
            span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
        };
        let method_binder_chirho = BinderChirho {
            id_chirho: CoreIdChirho(2),
            name_chirho: "fromInteger".to_string(),
            ty_chirho: TyChirho::ConChirho("Int".to_string()),
            span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
        };
        let selector_binder_chirho = BinderChirho {
            id_chirho: CoreIdChirho(10),
            name_chirho: "$sel_Num_fromInteger".to_string(),
            ty_chirho: TyChirho::ConChirho("Selector".to_string()),
            span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = CoreModuleChirho {
            name_chirho: "SelectorExec".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: selector_binder_chirho.clone(),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: dict_binder_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                            dict_binder_chirho.id_chirho,
                        )),
                        bind_chirho: dummy_binder_chirho("wild", 11),
                        result_ty_chirho: TyChirho::ConChirho("Selector".to_string()),
                        alts_chirho: vec![CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("$Dict_Num".to_string()),
                            binders_chirho: vec![
                                dummy_binder_chirho("f0", 12),
                                dummy_binder_chirho("f1", 13),
                                dummy_binder_chirho("f2", 14),
                                dummy_binder_chirho("f3", 15),
                                dummy_binder_chirho("f4", 16),
                                dummy_binder_chirho("f5", 17),
                                method_binder_chirho.clone(),
                                dummy_binder_chirho("f7", 18),
                                dummy_binder_chirho("f8", 19),
                            ],
                            rhs_chirho: CoreExprChirho::VarChirho(method_binder_chirho.id_chirho),
                        }],
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };
        let mut filtered_module_chirho = CoreModuleChirho {
            name_chirho: module_chirho.name_chirho.clone(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: selector_binder_chirho,
                rhs_chirho: CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                        dict_binder_chirho.id_chirho,
                    )),
                    bind_chirho: dummy_binder_chirho("wild", 11),
                    result_ty_chirho: TyChirho::ConChirho("Selector".to_string()),
                    alts_chirho: vec![],
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        restore_selector_bindings_chirho(&module_chirho, &mut filtered_module_chirho);

        match &filtered_module_chirho.bindings_chirho[0].rhs_chirho {
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                assert_eq!(binder_chirho.id_chirho, dict_binder_chirho.id_chirho);
                assert!(matches!(
                    body_chirho.as_ref(),
                    CoreExprChirho::CaseChirho { .. }
                ));
            }
            other_chirho => panic!("expected restored selector lambda, got {other_chirho:?}"),
        }
    }

    #[test]
    fn flatten_app_basic_chirho() {
        // f 1 2 → (f, [1, 2])
        let expr_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                arg_chirho: Box::new(int_lit_chirho(1)),
            }),
            arg_chirho: Box::new(int_lit_chirho(2)),
        };
        let (callee_chirho, args_chirho) = flatten_app_chirho(&expr_chirho);
        assert!(matches!(
            callee_chirho,
            CoreExprChirho::VarChirho(CoreIdChirho(0))
        ));
        assert_eq!(args_chirho.len(), 2);
    }
}
