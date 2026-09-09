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
