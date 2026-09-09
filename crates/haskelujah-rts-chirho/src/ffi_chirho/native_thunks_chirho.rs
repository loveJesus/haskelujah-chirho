// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Native thunk entry and memoization. State and the full-width code/result
//! payload occupy separate words. Enter follows indirections to WHNF, updates
//! the entire path, and roots active evaluations across allocating callbacks.

use super::{haskelujah_alloc_chirho, native_gc_runtime_lock_chirho};

const UNEVALUATED_CHIRHO: u64 = 0;
const BLACKHOLE_CHIRHO: u64 = 1;
const INDIRECTION_CHIRHO: u64 = 2;
const PREFIX_WORDS_CHIRHO: usize = 2;

/// Allocate [state, full-width entry/result, free variables...]. Generated
/// trampolines receive the free-variable slice, so they do not depend on this layout.
///
/// # Safety
/// `code_ptr_chirho` must name a valid `unsafe extern "C" fn(*const u64) -> u64`
/// for the thunk's lifetime. If `num_fvs_chirho` is nonzero, `fvs_chirho` must
/// point to that many initialized, readable `u64` values throughout this call.
/// Heap values reachable only through the environment must be rooted across
/// allocation; the callback must obey the native runtime's rooting protocol.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haskelujah_alloc_thunk_chirho(
    code_ptr_chirho: u64,
    num_fvs_chirho: u64,
    fvs_chirho: *const u64,
) -> u64 {
    let size_chirho = num_fvs_chirho
        .checked_add(PREFIX_WORDS_CHIRHO as u64)
        .and_then(|words_chirho| words_chirho.checked_mul(8))
        .unwrap_or_else(|| {
            eprintln!("haskelujah: thunk allocation overflow");
            std::process::abort()
        });
    if code_ptr_chirho == 0 || (num_fvs_chirho != 0 && fvs_chirho.is_null()) {
        eprintln!("haskelujah: invalid thunk entry or environment");
        std::process::abort();
    }
    let raw_chirho = haskelujah_alloc_chirho(size_chirho).cast::<u64>();
    if raw_chirho.is_null() {
        std::process::abort();
    }
    native_gc_runtime_lock_chirho()
        .allocations_by_ptr_chirho
        .get_mut(&(raw_chirho as usize))
        .expect("new allocation registered")
        .is_native_thunk_chirho = true;
    unsafe {
        raw_chirho.write(UNEVALUATED_CHIRHO);
        raw_chirho.add(1).write(code_ptr_chirho);
        if num_fvs_chirho != 0 {
            std::ptr::copy_nonoverlapping(
                fvs_chirho,
                raw_chirho.add(PREFIX_WORDS_CHIRHO),
                num_fvs_chirho as usize,
            );
        }
    }
    raw_chirho as u64 | 1
}

fn thunk_address_chirho(value_chirho: u64) -> Option<*mut u64> {
    if value_chirho & 1 == 0 {
        return None;
    }
    let runtime_chirho = native_gc_runtime_lock_chirho();
    let address_chirho = runtime_chirho.registered_heap_address_chirho(value_chirho as usize)?;
    runtime_chirho
        .allocations_by_ptr_chirho
        .get(&address_chirho)
        .filter(|record_chirho| record_chirho.is_native_thunk_chirho)
        .map(|_| address_chirho as *mut u64)
}

/// Demand WHNF, not just one callback result. Active thunk roots use a separate
/// set because generated callbacks also push/pop their own shadow-stack roots.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_enter_thunk_chirho(mut value_chirho: u64) -> u64 {
    let mut path_chirho = Vec::new();
    while let Some(raw_chirho) = thunk_address_chirho(value_chirho) {
        unsafe {
            match raw_chirho.read() {
                BLACKHOLE_CHIRHO => {
                    eprintln!("runtime error: thunk blackhole (infinite loop)");
                    std::process::abort();
                }
                UNEVALUATED_CHIRHO | INDIRECTION_CHIRHO => {
                    let state_chirho = raw_chirho.read();
                    let payload_chirho = raw_chirho.add(1).read();
                    native_gc_runtime_lock_chirho()
                        .active_thunks_chirho
                        .insert(raw_chirho as usize);
                    path_chirho.push(raw_chirho);
                    raw_chirho.write(BLACKHOLE_CHIRHO);
                    value_chirho = if state_chirho == INDIRECTION_CHIRHO {
                        payload_chirho
                    } else {
                        let code_chirho: unsafe extern "C" fn(*const u64) -> u64 =
                            std::mem::transmute(payload_chirho as usize);
                        code_chirho(raw_chirho.add(PREFIX_WORDS_CHIRHO))
                    };
                }
                _ => {
                    eprintln!("haskelujah: corrupt native thunk state");
                    std::process::abort();
                }
            }
        }
    }
    // The overwhelmingly common case is an immediate already in WHNF. There
    // are no updates or active roots to retire, so do not lock the collector.
    if path_chirho.is_empty() {
        return value_chirho;
    }
    let mut runtime_chirho = native_gc_runtime_lock_chirho();
    for raw_chirho in path_chirho {
        unsafe {
            raw_chirho.add(1).write(value_chirho);
            raw_chirho.write(INDIRECTION_CHIRHO);
        }
        runtime_chirho
            .active_thunks_chirho
            .remove(&(raw_chirho as usize));
    }
    value_chirho
}

/// Store an indirection without truncating signed integers, floating-point
/// payloads, or addresses. Entry will chase it if the result is another thunk.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_update_thunk_chirho(thunk_chirho: u64, value_chirho: u64) {
    let Some(raw_chirho) = thunk_address_chirho(thunk_chirho) else {
        eprintln!("haskelujah: update of a non-thunk");
        std::process::abort();
    };
    unsafe {
        raw_chirho.add(1).write(value_chirho);
        raw_chirho.write(INDIRECTION_CHIRHO);
    }
}
