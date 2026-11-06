// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The builtin interface tables outside `GHC.*`: each test pins one table entry it relies on.

use crate::env_chirho::NamespaceChirho;
use crate::iface_chirho::tests_chirho::mk_name_chirho;
use crate::iface_chirho::*;
use crate::resolve_chirho::compute_imported_names_chirho;

#[test]
fn builtin_dedup_data_map_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let data_map_chirho: Vec<_> = ifaces_chirho
        .iter()
        .filter(|i_chirho| i_chirho.name_chirho == "Data.Map")
        .collect();
    assert_eq!(
        data_map_chirho.len(),
        1,
        "Data.Map should be deduplicated to a single entry"
    );
    let dm_chirho = &data_map_chirho[0];
    // Should have standard Haskell API names from the second definition
    assert!(
        dm_chirho
            .exports_chirho
            .values_chirho
            .contains_key("singleton"),
        "Data.Map should export 'singleton' after dedup merge"
    );
    assert!(
        dm_chirho.exports_chirho.values_chirho.contains_key("empty"),
        "Data.Map should export 'empty' after dedup merge"
    );
    // Should also have internal names from the first definition
    assert!(
        dm_chirho
            .exports_chirho
            .values_chirho
            .contains_key("mapInsert"),
        "Data.Map should still export 'mapInsert' from first definition"
    );
}

#[test]
fn builtin_dedup_foreign_storable_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let fs_chirho: Vec<_> = ifaces_chirho
        .iter()
        .filter(|i_chirho| i_chirho.name_chirho == "Foreign.Storable")
        .collect();
    assert_eq!(
        fs_chirho.len(),
        1,
        "Foreign.Storable should be deduplicated to a single entry"
    );
    let s_chirho = &fs_chirho[0];
    assert!(
        s_chirho.exports_chirho.values_chirho.contains_key("sizeOf"),
        "Foreign.Storable should export 'sizeOf'"
    );
    assert!(
        s_chirho.exports_chirho.values_chirho.contains_key("peek"),
        "Foreign.Storable should export 'peek'"
    );
    assert!(
        s_chirho
            .exports_chirho
            .values_chirho
            .contains_key("peekElemOff"),
        "Foreign.Storable should export 'peekElemOff'"
    );
}

#[test]
fn builtin_foreign_reexports_peek_elem_off_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let foreign_chirho = ifaces_chirho
        .iter()
        .rev()
        .find(|iface_chirho| iface_chirho.name_chirho == "Foreign")
        .expect("Foreign builtin iface should exist");
    assert!(
        foreign_chirho
            .exports_chirho
            .values_chirho
            .contains_key("peekElemOff"),
        "Foreign should export 'peekElemOff'"
    );
}

#[test]
fn builtin_control_monad_class_methods_exported_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let control_monad_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Control.Monad")
        .expect("Control.Monad builtin iface should exist");
    let monad_chirho = control_monad_chirho
        .exports_chirho
        .types_chirho
        .get("Monad")
        .expect("Control.Monad should export Monad");
    assert!(monad_chirho.methods_chirho.contains(&"return".to_string()));
    assert!(monad_chirho.methods_chirho.contains(&">>=".to_string()));
    assert!(monad_chirho.methods_chirho.contains(&">>".to_string()));
    assert!(
        control_monad_chirho
            .exports_chirho
            .values_chirho
            .contains_key("return")
    );
    assert!(
        control_monad_chirho
            .exports_chirho
            .values_chirho
            .contains_key(">>=")
    );
}

#[test]
fn builtin_data_foldable_class_methods_exported_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let foldable_mod_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Foldable")
        .expect("Data.Foldable builtin iface should exist");
    let foldable_chirho = foldable_mod_chirho
        .exports_chirho
        .types_chirho
        .get("Foldable")
        .expect("Data.Foldable should export Foldable");
    for method_chirho in [
        "foldr", "foldl", "foldMap", "length", "null", "elem", "sum", "product", "maximum",
        "minimum", "toList",
    ] {
        assert!(
            foldable_chirho
                .methods_chirho
                .contains(&method_chirho.to_string()),
            "Foldable should expose {method_chirho} via (..)"
        );
        assert!(
            foldable_mod_chirho
                .exports_chirho
                .values_chirho
                .contains_key(method_chirho),
            "Data.Foldable should export value {method_chirho}"
        );
    }
}

#[test]
fn builtin_prelude_retains_legacy_helpers_without_an_alternative_class_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let prelude_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Prelude")
        .expect("Prelude builtin iface should exist");
    // GHC-76037: Prelude does not supply this class; its fake export poisons a real import.
    assert!(
        !prelude_chirho
            .exports_chirho
            .types_chirho
            .contains_key("Alternative"),
        "Prelude must not supply the Alternative class"
    );
    assert!(
        prelude_chirho
            .exports_chirho
            .types_chirho
            .contains_key("Const"),
        "Prelude should export Const"
    );
    for name_chirho in [
        "empty", "<|>", "some", "many", "optional", "liftA2", "bimap", "Const", "getConst", "!!",
    ] {
        assert!(
            prelude_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "Prelude should export {name_chirho}"
        );
    }
}

#[test]
fn builtin_control_applicative_const_dotdot_imports_getconst_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let control_applicative_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Control.Applicative")
        .expect("Control.Applicative builtin iface should exist");
    let imported_names_chirho = compute_imported_names_chirho(
        &control_applicative_chirho.exports_chirho,
        &Some(ImportSpecChirho {
            hiding_chirho: false,
            items_chirho: vec![ImportItemChirho::TyConChirho {
                name_chirho: mk_name_chirho("Const"),
                members_chirho: ExportMembersChirho::AllChirho,
            }],
        }),
    );
    assert!(
        imported_names_chirho
            .iter()
            .any(|(name_chirho, namespace_chirho, _span_chirho)| {
                name_chirho == "getConst" && *namespace_chirho == NamespaceChirho::ValueChirho
            }),
        "Const(..) should import getConst"
    );
}

#[test]
fn builtin_transformer_accessors_import_with_dotdot_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let backwards_mod_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Control.Applicative.Backwards")
        .expect("Control.Applicative.Backwards builtin iface should exist");
    let backwards_imported_names_chirho = compute_imported_names_chirho(
        &backwards_mod_chirho.exports_chirho,
        &Some(ImportSpecChirho {
            hiding_chirho: false,
            items_chirho: vec![ImportItemChirho::TyConChirho {
                name_chirho: mk_name_chirho("Backwards"),
                members_chirho: ExportMembersChirho::AllChirho,
            }],
        }),
    );
    assert!(
        backwards_imported_names_chirho.iter().any(
            |(name_chirho, namespace_chirho, _span_chirho)| {
                name_chirho == "forwards" && *namespace_chirho == NamespaceChirho::ValueChirho
            }
        ),
        "Backwards(..) should import forwards"
    );

    let reverse_mod_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Functor.Reverse")
        .expect("Data.Functor.Reverse builtin iface should exist");
    let reverse_imported_names_chirho = compute_imported_names_chirho(
        &reverse_mod_chirho.exports_chirho,
        &Some(ImportSpecChirho {
            hiding_chirho: false,
            items_chirho: vec![ImportItemChirho::TyConChirho {
                name_chirho: mk_name_chirho("Reverse"),
                members_chirho: ExportMembersChirho::AllChirho,
            }],
        }),
    );
    assert!(
        reverse_imported_names_chirho.iter().any(
            |(name_chirho, namespace_chirho, _span_chirho)| {
                name_chirho == "getReverse" && *namespace_chirho == NamespaceChirho::ValueChirho
            }
        ),
        "Reverse(..) should import getReverse"
    );
}

#[test]
fn builtin_collection_and_arrow_helpers_exported_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();

    let data_foldable_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Foldable")
        .expect("Data.Foldable builtin iface should exist");
    for name_chirho in ["foldrM", "foldlM"] {
        assert!(
            data_foldable_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "Data.Foldable should export {name_chirho}"
        );
    }

    let data_sequence_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Sequence")
        .expect("Data.Sequence builtin iface should exist");
    assert!(
        data_sequence_chirho
            .exports_chirho
            .values_chirho
            .contains_key("foldMapWithIndex"),
        "Data.Sequence should export foldMapWithIndex"
    );

    let data_intmap_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.IntMap")
        .expect("Data.IntMap builtin iface should exist");
    for name_chirho in [
        "foldMapWithKey",
        "traverseWithKey",
        "fromAscListWith",
        "alterF",
        "updateMin",
        "updateMax",
        "minViewWithKey",
        "maxViewWithKey",
    ] {
        assert!(
            data_intmap_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "Data.IntMap should export {name_chirho}"
        );
    }

    let data_map_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Map")
        .expect("Data.Map builtin iface should exist");
    for name_chirho in [
        "foldMapWithKey",
        "traverseWithKey",
        "fromAscListWith",
        "alterF",
        "updateMin",
        "updateMax",
        "isSubmapOfBy",
        "isProperSubmapOfBy",
        "mapKeysWith",
        "valid",
    ] {
        assert!(
            data_map_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "Data.Map should export {name_chirho}"
        );
    }

    let ghc_real_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Real")
        .expect("GHC.Real builtin iface should exist");
    assert!(
        ghc_real_chirho
            .exports_chirho
            .types_chirho
            .get("Ratio")
            .is_some_and(|ratio_chirho| {
                ratio_chirho.constructors_chirho.contains(&":%".to_string())
            }),
        "GHC.Real should export Ratio with :% constructor"
    );
    assert!(
        ghc_real_chirho
            .exports_chirho
            .values_chirho
            .contains_key(":%"),
        "GHC.Real should export :% as a value constructor"
    );

    let writer_cps_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Control.Monad.Trans.Writer.CPS")
        .expect("Control.Monad.Trans.Writer.CPS builtin iface should exist");
    assert!(
        writer_cps_chirho
            .exports_chirho
            .values_chirho
            .contains_key("writerT"),
        "Control.Monad.Trans.Writer.CPS should export writerT"
    );

    let th_syntax_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Language.Haskell.TH.Syntax")
        .expect("Language.Haskell.TH.Syntax builtin iface should exist");
    assert!(
        th_syntax_chirho
            .exports_chirho
            .values_chirho
            .contains_key("mkNameG_tc"),
        "Language.Haskell.TH.Syntax should export mkNameG_tc"
    );

    let reader_trans_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Control.Monad.Trans.Reader")
        .expect("Control.Monad.Trans.Reader builtin iface should exist");
    assert!(
        reader_trans_chirho
            .exports_chirho
            .values_chirho
            .contains_key("liftCatch"),
        "Control.Monad.Trans.Reader should export liftCatch"
    );

    let control_arrow_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Control.Arrow")
        .expect("Control.Arrow builtin iface should exist");
    let arrow_zero_imported_names_chirho = compute_imported_names_chirho(
        &control_arrow_chirho.exports_chirho,
        &Some(ImportSpecChirho {
            hiding_chirho: false,
            items_chirho: vec![ImportItemChirho::TyConChirho {
                name_chirho: mk_name_chirho("ArrowZero"),
                members_chirho: ExportMembersChirho::AllChirho,
            }],
        }),
    );
    assert!(
        arrow_zero_imported_names_chirho.iter().any(
            |(name_chirho, namespace_chirho, _span_chirho)| {
                name_chirho == "zeroArrow" && *namespace_chirho == NamespaceChirho::ValueChirho
            }
        ),
        "ArrowZero(..) should import zeroArrow"
    );
    let arrow_plus_imported_names_chirho = compute_imported_names_chirho(
        &control_arrow_chirho.exports_chirho,
        &Some(ImportSpecChirho {
            hiding_chirho: false,
            items_chirho: vec![ImportItemChirho::TyConChirho {
                name_chirho: mk_name_chirho("ArrowPlus"),
                members_chirho: ExportMembersChirho::AllChirho,
            }],
        }),
    );
    assert!(
        arrow_plus_imported_names_chirho.iter().any(
            |(name_chirho, namespace_chirho, _span_chirho)| {
                name_chirho == "<+>" && *namespace_chirho == NamespaceChirho::ValueChirho
            }
        ),
        "ArrowPlus(..) should import <+>"
    );
}

#[test]
fn builtin_bytestring_internal_and_system_io_unsafe_exports_effect_helpers_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let bytestring_internal_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.ByteString.Internal")
        .expect("Data.ByteString.Internal builtin iface should exist");
    assert!(
        bytestring_internal_chirho
            .exports_chirho
            .values_chirho
            .contains_key("accursedUnutterablePerformIO")
            && bytestring_internal_chirho
                .exports_chirho
                .values_chirho
                .contains_key("memset")
            && bytestring_internal_chirho
                .exports_chirho
                .values_chirho
                .contains_key("memcpy")
            && bytestring_internal_chirho
                .exports_chirho
                .values_chirho
                .contains_key("plusForeignPtr"),
        "Data.ByteString.Internal should export accursedUnutterablePerformIO, memset, memcpy, and plusForeignPtr"
    );

    let bytestring_unsafe_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.ByteString.Unsafe")
        .expect("Data.ByteString.Unsafe builtin iface should exist");
    for name_chirho in ["unsafeHead", "unsafeTail", "unsafeLast", "unsafeInit"] {
        assert!(
            bytestring_unsafe_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "Data.ByteString.Unsafe should export {name_chirho}"
        );
    }

    let system_io_unsafe_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "System.IO.Unsafe")
        .expect("System.IO.Unsafe builtin iface should exist");
    for name_chirho in ["unsafePerformIO", "inlinePerformIO"] {
        assert!(
            system_io_unsafe_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "System.IO.Unsafe should export {name_chirho}"
        );
    }
}

#[test]
fn builtin_foreign_ptr_exports_unsafe_with_foreign_ptr_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_foreign_ptr_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.ForeignPtr")
        .expect("GHC.ForeignPtr builtin iface should exist");
    assert!(
        ghc_foreign_ptr_chirho
            .exports_chirho
            .values_chirho
            .contains_key("unsafeWithForeignPtr")
            && ghc_foreign_ptr_chirho
                .exports_chirho
                .values_chirho
                .contains_key("memset")
            && ghc_foreign_ptr_chirho
                .exports_chirho
                .values_chirho
                .contains_key("mallocPlainForeignPtrBytes")
            && ghc_foreign_ptr_chirho
                .exports_chirho
                .values_chirho
                .contains_key("plusForeignPtr"),
        "GHC.ForeignPtr should export unsafeWithForeignPtr, memset, mallocPlainForeignPtrBytes, and plusForeignPtr"
    );
}

#[test]
fn builtin_control_monad_st_lazy_exports_conversion_helpers_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let control_monad_st_lazy_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Control.Monad.ST.Lazy")
        .expect("Control.Monad.ST.Lazy builtin iface should exist");
    assert!(
        control_monad_st_lazy_chirho
            .exports_chirho
            .values_chirho
            .contains_key("strictToLazyST"),
        "Control.Monad.ST.Lazy should export strictToLazyST"
    );
    assert!(
        control_monad_st_lazy_chirho
            .exports_chirho
            .values_chirho
            .contains_key("lazyToStrictST"),
        "Control.Monad.ST.Lazy should export lazyToStrictST"
    );
}

#[test]
fn builtin_control_monad_combinators_exports_core_reexports_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let control_monad_combinators_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Control.Monad.Combinators")
        .expect("Control.Monad.Combinators builtin iface should exist");
    for name_chirho in ["many", "some", "optional"] {
        assert!(
            control_monad_combinators_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "Control.Monad.Combinators should export {name_chirho}"
        );
    }
}

#[test]
fn builtin_test_tasty_exports_test_tree_helpers_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let test_tasty_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Test.Tasty")
        .expect("Test.Tasty builtin iface should exist");
    for name_chirho in [
        "defaultMain",
        "defaultMainWithIngredients",
        "testGroup",
        "includingOptions",
    ] {
        assert!(
            test_tasty_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "Test.Tasty should export {name_chirho}"
        );
    }
    assert!(
        test_tasty_chirho
            .exports_chirho
            .types_chirho
            .contains_key("TestTree"),
        "Test.Tasty should export TestTree"
    );
}

#[test]
fn builtin_test_tasty_options_exports_is_option_class_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let test_tasty_options_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Test.Tasty.Options")
        .expect("Test.Tasty.Options builtin iface should exist");
    for name_chirho in [
        "safeRead",
        "lookupOption",
        "Option",
        "defaultValue",
        "parseValue",
        "optionName",
        "optionHelp",
    ] {
        assert!(
            test_tasty_options_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "Test.Tasty.Options should export {name_chirho}"
        );
    }
    assert!(
        test_tasty_options_chirho
            .exports_chirho
            .types_chirho
            .get("IsOption")
            .is_some_and(|is_option_chirho| is_option_chirho
                .methods_chirho
                .contains(&"parseValue".to_string())),
        "Test.Tasty.Options should export IsOption with class methods"
    );
}

#[test]
fn builtin_text_read_exports_parens_and_readprec_method_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let text_read_chirho = ifaces_chirho
        .iter()
        .rev()
        .find(|iface_chirho| iface_chirho.name_chirho == "Text.Read")
        .expect("Text.Read builtin iface should exist");
    assert!(
        text_read_chirho
            .exports_chirho
            .values_chirho
            .contains_key("parens"),
        "Text.Read should export parens"
    );
    for name_chirho in ["step", "reset"] {
        assert!(
            text_read_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "Text.Read should export {name_chirho}"
        );
    }
    assert!(
        text_read_chirho
            .exports_chirho
            .types_chirho
            .get("Read")
            .is_some_and(|read_class_chirho| read_class_chirho
                .methods_chirho
                .contains(&"readPrec".to_string())),
        "Text.Read.Read should expose readPrec as a class method"
    );
}

#[test]
fn builtin_readp_exports_triple_plus_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let readp_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Text.ParserCombinators.ReadP")
        .expect("Text.ParserCombinators.ReadP builtin iface should exist");
    assert!(
        readp_chirho
            .exports_chirho
            .values_chirho
            .contains_key("+++"),
        "Text.ParserCombinators.ReadP should export +++"
    );
}

#[test]
fn builtin_system_mem_weak_reexports_ghc_weak_surface_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let system_mem_weak_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "System.Mem.Weak")
        .expect("System.Mem.Weak builtin iface should exist");
    assert!(
        system_mem_weak_chirho
            .exports_chirho
            .types_chirho
            .contains_key("Weak"),
        "System.Mem.Weak should export Weak"
    );
    for name_chirho in ["mkWeak", "deRefWeak", "finalize", "mkWeakIORef"] {
        assert!(
            system_mem_weak_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "System.Mem.Weak should export {name_chirho}"
        );
    }
}

#[test]
fn builtin_data_text_internal_exports_text_constructor_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let text_internal_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Text.Internal")
        .expect("Data.Text.Internal builtin iface should exist");
    assert!(
        text_internal_chirho
            .exports_chirho
            .types_chirho
            .get("Text")
            .is_some_and(|text_ty_chirho| text_ty_chirho
                .constructors_chirho
                .contains(&"Text".to_string())),
        "Data.Text.Internal should export Text(..)"
    );
}

#[test]
fn builtin_data_text_lazy_exports_foldr_chunks_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let text_lazy_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Text.Lazy")
        .expect("Data.Text.Lazy builtin iface should exist");
    assert!(
        text_lazy_chirho
            .exports_chirho
            .values_chirho
            .contains_key("foldrChunks"),
        "Data.Text.Lazy should export foldrChunks"
    );
}

#[test]
fn builtin_warp_support_modules_export_expected_names_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();

    let case_insensitive_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.CaseInsensitive")
        .expect("Data.CaseInsensitive builtin iface should exist");
    assert!(
        case_insensitive_chirho
            .exports_chirho
            .values_chirho
            .contains_key("original")
            && case_insensitive_chirho
                .exports_chirho
                .values_chirho
                .contains_key("foldedCase")
            && case_insensitive_chirho
                .exports_chirho
                .types_chirho
                .contains_key("CI"),
        "Data.CaseInsensitive should export CI/original/foldedCase"
    );

    let ghc_storable_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Storable")
        .expect("GHC.Storable builtin iface should exist");
    assert!(
        ghc_storable_chirho
            .exports_chirho
            .values_chirho
            .contains_key("writeWord8OffPtr"),
        "GHC.Storable should export writeWord8OffPtr"
    );

    let buffer_pool_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Network.Socket.BufferPool")
        .expect("Network.Socket.BufferPool builtin iface should exist");
    assert!(
        buffer_pool_chirho
            .exports_chirho
            .values_chirho
            .contains_key("copy")
            && buffer_pool_chirho
                .exports_chirho
                .values_chirho
                .contains_key("newBufferPool")
            && buffer_pool_chirho
                .exports_chirho
                .types_chirho
                .contains_key("Buffer"),
        "Network.Socket.BufferPool should export warp support names"
    );
}

#[test]
fn builtin_data_functor_compose_exports_getcompose_via_type_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let compose_chirho = ifaces_chirho
        .iter()
        .rev()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Functor.Compose")
        .expect("Data.Functor.Compose builtin iface should exist");
    assert!(
        compose_chirho
            .exports_chirho
            .types_chirho
            .get("Compose")
            .is_some_and(|compose_ty_chirho| compose_ty_chirho
                .methods_chirho
                .contains(&"getCompose".to_string())),
        "Data.Functor.Compose should carry getCompose on Compose(..)"
    );
}

#[test]
fn builtin_word_modules_export_byteswap_helpers_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    for module_name_chirho in ["Data.Word", "GHC.Word"] {
        let word_iface_chirho = ifaces_chirho
            .iter()
            .rev()
            .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
            .unwrap_or_else(|| panic!("{module_name_chirho} builtin iface should exist"));
        for value_name_chirho in ["byteSwap16", "byteSwap32", "byteSwap64"] {
            assert!(
                word_iface_chirho
                    .exports_chirho
                    .values_chirho
                    .contains_key(value_name_chirho),
                "{module_name_chirho} should export {value_name_chirho}"
            );
        }
    }
}

#[test]
fn builtin_with_index_instances_modules_exist_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    for module_name_chirho in &[
        "Data.Functor.WithIndex.Instances",
        "Data.Foldable.WithIndex.Instances",
        "Data.Traversable.WithIndex.Instances",
    ] {
        assert!(
            ifaces_chirho
                .iter()
                .any(|iface_chirho| iface_chirho.name_chirho == *module_name_chirho),
            "{module_name_chirho} builtin iface should exist"
        );
    }
}

#[test]
fn builtin_semigroup_traversable_exports_traverse1_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let semigroup_traversable_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Semigroup.Traversable")
        .expect("Data.Semigroup.Traversable builtin iface should exist");
    assert!(
        semigroup_traversable_chirho
            .exports_chirho
            .values_chirho
            .contains_key("traverse1"),
        "Data.Semigroup.Traversable should export traverse1"
    );
    assert!(
        semigroup_traversable_chirho
            .exports_chirho
            .types_chirho
            .contains_key("Traversable1"),
        "Data.Semigroup.Traversable should export Traversable1"
    );
}

#[test]
fn builtin_splitmix32_iface_exists_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let splitmix32_iface_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "System.Random.SplitMix32")
        .expect("System.Random.SplitMix32 builtin iface should exist");
    assert!(
        splitmix32_iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("nextWord32"),
        "System.Random.SplitMix32 should export nextWord32"
    );
    assert!(
        splitmix32_iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("SMGen"),
        "System.Random.SplitMix32 should export SMGen"
    );
}

#[test]
fn builtin_text_parser_combinators_parsec_exports_parser_values_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let parsec_iface_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Text.ParserCombinators.Parsec")
        .expect("Text.ParserCombinators.Parsec builtin iface should exist");
    assert!(
        parsec_iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("<?>"),
        "Text.ParserCombinators.Parsec should export <?> as a value"
    );
    assert!(
        parsec_iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("parse"),
        "Text.ParserCombinators.Parsec should export parse as a value"
    );
    assert!(
        parsec_iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key("ParseError"),
        "Text.ParserCombinators.Parsec should still export ParseError as a type"
    );
}
