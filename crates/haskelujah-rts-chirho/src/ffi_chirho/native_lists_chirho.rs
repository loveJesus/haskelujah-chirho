// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The strict reader of one native list cell, shared by the primitives that
//! walk a native list without guessing: `Map` values and, since row 557,
//! `show` for a `String`. A cell is a registered plain allocation whose first
//! word is the constructor tag, `0` for `[]` and `1` for `(:)`. A cons cell
//! keeps its head and tail in the next two words; the immediate `0` is `[]`.
//! Anything else is not a list cell, and each caller names that refusal.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

use super::{BOXED_PTR_MASK_CHIRHO, NATIVE_ALLOC_PLAIN_CHIRHO, NativeGcRuntimeChirho};

pub(super) const CONSTRUCTOR_WORDS_CHIRHO: usize = 3;
const LIST_NIL_TAG_CHIRHO: u64 = 0;
pub(super) const LIST_CONS_TAG_CHIRHO: u64 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum NativeListNodeChirho {
    NilChirho,
    ConsChirho {
        address_chirho: usize,
        head_chirho: u64,
        tail_chirho: u64,
    },
}

/// The address of a registered plain allocation of at least `minimum_words_chirho`
/// words that `value_bits_chirho` points to, low-bit tagged.
pub(super) fn plain_allocation_address_chirho(
    runtime_chirho: &NativeGcRuntimeChirho,
    value_bits_chirho: u64,
    minimum_words_chirho: usize,
) -> Option<usize> {
    if value_bits_chirho & BOXED_PTR_MASK_CHIRHO as u64 == 0 {
        return None;
    }
    let address_chirho =
        runtime_chirho.registered_heap_address_chirho(value_bits_chirho as usize)?;
    runtime_chirho
        .allocations_by_ptr_chirho
        .get(&address_chirho)
        .filter(|record_chirho| {
            record_chirho.kind_chirho == NATIVE_ALLOC_PLAIN_CHIRHO
                && record_chirho.size_chirho
                    >= minimum_words_chirho.saturating_mul(std::mem::size_of::<u64>())
        })
        .map(|_| address_chirho)
}

/// One list value in weak head normal form as a node, or None when it is not a
/// list cell.
pub(super) fn list_node_chirho(
    runtime_chirho: &NativeGcRuntimeChirho,
    value_bits_chirho: u64,
) -> Option<NativeListNodeChirho> {
    if value_bits_chirho == 0 {
        return Some(NativeListNodeChirho::NilChirho);
    }
    let address_chirho = plain_allocation_address_chirho(runtime_chirho, value_bits_chirho, 1)?;
    let raw_chirho = address_chirho as *const u64;
    match unsafe { raw_chirho.read() } {
        LIST_NIL_TAG_CHIRHO => Some(NativeListNodeChirho::NilChirho),
        LIST_CONS_TAG_CHIRHO => {
            plain_allocation_address_chirho(
                runtime_chirho,
                value_bits_chirho,
                CONSTRUCTOR_WORDS_CHIRHO,
            )?;
            Some(NativeListNodeChirho::ConsChirho {
                address_chirho,
                head_chirho: unsafe { raw_chirho.add(1).read() },
                tail_chirho: unsafe { raw_chirho.add(2).read() },
            })
        }
        _ => None,
    }
}
