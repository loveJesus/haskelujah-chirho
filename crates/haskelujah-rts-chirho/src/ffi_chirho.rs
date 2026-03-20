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

use std::alloc::{Layout, alloc_zeroed, dealloc};
use std::collections::{HashMap, VecDeque};
use std::ffi::CStr;
use std::io::Write;
use std::mem::{align_of, size_of};
use std::sync::{Mutex, MutexGuard, OnceLock};

/// GC threshold: collect after this many allocations.
const GC_THRESHOLD_CHIRHO: u64 = 65536;

#[derive(Debug)]
struct NativeAllocRecordChirho {
    size_chirho: usize,
    layout_chirho: Layout,
    marked_chirho: bool,
}

#[derive(Debug, Default)]
struct NativeGcRuntimeChirho {
    allocations_by_ptr_chirho: HashMap<usize, NativeAllocRecordChirho>,
    gc_roots_chirho: Vec<usize>,
    alloc_total_chirho: u64,
    alloc_count_since_gc_chirho: u64,
}

impl NativeGcRuntimeChirho {
    fn alloc_chirho(&mut self, requested_size_chirho: u64) -> *mut u8 {
        if self.alloc_count_since_gc_chirho >= GC_THRESHOLD_CHIRHO {
            self.collect_chirho();
        }

        let size_chirho = usize::try_from(requested_size_chirho).unwrap_or(usize::MAX);
        let actual_size_chirho = size_chirho.max(size_of::<usize>());
        let Ok(layout_chirho) = Layout::from_size_align(actual_size_chirho, align_of::<usize>())
        else {
            return std::ptr::null_mut();
        };

        let ptr_chirho = unsafe { alloc_zeroed(layout_chirho) };
        if ptr_chirho.is_null() {
            eprintln!(
                "haskelujah: out of memory (requested {} bytes)",
                actual_size_chirho
            );
            std::process::abort();
        }

        self.allocations_by_ptr_chirho.insert(
            ptr_chirho as usize,
            NativeAllocRecordChirho {
                size_chirho: actual_size_chirho,
                layout_chirho,
                marked_chirho: false,
            },
        );
        self.alloc_total_chirho += actual_size_chirho as u64;
        self.alloc_count_since_gc_chirho += 1;
        ptr_chirho
    }

    fn gc_root_push_chirho(&mut self, ptr_chirho: *mut u8) {
        if !ptr_chirho.is_null() {
            self.gc_roots_chirho.push(ptr_chirho as usize);
        }
    }

    fn gc_root_pop_chirho(&mut self) {
        let _ = self.gc_roots_chirho.pop();
    }

    fn collect_chirho(&mut self) {
        self.mark_all_reachable_chirho();
        self.sweep_unreachable_chirho();
        self.clear_marks_chirho();
        self.alloc_count_since_gc_chirho = 0;
    }

    fn mark_all_reachable_chirho(&mut self) {
        let mut worklist_chirho = VecDeque::new();
        let roots_snapshot_chirho = self.gc_roots_chirho.clone();
        for root_ptr_addr_chirho in roots_snapshot_chirho {
            if self.try_mark_ptr_addr_chirho(root_ptr_addr_chirho) {
                worklist_chirho.push_back(root_ptr_addr_chirho);
            }
        }

        while let Some(ptr_addr_chirho) = worklist_chirho.pop_front() {
            let Some(record_chirho) = self.allocations_by_ptr_chirho.get(&ptr_addr_chirho) else {
                continue;
            };
            let block_ptr_chirho = ptr_addr_chirho as *const u8;
            let block_size_chirho = record_chirho.size_chirho;
            let word_size_chirho = size_of::<usize>();
            let mut offset_chirho = 0usize;

            while offset_chirho + word_size_chirho <= block_size_chirho {
                let child_ptr_addr_chirho = unsafe {
                    block_ptr_chirho
                        .add(offset_chirho)
                        .cast::<usize>()
                        .read_unaligned()
                };
                if self.try_mark_ptr_addr_chirho(child_ptr_addr_chirho) {
                    worklist_chirho.push_back(child_ptr_addr_chirho);
                }
                offset_chirho += word_size_chirho;
            }
        }
    }

    fn try_mark_ptr_addr_chirho(&mut self, ptr_addr_chirho: usize) -> bool {
        let Some(record_chirho) = self.allocations_by_ptr_chirho.get_mut(&ptr_addr_chirho) else {
            return false;
        };
        if record_chirho.marked_chirho {
            return false;
        }
        record_chirho.marked_chirho = true;
        true
    }

    fn sweep_unreachable_chirho(&mut self) {
        let dead_ptrs_chirho: Vec<usize> = self
            .allocations_by_ptr_chirho
            .iter()
            .filter_map(|(ptr_addr_chirho, record_chirho)| {
                (!record_chirho.marked_chirho).then_some(*ptr_addr_chirho)
            })
            .collect();

        for dead_ptr_addr_chirho in dead_ptrs_chirho {
            if let Some(record_chirho) =
                self.allocations_by_ptr_chirho.remove(&dead_ptr_addr_chirho)
            {
                unsafe {
                    dealloc(dead_ptr_addr_chirho as *mut u8, record_chirho.layout_chirho);
                }
            }
        }
    }

    fn clear_marks_chirho(&mut self) {
        for record_chirho in self.allocations_by_ptr_chirho.values_mut() {
            record_chirho.marked_chirho = false;
        }
    }

    fn alloc_total_chirho(&self) -> u64 {
        self.alloc_total_chirho
    }

    fn clear_all_allocs_chirho(&mut self) {
        let live_ptrs_chirho: Vec<usize> = self.allocations_by_ptr_chirho.keys().copied().collect();
        for live_ptr_addr_chirho in live_ptrs_chirho {
            if let Some(record_chirho) =
                self.allocations_by_ptr_chirho.remove(&live_ptr_addr_chirho)
            {
                unsafe {
                    dealloc(live_ptr_addr_chirho as *mut u8, record_chirho.layout_chirho);
                }
            }
        }
    }

    #[cfg(test)]
    fn reset_chirho(&mut self) {
        self.gc_roots_chirho.clear();
        self.clear_all_allocs_chirho();
        self.alloc_total_chirho = 0;
        self.alloc_count_since_gc_chirho = 0;
    }

    #[cfg(test)]
    fn allocation_count_chirho(&self) -> usize {
        self.allocations_by_ptr_chirho.len()
    }

    #[cfg(test)]
    fn contains_alloc_chirho(&self, ptr_chirho: *mut u8) -> bool {
        self.allocations_by_ptr_chirho
            .contains_key(&(ptr_chirho as usize))
    }
}

impl Drop for NativeGcRuntimeChirho {
    fn drop(&mut self) {
        self.clear_all_allocs_chirho();
    }
}

fn native_gc_runtime_chirho() -> &'static Mutex<NativeGcRuntimeChirho> {
    static NATIVE_GC_RUNTIME_CHIRHO: OnceLock<Mutex<NativeGcRuntimeChirho>> = OnceLock::new();
    NATIVE_GC_RUNTIME_CHIRHO.get_or_init(|| Mutex::new(NativeGcRuntimeChirho::default()))
}

fn native_gc_runtime_lock_chirho() -> MutexGuard<'static, NativeGcRuntimeChirho> {
    native_gc_runtime_chirho()
        .lock()
        .unwrap_or_else(|poisoned_chirho| poisoned_chirho.into_inner())
}

/// Allocate `size` bytes of heap memory for a Haskell closure/value.
/// Returns a pointer to zeroed memory. Triggers GC when allocation count
/// exceeds the threshold.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_alloc_chirho(size_chirho: u64) -> *mut u8 {
    native_gc_runtime_lock_chirho().alloc_chirho(size_chirho)
}

/// Push a GC root onto the shadow stack.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_gc_root_push_chirho(ptr_chirho: *mut u8) {
    native_gc_runtime_lock_chirho().gc_root_push_chirho(ptr_chirho);
}

/// Pop a GC root from the shadow stack.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_gc_root_pop_chirho() {
    native_gc_runtime_lock_chirho().gc_root_pop_chirho();
}

/// Trigger a garbage collection cycle over native-code heap blocks.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_gc_collect_chirho() {
    native_gc_runtime_lock_chirho().collect_chirho();
}

/// Get total bytes allocated (for debugging/profiling).
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_alloc_total_chirho() -> u64 {
    native_gc_runtime_lock_chirho().alloc_total_chirho()
}

/// Print an integer value followed by a newline for native backends that avoid
/// variadic libc calls.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_print_int_chirho(value_chirho: i64) -> i64 {
    println!("{value_chirho}");
    let _ = std::io::stdout().flush();
    0
}

/// Print a NUL-terminated UTF-8 string followed by a newline using the same
/// stdout implementation as `haskelujah_print_int_chirho`, keeping IO order
/// stable for Cranelift executables.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_put_str_ln_chirho(ptr_bits_chirho: u64) -> i64 {
    if ptr_bits_chirho == 0 {
        println!();
        let _ = std::io::stdout().flush();
        return 0;
    }

    let ptr_chirho = ptr_bits_chirho as usize as *const std::ffi::c_char;
    let c_str_chirho = unsafe { CStr::from_ptr(ptr_chirho) };
    let text_chirho = c_str_chirho.to_string_lossy();
    println!("{text_chirho}");
    let _ = std::io::stdout().flush();
    0
}

/// Concatenate two NUL-terminated byte strings into a newly allocated
/// NUL-terminated buffer owned by the native RTS allocator.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_append_str_chirho(
    lhs_bits_chirho: u64,
    rhs_bits_chirho: u64,
) -> u64 {
    let lhs_bytes_chirho: &[u8] = if lhs_bits_chirho == 0 {
        &[]
    } else {
        let lhs_ptr_chirho = lhs_bits_chirho as usize as *const std::ffi::c_char;
        unsafe { CStr::from_ptr(lhs_ptr_chirho) }.to_bytes()
    };
    let rhs_bytes_chirho: &[u8] = if rhs_bits_chirho == 0 {
        &[]
    } else {
        let rhs_ptr_chirho = rhs_bits_chirho as usize as *const std::ffi::c_char;
        unsafe { CStr::from_ptr(rhs_ptr_chirho) }.to_bytes()
    };

    let total_len_chirho = lhs_bytes_chirho
        .len()
        .saturating_add(rhs_bytes_chirho.len())
        .saturating_add(1);
    let out_ptr_chirho = haskelujah_alloc_chirho(total_len_chirho as u64);
    if out_ptr_chirho.is_null() {
        return 0;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(
            lhs_bytes_chirho.as_ptr(),
            out_ptr_chirho,
            lhs_bytes_chirho.len(),
        );
        std::ptr::copy_nonoverlapping(
            rhs_bytes_chirho.as_ptr(),
            out_ptr_chirho.add(lhs_bytes_chirho.len()),
            rhs_bytes_chirho.len(),
        );
        out_ptr_chirho
            .add(lhs_bytes_chirho.len() + rhs_bytes_chirho.len())
            .write(0);
    }

    out_ptr_chirho as usize as u64
}

// ---------------------------------------------------------------------------
// Runtime error handling
// ---------------------------------------------------------------------------

/// Runtime error: print message and abort.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_error_chirho(msg_chirho: *const u8, len_chirho: u64) {
    let slice_chirho = unsafe { std::slice::from_raw_parts(msg_chirho, len_chirho as usize) };
    if let Ok(message_chirho) = std::str::from_utf8(slice_chirho) {
        eprintln!("haskelujah: error: {}", message_chirho);
    } else {
        eprintln!("haskelujah: error (non-UTF8 message)");
    }
    std::process::abort();
}

/// Print a boolean as True/False followed by newline.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_print_bool_chirho(b_chirho: i64) {
    if b_chirho != 0 {
        println!("True");
    } else {
        println!("False");
    }
}

/// Runtime panic for undefined/bottom values.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_undefined_chirho() {
    eprintln!("haskelujah: Prelude.undefined");
    std::process::abort();
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn ffi_test_lock_chirho() -> &'static Mutex<()> {
        static FFI_TEST_LOCK_CHIRHO: OnceLock<Mutex<()>> = OnceLock::new();
        FFI_TEST_LOCK_CHIRHO.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn rooted_block_survives_collection_chirho() {
        let _guard_chirho = ffi_test_lock_chirho()
            .lock()
            .unwrap_or_else(|poisoned_chirho| poisoned_chirho.into_inner());
        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
        drop(runtime_chirho);

        let root_ptr_chirho = haskelujah_alloc_chirho(16);
        assert!(!root_ptr_chirho.is_null());
        haskelujah_gc_root_push_chirho(root_ptr_chirho);
        haskelujah_gc_collect_chirho();

        let runtime_chirho = native_gc_runtime_lock_chirho();
        assert_eq!(runtime_chirho.allocation_count_chirho(), 1);
        assert!(runtime_chirho.contains_alloc_chirho(root_ptr_chirho));
        drop(runtime_chirho);

        haskelujah_gc_root_pop_chirho();
        haskelujah_gc_collect_chirho();

        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        assert_eq!(runtime_chirho.allocation_count_chirho(), 0);
        runtime_chirho.reset_chirho();
    }

    #[test]
    fn reachable_child_survives_via_parent_scan_chirho() {
        let _guard_chirho = ffi_test_lock_chirho()
            .lock()
            .unwrap_or_else(|poisoned_chirho| poisoned_chirho.into_inner());
        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
        drop(runtime_chirho);

        let parent_ptr_chirho = haskelujah_alloc_chirho(16);
        let child_ptr_chirho = haskelujah_alloc_chirho(16);
        assert!(!parent_ptr_chirho.is_null());
        assert!(!child_ptr_chirho.is_null());

        unsafe {
            parent_ptr_chirho
                .cast::<usize>()
                .write_unaligned(child_ptr_chirho as usize);
        }

        haskelujah_gc_root_push_chirho(parent_ptr_chirho);
        haskelujah_gc_collect_chirho();

        let runtime_chirho = native_gc_runtime_lock_chirho();
        assert_eq!(runtime_chirho.allocation_count_chirho(), 2);
        assert!(runtime_chirho.contains_alloc_chirho(parent_ptr_chirho));
        assert!(runtime_chirho.contains_alloc_chirho(child_ptr_chirho));
        drop(runtime_chirho);

        haskelujah_gc_root_pop_chirho();
        haskelujah_gc_collect_chirho();

        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        assert_eq!(runtime_chirho.allocation_count_chirho(), 0);
        runtime_chirho.reset_chirho();
    }
}
