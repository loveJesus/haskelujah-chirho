// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # STG Evaluation Loop
//!
//! The heart of the STG machine: a loop that repeatedly inspects the current
//! state (code pointer + local environment), evaluates the next step, and
//! interacts with the heap and stack to produce a final value in WHNF.
//!
//! ## State machine
//!
//! The evaluator operates in three modes:
//!
//! 1. **Enter** — enter a closure (function, thunk, constructor, PAP, etc.)
//! 2. **ReturnCon** — return a constructor to the stack (case continuation)
//! 3. **ReturnLit** — return an unboxed literal to the stack (primop / case)
//!
//! Transitions follow the STG operational semantics from SPJ's 1992 paper,
//! extended with PAP handling and primitive operations.

use std::collections::HashMap;

use crate::gc_chirho::{
    extract_roots_from_stack_chirho, extract_roots_from_values_chirho,
    GcConfigChirho, GcStateChirho, GcStatsChirho,
};
use crate::heap_chirho::HeapChirho;
use crate::prim_chirho::{apply_prim_binop_chirho, PrimErrorChirho};
use crate::stack_chirho::{FrameChirho, PrimOpKindChirho, StackChirho};
use crate::value_chirho::{
    ClosureChirho, CodePtrChirho, DataConTagChirho, HeapAddrChirho,
    InfoTagChirho, ValueChirho,
};

// ── Error type ──────────────────────────────────────────────────────────

/// Errors that can occur during evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum EvalErrorChirho {
    /// Tried to enter a blackhole — infinite loop detected.
    BlackholeChirho { addr_chirho: HeapAddrChirho },
    /// Stack underflow — nothing to return to.
    StackUnderflowChirho,
    /// A primitive operation failed.
    PrimFailChirho(PrimErrorChirho),
    /// Type error at runtime (e.g. case on non-constructor).
    TypeErrorChirho { message_chirho: String },
    /// Explicit runtime error via `error` or `undefined`.
    RuntimeErrorChirho(String),
    /// No matching alternative in a case expression.
    NoMatchingAltChirho {
        tag_chirho: DataConTagChirho,
        addr_chirho: HeapAddrChirho,
    },
    /// Step limit exceeded (runaway computation).
    StepLimitChirho { limit_chirho: u64 },
}

impl std::fmt::Display for EvalErrorChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BlackholeChirho { addr_chirho } => {
                write!(f_chirho, "<<loop>> at heap address {}", addr_chirho.0)
            }
            Self::StackUnderflowChirho => write!(f_chirho, "stack underflow"),
            Self::PrimFailChirho(e_chirho) => write!(f_chirho, "primop: {}", e_chirho),
            Self::TypeErrorChirho { message_chirho } => {
                write!(f_chirho, "type error: {}", message_chirho)
            }
            Self::NoMatchingAltChirho {
                tag_chirho,
                addr_chirho,
            } => write!(
                f_chirho,
                "no matching alternative for tag {} at @{}",
                tag_chirho.0, addr_chirho.0
            ),
            Self::RuntimeErrorChirho(msg_chirho) => {
                write!(f_chirho, "{}", msg_chirho)
            }
            Self::StepLimitChirho { limit_chirho } => {
                write!(f_chirho, "step limit ({}) exceeded", limit_chirho)
            }
        }
    }
}

// ── Code table ──────────────────────────────────────────────────────────

/// An instruction in the code table — what the evaluator should do next.
///
/// In a full compiler these would be bytecodes or IR; for now we use a
/// high-level enum that the evaluator interprets directly.
#[derive(Debug, Clone)]
pub enum CodeChirho {
    /// Enter a closure on the heap.
    EnterChirho(HeapAddrChirho),

    /// Push arguments and enter a function.
    AppChirho {
        fun_chirho: HeapAddrChirho,
        args_chirho: Vec<ArgSourceChirho>,
    },

    /// Evaluate the scrutinee, then branch.
    CaseChirho {
        scrutinee_chirho: ArgSourceChirho,
        alts_chirho: Vec<(u16, u32)>,
        default_chirho: Option<u32>,
    },

    /// Construct a data constructor on the heap and return it.
    ConAppChirho {
        tag_chirho: DataConTagChirho,
        name_chirho: String,
        fields_chirho: Vec<ValueChirho>,
    },

    /// Return an unboxed literal.
    LitChirho(ValueChirho),

    /// Apply a primitive operation to arguments.
    PrimChirho {
        op_chirho: PrimOpKindChirho,
        args_chirho: Vec<ArgSourceChirho>,
    },

    /// Allocate closures on the heap (let/letrec), then execute body.
    LetChirho {
        closures_chirho: Vec<ClosureChirho>,
        body_chirho: u32, // code table index for the body
    },

    /// Allocate a thunk and enter it (forcing evaluation).
    ForceChirho {
        thunk_chirho: ClosureChirho,
    },

    /// Read argument register `index_chirho` and return it as a literal.
    ArgChirho {
        index_chirho: usize,
    },

    /// Apply a function from an arg register to arguments.
    /// Used when a lambda parameter is itself applied as a function.
    /// Each argument is an `ArgSourceChirho` resolved at runtime from the
    /// current arg_regs, avoiding stale thunk captures.
    AppFromArgChirho {
        fun_arg_index_chirho: usize,
        arg_sources_chirho: Vec<ArgSourceChirho>,
    },

    /// Case dispatch on an unboxed literal value.
    /// Matches the scrutinee against each literal; falls through to
    /// `default_chirho` if no match.
    CaseLitChirho {
        /// The unboxed value to dispatch on (resolved at runtime).
        scrutinee_chirho: ArgSourceChirho,
        /// (literal_value, code_entry) pairs.
        alts_chirho: Vec<(ValueChirho, u32)>,
        /// Fallback code entry if no literal matches.
        default_chirho: Option<u32>,
    },

    /// Allocate a function closure at runtime, capturing free variable
    /// values from the current arg registers into the closure payload.
    /// The resulting HeapPtrChirho is the return value.
    /// When the closure is later entered, payload values are prepended
    /// to arg_regs before the function arguments.
    AllocFunChirho {
        /// Number of function parameters (not counting captured vars).
        arity_chirho: u16,
        /// Code entry point for the function body.
        code_ptr_chirho: u32,
        /// Name for debugging.
        name_chirho: String,
        /// Sources to capture into the closure payload at allocation time.
        captures_chirho: Vec<ArgSourceChirho>,
    },

    /// Construct a data constructor on the heap using ArgSourceChirho
    /// fields that are resolved at runtime. Used when constructor fields
    /// reference function parameters or complex expressions inside
    /// function bodies.
    ConAppFromArgChirho {
        tag_chirho: DataConTagChirho,
        name_chirho: String,
        fields_chirho: Vec<ArgSourceChirho>,
    },

    /// Allocate a function closure at runtime with captures, store the
    /// resulting HeapPtrChirho in a specified arg-register slot, then
    /// fall through to the next instruction. Used for let-bound lambda
    /// bindings that capture outer variables.
    StoreAllocFunChirho {
        /// Number of function parameters (not counting captured vars).
        arity_chirho: u16,
        /// Code entry point for the function body.
        code_ptr_chirho: u32,
        /// Name for debugging.
        name_chirho: String,
        /// Sources to capture into the closure payload at allocation time.
        captures_chirho: Vec<ArgSourceChirho>,
        /// Arg register slot to store the resulting HeapPtr.
        dest_reg_chirho: usize,
        /// Code entry for the body to continue to.
        body_chirho: u32,
        /// If set, write an indirection from this heap address to the
        /// newly allocated closure. Used for recursive let bindings so
        /// that self-references through the pre-allocated placeholder
        /// are redirected to the real closure (which has captures).
        patch_addr_chirho: Option<HeapAddrChirho>,
    },

    /// Allocate a thunk at runtime with captures, store the resulting
    /// HeapPtrChirho in a specified arg-register slot, then fall through
    /// to the body instruction.  Used for let-bound thunks whose RHS
    /// references lambda parameters (arg registers) that would be stale
    /// if the thunk were allocated statically at compile time.
    StoreAllocThunkChirho {
        /// Code entry point for the thunk body.
        code_ptr_chirho: u32,
        /// Name for debugging.
        name_chirho: String,
        /// Sources to capture into the thunk payload at allocation time.
        captures_chirho: Vec<ArgSourceChirho>,
        /// Arg register slot to store the resulting HeapPtr.
        dest_reg_chirho: usize,
        /// Code entry for the continuation body.
        body_chirho: u32,
        /// If set, write an indirection from this heap address to the
        /// newly allocated thunk. Used for letrec thunk bindings.
        patch_addr_chirho: Option<HeapAddrChirho>,
    },
}

/// Source of an argument value, resolved at runtime.
#[derive(Debug, Clone)]
pub enum ArgSourceChirho {
    /// A statically known value (literal or heap pointer).
    StaticChirho(ValueChirho),
    /// Read from an arg register at runtime.
    ArgRegChirho(usize),
    /// Allocate a fresh thunk at runtime with captured values.
    /// Used for complex sub-expressions inside function bodies
    /// so each call gets a fresh thunk (avoids stale blackhole).
    ThunkCodeChirho {
        code_ptr_chirho: u32,
        captures_chirho: Vec<ArgSourceChirho>,
    },
}

// ── Machine state ───────────────────────────────────────────────────────

/// The STG machine state.
#[derive(Debug)]
pub struct MachineChirho {
    pub heap_chirho: HeapChirho,
    pub stack_chirho: StackChirho,
    pub code_table_chirho: Vec<CodeChirho>,
    /// Argument registers: populated when entering a function body.
    pub arg_regs_chirho: Vec<ValueChirho>,
    /// Number of reduction steps taken.
    pub steps_chirho: u64,
    /// Maximum steps before aborting (0 = unlimited).
    pub step_limit_chirho: u64,
    /// Garbage collector state.
    pub gc_state_chirho: GcStateChirho,
    /// Last GC statistics (None if GC hasn't run yet).
    pub last_gc_stats_chirho: Option<GcStatsChirho>,
    /// Constructor name → tag mapping from STG lowering.
    pub con_tags_chirho: HashMap<String, u16>,
    /// Captured I/O output (for testing and sandboxed execution).
    pub io_output_chirho: String,
    /// Stdin input feed (for testing). Lines consumed by getLine/getChar.
    pub io_input_chirho: std::collections::VecDeque<String>,
    /// IORef storage: mutable reference cells indexed by ID.
    pub iorefs_chirho: HashMap<u64, ValueChirho>,
    /// Next IORef ID counter.
    pub next_ioref_chirho: u64,
}

impl MachineChirho {
    /// Create a new machine with the given code table.
    pub fn new_chirho(code_table_chirho: Vec<CodeChirho>) -> Self {
        Self {
            heap_chirho: HeapChirho::with_capacity_chirho(1024),
            stack_chirho: StackChirho::new_chirho(),
            code_table_chirho,
            arg_regs_chirho: Vec::new(),
            steps_chirho: 0,
            step_limit_chirho: 0,
            gc_state_chirho: GcStateChirho::new_chirho(GcConfigChirho::default()),
            last_gc_stats_chirho: None,
            con_tags_chirho: HashMap::new(),
            io_output_chirho: String::new(),
            io_input_chirho: std::collections::VecDeque::new(),
            iorefs_chirho: HashMap::new(),
            next_ioref_chirho: 0,
        }
    }

    /// Set a step limit.
    pub fn with_step_limit_chirho(mut self, limit_chirho: u64) -> Self {
        self.step_limit_chirho = limit_chirho;
        self
    }

    /// Configure the garbage collector.
    pub fn with_gc_config_chirho(mut self, config_chirho: GcConfigChirho) -> Self {
        self.gc_state_chirho = GcStateChirho::new_chirho(config_chirho);
        self
    }

    /// Resolve an `ArgSourceChirho` to a concrete `ValueChirho` using the
    /// current arg-register state. Allocates fresh thunks for
    /// `ThunkCodeChirho` variants so each function call gets its own
    /// thunk (avoiding stale blackhole / cached values from prior calls).
    fn resolve_arg_source_chirho(&mut self, source_chirho: &ArgSourceChirho) -> ValueChirho {
        match source_chirho {
            ArgSourceChirho::StaticChirho(v_chirho) => v_chirho.clone(),
            ArgSourceChirho::ArgRegChirho(idx_chirho) => {
                if *idx_chirho < self.arg_regs_chirho.len() {
                    self.arg_regs_chirho[*idx_chirho].clone()
                } else {
                    ValueChirho::IntChirho(0)
                }
            }
            ArgSourceChirho::ThunkCodeChirho {
                code_ptr_chirho,
                captures_chirho,
            } => {
                let payload_chirho: Vec<ValueChirho> = captures_chirho
                    .iter()
                    .map(|c_chirho| self.resolve_arg_source_chirho(c_chirho))
                    .collect();
                let thunk_chirho = ClosureChirho::thunk_chirho(
                    CodePtrChirho(*code_ptr_chirho),
                    "$rt_thunk",
                    payload_chirho,
                );
                let addr_chirho = self.heap_chirho.alloc_chirho(thunk_chirho);
                ValueChirho::HeapPtrChirho(addr_chirho)
            }
        }
    }

    /// Resolve a batch of `ArgSourceChirho` values to `ValueChirho`.
    fn resolve_args_chirho(&mut self, sources_chirho: &[ArgSourceChirho]) -> Vec<ValueChirho> {
        sources_chirho
            .iter()
            .map(|s_chirho| self.resolve_arg_source_chirho(s_chirho))
            .collect()
    }

    /// Run garbage collection if the allocation threshold has been reached.
    /// Extracts roots from the stack and argument registers.
    fn maybe_gc_chirho(&mut self) {
        if !self.gc_state_chirho.notify_alloc_chirho() {
            return;
        }
        let mut roots_chirho =
            extract_roots_from_stack_chirho(self.stack_chirho.frames_chirho());
        roots_chirho.extend(extract_roots_from_values_chirho(&self.arg_regs_chirho));
        let stats_chirho =
            self.gc_state_chirho
                .collect_chirho(&mut self.heap_chirho, &roots_chirho);
        self.last_gc_stats_chirho = Some(stats_chirho);
    }

    /// Run the machine starting from the given code table index.
    /// Returns the final value in WHNF.
    pub fn run_chirho(
        &mut self,
        entry_chirho: u32,
    ) -> Result<ValueChirho, EvalErrorChirho> {
        let mut pc_chirho = entry_chirho;

        'eval: loop {
            // Step limit check
            if self.step_limit_chirho > 0 && self.steps_chirho >= self.step_limit_chirho {
                return Err(EvalErrorChirho::StepLimitChirho {
                    limit_chirho: self.step_limit_chirho,
                });
            }
            self.steps_chirho += 1;

            let code_chirho = self.code_table_chirho[pc_chirho as usize].clone();

            match code_chirho {
                // ── Enter a closure ─────────────────────────────────
                CodeChirho::EnterChirho(addr_chirho) => {
                    let addr_chirho = self.heap_chirho.follow_ind_chirho(addr_chirho);
                    let closure_chirho = self.heap_chirho.read_chirho(addr_chirho).clone();

                    match closure_chirho.info_chirho.tag_chirho {
                        InfoTagChirho::ConChirho => {
                            // Constructor in WHNF — return it
                            match self.return_con_chirho(addr_chirho)? {
                                ReturnActionChirho::DoneChirho(v_chirho) => return Ok(v_chirho),
                                ReturnActionChirho::ContinueChirho(next_chirho) => {
                                    pc_chirho = next_chirho;
                                }
                            }
                        }
                        InfoTagChirho::FunChirho => {
                            // Function — check for Apply frame
                            match self.enter_fun_chirho(addr_chirho, &closure_chirho)? {
                                ReturnActionChirho::DoneChirho(v_chirho) => return Ok(v_chirho),
                                ReturnActionChirho::ContinueChirho(next_chirho) => {
                                    pc_chirho = next_chirho;
                                }
                            }
                        }
                        InfoTagChirho::ThunkChirho => {
                            // Thunk — blackhole, push update frame, enter body.
                            // If the thunk has captured values in its payload,
                            // restore them as arg_regs so the thunk's code can
                            // read the captured values via ArgRegChirho indices.
                            self.heap_chirho.blackhole_chirho(addr_chirho);
                            self.stack_chirho.push_chirho(FrameChirho::UpdateChirho {
                                thunk_addr_chirho: addr_chirho,
                            });
                            if !closure_chirho.payload_chirho.is_empty() {
                                self.arg_regs_chirho =
                                    closure_chirho.payload_chirho.clone();
                            }
                            pc_chirho = closure_chirho.info_chirho.entry_chirho.0;
                        }
                        InfoTagChirho::PapChirho => {
                            // PAP — like function but with pre-applied args
                            match self.enter_pap_chirho(addr_chirho, &closure_chirho)? {
                                ReturnActionChirho::DoneChirho(v_chirho) => return Ok(v_chirho),
                                ReturnActionChirho::ContinueChirho(next_chirho) => {
                                    pc_chirho = next_chirho;
                                }
                            }
                        }
                        InfoTagChirho::IndChirho => {
                            // Should have been followed already
                            unreachable!("follow_ind_chirho should resolve indirections")
                        }
                        InfoTagChirho::BlackholeChirho => {
                            return Err(EvalErrorChirho::BlackholeChirho { addr_chirho });
                        }
                    }
                }

                // ── Application ─────────────────────────────────────
                CodeChirho::AppChirho {
                    fun_chirho,
                    args_chirho,
                } => {
                    if !args_chirho.is_empty() {
                        let resolved_chirho = self.resolve_args_chirho(&args_chirho);
                        self.stack_chirho.push_chirho(FrameChirho::ApplyChirho {
                            args_chirho: resolved_chirho,
                        });
                    }
                    // Re-dispatch to enter
                    pc_chirho = self.emit_enter_chirho(fun_chirho);
                }

                // ── Apply from arg register ─────────────────────────
                CodeChirho::AppFromArgChirho {
                    fun_arg_index_chirho,
                    arg_sources_chirho,
                } => {
                    let fun_val_chirho =
                        if fun_arg_index_chirho < self.arg_regs_chirho.len() {
                            self.arg_regs_chirho[fun_arg_index_chirho].clone()
                        } else {
                            return Err(EvalErrorChirho::TypeErrorChirho {
                                message_chirho: format!(
                                    "fun arg register {} out of bounds (have {})",
                                    fun_arg_index_chirho,
                                    self.arg_regs_chirho.len()
                                ),
                            });
                        };

                    // Resolve all arg sources from current arg_regs
                    let resolved_args_chirho = self.resolve_args_chirho(&arg_sources_chirho);

                    match fun_val_chirho {
                        ValueChirho::HeapPtrChirho(addr_chirho) => {
                            if !resolved_args_chirho.is_empty() {
                                self.stack_chirho.push_chirho(FrameChirho::ApplyChirho {
                                    args_chirho: resolved_args_chirho,
                                });
                            }
                            pc_chirho = self.emit_enter_chirho(addr_chirho);
                        }
                        _ => {
                            return Ok(fun_val_chirho);
                        }
                    }
                }

                // ── Case expression ─────────────────────────────────
                CodeChirho::CaseChirho {
                    scrutinee_chirho,
                    alts_chirho,
                    default_chirho,
                } => {
                    let resolved_scrut_chirho =
                        self.resolve_arg_source_chirho(&scrutinee_chirho);
                    self.stack_chirho.push_chirho(FrameChirho::CaseChirho {
                        alt_entries_chirho: alts_chirho,
                        default_entry_chirho: default_chirho,
                        saved_arg_regs_chirho: self.arg_regs_chirho.clone(),
                    });
                    match resolved_scrut_chirho {
                        ValueChirho::HeapPtrChirho(addr_chirho) => {
                            pc_chirho = self.emit_enter_chirho(addr_chirho);
                        }
                        other_chirho => {
                            // Already unboxed — return as lit to the Case frame
                            match self.return_lit_chirho(other_chirho)? {
                                ReturnActionChirho::DoneChirho(v_chirho) => {
                                    return Ok(v_chirho)
                                }
                                ReturnActionChirho::ContinueChirho(next_chirho) => {
                                    pc_chirho = next_chirho;
                                }
                            }
                        }
                    }
                }

                // ── Constructor application ─────────────────────────
                CodeChirho::ConAppChirho {
                    tag_chirho,
                    name_chirho,
                    fields_chirho,
                } => {
                    let closure_chirho =
                        ClosureChirho::con_chirho(tag_chirho, &name_chirho, fields_chirho);
                    let addr_chirho = self.heap_chirho.alloc_chirho(closure_chirho);
                    self.maybe_gc_chirho();
                    match self.return_con_chirho(addr_chirho)? {
                        ReturnActionChirho::DoneChirho(v_chirho) => return Ok(v_chirho),
                        ReturnActionChirho::ContinueChirho(next_chirho) => {
                            pc_chirho = next_chirho;
                        }
                    }
                }

                // ── Constructor application with runtime fields ──
                CodeChirho::ConAppFromArgChirho {
                    tag_chirho,
                    name_chirho,
                    fields_chirho,
                } => {
                    let resolved_fields_chirho: Vec<ValueChirho> = fields_chirho
                        .iter()
                        .map(|s_chirho| self.resolve_arg_source_chirho(s_chirho))
                        .collect();
                    let closure_chirho = ClosureChirho::con_chirho(
                        tag_chirho,
                        &name_chirho[..],
                        resolved_fields_chirho,
                    );
                    let addr_chirho = self.heap_chirho.alloc_chirho(closure_chirho);
                    self.maybe_gc_chirho();
                    match self.return_con_chirho(addr_chirho)? {
                        ReturnActionChirho::DoneChirho(v_chirho) => return Ok(v_chirho),
                        ReturnActionChirho::ContinueChirho(next_chirho) => {
                            pc_chirho = next_chirho;
                        }
                    }
                }

                // ── Literal case dispatch ──────────────────────────
                CodeChirho::CaseLitChirho {
                    scrutinee_chirho,
                    alts_chirho,
                    default_chirho,
                } => {
                    let resolved_chirho = self.resolve_arg_source_chirho(&scrutinee_chirho);
                    match resolved_chirho {
                        ValueChirho::HeapPtrChirho(addr_chirho) => {
                            // Scrutinee is a thunk — force it first.
                            // Push a CaseLitChirho frame so that when the
                            // thunk returns a literal, we can continue
                            // with the matching.
                            self.stack_chirho.push_chirho(FrameChirho::CaseLitChirho {
                                alt_entries_chirho: alts_chirho,
                                default_entry_chirho: default_chirho,
                            });
                            pc_chirho = self.emit_enter_chirho(addr_chirho);
                        }
                        _ => {
                            // Already a value — match directly.
                            pc_chirho = self.dispatch_case_lit_chirho(
                                &resolved_chirho,
                                &alts_chirho,
                                default_chirho,
                            )?;
                        }
                    }
                }

                // ── Literal return ──────────────────────────────────
                CodeChirho::LitChirho(val_chirho) => {
                    match self.return_lit_chirho(val_chirho)? {
                        ReturnActionChirho::DoneChirho(v_chirho) => return Ok(v_chirho),
                        ReturnActionChirho::ContinueChirho(next_chirho) => {
                            pc_chirho = next_chirho;
                        }
                    }
                }

                // ── Primitive operation ──────────────────────────────
                CodeChirho::PrimChirho {
                    op_chirho,
                    args_chirho,
                } => {
                    // Resolve ArgSource → ValueChirho (fresh thunks each time).
                    let resolved_chirho = self.resolve_args_chirho(&args_chirho);

                    // Check if any arg needs forcing (is a HeapPtr thunk).
                    let needs_forcing_chirho = resolved_chirho
                        .iter()
                        .any(|a_chirho| matches!(a_chirho, ValueChirho::HeapPtrChirho(_)));

                    if !needs_forcing_chirho {
                        // Fast path: all args already unboxed.
                        let result_chirho =
                            self.eval_prim_chirho(op_chirho, &resolved_chirho)?;
                        match self.return_lit_chirho(result_chirho)? {
                            ReturnActionChirho::DoneChirho(v_chirho) => {
                                return Ok(v_chirho)
                            }
                            ReturnActionChirho::ContinueChirho(next_chirho) => {
                                pc_chirho = next_chirho;
                            }
                        }
                    } else {
                        // Slow path: force args one at a time via the stack.
                        let total_chirho = resolved_chirho.len() as u16;
                        let mut pending_chirho = resolved_chirho;
                        let first_chirho = pending_chirho.remove(0);

                        self.stack_chirho.push_chirho(FrameChirho::PrimOpChirho {
                            op_chirho,
                            args_so_far_chirho: vec![],
                            pending_args_chirho: pending_chirho,
                            remaining_chirho: total_chirho,
                        });

                        match first_chirho {
                            ValueChirho::HeapPtrChirho(addr_chirho) => {
                                // Enter the closure to force it to WHNF.
                                pc_chirho = self.emit_enter_chirho(addr_chirho);
                            }
                            other_chirho => {
                                // Already unboxed — return as lit to the
                                // PrimOp frame.
                                match self.return_lit_chirho(other_chirho)? {
                                    ReturnActionChirho::DoneChirho(v_chirho) => {
                                        return Ok(v_chirho)
                                    }
                                    ReturnActionChirho::ContinueChirho(
                                        next_chirho,
                                    ) => {
                                        pc_chirho = next_chirho;
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Let allocation ──────────────────────────────────
                CodeChirho::LetChirho {
                    closures_chirho,
                    body_chirho,
                } => {
                    // Allocate all closures (addresses are sequential)
                    for c_chirho in closures_chirho {
                        self.heap_chirho.alloc_chirho(c_chirho);
                        self.maybe_gc_chirho();
                    }
                    pc_chirho = body_chirho;
                }

                // ── Allocate function closure with captures ────────
                CodeChirho::AllocFunChirho {
                    arity_chirho,
                    code_ptr_chirho,
                    name_chirho,
                    captures_chirho,
                } => {
                    // Resolve captured values from current arg registers
                    let payload_chirho = self.resolve_args_chirho(&captures_chirho);
                    let closure_chirho = ClosureChirho::fun_chirho(
                        arity_chirho,
                        CodePtrChirho(code_ptr_chirho),
                        &name_chirho,
                        payload_chirho,
                    );
                    let addr_chirho = self.heap_chirho.alloc_chirho(closure_chirho);
                    self.maybe_gc_chirho();
                    // Return the closure as a HeapPtr
                    let val_chirho = ValueChirho::HeapPtrChirho(addr_chirho);
                    match self.return_lit_chirho(val_chirho)? {
                        ReturnActionChirho::DoneChirho(v_chirho) => return Ok(v_chirho),
                        ReturnActionChirho::ContinueChirho(next_chirho) => {
                            pc_chirho = next_chirho;
                        }
                    }
                }

                // ── Allocate function with captures and store ──────
                CodeChirho::StoreAllocFunChirho {
                    arity_chirho,
                    code_ptr_chirho,
                    name_chirho,
                    captures_chirho,
                    dest_reg_chirho,
                    body_chirho,
                    patch_addr_chirho,
                } => {
                    let payload_chirho = self.resolve_args_chirho(&captures_chirho);
                    let closure_chirho = ClosureChirho::fun_chirho(
                        arity_chirho,
                        CodePtrChirho(code_ptr_chirho),
                        &name_chirho,
                        payload_chirho,
                    );
                    let addr_chirho = self.heap_chirho.alloc_chirho(closure_chirho);
                    // Patch the pre-allocated placeholder (if any) with an
                    // indirection to the real closure so self-references
                    // through the placeholder are correctly redirected.
                    if let Some(placeholder_chirho) = patch_addr_chirho {
                        self.heap_chirho
                            .update_to_ind_chirho(placeholder_chirho, addr_chirho);
                    }
                    self.maybe_gc_chirho();
                    // Store in dest arg register and continue to body
                    if self.arg_regs_chirho.len() <= dest_reg_chirho {
                        self.arg_regs_chirho
                            .resize(dest_reg_chirho + 1, ValueChirho::IntChirho(0));
                    }
                    self.arg_regs_chirho[dest_reg_chirho] =
                        ValueChirho::HeapPtrChirho(addr_chirho);
                    pc_chirho = body_chirho;
                }

                // ── Allocate thunk with captures and store ────────
                CodeChirho::StoreAllocThunkChirho {
                    code_ptr_chirho,
                    name_chirho,
                    captures_chirho,
                    dest_reg_chirho,
                    body_chirho,
                    patch_addr_chirho,
                } => {
                    let payload_chirho = self.resolve_args_chirho(&captures_chirho);
                    let closure_chirho = ClosureChirho::thunk_chirho(
                        CodePtrChirho(code_ptr_chirho),
                        &name_chirho,
                        payload_chirho,
                    );
                    let addr_chirho = self.heap_chirho.alloc_chirho(closure_chirho);
                    // Patch letrec placeholder if needed
                    if let Some(placeholder_chirho) = patch_addr_chirho {
                        self.heap_chirho
                            .update_to_ind_chirho(placeholder_chirho, addr_chirho);
                    }
                    self.maybe_gc_chirho();
                    // Store in dest arg register and continue to body
                    if self.arg_regs_chirho.len() <= dest_reg_chirho {
                        self.arg_regs_chirho
                            .resize(dest_reg_chirho + 1, ValueChirho::IntChirho(0));
                    }
                    self.arg_regs_chirho[dest_reg_chirho] =
                        ValueChirho::HeapPtrChirho(addr_chirho);
                    pc_chirho = body_chirho;
                }

                // ── Force a thunk ───────────────────────────────────
                CodeChirho::ForceChirho { thunk_chirho } => {
                    let addr_chirho = self.heap_chirho.alloc_chirho(thunk_chirho);
                    self.maybe_gc_chirho();
                    pc_chirho = self.emit_enter_chirho(addr_chirho);
                }

                // ── Read argument register ──────────────────────────
                CodeChirho::ArgChirho { index_chirho } => {
                    let val_chirho = self
                        .arg_regs_chirho
                        .get(index_chirho)
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0));
                    match val_chirho {
                        ValueChirho::HeapPtrChirho(addr_chirho) => {
                            pc_chirho = self.emit_enter_chirho(addr_chirho);
                        }
                        _ => {
                            match self.return_lit_chirho(val_chirho)? {
                                ReturnActionChirho::DoneChirho(v_chirho) => {
                                    return Ok(v_chirho)
                                }
                                ReturnActionChirho::ContinueChirho(next_chirho) => {
                                    pc_chirho = next_chirho;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // ── Return a constructor to the stack ───────────────────────────────

    fn return_con_chirho(
        &mut self,
        addr_chirho: HeapAddrChirho,
    ) -> Result<ReturnActionChirho, EvalErrorChirho> {
        loop {
            match self.stack_chirho.pop_chirho() {
                None => {
                    // Stack empty — we're done
                    return Ok(ReturnActionChirho::DoneChirho(
                        ValueChirho::HeapPtrChirho(addr_chirho),
                    ));
                }
                Some(FrameChirho::UpdateChirho { thunk_addr_chirho }) => {
                    // Update the thunk with an indirection to the result
                    self.heap_chirho
                        .update_to_ind_chirho(thunk_addr_chirho, addr_chirho);
                    // Continue unwinding the stack
                    continue;
                }
                Some(FrameChirho::CaseChirho {
                    alt_entries_chirho,
                    default_entry_chirho,
                    saved_arg_regs_chirho,
                }) => {
                    let closure_chirho = self.heap_chirho.read_chirho(addr_chirho);
                    let con_tag_chirho = closure_chirho.info_chirho.con_tag_chirho.0;
                    // Extract constructor fields for alt binder binding.
                    // If the constructor has fields, set arg_regs to those
                    // fields so alt binders can access them via ArgChirho.
                    // Otherwise, restore the saved arg_regs so the alt body
                    // can still access enclosing function parameters.
                    let fields_chirho = closure_chirho.payload_chirho.clone();
                    let new_regs_chirho = if fields_chirho.is_empty() {
                        saved_arg_regs_chirho
                    } else {
                        // Prepend constructor fields to saved arg regs so
                        // both the case alt binders AND outer function
                        // parameters are accessible.
                        let mut regs_chirho = fields_chirho;
                        regs_chirho.extend(saved_arg_regs_chirho);
                        regs_chirho
                    };

                    // Find matching alt
                    for (tag_chirho, entry_chirho) in &alt_entries_chirho {
                        if *tag_chirho == con_tag_chirho {
                            self.arg_regs_chirho = new_regs_chirho;
                            return Ok(ReturnActionChirho::ContinueChirho(*entry_chirho));
                        }
                    }
                    // Try default
                    if let Some(def_chirho) = default_entry_chirho {
                        self.arg_regs_chirho = new_regs_chirho;
                        return Ok(ReturnActionChirho::ContinueChirho(def_chirho));
                    }
                    return Err(EvalErrorChirho::NoMatchingAltChirho {
                        tag_chirho: DataConTagChirho(con_tag_chirho),
                        addr_chirho,
                    });
                }
                Some(FrameChirho::CaseLitChirho {
                    alt_entries_chirho,
                    default_entry_chirho,
                }) => {
                    // A constructor returned to a literal case frame.
                    // Try to unbox the constructor (e.g. I# n → IntChirho(n))
                    // and dispatch against the literal alternatives.
                    let closure_chirho = self.heap_chirho.read_chirho(addr_chirho);
                    let unboxed_chirho = if closure_chirho.payload_chirho.len() == 1 {
                        match &closure_chirho.payload_chirho[0] {
                            ValueChirho::IntChirho(n_chirho) => {
                                Some(ValueChirho::IntChirho(*n_chirho))
                            }
                            ValueChirho::FloatChirho(n_chirho) => {
                                Some(ValueChirho::FloatChirho(*n_chirho))
                            }
                            ValueChirho::CharChirho(c_chirho) => {
                                Some(ValueChirho::CharChirho(*c_chirho))
                            }
                            ValueChirho::BoolChirho(b_chirho) => {
                                Some(ValueChirho::BoolChirho(*b_chirho))
                            }
                            _ => None,
                        }
                    } else {
                        None
                    };
                    if let Some(val_chirho) = unboxed_chirho {
                        let entry_chirho = self.dispatch_case_lit_chirho(
                            &val_chirho,
                            &alt_entries_chirho,
                            default_entry_chirho,
                        )?;
                        return Ok(ReturnActionChirho::ContinueChirho(entry_chirho));
                    }
                    // Can't unbox — try default
                    if let Some(def_chirho) = default_entry_chirho {
                        return Ok(ReturnActionChirho::ContinueChirho(def_chirho));
                    }
                    return Err(EvalErrorChirho::TypeErrorChirho {
                        message_chirho: format!(
                            "cannot match constructor at @{} in literal case",
                            addr_chirho.0
                        ),
                    });
                }
                Some(FrameChirho::ApplyChirho { args_chirho }) => {
                    // Check if this is actually a PAP (created by under-
                    // application in enter_fun_chirho).  PAPs routed through
                    // return_con_chirho need to absorb remaining Apply args.
                    let closure_chirho_peek =
                        self.heap_chirho.read_chirho(addr_chirho).clone();
                    if closure_chirho_peek.info_chirho.tag_chirho
                        == InfoTagChirho::PapChirho
                    {
                        // Re-push the Apply frame and enter the PAP
                        self.stack_chirho.push_chirho(FrameChirho::ApplyChirho {
                            args_chirho,
                        });
                        return self.enter_pap_chirho(addr_chirho, &closure_chirho_peek);
                    }
                    // True constructor with extra args — type error
                    return Err(EvalErrorChirho::TypeErrorChirho {
                        message_chirho: format!(
                            "cannot apply {} args to constructor at @{}",
                            args_chirho.len(),
                            addr_chirho.0
                        ),
                    });
                }
                Some(FrameChirho::PrimOpChirho {
                    op_chirho,
                    mut args_so_far_chirho,
                    mut pending_args_chirho,
                    remaining_chirho,
                }) => {
                    // A primop waiting for a constructor result — try to
                    // extract an unboxed value from the constructor payload
                    // (e.g. I# n → IntChirho(n)).
                    // For compound-type primops (showMaybe#, showTuple2#,
                    // showList#), keep the HeapPtr so they can inspect
                    // the constructor structure.
                    let keep_heap_ptr_chirho = matches!(
                        op_chirho,
                        PrimOpKindChirho::ShowMaybeChirho
                        | PrimOpKindChirho::ShowTuple2Chirho
                        | PrimOpKindChirho::ShowListChirho
                    );
                    let closure_chirho =
                        self.heap_chirho.read_chirho(addr_chirho).clone();
                    let unboxed_chirho = if keep_heap_ptr_chirho {
                        ValueChirho::HeapPtrChirho(addr_chirho)
                    } else if closure_chirho.payload_chirho.len() == 1 {
                        match &closure_chirho.payload_chirho[0] {
                            ValueChirho::IntChirho(n_chirho) => {
                                ValueChirho::IntChirho(*n_chirho)
                            }
                            ValueChirho::FloatChirho(n_chirho) => {
                                ValueChirho::FloatChirho(*n_chirho)
                            }
                            ValueChirho::CharChirho(c_chirho) => {
                                ValueChirho::CharChirho(*c_chirho)
                            }
                            ValueChirho::BoolChirho(b_chirho) => {
                                ValueChirho::BoolChirho(*b_chirho)
                            }
                            _ => ValueChirho::HeapPtrChirho(addr_chirho),
                        }
                    } else {
                        ValueChirho::HeapPtrChirho(addr_chirho)
                    };

                    args_so_far_chirho.push(unboxed_chirho);
                    if remaining_chirho <= 1 {
                        let result_chirho =
                            self.eval_prim_chirho(op_chirho, &args_so_far_chirho)?;
                        return self.return_lit_chirho(result_chirho);
                    } else {
                        // Force the next pending arg
                        let next_chirho = pending_args_chirho.remove(0);
                        self.stack_chirho.push_chirho(FrameChirho::PrimOpChirho {
                            op_chirho,
                            args_so_far_chirho,
                            pending_args_chirho,
                            remaining_chirho: remaining_chirho - 1,
                        });
                        match next_chirho {
                            ValueChirho::HeapPtrChirho(a_chirho) => {
                                let enter_idx_chirho =
                                    self.emit_enter_chirho(a_chirho);
                                return Ok(ReturnActionChirho::ContinueChirho(
                                    enter_idx_chirho,
                                ));
                            }
                            other_chirho => {
                                return self.return_lit_chirho(other_chirho);
                            }
                        }
                    }
                }
            }
        }
    }

    // ── Return a literal to the stack ───────────────────────────────────

    fn return_lit_chirho(
        &mut self,
        val_chirho: ValueChirho,
    ) -> Result<ReturnActionChirho, EvalErrorChirho> {
        // If the value is a heap pointer (e.g. a constructor returned by a
        // primop like enumFromTo#), use the constructor return path directly
        // rather than boxing it.
        if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
            return self.return_heap_ptr_chirho(addr_chirho);
        }

        loop {
            match self.stack_chirho.pop_chirho() {
                None => {
                    return Ok(ReturnActionChirho::DoneChirho(val_chirho));
                }
                Some(FrameChirho::UpdateChirho { thunk_addr_chirho }) => {
                    // Box the literal into a constructor on the heap, then
                    // update the thunk with an indirection
                    let boxed_chirho = self.box_literal_chirho(&val_chirho);
                    let boxed_addr_chirho = self.heap_chirho.alloc_chirho(boxed_chirho);
                    self.heap_chirho
                        .update_to_ind_chirho(thunk_addr_chirho, boxed_addr_chirho);
                    continue;
                }
                Some(FrameChirho::PrimOpChirho {
                    op_chirho,
                    mut args_so_far_chirho,
                    mut pending_args_chirho,
                    remaining_chirho,
                }) => {
                    args_so_far_chirho.push(val_chirho.clone());
                    if remaining_chirho <= 1 {
                        // All args collected — execute the primop
                        let result_chirho =
                            self.eval_prim_chirho(op_chirho, &args_so_far_chirho)?;
                        // Recursively return the result
                        return self.return_lit_chirho(result_chirho);
                    } else {
                        // Force the next pending arg
                        let next_chirho = pending_args_chirho.remove(0);
                        self.stack_chirho.push_chirho(FrameChirho::PrimOpChirho {
                            op_chirho,
                            args_so_far_chirho,
                            pending_args_chirho,
                            remaining_chirho: remaining_chirho - 1,
                        });
                        match next_chirho {
                            ValueChirho::HeapPtrChirho(addr_chirho) => {
                                // Enter the closure to force it
                                let enter_idx_chirho =
                                    self.emit_enter_chirho(addr_chirho);
                                return Ok(ReturnActionChirho::ContinueChirho(
                                    enter_idx_chirho,
                                ));
                            }
                            other_chirho => {
                                // Already unboxed — recursively return it
                                return self.return_lit_chirho(other_chirho);
                            }
                        }
                    }
                }
                Some(frame_chirho @ FrameChirho::CaseChirho { .. }) => {
                    // Box the literal into a constructor and dispatch
                    // via return_con_chirho (e.g. BoolChirho(true) → True,
                    // IntChirho(n) → I# n).
                    let boxed_chirho = self.box_literal_chirho(&val_chirho);
                    let boxed_addr_chirho = self.heap_chirho.alloc_chirho(boxed_chirho);
                    // Push the case frame back so return_con_chirho can pop it
                    self.stack_chirho.push_chirho(frame_chirho);
                    return self.return_con_chirho(boxed_addr_chirho);
                }
                Some(FrameChirho::CaseLitChirho {
                    alt_entries_chirho,
                    default_entry_chirho,
                }) => {
                    // The scrutinee has been forced to a literal.
                    // Match it against the literal alternatives.
                    let entry_chirho = self.dispatch_case_lit_chirho(
                        &val_chirho,
                        &alt_entries_chirho,
                        default_entry_chirho,
                    )?;
                    return Ok(ReturnActionChirho::ContinueChirho(entry_chirho));
                }
                Some(FrameChirho::ApplyChirho { .. }) => {
                    return Err(EvalErrorChirho::TypeErrorChirho {
                        message_chirho: format!(
                            "cannot apply arguments to literal {}",
                            val_chirho
                        ),
                    });
                }
            }
        }
    }

    /// Return a heap pointer through the stack — handles Update frames
    /// with direct indirection and Case frames via `return_con_chirho`.
    fn return_heap_ptr_chirho(
        &mut self,
        addr_chirho: HeapAddrChirho,
    ) -> Result<ReturnActionChirho, EvalErrorChirho> {
        loop {
            match self.stack_chirho.pop_chirho() {
                None => {
                    return Ok(ReturnActionChirho::DoneChirho(
                        ValueChirho::HeapPtrChirho(addr_chirho),
                    ));
                }
                Some(FrameChirho::UpdateChirho { thunk_addr_chirho }) => {
                    // Update thunk to an indirection to the heap object.
                    self.heap_chirho
                        .update_to_ind_chirho(thunk_addr_chirho, addr_chirho);
                    continue;
                }
                Some(frame_chirho @ FrameChirho::CaseChirho { .. }) => {
                    // Dispatch via the constructor return path.
                    self.stack_chirho.push_chirho(frame_chirho);
                    return self.return_con_chirho(addr_chirho);
                }
                Some(FrameChirho::PrimOpChirho {
                    op_chirho,
                    mut args_so_far_chirho,
                    mut pending_args_chirho,
                    remaining_chirho,
                }) => {
                    args_so_far_chirho.push(ValueChirho::HeapPtrChirho(addr_chirho));
                    if remaining_chirho <= 1 {
                        let result_chirho =
                            self.eval_prim_chirho(op_chirho, &args_so_far_chirho)?;
                        return self.return_lit_chirho(result_chirho);
                    } else {
                        let next_chirho = pending_args_chirho.remove(0);
                        self.stack_chirho.push_chirho(FrameChirho::PrimOpChirho {
                            op_chirho,
                            args_so_far_chirho,
                            pending_args_chirho,
                            remaining_chirho: remaining_chirho - 1,
                        });
                        match next_chirho {
                            ValueChirho::HeapPtrChirho(a_chirho) => {
                                let enter_idx_chirho =
                                    self.emit_enter_chirho(a_chirho);
                                return Ok(ReturnActionChirho::ContinueChirho(
                                    enter_idx_chirho,
                                ));
                            }
                            other_chirho => {
                                return self.return_lit_chirho(other_chirho);
                            }
                        }
                    }
                }
                Some(FrameChirho::ApplyChirho { args_chirho }) => {
                    // The heap object is being applied to arguments — enter it.
                    self.stack_chirho.push_chirho(FrameChirho::ApplyChirho {
                        args_chirho,
                    });
                    let enter_idx_chirho = self.emit_enter_chirho(addr_chirho);
                    return Ok(ReturnActionChirho::ContinueChirho(enter_idx_chirho));
                }
                Some(FrameChirho::CaseLitChirho {
                    alt_entries_chirho,
                    default_entry_chirho,
                }) => {
                    let entry_chirho = self.dispatch_case_lit_chirho(
                        &ValueChirho::HeapPtrChirho(addr_chirho),
                        &alt_entries_chirho,
                        default_entry_chirho,
                    )?;
                    return Ok(ReturnActionChirho::ContinueChirho(entry_chirho));
                }
            }
        }
    }

    /// Dispatch a literal case: match the value against the alternatives
    /// and return the code entry for the matching branch.
    fn dispatch_case_lit_chirho(
        &self,
        val_chirho: &ValueChirho,
        alts_chirho: &[(ValueChirho, u32)],
        default_chirho: Option<u32>,
    ) -> Result<u32, EvalErrorChirho> {
        for (lit_chirho, entry_chirho) in alts_chirho {
            if *lit_chirho == *val_chirho {
                return Ok(*entry_chirho);
            }
        }
        if let Some(def_chirho) = default_chirho {
            Ok(def_chirho)
        } else {
            Err(EvalErrorChirho::TypeErrorChirho {
                message_chirho: format!(
                    "non-exhaustive literal case: {:?}",
                    val_chirho
                ),
            })
        }
    }

    // ── Enter a function closure ────────────────────────────────────────

    fn enter_fun_chirho(
        &mut self,
        addr_chirho: HeapAddrChirho,
        closure_chirho: &ClosureChirho,
    ) -> Result<ReturnActionChirho, EvalErrorChirho> {
        let arity_chirho = closure_chirho.info_chirho.arity_chirho as usize;
        let payload_chirho = &closure_chirho.payload_chirho;

        // Loop to skip past Update frames (update the thunk, keep looking
        // for an Apply frame that will actually supply arguments).
        loop {
            match self.stack_chirho.pop_chirho() {
                Some(FrameChirho::ApplyChirho { args_chirho }) => {
                    let n_args_chirho = args_chirho.len();
                    if n_args_chirho == arity_chirho {
                        // Exact application — prepend payload (captured free
                        // vars) then args into registers, enter body
                        let mut regs_chirho = payload_chirho.clone();
                        regs_chirho.extend(args_chirho);
                        self.arg_regs_chirho = regs_chirho;
                        return Ok(ReturnActionChirho::ContinueChirho(
                            closure_chirho.info_chirho.entry_chirho.0,
                        ));
                    } else if n_args_chirho > arity_chirho {
                        // Over-application: take arity args, push the rest
                        let (taken_chirho, rest_chirho) =
                            args_chirho.split_at(arity_chirho);
                        let mut regs_chirho = payload_chirho.clone();
                        regs_chirho.extend_from_slice(taken_chirho);
                        self.arg_regs_chirho = regs_chirho;
                        self.stack_chirho.push_chirho(FrameChirho::ApplyChirho {
                            args_chirho: rest_chirho.to_vec(),
                        });
                        return Ok(ReturnActionChirho::ContinueChirho(
                            closure_chirho.info_chirho.entry_chirho.0,
                        ));
                    } else {
                        // Under-application: build a PAP
                        let remaining_chirho =
                            (arity_chirho - n_args_chirho) as u16;
                        let pap_chirho = ClosureChirho::pap_chirho(
                            remaining_chirho,
                            addr_chirho,
                            args_chirho,
                        );
                        let pap_addr_chirho =
                            self.heap_chirho.alloc_chirho(pap_chirho);
                        return self.return_con_chirho(pap_addr_chirho);
                    }
                }
                Some(FrameChirho::UpdateChirho { thunk_addr_chirho }) => {
                    // A function value returned to an update frame — update
                    // the thunk to point at this function, then continue
                    // looking for an Apply frame.
                    self.heap_chirho
                        .update_to_ind_chirho(thunk_addr_chirho, addr_chirho);
                    continue;
                }
                Some(other_chirho) => {
                    // Push it back and return the function as a value
                    self.stack_chirho.push_chirho(other_chirho);
                    return Ok(ReturnActionChirho::DoneChirho(
                        ValueChirho::HeapPtrChirho(addr_chirho),
                    ));
                }
                None => {
                    // Stack empty — function is the final result
                    return Ok(ReturnActionChirho::DoneChirho(
                        ValueChirho::HeapPtrChirho(addr_chirho),
                    ));
                }
            }
        }
    }

    // ── Enter a PAP ─────────────────────────────────────────────────────

    fn enter_pap_chirho(
        &mut self,
        addr_chirho: HeapAddrChirho,
        closure_chirho: &ClosureChirho,
    ) -> Result<ReturnActionChirho, EvalErrorChirho> {
        let remaining_chirho = closure_chirho.info_chirho.arity_chirho as usize;

        match self.stack_chirho.pop_chirho() {
            Some(FrameChirho::ApplyChirho { args_chirho }) => {
                let n_args_chirho = args_chirho.len();
                if n_args_chirho >= remaining_chirho {
                    // Enough args to saturate — get the underlying function,
                    // combine PAP args + new args, enter the function
                    let fun_addr_chirho = match &closure_chirho.payload_chirho[0] {
                        ValueChirho::HeapPtrChirho(a_chirho) => *a_chirho,
                        _ => {
                            return Err(EvalErrorChirho::TypeErrorChirho {
                                message_chirho: "PAP payload[0] is not a heap pointer"
                                    .to_string(),
                            })
                        }
                    };

                    if n_args_chirho > remaining_chirho {
                        // Over-application: push extra args
                        let (_, rest_chirho) = args_chirho.split_at(remaining_chirho);
                        self.stack_chirho.push_chirho(FrameChirho::ApplyChirho {
                            args_chirho: rest_chirho.to_vec(),
                        });
                    }

                    // Load PAP's pre-applied args + new args into registers
                    let mut all_regs_chirho: Vec<ValueChirho> =
                        closure_chirho.payload_chirho[1..].to_vec();
                    let take_chirho = remaining_chirho.min(n_args_chirho);
                    all_regs_chirho.extend_from_slice(&args_chirho[..take_chirho]);
                    self.arg_regs_chirho = all_regs_chirho;

                    // Enter the original function
                    let fun_closure_chirho =
                        self.heap_chirho.read_chirho(fun_addr_chirho).clone();
                    Ok(ReturnActionChirho::ContinueChirho(
                        fun_closure_chirho.info_chirho.entry_chirho.0,
                    ))
                } else {
                    // Still not enough args — build a new PAP with more args
                    let new_remaining_chirho = (remaining_chirho - n_args_chirho) as u16;
                    let fun_addr_chirho = match &closure_chirho.payload_chirho[0] {
                        ValueChirho::HeapPtrChirho(a_chirho) => *a_chirho,
                        _ => {
                            return Err(EvalErrorChirho::TypeErrorChirho {
                                message_chirho: "PAP payload[0] is not a heap pointer"
                                    .to_string(),
                            })
                        }
                    };

                    let mut all_args_chirho: Vec<ValueChirho> =
                        closure_chirho.payload_chirho[1..].to_vec();
                    all_args_chirho.extend(args_chirho);

                    let new_pap_chirho = ClosureChirho::pap_chirho(
                        new_remaining_chirho,
                        fun_addr_chirho,
                        all_args_chirho,
                    );
                    let pap_addr_chirho = self.heap_chirho.alloc_chirho(new_pap_chirho);
                    self.return_con_chirho(pap_addr_chirho)
                }
            }
            Some(FrameChirho::UpdateChirho { thunk_addr_chirho }) => {
                self.heap_chirho
                    .update_to_ind_chirho(thunk_addr_chirho, addr_chirho);
                self.return_con_chirho(addr_chirho)
            }
            Some(other_chirho) => {
                self.stack_chirho.push_chirho(other_chirho);
                Ok(ReturnActionChirho::DoneChirho(
                    ValueChirho::HeapPtrChirho(addr_chirho),
                ))
            }
            None => Ok(ReturnActionChirho::DoneChirho(
                ValueChirho::HeapPtrChirho(addr_chirho),
            )),
        }
    }

    // ── Helpers ─────────────────────────────────────────────────────────

    /// Add an `Enter` instruction to the code table and return its index.
    fn emit_enter_chirho(&mut self, addr_chirho: HeapAddrChirho) -> u32 {
        let idx_chirho = self.code_table_chirho.len() as u32;
        self.code_table_chirho
            .push(CodeChirho::EnterChirho(addr_chirho));
        idx_chirho
    }

    /// Evaluate a primitive operation on the collected arguments.
    fn eval_prim_chirho(
        &mut self,
        op_chirho: PrimOpKindChirho,
        args_chirho: &[ValueChirho],
    ) -> Result<ValueChirho, EvalErrorChirho> {
        // Handle I/O and special primops first
        match op_chirho {
            PrimOpKindChirho::ErrorChirho => {
                let msg_chirho = match args_chirho.first() {
                    Some(ValueChirho::StringChirho(s_chirho)) => s_chirho.clone(),
                    Some(ValueChirho::HeapPtrChirho(a_chirho)) => {
                        self.resolve_string_arg_chirho(*a_chirho)?
                    }
                    _ => "error".to_string(),
                };
                return Err(EvalErrorChirho::RuntimeErrorChirho(msg_chirho));
            }
            PrimOpKindChirho::UndefinedChirho => {
                return Err(EvalErrorChirho::RuntimeErrorChirho(
                    "Prelude.undefined".to_string(),
                ));
            }
            PrimOpKindChirho::SeqChirho => {
                // seq a b = b (a is already evaluated to WHNF by the time we reach here)
                return Ok(args_chirho.get(1).cloned().unwrap_or(ValueChirho::IntChirho(0)));
            }
            PrimOpKindChirho::PutStrLnChirho => {
                if let Some(ValueChirho::StringChirho(s_chirho)) = args_chirho.first() {
                    self.io_output_chirho.push_str(s_chirho);
                    self.io_output_chirho.push('\n');
                    return Ok(ValueChirho::IntChirho(0)); // ()
                } else if let Some(ValueChirho::HeapPtrChirho(addr_chirho)) = args_chirho.first() {
                    let s_chirho = self.resolve_string_arg_chirho(*addr_chirho)?;
                    self.io_output_chirho.push_str(&s_chirho);
                    self.io_output_chirho.push('\n');
                    return Ok(ValueChirho::IntChirho(0));
                }
                return Ok(ValueChirho::IntChirho(0));
            }
            PrimOpKindChirho::PutStrChirho => {
                if let Some(ValueChirho::StringChirho(s_chirho)) = args_chirho.first() {
                    self.io_output_chirho.push_str(s_chirho);
                    return Ok(ValueChirho::IntChirho(0));
                } else if let Some(ValueChirho::HeapPtrChirho(addr_chirho)) = args_chirho.first() {
                    let s_chirho = self.resolve_string_arg_chirho(*addr_chirho)?;
                    self.io_output_chirho.push_str(&s_chirho);
                    return Ok(ValueChirho::IntChirho(0));
                }
                return Ok(ValueChirho::IntChirho(0));
            }
            PrimOpKindChirho::PutCharChirho => {
                match args_chirho.first() {
                    Some(ValueChirho::CharChirho(c_chirho)) => {
                        self.io_output_chirho.push(*c_chirho);
                    }
                    Some(ValueChirho::IntChirho(n_chirho)) => {
                        if let Some(c_chirho) = char::from_u32(*n_chirho as u32) {
                            self.io_output_chirho.push(c_chirho);
                        }
                    }
                    _ => {}
                }
                return Ok(ValueChirho::IntChirho(0));
            }
            PrimOpKindChirho::ReturnIOChirho => {
                // return x = x (in our simplified IO model)
                return Ok(args_chirho.first().cloned().unwrap_or(ValueChirho::IntChirho(0)));
            }
            PrimOpKindChirho::BindIOChirho | PrimOpKindChirho::ThenIOChirho => {
                // For now these are handled at the lowering level
                return Ok(ValueChirho::IntChirho(0));
            }
            PrimOpKindChirho::GetLineChirho => {
                // Read a line from the input feed (or empty string if exhausted)
                let line_chirho = self.io_input_chirho.pop_front().unwrap_or_default();
                return Ok(ValueChirho::StringChirho(line_chirho));
            }
            PrimOpKindChirho::GetCharChirho => {
                // Read a single char from the front of the first input line
                if let Some(front_chirho) = self.io_input_chirho.front_mut() {
                    if let Some(ch_chirho) = front_chirho.chars().next() {
                        // Remove the consumed char
                        front_chirho.drain(..ch_chirho.len_utf8());
                        if front_chirho.is_empty() {
                            self.io_input_chirho.pop_front();
                        }
                        return Ok(ValueChirho::CharChirho(ch_chirho));
                    }
                }
                // If no input, return newline (EOF-like sentinel)
                return Ok(ValueChirho::CharChirho('\n'));
            }
            PrimOpKindChirho::ReadFileChirho => {
                // readFile: for sandboxed testing, return empty string
                // In a real runtime, this would read from the filesystem
                return Ok(ValueChirho::StringChirho(String::new()));
            }
            PrimOpKindChirho::WriteFileChirho => {
                // writeFile: capture to io_output with a marker
                if let (Some(path_chirho), Some(content_chirho)) =
                    (args_chirho.first(), args_chirho.get(1))
                {
                    let path_str_chirho = match path_chirho {
                        ValueChirho::StringChirho(s_chirho) => s_chirho.clone(),
                        ValueChirho::HeapPtrChirho(a_chirho) => self.resolve_string_arg_chirho(*a_chirho)?,
                        _ => String::new(),
                    };
                    let content_str_chirho = match content_chirho {
                        ValueChirho::StringChirho(s_chirho) => s_chirho.clone(),
                        ValueChirho::HeapPtrChirho(a_chirho) => self.resolve_string_arg_chirho(*a_chirho)?,
                        _ => String::new(),
                    };
                    self.io_output_chirho.push_str(&format!(
                        "[writeFile:{path_str_chirho}]{content_str_chirho}"
                    ));
                }
                return Ok(ValueChirho::IntChirho(0));
            }
            PrimOpKindChirho::AppendFileChirho => {
                // appendFile: similar to writeFile
                if let (Some(path_chirho), Some(content_chirho)) =
                    (args_chirho.first(), args_chirho.get(1))
                {
                    let path_str_chirho = match path_chirho {
                        ValueChirho::StringChirho(s_chirho) => s_chirho.clone(),
                        ValueChirho::HeapPtrChirho(a_chirho) => self.resolve_string_arg_chirho(*a_chirho)?,
                        _ => String::new(),
                    };
                    let content_str_chirho = match content_chirho {
                        ValueChirho::StringChirho(s_chirho) => s_chirho.clone(),
                        ValueChirho::HeapPtrChirho(a_chirho) => self.resolve_string_arg_chirho(*a_chirho)?,
                        _ => String::new(),
                    };
                    self.io_output_chirho.push_str(&format!(
                        "[appendFile:{path_str_chirho}]{content_str_chirho}"
                    ));
                }
                return Ok(ValueChirho::IntChirho(0));
            }
            PrimOpKindChirho::EnumFromToChirho => {
                // enumFromTo# from to → build the list [from..to] on the heap.
                let from_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let to_chirho = match args_chirho.get(1) {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };

                let nil_tag_chirho = self.lookup_con_tag_chirho("[]", 0);
                let cons_tag_chirho = self.lookup_con_tag_chirho(":", 1);

                // Build the list backwards from `to` down to `from`.
                let nil_closure_chirho = ClosureChirho::con_chirho(
                    nil_tag_chirho, "[]", vec![],
                );
                let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);

                let mut i_chirho = to_chirho;
                while i_chirho >= from_chirho {
                    let cons_closure_chirho = ClosureChirho::con_chirho(
                        cons_tag_chirho, ":",
                        vec![
                            ValueChirho::IntChirho(i_chirho),
                            ValueChirho::HeapPtrChirho(tail_addr_chirho),
                        ],
                    );
                    tail_addr_chirho = self.heap_chirho.alloc_chirho(cons_closure_chirho);
                    i_chirho -= 1;
                }

                return Ok(ValueChirho::HeapPtrChirho(tail_addr_chirho));
            }
            PrimOpKindChirho::EnumFromChirho => {
                // enumFrom# from → [from..from+10000] (capped infinite list)
                let from_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let cap_chirho = from_chirho + 10_000;
                let nil_tag_chirho = self.lookup_con_tag_chirho("[]", 0);
                let cons_tag_chirho = self.lookup_con_tag_chirho(":", 1);
                let nil_closure_chirho = ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
                let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);
                let mut i_chirho = cap_chirho;
                while i_chirho >= from_chirho {
                    let cons_closure_chirho = ClosureChirho::con_chirho(
                        cons_tag_chirho, ":",
                        vec![
                            ValueChirho::IntChirho(i_chirho),
                            ValueChirho::HeapPtrChirho(tail_addr_chirho),
                        ],
                    );
                    tail_addr_chirho = self.heap_chirho.alloc_chirho(cons_closure_chirho);
                    i_chirho -= 1;
                }
                return Ok(ValueChirho::HeapPtrChirho(tail_addr_chirho));
            }
            PrimOpKindChirho::EnumFromThenChirho => {
                // enumFromThen# from then → [from,then..] (capped at 10000 elements)
                let from_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let then_chirho = match args_chirho.get(1) {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let step_chirho = then_chirho - from_chirho;
                let nil_tag_chirho = self.lookup_con_tag_chirho("[]", 0);
                let cons_tag_chirho = self.lookup_con_tag_chirho(":", 1);
                // Generate up to 10000 elements
                let mut values_chirho: Vec<i64> = Vec::new();
                let mut current_chirho = from_chirho;
                for _ in 0..10_000 {
                    values_chirho.push(current_chirho);
                    current_chirho += step_chirho;
                }
                // Build list backwards
                let nil_closure_chirho = ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
                let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);
                for val_chirho in values_chirho.into_iter().rev() {
                    let cons_closure_chirho = ClosureChirho::con_chirho(
                        cons_tag_chirho, ":",
                        vec![
                            ValueChirho::IntChirho(val_chirho),
                            ValueChirho::HeapPtrChirho(tail_addr_chirho),
                        ],
                    );
                    tail_addr_chirho = self.heap_chirho.alloc_chirho(cons_closure_chirho);
                }
                return Ok(ValueChirho::HeapPtrChirho(tail_addr_chirho));
            }
            PrimOpKindChirho::EnumFromThenToChirho => {
                // enumFromThenTo# from then to → [from,then..to]
                let from_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let then_chirho = match args_chirho.get(1) {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let to_chirho = match args_chirho.get(2) {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let step_chirho = then_chirho - from_chirho;
                let nil_tag_chirho = self.lookup_con_tag_chirho("[]", 0);
                let cons_tag_chirho = self.lookup_con_tag_chirho(":", 1);
                // Collect values
                let mut values_chirho: Vec<i64> = Vec::new();
                let mut current_chirho = from_chirho;
                if step_chirho > 0 {
                    while current_chirho <= to_chirho {
                        values_chirho.push(current_chirho);
                        current_chirho += step_chirho;
                    }
                } else if step_chirho < 0 {
                    while current_chirho >= to_chirho {
                        values_chirho.push(current_chirho);
                        current_chirho += step_chirho;
                    }
                }
                // Build list backwards
                let nil_closure_chirho = ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
                let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);
                for val_chirho in values_chirho.into_iter().rev() {
                    let cons_closure_chirho = ClosureChirho::con_chirho(
                        cons_tag_chirho, ":",
                        vec![
                            ValueChirho::IntChirho(val_chirho),
                            ValueChirho::HeapPtrChirho(tail_addr_chirho),
                        ],
                    );
                    tail_addr_chirho = self.heap_chirho.alloc_chirho(cons_closure_chirho);
                }
                return Ok(ValueChirho::HeapPtrChirho(tail_addr_chirho));
            }
            PrimOpKindChirho::ShowListChirho => {
                // showList# list → String representation "[1,2,3]"
                // Walk the list cells on the heap, resolving indirections
                // and Box wrappers on element values.
                if let Some(ValueChirho::HeapPtrChirho(addr_chirho)) = args_chirho.first() {
                    let mut elements_chirho: Vec<String> = Vec::new();
                    let mut current_chirho = *addr_chirho;

                    loop {
                        // Follow indirections on the list spine
                        let resolved_chirho = self.heap_chirho.follow_ind_chirho(current_chirho);
                        let closure_chirho = self.heap_chirho.read_chirho(resolved_chirho).clone();
                        let name_chirho = &closure_chirho.info_chirho.name_chirho;

                        if name_chirho == "[]" {
                            break;
                        } else if name_chirho == ":" {
                            // Cons cell: payload[0] = head, payload[1] = tail
                            if closure_chirho.payload_chirho.len() >= 2 {
                                let head_chirho = &closure_chirho.payload_chirho[0];
                                // Resolve the head value through HeapPtr/indirections/Box
                                let resolved_head_chirho =
                                    self.resolve_heap_value_chirho(head_chirho);
                                let elem_str_chirho = match &resolved_head_chirho {
                                    ValueChirho::IntChirho(n_chirho) => n_chirho.to_string(),
                                    ValueChirho::FloatChirho(f_chirho) => format!("{}", f_chirho),
                                    ValueChirho::CharChirho(c_chirho) => format!("'{}'", c_chirho),
                                    ValueChirho::StringChirho(s_chirho) => {
                                        format!("\"{}\"", s_chirho)
                                    }
                                    ValueChirho::BoolChirho(b_chirho) => {
                                        if *b_chirho {
                                            "True".to_string()
                                        } else {
                                            "False".to_string()
                                        }
                                    }
                                    _ => "?".to_string(),
                                };
                                elements_chirho.push(elem_str_chirho);

                                match &closure_chirho.payload_chirho[1] {
                                    ValueChirho::HeapPtrChirho(tail_chirho) => {
                                        current_chirho = *tail_chirho;
                                    }
                                    _ => break,
                                }
                            } else {
                                break;
                            }
                        } else {
                            // Not a list constructor — try showing as a value
                            break;
                        }
                    }

                    let result_chirho =
                        format!("[{}]", elements_chirho.join(","));
                    return Ok(ValueChirho::StringChirho(result_chirho));
                }
                return Ok(ValueChirho::StringChirho("[]".to_string()));
            }
            PrimOpKindChirho::ShowMaybeChirho => {
                // showMaybe# val → "Nothing" | "Just <inner>"
                if let Some(val_chirho) = args_chirho.first() {
                    return Ok(ValueChirho::StringChirho(
                        self.show_value_as_string_chirho(val_chirho),
                    ));
                }
                return Ok(ValueChirho::StringChirho("Nothing".to_string()));
            }
            PrimOpKindChirho::ShowTuple2Chirho => {
                // showTuple2# val → "(a,b)"
                if let Some(val_chirho) = args_chirho.first() {
                    return Ok(ValueChirho::StringChirho(
                        self.show_value_as_string_chirho(val_chirho),
                    ));
                }
                return Ok(ValueChirho::StringChirho("(,)".to_string()));
            }
            PrimOpKindChirho::InteractChirho => {
                // interact f = getContents >>= putStr . f
                // In our sandboxed runtime: read all io_input, apply f, write result
                // For now: collect all stdin lines, pass as single string, write output
                let all_input_chirho: String = self.io_input_chirho.drain(..).collect::<Vec<_>>().join("\n");
                // If the first arg is a closure (function), we'd need to apply it.
                // For the sandboxed test runtime, just write the input to output.
                self.io_output_chirho.push_str(&all_input_chirho);
                return Ok(ValueChirho::IntChirho(0));
            }
            PrimOpKindChirho::PrintChirho => {
                // print x = putStrLn (show x)
                if let Some(val_chirho) = args_chirho.first() {
                    let shown_chirho = self.show_value_as_string_chirho(val_chirho);
                    self.io_output_chirho.push_str(&shown_chirho);
                    self.io_output_chirho.push('\n');
                }
                return Ok(ValueChirho::IntChirho(0));
            }
            PrimOpKindChirho::LinesChirho => {
                // lines# s → split on newlines, build list of strings on heap
                if let Some(ValueChirho::StringChirho(s_chirho)) = args_chirho.first() {
                    let line_strs_chirho: Vec<&str> = s_chirho.split('\n').collect();
                    let nil_tag_chirho = self.lookup_con_tag_chirho("[]", 0);
                    let cons_tag_chirho = self.lookup_con_tag_chirho(":", 1);
                    let nil_closure_chirho = ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
                    let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);
                    for line_chirho in line_strs_chirho.into_iter().rev() {
                        let cons_closure_chirho = ClosureChirho::con_chirho(
                            cons_tag_chirho, ":",
                            vec![
                                ValueChirho::StringChirho(line_chirho.to_string()),
                                ValueChirho::HeapPtrChirho(tail_addr_chirho),
                            ],
                        );
                        tail_addr_chirho = self.heap_chirho.alloc_chirho(cons_closure_chirho);
                    }
                    return Ok(ValueChirho::HeapPtrChirho(tail_addr_chirho));
                }
                return Ok(ValueChirho::StringChirho(String::new()));
            }
            PrimOpKindChirho::UnlinesChirho => {
                // unlines# list → join strings with newlines (each line gets trailing \n)
                if let Some(ValueChirho::HeapPtrChirho(addr_chirho)) = args_chirho.first() {
                    let strings_chirho = self.collect_string_list_chirho(*addr_chirho);
                    let result_chirho = strings_chirho.iter()
                        .map(|s_chirho| format!("{}\n", s_chirho))
                        .collect::<String>();
                    return Ok(ValueChirho::StringChirho(result_chirho));
                }
                return Ok(ValueChirho::StringChirho(String::new()));
            }
            PrimOpKindChirho::FromIntegralChirho => {
                // fromIntegral# n → convert Int to Double
                if let Some(ValueChirho::IntChirho(n_chirho)) = args_chirho.first() {
                    return Ok(ValueChirho::FloatChirho(*n_chirho as f64));
                }
                return Ok(ValueChirho::FloatChirho(0.0));
            }
            PrimOpKindChirho::CeilingChirho => {
                // ceiling# x → round Double up to Int
                if let Some(ValueChirho::FloatChirho(f_chirho)) = args_chirho.first() {
                    return Ok(ValueChirho::IntChirho(f_chirho.ceil() as i64));
                }
                return Ok(ValueChirho::IntChirho(0));
            }
            PrimOpKindChirho::FloorChirho => {
                // floor# x → round Double down to Int
                if let Some(ValueChirho::FloatChirho(f_chirho)) = args_chirho.first() {
                    return Ok(ValueChirho::IntChirho(f_chirho.floor() as i64));
                }
                return Ok(ValueChirho::IntChirho(0));
            }
            PrimOpKindChirho::RoundChirho => {
                // round# x → round Double to nearest Int
                if let Some(ValueChirho::FloatChirho(f_chirho)) = args_chirho.first() {
                    return Ok(ValueChirho::IntChirho(f_chirho.round() as i64));
                }
                return Ok(ValueChirho::IntChirho(0));
            }
            PrimOpKindChirho::TruncateChirho => {
                // truncate# x → truncate Double toward zero
                if let Some(ValueChirho::FloatChirho(f_chirho)) = args_chirho.first() {
                    return Ok(ValueChirho::IntChirho(f_chirho.trunc() as i64));
                }
                return Ok(ValueChirho::IntChirho(0));
            }
            PrimOpKindChirho::TakeStrChirho => {
                // takeStr# n s → take first n elements
                // Works on both String values and heap-allocated lists.
                let n_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => (*n_chirho).max(0) as usize,
                    _ => return Ok(ValueChirho::StringChirho(String::new())),
                };
                match args_chirho.get(1) {
                    Some(ValueChirho::StringChirho(s_chirho)) => {
                        let result_chirho: String = s_chirho.chars().take(n_chirho).collect();
                        return Ok(ValueChirho::StringChirho(result_chirho));
                    }
                    Some(ValueChirho::HeapPtrChirho(addr_chirho)) => {
                        // Walk heap cons list and take first n elements
                        let nil_tag_chirho = self.lookup_con_tag_chirho("[]", 0);
                        let cons_tag_chirho = self.lookup_con_tag_chirho(":", 1);
                        let mut elements_chirho: Vec<ValueChirho> = Vec::new();
                        let mut current_chirho = *addr_chirho;
                        for _ in 0..n_chirho {
                            let resolved_chirho = self.heap_chirho.follow_ind_chirho(current_chirho);
                            let closure_chirho = self.heap_chirho.read_chirho(resolved_chirho).clone();
                            if closure_chirho.info_chirho.name_chirho == "[]" {
                                break;
                            }
                            if closure_chirho.info_chirho.name_chirho == ":"
                                && closure_chirho.payload_chirho.len() >= 2
                            {
                                elements_chirho.push(closure_chirho.payload_chirho[0].clone());
                                match &closure_chirho.payload_chirho[1] {
                                    ValueChirho::HeapPtrChirho(tail_chirho) => current_chirho = *tail_chirho,
                                    _ => break,
                                }
                            } else {
                                break;
                            }
                        }
                        // Build result list backwards
                        let nil_closure_chirho = ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
                        let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);
                        for elem_chirho in elements_chirho.into_iter().rev() {
                            let cons_closure_chirho = ClosureChirho::con_chirho(
                                cons_tag_chirho, ":",
                                vec![elem_chirho, ValueChirho::HeapPtrChirho(tail_addr_chirho)],
                            );
                            tail_addr_chirho = self.heap_chirho.alloc_chirho(cons_closure_chirho);
                        }
                        return Ok(ValueChirho::HeapPtrChirho(tail_addr_chirho));
                    }
                    _ => return Ok(ValueChirho::StringChirho(String::new())),
                }
            }
            PrimOpKindChirho::DropStrChirho => {
                // dropStr# n s → drop first n elements
                // Works on both String values and heap-allocated lists.
                let n_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => (*n_chirho).max(0) as usize,
                    _ => return Ok(ValueChirho::StringChirho(String::new())),
                };
                match args_chirho.get(1) {
                    Some(ValueChirho::StringChirho(s_chirho)) => {
                        let result_chirho: String = s_chirho.chars().skip(n_chirho).collect();
                        return Ok(ValueChirho::StringChirho(result_chirho));
                    }
                    Some(ValueChirho::HeapPtrChirho(addr_chirho)) => {
                        // Walk heap cons list, skip first n elements, return rest
                        let mut current_chirho = *addr_chirho;
                        for _ in 0..n_chirho {
                            let resolved_chirho = self.heap_chirho.follow_ind_chirho(current_chirho);
                            let closure_chirho = self.heap_chirho.read_chirho(resolved_chirho).clone();
                            if closure_chirho.info_chirho.name_chirho == "[]" {
                                return Ok(ValueChirho::HeapPtrChirho(resolved_chirho));
                            }
                            if closure_chirho.info_chirho.name_chirho == ":"
                                && closure_chirho.payload_chirho.len() >= 2
                            {
                                match &closure_chirho.payload_chirho[1] {
                                    ValueChirho::HeapPtrChirho(tail_chirho) => current_chirho = *tail_chirho,
                                    _ => return Ok(ValueChirho::HeapPtrChirho(resolved_chirho)),
                                }
                            } else {
                                return Ok(ValueChirho::HeapPtrChirho(resolved_chirho));
                            }
                        }
                        return Ok(ValueChirho::HeapPtrChirho(current_chirho));
                    }
                    _ => return Ok(ValueChirho::StringChirho(String::new())),
                }
            }
            PrimOpKindChirho::WordsStrChirho => {
                // wordsStr# s → split on whitespace, build list of strings on heap
                if let Some(ValueChirho::StringChirho(s_chirho)) = args_chirho.first() {
                    let words_chirho: Vec<&str> = s_chirho.split_whitespace().collect();
                    let nil_tag_chirho = self.lookup_con_tag_chirho("[]", 0);
                    let cons_tag_chirho = self.lookup_con_tag_chirho(":", 1);
                    let nil_closure_chirho = ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
                    let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);
                    for word_chirho in words_chirho.into_iter().rev() {
                        let cons_closure_chirho = ClosureChirho::con_chirho(
                            cons_tag_chirho, ":",
                            vec![
                                ValueChirho::StringChirho(word_chirho.to_string()),
                                ValueChirho::HeapPtrChirho(tail_addr_chirho),
                            ],
                        );
                        tail_addr_chirho = self.heap_chirho.alloc_chirho(cons_closure_chirho);
                    }
                    return Ok(ValueChirho::HeapPtrChirho(tail_addr_chirho));
                }
                return Ok(ValueChirho::StringChirho(String::new()));
            }
            PrimOpKindChirho::UnwordsStrChirho => {
                // unwordsStr# list → join strings with spaces
                if let Some(ValueChirho::HeapPtrChirho(addr_chirho)) = args_chirho.first() {
                    let strings_chirho = self.collect_string_list_chirho(*addr_chirho);
                    return Ok(ValueChirho::StringChirho(strings_chirho.join(" ")));
                }
                return Ok(ValueChirho::StringChirho(String::new()));
            }
            PrimOpKindChirho::ConcatStrChirho => {
                // concatStr# list → concatenate all strings
                if let Some(ValueChirho::HeapPtrChirho(addr_chirho)) = args_chirho.first() {
                    let strings_chirho = self.collect_string_list_chirho(*addr_chirho);
                    return Ok(ValueChirho::StringChirho(strings_chirho.concat()));
                }
                return Ok(ValueChirho::StringChirho(String::new()));
            }
            PrimOpKindChirho::IntercalateStrChirho => {
                // intercalateStr# sep list → join strings with separator
                if let (Some(ValueChirho::StringChirho(sep_chirho)), Some(ValueChirho::HeapPtrChirho(addr_chirho))) =
                    (args_chirho.first(), args_chirho.get(1))
                {
                    let strings_chirho = self.collect_string_list_chirho(*addr_chirho);
                    return Ok(ValueChirho::StringChirho(strings_chirho.join(sep_chirho)));
                }
                return Ok(ValueChirho::StringChirho(String::new()));
            }
            PrimOpKindChirho::AppendStrChirho => {
                // String concatenation — resolve HeapPtr args (cons list of Char)
                // back to String values so the binop handler can concatenate them.
                let a_chirho = self.resolve_value_to_string_chirho(&args_chirho[0])?;
                let b_chirho = self.resolve_value_to_string_chirho(&args_chirho[1])?;
                return Ok(ValueChirho::StringChirho(format!("{}{}", a_chirho, b_chirho)));
            }
            PrimOpKindChirho::CompareIntChirho => {
                let a_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let b_chirho = match args_chirho.get(1) {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let ordering_chirho = self.make_ordering_chirho(a_chirho.cmp(&b_chirho));
                return Ok(ordering_chirho);
            }
            PrimOpKindChirho::CompareCharChirho => {
                let a_chirho = match args_chirho.first() {
                    Some(ValueChirho::CharChirho(c_chirho)) => *c_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let b_chirho = match args_chirho.get(1) {
                    Some(ValueChirho::CharChirho(c_chirho)) => *c_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let ordering_chirho = self.make_ordering_chirho(a_chirho.cmp(&b_chirho));
                return Ok(ordering_chirho);
            }
            PrimOpKindChirho::CompareFloatChirho => {
                let a_chirho = match args_chirho.first() {
                    Some(ValueChirho::FloatChirho(n_chirho)) => *n_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let b_chirho = match args_chirho.get(1) {
                    Some(ValueChirho::FloatChirho(n_chirho)) => *n_chirho,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let ordering_chirho = self.make_ordering_chirho(
                    a_chirho.partial_cmp(&b_chirho).unwrap_or(std::cmp::Ordering::Equal),
                );
                return Ok(ordering_chirho);
            }
            // ── IORef operations ──
            PrimOpKindChirho::NewIORefChirho => {
                // newIORef# val → allocate a new mutable reference, return its id as Int
                let val_chirho = args_chirho.first().cloned().unwrap_or(ValueChirho::IntChirho(0));
                let id_chirho = self.next_ioref_chirho;
                self.next_ioref_chirho += 1;
                self.iorefs_chirho.insert(id_chirho, val_chirho);
                return Ok(ValueChirho::IntChirho(id_chirho as i64));
            }
            PrimOpKindChirho::ReadIORefChirho => {
                // readIORef# ref → read the current value from the mutable reference
                let ref_id_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho as u64,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let val_chirho = self.iorefs_chirho.get(&ref_id_chirho).cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                // If the stored value is a HeapPtr (thunk), force it to WHNF
                if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                    match self.force_addr_to_whnf_chirho(addr_chirho) {
                        Ok(forced_chirho) => {
                            let resolved_chirho = self.resolve_heap_value_chirho(&ValueChirho::HeapPtrChirho(forced_chirho));
                            return Ok(resolved_chirho);
                        }
                        Err(_) => return Ok(ValueChirho::HeapPtrChirho(addr_chirho)),
                    }
                }
                return Ok(val_chirho);
            }
            PrimOpKindChirho::WriteIORefChirho => {
                // writeIORef# ref val → overwrite the value in the mutable reference
                let ref_id_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho as u64,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let mut val_chirho = args_chirho.get(1).cloned().unwrap_or(ValueChirho::IntChirho(0));
                // Force thunks to WHNF before storing
                if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                    match self.force_addr_to_whnf_chirho(addr_chirho) {
                        Ok(forced_chirho) => {
                            val_chirho = self.resolve_heap_value_chirho(&ValueChirho::HeapPtrChirho(forced_chirho));
                        }
                        Err(_) => {}
                    }
                }
                self.iorefs_chirho.insert(ref_id_chirho, val_chirho);
                return Ok(ValueChirho::IntChirho(0)); // IO ()
            }
            PrimOpKindChirho::ModifyIORefChirho => {
                // modifyIORef# ref f → read value, apply f, write back
                // Since we can't easily apply a closure in the primop handler,
                // we treat this as: read the ref, push an apply frame for f,
                // then write the result back.
                // Simplified: for now, if f is an Int (identity-like), just keep the value.
                let ref_id_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho as u64,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                // Read current value
                let current_chirho = self.iorefs_chirho.get(&ref_id_chirho).cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                // If second arg is a function closure, we'd need to apply it.
                // For the basic case, if we have a HeapPtr for the function,
                // we push a special continuation. For now, just return the
                // current value — the full apply-and-writeback needs the
                // evaluator loop. Users should use readIORef + writeIORef for now.
                // TODO: full modifyIORef with closure application
                let _ = current_chirho;
                return Ok(ValueChirho::IntChirho(0)); // IO ()
            }
            _ => {}
        }

        // Resolve HeapPtr arguments to their underlying values when
        // they point to nullary constructors (e.g. True, False, Nothing).
        // This allows primops like showBool# to receive a concrete value
        // instead of a raw heap address.
        let resolved_args_chirho: Vec<ValueChirho> = args_chirho
            .iter()
            .map(|v_chirho| {
                if let ValueChirho::HeapPtrChirho(addr_chirho) = v_chirho {
                    let resolved_addr_chirho = self.heap_chirho.follow_ind_chirho(*addr_chirho);
                    let closure_chirho = self.heap_chirho.read_chirho(resolved_addr_chirho).clone();
                    let name_chirho = &closure_chirho.info_chirho.name_chirho;
                    match name_chirho.as_str() {
                        "True" => ValueChirho::BoolChirho(true),
                        "False" => ValueChirho::BoolChirho(false),
                        _ => v_chirho.clone(),
                    }
                } else {
                    v_chirho.clone()
                }
            })
            .collect();

        match resolved_args_chirho.len() {
            2 => apply_prim_binop_chirho(op_chirho, &resolved_args_chirho[0], &resolved_args_chirho[1])
                .map_err(EvalErrorChirho::PrimFailChirho),
            1 => {
                // Unary ops (NegInt)
                apply_prim_binop_chirho(
                    op_chirho,
                    &resolved_args_chirho[0],
                    &ValueChirho::IntChirho(0), // dummy second arg for unary
                )
                .map_err(EvalErrorChirho::PrimFailChirho)
            }
            n_chirho => Err(EvalErrorChirho::TypeErrorChirho {
                message_chirho: format!(
                    "primop {:?} got {} args, expected 1 or 2",
                    op_chirho, n_chirho
                ),
            }),
        }
    }

    /// Collect strings from a heap-allocated list [String].
    fn collect_string_list_chirho(&mut self, start_addr_chirho: HeapAddrChirho) -> Vec<String> {
        let mut result_chirho = Vec::new();
        let mut current_chirho = start_addr_chirho;
        loop {
            // Force the current address to WHNF in case it's a thunk
            let resolved_chirho = match self.force_addr_to_whnf_chirho(current_chirho) {
                Ok(a_chirho) => a_chirho,
                Err(_) => self.heap_chirho.follow_ind_chirho(current_chirho),
            };
            let closure_chirho = self.heap_chirho.read_chirho(resolved_chirho).clone();
            let name_chirho = &closure_chirho.info_chirho.name_chirho;
            if name_chirho == "[]" {
                break;
            } else if name_chirho == ":" && closure_chirho.payload_chirho.len() >= 2 {
                // Force head element too
                let head_val_chirho = match &closure_chirho.payload_chirho[0] {
                    ValueChirho::HeapPtrChirho(h_chirho) => {
                        match self.force_addr_to_whnf_chirho(*h_chirho) {
                            Ok(forced_chirho) => {
                                self.resolve_heap_value_chirho(&ValueChirho::HeapPtrChirho(forced_chirho))
                            }
                            Err(_) => self.resolve_heap_value_chirho(&closure_chirho.payload_chirho[0]),
                        }
                    }
                    other_chirho => self.resolve_heap_value_chirho(other_chirho),
                };
                if let ValueChirho::StringChirho(s_chirho) = head_val_chirho {
                    result_chirho.push(s_chirho);
                }
                match &closure_chirho.payload_chirho[1] {
                    ValueChirho::HeapPtrChirho(tail_chirho) => current_chirho = *tail_chirho,
                    _ => break,
                }
            } else {
                break;
            }
        }
        result_chirho
    }

    /// Resolve a value through HeapPtr indirections and Box/I#/D#/C#
    /// wrappers to get the underlying primitive value.
    fn resolve_heap_value_chirho(&self, val_chirho: &ValueChirho) -> ValueChirho {
        match val_chirho {
            ValueChirho::HeapPtrChirho(addr_chirho) => {
                let resolved_chirho = self.heap_chirho.follow_ind_chirho(*addr_chirho);
                let closure_chirho = self.heap_chirho.read_chirho(resolved_chirho);
                let name_chirho = &closure_chirho.info_chirho.name_chirho;
                match name_chirho.as_str() {
                    "I#" | "Box" => {
                        if let Some(inner_chirho) = closure_chirho.payload_chirho.first() {
                            self.resolve_heap_value_chirho(inner_chirho)
                        } else {
                            val_chirho.clone()
                        }
                    }
                    "D#" => {
                        if let Some(inner_chirho) = closure_chirho.payload_chirho.first() {
                            self.resolve_heap_value_chirho(inner_chirho)
                        } else {
                            val_chirho.clone()
                        }
                    }
                    "C#" => {
                        if let Some(inner_chirho) = closure_chirho.payload_chirho.first() {
                            self.resolve_heap_value_chirho(inner_chirho)
                        } else {
                            val_chirho.clone()
                        }
                    }
                    "Addr#" => {
                        if let Some(inner_chirho) = closure_chirho.payload_chirho.first() {
                            inner_chirho.clone()
                        } else {
                            val_chirho.clone()
                        }
                    }
                    "True" => ValueChirho::BoolChirho(true),
                    "False" => ValueChirho::BoolChirho(false),
                    _ => {
                        // Single-field constructor — try extracting its payload
                        if closure_chirho.payload_chirho.len() == 1 {
                            self.resolve_heap_value_chirho(
                                &closure_chirho.payload_chirho[0],
                            )
                        } else {
                            val_chirho.clone()
                        }
                    }
                }
            }
            other_chirho => other_chirho.clone(),
        }
    }

    /// Show any resolved value as a Haskell-style string.
    /// Used by showList#, showMaybe#, showTuple2# primops.
    /// Does NOT use resolve_heap_value_chirho first — preserves constructor
    /// structure so we can display "Just 42" instead of just "42".
    fn show_value_as_string_chirho(&mut self, val_chirho: &ValueChirho) -> String {
        match val_chirho {
            ValueChirho::IntChirho(n_chirho) => n_chirho.to_string(),
            ValueChirho::FloatChirho(f_chirho) => format!("{}", f_chirho),
            ValueChirho::CharChirho(c_chirho) => format!("'{}'", c_chirho),
            ValueChirho::StringChirho(s_chirho) => format!("\"{}\"", s_chirho),
            ValueChirho::BoolChirho(b_chirho) => {
                if *b_chirho { "True".to_string() } else { "False".to_string() }
            }
            ValueChirho::HeapPtrChirho(addr_chirho) => {
                // Force thunks first
                let forced_addr_chirho = match self.force_addr_to_whnf_chirho(*addr_chirho) {
                    Ok(a_chirho) => a_chirho,
                    Err(_) => return "?".to_string(),
                };
                let closure_chirho = self.heap_chirho.read_chirho(forced_addr_chirho).clone();
                let name_chirho = &closure_chirho.info_chirho.name_chirho;
                match name_chirho.as_str() {
                    // Unboxing wrappers
                    "I#" | "Box" | "D#" | "C#" | "Addr#" => {
                        if let Some(inner_chirho) = closure_chirho.payload_chirho.first() {
                            return self.show_value_as_string_chirho(inner_chirho);
                        }
                        "?".to_string()
                    }
                    "True" => "True".to_string(),
                    "False" => "False".to_string(),
                    "Nothing" => "Nothing".to_string(),
                    "Just" => {
                        if let Some(inner_chirho) = closure_chirho.payload_chirho.first() {
                            format!("Just {}", self.show_value_as_string_chirho(inner_chirho))
                        } else {
                            "Just ?".to_string()
                        }
                    }
                    "(,)" | "$tuple2" => {
                        if closure_chirho.payload_chirho.len() >= 2 {
                            let a_chirho = self.show_value_as_string_chirho(&closure_chirho.payload_chirho[0]);
                            let b_chirho = self.show_value_as_string_chirho(&closure_chirho.payload_chirho[1]);
                            format!("({},{})", a_chirho, b_chirho)
                        } else {
                            "(?,?)".to_string()
                        }
                    }
                    "[]" => "[]".to_string(),
                    ":" => {
                        // Show as a list
                        let mut elements_chirho: Vec<String> = Vec::new();
                        let mut cur_chirho = forced_addr_chirho;
                        loop {
                            let cl_chirho = self.heap_chirho.read_chirho(cur_chirho).clone();
                            let cn_chirho = &cl_chirho.info_chirho.name_chirho;
                            if cn_chirho == "[]" { break; }
                            if cn_chirho == ":" && cl_chirho.payload_chirho.len() >= 2 {
                                elements_chirho.push(self.show_value_as_string_chirho(&cl_chirho.payload_chirho[0]));
                                match &cl_chirho.payload_chirho[1] {
                                    ValueChirho::HeapPtrChirho(t_chirho) => {
                                        cur_chirho = self.heap_chirho.follow_ind_chirho(*t_chirho);
                                    }
                                    _ => break,
                                }
                            } else { break; }
                        }
                        format!("[{}]", elements_chirho.join(","))
                    }
                    _ => {
                        // Generic constructor: "Con field1 field2 ..."
                        if closure_chirho.payload_chirho.is_empty() {
                            name_chirho.clone()
                        } else {
                            let fields_chirho: Vec<String> = closure_chirho.payload_chirho.iter()
                                .map(|f_chirho| self.show_value_as_string_chirho(f_chirho))
                                .collect();
                            format!("{} {}", name_chirho, fields_chirho.join(" "))
                        }
                    }
                }
            }
            _ => "?".to_string(),
        }
    }

    /// Look up the tag for a constructor name, falling back to a default.
    fn lookup_con_tag_chirho(&self, name_chirho: &str, default_chirho: u16) -> DataConTagChirho {
        DataConTagChirho(
            self.con_tags_chirho
                .get(name_chirho)
                .copied()
                .unwrap_or(default_chirho),
        )
    }

    /// Force a heap address to WHNF by entering it if it's a thunk.
    /// Saves and restores machine state (stack, arg_regs) so it can be
    /// called from within primop handlers without disrupting evaluation.
    fn force_addr_to_whnf_chirho(
        &mut self,
        addr_chirho: HeapAddrChirho,
    ) -> Result<HeapAddrChirho, EvalErrorChirho> {
        let resolved_chirho = self.heap_chirho.follow_ind_chirho(addr_chirho);
        let tag_chirho = self.heap_chirho.read_chirho(resolved_chirho).info_chirho.tag_chirho;
        match tag_chirho {
            InfoTagChirho::ConChirho => Ok(resolved_chirho),
            InfoTagChirho::ThunkChirho | InfoTagChirho::PapChirho | InfoTagChirho::FunChirho => {
                let saved_stack_chirho = std::mem::replace(
                    &mut self.stack_chirho,
                    StackChirho::new_chirho(),
                );
                let saved_regs_chirho = std::mem::take(&mut self.arg_regs_chirho);
                let entry_chirho = self.code_table_chirho.len() as u32;
                self.code_table_chirho.push(CodeChirho::EnterChirho(resolved_chirho));
                let _result_chirho = self.run_chirho(entry_chirho)?;
                self.stack_chirho = saved_stack_chirho;
                self.arg_regs_chirho = saved_regs_chirho;
                Ok(self.heap_chirho.follow_ind_chirho(addr_chirho))
            }
            _ => Ok(resolved_chirho),
        }
    }

    /// Resolve a heap pointer to a string. Handles both:
    /// - Closures wrapping a StringChirho payload (e.g. Addr# "hello")
    /// - Cons lists of Char values (e.g. 'h' : 'e' : 'l' : [])
    /// Forces thunks in cons tails so lazy results from `take` etc. work.
    fn resolve_string_arg_chirho(
        &mut self,
        addr_chirho: HeapAddrChirho,
    ) -> Result<String, EvalErrorChirho> {
        let resolved_chirho = self.force_addr_to_whnf_chirho(addr_chirho)?;
        let closure_chirho = self.heap_chirho.read_chirho(resolved_chirho);
        // Check for direct StringChirho in payload
        if let Some(ValueChirho::StringChirho(s_chirho)) = closure_chirho.payload_chirho.first() {
            return Ok(s_chirho.clone());
        }
        // Check for cons list of Chars
        let name_chirho = closure_chirho.info_chirho.name_chirho.clone();
        if name_chirho == ":" || name_chirho == "[]" {
            let mut result_chirho = String::new();
            let mut current_chirho = resolved_chirho;
            loop {
                let c_chirho = self.heap_chirho.read_chirho(current_chirho).clone();
                if c_chirho.info_chirho.name_chirho == "[]" {
                    break;
                }
                if c_chirho.info_chirho.name_chirho == ":" && c_chirho.payload_chirho.len() >= 2 {
                    match &c_chirho.payload_chirho[0] {
                        ValueChirho::CharChirho(ch_chirho) => result_chirho.push(*ch_chirho),
                        ValueChirho::StringChirho(s_chirho) => {
                            result_chirho.push_str(s_chirho);
                            break;
                        }
                        ValueChirho::HeapPtrChirho(head_addr_chirho) => {
                            // Force the head thunk (e.g. from `map toUpper`)
                            let head_resolved_chirho = self.force_addr_to_whnf_chirho(*head_addr_chirho)?;
                            let head_closure_chirho = self.heap_chirho.read_chirho(head_resolved_chirho).clone();
                            if let Some(ValueChirho::CharChirho(ch_chirho)) = head_closure_chirho.payload_chirho.first() {
                                result_chirho.push(*ch_chirho);
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                    match &c_chirho.payload_chirho[1] {
                        ValueChirho::HeapPtrChirho(tail_chirho) => {
                            // Force the tail — it may be a thunk (lazy take/drop result)
                            current_chirho = self.force_addr_to_whnf_chirho(*tail_chirho)?;
                        }
                        ValueChirho::StringChirho(s_chirho) => {
                            result_chirho.push_str(s_chirho);
                            break;
                        }
                        _ => break,
                    }
                } else {
                    break;
                }
            }
            return Ok(result_chirho);
        }
        Ok(String::new())
    }

    /// Allocate an Ordering constructor (LT, EQ, GT) on the heap and return
    /// a HeapPtrChirho to it.
    fn make_ordering_chirho(&mut self, ord_chirho: std::cmp::Ordering) -> ValueChirho {
        let (name_chirho, tag_idx_chirho) = match ord_chirho {
            std::cmp::Ordering::Less => ("LT", 0u16),
            std::cmp::Ordering::Equal => ("EQ", 1u16),
            std::cmp::Ordering::Greater => ("GT", 2u16),
        };
        let tag_chirho = self.lookup_con_tag_chirho(name_chirho, tag_idx_chirho);
        let closure_chirho = ClosureChirho::con_chirho(tag_chirho, name_chirho, vec![]);
        let addr_chirho = self.heap_chirho.alloc_chirho(closure_chirho);
        ValueChirho::HeapPtrChirho(addr_chirho)
    }

    /// Resolve any ValueChirho to a String, handling StringChirho directly and
    /// HeapPtrChirho via resolve_string_arg_chirho (which forces thunks).
    fn resolve_value_to_string_chirho(
        &mut self,
        val_chirho: &ValueChirho,
    ) -> Result<String, EvalErrorChirho> {
        match val_chirho {
            ValueChirho::StringChirho(s_chirho) => Ok(s_chirho.clone()),
            ValueChirho::HeapPtrChirho(addr_chirho) => self.resolve_string_arg_chirho(*addr_chirho),
            other_chirho => Ok(format!("{}", other_chirho)),
        }
    }

    fn box_literal_chirho(&mut self, val_chirho: &ValueChirho) -> ClosureChirho {
        match val_chirho {
            ValueChirho::IntChirho(n_chirho) => ClosureChirho::con_chirho(
                self.lookup_con_tag_chirho("I#", 0),
                "I#",
                vec![ValueChirho::IntChirho(*n_chirho)],
            ),
            ValueChirho::FloatChirho(n_chirho) => ClosureChirho::con_chirho(
                self.lookup_con_tag_chirho("D#", 0),
                "D#",
                vec![ValueChirho::FloatChirho(*n_chirho)],
            ),
            ValueChirho::CharChirho(c_chirho) => ClosureChirho::con_chirho(
                self.lookup_con_tag_chirho("C#", 0),
                "C#",
                vec![ValueChirho::CharChirho(*c_chirho)],
            ),
            ValueChirho::BoolChirho(b_chirho) => {
                if *b_chirho {
                    ClosureChirho::con_chirho(
                        self.lookup_con_tag_chirho("True", 0),
                        "True",
                        vec![],
                    )
                } else {
                    ClosureChirho::con_chirho(
                        self.lookup_con_tag_chirho("False", 1),
                        "False",
                        vec![],
                    )
                }
            }
            ValueChirho::HeapPtrChirho(a_chirho) => ClosureChirho::con_chirho(
                DataConTagChirho(0),
                "Box",
                vec![ValueChirho::HeapPtrChirho(*a_chirho)],
            ),
            ValueChirho::StringChirho(s_chirho) => {
                // Treat String as [Char] for case dispatch: fully expand the
                // string into a cons list of Char on the heap so that all
                // tails are HeapPtrChirho and can be entered by the evaluator.
                let nil_tag_chirho = self.lookup_con_tag_chirho("[]", 0);
                let cons_tag_chirho = self.lookup_con_tag_chirho(":", 1);
                let nil_chirho = ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
                let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_chirho);
                for ch_chirho in s_chirho.chars().rev() {
                    let cons_chirho = ClosureChirho::con_chirho(
                        cons_tag_chirho, ":",
                        vec![
                            ValueChirho::CharChirho(ch_chirho),
                            ValueChirho::HeapPtrChirho(tail_addr_chirho),
                        ],
                    );
                    tail_addr_chirho = self.heap_chirho.alloc_chirho(cons_chirho);
                }
                // Return the outermost cons cell (not allocating it again)
                return self.heap_chirho.read_chirho(tail_addr_chirho).clone();
            }
        }
    }
}

/// Internal action after returning a value.
enum ReturnActionChirho {
    /// Evaluation complete — return this value.
    DoneChirho(ValueChirho),
    /// Continue evaluation at this code table index.
    ContinueChirho(u32),
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::value_chirho::CodePtrChirho;

    /// Helper: build a machine, run it from entry 0, return the result.
    fn run_code_chirho(
        code_chirho: Vec<CodeChirho>,
    ) -> Result<ValueChirho, EvalErrorChirho> {
        let mut machine_chirho = MachineChirho::new_chirho(code_chirho);
        machine_chirho.run_chirho(0)
    }

    #[test]
    fn literal_return_chirho() {
        // Code: just return Int 42
        let result_chirho = run_code_chirho(vec![CodeChirho::LitChirho(
            ValueChirho::IntChirho(42),
        )]);
        assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
    }

    #[test]
    fn primop_add_chirho() {
        // Code: 3 + 4
        let result_chirho = run_code_chirho(vec![CodeChirho::PrimChirho {
            op_chirho: PrimOpKindChirho::AddIntChirho,
            args_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(3)), ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(4))],
        }]);
        assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(7));
    }

    #[test]
    fn primop_mul_chirho() {
        let result_chirho = run_code_chirho(vec![CodeChirho::PrimChirho {
            op_chirho: PrimOpKindChirho::MulIntChirho,
            args_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(6)), ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(7))],
        }]);
        assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
    }

    #[test]
    fn con_return_chirho() {
        // Code: construct True and return it
        let result_chirho = run_code_chirho(vec![CodeChirho::ConAppChirho {
            tag_chirho: DataConTagChirho(0),
            name_chirho: "True".to_string(),
            fields_chirho: vec![],
        }]);
        // Returns a HeapPtr to the constructor
        match result_chirho.unwrap() {
            ValueChirho::HeapPtrChirho(_) => {} // OK
            other_chirho => panic!("expected HeapPtr, got {:?}", other_chirho),
        }
    }

    #[test]
    fn con_with_fields_chirho() {
        // Code: construct Just 42 — verify fields via machine
        let mut machine_chirho =
            MachineChirho::new_chirho(vec![CodeChirho::ConAppChirho {
                tag_chirho: DataConTagChirho(1),
                name_chirho: "Just".to_string(),
                fields_chirho: vec![ValueChirho::IntChirho(42)],
            }]);
        let result2_chirho = machine_chirho.run_chirho(0).unwrap();
        let addr2_chirho = match result2_chirho {
            ValueChirho::HeapPtrChirho(a_chirho) => a_chirho,
            _ => unreachable!(),
        };
        let closure_chirho = machine_chirho.heap_chirho.read_chirho(addr2_chirho);
        assert_eq!(closure_chirho.info_chirho.name_chirho, "Just");
        assert_eq!(
            closure_chirho.info_chirho.con_tag_chirho,
            DataConTagChirho(1)
        );
        assert_eq!(closure_chirho.payload_chirho[0], ValueChirho::IntChirho(42));
    }

    #[test]
    fn case_on_constructor_chirho() {
        // Simulate: case True of { True -> 1; False -> 0 }
        // Code table:
        //   0: ConApp True [] (builds True, returns to case frame)
        //   1: Lit 1 (True branch)
        //   2: Lit 0 (False branch)
        // But we need to set up the case frame first.
        // So: 0: Case scrutinee=addr, alts=[(0,1),(1,2)]
        // We need the scrutinee on the heap first.

        let mut machine_chirho = MachineChirho::new_chirho(vec![
            // 0: construct True on heap, then case on it
            CodeChirho::ConAppChirho {
                tag_chirho: DataConTagChirho(0),
                name_chirho: "True".to_string(),
                fields_chirho: vec![],
            },
            // 1: True branch
            CodeChirho::LitChirho(ValueChirho::IntChirho(1)),
            // 2: False branch
            CodeChirho::LitChirho(ValueChirho::IntChirho(0)),
        ]);

        // Pre-allocate True on heap so we can case on it
        let true_addr_chirho = machine_chirho.heap_chirho.alloc_chirho(
            ClosureChirho::con_chirho(DataConTagChirho(0), "True", vec![]),
        );

        // Replace code[0] with a Case
        machine_chirho.code_table_chirho[0] = CodeChirho::CaseChirho {
            scrutinee_chirho: ArgSourceChirho::StaticChirho(ValueChirho::HeapPtrChirho(true_addr_chirho)),
            alts_chirho: vec![(0, 1), (1, 2)],
            default_chirho: None,
        };

        let result_chirho = machine_chirho.run_chirho(0).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(1));
    }

    #[test]
    fn case_default_chirho() {
        let mut machine_chirho = MachineChirho::new_chirho(vec![
            CodeChirho::LitChirho(ValueChirho::IntChirho(0)), // placeholder
            CodeChirho::LitChirho(ValueChirho::IntChirho(99)), // default
        ]);

        // Allocate a constructor with tag 5 — no matching alt
        let addr_chirho = machine_chirho.heap_chirho.alloc_chirho(
            ClosureChirho::con_chirho(DataConTagChirho(5), "Unknown", vec![]),
        );

        machine_chirho.code_table_chirho[0] = CodeChirho::CaseChirho {
            scrutinee_chirho: ArgSourceChirho::StaticChirho(ValueChirho::HeapPtrChirho(addr_chirho)),
            alts_chirho: vec![(0, 0)], // only tag 0 handled
            default_chirho: Some(1),
        };

        let result_chirho = machine_chirho.run_chirho(0).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(99));
    }

    #[test]
    fn thunk_evaluation_chirho() {
        // Thunk that computes 3 + 4
        let mut machine_chirho = MachineChirho::new_chirho(vec![
            CodeChirho::LitChirho(ValueChirho::IntChirho(0)), // placeholder for entry
            CodeChirho::PrimChirho {
                // index 1: thunk body
                op_chirho: PrimOpKindChirho::AddIntChirho,
                args_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(3)), ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(4))],
            },
        ]);

        let thunk_addr_chirho = machine_chirho.heap_chirho.alloc_chirho(
            ClosureChirho::thunk_chirho(CodePtrChirho(1), "myThunk", vec![]),
        );

        // Entry: Enter the thunk
        machine_chirho.code_table_chirho[0] =
            CodeChirho::EnterChirho(thunk_addr_chirho);

        let result_chirho = machine_chirho.run_chirho(0).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(7));

        // Verify the thunk was updated (now an indirection)
        let updated_chirho = machine_chirho
            .heap_chirho
            .read_chirho(thunk_addr_chirho);
        assert_eq!(
            updated_chirho.info_chirho.tag_chirho,
            InfoTagChirho::IndChirho
        );
    }

    #[test]
    fn blackhole_detection_chirho() {
        // A thunk that tries to enter itself
        let mut machine_chirho = MachineChirho::new_chirho(vec![
            CodeChirho::LitChirho(ValueChirho::IntChirho(0)), // placeholder
            CodeChirho::LitChirho(ValueChirho::IntChirho(0)), // placeholder
        ]);

        let thunk_addr_chirho = machine_chirho.heap_chirho.alloc_chirho(
            ClosureChirho::thunk_chirho(CodePtrChirho(1), "loop", vec![]),
        );

        // code[0] = Enter thunk
        machine_chirho.code_table_chirho[0] =
            CodeChirho::EnterChirho(thunk_addr_chirho);
        // code[1] = Enter same thunk (self-reference)
        machine_chirho.code_table_chirho[1] =
            CodeChirho::EnterChirho(thunk_addr_chirho);

        let result_chirho = machine_chirho.run_chirho(0);
        assert!(matches!(
            result_chirho,
            Err(EvalErrorChirho::BlackholeChirho { .. })
        ));
    }

    #[test]
    fn step_limit_chirho() {
        // Infinite loop via step limit
        let mut machine_chirho = MachineChirho::new_chirho(vec![
            CodeChirho::LitChirho(ValueChirho::IntChirho(0)),
        ]);

        let thunk_addr_chirho = machine_chirho.heap_chirho.alloc_chirho(
            ClosureChirho::thunk_chirho(CodePtrChirho(0), "loop", vec![]),
        );

        machine_chirho.code_table_chirho[0] =
            CodeChirho::EnterChirho(thunk_addr_chirho);

        machine_chirho.step_limit_chirho = 5;
        let result_chirho = machine_chirho.run_chirho(0);
        // Should hit either blackhole or step limit
        assert!(result_chirho.is_err());
    }

    #[test]
    fn let_allocation_chirho() {
        // let x = I# 42 in x
        let mut machine_chirho = MachineChirho::new_chirho(vec![
            // 0: Let allocating one constructor, body at index 1
            CodeChirho::LetChirho {
                closures_chirho: vec![ClosureChirho::con_chirho(
                    DataConTagChirho(0),
                    "I#",
                    vec![ValueChirho::IntChirho(42)],
                )],
                body_chirho: 1,
            },
            // 1: Enter the just-allocated closure (addr 0)
            CodeChirho::EnterChirho(HeapAddrChirho(0)),
        ]);

        let result_chirho = machine_chirho.run_chirho(0).unwrap();
        match result_chirho {
            ValueChirho::HeapPtrChirho(addr_chirho) => {
                let c_chirho = machine_chirho.heap_chirho.read_chirho(addr_chirho);
                assert_eq!(c_chirho.info_chirho.name_chirho, "I#");
                assert_eq!(c_chirho.payload_chirho[0], ValueChirho::IntChirho(42));
            }
            other_chirho => panic!("expected HeapPtr, got {:?}", other_chirho),
        }
    }

    #[test]
    fn exact_application_chirho() {
        // f x = x + 1, f 41
        let mut machine_chirho = MachineChirho::new_chirho(vec![
            // 0: placeholder for App
            CodeChirho::LitChirho(ValueChirho::IntChirho(0)),
            // 1: function body: add arg + 1
            CodeChirho::PrimChirho {
                op_chirho: PrimOpKindChirho::AddIntChirho,
                args_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(41)), ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(1))],
            },
        ]);

        let fun_addr_chirho = machine_chirho.heap_chirho.alloc_chirho(
            ClosureChirho::fun_chirho(1, CodePtrChirho(1), "f", vec![]),
        );

        machine_chirho.code_table_chirho[0] = CodeChirho::AppChirho {
            fun_chirho: fun_addr_chirho,
            args_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(41))],
        };

        let result_chirho = machine_chirho.run_chirho(0).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(42));
    }

    #[test]
    fn under_application_builds_pap_chirho() {
        // f x y = x + y, f 10 (partial application)
        let mut machine_chirho = MachineChirho::new_chirho(vec![
            CodeChirho::LitChirho(ValueChirho::IntChirho(0)), // placeholder
            CodeChirho::PrimChirho {
                // arity 2 body
                op_chirho: PrimOpKindChirho::AddIntChirho,
                args_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(10)), ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(32))],
            },
        ]);

        let fun_addr_chirho = machine_chirho.heap_chirho.alloc_chirho(
            ClosureChirho::fun_chirho(2, CodePtrChirho(1), "add", vec![]),
        );

        // Apply with only 1 arg (arity is 2)
        machine_chirho.code_table_chirho[0] = CodeChirho::AppChirho {
            fun_chirho: fun_addr_chirho,
            args_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(10))],
        };

        let result_chirho = machine_chirho.run_chirho(0).unwrap();
        match result_chirho {
            ValueChirho::HeapPtrChirho(pap_addr_chirho) => {
                let c_chirho = machine_chirho.heap_chirho.read_chirho(pap_addr_chirho);
                assert_eq!(c_chirho.info_chirho.tag_chirho, InfoTagChirho::PapChirho);
                assert_eq!(c_chirho.info_chirho.arity_chirho, 1); // needs 1 more
            }
            other_chirho => panic!("expected PAP, got {:?}", other_chirho),
        }
    }

    #[test]
    fn div_by_zero_chirho() {
        let result_chirho = run_code_chirho(vec![CodeChirho::PrimChirho {
            op_chirho: PrimOpKindChirho::DivIntChirho,
            args_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(42)), ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(0))],
        }]);
        assert!(matches!(
            result_chirho,
            Err(EvalErrorChirho::PrimFailChirho(
                PrimErrorChirho::DivByZeroChirho
            ))
        ));
    }

    #[test]
    fn float_arithmetic_chirho() {
        let result_chirho = run_code_chirho(vec![CodeChirho::PrimChirho {
            op_chirho: PrimOpKindChirho::MulFloatChirho,
            args_chirho: vec![
                ArgSourceChirho::StaticChirho(ValueChirho::FloatChirho(3.0)),
                ArgSourceChirho::StaticChirho(ValueChirho::FloatChirho(14.0)),
            ],
        }]);
        assert_eq!(result_chirho.unwrap(), ValueChirho::FloatChirho(42.0));
    }
}
