// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! File discovery keeps home and explicit package selections distinct.
//! Sources are the unchanged, independently GHC-measured row688 project controls.
use std::path::PathBuf;

struct ProjectScratchChirho(PathBuf);
impl ProjectScratchChirho {
    fn new_chirho() -> Self {
        static NEXT_CHIRHO: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let path_chirho = std::env::temp_dir().join(format!(
            "package_selection_chirho_{}_{}",
            std::process::id(),
            NEXT_CHIRHO.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&path_chirho).expect("create project fixture");
        Self(path_chirho)
    }
}
impl Drop for ProjectScratchChirho {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn project_chirho(source_chirho: &str, dependencies_chirho: &[(&str, &str)], accepts_chirho: bool) {
    let scratch_chirho = ProjectScratchChirho::new_chirho();
    let checked_chirho = scratch_chirho.0.join("MainChirho.hs");
    std::fs::write(&checked_chirho, source_chirho).expect("write main source");
    for (name_chirho, source_chirho) in dependencies_chirho {
        let path_chirho = scratch_chirho.0.join(name_chirho);
        std::fs::create_dir_all(path_chirho.parent().expect("dependency parent"))
            .expect("create dependency directory");
        std::fs::write(&path_chirho, source_chirho).expect("write dependency");
    }
    let result_chirho =
        crate::check_source_path_chirho(&checked_chirho, crate::ExecutionModeChirho::BatchChirho);
    if accepts_chirho {
        result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
    } else {
        let diagnostics_chirho =
            result_chirho.expect_err("invalid source must reject through its selected provider");
        assert!(
            diagnostics_chirho.to_string().contains("error[E"),
            "{diagnostics_chirho}"
        );
    }
}

#[test]
fn mixed_package_home_positive_chirho() {
    project_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PackageImports, PolyKinds #-}
module MainChirho where
import qualified Data.Proxy as HomeChirho
import qualified "base" Data.Proxy as BaseChirho
data UsesChirho = UsesChirho (HomeChirho.Proxy Int) (BaseChirho.Proxy Maybe)
"####,
        &[(
            "Data/Proxy.hs",
            r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE KindSignatures #-}
module Data.Proxy where
import Data.Kind (Type)
data Proxy (aChirho :: Type) = SourceProxyChirho
"####,
        )],
        true,
    );
}

#[test]
fn mixed_package_home_wrong_chirho() {
    project_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PackageImports, PolyKinds #-}
module MainChirho where
import qualified Data.Proxy as HomeChirho
import qualified "base" Data.Proxy as BaseChirho
data UsesChirho = UsesChirho (HomeChirho.Proxy Maybe) (BaseChirho.Proxy Int)
"####,
        &[(
            "Data/Proxy.hs",
            r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE KindSignatures #-}
module Data.Proxy where
import Data.Kind (Type)
data Proxy (aChirho :: Type) = SourceProxyChirho
"####,
        )],
        false,
    );
}

#[test]
fn explicit_home_package_positive_chirho() {
    project_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PackageImports, PolyKinds #-}
module MainChirho where
import "this" Data.Proxy
valueChirho :: Proxy Int
valueChirho = SourceProxyChirho
"####,
        &[(
            "Data/Proxy.hs",
            r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE KindSignatures #-}
module Data.Proxy where
import Data.Kind (Type)
data Proxy (aChirho :: Type) = SourceProxyChirho
"####,
        )],
        true,
    );
}

#[test]
fn explicit_home_package_wrong_chirho() {
    project_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PackageImports, PolyKinds #-}
module MainChirho where
import "this" Data.Proxy
type WrongChirho = Proxy Maybe
"####,
        &[(
            "Data/Proxy.hs",
            r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE KindSignatures #-}
module Data.Proxy where
import Data.Kind (Type)
data Proxy (aChirho :: Type) = SourceProxyChirho
"####,
        )],
        false,
    );
}

#[test]
fn explicit_home_package_syntax_wrong_chirho() {
    project_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PackageImports, PolyKinds #-}
module MainChirho where
import "this" Data.Proxy ()
valueChirho :: Int
valueChirho = 0
"####,
        &[(
            "Data/Proxy.hs",
            r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Data.Proxy where
Just valueChirho -> valueChirho
"####,
        )],
        false,
    );
}

#[test]
fn hierarchical_source_proxy_positive_chirho() {
    project_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module MainChirho where
import Data.Proxy
valueChirho :: Proxy Int
valueChirho = SourceProxyChirho
"####,
        &[(
            "Data/Proxy.hs",
            r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Data.Proxy where
data Proxy aChirho = SourceProxyChirho
"####,
        )],
        true,
    );
}

#[test]
fn hierarchical_source_proxy_wrong_chirho() {
    project_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module MainChirho where
import Data.Proxy
type WrongChirho = Proxy Int
"####,
        &[(
            "Data/Proxy.hs",
            r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Data.Proxy where
data Proxy = SourceProxyChirho
"####,
        )],
        false,
    );
}

#[test]
fn hierarchical_source_syntax_wrong_chirho() {
    project_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module MainChirho where
import Data.Proxy ()
valueChirho :: Int
valueChirho = 0
"####,
        &[(
            "Data/Proxy.hs",
            r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Data.Proxy where
Just valueChirho -> valueChirho
"####,
        )],
        false,
    );
}

#[test]
fn hierarchical_unreached_syntax_positive_chirho() {
    project_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module MainChirho where
valueChirho :: Int
valueChirho = 0
"####,
        &[(
            "Data/Proxy.hs",
            r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Data.Proxy where
Just valueChirho -> valueChirho
"####,
        )],
        true,
    );
}

#[test]
fn package_qualified_ignores_home_syntax_chirho() {
    project_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PackageImports #-}
module MainChirho where
import "base" Data.Proxy ()
valueChirho :: Int
valueChirho = 0
"####,
        &[(
            "Data/Proxy.hs",
            r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Data.Proxy where
Just valueChirho -> valueChirho
"####,
        )],
        true,
    );
}

#[test]
fn project_dependency_edges_keep_only_home_selections_chirho() {
    let source_chirho = r#"module GraphChirho where
import Data.Proxy
import qualified "base" Data.Proxy as BaseChirho
import {-# SOURCE #-} "this" HomeChirho
import qualified "th\105s" EscapedHomeChirho
import "ba\115e" Data.Kind
"#;
    let mut home_chirho = crate::project_dir_chirho::extract_home_imports_chirho(source_chirho);
    home_chirho.sort();
    assert_eq!(
        home_chirho,
        ["Data.Proxy", "EscapedHomeChirho", "HomeChirho"]
    );
    assert_eq!(
        crate::project_dir_chirho::extract_imports_chirho(source_chirho).len(),
        5
    );
}

#[test]
fn final_home_publication_replaces_its_boot_without_erasing_package_chirho() {
    use haskelujah_naming_chirho::iface_chirho::{
        IfaceExportsChirho, IfaceExposureChirho, ModuleIfaceChirho, select_import_iface_chirho,
    };
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    for _round_chirho in 0..2 {
        super::admit_home_iface_chirho(
            &mut ifaces_chirho,
            ModuleIfaceChirho {
                name_chirho: "Data.Proxy".into(),
                exposure_chirho: None,
                exports_chirho: IfaceExportsChirho::default(),
            },
        );
    }
    let mut module_chirho = haskelujah_parser_chirho::lower_chirho::lower_module_chirho(
        &haskelujah_parser_chirho::cst_parser_chirho::ParserChirho::new_chirho(
            "module GraphChirho where\nimport \"this\" Data.Proxy\n",
            haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
        )
        .parse_chirho(),
        haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
    );
    let import_chirho = &mut module_chirho.imports_chirho[0];
    assert_eq!(
        select_import_iface_chirho(&ifaces_chirho, import_chirho)
            .expect("one replaced home lane")
            .exposure_chirho,
        Some(IfaceExposureChirho::HomeChirho)
    );
    import_chirho.package_chirho = Some("base".into());
    assert!(
        select_import_iface_chirho(&ifaces_chirho, import_chirho)
            .expect("original package survives")
            .exports_chirho
            .types_chirho
            .contains_key("Proxy")
    );
}
