// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Each CPP invocation owns its temporary input and support headers through exit.
//! Workflow: spec-chirho/workflows-chirho/testing-chirho/execution-oracles-chirho.md.

use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output};

use crate::{
    CPP_GHC_VERSION_MAJOR_CHIRHO, CPP_GHC_VERSION_MINOR_CHIRHO, CPP_GHC_VERSION_PATCH_CHIRHO,
    CPP_GLASGOW_HASKELL_VERSION_CHIRHO, cpp_include_dirs_chirho,
    discover_cpp_min_version_macro_options_chirho,
};

pub(crate) fn temporary_source_file_chirho(
    source_chirho: &str,
    parent_chirho: Option<&Path>,
    suffix_chirho: &str,
) -> io::Result<tempfile::NamedTempFile> {
    let mut builder_chirho = tempfile::Builder::new();
    builder_chirho
        .prefix(".haskelujah-source-chirho-")
        .suffix(suffix_chirho);
    let mut file_chirho = if let Some(parent_chirho) = parent_chirho {
        let parent_chirho = if parent_chirho.as_os_str().is_empty() {
            Path::new(".")
        } else {
            parent_chirho
        };
        builder_chirho.tempfile_in(parent_chirho)?
    } else {
        builder_chirho.tempfile()?
    };
    file_chirho.write_all(source_chirho.as_bytes())?;
    Ok(file_chirho)
}

pub(crate) struct CppInvocationChirho {
    command_chirho: Command,
    support_chirho: tempfile::TempDir,
}

impl CppInvocationChirho {
    pub(crate) fn output_chirho(mut self) -> io::Result<Output> {
        let result_chirho = self.command_chirho.output();
        // Even an unsuccessful child has finished reading before its headers retire.
        drop(self.support_chirho);
        result_chirho
    }
}

pub(crate) fn configure_cpp_command_chirho(
    path_chirho: &Path,
    traditional_chirho: bool,
    extra_cpp_options_chirho: &[String],
) -> io::Result<CppInvocationChirho> {
    let support_chirho = tempfile::tempdir()?;
    // Directive-only headers do not leak C comments into traditional CPP output.
    std::fs::write(
        support_chirho.path().join("MachDeps.h"),
        concat!(
            "#ifndef HASKELUJAH_SYNTHETIC_MACHDEPS_H_CHIRHO\n",
            "#define HASKELUJAH_SYNTHETIC_MACHDEPS_H_CHIRHO\n",
            "#define WORD_SIZE_IN_BITS 64\n",
            "#endif\n",
        ),
    )?;
    std::fs::write(
        support_chirho.path().join("HsBaseConfig.h"),
        concat!(
            "#ifndef HASKELUJAH_SYNTHETIC_HSBASECONFIG_H_CHIRHO\n",
            "#define HASKELUJAH_SYNTHETIC_HSBASECONFIG_H_CHIRHO\n",
            "#endif\n",
        ),
    )?;
    let mut command_chirho = Command::new("cpp");
    if traditional_chirho {
        command_chirho.arg("-traditional");
    }
    command_chirho
        .arg("-P")
        .arg(format!("-D__GLASGOW_HASKELL__={CPP_GLASGOW_HASKELL_VERSION_CHIRHO}"))
        .arg("-DWORD_SIZE_IN_BITS=64")
        .arg(format!("-DMIN_VERSION_ghc(x,y,z)=((x)<{CPP_GHC_VERSION_MAJOR_CHIRHO}||((x)=={CPP_GHC_VERSION_MAJOR_CHIRHO}&&((y)<{CPP_GHC_VERSION_MINOR_CHIRHO}||((y)=={CPP_GHC_VERSION_MINOR_CHIRHO}&&(z)<={CPP_GHC_VERSION_PATCH_CHIRHO}))))"))
        .arg(format!("-DMIN_VERSION_GLASGOW_HASKELL(x,y,z,w)=((x)<{CPP_GHC_VERSION_MAJOR_CHIRHO}||((x)=={CPP_GHC_VERSION_MAJOR_CHIRHO}&&((y)<{CPP_GHC_VERSION_MINOR_CHIRHO}||((y)=={CPP_GHC_VERSION_MINOR_CHIRHO}&&((z)<{CPP_GHC_VERSION_PATCH_CHIRHO}||((z)=={CPP_GHC_VERSION_PATCH_CHIRHO}&&(w)<=0))))))"));
    command_chirho.args(discover_cpp_min_version_macro_options_chirho(path_chirho));
    command_chirho.args(extra_cpp_options_chirho);
    command_chirho.arg(format!("-I{}", support_chirho.path().display()));
    for directory_chirho in cpp_include_dirs_chirho(path_chirho) {
        let absolute_chirho = if directory_chirho.is_absolute() {
            directory_chirho
        } else {
            std::env::current_dir()?.join(directory_chirho)
        };
        command_chirho.arg(format!("-I{}", absolute_chirho.display()));
    }
    if let Some(parent_chirho) = path_chirho
        .parent()
        .filter(|parent_chirho| !parent_chirho.as_os_str().is_empty())
    {
        command_chirho.current_dir(parent_chirho);
        command_chirho.arg(
            path_chirho
                .file_name()
                .map(Path::new)
                .unwrap_or(path_chirho),
        );
    } else {
        command_chirho.arg(path_chirho);
    }
    Ok(CppInvocationChirho {
        command_chirho,
        support_chirho,
    })
}
