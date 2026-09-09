// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Object target metadata must describe the selected platform so the linker
//! need not guess. Darwin's kernel version is not a macOS deployment version.

pub(super) fn annotate_platform_chirho(
    object_chirho: &mut cranelift_object::object::write::Object<'_>,
    triple_chirho: &str,
) -> Result<(), String> {
    let triple_chirho: target_lexicon::Triple = triple_chirho
        .parse()
        .map_err(|error_chirho| format!("invalid object target: {error_chirho}"))?;
    let version_chirho = match triple_chirho.operating_system {
        target_lexicon::OperatingSystem::MacOSX(Some(version_chirho)) => (
            version_chirho.major,
            version_chirho.minor,
            version_chirho.patch,
        ),
        target_lexicon::OperatingSystem::Darwin(_)
        | target_lexicon::OperatingSystem::MacOSX(None) => {
            haskelujah_rts_chirho::target_chirho::MACOS_DEPLOYMENT_CHIRHO
        }
        _ => return Ok(()),
    };
    let (major_chirho, minor_chirho, patch_chirho) = version_chirho;
    let mut build_chirho = cranelift_object::object::write::MachOBuildVersion::default();
    build_chirho.platform = cranelift_object::object::macho::PLATFORM_MACOS;
    build_chirho.minos =
        (u32::from(major_chirho) << 16) | (u32::from(minor_chirho) << 8) | u32::from(patch_chirho);
    // This backend does not consume an SDK; zero records that fact honestly.
    object_chirho.set_macho_build_version(build_chirho);
    Ok(())
}
