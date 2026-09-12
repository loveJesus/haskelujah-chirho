// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded, demand-driven lookup of authoritative source modules.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::{ParserChirho, SourceFileChirho, inject_prelude_import_chirho, lower_module_chirho};
use haskelujah_span_chirho::SourceMapChirho;

pub(crate) const MAX_MODULE_SEARCH_DEPTH_CHIRHO: usize = 64;
pub(crate) const MAX_MODULE_SEARCH_DIRS_CHIRHO: usize = 2048;
pub(crate) const MAX_MODULE_SEARCH_FILES_CHIRHO: usize = 16384;
const MAX_MODULE_SOURCE_BYTES_CHIRHO: usize = 8 * 1024 * 1024;
const MAX_MODULE_CLOSURE_BYTES_CHIRHO: usize = 64 * 1024 * 1024;

/// Invocation-wide admission bounds, shared by all reachable imports.
pub(crate) struct ModuleSearchBoundsChirho {
    pub(crate) dirs_entered_chirho: usize,
    pub(crate) files_read_chirho: usize,
    pub(crate) warned_chirho: bool,
}

impl ModuleSearchBoundsChirho {
    pub(crate) fn new_chirho() -> Self {
        Self {
            dirs_entered_chirho: 0,
            files_read_chirho: 0,
            warned_chirho: false,
        }
    }
    pub(crate) fn may_enter_dir_chirho(&mut self) -> bool {
        if self.dirs_entered_chirho >= MAX_MODULE_SEARCH_DIRS_CHIRHO {
            self.warned_chirho = true;
            return false;
        }
        self.dirs_entered_chirho += 1;
        true
    }
    pub(crate) fn may_read_file_chirho(&mut self) -> bool {
        if self.files_read_chirho >= MAX_MODULE_SEARCH_FILES_CHIRHO {
            self.warned_chirho = true;
            return false;
        }
        self.files_read_chirho += 1;
        true
    }
}

#[derive(Debug)]
pub(crate) struct LocalModuleSourceChirho {
    pub(crate) name_chirho: String,
    pub(crate) path_chirho: PathBuf,
    pub(crate) source_chirho: String,
    pub(crate) imports_chirho: Vec<ModuleSourceKeyChirho>,
}

/// Boot and implementation have one nominal module identity, but are different
/// dependency nodes and expose different checked interfaces.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ModuleSourceKeyChirho {
    pub(crate) name_chirho: String,
    pub(crate) boot_chirho: bool,
}

/// Read actual preprocessed syntax, not a line-oriented import approximation.
pub(crate) fn module_header_chirho(
    source_chirho: &str,
    file_name_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
) -> Result<(String, Vec<ModuleSourceKeyChirho>), String> {
    let file_chirho = SourceFileChirho::from_source_map_chirho(
        source_map_chirho,
        file_name_chirho,
        source_chirho,
    );
    let file_id_chirho = file_chirho.file_id_chirho();
    let mut module_chirho = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let green_chirho = ParserChirho::new_chirho(source_chirho, file_id_chirho).parse_chirho();
        lower_module_chirho(&green_chirho, file_id_chirho)
    }))
    .map_err(|_| format!("cannot parse dependency header in {file_name_chirho}"))?;
    let name_chirho = module_chirho.name_chirho.full_name_chirho();
    // Prelude does not depend on itself via the language's implicit import.
    if name_chirho != "Prelude" {
        inject_prelude_import_chirho(&mut module_chirho);
    }
    let imports_chirho = module_chirho
        .imports_chirho
        .iter()
        .map(|import_chirho| ModuleSourceKeyChirho {
            name_chirho: import_chirho.module_chirho.full_name_chirho(),
            boot_chirho: import_chirho.source_chirho,
        })
        .collect();
    Ok((name_chirho, imports_chirho))
}

pub(crate) struct LocalModuleSearchChirho {
    root_chirho: PathBuf,
    pub(crate) bounds_chirho: ModuleSearchBoundsChirho,
    directories_chirho: HashSet<PathBuf>,
    candidates_chirho: HashMap<PathBuf, Option<Arc<LocalModuleSourceChirho>>>,
    source_bytes_chirho: usize,
}

impl LocalModuleSearchChirho {
    pub(crate) fn new_chirho(root_chirho: &Path) -> Result<Self, String> {
        // Bare File.hs and ./File.hs select the same active root. Resolve the
        // caller's root once; never follow symlinks below that boundary.
        let root_chirho = if root_chirho.as_os_str().is_empty() {
            Path::new(".")
        } else {
            root_chirho
        };
        let root_chirho = root_chirho.canonicalize().map_err(|error_chirho| {
            format!(
                "cannot open source root {}: {error_chirho}",
                root_chirho.display()
            )
        })?;
        Ok(Self {
            root_chirho,
            bounds_chirho: ModuleSearchBoundsChirho::new_chirho(),
            directories_chirho: HashSet::new(),
            candidates_chirho: HashMap::new(),
            source_bytes_chirho: 0,
        })
    }

    /// Hierarchical ownership precedes a same-directory flat fixture. Both
    /// paths must declare the complete requested name, one candidate per rank.
    pub(crate) fn lookup_chirho(
        &mut self,
        module_name_chirho: &str,
        source_map_chirho: &mut SourceMapChirho,
    ) -> Result<Option<Arc<LocalModuleSourceChirho>>, String> {
        self.lookup_form_chirho(module_name_chirho, false, source_map_chirho)
    }

    pub(crate) fn lookup_boot_chirho(
        &mut self,
        module_name_chirho: &str,
        source_map_chirho: &mut SourceMapChirho,
    ) -> Result<Arc<LocalModuleSourceChirho>, String> {
        // A boot belongs beside the selected implementation. A lower-priority
        // flat boot may not replace a missing hierarchical owner's promise.
        let candidate_chirho = if let Some(implementation_chirho) =
            self.lookup_chirho(module_name_chirho, source_map_chirho)?
        {
            let relative_chirho = implementation_chirho
                .path_chirho
                .strip_prefix(&self.root_chirho)
                .map_err(|_| "boot owner is outside the active source root")?
                .with_extension("hs-boot");
            self.read_candidate_chirho(&relative_chirho, source_map_chirho)?
        } else {
            self.lookup_form_chirho(module_name_chirho, true, source_map_chirho)?
        };
        candidate_chirho
            .filter(|candidate_chirho| candidate_chirho.name_chirho == module_name_chirho)
            .ok_or_else(|| {
                format!(
                    "missing or incorrectly named boot contract: {}.hs-boot",
                    module_name_chirho.replace('.', "/")
                )
            })
    }

    fn lookup_form_chirho(
        &mut self,
        module_name_chirho: &str,
        boot_chirho: bool,
        source_map_chirho: &mut SourceMapChirho,
    ) -> Result<Option<Arc<LocalModuleSourceChirho>>, String> {
        let components_chirho: Vec<_> = module_name_chirho.split('.').collect();
        if components_chirho.len() > MAX_MODULE_SEARCH_DEPTH_CHIRHO
            || components_chirho.iter().any(|part_chirho| {
                part_chirho.is_empty()
                    || !part_chirho.chars().all(|character_chirho| {
                        character_chirho.is_alphanumeric() || matches!(character_chirho, '_' | '\'')
                    })
            })
        {
            return Err(format!(
                "invalid or over-depth source module name: {module_name_chirho}"
            ));
        }
        let mut hierarchical_chirho = PathBuf::new();
        for component_chirho in &components_chirho {
            hierarchical_chirho.push(component_chirho);
        }
        let extension_chirho = if boot_chirho { "hs-boot" } else { "hs" };
        hierarchical_chirho.set_extension(extension_chirho);
        let flat_chirho = PathBuf::from(format!(
            "{}.{extension_chirho}",
            components_chirho.last().unwrap()
        ));
        let candidates_chirho = if flat_chirho == hierarchical_chirho {
            vec![hierarchical_chirho]
        } else {
            vec![hierarchical_chirho, flat_chirho]
        };
        for relative_chirho in candidates_chirho {
            if let Some(candidate_chirho) =
                self.read_candidate_chirho(&relative_chirho, source_map_chirho)?
                && candidate_chirho.name_chirho == module_name_chirho
            {
                return Ok(Some(candidate_chirho));
            }
        }
        Ok(None)
    }

    fn read_candidate_chirho(
        &mut self,
        relative_chirho: &Path,
        source_map_chirho: &mut SourceMapChirho,
    ) -> Result<Option<Arc<LocalModuleSourceChirho>>, String> {
        if let Some(cached_chirho) = self.candidates_chirho.get(relative_chirho) {
            return Ok(cached_chirho.clone());
        }
        let candidate_chirho =
            self.read_uncached_candidate_chirho(relative_chirho, source_map_chirho)?;
        self.candidates_chirho
            .insert(relative_chirho.to_owned(), candidate_chirho.clone());
        Ok(candidate_chirho)
    }

    fn read_uncached_candidate_chirho(
        &mut self,
        relative_chirho: &Path,
        source_map_chirho: &mut SourceMapChirho,
    ) -> Result<Option<Arc<LocalModuleSourceChirho>>, String> {
        let mut path_chirho = self.root_chirho.clone();
        let count_chirho = relative_chirho.components().count();
        for (index_chirho, component_chirho) in relative_chirho.components().enumerate() {
            path_chirho.push(component_chirho);
            let metadata_chirho = match std::fs::symlink_metadata(&path_chirho) {
                Ok(metadata_chirho) => metadata_chirho,
                Err(error_chirho) if error_chirho.kind() == std::io::ErrorKind::NotFound => {
                    return Ok(None);
                }
                Err(error_chirho) => {
                    return Err(format!(
                        "cannot inspect source candidate {}: {error_chirho}",
                        path_chirho.display()
                    ));
                }
            };
            if metadata_chirho.file_type().is_symlink() {
                return Ok(None);
            }
            if index_chirho + 1 < count_chirho {
                if !metadata_chirho.is_dir() {
                    return Ok(None);
                }
                if self.directories_chirho.insert(path_chirho.clone())
                    && !self.bounds_chirho.may_enter_dir_chirho()
                {
                    return Err("module search directory budget exhausted".to_owned());
                }
            } else if !metadata_chirho.is_file() {
                return Ok(None);
            }
        }
        if !self.bounds_chirho.may_read_file_chirho() {
            return Err("module search source budget exhausted".to_owned());
        }
        let file_chirho = std::fs::File::open(&path_chirho).map_err(|error_chirho| {
            format!(
                "cannot read source candidate {}: {error_chirho}",
                path_chirho.display()
            )
        })?;
        let mut raw_chirho = String::new();
        file_chirho
            .take((MAX_MODULE_SOURCE_BYTES_CHIRHO + 1) as u64)
            .read_to_string(&mut raw_chirho)
            .map_err(|error_chirho| {
                format!(
                    "cannot read source candidate {}: {error_chirho}",
                    path_chirho.display()
                )
            })?;
        if raw_chirho.len() > MAX_MODULE_SOURCE_BYTES_CHIRHO {
            return Err(format!(
                "module source byte budget exhausted: {}",
                path_chirho.display()
            ));
        }
        // A failed preprocessor is a failed provider, not permission to strip
        // directives and publish an interface for a different program.
        let source_chirho =
            crate::preprocess_cpp_source_with_options_chirho(&path_chirho, &raw_chirho, &[])
                .map_err(|error_chirho| error_chirho.to_string())?;
        let source_chirho = crate::lower_maybe_like_unboxed_sums_chirho(&source_chirho);
        self.source_bytes_chirho += raw_chirho.len().max(source_chirho.len());
        if source_chirho.len() > MAX_MODULE_SOURCE_BYTES_CHIRHO
            || self.source_bytes_chirho > MAX_MODULE_CLOSURE_BYTES_CHIRHO
        {
            return Err(format!(
                "module closure byte budget exhausted at {}",
                path_chirho.display()
            ));
        }
        let (name_chirho, imports_chirho) = module_header_chirho(
            &source_chirho,
            &path_chirho.to_string_lossy(),
            source_map_chirho,
        )?;
        Ok(Some(Arc::new(LocalModuleSourceChirho {
            name_chirho,
            path_chirho,
            source_chirho,
            imports_chirho,
        })))
    }
}
