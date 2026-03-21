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
use std::thread;

/// GC threshold: disabled until proper root tracking is implemented.
/// Without GC roots, the collector frees live cons cells and corrupts data.
/// Programs will leak memory but produce correct results.
const GC_THRESHOLD_CHIRHO: u64 = u64::MAX;
const NATIVE_MAIN_STACK_SIZE_CHIRHO: usize = 1024 * 1024 * 1024; // 1GB

type NativeEntryFnChirho = unsafe extern "C" fn() -> i64;

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

// ---------------------------------------------------------------------------
// Thunk operations for lazy evaluation
// ---------------------------------------------------------------------------
//
// Header layout (matches runtime_layout_chirho.rs):
//   [63..8]  info-table pointer / entry code address
//   [7..4]   thunk state: 0=unevaluated, 1=blackhole, 2=evaluated(ind)
//   [3..2]   GC mark bits
//   [1..0]   object kind: 00=thunk, 01=fun, 10=con, 11=pap
//
// Thunk-specific header:
//   [63..8]  code_ptr (unevaluated) or result_ptr (evaluated)
//   [7..4]   state: 0=thunk, 1=blackhole, 2=indirection
//   [1..0]   0b00 (ThunkChirho kind)

const THUNK_STATE_UNEVALUATED_CHIRHO: u64 = 0 << 4;
const THUNK_STATE_BLACKHOLE_CHIRHO: u64 = 1 << 4;
const THUNK_STATE_INDIRECTION_CHIRHO: u64 = 2 << 4;
const THUNK_STATE_MASK_CHIRHO: u64 = 0xF0;
const KIND_THUNK_CHIRHO: u64 = 0b00;

/// Allocate a thunk on the heap.
///
/// Layout: [header (8 bytes)] [fv_0] [fv_1] ... [fv_n]
/// Header: (code_ptr << 8) | state | kind
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_alloc_thunk_chirho(
    code_ptr_chirho: u64,
    num_fvs_chirho: u64,
    fvs_chirho: *const u64,
) -> u64 {
    let total_size_chirho = 8 + num_fvs_chirho * 8;
    let ptr_chirho = haskelujah_alloc_chirho(total_size_chirho);
    if ptr_chirho.is_null() {
        return 0;
    }
    unsafe {
        let header_chirho = ptr_chirho as *mut u64;
        // Header: code_ptr in high bits, state=unevaluated, kind=thunk
        *header_chirho =
            (code_ptr_chirho << 8) | THUNK_STATE_UNEVALUATED_CHIRHO | KIND_THUNK_CHIRHO;

        if num_fvs_chirho > 0 && !fvs_chirho.is_null() {
            let fvs_dest_chirho = header_chirho.add(1);
            std::ptr::copy_nonoverlapping(
                fvs_chirho,
                fvs_dest_chirho,
                num_fvs_chirho as usize,
            );
        }
    }
    // Set high bit to mark as heap pointer
    (ptr_chirho as u64) | (1u64 << 63)
}

/// Enter (force) a thunk.
///
/// Checks the thunk state in the header:
/// - Unevaluated: blackhole it, call code, update to indirection
/// - Indirection: return the stored result
/// - Blackhole: abort (infinite loop)
/// - Not a thunk: return as-is
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_enter_thunk_chirho(thunk_ptr_chirho: u64) -> u64 {
    let raw_ptr_chirho = (thunk_ptr_chirho & !(1u64 << 63)) as *mut u64;
    if raw_ptr_chirho.is_null() {
        return 0;
    }

    unsafe {
        let header_chirho = *raw_ptr_chirho;
        let kind_chirho = header_chirho & 0b11;

        if kind_chirho != KIND_THUNK_CHIRHO {
            // Not a thunk — return as-is
            return thunk_ptr_chirho;
        }

        let state_chirho = header_chirho & THUNK_STATE_MASK_CHIRHO;

        if state_chirho == THUNK_STATE_INDIRECTION_CHIRHO {
            // Already evaluated — return result
            return header_chirho >> 8;
        }

        if state_chirho == THUNK_STATE_BLACKHOLE_CHIRHO {
            eprintln!("runtime error: thunk blackhole (infinite loop)");
            std::process::abort();
        }

        // Unevaluated thunk: blackhole, call code, update
        let code_addr_chirho = header_chirho >> 8;
        *raw_ptr_chirho = (code_addr_chirho << 8) | THUNK_STATE_BLACKHOLE_CHIRHO | KIND_THUNK_CHIRHO;

        let fvs_ptr_chirho = raw_ptr_chirho.add(1);
        let code_fn_chirho: unsafe extern "C" fn(*const u64) -> u64 =
            std::mem::transmute(code_addr_chirho as usize);
        let result_chirho = code_fn_chirho(fvs_ptr_chirho);

        // Update to indirection
        *raw_ptr_chirho = (result_chirho << 8) | THUNK_STATE_INDIRECTION_CHIRHO | KIND_THUNK_CHIRHO;

        result_chirho
    }
}

/// Update a thunk in place with the computed result.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_update_thunk_chirho(
    thunk_ptr_chirho: u64,
    result_chirho: u64,
) {
    let raw_ptr_chirho = (thunk_ptr_chirho & !(1u64 << 63)) as *mut u64;
    if !raw_ptr_chirho.is_null() {
        unsafe {
            *raw_ptr_chirho =
                (result_chirho << 8) | THUNK_STATE_INDIRECTION_CHIRHO | KIND_THUNK_CHIRHO;
        }
    }
}

/// Run a native backend entry function on a worker thread with an explicitly
/// large stack so non-tail-recursive list code does not immediately exhaust the
/// relatively small default macOS main-thread stack.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_main_with_large_stack_chirho(entry_fn_bits_chirho: u64) -> i64 {
    if entry_fn_bits_chirho == 0 {
        eprintln!("haskelujah: null native entry function");
        std::process::abort();
    }

    let entry_fn_chirho: NativeEntryFnChirho =
        unsafe { std::mem::transmute(entry_fn_bits_chirho as usize) };
    let join_handle_chirho = thread::Builder::new()
        .name("haskelujah-main-chirho".to_string())
        .stack_size(NATIVE_MAIN_STACK_SIZE_CHIRHO)
        .spawn(move || unsafe { entry_fn_chirho() })
        .unwrap_or_else(|error_chirho| {
            eprintln!("haskelujah: failed to spawn main thread: {error_chirho}");
            std::process::abort();
        });

    match join_handle_chirho.join() {
        Ok(result_chirho) => result_chirho,
        Err(_) => {
            eprintln!("haskelujah: native main thread panicked");
            std::process::abort();
        }
    }
}

/// Print an integer value followed by a newline for native backends that avoid
/// variadic libc calls.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_print_int_chirho(value_chirho: i64) -> i64 {
    println!("{value_chirho}");
    let _ = std::io::stdout().flush();
    0
}

/// Convert an integer to a heap-allocated NUL-terminated decimal string.
/// Returns a pointer that can be passed to `haskelujah_put_str_ln_chirho` or
/// used as a Haskell `String` value.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_show_int_chirho(value_chirho: i64) -> u64 {
    let s_chirho = format!("{value_chirho}");
    alloc_c_string_chirho(s_chirho.as_bytes())
}

fn alloc_c_string_chirho(bytes_chirho: &[u8]) -> u64 {
    let alloc_size_chirho = bytes_chirho.len() + 1;
    let ptr_chirho = haskelujah_alloc_chirho(alloc_size_chirho as u64);
    if !ptr_chirho.is_null() {
        unsafe {
            std::ptr::copy_nonoverlapping(bytes_chirho.as_ptr(), ptr_chirho, bytes_chirho.len());
            *ptr_chirho.add(bytes_chirho.len()) = 0;
        }
    }
    ptr_chirho as u64
}

/// Convert a boolean to a heap-allocated NUL-terminated `True`/`False` string.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_show_bool_chirho(value_chirho: i64) -> u64 {
    let text_chirho: &[u8] = if value_chirho != 0 {
        b"True".as_slice()
    } else {
        b"False".as_slice()
    };
    alloc_c_string_chirho(text_chirho)
}

/// Convert a character codepoint to a heap-allocated NUL-terminated Haskell
/// `show` representation like `'A'`.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_show_char_chirho(value_chirho: i64) -> u64 {
    let char_chirho = char::from_u32(value_chirho as u32).unwrap_or(char::REPLACEMENT_CHARACTER);
    let text_chirho = format!("{char_chirho:?}");
    alloc_c_string_chirho(text_chirho.as_bytes())
}

/// Convert an f64 bit-pattern to a heap-allocated NUL-terminated decimal
/// string for Haskell `show`.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_show_float_chirho(bits_chirho: i64) -> u64 {
    let value_chirho = f64::from_bits(bits_chirho as u64);
    let text_chirho = format!("{value_chirho}");
    alloc_c_string_chirho(text_chirho.as_bytes())
}

/// Convert a Haskell `[Int]` cons-list to a heap-allocated string like
/// `[1,2,3]`.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_show_int_list_chirho(list_bits_chirho: u64) -> u64 {
    let mut text_chirho = String::from("[");
    let mut current_chirho = list_bits_chirho;
    let mut first_elem_chirho = true;

    loop {
        if current_chirho == 0 {
            break;
        }
        if current_chirho & (1u64 << 63) == 0 {
            break;
        }

        let ptr_chirho = (current_chirho & 0x7FFFFFFFFFFFFFFF) as *const u64;
        let tag_chirho = unsafe { *ptr_chirho };
        if tag_chirho != 1 {
            break;
        }

        let head_chirho = unsafe { *ptr_chirho.add(1) } as i64;
        let tail_chirho = unsafe { *ptr_chirho.add(2) };
        if !first_elem_chirho {
            text_chirho.push(',');
        }
        first_elem_chirho = false;
        text_chirho.push_str(&head_chirho.to_string());
        current_chirho = tail_chirho;
    }

    text_chirho.push(']');
    alloc_c_string_chirho(text_chirho.as_bytes())
}

/// Convert a Haskell `[Bool]` cons-list to a heap-allocated string like
/// `[True,False,True]`.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_show_bool_list_chirho(list_bits_chirho: u64) -> u64 {
    let mut text_chirho = String::from("[");
    let mut current_chirho = list_bits_chirho;
    let mut first_elem_chirho = true;

    loop {
        if current_chirho == 0 {
            break;
        }
        if current_chirho & (1u64 << 63) == 0 {
            break;
        }

        let ptr_chirho = (current_chirho & 0x7FFFFFFFFFFFFFFF) as *const u64;
        let tag_chirho = unsafe { *ptr_chirho };
        if tag_chirho != 1 {
            break;
        }

        let head_chirho = unsafe { *ptr_chirho.add(1) };
        let tail_chirho = unsafe { *ptr_chirho.add(2) };
        if !first_elem_chirho {
            text_chirho.push(',');
        }
        first_elem_chirho = false;
        text_chirho.push_str(if head_chirho == 0 { "False" } else { "True" });
        current_chirho = tail_chirho;
    }

    text_chirho.push(']');
    alloc_c_string_chirho(text_chirho.as_bytes())
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
pub extern "C" fn haskelujah_append_str_chirho(lhs_bits_chirho: u64, rhs_bits_chirho: u64) -> u64 {
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

/// Read a line from stdin, returning a heap-allocated NUL-terminated string.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_get_line_chirho() -> u64 {
    let mut line_chirho = String::new();
    let _ = std::io::stdin().read_line(&mut line_chirho);
    // Remove trailing newline
    if line_chirho.ends_with('\n') {
        line_chirho.pop();
        if line_chirho.ends_with('\r') {
            line_chirho.pop();
        }
    }
    alloc_c_string_chirho(line_chirho.as_bytes())
}

/// Print a NUL-terminated string WITHOUT a trailing newline.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_put_str_chirho(ptr_bits_chirho: u64) -> i64 {
    if ptr_bits_chirho == 0 {
        return 0;
    }
    let ptr_chirho = ptr_bits_chirho as usize as *const std::ffi::c_char;
    let c_str_chirho = unsafe { CStr::from_ptr(ptr_chirho) };
    let text_chirho = c_str_chirho.to_string_lossy();
    print!("{text_chirho}");
    let _ = std::io::stdout().flush();
    0
}

/// Write a NUL-terminated string to a file. Both path and content are
/// NUL-terminated heap strings. Returns 0 on success.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_write_file_chirho(
    path_bits_chirho: u64,
    content_bits_chirho: u64,
) -> i64 {
    let path_chirho = if path_bits_chirho == 0 {
        return -1;
    } else {
        let ptr_chirho = path_bits_chirho as usize as *const std::ffi::c_char;
        unsafe { CStr::from_ptr(ptr_chirho) }
            .to_string_lossy()
            .into_owned()
    };
    let content_chirho = if content_bits_chirho == 0 {
        String::new()
    } else {
        let ptr_chirho = content_bits_chirho as usize as *const std::ffi::c_char;
        unsafe { CStr::from_ptr(ptr_chirho) }
            .to_string_lossy()
            .into_owned()
    };
    match std::fs::write(&path_chirho, &content_chirho) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

/// Read an entire file into a heap-allocated NUL-terminated string.
/// Returns 0 (null) on error.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_read_file_chirho(path_bits_chirho: u64) -> u64 {
    let path_chirho = if path_bits_chirho == 0 {
        return 0;
    } else {
        let ptr_chirho = path_bits_chirho as usize as *const std::ffi::c_char;
        unsafe { CStr::from_ptr(ptr_chirho) }
            .to_string_lossy()
            .into_owned()
    };
    match std::fs::read_to_string(&path_chirho) {
        Ok(content_chirho) => alloc_c_string_chirho(content_chirho.as_bytes()),
        Err(_) => 0,
    }
}

/// Parse a NUL-terminated UTF-8 string as an `Int`.
/// Returns 0 on parse failure.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_read_int_chirho(text_bits_chirho: u64) -> i64 {
    if text_bits_chirho == 0 {
        return 0;
    }
    let ptr_chirho = text_bits_chirho as usize as *const std::ffi::c_char;
    let text_chirho = unsafe { CStr::from_ptr(ptr_chirho) }.to_string_lossy();
    text_chirho.trim().parse::<i64>().unwrap_or(0)
}

/// Convert a Haskell [Char] cons-list back to a NUL-terminated C string.
/// Traverses the cons list collecting character codepoints into a buffer.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_pack_string_chirho(list_bits_chirho: u64) -> u64 {
    let mut chars_chirho: Vec<u8> = Vec::new();
    let mut current_chirho = list_bits_chirho;

    loop {
        // Check if boxed (high bit set)
        if current_chirho == 0 {
            break; // Nil (immediate 0)
        }
        if current_chirho & (1u64 << 63) == 0 {
            // Immediate value — tag 0 = Nil
            break;
        }
        // Unbox: clear high bit to get real pointer
        let ptr_chirho = (current_chirho & 0x7FFFFFFFFFFFFFFF) as *const u64;
        let tag_chirho = unsafe { *ptr_chirho };
        if tag_chirho != 1 {
            break; // Not Cons
        }
        let head_chirho = unsafe { *ptr_chirho.add(1) };
        let tail_chirho = unsafe { *ptr_chirho.add(2) };

        // Head is a character codepoint
        if head_chirho < 128 {
            chars_chirho.push(head_chirho as u8);
        } else {
            chars_chirho.push(b'?'); // Non-ASCII placeholder
        }
        current_chirho = tail_chirho;
    }

    alloc_c_string_chirho(&chars_chirho)
}

/// Convert a NUL-terminated C string to a Haskell [Char] cons-list.
/// Each character becomes a cons cell: tag=1, head=codepoint, tail=next.
/// The empty string returns tag=0 (Nil).
/// Layout: [tag:i64, head:i64, tail:i64] per cons cell (24 bytes).
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_unpack_string_chirho(str_bits_chirho: u64) -> u64 {
    if str_bits_chirho == 0 {
        return 0; // Nil
    }
    let ptr_chirho = str_bits_chirho as usize as *const u8;
    let c_str_chirho = unsafe { CStr::from_ptr(ptr_chirho as *const std::ffi::c_char) };
    let bytes_chirho = c_str_chirho.to_bytes();

    // Build the list from the END (so we can chain tails)
    let mut tail_chirho: u64 = 0; // Nil tag
    for &byte_chirho in bytes_chirho.iter().rev() {
        let cell_ptr_chirho = haskelujah_alloc_chirho(24);
        if cell_ptr_chirho.is_null() {
            return 0;
        }
        unsafe {
            // tag = 1 (Cons)
            *(cell_ptr_chirho as *mut u64) = 1;
            // head = character codepoint
            *(cell_ptr_chirho.add(8) as *mut u64) = byte_chirho as u64;
            // tail = previous cell (boxed) or 0 (Nil)
            *(cell_ptr_chirho.add(16) as *mut u64) = tail_chirho;
        }
        // Box the pointer (set high bit)
        tail_chirho = (cell_ptr_chirho as u64) | (1u64 << 63);
    }
    tail_chirho
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
    use std::ffi::CStr;

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

    #[test]
    fn show_bool_allocates_true_and_false_strings_chirho() {
        let _guard_chirho = ffi_test_lock_chirho()
            .lock()
            .unwrap_or_else(|poisoned_chirho| poisoned_chirho.into_inner());
        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
        drop(runtime_chirho);

        let true_ptr_bits_chirho = haskelujah_show_bool_chirho(1);
        let false_ptr_bits_chirho = haskelujah_show_bool_chirho(0);
        assert_ne!(true_ptr_bits_chirho, 0);
        assert_ne!(false_ptr_bits_chirho, 0);

        let true_text_chirho =
            unsafe { CStr::from_ptr(true_ptr_bits_chirho as usize as *const std::ffi::c_char) };
        let false_text_chirho =
            unsafe { CStr::from_ptr(false_ptr_bits_chirho as usize as *const std::ffi::c_char) };
        assert_eq!(true_text_chirho.to_bytes(), b"True");
        assert_eq!(false_text_chirho.to_bytes(), b"False");

        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
    }

    #[test]
    fn show_char_allocates_quoted_character_string_chirho() {
        let _guard_chirho = ffi_test_lock_chirho()
            .lock()
            .unwrap_or_else(|poisoned_chirho| poisoned_chirho.into_inner());
        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
        drop(runtime_chirho);

        let ptr_bits_chirho = haskelujah_show_char_chirho('A' as i64);
        assert_ne!(ptr_bits_chirho, 0);
        let text_chirho =
            unsafe { CStr::from_ptr(ptr_bits_chirho as usize as *const std::ffi::c_char) };
        assert_eq!(text_chirho.to_bytes(), b"'A'");

        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
    }

    #[test]
    fn show_float_allocates_decimal_string_chirho() {
        let _guard_chirho = ffi_test_lock_chirho()
            .lock()
            .unwrap_or_else(|poisoned_chirho| poisoned_chirho.into_inner());
        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
        drop(runtime_chirho);

        let ptr_bits_chirho = haskelujah_show_float_chirho(3.14f64.to_bits() as i64);
        assert_ne!(ptr_bits_chirho, 0);
        let text_chirho =
            unsafe { CStr::from_ptr(ptr_bits_chirho as usize as *const std::ffi::c_char) };
        assert_eq!(text_chirho.to_bytes(), b"3.14");

        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
    }

    #[test]
    fn read_int_parses_trimmed_decimal_string_chirho() {
        let _guard_chirho = ffi_test_lock_chirho()
            .lock()
            .unwrap_or_else(|poisoned_chirho| poisoned_chirho.into_inner());
        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
        drop(runtime_chirho);

        let ptr_bits_chirho = alloc_c_string_chirho(b"  -42  ");
        assert_ne!(ptr_bits_chirho, 0);
        assert_eq!(haskelujah_read_int_chirho(ptr_bits_chirho), -42);

        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
    }

    #[test]
    fn show_int_list_allocates_bracketed_values_chirho() {
        let _guard_chirho = ffi_test_lock_chirho()
            .lock()
            .unwrap_or_else(|poisoned_chirho| poisoned_chirho.into_inner());
        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
        drop(runtime_chirho);

        let nil_chirho = 0u64;
        let cell3_ptr_chirho = haskelujah_alloc_chirho(24);
        let cell2_ptr_chirho = haskelujah_alloc_chirho(24);
        let cell1_ptr_chirho = haskelujah_alloc_chirho(24);
        assert!(!cell1_ptr_chirho.is_null());
        assert!(!cell2_ptr_chirho.is_null());
        assert!(!cell3_ptr_chirho.is_null());

        unsafe {
            *(cell3_ptr_chirho as *mut u64) = 1;
            *(cell3_ptr_chirho.add(8) as *mut u64) = 3;
            *(cell3_ptr_chirho.add(16) as *mut u64) = nil_chirho;

            *(cell2_ptr_chirho as *mut u64) = 1;
            *(cell2_ptr_chirho.add(8) as *mut u64) = 2;
            *(cell2_ptr_chirho.add(16) as *mut u64) = (cell3_ptr_chirho as u64) | (1u64 << 63);

            *(cell1_ptr_chirho as *mut u64) = 1;
            *(cell1_ptr_chirho.add(8) as *mut u64) = 1;
            *(cell1_ptr_chirho.add(16) as *mut u64) = (cell2_ptr_chirho as u64) | (1u64 << 63);
        }

        let shown_ptr_bits_chirho =
            haskelujah_show_int_list_chirho((cell1_ptr_chirho as u64) | (1u64 << 63));
        assert_ne!(shown_ptr_bits_chirho, 0);
        let shown_text_chirho =
            unsafe { CStr::from_ptr(shown_ptr_bits_chirho as usize as *const std::ffi::c_char) };
        assert_eq!(shown_text_chirho.to_bytes(), b"[1,2,3]");

        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
    }

    #[test]
    fn show_bool_list_allocates_bracketed_values_chirho() {
        let _guard_chirho = ffi_test_lock_chirho()
            .lock()
            .unwrap_or_else(|poisoned_chirho| poisoned_chirho.into_inner());
        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
        drop(runtime_chirho);

        let nil_chirho = 0u64;
        let cell3_ptr_chirho = haskelujah_alloc_chirho(24);
        let cell2_ptr_chirho = haskelujah_alloc_chirho(24);
        let cell1_ptr_chirho = haskelujah_alloc_chirho(24);
        assert!(!cell1_ptr_chirho.is_null());
        assert!(!cell2_ptr_chirho.is_null());
        assert!(!cell3_ptr_chirho.is_null());

        unsafe {
            *(cell3_ptr_chirho as *mut u64) = 1;
            *(cell3_ptr_chirho.add(8) as *mut u64) = 1;
            *(cell3_ptr_chirho.add(16) as *mut u64) = nil_chirho;

            *(cell2_ptr_chirho as *mut u64) = 1;
            *(cell2_ptr_chirho.add(8) as *mut u64) = 0;
            *(cell2_ptr_chirho.add(16) as *mut u64) = (cell3_ptr_chirho as u64) | (1u64 << 63);

            *(cell1_ptr_chirho as *mut u64) = 1;
            *(cell1_ptr_chirho.add(8) as *mut u64) = 1;
            *(cell1_ptr_chirho.add(16) as *mut u64) = (cell2_ptr_chirho as u64) | (1u64 << 63);
        }

        let shown_ptr_bits_chirho =
            haskelujah_show_bool_list_chirho((cell1_ptr_chirho as u64) | (1u64 << 63));
        assert_ne!(shown_ptr_bits_chirho, 0);
        let shown_text_chirho =
            unsafe { CStr::from_ptr(shown_ptr_bits_chirho as usize as *const std::ffi::c_char) };
        assert_eq!(shown_text_chirho.to_bytes(), b"[True,False,True]");

        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
    }

    unsafe extern "C" fn ffi_test_entry_chirho() -> i64 {
        42
    }

    fn deep_stack_recurse_chirho(depth_chirho: usize) -> i64 {
        let stack_buf_chirho = [0u8; 64 * 1024];
        let next_acc_chirho = i64::from(stack_buf_chirho[0]);
        if depth_chirho == 0 {
            next_acc_chirho
        } else {
            next_acc_chirho + deep_stack_recurse_chirho(depth_chirho - 1)
        }
    }

    unsafe extern "C" fn ffi_test_deep_stack_entry_chirho() -> i64 {
        deep_stack_recurse_chirho(255)
    }

    #[test]
    fn main_with_large_stack_runs_entry_function_chirho() {
        let result_chirho = haskelujah_main_with_large_stack_chirho(
            ffi_test_entry_chirho as *const () as usize as u64,
        );
        assert_eq!(result_chirho, 42);
    }

    #[test]
    fn main_with_large_stack_handles_deep_stack_recursion_chirho() {
        let result_chirho = haskelujah_main_with_large_stack_chirho(
            ffi_test_deep_stack_entry_chirho as *const () as usize as u64,
        );
        assert_eq!(result_chirho, 0);
    }

    #[test]
    fn unpack_string_builds_cons_list_chirho() {
        let _guard_chirho = ffi_test_lock_chirho()
            .lock()
            .unwrap_or_else(|poisoned_chirho| poisoned_chirho.into_inner());
        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
        drop(runtime_chirho);

        let test_str_chirho = b"Hi\0";
        let result_chirho =
            haskelujah_unpack_string_chirho(test_str_chirho.as_ptr() as u64);

        // Result should be a boxed cons cell (high bit set)
        assert_ne!(result_chirho, 0, "unpack should not return Nil for non-empty string");
        assert!(
            result_chirho & (1u64 << 63) != 0,
            "result should be boxed (high bit set)"
        );

        // Decode first cons cell: tag=1, head='H'=72, tail=boxed
        let ptr_chirho = (result_chirho & 0x7FFFFFFFFFFFFFFF) as *const u64;
        unsafe {
            let tag_chirho = *ptr_chirho;
            let head_chirho = *ptr_chirho.add(1);
            assert_eq!(tag_chirho, 1, "first cell tag should be Cons (1)");
            assert_eq!(head_chirho, b'H' as u64, "first char should be 'H'");
        }

        let mut runtime_chirho = native_gc_runtime_lock_chirho();
        runtime_chirho.reset_chirho();
    }
}
