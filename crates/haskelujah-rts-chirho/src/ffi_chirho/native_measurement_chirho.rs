// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constant-time observations at existing native allocation/root/collection boundaries.
//! Reporting is opt-in, cold and does not collect or change program output/status.
//! Workflow: testing-chirho/native-runtime-measurement-chirho.md.

use std::io::Write;
use std::path::Path;

use super::NativeGcRuntimeChirho;

pub(super) const REPORT_ENV_CHIRHO: &str = "HASKELUJAH_NATIVE_MEASUREMENT_REPORT_CHIRHO";

#[derive(Debug, Default)]
pub(super) struct NativeMeasurementChirho {
    pub(super) root_depth_high_water_chirho: usize,
    pub(super) allocated_objects_chirho: u64,
    pub(super) live_bytes_chirho: u64,
    pub(super) collection_count_chirho: u64,
    pub(super) live_objects_after_gc_chirho: usize,
    pub(super) live_bytes_after_gc_chirho: u64,
    pub(super) live_objects_after_gc_high_water_chirho: usize,
    pub(super) live_bytes_after_gc_high_water_chirho: u64,
}

impl NativeGcRuntimeChirho {
    pub(super) fn write_measurement_report_chirho(
        &self,
        path_chirho: &Path,
    ) -> std::io::Result<()> {
        // The runner owns a fresh private directory. Never replace an existing file.
        let mut file_chirho = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path_chirho)?;
        let measurement_chirho = &self.measurement_chirho;
        writeln!(file_chirho, "version_chirho=1")?;
        for (key_chirho, value_chirho) in [
            ("root_depth_chirho", self.gc_roots_chirho.len() as u64),
            (
                "root_depth_high_water_chirho",
                measurement_chirho.root_depth_high_water_chirho as u64,
            ),
            (
                "allocated_objects_chirho",
                measurement_chirho.allocated_objects_chirho,
            ),
            ("allocated_bytes_chirho", self.alloc_total_chirho),
            (
                "live_objects_chirho",
                self.allocations_by_ptr_chirho.len() as u64,
            ),
            ("live_bytes_chirho", measurement_chirho.live_bytes_chirho),
            (
                "collection_count_chirho",
                measurement_chirho.collection_count_chirho,
            ),
            (
                "live_objects_after_gc_chirho",
                measurement_chirho.live_objects_after_gc_chirho as u64,
            ),
            (
                "live_bytes_after_gc_chirho",
                measurement_chirho.live_bytes_after_gc_chirho,
            ),
            (
                "live_objects_after_gc_high_water_chirho",
                measurement_chirho.live_objects_after_gc_high_water_chirho as u64,
            ),
            (
                "live_bytes_after_gc_high_water_chirho",
                measurement_chirho.live_bytes_after_gc_high_water_chirho,
            ),
        ] {
            writeln!(file_chirho, "{key_chirho}={value_chirho}")?;
        }
        file_chirho.flush()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::NativeGcRuntimeChirho;

    #[test]
    fn observations_track_real_allocation_roots_and_retirement_chirho() {
        let mut runtime_chirho = NativeGcRuntimeChirho::default();
        let live_chirho = runtime_chirho.alloc_chirho(24);
        let _dead_chirho = runtime_chirho.alloc_chirho(16);
        runtime_chirho.gc_root_push_chirho(live_chirho);
        runtime_chirho.gc_root_push_chirho(std::ptr::null_mut());
        runtime_chirho.gc_root_pop_chirho();
        runtime_chirho.collect_chirho();
        assert_eq!(
            runtime_chirho.measurement_chirho.allocated_objects_chirho,
            2
        );
        assert_eq!(runtime_chirho.alloc_total_chirho(), 40);
        assert_eq!(runtime_chirho.gc_roots_chirho.len(), 1);
        assert_eq!(
            runtime_chirho
                .measurement_chirho
                .root_depth_high_water_chirho,
            2
        );
        assert_eq!(runtime_chirho.measurement_chirho.collection_count_chirho, 1);
        assert_eq!(
            runtime_chirho
                .measurement_chirho
                .live_objects_after_gc_chirho,
            1
        );
        assert_eq!(
            runtime_chirho.measurement_chirho.live_bytes_after_gc_chirho,
            24
        );
        runtime_chirho.gc_root_pop_chirho();
        runtime_chirho.collect_chirho();
        assert_eq!(runtime_chirho.measurement_chirho.live_bytes_chirho, 0);
        assert_eq!(
            runtime_chirho
                .measurement_chirho
                .live_objects_after_gc_chirho,
            0
        );
        assert_eq!(
            runtime_chirho
                .measurement_chirho
                .live_objects_after_gc_high_water_chirho,
            1
        );
        assert_eq!(
            runtime_chirho
                .measurement_chirho
                .live_bytes_after_gc_high_water_chirho,
            24
        );
    }
}
