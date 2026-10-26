// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Native `Map` values use the generated Core's association-list boundary.
//! Empty and singleton maps need no key comparison, so this module implements
//! that representation for every key type and refuses a second pair until
//! comparator evidence reaches native lowering.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

use std::collections::HashSet;

use super::{
    BOXED_PTR_MASK_CHIRHO, NATIVE_ALLOC_PLAIN_CHIRHO, NativeGcRuntimeChirho,
    failure_chirho::internal_failure_chirho, haskelujah_enter_thunk_chirho,
    native_gc_runtime_lock_chirho,
};

const CONSTRUCTOR_WORDS_CHIRHO: usize = 3;
const LIST_NIL_TAG_CHIRHO: u64 = 0;
const LIST_CONS_TAG_CHIRHO: u64 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NativeListNodeChirho {
    NilChirho,
    ConsChirho {
        address_chirho: usize,
        head_chirho: u64,
        tail_chirho: u64,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NativeMapErrorChirho(&'static [u8]);

impl NativeMapErrorChirho {
    const CYCLIC_MAP_CHIRHO: Self = Self(b"mapSize# received a cyclic native map");
    const INVALID_LIST_CHIRHO: Self = Self(b"map primitive received an invalid native list");
    const INVALID_PAIR_CHIRHO: Self = Self(b"mapFromList# received an invalid pair");
    const MULTIPLE_PAIRS_CHIRHO: Self =
        Self(b"mapFromList# native maps currently require at most one pair");
    const SIZE_OVERFLOW_CHIRHO: Self = Self(b"mapSize# result overflow");

    fn message_chirho(self) -> &'static [u8] {
        self.0
    }
}

fn plain_allocation_address_chirho(
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

fn list_node_chirho(
    runtime_chirho: &NativeGcRuntimeChirho,
    value_bits_chirho: u64,
) -> Result<NativeListNodeChirho, NativeMapErrorChirho> {
    if value_bits_chirho == 0 {
        return Ok(NativeListNodeChirho::NilChirho);
    }
    let address_chirho = plain_allocation_address_chirho(runtime_chirho, value_bits_chirho, 1)
        .ok_or(NativeMapErrorChirho::INVALID_LIST_CHIRHO)?;
    let raw_chirho = address_chirho as *const u64;
    match unsafe { raw_chirho.read() } {
        LIST_NIL_TAG_CHIRHO => Ok(NativeListNodeChirho::NilChirho),
        LIST_CONS_TAG_CHIRHO => {
            plain_allocation_address_chirho(
                runtime_chirho,
                value_bits_chirho,
                CONSTRUCTOR_WORDS_CHIRHO,
            )
            .ok_or(NativeMapErrorChirho::INVALID_LIST_CHIRHO)?;
            Ok(NativeListNodeChirho::ConsChirho {
                address_chirho,
                head_chirho: unsafe { raw_chirho.add(1).read() },
                tail_chirho: unsafe { raw_chirho.add(2).read() },
            })
        }
        _ => Err(NativeMapErrorChirho::INVALID_LIST_CHIRHO),
    }
}

fn validate_pair_chirho(
    runtime_chirho: &NativeGcRuntimeChirho,
    pair_bits_chirho: u64,
    expected_pair_tag_chirho: u64,
) -> Result<(), NativeMapErrorChirho> {
    let address_chirho =
        plain_allocation_address_chirho(runtime_chirho, pair_bits_chirho, CONSTRUCTOR_WORDS_CHIRHO)
            .ok_or(NativeMapErrorChirho::INVALID_PAIR_CHIRHO)?;
    if unsafe { (address_chirho as *const u64).read() } != expected_pair_tag_chirho {
        return Err(NativeMapErrorChirho::INVALID_PAIR_CHIRHO);
    }
    Ok(())
}

fn allocate_singleton_map_chirho(
    runtime_chirho: &mut NativeGcRuntimeChirho,
    pair_bits_chirho: u64,
) -> u64 {
    let raw_chirho = runtime_chirho
        .alloc_chirho((CONSTRUCTOR_WORDS_CHIRHO * std::mem::size_of::<u64>()) as u64)
        .cast::<u64>();
    if raw_chirho.is_null() {
        internal_failure_chirho(b"native map allocation failed");
    }
    unsafe {
        raw_chirho.write(LIST_CONS_TAG_CHIRHO);
        raw_chirho.add(1).write(pair_bits_chirho);
        raw_chirho.add(2).write(0);
    }
    raw_chirho as u64 | BOXED_PTR_MASK_CHIRHO as u64
}

fn singleton_map_from_parts_chirho(
    runtime_chirho: &mut NativeGcRuntimeChirho,
    pair_bits_chirho: u64,
    tail_bits_chirho: u64,
    expected_pair_tag_chirho: u64,
) -> Result<u64, NativeMapErrorChirho> {
    match list_node_chirho(runtime_chirho, tail_bits_chirho)? {
        NativeListNodeChirho::NilChirho => {}
        NativeListNodeChirho::ConsChirho { .. } => {
            return Err(NativeMapErrorChirho::MULTIPLE_PAIRS_CHIRHO);
        }
    }
    validate_pair_chirho(runtime_chirho, pair_bits_chirho, expected_pair_tag_chirho)?;
    Ok(allocate_singleton_map_chirho(
        runtime_chirho,
        pair_bits_chirho,
    ))
}

#[cfg(test)]
fn map_from_list_whnf_in_runtime_chirho(
    runtime_chirho: &mut NativeGcRuntimeChirho,
    list_bits_chirho: u64,
    expected_pair_tag_chirho: u64,
) -> Result<u64, NativeMapErrorChirho> {
    match list_node_chirho(runtime_chirho, list_bits_chirho)? {
        NativeListNodeChirho::NilChirho => Ok(0),
        NativeListNodeChirho::ConsChirho {
            head_chirho,
            tail_chirho,
            ..
        } => singleton_map_from_parts_chirho(
            runtime_chirho,
            head_chirho,
            tail_chirho,
            expected_pair_tag_chirho,
        ),
    }
}

#[cfg(test)]
fn map_size_whnf_in_runtime_chirho(
    runtime_chirho: &NativeGcRuntimeChirho,
    map_bits_chirho: u64,
) -> Result<u64, NativeMapErrorChirho> {
    let mut current_chirho = map_bits_chirho;
    let mut count_chirho = 0_u64;
    let mut seen_chirho = HashSet::new();
    loop {
        match list_node_chirho(runtime_chirho, current_chirho)? {
            NativeListNodeChirho::NilChirho => return Ok(count_chirho),
            NativeListNodeChirho::ConsChirho {
                address_chirho,
                tail_chirho,
                ..
            } => {
                if !seen_chirho.insert(address_chirho) {
                    return Err(NativeMapErrorChirho::CYCLIC_MAP_CHIRHO);
                }
                count_chirho = count_chirho
                    .checked_add(1)
                    .ok_or(NativeMapErrorChirho::SIZE_OVERFLOW_CHIRHO)?;
                current_chirho = tail_chirho;
            }
        }
    }
}

fn push_root_chirho(value_bits_chirho: u64) {
    native_gc_runtime_lock_chirho().gc_root_push_chirho(value_bits_chirho as usize as *mut u8);
}

fn pop_root_chirho() {
    native_gc_runtime_lock_chirho().gc_root_pop_chirho();
}

/// `mapFromList#`: preserve the exact empty/singleton list representation.
/// A second pair remains a named boundary until native code receives `Ord k`.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_map_from_list_chirho(
    list_bits_chirho: u64,
    expected_pair_tag_chirho: u64,
) -> u64 {
    push_root_chirho(list_bits_chirho);
    let list_whnf_chirho = haskelujah_enter_thunk_chirho(list_bits_chirho);
    let node_chirho = {
        let runtime_chirho = native_gc_runtime_lock_chirho();
        list_node_chirho(&runtime_chirho, list_whnf_chirho)
    };
    let result_chirho = match node_chirho {
        Ok(NativeListNodeChirho::NilChirho) => Ok(0),
        Ok(NativeListNodeChirho::ConsChirho {
            head_chirho,
            tail_chirho,
            ..
        }) => {
            let tail_whnf_chirho = haskelujah_enter_thunk_chirho(tail_chirho);
            let pair_whnf_chirho = haskelujah_enter_thunk_chirho(head_chirho);
            singleton_map_from_parts_chirho(
                &mut native_gc_runtime_lock_chirho(),
                pair_whnf_chirho,
                tail_whnf_chirho,
                expected_pair_tag_chirho,
            )
        }
        Err(error_chirho) => Err(error_chirho),
    };
    pop_root_chirho();
    result_chirho
        .unwrap_or_else(|error_chirho| internal_failure_chirho(error_chirho.message_chirho()))
}

/// `mapSize#`: count a validated association-list spine without inspecting keys.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_map_size_chirho(map_bits_chirho: u64) -> u64 {
    push_root_chirho(map_bits_chirho);
    let mut current_chirho = map_bits_chirho;
    let mut count_chirho = 0_u64;
    let mut seen_chirho = HashSet::new();
    let result_chirho = loop {
        current_chirho = haskelujah_enter_thunk_chirho(current_chirho);
        let node_chirho = {
            let runtime_chirho = native_gc_runtime_lock_chirho();
            list_node_chirho(&runtime_chirho, current_chirho)
        };
        match node_chirho {
            Ok(NativeListNodeChirho::NilChirho) => break Ok(count_chirho),
            Ok(NativeListNodeChirho::ConsChirho {
                address_chirho,
                tail_chirho,
                ..
            }) => {
                if !seen_chirho.insert(address_chirho) {
                    break Err(NativeMapErrorChirho::CYCLIC_MAP_CHIRHO);
                }
                let Some(next_count_chirho) = count_chirho.checked_add(1) else {
                    break Err(NativeMapErrorChirho::SIZE_OVERFLOW_CHIRHO);
                };
                count_chirho = next_count_chirho;
                current_chirho = tail_chirho;
            }
            Err(error_chirho) => break Err(error_chirho),
        }
    };
    pop_root_chirho();
    result_chirho
        .unwrap_or_else(|error_chirho| internal_failure_chirho(error_chirho.message_chirho()))
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    const TEST_TUPLE_TAG_CHIRHO: u64 = 0x5455_504c;

    fn allocate_constructor_chirho(
        runtime_chirho: &mut NativeGcRuntimeChirho,
        tag_chirho: u64,
        first_chirho: u64,
        second_chirho: u64,
    ) -> u64 {
        let raw_chirho = runtime_chirho.alloc_chirho(24).cast::<u64>();
        unsafe {
            raw_chirho.write(tag_chirho);
            raw_chirho.add(1).write(first_chirho);
            raw_chirho.add(2).write(second_chirho);
        }
        raw_chirho as u64 | 1
    }

    fn pair_chirho(runtime_chirho: &mut NativeGcRuntimeChirho, key_chirho: u64) -> u64 {
        allocate_constructor_chirho(runtime_chirho, TEST_TUPLE_TAG_CHIRHO, key_chirho, 99)
    }

    #[test]
    fn empty_and_singleton_lists_become_bounded_native_maps_chirho() {
        let mut runtime_chirho = NativeGcRuntimeChirho::default();
        assert_eq!(
            map_from_list_whnf_in_runtime_chirho(&mut runtime_chirho, 0, TEST_TUPLE_TAG_CHIRHO,),
            Ok(0)
        );

        let pair_chirho = pair_chirho(&mut runtime_chirho, 1);
        let source_chirho =
            allocate_constructor_chirho(&mut runtime_chirho, LIST_CONS_TAG_CHIRHO, pair_chirho, 0);
        let map_chirho = map_from_list_whnf_in_runtime_chirho(
            &mut runtime_chirho,
            source_chirho,
            TEST_TUPLE_TAG_CHIRHO,
        )
        .expect("the singleton needs no key comparison");
        assert_ne!(map_chirho, source_chirho, "the map owns a fresh spine");
        assert_eq!(
            map_size_whnf_in_runtime_chirho(&runtime_chirho, map_chirho),
            Ok(1)
        );
        assert_eq!(map_size_whnf_in_runtime_chirho(&runtime_chirho, 0), Ok(0));
        assert!(matches!(
            list_node_chirho(&runtime_chirho, map_chirho),
            Ok(NativeListNodeChirho::ConsChirho {
                head_chirho,
                tail_chirho: 0,
                ..
            }) if head_chirho == pair_chirho
        ));
    }

    #[test]
    fn a_second_pair_and_malformed_values_fail_closed_chirho() {
        let mut runtime_chirho = NativeGcRuntimeChirho::default();
        let second_pair_chirho = pair_chirho(&mut runtime_chirho, 2);
        let tail_chirho = allocate_constructor_chirho(
            &mut runtime_chirho,
            LIST_CONS_TAG_CHIRHO,
            second_pair_chirho,
            0,
        );
        let first_pair_chirho = pair_chirho(&mut runtime_chirho, 1);
        let two_chirho = allocate_constructor_chirho(
            &mut runtime_chirho,
            LIST_CONS_TAG_CHIRHO,
            first_pair_chirho,
            tail_chirho,
        );
        assert_eq!(
            map_from_list_whnf_in_runtime_chirho(
                &mut runtime_chirho,
                two_chirho,
                TEST_TUPLE_TAG_CHIRHO,
            ),
            Err(NativeMapErrorChirho::MULTIPLE_PAIRS_CHIRHO)
        );

        let not_pair_chirho =
            allocate_constructor_chirho(&mut runtime_chirho, LIST_CONS_TAG_CHIRHO, 1, 0);
        let malformed_chirho = allocate_constructor_chirho(
            &mut runtime_chirho,
            LIST_CONS_TAG_CHIRHO,
            not_pair_chirho,
            0,
        );
        assert_eq!(
            map_from_list_whnf_in_runtime_chirho(
                &mut runtime_chirho,
                malformed_chirho,
                TEST_TUPLE_TAG_CHIRHO,
            ),
            Err(NativeMapErrorChirho::INVALID_PAIR_CHIRHO)
        );
    }

    #[test]
    fn map_size_counts_a_valid_spine_and_rejects_a_cycle_chirho() {
        let mut runtime_chirho = NativeGcRuntimeChirho::default();
        let second_pair_chirho = pair_chirho(&mut runtime_chirho, 2);
        let tail_chirho = allocate_constructor_chirho(
            &mut runtime_chirho,
            LIST_CONS_TAG_CHIRHO,
            second_pair_chirho,
            0,
        );
        let first_pair_chirho = pair_chirho(&mut runtime_chirho, 1);
        let map_chirho = allocate_constructor_chirho(
            &mut runtime_chirho,
            LIST_CONS_TAG_CHIRHO,
            first_pair_chirho,
            tail_chirho,
        );
        assert_eq!(
            map_size_whnf_in_runtime_chirho(&runtime_chirho, map_chirho),
            Ok(2)
        );

        let cycle_pair_chirho = pair_chirho(&mut runtime_chirho, 3);
        let cyclic_chirho = allocate_constructor_chirho(
            &mut runtime_chirho,
            LIST_CONS_TAG_CHIRHO,
            cycle_pair_chirho,
            0,
        );
        let address_chirho = cyclic_chirho & !1;
        unsafe {
            (address_chirho as *mut u64).add(2).write(cyclic_chirho);
        }
        assert_eq!(
            map_size_whnf_in_runtime_chirho(&runtime_chirho, cyclic_chirho),
            Err(NativeMapErrorChirho::CYCLIC_MAP_CHIRHO)
        );
    }
}
