// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Which builtin stock class a written deriving class names (row 694, #31771, #31786).
//!
//! The stock route runs a class's own generator, so it must know the class itself, not its
//! spelling: `deriving stock (P.Eq)` under `import qualified Prelude as P` names the builtin
//! `Eq`, while `deriving stock (Eq)` beside a local `class Eq` names the local class, which no
//! generator derives. The builtin class scope naming proved for the module (K01) is the proof:
//! a proven spelling names its builtin class, whatever qualifier it is written with.
//!
//! An explicit `stock` takes a generator only on that proof. K01 proves a spelling or says
//! nothing: it excludes a source module's re-export of the Prelude's class as well as a local
//! declaration, so an exclusion is no disproof and no authority either. Declaration validity
//! refuses the module's own class (GHC-00158) and names every other unproven spelling as
//! unsupported: an imported or re-exported source class, and Read and Generic, which K01's
//! table does not hold. An unwritten strategy keeps the legacy route, which takes a written name
//! K01 cannot judge for that stock class. A generated instance names its class with the written
//! spelling, so naming resolves it to the class proved here.
//!
//! K01's builtin `Typeable` is the one class no strategy derives: every type already has it,
//! and GHC ignores a request for it (`ignores_derived_class_chirho`, #32106).
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use std::collections::HashSet;

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, DerivedClassChirho, TyVarChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{
    BuiltinClassScopeChirho, CompilerClassChirho, NameChirho,
};
use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;

/// The classes GHC derives with the stock strategy: its refusal names any other class "not a
/// stock derivable class (Eq, Show, etc.)" (GHC-00158 on data, GHC-82023 on a newtype). Each
/// takes one parameter, the derived type.
pub(crate) const STOCK_DERIVABLE_CLASSES_CHIRHO: &[&str] = &[
    "Eq",
    "Ord",
    "Enum",
    "Bounded",
    "Show",
    "Read",
    "Ix",
    "Functor",
    "Foldable",
    "Traversable",
    "Generic",
    "Generic1",
    "Data",
    "Lift",
    "Typeable",
];

/// What decides, in one module, which class a written deriving class names: the builtin
/// classes naming proved (K01) and the type-namespace names the module declares itself.
pub(crate) struct StockIdentityChirho<'a> {
    builtin_classes_chirho: &'a BuiltinClassScopeChirho,
    module_name_chirho: String,
    /// Each local class with its visible parameter count.
    local_classes_chirho: Vec<(String, usize)>,
    /// Every name the module declares in the type namespace.
    local_types_chirho: HashSet<String>,
}

impl<'a> StockIdentityChirho<'a> {
    /// The judgment for `module_chirho`, with the builtin classes naming proved for it.
    pub(crate) fn of_module_chirho(
        module_chirho: &ModuleChirho,
        builtin_classes_chirho: &'a BuiltinClassScopeChirho,
    ) -> Self {
        let mut local_classes_chirho = Vec::new();
        let mut local_types_chirho = HashSet::new();
        for decl_chirho in &module_chirho.decls_chirho {
            match decl_chirho {
                DeclChirho::ClassDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    ..
                } => {
                    local_classes_chirho.push((
                        name_chirho.text_chirho().to_string(),
                        TyVarChirho::visible_binders_chirho(type_vars_chirho).len(),
                    ));
                    local_types_chirho.insert(name_chirho.text_chirho().to_string());
                }
                DeclChirho::DataDeclChirho { name_chirho, .. }
                | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
                | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
                | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. } => {
                    local_types_chirho.insert(name_chirho.text_chirho().to_string());
                }
                _ => {}
            }
        }
        Self {
            builtin_classes_chirho,
            module_name_chirho: module_chirho.name_chirho.full_name_chirho(),
            local_classes_chirho,
            local_types_chirho,
        }
    }

    /// The builtin classes naming proved for the module.
    pub(crate) fn builtin_classes_chirho(&self) -> &'a BuiltinClassScopeChirho {
        self.builtin_classes_chirho
    }

    /// Whether `class_chirho` is written as one of this module's own type-namespace names:
    /// unqualified, or qualified with the module's own name.
    pub(crate) fn names_local_chirho(&self, class_chirho: &NameChirho) -> bool {
        class_chirho
            .qualifier_chirho()
            .is_none_or(|qualifier_chirho| qualifier_chirho == self.module_name_chirho)
            && self.local_types_chirho.contains(class_chirho.text_chirho())
    }

    /// Whether the module declares a type-namespace name of this bare text, which then holds
    /// that name in the checker's environments (row 694, #31891).
    pub(crate) fn declares_type_name_chirho(&self, text_chirho: &str) -> bool {
        self.local_types_chirho.contains(text_chirho)
    }

    /// Whether K01 proved that `class_chirho`'s full spelling names a builtin class, stock
    /// derivable or not.
    pub(crate) fn proves_builtin_chirho(&self, class_chirho: &NameChirho) -> bool {
        self.builtin_classes_chirho
            .class_chirho(&class_chirho.full_name_chirho())
            .is_some()
    }

    /// Whether GHC ignores a request to derive `class_chirho`: K01 proved it names the
    /// builtin `Typeable`, which every type already has (the class environment's universal
    /// instance). GHC 9.14.1 accepts such a clause, unwritten or with an explicit `stock` or
    /// `newtype` strategy, and a standalone `deriving newtype instance`; a stock `Show`
    /// beside each keeps printing the constructor (typeable-route r2, r6, r7). So the request
    /// is no instance at all: nothing is derived, no representation dictionary is requested,
    /// and it duplicates nothing (#32106). A source class named `Typeable` is never proved
    /// here and keeps its own route.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(crate) fn ignores_derived_class_chirho(&self, class_chirho: &NameChirho) -> bool {
        self.builtin_classes_chirho
            .class_chirho(&class_chirho.full_name_chirho())
            .is_some_and(|builtin_chirho| {
                builtin_chirho.identity_chirho == CompilerClassChirho::TypeableChirho
            })
    }

    /// The stock derivable builtin class K01 proved `class_chirho` names, by its own name
    /// (`Eq` for a written `P.Eq`): the only authority an explicit `stock` accepts.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(crate) fn proven_stock_class_chirho(
        &self,
        class_chirho: &NameChirho,
    ) -> Option<&'static str> {
        let builtin_chirho = self
            .builtin_classes_chirho
            .class_chirho(&class_chirho.full_name_chirho())?;
        STOCK_DERIVABLE_CLASSES_CHIRHO
            .contains(&builtin_chirho.name_chirho)
            .then_some(builtin_chirho.name_chirho)
    }

    /// The stock class the legacy route takes for an unwritten strategy: K01's proof when
    /// naming gave one; none when the spelling names one of this module's own declarations or
    /// a proven builtin class GHC does not derive with the stock strategy; otherwise the
    /// written name, which K01 cannot judge.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(crate) fn legacy_stock_class_chirho(
        &self,
        class_chirho: &NameChirho,
    ) -> Option<&'static str> {
        if self.proves_builtin_chirho(class_chirho) {
            return self.proven_stock_class_chirho(class_chirho);
        }
        if self.names_local_chirho(class_chirho) {
            return None;
        }
        STOCK_DERIVABLE_CLASSES_CHIRHO
            .iter()
            .copied()
            .find(|stock_chirho| *stock_chirho == class_chirho.text_chirho())
    }

    /// The stock class whose generator may derive `derived_chirho`: an explicit `stock` needs
    /// K01's proof, an unwritten strategy keeps the legacy route, and `anyclass` and `newtype`
    /// never take a generator.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(crate) fn stock_class_for_chirho(
        &self,
        derived_chirho: &DerivedClassChirho,
    ) -> Option<&'static str> {
        match &derived_chirho.strategy_chirho {
            None => self.legacy_stock_class_chirho(&derived_chirho.class_chirho),
            Some(strategy_chirho) if strategy_chirho.is_stock_chirho() => {
                self.proven_stock_class_chirho(&derived_chirho.class_chirho)
            }
            Some(_) => None,
        }
    }

    /// How many parameters `derived_chirho`'s class takes, where the module can tell: a
    /// builtin class's visible parameters from K01's schema (a hidden kind binder is never
    /// written), a local class from its declaration, and, on the legacy route of an unwritten
    /// strategy, a stock class one. `None` for a class only an interface knows.
    pub(crate) fn parameter_count_chirho(
        &self,
        derived_chirho: &DerivedClassChirho,
    ) -> Option<usize> {
        let class_chirho = &derived_chirho.class_chirho;
        if let Some(builtin_chirho) = self
            .builtin_classes_chirho
            .class_chirho(&class_chirho.full_name_chirho())
        {
            return Some(
                builtin_chirho
                    .schema_chirho()
                    .visible_parameter_count_chirho(),
            );
        }
        if self.names_local_chirho(class_chirho) {
            return self
                .local_classes_chirho
                .iter()
                .find(|(name_chirho, _)| name_chirho == class_chirho.text_chirho())
                .map(|(_, count_chirho)| *count_chirho);
        }
        if derived_chirho.strategy_chirho.is_none() {
            return self.legacy_stock_class_chirho(class_chirho).map(|_| 1);
        }
        None
    }

    /// Whether `derived_chirho` asks for `stock_class_chirho`'s own stock code, exactly as the
    /// deriving pass routes it: it takes no class argument, and its strategy's stock judgment
    /// names that stock class.
    pub(crate) fn names_stock_class_chirho(
        &self,
        derived_chirho: &DerivedClassChirho,
        stock_class_chirho: &str,
    ) -> bool {
        !derived_chirho.has_class_arguments_chirho()
            && self.stock_class_for_chirho(derived_chirho) == Some(stock_class_chirho)
    }
}

/// Name the class of every instance a stock generator just wrote with the written spelling:
/// the generators name `stock_class_chirho` unqualified, which a qualified-only import or a
/// local class of that name would misread. The head's class and each context constraint on the
/// same class take `written_chirho`.
/// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
pub(crate) fn spell_generated_class_chirho(
    generated_chirho: &mut [DeclChirho],
    stock_class_chirho: &str,
    written_chirho: &NameChirho,
) {
    let generated_name_chirho = |name_chirho: &NameChirho| {
        name_chirho.qualifier_chirho().is_none() && name_chirho.text_chirho() == stock_class_chirho
    };
    for decl_chirho in generated_chirho {
        let DeclChirho::InstanceDeclChirho {
            class_chirho,
            context_chirho,
            ..
        } = decl_chirho
        else {
            continue;
        };
        if generated_name_chirho(class_chirho) {
            *class_chirho = written_chirho.clone();
        }
        for constraint_chirho in context_chirho {
            if let ConstraintChirho::ClassChirho { class_chirho, .. } = constraint_chirho
                && generated_name_chirho(class_chirho)
            {
                *class_chirho = written_chirho.clone();
            }
        }
    }
}

#[cfg(test)]
#[path = "stock_identity_tests_chirho.rs"]
mod tests_chirho;
