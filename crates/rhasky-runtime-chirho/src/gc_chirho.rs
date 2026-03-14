// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Garbage collector
//!
//! A mark-sweep garbage collector for the STG heap. Traverses the root set
//! (stack frames, local environment) to find all reachable closures, then
//! replaces dead closures with tombstones that can be reused by future
//! allocations.
//!
//! ## Design
//!
//! - **Mark**: BFS from root set, setting a mark bit on each reachable closure.
//! - **Sweep**: Walk the heap, replacing unmarked closures with dead markers.
//! - **Trigger**: Configurable: collect after N allocations or when heap exceeds
//!   a threshold size.
//! - **Stability**: HeapAddrChirho values remain valid across collections (no
//!   compaction — addresses are stable indices).

use std::collections::VecDeque;

use crate::heap_chirho::HeapChirho;
use crate::stack_chirho::FrameChirho;
use crate::value_chirho::{HeapAddrChirho, InfoTagChirho, ValueChirho};

// ---------------------------------------------------------------------------
// GC configuration
// ---------------------------------------------------------------------------

/// Configuration for the garbage collector.
#[derive(Debug, Clone)]
pub struct GcConfigChirho {
    /// Collect when the heap grows by this many allocations since last GC.
    pub alloc_threshold_chirho: u64,
    /// Minimum heap size before GC is considered.
    pub min_heap_size_chirho: usize,
}

impl Default for GcConfigChirho {
    fn default() -> Self {
        Self {
            alloc_threshold_chirho: 1024,
            min_heap_size_chirho: 64,
        }
    }
}

// ---------------------------------------------------------------------------
// GC statistics
// ---------------------------------------------------------------------------

/// Statistics from a single GC cycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GcStatsChirho {
    /// Number of closures alive (reachable) after collection.
    pub live_count_chirho: usize,
    /// Number of closures collected (freed).
    pub dead_count_chirho: usize,
    /// Total heap size (live + dead tombstones + free slots).
    pub heap_size_chirho: usize,
    /// Number of GC cycles performed so far.
    pub cycle_count_chirho: u64,
}

// ---------------------------------------------------------------------------
// GC state
// ---------------------------------------------------------------------------

/// The garbage collector state, maintained across collections.
#[derive(Debug)]
pub struct GcStateChirho {
    pub config_chirho: GcConfigChirho,
    /// Mark bits: one per heap slot. Reset on each cycle.
    mark_bits_chirho: Vec<bool>,
    /// Number of allocations since the last GC.
    allocs_since_gc_chirho: u64,
    /// Total number of GC cycles performed.
    cycle_count_chirho: u64,
    /// Free list: indices of dead/tombstone slots available for reuse.
    free_list_chirho: Vec<u32>,
}

impl GcStateChirho {
    pub fn new_chirho(config_chirho: GcConfigChirho) -> Self {
        Self {
            config_chirho,
            mark_bits_chirho: Vec::new(),
            allocs_since_gc_chirho: 0,
            cycle_count_chirho: 0,
            free_list_chirho: Vec::new(),
        }
    }

    /// Notify the GC that an allocation occurred. Returns `true` if collection
    /// should be triggered.
    pub fn notify_alloc_chirho(&mut self) -> bool {
        self.allocs_since_gc_chirho += 1;
        self.allocs_since_gc_chirho >= self.config_chirho.alloc_threshold_chirho
    }

    /// Try to pop a free slot from the free list for reuse.
    pub fn pop_free_slot_chirho(&mut self) -> Option<u32> {
        self.free_list_chirho.pop()
    }

    /// Number of free slots available for reuse.
    pub fn free_count_chirho(&self) -> usize {
        self.free_list_chirho.len()
    }

    /// Total GC cycles performed.
    pub fn cycle_count_chirho(&self) -> u64 {
        self.cycle_count_chirho
    }

    /// Run a mark-sweep collection.
    ///
    /// `roots_chirho` is the set of heap addresses directly reachable from
    /// the root set (stack + local environment).
    pub fn collect_chirho(
        &mut self,
        heap_chirho: &mut HeapChirho,
        roots_chirho: &[HeapAddrChirho],
    ) -> GcStatsChirho {
        let heap_size_chirho = heap_chirho.size_chirho();
        if heap_size_chirho < self.config_chirho.min_heap_size_chirho {
            // Heap too small — skip collection.
            return GcStatsChirho {
                live_count_chirho: heap_size_chirho,
                dead_count_chirho: 0,
                heap_size_chirho,
                cycle_count_chirho: self.cycle_count_chirho,
            };
        }

        // -- Mark phase --
        self.mark_bits_chirho.clear();
        self.mark_bits_chirho.resize(heap_size_chirho, false);

        let mut worklist_chirho: VecDeque<HeapAddrChirho> = VecDeque::new();
        for root_chirho in roots_chirho {
            let idx_chirho = root_chirho.0 as usize;
            if idx_chirho < heap_size_chirho && !self.mark_bits_chirho[idx_chirho] {
                self.mark_bits_chirho[idx_chirho] = true;
                worklist_chirho.push_back(*root_chirho);
            }
        }

        while let Some(addr_chirho) = worklist_chirho.pop_front() {
            let closure_chirho = heap_chirho.read_chirho(addr_chirho);
            // Scan payload for heap pointers.
            for val_chirho in &closure_chirho.payload_chirho {
                if let ValueChirho::HeapPtrChirho(target_chirho) = val_chirho {
                    let idx_chirho = target_chirho.0 as usize;
                    if idx_chirho < heap_size_chirho && !self.mark_bits_chirho[idx_chirho] {
                        self.mark_bits_chirho[idx_chirho] = true;
                        worklist_chirho.push_back(*target_chirho);
                    }
                }
            }
        }

        // -- Sweep phase --
        let mut live_count_chirho = 0usize;
        let mut dead_count_chirho = 0usize;
        self.free_list_chirho.clear();

        for idx_chirho in 0..heap_size_chirho {
            if self.mark_bits_chirho[idx_chirho] {
                live_count_chirho += 1;
            } else {
                // Check if already a tombstone (dead marker). Don't overwrite
                // if it's already dead.
                let closure_chirho = heap_chirho.read_chirho(HeapAddrChirho(idx_chirho as u32));
                if closure_chirho.info_chirho.tag_chirho != InfoTagChirho::BlackholeChirho
                    || closure_chirho.info_chirho.name_chirho != "$DEAD"
                {
                    // Replace with a tombstone.
                    heap_chirho.write_tombstone_chirho(HeapAddrChirho(idx_chirho as u32));
                    dead_count_chirho += 1;
                }
                self.free_list_chirho.push(idx_chirho as u32);
            }
        }

        self.allocs_since_gc_chirho = 0;
        self.cycle_count_chirho += 1;

        GcStatsChirho {
            live_count_chirho,
            dead_count_chirho,
            heap_size_chirho,
            cycle_count_chirho: self.cycle_count_chirho,
        }
    }
}

// ---------------------------------------------------------------------------
// Root set extraction
// ---------------------------------------------------------------------------

/// Extract all heap addresses from a set of values (the root set).
pub fn extract_roots_from_values_chirho(values_chirho: &[ValueChirho]) -> Vec<HeapAddrChirho> {
    values_chirho
        .iter()
        .filter_map(|v_chirho| match v_chirho {
            ValueChirho::HeapPtrChirho(addr_chirho) => Some(*addr_chirho),
            _ => None,
        })
        .collect()
}

/// Extract all heap addresses from the stack frames.
pub fn extract_roots_from_stack_chirho(
    frames_chirho: &[FrameChirho],
) -> Vec<HeapAddrChirho> {
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
    use crate::value_chirho::{ClosureChirho, CodePtrChirho, DataConTagChirho};

    fn make_test_heap_chirho() -> HeapChirho {
        let mut heap_chirho = HeapChirho::new_chirho();
        // addr 0: a live constructor (root)
        heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "True",
            vec![],
        ));
        // addr 1: a live constructor pointed to by addr 2
        heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "I#",
            vec![ValueChirho::IntChirho(42)],
        ));
        // addr 2: points to addr 1 (indirection)
        heap_chirho.alloc_chirho(ClosureChirho::ind_chirho(HeapAddrChirho(1)));
        // addr 3: unreachable (dead)
        heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "Dead",
            vec![],
        ));
        // addr 4: unreachable (dead)
        heap_chirho.alloc_chirho(ClosureChirho::thunk_chirho(
            CodePtrChirho(0),
            "dead_thunk",
            vec![],
        ));
        heap_chirho
    }

    #[test]
    fn mark_sweep_collects_dead_chirho() {
        let mut heap_chirho = make_test_heap_chirho();
        let mut gc_chirho = GcStateChirho::new_chirho(GcConfigChirho {
            alloc_threshold_chirho: 1,
            min_heap_size_chirho: 1,
        });

        // Roots: addr 0 (True) and addr 2 (IND -> addr 1)
        let roots_chirho = vec![HeapAddrChirho(0), HeapAddrChirho(2)];
        let stats_chirho = gc_chirho.collect_chirho(&mut heap_chirho, &roots_chirho);

        // addr 0: live (root), addr 1: live (reachable via 2), addr 2: live (root)
        // addr 3: dead, addr 4: dead
        assert_eq!(stats_chirho.live_count_chirho, 3);
        assert_eq!(stats_chirho.dead_count_chirho, 2);
        assert_eq!(stats_chirho.heap_size_chirho, 5);
        assert_eq!(stats_chirho.cycle_count_chirho, 1);

        // Dead slots should be in the free list
        assert_eq!(gc_chirho.free_count_chirho(), 2);
    }

    #[test]
    fn all_reachable_nothing_collected_chirho() {
        let mut heap_chirho = HeapChirho::new_chirho();
        let a0_chirho = heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "X",
            vec![],
        ));
        let a1_chirho = heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(1),
            "Y",
            vec![ValueChirho::HeapPtrChirho(a0_chirho)],
        ));

        let mut gc_chirho = GcStateChirho::new_chirho(GcConfigChirho {
            alloc_threshold_chirho: 1,
            min_heap_size_chirho: 1,
        });

        let roots_chirho = vec![a1_chirho];
        let stats_chirho = gc_chirho.collect_chirho(&mut heap_chirho, &roots_chirho);

        assert_eq!(stats_chirho.live_count_chirho, 2);
        assert_eq!(stats_chirho.dead_count_chirho, 0);
    }

    #[test]
    fn empty_roots_collects_everything_chirho() {
        let mut heap_chirho = HeapChirho::new_chirho();
        for i_chirho in 0..5u16 {
            heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
                DataConTagChirho(i_chirho),
                "Dead",
                vec![],
            ));
        }

        let mut gc_chirho = GcStateChirho::new_chirho(GcConfigChirho {
            alloc_threshold_chirho: 1,
            min_heap_size_chirho: 1,
        });

        let stats_chirho = gc_chirho.collect_chirho(&mut heap_chirho, &[]);

        assert_eq!(stats_chirho.live_count_chirho, 0);
        assert_eq!(stats_chirho.dead_count_chirho, 5);
        assert_eq!(gc_chirho.free_count_chirho(), 5);
    }

    #[test]
    fn notify_alloc_triggers_at_threshold_chirho() {
        let mut gc_chirho = GcStateChirho::new_chirho(GcConfigChirho {
            alloc_threshold_chirho: 3,
            min_heap_size_chirho: 1,
        });

        assert!(!gc_chirho.notify_alloc_chirho()); // 1
        assert!(!gc_chirho.notify_alloc_chirho()); // 2
        assert!(gc_chirho.notify_alloc_chirho());  // 3 — trigger!
    }

    #[test]
    fn small_heap_skips_gc_chirho() {
        let mut heap_chirho = HeapChirho::new_chirho();
        heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "X",
            vec![],
        ));

        let mut gc_chirho = GcStateChirho::new_chirho(GcConfigChirho {
            alloc_threshold_chirho: 1,
            min_heap_size_chirho: 100, // heap too small
        });

        let stats_chirho = gc_chirho.collect_chirho(&mut heap_chirho, &[]);
        // Nothing collected because heap is below min size
        assert_eq!(stats_chirho.dead_count_chirho, 0);
        assert_eq!(stats_chirho.live_count_chirho, 1);
    }

    #[test]
    fn extract_roots_from_values_chirho() {
        let values_chirho = vec![
            ValueChirho::IntChirho(1),
            ValueChirho::HeapPtrChirho(HeapAddrChirho(5)),
            ValueChirho::FloatChirho(3.14),
            ValueChirho::HeapPtrChirho(HeapAddrChirho(10)),
        ];
        let roots_chirho = super::extract_roots_from_values_chirho(&values_chirho);
        assert_eq!(roots_chirho, vec![HeapAddrChirho(5), HeapAddrChirho(10)]);
    }

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

    #[test]
    fn cycle_count_increments_chirho() {
        let mut heap_chirho = HeapChirho::new_chirho();
        for _ in 0..5 {
            heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
                DataConTagChirho(0),
                "X",
                vec![],
            ));
        }

        let mut gc_chirho = GcStateChirho::new_chirho(GcConfigChirho {
            alloc_threshold_chirho: 1,
            min_heap_size_chirho: 1,
        });

        let root_chirho = vec![HeapAddrChirho(0)];
        gc_chirho.collect_chirho(&mut heap_chirho, &root_chirho);
        gc_chirho.collect_chirho(&mut heap_chirho, &root_chirho);
        gc_chirho.collect_chirho(&mut heap_chirho, &root_chirho);

        assert_eq!(gc_chirho.cycle_count_chirho(), 3);
    }

    #[test]
    fn transitive_reachability_chirho() {
        let mut heap_chirho = HeapChirho::new_chirho();
        // Build a chain: root -> a0 -> a1 -> a2 (all reachable)
        // Plus a3 (unreachable)
        let a0_chirho = heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "A",
            vec![],
        ));
        let a1_chirho = heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "B",
            vec![ValueChirho::HeapPtrChirho(a0_chirho)],
        ));
        let a2_chirho = heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "C",
            vec![ValueChirho::HeapPtrChirho(a1_chirho)],
        ));
        // a3 is unreachable
        heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "D",
            vec![],
        ));

        let mut gc_chirho = GcStateChirho::new_chirho(GcConfigChirho {
            alloc_threshold_chirho: 1,
            min_heap_size_chirho: 1,
        });

        let stats_chirho = gc_chirho.collect_chirho(&mut heap_chirho, &[a2_chirho]);
        assert_eq!(stats_chirho.live_count_chirho, 3); // a0, a1, a2
        assert_eq!(stats_chirho.dead_count_chirho, 1); // a3
    }

    #[test]
    fn circular_references_handled_chirho() {
        let mut heap_chirho = HeapChirho::new_chirho();
        // Create a cycle: a0 -> a1 -> a0
        let a0_chirho = heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "CycA",
            vec![], // will be updated
        ));
        let a1_chirho = heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "CycB",
            vec![ValueChirho::HeapPtrChirho(a0_chirho)],
        ));
        // Update a0 to point to a1 (creating the cycle)
        *heap_chirho.read_mut_chirho(a0_chirho) = ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "CycA",
            vec![ValueChirho::HeapPtrChirho(a1_chirho)],
        );

        // Also add an unreachable node
        heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "Dead",
            vec![],
        ));

        let mut gc_chirho = GcStateChirho::new_chirho(GcConfigChirho {
            alloc_threshold_chirho: 1,
            min_heap_size_chirho: 1,
        });

        let stats_chirho = gc_chirho.collect_chirho(&mut heap_chirho, &[a0_chirho]);
        assert_eq!(stats_chirho.live_count_chirho, 2); // a0, a1 (cycle)
        assert_eq!(stats_chirho.dead_count_chirho, 1); // unreachable
    }
}
