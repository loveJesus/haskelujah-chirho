// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Heap — the closure store
//!
//! A simple arena-based heap for allocating closures. Each closure gets
//! a stable `HeapAddrChirho` (index into the backing array). The heap
//! supports in-place mutation for thunk updates (replacing a thunk with
//! an indirection after evaluation) and blackholing.
//!
//! This is intentionally simple: a flat `Vec<ClosureChirho>` with no
//! GC yet. GC will be added as a separate concern once the evaluator
//! is functional.

use crate::value_chirho::{ClosureChirho, HeapAddrChirho, InfoTagChirho, ValueChirho};

/// The heap: a flat array of closures indexed by HeapAddrChirho.
#[derive(Debug)]
pub struct HeapChirho {
    closures_chirho: Vec<ClosureChirho>,
    /// Number of allocations performed (for statistics).
    alloc_count_chirho: u64,
}

impl HeapChirho {
    /// Create a new empty heap.
    pub fn new_chirho() -> Self {
        Self {
            closures_chirho: Vec::new(),
            alloc_count_chirho: 0,
        }
    }

    /// Create a heap with pre-allocated capacity.
    pub fn with_capacity_chirho(capacity_chirho: usize) -> Self {
        Self {
            closures_chirho: Vec::with_capacity(capacity_chirho),
            alloc_count_chirho: 0,
        }
    }

    /// Allocate a closure on the heap, returning its address.
    pub fn alloc_chirho(&mut self, closure_chirho: ClosureChirho) -> HeapAddrChirho {
        let addr_chirho = HeapAddrChirho(self.closures_chirho.len() as u32);
        self.closures_chirho.push(closure_chirho);
        self.alloc_count_chirho += 1;
        addr_chirho
    }

    /// Read a closure at the given address.
    ///
    /// Panics if the address is out of bounds.
    pub fn read_chirho(&self, addr_chirho: HeapAddrChirho) -> &ClosureChirho {
        &self.closures_chirho[addr_chirho.0 as usize]
    }

    /// Read a closure mutably at the given address.
    pub fn read_mut_chirho(&mut self, addr_chirho: HeapAddrChirho) -> &mut ClosureChirho {
        &mut self.closures_chirho[addr_chirho.0 as usize]
    }

    /// Update a thunk to an indirection pointing at the result.
    ///
    /// This is the STG "update" operation: after evaluating a thunk,
    /// replace it with an indirection to the WHNF result.
    pub fn update_to_ind_chirho(
        &mut self,
        thunk_addr_chirho: HeapAddrChirho,
        result_addr_chirho: HeapAddrChirho,
    ) {
        let closure_chirho = &mut self.closures_chirho[thunk_addr_chirho.0 as usize];
        *closure_chirho = ClosureChirho::ind_chirho(result_addr_chirho);
    }

    /// Replace a thunk with a blackhole (marks it as being evaluated).
    pub fn blackhole_chirho(&mut self, addr_chirho: HeapAddrChirho) {
        let closure_chirho = &mut self.closures_chirho[addr_chirho.0 as usize];
        *closure_chirho = ClosureChirho::blackhole_chirho();
    }

    /// Follow indirection chains to find the final target.
    ///
    /// Returns the address of the first non-indirection closure.
    pub fn follow_ind_chirho(&self, mut addr_chirho: HeapAddrChirho) -> HeapAddrChirho {
        loop {
            let closure_chirho = self.read_chirho(addr_chirho);
            if closure_chirho.info_chirho.tag_chirho == InfoTagChirho::IndChirho {
                if let Some(ValueChirho::HeapPtrChirho(target_chirho)) =
                    closure_chirho.payload_chirho.first()
                {
                    addr_chirho = *target_chirho;
                    continue;
                }
            }
            return addr_chirho;
        }
    }

    /// Write a tombstone (dead marker) at the given address.
    ///
    /// Used by the garbage collector to mark a slot as dead/reclaimable.
    pub fn write_tombstone_chirho(&mut self, addr_chirho: HeapAddrChirho) {
        self.closures_chirho[addr_chirho.0 as usize] = ClosureChirho::tombstone_chirho();
    }

    /// Number of closures currently on the heap.
    pub fn size_chirho(&self) -> usize {
        self.closures_chirho.len()
    }

    /// Total number of allocations performed.
    pub fn alloc_count_chirho(&self) -> u64 {
        self.alloc_count_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::value_chirho::{CodePtrChirho, DataConTagChirho};

    #[test]
    fn alloc_and_read_chirho() {
        let mut heap_chirho = HeapChirho::new_chirho();
        let closure_chirho = ClosureChirho::con_chirho(DataConTagChirho(0), "True", vec![]);
        let addr_chirho = heap_chirho.alloc_chirho(closure_chirho);
        assert_eq!(addr_chirho, HeapAddrChirho(0));

        let read_chirho = heap_chirho.read_chirho(addr_chirho);
        assert_eq!(read_chirho.info_chirho.name_chirho, "True");
    }

    #[test]
    fn alloc_increments_address_chirho() {
        let mut heap_chirho = HeapChirho::new_chirho();
        let a0_chirho = heap_chirho.alloc_chirho(ClosureChirho::blackhole_chirho());
        let a1_chirho = heap_chirho.alloc_chirho(ClosureChirho::blackhole_chirho());
        assert_eq!(a0_chirho, HeapAddrChirho(0));
        assert_eq!(a1_chirho, HeapAddrChirho(1));
        assert_eq!(heap_chirho.size_chirho(), 2);
        assert_eq!(heap_chirho.alloc_count_chirho(), 2);
    }

    #[test]
    fn update_thunk_to_ind_chirho() {
        let mut heap_chirho = HeapChirho::new_chirho();

        // Allocate a thunk
        let thunk_addr_chirho =
            heap_chirho.alloc_chirho(ClosureChirho::thunk_chirho(CodePtrChirho(0), "x", vec![]));

        // Allocate the result
        let result_addr_chirho = heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "I#",
            vec![ValueChirho::IntChirho(42)],
        ));

        // Update thunk → indirection
        heap_chirho.update_to_ind_chirho(thunk_addr_chirho, result_addr_chirho);

        let updated_chirho = heap_chirho.read_chirho(thunk_addr_chirho);
        assert_eq!(
            updated_chirho.info_chirho.tag_chirho,
            InfoTagChirho::IndChirho
        );
    }

    #[test]
    fn follow_ind_chain_chirho() {
        let mut heap_chirho = HeapChirho::new_chirho();

        // addr 0: the actual value
        let val_addr_chirho = heap_chirho.alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(0),
            "I#",
            vec![ValueChirho::IntChirho(1)],
        ));

        // addr 1: indirection → addr 0
        let ind1_chirho = heap_chirho.alloc_chirho(ClosureChirho::ind_chirho(val_addr_chirho));

        // addr 2: indirection → addr 1 (chain)
        let ind2_chirho = heap_chirho.alloc_chirho(ClosureChirho::ind_chirho(ind1_chirho));

        // Following from addr 2 should reach addr 0
        let resolved_chirho = heap_chirho.follow_ind_chirho(ind2_chirho);
        assert_eq!(resolved_chirho, val_addr_chirho);
    }

    #[test]
    fn blackhole_chirho() {
        let mut heap_chirho = HeapChirho::new_chirho();
        let addr_chirho =
            heap_chirho.alloc_chirho(ClosureChirho::thunk_chirho(CodePtrChirho(0), "x", vec![]));

        heap_chirho.blackhole_chirho(addr_chirho);

        let read_chirho = heap_chirho.read_chirho(addr_chirho);
        assert_eq!(
            read_chirho.info_chirho.tag_chirho,
            InfoTagChirho::BlackholeChirho
        );
    }
}
