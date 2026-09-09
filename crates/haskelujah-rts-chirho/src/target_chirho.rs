// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared native artifact/linker target policy. Generated IR, object platform
//! metadata and the native linker must agree, rather than relying on overrides.

/// Default minimum for generated macOS artifacts, including Apple Silicon.
/// This is a deployment policy, not the running kernel or installed SDK version.
pub const MACOS_DEPLOYMENT_CHIRHO: (u16, u8, u8) = (11, 0, 0);

pub fn native_target_chirho() -> String {
    let host_chirho = target_lexicon::Triple::host();
    if matches!(
        host_chirho.operating_system,
        target_lexicon::OperatingSystem::Darwin(_) | target_lexicon::OperatingSystem::MacOSX(_)
    ) {
        let architecture_chirho = match host_chirho.architecture {
            target_lexicon::Architecture::Aarch64(_) => "arm64".to_string(),
            architecture_chirho => architecture_chirho.to_string(),
        };
        let (major_chirho, minor_chirho, patch_chirho) = MACOS_DEPLOYMENT_CHIRHO;
        format!("{architecture_chirho}-apple-macosx{major_chirho}.{minor_chirho}.{patch_chirho}")
    } else {
        host_chirho.to_string()
    }
}
