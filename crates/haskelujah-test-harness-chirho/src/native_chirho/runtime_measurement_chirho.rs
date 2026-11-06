// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Measurement through the existing native artifact boundary, with retained evidence.
//! Workflow: testing-chirho/native-runtime-measurement-chirho.md.

use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

use haskelujah_driver_chirho::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

use super::{NativeBackendChirho, native_artifact_chirho, native_artifact_in_directory_chirho};

pub(super) const REPORT_ENV_CHIRHO: &str = "HASKELUJAH_NATIVE_MEASUREMENT_REPORT_CHIRHO";
const REPORT_LIMIT_CHIRHO: u64 = 4096;

#[derive(Debug)]
pub struct NativeRuntimeCountersChirho {
    pub root_depth_chirho: u64,
    pub root_depth_high_water_chirho: u64,
    pub allocated_objects_chirho: u64,
    pub allocated_bytes_chirho: u64,
    pub live_objects_chirho: u64,
    pub live_bytes_chirho: u64,
    pub collection_count_chirho: u64,
    pub live_objects_after_gc_chirho: u64,
    pub live_bytes_after_gc_chirho: u64,
    pub live_objects_after_gc_high_water_chirho: u64,
    pub live_bytes_after_gc_high_water_chirho: u64,
}

#[derive(Debug)]
pub struct NativeRuntimeMeasurementChirho {
    pub exit_code_chirho: i32,
    pub stdout_chirho: String,
    pub counters_chirho: NativeRuntimeCountersChirho,
    /// Source, exact IR/object/executable/RTS, raw report and link/run streams survive failure.
    /// The gate owner hashes these bytes and retires the directory after its evidence fold.
    pub artifact_directory_chirho: PathBuf,
}

/// Run only the selected backend, with a report path set on this child alone.
/// No collector/reset operation is introduced by measuring it.
pub fn measure_native_runtime_chirho(
    source_chirho: &str,
    backend_chirho: NativeBackendChirho,
    deadline_chirho: Duration,
) -> Result<NativeRuntimeMeasurementChirho, String> {
    let directory_chirho = tempfile::tempdir()
        .map_err(|error_chirho| error_chirho.to_string())?
        .keep();
    eprintln!(
        "native measurement evidence: {}",
        directory_chirho.display()
    );
    let measured_chirho = (|| {
        std::fs::write(directory_chirho.join("source-chirho.hs"), source_chirho)
            .map_err(|error_chirho| error_chirho.to_string())?;
        let compiled_chirho =
            compile_source_chirho(source_chirho, &mut SourceMapChirho::new_chirho(), "Main.hs")
                .map_err(|diagnostics_chirho| format!("frontend failed: {diagnostics_chirho:?}"))?;
        let artifact_chirho = native_artifact_chirho(&compiled_chirho.core_chirho, backend_chirho)?;
        let (exit_code_chirho, stdout_chirho) = native_artifact_in_directory_chirho(
            artifact_chirho,
            "",
            deadline_chirho,
            &directory_chirho,
            true,
        )?;
        let file_chirho =
            std::fs::File::open(directory_chirho.join("runtime-report-chirho.txt"))
                .map_err(|error_chirho| format!("native runtime report missing: {error_chirho}"))?;
        let mut report_chirho = String::new();
        file_chirho
            .take(REPORT_LIMIT_CHIRHO + 1)
            .read_to_string(&mut report_chirho)
            .map_err(|error_chirho| format!("native runtime report unreadable: {error_chirho}"))?;
        if report_chirho.len() as u64 > REPORT_LIMIT_CHIRHO {
            return Err("native runtime report exceeds its bound".to_string());
        }
        Ok((
            exit_code_chirho,
            stdout_chirho,
            parse_counters_chirho(&report_chirho)?,
        ))
    })();
    let (exit_code_chirho, stdout_chirho, counters_chirho) =
        measured_chirho.map_err(|error_chirho| {
            format!("{error_chirho}; evidence: {}", directory_chirho.display())
        })?;
    Ok(NativeRuntimeMeasurementChirho {
        exit_code_chirho,
        stdout_chirho,
        counters_chirho,
        artifact_directory_chirho: directory_chirho,
    })
}

pub(super) fn retain_streams_chirho(
    directory_chirho: &Path,
    stage_chirho: &str,
    stdout_chirho: &[u8],
    stderr_chirho: &[u8],
) -> Result<(), String> {
    for (suffix_chirho, bytes_chirho) in [
        ("stdout-chirho.txt", stdout_chirho),
        ("stderr-chirho.txt", stderr_chirho),
    ] {
        std::fs::write(
            directory_chirho.join(format!("{stage_chirho}-{suffix_chirho}")),
            bytes_chirho,
        )
        .map_err(|error_chirho| error_chirho.to_string())?;
    }
    Ok(())
}

fn parse_counters_chirho(report_chirho: &str) -> Result<NativeRuntimeCountersChirho, String> {
    let mut values_chirho = BTreeMap::new();
    for line_chirho in report_chirho.lines() {
        let (key_chirho, raw_chirho) = line_chirho
            .split_once('=')
            .ok_or_else(|| "malformed native counter line".to_string())?;
        let value_chirho = raw_chirho
            .parse::<u64>()
            .map_err(|error_chirho| format!("invalid {key_chirho}: {error_chirho}"))?;
        if values_chirho.insert(key_chirho, value_chirho).is_some() {
            return Err(format!("duplicate native counter {key_chirho}"));
        }
    }
    let mut take_chirho = |key_chirho| {
        values_chirho
            .remove(key_chirho)
            .ok_or_else(|| format!("missing native counter {key_chirho}"))
    };
    if take_chirho("version_chirho")? != 1 {
        return Err("unknown native report version".to_string());
    }
    let counters_chirho = NativeRuntimeCountersChirho {
        root_depth_chirho: take_chirho("root_depth_chirho")?,
        root_depth_high_water_chirho: take_chirho("root_depth_high_water_chirho")?,
        allocated_objects_chirho: take_chirho("allocated_objects_chirho")?,
        allocated_bytes_chirho: take_chirho("allocated_bytes_chirho")?,
        live_objects_chirho: take_chirho("live_objects_chirho")?,
        live_bytes_chirho: take_chirho("live_bytes_chirho")?,
        collection_count_chirho: take_chirho("collection_count_chirho")?,
        live_objects_after_gc_chirho: take_chirho("live_objects_after_gc_chirho")?,
        live_bytes_after_gc_chirho: take_chirho("live_bytes_after_gc_chirho")?,
        live_objects_after_gc_high_water_chirho: take_chirho(
            "live_objects_after_gc_high_water_chirho",
        )?,
        live_bytes_after_gc_high_water_chirho: take_chirho(
            "live_bytes_after_gc_high_water_chirho",
        )?,
    };
    if !values_chirho.is_empty() {
        return Err("unknown native report counters".to_string());
    }
    Ok(counters_chirho)
}

#[cfg(test)]
mod tests_chirho {
    use super::parse_counters_chirho;

    #[test]
    fn absent_partial_or_duplicate_reports_cannot_supply_measurements_chirho() {
        for report_chirho in [
            "",
            "version_chirho=1\nroot_depth_chirho=0\n",
            "version_chirho=1\nversion_chirho=1\n",
            "version_chirho=1\nroot_depth_chirho=invalid\n",
        ] {
            assert!(parse_counters_chirho(report_chirho).is_err());
        }
    }
}
