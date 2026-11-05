// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The deriving extensions a module enables, read as GHC 9.14.1 reads them (row 694). Root's
//! fresh contracts pin each rule: a strategy needs DerivingStrategies (A1-A3), anyclass needs
//! DeriveAnyClass (B1-B4), GND is on under GHC2021 and GHC2024 and off under an explicit
//! Haskell2010 (C1-C5), and the last edition or written flag decides (O1-O4). The parser
//! appends each written flag after every edition, so last-wins here is GHC's order.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use haskelujah_ast_chirho::module_chirho::ModuleChirho;

/// What a module enables for deriving.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DerivingExtensionsChirho {
    /// DerivingStrategies, or DerivingVia, which implies it: a strategy may be written.
    pub(crate) strategies_chirho: bool,
    /// DeriveAnyClass.
    pub(crate) any_class_chirho: bool,
    /// GeneralizedNewtypeDeriving in either spelling; on unless an explicit Haskell2010 or
    /// Haskell98 edition, or a written flag, turns it off.
    pub(crate) newtype_chirho: bool,
}

impl DerivingExtensionsChirho {
    /// The deriving extensions `module_chirho`'s pragmas enable.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(crate) fn of_module_chirho(module_chirho: &ModuleChirho) -> Self {
        let extensions_chirho = &module_chirho.extensions_chirho;
        Self {
            strategies_chirho: last_setting_chirho(
                extensions_chirho,
                &["DerivingStrategies", "DerivingVia"],
                &["NoDerivingStrategies"],
            )
            .unwrap_or(false),
            any_class_chirho: last_setting_chirho(
                extensions_chirho,
                &["DeriveAnyClass"],
                &["NoDeriveAnyClass"],
            )
            .unwrap_or(false),
            newtype_chirho: last_setting_chirho(
                extensions_chirho,
                &["GeneralizedNewtypeDeriving", "GeneralisedNewtypeDeriving"],
                &[
                    "NoGeneralizedNewtypeDeriving",
                    "NoGeneralisedNewtypeDeriving",
                    "Haskell2010",
                    "Haskell98",
                ],
            )
            .unwrap_or(true),
        }
    }
}

/// The last flag naming one of `on_chirho` (true) or `off_chirho` (false), if any.
fn last_setting_chirho(
    extensions_chirho: &[String],
    on_chirho: &[&str],
    off_chirho: &[&str],
) -> Option<bool> {
    extensions_chirho.iter().rev().find_map(|extension_chirho| {
        if on_chirho.contains(&extension_chirho.as_str()) {
            Some(true)
        } else if off_chirho.contains(&extension_chirho.as_str()) {
            Some(false)
        } else {
            None
        }
    })
}
