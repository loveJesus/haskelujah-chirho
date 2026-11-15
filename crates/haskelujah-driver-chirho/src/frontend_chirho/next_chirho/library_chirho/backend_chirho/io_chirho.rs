// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Closed contracts and exact owners for IO layout substitutions.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::super::io_chirho::{HANDLE_TEXT_CHIRHO, HANDLE_TYPES_CHIRHO, STD_HANDLES_CHIRHO};
use super::*;
#[derive(Clone, Copy)]
pub(in crate::next_chirho) enum IoProviderChirho {
    StdoutChirho,
    WriteChirho,
}
pub(in crate::next_chirho) fn handle_chirho() -> OriginalNameChirho {
    original_chirho(HANDLE_TYPES_CHIRHO, "Handle", NamespaceChirho::TypeChirho)
}
impl IoProviderChirho {
    pub(in crate::next_chirho) fn contract_chirho(self) -> PrimitiveContractChirho {
        let handle_chirho = InterfaceTypeChirho::DeclaredChirho(handle_chirho());
        if matches!(self, Self::StdoutChirho) {
            return PrimitiveContractChirho {
                ty_chirho: handle_chirho,
                arity_chirho: 0,
            };
        }
        let string_chirho = InterfaceTypeChirho::AppChirho(
            Box::new(InterfaceTypeChirho::BuiltinChirho(
                InterfaceBuiltinChirho::ListChirho,
            )),
            Box::new(InterfaceTypeChirho::BuiltinChirho(
                InterfaceBuiltinChirho::CharChirho,
            )),
        );
        let mut result_chirho = InterfaceTypeChirho::AppChirho(
            Box::new(InterfaceTypeChirho::BuiltinChirho(
                InterfaceBuiltinChirho::IOChirho,
            )),
            Box::new(InterfaceTypeChirho::TupleChirho(Vec::new())),
        );
        for argument_chirho in [
            InterfaceTypeChirho::BuiltinChirho(InterfaceBuiltinChirho::BoolChirho),
            string_chirho,
            handle_chirho,
        ] {
            result_chirho =
                InterfaceTypeChirho::FunChirho(Box::new(argument_chirho), Box::new(result_chirho));
        }
        PrimitiveContractChirho {
            ty_chirho: result_chirho,
            arity_chirho: 3,
        }
    }
}
pub(in crate::next_chirho) fn operation_chirho(
    name_chirho: &OriginalNameChirho,
) -> Option<IoProviderChirho> {
    if *name_chirho == original_chirho(STD_HANDLES_CHIRHO, "stdout", NamespaceChirho::ValueChirho) {
        Some(IoProviderChirho::StdoutChirho)
    } else if *name_chirho
        == original_chirho(HANDLE_TEXT_CHIRHO, "hPutStr'", NamespaceChirho::ValueChirho)
    {
        Some(IoProviderChirho::WriteChirho)
    } else {
        None
    }
}
pub(super) fn install_chirho(module_chirho: &str, policy_chirho: &mut LoweringPolicyChirho) {
    for (owner_chirho, occurrence_chirho, provider_chirho) in [
        (STD_HANDLES_CHIRHO, "stdout", IoProviderChirho::StdoutChirho),
        (
            HANDLE_TEXT_CHIRHO,
            "hPutStr'",
            IoProviderChirho::WriteChirho,
        ),
    ] {
        if module_chirho == owner_chirho {
            policy_chirho.primitive_bindings_chirho.insert(
                original_chirho(
                    owner_chirho,
                    occurrence_chirho,
                    NamespaceChirho::ValueChirho,
                ),
                provider_chirho.contract_chirho(),
            );
        }
    }
    if module_chirho == HANDLE_TYPES_CHIRHO {
        policy_chirho.opaque_data_chirho.insert(
            handle_chirho(),
            vec!["FileHandle".into(), "DuplexHandle".into()],
        );
    }
    if module_chirho == "GHC.Internal.Types" {
        policy_chirho.wired_declarations_chirho.insert(
            WiredNameChirho::IOChirho.original_chirho(),
            WiredNameChirho::IOChirho,
        );
    }
}
