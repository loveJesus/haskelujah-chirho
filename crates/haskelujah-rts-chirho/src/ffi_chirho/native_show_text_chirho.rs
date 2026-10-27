// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `show` for a native `String` (row 557). Its code points reach the shared
//! renderer exactly, or the program fails loudly with one named reason.
//! - A NUL-terminated buffer (a literal, a `++#` result or `show` output) is read
//!   as UTF-8.
//! - A registered `[Char]` list is walked cell by cell, forcing each character
//!   as `haskelujah_pack_string_chirho` does.
//!
//! A cell holds a code point (row 560's unpack decodes UTF-8), and a list may end in a
//! literal's buffer instead of `[]` (row 576), which is read as UTF-8 too. What remains
//! refused is a buffer that is not UTF-8, or a value that is neither a list nor a buffer.
//!
//! workflow: print-show-evidence-chirho

use std::ffi::CStr;

use super::failure_chirho::user_failure_chirho;
use super::native_lists_chirho::{NativeListNodeChirho, list_node_chirho};
use super::{
    alloc_c_string_chirho, haskelujah_enter_thunk_chirho, haskelujah_gc_root_pop_chirho,
    haskelujah_gc_root_push_chirho, haskelujah_is_heap_ptr_chirho, native_gc_runtime_lock_chirho,
};

const UNREAD_STRING_MESSAGE_CHIRHO: &[u8] = b"show: no exact native reading of this String";

/// GHC's `show` for a native `String`, as a heap-allocated NUL-terminated buffer.
/// workflow: print-show-evidence-chirho
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_show_string_chirho(value_bits_chirho: u64) -> u64 {
    let value_bits_chirho = haskelujah_enter_thunk_chirho(value_bits_chirho);
    let code_points_chirho = if haskelujah_is_heap_ptr_chirho(value_bits_chirho) != 0 {
        list_code_points_chirho(value_bits_chirho)
    } else {
        buffer_code_points_chirho(value_bits_chirho)
    };
    let Some(code_points_chirho) = code_points_chirho else {
        user_failure_chirho(UNREAD_STRING_MESSAGE_CHIRHO)
    };
    alloc_c_string_chirho(crate::show_string_chirho(code_points_chirho).as_bytes())
}

/// The code points of a NUL-terminated buffer, or None when it is not UTF-8.
fn buffer_code_points_chirho(ptr_bits_chirho: u64) -> Option<Vec<u32>> {
    if ptr_bits_chirho == 0 {
        return Some(Vec::new());
    }
    // SAFETY: a nonzero String value that is not a registered heap cell is an
    // aligned NUL-terminated buffer, as `native_c_string_bits_chirho` relies on.
    let ptr_chirho = ptr_bits_chirho as usize as *const std::ffi::c_char;
    let bytes_chirho = unsafe { CStr::from_ptr(ptr_chirho) }.to_bytes();
    let text_chirho = std::str::from_utf8(bytes_chirho).ok()?;
    Some(text_chirho.chars().map(u32::from).collect())
}

/// The code points of a `[Char]` list, or None at a cell with no exact reading.
fn list_code_points_chirho(list_bits_chirho: u64) -> Option<Vec<u32>> {
    let mut code_points_chirho = Vec::new();
    let mut current_chirho = list_bits_chirho;
    loop {
        current_chirho = haskelujah_enter_thunk_chirho(current_chirho);
        // A forced, nonzero tail that is no registered cell is a NUL-terminated UTF-8 buffer
        // (row 576); registration decides it, since a literal may sit at an odd address.
        if current_chirho != 0 && haskelujah_is_heap_ptr_chirho(current_chirho) == 0 {
            code_points_chirho.extend(buffer_code_points_chirho(current_chirho)?);
            return Some(code_points_chirho);
        }
        let node_chirho = list_node_chirho(&native_gc_runtime_lock_chirho(), current_chirho)?;
        let NativeListNodeChirho::ConsChirho {
            head_chirho,
            tail_chirho,
            ..
        } = node_chirho
        else {
            return Some(code_points_chirho);
        };
        // Forcing the character may collect; the rooted cell keeps the tail alive.
        haskelujah_gc_root_push_chirho(current_chirho as usize as *mut u8);
        let head_chirho = haskelujah_enter_thunk_chirho(head_chirho);
        haskelujah_gc_root_pop_chirho();
        code_points_chirho.push(head_chirho as u32);
        current_chirho = tail_chirho;
    }
}
