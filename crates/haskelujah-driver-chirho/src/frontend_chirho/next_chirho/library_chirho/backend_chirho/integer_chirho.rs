// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-typed ordinary library functions with explicit opaque backend bodies.
//! These names are not wired declarations. Workflow: next-pipeline-chirho/ghc-implementation-map-chirho.

use super::*;
use haskelujah_runtime_chirho::integer_arithmetic_chirho::IntegerOperationChirho;

const OPERATIONS_CHIRHO: [(&str, IntegerOperationChirho); 13] = [
    ("integerSub", IntegerOperationChirho::SubtractChirho),
    ("integerAdd", IntegerOperationChirho::AddChirho),
    ("integerMul", IntegerOperationChirho::MultiplyChirho),
    ("integerNegate", IntegerOperationChirho::NegateChirho),
    ("integerAbs", IntegerOperationChirho::AbsoluteChirho),
    ("integerSignum", IntegerOperationChirho::SignumChirho),
    ("integerEq", IntegerOperationChirho::EqualChirho),
    ("integerNe", IntegerOperationChirho::NotEqualChirho),
    ("integerLe", IntegerOperationChirho::LessEqualChirho),
    ("integerLt", IntegerOperationChirho::LessChirho),
    ("integerGt", IntegerOperationChirho::GreaterChirho),
    ("integerGe", IntegerOperationChirho::GreaterEqualChirho),
    ("integerCompare", IntegerOperationChirho::CompareChirho),
];

pub(crate) fn operation_chirho(
    original_chirho: &OriginalNameChirho,
) -> Option<IntegerOperationChirho> {
    if original_chirho.unit_chirho != UnitIdentityChirho::CompilerChirho
        || original_chirho.module_chirho != INTEGER_MODULE_CHIRHO
        || original_chirho.namespace_chirho != NamespaceChirho::ValueChirho
    {
        return None;
    }
    OPERATIONS_CHIRHO
        .iter()
        .find_map(|(name_chirho, operation_chirho)| {
            (*name_chirho == original_chirho.occurrence_chirho).then_some(*operation_chirho)
        })
}

pub(super) fn install_policy_chirho(policy_chirho: &mut LoweringPolicyChirho) {
    for (name_chirho, operation_chirho) in OPERATIONS_CHIRHO {
        policy_chirho.primitive_bindings_chirho.insert(
            original_chirho(
                INTEGER_MODULE_CHIRHO,
                name_chirho,
                NamespaceChirho::ValueChirho,
            ),
            ProviderChirho::IntegerChirho(operation_chirho).contract_chirho(),
        );
    }
    policy_chirho.wired_declarations_chirho.insert(
        original_chirho(
            INTEGER_MODULE_CHIRHO,
            "Integer",
            NamespaceChirho::TypeChirho,
        ),
        WiredNameChirho::IntegerChirho,
    );
}
