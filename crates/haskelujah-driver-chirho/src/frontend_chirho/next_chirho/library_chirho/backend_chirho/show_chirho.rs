// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Exact layout-touching Show declarations; the class and all other bodies remain source.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;

pub(super) fn operation_chirho(
    name_chirho: &OriginalNameChirho,
) -> Option<ShowNumberOperationChirho> {
    (name_chirho
        == &original_chirho(
            SHOW_MODULE_CHIRHO,
            "showSignedInt",
            NamespaceChirho::ValueChirho,
        ))
        .then_some(ShowNumberOperationChirho::IntChirho)
}
pub(super) fn key_chirho() -> DeclaredBodyKeyChirho {
    DeclaredBodyKeyChirho {
        unit_chirho: UnitIdentityChirho::CompilerChirho,
        module_chirho: SHOW_MODULE_CHIRHO.into(),
        class_chirho: original_chirho(SHOW_MODULE_CHIRHO, "Show", NamespaceChirho::TypeChirho),
        head_chirho: WiredNameChirho::IntegerChirho,
        method_chirho: original_chirho(
            SHOW_MODULE_CHIRHO,
            "showsPrec",
            NamespaceChirho::ValueChirho,
        ),
    }
}
pub(super) fn declared_operation_chirho(
    key_chirho: &DeclaredBodyKeyChirho,
) -> Option<ShowNumberOperationChirho> {
    (key_chirho == &self::key_chirho()).then_some(ShowNumberOperationChirho::IntegerChirho)
}
pub(super) fn text_type_chirho() -> InterfaceTypeChirho {
    InterfaceTypeChirho::AppChirho(
        Box::new(InterfaceTypeChirho::BuiltinChirho(
            InterfaceBuiltinChirho::ListChirho,
        )),
        Box::new(InterfaceTypeChirho::BuiltinChirho(
            InterfaceBuiltinChirho::CharChirho,
        )),
    )
}
pub(super) fn contract_chirho(
    operation_chirho: ShowNumberOperationChirho,
) -> PrimitiveContractChirho {
    let mut ty_chirho = text_type_chirho();
    for argument_chirho in [
        text_type_chirho(),
        InterfaceTypeChirho::BuiltinChirho(int_type_chirho(operation_chirho.argument_chirho())),
        InterfaceTypeChirho::BuiltinChirho(InterfaceBuiltinChirho::IntChirho),
    ] {
        ty_chirho = InterfaceTypeChirho::FunChirho(Box::new(argument_chirho), Box::new(ty_chirho));
    }
    PrimitiveContractChirho {
        kinds_chirho: Vec::new(),
        ty_chirho,
        arity_chirho: 3,
    }
}
pub(super) fn install_policy_chirho(policy_chirho: &mut LoweringPolicyChirho) {
    policy_chirho.primitive_bindings_chirho.insert(
        original_chirho(
            SHOW_MODULE_CHIRHO,
            "showSignedInt",
            NamespaceChirho::ValueChirho,
        ),
        contract_chirho(ShowNumberOperationChirho::IntChirho),
    );
    policy_chirho.instance_bodies_chirho.insert(
        key_chirho(),
        contract_chirho(ShowNumberOperationChirho::IntegerChirho),
    );
}
