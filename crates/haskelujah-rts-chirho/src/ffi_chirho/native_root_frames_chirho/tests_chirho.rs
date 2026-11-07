// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::{NativeGcRuntimeChirho, NativeRootFramesChirho};

#[test]
fn nested_frames_preserve_caller_roots_and_retire_only_their_suffix_chirho() {
    let mut runtime_chirho = NativeGcRuntimeChirho::default();
    let outer_chirho = runtime_chirho.mark_root_frame_chirho().unwrap();
    let retained_chirho = runtime_chirho.alloc_chirho(24);
    runtime_chirho.gc_root_push_chirho(retained_chirho);
    let inner_chirho = runtime_chirho.mark_root_frame_chirho().unwrap();
    let retired_chirho = runtime_chirho.alloc_chirho(24);
    runtime_chirho.gc_root_push_chirho(retired_chirho);
    assert!(
        runtime_chirho
            .restore_root_frame_chirho(outer_chirho)
            .is_err()
    );
    assert_eq!(runtime_chirho.gc_roots_chirho.len(), 2);
    runtime_chirho
        .restore_root_frame_chirho(inner_chirho)
        .unwrap();
    runtime_chirho.collect_chirho();
    assert!(runtime_chirho.contains_alloc_chirho(retained_chirho));
    assert!(!runtime_chirho.contains_alloc_chirho(retired_chirho));
    assert!(
        runtime_chirho
            .restore_root_frame_chirho(inner_chirho)
            .is_err()
    );
    runtime_chirho
        .restore_root_frame_chirho(outer_chirho)
        .unwrap();
    runtime_chirho.collect_chirho();
    assert_eq!(runtime_chirho.allocation_count_chirho(), 0);
}

#[test]
fn reset_retains_its_token_and_balances_immediate_and_raw_roots_chirho() {
    let mut runtime_chirho = NativeGcRuntimeChirho::default();
    let frame_chirho = runtime_chirho.mark_root_frame_chirho().unwrap();
    let retained_chirho = runtime_chirho.alloc_chirho(24);
    runtime_chirho.gc_root_push_chirho(retained_chirho);
    runtime_chirho.gc_root_push_chirho(std::ptr::null_mut());
    runtime_chirho.gc_root_pop_chirho();
    runtime_chirho
        .reset_root_frame_chirho(frame_chirho)
        .unwrap();
    // No allocation or collection occurs during replacement. The raw object
    // is re-rooted before the next explicitly forced collection.
    runtime_chirho.gc_root_push_chirho(retained_chirho);
    runtime_chirho.collect_chirho();
    assert!(runtime_chirho.contains_alloc_chirho(retained_chirho));
    runtime_chirho
        .restore_root_frame_chirho(frame_chirho)
        .unwrap();
    assert!(runtime_chirho.root_frames_chirho.frames_chirho.is_empty());
    assert!(
        runtime_chirho
            .reset_root_frame_chirho(frame_chirho)
            .is_err()
    );
}

#[test]
fn invalid_depth_or_owner_cannot_remove_roots_chirho() {
    let mut frames_chirho = NativeRootFramesChirho::default();
    let token_chirho = frames_chirho.mark_chirho(3).unwrap();
    assert!(frames_chirho.start_chirho(token_chirho, 2).is_err());
    assert!(frames_chirho.validate_pop_chirho(3).is_err());
    let mut runtime_chirho = NativeGcRuntimeChirho::default();
    let frame_chirho = runtime_chirho.mark_root_frame_chirho().unwrap();
    let retained_chirho = runtime_chirho.alloc_chirho(24);
    runtime_chirho.gc_root_push_chirho(retained_chirho);
    let mut runtime_chirho = std::thread::spawn(move || {
        assert!(
            runtime_chirho
                .restore_root_frame_chirho(frame_chirho)
                .is_err()
        );
        assert_eq!(runtime_chirho.gc_roots_chirho.len(), 1);
        runtime_chirho
    })
    .join()
    .unwrap();
    runtime_chirho.collect_chirho();
    assert!(runtime_chirho.contains_alloc_chirho(retained_chirho));
    runtime_chirho
        .restore_root_frame_chirho(frame_chirho)
        .unwrap();
    runtime_chirho.collect_chirho();
    assert!(!runtime_chirho.contains_alloc_chirho(retained_chirho));
}
