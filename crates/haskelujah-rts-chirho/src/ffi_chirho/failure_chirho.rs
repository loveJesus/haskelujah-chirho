// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The one failure route of native executables. A program failure and a
//! violated runtime invariant both flush what the program wrote, write one
//! report to stderr and exit with status 1, the status GHC and the interpreter
//! give an uncaught failure. The prefix is the classification:
//! `haskelujah: error:` for the program, `haskelujah: internal error:` for an
//! invariant. Nothing on this route ends in `abort()` or a signal.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

use std::ffi::{c_int, c_void};
use std::io::Write;

use super::{
    haskelujah_alloc_thunk_chirho, haskelujah_is_heap_ptr_chirho, native_gc_runtime_lock_chirho,
};

const USER_PREFIX_CHIRHO: &[u8] = b"haskelujah: error: ";
const INTERNAL_PREFIX_CHIRHO: &[u8] = b"haskelujah: internal error: ";
const NO_MATCH_MESSAGE_CHIRHO: &[u8] = b"no matching alternative";
const UNDEFINED_MESSAGE_CHIRHO: &[u8] = b"Prelude.undefined";
const UNCAUGHT_EXCEPTION_MESSAGE_CHIRHO: &[u8] = b"uncaught exception";
const BOXED_SHOW_MESSAGE_CHIRHO: &[u8] = b"show: no native Show instance for a boxed value";
/// GHC.Real's `(^)` on a negative exponent.
pub(crate) const NEGATIVE_EXPONENT_MESSAGE_CHIRHO: &[u8] = b"Negative exponent";
const NUL_STRING_MESSAGE_CHIRHO: &[u8] = b"string: a NUL character has no native representation";
const INVALID_UTF8_MESSAGE_CHIRHO: &[u8] = b"string: a native String is not valid UTF-8";
const UNENCODABLE_CHAR_MESSAGE_CHIRHO: &[u8] = b"string: a character has no UTF-8 encoding";

unsafe extern "C" {
    // C stdio buffers what LLVM executables write through `printf` and `puts`.
    fn fflush(stream_chirho: *mut c_void) -> c_int;
}

/// The prefix, the payload byte for byte (GHC writes an `error` payload as
/// given, embedded newlines included), and one terminating newline.
fn failure_report_chirho(prefix_chirho: &[u8], message_chirho: &[u8]) -> Vec<u8> {
    let mut report_chirho = Vec::with_capacity(prefix_chirho.len() + message_chirho.len() + 1);
    report_chirho.extend_from_slice(prefix_chirho);
    report_chirho.extend_from_slice(message_chirho);
    report_chirho.push(b'\n');
    report_chirho
}

fn fail_chirho(prefix_chirho: &[u8], message_chirho: &[u8]) -> ! {
    // The program's output first, so a terminal shows it before the report.
    let _ = std::io::stdout().flush();
    // SAFETY: a null stream asks C stdio to flush every open output stream.
    unsafe {
        fflush(std::ptr::null_mut());
    }
    {
        let mut stderr_chirho = std::io::stderr().lock();
        let _ = stderr_chirho.write_all(&failure_report_chirho(prefix_chirho, message_chirho));
        let _ = stderr_chirho.flush();
    }
    std::process::exit(1)
}

/// A program failure: `haskelujah: error: <message>`, status 1.
pub(crate) fn user_failure_chirho(message_chirho: &[u8]) -> ! {
    fail_chirho(USER_PREFIX_CHIRHO, message_chirho)
}

/// A violated runtime invariant: `haskelujah: internal error: <message>`, status 1.
pub(crate) fn internal_failure_chirho(message_chirho: &[u8]) -> ! {
    fail_chirho(INTERNAL_PREFIX_CHIRHO, message_chirho)
}

/// An integer formatter must not print a heap address. `value_bits_chirho` is
/// refused when it is a live tagged allocation (low bit set and registered).
/// An odd immediate equal to a live address plus one would be misreported.
pub(crate) fn refuse_boxed_integer_chirho(value_bits_chirho: u64) {
    if haskelujah_is_heap_ptr_chirho(value_bits_chirho) != 0 {
        user_failure_chirho(BOXED_SHOW_MESSAGE_CHIRHO);
    }
}

/// Native Strings are NUL-terminated UTF-8 buffers (row 560), so a NUL character, invalid UTF-8
/// and a value with no UTF-8 encoding each fail by one named line instead of truncating or
/// guessing.
pub(crate) fn nul_string_failure_chirho() -> ! {
    user_failure_chirho(NUL_STRING_MESSAGE_CHIRHO)
}

pub(crate) fn invalid_utf8_failure_chirho() -> ! {
    user_failure_chirho(INVALID_UTF8_MESSAGE_CHIRHO)
}

pub(crate) fn unencodable_char_failure_chirho() -> ! {
    user_failure_chirho(UNENCODABLE_CHAR_MESSAGE_CHIRHO)
}

/// An evaluated string literal that holds NUL: its NUL-terminated global would read shorter.
/// workflow: testing-chirho/native-fail-loud-chirho
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_nul_string_literal_chirho() -> u64 {
    nul_string_failure_chirho()
}

/// Runtime `error`, `errorWithoutStackTrace` and pattern-match failure: the
/// payload exactly, on the program route.
///
/// # Safety
/// `msg_chirho` must be nonnull and point to `len_chirho` initialized, readable
/// bytes in one allocation. The message need not be NUL-terminated or UTF-8.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haskelujah_error_chirho(msg_chirho: *const u8, len_chirho: u64) -> ! {
    // SAFETY: the caller guarantees `len_chirho` readable bytes.
    let message_chirho = unsafe { std::slice::from_raw_parts(msg_chirho, len_chirho as usize) };
    user_failure_chirho(message_chirho)
}

/// Runtime `undefined`.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_undefined_chirho() -> ! {
    user_failure_chirho(UNDEFINED_MESSAGE_CHIRHO)
}

/// A case whose scrutinee matched none of the compiled alternatives.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_no_match_chirho() -> ! {
    user_failure_chirho(NO_MATCH_MESSAGE_CHIRHO)
}

/// `throw#` reached a native executable, which has no handler to catch it.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_uncaught_exception_chirho() -> ! {
    user_failure_chirho(UNCAUGHT_EXCEPTION_MESSAGE_CHIRHO)
}

/// The value of a reference with no native binding: a thunk whose entry fails
/// with ``missing native binding `<name>` ``. Code that never demands it runs
/// as before. Each name has one thunk for the whole program, kept as a GC
/// root, so a loop that mentions the name allocates only once.
///
/// # Safety
/// `name_chirho` must point to `len_chirho` readable bytes that stay valid and
/// unchanged for the rest of the program, such as a string constant.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haskelujah_unbound_thunk_chirho(
    name_chirho: *const u8,
    len_chirho: u64,
) -> u64 {
    let key_chirho = name_chirho as usize;
    let cached_chirho = native_gc_runtime_lock_chirho()
        .unbound_thunks_chirho
        .get(&key_chirho)
        .copied();
    if let Some(thunk_chirho) = cached_chirho {
        return thunk_chirho;
    }
    let environment_chirho = [name_chirho as u64, len_chirho];
    // SAFETY: the entry has the thunk signature and reads exactly the two
    // environment words copied here. No allocation happens between this one
    // and the root insertion below, so no collection can free the thunk.
    let thunk_chirho = unsafe {
        haskelujah_alloc_thunk_chirho(
            unbound_entry_chirho as *const () as u64,
            2,
            environment_chirho.as_ptr(),
        )
    };
    native_gc_runtime_lock_chirho()
        .unbound_thunks_chirho
        .insert(key_chirho, thunk_chirho);
    thunk_chirho
}

unsafe extern "C" fn unbound_entry_chirho(environment_chirho: *const u64) -> u64 {
    // SAFETY: haskelujah_unbound_thunk_chirho stored a static name and its length.
    let name_chirho = unsafe {
        std::slice::from_raw_parts(
            *environment_chirho as usize as *const u8,
            *environment_chirho.add(1) as usize,
        )
    };
    let mut message_chirho = b"missing native binding `".to_vec();
    message_chirho.extend_from_slice(name_chirho);
    message_chirho.push(b'`');
    user_failure_chirho(&message_chirho)
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn reports_carry_their_classification_prefix_and_one_newline_chirho() {
        assert_eq!(
            failure_report_chirho(USER_PREFIX_CHIRHO, b"boom"),
            b"haskelujah: error: boom\n"
        );
        assert_eq!(
            failure_report_chirho(INTERNAL_PREFIX_CHIRHO, b"corrupt native thunk state"),
            b"haskelujah: internal error: corrupt native thunk state\n"
        );
    }
}
