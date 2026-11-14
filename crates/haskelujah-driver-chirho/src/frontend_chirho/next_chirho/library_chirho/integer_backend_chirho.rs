// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-typed ordinary library functions with explicit opaque backend bodies.
//! These names are not wired declarations. Workflow: next-pipeline-chirho/ghc-implementation-map-chirho.

use super::*;
use haskelujah_parser_chirho::lexer_chirho::{LexerChirho, RawTokenKindChirho};
use haskelujah_runtime_chirho::integer_arithmetic_chirho::IntegerOperationChirho;
use haskelujah_span_chirho::FileIdChirho;

const OPERATIONS_CHIRHO: [(&str, IntegerOperationChirho); 6] = [
    ("integerSub", IntegerOperationChirho::SubtractChirho),
    ("integerAdd", IntegerOperationChirho::AddChirho),
    ("integerMul", IntegerOperationChirho::MultiplyChirho),
    ("integerNegate", IntegerOperationChirho::NegateChirho),
    ("integerAbs", IntegerOperationChirho::AbsoluteChirho),
    ("integerSignum", IntegerOperationChirho::SignumChirho),
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

pub(crate) fn install_policy_chirho(policy_chirho: &mut LoweringPolicyChirho) {
    for (name_chirho, operation_chirho) in OPERATIONS_CHIRHO {
        policy_chirho.primitive_bindings_chirho.insert(
            OriginalNameChirho {
                unit_chirho: UnitIdentityChirho::CompilerChirho,
                module_chirho: INTEGER_MODULE_CHIRHO.into(),
                namespace_chirho: NamespaceChirho::ValueChirho,
                occurrence_chirho: name_chirho.into(),
            },
            operation_chirho.arity_chirho(),
        );
    }
    policy_chirho.wired_declarations_chirho.insert(
        OriginalNameChirho {
            unit_chirho: UnitIdentityChirho::CompilerChirho,
            module_chirho: INTEGER_MODULE_CHIRHO.into(),
            namespace_chirho: NamespaceChirho::TypeChirho,
            occurrence_chirho: "Integer".into(),
        },
        WiredNameChirho::IntegerChirho,
    );
}

/// A body may be omitted only at an explicitly declared, single source signature.
/// The real type checker subsequently verifies its closed Integer arrow contract.
pub(super) fn validate_signature_span_chirho(
    source_chirho: &str,
    start_chirho: usize,
    end_chirho: usize,
) -> Result<(), NextErrorChirho> {
    let selection_chirho = source_chirho
        .get(start_chirho..end_chirho)
        .ok_or_else(|| error_chirho("invalid primitive signature span"))?;
    if (start_chirho > 0 && source_chirho.as_bytes().get(start_chirho - 1) != Some(&b'\n'))
        || !selection_chirho.ends_with('\n')
        || selection_chirho.lines().count() != 1
    {
        return Err(error_chirho(
            "primitive manifest does not select one complete signature line",
        ));
    }
    let tokens_chirho = LexerChirho::new_chirho(selection_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
        .lex_all_chirho()
        .into_iter()
        .filter(|token_chirho| !token_chirho.kind_chirho.is_trivia_chirho())
        .collect::<Vec<_>>();
    let Some(name_chirho) = tokens_chirho
        .first()
        .filter(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::VarIdChirho)
    else {
        return Err(error_chirho("primitive manifest has no value signature"));
    };
    if name_chirho.span_chirho.start_chirho().as_usize_chirho() != 0
        || tokens_chirho.get(1).is_none_or(|token_chirho| {
            token_chirho.kind_chirho != RawTokenKindChirho::ColonColonChirho
        })
        || !OPERATIONS_CHIRHO.iter().any(|(expected_chirho, _)| {
            Some(*expected_chirho) == name_chirho.span_chirho.text_chirho(selection_chirho)
        })
        || tokens_chirho.iter().skip(2).any(|token_chirho| {
            !matches!(
                token_chirho.kind_chirho,
                RawTokenKindChirho::ConIdChirho
                    | RawTokenKindChirho::RightArrowChirho
                    | RawTokenKindChirho::EofChirho
            )
        })
    {
        return Err(error_chirho(
            "source signature has no declared primitive body",
        ));
    }
    Ok(())
}

fn error_chirho(message_chirho: &str) -> NextErrorChirho {
    NextErrorChirho::new_chirho(NextErrorKindChirho::UnsupportedChirho, message_chirho)
}
