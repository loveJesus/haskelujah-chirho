// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Family reductions use the checked declaration's whole telescope locally
//! and through authoritative cold discovery, independently of result arrows.

use super::*;
use haskelujah_typing_chirho::infer_chirho::TypeFamilyIdChirho;

fn frontend_chirho(source_chirho: &str) -> crate::FrontendResultChirho {
    crate::run_frontend_chirho(
        source_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
        &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
        &std::collections::HashMap::new(),
    )
    .unwrap_or_else(|errors_chirho| {
        panic!(
            "{}",
            errors_chirho
                .diagnostics_chirho()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        )
    })
}

#[test]
fn local_closed_family_reduces_on_the_full_checked_kind_prefix_chirho() {
    let source_chirho = "{-# LANGUAGE StandaloneKindSignatures, ExplicitForAll, TypeFamilies, PolyKinds, KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\ntype FChirho :: forall k. k -> Type\ntype family FChirho a where\n  FChirho (a :: Type) = Int\n  FChirho (a :: Type -> Type) = Bool\nintChirho :: FChirho Int\nintChirho = 1\nboolChirho :: FChirho Maybe\nboolChirho = True\nmain = pure ()\n";
    let result_chirho = frontend_chirho(source_chirho);
    let key_chirho = TypeFamilyIdChirho::SourceChirho {
        module_chirho: "Main".to_owned(),
        name_chirho: "FChirho".to_owned(),
    }
    .key_chirho();
    let registry_chirho = &result_chirho.infer_result_chirho.type_families_chirho;
    assert_eq!(
        registry_chirho
            .definition_chirho(&key_chirho)
            .unwrap()
            .arity_chirho(),
        2
    );
    let header_chirho = registry_chirho
        .declaration_schemes_chirho()
        .get_chirho(&key_chirho)
        .unwrap();
    assert_eq!(header_chirho.visible_arity_chirho, 1);
    assert_eq!(header_chirho.reduction_arity_chirho, 2);
}

#[test]
fn a_nullary_family_returning_an_arrow_reduces_before_surplus_application_chirho() {
    frontend_chirho(
        "{-# LANGUAGE StandaloneKindSignatures, TypeFamilies #-}\nmodule Main where\nimport Data.Kind (Type)\ntype FChirho :: Type -> Type\ntype family FChirho where\n  FChirho = Maybe\nvalueChirho :: FChirho Int\nvalueChirho = Nothing\nmain = pure ()\n",
    );
}

#[test]
fn open_family_instances_transport_their_checked_hidden_prefix_chirho() {
    frontend_chirho(
        "{-# LANGUAGE DataKinds, TypeFamilies, PolyKinds, KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\nimport GHC.TypeLits (Nat)\ntype family FChirho (a :: k) :: Type\ntype instance FChirho (3 :: Nat) = Int\ntype instance FChirho (a :: Type -> Type) = Bool\nintChirho :: FChirho 3\nintChirho = 1\nboolChirho :: FChirho Maybe\nboolChirho = True\nmain = pure ()\n",
    );
}

#[test]
fn cold_source_registration_publishes_the_same_complete_telescope_chirho() {
    let source_chirho = "{-# LANGUAGE TypeFamilies, PolyKinds #-}\nmodule CheckedChirho where\nimport OriginChirho\nintChirho :: FChirho Int\nintChirho = 1\nboolChirho :: FChirho Maybe\nboolChirho = True\n";
    let (registry_chirho, ifaces_chirho) = discovery_chirho(
        source_chirho,
        &[
            "{-# LANGUAGE StandaloneKindSignatures, ExplicitForAll, TypeFamilies, PolyKinds, KindSignatures #-}\nmodule OriginChirho (FChirho) where\nimport Data.Kind (Type)\ntype FChirho :: forall k. k -> Type\ntype family FChirho a where\n  FChirho (a :: Type) = Int\n  FChirho (a :: Type -> Type) = Bool\nignoredBodyChirho = True + []\n",
        ],
    );
    checks_chirho(source_chirho, &registry_chirho, &ifaces_chirho);
    let key_chirho = TypeFamilyIdChirho::SourceChirho {
        module_chirho: "OriginChirho".to_owned(),
        name_chirho: "FChirho".to_owned(),
    }
    .key_chirho();
    assert_eq!(
        registry_chirho
            .definition_chirho(&key_chirho)
            .unwrap()
            .arity_chirho(),
        2
    );
    assert_eq!(
        registry_chirho
            .declaration_schemes_chirho()
            .get_chirho(&key_chirho)
            .unwrap()
            .reduction_arity_chirho,
        2
    );
}

#[test]
fn cold_declaration_dependencies_follow_the_exact_producer_chirho() {
    let source_chirho = "module CheckedChirho where\nimport AChirho\nvalueChirho :: AliasChirho\nvalueChirho = Just 1\n";
    let (registry_chirho, ifaces_chirho) = discovery_chirho(
        source_chirho,
        &[
            "module AChirho (AliasChirho) where\nimport ZChirho\ntype AliasChirho = WrapChirho Int\n",
            "module ZChirho (WrapChirho) where\ntype WrapChirho a = Maybe a\n",
        ],
    );
    checks_chirho(source_chirho, &registry_chirho, &ifaces_chirho);
}

#[test]
fn checked_synonym_reexports_keep_the_defining_supplier_across_homonyms_chirho() {
    let source_chirho = "{-# LANGUAGE PolyKinds #-}\nmodule CheckedChirho where\nimport qualified SecondChirho as SChirho\nimport qualified HomonymChirho as HChirho\nfirstChirho :: SChirho.IdentityChirho Int\nfirstChirho = 1\nsecondChirho :: HChirho.IdentityChirho Int\nsecondChirho = True\n";
    let (registry_chirho, ifaces_chirho) = discovery_chirho(
        source_chirho,
        &[
            "{-# LANGUAGE PolyKinds, KindSignatures #-}\nmodule OriginChirho (IdentityChirho) where\ntype IdentityChirho (a :: k) = a\n",
            "module FirstChirho (module OriginChirho) where\nimport OriginChirho\n",
            "module SecondChirho (module FirstChirho) where\nimport FirstChirho\n",
            "module HomonymChirho (IdentityChirho) where\ntype IdentityChirho a = Bool\n",
        ],
    );
    checks_chirho(source_chirho, &registry_chirho, &ifaces_chirho);
}
