// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Garbage collector
//!
//! Re-exports the backend-agnostic GC primitives from `haskelujah-rts-chirho`
//! (configuration, statistics, mark-sweep state, value root extraction).
//!
//! Adds the interpreter-specific `extract_roots_from_stack_chirho` which
//! depends on `FrameChirho` — a type that only exists in this crate.

// Re-export shared GC types from the RTS crate.
pub use haskelujah_rts_chirho::gc_chirho::{
    GcConfigChirho, GcStateChirho, GcStatsChirho, extract_roots_from_values_chirho,
};

use crate::stack_chirho::FrameChirho;
use crate::value_chirho::{HeapAddrChirho, ValueChirho};

// ---------------------------------------------------------------------------
// Stack root extraction (interpreter-specific)
// ---------------------------------------------------------------------------

/// Extract all heap addresses from the stack frames.
pub fn extract_roots_from_stack_chirho(frames_chirho: &[FrameChirho]) -> Vec<HeapAddrChirho> {
    let mut roots_chirho = Vec::new();
    for frame_chirho in frames_chirho {
        match frame_chirho {
            FrameChirho::ApplyChirho { args_chirho } => {
                for val_chirho in args_chirho {
                    if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                        roots_chirho.push(*addr_chirho);
                    }
                }
            }
            FrameChirho::UpdateChirho { thunk_addr_chirho } => {
                roots_chirho.push(*thunk_addr_chirho);
            }
            FrameChirho::CaseChirho {
                saved_arg_regs_chirho,
                ..
            } => {
                for val_chirho in saved_arg_regs_chirho {
                    if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                        roots_chirho.push(*addr_chirho);
                    }
                }
            }
            FrameChirho::CaseLitChirho {
                alt_entries_chirho,
                saved_arg_regs_chirho,
                ..
            } => {
                for (val_chirho, _) in alt_entries_chirho {
                    if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                        roots_chirho.push(*addr_chirho);
                    }
                }
                for val_chirho in saved_arg_regs_chirho {
                    if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                        roots_chirho.push(*addr_chirho);
                    }
                }
            }
            FrameChirho::PrimOpChirho {
                args_so_far_chirho,
                pending_args_chirho,
                ..
            } => {
                for val_chirho in args_so_far_chirho.iter().chain(pending_args_chirho.iter()) {
                    if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                        roots_chirho.push(*addr_chirho);
                    }
                }
            }
            FrameChirho::CatchChirho {
                handler_addr_chirho,
                saved_arg_regs_chirho,
            } => {
                roots_chirho.push(*handler_addr_chirho);
                for val_chirho in saved_arg_regs_chirho {
                    if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                        roots_chirho.push(*addr_chirho);
                    }
                }
            }
            FrameChirho::TryFrameChirho {
                saved_arg_regs_chirho,
            } => {
                for val_chirho in saved_arg_regs_chirho {
                    if let ValueChirho::HeapPtrChirho(addr_chirho) = val_chirho {
                        roots_chirho.push(*addr_chirho);
                    }
                }
            }
        }
    }
    roots_chirho
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn extract_roots_from_stack_chirho() {
        let frames_chirho = vec![
            FrameChirho::ApplyChirho {
                args_chirho: vec![
                    ValueChirho::HeapPtrChirho(HeapAddrChirho(1)),
                    ValueChirho::IntChirho(99),
                ],
            },
            FrameChirho::UpdateChirho {
                thunk_addr_chirho: HeapAddrChirho(7),
            },
            FrameChirho::CaseChirho {
                alt_entries_chirho: vec![],
                default_entry_chirho: None,
                saved_arg_regs_chirho: vec![],
            },
        ];
        let roots_chirho = super::extract_roots_from_stack_chirho(&frames_chirho);
        assert_eq!(roots_chirho, vec![HeapAddrChirho(1), HeapAddrChirho(7)]);
    }
}
