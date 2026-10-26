// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Native mutable references are one-word, allocation-kind-checked cells.
//! Their payload participates in the existing conservative native GC scan.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

use super::{
    BOXED_PTR_MASK_CHIRHO, NATIVE_ALLOC_IO_REF_CHIRHO, NativeGcRuntimeChirho,
    failure_chirho::internal_failure_chirho, native_gc_runtime_lock_chirho,
};

fn new_io_ref_in_runtime_chirho(
    runtime_chirho: &mut NativeGcRuntimeChirho,
    initial_value_chirho: u64,
) -> u64 {
    runtime_chirho.gc_root_push_chirho(initial_value_chirho as usize as *mut u8);
    let raw_chirho = runtime_chirho.alloc_chirho(8).cast::<u64>();
    runtime_chirho.gc_root_pop_chirho();
    if raw_chirho.is_null() {
        internal_failure_chirho(b"native IORef allocation failed");
    }
    runtime_chirho
        .allocations_by_ptr_chirho
        .get_mut(&(raw_chirho as usize))
        .unwrap_or_else(|| internal_failure_chirho(b"new native IORef is not registered"))
        .kind_chirho = NATIVE_ALLOC_IO_REF_CHIRHO;
    unsafe {
        raw_chirho.write(initial_value_chirho);
    }
    let io_ref_bits_chirho = raw_chirho as u64 | BOXED_PTR_MASK_CHIRHO as u64;
    // Native backends currently retain produced heap values for the process
    // lifetime. Keep the mutable cell under that same conservative contract.
    runtime_chirho.gc_root_push_chirho(io_ref_bits_chirho as usize as *mut u8);
    io_ref_bits_chirho
}

fn io_ref_address_chirho(
    runtime_chirho: &NativeGcRuntimeChirho,
    io_ref_bits_chirho: u64,
) -> Option<*mut u64> {
    if io_ref_bits_chirho & BOXED_PTR_MASK_CHIRHO as u64 == 0 {
        return None;
    }
    let address_chirho =
        runtime_chirho.registered_heap_address_chirho(io_ref_bits_chirho as usize)?;
    runtime_chirho
        .allocations_by_ptr_chirho
        .get(&address_chirho)
        .filter(|record_chirho| record_chirho.kind_chirho == NATIVE_ALLOC_IO_REF_CHIRHO)
        .map(|_| address_chirho as *mut u64)
}

/// `newIORef#`: keep the initial value alive if allocating the cell collects.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_new_io_ref_chirho(initial_value_chirho: u64) -> u64 {
    new_io_ref_in_runtime_chirho(&mut native_gc_runtime_lock_chirho(), initial_value_chirho)
}

/// `readIORef#`: read only a live allocation created as an IORef.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_read_io_ref_chirho(io_ref_bits_chirho: u64) -> u64 {
    let runtime_chirho = native_gc_runtime_lock_chirho();
    let raw_chirho = io_ref_address_chirho(&runtime_chirho, io_ref_bits_chirho)
        .unwrap_or_else(|| internal_failure_chirho(b"readIORef# received an invalid reference"));
    unsafe { raw_chirho.read() }
}

/// `writeIORef#`: replace the traced cell payload and return native `()`.
#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_write_io_ref_chirho(
    io_ref_bits_chirho: u64,
    value_chirho: u64,
) -> u64 {
    let runtime_chirho = native_gc_runtime_lock_chirho();
    let raw_chirho = io_ref_address_chirho(&runtime_chirho, io_ref_bits_chirho)
        .unwrap_or_else(|| internal_failure_chirho(b"writeIORef# received an invalid reference"));
    unsafe {
        raw_chirho.write(value_chirho);
    }
    0
}

#[cfg(test)]
mod tests_chirho {
    use super::super::GC_THRESHOLD_CHIRHO;
    use super::*;

    #[test]
    fn native_io_ref_round_trip_and_kind_check_chirho() {
        let mut runtime_chirho = NativeGcRuntimeChirho::default();
        let io_ref_chirho = new_io_ref_in_runtime_chirho(&mut runtime_chirho, 1);
        let raw_chirho = io_ref_address_chirho(&runtime_chirho, io_ref_chirho).unwrap();
        assert_eq!(unsafe { raw_chirho.read() }, 1);
        unsafe {
            raw_chirho.write(2);
        }
        assert_eq!(unsafe { raw_chirho.read() }, 2);

        let constructor_chirho = runtime_chirho.alloc_chirho(8) as u64 | 1;
        assert!(io_ref_address_chirho(&runtime_chirho, constructor_chirho).is_none());
        assert!(io_ref_address_chirho(&runtime_chirho, 2).is_none());
    }

    #[test]
    fn allocating_and_tracing_an_io_ref_keeps_its_payload_chirho() {
        let mut runtime_chirho = NativeGcRuntimeChirho::default();
        let payload_raw_chirho = runtime_chirho.alloc_chirho(8);
        let payload_chirho = payload_raw_chirho as u64 | 1;
        runtime_chirho.alloc_count_since_gc_chirho = GC_THRESHOLD_CHIRHO;

        let io_ref_chirho = new_io_ref_in_runtime_chirho(&mut runtime_chirho, payload_chirho);
        assert!(runtime_chirho.contains_alloc_chirho(payload_raw_chirho));
        runtime_chirho.collect_chirho();
        assert!(runtime_chirho.contains_alloc_chirho(payload_raw_chirho));
        assert!(
            runtime_chirho
                .registered_heap_address_chirho(io_ref_chirho as usize)
                .is_some()
        );
    }
}
