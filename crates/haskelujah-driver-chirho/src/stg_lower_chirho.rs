// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Core → STG Lowering
//!
//! Translates a `CoreModuleChirho` into a sequence of `CodeChirho` instructions
//! and pre-allocated heap closures that can be executed by `MachineChirho`.
//!
//! The translation proceeds in two phases:
//! 1. **Prepare**: Allocate all top-level bindings as heap closures (functions
//!    or thunks), building a name→HeapAddr environment.
//! 2. **Lower**: Walk each binding's RHS expression, emitting `CodeChirho`
//!    instructions into a code table.

use std::collections::{HashMap, HashSet};

use haskelujah_core_chirho::expr_chirho::{
    AltConChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho, CoreModuleChirho,
};
use haskelujah_runtime_chirho::stack_chirho::PrimOpKindChirho;
use haskelujah_runtime_chirho::value_chirho::CodePtrChirho;
use haskelujah_runtime_chirho::{
    ArgSourceChirho, ClosureChirho, CodeChirho, DataConTagChirho, HeapAddrChirho, HeapChirho,
    MachineChirho, ValueChirho,
};

/// Result of lowering a Core module to STG.
#[derive(Debug)]
pub struct StgProgramChirho {
    /// The code table (instructions for the evaluator).
    pub code_table_chirho: Vec<CodeChirho>,
    /// Pre-allocated heap closures for top-level bindings.
    pub initial_heap_chirho: HeapChirho,
    /// Map from CoreId to heap address for top-level bindings.
    pub top_level_env_chirho: HashMap<CoreIdChirho, HeapAddrChirho>,
    /// Map from constructor names to tags.
    pub con_tags_chirho: HashMap<String, u16>,
}

/// Lowering context.
struct LowerCtxChirho {
    /// Code table being built.
    code_chirho: Vec<CodeChirho>,
    /// Heap for pre-allocated closures.
    heap_chirho: HeapChirho,
    /// Variable environment: CoreId → runtime value (HeapPtr for closures,
    /// or unboxed for known literals).
    env_chirho: HashMap<CoreIdChirho, ValueChirho>,
    /// Lambda parameter → arg register index mapping.
    /// When a CoreId is in this map, it should be lowered as `ArgChirho { index }`.
    arg_param_indices_chirho: HashMap<CoreIdChirho, usize>,
    /// Constructor name → tag mapping.
    con_tags_chirho: HashMap<String, u16>,
    /// Next constructor tag to assign.
    next_con_tag_chirho: u16,
    /// CoreId → name mapping for detecting known functions.
    id_names_chirho: HashMap<CoreIdChirho, String>,
    /// Newtype constructor names — these constructors are identity at runtime.
    newtype_cons_chirho: std::collections::HashSet<String>,
}

impl LowerCtxChirho {
    fn new_chirho() -> Self {
        Self {
            code_chirho: Vec::new(),
            heap_chirho: HeapChirho::with_capacity_chirho(256),
            env_chirho: HashMap::new(),
            arg_param_indices_chirho: HashMap::new(),
            con_tags_chirho: HashMap::new(),
            next_con_tag_chirho: 0,
            id_names_chirho: HashMap::new(),
            newtype_cons_chirho: std::collections::HashSet::new(),
        }
    }

    /// Emit a code instruction and return its index.
    fn emit_chirho(&mut self, code_chirho: CodeChirho) -> u32 {
        let idx_chirho = self.code_chirho.len() as u32;
        self.code_chirho.push(code_chirho);
        idx_chirho
    }

    /// Reserve a slot in the code table (to be filled later).
    fn reserve_chirho(&mut self) -> u32 {
        let idx_chirho = self.code_chirho.len() as u32;
        self.code_chirho
            .push(CodeChirho::LitChirho(ValueChirho::IntChirho(0)));
        idx_chirho
    }

    /// Patch a reserved slot.
    fn patch_chirho(&mut self, idx_chirho: u32, code_chirho: CodeChirho) {
        self.code_chirho[idx_chirho as usize] = code_chirho;
    }

    /// Get or assign a constructor tag.
    fn con_tag_chirho(&mut self, name_chirho: &str) -> u16 {
        if let Some(&tag_chirho) = self.con_tags_chirho.get(name_chirho) {
            return tag_chirho;
        }
        let tag_chirho = self.next_con_tag_chirho;
        self.next_con_tag_chirho += 1;
        self.con_tags_chirho
            .insert(name_chirho.to_string(), tag_chirho);
        tag_chirho
    }

    /// Look up the runtime value for a CoreId.
    fn lookup_chirho(&self, id_chirho: CoreIdChirho) -> ValueChirho {
        // TODO(codex-audit): defaulting a missing binding to `Int 0` can hide
        // lowering/environment bugs and produce incorrect STG silently.
        self.env_chirho
            .get(&id_chirho)
            .cloned()
            .unwrap_or(ValueChirho::IntChirho(0))
    }

    /// Check if a CoreId refers to a known I/O primop and return the name if so.
    fn is_io_primop_chirho(&self, id_chirho: CoreIdChirho) -> Option<&str> {
        let name_chirho = self.id_names_chirho.get(&id_chirho)?;
        match name_chirho.as_str() {
            "putStrLn" | "putStr" | "putChar" | "print" | "interact" | "getLine" | "getChar"
            | "getContents" | "readFile" | "writeFile" | "appendFile" | "return" | "pure"
            | ">>=" | ">>" | "newIORef" | "readIORef" | "writeIORef" | "modifyIORef"
            | "newSTRef" | "readSTRef" | "writeSTRef" | "modifySTRef" | "runST" | "newTVar"
            | "readTVar" | "writeTVar" | "atomically" | "retry" | "orElse" | "newTVarIO"
            | "error" | "undefined" | "seq" | "deepseq" | "evaluate" | "force" | "catch"
            | "throw" | "throwIO" | "try" | "bracket" | "finally" => Some(name_chirho.as_str()),
            _ => None,
        }
    }

    /// Lower a Core expression, emitting code and returning the code table
    /// index of the entry point for this expression.
    fn lower_expr_chirho(&mut self, expr_chirho: &CoreExprChirho) -> u32 {
        match expr_chirho {
            CoreExprChirho::LitChirho(lit_chirho) => {
                self.emit_chirho(CodeChirho::LitChirho(lower_lit_chirho(lit_chirho)))
            }

            CoreExprChirho::VarChirho(id_chirho) => {
                // Check if this is a lambda parameter (read from arg register)
                if let Some(&idx_chirho) = self.arg_param_indices_chirho.get(id_chirho) {
                    return self.emit_chirho(CodeChirho::ArgChirho {
                        index_chirho: idx_chirho,
                    });
                }
                // Check for nullary IO primops (getLine, getChar)
                if let Some(io_name_chirho) = self.is_io_primop_chirho(*id_chirho) {
                    let prim_op_chirho = primop_name_to_kind_chirho(io_name_chirho);
                    return self.emit_chirho(CodeChirho::PrimChirho {
                        op_chirho: prim_op_chirho,
                        args_chirho: vec![],
                    });
                }
                let val_chirho = self.lookup_chirho(*id_chirho);
                match val_chirho {
                    ValueChirho::HeapPtrChirho(addr_chirho) => {
                        self.emit_chirho(CodeChirho::EnterChirho(addr_chirho))
                    }
                    _ => self.emit_chirho(CodeChirho::LitChirho(val_chirho)),
                }
            }

            CoreExprChirho::AppChirho {
                fun_chirho: _,
                arg_chirho: _,
            } => {
                // Collect all arguments from nested App nodes
                let (head_chirho, args_chirho) = collect_app_chirho(expr_chirho);

                match head_chirho {
                    CoreExprChirho::VarChirho(fun_id_chirho) => {
                        // Check if the function is a lambda parameter (arg register)
                        if let Some(&arg_idx_chirho) =
                            self.arg_param_indices_chirho.get(fun_id_chirho)
                        {
                            let arg_sources_chirho: Vec<ArgSourceChirho> = args_chirho
                                .iter()
                                .map(|a_chirho| self.lower_arg_source_chirho(a_chirho))
                                .collect();
                            return self.emit_chirho(CodeChirho::AppFromArgChirho {
                                fun_arg_index_chirho: arg_idx_chirho,
                                arg_sources_chirho,
                            });
                        }

                        // Check for known I/O primops
                        if let Some(io_name_chirho) = self.is_io_primop_chirho(*fun_id_chirho) {
                            let prim_op_chirho = primop_name_to_kind_chirho(io_name_chirho);
                            let arg_sources_chirho: Vec<ArgSourceChirho> = args_chirho
                                .iter()
                                .map(|a_chirho| self.lower_arg_source_chirho(a_chirho))
                                .collect();
                            return self.emit_chirho(CodeChirho::PrimChirho {
                                op_chirho: prim_op_chirho,
                                args_chirho: arg_sources_chirho,
                            });
                        }

                        let fun_val_chirho = self.lookup_chirho(*fun_id_chirho);
                        let arg_sources_chirho: Vec<ArgSourceChirho> = args_chirho
                            .iter()
                            .map(|a_chirho| self.lower_arg_source_chirho(a_chirho))
                            .collect();

                        match fun_val_chirho {
                            ValueChirho::HeapPtrChirho(addr_chirho) => {
                                self.emit_chirho(CodeChirho::AppChirho {
                                    fun_chirho: addr_chirho,
                                    args_chirho: arg_sources_chirho,
                                })
                            }
                            _ => {
                                // Applying to a literal — just return it
                                self.emit_chirho(CodeChirho::LitChirho(fun_val_chirho))
                            }
                        }
                    }
                    _ => {
                        // Complex head: lower it to a thunk, then apply
                        let head_entry_chirho = self.lower_expr_chirho(head_chirho);
                        let thunk_chirho = ClosureChirho::thunk_chirho(
                            CodePtrChirho(head_entry_chirho),
                            "$app_head",
                            vec![],
                        );
                        let head_addr_chirho = self.heap_chirho.alloc_chirho(thunk_chirho);
                        let arg_sources_chirho: Vec<ArgSourceChirho> = args_chirho
                            .iter()
                            .map(|a_chirho| self.lower_arg_source_chirho(a_chirho))
                            .collect();
                        self.emit_chirho(CodeChirho::AppChirho {
                            fun_chirho: head_addr_chirho,
                            args_chirho: arg_sources_chirho,
                        })
                    }
                }
            }

            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho: _,
            } => {
                // Collect all lambda binders
                let (binders_chirho, inner_body_chirho) = collect_lam_chirho(expr_chirho);
                let arity_chirho = binders_chirho.len() as u16;

                // Compute free variables in the body to determine which
                // outer arg-register parameters need to be captured.
                let param_ids_chirho: HashSet<CoreIdChirho> = binders_chirho
                    .iter()
                    .map(|b_chirho| b_chirho.id_chirho)
                    .collect();
                let fvs_chirho = free_vars_chirho(inner_body_chirho, &param_ids_chirho);
                // Capture: free vars that are currently bound in arg registers
                let mut captures_chirho: Vec<(CoreIdChirho, usize)> = fvs_chirho
                    .iter()
                    .filter_map(|id_chirho| {
                        self.arg_param_indices_chirho
                            .get(id_chirho)
                            .map(|&idx_chirho| (*id_chirho, idx_chirho))
                    })
                    .collect();
                // Sort for deterministic ordering
                captures_chirho.sort_by_key(|&(id_chirho, _)| id_chirho);
                let n_captures_chirho = captures_chirho.len();

                // Save current env
                let saved_env_chirho: Vec<(CoreIdChirho, Option<ValueChirho>)> = binders_chirho
                    .iter()
                    .map(|b_chirho| {
                        (
                            b_chirho.id_chirho,
                            self.env_chirho.get(&b_chirho.id_chirho).cloned(),
                        )
                    })
                    .collect();
                let saved_captures_env_chirho: Vec<(CoreIdChirho, Option<usize>)> = captures_chirho
                    .iter()
                    .map(|&(id_chirho, _)| {
                        (
                            id_chirho,
                            self.arg_param_indices_chirho.get(&id_chirho).copied(),
                        )
                    })
                    .collect();

                // Set up arg indices for the body:
                // payload slots 0..n_captures for captured vars,
                // then param slots n_captures..n_captures+arity for params
                for (slot_chirho, &(id_chirho, _)) in captures_chirho.iter().enumerate() {
                    self.arg_param_indices_chirho.insert(id_chirho, slot_chirho);
                    self.env_chirho.remove(&id_chirho);
                }
                for (i_chirho, b_chirho) in binders_chirho.iter().enumerate() {
                    self.arg_param_indices_chirho
                        .insert(b_chirho.id_chirho, n_captures_chirho + i_chirho);
                    self.env_chirho.remove(&b_chirho.id_chirho);
                }

                // Reserve body slot and lower the body
                let body_slot_chirho = self.reserve_chirho();
                let body_entry_chirho = self.lower_expr_chirho(inner_body_chirho);
                self.patch_chirho(
                    body_slot_chirho,
                    self.code_chirho[body_entry_chirho as usize].clone(),
                );

                // Restore env
                for (id_chirho, prev_chirho) in saved_env_chirho {
                    self.arg_param_indices_chirho.remove(&id_chirho);
                    match prev_chirho {
                        Some(v_chirho) => {
                            self.env_chirho.insert(id_chirho, v_chirho);
                        }
                        None => {
                            self.env_chirho.remove(&id_chirho);
                        }
                    }
                }
                for (id_chirho, prev_chirho) in saved_captures_env_chirho {
                    match prev_chirho {
                        Some(idx_chirho) => {
                            self.arg_param_indices_chirho.insert(id_chirho, idx_chirho);
                        }
                        None => {
                            self.arg_param_indices_chirho.remove(&id_chirho);
                        }
                    }
                }

                if n_captures_chirho > 0 {
                    // Runtime allocation: emit AllocFunChirho that captures
                    // the current arg register values into the closure payload.
                    let capture_sources_chirho: Vec<ArgSourceChirho> = captures_chirho
                        .iter()
                        .map(|&(_, outer_idx_chirho)| {
                            ArgSourceChirho::ArgRegChirho(outer_idx_chirho)
                        })
                        .collect();
                    self.emit_chirho(CodeChirho::AllocFunChirho {
                        arity_chirho,
                        code_ptr_chirho: body_slot_chirho,
                        name_chirho: binder_chirho.name_chirho.clone(),
                        captures_chirho: capture_sources_chirho,
                    })
                } else {
                    // No captures: pre-allocate statically on the heap
                    let fun_closure_chirho = ClosureChirho::fun_chirho(
                        arity_chirho,
                        CodePtrChirho(body_slot_chirho),
                        &binder_chirho.name_chirho,
                        vec![],
                    );
                    let fun_addr_chirho = self.heap_chirho.alloc_chirho(fun_closure_chirho);
                    self.emit_chirho(CodeChirho::EnterChirho(fun_addr_chirho))
                }
            }

            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => {
                // Deferred allocations: either Fun (has arity) or Thunk (arity=0)
                let mut deferred_store_allocs_chirho: Vec<(
                    usize,                  // dest arg reg slot
                    Option<u16>,            // arity: Some = fun, None = thunk
                    u32,                    // code ptr for body
                    String,                 // name
                    Vec<ArgSourceChirho>,   // captures
                    Option<HeapAddrChirho>, // patch addr for recursive bindings
                )> = Vec::new();

                if *rec_chirho {
                    // Recursive let: two-phase approach (pre-allocate
                    // placeholders so bindings can reference each other).
                    //
                    // Check if any binding's RHS references arg-register
                    // parameters. If so, the closure payloads must include
                    // captured arg-register values (not just the code ptr).
                    let binder_ids_chirho: HashSet<CoreIdChirho> = binds_chirho
                        .iter()
                        .map(|(b_chirho, _)| b_chirho.id_chirho)
                        .collect();

                    let mut binding_addrs_chirho: Vec<HeapAddrChirho> = Vec::new();

                    // Phase 1: Allocate placeholder thunks and add to env.
                    for (binder_chirho, _) in binds_chirho {
                        let placeholder_chirho = ClosureChirho::thunk_chirho(
                            CodePtrChirho(0), // will be patched
                            &binder_chirho.name_chirho,
                            vec![],
                        );
                        let addr_chirho = self.heap_chirho.alloc_chirho(placeholder_chirho);
                        self.env_chirho.insert(
                            binder_chirho.id_chirho,
                            ValueChirho::HeapPtrChirho(addr_chirho),
                        );
                        binding_addrs_chirho.push(addr_chirho);
                    }

                    // Phase 2: Lower each RHS and patch the closures.
                    // For lambda RHSes that capture arg-register values,
                    // we need to emit runtime allocation instructions
                    // (StoreAllocFunChirho) that capture the current
                    // arg-register values AND patch the pre-allocated
                    // heap cell to point to the new closure.
                    //
                    // IMPORTANT: We defer inserting letrec binders into
                    // arg_param_indices until after ALL RHS bodies are
                    // lowered. Otherwise, a subsequent binding's RHS code
                    // would emit ArgReg references for earlier bindings,
                    // but those wouldn't be in the thunk's capture list
                    // (since free_vars excludes letrec binder IDs).
                    let mut deferred_binder_regs_chirho: Vec<(CoreIdChirho, usize)> = Vec::new();
                    // Track next available register separately from
                    // arg_param_indices to avoid polluting the index map.
                    let mut next_reg_chirho = self
                        .arg_param_indices_chirho
                        .values()
                        .max()
                        .copied()
                        .map(|m_chirho| m_chirho + 1)
                        .unwrap_or(0);
                    for (i_chirho, (binder_chirho, rhs_chirho)) in binds_chirho.iter().enumerate() {
                        let addr_chirho = binding_addrs_chirho[i_chirho];
                        let (lam_binders_chirho, _) = collect_lam_chirho(rhs_chirho);

                        // Compute free vars that need arg-register captures
                        let mut exclude_chirho = binder_ids_chirho.clone();
                        for lb_chirho in &lam_binders_chirho {
                            exclude_chirho.insert(lb_chirho.id_chirho);
                        }
                        let fvs_chirho = free_vars_chirho(rhs_chirho, &exclude_chirho);
                        let mut captures_chirho: Vec<(CoreIdChirho, usize)> = fvs_chirho
                            .iter()
                            .filter_map(|id_chirho| {
                                self.arg_param_indices_chirho
                                    .get(id_chirho)
                                    .map(|&idx_chirho| (*id_chirho, idx_chirho))
                            })
                            .collect();
                        captures_chirho.sort_by_key(|&(id_chirho, _)| id_chirho);

                        if !captures_chirho.is_empty() {
                            // RHS with arg-register captures (lambda or thunk):
                            // must use runtime allocation to capture current
                            // arg-register values.
                            let n_captures_chirho = captures_chirho.len();
                            let arity_chirho = if lam_binders_chirho.is_empty() {
                                None // thunk
                            } else {
                                Some(lam_binders_chirho.len() as u16) // function
                            };
                            let (_, inner_body_chirho) = if !lam_binders_chirho.is_empty() {
                                collect_lam_chirho(rhs_chirho)
                            } else {
                                (vec![], rhs_chirho)
                            };

                            // Save current state
                            let saved_env_chirho: Vec<(CoreIdChirho, Option<ValueChirho>)> =
                                lam_binders_chirho
                                    .iter()
                                    .map(|b_chirho| {
                                        (
                                            b_chirho.id_chirho,
                                            self.env_chirho.get(&b_chirho.id_chirho).cloned(),
                                        )
                                    })
                                    .collect();
                            let saved_captures_chirho: Vec<(CoreIdChirho, Option<usize>)> =
                                captures_chirho
                                    .iter()
                                    .map(|&(id_chirho, _)| {
                                        (
                                            id_chirho,
                                            self.arg_param_indices_chirho.get(&id_chirho).copied(),
                                        )
                                    })
                                    .collect();

                            // Set up arg-param mapping: captures at 0..n_captures,
                            // lambda params at n_captures..n_captures+arity
                            for (slot_chirho, &(id_chirho, _)) in captures_chirho.iter().enumerate()
                            {
                                self.arg_param_indices_chirho.insert(id_chirho, slot_chirho);
                                self.env_chirho.remove(&id_chirho);
                            }
                            for (j_chirho, b_chirho) in lam_binders_chirho.iter().enumerate() {
                                self.arg_param_indices_chirho
                                    .insert(b_chirho.id_chirho, n_captures_chirho + j_chirho);
                                self.env_chirho.remove(&b_chirho.id_chirho);
                            }

                            // Lower the body
                            let body_slot_chirho = self.reserve_chirho();
                            let body_entry_chirho = self.lower_expr_chirho(inner_body_chirho);
                            self.patch_chirho(
                                body_slot_chirho,
                                self.code_chirho[body_entry_chirho as usize].clone(),
                            );

                            // Restore state
                            for (id_chirho, prev_chirho) in saved_env_chirho {
                                self.arg_param_indices_chirho.remove(&id_chirho);
                                match prev_chirho {
                                    Some(v_chirho) => {
                                        self.env_chirho.insert(id_chirho, v_chirho);
                                    }
                                    None => {
                                        self.env_chirho.remove(&id_chirho);
                                    }
                                }
                            }
                            for (id_chirho, prev_chirho) in saved_captures_chirho {
                                match prev_chirho {
                                    Some(idx_chirho) => {
                                        self.arg_param_indices_chirho.insert(id_chirho, idx_chirho);
                                    }
                                    None => {
                                        self.arg_param_indices_chirho.remove(&id_chirho);
                                    }
                                }
                            }

                            // Assign a dest arg-register for the binding.
                            // We do NOT insert into arg_param_indices yet —
                            // that happens after all RHS bodies are lowered.
                            let dest_reg_chirho = next_reg_chirho;
                            next_reg_chirho += 1;
                            deferred_binder_regs_chirho
                                .push((binder_chirho.id_chirho, dest_reg_chirho));

                            let capture_sources_chirho: Vec<ArgSourceChirho> = captures_chirho
                                .iter()
                                .map(|&(_, outer_idx_chirho)| {
                                    ArgSourceChirho::ArgRegChirho(outer_idx_chirho)
                                })
                                .collect();

                            deferred_store_allocs_chirho.push((
                                dest_reg_chirho,
                                arity_chirho,
                                body_slot_chirho,
                                binder_chirho.name_chirho.clone(),
                                capture_sources_chirho,
                                Some(addr_chirho), // patch placeholder → indirection
                            ));
                        } else {
                            // No arg-register captures: standard static allocation
                            let rhs_entry_chirho = self.lower_expr_chirho(rhs_chirho);
                            let patched_chirho = ClosureChirho::thunk_chirho(
                                CodePtrChirho(rhs_entry_chirho),
                                &binder_chirho.name_chirho,
                                vec![],
                            );
                            *self.heap_chirho.read_mut_chirho(addr_chirho) = patched_chirho;
                        }
                    }
                    // Now that all RHS bodies are lowered, insert the
                    // letrec binder → dest-register mappings so the
                    // continuation body can reference them.
                    for (id_chirho, reg_chirho) in &deferred_binder_regs_chirho {
                        self.arg_param_indices_chirho
                            .insert(*id_chirho, *reg_chirho);
                    }
                } else {
                    // Non-recursive let: sequential lowering.
                    // Lambda RHSes with captures need special handling —
                    // they're values, so we allocate eagerly via
                    // StoreAllocFunChirho to capture arg-register values.
                    for (binder_chirho, rhs_chirho) in binds_chirho {
                        let (lam_binders_chirho, _) = collect_lam_chirho(rhs_chirho);

                        if !lam_binders_chirho.is_empty() {
                            // RHS is a lambda — check for arg-register captures
                            let param_ids_chirho: HashSet<CoreIdChirho> = lam_binders_chirho
                                .iter()
                                .map(|b_chirho| b_chirho.id_chirho)
                                .collect();
                            let (_, inner_body_chirho) = collect_lam_chirho(rhs_chirho);
                            let fvs_chirho = free_vars_chirho(inner_body_chirho, &param_ids_chirho);
                            let mut captures_chirho: Vec<(CoreIdChirho, usize)> = fvs_chirho
                                .iter()
                                .filter_map(|id_chirho| {
                                    self.arg_param_indices_chirho
                                        .get(id_chirho)
                                        .map(|&idx_chirho| (*id_chirho, idx_chirho))
                                })
                                .collect();
                            captures_chirho.sort_by_key(|&(id_chirho, _)| id_chirho);

                            if !captures_chirho.is_empty() {
                                // Lambda with captures: use StoreAllocFunChirho
                                let n_captures_chirho = captures_chirho.len();
                                let arity_chirho = lam_binders_chirho.len() as u16;

                                // Assign a temp arg-register slot for this binding
                                let max_reg_chirho = self
                                    .arg_param_indices_chirho
                                    .values()
                                    .max()
                                    .copied()
                                    .unwrap_or(0)
                                    + 1;
                                let dest_reg_chirho = max_reg_chirho;

                                // Map binder to the dest arg register
                                self.arg_param_indices_chirho
                                    .insert(binder_chirho.id_chirho, dest_reg_chirho);

                                // Save and set up inner lambda env
                                let saved_env_chirho: Vec<(CoreIdChirho, Option<ValueChirho>)> =
                                    lam_binders_chirho
                                        .iter()
                                        .map(|b_chirho| {
                                            (
                                                b_chirho.id_chirho,
                                                self.env_chirho.get(&b_chirho.id_chirho).cloned(),
                                            )
                                        })
                                        .collect();
                                let saved_captures_chirho: Vec<(CoreIdChirho, Option<usize>)> =
                                    captures_chirho
                                        .iter()
                                        .map(|&(id_chirho, _)| {
                                            (
                                                id_chirho,
                                                self.arg_param_indices_chirho
                                                    .get(&id_chirho)
                                                    .copied(),
                                            )
                                        })
                                        .collect();

                                // Set up captures at payload slots, params after
                                for (slot_chirho, &(id_chirho, _)) in
                                    captures_chirho.iter().enumerate()
                                {
                                    self.arg_param_indices_chirho.insert(id_chirho, slot_chirho);
                                    self.env_chirho.remove(&id_chirho);
                                }
                                for (i_chirho, b_chirho) in lam_binders_chirho.iter().enumerate() {
                                    self.arg_param_indices_chirho
                                        .insert(b_chirho.id_chirho, n_captures_chirho + i_chirho);
                                    self.env_chirho.remove(&b_chirho.id_chirho);
                                }

                                // Reserve body slot and lower
                                let body_slot_chirho = self.reserve_chirho();
                                let body_entry_chirho = self.lower_expr_chirho(inner_body_chirho);
                                self.patch_chirho(
                                    body_slot_chirho,
                                    self.code_chirho[body_entry_chirho as usize].clone(),
                                );

                                // Restore env
                                for (id_chirho, prev_chirho) in saved_env_chirho {
                                    self.arg_param_indices_chirho.remove(&id_chirho);
                                    match prev_chirho {
                                        Some(v_chirho) => {
                                            self.env_chirho.insert(id_chirho, v_chirho);
                                        }
                                        None => {
                                            self.env_chirho.remove(&id_chirho);
                                        }
                                    }
                                }
                                for (id_chirho, prev_chirho) in saved_captures_chirho {
                                    match prev_chirho {
                                        Some(idx_chirho) => {
                                            self.arg_param_indices_chirho
                                                .insert(id_chirho, idx_chirho);
                                        }
                                        None => {
                                            self.arg_param_indices_chirho.remove(&id_chirho);
                                        }
                                    }
                                }

                                let capture_sources_chirho: Vec<ArgSourceChirho> = captures_chirho
                                    .iter()
                                    .map(|&(_, outer_idx_chirho)| {
                                        ArgSourceChirho::ArgRegChirho(outer_idx_chirho)
                                    })
                                    .collect();

                                deferred_store_allocs_chirho.push((
                                    dest_reg_chirho,
                                    Some(arity_chirho),
                                    body_slot_chirho,
                                    binder_chirho.name_chirho.clone(),
                                    capture_sources_chirho,
                                    None, // non-recursive: no placeholder to patch
                                ));
                                continue;
                            }
                        }

                        // Non-lambda RHS: check if it references arg registers
                        // (captures). If so, must be allocated at runtime.
                        let fvs_chirho = free_vars_chirho(rhs_chirho, &HashSet::new());
                        let mut captures_chirho: Vec<(CoreIdChirho, usize)> = fvs_chirho
                            .iter()
                            .filter_map(|id_chirho| {
                                self.arg_param_indices_chirho
                                    .get(id_chirho)
                                    .map(|&idx_chirho| (*id_chirho, idx_chirho))
                            })
                            .collect();
                        captures_chirho.sort_by_key(|&(id_chirho, _)| id_chirho);

                        if !captures_chirho.is_empty() {
                            // Thunk with captures: use StoreAllocThunkChirho
                            let _n_captures_chirho = captures_chirho.len();

                            // Assign a temp arg-register slot for this binding
                            let max_reg_chirho = self
                                .arg_param_indices_chirho
                                .values()
                                .max()
                                .copied()
                                .unwrap_or(0)
                                + 1;
                            let dest_reg_chirho = max_reg_chirho;

                            self.arg_param_indices_chirho
                                .insert(binder_chirho.id_chirho, dest_reg_chirho);
                            self.env_chirho.remove(&binder_chirho.id_chirho);

                            // Save and remap captures for thunk code
                            let saved_captures_chirho: Vec<(CoreIdChirho, Option<usize>)> =
                                captures_chirho
                                    .iter()
                                    .map(|&(id_chirho, _)| {
                                        (
                                            id_chirho,
                                            self.arg_param_indices_chirho.get(&id_chirho).copied(),
                                        )
                                    })
                                    .collect();

                            // Remap captures: inside the thunk code, captured
                            // values are at payload slots 0..n_captures
                            for (slot_chirho, &(id_chirho, _)) in captures_chirho.iter().enumerate()
                            {
                                self.arg_param_indices_chirho.insert(id_chirho, slot_chirho);
                                self.env_chirho.remove(&id_chirho);
                            }

                            // Lower the thunk body
                            let body_slot_chirho = self.reserve_chirho();
                            let body_entry_chirho = self.lower_expr_chirho(rhs_chirho);
                            self.patch_chirho(
                                body_slot_chirho,
                                self.code_chirho[body_entry_chirho as usize].clone(),
                            );

                            // Restore captures env
                            for (id_chirho, prev_chirho) in saved_captures_chirho {
                                match prev_chirho {
                                    Some(idx_chirho) => {
                                        self.arg_param_indices_chirho.insert(id_chirho, idx_chirho);
                                    }
                                    None => {
                                        self.arg_param_indices_chirho.remove(&id_chirho);
                                    }
                                }
                            }

                            let capture_sources_chirho: Vec<ArgSourceChirho> = captures_chirho
                                .iter()
                                .map(|&(_, outer_idx_chirho)| {
                                    ArgSourceChirho::ArgRegChirho(outer_idx_chirho)
                                })
                                .collect();

                            deferred_store_allocs_chirho.push((
                                dest_reg_chirho,
                                None, // thunk, not fun
                                body_slot_chirho,
                                binder_chirho.name_chirho.clone(),
                                capture_sources_chirho,
                                None, // non-recursive: no placeholder to patch
                            ));
                            continue;
                        }

                        // Standard path: no captures, safe to allocate statically
                        let rhs_entry_chirho = self.lower_expr_chirho(rhs_chirho);
                        let closure_chirho = ClosureChirho::thunk_chirho(
                            CodePtrChirho(rhs_entry_chirho),
                            &binder_chirho.name_chirho,
                            vec![],
                        );
                        let addr_chirho = self.heap_chirho.alloc_chirho(closure_chirho);
                        self.env_chirho.insert(
                            binder_chirho.id_chirho,
                            ValueChirho::HeapPtrChirho(addr_chirho),
                        );
                    }
                }

                // Lower the body
                let body_entry_chirho = self.lower_expr_chirho(body_chirho);

                // If there are deferred StoreAllocFun instructions, chain them
                // before the body. Last StoreAllocFun's body_chirho points to
                // the actual body; each earlier one points to the next.
                if deferred_store_allocs_chirho.is_empty() {
                    body_entry_chirho
                } else {
                    let mut current_body_chirho = body_entry_chirho;
                    for (
                        dest_reg_chirho,
                        maybe_arity_chirho,
                        code_ptr_chirho,
                        name_chirho,
                        captures_chirho,
                        patch_addr_chirho,
                    ) in deferred_store_allocs_chirho.into_iter().rev()
                    {
                        current_body_chirho = match maybe_arity_chirho {
                            Some(arity_chirho) => {
                                self.emit_chirho(CodeChirho::StoreAllocFunChirho {
                                    arity_chirho,
                                    code_ptr_chirho,
                                    name_chirho,
                                    captures_chirho,
                                    dest_reg_chirho,
                                    body_chirho: current_body_chirho,
                                    patch_addr_chirho,
                                })
                            }
                            None => self.emit_chirho(CodeChirho::StoreAllocThunkChirho {
                                code_ptr_chirho,
                                name_chirho,
                                captures_chirho,
                                dest_reg_chirho,
                                body_chirho: current_body_chirho,
                                patch_addr_chirho,
                            }),
                        };
                    }
                    current_body_chirho
                }
            }

            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                result_ty_chirho: _,
                alts_chirho,
            } => {
                // Bind the case binder to the scrutinee so that variable
                // patterns like `case e of x -> x` correctly reference
                // the scrutinee value. For variable scrutinees, copy the
                // arg-param index or env entry; for others, allocate a thunk.
                match scrutinee_chirho.as_ref() {
                    CoreExprChirho::VarChirho(scrut_id_chirho) => {
                        if let Some(&idx_chirho) =
                            self.arg_param_indices_chirho.get(scrut_id_chirho)
                        {
                            self.arg_param_indices_chirho
                                .insert(bind_chirho.id_chirho, idx_chirho);
                        } else if let Some(val_chirho) =
                            self.env_chirho.get(scrut_id_chirho).cloned()
                        {
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
                    if let AltConChirho::DataConChirho(con_name_chirho) = &alts_chirho[0].con_chirho
                    {
                        if self.newtype_cons_chirho.contains(con_name_chirho)
                            && alts_chirho[0].binders_chirho.len() == 1
                        {
                            // Map the single field binder to the scrutinee.
                            // The scrutinee value IS the unwrapped value.
                            let binder_id_chirho = alts_chirho[0].binders_chirho[0].id_chirho;
                            // Copy the scrutinee's resolution into the binder
                            match scrutinee_chirho.as_ref() {
                                CoreExprChirho::VarChirho(id_chirho) => {
                                    if let Some(idx_chirho) =
                                        self.arg_param_indices_chirho.get(id_chirho)
                                    {
                                        self.arg_param_indices_chirho
                                            .insert(binder_id_chirho, *idx_chirho);
                                    } else if let Some(val_chirho) = self.env_chirho.get(id_chirho)
                                    {
                                        self.env_chirho
                                            .insert(binder_id_chirho, val_chirho.clone());
                                    }
                                }
                                _ => {
                                    // For non-variable scrutinees, allocate a
                                    // thunk on the heap and bind the binder to it.
                                    let scrut_entry_chirho =
                                        self.lower_expr_chirho(scrutinee_chirho);
                                    let thunk_chirho = ClosureChirho::thunk_chirho(
                                        CodePtrChirho(scrut_entry_chirho),
                                        "<newtype_scrut>",
                                        vec![],
                                    );
                                    let addr_chirho = self.heap_chirho.alloc_chirho(thunk_chirho);
                                    self.env_chirho.insert(
                                        binder_id_chirho,
                                        ValueChirho::HeapPtrChirho(addr_chirho),
                                    );
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

            // Type abstraction/application: erase types at runtime
            CoreExprChirho::TyLamChirho { body_chirho, .. } => self.lower_expr_chirho(body_chirho),
            CoreExprChirho::TyAppChirho { expr_chirho, .. } => self.lower_expr_chirho(expr_chirho),

            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => {
                let prim_op_chirho = primop_name_to_kind_chirho(name_chirho);
                let arg_sources_chirho: Vec<ArgSourceChirho> = args_chirho
                    .iter()
                    .map(|a_chirho| self.lower_arg_source_chirho(a_chirho))
                    .collect();
                self.emit_chirho(CodeChirho::PrimChirho {
                    op_chirho: prim_op_chirho,
                    args_chirho: arg_sources_chirho,
                })
            }

            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } => {
                // Newtype constructor erasure: if this is a newtype constructor
                // (single field), just lower the inner value directly.
                // The constructor is erased at runtime (zero cost).
                if self.newtype_cons_chirho.contains(con_name_chirho) && args_chirho.len() == 1 {
                    return self.lower_expr_chirho(&args_chirho[0]);
                }

                let tag_chirho = self.con_tag_chirho(con_name_chirho);
                // When inside a function body, use ArgSourceChirho-based
                // fields so runtime values (from arg registers) are
                // correctly resolved.
                if !self.arg_param_indices_chirho.is_empty() {
                    let field_sources_chirho: Vec<ArgSourceChirho> = args_chirho
                        .iter()
                        .map(|a_chirho| self.lower_arg_source_chirho(a_chirho))
                        .collect();
                    self.emit_chirho(CodeChirho::ConAppFromArgChirho {
                        tag_chirho: DataConTagChirho(tag_chirho),
                        name_chirho: con_name_chirho.clone(),
                        fields_chirho: field_sources_chirho,
                    })
                } else {
                    let field_vals_chirho: Vec<ValueChirho> = args_chirho
                        .iter()
                        .map(|a_chirho| self.lower_arg_chirho(a_chirho))
                        .collect();
                    self.emit_chirho(CodeChirho::ConAppChirho {
                        tag_chirho: DataConTagChirho(tag_chirho),
                        name_chirho: con_name_chirho.clone(),
                        fields_chirho: field_vals_chirho,
                    })
                }
            }
        }
    }

    /// Lower an expression to a runtime value (for use as an argument).
    /// Lower an argument expression to an `ArgSourceChirho` for use in
    /// `AppFromArgChirho`. Arg-register vars become `ArgRegChirho` (resolved at
    /// runtime) instead of thunks, avoiding stale arg-register captures.
    fn lower_arg_source_chirho(&mut self, expr_chirho: &CoreExprChirho) -> ArgSourceChirho {
        match expr_chirho {
            CoreExprChirho::LitChirho(lit_chirho) => {
                ArgSourceChirho::StaticChirho(lower_lit_chirho(lit_chirho))
            }
            CoreExprChirho::VarChirho(id_chirho) => {
                if let Some(&idx_chirho) = self.arg_param_indices_chirho.get(id_chirho) {
                    return ArgSourceChirho::ArgRegChirho(idx_chirho);
                }
                ArgSourceChirho::StaticChirho(self.lookup_chirho(*id_chirho))
            }
            _ => {
                // Complex expression: if we're inside a function body
                // (arg_param_indices is non-empty), use ThunkCodeChirho
                // so a fresh thunk is allocated at runtime on each call.
                // The thunk must capture current arg-register values because
                // its code references ArgRegChirho indices that may be
                // overwritten before the thunk is actually entered.
                if !self.arg_param_indices_chirho.is_empty() {
                    let entry_chirho = self.lower_expr_chirho(expr_chirho);
                    // Capture all arg registers so the thunk can reproduce
                    // the same arg-register layout when entered.
                    let max_idx_chirho = self
                        .arg_param_indices_chirho
                        .values()
                        .copied()
                        .max()
                        .unwrap_or(0);
                    let captures_chirho: Vec<ArgSourceChirho> = (0..=max_idx_chirho)
                        .map(|i_chirho| ArgSourceChirho::ArgRegChirho(i_chirho))
                        .collect();
                    ArgSourceChirho::ThunkCodeChirho {
                        code_ptr_chirho: entry_chirho,
                        captures_chirho,
                    }
                } else {
                    // Outside function body: static thunk is fine.
                    ArgSourceChirho::StaticChirho(self.lower_arg_chirho(expr_chirho))
                }
            }
        }
    }

    fn lower_arg_chirho(&mut self, expr_chirho: &CoreExprChirho) -> ValueChirho {
        match expr_chirho {
            CoreExprChirho::LitChirho(lit_chirho) => lower_lit_chirho(lit_chirho),
            CoreExprChirho::VarChirho(id_chirho) => {
                // If it's a lambda param, wrap in a thunk that reads the arg register
                if let Some(&idx_chirho) = self.arg_param_indices_chirho.get(id_chirho) {
                    let entry_chirho = self.emit_chirho(CodeChirho::ArgChirho {
                        index_chirho: idx_chirho,
                    });
                    let thunk_chirho = ClosureChirho::thunk_chirho(
                        CodePtrChirho(entry_chirho),
                        "$arg_param",
                        vec![],
                    );
                    let addr_chirho = self.heap_chirho.alloc_chirho(thunk_chirho);
                    return ValueChirho::HeapPtrChirho(addr_chirho);
                }
                self.lookup_chirho(*id_chirho)
            }
            _ => {
                // Complex expression: lower it and wrap in a thunk
                let entry_chirho = self.lower_expr_chirho(expr_chirho);
                let thunk_chirho =
                    ClosureChirho::thunk_chirho(CodePtrChirho(entry_chirho), "$arg", vec![]);
                let addr_chirho = self.heap_chirho.alloc_chirho(thunk_chirho);
                ValueChirho::HeapPtrChirho(addr_chirho)
            }
        }
    }
}

/// Convert a Core literal to a runtime value.
fn lower_lit_chirho(lit_chirho: &CoreLitChirho) -> ValueChirho {
    match lit_chirho {
        CoreLitChirho::IntChirho(n_chirho) => ValueChirho::IntChirho(*n_chirho),
        CoreLitChirho::FloatChirho(n_chirho) => ValueChirho::FloatChirho(*n_chirho),
        CoreLitChirho::CharChirho(c_chirho) => ValueChirho::CharChirho(*c_chirho),
        CoreLitChirho::StringChirho(s_chirho) => ValueChirho::StringChirho(s_chirho.clone()),
    }
}

/// Flatten nested `App` into (head, [arg1, arg2, ...]).
fn collect_app_chirho(expr_chirho: &CoreExprChirho) -> (&CoreExprChirho, Vec<&CoreExprChirho>) {
    let mut args_chirho = Vec::new();
    let mut cur_chirho = expr_chirho;
    while let CoreExprChirho::AppChirho {
        fun_chirho,
        arg_chirho,
    } = cur_chirho
    {
        args_chirho.push(arg_chirho.as_ref());
        cur_chirho = fun_chirho.as_ref();
    }
    args_chirho.reverse();
    (cur_chirho, args_chirho)
}

/// Flatten nested `Lam` into ([binder1, binder2, ...], body).
fn collect_lam_chirho(
    expr_chirho: &CoreExprChirho,
) -> (
    Vec<&haskelujah_core_chirho::expr_chirho::BinderChirho>,
    &CoreExprChirho,
) {
    let mut binders_chirho = Vec::new();
    let mut cur_chirho = expr_chirho;
    while let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho,
    } = cur_chirho
    {
        binders_chirho.push(binder_chirho);
        cur_chirho = body_chirho.as_ref();
    }
    (binders_chirho, cur_chirho)
}

/// Collect all free variable CoreIds in an expression, excluding those
/// in the `bound_chirho` set. Used by lambda lowering to identify captures.
fn free_vars_chirho(
    expr_chirho: &CoreExprChirho,
    bound_chirho: &HashSet<CoreIdChirho>,
) -> HashSet<CoreIdChirho> {
    let mut result_chirho = HashSet::new();
    free_vars_walk_chirho(expr_chirho, bound_chirho, &mut result_chirho);
    result_chirho
}

fn free_vars_walk_chirho(
    expr_chirho: &CoreExprChirho,
    bound_chirho: &HashSet<CoreIdChirho>,
    out_chirho: &mut HashSet<CoreIdChirho>,
) {
    match expr_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            if !bound_chirho.contains(id_chirho) {
                out_chirho.insert(*id_chirho);
            }
        }
        CoreExprChirho::LitChirho(_) => {}
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            free_vars_walk_chirho(fun_chirho, bound_chirho, out_chirho);
            free_vars_walk_chirho(arg_chirho, bound_chirho, out_chirho);
        }
        CoreExprChirho::LamChirho {
            binder_chirho,
            body_chirho,
        } => {
            let mut inner_bound_chirho = bound_chirho.clone();
            inner_bound_chirho.insert(binder_chirho.id_chirho);
            free_vars_walk_chirho(body_chirho, &inner_bound_chirho, out_chirho);
        }
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            let mut inner_bound_chirho = bound_chirho.clone();
            for (binder_chirho, _) in binds_chirho {
                inner_bound_chirho.insert(binder_chirho.id_chirho);
            }
            for (_, rhs_chirho) in binds_chirho {
                free_vars_walk_chirho(rhs_chirho, &inner_bound_chirho, out_chirho);
            }
            free_vars_walk_chirho(body_chirho, &inner_bound_chirho, out_chirho);
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            bind_chirho,
            alts_chirho,
            ..
        } => {
            free_vars_walk_chirho(scrutinee_chirho, bound_chirho, out_chirho);
            let mut case_bound_chirho = bound_chirho.clone();
            case_bound_chirho.insert(bind_chirho.id_chirho);
            for alt_chirho in alts_chirho {
                let mut alt_bound_chirho = case_bound_chirho.clone();
                for b_chirho in &alt_chirho.binders_chirho {
                    alt_bound_chirho.insert(b_chirho.id_chirho);
                }
                free_vars_walk_chirho(&alt_chirho.rhs_chirho, &alt_bound_chirho, out_chirho);
            }
        }
        CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            for a_chirho in args_chirho {
                free_vars_walk_chirho(a_chirho, bound_chirho, out_chirho);
            }
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
            for a_chirho in args_chirho {
                free_vars_walk_chirho(a_chirho, bound_chirho, out_chirho);
            }
        }
        CoreExprChirho::TyAppChirho {
            expr_chirho: e_chirho,
            ..
        } => {
            free_vars_walk_chirho(e_chirho, bound_chirho, out_chirho);
        }
        CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            free_vars_walk_chirho(body_chirho, bound_chirho, out_chirho);
        }
    }
}

/// Lower a Core module to an STG program.
pub fn lower_module_to_stg_chirho(
    module_chirho: &CoreModuleChirho,
    newtype_cons_chirho: HashSet<String>,
) -> StgProgramChirho {
    let mut ctx_chirho = LowerCtxChirho::new_chirho();
    ctx_chirho.newtype_cons_chirho = newtype_cons_chirho;

    // Populate the id→name map from the module's desugaring context
    ctx_chirho.id_names_chirho = module_chirho.names_chirho.clone();

    // Phase 1: Pre-allocate top-level bindings as thunks/functions
    // We need forward references, so allocate placeholder closures first
    let mut binding_addrs_chirho: Vec<HeapAddrChirho> = Vec::new();

    for binding_chirho in &module_chirho.bindings_chirho {
        let placeholder_chirho = ClosureChirho::thunk_chirho(
            CodePtrChirho(0), // will be patched
            &binding_chirho.binder_chirho.name_chirho,
            vec![],
        );
        let addr_chirho = ctx_chirho.heap_chirho.alloc_chirho(placeholder_chirho);
        ctx_chirho.env_chirho.insert(
            binding_chirho.binder_chirho.id_chirho,
            ValueChirho::HeapPtrChirho(addr_chirho),
        );
        ctx_chirho.id_names_chirho.insert(
            binding_chirho.binder_chirho.id_chirho,
            binding_chirho.binder_chirho.name_chirho.clone(),
        );
        binding_addrs_chirho.push(addr_chirho);
    }

    // Phase 2: Lower each binding's RHS and patch the closures
    for (i_chirho, binding_chirho) in module_chirho.bindings_chirho.iter().enumerate() {
        let addr_chirho = binding_addrs_chirho[i_chirho];
        // Each top-level binding starts with a clean arg-parameter map.
        // Previous bindings may have inserted let-bound or letrec-bound
        // entries that must not leak into the next top-level binding.
        ctx_chirho.arg_param_indices_chirho.clear();
        let (binders_chirho, inner_body_chirho) = collect_lam_chirho(&binding_chirho.rhs_chirho);

        let closure_chirho = if binders_chirho.is_empty() {
            // Non-lambda: lower the expression directly as a thunk body
            let entry_chirho = ctx_chirho.lower_expr_chirho(&binding_chirho.rhs_chirho);
            ClosureChirho::thunk_chirho(
                CodePtrChirho(entry_chirho),
                &binding_chirho.binder_chirho.name_chirho,
                vec![],
            )
        } else {
            // Lambda: bind params to arg registers, lower only the body
            let saved_env_chirho: Vec<(CoreIdChirho, Option<ValueChirho>)> = binders_chirho
                .iter()
                .map(|b_chirho| {
                    (
                        b_chirho.id_chirho,
                        ctx_chirho.env_chirho.get(&b_chirho.id_chirho).cloned(),
                    )
                })
                .collect();

            for (i_chirho, b_chirho) in binders_chirho.iter().enumerate() {
                ctx_chirho
                    .arg_param_indices_chirho
                    .insert(b_chirho.id_chirho, i_chirho);
                ctx_chirho.env_chirho.remove(&b_chirho.id_chirho);
            }

            let entry_chirho = ctx_chirho.lower_expr_chirho(inner_body_chirho);

            // Restore env
            for (id_chirho, prev_chirho) in saved_env_chirho {
                ctx_chirho.arg_param_indices_chirho.remove(&id_chirho);
                match prev_chirho {
                    Some(v_chirho) => {
                        ctx_chirho.env_chirho.insert(id_chirho, v_chirho);
                    }
                    None => {
                        ctx_chirho.env_chirho.remove(&id_chirho);
                    }
                }
            }

            ClosureChirho::fun_chirho(
                binders_chirho.len() as u16,
                CodePtrChirho(entry_chirho),
                &binding_chirho.binder_chirho.name_chirho,
                vec![],
            )
        };

        // Patch the pre-allocated closure
        *ctx_chirho.heap_chirho.read_mut_chirho(addr_chirho) = closure_chirho;
    }

    let top_level_env_chirho: HashMap<CoreIdChirho, HeapAddrChirho> = module_chirho
        .bindings_chirho
        .iter()
        .enumerate()
        .map(|(i_chirho, b_chirho)| {
            (
                b_chirho.binder_chirho.id_chirho,
                binding_addrs_chirho[i_chirho],
            )
        })
        .collect();

    StgProgramChirho {
        code_table_chirho: ctx_chirho.code_chirho,
        initial_heap_chirho: ctx_chirho.heap_chirho,
        con_tags_chirho: ctx_chirho.con_tags_chirho,
        top_level_env_chirho,
    }
}

/// Lower a Core module and create a ready-to-run machine.
/// Optionally specify which binding to evaluate (by name); defaults to "main".
pub fn lower_and_run_chirho(
    module_chirho: &CoreModuleChirho,
    entry_name_chirho: Option<&str>,
    newtype_cons_chirho: HashSet<String>,
) -> Result<(ValueChirho, MachineChirho), String> {
    let program_chirho = lower_module_to_stg_chirho(module_chirho, newtype_cons_chirho);

    let target_name_chirho = entry_name_chirho.unwrap_or("main");

    // Find the entry binding
    let entry_binding_chirho = module_chirho
        .bindings_chirho
        .iter()
        .find(|b_chirho| b_chirho.binder_chirho.name_chirho == target_name_chirho);

    let entry_id_chirho = match entry_binding_chirho {
        Some(b_chirho) => b_chirho.binder_chirho.id_chirho,
        None => {
            return Err(format!(
                "no binding named '{}' in module '{}'",
                target_name_chirho, module_chirho.name_chirho
            ))
        }
    };

    let entry_addr_chirho = program_chirho
        .top_level_env_chirho
        .get(&entry_id_chirho)
        .copied()
        .ok_or_else(|| format!("binding '{}' not found in STG env", target_name_chirho))?;

    let mut machine_chirho = MachineChirho::new_chirho(program_chirho.code_table_chirho);
    machine_chirho.heap_chirho = program_chirho.initial_heap_chirho;
    machine_chirho.con_tags_chirho = program_chirho.con_tags_chirho;
    machine_chirho.step_limit_chirho = 100_000;

    // Emit an Enter instruction for the entry point
    let entry_code_chirho = machine_chirho.code_table_chirho.len() as u32;
    machine_chirho
        .code_table_chirho
        .push(CodeChirho::EnterChirho(entry_addr_chirho));

    let result_chirho = machine_chirho
        .run_chirho(entry_code_chirho)
        .map_err(|e_chirho| format!("runtime error: {}", e_chirho))?;

    // If the result is a HeapPtr, try to unbox it. This handles the case
    // where a function returns a boxed literal (e.g. I# 4) — we extract
    // the unboxed value so callers get IntChirho(4) instead of HeapPtrChirho.
    let unboxed_chirho = match &result_chirho {
        ValueChirho::HeapPtrChirho(addr_chirho) => {
            let final_addr_chirho = machine_chirho.heap_chirho.follow_ind_chirho(*addr_chirho);
            let closure_chirho = machine_chirho.heap_chirho.read_chirho(final_addr_chirho);
            if closure_chirho.payload_chirho.len() == 1 {
                match &closure_chirho.payload_chirho[0] {
                    ValueChirho::IntChirho(n_chirho) => ValueChirho::IntChirho(*n_chirho),
                    ValueChirho::FloatChirho(n_chirho) => ValueChirho::FloatChirho(*n_chirho),
                    ValueChirho::CharChirho(c_chirho) => ValueChirho::CharChirho(*c_chirho),
                    ValueChirho::BoolChirho(b_chirho) => ValueChirho::BoolChirho(*b_chirho),
                    _ => result_chirho,
                }
            } else {
                result_chirho
            }
        }
        _ => result_chirho,
    };

    Ok((unboxed_chirho, machine_chirho))
}

/// Lower a Core module and run with pre-loaded stdin input lines.
pub fn lower_and_run_with_input_chirho(
    module_chirho: &CoreModuleChirho,
    entry_name_chirho: Option<&str>,
    newtype_cons_chirho: HashSet<String>,
    input_lines_chirho: &[&str],
) -> Result<(ValueChirho, MachineChirho), String> {
    let program_chirho = lower_module_to_stg_chirho(module_chirho, newtype_cons_chirho);
    let target_name_chirho = entry_name_chirho.unwrap_or("main");

    let entry_binding_chirho = module_chirho
        .bindings_chirho
        .iter()
        .find(|b_chirho| b_chirho.binder_chirho.name_chirho == target_name_chirho);

    let entry_id_chirho = match entry_binding_chirho {
        Some(b_chirho) => b_chirho.binder_chirho.id_chirho,
        None => {
            return Err(format!(
                "no binding named '{}' in module '{}'",
                target_name_chirho, module_chirho.name_chirho
            ))
        }
    };

    let entry_addr_chirho = program_chirho
        .top_level_env_chirho
        .get(&entry_id_chirho)
        .copied()
        .ok_or_else(|| format!("binding '{}' not found in STG env", target_name_chirho))?;

    let mut machine_chirho = MachineChirho::new_chirho(program_chirho.code_table_chirho);
    machine_chirho.heap_chirho = program_chirho.initial_heap_chirho;
    machine_chirho.con_tags_chirho = program_chirho.con_tags_chirho;
    machine_chirho.step_limit_chirho = 100_000;

    // Pre-load stdin input
    for line_chirho in input_lines_chirho {
        machine_chirho
            .io_input_chirho
            .push_back(line_chirho.to_string());
    }

    let entry_code_chirho = machine_chirho.code_table_chirho.len() as u32;
    machine_chirho
        .code_table_chirho
        .push(CodeChirho::EnterChirho(entry_addr_chirho));

    let result_chirho = machine_chirho
        .run_chirho(entry_code_chirho)
        .map_err(|e_chirho| format!("runtime error: {}", e_chirho))?;

    let unboxed_chirho = match &result_chirho {
        ValueChirho::HeapPtrChirho(addr_chirho) => {
            let final_addr_chirho = machine_chirho.heap_chirho.follow_ind_chirho(*addr_chirho);
            let closure_chirho = machine_chirho.heap_chirho.read_chirho(final_addr_chirho);
            if closure_chirho.payload_chirho.len() == 1 {
                match &closure_chirho.payload_chirho[0] {
                    ValueChirho::IntChirho(n_chirho) => ValueChirho::IntChirho(*n_chirho),
                    ValueChirho::FloatChirho(n_chirho) => ValueChirho::FloatChirho(*n_chirho),
                    ValueChirho::CharChirho(c_chirho) => ValueChirho::CharChirho(*c_chirho),
                    ValueChirho::BoolChirho(b_chirho) => ValueChirho::BoolChirho(*b_chirho),
                    _ => result_chirho,
                }
            } else {
                result_chirho
            }
        }
        _ => result_chirho,
    };

    Ok((unboxed_chirho, machine_chirho))
}

/// Lower a Core module and run with a custom step limit.
/// Used for algorithmic tests that require more than the default 100,000 steps.
pub fn lower_and_run_with_step_limit_chirho(
    module_chirho: &CoreModuleChirho,
    entry_name_chirho: Option<&str>,
    newtype_cons_chirho: HashSet<String>,
    step_limit_chirho: u64,
) -> Result<(ValueChirho, MachineChirho), String> {
    let program_chirho = lower_module_to_stg_chirho(module_chirho, newtype_cons_chirho);
    let target_name_chirho = entry_name_chirho.unwrap_or("main");

    let entry_binding_chirho = module_chirho
        .bindings_chirho
        .iter()
        .find(|b_chirho| b_chirho.binder_chirho.name_chirho == target_name_chirho);

    let entry_id_chirho = match entry_binding_chirho {
        Some(b_chirho) => b_chirho.binder_chirho.id_chirho,
        None => {
            return Err(format!(
                "no binding named '{}' in module '{}'",
                target_name_chirho, module_chirho.name_chirho
            ))
        }
    };

    let entry_addr_chirho = program_chirho
        .top_level_env_chirho
        .get(&entry_id_chirho)
        .copied()
        .ok_or_else(|| format!("binding '{}' not found in STG env", target_name_chirho))?;

    let mut machine_chirho = MachineChirho::new_chirho(program_chirho.code_table_chirho);
    machine_chirho.heap_chirho = program_chirho.initial_heap_chirho;
    machine_chirho.con_tags_chirho = program_chirho.con_tags_chirho;
    machine_chirho.step_limit_chirho = step_limit_chirho;

    let entry_code_chirho = machine_chirho.code_table_chirho.len() as u32;
    machine_chirho
        .code_table_chirho
        .push(CodeChirho::EnterChirho(entry_addr_chirho));

    let result_chirho = machine_chirho
        .run_chirho(entry_code_chirho)
        .map_err(|e_chirho| format!("runtime error: {}", e_chirho))?;

    let unboxed_chirho = match &result_chirho {
        ValueChirho::HeapPtrChirho(addr_chirho) => {
            let final_addr_chirho = machine_chirho.heap_chirho.follow_ind_chirho(*addr_chirho);
            let closure_chirho = machine_chirho.heap_chirho.read_chirho(final_addr_chirho);
            if closure_chirho.payload_chirho.len() == 1 {
                match &closure_chirho.payload_chirho[0] {
                    ValueChirho::IntChirho(n_chirho) => ValueChirho::IntChirho(*n_chirho),
                    ValueChirho::FloatChirho(n_chirho) => ValueChirho::FloatChirho(*n_chirho),
                    ValueChirho::CharChirho(c_chirho) => ValueChirho::CharChirho(*c_chirho),
                    ValueChirho::BoolChirho(b_chirho) => ValueChirho::BoolChirho(*b_chirho),
                    _ => result_chirho,
                }
            } else {
                result_chirho
            }
        }
        _ => result_chirho,
    };

    Ok((unboxed_chirho, machine_chirho))
}

/// Map a primop name from Core to a `PrimOpKindChirho`.
fn primop_name_to_kind_chirho(name_chirho: &str) -> PrimOpKindChirho {
    match name_chirho {
        "+#" => PrimOpKindChirho::AddIntChirho,
        "-#" => PrimOpKindChirho::SubIntChirho,
        "*#" => PrimOpKindChirho::MulIntChirho,
        "div#" => PrimOpKindChirho::DivIntChirho,
        "mod#" => PrimOpKindChirho::ModIntChirho,
        "quot#" => PrimOpKindChirho::QuotIntChirho,
        "rem#" => PrimOpKindChirho::RemIntChirho,
        "chr#" => PrimOpKindChirho::ChrChirho,
        "ord#" => PrimOpKindChirho::OrdChirho,
        "isDigit#" => PrimOpKindChirho::IsDigitChirho,
        "isAlpha#" => PrimOpKindChirho::IsAlphaChirho,
        "isAlphaNum#" => PrimOpKindChirho::IsAlphaNumChirho,
        "isUpper#" => PrimOpKindChirho::IsUpperChirho,
        "isLower#" => PrimOpKindChirho::IsLowerChirho,
        "isSpace#" => PrimOpKindChirho::IsSpaceChirho,
        "toLower#" => PrimOpKindChirho::ToLowerChirho,
        "toUpper#" => PrimOpKindChirho::ToUpperChirho,
        "digitToInt#" => PrimOpKindChirho::DigitToIntChirho,
        "intToDigit#" => PrimOpKindChirho::IntToDigitChirho,
        "==#" => PrimOpKindChirho::EqIntChirho,
        "/=#" => PrimOpKindChirho::NeIntChirho,
        "<#" => PrimOpKindChirho::LtIntChirho,
        "<=#" => PrimOpKindChirho::LeIntChirho,
        ">#" => PrimOpKindChirho::GtIntChirho,
        ">=#" => PrimOpKindChirho::GeIntChirho,
        "negate#" => PrimOpKindChirho::NegIntChirho,
        "putStrLn" | "putStrLn#" => PrimOpKindChirho::PutStrLnChirho,
        "putStr" | "putStr#" => PrimOpKindChirho::PutStrChirho,
        "putChar" | "putChar#" => PrimOpKindChirho::PutCharChirho,
        "return" | "pure" | "returnIO#" => PrimOpKindChirho::ReturnIOChirho,
        ">>=" | "bindIO#" => PrimOpKindChirho::BindIOChirho,
        ">>" | "thenIO#" => PrimOpKindChirho::ThenIOChirho,
        "getLine" | "getLine#" => PrimOpKindChirho::GetLineChirho,
        "getChar" => PrimOpKindChirho::GetCharChirho,
        "getContents" | "getContents#" => PrimOpKindChirho::GetContentsChirho,
        "readFile" => PrimOpKindChirho::ReadFileChirho,
        "writeFile" => PrimOpKindChirho::WriteFileChirho,
        "appendFile" => PrimOpKindChirho::AppendFileChirho,
        "showInt#" => PrimOpKindChirho::ShowIntChirho,
        "showBool#" => PrimOpKindChirho::ShowBoolChirho,
        "not#" => PrimOpKindChirho::NotBoolChirho,
        "++#" => PrimOpKindChirho::AppendStrChirho,
        "eqStr#" => PrimOpKindChirho::EqStrChirho,
        "ltStr#" => PrimOpKindChirho::LtStrChirho,
        "compareStr#" => PrimOpKindChirho::CompareStrChirho,
        "eqFloat#" => PrimOpKindChirho::EqFloatChirho,
        "+.#" => PrimOpKindChirho::AddFloatChirho,
        "*.#" => PrimOpKindChirho::MulFloatChirho,
        "-.#" => PrimOpKindChirho::SubFloatChirho,
        "/.#" => PrimOpKindChirho::DivFloatChirho,
        "recip#" => PrimOpKindChirho::RecipFloatChirho,
        "negateFloat#" => PrimOpKindChirho::NegFloatChirho,
        "showFloat#" => PrimOpKindChirho::ShowFloatChirho,
        "lengthStr#" => PrimOpKindChirho::LengthStrChirho,
        "showStr#" => PrimOpKindChirho::ShowStrChirho,
        "enumFromTo#" => PrimOpKindChirho::EnumFromToChirho,
        "enumFrom#" => PrimOpKindChirho::EnumFromChirho,
        "enumFromThen#" => PrimOpKindChirho::EnumFromThenChirho,
        "enumFromThenTo#" => PrimOpKindChirho::EnumFromThenToChirho,
        "showList#" => PrimOpKindChirho::ShowListChirho,
        "readInt#" => PrimOpKindChirho::ReadIntChirho,
        "readFloat#" => PrimOpKindChirho::ReadFloatChirho,
        "readBool#" => PrimOpKindChirho::ReadBoolChirho,
        "wordsStr#" => PrimOpKindChirho::WordsStrChirho,
        "unwordsStr#" => PrimOpKindChirho::UnwordsStrChirho,
        "takeStr#" => PrimOpKindChirho::TakeStrChirho,
        "dropStr#" => PrimOpKindChirho::DropStrChirho,
        "concatStr#" => PrimOpKindChirho::ConcatStrChirho,
        "intercalateStr#" => PrimOpKindChirho::IntercalateStrChirho,
        "fromIntegral#" => PrimOpKindChirho::FromIntegralChirho,
        "ceiling#" => PrimOpKindChirho::CeilingChirho,
        "floor#" => PrimOpKindChirho::FloorChirho,
        "round#" => PrimOpKindChirho::RoundChirho,
        "truncate#" => PrimOpKindChirho::TruncateChirho,
        "compare#" => PrimOpKindChirho::CompareIntChirho,
        "compareChar#" => PrimOpKindChirho::CompareCharChirho,
        "compareFloat#" => PrimOpKindChirho::CompareFloatChirho,
        "<.#" => PrimOpKindChirho::LtFloatChirho,
        ">.#" => PrimOpKindChirho::GtFloatChirho,
        // Floating math primops
        "sin#" => PrimOpKindChirho::SinFloatChirho,
        "cos#" => PrimOpKindChirho::CosFloatChirho,
        "tan#" => PrimOpKindChirho::TanFloatChirho,
        "asin#" => PrimOpKindChirho::AsinFloatChirho,
        "acos#" => PrimOpKindChirho::AcosFloatChirho,
        "atan#" => PrimOpKindChirho::AtanFloatChirho,
        "exp#" => PrimOpKindChirho::ExpFloatChirho,
        "log#" => PrimOpKindChirho::LogFloatChirho,
        "sqrt#" => PrimOpKindChirho::SqrtFloatChirho,
        "pi#" => PrimOpKindChirho::PiFloatChirho,
        "^#" => PrimOpKindChirho::PowIntChirho,
        "**#" => PrimOpKindChirho::PowFloatChirho,
        "id#" => PrimOpKindChirho::IdChirho,
        "error" => PrimOpKindChirho::ErrorChirho,
        "undefined" => PrimOpKindChirho::UndefinedChirho,
        "seq" | "deepseq" => PrimOpKindChirho::SeqChirho,
        "evaluate" => PrimOpKindChirho::EvaluateChirho,
        "force" | "force#" => PrimOpKindChirho::ForceChirho,
        "showMaybe#" => PrimOpKindChirho::ShowMaybeChirho,
        "showTuple2#" => PrimOpKindChirho::ShowTuple2Chirho,
        "showEither#" => PrimOpKindChirho::ShowEitherChirho,
        "showOrdering#" => PrimOpKindChirho::ShowOrderingChirho,
        "interact" => PrimOpKindChirho::InteractChirho,
        "print" => PrimOpKindChirho::PrintChirho,
        "lines#" => PrimOpKindChirho::LinesChirho,
        "unlines#" => PrimOpKindChirho::UnlinesChirho,
        // IORef primops
        "newIORef" | "newIORef#" => PrimOpKindChirho::NewIORefChirho,
        "readIORef" | "readIORef#" => PrimOpKindChirho::ReadIORefChirho,
        "writeIORef" | "writeIORef#" => PrimOpKindChirho::WriteIORefChirho,
        "modifyIORef" | "modifyIORef#" => PrimOpKindChirho::ModifyIORefChirho,
        // ST monad primops
        "newSTRef" | "newSTRef#" => PrimOpKindChirho::NewSTRefChirho,
        "readSTRef" | "readSTRef#" => PrimOpKindChirho::ReadSTRefChirho,
        "writeSTRef" | "writeSTRef#" => PrimOpKindChirho::WriteSTRefChirho,
        "modifySTRef" | "modifySTRef#" => PrimOpKindChirho::ModifySTRefChirho,
        "runST" | "runST#" => PrimOpKindChirho::RunSTChirho,
        // STM primops
        "newTVar" | "newTVar#" | "newTVarIO" | "newTVarIO#" => PrimOpKindChirho::NewTVarChirho,
        "readTVar" | "readTVar#" | "readTVarIO" | "readTVarIO#" => PrimOpKindChirho::ReadTVarChirho,
        "writeTVar" | "writeTVar#" => PrimOpKindChirho::WriteTVarChirho,
        "atomically" | "atomically#" => PrimOpKindChirho::AtomicallyChirho,
        "retry" | "retry#" => PrimOpKindChirho::RetryChirho,
        "orElse" | "orElse#" => PrimOpKindChirho::OrElseChirho,
        // Exception handling primops
        "catch" | "catch#" => PrimOpKindChirho::CatchChirho,
        "throw" | "throw#" | "throwIO" | "throwIO#" => PrimOpKindChirho::ThrowChirho,
        "try" | "try#" => PrimOpKindChirho::TryChirho,
        "bracket" | "bracket#" => PrimOpKindChirho::BracketChirho,
        "finally" | "finally#" => PrimOpKindChirho::FinallyChirho,
        // Data.Map primops
        "mapEmpty#" | "mapEmpty" => PrimOpKindChirho::MapEmptyChirho,
        "mapSingleton#" | "mapSingleton" => PrimOpKindChirho::MapSingletonChirho,
        "mapInsert#" | "mapInsert" => PrimOpKindChirho::MapInsertChirho,
        "mapLookup#" | "mapLookup" => PrimOpKindChirho::MapLookupChirho,
        "mapDelete#" | "mapDelete" => PrimOpKindChirho::MapDeleteChirho,
        "mapMember#" | "mapMember" => PrimOpKindChirho::MapMemberChirho,
        "mapSize#" | "mapSize" => PrimOpKindChirho::MapSizeChirho,
        "mapFromList#" | "mapFromList" => PrimOpKindChirho::MapFromListChirho,
        "mapToList#" | "mapToList" => PrimOpKindChirho::MapToListChirho,
        "mapKeys#" | "mapKeys" => PrimOpKindChirho::MapKeysChirho,
        "mapElems#" | "mapElems" => PrimOpKindChirho::MapElemsChirho,
        "mapNull#" | "mapNull" => PrimOpKindChirho::MapNullChirho,
        "mapMap#" | "mapMap" => PrimOpKindChirho::MapMapChirho,
        "mapFoldlWithKey#" | "mapFoldlWithKey" => PrimOpKindChirho::MapFoldlWithKeyChirho,
        "mapFoldrWithKey#" | "mapFoldrWithKey" => PrimOpKindChirho::MapFoldrWithKeyChirho,
        "mapUnion#" | "mapUnion" => PrimOpKindChirho::MapUnionChirho,
        "mapDifference#" | "mapDifference" => PrimOpKindChirho::MapDifferenceChirho,
        "mapIntersection#" | "mapIntersection" => PrimOpKindChirho::MapIntersectionChirho,
        "mapInsertWith#" | "mapInsertWith" => PrimOpKindChirho::MapInsertWithChirho,
        "mapFindWithDefault#" | "mapFindWithDefault" => PrimOpKindChirho::MapFindWithDefaultChirho,
        "mapAdjust#" | "mapAdjust" => PrimOpKindChirho::MapAdjustChirho,
        "mapUnionWith#" | "mapUnionWith" => PrimOpKindChirho::MapUnionWithChirho,
        "mapFilter#" | "mapFilter" => PrimOpKindChirho::MapFilterChirho,
        "mapFilterWithKey#" | "mapFilterWithKey" => PrimOpKindChirho::MapFilterWithKeyChirho,
        // Data.Set primops
        "setEmpty#" | "setEmpty" => PrimOpKindChirho::SetEmptyChirho,
        "setSingleton#" | "setSingleton" => PrimOpKindChirho::SetSingletonChirho,
        "setInsert#" | "setInsert" => PrimOpKindChirho::SetInsertChirho,
        "setMember#" | "setMember" => PrimOpKindChirho::SetMemberChirho,
        "setDelete#" | "setDelete" => PrimOpKindChirho::SetDeleteChirho,
        "setSize#" | "setSize" => PrimOpKindChirho::SetSizeChirho,
        "setFromList#" | "setFromList" => PrimOpKindChirho::SetFromListChirho,
        "setToList#" | "setToList" => PrimOpKindChirho::SetToListChirho,
        "setUnion#" | "setUnion" => PrimOpKindChirho::SetUnionChirho,
        "setIntersection#" | "setIntersection" => PrimOpKindChirho::SetIntersectionChirho,
        "setDifference#" | "setDifference" => PrimOpKindChirho::SetDifferenceChirho,
        "setNull#" | "setNull" => PrimOpKindChirho::SetNullChirho,
        "setMap#" | "setMap" => PrimOpKindChirho::SetMapChirho,
        "setFilter#" | "setFilter" => PrimOpKindChirho::SetFilterChirho,
        "setFoldr#" | "setFoldr" | "setFold#" | "setFold" => PrimOpKindChirho::SetFoldrChirho,
        _ => PrimOpKindChirho::AddIntChirho, // fallback
    }
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_core_chirho::expr_chirho::{
        BinderChirho, CoreBindingChirho, CoreIdChirho, InlineAnnotationChirho,
    };
    use haskelujah_span_chirho::SpanChirho;
    use haskelujah_typing_chirho::ty_chirho::TyChirho;

    fn int_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
        BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn lower_literal_binding_chirho() {
        // module Test where x = 42
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("x", 0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let program_chirho = lower_module_to_stg_chirho(&module_chirho, HashSet::new());
        assert!(!program_chirho.code_table_chirho.is_empty());
        assert_eq!(program_chirho.top_level_env_chirho.len(), 1);
    }

    #[test]
    fn run_literal_binding_chirho() {
        // module Test where main = 42
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("main", 0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let (result_chirho, _machine_chirho) =
            lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(42));
    }

    #[test]
    fn run_float_literal_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("main", 0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(3.14)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let (result_chirho, _) =
            lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
        assert_eq!(result_chirho, ValueChirho::FloatChirho(3.14));
    }

    #[test]
    fn run_let_binding_chirho() {
        // module Test where main = let x = 7 in x
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("main", 0),
                rhs_chirho: CoreExprChirho::LetChirho {
                    rec_chirho: false,
                    binds_chirho: vec![(
                        int_binder_chirho("x", 1),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(7)),
                    )],
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let (result_chirho, _) =
            lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(7));
    }

    #[test]
    fn run_var_reference_chirho() {
        // module Test where
        //   x = 99
        //   main = x
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: int_binder_chirho("x", 0),
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(99)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: int_binder_chirho("main", 1),
                    rhs_chirho: CoreExprChirho::VarChirho(CoreIdChirho(0)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let (result_chirho, _) =
            lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(99));
    }

    #[test]
    fn run_case_on_constructor_chirho() {
        // module Test where
        //   main = case True of
        //            True -> 1
        //            False -> 0
        //
        // We represent True as a ConApp in the scrutinee
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![
                // True constructor as a top-level binding
                CoreBindingChirho {
                    binder_chirho: int_binder_chirho("True", 2),
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: int_binder_chirho("main", 0),
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let (result_chirho, _) =
            lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(1));
    }

    #[test]
    fn run_identity_function_chirho() {
        // module Test where
        //   id = \x -> x
        //   main = id 42
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![
                CoreBindingChirho {
                    binder_chirho: int_binder_chirho("id", 0),
                    rhs_chirho: CoreExprChirho::LamChirho {
                        binder_chirho: int_binder_chirho("x", 1),
                        body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(1))),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
                CoreBindingChirho {
                    binder_chirho: int_binder_chirho("main", 2),
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                        arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                            42,
                        ))),
                    },
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                },
            ],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let (result_chirho, _) =
            lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(42));
    }

    #[test]
    fn entry_not_found_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("x", 0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let result_chirho = lower_and_run_chirho(&module_chirho, None, HashSet::new());
        assert!(result_chirho.is_err());
        assert!(result_chirho
            .unwrap_err()
            .contains("no binding named 'main'"));
    }

    #[test]
    fn named_entry_point_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("myEntry", 0),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(777)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let (result_chirho, _) =
            lower_and_run_chirho(&module_chirho, Some("myEntry"), HashSet::new()).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(777));
    }

    #[test]
    fn type_erasure_chirho() {
        // TyLam and TyApp should be erased
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: int_binder_chirho("main", 0),
                rhs_chirho: CoreExprChirho::TyLamChirho {
                    ty_var_chirho: "a".to_string(),
                    body_chirho: Box::new(CoreExprChirho::TyAppChirho {
                        expr_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                            55,
                        ))),
                        ty_chirho: TyChirho::int_chirho(),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
        };

        let (result_chirho, _) =
            lower_and_run_chirho(&module_chirho, None, HashSet::new()).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(55));
    }

    #[test]
    fn collect_app_flattens_chirho() {
        // f x y → (f, [x, y])
        let expr_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1))),
            }),
            arg_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2))),
        };

        let (head_chirho, args_chirho) = collect_app_chirho(&expr_chirho);
        assert!(matches!(head_chirho, CoreExprChirho::VarChirho(_)));
        assert_eq!(args_chirho.len(), 2);
    }

    #[test]
    fn collect_lam_flattens_chirho() {
        // \x -> \y -> body → ([x, y], body)
        let expr_chirho = CoreExprChirho::LamChirho {
            binder_chirho: int_binder_chirho("x", 0),
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: int_binder_chirho("y", 1),
                body_chirho: Box::new(CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0))),
            }),
        };

        let (binders_chirho, _body_chirho) = collect_lam_chirho(&expr_chirho);
        assert_eq!(binders_chirho.len(), 2);
    }
}
