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
    GcConfigChirho, GcStateChirho, GcStatsChirho, extract_roots_from_stack_chirho,
    extract_roots_from_values_chirho,
};
use crate::heap_chirho::HeapChirho;
use crate::prim_chirho::{PrimErrorChirho, apply_prim_binop_chirho};
use crate::stack_chirho::{FrameChirho, PrimOpKindChirho, StackChirho};
use crate::value_chirho::{
    ClosureChirho, CodePtrChirho, DataConTagChirho, HeapAddrChirho, InfoTagChirho, ValueChirho,
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

/// One closure in a recursive group that must be allocated for each
/// invocation because it captures values from the current argument registers.
#[derive(Debug, Clone)]
pub struct RecBindingSpecChirho {
    /// Function arity, or `None` when this binding is a thunk.
    pub arity_chirho: Option<u16>,
    /// Code entry point for this binding.
    pub code_ptr_chirho: u32,
    /// Name for diagnostics and heap inspection.
    pub name_chirho: String,
    /// Outer values captured before the recursive group changes arg registers.
    pub captures_chirho: Vec<ArgSourceChirho>,
    /// Arg register slot that receives this invocation's fresh closure address.
    pub dest_reg_chirho: usize,
}

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
    ForceChirho { thunk_chirho: ClosureChirho },

    /// Read argument register `index_chirho` and return it as a literal.
    ArgChirho { index_chirho: usize },

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
    },

    /// Allocate a recursive closure group atomically for the current
    /// invocation. Each closure payload contains its outer captures followed
    /// by fresh pointers to every member of this group in binding order.
    StoreAllocRecGroupChirho {
        bindings_chirho: Vec<RecBindingSpecChirho>,
        body_chirho: u32,
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
    /// TVar storage: transactional variables indexed by ID.
    pub tvars_chirho: HashMap<u64, ValueChirho>,
    /// Next TVar ID counter.
    pub next_tvar_chirho: u64,
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
            tvars_chirho: HashMap::new(),
            next_tvar_chirho: 0,
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

    fn collect_roots_chirho(&self) -> Vec<HeapAddrChirho> {
        let mut roots_chirho = extract_roots_from_stack_chirho(self.stack_chirho.frames_chirho());
        roots_chirho.extend(extract_roots_from_values_chirho(&self.arg_regs_chirho));
        roots_chirho.extend(extract_roots_from_values_chirho(
            &self.iorefs_chirho.values().cloned().collect::<Vec<_>>(),
        ));
        roots_chirho.extend(extract_roots_from_values_chirho(
            &self.tvars_chirho.values().cloned().collect::<Vec<_>>(),
        ));
        roots_chirho
    }

    /// Run garbage collection if the allocation threshold has been reached.
    /// Extracts roots from the stack and argument registers.
    fn maybe_gc_chirho(&mut self) {
        if !self.gc_state_chirho.notify_alloc_chirho() {
            return;
        }
        let roots_chirho = self.collect_roots_chirho();
        let stats_chirho = self
            .gc_state_chirho
            .collect_chirho(&mut self.heap_chirho, &roots_chirho);
        self.last_gc_stats_chirho = Some(stats_chirho);
    }

    /// Build a `Left msg` Either constructor on the heap and return it as a
    /// `ValueChirho::HeapPtrChirho`.  Used by the `TryFrameChirho` error path.
    fn make_either_left_chirho(&mut self, msg_chirho: String) -> ValueChirho {
        let tag_chirho = self.lookup_con_tag_chirho("Left", 0);
        let inner_chirho = ValueChirho::StringChirho(msg_chirho);
        let closure_chirho = ClosureChirho::con_chirho(tag_chirho, "Left", vec![inner_chirho]);
        let addr_chirho = self.heap_chirho.alloc_chirho(closure_chirho);
        ValueChirho::HeapPtrChirho(addr_chirho)
    }

    /// Build a `Right val` Either constructor on the heap and return it as a
    /// `ValueChirho::HeapPtrChirho`.  Used by the `TryFrameChirho` success path.
    fn make_either_right_chirho(&mut self, val_chirho: ValueChirho) -> ValueChirho {
        let tag_chirho = self.lookup_con_tag_chirho("Right", 1);
        let closure_chirho = ClosureChirho::con_chirho(tag_chirho, "Right", vec![val_chirho]);
        let addr_chirho = self.heap_chirho.alloc_chirho(closure_chirho);
        ValueChirho::HeapPtrChirho(addr_chirho)
    }

    /// Emit a `CodeChirho::LitChirho` entry into the code table and return its index.
    /// Used at runtime to produce a synthetic "return this value" instruction
    /// when the `TryFrameChirho` must inject a `Left`/`Right` value into the
    /// continuation stack.
    fn emit_lit_code_chirho(&mut self, val_chirho: ValueChirho) -> u32 {
        let idx_chirho = self.code_table_chirho.len() as u32;
        self.code_table_chirho
            .push(CodeChirho::LitChirho(val_chirho));
        idx_chirho
    }

    /// Attempt to handle a `RuntimeErrorChirho` by unwinding the stack to
    /// a `CatchChirho` or `TryFrameChirho` frame.
    ///
    /// - `CatchChirho`: sets up the machine to invoke the handler closure and
    ///   returns `Ok(pc)` for the handler entry.
    /// - `TryFrameChirho`: wraps the error message in `Left` and emits a
    ///   synthetic `LitChirho` code entry for the `Left` heap pointer so that
    ///   the main evaluation loop continues normally with the `Either` value.
    /// - Otherwise re-returns the original error.
    fn try_catch_runtime_error_chirho(
        &mut self,
        msg_chirho: String,
    ) -> Result<u32, EvalErrorChirho> {
        match self.stack_chirho.unwind_to_catch_chirho() {
            Some(FrameChirho::CatchChirho {
                handler_addr_chirho,
                saved_arg_regs_chirho,
            }) => {
                // Restore saved arg registers
                self.arg_regs_chirho = saved_arg_regs_chirho;
                // Push an Apply frame to apply the handler to the error message
                let msg_val_chirho = ValueChirho::StringChirho(msg_chirho);
                self.stack_chirho.push_chirho(FrameChirho::ApplyChirho {
                    args_chirho: vec![msg_val_chirho],
                });
                // Enter the handler closure
                Ok(self.emit_enter_chirho(handler_addr_chirho))
            }
            Some(FrameChirho::TryFrameChirho {
                saved_arg_regs_chirho,
            }) => {
                // try# error path: wrap error message in Left and inject it
                // back into the evaluation loop as a synthetic literal code.
                self.arg_regs_chirho = saved_arg_regs_chirho;
                let left_val_chirho = self.make_either_left_chirho(msg_chirho);
                // Emit a synthetic LitChirho code entry so the main loop
                // picks up the Left value and delivers it to any outer
                // continuation frames (e.g. a subsequent >>= handler).
                Ok(self.emit_lit_code_chirho(left_val_chirho))
            }
            _ => Err(EvalErrorChirho::RuntimeErrorChirho(msg_chirho)),
        }
    }

    /// Check if an eval error is a RuntimeErrorChirho and attempt to catch
    /// it with a CatchChirho frame.  For non-runtime errors (type errors,
    /// blackholes, etc.) the error is always propagated.
    fn maybe_catch_error_chirho(
        &mut self,
        err_chirho: EvalErrorChirho,
    ) -> Result<u32, EvalErrorChirho> {
        match err_chirho {
            EvalErrorChirho::RuntimeErrorChirho(msg_chirho) => {
                self.try_catch_runtime_error_chirho(msg_chirho)
            }
            other_chirho => Err(other_chirho),
        }
    }

    /// Run the machine starting from the given code table index.
    /// Returns the final value in WHNF.
    ///
    /// When a `RuntimeErrorChirho` occurs anywhere in evaluation, the
    /// stack is searched for a `CatchChirho` frame.  If found, the
    /// handler is invoked with the error message string instead of
    /// propagating the error.
    pub fn run_chirho(&mut self, entry_chirho: u32) -> Result<ValueChirho, EvalErrorChirho> {
        // Helper macro: dispatch on a Result<ReturnActionChirho, _>.
        // On success, either return Done or set pc to Continue.
        // On RuntimeErrorChirho, try to catch with a CatchChirho frame.
        macro_rules! dispatch_action_chirho {
            ($self:ident, $pc:ident, $result:expr) => {
                match $result {
                    Ok(ReturnActionChirho::DoneChirho(v_chirho)) => {
                        return Ok(v_chirho);
                    }
                    Ok(ReturnActionChirho::ContinueChirho(next_chirho)) => {
                        $pc = next_chirho;
                    }
                    Err(e_chirho) => match $self.maybe_catch_error_chirho(e_chirho) {
                        Ok(next_chirho) => {
                            $pc = next_chirho;
                        }
                        Err(e2_chirho) => return Err(e2_chirho),
                    },
                }
            };
        }

        // Helper macro: dispatch on a Result<u32, _> (for dispatch_case_lit).
        macro_rules! dispatch_u32_chirho {
            ($self:ident, $pc:ident, $result:expr) => {
                match $result {
                    Ok(next_chirho) => {
                        $pc = next_chirho;
                    }
                    Err(e_chirho) => match $self.maybe_catch_error_chirho(e_chirho) {
                        Ok(next_chirho) => {
                            $pc = next_chirho;
                        }
                        Err(e2_chirho) => return Err(e2_chirho),
                    },
                }
            };
        }

        let mut pc_chirho = entry_chirho;

        loop {
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
                            let r_chirho = self.return_con_chirho(addr_chirho);
                            dispatch_action_chirho!(self, pc_chirho, r_chirho);
                        }
                        InfoTagChirho::FunChirho => {
                            // Function — check for Apply frame
                            let r_chirho = self.enter_fun_chirho(addr_chirho, &closure_chirho);
                            dispatch_action_chirho!(self, pc_chirho, r_chirho);
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
                                self.arg_regs_chirho = closure_chirho.payload_chirho.clone();
                            }
                            pc_chirho = closure_chirho.info_chirho.entry_chirho.0;
                        }
                        InfoTagChirho::PapChirho => {
                            // PAP — like function but with pre-applied args
                            let r_chirho = self.enter_pap_chirho(addr_chirho, &closure_chirho);
                            dispatch_action_chirho!(self, pc_chirho, r_chirho);
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
                    let fun_val_chirho = if fun_arg_index_chirho < self.arg_regs_chirho.len() {
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
                    let resolved_scrut_chirho = self.resolve_arg_source_chirho(&scrutinee_chirho);
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
                            let r_chirho = self.return_lit_chirho(other_chirho);
                            dispatch_action_chirho!(self, pc_chirho, r_chirho);
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
                    let r_chirho = self.return_con_chirho(addr_chirho);
                    dispatch_action_chirho!(self, pc_chirho, r_chirho);
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
                    let r_chirho = self.return_con_chirho(addr_chirho);
                    dispatch_action_chirho!(self, pc_chirho, r_chirho);
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
                                saved_arg_regs_chirho: self.arg_regs_chirho.clone(),
                            });
                            pc_chirho = self.emit_enter_chirho(addr_chirho);
                        }
                        _ => {
                            // Already a value — match directly.
                            let r_chirho = self.dispatch_case_lit_chirho(
                                &resolved_chirho,
                                &alts_chirho,
                                default_chirho,
                            );
                            dispatch_u32_chirho!(self, pc_chirho, r_chirho);
                        }
                    }
                }

                // ── Literal return ──────────────────────────────────
                CodeChirho::LitChirho(val_chirho) => {
                    let r_chirho = self.return_lit_chirho(val_chirho);
                    dispatch_action_chirho!(self, pc_chirho, r_chirho);
                }

                // ── Primitive operation ──────────────────────────────
                CodeChirho::PrimChirho {
                    op_chirho,
                    args_chirho,
                } => {
                    // Resolve ArgSource → ValueChirho (fresh thunks each time).
                    let resolved_chirho = self.resolve_args_chirho(&args_chirho);

                    // CatchChirho primop: push catch frame and evaluate body.
                    // The catch primop returns the body value from eval_prim,
                    // and then we need to continue evaluating it (it may be a
                    // HeapPtr thunk or a literal). Handle the result via the
                    // normal return path.
                    if matches!(
                        op_chirho,
                        PrimOpKindChirho::CatchChirho | PrimOpKindChirho::TryChirho
                    ) {
                        let prim_result_chirho = self.eval_prim_chirho(op_chirho, &resolved_chirho);
                        match prim_result_chirho {
                            Ok(ValueChirho::HeapPtrChirho(addr_chirho)) => {
                                pc_chirho = self.emit_enter_chirho(addr_chirho);
                                continue;
                            }
                            Ok(val_chirho) => {
                                let r_chirho = self.return_lit_chirho(val_chirho);
                                dispatch_action_chirho!(self, pc_chirho, r_chirho);
                                continue;
                            }
                            Err(e_chirho) => {
                                match self.maybe_catch_error_chirho(e_chirho) {
                                    Ok(next_chirho) => {
                                        pc_chirho = next_chirho;
                                    }
                                    Err(e2_chirho) => return Err(e2_chirho),
                                }
                                continue;
                            }
                        }
                    }

                    // These primops consume lazy values. `return`/`pure` must preserve
                    // its payload thunk; forcing happens only at an actual strict use or
                    // at the interpreter's observable-result boundary.
                    if matches!(
                        op_chirho,
                        PrimOpKindChirho::InteractChirho | PrimOpKindChirho::ReturnIOChirho
                    ) {
                        let result_chirho = self.eval_prim_chirho(op_chirho, &resolved_chirho);
                        match result_chirho {
                            Ok(val_chirho) => {
                                let r_chirho = self.return_lit_chirho(val_chirho);
                                dispatch_action_chirho!(self, pc_chirho, r_chirho);
                                continue;
                            }
                            Err(e_chirho) => {
                                match self.maybe_catch_error_chirho(e_chirho) {
                                    Ok(next_chirho) => {
                                        pc_chirho = next_chirho;
                                    }
                                    Err(e2_chirho) => return Err(e2_chirho),
                                }
                                continue;
                            }
                        }
                    }

                    // Check if any arg needs forcing (is a HeapPtr thunk).
                    let needs_forcing_chirho = resolved_chirho
                        .iter()
                        .any(|a_chirho| matches!(a_chirho, ValueChirho::HeapPtrChirho(_)));

                    if !needs_forcing_chirho {
                        // Fast path: all args already unboxed.
                        let result_chirho = self.eval_prim_chirho(op_chirho, &resolved_chirho);
                        match result_chirho {
                            Ok(val_chirho) => {
                                let r_chirho = self.return_lit_chirho(val_chirho);
                                dispatch_action_chirho!(self, pc_chirho, r_chirho);
                            }
                            Err(e_chirho) => match self.maybe_catch_error_chirho(e_chirho) {
                                Ok(next_chirho) => {
                                    pc_chirho = next_chirho;
                                }
                                Err(e2_chirho) => return Err(e2_chirho),
                            },
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
                                let r_chirho = self.return_lit_chirho(other_chirho);
                                dispatch_action_chirho!(self, pc_chirho, r_chirho);
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
                    let r_chirho = self.return_lit_chirho(val_chirho);
                    dispatch_action_chirho!(self, pc_chirho, r_chirho);
                }

                // ── Allocate function with captures and store ──────
                CodeChirho::StoreAllocFunChirho {
                    arity_chirho,
                    code_ptr_chirho,
                    name_chirho,
                    captures_chirho,
                    dest_reg_chirho,
                    body_chirho,
                } => {
                    let payload_chirho = self.resolve_args_chirho(&captures_chirho);
                    let closure_chirho = ClosureChirho::fun_chirho(
                        arity_chirho,
                        CodePtrChirho(code_ptr_chirho),
                        &name_chirho,
                        payload_chirho,
                    );
                    let addr_chirho = self.heap_chirho.alloc_chirho(closure_chirho);
                    self.maybe_gc_chirho();
                    // Store in dest arg register and continue to body
                    if self.arg_regs_chirho.len() <= dest_reg_chirho {
                        self.arg_regs_chirho
                            .resize(dest_reg_chirho + 1, ValueChirho::IntChirho(0));
                    }
                    self.arg_regs_chirho[dest_reg_chirho] = ValueChirho::HeapPtrChirho(addr_chirho);
                    pc_chirho = body_chirho;
                }

                // ── Allocate thunk with captures and store ────────
                CodeChirho::StoreAllocThunkChirho {
                    code_ptr_chirho,
                    name_chirho,
                    captures_chirho,
                    dest_reg_chirho,
                    body_chirho,
                } => {
                    let payload_chirho = self.resolve_args_chirho(&captures_chirho);
                    let closure_chirho = ClosureChirho::thunk_chirho(
                        CodePtrChirho(code_ptr_chirho),
                        &name_chirho,
                        payload_chirho,
                    );
                    let addr_chirho = self.heap_chirho.alloc_chirho(closure_chirho);
                    self.maybe_gc_chirho();
                    // Store in dest arg register and continue to body
                    if self.arg_regs_chirho.len() <= dest_reg_chirho {
                        self.arg_regs_chirho
                            .resize(dest_reg_chirho + 1, ValueChirho::IntChirho(0));
                    }
                    self.arg_regs_chirho[dest_reg_chirho] = ValueChirho::HeapPtrChirho(addr_chirho);
                    pc_chirho = body_chirho;
                }

                // ── Allocate one fresh recursive group per invocation ──
                CodeChirho::StoreAllocRecGroupChirho {
                    bindings_chirho,
                    body_chirho,
                } => {
                    // Resolve every outer capture before destination registers
                    // are overwritten by this invocation's fresh group.
                    let captured_values_chirho: Vec<Vec<ValueChirho>> = bindings_chirho
                        .iter()
                        .map(|binding_chirho| {
                            self.resolve_args_chirho(&binding_chirho.captures_chirho)
                        })
                        .collect();

                    let group_addrs_chirho: Vec<HeapAddrChirho> = bindings_chirho
                        .iter()
                        .map(|_| {
                            self.heap_chirho
                                .alloc_chirho(ClosureChirho::blackhole_chirho())
                        })
                        .collect();

                    for ((binding_chirho, mut payload_chirho), addr_chirho) in bindings_chirho
                        .iter()
                        .zip(captured_values_chirho)
                        .zip(group_addrs_chirho.iter().copied())
                    {
                        payload_chirho.extend(
                            group_addrs_chirho
                                .iter()
                                .copied()
                                .map(ValueChirho::HeapPtrChirho),
                        );
                        let closure_chirho = match binding_chirho.arity_chirho {
                            Some(arity_chirho) => ClosureChirho::fun_chirho(
                                arity_chirho,
                                CodePtrChirho(binding_chirho.code_ptr_chirho),
                                &binding_chirho.name_chirho,
                                payload_chirho,
                            ),
                            None => ClosureChirho::thunk_chirho(
                                CodePtrChirho(binding_chirho.code_ptr_chirho),
                                &binding_chirho.name_chirho,
                                payload_chirho,
                            ),
                        };
                        *self.heap_chirho.read_mut_chirho(addr_chirho) = closure_chirho;
                    }

                    for (binding_chirho, addr_chirho) in bindings_chirho
                        .iter()
                        .zip(group_addrs_chirho.iter().copied())
                    {
                        if self.arg_regs_chirho.len() <= binding_chirho.dest_reg_chirho {
                            self.arg_regs_chirho.resize(
                                binding_chirho.dest_reg_chirho + 1,
                                ValueChirho::IntChirho(0),
                            );
                        }
                        self.arg_regs_chirho[binding_chirho.dest_reg_chirho] =
                            ValueChirho::HeapPtrChirho(addr_chirho);
                    }

                    // The complete group is rooted before any allocation can
                    // trigger collection; one notification is retained per
                    // heap allocation.
                    for _binding_chirho in &bindings_chirho {
                        self.maybe_gc_chirho();
                    }
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
                            let r_chirho = self.return_lit_chirho(val_chirho);
                            dispatch_action_chirho!(self, pc_chirho, r_chirho);
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
                    return Ok(ReturnActionChirho::DoneChirho(ValueChirho::HeapPtrChirho(
                        addr_chirho,
                    )));
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
                    // Keep a copy of original saved regs for default alt
                    // (default alt has no binders, so no field prepending).
                    let saved_arg_regs_chirho_for_default = saved_arg_regs_chirho.clone();
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
                    // Try default — restore saved arg_regs WITHOUT prepending
                    // constructor fields, since the default alt has no binders
                    // that would consume the fields. The STG lowerer doesn't
                    // shift arg_param_indices for default alts.
                    if let Some(def_chirho) = default_entry_chirho {
                        self.arg_regs_chirho = saved_arg_regs_chirho_for_default;
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
                    saved_arg_regs_chirho,
                }) => {
                    // Restore arg_regs from the frame so alt RHS code can
                    // access enclosing function parameters correctly.
                    self.arg_regs_chirho = saved_arg_regs_chirho;
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
                    let closure_chirho_peek = self.heap_chirho.read_chirho(addr_chirho).clone();
                    if closure_chirho_peek.info_chirho.tag_chirho == InfoTagChirho::PapChirho {
                        // Re-push the Apply frame and enter the PAP
                        self.stack_chirho
                            .push_chirho(FrameChirho::ApplyChirho { args_chirho });
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
                Some(FrameChirho::CatchChirho { .. }) => {
                    // Exception handler frame — body succeeded, so the
                    // catch frame is simply discarded and the result
                    // passes through.
                    continue;
                }
                Some(FrameChirho::TryFrameChirho { .. }) => {
                    // try# frame — body succeeded.  Wrap the constructor
                    // heap pointer in a `Right` constructor so the caller
                    // receives `IO (Either String a)`.
                    let right_val_chirho =
                        self.make_either_right_chirho(ValueChirho::HeapPtrChirho(addr_chirho));
                    // Emit a synthetic LitChirho entry and continue from there so
                    // any outer continuation frames are satisfied.
                    let idx_chirho = self.emit_lit_code_chirho(right_val_chirho);
                    return Ok(ReturnActionChirho::ContinueChirho(idx_chirho));
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
                            | PrimOpKindChirho::ShowEitherChirho
                            | PrimOpKindChirho::ShowOrderingChirho
                            | PrimOpKindChirho::ShowListChirho
                    );
                    let closure_chirho = self.heap_chirho.read_chirho(addr_chirho).clone();
                    let unboxed_chirho = if keep_heap_ptr_chirho {
                        ValueChirho::HeapPtrChirho(addr_chirho)
                    } else if closure_chirho.payload_chirho.len() == 1 {
                        match &closure_chirho.payload_chirho[0] {
                            ValueChirho::IntChirho(n_chirho) => ValueChirho::IntChirho(*n_chirho),
                            ValueChirho::FloatChirho(n_chirho) => {
                                ValueChirho::FloatChirho(*n_chirho)
                            }
                            ValueChirho::CharChirho(c_chirho) => ValueChirho::CharChirho(*c_chirho),
                            ValueChirho::BoolChirho(b_chirho) => ValueChirho::BoolChirho(*b_chirho),
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
                                let enter_idx_chirho = self.emit_enter_chirho(a_chirho);
                                return Ok(ReturnActionChirho::ContinueChirho(enter_idx_chirho));
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
                                let enter_idx_chirho = self.emit_enter_chirho(addr_chirho);
                                return Ok(ReturnActionChirho::ContinueChirho(enter_idx_chirho));
                            }
                            other_chirho => {
                                // Already unboxed — recursively return it
                                return self.return_lit_chirho(other_chirho);
                            }
                        }
                    }
                }
                Some(FrameChirho::CatchChirho { .. }) => {
                    // Exception handler frame — body succeeded, so the
                    // catch frame is simply discarded and the result
                    // passes through.
                    continue;
                }
                Some(FrameChirho::TryFrameChirho { .. }) => {
                    // try# frame — body succeeded with a literal value.
                    // Wrap in Right so the caller receives Either String a.
                    let right_val_chirho = self.make_either_right_chirho(val_chirho.clone());
                    let idx_chirho = self.emit_lit_code_chirho(right_val_chirho);
                    return Ok(ReturnActionChirho::ContinueChirho(idx_chirho));
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
                    saved_arg_regs_chirho,
                }) => {
                    // Restore arg_regs from the frame so alt RHS code can
                    // access enclosing function parameters correctly.
                    self.arg_regs_chirho = saved_arg_regs_chirho;
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
                        message_chirho: format!("cannot apply arguments to literal {}", val_chirho),
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
                    return Ok(ReturnActionChirho::DoneChirho(ValueChirho::HeapPtrChirho(
                        addr_chirho,
                    )));
                }
                Some(FrameChirho::UpdateChirho { thunk_addr_chirho }) => {
                    // Update thunk to an indirection to the heap object.
                    self.heap_chirho
                        .update_to_ind_chirho(thunk_addr_chirho, addr_chirho);
                    continue;
                }
                Some(FrameChirho::CatchChirho { .. }) => {
                    // Exception handler frame — body succeeded, so the
                    // catch frame is simply discarded and the result
                    // passes through.
                    continue;
                }
                Some(FrameChirho::TryFrameChirho { .. }) => {
                    // try# frame — body succeeded with a heap pointer.
                    // Wrap in Right so the caller receives Either String a.
                    let right_val_chirho =
                        self.make_either_right_chirho(ValueChirho::HeapPtrChirho(addr_chirho));
                    let idx_chirho = self.emit_lit_code_chirho(right_val_chirho);
                    return Ok(ReturnActionChirho::ContinueChirho(idx_chirho));
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
                                let enter_idx_chirho = self.emit_enter_chirho(a_chirho);
                                return Ok(ReturnActionChirho::ContinueChirho(enter_idx_chirho));
                            }
                            other_chirho => {
                                return self.return_lit_chirho(other_chirho);
                            }
                        }
                    }
                }
                Some(FrameChirho::ApplyChirho { args_chirho }) => {
                    // The heap object is being applied to arguments — enter it.
                    self.stack_chirho
                        .push_chirho(FrameChirho::ApplyChirho { args_chirho });
                    let enter_idx_chirho = self.emit_enter_chirho(addr_chirho);
                    return Ok(ReturnActionChirho::ContinueChirho(enter_idx_chirho));
                }
                Some(FrameChirho::CaseLitChirho {
                    alt_entries_chirho,
                    default_entry_chirho,
                    saved_arg_regs_chirho,
                }) => {
                    // Restore arg_regs from the frame.
                    self.arg_regs_chirho = saved_arg_regs_chirho;
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
                message_chirho: format!("non-exhaustive literal case: {:?}", val_chirho),
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
                        let (taken_chirho, rest_chirho) = args_chirho.split_at(arity_chirho);
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
                        let remaining_chirho = (arity_chirho - n_args_chirho) as u16;
                        let pap_chirho =
                            ClosureChirho::pap_chirho(remaining_chirho, addr_chirho, args_chirho);
                        let pap_addr_chirho = self.heap_chirho.alloc_chirho(pap_chirho);
                        return self.return_heap_ptr_chirho(pap_addr_chirho);
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
                    // Push it back and route the function value through
                    // return_heap_ptr_chirho so that frames like PrimOpChirho
                    // receive the HeapPtr as a (forced-to-WHNF) argument
                    // instead of terminating evaluation prematurely.
                    self.stack_chirho.push_chirho(other_chirho);
                    return self.return_heap_ptr_chirho(addr_chirho);
                }
                None => {
                    // Stack empty — function is the final result
                    return Ok(ReturnActionChirho::DoneChirho(ValueChirho::HeapPtrChirho(
                        addr_chirho,
                    )));
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
                                message_chirho: "PAP payload[0] is not a heap pointer".to_string(),
                            });
                        }
                    };
                    let fun_addr_chirho = self.heap_chirho.follow_ind_chirho(fun_addr_chirho);
                    let fun_closure_chirho = self.heap_chirho.read_chirho(fun_addr_chirho).clone();
                    if fun_closure_chirho.info_chirho.tag_chirho != InfoTagChirho::FunChirho {
                        return Err(EvalErrorChirho::TypeErrorChirho {
                            message_chirho: format!(
                                "PAP target is not a function: {:?}",
                                fun_closure_chirho.info_chirho.tag_chirho
                            ),
                        });
                    }

                    if n_args_chirho > remaining_chirho {
                        // Over-application: push extra args
                        let (_, rest_chirho) = args_chirho.split_at(remaining_chirho);
                        self.stack_chirho.push_chirho(FrameChirho::ApplyChirho {
                            args_chirho: rest_chirho.to_vec(),
                        });
                    }

                    // Load PAP's pre-applied args + new args into registers
                    let mut all_regs_chirho: Vec<ValueChirho> =
                        fun_closure_chirho.payload_chirho.clone();
                    all_regs_chirho.extend_from_slice(&closure_chirho.payload_chirho[1..]);
                    let take_chirho = remaining_chirho.min(n_args_chirho);
                    all_regs_chirho.extend_from_slice(&args_chirho[..take_chirho]);
                    self.arg_regs_chirho = all_regs_chirho;

                    // Enter the original function
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
                                message_chirho: "PAP payload[0] is not a heap pointer".to_string(),
                            });
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
                    self.return_heap_ptr_chirho(pap_addr_chirho)
                }
            }
            Some(FrameChirho::UpdateChirho { thunk_addr_chirho }) => {
                self.heap_chirho
                    .update_to_ind_chirho(thunk_addr_chirho, addr_chirho);
                self.return_heap_ptr_chirho(addr_chirho)
            }
            Some(other_chirho) => {
                self.stack_chirho.push_chirho(other_chirho);
                self.return_heap_ptr_chirho(addr_chirho)
            }
            None => Ok(ReturnActionChirho::DoneChirho(ValueChirho::HeapPtrChirho(
                addr_chirho,
            ))),
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
                return Ok(args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0)));
            }
            PrimOpKindChirho::EvaluateChirho => {
                // evaluate a = return a (forces a to WHNF, wraps in IO)
                // Since our primop args are already forced, just return the value.
                return Ok(args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0)));
            }
            PrimOpKindChirho::ForceChirho => {
                // force a = deepseq a a (fully evaluates and returns a)
                // Since our values are already fully evaluated for primitives,
                // this is equivalent to identity.
                return Ok(args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0)));
            }
            PrimOpKindChirho::IdChirho => {
                // id# a = a (identity function — returns first arg unchanged)
                return Ok(args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0)));
            }
            PrimOpKindChirho::CatchChirho => {
                // catch# body handler
                // body is the IO action, handler is the exception handler.
                // We push a CatchChirho frame and then evaluate the body.
                // The body's result will pass through the CatchChirho frame
                // transparently on success. On error, run_chirho unwinds
                // to the CatchChirho frame and invokes the handler.
                let body_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let handler_val_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                // Allocate the handler on the heap if it's not already there
                let handler_addr_chirho = match handler_val_chirho {
                    ValueChirho::HeapPtrChirho(a_chirho) => a_chirho,
                    _ => {
                        // Wrap the handler value in a closure
                        let closure_chirho = ClosureChirho::con_chirho(
                            DataConTagChirho(0),
                            "$handler",
                            vec![handler_val_chirho],
                        );
                        self.heap_chirho.alloc_chirho(closure_chirho)
                    }
                };
                self.stack_chirho.push_chirho(FrameChirho::CatchChirho {
                    handler_addr_chirho,
                    saved_arg_regs_chirho: self.arg_regs_chirho.clone(),
                });
                // Return the body value so the caller enters/evaluates it
                return Ok(body_val_chirho);
            }
            PrimOpKindChirho::ThrowChirho => {
                // throw# msg — exactly like error#
                let msg_chirho = match args_chirho.first() {
                    Some(ValueChirho::StringChirho(s_chirho)) => s_chirho.clone(),
                    Some(ValueChirho::HeapPtrChirho(a_chirho)) => {
                        self.resolve_string_arg_chirho(*a_chirho)?
                    }
                    _ => "throw".to_string(),
                };
                return Err(EvalErrorChirho::RuntimeErrorChirho(msg_chirho));
            }
            PrimOpKindChirho::TryChirho => {
                // try# body — wraps body execution in a TryFrameChirho.
                // On success, return_con/return_lit/return_heap_ptr will
                // wrap the result in `Right`.  On RuntimeErrorChirho,
                // try_catch_runtime_error_chirho wraps the message in `Left`.
                // Both paths produce IO (Either String a).
                let body_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                self.stack_chirho.push_chirho(FrameChirho::TryFrameChirho {
                    saved_arg_regs_chirho: self.arg_regs_chirho.clone(),
                });
                return Ok(body_val_chirho);
            }
            PrimOpKindChirho::BracketChirho => {
                // bracket# acquire release body
                // :: IO a -> (a -> IO b) -> (a -> IO c) -> IO c
                //
                // Semantics:
                //   1. Run acquire to get resource r.
                //   2. Run (body r), catching any exception.
                //   3. Run (release r) regardless.
                //   4. Re-throw on exception, otherwise return body result.
                //
                // We run sub-computations by saving/restoring stack+regs and
                // calling run_chirho inline (same technique as force_addr_to_whnf_chirho).
                let acquire_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let release_val_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let body_val_chirho = args_chirho
                    .get(2)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));

                // Helper: run an IO value (HeapPtr or literal) in a sub-context.
                // Returns Ok(value) or Err(RuntimeErrorChirho message).
                let run_io_val_chirho = |machine_chirho: &mut MachineChirho,
                                         io_val_chirho: ValueChirho|
                 -> Result<ValueChirho, String> {
                    match io_val_chirho {
                        ValueChirho::HeapPtrChirho(addr_chirho) => {
                            let saved_stack_chirho = std::mem::replace(
                                &mut machine_chirho.stack_chirho,
                                StackChirho::new_chirho(),
                            );
                            let saved_regs_chirho =
                                std::mem::take(&mut machine_chirho.arg_regs_chirho);
                            let entry_chirho = machine_chirho.code_table_chirho.len() as u32;
                            machine_chirho
                                .code_table_chirho
                                .push(CodeChirho::EnterChirho(addr_chirho));
                            let result_chirho = machine_chirho.run_chirho(entry_chirho);
                            machine_chirho.stack_chirho = saved_stack_chirho;
                            machine_chirho.arg_regs_chirho = saved_regs_chirho;
                            match result_chirho {
                                Ok(v_chirho) => Ok(v_chirho),
                                Err(EvalErrorChirho::RuntimeErrorChirho(msg_chirho)) => {
                                    Err(msg_chirho)
                                }
                                Err(e_chirho) => Err(format!("{}", e_chirho)),
                            }
                        }
                        v_chirho => Ok(v_chirho),
                    }
                };

                // Helper: apply a function value to an argument.
                let apply_to_arg_chirho = |machine_chirho: &mut MachineChirho,
                                           func_val_chirho: ValueChirho,
                                           arg_val_chirho: ValueChirho|
                 -> Result<ValueChirho, String> {
                    match func_val_chirho {
                        ValueChirho::HeapPtrChirho(func_addr_chirho) => {
                            let saved_stack_chirho = std::mem::replace(
                                &mut machine_chirho.stack_chirho,
                                StackChirho::new_chirho(),
                            );
                            let saved_regs_chirho =
                                std::mem::take(&mut machine_chirho.arg_regs_chirho);
                            machine_chirho
                                .stack_chirho
                                .push_chirho(FrameChirho::ApplyChirho {
                                    args_chirho: vec![arg_val_chirho],
                                });
                            let entry_chirho = machine_chirho.emit_enter_chirho(func_addr_chirho);
                            let result_chirho = machine_chirho.run_chirho(entry_chirho);
                            machine_chirho.stack_chirho = saved_stack_chirho;
                            machine_chirho.arg_regs_chirho = saved_regs_chirho;
                            match result_chirho {
                                Ok(v_chirho) => Ok(v_chirho),
                                Err(EvalErrorChirho::RuntimeErrorChirho(msg_chirho)) => {
                                    Err(msg_chirho)
                                }
                                Err(e_chirho) => Err(format!("{}", e_chirho)),
                            }
                        }
                        v_chirho => Ok(v_chirho),
                    }
                };

                // Step 1: run acquire
                let resource_chirho = run_io_val_chirho(self, acquire_val_chirho)
                    .map_err(|msg_chirho| EvalErrorChirho::RuntimeErrorChirho(msg_chirho))?;

                // Step 2: run (body resource), catching exceptions
                let body_result_chirho: Result<ValueChirho, String> =
                    apply_to_arg_chirho(self, body_val_chirho, resource_chirho.clone());

                // Step 3: run (release resource) regardless
                let _ = apply_to_arg_chirho(self, release_val_chirho, resource_chirho);

                // Step 4: return body result or re-throw
                match body_result_chirho {
                    Ok(val_chirho) => return Ok(val_chirho),
                    Err(msg_chirho) => return Err(EvalErrorChirho::RuntimeErrorChirho(msg_chirho)),
                }
            }
            PrimOpKindChirho::FinallyChirho => {
                // finally# action cleanup :: IO a -> IO b -> IO a
                //
                // Run action (catching exceptions), always run cleanup,
                // then re-throw or return action result.
                let action_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let cleanup_val_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));

                // Run action in a sub-context, catching RuntimeError.
                let action_result_chirho: Result<ValueChirho, String> = match action_val_chirho {
                    ValueChirho::HeapPtrChirho(addr_chirho) => {
                        let saved_stack_chirho =
                            std::mem::replace(&mut self.stack_chirho, StackChirho::new_chirho());
                        let saved_regs_chirho = std::mem::take(&mut self.arg_regs_chirho);
                        let entry_chirho = self.code_table_chirho.len() as u32;
                        self.code_table_chirho
                            .push(CodeChirho::EnterChirho(addr_chirho));
                        let result_chirho = self.run_chirho(entry_chirho);
                        self.stack_chirho = saved_stack_chirho;
                        self.arg_regs_chirho = saved_regs_chirho;
                        match result_chirho {
                            Ok(v_chirho) => Ok(v_chirho),
                            Err(EvalErrorChirho::RuntimeErrorChirho(msg_chirho)) => Err(msg_chirho),
                            Err(e_chirho) => Err(format!("{}", e_chirho)),
                        }
                    }
                    v_chirho => Ok(v_chirho),
                };

                // Always run cleanup
                if let ValueChirho::HeapPtrChirho(cleanup_addr_chirho) = cleanup_val_chirho {
                    let saved_stack_chirho =
                        std::mem::replace(&mut self.stack_chirho, StackChirho::new_chirho());
                    let saved_regs_chirho = std::mem::take(&mut self.arg_regs_chirho);
                    let entry_chirho = self.code_table_chirho.len() as u32;
                    self.code_table_chirho
                        .push(CodeChirho::EnterChirho(cleanup_addr_chirho));
                    let _ = self.run_chirho(entry_chirho);
                    self.stack_chirho = saved_stack_chirho;
                    self.arg_regs_chirho = saved_regs_chirho;
                }

                // Return action result or re-throw
                match action_result_chirho {
                    Ok(val_chirho) => return Ok(val_chirho),
                    Err(msg_chirho) => return Err(EvalErrorChirho::RuntimeErrorChirho(msg_chirho)),
                }
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
                return Ok(args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0)));
            }
            PrimOpKindChirho::BindIOChirho => {
                let value_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let cont_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                return self.apply_fn_to_value_chirho(cont_chirho, value_chirho);
            }
            PrimOpKindChirho::ThenIOChirho => {
                return Ok(args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0)));
            }
            PrimOpKindChirho::GetContentsChirho => {
                // Read all remaining stdin as a single String (newline-joined)
                let all_lines_chirho: Vec<String> = self.io_input_chirho.drain(..).collect();
                let contents_chirho = all_lines_chirho.join("\n");
                return Ok(ValueChirho::StringChirho(contents_chirho));
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
                        ValueChirho::HeapPtrChirho(a_chirho) => {
                            self.resolve_string_arg_chirho(*a_chirho)?
                        }
                        _ => String::new(),
                    };
                    let content_str_chirho = match content_chirho {
                        ValueChirho::StringChirho(s_chirho) => s_chirho.clone(),
                        ValueChirho::HeapPtrChirho(a_chirho) => {
                            self.resolve_string_arg_chirho(*a_chirho)?
                        }
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
                        ValueChirho::HeapPtrChirho(a_chirho) => {
                            self.resolve_string_arg_chirho(*a_chirho)?
                        }
                        _ => String::new(),
                    };
                    let content_str_chirho = match content_chirho {
                        ValueChirho::StringChirho(s_chirho) => s_chirho.clone(),
                        ValueChirho::HeapPtrChirho(a_chirho) => {
                            self.resolve_string_arg_chirho(*a_chirho)?
                        }
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
                let nil_closure_chirho = ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
                let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);

                let mut i_chirho = to_chirho;
                while i_chirho >= from_chirho {
                    let cons_closure_chirho = ClosureChirho::con_chirho(
                        cons_tag_chirho,
                        ":",
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
                        cons_tag_chirho,
                        ":",
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
                        cons_tag_chirho,
                        ":",
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
                        cons_tag_chirho,
                        ":",
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

                    let result_chirho = format!("[{}]", elements_chirho.join(","));
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
            PrimOpKindChirho::ShowEitherChirho => {
                // showEither# val → "Left <inner>" | "Right <inner>"
                if let Some(val_chirho) = args_chirho.first() {
                    return Ok(ValueChirho::StringChirho(
                        self.show_value_as_string_chirho(val_chirho),
                    ));
                }
                return Ok(ValueChirho::StringChirho("?".to_string()));
            }
            PrimOpKindChirho::ShowOrderingChirho => {
                // showOrdering# val → "LT" | "EQ" | "GT"
                if let Some(val_chirho) = args_chirho.first() {
                    return Ok(ValueChirho::StringChirho(
                        self.show_value_as_string_chirho(val_chirho),
                    ));
                }
                return Ok(ValueChirho::StringChirho("?".to_string()));
            }
            PrimOpKindChirho::InteractChirho => {
                // interact f = getContents >>= putStr . f
                // Read all io_input, apply f (first arg) to the input string, write result
                let all_input_chirho: String = self
                    .io_input_chirho
                    .drain(..)
                    .collect::<Vec<_>>()
                    .join("\n");
                if let Some(func_val_chirho) = args_chirho.first().cloned() {
                    // Build a cons-list of Chars from the input string so
                    // list functions (reverse, map, etc.) can operate on it
                    let nil_tag_chirho = self.lookup_con_tag_chirho("[]", 0);
                    let cons_tag_chirho = self.lookup_con_tag_chirho(":", 1);
                    let nil_closure_chirho =
                        ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
                    let mut input_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);
                    for ch_chirho in all_input_chirho.chars().rev() {
                        let cons_closure_chirho = ClosureChirho::con_chirho(
                            cons_tag_chirho,
                            ":",
                            vec![
                                ValueChirho::CharChirho(ch_chirho),
                                ValueChirho::HeapPtrChirho(input_addr_chirho),
                            ],
                        );
                        input_addr_chirho = self.heap_chirho.alloc_chirho(cons_closure_chirho);
                    }

                    // Get function address
                    let func_addr_chirho = match &func_val_chirho {
                        ValueChirho::HeapPtrChirho(addr_chirho) => *addr_chirho,
                        _ => {
                            // Fallback: just output input directly
                            self.io_output_chirho.push_str(&all_input_chirho);
                            return Ok(ValueChirho::IntChirho(0));
                        }
                    };

                    // Save machine state for nested eval
                    let saved_stack_chirho =
                        std::mem::replace(&mut self.stack_chirho, StackChirho::new_chirho());
                    let saved_regs_chirho = std::mem::take(&mut self.arg_regs_chirho);

                    // Push Apply frame with the input string arg, then enter function
                    self.stack_chirho.push_chirho(FrameChirho::ApplyChirho {
                        args_chirho: vec![ValueChirho::HeapPtrChirho(input_addr_chirho)],
                    });
                    let entry_chirho = self.code_table_chirho.len() as u32;
                    self.code_table_chirho
                        .push(CodeChirho::EnterChirho(func_addr_chirho));
                    let result_val_chirho = self.run_chirho(entry_chirho)?;

                    // Restore machine state
                    self.stack_chirho = saved_stack_chirho;
                    self.arg_regs_chirho = saved_regs_chirho;

                    // Resolve result to string and write to output
                    let output_str_chirho = match &result_val_chirho {
                        ValueChirho::StringChirho(s_chirho) => s_chirho.clone(),
                        ValueChirho::HeapPtrChirho(_) => {
                            self.resolve_value_to_string_chirho(&result_val_chirho)?
                        }
                        _ => format!("{:?}", result_val_chirho),
                    };
                    self.io_output_chirho.push_str(&output_str_chirho);
                } else {
                    // No function arg: just echo input
                    self.io_output_chirho.push_str(&all_input_chirho);
                }
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
                    let nil_closure_chirho =
                        ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
                    let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);
                    for line_chirho in line_strs_chirho.into_iter().rev() {
                        let cons_closure_chirho = ClosureChirho::con_chirho(
                            cons_tag_chirho,
                            ":",
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
                    let result_chirho = strings_chirho
                        .iter()
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
                            let resolved_chirho =
                                self.heap_chirho.follow_ind_chirho(current_chirho);
                            let closure_chirho =
                                self.heap_chirho.read_chirho(resolved_chirho).clone();
                            if closure_chirho.info_chirho.name_chirho == "[]" {
                                break;
                            }
                            if closure_chirho.info_chirho.name_chirho == ":"
                                && closure_chirho.payload_chirho.len() >= 2
                            {
                                elements_chirho.push(closure_chirho.payload_chirho[0].clone());
                                match &closure_chirho.payload_chirho[1] {
                                    ValueChirho::HeapPtrChirho(tail_chirho) => {
                                        current_chirho = *tail_chirho
                                    }
                                    _ => break,
                                }
                            } else {
                                break;
                            }
                        }
                        // Build result list backwards
                        let nil_closure_chirho =
                            ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
                        let mut tail_addr_chirho =
                            self.heap_chirho.alloc_chirho(nil_closure_chirho);
                        for elem_chirho in elements_chirho.into_iter().rev() {
                            let cons_closure_chirho = ClosureChirho::con_chirho(
                                cons_tag_chirho,
                                ":",
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
                            let resolved_chirho =
                                self.heap_chirho.follow_ind_chirho(current_chirho);
                            let closure_chirho =
                                self.heap_chirho.read_chirho(resolved_chirho).clone();
                            if closure_chirho.info_chirho.name_chirho == "[]" {
                                return Ok(ValueChirho::HeapPtrChirho(resolved_chirho));
                            }
                            if closure_chirho.info_chirho.name_chirho == ":"
                                && closure_chirho.payload_chirho.len() >= 2
                            {
                                match &closure_chirho.payload_chirho[1] {
                                    ValueChirho::HeapPtrChirho(tail_chirho) => {
                                        current_chirho = *tail_chirho
                                    }
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
                    let nil_closure_chirho =
                        ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
                    let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);
                    for word_chirho in words_chirho.into_iter().rev() {
                        let cons_closure_chirho = ClosureChirho::con_chirho(
                            cons_tag_chirho,
                            ":",
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
                if let (
                    Some(ValueChirho::StringChirho(sep_chirho)),
                    Some(ValueChirho::HeapPtrChirho(addr_chirho)),
                ) = (args_chirho.first(), args_chirho.get(1))
                {
                    let strings_chirho = self.collect_string_list_chirho(*addr_chirho);
                    return Ok(ValueChirho::StringChirho(strings_chirho.join(sep_chirho)));
                }
                return Ok(ValueChirho::StringChirho(String::new()));
            }
            PrimOpKindChirho::AppendStrChirho => {
                // John 3:16 - For God so loved the world, that He gave His only begotten Son,
                // that whosoever believeth in Him should not perish, but have everlasting life.
                //
                // ++ operator — handles both String concat and cons-list append.
                let is_cons_list_chirho = |val_chirho: &ValueChirho,
                                           heap_chirho: &crate::heap_chirho::HeapChirho|
                 -> bool {
                    match val_chirho {
                        ValueChirho::HeapPtrChirho(addr_chirho) => {
                            let resolved_chirho = heap_chirho.follow_ind_chirho(*addr_chirho);
                            let c_chirho = heap_chirho.read_chirho(resolved_chirho);
                            let n_chirho = c_chirho.info_chirho.name_chirho.as_str();
                            n_chirho == ":" || n_chirho == "[]"
                        }
                        _ => false,
                    }
                };
                let a_is_list_chirho = is_cons_list_chirho(&args_chirho[0], &self.heap_chirho);
                let b_is_list_chirho = is_cons_list_chirho(&args_chirho[1], &self.heap_chirho);

                if a_is_list_chirho || b_is_list_chirho {
                    // List append: xs ++ ys
                    let ys_addr_chirho = match &args_chirho[1] {
                        ValueChirho::HeapPtrChirho(a_chirho) => {
                            self.heap_chirho.follow_ind_chirho(*a_chirho)
                        }
                        ValueChirho::StringChirho(s_chirho) => {
                            self.alloc_char_list_from_string_chirho(s_chirho)
                        }
                        _ => {
                            let nil_chirho = ClosureChirho::con_chirho(
                                self.lookup_con_tag_chirho("[]", 0),
                                "[]",
                                vec![],
                            );
                            self.heap_chirho.alloc_chirho(nil_chirho)
                        }
                    };

                    // Collect xs elements
                    let mut xs_elems_chirho: Vec<ValueChirho> = Vec::new();
                    match &args_chirho[0] {
                        ValueChirho::HeapPtrChirho(a_chirho) => {
                            let mut cur_chirho = self.heap_chirho.follow_ind_chirho(*a_chirho);
                            for _ in 0..10000 {
                                let forced_chirho = match self.force_addr_to_whnf_chirho(cur_chirho)
                                {
                                    Ok(a_chirho) => a_chirho,
                                    Err(_) => cur_chirho,
                                };
                                let cl_chirho = self.heap_chirho.read_chirho(forced_chirho).clone();
                                if cl_chirho.info_chirho.name_chirho == "[]" {
                                    break;
                                } else if cl_chirho.info_chirho.name_chirho == ":"
                                    && cl_chirho.payload_chirho.len() >= 2
                                {
                                    xs_elems_chirho.push(cl_chirho.payload_chirho[0].clone());
                                    match &cl_chirho.payload_chirho[1] {
                                        ValueChirho::HeapPtrChirho(tail_chirho) => {
                                            cur_chirho =
                                                self.heap_chirho.follow_ind_chirho(*tail_chirho);
                                        }
                                        _ => break,
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                        ValueChirho::StringChirho(s_chirho) => {
                            xs_elems_chirho.extend(s_chirho.chars().map(ValueChirho::CharChirho));
                        }
                        _ => {}
                    }

                    // Build result: prepend xs elements (reversed) onto ys
                    let mut result_addr_chirho = ys_addr_chirho;
                    for elem_chirho in xs_elems_chirho.into_iter().rev() {
                        let cons_chirho = ClosureChirho::con_chirho(
                            self.lookup_con_tag_chirho(":", 1),
                            ":",
                            vec![elem_chirho, ValueChirho::HeapPtrChirho(result_addr_chirho)],
                        );
                        result_addr_chirho = self.heap_chirho.alloc_chirho(cons_chirho);
                    }
                    return Ok(ValueChirho::HeapPtrChirho(result_addr_chirho));
                }

                // Fallback: string concatenation
                let a_chirho = self.resolve_value_to_string_chirho(&args_chirho[0])?;
                let b_chirho = self.resolve_value_to_string_chirho(&args_chirho[1])?;
                return Ok(ValueChirho::StringChirho(format!(
                    "{}{}",
                    a_chirho, b_chirho
                )));
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
                    a_chirho
                        .partial_cmp(&b_chirho)
                        .unwrap_or(std::cmp::Ordering::Equal),
                );
                return Ok(ordering_chirho);
            }
            PrimOpKindChirho::CompareStrChirho => {
                let a_chirho = self.resolve_value_to_string_chirho(&args_chirho[0])?;
                let b_chirho = self.resolve_value_to_string_chirho(&args_chirho[1])?;
                let ordering_chirho = self.make_ordering_chirho(a_chirho.cmp(&b_chirho));
                return Ok(ordering_chirho);
            }
            // ── IORef operations ──
            PrimOpKindChirho::NewIORefChirho => {
                // newIORef# val → allocate a new mutable reference, return its id as Int
                let val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
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
                let val_chirho = self
                    .iorefs_chirho
                    .get(&ref_id_chirho)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                // If the stored value is a HeapPtr (thunk), force it to WHNF
                if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                    match self.force_addr_to_whnf_chirho(addr_chirho) {
                        Ok(forced_chirho) => {
                            let resolved_chirho = self.resolve_heap_value_chirho(
                                &ValueChirho::HeapPtrChirho(forced_chirho),
                            );
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
                let mut val_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                // Force thunks to WHNF before storing
                if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                    match self.force_addr_to_whnf_chirho(addr_chirho) {
                        Ok(forced_chirho) => {
                            val_chirho = self.resolve_heap_value_chirho(
                                &ValueChirho::HeapPtrChirho(forced_chirho),
                            );
                        }
                        Err(_) => {}
                    }
                }
                self.iorefs_chirho.insert(ref_id_chirho, val_chirho);
                return Ok(ValueChirho::IntChirho(0)); // IO ()
            }
            PrimOpKindChirho::ModifyIORefChirho => {
                // modifyIORef# ref f → read current value, apply f, write back
                let ref_id_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho as u64,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                // Read current value from the ref store
                let current_chirho = self
                    .iorefs_chirho
                    .get(&ref_id_chirho)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                // Apply the function (second arg) to the current value
                let func_val_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let new_val_chirho = match func_val_chirho {
                    ValueChirho::HeapPtrChirho(func_addr_chirho) => {
                        // Allocate current_chirho on heap as argument if needed
                        let arg_val_chirho = match &current_chirho {
                            ValueChirho::IntChirho(_)
                            | ValueChirho::FloatChirho(_)
                            | ValueChirho::CharChirho(_)
                            | ValueChirho::BoolChirho(_)
                            | ValueChirho::StringChirho(_) => {
                                // Box into a constructor so it can be passed as a HeapPtr arg
                                let boxed_chirho = self.box_literal_chirho(&current_chirho);
                                let boxed_addr_chirho = self.heap_chirho.alloc_chirho(boxed_chirho);
                                ValueChirho::HeapPtrChirho(boxed_addr_chirho)
                            }
                            other_chirho => other_chirho.clone(),
                        };
                        // Save machine state for nested eval
                        let saved_stack_chirho =
                            std::mem::replace(&mut self.stack_chirho, StackChirho::new_chirho());
                        let saved_regs_chirho = std::mem::take(&mut self.arg_regs_chirho);
                        // Push Apply frame with argument, enter function
                        self.stack_chirho.push_chirho(FrameChirho::ApplyChirho {
                            args_chirho: vec![arg_val_chirho],
                        });
                        let entry_chirho = self.code_table_chirho.len() as u32;
                        self.code_table_chirho
                            .push(CodeChirho::EnterChirho(func_addr_chirho));
                        let result_val_chirho = self.run_chirho(entry_chirho)?;
                        // Restore machine state
                        self.stack_chirho = saved_stack_chirho;
                        self.arg_regs_chirho = saved_regs_chirho;
                        // Resolve HeapPtr result to unboxed value if possible
                        match &result_val_chirho {
                            ValueChirho::HeapPtrChirho(addr_chirho) => self
                                .resolve_heap_value_chirho(&ValueChirho::HeapPtrChirho(
                                    *addr_chirho,
                                )),
                            other_chirho => other_chirho.clone(),
                        }
                    }
                    // If f is not a heap closure (identity-like), keep the value unchanged
                    _ => current_chirho,
                };
                // Write the new value back to the ref store
                self.iorefs_chirho.insert(ref_id_chirho, new_val_chirho);
                return Ok(ValueChirho::IntChirho(0)); // IO ()
            }
            // ── ST monad operations ──
            // ST operations reuse the ioref infrastructure (same HashMap)
            // but are scoped via runST which gives pure semantics.
            PrimOpKindChirho::NewSTRefChirho => {
                // newSTRef# val → allocate a new mutable reference, return id as Int
                let val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let id_chirho = self.next_ioref_chirho;
                self.next_ioref_chirho += 1;
                self.iorefs_chirho.insert(id_chirho, val_chirho);
                return Ok(ValueChirho::IntChirho(id_chirho as i64));
            }
            PrimOpKindChirho::ReadSTRefChirho => {
                // readSTRef# ref → read the current value from the mutable reference
                let ref_id_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho as u64,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let val_chirho = self
                    .iorefs_chirho
                    .get(&ref_id_chirho)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                // If the stored value is a HeapPtr (thunk), force it to WHNF
                if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                    match self.force_addr_to_whnf_chirho(addr_chirho) {
                        Ok(forced_chirho) => {
                            let resolved_chirho = self.resolve_heap_value_chirho(
                                &ValueChirho::HeapPtrChirho(forced_chirho),
                            );
                            return Ok(resolved_chirho);
                        }
                        Err(_) => return Ok(ValueChirho::HeapPtrChirho(addr_chirho)),
                    }
                }
                return Ok(val_chirho);
            }
            PrimOpKindChirho::WriteSTRefChirho => {
                // writeSTRef# ref val → overwrite the value in the mutable reference
                let ref_id_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho as u64,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let mut val_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                // Force thunks to WHNF before storing
                if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                    match self.force_addr_to_whnf_chirho(addr_chirho) {
                        Ok(forced_chirho) => {
                            val_chirho = self.resolve_heap_value_chirho(
                                &ValueChirho::HeapPtrChirho(forced_chirho),
                            );
                        }
                        Err(_) => {}
                    }
                }
                self.iorefs_chirho.insert(ref_id_chirho, val_chirho);
                return Ok(ValueChirho::IntChirho(0)); // ST s ()
            }
            PrimOpKindChirho::ModifySTRefChirho => {
                // modifySTRef# ref f → read current value, apply f, write back; ST s ()
                let ref_id_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho as u64,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                // Read current value from the ST ref store
                let current_chirho = self
                    .iorefs_chirho
                    .get(&ref_id_chirho)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                // Apply the function (second arg) to the current value
                let func_val_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let new_val_chirho = match func_val_chirho {
                    ValueChirho::HeapPtrChirho(func_addr_chirho) => {
                        // Box literal arg onto heap so it can be passed as a HeapPtr
                        let arg_val_chirho = match &current_chirho {
                            ValueChirho::IntChirho(_)
                            | ValueChirho::FloatChirho(_)
                            | ValueChirho::CharChirho(_)
                            | ValueChirho::BoolChirho(_)
                            | ValueChirho::StringChirho(_) => {
                                let boxed_chirho = self.box_literal_chirho(&current_chirho);
                                let boxed_addr_chirho = self.heap_chirho.alloc_chirho(boxed_chirho);
                                ValueChirho::HeapPtrChirho(boxed_addr_chirho)
                            }
                            other_chirho => other_chirho.clone(),
                        };
                        // Save machine state for nested evaluation
                        let saved_stack_chirho =
                            std::mem::replace(&mut self.stack_chirho, StackChirho::new_chirho());
                        let saved_regs_chirho = std::mem::take(&mut self.arg_regs_chirho);
                        // Push Apply frame with argument, then enter the function closure
                        self.stack_chirho.push_chirho(FrameChirho::ApplyChirho {
                            args_chirho: vec![arg_val_chirho],
                        });
                        let entry_chirho = self.code_table_chirho.len() as u32;
                        self.code_table_chirho
                            .push(CodeChirho::EnterChirho(func_addr_chirho));
                        let result_val_chirho = self.run_chirho(entry_chirho)?;
                        // Restore machine state
                        self.stack_chirho = saved_stack_chirho;
                        self.arg_regs_chirho = saved_regs_chirho;
                        // Resolve HeapPtr result to an unboxed primitive if possible
                        match &result_val_chirho {
                            ValueChirho::HeapPtrChirho(addr_chirho) => self
                                .resolve_heap_value_chirho(&ValueChirho::HeapPtrChirho(
                                    *addr_chirho,
                                )),
                            other_chirho => other_chirho.clone(),
                        }
                    }
                    // If f is not a heap closure, keep the value unchanged
                    _ => current_chirho,
                };
                // Write the new value back into the ST ref store
                self.iorefs_chirho.insert(ref_id_chirho, new_val_chirho);
                return Ok(ValueChirho::IntChirho(0)); // ST s ()
            }
            PrimOpKindChirho::RunSTChirho => {
                // runST# computation → execute the ST computation and return result
                // Since our ST monad is just identity-like (same as IO without I/O),
                // we simply evaluate the argument which should be the result of the
                // ST computation.
                let val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                return Ok(val_chirho);
            }

            // ── STM (Software Transactional Memory) operations ──
            PrimOpKindChirho::NewTVarChirho => {
                // newTVar# val → allocate a new TVar, return its id as Int
                let val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let id_chirho = self.next_tvar_chirho;
                self.next_tvar_chirho += 1;
                self.tvars_chirho.insert(id_chirho, val_chirho);
                return Ok(ValueChirho::IntChirho(id_chirho as i64));
            }
            PrimOpKindChirho::ReadTVarChirho => {
                // readTVar# tvar → read the current value of a TVar
                let tvar_id_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho as u64,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let val_chirho = self
                    .tvars_chirho
                    .get(&tvar_id_chirho)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                // Force thunks to WHNF if needed
                if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                    match self.force_addr_to_whnf_chirho(addr_chirho) {
                        Ok(forced_chirho) => {
                            let resolved_chirho = self.resolve_heap_value_chirho(
                                &ValueChirho::HeapPtrChirho(forced_chirho),
                            );
                            return Ok(resolved_chirho);
                        }
                        Err(_) => return Ok(ValueChirho::HeapPtrChirho(addr_chirho)),
                    }
                }
                return Ok(val_chirho);
            }
            PrimOpKindChirho::WriteTVarChirho => {
                // writeTVar# tvar val → write a new value to a TVar
                let tvar_id_chirho = match args_chirho.first() {
                    Some(ValueChirho::IntChirho(n_chirho)) => *n_chirho as u64,
                    _ => return Ok(ValueChirho::IntChirho(0)),
                };
                let mut val_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                // Force thunks to WHNF before storing
                if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                    match self.force_addr_to_whnf_chirho(addr_chirho) {
                        Ok(forced_chirho) => {
                            val_chirho = self.resolve_heap_value_chirho(
                                &ValueChirho::HeapPtrChirho(forced_chirho),
                            );
                        }
                        Err(_) => {}
                    }
                }
                self.tvars_chirho.insert(tvar_id_chirho, val_chirho);
                return Ok(ValueChirho::IntChirho(0)); // STM ()
            }
            PrimOpKindChirho::AtomicallyChirho => {
                // atomically# action → execute STM action (in single-threaded evaluator,
                // transactions always succeed since there's no concurrent modification)
                let val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                return Ok(val_chirho);
            }
            PrimOpKindChirho::RetryChirho => {
                // retry# → in single-threaded evaluator, retry blocks forever since
                // there's no concurrent writer to change TVars. Signal error.
                return Err(EvalErrorChirho::RuntimeErrorChirho(
                    "STM retry: no concurrent transaction to resolve retry".to_string(),
                ));
            }
            PrimOpKindChirho::OrElseChirho => {
                // orElse# action1 action2 → try action1, if it retries, try action2
                // In single-threaded mode, just return the first action's result
                let val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                return Ok(val_chirho);
            }

            // ── Data.Map primops ──────────────────────────────────────────
            PrimOpKindChirho::MapEmptyChirho => {
                return Ok(ValueChirho::MapChirho(vec![]));
            }
            PrimOpKindChirho::MapSingletonChirho => {
                let k_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .first()
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let v_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .get(1)
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                return Ok(ValueChirho::MapChirho(vec![(k_chirho, v_chirho)]));
            }
            PrimOpKindChirho::MapInsertChirho => {
                let k_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .first()
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let v_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .get(1)
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let map_arg_chirho = args_chirho
                    .get(2)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let mut pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                // Binary search insert/overwrite
                match pairs_chirho
                    .binary_search_by(|(ek_chirho, _)| compare_values_chirho(ek_chirho, &k_chirho))
                {
                    Ok(pos_chirho) => {
                        pairs_chirho[pos_chirho].1 = v_chirho;
                    }
                    Err(pos_chirho) => {
                        pairs_chirho.insert(pos_chirho, (k_chirho, v_chirho));
                    }
                }
                return Ok(ValueChirho::MapChirho(pairs_chirho));
            }
            PrimOpKindChirho::MapLookupChirho => {
                let k_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .first()
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let map_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                match pairs_chirho
                    .binary_search_by(|(ek_chirho, _)| compare_values_chirho(ek_chirho, &k_chirho))
                {
                    Ok(pos_chirho) => {
                        let found_val_chirho = pairs_chirho[pos_chirho].1.clone();
                        // Allocate Just(v) on the heap
                        let just_tag_chirho = self.lookup_con_tag_chirho("Just", 1);
                        let just_closure_chirho = ClosureChirho::con_chirho(
                            just_tag_chirho,
                            "Just",
                            vec![found_val_chirho],
                        );
                        let just_addr_chirho = self.heap_chirho.alloc_chirho(just_closure_chirho);
                        return Ok(ValueChirho::HeapPtrChirho(just_addr_chirho));
                    }
                    Err(_) => {
                        // Allocate Nothing on the heap
                        let nothing_tag_chirho = self.lookup_con_tag_chirho("Nothing", 0);
                        let nothing_closure_chirho =
                            ClosureChirho::con_chirho(nothing_tag_chirho, "Nothing", vec![]);
                        let nothing_addr_chirho =
                            self.heap_chirho.alloc_chirho(nothing_closure_chirho);
                        return Ok(ValueChirho::HeapPtrChirho(nothing_addr_chirho));
                    }
                }
            }
            PrimOpKindChirho::MapDeleteChirho => {
                let k_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .first()
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let map_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let mut pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                if let Ok(pos_chirho) = pairs_chirho
                    .binary_search_by(|(ek_chirho, _)| compare_values_chirho(ek_chirho, &k_chirho))
                {
                    pairs_chirho.remove(pos_chirho);
                }
                return Ok(ValueChirho::MapChirho(pairs_chirho));
            }
            PrimOpKindChirho::MapMemberChirho => {
                let k_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .first()
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let map_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                let found_chirho = pairs_chirho
                    .binary_search_by(|(ek_chirho, _)| compare_values_chirho(ek_chirho, &k_chirho))
                    .is_ok();
                return Ok(ValueChirho::BoolChirho(found_chirho));
            }
            PrimOpKindChirho::MapSizeChirho => {
                let map_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                return Ok(ValueChirho::IntChirho(pairs_chirho.len() as i64));
            }
            PrimOpKindChirho::MapNullChirho => {
                let map_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                return Ok(ValueChirho::BoolChirho(pairs_chirho.is_empty()));
            }
            PrimOpKindChirho::MapFromListChirho => {
                // Build a map from a heap list of (k, v) tuples
                let list_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::HeapPtrChirho(HeapAddrChirho(0)));
                let pairs_chirho = self.collect_map_from_list_chirho(list_arg_chirho);
                return Ok(ValueChirho::MapChirho(pairs_chirho));
            }
            PrimOpKindChirho::MapToListChirho => {
                let map_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                // Build a heap list of (k,v) tuple constructors
                let heap_list_chirho = self.build_pair_list_chirho(pairs_chirho);
                return Ok(heap_list_chirho);
            }
            PrimOpKindChirho::MapKeysChirho => {
                let map_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                let keys_chirho: Vec<ValueChirho> = pairs_chirho
                    .into_iter()
                    .map(|(k_chirho, _)| k_chirho)
                    .collect();
                let heap_list_chirho = self.build_value_list_chirho(keys_chirho);
                return Ok(heap_list_chirho);
            }
            PrimOpKindChirho::MapElemsChirho => {
                let map_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                let vals_chirho: Vec<ValueChirho> = pairs_chirho
                    .into_iter()
                    .map(|(_, v_chirho)| v_chirho)
                    .collect();
                let heap_list_chirho = self.build_value_list_chirho(vals_chirho);
                return Ok(heap_list_chirho);
            }
            PrimOpKindChirho::MapMapChirho => {
                // mapMap# f map — apply f to each value
                // f is a heap closure; we apply it to each value using nested eval
                let func_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let map_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                let mut new_pairs_chirho: Vec<(ValueChirho, ValueChirho)> =
                    Vec::with_capacity(pairs_chirho.len());
                for (k_chirho, v_chirho) in pairs_chirho {
                    let raw_v_chirho =
                        self.apply_fn_to_value_chirho(func_val_chirho.clone(), v_chirho)?;
                    // Force result to primitive so lookup returns unboxed values
                    let new_v_chirho = self.force_to_prim_chirho(raw_v_chirho);
                    new_pairs_chirho.push((k_chirho, new_v_chirho));
                }
                return Ok(ValueChirho::MapChirho(new_pairs_chirho));
            }
            PrimOpKindChirho::MapFoldlWithKeyChirho => {
                // mapFoldlWithKey# f z map — strict left fold
                let func_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let mut acc_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .get(1)
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let map_arg_chirho = args_chirho
                    .get(2)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                for (k_chirho, v_chirho) in pairs_chirho {
                    // Apply f acc k v, forcing result after each step
                    let after_acc_chirho =
                        self.apply_fn_to_value_chirho(func_val_chirho.clone(), acc_chirho)?;
                    let after_k_chirho =
                        self.apply_fn_to_value_chirho(after_acc_chirho, k_chirho)?;
                    let raw_acc_chirho = self.apply_fn_to_value_chirho(after_k_chirho, v_chirho)?;
                    acc_chirho = self.force_to_prim_chirho(raw_acc_chirho);
                }
                return Ok(acc_chirho);
            }
            PrimOpKindChirho::MapFoldrWithKeyChirho => {
                // mapFoldrWithKey# f z map — right fold
                let func_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let init_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .get(1)
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let map_arg_chirho = args_chirho
                    .get(2)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                let mut acc_chirho = init_chirho;
                for (k_chirho, v_chirho) in pairs_chirho.into_iter().rev() {
                    // Apply f k v acc, forcing after each application
                    let after_k_chirho =
                        self.apply_fn_to_value_chirho(func_val_chirho.clone(), k_chirho)?;
                    let after_v_chirho = self.apply_fn_to_value_chirho(after_k_chirho, v_chirho)?;
                    let raw_acc_chirho =
                        self.apply_fn_to_value_chirho(after_v_chirho, acc_chirho)?;
                    acc_chirho = self.force_to_prim_chirho(raw_acc_chirho);
                }
                return Ok(acc_chirho);
            }
            PrimOpKindChirho::MapUnionChirho => {
                // mapUnion# left right — left-biased union
                let left_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let right_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let left_pairs_chirho = self.extract_map_pairs_chirho(left_arg_chirho);
                let right_pairs_chirho = self.extract_map_pairs_chirho(right_arg_chirho);
                let mut result_chirho = left_pairs_chirho;
                for (k_chirho, v_chirho) in right_pairs_chirho {
                    match result_chirho.binary_search_by(|(ek_chirho, _)| {
                        compare_values_chirho(ek_chirho, &k_chirho)
                    }) {
                        Ok(_) => {} // Key already in left; left-biased: skip
                        Err(pos_chirho) => {
                            result_chirho.insert(pos_chirho, (k_chirho, v_chirho));
                        }
                    }
                }
                return Ok(ValueChirho::MapChirho(result_chirho));
            }
            PrimOpKindChirho::MapDifferenceChirho => {
                // mapDifference# left right — keys in left not in right
                let left_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let right_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let left_pairs_chirho = self.extract_map_pairs_chirho(left_arg_chirho);
                let right_pairs_chirho = self.extract_map_pairs_chirho(right_arg_chirho);
                let result_chirho: Vec<(ValueChirho, ValueChirho)> = left_pairs_chirho
                    .into_iter()
                    .filter(|(k_chirho, _)| {
                        right_pairs_chirho
                            .binary_search_by(|(ek_chirho, _)| {
                                compare_values_chirho(ek_chirho, k_chirho)
                            })
                            .is_err()
                    })
                    .collect();
                return Ok(ValueChirho::MapChirho(result_chirho));
            }
            PrimOpKindChirho::MapIntersectionChirho => {
                // mapIntersection# left right — keys in both; left values
                let left_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let right_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let left_pairs_chirho = self.extract_map_pairs_chirho(left_arg_chirho);
                let right_pairs_chirho = self.extract_map_pairs_chirho(right_arg_chirho);
                let result_chirho: Vec<(ValueChirho, ValueChirho)> = left_pairs_chirho
                    .into_iter()
                    .filter(|(k_chirho, _)| {
                        right_pairs_chirho
                            .binary_search_by(|(ek_chirho, _)| {
                                compare_values_chirho(ek_chirho, k_chirho)
                            })
                            .is_ok()
                    })
                    .collect();
                return Ok(ValueChirho::MapChirho(result_chirho));
            }
            PrimOpKindChirho::MapIntersectionWithChirho => {
                // mapIntersectionWith# f left right — combine values only for shared keys
                let func_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let left_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let right_arg_chirho = args_chirho
                    .get(2)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let left_pairs_chirho = self.extract_map_pairs_chirho(left_arg_chirho);
                let right_pairs_chirho = self.extract_map_pairs_chirho(right_arg_chirho);
                let mut result_chirho = Vec::new();
                for (k_chirho, v_left_chirho) in left_pairs_chirho {
                    if let Ok(pos_chirho) = right_pairs_chirho.binary_search_by(|(ek_chirho, _)| {
                        compare_values_chirho(ek_chirho, &k_chirho)
                    }) {
                        let v_right_chirho = right_pairs_chirho[pos_chirho].1.clone();
                        let combined_chirho = {
                            let af_chirho = self
                                .apply_fn_to_value_chirho(func_val_chirho.clone(), v_left_chirho)?;
                            let raw_chirho =
                                self.apply_fn_to_value_chirho(af_chirho, v_right_chirho)?;
                            self.force_to_prim_chirho(raw_chirho)
                        };
                        result_chirho.push((k_chirho, combined_chirho));
                    }
                }
                return Ok(ValueChirho::MapChirho(result_chirho));
            }
            PrimOpKindChirho::MapInsertWithChirho => {
                // mapInsertWith# f k v map — insert with combiner f
                let func_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let k_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .get(1)
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let v_new_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .get(2)
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let map_arg_chirho = args_chirho
                    .get(3)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let mut pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                match pairs_chirho
                    .binary_search_by(|(ek_chirho, _)| compare_values_chirho(ek_chirho, &k_chirho))
                {
                    Ok(pos_chirho) => {
                        let old_v_chirho = pairs_chirho[pos_chirho].1.clone();
                        // f new_v old_v
                        let combined_chirho = {
                            let after_new_chirho = self
                                .apply_fn_to_value_chirho(func_val_chirho.clone(), v_new_chirho)?;
                            let raw_chirho =
                                self.apply_fn_to_value_chirho(after_new_chirho, old_v_chirho)?;
                            self.force_to_prim_chirho(raw_chirho)
                        };
                        pairs_chirho[pos_chirho].1 = combined_chirho;
                    }
                    Err(pos_chirho) => {
                        pairs_chirho.insert(pos_chirho, (k_chirho, v_new_chirho));
                    }
                }
                return Ok(ValueChirho::MapChirho(pairs_chirho));
            }
            PrimOpKindChirho::MapFindWithDefaultChirho => {
                // mapFindWithDefault# def k map — lookup with default
                let def_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .first()
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let k_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .get(1)
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let map_arg_chirho = args_chirho
                    .get(2)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                match pairs_chirho
                    .binary_search_by(|(ek_chirho, _)| compare_values_chirho(ek_chirho, &k_chirho))
                {
                    Ok(pos_chirho) => return Ok(pairs_chirho[pos_chirho].1.clone()),
                    Err(_) => return Ok(def_chirho),
                }
            }
            PrimOpKindChirho::MapAdjustChirho => {
                // mapAdjust# f k map — update value at key with f
                let func_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let k_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .get(1)
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let map_arg_chirho = args_chirho
                    .get(2)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let mut pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                if let Ok(pos_chirho) = pairs_chirho
                    .binary_search_by(|(ek_chirho, _)| compare_values_chirho(ek_chirho, &k_chirho))
                {
                    let old_v_chirho = pairs_chirho[pos_chirho].1.clone();
                    let raw_v_chirho =
                        self.apply_fn_to_value_chirho(func_val_chirho, old_v_chirho)?;
                    pairs_chirho[pos_chirho].1 = self.force_to_prim_chirho(raw_v_chirho);
                }
                return Ok(ValueChirho::MapChirho(pairs_chirho));
            }
            PrimOpKindChirho::MapUnionWithChirho => {
                // mapUnionWith# f left right — union with combiner for duplicate keys
                let func_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let left_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let right_arg_chirho = args_chirho
                    .get(2)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let mut result_chirho = self.extract_map_pairs_chirho(left_arg_chirho);
                let right_pairs_chirho = self.extract_map_pairs_chirho(right_arg_chirho);
                for (k_chirho, v_right_chirho) in right_pairs_chirho {
                    match result_chirho.binary_search_by(|(ek_chirho, _)| {
                        compare_values_chirho(ek_chirho, &k_chirho)
                    }) {
                        Ok(pos_chirho) => {
                            let v_left_chirho = result_chirho[pos_chirho].1.clone();
                            let combined_chirho = {
                                let af_chirho = self.apply_fn_to_value_chirho(
                                    func_val_chirho.clone(),
                                    v_left_chirho,
                                )?;
                                let raw_chirho =
                                    self.apply_fn_to_value_chirho(af_chirho, v_right_chirho)?;
                                self.force_to_prim_chirho(raw_chirho)
                            };
                            result_chirho[pos_chirho].1 = combined_chirho;
                        }
                        Err(pos_chirho) => {
                            result_chirho.insert(pos_chirho, (k_chirho, v_right_chirho));
                        }
                    }
                }
                return Ok(ValueChirho::MapChirho(result_chirho));
            }
            PrimOpKindChirho::MapFilterChirho => {
                // mapFilter# pred map — keep (k,v) where pred v is True
                let func_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let map_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                let mut kept_chirho: Vec<(ValueChirho, ValueChirho)> = Vec::new();
                for (k_chirho, v_chirho) in pairs_chirho {
                    let result_chirho =
                        self.apply_fn_to_value_chirho(func_val_chirho.clone(), v_chirho.clone())?;
                    let keep_chirho = match self.force_to_prim_chirho(result_chirho) {
                        ValueChirho::BoolChirho(b_chirho) => b_chirho,
                        ValueChirho::IntChirho(n_chirho) => n_chirho != 0,
                        _ => false,
                    };
                    if keep_chirho {
                        kept_chirho.push((k_chirho, v_chirho));
                    }
                }
                return Ok(ValueChirho::MapChirho(kept_chirho));
            }
            PrimOpKindChirho::MapFilterWithKeyChirho => {
                // mapFilterWithKey# pred map — keep (k,v) where pred k v is True
                let func_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let map_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::MapChirho(vec![]));
                let pairs_chirho = self.extract_map_pairs_chirho(map_arg_chirho);
                let mut kept_chirho: Vec<(ValueChirho, ValueChirho)> = Vec::new();
                for (k_chirho, v_chirho) in pairs_chirho {
                    let after_k_chirho =
                        self.apply_fn_to_value_chirho(func_val_chirho.clone(), k_chirho.clone())?;
                    let result_chirho =
                        self.apply_fn_to_value_chirho(after_k_chirho, v_chirho.clone())?;
                    let keep_chirho = match self.force_to_prim_chirho(result_chirho) {
                        ValueChirho::BoolChirho(b_chirho) => b_chirho,
                        ValueChirho::IntChirho(n_chirho) => n_chirho != 0,
                        _ => false,
                    };
                    if keep_chirho {
                        kept_chirho.push((k_chirho, v_chirho));
                    }
                }
                return Ok(ValueChirho::MapChirho(kept_chirho));
            }

            // ── Data.Set primops ──────────────────────────────────────────
            PrimOpKindChirho::SetEmptyChirho => {
                return Ok(ValueChirho::SetChirho(vec![]));
            }
            PrimOpKindChirho::SetSingletonChirho => {
                let e_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .first()
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                return Ok(ValueChirho::SetChirho(vec![e_chirho]));
            }
            PrimOpKindChirho::SetInsertChirho => {
                let e_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .first()
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let set_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let mut elems_chirho = self.extract_set_elems_chirho(set_arg_chirho);
                match elems_chirho
                    .binary_search_by(|ek_chirho| compare_values_chirho(ek_chirho, &e_chirho))
                {
                    Ok(_) => {} // already present — dedup
                    Err(pos_chirho) => {
                        elems_chirho.insert(pos_chirho, e_chirho);
                    }
                }
                return Ok(ValueChirho::SetChirho(elems_chirho));
            }
            PrimOpKindChirho::SetMemberChirho => {
                let e_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .first()
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let set_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let elems_chirho = self.extract_set_elems_chirho(set_arg_chirho);
                let found_chirho = elems_chirho
                    .binary_search_by(|ek_chirho| compare_values_chirho(ek_chirho, &e_chirho))
                    .is_ok();
                return Ok(ValueChirho::BoolChirho(found_chirho));
            }
            PrimOpKindChirho::SetDeleteChirho => {
                let e_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .first()
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let set_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let mut elems_chirho = self.extract_set_elems_chirho(set_arg_chirho);
                if let Ok(pos_chirho) = elems_chirho
                    .binary_search_by(|ek_chirho| compare_values_chirho(ek_chirho, &e_chirho))
                {
                    elems_chirho.remove(pos_chirho);
                }
                return Ok(ValueChirho::SetChirho(elems_chirho));
            }
            PrimOpKindChirho::SetSizeChirho => {
                let set_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let elems_chirho = self.extract_set_elems_chirho(set_arg_chirho);
                return Ok(ValueChirho::IntChirho(elems_chirho.len() as i64));
            }
            PrimOpKindChirho::SetNullChirho => {
                let set_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let elems_chirho = self.extract_set_elems_chirho(set_arg_chirho);
                return Ok(ValueChirho::BoolChirho(elems_chirho.is_empty()));
            }
            PrimOpKindChirho::SetFromListChirho => {
                // Build a sorted deduplicated set from a heap list
                let list_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::HeapPtrChirho(HeapAddrChirho(0)));
                let elems_chirho = self.collect_set_from_list_chirho(list_arg_chirho);
                return Ok(ValueChirho::SetChirho(elems_chirho));
            }
            PrimOpKindChirho::SetToListChirho => {
                let set_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let elems_chirho = self.extract_set_elems_chirho(set_arg_chirho);
                let heap_list_chirho = self.build_value_list_chirho(elems_chirho);
                return Ok(heap_list_chirho);
            }
            PrimOpKindChirho::SetUnionChirho => {
                // Merge two sorted vecs — union of sets
                let left_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let right_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let left_elems_chirho = self.extract_set_elems_chirho(left_arg_chirho);
                let right_elems_chirho = self.extract_set_elems_chirho(right_arg_chirho);
                let mut result_chirho = left_elems_chirho;
                for e_chirho in right_elems_chirho {
                    match result_chirho
                        .binary_search_by(|ek_chirho| compare_values_chirho(ek_chirho, &e_chirho))
                    {
                        Ok(_) => {} // already present
                        Err(pos_chirho) => {
                            result_chirho.insert(pos_chirho, e_chirho);
                        }
                    }
                }
                return Ok(ValueChirho::SetChirho(result_chirho));
            }
            PrimOpKindChirho::SetIntersectionChirho => {
                // Keep elements common to both sets
                let left_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let right_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let left_elems_chirho = self.extract_set_elems_chirho(left_arg_chirho);
                let right_elems_chirho = self.extract_set_elems_chirho(right_arg_chirho);
                let result_chirho: Vec<ValueChirho> = left_elems_chirho
                    .into_iter()
                    .filter(|e_chirho| {
                        right_elems_chirho
                            .binary_search_by(|ek_chirho| {
                                compare_values_chirho(ek_chirho, e_chirho)
                            })
                            .is_ok()
                    })
                    .collect();
                return Ok(ValueChirho::SetChirho(result_chirho));
            }
            PrimOpKindChirho::SetDifferenceChirho => {
                // Elements in left not in right
                let left_arg_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let right_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let left_elems_chirho = self.extract_set_elems_chirho(left_arg_chirho);
                let right_elems_chirho = self.extract_set_elems_chirho(right_arg_chirho);
                let result_chirho: Vec<ValueChirho> = left_elems_chirho
                    .into_iter()
                    .filter(|e_chirho| {
                        right_elems_chirho
                            .binary_search_by(|ek_chirho| {
                                compare_values_chirho(ek_chirho, e_chirho)
                            })
                            .is_err()
                    })
                    .collect();
                return Ok(ValueChirho::SetChirho(result_chirho));
            }
            PrimOpKindChirho::SetMapChirho => {
                // setMap# f set — apply f to each element, re-sort and dedup
                let func_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let set_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let elems_chirho = self.extract_set_elems_chirho(set_arg_chirho);
                let mut new_elems_chirho: Vec<ValueChirho> = Vec::with_capacity(elems_chirho.len());
                for e_chirho in elems_chirho {
                    let raw_chirho =
                        self.apply_fn_to_value_chirho(func_val_chirho.clone(), e_chirho)?;
                    new_elems_chirho.push(self.force_to_prim_chirho(raw_chirho));
                }
                new_elems_chirho.sort_by(compare_values_chirho);
                new_elems_chirho.dedup_by(|a_chirho, b_chirho| {
                    compare_values_chirho(a_chirho, b_chirho) == std::cmp::Ordering::Equal
                });
                return Ok(ValueChirho::SetChirho(new_elems_chirho));
            }
            PrimOpKindChirho::SetFilterChirho => {
                // setFilter# pred set — keep elements where pred e is True
                let func_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let set_arg_chirho = args_chirho
                    .get(1)
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let elems_chirho = self.extract_set_elems_chirho(set_arg_chirho);
                let mut kept_chirho: Vec<ValueChirho> = Vec::new();
                for e_chirho in elems_chirho {
                    let result_chirho =
                        self.apply_fn_to_value_chirho(func_val_chirho.clone(), e_chirho.clone())?;
                    let keep_chirho = match self.force_to_prim_chirho(result_chirho) {
                        ValueChirho::BoolChirho(b_chirho) => b_chirho,
                        ValueChirho::IntChirho(n_chirho) => n_chirho != 0,
                        _ => false,
                    };
                    if keep_chirho {
                        kept_chirho.push(e_chirho);
                    }
                }
                return Ok(ValueChirho::SetChirho(kept_chirho));
            }
            PrimOpKindChirho::SetFoldrChirho => {
                // setFoldr# f z set — right fold over set elements
                let func_val_chirho = args_chirho
                    .first()
                    .cloned()
                    .unwrap_or(ValueChirho::IntChirho(0));
                let init_chirho = self.force_to_prim_chirho(
                    args_chirho
                        .get(1)
                        .cloned()
                        .unwrap_or(ValueChirho::IntChirho(0)),
                );
                let set_arg_chirho = args_chirho
                    .get(2)
                    .cloned()
                    .unwrap_or(ValueChirho::SetChirho(vec![]));
                let elems_chirho = self.extract_set_elems_chirho(set_arg_chirho);
                let mut acc_chirho = init_chirho;
                for e_chirho in elems_chirho.into_iter().rev() {
                    let after_e_chirho =
                        self.apply_fn_to_value_chirho(func_val_chirho.clone(), e_chirho)?;
                    let raw_chirho = self.apply_fn_to_value_chirho(after_e_chirho, acc_chirho)?;
                    acc_chirho = self.force_to_prim_chirho(raw_chirho);
                }
                return Ok(acc_chirho);
            }

            _ => {}
        }

        // For Show primops receiving HeapPtrChirho (compound values like
        // nested lists, tuples, etc.): use show_value_as_string_chirho which
        // handles all runtime value types recursively.
        // Additionally, when a show primop receives a value of a mismatched
        // concrete type (e.g. ShowStrChirho applied to IntChirho due to
        // type-key inference limitations), fall back to show_value_as_string_chirho
        // to produce a correct result rather than failing.
        let is_show_primop_chirho = matches!(
            op_chirho,
            PrimOpKindChirho::ShowIntChirho
                | PrimOpKindChirho::ShowFloatChirho
                | PrimOpKindChirho::ShowStrChirho
                | PrimOpKindChirho::ShowBoolChirho
                | PrimOpKindChirho::ShowCharChirho
        );
        if is_show_primop_chirho {
            if let Some(val_chirho) = args_chirho.first() {
                let needs_fallback_chirho = matches!(val_chirho, ValueChirho::HeapPtrChirho(_))
                    || match (op_chirho, val_chirho) {
                        // ShowStrChirho applied to non-String: use universal show
                        (PrimOpKindChirho::ShowStrChirho, ValueChirho::StringChirho(_)) => false,
                        (PrimOpKindChirho::ShowStrChirho, _) => true,
                        // ShowIntChirho applied to non-Int: use universal show
                        (PrimOpKindChirho::ShowIntChirho, ValueChirho::IntChirho(_)) => false,
                        (PrimOpKindChirho::ShowIntChirho, _) => true,
                        // ShowBoolChirho applied to non-Bool/Int: use universal show
                        (PrimOpKindChirho::ShowBoolChirho, ValueChirho::BoolChirho(_)) => false,
                        (PrimOpKindChirho::ShowBoolChirho, ValueChirho::IntChirho(_)) => false,
                        (PrimOpKindChirho::ShowBoolChirho, _) => true,
                        // ShowCharChirho applied to non-Char: use universal show
                        (PrimOpKindChirho::ShowCharChirho, ValueChirho::CharChirho(_)) => false,
                        (PrimOpKindChirho::ShowCharChirho, _) => true,
                        // ShowFloatChirho: only the prim handles decimal formatting correctly
                        _ => false,
                    };
                if needs_fallback_chirho {
                    let s_chirho = self.show_value_as_string_chirho(val_chirho);
                    return Ok(ValueChirho::StringChirho(s_chirho));
                }
            }
        }

        // Force primitive operands before dispatch so string/list thunks used by
        // desugared string equality reach string-capable primops as StringChirho values.
        let mut resolved_args_chirho: Vec<ValueChirho> = args_chirho
            .iter()
            .cloned()
            .map(|v_chirho| self.force_to_prim_chirho(v_chirho))
            .collect();

        let is_string_compare_primop_chirho = matches!(
            op_chirho,
            PrimOpKindChirho::EqStrChirho
                | PrimOpKindChirho::LtStrChirho
                | PrimOpKindChirho::EqIntChirho
                | PrimOpKindChirho::NeIntChirho
                | PrimOpKindChirho::LtIntChirho
                | PrimOpKindChirho::LeIntChirho
                | PrimOpKindChirho::GtIntChirho
                | PrimOpKindChirho::GeIntChirho
        );
        if is_string_compare_primop_chirho && resolved_args_chirho.len() == 2 {
            let left_str_chirho =
                self.try_resolve_value_to_string_chirho(&resolved_args_chirho[0])?;
            let right_str_chirho =
                self.try_resolve_value_to_string_chirho(&resolved_args_chirho[1])?;
            if let (Some(left_str_chirho), Some(right_str_chirho)) =
                (left_str_chirho, right_str_chirho)
            {
                resolved_args_chirho[0] = ValueChirho::StringChirho(left_str_chirho);
                resolved_args_chirho[1] = ValueChirho::StringChirho(right_str_chirho);
            }
        }

        match resolved_args_chirho.len() {
            2 => apply_prim_binop_chirho(
                op_chirho,
                &resolved_args_chirho[0],
                &resolved_args_chirho[1],
            )
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
                            Ok(forced_chirho) => self.resolve_heap_value_chirho(
                                &ValueChirho::HeapPtrChirho(forced_chirho),
                            ),
                            Err(_) => {
                                self.resolve_heap_value_chirho(&closure_chirho.payload_chirho[0])
                            }
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

    fn alloc_char_list_from_string_chirho(&mut self, s_chirho: &str) -> HeapAddrChirho {
        let nil_tag_chirho = self.lookup_con_tag_chirho("[]", 0);
        let cons_tag_chirho = self.lookup_con_tag_chirho(":", 1);
        let nil_chirho = ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
        let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_chirho);
        for ch_chirho in s_chirho.chars().rev() {
            let cons_chirho = ClosureChirho::con_chirho(
                cons_tag_chirho,
                ":",
                vec![
                    ValueChirho::CharChirho(ch_chirho),
                    ValueChirho::HeapPtrChirho(tail_addr_chirho),
                ],
            );
            tail_addr_chirho = self.heap_chirho.alloc_chirho(cons_chirho);
        }
        tail_addr_chirho
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
                            self.resolve_heap_value_chirho(&closure_chirho.payload_chirho[0])
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
                if *b_chirho {
                    "True".to_string()
                } else {
                    "False".to_string()
                }
            }
            ValueChirho::MapChirho(pairs_chirho) => {
                let pairs_clone_chirho = pairs_chirho.clone();
                if pairs_clone_chirho.is_empty() {
                    "fromList []".to_string()
                } else {
                    let entries_chirho: Vec<String> = pairs_clone_chirho
                        .into_iter()
                        .map(|(k_chirho, v_chirho)| {
                            let k_str_chirho = self.show_value_as_string_chirho(&k_chirho);
                            let v_str_chirho = self.show_value_as_string_chirho(&v_chirho);
                            format!("({},{})", k_str_chirho, v_str_chirho)
                        })
                        .collect();
                    format!("fromList [{}]", entries_chirho.join(","))
                }
            }
            ValueChirho::SetChirho(elems_chirho) => {
                let elems_clone_chirho = elems_chirho.clone();
                if elems_clone_chirho.is_empty() {
                    "fromList []".to_string()
                } else {
                    let items_chirho: Vec<String> = elems_clone_chirho
                        .iter()
                        .map(|e_chirho| self.show_value_as_string_chirho(e_chirho))
                        .collect();
                    format!("fromList [{}]", items_chirho.join(","))
                }
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
                            let inner_str_chirho = self.show_value_as_string_chirho(inner_chirho);
                            // Wrap in parens if inner contains spaces (compound value)
                            if inner_str_chirho.contains(' ') {
                                format!("Just ({})", inner_str_chirho)
                            } else {
                                format!("Just {}", inner_str_chirho)
                            }
                        } else {
                            "Just ?".to_string()
                        }
                    }
                    "Left" => {
                        if let Some(inner_chirho) = closure_chirho.payload_chirho.first() {
                            let inner_str_chirho = self.show_value_as_string_chirho(inner_chirho);
                            if inner_str_chirho.contains(' ') {
                                format!("Left ({})", inner_str_chirho)
                            } else {
                                format!("Left {}", inner_str_chirho)
                            }
                        } else {
                            "Left ?".to_string()
                        }
                    }
                    "Right" => {
                        if let Some(inner_chirho) = closure_chirho.payload_chirho.first() {
                            let inner_str_chirho = self.show_value_as_string_chirho(inner_chirho);
                            if inner_str_chirho.contains(' ') {
                                format!("Right ({})", inner_str_chirho)
                            } else {
                                format!("Right {}", inner_str_chirho)
                            }
                        } else {
                            "Right ?".to_string()
                        }
                    }
                    "LT" => "LT".to_string(),
                    "EQ" => "EQ".to_string(),
                    "GT" => "GT".to_string(),
                    name_chirho
                        if name_chirho.starts_with("$tuple")
                            || name_chirho == "(,)"
                            || name_chirho == "(,,)"
                            || name_chirho == "(,,,)"
                            || name_chirho == "(,,,,)" =>
                    {
                        let parts_chirho: Vec<String> = closure_chirho
                            .payload_chirho
                            .iter()
                            .map(|v_chirho| self.show_value_as_string_chirho(v_chirho))
                            .collect();
                        format!("({})", parts_chirho.join(","))
                    }
                    "[]" => "[]".to_string(),
                    ":" => {
                        // Show as a list — first collect raw values to detect
                        // all-Char lists (i.e., Strings) for pretty display.
                        let mut raw_values_chirho: Vec<ValueChirho> = Vec::new();
                        let mut cur_chirho = forced_addr_chirho;
                        let mut all_char_chirho = true;
                        loop {
                            let cl_chirho = self.heap_chirho.read_chirho(cur_chirho).clone();
                            let cn_chirho = &cl_chirho.info_chirho.name_chirho;
                            if cn_chirho == "[]" {
                                break;
                            }
                            if cn_chirho == ":" && cl_chirho.payload_chirho.len() >= 2 {
                                let hd_chirho = &cl_chirho.payload_chirho[0];
                                // Check if element is a Char
                                match hd_chirho {
                                    ValueChirho::CharChirho(_) => {}
                                    _ => {
                                        all_char_chirho = false;
                                    }
                                }
                                raw_values_chirho.push(hd_chirho.clone());
                                match &cl_chirho.payload_chirho[1] {
                                    ValueChirho::HeapPtrChirho(t_chirho) => {
                                        // Force thunks in the tail to ensure
                                        // lazy list spines are fully evaluated.
                                        match self.force_addr_to_whnf_chirho(*t_chirho) {
                                            Ok(resolved_chirho) => {
                                                cur_chirho = resolved_chirho;
                                            }
                                            Err(_) => break,
                                        }
                                    }
                                    _ => break,
                                }
                            } else {
                                break;
                            }
                        }
                        // Display all-Char lists as strings
                        if all_char_chirho && !raw_values_chirho.is_empty() {
                            let s_chirho: String = raw_values_chirho
                                .iter()
                                .map(|v_chirho| {
                                    if let ValueChirho::CharChirho(c_chirho) = v_chirho {
                                        *c_chirho
                                    } else {
                                        '?'
                                    }
                                })
                                .collect();
                            format!("\"{}\"", s_chirho)
                        } else {
                            let elements_chirho: Vec<String> = raw_values_chirho
                                .iter()
                                .map(|v_chirho| self.show_value_as_string_chirho(v_chirho))
                                .collect();
                            format!("[{}]", elements_chirho.join(","))
                        }
                    }
                    _ => {
                        // Generic constructor: "Con field1 field2 ..."
                        if closure_chirho.payload_chirho.is_empty() {
                            name_chirho.clone()
                        } else {
                            let fields_chirho: Vec<String> = closure_chirho
                                .payload_chirho
                                .iter()
                                .map(|f_chirho| {
                                    let s_chirho = self.show_value_as_string_chirho(f_chirho);
                                    // Wrap compound values in parens
                                    if s_chirho.contains(' ')
                                        && !s_chirho.starts_with('(')
                                        && !s_chirho.starts_with('[')
                                        && !s_chirho.starts_with('"')
                                    {
                                        format!("({})", s_chirho)
                                    } else {
                                        s_chirho
                                    }
                                })
                                .collect();
                            format!("{} {}", name_chirho, fields_chirho.join(" "))
                        }
                    }
                }
            }
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
        let tag_chirho = self
            .heap_chirho
            .read_chirho(resolved_chirho)
            .info_chirho
            .tag_chirho;
        match tag_chirho {
            InfoTagChirho::ConChirho => Ok(resolved_chirho),
            InfoTagChirho::ThunkChirho | InfoTagChirho::PapChirho | InfoTagChirho::FunChirho => {
                let saved_stack_chirho =
                    std::mem::replace(&mut self.stack_chirho, StackChirho::new_chirho());
                let saved_regs_chirho = std::mem::take(&mut self.arg_regs_chirho);
                let entry_chirho = self.code_table_chirho.len() as u32;
                self.code_table_chirho
                    .push(CodeChirho::EnterChirho(resolved_chirho));
                let _result_chirho = self.run_chirho(entry_chirho)?;
                self.stack_chirho = saved_stack_chirho;
                self.arg_regs_chirho = saved_regs_chirho;
                Ok(self.heap_chirho.follow_ind_chirho(addr_chirho))
            }
            InfoTagChirho::IndChirho => {
                // Indirections should have been resolved by follow_ind_chirho above.
                // If we still see one, follow it again.
                Ok(self.heap_chirho.follow_ind_chirho(resolved_chirho))
            }
            InfoTagChirho::BlackholeChirho => Err(EvalErrorChirho::BlackholeChirho {
                addr_chirho: resolved_chirho,
            }),
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
                            let head_resolved_chirho =
                                self.force_addr_to_whnf_chirho(*head_addr_chirho)?;
                            let head_closure_chirho =
                                self.heap_chirho.read_chirho(head_resolved_chirho).clone();
                            if let Some(ValueChirho::CharChirho(ch_chirho)) =
                                head_closure_chirho.payload_chirho.first()
                            {
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

    fn try_resolve_value_to_string_chirho(
        &mut self,
        val_chirho: &ValueChirho,
    ) -> Result<Option<String>, EvalErrorChirho> {
        match val_chirho {
            ValueChirho::StringChirho(s_chirho) => Ok(Some(s_chirho.clone())),
            ValueChirho::HeapPtrChirho(addr_chirho) => {
                let resolved_addr_chirho = self.force_addr_to_whnf_chirho(*addr_chirho)?;
                let closure_chirho = self.heap_chirho.read_chirho(resolved_addr_chirho).clone();
                let is_string_like_chirho = matches!(
                    closure_chirho.info_chirho.name_chirho.as_str(),
                    "Addr#" | ":" | "[]"
                ) || matches!(
                    closure_chirho.payload_chirho.first(),
                    Some(ValueChirho::StringChirho(_))
                );
                if is_string_like_chirho {
                    self.resolve_string_arg_chirho(resolved_addr_chirho)
                        .map(Some)
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
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
                    ClosureChirho::con_chirho(self.lookup_con_tag_chirho("True", 0), "True", vec![])
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
                        cons_tag_chirho,
                        ":",
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
            ValueChirho::MapChirho(pairs_chirho) => {
                // Store the entire map in the payload of a "MapBox" constructor
                // so it can be retrieved by extract_map_pairs_chirho later.
                ClosureChirho::con_chirho(
                    DataConTagChirho(0),
                    "MapBox",
                    vec![ValueChirho::MapChirho(pairs_chirho.clone())],
                )
            }
            ValueChirho::SetChirho(elems_chirho) => {
                // Store the entire set in the payload of a "SetBox" constructor
                // so it can be retrieved by extract_set_elems_chirho later.
                ClosureChirho::con_chirho(
                    DataConTagChirho(0),
                    "SetBox",
                    vec![ValueChirho::SetChirho(elems_chirho.clone())],
                )
            }
        }
    }

    // ── Data.Map helper methods ──────────────────────────────────────────

    /// Extract sorted key-value pairs from a `ValueChirho`.
    /// Handles `MapChirho` directly, and also resolves heap pointers
    /// so that primop-wrapped maps can be passed through.
    fn extract_map_pairs_chirho(
        &mut self,
        val_chirho: ValueChirho,
    ) -> Vec<(ValueChirho, ValueChirho)> {
        match val_chirho {
            ValueChirho::MapChirho(pairs_chirho) => pairs_chirho,
            ValueChirho::HeapPtrChirho(addr_chirho) => {
                // Force the closure to WHNF first so thunks are evaluated
                let whnf_addr_chirho = match self.force_addr_to_whnf_chirho(addr_chirho) {
                    Ok(a_chirho) => a_chirho,
                    Err(_) => return vec![],
                };
                let resolved_chirho = self.heap_chirho.follow_ind_chirho(whnf_addr_chirho);
                let closure_chirho = self.heap_chirho.read_chirho(resolved_chirho).clone();
                let name_chirho = &closure_chirho.info_chirho.name_chirho;
                // MapBox: a boxed MapChirho stored as payload[0]
                if name_chirho == "MapBox" {
                    if let Some(ValueChirho::MapChirho(pairs_chirho)) =
                        closure_chirho.payload_chirho.first()
                    {
                        return pairs_chirho.clone();
                    }
                }
                // MapEmpty: empty map (was used as placeholder)
                if name_chirho == "MapEmpty" && closure_chirho.payload_chirho.is_empty() {
                    return vec![];
                }
                // Check any payload slot for a MapChirho (fallback)
                for payload_val_chirho in &closure_chirho.payload_chirho {
                    if let ValueChirho::MapChirho(pairs_chirho) = payload_val_chirho {
                        return pairs_chirho.clone();
                    }
                }
                // Single-field wrapper — recurse
                if closure_chirho.payload_chirho.len() == 1 {
                    return self.extract_map_pairs_chirho(closure_chirho.payload_chirho[0].clone());
                }
                vec![]
            }
            _ => vec![],
        }
    }

    // ── Data.Set helper methods ──────────────────────────────────────────

    /// Extract sorted elements from a `ValueChirho::SetChirho` or a boxed heap
    /// pointer wrapping one (via "SetBox" constructor).
    fn extract_set_elems_chirho(&mut self, val_chirho: ValueChirho) -> Vec<ValueChirho> {
        match val_chirho {
            ValueChirho::SetChirho(elems_chirho) => elems_chirho,
            ValueChirho::HeapPtrChirho(addr_chirho) => {
                let whnf_addr_chirho = match self.force_addr_to_whnf_chirho(addr_chirho) {
                    Ok(a_chirho) => a_chirho,
                    Err(_) => return vec![],
                };
                let resolved_chirho = self.heap_chirho.follow_ind_chirho(whnf_addr_chirho);
                let closure_chirho = self.heap_chirho.read_chirho(resolved_chirho).clone();
                let name_chirho = &closure_chirho.info_chirho.name_chirho;
                if name_chirho == "SetBox" {
                    if let Some(ValueChirho::SetChirho(elems_chirho)) =
                        closure_chirho.payload_chirho.first()
                    {
                        return elems_chirho.clone();
                    }
                }
                // Check any payload slot for a SetChirho (fallback)
                for payload_val_chirho in &closure_chirho.payload_chirho {
                    if let ValueChirho::SetChirho(elems_chirho) = payload_val_chirho {
                        return elems_chirho.clone();
                    }
                }
                // Single-field wrapper — recurse
                if closure_chirho.payload_chirho.len() == 1 {
                    return self.extract_set_elems_chirho(closure_chirho.payload_chirho[0].clone());
                }
                vec![]
            }
            _ => vec![],
        }
    }

    /// Collect elements from a heap list of values, building a sorted deduplicated set.
    fn collect_set_from_list_chirho(&mut self, list_val_chirho: ValueChirho) -> Vec<ValueChirho> {
        let mut elems_chirho: Vec<ValueChirho> = Vec::new();
        let start_addr_chirho = match list_val_chirho {
            ValueChirho::HeapPtrChirho(addr_chirho) => addr_chirho,
            _ => return elems_chirho,
        };
        let mut current_chirho = start_addr_chirho;
        loop {
            let resolved_chirho = match self.force_addr_to_whnf_chirho(current_chirho) {
                Ok(a_chirho) => a_chirho,
                Err(_) => break,
            };
            let closure_chirho = self.heap_chirho.read_chirho(resolved_chirho).clone();
            let name_chirho = closure_chirho.info_chirho.name_chirho.clone();
            if name_chirho == "[]" {
                break;
            } else if name_chirho == ":" && closure_chirho.payload_chirho.len() >= 2 {
                let raw_e_chirho = closure_chirho.payload_chirho[0].clone();
                let e_chirho = self.force_to_prim_chirho(raw_e_chirho);
                // Insert in sorted order, dedup
                match elems_chirho
                    .binary_search_by(|ek_chirho| compare_values_chirho(ek_chirho, &e_chirho))
                {
                    Ok(_) => {} // duplicate — skip
                    Err(pos_chirho) => {
                        elems_chirho.insert(pos_chirho, e_chirho);
                    }
                }
                // Advance to tail
                match &closure_chirho.payload_chirho[1] {
                    ValueChirho::HeapPtrChirho(tail_addr_chirho) => {
                        current_chirho = *tail_addr_chirho;
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        elems_chirho
    }

    /// Force a `ValueChirho` to a primitive (unboxed) value.
    /// Heap pointers are fully evaluated via `force_addr_to_whnf_chirho`, then
    /// the result is unboxed through indirections and wrapper constructors.
    pub fn force_to_prim_chirho(&mut self, val_chirho: ValueChirho) -> ValueChirho {
        match val_chirho {
            ValueChirho::HeapPtrChirho(addr_chirho) => {
                // Force the thunk to WHNF first (evaluates pending computations)
                let whnf_addr_chirho = match self.force_addr_to_whnf_chirho(addr_chirho) {
                    Ok(a_chirho) => a_chirho,
                    Err(_) => return ValueChirho::HeapPtrChirho(addr_chirho),
                };
                // Now resolve through indirections and unbox wrapper constructors
                let resolved_chirho =
                    self.resolve_heap_value_chirho(&ValueChirho::HeapPtrChirho(whnf_addr_chirho));
                match resolved_chirho {
                    ValueChirho::HeapPtrChirho(inner_addr_chirho) => {
                        let inner_addr_chirho =
                            self.heap_chirho.follow_ind_chirho(inner_addr_chirho);
                        if inner_addr_chirho == whnf_addr_chirho {
                            ValueChirho::HeapPtrChirho(inner_addr_chirho)
                        } else {
                            self.force_to_prim_chirho(ValueChirho::HeapPtrChirho(inner_addr_chirho))
                        }
                    }
                    other_chirho => other_chirho,
                }
            }
            other_chirho => other_chirho,
        }
    }

    /// Build a heap list of `ValueChirho` elements.
    fn build_value_list_chirho(&mut self, values_chirho: Vec<ValueChirho>) -> ValueChirho {
        let nil_tag_chirho = self.lookup_con_tag_chirho("[]", 0);
        let cons_tag_chirho = self.lookup_con_tag_chirho(":", 1);
        let nil_closure_chirho = ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
        let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);
        for val_chirho in values_chirho.into_iter().rev() {
            let cons_closure_chirho = ClosureChirho::con_chirho(
                cons_tag_chirho,
                ":",
                vec![val_chirho, ValueChirho::HeapPtrChirho(tail_addr_chirho)],
            );
            tail_addr_chirho = self.heap_chirho.alloc_chirho(cons_closure_chirho);
        }
        ValueChirho::HeapPtrChirho(tail_addr_chirho)
    }

    /// Build a heap list of `(k, v)` tuples from sorted pairs.
    fn build_pair_list_chirho(
        &mut self,
        pairs_chirho: Vec<(ValueChirho, ValueChirho)>,
    ) -> ValueChirho {
        let nil_tag_chirho = self.lookup_con_tag_chirho("[]", 0);
        let cons_tag_chirho = self.lookup_con_tag_chirho(":", 1);
        let tuple2_tag_chirho = self.lookup_con_tag_chirho("(,)", 0);
        let nil_closure_chirho = ClosureChirho::con_chirho(nil_tag_chirho, "[]", vec![]);
        let mut tail_addr_chirho = self.heap_chirho.alloc_chirho(nil_closure_chirho);
        for (k_chirho, v_chirho) in pairs_chirho.into_iter().rev() {
            let tup_closure_chirho =
                ClosureChirho::con_chirho(tuple2_tag_chirho, "(,)", vec![k_chirho, v_chirho]);
            let tup_addr_chirho = self.heap_chirho.alloc_chirho(tup_closure_chirho);
            let cons_closure_chirho = ClosureChirho::con_chirho(
                cons_tag_chirho,
                ":",
                vec![
                    ValueChirho::HeapPtrChirho(tup_addr_chirho),
                    ValueChirho::HeapPtrChirho(tail_addr_chirho),
                ],
            );
            tail_addr_chirho = self.heap_chirho.alloc_chirho(cons_closure_chirho);
        }
        ValueChirho::HeapPtrChirho(tail_addr_chirho)
    }

    /// Collect (k, v) pairs from a heap list of tuples, building a sorted map.
    fn collect_map_from_list_chirho(
        &mut self,
        list_val_chirho: ValueChirho,
    ) -> Vec<(ValueChirho, ValueChirho)> {
        let mut pairs_chirho: Vec<(ValueChirho, ValueChirho)> = Vec::new();
        let start_addr_chirho = match list_val_chirho {
            ValueChirho::HeapPtrChirho(addr_chirho) => addr_chirho,
            _ => return pairs_chirho,
        };
        let mut current_chirho = start_addr_chirho;
        loop {
            let resolved_chirho = match self.force_addr_to_whnf_chirho(current_chirho) {
                Ok(a_chirho) => a_chirho,
                Err(_) => break,
            };
            let closure_chirho = self.heap_chirho.read_chirho(resolved_chirho).clone();
            let name_chirho = closure_chirho.info_chirho.name_chirho.clone();
            if name_chirho == "[]" {
                break;
            } else if name_chirho == ":" && closure_chirho.payload_chirho.len() >= 2 {
                // Head is a tuple: extract (k, v)
                let head_val_chirho = closure_chirho.payload_chirho[0].clone();
                if let ValueChirho::HeapPtrChirho(tup_addr_chirho) = head_val_chirho {
                    let tup_resolved_chirho = match self.force_addr_to_whnf_chirho(tup_addr_chirho)
                    {
                        Ok(a_chirho) => a_chirho,
                        Err(_) => break,
                    };
                    let tup_closure_chirho =
                        self.heap_chirho.read_chirho(tup_resolved_chirho).clone();
                    if tup_closure_chirho.payload_chirho.len() >= 2 {
                        let raw_k_chirho = tup_closure_chirho.payload_chirho[0].clone();
                        let raw_v_chirho = tup_closure_chirho.payload_chirho[1].clone();
                        let k_chirho = self.force_to_prim_chirho(raw_k_chirho);
                        let v_chirho = self.force_to_prim_chirho(raw_v_chirho);
                        // Insert in sorted order
                        match pairs_chirho.binary_search_by(|(ek_chirho, _)| {
                            compare_values_chirho(ek_chirho, &k_chirho)
                        }) {
                            Ok(pos_chirho) => {
                                pairs_chirho[pos_chirho].1 = v_chirho;
                            }
                            Err(pos_chirho) => {
                                pairs_chirho.insert(pos_chirho, (k_chirho, v_chirho));
                            }
                        }
                    }
                }
                // Advance to tail
                match &closure_chirho.payload_chirho[1] {
                    ValueChirho::HeapPtrChirho(tail_addr_chirho) => {
                        current_chirho = *tail_addr_chirho;
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        pairs_chirho
    }

    /// Apply a function value (heap closure) to a single argument.
    /// Saves and restores machine state for nested evaluation.
    fn apply_fn_to_value_chirho(
        &mut self,
        fun_val_chirho: ValueChirho,
        arg_val_chirho: ValueChirho,
    ) -> Result<ValueChirho, EvalErrorChirho> {
        let fun_addr_chirho = match fun_val_chirho {
            ValueChirho::HeapPtrChirho(a_chirho) => a_chirho,
            // Non-closure (identity-like) — return the arg
            _ => return Ok(arg_val_chirho),
        };
        // Save machine state
        let saved_stack_chirho =
            std::mem::replace(&mut self.stack_chirho, StackChirho::new_chirho());
        let saved_regs_chirho = std::mem::take(&mut self.arg_regs_chirho);
        // Box the argument so it can be placed on the heap if needed
        let arg_for_stack_chirho = match &arg_val_chirho {
            ValueChirho::HeapPtrChirho(_) => arg_val_chirho.clone(),
            other_chirho => {
                let boxed_chirho = self.box_literal_chirho(other_chirho);
                let boxed_addr_chirho = self.heap_chirho.alloc_chirho(boxed_chirho);
                ValueChirho::HeapPtrChirho(boxed_addr_chirho)
            }
        };
        // Push Apply frame and enter function
        self.stack_chirho.push_chirho(FrameChirho::ApplyChirho {
            args_chirho: vec![arg_for_stack_chirho],
        });
        let entry_chirho = self.code_table_chirho.len() as u32;
        self.code_table_chirho
            .push(CodeChirho::EnterChirho(fun_addr_chirho));
        let result_chirho = self.run_chirho(entry_chirho);
        // Restore machine state
        self.stack_chirho = saved_stack_chirho;
        self.arg_regs_chirho = saved_regs_chirho;
        let result_val_chirho = result_chirho?;
        // Unbox result if it's a heap pointer to a primitive
        Ok(self.force_to_prim_chirho(result_val_chirho))
    }
}

/// Internal action after returning a value.
enum ReturnActionChirho {
    /// Evaluation complete — return this value.
    DoneChirho(ValueChirho),
    /// Continue evaluation at this code table index.
    ContinueChirho(u32),
}

/// Compare two `ValueChirho` values for use in sorted map operations.
/// Supports Int, Float, Char, Bool, and String keys.
/// Returns `std::cmp::Ordering::Equal` for incomparable pairs.
fn compare_values_chirho(a_chirho: &ValueChirho, b_chirho: &ValueChirho) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    match (a_chirho, b_chirho) {
        (ValueChirho::IntChirho(a_n_chirho), ValueChirho::IntChirho(b_n_chirho)) => {
            a_n_chirho.cmp(b_n_chirho)
        }
        (ValueChirho::FloatChirho(a_f_chirho), ValueChirho::FloatChirho(b_f_chirho)) => a_f_chirho
            .partial_cmp(b_f_chirho)
            .unwrap_or(Ordering::Equal),
        (ValueChirho::CharChirho(a_c_chirho), ValueChirho::CharChirho(b_c_chirho)) => {
            a_c_chirho.cmp(b_c_chirho)
        }
        (ValueChirho::BoolChirho(a_b_chirho), ValueChirho::BoolChirho(b_b_chirho)) => {
            a_b_chirho.cmp(b_b_chirho)
        }
        (ValueChirho::StringChirho(a_s_chirho), ValueChirho::StringChirho(b_s_chirho)) => {
            a_s_chirho.cmp(b_s_chirho)
        }
        // Mixed Int/Float comparison
        (ValueChirho::IntChirho(a_n_chirho), ValueChirho::FloatChirho(b_f_chirho)) => (*a_n_chirho
            as f64)
            .partial_cmp(b_f_chirho)
            .unwrap_or(Ordering::Equal),
        (ValueChirho::FloatChirho(a_f_chirho), ValueChirho::IntChirho(b_n_chirho)) => a_f_chirho
            .partial_cmp(&(*b_n_chirho as f64))
            .unwrap_or(Ordering::Equal),
        // Fallback: compare by debug representation
        _ => format!("{:?}", a_chirho).cmp(&format!("{:?}", b_chirho)),
    }
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::value_chirho::CodePtrChirho;

    /// Helper: build a machine, run it from entry 0, return the result.
    fn run_code_chirho(code_chirho: Vec<CodeChirho>) -> Result<ValueChirho, EvalErrorChirho> {
        let mut machine_chirho = MachineChirho::new_chirho(code_chirho);
        machine_chirho.run_chirho(0)
    }

    #[test]
    fn literal_return_chirho() {
        // Code: just return Int 42
        let result_chirho =
            run_code_chirho(vec![CodeChirho::LitChirho(ValueChirho::IntChirho(42))]);
        assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
    }

    #[test]
    fn primop_add_chirho() {
        // Code: 3 + 4
        let result_chirho = run_code_chirho(vec![CodeChirho::PrimChirho {
            op_chirho: PrimOpKindChirho::AddIntChirho,
            args_chirho: vec![
                ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(3)),
                ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(4)),
            ],
        }]);
        assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(7));
    }

    #[test]
    fn primop_mul_chirho() {
        let result_chirho = run_code_chirho(vec![CodeChirho::PrimChirho {
            op_chirho: PrimOpKindChirho::MulIntChirho,
            args_chirho: vec![
                ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(6)),
                ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(7)),
            ],
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
        let mut machine_chirho = MachineChirho::new_chirho(vec![CodeChirho::ConAppChirho {
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
        let true_addr_chirho = machine_chirho
            .heap_chirho
            .alloc_chirho(ClosureChirho::con_chirho(
                DataConTagChirho(0),
                "True",
                vec![],
            ));

        // Replace code[0] with a Case
        machine_chirho.code_table_chirho[0] = CodeChirho::CaseChirho {
            scrutinee_chirho: ArgSourceChirho::StaticChirho(ValueChirho::HeapPtrChirho(
                true_addr_chirho,
            )),
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
        let addr_chirho = machine_chirho
            .heap_chirho
            .alloc_chirho(ClosureChirho::con_chirho(
                DataConTagChirho(5),
                "Unknown",
                vec![],
            ));

        machine_chirho.code_table_chirho[0] = CodeChirho::CaseChirho {
            scrutinee_chirho: ArgSourceChirho::StaticChirho(ValueChirho::HeapPtrChirho(
                addr_chirho,
            )),
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
                args_chirho: vec![
                    ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(3)),
                    ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(4)),
                ],
            },
        ]);

        let thunk_addr_chirho =
            machine_chirho
                .heap_chirho
                .alloc_chirho(ClosureChirho::thunk_chirho(
                    CodePtrChirho(1),
                    "myThunk",
                    vec![],
                ));

        // Entry: Enter the thunk
        machine_chirho.code_table_chirho[0] = CodeChirho::EnterChirho(thunk_addr_chirho);

        let result_chirho = machine_chirho.run_chirho(0).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(7));

        // Verify the thunk was updated (now an indirection)
        let updated_chirho = machine_chirho.heap_chirho.read_chirho(thunk_addr_chirho);
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

        let thunk_addr_chirho =
            machine_chirho
                .heap_chirho
                .alloc_chirho(ClosureChirho::thunk_chirho(
                    CodePtrChirho(1),
                    "loop",
                    vec![],
                ));

        // code[0] = Enter thunk
        machine_chirho.code_table_chirho[0] = CodeChirho::EnterChirho(thunk_addr_chirho);
        // code[1] = Enter same thunk (self-reference)
        machine_chirho.code_table_chirho[1] = CodeChirho::EnterChirho(thunk_addr_chirho);

        let result_chirho = machine_chirho.run_chirho(0);
        assert!(matches!(
            result_chirho,
            Err(EvalErrorChirho::BlackholeChirho { .. })
        ));
    }

    #[test]
    fn return_io_preserves_lazy_payload_chirho() {
        let mut machine_chirho = MachineChirho::new_chirho(vec![
            CodeChirho::LitChirho(ValueChirho::IntChirho(0)),
            CodeChirho::LitChirho(ValueChirho::IntChirho(0)),
        ]);
        let thunk_addr_chirho =
            machine_chirho
                .heap_chirho
                .alloc_chirho(ClosureChirho::thunk_chirho(
                    CodePtrChirho(1),
                    "return_payload_chirho",
                    vec![],
                ));
        machine_chirho.code_table_chirho[0] = CodeChirho::PrimChirho {
            op_chirho: PrimOpKindChirho::ReturnIOChirho,
            args_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::HeapPtrChirho(
                thunk_addr_chirho,
            ))],
        };
        machine_chirho.code_table_chirho[1] = CodeChirho::EnterChirho(thunk_addr_chirho);
        assert_eq!(
            machine_chirho.run_chirho(0),
            Ok(ValueChirho::HeapPtrChirho(thunk_addr_chirho))
        );
        assert_eq!(
            machine_chirho
                .heap_chirho
                .read_chirho(thunk_addr_chirho)
                .info_chirho
                .tag_chirho,
            InfoTagChirho::ThunkChirho
        );
    }

    #[test]
    fn step_limit_chirho() {
        // Infinite loop via step limit
        let mut machine_chirho =
            MachineChirho::new_chirho(vec![CodeChirho::LitChirho(ValueChirho::IntChirho(0))]);

        let thunk_addr_chirho =
            machine_chirho
                .heap_chirho
                .alloc_chirho(ClosureChirho::thunk_chirho(
                    CodePtrChirho(0),
                    "loop",
                    vec![],
                ));

        machine_chirho.code_table_chirho[0] = CodeChirho::EnterChirho(thunk_addr_chirho);

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
                args_chirho: vec![
                    ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(41)),
                    ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(1)),
                ],
            },
        ]);

        let fun_addr_chirho = machine_chirho
            .heap_chirho
            .alloc_chirho(ClosureChirho::fun_chirho(1, CodePtrChirho(1), "f", vec![]));

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
                args_chirho: vec![
                    ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(10)),
                    ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(32)),
                ],
            },
        ]);

        let fun_addr_chirho = machine_chirho
            .heap_chirho
            .alloc_chirho(ClosureChirho::fun_chirho(
                2,
                CodePtrChirho(1),
                "add",
                vec![],
            ));

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
            args_chirho: vec![
                ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(42)),
                ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(0)),
            ],
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

    #[test]
    fn maybe_gc_keeps_ioref_and_tvar_payloads_alive_chirho() {
        let mut machine_chirho =
            MachineChirho::new_chirho(vec![]).with_gc_config_chirho(GcConfigChirho {
                alloc_threshold_chirho: 1,
                min_heap_size_chirho: 0,
            });

        let ioref_payload_addr_chirho =
            machine_chirho
                .heap_chirho
                .alloc_chirho(ClosureChirho::con_chirho(
                    DataConTagChirho(0),
                    "I#",
                    vec![ValueChirho::IntChirho(11)],
                ));
        let tvar_payload_addr_chirho =
            machine_chirho
                .heap_chirho
                .alloc_chirho(ClosureChirho::con_chirho(
                    DataConTagChirho(0),
                    "I#",
                    vec![ValueChirho::IntChirho(29)],
                ));
        let dead_addr_chirho = machine_chirho
            .heap_chirho
            .alloc_chirho(ClosureChirho::con_chirho(
                DataConTagChirho(0),
                "I#",
                vec![ValueChirho::IntChirho(99)],
            ));

        machine_chirho
            .iorefs_chirho
            .insert(0, ValueChirho::HeapPtrChirho(ioref_payload_addr_chirho));
        machine_chirho
            .tvars_chirho
            .insert(0, ValueChirho::HeapPtrChirho(tvar_payload_addr_chirho));

        machine_chirho.maybe_gc_chirho();

        let ioref_payload_chirho = machine_chirho
            .heap_chirho
            .read_chirho(ioref_payload_addr_chirho);
        assert_eq!(ioref_payload_chirho.info_chirho.name_chirho, "I#");
        assert_eq!(
            ioref_payload_chirho.payload_chirho,
            vec![ValueChirho::IntChirho(11)]
        );

        let tvar_payload_chirho = machine_chirho
            .heap_chirho
            .read_chirho(tvar_payload_addr_chirho);
        assert_eq!(tvar_payload_chirho.info_chirho.name_chirho, "I#");
        assert_eq!(
            tvar_payload_chirho.payload_chirho,
            vec![ValueChirho::IntChirho(29)]
        );

        let dead_closure_chirho = machine_chirho.heap_chirho.read_chirho(dead_addr_chirho);
        assert_eq!(dead_closure_chirho.info_chirho.name_chirho, "$DEAD");
    }

    #[test]
    fn recursive_group_is_rooted_before_gc_chirho() {
        let bindings_chirho = vec![
            RecBindingSpecChirho {
                arity_chirho: None,
                code_ptr_chirho: 1,
                name_chirho: "leftChirho".to_string(),
                captures_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(7))],
                dest_reg_chirho: 0,
            },
            RecBindingSpecChirho {
                arity_chirho: None,
                code_ptr_chirho: 1,
                name_chirho: "rightChirho".to_string(),
                captures_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(11))],
                dest_reg_chirho: 1,
            },
        ];
        let mut machine_chirho = MachineChirho::new_chirho(vec![
            CodeChirho::StoreAllocRecGroupChirho {
                bindings_chirho,
                body_chirho: 1,
            },
            CodeChirho::LitChirho(ValueChirho::IntChirho(42)),
        ])
        .with_gc_config_chirho(GcConfigChirho {
            alloc_threshold_chirho: 1,
            min_heap_size_chirho: 0,
        });

        assert_eq!(
            machine_chirho.run_chirho(0).unwrap(),
            ValueChirho::IntChirho(42)
        );

        let left_addr_chirho = HeapAddrChirho(0);
        let right_addr_chirho = HeapAddrChirho(1);
        assert_eq!(
            machine_chirho
                .heap_chirho
                .read_chirho(left_addr_chirho)
                .payload_chirho,
            vec![
                ValueChirho::IntChirho(7),
                ValueChirho::HeapPtrChirho(left_addr_chirho),
                ValueChirho::HeapPtrChirho(right_addr_chirho),
            ]
        );
        assert_eq!(
            machine_chirho
                .heap_chirho
                .read_chirho(right_addr_chirho)
                .payload_chirho,
            vec![
                ValueChirho::IntChirho(11),
                ValueChirho::HeapPtrChirho(left_addr_chirho),
                ValueChirho::HeapPtrChirho(right_addr_chirho),
            ]
        );
    }

    #[test]
    fn enter_pap_flows_through_primop_frames_chirho() {
        let mut machine_chirho = MachineChirho::new_chirho(vec![CodeChirho::PrimChirho {
            op_chirho: PrimOpKindChirho::AddIntChirho,
            args_chirho: vec![
                ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(1)),
                ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(2)),
            ],
        }]);

        let fun_addr_chirho = machine_chirho
            .heap_chirho
            .alloc_chirho(ClosureChirho::fun_chirho(2, CodePtrChirho(0), "f", vec![]));
        let pap_addr_chirho = machine_chirho
            .heap_chirho
            .alloc_chirho(ClosureChirho::pap_chirho(
                1,
                fun_addr_chirho,
                vec![ValueChirho::IntChirho(10)],
            ));
        let pap_closure_chirho = machine_chirho
            .heap_chirho
            .read_chirho(pap_addr_chirho)
            .clone();

        machine_chirho
            .stack_chirho
            .push_chirho(FrameChirho::PrimOpChirho {
                op_chirho: PrimOpKindChirho::SeqChirho,
                args_so_far_chirho: vec![],
                pending_args_chirho: vec![ValueChirho::IntChirho(42)],
                remaining_chirho: 2,
            });

        let result_chirho = machine_chirho
            .enter_pap_chirho(pap_addr_chirho, &pap_closure_chirho)
            .unwrap();

        match result_chirho {
            ReturnActionChirho::DoneChirho(ValueChirho::IntChirho(42)) => {}
            ReturnActionChirho::DoneChirho(other_chirho) => {
                panic!("expected seq result 42, got {:?}", other_chirho)
            }
            ReturnActionChirho::ContinueChirho(next_chirho) => {
                panic!(
                    "expected final seq result, got continuation {}",
                    next_chirho
                )
            }
        }
    }

    #[test]
    fn enter_pap_preserves_function_payload_chirho() {
        let mut machine_chirho = MachineChirho::new_chirho(vec![
            CodeChirho::PrimChirho {
                op_chirho: PrimOpKindChirho::AddIntChirho,
                args_chirho: vec![
                    ArgSourceChirho::ArgRegChirho(0),
                    ArgSourceChirho::ArgRegChirho(2),
                ],
            },
            CodeChirho::LitChirho(ValueChirho::IntChirho(0)),
        ]);

        let fun_addr_chirho = machine_chirho
            .heap_chirho
            .alloc_chirho(ClosureChirho::fun_chirho(
                2,
                CodePtrChirho(0),
                "captured_add_chirho",
                vec![ValueChirho::IntChirho(10)],
            ));
        let pap_addr_chirho = machine_chirho
            .heap_chirho
            .alloc_chirho(ClosureChirho::pap_chirho(
                1,
                fun_addr_chirho,
                vec![ValueChirho::IntChirho(20)],
            ));

        machine_chirho.code_table_chirho[1] = CodeChirho::AppChirho {
            fun_chirho: pap_addr_chirho,
            args_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(32))],
        };

        let result_chirho = machine_chirho.run_chirho(1).unwrap();
        assert_eq!(result_chirho, ValueChirho::IntChirho(42));
    }
}
