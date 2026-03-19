// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # C FFI for the Haskelujah Runtime System
//!
//! Exposes RTS functions with C ABI so LLVM-compiled Haskell programs can
//! call them. The LLVM codegen emits calls to these `@haskelujah_*` symbols,
//! and they are resolved at link time against the RTS static library.
//!
//! ## Usage
//!
//! The RTS crate is compiled as a `staticlib`. LLVM-compiled executables
//! are linked with `clang -lhaskelujah_rts_chirho` to resolve these symbols.

use std::alloc::{alloc, dealloc, Layout};
use std::sync::atomic::{AtomicU64, Ordering};

/// Total bytes allocated (for GC triggering heuristics).
static ALLOC_TOTAL_CHIRHO: AtomicU64 = AtomicU64::new(0);

/// Allocation count since last GC.
static ALLOC_COUNT_CHIRHO: AtomicU64 = AtomicU64::new(0);

/// GC threshold: collect after this many allocations.
const GC_THRESHOLD_CHIRHO: u64 = 65536;

// ---------------------------------------------------------------------------
// Shadow stack for GC roots
// ---------------------------------------------------------------------------

/// Maximum shadow stack depth (nested function calls).
const MAX_SHADOW_STACK_CHIRHO: usize = 16384;

/// Shadow stack for GC root tracking.
/// Each entry is a pointer to a heap-allocated object that must survive GC.
static mut SHADOW_STACK_CHIRHO: [*mut u8; MAX_SHADOW_STACK_CHIRHO] =
    [std::ptr::null_mut(); MAX_SHADOW_STACK_CHIRHO];
static SHADOW_STACK_TOP_CHIRHO: AtomicU64 = AtomicU64::new(0);

// ---------------------------------------------------------------------------
// Allocation
// ---------------------------------------------------------------------------

/// Allocate `size` bytes of heap memory for a Haskell closure/value.
/// Returns a pointer to zeroed memory. Triggers GC when allocation count
/// exceeds the threshold.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_alloc_chirho(size: u64) -> *mut u8 {
    let size_chirho = size.max(8) as usize;
    let layout_chirho = Layout::from_size_align(size_chirho, 8).unwrap();

    // Check if we should trigger GC
    let count_chirho = ALLOC_COUNT_CHIRHO.fetch_add(1, Ordering::Relaxed);
    if count_chirho > GC_THRESHOLD_CHIRHO {
        // For now, just reset the counter. Real GC will be wired here.
        ALLOC_COUNT_CHIRHO.store(0, Ordering::Relaxed);
        // TODO: call actual GC sweep
    }

    let ptr_chirho = unsafe { alloc(layout_chirho) };
    if ptr_chirho.is_null() {
        // OOM — abort
        eprintln!("haskelujah: out of memory (requested {} bytes)", size_chirho);
        std::process::abort();
    }

    // Zero the memory
    unsafe {
        std::ptr::write_bytes(ptr_chirho, 0, size_chirho);
    }

    ALLOC_TOTAL_CHIRHO.fetch_add(size_chirho as u64, Ordering::Relaxed);
    ptr_chirho
}

/// Push a GC root onto the shadow stack.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_gc_root_push_chirho(ptr: *mut u8) {
    let top_chirho = SHADOW_STACK_TOP_CHIRHO.fetch_add(1, Ordering::Relaxed) as usize;
    if top_chirho < MAX_SHADOW_STACK_CHIRHO {
        unsafe {
            SHADOW_STACK_CHIRHO[top_chirho] = ptr;
        }
    }
}

/// Pop a GC root from the shadow stack.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_gc_root_pop_chirho() {
    let _prev_chirho = SHADOW_STACK_TOP_CHIRHO.fetch_sub(1, Ordering::Relaxed);
}

/// Trigger a garbage collection cycle.
/// Currently a no-op placeholder — will be wired to the mark-sweep collector.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_gc_collect_chirho() {
    // TODO: implement actual GC using the shadow stack roots
    // For now, just report stats
    let total_chirho = ALLOC_TOTAL_CHIRHO.load(Ordering::Relaxed);
    let count_chirho = ALLOC_COUNT_CHIRHO.load(Ordering::Relaxed);
    eprintln!(
        "[GC] total allocated: {} bytes, {} allocations since last GC",
        total_chirho, count_chirho
    );
}

/// Get total bytes allocated (for debugging/profiling).
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_alloc_total_chirho() -> u64 {
    ALLOC_TOTAL_CHIRHO.load(Ordering::Relaxed)
}

// ---------------------------------------------------------------------------
// Runtime error handling
// ---------------------------------------------------------------------------

/// Runtime error: print message and abort.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_error_chirho(msg: *const u8, len: u64) {
    let slice_chirho = unsafe { std::slice::from_raw_parts(msg, len as usize) };
    if let Ok(s_chirho) = std::str::from_utf8(slice_chirho) {
        eprintln!("haskelujah: error: {}", s_chirho);
    } else {
        eprintln!("haskelujah: error (non-UTF8 message)");
    }
    std::process::abort();
}

/// Runtime panic for undefined/bottom values.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_undefined_chirho() {
    eprintln!("haskelujah: Prelude.undefined");
    std::process::abort();
}
