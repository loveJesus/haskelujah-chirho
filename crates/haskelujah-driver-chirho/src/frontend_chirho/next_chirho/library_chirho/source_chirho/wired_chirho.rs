// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Read a wired language fixity from its unmodified defining source, never a name table.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use std::sync::OnceLock;

const TYPES_CHIRHO: &str = include_str!("../vendor-chirho/ghc-internal-chirho/Types.hs");
const TYPES_SHA_CHIRHO: &str = "a8fe6ab5c7a84be9b0710078b549aa63e511e7cb12d056cef91403d05ca6d2bb";

pub(crate) fn cons_fixity_chirho() -> Result<InterfaceFixityChirho, NextErrorChirho> {
    static FIXITY_CHIRHO: OnceLock<Result<InterfaceFixityChirho, String>> = OnceLock::new();
    FIXITY_CHIRHO
        .get_or_init(|| {
            read_chirho(TYPES_CHIRHO, TYPES_SHA_CHIRHO)
                .map_err(|error_chirho| error_chirho.message_chirho)
        })
        .clone()
        .map_err(|message_chirho| error_chirho(&message_chirho))
}

fn read_chirho(
    source_chirho: &str,
    sha_chirho: &str,
) -> Result<InterfaceFixityChirho, NextErrorChirho> {
    verify_source_chirho(
        source_chirho,
        sha_chirho,
        "libraries/ghc-internal/src/GHC/Internal/Types.hs",
        "9.14.1",
    )?;
    let declaration_chirho = source_chirho
        .get(4497..4508)
        .ok_or_else(|| error_chirho("wired cons fixity span is missing"))?;
    if source_chirho.as_bytes().get(4496) != Some(&b'\n') || !declaration_chirho.ends_with('\n') {
        return Err(error_chirho(
            "wired cons fixity is not one complete source line",
        ));
    }
    // This one unconditional top-level declaration has no name or CPP inference.
    let words_chirho: Vec<_> = declaration_chirho.split_whitespace().collect();
    let [associativity_chirho, precedence_chirho, ":"] = words_chirho.as_slice() else {
        return Err(error_chirho("wired cons fixity declaration changed"));
    };
    let associativity_chirho = match *associativity_chirho {
        "infixr" => AssociativityChirho::RightChirho,
        "infixl" => AssociativityChirho::LeftChirho,
        "infix" => AssociativityChirho::NoneChirho,
        _ => return Err(error_chirho("unknown wired cons associativity")),
    };
    let precedence_chirho = precedence_chirho
        .parse::<u8>()
        .ok()
        .filter(|value_chirho| *value_chirho <= 9)
        .ok_or_else(|| error_chirho("invalid wired cons precedence"))?;
    Ok(InterfaceFixityChirho {
        associativity_chirho,
        precedence_chirho,
    })
}

fn error_chirho(message_chirho: &str) -> NextErrorChirho {
    NextErrorChirho::new_chirho(NextErrorKindChirho::UnsupportedChirho, message_chirho)
}
