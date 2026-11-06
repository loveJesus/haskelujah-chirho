// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One process owns the native RTS global state for this entire protocol test.
//! These callbacks prove RTS behavior; emitted-code proof uses the unchanged
//! native growth and constructor/closure/PAP controls in the driver.
//! Workflow: testing-chirho/native-root-lifetimes-chirho.md.

use haskelujah_rts::ffi_chirho::{
    haskelujah_alloc_chirho, haskelujah_alloc_thunk_chirho, haskelujah_enter_thunk_chirho,
    haskelujah_gc_collect_chirho, haskelujah_gc_root_frame_mark_chirho,
    haskelujah_gc_root_frame_reset_chirho, haskelujah_gc_root_frame_restore_chirho,
    haskelujah_gc_root_push_chirho, haskelujah_is_heap_ptr_chirho, haskelujah_update_thunk_chirho,
};
use std::sync::atomic::{AtomicU64, Ordering};

static ACTIVE_CALLBACK_THUNK_CHIRHO: AtomicU64 = AtomicU64::new(0);

fn cell_chirho(payload_chirho: u64) -> u64 {
    let raw_chirho = haskelujah_alloc_chirho(16).cast::<u64>();
    unsafe { raw_chirho.write(payload_chirho) };
    raw_chirho as u64 | 1
}

fn assert_cell_chirho(value_chirho: u64, payload_chirho: u64) {
    assert_eq!(haskelujah_is_heap_ptr_chirho(value_chirho), 1);
    assert_eq!(
        unsafe { ((value_chirho & !1) as *const u64).read() },
        payload_chirho,
    );
}

unsafe extern "C" fn return_captured_alias_chirho(environment_chirho: *const u64) -> u64 {
    haskelujah_gc_collect_chirho();
    let obsolete_chirho = unsafe { environment_chirho.read() };
    let result_chirho = unsafe { environment_chirho.add(1).read() };
    assert_cell_chirho(obsolete_chirho, 41);
    assert_cell_chirho(result_chirho, 42);
    result_chirho
}

unsafe extern "C" fn update_then_read_active_capture_chirho(environment_chirho: *const u64) -> u64 {
    let thunk_chirho = ACTIVE_CALLBACK_THUNK_CHIRHO.load(Ordering::Relaxed);
    haskelujah_update_thunk_chirho(thunk_chirho, 99);
    // The callback has not finished using its environment. Neither explicit
    // publication nor real collection may retire its still-live capture.
    haskelujah_gc_collect_chirho();
    let child_chirho = unsafe { environment_chirho.read() };
    assert_cell_chirho(child_chirho, 43);
    99
}

fn completed_callback_retires_old_captures_chirho() {
    let frame_chirho = haskelujah_gc_root_frame_mark_chirho();
    let obsolete_chirho = cell_chirho(41);
    haskelujah_gc_root_push_chirho(obsolete_chirho as *mut u8);
    let result_chirho = cell_chirho(42);
    haskelujah_gc_root_push_chirho(result_chirho as *mut u8);
    let environment_chirho = [obsolete_chirho, result_chirho];
    let thunk_chirho = unsafe {
        haskelujah_alloc_thunk_chirho(
            return_captured_alias_chirho as *const () as u64,
            environment_chirho.len() as u64,
            environment_chirho.as_ptr(),
        )
    };
    haskelujah_gc_root_frame_reset_chirho(frame_chirho);
    haskelujah_gc_root_push_chirho(thunk_chirho as *mut u8);
    assert_eq!(haskelujah_enter_thunk_chirho(thunk_chirho), result_chirho);
    haskelujah_gc_collect_chirho();
    assert_eq!(haskelujah_is_heap_ptr_chirho(obsolete_chirho), 0);
    assert_cell_chirho(result_chirho, 42);
    assert_eq!(haskelujah_enter_thunk_chirho(thunk_chirho), result_chirho);
    haskelujah_gc_root_frame_restore_chirho(frame_chirho);
    haskelujah_gc_collect_chirho();
    assert_eq!(haskelujah_is_heap_ptr_chirho(result_chirho), 0);
}

fn inactive_update_retires_old_captures_chirho() {
    let frame_chirho = haskelujah_gc_root_frame_mark_chirho();
    let obsolete_chirho = cell_chirho(41);
    haskelujah_gc_root_push_chirho(obsolete_chirho as *mut u8);
    let result_chirho = cell_chirho(42);
    haskelujah_gc_root_push_chirho(result_chirho as *mut u8);
    let environment_chirho = [obsolete_chirho, result_chirho];
    let thunk_chirho = unsafe {
        haskelujah_alloc_thunk_chirho(
            return_captured_alias_chirho as *const () as u64,
            environment_chirho.len() as u64,
            environment_chirho.as_ptr(),
        )
    };
    haskelujah_gc_root_frame_reset_chirho(frame_chirho);
    haskelujah_gc_root_push_chirho(thunk_chirho as *mut u8);
    // No allocation lies between releasing the construction roots and
    // publishing the live result into the sole rooted thunk.
    haskelujah_update_thunk_chirho(thunk_chirho, result_chirho);
    haskelujah_gc_collect_chirho();
    assert_eq!(haskelujah_is_heap_ptr_chirho(obsolete_chirho), 0);
    assert_cell_chirho(result_chirho, 42);
    assert_eq!(haskelujah_enter_thunk_chirho(thunk_chirho), result_chirho);
    haskelujah_gc_root_frame_restore_chirho(frame_chirho);
    haskelujah_gc_collect_chirho();
    assert_eq!(haskelujah_is_heap_ptr_chirho(result_chirho), 0);
}

fn active_update_preserves_capture_until_callback_returns_chirho() {
    let frame_chirho = haskelujah_gc_root_frame_mark_chirho();
    let child_chirho = cell_chirho(43);
    haskelujah_gc_root_push_chirho(child_chirho as *mut u8);
    let thunk_chirho = unsafe {
        haskelujah_alloc_thunk_chirho(
            update_then_read_active_capture_chirho as *const () as u64,
            1,
            &child_chirho,
        )
    };
    haskelujah_gc_root_frame_reset_chirho(frame_chirho);
    haskelujah_gc_root_push_chirho(thunk_chirho as *mut u8);
    ACTIVE_CALLBACK_THUNK_CHIRHO.store(thunk_chirho, Ordering::Relaxed);
    assert_eq!(haskelujah_enter_thunk_chirho(thunk_chirho), 99);
    haskelujah_gc_collect_chirho();
    assert_eq!(haskelujah_is_heap_ptr_chirho(child_chirho), 0);
    assert_eq!(haskelujah_enter_thunk_chirho(thunk_chirho), 99);
    ACTIVE_CALLBACK_THUNK_CHIRHO.store(0, Ordering::Relaxed);
    haskelujah_gc_root_frame_restore_chirho(frame_chirho);
    haskelujah_gc_collect_chirho();
}

#[test]
fn completed_thunks_retire_captures_without_losing_live_results_or_active_environments_chirho() {
    completed_callback_retires_old_captures_chirho();
    inactive_update_retires_old_captures_chirho();
    active_update_preserves_capture_until_callback_returns_chirho();
}
