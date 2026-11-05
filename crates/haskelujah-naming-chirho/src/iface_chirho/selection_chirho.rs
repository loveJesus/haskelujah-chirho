// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Select an interface exposure from the actual import edge. This metadata
//! never constructs or changes a source-unit, class or type identity proof.

use super::ModuleIfaceChirho;
use haskelujah_ast_chirho::module_chirho::ImportDeclChirho;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IfaceExposureChirho {
    /// Admitted from the actual current source unit, after its authority check.
    HomeChirho,
    /// The positive provider exposes this module under this package selector.
    PackagesChirho(Arc<[Arc<str>]>),
}

impl IfaceExposureChirho {
    /// Positive providers can expose one module through several packages via
    /// re-export. The list is an exposure claim, never a declaration unit ID.
    pub fn packages_from_producer_chirho(selectors_chirho: &[&str]) -> Self {
        let mut selectors_chirho = selectors_chirho
            .iter()
            .map(|selector_chirho| Arc::<str>::from(*selector_chirho))
            .collect::<Vec<_>>();
        selectors_chirho.sort();
        selectors_chirho.dedup();
        Self::PackagesChirho(selectors_chirho.into())
    }
}

/// All ordinary import consumers use the same candidate. A package spelling
/// filters producer-owned exposure facts; it never upgrades unknown metadata.
/// Default imports retain legacy lookup only when no admitted home exists.
pub fn select_import_iface_chirho<'iface_chirho>(
    available_chirho: &'iface_chirho [ModuleIfaceChirho],
    import_chirho: &ImportDeclChirho,
) -> Option<&'iface_chirho ModuleIfaceChirho> {
    let name_chirho = import_chirho.module_chirho.full_name_chirho();
    match import_chirho.package_chirho.as_deref() {
        Some("this") | None => {
            let mut home_chirho = available_chirho.iter().filter(|iface_chirho| {
                iface_chirho.name_chirho == name_chirho
                    && iface_chirho.exposure_chirho == Some(IfaceExposureChirho::HomeChirho)
            });
            if let Some(first_chirho) = home_chirho.next() {
                return home_chirho.next().is_none().then_some(first_chirho);
            }
            if import_chirho.package_chirho.is_some() {
                return None;
            }
            available_chirho
                .iter()
                .rev()
                .find(|iface_chirho| iface_chirho.name_chirho == name_chirho)
        }
        Some(selector_chirho) => {
            let mut selected_chirho = available_chirho.iter().filter(|iface_chirho| iface_chirho.name_chirho == name_chirho && matches!(&iface_chirho.exposure_chirho, Some(IfaceExposureChirho::PackagesChirho(actual_chirho)) if actual_chirho.iter().any(|package_chirho| package_chirho.as_ref() == selector_chirho)));
            let first_chirho = selected_chirho.next()?;
            selected_chirho.next().is_none().then_some(first_chirho)
        }
    }
}
