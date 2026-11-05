// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The builtin interface tables for `GHC.*`: each test pins one table entry it relies on.

use crate::iface_chirho::*;

#[test]
fn representation_producers_keep_exact_roles_after_duplicate_merge_chirho() {
    use haskelujah_ast_chirho::name_chirho::{
        CompilerRepresentationChirho, CompilerTypeConstructorChirho,
        TypeConstructorDeclarationChirho,
    };
    let builtins_chirho = builtin_module_ifaces_chirho();
    for module_chirho in ["GHC.Types", "GHC.Exts"] {
        let iface_chirho = builtins_chirho
            .iter()
            .find(|iface_chirho| iface_chirho.name_chirho == module_chirho)
            .expect("real producer");
        for representation_chirho in CompilerRepresentationChirho::ALL_CHIRHO {
            let ty_chirho = iface_chirho
                .exports_chirho
                .types_chirho
                .get(representation_chirho.name_chirho())
                .expect("representation export");
            let proof_chirho = TypeConstructorDeclarationChirho::compiler_data_chirho(
                CompilerTypeConstructorChirho::RepresentationChirho(representation_chirho),
            );
            assert_eq!(
                ty_chirho.declaration_chirho.as_ref(),
                Some(&proof_chirho),
                "{module_chirho}:{}",
                representation_chirho.name_chirho()
            );
            assert_eq!(
                ty_chirho.constructors_chirho,
                representation_chirho.constructors_chirho()
            );
            assert_ne!(
                proof_chirho.type_key_chirho(),
                representation_chirho.name_chirho()
            );
        }
    }
    let prim_chirho = builtins_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Prim")
        .expect("primitive producer");
    for representation_chirho in [
        CompilerRepresentationChirho::RuntimeTypeChirho,
        CompilerRepresentationChirho::RuntimeConstraintChirho,
    ] {
        assert_eq!(
            prim_chirho.exports_chirho.types_chirho[representation_chirho.name_chirho()]
                .declaration_chirho
                .as_ref(),
            Some(&TypeConstructorDeclarationChirho::compiler_data_chirho(
                CompilerTypeConstructorChirho::RepresentationChirho(representation_chirho)
            ))
        );
    }
    for opaque_chirho in ["RuntimeRep", "LiftedRep", "UnliftedRep"] {
        assert!(
            prim_chirho.exports_chirho.types_chirho[opaque_chirho]
                .declaration_chirho
                .is_none(),
            "historical opaque placeholder {opaque_chirho} is not a producer"
        );
    }
}

#[test]
fn builtin_ghc_typelits_exports_symbol_predicates_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_typelits_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.TypeLits")
        .expect("GHC.TypeLits builtin iface should exist");
    for name_chirho in ["isDigit", "isSymbol"] {
        assert!(
            ghc_typelits_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "GHC.TypeLits should export {name_chirho}"
        );
    }
    for name_chirho in ["CharToNat", "NatToChar", "<=?", "Div", "Mod", "Log2"] {
        assert!(
            ghc_typelits_chirho
                .exports_chirho
                .types_chirho
                .contains_key(name_chirho),
            "GHC.TypeLits should export {name_chirho}"
        );
    }

    let data_type_equality_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Type.Equality")
        .expect("Data.Type.Equality builtin iface should exist");
    assert!(
        data_type_equality_chirho
            .exports_chirho
            .types_chirho
            .contains_key("=="),
        "Data.Type.Equality should export the == type family"
    );
}

#[test]
fn builtin_ghc_exts_exports_aligned_byte_array_primop_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_exts_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Exts")
        .expect("GHC.Exts builtin iface should exist");
    for primop_name_chirho in &["newAlignedPinnedByteArray#", "setByteArray#"] {
        assert!(
            ghc_exts_chirho
                .exports_chirho
                .values_chirho
                .contains_key(*primop_name_chirho),
            "GHC.Exts should export {primop_name_chirho}"
        );
    }
}

#[test]
fn builtin_ghc_exts_exports_seq_hash_primop_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_exts_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Exts")
        .expect("GHC.Exts builtin iface should exist");
    assert!(
        ghc_exts_chirho
            .exports_chirho
            .values_chirho
            .contains_key("seq#"),
        "GHC.Exts should export seq#"
    );
}

#[test]
fn builtin_ghc_exts_exports_ishiftra_hash_primop_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_exts_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Exts")
        .expect("GHC.Exts builtin iface should exist");
    assert!(
        ghc_exts_chirho
            .exports_chirho
            .values_chirho
            .contains_key("iShiftRA#"),
        "GHC.Exts should export iShiftRA#"
    );
    assert!(
        ghc_exts_chirho
            .exports_chirho
            .values_chirho
            .contains_key("keepAlive#"),
        "GHC.Exts should export keepAlive#"
    );
}

#[test]
fn builtin_ghc_exts_exports_runrw_hash_primop_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_exts_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Exts")
        .expect("GHC.Exts builtin iface should exist");
    assert!(
        ghc_exts_chirho
            .exports_chirho
            .values_chirho
            .contains_key("runRW#"),
        "GHC.Exts should export runRW#"
    );
}

#[test]
fn builtin_ghc_prim_exports_mutable_array_primops_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_prim_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Prim")
        .expect("GHC.Prim builtin iface should exist");
    for name_chirho in &[
        "freezeArray#",
        "thawArray#",
        "copyArray#",
        "copyMutableArray#",
        "cloneArray#",
        "cloneMutableArray#",
        "sameMutableArray#",
        "setByteArray#",
        "keepAlive#",
    ] {
        assert!(
            ghc_prim_chirho
                .exports_chirho
                .values_chirho
                .contains_key(*name_chirho),
            "GHC.Prim should export {name_chirho}"
        );
    }
}

#[test]
fn builtin_ghc_base_exports_unsafe_chr_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_base_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Base")
        .expect("GHC.Base builtin iface should exist");
    assert!(
        ghc_base_chirho
            .exports_chirho
            .values_chirho
            .contains_key("unsafeChr"),
        "GHC.Base should export unsafeChr"
    );
}

#[test]
fn builtin_ghc_base_exports_mkweak_primop_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_base_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Base")
        .expect("GHC.Base builtin iface should exist");
    assert!(
        ghc_base_chirho
            .exports_chirho
            .values_chirho
            .contains_key("mkWeak#"),
        "GHC.Base should export mkWeak#"
    );
}

#[test]
fn builtin_ghc_exts_exports_extra_mvar_primops_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_exts_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Exts")
        .expect("GHC.Exts builtin iface should exist");
    for primop_name_chirho in &["readMVar#", "tryReadMVar#", "isEmptyMVar#", "sameMVar#"] {
        assert!(
            ghc_exts_chirho
                .exports_chirho
                .values_chirho
                .contains_key(*primop_name_chirho),
            "GHC.Exts should export {primop_name_chirho}"
        );
    }
}

#[test]
fn builtin_ghc_exts_exports_mutvar_primops_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_exts_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Exts")
        .expect("GHC.Exts builtin iface should exist");
    for primop_name_chirho in &["sameMutVar#", "atomicModifyMutVar#"] {
        assert!(
            ghc_exts_chirho
                .exports_chirho
                .values_chirho
                .contains_key(*primop_name_chirho),
            "GHC.Exts should export {primop_name_chirho}"
        );
    }
}

#[test]
fn builtin_ghc_integer_gmp_internals_exports_integer_constructors_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let gmp_internals_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Integer.GMP.Internals")
        .expect("GHC.Integer.GMP.Internals builtin iface should exist");
    assert!(
        gmp_internals_chirho
            .exports_chirho
            .types_chirho
            .get("Integer")
            .is_some_and(|integer_ty_chirho| integer_ty_chirho
                .constructors_chirho
                .contains(&"IS".to_string())
                && integer_ty_chirho
                    .constructors_chirho
                    .contains(&"IN".to_string())
                && integer_ty_chirho
                    .constructors_chirho
                    .contains(&"IP".to_string())),
        "GHC.Integer.GMP.Internals should export Integer(..)"
    );
}

#[test]
fn builtin_ghc_base_exports_primitive_char_ops_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_base_chirho = ifaces_chirho
        .iter()
        .rev()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.Base")
        .expect("GHC.Base builtin iface should exist");
    for primop_name_chirho in &["chr#", "ord#", "+#"] {
        assert!(
            ghc_base_chirho
                .exports_chirho
                .values_chirho
                .contains_key(*primop_name_chirho),
            "GHC.Base should export {primop_name_chirho}"
        );
    }
}

#[test]
fn builtin_ghc_cstring_exports_unpackers_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_cstring_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.CString")
        .expect("GHC.CString builtin iface should exist");
    for name_chirho in [
        "unpackCString#",
        "unpackCStringUtf8#",
        "unpackFoldrCString#",
    ] {
        assert!(
            ghc_cstring_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "GHC.CString should export {name_chirho}"
        );
    }
}

#[test]
fn builtin_ghc_byteorder_exports_type_and_value_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ghc_byteorder_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.ByteOrder")
        .expect("GHC.ByteOrder builtin iface should exist");
    assert!(
        ghc_byteorder_chirho
            .exports_chirho
            .types_chirho
            .get("ByteOrder")
            .is_some_and(|byteorder_ty_chirho| byteorder_ty_chirho
                .constructors_chirho
                .contains(&"LittleEndian".to_string())
                && byteorder_ty_chirho
                    .constructors_chirho
                    .contains(&"BigEndian".to_string())),
        "GHC.ByteOrder should export ByteOrder(..)"
    );
    for name_chirho in ["LittleEndian", "BigEndian", "targetByteOrder"] {
        assert!(
            ghc_byteorder_chirho
                .exports_chirho
                .values_chirho
                .contains_key(name_chirho),
            "GHC.ByteOrder should export {name_chirho}"
        );
    }
}

#[test]
fn builtin_ghc_ioref_exports_constructor_value_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let ioref_iface_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.IORef")
        .expect("GHC.IORef builtin iface should exist");
    assert!(
        ioref_iface_chirho
            .exports_chirho
            .types_chirho
            .get("IORef")
            .is_some_and(|io_ref_ty_chirho| io_ref_ty_chirho
                .constructors_chirho
                .contains(&"IORef".to_string())),
        "GHC.IORef should export IORef(..)"
    );
    assert!(
        ioref_iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("IORef"),
        "GHC.IORef should export IORef constructor as a value"
    );
}

#[test]
fn builtin_ghc_stref_exports_constructor_value_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    let stref_iface_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "GHC.STRef")
        .expect("GHC.STRef builtin iface should exist");
    assert!(
        stref_iface_chirho
            .exports_chirho
            .types_chirho
            .get("STRef")
            .is_some_and(|st_ref_ty_chirho| st_ref_ty_chirho
                .constructors_chirho
                .contains(&"STRef".to_string())),
        "GHC.STRef should export STRef(..)"
    );
    assert!(
        stref_iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("STRef"),
        "GHC.STRef should export STRef constructor as a value"
    );
}
