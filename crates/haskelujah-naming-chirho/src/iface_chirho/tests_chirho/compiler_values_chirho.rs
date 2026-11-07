// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Actual builtin value origins survive forwarding, never conflicting fragment replacement.

use crate::iface_chirho::{IfaceOriginChirho, ModuleIfaceChirho, merge_module_ifaces_chirho};
use haskelujah_ast_chirho::name_chirho::CompilerValueChirho;

fn prelude_chirho() -> ModuleIfaceChirho {
    crate::builtin_module_ifaces_chirho()
        .into_iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Prelude")
        .unwrap()
}

#[test]
fn a_real_forwarded_builtin_keeps_its_value_origin_even_in_a_source_interface_chirho() {
    let mut forwarded_chirho = prelude_chirho();
    assert_eq!(
        forwarded_chirho.exports_chirho.values_chirho["(++)"].compiler_value_chirho,
        Some(CompilerValueChirho::ListAppendChirho),
    );
    forwarded_chirho.name_chirho = "ForwardAppendChirho".to_string();
    forwarded_chirho.exports_chirho.origin_chirho = IfaceOriginChirho::SourceChirho;
    let merged_chirho =
        merge_module_ifaces_chirho(vec![forwarded_chirho.clone(), forwarded_chirho]);
    assert_eq!(
        merged_chirho[0].exports_chirho.values_chirho["(++)"].compiler_value_chirho,
        Some(CompilerValueChirho::ListAppendChirho),
    );
}

#[test]
fn conflicting_source_or_unknown_fragments_cannot_keep_a_builtin_value_claim_chirho() {
    for source_definer_chirho in [None, Some("OwnPreludeChirho".to_string())] {
        let builtin_chirho = prelude_chirho();
        let mut source_chirho = builtin_chirho.clone();
        source_chirho.exports_chirho.origin_chirho = IfaceOriginChirho::SourceChirho;
        let value_chirho = source_chirho
            .exports_chirho
            .values_chirho
            .get_mut("(++)")
            .unwrap();
        value_chirho.defining_module_chirho = source_definer_chirho;
        value_chirho.compiler_value_chirho = None;
        for copies_chirho in [
            vec![builtin_chirho.clone(), source_chirho.clone()],
            vec![source_chirho.clone(), builtin_chirho.clone()],
        ] {
            let merged_chirho = merge_module_ifaces_chirho(copies_chirho);
            assert_eq!(
                merged_chirho[0].exports_chirho.values_chirho["(++)"].compiler_value_chirho,
                None,
            );
        }
    }
}

#[test]
fn parenthesized_and_canonical_operator_fragments_share_one_proof_boundary_chirho() {
    let builtin_chirho = prelude_chirho();
    let mut source_chirho = builtin_chirho.clone();
    source_chirho.exports_chirho.origin_chirho = IfaceOriginChirho::SourceChirho;
    let mut value_chirho = source_chirho
        .exports_chirho
        .values_chirho
        .remove("(++)")
        .unwrap();
    value_chirho.name_chirho = "++".to_string();
    value_chirho.compiler_value_chirho = None;
    value_chirho.defining_module_chirho = Some("OwnPreludeChirho".to_string());
    source_chirho
        .exports_chirho
        .values_chirho
        .insert("++".to_string(), value_chirho);
    for copies_chirho in [
        vec![builtin_chirho.clone(), source_chirho.clone()],
        vec![source_chirho.clone(), builtin_chirho.clone()],
    ] {
        let merged_chirho = merge_module_ifaces_chirho(copies_chirho);
        for name_chirho in ["++", "(++)"] {
            assert_eq!(
                merged_chirho[0].exports_chirho.values_chirho[name_chirho].compiler_value_chirho,
                None
            );
        }
    }
}
