// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin class identity for one module (K01).
//!
//! A class spelling the module can write (`Eq`, `P.Eq`) names a builtin class only when a
//! trusted builtin interface provides it: the interface's origin is builtin, the class is
//! in the compiler's table, the interface is one of that entry's exact providers, and its
//! type export is the class entry, listing the class's witness method. A spelling also
//! bound to anything else (a local declaration, a source interface, an untrusted builtin
//! homonym, a different class) is excluded for good: a later import cannot restore it.
//! workflow: compiler-pipeline-chirho/builtin-class-identity-chirho

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::promotion_scope_chirho::{declared_type_names_chirho, spelling_chirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{
    BuiltinClassChirho, BuiltinClassScopeChirho, CompilerClassChirho,
};

use super::{IfaceOriginChirho, ModuleIfaceChirho};
use crate::env_chirho::NamespaceChirho;
use crate::resolve_chirho::compute_imported_names_chirho;

/// One class the compiler defines, with the builtin interfaces trusted to provide it.
struct BuiltinClassEntryChirho {
    class_chirho: BuiltinClassChirho,
    /// A method the provider's type export must list, so the export is the class itself.
    witness_method_chirho: &'static str,
    /// The builtin interfaces trusted to provide the class, exactly. A builtin homonym
    /// anywhere else stays unproven.
    providers_chirho: &'static [&'static str],
}

/// A parameter of kind `Type`.
const TYPE_PARAMETER_CHIRHO: &[u8] = &[0];
/// A parameter of kind `Type -> Type`.
const CONSTRUCTOR_PARAMETER_CHIRHO: &[u8] = &[1];

const fn entry_chirho(
    identity_chirho: CompilerClassChirho,
    module_chirho: &'static str,
    name_chirho: &'static str,
    parameter_arities_chirho: &'static [u8],
    witness_method_chirho: &'static str,
    providers_chirho: &'static [&'static str],
) -> BuiltinClassEntryChirho {
    BuiltinClassEntryChirho {
        class_chirho: BuiltinClassChirho {
            identity_chirho,
            module_chirho,
            name_chirho,
            parameter_arities_chirho,
        },
        witness_method_chirho,
        providers_chirho,
    }
}

/// The classes the kind pass may judge (#31175, #31176): monomorphic in GHC 9.14.1, with
/// a real method every builtin entry lists. Read (its builtin entry lists `read`, not a
/// method) and Real, RealFrac and RealFloat (no methods listed) stay out. The providers
/// are a draft scanned from the builtin sources, held to the supplier guard and to GHC's
/// export lists before any consumer reads them.
/// workflow: compiler-pipeline-chirho/builtin-class-identity-chirho
const BUILTIN_CLASSES_CHIRHO: &[BuiltinClassEntryChirho] = &[
    entry_chirho(
        CompilerClassChirho::EqChirho,
        "GHC.Classes",
        "Eq",
        TYPE_PARAMETER_CHIRHO,
        "==",
        &["Prelude", "Data.Eq", "GHC.Classes"],
    ),
    entry_chirho(
        CompilerClassChirho::OrdChirho,
        "GHC.Classes",
        "Ord",
        TYPE_PARAMETER_CHIRHO,
        "compare",
        &["Prelude", "Data.Ord", "GHC.Classes"],
    ),
    entry_chirho(
        CompilerClassChirho::ShowChirho,
        "GHC.Show",
        "Show",
        TYPE_PARAMETER_CHIRHO,
        "show",
        &["Prelude", "Text.Show", "GHC.Show"],
    ),
    entry_chirho(
        CompilerClassChirho::EnumChirho,
        "GHC.Enum",
        "Enum",
        TYPE_PARAMETER_CHIRHO,
        "succ",
        &["Prelude", "GHC.Enum"],
    ),
    entry_chirho(
        CompilerClassChirho::BoundedChirho,
        "GHC.Enum",
        "Bounded",
        TYPE_PARAMETER_CHIRHO,
        "minBound",
        &["Prelude", "GHC.Enum"],
    ),
    entry_chirho(
        CompilerClassChirho::NumChirho,
        "GHC.Num",
        "Num",
        TYPE_PARAMETER_CHIRHO,
        "fromInteger",
        &["Prelude", "GHC.Num"],
    ),
    entry_chirho(
        CompilerClassChirho::IntegralChirho,
        "GHC.Real",
        "Integral",
        TYPE_PARAMETER_CHIRHO,
        "toInteger",
        &["Prelude", "GHC.Real"],
    ),
    entry_chirho(
        CompilerClassChirho::FractionalChirho,
        "GHC.Real",
        "Fractional",
        TYPE_PARAMETER_CHIRHO,
        "fromRational",
        &["Prelude", "GHC.Real"],
    ),
    entry_chirho(
        CompilerClassChirho::FloatingChirho,
        "GHC.Float",
        "Floating",
        TYPE_PARAMETER_CHIRHO,
        "pi",
        &["Prelude", "GHC.Float"],
    ),
    entry_chirho(
        CompilerClassChirho::SemigroupChirho,
        "GHC.Base",
        "Semigroup",
        TYPE_PARAMETER_CHIRHO,
        "<>",
        &["Prelude", "Data.Semigroup", "GHC.Base"],
    ),
    entry_chirho(
        CompilerClassChirho::MonoidChirho,
        "GHC.Base",
        "Monoid",
        TYPE_PARAMETER_CHIRHO,
        "mempty",
        &["Prelude", "Data.Monoid", "GHC.Base"],
    ),
    entry_chirho(
        CompilerClassChirho::FunctorChirho,
        "GHC.Base",
        "Functor",
        CONSTRUCTOR_PARAMETER_CHIRHO,
        "fmap",
        &["Prelude", "Data.Functor", "GHC.Base"],
    ),
    entry_chirho(
        CompilerClassChirho::ApplicativeChirho,
        "GHC.Base",
        "Applicative",
        CONSTRUCTOR_PARAMETER_CHIRHO,
        "pure",
        &["Prelude", "Control.Applicative", "GHC.Base"],
    ),
    entry_chirho(
        CompilerClassChirho::MonadChirho,
        "GHC.Base",
        "Monad",
        CONSTRUCTOR_PARAMETER_CHIRHO,
        ">>=",
        &["Prelude", "Control.Monad", "GHC.Base"],
    ),
    entry_chirho(
        CompilerClassChirho::MonadFailChirho,
        "Control.Monad.Fail",
        "MonadFail",
        CONSTRUCTOR_PARAMETER_CHIRHO,
        "fail",
        &["Prelude", "Control.Monad", "Control.Monad.Fail"],
    ),
    entry_chirho(
        CompilerClassChirho::FoldableChirho,
        "Data.Foldable",
        "Foldable",
        CONSTRUCTOR_PARAMETER_CHIRHO,
        "foldr",
        &["Prelude", "Data.Foldable"],
    ),
    entry_chirho(
        CompilerClassChirho::TraversableChirho,
        "Data.Traversable",
        "Traversable",
        CONSTRUCTOR_PARAMETER_CHIRHO,
        "traverse",
        &["Prelude", "Data.Traversable"],
    ),
];

/// Every table-named spelling bound so far: its class, or `None` once excluded.
type ClassBindingsChirho = HashMap<String, Option<&'static BuiltinClassEntryChirho>>;

/// One module's builtin class scope, from its declarations and the interfaces its
/// imports name, the same ones resolution reads.
/// workflow: compiler-pipeline-chirho/builtin-class-identity-chirho
pub fn build_builtin_class_scope_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> BuiltinClassScopeChirho {
    let mut bindings_chirho = ClassBindingsChirho::new();
    let module_name_chirho = module_chirho.name_chirho.full_name_chirho();
    for decl_chirho in &module_chirho.decls_chirho {
        for type_name_chirho in declared_type_names_chirho(decl_chirho) {
            if table_entry_chirho(type_name_chirho).is_none() {
                continue;
            }
            bind_chirho(
                &mut bindings_chirho,
                spelling_chirho(None, type_name_chirho),
                None,
            );
            bind_chirho(
                &mut bindings_chirho,
                spelling_chirho(Some(&module_name_chirho), type_name_chirho),
                None,
            );
        }
    }
    for import_chirho in &module_chirho.imports_chirho {
        let imported_module_chirho = import_chirho.module_chirho.full_name_chirho();
        // Resolution reports a missing interface; its names stay unbound here.
        let Some(iface_chirho) = ifaces_chirho
            .iter()
            .rev()
            .find(|iface_chirho| iface_chirho.name_chirho == imported_module_chirho)
        else {
            continue;
        };
        let qualifier_chirho = import_chirho
            .alias_chirho
            .as_ref()
            .map_or(imported_module_chirho.clone(), |alias_chirho| {
                alias_chirho.text_chirho().to_string()
            });
        for (name_chirho, namespace_chirho, _) in
            compute_imported_names_chirho(&iface_chirho.exports_chirho, &import_chirho.spec_chirho)
        {
            if namespace_chirho != NamespaceChirho::TypeChirho
                || table_entry_chirho(&name_chirho).is_none()
            {
                continue;
            }
            let candidate_chirho = trusted_builtin_class_chirho(iface_chirho, &name_chirho);
            bind_chirho(
                &mut bindings_chirho,
                spelling_chirho(Some(&qualifier_chirho), &name_chirho),
                candidate_chirho,
            );
            if !import_chirho.qualified_chirho {
                bind_chirho(&mut bindings_chirho, name_chirho, candidate_chirho);
            }
        }
    }
    BuiltinClassScopeChirho::new_chirho(
        bindings_chirho
            .into_iter()
            .filter_map(|(spelling_chirho, bound_chirho)| {
                bound_chirho.map(|entry_chirho| (spelling_chirho, entry_chirho.class_chirho))
            })
            .collect(),
    )
}

/// The table entry named `name_chirho`, if the compiler defines such a class.
fn table_entry_chirho(name_chirho: &str) -> Option<&'static BuiltinClassEntryChirho> {
    BUILTIN_CLASSES_CHIRHO
        .iter()
        .find(|entry_chirho| entry_chirho.class_chirho.name_chirho == name_chirho)
}

/// The table entry that trusts `iface_chirho` to give `name_chirho` the builtin class's
/// identity: a builtin interface, one of the entry's providers, and a type export that is
/// the class entry, listing the witness method.
fn trusted_builtin_class_chirho(
    iface_chirho: &ModuleIfaceChirho,
    name_chirho: &str,
) -> Option<&'static BuiltinClassEntryChirho> {
    if iface_chirho.exports_chirho.origin_chirho != IfaceOriginChirho::BuiltinChirho {
        return None;
    }
    let entry_chirho = table_entry_chirho(name_chirho)?;
    if !entry_chirho
        .providers_chirho
        .contains(&iface_chirho.name_chirho.as_str())
    {
        return None;
    }
    let export_chirho = iface_chirho.exports_chirho.types_chirho.get(name_chirho)?;
    export_chirho
        .methods_chirho
        .iter()
        .any(|method_chirho| method_chirho == entry_chirho.witness_method_chirho)
        .then_some(entry_chirho)
}

/// Bind `spelling_chirho` to `candidate_chirho`. Exclusion is monotonic: a spelling bound
/// to anything but one builtin class stays excluded whatever is bound after it.
fn bind_chirho(
    bindings_chirho: &mut ClassBindingsChirho,
    spelling_chirho: String,
    candidate_chirho: Option<&'static BuiltinClassEntryChirho>,
) {
    match bindings_chirho.entry(spelling_chirho) {
        Entry::Vacant(vacant_chirho) => {
            vacant_chirho.insert(candidate_chirho);
        }
        Entry::Occupied(mut occupied_chirho) => {
            let same_class_chirho = matches!(
                (*occupied_chirho.get(), candidate_chirho),
                (Some(bound_chirho), Some(new_chirho))
                    if bound_chirho.class_chirho.name_chirho == new_chirho.class_chirho.name_chirho
            );
            if !same_class_chirho {
                occupied_chirho.insert(None);
            }
        }
    }
}

#[cfg(test)]
#[path = "class_identity_tests_chirho.rs"]
mod tests_chirho;
