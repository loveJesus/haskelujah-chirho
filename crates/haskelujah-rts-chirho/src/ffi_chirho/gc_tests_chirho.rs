// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::NativeGcRuntimeChirho;

#[test]
fn immediate_roots_are_values_not_addresses_to_dereference_chirho() {
    let mut runtime_chirho = NativeGcRuntimeChirho::default();
    let live_chirho = runtime_chirho.alloc_chirho(16);
    let dead_chirho = runtime_chirho.alloc_chirho(16);
    runtime_chirho.gc_root_push_chirho(live_chirho);
    for bits_chirho in [0usize, 42, usize::MAX, usize::MAX - 1] {
        runtime_chirho.gc_root_push_chirho(bits_chirho as *mut u8);
    }
    runtime_chirho.collect_chirho();
    assert!(runtime_chirho.contains_alloc_chirho(live_chirho));
    assert!(!runtime_chirho.contains_alloc_chirho(dead_chirho));
}

#[test]
fn zero_root_still_balances_its_pop_chirho() {
    let mut runtime_chirho = NativeGcRuntimeChirho::default();
    let live_chirho = runtime_chirho.alloc_chirho(16);
    runtime_chirho.gc_root_push_chirho(live_chirho);
    runtime_chirho.gc_root_push_chirho(std::ptr::null_mut());
    runtime_chirho.gc_root_pop_chirho();
    runtime_chirho.collect_chirho();
    assert!(runtime_chirho.contains_alloc_chirho(live_chirho));
    runtime_chirho.gc_root_pop_chirho();
    runtime_chirho.collect_chirho();
    assert!(!runtime_chirho.contains_alloc_chirho(live_chirho));
}

#[test]
fn growing_live_heap_has_bounded_collection_work_and_still_retires_chirho() {
    let mut runtime_chirho = NativeGcRuntimeChirho::default();
    let count_chirho = 32_000;
    for _index_chirho in 0..count_chirho {
        let pointer_chirho = runtime_chirho.alloc_chirho(16);
        runtime_chirho.gc_root_push_chirho(pointer_chirho);
    }
    assert_eq!(runtime_chirho.allocation_count_chirho(), count_chirho);
    assert!(
        runtime_chirho.mark_visits_chirho < 3 * count_chirho,
        "retaining N objects must not cause quadratic rescanning: {} visits for {count_chirho} allocations",
        runtime_chirho.mark_visits_chirho
    );
    runtime_chirho.gc_roots_chirho.clear();
    runtime_chirho.collect_chirho();
    assert_eq!(runtime_chirho.allocation_count_chirho(), 0);
}

/// Row 566: a chain linked by tagged pointers survives whole. The marker queued each child by
/// its tagged value, whose registry lookup failed, so only a root's direct children survived.
#[test]
fn a_tagged_chain_survives_collection_whole_chirho() {
    let mut runtime_chirho = NativeGcRuntimeChirho::default();
    let cells_chirho: Vec<*mut u8> = (0..4).map(|_| runtime_chirho.alloc_chirho(24)).collect();
    for pair_chirho in cells_chirho.windows(2) {
        unsafe {
            pair_chirho[0]
                .cast::<usize>()
                .add(2)
                .write_unaligned(pair_chirho[1] as usize | 1);
        }
    }
    runtime_chirho.gc_root_push_chirho((cells_chirho[0] as usize | 1) as *mut u8);
    runtime_chirho.collect_chirho();
    for (depth_chirho, cell_chirho) in cells_chirho.iter().enumerate() {
        assert!(
            runtime_chirho.contains_alloc_chirho(*cell_chirho),
            "the cell at depth {depth_chirho} was collected"
        );
    }
}
