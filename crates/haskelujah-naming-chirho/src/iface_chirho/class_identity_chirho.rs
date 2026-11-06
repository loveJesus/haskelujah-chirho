// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin class identity for one module (K01).
//!
//! A class spelling the module can write (`Eq`, `P.Eq`) names a builtin class only when a
//! trusted builtin interface provides it: the interface's origin is builtin, the class is
//! in the compiler's table, the interface is one of that entry's exact providers, and its
//! type export is the class entry: it lists the class's witness method or, for a class with
//! no method to list (`Typeable`), its producer marks it a class and it lists none. A spelling
//! also bound to anything else (a local declaration, a source interface, an untrusted builtin
//! homonym, a different class) is excluded for good: a later import cannot restore it.
//! workflow: compiler-pipeline-chirho/builtin-class-identity-chirho

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::promotion_scope_chirho::{declared_type_names_chirho, spelling_chirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{
    BuiltinClassChirho, BuiltinClassScopeChirho, CompilerClassChirho,
};

use super::{IfaceOriginChirho, IfaceTypeChirho, ModuleIfaceChirho};
use crate::env_chirho::NamespaceChirho;
use crate::resolve_chirho::compute_imported_names_chirho;

/// What shows that a provider's type export is the class itself.
#[derive(Debug, Clone, Copy)]
enum ClassWitnessChirho {
    /// The export lists this method.
    MethodChirho(&'static str),
    /// A class with no method to list: its producer marks the export a class, and the
    /// export lists no method (#32090).
    MethodlessClassChirho,
}

impl ClassWitnessChirho {
    /// Whether `export_chirho` shows this witness.
    fn shown_by_chirho(self, export_chirho: &IfaceTypeChirho) -> bool {
        match self {
            Self::MethodChirho(method_chirho) => export_chirho
                .methods_chirho
                .iter()
                .any(|listed_chirho| listed_chirho == method_chirho),
            Self::MethodlessClassChirho => {
                export_chirho.is_class_chirho && export_chirho.methods_chirho.is_empty()
            }
        }
    }
}

/// One class the compiler defines, with the builtin interfaces trusted to provide it.
struct BuiltinClassEntryChirho {
    class_chirho: BuiltinClassChirho,
    /// What the provider's type export must show, so the export is the class itself.
    witness_chirho: ClassWitnessChirho,
    /// The builtin interfaces trusted to provide the class, exactly. A builtin homonym
    /// anywhere else stays unproven.
    providers_chirho: &'static [&'static str],
}

const fn entry_chirho(
    identity_chirho: CompilerClassChirho,
    module_chirho: &'static str,
    name_chirho: &'static str,
    witness_method_chirho: &'static str,
    providers_chirho: &'static [&'static str],
) -> BuiltinClassEntryChirho {
    BuiltinClassEntryChirho {
        class_chirho: BuiltinClassChirho {
            identity_chirho,
            module_chirho,
            name_chirho,
        },
        witness_chirho: ClassWitnessChirho::MethodChirho(witness_method_chirho),
        providers_chirho,
    }
}

/// The entry of a class with no method to list, which a class-marked export witnesses.
const fn methodless_entry_chirho(
    identity_chirho: CompilerClassChirho,
    module_chirho: &'static str,
    name_chirho: &'static str,
    providers_chirho: &'static [&'static str],
) -> BuiltinClassEntryChirho {
    BuiltinClassEntryChirho {
        class_chirho: BuiltinClassChirho {
            identity_chirho,
            module_chirho,
            name_chirho,
        },
        witness_chirho: ClassWitnessChirho::MethodlessClassChirho,
        providers_chirho,
    }
}

/// The classes naming proves. All but one are those the kind pass may judge (#31175,
/// #31176): monomorphic in GHC 9.14.1, with a real method every builtin entry lists. Read
/// (its builtin entry lists `read`, not a method) stays out. Real, RealFrac and RealFloat take
/// their witnesses from the class table's GHC member sets (#32156). Their typing seed
/// (class_chirho.rs) lists no method, so the witness proves which class a spelling names, never
/// that the class environment implements its methods. The one methodless entry is the
/// poly-kinded `Typeable` (#32090),
/// whose hidden kind binder the kind pass never judges, and whose providers are the three
/// builtin interfaces that export it, each measured importing it under GHC 9.14.1. The
/// providers are a draft scanned from the builtin sources, held to the supplier guard and to
/// GHC's export lists before any consumer reads them.
/// workflow: compiler-pipeline-chirho/builtin-class-identity-chirho
const BUILTIN_CLASSES_CHIRHO: &[BuiltinClassEntryChirho] = &[
    entry_chirho(
        CompilerClassChirho::AlternativeChirho,
        "Control.Applicative",
        "Alternative",
        "empty",
        &["Control.Applicative"],
    ),
    entry_chirho(
        CompilerClassChirho::MonadFixChirho,
        "Control.Monad.Fix",
        "MonadFix",
        "mfix",
        &["Control.Monad.Fix"],
    ),
    entry_chirho(
        CompilerClassChirho::MonadIoChirho,
        "Control.Monad.IO.Class",
        "MonadIO",
        "liftIO",
        &["Control.Monad.IO.Class", "Control.Monad.Trans"],
    ),
    entry_chirho(
        CompilerClassChirho::MonadTransChirho,
        "Control.Monad.Trans.Class",
        "MonadTrans",
        "lift",
        &["Control.Monad.Trans.Class", "Control.Monad.Trans"],
    ),
    entry_chirho(
        CompilerClassChirho::EqChirho,
        "GHC.Classes",
        "Eq",
        "==",
        &["Prelude", "Data.Eq", "GHC.Classes"],
    ),
    entry_chirho(
        CompilerClassChirho::OrdChirho,
        "GHC.Classes",
        "Ord",
        "compare",
        &["Prelude", "Data.Ord", "GHC.Classes"],
    ),
    entry_chirho(
        CompilerClassChirho::ShowChirho,
        "GHC.Show",
        "Show",
        "show",
        &["Prelude", "Text.Show", "GHC.Show"],
    ),
    entry_chirho(
        CompilerClassChirho::EnumChirho,
        "GHC.Enum",
        "Enum",
        "succ",
        &["Prelude", "GHC.Enum"],
    ),
    entry_chirho(
        CompilerClassChirho::BoundedChirho,
        "GHC.Enum",
        "Bounded",
        "minBound",
        &["Prelude", "GHC.Enum"],
    ),
    entry_chirho(
        CompilerClassChirho::NumChirho,
        "GHC.Num",
        "Num",
        "fromInteger",
        &["Prelude", "GHC.Num"],
    ),
    entry_chirho(
        CompilerClassChirho::IntegralChirho,
        "GHC.Real",
        "Integral",
        "toInteger",
        &["Prelude", "GHC.Real"],
    ),
    entry_chirho(
        CompilerClassChirho::FractionalChirho,
        "GHC.Real",
        "Fractional",
        "fromRational",
        &["Prelude", "GHC.Real"],
    ),
    entry_chirho(
        CompilerClassChirho::FloatingChirho,
        "GHC.Float",
        "Floating",
        "pi",
        &["Prelude", "GHC.Float"],
    ),
    entry_chirho(
        CompilerClassChirho::RealChirho,
        "GHC.Real",
        "Real",
        "toRational",
        &["Prelude", "GHC.Real"],
    ),
    entry_chirho(
        CompilerClassChirho::RealFracChirho,
        "GHC.Real",
        "RealFrac",
        "properFraction",
        &["Prelude", "GHC.Real"],
    ),
    entry_chirho(
        CompilerClassChirho::RealFloatChirho,
        "GHC.Float",
        "RealFloat",
        "floatRadix",
        &["Prelude", "GHC.Float"],
    ),
    entry_chirho(
        CompilerClassChirho::SemigroupChirho,
        "GHC.Base",
        "Semigroup",
        "<>",
        &["Prelude", "Data.Semigroup", "GHC.Base"],
    ),
    entry_chirho(
        CompilerClassChirho::MonoidChirho,
        "GHC.Base",
        "Monoid",
        "mempty",
        &["Prelude", "Data.Monoid", "GHC.Base"],
    ),
    entry_chirho(
        CompilerClassChirho::FunctorChirho,
        "GHC.Base",
        "Functor",
        "fmap",
        &["Prelude", "Data.Functor", "GHC.Base"],
    ),
    entry_chirho(
        CompilerClassChirho::ApplicativeChirho,
        "GHC.Base",
        "Applicative",
        "pure",
        &["Prelude", "Control.Applicative", "GHC.Base"],
    ),
    entry_chirho(
        CompilerClassChirho::MonadChirho,
        "GHC.Base",
        "Monad",
        ">>=",
        &["Prelude", "Control.Monad", "GHC.Base"],
    ),
    entry_chirho(
        CompilerClassChirho::MonadFailChirho,
        "Control.Monad.Fail",
        "MonadFail",
        "fail",
        &["Prelude", "Control.Monad", "Control.Monad.Fail"],
    ),
    entry_chirho(
        CompilerClassChirho::FoldableChirho,
        "Data.Foldable",
        "Foldable",
        "foldr",
        &["Prelude", "Data.Foldable"],
    ),
    entry_chirho(
        CompilerClassChirho::TraversableChirho,
        "Data.Traversable",
        "Traversable",
        "traverse",
        &["Prelude", "Data.Traversable"],
    ),
    methodless_entry_chirho(
        CompilerClassChirho::TypeableChirho,
        "Data.Typeable",
        "Typeable",
        &["Data.Typeable", "Type.Reflection", "Data.Data"],
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
        let Some(iface_chirho) =
            crate::iface_chirho::select_import_iface_chirho(ifaces_chirho, import_chirho)
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
/// identity: a builtin interface, one of the entry's providers, and a type export that shows
/// the entry's witness, so it is the class entry.
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
    entry_chirho
        .witness_chirho
        .shown_by_chirho(export_chirho)
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
