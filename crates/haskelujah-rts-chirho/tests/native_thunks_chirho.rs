// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use haskelujah_rts::ffi_chirho::{
    haskelujah_alloc_chirho, haskelujah_alloc_thunk_chirho, haskelujah_enter_thunk_chirho,
    haskelujah_gc_root_pop_chirho, haskelujah_gc_root_push_chirho, haskelujah_update_thunk_chirho,
};
use std::sync::atomic::{AtomicUsize, Ordering};

static CALLS_CHIRHO: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn return_environment_chirho(environment_chirho: *const u64) -> u64 {
    CALLS_CHIRHO.fetch_add(1, Ordering::Relaxed);
    unsafe { *environment_chirho }
}

unsafe extern "C" fn allocating_entry_chirho(environment_chirho: *const u64) -> u64 {
    for _index_chirho in 0..1100 {
        haskelujah_alloc_chirho(16);
    }
    unsafe { *environment_chirho }
}

#[test]
fn native_entry_reaches_whnf_memoizes_full_values_and_survives_gc_chirho() {
    for value_chirho in [0, 42, u64::MAX, i64::MIN as u64, (-2.5_f64).to_bits()] {
        CALLS_CHIRHO.store(0, Ordering::Relaxed);
        // SAFETY: the callback reads the one initialized environment word copied here.
        let leaf_chirho = unsafe {
            haskelujah_alloc_thunk_chirho(
                return_environment_chirho as *const () as u64,
                1,
                &value_chirho,
            )
        };
        haskelujah_gc_root_push_chirho(leaf_chirho as *mut u8);
        let mut outer_chirho = leaf_chirho;
        for _index_chirho in 0..150 {
            // SAFETY: one readable word; its referenced thunk is already rooted.
            outer_chirho = unsafe {
                haskelujah_alloc_thunk_chirho(
                    return_environment_chirho as *const () as u64,
                    1,
                    &outer_chirho,
                )
            };
            haskelujah_gc_root_push_chirho(outer_chirho as *mut u8);
        }
        assert_eq!(haskelujah_enter_thunk_chirho(outer_chirho), value_chirho);
        assert_eq!(haskelujah_enter_thunk_chirho(outer_chirho), value_chirho);
        assert_eq!(haskelujah_enter_thunk_chirho(leaf_chirho), value_chirho);
        assert_eq!(CALLS_CHIRHO.load(Ordering::Relaxed), 151);
        haskelujah_update_thunk_chirho(leaf_chirho, !value_chirho);
        assert_eq!(haskelujah_enter_thunk_chirho(leaf_chirho), !value_chirho);
        for _index_chirho in 0..151 {
            haskelujah_gc_root_pop_chirho();
        }
    }
    let value_chirho = 123;
    // SAFETY: the callback has the required ABI and its one-word environment is valid.
    let active_chirho = unsafe {
        haskelujah_alloc_thunk_chirho(
            allocating_entry_chirho as *const () as u64,
            1,
            &value_chirho,
        )
    };
    assert_eq!(haskelujah_enter_thunk_chirho(active_chirho), value_chirho);
    assert_eq!(haskelujah_enter_thunk_chirho(active_chirho), value_chirho);
}
