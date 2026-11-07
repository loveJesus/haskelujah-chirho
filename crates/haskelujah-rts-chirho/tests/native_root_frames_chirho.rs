// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One isolated-process RTS protocol test, not proof of emitted backend instructions.
//! Actual emitted constructor/closure/PAP paths have separate driver controls.
//! Workflow: testing-chirho/native-root-lifetimes-chirho.md.

use haskelujah_rts::ffi_chirho::{
    haskelujah_alloc_chirho, haskelujah_alloc_thunk_chirho, haskelujah_enter_thunk_chirho,
    haskelujah_gc_collect_chirho, haskelujah_gc_root_frame_mark_chirho,
    haskelujah_gc_root_frame_restore_chirho, haskelujah_gc_root_pop_chirho,
    haskelujah_gc_root_push_chirho, haskelujah_is_heap_ptr_chirho,
};

unsafe extern "C" fn framed_callback_chirho(_environment_chirho: *const u64) -> u64 {
    let frame_chirho = haskelujah_gc_root_frame_mark_chirho();
    let child_chirho = haskelujah_alloc_chirho(16).cast::<u64>();
    haskelujah_gc_root_push_chirho(child_chirho.cast());
    let parent_chirho = haskelujah_alloc_chirho(24).cast::<u64>();
    haskelujah_gc_root_push_chirho(parent_chirho.cast());
    unsafe {
        child_chirho.write(42);
        parent_chirho.write(7);
        parent_chirho.add(1).write(child_chirho as u64 | 1);
        parent_chirho.add(2).write(99);
    }
    for _index_chirho in 0..1100 {
        haskelujah_alloc_chirho(16);
    }
    haskelujah_gc_root_frame_restore_chirho(frame_chirho);
    // No native allocation occurs between restoration and the actual thunk update.
    parent_chirho as u64 | 1
}

#[test]
fn thunk_update_owns_a_framed_callback_result_before_collection_chirho() {
    let thunk_chirho = unsafe {
        haskelujah_alloc_thunk_chirho(
            framed_callback_chirho as *const () as u64,
            0,
            std::ptr::null(),
        )
    };
    haskelujah_gc_root_push_chirho(thunk_chirho as *mut u8);
    let result_chirho = haskelujah_enter_thunk_chirho(thunk_chirho);
    // Force real collection after the callback's frame is gone and after the
    // RTS has actually memoized its result into the still-rooted original thunk.
    haskelujah_gc_collect_chirho();
    assert_eq!(haskelujah_is_heap_ptr_chirho(result_chirho), 1);
    let parent_chirho = (result_chirho & !1) as *const u64;
    let child_chirho = unsafe {
        assert_eq!(parent_chirho.add(2).read(), 99);
        parent_chirho.add(1).read()
    };
    assert_eq!(haskelujah_is_heap_ptr_chirho(child_chirho), 1);
    assert_eq!(unsafe { ((child_chirho & !1) as *const u64).read() }, 42);
    assert_eq!(haskelujah_enter_thunk_chirho(thunk_chirho), result_chirho);
    haskelujah_gc_root_pop_chirho();
    haskelujah_gc_collect_chirho();
    assert_eq!(haskelujah_is_heap_ptr_chirho(result_chirho), 0);
    assert_eq!(haskelujah_is_heap_ptr_chirho(child_chirho), 0);
}
